---
description: Render Markdown content inside JSX contexts.
title: Markdown
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Markdown

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/build-the-page/components/markdown/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

The `Markdown` component is used `27` times on `16` pages.

See all examples of pages that use Markdown

Used **27** times.

**Pages**

* [/cloudflare-one/networks/connectors/cloudflare-wan/legal/3rdparty/](https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-wan/legal/3rdparty/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/cloudflare-one/networks/connectors/cloudflare-wan/legal/3rdparty.mdx)
* [/cloudflare-wan/legal/3rdparty/](https://developers.cloudflare.com/cloudflare-wan/legal/3rdparty/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/cloudflare-wan/legal/3rdparty.mdx)
* [/waf/managed-rules/reference/cloudflare-managed-ruleset/](https://developers.cloudflare.com/waf/managed-rules/reference/cloudflare-managed-ruleset/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/waf/managed-rules/reference/cloudflare-managed-ruleset.mdx)

**Partials**

* [src/content/partials/cloudflare-one/gateway/add-block-page.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/cloudflare-one/gateway/add-block-page.mdx)
* [src/content/partials/cloudflare-one/gateway/client-notifications.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/cloudflare-one/gateway/client-notifications.mdx)
* [src/content/partials/cloudflare-one/gateway/inspect-on-all-ports.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/cloudflare-one/gateway/inspect-on-all-ports.mdx)
* [src/content/partials/cloudflare-one/gateway/logical-operators.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/cloudflare-one/gateway/logical-operators.mdx)
* [src/content/partials/cloudflare-one/ssh/upload-ssh-key.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/cloudflare-one/ssh/upload-ssh-key.mdx)
* [src/content/partials/cloudflare-one/tunnel/warp-to-tunnel-route-ips.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/cloudflare-one/tunnel/warp-to-tunnel-route-ips.mdx)
* [src/content/partials/networking-services/tunnel-health/troubleshoot-tunnel-health.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/networking-services/tunnel-health/troubleshoot-tunnel-health.mdx)
* [src/content/partials/ssl/aop-configure-origin.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/ssl/aop-configure-origin.mdx)
* [src/content/partials/waf/api-generic-create-rule-procedure.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/waf/api-generic-create-rule-procedure.mdx)
* [src/content/partials/waf/rulesets/api-account/step2-create-rule.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/waf/rulesets/api-account/step2-create-rule.mdx)
* [src/content/partials/waf/rulesets/api-account/step3-create-ruleset.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/waf/rulesets/api-account/step3-create-ruleset.mdx)
* [src/content/partials/waf/rulesets/api-zone/step2-create-rule.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/waf/rulesets/api-zone/step2-create-rule.mdx)
* [src/content/partials/waf/rulesets/api-zone/step3-create-ruleset.mdx](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/waf/rulesets/api-zone/step3-create-ruleset.mdx)

This component uses [marked ↗](https://marked.js.org/) to render [CommonMark and various other Markdown flavours ↗](https://marked.js.org/#specifications).

Caution

This component can not use [MDX ↗](https://mdxjs.com/) or [Astro ↗](https://docs.astro.build/en/guides/markdown-content/) features, such as [optimised images in the assets directory ↗](https://docs.astro.build/en/guides/images/#images-in-mdx-files).

Headings should not be used with this component, as they will not receive an `id`, copyable link or appear in the table of contents.

Code blocks should not be used with this component, as they will not receive syntax highlighting or a copy to clipboard button.

```mdx
import { Markdown } from "~/components";

<Markdown text="**foo** <br/> [bar](/style-guide/build-the-page/components/markdown/)" />
```

## Example for variables in partials

If you have a variable that needs to be formatted in any special way (for example, it needs to be a URL, an unordered list, or something else), you can wrap the variable with the markdown component in your partial file. For example:

```mdx
<Markdown text={props.foo} />
```

Note that you need to wrap your variable in curly braces, as well as use `text=` or this will not work.

## Multi-line strings

The Markdown component uses the [dedent ↗](https://www.npmjs.com/package/dedent) library to remove indentation from multi-line strings.

This is because the [CommonMark spec ↗](https://spec.commonmark.org/0.22/#indented-code-blocks) treats indented text as code blocks, unlike [MDX ↗](https://mdxjs.com/docs/what-is-mdx/#:~:text=Indented%20code%20does%20not%20work%20in%20MDX%3A).

```mdx
import { Markdown } from "~/components";

<>
  <Markdown
  	text={`
    You need to purchase [Cloudflare WAN](https://www.cloudflare.com/magic-wan/) before you can purchase and use the Cloudflare One Appliance. The Cloudflare One Appliance can function as your primary edge device for your network, or be deployed in-line with existing network gear.

  	You also need to purchase a Cloudflare One Appliance before you can start configuring your settings in the Cloudflare dashboard. After buying a Cloudflare One Appliance, the device will be registered with your Cloudflare account and show up in your Cloudflare dashboard.

    Contact your account representative to learn more about purchasing options for the Cloudflare One Appliance device.
    `}
    inline={false}
  />
</>
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/build-the-page/components/markdown/#page","headline":"Markdown · Cloudflare Style Guide","description":"Render Markdown content inside JSX contexts.","url":"https://developers.cloudflare.com/style-guide/build-the-page/components/markdown/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
