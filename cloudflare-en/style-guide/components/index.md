---
description: Browse available MDX components for Cloudflare docs.
title: Components
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Components

Last updated Aug 14, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/components/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

When you are [contributing to the Cloudflare Docs](https://developers.cloudflare.com/style-guide/contributions/), you can use our custom components to add additional formatting, such as buttons, tabs, and collapsible sections.

This guide shows you the basics of importing and adding a component to a page. Refer to each component page in this Style Guide to learn the specific props and requirements for each.

Our components are based on [Astro components ↗](https://docs.astro.build/en/basics/astro-components/) and are written in [MDX ↗](https://docs.astro.build/en/guides/markdown-content/), an extended version of Markdown. [Learn more about the Cloudflare Docs framework](https://developers.cloudflare.com/style-guide/how-we-docs/our-site/#site-framework).

## Add a component to a page

To add a component to a page:

1. Import the component to the page by adding this text directly below the [frontmatter](https://developers.cloudflare.com/style-guide/frontmatter/):  
```mdx  
import { COMPONENT_NAME } from "~/components";  
;  
```  
For example, if you were to add [the DashButton component](https://developers.cloudflare.com/style-guide/components/dash-button/) to the [Images getting started page](https://developers.cloudflare.com/images/get-started/), the top of the MDX file corresponding to that page would look like the following:  
```mdx
---  
pcx_content_type: get-started  
title: Getting started  
 products:
   - images  
sidebar:  
  order: 2
---  
import { DashButton } from "~/components";  
;  
```  
Page-specific wrapper components or one-off components do not need to be added to the barrel — import them via a deep path:  
```mdx  
import BaseSchemaProperties from "~/components/BaseSchemaProperties.astro";  
```
2. Add the component to the page by adding this text anywhere on the page you want the component to appear:  
```mdx  
<COMPONENT_NAME PROP_NAME="PROP_VALUE" />  
```  
For example, if you were to add the `DashButton` component to some steps in the [Images getting started page](https://developers.cloudflare.com/images/get-started/), here is how the MDX file would look:  
```mdx
1. In the Cloudflare dashboard, go to the **Transformations** page.  
   <DashButton url="/?to=/:account/images/transformations" />

2. Go to the specific zone where you want to enable transformations.  
```

This is how this example would display after it is published:

![DashButton component
example](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=1074,height=282,format=webp/_astro/dashbutton-example.Dr0ifkyr.png) 

## Choose the right component

To choose the right component for your use case, browse this table which contains our most commonly used components and a visual example of each. For full documentation on all available components and their use cases, browse the individual component pages in this Style Guide.

| Component                                                                                              | Description & visual example                                                                                                                                                                                                                                                                                                                                                           |
| ------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [APIRequest](https://developers.cloudflare.com/style-guide/components/api-request/)                    | Styled API request block. Generate executable cURL API commands with the required API token permissions. ![APIRequest component example](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=800,height=454,format=webp/_astro/apirequest-example.hp_52Lbh.png)                                                                                                     |
| [Badge](https://developers.cloudflare.com/style-guide/components/badges/)                              | Small descriptive pill. Label content with status, version, category, or other short metadata. ![Badge component example](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=800,height=100,format=webp/_astro/badge-example.CPJJLYDk.png)                                                                                                                         |
| [DashButton](https://developers.cloudflare.com/style-guide/components/dash-button/)                    | Dashboard deep-link button. Directly link users from documentation into a specific, relevant section of the Cloudflare Dashboard. ![DashButton component example](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=800,height=256,format=webp/_astro/dashbutton-example-2.LsbX6wB1.png)                                                                          |
| [Details](https://developers.cloudflare.com/style-guide/components/details/)                           | Click-to-expand content block. Hide non-essential, complex, or advanced technical content, allowing users to expand the section when needed. ![Details example](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=800,height=380,format=webp/_astro/details-example.ceYRqozl.png)                                                                                 |
| [DirectoryListing](https://developers.cloudflare.com/style-guide/components/directory-listing)         | Auto-generated sub-page list. Automatically generate a navigable list of links to sub-pages within a specified documentation folder path. ![DirectoryListing component example](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=800,height=246,format=webp/_astro/directorylisting-example.D0UZYG46.png)                                                        |
| [Feature](https://developers.cloudflare.com/style-guide/components/feature/)                           | Product feature list item. Highlight a product feature with a description and a direct link button. ![Feature component example](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=780,height=320,format=webp/_astro/feature-example.DNvnxjFl.png)                                                                                                                |
| [GlossaryTooltip](https://developers.cloudflare.com/style-guide/components/glossary-tooltip/)          | Hover-activated glossary popup. Provide non-disruptive, hover-activated definitions for technical terms pulled from the documentation glossary. ![Glossary tooltip example](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=802,height=214,format=webp/_astro/glossarytooltip-example.DDUbgTTz.png)                                                             |
| [LinkCard](https://developers.cloudflare.com/style-guide/components/link-cards/)                       | Navigational cards. Present related tutorials, concepts, or guides in a visually engaging format. ![LinkCard component example](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=1624,height=614,format=webp/_astro/linkcard-example.DPZVc0vQ.png)                                                                                                               |
| [PackageManagers](https://developers.cloudflare.com/style-guide/components/package-managers)           | Command switcher tabs. Display equivalent installation or execution commands for different package managers. ![DirectoryListing component example](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=800,height=282,format=webp/_astro/packagemanagers-example.BogJLxs-.png)                                                                                      |
| [Plan](https://developers.cloudflare.com/style-guide/components/plan/)                                 | Product plan availability badge. Show the plan required for a product or specific feature. ![Plan component example](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=1604,height=146,format=webp/_astro/plan-example.CKcqf27w.png)                                                                                                                              |
| [RelatedProduct](https://developers.cloudflare.com/style-guide/components/related-product/)            | Formatted product reference. Visually highlight and link to a specific, complementary Cloudflare product, also featuring the product's logo. ![RelatedProduct component example](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=1592,height=782,format=webp/_astro/relatedproduct-example.PHvfW3li.png)                                                        |
| [ResourcesBySelector](https://developers.cloudflare.com/style-guide/components/resources-by-selector/) | Filterable code example library. Pull and display lists of code examples and resources based on content type or products. ![ResourcesBySelector component example](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=1520,height=1302,format=webp/_astro/resourcesbyselector-example.DNA80nn-.png)                                                                |
| [Stream](https://developers.cloudflare.com/style-guide/components/stream/)                             | Embeddable video player. Display a video player optimized for Cloudflare Stream. ![Stream example](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=1584,height=998,format=webp/_astro/stream-example.MfwqXnaD.png)                                                                                                                                              |
| [Tabs and TabItem](https://developers.cloudflare.com/style-guide/components/tabs/)                     | Switchable content tabs. Allow easy switching between content views for different code languages or configuration methods. ![Tabs example](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=802,height=250,format=webp/_astro/tabs-example.Bo6un1S4.png)                                                                                                         |
| [Type and MetaInfo](https://developers.cloudflare.com/style-guide/components/type-highlighting/)       | Pill-shaped data type badge and metadata annotation about a field or property. Type indicates API parameter data types (String, Integer) and MetaInfo indicates metadata constraints (Required, Optional, Read-only). ![Type and MetaInfo example](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=802,height=148,format=webp/_astro/type-example.DQadfRUC.png) |
| [WranglerConfig](https://developers.cloudflare.com/style-guide/components/wrangler-config/)            | Tabbed Wrangler config display. Show Wrangler configuration files (JSONC and TOML) and bindings with automatic format switching. ![WranglerConfig example](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=798,height=356,format=webp/_astro/wranglerconfig-example.Bc0AW5RB.png)                                                                               |
| [YouTube](https://developers.cloudflare.com/style-guide/components/youtube/)                           | Embeddable video player. Embeds a YouTube video player with a specified video ID. ![YouTube example](https://developers.cloudflare.com/cdn-cgi/image/onerror=redirect,width=1568,height=900,format=webp/_astro/youtube-example.Du_GD2xs.png)                                                                                                                                           |

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/components/#page","headline":"Components · Cloudflare Style Guide","description":"Browse available MDX components for Cloudflare docs.","url":"https://developers.cloudflare.com/style-guide/components/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-14","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
