# TextNet

## Overview

The TextNet model was proposed in [FAST: Faster Arbitrarily-Shaped Text Detector with Minimalist Kernel Representation](https://huggingface.co/papers/2111.02394) by Zhe Chen, Jiahao Wang, Wenhai Wang, Guo Chen, Enze Xie, Ping Luo, Tong Lu. TextNet is a vision backbone useful for text detection tasks. It is the result of neural architecture search (NAS) on backbones with reward function as text detection task (to provide powerful features for text detection).

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/fast_architecture.png"
alt="drawing" width="600"/>

 TextNet backbone as part of FAST. Taken from the original paper. 

This model was contributed by [Raghavan](https://huggingface.co/Raghavan), [jadechoghari](https://huggingface.co/jadechoghari) and [nielsr](https://huggingface.co/nielsr).

## Usage tips

TextNet is mainly used as a backbone network for the architecture search of text detection. Each stage of the backbone network is comprised of a stride-2 convolution and searchable blocks.
Specifically, we present a layer-level candidate set, defined as {conv3×3, conv1×3, conv3×1, identity}. As the 1×3 and 3×1 convolutions have asymmetric kernels and oriented structure priors, they may help to capture the features of extreme aspect-ratio and rotated text lines.

TextNet is the backbone for Fast, but can also be used as an efficient text/image classification, we add a `TextNetForImageClassification` as is it would allow people to train an image classifier on top of the pre-trained textnet weights

## TextNetConfig[[transformers.TextNetConfig]]

- **stem_kernel_size** (`int`, *optional*, defaults to 3) --
  The kernel size for the initial convolution layer.
- **stem_stride** (`int`, *optional*, defaults to 2) --
  The stride for the initial convolution layer.
- **stem_num_channels** (`int`, *optional*, defaults to 3) --
  The num of channels in input for the initial convolution layer.
- **stem_out_channels** (`int`, *optional*, defaults to 64) --
  The num of channels in out for the initial convolution layer.
- **stem_act_func** (`str`, *optional*, defaults to `"relu"`) --
  The activation function for the initial convolution layer.
- **image_size** (`Union[list[int], tuple[int, int], int]`, *optional*, defaults to `(640, 640)`) --
  The size (resolution) of each image.
- **conv_layer_kernel_sizes** (`list[list[list[int]]]`, *optional*) --
  A list of stage-wise kernel sizes. If `None`, defaults to:
  `[[[3, 3], [3, 3], [3, 3]], [[3, 3], [1, 3], [3, 3], [3, 1]], [[3, 3], [3, 3], [3, 1], [1, 3]], [[3, 3], [3, 1], [1, 3], [3, 3]]]`.
- **conv_layer_strides** (`list[list[int]]`, *optional*) --
  A list of stage-wise strides. If `None`, defaults to:
  `[[1, 2, 1], [2, 1, 1, 1], [2, 1, 1, 1], [2, 1, 1, 1]]`.
- **hidden_sizes** (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(64, 64, 128, 256, 512)`) --
  Dimensionality (hidden size) at each stage of the model.
- **batch_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the batch normalization layers.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a TextNetModel. It is used to instantiate a Textnet
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [czczup/textnet-base](https://huggingface.co/czczup/textnet-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import TextNetConfig, TextNetBackbone

>>> # Initializing a TextNetConfig
>>> configuration = TextNetConfig()

>>> # Initializing a model (with random weights)
>>> model = TextNetBackbone(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## TextNetImageProcessor[[transformers.TextNetImageProcessor]]

- **size_divisor** (`int`, *kwargs*, *optional*, defaults to `self.size_divisor`) --
  Ensures height and width are rounded to a multiple of this value after resizing.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a TextNetImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **size_divisor** (`int`, *kwargs*, *optional*, defaults to `self.size_divisor`) --
  Ensures height and width are rounded to a multiple of this value after resizing.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## TextNetImageProcessorPil[[transformers.TextNetImageProcessorPil]]

- **size_divisor** (`int`, *kwargs*, *optional*, defaults to `self.size_divisor`) --
  Ensures height and width are rounded to a multiple of this value after resizing.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a TextNetImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **size_divisor** (`int`, *kwargs*, *optional*, defaults to `self.size_divisor`) --
  Ensures height and width are rounded to a multiple of this value after resizing.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## TextNetModel[[transformers.TextNetModel]]

- **config** ([TextNetModel](/docs/transformers/v5.14.0/en/model_doc/textnet#transformers.TextNetModel)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Textnet Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "output_hidden_states", "val": ": bool | None = None"}, {"name": "return_dict", "val": ": bool | None = None"}, {"name": "**kwargs", "val": ""}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [TextNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/textnet#transformers.TextNetImageProcessor). See `TextNetImageProcessor.__call__()` for details (`processor_class` uses
  [TextNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/textnet#transformers.TextNetImageProcessor) for processing images).
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.`BaseModelOutputWithPoolingAndNoAttention` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithPoolingAndNoAttention` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([TextNetConfig](/docs/transformers/v5.14.0/en/model_doc/textnet#transformers.TextNetConfig)) and inputs.
The [TextNetModel](/docs/transformers/v5.14.0/en/model_doc/textnet#transformers.TextNetModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Last layer hidden-state after a pooling operation on the spatial dimensions.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, num_channels, height, width)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

## TextNetForImageClassification[[transformers.TextNetForImageClassification]]

- **config** ([TextNetForImageClassification](/docs/transformers/v5.14.0/en/model_doc/textnet#transformers.TextNetForImageClassification)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

TextNet Model with an image classification head on top (a linear layer on top of the pooled features), e.g. for
ImageNet.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [TextNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/textnet#transformers.TextNetImageProcessor). See `TextNetImageProcessor.__call__()` for details (`processor_class` uses
  [TextNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/textnet#transformers.TextNetImageProcessor) for processing images).
- **labels** (`torch.LongTensor` of shape `(batch_size,)`, *optional*) --
  Labels for computing the image classification/regression loss. Indices should be in `[0, ...,
  config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If
  `config.num_labels > 1` a classification loss is computed (Cross-Entropy).
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.[ImageClassifierOutputWithNoAttention](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutputWithNoAttention) or `tuple(torch.FloatTensor)`A [ImageClassifierOutputWithNoAttention](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutputWithNoAttention) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([TextNetConfig](/docs/transformers/v5.14.0/en/model_doc/textnet#transformers.TextNetConfig)) and inputs.
The [TextNetForImageClassification](/docs/transformers/v5.14.0/en/model_doc/textnet#transformers.TextNetForImageClassification) forward method, overrides the `__call__` special method.

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
>>> from transformers import TextNetForImageClassification, TextNetImageProcessor
>>> from PIL import Image

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> processor = TextNetImageProcessor.from_pretrained("czczup/textnet-base")
>>> model = TextNetForImageClassification.from_pretrained("czczup/textnet-base")

>>> inputs = processor(images=image, return_tensors="pt")
>>> with torch.no_grad():
...     outputs = model(**inputs)
>>> outputs.logits.shape
torch.Size([1, 2])
```
