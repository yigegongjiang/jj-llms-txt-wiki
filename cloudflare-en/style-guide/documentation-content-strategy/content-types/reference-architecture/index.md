---
description: Write reference architecture documentation that shows how Cloudflare products fit a customer's infrastructure and maps use cases to solutions.
title: Reference architecture
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Reference architecture

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/reference-architecture/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

A reference architecture is a high-level design document that shows how Cloudflare products fit into a customer's existing infrastructure and maps their use cases to Cloudflare solutions. The tone is guiding and straightforward.

This page covers how to write one. For the published architectures themselves, refer to [Reference architectures](https://developers.cloudflare.com/reference-architecture/).

## When to use it

Write a reference architecture when you need to show, at the design level, how several Cloudflare products combine to fit a customer's environment and use cases. These documents are typically detailed. It is not:

* **A concept.** A concept explains one idea in depth, whereas a reference architecture shows how multiple products fit together in a real environment.
* **A how-to.** A reference architecture describes and designs rather than giving procedural steps.

For a single architecture that needs little written explanation, use a [reference architecture diagram](#reference-architecture-diagrams) instead. For the full comparison, refer to [Content types](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/). For live examples, refer to [Cloudflare Load Balancing Reference Architecture](https://developers.cloudflare.com/reference-architecture/architectures/load-balancing/), [Magic Transit Reference Architecture](https://developers.cloudflare.com/reference-architecture/architectures/magic-transit/), and [Evolving to a SASE architecture with Cloudflare](https://developers.cloudflare.com/reference-architecture/architectures/sase/).

## Title & description

* **Title**: a noun phrase naming the architecture or solution, such as "Cloudflare Load Balancing Reference Architecture".
* **Description**: name the solution and the products, say how they fit into existing infrastructure and for which use case, and name the intended audience, such as IT and security professionals.

## Scaffold this page

Copy this skeleton and adapt it to your architecture:

```plaintext
---
title: <Noun phrase naming the architecture or solution>
description: How <products> fit <infrastructure> for <use case>, written for <the intended audience>.
pcx_content_type: reference-architecture
sidebar:
  order: 10
products:
  - product-a
---

Open with two or three paragraphs on the subject matter, then state who the document is for and what they will learn.

## <Architecture area>

Present the reference diagram with numbered callouts, and explain each element in prose so the meaning survives without the image.

## <Use case to solution>

Map the customer use case to the Cloudflare solution, and flag any caveats that affect how the architecture applies.

## Related links

Link the supporting how-tos, concepts, and product documentation with current routes.
```

## Component guidance

* **Diagrams** are the signature component: a single reference diagram reflects the overall architecture, with captions and numbered callouts explaining each element, and supporting diagrams develop specific parts.
* **Introduction and intended audience** open the document in prose, with two or three paragraphs on the subject matter followed by who it is for and what they will learn.
* **Notes and warnings** flag caveats that affect how the architecture applies.
* [**PublicStats**](https://developers.cloudflare.com/style-guide/build-the-page/components/public-stats/) surfaces Cloudflare network statistics where they strengthen the case for the architecture.
* **What does not fit:** procedural steps, because a reference architecture designs rather than instructs. Link a how-to for implementation.

## Frontmatter

```yaml
pcx_content_type: reference-architecture
products:
  - product-a
  - product-b
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## Reference architecture diagrams

A reference architecture diagram is the lighter variant: a single diagram with numbered callouts and just enough text to explain it, for a solution that does not need a full written architecture. The tone is instructional and straightforward.

* **When to use it**: reach for it when one diagram carries the solution and needs little written explanation. Choose a full reference architecture when the design needs detailed discussion.
* **Title**: a noun phrase, as for a full reference architecture.
* **Structure**: a single reference diagram, numbered callouts that explain each element, a short description of what the diagram relates to, and related links to supporting content.
* **Frontmatter**: set `pcx_content_type` to `reference-architecture-diagram`.

## Writing for AI and agents

* **Text equivalents for every diagram.** Explain each numbered callout in prose and give the diagram a text equivalent, because an agent cannot read the image and the meaning must survive without it.
* **Literal product and use-case names.** Name the exact Cloudflare products and the use case in text, not only inside the diagram, so the architecture is retrievable on its own.
* **Load-bearing links.** Link the supporting how-tos, concepts, and product docs with real, current routes, because the architecture points outward to implementation.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/reference-architecture/#page","headline":"Reference architecture · Cloudflare Style Guide","description":"Write reference architecture documentation that shows how Cloudflare products fit a customer's infrastructure and maps use cases to solutions.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/reference-architecture/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
