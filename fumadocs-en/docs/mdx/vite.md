# Fumadocs MDX (the built-in content source): Vite

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/mdx/(integrations)/vite.mdx

Use Fumadocs MDX with Vite

## Setup [#setup]

<div className="fd-steps [&_h3]:fd-step">

### Installation [#installation]

```npm
npm i fumadocs-mdx fumadocs-core @types/mdx
```

Add the Vite plugin:

```ts tab="Vite" title="vite.config.ts"
import { defineConfig } from 'vite';
import { fumadocsMdx } from 'fumadocs-mdx/vite';

export default defineConfig({
  plugins: [
    // [!code ++]
    fumadocsMdx(),
    // ...
  ],
});
```

```ts tab="Waku" title="waku.config.ts"
import { type Config, defineConfig } from 'waku/config';
import { fumadocsMdx } from 'fumadocs-mdx/vite';
import type { UserConfig } from 'vite';

export default defineConfig({
  vite: {
    plugins: [
      // [!code ++]
      fumadocsMdx(),
    ],
    resolve: {
      tsconfigPaths: true,
    },
  } satisfies UserConfig as Config['vite'],
});
```

### Integrate with Fumadocs [#integrate-with-fumadocs]

Create a `lib/source.ts` file:

```ts title="app/lib/source.ts"
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

### Done [#done]

You can now write content in `content/docs` folder.

</div>

## What is Next? [#what-is-next]

<Cards>
  <Card title="Macro API" href="/docs/mdx/macro">
    Define collections in your app modules with `fumadocs-mdx/macro`.
  </Card>
  <Card title="Config API" href="/docs/mdx/collections">
    Define collections in `source.config.ts` with generated entry files.
  </Card>
  <Card title="Lazy Loading" href="/docs/mdx/async">
    Hit performance bottleneck? You can try lazy loading.
  </Card>
</Cards>
