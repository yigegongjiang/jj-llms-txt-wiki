---
description: Required and optional frontmatter fields for Cloudflare docs pages, including title, description, pcx_content_type, sidebar, and tags.
title: Frontmatter
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Frontmatter

Last updated Jul 22, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/frontmatter/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

Frontmatter contains the metadata for a page, such as the `title`. It is written as YAML, between `---`, at the top of the page.

For example:

```yaml
---
title: Create a Cloudflare Tunnel
pcx_content_type: how-to
products:
  - cloudflare-tunnel
description: Create a Cloudflare Tunnel to securely connect your private network or application to Cloudflare without exposing a public IP address. Requires cloudflared and a Cloudflare account.
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
description: This page explains how to create a Cloudflare Tunnel.
```

```yaml
description: Create a Cloudflare Tunnel to securely connect your private network or application to Cloudflare without exposing a public IP address. Requires cloudflared and a Cloudflare account.
```

```yaml
description: Get started with Workers.
```

```yaml
description: Set up your first Cloudflare Worker by installing Wrangler, writing a Hello World script, and deploying it to the Cloudflare network.
```

## Optional fields

For optional fields such as `sidebar`, `tags`, `products`, `difficulty`, and `reviewed`, refer to [Custom properties](https://developers.cloudflare.com/style-guide/frontmatter/custom-properties/).

For more information on the available fields, refer to [Nimbus's documentation ↗](https://nimbus-docs.com/writing/frontmatter/).

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/frontmatter/#page","headline":"Frontmatter · Cloudflare Style Guide","description":"Required and optional frontmatter fields for Cloudflare docs pages, including title, description, pcx\\_content\\_type, sidebar, and tags.","url":"https://developers.cloudflare.com/style-guide/frontmatter/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-07-22","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
