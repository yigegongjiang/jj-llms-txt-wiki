# Interface: UrlParams

## Properties

### authMethod

• **authMethod**: [`AuthMethod`](../modules#authmethod)

#### Defined in[[authmethod.defined-in]]

[inference/src/types.ts:171](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L171)

___

### model

• **model**: `string`

#### Defined in[[model.defined-in]]

[inference/src/types.ts:172](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L172)

___

### task

• `Optional` **task**: [`InferenceTask`](../modules#inferencetask)

#### Defined in[[task.defined-in]]

[inference/src/types.ts:173](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L173)

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

[inference/src/types.ts:174](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L174)
