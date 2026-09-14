# Fumadocs Core (the core library of Fumadocs): Get TOC

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/headless/utils/get-toc.mdx

Parse Table of contents from markdown/mdx content

Parse Table of contents from markdown/mdx content.

> [You can use the remark plugin directly](/docs/headless/mdx/headings)

## Usage [#usage]

Note: If you're using a CMS, you should use the API provided by the CMS instead.

```ts
import { getTableOfContents } from 'fumadocs-core/content/toc';

const toc = getTableOfContents('## markdown content');
```

### Output [#output]

An array of [`TOCItemType`](/docs/headless/mdx/headings#output) is returned.
