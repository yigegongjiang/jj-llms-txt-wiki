# Fumadocs UI (the default theme of Fumadocs): Files

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/ui/components/files.mdx

Display file structure in your documentation

<Installation name="files" />

## Usage [#usage]

Wrap file components in `Files`, you can use it in your MDX content, or as a normal React.js component.

```mdx title="content.mdx"
import { File, Folder, Files } from 'fumadocs-ui/components/files';

<Files>
  <Folder name="app" defaultOpen>
    <File name="layout.tsx" />
    <File name="page.tsx" />
    <File name="global.css" />
  </Folder>
  <Folder name="components">
    <File name="button.tsx" />
    <File name="tabs.tsx" />
    <File name="dialog.tsx" />
  </Folder>
  <File name="package.json" />
</Files>
```

### File [#file]

### FileProps

| Prop    | Type        | Description |
| ------- | ----------- | ----------- |
| `name`  | `string`    |             |
| `icon?` | `ReactNode` |             |


### Folder [#folder]

### FolderProps

| Prop           | Type     | Description                             |
| -------------- | -------- | --------------------------------------- |
| `name`         | `string` |                                         |
| `disabled?`    | `union`  |                                         |
| `defaultOpen?` | `union`  | Open folder by default Default: `false` |


## Remark Plugin [#remark-plugin]

You can enable [`remark-mdx-files`](/docs/headless/mdx/remark-mdx-files) for additional feature & syntax.

```tsx title="source.config.ts (Fumadocs MDX)"
import { remarkMdxFiles } from 'fumadocs-core/mdx-plugins/remark-mdx-files';
import { defineConfig } from 'fumadocs-mdx/config';

export default defineConfig({
  mdxOptions: {
    // [!code ++]
    remarkPlugins: [remarkMdxFiles],
  },
});
```

### CodeBlock Syntax [#codeblock-syntax]

It will convert `files` codeblocks into `<Files />` component, like:

````md title="content.md"
```files
project
├── src
│   ├── index.js
│   └── utils
│       └── helper.js
├── package.json
```
````

### `<auto-files>` [#auto-files]

Generate `<Files />` component from glob.

```mdx title="content.mdx"
<auto-files dir="./my-dir" pattern="**/*.{ts,tsx}" />

<auto-files dir="./my-dir" pattern="**/*.{ts,tsx}" defaultOpenAll />
```
