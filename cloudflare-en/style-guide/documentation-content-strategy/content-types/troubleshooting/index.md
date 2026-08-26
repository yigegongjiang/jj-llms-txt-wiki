---
description: Write troubleshooting documentation that pairs the symptoms a reader sees with the causes and the steps that resolve them.
title: Troubleshooting
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Troubleshooting

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/troubleshooting/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

A troubleshooting page pairs each failure a reader hits with its cause and the steps that resolve it, organized by symptom rather than by question. The tone is guiding, straightforward, and solution-oriented.

## When to use it

Troubleshooting lives in two places: inline as a "What if ..." callout or accordion on the page where the failure happens, and on a dedicated troubleshooting page per product area once the inline entries pass roughly five or the same failure spans several pages. Reach for it to help a reader recover from a failure. It is not:

* **A how-to.** A how-to pursues a goal, whereas troubleshooting recovers from a failure.
* **An error reference.** A complete per-code catalog is reference material. This page covers symptoms, multi-cause problems, and the "it is slow or flaky" cases that error codes do not capture.
* **A global FAQ.** A troubleshooting page is organized by failure, not by question.

For the full comparison, refer to [Content types](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/).

## Title & description

* **Page title**: "Troubleshooting" followed by the product, feature, or area, as in "Troubleshooting delivery".
* **Entry titles**: the verbatim symptom, ideally the exact error message, because "Error: signature timestamp outside tolerance" beats "Signature problems" in search and in a sidebar scan. Trim a long message to its distinctive substring of roughly 70 characters with an ASCII "..." marking the cut, and keep the message type such as "Error:". A symptom with no message gets observable phrasing, as in "Deliveries succeed but arrive twice".
* **Description**: state that the page fixes the common failures in the area, by symptom.

## Scaffold this page

Use the Nimbus troubleshooting recipe to generate this page. Your coding agent pulls the full page skeleton and self-review checklist, then adapts them to your product:

npmyarnpnpm

```
npx @cloudflare/nimbus-docs add content-troubleshooting
```

```
yarn @cloudflare/nimbus-docs add content-troubleshooting
```

```
pnpm @cloudflare/nimbus-docs add content-troubleshooting
```

Adapt the frontmatter the recipe emits to Cloudflare's schema: set [pcx\_content\_type](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype) and `products` instead of the generic fields the recipe emits, such as `type`.

## Component guidance

* **Fenced code blocks** carry the verbatim error message, the match target for search, readers, and retrieval. Show one realistic concrete instance with real values, and never paraphrase the message or elide its distinctive part.
* **Bold Cause, Fix, and Verify labels** give every entry the same internal order, so a panicking reader can skip straight to the fix.
* [**Details**](https://developers.cloudflare.com/style-guide/build-the-page/components/details/) accordions fit the inline form at a feature page's end, but on a dedicated page the entries stay open, because a hidden symptom is unfindable.
* **What does not fit:** Cards, a marketing tone, and reassurance offered without a fix.

## Frontmatter

```yaml
pcx_content_type: troubleshooting
products:
  - product-a
  - product-b
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## Organizing entries

Order entries by frequency, most common failure first, and let a data-loss failure jump the queue. Keep troubleshooting inline until a product area passes roughly five entries, then move it to a dedicated page and leave a link behind at the inline site. Every workaround states its cost and names its permanent alternative. A dedicated page ends with a "Still stuck?" section that gives the escalation path and a collect-this-first list, which is the type's honesty clause: a page implying completeness strands the reader with the one failure it missed.

## Writing for AI and agents

* **Self-contained entries.** Write each entry so it stands alone, because entry N is retrieved without the entry before it and without the page intro. Give it the full symptom, cause, and fix.
* **Executable fixes.** Structure cause and fix as declaratives an agent can run: "check your configuration" is not a fix, a concrete command such as `hookline test-event --endpoint <id>` is.
* **Verbatim symptoms.** Show the error message verbatim in a fenced block as one concrete instance, so search, readers, and retrieval all match on the literal text.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/troubleshooting/#page","headline":"Troubleshooting · Cloudflare Style Guide","description":"Write troubleshooting documentation that pairs the symptoms a reader sees with the causes and the steps that resolve them.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/troubleshooting/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
