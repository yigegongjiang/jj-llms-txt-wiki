# Pixio

[Pixio]() is a vision foundation model that uses [ViT](./vit) as a feature extractor for multiple downstream tasks like depth estimation, semantic segmentation, feed-forward 3D reconstruction, robotics, and image classification. It is built on the Masked Autoencoder (MAE) pre-training framework, with four minimal yet critical updates: 1) deeper decoder, 2) larger masking granularity, 3) more class tokens, and 4) web-scale curated training data.

You can find all the original Pixio checkpoints under the [Pixio]() collection.

The example below demonstrates how to obtain an image embedding with the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) class.

```python
import requests
from PIL import Image

from transformers import AutoImageProcessor, AutoModel

url = "http://images.cocodataset.org/val2017/000000039769.jpg"
image = Image.open(requests.get(url, stream=True).raw)

processor = AutoImageProcessor.from_pretrained("facebook/pixio-vith16")
model = AutoModel.from_pretrained("facebook/pixio-vith16", device_map="auto")

inputs = processor(images=image, return_tensors="pt").to(model.device)
outputs = model(**inputs)
features_norm = outputs.last_hidden_state # class tokens + patch tokens after last LayerNorm
features = outputs.hidden_states[-1] # class tokens + patch tokens before last LayerNorm
```

## Notes

- The example below shows how to split the output tensor into:
  - a set of global embeddings for the whole image, commonly referred to as `CLS` token,
    useful for classification and retrieval.
    You can either average them (recommended) or concatenate them along the channel dimension.
  - a set of local embeddings, one for each `16x16` patch of the input image,
    useful for dense tasks, such as depth estimation and semantic segmentation.

  ```py
  from transformers import AutoImageProcessor, AutoModel
  from PIL import Image
  import requests

  url = 'http://images.cocodataset.org/val2017/000000039769.jpg'
  image = Image.open(requests.get(url, stream=True).raw)
  print(image.height, image.width)  # [480, 640]

  processor = AutoImageProcessor.from_pretrained('facebook/pixio-vith16')
  model = AutoModel.from_pretrained('facebook/pixio-vith16', device_map="auto")
  patch_size = model.config.patch_size

  inputs = processor(images=image, return_tensors="pt").to(model.device)
  print(inputs.pixel_values.shape)  # [1, 3, 256, 256]
  batch_size, rgb, img_height, img_width = inputs.pixel_values.shape
  num_patches_height, num_patches_width = img_height // patch_size, img_width // patch_size
  num_patches_flat = num_patches_height * num_patches_width

  outputs = model(**inputs)
  last_hidden_states = outputs.last_hidden_state
  print(last_hidden_states.shape)  # [1, 8 + 256, 1280]
  assert last_hidden_states.shape == (batch_size, model.config.n_cls_tokens + num_patches_flat, model.config.hidden_size)

  cls_tokens = last_hidden_states[:, :model.config.n_cls_tokens, :]
  patch_features = last_hidden_states[:, model.config.n_cls_tokens:, :].unflatten(1, (num_patches_height, num_patches_width))
  ```

- Use [torch.compile](https://pytorch.org/tutorials/intermediate/torch_compile_tutorial.html) to speedup inference.

  ```py
  import torch
  from transformers import AutoImageProcessor, AutoModel
  from PIL import Image
  import requests

  url = 'http://images.cocodataset.org/val2017/000000039769.jpg'
  image = Image.open(requests.get(url, stream=True).raw)

  processor = AutoImageProcessor.from_pretrained('facebook/pixio-vith16')
  model = AutoModel.from_pretrained('facebook/pixio-vith16', device_map="auto")

  compiled_model = torch.compile(model)

  inputs = processor(images=image, return_tensors="pt").to(model.device)
  outputs = compiled_model(**inputs)
  last_hidden_states = outputs.last_hidden_state
  ```

## PixioConfig[[transformers.PixioConfig]]

- **hidden_size** (`int`, *optional*, defaults to `1280`) --
  Dimension of the hidden representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `32`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **mlp_ratio** (`int`, *optional*, defaults to `4`) --
  Ratio of the MLP hidden dim to the embedding dim.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **hidden_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **attention_probs_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `256`) --
  The size (resolution) of each image.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) --
  The size (resolution) of each patch.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **qkv_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to add a bias to the queries, keys and values.
- **drop_path_rate** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  Drop path rate for the patch fusion.
- **apply_layernorm** (`bool`, *optional*, defaults to `True`) --
  Whether to apply layer normalization to the feature maps in case the model is used as backbone.
- **reshape_hidden_states** (`bool`, *optional*, defaults to `True`) --
  Whether to reshape the feature maps to 4D tensors of shape `(batch_size, hidden_size, height, width)` in
  case the model is used as backbone. If `False`, the feature maps will be 3D tensors of shape `(batch_size,
  seq_len, hidden_size)`.
- **n_cls_tokens** (`int`, *optional*, defaults to 8) --
  Number of class tokens in the Transformer encoder.

This is the configuration class to store the configuration of a PixioModel. It is used to instantiate a Pixio
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/pixio-huge](https://huggingface.co/facebook/pixio-huge)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import PixioConfig, PixioModel

>>> # Initializing a Pixio pixio-huge style configuration
>>> configuration = PixioConfig()

>>> # Initializing a model (with random weights) from the pixio-huge style configuration
>>> model = PixioModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## PixioModel[[transformers.PixioModel]]

- **config** ([PixioConfig](/docs/transformers/v5.14.0/en/model_doc/pixio#transformers.PixioConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Pixio Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [BitImageProcessor](/docs/transformers/v5.14.0/en/model_doc/bit#transformers.BitImageProcessor). See `BitImageProcessor.__call__()` for details (`processor_class` uses
  [BitImageProcessor](/docs/transformers/v5.14.0/en/model_doc/bit#transformers.BitImageProcessor) for processing images).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PixioConfig](/docs/transformers/v5.14.0/en/model_doc/pixio#transformers.PixioConfig)) and inputs.
The [PixioModel](/docs/transformers/v5.14.0/en/model_doc/pixio#transformers.PixioModel) forward method, overrides the `__call__` special method.

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

## PixioBackbone[[transformers.PixioBackbone]]

- **config** ([PixioConfig](/docs/transformers/v5.14.0/en/model_doc/pixio#transformers.PixioConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Pixio backbone, to be used with frameworks like DETR and MaskFormer.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [BitImageProcessor](/docs/transformers/v5.14.0/en/model_doc/bit#transformers.BitImageProcessor). See `BitImageProcessor.__call__()` for details (`processor_class` uses
  [BitImageProcessor](/docs/transformers/v5.14.0/en/model_doc/bit#transformers.BitImageProcessor) for processing images).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)`BackboneOutput` or `tuple(torch.FloatTensor)`A `BackboneOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PixioConfig](/docs/transformers/v5.14.0/en/model_doc/pixio#transformers.PixioConfig)) and inputs.
The [PixioBackbone](/docs/transformers/v5.14.0/en/model_doc/pixio#transformers.PixioBackbone) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoImageProcessor, AutoBackbone
>>> import torch
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> processor = AutoImageProcessor.from_pretrained("facebook/pixio-huge")
>>> model = AutoBackbone.from_pretrained(
...     "facebook/pixio-huge", out_features=["stage7", "stage15", "stage23", "stage31"]
... )

>>> inputs = processor(image, return_tensors="pt")

>>> outputs = model(**inputs)
>>> feature_maps = outputs.feature_maps
>>> list(feature_maps[-1].shape)
[1, 1280, 16, 16]
```
