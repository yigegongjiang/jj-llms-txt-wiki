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

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/build-the-page/components/anchor-heading/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

The `AnchorHeading` component is used `93` times on `19` pages.

See all examples of pages that use AnchorHeading

Used **93** times.

**Pages**

* [/cloudflare-one/networks/connectors/cloudflare-wan/analytics/](https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-wan/analytics/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/cloudflare-one/networks/connectors/cloudflare-wan/analytics/index.mdx)
* [/cloudflare-one/networks/connectors/cloudflare-wan/configuration/appliance/network-options/application-based-policies/breakout-traffic/](https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-wan/configuration/appliance/network-options/application-based-policies/breakout-traffic/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/cloudflare-one/networks/connectors/cloudflare-wan/configuration/appliance/network-options/application-based-policies/breakout-traffic.mdx)
* [/cloudflare-one/networks/connectors/cloudflare-wan/reference/traffic-steering/](https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-wan/reference/traffic-steering/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/cloudflare-one/networks/connectors/cloudflare-wan/reference/traffic-steering.mdx)
* [/cloudflare-wan/analytics/](https://developers.cloudflare.com/cloudflare-wan/analytics/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/cloudflare-wan/analytics/index.mdx)
* [/cloudflare-wan/configuration/appliance/network-options/application-based-policies/breakout-traffic/](https://developers.cloudflare.com/cloudflare-wan/configuration/appliance/network-options/application-based-policies/breakout-traffic/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/cloudflare-wan/configuration/appliance/network-options/application-based-policies/breakout-traffic.mdx)
* [/cloudflare-wan/](https://developers.cloudflare.com/cloudflare-wan/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/cloudflare-wan/index.mdx)
* [/cloudflare-wan/reference/traffic-steering/](https://developers.cloudflare.com/cloudflare-wan/reference/traffic-steering/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/cloudflare-wan/reference/traffic-steering.mdx)
* [/fundamentals/api/reference/limits/](https://developers.cloudflare.com/fundamentals/api/reference/limits/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/fundamentals/api/reference/limits.mdx)
* [/learning-paths/data-center-protection/get-started/](https://developers.cloudflare.com/learning-paths/data-center-protection/get-started/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/learning-paths/data-center-protection/get-started.mdx)
* [/magic-transit/analytics/](https://developers.cloudflare.com/magic-transit/analytics/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/magic-transit/analytics/index.mdx)
* [/magic-transit/get-started/](https://developers.cloudflare.com/magic-transit/get-started/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/magic-transit/get-started.mdx)
* [/magic-transit/reference/gre-ipsec-tunnels/](https://developers.cloudflare.com/magic-transit/reference/gre-ipsec-tunnels/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/magic-transit/reference/gre-ipsec-tunnels.mdx)
* [/magic-transit/reference/mtu-mss/](https://developers.cloudflare.com/magic-transit/reference/mtu-mss/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/magic-transit/reference/mtu-mss.mdx)
* [/magic-transit/reference/traffic-steering/](https://developers.cloudflare.com/magic-transit/reference/traffic-steering/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/magic-transit/reference/traffic-steering.mdx)
* [/magic-transit/troubleshooting/routing-and-bgp/](https://developers.cloudflare.com/magic-transit/troubleshooting/routing-and-bgp/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/magic-transit/troubleshooting/routing-and-bgp.mdx)

**Partials**

* [src/content/partials/durable-objects/api-async-kv-legacy.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/durable-objects/api-async-kv-legacy.mdx)
* [src/content/partials/durable-objects/durable-objects-pricing.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/durable-objects/durable-objects-pricing.mdx)
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
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/build-the-page/components/anchor-heading/#page","headline":"Anchor heading · Cloudflare Style Guide","description":"Create a heading with a custom anchor ID.","url":"https://developers.cloudflare.com/style-guide/build-the-page/components/anchor-heading/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
