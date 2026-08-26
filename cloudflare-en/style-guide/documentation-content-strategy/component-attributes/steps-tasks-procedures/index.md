---
description: Write clear procedural steps.
title: Steps/tasks/procedures
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Steps/tasks/procedures

Last updated Aug 20, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/steps-tasks-procedures/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

## Definition

Action-oriented processes that outline steps to take and the order the steps should be taken.

## Used in

[How to](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/how-to/), [Tutorial](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/tutorial/)

## Structure

**Single-step procedures**: When a procedure consists of just one step, add the step into the introductory sentence.

**Sub-steps in numbered procedures**: In a numbered procedure, sub-steps should be lowercase letters, and sub-sub-steps get lowercase Roman numerals.

* When a step has sub-steps, treat the step like an introductory sentence. Put a colon or a period at the end of the step where appropriate.

**Multi-action procedures**: Use one step per action. However, you can combine small actions into one step.

**Multiple procedures for the same task**: If there is more than one way to complete a task, pick one procedure to document that is accessible for all users. If all of the procedures need to be documented, use separate headings or pages or tabs to separate the procedures to make it clear to the reader that this is an alternative way to complete the same task.

The following guidelines can help you choose which procedure to document:

* Choose a procedure that lets readers do all the steps using only a keyboard.
* Choose the shortest procedure.
* Choose a procedure that uses a programming language that the majority of your audience is familiar with.

**Repetitive procedures**: Use concise procedures to avoid repetitiveness and overwhelming the user with a lot of bold UI elements.

[Bullets vs. Numbered Lists](https://developers.cloudflare.com/style-guide/style-and-grammar/formatting/structure/lists/)

**Post requisites**: Not used at this time. If you feel like you need a post requisites section, consider adding the task as the final step of a procedure or moving the content into Next steps.

## Guidelines for writing procedures

If the user must log in to the dashboard as a first step, consolidate logging in and navigation into the first step. Also, write "log in to" (three words) instead of "log into".

If the user must press **Enter** after a step, then include that instruction as part of the step.

If the user has to turn a setting on or off, use "turn <FEATURE\_NAME> on/off" — or "turn on/off <FEATURE\_NAME>" for features with long names — instead of "enable/disable".

State the purpose of the action before stating the action.

Write in the order that the reader needs to follow. State the location of the action before stating the action. If there are multiple headings associated with a set of procedures, restate the location of the action in the first step of each procedure, even if the location is the same as in the previous procedure.

Do not use "please."

## Additional information

Use complete sentences.

Use parallel structure.

Use second person imperative. Refer to the Style Guide for guidance on when to use certain verbs (click, select, choose, etc).

For an optional step, type (Optional) as the first word of the step.

* For example: (Optional) Type an arbitrary string, to be delivered to the target address with each notification delivered over this channel.

Do not include keyboard shortcuts.

Do not use directional language to orient the reader, such as above, below, or right-hand side. This type of language does not work well for accessibility or for localization. If a UI element is hard to find, provide a screenshot.

Use sentences like "The `<screen/page/card>` displays." wisely.

Note

Usually, you only need this kind of helper sentence if the user ended up in an unexpected location, or if there was more than one possible target location, depending on the options that the user selected.

As an alternative, consider adding the `<screen/page/card>` mention at the beginning of the next step: "5\. In `<screen/page/card>`, select Save."

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/steps-tasks-procedures/#page","headline":"Steps/tasks/procedures · Cloudflare Style Guide","description":"Write clear procedural steps.","url":"https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/steps-tasks-procedures/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-08-20","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
