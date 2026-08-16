---
description: Write conceptual explanation documentation.
title: Concept
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Concept

Last updated Apr 24, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/concept/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

## Purpose

The purpose of a concept is to provide conceptual or descriptive information so users understand the background and context of a particular topic.

## Tone

instructional, descriptive, approachable, supportive

## content\_type

```yaml
pcx_content_type: concept
products:
  - product-a
  - product-b
  - product-c
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## Structure

### Required components

[**Title**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/titles/): Use "About" for concept pages that describe the functionality of your product. Otherwise, use a short noun phrase (feature name, functionality, Internet concept - Health checks, Status resource protection, CDN)

[**Context**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/context/): Provide a brief description of why users should care about this information.

**Explanation**: Explain the page topic. Keep paragraphs short and concise to avoid large blocks of text. Feel free to use bulleted lists, notes, and headings for visual breaks.

## Template

```plaintext

---
title: About (for high-level product concept page only - otherwise omit this line)
weight: xx
pcx_content_type: concept
description: <Product or feature> <what it does and why it matters>. <Key detail or scope>.
products:
  - product-a
  - product-b
  - product-c
---

# About <product> or noun phrase

Provide a brief description of why users should care about this information.

Explain the page topic. Keep paragraphs short and concise to avoid large blocks of text. Feel free to use bulleted lists, notes, and headings for visual breaks.
```

## Additional information

Do not recreate information that's already available online. Instead, consider why a topic needs to be explained, what Cloudflare's perspective is on that topic, and what users need to understand about the topic in order to successfully use our products.

## Examples

[Load Balancing](https://developers.cloudflare.com/load-balancing/)

[WAF](https://developers.cloudflare.com/waf/)

[Magic Transit](https://developers.cloudflare.com/magic-transit/about/)

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/concept/#page","headline":"Concept · Cloudflare Style Guide","description":"Write conceptual explanation documentation.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/concept/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-24","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
