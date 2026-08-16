use std::collections::{HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinSet;
use tokio::time::{Instant, sleep_until};
use url::Url;

use crate::discovery::{declared_links, discover};
use crate::http::{FetchOutcome, HttpClient};
use crate::manifest::{MANIFEST_FILE, Manifest, Validator};
use crate::url_map::{AllowedOrigins, CanonicalUrl, LocalPath, PathRegistry};

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Content pages larger than this are almost never real documentation — an
/// oversized single Markdown file usually means a mis-served aggregate or a
/// binary blob. Drop them during the crawl instead of mirroring the bloat. The
/// entry document (`llms.txt` / `llms-full.txt`) is exempt: it carries
/// `output == None`, so it is never subject to this cap.
pub const DEFAULT_MAX_DOCUMENT_BYTES: usize = 3 * 1024 * 1024;

/// Hard ceiling on download slots. Slots are genuinely simultaneous sockets, so
/// an unbounded value is a stability hazard, not a speed knob: it exhausts file
/// descriptors on hosts with a low `ulimit -n` and earns `429` from the doc host,
/// which this crawler treats as a hard sync failure. 64 already saturates any
/// CDN-backed docs site.
pub const MAX_CONCURRENCY: usize = 64;

/// Degraded pages tolerated on a small site, before the ratio below takes over.
/// A handful of pages that keep erroring after retries is a normal upstream dead
/// end — a docs index that still links pages its own server can no longer render
/// (a `5xx` that is really a `404`). Blocking the snapshot on those would freeze
/// the whole site forever, since the next run meets the same dead links.
const DEGRADED_FLOOR: usize = 3;
/// Above the floor, degraded pages are tolerated up to `processed / this`. A
/// flood is an outage or a blanket block, not a dead link: publishing then would
/// commit a snapshot that silently drops real content, so the site fails instead
/// and its last good snapshot survives.
const DEGRADED_RATIO: usize = 100;

#[derive(Clone, Copy, Debug)]
pub struct CrawlOptions {
    /// Download slots. While the queue has work, exactly this many requests are
    /// in flight — it is a true simultaneity level, not a rate limit.
    pub concurrency: usize,
    /// How long a slot rests after finishing one request, before it starts the
    /// next one. Local to the slot: the other slots keep running.
    pub interval: Duration,
    pub timeout: Duration,
    /// Per-document byte cap for content pages; entry documents are exempt.
    pub max_document_bytes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CrawlFailure {
    pub url: String,
    pub message: String,
}

#[derive(Debug, Default)]
pub struct CrawlReport {
    pub downloaded: usize,
    pub resumed: usize,
    pub unchanged: usize,
    pub missing: usize,
    pub ignored: usize,
    /// Content pages dropped for exceeding `max_document_bytes`. Not a failure:
    /// the crawl still succeeds and the exit code is unaffected.
    pub oversize: usize,
    pub failures: Vec<CrawlFailure>,
    /// Content pages that still errored after every retry, kept out of `failures`
    /// so a dead link upstream does not block the snapshot. Each one either
    /// carried its previous copy forward or is absent from this snapshot; the
    /// crawl still succeeds and the exit code is unaffected. Promoted back into
    /// `failures` wholesale when there are too many (see [`degraded_tolerance`]).
    pub degraded: Vec<CrawlFailure>,
    /// URLs that answered 404/410. Kept alongside the `missing` count so the
    /// persisted run log can name the dead links, not just tally them.
    pub missing_urls: Vec<String>,
    /// URLs dropped for exceeding the size cap, kept alongside `oversize` so the
    /// run log can name what was excluded, not just tally it.
    pub oversize_urls: Vec<String>,
}

impl CrawlReport {
    pub fn failed(&self) -> usize {
        self.failures.len()
    }

    pub fn is_success(&self) -> bool {
        self.failures.is_empty()
    }

    /// URLs this crawl reached a verdict on. `ignored` is excluded: those were
    /// filtered before any request, so counting them would inflate the degraded
    /// budget with work that never happened.
    fn processed(&self) -> usize {
        self.downloaded
            + self.resumed
            + self.unchanged
            + self.missing
            + self.oversize
            + self.degraded.len()
    }
}

/// How many degraded pages a crawl of `processed` URLs may publish anyway.
fn degraded_tolerance(processed: usize) -> usize {
    DEGRADED_FLOOR.max(processed / DEGRADED_RATIO)
}

#[derive(Clone, Debug)]
pub enum CrawlEvent {
    /// A request went on the wire. Paired with [`CrawlEvent::Finished`] to bracket
    /// exactly the time a download slot is busy on the network, so an observer's
    /// in-flight count is the true one — the per-outcome events below land later,
    /// once the crawl loop has drained and classified the result.
    Started(String),
    /// The request came back, whatever its outcome.
    Finished(String),
    Downloaded(String),
    Resumed(String),
    Unchanged(String),
    Missing(String),
    Ignored(String),
    Oversize(String),
    Degraded(String),
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
    validator: Option<Validator>,
}

pub async fn crawl(
    entries: &[Url],
    snapshot_root: &Path,
    previous_root: Option<&Path>,
    previous_manifest: &Manifest,
    options: CrawlOptions,
    observer: Arc<dyn CrawlObserver>,
) -> Result<CrawlReport, String> {
    if options.concurrency == 0 {
        return Err("concurrency must be greater than 0".to_owned());
    }
    if entries.is_empty() {
        return Err("crawl needs at least one entry URL".to_owned());
    }
    if !snapshot_root.is_absolute() {
        return Err(format!(
            "snapshot root must be absolute: {}",
            snapshot_root.display()
        ));
    }

    // Seeded with every entry's origin; each entry document expands it once (see
    // the Document branch below) so a site whose entry host differs from its
    // content host still crawls. Shared with the HTTP client's redirect policy.
    let allowed = AllowedOrigins::from_entries(entries);
    let client = HttpClient::new(&allowed, options.timeout)?;
    let mut seen = HashSet::new();
    let mut entry_set = HashSet::new();
    let mut queue = VecDeque::new();
    for entry in entries {
        let canonical = CanonicalUrl::new(entry.clone());
        if !seen.insert(canonical.clone()) {
            continue;
        }
        entry_set.insert(canonical.clone());
        queue.push_back(WorkItem {
            url: canonical,
            output: None,
            validator: None,
        });
    }
    // While any entry document is still unresolved the allow-list can still grow,
    // so content pages discovered by an entry that already landed wait here rather
    // than going on the wire: a page fetched before the last entry expanded the
    // list would silently drop its links to origins only that entry vouches for,
    // and which entry lands first is a network race. Drained below, once the entry
    // batch has fully drained — with a single entry there is nothing to wait for,
    // so the deferral never engages and the crawl behaves exactly as before.
    let mut deferred: VecDeque<WorkItem> = VecDeque::new();
    let mut entries_open = entry_set.len() > 1;
    let mut paths = PathRegistry::default();
    let mut tasks = JoinSet::new();
    let mut report = CrawlReport::default();
    let mut manifest = Manifest::default();
    // Rest deadlines of the download slots freed by finished requests, oldest
    // first. `tasks` holds at most `concurrency` entries, so an occupied slot is
    // exactly one live task and this queue is the pacing state of the free ones:
    // a slot that just finished a request may not start its next one before its
    // deadline. Empty at the start — the first `concurrency` requests fire at
    // once — and never populated when `interval` is zero. It lives on this task
    // (the only place that spawns), so no lock is involved and no slot ever
    // waits on another slot's rest.
    let mut slot_ready: VecDeque<Instant> = VecDeque::new();

    while !queue.is_empty() || !tasks.is_empty() || !deferred.is_empty() {
        // Nothing queued and nothing in flight means every entry document has
        // reached a verdict, so the allow-list is frozen and the content pages they
        // discovered can be released. Phrased as a drain condition rather than an
        // entry counter so a task that dies without returning its item (a panicked
        // fetch) cannot strand the deferred pages.
        if queue.is_empty() && tasks.is_empty() {
            queue.append(&mut deferred);
            entries_open = false;
        }
        while tasks.len() < options.concurrency {
            let Some(item) = queue.pop_front() else {
                break;
            };
            // Resume: a work item whose file already exists in the snapshot was
            // downloaded by an interrupted earlier run and adopted into this one.
            // Reuse it verbatim and re-discover its links, skipping both the
            // network request and the rate-limiting gate. A read failure falls
            // through to a normal fetch, so a corrupt leftover self-heals.
            if let Some(path) = item.output.as_ref()
                && let Ok(target) = path.join_under(snapshot_root)
                && target.is_file()
                && let Ok(body) = tokio::fs::read_to_string(&target).await
            {
                // A leftover partial from a pre-cap run could exceed the limit;
                // remove it so the guarantee "no content file over the cap reaches
                // the snapshot" holds even across the resume seam.
                if body.len() > options.max_document_bytes {
                    let _ = tokio::fs::remove_file(&target).await;
                    observer.event(CrawlEvent::Oversize(item.url.to_string()));
                    report.oversize += 1;
                    report.oversize_urls.push(item.url.to_string());
                    continue;
                }
                if let Some(validator) = previous_manifest.get(&item.url.to_string()) {
                    manifest.insert(item.url.to_string(), validator.clone());
                }
                observer.event(CrawlEvent::Resumed(item.url.to_string()));
                report.resumed += 1;
                let base = item.url.as_url().clone();
                enqueue_discovered(
                    &body,
                    &base,
                    &entry_set,
                    &allowed,
                    previous_root,
                    previous_manifest,
                    &mut seen,
                    &mut paths,
                    sink(entries_open, &mut queue, &mut deferred),
                    &mut report,
                    observer.as_ref(),
                );
                continue;
            }
            let client = client.clone();
            let task_observer = Arc::clone(&observer);
            // The entry document (output == None) is exempt from the cap; only
            // content pages are subject to it.
            let max_bytes = item.output.as_ref().map(|_| options.max_document_bytes);
            // This slot's rest, if it just finished a request. Resting before the
            // request (not after) keeps `Started` equal to the real in-flight
            // count and hands the fetch result to the loop the moment it lands,
            // so discovery never stalls behind another slot's rest.
            let rest_until = slot_ready.pop_front();
            tasks.spawn(async move {
                if let Some(deadline) = rest_until {
                    sleep_until(deadline).await;
                }
                task_observer.event(CrawlEvent::Started(item.url.to_string()));
                let result = client
                    .fetch(&item.url, item.validator.as_ref(), max_bytes)
                    .await;
                task_observer.event(CrawlEvent::Finished(item.url.to_string()));
                // Measured here, not at join time: the rest starts when the
                // request actually ended, so loop-side bookkeeping is never
                // charged against it.
                (item, result, Instant::now())
            });
        }

        let Some(joined) = tasks.join_next().await else {
            continue;
        };
        let (item, outcome, finished_at) = match joined {
            Ok(value) => value,
            Err(error) => {
                // The slot still spent a request, so it still rests.
                rest_slot(&mut slot_ready, Instant::now(), options.interval);
                report.failures.push(CrawlFailure {
                    url: "internal".to_owned(),
                    message: format!("crawler task failed: {error}"),
                });
                continue;
            }
        };
        rest_slot(&mut slot_ready, finished_at, options.interval);

        match outcome {
            Ok(FetchOutcome::Document {
                final_url,
                body,
                validator,
            }) => {
                if let Some(path) = &item.output {
                    if let Err(error) = write_document(snapshot_root, path, &body).await {
                        observer.event(CrawlEvent::Failed(item.url.to_string()));
                        report.failures.push(CrawlFailure {
                            url: item.url.to_string(),
                            message: error,
                        });
                        continue;
                    }
                    manifest.insert(item.url.to_string(), validator);
                    observer.event(CrawlEvent::Downloaded(item.url.to_string()));
                    report.downloaded += 1;
                }

                // Entry documents are the only ones that expand the allow-list, and
                // they do so here — strictly before the discovery below and with no
                // await in between. Content pages spawned in a later iteration
                // therefore observe the frozen set: the deferral above holds them
                // back until the last entry has passed through here.
                if entry_set.contains(&item.url) {
                    for link in declared_links(&body, &final_url) {
                        allowed.allow(&link);
                    }
                }

                enqueue_discovered(
                    &body,
                    &final_url,
                    &entry_set,
                    &allowed,
                    previous_root,
                    previous_manifest,
                    &mut seen,
                    &mut paths,
                    sink(entries_open, &mut queue, &mut deferred),
                    &mut report,
                    observer.as_ref(),
                );
            }
            Ok(FetchOutcome::NotModified { final_url }) => {
                let Some(path) = &item.output else {
                    observer.event(CrawlEvent::Failed(item.url.to_string()));
                    report.failures.push(CrawlFailure {
                        url: item.url.to_string(),
                        message: "unexpected 304 Not Modified for entry document".to_owned(),
                    });
                    continue;
                };
                let body = match reuse_previous(previous_root, snapshot_root, path).await {
                    Ok(body) => body,
                    Err(error) => {
                        observer.event(CrawlEvent::Failed(item.url.to_string()));
                        report.failures.push(CrawlFailure {
                            url: item.url.to_string(),
                            message: error,
                        });
                        continue;
                    }
                };
                // 304 twin of the resume seam: a pre-cap file that still
                // revalidates would otherwise be copied forward forever, keeping
                // its validator and re-304-ing every run. Drop it so the cap holds
                // on this path too — symmetric with the resume branch above.
                if body.len() > options.max_document_bytes {
                    if let Ok(target) = path.join_under(snapshot_root) {
                        let _ = tokio::fs::remove_file(&target).await;
                    }
                    observer.event(CrawlEvent::Oversize(item.url.to_string()));
                    report.oversize += 1;
                    report.oversize_urls.push(item.url.to_string());
                    continue;
                }
                if let Some(validator) = previous_manifest.get(&item.url.to_string()) {
                    manifest.insert(item.url.to_string(), validator.clone());
                }
                observer.event(CrawlEvent::Unchanged(item.url.to_string()));
                report.unchanged += 1;

                enqueue_discovered(
                    &body,
                    &final_url,
                    &entry_set,
                    &allowed,
                    previous_root,
                    previous_manifest,
                    &mut seen,
                    &mut paths,
                    sink(entries_open, &mut queue, &mut deferred),
                    &mut report,
                    observer.as_ref(),
                );
            }
            Ok(FetchOutcome::Missing) => {
                observer.event(CrawlEvent::Missing(item.url.to_string()));
                report.missing += 1;
                report.missing_urls.push(item.url.to_string());
            }
            Ok(FetchOutcome::IgnoredRedirect) => {
                observer.event(CrawlEvent::Ignored(item.url.to_string()));
                report.ignored += 1;
            }
            Ok(FetchOutcome::Oversize { .. }) => {
                // Excluded, not failed: skip the write, the manifest and link
                // discovery so the bloat leaves no trace and never revalidates.
                observer.event(CrawlEvent::Oversize(item.url.to_string()));
                report.oversize += 1;
                report.oversize_urls.push(item.url.to_string());
            }
            Err(message) => {
                // The entry document is load-bearing — it freezes the allow-list
                // and seeds the queue — so losing it still aborts the site.
                let Some(path) = &item.output else {
                    observer.event(CrawlEvent::Failed(item.url.to_string()));
                    report.failures.push(CrawlFailure {
                        url: item.url.to_string(),
                        message,
                    });
                    continue;
                };
                // A content page that survived every retry is a settled verdict.
                // Carry its previous copy forward when there is one — including
                // its links, so the discovered set never shrinks and pages
                // reachable only through it are not deleted from the snapshot —
                // and degrade rather than fail. `degraded_tolerance` below decides
                // whether the run as a whole still deserves to publish.
                if let Some(body) = adopt_previous(
                    previous_root,
                    snapshot_root,
                    path,
                    options.max_document_bytes,
                )
                .await
                {
                    if let Some(validator) = previous_manifest.get(&item.url.to_string()) {
                        manifest.insert(item.url.to_string(), validator.clone());
                    }
                    let base = item.url.as_url().clone();
                    enqueue_discovered(
                        &body,
                        &base,
                        &entry_set,
                        &allowed,
                        previous_root,
                        previous_manifest,
                        &mut seen,
                        &mut paths,
                        sink(entries_open, &mut queue, &mut deferred),
                        &mut report,
                        observer.as_ref(),
                    );
                }
                observer.event(CrawlEvent::Degraded(item.url.to_string()));
                report.degraded.push(CrawlFailure {
                    url: item.url.to_string(),
                    message,
                });
            }
        }
    }

    if report.degraded.len() > degraded_tolerance(report.processed()) {
        report.failures.append(&mut report.degraded);
    }

    if report.is_success() {
        prune_unclaimed(snapshot_root, &paths);
        if let Err(error) = manifest.save(snapshot_root) {
            report.failures.push(CrawlFailure {
                url: "manifest".to_owned(),
                message: error,
            });
        }
    }

    Ok(report)
}

/// Drop staged files no URL of this crawl maps to. An adopted partial carries
/// whatever the interrupted run wrote, including pages the site has since
/// removed and files named by an older mapping rule; publishing those would mix
/// unverified content into the snapshot. Degraded and 404 pages keep their files:
/// their URLs are registered at discovery time, before the fetch verdict.
/// Best-effort — a file that resists deletion stays rather than failing the site.
fn prune_unclaimed(root: &Path, paths: &PathRegistry) {
    prune_directory(root, root, paths);
}

/// Prune `dir` recursively; returns whether it ended up empty, so the parent can
/// remove directories an older naming rule left behind.
fn prune_directory(root: &Path, dir: &Path, paths: &PathRegistry) -> bool {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return false;
    };
    let mut empty = true;
    for entry in entries.flatten() {
        let path = entry.path();
        // `file_type` does not follow symlinks, so a linked directory is treated
        // as a plain unclaimed file and unlinked, never descended into.
        let is_dir = entry.file_type().is_ok_and(|kind| kind.is_dir());
        let removed = if is_dir {
            prune_directory(root, &path, paths) && std::fs::remove_dir(&path).is_ok()
        } else {
            path.strip_prefix(root).is_ok_and(|relative| {
                relative != Path::new(MANIFEST_FILE) && !paths.claims(relative)
            }) && std::fs::remove_file(&path).is_ok()
        };
        empty &= removed;
    }
    empty
}

/// Where newly discovered content pages go: held back in `deferred` while entry
/// documents are still landing, straight into `queue` once the allow-list is frozen.
fn sink<'a>(
    entries_open: bool,
    queue: &'a mut VecDeque<WorkItem>,
    deferred: &'a mut VecDeque<WorkItem>,
) -> &'a mut VecDeque<WorkItem> {
    if entries_open { deferred } else { queue }
}

/// Put the slot freed by a request that finished at `finished_at` on rest until
/// `finished_at + interval`. A zero interval enqueues nothing, so the
/// no-rate-limit path never allocates and never awaits.
fn rest_slot(slot_ready: &mut VecDeque<Instant>, finished_at: Instant, interval: Duration) {
    if !interval.is_zero() {
        slot_ready.push_back(finished_at + interval);
    }
}

/// Discover allowed-origin Markdown links in `body` and enqueue the unseen ones,
/// attaching a cached validator when the file can be revalidated conditionally.
#[allow(clippy::too_many_arguments)]
fn enqueue_discovered(
    body: &str,
    final_url: &Url,
    entries: &HashSet<CanonicalUrl>,
    allowed: &AllowedOrigins,
    previous_root: Option<&Path>,
    previous_manifest: &Manifest,
    seen: &mut HashSet<CanonicalUrl>,
    paths: &mut PathRegistry,
    queue: &mut VecDeque<WorkItem>,
    report: &mut CrawlReport,
    observer: &dyn CrawlObserver,
) {
    for candidate in discover(body, final_url, entries, allowed) {
        if !seen.insert(candidate.clone()) {
            continue;
        }
        match paths.register(&candidate) {
            Ok(output) => {
                let validator =
                    conditional_validator(previous_root, &output, &candidate, previous_manifest);
                queue.push_back(WorkItem {
                    url: candidate,
                    output: Some(output),
                    validator,
                });
            }
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

/// A cached validator is reusable only when the previously downloaded file still
/// exists locally, so a 304 can be satisfied by copying it. Missing file → no
/// validator, forcing a full 200 download.
fn conditional_validator(
    previous_root: Option<&Path>,
    output: &LocalPath,
    url: &CanonicalUrl,
    previous_manifest: &Manifest,
) -> Option<Validator> {
    let previous_root = previous_root?;
    let validator = previous_manifest.get(&url.to_string())?;
    let local = output.join_under(previous_root).ok()?;
    local.is_file().then(|| validator.clone())
}

/// Copy the previously downloaded file into the new snapshot and return its body
/// for link discovery. 304 guarantees the bytes match the remote, so re-running
/// `discover` on the local copy yields the identical allowed-origin link set.
async fn reuse_previous(
    previous_root: Option<&Path>,
    snapshot_root: &Path,
    path: &LocalPath,
) -> Result<String, String> {
    let previous_root =
        previous_root.ok_or_else(|| "304 Not Modified without a previous snapshot".to_owned())?;
    let source = path.join_under(previous_root)?;
    let body = tokio::fs::read_to_string(&source)
        .await
        .map_err(|error| format!("reuse {}: {error}", source.display()))?;
    write_document(snapshot_root, path, &body).await?;
    Ok(body)
}

/// Carry a previously downloaded file into the new snapshot for a page whose
/// fetch failed for good, returning its body for link discovery. Best-effort by
/// design: no previous snapshot, no previous copy, an unreadable file or one past
/// the cap all yield `None`, which just means the page is absent from this
/// snapshot — the caller already counts it as degraded either way.
async fn adopt_previous(
    previous_root: Option<&Path>,
    snapshot_root: &Path,
    path: &LocalPath,
    max_bytes: usize,
) -> Option<String> {
    let source = path.join_under(previous_root?).ok()?;
    let body = tokio::fs::read_to_string(&source).await.ok()?;
    if body.len() > max_bytes {
        return None;
    }
    write_document(snapshot_root, path, &body).await.ok()?;
    Some(body)
}

/// Write `body` to `path` atomically: stage into a sibling `.part` file, then
/// rename over the target. A rename is atomic on the same filesystem, so an
/// interruption never leaves a truncated file at the final path — resume relies
/// on "file exists ⇒ content complete".
pub(crate) async fn write_document(
    root: &Path,
    path: &LocalPath,
    body: &str,
) -> Result<(), String> {
    let target = path.join_under(root)?;
    let parent = target
        .parent()
        .ok_or_else(|| format!("output path has no parent: {}", target.display()))?;
    tokio::fs::create_dir_all(parent)
        .await
        .map_err(|error| format!("create directory {}: {error}", parent.display()))?;
    let mut temporary = target.clone().into_os_string();
    temporary.push(".part");
    let temporary = PathBuf::from(temporary);
    tokio::fs::write(&temporary, body)
        .await
        .map_err(|error| format!("write {}: {error}", temporary.display()))?;
    tokio::fs::rename(&temporary, &target)
        .await
        .map_err(|error| format!("finalize {}: {error}", target.display()))
}

#[cfg(test)]
mod tests {
    use super::{CrawlObserver, CrawlOptions, CrawlReport, NoopObserver, crawl};
    use crate::manifest::{MANIFEST_FILE, Manifest, Validator};
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};
    use tempfile::tempdir;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::task::JoinHandle;
    use url::Url;

    async fn fresh_crawl(
        entry: Url,
        snapshot_root: &std::path::Path,
        options: CrawlOptions,
        observer: Arc<dyn CrawlObserver>,
    ) -> Result<CrawlReport, String> {
        fresh_crawl_all(&[entry], snapshot_root, options, observer).await
    }

    async fn fresh_crawl_all(
        entries: &[Url],
        snapshot_root: &std::path::Path,
        options: CrawlOptions,
        observer: Arc<dyn CrawlObserver>,
    ) -> Result<CrawlReport, String> {
        crawl(
            entries,
            snapshot_root,
            None,
            &Manifest::default(),
            options,
            observer,
        )
        .await
    }

    #[derive(Clone)]
    struct Response {
        status: u16,
        body: Vec<u8>,
        location: Option<String>,
        etag: Option<String>,
        delay: Duration,
    }

    impl Response {
        fn ok(body: &str) -> Self {
            Self {
                status: 200,
                body: body.as_bytes().to_vec(),
                location: None,
                etag: None,
                delay: Duration::ZERO,
            }
        }

        fn status(status: u16) -> Self {
            Self {
                status,
                body: Vec::new(),
                location: None,
                etag: None,
                delay: Duration::ZERO,
            }
        }

        fn redirect(location: String) -> Self {
            Self {
                status: 302,
                body: Vec::new(),
                location: Some(location),
                etag: None,
                delay: Duration::ZERO,
            }
        }

        fn delayed(mut self, delay: Duration) -> Self {
            self.delay = delay;
            self
        }

        fn with_etag(mut self, etag: &str) -> Self {
            self.etag = Some(etag.to_owned());
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
                    let if_none_match = request
                        .lines()
                        .find(|line| line.to_ascii_lowercase().starts_with("if-none-match:"))
                        .and_then(|line| line.split_once(':'))
                        .map(|(_, value)| value.trim().to_owned());
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
                    let etag_header = response
                        .etag
                        .as_ref()
                        .map(|value| format!("ETag: {value}\r\n"))
                        .unwrap_or_default();
                    let not_modified = matches!(
                        (&if_none_match, &response.etag),
                        (Some(inm), Some(etag)) if inm == etag
                    );
                    if not_modified {
                        let head = format!(
                            "HTTP/1.1 304 Not Modified\r\n{etag_header}Connection: close\r\n\r\n"
                        );
                        let _ = stream.write_all(head.as_bytes()).await;
                    } else {
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
                            "HTTP/1.1 {} {}\r\n{}{}Content-Length: {}\r\nConnection: close\r\n\r\n",
                            response.status,
                            reason,
                            location,
                            etag_header,
                            response.body.len()
                        );
                        let _ = stream.write_all(head.as_bytes()).await;
                        let _ = stream.write_all(&response.body).await;
                    }
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
            max_document_bytes: super::DEFAULT_MAX_DOCUMENT_BYTES,
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
        let report = fresh_crawl(
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
    async fn follows_nested_llms_txt_indexes_to_markdown_docs() {
        // Real-world shape (Cloudflare): the entry llms.txt links only to
        // per-section llms.txt indexes, each of which links to the actual .md
        // pages. Both hops must be followed — the entry index alone yields no
        // .md, so a discovery filter that rejects llms.txt downloads nothing.
        let server = server(HashMap::from([
            (
                "/llms.txt".to_owned(),
                Response::ok("[section](/sub/llms.txt)"),
            ),
            (
                "/sub/llms.txt".to_owned(),
                Response::ok("[page](/sub/page.md)"),
            ),
            ("/sub/page.md".to_owned(), Response::ok("body")),
        ]))
        .await;
        let directory = tempdir().unwrap();
        let report = fresh_crawl(
            server.url.clone(),
            directory.path(),
            options(2, Duration::ZERO),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();
        assert!(report.is_success());
        // The nested index is downloaded and re-discovered; its .md follows.
        assert_eq!(report.downloaded, 2);
        assert_eq!(
            std::fs::read_to_string(directory.path().join("sub/llms.txt")).unwrap(),
            "[page](/sub/page.md)"
        );
        assert_eq!(
            std::fs::read_to_string(directory.path().join("sub/page.md")).unwrap(),
            "body"
        );
        // The entry document itself is never written to disk.
        assert!(!directory.path().join("llms.txt").exists());
    }

    #[tokio::test]
    async fn resumes_existing_files_and_rediscovers_their_links() {
        let server = server(HashMap::from([
            (
                "/llms.txt".to_owned(),
                Response::ok("[a](/a.md) [b](/b.md)"),
            ),
            ("/a.md".to_owned(), Response::ok("[c](/c.md)")),
            ("/b.md".to_owned(), Response::ok("B")),
            ("/c.md".to_owned(), Response::ok("C")),
        ]))
        .await;
        let directory = tempdir().unwrap();
        // An interrupted earlier run already downloaded a.md; adopt it verbatim.
        std::fs::write(directory.path().join("a.md"), "[c](/c.md)").unwrap();

        let report = fresh_crawl(
            server.url.clone(),
            directory.path(),
            options(2, Duration::ZERO),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();

        assert!(report.is_success());
        assert_eq!(report.resumed, 1, "a.md is reused from the partial");
        assert_eq!(
            report.downloaded, 2,
            "only b.md and the rediscovered c.md are downloaded"
        );
        let paths: Vec<String> = server
            .requests
            .lock()
            .unwrap()
            .iter()
            .map(|(path, _)| path.clone())
            .collect();
        assert!(
            !paths.iter().any(|path| path == "/a.md"),
            "a resumed file must not hit the network"
        );
        assert!(
            paths.iter().any(|path| path == "/c.md"),
            "links inside a resumed file are still discovered and fetched"
        );
        assert_eq!(
            std::fs::read_to_string(directory.path().join("c.md")).unwrap(),
            "C"
        );
    }

    /// Entry linking `count` delayed content pages, for the pacing tests below.
    async fn paced_server(count: usize, delay: Duration) -> Server {
        let mut routes = HashMap::new();
        let links: String = (0..count)
            .map(|index| format!("[p{index}](/p{index}.md) "))
            .collect();
        routes.insert("/llms.txt".to_owned(), Response::ok(&links));
        for index in 0..count {
            routes.insert(format!("/p{index}.md"), Response::ok("body").delayed(delay));
        }
        server(routes).await
    }

    /// Content request start instants, entry request excluded.
    fn content_starts(server: &Server) -> Vec<Instant> {
        let mut starts: Vec<Instant> = server
            .requests
            .lock()
            .unwrap()
            .iter()
            .filter(|(path, _)| path != "/llms.txt")
            .map(|(_, started)| *started)
            .collect();
        starts.sort_unstable();
        starts
    }

    #[tokio::test]
    async fn saturates_every_download_slot() {
        // Concurrency is a simultaneity level, not a ceiling that a rate gate is
        // free to undershoot: with more queued work than slots, the server MUST
        // observe exactly `concurrency` requests at once.
        let server = paced_server(8, Duration::from_millis(120)).await;
        let directory = tempdir().unwrap();
        let report = fresh_crawl(
            server.url.clone(),
            directory.path(),
            options(4, Duration::from_millis(30)),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();
        assert!(report.is_success());
        assert_eq!(report.downloaded, 8);
        assert_eq!(
            server.max_active.load(Ordering::SeqCst),
            4,
            "all four slots must be in flight together"
        );
    }

    #[tokio::test]
    async fn interval_rests_only_the_slot_that_finished() {
        // The rest is slot-local, never a global start gate: the entry fetch put
        // its own slot on rest, and the remaining slots MUST still fan out
        // together even though the interval dwarfs the request latency. Under the
        // old global gate every start was one interval apart instead.
        let concurrency = 3;
        let interval = Duration::from_millis(400);
        let server = paced_server(3, Duration::from_millis(150)).await;
        let directory = tempdir().unwrap();
        let report = fresh_crawl(
            server.url.clone(),
            directory.path(),
            options(concurrency, interval),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();
        assert!(report.is_success());
        assert_eq!(report.downloaded, 3);
        let starts = content_starts(&server);
        assert_eq!(starts.len(), 3);
        let paired = starts[1].duration_since(starts[0]);
        assert!(
            paired < interval / 2,
            "second start came {paired:?} after the first; the interval must not gate other slots"
        );
        assert!(
            server.max_active.load(Ordering::SeqCst) >= concurrency - 1,
            "every slot except the one resting from the entry fetch must run at once"
        );
    }

    #[tokio::test]
    async fn interval_delays_the_next_request_of_a_slot() {
        // One slot isolates the rest: consecutive starts are separated by the
        // request duration plus the full interval.
        let interval = Duration::from_millis(120);
        let server = paced_server(3, Duration::ZERO).await;
        let directory = tempdir().unwrap();
        let report = fresh_crawl(
            server.url.clone(),
            directory.path(),
            options(1, interval),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();
        assert!(report.is_success());
        assert_eq!(report.downloaded, 3);
        assert_eq!(server.max_active.load(Ordering::SeqCst), 1);
        let starts = content_starts(&server);
        for pair in starts.windows(2) {
            let gap = pair[1].duration_since(pair[0]);
            assert!(
                gap >= interval,
                "slot restarted after {gap:?}, expected at least {interval:?}"
            );
        }
    }

    #[tokio::test]
    async fn zero_interval_keeps_slots_running_without_rest() {
        let server = paced_server(6, Duration::from_millis(60)).await;
        let directory = tempdir().unwrap();
        let report = fresh_crawl(
            server.url.clone(),
            directory.path(),
            options(3, Duration::ZERO),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();
        assert!(report.is_success());
        assert_eq!(report.downloaded, 6);
        assert_eq!(server.max_active.load(Ordering::SeqCst), 3);
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
        let report = fresh_crawl(
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
    async fn crawls_cross_origin_content_the_entry_declares_but_not_origins_only_content_names() {
        // Real-world shape (bun): the entry llms.txt is served on one host but
        // links its `.md` content to a *different* host. The content host must be
        // crawled, yet a third origin that only a content page references — never
        // declared by the entry — must stay out of reach.
        let evil = server(HashMap::from([(
            "/evil.md".to_owned(),
            Response::ok("must not fetch"),
        )]))
        .await;
        let evil_md = evil.url.join("/evil.md").unwrap().to_string();
        let content = server(HashMap::from([
            (
                "/docs/page.md".to_owned(),
                Response::ok(&format!("[deep](/docs/deep.md) [evil]({evil_md})")),
            ),
            ("/docs/deep.md".to_owned(), Response::ok("deep")),
        ]))
        .await;
        let content_page = content.url.join("/docs/page.md").unwrap().to_string();
        let entry = server(HashMap::from([(
            "/llms.txt".to_owned(),
            Response::ok(&format!("[content]({content_page})")),
        )]))
        .await;

        let directory = tempdir().unwrap();
        let report = fresh_crawl(
            entry.url.clone(),
            directory.path(),
            options(2, Duration::ZERO),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();

        assert!(report.is_success());
        // The declared content origin is crawled: page.md plus its same-origin deep.md.
        assert_eq!(report.downloaded, 2);
        assert_eq!(
            std::fs::read_to_string(directory.path().join("docs/deep.md")).unwrap(),
            "deep"
        );
        // The entry document itself is never written.
        assert!(!directory.path().join("llms.txt").exists());
        // A content page cannot expand the allow-list: the third origin is untouched.
        assert_eq!(evil.requests.lock().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn crawls_every_entry_of_a_site_into_one_snapshot() {
        // The common multi-entry shape: one docs host splitting its sections across
        // per-section index paths, with the sections cross-linking each other.
        let server = server(HashMap::from([
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
        let workers = server.url.join("/workers/llms.txt").unwrap();
        let pages = server.url.join("/pages/llms.txt").unwrap();

        let directory = tempdir().unwrap();
        let report = fresh_crawl_all(
            // The repeated entry is dropped, not fetched twice.
            &[workers.clone(), pages.clone(), workers.clone()],
            directory.path(),
            options(4, Duration::ZERO),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();

        assert!(report.is_success());
        assert_eq!(report.downloaded, 2);
        assert_eq!(
            std::fs::read_to_string(directory.path().join("workers/a.md")).unwrap(),
            "a"
        );
        assert_eq!(
            std::fs::read_to_string(directory.path().join("pages/b.md")).unwrap(),
            "b"
        );
        // Cross-linked siblings are entries, not content: neither is written, and
        // the duplicate entry never reached the wire a second time.
        assert!(!directory.path().join("pages/llms.txt").exists());
        assert!(!directory.path().join("workers/llms.txt").exists());
        let requests = server.requests.lock().unwrap();
        assert_eq!(
            requests
                .iter()
                .filter(|(path, _)| path == "/workers/llms.txt")
                .count(),
            1
        );
    }

    #[tokio::test]
    async fn holds_content_pages_until_every_entry_expanded_the_allow_list() {
        // The entry that vouches for the third origin answers last. Its sibling's
        // content page links there, so releasing that page before the slow entry
        // landed would drop the link — and which entry answers first is a network
        // race, so the crawl must not depend on it.
        let third = server(HashMap::from([
            ("/seed.md".to_owned(), Response::ok("seed")),
            ("/x.md".to_owned(), Response::ok("x")),
        ]))
        .await;
        let seed = third.url.join("/seed.md").unwrap().to_string();
        let cross = third.url.join("/x.md").unwrap().to_string();
        let fast = server(HashMap::from([
            ("/llms.txt".to_owned(), Response::ok("[a](/a.md)")),
            ("/a.md".to_owned(), Response::ok(&format!("[x]({cross})"))),
        ]))
        .await;
        let slow = server(HashMap::from([(
            "/llms.txt".to_owned(),
            Response::ok(&format!("[seed]({seed})")).delayed(Duration::from_millis(150)),
        )]))
        .await;

        let directory = tempdir().unwrap();
        let report = fresh_crawl_all(
            &[fast.url.clone(), slow.url.clone()],
            directory.path(),
            options(4, Duration::ZERO),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();

        assert!(report.is_success());
        // a.md, the slow entry's own seed.md, and the cross-origin x.md that only
        // the slow entry's declaration made reachable.
        assert_eq!(report.downloaded, 3, "{report:?}");
        assert_eq!(
            std::fs::read_to_string(directory.path().join("x.md")).unwrap(),
            "x"
        );
        assert_eq!(report.ignored, 0);
    }

    #[tokio::test]
    async fn reports_http_utf8_and_timeout_failures() {
        let server = server(HashMap::from([
            (
                "/llms.txt".to_owned(),
                Response::ok("[bad](/bad.md) [busy](/busy.md) [denied](/denied.md) [utf8](/utf8.md) [slow](/slow.md) [q1](/same.md?q=1) [q2](/same.md?q=2) [gone](/gone.md)"),
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
                    etag: None,
                    delay: Duration::ZERO,
                },
            ),
            (
                "/slow.md".to_owned(),
                Response::ok("slow").delayed(Duration::from_millis(100)),
            ),
            ("/same.md?q=1".to_owned(), Response::ok("one")),
            ("/gone.md".to_owned(), Response::status(500)),
        ]))
        .await;
        let directory = tempdir().unwrap();
        let report = fresh_crawl(
            server.url.clone(),
            directory.path(),
            CrawlOptions {
                concurrency: 4,
                interval: Duration::ZERO,
                timeout: Duration::from_millis(30),
                max_document_bytes: super::DEFAULT_MAX_DOCUMENT_BYTES,
            },
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();
        // Six settled errors is well past `degraded_tolerance`, so they are
        // promoted back into hard failures and nothing publishes.
        assert!(!report.is_success());
        assert!(report.failed() >= 6, "{:?}", report.failures);
        assert!(report.degraded.is_empty());
    }

    #[tokio::test]
    async fn query_variants_land_side_by_side_and_stale_files_are_pruned() {
        // Same path, different content per query — the shape learn.chatgpt.com
        // serves with `?surface=cli` / `?surface=ide`.
        let server = server(HashMap::from([
            (
                "/llms.txt".to_owned(),
                Response::ok("[cli](/docs/x.md?surface=cli) [ide](/docs/x.md?surface=ide)"),
            ),
            ("/docs/x.md?surface=cli".to_owned(), Response::ok("cli")),
            ("/docs/x.md?surface=ide".to_owned(), Response::ok("ide")),
        ]))
        .await;
        let directory = tempdir().unwrap();
        let root = directory.path();
        // What an interrupted older run left staged: the same page under the old
        // query-less name, a page the site no longer links, and a manifest.
        std::fs::create_dir_all(root.join("docs/gone")).unwrap();
        std::fs::write(root.join("docs/x.md"), "stale").unwrap();
        std::fs::write(root.join("docs/gone/old.md"), "stale").unwrap();
        std::fs::write(root.join(MANIFEST_FILE), "{}").unwrap();

        let report = fresh_crawl(
            server.url.clone(),
            root,
            options(2, Duration::ZERO),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();

        assert!(report.is_success());
        assert_eq!(report.downloaded, 2);
        assert_eq!(
            std::fs::read_to_string(root.join("docs/x__surface=cli.md")).unwrap(),
            "cli"
        );
        assert_eq!(
            std::fs::read_to_string(root.join("docs/x__surface=ide.md")).unwrap(),
            "ide"
        );
        assert!(!root.join("docs/x.md").exists(), "old name is pruned");
        assert!(!root.join("docs/gone").exists(), "emptied dir is pruned");
        assert!(
            root.join(MANIFEST_FILE).exists(),
            "manifest is never pruned"
        );
    }

    #[tokio::test]
    async fn pruning_keeps_files_the_resumed_partial_supplies() {
        // Resume reuses a staged file instead of re-fetching it; its URL is still
        // registered, so the sweep must leave it alone.
        let server = server(HashMap::from([
            (
                "/llms.txt".to_owned(),
                Response::ok("[a](/a.md) [b](/b.md)"),
            ),
            ("/a.md".to_owned(), Response::ok("a")),
            ("/b.md".to_owned(), Response::ok("b")),
        ]))
        .await;
        let directory = tempdir().unwrap();
        let root = directory.path();
        std::fs::write(root.join("b.md"), "already downloaded").unwrap();

        let report = fresh_crawl(
            server.url.clone(),
            root,
            options(2, Duration::ZERO),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();

        assert!(report.is_success());
        assert_eq!((report.downloaded, report.resumed), (1, 1));
        assert_eq!(
            std::fs::read_to_string(root.join("b.md")).unwrap(),
            "already downloaded"
        );
    }

    #[test]
    fn degraded_tolerance_floors_small_sites_and_scales_with_large_ones() {
        assert_eq!(super::degraded_tolerance(0), 3);
        assert_eq!(super::degraded_tolerance(300), 3);
        assert_eq!(super::degraded_tolerance(700), 7);
    }

    #[tokio::test]
    async fn spends_the_retry_budget_before_degrading_a_page() {
        let server = server(HashMap::from([
            (
                "/llms.txt".to_owned(),
                Response::ok("[ok](/ok.md) [dead](/dead.md)"),
            ),
            ("/ok.md".to_owned(), Response::ok("fine")),
            ("/dead.md".to_owned(), Response::status(500)),
        ]))
        .await;
        let directory = tempdir().unwrap();
        let report = fresh_crawl(
            server.url.clone(),
            directory.path(),
            options(2, Duration::ZERO),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();

        // One dead page is inside the tolerance: the crawl succeeds, the page is
        // simply absent (no previous snapshot to carry forward from).
        assert!(report.is_success(), "{:?}", report.failures);
        assert_eq!(report.downloaded, 1);
        assert_eq!(report.degraded.len(), 1);
        assert!(!directory.path().join("dead.md").exists());

        let requests = server.requests.lock().unwrap();
        let attempts = |path: &str| requests.iter().filter(|(seen, _)| seen == path).count();
        // The 5xx was given every attempt before being written off…
        assert_eq!(attempts("/dead.md"), crate::http::MAX_ATTEMPTS);
        // …while a page that answered first time cost exactly one request.
        assert_eq!(attempts("/ok.md"), 1);
    }

    #[tokio::test]
    async fn reports_network_redirect_loop_and_file_failures() {
        let closed = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let closed_url =
            Url::parse(&format!("http://{}/llms.txt", closed.local_addr().unwrap())).unwrap();
        drop(closed);
        let directory = tempdir().unwrap();
        let network = fresh_crawl(
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
        let redirect = fresh_crawl(
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
        let file = fresh_crawl(
            server.url.clone(),
            directory.path(),
            options(1, Duration::ZERO),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();
        assert_eq!(file.failed(), 1);
    }

    #[tokio::test]
    async fn revalidates_conditionally_reuses_304_and_rewrites_manifest() {
        let server = server(HashMap::from([
            (
                "/llms.txt".to_owned(),
                Response::ok("[a](/docs/a.md) [b](/docs/b.md)"),
            ),
            (
                "/docs/a.md".to_owned(),
                Response::ok("REMOTE-A-MUST-NOT-DOWNLOAD").with_etag("\"v1\""),
            ),
            (
                "/docs/b.md".to_owned(),
                Response::ok("fresh-b").with_etag("\"v2\""),
            ),
        ]))
        .await;

        // Previous snapshot on disk: a.md unchanged (etag v1), b.md stale (cached etag differs).
        let previous = tempdir().unwrap();
        std::fs::create_dir_all(previous.path().join("docs")).unwrap();
        std::fs::write(
            previous.path().join("docs/a.md"),
            "CACHED-A [c](/docs/c.md)",
        )
        .unwrap();
        std::fs::write(previous.path().join("docs/b.md"), "stale-b").unwrap();

        let a_url = server.url.join("/docs/a.md").unwrap().to_string();
        let b_url = server.url.join("/docs/b.md").unwrap().to_string();
        let mut previous_manifest = Manifest::default();
        previous_manifest.insert(
            a_url.clone(),
            Validator {
                etag: Some("\"v1\"".to_owned()),
                last_modified: None,
            },
        );
        previous_manifest.insert(
            b_url.clone(),
            Validator {
                etag: Some("\"stale\"".to_owned()),
                last_modified: None,
            },
        );

        let snapshot = tempdir().unwrap();
        let report = crawl(
            std::slice::from_ref(&server.url),
            snapshot.path(),
            Some(previous.path()),
            &previous_manifest,
            options(2, Duration::ZERO),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();

        assert!(report.is_success());
        assert_eq!(report.unchanged, 1, "a.md revalidated via 304");
        assert_eq!(report.downloaded, 1, "b.md re-downloaded on etag mismatch");
        // 304 reuses the local copy, never the remote body.
        assert_eq!(
            std::fs::read_to_string(snapshot.path().join("docs/a.md")).unwrap(),
            "CACHED-A [c](/docs/c.md)"
        );
        assert_eq!(
            std::fs::read_to_string(snapshot.path().join("docs/b.md")).unwrap(),
            "fresh-b"
        );
        // Links discovered from the reused local body keep the crawl going: /docs/c.md is 404.
        assert_eq!(report.missing, 1);
        // New manifest: a.md keeps v1, b.md updates to the server's v2.
        let written = Manifest::load(snapshot.path());
        assert_eq!(written.get(&a_url).unwrap().etag.as_deref(), Some("\"v1\""));
        assert_eq!(written.get(&b_url).unwrap().etag.as_deref(), Some("\"v2\""));
    }

    fn capped_options(max_document_bytes: usize) -> CrawlOptions {
        CrawlOptions {
            concurrency: 2,
            interval: Duration::ZERO,
            timeout: Duration::from_secs(2),
            max_document_bytes,
        }
    }

    #[tokio::test]
    async fn drops_oversize_content_pages_exempts_the_entry_and_continues() {
        // The entry (33 bytes) is larger than the 16-byte cap yet exempt, so its
        // links are still discovered; big.md (64 bytes) is dropped, small.md kept.
        let server = server(HashMap::from([
            (
                "/llms.txt".to_owned(),
                Response::ok("[small](/small.md) [big](/big.md)"),
            ),
            ("/small.md".to_owned(), Response::ok("tiny")),
            ("/big.md".to_owned(), Response::ok(&"x".repeat(64))),
        ]))
        .await;
        let directory = tempdir().unwrap();
        let report = fresh_crawl(
            server.url.clone(),
            directory.path(),
            capped_options(16),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();
        assert!(
            report.is_success(),
            "oversize is an exclusion, not a failure"
        );
        assert_eq!(report.downloaded, 1, "only small.md is kept");
        assert_eq!(report.oversize, 1);
        assert_eq!(report.oversize_urls.len(), 1);
        assert!(report.oversize_urls[0].ends_with("/big.md"));
        assert!(
            !directory.path().join("big.md").exists(),
            "the oversized page never lands on disk"
        );
        assert_eq!(
            std::fs::read_to_string(directory.path().join("small.md")).unwrap(),
            "tiny"
        );
    }

    #[tokio::test]
    async fn resume_seam_drops_oversize_leftovers_without_a_network_hit() {
        let server = server(HashMap::from([
            ("/llms.txt".to_owned(), Response::ok("[big](/big.md)")),
            ("/big.md".to_owned(), Response::ok("MUST-NOT-FETCH")),
        ]))
        .await;
        let directory = tempdir().unwrap();
        // A leftover partial from a pre-cap run: an oversized file already on disk.
        std::fs::write(directory.path().join("big.md"), "y".repeat(64)).unwrap();
        let report = fresh_crawl(
            server.url.clone(),
            directory.path(),
            capped_options(16),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();
        assert!(report.is_success());
        assert_eq!(report.oversize, 1);
        assert_eq!(report.resumed, 0, "the leftover is dropped, not resumed");
        assert!(
            !directory.path().join("big.md").exists(),
            "the oversize leftover is removed from the snapshot"
        );
        let paths: Vec<String> = server
            .requests
            .lock()
            .unwrap()
            .iter()
            .map(|(path, _)| path.clone())
            .collect();
        assert!(
            !paths.iter().any(|path| path == "/big.md"),
            "a resumed file must not hit the network, even when dropped"
        );
    }

    #[tokio::test]
    async fn drops_oversize_on_304_revalidation_without_carrying_it_forward() {
        let server = server(HashMap::from([
            ("/llms.txt".to_owned(), Response::ok("[big](/docs/big.md)")),
            (
                "/docs/big.md".to_owned(),
                Response::ok("REMOTE").with_etag("\"v1\""),
            ),
        ]))
        .await;

        // Previous snapshot: an oversized big.md left by a pre-cap run that still
        // revalidates (etag v1), so the crawl takes the 304 reuse path.
        let previous = tempdir().unwrap();
        std::fs::create_dir_all(previous.path().join("docs")).unwrap();
        std::fs::write(previous.path().join("docs/big.md"), "z".repeat(64)).unwrap();

        let big_url = server.url.join("/docs/big.md").unwrap().to_string();
        let mut previous_manifest = Manifest::default();
        previous_manifest.insert(
            big_url.clone(),
            Validator {
                etag: Some("\"v1\"".to_owned()),
                last_modified: None,
            },
        );

        let snapshot = tempdir().unwrap();
        let report = crawl(
            std::slice::from_ref(&server.url),
            snapshot.path(),
            Some(previous.path()),
            &previous_manifest,
            capped_options(16),
            Arc::new(NoopObserver),
        )
        .await
        .unwrap();

        assert!(report.is_success());
        assert_eq!(
            report.oversize, 1,
            "the revalidated oversize file is dropped"
        );
        assert_eq!(report.unchanged, 0);
        assert_eq!(report.downloaded, 0);
        assert!(
            !snapshot.path().join("docs/big.md").exists(),
            "the oversize file is not carried into the new snapshot"
        );
        // Its validator is not re-persisted, so it re-fetches (and re-drops) next
        // run instead of 304-ing forever.
        let written = Manifest::load(snapshot.path());
        assert!(written.get(&big_url).is_none());
    }
}
