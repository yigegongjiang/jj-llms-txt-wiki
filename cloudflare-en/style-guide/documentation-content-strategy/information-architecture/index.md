---
description: Build every product's documentation from a shared core of sections, with the folder and ordering rules that keep the docs consistent.
title: Information architecture
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Information architecture

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/information-architecture/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

Every product's documentation is built from the same core set of sections, so a reader who knows one product's docs can predict where to find things in another.

Consistency is enforced, not optional. A product may add sections that it genuinely needs, but those additions are additive: a product never renames, restructures, or redefines a core section because a different shape "makes more sense" for it. Keep the core the same, and grow around it.

This page defines the shared core at the section (folder) level. To choose the type of an individual page, refer to [Content types](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/).

## Required sections

Every product includes at least these two pages, from its first release:

* [Overview](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/overview/), which orients a new reader and routes them onward.
* [Get started](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/get-started/), which takes a new user from nothing to a first working result.

## Core sections

Beyond the required pair, use these standard sections whenever your product has the content they describe. Use the standard name so that readers and agents navigate every product's docs the same way.

| Section        | What it contains                                                                | Related content type                                                                                                       |
| -------------- | ------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| Overview       | Orients a new reader to the product and routes them onward. Required.           | [Overview](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/overview/)           |
| Get started    | The shortest path from nothing to a first working result. Required.             | [Get started](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/get-started/)     |
| Concepts       | What the product's key ideas are and why they work the way they do.             | [Concept](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/concept/)             |
| Features       | Groups the task and settings content for a major feature of the product.        | [How to](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/how-to/)               |
| Guides         | Task-focused pages for completing one specific job.                             | [How to](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/how-to/)               |
| Tutorials      | End-to-end lessons where the reader builds a real project.                      | [Tutorial](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/tutorial/)           |
| Examples       | Complete, runnable samples that show how something is done.                     | None                                                                                                                       |
| Configuration  | The settings, values, and options for a configuration-intensive feature.        | [Configuration](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/configuration/) |
| Reference      | Complete, neutral lookup details such as parameters, values, and options.       | [Reference](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/reference/)         |
| API            | The product's API documentation and command guidance.                           | [API content strategy](https://developers.cloudflare.com/style-guide/api-content-strategy/)                                |
| Models         | The available models and their details, for AI products.                        | [Reference](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/reference/)         |
| Observability  | Testing, metrics, analytics, and local development.                             | None                                                                                                                       |
| Best practices | Recommended patterns and guidance for using the product well.                   | None                                                                                                                       |
| Platform       | Product-wide pages such as pricing, limits, changelog, betas, and known issues. | [Changelog](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/changelog/)         |
| Glossary       | The product's defined terms.                                                    | [Glossary](https://developers.cloudflare.com/style-guide/build-the-page/components/glossary/)                              |

## Structure and ordering rules

* Make each core section a folder, even when it currently holds a single page. A lone `get-started.mdx` becomes a `get-started/` folder.
* Place the core folders before any product-specific folders, in the order given under [Core sections](#core-sections).
* Give every product a Platform folder that holds at least one page, so this section is present consistently rather than only on some products.
* Name any product-specific folder uniquely and clearly. A product-specific folder is additive: it adds to the core, and it never replaces or reshapes a core section.
* Add sections freely, but do not edit the core. If a core section does not fit your product as written, raise it through docs governance rather than renaming or restructuring it locally.

## Bring an existing product into line

Audit the product against the core, then close the gaps:

* Rename non-standard folders to the standard names. For example, rename a `getting-started` folder to `get-started`, and rename a `how-to` folder to the standard `guides`.
* Fold loose files into their core folder. A single `concepts.mdx` becomes a `concepts/` folder.
* Pull core content up to the top level when it sits inside `platform/` but belongs to a core section.
* Create the core sections your product is missing.
* Keep useful product-specific folders, and confirm each one is uniquely named and additive.

## Product categories

The core applies across every product category, including Compute, Storage, AI, Media, and the vertical products. A category can share additional sections that its products all need. For example, AI products commonly add a Models section. As a product matures it keeps the same core and grows by adding sections, not by reshaping the core.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/information-architecture/#page","headline":"Information architecture · Cloudflare Style Guide","description":"Build every product's documentation from a shared core of sections, with the folder and ordering rules that keep the docs consistent.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/information-architecture/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
