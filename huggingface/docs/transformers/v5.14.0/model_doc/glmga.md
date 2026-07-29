# Glmga

## Overview

The Glmga model was proposed in []() by .

The abstract from the paper is the following:

Tips:

This model was contributed by [INSERT YOUR HF USERNAME HERE](https://huggingface.co/).
The original code can be found [here]().

## Usage examples

Glmga reuses the [GLM-4.6V](./glm46v) modeling and processor; only its configuration and image/video
processors are model-specific. Load it with the `Auto*` classes (e.g. `AutoModelForImageTextToText`,
`AutoProcessor`), which resolve to the GLM-4.6V implementation.

## GlmgaConfig[[transformers.GlmgaConfig]]

- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **image_token_id** (`int`, *optional*, defaults to `151343`) --
  The image token index used as a placeholder for input images.
- **video_token_id** (`int`, *optional*, defaults to `151344`) --
  The video token index used as a placeholder for input videos.
- **image_start_token_id** (`int`, *optional*, defaults to 151339) --
  The image start token index to encode the start of image.
- **image_end_token_id** (`int`, *optional*, defaults to 151340) --
  The image end token index to encode the end of image.
- **video_start_token_id** (`int`, *optional*, defaults to 151361) --
  The video start token index to encode the start of video.
- **video_end_token_id** (`int`, *optional*, defaults to 151362) --
  The video end token index to encode the end of video.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Glm46VModel. It is used to instantiate a Glmga
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [zai-org/GLM-4.1V-9B-Thinking](https://huggingface.co/zai-org/GLM-4.1V-9B-Thinking)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import AutoModelForImageTextToText, GlmgaConfig

>>> # Initializing a Glmga style configuration
>>> configuration = GlmgaConfig()

>>> # Initializing a model (reusing the GLM-4.6V implementation) from that configuration
>>> model = AutoModelForImageTextToText.from_config(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## GlmgaImageProcessor[[transformers.GlmgaImageProcessor]]

- **patch_size** (`int`, *kwargs*, *optional*, defaults to 14) --
  The spatial patch size of the vision encoder.
- **temporal_patch_size** (`int`, *kwargs*, *optional*, defaults to 2) --
  The temporal patch size of the vision encoder.
- **merge_size** (`int`, *kwargs*, *optional*, defaults to 2) --
  The merge size of the vision encoder to llm encoder.
- **patch_expand_factor** (`int`, *kwargs*, *optional*, defaults to 1) --
  The patch_expand_factor of the vision encoder to llm encoder.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a GlmgaImageProcessor image processor.

- **height** (`int`) --
  Height of the input image.
- **width** (`int`) --
  Width of the input image.
- **images_kwargs** (`dict`, *optional*) --
  Any kwargs to override defaults of the image processor.`int`Number of image patches per image.

A utility that returns number of image patches for a given image size.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **patch_size** (`int`, *kwargs*, *optional*, defaults to 14) --
  The spatial patch size of the vision encoder.
- **temporal_patch_size** (`int`, *kwargs*, *optional*, defaults to 2) --
  The temporal patch size of the vision encoder.
- **merge_size** (`int`, *kwargs*, *optional*, defaults to 2) --
  The merge size of the vision encoder to llm encoder.
- **patch_expand_factor** (`int`, *kwargs*, *optional*, defaults to 1) --
  The patch_expand_factor of the vision encoder to llm encoder.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## GlmgaVideoProcessor[[transformers.GlmgaVideoProcessor]]

- **do_resize** (`bool`, *optional*, defaults to `self.do_resize`) --
  Whether to resize the video's (height, width) dimensions to the specified `size`. Can be overridden by the
  `do_resize` parameter in the `preprocess` method.
- **size** (`dict`, *optional*, defaults to `self.size`) --
  Size of the output video after resizing. Can be overridden by the `size` parameter in the `preprocess`
  method.
- **size_divisor** (`int`, *optional*, defaults to `self.size_divisor`) --
  The size by which to make sure both the height and width can be divided.
- **default_to_square** (`bool`, *optional*, defaults to `self.default_to_square`) --
  Whether to default to a square video when resizing, if size is an int.
- **resample** (`PILImageResampling`, *optional*, defaults to `self.resample`) --
  Resampling filter to use if resizing the video. Only has an effect if `do_resize` is set to `True`. Can be
  overridden by the `resample` parameter in the `preprocess` method.
- **do_center_crop** (`bool`, *optional*, defaults to `self.do_center_crop`) --
  Whether to center crop the video to the specified `crop_size`. Can be overridden by `do_center_crop` in the
  `preprocess` method.
- **crop_size** (`dict[str, int]` *optional*, defaults to `self.crop_size`) --
  Size of the output video after applying `center_crop`. Can be overridden by `crop_size` in the `preprocess`
  method.
- **do_rescale** (`bool`, *optional*, defaults to `self.do_rescale`) --
  Whether to rescale the video by the specified scale `rescale_factor`. Can be overridden by the
  `do_rescale` parameter in the `preprocess` method.
- **rescale_factor** (`int` or `float`, *optional*, defaults to `self.rescale_factor`) --
  Scale factor to use if rescaling the video. Only has an effect if `do_rescale` is set to `True`. Can be
  overridden by the `rescale_factor` parameter in the `preprocess` method.
- **do_normalize** (`bool`, *optional*, defaults to `self.do_normalize`) --
  Whether to normalize the video. Can be overridden by the `do_normalize` parameter in the `preprocess`
  method. Can be overridden by the `do_normalize` parameter in the `preprocess` method.
- **image_mean** (`float` or `list[float]`, *optional*, defaults to `self.image_mean`) --
  Mean to use if normalizing the video. This is a float or list of floats the length of the number of
  channels in the video. Can be overridden by the `image_mean` parameter in the `preprocess` method. Can be
  overridden by the `image_mean` parameter in the `preprocess` method.
- **image_std** (`float` or `list[float]`, *optional*, defaults to `self.image_std`) --
  Standard deviation to use if normalizing the video. This is a float or list of floats the length of the
  number of channels in the video. Can be overridden by the `image_std` parameter in the `preprocess` method.
  Can be overridden by the `image_std` parameter in the `preprocess` method.
- **do_convert_rgb** (`bool`, *optional*, defaults to `self.image_std`) --
  Whether to convert the video to RGB.
- **video_metadata** (`VideoMetadata`, *optional*) --
  Metadata of the video containing information about total duration, fps and total number of frames.
- **do_sample_frames** (`int`, *optional*, defaults to `self.do_sample_frames`) --
  Whether to sample frames from the video before processing or to process the whole video.
- **num_frames** (`int`, *optional*, defaults to `self.num_frames`) --
  Maximum number of frames to sample when `do_sample_frames=True`.
- **fps** (`int` or `float`, *optional*, defaults to `self.fps`) --
  Target frames to sample per second when `do_sample_frames=True`.
- **return_tensors** (`str` or `TensorType`, *optional*) --
  Returns stacked tensors if set to `pt, otherwise returns a list of tensors.
- **data_format** (`ChannelDimension` or `str`, *optional*, defaults to `ChannelDimension.FIRST`) --
  The channel dimension format for the output video. Can be one of:
  - `"channels_first"` or `ChannelDimension.FIRST`: video in (num_channels, height, width) format.
  - `"channels_last"` or `ChannelDimension.LAST`: video in (height, width, num_channels) format.
  - Unset: Use the channel dimension format of the input video.
- **input_data_format** (`ChannelDimension` or `str`, *optional*) --
  The channel dimension format for the input video. If unset, the channel dimension format is inferred
  from the input video. Can be one of:
  - `"channels_first"` or `ChannelDimension.FIRST`: video in (num_channels, height, width) format.
  - `"channels_last"` or `ChannelDimension.LAST`: video in (height, width, num_channels) format.
  - `"none"` or `ChannelDimension.NONE`: video in (height, width) format.
- **device** (`torch.device`, *optional*) --
  The device to process the videos on. If unset, the device is inferred from the input videos.
- **return_metadata** (`bool`, *optional*) --
  Whether to return video metadata or not.

- **patch_size** (`int`, *optional*, defaults to 14) --
  The spacial patch size of the vision encoder.
- **temporal_patch_size** (`int`, *optional*, defaults to 2) --
  The temporal patch size of the vision encoder.
- **merge_size** (`int`, *optional*, defaults to 2) --
  The merge size of the vision encoder to llm encoder.
Constructs a fast GLM-4V image processor that dynamically resizes videos based on the original videos.

- **metadata** (`VideoMetadata`) --
  Metadata of the video containing information about total duration, fps and total number of frames.
- **fps** (`int` or `float`, *optional*) --
  Target frames to sample per second. Defaults to `self.fps`.np.ndarrayIndices to sample video frames.

## GlmgaImageProcessorPil[[transformers.GlmgaImageProcessorPil]]

- **patch_size** (`int`, *kwargs*, *optional*, defaults to 14) --
  The spatial patch size of the vision encoder.
- **temporal_patch_size** (`int`, *kwargs*, *optional*, defaults to 2) --
  The temporal patch size of the vision encoder.
- **merge_size** (`int`, *kwargs*, *optional*, defaults to 2) --
  The merge size of the vision encoder to llm encoder.
- **patch_expand_factor** (`int`, *kwargs*, *optional*, defaults to 1) --
  The patch_expand_factor of the vision encoder to llm encoder.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a GlmgaImageProcessor image processor.

- **height** (`int`) --
  Height of the input image.
- **width** (`int`) --
  Width of the input image.
- **images_kwargs** (`dict`, *optional*) --
  Any kwargs to override defaults of the image processor.`int`Number of image patches per image.

A utility that returns number of image patches for a given image size.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **patch_size** (`int`, *kwargs*, *optional*, defaults to 14) --
  The spatial patch size of the vision encoder.
- **temporal_patch_size** (`int`, *kwargs*, *optional*, defaults to 2) --
  The temporal patch size of the vision encoder.
- **merge_size** (`int`, *kwargs*, *optional*, defaults to 2) --
  The merge size of the vision encoder to llm encoder.
- **patch_expand_factor** (`int`, *kwargs*, *optional*, defaults to 1) --
  The patch_expand_factor of the vision encoder to llm encoder.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.
