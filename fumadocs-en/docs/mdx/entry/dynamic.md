# Fumadocs MDX (the built-in content source): Dynamic Entry

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/mdx/entry/dynamic.mdx

Accessing your collections with on-demand compilation.

## Usage [#usage]

Enable [dynamic mode](/docs/mdx/async#dynamic-mode) on docs/doc collections, and import it from `dynamic` entry instead of `server`.

It offers the same output as `server` entry, while compiled properties have to be loaded via `load()` method.

```ts tab="Docs (Meta + Doc)"
// [!code highlight]
import { docs } from 'collections/dynamic';
import { loader } from 'fumadocs-core/source';

export const source = loader({
  baseUrl: '/docs',
  source: docs.toFumadocsSource(),
});
```

```ts tab="Doc"
// [!code highlight]
import { blogPosts } from 'collections/dynamic';
import { toFumadocsSource } from 'fumadocs-mdx/runtime/server';
import { loader } from 'fumadocs-core/source';

export const blog = loader({
  baseUrl: '/blog',
  source: toFumadocsSource(blogPosts, []),
});
```
