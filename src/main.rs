mod cli;
mod config;
pub mod crawler;
pub mod discovery;
mod git;
pub mod http;
mod lifecycle;
pub mod manifest;
mod progress;
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
        Some(Command::Version) => {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Some(Command::Site { command }) => site::run(command, &config::default_path()?),
        Some(Command::Sync {
            site,
            concurrency,
            interval,
            verbose,
            quiet,
        }) => {
            let verbosity = if quiet {
                progress::Verbosity::Quiet
            } else if verbose {
                progress::Verbosity::Verbose
            } else {
                progress::Verbosity::Normal
            };
            sync::run(
                &config::default_path()?,
                site,
                concurrency,
                interval,
                verbosity,
                push_snapshot_url(),
            )
            .await
        }
        Some(Command::Update) => lifecycle::update(),
        Some(Command::Uninstall) => lifecycle::uninstall(),
    }
}

/// Snapshot mirror target: `LLMS_WIKI_PUSH_URL` env var overrides the default
/// (`CARGO_PKG_REPOSITORY` + `.git`). Empty string disables push explicitly —
/// used by tests and by anyone who wants to opt out. Forks published under a
/// different `[package].repository` in `Cargo.toml` auto-target their own
/// remote; users without push credentials just get a silent, fast failure.
fn push_snapshot_url() -> Option<String> {
    if let Ok(value) = std::env::var("LLMS_WIKI_PUSH_URL") {
        return if value.is_empty() { None } else { Some(value) };
    }
    let repository = env!("CARGO_PKG_REPOSITORY");
    if repository.is_empty() {
        return None;
    }
    Some(if repository.ends_with(".git") {
        repository.to_string()
    } else {
        format!("{repository}.git")
    })
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
