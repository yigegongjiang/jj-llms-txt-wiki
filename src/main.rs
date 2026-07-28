mod cli;
mod config;
pub mod crawler;
pub mod discovery;
mod full;
mod git;
pub mod http;
mod lifecycle;
pub mod manifest;
mod progress;
mod report;
mod site;
mod snapshot;
mod sync;
pub mod url_map;

use clap::{CommandFactory, Parser};
use cli::{Cli, Command};
use console::style;
use std::process::ExitCode;

async fn run() -> Result<(), String> {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(error) => {
            error.print().map_err(|error| error.to_string())?;
            return if error.use_stderr() {
                Err(String::new())
            } else {
                Ok(())
            };
        }
    };

    match cli.command {
        None => Cli::command()
            .print_help()
            .map_err(|error| format!("print help: {error}")),
        Some(Command::Site { command }) => site::run(command, &config::default_path()?),
        Some(Command::Sync {
            site,
            concurrency,
            interval,
        }) => {
            sync::run(
                &config::default_path()?,
                site,
                concurrency,
                interval,
                push_snapshot_url(),
            )
            .await
        }
        Some(Command::Update) => lifecycle::update(),
        Some(Command::Uninstall) => lifecycle::uninstall(),
    }
}

/// Snapshot mirror target, resolved at build time from the source checkout's
/// `origin` (see `build.rs`) and falling back to `[package].repository`. A fork
/// needs no configuration: it mirrors to the repository it was built from.
/// `JJ_LLMS_TXT_WIKI_PUSH_URL` is the opt-out — empty disables the mirror
/// entirely, which is what the e2e tests use so a test run can never push.
/// Users without credentials for the target just get a silent, fast failure.
fn push_snapshot_url() -> Option<String> {
    if let Ok(value) = std::env::var("JJ_LLMS_TXT_WIKI_PUSH_URL") {
        return if value.is_empty() { None } else { Some(value) };
    }
    mirror_url(option_env!("JJ_LLMS_TXT_WIKI_ORIGIN").unwrap_or(env!("CARGO_PKG_REPOSITORY")))
}

/// SSH remotes (`git@host:owner/repo[.git]`) are already valid as-is; only the
/// bare HTTPS form that `[package].repository` and `actions/checkout` produce
/// needs `.git` appended.
fn mirror_url(target: &str) -> Option<String> {
    if target.is_empty() {
        return None;
    }
    Some(
        if target.ends_with(".git") || !target.starts_with("https://") {
            target.to_string()
        } else {
            format!("{target}.git")
        },
    )
}

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            if !error.is_empty() {
                eprintln!("{}: {error}", style("error").for_stderr().red().bold());
            }
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::mirror_url;

    #[test]
    fn mirror_url_accepts_every_origin_form_git_produces() {
        // `actions/checkout` and `[package].repository`: bare HTTPS.
        assert_eq!(
            mirror_url("https://github.com/alice/wiki").as_deref(),
            Some("https://github.com/alice/wiki.git")
        );
        // Already-suffixed and SSH remotes pass through untouched.
        for origin in [
            "https://github.com/alice/wiki.git",
            "git@github.com:alice/wiki.git",
            "git@p.github.com:alice/wiki.git",
            "ssh://git@github.com/alice/wiki",
        ] {
            assert_eq!(mirror_url(origin).as_deref(), Some(origin));
        }
        // No checkout and no `repository` field: no mirror.
        assert_eq!(mirror_url(""), None);
    }

    #[test]
    fn baked_mirror_target_points_at_this_checkout() {
        // The whole point of `build.rs`: a fork mirrors to itself, so the value
        // must track `origin` rather than being pinned to any one repository.
        // Builds with no checkout to ask (source tarball) must bake nothing and
        // fall back instead.
        let origin = std::process::Command::new("git")
            .args(["remote", "get-url", "origin"])
            .output()
            .ok()
            .filter(|output| output.status.success())
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .map(|origin| origin.trim().to_string())
            .filter(|origin| !origin.is_empty());
        assert_eq!(
            option_env!("JJ_LLMS_TXT_WIKI_ORIGIN"),
            origin.as_deref(),
            "build.rs must bake this checkout's origin, or nothing when there is none"
        );
    }
}
