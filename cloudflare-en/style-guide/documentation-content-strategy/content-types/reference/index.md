---
description: Write reference documentation for APIs and configs.
title: Reference
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Reference

Last updated Apr 24, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/reference/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

## Purpose

The purpose of reference content is to provide supplemental information for further learning on settings, values, or options. While reference information is helpful for users, reference information should not block or prevent users from completing tasks.

## Tone

plain, straightforward

## content\_type

```yaml
pcx_content_type: reference
products:
  - product-a
  - product-b
  - product-c
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## Structure

### Required components

[**Title**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/titles/): For a single Reference page, use "Reference" as the title. For a reference section with child pages, use nouns in the title. For example, [Common Cf-Polished statuses ↗](https://developers.cloudflare.com/images/polish/cf-polished-statuses/).

[**Context**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/context/): Provide an introductory paragraph to explain how and why a user might utilize the information on this page.

### Optional components

**Code snippets**: Examples of API responses or commands to run certain tasks.

[**Dynamic Lists**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/dynamic-lists/): Long lists of fields (more than 20).

[**Examples**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/examples/): Code samples that reference a specific configuration or API call.

[**Notes/tips/warnings**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/notes-tips-warnings/): Relevant information that can help or simplify concepts or warn users of potential impacts.

**Screenshots**: Images of a completed configuration for complicated tasks.

**Tables**: Longer lists of features and an associated number value or terms and their definitions.

## Examples

[Cache: Common Cf-Polished statuses](https://developers.cloudflare.com/images/polish/cf-polished-statuses/)

[Logpush: Logpush API configuration](https://developers.cloudflare.com/logs/logpush/logpush-job/api-configuration/)

## Template

Single reference page

```plaintext

---
weight: xx
pcx_content_type: reference
description: Reference information for <Cloudflare product or feature>, including <key settings, values, or options covered>.
products:
  - product-a
  - product-b
  - product-c
---

# Reference

Write an overview of the reference information on this page. If this section has child pages, add navigation links below using the DirectoryListing snippet to add links for each child page in a bulleted list.

<DirectoryListing path="/reference"/>

## Concise noun title

Brief description of content in this section.

## Concise noun title

Brief description of content in this section.
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/reference/#page","headline":"Reference · Cloudflare Style Guide","description":"Write reference documentation for APIs and configs.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/reference/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-24","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
