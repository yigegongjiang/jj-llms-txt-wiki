# Quantization

Quantization techniques reduce memory and computational costs by representing weights and activations with lower-precision data types like 8-bit integers (int8). This enables loading larger models you normally wouldn't be able to fit into memory, and speeding up inference. Transformers supports the AWQ and GPTQ quantization algorithms and it supports 8-bit and 4-bit quantization with bitsandbytes.

Quantization techniques that aren't supported in Transformers can be added with the `HfQuantizer` class.

Learn how to quantize models in the [Quantization](../quantization/overview) guide.

## QuantoConfig[[transformers.QuantoConfig]]

- **weights** (`str`, *optional*, defaults to `"int8"`) --
  The target dtype for the weights after quantization. Supported values are ("float8","int8","int4","int2")
- **activations** (`str`, *optional*) --
  The target dtype for the activations after quantization. Supported values are (None,"int8","float8")
- **modules_to_not_convert** (`list`, *optional*, default to `None`) --
  The list of modules to not quantize, useful for quantizing models that explicitly require to have
  some modules left in their original precision (e.g. Whisper encoder, Llava encoder, Mixtral gate layers).

This is a wrapper class about all possible attributes and features that you can play with a model that has been
loaded using `quanto`.

Safety checker that arguments are correct

## AqlmConfig[[transformers.AqlmConfig]]

- **in_group_size** (`int`, *optional*, defaults to 8) --
  The group size along the input dimension.
- **out_group_size** (`int`, *optional*, defaults to 1) --
  The group size along the output dimension. It's recommended to always use 1.
- **num_codebooks** (`int`, *optional*, defaults to 1) --
  Number of codebooks for the Additive Quantization procedure.
- **nbits_per_codebook** (`int`, *optional*, defaults to 16) --
  Number of bits encoding a single codebook vector. Codebooks size is 2**nbits_per_codebook.
- **linear_weights_not_to_quantize** (`Optional[list[str]]`, *optional*) --
  List of full paths of `nn.Linear` weight parameters that shall not be quantized.
- **kwargs** (`dict[str, Any]`, *optional*) --
  Additional parameters from which to initialize the configuration object.

This is a wrapper class about `aqlm` parameters.

Safety checker that arguments are correct - also replaces some NoneType arguments with their default values.

## VptqConfig[[transformers.VptqConfig]]

- **enable_proxy_error** (`bool`, *optional*, defaults to `False`) -- calculate proxy error for each layer
- **config_for_layers** (`Dict`, *optional*, defaults to `{}`) -- quantization params for each layer
- **shared_layer_config** (`Dict`, *optional*, defaults to `{}`) -- shared quantization params among layers
- **modules_to_not_convert** (`list`, *optional*, default to `None`) --
  The list of modules to not quantize, useful for quantizing models that explicitly require to have
  some modules left in their original precision (e.g. Whisper encoder, Llava encoder, Mixtral gate layers).
- **kwargs** (`dict[str, Any]`, *optional*) --
  Additional parameters from which to initialize the configuration object.

This is a wrapper class about `vptq` parameters.

Safety checker that arguments are correct

## AwqConfig[[transformers.AwqConfig]]

"}, {"name": "modules_to_not_convert", "val": ": list | None = None"}, {"name": "**kwargs", "val": ""}]}>
- **bits** (`int`, *optional*, defaults to 4) --
  The number of bits to quantize to.
- **group_size** (`int`, *optional*, defaults to 128) --
  The group size to use for quantization. Recommended value is 128 and -1 uses per-column quantization.
- **zero_point** (`bool`, *optional*, defaults to `True`) --
  Whether to use zero point quantization.
- **backend** (`AwqBackend`, *optional*, defaults to `AwqBackend.AUTO`) --
  The quantization backend.
- **modules_to_not_convert** (`list`, *optional*, default to `None`) --
  The list of modules to not quantize, useful for quantizing models that explicitly require to have
  some modules left in their original precision (e.g. Whisper encoder, Llava encoder, Mixtral gate layers).
  Note you cannot quantize directly with transformers, please refer to `AutoAWQ` documentation for quantizing HF models.

This is a wrapper class about all possible attributes and features that you can play with a model that has been
loaded using `auto-awq` library awq quantization relying on auto_awq backend.

## EetqConfig[[transformers.EetqConfig]]

- **weights** (`str`, *optional*, defaults to `"int8"`) --
  The target dtype for the weights. Supported value is only "int8"
- **modules_to_not_convert** (`list`, *optional*, default to `None`) --
  The list of modules to not quantize, useful for quantizing models that explicitly require to have
  some modules left in their original precision.

This is a wrapper class about all possible attributes and features that you can play with a model that has been
loaded using `eetq`.

Safety checker that arguments are correct

## GPTQConfig[[transformers.GPTQConfig]]

- **bits** (`int`) --
  The number of bits to quantize to, supported numbers are (2, 3, 4, 8).
- **tokenizer** (`str` or `PreTrainedTokenizerBase`, *optional*) --
  The tokenizer used to process the dataset. You can pass either:
  - A custom tokenizer object.
  - A string, the *model id* of a predefined tokenizer hosted inside a model repo on huggingface.co.
  - A path to a *directory* containing vocabulary files required by the tokenizer, for instance saved
    using the [save_pretrained()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.save_pretrained) method, e.g., `./my_model_directory/`.
- **dataset** (`Union[list[str]]`, *optional*) --
  The dataset used for quantization. You can provide your own dataset in a list of string or just use the
  original datasets used in GPTQ paper ['wikitext2','c4','c4-new']
- **group_size** (`int`, *optional*, defaults to 128) --
  The group size to use for quantization. Recommended value is 128 and -1 uses per-column quantization.
- **damp_percent** (`float`, *optional*, defaults to 0.1) --
  The percent of the average Hessian diagonal to use for dampening. Recommended value is 0.1.
- **desc_act** (`bool`, *optional*, defaults to `False`) --
  Whether to quantize columns in order of decreasing activation size. Setting it to False can significantly
  speed up inference but the perplexity may become slightly worse. Also known as act-order.
- **act_group_aware** (`bool`, *optional*, defaults to `True`) --
  Use GAR (group aware activation order) during quantization. Has measurable positive impact on quantization
  quality. Only applicable when `desc_act = False`. Will forced to be `False` when `desc_act = True`.
- **sym** (`bool`, *optional*, defaults to `True`) --
  Whether to use symmetric quantization.
- **true_sequential** (`bool`, *optional*, defaults to `True`) --
  Whether to perform sequential quantization even within a single Transformer block. Instead of quantizing
  the entire block at once, we perform layer-wise quantization. As a result, each layer undergoes
  quantization using inputs that have passed through the previously quantized layers.
- **format** (`str`, *optional*, defaults to `"gptq"`) --
  GPTQ weight format. `gptq` (v1) is supported by gptqmodel. `gptq_v2` is gptqmodel only.
- **meta** (`dict[str, any]`, *optional*) --
  Properties, such as tooling:version, that do not directly contributes to quantization or quant inference are stored in meta.
  i.e. `meta.quantizer`: ["optimum:_version_", "gptqmodel:_version_"]
- **backend** (`str`, *optional*) --
  Controls which kernel to use. Valid values for gptqmodel are `auto`, `auto_trainable` and more. Ref gptqmodel backends:
  https://github.com/ModelCloud/GPTQModel/blob/main/gptqmodel/utils/backend.py
- **model_seqlen** (`int`, *optional*) --
  The maximum sequence length that the model can take.
- **block_name_to_quantize** (`str`, *optional*) --
  The transformers block name to quantize. If None, we will infer the block name using common patterns (e.g. model.layers)
- **module_name_preceding_first_block** (`list[str]`, *optional*) --
  The layers that are preceding the first Transformer block.
- **batch_size** (`int`, *optional*, defaults to 1) --
  The batch size used when processing the dataset
- **pad_token_id** (`int`, *optional*) --
  The pad token id. Needed to prepare the dataset when `batch_size` > 1.
- **max_input_length** (`int`, *optional*) --
  The maximum input length. This is needed to initialize a buffer that depends on the maximum expected input
  length. It is specific to the exllama backend with act-order.
- **cache_block_outputs** (`bool`, *optional*, defaults to `True`) --
  Whether to cache block outputs to reuse as inputs for the succeeding block.
- **modules_in_block_to_quantize** (`list[list[str]]`, *optional*) --
  List of list of module names to quantize in the specified block. This argument is useful to exclude certain linear modules from being quantized.
  The block to quantize can be specified by setting `block_name_to_quantize`. We will quantize each list sequentially. If not set, we will quantize all linear layers.
  Example: `modules_in_block_to_quantize =[["self_attn.k_proj", "self_attn.v_proj", "self_attn.q_proj"], ["self_attn.o_proj"]]`.
  In this example, we will first quantize the q,k,v layers simultaneously since they are independent.
  Then, we will quantize `self_attn.o_proj` layer with the q,k,v layers quantized. This way, we will get
  better results since it reflects the real input `self_attn.o_proj` will get when the model is quantized.

This is a wrapper class about all possible attributes and features that you can play with a model that has been
loaded using `optimum` api for GPTQ quantization relying on the gptqmodel backend.

Get compatible class with optimum gptq config dict

Safety checker that arguments are correct

Get compatible dict for optimum gptq config

## BitsAndBytesConfig[[transformers.BitsAndBytesConfig]]

- **load_in_8bit** (`bool`, *optional*, defaults to `False`) --
  This flag is used to enable 8-bit quantization with LLM.int8().
- **load_in_4bit** (`bool`, *optional*, defaults to `False`) --
  This flag is used to enable 4-bit quantization by replacing the Linear layers with FP4/NF4 layers from
  `bitsandbytes`.
- **llm_int8_threshold** (`float`, *optional*, defaults to 6.0) --
  This corresponds to the outlier threshold for outlier detection as described in `LLM.int8() : 8-bit Matrix
  Multiplication for Transformers at Scale` paper: https://huggingface.co/papers/2208.07339 Any hidden states value
  that is above this threshold will be considered an outlier and the operation on those values will be done
  in fp16. Values are usually normally distributed, that is, most values are in the range [-3.5, 3.5], but
  there are some exceptional systematic outliers that are very differently distributed for large models.
  These outliers are often in the interval [-60, -6] or [6, 60]. Int8 quantization works well for values of
  magnitude ~5, but beyond that, there is a significant performance penalty. A good default threshold is 6,
  but a lower threshold might be needed for more unstable models (small models, fine-tuning).
- **llm_int8_skip_modules** (`list[str]`, *optional*) --
  An explicit list of the modules that we do not want to convert in 8-bit. This is useful for models such as
  Jukebox that has several heads in different places and not necessarily at the last position. For example
  for `CausalLM` models, the last `lm_head` is kept in its original `dtype`.
- **llm_int8_enable_fp32_cpu_offload** (`bool`, *optional*, defaults to `False`) --
  This flag is used for advanced use cases and users that are aware of this feature. If you want to split
  your model in different parts and run some parts in int8 on GPU and some parts in fp32 on CPU, you can use
  this flag. This is useful for offloading large models such as `google/flan-t5-xxl`. Note that the int8
  operations will not be run on CPU.
- **llm_int8_has_fp16_weight** (`bool`, *optional*, defaults to `False`) --
  This flag runs LLM.int8() with 16-bit main weights. This is useful for fine-tuning as the weights do not
  have to be converted back and forth for the backward pass.
- **bnb_4bit_compute_dtype** (`torch.dtype` or str, *optional*, defaults to `torch.float32`) --
  This sets the computational type which might be different than the input type. For example, inputs might be
  fp32, but computation can be set to bf16 for speedups.
- **bnb_4bit_quant_type** (`str`,  *optional*, defaults to `"fp4"`) --
  This sets the quantization data type in the bnb.nn.Linear4Bit layers. Options are FP4 and NF4 data types
  which are specified by `fp4` or `nf4`.
- **bnb_4bit_use_double_quant** (`bool`, *optional*, defaults to `False`) --
  This flag is used for nested quantization where the quantization constants from the first quantization are
  quantized again.
- **bnb_4bit_quant_storage** (`torch.dtype` or str, *optional*, defaults to `torch.uint8`) --
  This sets the storage type to pack the quantized 4-bit params.
- **kwargs** (`dict[str, Any]`, *optional*) --
  Additional parameters from which to initialize the configuration object.

This is a wrapper class about all possible attributes and features that you can play with a model that has been
loaded using `bitsandbytes`.

Currently only supports `LLM.int8()`, `FP4`, and `NF4` quantization. If more methods are added to `bitsandbytes`,
then more arguments will be added to this class.

Returns `True` if the model is quantizable, `False` otherwise.

Safety checker that arguments are correct - also replaces some NoneType arguments with their default values.

This method returns the quantization method used for the model. If the model is not quantizable, it returns
`None`.

`dict[str, Any]`Dictionary of all the attributes that make up this configuration instance,

Removes all attributes from config which correspond to the default config attributes for better readability and
serializes to a Python dictionary.

## HfQuantizer[[transformers.quantizers.HfQuantizer]]

Abstract class of the HuggingFace quantizer. Supports for now quantizing HF transformers models for inference and/or quantization.
This class is used only for transformers.PreTrainedModel.from_pretrained and cannot be easily used outside the scope of that method
yet.

Attributes
quantization_config (`transformers.utils.quantization_config.QuantizationConfigMixin`):
The quantization config that defines the quantization parameters of your model that you want to quantize.
requires_calibration (`bool`):
Whether the quantization method requires to calibrate the model before using it.

adjust max_memory argument for infer_auto_device_map() if extra memory is needed for quantization

Potentially dequantize the model to retrieve the original model, with some loss in accuracy / performance.
Note not all quantization schemes support this.

Override this method if you want to adjust the `param_name`.

Get state dict and metadata. Useful when we need to modify a bit the state dict due to quantization

Check whether a given param needs to be quantized.

- **model** (`~transformers.PreTrainedModel`) --
  The model to quantize
- **kwargs** (`dict`, *optional*) --
  The keyword arguments that are passed along `_process_model_after_weight_loading`.

Post-process the model post weights loading.
Make sure to override the abstract method `_process_model_after_weight_loading`.

- **model** (`~transformers.PreTrainedModel`) --
  The model to quantize
- **kwargs** (`dict`, *optional*) --
  The keyword arguments that are passed along `_process_model_before_weight_loading`.

Setting model attributes and/or converting model before weights loading. At this point
the model should be initialized on the meta device so you can freely manipulate the skeleton
of the model in order to replace modules in-place. Make sure to override the abstract method `_process_model_before_weight_loading`.

Remove the quantization config from the model.

- **device_map** (`Union[dict, str]`, *optional*) --
  The device_map that is passed through the `from_pretrained` method.

Override this method if you want to pass a override the existing device map with a new
one. E.g. for bitsandbytes, since `accelerate` is a hard requirement, if no device_map is
passed, the device_map is set to `"auto"``

- **dtype** (`torch.dtype`) --
  The input dtype that is passed in `from_pretrained`

Some quantization methods require to explicitly set the dtype of the model to a
target dtype. You need to override this method in case you want to make sure that behavior is
preserved

updates the tp plan for the scales

updates the tp plan for the scales

Give the quantizer a chance to rewrite the weight conversion pipeline.

Loading runs `renamings → converters → (dequant → merge → concat)`. Dequant
has to happen *before* any merge/concat op because those operations aren't
aware of per-block scales, so the per-expert (weight, scale) pairs need to be
collapsed into full-precision tensors first. Subclasses (e.g. the FP8
quantizer in `dequantize=True` mode) override this to inject a dequantize
op at the start of each model-provided [WeightConverter](/docs/transformers/v5.14.0/en/internal/modeling_utils#transformers.WeightConverter) and attach the
matching scale source patterns. Default: no-op.

This method is used to potentially check for potential conflicts with arguments that are
passed in `from_pretrained`. You need to define it for all future quantizers that are integrated with transformers.
If no explicit check are needed, simply return nothing.

## HiggsConfig[[transformers.HiggsConfig]]

- **bits** (int, *optional*, defaults to 4) --
  Number of bits to use for quantization. Can be 2, 3 or 4. Default is 4.
- **p** (int, *optional*, defaults to 2) --
  Quantization grid dimension. 1 and 2 are supported. 2 is always better in practice. Default is 2.
- **modules_to_not_convert** (`list`, *optional*, default to ["lm_head"]) --
  List of linear layers that should not be quantized.
- **hadamard_size** (int, *optional*, defaults to 512) --
  Hadamard size for the HIGGS method. Default is 512. Input dimension of matrices is padded to this value. Decreasing this below 512 will reduce the quality of the quantization.
- **group_size** (int, *optional*, defaults to 256) --
  Group size for the HIGGS method. Can be 64, 128 or 256. Decreasing it barely affects the performance. Default is 256. Must be a divisor of hadamard_size.
- **tune_metadata** ('dict', *optional*, defaults to {}) --
  Module-wise metadata (gemm block shapes, GPU metadata, etc.) for saving the kernel tuning results. Default is an empty dictionary. Is set automatically during tuning.

HiggsConfig is a configuration class for quantization using the HIGGS method.

Safety checker that arguments are correct - also replaces some NoneType arguments with their default values.

## HqqConfig[[transformers.HqqConfig]]

- **nbits** (`int`, *optional*, defaults to 4) --
  Number of bits. Supported values are (8, 4, 3, 2, 1).
- **group_size** (`int`, *optional*, defaults to 64) --
  Group-size value. Supported values are any value that is divisible by weight.shape[axis]).
- **view_as_float** (`bool`, *optional*, defaults to `False`) --
  View the quantized weight as float (used in distributed training) if set to `True`.
- **axis** (`Optional[int]`, *optional*) --
  Axis along which grouping is performed. Supported values are 0 or 1.
- **dynamic_config** (dict, *optional*) --
  Parameters for dynamic configuration. The key is the name tag of the layer and the value is a quantization config.
  If set, each layer specified by its id will use its dedicated quantization configuration.
- **skip_modules** (`list[str]`, *optional*, defaults to `['lm_head']`) --
  List of `nn.Linear` layers to skip.
- **kwargs** (`dict[str, Any]`, *optional*) --
  Additional parameters from which to initialize the configuration object.

This is wrapper around hqq's BaseQuantizeConfig.

Override from_dict, used in AutoQuantizationConfig.from_dict in quantizers/auto.py

Safety checker that arguments are correct - also replaces some NoneType arguments with their default values.

`dict[str, Any]`Dictionary of all the attributes that make up this configuration instance,

Removes all attributes from config which correspond to the default config attributes for better readability and
serializes to a Python dictionary.

## MetalConfig[[transformers.MetalConfig]]

Configuration class for Metal affine quantization targeting Apple Silicon (MPS) devices.

This quantization method uses the `mlx-quantization-metal-kernels` Metal kernels from the Hugging Face Hub
to perform affine quantization (scales + qbiases) with configurable bit-width and group size.
The quantized weights are packed into `uint32` tensors and the forward pass uses fused
dequantization + matmul Metal kernels.

## Mxfp4Config[[transformers.Mxfp4Config]]

- **modules_to_not_convert** (`list`, *optional*, default to `None`) --
  The list of modules to not quantize, useful for quantizing models that explicitly require to have
  some modules left in their original precision.
- **dequantize** (`bool`, *optional*, default to `False`) --
  Whether we dequantize the model to bf16 precision or not

This is a wrapper class about all possible attributes and features that you can play with a model that has been
loaded using mxfp4 quantization.

## FbgemmFp8Config[[transformers.FbgemmFp8Config]]

- **activation_scale_ub** (`float`, *optional*, defaults to 1200.0) --
  The activation scale upper bound. This is used when quantizing the input activation.
- **modules_to_not_convert** (`list`, *optional*, default to `None`) --
  The list of modules to not quantize, useful for quantizing models that explicitly require to have
  some modules left in their original precision.

This is a wrapper class about all possible attributes and features that you can play with a model that has been
loaded using fbgemm fp8 quantization.

## CompressedTensorsConfig[[transformers.CompressedTensorsConfig]]

- **config_groups** (`typing.dict[str, typing.Union[ForwardRef('QuantizationScheme'), typing.list[str]]]`, *optional*) --
  dictionary mapping group name to a quantization scheme definition
- **format** (`str`, *optional*, defaults to `"dense"`) --
  format the model is represented as. Set `run_compressed` True to execute model as the
  compressed format if not `dense`
- **quantization_status** (`QuantizationStatus`, *optional*, defaults to `"initialized"`) --
  status of model in the quantization lifecycle, ie 'initialized', 'calibration', 'frozen'
- **kv_cache_scheme** (`typing.Union[QuantizationArgs, NoneType]`, *optional*) --
  specifies quantization of the kv cache. If None, kv cache is not quantized.
- **global_compression_ratio** (`typing.Union[float, NoneType]`, *optional*) --
  0-1 float percentage of model compression
- **ignore** (`typing.Union[typing.list[str], NoneType]`, *optional*) --
  layer names or types to not quantize, supports regex prefixed by 're:'
- **quant_method** (`str`, *optional*, defaults to `"compressed-tensors"`) --
  do not override, should be compressed-tensors
- **run_compressed** (`bool`, *optional*, defaults to `True`) -- alter submodules (usually linear) in order to
  emulate compressed model execution if True, otherwise use default submodule

This is a wrapper class that handles compressed-tensors quantization config options.
It is a wrapper around `compressed_tensors.QuantizationConfig`

- **config_dict** (`dict[str, Any]`) --
  Dictionary that will be used to instantiate the configuration object.
- **return_unused_kwargs** (`bool`,*optional*, defaults to `False`) --
  Whether or not to return a list of unused keyword arguments. Used for `from_pretrained` method in
  `PreTrainedModel`.
- **kwargs** (`dict[str, Any]`) --
  Additional parameters from which to initialize the configuration object.`QuantizationConfigMixin`The configuration object instantiated from those parameters.

Instantiates a [CompressedTensorsConfig](/docs/transformers/v5.14.0/en/main_classes/quantization#transformers.CompressedTensorsConfig) from a Python dictionary of parameters.
Optionally unwraps any args from the nested quantization_config

Quantization config to be added to config.json

Serializes this instance to a Python dictionary. Returns:
`dict[str, Any]`: Dictionary of all the attributes that make up this configuration instance.

`dict[str, Any]`Dictionary of all the attributes that make up this configuration instance,

Removes all attributes from config which correspond to the default config attributes for better readability and
serializes to a Python dictionary.

## TorchAoConfig[[transformers.TorchAoConfig]]

- **quant_type** (`AOBaseConfig`) --
  A torchao `AOBaseConfig` instance specifying the quantization type, e.g.
  `Int4WeightOnlyConfig(group_size=32)`, `Int8WeightOnlyConfig()`,
  `Int8DynamicActivationInt8WeightConfig()`, `Float8WeightOnlyConfig()`, etc.
- **modules_to_not_convert** (`list`, *optional*, default to `None`) --
  The list of modules to not quantize, useful for quantizing models that explicitly require to have
  some modules left in their original precision.
- **include_input_output_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to include embedding in quantization or not, input embedding will be removed from
  the module_not_to_convert list as well if this flag is set.
- **untie_embedding_weights** (`bool`, *optional*, defaults to `False`) --
  Whether to untie the weights when we are quantizing input embedding weights that is tied
  to other weights.
Config class for torchao quantization/sparsity techniques.

Example:

```python
from torchao.quantization import Int4WeightOnlyConfig

quantization_config = TorchAoConfig(Int4WeightOnlyConfig(group_size=32))
model = AutoModelForCausalLM.from_pretrained(
    model_id, device_map="cuda", torch_dtype=torch.bfloat16, quantization_config=quantization_config
)
```

Create configuration from a dictionary.

Return the quantization config to apply.

Validate configuration and set defaults.

Convert configuration to a dictionary.

## BitNetQuantConfig[[transformers.BitNetQuantConfig]]

- **modules_to_not_convert** (`Optional[List]`, *optional*) --
  Optionally, provides a list of full paths of `nn.Linear` weight parameters
  that shall not be quantized. Defaults to None.
- **linear_class** (`str`, *optional*, defaults to `"bitlinear"`) --
  The type of linear class to use. Can be either `bitlinear` or `autobitlinear`.
- **quantization_mode** (`str`, *optional*, defaults to `"offline"`) --
  The quantization mode to use. Can be either `online` or `offline`.
  In `online` mode, the weight quantization parameters are calculated dynamically
  during each forward pass (e.g., based on the current weight values). This can
  adapt to weight changes during training (Quantization-Aware Training - QAT).
  In `offline` mode, quantization parameters are pre-calculated *before* inference.
  These parameters are then fixed and loaded into the quantized model. This
  generally results in lower runtime overhead compared to online quantization.
- **use_rms_norm** (`bool`, *optional*, defaults to `False`) --
  Whether to apply RMSNorm on the activations before quantization. This matches the original BitNet paper's approach
  of normalizing activations before quantization/packing.
- **rms_norm_eps** (`float`, *optional*, defaults to 1e-06) --
  The epsilon value used in the RMSNorm layer for numerical stability.
- **kwargs** (`dict[str, Any]`, *optional*) --
  Additional keyword arguments that may be used by specific quantization
  backends or future versions.

Configuration class for applying BitNet quantization.

Safety checker that arguments are correct

## SpQRConfig[[transformers.SpQRConfig]]

- **bits** (`int`, *optional*, defaults to 3) --
  Specifies the bit count for the weights and first order zero-points and scales.
  Currently only bits = 3 is supported.
- **beta1** (`int`, *optional*, defaults to 16) --
  SpQR tile width. Currently only beta1 = 16 is supported.
- **beta2** (`int`, *optional*, defaults to 16) --
  SpQR tile height. Currently only beta2 = 16 is supported.
- **shapes** (`Optional`, *optional*) --
  A dictionary holding the shape of each object. We need this because it's impossible
  to deduce the exact size of the parameters just from bits, beta1, beta2.
- **modules_to_not_convert** (`Optional[list[str]]`, *optional*) --
  Optionally, provides a list of full paths of `nn.Linear` weight parameters that shall not be quantized.
  Defaults to None.
- **kwargs** (`dict[str, Any]`, *optional*) --
  Additional parameters from which to initialize the configuration object.

This is a wrapper class about `spqr` parameters. Refer to the original publication for more details.

Safety checker that arguments are correct - also replaces some NoneType arguments with their default values.

## FineGrainedFP8Config[[transformers.FineGrainedFP8Config]]

- **activation_scheme** (`str`, *optional*, defaults to `"dynamic"`) --
  The scheme used for activation, the defaults and only support scheme for now is "dynamic".
- **weight_block_size** (`typing.tuple[int, int]`, *optional*, defaults to `(128, 128)`) --
  The size of the weight blocks for quantization, default is (128, 128).
- **dequantize** (`bool`, *optional*, defaults to `False`) --
  Whether to dequantize the model during loading.
- **modules_to_not_convert** (`list`, *optional*) --
  A list of module names that should not be converted during quantization.
- **scale_fmt** (`str`, *optional*, defaults to `"float"`) --
  Storage dtype of the per-block weight scales: `"float"` (fp32, V3-style) or
  `"ue8m0"` (1-byte `torch.float8_e8m0fnu`, V4-style).

FineGrainedFP8Config is a configuration class for fine-grained FP8 quantization used mainly for deepseek models.

Safety checker that arguments are correct

## QuarkConfig[[transformers.QuarkConfig]]

## FourOverSixConfig[[transformers.FourOverSixConfig]]

- **activation_dtype** (`str`, *optional*) --
  Data type to use when quantizing activation tensors. If not provided, `dtype` is used.
- **activation_scale_rule** (`str`, *optional*) --
  Scaling rule to use when selecting a scale for blocks in activation tensors. If not
  provided, `scale_rule` is used.
- **dtype** (`str`, default "nvfp4", *optional*, defaults to `"nvfp4"`) --
  The data type to use for the layer's weights, activations, and tensors. Can be
  `"nvfp4"` or `"mxfp4"`.
- **gradient_dtype** (`str`, *optional*) --
  Data type to use when quantizing gradient tensors. If not provided, `dtype` is used.
- **gradient_scale_rule** (`str`, *optional*) --
  Scaling rule to use when selecting a scale for blocks in gradient tensors. If not
  provided, `scale_rule` is used.
- **keep_master_weights** (`bool`, default False, *optional*, defaults to `False`) --
  Whether to keep the master weights. If `True`, high-precision weights are kept at all
  times and weights are quantized online in each forward pass. This is useful for
  quantized training.
- **matmul_backend** (`str`, *optional*) --
  The backend to use for matrix multiplications. Can be `"cutlass"` or `"pytorch"`. If
  not provided, CUTLASS will be used if available and PyTorch will be used otherwise.
- **output_dtype** (`str`, *optional*, defaults to `"bfloat16"`) --
  The data type to use for the output of the layer. Can be `"bfloat16"` or `"float16"`.
- **quantize_backend** (`str`, *optional*) --
  The backend to use for quantization. Can be `"cuda"`, `"triton"`, or `"pytorch"`. If
  not provided, the fastest backend will be selected based on your environment, and based
  on the options supported by each backend. Typically, `"cuda"` will be used for
  inference, `"triton"` will be used for training, and `"pytorch"` will be used on
  non-CUDA devices.
- **scale_rule** (`str`, default "mse", *optional*, defaults to `"mse"`) --
  Rule to use when selecting block scales. Can be `"mse"`, `"mae"`, or `"abs_max"` for
  Four Over Six, `"static_6"` for default NVFP4 quantization, or `"static_4"` to scale
  all blocks to a maximum value of 4.
- **weight_dtype** (`str`, *optional*) --
  Data type to use when quantizing weight tensors. If not provided, `dtype` is used.
- **weight_scale_2d** (`bool`, default False, *optional*, defaults to `False`) --
  Whether to compute scale factors on weight tensors in 2D blocks. This should be done
  during training.
- **weight_scale_rule** (`str`, *optional*) --
  Scaling rule to use when selecting a scale for blocks in weight tensors. If not
  provided, `scale_rule` is used.
- **module_config_overrides** (`dict[str, dict[str, Any]]`, *optional*) --
  A dictionary of module-specific configuration overrides. Keys should be module names, and
  values should be dictionaries containing the quantization configuration for that module.
  This can be used to override the default configuration for specific modules.
- **modules_to_not_convert** (`list[str]`, *optional*, defaults to `['lm_head']`) --
  The list of modules to exclude from quantization. By default, the `lm_head` is excluded.

This is a wrapper class containing all options for quantization with `fouroversix`. In brief,
Four Over Six is a modification to NVFP4 quantization which adaptively scales the largest value
in each block of 16 FP4 values to either 4 or 6. Selecting a scale of 6 uses the full range of
FP4 values, but selecting a scale of 4 allows for a more uniform distribution of quantization
error. Refer to the original publication for more details: https://arxiv.org/abs/2512.02010.

## FPQuantConfig[[transformers.FPQuantConfig]]

- **forward_dtype** (`str`, *optional*, defaults to `"nvfp4"`) --
  The dtype to use for the forward pass.
- **forward_method** (`str`, *optional*, defaults to `"abs_max"`) --
  The scaling to use for the forward pass. Can be `"abs_max"` or `"quest"`. `"abs_max"` is better for PTQ, `"quest"` is better for QAT.
- **backward_dtype** (`str`, *optional*, defaults to `"bf16"`) --
  The dtype to use for the backward pass.
- **store_master_weights** (`bool`, *optional*, defaults to `False`) --
  Whether to store the master weights. Needed for QAT over layer weights.
- **hadamard_group_size** (`int`, *optional*) --
  The group size for the hadamard transform before quantization for `"quest"` it matches the MXFP4 group size (32). If `None`, it will be set to 16 for `"nvfp4"` and 32 for `"mxfp4"`.
- **pseudoquantization** (`bool`, *optional*, defaults to `False`) --
  Whether to use Triton-based pseudo-quantization. Is mandatory for non-Blackwell GPUs. Doesn't provide any speedup. For debugging purposes.
- **transform_init** (`str`, *optional*, defaults to `"hadamard"`) -- a method to initialize the pre-processing matrix with. Can be `"hadamard"`, `"identity"` or `"gsr"`.
- **modules_to_not_convert** (`list`, *optional*) --
  The list of modules to not quantize, useful for quantizing models that explicitly require to have
  some modules left in their original precision.

FPQuantConfig is a configuration class for quantization using the FPQuant method.

Safety checker that arguments are correct - also replaces some NoneType arguments with their default values.

## AutoRoundConfig[[transformers.AutoRoundConfig]]

- **bits** (`int`, *optional*, defaults to 4) --
  The number of bits to quantize to, supported numbers are (2, 3, 4, 8).
- **group_size** (`int`, *optional*, defaults to 128) -- Group-size value
- **sym** (`bool`, *optional*, defaults to `True`) -- Symmetric quantization or not
- **backend** (`str`, *optional*, defaults to `"auto"`) -- The kernel to use, e.g., ipex,marlin, exllamav2, triton, etc. Ref. https://github.com/intel/auto-round?tab=readme-ov-file#specify-backend
This is a wrapper class about all possible attributes and features that you can play with a model that has been
loaded AutoRound quantization.

Safety checker that arguments are correct.

## SinqConfig[[transformers.SinqConfig]]

- **nbits** (`int`, default 4) --
  Quantization bits for weights.
- **group_size** (`int`, default 64) --
  Group size used in SINQ weight quantization (must be multiple of 8).
- **tiling_mode** (`str`, default "1D") --
  Tiling mode for SINQ (typically "1D"; "2D" if supported in your backend).
- **method** (`str`, default "sinq") --
  "sinq"  – calibration-free weight-only SINQ
  "asinq" – A-SINQ (activation-aware), not supported in Hugging Face. Please refer to the official SINQ repository.
- **modules_to_not_convert** (`list[str]`, *optional*) --
  List of module names/prefixes to keep in full precision.

- ****kwargs** --
  Extra user arguments (kept in `_extra_kwargs` for round-tripping).

Quantization config for SINQ / A-SINQ.

Pass this to:

AutoModel.from_pretrained(..., quantization_config=SinqConfig(...))
