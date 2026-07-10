use std::collections::{HashSet, VecDeque};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use tokio::time::{Instant, sleep_until};
use url::Url;

use crate::discovery::discover;
use crate::http::{FetchOutcome, HttpClient};
use crate::url_map::{CanonicalUrl, LocalPath, PathRegistry};

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Clone, Copy, Debug)]
pub struct CrawlOptions {
    pub concurrency: usize,
    pub interval: Duration,
    pub timeout: Duration,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CrawlFailure {
    pub url: String,
    pub message: String,
}

#[derive(Debug, Default)]
pub struct CrawlReport {
    pub downloaded: usize,
    pub missing: usize,
    pub ignored: usize,
    pub failures: Vec<CrawlFailure>,
}

impl CrawlReport {
    pub fn failed(&self) -> usize {
        self.failures.len()
    }

    pub fn is_success(&self) -> bool {
        self.failures.is_empty()
    }
}

#[derive(Clone, Debug)]
pub enum CrawlEvent {
    Started(String),
    Downloaded(String),
    Missing(String),
    Ignored(String),
    Failed(String),
}

pub trait CrawlObserver: Send + Sync {
    fn event(&self, event: CrawlEvent);
}

pub struct NoopObserver;

impl CrawlObserver for NoopObserver {
    fn event(&self, _event: CrawlEvent) {}
}

#[derive(Clone)]
struct WorkItem {
    url: CanonicalUrl,
    output: Option<LocalPath>,
}

struct RequestGate {
    interval: Duration,
    next: Mutex<Instant>,
}

impl RequestGate {
    fn new(interval: Duration) -> Self {
        Self {
            interval,
            next: Mutex::new(Instant::now()),
        }
    }

    async fn wait(&self) {
        let mut next = self.next.lock().await;
        let now = Instant::now();
        if *next > now {
            sleep_until(*next).await;
        }
        *next = Instant::now() + self.interval;
    }
}

pub async fn crawl(
    entry: Url,
    snapshot_root: &Path,
    options: CrawlOptions,
    observer: Arc<dyn CrawlObserver>,
) -> Result<CrawlReport, String> {
    if options.concurrency == 0 {
        return Err("concurrency must be greater than 0".to_owned());
    }
    if !snapshot_root.is_absolute() {
        return Err(format!(
            "snapshot root must be absolute: {}",
            snapshot_root.display()
        ));
    }

    let client = HttpClient::new(&entry, options.timeout)?;
    let gate = Arc::new(RequestGate::new(options.interval));
    let canonical_entry = CanonicalUrl::new(entry.clone());
    let mut seen = HashSet::from([canonical_entry.clone()]);
    let mut paths = PathRegistry::default();
    let mut queue = VecDeque::from([WorkItem {
        url: canonical_entry,
        output: None,
    }]);
    let mut tasks = JoinSet::new();
    let mut report = CrawlReport::default();

    while !queue.is_empty() || !tasks.is_empty() {
        while tasks.len() < options.concurrency {
            let Some(item) = queue.pop_front() else {
                break;
            };
            let client = client.clone();
            let gate = Arc::clone(&gate);
            let task_observer = Arc::clone(&observer);
            tasks.spawn(async move {
                gate.wait().await;
                task_observer.event(CrawlEvent::Started(item.url.to_string()));
                let result = client.fetch(&item.url).await;
                (item, result)
            });
        }

        let Some(joined) = tasks.join_next().await else {
            continue;
        };
        let (item, outcome) = match joined {
            Ok(value) => value,
            Err(error) => {
                report.failures.push(CrawlFailure {
                    url: "internal".to_owned(),
                    message: format!("crawler task failed: {error}"),
                });
                continue;
            }
        };

        match outcome {
            Ok(FetchOutcome::Document { final_url, body }) => {
                if let Some(path) = &item.output {
                    if let Err(error) = write_document(snapshot_root, path, &body).await {
                        observer.event(CrawlEvent::Failed(item.url.to_string()));
                        report.failures.push(CrawlFailure {
                            url: item.url.to_string(),
                            message: error,
                        });
                        continue;
                    }
                    observer.event(CrawlEvent::Downloaded(item.url.to_string()));
                    report.downloaded += 1;
                }

                for candidate in discover(&body, &final_url, &entry) {
                    if !seen.insert(candidate.clone()) {
                        continue;
                    }
                    match paths.register(&candidate) {
                        Ok(output) => queue.push_back(WorkItem {
                            url: candidate,
                            output: Some(output),
                        }),
                        Err(error) => {
                            observer.event(CrawlEvent::Failed(candidate.to_string()));
                            report.failures.push(CrawlFailure {
                                url: candidate.to_string(),
                                message: error,
                            });
                        }
                    }
                }
            }
            Ok(FetchOutcome::Missing) => {
                observer.event(CrawlEvent::Missing(item.url.to_string()));
                report.missing += 1;
            }
            Ok(FetchOutcome::IgnoredRedirect) => {
                observer.event(CrawlEvent::Ignored(item.url.to_string()));
                report.ignored += 1;
            }
            Err(message) => {
                observer.event(CrawlEvent::Failed(item.url.to_string()));
                report.failures.push(CrawlFailure {
                    url: item.url.to_string(),
                    message,
                });
            }
        }
    }

    Ok(report)
}

async fn write_document(root: &Path, path: &LocalPath, body: &str) -> Result<(), String> {
    let target = path.join_under(root)?;
    let parent = target
        .parent()
        .ok_or_else(|| format!("output path has no parent: {}", target.display()))?;
    tokio::fs::create_dir_all(parent)
        .await
        .map_err(|error| format!("create directory {}: {error}", parent.display()))?;
    tokio::fs::write(&target, body)
        .await
        .map_err(|error| format!("write {}: {error}", target.display()))
}

#[cfg(test)]
mod tests {
    use super::{CrawlOptions, NoopObserver, crawl};
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};
    use tempfile::tempdir;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::task::JoinHandle;
    use url::Url;

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

        fn redirect(location: String) -> Self {
            Self {
                status: 302,
                body: Vec::new(),
                location: Some(location),
                delay: Duration::ZERO,
            }
        }

        fn delayed(mut self, delay: Duration) -> Self {
            self.delay = delay;
            self
        }
    }

    struct Server {
        url: Url,
        requests: Arc<Mutex<Vec<(String, Instant)>>>,
        max_active: Arc<AtomicUsize>,
        task: JoinHandle<()>,
    }

    impl Drop for Server {
        fn drop(&mut self) {
            self.task.abort();
        }
    }

    async fn server(routes: HashMap<String, Response>) -> Server {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let routes = Arc::new(routes);
        let requests = Arc::new(Mutex::new(Vec::new()));
        let active = Arc::new(AtomicUsize::new(0));
        let max_active = Arc::new(AtomicUsize::new(0));
        let task_requests = Arc::clone(&requests);
        let task_active = Arc::clone(&active);
        let task_max = Arc::clone(&max_active);
        let task = tokio::spawn(async move {
            loop {
                let Ok((mut stream, _)) = listener.accept().await else {
                    return;
                };
                let routes = Arc::clone(&routes);
                let requests = Arc::clone(&task_requests);
                let active = Arc::clone(&task_active);
                let max_active = Arc::clone(&task_max);
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
                    requests
                        .lock()
                        .unwrap()
                        .push((path.clone(), Instant::now()));
                    let current = active.fetch_add(1, Ordering::SeqCst) + 1;
                    max_active.fetch_max(current, Ordering::SeqCst);
                    let response = routes
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
                    active.fetch_sub(1, Ordering::SeqCst);
                });
            }
        });
        Server {
            url: Url::parse(&format!("http://{address}/llms.txt")).unwrap(),
            requests,
            max_active,
            task,
        }
    }

    fn options(concurrency: usize, interval: Duration) -> CrawlOptions {
        CrawlOptions {
            concurrency,
            interval,
            timeout: Duration::from_secs(2),
        }
    }

    #[tokio::test]
    async fn recursively_downloads_deduplicates_and_skips_missing() {
        let server = server(HashMap::from([
            (
                "/llms.txt".to_owned(),
                Response::ok("[a](/docs/a.md) [gone](/gone.md) [old](/old.md)"),
            ),
            (
                "/docs/a.md".to_owned(),
                Response::ok("[b](b.md) [again](/docs/a.md)"),
            ),
            ("/docs/b.md".to_owned(), Response::ok("done")),
            ("/gone.md".to_owned(), Response::status(404)),
            ("/old.md".to_owned(), Response::status(410)),
        ]))
        .await;
        let directory = tempdir().unwrap();
        let report = crawl(
            server.url.clone(),
            directory.path(),
            options(2, Duration::ZERO),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();
        assert!(report.is_success());
        assert_eq!(report.downloaded, 2);
        assert_eq!(report.missing, 2);
        assert_eq!(
            std::fs::read_to_string(directory.path().join("docs/a.md")).unwrap(),
            "[b](b.md) [again](/docs/a.md)"
        );
        assert_eq!(
            std::fs::read_to_string(directory.path().join("docs/b.md")).unwrap(),
            "done"
        );
        assert!(!directory.path().join("llms.txt").exists());
    }

    #[tokio::test]
    async fn enforces_concurrency_and_global_start_interval() {
        let links = "[a](/a.md) [b](/b.md) [c](/c.md)";
        let delay = Duration::from_millis(140);
        let server = server(HashMap::from([
            ("/llms.txt".to_owned(), Response::ok(links)),
            ("/a.md".to_owned(), Response::ok("a").delayed(delay)),
            ("/b.md".to_owned(), Response::ok("b").delayed(delay)),
            ("/c.md".to_owned(), Response::ok("c").delayed(delay)),
        ]))
        .await;
        let directory = tempdir().unwrap();
        let report = crawl(
            server.url.clone(),
            directory.path(),
            options(2, Duration::from_millis(40)),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();
        assert!(report.is_success());
        assert!(server.max_active.load(Ordering::SeqCst) <= 2);
        let starts: Vec<_> = server
            .requests
            .lock()
            .unwrap()
            .iter()
            .map(|(_, started)| *started)
            .collect();
        for pair in starts.windows(2) {
            assert!(pair[1].duration_since(pair[0]) >= Duration::from_millis(35));
        }
    }

    #[tokio::test]
    async fn follows_same_origin_and_ignores_cross_origin_redirects() {
        let other = server(HashMap::from([(
            "/target.md".to_owned(),
            Response::ok("must not fetch"),
        )]))
        .await;
        let target = other.url.join("target.md").unwrap().to_string();
        let server = server(HashMap::from([
            (
                "/llms.txt".to_owned(),
                Response::ok("[same](/same.md) [cross](/cross.md)"),
            ),
            (
                "/same.md".to_owned(),
                Response::redirect("/final.md".to_owned()),
            ),
            ("/final.md".to_owned(), Response::ok("final")),
            ("/cross.md".to_owned(), Response::redirect(target)),
        ]))
        .await;
        let directory = tempdir().unwrap();
        let report = crawl(
            server.url.clone(),
            directory.path(),
            options(2, Duration::ZERO),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();
        assert!(report.is_success());
        assert_eq!(report.downloaded, 1);
        assert_eq!(report.ignored, 1);
        assert_eq!(other.requests.lock().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn reports_http_utf8_timeout_and_path_collision_failures() {
        let server = server(HashMap::from([
            (
                "/llms.txt".to_owned(),
                Response::ok("[bad](/bad.md) [busy](/busy.md) [denied](/denied.md) [utf8](/utf8.md) [slow](/slow.md) [q1](/same.md?q=1) [q2](/same.md?q=2)"),
            ),
            ("/bad.md".to_owned(), Response::status(500)),
            ("/busy.md".to_owned(), Response::status(429)),
            ("/denied.md".to_owned(), Response::status(403)),
            (
                "/utf8.md".to_owned(),
                Response {
                    status: 200,
                    body: vec![0xff],
                    location: None,
                    delay: Duration::ZERO,
                },
            ),
            (
                "/slow.md".to_owned(),
                Response::ok("slow").delayed(Duration::from_millis(100)),
            ),
            ("/same.md?q=1".to_owned(), Response::ok("one")),
        ]))
        .await;
        let directory = tempdir().unwrap();
        let report = crawl(
            server.url.clone(),
            directory.path(),
            CrawlOptions {
                concurrency: 4,
                interval: Duration::ZERO,
                timeout: Duration::from_millis(30),
            },
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();
        assert!(!report.is_success());
        assert!(report.failed() >= 6, "{:?}", report.failures);
    }

    #[tokio::test]
    async fn reports_network_redirect_loop_and_file_failures() {
        let closed = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let closed_url =
            Url::parse(&format!("http://{}/llms.txt", closed.local_addr().unwrap())).unwrap();
        drop(closed);
        let directory = tempdir().unwrap();
        let network = crawl(
            closed_url,
            directory.path(),
            options(1, Duration::ZERO),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();
        assert_eq!(network.failed(), 1);

        let looping = server(HashMap::from([(
            "/llms.txt".to_owned(),
            Response::redirect("/llms.txt".to_owned()),
        )]))
        .await;
        let directory = tempdir().unwrap();
        let redirect = crawl(
            looping.url.clone(),
            directory.path(),
            options(1, Duration::ZERO),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();
        assert_eq!(redirect.failed(), 1);

        let server = server(HashMap::from([
            (
                "/llms.txt".to_owned(),
                Response::ok("[document](/docs/a.md)"),
            ),
            ("/docs/a.md".to_owned(), Response::ok("content")),
        ]))
        .await;
        let directory = tempdir().unwrap();
        std::fs::write(directory.path().join("docs"), "not a directory").unwrap();
        let file = crawl(
            server.url.clone(),
            directory.path(),
            options(1, Duration::ZERO),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();
        assert_eq!(file.failed(), 1);
    }
}
