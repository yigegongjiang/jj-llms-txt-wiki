---
description: Access Analytics Engine data from within a Worker.
title: Querying from a Worker
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/analytics/llms.txt  
> Use this file to discover all available pages before exploring further.

# Querying from a Worker

Last updated Apr 23, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/analytics/analytics-engine/worker-querying/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

If you want to access Analytics Engine data from within a Worker you can use `fetch` to access the SQL API. The API can return JSON data that is easy to interact with in JavaScript.

## Authentication

In order that your Worker can authenticate with the API you will need your account ID and an API token.

* Your 32 character account ID can be obtained from the Cloudflare dashboard.
* An API token can also be generated in the dashboard. Refer to the [SQL API docs](https://developers.cloudflare.com/analytics/analytics-engine/sql-api/#authentication) for more information on this.

We recommend storing the account ID as an environment variable and the API token as a secret in your worker. This can be done through the dashboard or through Wrangler. Refer to the [Workers documentation](https://developers.cloudflare.com/workers/configuration/environment-variables/) for more details on this.

## Querying

Use the JavaScript `fetch` API as follows to execute a query:

```js
const query = "SELECT * FROM my_dataset";
const API = `https://api.cloudflare.com/client/v4/accounts/${env.ACCOUNT_ID}/analytics_engine/sql`;
const response = await fetch(API, {
	method: "POST",
	headers: {
		Authorization: `Bearer ${env.API_TOKEN}`,
	},
	body: query,
});
const responseJSON = await response.json();
```

The data will be returned in the format described in the [FORMAT](https://developers.cloudflare.com/analytics/analytics-engine/sql-reference/statements/#json) section of the documentation, allowing you to extract meta information about the names and types of returned columns in addition to the data itself and a row count.

## Example Worker

The following is a sample Worker which executes a query against a dataset of weather readings and displays minimum and maximum values for each city.

### Environment variable setup

First the environment variables are set up with the account ID and API token.

The account ID is set in the [Wrangler configuration file](https://developers.cloudflare.com/workers/wrangler/configuration/):

```jsonc
{
	"vars": {
		"ACCOUNT_ID": "<account_id>"
	}
}
```

```toml
[vars]
ACCOUNT_ID = "<account_id>"
```

The API\_TOKEN can be set as a secret, using the wrangler command line tool, by running the following and entering your token string:

```sh
npx wrangler secret put API_TOKEN
```

### Worker script

The worker script itself executes a query and formats the result:

```js
export default {
	async fetch(request, env) {
		// This worker only responds to requests at the root.
		if (new URL(request.url).pathname != "/") {
			return new Response("Not found", { status: 404 });
		}

		// SQL string to be executed.
		const query = `
            SELECT
                blob1 AS city,
                max(double1) as max_temp,
                min(double1) as min_temp
            FROM weather
            WHERE timestamp > NOW() - INTERVAL '1' DAY
            GROUP BY city
            ORDER BY city`;

		// Build the API endpoint URL and make a POST request with the query string
		const API = `https://api.cloudflare.com/client/v4/accounts/${env.ACCOUNT_ID}/analytics_engine/sql`;
		const queryResponse = await fetch(API, {
			method: "POST",
			headers: {
				Authorization: `Bearer ${env.API_TOKEN}`,
			},
			body: query,
		});

		// The API will return a 200 status code if the query succeeded.
		// In case of failure we log the error message and return a failure message.
		if (queryResponse.status != 200) {
			console.error("Error querying:", await queryResponse.text());
			return new Response("An error occurred!", { status: 500 });
		}

		// Read the JSON data from the query response and render the data as HTML.
		const queryJSON = await queryResponse.json();
		return new Response(renderResponse(queryJSON.data), {
			headers: { "content-type": "text/html" },
		});
	},
};

// renderCity renders a table row as HTML from a data row.
function renderCity(row) {
	return `<tr><td>${row.city}</td><td>${row.min_temp}</td><td>${row.max_temp}</td></tr>`;
}

// renderResponse renders a simple HTML table of results.
function renderResponse(data) {
	return `<!DOCTYPE html>
<html>
    <body>
        <table>
            <tr><th>City</th><th>Min Temp</th><th>Max Temp</th></tr>
            ${data.map(renderCity).join("\n")}
        </table>
    </body>
<html>`;
}
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/analytics/analytics-engine/worker-querying/#page","headline":"Querying Workers Analytics Engine from a Worker · Cloudflare Analytics docs","description":"Access Analytics Engine data from within a Worker.","url":"https://developers.cloudflare.com/analytics/analytics-engine/worker-querying/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-23","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
