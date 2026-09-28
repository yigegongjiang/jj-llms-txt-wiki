# Interface: ModelConfig

## Properties

### expert\_dtype

• `Optional` **expert\_dtype**: `string`

Some MoEs store their experts at a narrower precision than the rest of the model and declare
it here, outside `quantization_config` (e.g. DeepSeek-V4). Used as a fallback when the
quantization config does not declare its own `expert_dtype` (as DeepSeek-V4.1 does).

#### Defined in[[expertdtype.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:804](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L804)

___

### quantization

• `Optional` **quantization**: [`MlxQuantizationConfig`](MlxQuantizationConfig)

Current MLX format, optionally with per-module overrides keyed by module name.

#### Defined in[[quantization.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:796](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L796)

___

### quantization\_config

• `Optional` **quantization\_config**: [`MlxQuantizationConfig`](MlxQuantizationConfig) \| [`QuantizationConfig`](QuantizationConfig)

#### Defined in[[quantizationconfig.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:797](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L797)

___

### text\_config

• `Optional` **text\_config**: `Pick`\<[`ModelConfig`](ModelConfig), ``"expert_dtype"`` \| ``"quantization"`` \| ``"quantization_config"``\>

#### Defined in[[textconfig.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:798](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L798)
