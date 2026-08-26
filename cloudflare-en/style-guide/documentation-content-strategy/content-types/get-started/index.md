---
description: Write get-started pages that take a new user from nothing to a first working setup by the shortest honest path.
title: Get started
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Get started

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/get-started/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

A get-started page takes a new user from not using a product to a first working setup by the shortest honest path. The tone is instructional and encouraging.

## When to use it

Write a get-started page when a new user needs the shortest path from nothing to one working result, before they explore the product in depth. It is not:

* **A how-to.** A how-to completes one specific task for a reader who already uses the product, whereas a get-started page delivers a new user's first success end to end.
* **A tutorial.** A tutorial teaches through a longer guided project, whereas a get-started page stops at the first working result and routes the reader onward.

For the full comparison, refer to [Content types](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/).

## Title & description

* **Title**: the page title is Get started.
* **Description**: name the product, summarize the first setup the reader completes, and note the key prerequisites.

## Scaffold this page

Use the Nimbus get-started recipe to generate this page. Your coding agent pulls the full page skeleton and self-review checklist, then adapts them to your product:

npmyarnpnpm

```
npx @cloudflare/nimbus-docs add content-quickstart
```

```
yarn @cloudflare/nimbus-docs add content-quickstart
```

```
pnpm @cloudflare/nimbus-docs add content-quickstart
```

Adapt the frontmatter the recipe emits to Cloudflare's schema: set [pcx\_content\_type](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype) and `products` instead of the generic fields the recipe emits, such as `type`.

## Component guidance

* [**Prerequisites**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/prerequisites/) list what the reader needs before starting, such as an active zone, a subscription or plan, or setup outside Cloudflare.
* [**Steps**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/steps-tasks-procedures/) lead the reader to product adoption, covering the minimum setup plus the most general use case, and often reuse partials from the how-to pages.
* [**Links**](https://developers.cloudflare.com/style-guide/style-and-grammar/formatting/structure/links/) close the page with next steps that point the reader toward deeper configuration once they have a working setup.
* **What does not fit:** exhaustive configuration or edge cases. Keep those in how-tos and reference pages, and link to them.

## Frontmatter

```yaml
pcx_content_type: get-started
products:
  - product-a
  - product-b
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## Examples

* [Waiting Room: Get started](https://developers.cloudflare.com/waiting-room/get-started/)

## Writing for AI and agents

* **Shortest honest path.** Include only the steps that reach a first working result, because a get-started page is judged by how quickly a reader or agent reaches success, not by coverage.
* **Show the result.** State and show the working outcome the steps produce, so a reader or agent can verify success before moving on.
* **Complete prerequisites.** State exactly what the reader needs before starting, because an agent cannot begin a setup it is not equipped to reach.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/get-started/#page","headline":"Get started · Cloudflare Style Guide","description":"Write get-started pages that take a new user from nothing to a first working setup by the shortest honest path.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/get-started/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
