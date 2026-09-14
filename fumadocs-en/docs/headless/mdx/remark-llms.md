# Fumadocs Core (the core library of Fumadocs): Remark LLMs

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/headless/mdx/remark-llms.mdx

Generate markdown for LLM consumption with customizable placeholders

The `remarkLLMs` plugin stringifies the processed Markdown AST to plain Markdown, the output is generated as an ESM export. This is useful for feeding content to LLMs, search indexing, or other text-based processing.

## Usage [#usage]

### Fumadocs MDX [#fumadocs-mdx]

Enable `includeProcessedMarkdown` in your collection config, Fumadocs MDX runs `remarkLLMs` internally when this is set.

```ts title="source.config.ts"
import { defineDocs } from 'fumadocs-mdx/config';
import { type LLMsOptions } from 'fumadocs-core/mdx-plugins/remark-llms';

const llmsOptions: LLMsOptions = {
  // options...
};

export default defineDocs({
  docs: {
    postprocess: {
      includeProcessedMarkdown: llmsOptions,
    },
  },
});
```

The stringified Markdown is exported as `_markdown` and available via `getText('processed')`, see [Collections](/docs/mdx/collections#includeprocessedmarkdown) for details.

### MDX Compiler [#mdx-compiler]

When using the MDX Compiler (or a custom pipeline), add the plugin:

```ts
import { compile } from '@mdx-js/mdx';
import { remarkLLMs, type LLMsOptions } from 'fumadocs-core/mdx-plugins/remark-llms';

const llmsOptions: LLMsOptions = {
  // The output will be exported as `_markdown`
  as: '_markdown',
};

const vfile = await compile('...', {
  remarkPlugins: [[remarkLLMs, llmsOptions]],
});
```

## Output [#output]

The `output` option controls the form of the export:

- `string` (default): the stringified Markdown.
- `function`: a component. Markdown content is still stringified at compile time, while JSX elements stay as JSX, receiving their original props.

```ts
import { type LLMsOptions } from 'fumadocs-core/mdx-plugins/remark-llms';

const llmsOptions: LLMsOptions = {
  // [!code ++]
  output: 'function',
};
```

With Fumadocs MDX, pass your MDX components to `getText('processed')`, it renders the component for you:

```ts
import { getMDXComponents } from '@/components/mdx';

const text = await page.data.getText('processed', {
  components: getMDXComponents(),
});
```

Otherwise, render it with `renderToMarkdown` from `fumadocs-core/server`, elements resolve from `props.components`:

```tsx
import { renderToMarkdown } from 'fumadocs-core/server';

const text = await renderToMarkdown(<Markdown components={{ Callout }} />);
```

A component defines its own Markdown form by calling `asMarkdown()` during render:

```tsx
import { asMarkdown, md } from 'fumadocs-core/server';

function Callout({ title, children }) {
  if (asMarkdown()) return md.linePrefix('> ')`**${title}**\n${children}`;

  return <div className="...">...</div>;
}
```

Components that don't opt in (including client components) are serialized as JSX syntax. See [Markdown Rendering](/docs/headless/utils/markdown) for details.

## Placeholders [#placeholders]

> Placeholders apply to the `string` output.

Placeholders let you keep certain MDX components in the stringified output as special tokens instead of dropping or inlining them. That way you can:

1. Rehydrate those tokens later via `renderPlaceholder()`.
2. Generate a better Markdown representation with runtime data.

### Using `mdxAsPlaceholder` [#using-mdxasplaceholder]

List component names to treat as placeholders:

```ts
import { type LLMsOptions } from 'fumadocs-core/mdx-plugins/remark-llms';

const llmsOptions: LLMsOptions = {
  // [!code ++]
  mdxAsPlaceholder: ['Callout', 'Card'],
};
```

In the exported Markdown, `<Callout>` and `<Card>` will appear as placeholder tokens, like:

```
\0{"name": "Callout", ...}\0
```

### Using `stringify` [#using-stringify]

For full control, use the `stringify` option and with the `placeholder()` helper:

```ts
import { placeholder, type LLMsOptions } from 'fumadocs-core/mdx-plugins/remark-llms';

const llmsOptions: LLMsOptions = {
  stringify(node, parent, state, info) {
    if (node.type === 'mdxJsxFlowElement' && node.name === 'MyPage') {
      return placeholder(node, parent, state, info);
    }
  },
};
```

### Rendering Placeholders [#rendering-placeholders]

Use `renderPlaceholder()` from the runtime module to replace placeholder tokens with custom output:

```ts
import { renderPlaceholder } from 'fumadocs-core/mdx-plugins/remark-llms.runtime';

const markdown = await getExportedMarkdown(); // e.g. from _markdown

const rendered = await renderPlaceholder(markdown, {
  async Callout({ name, attributes, children }) {
    // note: you can also fetch data here
    const data = await fetchData();
    return `[${attributes.type ?? 'info'}] ${children}`;
  },
  Card({ attributes, children }) {
    return `**${attributes.title}**\n\n${children}`;
  },
});
```

Each renderer receives `PlaceholderData`:

- `name`
- `attributes`
- `children` (the stringified child content).

## Example [#example]

```mdx tab="Input"
# Getting started

<Callout type="tip">Hello **world**.</Callout>

Some paragraph.
```

```md tab="Output"
# Getting started [#getting-started]

\0{"name":"Callout","attributes":{"type":"tip"},"children":"Hello **world**."}\0

Some paragraph.
```

## Options [#options]

### $Fumadocs

| Prop                   | Type       | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ---------------------- | ---------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `filterMdxAttributes?` | `function` | Filter the attributes to stringify.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `as?`                  | `string`   | export name for output Markdown. Default: `_markdown`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `headingIds?`          | `union`    | Explicit heading IDs in output. Default: `true`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `filterElement?`       | `function` | Filter the elements to be included in the output: - `true`: include element & its children. - `children-only`: exclude element but keep its children. - `false`: exclude element & its children. Default: ```ts filterElement = (node) => { switch (node.type) { case 'mdxjsEsm': return false; default: return true; } }, ```                                                                                                                                                                                                                                                                                                                                                                                                    |
| `mdxAsPlaceholder?`    | `array`    | Tag names of MDX components to be stringified as `placeholder()`, you can also use `placeholder()` directly in `stringify` callback. Ignored with `output: 'function'`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `output?`              | `union`    | Form of the export: - `string`: the output Markdown. - `function`: a component: Markdown content is still stringified at compile time, while JSX elements are kept as JSX, receiving their original props and resolving from `props.components`. Render it with `renderToMarkdown` from `fumadocs-core/server`, where a component can call `asMarkdown()` to define its own Markdown form. Default: `string`                                                                                                                                                                                                                                                                                                                      |
| `_data?`               | `union`    |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `bulletOther?`         | `union`    | Marker to use in certain cases where the primary bullet doesn’t work (default: `'-'` when `bullet` is `'*'`, `'*'` otherwise). Cannot be equal to `bullet`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `bulletOrdered?`       | `union`    | Marker to use for bullets of items in ordered lists (default: `'.'`). There is one case where the primary bullet for ordered items cannot be used: * when two ordered lists appear next to each other: `1. a\n2) b`; to solve that, `'.'` will be used when `bulletOrdered` is `')'`, and `'.'` otherwise                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `bullet?`              | `union`    | Marker to use for bullets of items in unordered lists (default: `'*'`). There are three cases where the primary bullet cannot be used: * when three or more list items are on their own, the last one is empty, and `bullet` is also a valid `rule`: `* - +`; this would turn into a thematic break if serialized with three primary bullets; `bulletOther` is used for the last item * when a thematic break is the first child of a list item and `bullet` is the same character as `rule`: `- ***`; this would turn into a single thematic break if serialized with primary bullets; `bulletOther` is used for the item * when two unordered lists appear next to each other: `* a\n- b`; `bulletOther` is used for such lists |
| `closeAtx?`            | `union`    | Whether to add the same number of number signs (`#`) at the end of an ATX heading as the opening sequence (default: `false`).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `emphasis?`            | `union`    | Marker to use for emphasis (default: `'*'`).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `extensions?`          | `union`    | List of extensions to include (optional). Each `ToMarkdownExtension` is an object with the same interface as `Options` here.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `fences?`              | `union`    | Whether to use fenced code always (default: `true`). The default is to use fenced code if there is a language defined, if the code is empty, or if it starts or ends in blank lines.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `fence?`               | `union`    | Marker to use for fenced code (default: ``'`'``).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `handlers?`            | `union`    | Handle particular nodes (optional). Each key is a node type, each value its corresponding handler.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `incrementListMarker?` | `union`    | Whether to increment the counter of ordered lists items (default: `true`).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `join?`                | `union`    | How to join blocks (optional).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `listItemIndent?`      | `union`    | How to indent the content of list items (default: `'one'`). Either with the size of the bullet plus one space (when `'one'`), a tab stop (`'tab'`), or depending on the item and its parent list (`'mixed'`, uses `'one'` if the item and list are tight and `'tab'` otherwise).                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `quote?`               | `union`    | Marker to use for titles (default: `'"'`).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `resourceLink?`        | `union`    | Whether to always use resource links (default: `false`). The default is to use autolinks (`<https://example.com>`) when possible and resource links (`[text](url)`) otherwise.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `ruleRepetition?`      | `union`    | Number of markers to use for thematic breaks (default: `3`).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `ruleSpaces?`          | `union`    | Whether to add spaces between markers in thematic breaks (default: `false`).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `rule?`                | `union`    | Marker to use for thematic breaks (default: `'*'`).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `setext?`              | `union`    | Whether to use setext headings when possible (default: `false`). The default is to always use ATX headings (`# heading`) instead of setext headings (`heading\n=======`). Setext headings cannot be used for empty headings or headings with a rank of three or more.                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `strong?`              | `union`    | Marker to use for strong (default: `'*'`).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `tightDefinitions?`    | `union`    | Whether to join definitions without a blank line (default: `false`). The default is to add blank lines between any flow (“block”) construct. Turning this option on is a shortcut for a join function like so: ```js function joinTightDefinitions(left, right) { if (left.type === 'definition' && right.type === 'definition') { return 0 } } ```                                                                                                                                                                                                                                                                                                                                                                               |
| `unsafe?`              | `union`    | Schemas that define when characters cannot occur (optional).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `stringify?`           | `function` | override default stringifier                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
