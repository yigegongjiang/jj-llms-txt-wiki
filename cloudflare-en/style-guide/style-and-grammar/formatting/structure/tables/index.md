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

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/style-and-grammar/formatting/structure/tables/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

Using tables to simplify content and data provides a comprehensive way to arrange design, structure, outlines, pattern, or order. It is a great tool for comparisons, breakdowns, lists, functions, and descriptions.

Caution

Try limit tables to four columns where possible, otherwise, mobile users will have a hard time consuming tabular information.

Here are some tips when creating tables:

* Label column headers.
* Label row headers if appropriate.
* Avoid merged cells. Merged cells break screen reader navigation and make content harder for AI systems to parse.
* Avoid too much text. Limit each cell to one sentence of content.
* Aim for parallelism within the column.
* Keep tables as simple and as small as possible.
* Sort rows in a logical order. If no logical order exists, use alphabetical order.
* Introduce tables with a complete sentence that describes the purpose of the table because not all screen readers preannounce tables. The introductory sentence can end with a colon or a period; usually a colon if it immediately precedes the table, and usually a period if there's more material (such as a note paragraph) between the introduction and the table.

## Introductory sentences

Introduce tables with a complete sentence that describes the purpose of the table because not all screen readers preannounce tables. The introductory sentence can end with a colon or a period; usually a colon if it immediately precedes the table, and usually a period if there's more material (such as a note paragraph) between the introduction and the table.

When referring to a table, use a phrase like "the following table" or "the preceding table." Do not place a table in the middle of a sentence.

## Column headings

* Use sentence case.
* Write concise headings that clearly describe the column content.
* Do not end column headings with punctuation, including periods, ellipses, or colons.
* Use the `th` element for column headings in HTML tables. Include the `scope` attribute for accessibility.

```html
<thead>
	<tr>
		<th scope="col">Name</th>
		<th scope="col">Description</th>
	</tr>
</thead>
```

## Table placement

* Place each table directly after the sentence that introduces it.
* Do not place a table in the middle of a numbered procedure. Place the table immediately after the relevant step.
* If a table has footnotes, place them immediately after the table.

## Table captions

If a page contains only one table, it does not need a caption. Place the table adjacent to the text that refers to it.

If a page contains more than one table in close proximity, add a caption to each table. Start the caption with a number in the form **Table NUMBER.** followed by a brief description. Use sentence case. Do not place a period at the end of the caption.

When referring to a captioned table from text, refer to it by number — for example, "as shown in table 2." Do not capitalize "table" unless it starts a sentence.

In Markdown, place the caption as a bold line immediately before the table:

```markdown
**Table 1.** Supported DNS record types

| Type | Description                      |
| ---- | -------------------------------- |
| A    | Maps a domain to an IPv4 address |
| AAAA | Maps a domain to an IPv6 address |
```

In HTML, use the `caption` element as the first child of the `table` element:

```html
<table>
	<caption>
		<b>Table 1.</b>
		Supported DNS record types
	</caption>
	<thead>
		<tr>
			<th scope="col">Type</th>
			<th scope="col">Description</th>
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

## When to use tables

The purpose of a table is to provide a scannable content experience. Tables display pieces of information that have some sort of relationship.

Use tables for:

* Simple mappings of data and values
* Categories of things with examples
* Collections of things with different attributes
* Dates and descriptions, like a changelog
* A list of products with attributes

## When not to use tables

Do not use tables to format a page.

If your information does not fit within these guidelines, consider another method of presentation:

* Lists
* Subsections
* [Tabs](https://developers.cloudflare.com/style-guide/build-the-page/components/tabs/)
* [Details](https://developers.cloudflare.com/style-guide/build-the-page/components/details/)

## Markdown examples

**Add a table**

To add a table, use three or more hyphens (---) to create each column’s header, and use pipes (|) to separate each column. For compatibility, you should also add a pipe on either end of the row.

```plaintext
| Syntax      | Description |
| ----------- | ----------- |
| Header      | Title       |
| Paragraph   | Text        |
```

The rendered output looks like this:

| Syntax    | Description |
| --------- | ----------- |
| Header    | Title       |
| Paragraph | Text        |

Tip: Creating tables with hyphens and pipes can be tedious. To speed up the process, try using the [Markdown Tables Generator ↗](https://www.tablesgenerator.com/markdown%5Ftables).

## Alignment

You can align text in the columns to the left, right, or center by adding a colon (:) to the left, right, or on both side of the hyphens within the header row.

```plaintext
| Syntax      | Description | Test Text     |
| :---        |    :----:   |          ---: |
| Header      | Title       | Here is this  |
| Paragraph   | Text        | And more      |
```

The rendered output looks like this:

| Syntax    | Description | Test Text    |
| --------- | ----------- | ------------ |
| Header    | Title       | Here is this |
| Paragraph | Text        | And more     |

## Formatting text in tables

You can format the text within tables. For example, you can add links, code, and emphasis.

You can’t add headings, blockquotes, lists, horizontal rules, images, or HTML tags.

## Escaping pipe characters in tables

You can display a pipe (|) character in a table by using its HTML character code ("|").

## HTML examples

For complex tables, consider using HTML. The following example is created with HTML:

| Field             | Description                                                                                                        |
| ----------------- | ------------------------------------------------------------------------------------------------------------------ |
| http.cookieString | Represents the entire cookie as a string.Example value: session=8521F670545D7865F79C3D7BEDC29CCE;-background=light |
| http.hostString   | Represents the hostname used in the full request URI.Example value: [www.example.org ↗](http://www.example.org)    |

## Large tables

Generally, avoid large tables in documentation. If you have a unique use case, wrap the table in the `<table-wrap>` component to make it responsive and scrollable.

| Header 1 | Header 2 | Header 3 | Header 4 |
| -------- | -------- | -------- | -------- |
| test     | test     | test     | test     |

```txt
<table-wrap>

| Header 1 | Header 2 | Header 3 | Header 4 |
| --- | --- | --- | --- |
| test | test | test | test |

</table-wrap>
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/style-and-grammar/formatting/structure/tables/#page","headline":"Tables · Cloudflare Style Guide","description":"Format tables consistently in documentation.","url":"https://developers.cloudflare.com/style-guide/style-and-grammar/formatting/structure/tables/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
