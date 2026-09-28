# Interface: MlxQuantizationConfig

## Hierarchy

- [`QuantizationConfig`](QuantizationConfig)

  ↳ **`MlxQuantizationConfig`**

## Indexable

▪ [key: `string`]: `unknown`

## Properties

### bits

• `Optional` **bits**: `number`

#### Inherited from[[bits.inherited-from]]

[QuantizationConfig](QuantizationConfig).[bits](QuantizationConfig#bits)

#### Defined in[[bits.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:773](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L773)

___

### config\_groups

• `Optional` **config\_groups**: `Record`\<`string`, \{ `format?`: `string` ; `targets?`: `string`[] ; `weights?`: \{ `num_bits?`: `number`  }  }\>

#### Inherited from[[configgroups.inherited-from]]

[QuantizationConfig](QuantizationConfig).[config_groups](QuantizationConfig#config_groups)

#### Defined in[[configgroups.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:778](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L778)

___

### expert\_dtype

• `Optional` **expert\_dtype**: `string`

Routed expert precision when it differs from the main quantizer (e.g. FP4 experts with FP8 attention).

#### Inherited from[[expertdtype.inherited-from]]

[QuantizationConfig](QuantizationConfig).[expert_dtype](QuantizationConfig#expert_dtype)

#### Defined in[[expertdtype.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:763](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L763)

___

### format

• `Optional` **format**: `string`

#### Inherited from[[format.inherited-from]]

[QuantizationConfig](QuantizationConfig).[format](QuantizationConfig#format)

#### Defined in[[format.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:777](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L777)

___

### group\_size

• `Optional` **group\_size**: `number`

#### Inherited from[[groupsize.inherited-from]]

[QuantizationConfig](QuantizationConfig).[group_size](QuantizationConfig#group_size)

#### Defined in[[groupsize.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:771](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L771)

___

### ignore

• `Optional` **ignore**: `string`[]

compressed-tensors names its exclusion list `ignore` rather than `modules_to_not_convert`,
using the same `re:`-prefixed target syntax as `config_groups[].targets`.

#### Inherited from[[ignore.inherited-from]]

[QuantizationConfig](QuantizationConfig).[ignore](QuantizationConfig#ignore)

#### Defined in[[ignore.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:783](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L783)

___

### load\_in\_4bit

• `Optional` **load\_in\_4bit**: `boolean`

#### Inherited from[[loadin4bit.inherited-from]]

[QuantizationConfig](QuantizationConfig).[load_in_4bit](QuantizationConfig#load_in_4bit)

#### Defined in[[loadin4bit.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:774](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L774)

___

### load\_in\_8bit

• `Optional` **load\_in\_8bit**: `boolean`

#### Inherited from[[loadin8bit.inherited-from]]

[QuantizationConfig](QuantizationConfig).[load_in_8bit](QuantizationConfig#load_in_8bit)

#### Defined in[[loadin8bit.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:775](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L775)

___

### mode

• `Optional` **mode**: `string`

MLX quantization mode (e.g. `affine`); MLX configs do not declare `quant_method`.

#### Inherited from[[mode.inherited-from]]

[QuantizationConfig](QuantizationConfig).[mode](QuantizationConfig#mode)

#### Defined in[[mode.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:770](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L770)

___

### modules\_to\_not\_convert

• `Optional` **modules\_to\_not\_convert**: `string`[]

#### Inherited from[[modulestonotconvert.inherited-from]]

[QuantizationConfig](QuantizationConfig).[modules_to_not_convert](QuantizationConfig#modules_to_not_convert)

#### Defined in[[modulestonotconvert.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:772](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L772)

___

### quant\_method

• `Optional` **quant\_method**: `string`

#### Inherited from[[quantmethod.inherited-from]]

[QuantizationConfig](QuantizationConfig).[quant_method](QuantizationConfig#quant_method)

#### Defined in[[quantmethod.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:761](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L761)

___

### store\_dtype

• `Optional` **store\_dtype**: `string`

Same role as `expert_dtype` under another name: MiMo-V2.6 is `quant_method: "fp8"` for its
dense layers but stores the routed experts as `store_dtype: "mxfp4"`, packed two per `U8`.

#### Inherited from[[storedtype.inherited-from]]

[QuantizationConfig](QuantizationConfig).[store_dtype](QuantizationConfig#store_dtype)

#### Defined in[[storedtype.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:768](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L768)
