---
description: Create a heading with a custom anchor ID.
title: Anchor heading
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Anchor heading

Last updated Jun 18, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/components/anchor-heading/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

The `AnchorHeading` component is used `29` times on `6` pages.

See all examples of pages that use AnchorHeading

Used **29** times.

**Pages**

**Partials**

* [src/content/partials/durable-objects/api-async-kv-legacy.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/durable-objects/api-async-kv-legacy.mdx)
* [src/content/partials/networking-services/analytics/overview.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/networking-services/analytics/overview.mdx)
* [src/content/partials/networking-services/reference/mtu-mss.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/networking-services/reference/mtu-mss.mdx)
* [src/content/partials/networking-services/reference/traffic-steering.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/networking-services/reference/traffic-steering.mdx)
* [src/content/partials/workers/wrangler-commands/containers.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/workers/wrangler-commands/containers.mdx)
* [src/content/partials/workers/wrangler-commands/tunnel.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/workers/wrangler-commands/tunnel.mdx)

The `AnchorHeading` component defines headings. Specifically, `AnchorHeading` performs the following:

1. Generates URL fragments corresponding to headings.
2. Formats URL fragments into compatible syntax. For example, a `&` is replaced with a `-`.
3. Creates a button to copy the URL at each fragment.
4. Allows heading fragments to be defined separately from the text of the heading itself.

```mdx
import { AnchorHeading } from "~/components";

<AnchorHeading title="How to use AnchorHeading" slug="use-anchorheading" depth={2} />
```

Markdown files (including partials) have this behavior by default, applied via rehype plugins. Therefore, the `AnchorHeading` component is usually only required when writing headings yourself inside components, or when working on non-markdown files.

To override the ID given to a heading within Markdown, add an MDX comment at the end of the line:

```mdx
## foo {/*bar*/}
{/* HTML: <h2 id="bar">foo</h2> */}
```

Note

The `AnchorHeading` component emulates the behavior of the [rehype-slug ↗](https://github.com/rehypejs/rehype-slug) and the [rehype-autolink-headings ↗](https://github.com/rehypejs/rehype-autolink-headings). It adds an `id` based on the output of [github-slugger ↗](https://github.com/Flet/github-slugger/) to the heading, as well as adding a button to copy a link to that particular heading.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/components/anchor-heading/#page","headline":"Anchor heading · Cloudflare Style Guide","description":"Create a heading with a custom anchor ID.","url":"https://developers.cloudflare.com/style-guide/components/anchor-heading/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-06-18","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
