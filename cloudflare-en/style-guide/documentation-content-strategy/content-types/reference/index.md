---
description: Write reference documentation that enumerates the settings, values, and options of one surface, completely and in a uniform structure.
title: Reference
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Reference

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/reference/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

A reference page enumerates the facts about one nameable surface, such as a file format, a command, or a set of limits, completely and in a uniform structure. The tone is plain and straightforward.

## When to use it

Reach for a reference page when a reader needs to look up exact facts about one surface. Two laws govern the type: completeness, because a missing entry breaks a reference the way a missing word breaks a dictionary, and uniformity, because every entry answers the same questions in the same order. It is not:

* **A how-to.** Reference describes, never instructs. A "to do this, first ..." entry means you extract a how-to and link it.
* **A concept.** Opinion and rationale live on the concept page, so one orienting sentence with a concept link is the whole prose allowance at the top.
* **A dumping ground.** "Miscellaneous" is where facts go to become unfindable. Keep one surface per page and mirror the product's own structure.

For the full comparison, refer to [Content types](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/). For live examples, refer to [Common Cf-Polished statuses](https://developers.cloudflare.com/images/polish/cf-polished-statuses/) and [Logpush API configuration](https://developers.cloudflare.com/logs/logpush/logpush-job/api-configuration/).

## Title & description

* **Title**: the surface's name as the reader searches for it, such as "CLI commands", "Event types", or "Limits". Use "Reference" for a single standalone page, use nouns for a section with child pages, and add "reference" to a noun only when the bare name is ambiguous, as in "Retry policy reference".
* **Description**: name the entry kinds the surface accepts and the fact categories each entry lists.

## Scaffold this page

Use the Nimbus reference recipe to generate this page. Your coding agent pulls the full page skeleton and self-review checklist, then adapts them to your product:

npmyarnpnpm

```
npx @cloudflare/nimbus-docs add content-reference
```

```
yarn @cloudflare/nimbus-docs add content-reference
```

```
pnpm @cloudflare/nimbus-docs add content-reference
```

Adapt the frontmatter the recipe emits to Cloudflare's schema: set [pcx\_content\_type](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype) and `products` instead of the generic fields the recipe emits, such as `type`.

## Component guidance

* [**Tables**](https://developers.cloudflare.com/style-guide/style-and-grammar/formatting/structure/tables/) are the signature component: a quick-reference table before the entries makes the common lookup zero-scroll. Keep tables simple, because merged cells and meaning-by-layout break both scanning and extraction.
* **Definition lines** carry the same facts in a fixed order, such as type, default, required, and constraints, bolded or badged consistently across every entry.
* [**DirectoryListing**](https://developers.cloudflare.com/style-guide/build-the-page/components/directory-listing/) links the child pages when the reference is a section rather than a single page.
* **What does not fit:** Steps, Cards, and callouts (a fact that needs a warning usually belongs in the entry as a constraint), plus tabs or accordions that hide entries (a collapsed entry is invisible to search-and-grab readers and to extraction).

## Frontmatter

```yaml
pcx_content_type: reference
products:
  - product-a
  - product-b
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## Completeness and source of truth

A reference is complete or clearly scoped, with nothing in between, so if a subset lives elsewhere the first line says where. Give every fact exactly one source: generate values that live in code or a schema, and where generation does not yet exist, name the source so maintainers know what to diff against. Hand-maintained fact pages such as limits or quotas carry a visible `reviewed` date. When a surface passes roughly 30 entries, split it along its own seams, such as file layout or command groups, never alphabetically.

## Writing for AI and agents

* **Self-identifying entries.** Give every entry heading its full name, such as the dotted path `retry.max_attempts` rather than `max_attempts` under a "Retry" heading, so a retrieved chunk carries its own identity.
* **Machine-checkable values.** State ranges, defaults, and limits as literal values rather than "a reasonable number", and keep examples minimal in fenced blocks with realistic values.
* **Nothing hidden.** Keep every entry in the open, because a collapsed or tabbed entry is invisible to extraction. The Markdown twin of a reference page is the page.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/reference/#page","headline":"Reference · Cloudflare Style Guide","description":"Write reference documentation that enumerates the settings, values, and options of one surface, completely and in a uniform structure.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/reference/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
