---
description: Write product overview pages.
title: Overview
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Overview

Last updated Jul 22, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/overview/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

## Purpose

The purpose of a landing page is to welcome users and provide an overview of the product.

## Tone

Accessible, welcoming, conversational, outspoken

## content\_type

```yaml
pcx_content_type: overview
products:
  - product-a
  - product-b
  - product-c
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## Structure

### Most used components

* [PublicStats](https://developers.cloudflare.com/style-guide/components/public-stats/):  
The `PublicStats` component allows you to reference specific values about Cloudflare's network without maintaining those values in multiple files.

### Required components

**Metadata title**: Overview

**Title**: Name of the product, group of products, or conceptual content area. H1\. Usually a noun. Do not add "documentation" to the title. Do not use gerund phrases.

**Intro/overview**: Brief welcoming introductory content. May be combined with product description.

**Product description**: What does this product do? Why would you use it?

**Product availability**: What plan(s) is this available to? Review [available plan types ↗](https://github.com/cloudflare/cloudflare-docs/blob/production/src/components/Plan.astro).

**Product attributes**: What is included with this product? (Specific actions, protections, etc.)

### Optional components

**Features**: A few main features specific to product. Includes a link to relevant documentation on feature.

**Related products**: Links to documentation for products used or configured together with current product. For product icons, refer to this [icon library ↗](https://cfdata.lol/icons/).

**More resources**: External links to related resources, such as plans, pricing. Do not duplicate the information from the footer. Also, if the product is free to use or there are not any useful links, feel free to skip this section. Review [available icons](https://developers.cloudflare.com/style-guide/components/icons/).

**Visual**: Graphic or image that enhances the landing page. It should be something relatively static that will not require much (if any) updating in the future.

**Integration information**

## Template

```plaintext

---
title: Overview
weight: xx
layout: overview
pcx_content_type: overview
description: <Cloudflare product> <what it does in one sentence>. Available on <plan types>.
products:
  - product-a
  - product-b
  - product-c
---

# Cloudflare <product name> (or {{</*beta*/>}}Cloudflare <product name>{{</*/beta*/>}} for products in beta)

{{</*description*/>}}
Product description - What does this product do? Why would you use it? Short overview of product capability (~10-15 words).
{{</*/description*/>}}

{{</*plan type="<type>"*/>}}

Summary - Brief welcoming introductory content. A few sentences describing the product’s benefits to the customer. Focus on customer benefit but can also include general product information.

Learn how to [get started](/<product>/get-started/).

---

## Features

{{</*feature header="Name of feature" href="https://developers.cloudflare.com/link/to/feature/" cta="Optional message that's different from the name of the feature"*/>}}
Description highlighting capabilities of product feature. This section accepts Markdown lists for multiple attributes.
{{</*/feature*/>}}

---

## Related products

{{</*related header="<Name of product>" href="https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/overview/</link/to/product>" product="<slugified-product-name>"*/>}}
Description of product used together or connected configuration with current product.
{{</*/related*/>}}

---

## More resources

{{</*resource-group*/>}}

{{</*resource header="<Resource name>" href="https://www.cloudflare.com/link-to-resource/" icon="icon-name"*/>}}
Description of external resource related to current product.
{{</*/resource*/>}}

{{</*/resource-group*/>}}
```

## Additional Information

Overview pages are the default "first" page in any nested navigation. In some cases, to ensure good information architecture and navigability, you may need to rename or remove overview pages.

### When to consider removing an verview page

If the overview acts as a table of contents that provides no additional information or context, consider removing it altogether.

### How to remove an overview page

Deleting the `index.mdx` file in a nested folder is not possible and will result in a build error. To remove the page from the docs, the `index.mdx` file must be hidden using sidebar styling changes. Also, to ensure that users do not accidentally access the overview page, add a redirect.

To hide an overview page, set the `group.hideIndex` property to `true` in the page's frontmatter.

```plaintext
---
title: Placeholder
sidebar:
  group:
    hideIndex: true
---
```

## Examples

[Argo Smart Routing documentation](https://developers.cloudflare.com/argo-smart-routing/)

### Many availabilities

[Images](https://developers.cloudflare.com/images/) is an example of a product whose availability is complex, and is not easy to just use one type of plan. To create components for each plan type, insert `{{</*plan type="<PLAN_TYPE>"*/>}}` below each of the feature component, like so:

```plaintext
{{</*feature header="<CLOUDFLARE_PRODUCT>" href="https://developers.cloudflare.com/path/to/product"*/>}}

{{</*plan type="PLAN_TYPE"*/>}}

Description of content in this section.

{{</*/feature*/>}}
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.DMYpXs3t.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/overview/#page","headline":"Overview · Cloudflare Style Guide","description":"Write product overview pages.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/overview/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-07-22","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
