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

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/style-and-grammar/formatting/structure/links/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

A link is a reference to another page, part of a page, or external resource. Hyperlinks are useful, but if overdone, they can distract the reader. Follow these guidelines for link text and placement.

## Types of links

There are 3 types of links:

* **External**: To other resources, such as [www.cloudflare.com ↗](http://www.cloudflare.com).
* **Internal**: To other pages in the docs, such as [Workers](https://developers.cloudflare.com/workers/).
* **Anchor**: To specific parts of other pages in our docs, such as [Proxied records](https://developers.cloudflare.com/dns/proxy-status/#proxied-records).

## Create links

Use the path to the product when creating a link.

* **Do**:  
  * `This is a link for Cloudflare WAN's [Get started](/cloudflare-wan/get-started/)`
* **Don't:**  
  * `This is a link for Cloudflare WAN's [Get started](https://developers.cloudflare.com/cloudflare-wan/get-started/)`

**Also not supported:**

* Relative links: `` A link to [`DurableObjectNamespace::get`](./namespace) ``
* Using the file extension in links: `This is a link for Cloudflare WAN's [Get started](/cloudflare-wan/get-started.mdx/)`

## Standard text

As much as possible, use text that follows one of these patterns:

* `For more information, refer to [<PAGE_TITLE>](LINK).`
* `To <DO_SOMETHING>, refer to [<SECTION_TITLE>](LINK).`

Do not use the following constructions:

* `Learn more about...`
* `To read more....`
* `For more information, refer the [Merge requests](LINK) page.`
* `For more information, refer the [Merge requests](LINK) documentation.`

## Descriptive link text

The more descriptive your link text, the easier it is for people to navigate your site and for Google to understand what you are linking to.

Practically, this means you should avoid link text like `here`, `this page`, or `read more`.

For example, instead of:

* `For more information, refer to [this page](LINK).`
* `For more information, go [here](LINK).`

Use:

* `For more information, refer to [set up Cloudflare](LINK).`

Follow these additional guidelines for inline paragraph links:

* Use the actual title of the target page, or an abbreviated version of that title. This helps readers confirm they reached the page they intended to visit.
* Use unique link text. Speech recognition software does not handle duplicated link text well.
* Use in-paragraph links only when they are internal to Cloudflare's websites and the material relates directly to what is being described. Consider whether the linked content helps the reader make a decision or accomplish something before continuing to read.
* Avoid directional language.

## Dashboard link text

When directing users to the Cloudflare dashboard, use the following convention:

```text
1. Log in to the [Cloudflare dashboard](https://dash.cloudflare.com/login) and select your account and domain.
2. Go to **DNS** > **Records**.
```

## Related resources

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

Use placeholders in links with account- or user-specific information, and explain what to replace the referential text with.

* For example, for the link "`https://api.cloudflare.com/client/v4/accounts/a0b1c2d3/rulesets`" use "`https://api.cloudflare.com/client/v4/accounts/<ACCOUNTID>/rulesets`" and add text to say "replace `<ACCOUNTID>` with your Account ID" or similar.

Refer to [angle brackets](https://developers.cloudflare.com/style-guide/style-and-grammar/formatting/code-conventions-and-format/) in Code conventions and formatting.

## Maintenance

For more details on how we handle link maintenance, refer to [Link maintenance](https://developers.cloudflare.com/style-guide/how-we-docs/links/).

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/style-and-grammar/formatting/structure/links/#page","headline":"Links · Cloudflare Style Guide","description":"Write and format links in documentation.","url":"https://developers.cloudflare.com/style-guide/style-and-grammar/formatting/structure/links/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
