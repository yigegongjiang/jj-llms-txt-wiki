# Fumadocs (Framework Mode): Waku

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/(framework)/manual-installation/waku.mdx

Setup Fumadocs on Waku.

## Getting Started [#getting-started]

Before continuing, make sure to configure:

- Tailwind CSS 4.
- Fumadocs MDX: follow the [Vite setup guide](/docs/mdx/vite) and create essential files like `lib/source.ts`.

### Installation [#installation]

```npm
npm i fumadocs-core fumadocs-ui
```

### Styles [#styles]

Add the following to your Tailwind CSS file:

```css title="src/styles/globals.css"
@import 'tailwindcss';
/* [!code ++:2] */
@import 'fumadocs-ui/css/neutral.css';
@import 'fumadocs-ui/css/preset.css';
```

### Create Pages [#create-pages]

Create a file for sharing layout props:

```tsx title="lib/layout.shared.tsx"
import type { BaseLayoutProps } from 'fumadocs-ui/layouts/shared';
import { appName, gitConfig } from './shared';

export function baseOptions(): BaseLayoutProps {
  return {
    nav: {
      // JSX supported
      title: appName,
    },
    githubUrl: `https://github.com/${gitConfig.user}/${gitConfig.repo}`,
  };
}

```

Create the following files:

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
```tsx tab="pages/docs/_layout.tsx"
import type { ReactNode } from 'react';
import { DocsLayout } from 'fumadocs-ui/layouts/docs';
import { source } from '@/lib/source';
import { baseOptions } from '@/lib/layout.shared';

export default function Layout({ children }: { children: ReactNode }) {
  return (
    <DocsLayout {...baseOptions()} tree={source.getPageTree()}>
      {children}
    </DocsLayout>
  );
}

```
```tsx tab="pages/docs/[...slugs].tsx"
import { source } from '@/lib/source';
import { PageProps } from 'waku/router';
import { createRelativeLink } from 'fumadocs-ui/mdx';
import {
  DocsBody,
  DocsDescription,
  DocsPage,
  DocsTitle,
  MarkdownCopyButton,
  ViewOptionsPopover,
} from 'fumadocs-ui/layouts/docs/page';
import { unstable_notFound } from 'waku/router/server';
import { getMDXComponents } from '@/components/mdx';
import { getPageImageUrl, getPageMarkdownUrl, gitConfig } from '@/lib/shared';

export default function Page({ slugs }: PageProps<'/docs/[...slugs]'>) {
  const page = source.getPage(slugs);
  if (!page) unstable_notFound();

  const MDX = page.data.body;
  const markdownUrl = getPageMarkdownUrl(page).url;
  return (
    <DocsPage toc={page.data.toc}>
      <meta property="og:image" content={getPageImageUrl(page).url} />
      <DocsTitle>{page.data.title}</DocsTitle>
      <DocsDescription className="mb-0">{page.data.description}</DocsDescription>
      <div className="flex flex-row gap-2 items-center border-b pt-2 pb-6">
        <MarkdownCopyButton markdownUrl={markdownUrl} />
        <ViewOptionsPopover
          markdownUrl={markdownUrl}
          githubUrl={`https://github.com/${gitConfig.user}/${gitConfig.repo}/blob/${gitConfig.branch}/content/docs/${page.path}`}
        />
      </div>
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

export async function getConfig() {
  const pages = source
    .generateParams()
    .map((item) => (item.lang ? [item.lang, ...item.slug] : item.slug));

  return {
    render: 'static' as const,
    staticPaths: pages,
  } as const;
}

```
```ts tab="pages/_api/api/search.ts"
import { createFromSource } from 'fumadocs-core/search/server';
import { source } from '@/lib/source';

export const { GET } = createFromSource(source);

```

Finally, wrap your entire app under Fumadocs providers:

```tsx tab="components/provider.tsx"
'use client';
import type { ReactNode } from 'react';
import { RootProvider } from 'fumadocs-ui/provider/waku';

export function Provider({ children }: { children: ReactNode }) {
  return <RootProvider>{children}</RootProvider>;
}

```

```tsx tab="pages/_layout.tsx"
import '@/styles/globals.css';
import type { ReactNode } from 'react';
import { Provider } from '@/components/provider';

// [!code ++:3]
export default function RootLayout({ children }: { children: ReactNode }) {
  return <Provider>{children}</Provider>;
}

export const getConfig = async () => {
  return {
    render: 'static',
  };
};
```

### Done [#done]

You can start writing documents at `content/docs`:

```mdx title="content/docs/index.mdx"
---
title: Hello World
---

I love Fumadocs
```
