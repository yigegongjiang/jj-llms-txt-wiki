---
description: Display a single Cloudflare CLI command with details.
title: CfCommand
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# CfCommand

Last updated Sep 23, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/style-guide/build-the-page/components/cf-command/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

The `CfCommand` component is used `0` times on `0` pages.

<details>

<summary>

See all examples of pages that use CfCommand

</summary>

Used **0** times.

**Pages**

**Partials**

</details>

The `CfCommand` component documents one Cloudflare CLI command using metadata from the version of `cf` installed in the [`cloudflare-docs` repository ↗︎](https://github.com/cloudflare/cloudflare-docs/blob/production/package.json).

## Import

```mdx
import { CfCommand } from "~/components";
```

## Usage

```mdx
import { CfCommand } from "~/components";

<CfCommand command="deploy" />
<CfCommand command="auth whoami" headingLevel={3} />
```

## Arguments

`command` `string` required is the command without the `cf` prefix, such as `deploy` or `auth whoami`.

`headingLevel` `number` (default: 2) optional sets the heading level for the command name.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/build-the-page/components/cf-command/#page","headline":"CfCommand","description":"Display a single Cloudflare CLI command with details.","url":"https://developers.cloudflare.com/style-guide/build-the-page/components/cf-command/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-09-23","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
