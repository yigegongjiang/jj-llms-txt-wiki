---
description: Cloudflare docs pages are authored in MDX, Markdown extended with components. Learn the body syntax, importing components, and escaping special characters.
title: Markdown and MDX
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Markdown and MDX

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/build-the-page/markdown-and-mdx/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

Cloudflare docs pages are authored in MDX, which is Markdown extended with JSX components. Every page is a `.mdx` file with a [frontmatter](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/) block at the top, followed by the body content.

## Body content

The body is standard Markdown. Use it for headings, paragraphs, lists, tables, links, and code:

```md
## A heading

A paragraph with a [link](/style-guide/) and `inline code`.

- A list item
- Another list item
```

Refer to the [formatting](https://developers.cloudflare.com/style-guide/style-and-grammar/formatting/) section for the rules that govern how to write each of these elements.

## Import components

Components add formatting that plain Markdown cannot, such as tabs, asides, and collapsible sections. Import them from `~/components` after the frontmatter block, then add them anywhere in the body:

```mdx
---
title: Example page
---

import { Aside } from "~/components";

<Aside type="note">This is an aside.</Aside>
```

Refer to the [components](https://developers.cloudflare.com/style-guide/build-the-page/components/) section for the props and requirements of each component.

## Escape special characters

MDX treats `{`, `}`, `<`, and `>` as syntax. When these characters are part of your content rather than code, wrap them in backticks so they render literally:

```md
Set the value to `{"key": "value"}`.
```

Characters inside a fenced code block are already literal and do not need escaping.

## Code blocks

Open a fenced code block with a lowercase language identifier so the code is highlighted correctly. Use `txt` for generic output that has no language:

```md
```js
const value = 1;
```

```txt
Deployment complete.
```
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/build-the-page/markdown-and-mdx/#page","headline":"Markdown and MDX · Cloudflare Style Guide","description":"Cloudflare docs pages are authored in MDX, Markdown extended with components. Learn the body syntax, importing components, and escaping special characters.","url":"https://developers.cloudflare.com/style-guide/build-the-page/markdown-and-mdx/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
