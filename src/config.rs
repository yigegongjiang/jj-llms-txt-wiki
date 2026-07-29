use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::collections::{BTreeMap, HashSet};
use std::env;
use std::fs;
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;
use url::Url;

use crate::site::{parse_entry_url, validate_name};
use crate::url_map::{CanonicalUrl, EntryKind};

pub const DEFAULT_OUTPUT_DIR: &str = "~/.config/jj-llms-txt-wiki/wiki";
/// Download slots. 8 is the established per-domain simultaneity level for
/// crawlers (Scrapy's `CONCURRENT_REQUESTS_PER_DOMAIN`); sites are synced one at
/// a time, so all slots hit a single host.
pub const DEFAULT_CONCURRENCY: usize = 8;
/// Rest per slot after each finished request. Keeps the default sustained rate
/// polite (~`concurrency / (latency + 0.1s)`) without idling the slots.
pub const DEFAULT_INTERVAL_MS: u64 = 100;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Config {
    pub output_dir: String,
    pub concurrency: usize,
    pub interval_ms: u64,
    pub sites: BTreeMap<String, SiteConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            output_dir: DEFAULT_OUTPUT_DIR.to_owned(),
            concurrency: DEFAULT_CONCURRENCY,
            interval_ms: DEFAULT_INTERVAL_MS,
            sites: BTreeMap::new(),
        }
    }
}

/// A site's entry documents. One site can declare several entries — docs hosts
/// commonly split one product family across per-section `llms.txt` /
/// `llms-full.txt` paths — and they all land in the same site directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiteConfig {
    pub urls: Vec<String>,
}

impl SiteConfig {
    pub fn new(urls: Vec<String>) -> Self {
        Self { urls }
    }

    /// Parse every entry URL and report the chain they all share.
    ///
    /// Rejects a mix of `llms.txt` and `llms-full.txt` entries: the two chains
    /// disagree on snapshot strategy — the index chain resumes an interrupted
    /// partial and revalidates against a manifest, the aggregate chain rebuilds
    /// from empty every run — and a site can only have one snapshot. Also rejects
    /// duplicate entries, which would make the aggregate chain parse the same
    /// bundle twice and report every one of its pages as a duplicate.
    pub fn entries(&self) -> Result<(Vec<Url>, EntryKind), String> {
        let mut parsed = Vec::with_capacity(self.urls.len());
        let mut seen = HashSet::new();
        for url in &self.urls {
            let entry = parse_entry_url(url).map_err(|error| format!("invalid url: {error}"))?;
            if !seen.insert(CanonicalUrl::new(entry.clone())) {
                return Err(format!("duplicate url: {url}"));
            }
            parsed.push(entry);
        }
        let Some(first) = parsed.first() else {
            return Err("must declare at least one url".to_owned());
        };
        let kind = EntryKind::from_url(first);
        for entry in &parsed[1..] {
            if EntryKind::from_url(entry) != kind {
                return Err(format!(
                    "mixes llms.txt and llms-full.txt entries ({first} and {entry}); \
                     one site must use a single kind"
                ));
            }
        }
        Ok((parsed, kind))
    }
}

/// Accepts both spellings so a config written before multi-entry support keeps
/// loading verbatim: `url = "…"` for one entry, `urls = […]` for several. Both at
/// once is rejected rather than merged — two places declaring the same thing has
/// no obviously right precedence, and silently picking one would hide the typo.
impl<'de> Deserialize<'de> for SiteConfig {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            #[serde(default)]
            url: Option<String>,
            #[serde(default)]
            urls: Option<Vec<String>>,
        }

        let raw = Raw::deserialize(deserializer)?;
        let urls = match (raw.url, raw.urls) {
            (Some(_), Some(_)) => {
                return Err(de::Error::custom(
                    "site declares both url and urls; keep one",
                ));
            }
            (Some(url), None) => vec![url],
            (None, Some(urls)) => urls,
            (None, None) => return Err(de::Error::missing_field("url")),
        };
        Ok(Self { urls })
    }
}

/// Writes back the narrower spelling a single-entry site came in as, so adding
/// multi-entry support does not rewrite every existing site on the next `save`.
impl Serialize for SiteConfig {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;
        match self.urls.as_slice() {
            [single] => map.serialize_entry("url", single)?,
            many => map.serialize_entry("urls", many)?,
        }
        map.end()
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Self, String> {
        let contents = match fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Self::default()),
            Err(error) => return Err(format!("read config {}: {error}", path.display())),
        };
        let config: Self = toml::from_str(&contents)
            .map_err(|error| format!("parse config {}: {error}", path.display()))?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.concurrency == 0 {
            return Err("config concurrency must be greater than 0".to_owned());
        }
        if self.output_dir.is_empty() {
            return Err("config output_dir must not be empty".to_owned());
        }
        self.output_path()?;
        for (name, site) in &self.sites {
            validate_name(name)?;
            site.entries()
                .map_err(|error| format!("site {name} {error}"))?;
        }
        Ok(())
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        self.validate()?;
        let parent = path
            .parent()
            .ok_or_else(|| format!("config path has no parent: {}", path.display()))?;
        fs::create_dir_all(parent)
            .map_err(|error| format!("create config directory {}: {error}", parent.display()))?;
        let contents = toml::to_string_pretty(self)
            .map_err(|error| format!("serialize config {}: {error}", path.display()))?;
        let mut temporary = NamedTempFile::new_in(parent)
            .map_err(|error| format!("create temporary config in {}: {error}", parent.display()))?;
        temporary
            .write_all(contents.as_bytes())
            .and_then(|()| temporary.as_file_mut().sync_all())
            .map_err(|error| format!("write temporary config for {}: {error}", path.display()))?;
        temporary
            .persist(path)
            .map_err(|error| format!("replace config {}: {}", path.display(), error.error))?;
        Ok(())
    }

    pub fn output_path(&self) -> Result<PathBuf, String> {
        expand_home(
            &self.output_dir,
            env::var_os("HOME").as_deref().map(Path::new),
        )
    }
}

pub fn default_path() -> Result<PathBuf, String> {
    let home = env::var_os("HOME").ok_or_else(|| "HOME is not set".to_owned())?;
    Ok(PathBuf::from(home).join(".config/jj-llms-txt-wiki/config.toml"))
}

pub fn expand_home(value: &str, home: Option<&Path>) -> Result<PathBuf, String> {
    let expanded = if value == "~" {
        home.ok_or_else(|| "HOME is not set".to_owned())?
            .to_path_buf()
    } else if let Some(suffix) = value.strip_prefix("~/") {
        home.ok_or_else(|| "HOME is not set".to_owned())?
            .join(suffix)
    } else if value.starts_with('~') {
        return Err(format!("unsupported home path: {value}"));
    } else {
        PathBuf::from(value)
    };

    if !expanded.is_absolute() {
        return Err(format!(
            "output_dir must be absolute or start with ~/: {value}"
        ));
    }
    Ok(expanded)
}

#[cfg(test)]
mod tests {
    use super::{Config, SiteConfig, default_path, expand_home};
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::{Path, PathBuf};
    use tempfile::tempdir;

    #[test]
    fn missing_config_uses_documented_defaults() {
        let directory = tempdir().expect("tempdir");
        let config = Config::load(&directory.path().join("missing.toml")).expect("default config");
        assert_eq!(config, Config::default());
    }

    #[test]
    fn loads_partial_and_complete_toml() {
        let directory = tempdir().expect("tempdir");
        let path = directory.path().join("config.toml");
        fs::write(&path, "concurrency = 2\n").expect("write partial config");
        let partial = Config::load(&path).expect("partial config");
        assert_eq!(partial.concurrency, 2);
        assert_eq!(partial.output_dir, "~/.config/jj-llms-txt-wiki/wiki");

        fs::write(
            &path,
            "output_dir = \"/tmp/wiki\"\nconcurrency = 3\ninterval_ms = 20\n[sites.docs]\nurl = \"https://example.com/llms.txt\"\n",
        )
        .expect("write complete config");
        let complete = Config::load(&path).expect("complete config");
        assert_eq!(complete.interval_ms, 20);
        assert_eq!(complete.sites.len(), 1);
    }

    #[test]
    fn loads_single_and_multi_entry_sites() {
        let directory = tempdir().expect("tempdir");
        let path = directory.path().join("config.toml");
        fs::write(
            &path,
            "[sites.one]\nurl = \"https://example.com/llms.txt\"\n\
             [sites.many]\nurls = [\n  \"https://example.com/workers/llms.txt\",\n  \"https://example.com/pages/llms.txt\",\n]\n",
        )
        .expect("write config");
        let config = Config::load(&path).expect("multi-entry config");
        assert_eq!(config.sites["one"].urls.len(), 1);
        assert_eq!(config.sites["many"].urls.len(), 2);
    }

    #[test]
    fn rejects_unusable_site_entry_declarations() {
        let directory = tempdir().expect("tempdir");
        let path = directory.path().join("config.toml");
        for site in [
            // Both spellings at once: no obviously right precedence.
            "url = \"https://example.com/llms.txt\"\nurls = [\"https://example.com/a/llms.txt\"]\n",
            // A typo must surface, not silently leave the site with no entry.
            "urlz = [\"https://example.com/llms.txt\"]\n",
            "urls = []\n",
            "urls = [\"https://example.com/llms.txt\", \"https://example.com/llms.txt\"]\n",
            // The two chains disagree on snapshot strategy.
            "urls = [\"https://example.com/llms.txt\", \"https://example.com/llms-full.txt\"]\n",
            "urls = [\"/relative/llms.txt\"]\n",
        ] {
            fs::write(&path, format!("[sites.docs]\n{site}")).expect("write config");
            assert!(Config::load(&path).is_err(), "{site}");
        }
    }

    #[test]
    fn round_trips_each_entry_spelling_unchanged() {
        let directory = tempdir().expect("tempdir");
        let path = directory.path().join("config.toml");
        let mut sites = BTreeMap::new();
        sites.insert(
            "one".to_owned(),
            SiteConfig::new(vec!["https://example.com/llms.txt".to_owned()]),
        );
        sites.insert(
            "many".to_owned(),
            SiteConfig::new(vec![
                "https://example.com/workers/llms.txt".to_owned(),
                "https://example.com/pages/llms.txt".to_owned(),
            ]),
        );
        let config = Config {
            sites,
            ..Config::default()
        };
        config.save(&path).expect("save config");

        let written = fs::read_to_string(&path).expect("read config");
        // A single-entry site keeps the original spelling, so enabling multi-entry
        // support does not rewrite every existing site on the next save.
        assert!(
            written.contains("url = \"https://example.com/llms.txt\""),
            "{written}"
        );
        assert!(written.contains("urls = ["), "{written}");
        assert_eq!(Config::load(&path).unwrap(), config);
    }

    #[test]
    fn rejects_invalid_toml_and_zero_concurrency() {
        let directory = tempdir().expect("tempdir");
        let path = directory.path().join("config.toml");
        fs::write(&path, "not toml").expect("write invalid config");
        assert!(Config::load(&path).is_err());
        fs::write(&path, "concurrency = 0\n").expect("write zero config");
        assert!(Config::load(&path).is_err());
    }

    #[test]
    fn expands_supported_home_paths() {
        let home = Path::new("/Users/test");
        assert_eq!(expand_home("~", Some(home)).unwrap(), home);
        assert_eq!(
            expand_home("~/wiki", Some(home)).unwrap(),
            PathBuf::from("/Users/test/wiki")
        );
        assert_eq!(
            expand_home("/var/tmp/wiki", None).unwrap(),
            PathBuf::from("/var/tmp/wiki")
        );
        assert!(expand_home("~other/wiki", Some(home)).is_err());
        assert!(expand_home("relative/wiki", Some(home)).is_err());
        assert!(expand_home("~/wiki", None).is_err());
    }

    #[test]
    fn saves_atomically_and_round_trips() {
        let directory = tempdir().expect("tempdir");
        let path = directory.path().join("nested/config.toml");
        let mut sites = BTreeMap::new();
        sites.insert(
            "docs".to_owned(),
            SiteConfig::new(vec!["https://example.com/llms.txt".to_owned()]),
        );
        let config = Config {
            sites,
            ..Config::default()
        };
        config.save(&path).expect("save config");
        assert_eq!(Config::load(&path).unwrap(), config);
        assert_eq!(
            fs::read_dir(path.parent().unwrap()).unwrap().count(),
            1,
            "temporary file must be cleaned"
        );
    }

    #[test]
    fn failed_replace_preserves_existing_target() {
        let directory = tempdir().expect("tempdir");
        let path = directory.path().join("config.toml");
        fs::create_dir(&path).expect("create target directory");
        assert!(Config::default().save(&path).is_err());
        assert!(path.is_dir());
    }

    #[test]
    fn default_path_requires_home() {
        if std::env::var_os("HOME").is_some() {
            assert!(
                default_path()
                    .unwrap()
                    .ends_with(".config/jj-llms-txt-wiki/config.toml")
            );
        }
    }
}
