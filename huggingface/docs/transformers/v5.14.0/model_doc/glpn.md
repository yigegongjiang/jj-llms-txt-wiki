# GLPN

This is a recently introduced model so the API hasn't been tested extensively. There may be some bugs or slight
breaking changes to fix it in the future. If you see something strange, file a [Github Issue](https://github.com/huggingface/transformers/issues/new?assignees=&labels=&template=bug-report.md&title).

## Overview

The GLPN model was proposed in [Global-Local Path Networks for Monocular Depth Estimation with Vertical CutDepth](https://huggingface.co/papers/2201.07436)  by Doyeon Kim, Woonghyun Ga, Pyungwhan Ahn, Donggyu Joo, Sehwan Chun, Junmo Kim.
GLPN combines [SegFormer](segformer)'s hierarchical mix-Transformer with a lightweight decoder for monocular depth estimation. The proposed decoder shows better performance than the previously proposed decoders, with considerably
less computational complexity.

The abstract from the paper is the following:

*Depth estimation from a single image is an important task that can be applied to various fields in computer vision, and has grown rapidly with the development of convolutional neural networks. In this paper, we propose a novel structure and training strategy for monocular depth estimation to further improve the prediction accuracy of the network. We deploy a hierarchical transformer encoder to capture and convey the global context, and design a lightweight yet powerful decoder to generate an estimated depth map while considering local connectivity. By constructing connected paths between multi-scale local features and the global decoding stream with our proposed selective feature fusion module, the network can integrate both representations and recover fine details. In addition, the proposed decoder shows better performance than the previously proposed decoders, with considerably less computational complexity. Furthermore, we improve the depth-specific augmentation method by utilizing an important observation in depth estimation to enhance the model. Our network achieves state-of-the-art performance over the challenging depth dataset NYU Depth V2. Extensive experiments have been conducted to validate and show the effectiveness of the proposed approach. Finally, our model shows better generalisation ability and robustness than other comparative models.*

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/glpn_architecture.jpg"
alt="drawing" width="600"/>

 Summary of the approach. Taken from the original paper. 

This model was contributed by [nielsr](https://huggingface.co/nielsr). The original code can be found [here](https://github.com/vinvino02/GLPDepth).

## Resources

A list of official Hugging Face and community (indicated by 🌎) resources to help you get started with GLPN.

- Demo notebooks for [GLPNForDepthEstimation](/docs/transformers/v5.14.0/en/model_doc/glpn#transformers.GLPNForDepthEstimation) can be found [here](https://github.com/NielsRogge/Transformers-Tutorials/tree/master/GLPN).
- [Monocular depth estimation task guide](../tasks/monocular_depth_estimation)

## GLPNConfig[[transformers.GLPNConfig]]

- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **num_encoder_blocks** (`int`, *optional*, defaults to 4) --
  The number of encoder blocks (i.e. stages in the Mix Transformer encoder).
- **depths** (`list[int]`, *optional*, defaults to `[2, 2, 2, 2]`) --
  The number of layers in each encoder block.
- **sr_ratios** (`list[int]`, *optional*, defaults to `[8, 4, 2, 1]`) --
  Sequence reduction ratios in each encoder block.
- **hidden_sizes** (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(32, 64, 160, 256)`) --
  Dimensionality (hidden size) at each stage of the model.
- **patch_sizes** (`list[int]`, *optional*, defaults to `[7, 3, 3, 3]`) --
  Patch size before each encoder block.
- **strides** (`list[int]`, *optional*, defaults to `[4, 2, 2, 2]`) --
  Stride before each encoder block.
- **num_attention_heads** (`list[int]`, *optional*, defaults to `[1, 2, 5, 8]`) --
  Number of attention heads for each attention layer in each block of the Transformer encoder.
- **mlp_ratios** (`list[int]`, *optional*, defaults to `[4, 4, 4, 4]`) --
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
- **drop_path_rate** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  Drop path rate for the patch fusion.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **decoder_hidden_size** (`int`, *optional*, defaults to 64) --
  The dimension of the decoder.
- **max_depth** (`int`, *optional*, defaults to 10) --
  The maximum depth of the decoder.
- **head_in_index** (`int`, *optional*, defaults to -1) --
  The index of the features to use in the head.

This is the configuration class to store the configuration of a GLPNModel. It is used to instantiate a Glpn
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [vinvino02/glpn-kitti](https://huggingface.co/vinvino02/glpn-kitti)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import GLPNModel, GLPNConfig

>>> # Initializing a GLPN vinvino02/glpn-kitti style configuration
>>> configuration = GLPNConfig()

>>> # Initializing a model from the vinvino02/glpn-kitti style configuration
>>> model = GLPNModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## GLPNImageProcessor[[transformers.GLPNImageProcessor]]

- **size_divisor** (`int`, *kwargs*, *optional*, defaults to 32) --
  When `do_resize` is `True`, images are resized so their height and width are rounded down to the closest
  multiple of `size_divisor`.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a GLPNImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **size_divisor** (`int`, *kwargs*, *optional*, defaults to 32) --
  When `do_resize` is `True`, images are resized so their height and width are rounded down to the closest
  multiple of `size_divisor`.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## GLPNImageProcessorPil[[transformers.GLPNImageProcessorPil]]

- **size_divisor** (`int`, *kwargs*, *optional*, defaults to 32) --
  When `do_resize` is `True`, images are resized so their height and width are rounded down to the closest
  multiple of `size_divisor`.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a GLPNImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **size_divisor** (`int`, *kwargs*, *optional*, defaults to 32) --
  When `do_resize` is `True`, images are resized so their height and width are rounded down to the closest
  multiple of `size_divisor`.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## GLPNModel[[transformers.GLPNModel]]

- **config** ([GLPNModel](/docs/transformers/v5.14.0/en/model_doc/glpn#transformers.GLPNModel)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Glpn Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [GLPNImageProcessor](/docs/transformers/v5.14.0/en/model_doc/glpn#transformers.GLPNImageProcessor). See `GLPNImageProcessor.__call__()` for details (`processor_class` uses
  [GLPNImageProcessor](/docs/transformers/v5.14.0/en/model_doc/glpn#transformers.GLPNImageProcessor) for processing images).
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.[BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or `tuple(torch.FloatTensor)`A [BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([GLPNConfig](/docs/transformers/v5.14.0/en/model_doc/glpn#transformers.GLPNConfig)) and inputs.
The [GLPNModel](/docs/transformers/v5.14.0/en/model_doc/glpn#transformers.GLPNModel) forward method, overrides the `__call__` special method.

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

## GLPNForDepthEstimation[[transformers.GLPNForDepthEstimation]]

- **config** ([GLPNForDepthEstimation](/docs/transformers/v5.14.0/en/model_doc/glpn#transformers.GLPNForDepthEstimation)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

GLPN Model transformer with a lightweight depth estimation head on top e.g. for KITTI, NYUv2.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [GLPNImageProcessor](/docs/transformers/v5.14.0/en/model_doc/glpn#transformers.GLPNImageProcessor). See `GLPNImageProcessor.__call__()` for details (`processor_class` uses
  [GLPNImageProcessor](/docs/transformers/v5.14.0/en/model_doc/glpn#transformers.GLPNImageProcessor) for processing images).
- **labels** (`torch.FloatTensor` of shape `(batch_size, height, width)`, *optional*) --
  Ground truth depth estimation maps for computing the loss.
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.[DepthEstimatorOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.DepthEstimatorOutput) or `tuple(torch.FloatTensor)`A [DepthEstimatorOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.DepthEstimatorOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([GLPNConfig](/docs/transformers/v5.14.0/en/model_doc/glpn#transformers.GLPNConfig)) and inputs.
The [GLPNForDepthEstimation](/docs/transformers/v5.14.0/en/model_doc/glpn#transformers.GLPNForDepthEstimation) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **predicted_depth** (`torch.FloatTensor` of shape `(batch_size, height, width)`) -- Predicted depth for each pixel.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, num_channels, height, width)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, patch_size,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import AutoImageProcessor, GLPNForDepthEstimation
>>> import torch
>>> import numpy as np
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("vinvino02/glpn-kitti")
>>> model = GLPNForDepthEstimation.from_pretrained("vinvino02/glpn-kitti")

>>> # prepare image for the model
>>> inputs = image_processor(images=image, return_tensors="pt")

>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> # interpolate to original size
>>> post_processed_output = image_processor.post_process_depth_estimation(
...     outputs,
...     target_sizes=[(image.height, image.width)],
... )

>>> # visualize the prediction
>>> predicted_depth = post_processed_output[0]["predicted_depth"]
>>> depth = predicted_depth * 255 / predicted_depth.max()
>>> depth = depth.detach().cpu().numpy()
>>> depth = Image.fromarray(depth.astype("uint8"))
```
