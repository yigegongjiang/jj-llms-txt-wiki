# Fumadocs UI (the default theme of Fumadocs): Graph View

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/ui/components/graph-view.mdx

A graph of all pages.

## Installation [#installation]

You can install this from CLI:

```npm
npx @fumadocs/cli add graph-view
```

Enable `extractLinkReferences` on Fumadocs MDX.

```ts title="source.config.ts"
import { defineDocs } from 'fumadocs-mdx/config';

export const docs = defineDocs({
  docs: {
    postprocess: {
      // [!code ++]
      extractLinkReferences: true,
    },
  },
});
```

## Usage [#usage]

You can use it in MDX files or the layout components (e.g. in `page.tsx`):

```tsx title="page.tsx"
import { GraphView } from '@/components/graph-view';
import { buildGraph } from '@/lib/build-graph';

export function PageBody() {
  return (
    <div>
      <GraphView graph={buildGraph()} />
      {/* ... */}
    </div>
  );
}
```
