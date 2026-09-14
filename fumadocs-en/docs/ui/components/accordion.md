# Fumadocs UI (the default theme of Fumadocs): Accordion

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/ui/components/accordion.mdx

Add Accordions to your documentation

<Installation name="accordion" />

## Usage [#usage]

Based on
[Base UI Accordion](https://base-ui.com/react/components/accordion), useful for FAQ sections.

> For the Radix UI package, this is based on [Radix UI Accordion](https://www.radix-ui.com/primitives/docs/components/accordion) instead.

Use it in MDX files or as a normal React component.

```mdx tab="MDX"
---
title: Hello World
---

import { Accordion, Accordions } from 'fumadocs-ui/components/accordion';

<Accordions type="single">
  <Accordion title="My Title">My Content</Accordion>
</Accordions>
```

```tsx tab="React.js"
import { Accordion, Accordions } from 'fumadocs-ui/components/accordion';

export default function Page() {
  return (
    <Accordions type="single">
      <Accordion title="My Title">My Content</Accordion>
    </Accordions>
  );
}
```

### Accordions [#accordions]

### AccordionsProps

| Prop                | Type    | Description                                                                                                                                                                                                              |
| ------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `render?`           | `union` | Allows you to replace the component's HTML element with a different tag, or compose it with another component. Accepts a `ReactElement` or a function that returns the element to render.                                |
| `disabled?`         | `union` | Whether the component should ignore user interaction. Default: `false`                                                                                                                                                   |
| `hiddenUntilFound?` | `union` | Allows the browser's built-in page search to find and expand the panel contents. Overrides the `keepMounted` prop and uses `hidden="until-found"` to hide the element without removing it from the DOM. Default: `false` |
| `keepMounted?`      | `union` | Whether to keep the element in the DOM while the panel is closed. This prop is ignored when `hiddenUntilFound` is used. Default: `false`                                                                                 |
| `loopFocus?`        | `union` | **Deprecated.** Deprecated following the [APG guidance update](https://github.com/w3c/aria-practices/pull/3434) to remove roving focus. This prop no longer affects keyboard focus behavior.                             |
| `multiple?`         | `union` | Whether multiple items can be open at the same time. Default: `false`                                                                                                                                                    |
| `orientation?`      | `union` | **Deprecated.** Deprecated following the [APG guidance update](https://github.com/w3c/aria-practices/pull/3434) to remove roving focus. This prop no longer affects keyboard focus behavior. Default: `'vertical'`       |


### Accordion [#accordion]

### AccordionProps

| Prop            | Type       | Description                                                                                                                                                                               |
| --------------- | ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `render?`       | `union`    | Allows you to replace the component's HTML element with a different tag, or compose it with another component. Accepts a `ReactElement` or a function that returns the element to render. |
| `disabled?`     | `union`    | Whether the component should ignore user interaction. Default: `false`                                                                                                                    |
| `onOpenChange?` | `function` | Event handler called when the panel is opened or closed.                                                                                                                                  |
| `value?`        | `string`   |                                                                                                                                                                                           |


### Linking to Accordion [#linking-to-accordion]

You can specify an `id` for accordion. The accordion will automatically open when the user is navigating to the page with the specified `id` in hash parameter.

```mdx
<Accordions>
<Accordion title="My Title" id="my-title">

My Content

</Accordion>
</Accordions>
```

> The value of accordion is same as title by default. When an id presents, it will be used as the value instead.
