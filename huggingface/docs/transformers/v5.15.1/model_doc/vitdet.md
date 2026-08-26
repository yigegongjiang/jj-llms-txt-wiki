# ViTDet

## Overview

The ViTDet model was proposed in [Exploring Plain Vision Transformer Backbones for Object Detection](https://huggingface.co/papers/2203.16527) by Yanghao Li, Hanzi Mao, Ross Girshick, Kaiming He.
VitDet leverages the plain [Vision Transformer](vit) for the task of object detection.

The abstract from the paper is the following:

*We explore the plain, non-hierarchical Vision Transformer (ViT) as a backbone network for object detection. This design enables the original ViT architecture to be fine-tuned for object detection without needing to redesign a hierarchical backbone for pre-training. With minimal adaptations for fine-tuning, our plain-backbone detector can achieve competitive results. Surprisingly, we observe: (i) it is sufficient to build a simple feature pyramid from a single-scale feature map (without the common FPN design) and (ii) it is sufficient to use window attention (without shifting) aided with very few cross-window propagation blocks. With plain ViT backbones pre-trained as Masked Autoencoders (MAE), our detector, named ViTDet, can compete with the previous leading methods that were all based on hierarchical backbones, reaching up to 61.3 AP_box on the COCO dataset using only ImageNet-1K pre-training. We hope our study will draw attention to research on plain-backbone detectors.*

This model was contributed by [nielsr](https://huggingface.co/nielsr).
The original code can be found [here](https://github.com/facebookresearch/detectron2/tree/main/projects/ViTDet).

Tips:

- At the moment, only the backbone is available.

## VitDetConfig[[transformers.VitDetConfig]]

#### transformers.VitDetConfig[[transformers.VitDetConfig]]

```python
transformers.VitDetConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 768, num_hidden_layers: int = 12, num_attention_heads: int = 12, mlp_ratio: int = 4, hidden_act: str = 'gelu', dropout_prob: float | int = 0.0, initializer_range: float = 0.02, layer_norm_eps: float = 1e-06, image_size: int | list[int] | tuple[int, int] = 224, pretrain_image_size: int | list[int] | tuple[int, int] = 224, patch_size: int | list[int] | tuple[int, int] = 16, num_channels: int = 3, qkv_bias: bool = True, drop_path_rate: float | int = 0.0, window_block_indices: list[int] | tuple[int, ...] = (), residual_block_indices: list[int] | tuple[int, ...] = (), use_absolute_position_embeddings: bool = True, use_relative_position_embeddings: bool = False, window_size: int = 0, _out_features: list[str] | None = None, _out_indices: list[int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/vitdet/configuration_vitdet.py#L25)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

num_hidden_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer decoder.

mlp_ratio (`int`, *optional*, defaults to `4`) : Ratio of the MLP hidden dim to the embedding dim.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The ratio for all dropout layers.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

image_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `224`) : The size (resolution) of each image.

pretrain_image_size (`int`, *optional*, defaults to 224) : The size (resolution) of each image during pretraining.

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) : The size (resolution) of each patch.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

qkv_bias (`bool`, *optional*, defaults to `True`) : Whether to add a bias to the queries, keys and values.

drop_path_rate (`Union[float, int]`, *optional*, defaults to `0.0`) : Drop path rate for the patch fusion.

window_block_indices (`list[int]`, *optional*, defaults to `[]`) : List of indices of blocks that should have window attention instead of regular global self-attention.

residual_block_indices (`list[int]`, *optional*, defaults to `[]`) : List of indices of blocks that should have an extra residual block after the MLP.

use_absolute_position_embeddings (`bool`, *optional*, defaults to `True`) : Whether to use absolute position embeddings.

use_relative_position_embeddings (`bool`, *optional*, defaults to `False`) : Whether to add relative position embeddings to the attention maps.

window_size (`int`, *optional*, defaults to 0) : The size of the attention window.

This is the configuration class to store the configuration of a VitDetModel. It is used to instantiate a Vitdet
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/vitdet-base-patch16-224](https://huggingface.co/google/vitdet-base-patch16-224)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import VitDetConfig, VitDetModel

>>> # Initializing a VitDet configuration
>>> configuration = VitDetConfig()

>>> # Initializing a model (with random weights) from the configuration
>>> model = VitDetModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## VitDetModel[[transformers.VitDetModel]]

#### transformers.VitDetModel[[transformers.VitDetModel]]

```python
transformers.VitDetModel(config: VitDetConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/vitdet/modeling_vitdet.py#L600)

**Parameters:**

config ([VitDetConfig](/docs/transformers/v5.15.1/en/model_doc/vitdet#transformers.VitDetConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Vitdet Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.VitDetModel.forward]]

```python
forward(pixel_values: typing.Optional[torch.Tensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/vitdet/modeling_vitdet.py#L614)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses `image_processor_class` for processing images).

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [BaseModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or `tuple(torch.FloatTensor)`

A [BaseModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([VitDetConfig](/docs/transformers/v5.15.1/en/model_doc/vitdet#transformers.VitDetConfig)) and inputs.

The [VitDetModel](/docs/transformers/v5.15.1/en/model_doc/vitdet#transformers.VitDetModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import VitDetConfig, VitDetModel
>>> import torch

>>> config = VitDetConfig()
>>> model = VitDetModel(config)

>>> pixel_values = torch.randn(1, 3, 224, 224)

>>> with torch.no_grad():
...     outputs = model(pixel_values)

>>> last_hidden_states = outputs.last_hidden_state
>>> list(last_hidden_states.shape)
[1, 768, 14, 14]
```
