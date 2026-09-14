# Fumadocs Core (the core library of Fumadocs): TOC

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/headless/components/toc.mdx

Table of Contents

A Table of Contents with active anchor observer and auto scroll.

## Usage [#usage]

```tsx
import * as Base from 'fumadocs-core/toc';

return (
  <Base.AnchorProvider>
    <Base.ScrollProvider>
      <Base.TOCItem />
      <Base.TOCItem />
    </Base.ScrollProvider>
  </Base.AnchorProvider>
);
```

### Anchor Provider [#anchor-provider]

Watches for the active anchor using the Intersection Observer API.

### AnchorProviderProps

| Prop        | Type        | Description                                          |
| ----------- | ----------- | ---------------------------------------------------- |
| `toc`       | `array`     |                                                      |
| `single?`   | `union`     | Only accept one active item at most Default: `false` |
| `children?` | `ReactNode` |                                                      |


### Scroll Provider [#scroll-provider]

Scrolls the scroll container to the active anchor.

### ScrollProviderProps

| Prop           | Type        | Description                                   |
| -------------- | ----------- | --------------------------------------------- |
| `containerRef` | `object`    | Scroll into the view of container when active |
| `children?`    | `ReactNode` |                                               |


### TOC Item [#toc-item]

An anchor item for jumping to the target anchor.

| Data Attribute | Values        | Description          |
| -------------- | ------------- | -------------------- |
| `data-active`  | `true, false` | Is the anchor active |

## Example [#example]

```tsx
import { AnchorProvider, ScrollProvider, TOCItem, type TOCItemType } from 'fumadocs-core/toc';
import { type ReactNode, useRef } from 'react';

export function Page({ items, children }: { items: TOCItemType[]; children: ReactNode }) {
  const viewRef = useRef<HTMLDivElement>(null);

  return (
    <AnchorProvider toc={items}>
      <div ref={viewRef} className="overflow-auto">
        <ScrollProvider containerRef={viewRef}>
          {items.map((item) => (
            <TOCItem key={item.url} href={item.url}>
              {item.title}
            </TOCItem>
          ))}
        </ScrollProvider>
      </div>
      {children}
    </AnchorProvider>
  );
}

```
