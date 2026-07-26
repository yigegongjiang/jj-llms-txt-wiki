---
description: Write changelog entries for product updates.
title: Changelog
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Changelog

Last updated Apr 24, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/changelog/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

## Purpose

The purpose of a changelog is to log or record notable changes, which then appear as part of the [Cloudflare changelog](https://developers.cloudflare.com/changelog/) and on product-specific changelog pages.

Disambiguation

This page describes the content strategy for changelogs. For updates on Cloudflare products, refer to [Changelog](https://developers.cloudflare.com/changelog/).

## Tone

instructional, straightforward

## content\_type

```yaml
pcx_content_type: changelog
products:
  - product-a
  - product-b
  - product-c
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## Ownership

Product managers and engineers maintain changelogs manually or via an automated process that their team owns. PCX provides a review but does not own creating or writing changelogs.

## Structure

When creating a changelog, you need an MDX page file and a corresponding folder of changelog entries.

The combination of these files allows us to:

* Render traditional changelog content on an [HTML page](https://developers.cloudflare.com/dns/changelog/).
* Programmatically create an [RSS feed](https://developers.cloudflare.com/changelog/rss/dns.xml) with the changelog content.
* Pull all our changelog content into a [Cloudflare-wide changelog](https://developers.cloudflare.com/changelog/).

### Markdown file

Your Markdown file needs to have several special values to pull in the changelog information. These values are highlighted in the sample page.

For more information about the `ProductChangelog` component, refer to the [style guide](https://developers.cloudflare.com/style-guide/components/product-changelog/).

```mdx
---
pcx_content_type: changelog
products:
  - dns
title: Changelog
description: Track recent changes, new features, and bug fixes for Cloudflare DNS.
---

import { ProductChangelog } from "~/components";

{/* <!-- Actual content lives in /src/content/changelog/dns/. --> */}

<ProductChangelog product="dns" />
```

### Changelog entries

Changelog entries live in a different location of our docs, [/src/content/changelog/ ↗](https://github.com/cloudflare/cloudflare-docs/tree/production/src/content/changelog).

Each entry will be its own MDX file, similar to the following.

```mdx
---
title: Account-level DNS analytics now available via GraphQL Analytics API
description: Authoritative DNS analytics can now be accessed on the account level via the GraphQL Analytics API.
products:
  - dns
date: 2025-06-19
---

Authoritative DNS analytics are now available on the **account level** via the [Cloudflare GraphQL Analytics API](/analytics/graphql-api/).

This allows users to query DNS analytics across multiple zones in their account, by using the `accounts` filter.

Here is an example to retrieve all DNS queries across all zones in an account that resulted in an `NXDOMAIN` response over a given time frame. Please replace `a30f822fcd7c401984bf85d8f2a5111c` with your actual account ID.

```graphql graphql-api-explorer title="GraphQL example for account-level DNS analytics"
query Viewer {
	viewer {
		accounts(filter: { accountTag: "a30f822fcd7c401984bf85d8f2a5111c" }) {
			dnsAnalyticsAdaptive(
				limit: 10
				filter: {
					date_geq: "2025-06-16"
					responseCode: "NXDOMAIN"
					date_leq: "2025-06-18"
				}
				orderBy: [datetime_DESC]
			) {
				zoneTag
				queryName
				responseCode
				queryType
				datetime
			}
		}
	}
}
```

To learn more and get started, refer to the [DNS Analytics documentation](/dns/additional-options/analytics/#analytics).
```

### Properties

Each changelog entries has the following properties:

* `title` `string`required

  * Shown in the title heading and on social media embeds.
* `description` `string`required

  * Shown in social media embeds.
* `date` `date`required

  * This should be a date in `YYYY-MM-DD` format. For example, `2025-02-04`.
* `products` `Array<String>`(default: current location) required

  * The products list is case-sensitive. Only use lowercase.
  * This should be an array of strings, each referring to the name of a file in the products collection without the file extension.
  * The folder that your entry is in, such as `src/content/changelog/workers/2025-02-13-new-product-feature.mdx`, is inferred as part of this property. If you do not want to associate the entry with additional products, you can omit it from the frontmatter entirely.
  * If you wish to reference a product that does not exist in this collection, such as one that resides in the subpath of an existing product, you can create a "metadata only" entry:  
  ```yaml  
  name: Workers Observability  
  product:  
  	title: Workers Observability  
  	url: /workers/observability/  
  	group: Developer platform  
  	show: false  
  ```
* `hidden` `Boolean`(default: false) optional

  * If `true`, this page will be accessible from the direct link, but hidden from the main [changelog](https://developers.cloudflare.com/changelog/) page and all RSS feeds.
  * If `true`, will also add a `noindex` property so the page is not indexed by search crawlers.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.DMYpXs3t.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/changelog/#page","headline":"Changelog · Cloudflare Style Guide","description":"Write changelog entries for product updates.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/changelog/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-24","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
