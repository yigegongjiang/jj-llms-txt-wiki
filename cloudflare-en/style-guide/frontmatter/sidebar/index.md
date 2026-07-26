---
description: Configuring how folders and pages appear in the sidebar.
title: Sidebar
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# Sidebar

Last updated Apr 24, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/frontmatter/sidebar/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

## Labels

Labels are controlled by frontmatter properties on a given page, which vary depending on if you are configuring a group or a link.

### Links

In order of precedence:

1. `sidebar.label`
2. `title`

#### On an index page

Index page labels default to `Overview` if `sidebar.label` is not defined.

`title` is not taken into consideration due to `title` being used in group labelling.

### Groups

In order of precedence:

1. `sidebar.group.label`
2. `title`

### Example

For example, given the following pages:

```mdx
---
title: Bar
sidebar:
  label: IndexTitle
  group:
    label: GroupTitle
---
```

```mdx
---
title: Baz
sidebar:
  label: PageTitle
---
```

The sidebar structure will look like:

* GroupTitle  
  * IndexTitle
  * PageTitle

If we remove the `sidebar` property from both, it will now look like this:

* Bar  
  * Overview
  * Baz

## Ordering

Both links and groups use the `sidebar.order` frontmatter property to configure their ordering, where groups are ordered based on the index page's order.

If `sidebar.order` is not specified, it will fallback to alphabetical ordering.

For example, given the following pages:

```mdx
---
title: Alpha
sidebar:
  order: 3
---
```

```mdx
---
title: Beta
sidebar:
  order: 2
---
```

The sidebar structure will look like:

* Beta  
  * ...
* Alpha  
  * ...

If we remove the `sidebar` property from both, it will now look like this:

* Alpha  
  * ...
* Beta  
  * ...

## Hiding pages

There are three properties that can be used for hiding pages from the sidebar.

### Hiding individual pages

#### `hidden`

This property should only be used when the page is **not** an index page for a group.

```mdx
---
title: Placeholder
sidebar:
  hidden: true
---
```

#### `group.hideIndex`

Since index pages are relied on to configure the label and sort order of groups, we have a special property that still makes the page available to our sidebar component and allows us to remove it after labelling and ordering groups.

```mdx
---
title: Placeholder
sidebar:
  group:
    hideIndex: true
---

import { DirectoryListing } from "~/components";

<DirectoryListing />
```

Note

Since these pages are still accessible via other links and directly navigating to the URL, always include a `DirectoryListing` component within the page content.

### Hiding child pages of a group

To make a group render as if it was a single page, which links to the index page, use the top-level `hideChildren` property.

## Badges

### Links

To specify a badge next to the link, use the `sidebar.badge` property.

```mdx
---
title: Example
sidebar:
  badge: New!
---
```

* Examples  
  * Example \[New!\]

### Groups

To specify a badge next to the group label, use the `sidebar.group.badge` inside the group's `index.mdx` frontmatter.

```mdx
---
title: Examples
sidebar:
  group:
    badge: New!
---
```

* Examples \[New!\]  
  * Example

### Automatic "Beta" badges

A "Beta" badge is automatically added to sidebar links and groups whose URL matches a directory entry with a "Beta" availability status. This badge is **not** controlled by frontmatter — it is derived from the product availability data associated with the entry in `src/content/directory/`.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.DMYpXs3t.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/frontmatter/sidebar/#page","headline":"Sidebar · Cloudflare Style Guide","description":"Configuring how folders and pages appear in the sidebar.","url":"https://developers.cloudflare.com/style-guide/frontmatter/sidebar/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-24","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
