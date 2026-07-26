---
description: Write task-oriented how-to documentation.
title: How to
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# How to

Last updated Apr 24, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/how-to/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

## Purpose

The purpose of a how to is to explain how to complete a task within the product.

Note

If you are unsure about when to categorize something as a how-to, tutorial, or solution guide, remember:

* A how-to helps a user who has already chosen a Cloudflare product complete a singular task within that product.
* A tutorial guides a user who has already chosen a Cloudflare product through a goal or use case, and may involve multiple products.
* A solution guide is for users who arrive with a goal or problem rather than a product name. It identifies which Cloudflare products apply and walks the user through configuring them together to achieve their goal.

If you are unsure which content type to use, refer to [How to select a content type](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/select-content-type/).

## Tone

instructional, straightforward

## content\_type

```yaml
pcx_content_type: how-to
products:
  - product-a
  - product-b
  - product-c
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## Structure

### Required components

[**Title**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/titles/): Short verb phrase in second-person imperative. Do not use gerund phrases.

[**Steps**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/steps-tasks-procedures/): Numbered steps that complete a task.

[**Next steps**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/next-steps/): What users should see as the end result of the steps and/or actionable next steps.

### Optional components

[**Context**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/context/): An introductory paragraph on the following steps and what they will accomplish.

Provide context to the reader that is not in the section heading.

End with a colon or a period. Use a colon if it immediately precedes the steps. Use a period if there is more material (such as a note) between the context and the procedure.

Do not provide context for steps with a partial sentence that is completed by the numbered steps.

[**Prerequisites**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/prerequisites/): Tasks or conditions that must be completed before a user can complete a series of steps.

[**Notes/warnings**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/notes-tips-warnings/)

[**Examples**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/examples/)

**Screenshots**

[**Related links**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/links/): Bulleted list of links to associated resources.

## Template

Single procedure how-to

```plaintext

---
weight: xx
pcx_content_type: how-to
description: <Verb> <Cloudflare product or feature> to <what the task accomplishes>. <Key detail or prerequisite>.
products:
  - product-a
  - product-b
  - product-c
---

# Second-person imperative verb phrase

Context for procedure (optional)

1. Step one
1. Step two
1. Step three
1. ...

Next steps sentence - what users should see as the end result and/or actionable next steps.
```

How-to with multiple procedures

```plaintext

---
weight: xx
pcx_content_type: how-to
description: <Verb> <Cloudflare product or feature> to <what the task accomplishes>. <Key detail or prerequisite>.
products:
  - product-a
  - product-b
  - product-c
---

# Second-person imperative verb phrase

Context for procedures on page (optional)

## Second-person imperative verb phrase

1. Step one
1. Step two
1. Step three
1. ...

Next steps sentence - what users should see as the end result and/or actionable next steps.

## Second-person imperative verb phrase

1. Step one
1. Step two
1. Step three
1. ...

Next steps sentence - what users should see as the end result and/or actionable next steps.
```

How-to with multiple procedures that must be completed in order

```plaintext

---
weight: xx
pcx_content_type: how-to
description: <Verb> <Cloudflare product or feature> to <what the task accomplishes>. <Key detail or prerequisite>.
products:
  - product-a
  - product-b
  - product-c
---

# Second-person imperative verb phrase

Context for procedures on page (optional)

## 1. Second-person imperative verb phrase

1. Step one
1. Step two
1. Step three
1. ...

Next steps sentence - what users should see as the end result and/or actionable next steps.

## 2. Second-person imperative verb phrase

1. Step one
1. Step two
1. Step three
1. ...

Next steps sentence - what users should see as the end result and/or actionable next steps.

## 3. Second-person imperative verb phrase

1. Step one
1. Step two
1. Step three
1. ...

Next steps sentence - what users should see as the end result and/or actionable next steps.
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.DMYpXs3t.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/how-to/#page","headline":"How to · Cloudflare Style Guide","description":"Write task-oriented how-to documentation.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/how-to/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-24","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
