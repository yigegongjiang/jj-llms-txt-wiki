---
description: Follow file naming and organization conventions.
title: File conventions
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# File conventions

Last updated Apr 24, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/file-conventions/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

Our docs have a few conventions around files.

## Naming

When creating new files, follow specific conventions for your naming.

Filenames should:

* Semantically communicate the purpose of the file
* Be lowercased
* Use dashes between words

```txt
/src/content/docs/fundamentals/concepts/what-is-cloudflare.mdx
/src/assets/images/api-shield/api-shield-call-sequence.png
```

```txt
/src/content/docs/fundamentals/concepts/What is Cloudflare.mdx
/src/content/docs/fundamentals/concepts/What-is-Cloudflare.mdx
/src/assets/images/api-shield/API_Image_1.png
```

These conventions are important for user readability, SEO conventions, and making sure our GitHub actions do not break.

## Folders

Each folder should have a file named `index.mdx`.

```txt
/src/content/docs/fundamentals/concepts/index.mdx
```

The content at `/src/content/docs/fundamentals/concepts/index.mdx` will be rendered at `https://developers.cloudflare.com/fundamentals/concepts/`.

## Content files

Add regular content files to the `/src/content/docs/{product_folder}/` directory.

```txt
/src/content/docs/fundamentals/concepts/what-is-cloudflare.mdx
```

## Image files

Add image files to the `/src/assets/images/{product_folder}/` directory.

```txt
/src/assets/images/api-shield/api-shield-call-sequence.png
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/file-conventions/#page","headline":"File conventions · Cloudflare Style Guide","description":"Follow file naming and organization conventions.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/file-conventions/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-24","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
