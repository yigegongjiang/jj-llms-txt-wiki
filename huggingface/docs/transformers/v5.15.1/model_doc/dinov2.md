# DINOv2

[DINOv2](https://huggingface.co/papers/2304.07193) is a vision foundation model that uses [ViT](./vit) as a feature extractor for multiple downstream tasks like image classification and depth estimation. It focuses on stabilizing and accelerating training through techniques like a faster memory-efficient attention, sequence packing, improved stochastic depth, Fully Sharded Data Parallel (FSDP), and model distillation.

You can find all the original DINOv2 checkpoints under the [Dinov2](https://huggingface.co/collections/facebook/dinov2-6526c98554b3d2576e071ce3) collection.

> [!TIP]
> Click on the DINOv2 models in the right sidebar for more examples of how to apply DINOv2 to different vision tasks.

The example below demonstrates how to obtain an image embedding with [Pipeline](/docs/transformers/v5.15.1/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline

pipe = pipeline(
    task="image-classification",
    model="facebook/dinov2-small-imagenet1k-1-layer",
    device=0
)

pipe("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg")
```

```python
import requests
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForImageClassification

url = "http://images.cocodataset.org/val2017/000000039769.jpg"
image = Image.open(requests.get(url, stream=True).raw)

processor = AutoImageProcessor.from_pretrained("facebook/dinov2-small-imagenet1k-1-layer")
model = AutoModelForImageClassification.from_pretrained(
    "facebook/dinov2-small-imagenet1k-1-layer",
    device_map="auto",
    attn_implementation="sdpa"
)

inputs = processor(images=image, return_tensors="pt").to(model.device)
logits = model(**inputs).logits
predicted_class_idx = logits.argmax(-1).item()
print("Predicted class:", model.config.id2label[predicted_class_idx])
```

Quantization reduces the memory burden of large models by representing the weights in a lower precision. Refer to the [Quantization](../quantization/overview) overview for more available quantization backends.

The example below uses [torchao](../quantization/torchao) to only quantize the weights to int4.

```python
# pip install torchao
import requests
from PIL import Image
from torchao.quantization import Int4WeightOnlyConfig

from transformers import AutoImageProcessor, AutoModelForImageClassification, TorchAoConfig

url = 'http://images.cocodataset.org/val2017/000000039769.jpg'
image = Image.open(requests.get(url, stream=True).raw)

processor = AutoImageProcessor.from_pretrained('facebook/dinov2-giant-imagenet1k-1-layer')

quant_config = Int4WeightOnlyConfig(group_size=128)
quantization_config = TorchAoConfig(quant_type=quant_config)

model = AutoModelForImageClassification.from_pretrained(
    'facebook/dinov2-giant-imagenet1k-1-layer',
    device_map="auto",
    quantization_config=quantization_config
)

inputs = processor(images=image, return_tensors="pt").to(model.device)
outputs = model(**inputs)
logits = outputs.logits
predicted_class_idx = logits.argmax(-1).item()
print("Predicted class:", model.config.id2label[predicted_class_idx])
```

## Notes

- The example below shows how to split the output tensor into:
  - one embedding for the whole image, commonly referred to as a `CLS` token,
    useful for classification and retrieval
  - a set of local embeddings, one for each `14x14` patch of the input image,
    useful for dense tasks, such as semantic segmentation

  ```py
  from transformers import AutoImageProcessor, AutoModel
  from PIL import Image
  import requests

  url = 'http://images.cocodataset.org/val2017/000000039769.jpg'
  image = Image.open(requests.get(url, stream=True).raw)
  print(image.height, image.width)  # [480, 640]

  processor = AutoImageProcessor.from_pretrained('facebook/dinov2-base')
  model = AutoModel.from_pretrained('facebook/dinov2-base', device_map="auto")
  patch_size = model.config.patch_size

  inputs = processor(images=image, return_tensors="pt").to(model.device)
  print(inputs.pixel_values.shape)  # [1, 3, 224, 224]
  batch_size, rgb, img_height, img_width = inputs.pixel_values.shape
  num_patches_height, num_patches_width = img_height // patch_size, img_width // patch_size
  num_patches_flat = num_patches_height * num_patches_width

  outputs = model(**inputs)
  last_hidden_states = outputs[0]
  print(last_hidden_states.shape)  # [1, 1 + 256, 768]
  assert last_hidden_states.shape == (batch_size, 1 + num_patches_flat, model.config.hidden_size)

  cls_token = last_hidden_states[:, 0, :]
  patch_features = last_hidden_states[:, 1:, :].unflatten(1, (num_patches_height, num_patches_width))
  ```

- Use [torch.jit.trace](https://pytorch.org/docs/stable/generated/torch.jit.trace.html) to speedup inference.
  However, it will produce some mismatched elements. The difference between the original and traced model is 1e-4.

  ```py
  import torch
  from transformers import AutoImageProcessor, AutoModel
  from PIL import Image
  import requests

  url = 'http://images.cocodataset.org/val2017/000000039769.jpg'
  image = Image.open(requests.get(url, stream=True).raw)

  processor = AutoImageProcessor.from_pretrained('facebook/dinov2-base')
  model = AutoModel.from_pretrained('facebook/dinov2-base', device_map="auto")

  inputs = processor(images=image, return_tensors="pt").to(model.device)
  outputs = model(**inputs)
  last_hidden_states = outputs[0]

  # We have to force return_dict=False for tracing
  model.config.return_dict = False

  with torch.no_grad():
      traced_model = torch.jit.trace(model, [inputs.pixel_values])
      traced_outputs = traced_model(inputs.pixel_values)

  print((last_hidden_states - traced_outputs[0]).abs().max())
  ```

## Dinov2Config[[transformers.Dinov2Config]]

#### transformers.Dinov2Config[[transformers.Dinov2Config]]

```python
transformers.Dinov2Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 768, num_hidden_layers: int = 12, num_attention_heads: int = 12, mlp_ratio: int = 4, hidden_act: str = 'gelu', hidden_dropout_prob: float | int = 0.0, attention_probs_dropout_prob: float | int = 0.0, initializer_range: float = 0.02, layer_norm_eps: float = 1e-06, image_size: int | list[int] | tuple[int, int] = 224, patch_size: int | list[int] | tuple[int, int] = 14, num_channels: int = 3, qkv_bias: bool = True, layerscale_value: float = 1.0, drop_path_rate: float | int = 0.0, use_swiglu_ffn: bool = False, _out_features: list[str] | None = None, _out_indices: list[int] | None = None, apply_layernorm: bool = True, reshape_hidden_states: bool = True, use_mask_token: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dinov2/configuration_dinov2.py#L25)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

num_hidden_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer decoder.

mlp_ratio (`int`, *optional*, defaults to `4`) : Ratio of the MLP hidden dim to the embedding dim.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

hidden_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

attention_probs_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

image_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `224`) : The size (resolution) of each image.

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `14`) : The size (resolution) of each patch.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

qkv_bias (`bool`, *optional*, defaults to `True`) : Whether to add a bias to the queries, keys and values.

layerscale_value (`float`, *optional*, defaults to 1.0) : Initial value to use for layer scale.

drop_path_rate (`Union[float, int]`, *optional*, defaults to `0.0`) : Drop path rate for the patch fusion.

use_swiglu_ffn (`bool`, *optional*, defaults to `False`) : Whether to use the SwiGLU feedforward neural network.

apply_layernorm (`bool`, *optional*, defaults to `True`) : Whether to apply layer normalization to the feature maps in case the model is used as backbone.

reshape_hidden_states (`bool`, *optional*, defaults to `True`) : Whether to reshape the feature maps to 4D tensors of shape `(batch_size, hidden_size, height, width)` in case the model is used as backbone. If `False`, the feature maps will be 3D tensors of shape `(batch_size, seq_len, hidden_size)`.

use_mask_token (`bool`, *optional*, defaults to `True`) : Whether to use mask_token in embeddings.

This is the configuration class to store the configuration of a Dinov2Model. It is used to instantiate a Dinov2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/dinov2-base](https://huggingface.co/facebook/dinov2-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Dinov2Config, Dinov2Model

>>> # Initializing a Dinov2 dinov2-base style configuration
>>> configuration = Dinov2Config()

>>> # Initializing a model (with random weights) from the dinov2-base style configuration
>>> model = Dinov2Model(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Dinov2Model[[transformers.Dinov2Model]]

#### transformers.Dinov2Model[[transformers.Dinov2Model]]

```python
transformers.Dinov2Model(config: Dinov2Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dinov2/modeling_dinov2.py#L433)

**Parameters:**

config ([Dinov2Config](/docs/transformers/v5.15.1/en/model_doc/dinov2#transformers.Dinov2Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Dinov2 Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Dinov2Model.forward]]

```python
forward(pixel_values: typing.Optional[torch.Tensor] = None, bool_masked_pos: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dinov2/modeling_dinov2.py#L449)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [BitImageProcessor](/docs/transformers/v5.15.1/en/model_doc/bit#transformers.BitImageProcessor). See `BitImageProcessor.__call__()` for details (`processor_class` uses [BitImageProcessor](/docs/transformers/v5.15.1/en/model_doc/bit#transformers.BitImageProcessor) for processing images).

bool_masked_pos (`torch.BoolTensor` of shape `(batch_size, sequence_length)`) : Boolean masked positions. Indicates which patches are masked (1) and which aren't (0). Only relevant for pre-training.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Dinov2Config](/docs/transformers/v5.15.1/en/model_doc/dinov2#transformers.Dinov2Config)) and inputs.

The [Dinov2Model](/docs/transformers/v5.15.1/en/model_doc/dinov2#transformers.Dinov2Model) forward method, overrides the `__call__` special method.

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

## Dinov2ForImageClassification[[transformers.Dinov2ForImageClassification]]

#### transformers.Dinov2ForImageClassification[[transformers.Dinov2ForImageClassification]]

```python
transformers.Dinov2ForImageClassification(config: Dinov2Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dinov2/modeling_dinov2.py#L486)

**Parameters:**

config ([Dinov2Config](/docs/transformers/v5.15.1/en/model_doc/dinov2#transformers.Dinov2Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Dinov2 Model transformer with an image classification head on top (a linear layer on top of the final hidden state
of the [CLS] token) e.g. for ImageNet.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Dinov2ForImageClassification.forward]]

```python
forward(pixel_values: typing.Optional[torch.Tensor] = None, labels: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dinov2/modeling_dinov2.py#L501)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [BitImageProcessor](/docs/transformers/v5.15.1/en/model_doc/bit#transformers.BitImageProcessor). See `BitImageProcessor.__call__()` for details (`processor_class` uses [BitImageProcessor](/docs/transformers/v5.15.1/en/model_doc/bit#transformers.BitImageProcessor) for processing images).

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the image classification/regression loss. Indices should be in `[0, ..., config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If `config.num_labels > 1` a classification loss is computed (Cross-Entropy).

**Returns:** [ImageClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutput) or `tuple(torch.FloatTensor)`

A [ImageClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Dinov2Config](/docs/transformers/v5.15.1/en/model_doc/dinov2#transformers.Dinov2Config)) and inputs.

The [Dinov2ForImageClassification](/docs/transformers/v5.15.1/en/model_doc/dinov2#transformers.Dinov2ForImageClassification) forward method, overrides the `__call__` special method.

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

Example:

```python
>>> from transformers import AutoImageProcessor, Dinov2ForImageClassification
>>> import torch
>>> from datasets import load_dataset

>>> dataset = load_dataset("huggingface/cats-image")
>>> image = dataset["test"]["image"][0]

>>> image_processor = AutoImageProcessor.from_pretrained("facebook/dinov2-base")
>>> model = Dinov2ForImageClassification.from_pretrained("facebook/dinov2-base")

>>> inputs = image_processor(image, return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # model predicts one of the 1000 ImageNet classes
>>> predicted_label = logits.argmax(-1).item()
>>> print(model.config.id2label[predicted_label])
...
```
