# Class: InferenceClientRoutingError

Base class for all inference-related errors.

## Hierarchy

- [`InferenceClientError`](InferenceClientError)

  ↳ **`InferenceClientRoutingError`**

## Constructors

### constructor

• **new InferenceClientRoutingError**(`message`): [`InferenceClientRoutingError`](InferenceClientRoutingError)

#### Parameters[[constructor.parameters]]

| Name | Type |
| :------ | :------ |
| `message` | `string` |

#### Returns[[constructor.returns]]

[`InferenceClientRoutingError`](InferenceClientRoutingError)

#### Overrides[[constructor.overrides]]

[InferenceClientError](InferenceClientError).[constructor](InferenceClientError#constructor)

#### Defined in[[constructor.defined-in]]

[inference/src/errors.ts:21](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/errors.ts#L21)

## Properties

### cause

• `Optional` **cause**: `unknown`

#### Inherited from[[cause.inherited-from]]

[InferenceClientError](InferenceClientError).[cause](InferenceClientError#cause)

#### Defined in[[cause.defined-in]]

doc-internal/node_modules/.pnpm/typescript@5.8.3/node_modules/typescript/lib/lib.es2022.error.d.ts:26

___

### message

• **message**: `string`

#### Inherited from[[message.inherited-from]]

[InferenceClientError](InferenceClientError).[message](InferenceClientError#message)

#### Defined in[[message.defined-in]]

doc-internal/node_modules/.pnpm/typescript@5.8.3/node_modules/typescript/lib/lib.es5.d.ts:1077

___

### name

• **name**: `string`

#### Inherited from[[name.inherited-from]]

[InferenceClientError](InferenceClientError).[name](InferenceClientError#name)

#### Defined in[[name.defined-in]]

doc-internal/node_modules/.pnpm/typescript@5.8.3/node_modules/typescript/lib/lib.es5.d.ts:1076

___

### stack

• `Optional` **stack**: `string`

#### Inherited from[[stack.inherited-from]]

[InferenceClientError](InferenceClientError).[stack](InferenceClientError#stack)

#### Defined in[[stack.defined-in]]

doc-internal/node_modules/.pnpm/typescript@5.8.3/node_modules/typescript/lib/lib.es5.d.ts:1078

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

[InferenceClientError](InferenceClientError).[prepareStackTrace](InferenceClientError#preparestacktrace)

#### Defined in[[preparestacktrace.defined-in]]

inference/node_modules/.pnpm/@types+node@18.13.0/node_modules/@types/node/globals.d.ts:11

___

### stackTraceLimit

▪ `Static` **stackTraceLimit**: `number`

#### Inherited from[[stacktracelimit.inherited-from]]

[InferenceClientError](InferenceClientError).[stackTraceLimit](InferenceClientError#stacktracelimit)

#### Defined in[[stacktracelimit.defined-in]]

inference/node_modules/.pnpm/@types+node@18.13.0/node_modules/@types/node/globals.d.ts:13

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

[InferenceClientError](InferenceClientError).[captureStackTrace](InferenceClientError#capturestacktrace)

#### Defined in[[capturestacktrace.defined-in]]

inference/node_modules/.pnpm/@types+node@18.13.0/node_modules/@types/node/globals.d.ts:4
