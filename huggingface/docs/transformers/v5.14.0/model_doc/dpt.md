# DPT

## Overview

The DPT model was proposed in [Vision Transformers for Dense Prediction](https://huggingface.co/papers/2103.13413) by René Ranftl, Alexey Bochkovskiy, Vladlen Koltun.
DPT is a model that leverages the [Vision Transformer (ViT)](vit) as backbone for dense prediction tasks like semantic segmentation and depth estimation.

The abstract from the paper is the following:

*We introduce dense vision transformers, an architecture that leverages vision transformers in place of convolutional networks as a backbone for dense prediction tasks. We assemble tokens from various stages of the vision transformer into image-like representations at various resolutions and progressively combine them into full-resolution predictions using a convolutional decoder. The transformer backbone processes representations at a constant and relatively high resolution and has a global receptive field at every stage. These properties allow the dense vision transformer to provide finer-grained and more globally coherent predictions when compared to fully-convolutional networks. Our experiments show that this architecture yields substantial improvements on dense prediction tasks, especially when a large amount of training data is available. For monocular depth estimation, we observe an improvement of up to 28% in relative performance when compared to a state-of-the-art fully-convolutional network. When applied to semantic segmentation, dense vision transformers set a new state of the art on ADE20K with 49.02% mIoU. We further show that the architecture can be fine-tuned on smaller datasets such as NYUv2, KITTI, and Pascal Context where it also sets the new state of the art.*

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/dpt_architecture.jpg"
alt="drawing" width="600"/>

 DPT architecture. Taken from the original paper. 

This model was contributed by [nielsr](https://huggingface.co/nielsr). The original code can be found [here](https://github.com/isl-org/DPT).

## Usage tips

DPT is compatible with the [AutoBackbone](/docs/transformers/v5.14.0/en/main_classes/backbones#transformers.AutoBackbone) class. This allows to use the DPT framework with various computer vision backbones available in the library, such as `VitDetBackbone` or `Dinov2Backbone`. One can create it as follows:

```python
from transformers import Dinov2Config, DPTConfig, DPTForDepthEstimation

# initialize with a Transformer-based backbone such as DINOv2
# in that case, we also specify `reshape_hidden_states=False` to get feature maps of shape (batch_size, num_channels, height, width)
backbone_config = Dinov2Config.from_pretrained("facebook/dinov2-base", out_features=["stage1", "stage2", "stage3", "stage4"], reshape_hidden_states=False)

config = DPTConfig(backbone_config=backbone_config)
model = DPTForDepthEstimation(config=config)
```

## Resources

A list of official Hugging Face and community (indicated by 🌎) resources to help you get started with DPT.

- Demo notebooks for [DPTForDepthEstimation](/docs/transformers/v5.14.0/en/model_doc/dpt#transformers.DPTForDepthEstimation) can be found [here](https://github.com/NielsRogge/Transformers-Tutorials/tree/master/DPT).

- [Semantic segmentation task guide](../tasks/semantic_segmentation)
- [Monocular depth estimation task guide](../tasks/monocular_depth_estimation)

If you're interested in submitting a resource to be included here, please feel free to open a Pull Request and we'll review it! The resource should ideally demonstrate something new instead of duplicating an existing resource.

## DPTConfig[[transformers.DPTConfig]]

- **hidden_size** (`int`, *optional*, defaults to `768`) --
  Dimension of the hidden representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `12`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `12`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **intermediate_size** (`int`, *optional*, defaults to `3072`) --
  Dimension of the MLP representations.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **hidden_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **attention_probs_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-12`) --
  The epsilon used by the layer normalization layers.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `384`) --
  The size (resolution) of each image.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) --
  The size (resolution) of each patch.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **is_hybrid** (`bool`, *optional*, defaults to `False`) --
  Whether to use a hybrid backbone. Useful in the context of loading DPT-Hybrid models.
- **qkv_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to add a bias to the queries, keys and values.
- **backbone_out_indices** (`list[int]`, *optional*, defaults to `[2, 5, 8, 11]`) --
  Indices of the intermediate hidden states to use from backbone.
- **readout_type** (`str`, *optional*, defaults to `"project"`) --
  The readout type to use when processing the readout token (CLS token) of the intermediate hidden states of
  the ViT backbone. Can be one of [`"ignore"`, `"add"`, `"project"`].
  - "ignore" simply ignores the CLS token.
  - "add" passes the information from the CLS token to all other tokens by adding the representations.
  - "project" passes information to the other tokens by concatenating the readout to all other tokens before
    projecting the
  representation to the original feature dimension D using a linear layer followed by a GELU non-linearity.
- **reassemble_factors** (`list[int]`, *optional*, defaults to `[4, 2, 1, 0.5]`) --
  The up/downsampling factors of the reassemble layers.
- **neck_hidden_sizes** (`list[str]`, *optional*, defaults to `[96, 192, 384, 768]`) --
  The hidden sizes to project to for the feature maps of the backbone.
- **fusion_hidden_size** (`int`, *optional*, defaults to 256) --
  The number of channels before fusion.
- **head_in_index** (`int`, *optional*, defaults to -1) --
  The index of the features to use in the heads.
- **use_batch_norm_in_fusion_residual** (`bool`, *optional*, defaults to `False`) --
  Whether to use batch normalization in the pre-activate residual units of the fusion blocks.
- **use_bias_in_fusion_residual** (`bool`, *optional*, defaults to `True`) --
  Whether to use bias in the pre-activate residual units of the fusion blocks.
- **add_projection** (`bool`, *optional*, defaults to `False`) --
  Whether to add a projection layer before the depth estimation head.
- **use_auxiliary_head** (`bool`, *optional*, defaults to `True`) --
  Whether to use an auxiliary head during training.
- **auxiliary_loss_weight** (`float`, *optional*, defaults to 0.4) --
  Weight of the cross-entropy loss of the auxiliary head.
- **semantic_loss_ignore_index** (`int`, *optional*, defaults to `255`) --
  The index that is ignored by the loss function of the semantic segmentation model.
- **semantic_classifier_dropout** (`float`, *optional*, defaults to 0.1) --
  The dropout ratio for the semantic classification head.
- **backbone_featmap_shape** (`list[int]`, *optional*, defaults to `[1, 1024, 24, 24]`) --
  Used only for the `hybrid` embedding type. The shape of the feature maps of the backbone.
- **neck_ignore_stages** (`list[int]`, *optional*, defaults to `[0, 1]`) --
  Used only for the `hybrid` embedding type. The stages of the readout layers to ignore.
- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **pooler_output_size** (`int`, *optional*) --
  Dimensionality of the pooler layer. If None, defaults to `hidden_size`.
- **pooler_act** (`str`, *optional*, defaults to `"tanh"`) --
  The activation function to be used by the pooler.

This is the configuration class to store the configuration of a DPTModel. It is used to instantiate a Dpt
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Intel/dpt-large](https://huggingface.co/Intel/dpt-large)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import DPTModel, DPTConfig

>>> # Initializing a DPT dpt-large style configuration
>>> configuration = DPTConfig()

>>> # Initializing a model from the dpt-large style configuration
>>> model = DPTModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## DPTImageProcessor[[transformers.DPTImageProcessor]]

- **ensure_multiple_of** (`int`, *kwargs*, *optional*, defaults to 1) --
  If `do_resize` is `True`, the image is resized to a size that is a multiple of this value. Can be overridden
  by `ensure_multiple_of` in `preprocess`.
- **keep_aspect_ratio** (`bool`, *kwargs*, *optional*, defaults to `False`) --
  If `True`, the image is resized to the largest possible size such that the aspect ratio is preserved. Can
  be overridden by `keep_aspect_ratio` in `preprocess`.
- **do_reduce_labels** (`bool`, *kwargs*, *optional*, defaults to `self.do_reduce_labels`) --
  Whether or not to reduce all label values of segmentation maps by 1. Usually used for datasets where 0
  is used for background, and background itself is not included in all classes of a dataset (e.g.
  ADE20k). The background label will be replaced by 255.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a DPTImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **segmentation_maps** (`ImageInput`, *optional*) --
  The segmentation maps to preprocess.
- **ensure_multiple_of** (`int`, *kwargs*, *optional*, defaults to 1) --
  If `do_resize` is `True`, the image is resized to a size that is a multiple of this value. Can be overridden
  by `ensure_multiple_of` in `preprocess`.
- **keep_aspect_ratio** (`bool`, *kwargs*, *optional*, defaults to `False`) --
  If `True`, the image is resized to the largest possible size such that the aspect ratio is preserved. Can
  be overridden by `keep_aspect_ratio` in `preprocess`.
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

## DPTImageProcessorPil[[transformers.DPTImageProcessorPil]]

- **ensure_multiple_of** (`int`, *kwargs*, *optional*, defaults to 1) --
  If `do_resize` is `True`, the image is resized to a size that is a multiple of this value. Can be overridden
  by `ensure_multiple_of` in `preprocess`.
- **keep_aspect_ratio** (`bool`, *kwargs*, *optional*, defaults to `False`) --
  If `True`, the image is resized to the largest possible size such that the aspect ratio is preserved. Can
  be overridden by `keep_aspect_ratio` in `preprocess`.
- **do_reduce_labels** (`bool`, *kwargs*, *optional*, defaults to `self.do_reduce_labels`) --
  Whether or not to reduce all label values of segmentation maps by 1. Usually used for datasets where 0
  is used for background, and background itself is not included in all classes of a dataset (e.g.
  ADE20k). The background label will be replaced by 255.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a DPTImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **segmentation_maps** (`ImageInput`, *optional*) --
  The segmentation maps to preprocess.
- **ensure_multiple_of** (`int`, *kwargs*, *optional*, defaults to 1) --
  If `do_resize` is `True`, the image is resized to a size that is a multiple of this value. Can be overridden
  by `ensure_multiple_of` in `preprocess`.
- **keep_aspect_ratio** (`bool`, *kwargs*, *optional*, defaults to `False`) --
  If `True`, the image is resized to the largest possible size such that the aspect ratio is preserved. Can
  be overridden by `keep_aspect_ratio` in `preprocess`.
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

- **outputs** ([DPTForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/dpt#transformers.DPTForSemanticSegmentation)) --
  Raw outputs of the model.
- **target_sizes** (`list[tuple]`, *optional*) --
  A list of tuples (`tuple[int, int]`) containing the target size (height, width) of each image in the
  batch. If unset, predictions will not be resized.
- **return_segmentation_scores** (`bool`, *optional*, defaults to `False`) --
  Whether to return segmentation scores alongside the segmentation map. When `True`, each element of
  the returned list is a `SemanticSegmentationPostProcessorOutput` with fields `segmentation`
  (class IDs, shape `(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`).`list[torch.Tensor]` or `list[SemanticSegmentationPostProcessorOutput]`When
`return_segmentation_scores=False` (default), a list of length `batch_size` where each item is a
segmentation map of shape `(height, width)` with class IDs. When `return_segmentation_scores=True`,
a list of `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape
`(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`). In both cases,
`(height, width)` corresponds to the target size (if `target_sizes` is specified).

Converts the output of [DPTForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/dpt#transformers.DPTForSemanticSegmentation) into semantic segmentation maps.

Converts the raw output of `DepthEstimatorOutput` into final depth predictions.

## DPTModel[[transformers.DPTModel]]

- **config** ([DPTConfig](/docs/transformers/v5.14.0/en/model_doc/dpt#transformers.DPTConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
- **add_pooling_layer** (`bool`, *optional*, defaults to `True`) --
  Whether to add a pooling layer

The bare Dpt Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [DPTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/dpt#transformers.DPTImageProcessor). See `DPTImageProcessor.__call__()` for details (`processor_class` uses
  [DPTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/dpt#transformers.DPTImageProcessor) for processing images).`BaseModelOutputWithPoolingAndIntermediateActivations` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithPoolingAndIntermediateActivations` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DPTConfig](/docs/transformers/v5.14.0/en/model_doc/dpt#transformers.DPTConfig)) and inputs.
The [DPTModel](/docs/transformers/v5.14.0/en/model_doc/dpt#transformers.DPTModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Last layer hidden-state of the first token of the sequence (classification token) after further processing
  through the layers used for the auxiliary pretraining task. E.g. for BERT-family of models, this returns
  the classification token after processing through a linear layer and a tanh activation function. The linear
  layer weights are trained from the next sentence prediction (classification) objective during pretraining.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **intermediate_activations** (`tuple(torch.FloatTensor)`, *optional*) -- Intermediate activations that can be used to compute hidden states of the model at various layers.

Example:

```python
```

## DPTForDepthEstimation[[transformers.DPTForDepthEstimation]]

- **config** ([DPTForDepthEstimation](/docs/transformers/v5.14.0/en/model_doc/dpt#transformers.DPTForDepthEstimation)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

DPT Model with a depth estimation head on top (consisting of 3 convolutional layers) e.g. for KITTI, NYUv2.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [DPTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/dpt#transformers.DPTImageProcessor). See `DPTImageProcessor.__call__()` for details (`processor_class` uses
  [DPTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/dpt#transformers.DPTImageProcessor) for processing images).
- **labels** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Ground truth depth estimation maps for computing the loss.[DepthEstimatorOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.DepthEstimatorOutput) or `tuple(torch.FloatTensor)`A [DepthEstimatorOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.DepthEstimatorOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DPTConfig](/docs/transformers/v5.14.0/en/model_doc/dpt#transformers.DPTConfig)) and inputs.
The [DPTForDepthEstimation](/docs/transformers/v5.14.0/en/model_doc/dpt#transformers.DPTForDepthEstimation) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoImageProcessor, DPTForDepthEstimation
>>> import torch
>>> import numpy as np
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("Intel/dpt-large")
>>> model = DPTForDepthEstimation.from_pretrained("Intel/dpt-large")

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

## DPTForSemanticSegmentation[[transformers.DPTForSemanticSegmentation]]

- **config** ([DPTConfig](/docs/transformers/v5.14.0/en/model_doc/dpt#transformers.DPTConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Dpt Model with a semantic segmentation head on top e.g. for ADE20K, CityScapes.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [DPTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/dpt#transformers.DPTImageProcessor). See `DPTImageProcessor.__call__()` for details (`processor_class` uses
  [DPTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/dpt#transformers.DPTImageProcessor) for processing images).
- **labels** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Ground truth semantic segmentation maps for computing the loss. Indices should be in `[0, ...,
  config.num_labels - 1]`. If `config.num_labels > 1`, a classification loss is computed (Cross-Entropy).</paramsdesc><rettype>[SemanticSegmenterOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.SemanticSegmenterOutput) or `tuple(torch.FloatTensor)`A [SemanticSegmenterOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.SemanticSegmenterOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DPTConfig](/docs/transformers/v5.14.0/en/model_doc/dpt#transformers.DPTConfig)) and inputs.
The [DPTForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/dpt#transformers.DPTForSemanticSegmentation) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoImageProcessor, DPTForSemanticSegmentation
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("Intel/dpt-large-ade")
>>> model = DPTForSemanticSegmentation.from_pretrained("Intel/dpt-large-ade")

>>> inputs = image_processor(images=image, return_tensors="pt")

>>> outputs = model(**inputs)
>>> logits = outputs.logits
```
