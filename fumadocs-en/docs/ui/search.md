# Fumadocs UI (the default theme of Fumadocs): Search UI

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/ui/search.mdx

The UI for document search

## Overview [#overview]

You can customize Search UI from [`<RootProvider />`](/docs/ui/layouts/root-provider).

```tsx title="app/layout.tsx"
import { RootProvider } from 'fumadocs-ui/provider/<framework>';
import type { ReactNode } from 'react';
import { SearchDialog } from '@/components/my-search-dialog';

export default function Layout({ children }: { children: ReactNode }) {
  return (
    <html>
      <body>
        <RootProvider
          // [!code highlight:4]
          search={{
            enabled: false, // disable search entirely
            SearchDialog, // replace search dialog
          }}
        >
          {children}
        </RootProvider>
      </body>
    </html>
  );
}
```

### Hot Keys [#hot-keys]

Customize the hot keys to trigger search dialog, by default it's <kbd>⌘</kbd> <kbd>K</kbd> or <kbd>Ctrl</kbd> <kbd>K</kbd>.

```tsx
import { RootProvider } from 'fumadocs-ui/provider/<framework>';

<RootProvider
  search={{
    hotKey: [
      {
        display: 'K',
        key: 'k', // key code, or a function determining whether the key is pressed
      },
    ],
  }}
>
  {children}
</RootProvider>;
```

### References [#references]

A full list of options.

### SearchProps

| Prop            | Type              | Description                                                                                                                                                                      |
| --------------- | ----------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `preload?`      | `union`           | **Deprecated.** Preload search dialog before opening it Default: ``true``                                                                                                        |
| `links?`        | `array`           | Custom links to be displayed if search is empty                                                                                                                                  |
| `hotKey?`       | `array`           | Hotkeys for triggering search dialog Default: `Meta/Ctrl + K`                                                                                                                    |
| `SearchDialog?` | `union`           | Replace default search dialog, allowing you to use other solutions such as Algolia Search It receives the `open` and `onOpenChange` prop, can be lazy loaded with `React.lazy()` |
| `options?`      | `Partial<object>` | Additional props to the dialog                                                                                                                                                   |
| `enabled?`      | `union`           | Enable search functionality Default: ``true``                                                                                                                                    |


## Custom UI [#custom-ui]

Fumadocs UI also exposes a lower level `<SearchDialog />` component for advanced use, you can create your own search dialog to replace the default one.

```tsx twoslash
'use client';
import React from 'react';
import { useDocsSearch } from 'fumadocs-core/search/client';
import { fetchClient } from 'fumadocs-core/search/client/fetch';
import {
  SearchDialog,
  SearchDialogClose,
  SearchDialogContent,
  SearchDialogHeader,
  SearchDialogFooter,
  SearchDialogIcon,
  SearchDialogInput,
  SearchDialogList,
  SearchDialogOverlay,
  type SharedProps,
} from 'fumadocs-ui/components/dialog/search';
import { useI18n } from 'fumadocs-ui/contexts/i18n';

export default function DefaultSearchDialog(props: SharedProps) {
  const { locale } = useI18n(); // (optional) for i18n
  const { search, setSearch, query } = useDocsSearch({
    client: fetchClient({
      locale,
    }),
  });

  return (
    <SearchDialog search={search} onSearchChange={setSearch} isLoading={query.isLoading} {...props}>
      <SearchDialogOverlay />
      <SearchDialogContent>
        <SearchDialogHeader>
          <SearchDialogIcon />
          <SearchDialogInput />
          <SearchDialogClose />
        </SearchDialogHeader>
        <SearchDialogList items={query.data !== 'empty' ? query.data : null} />
      </SearchDialogContent>
      <SearchDialogFooter>{/* footer items */}</SearchDialogFooter>
    </SearchDialog>
  );
}
```

### Content Renderer [#content-renderer]

The content in search result supports Markdown, match highlights are expressed using `<mark />`.

You can customize the Markdown renderer.

```tsx title="components/search.tsx"
<SearchDialogList
  items={query.data !== 'empty' ? query.data : null}
  // [!code highlight:8]
  Item={(props) => (
    <SearchDialogListItem
      {...props}
      renderMarkdown={(text) => {
        // ...
      }}
    />
  )}
/>
```
