---
description: Display a single Wrangler command with details.
title: WranglerCommand
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# WranglerCommand

Last updated Sep 23, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/style-guide/build-the-page/components/wrangler-command/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

The `WranglerCommand` component is used `104` times on `8` pages.

<details>

<summary>

See all examples of pages that use WranglerCommand

</summary>

Used **104** times.

**Pages**

- <a href="https://developers.cloudflare.com/workers/wrangler/commands/certificates/">/workers/wrangler/commands/certificates/</a>-<a href="https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/workers/wrangler/commands/certificates.mdx">Source</a>
- <a href="https://developers.cloudflare.com/workers/wrangler/commands/general/">/workers/wrangler/commands/general/</a>-<a href="https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/workers/wrangler/commands/general.mdx">Source</a>
- <a href="https://developers.cloudflare.com/workers/wrangler/commands/secrets-store/">/workers/wrangler/commands/secrets-store/</a>-<a href="https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/workers/wrangler/commands/secrets-store.mdx">Source</a>
- <a href="https://developers.cloudflare.com/workers/wrangler/commands/workers-for-platforms/">/workers/wrangler/commands/workers-for-platforms/</a>-<a href="https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/workers/wrangler/commands/workers-for-platforms.mdx">Source</a>
- <a href="https://developers.cloudflare.com/workers/wrangler/commands/workers/">/workers/wrangler/commands/workers/</a>-<a href="https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/workers/wrangler/commands/workers.mdx">Source</a>

**Partials**

- <a href="https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/workers/wrangler-commands/kv.mdx">src/content/partials/workers/wrangler-commands/kv.mdx</a>
- <a href="https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/workers/wrangler-commands/r2-sql.mdx">src/content/partials/workers/wrangler-commands/r2-sql.mdx</a>
- <a href="https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/partials/workers/wrangler-commands/r2.mdx">src/content/partials/workers/wrangler-commands/r2.mdx</a>

</details>

The `WranglerCommand` component documents the available options for a given command.

This is generated using the Wrangler version in the [`cloudflare-docs` repository ↗︎](https://github.com/cloudflare/cloudflare-docs/blob/production/package.json).

## Import

```mdx
import { WranglerCommand } from "~/components";
```

## Usage

```mdx
import { WranglerCommand } from "~/components";

<WranglerCommand
	command="deploy"
	description={"Deploy a [Worker](/workers/)"}
/>

<WranglerCommand command="d1 execute" />

<WranglerCommand command="deploy" cfCommand="deploy" />

<WranglerCommand
	command="d1 execute"
	cfCommand={["d1 query", "d1 raw"]}
	includeHiddenCf
/>
```

## With ExtraFlagDetails

You can add or replace help text for specific flags using the `ExtraFlagDetails` component:

```mdx
import { WranglerCommand } from "~/components";
import ExtraFlagDetails from "~/components/cf/ExtraFlagDetails.astro";

<WranglerCommand command="deploy">
	<ExtraFlagDetails key="dry-run">
		Additional details about the dry-run flag that will be appended to the
		original help text. Here is a [link](https://cloudflare.com) for more
		information.
	</ExtraFlagDetails>
	<ExtraFlagDetails key="compatibility-date" mode="replace">
		Custom help text that completely replaces the original description for this
		flag.
	</ExtraFlagDetails>
</WranglerCommand>
```

## Arguments

- `command` `string` required
  - The name of the command, i.e `d1 execute`.
- `headingLevel` `number` (default: 2) optional
  - The heading level that the command name should be added at on the page, i.e `2` for a `h2`.
- `description` `string` optional
  - A description to render below the command heading. If not set, defaults to the value specified in the Wrangler help API.
- `cfCommand` `string | string[]` optional
  - One or more reviewed Cloudflare CLI equivalents without the `cf` prefix. Supplying this prop displays the shared CLI selector. Explain partial or non-equivalent workflows in the surrounding page content.
- `includeHiddenCf` `boolean` (default: false) optional
  - Allows explicitly reviewed `cfCommand` values that the installed Cloudflare CLI marks as hidden.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/build-the-page/components/wrangler-command/#page","headline":"WranglerCommand","description":"Display a single Wrangler command with details.","url":"https://developers.cloudflare.com/style-guide/build-the-page/components/wrangler-command/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-09-23","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
