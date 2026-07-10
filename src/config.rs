use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

use crate::site::{parse_entry_url, validate_name};

pub const DEFAULT_OUTPUT_DIR: &str = "~/llms-wiki";
pub const DEFAULT_CONCURRENCY: usize = 4;
pub const DEFAULT_INTERVAL_MS: u64 = 500;

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SiteConfig {
    pub url: String,
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
            parse_entry_url(&site.url)
                .map_err(|error| format!("site {name} has invalid url: {error}"))?;
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
    Ok(PathBuf::from(home).join(".config/llms-wiki/config.toml"))
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
        assert_eq!(partial.output_dir, "~/llms-wiki");

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
            SiteConfig {
                url: "https://example.com/llms.txt".to_owned(),
            },
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
                    .ends_with(".config/llms-wiki/config.toml")
            );
        }
    }
}
