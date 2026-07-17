//! End-of-run failure reporting: a terminal block that tells the user exactly
//! what failed and what to do next, plus a durable plain-text log so the detail
//! survives the terminal scrollback. Per-site *live* verdict lines are owned by
//! `progress`; this module owns the *aggregate* view printed after all sites.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use console::style;

use crate::crawler::CrawlReport;
use crate::progress::short_path;

/// Root-relative location of the durable run log. Overwritten every run — an
/// append-forever log would grow unbounded, which the project forbids. Kept in a
/// dot-dir so it is trivially gitignored and never mixes with mirrored content.
const LOG_DIR: &str = ".jj-llms-txt-wiki";
const LOG_FILE: &str = "last-run.log";
/// Cap per-site failures shown in the terminal; the full list always lands in
/// the log, whose path the block points at.
const TERMINAL_FAILURE_CAP: usize = 8;

/// One site's final outcome. `Failed` carries a report whose `failures` is
/// non-empty (synthesized for a commit/git error on an otherwise clean crawl);
/// `Aborted` is a whole-site error that never produced counts.
pub enum Outcome {
    Ok(CrawlReport),
    Failed(CrawlReport),
    Aborted(String),
}

pub struct SiteReport {
    pub site: String,
    pub outcome: Outcome,
}

impl SiteReport {
    pub fn is_ok(&self) -> bool {
        matches!(self.outcome, Outcome::Ok(_))
    }
}

/// Absolute path of the run log under `root`.
pub fn log_path(root: &Path) -> PathBuf {
    root.join(LOG_DIR).join(LOG_FILE)
}

/// Write the full plain-text run log (best-effort: a write failure must never
/// crash the sync or mask the real errors). Always overwrites. Returns the path
/// so the caller can point the user at it.
pub fn write_log(root: &Path, reports: &[SiteReport], timestamp: &str) -> PathBuf {
    let path = log_path(root);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&path, render_log(reports, timestamp, root));
    path
}

/// Print the end-of-run failure report to stderr, or nothing when every site
/// succeeded. Always runs regardless of verbosity — an error is exactly when a
/// `--quiet` user still needs to know what happened and where to look.
pub fn print_failures(reports: &[SiteReport], log: &Path) {
    let failed: Vec<&SiteReport> = reports.iter().filter(|report| !report.is_ok()).collect();
    if failed.is_empty() {
        return;
    }

    let heading = style(format!(
        "✗ {} site(s) failed — last successful snapshot kept",
        failed.len()
    ))
    .for_stderr()
    .red()
    .bold();
    eprintln!("\n{heading}");

    for report in &failed {
        match &report.outcome {
            Outcome::Aborted(reason) => {
                eprintln!(
                    "  {} could not start — {reason}",
                    style(format!("{}:", report.site)).for_stderr().bold()
                );
            }
            Outcome::Failed(crawl) => {
                eprintln!(
                    "  {}",
                    style(format!("{}: {} failure(s)", report.site, crawl.failed()))
                        .for_stderr()
                        .bold()
                );
                for failure in crawl.failures.iter().take(TERMINAL_FAILURE_CAP) {
                    eprintln!(
                        "    {} {}  {}",
                        style("✗").for_stderr().red(),
                        short_path(&failure.url),
                        style(&failure.message).for_stderr().dim()
                    );
                }
                let omitted = crawl.failures.len().saturating_sub(TERMINAL_FAILURE_CAP);
                if omitted > 0 {
                    eprintln!(
                        "    {}",
                        style(format!("… {omitted} more (see log)"))
                            .for_stderr()
                            .dim()
                    );
                }
                if crawl.missing > 0 {
                    eprintln!(
                        "    {}",
                        style(format!(
                            "· {} link(s) returned 404/410 (see log)",
                            crawl.missing
                        ))
                        .for_stderr()
                        .dim()
                    );
                }
            }
            Outcome::Ok(_) => {}
        }
    }

    let sites: Vec<&str> = failed.iter().map(|report| report.site.as_str()).collect();
    let retry = sites
        .iter()
        .map(|site| format!("jj-llms-txt-wiki sync {site}"))
        .collect::<Vec<_>>()
        .join("  ·  ");
    eprintln!();
    eprintln!(
        "  {} {}",
        style("full log:").for_stderr().cyan().bold(),
        log.display()
    );
    eprintln!(
        "  {} {retry}",
        style("retry:   ").for_stderr().cyan().bold()
    );
}

/// Print the always-on final one-liner — the canonical last line the eye learns
/// to check. Green `✓ N/N synced` on a clean run; red `✗ … → <log>` otherwise.
/// Symmetric and unconditional, so a failure can never hide as "looked done"
/// among many green per-site lines, and the outcome always lands in one fixed
/// spot. Call this last, after `print_failures`.
pub fn print_summary(reports: &[SiteReport], log: &Path) {
    eprintln!();
    eprintln!("{}", render_summary(reports, log));
}

/// Build the summary line. Non-`Ok` runs list only the non-zero failure buckets
/// (`Failed` → "failed", `Aborted` → "error") plus the log pointer, so the user
/// always has a next step even if the detail block scrolled away.
fn render_summary(reports: &[SiteReport], log: &Path) -> String {
    let total = reports.len();
    let ok = reports.iter().filter(|report| report.is_ok()).count();
    if ok == total {
        return style(format!("✓ {ok}/{total} synced"))
            .for_stderr()
            .green()
            .bold()
            .to_string();
    }
    let failed = reports
        .iter()
        .filter(|report| matches!(report.outcome, Outcome::Failed(_)))
        .count();
    let aborted = reports
        .iter()
        .filter(|report| matches!(report.outcome, Outcome::Aborted(_)))
        .count();
    let mut parts = vec![format!("{ok} ok")];
    if failed > 0 {
        parts.push(format!("{failed} failed"));
    }
    if aborted > 0 {
        parts.push(format!("{aborted} error"));
    }
    style(format!("✗ {} → {}", parts.join(" · "), log.display()))
        .for_stderr()
        .red()
        .bold()
        .to_string()
}

/// Render the durable log body: a complete, uncolored record of every site's
/// counts, all failures, and all missing links.
fn render_log(reports: &[SiteReport], timestamp: &str, root: &Path) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "jj-llms-txt-wiki sync report — {timestamp}");
    let _ = writeln!(out, "output: {}", root.display());
    let _ = writeln!(out);

    for report in reports {
        match &report.outcome {
            Outcome::Ok(crawl) => {
                let _ = writeln!(out, "{}: ok        {}", report.site, counts(crawl));
                write_oversize(&mut out, crawl);
            }
            Outcome::Failed(crawl) => {
                let _ = writeln!(
                    out,
                    "{}: FAILED    {}, failed={}",
                    report.site,
                    counts(crawl),
                    crawl.failed()
                );
                for failure in &crawl.failures {
                    let _ = writeln!(out, "    ✗ {}  —  {}", failure.url, failure.message);
                }
                if !crawl.missing_urls.is_empty() {
                    let _ = writeln!(out, "    missing ({}):", crawl.missing_urls.len());
                    for url in &crawl.missing_urls {
                        let _ = writeln!(out, "      · {url}");
                    }
                }
                write_oversize(&mut out, crawl);
            }
            Outcome::Aborted(reason) => {
                let _ = writeln!(out, "{}: ERROR — {reason}", report.site);
            }
        }
    }
    out
}

fn counts(report: &CrawlReport) -> String {
    format!(
        "downloaded={}, resumed={}, unchanged={}, missing={}, ignored={}, oversize={}",
        report.downloaded,
        report.resumed,
        report.unchanged,
        report.missing,
        report.ignored,
        report.oversize,
    )
}

/// Append the dropped-oversized URLs to the durable log. Called for both `Ok` and
/// `Failed` sites: oversize is an exclusion, not a failure, so it can land on an
/// otherwise-successful run and still deserves a named record.
fn write_oversize(out: &mut String, report: &CrawlReport) {
    if report.oversize_urls.is_empty() {
        return;
    }
    let _ = writeln!(out, "    oversize ({}):", report.oversize_urls.len());
    for url in &report.oversize_urls {
        let _ = writeln!(out, "      · {url}");
    }
}

#[cfg(test)]
mod tests {
    use super::{Outcome, SiteReport, log_path, render_log, render_summary, write_log};
    use crate::crawler::{CrawlFailure, CrawlReport};
    use std::path::Path;
    use tempfile::tempdir;

    fn failed_report() -> CrawlReport {
        CrawlReport {
            downloaded: 3,
            missing: 2,
            failures: vec![CrawlFailure {
                url: "https://example.com/a.md".to_owned(),
                message: "GET https://example.com/a.md: HTTP 500 Internal Server Error".to_owned(),
            }],
            missing_urls: vec![
                "https://example.com/x.md".to_owned(),
                "https://example.com/y.md".to_owned(),
            ],
            oversize: 1,
            oversize_urls: vec!["https://example.com/huge.md".to_owned()],
            ..CrawlReport::default()
        }
    }

    #[test]
    fn log_records_counts_failures_and_missing_urls() {
        let reports = vec![
            SiteReport {
                site: "good".to_owned(),
                outcome: Outcome::Ok(CrawlReport {
                    downloaded: 5,
                    ..CrawlReport::default()
                }),
            },
            SiteReport {
                site: "bad".to_owned(),
                outcome: Outcome::Failed(failed_report()),
            },
            SiteReport {
                site: "dead".to_owned(),
                outcome: Outcome::Aborted("GET .../llms.txt: connection refused".to_owned()),
            },
        ];
        let body = render_log(&reports, "2026-07-12T00:00:00Z", std::path::Path::new("/w"));
        assert!(body.contains("good: ok        downloaded=5"));
        assert!(body.contains("bad: FAILED"));
        assert!(body.contains("HTTP 500"));
        assert!(body.contains("missing (2):"));
        assert!(body.contains("· https://example.com/x.md"));
        assert!(body.contains("oversize (1):"));
        assert!(body.contains("· https://example.com/huge.md"));
        assert!(body.contains("dead: ERROR — GET .../llms.txt: connection refused"));
    }

    #[test]
    fn summary_is_green_synced_when_every_site_ok() {
        console::set_colors_enabled_stderr(false);
        let reports = vec![
            SiteReport {
                site: "a".to_owned(),
                outcome: Outcome::Ok(CrawlReport::default()),
            },
            SiteReport {
                site: "b".to_owned(),
                outcome: Outcome::Ok(CrawlReport::default()),
            },
        ];
        assert_eq!(
            render_summary(&reports, Path::new("/w/.jj-llms-txt-wiki/last-run.log")),
            "✓ 2/2 synced"
        );
    }

    #[test]
    fn summary_counts_failed_and_aborted_and_points_at_log() {
        console::set_colors_enabled_stderr(false);
        let reports = vec![
            SiteReport {
                site: "a".to_owned(),
                outcome: Outcome::Ok(CrawlReport::default()),
            },
            SiteReport {
                site: "b".to_owned(),
                outcome: Outcome::Failed(failed_report()),
            },
            SiteReport {
                site: "c".to_owned(),
                outcome: Outcome::Aborted("connection refused".to_owned()),
            },
        ];
        assert_eq!(
            render_summary(&reports, Path::new("/w/.jj-llms-txt-wiki/last-run.log")),
            "✗ 1 ok · 1 failed · 1 error → /w/.jj-llms-txt-wiki/last-run.log"
        );
    }

    #[test]
    fn summary_omits_zero_buckets() {
        console::set_colors_enabled_stderr(false);
        let reports = vec![
            SiteReport {
                site: "a".to_owned(),
                outcome: Outcome::Ok(CrawlReport::default()),
            },
            SiteReport {
                site: "b".to_owned(),
                outcome: Outcome::Failed(failed_report()),
            },
        ];
        assert_eq!(
            render_summary(&reports, Path::new("/w/.jj-llms-txt-wiki/last-run.log")),
            "✗ 1 ok · 1 failed → /w/.jj-llms-txt-wiki/last-run.log"
        );
    }

    #[test]
    fn write_log_creates_file_under_dot_dir() {
        let dir = tempdir().unwrap();
        let reports = vec![SiteReport {
            site: "bad".to_owned(),
            outcome: Outcome::Failed(failed_report()),
        }];
        let path = write_log(dir.path(), &reports, "2026-07-12T00:00:00Z");
        assert_eq!(path, log_path(dir.path()));
        assert!(path.exists());
        assert!(
            std::fs::read_to_string(&path)
                .unwrap()
                .contains("bad: FAILED")
        );
    }
}
