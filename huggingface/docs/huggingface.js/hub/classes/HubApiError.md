# Class: HubApiError

Error thrown when an API call to the Hugging Face Hub fails.

## Hierarchy

- `Error`

  ↳ **`HubApiError`**

## Constructors

### constructor

• **new HubApiError**(`url`, `statusCode`, `requestId?`, `message?`): [`HubApiError`](HubApiError)

#### Parameters[[constructor.parameters]]

| Name | Type |
| :------ | :------ |
| `url` | `string` |
| `statusCode` | `number` |
| `requestId?` | `string` |
| `message?` | `string` |

#### Returns[[constructor.returns]]

[`HubApiError`](HubApiError)

#### Overrides[[constructor.overrides]]

Error.constructor

#### Defined in[[constructor.defined-in]]

[packages/hub/src/error.ts:40](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/error.ts#L40)

## Properties

### cause

• `Optional` **cause**: `unknown`

#### Inherited from[[cause.inherited-from]]

Error.cause

#### Defined in[[cause.defined-in]]

packages/doc-internal/node_modules/.pnpm/typescript@5.8.3/node_modules/typescript/lib/lib.es2022.error.d.ts:26

___

### data

• `Optional` **data**: `JsonObject`

#### Defined in[[data.defined-in]]

[packages/hub/src/error.ts:38](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/error.ts#L38)

___

### message

• **message**: `string`

#### Inherited from[[message.inherited-from]]

Error.message

#### Defined in[[message.defined-in]]

packages/doc-internal/node_modules/.pnpm/typescript@5.8.3/node_modules/typescript/lib/lib.es5.d.ts:1077

___

### name

• **name**: `string`

#### Inherited from[[name.inherited-from]]

Error.name

#### Defined in[[name.defined-in]]

packages/doc-internal/node_modules/.pnpm/typescript@5.8.3/node_modules/typescript/lib/lib.es5.d.ts:1076

___

### requestId

• `Optional` **requestId**: `string`

#### Defined in[[requestid.defined-in]]

[packages/hub/src/error.ts:37](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/error.ts#L37)

___

### stack

• `Optional` **stack**: `string`

#### Inherited from[[stack.inherited-from]]

Error.stack

#### Defined in[[stack.defined-in]]

packages/doc-internal/node_modules/.pnpm/typescript@5.8.3/node_modules/typescript/lib/lib.es5.d.ts:1078

___

### statusCode

• **statusCode**: `number`

#### Defined in[[statuscode.defined-in]]

[packages/hub/src/error.ts:35](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/error.ts#L35)

___

### url

• **url**: `string`

#### Defined in[[url.defined-in]]

[packages/hub/src/error.ts:36](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/error.ts#L36)

___

### prepareStackTrace

▪ `Static` `Optional` **prepareStackTrace**: (`err`: `Error`, `stackTraces`: `CallSite`[]) => `any`

Optional override for formatting stack traces

**`See`**

https://v8.dev/docs/stack-trace-api#customizing-stack-traces

#### Type declaration[[preparestacktrace.type-declaration]]

▸ (`err`, `stackTraces`): `any`

##### Parameters[[preparestacktrace.parameters]]

| Name | Type |
| :------ | :------ |
| `err` | `Error` |
| `stackTraces` | `CallSite`[] |

##### Returns[[preparestacktrace.returns]]

`any`

#### Inherited from[[preparestacktrace.inherited-from]]

Error.prepareStackTrace

#### Defined in[[preparestacktrace.defined-in]]

node_modules/.pnpm/@types+node@22.14.1/node_modules/@types/node/globals.d.ts:143

___

### stackTraceLimit

▪ `Static` **stackTraceLimit**: `number`

#### Inherited from[[stacktracelimit.inherited-from]]

Error.stackTraceLimit

#### Defined in[[stacktracelimit.defined-in]]

node_modules/.pnpm/@types+node@22.14.1/node_modules/@types/node/globals.d.ts:145

## Methods

### captureStackTrace

▸ **captureStackTrace**(`targetObject`, `constructorOpt?`): `void`

Create .stack property on a target object

#### Parameters[[capturestacktrace.parameters]]

| Name | Type |
| :------ | :------ |
| `targetObject` | `object` |
| `constructorOpt?` | `Function` |

#### Returns[[capturestacktrace.returns]]

`void`

#### Inherited from[[capturestacktrace.inherited-from]]

Error.captureStackTrace

#### Defined in[[capturestacktrace.defined-in]]

node_modules/.pnpm/@types+node@22.14.1/node_modules/@types/node/globals.d.ts:136
