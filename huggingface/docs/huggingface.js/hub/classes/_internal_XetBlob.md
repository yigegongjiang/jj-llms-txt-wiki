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

[packages/hub/src/utils/XetBlob.ts:312](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L312)

## Properties

### #reconstructionInfoPromise

• `Private` `Optional` **#reconstructionInfoPromise**: `Promise`\<`ReconstructionInfo`\>

#### Defined in[[reconstructioninfopromise.defined-in]]

[packages/hub/src/utils/XetBlob.ts:377](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L377)

___

### accessToken

• `Optional` **accessToken**: `string`

#### Defined in[[accesstoken.defined-in]]

[packages/hub/src/utils/XetBlob.ts:301](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L301)

___

### end

• **end**: `number` = `0`

#### Defined in[[end.defined-in]]

[packages/hub/src/utils/XetBlob.ts:306](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L306)

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

[packages/hub/src/utils/XetBlob.ts:300](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L300)

___

### hash

• `Optional` **hash**: `string`

#### Defined in[[hash.defined-in]]

[packages/hub/src/utils/XetBlob.ts:304](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L304)

___

### internalLogging

• **internalLogging**: `boolean` = `false`

#### Defined in[[internallogging.defined-in]]

[packages/hub/src/utils/XetBlob.ts:307](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L307)

___

### listener

• **listener**: `undefined` \| (`arg`: \{ `event`: ``"read"``  } \| \{ `event`: ``"progress"`` ; `progress`: \{ `read`: `number` ; `total`: `number`  }  }) => `void`

#### Defined in[[listener.defined-in]]

[packages/hub/src/utils/XetBlob.ts:309](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L309)

___

### parallelDownloads

• `Optional` **parallelDownloads**: `boolean` \| [`ParallelDownloadOptions`](../interfaces/ParallelDownloadOptions)

#### Defined in[[paralleldownloads.defined-in]]

[packages/hub/src/utils/XetBlob.ts:310](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L310)

___

### reconstructionInfo

• **reconstructionInfo**: `undefined` \| `ReconstructionInfo`

#### Defined in[[reconstructioninfo.defined-in]]

[packages/hub/src/utils/XetBlob.ts:308](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L308)

___

### reconstructionUrl

• `Optional` **reconstructionUrl**: `string`

#### Defined in[[reconstructionurl.defined-in]]

[packages/hub/src/utils/XetBlob.ts:303](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L303)

___

### refreshUrl

• **refreshUrl**: `string`

#### Defined in[[refreshurl.defined-in]]

[packages/hub/src/utils/XetBlob.ts:302](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L302)

___

### start

• **start**: `number` = `0`

#### Defined in[[start.defined-in]]

[packages/hub/src/utils/XetBlob.ts:305](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L305)

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

[packages/hub/src/utils/XetBlob.ts:335](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L335)

## Methods

### #clone

▸ **#clone**(): [`__internal_XetBlob`](_internal_XetBlob)

#### Returns[[clone.returns]]

[`__internal_XetBlob`](_internal_XetBlob)

#### Defined in[[clone.defined-in]]

[packages/hub/src/utils/XetBlob.ts:339](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L339)

___

### #fetch

▸ **#fetch**(): `Promise`\<`ReadableStream`\<`Uint8Array`\<`ArrayBufferLike`\>\>\>

#### Returns[[fetch.returns]]

`Promise`\<`ReadableStream`\<`Uint8Array`\<`ArrayBufferLike`\>\>\>

#### Defined in[[fetch.defined-in]]

[packages/hub/src/utils/XetBlob.ts:414](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L414)

___

### #loadReconstructionInfo

▸ **#loadReconstructionInfo**(): `Promise`\<`ReconstructionInfo`\>

#### Returns[[loadreconstructioninfo.returns]]

`Promise`\<`ReconstructionInfo`\>

#### Defined in[[loadreconstructioninfo.defined-in]]

[packages/hub/src/utils/XetBlob.ts:379](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L379)

___

### arrayBuffer

▸ **arrayBuffer**(): `Promise`\<`ArrayBuffer`\>

#### Returns[[arraybuffer.returns]]

`Promise`\<`ArrayBuffer`\>

#### Overrides[[arraybuffer.overrides]]

Blob.arrayBuffer

#### Defined in[[arraybuffer.defined-in]]

[packages/hub/src/utils/XetBlob.ts:1321](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L1321)

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

[packages/hub/src/utils/XetBlob.ts:1341](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L1341)

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

[packages/hub/src/utils/XetBlob.ts:360](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L360)

___

### stream

▸ **stream**(): `ReadableStream`\<`Uint8Array`\<`ArrayBufferLike`\>\>

#### Returns[[stream.returns]]

`ReadableStream`\<`Uint8Array`\<`ArrayBufferLike`\>\>

#### Overrides[[stream.overrides]]

Blob.stream

#### Defined in[[stream.defined-in]]

[packages/hub/src/utils/XetBlob.ts:1347](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L1347)

___

### text

▸ **text**(): `Promise`\<`string`\>

#### Returns[[text.returns]]

`Promise`\<`string`\>

#### Overrides[[text.overrides]]

Blob.text

#### Defined in[[text.defined-in]]

[packages/hub/src/utils/XetBlob.ts:1337](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L1337)
