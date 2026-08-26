# SegGPT

## Overview

The SegGPT model was proposed in [SegGPT: Segmenting Everything In Context](https://huggingface.co/papers/2304.03284) by Xinlong Wang, Xiaosong Zhang, Yue Cao, Wen Wang, Chunhua Shen, Tiejun Huang. SegGPT employs a decoder-only Transformer that can generate a segmentation mask given an input image, a prompt image and its corresponding prompt mask. The model achieves remarkable one-shot results with 56.1 mIoU on COCO-20 and 85.6 mIoU on FSS-1000.

The abstract from the paper is the following:

*We present SegGPT, a generalist model for segmenting everything in context. We unify various segmentation tasks into a generalist in-context learning framework that accommodates different kinds of segmentation data by transforming them into the same format of images. The training of SegGPT is formulated as an in-context coloring problem with random color mapping for each data sample. The objective is to accomplish diverse tasks according to the context, rather than relying on specific colors. After training, SegGPT can perform arbitrary segmentation tasks in images or videos via in-context inference, such as object instance, stuff, part, contour, and text. SegGPT is evaluated on a broad range of tasks, including few-shot semantic segmentation, video object segmentation, semantic segmentation, and panoptic segmentation. Our results show strong capabilities in segmenting in-domain and out-of*

Tips:

- One can use [SegGptImageProcessor](/docs/transformers/v5.15.1/en/model_doc/seggpt#transformers.SegGptImageProcessor) to prepare image input, prompt and mask to the model.
- One can either use segmentation maps or RGB images as prompt masks. If using the latter make sure to set `do_convert_rgb=False` in the `preprocess` method.
- It's highly advisable to pass `num_labels` when using `segmentation_maps` (not considering background) during preprocessing and postprocessing with [SegGptImageProcessor](/docs/transformers/v5.15.1/en/model_doc/seggpt#transformers.SegGptImageProcessor) for your use case.
- When doing inference with [SegGptForImageSegmentation](/docs/transformers/v5.15.1/en/model_doc/seggpt#transformers.SegGptForImageSegmentation) if your `batch_size` is greater than 1 you can use feature ensemble across your images by passing `feature_ensemble=True` in the forward method.

Here's how to use the model for one-shot semantic segmentation:

```python
import torch
from datasets import load_dataset

from transformers import SegGptForImageSegmentation, SegGptImageProcessor

checkpoint = "BAAI/seggpt-vit-large"
image_processor = SegGptImageProcessor.from_pretrained(checkpoint)
model = SegGptForImageSegmentation.from_pretrained(checkpoint, device_map="auto")

dataset_id = "EduardoPacheco/FoodSeg103"
ds = load_dataset(dataset_id, split="train")
# Number of labels in FoodSeg103 (not including background)
num_labels = 103

image_input = ds[4]["image"]
ground_truth = ds[4]["label"]
image_prompt = ds[29]["image"]
mask_prompt = ds[29]["label"]

inputs = image_processor(
    images=image_input,
    prompt_images=image_prompt,
    segmentation_maps=mask_prompt,
    num_labels=num_labels,
    return_tensors="pt"
)

with torch.no_grad():
    outputs = model(**inputs)

target_sizes = [image_input.size[::-1]]
mask = image_processor.post_process_semantic_segmentation(outputs, target_sizes, num_labels=num_labels)[0]
```

This model was contributed by [EduardoPacheco](https://huggingface.co/EduardoPacheco).
The original code can be found [here](https://github.com/baaivision/Painter/tree/main).

## SegGptConfig[[transformers.SegGptConfig]]

#### transformers.SegGptConfig[[transformers.SegGptConfig]]

```python
transformers.SegGptConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 1024, num_hidden_layers: int = 24, num_attention_heads: int = 16, hidden_act: str = 'gelu', hidden_dropout_prob: float | int = 0.0, initializer_range: float = 0.02, layer_norm_eps: float = 1e-06, image_size: int | list[int] | tuple[int, ...] = (896, 448), patch_size: int | list[int] | tuple[int, int] = 16, num_channels: int = 3, qkv_bias: bool = True, mlp_dim: int | None = None, drop_path_rate: float | int = 0.1, pretrain_image_size: int | list[int] | tuple[int, int] = 224, decoder_hidden_size: int = 64, use_relative_position_embeddings: bool = True, merge_index: int = 2, intermediate_hidden_state_indices: list[int] | tuple[int, ...] = (5, 11, 17, 23), beta: float = 0.01)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seggpt/configuration_seggpt.py#L24)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `1024`) : Dimension of the hidden representations.

num_hidden_layers (`int`, *optional*, defaults to `24`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer decoder.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

hidden_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

image_size (`Union[int, list[int], tuple[int, ...]]`, *optional*, defaults to `(896, 448)`) : The size (resolution) of each image.

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) : The size (resolution) of each patch.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

qkv_bias (`bool`, *optional*, defaults to `True`) : Whether to add a bias to the queries, keys and values.

mlp_dim (`int`, *optional*) : The dimensionality of the MLP layer in the Transformer encoder. If unset, defaults to `hidden_size` * 4.

drop_path_rate (`Union[float, int]`, *optional*, defaults to `0.1`) : Drop path rate for the patch fusion.

pretrain_image_size (`int`, *optional*, defaults to 224) : The pretrained size of the absolute position embeddings.

decoder_hidden_size (`int`, *optional*, defaults to `64`) : Dimension of the hidden representations.

use_relative_position_embeddings (`bool`, *optional*, defaults to `True`) : Whether to use relative position embeddings in the attention layers.

merge_index (`int`, *optional*, defaults to 2) : The index of the encoder layer to merge the embeddings.

intermediate_hidden_state_indices (`list[int]`, *optional*, defaults to `[5, 11, 17, 23]`) : The indices of the encoder layers which we store as features for the decoder.

beta (`float`, *optional*, defaults to 0.01) : Regularization factor for SegGptLoss (smooth-l1 loss).

This is the configuration class to store the configuration of a SegGptModel. It is used to instantiate a Seggpt
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [BAAI/seggpt-vit-large](https://huggingface.co/BAAI/seggpt-vit-large)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import SegGptConfig, SegGptModel

>>> # Initializing a SegGPT seggpt-vit-large style configuration
>>> configuration = SegGptConfig()

>>> # Initializing a model (with random weights) from the seggpt-vit-large style configuration
>>> model = SegGptModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## SegGptImageProcessor[[transformers.SegGptImageProcessor]]

#### transformers.SegGptImageProcessor[[transformers.SegGptImageProcessor]]

```python
transformers.SegGptImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seggpt/image_processing_seggpt.py#L73)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 448, 'width': 448}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BICUBIC`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.485, 0.456, 0.406]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.229, 0.224, 0.225]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

num_labels (`int`, *kwargs*, *optional*) : Number of classes in the segmentation task (excluding the background). If specified, a palette will be built, assuming that class_idx 0 is the background, to map the prompt mask from a plain segmentation map to a 3-channel RGB image. Not specifying this will result in the prompt mask being duplicated across the channel dimension when `do_convert_rgb` is `True`.

Constructs a SegGptImageProcessor image processor.

#### preprocess[[transformers.SegGptImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, prompt_images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, prompt_masks: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seggpt/image_processing_seggpt.py#L134)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

prompt_images (`ImageInput`, *optional*) : Prompt images to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255.

prompt_masks (`ImageInput`, *optional*) : Prompt masks to preprocess. Can be in the format of segmentation maps (no channels) or RGB images. If in the format of RGB images, `do_convert_rgb` should be set to `False`. If in the format of segmentation maps, specifying `num_labels` is recommended to build a palette to map the prompt mask from a single channel to a 3-channel RGB. If `num_labels` is not specified, the prompt mask will be duplicated across the channel dimension.

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

num_labels (`int`, *kwargs*, *optional*) : Number of classes in the segmentation task (excluding the background). If specified, a palette will be built, assuming that class_idx 0 is the background, to map the prompt mask from a plain segmentation map to a 3-channel RGB image. Not specifying this will result in the prompt mask being duplicated across the channel dimension when `do_convert_rgb` is `True`.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

#### post_process_semantic_segmentation[[transformers.SegGptImageProcessor.post_process_semantic_segmentation]]

```python
post_process_semantic_segmentation(outputs, target_sizes: list[tuple[int, int]] | None = None, num_labels: int | None = None, return_segmentation_scores: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seggpt/image_processing_seggpt.py#L255)

**Parameters:**

outputs (`SegGptImageSegmentationOutput`) : Raw outputs of the model.

target_sizes (`list[tuple[int, int]]`, *optional*) : List of length `batch_size`, where each item corresponds to the requested final size `(height, width)` of each prediction. If left to `None`, predictions will not be resized.

num_labels (`int`, *optional*) : Number of classes in the segmentation task (excluding the background). If specified, a palette will be built to map prediction masks from RGB values back to class indices. Should match the value used during preprocessing.

return_segmentation_scores (`bool`, *optional*, defaults to `False`) : Whether to return segmentation scores alongside the segmentation map. When `True`, each element of the returned list is a `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape `(height, width)`) and `segmentation_scores` (shape `(num_labels+1, height, width)` of negative squared L2 distances to each palette color, or `None` when `num_labels` is not provided).

**Returns:** `list[torch.Tensor]` or `list[SemanticSegmentationPostProcessorOutput]`

When
`return_segmentation_scores=False` (default), a list of length `batch_size` where each item is a
segmentation map of shape `(height, width)` with class IDs. When `return_segmentation_scores=True`,
a list of `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape
`(height, width)`) and `segmentation_scores` (shape `(num_labels+1, height, width)`). In both cases,
`(height, width)` corresponds to the target size (if `target_sizes` is specified).

Converts the output of `SegGptImageSegmentationOutput` into segmentation maps. Only supports PyTorch.

## SegGptImageProcessorPil[[transformers.SegGptImageProcessorPil]]

#### transformers.SegGptImageProcessorPil[[transformers.SegGptImageProcessorPil]]

```python
transformers.SegGptImageProcessorPil(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seggpt/image_processing_pil_seggpt.py#L75)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 448, 'width': 448}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BICUBIC`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.485, 0.456, 0.406]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.229, 0.224, 0.225]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

num_labels (`int`, *kwargs*, *optional*) : Number of classes in the segmentation task (excluding the background). If specified, a palette will be built, assuming that class_idx 0 is the background, to map the prompt mask from a plain segmentation map to a 3-channel RGB image. Not specifying this will result in the prompt mask being duplicated across the channel dimension when `do_convert_rgb` is `True`.

Constructs a SegGptImageProcessor image processor.

#### preprocess[[transformers.SegGptImageProcessorPil.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, prompt_images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, prompt_masks: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seggpt/image_processing_pil_seggpt.py#L136)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

prompt_images (`ImageInput`, *optional*) : Prompt images to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255.

prompt_masks (`ImageInput`, *optional*) : Prompt masks to preprocess. Can be in the format of segmentation maps (no channels) or RGB images. If in the format of RGB images, `do_convert_rgb` should be set to `False`. If in the format of segmentation maps, specifying `num_labels` is recommended to build a palette to map the prompt mask from a single channel to a 3-channel RGB. If `num_labels` is not specified, the prompt mask will be duplicated across the channel dimension.

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

num_labels (`int`, *kwargs*, *optional*) : Number of classes in the segmentation task (excluding the background). If specified, a palette will be built, assuming that class_idx 0 is the background, to map the prompt mask from a plain segmentation map to a 3-channel RGB image. Not specifying this will result in the prompt mask being duplicated across the channel dimension when `do_convert_rgb` is `True`.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

#### post_process_semantic_segmentation[[transformers.SegGptImageProcessorPil.post_process_semantic_segmentation]]

```python
post_process_semantic_segmentation(outputs, target_sizes: list[tuple[int, int]] | None = None, num_labels: int | None = None, return_segmentation_scores: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seggpt/image_processing_pil_seggpt.py#L238)

**Parameters:**

outputs (`SegGptImageSegmentationOutput`) : Raw outputs of the model.

target_sizes (`list[tuple[int, int]]`, *optional*) : List of length `batch_size`, where each item corresponds to the requested final size `(height, width)` of each prediction. If left to `None`, predictions will not be resized.

num_labels (`int`, *optional*) : Number of classes in the segmentation task (excluding the background). If specified, a palette will be built to map prediction masks from RGB values back to class indices. Should match the value used during preprocessing.

return_segmentation_scores (`bool`, *optional*, defaults to `False`) : Whether to return segmentation scores alongside the segmentation map. When `True`, each element of the returned list is a `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape `(height, width)`) and `segmentation_scores` (shape `(num_labels+1, height, width)` of negative squared L2 distances to each palette color, or `None` when `num_labels` is not provided).

**Returns:** `list[torch.Tensor]` or `list[SemanticSegmentationPostProcessorOutput]`

When
`return_segmentation_scores=False` (default), a list of length `batch_size` where each item is a
segmentation map of shape `(height, width)` with class IDs. When `return_segmentation_scores=True`,
a list of `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape
`(height, width)`) and `segmentation_scores` (shape `(num_labels+1, height, width)`). In both cases,
`(height, width)` corresponds to the target size (if `target_sizes` is specified).

Converts the output of `SegGptImageSegmentationOutput` into segmentation maps. Only supports PyTorch.

## SegGptModel[[transformers.SegGptModel]]

#### transformers.SegGptModel[[transformers.SegGptModel]]

```python
transformers.SegGptModel(config: SegGptConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seggpt/modeling_seggpt.py#L613)

**Parameters:**

config ([SegGptConfig](/docs/transformers/v5.15.1/en/model_doc/seggpt#transformers.SegGptConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Seggpt Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.SegGptModel.forward]]

```python
forward(pixel_values: Tensor, prompt_pixel_values: Tensor, prompt_masks: Tensor, bool_masked_pos: typing.Optional[torch.BoolTensor] = None, feature_ensemble: bool | None = None, embedding_type: str | None = None, labels: typing.Optional[torch.FloatTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seggpt/modeling_seggpt.py#L627)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [SegGptImageProcessor](/docs/transformers/v5.15.1/en/model_doc/seggpt#transformers.SegGptImageProcessor). See `SegGptImageProcessor.__call__()` for details (`processor_class` uses [SegGptImageProcessor](/docs/transformers/v5.15.1/en/model_doc/seggpt#transformers.SegGptImageProcessor) for processing images).

prompt_pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) : Prompt pixel values. Prompt pixel values can be obtained using [AutoImageProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoImageProcessor). See `SegGptImageProcessor.__call__()` for details.

prompt_masks (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) : Prompt mask. Prompt mask can be obtained using [AutoImageProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoImageProcessor). See `SegGptImageProcessor.__call__()` for details.

bool_masked_pos (`torch.BoolTensor` of shape `(batch_size, num_patches)`, *optional*) : Boolean masked positions. Indicates which patches are masked (1) and which aren't (0).

feature_ensemble (`bool`, *optional*) : Boolean indicating whether to use feature ensemble or not. If `True`, the model will use feature ensemble if we have at least two prompts. If `False`, the model will not use feature ensemble. This argument should be considered when doing few-shot inference on an input image i.e. more than one prompt for the same image.

embedding_type (`str`, *optional*) : Embedding type. Indicates whether the prompt is a semantic or instance embedding. Can be either instance or semantic.

labels (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`, `optional`) : Ground truth mask for input images.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `SegGptEncoderOutput` or `tuple(torch.FloatTensor)`

A `SegGptEncoderOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SegGptConfig](/docs/transformers/v5.15.1/en/model_doc/seggpt#transformers.SegGptConfig)) and inputs.

The [SegGptModel](/docs/transformers/v5.15.1/en/model_doc/seggpt#transformers.SegGptModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, patch_height, patch_width, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple[torch.FloatTensor]`, `optional`, returned when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer)
  of shape `(batch_size, patch_height, patch_width, hidden_size)`.
- **attentions** (`tuple[torch.FloatTensor]`, `optional`, returned when `config.output_attentions=True`) -- Tuple of *torch.FloatTensor* (one for each layer) of shape
  `(batch_size, num_heads, seq_len, seq_len)`.
- **intermediate_hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `config.intermediate_hidden_state_indices` is set) -- Tuple of `torch.FloatTensor` of shape `(batch_size, patch_height, patch_width, hidden_size)`.
  Each element in the Tuple corresponds to the output of the layer specified in `config.intermediate_hidden_state_indices`.
  Additionally, each feature passes through a LayerNorm.

Examples:

```python
>>> from transformers import SegGptImageProcessor, SegGptModel
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> image_input_url = "https://raw.githubusercontent.com/baaivision/Painter/main/SegGPT/SegGPT_inference/examples/hmbb_2.jpg"
>>> image_prompt_url = "https://raw.githubusercontent.com/baaivision/Painter/main/SegGPT/SegGPT_inference/examples/hmbb_1.jpg"
>>> mask_prompt_url = "https://raw.githubusercontent.com/baaivision/Painter/main/SegGPT/SegGPT_inference/examples/hmbb_1_target.png"

>>> with httpx.stream("GET", image_input_url) as response:
...     image_input = Image.open(BytesIO(response.read()))

>>> with httpx.stream("GET", image_prompt_url) as response:
...     image_prompt = Image.open(BytesIO(response.read()))

>>> with httpx.stream("GET", mask_prompt_url) as response:
...     mask_prompt = Image.open(BytesIO(response.read())).convert("L")

>>> checkpoint = "BAAI/seggpt-vit-large"
>>> model = SegGptModel.from_pretrained(checkpoint)
>>> image_processor = SegGptImageProcessor.from_pretrained(checkpoint)

>>> inputs = image_processor(images=image_input, prompt_images=image_prompt, prompt_masks=mask_prompt, return_tensors="pt")

>>> outputs = model(**inputs)
>>> list(outputs.last_hidden_state.shape)
[1, 56, 28, 1024]
```

## SegGptForImageSegmentation[[transformers.SegGptForImageSegmentation]]

#### transformers.SegGptForImageSegmentation[[transformers.SegGptForImageSegmentation]]

```python
transformers.SegGptForImageSegmentation(config: SegGptConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seggpt/modeling_seggpt.py#L819)

**Parameters:**

config ([SegGptConfig](/docs/transformers/v5.15.1/en/model_doc/seggpt#transformers.SegGptConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

SegGpt model with a decoder on top for one-shot image segmentation.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.SegGptForImageSegmentation.forward]]

```python
forward(pixel_values: Tensor, prompt_pixel_values: Tensor, prompt_masks: Tensor, bool_masked_pos: typing.Optional[torch.BoolTensor] = None, feature_ensemble: bool | None = None, embedding_type: str | None = None, labels: typing.Optional[torch.FloatTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seggpt/modeling_seggpt.py#L830)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [SegGptImageProcessor](/docs/transformers/v5.15.1/en/model_doc/seggpt#transformers.SegGptImageProcessor). See `SegGptImageProcessor.__call__()` for details (`processor_class` uses [SegGptImageProcessor](/docs/transformers/v5.15.1/en/model_doc/seggpt#transformers.SegGptImageProcessor) for processing images).

prompt_pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) : Prompt pixel values. Prompt pixel values can be obtained using [AutoImageProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoImageProcessor). See `SegGptImageProcessor.__call__()` for details.

prompt_masks (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) : Prompt mask. Prompt mask can be obtained using [AutoImageProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoImageProcessor). See `SegGptImageProcessor.__call__()` for details.

bool_masked_pos (`torch.BoolTensor` of shape `(batch_size, num_patches)`, *optional*) : Boolean masked positions. Indicates which patches are masked (1) and which aren't (0).

feature_ensemble (`bool`, *optional*) : Boolean indicating whether to use feature ensemble or not. If `True`, the model will use feature ensemble if we have at least two prompts. If `False`, the model will not use feature ensemble. This argument should be considered when doing few-shot inference on an input image i.e. more than one prompt for the same image.

embedding_type (`str`, *optional*) : Embedding type. Indicates whether the prompt is a semantic or instance embedding. Can be either instance or semantic.

labels (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`, `optional`) : Ground truth mask for input images.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `SegGptImageSegmentationOutput` or `tuple(torch.FloatTensor)`

A `SegGptImageSegmentationOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SegGptConfig](/docs/transformers/v5.15.1/en/model_doc/seggpt#transformers.SegGptConfig)) and inputs.

The [SegGptForImageSegmentation](/docs/transformers/v5.15.1/en/model_doc/seggpt#transformers.SegGptForImageSegmentation) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor`, *optional*, returned when `labels` is provided) -- The loss value.
- **pred_masks** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- The predicted masks.
- **hidden_states** (`tuple[torch.FloatTensor]`, `optional`, returned when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer)
  of shape `(batch_size, patch_height, patch_width, hidden_size)`.
- **attentions** (`tuple[torch.FloatTensor]`, `optional`, returned when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape
  `(batch_size, num_heads, seq_len, seq_len)`.

Examples:

```python
>>> from transformers import SegGptImageProcessor, SegGptForImageSegmentation
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> image_input_url = "https://raw.githubusercontent.com/baaivision/Painter/main/SegGPT/SegGPT_inference/examples/hmbb_2.jpg"
>>> image_prompt_url = "https://raw.githubusercontent.com/baaivision/Painter/main/SegGPT/SegGPT_inference/examples/hmbb_1.jpg"
>>> mask_prompt_url = "https://raw.githubusercontent.com/baaivision/Painter/main/SegGPT/SegGPT_inference/examples/hmbb_1_target.png"

>>> with httpx.stream("GET", image_input_url) as response:
...     image_input = Image.open(BytesIO(response.read()))

>>> with httpx.stream("GET", image_prompt_url) as response:
...     image_prompt = Image.open(BytesIO(response.read()))

>>> with httpx.stream("GET", mask_prompt_url) as response:
...     mask_prompt = Image.open(BytesIO(response.read())).convert("L")

>>> checkpoint = "BAAI/seggpt-vit-large"
>>> model = SegGptForImageSegmentation.from_pretrained(checkpoint)
>>> image_processor = SegGptImageProcessor.from_pretrained(checkpoint)

>>> inputs = image_processor(images=image_input, prompt_images=image_prompt, prompt_masks=mask_prompt, return_tensors="pt")
>>> outputs = model(**inputs)
>>> result = image_processor.post_process_semantic_segmentation(outputs, target_sizes=[(image_input.height, image_input.width)])[0]
>>> print(list(result.shape))
[170, 297]
```
