# Fumadocs Core (the core library of Fumadocs): Algolia Search

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/headless/search/algolia.mdx

Integrate Algolia Search with Fumadocs

<Callout title="Notice">
  If you're using Algolia's free tier, you have to [display their logo on your search
  dialog](https://algolia.com/policies/free-services-terms).
</Callout>

## Setup [#setup]

Install dependencies:

```package-install
algoliasearch
```

### Sign up on Algolia [#sign-up-on-algolia]

Sign up and obtain the app id and API keys for your search. Store these
credentials in environment variables.

### Sync Search Indexes [#sync-search-indexes]

Pre-render a static route `/static.json` to export search indexes into production build:

```ts title="lib/export-search-indexes.ts"
import { source } from '@/lib/source';
import { toDocuments } from 'fumadocs-core/search/algolia';

export function exportSearchIndexes() {
  return toDocuments(source);
}
```

```ts tab="Next.js" title="app/static.json/route.ts"
import { exportSearchIndexes } from '@/lib/export-search-indexes';

export const revalidate = false;

export async function GET() {
  return Response.json(await exportSearchIndexes());
}
```

```ts tab="React Router" title="app/routes/static.ts"
import { exportSearchIndexes } from '@/lib/export-search-indexes';

export async function loader() {
  return Response.json(await exportSearchIndexes());
}
```

```ts tab="React Router" title="app/routes.ts"
import { route, type RouteConfig } from '@react-router/dev/routes';

export default [
  // [!code ++]
  route('static.json', 'routes/static.ts'),
] satisfies RouteConfig;
```

```ts tab="Tanstack Start" title="src/routes/static[.]json.ts"
import { createFileRoute } from '@tanstack/react-router';
import { exportSearchIndexes } from '@/lib/export-search-indexes';

export const Route = createFileRoute('/static.json')({
  server: {
    handlers: {
      GET: async () => Response.json(await exportSearchIndexes()),
    },
  },
});
```

```ts tab="Tanstack Start" title="vite.config.ts"
import { tanstackStart } from '@tanstack/react-start/plugin/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [
    // ...
    tanstackStart({
      prerender: {
        enabled: true,
      },
      // [!code ++] pre-render the static file
      pages: [{ path: '/static.json' }],
    }),
  ],
});
```

```ts tab="Waku" title="pages/_api/static.json.ts"
import { exportSearchIndexes } from '@/lib/export-search-indexes';

export async function GET() {
  return Response.json(await exportSearchIndexes());
}

export const getConfig = () => ({
  render: 'static',
});
```

Make a script to sync search indexes:

```ts title="scripts/sync-content.ts" twoslash
import { algoliasearch } from 'algoliasearch';
import { sync, DocumentRecord } from 'fumadocs-core/search/algolia';
import * as fs from 'node:fs';

const filePath = '<see below>';
const content = fs.readFileSync(filePath);

const records = JSON.parse(content.toString()) as DocumentRecord[];

const client = algoliasearch('id', 'key');

// update the index settings and sync search indexes
void sync(client, {
  indexName: 'document',
  documents: records,
});
```

`filePath` refers to the path of pre-rendered `static.json`, choose one according to your setup:

<section id="static-route-output">

```ts tab="Next.js"
const filePath = '.next/server/app/static.json.body';
```

```ts tab="Tanstack Start"
const filePath = '.output/public/static.json';
```

```ts tab="React Router"
const filePath = 'build/client/static.json';
```

```ts tab="Waku"
const filePath = 'dist/public/static.json';
```

</section>

Now run the script after build:

```json title="package.json"
{
  "scripts": {
    "build": "... && bun ./scripts/sync-content.ts"
  }
}
```

### Workflow [#workflow]

You may manually upload search indexes with the script, or integrate it with your CI/CD pipeline.

### Search UI [#search-ui]

You can consider different options for implementing the UI:

- Using [Fumadocs UI search dialog](/docs/search/algolia).
- Build your own using the built-in search client hook:

  ```ts twoslash
  import { liteClient } from 'algoliasearch/lite';
  import { useDocsSearch } from 'fumadocs-core/search/client';
  import { algoliaClient } from 'fumadocs-core/search/client/algolia';

  const algolia = liteClient('id', 'key');

  const { search, setSearch, query } = useDocsSearch({
    client: algoliaClient({
      indexName: 'document',
      client: algolia,
    }),
  });
  ```

- Use their official clients directly.

## Advanced [#advanced]

### Tag Filter [#tag-filter]

To configure tag filtering, add a `tag` value to indexes.

```ts title="lib/export-search-indexes.ts"
import { source } from '@/lib/source';
import { toDocuments } from 'fumadocs-core/search/algolia';

export function exportSearchIndexes() {
  // [!code ++]
  return toDocuments(source, { tag: (page) => page.slugs[0] });
}
```

And update your search client:

- **Fumadocs UI**: Enable [Tag Filter](/docs/search/algolia#tag-filter) on Search UI.
- **Search Client**: You can add the tag filter like:

  ```ts
  import { useDocsSearch } from 'fumadocs-core/search/client';
  import { algoliaClient } from 'fumadocs-core/search/client/algolia';

  const { search, setSearch, query } = useDocsSearch({
    client: algoliaClient({
      tag: '<your tag value>',
      // ...
    }),
  });
  ```

The `tag` field is an attribute for faceting. You can also use the filter `tag:value` on Algolia search clients.

### Under the Hood [#under-the-hood]

The Algolia Integration automatically configures Algolia Search for document search.

It creates a record for **each paragraph** in your document, it is also recommended by Algolia.

Each record contains searchable attributes:

| Attribute | Description           |
| --------- | --------------------- |
| `title`   | Page Title            |
| `section` | Heading ID (nullable) |
| `content` | Paragraph content     |

The `section` field only exists in paragraphs under a heading. Headings and
paragraphs are indexed as an individual record, grouped by their page ID.

Notice that it expects the `url` property of a page to be unique, you shouldn't have two pages with the same
url.
