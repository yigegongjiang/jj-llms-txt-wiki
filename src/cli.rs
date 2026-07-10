use clap::{Parser, Subcommand};
use std::time::Duration;

#[derive(Debug, Parser)]
#[command(name = "llms-wiki", version, about, arg_required_else_help = false)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Manage configured sites
    Site {
        #[command(subcommand)]
        command: SiteCommand,
    },
    /// Synchronize configured Markdown sites
    Sync {
        /// Site name; omit to synchronize all sites
        site: Option<String>,
        /// Maximum simultaneous requests
        #[arg(long, value_parser = parse_concurrency)]
        concurrency: Option<usize>,
        /// Minimum delay between request starts
        #[arg(long, value_parser = parse_duration)]
        interval: Option<Duration>,
    },
    /// Print version information
    Version,
    /// Download the latest release and replace this binary
    #[command(alias = "upgrade")]
    Update,
    /// Remove this binary from disk
    Uninstall,
}

fn parse_concurrency(value: &str) -> Result<usize, String> {
    let concurrency = value
        .parse::<usize>()
        .map_err(|error| format!("invalid concurrency: {error}"))?;
    if concurrency == 0 {
        return Err("concurrency must be greater than 0".to_owned());
    }
    Ok(concurrency)
}

fn parse_duration(value: &str) -> Result<Duration, String> {
    humantime::parse_duration(value).map_err(|error| format!("invalid duration: {error}"))
}

#[derive(Debug, Subcommand)]
pub enum SiteCommand {
    /// Add a site
    Add {
        /// Safe local site name
        name: String,
        /// Absolute HTTP(S) llms.txt URL
        url: String,
    },
    /// List configured sites
    List,
}

#[cfg(test)]
mod tests {
    use clap::{CommandFactory, Parser};

    use super::{Cli, Command, SiteCommand};

    #[test]
    fn parses_lifecycle_commands() {
        assert!(matches!(
            Cli::try_parse_from(["llms-wiki", "version"])
                .expect("version command")
                .command,
            Some(Command::Version)
        ));
        assert!(matches!(
            Cli::try_parse_from(["llms-wiki", "update"])
                .expect("update command")
                .command,
            Some(Command::Update)
        ));
        assert!(matches!(
            Cli::try_parse_from(["llms-wiki", "upgrade"])
                .expect("upgrade alias")
                .command,
            Some(Command::Update)
        ));
        assert!(matches!(
            Cli::try_parse_from(["llms-wiki", "uninstall"])
                .expect("uninstall command")
                .command,
            Some(Command::Uninstall)
        ));
    }

    #[test]
    fn supports_standard_help_and_version_flags() {
        assert!(Cli::try_parse_from(["llms-wiki", "--help"]).is_err());
        assert!(Cli::try_parse_from(["llms-wiki", "--version"]).is_err());
        Cli::command().debug_assert();
    }

    #[test]
    fn rejects_unknown_commands() {
        assert!(Cli::try_parse_from(["llms-wiki", "unknown"]).is_err());
    }

    #[test]
    fn parses_site_commands() {
        let cli = Cli::try_parse_from([
            "llms-wiki",
            "site",
            "add",
            "docs",
            "https://example.com/llms.txt",
        ])
        .expect("site add command");
        assert!(matches!(
            cli.command,
            Some(Command::Site {
                command: SiteCommand::Add { .. }
            })
        ));

        let cli = Cli::try_parse_from(["llms-wiki", "site", "list"]).expect("site list command");
        assert!(matches!(
            cli.command,
            Some(Command::Site {
                command: SiteCommand::List
            })
        ));
    }

    #[test]
    fn parses_sync_overrides() {
        let cli = Cli::try_parse_from([
            "llms-wiki",
            "sync",
            "docs",
            "--concurrency",
            "2",
            "--interval",
            "500ms",
        ])
        .expect("sync command");
        assert!(matches!(
            cli.command,
            Some(Command::Sync {
                site: Some(ref site),
                concurrency: Some(2),
                interval: Some(duration),
            }) if site == "docs" && duration == std::time::Duration::from_millis(500)
        ));
        assert!(Cli::try_parse_from(["llms-wiki", "sync", "--concurrency", "0"]).is_err());
    }
}
