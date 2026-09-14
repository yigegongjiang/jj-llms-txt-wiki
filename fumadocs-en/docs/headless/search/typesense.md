# Fumadocs Core (the core library of Fumadocs): Typesense Search

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/headless/search/typesense.mdx

Integrate Typesense Search with Fumadocs

> This is a community-maintained integration.

## Setup [#setup]

### Install Dependencies [#install-dependencies]

```package-install
typesense typesense-fumadocs-adapter
```

### Start Typesense Server [#start-typesense-server]

You can either self-host the Typesense server or use their cloud service. Follow their [getting started guide](https://typesense.org/docs/guide/install-typesense.html) to set up your server and obtain the API key and server URL.

### Sync Dataset [#sync-dataset]

Export the search indexes by pre-rendering a static route.

```ts title="lib/export-search-indexes.ts"
import { source } from '@/lib/source';
import { findPath } from 'fumadocs-core/page-tree';
import type { DocumentRecord } from 'typesense-fumadocs-adapter';

export async function exportSearchIndexes() {
  const results: DocumentRecord[] = [];

  function isBreadcrumbItem(item: unknown): item is string {
    return typeof item === 'string' && item.length > 0;
  }

  for (const page of source.getPages()) {
    let breadcrumbs: string[] | undefined;
    const pageTree = source.getPageTree(page.locale);
    const path = findPath(
      pageTree.children,
      (node) => node.type === 'page' && node.url === page.url,
    );

    if (path) {
      breadcrumbs = [];
      path.pop();
      if (isBreadcrumbItem(pageTree.name)) {
        breadcrumbs.push(pageTree.name);
      }
      for (const segment of path) {
        if (!isBreadcrumbItem(segment.name)) continue;
        breadcrumbs.push(segment.name);
      }
    }

    results.push({
      _id: page.url,
      structured: page.data.structuredData,
      url: page.url,
      title: page.data.title,
      description: page.data.description,
      breadcrumbs,
      locale: page.locale,
    });
  }

  return results;
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

Create a script to sync search indexes:

```ts title="scripts/sync-content.ts"
import * as fs from 'node:fs';
import { sync, DocumentRecord } from 'typesense-fumadocs-adapter';
import { Client } from 'typesense';

const filePath = '<see below>';
const content = fs.readFileSync(filePath);

const records = JSON.parse(content.toString()) as DocumentRecord[];

const client = new Client({
  nodes: [{ url: 'YOUR_TYPESENSE_SERVER_URL' }],
  apiKey: 'YOUR_TYPESENSE_API_KEY_WITH_WRITE_ACCESS',
  connectionTimeoutSeconds: 60 * 15,
});

// update the collection settings and sync search indexes
void sync(client, {
  typesenseCollectionName: 'YOUR_COLLECTION_NAME',
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

Make sure to run the script after build:

```json title="package.json"
{
  "scripts": {
    "build": "... && bun scripts/sync-content.ts"
  }
}
```

> You can also integrate it with your CI/CD pipeline.

### Search UI [#search-ui]

To implement the search UI, you can either:

- Use [Fumadocs UI search dialog](/docs/search/typesense).
- Build your own search UI with the Typesense search client hook.

  ```ts
  import { Client } from 'typesense';
  import { useTypesenseSearch } from 'typesense-fumadocs-adapter/client';

  const client = new Client({
    nodes: [{ url: 'YOUR_TYPESENSE_SERVER_URL' }],
    apiKey: 'YOUR_TYPESENSE_SEARCH_ONLY_API_KEY',
  });

  const { search, setSearch, query } = useTypesenseSearch({
    typesenseCollectionName: 'YOUR_COLLECTION_NAME',
    client,
  });
  ```

## Options [#options]

### Tag Filter [#tag-filter]

To configure tag filtering, add a `tag` value to indexes.

```ts title="lib/export-search-indexes.ts"
import { source } from '@/lib/source';
import type { DocumentRecord } from 'typesense-fumadocs-adapter';

export async function exportSearchIndexes() {
  const results: DocumentRecord[] = [];

  for (const page of source.getPages()) {
    results.push({
      // other fields...
      // [!code ++]
      tag: '<your value>',
    });
  }

  return results;
}
```

And update your search client:

- **Fumadocs UI**: Enable [Tag Filter](/docs/search/typesense#tag-filter) on Search UI.
- **Search Client**: You can add the tag filter like:

  ```ts
  import { useTypesenseSearch } from 'typesense-fumadocs-adapter/client';
  const { search, setSearch, query } = useTypesenseSearch({
    tag: '<your tag value>',
    // ...
  });
  ```

### Internationalization (i18n) [#internationalization-i18n]

To support internationalization, make sure to add a `locale` value to each document.

```ts title="lib/export-search-indexes.ts"
import { source } from '@/lib/source';
import { findPath } from 'fumadocs-core/page-tree';
import type { DocumentRecord } from 'typesense-fumadocs-adapter';

export async function exportSearchIndexes() {
  const results: DocumentRecord[] = [];

  for (const page of source.getPages()) {
    results.push({
      // other fields...
      // [!code ++]
      locale: page.locale,
    });
  }

  return results;
}
```

And update your search client:

```ts
import { useI18n } from 'fumadocs-ui/contexts/i18n';
import { useTypesenseSearch } from 'typesense-fumadocs-adapter/client';

const { locale } = useI18n();
const { search, setSearch, query } = useTypesenseSearch({
  // [!code ++]
  locale,
  // other fields...
});
```

Under the hood, Typesense will create separate collections for each locale. The collection name is appended with the `_{locale}` suffix (e.g., `YOUR_COLLECTION_NAME_en`, `YOUR_COLLECTION_NAME_fr`).
