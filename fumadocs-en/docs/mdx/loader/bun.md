# Fumadocs MDX (the built-in content source): Bun

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/mdx/(integrations)/loader/bun.mdx

Access content in Bun.

## Setup [#setup]

Register the plugin in preload script:

```toml tab="bunfig.toml"
preload = ["./scripts/preload.ts"]
```

```ts tab="scripts/preload.ts"
import { createMdxPlugin } from 'fumadocs-mdx/bun';

Bun.plugin(createMdxPlugin());
```

It will add support for loading MDX/meta files on runtime.

Now running any script with Bun, you can access your content files directly.

### Re-generate `.source` folder [#re-generate-source-folder]

Optionally, you can re-generate the `.source` folder.

It is useful when the folder could be missing,
or you are using Vite, which defaults to generating code with Vite-specific APIs.

```ts title="preload.ts" tab="Next.js"
import { postInstall } from 'fumadocs-mdx/next';

await postInstall({ configPath: 'source.config.ts' });
```

```ts title="preload.ts" tab="Vite"
import { postInstall } from 'fumadocs-mdx/vite';

await postInstall({
  configPath: 'source.config.ts',
  index: {
    target: 'bun',
  },
});
```
