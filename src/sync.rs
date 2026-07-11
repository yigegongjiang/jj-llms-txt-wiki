use console::style;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use jiff::Timestamp;

use crate::config::Config;
use crate::crawler::{CrawlFailure, CrawlObserver, CrawlOptions, DEFAULT_TIMEOUT, crawl};
use crate::git::Repository;
use crate::manifest::Manifest;
use crate::progress::{SyncProgress, error_line};
use crate::report::{self, Outcome, SiteReport};
use crate::site::parse_entry_url;
use crate::snapshot::Snapshot;

pub async fn run(
    config_path: &Path,
    requested_site: Option<String>,
    concurrency: Option<usize>,
    interval: Option<Duration>,
    push_url: Option<String>,
) -> Result<(), String> {
    let config = Config::load(config_path)?;
    if config.sites.is_empty() {
        return Err("no sites configured".to_owned());
    }
    let output_root = config.output_path()?;
    let targets: Vec<_> = if let Some(name) = requested_site {
        let site = config
            .sites
            .get(&name)
            .ok_or_else(|| format!("unknown site: {name}"))?;
        vec![(name, site.clone())]
    } else {
        config.sites.into_iter().collect()
    };
    let repository = Repository::prepare(&output_root)?;
    let options = CrawlOptions {
        concurrency: concurrency.unwrap_or(config.concurrency),
        interval: interval.unwrap_or(Duration::from_millis(config.interval_ms)),
        timeout: DEFAULT_TIMEOUT,
    };
    let total = targets.len();
    let mut reports: Vec<SiteReport> = Vec::with_capacity(total);
    let mut committed = false;

    for (index, (name, site)) in targets.into_iter().enumerate() {
        let position = index + 1;
        eprintln!(
            "{}",
            style(format!("── [{position}/{total}] {name} ──"))
                .for_stderr()
                .cyan()
                .bold()
        );
        let outcome = sync_site(
            &name,
            &site.url,
            &output_root,
            options,
            position,
            total,
            &repository,
        )
        .await;
        if matches!(outcome, Outcome::Ok(_)) {
            committed = true;
        }
        reports.push(SiteReport {
            site: name,
            outcome,
        });
    }

    // Best-effort mirror — a push failure here (auth / offline / fork without
    // write access) is expected in most environments and MUST NOT fail the
    // sync.
    if committed
        && let Some(url) = push_url.as_deref()
        && let Err(error) = repository.push_snapshot(url)
    {
        eprintln!(
            "{}: push snapshot skipped: {error}",
            style("warning").for_stderr().yellow().bold()
        );
    }

    // Durable record of this run, overwritten each time; the failure block below
    // points the user at it. Written for successful runs too, so "what happened
    // last run" always has an answer on disk.
    let timestamp = Timestamp::now().strftime("%Y-%m-%dT%H:%M:%SZ").to_string();
    let log = report::write_log(&output_root, &reports, &timestamp);

    if reports.iter().any(|report| !report.is_ok()) {
        report::print_failures(&reports, &log);
        // The block above is the user-facing error report; returning an empty
        // string keeps the failure exit code without making main.rs print a
        // second, redundant `error:` line on top of it.
        Err(String::new())
    } else {
        Ok(())
    }
}

/// Sync a single site end to end, printing its live verdict line, and return the
/// outcome for the aggregate report. Whole-site errors (bad entry URL, snapshot
/// setup, or a crawl that never produced a report) become `Aborted` carrying the
/// real reason, so the verdict reads `error — <reason>` instead of a misleading
/// `failed; … failed=0`.
async fn sync_site(
    name: &str,
    url: &str,
    output_root: &Path,
    options: CrawlOptions,
    position: usize,
    total: usize,
    repository: &Repository,
) -> Outcome {
    let entry = match parse_entry_url(url) {
        Ok(entry) => entry,
        Err(error) => {
            eprintln!("{}", error_line(name, &error));
            return Outcome::Aborted(error);
        }
    };
    let snapshot = match Snapshot::new(output_root, name) {
        Ok(snapshot) => snapshot,
        Err(error) => {
            eprintln!("{}", error_line(name, &error));
            return Outcome::Aborted(error);
        }
    };
    if snapshot.resumed() {
        eprintln!(
            "{}",
            style("   ↻ resuming interrupted partial")
                .for_stderr()
                .cyan()
        );
    }
    let previous_site = output_root.join(name);
    let previous_manifest = Manifest::load(&previous_site);
    let previous_root = previous_site.is_dir().then_some(previous_site.as_path());
    let progress = Arc::new(SyncProgress::new(name, position, total));
    let observer: Arc<dyn CrawlObserver> = progress.clone();
    let mut report = match crawl(
        entry,
        snapshot.path(),
        previous_root,
        &previous_manifest,
        options,
        observer,
    )
    .await
    {
        Ok(report) => report,
        Err(error) => {
            progress.abort();
            eprintln!("{}", error_line(name, &error));
            return Outcome::Aborted(error);
        }
    };

    if !report.is_success() {
        progress.finish(&report, false);
        return Outcome::Failed(report);
    }

    // Clean crawl: commit the snapshot, then record it in git. A failure in
    // either step is a real, user-visible failure even though the bytes are on
    // disk — fold it into the report so the verdict line and the log both carry
    // it (and never show `failed=0`). Push before `finish` so the summary line
    // counts it.
    if let Err(error) = snapshot.commit() {
        report.failures.push(CrawlFailure {
            url: format!("(commit {name})"),
            message: error,
        });
        progress.finish(&report, false);
        return Outcome::Failed(report);
    }
    if let Err(error) = repository.record_site(name) {
        // Content is on disk; the next sync's preflight recovery absorbs it, so
        // no data is lost — but surface the error so operators notice.
        report.failures.push(CrawlFailure {
            url: format!("(git {name})"),
            message: error,
        });
        progress.finish(&report, false);
        return Outcome::Failed(report);
    }

    progress.finish(&report, true);
    Outcome::Ok(report)
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::config::{Config, SiteConfig};
    use std::collections::{BTreeMap, HashMap};
    use std::fs;
    use std::process::Command;
    use std::sync::{Arc, RwLock};
    use tempfile::tempdir;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::task::JoinHandle;

    type Route = (u16, String, Option<String>);

    fn route(status: u16, body: &str) -> Route {
        (status, body.to_owned(), None)
    }

    fn route_etag(body: &str, etag: &str) -> Route {
        (200, body.to_owned(), Some(etag.to_owned()))
    }

    struct Server {
        origin: String,
        routes: Arc<RwLock<HashMap<String, Route>>>,
        task: JoinHandle<()>,
    }

    impl Drop for Server {
        fn drop(&mut self) {
            self.task.abort();
        }
    }

    async fn server(initial: HashMap<String, Route>) -> Server {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let routes = Arc::new(RwLock::new(initial));
        let task_routes = Arc::clone(&routes);
        let task = tokio::spawn(async move {
            loop {
                let Ok((mut stream, _)) = listener.accept().await else {
                    return;
                };
                let routes = Arc::clone(&task_routes);
                tokio::spawn(async move {
                    let mut buffer = [0_u8; 4096];
                    let Ok(length) = stream.read(&mut buffer).await else {
                        return;
                    };
                    let request = String::from_utf8_lossy(&buffer[..length]);
                    let path = request
                        .lines()
                        .next()
                        .and_then(|line| line.split_whitespace().nth(1))
                        .unwrap_or("/");
                    let if_none_match = request
                        .lines()
                        .find(|line| line.to_ascii_lowercase().starts_with("if-none-match:"))
                        .and_then(|line| line.split_once(':'))
                        .map(|(_, value)| value.trim().to_owned());
                    let (status, body, etag) = routes
                        .read()
                        .unwrap()
                        .get(path)
                        .cloned()
                        .unwrap_or((404, String::new(), None));
                    let etag_header = etag
                        .as_ref()
                        .map(|value| format!("ETag: {value}\r\n"))
                        .unwrap_or_default();
                    let not_modified = matches!(
                        (&if_none_match, &etag),
                        (Some(inm), Some(tag)) if inm == tag
                    );
                    let response = if not_modified {
                        format!(
                            "HTTP/1.1 304 Not Modified\r\n{etag_header}Connection: close\r\n\r\n"
                        )
                    } else {
                        let reason = if status == 200 { "OK" } else { "Error" };
                        format!(
                            "HTTP/1.1 {status} {reason}\r\n{etag_header}Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
                            body.len()
                        )
                    };
                    let _ = stream.write_all(response.as_bytes()).await;
                });
            }
        });
        Server {
            origin: format!("http://{address}"),
            routes,
            task,
        }
    }

    fn config(output: &std::path::Path, server: &Server) -> Config {
        Config {
            output_dir: output.display().to_string(),
            concurrency: 2,
            interval_ms: 0,
            sites: BTreeMap::from([
                (
                    "bad".to_owned(),
                    SiteConfig {
                        url: format!("{}/bad.txt", server.origin),
                    },
                ),
                (
                    "good".to_owned(),
                    SiteConfig {
                        url: format!("{}/good.txt", server.origin),
                    },
                ),
            ]),
        }
    }

    fn git(root: &std::path::Path, args: &[&str]) -> String {
        let output = Command::new("git")
            .current_dir(root)
            .args(args)
            .output()
            .unwrap();
        assert!(output.status.success());
        String::from_utf8(output.stdout).unwrap().trim().to_owned()
    }

    #[tokio::test]
    async fn commits_success_preserves_failure_and_removes_stale_files() {
        let server = server(HashMap::from([
            ("/good.txt".to_owned(), route(200, "[a](/docs/a.md)")),
            ("/bad.txt".to_owned(), route(200, "[fail](/fail.md)")),
            ("/docs/a.md".to_owned(), route(200, "old-a")),
            ("/fail.md".to_owned(), route(500, "")),
        ]))
        .await;
        let directory = tempdir().unwrap();
        let output = directory.path().join("wiki");
        let config_path = directory.path().join("config.toml");
        config(&output, &server).save(&config_path).unwrap();
        fs::create_dir_all(output.join("bad")).unwrap();
        fs::write(output.join("bad/old.md"), "preserve").unwrap();

        assert!(run(&config_path, None, None, None, None).await.is_err());
        assert_eq!(
            fs::read_to_string(output.join("good/docs/a.md")).unwrap(),
            "old-a"
        );
        assert_eq!(
            fs::read_to_string(output.join("bad/old.md")).unwrap(),
            "preserve"
        );
        assert_eq!(git(&output, &["rev-list", "--count", "HEAD"]), "1");
        assert!(git(&output, &["log", "-1", "--format=%s"]).starts_with("chore(sync): good @ "));

        server
            .routes
            .write()
            .unwrap()
            .insert("/good.txt".to_owned(), route(200, "[new](/docs/new.md)"));
        server
            .routes
            .write()
            .unwrap()
            .insert("/docs/new.md".to_owned(), route(200, "new"));
        let config_before = fs::read(&config_path).unwrap();
        run(&config_path, Some("good".to_owned()), Some(1), None, None)
            .await
            .unwrap();
        assert!(!output.join("good/docs/a.md").exists());
        assert_eq!(
            fs::read_to_string(output.join("good/docs/new.md")).unwrap(),
            "new"
        );
        assert_eq!(fs::read(&config_path).unwrap(), config_before);
        assert!(!output.join(".cache").exists());
        let names: Vec<_> = fs::read_dir(&output)
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect();
        // A committed site leaves no staging/backup directory behind; an
        // interrupted site's `.sync.` partial is intentionally kept for resume.
        assert!(
            names.iter().all(|name| {
                let name = name.to_string_lossy();
                !name.contains(".backup.") && !name.starts_with(".good.sync.")
            }),
            "unexpected leftovers: {names:?}"
        );
    }

    #[tokio::test]
    async fn invalid_git_repository_prevents_snapshot_changes() {
        let server = server(HashMap::from([
            ("/good.txt".to_owned(), route(200, "[new](/new.md)")),
            ("/new.md".to_owned(), route(200, "new")),
        ]))
        .await;
        let directory = tempdir().unwrap();
        let output = directory.path().join("wiki");
        fs::create_dir_all(output.join("good")).unwrap();
        fs::write(output.join("good/old.md"), "old").unwrap();
        fs::write(output.join(".git"), "invalid").unwrap();
        let config_path = directory.path().join("config.toml");
        let config = Config {
            output_dir: output.display().to_string(),
            concurrency: 1,
            interval_ms: 0,
            sites: BTreeMap::from([(
                "good".to_owned(),
                SiteConfig {
                    url: format!("{}/good.txt", server.origin),
                },
            )]),
        };
        config.save(&config_path).unwrap();

        assert!(
            run(&config_path, Some("good".to_owned()), None, None, None,)
                .await
                .is_err()
        );
        assert_eq!(
            fs::read_to_string(output.join("good/old.md")).unwrap(),
            "old"
        );
        assert!(!output.join("good/new.md").exists());
    }

    #[tokio::test]
    async fn rejects_empty_and_unknown_site_sets() {
        let directory = tempdir().unwrap();
        let config_path = directory.path().join("config.toml");
        let mut empty = Config {
            output_dir: directory.path().join("wiki").display().to_string(),
            ..Config::default()
        };
        empty.save(&config_path).unwrap();
        assert!(run(&config_path, None, None, None, None).await.is_err());

        empty.sites.insert(
            "known".to_owned(),
            SiteConfig {
                url: "http://127.0.0.1:9/llms.txt".to_owned(),
            },
        );
        empty.save(&config_path).unwrap();
        assert!(
            run(&config_path, Some("unknown".to_owned()), None, None, None,)
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn second_sync_reuses_unchanged_files_via_conditional_requests() {
        let server = server(HashMap::from([
            ("/good.txt".to_owned(), route(200, "[a](/docs/a.md)")),
            ("/docs/a.md".to_owned(), route_etag("body-a", "\"v1\"")),
        ]))
        .await;
        let directory = tempdir().unwrap();
        let output = directory.path().join("wiki");
        let config_path = directory.path().join("config.toml");
        let config = Config {
            output_dir: output.display().to_string(),
            concurrency: 1,
            interval_ms: 0,
            sites: BTreeMap::from([(
                "good".to_owned(),
                SiteConfig {
                    url: format!("{}/good.txt", server.origin),
                },
            )]),
        };
        config.save(&config_path).unwrap();

        // First sync downloads everything and persists the manifest into the site dir.
        run(&config_path, Some("good".to_owned()), None, None, None)
            .await
            .unwrap();
        assert_eq!(
            fs::read_to_string(output.join("good/docs/a.md")).unwrap(),
            "body-a"
        );
        assert!(output.join("good/.llms-wiki.json").exists());

        // Remote body changes but the ETag does not: the server answers 304, so the
        // second sync must reuse the local copy and never see "REMOTE-CHANGED".
        server.routes.write().unwrap().insert(
            "/docs/a.md".to_owned(),
            route_etag("REMOTE-CHANGED", "\"v1\""),
        );
        run(&config_path, Some("good".to_owned()), None, None, None)
            .await
            .unwrap();
        assert_eq!(
            fs::read_to_string(output.join("good/docs/a.md")).unwrap(),
            "body-a"
        );
        assert_eq!(git(&output, &["rev-list", "--count", "HEAD"]), "2");
    }

    #[tokio::test]
    async fn sync_pushes_committed_snapshot_when_url_provided() {
        let server = server(HashMap::from([
            ("/good.txt".to_owned(), route(200, "[a](/docs/a.md)")),
            ("/docs/a.md".to_owned(), route(200, "a")),
        ]))
        .await;
        let directory = tempdir().unwrap();
        let output = directory.path().join("wiki");
        let bare = directory.path().join("remote.git");
        Command::new("git")
            .args(["init", "--quiet", "--bare"])
            .arg(&bare)
            .status()
            .unwrap();
        let config_path = directory.path().join("config.toml");
        let config = Config {
            output_dir: output.display().to_string(),
            concurrency: 1,
            interval_ms: 0,
            sites: BTreeMap::from([(
                "good".to_owned(),
                SiteConfig {
                    url: format!("{}/good.txt", server.origin),
                },
            )]),
        };
        config.save(&config_path).unwrap();
        let url = format!("file://{}", bare.display());

        run(&config_path, None, None, None, Some(url.clone()))
            .await
            .unwrap();
        assert_eq!(
            git(&bare, &["rev-parse", "refs/heads/wiki-data"]),
            git(&output, &["rev-parse", "HEAD"]),
            "wiki-data on the remote must mirror the local HEAD after sync"
        );
    }

    #[tokio::test]
    async fn sync_ignores_push_failure_and_returns_ok() {
        let server = server(HashMap::from([
            ("/good.txt".to_owned(), route(200, "[a](/docs/a.md)")),
            ("/docs/a.md".to_owned(), route(200, "a")),
        ]))
        .await;
        let directory = tempdir().unwrap();
        let output = directory.path().join("wiki");
        let config_path = directory.path().join("config.toml");
        let config = Config {
            output_dir: output.display().to_string(),
            concurrency: 1,
            interval_ms: 0,
            sites: BTreeMap::from([(
                "good".to_owned(),
                SiteConfig {
                    url: format!("{}/good.txt", server.origin),
                },
            )]),
        };
        config.save(&config_path).unwrap();
        let missing = directory.path().join("does-not-exist.git");
        let url = format!("file://{}", missing.display());

        // Unreachable remote must not surface as a sync failure — the local
        // snapshot commit still lands, and the caller sees Ok.
        run(&config_path, None, None, None, Some(url))
            .await
            .unwrap();
        assert_eq!(git(&output, &["rev-list", "--count", "HEAD"]), "1");
    }
}
