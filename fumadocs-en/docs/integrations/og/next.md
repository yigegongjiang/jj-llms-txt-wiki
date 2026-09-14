# Fumadocs (Framework Mode): next/og

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/(framework)/integrations/og/next.mdx

Usage with Next.js Metadata API.

> Make sure to read their [Metadata section](https://nextjs.org/docs/app/building-your-application/optimizing/metadata) for the fundamentals of Metadata API.

## Metadata Image [#metadata-image]

You can generate metadata images dynamically using `next/og`.

Add the following under your loader, and define image metadata for pages:

```ts tab="lib/source.ts"
// [!code ++:8]
export function getPageImageUrl(page: (typeof source)['$inferPage']) {
  const segments = [...page.slugs, 'image.png'];

  return {
    segments,
    url: '/' + [page.locale, 'og', 'docs', ...segments].filter(Boolean).join('/'),
  };
}
```

```tsx tab="app/docs/[[...slug]]/page.tsx"
import { notFound } from 'next/navigation';
import { source, getPageImageUrl } from '@/lib/source';
import type { Metadata } from 'next';

export async function generateMetadata(props: PageProps<'/docs/[[...slug]]'>): Promise<Metadata> {
  const params = await props.params;
  const page = source.getPage(params.slug);
  if (!page) notFound();

  return {
    title: page.data.title,
    description: page.data.description,
    openGraph: {
      // [!code ++]
      images: getPageImageUrl(page).url,
    },
  };
}
```

> We append `image.png` to the end of slugs so that we can access it via `/og/docs/my-page/image.png`.

Finally, create a route handler to generate images at build time:

```tsx title="app/og/docs/[...slug]/route.tsx"
import { source } from '@/lib/source';
import { notFound } from 'next/navigation';
import { generateOGImage } from 'fumadocs-ui/og';
import { appName, getPageImageUrl } from '@/lib/shared';

export const revalidate = false;

export async function GET(_req: Request, { params }: RouteContext<'/og/docs/[...slug]'>) {
  const { slug } = await params;
  const page = source.getPage(slug.slice(0, -1));
  if (!page) notFound();

  return generateOGImage({
    title: page.data.title,
    description: page.data.description,
    site: appName,
  });
}

export function generateStaticParams() {
  return source.getPages().map((page) => ({
    lang: page.locale,
    slug: getPageImageUrl(page).segments,
  }));
}

```

You can specify options for Satori (used by `next/og`), see https://github.com/vercel/satori for reference.

### Other Presets [#other-presets]

There's other available styles on Fumadocs CLI, such as `mono`:

```npm
npx @fumadocs/cli@latest add og/mono
```

Replace your old `generate` with the installed one.
