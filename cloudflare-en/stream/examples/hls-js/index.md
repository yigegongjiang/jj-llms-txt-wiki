---
description: Example of video playback with Cloudflare Stream and the HLS reference player (hls.js)
title: hls.js
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/stream/llms.txt  
> Use this file to discover all available pages before exploring further.

# hls.js

Example of video playback with Cloudflare Stream and the HLS reference player (hls.js)

Last updated Apr 21, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/stream/examples/hls-js/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

```html
<html>
	<head>
		<script src="//cdn.jsdelivr.net/npm/hls.js@latest"></script>
	</head>
	<body>
		<video id="video"></video>
		<script>
			if (Hls.isSupported()) {
				const video = document.getElementById('video');
				const hls = new Hls();
				hls.attachMedia(video);
				hls.on(Hls.Events.MEDIA_ATTACHED, () => {
					hls.loadSource(
						'https://customer-f33zs165nr7gyfy4.cloudflarestream.com/6b9e68b07dfee8cc2d116e4c51d6a957/manifest/video.m3u8'
					);
				});
			}

			video.play();
		</script>
	</body>
</html>
```

Refer to the [hls.js documentation ↗](https://github.com/video-dev/hls.js/blob/master/docs/API.md) for more information.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.DMYpXs3t.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/stream/examples/hls-js/#page","headline":"hls.js · Cloudflare Stream docs","description":"Example of video playback with Cloudflare Stream and the HLS reference player (hls.js)","url":"https://developers.cloudflare.com/stream/examples/hls-js/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-21","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"},"keywords":["Playback"]}
```
