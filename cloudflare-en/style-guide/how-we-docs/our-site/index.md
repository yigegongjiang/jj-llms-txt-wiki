---
description: Understand the documentation site architecture.
title: Our site
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Our site

Last updated Jul 22, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/how-we-docs/our-site/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

We use a variety of tools to make our docs site work. You could use these tools to build up your own docs site and - in most cases - do so for free or starting on a free tier.

## Content management system

Our content lives in a public GitHub repository, [cloudflare-docs ↗](https://github.com/cloudflare/cloudflare-docs).

GitHub offers a generous [free tier ↗](https://github.com/pricing).

## Search

We use [Algolia ↗](https://www.algolia.com/) as our search provider.

If you have open-source docs, you can be part of the free [DocSearch program ↗](https://docsearch.algolia.com/).

## Site framework

We use [Nimbus ↗](https://nimbus-docs.com/) for our docs, a documentation framework built on [Astro ↗](https://astro.build/).

Nimbus's component [registry ↗](https://nimbus-docs.com/registry/) and [linting ↗](https://nimbus-docs.com/writing/linting/) system have exponentially increased our [site's capabilities](https://developers.cloudflare.com/style-guide/components/) (without much extra work).

## Builds

We use [GitHub Actions ↗](https://github.com/features/actions) to build our site, which is then [hosted](#hosting) on Cloudflare.

We are moving to [Workers CI/CD](https://developers.cloudflare.com/workers/ci-cd/), which currently runs in the background.

Both of these options include a free tier.

## Hosting

We host our content using [Cloudflare Workers](https://developers.cloudflare.com/workers/static-assets/), specifically using their built in values for [Astro sites](https://developers.cloudflare.com/workers/framework-guides/web-apps/astro/)

Workers offers a generous [free tier](https://developers.cloudflare.com/workers/platform/pricing/).

## Analytics

We send analytics to multiple destinations using [Cloudflare Zaraz](https://developers.cloudflare.com/zaraz/), which has a generous [free tier](https://developers.cloudflare.com/zaraz/pricing-info/).

Note

If you want to opt out of analytics tracking, use the icon at the bottom of your screen.

![Opt out of analytics with the icon at the bottom of your screen](https://developers.cloudflare.com/_astro/privacy-opt-out.Cthj3AFl_hVM54.webp)

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.DMYpXs3t.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/how-we-docs/our-site/#page","headline":"Our site · Cloudflare Style Guide","description":"Understand the documentation site architecture.","url":"https://developers.cloudflare.com/style-guide/how-we-docs/our-site/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-07-22","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
