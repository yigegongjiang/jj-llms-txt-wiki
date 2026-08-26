# LightGlue

[LightGlue](https://huggingface.co/papers/2306.13643) is a deep neural network that learns to match local features across images. It revisits multiple design decisions of SuperGlue and derives simple but effective improvements. Cumulatively, these improvements make LightGlue more efficient - in terms of both memory and computation, more accurate, and much easier to train. Similar to [SuperGlue](https://huggingface.co/magic-leap-community/superglue_outdoor), this model consists of matching two sets of local features extracted from two images, with the goal of being faster than SuperGlue. Paired with the [SuperPoint model](https://huggingface.co/magic-leap-community/superpoint), it can be used to match two images and estimate the pose between them.

You can find all the original LightGlue checkpoints under the [ETH-CVG](https://huggingface.co/ETH-CVG) organization.

> [!TIP]
> This model was contributed by [stevenbucaille](https://huggingface.co/stevenbucaille).
>
> Click on the LightGlue models in the right sidebar for more examples of how to apply LightGlue to different computer vision tasks.

The example below demonstrates how to match keypoints between two images with [Pipeline](/docs/transformers/v5.15.1/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline

keypoint_matcher = pipeline(task="keypoint-matching", model="ETH-CVG/lightglue_superpoint")

url_0 = "https://raw.githubusercontent.com/magicleap/SuperGluePretrainedNetwork/refs/heads/master/assets/phototourism_sample_images/united_states_capitol_98169888_3347710852.jpg"
url_1 = "https://raw.githubusercontent.com/magicleap/SuperGluePretrainedNetwork/refs/heads/master/assets/phototourism_sample_images/united_states_capitol_26757027_6717084061.jpg"

results = keypoint_matcher([url_0, url_1], threshold=0.9)
print(results[0])
# {'keypoint_image_0': {'x': ..., 'y': ...}, 'keypoint_image_1': {'x': ..., 'y': ...}, 'score': ...}
```

```python
import requests
import torch
from PIL import Image

from transformers import AutoImageProcessor, AutoModel

url_image1 = "https://raw.githubusercontent.com/magicleap/SuperGluePretrainedNetwork/refs/heads/master/assets/phototourism_sample_images/united_states_capitol_98169888_3347710852.jpg"
image1 = Image.open(requests.get(url_image1, stream=True).raw)
url_image2 = "https://raw.githubusercontent.com/magicleap/SuperGluePretrainedNetwork/refs/heads/master/assets/phototourism_sample_images/united_states_capitol_26757027_6717084061.jpg"
image2 = Image.open(requests.get(url_image2, stream=True).raw)

images = [image1, image2]

processor = AutoImageProcessor.from_pretrained("ETH-CVG/lightglue_superpoint")
model = AutoModel.from_pretrained("ETH-CVG/lightglue_superpoint", device_map="auto")

inputs = processor(images, return_tensors="pt").to(model.device)
with torch.inference_mode():
    outputs = model(**inputs)

# Post-process to get keypoints and matches
image_sizes = [[(image.height, image.width) for image in images]]
processed_outputs = processor.post_process_keypoint_matching(outputs, image_sizes, threshold=0.2)
```

## Notes

- LightGlue is adaptive to the task difficulty. Inference is much faster on image pairs that are intuitively easy to match, for example, because of a larger visual overlap or limited appearance change.

    ```py
    from transformers import AutoImageProcessor, AutoModel
    import torch
    from PIL import Image
    import requests

    processor = AutoImageProcessor.from_pretrained("ETH-CVG/lightglue_superpoint")
    model = AutoModel.from_pretrained("ETH-CVG/lightglue_superpoint", device_map="auto")

    # LightGlue requires pairs of images
    images = [image1, image2]
    inputs = processor(images, return_tensors="pt").to(model.device)
    with torch.inference_mode():
        outputs = model(**inputs)

    # Extract matching information
    keypoints0 = outputs.keypoints0  # Keypoints in first image
    keypoints1 = outputs.keypoints1  # Keypoints in second image
    matches = outputs.matches        # Matching indices
    matching_scores = outputs.matching_scores  # Confidence scores
    ```

- The model outputs matching indices, keypoints, and confidence scores for each match, similar to SuperGlue but with improved efficiency.
- For better visualization and analysis, use the [LightGlueImageProcessor.post_process_keypoint_matching()](/docs/transformers/v5.15.1/en/model_doc/lightglue#transformers.LightGlueImageProcessor.post_process_keypoint_matching) method to get matches in a more readable format.

    ```py
    # Process outputs for visualization
    image_sizes = [[(image.height, image.width) for image in images]]
    processed_outputs = processor.post_process_keypoint_matching(outputs, image_sizes, threshold=0.2)

    for i, output in enumerate(processed_outputs):
        print(f"For the image pair {i}")
        for keypoint0, keypoint1, matching_score in zip(
                output["keypoints0"], output["keypoints1"], output["matching_scores"]
        ):
            print(f"Keypoint at {keypoint0.numpy()} matches with keypoint at {keypoint1.numpy()} with score {matching_score}")
    ```

- Visualize the matches between the images using the built-in plotting functionality.

    ```py
    # Easy visualization using the built-in plotting method
    processor.visualize_keypoint_matching(images, processed_outputs)
    ```

    

## Resources

- Refer to the [original LightGlue repository](https://github.com/cvg/LightGlue) for more examples and implementation details.

## LightGlueConfig[[transformers.LightGlueConfig]]

#### transformers.LightGlueConfig[[transformers.LightGlueConfig]]

```python
transformers.LightGlueConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, keypoint_detector_config: dict | transformers.models.superpoint.configuration_superpoint.SuperPointConfig | None = None, descriptor_dim: int = 256, num_hidden_layers: int = 9, num_attention_heads: int = 4, num_key_value_heads: int | None = None, depth_confidence: float = 0.95, width_confidence: float = 0.99, filter_threshold: float = 0.1, initializer_range: float = 0.02, hidden_act: str = 'gelu', attention_dropout: float | int = 0.0, attention_bias: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lightglue/configuration_lightglue.py#L31)

**Parameters:**

keypoint_detector_config (`Union[AutoConfig, dict]`,  *optional*, defaults to `SuperPointConfig`) : The config object or dictionary of the keypoint detector.

descriptor_dim (`int`, *optional*, defaults to 256) : The dimension of the descriptors.

num_hidden_layers (`int`, *optional*, defaults to `9`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `4`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

depth_confidence (`float`, *optional*, defaults to 0.95) : The confidence threshold used to perform early stopping

width_confidence (`float`, *optional*, defaults to 0.99) : The confidence threshold used to prune points

filter_threshold (`float`, *optional*, defaults to 0.1) : The confidence threshold used to filter matches

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

attention_bias (`bool`, *optional*, defaults to `True`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

This is the configuration class to store the configuration of a LightGlueForKeypointMatching. It is used to instantiate a Lightglue
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [ETH-CVG/lightglue_superpoint](https://huggingface.co/ETH-CVG/lightglue_superpoint)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:
```python
>>> from transformers import LightGlueConfig, LightGlueForKeypointMatching

>>> # Initializing a LightGlue style configuration
>>> configuration = LightGlueConfig()

>>> # Initializing a model from the LightGlue style configuration
>>> model = LightGlueForKeypointMatching(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## LightGlueImageProcessor[[transformers.LightGlueImageProcessor]]

#### transformers.LightGlueImageProcessor[[transformers.LightGlueImageProcessor]]

```python
transformers.LightGlueImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lightglue/image_processing_lightglue.py#L121)

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

Constructs a LightGlueImageProcessor image processor.

#### preprocess[[transformers.LightGlueImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lightglue/image_processing_lightglue.py#L135)

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

do_grayscale (`bool`, *kwargs*, *optional*, defaults to `self.do_grayscale`) : Whether to convert the image to grayscale. Can be overridden by `do_grayscale` in the `preprocess` method.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

#### post_process_keypoint_matching[[transformers.LightGlueImageProcessor.post_process_keypoint_matching]]

```python
post_process_keypoint_matching(outputs: LightGlueKeypointMatchingOutput, target_sizes: transformers.utils.generic.TensorType | list[tuple], threshold: float = 0.0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lightglue/image_processing_lightglue.py#L191)

**Parameters:**

outputs (`LightGlueKeypointMatchingOutput`) : Raw outputs of the model.

target_sizes (`torch.Tensor` or `list[tuple[tuple[int, int]]]`, *optional*) : Tensor of shape `(batch_size, 2, 2)` or list of tuples of tuples (`tuple[int, int]`) containing the target size `(height, width)` of each image in the batch. This must be the original image size (before any processing).

threshold (`float`, *optional*, defaults to `0.0`) : Threshold to filter out the matches with low scores.

**Returns:** `list[Dict]`

A list of dictionaries, each dictionary containing the keypoints in the first and second image
of the pair, the matching scores and the matching indices.

Converts the raw output of `LightGlueKeypointMatchingOutput` into lists of keypoints, scores and descriptors
with coordinates absolute to the original image sizes.

#### visualize_keypoint_matching[[transformers.LightGlueImageProcessor.visualize_keypoint_matching]]

```python
visualize_keypoint_matching(images, keypoint_matching_output: list)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lightglue/image_processing_lightglue.py#L259)

**Parameters:**

images : Image pairs to plot. Same as `EfficientLoFTRImageProcessor.preprocess`. Expects either a list of 2 images or a list of list of 2 images list with pixel values ranging from 0 to 255.

keypoint_matching_output (List[Dict[str, torch.Tensor]]]) : A post processed keypoint matching output

**Returns:** `List[PIL.Image.Image]`

A list of PIL images, each containing the image pairs side by side with the detected
keypoints as well as the matching between them.

Plots the image pairs side by side with the detected keypoints as well as the matching between them.

## LightGlueImageProcessorPil[[transformers.LightGlueImageProcessorPil]]

#### transformers.LightGlueImageProcessorPil[[transformers.LightGlueImageProcessorPil]]

```python
transformers.LightGlueImageProcessorPil(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lightglue/image_processing_pil_lightglue.py#L122)

#### preprocess[[transformers.LightGlueImageProcessorPil.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lightglue/image_processing_pil_lightglue.py#L136)

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

do_grayscale (`bool`, *kwargs*, *optional*, defaults to `self.do_grayscale`) : Whether to convert the image to grayscale. Can be overridden by `do_grayscale` in the `preprocess` method.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

#### post_process_keypoint_matching[[transformers.LightGlueImageProcessorPil.post_process_keypoint_matching]]

```python
post_process_keypoint_matching(outputs: LightGlueKeypointMatchingOutput, target_sizes: transformers.utils.generic.TensorType | list[tuple], threshold: float = 0.0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lightglue/image_processing_pil_lightglue.py#L177)

**Parameters:**

outputs (`LightGlueKeypointMatchingOutput`) : Raw outputs of the model.

target_sizes (`torch.Tensor` or `list[tuple[tuple[int, int]]]`, *optional*) : Tensor of shape `(batch_size, 2, 2)` or list of tuples of tuples (`tuple[int, int]`) containing the target size `(height, width)` of each image in the batch. This must be the original image size (before any processing).

threshold (`float`, *optional*, defaults to `0.0`) : Threshold to filter out the matches with low scores.

**Returns:** `list[Dict]`

A list of dictionaries, each dictionary containing the keypoints in the first and second image
of the pair, the matching scores and the matching indices.

Converts the raw output of `LightGlueKeypointMatchingOutput` into lists of keypoints, scores and descriptors
with coordinates absolute to the original image sizes.

#### visualize_keypoint_matching[[transformers.LightGlueImageProcessorPil.visualize_keypoint_matching]]

```python
visualize_keypoint_matching(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], keypoint_matching_output: list)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lightglue/image_processing_pil_lightglue.py#L248)

**Parameters:**

images (`ImageInput`) : Image pairs to plot. Same as `LightGlueImageProcessor.preprocess`. Expects either a list of 2 images or a list of list of 2 images list with pixel values ranging from 0 to 255.

keypoint_matching_output (List[Dict[str, torch.Tensor]]]) : A post processed keypoint matching output

**Returns:** `List[PIL.Image.Image]`

A list of PIL images, each containing the image pairs side by side with the detected
keypoints as well as the matching between them.

Plots the image pairs side by side with the detected keypoints as well as the matching between them.

## LightGlueForKeypointMatching[[transformers.LightGlueForKeypointMatching]]

#### transformers.LightGlueForKeypointMatching[[transformers.LightGlueForKeypointMatching]]

```python
transformers.LightGlueForKeypointMatching(config: LightGlueConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lightglue/modeling_lightglue.py#L487)

**Parameters:**

config ([LightGlueConfig](/docs/transformers/v5.15.1/en/model_doc/lightglue#transformers.LightGlueConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

LightGlue model taking images as inputs and outputting the matching of them.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.LightGlueForKeypointMatching.forward]]

```python
forward(pixel_values: FloatTensor, labels: typing.Optional[torch.LongTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lightglue/modeling_lightglue.py#L864)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [LightGlueImageProcessor](/docs/transformers/v5.15.1/en/model_doc/lightglue#transformers.LightGlueImageProcessor). See `LightGlueImageProcessor.__call__()` for details (`processor_class` uses [LightGlueImageProcessor](/docs/transformers/v5.15.1/en/model_doc/lightglue#transformers.LightGlueImageProcessor) for processing images).

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

**Returns:** `LightGlueKeypointMatchingOutput` or `tuple(torch.FloatTensor)`

A `LightGlueKeypointMatchingOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LightGlueConfig](/docs/transformers/v5.15.1/en/model_doc/lightglue#transformers.LightGlueConfig)) and inputs.

The [LightGlueForKeypointMatching](/docs/transformers/v5.15.1/en/model_doc/lightglue#transformers.LightGlueForKeypointMatching) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*) -- Loss computed during training.
- **matches** (`torch.FloatTensor` of shape `(batch_size, 2, num_matches)`) -- Index of keypoint matched in the other image.
- **matching_scores** (`torch.FloatTensor` of shape `(batch_size, 2, num_matches)`) -- Scores of predicted matches.
- **keypoints** (`torch.FloatTensor` of shape `(batch_size, num_keypoints, 2)`) -- Absolute (x, y) coordinates of predicted keypoints in a given image.
- **prune** (`torch.IntTensor` of shape `(batch_size, num_keypoints)`) -- Pruning mask indicating which keypoints are removed and at which layer.
- **mask** (`torch.BoolTensor` of shape `(batch_size, num_keypoints)`) -- Mask indicating which values in matches, matching_scores, keypoints and prune are keypoint matching
  information.
- **hidden_states** (`Tuple[torch.FloatTensor, ...]`, *optional*) -- Tuple of `torch.FloatTensor` (one for the output of each stage) of shape `(batch_size, 2, num_channels,
  num_keypoints)` returned when `output_hidden_states=True` is passed or when
  `config.output_hidden_states=True`
- **attentions** (`Tuple[torch.FloatTensor, ...]`, *optional*) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, 2, num_heads, num_keypoints,
  num_keypoints)` returned when `output_attentions=True` is passed or when
  `config.output_attentions=True`
