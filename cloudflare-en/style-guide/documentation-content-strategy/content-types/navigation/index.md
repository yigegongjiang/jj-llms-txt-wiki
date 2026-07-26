---
description: Create navigation pages with directory listings.
title: Navigation
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Navigation

Last updated Apr 24, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/navigation/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

## Purpose

The purpose of a navigation page is to direct users deeper into the doc set and act as a sub-landing page for a specific area of the docs.

## content\_type

```yaml
pcx_content_type: navigation
products:
  - product-a
  - product-b
  - product-c
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## Components

[DirectoryListing](https://developers.cloudflare.com/style-guide/components/directory-listing/): 

Use `<DirectoryListing />` to display the directory of a specific folder, which appears as a list of links.

## Template

```mdx
---
weight: xx
pcx_content_type: navigation
description: Explore <Cloudflare product> guides for <key topic areas covered by child pages>.
products:
  - product-a
  - product-b
  - product-c
---

import { DirectoryListing } from "~/components";

# Name of section

<DirectoryListing />
```

## Examples

[Logs: Enable destinations](https://developers.cloudflare.com/logs/logpush/logpush-job/enable-destinations/)

[Cloudflare Tunnel: Get Started](https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-tunnel/get-started/)

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.DMYpXs3t.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/navigation/#page","headline":"Navigation · Cloudflare Style Guide","description":"Create navigation pages with directory listings.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/navigation/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-24","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
