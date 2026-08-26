---
description: Optimize documentation for AI consumption.
title: AI consumability
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# AI consumability

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/how-we-docs/ai-consumability/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

We have various approaches for making our content visible to AI as well as making sure it's easily consumed in a plain-text format.

## AI discoverability

The primary proposal in this space is [llms.txt ↗](https://llmstxt.org/), offering a well-known path for a Markdown list of all your pages.

We have implemented `llms.txt` and `llms-full.txt` as follows:

* [llms.txt](https://developers.cloudflare.com/llms.txt) — A directory of all Cloudflare documentation products, grouped by category. Each entry links to that product's own `llms.txt` — for example, [/workers/llms.txt](https://developers.cloudflare.com/workers/llms.txt) — which lists every page for that product in Markdown format.
* [llms-full.txt](https://developers.cloudflare.com/llms-full.txt) — The full contents of all Cloudflare documentation in a single file, intended for offline indexing, bulk vectorization, or large-context models. We also provide a `llms-full.txt` file on a per-product basis — for example, [/workers/llms-full.txt](https://developers.cloudflare.com/workers/llms-full.txt).

To obtain a Markdown version of a single documentation page, you can:

* Send a request to `/$page/index.md` — Add `/index.md` to the end of any page to get the Markdown version. For example, [/docs-for-agents/index.md](https://developers.cloudflare.com/docs-for-agents/index.md).
* Send a request to any page with an `Accept: text/markdown` header — Uses [Markdown for Agents](https://developers.cloudflare.com/fundamentals/reference/markdown-for-agents/) to convert the page to Markdown at the network layer. For example:  
```bash  
curl "https://developers.cloudflare.com/docs-for-agents/" \
  --header "Accept: text/markdown"  
```

Both methods return the same Markdown output, powered by [Markdown for Agents](https://developers.cloudflare.com/fundamentals/reference/markdown-for-agents/).

In the top right of this page, you will see a `Page options` button where you can copy the current page as Markdown that can be given to your LLM of choice.

![Page options
button](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=624,height=762,format=webp/_astro/page-options.T2MlgPLy.png)

## Textual representation of interactive elements

HTML is easily parsed - after all, the browser has to parse it to decide how to render the page you're reading now - it tends to not be very _portable_. This limitation is especially painful in an AI context, because all the extra presentation information consumes additional tokens.

For example, given our [Tabs](https://developers.cloudflare.com/style-guide/build-the-page/components/tabs/), the panels are hidden until the tab itself is clicked:

One Content

Two Content

If we run the resulting HTML from this component through a solution like [turndown ↗](https://www.npmjs.com/package/turndown):

```md
- [One](#tab-panel-6)
- [Two](#tab-panel-7)

One Content

Two Content
```

The references to the panels `id`, usually handled by JavaScript, are visible but non-functional.

The primary answer or core instruction should always appear in the main content flow, not exclusively inside a tab or collapsible section.

Use tabs for platform-specific variations (for example, Dashboard versus API versus Terraform) only after stating the general concept. Use Details for supplementary information, not for the primary answer.

### Turning our components into "Markdownable" HTML

To solve this, we use [Markdown for Agents](https://developers.cloudflare.com/fundamentals/reference/markdown-for-agents/), which converts HTML to Markdown at the Cloudflare network layer. It handles:

* Removing non-content tags (`script`, `style`, `link`, etc.)
* Transforming interactive components like `Tabs` into standard unordered lists
* Adapting code block HTML into clean Markdown fenced code blocks

Taking the `Tabs` example from the previous section, Markdown for Agents will give us a normal unordered list with the content properly associated with a given list item:

```md
- One

  One Content

- Two

  Two Content
```

You can request any page as Markdown in two ways:

* Send a request with an `Accept: text/markdown` header:  
```bash  
curl "https://developers.cloudflare.com/docs-for-agents/" \
  --header "Accept: text/markdown"  
```
* Append `index.md` to the URL — for example, [/docs-for-agents/index.md](https://developers.cloudflare.com/docs-for-agents/index.md)

### Saving on tokens

Most AI pricing is around input & output tokens and Markdown greatly reduces the amount of input tokens required.

For example, let's take a look at the amount of tokens required for the [Workers Get Started](https://developers.cloudflare.com/workers/get-started/guide/) using [OpenAI's tokenizer ↗](https://platform.openai.com/tokenizer):

* HTML: 15,229 tokens
* Markdown: 2,110 tokens (7.22x less than HTML)

When providing our content to AI, we can see a real-world \~7x saving in input tokens cost.

## Curating content

Other than the work making our content [discoverable](#ai-discoverability), most of the other work of making content for AI aligns with SEO or content best practices, such as:

* Using semantic HTML
* Adding headings
* Reducing inconsistencies in naming or outdated information

For more details, refer to [Google's AI guidance ↗](https://developers.google.com/search/docs/appearance/ai-features#seo-best-practices).

### `noindex` directives

The only _special_ work we have done is adding a [noindex directives ↗](https://developers.google.com/search/docs/crawling-indexing/block-indexing) to specific types of content (via a [frontmatter tag](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/custom-properties/#noindex)).

```html
<meta name="robots" content="noindex">
```

For example, we have certain pages that discuss deprecated features, such as [Wrangler 1](https://developers.cloudflare.com/workers/wrangler/migration/v1-to-v2/wrangler-legacy/). While technically accurate, they are no longer advisable to follow and could potentially confuse AI outputs.

At the moment, it's unclear whether all AI crawlers will respect these directives, but it's the only signal we have to exclude something from their indexing (and we do not want to set up [WAF](https://developers.cloudflare.com/waf/) rules for individual pages).

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/how-we-docs/ai-consumability/#page","headline":"AI consumability · Cloudflare Style Guide","description":"Optimize documentation for AI consumption.","url":"https://developers.cloudflare.com/style-guide/how-we-docs/ai-consumability/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
