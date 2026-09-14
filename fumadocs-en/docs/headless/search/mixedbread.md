# Fumadocs Core (the core library of Fumadocs): Mixedbread

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/headless/search/mixedbread.mdx

Integrate Mixedbread Search with Fumadocs

## Introduction [#introduction]

The Mixedbread Integration uses vector search to provide semantic search capabilities for your documentation. It indexes your documentation content into a store, enabling users to search using natural language queries and find relevant content based on meaning rather than just keyword matching.

## Setup [#setup]

### Get your API Key [#get-your-api-key]

1. Sign up at [Mixedbread](https://platform.mixedbread.com)
2. Navigate to [API Keys](https://platform.mixedbread.com/platform?next=api-keys)
3. Create a new API key and store it in your environment variables

### Create a Store [#create-a-store]

To sync your documentation, you'll need to create a store:

1. Go to the [Stores](https://platform.mixedbread.com/platform?next=stores) in your Mixedbread dashboard
2. Create a new store for your documentation
3. Copy the store ID

### Sync Documentation [#sync-documentation]

Use the [Mixedbread CLI](https://www.mixedbread.com/cli) to sync your documentation:

Install the CLI:

```package-install
@mixedbread/cli -D
```

Configure authentication and sync your documentation:

```bash
# Configure authentication
mxbai config keys add YOUR_API_KEY

# Sync your documentation
mxbai vs sync YOUR_STORE_ID "./content/docs"
```

The CLI will automatically detect changes in your documentation and update the store accordingly.

### Workflow [#workflow]

You can automatically sync your documentation by adding a sync script to your `package.json`:

```json
{
  "scripts": {
    "build": "... && mxbai vs sync YOUR_STORE_ID './content/docs' --ci"
  }
}
```

### Search API [#search-api]

Create an API route to handle search requests server-side:

```ts title="app/api/search/route.ts"
import { createMixedbreadSearchAPI } from 'fumadocs-core/search/mixedbread';
import Mixedbread from '@mixedbread/sdk';

const client = new Mixedbread({
  apiKey: 'YOUR_API_KEY',
});

export const { GET } = createMixedbreadSearchAPI({
  client,
  storeIdentifier: 'YOUR_STORE_ID',
});
```

### Search Client [#search-client]

- **Fumadocs UI**: see [Search UI](/docs/search/mixedbread) for details.
- **Search Client**:

  ```ts
  import { useDocsSearch } from 'fumadocs-core/search/client';
  import { fetchClient } from 'fumadocs-core/search/client/fetch';

  const client = useDocsSearch({
    client: fetchClient({
      api: '/api/search',
    }),
  });
  ```

## Options [#options]

### Tag Filter [#tag-filter]

To filter search results by tags, add a tag field to your document metadata:

```md
---
title: Mixedbread
description: Integrate Mixedbread Search with Fumadocs
url: /docs/headless/search/mixedbread
// [!code ++]
tag: docs
---
```

And update your search client:

- **Fumadocs UI**: Enable [Tag Filter](/docs/search/mixedbread#tag-filter) on Search UI.
- **Search Client**: You can add the tag filter like:

  ```ts
  import { useDocsSearch } from 'fumadocs-core/search/client';
  import { fetchClient } from 'fumadocs-core/search/client/fetch';

  const { search, setSearch, query } = useDocsSearch({
    client: fetchClient({
      api: '/api/search',
      tag: '<your tag value>',
      // ...
    }),
  });
  ```

This allows you to scope searches to specific sections of your documentation.
