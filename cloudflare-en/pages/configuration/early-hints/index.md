---
description: Improve page load performance on Cloudflare Pages with Early Hints for preloading assets.
title: Early Hints
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/pages/llms.txt  
> Use this file to discover all available pages before exploring further.

# Early Hints

Last updated Apr 21, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/pages/configuration/early-hints/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

[Early Hints](https://developers.cloudflare.com/cache/advanced-configuration/early-hints/) help the browser to load webpages faster. Early Hints is enabled automatically on all `pages.dev` domains and custom domains.

Early Hints automatically caches any [preload ↗](https://developer.mozilla.org/en-US/docs/Web/HTML/Link%5Ftypes/preload) and [preconnect ↗](https://developer.mozilla.org/en-US/docs/Web/HTML/Link%5Ftypes/preconnect) type [Link headers ↗](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Link) to send as Early Hints to the browser. The hints are sent to the browser before the full response is prepared, and the browser can figure out how to load the webpage faster for the end user. There are two ways to create these `Link` headers in Pages:

## Configure Early Hints

Early Hints can be created with either of the two methods detailed below.

### 1\. Configure your `_headers` file

Create custom headers using the [\_headers file](https://developers.cloudflare.com/pages/configuration/headers/). If you include a particular stylesheet on your `/blog/` section of your website, you would create the following rule:

```txt
/blog/*
  Link: </styles.css>; rel=preload; as=style
```

Pages will attach this `Link: </styles.css>; rel=preload; as=style` header. Early Hints will then emit this header as an Early Hint once cached.

### 2\. Automatic `Link` header generation

In order to make the authoring experience easier, Pages also automatically generates `Link` headers from any `<link>` HTML elements with the following attributes:

* `href`
* `as` (optional)
* `rel` (one of `preconnect`, `preload`, or `modulepreload`)

`<link>` elements which contain any other additional attributes (for example, `fetchpriority`, `crossorigin` or `data-do-not-generate-a-link-header`) will not be used to generate `Link` headers in order to prevent accidentally losing any custom prioritization logic that would otherwise be dropped as an Early Hint.

This allows you to directly create Early Hints as you are writing your document, without needing to alternate between your HTML and `_headers` file.

```html
<html>
	<head>
		<link rel="preload" href="/style.css" as="style" />
		<link rel="stylesheet" href="/style.css" />
	</head>
</html>
```

### Disable automatic `Link` header generation Automatic `Link` header

Remove any automatically generated `Link` headers by adding the following to your `_headers` file:

```txt
/*
  ! Link
```

Caution

Automatic `Link` header generation should not have any negative performance impact on your website. If you need to disable this feature, contact us by letting us know about your circumstance in our [Discord server ↗](https://discord.com/invite/cloudflaredev).

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/pages/configuration/early-hints/#page","headline":"Early Hints · Cloudflare Pages docs","description":"Improve page load performance on Cloudflare Pages with Early Hints for preloading assets.","url":"https://developers.cloudflare.com/pages/configuration/early-hints/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-21","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
