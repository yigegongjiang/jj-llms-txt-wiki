# DINOv3

[DINOv3](https://huggingface.co/papers/2508.10104) is a family of versatile vision foundation models that outperforms the specialized state of the art across a broad range of settings, without fine-tuning. DINOv3 produces high-quality dense features that achieve outstanding performance on various vision tasks, significantly surpassing previous self- and weakly-supervised foundation models.

You can find all the original DINOv3 checkpoints under the [DINOv3](https://huggingface.co/collections/facebook/dinov3-68924841bd6b561778e31009) collection.

> [!TIP]
> Click on the DINOv3 models in the right sidebar for more examples of how to apply DINOv3 to different vision tasks.

The example below demonstrates how to obtain an image embedding with [Pipeline](/docs/transformers/v5.15.1/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline

pipe = pipeline(
    task="image-feature-extraction",
    model="facebook/dinov3-vits16-pretrain-lvd1689m",
)

pipe("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg")
```

```python
import torch

from transformers import AutoImageProcessor, AutoModel
from transformers.image_utils import load_image

url = "http://images.cocodataset.org/val2017/000000039769.jpg"
image = load_image(url)

processor = AutoImageProcessor.from_pretrained("facebook/dinov3-vits16-pretrain-lvd1689m")
model = AutoModel.from_pretrained(
    "facebook/dinov3-vits16-pretrain-lvd1689m",
    device_map="auto",
    attn_implementation="sdpa"
)

inputs = processor(images=image, return_tensors="pt").to(model.device)
with torch.inference_mode():
    outputs = model(**inputs)

pooled_output = outputs.pooler_output
print("Pooled output shape:", pooled_output.shape)
```

Quantization reduces the memory burden of large models by representing the weights in a lower precision. Refer to the [Quantization](../quantization/overview) overview for more available quantization backends.

The example below uses [torchao](../quantization/torchao) to only quantize the weights to int4.

```python
# pip install torchao
import torch
from torchao.quantization import Int4WeightOnlyConfig

from transformers import AutoImageProcessor, AutoModel, TorchAoConfig
from transformers.image_utils import load_image

url = "http://images.cocodataset.org/val2017/000000039769.jpg"
image = load_image(url)

processor = AutoImageProcessor.from_pretrained("facebook/dinov3-vitsplus-pretrain-lvd1689m")

quant_type = Int4WeightOnlyConfig(group_size=128)
quantization_config = TorchAoConfig(quant_type=quant_type)

model = AutoModel.from_pretrained(
    "facebook/dinov3-vit7b16-pretrain-lvd1689m",
    device_map="auto",
    quantization_config=quantization_config
)

inputs = processor(images=image, return_tensors="pt").to(model.device)
with torch.inference_mode():
    outputs = model(**inputs)

pooled_output = outputs.pooler_output
print("Pooled output shape:", pooled_output.shape)
```

## Notes

- The example below shows how to split the output tensor into:
  - one embedding for the whole image, commonly referred to as a `CLS` token,
    useful for classification and retrieval
  - register tokens - learnable embeddings that act as dedicated “memory slots” for global information,
    they reduce high-norm artifacts in patch tokens, yielding cleaner attention maps and better
    performance on dense prediction tasks.
  - a set of local embeddings, one for each `16x16` patch of the input image,
    useful for dense tasks, such as semantic segmentation

  ```py
  import torch
  from transformers import AutoImageProcessor, AutoModel
  from transformers.image_utils import load_image

  url = "http://images.cocodataset.org/val2017/000000039769.jpg"
  image = load_image(url)
  print("Image size:", image.height, image.width)  # [480, 640]

  processor = AutoImageProcessor.from_pretrained("facebook/dinov3-vits16-pretrain-lvd1689m")
  model = AutoModel.from_pretrained("facebook/dinov3-vits16-pretrain-lvd1689m", device_map="auto")
  patch_size = model.config.patch_size
  print("Patch size:", patch_size) # 16
  print("Num register tokens:", model.config.num_register_tokens) # 4

  inputs = processor(images=image, return_tensors="pt").to(model.device)
  print("Preprocessed image size:", inputs.pixel_values.shape)  # [1, 3, 224, 224]

  batch_size, _, img_height, img_width = inputs.pixel_values.shape
  num_patches_height, num_patches_width = img_height // patch_size, img_width // patch_size
  num_patches_flat = num_patches_height * num_patches_width

  with torch.inference_mode():
    outputs = model(**inputs)

  last_hidden_states = outputs.last_hidden_state
  print(last_hidden_states.shape)  # [1, 1 + 4 + 256, 384]
  assert last_hidden_states.shape == (batch_size, 1 + model.config.num_register_tokens + num_patches_flat, model.config.hidden_size)

  cls_token = last_hidden_states[:, 0, :]
  patch_features_flat = last_hidden_states[:, 1 + model.config.num_register_tokens:, :]
  patch_features = patch_features_flat.unflatten(1, (num_patches_height, num_patches_width))
  ```

## DINOv3ViTConfig[[transformers.DINOv3ViTConfig]]

#### transformers.DINOv3ViTConfig[[transformers.DINOv3ViTConfig]]

```python
transformers.DINOv3ViTConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, patch_size: int | list[int] | tuple[int, int] = 16, hidden_size: int = 384, intermediate_size: int = 1536, num_hidden_layers: int = 12, num_attention_heads: int = 6, hidden_act: str = 'gelu', attention_dropout: float | int = 0.0, initializer_range: float = 0.02, layer_norm_eps: float = 1e-05, rope_theta: float = 100.0, image_size: int | list[int] | tuple[int, int] = 224, num_channels: int = 3, query_bias: bool = True, key_bias: bool = False, value_bias: bool = True, proj_bias: bool = True, mlp_bias: bool = True, layerscale_value: float = 1.0, drop_path_rate: float | int = 0.0, use_gated_mlp: bool = False, num_register_tokens: int = 0, pos_embed_shift: float | None = None, pos_embed_jitter: float | None = None, pos_embed_rescale: float | None = 2.0, _out_features: list[str] | None = None, _out_indices: list[int] | None = None, apply_layernorm: bool = True, reshape_hidden_states: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dinov3_vit/configuration_dinov3_vit.py#L25)

**Parameters:**

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) : The size (resolution) of each patch.

hidden_size (`int`, *optional*, defaults to `384`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `1536`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `6`) : Number of attention heads for each attention layer in the Transformer decoder.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

rope_theta (`float`, *optional*, defaults to 100.0) : The base period of the RoPE embeddings.

image_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `224`) : The size (resolution) of each image.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

query_bias (`bool`, *optional*, defaults to `True`) : Whether to add a bias to the query projection.

key_bias (`bool`, *optional*, defaults to `False`) : Whether to add a bias to the key projection.

value_bias (`bool`, *optional*, defaults to `True`) : Whether to add a bias to the value projection.

proj_bias (`bool`, *optional*, defaults to `True`) : Whether to add a bias to the output projection.

mlp_bias (`bool`, *optional*, defaults to `True`) : Whether to use a bias in up_proj, down_proj and gate_proj layers in the MLP layers.

layerscale_value (`float`, *optional*, defaults to 1.0) : Initial value to use for layer scale.

drop_path_rate (`Union[float, int]`, *optional*, defaults to `0.0`) : Drop path rate for the patch fusion.

use_gated_mlp (`bool`, *optional*, defaults to `False`) : Whether to use the SwiGLU feedforward neural network.

num_register_tokens (`int`, *optional*, defaults to 0) : The number of register tokens.

pos_embed_shift (`float`, *optional*) : Amount to randomly shift position embedding coordinates in [-shift, shift], applied only in training mode if not `None`.

pos_embed_jitter (`float`, *optional*) : Amount to randomly jitter position embedding coordinates in log-uniform value in [1/jitter, jitter], applied only in training mode if not `None`.

pos_embed_rescale (`float`, *optional*, defaults to 2.0) : Amount to randomly rescale position embedding coordinates in log-uniform value in [1/rescale, rescale], applied only in training mode if not `None`.

apply_layernorm (`bool`, *optional*, defaults to `True`) : Whether to apply layer normalization to the feature maps when used as backbone.

reshape_hidden_states (`bool`, *optional*, defaults to `True`) : Whether to reshape the hidden states to spatial dimensions when used as backbone.

This is the configuration class to store the configuration of a DINOv3ViTModel. It is used to instantiate a Dinov3 Vit
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/dinov3-vits16-pretrain-lvd1689m](https://huggingface.co/facebook/dinov3-vits16-pretrain-lvd1689m)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import DINOv3ViTConfig, DINOv3ViTModel

>>> # Initializing a DINOv3 ViT-small style configuration
>>> config = DINOv3ViTConfig()

>>> # Initializing a model (with random weights) from the config
>>> model = DINOv3ViTModel(config)

>>> # Accessing the model config
>>> config = model.config
```

## DINOv3ConvNextConfig[[transformers.DINOv3ConvNextConfig]]

#### transformers.DINOv3ConvNextConfig[[transformers.DINOv3ConvNextConfig]]

```python
transformers.DINOv3ConvNextConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, num_channels: int = 3, hidden_sizes: list[int] | None = None, depths: list[int] | None = None, hidden_act: str = 'gelu', initializer_range: float = 0.02, layer_norm_eps: float = 1e-06, layer_scale_init_value: float = 1e-06, drop_path_rate: float | int = 0.0, image_size: int | list[int] | tuple[int, int] = 224, _out_features: list[str] | None = None, _out_indices: list[int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dinov3_convnext/configuration_dinov3_convnext.py#L25)

**Parameters:**

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

hidden_sizes (`list[int]`, *optional*) : Dimensionality (hidden size) at each stage of the model.

depths (`list[int]`, *optional*) : Depth of each layer in the Transformer.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

layer_scale_init_value (`float`, *optional*, defaults to `1e-06`) : Scale to use in the self-attention layers. 0.1 for base, 1e-6 for large. Set 0 to disable layer scale.

drop_path_rate (`Union[float, int]`, *optional*, defaults to `0.0`) : Drop path rate for the patch fusion.

image_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `224`) : The size (resolution) of each image.

This is the configuration class to store the configuration of a DINOv3ConvNextModel. It is used to instantiate a Dinov3 Convnext
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/dinov3-convnext-tiny-pretrain-lvd1689m](https://huggingface.co/facebook/dinov3-convnext-tiny-pretrain-lvd1689m)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import DINOv3ConvNextConfig, DINOv3ConvNextModel

>>> # Initializing a DINOv3ConvNext (tiny variant) style configuration
>>> config = DINOv3ConvNextConfig()

>>> # Initializing a model (with random weights)
>>> model = DINOv3ConvNextModel(config)

>>> # Accessing the model config
>>> config = model.config
```

## DINOv3ViTModel[[transformers.DINOv3ViTModel]]

#### transformers.DINOv3ViTModel[[transformers.DINOv3ViTModel]]

```python
transformers.DINOv3ViTModel(config: DINOv3ViTConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dinov3_vit/modeling_dinov3_vit.py#L507)

**Parameters:**

config ([DINOv3ViTConfig](/docs/transformers/v5.15.1/en/model_doc/dinov3#transformers.DINOv3ViTConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Dinov3 Vit Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.DINOv3ViTModel.forward]]

```python
forward(pixel_values: Tensor, bool_masked_pos: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dinov3_vit/modeling_dinov3_vit.py#L521)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [DINOv3ViTImageProcessor](/docs/transformers/v5.15.1/en/model_doc/dinov3#transformers.DINOv3ViTImageProcessor). See `DINOv3ViTImageProcessor.__call__()` for details (`processor_class` uses [DINOv3ViTImageProcessor](/docs/transformers/v5.15.1/en/model_doc/dinov3#transformers.DINOv3ViTImageProcessor) for processing images).

bool_masked_pos (`torch.BoolTensor` of shape `(batch_size, sequence_length)`) : Boolean masked positions. Indicates which patches are masked (1) and which aren't (0). Only relevant for pre-training.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DINOv3ViTConfig](/docs/transformers/v5.15.1/en/model_doc/dinov3#transformers.DINOv3ViTConfig)) and inputs.

The [DINOv3ViTModel](/docs/transformers/v5.15.1/en/model_doc/dinov3#transformers.DINOv3ViTModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Last layer hidden-state of the first token of the sequence (classification token) after further processing
  through the layers used for the auxiliary pretraining task. E.g. for BERT-family of models, this returns
  the classification token after processing through a linear layer and a tanh activation function. The linear
  layer weights are trained from the next sentence prediction (classification) objective during pretraining.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
```

## DINOv3ViTBackbone[[transformers.DINOv3ViTBackbone]]

#### transformers.DINOv3ViTBackbone[[transformers.DINOv3ViTBackbone]]

```python
transformers.DINOv3ViTBackbone(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dinov3_vit/modeling_dinov3_vit.py#L552)

**Parameters:**

config ([DINOv3ViTBackbone](/docs/transformers/v5.15.1/en/model_doc/dinov3#transformers.DINOv3ViTBackbone)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Dinov3 Vit backbone.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.DINOv3ViTBackbone.forward]]

```python
forward(pixel_values: Tensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dinov3_vit/modeling_dinov3_vit.py#L568)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [DINOv3ViTImageProcessor](/docs/transformers/v5.15.1/en/model_doc/dinov3#transformers.DINOv3ViTImageProcessor). See `DINOv3ViTImageProcessor.__call__()` for details (`processor_class` uses [DINOv3ViTImageProcessor](/docs/transformers/v5.15.1/en/model_doc/dinov3#transformers.DINOv3ViTImageProcessor) for processing images).

**Returns:** `DINOv3ViTBackboneOutput` or `tuple(torch.FloatTensor)`

A `DINOv3ViTBackboneOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DINOv3ViTConfig](/docs/transformers/v5.15.1/en/model_doc/dinov3#transformers.DINOv3ViTConfig)) and inputs.

The [DINOv3ViTBackbone](/docs/transformers/v5.15.1/en/model_doc/dinov3#transformers.DINOv3ViTBackbone) forward method, overrides the `__call__` special method.

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
- **cls_tokens** (`tuple(torch.FloatTensor)`, *optional*) -- CLS token from each selected feature stage, each of shape `(batch_size, hidden_size)`.
  Only present when `config.return_class_token=True`.

## DINOv3ConvNextModel[[transformers.DINOv3ConvNextModel]]

#### transformers.DINOv3ConvNextModel[[transformers.DINOv3ConvNextModel]]

```python
transformers.DINOv3ConvNextModel(config: DINOv3ConvNextConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dinov3_convnext/modeling_dinov3_convnext.py#L217)

**Parameters:**

config ([DINOv3ConvNextConfig](/docs/transformers/v5.15.1/en/model_doc/dinov3#transformers.DINOv3ConvNextConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Dinov3 Convnext Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.DINOv3ConvNextModel.forward]]

```python
forward(pixel_values: FloatTensor, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dinov3_convnext/modeling_dinov3_convnext.py#L225)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses `image_processor_class` for processing images).

**Returns:** `BaseModelOutputWithPoolingAndNoAttention` or `tuple(torch.FloatTensor)`

A `BaseModelOutputWithPoolingAndNoAttention` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DINOv3ConvNextConfig](/docs/transformers/v5.15.1/en/model_doc/dinov3#transformers.DINOv3ConvNextConfig)) and inputs.

The [DINOv3ConvNextModel](/docs/transformers/v5.15.1/en/model_doc/dinov3#transformers.DINOv3ConvNextModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Last layer hidden-state after a pooling operation on the spatial dimensions.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, num_channels, height, width)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

Example:

```python
```

## DINOv3ViTImageProcessor[[transformers.DINOv3ViTImageProcessor]]

#### transformers.DINOv3ViTImageProcessor[[transformers.DINOv3ViTImageProcessor]]

```python
transformers.DINOv3ViTImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dinov3_vit/image_processing_dinov3_vit.py#L34)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 224, 'width': 224}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BILINEAR`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.485, 0.456, 0.406]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.229, 0.224, 0.225]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models. Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models. Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models. Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models. Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models. Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models. Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors. --

Constructs a DINOv3ViTImageProcessor image processor.

disable_grouping (`bool`, *kwargs*, *optional*):
Whether to disable grouping of images by size to process them individually and not in batches.
If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on
empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157
image_seq_length (`int`, *kwargs*, *optional*):
The number of image tokens to be used for each image in the input.
Added for backward compatibility but this should be set as a processor attribute in future models.

#### preprocess[[transformers.DINOv3ViTImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], *args, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_utils.py#L382)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## DINOv3ConvNextBackbone[[transformers.DINOv3ConvNextBackbone]]

#### transformers.DINOv3ConvNextBackbone[[transformers.DINOv3ConvNextBackbone]]

```python
transformers.DINOv3ConvNextBackbone(config: DINOv3ConvNextConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dinov3_convnext/modeling_dinov3_convnext.py#L252)

**Parameters:**

config ([DINOv3ConvNextConfig](/docs/transformers/v5.15.1/en/model_doc/dinov3#transformers.DINOv3ConvNextConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Dinov3 Convnext backbone.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.DINOv3ConvNextBackbone.forward]]

```python
forward(pixel_values: FloatTensor, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dinov3_convnext/modeling_dinov3_convnext.py#L267)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses `image_processor_class` for processing images).

**Returns:** `BackboneOutput` or `tuple(torch.FloatTensor)`

A `BackboneOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DINOv3ConvNextConfig](/docs/transformers/v5.15.1/en/model_doc/dinov3#transformers.DINOv3ConvNextConfig)) and inputs.

The [DINOv3ConvNextBackbone](/docs/transformers/v5.15.1/en/model_doc/dinov3#transformers.DINOv3ConvNextBackbone) forward method, overrides the `__call__` special method.

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
