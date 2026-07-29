# Interface: UrlParams

## Properties

### authMethod

• **authMethod**: [`AuthMethod`](../modules#authmethod)

#### Defined in[[authmethod.defined-in]]

[inference/src/types.ts:169](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L169)

___

### model

• **model**: `string`

#### Defined in[[model.defined-in]]

[inference/src/types.ts:170](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L170)

___

### task

• `Optional` **task**: [`InferenceTask`](../modules#inferencetask)

#### Defined in[[task.defined-in]]

[inference/src/types.ts:171](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L171)

___

### urlTransform

• `Optional` **urlTransform**: (`url`: `string`) => `string`

#### Type declaration[[urltransform.type-declaration]]

▸ (`url`): `string`

##### Parameters[[urltransform.parameters]]

| Name | Type |
| :------ | :------ |
| `url` | `string` |

##### Returns[[urltransform.returns]]

`string`

#### Defined in[[urltransform.defined-in]]

[inference/src/types.ts:172](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L172)
