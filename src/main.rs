mod cli;
mod config;
pub mod crawler;
pub mod discovery;
mod git;
pub mod http;
mod lifecycle;
mod progress;
mod site;
mod snapshot;
mod sync;
pub mod url_map;

use clap::{CommandFactory, Parser};
use cli::{Cli, Command};
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
        }) => sync::run(&config::default_path()?, site, concurrency, interval).await,
        Some(Command::Update) => lifecycle::update(),
        Some(Command::Uninstall) => lifecycle::uninstall(),
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            if !error.is_empty() {
                eprintln!("error: {error}");
            }
            ExitCode::FAILURE
        }
    }
}
