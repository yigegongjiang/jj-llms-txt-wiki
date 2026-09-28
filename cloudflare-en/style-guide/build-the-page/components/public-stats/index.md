---
description: Display public statistics from Cloudflare data.
title: Public stats
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Public stats

Last updated Aug 20, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/style-guide/build-the-page/components/public-stats/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

The `PublicStats` component is used `16` times on `8` pages.

<details>

<summary>

See all examples of pages that use PublicStats

</summary>

Used **16** times.

**Pages**

- <a href="https://developers.cloudflare.com/learning-paths/data-center-protection/concepts/benefits-magic-transit/">/learning-paths/data-center-protection/concepts/benefits-magic-transit/</a>-<a href="https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/learning-paths/data-center-protection/concepts/benefits-magic-transit.mdx">Source</a>
- <a href="https://developers.cloudflare.com/reference-architecture/architectures/cdn/">/reference-architecture/architectures/cdn/</a>-<a href="https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/reference-architecture/architectures/cdn.mdx">Source</a>
- <a href="https://developers.cloudflare.com/reference-architecture/architectures/load-balancing/">/reference-architecture/architectures/load-balancing/</a>-<a href="https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/reference-architecture/architectures/load-balancing.mdx">Source</a>
- <a href="https://developers.cloudflare.com/reference-architecture/architectures/sase/">/reference-architecture/architectures/sase/</a>-<a href="https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/reference-architecture/architectures/sase.mdx">Source</a>
- <a href="https://developers.cloudflare.com/reference-architecture/architectures/security/">/reference-architecture/architectures/security/</a>-<a href="https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/reference-architecture/architectures/security.mdx">Source</a>
- <a href="https://developers.cloudflare.com/reference-architecture/design-guides/securing-guest-wireless-networks/">/reference-architecture/design-guides/securing-guest-wireless-networks/</a>-<a href="https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/reference-architecture/design-guides/securing-guest-wireless-networks.mdx">Source</a>
- <a href="https://developers.cloudflare.com/reference-architecture/diagrams/sase/deploying-self-hosted-voip-services-for-hybrid-users/">/reference-architecture/diagrams/sase/deploying-self-hosted-voip-services-for-hybrid-users/</a>-<a href="https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/reference-architecture/diagrams/sase/deploying-self-hosted-VoIP-services-for-hybrid-users.mdx">Source</a>
- <a href="https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/introductions/">/style-guide/documentation-content-strategy/component-attributes/introductions/</a>-<a href="https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/style-guide/documentation-content-strategy/component-attributes/introductions.mdx">Source</a>

**Partials**

</details>

The `PublicStats` component allows you to reference specific values about Cloudflare's network without maintaining those values in multiple files.

Refer to the examples below for more information.

```mdx
import { PublicStats } from "~/components";

Cloudflare has data centers in <PublicStats id="data_center_cities" />.

Our network has <PublicStats id="total_bandwidth" />.

Cloudflare also has <PublicStats id="network_peers" />.
```

Note

If you need more stats or to update these stats, submit a pull request to update [PublicStats.astro ↗︎](https://github.com/cloudflare/cloudflare-docs/blob/production/src/components/PublicStats.astro)

## Associated content types

The `PublicStats` component is commonly used on the following type of pages:

- [Overview](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/overview/)
- [Reference Architecture](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/reference-architecture/)
- [Reference Architecture Diagrams](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/reference-architecture/#reference-architecture-diagrams)

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/build-the-page/components/public-stats/#page","headline":"Public stats","description":"Display public statistics from Cloudflare data.","url":"https://developers.cloudflare.com/style-guide/build-the-page/components/public-stats/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
