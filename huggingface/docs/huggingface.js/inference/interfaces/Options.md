# Interface: Options

## Properties

### billTo

• `Optional` **billTo**: `string`

The billing account to use for the requests.

By default the requests are billed on the user's account.
Requests can only be billed to an organization the user is a member of, and which has subscribed to Enterprise Hub.

#### Defined in[[billto.defined-in]]

[inference/src/types.ts:42](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L42)

___

### fetch

• `Optional` **fetch**: (`input`: `RequestInfo` \| `URL`, `init?`: `RequestInit`) => `Promise`\<`Response`\>

Custom fetch function to use instead of the default one, for example to use a proxy or edit headers.

#### Type declaration[[fetch.type-declaration]]

▸ (`input`, `init?`): `Promise`\<`Response`\>

##### Parameters[[fetch.parameters]]

| Name | Type |
| :------ | :------ |
| `input` | `RequestInfo` \| `URL` |
| `init?` | `RequestInit` |

##### Returns[[fetch.returns]]

`Promise`\<`Response`\>

#### Defined in[[fetch.defined-in]]

[inference/src/types.ts:25](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L25)

___

### includeCredentials

• `Optional` **includeCredentials**: `string` \| `boolean`

(Default: "same-origin"). String | Boolean. Credentials to use for the request. If this is a string, it will be passed straight on. If it's a boolean, true will be "include" and false will not send credentials at all.

#### Defined in[[includecredentials.defined-in]]

[inference/src/types.ts:34](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L34)

___

### retry\_on\_error

• `Optional` **retry\_on\_error**: `boolean`

(Default: true) Boolean. If a request 503s, the request will be retried with the same parameters.

#### Defined in[[retryonerror.defined-in]]

[inference/src/types.ts:20](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L20)

___

### signal

• `Optional` **signal**: `AbortSignal`

Abort Controller signal to use for request interruption.

#### Defined in[[signal.defined-in]]

[inference/src/types.ts:29](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L29)
