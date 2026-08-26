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

#### transformers.CHMv2Config[[transformers.CHMv2Config]]

```python
transformers.CHMv2Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, backbone_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, patch_size: int = 16, initializer_range: float = 0.02, reassemble_factors: list[float | int] | None = None, post_process_channels: list[int] | None = None, fusion_hidden_size: int = 256, head_hidden_size: int = 128, number_output_channels: int = 256, readout_type: str = 'project', min_depth: float = 0.001, max_depth: float = 96.0, bins_strategy: typing.Literal['linear', 'log', 'chmv2_mixlog'] = 'chmv2_mixlog', norm_strategy: typing.Literal['linear', 'softmax', 'sigmoid', 'chmv2_mixlog'] = 'chmv2_mixlog')
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/chmv2/configuration_chmv2.py#L33)

**Parameters:**

backbone_config (`Union[dict, "PreTrainedConfig"]`, *optional*) : The configuration of the backbone model. Only DINOv3ViTConfig is currently supported.

patch_size (`int`, *optional*, defaults to 16) : The patch size used by the backbone vision transformer.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

reassemble_factors (`list[float]`, *optional*, defaults to `[4, 2, 1, 0.5]`) : The up/downsampling factors of the reassemble layers.

post_process_channels (`list[int]`, *optional*, defaults to `[128, 256, 512, 1024]`) : The output channel sizes of the reassemble stage for each backbone feature level.

fusion_hidden_size (`int`, *optional*, defaults to 256) : The number of channels before fusion.

head_hidden_size (`int`, *optional*, defaults to 128) : The number of channels in the hidden layer of the depth estimation head.

number_output_channels (`int`, *optional*, defaults to 256) : Number of output channels for the CHMv2 head (number of depth bins).

readout_type (`str`, *optional*, defaults to `"project"`) : Type of readout operation for the CLS token. One of `["ignore", "add", "project"]`.

min_depth (`float`, *optional*, defaults to 0.001) : The minimum depth value for depth bin calculation.

max_depth (`float`, *optional*, defaults to 96.0) : The maximum depth value for depth bin calculation.

bins_strategy (`str`, *optional*, defaults to `"chmv2_mixlog"`) : The strategy for depth bins distribution. One of `["linear", "log", "chmv2_mixlog"]`.

norm_strategy (`str`, *optional*, defaults to `"chmv2_mixlog"`) : The normalization strategy for depth prediction. One of `["linear", "softmax", "sigmoid", "chmv2_mixlog"]`.

This is the configuration class to store the configuration of a Chmv2Model. It is used to instantiate a Chmv2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/dinov3-vitl16-chmv2-dpt-head](https://huggingface.co/facebook/dinov3-vitl16-chmv2-dpt-head)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import CHMv2Config, CHMv2ForDepthEstimation

>>> configuration = CHMv2Config()
>>> model = CHMv2ForDepthEstimation(configuration)
>>> configuration = model.config
```

## CHMv2ImageProcessor[[transformers.CHMv2ImageProcessor]]

#### transformers.CHMv2ImageProcessor[[transformers.CHMv2ImageProcessor]]

```python
transformers.CHMv2ImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/chmv2/image_processing_chmv2.py#L99)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 384, 'width': 384}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `None`) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BICUBIC`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.42, 0.411, 0.296]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.213, 0.156, 0.143]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*, defaults to `None`) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

ensure_multiple_of (`int`, *kwargs*, *optional*, defaults to 1) : If `do_resize` is `True`, the image is resized to a size that is a multiple of this value. Can be overridden by `ensure_multiple_of` in `preprocess`.

size_divisor (`int`, *kwargs*, defaults to `16`) : The size by which to make sure both the height and width can be divided.

keep_aspect_ratio (`bool`, *kwargs*, *optional*, defaults to `False`) : If `True`, the image is resized to the largest possible size such that the aspect ratio is preserved. Can be overridden by `keep_aspect_ratio` in `preprocess`.

do_reduce_labels (`bool`, *kwargs*, *optional*, defaults to `self.do_reduce_labels`) : Whether or not to reduce all label values of segmentation maps by 1. Usually used for datasets where 0 is used for background, and background itself is not included in all classes of a dataset (e.g. ADE20k). The background label will be replaced by 255.

Constructs a CHMv2ImageProcessor image processor.

#### preprocess[[transformers.CHMv2ImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], segmentation_maps: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/chmv2/image_processing_chmv2.py#L125)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

segmentation_maps (`ImageInput`, *optional*) : The segmentation maps to preprocess.

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

ensure_multiple_of (`int`, *kwargs*, *optional*, defaults to 1) : If `do_resize` is `True`, the image is resized to a size that is a multiple of this value. Can be overridden by `ensure_multiple_of` in `preprocess`.

size_divisor (`int`, *kwargs*) : The size by which to make sure both the height and width can be divided.

keep_aspect_ratio (`bool`, *kwargs*, *optional*, defaults to `False`) : If `True`, the image is resized to the largest possible size such that the aspect ratio is preserved. Can be overridden by `keep_aspect_ratio` in `preprocess`.

do_reduce_labels (`bool`, *kwargs*, *optional*, defaults to `self.do_reduce_labels`) : Whether or not to reduce all label values of segmentation maps by 1. Usually used for datasets where 0 is used for background, and background itself is not included in all classes of a dataset (e.g. ADE20k). The background label will be replaced by 255.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

#### post_process_depth_estimation[[transformers.CHMv2ImageProcessor.post_process_depth_estimation]]

```python
post_process_depth_estimation(outputs: DepthEstimatorOutput, target_sizes: transformers.utils.generic.TensorType | list[tuple[int, int]] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/chmv2/image_processing_chmv2.py#L385)

**Parameters:**

outputs (`DepthEstimatorOutput`) : Raw outputs of the model.

target_sizes (`TensorType` or `List[Tuple[int, int]]`, *optional*) : Tensor of shape `(batch_size, 2)` or list of tuples (`Tuple[int, int]`) containing the target size (height, width) of each image in the batch. If left to None, predictions will not be resized.

**Returns:** `List[Dict[str, TensorType]]`

A list of dictionaries of tensors representing the processed depth
predictions.

Converts the raw output of `DepthEstimatorOutput` into final depth predictions and depth PIL images.
Only supports PyTorch.

## CHMv2ForDepthEstimation[[transformers.CHMv2ForDepthEstimation]]

#### transformers.CHMv2ForDepthEstimation[[transformers.CHMv2ForDepthEstimation]]

```python
transformers.CHMv2ForDepthEstimation(config: CHMv2Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/chmv2/modeling_chmv2.py#L384)

**Parameters:**

config ([CHMv2Config](/docs/transformers/v5.15.1/en/model_doc/chmv2#transformers.CHMv2Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

CHMv2 Model with a depth estimation head on top (consisting of convolutional layers) e.g. for canopy height
estimation.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.CHMv2ForDepthEstimation.forward]]

```python
forward(pixel_values: FloatTensor, labels: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/chmv2/modeling_chmv2.py#L397)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [CHMv2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/chmv2#transformers.CHMv2ImageProcessor). See `CHMv2ImageProcessor.__call__()` for details (`processor_class` uses [CHMv2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/chmv2#transformers.CHMv2ImageProcessor) for processing images).

labels (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) : Ground truth depth estimation maps for computing the loss.

**Returns:** [DepthEstimatorOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.DepthEstimatorOutput) or `tuple(torch.FloatTensor)`

A [DepthEstimatorOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.DepthEstimatorOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([CHMv2Config](/docs/transformers/v5.15.1/en/model_doc/chmv2#transformers.CHMv2Config)) and inputs.

The [CHMv2ForDepthEstimation](/docs/transformers/v5.15.1/en/model_doc/chmv2#transformers.CHMv2ForDepthEstimation) forward method, overrides the `__call__` special method.

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
