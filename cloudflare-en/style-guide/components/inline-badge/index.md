---
description: Display inline status badges like Beta or New.
title: Inline badge
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Inline badge

Last updated Jun 18, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/components/inline-badge/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

The `InlineBadge` component is used `11` times on `11` pages.

See all examples of pages that use InlineBadge

Used **11** times.

**Pages**

* [/agents/communication-channels/voice/](https://developers.cloudflare.com/agents/communication-channels/voice/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/agents/communication-channels/voice.mdx)
* [/agents/examples/browser-agent/](https://developers.cloudflare.com/agents/examples/browser-agent/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/agents/examples/browser-agent.mdx)
* [/agents/examples/voice-agent/](https://developers.cloudflare.com/agents/examples/voice-agent/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/agents/examples/voice-agent.mdx)
* [/agents/tools/browser/](https://developers.cloudflare.com/agents/tools/browser/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/agents/tools/browser.mdx)
* [/browser-run/features/session-recording/](https://developers.cloudflare.com/browser-run/features/session-recording/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/browser-run/features/session-recording.mdx)
* [/ddos-protection/about/attack-coverage/](https://developers.cloudflare.com/ddos-protection/about/attack-coverage/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/ddos-protection/about/attack-coverage.mdx)
* [/ssl/edge-certificates/geokey-manager/setup/](https://developers.cloudflare.com/ssl/edge-certificates/geokey-manager/setup/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/ssl/edge-certificates/geokey-manager/setup.mdx)
* [/stream/stream-live/start-stream-live/](https://developers.cloudflare.com/stream/stream-live/start-stream-live/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/stream/stream-live/start-stream-live.mdx)
* [/stream/viewing-videos/using-the-stream-player/](https://developers.cloudflare.com/stream/viewing-videos/using-the-stream-player/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/stream/viewing-videos/using-the-stream-player/index.mdx)
* [/workers/wrangler/commands/artifacts/](https://developers.cloudflare.com/workers/wrangler/commands/artifacts/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/workers/wrangler/commands/artifacts.mdx)
* [/workers/wrangler/configuration/](https://developers.cloudflare.com/workers/wrangler/configuration/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/workers/wrangler/configuration.mdx)

**Partials**

Recommendation: Avoid inline badges

Our current recommendation is to avoid inline badges, since they may hurt readability.

Guidelines:

* Mention beta/alpha/early access in the feature's main documentation page (use the [<Badge>](https://developers.cloudflare.com/style-guide/components/badges/) component for this purpose).
* If an additional reference is needed in the middle of the text, use "(beta)", with no special formatting, after the feature name.
* For instructions related to the feature (such as instructions on turning the feature on or off), you may mention again it's in beta, and also include "(beta)" in the side nav.

## Component

To adopt this styling in a React component, apply the `sl-badge` class to a `span` element.

```mdx
import { InlineBadge } from '~/components';

### Alpha <InlineBadge preset="alpha" />

### Beta <InlineBadge preset="beta" />

### Deprecated <InlineBadge preset="deprecated" />

### Early Access <InlineBadge preset="early-access" />

### Legacy <InlineBadge preset="legacy" />

### Default <InlineBadge text="Default" />
```

## Inputs

Either `preset` or `text` and `variant` must be specified.

### Presets

* `alpha`

  * **Text**: `Alpha`
  * **Variant** `success`
* `beta`

  * **Text**: `Beta`
  * **Variant** `caution`
* `deprecated`

  * **Text**: `Deprecated`
  * **Variant** `danger`
* `early-access`

  * **Text**: `Early Access`
  * **Variant** `note`
* `legacy`

  * **Text**: `Legacy`
  * **Variant** `danger`

### Text

Any string.

### Variant

* `note`

  * **Color**: Blue
* `tip`

  * **Color**: Purple
* `danger`

  * **Color**: Red
* `caution`

  * **Color**: Orange
* `success`

  * **Color**: Green

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.DMYpXs3t.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/components/inline-badge/#page","headline":"Inline badge · Cloudflare Style Guide","description":"Display inline status badges like Beta or New.","url":"https://developers.cloudflare.com/style-guide/components/inline-badge/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-06-18","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
