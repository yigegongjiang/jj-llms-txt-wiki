---
description: Write conceptual documentation that explains what something is, how it works, and where its boundaries lie.
title: Concept
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Concept

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/concept/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

A concept page builds the reader's mental model of a topic: what a thing is, why it works the way it does, and where its boundaries lie. It serves anyone getting oriented or already operating the product and filling in the why. The tone is instructional, descriptive, approachable, and supportive.

## When to use it

Write a concept page when readers keep needing the same explanation in the middle of other pages, because that recurring detour is the signal the model deserves its own home. It is not:

* **A how-to.** A concept carries no procedural steps or configuration walkthroughs. Code that shows the idea is welcome, code the reader follows along with is not.
* **A reference.** Reference is complete and neutral, whereas a concept is selective and opinionated, so "we recommend" belongs here.
* **An overview.** An overview routes the reader onward, whereas a concept explains. Keep one concept per page.

For the full comparison, refer to [Content types](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/).

## Title & description

* **Title**: a concise noun phrase naming the concept. Use "About" for a high-level product concept page. Otherwise use a feature name, functionality, or Internet concept such as Health checks or CDN. Do not use "Overview", "Introduction", or "How it works", because they name the genre rather than the subject. As a self-check, a good title still reads naturally with "About" in front of it.
* **Description**: state what the concept is and what it means for the reader's code or choices.

## Scaffold this page

Use the Nimbus concept recipe to generate this page. Your coding agent pulls the full page skeleton and self-review checklist, then adapts them to your product:

npmyarnpnpm

```
npx @cloudflare/nimbus-docs add content-concept
```

```
yarn @cloudflare/nimbus-docs add content-concept
```

```
pnpm @cloudflare/nimbus-docs add content-concept
```

Adapt the frontmatter the recipe emits to Cloudflare's schema: set [pcx\_content\_type](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype) and `products` instead of the generic fields the recipe emits, such as `type`.

## Component guidance

* **Prose is the primary component.** Short paragraphs, one idea per section: this is the type where writing quality carries the page.
* **Diagrams and illustrative code** fit when they show the model, and a diagram always ships with a text equivalent.
* [**Comparison tables**](https://developers.cloudflare.com/style-guide/style-and-grammar/formatting/structure/tables/) fit where there is a genuine either/or, alongside the boundaries the concept is confused with.
* **What does not fit:** Steps (a concept has no procedural steps or walkthroughs), Tabs (a concept does not vary by platform, and if it does it is two concepts), and Cards.

## Frontmatter

```yaml
pcx_content_type: concept
products:
  - product-a
  - product-b
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## Writing for AI and agents

* **Self-contained definition.** Lead with the contract in checkable terms such as "at least once" or "not ordered" rather than reassuring adjectives, because the definition paragraphs are what an agent retrieves and quotes, so they must stand on their own.
* **Literal payloads.** Keep illustrative code and payloads in fenced blocks with complete, realistic values rather than paraphrase.
* **Declarative boundaries.** Write the boundaries as flat declarative bullets stating what the concept is not, so an agent can lift them without reconstructing the prose.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/concept/#page","headline":"Concept · Cloudflare Style Guide","description":"Write conceptual documentation that explains what something is, how it works, and where its boundaries lie.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/concept/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
