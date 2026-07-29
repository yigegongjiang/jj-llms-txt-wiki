# CHMv2

## Overview

The Canopy Height Maps v2 (CHMv2) model was proposed in [CHMv2: Improvements in Global Canopy Height Mapping using DINOv3](https://huggingface.co/papers/2603.06382). Building on our [original high-resolution canopy height maps](https://sustainability.atmeta.com/blog/2024/04/22/using-artificial-intelligence-to-map-the-earths-forests/) released in 2024, CHMv2 delivers substantial improvements in accuracy, detail, and global consistency by leveraging DINOv3, Meta's self-supervised vision model.

You can find more information [here](http://ai.meta.com/blog/world-resources-institute-dino-canopy-height-maps-v2), and the original code [here](https://github.com/facebookresearch/dinov3).

The abstract from the paper is the following:

*Accurate canopy height information is essential for quantifying forest carbon, monitoring restoration and degradation, and assessing habitat structure, yet high-fidelity measurements from airborne laser scanning (ALS) remain unevenly available globally. Here we present CHMv2, a global, meter-resolution canopy height map derived from high-resolution optical satellite imagery using a depth-estimation model built on DINOv3 and trained against ALS canopy height models. Compared to existing products, CHMv2 substantially improves accuracy, reduces bias in tall forests, and better preserves fine-scale structure such as canopy edges and gaps. These gains are enabled by a large expansion of geographically diverse training data, automated data curation and registration, and a loss formulation and data sampling strategy tailored to canopy height distributions. We validate CHMv2 against independent ALS test sets and against tens of millions of GEDI and ICESat-2 observations, demonstrating consistent performance across major forest biomes.*

## Usage examples

Run inference on an image with the following code:

```python
import torch
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForDepthEstimation

processor = AutoImageProcessor.from_pretrained("facebook/dinov3-vitl16-chmv2-dpt-head")
model = AutoModelForDepthEstimation.from_pretrained("facebook/dinov3-vitl16-chmv2-dpt-head", device_map="auto")

image = Image.open("image.tif")
inputs = processor(images=image, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

depth = processor.post_process_depth_estimation(
    outputs, target_sizes=[(image.height, image.width)]
)[0]["predicted_depth"]
```

## CHMv2Config[[transformers.CHMv2Config]]

- **backbone_config** (`Union[dict, "PreTrainedConfig"]`, *optional*) --
  The configuration of the backbone model. Only DINOv3ViTConfig is currently supported.
- **patch_size** (`int`, *optional*, defaults to 16) --
  The patch size used by the backbone vision transformer.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **reassemble_factors** (`list[float]`, *optional*, defaults to `[4, 2, 1, 0.5]`) --
  The up/downsampling factors of the reassemble layers.
- **post_process_channels** (`list[int]`, *optional*, defaults to `[128, 256, 512, 1024]`) --
  The output channel sizes of the reassemble stage for each backbone feature level.
- **fusion_hidden_size** (`int`, *optional*, defaults to 256) --
  The number of channels before fusion.
- **head_hidden_size** (`int`, *optional*, defaults to 128) --
  The number of channels in the hidden layer of the depth estimation head.
- **number_output_channels** (`int`, *optional*, defaults to 256) --
  Number of output channels for the CHMv2 head (number of depth bins).
- **readout_type** (`str`, *optional*, defaults to `"project"`) --
  Type of readout operation for the CLS token. One of `["ignore", "add", "project"]`.
- **min_depth** (`float`, *optional*, defaults to 0.001) --
  The minimum depth value for depth bin calculation.
- **max_depth** (`float`, *optional*, defaults to 96.0) --
  The maximum depth value for depth bin calculation.
- **bins_strategy** (`str`, *optional*, defaults to `"chmv2_mixlog"`) --
  The strategy for depth bins distribution. One of `["linear", "log", "chmv2_mixlog"]`.
- **norm_strategy** (`str`, *optional*, defaults to `"chmv2_mixlog"`) --
  The normalization strategy for depth prediction. One of `["linear", "softmax", "sigmoid", "chmv2_mixlog"]`.

This is the configuration class to store the configuration of a Chmv2Model. It is used to instantiate a Chmv2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/dinov3-vitl16-chmv2-dpt-head](https://huggingface.co/facebook/dinov3-vitl16-chmv2-dpt-head)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import CHMv2Config, CHMv2ForDepthEstimation

>>> configuration = CHMv2Config()
>>> model = CHMv2ForDepthEstimation(configuration)
>>> configuration = model.config
```

## CHMv2ImageProcessor[[transformers.CHMv2ImageProcessor]]

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
Constructs a CHMv2ImageProcessor image processor.

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

- **outputs** (`DepthEstimatorOutput`) --
  Raw outputs of the model.
- **target_sizes** (`TensorType` or `List[Tuple[int, int]]`, *optional*) --
  Tensor of shape `(batch_size, 2)` or list of tuples (`Tuple[int, int]`) containing the target size
  (height, width) of each image in the batch. If left to None, predictions will not be resized.`List[Dict[str, TensorType]]`A list of dictionaries of tensors representing the processed depth
predictions.

Converts the raw output of `DepthEstimatorOutput` into final depth predictions and depth PIL images.
Only supports PyTorch.

## CHMv2ForDepthEstimation[[transformers.CHMv2ForDepthEstimation]]

- **config** ([CHMv2Config](/docs/transformers/v5.14.0/en/model_doc/chmv2#transformers.CHMv2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

CHMv2 Model with a depth estimation head on top (consisting of convolutional layers) e.g. for canopy height
estimation.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [CHMv2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/chmv2#transformers.CHMv2ImageProcessor). See `CHMv2ImageProcessor.__call__()` for details (`processor_class` uses
  [CHMv2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/chmv2#transformers.CHMv2ImageProcessor) for processing images).
- **labels** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Ground truth depth estimation maps for computing the loss.[DepthEstimatorOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.DepthEstimatorOutput) or `tuple(torch.FloatTensor)`A [DepthEstimatorOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.DepthEstimatorOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([CHMv2Config](/docs/transformers/v5.14.0/en/model_doc/chmv2#transformers.CHMv2Config)) and inputs.
The [CHMv2ForDepthEstimation](/docs/transformers/v5.14.0/en/model_doc/chmv2#transformers.CHMv2ForDepthEstimation) forward method, overrides the `__call__` special method.

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

Example:

```python
>>> from transformers import AutoImageProcessor, CHMv2ForDepthEstimation
>>> import torch
>>> from PIL import Image
>>> import httpx
    >>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read())).convert("RGB")

>>> processor = AutoImageProcessor.from_pretrained("facebook/dinov3-vitl16-chmv2-dpt-head")
>>> model = CHMv2ForDepthEstimation.from_pretrained("facebook/dinov3-vitl16-chmv2-dpt-head")

>>> device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
>>> model.to(device)

>>> # prepare image for the model
>>> inputs = processor(images=image, return_tensors="pt").to(device)

>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> # interpolate to original size
>>> post_processed_output = processor.post_process_depth_estimation(
...     outputs, [(image.height, image.width)],
... )
>>> predicted_depth = post_processed_output[0]["predicted_depth"]
```
