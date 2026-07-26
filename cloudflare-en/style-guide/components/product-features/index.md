---
description: Display a product feature comparison table.
title: Product features
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Product features

Last updated Jun 18, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/components/product-features/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

The `ProductFeatures` component is used `3` times on `3` pages.

See all examples of pages that use ProductFeatures

Used **3** times.

**Pages**

* [/cache/plans/](https://developers.cloudflare.com/cache/plans/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/cache/plans.mdx)
* [/dns/reference/all-features/](https://developers.cloudflare.com/dns/reference/all-features/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/dns/reference/all-features.mdx)
* [/ssl/reference/all-features/](https://developers.cloudflare.com/ssl/reference/all-features/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/ssl/reference/all-features.mdx)

**Partials**

Use when you need to list all available features within a product grouping inside of the `index.json` file in `src/content/plans/`. For the id property, specify the product object you want to use.

```mdx
import { ProductFeatures } from "~/components"

<ProductFeatures id="dns" />
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.DMYpXs3t.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/components/product-features/#page","headline":"Product features · Cloudflare Style Guide","description":"Display a product feature comparison table.","url":"https://developers.cloudflare.com/style-guide/components/product-features/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-06-18","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
