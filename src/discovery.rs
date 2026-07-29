use comrak::nodes::NodeValue;
use comrak::{Arena, Options, parse_document};
use std::collections::HashSet;
use url::Url;

use crate::url_map::{AllowedOrigins, CanonicalUrl, has_encoded_unsafe_segment, is_syncable_url};

/// Parse `markdown` and return every syncable link resolved against `base`,
/// with no origin filtering. Shared by link discovery and origin expansion.
fn syncable_links(markdown: &str, base: &Url) -> Vec<Url> {
    let arena = Arena::new();
    let mut options = Options::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.tasklist = true;
    let root = parse_document(&arena, markdown, &options);
    let mut links = Vec::new();

    for node in root.descendants() {
        let target = {
            let data = node.data.borrow();
            match &data.value {
                NodeValue::Link(link) => link.url.clone(),
                _ => continue,
            }
        };
        if has_encoded_unsafe_segment(&target) {
            continue;
        }
        let Ok(url) = base.join(&target) else {
            continue;
        };
        if is_syncable_url(&url) {
            links.push(url);
        }
    }
    links
}

/// The syncable links declared in `markdown`. Only the entry document feeds
/// these into [`AllowedOrigins::allow`], trusting the hosts the entry vouches
/// for (e.g. a `bun.sh` entry declaring `bun.com` content links).
pub fn declared_links(markdown: &str, base: &Url) -> Vec<Url> {
    syncable_links(markdown, base)
}

/// Discover syncable Markdown links whose origin is allowed, minus the site's own
/// entry documents, sorted and deduplicated. Every entry is excluded, not just the
/// one being parsed: sibling entries are crawled as entries in their own right, so
/// a cross-link between them must not also enqueue one as a content page.
pub fn discover(
    markdown: &str,
    base: &Url,
    entries: &HashSet<CanonicalUrl>,
    allowed: &AllowedOrigins,
) -> Vec<CanonicalUrl> {
    let mut found = HashSet::new();

    for url in syncable_links(markdown, base) {
        if !allowed.contains(&url) {
            continue;
        }
        let canonical = CanonicalUrl::new(url);
        if !entries.contains(&canonical) {
            found.insert(canonical);
        }
    }

    let mut urls: Vec<_> = found.into_iter().collect();
    urls.sort_by(|left, right| left.as_url().as_str().cmp(right.as_url().as_str()));
    urls
}

#[cfg(test)]
mod tests {
    use super::{declared_links, discover};
    use crate::url_map::{AllowedOrigins, CanonicalUrl};
    use std::collections::HashSet;
    use url::Url;

    fn strings(markdown: &str, base: &str, entry: &str) -> Vec<String> {
        entry_strings(markdown, base, &[entry])
    }

    fn entry_strings(markdown: &str, base: &str, entries: &[&str]) -> Vec<String> {
        let parsed: Vec<Url> = entries.iter().map(|url| Url::parse(url).unwrap()).collect();
        // Default allow-list is just the entry origins, matching a same-origin site.
        let allowed = AllowedOrigins::from_entries(&parsed);
        let entry_set: HashSet<CanonicalUrl> = parsed.into_iter().map(CanonicalUrl::new).collect();
        discover(markdown, &Url::parse(base).unwrap(), &entry_set, &allowed)
            .into_iter()
            .map(|url| url.to_string())
            .collect()
    }

    #[test]
    fn excludes_every_entry_of_a_multi_entry_site() {
        // Sibling section indexes cross-link each other; each is crawled as an
        // entry, so neither may also be enqueued as a content page.
        let markdown =
            "[workers](/workers/llms.txt) [pages](/pages/llms.txt) [page](/workers/a.md)";
        assert_eq!(
            entry_strings(
                markdown,
                "https://example.com/workers/llms.txt",
                &[
                    "https://example.com/workers/llms.txt",
                    "https://example.com/pages/llms.txt",
                ]
            ),
            ["https://example.com/workers/a.md"]
        );
    }

    #[test]
    fn resolves_relative_root_and_parent_links() {
        let markdown = "[a](child.md) [b](/root.md) [c](../parent.markdown)";
        assert_eq!(
            strings(
                markdown,
                "https://example.com/docs/current.md",
                "https://example.com/llms.txt"
            ),
            [
                "https://example.com/docs/child.md",
                "https://example.com/parent.markdown",
                "https://example.com/root.md",
            ]
        );
    }

    #[test]
    fn filters_non_links_cross_origin_and_non_markdown() {
        let markdown = r#"
[same](/same.md)
[other](https://other.test/other.md)
[html](/page.html)
![image](/image.md)
<https://example.com/autolink.md>
https://example.com/plain.md
<a href="/html.md">html</a>
`[code](/code.md)`
"#;
        assert_eq!(
            strings(
                markdown,
                "https://example.com/llms.txt",
                "https://example.com/llms.txt"
            ),
            [
                "https://example.com/autolink.md",
                "https://example.com/same.md"
            ]
        );
    }

    #[test]
    fn removes_fragments_keeps_queries_and_deduplicates() {
        let markdown = "[a](/a.md?q=1#one) [b](/a.md?q=1#two) [c](/a.md?q=2)";
        assert_eq!(
            strings(
                markdown,
                "https://example.com/llms.txt",
                "https://example.com/llms.txt"
            ),
            [
                "https://example.com/a.md?q=1",
                "https://example.com/a.md?q=2"
            ]
        );
    }

    #[test]
    fn discovers_nested_llms_txt_indexes() {
        // Cloudflare's root llms.txt links to per-section llms.txt indexes that
        // sit alongside the actual .md pages; both must be discovered.
        let markdown = "[section](/cache/llms.txt) [page](/cache/index.md)";
        assert_eq!(
            strings(
                markdown,
                "https://example.com/llms.txt",
                "https://example.com/llms.txt"
            ),
            [
                "https://example.com/cache/index.md",
                "https://example.com/cache/llms.txt"
            ]
        );
    }

    #[test]
    fn admits_cross_origin_links_once_their_origin_is_allowed() {
        // A bun.sh entry declaring bun.com content: the entry's declared links
        // seed the allow-list, after which discovery follows them.
        let markdown = "[a](https://bun.com/docs/a.md) [b](https://other.test/b.md)";
        let base = Url::parse("https://bun.sh/docs/llms.txt").unwrap();
        let entry = base.clone();
        let allowed = AllowedOrigins::new(&entry);
        for link in declared_links(markdown, &base) {
            allowed.allow(&link);
        }
        // bun.com and other.test were both declared, so both are now allowed.
        let entry_set = HashSet::from([CanonicalUrl::new(entry)]);
        let found: Vec<String> = discover(markdown, &base, &entry_set, &allowed)
            .into_iter()
            .map(|url| url.to_string())
            .collect();
        assert_eq!(
            found,
            ["https://bun.com/docs/a.md", "https://other.test/b.md"]
        );
        // A link to a third origin the entry never declared stays excluded.
        assert!(!allowed.contains(&Url::parse("https://evil.test/x.md").unwrap()));
    }

    #[test]
    fn ignores_links_back_to_entry() {
        assert!(
            strings(
                "[entry](/index.md#again)",
                "https://example.com/index.md",
                "https://example.com/index.md"
            )
            .is_empty()
        );
    }
}
