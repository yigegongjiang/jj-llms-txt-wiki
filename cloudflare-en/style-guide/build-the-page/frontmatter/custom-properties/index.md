---
description: Configure custom frontmatter properties.
title: Custom properties
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Custom properties

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/custom-properties/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

We have added specific custom [frontmatter](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/) properties to meet specific needs.

Note

The `description` field is a Nimbus built-in, not a custom property, but it is required for all pages with a `pcx_content_type`. For writing guidance, refer to [Writing a description](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/#writing-a-description).

## Properties

### banner

**Type:** `object` optional

**Description:** Displays a [Banner](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/banner/) on the current docs page.

### canonical

**Type:** `string` optional

**Description:** A canonical URL or path to set as the `<link rel="canonical">` in the page `<head>`, overriding the default derived from the page URL.

### difficulty

**Type:** `string` optional

**Description:** Difficulty is displayed as a column in the [ListTutorials component](https://developers.cloudflare.com/style-guide/build-the-page/components/list-tutorials/).

### external\_link

**Type:** `string` optional

**Description:** Path to another page in our docs or elsewhere. Used to add a crosslink entry to the lefthand navigation sidebar.

### feedback

**Type:** `boolean`

**Description:** Whether to show the FeedbackPrompt on the page, defaults to true

### hideChildren

**Type:** `boolean` optional

**Description:** Renders this group as a single link on the sidebar, to the index page. Refer to [Sidebar](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/sidebar/).

### noindex

**Type:** `boolean` optional

**Description:** If true, this property adds a `noindex` declaration to the page, which will tell internal / external search crawlers to ignore this page. Helpful for pages that are historically accurate, but no longer recommended, such as [Workers Sites](https://developers.cloudflare.com/workers/configuration/sites/).

### pcx\_content\_type

**Type:** `string` optional

**Description:** The purpose of the page, and defined through specific pages in [Content strategy](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/).

### products

**Type:** `array`

**Description:** The names of related directory entries (according to their file name in `src/content/directory`). Usually, these correspond to file paths, but not always, such as with `cloudflare-tunnel`

### release\_notes\_file\_name

**Type:** `array` optional

**Description:** Required for the [ProductReleaseNotes](https://developers.cloudflare.com/style-guide/build-the-page/components/usage/#productreleasenotes) component.

### reviewed

**Type:** `undefined` optional

**Description:** A `YYYY-MM-DD` value that signals when the page was last explicitly reviewed from beginning to end.

### sidebar

**Type:** `object`

**Description:** Used to configure various sidebar options. Refer to [Sidebar](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/sidebar/).

### styleGuide

**Type:** `object` optional

**Description:** Used by overrides for style guide component documentation, which helps us display the [usage counts](https://developers.cloudflare.com/style-guide/build-the-page/components/usage/) for components directly on the component page itself.

### summary

**Type:** `string` optional

**Description:** Renders a summary description directly below the page title.

### tags

**Type:** `array` optional

**Description:** A group of related keywords relating to the purpose of the page.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/custom-properties/#page","headline":"Custom properties · Cloudflare Style Guide","description":"Configure custom frontmatter properties.","url":"https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/custom-properties/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
