use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use url::Url;

use crate::crawler::{CrawlEvent, CrawlFailure, CrawlObserver, CrawlReport, write_document};
use crate::http::{FetchOutcome, HttpClient};
use crate::site::parse_entry_url;
use crate::url_map::{
    AllowedOrigins, CanonicalUrl, PathRegistry, full_markdown_path, has_encoded_unsafe_segment,
};

#[derive(Debug)]
struct FullPage {
    url: Url,
    markdown: String,
}

#[derive(Clone, Copy)]
struct Line<'a> {
    number: usize,
    start: usize,
    end: usize,
    text: &'a str,
    code: bool,
}

/// How a page header declares its URL. A bundle uses exactly one variant; the
/// marker form wins when both could match, so adding the bare form cannot change
/// how an already-working bundle is split.
#[derive(Clone, Copy, PartialEq, Eq)]
enum HeaderKind {
    /// `URL: <url>` on its own line, below an H1 and an optional quote block.
    /// Heading and quote block are page content and stay in the output.
    Marker,
    /// A bare URL on the line immediately below a heading (HuggingFace style).
    /// The heading is a bundle-level separator, not page content — every page
    /// body carries its own title — so it is dropped from the output.
    Bare,
}

struct PageHeader {
    heading: usize,
    url_line: usize,
    boundary: usize,
    url: Url,
    kind: HeaderKind,
}

pub async fn crawl(
    entry: Url,
    snapshot_root: &Path,
    timeout: Duration,
    observer: Arc<dyn CrawlObserver>,
) -> Result<CrawlReport, String> {
    if !snapshot_root.is_absolute() {
        return Err(format!(
            "snapshot root must be absolute: {}",
            snapshot_root.display()
        ));
    }

    let allowed = AllowedOrigins::new(&entry);
    let client = HttpClient::new(&allowed, timeout)?;
    let canonical_entry = CanonicalUrl::new(entry);
    observer.event(CrawlEvent::Started(canonical_entry.to_string()));
    // `None` disables the size cap: an aggregate llms-full.txt is legitimately
    // large, so the per-document cap (a content-page rule) must not apply here.
    let outcome = client.fetch(&canonical_entry, None, None).await;
    observer.event(CrawlEvent::Finished(canonical_entry.to_string()));
    let body = match outcome {
        Ok(FetchOutcome::Document { body, .. }) => body,
        Ok(FetchOutcome::Missing) => {
            return Err(format!("llms-full.txt entry is missing: {canonical_entry}"));
        }
        Ok(FetchOutcome::IgnoredRedirect) => {
            return Err(format!(
                "llms-full.txt entry redirected outside its origin: {canonical_entry}"
            ));
        }
        Ok(FetchOutcome::Oversize { .. }) => {
            return Err("unexpected oversize outcome for llms-full.txt entry".to_owned());
        }
        Ok(FetchOutcome::NotModified { .. }) => {
            return Err("unexpected 304 Not Modified for llms-full.txt entry".to_owned());
        }
        Err(error) => return Err(error),
    };

    let pages = split(&body)?;
    let mut registry = PathRegistry::default();
    let mut outputs = Vec::with_capacity(pages.len());
    for page in pages {
        let canonical = CanonicalUrl::new(page.url);
        let path = full_markdown_path(canonical.as_url())?;
        let path = registry.register_path(&canonical, path)?;
        outputs.push((canonical, path, page.markdown));
    }

    let mut report = CrawlReport::default();
    for (url, path, markdown) in &outputs {
        if let Err(error) = write_document(snapshot_root, path, markdown).await {
            observer.event(CrawlEvent::Failed(url.to_string()));
            report.failures.push(CrawlFailure {
                url: url.to_string(),
                message: error,
            });
            return Ok(report);
        }
    }

    report.downloaded = outputs.len();
    observer.event(CrawlEvent::Downloaded(canonical_entry.to_string()));
    Ok(report)
}

fn split(body: &str) -> Result<Vec<FullPage>, String> {
    let lines = scan_lines(body);
    let headers = collect_headers(&lines)?;
    if headers.is_empty() {
        return Err("llms-full.txt contains no valid page headers".to_owned());
    }

    let mut pages = Vec::with_capacity(headers.len());
    for (index, header) in headers.iter().enumerate() {
        let heading = lines[header.heading];
        let url_line = lines[header.url_line];
        let end = headers
            .get(index + 1)
            .map_or(body.len(), |next| next.boundary);
        let page_body = trim_newlines(&body[url_line.end..end]);
        let markdown = match header.kind {
            HeaderKind::Marker => {
                let head = body[heading.start..url_line.start].trim_end();
                if page_body.is_empty() {
                    format!("{head}\n")
                } else {
                    format!("{head}\n\n{page_body}\n")
                }
            }
            // The separator heading is dropped, so a page with no body would
            // otherwise become an empty file — fall back to its title as an H1.
            HeaderKind::Bare if page_body.is_empty() => {
                format!("# {}\n", heading_title(heading.text).unwrap_or_default())
            }
            HeaderKind::Bare => format!("{page_body}\n"),
        };
        pages.push(FullPage {
            url: header.url.clone(),
            markdown,
        });
    }
    Ok(pages)
}

/// Pick the bundle's header variant and collect its pages. Marker headers win:
/// a bundle that declares even one `URL:` marker is split exactly as before, so
/// the bare-URL form can never reinterpret an already-working bundle.
fn collect_headers(lines: &[Line<'_>]) -> Result<Vec<PageHeader>, String> {
    let markers = collect_marker_headers(lines)?;
    if markers.is_empty() {
        return collect_bare_headers(lines);
    }
    Ok(markers)
}

fn collect_marker_headers(lines: &[Line<'_>]) -> Result<Vec<PageHeader>, String> {
    let mut seen_urls = HashSet::new();
    let mut headers = Vec::new();
    for (heading, line) in lines.iter().enumerate() {
        if line.code || heading_level(line.text) != Some(1) {
            continue;
        }
        let Some(url_line) = header_url_line(lines, heading) else {
            continue;
        };
        let raw = url_marker(lines[url_line].text).expect("header URL marker");
        if raw.is_empty() {
            return Err(format!(
                "empty URL marker at line {}",
                lines[url_line].number
            ));
        }
        let url = accept_page_url(raw, lines[url_line].number, &mut seen_urls)?;
        headers.push(PageHeader {
            heading,
            url_line,
            boundary: page_boundary(lines, heading),
            url,
            kind: HeaderKind::Marker,
        });
    }
    Ok(headers)
}

/// Headers of the form `### Title` immediately followed by a bare URL line.
///
/// The pattern is loose enough to appear by accident inside prose, so only the
/// dominant heading level is accepted: a bundle separates its pages at one fixed
/// level, and a stray `#### Ref` + link in body text is a different one. That
/// leaves same-level accidents, which then have to carry a page-shaped URL and
/// stay unique across the bundle — cheap enough to keep a rare false page from
/// stealing a slice of its neighbour.
fn collect_bare_headers(lines: &[Line<'_>]) -> Result<Vec<PageHeader>, String> {
    let candidates: Vec<(usize, usize)> = lines
        .iter()
        .enumerate()
        .filter_map(|(heading, line)| {
            if line.code {
                return None;
            }
            let level = heading_level(line.text)?;
            let next = lines.get(heading + 1)?;
            (!next.code && bare_url(next.text).is_some()).then_some((heading, level))
        })
        .collect();
    let Some(level) = dominant_level(&candidates) else {
        return Ok(Vec::new());
    };

    let mut seen_urls = HashSet::new();
    let mut headers = Vec::new();
    for (heading, _) in candidates.iter().filter(|(_, found)| *found == level) {
        let url_line = heading + 1;
        let raw = bare_url(lines[url_line].text).expect("bare header URL");
        let url = accept_page_url(raw, lines[url_line].number, &mut seen_urls)?;
        headers.push(PageHeader {
            heading: *heading,
            url_line,
            boundary: page_boundary(lines, *heading),
            url,
            kind: HeaderKind::Bare,
        });
    }
    Ok(headers)
}

/// The heading level carrying the most header candidates; ties go to whichever
/// level appears first, so the choice never depends on iteration order.
fn dominant_level(candidates: &[(usize, usize)]) -> Option<usize> {
    let mut counts = [0usize; 7];
    for (_, level) in candidates {
        counts[*level] += 1;
    }
    let most = *counts.iter().max()?;
    if most == 0 {
        return None;
    }
    candidates
        .iter()
        .map(|(_, level)| *level)
        .find(|level| counts[*level] == most)
}

fn accept_page_url(
    raw: &str,
    line: usize,
    seen: &mut HashSet<CanonicalUrl>,
) -> Result<Url, String> {
    if has_encoded_unsafe_segment(raw) {
        return Err(format!("unsafe page URL at line {line}: {raw}"));
    }
    let url = parse_entry_url(raw)
        .map_err(|error| format!("invalid page URL at line {line}: {error}"))?;
    if !seen.insert(CanonicalUrl::new(url.clone())) {
        return Err(format!("duplicate page URL at line {line}: {url}"));
    }
    Ok(url)
}

fn scan_lines(body: &str) -> Vec<Line<'_>> {
    let mut lines = Vec::new();
    let mut offset = 0;
    let mut fence: Option<(u8, usize)> = None;
    for (index, chunk) in body.split_inclusive('\n').enumerate() {
        let end = offset + chunk.len();
        let text = chunk.strip_suffix('\n').unwrap_or(chunk);
        let text = text.strip_suffix('\r').unwrap_or(text);
        let marker = fence_marker(text);
        let code = if let Some((kind, length)) = fence {
            if marker.is_some_and(|(candidate, run, tail)| {
                candidate == kind && run >= length && tail.trim().is_empty()
            }) {
                fence = None;
            }
            true
        } else if let Some((kind, length, _)) = marker {
            fence = Some((kind, length));
            true
        } else {
            false
        };
        lines.push(Line {
            number: index + 1,
            start: offset,
            end,
            text,
            code,
        });
        offset = end;
    }
    lines
}

fn fence_marker(line: &str) -> Option<(u8, usize, &str)> {
    let bytes = line.as_bytes();
    let indent = bytes.iter().take_while(|byte| **byte == b' ').count();
    if indent > 3 || indent == bytes.len() {
        return None;
    }
    let rest = &line[indent..];
    let kind = *rest.as_bytes().first()?;
    if !matches!(kind, b'`' | b'~') {
        return None;
    }
    let length = rest
        .as_bytes()
        .iter()
        .take_while(|byte| **byte == kind)
        .count();
    (length >= 3).then(|| (kind, length, &rest[length..]))
}

fn header_url_line(lines: &[Line<'_>], heading: usize) -> Option<usize> {
    let mut cursor = heading + 1;
    skip_blanks(lines, &mut cursor);
    while lines.get(cursor).is_some_and(|line| {
        !line.code && markdown_content(line.text).is_some_and(|content| content.starts_with('>'))
    }) {
        cursor += 1;
        skip_blanks(lines, &mut cursor);
    }
    lines
        .get(cursor)
        .filter(|line| !line.code && url_marker(line.text).is_some())
        .map(|_| cursor)
}

fn skip_blanks(lines: &[Line<'_>], cursor: &mut usize) {
    while lines
        .get(*cursor)
        .is_some_and(|line| !line.code && line.text.trim().is_empty())
    {
        *cursor += 1;
    }
}

fn page_boundary(lines: &[Line<'_>], heading: usize) -> usize {
    let mut cursor = heading;
    while cursor > 0 && lines[cursor - 1].text.trim().is_empty() {
        cursor -= 1;
    }
    if cursor > 0
        && !lines[cursor - 1].code
        && markdown_content(lines[cursor - 1].text).is_some_and(|line| line.trim_end() == "---")
    {
        lines[cursor - 1].start
    } else {
        lines[heading].start
    }
}

/// ATX heading level (1-6) of a line with a non-empty title, else `None`.
fn heading_level(line: &str) -> Option<usize> {
    let content = markdown_content(line)?;
    let level = content.bytes().take_while(|byte| *byte == b'#').count();
    if level == 0 || level > 6 {
        return None;
    }
    let title = content.get(level..)?;
    let title = title
        .strip_prefix(' ')
        .or_else(|| title.strip_prefix('\t'))?;
    (!title.trim().is_empty()).then_some(level)
}

fn heading_title(line: &str) -> Option<&str> {
    let content = markdown_content(line)?;
    let level = heading_level(line)?;
    Some(content[level..].trim())
}

/// A line that is nothing but an absolute HTTP(S) URL. Anything alongside it —
/// prose, Markdown link syntax, a trailing note — disqualifies the line, which
/// is what keeps ordinary body text from reading as a page header.
fn bare_url(line: &str) -> Option<&str> {
    let value = markdown_content(line)?.trim();
    if value.contains(char::is_whitespace) {
        return None;
    }
    ["http://", "https://"]
        .iter()
        .any(|scheme| {
            value.len() > scheme.len()
                && value
                    .get(..scheme.len())
                    .is_some_and(|prefix| prefix.eq_ignore_ascii_case(scheme))
        })
        .then_some(value)
}

fn url_marker(line: &str) -> Option<&str> {
    markdown_content(line)?
        .trim_end()
        .strip_prefix("URL:")
        .map(str::trim)
}

fn markdown_content(line: &str) -> Option<&str> {
    let indent = line
        .as_bytes()
        .iter()
        .take_while(|byte| **byte == b' ')
        .count();
    (indent <= 3).then(|| &line[indent..])
}

fn trim_newlines(value: &str) -> &str {
    value.trim_matches(['\r', '\n'])
}

#[cfg(test)]
mod tests {
    use super::split;

    #[test]
    fn splits_pages_and_preserves_page_content() {
        let pages = split(
            "# Bundle\n\n> Preamble\n\n# One\n\nURL: https://example.com/one\n\nBody one.\n\n---\n\n# Two\n\n> Description\n\nURL: https://example.com/two/\n\nBody two.\n",
        )
        .unwrap();
        assert_eq!(pages.len(), 2);
        assert_eq!(pages[0].url.as_str(), "https://example.com/one");
        assert_eq!(pages[0].markdown, "# One\n\nBody one.\n");
        assert_eq!(pages[1].markdown, "# Two\n\n> Description\n\nBody two.\n");
    }

    #[test]
    fn ignores_non_header_markers_and_body_separators() {
        let pages = split(
            "# One\n\nURL: https://example.com/one\n\nBefore\n\nURL:\n\n---\n\n```text\n# Fake\nURL: https://evil.test/fake\n---\n```\n\nAfter\n",
        )
        .unwrap();
        assert_eq!(pages.len(), 1);
        assert!(pages[0].markdown.contains("\nURL:\n"));
        assert!(pages[0].markdown.contains("URL: https://evil.test/fake"));
        assert!(pages[0].markdown.contains("After"));
    }

    #[test]
    fn rejects_ambiguous_or_invalid_bundles() {
        for body in [
            "# No pages\n",
            "# One\n\nURL:\n\nBody\n",
            "    # Indented code\n\n    URL: https://example.com/code\n",
            "# One\n\nURL: https://example.com/one\n\nBody\n\n---\n\n# Duplicate\n\nURL: https://example.com/one#part\n\nBody\n",
            "# One\n\nURL: file:///tmp/one\n\nBody\n",
            "# One\n\nURL: https://example.com/%2e%2e/one\n\nBody\n",
        ] {
            assert!(split(body).is_err(), "{body}");
        }
    }

    #[test]
    fn accepts_title_only_pages() {
        let pages = split("# Empty\n\nURL: https://example.com/empty\n").unwrap();
        assert_eq!(pages[0].markdown, "# Empty\n");
    }

    /// HuggingFace style: an llms.txt-shaped index, then pages separated by a
    /// heading plus a bare URL. The separator heading is dropped — each body
    /// carries its own title — and the leading index is not a page.
    #[test]
    fn splits_bare_url_headers_and_drops_separators() {
        let pages = split(concat!(
            "# Bundle\n\n## Docs\n\n- [One](https://example.com/one.md)\n\n",
            "### One\nhttps://example.com/one.md\n\n# One\n\nBody one.\n\n",
            "### Two\nhttps://example.com/two.md\n\n# Two\n\nBody two.\n",
        ))
        .unwrap();
        assert_eq!(pages.len(), 2);
        assert_eq!(pages[0].url.as_str(), "https://example.com/one.md");
        assert_eq!(pages[0].markdown, "# One\n\nBody one.\n");
        assert_eq!(pages[1].markdown, "# Two\n\nBody two.\n");
    }

    #[test]
    fn marker_headers_win_over_bare_urls() {
        let pages = split(
            "# One\n\nURL: https://example.com/one\n\n### Link\nhttps://example.com/two\n\nBody.\n",
        )
        .unwrap();
        assert_eq!(pages.len(), 1);
        assert!(pages[0].markdown.contains("https://example.com/two"));
    }

    /// Only the dominant heading level separates pages: a lone `#### Ref` plus a
    /// link inside body text is prose, not a page.
    #[test]
    fn bare_url_headers_ignore_off_level_and_non_url_lines() {
        let pages = split(concat!(
            "### One\nhttps://example.com/one.md\n\n#### Ref\nhttps://example.com/paper\n\nBody.\n\n",
            "### Two\nhttps://example.com/two.md\n\n## See\n[link](https://example.com/x)\n\nEnd.\n",
        ))
        .unwrap();
        assert_eq!(pages.len(), 2);
        assert!(pages[0].markdown.contains("#### Ref"));
        assert!(pages[1].markdown.contains("[link](https://example.com/x)"));
    }

    #[test]
    fn bare_url_headers_reject_unsafe_and_duplicate_urls() {
        for body in [
            "### One\nhttps://example.com/one.md\n\n### Same\nhttps://example.com/one.md#part\n",
            "### One\nhttps://example.com/%2e%2e/one.md\n",
            "### One\nfile:///tmp/one.md\n\n### Two\nfile:///tmp/two.md\n",
            "### None\nnot-a-url\n",
            "```text\n### Fake\nhttps://evil.test/fake.md\n```\n",
        ] {
            assert!(split(body).is_err(), "{body}");
        }
    }

    #[test]
    fn bare_url_pages_keep_their_title_when_empty() {
        let pages = split("### Empty\nhttps://example.com/empty.md\n").unwrap();
        assert_eq!(pages[0].markdown, "# Empty\n");
    }
}
