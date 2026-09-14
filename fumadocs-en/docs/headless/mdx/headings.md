# Fumadocs Core (the core library of Fumadocs): Headings

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/headless/mdx/headings.mdx

Process headings from your document

## Remark Heading [#remark-heading]

Applies ids to headings.

```ts title="MDX Compiler"
import { compile } from '@mdx-js/mdx';
import { remarkHeading } from 'fumadocs-core/mdx-plugins';

await compile('...', {
  remarkPlugins: [remarkHeading],
});
```

> This plugin is included by default on Fumadocs MDX.

### Extract TOC [#extract-toc]

By default, it extracts the headings (table of contents) of a document to `vfile.data.toc`.
You can disable it with:

```ts
import { remarkHeading } from 'fumadocs-core/mdx-plugins';

export default {
  remarkPlugins: [[remarkHeading, { generateToc: false }]],
};
```

### Custom Ids [#custom-heading-id]

You can customize the heading id with `[#slug]`.

```md
# heading [#slug]
```

### Output [#output]

An array of `TOCItemType`.

### TOCItemType

| Prop     | Type        | Description                    |
| -------- | ----------- | ------------------------------ |
| `title`  | `ReactNode` |                                |
| `url`    | `string`    |                                |
| `depth`  | `number`    |                                |
| `_step?` | `number`    | [remark-steps] the step number |


## Rehype TOC [#rehype-toc]

Exports table of contents (an array of `TOCItemType`), it allows JSX nodes which is not possible with a Remark plugin.

> It requires MDX.js.

### Usage [#usage]

```ts
import { rehypeToc } from 'fumadocs-core/mdx-plugins';

export default {
  rehypePlugins: [rehypeToc],
};
```

### Output [#output-1]

For a Markdown document:

```md
## Hello `code`
```

An export will be created:

```jsx
export const toc = [
  {
    title: (
      <>
        Hello <code>code</code>
      </>
    ),
    depth: 2,
    url: '#hello-code',
  },
];
```
