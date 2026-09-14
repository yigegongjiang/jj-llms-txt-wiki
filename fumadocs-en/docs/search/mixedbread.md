# Fumadocs (Framework Mode): Mixedbread

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/(framework)/search/mixedbread.mdx

Using Mixedbread with Fumadocs UI.

## Setup [#setup]

Mixedbread search runs on your server at request time, it isn't available for static sites.

Fumadocs CLI can set it up for you, it creates the search dialog, the search route and a script to sync your content after each build:

```package-install
npx @fumadocs/cli feature search --provider mixedbread
```

Or manually:

1. Integrate [Mixedbread Search](/docs/headless/search/mixedbread).

2. Create a search dialog component.

   ```tsx title="components/search.tsx"
'use client';

import {
  SearchDialog,
  SearchDialogClose,
  SearchDialogContent,
  SearchDialogFooter,
  SearchDialogHeader,
  SearchDialogIcon,
  SearchDialogInput,
  SearchDialogList,
  SearchDialogOverlay,
  type SharedProps,
} from 'fumadocs-ui/components/dialog/search';
import { useDocsSearch } from 'fumadocs-core/search/client';
import { fetchClient } from 'fumadocs-core/search/client/fetch';
import { useI18n } from 'fumadocs-ui/contexts/i18n';

export default function CustomSearchDialog(props: SharedProps) {
  const { locale } = useI18n(); // (optional) for i18n
  const { search, setSearch, query } = useDocsSearch({
    client: fetchClient({
      api: '/api/search',
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
        <SearchDialogFooter>
          <a
            href="https://mixedbread.com"
            rel="noreferrer noopener"
            className="ms-auto text-xs text-fd-muted-foreground"
          >
            Search powered by Mixedbread
          </a>
        </SearchDialogFooter>
      </SearchDialogContent>
    </SearchDialog>
  );
}

```

### Replace Search Dialog

Replace the search dialog with yours from [`<RootProvider />`](/docs/ui/layouts/root-provider):

```tsx
import { RootProvider } from 'fumadocs-ui/provider/<framework>';
// [!code ++]
import SearchDialog from '@/components/search';

<RootProvider
  // [!code ++:3]
  search={{
    SearchDialog,
  }}
>
  {children}
</RootProvider>;
```

If it was in a server component, you would need a separate client component for provider to pass functions:

```tsx tab="provider.tsx"
'use client';
import { RootProvider } from 'fumadocs-ui/provider/<framework>';
import SearchDialog from '@/components/search';
import type { ReactNode } from 'react';

export function Provider({ children }: { children: ReactNode }) {
  return (
    <RootProvider
      search={{
        SearchDialog,
      }}
    >
      {children}
    </RootProvider>
  );
}
```

```tsx tab="app/layout.tsx"
import { Provider } from './provider';
import type { ReactNode } from 'react';

export default function Layout({ children }: { children: ReactNode }) {
  return (
    <html lang="en">
      <body>
        {/* [!code --] */}
        <RootProvider>{children}</RootProvider>
        {/* [!code ++] */}
        <Provider>{children}</Provider>
      </body>
    </html>
  );
}
```

### Tag Filter [#tag-filter]

Optionally, you can add UI for filtering results by tags. Configure [Tag Filter](/docs/headless/search/mixedbread#tag-filter) on search server and add the following:

```tsx
'use client';

import {
  SearchDialog,
  SearchDialogContent,
  SearchDialogFooter,
  SearchDialogOverlay,
  type SharedProps,
  TagsList,
  TagsListItem,
} from 'fumadocs-ui/components/dialog/search';
import { useState } from 'react';
import { useDocsSearch } from 'fumadocs-core/search/client';
import { fetchClient } from 'fumadocs-core/search/client/fetch';

export default function CustomSearchDialog(props: SharedProps) {
  // [!code ++]
  const [tag, setTag] = useState<string | undefined>();
  const { search, setSearch, query } = useDocsSearch({
    client: fetchClient({
      tag, // [!code ++]
    }),
  });

  return (
    <SearchDialog>
      <SearchDialogOverlay />
      <SearchDialogContent>
        ...
        <SearchDialogFooter className="flex flex-row">
          {/* [!code ++:3] */}
          <TagsList tag={tag} onTagChange={setTag}>
            <TagsListItem value="my-value">My Value</TagsListItem>
          </TagsList>
        </SearchDialogFooter>
      </SearchDialogContent>
    </SearchDialog>
  );
}
```
