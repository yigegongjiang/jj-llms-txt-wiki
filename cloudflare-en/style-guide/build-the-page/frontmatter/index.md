---
description: Set the required and optional frontmatter fields that carry a page's metadata, such as title, description, and pcx_content_type.
title: Frontmatter
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Frontmatter

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

Frontmatter contains the metadata for a page, such as the `title`. It is written as YAML, between `---`, at the top of the page.

For example:

```yaml
---
title: Create a Cloudflare Tunnel
pcx_content_type: how-to
products:
  - cloudflare-tunnel
description: Set the required and optional frontmatter fields that carry a page's metadata, such as title, description, and pcx_content_type.
sidebar:
  order: 2
---
```

For more information on the available fields, refer to [Nimbus's documentation ↗](https://nimbus-docs.com/writing/frontmatter/).

## Required fields

Every page with a `pcx_content_type` must include:

| Field              | Description                                                                                                                                          |
| ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| title              | The page title. Plain text.                                                                                                                          |
| pcx\_content\_type | The content type of the page. Refer to [content types](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/). |
| description        | A 1-2 sentence summary used for the <meta name="description"> tag. Refer to [writing a description](#writing-a-description).                         |

## Writing a description

The `description` field populates the `<meta name="description">` tag in the HTML head. This is the single most important metadata field for search engines, AI crawlers, and `llms.txt` consumers when deciding whether to surface or cite a page.

A strong description:

* Is 1-2 self-contained sentences (aim for 50-160 characters).
* Names the product or feature.
* States what the page helps the reader do or understand.
* Works as a standalone answer snippet when extracted from the page.

Do not start with generic openers like "This page describes...", "Learn more about...", or "This document explains...". These waste the most valuable metadata space without adding information.

The existing `summary` field remains useful for the on-page experience but is secondary to `description` for AI and search purposes.

### Examples

```yaml
description: Set the required and optional frontmatter fields that carry a page's metadata, such as title, description, and pcx_content_type.
```

```yaml
description: Set the required and optional frontmatter fields that carry a page's metadata, such as title, description, and pcx_content_type.
```

```yaml
description: Set the required and optional frontmatter fields that carry a page's metadata, such as title, description, and pcx_content_type.
```

```yaml
description: Set the required and optional frontmatter fields that carry a page's metadata, such as title, description, and pcx_content_type.
```

## Optional fields

For optional fields such as `sidebar`, `tags`, `products`, `difficulty`, and `reviewed`, refer to [Custom properties](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/custom-properties/).

For more information on the available fields, refer to [Nimbus's documentation ↗](https://nimbus-docs.com/writing/frontmatter/).

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/#page","headline":"Frontmatter · Cloudflare Style Guide","description":"Set the required and optional frontmatter fields that carry a page's metadata, such as title, description, and pcx\\_content\\_type.","url":"https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
