---
description: Write task-oriented how-to documentation that guides a reader through completing a single task in a Cloudflare product.
title: How to
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# How to

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/how-to/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

A how-to explains how to complete a single task within a product. The tone is instructional and straightforward.

## When to use it

Use a how-to when the reader has already chosen a product and needs to complete one specific task within it. It is not:

* **A tutorial.** A tutorial teaches by building something and cannot fail the reader, whereas a how-to serves someone mid-task who already knows the goal.
* **A concept.** If you find yourself explaining why the product works this way for more than a sentence, move it to a concept page and link to it.

For the full comparison, refer to [Content types](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/).

## Title & description

* **Title**: a short verb phrase in the second-person imperative. Do not use gerunds, bare nouns, or a "How to" prefix.
* **Description**: start with a verb, name the Cloudflare product or feature, and state the task it accomplishes, then add a key detail or prerequisite.

## Scaffold this page

Use the Nimbus how-to recipe to generate this page. Your coding agent pulls the full page skeleton and self-review checklist, then adapts them to your product:

npmyarnpnpm

```
npx @cloudflare/nimbus-docs add content-how-to
```

```
yarn @cloudflare/nimbus-docs add content-how-to
```

```
pnpm @cloudflare/nimbus-docs add content-how-to
```

Adapt the frontmatter the recipe emits to Cloudflare's schema: set [pcx\_content\_type](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype) and `products` instead of the generic fields the recipe emits, such as `type`.

## Component guidance

* [**Steps**](https://developers.cloudflare.com/style-guide/build-the-page/components/steps/) are the signature structure: the Steps component or a plain ordered list, whichever you use, must read identically in the Markdown twin. If a page has no steps, question whether it is a how-to.
* [**Tabs and code groups**](https://developers.cloudflare.com/style-guide/build-the-page/components/tabs/) carry variant axes such as language, platform, or CLI versus dashboard inside one canonical page. For alternative methods, pick the recommended one and link the rest, and never duplicate the page.
* **Callouts** warn before a destructive step. A page drowning in exception callouts has the wrong happy path.
* **End in a fixed order:** verification, then the irreversible closing step if there is one, then optional blocks, then Next steps. Never end on the last numbered step.
* **Multi-procedure pages** number their section headings (`## 1.`, `## 2.`) so the sequence is unambiguous, and cap each phase at roughly ten steps.

## Frontmatter

```yaml
pcx_content_type: how-to
products:
  - product-a
  - product-b
  - product-c
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## Writing for AI and agents

* **Self-contained steps.** Name the product area, the full command, and the exact label the reader selects, and never use a positional reference such as "as configured above."
* **Literal output.** Keep expected output in fenced code blocks with complete, realistic values rather than truncated placeholders, because agents match on the literal text you show.
* **Twin-safe steps.** When a step's only copy lives inside a tab or other component, make sure it survives as labeled text in the Markdown twin so conversion cannot drop it.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/how-to/#page","headline":"How to · Cloudflare Style Guide","description":"Write task-oriented how-to documentation that guides a reader through completing a single task in a Cloudflare product.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/how-to/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
