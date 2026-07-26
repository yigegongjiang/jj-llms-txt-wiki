---
description: Write solution guides for complex use cases.
title: Solution guide
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Solution guide

Last updated Apr 24, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/solution-guide/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

## Purpose

A solution guide is for users who arrive with a goal or problem rather than a product name. Some users know they have a problem and need to solve it. Others are just getting started and want to know what they should configure. In both cases, the guide identifies which Cloudflare products apply and walks the user through configuring them together.

A solution guide answers the question: "I want to do X. What do I need, why does each piece matter, and how do I set it up?"

Solution guides live at `/use-cases/solutions/{guide-slug}/`, outside any product vertical. This placement ensures they are discoverable by users who have not yet chosen a product.

## When to use this content type

Note

If you are unsure about when to categorize something as a how-to, tutorial, or solution guide, remember:

* A how-to helps a user who has already chosen a Cloudflare product complete a singular task within that product.
* A tutorial guides a user who has already chosen a Cloudflare product through a goal or use case, and may involve multiple products.
* A solution guide is for users who arrive with a goal or problem rather than a product name. It identifies which Cloudflare products apply and walks the user through configuring them together to achieve their goal.

If you are unsure which content type to use, refer to [How to select a content type](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/select-content-type/).

## Tone

Guiding, plain language. Choose a recommended path and commit to it. Explain the reasoning behind each configuration step so the reader understands why, not just what. Define technical terms on first use. Write conceptual sections so a reader without a networking or security background can follow them.

Where meaningful alternatives exist due to plan tier, infrastructure configuration, or user preference, surface them in a callout with a link to the relevant documentation. Alternatives should not interrupt the main path or require the reader to make a decision before continuing.

## content\_type

```yaml
pcx_content_type: solution-guide
products:
  - product-a
  - product-b
  - product-c
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## Callouts

Use callouts to surface meaningful alternatives at the point in the guide where they become relevant. The most common case is when a feature requires a higher plan tier than the core workflow assumes. Always include a link to the relevant documentation.

The core workflow should be achievable on Free or Pro plans. Do not make Enterprise-only features the primary path through the guide.

Use this pattern:

```markdown
:::note[Optional title]
[Brief description of the alternative and when it applies.] Refer to [relevant documentation link].
:::
```

## Structure

Organize content by **workflow stage**, not by product. Each section should have a clear outcome so the reader knows what they have accomplished before moving on.

### Required

[**Title**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/titles/): A goal statement in plain language reflecting what the user is trying to accomplish, not the product name.

* Good: "Stop Malicious Bots While Allowing Legitimate Traffic"
* Avoid: "Bot Management Configuration Guide"

**Introduction**: One to two paragraphs describing the problem the user is experiencing and why it matters. Introduce the Cloudflare products involved after establishing the problem, not before.

[**Steps**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/steps-tasks-procedures/): Numbered steps within each workflow stage. Each step should explain what to do and why it matters.

**Callouts**: At each decision point where a meaningful alternative exists, include a callout with a link to the relevant documentation. Refer to [Callouts](#callouts) for the pattern.

### Optional

[**Prerequisites**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/prerequisites/): Conditions the user must meet before starting. Keep prerequisites minimal and specific.

[**Notes/warnings**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/notes-tips-warnings/)

[**Diagrams**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/diagrams/)

**Screenshots**

[**Related links**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/links/): Bulleted list of links to associated resources.

## Template

```plaintext
---
pcx_content_type: solution-guide
title: Goal-oriented title in plain language
description: <What the reader will accomplish> using <Cloudflare products involved>. Covers <key workflow stages>.
products:
  - product-a
  - product-b
  - product-c
---

# Goal-oriented title in plain language

Introductory paragraph describing the problem the user is experiencing and why it matters. Introduce the products involved after establishing the problem, not before.

## [First workflow stage]

### [First task]

Why this step matters.

1. Step one
1. Step two
1. Step three

:::note[Optional title]
[Brief description of the alternative and when it applies.] Refer to [relevant documentation link].
:::

### [Second task]

...

## [Second workflow stage]

...
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.DMYpXs3t.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/solution-guide/#page","headline":"Solution guide · Cloudflare Style Guide","description":"Write solution guides for complex use cases.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/solution-guide/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-24","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
