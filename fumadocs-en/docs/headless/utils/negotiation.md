# Fumadocs Core (the core library of Fumadocs): Negotiation

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/headless/utils/negotiation.mdx

Content negotiation utilities.

## Overview [#overview]

For React.js frameworks supporting middlewares, you can use the Negotiation API of Fumadocs for basic functionalities.

### Accept Markdown [#accept-markdown]

Serve markdown or HTML depending on the `Accept` header, this can improve the experience for AI agents.

```ts title="proxy.ts (Next.js)"
import { NextRequest, NextResponse } from 'next/server';
import { isMarkdownPreferred, rewritePath } from 'fumadocs-core/negotiation';

const { rewrite: rewriteLLM } = rewritePath('/docs/*path', '/llms.mdx/*path');

export default function proxy(request: NextRequest) {
  if (isMarkdownPreferred(request)) {
    const result = rewriteLLM(request.nextUrl.pathname);

    if (result) {
      return NextResponse.rewrite(new URL(result, request.nextUrl));
    }
  }

  return NextResponse.next();
}
```
