---
description: Use notes and admonitions consistently.
title: Notes and other notation types
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Notes and other notation types

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/style-and-grammar/formatting/notes-and-other-notation-types/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

When adding a note to a page, always use this special note formatting. There are three types of formatted notes: `note`, `caution`, and `tip`.

Here is some additional information about notes:

* The color of the note depends on the type of note: `note` is blue, `caution` is yellow, and `tip` is purple.
* For every note type, the header text is optional.
* All note types can contain text and additional formatting like lists, code blocks, and images.

To learn how notes fit into our content strategy, refer to [Notes/tips/warnings](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/notes-tips-warnings/).

## Note

Use Note for small additions or when you need to provide extra context that is not essential to the main content.

If you do not provide a header, this aside will default to `Note`.

```mdx
:::note[Header]
Hello, world!
:::
```

Header

Hello, world!

## Caution/Warning

Use Caution to highlight actions that could cause issues for a user.

If you do not provide a header, this aside will default to `Warning`.

```mdx
:::caution[Feature conflict]
If you use feature A and feature B together, your configuration will not work.
:::
```

Feature conflict

If you use feature A and feature B together, your configuration will not work.

## Tip

Use Tip to share best practices or opinionated use cases that do not fit into the main documentation.

If you do not provide a header, this aside will default to `Tip`.

```mdx
:::tip[Best practice]
Cloudflare recommends you use [1.1.1.1](/1.1.1.1/) as your DNS resolver.
:::
```

Best practice

Cloudflare recommends you use [1.1.1.1](https://developers.cloudflare.com/1.1.1.1/) as your DNS resolver.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/style-and-grammar/formatting/notes-and-other-notation-types/#page","headline":"Notes and other notation types · Cloudflare Style Guide","description":"Use notes and admonitions consistently.","url":"https://developers.cloudflare.com/style-guide/style-and-grammar/formatting/notes-and-other-notation-types/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
