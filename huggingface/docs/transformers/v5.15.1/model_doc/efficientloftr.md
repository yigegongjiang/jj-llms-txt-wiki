# EfficientLoFTR

[EfficientLoFTR](https://huggingface.co/papers/2403.04765) is an efficient detector-free local feature matching method that produces semi-dense matches across images with sparse-like speed. It builds upon the original [LoFTR](https://huggingface.co/papers/2104.00680) architecture but introduces significant improvements for both efficiency and accuracy. The key innovation is an aggregated attention mechanism with adaptive token selection that makes the model ~2.5× faster than LoFTR while achieving higher accuracy. EfficientLoFTR can even surpass state-of-the-art efficient sparse matching pipelines like [SuperPoint](./superpoint) + [LightGlue](./lightglue) in terms of speed, making it suitable for large-scale or latency-sensitive applications such as image retrieval and 3D reconstruction.

> [!TIP]
> This model was contributed by [stevenbucaille](https://huggingface.co/stevenbucaille).
>
> Click on the EfficientLoFTR models in the right sidebar for more examples of how to apply EfficientLoFTR to different computer vision tasks.

The example below demonstrates how to match keypoints between two images with [Pipeline](/docs/transformers/v5.15.1/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline

keypoint_matcher = pipeline(task="keypoint-matching", model="zju-community/efficientloftr")

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

from transformers import AutoImageProcessor, AutoModelForKeypointMatching

url_image1 = "https://raw.githubusercontent.com/magicleap/SuperGluePretrainedNetwork/refs/heads/master/assets/phototourism_sample_images/united_states_capitol_98169888_3347710852.jpg"
image1 = Image.open(requests.get(url_image1, stream=True).raw)
url_image2 = "https://raw.githubusercontent.com/magicleap/SuperGluePretrainedNetwork/refs/heads/master/assets/phototourism_sample_images/united_states_capitol_26757027_6717084061.jpg"
image2 = Image.open(requests.get(url_image2, stream=True).raw)

images = [image1, image2]

processor = AutoImageProcessor.from_pretrained("zju-community/efficientloftr")
model = AutoModelForKeypointMatching.from_pretrained("zju-community/efficientloftr", device_map="auto")

inputs = processor(images, return_tensors="pt").to(model.device)
with torch.inference_mode():
    outputs = model(**inputs)

# Post-process to get keypoints and matches
image_sizes = [[(image.height, image.width) for image in images]]
processed_outputs = processor.post_process_keypoint_matching(outputs, image_sizes, threshold=0.2)
```

## Notes

- EfficientLoFTR is designed for efficiency while maintaining high accuracy. It uses an aggregated attention mechanism with adaptive token selection to reduce computational overhead compared to the original LoFTR.

    ```py
    from transformers import AutoImageProcessor, AutoModelForKeypointMatching
    import torch
    from PIL import Image
    import requests
    
    processor = AutoImageProcessor.from_pretrained("zju-community/efficientloftr")
    model = AutoModelForKeypointMatching.from_pretrained("zju-community/efficientloftr", device_map="auto")
    
    # EfficientLoFTR requires pairs of images
    images = [image1, image2]
    inputs = processor(images, return_tensors="pt").to(model.device)
    with torch.inference_mode():
        outputs = model(**inputs)
    
    # Extract matching information
    keypoints = outputs.keypoints        # Keypoints in both images
    matches = outputs.matches            # Matching indices 
    matching_scores = outputs.matching_scores  # Confidence scores
    ```

- The model produces semi-dense matches, offering a good balance between the density of matches and computational efficiency. It excels in handling large viewpoint changes and texture-poor scenarios.

- For better visualization and analysis, use the [post_process_keypoint_matching()](/docs/transformers/v5.15.1/en/model_doc/efficientloftr#transformers.EfficientLoFTRImageProcessor.post_process_keypoint_matching) method to get matches in a more readable format.

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
    visualized_images = processor.visualize_keypoint_matching(images, processed_outputs)
    ```

- EfficientLoFTR uses a novel two-stage correlation layer that achieves accurate subpixel correspondences, improving upon the original LoFTR's fine correlation module.

    

## Resources

- Refer to the [original EfficientLoFTR repository](https://github.com/zju3dv/EfficientLoFTR) for more examples and implementation details.
- [EfficientLoFTR project page](https://zju3dv.github.io/efficientloftr/) with interactive demos and additional information.

## EfficientLoFTRConfig[[transformers.EfficientLoFTRConfig]]

#### transformers.EfficientLoFTRConfig[[transformers.EfficientLoFTRConfig]]

```python
transformers.EfficientLoFTRConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, stage_num_blocks: list[int] | None = None, out_features: list[int] | None = None, stage_stride: list[int] | None = None, hidden_size: int = 256, activation_function: str = 'relu', q_aggregation_kernel_size: int = 4, kv_aggregation_kernel_size: int = 4, q_aggregation_stride: int = 4, kv_aggregation_stride: int = 4, num_attention_layers: int = 4, num_attention_heads: int = 8, attention_dropout: float | int = 0.0, attention_bias: bool = False, mlp_activation_function: str = 'leaky_relu', coarse_matching_skip_softmax: bool = False, coarse_matching_threshold: float = 0.2, coarse_matching_temperature: float = 0.1, coarse_matching_border_removal: int = 2, fine_kernel_size: int = 8, batch_norm_eps: float = 1e-05, rope_parameters: dict | None = None, fine_matching_slice_dim: int = 8, fine_matching_regress_temperature: float = 10.0, initializer_range: float = 0.02)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/efficientloftr/configuration_efficientloftr.py#L24)

**Parameters:**

stage_num_blocks (`List`, *optional*, defaults to [1, 2, 4, 14]) : The number of blocks in each stages

out_features (`list[int]`, *optional*) : Names of the intermediate hidden states (feature maps) to return from the backbone. One of `"stem"`, `"stage1"`, `"stage2"`, etc.

stage_stride (`List`, *optional*, defaults to [2, 1, 2, 2]) : The stride used in each stage

hidden_size (`int`, *optional*, defaults to `256`) : Dimension of the hidden representations.

activation_function (`str`, *optional*, defaults to `relu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

q_aggregation_kernel_size (`int`, *optional*, defaults to 4) : The kernel size of the aggregation of query states in the fusion network

kv_aggregation_kernel_size (`int`, *optional*, defaults to 4) : The kernel size of the aggregation of key and value states in the fusion network

q_aggregation_stride (`int`, *optional*, defaults to 4) : The stride of the aggregation of query states in the fusion network

kv_aggregation_stride (`int`, *optional*, defaults to 4) : The stride of the aggregation of key and value states in the fusion network

num_attention_layers (`int`, *optional*, defaults to 4) : Number of attention layers in the LocalFeatureTransformer

num_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer decoder.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

attention_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

mlp_activation_function (`str`, *optional*, defaults to `"leaky_relu"`) : Activation function used in the attention mlp layer.

coarse_matching_skip_softmax (`bool`, *optional*, defaults to `False`) : Whether to skip softmax or not at the coarse matching step.

coarse_matching_threshold (`float`, *optional*, defaults to 0.2) : The threshold for the minimum score required for a match.

coarse_matching_temperature (`float`, *optional*, defaults to 0.1) : The temperature to apply to the coarse similarity matrix

coarse_matching_border_removal (`int`, *optional*, defaults to 2) : The size of the border to remove during coarse matching

fine_kernel_size (`int`, *optional*, defaults to 8) : Kernel size used for the fine feature matching

batch_norm_eps (`float`, *optional*, defaults to 1e-05) : The epsilon used by the batch normalization layers

rope_parameters (`dict`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

fine_matching_slice_dim (`int`, *optional*, defaults to 8) : The size of the slice used to divide the fine features for the first and second fine matching stages.

fine_matching_regress_temperature (`float`, *optional*, defaults to 10.0) : The temperature to apply to the fine similarity matrix

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a EfficientLoFTRModel. It is used to instantiate a Efficientloftr
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [zju-community/efficientloftr](https://huggingface.co/zju-community/efficientloftr)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:
```python
>>> from transformers import EfficientLoFTRConfig, EfficientLoFTRForKeypointMatching

>>> # Initializing a EfficientLoFTR configuration
>>> configuration = EfficientLoFTRConfig()

>>> # Initializing a model from the EfficientLoFTR configuration
>>> model = EfficientLoFTRForKeypointMatching(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## EfficientLoFTRImageProcessor[[transformers.EfficientLoFTRImageProcessor]]

#### transformers.EfficientLoFTRImageProcessor[[transformers.EfficientLoFTRImageProcessor]]

```python
transformers.EfficientLoFTRImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/efficientloftr/image_processing_efficientloftr.py#L110)

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

Constructs a EfficientLoFTRImageProcessor image processor.

#### preprocess[[transformers.EfficientLoFTRImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/efficientloftr/image_processing_efficientloftr.py#L124)

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

#### post_process_keypoint_matching[[transformers.EfficientLoFTRImageProcessor.post_process_keypoint_matching]]

```python
post_process_keypoint_matching(outputs: EfficientLoFTRKeypointMatchingOutput, target_sizes: transformers.utils.generic.TensorType | list[tuple], threshold: float = 0.0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/efficientloftr/image_processing_efficientloftr.py#L180)

**Parameters:**

outputs (`EfficientLoFTRKeypointMatchingOutput`) : Raw outputs of the model.

target_sizes (`torch.Tensor` or `List[Tuple[Tuple[int, int]]]`, *optional*) : Tensor of shape `(batch_size, 2, 2)` or list of tuples of tuples (`Tuple[int, int]`) containing the target size `(height, width)` of each image in the batch. This must be the original image size (before any processing).

threshold (`float`, *optional*, defaults to 0.0) : Threshold to filter out the matches with low scores.

**Returns:** `List[Dict]`

A list of dictionaries, each dictionary containing the keypoints in the first and second image
of the pair, the matching scores and the matching indices.

Converts the raw output of `EfficientLoFTRKeypointMatchingOutput` into lists of keypoints, scores and descriptors
with coordinates absolute to the original image sizes.

#### visualize_keypoint_matching[[transformers.EfficientLoFTRImageProcessor.visualize_keypoint_matching]]

```python
visualize_keypoint_matching(images, keypoint_matching_output: list)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/efficientloftr/image_processing_efficientloftr.py#L239)

**Parameters:**

images : Image pairs to plot. Same as `EfficientLoFTRImageProcessor.preprocess`. Expects either a list of 2 images or a list of list of 2 images list with pixel values ranging from 0 to 255.

keypoint_matching_output (List[Dict[str, torch.Tensor]]]) : A post processed keypoint matching output

**Returns:** `List[PIL.Image.Image]`

A list of PIL images, each containing the image pairs side by side with the detected
keypoints as well as the matching between them.

Plots the image pairs side by side with the detected keypoints as well as the matching between them.

## EfficientLoFTRImageProcessorPil[[transformers.EfficientLoFTRImageProcessorPil]]

#### transformers.EfficientLoFTRImageProcessorPil[[transformers.EfficientLoFTRImageProcessorPil]]

```python
transformers.EfficientLoFTRImageProcessorPil(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/efficientloftr/image_processing_pil_efficientloftr.py#L108)

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

Constructs a EfficientLoFTRImageProcessor image processor.

#### preprocess[[transformers.EfficientLoFTRImageProcessorPil.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/efficientloftr/image_processing_pil_efficientloftr.py#L122)

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

#### post_process_keypoint_matching[[transformers.EfficientLoFTRImageProcessorPil.post_process_keypoint_matching]]

```python
post_process_keypoint_matching(outputs: EfficientLoFTRKeypointMatchingOutput, target_sizes: transformers.utils.generic.TensorType | list[tuple], threshold: float = 0.0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/efficientloftr/image_processing_pil_efficientloftr.py#L163)

**Parameters:**

outputs (`EfficientLoFTRKeypointMatchingOutput`) : Raw outputs of the model.

target_sizes (`torch.Tensor` or `List[Tuple[Tuple[int, int]]]`, *optional*) : Tensor of shape `(batch_size, 2, 2)` or list of tuples of tuples (`Tuple[int, int]`) containing the target size `(height, width)` of each image in the batch. This must be the original image size (before any processing).

threshold (`float`, *optional*, defaults to 0.0) : Threshold to filter out the matches with low scores.

**Returns:** `List[Dict]`

A list of dictionaries, each dictionary containing the keypoints in the first and second image
of the pair, the matching scores and the matching indices.

Converts the raw output of `EfficientLoFTRKeypointMatchingOutput` into lists of keypoints, scores and descriptors
with coordinates absolute to the original image sizes.

#### visualize_keypoint_matching[[transformers.EfficientLoFTRImageProcessorPil.visualize_keypoint_matching]]

```python
visualize_keypoint_matching(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], keypoint_matching_output: list)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/efficientloftr/image_processing_pil_efficientloftr.py#L225)

**Parameters:**

images (`ImageInput`) : Image pairs to plot. Same as `EfficientLoFTRImageProcessor.preprocess`. Expects either a list of 2 images or a list of list of 2 images list with pixel values ranging from 0 to 255.

keypoint_matching_output (List[Dict[str, torch.Tensor]]]) : A post processed keypoint matching output

**Returns:** `List[PIL.Image.Image]`

A list of PIL images, each containing the image pairs side by side with the detected
keypoints as well as the matching between them.

Plots the image pairs side by side with the detected keypoints as well as the matching between them.

## EfficientLoFTRModel[[transformers.EfficientLoFTRModel]]

#### transformers.EfficientLoFTRModel[[transformers.EfficientLoFTRModel]]

```python
transformers.EfficientLoFTRModel(config: EfficientLoFTRConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/efficientloftr/modeling_efficientloftr.py#L698)

**Parameters:**

config ([EfficientLoFTRConfig](/docs/transformers/v5.15.1/en/model_doc/efficientloftr#transformers.EfficientLoFTRConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

EfficientLoFTR model taking images as inputs and outputting the features of the images.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.EfficientLoFTRModel.forward]]

```python
forward(pixel_values: FloatTensor, labels: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/efficientloftr/modeling_efficientloftr.py#L709)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [EfficientLoFTRImageProcessor](/docs/transformers/v5.15.1/en/model_doc/efficientloftr#transformers.EfficientLoFTRImageProcessor). See `EfficientLoFTRImageProcessor.__call__()` for details (`processor_class` uses [EfficientLoFTRImageProcessor](/docs/transformers/v5.15.1/en/model_doc/efficientloftr#transformers.EfficientLoFTRImageProcessor) for processing images).

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

**Returns:** `BackboneOutput` or `tuple(torch.FloatTensor)`

A `BackboneOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([EfficientLoFTRConfig](/docs/transformers/v5.15.1/en/model_doc/efficientloftr#transformers.EfficientLoFTRConfig)) and inputs.

The [EfficientLoFTRModel](/docs/transformers/v5.15.1/en/model_doc/efficientloftr#transformers.EfficientLoFTRModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **feature_maps** (`tuple(torch.FloatTensor)` of shape `(batch_size, num_channels, height, width)`) -- Feature maps of the stages.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer) of
  shape `(batch_size, sequence_length, hidden_size)` or `(batch_size, num_channels, height, width)`,
  depending on the backbone.

  Hidden-states of the model at the output of each stage plus the initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`. Only applicable if the backbone uses attention.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import AutoImageProcessor, AutoModel
>>> import torch
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "https://github.com/magicleap/SuperGluePretrainedNetwork/blob/master/assets/phototourism_sample_images/london_bridge_78916675_4568141288.jpg?raw=true"
>>> with httpx.stream("GET", url) as response:
...     image1 = Image.open(BytesIO(response.read()))

>>> url = "https://github.com/magicleap/SuperGluePretrainedNetwork/blob/master/assets/phototourism_sample_images/london_bridge_19481797_2295892421.jpg?raw=true"
>>> with httpx.stream("GET", url) as response:
...     image2 = Image.open(BytesIO(response.read()))

>>> images = [image1, image2]

>>> processor = AutoImageProcessor.from_pretrained("zju-community/efficient_loftr")
>>> model = AutoModel.from_pretrained("zju-community/efficient_loftr")

>>> with torch.no_grad():
>>>     inputs = processor(images, return_tensors="pt")
>>>     outputs = model(**inputs)
```

## EfficientLoFTRForKeypointMatching[[transformers.EfficientLoFTRForKeypointMatching]]

#### transformers.EfficientLoFTRForKeypointMatching[[transformers.EfficientLoFTRForKeypointMatching]]

```python
transformers.EfficientLoFTRForKeypointMatching(config: EfficientLoFTRConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/efficientloftr/modeling_efficientloftr.py#L921)

**Parameters:**

config ([EfficientLoFTRConfig](/docs/transformers/v5.15.1/en/model_doc/efficientloftr#transformers.EfficientLoFTRConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

EfficientLoFTR model taking images as inputs and outputting the matching of them.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.EfficientLoFTRForKeypointMatching.forward]]

```python
forward(pixel_values: FloatTensor, labels: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/efficientloftr/modeling_efficientloftr.py#L1286)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses `image_processor_class` for processing images).

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

**Returns:** `EfficientLoFTRKeypointMatchingOutput` or `tuple(torch.FloatTensor)`

A `EfficientLoFTRKeypointMatchingOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.

The [EfficientLoFTRForKeypointMatching](/docs/transformers/v5.15.1/en/model_doc/efficientloftr#transformers.EfficientLoFTRForKeypointMatching) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*) -- Loss computed during training.
- **matches** (`torch.FloatTensor` of shape `(batch_size, 2, num_matches)`) -- Index of keypoint matched in the other image.
- **matching_scores** (`torch.FloatTensor` of shape `(batch_size, 2, num_matches)`) -- Scores of predicted matches.
- **keypoints** (`torch.FloatTensor` of shape `(batch_size, num_keypoints, 2)`) -- Absolute (x, y) coordinates of predicted keypoints in a given image.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*) -- Tuple of `torch.FloatTensor` (one for the output of each stage) of shape `(batch_size, 2, num_channels,
  num_keypoints)`, returned when `output_hidden_states=True` is passed or when
  `config.output_hidden_states=True`)
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, 2, num_heads, num_keypoints,
  num_keypoints)`, returned when `output_attentions=True` is passed or when `config.output_attentions=True`)

Examples:

```python
>>> from transformers import AutoImageProcessor, AutoModel
>>> import torch
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "https://github.com/magicleap/SuperGluePretrainedNetwork/blob/master/assets/phototourism_sample_images/london_bridge_78916675_4568141288.jpg?raw=true"
>>> with httpx.stream("GET", url) as response:
...     image1 = Image.open(BytesIO(response.read()))

>>> url = "https://github.com/magicleap/SuperGluePretrainedNetwork/blob/master/assets/phototourism_sample_images/london_bridge_19481797_2295892421.jpg?raw=true"
>>> with httpx.stream("GET", url) as response:
...     image2 = Image.open(BytesIO(response.read()))

>>> images = [image1, image2]

>>> processor = AutoImageProcessor.from_pretrained("zju-community/efficient_loftr")
>>> model = AutoModel.from_pretrained("zju-community/efficient_loftr")

>>> with torch.no_grad():
>>>     inputs = processor(images, return_tensors="pt")
>>>     outputs = model(**inputs)
```
