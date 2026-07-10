use comrak::nodes::NodeValue;
use comrak::{Arena, Options, parse_document};
use std::collections::HashSet;
use url::Url;

use crate::url_map::{CanonicalUrl, has_encoded_unsafe_segment, is_markdown_url, same_origin};

pub fn discover(markdown: &str, base: &Url, entry: &Url) -> Vec<CanonicalUrl> {
    let arena = Arena::new();
    let mut options = Options::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.tasklist = true;
    let root = parse_document(&arena, markdown, &options);
    let canonical_entry = CanonicalUrl::new(entry.clone());
    let mut found = HashSet::new();

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
        if !same_origin(entry, &url) || !is_markdown_url(&url) {
            continue;
        }
        let canonical = CanonicalUrl::new(url);
        if canonical != canonical_entry {
            found.insert(canonical);
        }
    }

    let mut urls: Vec<_> = found.into_iter().collect();
    urls.sort_by(|left, right| left.as_url().as_str().cmp(right.as_url().as_str()));
    urls
}

#[cfg(test)]
mod tests {
    use super::discover;
    use url::Url;

    fn strings(markdown: &str, base: &str, entry: &str) -> Vec<String> {
        discover(
            markdown,
            &Url::parse(base).unwrap(),
            &Url::parse(entry).unwrap(),
        )
        .into_iter()
        .map(|url| url.to_string())
        .collect()
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
