---
description: Use footnotes in documentation.
title: Footnotes
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Footnotes

Last updated Jun 18, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/formatting/footnotes/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

Use footnotes to add details or context about something without distracting from the main content. We recommend using hover-activated footnotes, but you can also use plain text.

### Hover-activated footnotes

To add hover-activated footnotes, use the following syntax:

```mdx
This is a sentence with a footnote.[^1]

[^1]: A footnote adds details or context.
```

With this type of footnote, you can add the numbers to the MDX file in any order and they will still display in numerical order on the page.

The hover ability of this type of footnote is powered by [tippy.js ↗](https://atomiks.github.io/tippyjs/).

Caution

Do not use this syntax inside `<Tabs>` blocks or in partials included inside `<Tabs>` blocks. It generates a `## Footnotes` heading inside each tab panel, which repeats across tabs and breaks agent-readable documentation. Use plain text footnotes or inline the text instead.

### Plain text footnotes

To add plain text footnotes, use the syntax in this example:

```mdx
This is a sentence with a footnote.<sup>1</sup>

<sup>1</sup> A footnote adds details or context.
```

With this type of footnote, you can add the footnote note anywhere on the page. We recommend adding it to the bottom of the section or table where the footnote is referenced or to the bottom of the page.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.DMYpXs3t.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/formatting/footnotes/#page","headline":"Footnotes · Cloudflare Style Guide","description":"Use footnotes in documentation.","url":"https://developers.cloudflare.com/style-guide/formatting/footnotes/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-06-18","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
