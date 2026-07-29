# MobileViT

[MobileViT](https://huggingface.co/papers/2110.02178) is a lightweight vision transformer for mobile devices that merges CNNs's efficiency and inductive biases with transformers global context modeling. It treats transformers as convolutions, enabling global information processing without the heavy computational cost of standard ViTs.

   

You can find all the original MobileViT checkpoints under the [Apple](https://huggingface.co/apple/models?search=mobilevit) organization.

> [!TIP]
>
> - This model was contributed by [matthijs](https://huggingface.co/Matthijs).
>
> Click on the MobileViT models in the right sidebar for more examples of how to apply MobileViT to different vision tasks.

The example below demonstrates how to do [Image Classification] with [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) and the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline

classifier = pipeline(
   task="image-classification",
   model="apple/mobilevit-small",
   device=0,
)

preds = classifier("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg")
print(f"Prediction: {preds}\n")
```

```python
import requests
import torch
from PIL import Image

from transformers import AutoImageProcessor, MobileViTForImageClassification

image_processor = AutoImageProcessor.from_pretrained(
   "apple/mobilevit-small",
   use_fast=True,
)
model = MobileViTForImageClassification.from_pretrained("apple/mobilevit-small", device_map="auto")

url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
image = Image.open(requests.get(url, stream=True).raw)
inputs = image_processor(image, return_tensors="pt").to(model.device)

with torch.no_grad():
    logits = model(**inputs).logits
predicted_class_id = logits.argmax(dim=-1).item()

class_labels = model.config.id2label
predicted_class_label = class_labels[predicted_class_id]
print(f"The predicted class label is:{predicted_class_label}")
```

## Notes

- Does **not** operate on sequential data, it's purely designed for image tasks.
- Feature maps are used directly instead of token embeddings.
- Use [MobileViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mobilevit#transformers.MobileViTImageProcessor) to preprocess images.
- If using custom preprocessing, ensure that images are in **BGR** format (not RGB), as expected by the pretrained weights.
- The classification models are pretrained on [ImageNet-1k](https://huggingface.co/datasets/ILSVRC/imagenet-1k).
- The segmentation models use a [DeepLabV3](https://huggingface.co/papers/1706.05587) head and are pretrained on [PASCAL VOC](http://host.robots.ox.ac.uk/pascal/VOC/).

## MobileViTConfig[[transformers.MobileViTConfig]]

- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `256`) --
  The size (resolution) of each image.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `2`) --
  The size (resolution) of each patch.
- **hidden_sizes** (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(144, 192, 240)`) --
  Dimensionality (hidden size) at each stage of the model.
- **neck_hidden_sizes** (`list[int]`, *optional*, defaults to `[16, 32, 64, 96, 128, 160, 640]`) --
  The number of channels for the feature maps of the backbone.
- **num_attention_heads** (`int`, *optional*, defaults to `4`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **mlp_ratio** (`float`, *optional*, defaults to `2.0`) --
  Ratio of the MLP hidden dim to the embedding dim.
- **expand_ratio** (`float`, *optional*, defaults to `4.0`) --
  Expand ratio to set the output dimensions for the expansion
- **hidden_act** (`str`, *optional*, defaults to `silu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **conv_kernel_size** (`int`, *optional*, defaults to `3`) --
  The size of the convolutional kernel.
- **output_stride** (`int`, *optional*, defaults to `32`) --
  The ratio between the spatial resolution of the input and output feature maps.
- **hidden_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **attention_probs_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **classifier_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout ratio for classifier.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the layer normalization layers.
- **qkv_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to add a bias to the queries, keys and values.
- **aspp_out_channels** (`int`, *optional*, defaults to 256) --
  Number of output channels used in the ASPP layer for semantic segmentation.
- **atrous_rates** (`list[int]`, *optional*, defaults to `[6, 12, 18]`) --
  Dilation (atrous) factors used in the ASPP layer for semantic segmentation.
- **aspp_dropout_prob** (`float`, *optional*, defaults to 0.1) --
  The dropout ratio for the ASPP layer for semantic segmentation.
- **semantic_loss_ignore_index** (`int`, *optional*, defaults to `255`) --
  The index that is ignored by the loss function of the semantic segmentation model.

This is the configuration class to store the configuration of a MobileViTModel. It is used to instantiate a Mobilevit
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/mobilenet_v2_1.0_224](https://huggingface.co/google/mobilenet_v2_1.0_224)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import MobileViTConfig, MobileViTModel

>>> # Initializing a mobilevit-small style configuration
>>> configuration = MobileViTConfig()

>>> # Initializing a model from the mobilevit-small style configuration
>>> model = MobileViTModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## MobileViTImageProcessor[[transformers.MobileViTImageProcessor]]

- **do_flip_channel_order** (`bool`, *kwargs*, *optional*, defaults to `self.do_flip_channel_order`) --
  Whether to flip the color channels from RGB to BGR or vice versa.
- **do_reduce_labels** (`bool`, *kwargs*, *optional*, defaults to `self.do_reduce_labels`) --
  Whether or not to reduce all label values of segmentation maps by 1. Usually used for datasets where 0
  is used for background, and background itself is not included in all classes of a dataset (e.g.
  ADE20k). The background label will be replaced by 255.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a MobileViTImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **segmentation_maps** (`ImageInput`, *optional*) --
  The segmentation maps to preprocess.
- **do_flip_channel_order** (`bool`, *kwargs*, *optional*, defaults to `self.do_flip_channel_order`) --
  Whether to flip the color channels from RGB to BGR or vice versa.
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

- **outputs** ([MobileViTForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/mobilevit#transformers.MobileViTForSemanticSegmentation)) --
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

Converts the output of [MobileViTForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/mobilevit#transformers.MobileViTForSemanticSegmentation) into semantic segmentation maps.

## MobileViTImageProcessorPil[[transformers.MobileViTImageProcessorPil]]

- **do_flip_channel_order** (`bool`, *kwargs*, *optional*, defaults to `self.do_flip_channel_order`) --
  Whether to flip the color channels from RGB to BGR or vice versa.
- **do_reduce_labels** (`bool`, *kwargs*, *optional*, defaults to `self.do_reduce_labels`) --
  Whether or not to reduce all label values of segmentation maps by 1. Usually used for datasets where 0
  is used for background, and background itself is not included in all classes of a dataset (e.g.
  ADE20k). The background label will be replaced by 255.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a MobileViTImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **segmentation_maps** (`ImageInput`, *optional*) --
  The segmentation maps to preprocess.
- **do_flip_channel_order** (`bool`, *kwargs*, *optional*, defaults to `self.do_flip_channel_order`) --
  Whether to flip the color channels from RGB to BGR or vice versa.
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

- **outputs** ([MobileViTForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/mobilevit#transformers.MobileViTForSemanticSegmentation)) --
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

Converts the output of [MobileViTForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/mobilevit#transformers.MobileViTForSemanticSegmentation) into semantic segmentation maps.

## MobileViTModel[[transformers.MobileViTModel]]

- **config** ([MobileViTConfig](/docs/transformers/v5.14.0/en/model_doc/mobilevit#transformers.MobileViTConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
- **expand_output** (`bool`, *optional*, defaults to `True`) --
  Whether to expand the output of the model using a 1x1 convolution. If `True`, the model will apply an additional
  1x1 convolution to expand the output channels from `config.neck_hidden_sizes[5]` to `config.neck_hidden_sizes[6]`.

The bare Mobilevit Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [MobileViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mobilevit#transformers.MobileViTImageProcessor). See `MobileViTImageProcessor.__call__()` for details (`processor_class` uses
  [MobileViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mobilevit#transformers.MobileViTImageProcessor) for processing images).
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.`BaseModelOutputWithPoolingAndNoAttention` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithPoolingAndNoAttention` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MobileViTConfig](/docs/transformers/v5.14.0/en/model_doc/mobilevit#transformers.MobileViTConfig)) and inputs.
The [MobileViTModel](/docs/transformers/v5.14.0/en/model_doc/mobilevit#transformers.MobileViTModel) forward method, overrides the `__call__` special method.

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

## MobileViTForImageClassification[[transformers.MobileViTForImageClassification]]

- **config** ([MobileViTConfig](/docs/transformers/v5.14.0/en/model_doc/mobilevit#transformers.MobileViTConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

MobileViT model with an image classification head on top (a linear layer on top of the pooled features), e.g. for
ImageNet.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [MobileViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mobilevit#transformers.MobileViTImageProcessor). See `MobileViTImageProcessor.__call__()` for details (`processor_class` uses
  [MobileViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mobilevit#transformers.MobileViTImageProcessor) for processing images).
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
elements depending on the configuration ([MobileViTConfig](/docs/transformers/v5.14.0/en/model_doc/mobilevit#transformers.MobileViTConfig)) and inputs.
The [MobileViTForImageClassification](/docs/transformers/v5.14.0/en/model_doc/mobilevit#transformers.MobileViTForImageClassification) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoImageProcessor, MobileViTForImageClassification
>>> import torch
>>> from datasets import load_dataset

>>> dataset = load_dataset("huggingface/cats-image")
>>> image = dataset["test"]["image"][0]

>>> image_processor = AutoImageProcessor.from_pretrained("google/mobilenet_v2_1.0_224")
>>> model = MobileViTForImageClassification.from_pretrained("google/mobilenet_v2_1.0_224")

>>> inputs = image_processor(image, return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # model predicts one of the 1000 ImageNet classes
>>> predicted_label = logits.argmax(-1).item()
>>> print(model.config.id2label[predicted_label])
...
```

## MobileViTForSemanticSegmentation[[transformers.MobileViTForSemanticSegmentation]]

- **config** ([MobileViTConfig](/docs/transformers/v5.14.0/en/model_doc/mobilevit#transformers.MobileViTConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

MobileViT model with a semantic segmentation head on top, e.g. for Pascal VOC.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [MobileViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mobilevit#transformers.MobileViTImageProcessor). See `MobileViTImageProcessor.__call__()` for details (`processor_class` uses
  [MobileViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mobilevit#transformers.MobileViTImageProcessor) for processing images).
- **labels** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Ground truth semantic segmentation maps for computing the loss. Indices should be in `[0, ...,
  config.num_labels - 1]`. If `config.num_labels > 1`, a classification loss is computed (Cross-Entropy).
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.[SemanticSegmenterOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.SemanticSegmenterOutput) or `tuple(torch.FloatTensor)`A [SemanticSegmenterOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.SemanticSegmenterOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MobileViTConfig](/docs/transformers/v5.14.0/en/model_doc/mobilevit#transformers.MobileViTConfig)) and inputs.
The [MobileViTForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/mobilevit#transformers.MobileViTForSemanticSegmentation) forward method, overrides the `__call__` special method.

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
>>> import httpx
>>> from io import BytesIO
>>> import torch
>>> from PIL import Image
>>> from transformers import AutoImageProcessor, MobileViTForSemanticSegmentation

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("apple/deeplabv3-mobilevit-small")
>>> model = MobileViTForSemanticSegmentation.from_pretrained("apple/deeplabv3-mobilevit-small")

>>> inputs = image_processor(images=image, return_tensors="pt")

>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> # logits are of shape (batch_size, num_labels, height, width)
>>> logits = outputs.logits
```
