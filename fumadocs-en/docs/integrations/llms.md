# Fumadocs (Framework Mode): AI & LLMs

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/(framework)/integrations/llms.mdx

Integrate AI functionality to Fumadocs.

## Docs for LLM [#docs-for-llm]

Serve your docs as Markdown for LLMs and AI agents:

```package-install
npx @fumadocs/cli feature llms
```

It enables processed Markdown on your docs collection, adds `docsLlms` to `lib/source.ts`, and creates the routes below. The generated files are listed here if you prefer a manual setup.

### `docsLlms` [#docsllms]

`llms()` renders your docs for LLMs. `renderPage` turns one page into Markdown, from the processed document rather than the raw file content:

```ts title="lib/source.ts"
import { llms } from 'fumadocs-core/source';

export const docsLlms = llms(source, {
  renderPage: async (page) => `# ${page.data.title} (${page.url})

${await page.data.getText('processed')}`,
});

```

| Method         | Output                                         |
| -------------- | ---------------------------------------------- |
| `index(lang?)` | the `llms.txt` index, built from the page tree |
| `page(page)`   | one page, rendered with `renderPage`           |
| `full(lang?)`  | every page rendered with `renderPage`, joined  |

<Callout title="renderPage is required">
  `page()` and `full()` are only available when you pass `renderPage`, since Fumadocs cannot know
  how your content source exposes its Markdown.
</Callout>

Runtime content sources like [`@fumadocs/local-md`](/docs/integrations/content/local-md) resolve on demand. Pass their `getSource` and the CLI generates the same routes:

```ts
export const docsLlms = llms(getSource, {
  renderPage: (page) => `# ${page.data.title} (${page.url})

${page.data.content}`,
});
```

It requires `includeProcessedMarkdown` in **Fumadocs MDX**:

```ts title="source.config.ts"
import { defineDocs } from 'fumadocs-mdx/config';

export const docs = defineDocs({
  dir: 'content/docs',
  docs: { // [!code ++:5]
    postprocess: {
      includeProcessedMarkdown: true,
    },
  },
});

```

MDX components appear as JSX syntax by default, you can render them into meaningful Markdown with the [`output` option](/docs/headless/mdx/remark-llms#output).

### `llms.txt` [#llmstxt]

An index of all pages, generated from the page tree.

```ts tab="Next.js" title="app/llms.txt/route.ts"
import { docsLlms } from '@/lib/source';

export const revalidate = false;

export async function GET() {
  return new Response(await docsLlms.index());
}

```

```ts tab="React Router" title="app/routes.ts"
import { index, route, type RouteConfig } from '@react-router/dev/routes';

export default [
  index('routes/home.tsx'),
  route('llms.txt', 'routes/llms.txt.ts'), // [!code ++:3]
  route('llms-full.txt', 'routes/llms-full.txt.ts'),
  route('llms.mdx/docs/*', 'routes/llms.mdx.docs.ts'),
] satisfies RouteConfig;

```

```ts tab="React Router" title="app/routes/llms.txt.ts"
import { docsLlms } from '@/lib/source';

export async function loader() {
  return new Response(await docsLlms.index());
}

```

```ts tab="Tanstack Start" title="src/routes/llms[.]txt.ts"
import { createFileRoute } from '@tanstack/react-router';
import { docsLlms } from '@/lib/source';

export const Route = createFileRoute('/llms.txt')({
  server: {
    handlers: {
      GET: async () => new Response(await docsLlms.index()),
    },
  },
});

```

```ts tab="Waku" title="pages/_api/llms.txt.ts"
import { docsLlms } from '@/lib/source';

export async function GET() {
  return new Response(await docsLlms.index());
}

export async function getConfig() {
  return {
    render: 'static' as const,
  } as const;
}

```

### `llms-full.txt` [#llms-fulltxt]

The content of all pages in a single file.

```ts tab="Next.js" title="app/llms-full.txt/route.ts"
import { docsLlms } from '@/lib/source';

export const revalidate = false;

export async function GET() {
  return new Response(await docsLlms.full());
}

```

```ts tab="React Router" title="app/routes/llms-full.txt.ts"
import { docsLlms } from '@/lib/source';

export async function loader() {
  return new Response(await docsLlms.full());
}

```

```ts tab="Tanstack Start" title="src/routes/llms-full[.]txt.ts"
import { createFileRoute } from '@tanstack/react-router';
import { docsLlms } from '@/lib/source';

export const Route = createFileRoute('/llms-full.txt')({
  server: {
    handlers: {
      GET: async () => new Response(await docsLlms.full()),
    },
  },
});

```

```ts tab="Waku" title="pages/_api/llms-full.txt.ts"
import { docsLlms } from '@/lib/source';

export async function GET() {
  return new Response(await docsLlms.full());
}

export async function getConfig() {
  return {
    render: 'static' as const,
  } as const;
}

```

### `*.md` [#md-extension]

The Markdown of a single page, for AI agents.

```ts tab="Next.js" title="lib/shared.ts"
import { createGetUrl } from 'fumadocs-core/source';

export const docsContentRoute = '/llms.mdx/docs';

const getContentUrl = createGetUrl(docsContentRoute);

export function getPageMarkdownUrl(page: { slugs: string[]; locale?: string }) {
  const segments = [...page.slugs, 'content.md'];

  return { segments, url: getContentUrl(segments, page.locale) };
}

```

```ts tab="Next.js" title="app/llms.mdx/docs/[[...slug]]/route.ts"
import { docsLlms, source } from '@/lib/source';
import { notFound } from 'next/navigation';

export const revalidate = false;

export async function GET(_req: Request, { params }: RouteContext<'/llms.mdx/docs/[[...slug]]'>) {
  const { slug } = await params;
  // remove the appended "content.md", `/docs/index.md` is rewritten to the root page
  const slugs = slug?.slice(0, -1) ?? [];
  if (slugs.at(-1) === 'index') slugs.pop();
  const page = source.getPage(slugs);
  if (!page) notFound();

  return new Response(await docsLlms.page(page), {
    headers: {
      'Content-Type': 'text/markdown',
    },
  });
}

export function generateStaticParams() {
  return source.generateParams().map((item) => ({
    ...item,
    slug: [...item.slug, 'content.md'],
  }));
}

```

```ts tab="Next.js" title="next.config.ts"
import type { NextConfig } from 'next';

const config: NextConfig = {
  reactStrictMode: true,
  async rewrites() { // [!code ++:8]
    return [
      {
        source: '/docs/:slug*.md',
        destination: '/llms.mdx/docs/:slug*/content.md',
      },
    ];
  },
};

export default config;

```

```ts tab="React Router" title="app/lib/shared.ts"
import { createGetUrl } from 'fumadocs-core/source';

export const docsContentRoute = '/llms.mdx/docs';

const getContentUrl = createGetUrl(docsContentRoute);

export function getPageMarkdownUrl(page: { slugs: string[]; locale?: string }) {
  const segments = [...page.slugs, 'content.md'];

  return { segments, url: getContentUrl(segments, page.locale) };
}

```

```ts tab="React Router" title="app/routes/llms.mdx.docs.ts"
import type { Route } from './+types/llms.mdx.docs';
import { docsLlms, source } from '@/lib/source';

export async function loader({ params }: Route.LoaderArgs) {
  const slugs = params['*'].split('/').filter((v) => v.length > 0);
  // remove the appended "content.md"
  slugs.pop();
  const page = source.getPage(slugs);
  if (!page) return new Response('not found', { status: 404 });

  return new Response(await docsLlms.page(page), {
    headers: {
      'Content-Type': 'text/markdown',
    },
  });
}

```

```ts tab="React Router" title="app/root.tsx"
import { rewritePath } from 'fumadocs-core/negotiation';
import type { Route } from './+types/root';

// serve `/docs/page.md` [!code ++:11]
const { rewrite: rewriteLLM } = rewritePath(
  '/docs{/*path}.md',
  '/llms.mdx/docs{/*path}/content.md',
);
const serverMiddleware: Route.MiddlewareFunction = async ({ request }, next) => {
  const url = new URL(request.url);
  const path = rewriteLLM(url.pathname);
  if (path) return Response.redirect(new URL(path, url));

  return next();
};

export const middleware = [serverMiddleware];
```

```ts tab="React Router" title="react-router.config.ts"
import type { Config } from '@react-router/dev/config';

export default {
  // enable middleware [!code ++:3]
  future: {
    v8_middleware: true,
  },
} satisfies Config;
```

```ts tab="Tanstack Start" title="src/lib/shared.ts"
import { createGetUrl } from 'fumadocs-core/source';

export const docsRoute = '/docs';

const getDocsUrl = createGetUrl(docsRoute);

export function getPageMarkdownUrl(page: { slugs: string[]; locale?: string }) {
  const segments = [...page.slugs];
  if (segments.length === 0) {
    segments.push('index.md');
  } else {
    segments[segments.length - 1] += '.md';
  }

  return { segments, url: getDocsUrl(segments, page.locale) };
}

/** @returns page slugs */
export function decodeMarkdownUrl(segments: string[]) {
  if (segments.length === 0) return [];

  const out = [...segments];
  out[out.length - 1] = out[out.length - 1].replace(/\.md$/, '');
  if (out.length === 1 && out[0] === 'index') out.pop();
  return out;
}

```

```ts tab="Tanstack Start" title="src/routes/docs/{$}[.]md.ts"
import { createFileRoute, notFound } from '@tanstack/react-router';
import { decodeMarkdownUrl } from '@/lib/shared';
import { docsLlms, source } from '@/lib/source';

export const Route = createFileRoute('/docs/{$}.md')({
  server: {
    handlers: {
      GET: async ({ params }) => {
        const slugs = decodeMarkdownUrl(params._splat?.split('/') ?? []);
        const page = source.getPage(slugs);
        if (!page) throw notFound();

        return new Response(await docsLlms.page(page), {
          headers: {
            'Content-Type': 'text/markdown',
          },
        });
      },
    },
  },
});

```

```ts tab="Waku" title="lib/shared.ts"
import { createGetUrl } from 'fumadocs-core/source';

export const docsContentRoute = '/llms.mdx/docs';

const getContentUrl = createGetUrl(docsContentRoute);

export function getPageMarkdownUrl(page: { slugs: string[]; locale?: string }) {
  const segments = [...page.slugs, 'content.md'];

  return { segments, url: getContentUrl(segments, page.locale) };
}

```

```ts tab="Waku" title="pages/_api/llms.mdx/docs/[...slug]/content.md.ts"
import { docsLlms, source } from '@/lib/source';
import type { ApiContext } from 'waku/router';
import { unstable_notFound } from 'waku/router/server';

export async function GET(_: Request, { params }: ApiContext<'/llms.mdx/docs/[...slug]/content.md'>) {
  const page = source.getPage(params.slug);
  if (!page) unstable_notFound();

  return new Response(await docsLlms.page(page), {
    headers: {
      'Content-Type': 'text/markdown',
    },
  });
}

export async function getConfig() {
  return {
    render: 'static' as const,
    staticPaths: source.generateParams().map((item) => item.slug),
  } as const;
}

```

#### `Accept` [#accept]

To serve the Markdown content instead for AI agents, you can leverage the `Accept` header.

```ts title="proxy.ts (Next.js)"
import { NextRequest, NextResponse } from 'next/server';
import { isMarkdownPreferred, rewritePath } from 'fumadocs-core/negotiation';

const { rewrite: rewriteLLM } = rewritePath('/docs{/*path}', '/llms.mdx/docs{/*path}');

export default function proxy(request: NextRequest) {
  if (isMarkdownPreferred(request)) {
    const result = rewriteLLM(request.nextUrl.pathname);

    if (result) {
      return NextResponse.rewrite(new URL(result, request.nextUrl), {
        headers: { Vary: 'Accept' }, // [!code highlight]
      });
    }
  }

  return NextResponse.next();
}
```

Because the same URL now has two representations, the response needs `Vary: Accept` to prevent cache collisions.

<Callout title="Next.js">
  Next.js discards `Vary` on App Router **page** responses, so the HTML side of the branch above
  can't carry the header from inside the app. Set it at your CDN if you serve documentation through
  a shared cache.
</Callout>

### Page Actions [#page-actions]

Common page actions for AI, require [`*.md`](#md-extension) to be implemented first.

![AI Page Actions](/docs/ai-page-actions.png)

Use them in your docs page like:

```tsx title="app/docs/[[...slug]]/page.tsx"
import { MarkdownCopyButton, ViewOptionsPopover } from 'fumadocs-ui/layouts/docs/page';
import { getPageMarkdownUrl } from '@/lib/shared';

const markdownUrl = getPageMarkdownUrl(page).url;

<div className="flex flex-row gap-2 items-center border-b pt-2 pb-6">
  <MarkdownCopyButton markdownUrl={markdownUrl} />
  <ViewOptionsPopover
    markdownUrl={markdownUrl}
    githubUrl={`https://github.com/${owner}/${repo}/blob/main/content/docs/${page.path}`}
  />
</div>;
```

## MCP Server [#mcp-server]

Expose your docs to AI agents through a [MCP](https://modelcontextprotocol.io) server, with tools to list, search and read pages.

```package-install
npx @fumadocs/cli feature mcp
```

It creates a `/api/mcp` route with the streamable HTTP transport, on top of the [LLM routes](#docs-for-llm) above.

The tools come from [`fumadocs-core/mcp`](/docs/headless/utils/mcp), see linked docs for details.

```ts tab="Next.js" title="app/api/mcp/route.ts"
import { createMcpHandler, McpServer } from '@modelcontextprotocol/server';
import { registerSearchTool, registerSourceTools } from 'fumadocs-core/mcp';
import { createFromSource } from 'fumadocs-core/search/server';
import { docsLlms, source } from '@/lib/source';

const handler = createMcpHandler(() => {
  const mcp = new McpServer({
    name: 'docs',
    version: '1.0.0',
  });

  registerSourceTools(mcp, source, docsLlms);
  registerSearchTool(mcp, createFromSource(source));

  return mcp;
});

export async function GET(request: Request) {
  return handler.fetch(request);
}

export async function POST(request: Request) {
  return handler.fetch(request);
}

export async function DELETE(request: Request) {
  return handler.fetch(request);
}

```

```ts tab="React Router" title="app/routes/api.mcp.ts"
import type { Route } from './+types/api.mcp';
import { createMcpHandler, McpServer } from '@modelcontextprotocol/server';
import { registerSearchTool, registerSourceTools } from 'fumadocs-core/mcp';
import { createFromSource } from 'fumadocs-core/search/server';
import { docsLlms, source } from '@/lib/source';

const handler = createMcpHandler(() => {
  const mcp = new McpServer({
    name: 'docs',
    version: '1.0.0',
  });

  registerSourceTools(mcp, source, docsLlms);
  registerSearchTool(mcp, createFromSource(source));

  return mcp;
});

export async function loader({ request }: Route.LoaderArgs) {
  return handler.fetch(request);
}

export async function action({ request }: Route.ActionArgs) {
  return handler.fetch(request);
}

```

```ts tab="Tanstack Start" title="src/routes/api/mcp.ts"
import { createFileRoute } from '@tanstack/react-router';
import { createMcpHandler, McpServer } from '@modelcontextprotocol/server';
import { registerSearchTool, registerSourceTools } from 'fumadocs-core/mcp';
import { createFromSource } from 'fumadocs-core/search/server';
import { docsLlms, source } from '@/lib/source';

const handler = createMcpHandler(() => {
  const mcp = new McpServer({
    name: 'docs',
    version: '1.0.0',
  });

  registerSourceTools(mcp, source, docsLlms);
  registerSearchTool(mcp, createFromSource(source));

  return mcp;
});

export const Route = createFileRoute('/api/mcp')({
  server: {
    handlers: {
      GET: async ({ request }) => handler.fetch(request),
      POST: async ({ request }) => handler.fetch(request),
      DELETE: async ({ request }) => handler.fetch(request),
    },
  },
});

```

```ts tab="Waku" title="pages/_api/api/mcp.ts"
import { createMcpHandler, McpServer } from '@modelcontextprotocol/server';
import { registerSearchTool, registerSourceTools } from 'fumadocs-core/mcp';
import { createFromSource } from 'fumadocs-core/search/server';
import { docsLlms, source } from '@/lib/source';

const handler = createMcpHandler(() => {
  const mcp = new McpServer({
    name: 'docs',
    version: '1.0.0',
  });

  registerSourceTools(mcp, source, docsLlms);
  registerSearchTool(mcp, createFromSource(source));

  return mcp;
});

export async function GET(request: Request) {
  return handler.fetch(request);
}

export async function POST(request: Request) {
  return handler.fetch(request);
}

export async function DELETE(request: Request) {
  return handler.fetch(request);
}

```

Connect an agent to it with:

```json
{
  "mcpServers": {
    "docs": { "url": "https://your-site.com/api/mcp" }
  }
}
```

## WebMCP [#webmcp]

<Callout type="warn" title="Experimental">
  [WebMCP](https://webmachinelearning.github.io/webmcp/) is an early web standard, available in
  Chrome 149+ behind the `#enable-webmcp-testing` flag and an origin trial. The API may still
  change.
</Callout>

Where the MCP server above serves agents connecting to your site, WebMCP exposes tools to the AI agent
of the browser, on the page the reader is viewing.

```package-install
npx @fumadocs/cli feature webmcp
```

It creates a `WebMCP` component on top of the [LLM routes](#docs-for-llm), you can render it inside `<RootProvider />`.

- `search_docs` queries the search API route, you can use [other clients](/docs/search) than `fetchClient`.
- `read_page` fetches the Markdown of a page, `/docs/page.md` on TanStack Start.

## Ask AI [#ask-ai]

![AI Search](/docs/ai-search.png)

You can install the AI chat dialog using Fumadocs CLI.

<Tabs items={['AI SDK', 'LLMGateway', 'Inkeep AI']}>
<Tab>

```npm
npx @fumadocs/cli add ai/openrouter
```

It's automatically configured for [OpenRouter](https://openrouter.ai) using Vercel AI SDK, with a `/search` tool for AI.

You can use other models by updating the `/api/chat` route.

> Fumadocs doesn't provide the AI model, it's up to you.
>
> Your AI model can use the `llms-full.txt` file generated above, or more diversified sources of information when combined with 3rd party solutions.

</Tab>
<Tab>

```npm
npx @fumadocs/cli add ai/llmgateway
```

It's automatically configured for [LLMGateway](https://llmgateway.io) using Vercel AI SDK, with a `/search` tool for AI.

LLMGateway is an open-source API gateway that proxies requests to 300+ language models, so the same route handler can switch between providers by changing the model ID.

Add your LLMGateway API key to environment variables:

```dotenv
LLM_GATEWAY_API_KEY="..."
```

By default the route uses `anthropic/claude-3.5-sonnet`. To switch to any other model supported by LLMGateway, set the `LLM_GATEWAY_MODEL` environment variable—no code change required:

```dotenv
# any model id from https://llmgateway.io/models
LLM_GATEWAY_MODEL="openai/gpt-4o"
```

</Tab>
<Tab>

```npm
npx @fumadocs/cli add ai/inkeep
```

It's automatically configured for [Inkeep AI](https://inkeep.com) using Vercel AI SDK.

Add your Inkeep API key to environment variables:

```dotenv
INKEEP_API_KEY="..."
```

</Tab>
</Tabs>

Add the component & trigger to docs layout:

```tsx
import { DocsLayout } from 'fumadocs-ui/layouts/docs';
// import the installed components, e.g.
import { AISearch, AISearchPanel, AISearchTrigger } from '@/components/ai/search';
import { MessageCircleIcon } from 'lucide-react';
// or import your own button styles
import { buttonVariants } from 'fumadocs-ui/components/ui/button';

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <DocsLayout>
      {/* [!code ++:17] */}
      <AISearch>
        <AISearchPanel />
        <AISearchTrigger
          position="float"
          className={cn(
            buttonVariants({
              variant: 'secondary',
              className: 'text-fd-muted-foreground rounded-2xl',
            }),
          )}
        >
          <MessageCircleIcon className="size-4.5" />
          Ask AI
        </AISearchTrigger>
      </AISearch>

      {children}
    </DocsLayout>
  );
}
```
