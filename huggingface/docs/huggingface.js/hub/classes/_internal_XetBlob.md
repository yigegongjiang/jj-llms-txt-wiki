# Class: \_\_internal\_XetBlob

XetBlob is a blob implementation that fetches data directly from the Xet storage

## Hierarchy

- `Blob`

  ↳ **`__internal_XetBlob`**

## Constructors

### constructor

• **new __internal_XetBlob**(`params`): [`__internal_XetBlob`](_internal_XetBlob)

#### Parameters[[constructor.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | `XetBlobCreateOptions` |

#### Returns[[constructor.returns]]

[`__internal_XetBlob`](_internal_XetBlob)

#### Overrides[[constructor.overrides]]

Blob.constructor

#### Defined in[[constructor.defined-in]]

[packages/hub/src/utils/XetBlob.ts:106](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L106)

## Properties

### #reconstructionInfoPromise

• `Private` `Optional` **#reconstructionInfoPromise**: `Promise`\<`ReconstructionInfo`\>

#### Defined in[[reconstructioninfopromise.defined-in]]

[packages/hub/src/utils/XetBlob.ts:169](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L169)

___

### accessToken

• `Optional` **accessToken**: `string`

#### Defined in[[accesstoken.defined-in]]

[packages/hub/src/utils/XetBlob.ts:96](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L96)

___

### end

• **end**: `number` = `0`

#### Defined in[[end.defined-in]]

[packages/hub/src/utils/XetBlob.ts:101](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L101)

___

### fetch

• **fetch**: (`input`: `URL` \| `RequestInfo`, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\>

#### Type declaration[[fetch.type-declaration]]

▸ (`input`, `init?`): `Promise`\<`Response`\>

[MDN Reference](https://developer.mozilla.org/docs/Web/API/Window/fetch)

##### Parameters[[fetch.parameters]]

| Name | Type |
| :------ | :------ |
| `input` | `URL` \| `RequestInfo` |
| `init?` | `RequestInit` |

##### Returns[[fetch.returns]]

`Promise`\<`Response`\>

▸ (`input`, `init?`): `Promise`\<`Response`\>

##### Parameters[[fetch.parameters]]

| Name | Type |
| :------ | :------ |
| `input` | `string` \| `URL` \| `Request` |
| `init?` | `RequestInit` |

##### Returns[[fetch.returns]]

`Promise`\<`Response`\>

#### Defined in[[fetch.defined-in]]

[packages/hub/src/utils/XetBlob.ts:95](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L95)

___

### hash

• `Optional` **hash**: `string`

#### Defined in[[hash.defined-in]]

[packages/hub/src/utils/XetBlob.ts:99](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L99)

___

### internalLogging

• **internalLogging**: `boolean` = `false`

#### Defined in[[internallogging.defined-in]]

[packages/hub/src/utils/XetBlob.ts:102](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L102)

___

### listener

• **listener**: `undefined` \| (`arg`: \{ `event`: ``"read"``  } \| \{ `event`: ``"progress"`` ; `progress`: \{ `read`: `number` ; `total`: `number`  }  }) => `void`

#### Defined in[[listener.defined-in]]

[packages/hub/src/utils/XetBlob.ts:104](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L104)

___

### reconstructionInfo

• **reconstructionInfo**: `undefined` \| `ReconstructionInfo`

#### Defined in[[reconstructioninfo.defined-in]]

[packages/hub/src/utils/XetBlob.ts:103](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L103)

___

### reconstructionUrl

• `Optional` **reconstructionUrl**: `string`

#### Defined in[[reconstructionurl.defined-in]]

[packages/hub/src/utils/XetBlob.ts:98](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L98)

___

### refreshUrl

• **refreshUrl**: `string`

#### Defined in[[refreshurl.defined-in]]

[packages/hub/src/utils/XetBlob.ts:97](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L97)

___

### start

• **start**: `number` = `0`

#### Defined in[[start.defined-in]]

[packages/hub/src/utils/XetBlob.ts:100](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L100)

___

### type

• `Readonly` **type**: `string`

[MDN Reference](https://developer.mozilla.org/docs/Web/API/Blob/type)

#### Inherited from[[type.inherited-from]]

Blob.type

#### Defined in[[type.defined-in]]

packages/doc-internal/node_modules/.pnpm/typescript@5.8.3/node_modules/typescript/lib/lib.dom.d.ts:3501

## Accessors

### size

• `get` **size**(): `number`

#### Returns[[size.returns]]

`number`

#### Overrides[[size.overrides]]

Blob.size

#### Defined in[[size.defined-in]]

[packages/hub/src/utils/XetBlob.ts:128](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L128)

## Methods

### #clone

▸ **#clone**(): [`__internal_XetBlob`](_internal_XetBlob)

#### Returns[[clone.returns]]

[`__internal_XetBlob`](_internal_XetBlob)

#### Defined in[[clone.defined-in]]

[packages/hub/src/utils/XetBlob.ts:132](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L132)

___

### #fetch

▸ **#fetch**(): `Promise`\<`ReadableStream`\<`Uint8Array`\<`ArrayBufferLike`\>\>\>

#### Returns[[fetch.returns]]

`Promise`\<`ReadableStream`\<`Uint8Array`\<`ArrayBufferLike`\>\>\>

#### Defined in[[fetch.defined-in]]

[packages/hub/src/utils/XetBlob.ts:202](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L202)

___

### #loadReconstructionInfo

▸ **#loadReconstructionInfo**(): `Promise`\<`ReconstructionInfo`\>

#### Returns[[loadreconstructioninfo.returns]]

`Promise`\<`ReconstructionInfo`\>

#### Defined in[[loadreconstructioninfo.defined-in]]

[packages/hub/src/utils/XetBlob.ts:171](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L171)

___

### arrayBuffer

▸ **arrayBuffer**(): `Promise`\<`ArrayBuffer`\>

#### Returns[[arraybuffer.returns]]

`Promise`\<`ArrayBuffer`\>

#### Overrides[[arraybuffer.overrides]]

Blob.arrayBuffer

#### Defined in[[arraybuffer.defined-in]]

[packages/hub/src/utils/XetBlob.ts:520](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L520)

___

### bytes

▸ **bytes**(): `Promise`\<`Uint8Array`\<`ArrayBufferLike`\>\>

[MDN Reference](https://developer.mozilla.org/docs/Web/API/Blob/bytes)

#### Returns[[bytes.returns]]

`Promise`\<`Uint8Array`\<`ArrayBufferLike`\>\>

#### Inherited from[[bytes.inherited-from]]

Blob.bytes

#### Defined in[[bytes.defined-in]]

packages/doc-internal/node_modules/.pnpm/typescript@5.8.3/node_modules/typescript/lib/lib.dom.d.ts:3505

___

### response

▸ **response**(): `Promise`\<`Response`\>

#### Returns[[response.returns]]

`Promise`\<`Response`\>

#### Defined in[[response.defined-in]]

[packages/hub/src/utils/XetBlob.ts:532](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L532)

___

### slice

▸ **slice**(`start?`, `end?`): [`__internal_XetBlob`](_internal_XetBlob)

#### Parameters[[slice.parameters]]

| Name | Type | Default value |
| :------ | :------ | :------ |
| `start` | `number` | `0` |
| `end` | `number` | `undefined` |

#### Returns[[slice.returns]]

[`__internal_XetBlob`](_internal_XetBlob)

#### Overrides[[slice.overrides]]

Blob.slice

#### Defined in[[slice.defined-in]]

[packages/hub/src/utils/XetBlob.ts:152](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L152)

___

### stream

▸ **stream**(): `ReadableStream`\<`Uint8Array`\<`ArrayBufferLike`\>\>

#### Returns[[stream.returns]]

`ReadableStream`\<`Uint8Array`\<`ArrayBufferLike`\>\>

#### Overrides[[stream.overrides]]

Blob.stream

#### Defined in[[stream.defined-in]]

[packages/hub/src/utils/XetBlob.ts:538](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L538)

___

### text

▸ **text**(): `Promise`\<`string`\>

#### Returns[[text.returns]]

`Promise`\<`string`\>

#### Overrides[[text.overrides]]

Blob.text

#### Defined in[[text.defined-in]]

[packages/hub/src/utils/XetBlob.ts:526](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L526)
