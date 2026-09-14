# Fumadocs (Framework Mode): Next.js

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/(framework)/manual-installation/next.mdx

Setup Fumadocs on Next.js.

## Prerequisite [#prerequisite]

Before continuing, make sure you have configured:

- Next.js 16.
- Tailwind CSS 4.

We will use [Fumadocs MDX](/docs/mdx) as a content source, you can configure it first:

<div className="fd-steps [&_h3]:fd-step">

### Installation

```npm
npm i fumadocs-mdx fumadocs-core @types/mdx
```

Add the plugin to Next.js config:

```js title="next.config.mjs"
import { createMDX } from 'fumadocs-mdx/next';

/** @type {import('next').NextConfig} */
const config = {
  reactStrictMode: true,
};

// [!code ++:4]
const withMDX = createMDX();

// [!code highlight]
export default withMDX(config);
```

<Callout title="ESM Only" type='warn'>

Fumadocs MDX is ESM-only, it's recommended to use `next.config.mjs` for accurate ESM resolution.

For TypeScript config file, it requires Native Node.js TypeScript Resolver, you can see [Next.js docs](https://nextjs.org/docs/app/api-reference/config/typescript#using-nodejs-native-typescript-resolver-for-nextconfigts) for details.

</Callout>

### Integrate with Fumadocs

Create a `lib/source.ts` file:

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

### Done

You can now write content in `content/docs` folder.

</div>

<Callout title='Good to Know'>

Fumadocs also supports other content sources, including [Content Collections](/docs/headless/content-collections) and headless CMS.

</Callout>

## Getting Started [#getting-started]

```npm
npm i fumadocs-ui fumadocs-core
```

### Root Layout [#root-layout]

Wrap the entire application inside [Root Provider](/docs/ui/layouts/root-provider), and add required styles to `body`.

```tsx title="app/layout.tsx"
import { RootProvider } from 'fumadocs-ui/provider/next';
import type { ReactNode } from 'react';

export default function Layout({ children }: { children: ReactNode }) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body
        // required styles [!code ++]
        className="flex flex-col min-h-screen"
      >
        <RootProvider>{children}</RootProvider>
      </body>
    </html>
  );
}
```

### Styles [#styles]

Add the following Tailwind CSS styles to `global.css`.

```css title="global.css"
@import 'tailwindcss';
/* [!code ++:2] */
@import 'fumadocs-ui/css/neutral.css';
@import 'fumadocs-ui/css/preset.css';
```

> It doesn't come with a default font, you may choose one from `next/font`.

### Routes [#routes]

Create a `lib/layout.shared.tsx` file to put the shared options for our layouts.

```tsx title="lib/layout.shared.tsx"
import type { BaseLayoutProps } from 'fumadocs-ui/layouts/shared';

export function baseOptions(): BaseLayoutProps {
  return {
    nav: {
      title: 'My App',
    },
  };
}

```

Create the following files & routes:

```tsx tab="components/mdx.tsx"
import defaultMdxComponents from 'fumadocs-ui/mdx';
import type { MDXComponents } from 'mdx/types';

export function getMDXComponents(components?: MDXComponents) {
  return {
    ...defaultMdxComponents,
    ...components,
  } satisfies MDXComponents;
}

export const useMDXComponents = getMDXComponents;

declare global {
  type MDXProvidedComponents = ReturnType<typeof getMDXComponents>;
}

```

```tsx tab="app/docs/layout.tsx"
import { source } from '@/lib/source';
import { DocsLayout } from 'fumadocs-ui/layouts/docs';
import { baseOptions } from '@/lib/layout.shared';

export default function Layout({ children }: LayoutProps<'/docs'>) {
  return (
    <DocsLayout tree={source.getPageTree()} {...baseOptions()}>
      {children}
    </DocsLayout>
  );
}

```

```tsx tab="app/docs/[[...slug]]/page.tsx"
import { source } from '@/lib/source';
import { DocsBody, DocsDescription, DocsPage, DocsTitle } from 'fumadocs-ui/layouts/docs/page';
import { notFound } from 'next/navigation';
import { getMDXComponents } from '@/components/mdx';
import type { Metadata } from 'next';
import { createRelativeLink } from 'fumadocs-ui/mdx';

export default async function Page(props: PageProps<'/docs/[[...slug]]'>) {
  const params = await props.params;
  const page = source.getPage(params.slug);
  if (!page) notFound();

  const MDX = page.data.body;

  return (
    <DocsPage toc={page.data.toc} full={page.data.full}>
      <DocsTitle>{page.data.title}</DocsTitle>
      <DocsDescription>{page.data.description}</DocsDescription>
      <DocsBody>
        <MDX
          components={getMDXComponents({
            // this allows you to link to other pages with relative file paths
            a: createRelativeLink(source, page),
          })}
        />
      </DocsBody>
    </DocsPage>
  );
}

export async function generateStaticParams() {
  return source.generateParams();
}

export async function generateMetadata(props: PageProps<'/docs/[[...slug]]'>): Promise<Metadata> {
  const params = await props.params;
  const page = source.getPage(params.slug);
  if (!page) notFound();

  return {
    title: page.data.title,
    description: page.data.description,
  };
}

```

```ts tab="app/api/search/route.ts"
import { source } from '@/lib/source';
import { createFromSource } from 'fumadocs-core/search/server';

export const { GET } = createFromSource(source, {
  // https://docs.orama.com/docs/orama-js/supported-languages
  language: 'english',
});

```

> The search is powered by ZBSearch, learn more about [Document Search](/docs/headless/search).

### Finish [#finish]

You can start the dev server and create MDX files.

```mdx title="content/docs/index.mdx"
---
title: Hello World
---

## Introduction

I love Anime.
```
