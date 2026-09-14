# Fumadocs UI (the default theme of Fumadocs): Layouts

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/ui/layouts/index.mdx

A list of layout components.

## Overview [#overview]

Fumadocs UI provides essential layouts to display content.

<DocsCategory />

### Configurations [#configurations]

Each layout supports a shared set of options.

It is recommended to store these options into a file, and pass them to the layouts.

```tsx tab="Shared Options" title="lib/layout.shared.tsx"
import type { BaseLayoutProps } from 'fumadocs-ui/layouts/shared';

export function baseOptions(): BaseLayoutProps {
  return {
    nav: {
      title: 'My App',
    },
  };
}
```

```tsx tab="Docs Layout" title="app/docs/layout.tsx"
import { DocsLayout } from 'fumadocs-ui/layouts/docs';
import { baseOptions } from '@/lib/layout.shared';
import { source } from '@/lib/source';
import type { ReactNode } from 'react';

export default function Layout({ children }: { children: ReactNode }) {
  return (
    <DocsLayout
      // [!code highlight]
      {...baseOptions()}
      tree={source.getPageTree()}
    >
      {children}
    </DocsLayout>
  );
}
```

```tsx tab="Home Layout" title="app/(home)/layout.tsx"
import { HomeLayout } from 'fumadocs-ui/layouts/home';
import { baseOptions } from '@/lib/layout.shared';
import type { ReactNode } from 'react';

export default function Layout({ children }: { children: ReactNode }) {
  return (
    <HomeLayout
      // [!code highlight]
      {...baseOptions()}
    >
      {children}
    </HomeLayout>
  );
}
```

See detailed docs for [`links`](/docs/ui/layouts/links) and [`nav`](/docs/ui/layouts/nav) options.

### BaseLayoutProps

| Prop            | Type              | Description       |
| --------------- | ----------------- | ----------------- |
| `githubUrl?`    | `string`          | GitHub url        |
| `links?`        | `array`           |                   |
| `nav?`          | `object`          | navigation config |
| `slots?`        | `Partial<object>` |                   |
| `children?`     | `ReactNode`       |                   |
| `themeSwitch?`  | `object`          |                   |
| `searchToggle?` | `object`          |                   |
| `i18n?`         | `union`           | **Deprecated.**   |
