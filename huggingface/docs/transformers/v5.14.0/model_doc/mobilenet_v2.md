# MobileNet V2

[MobileNet V2](https://huggingface.co/papers/1801.04381) improves performance on mobile devices with a more efficient architecture. It uses inverted residual blocks and linear bottlenecks to start with a smaller representation of the data, expands it for processing, and shrinks it again to reduce the number of computations. The model also removes non-linearities to maintain accuracy despite its simplified design. Like [MobileNet V1](./mobilenet_v1), it uses depthwise separable convolutions for efficiency.

You can all the original MobileNet checkpoints under the [Google](https://huggingface.co/google?search_models=mobilenet) organization.

> [!TIP]
> Click on the MobileNet V2 models in the right sidebar for more examples of how to apply MobileNet to different vision tasks.

The examples below demonstrate how to classify an image with [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline

pipeline = pipeline(
    task="image-classification",
    model="google/mobilenet_v2_1.4_224",
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
    "google/mobilenet_v2_1.4_224",
)
model = AutoModelForImageClassification.from_pretrained(
    "google/mobilenet_v2_1.4_224",
    device_map="auto",
)

url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
image = Image.open(requests.get(url, stream=True).raw)
inputs = image_processor(image, return_tensors="pt").to(model.device)

with torch.no_grad():
  logits = model(**inputs).logits
predicted_class_id = logits.argmax(dim=-1).item()

class_labels = model.config.id2label
predicted_class_label = class_labels[predicted_class_id]
print(f"The predicted class label is: {predicted_class_label}")
```

## Notes

- Classification checkpoint names follow the pattern `mobilenet_v2_{depth_multiplier}_{resolution}`, like `mobilenet_v2_1.4_224`. `1.4` is the depth multiplier and `224` is the image resolution. Segmentation checkpoint names follow the pattern `deeplabv3_mobilenet_v2_{depth_multiplier}_{resolution}`.
- While trained on images of a specific sizes, the model architecture works with images of different sizes (minimum 32x32). The [MobileNetV2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mobilenet_v2#transformers.MobileNetV2ImageProcessor) handles the necessary preprocessing.
- MobileNet is pretrained on [ImageNet-1k](https://huggingface.co/datasets/ILSVRC/imagenet-1k), a dataset with 1000 classes. However, the model actually predicts 1001 classes. The additional class is an extra "background" class (index 0).
- The segmentation models use a [DeepLabV3+](https://huggingface.co/papers/1802.02611) head which is often pretrained on datasets like [PASCAL VOC](https://huggingface.co/datasets/merve/pascal-voc).
- The original TensorFlow checkpoints determines the padding amount at inference because it depends on the input image size. To use the native PyTorch padding behavior, set `tf_padding=False` in [MobileNetV2Config](/docs/transformers/v5.14.0/en/model_doc/mobilenet_v2#transformers.MobileNetV2Config).

    ```python
    from transformers import MobileNetV2Config

    config = MobileNetV2Config.from_pretrained("google/mobilenet_v2_1.4_224", tf_padding=True)
    ```

- The Transformers implementation does not support the following features.
  - Uses global average pooling instead of the optional 7x7 average pooling with stride 2. For larger inputs, this gives a pooled output that is larger than a 1x1 pixel.
  - `output_hidden_states=True` returns *all* intermediate hidden states. It is not possible to extract the output from specific layers for other downstream purposes.
  - Does not include the quantized models from the original checkpoints because they include "FakeQuantization" operations to unquantize the weights.
  - For segmentation models, the final convolution layer of the backbone is computed even though the DeepLabV3+ head doesn't use it.

## MobileNetV2Config[[transformers.MobileNetV2Config]]

- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `224`) --
  The size (resolution) of each image.
- **depth_multiplier** (`Union[float, int]`, *optional*, defaults to `1.0`) --
  Shrinks or expands the number of channels in each layer. This is sometimes also called "alpha" or "width multiplier".
- **depth_divisible_by** (`int`, *optional*, defaults to 8) --
  The number of channels in each layer will always be a multiple of this number.
- **min_depth** (`int`, *optional*, defaults to 8) --
  All layers will have at least this many channels.
- **expand_ratio** (`float`, *optional*, defaults to 6.0) --
  The number of output channels of the first layer in each block is input channels times expansion ratio.
- **output_stride** (`int`, *optional*, defaults to 32) --
  The ratio between the spatial resolution of the input and output feature maps. By default the model reduces
  the input dimensions by a factor of 32. If `output_stride` is 8 or 16, the model uses dilated convolutions
  on the depthwise layers instead of regular convolutions, so that the feature maps never become more than 8x
  or 16x smaller than the input image.
- **first_layer_is_expansion** (`bool`, *optional*, defaults to `True`) --
  True if the very first convolution layer is also the expansion layer for the first expansion block.
- **finegrained_output** (`bool`, *optional*, defaults to `True`) --
  If true, the number of output channels in the final convolution layer will stay large (1280) even if
  `depth_multiplier` is less than 1.
- **hidden_act** (`str`, *optional*, defaults to `relu6`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **tf_padding** (`bool`, *optional*, defaults to `True`) --
  Whether to use TensorFlow padding rules on the convolution layers.
- **classifier_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.8`) --
  The dropout ratio for classifier.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **layer_norm_eps** (`float`, *optional*, defaults to `0.001`) --
  The epsilon used by the layer normalization layers.
- **semantic_loss_ignore_index** (`int`, *optional*, defaults to `255`) --
  The index that is ignored by the loss function of the semantic segmentation model.

This is the configuration class to store the configuration of a MobileNetV2Model. It is used to instantiate a Mobilenet V2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/mobilenet_v2_1.0_224](https://huggingface.co/google/mobilenet_v2_1.0_224)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import MobileNetV2Config, MobileNetV2Model

>>> # Initializing a "mobilenet_v2_1.0_224" style configuration
>>> configuration = MobileNetV2Config()

>>> # Initializing a model from the "mobilenet_v2_1.0_224" style configuration
>>> model = MobileNetV2Model(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## MobileNetV2ImageProcessor[[transformers.MobileNetV2ImageProcessor]]

- **do_reduce_labels** (`bool`, *kwargs*, *optional*, defaults to `self.do_reduce_labels`) --
  Whether or not to reduce all label values of segmentation maps by 1. Usually used for datasets where 0
  is used for background, and background itself is not included in all classes of a dataset (e.g.
  ADE20k). The background label will be replaced by 255.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a MobileNetV2ImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **segmentation_maps** (`ImageInput`, *optional*) --
  The segmentation maps to preprocess.
- **do_reduce_labels** (`bool`, *kwargs*, *optional*, defaults to `self.do_reduce_labels`) --
  Whether or not to reduce all label values of segmentation maps by 1. Usually used for datasets where 0
  is used for background, and background itself is not included in all classes of a dataset (e.g.
  ADE20k). The background label will be replaced by 255.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** ([MobileNetV2ForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/mobilenet_v2#transformers.MobileNetV2ForSemanticSegmentation)) --
  Raw outputs of the model.
- **target_sizes** (`list[tuple[int, int]]`, *optional*) --
  List of tuples corresponding to the requested final size (height, width) of each prediction.
- **return_segmentation_scores** (`bool`, *optional*, defaults to `False`) --
  Whether to return segmentation scores alongside the segmentation map. When `True`, each element of
  the returned list is a `SemanticSegmentationPostProcessorOutput` with fields `segmentation`
  (class IDs, shape `(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`).`list[torch.Tensor]` or `list[SemanticSegmentationPostProcessorOutput]`When
`return_segmentation_scores=False` (default), a list of length `batch_size` where each item is a
segmentation map of shape `(height, width)` with class IDs. When `return_segmentation_scores=True`,
a list of `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape
`(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`). In both cases,
`(height, width)` corresponds to the target size (if `target_sizes` is specified).

Converts the output of [MobileNetV2ForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/mobilenet_v2#transformers.MobileNetV2ForSemanticSegmentation) into semantic segmentation maps.

## MobileNetV2ImageProcessorPil[[transformers.MobileNetV2ImageProcessorPil]]

- **do_reduce_labels** (`bool`, *kwargs*, *optional*, defaults to `self.do_reduce_labels`) --
  Whether or not to reduce all label values of segmentation maps by 1. Usually used for datasets where 0
  is used for background, and background itself is not included in all classes of a dataset (e.g.
  ADE20k). The background label will be replaced by 255.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a MobileNetV2ImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **segmentation_maps** (`ImageInput`, *optional*) --
  The segmentation maps to preprocess.
- **do_reduce_labels** (`bool`, *kwargs*, *optional*, defaults to `self.do_reduce_labels`) --
  Whether or not to reduce all label values of segmentation maps by 1. Usually used for datasets where 0
  is used for background, and background itself is not included in all classes of a dataset (e.g.
  ADE20k). The background label will be replaced by 255.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** ([MobileNetV2ForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/mobilenet_v2#transformers.MobileNetV2ForSemanticSegmentation)) --
  Raw outputs of the model.
- **target_sizes** (`list[tuple[int, int]]`, *optional*) --
  List of tuples corresponding to the requested final size (height, width) of each prediction.
- **return_segmentation_scores** (`bool`, *optional*, defaults to `False`) --
  Whether to return segmentation scores alongside the segmentation map. When `True`, each element of
  the returned list is a `SemanticSegmentationPostProcessorOutput` with fields `segmentation`
  (class IDs, shape `(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`).`list[torch.Tensor]` or `list[SemanticSegmentationPostProcessorOutput]`When
`return_segmentation_scores=False` (default), a list of length `batch_size` where each item is a
segmentation map of shape `(height, width)` with class IDs. When `return_segmentation_scores=True`,
a list of `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape
`(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`). In both cases,
`(height, width)` corresponds to the target size (if `target_sizes` is specified).

Converts the output of [MobileNetV2ForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/mobilenet_v2#transformers.MobileNetV2ForSemanticSegmentation) into semantic segmentation maps.

## MobileNetV2Model[[transformers.MobileNetV2Model]]

- **config** ([MobileNetV2Config](/docs/transformers/v5.14.0/en/model_doc/mobilenet_v2#transformers.MobileNetV2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
- **add_pooling_layer** (`bool`, *optional*, defaults to `True`) --
  Whether to add a pooling layer

The bare Mobilenet V2 Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [MobileNetV2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mobilenet_v2#transformers.MobileNetV2ImageProcessor). See `MobileNetV2ImageProcessor.__call__()` for details (`processor_class` uses
  [MobileNetV2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mobilenet_v2#transformers.MobileNetV2ImageProcessor) for processing images).
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.`BaseModelOutputWithPoolingAndNoAttention` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithPoolingAndNoAttention` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MobileNetV2Config](/docs/transformers/v5.14.0/en/model_doc/mobilenet_v2#transformers.MobileNetV2Config)) and inputs.
The [MobileNetV2Model](/docs/transformers/v5.14.0/en/model_doc/mobilenet_v2#transformers.MobileNetV2Model) forward method, overrides the `__call__` special method.

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

## MobileNetV2ForImageClassification[[transformers.MobileNetV2ForImageClassification]]

- **config** ([MobileNetV2Config](/docs/transformers/v5.14.0/en/model_doc/mobilenet_v2#transformers.MobileNetV2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

MobileNetV2 model with an image classification head on top (a linear layer on top of the pooled features), e.g. for
ImageNet.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [MobileNetV2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mobilenet_v2#transformers.MobileNetV2ImageProcessor). See `MobileNetV2ImageProcessor.__call__()` for details (`processor_class` uses
  [MobileNetV2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mobilenet_v2#transformers.MobileNetV2ImageProcessor) for processing images).
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **labels** (`torch.LongTensor` of shape `(batch_size,)`, *optional*) --
  Labels for computing the image classification/regression loss. Indices should be in `[0, ...,
  config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss). If
  `config.num_labels > 1` a classification loss is computed (Cross-Entropy).
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.[ImageClassifierOutputWithNoAttention](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutputWithNoAttention) or `tuple(torch.FloatTensor)`A [ImageClassifierOutputWithNoAttention](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutputWithNoAttention) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MobileNetV2Config](/docs/transformers/v5.14.0/en/model_doc/mobilenet_v2#transformers.MobileNetV2Config)) and inputs.
The [MobileNetV2ForImageClassification](/docs/transformers/v5.14.0/en/model_doc/mobilenet_v2#transformers.MobileNetV2ForImageClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Classification (or regression if config.num_labels==1) scores (before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each stage) of shape `(batch_size, num_channels, height, width)`. Hidden-states (also
  called feature maps) of the model at the output of each stage.

Example:

```python
>>> from transformers import AutoImageProcessor, MobileNetV2ForImageClassification
>>> import torch
>>> from datasets import load_dataset

>>> dataset = load_dataset("huggingface/cats-image")
>>> image = dataset["test"]["image"][0]

>>> image_processor = AutoImageProcessor.from_pretrained("google/mobilenet_v2_1.0_224")
>>> model = MobileNetV2ForImageClassification.from_pretrained("google/mobilenet_v2_1.0_224")

>>> inputs = image_processor(image, return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # model predicts one of the 1000 ImageNet classes
>>> predicted_label = logits.argmax(-1).item()
>>> print(model.config.id2label[predicted_label])
...
```

## MobileNetV2ForSemanticSegmentation[[transformers.MobileNetV2ForSemanticSegmentation]]

- **config** ([MobileNetV2Config](/docs/transformers/v5.14.0/en/model_doc/mobilenet_v2#transformers.MobileNetV2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

MobileNetV2 model with a semantic segmentation head on top, e.g. for Pascal VOC.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [MobileNetV2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mobilenet_v2#transformers.MobileNetV2ImageProcessor). See `MobileNetV2ImageProcessor.__call__()` for details (`processor_class` uses
  [MobileNetV2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mobilenet_v2#transformers.MobileNetV2ImageProcessor) for processing images).
- **labels** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Ground truth semantic segmentation maps for computing the loss. Indices should be in `[0, ...,
  config.num_labels - 1]`. If `config.num_labels > 1`, a classification loss is computed (Cross-Entropy).
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.[SemanticSegmenterOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.SemanticSegmenterOutput) or `tuple(torch.FloatTensor)`A [SemanticSegmenterOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.SemanticSegmenterOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MobileNetV2Config](/docs/transformers/v5.14.0/en/model_doc/mobilenet_v2#transformers.MobileNetV2Config)) and inputs.
The [MobileNetV2ForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/mobilenet_v2#transformers.MobileNetV2ForSemanticSegmentation) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels, logits_height, logits_width)`) -- Classification scores for each pixel.

  

  The logits returned do not necessarily have the same size as the `pixel_values` passed as inputs. This is
  to avoid doing two interpolations and lose some quality when a user needs to resize the logits to the
  original image size as post-processing. You should always check your logits shape and resize as needed.

  
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, patch_size, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, patch_size,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import AutoImageProcessor, MobileNetV2ForSemanticSegmentation
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("google/deeplabv3_mobilenet_v2_1.0_513")
>>> model = MobileNetV2ForSemanticSegmentation.from_pretrained("google/deeplabv3_mobilenet_v2_1.0_513")

>>> inputs = image_processor(images=image, return_tensors="pt")

>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> # logits are of shape (batch_size, num_labels, height, width)
>>> logits = outputs.logits
```
