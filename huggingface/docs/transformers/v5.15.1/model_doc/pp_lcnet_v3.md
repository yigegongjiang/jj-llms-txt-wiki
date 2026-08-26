# PPLCNetV3

[PPLCNetV3](https://huggingface.co/papers/2109.15099) is a lightweight CPU-optimized convolutional backbone designed for efficient image classification and downstream vision tasks. It builds on the PP-LCNet architecture with improved training strategies and structural refinements for better accuracy-latency tradeoffs on CPU hardware.

## Notes

- PPLCNetV3 is provided as a backbone network only. No pre-trained image classification checkpoint has been officially released.

## PPLCNetV3Config[[transformers.PPLCNetV3Config]]

#### transformers.PPLCNetV3Config[[transformers.PPLCNetV3Config]]

```python
transformers.PPLCNetV3Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, scale: float | int = 1.0, block_configs: list | None = None, stem_channels: int = 16, stem_stride: int = 2, reduction: int = 4, divisor: int = 8, hidden_act: str = 'hardswish', _out_features: list[str] | None = None, _out_indices: list[int] | None = None, conv_symmetric_num: int = 4)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_lcnet_v3/configuration_pp_lcnet_v3.py#L31)

**Parameters:**

scale (`float`, *optional*, defaults to 1.0) : The scaling factor for the model's channel dimensions, used to adjust the model size and computational cost without changing the overall architecture (e.g., 0.25, 0.5, 1.0, 1.5).

block_configs (`list[list[tuple]]`, *optional*, defaults to `None`) : Configuration for each block in each stage. Each tuple contains: (kernel_size, in_channels, out_channels, stride, use_squeeze_excitation). If `None`, uses the default PP-LCNet configuration.

stem_channels (`int`, *optional*, defaults to 16) : The number of output channels for the stem layer.

stem_stride (`int`, *optional*, defaults to 2) : The stride for the stem convolution layer.

reduction (`int`, *optional*, defaults to 4) : The reduction factor for feature channel dimensions in the squeeze-and-excitation (SE) blocks, used to reduce the number of model parameters and computational complexity while maintaining feature representability.

divisor (`int`, *optional*, defaults to 8) : The divisor used to ensure that various model parameters (e.g., channel dimensions, kernel sizes) are multiples of this value, promoting efficient model implementation and resource utilization.

hidden_act (`str`, *optional*, defaults to `hardswish`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

conv_symmetric_num (`int`, *optional*, defaults to `4`) : The number of kxk convolution branches in the learnable reparameterization layer, used to enhance feature extraction capability through multi-branch architecture during training while enabling efficient inference via structural reparameterization.

This is the configuration class to store the configuration of a Pp Lcnet V3Model. It is used to instantiate a Pp Lcnet V3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/Not_yet_released](https://huggingface.co/PaddlePaddle/Not_yet_released)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## PPLCNetV3Backbone[[transformers.PPLCNetV3Backbone]]

#### transformers.PPLCNetV3Backbone[[transformers.PPLCNetV3Backbone]]

```python
transformers.PPLCNetV3Backbone(config: PPLCNetV3Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_lcnet_v3/modeling_pp_lcnet_v3.py#L326)

**Parameters:**

config ([PPLCNetV3Config](/docs/transformers/v5.15.1/en/model_doc/pp_lcnet_v3#transformers.PPLCNetV3Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

PPLCNetV3 backbone model for feature extraction.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.PPLCNetV3Backbone.forward]]

```python
forward(pixel_values: Tensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_lcnet_v3/modeling_pp_lcnet_v3.py#L339)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses `image_processor_class` for processing images).

**Returns:** `BackboneOutput` or `tuple(torch.FloatTensor)`

A `BackboneOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PPLCNetV3Config](/docs/transformers/v5.15.1/en/model_doc/pp_lcnet_v3#transformers.PPLCNetV3Config)) and inputs.

The [PPLCNetV3Backbone](/docs/transformers/v5.15.1/en/model_doc/pp_lcnet_v3#transformers.PPLCNetV3Backbone) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **feature_maps** (`tuple(torch.FloatTensor)` of shape `(batch_size, num_channels, height, width)`) -- Feature maps of the stages.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer) of
  shape `(batch_size, sequence_length, hidden_size)` or `(batch_size, num_channels, height, width)`,
  depending on the backbone.

  Hidden-states of the model at the output of each stage plus the initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`. Only applicable if the backbone uses attention.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import PPLCNetV3Config, PPLCNetV3Backbone
>>> import torch

>>> config = PPLCNetV3Config()
>>> model = PPLCNetV3Backbone(config)

>>> pixel_values = torch.randn(1, 3, 224, 224)

>>> with torch.no_grad():
...     outputs = model(pixel_values)

>>> feature_maps = outputs.feature_maps
>>> list(feature_maps[-1].shape)
```
