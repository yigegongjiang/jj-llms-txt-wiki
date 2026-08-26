---
description: Enable automatic browser refresh in Miniflare when your Workers script changes during local development.
title: Live Reload
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/workers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Live Reload

Last updated Apr 23, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/workers/testing/miniflare/developing/live-reload/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

Miniflare automatically refreshes your browser when your Worker script changes when `liveReload` is set to `true`.

```js
const mf = new Miniflare({
	liveReload: true,
});
```

Miniflare will only inject the `<script>` tag required for live-reload at the end of responses with the `Content-Type` header set to `text/html`:

```js
export default {
	fetch() {
		const body = `
      <!DOCTYPE html>
      <html>
      <body>
        <p>Try update me!</p>
      </body>
      </html>
    `;

		return new Response(body, {
			headers: { "Content-Type": "text/html; charset=utf-8" },
		});
	},
};
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/workers/testing/miniflare/developing/live-reload/#page","headline":"Live Reload · Cloudflare Workers docs","description":"Enable automatic browser refresh in Miniflare when your Workers script changes during local development.","url":"https://developers.cloudflare.com/workers/testing/miniflare/developing/live-reload/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-23","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
