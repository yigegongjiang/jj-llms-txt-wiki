---
description: Write and format links in documentation.
title: Links
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Links

Last updated Apr 24, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/links/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

A link is a reference to another page, part of a page, or external resource.

Hyperlinks are incredibly useful but - if overdone - can be distracting.

## Types of links

There are 3 types of links:

* **External**: To other resources, such as [www.cloudflare.com ↗](http://www.cloudflare.com).
* **Internal**: To other pages in the docs, such as [Workers](https://developers.cloudflare.com/workers/).
* **Anchor**: To specific parts of other pages in our docs, such as [Proxied records](https://developers.cloudflare.com/dns/proxy-status/#proxied-records).

## Guidance for inline paragraph links

Avoid non-descriptive link text like: `click here` and `this page`; instead, use the actual title of the target page or an abbreviated version of that title. This is also important so that readers see that when they get there, they actually linked to the page they intended to visit.

Use unique link text. Speech recognition software does not handle duplicated link text well.

Use in-paragraph links only if they are internal (those within Cloudflare's websites) and if the material relates directly to what's being described. In other words, will the content behind the link help the reader make a decision or accomplish something before continuing to read the current document?

Avoid directional language.

## Links for the Related resources section

Use a _Related resources_ section at the end of your document for:

* Internal links that loosely relate to the topic or offer a chance for deeper learning
* All external links (not residing in Cloudflare's websites)
* Internal and external links that represent the next logical steps to follow

External links placed in-paragraph are strongly discouraged because Cloudflare has no control over them. For example, if a link no longer resolves, our content feels less reliable. By shifting all external links to the end of the document, the impact of a broken link is less dramatic.

## Cross-linking requirements

Cross-links between related pages create a navigable knowledge graph. When an AI system encounters a concept page, it can follow links to find step-by-step instructions, troubleshooting guidance, or reference data and cite the most relevant page for a user's query. Search engines use the same link structure to understand topic relationships.

Every page with a `pcx_content_type` should include links to related pages in its **Related resources** section. Use the following table to determine which content types to link to from each page.

| Content type    | Must link to                                                                 |
| --------------- | ---------------------------------------------------------------------------- |
| Concept         | Related how-to or get-started page; related reference page                   |
| How-to          | Prerequisite concept page; relevant configuration page; troubleshooting page |
| Get started     | Next-level how-to pages; product overview page                               |
| Troubleshooting | Related how-to page; relevant configuration page                             |
| Configuration   | Parent how-to or get-started page; relevant concept page                     |
| Reference       | Related concept page; how-to pages that use the reference                    |
| Tutorial        | Related product overview; prerequisite get-started page                      |

Links should be bidirectional. If a concept page links to a how-to, the how-to should link back to the concept page. This ensures that users (and AI systems) can traverse between pages in either direction.

### Example

A concept page about DNS records should link to related how-to, troubleshooting, and reference pages:

```markdown
## Related resources

- To create or modify DNS records, refer to [Manage DNS records](/dns/manage-dns-records/how-to/create-dns-records/).
- For common DNS issues, refer to [Troubleshoot DNS records](/dns/troubleshooting/).
- For a complete list of supported record types, refer to [DNS record types](/dns/manage-dns-records/reference/dns-record-types/).
```

The corresponding how-to page should link back:

```markdown
## Related resources

- To learn how DNS records work, refer to [DNS records](/dns/manage-dns-records/).
- For record type details, refer to [DNS record types](/dns/manage-dns-records/reference/dns-record-types/).
- For common DNS issues, refer to [Troubleshoot DNS records](/dns/troubleshooting/).
```

### When links do not exist

Not every content type will have a matching page for every row in the table. Link to what exists. If a related page does not exist yet, do not create a placeholder link. Instead, consider whether the missing page represents a gap in the doc set that should be addressed.

## Links for instructions in documentation

Place links for example requests and API calls in code blocks.

Use placeholders in links with account- or user-specific information. And explain what to replace the referential text with.

* For example, for the link "`https://api.cloudflare.com/client/v4/accounts/a0b1c2d3/rulesets`" use "`https://api.cloudflare.com/client/v4/accounts/<ACCOUNTID>/rulesets`" and add text to say "replace `<ACCOUNTID>` with your Account ID" or similar.

See [angle brackets](https://developers.cloudflare.com/style-guide/formatting/code-conventions-and-format/) in Code Conventions and Formatting.

## Maintenance

For more details on how we handle link maintenance, refer to [Link maintenance](https://developers.cloudflare.com/style-guide/how-we-docs/links/).

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/links/#page","headline":"Links · Cloudflare Style Guide","description":"Write and format links in documentation.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/links/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-24","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
