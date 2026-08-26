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

DPT is compatible with the [AutoBackbone](/docs/transformers/v5.15.1/en/main_classes/backbones#transformers.AutoBackbone) class. This allows to use the DPT framework with various computer vision backbones available in the library, such as `VitDetBackbone` or `Dinov2Backbone`. One can create it as follows:

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

- Demo notebooks for [DPTForDepthEstimation](/docs/transformers/v5.15.1/en/model_doc/dpt#transformers.DPTForDepthEstimation) can be found [here](https://github.com/NielsRogge/Transformers-Tutorials/tree/master/DPT).

- [Semantic segmentation task guide](../tasks/semantic_segmentation)
- [Monocular depth estimation task guide](../tasks/monocular_depth_estimation)

If you're interested in submitting a resource to be included here, please feel free to open a Pull Request and we'll review it! The resource should ideally demonstrate something new instead of duplicating an existing resource.

## DPTConfig[[transformers.DPTConfig]]

#### transformers.DPTConfig[[transformers.DPTConfig]]

```python
transformers.DPTConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 768, num_hidden_layers: None | int = 12, num_attention_heads: int | None = 12, intermediate_size: int | None = 3072, hidden_act: str = 'gelu', hidden_dropout_prob: float | int | None = 0.0, attention_probs_dropout_prob: float | int | None = 0.0, initializer_range: float = 0.02, layer_norm_eps: float | None = 1e-12, image_size: int | list[int] | tuple[int, int] | None = 384, patch_size: int | list[int] | tuple[int, int] | None = 16, num_channels: int | None = 3, is_hybrid: bool = False, qkv_bias: bool | None = True, backbone_out_indices: list[int] | tuple[int, ...] | None = (2, 5, 8, 11), readout_type: str = 'project', reassemble_factors: list[int | float] | tuple[int | float, ...] = (4, 2, 1, 0.5), neck_hidden_sizes: list[int] | tuple[int, ...] = (96, 192, 384, 768), fusion_hidden_size: int = 256, head_in_index: int = -1, use_batch_norm_in_fusion_residual: bool | None = False, use_bias_in_fusion_residual: bool | None = None, add_projection: bool = False, use_auxiliary_head: bool | None = True, auxiliary_loss_weight: float = 0.4, semantic_loss_ignore_index: int = 255, semantic_classifier_dropout: float | int = 0.1, backbone_featmap_shape: list[int] | tuple[int, ...] | None = (1, 1024, 24, 24), neck_ignore_stages: list[int] | tuple[int, ...] = (0, 1), backbone_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, pooler_output_size: int | None = None, pooler_act: str = 'tanh')
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dpt/configuration_dpt.py#L26)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

num_hidden_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer decoder.

intermediate_size (`int`, *optional*, defaults to `3072`) : Dimension of the MLP representations.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

hidden_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

attention_probs_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_eps (`float`, *optional*, defaults to `1e-12`) : The epsilon used by the layer normalization layers.

image_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `384`) : The size (resolution) of each image.

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) : The size (resolution) of each patch.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

is_hybrid (`bool`, *optional*, defaults to `False`) : Whether to use a hybrid backbone. Useful in the context of loading DPT-Hybrid models.

qkv_bias (`bool`, *optional*, defaults to `True`) : Whether to add a bias to the queries, keys and values.

backbone_out_indices (`list[int]`, *optional*, defaults to `[2, 5, 8, 11]`) : Indices of the intermediate hidden states to use from backbone.

readout_type (`str`, *optional*, defaults to `"project"`) : The readout type to use when processing the readout token (CLS token) of the intermediate hidden states of the ViT backbone. Can be one of [`"ignore"`, `"add"`, `"project"`]. - "ignore" simply ignores the CLS token. - "add" passes the information from the CLS token to all other tokens by adding the representations. - "project" passes information to the other tokens by concatenating the readout to all other tokens before projecting the representation to the original feature dimension D using a linear layer followed by a GELU non-linearity.

reassemble_factors (`list[int]`, *optional*, defaults to `[4, 2, 1, 0.5]`) : The up/downsampling factors of the reassemble layers.

neck_hidden_sizes (`list[str]`, *optional*, defaults to `[96, 192, 384, 768]`) : The hidden sizes to project to for the feature maps of the backbone.

fusion_hidden_size (`int`, *optional*, defaults to 256) : The number of channels before fusion.

head_in_index (`int`, *optional*, defaults to -1) : The index of the features to use in the heads.

use_batch_norm_in_fusion_residual (`bool`, *optional*, defaults to `False`) : Whether to use batch normalization in the pre-activate residual units of the fusion blocks.

use_bias_in_fusion_residual (`bool`, *optional*, defaults to `True`) : Whether to use bias in the pre-activate residual units of the fusion blocks.

add_projection (`bool`, *optional*, defaults to `False`) : Whether to add a projection layer before the depth estimation head.

use_auxiliary_head (`bool`, *optional*, defaults to `True`) : Whether to use an auxiliary head during training.

auxiliary_loss_weight (`float`, *optional*, defaults to 0.4) : Weight of the cross-entropy loss of the auxiliary head.

semantic_loss_ignore_index (`int`, *optional*, defaults to `255`) : The index that is ignored by the loss function of the semantic segmentation model.

semantic_classifier_dropout (`float`, *optional*, defaults to 0.1) : The dropout ratio for the semantic classification head.

backbone_featmap_shape (`list[int]`, *optional*, defaults to `[1, 1024, 24, 24]`) : Used only for the `hybrid` embedding type. The shape of the feature maps of the backbone.

neck_ignore_stages (`list[int]`, *optional*, defaults to `[0, 1]`) : Used only for the `hybrid` embedding type. The stages of the readout layers to ignore.

backbone_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The configuration of the backbone model.

pooler_output_size (`int`, *optional*) : Dimensionality of the pooler layer. If None, defaults to `hidden_size`.

pooler_act (`str`, *optional*, defaults to `"tanh"`) : The activation function to be used by the pooler.

This is the configuration class to store the configuration of a DPTModel. It is used to instantiate a Dpt
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Intel/dpt-large](https://huggingface.co/Intel/dpt-large)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

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

#### transformers.DPTImageProcessor[[transformers.DPTImageProcessor]]

```python
transformers.DPTImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dpt/image_processing_dpt.py#L110)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 384, 'width': 384}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `None`) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BICUBIC`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.5, 0.5, 0.5]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.5, 0.5, 0.5]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*, defaults to `None`) : Whether to center crop the image.

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

Constructs a DPTImageProcessor image processor.

#### preprocess[[transformers.DPTImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], segmentation_maps: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dpt/image_processing_dpt.py#L135)

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

## DPTImageProcessorPil[[transformers.DPTImageProcessorPil]]

#### transformers.DPTImageProcessorPil[[transformers.DPTImageProcessorPil]]

```python
transformers.DPTImageProcessorPil(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dpt/image_processing_pil_dpt.py#L110)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 384, 'width': 384}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BICUBIC`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.5, 0.5, 0.5]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.5, 0.5, 0.5]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

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

Constructs a DPTImageProcessor image processor.

#### preprocess[[transformers.DPTImageProcessorPil.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], segmentation_maps: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dpt/image_processing_pil_dpt.py#L131)

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

#### post_process_semantic_segmentation[[transformers.DPTImageProcessorPil.post_process_semantic_segmentation]]

```python
post_process_semantic_segmentation(outputs, target_sizes: list[tuple] | None = None, return_segmentation_scores: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dpt/image_processing_pil_dpt.py#L268)

**Parameters:**

outputs ([DPTForSemanticSegmentation](/docs/transformers/v5.15.1/en/model_doc/dpt#transformers.DPTForSemanticSegmentation)) : Raw outputs of the model.

target_sizes (`list[tuple]`, *optional*) : A list of tuples (`tuple[int, int]`) containing the target size (height, width) of each image in the batch. If unset, predictions will not be resized.

return_segmentation_scores (`bool`, *optional*, defaults to `False`) : Whether to return segmentation scores alongside the segmentation map. When `True`, each element of the returned list is a `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape `(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`).

**Returns:** `list[torch.Tensor]` or `list[SemanticSegmentationPostProcessorOutput]`

When
`return_segmentation_scores=False` (default), a list of length `batch_size` where each item is a
segmentation map of shape `(height, width)` with class IDs. When `return_segmentation_scores=True`,
a list of `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape
`(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`). In both cases,
`(height, width)` corresponds to the target size (if `target_sizes` is specified).

Converts the output of [DPTForSemanticSegmentation](/docs/transformers/v5.15.1/en/model_doc/dpt#transformers.DPTForSemanticSegmentation) into semantic segmentation maps.

#### post_process_depth_estimation[[transformers.DPTImageProcessorPil.post_process_depth_estimation]]

```python
post_process_depth_estimation(outputs: DepthEstimatorOutput, target_sizes: transformers.utils.generic.TensorType | list[tuple[int, int]] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dpt/image_processing_pil_dpt.py#L331)

Converts the raw output of `DepthEstimatorOutput` into final depth predictions.

## DPTModel[[transformers.DPTModel]]

#### transformers.DPTModel[[transformers.DPTModel]]

```python
transformers.DPTModel(config: DPTConfig, add_pooling_layer: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dpt/modeling_dpt.py#L752)

**Parameters:**

config ([DPTConfig](/docs/transformers/v5.15.1/en/model_doc/dpt#transformers.DPTConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

add_pooling_layer (`bool`, *optional*, defaults to `True`) : Whether to add a pooling layer

The bare Dpt Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.DPTModel.forward]]

```python
forward(pixel_values: FloatTensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dpt/modeling_dpt.py#L780)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [DPTImageProcessor](/docs/transformers/v5.15.1/en/model_doc/dpt#transformers.DPTImageProcessor). See `DPTImageProcessor.__call__()` for details (`processor_class` uses [DPTImageProcessor](/docs/transformers/v5.15.1/en/model_doc/dpt#transformers.DPTImageProcessor) for processing images).

**Returns:** `BaseModelOutputWithPoolingAndIntermediateActivations` or `tuple(torch.FloatTensor)`

A `BaseModelOutputWithPoolingAndIntermediateActivations` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DPTConfig](/docs/transformers/v5.15.1/en/model_doc/dpt#transformers.DPTConfig)) and inputs.

The [DPTModel](/docs/transformers/v5.15.1/en/model_doc/dpt#transformers.DPTModel) forward method, overrides the `__call__` special method.

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

#### transformers.DPTForDepthEstimation[[transformers.DPTForDepthEstimation]]

```python
transformers.DPTForDepthEstimation(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dpt/modeling_dpt.py#L923)

**Parameters:**

config ([DPTForDepthEstimation](/docs/transformers/v5.15.1/en/model_doc/dpt#transformers.DPTForDepthEstimation)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

DPT Model with a depth estimation head on top (consisting of 3 convolutional layers) e.g. for KITTI, NYUv2.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.DPTForDepthEstimation.forward]]

```python
forward(pixel_values: FloatTensor, labels: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dpt/modeling_dpt.py#L942)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [DPTImageProcessor](/docs/transformers/v5.15.1/en/model_doc/dpt#transformers.DPTImageProcessor). See `DPTImageProcessor.__call__()` for details (`processor_class` uses [DPTImageProcessor](/docs/transformers/v5.15.1/en/model_doc/dpt#transformers.DPTImageProcessor) for processing images).

labels (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) : Ground truth depth estimation maps for computing the loss.

**Returns:** [DepthEstimatorOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.DepthEstimatorOutput) or `tuple(torch.FloatTensor)`

A [DepthEstimatorOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.DepthEstimatorOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DPTConfig](/docs/transformers/v5.15.1/en/model_doc/dpt#transformers.DPTConfig)) and inputs.

The [DPTForDepthEstimation](/docs/transformers/v5.15.1/en/model_doc/dpt#transformers.DPTForDepthEstimation) forward method, overrides the `__call__` special method.

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

#### transformers.DPTForSemanticSegmentation[[transformers.DPTForSemanticSegmentation]]

```python
transformers.DPTForSemanticSegmentation(config: DPTConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dpt/modeling_dpt.py#L1079)

**Parameters:**

config ([DPTConfig](/docs/transformers/v5.15.1/en/model_doc/dpt#transformers.DPTConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Dpt Model with a semantic segmentation head on top e.g. for ADE20K, CityScapes.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.DPTForSemanticSegmentation.forward]]

```python
forward(pixel_values: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dpt/modeling_dpt.py#L1095)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [DPTImageProcessor](/docs/transformers/v5.15.1/en/model_doc/dpt#transformers.DPTImageProcessor). See `DPTImageProcessor.__call__()` for details (`processor_class` uses [DPTImageProcessor](/docs/transformers/v5.15.1/en/model_doc/dpt#transformers.DPTImageProcessor) for processing images).

labels (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) : Ground truth semantic segmentation maps for computing the loss. Indices should be in `[0, ..., config.num_labels - 1]`. If `config.num_labels > 1`, a classification loss is computed (Cross-Entropy).

**Returns:** [SemanticSegmenterOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.SemanticSegmenterOutput) or `tuple(torch.FloatTensor)`

A [SemanticSegmenterOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.SemanticSegmenterOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DPTConfig](/docs/transformers/v5.15.1/en/model_doc/dpt#transformers.DPTConfig)) and inputs.

The [DPTForSemanticSegmentation](/docs/transformers/v5.15.1/en/model_doc/dpt#transformers.DPTForSemanticSegmentation) forward method, overrides the `__call__` special method.

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
