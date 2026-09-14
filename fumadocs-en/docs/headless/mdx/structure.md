# Fumadocs Core (the core library of Fumadocs): Remark Structure

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/headless/mdx/structure.mdx

Extract information from your documents, useful for implementing document search

## Usage [#usage]

Add it as a remark plugin.

```ts tab="MDX Compiler"
import { compile } from '@mdx-js/mdx';
import { remarkStructure } from 'fumadocs-core/mdx-plugins';

const vfile = await compile('...', {
  remarkPlugins: [remarkStructure],
});
```

```ts tab="Fumadocs MDX" title="source.config.ts"
import { defineConfig } from 'fumadocs-mdx/config';

export default defineConfig({
  mdxOptions: {
    // This plugin is enabled by default on Fumadocs MDX
    // options:
    remarkStructureOptions: {},
  },
});
```

Extracted information could be found in `vfile.data.structuredData`, you may
write your own plugin to convert it into a MDX export.

### Options [#options]

### StructureOptions

| Prop                    | Type       | Description                                                                                                                                                                                                                        |
| ----------------------- | ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `types?`                | `union`    | MDAST node types to be scanned as a content block. If a node's type is listed in this array, it will be converted into a single content block. Default: `['heading', 'paragraph', 'blockquote', 'tableCell', 'mdxJsxFlowElement']` |
| `stringify?`            | `union`    | stringify text content from a MDAST node.                                                                                                                                                                                          |
| `mdxTypes?`             | `function` | Whether the MDX element should be treated as a single content block, only effective if `types` has `mdxJsxFlowElement`. Default: return `true` if the element is a leaf node, otherwise `false`.                                   |
| `allowedMdxAttributes?` | `union`    | **Deprecated.**                                                                                                                                                                                                                    |
| `exportAs?`             | `union`    | export as `structuredData` (if true) or specified variable name.                                                                                                                                                                   |


### Output [#output]

A list of headings and contents. Paragraphs will be extracted to the `contents`
array, each item contains a `heading` prop indicating the heading of paragraph.

<Callout title="Note">A heading can have multiple paragraphs.</Callout>

#### Heading [#heading]

| Prop      |                                      |
| --------- | ------------------------------------ |
| `id`      | unique identifier or slug of heading |
| `content` | Text content (Markdown)              |

#### Content [#content]

| Prop      |                                 |
| --------- | ------------------------------- |
| `heading` | Heading of paragraph (nullable) |
| `content` | Text content (Markdown)         |

The content is stringified as Markdown text, you can customize it using the [`stringify`](#options) option.

## As a Function [#as-a-function]

Accepts MDX/markdown content and return structurized data.

```ts
import { structure } from 'fumadocs-core/mdx-plugins';

structure(page.body.raw);
```

<Callout title="Tip" className="mt-4">
If you have custom remark plugins enabled, such as
`remark-math`, you have to pass these plugins to the function. This avoids unreadable content on paragraphs.

```ts
import { structure } from 'fumadocs-core/mdx-plugins';
import remarkMath from 'remark-math';

structure(page.body.raw, [remarkMath]);
```

</Callout>

### Parameters [#parameters]

| Parameter       |                        |
| --------------- | ---------------------- |
| `content`       | MDX/markdown content   |
| `remarkPlugins` | List of remark plugins |
| `options`       | Custom options         |
