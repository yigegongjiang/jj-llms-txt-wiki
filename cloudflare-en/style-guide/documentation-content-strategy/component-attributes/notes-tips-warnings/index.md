---
description: Use admonitions effectively in documentation.
title: Notes/tips/warnings
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Notes/tips/warnings

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/notes-tips-warnings/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

## Definition

A colored info box or aside with content (text, images, lists, code blocks) that adds relevant notes that do not fit the text or warns users of specific behavior that can break functionality or impact security.

## Used in

[How to](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/how-to/), [Configuration](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/configuration/), [FAQ](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/faq/), [Concept](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/concept/), [Reference](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/reference/), [Tutorial](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/tutorial/)

## Structure

**Type**: note or warning (defines the background color)

**Aside content**

**(optional) Title/Header**

## Templates

To learn how to format notes, refer to [Notes and other notation types](https://developers.cloudflare.com/style-guide/style-and-grammar/formatting/notes-and-other-notation-types/).

## Rendered examples

Header text

This is a note with a header.

Note

This is a note without a header.

Caution

This is a warning.

Tip

This is a tip.

## When should I use a note/warning?

Use a note to alert a reader to additional useful information that you cannot integrate into the text.

Use a warning to alert a reader to behavior that could impact the security of a users network or break functionality.

## Recommendations

* **An aside should not contain too much content**, since it breaks the normal text flow. For example, up to 3 paragraphs or bulleted lists up to 3 items. If you need to include more content, consider creating a documentation section "Important notes" or similar.
* **Use asides sparingly.** Each section should not have more than one aside of the same type. The only exception is a possible availability disclaimer right after the heading.
* **Asides inside task step instructions should not have a header.** They take too much space and the background color is enough to distinguish the aside content from regular text.
* **Use a `note` aside to state the restricted availability of a feature** (for example, "Only available for customers on an Enterprise plan.") at the beginning of a page, without a header.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/notes-tips-warnings/#page","headline":"Notes/tips/warnings · Cloudflare Style Guide","description":"Use admonitions effectively in documentation.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/notes-tips-warnings/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
