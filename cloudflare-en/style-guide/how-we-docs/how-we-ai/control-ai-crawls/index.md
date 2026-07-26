---
description: Manage AI crawler access to documentation.
title: Control how AI crawls your docs
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Control how AI crawls your docs

Last updated Apr 24, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/how-we-docs/how-we-ai/control-ai-crawls/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

Allowing AI crawlers to crawl your documentation enables end users to extract useful information from their preferred AI tool. For example, it enables users to ask tools like ChatGPT or Gemini questions about your documentation, rather than reading entire pages. However, it is important to control how the AI crawlers interact with your site for optimal results.

## Instruct AI crawlers to crawl correct pages

You should first consider which pages you want AI crawlers to crawl. You may wish the AI crawler to access most of your pages, but there may be certain exceptions.

### Use `robots.txt` to control AI crawlers

You can use the `robots.txt` file to control which pages AI crawlers can access. This is a simple text file which instructs crawlers to follow certain rules. The crawler is not forced to follow them, but many crawlers operated by major companies such as Google and OpenAI respect `robots.txt` files. Refer to [robots.txt setting](https://developers.cloudflare.com/bots/additional-configurations/managed-robots-txt/) for more information.

For example, you can add the following to your `robots.txt` file to prevent AI crawlers from accessing a beta product called "Product A", located in `/docs-site/product-a/`:

```txt
User-agent: *
Disallow: `/product-a/`
```

By specifying explicit disallow conditions in your `robots.txt` file, you allow access to most of your pages, with only a small number of exceptions.

Refer to [https://developers.cloudflare.com/robots.txt ↗](https://developers.cloudflare.com/robots.txt) as an example.

### Use security control to completely block access

Sometimes, you may wish to completely block crawlers from accessing a certain page. You cannot solely rely on `robots.txt` to block access, as crawlers are not forced to follow `robots.txt` files.

To ensure complete blocking, you can use security controls, such as Cloudflare's [AI Crawl Control](https://developers.cloudflare.com/ai-crawl-control/), [bot solutions](https://developers.cloudflare.com/bots/), or some other security tool.

### Case study: GitHub preview sites

Cloudflare's developer documentation is publicly available at [cloudflare-docs GitHub repository ↗](https://github.com/cloudflare/cloudflare-docs). Every time a git pull request is created, we generate a temporary preview site for visual inspection of what the documentation will look like when it merges into production. Since these preview sites are generated from pull requests that are under review, they sometimes contain incomplete information.

In November 2025, using AI Crawl Control, we found that as much as 80% of the AI crawls were accessing our preview sites instead of our main site. This meant that AI crawlers were prone to returning inaccurate information, which may have seemed like hallucinations.

After the discovery, we initially implemented a `robots.txt` on our preview sites disallowing access. This reduced the number of crawls on these sites from 80% to 20%.

To further reduce crawlers from accessing incomplete information, we promptly implemented a [security rule](https://developers.cloudflare.com/security/rules/) to completely block all access to our preview sites, which simply reduced the number of crawls on our preview sites from 20% to 0% (the security rule completely blocked access to our preview sites).

This change likely has significantly improved the accuracy of information returned to users, improving their documentation experience.

This case study highlights the importance of understanding and controlling how AI crawlers interact with our documentation sites, as documentation is a source of truth that influences AI accuracy, relevancy, and customer experiences (even if it is through a third-party application, like ChatGPT, Gemini, or Claude).

## Action points

* Identify pages you want AI crawlers to access.
* If you wish to guide AI crawlers, use `robots.txt` to instruct AI crawlers.
* If you wish to completely block access to certain pages from AI crawlers, use security controls such as Cloudflare's [AI Crawl Control](https://developers.cloudflare.com/ai-crawl-control/), [bot solutions](https://developers.cloudflare.com/bots/), or some other security tool to completely block access to certain pages.
* If your documentation site generates preview sites, make sure these sites are not being accessed by AI crawlers to improve your end user experience.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.DMYpXs3t.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/how-we-docs/how-we-ai/control-ai-crawls/#page","headline":"Control how AI crawls your docs · Cloudflare Style Guide","description":"Manage AI crawler access to documentation.","url":"https://developers.cloudflare.com/style-guide/how-we-docs/how-we-ai/control-ai-crawls/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-24","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
