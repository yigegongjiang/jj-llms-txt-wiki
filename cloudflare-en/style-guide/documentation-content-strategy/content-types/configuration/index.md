---
description: Write configuration reference documentation.
title: Configuration
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Configuration

Last updated Apr 24, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/configuration/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

## Purpose

The purpose of a configuration is to show examples of specific settings, values, and options.

## Tone

plain, descriptive, straightforward

## content\_type

```yaml
pcx_content_type: configuration
products:
  - product-a
  - product-b
  - product-c
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## When to use

Configurations are useful for parts of the product that are very configuration-intensive; for example, rules.

## Structure

### Required components

[**Title**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/titles/): The title should be noun-based, because configurations are not designed to guide users towards achieving a goal – rather, they describe common ways to set up a specific feature depending on the user's needs.

**Context**: The context should be given in a paragraph right after the title. It should introduce the features, contextualize what type of configurations the user will encounter, and link to other relevant documentation.

**Settings and values**: This should be a reference table with a 1:1 correspondence between a setting the user can change, and the value they should input/select in order to reach the goal outlined in the context paragraph.

### Optional components

[**Navigation**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/navigation/): When we have many configurations to cover, it's useful to include a navigation list to help the user find what they need.

## Template

```plaintext

---
weight: xx
pcx_content_type: configuration
description: Configure <Cloudflare product or feature> settings for <use case or goal>. Covers <key settings or values>.
products:
  - product-a
  - product-b
  - product-c
---

# Title

Write an overview of the high-level feature here, not more than 2-3 sentences. Outline what users can achieve with it, and if necessary, link to other parts of the docs.

* [Feature 1](/feature-1)
* [Feature 2](/feature-2)
* [Feature 3](/feature-3)

## Feature 1

(Feature 1) allows you to (placeholder). For example, the following configuration (placeholder).

| Setting 1 | Setting 2 | Setting 3 |

| - | - | - |

| Value 1 | Value 2 | Value 3 |

## Feature 2

(Feature 2) allows you to (placeholder). For example, the following configuration (placeholder).

| Setting 1 | Setting 2 | Setting 3 |

| - | - | - |

| Value 1 | Value 2 | Value 3 |

## Feature 3

(Feature 3) allows you to (placeholder). For example, the following configuration (placeholder).

| Setting 1 | Setting 2 | Setting 3 |

| - | - | - |

| Value 1 | Value 2 | Value 3 |
```

## Additional Information

Configurations, also known as use cases, are reference pages with examples of how you might set a product up based on your requirements. If you are creating a configuration and feel yourself wanting to include instructions, consider a [tutorial](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/tutorial/), [how-to](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/how-to/), or [example](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/examples/) instead.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.DMYpXs3t.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/configuration/#page","headline":"Configuration · Cloudflare Style Guide","description":"Write configuration reference documentation.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/configuration/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-24","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
