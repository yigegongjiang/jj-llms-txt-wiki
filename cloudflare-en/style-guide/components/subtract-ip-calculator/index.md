---
description: Interactive IP subtraction calculator component.
title: Subtract IP calculator
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Subtract IP calculator

Last updated Aug 3, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/components/subtract-ip-calculator/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

The `SubtractIPCalculator` component is used `6` times on `5` pages.

See all examples of pages that use SubtractIPCalculator

Used **6** times.

**Pages**

* [/cloudflare-one/networks/routes/reserved-ips/](https://developers.cloudflare.com/cloudflare-one/networks/routes/reserved-ips/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/cloudflare-one/networks/routes/reserved-ips.mdx)
* [/style-guide/components/subtract-ip-calculator/](https://developers.cloudflare.com/style-guide/components/subtract-ip-calculator/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/style-guide/components/subtract-ip-calculator.mdx)

**Partials**

* [src/content/partials/cloudflare-one/tunnel/deployment-guides/cloud-private-ip.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/cloudflare-one/tunnel/deployment-guides/cloud-private-ip.mdx)
* [src/content/partials/cloudflare-one/tunnel/warp-to-tunnel-route-ips.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/cloudflare-one/tunnel/warp-to-tunnel-route-ips.mdx)
* [src/content/partials/cloudflare-one/warp/add-split-tunnels-route.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/cloudflare-one/warp/add-split-tunnels-route.mdx)

## Import

```mdx
import { SubtractIPCalculator } from "~/components";
```

## Usage

  
Base CIDRSubtracted CIDRs

Calculate

```mdx
import { SubtractIPCalculator } from "~/components";

<SubtractIPCalculator client:load />
```

## `<SubtractIPCalculator>` Props

### `defaults`

**type:** `object`

An optional object containing `base` (`string`) and `subtract` (`string[]`) properties, to set default inputs.

**example:**

Base CIDRSubtracted CIDRs

Calculate

```mdx
import { SubtractIPCalculator } from "~/components";

<SubtractIPCalculator
	client:load
	defaults={{
		base: "10.0.0.0/8",
		subtract: ["10.0.0.0/24", "10.32.0.0/11"]
	}}
/>
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/components/subtract-ip-calculator/#page","headline":"Subtract IP calculator · Cloudflare Style Guide","description":"Interactive IP subtraction calculator component.","url":"https://developers.cloudflare.com/style-guide/components/subtract-ip-calculator/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-03","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
