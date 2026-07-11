use percent_encoding::percent_decode_str;
use std::collections::{HashMap, HashSet};
use std::path::{Component, Path, PathBuf};
use std::sync::{Arc, Mutex};
use url::Url;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CanonicalUrl(Url);

impl CanonicalUrl {
    pub fn new(mut url: Url) -> Self {
        url.set_fragment(None);
        Self(url)
    }

    pub fn as_url(&self) -> &Url {
        &self.0
    }
}

impl std::fmt::Display for CanonicalUrl {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocalPath(PathBuf);

impl LocalPath {
    pub fn as_path(&self) -> &Path {
        &self.0
    }

    pub fn join_under(&self, root: &Path) -> Result<PathBuf, String> {
        if !root.is_absolute() {
            return Err(format!(
                "snapshot root must be absolute: {}",
                root.display()
            ));
        }
        let target = root.join(&self.0);
        if !target.starts_with(root) {
            return Err(format!("path escapes snapshot root: {}", self.0.display()));
        }
        Ok(target)
    }
}

/// Canonical origin string (`scheme://host[:non-default-port]`) used to compare
/// origins. `Url::origin().ascii_serialization()` normalizes default ports, so
/// `https://x:443` and `https://x` collapse to the same key.
pub fn origin_key(url: &Url) -> String {
    url.origin().ascii_serialization()
}

/// The set of origins the crawl is allowed to fetch from.
///
/// An `llms.txt` entry is often served on an alias host that differs from the
/// host its content links point to (e.g. `bun.sh/docs/llms.txt` lists
/// `bun.com/...md`), so a strict "same origin as the entry URL" rule downloads
/// nothing. Instead the entry document is trusted to declare its own content
/// origins: the set is seeded with the entry origin, then expanded once with the
/// origins of the syncable links inside the entry document, and frozen. Every
/// discovered link and redirect target must belong to this set — content pages
/// cannot expand it, so the crawl never escapes the site the entry vouched for.
///
/// Cloneable and internally synchronized: the same set backs both link discovery
/// and the HTTP redirect policy. The one expansion happens while processing the
/// entry, strictly before any content fetch is spawned, so concurrent reads
/// always observe the frozen set.
#[derive(Clone, Debug)]
pub struct AllowedOrigins {
    origins: Arc<Mutex<HashSet<String>>>,
}

impl AllowedOrigins {
    pub fn new(entry: &Url) -> Self {
        Self {
            origins: Arc::new(Mutex::new(HashSet::from([origin_key(entry)]))),
        }
    }

    pub fn allow(&self, url: &Url) {
        self.origins.lock().unwrap().insert(origin_key(url));
    }

    pub fn contains(&self, url: &Url) -> bool {
        self.origins.lock().unwrap().contains(&origin_key(url))
    }
}

/// A link worth following: a Markdown document (`.md`/`.markdown`) or a nested
/// `llms.txt` index. An llms.txt lists its section's pages, so following one is
/// how discovery reaches the actual docs — e.g. Cloudflare's root llms.txt links
/// only to per-product llms.txt indexes, which in turn link to the `.md` pages.
/// The `/llms.txt` suffix (not bare `llms.txt`) avoids matching `foollms.txt`.
pub fn is_syncable_url(url: &Url) -> bool {
    let path = url.path().to_ascii_lowercase();
    path.ends_with(".md") || path.ends_with(".markdown") || path.ends_with("/llms.txt")
}

pub fn has_encoded_unsafe_segment(value: &str) -> bool {
    let path = value.split(['?', '#']).next().unwrap_or(value);
    path.split('/').any(|segment| {
        if !segment.contains('%') {
            return false;
        }
        let Ok(decoded) = percent_decode_str(segment).decode_utf8() else {
            return true;
        };
        decoded == "."
            || decoded == ".."
            || decoded.contains('/')
            || decoded.contains('\\')
            || decoded.contains('\0')
    })
}

pub fn local_path(url: &Url) -> Result<LocalPath, String> {
    let path = url
        .path()
        .strip_prefix('/')
        .ok_or_else(|| format!("URL path is not absolute: {url}"))?;
    if path.is_empty() || path.ends_with('/') {
        return Err(format!("URL path is not a file: {url}"));
    }

    let mut local = PathBuf::new();
    for segment in path.split('/') {
        if segment.is_empty() {
            return Err(format!("URL path contains an empty segment: {url}"));
        }
        let decoded = percent_decode_str(segment)
            .decode_utf8()
            .map_err(|_| format!("URL path contains invalid UTF-8: {url}"))?;
        if decoded == "."
            || decoded == ".."
            || decoded.contains('/')
            || decoded.contains('\\')
            || decoded.contains('\0')
        {
            return Err(format!("URL path contains an unsafe segment: {url}"));
        }
        local.push(segment);
    }

    if local
        .components()
        .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(format!("URL path is unsafe: {url}"));
    }
    Ok(LocalPath(local))
}

#[derive(Debug, Default)]
pub struct PathRegistry {
    urls: HashMap<LocalPath, CanonicalUrl>,
}

impl PathRegistry {
    pub fn register(&mut self, url: &CanonicalUrl) -> Result<LocalPath, String> {
        let path = local_path(url.as_url())?;
        if let Some(existing) = self.urls.get(&path) {
            if existing != url {
                return Err(format!(
                    "URL path collision: {existing} and {url} both map to {}",
                    path.as_path().display()
                ));
            }
        } else {
            self.urls.insert(path.clone(), url.clone());
        }
        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AllowedOrigins, CanonicalUrl, PathRegistry, has_encoded_unsafe_segment, is_syncable_url,
        local_path,
    };
    use std::path::{Path, PathBuf};
    use url::Url;

    fn url(value: &str) -> Url {
        Url::parse(value).unwrap()
    }

    #[test]
    fn allowed_origins_seed_with_entry_and_normalize_default_port() {
        let allowed = AllowedOrigins::new(&url("https://example.com/llms.txt"));
        assert!(allowed.contains(&url("https://example.com:443/a.md")));
        assert!(!allowed.contains(&url("http://example.com/a.md")));
        assert!(!allowed.contains(&url("https://other.test/a.md")));
        assert!(!allowed.contains(&url("https://example.com:444/a.md")));
    }

    #[test]
    fn allowed_origins_expand_then_admit_new_origin() {
        let allowed = AllowedOrigins::new(&url("https://bun.sh/docs/llms.txt"));
        assert!(!allowed.contains(&url("https://bun.com/docs/a.md")));
        allowed.allow(&url("https://bun.com/docs/a.md"));
        assert!(allowed.contains(&url("https://bun.com/docs/other.md")));
        // The entry origin stays allowed after expansion.
        assert!(allowed.contains(&url("https://bun.sh/docs/x.md")));
    }

    #[test]
    fn recognizes_markdown_and_llms_txt_urls_case_insensitively() {
        assert!(is_syncable_url(&url("https://example.com/a.md")));
        assert!(is_syncable_url(&url("https://example.com/a.MARKDOWN?q=1")));
        assert!(is_syncable_url(&url("https://example.com/llms.txt")));
        assert!(is_syncable_url(&url("https://example.com/cache/llms.txt")));
        assert!(is_syncable_url(&url("https://example.com/cache/LLMS.TXT")));
        assert!(!is_syncable_url(&url("https://example.com/a.md/child")));
        assert!(!is_syncable_url(&url("https://example.com/a.html")));
        assert!(!is_syncable_url(&url("https://example.com/foollms.txt")));
    }

    #[test]
    fn canonical_url_removes_fragment_and_keeps_query() {
        let canonical = CanonicalUrl::new(url("https://example.com/a.md?q=1#part"));
        assert_eq!(canonical.to_string(), "https://example.com/a.md?q=1");
    }

    #[test]
    fn maps_url_path_without_decoding_safe_segments() {
        let path = local_path(&url("https://example.com/docs/%E6%97%A5%E6%9C%AC.md")).unwrap();
        assert_eq!(path.as_path(), Path::new("docs/%E6%97%A5%E6%9C%AC.md"));
        assert_eq!(
            path.join_under(Path::new("/tmp/site")).unwrap(),
            PathBuf::from("/tmp/site/docs/%E6%97%A5%E6%9C%AC.md")
        );
    }

    #[test]
    fn rejects_unsafe_url_paths() {
        for value in [
            "https://example.com/",
            "https://example.com/docs/",
            "https://example.com/docs//a.md",
            "https://example.com/docs/%2Fetc.md",
            "https://example.com/docs/%5Cetc.md",
            "https://example.com/docs/%00.md",
        ] {
            assert!(local_path(&url(value)).is_err(), "{value}");
        }
    }

    #[test]
    fn detects_encoded_traversal_before_url_normalization() {
        for value in ["../a.md", "./a.md", "/safe/%E6%97%A5.md"] {
            assert!(!has_encoded_unsafe_segment(value), "{value}");
        }
        for value in ["%2e%2e/a.md", "/safe/%2E/a.md", "/safe/%2fetc.md"] {
            assert!(has_encoded_unsafe_segment(value), "{value}");
        }
    }

    #[test]
    fn detects_query_path_collisions() {
        let mut registry = PathRegistry::default();
        registry
            .register(&CanonicalUrl::new(url("https://example.com/a.md?q=1")))
            .unwrap();
        assert!(
            registry
                .register(&CanonicalUrl::new(url("https://example.com/a.md?q=2")))
                .is_err()
        );
    }
}
