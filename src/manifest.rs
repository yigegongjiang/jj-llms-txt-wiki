use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// Per-site sync metadata, stored inside the site directory so it is swapped
/// atomically with content by `Snapshot::commit` and versioned by the data repository.
pub const MANIFEST_FILE: &str = ".jj-llms-txt-wiki.json";

const MANIFEST_VERSION: u32 = 1;

/// HTTP validators for a single document, echoed verbatim in conditional requests.
/// `etag`/`last_modified` are stored exactly as received (quotes, `W/` weak prefix
/// and raw HTTP-date included); any normalization would break server-side matching.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Validator {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
}

impl Validator {
    pub fn is_empty(&self) -> bool {
        self.etag.is_none() && self.last_modified.is_none()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(default)]
    version: u32,
    #[serde(default)]
    entries: BTreeMap<String, Validator>,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            version: MANIFEST_VERSION,
            entries: BTreeMap::new(),
        }
    }
}

impl Manifest {
    /// Load `<site_root>/.jj-llms-txt-wiki.json`. Missing or corrupt manifest degrades to
    /// empty: the cache is self-healing, so the worst case is a full re-download with
    /// no correctness risk (unlike user-authored config, which fails loudly).
    pub fn load(site_root: &Path) -> Self {
        let path = site_root.join(MANIFEST_FILE);
        match fs::read_to_string(&path) {
            Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn get(&self, url: &str) -> Option<&Validator> {
        self.entries.get(url)
    }

    /// Record a validator for `url`. Entries without any validator are dropped:
    /// they carry no conditional-request value and only bloat the manifest.
    pub fn insert(&mut self, url: String, validator: Validator) {
        if !validator.is_empty() {
            self.entries.insert(url, validator);
        }
    }

    /// Write the manifest into `site_root`. Called against the snapshot temporary
    /// directory; atomicity is provided by `Snapshot::commit`'s directory rename.
    pub fn save(&self, site_root: &Path) -> Result<(), String> {
        let path = site_root.join(MANIFEST_FILE);
        let contents = serde_json::to_string_pretty(self)
            .map_err(|error| format!("serialize manifest: {error}"))?;
        fs::write(&path, contents)
            .map_err(|error| format!("write manifest {}: {error}", path.display()))
    }
}

#[cfg(test)]
mod tests {
    use super::{MANIFEST_FILE, Manifest, Validator};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn missing_manifest_loads_empty() {
        let directory = tempdir().unwrap();
        let manifest = Manifest::load(directory.path());
        assert!(manifest.get("https://example.com/a.md").is_none());
    }

    #[test]
    fn corrupt_manifest_degrades_to_empty() {
        let directory = tempdir().unwrap();
        fs::write(directory.path().join(MANIFEST_FILE), "{ not json").unwrap();
        let manifest = Manifest::load(directory.path());
        assert!(manifest.get("https://example.com/a.md").is_none());
    }

    #[test]
    fn round_trips_validators_verbatim() {
        let directory = tempdir().unwrap();
        let mut manifest = Manifest::default();
        manifest.insert(
            "https://example.com/a.md".to_owned(),
            Validator {
                etag: Some("\"abc\"".to_owned()),
                last_modified: Some("Wed, 21 Oct 2015 07:28:00 GMT".to_owned()),
            },
        );
        manifest.insert(
            "https://example.com/weak.md".to_owned(),
            Validator {
                etag: Some("W/\"weak\"".to_owned()),
                last_modified: None,
            },
        );
        manifest.save(directory.path()).unwrap();

        let loaded = Manifest::load(directory.path());
        assert_eq!(
            loaded
                .get("https://example.com/a.md")
                .unwrap()
                .etag
                .as_deref(),
            Some("\"abc\"")
        );
        assert_eq!(
            loaded
                .get("https://example.com/a.md")
                .unwrap()
                .last_modified
                .as_deref(),
            Some("Wed, 21 Oct 2015 07:28:00 GMT")
        );
        assert_eq!(
            loaded
                .get("https://example.com/weak.md")
                .unwrap()
                .etag
                .as_deref(),
            Some("W/\"weak\"")
        );
    }

    #[test]
    fn drops_entries_without_validator() {
        let mut manifest = Manifest::default();
        manifest.insert("https://example.com/a.md".to_owned(), Validator::default());
        assert!(manifest.get("https://example.com/a.md").is_none());
    }
}
