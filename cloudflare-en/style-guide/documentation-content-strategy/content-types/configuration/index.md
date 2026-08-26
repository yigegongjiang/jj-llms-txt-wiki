---
description: Write configuration pages that show the settings and values for a configuration-intensive feature so readers can copy the right setup.
title: Configuration
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Configuration

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/configuration/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

A configuration page shows the specific settings, values, and options for a configuration-intensive feature, so a reader can copy the right setup for their use case rather than follow a procedure. Also known as use cases, configurations are reference pages, not instructions. The tone is plain, descriptive, and straightforward.

## When to use it

Write a configuration when a feature is configuration-intensive, such as rules, and readers mainly need to know which settings and values produce a given outcome. It is not:

* **A how-to.** A how-to walks through the steps to complete a task, whereas a configuration only shows the settings and values for a setup, with no procedural steps.
* **A tutorial.** A tutorial teaches through a guided project, whereas a configuration is a reference the reader consults for the right values, not a lesson.
* **A reference.** A reference exhaustively documents every parameter, whereas a configuration curates example setups for common use cases.

For the full comparison, refer to [Content types](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/).

## Title & description

* **Title**: noun-based, naming the feature being configured, because a configuration describes ways to set up a feature rather than guiding the reader toward a goal.
* **Description**: name the product or feature configured, the use case it serves, and the key settings or values covered.

## Scaffold this page

Copy this skeleton and adapt it to your feature:

```plaintext
---
title: <Feature> configuration
description: Configure <product or feature> for <use case>, covering <the key settings and values>.
pcx_content_type: configuration
sidebar:
  order: 10
products:
  - product-a
---

Introduce the feature in two or three sentences, frame which configurations the reader will encounter, and link to related documentation.

## <Feature area>

State the outcome this configuration produces, then give the settings and values in a table.

| Setting        | Value                      | Notes            |
| -------------- | -------------------------- | ---------------- |
| <setting name> | <value to enter or select> | <when to use it> |
```

## Component guidance

* [**Tables**](https://developers.cloudflare.com/style-guide/style-and-grammar/formatting/structure/tables/) are the signature: a reference table with a 1:1 correspondence between each setting the reader can change and the value to enter or select for a given use case.
* [**Navigation**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/navigation/) helps readers find the right configuration when a feature has many.
* **What does not fit:** step-by-step procedures. If you find yourself writing instructions, use a [how-to](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/how-to/), [tutorial](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/tutorial/), or [example](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/examples/) instead.

## Frontmatter

```yaml
pcx_content_type: configuration
products:
  - product-a
  - product-b
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## Organizing configurations

Open each configuration with a short context paragraph after the title that introduces the feature, frames which configurations the reader will encounter, and links to related documentation. Group the body by feature, giving each feature its own settings table so the reader can scan to the setup that matches their use case.

## Writing for AI and agents

* **Complete setting-value pairs.** Give every table row an explicit setting and the exact value to enter or select, because an agent applies the pair directly with no room to infer.
* **Use-case framing.** State the outcome each configuration produces in its context, so a reader or agent can match a goal to the right table without reading a procedure.
* **Instructions point outward.** Link to the how-to, tutorial, or example that carries any steps, because a configuration itself is not executable as a procedure.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/configuration/#page","headline":"Configuration · Cloudflare Style Guide","description":"Write configuration pages that show the settings and values for a configuration-intensive feature so readers can copy the right setup.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/configuration/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
