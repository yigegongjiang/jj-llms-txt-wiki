# Image Processor

An image processor is in charge of loading images (optionally), preparing input features for vision models and post processing their outputs. This includes transformations such as resizing, normalization, and conversion to PyTorch and Numpy tensors. It may also include model specific post-processing such as converting logits to segmentation masks.

Image processors use a backend-based architecture. The class hierarchy is:

- [BaseImageProcessor](/docs/transformers/v5.15.1/en/main_classes/image_processor#transformers.BaseImageProcessor) — abstract base class (for backward compatibility only; do not instantiate directly)
  - [TorchvisionBackend](/docs/transformers/v5.15.1/en/main_classes/image_processor#transformers.TorchvisionBackend) — the **default** [torchvision-backed](https://pytorch.org/vision/stable/index.html) backend. GPU-accelerated and significantly faster than the PIL backend. All models expose a `<Model>ImageProcessor` class that inherits from it.
  - [PilBackend](/docs/transformers/v5.15.1/en/main_classes/image_processor#transformers.PilBackend) — the PIL/NumPy alternative backend. Portable, CPU-only. Only available for older models via a `<Model>ImageProcessorPil` class; useful when exact numerical parity with the original implementation is required.

Both backends expose the same API. Use the `backend` attribute to inspect which backend a loaded processor uses (e.g. `processor.backend == "torchvision"`).

Pass `backend` to [AutoImageProcessor.from_pretrained()](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoImageProcessor.from_pretrained) to select a backend. When `backend` is omitted (the default), torchvision is picked if it's installed, otherwise PIL is used. A few models that use Lanczos interpolation (Chameleon, Flava, Idefics3, SmolVLM) are an exception, and they default to PIL when torchvision < 0.27. Forcing `backend="torchvision"` for these models on torchvision < 0.27 falls back to BICUBIC interpolation, since torchvision supports Lanczos for tensors only from 0.27 onward.

```python
from transformers import AutoImageProcessor

# Default: picks torchvision if available, otherwise pil
processor = AutoImageProcessor.from_pretrained("facebook/detr-resnet-50")

# Explicitly request torchvision
processor = AutoImageProcessor.from_pretrained("facebook/detr-resnet-50", backend="torchvision")

# Explicitly request PIL
processor = AutoImageProcessor.from_pretrained("facebook/detr-resnet-50", backend="pil")
```

When using the torchvision backend, you can set the `device` argument to specify the device on which the processing should be done. By default, the processing is done on the same device as the inputs if the inputs are tensors, or on the CPU otherwise.

```python
import torch
from torchvision.io import read_image
from transformers import DetrImageProcessor

device = torch.accelerator.current_accelerator().type if torch.accelerator.is_available() else "cpu"

images = read_image("image.jpg")
processor = DetrImageProcessor.from_pretrained("facebook/detr-resnet-50")
images_processed = processor(images, return_tensors="pt", device=device)
```

Here are some speed comparisons between the torchvision and PIL backends for the `DETR` and `RT-DETR` models, and how they impact overall inference time:

  

  

  

  

These benchmarks were run on an [AWS EC2 g5.2xlarge instance](https://aws.amazon.com/ec2/instance-types/g5/), utilizing an NVIDIA A10G Tensor Core GPU.

## ImageProcessingMixin[[transformers.ImageProcessingMixin]]

#### transformers.ImageProcessingMixin[[transformers.ImageProcessingMixin]]

```python
transformers.ImageProcessingMixin(**kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_base.py#L61)

This is an image processor mixin used to provide saving/loading functionality for sequential and image feature
extractors.

#### from_pretrained[[transformers.ImageProcessingMixin.from_pretrained]]

```python
from_pretrained(pretrained_model_name_or_path: str | os.PathLike, cache_dir: str | os.PathLike | None = None, force_download: bool = False, local_files_only: bool = False, token: str | bool | None = None, revision: str = 'main', **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_base.py#L84)

**Parameters:**

pretrained_model_name_or_path (`str` or `os.PathLike`) : This can be either:  - a string, the *model id* of a pretrained image_processor hosted inside a model repo on huggingface.co. - a path to a *directory* containing a image processor file saved using the [save_pretrained()](/docs/transformers/v5.15.1/en/main_classes/image_processor#transformers.ImageProcessingMixin.save_pretrained) method, e.g., `./my_model_directory/`. - a path to a saved image processor JSON *file*, e.g., `./my_model_directory/preprocessor_config.json`.

cache_dir (`str` or `os.PathLike`, *optional*) : Path to a directory in which a downloaded pretrained model image processor should be cached if the standard cache should not be used.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force to (re-)download the image processor files and override the cached versions if they exist.

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, e.g., `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}.` The proxies are used on each request.

token (`str` or `bool`, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, or not specified, will use the token generated when running `hf auth login` (stored in `~/.huggingface`).

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, or a commit id, since we use a git-based system for storing models and other artifacts on huggingface.co, so `revision` can be any identifier allowed by git.     To test a pull request you made on the Hub, you can pass `revision="refs/pr/<pr_number>"`.   

return_unused_kwargs (`bool`, *optional*, defaults to `False`) : If `False`, then this function returns just the final image processor object. If `True`, then this functions returns a `Tuple(image_processor, unused_kwargs)` where *unused_kwargs* is a dictionary consisting of the key/value pairs whose keys are not image processor attributes: i.e., the part of `kwargs` which has not been used to update `image_processor` and is otherwise ignored.

subfolder (`str`, *optional*, defaults to `""`) : In case the relevant files are located inside a subfolder of the model repo on huggingface.co, you can specify the folder name here.

kwargs (`dict[str, Any]`, *optional*) : The values in kwargs of any keys which are image processor attributes will be used to override the loaded values. Behavior concerning key/value pairs whose keys are *not* image processor attributes is controlled by the `return_unused_kwargs` keyword parameter.

**Returns:**

A image processor of type [ImageProcessingMixin](/docs/transformers/v5.15.1/en/main_classes/image_processor#transformers.ImageProcessingMixin).

Instantiate a type of [ImageProcessingMixin](/docs/transformers/v5.15.1/en/main_classes/image_processor#transformers.ImageProcessingMixin) from an image processor.

Examples:

```python
# We can't instantiate directly the base class *ImageProcessingMixin* so let's show the examples on a
# derived class: *CLIPImageProcessor*
image_processor = CLIPImageProcessor.from_pretrained(
    "openai/clip-vit-base-patch32"
)  # Download image_processing_config from huggingface.co and cache.
image_processor = CLIPImageProcessor.from_pretrained(
    "./test/saved_model/"
)  # E.g. image processor (or model) was saved using *save_pretrained('./test/saved_model/')*
image_processor = CLIPImageProcessor.from_pretrained("./test/saved_model/preprocessor_config.json")
image_processor = CLIPImageProcessor.from_pretrained(
    "openai/clip-vit-base-patch32", do_normalize=False, foo=False
)
assert image_processor.do_normalize is False
image_processor, unused_kwargs = CLIPImageProcessor.from_pretrained(
    "openai/clip-vit-base-patch32", do_normalize=False, foo=False, return_unused_kwargs=True
)
assert image_processor.do_normalize is False
assert unused_kwargs == {"foo": False}
```

#### save_pretrained[[transformers.ImageProcessingMixin.save_pretrained]]

```python
save_pretrained(save_directory: str | os.PathLike, push_to_hub: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_base.py#L183)

**Parameters:**

save_directory (`str` or `os.PathLike`) : Directory where the image processor JSON file will be saved (will be created if it does not exist).

push_to_hub (`bool`, *optional*, defaults to `False`) : Whether or not to push your model to the Hugging Face model hub after saving it. You can specify the repository you want to push to with `repo_id` (will default to the name of `save_directory` in your namespace).

kwargs (`dict[str, Any]`, *optional*) : Additional key word arguments passed along to the [push_to_hub()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.utils.PushToHubMixin.push_to_hub) method.

Save an image processor object to the directory `save_directory`, so that it can be re-loaded using the
[from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/image_processor#transformers.ImageProcessingMixin.from_pretrained) class method.

## BatchFeature[[transformers.BatchFeature]]

#### transformers.BatchFeature[[transformers.BatchFeature]]

```python
transformers.BatchFeature(data: dict[str, typing.Any] | None = None, tensor_type: None | str | transformers.utils.generic.TensorType = None, skip_tensor_conversion: list[str] | set[str] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/feature_extraction_utils.py#L58)

**Parameters:**

data (`dict`, *optional*) : Dictionary of lists/arrays/tensors returned by the __call__/pad methods ('input_values', 'attention_mask', etc.).

tensor_type (`Union[None, str, TensorType]`, *optional*) : You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at initialization.

skip_tensor_conversion (`list[str]` or `set[str]`, *optional*) : List or set of keys that should NOT be converted to tensors, even when `tensor_type` is specified.

Holds the output of the [pad()](/docs/transformers/v5.15.1/en/main_classes/feature_extractor#transformers.SequenceFeatureExtractor.pad) and feature extractor specific `__call__` methods.

This class is derived from a python dictionary and can be used as a dictionary.

#### convert_to_tensors[[transformers.BatchFeature.convert_to_tensors]]

```python
convert_to_tensors(tensor_type: str | transformers.utils.generic.TensorType | None = None, skip_tensor_conversion: list[str] | set[str] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/feature_extraction_utils.py#L158)

**Parameters:**

tensor_type (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : The type of tensors to use. If `str`, should be one of the values of the enum [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType). If `None`, no modification is done.

skip_tensor_conversion (`list[str]` or `set[str]`, *optional*) : List or set of keys that should NOT be converted to tensors, even when `tensor_type` is specified.

Convert the inner content to tensors.

Note:
Values that don't have an array-like structure (e.g., strings, dicts, lists of strings) are
automatically skipped and won't be converted to tensors. Ragged arrays (lists of arrays with
different lengths) are still attempted, though they may raise errors during conversion.

#### to[[transformers.BatchFeature.to]]

```python
to(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/feature_extraction_utils.py#L215)

**Parameters:**

args (`Tuple`) : Will be passed to the `to(...)` function of the tensors.

kwargs (`Dict`, *optional*) : Will be passed to the `to(...)` function of the tensors. To enable asynchronous data transfer, set the `non_blocking` flag in `kwargs` (defaults to `False`).

**Returns:** [BatchFeature](/docs/transformers/v5.15.1/en/main_classes/image_processor#transformers.BatchFeature)

The same instance after modification.

Send all values to device by calling `v.to(*args, **kwargs)` (PyTorch only). This should support casting in
different `dtypes` and sending the `BatchFeature` to a different `device`.

## BaseImageProcessor[[transformers.BaseImageProcessor]]

#### transformers.BaseImageProcessor[[transformers.BaseImageProcessor]]

```python
transformers.BaseImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_utils.py#L60)

Base class for image processors with an inheritance-based backend architecture.

This class defines the preprocessing pipeline: kwargs validation, input preparation, and dispatching to the
backend's `_preprocess` method. Backend subclasses (`TorchvisionBackend`, `PilBackend`) inherit from this class
and implement the actual image operations (resize, crop, rescale, normalize, etc.). Model-specific image
processors then inherit from the appropriate backend class.

Architecture Overview
---------------------

The class hierarchy is:

BaseImageProcessor (this class)
├── TorchvisionBackend    (GPU-accelerated, torch.Tensor)
│   └── ModelImageProcessor (e.g. LlavaNextImageProcessor)
└── PilBackend            (portable CPU, np.ndarray)
└── ModelImageProcessorPil (e.g. CLIPImageProcessorPil)

The preprocessing flow is:

__call__() → preprocess() → _preprocess_image_like_inputs() → _prepare_image_like_inputs()
(calls process_image per image)
→ _preprocess()
(batch operations: resize, crop, etc.)

- `process_image`: Implemented by backends. Converts a single raw input (PIL, NumPy, or Tensor) to the
  backend's working format (torch.Tensor or np.ndarray), handles RGB conversion and channel reordering.
- `_preprocess`: Implemented by backends. Performs the actual batch processing (resize, center crop, rescale,
  normalize, pad) and returns a `BatchFeature`.

Basic Implementation
--------------------

For processors that only need standard operations (resize, center crop, rescale, normalize), inherit from
a backend and define class attributes:

from transformers.image_processing_backends import PilBackend

class MyImageProcessorPil(PilBackend):
resample = PILImageResampling.BILINEAR
image_mean = IMAGENET_DEFAULT_MEAN
image_std = IMAGENET_DEFAULT_STD
size = {"height": 224, "width": 224}
do_resize = True
do_rescale = True
do_normalize = True

The backend's `_preprocess` method handles the standard pipeline automatically.

Custom Processing
-----------------

For processors that need custom logic (e.g., patch-based processing, multiple input types), override
`_preprocess` in your model-specific processor. The `_preprocess` method receives already-prepared images
(converted to the backend format with channels-first ordering) and performs the actual processing:

class MyImageProcessor(TorchvisionBackend):
def _preprocess(self, images, do_resize, size, do_normalize, image_mean, image_std, **kwargs):
# Group images by shape for efficient batched operations
grouped_images, grouped_images_index = group_images_by_shape(images)
processed_groups = {}
for shape, stacked_images in grouped_images.items():
if do_resize:
stacked_images = self.resize(stacked_images, size=size)
if do_normalize:
stacked_images = self.normalize(stacked_images, mean=image_mean, std=image_std)
processed_groups[shape] = stacked_images
processed_images = reorder_images(processed_groups, grouped_images_index)
return BatchFeature(data={"pixel_values": processed_images})

For processors handling multiple input types (e.g., images + segmentation maps), override
`_preprocess_image_like_inputs`:

def _preprocess_image_like_inputs(
self,
images: ImageInput,
segmentation_maps: ImageInput | None = None,
**kwargs,
) -> BatchFeature:
images = self._prepare_image_like_inputs(images, **kwargs)
batch_feature = self._preprocess(images, **kwargs)

if segmentation_maps is not None:
maps = self._prepare_image_like_inputs(segmentation_maps, **kwargs)
batch_feature["labels"] = self._preprocess(maps, **kwargs).pixel_values

return batch_feature

Extending Backend Behavior
--------------------------

To customize operations for a specific backend, subclass the backend and override its methods:

from transformers.image_processing_backends import TorchvisionBackend, PilBackend

class MyTorchvisionProcessor(TorchvisionBackend):
def resize(self, image, size, **kwargs):
# Custom resize logic for torchvision
return super().resize(image, size, **kwargs)

class MyPilProcessor(PilBackend):
def resize(self, image, size, **kwargs):
# Custom resize logic for PIL
return super().resize(image, size, **kwargs)

Custom Parameters
-----------------

To add parameters beyond `ImagesKwargs`, create a custom kwargs class and set it as `valid_kwargs`:

class MyImageProcessorKwargs(ImagesKwargs):
custom_param: int | None = None

class MyImageProcessor(TorchvisionBackend):
valid_kwargs = MyImageProcessorKwargs
custom_param = 10  # default value

Key Notes
---------

- Backend selection is done at the class level: inherit from `TorchvisionBackend` or `PilBackend`
- Backends receive images as `torch.Tensor` (Torchvision) or `np.ndarray` (PIL), always channels-first
- All images have channel dimension first during processing, regardless of backend
- Arguments not provided by users default to class attribute values
- Backend classes encapsulate backend-specific logic (resize, normalize, etc.) and can be overridden

#### center_crop[[transformers.BaseImageProcessor.center_crop]]

```python
center_crop(image: ndarray, size: dict, data_format: str | transformers.image_utils.ChannelDimension | None = None, input_data_format: str | transformers.image_utils.ChannelDimension | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_utils.py#L492)

**Parameters:**

image (`np.ndarray`) : Image to center crop.

size (`dict[str, int]`) : Size of the output image.

data_format (`str` or `ChannelDimension`, *optional*) : The channel dimension format for the output image. If unset, the channel dimension format of the input image is used. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format.

input_data_format (`ChannelDimension` or `str`, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format.

Center crop an image to `(size["height"], size["width"])`. If the input size is smaller than `crop_size` along
any edge, the image is padded with 0's and then center cropped.

#### normalize[[transformers.BaseImageProcessor.normalize]]

```python
normalize(image: ndarray, mean: float | collections.abc.Iterable[float], std: float | collections.abc.Iterable[float], data_format: str | transformers.image_utils.ChannelDimension | None = None, input_data_format: str | transformers.image_utils.ChannelDimension | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_utils.py#L455)

**Parameters:**

image (`np.ndarray`) : Image to normalize.

mean (`float` or `Iterable[float]`) : Image mean to use for normalization.

std (`float` or `Iterable[float]`) : Image standard deviation to use for normalization.

data_format (`str` or `ChannelDimension`, *optional*) : The channel dimension format for the output image. If unset, the channel dimension format of the input image is used. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format.

input_data_format (`ChannelDimension` or `str`, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format.

**Returns:** `np.ndarray`

The normalized image.

Normalize an image. image = (image - image_mean) / image_std.

#### preprocess[[transformers.BaseImageProcessor.preprocess]]

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

#### process_image[[transformers.BaseImageProcessor.process_image]]

```python
process_image(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_utils.py#L219)

Process a single raw image into the backend's working format.

Implemented by backend subclasses (`TorchvisionBackend`, `PilBackend`). Converts a raw input
(PIL Image, NumPy array, or torch Tensor) to the backend's internal format (`torch.Tensor` for
Torchvision, `np.ndarray` for PIL), handles RGB conversion and ensures channels-first ordering.

#### rescale[[transformers.BaseImageProcessor.rescale]]

```python
rescale(image: ndarray, scale: float, data_format: str | transformers.image_utils.ChannelDimension | None = None, input_data_format: str | transformers.image_utils.ChannelDimension | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_utils.py#L422)

**Parameters:**

image (`np.ndarray`) : Image to rescale.

scale (`float`) : The scaling factor to rescale pixel values by.

data_format (`str` or `ChannelDimension`, *optional*) : The channel dimension format for the output image. If unset, the channel dimension format of the input image is used. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format.

input_data_format (`ChannelDimension` or `str`, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format.

**Returns:** `np.ndarray`

The rescaled image.

Rescale an image by a scale factor. image = image * scale.

## TorchvisionBackend[[transformers.TorchvisionBackend]]

#### transformers.TorchvisionBackend[[transformers.TorchvisionBackend]]

```python
transformers.TorchvisionBackend(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_backends.py#L86)

Torchvision backend for GPU-accelerated batched image processing.

#### center_crop[[transformers.TorchvisionBackend.center_crop]]

```python
center_crop(image: torch.Tensor, size: SizeDict, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_backends.py#L340)

Center crop an image using Torchvision.

#### convert_to_rgb[[transformers.TorchvisionBackend.convert_to_rgb]]

```python
convert_to_rgb(image: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']])
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_backends.py#L151)

Convert an image to RGB format.

#### fetch_images[[transformers.TorchvisionBackend.fetch_images]]

```python
fetch_images(image_url_or_urls: str | list[str] | list[list[str]])
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_backends.py#L100)

Convert a single or a list of URLs / paths into `torch.Tensor` objects.

Already-valid image objects (tensors, numpy arrays, PIL Images) are passed through
unchanged so that callers who pre-load images are unaffected.

#### normalize[[transformers.TorchvisionBackend.normalize]]

```python
normalize(image: torch.Tensor, mean: float | collections.abc.Iterable[float], std: float | collections.abc.Iterable[float], **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_backends.py#L288)

Normalize an image using Torchvision.

#### pad[[transformers.TorchvisionBackend.pad]]

```python
pad(images: list, pad_size: SizeDict = None, fill_value: int | None = 0, padding_mode: str | None = 'constant', return_mask: bool = False, disable_grouping: bool | None = False, is_nested: bool | None = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_backends.py#L155)

Pad images using Torchvision with batched operations.

#### process_image[[transformers.TorchvisionBackend.process_image]]

```python
process_image(image: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], do_convert_rgb: bool | None = None, input_data_format: str | transformers.image_utils.ChannelDimension | None = None, device: typing.Optional[ForwardRef('torch.device')] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_backends.py#L116)

Process a single image for torchvision backend.

#### rescale[[transformers.TorchvisionBackend.rescale]]

```python
rescale(image: torch.Tensor, scale: float, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_backends.py#L279)

Rescale an image by a scale factor using Torchvision.

#### rescale_and_normalize[[transformers.TorchvisionBackend.rescale_and_normalize]]

```python
rescale_and_normalize(images: torch.Tensor, do_rescale: bool, rescale_factor: float, do_normalize: bool, image_mean: float | list[float], image_std: float | list[float])
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_backends.py#L315)

Rescale and normalize images using Torchvision (fused for efficiency).

#### resize[[transformers.TorchvisionBackend.resize]]

```python
resize(image: torch.Tensor, size: SizeDict, resample: PILImageResampling | tvF.InterpolationMode | int | None = None, antialias: bool = True, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_backends.py#L205)

Resize an image using Torchvision.

## PilBackend[[transformers.PilBackend]]

#### transformers.PilBackend[[transformers.PilBackend]]

```python
transformers.PilBackend(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_backends.py#L417)

PIL/NumPy backend for portable CPU-only image processing.

#### center_crop[[transformers.PilBackend.center_crop]]

```python
center_crop(image: ndarray, size: SizeDict, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_backends.py#L603)

Center crop an image using NumPy.

#### convert_to_rgb[[transformers.PilBackend.convert_to_rgb]]

```python
convert_to_rgb(image: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']])
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_backends.py#L467)

Convert an image to RGB format.

#### normalize[[transformers.PilBackend.normalize]]

```python
normalize(image: ndarray, mean: float | collections.abc.Iterable[float], std: float | collections.abc.Iterable[float], **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_backends.py#L587)

Normalize an image using NumPy.

#### pad[[transformers.PilBackend.pad]]

```python
pad(images: list, pad_size: SizeDict = None, fill_value: int | None = 0, padding_mode: str | None = 'constant', return_mask: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_backends.py#L471)

Pad images to specified size using NumPy.

#### process_image[[transformers.PilBackend.process_image]]

```python
process_image(image: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], do_convert_rgb: bool | None = None, input_data_format: str | transformers.image_utils.ChannelDimension | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_backends.py#L431)

Process a single image for PIL backend.

#### rescale[[transformers.PilBackend.rescale]]

```python
rescale(image: ndarray, scale: float, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_backends.py#L573)

Rescale an image by a scale factor using NumPy.

#### resize[[transformers.PilBackend.resize]]

```python
resize(image: ndarray, size: SizeDict, resample: PILImageResampling | None = None, reducing_gap: int | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_backends.py#L522)

Resize an image using PIL/NumPy.
