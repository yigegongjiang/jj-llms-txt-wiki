# SuperPoint

[SuperPoint](https://huggingface.co/papers/1712.07629) is the result of self-supervised training of a fully-convolutional network for interest point detection and description. The model is able to detect interest points that are repeatable under homographic transformations and provide a descriptor for each point. Usage on it's own is limited, but it can be used as a feature extractor for other tasks such as homography estimation and image matching.

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/superpoint_architecture.png"
alt="drawing" width="500"/>

You can find all the original SuperPoint checkpoints under the [Magic Leap Community](https://huggingface.co/magic-leap-community) organization.

> [!TIP]
> This model was contributed by [stevenbucaille](https://huggingface.co/stevenbucaille).
>
> Click on the SuperPoint models in the right sidebar for more examples of how to apply SuperPoint to different computer vision tasks.

The example below demonstrates how to detect interest points in an image with the [AutoModel](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel) class.

```python
import requests
import torch
from PIL import Image

from transformers import AutoImageProcessor, SuperPointForKeypointDetection

url = "http://images.cocodataset.org/val2017/000000039769.jpg"
image = Image.open(requests.get(url, stream=True).raw)

processor = AutoImageProcessor.from_pretrained("magic-leap-community/superpoint")
model = SuperPointForKeypointDetection.from_pretrained("magic-leap-community/superpoint", device_map="auto")

inputs = processor(image, return_tensors="pt").to(model.device)
with torch.no_grad():
    outputs = model(**inputs)

# Post-process to get keypoints, scores, and descriptors
image_size = (image.height, image.width)
processed_outputs = processor.post_process_keypoint_detection(outputs, [image_size])
```

## Notes

- SuperPoint outputs a dynamic number of keypoints per image, which makes it suitable for tasks requiring variable-length feature representations.

    ```py
    from transformers import AutoImageProcessor, SuperPointForKeypointDetection
    import torch
    from PIL import Image
    import requests
    processor = AutoImageProcessor.from_pretrained("magic-leap-community/superpoint")
    model = SuperPointForKeypointDetection.from_pretrained("magic-leap-community/superpoint", device_map="auto")
    url_image_1 = "http://images.cocodataset.org/val2017/000000039769.jpg"
    image_1 = Image.open(requests.get(url_image_1, stream=True).raw)
    url_image_2 = "http://images.cocodataset.org/test-stuff2017/000000000568.jpg"
    image_2 = Image.open(requests.get(url_image_2, stream=True).raw)
    images = [image_1, image_2]
    inputs = processor(images, return_tensors="pt").to(model.device)
    # Example of handling dynamic keypoint output
    outputs = model(**inputs)
    keypoints = outputs.keypoints  # Shape varies per image
    scores = outputs.scores        # Confidence scores for each keypoint
    descriptors = outputs.descriptors  # 256-dimensional descriptors
    mask = outputs.mask # Value of 1 corresponds to a keypoint detection
    ```

- The model provides both keypoint coordinates and their corresponding descriptors (256-dimensional vectors) in a single forward pass.
- For batch processing with multiple images, you need to use the mask attribute to retrieve the respective information for each image. You can use the `post_process_keypoint_detection` from the `SuperPointImageProcessor` to retrieve the each image information.

    ```py
    # Batch processing example
    images = [image1, image2, image3]
    inputs = processor(images, return_tensors="pt").to(model.device)
    outputs = model(**inputs)
    image_sizes = [(img.height, img.width) for img in images]
    processed_outputs = processor.post_process_keypoint_detection(outputs, image_sizes)
    ```

- You can then print the keypoints on the image of your choice to visualize the result:

    ```py
    import matplotlib.pyplot as plt
    plt.axis("off")
    plt.imshow(image_1)
    plt.scatter(
        outputs[0]["keypoints"][:, 0],
        outputs[0]["keypoints"][:, 1],
        c=outputs[0]["scores"] * 100,
        s=outputs[0]["scores"] * 50,
        alpha=0.8
    )
    plt.savefig(f"output_image.png")
    ```

    

## Resources

- Refer to this [notebook](https://github.com/NielsRogge/Transformers-Tutorials/blob/master/SuperPoint/Inference_with_SuperPoint_to_detect_interest_points_in_an_image.ipynb) for an inference and visualization example.

## SuperPointConfig[[transformers.SuperPointConfig]]

#### transformers.SuperPointConfig[[transformers.SuperPointConfig]]

```python
transformers.SuperPointConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, encoder_hidden_sizes: list[int] | tuple[int, ...] = (64, 64, 128, 128), decoder_hidden_size: int = 256, keypoint_decoder_dim: int = 65, descriptor_decoder_dim: int = 256, keypoint_threshold: float = 0.005, max_keypoints: int = -1, nms_radius: int = 4, border_removal_distance: int = 4, initializer_range: float = 0.02)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/superpoint/configuration_superpoint.py#L24)

**Parameters:**

encoder_hidden_sizes (`List`, *optional*, defaults to `[64, 64, 128, 128]`) : The number of channels in each convolutional layer in the encoder.

decoder_hidden_size (`int`, *optional*, defaults to `256`) : Dimension of the hidden representations.

keypoint_decoder_dim (`int`, *optional*, defaults to 65) : The output dimension of the keypoint decoder.

descriptor_decoder_dim (`int`, *optional*, defaults to 256) : The output dimension of the descriptor decoder.

keypoint_threshold (`float`, *optional*, defaults to 0.005) : The threshold to use for extracting keypoints.

max_keypoints (`int`, *optional*, defaults to -1) : The maximum number of keypoints to extract. If `-1`, will extract all keypoints.

nms_radius (`int`, *optional*, defaults to 4) : The radius for non-maximum suppression.

border_removal_distance (`int`, *optional*, defaults to 4) : The distance from the border to remove keypoints.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a SuperpointModel. It is used to instantiate a Superpoint
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [magic-leap-community/superpoint](https://huggingface.co/magic-leap-community/superpoint)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import SuperPointConfig, SuperPointForKeypointDetection

>>> # Initializing a SuperPoint superpoint style configuration
>>> configuration = SuperPointConfig()
>>> # Initializing a model from the superpoint style configuration
>>> model = SuperPointForKeypointDetection(configuration)
>>> # Accessing the model configuration
>>> configuration = model.config
```

## SuperPointImageProcessor[[transformers.SuperPointImageProcessor]]

#### transformers.SuperPointImageProcessor[[transformers.SuperPointImageProcessor]]

```python
transformers.SuperPointImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/superpoint/image_processing_superpoint.py#L70)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 480, 'width': 640}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BILINEAR`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `None`) : Whether to normalize the image.

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

do_grayscale (`bool`, *kwargs*, *optional*, defaults to `self.do_grayscale`) : Whether to convert the image to grayscale. Can be overridden by `do_grayscale` in the `preprocess` method.

Constructs a SuperPointImageProcessor image processor.

#### preprocess[[transformers.SuperPointImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], *args, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_utils.py#L382)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

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

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## SuperPointImageProcessorPil[[transformers.SuperPointImageProcessorPil]]

#### transformers.SuperPointImageProcessorPil[[transformers.SuperPointImageProcessorPil]]

```python
transformers.SuperPointImageProcessorPil(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/superpoint/image_processing_pil_superpoint.py#L72)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 480, 'width': 640}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BILINEAR`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `None`) : Whether to normalize the image.

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

do_grayscale (`bool`, *kwargs*, *optional*, defaults to `self.do_grayscale`) : Whether to convert the image to grayscale. Can be overridden by `do_grayscale` in the `preprocess` method.

Constructs a SuperPointImageProcessor image processor.

#### preprocess[[transformers.SuperPointImageProcessorPil.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], *args, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_utils.py#L382)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

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

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

#### post_process_keypoint_detection[[transformers.SuperPointImageProcessorPil.post_process_keypoint_detection]]

```python
post_process_keypoint_detection(outputs: SuperPointKeypointDescriptionOutput, target_sizes: transformers.utils.generic.TensorType | list[tuple])
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/superpoint/image_processing_pil_superpoint.py#L113)

**Parameters:**

outputs (`SuperPointKeypointDescriptionOutput`) : Raw outputs of the model containing keypoints in a relative (x, y) format, with scores and descriptors.

target_sizes (`torch.Tensor` or `list[tuple[int, int]]`) : Tensor of shape `(batch_size, 2)` or list of tuples (`tuple[int, int]`) containing the target size `(height, width)` of each image in the batch. This must be the original image size (before any processing).

**Returns:** `list[Dict]`

A list of dictionaries, each dictionary containing the keypoints in absolute format according
to target_sizes, scores and descriptors for an image in the batch as predicted by the model.

Converts the raw output of [SuperPointForKeypointDetection](/docs/transformers/v5.15.1/en/model_doc/superpoint#transformers.SuperPointForKeypointDetection) into lists of keypoints, scores and descriptors
with coordinates absolute to the original image sizes.

## SuperPointForKeypointDetection[[transformers.SuperPointForKeypointDetection]]

#### transformers.SuperPointForKeypointDetection[[transformers.SuperPointForKeypointDetection]]

```python
transformers.SuperPointForKeypointDetection(config: SuperPointConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/superpoint/modeling_superpoint.py#L352)

**Parameters:**

config ([SuperPointConfig](/docs/transformers/v5.15.1/en/model_doc/superpoint#transformers.SuperPointConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

SuperPoint model outputting keypoints and descriptors.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.SuperPointForKeypointDetection.forward]]

```python
forward(pixel_values: FloatTensor, labels: typing.Optional[torch.LongTensor] = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/superpoint/modeling_superpoint.py#L373)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [SuperPointImageProcessor](/docs/transformers/v5.15.1/en/model_doc/superpoint#transformers.SuperPointImageProcessor). See `SuperPointImageProcessor.__call__()` for details (`processor_class` uses [SuperPointImageProcessor](/docs/transformers/v5.15.1/en/model_doc/superpoint#transformers.SuperPointImageProcessor) for processing images).

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `SuperPointKeypointDescriptionOutput` or `tuple(torch.FloatTensor)`

A `SuperPointKeypointDescriptionOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SuperPointConfig](/docs/transformers/v5.15.1/en/model_doc/superpoint#transformers.SuperPointConfig)) and inputs.

The [SuperPointForKeypointDetection](/docs/transformers/v5.15.1/en/model_doc/superpoint#transformers.SuperPointForKeypointDetection) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*) -- Loss computed during training.
- **keypoints** (`torch.FloatTensor` of shape `(batch_size, num_keypoints, 2)`) -- Relative (x, y) coordinates of predicted keypoints in a given image.
- **scores** (`torch.FloatTensor` of shape `(batch_size, num_keypoints)`) -- Scores of predicted keypoints.
- **descriptors** (`torch.FloatTensor` of shape `(batch_size, num_keypoints, descriptor_size)`) -- Descriptors of predicted keypoints.
- **mask** (`torch.BoolTensor` of shape `(batch_size, num_keypoints)`) -- Mask indicating which values in keypoints, scores and descriptors are keypoint information.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or
  when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each stage) of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states
  (also called feature maps) of the model at the output of each stage.

Examples:

```python
>>> from transformers import AutoImageProcessor, SuperPointForKeypointDetection
>>> import torch
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> processor = AutoImageProcessor.from_pretrained("magic-leap-community/superpoint")
>>> model = SuperPointForKeypointDetection.from_pretrained("magic-leap-community/superpoint")

>>> inputs = processor(image, return_tensors="pt")
>>> outputs = model(**inputs)
```
