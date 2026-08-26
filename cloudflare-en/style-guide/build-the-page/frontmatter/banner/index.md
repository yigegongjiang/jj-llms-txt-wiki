---
description: How to display a banner at the top of the page and when to use it.
title: Banner
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

Do **not** use banners in the [Frontmatter](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/) unless a change will cause customer application to break.

# Banner

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/banner/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

One of the fields you can add to the [Frontmatter](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/) is `banner`. It displays a prominent section at the top of the page and supports the use of HTML for links and formatting.

Only use it to alert about disruptive situations and take note to remove it when applicable.

## Example

```mdx
---
title: Banner
description: How to display a banner at the top of the page and when to use it.
banner:
  content: Do <strong>not</strong> use banners in the <a href="https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/">Frontmatter</a> unless a change will cause customer application to break.
---
```

## Styles / Types

### Note

The note banner is used to alert about important information.

```mdx
---
title: Banner
description: How to display a banner at the top of the page and when to use it.
banner:
  content: Ensure you read this!
  type: note
---
```

### Tip

The tip banner is used to alert about important suggestions.

```mdx
---
title: Banner
description: How to display a banner at the top of the page and when to use it.
banner:
  content: Consider this alternative!
  type: tip
---
```

### Caution

The caution banner is used to warn readers of upcoming disruptive changes.

```mdx
---
title: Banner
description: How to display a banner at the top of the page and when to use it.
banner:
  content: This is deprecated and will break on <strong>1970-01-01</strong>!
  type: caution
---
```

### Danger

The danger banner is used to alert about errors.

```mdx
---
title: Banner
description: How to display a banner at the top of the page and when to use it.
banner:
  content: This has been removed!
  type: danger
---
```

### Default

The default banner is used in all other circumstances.

```mdx
---
title: Banner
description: How to display a banner at the top of the page and when to use it.
banner:
  content: Do <strong>not</strong> use banners in the <a href="https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/">Frontmatter</a> unless a change will cause customer application to break.
---
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/banner/#page","headline":"Banner · Cloudflare Style Guide","description":"How to display a banner at the top of the page and when to use it.","url":"https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/banner/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
