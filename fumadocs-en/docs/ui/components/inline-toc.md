# Fumadocs UI (the default theme of Fumadocs): Inline TOC

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/ui/components/inline-toc.mdx

Add Inline TOC into your documentation

<Installation name="inline-toc" />

## Usage [#usage]

You can use it in your MDX content:

```mdx title="content.mdx"
import { InlineTOC } from 'fumadocs-ui/components/inline-toc';

<InlineTOC items={toc}>Table of Contents</InlineTOC>
```

Or adding it to every page.

```tsx title="page.tsx"
import { DocsPage } from 'fumadocs-ui/layouts/docs/page';
import { InlineTOC } from 'fumadocs-ui/components/inline-toc';

export default function Page() {
  // ...
  return (
    <DocsPage>
      {/* [!code ++] */}
      <InlineTOC items={page.data.toc}>Table of Contents</InlineTOC>
    </DocsPage>
  );
}
```

## Reference [#reference]

### InlineTOCProps

| Prop            | Type       | Description                                                                                                                                                                               |
| --------------- | ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `render?`       | `union`    | Allows you to replace the component's HTML element with a different tag, or compose it with another component. Accepts a `ReactElement` or a function that returns the element to render. |
| `open?`         | `union`    | Whether the collapsible panel is currently open. To render an uncontrolled collapsible, use the `defaultOpen` prop instead.                                                               |
| `defaultOpen?`  | `union`    | Whether the collapsible panel is initially open. To render a controlled collapsible, use the `open` prop instead. Default: `false`                                                        |
| `onOpenChange?` | `function` | Event handler called when the panel is opened or closed.                                                                                                                                  |
| `disabled?`     | `union`    | Whether the component should ignore user interaction. Default: `false`                                                                                                                    |
| `items`         | `array`    |                                                                                                                                                                                           |
