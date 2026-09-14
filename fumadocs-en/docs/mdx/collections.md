# Fumadocs MDX (the built-in content source): Collections

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/mdx/collections.mdx

Collection of content data for your app

## Define Collections [#define-collections]

Define a collection to parse a certain set of files.

```ts
import { defineCollections } from 'fumadocs-mdx/config';
import { z } from 'zod';

export const blog = defineCollections({
  type: 'doc',
  dir: './content/blog',
  schema: z.object({
    // schema
  }),
  // other options
});
```

### `type` [#type]

The accepted type of collection.

```ts
import { defineCollections } from 'fumadocs-mdx/config';

// only scan for json/yaml files
export const metaFiles = defineCollections({
  type: 'meta',
  // options
});
```

- `type: meta`

  Accept JSON/YAML files, available options:

  ### MetaCollection

| Prop      | Type     | Description |
| --------- | -------- | ----------- |
| `type`    | `"meta"` |             |
| `schema?` | `union`  |             |
| `dir`     | `string` |             |
| `files?`  | `array`  |             |


- `type: doc`

  Markdown/MDX documents, available options:

  ### DocCollection

| Prop              | Type                  | Description                                                                                                                                                                                                                                                                 |
| ----------------- | --------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`            | `"doc"`               |                                                                                                                                                                                                                                                                             |
| `compiler?`       | `union`               |                                                                                                                                                                                                                                                                             |
| `mdxOptions?`     | `union`               | By defining a collection-level MDX options, **the default options & plugins will be removed**.                                                                                                                                                                              |
| `satteriOptions?` | `SatteriOptionsInput` | Sätteri compile options. When omitted, the global `satteriOptions` preset is used.                                                                                                                                                                                          |
| `dir`             | `string`              |                                                                                                                                                                                                                                                                             |
| `files?`          | `array`               |                                                                                                                                                                                                                                                                             |
| `postprocess?`    | `Partial<object>`     |                                                                                                                                                                                                                                                                             |
| `async?`          | `union`               |                                                                                                                                                                                                                                                                             |
| `dynamic?`        | `union`               |                                                                                                                                                                                                                                                                             |
| `schema?`         | `union`               |                                                                                                                                                                                                                                                                             |
| `lastModified?`   | `union`               | Expose the last modified date of each document. - `true`: obtained from Git. Requires `git` to be installed. If you are using Vercel, please set the `VERCEL_DEEP_CLONE` environment variable to `true`. - A function: return the last modified time for a given file path. |


### `schema` [#schema]

The schema to validate file data (frontmatter on `doc` type, content on `meta` type).

```ts
import { defineCollections } from 'fumadocs-mdx/config';
import { z } from 'zod';

export const blog = defineCollections({
  type: 'doc',
  dir: './content/blog',
  schema: z.object({
    name: z.string(),
  }),
});
```

> [Standard Schema](https://standardschema.dev) compatible libraries, including Zod, are supported.

Note that the validation is done at build time, hence the output must be serializable.
You can also pass a function that receives the transform context.

```ts
import { defineCollections } from 'fumadocs-mdx/config';
import { z } from 'zod';

export const blog = defineCollections({
  type: 'doc',
  dir: './content/blog',
  schema: (ctx) => {
    return z.object({
      name: z.string(),
      testPath: z.string().default(
        // original file path
        ctx.path,
      ),
    });
  },
});
```

### `mdxOptions` [#mdxoptions]

Customize MDX options at the collection level.

> This API is only available on `doc` type.

```ts title="source.config.ts"
import { defineCollections } from 'fumadocs-mdx/config';

export const blog = defineCollections({
  type: 'doc',
  mdxOptions: {
    // mdx options
  },
});
```

By design, this will remove all default settings applied by your global config and Fumadocs MDX.
You have full control over MDX options.

You can use [`applyMdxPreset`](/docs/mdx/mdx) to apply the default MDX preset.

```ts title="source.config.ts"
import { defineCollections } from 'fumadocs-mdx/config';

export const blog = defineCollections({
  type: 'doc',
  mdxOptions: applyMdxPreset({
    // extend the preset
  }),
});
```

### `postprocess` [#postprocess]

> This API is only available on `doc` type.

You can pass build-time information to runtime using the `postprocess` API.

- #### `includeProcessedMarkdown` [#includeprocessedmarkdown]

  Include the processed Markdown content (before being converted into HTML).

  ```ts tab="docs"
  import { defineDocs } from 'fumadocs-mdx/config';

  export default defineDocs({
    docs: {
      postprocess: {
        // [!code ++]
        includeProcessedMarkdown: true,
      },
    },
  });
  ```

  ```ts tab="doc"
  import { defineCollections } from 'fumadocs-mdx/config';

  export const blog = defineCollections({
    type: 'doc',
    postprocess: {
      // [!code ++]
      includeProcessedMarkdown: true,
    },
  });
  ```

  `getText('processed')` is the recommended way to obtain it (e.g. with `loader()`):

  ```ts
  import { source } from '@/lib/source';

  const page = source.getPage('...');
  console.log(await page.data.getText('processed'));
  ```

  See [available options](/docs/headless/mdx/remark-llms).

- #### `valueToExport` [#valuetoexport]

  Some remark plugins store their output in `vfile.data` (compile-time memory) which cannot be accessed from your code.

  You can specify the name of properties to include, they will get converted into ESM exports that you can access when importing the MDX file.

  ```ts tab="docs"
  import { defineDocs } from 'fumadocs-mdx/config';

  export default defineDocs({
    docs: {
      postprocess: {
        // [!code ++]
        valueToExport: ['dataName'],
      },
    },
  });
  ```

  ```ts tab="doc"
  import { defineCollections } from 'fumadocs-mdx/config';

  export const blog = defineCollections({
    type: 'doc',
    postprocess: {
      // [!code ++]
      valueToExport: ['dataName'],
    },
  });
  ```

## Define Docs [#define-docs]

Define a collection for Fumadocs.

```ts
import { defineDocs } from 'fumadocs-mdx/config';

export const docs = defineDocs({
  dir: '/my/content/dir',
  docs: {
    // optional, options of `doc` collection
  },
  meta: {
    // optional, options of `meta` collection
  },
});
```

### `dir` [#dir]

Instead of per collection, you should customize content directory from `defineDocs`:

```ts
import { defineDocs } from 'fumadocs-mdx/config';

export const docs = defineDocs({
  dir: 'content/guides',
});
```

### `schema` [#schema-1]

You can extend the default Zod 4 schema of `docs` and `meta`.

```ts
import { defineDocs } from 'fumadocs-mdx/config';
import { metaSchema, pageSchema } from 'fumadocs-core/source/schema';
import { z } from 'zod';

export const docs = defineDocs({
  docs: {
    schema: pageSchema.extend({
      index: z.boolean().default(false),
    }),
  },
  meta: {
    schema: metaSchema.extend({
      // other props
    }),
  },
});
```
