---
description: Display Cloudflare CLI namespace documentation.
title: CfNamespace
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# CfNamespace

Last updated Sep 23, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/style-guide/build-the-page/components/cf-namespace/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

The `CfNamespace` component is used `0` times on `0` pages.

<details>

<summary>

See all examples of pages that use CfNamespace

</summary>

Used **0** times.

**Pages**

**Partials**

</details>

The `CfNamespace` component documents every visible command in a Cloudflare CLI namespace using metadata from the version of `cf` installed in the [`cloudflare-docs` repository ↗︎](https://github.com/cloudflare/cloudflare-docs/blob/production/package.json).

## Import

```mdx
import { CfNamespace } from "~/components";
```

## Usage

```mdx
import { CfNamespace } from "~/components";

<CfNamespace namespace="hyperdrive" />
```

## Arguments

`namespace` `string` required is the namespace without the `cf` prefix, such as `hyperdrive`.

`headingLevel` `number` (default: 2) optional sets the heading level for each command name.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/build-the-page/components/cf-namespace/#page","headline":"CfNamespace","description":"Display Cloudflare CLI namespace documentation.","url":"https://developers.cloudflare.com/style-guide/build-the-page/components/cf-namespace/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-09-23","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
