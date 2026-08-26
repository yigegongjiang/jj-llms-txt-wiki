---
description: Create navigation pages that signpost a docs area with an automatically generated directory listing of their child pages.
title: Navigation
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Navigation

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/navigation/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

A navigation page is a sub-landing page that points a reader deeper into a specific area of the documentation. It carries almost no prose of its own: a short introduction and an automatically generated listing of the child pages under it. The tone is brief and functional.

## When to use it

Write a navigation page when an area of the documentation has enough child pages that a reader needs a signposted entry point into them. It is not:

* **An overview.** An overview introduces a product and orients a new reader with prose, whereas a navigation page mainly signposts the child pages under it.
* **A concept.** A concept explains how something works, whereas a navigation page explains nothing and only routes the reader onward.

For the full comparison, refer to [Content types](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/).

## Title & description

* **Title**: name the section or area the page fronts, matching the heading a reader clicked to arrive.
* **Description**: invite the reader to explore the area, and name the key topics the child pages cover.

## Scaffold this page

Copy this skeleton and adapt it to your area:

```plaintext
---
title: <Section or area name>
description: Explore <area>, covering <the key topics the child pages cover>.
pcx_content_type: navigation
sidebar:
  order: 10
products:
  - product-a
---

import { DirectoryListing } from "~/components";

Introduce the area in one or two sentences, then let the listing route the reader to the child pages.

<DirectoryListing />
```

## Component guidance

* [**DirectoryListing**](https://developers.cloudflare.com/style-guide/build-the-page/components/directory-listing/) carries the body of the page: it displays the child pages of a folder as a list of links, generated automatically so the listing stays current as pages are added or removed.
* **What does not fit:** substantive explanation or procedures. A navigation page holds no content of its own, so put explanations on the pages it links to.

## Frontmatter

```yaml
pcx_content_type: navigation
products:
  - product-a
  - product-b
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## Examples

* [Logs: Enable destinations](https://developers.cloudflare.com/logs/logpush/logpush-job/enable-destinations/)
* [Cloudflare Tunnel: Get started](https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-tunnel/get-started/)

## Writing for AI and agents

* **Automatic listing.** Use DirectoryListing rather than a hand-written list, so the routes an agent follows are always the current child pages.
* **No orphaned content.** Keep explanations and procedures off the navigation page, because an agent that lands here should be routed onward, not asked to read.
* **Descriptive child titles.** The listing shows each child page's title, so write those titles to stand alone, because they are the only signal a reader or agent has when choosing where to go.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/navigation/#page","headline":"Navigation · Cloudflare Style Guide","description":"Create navigation pages that signpost a docs area with an automatically generated directory listing of their child pages.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/navigation/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
