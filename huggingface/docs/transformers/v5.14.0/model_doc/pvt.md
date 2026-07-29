# Pyramid Vision Transformer (PVT)

## Overview

The PVT model was proposed in
[Pyramid Vision Transformer: A Versatile Backbone for Dense Prediction without Convolutions](https://huggingface.co/papers/2102.12122)
by Wenhai Wang, Enze Xie, Xiang Li, Deng-Ping Fan, Kaitao Song, Ding Liang, Tong Lu, Ping Luo, Ling Shao. The PVT is a type of
vision transformer that utilizes a pyramid structure to make it an effective backbone for dense prediction tasks. Specifically
it allows for more fine-grained inputs (4 x 4 pixels per patch) to be used, while simultaneously shrinking the sequence length
of the Transformer as it deepens - reducing the computational cost. Additionally, a spatial-reduction attention (SRA) layer
is used to further reduce the resource consumption when learning high-resolution features.

The abstract from the paper is the following:

*Although convolutional neural networks (CNNs) have achieved great success in computer vision, this work investigates a
simpler, convolution-free backbone network useful for many dense prediction tasks. Unlike the recently proposed Vision
Transformer (ViT) that was designed for image classification specifically, we introduce the Pyramid Vision Transformer
(PVT), which overcomes the difficulties of porting Transformer to various dense prediction tasks. PVT has several
merits compared to current state of the arts. Different from ViT that typically yields low resolution outputs and
incurs high computational and memory costs, PVT not only can be trained on dense partitions of an image to achieve high
output resolution, which is important for dense prediction, but also uses a progressive shrinking pyramid to reduce the
computations of large feature maps. PVT inherits the advantages of both CNN and Transformer, making it a unified
backbone for various vision tasks without convolutions, where it can be used as a direct replacement for CNN backbones.
We validate PVT through extensive experiments, showing that it boosts the performance of many downstream tasks, including
object detection, instance and semantic segmentation. For example, with a comparable number of parameters, PVT+RetinaNet
achieves 40.4 AP on the COCO dataset, surpassing ResNet50+RetinNet (36.3 AP) by 4.1 absolute AP (see Figure 2). We hope
that PVT could serve as an alternative and useful backbone for pixel-level predictions and facilitate future research.*

This model was contributed by [Xrenya](https://huggingface.co/Xrenya). The original code can be found [here](https://github.com/whai362/PVT).

- PVTv1 on ImageNet-1K

| **Model variant**  |**Size** |**Acc@1**|**Params (M)**|
|--------------------|:-------:|:-------:|:------------:|
| PVT-Tiny           |    224  |   75.1  |     13.2     |
| PVT-Small          |    224  |   79.8  |     24.5     |
| PVT-Medium         |    224  |   81.2  |     44.2     |
| PVT-Large          |    224  |   81.7  |     61.4     |

## PvtConfig[[transformers.PvtConfig]]

- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `224`) --
  The size (resolution) of each image.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **num_encoder_blocks** (`int`, *optional*, defaults to 4) --
  The number of encoder blocks (i.e. stages in the Mix Transformer encoder).
- **depths** (`list[int]`, *optional*, defaults to `[2, 2, 2, 2]`) --
  The number of layers in each encoder block.
- **sequence_reduction_ratios** (`list[int]`, *optional*, defaults to `[8, 4, 2, 1]`) --
  Sequence reduction ratios in each encoder block.
- **hidden_sizes** (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(64, 128, 320, 512)`) --
  Dimensionality (hidden size) at each stage of the model.
- **patch_sizes** (`list[int]`, *optional*, defaults to `[4, 2, 2, 2]`) --
  Patch size before each encoder block.
- **strides** (`list[int]`, *optional*, defaults to `[4, 2, 2, 2]`) --
  Stride before each encoder block.
- **num_attention_heads** (`list[int]`, *optional*, defaults to `[1, 2, 5, 8]`) --
  Number of attention heads for each attention layer in each block of the Transformer encoder.
- **mlp_ratios** (`list[int]`, *optional*, defaults to `[8, 8, 4, 4]`) --
  Ratio of the size of the hidden layer compared to the size of the input layer of the Mix FFNs in the
  encoder blocks.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **hidden_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **attention_probs_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **drop_path_rate** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  Drop path rate for the patch fusion.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **qkv_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to add a bias to the queries, keys and values.
- **num_labels** (`'int'`, *optional*, defaults to 1000) --
  The number of classes.

This is the configuration class to store the configuration of a PvtModel. It is used to instantiate a Pvt
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Xrenya/pvt-tiny-224](https://huggingface.co/Xrenya/pvt-tiny-224)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import PvtModel, PvtConfig

>>> # Initializing a PVT Xrenya/pvt-tiny-224 style configuration
>>> configuration = PvtConfig()

>>> # Initializing a model from the Xrenya/pvt-tiny-224 style configuration
>>> model = PvtModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## PvtImageProcessor[[transformers.PvtImageProcessor]]

- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a PvtImageProcessor image processor.

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

## PvtImageProcessorPil[[transformers.PvtImageProcessorPil]]

- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a PvtImageProcessor image processor.

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

## PvtForImageClassification[[transformers.PvtForImageClassification]]

- **config** ([PvtConfig](/docs/transformers/v5.14.0/en/model_doc/pvt#transformers.PvtConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Pvt Model transformer with an image classification head on top (a linear layer on top of the final hidden state of
the [CLS] token) e.g. for ImageNet.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [PvtImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pvt#transformers.PvtImageProcessor). See `PvtImageProcessor.__call__()` for details (`processor_class` uses
  [PvtImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pvt#transformers.PvtImageProcessor) for processing images).
- **labels** (`torch.LongTensor` of shape `(batch_size,)`, *optional*) --
  Labels for computing the image classification/regression loss. Indices should be in `[0, ...,
  config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If
  `config.num_labels > 1` a classification loss is computed (Cross-Entropy).
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.[ImageClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutput) or `tuple(torch.FloatTensor)`A [ImageClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PvtConfig](/docs/transformers/v5.14.0/en/model_doc/pvt#transformers.PvtConfig)) and inputs.
The [PvtForImageClassification](/docs/transformers/v5.14.0/en/model_doc/pvt#transformers.PvtForImageClassification) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoImageProcessor, PvtForImageClassification
>>> import torch
>>> from datasets import load_dataset

>>> dataset = load_dataset("huggingface/cats-image")
>>> image = dataset["test"]["image"][0]

>>> image_processor = AutoImageProcessor.from_pretrained("Xrenya/pvt-tiny-224")
>>> model = PvtForImageClassification.from_pretrained("Xrenya/pvt-tiny-224")

>>> inputs = image_processor(image, return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # model predicts one of the 1000 ImageNet classes
>>> predicted_label = logits.argmax(-1).item()
>>> print(model.config.id2label[predicted_label])
...
```

## PvtModel[[transformers.PvtModel]]

- **config** ([PvtConfig](/docs/transformers/v5.14.0/en/model_doc/pvt#transformers.PvtConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Pvt Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [PvtImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pvt#transformers.PvtImageProcessor). See `PvtImageProcessor.__call__()` for details (`processor_class` uses
  [PvtImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pvt#transformers.PvtImageProcessor) for processing images).
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.[BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or `tuple(torch.FloatTensor)`A [BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PvtConfig](/docs/transformers/v5.14.0/en/model_doc/pvt#transformers.PvtConfig)) and inputs.
The [PvtModel](/docs/transformers/v5.14.0/en/model_doc/pvt#transformers.PvtModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
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
