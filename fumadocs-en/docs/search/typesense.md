# Fumadocs (Framework Mode): Typesense

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/(framework)/search/typesense.mdx

Using Typesense with Fumadocs UI.

> This is a community-maintained integration.

## Setup [#setup]

Fumadocs CLI can set it up for you, it creates the search dialog, the `static.json` route and a script to sync the search index after each build:

```package-install
npx @fumadocs/cli feature search --provider typesense
```

Or manually:

1. Integrate [Typesense Search](/docs/headless/search/typesense).

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
   import { useI18n } from 'fumadocs-ui/contexts/i18n';
   import { Client } from 'typesense';
   import { useTypesenseSearch } from 'typesense-fumadocs-adapter/client';

   const client = new Client({
     nodes: [{ url: 'YOUR_TYPESENSE_SERVER_URL' }],
     apiKey: 'YOUR_TYPESENSE_SEARCH_ONLY_API_KEY',
   });

   export default function TypesenseSearchDialog(props: SharedProps) {
     const { locale } = useI18n(); // optional
     const { search, setSearch, query } = useTypesenseSearch({
       typesenseCollectionName: `YOUR_TYPESENSE_COLLECTION_NAME`,
       locale,
       legacy: false, // optional, set to true for fumadocs-ui version < 16.6.0
       client,
     });

     return (
       <SearchDialog
         search={search}
         onSearchChange={setSearch}
         isLoading={query.isLoading}
         {...props}
       >
         <SearchDialogOverlay />
         <SearchDialogContent>
           <SearchDialogHeader>
             <SearchDialogIcon />
             <SearchDialogInput />
             <SearchDialogClose />
           </SearchDialogHeader>
           <SearchDialogList items={query.data !== 'empty' ? query.data : null} />
           <SearchDialogFooter>
             <div className="w-full text-right text-xs text-fd-muted-foreground">
               <a href="https://typesense.org" rel="noreferrer noopener" target="_blank">
                 Search powered by Typesense
               </a>
             </div>
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

Optionally, you can add UI for filtering results by tags. Configure [Tag Filter](/docs/headless/search/typesense#tag-filter) on search server and add the following:

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
import { useTypesenseSearch } from 'typesense-fumadocs-adapter/client';

export default function TypesenseSearchDialog(props: SharedProps) {
  // [!code ++]
  const [tag, setTag] = useState<string | undefined>();
  const { search, setSearch, query } = useTypesenseSearch({
    tag, // [!code ++]
    // other options...
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
