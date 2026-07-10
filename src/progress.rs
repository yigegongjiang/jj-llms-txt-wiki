use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use std::io::IsTerminal;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use url::Url;

use crate::crawler::{CrawlEvent, CrawlObserver, CrawlReport};

/// How much per-request detail `sync` prints. The summary line always prints.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Verbosity {
    /// No spinner, no per-request lines; only the final summary.
    Quiet,
    /// Spinner summary line + only failures printed as scrollback.
    #[default]
    Normal,
    /// Spinner summary line + every completed request printed as scrollback.
    Verbose,
}

/// Per-category tallies used to render the summary line. `started` drives the
/// live `inflight` count (started minus everything already completed).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Counts {
    started: u64,
    downloaded: u64,
    unchanged: u64,
    missing: u64,
    ignored: u64,
    failed: u64,
}

impl Counts {
    fn inflight(&self) -> u64 {
        self.started.saturating_sub(
            self.downloaded + self.unchanged + self.missing + self.ignored + self.failed,
        )
    }
}

pub struct SyncProgress {
    site: String,
    verbosity: Verbosity,
    bar: ProgressBar,
    started: AtomicU64,
    downloaded: AtomicU64,
    unchanged: AtomicU64,
    missing: AtomicU64,
    ignored: AtomicU64,
    failed: AtomicU64,
}

impl SyncProgress {
    pub fn new(site: &str, index: usize, total: usize, verbosity: Verbosity) -> Self {
        let bar = ProgressBar::new_spinner();
        bar.set_draw_target(if verbosity == Verbosity::Quiet {
            ProgressDrawTarget::hidden()
        } else {
            ProgressDrawTarget::stderr()
        });
        bar.set_style(
            ProgressStyle::with_template("{prefix} {spinner:.cyan} {elapsed} {msg}")
                .expect("static progress template"),
        );
        bar.set_prefix(format!("[{index}/{total}] {site}"));
        bar.enable_steady_tick(Duration::from_millis(80));
        Self {
            site: site.to_owned(),
            verbosity,
            bar,
            started: AtomicU64::new(0),
            downloaded: AtomicU64::new(0),
            unchanged: AtomicU64::new(0),
            missing: AtomicU64::new(0),
            ignored: AtomicU64::new(0),
            failed: AtomicU64::new(0),
        }
    }

    pub fn finish(&self, report: &CrawlReport, committed: bool) {
        self.bar.finish_and_clear();
        eprintln!(
            "{}",
            summary_line(&self.site, report, if committed { "ok" } else { "failed" })
        );
    }

    fn counts(&self) -> Counts {
        Counts {
            started: self.started.load(Ordering::Relaxed),
            downloaded: self.downloaded.load(Ordering::Relaxed),
            unchanged: self.unchanged.load(Ordering::Relaxed),
            missing: self.missing.load(Ordering::Relaxed),
            ignored: self.ignored.load(Ordering::Relaxed),
            failed: self.failed.load(Ordering::Relaxed),
        }
    }

    /// Bump one completed-category counter, refresh the summary line, and emit a
    /// scrollback line per the active verbosity.
    fn complete(&self, tag: &str, counter: &AtomicU64, url: &str) {
        counter.fetch_add(1, Ordering::Relaxed);
        let path = short_path(url);
        self.bar.set_message(summary_msg(&self.counts(), &path));
        let logged = match self.verbosity {
            Verbosity::Verbose => true,
            Verbosity::Normal => tag == "FAIL",
            Verbosity::Quiet => false,
        };
        if logged {
            self.log_line(&format!("[{}] {tag:<9} {path}", self.site));
        }
    }

    /// Emit a scrollback line. On a TTY, `bar.println` prints it above the live
    /// spinner; when stderr is not a terminal the spinner is suppressed and
    /// `bar.println` would drop the line, so write it directly instead.
    fn log_line(&self, line: &str) {
        if std::io::stderr().is_terminal() {
            self.bar.println(line);
        } else {
            eprintln!("{line}");
        }
    }
}

impl CrawlObserver for SyncProgress {
    fn event(&self, event: CrawlEvent) {
        match event {
            CrawlEvent::Started(url) => {
                self.started.fetch_add(1, Ordering::Relaxed);
                self.bar
                    .set_message(summary_msg(&self.counts(), &short_path(&url)));
            }
            CrawlEvent::Downloaded(url) => self.complete("OK", &self.downloaded, &url),
            CrawlEvent::Unchanged(url) => self.complete("UNCHANGED", &self.unchanged, &url),
            CrawlEvent::Missing(url) => self.complete("MISS", &self.missing, &url),
            CrawlEvent::Ignored(url) => self.complete("IGNORED", &self.ignored, &url),
            CrawlEvent::Failed(url) => self.complete("FAIL", &self.failed, &url),
        }
    }
}

/// Dynamic segment of the spinner line: labelled tallies + live inflight + the
/// path currently in focus. All values are real and known, so no fake total.
fn summary_msg(counts: &Counts, path: &str) -> String {
    format!(
        "dl={} unchanged={} miss={} fail={}  inflight={}  · {path}",
        counts.downloaded,
        counts.unchanged,
        counts.missing,
        counts.failed,
        counts.inflight(),
    )
}

/// Shorten a full URL to its origin-relative `path[?query]` so long URLs do not
/// overflow and flicker the single spinner line. Falls back to the raw string
/// when the URL cannot be parsed.
fn short_path(url: &str) -> String {
    match Url::parse(url) {
        Ok(parsed) => match parsed.query() {
            Some(query) => format!("{}?{query}", parsed.path()),
            None => parsed.path().to_owned(),
        },
        Err(_) => url.to_owned(),
    }
}

pub fn summary_line(site: &str, report: &CrawlReport, status: &str) -> String {
    format!(
        "{site}: {status}; downloaded={}, unchanged={}, missing={}, ignored={}, failed={}",
        report.downloaded,
        report.unchanged,
        report.missing,
        report.ignored,
        report.failed()
    )
}

#[cfg(test)]
mod tests {
    use super::{Counts, short_path, summary_line, summary_msg};
    use crate::crawler::{CrawlFailure, CrawlReport};

    #[test]
    fn summary_msg_labels_counts_and_derives_inflight() {
        let counts = Counts {
            started: 8,
            downloaded: 3,
            unchanged: 1,
            missing: 1,
            ignored: 0,
            failed: 1,
        };
        assert_eq!(
            summary_msg(&counts, "/docs/a.md"),
            "dl=3 unchanged=1 miss=1 fail=1  inflight=2  · /docs/a.md"
        );
    }

    #[test]
    fn inflight_never_underflows() {
        let counts = Counts {
            started: 1,
            downloaded: 5,
            ..Counts::default()
        };
        assert_eq!(counts.inflight(), 0);
    }

    #[test]
    fn short_path_extracts_path_and_query_or_falls_back() {
        assert_eq!(
            short_path("https://example.com/docs/en/api/a.md"),
            "/docs/en/api/a.md"
        );
        assert_eq!(short_path("https://example.com/a.md?q=1"), "/a.md?q=1");
        assert_eq!(short_path("not a url"), "not a url");
    }

    #[test]
    fn formats_non_tty_summary() {
        let report = CrawlReport {
            downloaded: 2,
            unchanged: 4,
            missing: 1,
            ignored: 3,
            failures: vec![CrawlFailure {
                url: "https://example.com/a.md".to_owned(),
                message: "failure".to_owned(),
            }],
        };
        assert_eq!(
            summary_line("docs", &report, "failed"),
            "docs: failed; downloaded=2, unchanged=4, missing=1, ignored=3, failed=1"
        );
    }
}
