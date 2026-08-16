---
description: Display formatted curl command examples.
title: CURL
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# CURL

Last updated Aug 3, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/components/curl/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

The `CURL` component is used `46` times on `15` pages.

See all examples of pages that use CURL

Used **46** times.

**Pages**

* [/ai-gateway/configuration/custom-domains/](https://developers.cloudflare.com/ai-gateway/configuration/custom-domains/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/ai-gateway/configuration/custom-domains.mdx)
* [/browser-run/cdp/session-management/](https://developers.cloudflare.com/browser-run/cdp/session-management/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/browser-run/cdp/session-management.mdx)
* [/browser-run/features/live-view/](https://developers.cloudflare.com/browser-run/features/live-view/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/browser-run/features/live-view.mdx)
* [/cloudflare-one/access-controls/ai-controls/mcp-portals/](https://developers.cloudflare.com/cloudflare-one/access-controls/ai-controls/mcp-portals/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/cloudflare-one/access-controls/ai-controls/mcp-portals.mdx)
* [/cloudflare-one/integrations/identity-providers/idp-federation/](https://developers.cloudflare.com/cloudflare-one/integrations/identity-providers/idp-federation/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/cloudflare-one/integrations/identity-providers/idp-federation.mdx)
* [/cloudflare-one/networks/routes/configure-initial-resolved-ips/](https://developers.cloudflare.com/cloudflare-one/networks/routes/configure-initial-resolved-ips/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/cloudflare-one/networks/routes/configure-initial-resolved-ips.mdx)
* [/data-localization/regional-services/ip-bindings/](https://developers.cloudflare.com/data-localization/regional-services/ip-bindings/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/data-localization/regional-services/ip-bindings.mdx)
* [/magic-transit/network-health/run-endpoint-health-checks/](https://developers.cloudflare.com/magic-transit/network-health/run-endpoint-health-checks/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/magic-transit/network-health/run-endpoint-health-checks.mdx)
* [/speed/optimization/content/shared-dictionaries/](https://developers.cloudflare.com/speed/optimization/content/shared-dictionaries/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/speed/optimization/content/shared-dictionaries.mdx)

**Partials**

* [src/content/partials/networking-services/cloudflare-wan/custom-ike-id-ipsec.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/networking-services/cloudflare-wan/custom-ike-id-ipsec.mdx)
* [src/content/partials/networking-services/mconn/network-options/app-aware-policies/breakout-prioritized.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/networking-services/mconn/network-options/app-aware-policies/breakout-prioritized.mdx)
* [src/content/partials/networking-services/mconn/network-options/app-aware-policies/netflow.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/networking-services/mconn/network-options/app-aware-policies/netflow.mdx)
* [src/content/partials/networking-services/routing/configure-tunnels.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/networking-services/routing/configure-tunnels.mdx)
* [src/content/partials/networking-services/tunnel-health/update-tunnel-health-checks-frequency.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/networking-services/tunnel-health/update-tunnel-health-checks-frequency.mdx)

The `CURL` component is used to display a cURL command for making HTTP requests.

## Import

```mdx
import { CURL } from "~/components";
```

## Usage

```mdx
import { CURL } from "~/components";

<CURL
	url="https://httpbin.org/anything"
	method="POST"
	json={{
		key: "va'l'ue",
	}}
	query={{
		foo: "bar",
		bar: ["baz", "qux"],
	}}
	code={{
		mark: "value",
	}}
/>

<CURL
	url="https://httpbin.org/anything"
	method="POST"
	form={{
		key: "value",
	}}
	code={{
		mark: "value",
	}}
/>
```

## `<CURL>` Props

### `url`

**required**

**type:** `string`

The URL to make the request to.

### `method`

**type:** `"GET" | "HEAD" | "POST" | "PUT" | "DELETE" | "OPTIONS" | "PATCH"`

**default:** `"GET"`

The HTTP method to use for the request.

### `headers`

**type:** `Record<string, string>`

The headers to include in the request.

### `json`

**type:** `Record<string, any> | Record<string, any>[]`

JSON data to include in the request.

### `form`

**type:** `Record<string, any>`

The FormData payload to send.

### `query`

**type:** `Record<string, string | string[]>`

URL query parameters to append to the request URL.

### `code`

**type:** `object`

An object of Astro `Code` props. Refer to the [Astro Code component documentation ↗](https://docs.astro.build/en/reference/api-reference/#code-) for available props.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/components/curl/#page","headline":"CURL · Cloudflare Style Guide","description":"Display formatted curl command examples.","url":"https://developers.cloudflare.com/style-guide/components/curl/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-03","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
