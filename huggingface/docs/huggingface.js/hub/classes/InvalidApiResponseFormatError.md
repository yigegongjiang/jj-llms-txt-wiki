# Class: InvalidApiResponseFormatError

## Hierarchy

- `Error`

  ↳ **`InvalidApiResponseFormatError`**

## Constructors

### constructor

• **new InvalidApiResponseFormatError**(`message?`): [`InvalidApiResponseFormatError`](InvalidApiResponseFormatError)

#### Parameters[[constructor.parameters]]

| Name | Type |
| :------ | :------ |
| `message?` | `string` |

#### Returns[[constructor.returns]]

[`InvalidApiResponseFormatError`](InvalidApiResponseFormatError)

#### Inherited from[[constructor.inherited-from]]

Error.constructor

#### Defined in[[constructor.defined-in]]

packages/doc-internal/node_modules/.pnpm/typescript@5.8.3/node_modules/typescript/lib/lib.es5.d.ts:1082

• **new InvalidApiResponseFormatError**(`message?`, `options?`): [`InvalidApiResponseFormatError`](InvalidApiResponseFormatError)

#### Parameters[[constructor.parameters]]

| Name | Type |
| :------ | :------ |
| `message?` | `string` |
| `options?` | `ErrorOptions` |

#### Returns[[constructor.returns]]

[`InvalidApiResponseFormatError`](InvalidApiResponseFormatError)

#### Inherited from[[constructor.inherited-from]]

Error.constructor

#### Defined in[[constructor.defined-in]]

packages/doc-internal/node_modules/.pnpm/typescript@5.8.3/node_modules/typescript/lib/lib.es5.d.ts:1082

## Properties

### cause

• `Optional` **cause**: `unknown`

#### Inherited from[[cause.inherited-from]]

Error.cause

#### Defined in[[cause.defined-in]]

packages/doc-internal/node_modules/.pnpm/typescript@5.8.3/node_modules/typescript/lib/lib.es2022.error.d.ts:26

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

### stack

• `Optional` **stack**: `string`

#### Inherited from[[stack.inherited-from]]

Error.stack

#### Defined in[[stack.defined-in]]

packages/doc-internal/node_modules/.pnpm/typescript@5.8.3/node_modules/typescript/lib/lib.es5.d.ts:1078

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
