# Quantization

Quantization techniques reduce memory and computational costs by representing weights and activations with lower-precision data types like 8-bit integers (int8). This enables loading larger models you normally wouldn't be able to fit into memory, and speeding up inference.

> [!TIP]
> Learn how to quantize models in the [Quantization](../quantization/overview) guide.

## PipelineQuantizationConfig[[diffusers.PipelineQuantizationConfig]]

#### diffusers.PipelineQuantizationConfig[[diffusers.PipelineQuantizationConfig]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/pipe_quant_config.py#L34)

Configuration class to be used when applying quantization on-the-fly to [from_pretrained()](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline.from_pretrained).

**Parameters:**

quant_backend (`str`) : Quantization backend to be used. When using this option, we assume that the backend is available to both `diffusers` and `transformers`.

quant_kwargs (`dict`) : Params to initialize the quantization backend class.

components_to_quantize (`list`) : Components of a pipeline to be quantized.

quant_mapping (`dict`) : Mapping defining the quantization specs to be used for the pipeline components. When using this argument, users are not expected to provide `quant_backend`, `quant_kawargs`, and `components_to_quantize`.

## BitsAndBytesConfig[[diffusers.BitsAndBytesConfig]]

#### diffusers.BitsAndBytesConfig[[diffusers.BitsAndBytesConfig]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/quantization_config.py#L170)

This is a wrapper class about all possible attributes and features that you can play with a model that has been
loaded using `bitsandbytes`.

This replaces `load_in_8bit` or `load_in_4bit` therefore both options are mutually exclusive.

Currently only supports `LLM.int8()`, `FP4`, and `NF4` quantization. If more methods are added to `bitsandbytes`,
then more arguments will be added to this class.

is_quantizablediffusers.BitsAndBytesConfig.is_quantizablehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/quantization_config.py#L349[]

Returns `True` if the model is quantizable, `False` otherwise.

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
#### post_init[[diffusers.BitsAndBytesConfig.post_init]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/quantization_config.py#L312)

Safety checker that arguments are correct - also replaces some NoneType arguments with their default values.
#### quantization_method[[diffusers.BitsAndBytesConfig.quantization_method]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/quantization_config.py#L355)

This method returns the quantization method used for the model. If the model is not quantizable, it returns
`None`.
#### to_diff_dict[[diffusers.BitsAndBytesConfig.to_diff_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/quantization_config.py#L386)

Removes all attributes from config which correspond to the default config attributes for better readability and
serializes to a Python dictionary.

**Returns:**

``dict[str, Any]``

Dictionary of all the attributes that make up this configuration instance,

## GGUFQuantizationConfig[[diffusers.GGUFQuantizationConfig]]

#### diffusers.GGUFQuantizationConfig[[diffusers.GGUFQuantizationConfig]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/quantization_config.py#L410)

This is a config class for GGUF Quantization techniques.

**Parameters:**

compute_dtype : (`torch.dtype`, defaults to `torch.float32`): This sets the computational type which might be different than the input type. For example, inputs might be fp32, but computation can be set to bf16 for speedups.

## QuantoConfig[[diffusers.QuantoConfig]]

#### diffusers.QuantoConfig[[diffusers.QuantoConfig]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/quantization_config.py#L528)

This is a wrapper class about all possible attributes and features that you can play with a model that has been
loaded using `quanto`.

modules_to_not_convert (`list`, *optional*, default to `None`):
The list of modules to not quantize, useful for quantizing models that explicitly require to have some
modules left in their original precision (e.g. Whisper encoder, Llava encoder, Mixtral gate layers).

post_initdiffusers.QuantoConfig.post_inithttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/quantization_config.py#L555[]

Safety checker that arguments are correct

**Parameters:**

weights_dtype (`str`, *optional*, defaults to `"int8"`) : The target dtype for the weights after quantization. Supported values are ("float8","int8","int4","int2")

## TorchAoConfig[[diffusers.TorchAoConfig]]

#### diffusers.TorchAoConfig[[diffusers.TorchAoConfig]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/quantization_config.py#L433)

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

from_dictdiffusers.TorchAoConfig.from_dicthttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/quantization_config.py#L496[{"name": "config_dict", "val": ""}, {"name": "return_unused_kwargs", "val": " = False"}, {"name": "**kwargs", "val": ""}]
Create configuration from a dictionary.

**Parameters:**

quant_type (`AOBaseConfig`) : An `AOBaseConfig` subclass instance specifying the quantization type. See the [torchao documentation](https://docs.pytorch.org/ao/main/api_ref_quantization.html#inference-apis-for-quantize) for available config classes (e.g. `Int4WeightOnlyConfig`, `Int8WeightOnlyConfig`, `Float8WeightOnlyConfig`, `Float8DynamicActivationFloat8WeightConfig`, etc.).

modules_to_not_convert (`list[str]`, *optional*, default to `None`) : The list of modules to not quantize, useful for quantizing models that explicitly require to have some modules left in their original precision.
#### get_apply_tensor_subclass[[diffusers.TorchAoConfig.get_apply_tensor_subclass]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/quantization_config.py#L518)

Create the appropriate quantization method based on configuration.
#### to_dict[[diffusers.TorchAoConfig.to_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/quantization_config.py#L482)

Convert configuration to a dictionary.

## DiffusersQuantizer[[diffusers.DiffusersQuantizer]]

#### diffusers.DiffusersQuantizer[[diffusers.DiffusersQuantizer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/base.py#L34)

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

adjust_max_memorydiffusers.DiffusersQuantizer.adjust_max_memoryhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/base.py#L133[{"name": "max_memory", "val": ": dict"}]
adjust max_memory argument for infer_auto_device_map() if extra memory is needed for quantization
#### adjust_target_dtype[[diffusers.DiffusersQuantizer.adjust_target_dtype]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/base.py#L91)

Override this method if you want to adjust the `target_dtype` variable used in `from_pretrained` to compute the
device_map in case the device_map is a `str`. E.g. for bitsandbytes we force-set `target_dtype` to `torch.int8`
and for 4-bit we pass a custom enum `accelerate.CustomDtype.int4`.

**Parameters:**

torch_dtype (`torch.dtype`, *optional*) : The torch_dtype that is used to compute the device_map.
#### check_if_quantized_param[[diffusers.DiffusersQuantizer.check_if_quantized_param]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/base.py#L137)

checks if a loaded state_dict component is part of quantized param + some validation; only defined for
quantization methods that require to create a new parameters for quantization.
#### check_quantized_param_shape[[diffusers.DiffusersQuantizer.check_quantized_param_shape]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/base.py#L157)

checks if the quantized param has expected shape.
#### create_quantized_param[[diffusers.DiffusersQuantizer.create_quantized_param]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/base.py#L151)

takes needed components from state_dict and creates quantized param.
#### dequantize[[diffusers.DiffusersQuantizer.dequantize]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/base.py#L219)

Potentially dequantize the model to retrieve the original model, with some loss in accuracy / performance. Note
not all quantization schemes support this.
#### get_cuda_warm_up_factor[[diffusers.DiffusersQuantizer.get_cuda_warm_up_factor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/base.py#L232)

The factor to be used in `caching_allocator_warmup` to get the number of bytes to pre-allocate to warm up cuda.
A factor of 2 means we allocate all bytes in the empty model (since we allocate in fp16), a factor of 4 means
we allocate half the memory of the weights residing in the empty model, etc...
#### get_special_dtypes_update[[diffusers.DiffusersQuantizer.get_special_dtypes_update]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/base.py#L113)

returns dtypes for modules that are not quantized - used for the computation of the device_map in case one
passes a str as a device_map. The method will use the `modules_to_not_convert` that is modified in
`_process_model_before_weight_loading`. `diffusers` models don't have any `modules_to_not_convert` attributes
yet but this can change soon in the future.

**Parameters:**

model (`~diffusers.models.modeling_utils.ModelMixin`) : The model to quantize

torch_dtype (`torch.dtype`) : The dtype passed in `from_pretrained` method.
#### postprocess_model[[diffusers.DiffusersQuantizer.postprocess_model]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/base.py#L206)

Post-process the model post weights loading. Make sure to override the abstract method
`_process_model_after_weight_loading`.

**Parameters:**

model (`~diffusers.models.modeling_utils.ModelMixin`) : The model to quantize

kwargs (`dict`, *optional*) : The keyword arguments that are passed along `_process_model_after_weight_loading`.
#### preprocess_model[[diffusers.DiffusersQuantizer.preprocess_model]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/base.py#L190)

Setting model attributes and/or converting model before weights loading. At this point the model should be
initialized on the meta device so you can freely manipulate the skeleton of the model in order to replace
modules in-place. Make sure to override the abstract method `_process_model_before_weight_loading`.

**Parameters:**

model (`~diffusers.models.modeling_utils.ModelMixin`) : The model to quantize

kwargs (`dict`, *optional*) : The keyword arguments that are passed along `_process_model_before_weight_loading`.
#### update_device_map[[diffusers.DiffusersQuantizer.update_device_map]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/base.py#L79)

Override this method if you want to pass a override the existing device map with a new one. E.g. for
bitsandbytes, since `accelerate` is a hard requirement, if no device_map is passed, the device_map is set to
`"auto"``

**Parameters:**

device_map (`dict | str`, *optional*) : The device_map that is passed through the `from_pretrained` method.
#### update_missing_keys[[diffusers.DiffusersQuantizer.update_missing_keys]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/base.py#L103)

Override this method if you want to adjust the `missing_keys`.

**Parameters:**

missing_keys (`list[str]`, *optional*) : The list of missing keys in the checkpoint compared to the state dict of the model
#### update_torch_dtype[[diffusers.DiffusersQuantizer.update_torch_dtype]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/base.py#L68)

Some quantization methods require to explicitly set the dtype of the model to a target dtype. You need to
override this method in case you want to make sure that behavior is preserved

**Parameters:**

torch_dtype (`torch.dtype`) : The input dtype that is passed in `from_pretrained`
#### validate_environment[[diffusers.DiffusersQuantizer.validate_environment]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/quantizers/base.py#L163)

This method is used to potentially check for potential conflicts with arguments that are passed in
`from_pretrained`. You need to define it for all future quantizers that are integrated with diffusers. If no
explicit check are needed, simply return nothing.
