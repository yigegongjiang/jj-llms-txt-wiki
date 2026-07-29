# Interface: ModelConfig

## Properties

### expert\_dtype

• `Optional` **expert\_dtype**: `string`

Some MoEs store their experts at a narrower precision than the rest of the model and declare
it here, *outside* `quantization_config` (e.g. DeepSeek-V4 is `quant_method: "fp8"` for
attention but `expert_dtype: "fp4"` for the experts, which dominate the parameter count).

#### Defined in[[expertdtype.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:586](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L586)

___

### quantization\_config

• `Optional` **quantization\_config**: [`QuantizationConfig`](QuantizationConfig)

#### Defined in[[quantizationconfig.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:579](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L579)

___

### text\_config

• `Optional` **text\_config**: \{ `quantization_config?`: [`QuantizationConfig`](QuantizationConfig)  } & `Pick`\<[`ModelConfig`](ModelConfig), ``"expert_dtype"``\>

#### Defined in[[textconfig.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:580](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L580)
