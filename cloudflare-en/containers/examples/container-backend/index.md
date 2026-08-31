---
description: A simple frontend app with a containerized backend
title: Static Frontend, Container Backend
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Static Frontend, Container Backend

A simple frontend app with a containerized backend

Last updated Aug 28, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/containers/examples/container-backend/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

A common pattern is to serve a static frontend application (e.g., React, Vue, Svelte) using Static Assets, then pass backend requests to a containerized backend application.

In this example, we'll show an example using a simple `index.html` file served as a static asset, but you can select from one of many frontend frameworks. See our [Workers framework examples](https://developers.cloudflare.com/workers/framework-guides/web-apps/) for more information.

For a full example, see the [Static Frontend + Container Backend Template ↗](https://github.com/mikenomitch/static-frontend-container-backend).

## Configure Static Assets and a Container

```jsonc
{
  "name": "cron-container",
  "main": "src/index.ts",
  "assets": {
    "directory": "./dist",
    "binding": "ASSETS"
  },
  "containers": [
    {
      "class_name": "Backend",
      "image": "./Dockerfile",
			"max_instances": 3
    }
  ],
  "durable_objects": {
    "bindings": [
      {
        "class_name": "Backend",
        "name": "BACKEND"
      }
    ]
  },
  "migrations": [
    {
      "new_sqlite_classes": [
        "Backend"
      ],
      "tag": "v1"
    }
  ]
}
```

```toml
name = "cron-container"
main = "src/index.ts"

[assets]
directory = "./dist"
binding = "ASSETS"

[[containers]]
class_name = "Backend"
image = "./Dockerfile"
max_instances = 3

[[durable_objects.bindings]]
class_name = "Backend"
name = "BACKEND"

[[migrations]]
new_sqlite_classes = [ "Backend" ]
tag = "v1"
```

## Add a simple index.html file to serve

Create a simple `index.html` file in the `./dist` directory.

index.html

```html
<!DOCTYPE html>
<html lang="en">

<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Widgets</title>
  <script defer src="https://cdnjs.cloudflare.com/ajax/libs/alpinejs/3.13.3/cdn.min.js"></script>
</head>

<body>
  <div x-data="widgets()" x-init="fetchWidgets()">
    <h1>Widgets</h1>
    <div x-show="loading">Loading...</div>
    <div x-show="error" x-text="error" style="color: red;"></div>
    <ul x-show="!loading && !error">
      <template x-for="widget in widgets" :key="widget.id">
        <li>
          <span x-text="widget.name"></span> - (ID: <span x-text="widget.id"></span>)
        </li>
      </template>
    </ul>

    <div x-show="!loading && !error && widgets.length === 0">
      No widgets found.
    </div>

  </div>

  <script>
    function widgets() {
      return {
        widgets: [],
        loading: false,
        error: null,

        async fetchWidgets() {
          this.loading = true;
          this.error = null;

          try {
            const response = await fetch('/api/widgets');
            if (!response.ok) {
              throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }
            this.widgets = await response.json();
          } catch (err) {
            this.error = err.message;
          } finally {
            this.loading = false;
          }
        }
      }
    }
  </script>

</body>

</html>
```

In this example, we are using [Alpine.js ↗](https://alpinejs.dev/) to fetch a list of widgets from `/api/widgets`.

This is meant to be a very simple example, but you can get significantly more complex. See [examples of Workers integrating with frontend frameworks](https://developers.cloudflare.com/workers/framework-guides/web-apps/) for more information.

## Define a Worker

Your Worker needs to be able to both serve static assets and route requests to the containerized backend.

In this case, we will pass requests to one of three container instances if the route starts with `/api`, and all other requests will be served as static assets.

```javascript
import { Container, getRandom } from "@cloudflare/containers";

const INSTANCE_COUNT = 3;

class Backend extends Container {
	defaultPort = 8080; // pass requests to port 8080 in the container
	sleepAfter = "2h"; // only sleep a container if it hasn't gotten requests in 2 hours
}

export default {
	async fetch(request, env) {
		const url = new URL(request.url);
		if (url.pathname.startsWith("/api")) {
			const containerInstance = await getRandom(env.BACKEND, INSTANCE_COUNT);
			return containerInstance.fetch(request);
		}

		return env.ASSETS.fetch(request);
	},
};
```

Note

This example uses `getRandom`, which randomly selects one of a fixed number of Container instances for each request.

In the future, we will provide improved latency-aware load balancing and autoscaling.

This will make scaling stateless instances simple and routing more efficient. See the [autoscaling documentation](https://developers.cloudflare.com/containers/configuration/scaling-and-routing) for more details.

## Define a backend container

Your container should be able to handle requests to `/api/widgets`.

In this case, we'll use a simple Golang backend that returns a hard-coded list of widgets.

server.go

```go
package main

import (
	"encoding/json"
	"log"
	"net/http"
)

func handler(w http.ResponseWriter, r \*http.Request) {
	widgets := []map[string]interface{}{
		{"id": 1, "name": "Widget A"},
		{"id": 2, "name": "Sprocket B"},
		{"id": 3, "name": "Gear C"},
	}

	w.Header().Set("Content-Type", "application/json")
	w.Header().Set("Access-Control-Allow-Origin", "*")
	json.NewEncoder(w).Encode(widgets)

}

func main() {
	http.HandleFunc("/api/widgets", handler)
	log.Fatal(http.ListenAndServe(":8080", nil))
}
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/examples/container-backend/#page","headline":"Static Frontend, Container Backend · Cloudflare Containers docs","description":"A simple frontend app with a containerized backend","url":"https://developers.cloudflare.com/containers/examples/container-backend/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-28","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
