# Fumadocs Core (the core library of Fumadocs): Rehype Code

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/headless/mdx/rehype-code.mdx

Code syntax highlighter

A wrapper of [`@shikijs/rehype`](https://shiki.style/packages/rehype), the syntax highlighter used by Fumadocs.

It converts raw `<pre />` codeblocks into highlighted codeblocks:

````md tab="Input"
```ts
console.log('Hello World');
```
````

```html tab="Output"
<pre>
  <code>
    <span class="line">
      <span style="--shiki-light: #4C4F69; --shiki-dark: #CDD6F4;">console</span>...
    </span>
  </code>
</pre>
```

## Usage [#usage]

Add the rehype plugin.

```ts tab="MDX Compiler"
import { compile } from '@mdx-js/mdx';
import { rehypeCode, type RehypeCodeOptions } from 'fumadocs-core/mdx-plugins';

const rehypeCodeOptions: RehypeCodeOptions = {
  themes: {
    light: 'github-light',
    dark: 'github-dark',
  },
};

await compile('...', {
  rehypePlugins: [
    // using default settings
    rehypeCode,
    // or with custom options
    [rehypeCode, rehypeCodeOptions],
  ],
});
```

```ts tab="Fumadocs MDX"
import { defineConfig } from 'fumadocs-mdx/config';

export default defineConfig({
  mdxOptions: {
    rehypeCodeOptions: {
      // enabled by default on Fumadocs MDX
      // configure options here
    },
  },
});
```

### Meta [#meta]

It parses the `title` meta string, and adds it to the `pre` element as an attribute.

````mdx tab="Input"
```js title="Title"
console.log('Hello');
```
````

```jsx tab="Output"
<pre title="Title">...</pre>
```

You may filter the meta string before processing it with the `filterMetaString` option.

### Inline Code [#inline-code]

You can enable it with `inline` option:

```ts
import { type RehypeCodeOptions } from 'fumadocs-core/mdx-plugins';

const rehypeCodeOptions: RehypeCodeOptions = {
  // [!code ++]
  inline: 'tailing-curly-colon',
};
```

```md title="Syntax"
This is a highlighted inline code: `console.log("hello world"){:js}`.
```

This is a highlighted inline code: `console.log("hello world"){:js}`.

### Icon [#icon]

The plugin will automatically adds an `icon` attribute according to the language meta string.
It is a HTML string, you can render it with React `dangerouslySetInnerHTML`.

````md tab="Input"
```ts
console.log('This should shows the logo of TypeScript');
```
````

```jsx tab="Output"
<pre icon="<svg />">...</pre>
```

Disable or customize icons with the `icon` option.

### More Options [#more-options]

The options are inherited from [Shiki](https://shiki.style), see their docs for other options.
