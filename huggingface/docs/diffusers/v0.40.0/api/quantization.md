# Quantization

Quantization techniques reduce memory and computational costs by representing weights and activations with lower-precision data types like 8-bit integers (int8). This enables loading larger models you normally wouldn't be able to fit into memory, and speeding up inference.

> [!TIP]
> Learn how to quantize models in the [Quantization](../quantization/overview) guide.

## PipelineQuantizationConfig[[diffusers.PipelineQuantizationConfig]]

#### diffusers.PipelineQuantizationConfig[[diffusers.PipelineQuantizationConfig]]

```python
diffusers.PipelineQuantizationConfig(quant_backend: str = None, quant_kwargs: dict[str, str | float | int | dict] = None, components_to_quantize: list[str] | str | None = None, quant_mapping: dict[str, DiffQuantConfigMixin | 'TransformersQuantConfigMixin'] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/pipe_quant_config.py#L34)

**Parameters:**

quant_backend (`str`) : Quantization backend to be used. When using this option, we assume that the backend is available to both `diffusers` and `transformers`.

quant_kwargs (`dict`) : Params to initialize the quantization backend class.

components_to_quantize (`list`) : Components of a pipeline to be quantized.

quant_mapping (`dict`) : Mapping defining the quantization specs to be used for the pipeline components. When using this argument, users are not expected to provide `quant_backend`, `quant_kawargs`, and `components_to_quantize`.

Configuration class to be used when applying quantization on-the-fly to [from_pretrained()](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline.from_pretrained).

## BitsAndBytesConfig[[diffusers.BitsAndBytesConfig]]

#### diffusers.BitsAndBytesConfig[[diffusers.BitsAndBytesConfig]]

```python
diffusers.BitsAndBytesConfig(load_in_8bit = False, load_in_4bit = False, llm_int8_threshold = 6.0, llm_int8_skip_modules = None, llm_int8_enable_fp32_cpu_offload = False, llm_int8_has_fp16_weight = False, bnb_4bit_compute_dtype = None, bnb_4bit_quant_type = 'fp4', bnb_4bit_use_double_quant = False, bnb_4bit_quant_storage = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/quantization_config.py#L172)

**Parameters:**

load_in_8bit (`bool`, *optional*, defaults to `False`) : This flag is used to enable 8-bit quantization with LLM.int8().

load_in_4bit (`bool`, *optional*, defaults to `False`) : This flag is used to enable 4-bit quantization by replacing the Linear layers with FP4/NF4 layers from `bitsandbytes`.

llm_int8_threshold (`float`, *optional*, defaults to 6.0) : This corresponds to the outlier threshold for outlier detection as described in `LLM.int8() : 8-bit Matrix Multiplication for Transformers at Scale` paper: https://huggingface.co/papers/2208.07339 Any hidden states value that is above this threshold will be considered an outlier and the operation on those values will be done in fp16. Values are usually normally distributed, that is, most values are in the range [-3.5, 3.5], but there are some exceptional systematic outliers that are very differently distributed for large models. These outliers are often in the interval [-60, -6] or [6, 60]. Int8 quantization works well for values of magnitude ~5, but beyond that, there is a significant performance penalty. A good default threshold is 6, but a lower threshold might be needed for more unstable models (small models, fine-tuning).

llm_int8_skip_modules (`list[str]`, *optional*) : An explicit list of the modules that we do not want to convert in 8-bit. This is useful for models such as Jukebox that has several heads in different places and not necessarily at the last position. For example for `CausalLM` models, the last `lm_head` is typically kept in its original `dtype`.

llm_int8_enable_fp32_cpu_offload (`bool`, *optional*, defaults to `False`) : This flag is used for advanced use cases and users that are aware of this feature. If you want to split your model in different parts and run some parts in int8 on GPU and some parts in fp32 on CPU, you can use this flag. This is useful for offloading large models such as `google/flan-t5-xxl`. Note that the int8 operations will not be run on CPU.

llm_int8_has_fp16_weight (`bool`, *optional*, defaults to `False`) : This flag runs LLM.int8() with 16-bit main weights. This is useful for fine-tuning as the weights do not have to be converted back and forth for the backward pass.

bnb_4bit_compute_dtype (`torch.dtype` or str, *optional*, defaults to `torch.float32`) : This sets the computational type which might be different than the input type. For example, inputs might be fp32, but computation can be set to bf16 for speedups.

bnb_4bit_quant_type (`str`,  *optional*, defaults to `"fp4"`) : This sets the quantization data type in the bnb.nn.Linear4Bit layers. Options are FP4 and NF4 data types which are specified by `fp4` or `nf4`.

bnb_4bit_use_double_quant (`bool`, *optional*, defaults to `False`) : This flag is used for nested quantization where the quantization constants from the first quantization are quantized again.

bnb_4bit_quant_storage (`torch.dtype` or str, *optional*, defaults to `torch.uint8`) : This sets the storage type to pack the quanitzed 4-bit prarams.

kwargs (`dict[str, Any]`, *optional*) : Additional parameters from which to initialize the configuration object.

This is a wrapper class about all possible attributes and features that you can play with a model that has been
loaded using `bitsandbytes`.

This replaces `load_in_8bit` or `load_in_4bit` therefore both options are mutually exclusive.

Currently only supports `LLM.int8()`, `FP4`, and `NF4` quantization. If more methods are added to `bitsandbytes`,
then more arguments will be added to this class.

#### is_quantizable[[diffusers.BitsAndBytesConfig.is_quantizable]]

```python
is_quantizable()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/quantization_config.py#L351)

Returns `True` if the model is quantizable, `False` otherwise.

#### post_init[[diffusers.BitsAndBytesConfig.post_init]]

```python
post_init()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/quantization_config.py#L314)

Safety checker that arguments are correct - also replaces some NoneType arguments with their default values.

#### quantization_method[[diffusers.BitsAndBytesConfig.quantization_method]]

```python
quantization_method()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/quantization_config.py#L357)

This method returns the quantization method used for the model. If the model is not quantizable, it returns
`None`.

#### to_diff_dict[[diffusers.BitsAndBytesConfig.to_diff_dict]]

```python
to_diff_dict()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/quantization_config.py#L388)

**Returns:** `dict[str, Any]`

Dictionary of all the attributes that make up this configuration instance,

Removes all attributes from config which correspond to the default config attributes for better readability and
serializes to a Python dictionary.

## GGUFQuantizationConfig[[diffusers.GGUFQuantizationConfig]]

#### diffusers.GGUFQuantizationConfig[[diffusers.GGUFQuantizationConfig]]

```python
diffusers.GGUFQuantizationConfig(compute_dtype: 'torch.dtype' | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/quantization_config.py#L412)

**Parameters:**

compute_dtype : (`torch.dtype`, defaults to `torch.float32`): This sets the computational type which might be different than the input type. For example, inputs might be fp32, but computation can be set to bf16 for speedups.

This is a config class for GGUF Quantization techniques.

## NunchakuLiteQuantizationConfig[[diffusers.NunchakuLiteQuantizationConfig]]

#### diffusers.NunchakuLiteQuantizationConfig[[diffusers.NunchakuLiteQuantizationConfig]]

```python
diffusers.NunchakuLiteQuantizationConfig(compute_dtype: 'torch.dtype' | str | None = None, svdq_w4a4: dict[str, Any] | None = None, awq_w4a16: dict[str, Any] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/quantization_config.py#L435)

**Parameters:**

compute_dtype (`torch.dtype`, defaults to `torch.bfloat16`) : Runtime dtype used by the floating-point buffers in the quantized modules.

svdq_w4a4 (`dict`, *optional*) : Explicit SVDQ W4A4 target configuration with `precision`, `group_size`, `rank`, and `targets`.

awq_w4a16 (`dict`, *optional*) : Explicit AWQ W4A16 target configuration with `precision`, `group_size`, and `targets`.

Configuration for loading Nunchaku Lite checkpoints.

Nunchaku Lite support in Diffusers loads prequantized checkpoints. To create a compatible checkpoint, use
[`diffuse-compressor`](https://github.com/rootonchair/diffuse-compressor) to choose or adapt a target configuration
for the model architecture, quantize and export the transformer, and package it as a Diffusers pipeline with the
compact `quantization_config` stored in `config.json`.

The exported state dict must match the target Diffusers model architecture exactly. Checkpoints quantized with
fused QKV projections won't load into a model config that expects separate Q, K, and V projection modules.

Example compact `config.json` config:

```json
{
  "_class_name": "ErnieImageTransformer2DModel",
  "quantization_config": {
    "quant_method": "nunchaku_lite",
    "compute_dtype": "bfloat16",
    "svdq_w4a4": {
      "precision": "nvfp4",
      "group_size": 16,
      "rank": 32,
      "targets": ["layers.0.self_attention.to_q"]
    },
    "awq_w4a16": {
      "precision": "int4",
      "group_size": 64,
      "targets": ["final_linear"]
    }
  }
}
```

## QuantoConfig[[diffusers.QuantoConfig]]

#### diffusers.QuantoConfig[[diffusers.QuantoConfig]]

```python
diffusers.QuantoConfig(weights_dtype: str = 'int8', modules_to_not_convert: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/quantization_config.py#L651)

**Parameters:**

weights_dtype (`str`, *optional*, defaults to `"int8"`) : The target dtype for the weights after quantization. Supported values are ("float8","int8","int4","int2")

This is a wrapper class about all possible attributes and features that you can play with a model that has been
loaded using `quanto`.

modules_to_not_convert (`list`, *optional*, default to `None`):
The list of modules to not quantize, useful for quantizing models that explicitly require to have some
modules left in their original precision (e.g. Whisper encoder, Llava encoder, Mixtral gate layers).

#### post_init[[diffusers.QuantoConfig.post_init]]

```python
post_init()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/quantization_config.py#L678)

Safety checker that arguments are correct

## SDNQConfig[[diffusers.SDNQConfig]]

#### diffusers.SDNQConfig[[diffusers.SDNQConfig]]

```python
diffusers.SDNQConfig(*args, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/quantization_config.py#L956)

**Parameters:**

weights_dtype (`str`, *optional*, defaults to `"int8"`) : The target dtype for the weights after quantization, e.g. `"int8"`, `"uint4"`, `"float8_e4m3fn"`. See `sdnq.common.accepted_weight_dtypes` for all supported values.

group_size (`int`, *optional*, defaults to `0`) : How many elements of a tensor share the same quantization group. `0` auto-selects based on `weights_dtype`, `-1` disables grouping and uses row-wise quantization.

use_svd (`bool`, *optional*, defaults to `False`) : Whether to apply the SVDQuant algorithm on top of SDNQ quantization.

use_quantized_matmul (`bool`, *optional*, defaults to `False`) : Whether to use quantized INT8 / FP8 / FP16 matmul on the forward pass instead of BF16 / FP16.

modules_to_not_convert (`list`, *optional*, defaults to `None`) : The list of modules to skip during quantization.

kwargs (`dict[str, Any]`, *optional*) : Additional keyword arguments forwarded to `sdnq.SDNQConfig` (e.g. `quantized_matmul_dtype`, `svd_rank`, `use_hadamard`, `quant_conv`, `quant_embedding`, `modules_dtype_dict`).

Configuration class for SDNQ (SD.Next Quantization).

The `sdnq` library ships its own diffusers-compatible config and quantizer; this class is a thin factory that
defers to them so that `quant_method="sdnq"` checkpoints load natively with diffusers. All arguments are forwarded
to `sdnq.SDNQConfig`. Requires the `sdnq` library: `pip install sdnq`.

Reference: https://github.com/Disty0/sdnq

## TorchAoConfig[[diffusers.TorchAoConfig]]

#### diffusers.TorchAoConfig[[diffusers.TorchAoConfig]]

```python
diffusers.TorchAoConfig(quant_type: 'AOBaseConfig', modules_to_not_convert: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/quantization_config.py#L556)

**Parameters:**

quant_type (`AOBaseConfig`) : An `AOBaseConfig` subclass instance specifying the quantization type. See the [torchao documentation](https://docs.pytorch.org/ao/main/api_ref_quantization.html#inference-apis-for-quantize) for available config classes (e.g. `Int4WeightOnlyConfig`, `Int8WeightOnlyConfig`, `Float8WeightOnlyConfig`, `Float8DynamicActivationFloat8WeightConfig`, etc.).

modules_to_not_convert (`list[str]`, *optional*, default to `None`) : The list of modules to not quantize, useful for quantizing models that explicitly require to have some modules left in their original precision.

This is a config class for torchao quantization/sparsity techniques.

Example:
```python
from diffusers import FluxTransformer2DModel, TorchAoConfig
from torchao.quantization import Int8WeightOnlyConfig

quantization_config = TorchAoConfig(Int8WeightOnlyConfig())
transformer = FluxTransformer2DModel.from_pretrained(
    "black-forest-labs/Flux.1-Dev",
    subfolder="transformer",
    quantization_config=quantization_config,
    torch_dtype=torch.bfloat16,
)
```

#### from_dict[[diffusers.TorchAoConfig.from_dict]]

```python
from_dict(config_dict, return_unused_kwargs = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/quantization_config.py#L619)

Create configuration from a dictionary.

#### get_apply_tensor_subclass[[diffusers.TorchAoConfig.get_apply_tensor_subclass]]

```python
get_apply_tensor_subclass()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/quantization_config.py#L641)

Create the appropriate quantization method based on configuration.

#### to_dict[[diffusers.TorchAoConfig.to_dict]]

```python
to_dict()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/quantization_config.py#L605)

Convert configuration to a dictionary.

## DiffusersQuantizer[[diffusers.DiffusersQuantizer]]

#### diffusers.DiffusersQuantizer[[diffusers.DiffusersQuantizer]]

```python
diffusers.DiffusersQuantizer(quantization_config: QuantizationConfigMixin, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/base.py#L34)

Abstract class of the HuggingFace quantizer. Supports for now quantizing HF diffusers models for inference and/or
quantization. This class is used only for diffusers.models.modeling_utils.ModelMixin.from_pretrained and cannot be
easily used outside the scope of that method yet.

Attributes
quantization_config (`diffusers.quantizers.quantization_config.QuantizationConfigMixin`):
The quantization config that defines the quantization parameters of your model that you want to quantize.
modules_to_not_convert (`list[str]`, *optional*):
The list of module names to not convert when quantizing the model.
required_packages (`list[str]`, *optional*):
The list of required pip packages to install prior to using the quantizer
requires_calibration (`bool`):
Whether the quantization method requires to calibrate the model before using it.

#### adjust_max_memory[[diffusers.DiffusersQuantizer.adjust_max_memory]]

```python
adjust_max_memory(max_memory: dict)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/base.py#L133)

adjust max_memory argument for infer_auto_device_map() if extra memory is needed for quantization

#### adjust_target_dtype[[diffusers.DiffusersQuantizer.adjust_target_dtype]]

```python
adjust_target_dtype(torch_dtype: torch.dtype)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/base.py#L91)

**Parameters:**

torch_dtype (`torch.dtype`, *optional*) : The torch_dtype that is used to compute the device_map.

Override this method if you want to adjust the `target_dtype` variable used in `from_pretrained` to compute the
device_map in case the device_map is a `str`. E.g. for bitsandbytes we force-set `target_dtype` to `torch.int8`
and for 4-bit we pass a custom enum `accelerate.CustomDtype.int4`.

#### check_if_quantized_param[[diffusers.DiffusersQuantizer.check_if_quantized_param]]

```python
check_if_quantized_param(model: ModelMixin, param_value: torch.Tensor, param_name: str, state_dict: dict, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/base.py#L137)

checks if a loaded state_dict component is part of quantized param + some validation; only defined for
quantization methods that require to create a new parameters for quantization.

#### check_quantized_param_shape[[diffusers.DiffusersQuantizer.check_quantized_param_shape]]

```python
check_quantized_param_shape(*args, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/base.py#L157)

checks if the quantized param has expected shape.

#### create_quantized_param[[diffusers.DiffusersQuantizer.create_quantized_param]]

```python
create_quantized_param(*args, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/base.py#L151)

takes needed components from state_dict and creates quantized param.

#### dequantize[[diffusers.DiffusersQuantizer.dequantize]]

```python
dequantize(model)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/base.py#L219)

Potentially dequantize the model to retrieve the original model, with some loss in accuracy / performance. Note
not all quantization schemes support this.

#### get_cuda_warm_up_factor[[diffusers.DiffusersQuantizer.get_cuda_warm_up_factor]]

```python
get_cuda_warm_up_factor()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/base.py#L232)

The factor to be used in `caching_allocator_warmup` to get the number of bytes to pre-allocate to warm up cuda.
A factor of 2 means we allocate all bytes in the empty model (since we allocate in fp16), a factor of 4 means
we allocate half the memory of the weights residing in the empty model, etc...

#### get_special_dtypes_update[[diffusers.DiffusersQuantizer.get_special_dtypes_update]]

```python
get_special_dtypes_update(model, torch_dtype: torch.dtype)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/base.py#L113)

**Parameters:**

model (`~diffusers.models.modeling_utils.ModelMixin`) : The model to quantize

torch_dtype (`torch.dtype`) : The dtype passed in `from_pretrained` method.

returns dtypes for modules that are not quantized - used for the computation of the device_map in case one
passes a str as a device_map. The method will use the `modules_to_not_convert` that is modified in
`_process_model_before_weight_loading`. `diffusers` models don't have any `modules_to_not_convert` attributes
yet but this can change soon in the future.

#### postprocess_model[[diffusers.DiffusersQuantizer.postprocess_model]]

```python
postprocess_model(model: ModelMixin, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/base.py#L206)

**Parameters:**

model (`~diffusers.models.modeling_utils.ModelMixin`) : The model to quantize

kwargs (`dict`, *optional*) : The keyword arguments that are passed along `_process_model_after_weight_loading`.

Post-process the model post weights loading. Make sure to override the abstract method
`_process_model_after_weight_loading`.

#### preprocess_model[[diffusers.DiffusersQuantizer.preprocess_model]]

```python
preprocess_model(model: ModelMixin, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/base.py#L190)

**Parameters:**

model (`~diffusers.models.modeling_utils.ModelMixin`) : The model to quantize

kwargs (`dict`, *optional*) : The keyword arguments that are passed along `_process_model_before_weight_loading`.

Setting model attributes and/or converting model before weights loading. At this point the model should be
initialized on the meta device so you can freely manipulate the skeleton of the model in order to replace
modules in-place. Make sure to override the abstract method `_process_model_before_weight_loading`.

#### update_device_map[[diffusers.DiffusersQuantizer.update_device_map]]

```python
update_device_map(device_map: dict[str, typing.Any] | None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/base.py#L79)

**Parameters:**

device_map (`dict | str`, *optional*) : The device_map that is passed through the `from_pretrained` method.

Override this method if you want to pass a override the existing device map with a new one. E.g. for
bitsandbytes, since `accelerate` is a hard requirement, if no device_map is passed, the device_map is set to
`"auto"``

#### update_missing_keys[[diffusers.DiffusersQuantizer.update_missing_keys]]

```python
update_missing_keys(model, missing_keys: list, prefix: str)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/base.py#L103)

**Parameters:**

missing_keys (`list[str]`, *optional*) : The list of missing keys in the checkpoint compared to the state dict of the model

Override this method if you want to adjust the `missing_keys`.

#### update_torch_dtype[[diffusers.DiffusersQuantizer.update_torch_dtype]]

```python
update_torch_dtype(torch_dtype: torch.dtype)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/base.py#L68)

**Parameters:**

torch_dtype (`torch.dtype`) : The input dtype that is passed in `from_pretrained`

Some quantization methods require to explicitly set the dtype of the model to a target dtype. You need to
override this method in case you want to make sure that behavior is preserved

#### validate_environment[[diffusers.DiffusersQuantizer.validate_environment]]

```python
validate_environment(*args, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/quantizers/base.py#L163)

This method is used to potentially check for potential conflicts with arguments that are passed in
`from_pretrained`. You need to define it for all future quantizers that are integrated with diffusers. If no
explicit check are needed, simply return nothing.
