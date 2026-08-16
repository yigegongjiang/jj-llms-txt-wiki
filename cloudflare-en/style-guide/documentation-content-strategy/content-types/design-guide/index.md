---
description: Write design guide documentation.
title: Design guide
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Design guide

Last updated Apr 24, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/design-guide/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

## Purpose

Help users understand how to plan and design a solution using Cloudflare. Typically [design guides](https://developers.cloudflare.com/reference-architecture/design-guides/) are a subset of a [reference architecture](https://developers.cloudflare.com/reference-architecture/).

Disambiguation

This page describes the content strategy for a design guide. For help with Cloudflare products, refer to [Design guides](https://developers.cloudflare.com/reference-architecture/design-guides/).

## Tone

instructional, straightforward

## content\_type

```yaml
pcx_content_type: design-guide
products:
  - product-a
  - product-b
  - product-c
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## Examples

[Securely deliver applications with Cloudflare](https://developers.cloudflare.com/reference-architecture/design-guides/secure-application-delivery/)

## Structure

### Required components

[**Title**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/titles/): Short verb phrase in second-person imperative. Do not use gerund phrases.

[**Introduction**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/introduction/): Two to three paragraphs describing the subject matter.

[**Intended audience**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/intended-audience/): Summary of who the content is aimed at and what users will learn.

### Optional components

[**Notes/warnings**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/notes-tips-warnings/)

[**Examples**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/examples/)

[**Diagrams**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/diagrams/)

**Screenshots**

[**Related links**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/links/): Bulleted list of links to associated resources.

## Template

```plaintext

---
title: "Example design guide"
pcx_content_type: design-guide
description: Plan and design <solution> using Cloudflare <products>. Covers architecture decisions, configuration steps, and deployment best practices.
products:
  - cloudflare-one
  - cloudflare-wan
  - cloudflare-network-firewall
weight:
meta:
  title: "Design guide: An example"
---

# Design guide title

## Introduction
Provide context to what this guide is going to cover. Ensure you describe the end state of the solution this guide will detail.

### Who is this for?
This reference architecture is designed for IT or security professionals with some responsibility over or familiarity with their organization’s existing infrastructure. It is useful to have some experience with technologies important to securing hybrid work, including identity providers (IdPs), user directories, single sign on (SSO), endpoint security or management (EPP, XDR, UEM, MDM), firewalls, routers, and point solutions like packet or content inspection hardware, threat prevention, and data loss prevention technologies.

## Heading 1
### Subheading 1

## Heading 2
### Subheading 2

## Heading 3
### Subheading 4

## Summary
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/design-guide/#page","headline":"Design guide · Cloudflare Style Guide","description":"Write design guide documentation.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/design-guide/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-24","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
