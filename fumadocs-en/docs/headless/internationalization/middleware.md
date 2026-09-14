# Fumadocs Core (the core library of Fumadocs): Middleware

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/headless/internationalization/middleware.mdx

Next.js proxy for implementing i18n routing

## Setup [#setup]

Redirects users to appropriate locale, it can be customized from `i18n.ts` config file.

```ts title="proxy.ts"
import { createI18nMiddleware } from 'fumadocs-core/i18n/middleware';
import { i18n } from '@/lib/i18n';

export default createI18nMiddleware(i18n);

export const config = {
  // Matcher ignoring `/_next/` and `/api/`
  // You may need to adjust it to ignore static assets in `/public` folder
  matcher: ['/((?!api|_next/static|_next/image|favicon.ico).*)'],
};

```

> When `hideLocale` is enabled, it uses `NextResponse.rewrite` to hide locale prefixes.
