# Interface: QuantizationConfig

## Properties

### bits

• `Optional` **bits**: `number`

#### Defined in[[bits.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:685](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L685)

___

### config\_groups

• `Optional` **config\_groups**: `Record`\<`string`, \{ `format?`: `string` ; `targets?`: `string`[] ; `weights?`: \{ `num_bits?`: `number`  }  }\>

#### Defined in[[configgroups.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:690](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L690)

___

### format

• `Optional` **format**: `string`

#### Defined in[[format.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:689](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L689)

___

### ignore

• `Optional` **ignore**: `string`[]

compressed-tensors names its exclusion list `ignore` rather than `modules_to_not_convert`,
using the same `re:`-prefixed target syntax as `config_groups[].targets`.

#### Defined in[[ignore.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:695](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L695)

___

### load\_in\_4bit

• `Optional` **load\_in\_4bit**: `boolean`

#### Defined in[[loadin4bit.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:686](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L686)

___

### load\_in\_8bit

• `Optional` **load\_in\_8bit**: `boolean`

#### Defined in[[loadin8bit.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:687](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L687)

___

### modules\_to\_not\_convert

• `Optional` **modules\_to\_not\_convert**: `string`[]

#### Defined in[[modulestonotconvert.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:684](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L684)

___

### quant\_method

• `Optional` **quant\_method**: `string`

#### Defined in[[quantmethod.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:683](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L683)
