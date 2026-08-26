# HGNet-V2

[HGNetV2](https://github.com/PaddlePaddle/PaddleClas/blob/v2.6.0/docs/zh_CN/models/ImageNet1k/PP-HGNetV2.md) is a next-generation convolutional neural network (CNN) backbone built for optimal accuracy-latency tradeoff on NVIDIA GPUs. Building on the original[HGNet](https://github.com/PaddlePaddle/PaddleClas/blob/v2.6.0/docs/en/models/PP-HGNet_en.md), HGNetV2 delivers high accuracy at fast inference speeds and performs strongly on tasks like image classification, object detection, and segmentation, making it a practical choice for GPU-based computer vision applications.

You can find all the original HGNet V2 models under the [USTC](https://huggingface.co/ustc-community/models?search=hgnet) organization.

> [!TIP]
> This model was contributed by [VladOS95-cyber](https://github.com/VladOS95-cyber).
> Click on the HGNet V2 models in the right sidebar for more examples of how to apply HGNet V2 to different computer vision tasks.

The example below demonstrates how to classify an image with [Pipeline](/docs/transformers/v5.15.1/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline

pipeline = pipeline(
    task="image-classification",
    model="ustc-community/hgnet-v2",
    device=0
)
pipeline("http://images.cocodataset.org/val2017/000000039769.jpg")
```

```python
import requests
import torch
from PIL import Image

from transformers import AutoImageProcessor, HGNetV2ForImageClassification

url = "http://images.cocodataset.org/val2017/000000039769.jpg"
image = Image.open(requests.get(url, stream=True).raw)

model = HGNetV2ForImageClassification.from_pretrained("ustc-community/hgnet-v2", device_map="auto")
processor = AutoImageProcessor.from_pretrained("ustc-community/hgnet-v2")

inputs = processor(images=image, return_tensors="pt").to(model.device)
with torch.no_grad():
    logits = model(**inputs).logits
predicted_class_id = logits.argmax(dim=-1).item()

class_labels = model.config.id2label
predicted_class_label = class_labels[predicted_class_id]
print(f"The predicted class label is: {predicted_class_label}")
```

## HGNetV2Config[[transformers.HGNetV2Config]]

#### transformers.HGNetV2Config[[transformers.HGNetV2Config]]

```python
transformers.HGNetV2Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, num_channels: int = 3, embedding_size: int = 64, depths: list[int] | tuple[int, ...] = (3, 4, 6, 3), hidden_sizes: list[int] | tuple[int, ...] = (256, 512, 1024, 2048), hidden_act: str = 'relu', _out_features: list[str] | None = None, _out_indices: list[int] | None = None, stem_channels: list[int] | tuple[int, ...] = (3, 32, 48), stem_strides: Sequence = (2, 1, 1, 2, 1), stage_in_channels: list[int] | tuple[int, ...] = (48, 128, 512, 1024), stage_mid_channels: list[int] | tuple[int, ...] = (48, 96, 192, 384), stage_out_channels: list[int] | tuple[int, ...] = (128, 512, 1024, 2048), stage_num_blocks: list[int] | tuple[int, ...] = (1, 1, 3, 1), stage_downsample: list[bool] | tuple[bool, ...] = (False, True, True, True), stage_downsample_strides: Sequence = (2, 2, 2, 2), stage_light_block: list[bool] | tuple[bool, ...] = (False, False, True, True), stage_kernel_size: list[int] | tuple[int, ...] = (3, 3, 5, 5), stage_numb_of_layers: list[int] | tuple[int, ...] = (6, 6, 6, 6), use_learnable_affine_block: bool = False, initializer_range: float = 0.02)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hgnet_v2/configuration_hgnet_v2.py#L35)

**Parameters:**

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

embedding_size (`int`, *optional*, defaults to `64`) : Dimensionality of the embeddings and hidden states.

depths (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(3, 4, 6, 3)`) : Depth of each layer in the Transformer.

hidden_sizes (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(256, 512, 1024, 2048)`) : Dimensionality (hidden size) at each stage of the model.

hidden_act (`str`, *optional*, defaults to `relu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

stem_channels (`list[int]`, *optional*, defaults to `[3, 32, 48]`) : Channel dimensions for the stem layers: - First number (3) is input image channels - Second number (32) is intermediate stem channels - Third number (48) is output stem channels

stem_strides (`Sequence[int | list[int] | tuple[int, ...]]`, *optional*, defaults to `(2, 1, 1, 2, 1)`) : Stride patterns for the stem layers.

stage_in_channels (`list[int]`, *optional*, defaults to `[48, 128, 512, 1024]`) : Input channel dimensions for each stage of the backbone. This defines how many channels the input to each stage will have.

stage_mid_channels (`list[int]`, *optional*, defaults to `[48, 96, 192, 384]`) : Mid-channel dimensions for each stage of the backbone. This defines the number of channels used in the intermediate layers of each stage.

stage_out_channels (`list[int]`, *optional*, defaults to `[128, 512, 1024, 2048]`) : Output channel dimensions for each stage of the backbone. This defines how many channels the output of each stage will have.

stage_num_blocks (`list[int]`, *optional*, defaults to `[1, 1, 3, 1]`) : Number of blocks to be used in each stage of the backbone. This controls the depth of each stage by specifying how many convolutional blocks to stack.

stage_downsample (`list[bool]`, *optional*, defaults to `[False, True, True, True]`) : Indicates whether to downsample the feature maps at each stage. If `True`, the spatial dimensions of the feature maps will be reduced.

stage_downsample_strides (`Sequence[int | list[int] | tuple[int, ...]]`, *optional*, defaults to `(2, 2, 2, 2)`) : Stride patterns for each stage layer.

stage_light_block (`list[bool]`, *optional*, defaults to `[False, False, True, True]`) : Indicates whether to use light blocks in each stage. Light blocks are a variant of convolutional blocks that may have fewer parameters.

stage_kernel_size (`list[int]`, *optional*, defaults to `[3, 3, 5, 5]`) : Kernel sizes for the convolutional layers in each stage.

stage_numb_of_layers (`list[int]`, *optional*, defaults to `[6, 6, 6, 6]`) : Number of layers to be used in each block of the stage.

use_learnable_affine_block (`bool`, *optional*, defaults to `False`) : Whether to use Learnable Affine Blocks (LAB) in the network. LAB adds learnable scale and bias parameters after certain operations.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a HGNetV2Backbone. It is used to instantiate a Hgnet V2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [ustc-community/dfine_x_coco](https://huggingface.co/ustc-community/dfine_x_coco)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## HGNetV2Backbone[[transformers.HGNetV2Backbone]]

#### transformers.HGNetV2Backbone[[transformers.HGNetV2Backbone]]

```python
transformers.HGNetV2Backbone(config: HGNetV2Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hgnet_v2/modeling_hgnet_v2.py#L343)

#### forward[[transformers.HGNetV2Backbone.forward]]

```python
forward(pixel_values: Tensor, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hgnet_v2/modeling_hgnet_v2.py#L356)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses `image_processor_class` for processing images).

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `BackboneOutput` or `tuple(torch.FloatTensor)`

A `BackboneOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([HGNetV2Config](/docs/transformers/v5.15.1/en/model_doc/hgnet_v2#transformers.HGNetV2Config)) and inputs.

The [HGNetV2Backbone](/docs/transformers/v5.15.1/en/model_doc/hgnet_v2#transformers.HGNetV2Backbone) forward method, overrides the `__call__` special method.

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
>>> from transformers import HGNetV2Config, HGNetV2Backbone
>>> import torch

>>> config = HGNetV2Config()
>>> model = HGNetV2Backbone(config)

>>> pixel_values = torch.randn(1, 3, 224, 224)

>>> with torch.no_grad():
...     outputs = model(pixel_values)

>>> feature_maps = outputs.feature_maps
>>> list(feature_maps[-1].shape)
[1, 2048, 7, 7]
```

## HGNetV2ForImageClassification[[transformers.HGNetV2ForImageClassification]]

#### transformers.HGNetV2ForImageClassification[[transformers.HGNetV2ForImageClassification]]

```python
transformers.HGNetV2ForImageClassification(config: HGNetV2Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hgnet_v2/modeling_hgnet_v2.py#L420)

**Parameters:**

config ([HGNetV2Config](/docs/transformers/v5.15.1/en/model_doc/hgnet_v2#transformers.HGNetV2Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

HGNetV2 Model with an image classification head on top (a linear layer on top of the pooled features), e.g. for
ImageNet.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.HGNetV2ForImageClassification.forward]]

```python
forward(pixel_values: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hgnet_v2/modeling_hgnet_v2.py#L436)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses `image_processor_class` for processing images).

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the image classification/regression loss. Indices should be in `[0, ..., config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If `config.num_labels > 1` a classification loss is computed (Cross-Entropy).

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [ImageClassifierOutputWithNoAttention](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutputWithNoAttention) or `tuple(torch.FloatTensor)`

A [ImageClassifierOutputWithNoAttention](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutputWithNoAttention) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([HGNetV2Config](/docs/transformers/v5.15.1/en/model_doc/hgnet_v2#transformers.HGNetV2Config)) and inputs.

The [HGNetV2ForImageClassification](/docs/transformers/v5.15.1/en/model_doc/hgnet_v2#transformers.HGNetV2ForImageClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Classification (or regression if config.num_labels==1) scores (before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each stage) of shape `(batch_size, num_channels, height, width)`. Hidden-states (also
  called feature maps) of the model at the output of each stage.

Examples:
```python
>>> import torch
>>> import httpx
>>> from io import BytesIO
>>> from transformers import HGNetV2ForImageClassification, AutoImageProcessor
>>> from PIL import Image

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> model = HGNetV2ForImageClassification.from_pretrained("ustc-community/hgnet-v2")
>>> processor = AutoImageProcessor.from_pretrained("ustc-community/hgnet-v2")

>>> inputs = processor(images=image, return_tensors="pt")
>>> with torch.no_grad():
...     outputs = model(**inputs)
>>> outputs.logits.shape
torch.Size([1, 2])
```
