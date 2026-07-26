---
description: Write get-started guides for new users.
title: Get started
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Get started

Last updated Apr 24, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/get-started/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

## Purpose

The purpose of Get started content is to help users go from not using a product to successfully configuring and setting up.

## content\_type

```yaml
pcx_content_type: get-started
products:
  - product-a
  - product-b
  - product-c
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## Structure

### Required components

[**Title**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/titles/): Should be "Get started"

[**Prerequisites**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/prerequisites/): Which may include:

* An active zone
* Certain subscription / enabled product / plan
* Other tasks you might need to do to set up other things (your origin) outside of CF
* Do you need to make certain decisions before you start?

[**Steps**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/steps-tasks-procedures/): Steps that lead someone to whatever would be considered Product Adoption.

* Often, these can be partialized files from your How-to pages.
* This is usually the bare minimum (a single Bot Management FW rule) + the most general use case for a product.
* This may at times contradict the flow in the Cloudflare dashboard at times. If it does, consider raising it up to the Product team.

### Optional components

**Next steps**: Point someone towards additional configuration options.

## Template

```plaintext

---
weight: xx
pcx_content_type: get-started
description: Set up <Cloudflare product> for the first time by <key steps summarized>. Requires <key prerequisites>.
products:
  - product-a
  - product-b
  - product-c
---

# Get started

Description

## Before you begin

All the things you need to do before you start configuring your product, both within Cloudflare and outside.

## 1. Step description

## 2. Steps until you get to activation

---

## Next steps

Point to more complex setup options.
```

## Example

[Waiting Room: Get started](https://developers.cloudflare.com/waiting-room/get-started/)

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.DMYpXs3t.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/get-started/#page","headline":"Get started · Cloudflare Style Guide","description":"Write get-started guides for new users.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/get-started/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-24","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
