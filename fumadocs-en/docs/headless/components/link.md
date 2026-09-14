# Fumadocs Core (the core library of Fumadocs): Link

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/headless/components/link.mdx

A Link component that handles external links

A component that wraps the Link component of your React framework (e.g. `next/link`) and automatically handles external links in the document.~
When an external URL is detected, it uses `<a>` instead of the Link Component.

The `rel` property is automatically generated.

## Usage [#usage]

Usage is the same as using `<a>`.

```mdx
import Link from 'fumadocs-core/link';

<Link href="/docs/components">Click Me</Link>
```

### External [#external]

You can force a URL to be external by passing an `external` prop.

### Dynamic hrefs [#dynamic-hrefs]

You can enable dynamic hrefs by importing `dynamic-link`.

```mdx
import { DynamicLink } from 'fumadocs-core/dynamic-link';

<DynamicLink href="/[lang]/components">Click Me</DynamicLink>
```
