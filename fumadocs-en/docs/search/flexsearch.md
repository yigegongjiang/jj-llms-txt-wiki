# Fumadocs (Framework Mode): FlexSearch

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/(framework)/search/flexsearch.mdx

A fast & minimal search engine.

## Overview [#overview]

For the setup guide, see [Integrate FlexSearch](/docs/headless/search/flexsearch).

It works through a API endpoint (running on server), or a statically cached JSON file for static websites.

## Setup [#setup]

Create a search dialog.

<Tabs items={['fetch (default)', 'static']}>

    <Tab>

```tsx title="components/search.tsx"
'use client';
import { useDocsSearch } from 'fumadocs-core/search/client';
import {
  SearchDialog,
  SearchDialogClose,
  SearchDialogContent,
  SearchDialogHeader,
  SearchDialogIcon,
  SearchDialogInput,
  SearchDialogList,
  SearchDialogOverlay,
  type SharedProps,
} from 'fumadocs-ui/components/dialog/search';
import { useI18n } from 'fumadocs-ui/contexts/i18n';
import { fetchClient } from 'fumadocs-core/search/client/fetch';

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
    </SearchDialog>
  );
}

```

    </Tab>

    <Tab id='static'>

For Static Export, you can configure [static mode](/docs/headless/search/flexsearch#static-export) on search server, and change the search client:

```tsx title="components/search.tsx"
'use client';
import {
  SearchDialog,
  SearchDialogClose,
  SearchDialogContent,
  SearchDialogHeader,
  SearchDialogIcon,
  SearchDialogInput,
  SearchDialogList,
  SearchDialogOverlay,
  type SharedProps,
} from 'fumadocs-ui/components/dialog/search';
import { useDocsSearch } from 'fumadocs-core/search/client';
import { flexsearchStaticClient } from 'fumadocs-core/search/client/flexsearch-static';
import { useI18n } from 'fumadocs-ui/contexts/i18n';

export default function DefaultSearchDialog(props: SharedProps) {
  const { locale } = useI18n(); // (optional) for i18n
  const { search, setSearch, query } = useDocsSearch({
    client: flexsearchStaticClient({ locale }),
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
    </SearchDialog>
  );
}

```

    </Tab>

</Tabs>

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

Optionally, you can add UI for filtering results by tags. Configure [Tag Filter](/docs/headless/search/flexsearch#tag-filter) on search server and add the following:

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
import { flexsearchStaticClient } from 'fumadocs-core/search/client/flexsearch-static';

export default function CustomSearchDialog(props: SharedProps) {
  // [!code ++]
  const [tag, setTag] = useState<string | undefined>();
  const { search, setSearch, query } = useDocsSearch({
    client: flexsearchStaticClient({
      // [!code ++]
      tag,
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
