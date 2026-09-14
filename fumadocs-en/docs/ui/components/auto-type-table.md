# Fumadocs UI (the default theme of Fumadocs): Auto Type Table

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/ui/components/auto-type-table.mdx

Auto-generated type table

<Wrapper>

### AutoTypeTableExample

| Prop       | Type              | Description                                                                                              |
| ---------- | ----------------- | -------------------------------------------------------------------------------------------------------- |
| `name`     | `union`           | Markdown syntax like links, `code` are supported. See https://fumadocs.dev/docs/ui/components/type-table |
| `fn`       | `function`        |                                                                                                          |
| `options?` | `Partial<object>` | We love Shiki. ```ts console.log("Hello World, powered by Shiki"); ``` Default: `{ a: "test" }`          |


</Wrapper>

<Callout title="Server Component only" type="warn">

You cannot use this in a client component, instead, try the [build-time MDX integration](/docs/integrations/typescript#mdx-integration) instead.

</Callout>

It generates a table for your docs based on TypeScript definitions.

## Usage [#usage]

```npm
npm i fumadocs-typescript
```

Initialize the TypeScript compiler and add it as a MDX component.

```tsx title="components/mdx.tsx"
import defaultComponents from 'fumadocs-ui/mdx';
import type { MDXComponents } from 'mdx/types';
import { createGenerator, createFileSystemGeneratorCache } from 'fumadocs-typescript';
import { AutoTypeTable, type AutoTypeTableProps } from 'fumadocs-typescript/ui';

const generator = createGenerator({
  // set a cache, necessary for serverless platform like Vercel
  cache: createFileSystemGeneratorCache('.next/fumadocs-typescript'),
});

export function getMDXComponents(components?: MDXComponents) {
  return {
    ...defaultComponents,
    // [!code ++]
    AutoTypeTable: (props: Partial<AutoTypeTableProps>) => (
      <AutoTypeTable {...props} generator={generator} />
    ),
    ...components,
  } satisfies MDXComponents;
}
```

You can now reference `<AutoTypeTable />` in your MDX content.

See [TypeScript DocGen](/docs/integrations/typescript) for more usages.

### From File [#from-file]

It accepts a `path` prop that points to a typescript file, and `name` for the exported type name.

```ts title="path/to/file.ts"
export interface MyInterface {
  name: string;
}
```

```mdx title="content.mdx"
<AutoTypeTable path="./path/to/file.ts" name="MyInterface" />
```

The path is relative to your project directory (`cwd`), because `AutoTypeTable` is a React Server Component, it cannot access build-time information like MDX file path.

### From Type [#from-type]

You can specify the type to generate, without an actual TypeScript file.

```mdx title="content.mdx"
<AutoTypeTable type="{ hello: string }" />
```

When a `path` is given, it shares the same context as the TypeScript file.

```ts title="file.ts"
export type A = { hello: string };
```

```mdx title="content.mdx"
<AutoTypeTable path="file.ts" type="A & { world: string }" />
```

When `type` has multiple lines, the export statement and `name` prop are required.

```mdx title="content.mdx"
<AutoTypeTable
  path="file.ts"
  name="B"
  type={`
import { ReactNode } from "react"
export type B = ReactNode | { world: string }
`}
/>
```

### Functions [#functions]

Notice that only object type is allowed. For functions, you should wrap them into an object instead.

```ts
export interface MyInterface {
  myFn: (input: string) => void;
}
```

## TypeScript Compiler [#typescript-compiler]

Under the hood, it uses the [Typescript Compiler API](https://github.com/microsoft/TypeScript/wiki/Using-the-Compiler-API) to extract type information.
Your `tsconfig.json` file in the current working directory will be loaded.

You can change the compiler settings from [`createGenerator()`](/docs/integrations/typescript).

```ts
import { createGenerator, createFileSystemGeneratorCache } from 'fumadocs-typescript';

const generator = createGenerator({
  tsconfigPath: './tsconfig.json',
  // where to resolve relative paths (normally cwd)
  basePath: './',
  // other options...
});
```

### File System [#file-system]

It relies on the file system, hence, the page referencing this component must be built in **build time**. Rendering the component on serverless runtime may cause problems.

## References [#references]

### AutoTypeTableProps

| Prop              | Type       | Description                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ----------------- | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `path?`           | `string`   | The path to source TypeScript file.                                                                                                                                                                                                                                                                                                                                                                                             |
| `name?`           | `string`   | Exported type name to generate from.                                                                                                                                                                                                                                                                                                                                                                                            |
| `type?`           | `string`   | Set the type to generate from. When used with `name`, it generates the type with `name` as export name. ```ts export const myName = MyType; ``` When `type` contains multiple lines, `export const` is not added. You need to export it manually, and specify the type name with `name`. ```tsx <AutoTypeTable path="./file.ts" type={`import { ReactNode } from "react" export const MyName = ReactNode`} name="MyName" /> ``` |
| `generator`       | `object`   |                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `shiki?`          | `union`    | Shiki configuration when using default `renderMarkdown` & `renderType`                                                                                                                                                                                                                                                                                                                                                          |
| `options?`        | `object`   |                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `renderMarkdown?` | `function` |                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `renderType?`     | `function` |                                                                                                                                                                                                                                                                                                                                                                                                                                 |
