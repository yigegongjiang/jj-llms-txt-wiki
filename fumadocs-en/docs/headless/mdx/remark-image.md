# Fumadocs Core (the core library of Fumadocs): Remark Image

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/headless/mdx/remark-image.mdx

Adding size attributes to images.

This plugin adds `width` and `height` attributes to your image elements, which is needed for Image Optimization on Next.js and some other frameworks.

## Usage [#usage]

Add it to your Remark plugins.

```ts tab="MDX Compiler"
import { compile } from '@mdx-js/mdx';
import { remarkImage } from 'fumadocs-core/mdx-plugins';

await compile('...', {
  remarkPlugins: [remarkImage],
});
```

```ts title="source.config.ts" tab="Fumadocs MDX"
import { defineConfig } from 'fumadocs-mdx/config';

export default defineConfig({
  mdxOptions: {
    remarkImageOptions: {
      // it is enabled by default, customize it here
    },
  },
});
```

Supported:

- Local Images
- External URLs
- Next.js static imports

### How It Works [#how-it-works]

For Next.js, it uses **static imports** to import local images, which supports the `placeholder` option of Next.js Image.
Next.js can handle image imports with its built-in image loader.

Otherwise, it uses the file system or an HTTP request to download the image and obtain its size.

### Options [#options]

### RemarkImageOptions

| Prop           | Type     | Description                                                                                                                                                                                                                  |
| -------------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `publicDir?`   | `string` | Directory or base URL to resolve absolute image paths                                                                                                                                                                        |
| `placeholder?` | `union`  | Preferred placeholder type, only available with `useImport` + local images. Default: `'none'`                                                                                                                                |
| `onError?`     | `union`  | Define how to handle errors when fetching image size. - `error` (default): throw an error. - `ignore`: do absolutely nothing (Next.js Image component may complain). - `hide`: remove that image element. Default: `'error'` |
| `useImport?`   | `union`  | Import images in the file, and let bundlers handle it. ```tsx import MyImage from "./public/img.png"; <img src={MyImage} /> ``` When disabled, `placeholder` will be ignored. Default: `true`                                |
| `external?`    | `union`  | Fetch image size of external URLs Default: `true`                                                                                                                                                                            |


### Example: With Imports [#example-with-imports]

```mdx
![Hello](/hello.png)
![Test](https://example.com/image.png)
```

Yields:

```mdx
import HelloImage from './public/hello.png';

<img alt="Hello" src={HelloImage} />
<img alt="Test" src="https://example.com/image.png" width="1980" height="1080" />
```

Where `./public/hello.png` points to the image in public directory.

### Example: Without Imports [#example-without-imports]

For Next.js, you can disable static imports on local images.

```ts tab="MDX Compiler"
import { compile } from '@mdx-js/mdx';
import { remarkImage } from 'fumadocs-core/mdx-plugins';

await compile('...', {
  remarkPlugins: [[remarkImage, { useImport: false }]],
});
```

```ts title="source.config.ts" tab="Fumadocs MDX"
import { defineConfig } from 'fumadocs-mdx/config';

export default defineConfig({
  mdxOptions: {
    remarkImageOptions: {
      useImport: false,
    },
  },
});
```

```mdx
![Hello](/hello.png)
![Test](https://example.com/image.png)
```

Yields:

```mdx
<img alt="Hello" src="/hello.png" width="1980" height="1080" />
<img alt="Test" src="https://example.com/image.png" width="1980" height="1080" />
```

### Example: Relative Paths [#example-relative-paths]

When `useImport` is enabled, you can reference local images using relative paths.

```mdx
![Hello](./hello.png)
```

Be careful that using it with `useImport` disabled **doesn't work**.
Next.js will not add the image to public assets unless you have imported it in code.
For images in public directory, you can just reference them without relative paths.

### Example: Public Directory [#example-public-directory]

Customize the path of public directory

```ts tab="MDX Compiler"
import { compile } from '@mdx-js/mdx';
import { remarkImage } from 'fumadocs-core/mdx-plugins';
import path from 'node:path';

await compile('...', {
  remarkPlugins: [[remarkImage, { publicDir: path.join(process.cwd(), 'dir') }]],
});
```

```ts title="source.config.ts" tab="Fumadocs MDX"
import { defineConfig } from 'fumadocs-mdx/config';
import path from 'node:path';

export default defineConfig({
  mdxOptions: {
    remarkImageOptions: {
      publicDir: path.join(process.cwd(), 'dir'),
    },
  },
});
```

You can pass a URL too.

```ts tab="MDX Compiler"
import { compile } from '@mdx-js/mdx';
import { remarkImage } from 'fumadocs-core/mdx-plugins';

await compile('...', {
  remarkPlugins: [[remarkImage, { publicDir: 'https://my-cdn.com/images' }]],
});
```

```ts title="source.config.ts" tab="Fumadocs MDX"
import { defineConfig } from 'fumadocs-mdx/config';

export default defineConfig({
  mdxOptions: {
    remarkImageOptions: {
      publicDir: 'https://my-cdn.com/images',
    },
  },
});
```
