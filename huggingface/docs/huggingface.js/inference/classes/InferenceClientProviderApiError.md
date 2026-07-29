# Class: InferenceClientProviderApiError

Thrown when the HTTP request to the provider fails, e.g. due to API issues or server errors.

## Hierarchy

- `InferenceClientHttpRequestError`

  ↳ **`InferenceClientProviderApiError`**

## Constructors

### constructor

• **new InferenceClientProviderApiError**(`message`, `httpRequest`, `httpResponse`): [`InferenceClientProviderApiError`](InferenceClientProviderApiError)

#### Parameters[[constructor.parameters]]

| Name | Type |
| :------ | :------ |
| `message` | `string` |
| `httpRequest` | `HttpRequest` |
| `httpResponse` | `HttpResponse` |

#### Returns[[constructor.returns]]

[`InferenceClientProviderApiError`](InferenceClientProviderApiError)

#### Overrides[[constructor.overrides]]

InferenceClientHttpRequestError.constructor

#### Defined in[[constructor.defined-in]]

[inference/src/errors.ts:65](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/errors.ts#L65)

## Properties

### cause

• `Optional` **cause**: `unknown`

#### Inherited from[[cause.inherited-from]]

InferenceClientHttpRequestError.cause

#### Defined in[[cause.defined-in]]

doc-internal/node_modules/.pnpm/typescript@5.8.3/node_modules/typescript/lib/lib.es2022.error.d.ts:26

___

### httpRequest

• **httpRequest**: `HttpRequest`

#### Inherited from[[httprequest.inherited-from]]

InferenceClientHttpRequestError.httpRequest

#### Defined in[[httprequest.defined-in]]

[inference/src/errors.ts:41](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/errors.ts#L41)

___

### httpResponse

• **httpResponse**: `HttpResponse`

#### Inherited from[[httpresponse.inherited-from]]

InferenceClientHttpRequestError.httpResponse

#### Defined in[[httpresponse.defined-in]]

[inference/src/errors.ts:42](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/errors.ts#L42)

___

### message

• **message**: `string`

#### Inherited from[[message.inherited-from]]

InferenceClientHttpRequestError.message

#### Defined in[[message.defined-in]]

doc-internal/node_modules/.pnpm/typescript@5.8.3/node_modules/typescript/lib/lib.es5.d.ts:1077

___

### name

• **name**: `string`

#### Inherited from[[name.inherited-from]]

InferenceClientHttpRequestError.name

#### Defined in[[name.defined-in]]

doc-internal/node_modules/.pnpm/typescript@5.8.3/node_modules/typescript/lib/lib.es5.d.ts:1076

___

### stack

• `Optional` **stack**: `string`

#### Inherited from[[stack.inherited-from]]

InferenceClientHttpRequestError.stack

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

InferenceClientHttpRequestError.prepareStackTrace

#### Defined in[[preparestacktrace.defined-in]]

inference/node_modules/.pnpm/@types+node@18.13.0/node_modules/@types/node/globals.d.ts:11

___

### stackTraceLimit

▪ `Static` **stackTraceLimit**: `number`

#### Inherited from[[stacktracelimit.inherited-from]]

InferenceClientHttpRequestError.stackTraceLimit

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

InferenceClientHttpRequestError.captureStackTrace

#### Defined in[[capturestacktrace.defined-in]]

inference/node_modules/.pnpm/@types+node@18.13.0/node_modules/@types/node/globals.d.ts:4
