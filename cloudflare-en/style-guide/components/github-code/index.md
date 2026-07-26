---
description: Fetch and display code from a GitHub repository.
title: GitHubCode
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/style-guide/llms.txt  
> Use this file to discover all available pages before exploring further.

# GitHubCode

Last updated Jun 18, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/style-guide/components/github-code/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

The `GitHubCode` component is used `25` times on `12` pages.

See all examples of pages that use GitHubCode

Used **25** times.

**Pages**

* [/agents/concepts/agentic-patterns/](https://developers.cloudflare.com/agents/concepts/agentic-patterns/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/agents/concepts/agentic-patterns/index.mdx)
* [/d1/best-practices/read-replication/](https://developers.cloudflare.com/d1/best-practices/read-replication/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/d1/best-practices/read-replication.mdx)
* [/d1/tutorials/d1-and-prisma-orm/](https://developers.cloudflare.com/d1/tutorials/d1-and-prisma-orm/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/d1/tutorials/d1-and-prisma-orm.mdx)
* [/kv/get-started/](https://developers.cloudflare.com/kv/get-started/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/kv/get-started.mdx)
* [/style-guide/how-we-docs/image-maintenance/](https://developers.cloudflare.com/style-guide/how-we-docs/image-maintenance/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/style-guide/how-we-docs/image-maintenance.mdx)
* [/style-guide/how-we-docs/metadata/](https://developers.cloudflare.com/style-guide/how-we-docs/metadata/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/style-guide/how-we-docs/metadata.mdx)
* [/style-guide/how-we-docs/redirects/](https://developers.cloudflare.com/style-guide/how-we-docs/redirects/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/style-guide/how-we-docs/redirects.mdx)
* [/style-guide/how-we-docs/reviews/](https://developers.cloudflare.com/style-guide/how-we-docs/reviews/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/style-guide/how-we-docs/reviews.mdx)
* [/workers/platform/infrastructure-as-code/](https://developers.cloudflare.com/workers/platform/infrastructure-as-code/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/workers/platform/infrastructure-as-code.mdx)
* [/workers/static-assets/direct-upload/](https://developers.cloudflare.com/workers/static-assets/direct-upload/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/workers/static-assets/direct-upload.mdx)
* [/workers/tutorials/deploy-an-express-app/](https://developers.cloudflare.com/workers/tutorials/deploy-an-express-app/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/workers/tutorials/deploy-an-express-app.mdx)
* [/workflows/examples/wait-for-event/](https://developers.cloudflare.com/workflows/examples/wait-for-event/)\-[Source](https://github.com/cloudflare/cloudflare-docs/blob/production/src/content/docs/workflows/examples/wait-for-event.mdx)

**Partials**

The `GitHubCode` component allows you to include files from Cloudflare repositories.

The remote content can be filtered by lines or a region enclosed in tags.

## Import

```mdx
import { GitHubCode } from "~/components";
```

## Usage

```mdx
import { GitHubCode } from "~/components";

<GitHubCode
    repo="cloudflare/workflows-starter"
    file="src/index.ts"
    commit="a844e629ec80968118d4b116d4b26f5dcb107137"
    lang="ts"
    useTypeScriptExample={true}
/>
```

### Filtering by lines

```mdx
import { GitHubCode } from "~/components";

{/*
import { foo } from "bar";

const baz = foo();

console.log(baz);
*/}
<GitHubCode
    repo="..."
    file="..."
    commit="..."
    lang="..."
    lines="1-3"
/>
{/*
import { foo } from "bar";

const baz = foo();
*/}
```

### Filtering by tag

```mdx
import { GitHubCode } from "~/components";

{/*
<docs-tag name="no-logging">
import { foo } from "bar";

const baz = foo();
</docs-tag name="no-logging">

console.log(baz);
*/}
<GitHubCode
    repo="..."
    file="..."
    commit="..."
    lang="..."
    tag="no-logging"
/>
{/*
import { foo } from "bar";

const baz = foo();
*/}
```

## `<GitHubCode>` Props

### `repo`

**required** **type:** `string`

The owner and repository to pull from, in the form of `cloudflare/<REPO-NAME>`

For example:

* `cloudflare/workers-rs`.
* `cloudflare/templates`.

### `file`

**required** **type:** `string`

The file path to pull from, in the form of `path/to/filename-including-extensions`. This path excludes the repo name.

For example:

* `templates/hello-world/src/lib.rs`.
* `d1-starter-sessions-api/src/index.ts`.

### `commit`

**required** **type:** `string`

The long (40-characters) Git commit hash to pull from, for example `ab3951b5c95329a600a7baa9f9bb1a7a95f1aeaa`.

### `lang`

**required** **type:** `string`

The language to use for the code block, for example `rs`.

### `useTypeScriptExample`

**type:** `boolean`

If the `lang` is `"ts"` and `useTypeScriptExample` is `true`, the [TypeScriptExample](https://developers.cloudflare.com/style-guide/components/typescript-example/) component will be used to provide a JavaScript tab as well.

### `lines`

**type:** `string`

A range of lines to filter the content using, for example `1-3`.

### `tag`

**type:** `string`

A region to filter the content with, for example `no-logging`.

This should be represented as starting `<docs-tag name="no-logging">` and closing `</docs-tag name="no-logging">` comments in the source file.

### `code`

**type**: `object`

Props to pass to the [Expressive Code component ↗](https://expressive-code.com/key-features/code-component/).

## Associated content types

* [Tutorial](https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/tutorial/)

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.DMYpXs3t.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/style-guide/components/github-code/#page","headline":"GitHubCode · Cloudflare Style Guide","description":"Fetch and display code from a GitHub repository.","url":"https://developers.cloudflare.com/style-guide/components/github-code/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-06-18","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
