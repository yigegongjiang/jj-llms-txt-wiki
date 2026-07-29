# EfficientNet

## Overview

The EfficientNet model was proposed in [EfficientNet: Rethinking Model Scaling for Convolutional Neural Networks](https://huggingface.co/papers/1905.11946)
by Mingxing Tan and Quoc V. Le. EfficientNets are a family of image classification models, which achieve state-of-the-art accuracy, yet being an order-of-magnitude smaller and faster than previous models.

The abstract from the paper is the following:

*Convolutional Neural Networks (ConvNets) are commonly developed at a fixed resource budget, and then scaled up for better accuracy if more resources are available. In this paper, we systematically study model scaling and identify that carefully balancing network depth, width, and resolution can lead to better performance. Based on this observation, we propose a new scaling method that uniformly scales all dimensions of depth/width/resolution using a simple yet highly effective compound coefficient. We demonstrate the effectiveness of this method on scaling up MobileNets and ResNet.
To go even further, we use neural architecture search to design a new baseline network and scale it up to obtain a family of models, called EfficientNets, which achieve much better accuracy and efficiency than previous ConvNets. In particular, our EfficientNet-B7 achieves state-of-the-art 84.3% top-1 accuracy on ImageNet, while being 8.4x smaller and 6.1x faster on inference than the best existing ConvNet. Our EfficientNets also transfer well and achieve state-of-the-art accuracy on CIFAR-100 (91.7%), Flowers (98.8%), and 3 other transfer learning datasets, with an order of magnitude fewer parameters.*

This model was contributed by [adirik](https://huggingface.co/adirik).
The original code can be found [here](https://github.com/tensorflow/tpu/tree/master/models/official/efficientnet).

## EfficientNetConfig[[transformers.EfficientNetConfig]]

- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `600`) --
  The size (resolution) of each image.
- **width_coefficient** (`float`, *optional*, defaults to 2.0) --
  Scaling coefficient for network width at each stage.
- **depth_coefficient** (`float`, *optional*, defaults to 3.1) --
  Scaling coefficient for network depth at each stage.
- **depth_divisor** (`int`, *optional*, defaults to 8) --
  A unit of network width.
- **kernel_sizes** (`list[int]`, *optional*, defaults to `[3, 3, 5, 3, 5, 5, 3]`) --
  List of kernel sizes to be used in each block.
- **in_channels** (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(32, 16, 24, 40, 80, 112, 192)`) --
  The number of input channels.
- **out_channels** (`list[int]`, *optional*, defaults to `[16, 24, 40, 80, 112, 192, 320]`) --
  List of output channel sizes to be used in each block for convolutional layers.
- **depthwise_padding** (`list[int]`, *optional*, defaults to `[]`) --
  List of block indices with square padding.
- **strides** (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(1, 2, 2, 2, 1, 2, 1)`) --
  Stride at each stage of the model.
- **num_block_repeats** (`list[int]`, *optional*, defaults to `[1, 2, 2, 3, 3, 4, 1]`) --
  List of the number of times each block is to repeated.
- **expand_ratios** (`list[int]`, *optional*, defaults to `[1, 6, 6, 6, 6, 6, 6]`) --
  List of scaling coefficient of each block.
- **squeeze_expansion_ratio** (`float`, *optional*, defaults to 0.25) --
  Squeeze expansion ratio.
- **hidden_act** (`str`, *optional*, defaults to `swish`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **hidden_dim** (`int`, *optional*, defaults to `2560`) --
  Dimension of the hidden representations.
- **pooling_type** (`str` or `function`, *optional*, defaults to `"mean"`) --
  Type of final pooling to be applied before the dense classification head. Available options are [`"mean"`,
  `"max"`]
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **batch_norm_eps** (`float`, *optional*, defaults to `0.001`) --
  The epsilon used by the batch normalization layers.
- **batch_norm_momentum** (`float`, *optional*, defaults to 0.99) --
  The momentum used by the batch normalization layers.
- **dropout_rate** (`Union[float, int]`, *optional*, defaults to `0.5`) --
  The ratio for all dropout layers.
- **drop_connect_rate** (`float`, *optional*, defaults to 0.2) --
  The drop rate for skip connections.

This is the configuration class to store the configuration of a EfficientNetModel. It is used to instantiate a Efficientnet
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/efficientnet-b7](https://huggingface.co/google/efficientnet-b7)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import EfficientNetConfig, EfficientNetModel

>>> # Initializing a EfficientNet efficientnet-b7 style configuration
>>> configuration = EfficientNetConfig()

>>> # Initializing a model (with random weights) from the efficientnet-b7 style configuration
>>> model = EfficientNetModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## EfficientNetImageProcessor[[transformers.EfficientNetImageProcessor]]

- **rescale_offset** (`bool`, *kwargs*, *optional*, defaults to `self.rescale_offset`) --
  Whether to rescale the image between [-max_range/2, scale_range/2] instead of [0, scale_range].
- **include_top** (`bool`, *kwargs*, *optional*, defaults to `self.include_top`) --
  Normalize the image again with the standard deviation only for image classification if set to True.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a EfficientNetImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## EfficientNetImageProcessorPil[[transformers.EfficientNetImageProcessorPil]]

- **rescale_offset** (`bool`, *kwargs*, *optional*, defaults to `self.rescale_offset`) --
  Whether to rescale the image between [-max_range/2, scale_range/2] instead of [0, scale_range].
- **include_top** (`bool`, *kwargs*, *optional*, defaults to `self.include_top`) --
  Normalize the image again with the standard deviation only for image classification if set to True.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a EfficientNetImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## EfficientNetModel[[transformers.EfficientNetModel]]

- **config** ([EfficientNetConfig](/docs/transformers/v5.14.0/en/model_doc/efficientnet#transformers.EfficientNetConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Efficientnet Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [EfficientNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/efficientnet#transformers.EfficientNetImageProcessor). See `EfficientNetImageProcessor.__call__()` for details (`processor_class` uses
  [EfficientNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/efficientnet#transformers.EfficientNetImageProcessor) for processing images).
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.`BaseModelOutputWithPoolingAndNoAttention` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithPoolingAndNoAttention` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([EfficientNetConfig](/docs/transformers/v5.14.0/en/model_doc/efficientnet#transformers.EfficientNetConfig)) and inputs.
The [EfficientNetModel](/docs/transformers/v5.14.0/en/model_doc/efficientnet#transformers.EfficientNetModel) forward method, overrides the `__call__` special method.

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

## EfficientNetForImageClassification[[transformers.EfficientNetForImageClassification]]

- **config** ([EfficientNetForImageClassification](/docs/transformers/v5.14.0/en/model_doc/efficientnet#transformers.EfficientNetForImageClassification)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

EfficientNet Model with an image classification head on top (a linear layer on top of the pooled features), e.g.
for ImageNet.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [EfficientNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/efficientnet#transformers.EfficientNetImageProcessor). See `EfficientNetImageProcessor.__call__()` for details (`processor_class` uses
  [EfficientNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/efficientnet#transformers.EfficientNetImageProcessor) for processing images).
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
elements depending on the configuration ([EfficientNetConfig](/docs/transformers/v5.14.0/en/model_doc/efficientnet#transformers.EfficientNetConfig)) and inputs.
The [EfficientNetForImageClassification](/docs/transformers/v5.14.0/en/model_doc/efficientnet#transformers.EfficientNetForImageClassification) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoImageProcessor, EfficientNetForImageClassification
>>> import torch
>>> from datasets import load_dataset

>>> dataset = load_dataset("huggingface/cats-image")
>>> image = dataset["test"]["image"][0]

>>> image_processor = AutoImageProcessor.from_pretrained("google/efficientnet-b7")
>>> model = EfficientNetForImageClassification.from_pretrained("google/efficientnet-b7")

>>> inputs = image_processor(image, return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # model predicts one of the 1000 ImageNet classes
>>> predicted_label = logits.argmax(-1).item()
>>> print(model.config.id2label[predicted_label])
...
```
