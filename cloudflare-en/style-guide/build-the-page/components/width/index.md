---
description: Constrain content width for layout control.
title: Width
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Width

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/build-the-page/components/width/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

The `Width` component is used `8` times on `4` pages.

See all examples of pages that use Width

Used **8** times.

**Pages**

* [/cloudflare-one/team-and-resources/devices/cloudflare-one-client/configure/modes/device-information-only/](https://developers.cloudflare.com/cloudflare-one/team-and-resources/devices/cloudflare-one-client/configure/modes/device-information-only/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/cloudflare-one/team-and-resources/devices/cloudflare-one-client/configure/modes/device-information-only.mdx)
* [/magic-transit/network-flow/](https://developers.cloudflare.com/magic-transit/network-flow/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/magic-transit/network-flow.mdx)

**Partials**

* [src/content/partials/networking-services/mnm-magic-transit-integration.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/networking-services/mnm-magic-transit-integration.mdx)
* [src/content/partials/style-guide/llms-txt.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/style-guide/llms-txt.mdx)

This component can be used to constrain the width of content, such as text or images.

## Import

```mdx
import { Width } from "~/components";
```

## Usage

```mdx
import { Width } from "~/components";

<Width size="large">This content will take up 75% of the container width</Width>

<Width size="medium">
	This content will take up 50% of the container width
</Width>

<Width size="small">This content will take up 25% of the container width</Width>

<Width size="small" center>
	This content will take up 25% of the container width and be centered
</Width>
```

## `<Width>` Props

### `size`

**required**

**type:** `"large" | "medium" | "small"`

Controls the width of the container:

* `large`: 75% of container width
* `medium`: 50% of container width
* `small`: 25% of container width

### `center`

**type:** `boolean`

Whether to horizontally center the content.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/build-the-page/components/width/#page","headline":"Width · Cloudflare Style Guide","description":"Constrain content width for layout control.","url":"https://developers.cloudflare.com/style-guide/build-the-page/components/width/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
