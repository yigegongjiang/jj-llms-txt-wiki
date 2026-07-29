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

pub fn add(config: &mut Config, name: &str, urls: &[String]) -> Result<(), String> {
    validate_name(name)?;
    let site = SiteConfig::new(urls.to_vec());
    site.entries()?;
    if config.sites.contains_key(name) {
        return Err(format!("site already exists: {name}"));
    }
    config.sites.insert(name.to_owned(), site);
    Ok(())
}

/// Entry URLs on one line, space separated — the separator `site list` has always
/// used between fields is a tab, so keeping it out of the URL column leaves the
/// output splittable by field.
fn format_urls(urls: &[String]) -> String {
    urls.join(" ")
}

pub fn run(command: SiteCommand, config_path: &Path) -> Result<(), String> {
    let mut config = Config::load(config_path)?;
    match command {
        SiteCommand::Add { name, urls } => {
            add(&mut config, &name, &urls)?;
            config.save(config_path)?;
            println!("{name}\t{}", format_urls(&urls));
        }
        SiteCommand::List => {
            for (name, site) in config.sites {
                println!("{name}\t{}", format_urls(&site.urls));
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

    fn urls(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }

    #[test]
    fn adds_once_without_overwriting() {
        let mut config = Config::default();
        add(
            &mut config,
            "docs",
            &urls(&["https://example.com/llms.txt"]),
        )
        .unwrap();
        let before = config.clone();
        assert!(add(&mut config, "docs", &urls(&["https://other.test/llms.txt"])).is_err());
        assert_eq!(config, before);
    }

    #[test]
    fn adds_several_entries_and_rejects_unusable_sets() {
        let mut config = Config::default();
        add(
            &mut config,
            "docs",
            &urls(&[
                "https://example.com/workers/llms.txt",
                "https://example.com/pages/llms.txt",
            ]),
        )
        .unwrap();
        assert_eq!(config.sites["docs"].urls.len(), 2);

        // Mixed chains, duplicates and an empty set are all unusable.
        for set in [
            vec![
                "https://example.com/llms.txt",
                "https://example.com/llms-full.txt",
            ],
            vec![
                "https://example.com/llms.txt",
                "https://example.com/llms.txt",
            ],
            vec![],
        ] {
            assert!(add(&mut config, "other", &urls(&set)).is_err(), "{set:?}");
            assert!(!config.sites.contains_key("other"), "{set:?}");
        }
    }
}
