---
description: Write design guides that walk a reader through planning and designing a specific Cloudflare solution and the architecture decisions behind it.
title: Design guide
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Design guide

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/design-guide/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

A design guide helps a reader plan and design a specific solution with Cloudflare, focusing on the architecture decisions behind that solution before any configuration. A design guide is a focused subset of a [reference architecture](https://developers.cloudflare.com/reference-architecture/). The tone is instructional and straightforward.

## When to use it

Write a design guide when a reader needs to plan the architecture of one specific solution, understanding the decisions and trade-offs before they build it. It is not:

* **A reference architecture.** A reference architecture describes a broad, product-spanning architecture, whereas a design guide narrows to planning one specific solution within it.
* **A how-to.** A how-to gives the steps to configure a product, whereas a design guide reasons through the architecture decisions and trade-offs before any configuration begins.

For the full comparison, refer to [Content types](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/).

## Title & description

* **Title**: a short verb phrase in the second-person imperative, not a gerund. Prefer "Securely deliver applications with Cloudflare" over "Securely delivering applications".
* **Description**: name the solution the reader will plan and design, the Cloudflare products involved, and the architecture decisions covered.

## Scaffold this page

Copy this skeleton and adapt it to your solution:

```plaintext
---
title: <Verb phrase naming the solution to plan>
description: Plan and design <solution> with <Cloudflare products>, covering <the architecture decisions>.
pcx_content_type: design-guide
sidebar:
  order: 10
products:
  - product-a
---

Open with two or three paragraphs describing the subject matter and the end state of the solution.

## Intended audience

Summarize who the guide is for and what they will learn.

## <Architecture decision or design area>

Describe the design and the decisions and trade-offs behind it, include a diagram of the architecture, and link out to the how-tos and tutorials that implement it.

## Related links

Point to the reference architecture and product documentation the design draws on.
```

## Component guidance

* [**Introduction**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/introductions/#introduction) opens with two to three paragraphs describing the subject matter and the end state of the solution the guide details.
* [**Intended audience**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/introductions/#intended-audience) summarizes who the guide is for and what they will learn.
* [**Diagrams**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/images-and-diagrams/#diagrams) show the architecture, which is central to a design guide.
* [**Notes and warnings**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/notes-tips-warnings/) flag caveats and trade-offs that affect how the design applies.
* [**Related links**](https://developers.cloudflare.com/style-guide/style-and-grammar/formatting/structure/links/) point to the reference architecture and product documentation the design draws on.
* **What does not fit:** step-by-step configuration procedures. A design guide plans the solution, so link out to the how-tos and tutorials that implement it.

## Frontmatter

```yaml
pcx_content_type: design-guide
products:
  - product-a
  - product-b
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## Examples

* [Securely deliver applications with Cloudflare](https://developers.cloudflare.com/reference-architecture/design-guides/secure-application-delivery/)

## Writing for AI and agents

* **Design, not steps.** Describe the architecture and the decisions behind it, linking out to the how-tos and tutorials that implement them, because a design guide is a plan an agent reasons from, not a procedure it runs.
* **State the end state.** Describe the finished solution the guide produces up front, so a reader or agent knows the target before following the design.
* **Name the audience and assumptions.** State who the guide is for and the infrastructure it assumes, because a design guide is only actionable for a reader with the right context.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/design-guide/#page","headline":"Design guide · Cloudflare Style Guide","description":"Write design guides that walk a reader through planning and designing a specific Cloudflare solution and the architecture decisions behind it.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/design-guide/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
