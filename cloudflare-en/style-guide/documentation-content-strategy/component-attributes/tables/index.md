---
description: Format tables consistently in documentation.
title: Tables
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Tables

Last updated Apr 24, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/tables/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

Tables make complex information easier to understand by presenting it in a clear structure.

Caution

Limit tables to three columns (or four if the information is very condensed). Otherwise, mobile users will have a hard time consuming tabular information.

## Use cases

The purpose of a table is to provide a scannable content experience.

Use tables for:

* Simple mappings of data and values
* Categories of things with examples
* Collections of things with different attributes

Each cell within a table should not contain more than **one sentence** of content.

## Usage

We use standard Markdown tables for our documentation.

### Example

| Category             | Range                                                                                  |
| -------------------- | -------------------------------------------------------------------------------------- |
| **Not computed**     | Bot scores of 0.                                                                       |
| **Automated**        | Bot scores of 1.                                                                       |
| **Likely automated** | Bot scores of 2 through 29.                                                            |
| **Likely human**     | Bot scores of 30 through 99.                                                           |
| **Verified bot**     | Non-malicious automated traffic (used to power search engines and other applications). |

```txt
| Category | Range |
| ---- | ---- |
| **Not computed** | Bot scores of 0. |
| **Automated** | Bot scores of 1. |
| **Likely automated** | Bot scores of 2 through 29. |
| **Likely human** | Bot scores of 30 through 99. |
| **Verified bot** | Non-malicious automated traffic (used to power search engines and other applications). |
```

### Guidelines

When using tables:

* Check whether the tables work for both desktop and mobile users.
* Limit tables to three columns (or four if the information is very condensed).
* Avoid long sentences or information that is so dense that it defeats the purpose of having tabular displays.
* Introduce tables with a complete sentence that describes the purpose of the table because not all screen readers preannounce tables. The introductory sentence can end with a colon or a period; usually a colon if it immediately precedes the table, and usually a period if there's more material (such as a note paragraph) between the introduction and the table.
* Do not merge cells. Merged cells break screen reader navigation and make content harder for AI systems to parse.
* Sort rows in a logical order. If no logical order exists, use alphabetical order.

### Introductory sentences

Introduce tables with a complete sentence that describes the purpose of the table because not all screen readers preannounce tables. The introductory sentence can end with a colon or a period; usually a colon if it immediately precedes the table, and usually a period if there's more material (such as a note paragraph) between the introduction and the table.

When referring to a table, use a phrase like "the following table" or "the preceding table." Do not place a table in the middle of a sentence.

### Column headings

* Use sentence case for column headings.
* Write concise headings that clearly describe the column content.
* Do not end column headings with punctuation, including periods, ellipses, or colons.
* Use headings for the first row only. Bold the first column values when they serve as row labels.

### Table placement

* Place each table directly after the sentence that introduces it.
* Do not place a table in the middle of a numbered procedure. If a step requires tabular data, place the table immediately after the step, not between two steps.
* If a table has footnotes, place them immediately after the table.

### Table captions

If a page contains only one table, it does not need a caption. Place the table adjacent to the text that refers to it.

If a page contains more than one table in close proximity, add a caption to each table so readers (and AI systems) can distinguish between them. In Markdown, place the caption as a bold line immediately before the table. In HTML, use the `caption` element as the first child of the `table` element.

Start the caption with a number in the form **Table NUMBER.** followed by a brief description. Use sentence case. Do not place a period at the end of the caption.

When referring to a captioned table from text, refer to it by number — for example, "as shown in table 2." Do not capitalize "table" unless it starts a sentence.

**Markdown example:**

```markdown
**Table 1.** Supported DNS record types

| Type | Description |
| --- | --- |
| A | Maps a domain to an IPv4 address |
| AAAA | Maps a domain to an IPv6 address |
```

**HTML example:**

```html
<table>
  <caption><b>Table 1.</b> Supported DNS record types</caption>
  <thead>
    <tr>
      <th>Type</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>A</td>
      <td>Maps a domain to an IPv4 address</td>
    </tr>
  </tbody>
</table>
```

### Alternatives

If your information does not fit within the [guidelines](#guidelines), consider using the following methods of presentation:

* Lists
* Subsections
* [Tabs](https://developers.cloudflare.com/style-guide/components/tabs)
* [Details](https://developers.cloudflare.com/style-guide/components/details/)

### Large tables

As stated in the [guidelines](#guidelines), we generally avoid large tables in our documentation.

However, if you have a unique use case, use the `{{</*table-wrap*/>}}` shortcode to make your table responsive and scrollable.

| Header 1 | Header 2 | Header 3 | Header 4 |
| -------- | -------- | -------- | -------- |
| test     | test     | test     | test     |

```txt
{{</*table-wrap*/>}}

| Header 1 | Header 2 | Header 3 | Header 4 |
| --- | --- | --- | --- |
| test | test | test | test |

{{</*/table-wrap*/>}}
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/tables/#page","headline":"Tables · Cloudflare Style Guide","description":"Format tables consistently in documentation.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/tables/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-24","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
