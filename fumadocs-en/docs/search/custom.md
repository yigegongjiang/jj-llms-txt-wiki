# Fumadocs (Framework Mode): Custom Search

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/(framework)/search/custom.mdx

For other search solutions.

## Overview [#overview]

Fumadocs is very flexible, you can integrate custom search with Fumadocs easily.

### Search Indexes [#search-indexes]

With Fumadocs MDX, the generated search indexes are exposed during runtime.

To expose search indexes statically, create a function to generate records.

```ts title="lib/export-search-indexes.ts"
import { source } from '@/lib/source';
import type { StructuredData } from 'fumadocs-core/mdx-plugins';

export interface DocumentRecord {
  title: string;
  description?: string;
  url: string;
  structured: StructuredData;
}

export async function exportSearchIndexes() {
  const results: DocumentRecord[] = [];

  for (const page of source.getPages()) {
    results.push({
      structured: page.data.structuredData,
      url: page.url,
      title: page.data.title,
      description: page.data.description,
    });
  }

  return results;
}
```

Export it via static route handler.

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

Finally, the exported records can be accessed at:

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

You can read it for further processing:

```ts
import * as fs from 'node:fs';
import type { DocumentRecord } from '@/lib/export-search-indexes';

const content = fs.readFileSync(filePath);
const records = JSON.parse(content.toString()) as DocumentRecord[];
```

Use `structuredData` to generate accurate search indexes.
