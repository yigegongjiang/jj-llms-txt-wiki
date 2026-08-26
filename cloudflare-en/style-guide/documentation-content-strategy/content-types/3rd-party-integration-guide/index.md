---
description: Write third-party integration guides that connect an external product with Cloudflare, favoring links to the third party's own maintained documentation.
title: 3rd-party integration guide
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# 3rd-party integration guide

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/3rd-party-integration-guide/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

A third-party integration guide explains how to use a third-party product with Cloudflare. Because Cloudflare does not control the third party's interface, these guides carry real maintenance risk, so they favor linking to externally maintained documentation over reproducing another product's steps. The tone is instructional and straightforward.

## When to use it

Write a third-party integration guide when a reader needs to connect one specific external product with Cloudflare, and the integration is worth publishing with an ongoing maintenance commitment. It is not:

* **A how-to.** A how-to documents a task entirely within Cloudflare, whereas an integration guide crosses into a third-party product Cloudflare does not control.
* **A blog post.** Publish an integration guide only with the expectation of maintenance. If you do not intend to maintain it, write a blog post instead.

For the full comparison, refer to [Content types](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/).

## Title & description

* **Title**: a short verb phrase in the second-person imperative that includes the third-party product name, not a gerund. If the integration is with a Cloudflare technology partner, add the partner component after the title.
* **Description**: name the third-party product and the Cloudflare product, state what the integration accomplishes, and note the key prerequisites.

## Scaffold this page

Copy this skeleton and adapt it to your integration:

```plaintext
---
title: <Verb phrase naming the third-party product>
description: Connect <third-party product> with <Cloudflare product> to <what the integration accomplishes>.
pcx_content_type: integration-guide
sidebar:
  order: 10
products:
  - product-a
---

Introduce what the integration accomplishes and any considerations unique to the third party.

## Prerequisites

List what the reader needs on the third-party side before starting.

## Set up the integration

Give the steps that complete the integration, linking to the third party's own documentation rather than reproducing its process.

## Related links

Point to the third party's maintained documentation and the related Cloudflare product docs.
```

## Component guidance

* [**Context**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/introductions/#context) introduces what the steps accomplish and any considerations unique to the third party.
* [**Prerequisites**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/prerequisites/) list what the reader needs on the third-party side before starting.
* [**Steps**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/steps-tasks-procedures/) complete the integration, linking out for basic concepts rather than teaching them.
* [**Links**](https://developers.cloudflare.com/style-guide/style-and-grammar/formatting/structure/links/) point to the third party's own documentation, preferred over reproducing its process, and only to reputable sources.
* **What does not fit:** step-by-step instructions of the third-party product, which are discouraged because they go out of date the moment the third party changes. Screenshots of the third-party product are the most fragile of all, so avoid them and link to the externally maintained source instead.

## Frontmatter

```yaml
pcx_content_type: integration-guide
products:
  - product-a
  - product-b
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/build-the-page/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## Maintenance and scope

Publish a third-party integration guide as post-sales, use-phase content, with the expectation that someone will maintain it. External guides cost more to maintain because Cloudflare does not control the third party's interface and does not get the same visibility into changes. If you want to publish something without that maintenance commitment, write a blog post instead. This content appears most often around [Workers](https://developers.cloudflare.com/workers/tutorials/), [Zero Trust](https://developers.cloudflare.com/cloudflare-one/integrations/identity-providers/), and [Analytics](https://developers.cloudflare.com/analytics/analytics-integrations/) integrations.

## Examples

Integration handled in the Cloudflare dashboard:

* [Enable Logpush to Sumo Logic](https://developers.cloudflare.com/logs/logpush/logpush-job/enable-destinations/sumo-logic/)
* [Device Posture - Carbon Black](https://developers.cloudflare.com/cloudflare-one/reusable-components/posture-checks/client-checks/carbon-black/)

Linking out to external documentation:

* [GitHub SMS notifications using Twilio](https://developers.cloudflare.com/workers/tutorials/github-sms-notifications-using-twilio/#sending-a-text-with-twilio)

Instructions in both the third-party environment and the Cloudflare dashboard, which is discouraged but sometimes acceptable:

* [IdP integration - Microsoft Entra ID](https://developers.cloudflare.com/cloudflare-one/integrations/identity-providers/entra-id/)
* [Managed deployment - Partners - Jamf](https://developers.cloudflare.com/cloudflare-one/team-and-resources/devices/cloudflare-one-client/deployment/mdm-deployment/partners/jamf/)

## Writing for AI and agents

* **Link, do not reproduce.** Point to the third party's own documentation for its steps, because reproduced instructions and screenshots go stale as soon as the external product changes.
* **Complete prerequisites.** State exactly what the reader needs on the third-party side before starting, because an agent cannot complete an integration it is not set up to reach.
* **Name both products.** Name the third-party product and the Cloudflare product in the title and context, so a reader or agent can match the guide to the integration they need.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/3rd-party-integration-guide/#page","headline":"3rd-party integration guide · Cloudflare Style Guide","description":"Write third-party integration guides that connect an external product with Cloudflare, favoring links to the third party's own maintained documentation.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/3rd-party-integration-guide/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
