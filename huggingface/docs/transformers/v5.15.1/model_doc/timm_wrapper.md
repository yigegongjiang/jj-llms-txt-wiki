# TimmWrapper

## Overview

Helper class to enable loading timm models to be used with the transformers library and its autoclasses.

```python
from urllib.request import urlopen

import torch
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForImageClassification

# Load image
image = Image.open(urlopen(
    'https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/beignets-task-guide.png'
))

# Load model and image processor
checkpoint = "timm/resnet50.a1_in1k"
image_processor = AutoImageProcessor.from_pretrained(checkpoint)
model = AutoModelForImageClassification.from_pretrained(checkpoint).eval( device_map="auto")

# Preprocess image
inputs = image_processor(image)

# Forward pass
with torch.no_grad():
    logits = model(**inputs).logits

# Get top 5 predictions
top5_probabilities, top5_class_indices = torch.topk(logits.softmax(dim=1) * 100, k=5)
```

## Resources

A list of official Hugging Face and community (indicated by 🌎) resources to help you get started with TimmWrapper.

- [Collection of Example Notebook](https://github.com/ariG23498/timm-wrapper-examples) 🌎

> [!TIP]
> For a more detailed overview please read the [official blog post](https://huggingface.co/blog/timm-transformers) on the timm integration.

## TimmWrapperConfig[[transformers.TimmWrapperConfig]]

#### transformers.TimmWrapperConfig[[transformers.TimmWrapperConfig]]

```python
transformers.TimmWrapperConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, architecture: str = 'resnet50', initializer_range: float = 0.02, do_pooling: bool = True, model_args: dict[str, typing.Any] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/timm_wrapper/configuration_timm_wrapper.py#L31)

**Parameters:**

architecture (`str`, *optional*, defaults to `"resnet50"`) : The timm architecture to load.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

do_pooling (`bool`, *optional*, defaults to `True`) : Whether to do pooling for the last_hidden_state in `TimmWrapperModel` or not.

model_args (`dict[str, Any]`, *optional*) : Additional keyword arguments to pass to the `timm.create_model` function. e.g. `model_args={"depth": 3}` for `timm/vit_base_patch32_clip_448.laion2b_ft_in12k_in1k` to create a model with 3 blocks. Defaults to `None`.

This is the configuration class to store the configuration of a TimmWrapperModel. It is used to instantiate a Timm Wrapper
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [resnet50](https://huggingface.co/resnet50)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import TimmWrapperModel

>>> # Initializing a timm model
>>> model = TimmWrapperModel.from_pretrained("timm/resnet18.a1_in1k")

>>> # Accessing the model configuration
>>> configuration = model.config
```

## TimmWrapperImageProcessor[[transformers.TimmWrapperImageProcessor]]

#### transformers.TimmWrapperImageProcessor[[transformers.TimmWrapperImageProcessor]]

```python
transformers.TimmWrapperImageProcessor(pretrained_cfg: dict, architecture: str | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/timm_wrapper/image_processing_timm_wrapper.py#L38)

**Parameters:**

pretrained_cfg (`dict[str, Any]`) : The configuration of the pretrained model used to resolve evaluation and training transforms.

architecture (`Optional[str]`, *optional*) : Name of the architecture of the model.

Wrapper class for timm models to be used within transformers.

#### preprocess[[transformers.TimmWrapperImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], return_tensors: str | transformers.utils.generic.TensorType | None = 'pt')
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/timm_wrapper/image_processing_timm_wrapper.py#L96)

**Parameters:**

images (`ImageInput`) : Image to preprocess. Expects a single or batch of images

return_tensors (`str` or `TensorType`, *optional*) : The type of tensors to return.

Preprocess an image or batch of images.

## TimmWrapperModel[[transformers.TimmWrapperModel]]

#### transformers.TimmWrapperModel[[transformers.TimmWrapperModel]]

```python
transformers.TimmWrapperModel(config: TimmWrapperConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/timm_wrapper/modeling_timm_wrapper.py#L151)

Wrapper class for timm models to be used in transformers.

#### forward[[transformers.TimmWrapperModel.forward]]

```python
forward(pixel_values: FloatTensor, output_attentions: bool | None = None, output_hidden_states: bool | list[int] | None = None, return_dict: bool | None = None, do_pooling: bool | None = None, use_cache: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/timm_wrapper/modeling_timm_wrapper.py#L165)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [TimmWrapperImageProcessor](/docs/transformers/v5.15.1/en/model_doc/timm_wrapper#transformers.TimmWrapperImageProcessor). See `TimmWrapperImageProcessor.__call__()` for details (`processor_class` uses [TimmWrapperImageProcessor](/docs/transformers/v5.15.1/en/model_doc/timm_wrapper#transformers.TimmWrapperImageProcessor) for processing images).

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. Not compatible with timm wrapped models.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. Not compatible with timm wrapped models.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

do_pooling (`bool`, *optional*) : Whether to do pooling for the last_hidden_state in `TimmWrapperModel` or not. If `None` is passed, the `do_pooling` value from the config is used.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** `TimmWrapperModelOutput` or `tuple(torch.FloatTensor)`

A `TimmWrapperModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([TimmWrapperConfig](/docs/transformers/v5.15.1/en/model_doc/timm_wrapper#transformers.TimmWrapperConfig)) and inputs.

The [TimmWrapperModel](/docs/transformers/v5.15.1/en/model_doc/timm_wrapper#transformers.TimmWrapperModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor`) -- The last hidden state of the model, output before applying the classification head.
- **pooler_output** (`torch.FloatTensor`, *optional*) -- The pooled output derived from the last hidden state, if applicable.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned if `output_hidden_states=True` is set or if `config.output_hidden_states=True`) -- A tuple containing the intermediate hidden states of the model at the output of each layer or specified layers.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned if `output_attentions=True` is set or if `config.output_attentions=True`.) -- A tuple containing the intermediate attention weights of the model at the output of each layer.
  Note: Currently, Timm models do not support attentions output.

Examples:
```python
>>> import torch
>>> from PIL import Image
>>> from urllib.request import urlopen
>>> from transformers import AutoModel, AutoImageProcessor

>>> # Load image
>>> image = Image.open(urlopen(
...     'https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/beignets-task-guide.png'
... ))

>>> # Load model and image processor
>>> checkpoint = "timm/resnet50.a1_in1k"
>>> image_processor = AutoImageProcessor.from_pretrained(checkpoint)
>>> model = AutoModel.from_pretrained(checkpoint).eval()

>>> # Preprocess image
>>> inputs = image_processor(image)

>>> # Forward pass
>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> # Get pooled output
>>> pooled_output = outputs.pooler_output

>>> # Get last hidden state
>>> last_hidden_state = outputs.last_hidden_state
```

## TimmWrapperForImageClassification[[transformers.TimmWrapperForImageClassification]]

#### transformers.TimmWrapperForImageClassification[[transformers.TimmWrapperForImageClassification]]

```python
transformers.TimmWrapperForImageClassification(config: TimmWrapperConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/timm_wrapper/modeling_timm_wrapper.py#L268)

Wrapper class for timm models to be used in transformers for image classification.

#### forward[[transformers.TimmWrapperForImageClassification.forward]]

```python
forward(pixel_values: FloatTensor, labels: typing.Optional[torch.LongTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | list[int] | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/timm_wrapper/modeling_timm_wrapper.py#L291)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [TimmWrapperImageProcessor](/docs/transformers/v5.15.1/en/model_doc/timm_wrapper#transformers.TimmWrapperImageProcessor). See `TimmWrapperImageProcessor.__call__()` for details (`processor_class` uses [TimmWrapperImageProcessor](/docs/transformers/v5.15.1/en/model_doc/timm_wrapper#transformers.TimmWrapperImageProcessor) for processing images).

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the image classification/regression loss. Indices should be in `[0, ..., config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If `config.num_labels > 1` a classification loss is computed (Cross-Entropy).

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. Not compatible with timm wrapped models.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. Not compatible with timm wrapped models.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple. **kwargs: Additional keyword arguments passed along to the `timm` model forward.

**Returns:** [ImageClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutput) or `tuple(torch.FloatTensor)`

A [ImageClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([TimmWrapperConfig](/docs/transformers/v5.15.1/en/model_doc/timm_wrapper#transformers.TimmWrapperConfig)) and inputs.

The [TimmWrapperForImageClassification](/docs/transformers/v5.15.1/en/model_doc/timm_wrapper#transformers.TimmWrapperForImageClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Classification (or regression if config.num_labels==1) scores (before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each stage) of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states
  (also called feature maps) of the model at the output of each stage.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, patch_size,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:
```python
>>> import torch
>>> from PIL import Image
>>> from urllib.request import urlopen
>>> from transformers import AutoModelForImageClassification, AutoImageProcessor

>>> # Load image
>>> image = Image.open(urlopen(
...     'https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/beignets-task-guide.png'
... ))

>>> # Load model and image processor
>>> checkpoint = "timm/resnet50.a1_in1k"
>>> image_processor = AutoImageProcessor.from_pretrained(checkpoint)
>>> model = AutoModelForImageClassification.from_pretrained(checkpoint).eval()

>>> # Preprocess image
>>> inputs = image_processor(image)

>>> # Forward pass
>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # Get top 5 predictions
>>> top5_probabilities, top5_class_indices = torch.topk(logits.softmax(dim=1) * 100, k=5)
```
