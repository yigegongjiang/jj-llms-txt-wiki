---
description: Add AI Search to your website using pre-built, customizable web components for search and chat.
title: UI snippets
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/ai-search/llms.txt  
> Use this file to discover all available pages before exploring further.

# UI snippets

Last updated Aug 6, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/ai-search/configuration/retrieval/public-endpoint/embed-search-snippets/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

You can add AI Search easily into your website using the [Cloudflare AI Search UI snippet library ↗](https://search.ai.cloudflare.com/), which provides production-ready, customizable web components.

The library is open source at [github.com/cloudflare/ai-search-snippet ↗](https://github.com/cloudflare/ai-search-snippet).

## Available components

The snippet library provides four web components. Each component connects to your AI Search instance using the `api-url` attribute, which should point to your public endpoint URL.

| Component              | Description                                                 |
| ---------------------- | ----------------------------------------------------------- |
| <search-bar-snippet>   | An inline search bar that displays results in a dropdown    |
| <search-modal-snippet> | A search modal that opens with Cmd/Ctrl+K keyboard shortcut |
| <chat-bubble-snippet>  | A floating chat bubble in the corner of the page            |
| <chat-page-snippet>    | A full-page chat interface with conversation history        |

For advanced styling and configuration, visit [search.ai.cloudflare.com ↗](https://search.ai.cloudflare.com/).

## Prerequisites

UI snippets connect to your AI Search instance through a public endpoint. You need to enable this endpoint before using the snippets.

1. Go to **AI Search** in the Cloudflare dashboard.  
[Go to **AI Search** ↗](https://dash.cloudflare.com/?to=/:account/ai/ai-search)
2. Select your AI Search instance.
3. Go to **Settings** \> **Public Endpoint**.
4. Turn on **Enable Public Endpoint**.
5. Copy the public endpoint URL. You will use this as the `api-url` attribute in your snippets.

## Use with HTML

1. Add the script tag to your HTML file (for example, `index.html`). Replace `<PUBLIC_ENDPOINT_ID>` with your AI Search instance's public endpoint ID, which you can find in your AI Search instance's **Settings** \> **Public Endpoint**.  
```html  
<script  
	type="module"  
	src="https://<PUBLIC_ENDPOINT_ID>.search.ai.cloudflare.com/assets/v0.0.25/search-snippet.es.js"  
></script>  
```
2. Add a component with your `api-url`.  
```html  
<search-bar-snippet  
	api-url="https://<PUBLIC_ENDPOINT_ID>.search.ai.cloudflare.com/"  
	placeholder="Search..."  
></search-bar-snippet>  
```
3. Before testing, [configure CORS](#configure-cors-for-local-testing) to allow your local origin. Then open the HTML file in your browser to test.

### Full HTML example

The following example shows a complete HTML page with a search bar. When a user types in the search bar, results appear in a dropdown below.

```html
<!doctype html>
<html>
	<head>
		<script
			type="module"
			src="https://<PUBLIC_ENDPOINT_ID>.search.ai.cloudflare.com/assets/v0.0.25/search-snippet.es.js"
		></script>
	</head>
	<body>
		<search-bar-snippet
			api-url="https://<PUBLIC_ENDPOINT_ID>.search.ai.cloudflare.com/"
			placeholder="Search..."
			max-results="10"
		></search-bar-snippet>
	</body>
</html>
```

## Use with a framework

1. Open your React project and install the package:  
```bash  
npm install @cloudflare/ai-search-snippet  
```
2. In your component file (for example, `src/App.tsx`), import the package:  
```tsx  
import "@cloudflare/ai-search-snippet";  
```
3. Add a component to your JSX:  
```tsx  
export default function App() {  
	return (  
		<search-bar-snippet  
			api-url="https://<PUBLIC_ENDPOINT_ID>.search.ai.cloudflare.com/"  
			placeholder="Search..."  
		/>  
	);  
}  
```
4. Before testing, [configure CORS](#configure-cors-for-local-testing) to allow your local origin. Then run your development server:  
```bash  
npm run dev  
```

The package includes TypeScript types and works with React, Next.js, and other React frameworks.

1. Open your Vue project and install the package:  
```bash  
npm install @cloudflare/ai-search-snippet  
```
2. In your component file (for example, `src/App.vue`), import the package and add the component:  
```vue  
<template>  
	<search-bar-snippet :api-url="apiUrl" placeholder="Search..." />  
</template>  
<script setup>  
import "@cloudflare/ai-search-snippet";  
const apiUrl = "https://<PUBLIC_ENDPOINT_ID>.search.ai.cloudflare.com/";  
</script>  
```
3. Before testing, [configure CORS](#configure-cors-for-local-testing) to allow your local origin. Then run your development server:  
```bash  
npm run dev  
```

## Configure a component

Each component accepts attributes that control its behavior. Common attributes include:

| Attribute     | Description                                                 |
| ------------- | ----------------------------------------------------------- |
| api-url       | Required. Your instance's public endpoint URL.              |
| placeholder   | Placeholder text for the input.                             |
| max-results   | Maximum number of results to request.                       |
| theme         | light, dark, or auto (default) to follow the system theme.  |
| hide-branding | Hide the Cloudflare branding.                               |
| translations  | Override the user-facing strings to localize the component. |

The chat components (`<chat-bubble-snippet>` and `<chat-page-snippet>`) also accept `chat-query-rewrite` to rewrite follow-up messages into standalone queries.

For the complete list of attributes and a live editor that generates the HTML, React, or Vue code for you, use the [snippet playground ↗](https://search.ai.cloudflare.com/).

## Customize the appearance

Style the components with CSS custom properties, all prefixed with `--search-snippet-`. Set them on the component or a parent element:

```css
search-bar-snippet {
	--search-snippet-primary-color: #f6821f;
	--search-snippet-border-radius: 12px;
}
```

The [playground ↗](https://search.ai.cloudflare.com/) lists every available variable and previews your changes live.

## Configure CORS for local testing

When testing locally (for example, `http://localhost:3000`), you need to allow your local origin in the public endpoint settings.

1. Go to **AI Search** in the Cloudflare dashboard.  
[Go to **AI Search** ↗](https://dash.cloudflare.com/?to=/:account/ai/ai-search)
2. Select your AI Search instance.
3. Go to **Settings** \> **Public Endpoint**.
4. Under **Authorized hosts**, add your local URL (for example, `http://localhost:3000`) or `*` to allow all origins during testing.
5. Select **Save**.

When you deploy, replace `*` with your production origin so that other sites cannot embed your search components. Allowed origins are a browser control, not an access control, so this does not stop a direct request from `curl` or a script. To restrict who can query the endpoint, refer to [Cloudflare Access](https://developers.cloudflare.com/ai-search/configuration/retrieval/public-endpoint/cloudflare-access/).

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/ai-search/configuration/retrieval/public-endpoint/embed-search-snippets/#page","headline":"UI snippets · Cloudflare AI Search docs","description":"Add AI Search to your website using pre-built, customizable web components for search and chat.","url":"https://developers.cloudflare.com/ai-search/configuration/retrieval/public-endpoint/embed-search-snippets/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-06","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
