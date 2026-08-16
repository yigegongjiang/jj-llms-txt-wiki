# Interface: SafetensorsIndexJson

## Properties

### dtype

• `Optional` **dtype**: `string`

#### Defined in[[dtype.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:284](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L284)

___

### metadata

• `Optional` **metadata**: \{ `total_parameters?`: `string` \| `number`  } & `Record`\<`string`, `string`\>

#### Defined in[[metadata.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:286](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L286)

___

### weight\_map

• `Optional` **weight\_map**: `Record`\<`string`, `string`\>

Mapping of tensor name -> shard filename.

Omitted when the index holds more than `MAX_WEIGHT_MAP_ENTRIES` tensors: such indexes are
consumed in streaming mode and the map is never materialized, so that memory stays bounded
(see `parse-safetensors-index.ts`). Every model below that threshold — i.e. all of them bar a
couple of huge MoEs, which previously failed to parse outright — is unaffected.

If you only need the shard list, use `filepaths` on the parse result instead.

#### Defined in[[weightmap.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:298](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L298)
