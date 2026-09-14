# Fumadocs (Framework Mode): createAsyncAPI()

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/(framework)/integrations/asyncapi/server.mdx

The AsyncAPI server instance.

## AsyncAPI Server [#asyncapi-server]

The main config for Fumadocs AsyncAPI.

> It should not be referenced in browser environments.

### `input` [#input]

The AsyncAPI schemas to read from.

- File Paths
- External URLs
- Functions (see below)

```ts tab="Basic"
import { createAsyncAPI } from '@fumadocs/asyncapi/server';

export const asyncapi = createAsyncAPI({
  input: ['./streetlights.yaml'],
});
```

```ts tab="Functions"
import { createAsyncAPI } from '@fumadocs/asyncapi/server';

export const asyncapi = createAsyncAPI({
  input: {
    // [id]: downloaded AsyncAPI Schema
    galaxy: async () => {
      const res = await fetch('https://example.com/asyncapi.yaml');
      return res.text();
    },
  },
});
```

### `disableCache` [#disablecache]

Disable caching of loaded schemas. Useful during development when schemas change frequently.

```ts
import { createAsyncAPI } from '@fumadocs/asyncapi/server';

export const asyncapi = createAsyncAPI({
  input: ['./asyncapi.yaml'],
  disableCache: true,
});
```
