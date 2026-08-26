---
description: Import ES Modules, WebAssembly, text, and binary files in Pages Functions.
title: Module support
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/pages/llms.txt  
> Use this file to discover all available pages before exploring further.

# Module support

Last updated Apr 21, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/pages/functions/module-support/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

Pages Functions provide support for several module types, much like [Workers ↗](https://blog.cloudflare.com/workers-javascript-modules/). This means that you can import and use external modules such as WebAssembly (Wasm), `text` and `binary` files inside your Functions code.

This guide will instruct you on how to use these different module types inside your Pages Functions.

## ECMAScript Modules

ECMAScript modules (or in short ES Modules) is the official, [standardized ↗](https://tc39.es/ecma262/#sec-modules) module system for JavaScript. It is the recommended mechanism for writing modular and reusable JavaScript code.

[ES Modules ↗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Modules) are defined by the use of `import` and `export` statements. Below is an example of a script written in ES Modules format, and a Pages Function that imports that module:

```js
export function greeting(name: string): string {
  return `Hello ${name}!`;
}
```

```js
import { greeting } from "../src/greeting.ts";

export async function onRequest(context) {
	return new Response(`${greeting("Pages Functions")}`);
}
```

## WebAssembly Modules

[WebAssembly](https://developers.cloudflare.com/workers/runtime-apis/webassembly/) (abbreviated Wasm) allows you to compile languages like Rust, Go, or C to a binary format that can run in a wide variety of environments, including web browsers, Cloudflare Workers, Cloudflare Pages Functions, and other WebAssembly runtimes.

The distributable, loadable, and executable unit of code in WebAssembly is called a [module ↗](https://webassembly.github.io/spec/core/syntax/modules.html).

Below is a basic example of how you can import Wasm Modules inside your Pages Functions code:

```js
import addModule from "add.wasm";

export async function onRequest() {
	const addInstance = await WebAssembly.instantiate(addModule);
	return new Response(
		`The meaning of life is ${addInstance.exports.add(20, 1)}`,
	);
}
```

## Text Modules

Text Modules are a non-standardized means of importing resources such as HTML files as a `String`.

To import the below HTML file into your Pages Functions code:

```html
<!DOCTYPE html>
<html>
	<body>
		<h1>Hello Pages Functions!</h1>
	</body>
</html>
```

Use the following script:

```js
import html from "../index.html";

export async function onRequest() {
	return new Response(html, {
		headers: { "Content-Type": "text/html" },
	});
}
```

## Binary Modules

Binary Modules are a non-standardized way of importing binary data such as images as an `ArrayBuffer`.

Below is a basic example of how you can import the data from a binary file inside your Pages Functions code:

```js
import data from "../my-data.bin";

export async function onRequest() {
	return new Response(data, {
		headers: { "Content-Type": "application/octet-stream" },
	});
}
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/pages/functions/module-support/#page","headline":"Module support · Cloudflare Pages docs","description":"Import ES Modules, WebAssembly, text, and binary files in Pages Functions.","url":"https://developers.cloudflare.com/pages/functions/module-support/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-21","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
