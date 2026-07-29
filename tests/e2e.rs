use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tempfile::tempdir;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

#[derive(Clone)]
struct Response {
    status: u16,
    body: Vec<u8>,
    location: Option<String>,
    delay: Duration,
}

impl Response {
    fn ok(body: &str) -> Self {
        Self {
            status: 200,
            body: body.as_bytes().to_vec(),
            location: None,
            delay: Duration::ZERO,
        }
    }

    fn status(status: u16) -> Self {
        Self {
            status,
            body: Vec::new(),
            location: None,
            delay: Duration::ZERO,
        }
    }

    fn redirect(location: impl Into<String>) -> Self {
        Self {
            status: 302,
            body: Vec::new(),
            location: Some(location.into()),
            delay: Duration::ZERO,
        }
    }
}

struct Server {
    origin: String,
    routes: Arc<RwLock<HashMap<String, Response>>>,
    requests: Arc<RwLock<Vec<String>>>,
    task: JoinHandle<()>,
}

impl Drop for Server {
    fn drop(&mut self) {
        self.task.abort();
    }
}

async fn server(initial: HashMap<String, Response>) -> Server {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let routes = Arc::new(RwLock::new(initial));
    let requests = Arc::new(RwLock::new(Vec::new()));
    let task_routes = Arc::clone(&routes);
    let task_requests = Arc::clone(&requests);
    let task = tokio::spawn(async move {
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                return;
            };
            let routes = Arc::clone(&task_routes);
            let requests = Arc::clone(&task_requests);
            tokio::spawn(async move {
                let mut buffer = [0_u8; 8192];
                let Ok(length) = stream.read(&mut buffer).await else {
                    return;
                };
                let request = String::from_utf8_lossy(&buffer[..length]);
                let path = request
                    .lines()
                    .next()
                    .and_then(|line| line.split_whitespace().nth(1))
                    .unwrap_or("/")
                    .to_owned();
                requests.write().unwrap().push(path.clone());
                let response = routes
                    .read()
                    .unwrap()
                    .get(&path)
                    .cloned()
                    .unwrap_or_else(|| Response::status(404));
                tokio::time::sleep(response.delay).await;
                let reason = match response.status {
                    200 => "OK",
                    302 => "Found",
                    404 => "Not Found",
                    410 => "Gone",
                    429 => "Too Many Requests",
                    500 => "Internal Server Error",
                    _ => "Status",
                };
                let location = response
                    .location
                    .map(|value| format!("Location: {value}\r\n"))
                    .unwrap_or_default();
                let head = format!(
                    "HTTP/1.1 {} {}\r\n{}Content-Length: {}\r\nConnection: close\r\n\r\n",
                    response.status,
                    reason,
                    location,
                    response.body.len()
                );
                let _ = stream.write_all(head.as_bytes()).await;
                let _ = stream.write_all(&response.body).await;
            });
        }
    });
    Server {
        origin: format!("http://{address}"),
        routes,
        requests,
        task,
    }
}

fn cli(home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jj-llms-txt-wiki"))
        .args(args)
        // Suppress the auto-mirror push in every e2e run. Without this the
        // spawned binary would try to `git push` to the code repo's remote
        // (baked from `CARGO_PKG_REPOSITORY`), which on CI has a token and
        // could actually mutate the public repository.
        .env("JJ_LLMS_TXT_WIKI_PUSH_URL", "")
        .env("HOME", home)
        .output()
        .unwrap()
}

fn success(home: &Path, args: &[&str]) -> Output {
    let output = cli(home, args);
    assert!(
        output.status.success(),
        "args={args:?}\nstdout={}\nstderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

fn git(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "args={args:?}\nstdout={}\nstderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

fn tree(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fn visit(root: &Path, current: &Path, files: &mut BTreeMap<PathBuf, Vec<u8>>) {
        if !current.exists() {
            return;
        }
        for entry in fs::read_dir(current).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                visit(root, &path, files);
            } else {
                files.insert(
                    path.strip_prefix(root).unwrap().to_path_buf(),
                    fs::read(path).unwrap(),
                );
            }
        }
    }
    let mut files = BTreeMap::new();
    visit(root, root, &mut files);
    files
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn real_cli_covers_sites_recursive_sync_and_snapshot_rollback() {
    // A truly external origin: the entry never declares it, so a redirect that
    // lands here must be ignored — this guards against redirect-escape.
    let foreign = server(HashMap::from([(
        "/target.md".to_owned(),
        Response::ok("must-not-follow-redirect"),
    )]))
    .await;
    // beta's host also carries a `/foreign.md` that alpha's entry links to
    // directly. The entry vouches for that origin, so it is legitimately mirrored
    // (the bun.sh entry → bun.com content shape).
    let beta = server(HashMap::from([
        ("/llms.txt".to_owned(), Response::ok("[beta](/beta.md)")),
        ("/beta.md".to_owned(), Response::ok("beta")),
        ("/foreign.md".to_owned(), Response::ok("beta-cross-origin")),
    ]))
    .await;
    let alpha_entry = format!(
        "[a](/docs/a.md) [same](/same.md) [cross](/cross.md) [missing](/missing.md) [gone](/gone.md) [foreign]({}/foreign.md)",
        beta.origin
    );
    let alpha = server(HashMap::from([
        ("/llms.txt".to_owned(), Response::ok(&alpha_entry)),
        (
            "/docs/a.md".to_owned(),
            Response::ok("[b](b.md) [cycle](/docs/a.md)"),
        ),
        ("/docs/b.md".to_owned(), Response::ok("b-v1")),
        ("/same.md".to_owned(), Response::redirect("/final.md")),
        ("/final.md".to_owned(), Response::ok("redirected")),
        (
            "/cross.md".to_owned(),
            Response::redirect(format!("{}/target.md", foreign.origin)),
        ),
        ("/missing.md".to_owned(), Response::status(404)),
        ("/gone.md".to_owned(), Response::status(410)),
    ]))
    .await;
    let home = tempdir().unwrap();

    success(
        home.path(),
        &["site", "add", "beta", &format!("{}/llms.txt", beta.origin)],
    );
    success(
        home.path(),
        &[
            "site",
            "add",
            "alpha",
            &format!("{}/llms.txt", alpha.origin),
        ],
    );
    let listed = success(home.path(), &["site", "list"]);
    let listed = String::from_utf8(listed.stdout).unwrap();
    assert!(listed.starts_with("alpha\t"));
    assert!(listed.lines().nth(1).unwrap().starts_with("beta\t"));

    let config_path = home.path().join(".config/jj-llms-txt-wiki/config.toml");
    let config_before = fs::read(&config_path).unwrap();
    let alpha_sync = success(
        home.path(),
        &["sync", "alpha", "--concurrency", "2", "--interval", "0ms"],
    );
    assert!(String::from_utf8_lossy(&alpha_sync.stderr).contains("alpha: ok"));
    let wiki = home.path().join(".config/jj-llms-txt-wiki/wiki");
    assert!(wiki.join("alpha/docs/a.md").exists());
    assert_eq!(
        fs::read_to_string(wiki.join("alpha/docs/b.md")).unwrap(),
        "b-v1"
    );
    assert_eq!(
        fs::read_to_string(wiki.join("alpha/same.md")).unwrap(),
        "redirected"
    );
    assert!(!wiki.join("alpha/missing.md").exists());
    assert!(!wiki.join("alpha/gone.md").exists());
    // Cross-origin content the entry explicitly declares is mirrored under alpha,
    // keyed by URL path (host-agnostic), not under a beta directory.
    assert_eq!(
        fs::read_to_string(wiki.join("alpha/foreign.md")).unwrap(),
        "beta-cross-origin"
    );
    assert!(!wiki.join("beta").exists());
    assert_eq!(fs::read(&config_path).unwrap(), config_before);
    assert!(wiki.join(".git").is_dir());
    assert_eq!(git(&wiki, &["rev-list", "--count", "HEAD"]), "1");
    let subject = git(&wiki, &["log", "-1", "--format=%s"]);
    assert!(subject.starts_with("chore(sync): alpha @ "));
    assert!(subject.ends_with('Z'));
    assert!(git(&wiki, &["status", "--porcelain"]).is_empty());

    // Slots are real sockets now, so an out-of-range request is clamped with a
    // warning instead of being honoured or rejected — a config written for the
    // old global-rate-gate semantics MUST keep syncing.
    let all_sync = success(
        home.path(),
        &["sync", "--concurrency", "500", "--interval", "0ms"],
    );
    assert!(
        String::from_utf8_lossy(&all_sync.stderr)
            .contains("concurrency 500 exceeds the maximum, using 64")
    );
    assert_eq!(
        fs::read_to_string(wiki.join("beta/beta.md")).unwrap(),
        "beta"
    );
    // Per-site commits: alpha's earlier commit + this run's alpha + beta = 3.
    assert_eq!(git(&wiki, &["rev-list", "--count", "HEAD"]), "3");
    let subjects = git(&wiki, &["log", "-2", "--format=%s"]);
    assert!(
        subjects
            .lines()
            .any(|s| s.starts_with("chore(sync): alpha @ "))
            && subjects
                .lines()
                .any(|s| s.starts_with("chore(sync): beta @ ")),
        "expected per-site commits, got: {subjects}"
    );
    // The entry-declared cross-origin link on beta's host IS fetched.
    let beta_requests = beta.requests.read().unwrap();
    assert!(beta_requests.iter().any(|path| path == "/foreign.md"));
    drop(beta_requests);
    // A same-origin page redirecting to an origin the entry never declared is
    // ignored — the foreign target is never requested.
    assert!(
        !foreign
            .requests
            .read()
            .unwrap()
            .iter()
            .any(|path| path == "/target.md")
    );

    alpha
        .routes
        .write()
        .unwrap()
        .insert("/llms.txt".to_owned(), Response::ok("[new](/docs/new.md)"));
    alpha
        .routes
        .write()
        .unwrap()
        .insert("/docs/new.md".to_owned(), Response::ok("new-v2"));
    success(home.path(), &["sync", "alpha", "--interval", "0ms"]);
    assert!(!wiki.join("alpha/docs/a.md").exists());
    assert!(!wiki.join("alpha/docs/b.md").exists());
    assert_eq!(
        fs::read_to_string(wiki.join("alpha/docs/new.md")).unwrap(),
        "new-v2"
    );
    assert_eq!(git(&wiki, &["rev-list", "--count", "HEAD"]), "4");

    success(home.path(), &["sync", "alpha", "--interval", "0ms"]);
    assert_eq!(
        git(&wiki, &["rev-list", "--count", "HEAD"]),
        "5",
        "unchanged sync must remain visible in history"
    );

    // One page still erroring after its retries is a dead link upstream, not an
    // outage: it degrades, and the rest of the site publishes as usual.
    alpha.routes.write().unwrap().insert(
        "/llms.txt".to_owned(),
        Response::ok("[new](/docs/new.md) [broken](/broken.md)"),
    );
    alpha
        .routes
        .write()
        .unwrap()
        .insert("/broken.md".to_owned(), Response::status(500));
    let degraded = cli(home.path(), &["sync", "alpha", "--interval", "0ms"]);
    let stderr = String::from_utf8_lossy(&degraded.stderr);
    assert!(degraded.status.success(), "stderr={stderr}");
    assert!(stderr.contains("1 degraded"), "stderr={stderr}");
    assert!(wiki.join("alpha/docs/new.md").exists());
    assert!(!wiki.join("alpha/broken.md").exists());
    assert_eq!(git(&wiki, &["rev-list", "--count", "HEAD"]), "6");

    // Past the tolerance the same errors read as an outage: the site fails whole
    // and its last good snapshot survives untouched.
    let stable = tree(&wiki.join("alpha"));
    alpha.routes.write().unwrap().insert(
        "/llms.txt".to_owned(),
        Response::ok("[1](/b1.md) [2](/b2.md) [3](/b3.md) [4](/b4.md)"),
    );
    for (status, path) in [
        (429, "/b1.md"),
        (429, "/b2.md"),
        (500, "/b3.md"),
        (503, "/b4.md"),
    ] {
        alpha
            .routes
            .write()
            .unwrap()
            .insert(path.to_owned(), Response::status(status));
    }
    let failed = cli(home.path(), &["sync", "alpha", "--interval", "0ms"]);
    assert!(!failed.status.success());
    assert_eq!(tree(&wiki.join("alpha")), stable);
    assert_eq!(git(&wiki, &["rev-list", "--count", "HEAD"]), "6");

    assert!(!wiki.join(".cache").exists());
    // A failed sync leaves its `.alpha.sync.*` partial on disk for resume, but it
    // is gitignored, so no backup dir survives and the work tree stays clean.
    assert!(
        fs::read_dir(&wiki).unwrap().all(|entry| {
            let name = entry.unwrap().file_name();
            !name.to_string_lossy().contains(".backup.")
        }),
        "commit backup directory must not survive"
    );
    assert!(git(&wiki, &["status", "--porcelain"]).is_empty());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn real_cli_syncs_full_bundle_with_fresh_atomic_snapshots() {
    let first = "# Bundle\n\n> Complete docs\n\n# One\n\nURL: http://placeholder/one\n\nOne body.\n\n---\n\n# Guide\n\n> Guide description\n\nURL: http://placeholder/guide/\n\nGuide v1.\n";
    let srv = server(HashMap::from([
        ("/llms-full.txt".to_owned(), Response::ok(first)),
        ("/llms.txt".to_owned(), Response::ok("[doc](/doc.md)")),
        ("/doc.md".to_owned(), Response::ok("recursive")),
    ]))
    .await;
    let bundle = first.replace("http://placeholder", &srv.origin);
    srv.routes
        .write()
        .unwrap()
        .insert("/llms-full.txt".to_owned(), Response::ok(&bundle));
    let home = tempdir().unwrap();
    success(
        home.path(),
        &[
            "site",
            "add",
            "full",
            &format!("{}/llms-full.txt", srv.origin),
        ],
    );
    success(
        home.path(),
        &["site", "add", "index", &format!("{}/llms.txt", srv.origin)],
    );

    let wiki = home.path().join(".config/jj-llms-txt-wiki/wiki");
    let stale = wiki.join(".full.sync.stale");
    fs::create_dir_all(&stale).unwrap();
    fs::write(stale.join("removed.md"), "stale").unwrap();

    let synced = success(home.path(), &["sync", "full"]);
    let stderr = String::from_utf8_lossy(&synced.stderr);
    assert!(stderr.contains("full: ok"), "stderr={stderr}");
    assert!(stderr.contains("downloaded=2"), "stderr={stderr}");
    assert_eq!(
        fs::read_to_string(wiki.join("full/one.md")).unwrap(),
        "# One\n\nOne body.\n"
    );
    assert_eq!(
        fs::read_to_string(wiki.join("full/guide/index.md")).unwrap(),
        "# Guide\n\n> Guide description\n\nGuide v1.\n"
    );
    assert!(!wiki.join("full/.jj-llms-txt-wiki.json").exists());
    assert!(
        !stale.exists(),
        "full sync must discard interrupted partials"
    );
    assert_eq!(
        srv.requests
            .read()
            .unwrap()
            .iter()
            .filter(|path| path.as_str() == "/llms-full.txt")
            .count(),
        1,
        "full sync uses one HTTP request"
    );

    let second = format!(
        "# Bundle\n\n# Guide\n\nURL: {}/guide/\n\nGuide v2.\n",
        srv.origin
    );
    srv.routes
        .write()
        .unwrap()
        .insert("/llms-full.txt".to_owned(), Response::ok(&second));
    success(home.path(), &["sync"]);
    assert!(!wiki.join("full/one.md").exists());
    assert_eq!(
        fs::read_to_string(wiki.join("full/guide/index.md")).unwrap(),
        "# Guide\n\nGuide v2.\n"
    );
    assert_eq!(
        fs::read_to_string(wiki.join("index/doc.md")).unwrap(),
        "recursive"
    );

    srv.routes
        .write()
        .unwrap()
        .insert("/bundle.txt".to_owned(), Response::ok(&second));
    srv.routes.write().unwrap().insert(
        "/llms-full.txt".to_owned(),
        Response::redirect("/bundle.txt"),
    );
    success(home.path(), &["sync", "full"]);
    assert!(
        srv.requests
            .read()
            .unwrap()
            .iter()
            .any(|path| path == "/bundle.txt")
    );

    let stable = tree(&wiki.join("full"));
    let commits = git(&wiki, &["rev-list", "--count", "HEAD"]);
    let collision = format!(
        "# One\n\nURL: {}/same?q=1\n\nOne\n\n---\n\n# Two\n\nURL: {}/same?q=2\n\nTwo\n",
        srv.origin, srv.origin
    );
    for response in [
        Response::ok(&collision),
        Response::status(404),
        Response::status(500),
        Response {
            status: 200,
            body: vec![0xff],
            location: None,
            delay: Duration::ZERO,
        },
    ] {
        srv.routes
            .write()
            .unwrap()
            .insert("/llms-full.txt".to_owned(), response);
        let failed = cli(home.path(), &["sync", "full"]);
        assert!(!failed.status.success());
        assert_eq!(tree(&wiki.join("full")), stable);
        assert_eq!(git(&wiki, &["rev-list", "--count", "HEAD"]), commits);
    }
    assert!(fs::read_dir(&wiki).unwrap().all(|entry| {
        !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".full.sync.")
    }));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn real_cli_syncs_multi_entry_sites_on_both_chains() {
    let one = "# A\n\nURL: http://placeholder/a\n\nA body.\n";
    let two = "# B\n\nURL: http://placeholder/b\n\nB body.\n";
    let srv = server(HashMap::from([
        (
            "/workers/llms.txt".to_owned(),
            Response::ok("[a](/workers/a.md) [pages](/pages/llms.txt)"),
        ),
        (
            "/pages/llms.txt".to_owned(),
            Response::ok("[b](/pages/b.md)"),
        ),
        ("/workers/a.md".to_owned(), Response::ok("a")),
        ("/pages/b.md".to_owned(), Response::ok("b")),
    ]))
    .await;
    for (path, bundle) in [("/one/llms-full.txt", one), ("/two/llms-full.txt", two)] {
        let body = bundle.replace("http://placeholder", &srv.origin);
        srv.routes
            .write()
            .unwrap()
            .insert(path.to_owned(), Response::ok(&body));
    }

    let home = tempdir().unwrap();
    let added = success(
        home.path(),
        &[
            "site",
            "add",
            "index",
            &format!("{}/workers/llms.txt", srv.origin),
            &format!("{}/pages/llms.txt", srv.origin),
        ],
    );
    assert_eq!(
        String::from_utf8_lossy(&added.stdout).trim(),
        format!(
            "index\t{}/workers/llms.txt {}/pages/llms.txt",
            srv.origin, srv.origin
        )
    );
    success(
        home.path(),
        &[
            "site",
            "add",
            "full",
            &format!("{}/one/llms-full.txt", srv.origin),
            &format!("{}/two/llms-full.txt", srv.origin),
        ],
    );
    // One site cannot straddle the two chains — they disagree on snapshot strategy.
    let mixed = cli(
        home.path(),
        &[
            "site",
            "add",
            "mixed",
            &format!("{}/workers/llms.txt", srv.origin),
            &format!("{}/one/llms-full.txt", srv.origin),
        ],
    );
    assert!(!mixed.status.success());
    assert!(
        String::from_utf8_lossy(&mixed.stderr).contains("mixes llms.txt and llms-full.txt"),
        "stderr={}",
        String::from_utf8_lossy(&mixed.stderr)
    );

    let config =
        fs::read_to_string(home.path().join(".config/jj-llms-txt-wiki/config.toml")).unwrap();
    assert!(config.contains("urls = ["), "config={config}");
    assert!(!config.contains("[sites.mixed]"), "config={config}");

    let synced = success(home.path(), &["sync"]);
    let stderr = String::from_utf8_lossy(&synced.stderr);
    assert!(stderr.contains("index: ok"), "stderr={stderr}");
    assert!(stderr.contains("full: ok"), "stderr={stderr}");

    let wiki = home.path().join(".config/jj-llms-txt-wiki/wiki");
    // Index chain: every entry's content lands in the one site directory, and a
    // sibling entry linked as content is still treated as an entry (never written).
    assert_eq!(
        fs::read_to_string(wiki.join("index/workers/a.md")).unwrap(),
        "a"
    );
    assert_eq!(
        fs::read_to_string(wiki.join("index/pages/b.md")).unwrap(),
        "b"
    );
    assert!(!wiki.join("index/pages/llms.txt").exists());
    // Aggregate chain: both bundles merge into one snapshot.
    assert_eq!(
        fs::read_to_string(wiki.join("full/a.md")).unwrap(),
        "# A\n\nA body.\n"
    );
    assert_eq!(
        fs::read_to_string(wiki.join("full/b.md")).unwrap(),
        "# B\n\nB body.\n"
    );

    // A page carried by two bundles has no defensible winner: fail the site and
    // keep the last good snapshot rather than silently letting one overwrite the other.
    let stable = tree(&wiki.join("full"));
    let clash = two
        .replace("/b", "/a")
        .replace("http://placeholder", &srv.origin);
    srv.routes
        .write()
        .unwrap()
        .insert("/two/llms-full.txt".to_owned(), Response::ok(&clash));
    let failed = cli(home.path(), &["sync", "full"]);
    assert!(!failed.status.success());
    assert!(
        String::from_utf8_lossy(&failed.stderr).contains("duplicate page"),
        "stderr={}",
        String::from_utf8_lossy(&failed.stderr)
    );
    assert_eq!(tree(&wiki.join("full")), stable);
    assert!(git(&wiki, &["status", "--porcelain"]).is_empty());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn real_cli_resumes_an_interrupted_partial() {
    let srv = server(HashMap::from([
        (
            "/llms.txt".to_owned(),
            Response::ok("[keep](/keep.md) [fresh](/fresh.md)"),
        ),
        ("/keep.md".to_owned(), Response::ok("KEEP")),
        ("/fresh.md".to_owned(), Response::ok("FRESH")),
    ]))
    .await;
    let home = tempdir().unwrap();
    success(
        home.path(),
        &["site", "add", "docs", &format!("{}/llms.txt", srv.origin)],
    );

    // Simulate an earlier run interrupted after downloading keep.md but before
    // committing: a leftover `.docs.sync.*` working directory on disk.
    let wiki = home.path().join(".config/jj-llms-txt-wiki/wiki");
    let leftover = wiki.join(".docs.sync.leftover");
    fs::create_dir_all(&leftover).unwrap();
    fs::write(leftover.join("keep.md"), "KEEP").unwrap();

    let sync = success(
        home.path(),
        &["sync", "docs", "--concurrency", "2", "--interval", "0ms"],
    );
    let stderr = String::from_utf8_lossy(&sync.stderr);
    assert!(
        stderr.contains("resuming interrupted partial"),
        "stderr={stderr}"
    );
    assert!(stderr.contains("docs: ok"), "stderr={stderr}");

    let requested = srv.requests.read().unwrap().clone();
    assert!(
        !requested.iter().any(|path| path == "/keep.md"),
        "a resumed file must not be re-fetched: {requested:?}"
    );
    assert!(
        requested.iter().any(|path| path == "/fresh.md"),
        "the missing file is still downloaded: {requested:?}"
    );

    assert_eq!(
        fs::read_to_string(wiki.join("docs/keep.md")).unwrap(),
        "KEEP"
    );
    assert_eq!(
        fs::read_to_string(wiki.join("docs/fresh.md")).unwrap(),
        "FRESH"
    );
    // The adopted partial was committed, so nothing is left behind.
    assert!(!leftover.exists());
    assert!(git(&wiki, &["status", "--porcelain"]).is_empty());
}
