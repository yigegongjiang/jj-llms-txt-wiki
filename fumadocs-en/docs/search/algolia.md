# Fumadocs (Framework Mode): Algolia

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/(framework)/search/algolia.mdx

Using Algolia with Fumadocs UI.

## Overview [#overview]

For the setup guide, see [Integrate Algolia Search](/docs/headless/search/algolia).

While generally we recommend building your own search with their client-side
SDK, you can also plug the built-in dialog interface.

## Setup [#setup]

Fumadocs CLI can set it up for you, it creates the search dialog, the `static.json` route and a script to sync the search index after each build:

```package-install
npx @fumadocs/cli feature search --provider algolia
```

Or manually:

Create a search dialog, replace `appId`, `apiKey` and `indexName` with your desired values.

```tsx title="components/search.tsx"
'use client';
import { liteClient } from 'algoliasearch/lite';
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
import { algoliaClient } from 'fumadocs-core/search/client/algolia';
import { useI18n } from 'fumadocs-ui/contexts/i18n';

const appId = 'replace me';
const apiKey = 'replace me';
const algolia = liteClient(appId, apiKey);

export default function CustomSearchDialog(props: SharedProps) {
  const { locale } = useI18n(); // (optional) for i18n
  const { search, setSearch, query } = useDocsSearch({
    client: algoliaClient({
      client: algolia,
      indexName: 'document',
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
            href="https://algolia.com"
            rel="noreferrer noopener"
            className="ms-auto text-xs text-fd-muted-foreground"
          >
            Search powered by Algolia
          </a>
        </SearchDialogFooter>
      </SearchDialogContent>
    </SearchDialog>
  );
}

```

<Callout title="Note" className='mt-4'>

    `useDocsSearch()` doesn't use instant search (their official
    Javascript client).

</Callout>

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

Optionally, you can add UI for filtering results by tags. Configure [Tag Filter](/docs/headless/search/algolia#tag-filter) on search server and add the following:

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
