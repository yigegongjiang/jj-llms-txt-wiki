# Fumadocs UI (the default theme of Fumadocs): Type Table

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/ui/components/type-table.mdx

A table for documenting types

<Installation name="type-table" />

## Usage [#usage]

It accepts a `type` property.

```mdx
import { TypeTable } from 'fumadocs-ui/components/type-table';

<TypeTable
  type={{
    percentage: {
      description: 'The percentage of scroll position to display the roll button',
      type: 'number',
      default: 0.2,
    },
  }}
/>
```

## References [#references]

### Type Table [#type-table]

### TypeTableProps

| Prop   | Type                     | Description |
| ------ | ------------------------ | ----------- |
| `type` | `Record<string, object>` |             |


### Object Type [#object-type]

### ObjectTypeProps

| Prop                   | Type        | Description                                          |
| ---------------------- | ----------- | ---------------------------------------------------- |
| `description?`         | `ReactNode` | Additional description of the field                  |
| `type`                 | `ReactNode` | type signature (short)                               |
| `typeDescription?`     | `ReactNode` | type signature (full)                                |
| `typeDescriptionLink?` | `string`    | Optional `href` for the type                         |
| `default?`             | `ReactNode` |                                                      |
| `required?`            | `union`     |                                                      |
| `deprecated?`          | `union`     |                                                      |
| `parameters?`          | `array`     | a list of parameters info if the type is a function. |
| `returns?`             | `ReactNode` |                                                      |
