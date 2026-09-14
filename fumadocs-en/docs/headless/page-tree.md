# Fumadocs Core (the core library of Fumadocs): Page Tree

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/headless/page-tree.mdx

The structure of page tree.

Page tree is a tree structure that describes all navigation links, with other items like separator and folders.

It will be sent to the client and being referenced in navigation elements including the sidebar and breadcrumb.
Hence, you shouldn't store any sensitive or large data in page tree.

<Callout title="Note">

By design, page tree only contains necessary information of all pages and folders.

Unserializable data such as functions can't be passed to page tree.

</Callout>

## Conventions [#conventions]

You can import the type definitions of page tree, useful if you want to hardcode or generate it.

```ts
import type * as PageTree from 'fumadocs-core/page-tree';

const tree: PageTree.Root = {
  // props
};
```

Certain nodes contain a `$ref` property, they are internal and not used when hardcoding it.

### Root [#root]

The initial root of page trees.

### PageTreeRoot

| Prop           | Type        | Description                                                               |
| -------------- | ----------- | ------------------------------------------------------------------------- |
| `type?`        | `"root"`    |                                                                           |
| `name`         | `ReactNode` |                                                                           |
| `description?` | `ReactNode` |                                                                           |
| `children`     | `array`     |                                                                           |
| `fallback?`    | `object`    | Another page tree that won't be displayed unless being opened.            |
| `$id?`         | `string`    | ID for the node, unique in all page trees (even across different locales) |


### Page [#page]

A node representing link.

```json
{
  "type": "page",
  "name": "Quick Start",
  "url": "/docs"
}
```

> External urls are also supported

### PageTreeItem

| Prop           | Type        | Description                                                                                                                 |
| -------------- | ----------- | --------------------------------------------------------------------------------------------------------------------------- |
| `type`         | `"page"`    |                                                                                                                             |
| `name`         | `ReactNode` |                                                                                                                             |
| `url`          | `string`    |                                                                                                                             |
| `external?`    | `union`     | Whether the link should be treated as external (e.g. use HTML <a> tag). When unspecified, it depends on the value of `url`. |
| `description?` | `ReactNode` |                                                                                                                             |
| `icon?`        | `ReactNode` |                                                                                                                             |
| `$id?`         | `string`    | ID for the node, unique in all page trees (even across different locales)                                                   |


### Folder [#folder]

A node containing multiple children nodes.

```json
{
    "type": "folder",
    "name": "Guide",
    "index": {
        "type": "page",
        ...
    }
    "children": [
        ...
    ]
}
```

### PageTreeFolder

| Prop           | Type        | Description                                                                                                                                                                                                                                                      |
| -------------- | ----------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`         | `"folder"`  |                                                                                                                                                                                                                                                                  |
| `name`         | `ReactNode` |                                                                                                                                                                                                                                                                  |
| `description?` | `ReactNode` |                                                                                                                                                                                                                                                                  |
| `root?`        | `union`     | Mark as a root folder, `true` for the default type. A string value specifies its **root type**: root folders of the same type under the same parent scope are interchangeable (e.g. versions), and pages can map to their structural projection in another root. |
| `defaultOpen?` | `union`     |                                                                                                                                                                                                                                                                  |
| `collapsible?` | `union`     |                                                                                                                                                                                                                                                                  |
| `index?`       | `object`    |                                                                                                                                                                                                                                                                  |
| `icon?`        | `ReactNode` |                                                                                                                                                                                                                                                                  |
| `children`     | `array`     |                                                                                                                                                                                                                                                                  |
| `$id?`         | `string`    | ID for the node, unique in all page trees (even across different locales)                                                                                                                                                                                        |


### Separator [#separator]

A label between items.

```json
{
  "type": "separator",
  "name": "Components"
}
```

### PageTreeSeparator

| Prop    | Type          | Description                                                               |
| ------- | ------------- | ------------------------------------------------------------------------- |
| `type`  | `"separator"` |                                                                           |
| `name?` | `ReactNode`   |                                                                           |
| `icon?` | `ReactNode`   |                                                                           |
| `$id?`  | `string`      | ID for the node, unique in all page trees (even across different locales) |


## Icons [#icons]

Icon is a `ReactElement`, supported by pages and folders.
