---
description: Allow a client to request static assets while waiting for the HTML response.
title: 103 Early Hints
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/workers/llms.txt  
> Use this file to discover all available pages before exploring further.

# 103 Early Hints

Allow a client to request static assets while waiting for the HTML response.

Last updated Apr 23, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/workers/examples/103-early-hints/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

If you want to get started quickly, click on the button below.

[![Deploy to Cloudflare](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/cloudflare/docs-examples/tree/main/workers/103-early-hints)

This creates a repository in your GitHub account and deploys the application to Cloudflare Workers.

`103` Early Hints is an HTTP status code designed to speed up content delivery. When enabled, Cloudflare can cache the `Link` headers marked with preload and/or preconnect from HTML pages and serve them in a `103` Early Hints response before reaching the origin server. Browsers can use these hints to fetch linked assets while waiting for the origin’s final response, dramatically improving page load speeds.

To ensure Early Hints are enabled on your zone:

1. In the Cloudflare dashboard, go to the **Speed settings** page.  
[Go to **Settings** ↗](https://dash.cloudflare.com/?to=/:account/:zone/speed/optimization)
2. Go to **Content Optimization**.
3. Enable the **Early Hints** toggle to on.

You can return `Link` headers from a Worker running on your zone to speed up your page load times.

```js
const CSS = "body { color: red; }";
const HTML = `
<!doctype html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>Early Hints test</title>
    <link rel="stylesheet" href="https://developers.cloudflare.com/test.css">
</head>
<body>
    <h1>Early Hints test page</h1>
</body>
</html>
`;

export default {
	async fetch(req) {
		// If request is for test.css, serve the raw CSS
		if (/test\.css$/.test(req.url)) {
			return new Response(CSS, {
				headers: {
					"content-type": "text/css",
				},
			});
		} else {
			// Serve raw HTML using Early Hints for the CSS file
			return new Response(HTML, {
				headers: {
					"content-type": "text/html",
					link: "</test.css>; rel=preload; as=style",
				},
			});
		}
	},
};
```

```js
const CSS = "body { color: red; }";
const HTML = `
<!doctype html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>Early Hints test</title>
    <link rel="stylesheet" href="https://developers.cloudflare.com/test.css">
</head>
<body>
    <h1>Early Hints test page</h1>
</body>
</html>
`;

export default {
  async fetch(req): Promise<Response> {
    // If request is for test.css, serve the raw CSS
    if (/test\.css$/.test(req.url)) {
      return new Response(CSS, {
        headers: {
          "content-type": "text/css",
        },
      });
    } else {
      // Serve raw HTML using Early Hints for the CSS file
      return new Response(HTML, {
        headers: {
          "content-type": "text/html",
          link: "</test.css>; rel=preload; as=style",
        },
      });
    }
  },
} satisfies ExportedHandler;
```

```py
import re
from workers import Response, WorkerEntrypoint

CSS = "body { color: red; }"
HTML = """
<!doctype html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>Early Hints test</title>
    <link rel="stylesheet" href="https://developers.cloudflare.com/test.css">
</head>
<body>
    <h1>Early Hints test page</h1>
</body>
</html>
"""

class Default(WorkerEntrypoint):
    async def fetch(self, request):
        if re.search("test.css", request.url):
            headers = {"content-type": "text/css"}
            return Response(CSS, headers=headers)
        else:
            headers = {"content-type": "text/html","link": "</test.css>; rel=preload; as=style"}
        return Response(HTML, headers=headers)
```

```ts
import { Hono } from "hono";

const app = new Hono();

const CSS = "body { color: red; }";
const HTML = `
<!doctype html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>Early Hints test</title>
    <link rel="stylesheet" href="https://developers.cloudflare.com/test.css">
</head>
<body>
    <h1>Early Hints test page</h1>
</body>
</html>
`;

// Serve CSS file
app.get("/test.css", (c) => {
	return c.body(CSS, {
		headers: {
			"content-type": "text/css",
		},
	});
});

// Serve HTML with early hints
app.get("*", (c) => {
	return c.html(HTML, {
		headers: {
			link: "</test.css>; rel=preload; as=style",
		},
	});
});

export default app;
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/workers/examples/103-early-hints/#page","headline":"103 Early Hints · Cloudflare Workers docs","description":"Allow a client to request static assets while waiting for the HTML response.","url":"https://developers.cloudflare.com/workers/examples/103-early-hints/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-23","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"},"keywords":["Middleware","Headers","JavaScript","TypeScript","Python"]}
```
