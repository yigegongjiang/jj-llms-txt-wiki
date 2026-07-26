---
description: Maintain and update documentation images.
title: Image maintenance
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Image maintenance

Last updated Jul 22, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/how-we-docs/image-maintenance/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

Though valuable for user understanding, images are difficult to maintain. We have a few strategies that we use to help make this easier.

## Guidelines

We support a few different types of images in our docs, including:

* [Diagrams](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/diagrams/)
* [Screenshots](https://developers.cloudflare.com/style-guide/documentation-content-strategy/component-attributes/screenshots/)

Of these, we prefer Mermaid diagrams because they are searchable and easily changeable. The "cost" of updating a Mermaid diagram is much lower than re-taking a screenshot or working with a designer to update a diagram.

## Maintenance

The best way to improve image maintenance is to avoid using them.

The other way to streamline maintenance is to remove images that are no longer referenced in your documentation. This pattern becomes particularly helpful if you need to audit images for UI changes or leaked information, because then you are not wasting time looking at unused images too.

We do that through a combination of GitHub actions.

### Flag unused images

We have a specific GitHub action to [flag unused images ↗](https://github.com/cloudflare/cloudflare-docs/blob/production/.github/workflows/image-audit.yml).

What the GitHub action does is:

1. Finds all `.png` or `.svg` files in our content.
2. Checks to see if those files are referenced in any of our MDX files.
3. Creates a [GitHub issue ↗](https://github.com/cloudflare/cloudflare-docs/issues/23343) if there are unreferenced files.

### Evaluate image paths

In combination with [flagging unused images](#flag-unused-images), we also have logic in our [build process ↗](https://github.com/cloudflare/cloudflare-docs/blob/production/astro.config.ts) to validate image paths.

```ts
export default defineConfig({
	site: "https://developers.cloudflare.com",
	markdown: {
		smartypants: false,
		remarkPlugins: [remarkValidateImages],
		rehypePlugins: [
			rehypeMermaid,
			rehypeExternalLinks,
			rehypeHeadingSlugs,
			rehypeAutolinkHeadings,
			// @ts-expect-error plugins types are outdated but functional
			rehypeTitleFigure,
			rehypeShiftHeadings,
		],
	},
```

This ensures that the build-time `nimbus/image-ref` lint rule validates all image paths. If the path does not exist, we throw an error and prevent the site from building.

When paired with [flagging unused images](#flag-unused-images), this path validation ensures that a tech writer can safely delete unused files in a pull request. So long as the site builds correctly, you have only deleted image files that are not referenced anywhere.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.DMYpXs3t.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/how-we-docs/image-maintenance/#page","headline":"Image maintenance · Cloudflare Style Guide","description":"Maintain and update documentation images.","url":"https://developers.cloudflare.com/style-guide/how-we-docs/image-maintenance/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-07-22","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
