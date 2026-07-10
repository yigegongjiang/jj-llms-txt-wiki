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
    Command::new(env!("CARGO_BIN_EXE_llms-wiki"))
        .args(args)
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
    let beta = server(HashMap::from([
        ("/llms.txt".to_owned(), Response::ok("[beta](/beta.md)")),
        ("/beta.md".to_owned(), Response::ok("beta")),
        ("/target.md".to_owned(), Response::ok("must-not-fetch")),
        ("/foreign.md".to_owned(), Response::ok("must-not-discover")),
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
            Response::redirect(format!("{}/target.md", beta.origin)),
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

    let config_path = home.path().join(".config/llms-wiki/config.toml");
    let config_before = fs::read(&config_path).unwrap();
    let alpha_sync = success(
        home.path(),
        &["sync", "alpha", "--concurrency", "2", "--interval", "0ms"],
    );
    assert!(String::from_utf8_lossy(&alpha_sync.stderr).contains("alpha: ok"));
    let wiki = home.path().join("llms-wiki");
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
    assert!(!wiki.join("beta").exists());
    assert_eq!(fs::read(&config_path).unwrap(), config_before);
    assert!(wiki.join(".git").is_dir());
    assert_eq!(git(&wiki, &["rev-list", "--count", "HEAD"]), "1");
    let subject = git(&wiki, &["log", "-1", "--format=%s"]);
    assert!(subject.starts_with("chore(sync): alpha @ "));
    assert!(subject.ends_with('Z'));
    assert!(git(&wiki, &["status", "--porcelain"]).is_empty());

    success(
        home.path(),
        &["sync", "--concurrency", "2", "--interval", "0ms"],
    );
    assert_eq!(
        fs::read_to_string(wiki.join("beta/beta.md")).unwrap(),
        "beta"
    );
    assert_eq!(git(&wiki, &["rev-list", "--count", "HEAD"]), "2");
    assert!(git(&wiki, &["log", "-1", "--format=%s"]).starts_with("chore(sync): alpha,beta @ "));
    let beta_requests = beta.requests.read().unwrap();
    assert!(!beta_requests.iter().any(|path| path == "/target.md"));
    assert!(!beta_requests.iter().any(|path| path == "/foreign.md"));
    drop(beta_requests);

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
    assert_eq!(git(&wiki, &["rev-list", "--count", "HEAD"]), "3");

    success(home.path(), &["sync", "alpha", "--interval", "0ms"]);
    assert_eq!(
        git(&wiki, &["rev-list", "--count", "HEAD"]),
        "4",
        "unchanged sync must remain visible in history"
    );

    let stable = tree(&wiki.join("alpha"));
    for (status, path) in [(429, "/busy.md"), (500, "/broken.md")] {
        alpha.routes.write().unwrap().insert(
            "/llms.txt".to_owned(),
            Response::ok(&format!("[failure]({path})")),
        );
        alpha
            .routes
            .write()
            .unwrap()
            .insert(path.to_owned(), Response::status(status));
        let failed = cli(home.path(), &["sync", "alpha", "--interval", "0ms"]);
        assert!(!failed.status.success());
        assert_eq!(tree(&wiki.join("alpha")), stable);
        assert_eq!(git(&wiki, &["rev-list", "--count", "HEAD"]), "4");
    }

    assert!(!wiki.join(".cache").exists());
    assert!(fs::read_dir(&wiki).unwrap().all(|entry| {
        let name = entry.unwrap().file_name();
        name == ".git" || !name.to_string_lossy().starts_with('.')
    }));
    assert!(git(&wiki, &["status", "--porcelain"]).is_empty());
}
