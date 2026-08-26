---
description: Write glossary term definitions.
title: Glossary entry
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Glossary entry

Last updated Apr 24, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/glossary-entry/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

## Definition

A single term and corresponding definition in the glossary.

## Used in

Glossary, documentation pages, tooltips.

## Structure

### Data

The data underlying our glossary lives with YAML files in the [/src/content/glossary/\* ↗](https://github.com/cloudflare/cloudflare-docs/tree/production/src/content/glossary) folder.

Each file should be structured similar to the following:

```yaml
---
productName: DNS
entries:
  - term: active zone
    general_definition: |-
      a DNS zone that is active on Cloudflare requires changing its nameservers to Cloudflare's for management.
    associated_products:
      - Cloudflare One

  - term: apex domain
    general_definition: |-
      apex domain is used to refer to a domain that does not contain a subdomain part, such as `example.com` (without `www.`). It is also known as "root domain" or "naked domain".

  - term: DNS over HTTPS
    general_definition: |-
      DNS over HTTPS (DoH) is a standard for encrypting DNS traffic, preventing tracking and spoofing of DNS queries.
    associated_products:
      - 1.1.1.1
      - Cloudflare One

  - term: DNS over TLS
    general_definition: |-
      DNS over TLS (DoT) is a standard for encrypting DNS traffic using its own port (853) and TLS encryption.
    associated_products:
      - 1.1.1.1
      - Cloudflare One
```

Relevant values include the following:

* `productName` string required

  * Core product associated with this file. Should always match the same formatting / styling used in `associated_products`.
* `entries` object required

  * `term` string required

    * The glossary term itself.
  * `general_definition` string required

    * Definition of the term. Should be general enough to apply to multiple products. Should also start with a lowercase letter unless starting with a proper noun.
  * `associated_products` array optional

    * If the term is associated with other products. Any names used should correspond to the `productName` of that associated file.

### Usage

Because of the [structured data](#data) associated with our glossaries, we can pull these terms into multiple places.

#### Product-level glossary

A product-level glossary includes all terms associated with a particular product, which will pull in terms directly in that product's glossary file and any terms that include the product in its `associated_products`.

```mdx
---
title: Glossary
pcx_content_type: glossary
---

import { Glossary } from "~/components";

Review the definitions for terms used across Cloudflare's DNS documentation.

<Glossary product="dns" />
```

#### Glossary definition

Pull glossary definitions directly into your Markdown by using the `<GlossaryDefinition>` component.

> A DNS zone that is active on Cloudflare requires changing its nameservers to Cloudflare's for management.

Is a quoted definition that comes from:

```mdx
<GlossaryDefinition term="active zone" prepend="An active zone is " />
```

Properties are:

* `term` string required

  * Should match a term within an existing glossary YAML file.
* `prepend` string optional

  * Text to add before a definition.

#### Glossary tooltip

Pull component definitions into a focusable tooltip for a specific phrase by using the `<GlossaryTooltip>` component.

Here's a tooltip example.

```mdx
Here's a <GlossaryTooltip term="active zone">tooltip</GlossaryTooltip> example.
```

Properties are:

* `term` string required

  * Should match a term within an existing glossary YAML file.
* `prepend` string optional

  * Text to add before a definition.
* `link` string optional

  * Wraps the inner text in a markdown link, similar to normal markdown formatting.

Because of space limitations, the tooltip will always default to the short definition of a term, meaning the definition text before the first line break.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/glossary-entry/#page","headline":"Glossary entry · Cloudflare Style Guide","description":"Write glossary term definitions.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/glossary-entry/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-24","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
