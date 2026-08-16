---
description: Browse available product and UI icons.
title: Icons
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Icons

Last updated Aug 11, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/components/icons/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

There are two icon components which pull from two different icon sets.

## Icon

The `Icon` component from Nimbus is available as a standalone component.

Primarily, this is used for Cloudflare product icons which are stored in `/src/icons/*.svg`.

```mdx
import { Icon } from "~/components";

<Icon name="workers" class="text-5xl text-orange-400" />
```

## Card and LinkCard

Components like `Card` and `LinkCard` accept a plain `icon` string prop — any [iconify ↗](https://icon-sets.iconify.design/) icon name.

```mdx
import { Card, LinkCard } from "~/components";

<Card title="Example" icon="ph:rocket-launch" />
<LinkCard title="Example" href="/workers/" icon="ph:rocket-launch" />
```

Content authored before the Nimbus migration may still use Starlight-style icon names (for example `icon="ph:terminal-window"`). These are automatically mapped to an equivalent iconify icon at build time. New content should use iconify names directly.

## Icon library

Optionally, you can choose a corresponding icon from Starlight’s [Icons ↗](https://starlight.astro.build/reference/icons/#all-icons) for cards or tabs.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/components/icons/#page","headline":"Icons · Cloudflare Style Guide","description":"Browse available product and UI icons.","url":"https://developers.cloudflare.com/style-guide/components/icons/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-11","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
