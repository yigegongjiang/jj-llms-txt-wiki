# Fumadocs Core (the core library of Fumadocs): Content Collections

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/headless/content-collections/index.mdx

Use Content Collections for Fumadocs

[Content Collections](https://www.content-collections.dev) is a library that transforms your content into type-safe data collections.

## Setup [#setup]

Follow their [official guide](https://content-collections.dev/docs) to setup Content Collections for your React framework.

Make sure you have MDX configured:

```npm
npm i @content-collections/mdx
```

To integrate with Fumadocs, add the following to your `content-collections.ts`.

```ts title="content-collections.ts"
import { defineCollection, defineConfig } from '@content-collections/core';
import { transformMDX } from '@fumadocs/content-collections/configuration';
import { metaSchema, pageSchema } from 'fumadocs-core/source/schema';

const docs = defineCollection({
  name: 'docs',
  directory: 'content/docs',
  include: '**/*.mdx',
  schema: pageSchema,
  transform: transformMDX,
});

const metas = defineCollection({
  name: 'meta',
  directory: 'content/docs',
  include: '**/meta.json',
  parser: 'json',
  schema: metaSchema,
});

export default defineConfig({
  collections: [docs, metas],
});

```

And pass it to `loader()`.

```ts title="lib/source.ts"
import { allDocs, allMetas } from 'content-collections';
import { loader } from 'fumadocs-core/source';
import { createMDXSource } from '@fumadocs/content-collections';

export const source = loader({
  baseUrl: '/docs',
  source: createMDXSource(allDocs, allMetas),
});

```

Done! You can access the pages and generated page tree from Source API.

```ts
import { getPage } from '@/lib/source';

const page = getPage(slugs);

// MDX output
page?.data.body;

// Table of contents
page?.data.toc;

// Structured Data, for Search API
page?.data.structuredData;
```

### MDX Options [#mdx-options]

You can customize MDX options in the `transformMDX` function.

```ts
import { defineCollection } from '@content-collections/core';
import { transformMDX } from '@fumadocs/content-collections/configuration';

const docs = defineCollection({
  transform: (document, context) =>
    transformMDX(document, context, {
      // options here
    }),
});
```

### Import Components [#import-components]

To use components from other packages like Fumadocs UI, pass them to your `<MDXContent />` component.

```tsx
import { MDXContent } from '@content-collections/mdx/react';
import { getMDXComponents } from '@/components/mdx';

return <MDXContent code={page.data.body} components={getMDXComponents()} />;
```

You can also import them in MDX Files, but it is not recommended.

<Callout title='Deep Dive: Why?'>
    Content Collections uses `mdx-bundler` to bundle MDX files.

    To support importing a package from node modules, Fumadocs added a default value to the `cwd` option of MDX Bundler.
    It works good, but we still **do not** recommend importing components in MDX files.

    Reasons:

    - It requires esbuild to bundle these components, while it should be done by the framework's bundler (e.g. Vite or Turbopack)
    - You can refactor the import path of components without changing your MDX files.
    - With Remote Sources, it doesn't make sense to add an import in MDX files.

</Callout>
