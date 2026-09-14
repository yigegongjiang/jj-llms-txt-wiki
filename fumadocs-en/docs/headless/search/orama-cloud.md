# Fumadocs Core (the core library of Fumadocs): Orama Cloud

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/headless/search/orama-cloud.mdx

Integrate with Orama Cloud

To begin, create an account on Orama Cloud.

## REST API [#rest-api]

REST API integration requires your docs to upload the indexes.

1. Create a new project with **REST API** data source on Orama Cloud dashboard.

   Store your credentials in environment variables, for example:

   ```dotenv
   NEXT_PUBLIC_ORAMA_DATASOURCE_ID="Rest API data source ID"
   NEXT_PUBLIC_ORAMA_PROJECT_ID="project ID"
   NEXT_PUBLIC_ORAMA_API_KEY="public API key"

   ORAMA_PRIVATE_API_KEY="private API key"
   ```

2. Export the search indexes by pre-rendering a static route.

   ```ts title="lib/export-search-indexes.ts"
   import { source } from '@/lib/source';
   import { toDocuments } from 'fumadocs-core/search/orama-cloud';

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

3. Create a script to sync search indexes.

   ```ts title="scripts/sync-content.ts"
   import { sync, type OramaDocument } from 'fumadocs-core/search/orama-cloud';
   import * as fs from 'node:fs/promises';
   import { OramaCloud } from '@orama/core';

   const filePath = '<see below>';

   async function main() {
     const orama = new OramaCloud({
       projectId: process.env.NEXT_PUBLIC_ORAMA_PROJECT_ID,
       apiKey: process.env.ORAMA_PRIVATE_API_KEY,
     });

     const content = await fs.readFile(filePath);
     const records = JSON.parse(content.toString()) as OramaDocument[];

     await sync(orama, {
       index: process.env.NEXT_PUBLIC_ORAMA_DATASOURCE_ID,
       documents: records,
     });

     console.log(`search updated: ${records.length} records`);
   }

   void main();
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

4. Run the script after production build, make sure the environment variables are available (e.g. Bun reads from `.env` files):

   ```json title="package.json"
   {
     "scripts": {
       "build": "... && bun scripts/sync-content.ts"
     }
   }
   ```

### Search Client [#search-client]

To search documents on the client side, consider:

- Using [Fumadocs UI search dialog](/docs/search/orama-cloud).
- Custom search UI using the built-in hook of Fumadocs:

  ```ts
  import { useDocsSearch } from 'fumadocs-core/search/client';
  import { oramaCloudClient } from 'fumadocs-core/search/client/orama-cloud';
  import { OramaCloud } from '@orama/core';

  const orama = new OramaCloud({
    projectId: process.env.NEXT_PUBLIC_ORAMA_PROJECT_ID,
    apiKey: process.env.NEXT_PUBLIC_ORAMA_API_KEY,
  });

  const { search, setSearch, query } = useDocsSearch({
    client: oramaCloudClient({
      client: orama,
      params: {
        // optional search params
      },
    }),
  });
  ```

- Use their search client directly.

## Web Crawler [#web-crawler]

1. Create a Crawler data source from dashboard, and configure it correctly with the "Documentation" preset.
2. Copy the credentials from dashboard.

### Search Client [#search-client-1]

Same as REST API integration, but make sure to set `index` to `crawler`.

```ts
import { useDocsSearch } from 'fumadocs-core/search/client';
import { oramaCloudClient } from 'fumadocs-core/search/client/orama-cloud';
import { OramaCloud } from '@orama/core';

const orama = new OramaCloud({
  projectId: '<project id>',
  apiKey: '<read only api key>',
});

const { search, setSearch, query } = useDocsSearch({
  client: oramaCloudClient({
    index: 'crawler',
    client: orama,
    params: {
      // optional search params
    },
  }),
});
```

It's same for Fumadocs UI.
