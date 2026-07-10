use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use std::time::Duration;

use crate::crawler::{CrawlEvent, CrawlObserver, CrawlReport};

pub struct SyncProgress {
    site: String,
    bar: ProgressBar,
}

impl SyncProgress {
    pub fn new(site: &str) -> Self {
        let bar = ProgressBar::new_spinner();
        bar.set_draw_target(ProgressDrawTarget::stderr());
        bar.set_style(
            ProgressStyle::with_template("{spinner:.cyan} {prefix} {pos} {msg}")
                .expect("static progress template"),
        );
        bar.set_prefix(site.to_owned());
        bar.enable_steady_tick(Duration::from_millis(80));
        Self {
            site: site.to_owned(),
            bar,
        }
    }

    pub fn finish(&self, report: &CrawlReport, committed: bool) {
        self.bar.finish_and_clear();
        eprintln!(
            "{}",
            summary_line(&self.site, report, if committed { "ok" } else { "failed" })
        );
    }
}

impl CrawlObserver for SyncProgress {
    fn event(&self, event: CrawlEvent) {
        match event {
            CrawlEvent::Started(url) => self.bar.set_message(format!("GET {url}")),
            CrawlEvent::Downloaded(url)
            | CrawlEvent::Unchanged(url)
            | CrawlEvent::Missing(url)
            | CrawlEvent::Ignored(url)
            | CrawlEvent::Failed(url) => {
                self.bar.inc(1);
                self.bar.set_message(url);
            }
        }
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
    use super::summary_line;
    use crate::crawler::{CrawlFailure, CrawlReport};

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
