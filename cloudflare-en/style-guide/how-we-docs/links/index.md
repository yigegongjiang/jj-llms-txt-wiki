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

Last updated Aug 10, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/how-we-docs/links/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

Though [links](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/links/) are an important part of documentation, they also have their own maintenance cost.

We have a few strategies we use to make link maintenance easier.

## Link types

There are 3 types of links:

* **External**: To other resources, such as [www.cloudflare.com ↗](http://www.cloudflare.com).
* **Internal**: To other pages in the docs, such as [Workers](https://developers.cloudflare.com/workers/).
* **Anchor**: To specific parts of other pages in our docs, such as [Proxied records](https://developers.cloudflare.com/dns/proxy-status/#proxied-records).

For each type of link, we think through a few different aspects of the experience.

* **External**:  
  * _Source of truth_: Another site.
  * _Why does it break_: Another site changed its content.
  * _Customer experience of a break_: `404` page on another site.
* **Internal**:  
  * _Source of truth_: Your site.
  * _Why does it break_: Your site changed its content.
  * _Customer experience of a break_: `404` page on your site.
* **Anchor**:  
  * _Source of truth_: Your site.
  * _Why does it break_: Your site changed its content.
  * _Customer experience of a break_: Page load on your site. Content might be further down the page or have been moved to another page.

## Checks

### Internal links

Of these three [link types](#link-types), only **Internal** links:

* Happen _within_ the context of a change to your site's content.
* Universally lead to a bad customer experience (a `404` page).
* Are easily auditable within the current context.

For these reasons, we choose to make a build **fail** based on broken internal links. For our implementation, we rely on [Nimbus ↗](https://nimbus-docs.com/)'s `nimbus/internal-link` [lint rule ↗](https://nimbus-docs.com/writing/linting/), configured in [astro.config.ts ↗](https://github.com/cloudflare/cloudflare-docs/blob/production/astro.config.ts).

We also make two intentional decisions about this link auditing:

* **Absolute links, not relative**: We enforce absolute links (`/style-guide/how-we-docs/metadata/`) and fail on relative links (`../metadata/`) to avoid time-consuming maintenance in the future. This decision also helps with find/replace work and any future platform migrations.
* **No redirects**: We do not consider redirects when evaluating links. We have the current source of truth, so we should utilize that truth to its fullest (as well as helping us avoid redirect chains and future maintenance).

### External links

Though external links are not good for the customer experience, they also don't change within the context of a change to your site's content. Additionally, external link checking can be time consuming and error prone, which can slow down contributions.

We use an external SEO tool to help flag these broken external links for us, addressing them as needed (instead of making a build fail because of them).

### Anchor links

Anchor links do not have as dramatic as consequences of being wrong as internal links. If you have a broken anchor link, a customer will either need to manually scroll to the header or - in some cases - go to another page.

Because of these characteristics, we run [periodic, background checks ↗](https://github.com/cloudflare/cloudflare-docs/blob/production/.github/workflows/anchor-link-audit.yml) to flag broken anchor links, using the `htmltest` library.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/how-we-docs/links/#page","headline":"Links · Cloudflare Style Guide","description":"Write and format links in documentation.","url":"https://developers.cloudflare.com/style-guide/how-we-docs/links/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-10","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
