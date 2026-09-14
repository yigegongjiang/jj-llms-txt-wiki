# Fumadocs MDX (the built-in content source): Getting Started

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/mdx/index.mdx

Introducing Fumadocs MDX, the official content source of Fumadocs.

## Introduction [#introduction]

Fumadocs MDX is a tool to transform content into type-safe data, similar to Content Collections.

It is not a full CMS but rather a content processing layer for React frameworks, you can use it to handle blog posts and other contents.

### What is a Collection? [#what-is-a-collection]

**Collection** refers to a collection containing a certain type of files. You can define them with the [Macro API](/docs/mdx/macro) in your app:

```ts title="lib/source.ts"
import { defineDocs } from 'fumadocs-mdx/macro';
import { loader } from 'fumadocs-core/source';

const docs = defineDocs({
  dir: 'content/docs',
});

export const source = loader({
  baseUrl: '/docs',
  source: docs.toFumadocsSource(),
});
```

Available collections:

<Tabs items={["doc", "meta", 'docs']}>

    <Tab value='doc'>

Compile Markdown & MDX files into a React component, with useful properties like **Table of Contents**.

```ts
import { defineCollections } from 'fumadocs-mdx/macro';

export const test = defineCollections({
  type: 'doc',
  dir: 'content/docs',
});
```

    </Tab>

    <Tab value='meta'>

Transform YAML/JSON files into an array of data.

```ts
import { defineCollections } from 'fumadocs-mdx/macro';

export const test = defineCollections({
  type: 'meta',
  dir: 'content/docs',
});
```

    </Tab>

    <Tab value='docs'>

Combination of `meta` and `doc` collections, which is needed for Fumadocs.

```ts
import { defineDocs } from 'fumadocs-mdx/macro';

export const docs = defineDocs({
  dir: 'content/docs',
  docs: {
    // options for `doc` collection
  },
  meta: {
    // options for `meta` collection
  },
});
```

    </Tab>

</Tabs>

For example, a `doc` collection will transform the `.md` and `.mdx` files:

<Files>
  <Folder name="folder" defaultOpen>
    <File name="ui.md" />
  </Folder>
  <File name="hello.md" />
  <File name="index.mdx" />
  <File name="meta.json" className="opacity-50 cursor-not-allowed" aria-disabled />
</Files>

You can also define collections in `source.config.ts` with the [Config API](/docs/mdx/collections), which generates entry files under `.source`.

## Installation [#installation]

Configure Fumadocs MDX on:

<Cards>
  <Card title="Next.js" href="/docs/mdx/next">
    Using Fumadocs MDX with Next.js.
  </Card>
  <Card title="Vite" href="/docs/mdx/vite">
    Using Fumadocs MDX with Vite.
  </Card>
  <Card title="Runtime Loader" href="/docs/mdx/loader">
    Using Fumadocs MDX with a JavaScript runtime.
  </Card>
</Cards>

## FAQ [#faq]

### Built-in Properties [#built-in-properties]

These properties are exported from MDX files by default.

| Property              | Description                                     |
| --------------------- | ----------------------------------------------- |
| `frontmatter`         | Frontmatter                                     |
| `toc`                 | Table of Contents                               |
| `structuredData`      | Structured Data, useful for implementing search |
| `extractedReferences` | For analyzing `hrefs` references                |

### Customize Frontmatter [#customize-frontmatter]

Use the [`schema`](/docs/mdx/collections#schema-1) option to pass a validation schema to validate frontmatter and define its output properties.

### Customize MDX Compiler [#customize-mdx-compiler]

Fumadocs MDX uses [MDX Compiler](https://mdxjs.com/packages/mdx) to compile MDX files into JavaScript files.
The [default preset](/docs/mdx/mdx) includes a set of plugins and configurations out-of-the-box.

You can customize it on [Global Config](/docs/mdx/global#mdx-options) or [Collection Config](/docs/mdx/collections#mdxoptions).
