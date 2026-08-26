# Swin Transformer V2

[Swin Transformer V2](https://huggingface.co/papers/2111.09883) is a 3B parameter model that focuses on how to scale a vision model to billions of parameters. It introduces techniques like residual-post-norm combined with cosine attention for improved training stability, log-spaced continuous position bias to better handle varying image resolutions between pre-training and fine-tuning, and a new pre-training method (SimMIM) to reduce the need for large amounts of labeled data. These improvements enable efficiently training very large models (up to 3 billion parameters) capable of processing high-resolution images.

You can find official Swin Transformer V2 checkpoints under the [Microsoft](https://huggingface.co/microsoft?search_models=swinv2) organization.

> [!TIP]
> Click on the Swin Transformer V2 models in the right sidebar for more examples of how to apply Swin Transformer V2 to vision tasks.

```python
from transformers import pipeline

pipeline = pipeline(
    task="image-classification",
    model="microsoft/swinv2-tiny-patch4-window8-256",
    device=0
)
pipeline("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg")
```

```python
import requests
import torch
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForImageClassification

image_processor = AutoImageProcessor.from_pretrained(
    "microsoft/swinv2-tiny-patch4-window8-256",
)
model = AutoModelForImageClassification.from_pretrained(
    "microsoft/swinv2-tiny-patch4-window8-256",
    device_map="auto"
)

url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
image = Image.open(requests.get(url, stream=True).raw)
inputs = image_processor(image, return_tensors="pt").to(model.device)

with torch.no_grad():
  logits = model(**inputs).logits

predicted_class_id = logits.argmax(dim=-1).item()
predicted_class_label = model.config.id2label[predicted_class_id]
print(f"The predicted class label is: {predicted_class_label}")
```

## Notes

- Swin Transformer V2 can pad the inputs for any input height and width divisible by `32`.
- Swin Transformer V2 can be used as a [backbone](../backbones). When `output_hidden_states = True`, it outputs both `hidden_states` and `reshaped_hidden_states`. The `reshaped_hidden_states` have a shape of `(batch, num_channels, height, width)` rather than `(batch_size, sequence_length, num_channels)`.

## Swinv2Config[[transformers.Swinv2Config]]

#### transformers.Swinv2Config[[transformers.Swinv2Config]]

```python
transformers.Swinv2Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, image_size: int | list[int] | tuple[int, int] = 224, patch_size: int | list[int] | tuple[int, int] = 4, num_channels: int = 3, embed_dim: int = 96, depths: list[int] | tuple[int, ...] = (2, 2, 6, 2), num_heads: list[int] | tuple[int, ...] = (3, 6, 12, 24), window_size: int = 7, pretrained_window_sizes: list[int] | tuple[int, ...] = (0, 0, 0, 0), mlp_ratio: float = 4.0, qkv_bias: bool = True, hidden_dropout_prob: float | int = 0.0, attention_probs_dropout_prob: float | int = 0.0, drop_path_rate: float | int = 0.1, hidden_act: str = 'gelu', use_absolute_embeddings: bool = False, initializer_range: float = 0.02, layer_norm_eps: float = 1e-05, encoder_stride: int = 32, _out_features: list[str] | None = None, _out_indices: list[int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/swinv2/configuration_swinv2.py#L25)

**Parameters:**

image_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `224`) : The size (resolution) of each image.

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `4`) : The size (resolution) of each patch.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

embed_dim (`int`, *optional*, defaults to `96`) : Dimensionality of the embeddings and hidden states.

depths (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(2, 2, 6, 2)`) : Depth of each layer in the Transformer.

num_heads (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(3, 6, 12, 24)`) : Number of attention heads for each attention layer in the Transformer decoder.

window_size (`int`, *optional*, defaults to 7) : Size of windows.

pretrained_window_sizes (`list(int)`, *optional*, defaults to `[0, 0, 0, 0]`) : Size of windows during pretraining.

mlp_ratio (`float`, *optional*, defaults to `4.0`) : Ratio of the MLP hidden dim to the embedding dim.

qkv_bias (`bool`, *optional*, defaults to `True`) : Whether to add a bias to the queries, keys and values.

hidden_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

attention_probs_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

drop_path_rate (`Union[float, int]`, *optional*, defaults to `0.1`) : Drop path rate for the patch fusion.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

use_absolute_embeddings (`bool`, *optional*, defaults to `False`) : Whether to use absolute position embeddings.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

encoder_stride (`int`, *optional*, defaults to 32) : Factor to increase the spatial resolution by in the decoder head for masked image modeling.

This is the configuration class to store the configuration of a Swinv2Model. It is used to instantiate a Swinv2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [microsoft/swinv2-tiny-patch4-window8-256](https://huggingface.co/microsoft/swinv2-tiny-patch4-window8-256)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Swinv2Config, Swinv2Model

>>> # Initializing a Swinv2 microsoft/swinv2-tiny-patch4-window8-256 style configuration
>>> configuration = Swinv2Config()

>>> # Initializing a model (with random weights) from the microsoft/swinv2-tiny-patch4-window8-256 style configuration
>>> model = Swinv2Model(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Swinv2Model[[transformers.Swinv2Model]]

#### transformers.Swinv2Model[[transformers.Swinv2Model]]

```python
transformers.Swinv2Model(config, add_pooling_layer = True, use_mask_token = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/swinv2/modeling_swinv2.py#L891)

**Parameters:**

config ([Swinv2Model](/docs/transformers/v5.15.1/en/model_doc/swinv2#transformers.Swinv2Model)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

add_pooling_layer (`bool`, *optional*, defaults to `True`) : Whether or not to apply pooling layer.

use_mask_token (`bool`, *optional*, defaults to `False`) : Whether or not to create and apply mask tokens in the embedding layer.

The bare Swinv2 Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Swinv2Model.forward]]

```python
forward(pixel_values: typing.Optional[torch.FloatTensor] = None, bool_masked_pos: typing.Optional[torch.BoolTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, interpolate_pos_encoding: bool = False, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/swinv2/modeling_swinv2.py#L916)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [ViTImageProcessor](/docs/transformers/v5.15.1/en/model_doc/vit#transformers.ViTImageProcessor). See `ViTImageProcessor.__call__()` for details (`processor_class` uses [ViTImageProcessor](/docs/transformers/v5.15.1/en/model_doc/vit#transformers.ViTImageProcessor) for processing images).

bool_masked_pos (`torch.BoolTensor` of shape `(batch_size, num_patches)`, *optional*) : Boolean masked positions. Indicates which patches are masked (1) and which aren't (0).

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

interpolate_pos_encoding (`bool`, *optional*, defaults to `False`) : Whether to interpolate the pre-trained position encodings.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `Swinv2ModelOutput` or `tuple(torch.FloatTensor)`

A `Swinv2ModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Swinv2Config](/docs/transformers/v5.15.1/en/model_doc/swinv2#transformers.Swinv2Config)) and inputs.

The [Swinv2Model](/docs/transformers/v5.15.1/en/model_doc/swinv2#transformers.Swinv2Model) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`, *optional*, returned when `add_pooling_layer=True` is passed) -- Average pooling of the last layer hidden-state.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **reshaped_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, hidden_size, height, width)`.

  Hidden-states of the model at the output of each layer plus the initial embedding outputs reshaped to
  include the spatial dimensions.

Example:

```python
```

## Swinv2ForMaskedImageModeling[[transformers.Swinv2ForMaskedImageModeling]]

#### transformers.Swinv2ForMaskedImageModeling[[transformers.Swinv2ForMaskedImageModeling]]

```python
transformers.Swinv2ForMaskedImageModeling(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/swinv2/modeling_swinv2.py#L988)

**Parameters:**

config ([Swinv2ForMaskedImageModeling](/docs/transformers/v5.15.1/en/model_doc/swinv2#transformers.Swinv2ForMaskedImageModeling)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Swinv2 Model with a decoder on top for masked image modeling, as proposed in
[SimMIM](https://huggingface.co/papers/2111.09886).

Note that we provide a script to pre-train this model on custom data in our [examples
directory](https://github.com/huggingface/transformers/tree/main/examples/pytorch/image-pretraining).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Swinv2ForMaskedImageModeling.forward]]

```python
forward(pixel_values: typing.Optional[torch.FloatTensor] = None, bool_masked_pos: typing.Optional[torch.BoolTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, interpolate_pos_encoding: bool = False, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/swinv2/modeling_swinv2.py#L1005)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [ViTImageProcessor](/docs/transformers/v5.15.1/en/model_doc/vit#transformers.ViTImageProcessor). See `ViTImageProcessor.__call__()` for details (`processor_class` uses [ViTImageProcessor](/docs/transformers/v5.15.1/en/model_doc/vit#transformers.ViTImageProcessor) for processing images).

bool_masked_pos (`torch.BoolTensor` of shape `(batch_size, num_patches)`) : Boolean masked positions. Indicates which patches are masked (1) and which aren't (0).

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

interpolate_pos_encoding (`bool`, *optional*, defaults to `False`) : Whether to interpolate the pre-trained position encodings.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `Swinv2MaskedImageModelingOutput` or `tuple(torch.FloatTensor)`

A `Swinv2MaskedImageModelingOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Swinv2Config](/docs/transformers/v5.15.1/en/model_doc/swinv2#transformers.Swinv2Config)) and inputs.

The [Swinv2ForMaskedImageModeling](/docs/transformers/v5.15.1/en/model_doc/swinv2#transformers.Swinv2ForMaskedImageModeling) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `bool_masked_pos` is provided) -- Masked image modeling (MLM) loss.
- **reconstruction** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Reconstructed pixel values.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **reshaped_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, hidden_size, height, width)`.

  Hidden-states of the model at the output of each layer plus the initial embedding outputs reshaped to
  include the spatial dimensions.

Examples:
```python
>>> from transformers import AutoImageProcessor, Swinv2ForMaskedImageModeling
>>> import torch
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("microsoft/swinv2-tiny-patch4-window8-256")
>>> model = Swinv2ForMaskedImageModeling.from_pretrained("microsoft/swinv2-tiny-patch4-window8-256")

>>> num_patches = (model.config.image_size // model.config.patch_size) ** 2
>>> pixel_values = image_processor(images=image, return_tensors="pt").pixel_values
>>> # create random boolean mask of shape (batch_size, num_patches)
>>> bool_masked_pos = torch.randint(low=0, high=2, size=(1, num_patches)).bool()

>>> outputs = model(pixel_values, bool_masked_pos=bool_masked_pos)
>>> loss, reconstructed_pixel_values = outputs.loss, outputs.reconstruction
>>> list(reconstructed_pixel_values.shape)
[1, 3, 256, 256]
```

## Swinv2ForImageClassification[[transformers.Swinv2ForImageClassification]]

#### transformers.Swinv2ForImageClassification[[transformers.Swinv2ForImageClassification]]

```python
transformers.Swinv2ForImageClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/swinv2/modeling_swinv2.py#L1107)

**Parameters:**

config ([Swinv2ForImageClassification](/docs/transformers/v5.15.1/en/model_doc/swinv2#transformers.Swinv2ForImageClassification)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Swinv2 Model transformer with an image classification head on top (a linear layer on top of the final hidden state
of the [CLS] token) e.g. for ImageNet.

Note that it's possible to fine-tune SwinV2 on higher resolution images than the ones it has been trained on, by
setting `interpolate_pos_encoding` to `True` in the forward of the model. This will interpolate the pre-trained
position embeddings to the higher resolution.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Swinv2ForImageClassification.forward]]

```python
forward(pixel_values: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, interpolate_pos_encoding: bool = False, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/swinv2/modeling_swinv2.py#L1122)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [ViTImageProcessor](/docs/transformers/v5.15.1/en/model_doc/vit#transformers.ViTImageProcessor). See `ViTImageProcessor.__call__()` for details (`processor_class` uses [ViTImageProcessor](/docs/transformers/v5.15.1/en/model_doc/vit#transformers.ViTImageProcessor) for processing images).

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the image classification/regression loss. Indices should be in `[0, ..., config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If `config.num_labels > 1` a classification loss is computed (Cross-Entropy).

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

interpolate_pos_encoding (`bool`, *optional*, defaults to `False`) : Whether to interpolate the pre-trained position encodings.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `Swinv2ImageClassifierOutput` or `tuple(torch.FloatTensor)`

A `Swinv2ImageClassifierOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Swinv2Config](/docs/transformers/v5.15.1/en/model_doc/swinv2#transformers.Swinv2Config)) and inputs.

The [Swinv2ForImageClassification](/docs/transformers/v5.15.1/en/model_doc/swinv2#transformers.Swinv2ForImageClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Classification (or regression if config.num_labels==1) scores (before SoftMax).
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **reshaped_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, hidden_size, height, width)`.

  Hidden-states of the model at the output of each layer plus the initial embedding outputs reshaped to
  include the spatial dimensions.

Example:

```python
>>> from transformers import AutoImageProcessor, Swinv2ForImageClassification
>>> import torch
>>> from datasets import load_dataset

>>> dataset = load_dataset("huggingface/cats-image")
>>> image = dataset["test"]["image"][0]

>>> image_processor = AutoImageProcessor.from_pretrained("microsoft/swinv2-tiny-patch4-window8-256")
>>> model = Swinv2ForImageClassification.from_pretrained("microsoft/swinv2-tiny-patch4-window8-256")

>>> inputs = image_processor(image, return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # model predicts one of the 1000 ImageNet classes
>>> predicted_label = logits.argmax(-1).item()
>>> print(model.config.id2label[predicted_label])
...
```
