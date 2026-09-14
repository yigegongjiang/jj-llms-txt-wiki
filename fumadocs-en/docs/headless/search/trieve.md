# Fumadocs Core (the core library of Fumadocs): Trieve Search

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/headless/search/trieve.mdx

Integrate Trieve Search with Fumadocs

> This is a community maintained integration.

## Introduction [#introduction]

The Trieve Integration automatically configures Trieve Search for site search.

By default, it creates a chunk for **each paragraph** in your document, it is
officially recommended by Trieve.

## Setup [#setup]

### Install Dependencies [#install-dependencies]

```package-install
trieve-ts-sdk trieve-fumadocs-adapter
```

### Sign up on Trieve [#sign-up-on-trieve]

Sign up and create a dataset. Then obtain 2 API keys where one has only read access and the other has admin access to create and delete chunks.
Store these credentials in environment variables.

<Callout title="Notice">
  One API Key should have only read access for the public facing search and the other should have
  admin access to create and delete chunks.
</Callout>

### Sync Dataset [#sync-dataset]

Export the search indexes by pre-rendering a static route.

```ts title="lib/export-search-indexes.ts"
import { source } from '@/lib/source';
import { type TrieveDocument } from 'trieve-fumadocs-adapter/search/sync';

export async function exportSearchIndexes() {
  const results: TrieveDocument[] = [];

  for (const page of source.getPages()) {
    results.push({
      _id: page.url,
      structured: page.data.structuredData,
      url: page.url,
      title: page.data.title,
      description: page.data.description,
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

Create a script, the `sync` function will sync search indexes.

```ts title="scripts/sync-content.ts"
import * as fs from 'node:fs';
import { sync, type TrieveDocument } from 'trieve-fumadocs-adapter/search/sync';
import { TrieveSDK } from 'trieve-ts-sdk';

const filePath = '<see below>';
const content = fs.readFileSync(filePath);

const records: TrieveDocument[] = JSON.parse(content.toString());

const client = new TrieveSDK({
  apiKey: 'adminApiKey',
  datasetId: 'datasetId',
});

sync(client, records);
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

You can use their `SearchDialog` component:

```tsx title="components/search.tsx"
'use client';
import type { SharedProps } from 'fumadocs-ui/components/dialog/search';
import SearchDialog from 'trieve-fumadocs-adapter/components/dialog/search';
import { TrieveSDK } from 'trieve-ts-sdk';

const trieveClient = new TrieveSDK({
  apiKey: 'readOnlyApiKey',
  datasetId: 'datasetId',
});

export default function CustomSearchDialog(props: SharedProps) {
  return <SearchDialog trieveClient={trieveClient} {...props} />;
}
```

1. Replace `apiKey` and `datasetId` with your desired values.

2. Replace the default search dialog with your new one.

### Search Client [#search-client]

Add the `useTrieveSearch` hook:

```ts
import { TrieveSDK } from 'trieve-ts-sdk';
import { useTrieveSearch } from 'trieve-fumadocs-adapter/search/trieve';

const client = new TrieveSDK({
  apiKey: 'readOnlyApiKey',
  datasetId: 'datasetId',
});

const { search, setSearch, query } = useTrieveSearch(client);
```

## Options [#options]

### Tag Filter [#tag-filter]

To configure tag filtering, add a `tag` value to indexes.

```js
import { sync } from 'trieve-fumadocs-adapter/search/sync';
import { TrieveSDK } from 'trieve-ts-sdk';

const client = new TrieveSDK({
  apiKey: 'adminApiKey',
  datasetId: 'datasetId',
});

const documents = records.map((index) => ({
  ...index,
  tag: 'value', // [!code highlight]
}));

sync(client, documents);
```

#### Search UI [#search-ui-1]

Enable Tag Filter.

```tsx title="components/search.tsx"
import SearchDialog from 'trieve-fumadocs-adapter/components/dialog/search';

<SearchDialog
  defaultTag="value"
  tags={[
    {
      name: 'Tag Name',
      value: 'value',
    },
  ]}
/>;
```

#### Search Client [#search-client-1]

The `tag_set` field is an attribute for filtering. To filter indexes by tag, use the filter on Trieve search clients.

```json
{
  "must": [
    {
      "field": "tag_set",
      "match": ["value"]
    }
  ]
}
```

Or with `useTrieveSearch` hook:

```ts
import { TrieveSDK } from 'trieve-ts-sdk';
import { useTrieveSearch } from 'trieve-fumadocs-adapter/search/trieve';

const client = new TrieveSDK({
  apiKey: 'readOnlyApiKey',
  datasetId: 'datasetId',
});

const { search, setSearch, query } = useTrieveSearch(client, undefined, '<your tag value>');
```
