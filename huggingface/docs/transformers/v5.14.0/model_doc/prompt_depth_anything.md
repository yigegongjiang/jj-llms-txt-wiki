# Prompt Depth Anything

## Overview

The Prompt Depth Anything model was introduced in [Prompting Depth Anything for 4K Resolution Accurate Metric Depth Estimation](https://huggingface.co/papers/2412.14015) by Haotong Lin, Sida Peng, Jingxiao Chen, Songyou Peng, Jiaming Sun, Minghuan Liu, Hujun Bao, Jiashi Feng, Xiaowei Zhou, Bingyi Kang.

The abstract from the paper is as follows:

*Prompts play a critical role in unleashing the power of language and vision foundation models for specific tasks. For the first time, we introduce prompting into depth foundation models, creating a new paradigm for metric depth estimation termed Prompt Depth Anything. Specifically, we use a low-cost LiDAR as the prompt to guide the Depth Anything model for accurate metric depth output, achieving up to 4K resolution. Our approach centers on a concise prompt fusion design that integrates the LiDAR at multiple scales within the depth decoder. To address training challenges posed by limited datasets containing both LiDAR depth and precise GT depth, we propose a scalable data pipeline that includes synthetic data LiDAR simulation and real data pseudo GT depth generation. Our approach sets new state-of-the-arts on the ARKitScenes and ScanNet++ datasets and benefits downstream applications, including 3D reconstruction and generalized robotic grasping.*

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/prompt_depth_anything_architecture.jpg"
alt="drawing" width="600"/>

 Prompt Depth Anything overview. Taken from the original paper.

## Usage example

The Transformers library allows you to use the model with just a few lines of code:

```python
import requests
import torch
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForDepthEstimation

url = "https://github.com/DepthAnything/PromptDA/blob/main/assets/example_images/image.jpg?raw=true"
image = Image.open(requests.get(url, stream=True).raw)

image_processor = AutoImageProcessor.from_pretrained("depth-anything/prompt-depth-anything-vits-hf")
model = AutoModelForDepthEstimation.from_pretrained("depth-anything/prompt-depth-anything-vits-hf", device_map="auto")

prompt_depth_url = "https://github.com/DepthAnything/PromptDA/blob/main/assets/example_images/arkit_depth.png?raw=true"
prompt_depth = Image.open(requests.get(prompt_depth_url, stream=True).raw)
# the prompt depth can be None, and the model will output a monocular relative depth.

# prepare image for the model
inputs = image_processor(images=image, return_tensors="pt", prompt_depth=prompt_depth).to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

# interpolate to original size
post_processed_output = image_processor.post_process_depth_estimation(
    outputs,
    target_sizes=[(image.height, image.width)],
)

# visualize the prediction
predicted_depth = post_processed_output[0]["predicted_depth"]
depth = predicted_depth * 1000
depth = depth.detach().cpu().numpy()
depth = Image.fromarray(depth.astype("uint16")) # mm
```

## Resources

A list of official Hugging Face and community (indicated by 🌎) resources to help you get started with Prompt Depth Anything.

- [Prompt Depth Anything Demo](https://huggingface.co/spaces/depth-anything/PromptDA)
- [Prompt Depth Anything Interactive Results](https://promptda.github.io/interactive.html)

If you are interested in submitting a resource to be included here, please feel free to open a Pull Request and we'll review it! The resource should ideally demonstrate something new instead of duplicating an existing resource.

## PromptDepthAnythingConfig[[transformers.PromptDepthAnythingConfig]]

- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `14`) --
  The size (resolution) of each patch.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **reassemble_hidden_size** (`int`, *optional*, defaults to 384) --
  The number of input channels of the reassemble layers.
- **reassemble_factors** (`list[int]`, *optional*, defaults to `[4, 2, 1, 0.5]`) --
  The up/downsampling factors of the reassemble layers.
- **neck_hidden_sizes** (`list[str]`, *optional*, defaults to `[48, 96, 192, 384]`) --
  The hidden sizes to project to for the feature maps of the backbone.
- **fusion_hidden_size** (`int`, *optional*, defaults to 64) --
  The number of channels before fusion.
- **head_in_index** (`int`, *optional*, defaults to -1) --
  The index of the features to use in the depth estimation head.
- **head_hidden_size** (`int`, *optional*, defaults to 32) --
  The number of output channels in the second convolution of the depth estimation head.
- **depth_estimation_type** (`str`, *optional*, defaults to `"relative"`) --
  The type of depth estimation to use. Can be one of `["relative", "metric"]`.
- **max_depth** (`float`, *optional*) --
  The maximum depth to use for the "metric" depth estimation head. 20 should be used for indoor models
  and 80 for outdoor models. For "relative" depth estimation, this value is ignored.

This is the configuration class to store the configuration of a Prompt Depth AnythingModel. It is used to instantiate a Prompt Depth Anything
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [LiheYoung/depth-anything-small-hf](https://huggingface.co/LiheYoung/depth-anything-small-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import PromptDepthAnythingConfig, PromptDepthAnythingForDepthEstimation

>>> # Initializing a PromptDepthAnything small style configuration
>>> configuration = PromptDepthAnythingConfig()

>>> # Initializing a model from the PromptDepthAnything small style configuration
>>> model = PromptDepthAnythingForDepthEstimation(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## PromptDepthAnythingForDepthEstimation[[transformers.PromptDepthAnythingForDepthEstimation]]

- **config** ([PromptDepthAnythingForDepthEstimation](/docs/transformers/v5.14.0/en/model_doc/prompt_depth_anything#transformers.PromptDepthAnythingForDepthEstimation)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Prompt Depth Anything Model with a depth estimation head on top (consisting of 3 convolutional layers) e.g. for KITTI, NYUv2.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [PromptDepthAnythingImageProcessor](/docs/transformers/v5.14.0/en/model_doc/prompt_depth_anything#transformers.PromptDepthAnythingImageProcessor). See `PromptDepthAnythingImageProcessor.__call__()` for details (`processor_class` uses
  [PromptDepthAnythingImageProcessor](/docs/transformers/v5.14.0/en/model_doc/prompt_depth_anything#transformers.PromptDepthAnythingImageProcessor) for processing images).
- **prompt_depth** (`torch.FloatTensor` of shape `(batch_size, 1, height, width)`, *optional*) --
  Prompt depth is the sparse or low-resolution depth obtained from multi-view geometry or a
  low-resolution depth sensor. It generally has shape (height, width), where height
  and width can be smaller than those of the images. It is optional and can be None, which means no prompt depth
  will be used. If it is None, the output will be a monocular relative depth.
  The values are recommended to be in meters, but this is not necessary.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.[DepthEstimatorOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.DepthEstimatorOutput) or `tuple(torch.FloatTensor)`A [DepthEstimatorOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.DepthEstimatorOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PromptDepthAnythingConfig](/docs/transformers/v5.14.0/en/model_doc/prompt_depth_anything#transformers.PromptDepthAnythingConfig)) and inputs.
The [PromptDepthAnythingForDepthEstimation](/docs/transformers/v5.14.0/en/model_doc/prompt_depth_anything#transformers.PromptDepthAnythingForDepthEstimation) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoImageProcessor, AutoModelForDepthEstimation
>>> import torch
>>> import numpy as np
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "https://github.com/DepthAnything/PromptDA/blob/main/assets/example_images/image.jpg?raw=true"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("depth-anything/prompt-depth-anything-vits-hf")
>>> model = AutoModelForDepthEstimation.from_pretrained("depth-anything/prompt-depth-anything-vits-hf")

>>> prompt_depth_url = "https://github.com/DepthAnything/PromptDA/blob/main/assets/example_images/arkit_depth.png?raw=true"
>>> with httpx.stream("GET", prompt_depth_url) as response:
...     prompt_depth = Image.open(BytesIO(response.read()))

>>> # prepare image for the model
>>> inputs = image_processor(images=image, return_tensors="pt", prompt_depth=prompt_depth)

>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> # interpolate to original size
>>> post_processed_output = image_processor.post_process_depth_estimation(
...     outputs,
...     target_sizes=[(image.height, image.width)],
... )

>>> # visualize the prediction
>>> predicted_depth = post_processed_output[0]["predicted_depth"]
>>> depth = predicted_depth * 1000.
>>> depth = depth.detach().cpu().numpy()
>>> depth = Image.fromarray(depth.astype("uint16")) # mm
```

## PromptDepthAnythingImageProcessor[[transformers.PromptDepthAnythingImageProcessor]]

- **keep_aspect_ratio** (`bool`, *kwargs*, *optional*) --
  If `True`, the image is resized to the largest possible size such that the aspect ratio is preserved.
- **ensure_multiple_of** (`int`, *kwargs*, *optional*) --
  If `do_resize` is `True`, the image is resized to a size that is a multiple of this value.
- **size_divisor** (`int`, *kwargs*, *optional*) --
  If `do_pad` is `True`, pads the image dimensions to be divisible by this value.
- **prompt_scale_to_meter** (`float`, *kwargs*, *optional*) --
  Scale factor to convert the prompt depth to meters.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a PromptDepthAnythingImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **prompt_depth** (`ImageInput`, *optional*) --
  Prompt depth to preprocess.
- **keep_aspect_ratio** (`bool`, *kwargs*, *optional*) --
  If `True`, the image is resized to the largest possible size such that the aspect ratio is preserved.
- **ensure_multiple_of** (`int`, *kwargs*, *optional*) --
  If `do_resize` is `True`, the image is resized to a size that is a multiple of this value.
- **size_divisor** (`int`, *kwargs*, *optional*) --
  If `do_pad` is `True`, pads the image dimensions to be divisible by this value.
- **prompt_scale_to_meter** (`float`, *kwargs*, *optional*) --
  Scale factor to convert the prompt depth to meters.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** (`DepthEstimatorOutput`) --
  Raw outputs of the model.
- **target_sizes** (`TensorType` or `list[tuple[int, int]]`, *optional*) --
  Tensor of shape `(batch_size, 2)` or list of tuples (`tuple[int, int]`) containing the target size
  (height, width) of each image in the batch. If left to None, predictions will not be resized.`list[dict[str, TensorType]]`A list of dictionaries of tensors representing the processed depth
predictions.

Converts the raw output of `DepthEstimatorOutput` into final depth predictions and depth PIL images.
Only supports PyTorch.

## PromptDepthAnythingImageProcessorPil[[transformers.PromptDepthAnythingImageProcessorPil]]

- **keep_aspect_ratio** (`bool`, *kwargs*, *optional*) --
  If `True`, the image is resized to the largest possible size such that the aspect ratio is preserved.
- **ensure_multiple_of** (`int`, *kwargs*, *optional*) --
  If `do_resize` is `True`, the image is resized to a size that is a multiple of this value.
- **size_divisor** (`int`, *kwargs*, *optional*) --
  If `do_pad` is `True`, pads the image dimensions to be divisible by this value.
- **prompt_scale_to_meter** (`float`, *kwargs*, *optional*) --
  Scale factor to convert the prompt depth to meters.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a PromptDepthAnythingImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **prompt_depth** (`ImageInput`, *optional*) --
  Prompt depth to preprocess.
- **keep_aspect_ratio** (`bool`, *kwargs*, *optional*) --
  If `True`, the image is resized to the largest possible size such that the aspect ratio is preserved.
- **ensure_multiple_of** (`int`, *kwargs*, *optional*) --
  If `do_resize` is `True`, the image is resized to a size that is a multiple of this value.
- **size_divisor** (`int`, *kwargs*, *optional*) --
  If `do_pad` is `True`, pads the image dimensions to be divisible by this value.
- **prompt_scale_to_meter** (`float`, *kwargs*, *optional*) --
  Scale factor to convert the prompt depth to meters.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** (`DepthEstimatorOutput`) --
  Raw outputs of the model.
- **target_sizes** (`TensorType` or `list[tuple[int, int]]`, *optional*) --
  Tensor of shape `(batch_size, 2)` or list of tuples (`tuple[int, int]`) containing the target size
  (height, width) of each image in the batch. If left to None, predictions will not be resized.`list[dict[str, TensorType]]`A list of dictionaries of tensors representing the processed depth
predictions.

Converts the raw output of `DepthEstimatorOutput` into final depth predictions and depth PIL images.
Only supports PyTorch.
