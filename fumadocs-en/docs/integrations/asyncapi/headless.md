# Fumadocs (Framework Mode): Headless

Source: https://raw.githubusercontent.com/fuma-nama/fumadocs/refs/heads/main/apps/docs/content/docs/(framework)/integrations/asyncapi/headless.mdx

The state and logic of AsyncAPI pages, without UI.

## Overview [#overview]

Everything outside `@fumadocs/asyncapi/ui` is headless.

| Path                           | Module                                                    |
| ------------------------------ | --------------------------------------------------------- |
| `@fumadocs/asyncapi`           | the page: the document, its servers and your components   |
| `@fumadocs/asyncapi/operation` | an operation: its channel, parameters, messages and reply |

## Page [#page]

`createAsyncAPIRenderer()` takes your components and returns `<AsyncAPIPage />`, which accepts the props of generated pages.

```tsx title="components/api-page.tsx"
'use client';
import { type CodeBlockProps, createAsyncAPIRenderer } from '@fumadocs/asyncapi';
import { Operation } from '@/components/my-operation';
import { SchemaUI } from '@/components/my-schema';

export const AsyncAPIPage = createAsyncAPIRenderer({
  components: {
    Operation,
    SchemaUI,
    CodeBlock({ lang, code }: CodeBlockProps) {
      return (
        <pre>
          <code className={`language-${lang}`}>{code}</code>
        </pre>
      );
    },
  },
});
```

| Component   | Renders                                                      |
| ----------- | ------------------------------------------------------------ |
| `Operation` | each operation of the page, see [Operation](#operation)      |
| `SchemaUI`  | the JSON schemas of parameters, headers and message payloads |
| `Layout`    | optional, wraps the rendered operations                      |

### Hooks [#hooks]

Components under the page read its state.

| Hook                 | Returns                                            |
| -------------------- | -------------------------------------------------- |
| `useAsyncAPI()`      | the dereferenced document (`doc`) and page options |
| `useComponents()`    | the components passed to the page                  |
| `useServer()`        | the servers, the selected one and its variables    |
| `useRenderContext()` | the render options of the page, like `content`     |

To render operations yourself, mount `<AsyncAPIProvider document={bundled} shiki={shiki} components={...} />` in place of `createAsyncAPIRenderer()`.

## Operation [#operation]

`<OperationProvider />` derives the details of an operation once: traits are applied, and its channel, parameters, messages and reply are resolved.

```tsx title="components/my-operation.tsx"
'use client';
import {
  OperationProvider,
  type PageOperationProps,
  useOperation,
  useOperationSecurity,
} from '@fumadocs/asyncapi/operation';

export function Operation(props: PageOperationProps) {
  return (
    <OperationProvider {...props}>
      <Content />
    </OperationProvider>
  );
}

function Content() {
  const { title, channel, parameters, messages } = useOperation();
  const schemes = useOperationSecurity();

  return (
    <>
      <h2>{title}</h2>
      <code>{channel.address}</code>
      <ul>
        {parameters.map((param) => (
          <li key={param.name}>{param.name}</li>
        ))}
      </ul>
      {messages.map((message) => (
        <section key={message.id}>
          <h3>{message.name}</h3>
          {message.examples.map((example) => (
            <pre key={example.id}>{JSON.stringify(example.payload, null, 2)}</pre>
          ))}
        </section>
      ))}
      <p>{schemes.map((scheme) => scheme.type).join(', ')}</p>
    </>
  );
}
```

| Hook                     | Returns                                                                  |
| ------------------------ | ------------------------------------------------------------------------ |
| `useOperation()`         | the operation with its details resolved                                  |
| `useOperationSecurity()` | security schemes of the operation, falling back to the selected server's |

### OperationInfo

| Prop           | Type     | Description                   |
| -------------- | -------- | ----------------------------- |
| `id`           | `string` |                               |
| `action`       | `union`  |                               |
| `operation`    | `object` | resolved, with traits applied |
| `channel`      | `object` |                               |
| `title`        | `string` |                               |
| `description?` | `string` |                               |
| `parameters`   | `array`  |                               |
| `messages`     | `array`  |                               |
| `reply?`       | `object` |                               |


Examples of a message come from the document, or are generated from its schemas.

### OperationMessage

| Prop       | Type     | Description                         |
| ---------- | -------- | ----------------------------------- |
| `id`       | `string` | id for anchors and accordion values |
| `name`     | `string` | display name                        |
| `message`  | `object` | resolved, with traits applied       |
| `headers?` | `union`  |                                     |
| `payload?` | `union`  |                                     |
| `examples` | `array`  | example messages                    |
