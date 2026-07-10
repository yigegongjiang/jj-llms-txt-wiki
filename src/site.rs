use std::path::Path;
use url::Url;

use crate::cli::SiteCommand;
use crate::config::{Config, SiteConfig};

pub fn validate_name(name: &str) -> Result<(), String> {
    let mut characters = name.chars();
    let Some(first) = characters.next() else {
        return Err("site name must not be empty".to_owned());
    };
    if !first.is_ascii_alphanumeric()
        || !characters.all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-')
        })
    {
        return Err(format!("invalid site name: {name}"));
    }
    Ok(())
}

pub fn parse_entry_url(value: &str) -> Result<Url, String> {
    let url = Url::parse(value).map_err(|error| format!("invalid URL {value}: {error}"))?;
    if !matches!(url.scheme(), "http" | "https") || url.host_str().is_none() {
        return Err(format!("URL must be absolute HTTP(S): {value}"));
    }
    Ok(url)
}

pub fn add(config: &mut Config, name: &str, url: &str) -> Result<(), String> {
    validate_name(name)?;
    parse_entry_url(url)?;
    if config.sites.contains_key(name) {
        return Err(format!("site already exists: {name}"));
    }
    config.sites.insert(
        name.to_owned(),
        SiteConfig {
            url: url.to_owned(),
        },
    );
    Ok(())
}

pub fn run(command: SiteCommand, config_path: &Path) -> Result<(), String> {
    let mut config = Config::load(config_path)?;
    match command {
        SiteCommand::Add { name, url } => {
            add(&mut config, &name, &url)?;
            config.save(config_path)?;
            println!("{name}\t{url}");
        }
        SiteCommand::List => {
            for (name, site) in config.sites {
                println!("{name}\t{}", site.url);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{add, parse_entry_url, validate_name};
    use crate::config::Config;

    #[test]
    fn validates_site_names() {
        for name in ["docs", "docs-v2", "docs.v2", "v_2", "9docs"] {
            validate_name(name).unwrap();
        }
        for name in ["", ".", "..", "-docs", "_docs", "docs/path", "文档"] {
            assert!(validate_name(name).is_err(), "{name}");
        }
    }

    #[test]
    fn validates_entry_urls() {
        parse_entry_url("https://example.com/llms.txt").unwrap();
        parse_entry_url("http://localhost:8080/llms.txt").unwrap();
        for url in ["/llms.txt", "file:///tmp/llms.txt", "mailto:a@example.com"] {
            assert!(parse_entry_url(url).is_err(), "{url}");
        }
    }

    #[test]
    fn adds_once_without_overwriting() {
        let mut config = Config::default();
        add(&mut config, "docs", "https://example.com/llms.txt").unwrap();
        let before = config.clone();
        assert!(add(&mut config, "docs", "https://other.test/llms.txt").is_err());
        assert_eq!(config, before);
    }
}
