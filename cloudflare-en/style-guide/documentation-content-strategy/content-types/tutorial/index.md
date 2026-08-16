---
description: Write step-by-step tutorial documentation.
title: Tutorial
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Tutorial

Last updated Aug 10, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/tutorial/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

A tutorial is a practical lesson that takes you from a clear starting to ending point.

The goal is to connect products to real-world scenarios to meet a user’s goal.

Note

If you are unsure about when to categorize something as a how-to, tutorial, or solution guide, remember:

* A how-to helps a user who has already chosen a Cloudflare product complete a singular task within that product.
* A tutorial guides a user who has already chosen a Cloudflare product through a goal or use case, and may involve multiple products.
* A solution guide is for users who arrive with a goal or problem rather than a product name. It identifies which Cloudflare products apply and walks the user through configuring them together to achieve their goal.

If you are unsure which content type to use, refer to [How to select a content type](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/select-content-type/).

## Guidelines

**A tutorial is:**

* For users who have already chosen a Cloudflare product to work with
* User-focused
* Aligned to a user's goal or job-to-be-done
* Descriptive and guiding

**A tutorial can:**

* Describe how to integrate with a third party
* Be delivered in the Cloudflare dashboard
* Describe how to set up multiple products to complete a single job-to-be-done

**A tutorial is not:**

* Product configuration information, how-to (or any of the other content types)
* How to complete a task in the UI or API
* A dumping ground for screenshots
* Content with no end goal or job-to-be-done

### Tone

Guiding, straightforward, educational, authoritative

### content\_type

```yaml
pcx_content_type: tutorial
products:
  - product-a
  - product-b
  - product-c
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

### Components

#### Most used

* [ListTutorials](https://developers.cloudflare.com/style-guide/components/list-tutorials/)

#### Required

[**Title**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/titles/): Short verb phrase in second-person imperative.

**Context**: An introductory paragraph on the user's goal or job-to-be-done and how they will accomplish that in the tutorial. Consider including the intended audience for the tutorial. Refer to [Context](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/context/) for more information.

**Consider the user story framing**: "As a `___`, I want to `___` so I can `___`."

[**Steps**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/steps-tasks-procedures/): Numbered steps that complete a task.

#### Optional

[**Notes/warnings**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/notes-tips-warnings/)

[**Examples**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/examples/)

**Screenshots**

[**Links**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/links/)

**Boundaries**

## Template

```plaintext

---
pcx_content_type: tutorial
title: Second-person imperative verb phrase
products:
  - <product>
description: Build <what the tutorial produces> with <Cloudflare product>. Covers <key technologies or steps involved>.
difficulty: Beginner
---

# Second-person imperative verb phrase

Context paragraph describing the user's goal and how the tutorial accomplishes it.

## Prerequisites

## 1. First step

## 2. Second step

## Next steps
```

## Examples

[Workers Tutorials](https://developers.cloudflare.com/workers/tutorials)

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/tutorial/#page","headline":"Tutorial · Cloudflare Style Guide","description":"Write step-by-step tutorial documentation.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/tutorial/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-10","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
