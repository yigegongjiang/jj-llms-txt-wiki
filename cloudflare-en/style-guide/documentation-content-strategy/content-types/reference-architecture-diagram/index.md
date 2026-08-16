---
description: Create reference architecture diagram pages.
title: Reference architecture diagram
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Reference architecture diagram

Last updated Apr 24, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/reference-architecture-diagram/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

## Purpose

To provide a visual reference and explanation of using Cloudflare for a specific solution.

Disambiguation

This page describes the content strategy for a reference architecture diagram. For help with Cloudflare products, refer to [Reference architectures diagrams](https://developers.cloudflare.com/reference-architecture/diagrams/).

## Tone

instructional, straightforward

## content\_type

```yaml
pcx_content_type: reference-architecture-diagram
products:
  - product-a
  - product-b
  - product-c
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## Components

### Most used

* [PublicStats](https://developers.cloudflare.com/style-guide/components/public-stats/):  
The `PublicStats` component allows you to reference specific values about Cloudflare's network without maintaining those values in multiple files.
* [Diagrams](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/diagrams/): Particularly helpful for image captions.

### Required

[**Title**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/titles/): Short verb phrase in second-person imperative. Do not use gerund phrases.

[**Reference diagram**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/reference-diagram/): A single diagram that reflects the overall reference architecture.

### Optional

[**Notes/warnings**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/notes-tips-warnings/)

**Screenshots**

[**Related links**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/links/): Bulleted list of links to associated resources.

## Template

```plaintext

---
pcx_content_type: reference-architecture-diagram
title: Cloudflare Reference Architecture Diagram
description: Architecture diagram showing how to <use case> with <Cloudflare products>. Includes component callouts and links to supporting documentation.
---

# Cloudflare Reference Architecture Diagram
---
title: Cloudflare Reference Architecture Diagram
pcx_content_type: reference-architecture-diagram
products:
  - cloudflare-one
  - cloudflare-wan
  - cloudflare-network-firewall
weight: 1
meta:
    title: "Reference Architecture Diagram: An example Cloudflare solution"
---

## Cloudflare Reference Architecture Diagram

Provide a description as to what the diagram below contains.

![Example reference architecture diagram](/images/reference-architecture/cloudflare-one-reference-architecture-images/cf1-ref-arch-14.svg "The above is an example reference architecture diagram")

1. Call out
2. Any numbered items
3. In the diagram
4. To explain their meaning/use

Provide some context to the diagram. What it relates to and link to any supporting content.
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/reference-architecture-diagram/#page","headline":"Reference architecture diagram · Cloudflare Style Guide","description":"Create reference architecture diagram pages.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/reference-architecture-diagram/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-24","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
