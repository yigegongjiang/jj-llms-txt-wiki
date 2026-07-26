---
description: Write and format code blocks correctly.
title: Code block guidelines
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Code block guidelines

Last updated Jun 18, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/formatting/code-block-guidelines/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

To create a code block:

* Use triple-grave characters (```` ``` ````) as a fence, and enter a [language](#languages) name after the first ```` ``` ```` fence
* Indent lines by four spaces or one tab

[Learn about conventions for code blocks](https://developers.cloudflare.com/style-guide/formatting/code-conventions-and-format/)

[Learn about code block special formatting and functionality](#add-special-formatting)

Here is an example of a JSON code block:

```plaintext
```json
{
	"firstName": "John",
	"lastName": "Smith",
	"age": 25
}
```
```

The rendered output looks like this:

```json
{
	"firstName": "John",
	"lastName": "Smith",
	"age": 25
}
```

### Add output

To add the output of your code block, create a second code block below the first and add the `output` property to the opening code fence, like this:

```sh
npx wrangler vectorize create tutorial-index --dimensions=3 --metric=cosine
```

```txt
✅ Successfully created index 'tutorial-index'

[[vectorize]]
binding = "VECTORIZE_INDEX" # available in your Worker on env.VECTORIZE_INDEX
index_name = "tutorial-index"
```

```mdx
```sh
npx wrangler vectorize create tutorial-index --dimensions=3 --metric=cosine
```

```txt output
✅ Successfully created index 'tutorial-index'

[[vectorize]]
binding = "VECTORIZE_INDEX" # available in your Worker on env.VECTORIZE_INDEX
index_name = "tutorial-index"
```
```

## Languages

To define the language of your code block, enter the name of the language after the first ```` ``` ```` fence.

Language names must be lowercase. For example, use `javascript`, not `JavaScript`.

Use `txt` (aliases: `text`, `plaintext`) when there is no appropriate syntax language.

### Terminal commands

* Use the `sh` or `bash` language for commands executed in the Linux/macOS terminal, including:

  * One-line commands
  * Commands that span multiple lines (usually each line ends with a `\`)
  * Commands for specific shells (for example, a command specifically for the `zsh` shell)
* Use the `powershell` language for Windows PowerShell commands. When rendered, these blocks will have a `PowerShell` title.
* Use the `txt` language for Windows console commands.

The **Copy to clipboard** button, available in the top-right corner of each code block, will copy the entire content of the code block, including any command output included in the block.

Do not include a prefix (`$`, `%`, `PS>`, `C:\>`, or similar) before a command so that the user can run the command immediately after copying and pasting without having to remove the prefix. Similarly, do not write the folder where the command is being executed unless it is an essential part of the explanation.

### JSON

Use `json` for JSON code blocks or JSON fragments.

Multi-line curl commands with a JSON body should use the `sh` or `bash` syntax highlighting, as stated in [Terminal commands](#terminal-commands).

:::note JSON fragments may appear with a red background in GitHub because they are not valid JSON. Make it clear in the documentation that it is a fragment and not an entire piece of valid JSON content. :::

## Add special formatting

You can add special formatting to code blocks, such as collapsed sections, line numbers, and highlighting. Here is a showcase of some of the functionality. You can find more options at [Expressive Code ↗](https://expressive-code.com/), a project by Astro.

```mdx
```powershell title="Write string example"
Write-Output "This one has a title"
```

```js collapse={3-5}
// Collapsing
const foo = {
	1: 1,
	2: 2,
	3: 3,
};
```

```js showLineNumbers
// Line numbers
const foo = "bar";
const bar = "baz";
```

```js wrap
// Example with wrap
function getLongString() {
	return "This is a very long string that will most probably not fit into the available space unless the container is extremely wide";
}
```

```js "return true;" ins="inserted" del="deleted"
function demo() {
	console.log("These are inserted and deleted marker types");
	// The return statement uses the default marker type
	return true;
}
```

```diff lang="js"
  function thisIsJavaScript() {
    // This entire block gets highlighted as JavaScript,
    // and we can still add diff markers to it!
-   console.log('Old code to be removed')
+   console.log('New and shiny code!')
  }
```
```

:::caution Do not use the `$` sign in your code blocks before a command. :::

## Workers Playground

If you add the `playground` option to the opening code fence for a Worker example, it will add a "Run Worker in Playground" link that will take the user to the [Worker's playground](https://developers.cloudflare.com/workers/playground/).

### Live demo

```js
export default {
	fetch() {
		return new Response("Test!");
	},
};
```

### How to use

```mdx
```js playground
export default {
	fetch() {
		return new Response("Test!");
	},
};
```
```

## GraphQL API Explorer

Add `graphql-api-explorer` to the opening code fence to create a `graphql` code block with a **Run in GraphQL API Explorer** button that leads to [GraphQL API Explorer ↗](https://graphql.cloudflare.com/explorer).

:::note This button only works if the person selecting it is logged in or has an API token saved. :::

```mdx
```graphql graphql-api-explorer title="A GraphQL query"
query ASingleDatasetExample($zoneTag: string, $start: Time, $end: Time) {
	viewer {
		zones(filter: { zoneTag: $zoneTag }) {
			firewallEventsAdaptive(
				filter: { datetime_gt: $start, datetime_lt: $end }
				limit: 2
				orderBy: [datetime_DESC]
			) {
				action
				datetime
				host: clientRequestHTTPHost
			}
		}
	}
}
```
```

### Variables

In the GraphQL API Explorer, the **Variables** section is automatically filled based on the names and types of the variables defined in your query:

* Variables that include `start` and are of type `Time` are set to six hours before the current time
* Variables that include `end` and are of type `Time` are set to the current time
* Variables that include `start` and are of type `Date` are set to 24 hours before the current date
* Variables that include `end` and are of type `Date` are set to the current date
* Variables that include `zoneTag` and are of type `string` are set to "ZONE\_ID"
* Variables that include `accountTag` and are of type `string` are set to "ACCOUNT\_ID"
* Variables that include `id` and are of type `string` are set to "REPLACE\_WITH\_ID"
* Variables that include `limit` and are of type `int` are set to 100
* Any other variable with a type of `string` is set to "REPLACE\_WITH\_STRING"

You can also add custom variables by setting their values as a JSON string in the `graphql-api-explorer` metadata. The custom variables will be merged with the automatically populated variables.

In the following example, the custom value is `custom-variable`:

```mdx
```graphql graphql-api-explorer='{"uID": "custom-variable"}' title="A GraphQL query"
query GraphqlExample($zoneTag: string, $start: Time, $end: Time) {
 viewer {
   zones(filter: { zoneTag: $zoneTag }) {
     ...
   }
 }
}
```
```

So, the **Variables** would look something like this:

```txt
{"zoneTag":"ZONE_ID", "start":"2025-09-11T14:00:00Z", "end":"2025-09-11T20:00:00Z", "uId":"custom-variable"}
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.DMYpXs3t.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/formatting/code-block-guidelines/#page","headline":"Code block guidelines · Cloudflare Style Guide","description":"Write and format code blocks correctly.","url":"https://developers.cloudflare.com/style-guide/formatting/code-block-guidelines/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-06-18","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
