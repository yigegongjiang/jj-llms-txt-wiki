# Fumadocs MDX (the built-in content source): Node.js

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/mdx/(integrations)/loader/node.mdx

Access content in Node.js runtime.

## Setup [#setup]

Make sure to run the script under ESM environment.

```js title="scripts/example.js"
import { register } from 'fumadocs-mdx/node';

// register the Node.js loader
register();

// accessing content
const { source } = await import('./lib/source');
console.log(source.getPages());
```

The `register()` function accepts options:

```ts
register({
  configPath: '...',
  // ...
});
```
