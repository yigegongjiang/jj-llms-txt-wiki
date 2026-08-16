---
description: Write third-party integration guide documentation.
title: 3rd-party integration guide
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# 3rd-party integration guide

Last updated Apr 24, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/3rd-party-integration-guide/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

## Purpose

The purpose of a 3rd-party integration guide is to explain how to use a 3rd-party product with Cloudflare. Although we want to help our customers as integrations between different products can be a pain point, there is a large risk and maintenance cost associated with specific types of 3rd-party resources.

## Tone

instructional, straightforward

## content\_type

```yaml
pcx_content_type: integration-guide
products:
  - product-a
  - product-b
  - product-c
```

For more details, refer to [pcx\_content\_type](https://developers.cloudflare.com/style-guide/frontmatter/custom-properties/#pcx%5Fcontent%5Ftype).

## Structure

### Required components

[**Title**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/titles/): Short verb phrase in second-person imperative that includes the 3rd-party name. Do not use gerund phrases.

If a 3rd-party integration guide is with a specific Cloudflare technology partner, add a Markdown component that indicates `<partner>` after the title.

[**Context**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/context/): An introductory paragraph on the following steps and what they will accomplish.

Provide context to the reader that is not in the section heading.

End with a colon or a period. Use a colon if it immediately precedes the steps. Use a period if there is more material (such as a note) between the context and the procedure.

Do not provide context for steps with a partial sentence that is completed by the numbered steps.

Mention any unique considerations between the 3rd-party and Cloudflare.

[**Prerequisites**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/prerequisites/): Tasks or conditions that must be completed before a user can complete a series of steps.

For 3rd-party integration guides, include information about what you need to interact with the third party for the following steps.

[**Steps**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/steps-tasks-procedures/): Numbered steps that complete a task.

Link out for basic concepts (Regex, JavaScript, web server maintenance).

Caution

Step-by-step instructions of 3rd-party environments are discouraged generally, but acceptable in certain situations. General preference is to link back to an article that someone else maintains.

They easily become out-of-date, especially if we can not access the 3rd-party product

[**Links**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/links/): May be a bulleted list that references the 3rd-party product or in-text links to the 3rd-party process documentation.

Link to reputable sources within reason.

### Optional components

[**Notes/warnings**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/notes-tips-warnings/)

[**Examples**](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/examples/)

**Screenshots**

**Tables**

**Step validation**

Note

Screenshots of the 3rd-party product are highly discouraged. It has all the problems of video or screenshot maintenance, but with a much greater risk that something changes and we are not aware of it.

It may become a bigger problem if we can not access the 3rd-party product.

## Templates

Single procedure 3rd-party integration guide

```plaintext

---
weight: xx
pcx_content_type: integration-guide
description: Integrate <3rd-party name> with <Cloudflare product> to <what the integration accomplishes>. Requires <key prerequisites>.
products:
  - product-a
  - product-b
  - product-c
---

# Second-person imperative verb phrase with 3rd-party name included

Context for procedure

Prerequisites

1. Step one
2. Step two
3. Step three
4. ...
```

3rd-party integration guide with multiple procedures that must be completed in order

```plaintext

---
weight: xx
pcx_content_type: integration-guide
description: Integrate <3rd-party name> with <Cloudflare product> to <what the integration accomplishes>. Requires <key prerequisites>.
products:
  - product-a
  - product-b
  - product-c
---

# Second-person imperative verb phrase with 3rd-party name included

Context for procedure

Prerequisites

## 1. Second-person imperative verb phrase

1. Step one
2. Step two
3. Step three
4. ...

## 2. Second-person imperative verb phrase

1. Step one
2. Step two
3. Step three
4. ...

## 3. Second-person imperative verb phrase

1. Step one
2. Step two
3. Step three
4. ...
```

## Examples

**3rd-party integration in the Cloudflare dashboard**:

* [Enable Logpush to Sumo Logic](https://developers.cloudflare.com/logs/logpush/logpush-job/enable-destinations/sumo-logic/)
* [Device Posture - Carbon Black](https://developers.cloudflare.com/cloudflare-one/reusable-components/posture-checks/client-checks/carbon-black/)

**Linking to external documentation**:

* [GitHub SMS notifications using Twilio](https://developers.cloudflare.com/workers/tutorials/github-sms-notifications-using-twilio/#sending-a-text-with-twilio)

(Discouraged but acceptable scenario) **How to with instructions in 3rd-party environment and within Cloudflare dashboard**:

* [IDP integration - Microsoft Entra ID (formerly Azure Active Directory)](https://developers.cloudflare.com/cloudflare-one/integrations/identity-providers/entra-id/)
* [Managed deployment - Partners - Jamf](https://developers.cloudflare.com/cloudflare-one/team-and-resources/devices/cloudflare-one-client/deployment/mdm-deployment/partners/jamf/)

### Additional information

External integration guides are more costly to maintain because we do not control external UI and we do not typically have visibility into changes the same way we do for internal products.

We publish post-sales content. It might be referred to during pre-sales, but we publish use-phase content.

We publish with the expectation of maintenance. If you want to publish something without the expectation of maintenance, write a blog.

### Products where we frequently see 3rd-party information

* [Workers](https://developers.cloudflare.com/workers/tutorials/)
* [Zero Trust](https://developers.cloudflare.com/cloudflare-one/integrations/identity-providers/)
* [Analytics](https://developers.cloudflare.com/analytics/analytics-integrations/)

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/3rd-party-integration-guide/#page","headline":"3rd-party integration guide · Cloudflare Style Guide","description":"Write third-party integration guide documentation.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/3rd-party-integration-guide/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-24","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
