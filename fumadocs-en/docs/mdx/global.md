# Fumadocs MDX (the built-in content source): Global Options

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/mdx/global.mdx

Customize Fumadocs MDX

## Global Options [#global-options]

Shared options of Fumadocs MDX.

```ts title="source.config.ts"
import { defineConfig } from 'fumadocs-mdx/config';

export default defineConfig({
  // global options
});
```

### GlobalConfig

| Prop                      | Type                     | Description                                                                                                                                                                       |
| ------------------------- | ------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plugins?`                | `array`                  |                                                                                                                                                                                   |
| `compiler?`               | `union`                  | The compiler for files compiled without a collection (e.g. `page.mdx` routes). Collections choose their own compiler via the collection-level `compiler` option. Default: `'mdx'` |
| `mdxOptions?`             | `union`                  | Configure global MDX options, used by `doc` collections with the default MDX compiler.                                                                                            |
| `satteriOptions?`         | `SatteriOptionsInput`    | Configure global Sätteri options, used by `doc` collections with `compiler: "satteri"`.                                                                                           |
| `workspaces?`             | `Record<string, object>` |                                                                                                                                                                                   |
| `experimentalBuildCache?` | `string`                 |                                                                                                                                                                                   |


### MDX Options [#mdx-options]

Customize the [default MDX preset](/docs/mdx/mdx).

```ts title="source.config.ts"
import { defineConfig } from 'fumadocs-mdx/config';
import rehypeKatex from 'rehype-katex';
import remarkMath from 'remark-math';

export default defineConfig({
  mdxOptions: {
    remarkPlugins: [remarkMath],
    // When order matters
    rehypePlugins: (v) => [rehypeKatex, ...v],
  },
});
```

Or using the minimal preset:

```ts title="source.config.ts"
import { defineConfig } from 'fumadocs-mdx/config';

export default defineConfig({
  mdxOptions: {
    preset: 'minimal',
    // now it accepts only MDX processor options
  },
});
```
