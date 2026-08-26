# EoMT

## Overview

[The Encoder-only Mask Transformer](https://www.tue-mps.org/eomt) (EoMT) model was introduced in the CVPR 2025 Highlight Paper *[Your ViT is Secretly an Image Segmentation Model](https://huggingface.co/papers/2503.19108)* by Tommie Kerssies, Niccolò Cavagnero, Alexander Hermans, Narges Norouzi, Giuseppe Averta, Bastian Leibe, Gijs Dubbelman, and Daan de Geus.
EoMT reveals Vision Transformers can perform image segmentation efficiently without task-specific components.

The abstract from the paper is the following:

*Vision Transformers (ViTs) have shown remarkable performance and scalability across various computer vision tasks. To apply single-scale ViTs to image segmentation, existing methods adopt a convolutional adapter to generate multi-scale features, a pixel decoder to fuse these features, and a Transformer decoder that uses the fused features to make predictions. In this paper, we show that the inductive biases introduced by these task-specific components can instead be learned by the ViT itself, given sufficiently large models and extensive pre-training. Based on these findings, we introduce the Encoder-only Mask Transformer (EoMT), which repurposes the plain ViT architecture to conduct image segmentation. With large-scale models and pre-training, EoMT obtains a segmentation accuracy similar to state-of-the-art models that use task-specific components. At the same time, EoMT is significantly faster than these methods due to its architectural simplicity, e.g., up to 4x faster with ViT-L. Across a range of model sizes, EoMT demonstrates an optimal balance between segmentation accuracy and prediction speed, suggesting that compute resources are better spent on scaling the ViT itself rather than adding architectural complexity.*

This model was contributed by [Yaswanth Gali](https://huggingface.co/yaswanthgali).
The original code can be found [here](https://github.com/tue-mps/eomt).

## Architecture Info

The `EoMT` model uses a DINOv2-pretrained Vision Transformer with **register tokens** as its backbone. EoMT simplifies the segmentation pipeline by relying solely on the encoder, eliminating the need for task-specific decoders commonly used in prior approaches.

Architecturally, EoMT introduces a small set of **learned queries** and a lightweight **mask prediction module**. These queries are injected into the final encoder blocks, enabling **joint attention** between image patches and object queries. During training, **masked attention** is applied to constrain each query to focus on its corresponding region—effectively mimicking cross-attention. This constraint is gradually phased out via a **mask annealing strategy**, allowing for **efficient, decoder-free inference** without compromising segmentation performance.

  <img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/eomt_architecture.png"
       alt="drawing" width="500"/>

The model supports semantic, instance, and panoptic segmentation using a unified architecture and task-specific post-processing.

## Usage Examples

Use the Hugging Face implementation of EoMT for inference with pre-trained models.

### Semantic Segmentation

The EoMT model performs semantic segmentation using sliding-window inference. The input image is resized such that the shorter side matches the target input size, then it is split into overlapping crops. Each crop is then passed through the model. After inference, the predicted logits from each crop are stitched back together and rescaled to the original image size to get the final segmentation mask.

> **Note:**  
> If you want to use a custom target size for **semantic segmentation**, specify it in the following format:  
> `{"shortest_edge": 512}`  
> Notice that `longest_edge` is not provided here — this is intentional. For semantic segmentation, images are typically **scaled so that the shortest edge is greater than or equal to the target size** hence longest_edge is not necessary.

```python
import matplotlib.pyplot as plt
import requests
import torch
from PIL import Image

from transformers import AutoImageProcessor, EomtForUniversalSegmentation

model_id = "tue-mps/ade20k_semantic_eomt_large_512"
processor = AutoImageProcessor.from_pretrained(model_id)
model = EomtForUniversalSegmentation.from_pretrained(model_id, device_map="auto")

image = Image.open(requests.get("http://images.cocodataset.org/val2017/000000039769.jpg", stream=True).raw)

inputs = processor(
    images=image,
    return_tensors="pt",
)

with torch.inference_mode():
    outputs = model(**inputs)

# Prepare the original image size in the format (height, width)
target_sizes = [(image.height, image.width)]

# Post-process the model outputs to get final segmentation prediction
preds = processor.post_process_semantic_segmentation(
    outputs,
    target_sizes=target_sizes,
)

# Visualize the segmentation mask
plt.imshow(preds[0])
plt.axis("off")
plt.title("Semantic Segmentation")
plt.show()
```

### Instance Segmentation

The EoMT model performs instance segmentation using padded inference. The input image is resized so that the longer side matches the target input size, and the shorter side is zero-padded to form a square. The resulting mask and class logits are combined through post-processing (adapted from Mask2Former) to produce a unified instance segmentation map, along with segment metadata like segment id, class labels and confidence scores.

> **Note:**  
> To use a custom target size, specify the size as a dictionary in the following format:  
> `{"shortest_edge": 512, "longest_edge": 512}`  
> For both instance and panoptic segmentation, input images will be **scaled and padded** to this target size.

```python
import matplotlib.pyplot as plt
import requests
import torch
from PIL import Image

from transformers import AutoImageProcessor, EomtForUniversalSegmentation

model_id = "tue-mps/coco_instance_eomt_large_640"
processor = AutoImageProcessor.from_pretrained(model_id)
model = EomtForUniversalSegmentation.from_pretrained(model_id, device_map="auto")

image = Image.open(requests.get("http://images.cocodataset.org/val2017/000000039769.jpg", stream=True).raw)

inputs = processor(
    images=image,
    return_tensors="pt",
)

with torch.inference_mode():
    outputs = model(**inputs)

# Prepare the original image size in the format (height, width)
target_sizes = [(image.height, image.width)]

# Post-process the model outputs to get final segmentation prediction
preds = processor.post_process_instance_segmentation(
    outputs,
    target_sizes=target_sizes,
)

# Visualize the segmentation mask
plt.imshow(preds[0]["segmentation"])
plt.axis("off")
plt.title("Instance Segmentation")
plt.show()
```

### Panoptic Segmentation

The EoMT model performs panoptic segmentation using the same padded inference strategy as in instance segmentation. After padding and normalization, the model predicts both thing (instances) and stuff (amorphous regions) classes. The resulting mask and class logits are combined through post-processing (adapted from Mask2Former) to produce a unified panoptic segmentation map, along with segment metadata like segment id, class labels and confidence scores.

```python
import matplotlib.pyplot as plt
import requests
import torch
from PIL import Image

from transformers import AutoImageProcessor, EomtForUniversalSegmentation

model_id = "tue-mps/coco_panoptic_eomt_large_640"
processor = AutoImageProcessor.from_pretrained(model_id)
model = EomtForUniversalSegmentation.from_pretrained(model_id, device_map="auto")

image = Image.open(requests.get("http://images.cocodataset.org/val2017/000000039769.jpg", stream=True).raw)

inputs = processor(
    images=image,
    return_tensors="pt",
)

with torch.inference_mode():
    outputs = model(**inputs)

# Prepare the original image size in the format (height, width)
target_sizes = [(image.height, image.width)]

# Post-process the model outputs to get final segmentation prediction
preds = processor.post_process_panoptic_segmentation(
    outputs,
    target_sizes=target_sizes,
)

# Visualize the panoptic segmentation mask
plt.imshow(preds[0]["segmentation"])
plt.axis("off")
plt.title("Panoptic Segmentation")
plt.show()
```

## EomtImageProcessor[[transformers.EomtImageProcessor]]

#### transformers.EomtImageProcessor[[transformers.EomtImageProcessor]]

```python
transformers.EomtImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/eomt/image_processing_eomt.py#L214)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'shortest_edge' : 640, 'longest_edge': 640}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BILINEAR`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.485, 0.456, 0.406]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.229, 0.224, 0.225]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

do_split_image (`bool`, *kwargs*, *optional*, defaults to `self.do_split_image`) : Whether to split the input images into overlapping patches for semantic segmentation. If set to `True`, the input images will be split into patches of size `size["shortest_edge"]` with an overlap between patches. Otherwise, the input images will be padded to the target size.

ignore_index (`int`, *kwargs*, *optional*, defaults to `self.ignore_index`) : Label to be assigned to background pixels in segmentation maps. If provided, segmentation map pixels denoted with 0 (background) will be replaced with `ignore_index`.

Constructs a EomtImageProcessor image processor.

#### preprocess[[transformers.EomtImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], segmentation_maps: list[torch.Tensor] | None = None, instance_id_to_semantic_id: dict[int, int] | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/eomt/image_processing_eomt.py#L273)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

segmentation_maps (`ImageInput`, *optional*) : The segmentation maps to preprocess for corresponding images.

instance_id_to_semantic_id (`list[dict[int, int]]` or `dict[int, int]`, *optional*) : A mapping between object instance ids and class ids.

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

do_split_image (`bool`, *kwargs*, *optional*, defaults to `self.do_split_image`) : Whether to split the input images into overlapping patches for semantic segmentation. If set to `True`, the input images will be split into patches of size `size["shortest_edge"]` with an overlap between patches. Otherwise, the input images will be padded to the target size.

ignore_index (`int`, *kwargs*, *optional*, defaults to `self.ignore_index`) : Label to be assigned to background pixels in segmentation maps. If provided, segmentation map pixels denoted with 0 (background) will be replaced with `ignore_index`.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

#### post_process_semantic_segmentation[[transformers.EomtImageProcessor.post_process_semantic_segmentation]]

```python
post_process_semantic_segmentation(outputs, target_sizes: list, size: dict[str, int] | None = None, return_segmentation_scores: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/eomt/image_processing_eomt.py#L502)

**Parameters:**

outputs ([EomtForUniversalSegmentation](/docs/transformers/v5.15.1/en/model_doc/eomt#transformers.EomtForUniversalSegmentation)) : Raw outputs of the model.

target_sizes (`list[tuple[int, int]]`) : A list of tuples (`tuple[int, int]`) containing the target size (height, width) of each image in the batch.

size (`dict[str, int]`, *optional*) : The size to which the intermediate masks are interpolated. Defaults to `self.size`.

return_segmentation_scores (`bool`, *optional*, defaults to `False`) : Whether to return segmentation scores alongside the segmentation map. When `True`, each element of the returned list is a `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape `(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`).

**Returns:** `list[torch.Tensor]` or `list[SemanticSegmentationPostProcessorOutput]`

When
`return_segmentation_scores=False` (default), a list of length `batch_size` where each item is a
segmentation map of shape `(height, width)` with class IDs. When `return_segmentation_scores=True`,
a list of `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape
`(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`). In both cases,
`(height, width)` corresponds to the target size.

Converts the output of [EomtForUniversalSegmentation](/docs/transformers/v5.15.1/en/model_doc/eomt#transformers.EomtForUniversalSegmentation) into semantic segmentation maps.

#### post_process_instance_segmentation[[transformers.EomtImageProcessor.post_process_instance_segmentation]]

```python
post_process_instance_segmentation(outputs, target_sizes: list, threshold: float = 0.8, size: dict[str, int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/eomt/image_processing_eomt.py#L635)

Post-processes model outputs into Instance Segmentation Predictions.

#### post_process_panoptic_segmentation[[transformers.EomtImageProcessor.post_process_panoptic_segmentation]]

```python
post_process_panoptic_segmentation(outputs, target_sizes: list, threshold: float = 0.8, mask_threshold: float = 0.5, overlap_mask_area_threshold: float = 0.8, stuff_classes: list[int] | None = None, size: dict[str, int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/eomt/image_processing_eomt.py#L578)

Post-processes model outputs into final panoptic segmentation prediction.

## EomtImageProcessorPil[[transformers.EomtImageProcessorPil]]

#### transformers.EomtImageProcessorPil[[transformers.EomtImageProcessorPil]]

```python
transformers.EomtImageProcessorPil(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/eomt/image_processing_pil_eomt.py#L215)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'shortest_edge' : 640, 'longest_edge': 640}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BILINEAR`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.485, 0.456, 0.406]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.229, 0.224, 0.225]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

do_split_image (`bool`, *kwargs*, *optional*, defaults to `self.do_split_image`) : Whether to split the input images into overlapping patches for semantic segmentation. If set to `True`, the input images will be split into patches of size `size["shortest_edge"]` with an overlap between patches. Otherwise, the input images will be padded to the target size.

ignore_index (`int`, *kwargs*, *optional*, defaults to `self.ignore_index`) : Label to be assigned to background pixels in segmentation maps. If provided, segmentation map pixels denoted with 0 (background) will be replaced with `ignore_index`.

Constructs a EomtImageProcessor image processor.

#### preprocess[[transformers.EomtImageProcessorPil.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], segmentation_maps: list[torch.Tensor] | None = None, instance_id_to_semantic_id: dict[int, int] | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/eomt/image_processing_pil_eomt.py#L281)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

segmentation_maps (`ImageInput`, *optional*) : The segmentation maps to preprocess for corresponding images.

instance_id_to_semantic_id (`list[dict[int, int]]` or `dict[int, int]`, *optional*) : A mapping between object instance ids and class ids.

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

do_split_image (`bool`, *kwargs*, *optional*, defaults to `self.do_split_image`) : Whether to split the input images into overlapping patches for semantic segmentation. If set to `True`, the input images will be split into patches of size `size["shortest_edge"]` with an overlap between patches. Otherwise, the input images will be padded to the target size.

ignore_index (`int`, *kwargs*, *optional*, defaults to `self.ignore_index`) : Label to be assigned to background pixels in segmentation maps. If provided, segmentation map pixels denoted with 0 (background) will be replaced with `ignore_index`.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

#### post_process_semantic_segmentation[[transformers.EomtImageProcessorPil.post_process_semantic_segmentation]]

```python
post_process_semantic_segmentation(outputs, target_sizes: list, size: dict[str, int] | None = None, return_segmentation_scores: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/eomt/image_processing_pil_eomt.py#L495)

**Parameters:**

outputs ([EomtForUniversalSegmentation](/docs/transformers/v5.15.1/en/model_doc/eomt#transformers.EomtForUniversalSegmentation)) : Raw outputs of the model.

target_sizes (`list[tuple[int, int]]`) : A list of tuples (`tuple[int, int]`) containing the target size (height, width) of each image in the batch.

size (`dict[str, int]`, *optional*) : The size to which the intermediate masks are interpolated. Defaults to `self.size`.

return_segmentation_scores (`bool`, *optional*, defaults to `False`) : Whether to return segmentation scores alongside the segmentation map. When `True`, each element of the returned list is a `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape `(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`).

**Returns:** `list[torch.Tensor]` or `list[SemanticSegmentationPostProcessorOutput]`

When
`return_segmentation_scores=False` (default), a list of length `batch_size` where each item is a
segmentation map of shape `(height, width)` with class IDs. When `return_segmentation_scores=True`,
a list of `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape
`(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`). In both cases,
`(height, width)` corresponds to the target size.

Converts the output of [EomtForUniversalSegmentation](/docs/transformers/v5.15.1/en/model_doc/eomt#transformers.EomtForUniversalSegmentation) into semantic segmentation maps.

#### post_process_instance_segmentation[[transformers.EomtImageProcessorPil.post_process_instance_segmentation]]

```python
post_process_instance_segmentation(outputs, target_sizes: list, threshold: float = 0.8, size: dict[str, int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/eomt/image_processing_pil_eomt.py#L620)

Post-processes model outputs into Instance Segmentation Predictions.

#### post_process_panoptic_segmentation[[transformers.EomtImageProcessorPil.post_process_panoptic_segmentation]]

```python
post_process_panoptic_segmentation(outputs, target_sizes: list, threshold: float = 0.8, mask_threshold: float = 0.5, overlap_mask_area_threshold: float = 0.8, stuff_classes: list[int] | None = None, size: dict[str, int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/eomt/image_processing_pil_eomt.py#L567)

Post-processes model outputs into final panoptic segmentation prediction.

## EomtConfig[[transformers.EomtConfig]]

#### transformers.EomtConfig[[transformers.EomtConfig]]

```python
transformers.EomtConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 1024, num_hidden_layers: int = 24, num_attention_heads: int = 16, hidden_act: str = 'gelu', hidden_dropout_prob: float | int = 0.0, initializer_range: float = 0.02, layer_norm_eps: float = 1e-06, image_size: int | list[int] | tuple[int, int] = 640, patch_size: int | list[int] | tuple[int, int] = 16, num_channels: int = 3, mlp_ratio: int = 4, layerscale_value: float = 1.0, drop_path_rate: float | int = 0.0, num_upscale_blocks: int = 2, attention_dropout: float | int = 0.0, use_swiglu_ffn: bool = False, num_blocks: int = 4, no_object_weight: float = 0.1, class_weight: float = 2.0, mask_weight: float = 5.0, dice_weight: float = 5.0, train_num_points: int = 12544, oversample_ratio: float = 3.0, importance_sample_ratio: float = 0.75, num_queries: int = 200, num_register_tokens: int = 4)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/eomt/configuration_eomt.py#L28)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `1024`) : Dimension of the hidden representations.

num_hidden_layers (`int`, *optional*, defaults to `24`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer decoder.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

hidden_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

image_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `640`) : The size (resolution) of each image.

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) : The size (resolution) of each patch.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

mlp_ratio (`int`, *optional*, defaults to `4`) : Ratio of the MLP hidden dim to the embedding dim.

layerscale_value (`float`, *optional*, defaults to 1.0) : Initial value for the LayerScale parameter.

drop_path_rate (`Union[float, int]`, *optional*, defaults to `0.0`) : Drop path rate for the patch fusion.

num_upscale_blocks (`int`, *optional*, defaults to 2) : Number of upsampling blocks used in the decoder or segmentation head.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

use_swiglu_ffn (`bool`, *optional*, defaults to `False`) : Whether to use the SwiGLU feedforward neural network.

num_blocks (`int`, *optional*, defaults to 4) : Number of feature blocks or stages in the architecture.

no_object_weight (`float`, *optional*, defaults to 0.1) : Loss weight for the 'no object' class in panoptic/instance segmentation.

class_weight (`float`, *optional*, defaults to 2.0) : Loss weight for classification targets.

mask_weight (`float`, *optional*, defaults to 5.0) : Loss weight for mask prediction.

dice_weight (`float`, *optional*, defaults to `5.0`) : Relative weight of the dice loss in the panoptic segmentation loss.

train_num_points (`int`, *optional*, defaults to 12544) : Number of points to sample for mask loss computation during training.

oversample_ratio (`float`, *optional*, defaults to 3.0) : Oversampling ratio used in point sampling for mask training.

importance_sample_ratio (`float`, *optional*, defaults to 0.75) : Ratio of points to sample based on importance during training.

num_queries (`int`, *optional*, defaults to 200) : Number of object queries in the Transformer.

num_register_tokens (`int`, *optional*, defaults to 4) : Number of learnable register tokens added to the transformer input.

This is the configuration class to store the configuration of a EomtModel. It is used to instantiate a Eomt
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [tue-mps/coco_panoptic_eomt_large_640](https://huggingface.co/tue-mps/coco_panoptic_eomt_large_640)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import EomtConfig, EomtForUniversalSegmentation

>>> # Initialize configuration
>>> config = EomtConfig()

>>> # Initialize model
>>> model = EomtForUniversalSegmentation(config)

>>> # Access config
>>> config = model.config
```

## EomtForUniversalSegmentation[[transformers.EomtForUniversalSegmentation]]

#### transformers.EomtForUniversalSegmentation[[transformers.EomtForUniversalSegmentation]]

```python
transformers.EomtForUniversalSegmentation(config: EomtConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/eomt/modeling_eomt.py#L1026)

**Parameters:**

config ([EomtConfig](/docs/transformers/v5.15.1/en/model_doc/eomt#transformers.EomtConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The EoMT Model with head on top for instance/semantic/panoptic segmentation.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.EomtForUniversalSegmentation.forward]]

```python
forward(pixel_values: Tensor, mask_labels: list[torch.Tensor] | None = None, class_labels: list[torch.Tensor] | None = None, patch_offsets: list[torch.Tensor] | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/eomt/modeling_eomt.py#L1084)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [EomtImageProcessor](/docs/transformers/v5.15.1/en/model_doc/eomt#transformers.EomtImageProcessor). See `EomtImageProcessor.__call__()` for details (`processor_class` uses [EomtImageProcessor](/docs/transformers/v5.15.1/en/model_doc/eomt#transformers.EomtImageProcessor) for processing images).

mask_labels (`list[torch.Tensor]`, *optional*) : list of mask labels of shape `(num_labels, height, width)` to be fed to a model

class_labels (`list[torch.LongTensor]`, *optional*) : list of target class labels of shape `(num_labels, height, width)` to be fed to a model. They identify the labels of `mask_labels`, e.g. the label of `mask_labels[i][j]` if `class_labels[i][j]`.

patch_offsets (`list[torch.Tensor]`, *optional*) : list of tuples indicating the image index and start and end positions of patches for semantic segmentation.

**Returns:** `EomtForUniversalSegmentationOutput` or `tuple(torch.FloatTensor)`

A `EomtForUniversalSegmentationOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([EomtConfig](/docs/transformers/v5.15.1/en/model_doc/eomt#transformers.EomtConfig)) and inputs.

The [EomtForUniversalSegmentation](/docs/transformers/v5.15.1/en/model_doc/eomt#transformers.EomtForUniversalSegmentation) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.Tensor`, *optional*) -- The computed loss, returned when labels are present.
- **class_queries_logits** (`torch.FloatTensor`, *optional*) -- A tensor of shape `(batch_size, num_queries, num_labels + 1)` representing the proposed classes for each
  query. Note the `+ 1` is needed because we incorporate the null class.
- **masks_queries_logits** (`torch.FloatTensor`, *optional*) -- A tensor of shape `(batch_size, num_queries, height, width)` representing the proposed masks for each
  query.
- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Last hidden states (final feature map) of the last layer.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, sequence_length, hidden_size)`. Hidden-states all layers of the model.
- **attentions** (`tuple(tuple(torch.FloatTensor))`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `tuple(torch.FloatTensor)` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`. Self and Cross Attentions weights from transformer decoder.
- **patch_offsets** (`list[torch.Tensor]`, *optional*) -- list of tuples indicating the image index and start and end positions of patches for semantic segmentation.
