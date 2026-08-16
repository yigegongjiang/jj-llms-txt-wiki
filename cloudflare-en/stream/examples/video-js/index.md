---
description: Example of video playback with Cloudflare Stream and Video.js
title: Video.js
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/stream/llms.txt  
> Use this file to discover all available pages before exploring further.

# Video.js

Example of video playback with Cloudflare Stream and Video.js

Last updated Apr 21, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/stream/examples/video-js/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

```html
<html>
	<head>
		<link
			href="https://cdnjs.cloudflare.com/ajax/libs/video.js/7.10.2/video-js.min.css"
			rel="stylesheet"
		/>
		<script src="https://cdnjs.cloudflare.com/ajax/libs/video.js/7.10.2/video.min.js"></script>
	</head>
	<body>
		<video-js id="vid1" controls preload="auto">
			<source
				src="https://customer-f33zs165nr7gyfy4.cloudflarestream.com/6b9e68b07dfee8cc2d116e4c51d6a957/manifest/video.m3u8"
				type="application/x-mpegURL"
			/>
		</video-js>

		<script>
			const vid = document.getElementById('vid1');
			const player = videojs(vid);
		</script>
	</body>
</html>
```

Refer to the [Video.js documentation ↗](https://docs.videojs.com/) for more information.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/stream/examples/video-js/#page","headline":"Video.js · Cloudflare Stream docs","description":"Example of video playback with Cloudflare Stream and Video.js","url":"https://developers.cloudflare.com/stream/examples/video-js/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-21","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"},"keywords":["Playback"]}
```
