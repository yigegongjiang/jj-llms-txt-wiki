---
description: Write changelog pages that record notable, dated product changes, the single type for release notes and other product updates.
title: Changelog
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Changelog

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/changelog/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

A changelog logs notable, dated changes to a product. The tone is instructional and straightforward.

This page covers how to write one. For the published updates themselves, refer to [Changelog](https://developers.cloudflare.com/changelog/).

## When to use it

Write a changelog when you need to record notable, dated changes to a product as an ongoing feed. It is not:

* **A blog post.** A changelog entry is a short, factual record of one change, whereas a blog post explains and promotes at length.
* **The documentation of a change.** An entry records that something changed and links out, whereas the how-to, reference, or concept is where the change is actually documented.

For the full comparison, refer to [Content types](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/).

## Title & description

* **Title**: the page title is Changelog.
* **Description**: name the product and what the changelog tracks, such as recent changes, new features, and bug fixes.

## Scaffold this page

Use the Nimbus changelog recipe to generate this page. Your coding agent pulls the full page skeleton and self-review checklist, then adapts them to your product:

npmyarnpnpm

```
npx @cloudflare/nimbus-docs add content-changelog
```

```
yarn @cloudflare/nimbus-docs add content-changelog
```

```
pnpm @cloudflare/nimbus-docs add content-changelog
```

Adapt the frontmatter the recipe emits to Cloudflare's schema: set [pcx\_content\_type](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype) and `products` instead of the generic fields the recipe emits, such as `type`.

## Component guidance

* [**ProductChangelog**](https://developers.cloudflare.com/style-guide/build-the-page/components/product-changelog/) renders a product's entries on the changelog page, pulling from the entries folder so the page stays current as entries are added.
* **Entries** are the body: each dated MDX file in the changelog collection is one notable change, with its own title, description, and date.
* **What does not fit:** long-form explanation or promotion. Keep an entry short and factual, and link to the how-to or concept that covers the change in depth.

## Frontmatter

```yaml
pcx_content_type: changelog
products:
  - product-a
  - product-b
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## Ownership

Product managers and engineers maintain changelogs manually or through an automated process that their team owns. PCX provides a review but does not own creating or writing changelogs.

## Building a changelog

A changelog needs an MDX page file and a corresponding folder of changelog entries. The combination of these files allows us to:

* Render traditional changelog content on an [HTML page](https://developers.cloudflare.com/dns/changelog/).
* Programmatically create an [RSS feed](https://developers.cloudflare.com/changelog/rss/dns.xml) with the changelog content.
* Pull all our changelog content into a [Cloudflare-wide changelog](https://developers.cloudflare.com/changelog/).

### Changelog page

The MDX page needs several special values to pull in the changelog information, highlighted in the sample page. For more information about the ProductChangelog component, refer to [ProductChangelog](https://developers.cloudflare.com/style-guide/build-the-page/components/product-changelog/).

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

Changelog entries live in a different location of our docs, [/src/content/changelog/ ↗](https://github.com/cloudflare/cloudflare-docs/tree/production/src/content/changelog). Each entry is its own MDX file, similar to the following.

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

### Entry properties

Each changelog entry has the following properties:

* `title` `string` required

  * Shown in the title heading and on social media embeds.
* `description` `string` required

  * Shown in social media embeds.
* `date` `date` required

  * This should be a date in `YYYY-MM-DD` format. For example, `2025-02-04`.
* `products` `Array<String>` (default: current location) required

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
* `hidden` `Boolean` (default: false) optional

  * If `true`, this page will be accessible from the direct link, but hidden from the main [changelog](https://developers.cloudflare.com/changelog/) page and all RSS feeds.
  * If `true`, will also add a `noindex` property so the page is not indexed by search crawlers.

## Writing for AI and agents

* **Dated, atomic entries.** Give each change its own dated entry with a title and description, because an agent extracts and orders changes by entry rather than by scanning prose.
* **Literal dates and products.** Use `YYYY-MM-DD` dates and exact product names in frontmatter, so a reader or agent can filter and sort the feed reliably.
* **Link out for depth.** Keep the entry to the change itself and link to the how-to or concept that explains it, because the changelog records what changed, not how to use it.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/changelog/#page","headline":"Changelog · Cloudflare Style Guide","description":"Write changelog pages that record notable, dated product changes, the single type for release notes and other product updates.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/changelog/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
