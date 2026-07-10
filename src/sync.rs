use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use crate::config::Config;
use crate::crawler::{CrawlOptions, CrawlReport, DEFAULT_TIMEOUT, crawl};
use crate::git::Repository;
use crate::progress::SyncProgress;
use crate::site::parse_entry_url;
use crate::snapshot::Snapshot;

pub async fn run(
    config_path: &Path,
    requested_site: Option<String>,
    concurrency: Option<usize>,
    interval: Option<Duration>,
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
    let mut failures = Vec::new();
    let mut successes = Vec::new();

    for (name, site) in targets {
        let entry = match parse_entry_url(&site.url) {
            Ok(entry) => entry,
            Err(error) => {
                failures.push(format!("{name}: {error}"));
                continue;
            }
        };
        let snapshot = match Snapshot::new(&output_root, &name) {
            Ok(snapshot) => snapshot,
            Err(error) => {
                failures.push(format!("{name}: {error}"));
                continue;
            }
        };
        let progress = Arc::new(SyncProgress::new(&name));
        let observer: Arc<dyn crate::crawler::CrawlObserver> = progress.clone();
        let report = match crawl(entry, snapshot.path(), options, observer).await {
            Ok(report) => report,
            Err(error) => {
                let report = CrawlReport::default();
                progress.finish(&report, false);
                failures.push(format!("{name}: {error}"));
                continue;
            }
        };

        if report.is_success() {
            match snapshot.commit() {
                Ok(()) => {
                    progress.finish(&report, true);
                    successes.push(name);
                }
                Err(error) => {
                    progress.finish(&report, false);
                    failures.push(format!("{name}: {error}"));
                }
            }
        } else {
            progress.finish(&report, false);
            failures.push(format_report_failure(&name, &report));
        }
    }

    let git_failure = repository.record_sync(&successes).err();

    let site_failure = if failures.is_empty() {
        None
    } else {
        Some(format!(
            "{} site(s) failed: {}",
            failures.len(),
            failures.join("; ")
        ))
    };
    match (site_failure, git_failure) {
        (None, None) => Ok(()),
        (Some(error), None) | (None, Some(error)) => Err(error),
        (Some(site_error), Some(git_error)) => Err(format!("{site_error}; {git_error}")),
    }
}

fn format_report_failure(site: &str, report: &CrawlReport) -> String {
    let details = report
        .failures
        .iter()
        .take(5)
        .map(|failure| format!("{}: {}", failure.url, failure.message))
        .collect::<Vec<_>>()
        .join(", ");
    let omitted = report.failures.len().saturating_sub(5);
    if omitted == 0 {
        format!("{site}: {details}")
    } else {
        format!("{site}: {details}, and {omitted} more")
    }
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

    struct Server {
        origin: String,
        routes: Arc<RwLock<HashMap<String, (u16, String)>>>,
        task: JoinHandle<()>,
    }

    impl Drop for Server {
        fn drop(&mut self) {
            self.task.abort();
        }
    }

    async fn server(initial: HashMap<String, (u16, String)>) -> Server {
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
                    let (status, body) = routes
                        .read()
                        .unwrap()
                        .get(path)
                        .cloned()
                        .unwrap_or((404, String::new()));
                    let reason = if status == 200 { "OK" } else { "Error" };
                    let response = format!(
                        "HTTP/1.1 {status} {reason}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                        body.len()
                    );
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
            ("/good.txt".to_owned(), (200, "[a](/docs/a.md)".to_owned())),
            ("/bad.txt".to_owned(), (200, "[fail](/fail.md)".to_owned())),
            ("/docs/a.md".to_owned(), (200, "old-a".to_owned())),
            ("/fail.md".to_owned(), (500, String::new())),
        ]))
        .await;
        let directory = tempdir().unwrap();
        let output = directory.path().join("wiki");
        let config_path = directory.path().join("config.toml");
        config(&output, &server).save(&config_path).unwrap();
        fs::create_dir_all(output.join("bad")).unwrap();
        fs::write(output.join("bad/old.md"), "preserve").unwrap();

        assert!(run(&config_path, None, None, None).await.is_err());
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

        server.routes.write().unwrap().insert(
            "/good.txt".to_owned(),
            (200, "[new](/docs/new.md)".to_owned()),
        );
        server
            .routes
            .write()
            .unwrap()
            .insert("/docs/new.md".to_owned(), (200, "new".to_owned()));
        let config_before = fs::read(&config_path).unwrap();
        run(&config_path, Some("good".to_owned()), Some(1), None)
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
        assert!(
            names
                .iter()
                .all(|name| name == ".git" || !name.to_string_lossy().starts_with('.'))
        );
    }

    #[tokio::test]
    async fn invalid_git_repository_prevents_snapshot_changes() {
        let server = server(HashMap::from([
            ("/good.txt".to_owned(), (200, "[new](/new.md)".to_owned())),
            ("/new.md".to_owned(), (200, "new".to_owned())),
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
            run(&config_path, Some("good".to_owned()), None, None)
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
        assert!(run(&config_path, None, None, None).await.is_err());

        empty.sites.insert(
            "known".to_owned(),
            SiteConfig {
                url: "http://127.0.0.1:9/llms.txt".to_owned(),
            },
        );
        empty.save(&config_path).unwrap();
        assert!(
            run(&config_path, Some("unknown".to_owned()), None, None)
                .await
                .is_err()
        );
    }
}
