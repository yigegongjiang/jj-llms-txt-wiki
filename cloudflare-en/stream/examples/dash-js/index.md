---
description: Example of video playback with Cloudflare Stream and the DASH reference player (dash.js)
title: dash.js
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/stream/llms.txt  
> Use this file to discover all available pages before exploring further.

# dash.js

Example of video playback with Cloudflare Stream and the DASH reference player (dash.js)

Last updated Apr 21, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/stream/examples/dash-js/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

```html
<html>
	<head>
		<script src="https://cdn.dashjs.org/latest/dash.all.min.js"></script>
	</head>
	<body>
		<div>
			<div class="code">
				<video
					data-dashjs-player=""
					autoplay=""
					src="https://customer-f33zs165nr7gyfy4.cloudflarestream.com/6b9e68b07dfee8cc2d116e4c51d6a957/manifest/video.mpd"
					controls="true"
				></video>
			</div>
		</div>
	</body>
</html>
```

Refer to the [dash.js documentation ↗](https://github.com/Dash-Industry-Forum/dash.js/) for more information.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/stream/examples/dash-js/#page","headline":"dash.js · Cloudflare Stream docs","description":"Example of video playback with Cloudflare Stream and the DASH reference player (dash.js)","url":"https://developers.cloudflare.com/stream/examples/dash-js/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-21","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"},"keywords":["Playback"]}
```
