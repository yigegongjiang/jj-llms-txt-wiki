---
description: Serve a custom maintenance page instead of fetching content from the origin server or cache. Ideal for downtime notifications, planned maintenance, or emergency messages.
title: Maintenance page
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/rules/llms.txt  
> Use this file to discover all available pages before exploring further.

# Maintenance page

Serve a custom maintenance page. Ideal for downtime notifications, planned maintenance, or emergency messages.

Last updated Oct 13, 2025|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/rules/snippets/examples/maintenance/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

## Snippet code

```js
// Define your customizable inputs
const statusCode = 503;
const title = "We'll Be Right Back!";
const message =
	"Our site is currently undergoing scheduled maintenance. We’re working hard to bring you a better experience. Thank you for your patience and understanding.";
const estimatedTime = "1 hour";
const contactEmail = "support@example.com";
const contactPhone = "+1 234 567 89";

export default {
	async fetch(request) {
		// Serve the maintenance page as a response
		return new Response(generateMaintenancePage(), {
			status: statusCode,
			headers: {
				"Content-Type": "text/html",
				"Retry-After": "3600", // Suggest retry after 1 hour
			},
		});
	},
};

function generateMaintenancePage() {
	return `
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>${title}</title>
        <style>
            body {
                margin: 0;
                font-family: Arial, sans-serif;
                display: flex;
                align-items: center;
                justify-content: center;
                height: 100vh;
                background-color: #f4f4f4;
                color: #333;
                text-align: center;
            }
            .container {
                max-width: 600px;
                padding: 20px;
            }
            h1 {
                font-size: 2rem;
                color: #0056b3;
                margin-bottom: 10px;
            }
            p {
                font-size: 1rem;
                margin-bottom: 20px;
                line-height: 1.5;
            }
            .contact {
                margin-top: 20px;
                font-size: 0.9rem;
                color: #666;
            }
            .contact a {
                color: #0056b3;
                text-decoration: none;
            }
            .contact a:hover {
                text-decoration: underline;
            }
            .logo {
                margin: 20px 0;
                max-width: 150px;
            }
            .timer {
                font-weight: bold;
                color: #e63946;
            }
        </style>
    </head>
    <body>
        <div class="container">
            <h1>${title}</h1>
            <p>${message}</p>
            <p>If all goes to plan, we'll be back online in <span class="timer">${estimatedTime}</span>. 🚀</p>
            <p class="contact">
                Need help? Reach out to us at <a href="mailto:${contactEmail}">${contactEmail}</a>
                or call us at <a href="tel:${contactPhone}">${contactPhone}</a>.
            </p>
        </div>
    </body>
    </html>
    `;
}
```

## Snippet rule

Configure a custom filter expression:

| Field             | Operator       | Value      |
| ----------------- | -------------- | ---------- |
| IP Source Address | is not in list | admin\_ips |

If you are using the Expression Editor, enter the following expression:

```txt
(not ip.src in $admin_ips)
```

The [IP list](https://developers.cloudflare.com/waf/tools/lists/custom-lists/#ip-lists) `admin_ips` was previously created and contains the list of IP addresses of the site administrators, which will be able to access the site during the maintenance period.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.DMYpXs3t.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/rules/snippets/examples/maintenance/#page","headline":"Maintenance page · Cloudflare Rules docs","description":"Serve a custom maintenance page instead of fetching content from the origin server or cache. Ideal for downtime notifications, planned maintenance, or emergency messages.","url":"https://developers.cloudflare.com/rules/snippets/examples/maintenance/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2025-10-13","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"},"keywords":["Redirects"]}
```
