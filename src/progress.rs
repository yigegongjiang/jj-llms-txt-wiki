use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::IsTerminal;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use url::Url;

use crate::crawler::{CrawlEvent, CrawlObserver, CrawlReport};

/// Per-category tallies used to render the summary line. `started` drives the
/// live `inflight` count (started minus everything already completed).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Counts {
    started: u64,
    downloaded: u64,
    resumed: u64,
    unchanged: u64,
    missing: u64,
    ignored: u64,
    oversize: u64,
    failed: u64,
}

impl Counts {
    fn inflight(&self) -> u64 {
        self.started.saturating_sub(
            self.downloaded
                + self.unchanged
                + self.missing
                + self.ignored
                + self.oversize
                + self.failed,
        )
    }
}

pub struct SyncProgress {
    site: String,
    bar: ProgressBar,
    started: AtomicU64,
    downloaded: AtomicU64,
    resumed: AtomicU64,
    unchanged: AtomicU64,
    missing: AtomicU64,
    ignored: AtomicU64,
    oversize: AtomicU64,
    failed: AtomicU64,
}

impl SyncProgress {
    /// The spinner draws to stderr; indicatif auto-suppresses it when stderr is
    /// not a terminal, so piped runs stay clean without any flag.
    pub fn new(site: &str, index: usize, total: usize) -> Self {
        let bar = ProgressBar::new_spinner();
        bar.set_style(
            ProgressStyle::with_template("{prefix} {spinner:.cyan} {elapsed} {msg}")
                .expect("static progress template"),
        );
        bar.set_prefix(format!("[{index}/{total}] {site}"));
        bar.enable_steady_tick(Duration::from_millis(80));
        Self {
            site: site.to_owned(),
            bar,
            started: AtomicU64::new(0),
            downloaded: AtomicU64::new(0),
            resumed: AtomicU64::new(0),
            unchanged: AtomicU64::new(0),
            missing: AtomicU64::new(0),
            ignored: AtomicU64::new(0),
            oversize: AtomicU64::new(0),
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

    /// Clear the spinner without printing a summary line. Used when the crawl
    /// aborts before producing a report (e.g. the entry document itself fails):
    /// the caller prints an `error_line` carrying the real reason instead of a
    /// misleading `failed; … failed=0` summary.
    pub fn abort(&self) {
        self.bar.finish_and_clear();
    }

    fn counts(&self) -> Counts {
        Counts {
            started: self.started.load(Ordering::Relaxed),
            downloaded: self.downloaded.load(Ordering::Relaxed),
            resumed: self.resumed.load(Ordering::Relaxed),
            unchanged: self.unchanged.load(Ordering::Relaxed),
            missing: self.missing.load(Ordering::Relaxed),
            ignored: self.ignored.load(Ordering::Relaxed),
            oversize: self.oversize.load(Ordering::Relaxed),
            failed: self.failed.load(Ordering::Relaxed),
        }
    }

    /// Bump one completed-category counter and refresh the live summary line.
    /// Successes stay in the summary only; failures go through [`Self::fail`].
    fn complete(&self, counter: &AtomicU64, url: &str) {
        counter.fetch_add(1, Ordering::Relaxed);
        self.bar
            .set_message(summary_msg(&self.counts(), &short_path(url)));
    }

    /// Record a failure and print it as scrollback so it survives above the
    /// cleared spinner.
    fn fail(&self, url: &str) {
        self.failed.fetch_add(1, Ordering::Relaxed);
        let path = short_path(url);
        self.bar.set_message(summary_msg(&self.counts(), &path));
        let tag = style("FAIL").for_stderr().red().bold();
        self.log_line(&format!("[{}] {tag} {path}", self.site));
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
            CrawlEvent::Downloaded(url) => self.complete(&self.downloaded, &url),
            CrawlEvent::Resumed(url) => self.complete(&self.resumed, &url),
            CrawlEvent::Unchanged(url) => self.complete(&self.unchanged, &url),
            CrawlEvent::Missing(url) => self.complete(&self.missing, &url),
            CrawlEvent::Ignored(url) => self.complete(&self.ignored, &url),
            CrawlEvent::Oversize(url) => self.complete(&self.oversize, &url),
            CrawlEvent::Failed(url) => self.fail(&url),
        }
    }
}

/// Dynamic segment of the spinner line: labelled tallies + live inflight + the
/// path currently in focus. All values are real and known, so no fake total.
/// `dl` stays green as the primary metric; `fail` lights up red the instant a
/// failure lands, so a running sync's health is legible without reading digits.
fn summary_msg(counts: &Counts, path: &str) -> String {
    let dl = style(format!("dl={}", counts.downloaded))
        .for_stderr()
        .green();
    let fail = style(format!("fail={}", counts.failed)).for_stderr();
    let fail = if counts.failed > 0 {
        fail.red().bold()
    } else {
        fail
    };
    format!(
        "{dl} resume={} unchanged={} miss={} {fail}  inflight={}  · {path}",
        counts.resumed,
        counts.unchanged,
        counts.missing,
        counts.inflight(),
    )
}

/// Per-site verdict line for a whole-site abort — an entry/snapshot error or a
/// crawl that never produced a report. Unlike `summary_line` it carries the real
/// `reason` inline, so the user never sees a bare `failed` with `failed=0`.
pub fn error_line(site: &str, reason: &str) -> String {
    let verdict = style("error").for_stderr().red().bold();
    format!("{site}: {verdict} — {reason}")
}

/// Shorten a full URL to its origin-relative `path[?query]` so long URLs do not
/// overflow and flicker the single spinner line. Falls back to the raw string
/// when the URL cannot be parsed.
pub(crate) fn short_path(url: &str) -> String {
    match Url::parse(url) {
        Ok(parsed) => match parsed.query() {
            Some(query) => format!("{}?{query}", parsed.path()),
            None => parsed.path().to_owned(),
        },
        Err(_) => url.to_owned(),
    }
}

/// Final per-site verdict. The status word and `failed` count carry the "what
/// now" signal: green `ok` = nothing to do, red `failed` + red `failed=N` = the
/// last snapshot was kept, retry after inspecting the scrollback failures above.
pub fn summary_line(site: &str, report: &CrawlReport, status: &str) -> String {
    let verdict = style(status).for_stderr().bold();
    let verdict = if status == "ok" {
        verdict.green()
    } else {
        verdict.red()
    };
    let downloaded = style(format!("downloaded={}", report.downloaded))
        .for_stderr()
        .green();
    let failed_count = report.failed();
    let failed = style(format!("failed={failed_count}")).for_stderr();
    let failed = if failed_count > 0 {
        failed.red().bold()
    } else {
        failed
    };
    // `oversize` is dropped-bloat, not the norm, so it only earns a slot on the
    // line when it actually happened — a clean run stays uncluttered.
    let oversize = if report.oversize > 0 {
        format!(", oversize={}", report.oversize)
    } else {
        String::new()
    };
    format!(
        "{site}: {verdict}; {downloaded}, resumed={}, unchanged={}, missing={}, ignored={}{oversize}, {failed}",
        report.resumed, report.unchanged, report.missing, report.ignored,
    )
}

#[cfg(test)]
mod tests {
    use super::{Counts, short_path, summary_line, summary_msg};
    use crate::crawler::{CrawlFailure, CrawlReport};

    #[test]
    fn summary_msg_labels_counts_and_derives_inflight() {
        // Pin colors off so the assert checks the plain skeleton regardless of
        // how the test harness is run (`--nocapture`, `CLICOLOR_FORCE`, …).
        console::set_colors_enabled_stderr(false);
        let counts = Counts {
            started: 8,
            downloaded: 3,
            resumed: 2,
            unchanged: 1,
            missing: 1,
            ignored: 0,
            oversize: 0,
            failed: 1,
        };
        assert_eq!(
            summary_msg(&counts, "/docs/a.md"),
            "dl=3 resume=2 unchanged=1 miss=1 fail=1  inflight=2  · /docs/a.md"
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
        // Pin colors off so the assert checks the plain skeleton regardless of
        // how the test harness is run (`--nocapture`, `CLICOLOR_FORCE`, …).
        console::set_colors_enabled_stderr(false);
        let report = CrawlReport {
            downloaded: 2,
            resumed: 5,
            unchanged: 4,
            missing: 1,
            ignored: 3,
            failures: vec![CrawlFailure {
                url: "https://example.com/a.md".to_owned(),
                message: "failure".to_owned(),
            }],
            ..CrawlReport::default()
        };
        assert_eq!(
            summary_line("docs", &report, "failed"),
            "docs: failed; downloaded=2, resumed=5, unchanged=4, missing=1, ignored=3, failed=1"
        );
    }

    #[test]
    fn summary_line_shows_oversize_only_when_present() {
        console::set_colors_enabled_stderr(false);
        let report = CrawlReport {
            downloaded: 4,
            oversize: 2,
            ..CrawlReport::default()
        };
        assert_eq!(
            summary_line("docs", &report, "ok"),
            "docs: ok; downloaded=4, resumed=0, unchanged=0, missing=0, ignored=0, oversize=2, failed=0"
        );
    }
}
