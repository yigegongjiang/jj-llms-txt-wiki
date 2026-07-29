# TIPSv2 DPT

    
        
        
    

## Overview

[TIPSv2](./tipsv2) with dense prediction transformer head (DPT), see the [TIPSv2](./tipsv2) documentation for more information.
<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/tipsv2_architecture.jpg"
alt="tipsv2 architecture overview" width="600"/>

This model was contributed by [guarin](https://huggingface.co/guarin).
The original code can be found [here](https://github.com/google-deepmind/tips).

You can find all the original TIPSv2 DPT checkpoints under the [TIPSv2](https://huggingface.co/collections/google/tipsv2) collection.

> [!TIP]
> See [TIPSv2](./tipsv2) for the TIPSv2 vision and language backbones as well as zero shot image classification.

```python
from transformers import pipeline

pipe = pipeline(task="depth-estimation", model="google/tipsv2-b14-dpt", device_map="auto")
out = pipe("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/room.jpg")
out["depth"]  # PIL Image normalized to [min, max] of predicted_depth

# Visualization
import matplotlib.pyplot as plt
from transformers.image_utils import load_image

image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/room.jpg")

figure, axes = plt.subplots(1, 2, figsize=(10, 5))
axes[0].imshow(image)
axes[1].imshow(out["depth"])
axes[1].set_title("Depth")
for axis in axes:
    axis.axis("off")
plt.show()
```

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/tipsv2_depth_estimation.jpg"
alt="tipsv2 depth estimation pipeline" width="600"/>

```python
from transformers import pipeline

pipe = pipeline(task="image-segmentation", model="google/tipsv2-b14-dpt", device_map="auto")
out = pipe("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/room.jpg")
out[0]["label"] # string class label
out[0]["mask"]  # PIL Image with binary mask set to class id

# Visualization
import matplotlib.pyplot as plt
from transformers.image_utils import load_image

image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/room.jpg")

figure, axes = plt.subplots(1, 2, figsize=(10, 5))
axes[0].imshow(image)
axes[1].imshow(out[0]["mask"], cmap="gray")
axes[1].set_title(out[0]["label"])
for axis in axes:
    axis.axis("off")
plt.show()
```

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/tipsv2_pipeline_segmentation.jpg"
alt="tipsv2 image segmentation pipeline" width="600"/>

Use [Tipsv2DptForDensePrediction](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptForDensePrediction) to run all three tasks (depth, normals, and segmentation) in a single forward pass over a shared backbone.

```python
import torch
from transformers import Tipsv2DptForDensePrediction, AutoImageProcessor
from transformers.image_utils import load_image

model_id = "google/tipsv2-b14-dpt"
model = Tipsv2DptForDensePrediction.from_pretrained(model_id, device_map="auto")
image_processor = AutoImageProcessor.from_pretrained(model_id)

image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/room.jpg")
inputs = image_processor(images=image, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

outputs.predicted_depth # (batch_size, height, width) tensor with predicted depth in meters
outputs.normals # (batch_size, 3, height, width) tensor with normals in XYZ format (unnormalized)
outputs.segmentation_logits # # (batch_size, config.num_labels, height, width) tensor with segmentation logits

depth_results = image_processor.post_process_depth_estimation(outputs, target_sizes=[(image.height, image.width)])
normal_results = image_processor.post_process_normal_estimation(outputs, target_sizes=[(image.height, image.width)])
segmentation_results = image_processor.post_process_semantic_segmentation(outputs, target_sizes=[(image.height, image.width)])

predicted_depth = depth_results[0]["predicted_depth"]  # (height, width) tensor with predicted depth in meters
normals = normal_results[0]["normals"] # (3, height, width) tensor with normals in XYZ format (L2-normalized)
segmentation = segmentation_results[0] # (height, width) tensor with class ids

# Visualization
import matplotlib.pyplot as plt

# Convert L2-normalized normals in [-1, 1] to RGB in [0, 255]
normals_rgb = ((normals + 1.0) / 2.0 * 255.0).clamp(0, 255).to(torch.uint8)

figure, axes = plt.subplots(2, 2, figsize=(10, 6))
axes[0, 0].imshow(image)
axes[0, 1].imshow(predicted_depth.cpu())
axes[0, 1].set_title("Depth")
axes[1, 0].imshow(normals_rgb.permute(1, 2, 0).cpu())
axes[1, 0].set_title("Normals")
axes[1, 1].imshow(segmentation.cpu())
axes[1, 1].set_title("Segmentation")
for axis in axes.flat:
    axis.axis("off")
plt.show()
```

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/tipsv2_all_tasks.jpg"
alt="tipsv2 all tasks" width="600"/>

```python
import torch
from transformers import AutoModelForDepthEstimation, AutoImageProcessor
from transformers.image_utils import load_image

model_id = "google/tipsv2-b14-dpt"
model = AutoModelForDepthEstimation.from_pretrained(model_id, device_map="auto")
image_processor = AutoImageProcessor.from_pretrained(model_id)

image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/room.jpg")
inputs = image_processor(images=image, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

results = image_processor.post_process_depth_estimation(outputs, target_sizes=[(image.height, image.width)])
predicted_depth = results[0]["predicted_depth"] # (height, width) tensor with predicted depth in meters

# Visualization
import matplotlib.pyplot as plt

figure, axes = plt.subplots(1, 2, figsize=(10, 5))
axes[0].imshow(image)
axes[1].imshow(predicted_depth.cpu())
axes[1].set_title("Depth")
for axis in axes:
    axis.axis("off")
plt.show()
```

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/tipsv2_depth_estimation.jpg"
alt="tipsv2 depth estimation" width="600"/>

```python
import torch
from transformers import Tipsv2DptForNormalEstimation, AutoImageProcessor
from transformers.image_utils import load_image

model_id = "google/tipsv2-b14-dpt"
model = Tipsv2DptForNormalEstimation.from_pretrained(model_id, device_map="auto")
image_processor = AutoImageProcessor.from_pretrained(model_id)

image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/room.jpg")
inputs = image_processor(images=image, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

results = image_processor.post_process_normal_estimation(outputs, target_sizes=[(image.height, image.width)])
normals = results[0]["normals"]  # (3, height, width) tensor with normals in XYZ format (L2-normalized)

# Visualization
import matplotlib.pyplot as plt

# Convert L2-normalized normals in [-1, 1] to RGB in [0, 255]
normals_rgb = ((normals + 1.0) / 2.0 * 255.0).clamp(0, 255).to(torch.uint8)

figure, axes = plt.subplots(1, 2, figsize=(10, 5))
axes[0].imshow(image)
axes[1].imshow(normals_rgb.permute(1, 2, 0).cpu())
axes[1].set_title("Normals")
for axis in axes:
    axis.axis("off")
plt.show()
```

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/tipsv2_normal_estimation.jpg"
alt="tipsv2 normal estimation" width="600"/>

```python
import torch
from transformers import AutoModelForSemanticSegmentation, AutoImageProcessor
from transformers.image_utils import load_image

model_id = "google/tipsv2-b14-dpt"
model = AutoModelForSemanticSegmentation.from_pretrained(model_id, device_map="auto")
image_processor = AutoImageProcessor.from_pretrained(model_id)

image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/room.jpg")
inputs = image_processor(images=image, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

results = image_processor.post_process_semantic_segmentation(outputs, target_sizes=[(image.height, image.width)])
segmentation_map = results[0]  # (height, width) tensor with class ids

# Visualization
import matplotlib.pyplot as plt

figure, axes = plt.subplots(1, 2, figsize=(10, 5))
axes[0].imshow(image)
axes[1].imshow(segmentation_map.cpu())
axes[1].set_title("Segmentation")
for axis in axes:
    axis.axis("off")
plt.show()
```

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/tipsv2_semantic_segmentation.jpg"
alt="tipsv2 semantic segmentation" width="600"/>

## Notes

- [Tipsv2DptForDensePrediction](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptForDensePrediction) runs a single shared backbone forward pass and produces three outputs simultaneously: `predicted_depth`, `normals`, and `segmentation_logits`. Use it when you need all three tasks for the same image.
- [Tipsv2DptForDepthEstimation](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptForDepthEstimation), [Tipsv2DptForNormalEstimation](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptForNormalEstimation), and [Tipsv2DptForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptForSemanticSegmentation) are single-task variants that discard the other heads. Use them in a pipeline or for inference on a single task.

## Tipsv2DptConfig[[transformers.Tipsv2DptConfig]]

- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **neck_hidden_sizes** (`list[int]`, *optional*, defaults to `[96, 192, 384, 768]`) --
  The hidden sizes to project to for the feature maps of the backbone.
- **fusion_hidden_size** (`int`, *optional*, defaults to 256) --
  The number of channels before fusion.
- **reassemble_factors** (`list[float]`, *optional*, defaults to `[4, 2, 1, 0.5]`) --
  The up/downsampling factors of the reassemble layers.
- **readout_activation** (`str`, *optional*, defaults to `"gelu_pytorch_tanh"`) --
  Activation applied after the readout projection layer.
- **num_depth_bins** (`int`, *optional*, defaults to 256) --
  The number of depth bins used by the depth-estimation head.
- **min_depth** (`float`, *optional*, defaults to 0.001) --
  The minimum depth value (meters) for depth bin calculation.
- **max_depth** (`float`, *optional*, defaults to 10.0) --
  The maximum depth value (meters) for depth bin calculation.
- **depth_decoder_activation** (`str`, *optional*, defaults to `"relu"`) --
  Activation applied after the depth decoder projection layer.
- **semantic_loss_ignore_index** (`int`, *optional*, defaults to 255) --
  Label index to ignore in the cross-entropy loss for semantic segmentation.

This is the configuration class to store the configuration of a Tipsv2 DptModel. It is used to instantiate a Tipsv2 Dpt
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/tipsv2-b14-dpt](https://huggingface.co/google/tipsv2-b14-dpt)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Tipsv2DptConfig, Tipsv2DptForDensePrediction

>>> configuration = Tipsv2DptConfig()
>>> model = Tipsv2DptForDensePrediction(configuration)
>>> configuration = model.config
```

## Tipsv2DptImageProcessor[[transformers.Tipsv2DptImageProcessor]]

- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a Tipsv2DptImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** (`DepthEstimatorOutput` or `Tipsv2DptDensePredictorOutput`) --
  Raw outputs of the model.
- **target_sizes** ([TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType) or `list[tuple[int, int]]`, *optional*) --
  Tensor of shape `(batch_size, 2)` or list of tuples (`tuple[int, int]`) containing the target size
  (height, width) of each image in the batch. If left to None, predictions will not be resized.`list[dict[str, torch.Tensor]]`A list of dictionaries of tensors representing the processed depth
predictions.

Converts the output of [Tipsv2DptForDepthEstimation](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptForDepthEstimation) or [Tipsv2DptForDensePrediction](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptForDensePrediction) into final depth predictions.

- **outputs** (`Tipsv2DptNormalEstimatorOutput` or `Tipsv2DptDensePredictorOutput`) --
  Raw outputs of the model.
- **target_sizes** ([TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType) or `list[tuple[int, int]]`, *optional*) --
  Tensor of shape `(batch_size, 2)` or list of tuples (`tuple[int, int]`) containing the target size
  (height, width) of each image in the batch. If left to None, predictions will not be resized.`list[dict[str, torch.Tensor]]` of length `batch_size`. Each dict has a `"normals"` key
mapping to a tensor of shape `(3, height, width)` with L2-normalized unit vectors in
`[-1, 1]` per channel (XYZ surface normals).

Converts the output of [Tipsv2DptForNormalEstimation](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptForNormalEstimation) or [Tipsv2DptForDensePrediction](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptForDensePrediction) into L2-normalized surface normal maps.

- **outputs** (`SemanticSegmenterOutput` or `Tipsv2DptDensePredictorOutput`) --
  Raw outputs of the model.
- **target_sizes** [([TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType) or `list[tuple[int, int]]`, *optional*) --
  Tensor of shape `(batch_size, 2)` or list of tuples (`tuple[int, int]`) containing the target size
  (height, width) of each image in the batch. If left to None, predictions will not be resized.
- **return_segmentation_scores** (`bool`, *optional*, defaults to `False`) --
  Whether to return segmentation scores alongside the segmentation map. When `True`, each element of
  the returned list is a `SemanticSegmentationPostProcessorOutput` with fields `segmentation`
  (class IDs, shape `(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`).`list[torch.Tensor]` or `list[SemanticSegmentationPostProcessorOutput]`When
`return_segmentation_scores=False` (default), a list of length `batch_size` where each item is a
segmentation map of shape `(height, width)` with class IDs. When `return_segmentation_scores=True`,
a list of `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape
`(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`). In both cases,
`(height, width)` corresponds to the target size (if `target_sizes` is specified).

Converts the output of [Tipsv2DptForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptForSemanticSegmentation) or [Tipsv2DptForDensePrediction](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptForDensePrediction) into semantic segmentation maps.

## Tipsv2DptForDensePrediction[[transformers.Tipsv2DptForDensePrediction]]

- **config** ([Tipsv2DptConfig](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

TIPSv2-DPT Model with three independent heads for depth estimation, surface normal estimation,
and semantic segmentation — running a single shared backbone forward pass.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Tipsv2DptImageProcessor](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptImageProcessor). See `Tipsv2DptImageProcessor.__call__()` for details (`processor_class` uses
  [Tipsv2DptImageProcessor](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptImageProcessor) for processing images).`Tipsv2DptDensePredictorOutput` or `tuple(torch.FloatTensor)`A `Tipsv2DptDensePredictorOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Tipsv2DptConfig](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptConfig)) and inputs.
The [Tipsv2DptForDensePrediction](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptForDensePrediction) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **predicted_depth** (`torch.FloatTensor` of shape `(batch_size, height, width)`) -- Predicted depth for each pixel.
- **normals** (`torch.FloatTensor` of shape `(batch_size, 3, height, width)`) -- Raw normal map predictions (unnormalized).
- **segmentation_logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels, height, width)`) -- Classification scores for each pixel.
  
  The logits returned do not necessarily have the same size as the `pixel_values` passed as inputs. This is
  to avoid doing two interpolations and lose some quality when a user needs to resize the logits to the
  original image size as post-processing. You should always check your logits shape and resize as needed.
  

Example:

```python
>>> import torch
>>> from transformers import Tipsv2DptForDensePrediction, AutoImageProcessor
>>> from transformers.image_utils import load_image

>>> model_id = "google/tipsv2-b14-dpt"
>>> model = Tipsv2DptForDensePrediction.from_pretrained(model_id, device_map="auto")
>>> image_processor = AutoImageProcessor.from_pretrained(model_id)

>>> image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/room.jpg")
>>> inputs = image_processor(images=image, return_tensors="pt").to(model.device)

>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> # outputs.predicted_depth: (batch_size, height, width) tensor with predicted depth in meters
>>> # outputs.normals: (batch_size, 3, height, width) tensor with normals in XYZ format (unnormalized)
>>> # outputs.segmentation_logits: (batch_size, config.num_labels, height, width) tensor with segmentation logits
>>> depth_results = image_processor.post_process_depth_estimation(outputs, target_sizes=[(image.height, image.width)])
>>> normal_results = image_processor.post_process_normal_estimation(outputs, target_sizes=[(image.height, image.width)])
>>> segmentation_results = image_processor.post_process_semantic_segmentation(outputs, target_sizes=[(image.height, image.width)])

>>> predicted_depth = depth_results[0]["predicted_depth"]  # (height, width) tensor with predicted depth in meters
>>> normals = normal_results[0]["normals"]  # (3, height, width) tensor with normals in XYZ format (L2-normalized)
>>> segmentation = segmentation_results[0]  # (height, width) tensor with class ids
```

## Tipsv2DptForDepthEstimation[[transformers.Tipsv2DptForDepthEstimation]]

- **config** ([Tipsv2DptConfig](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

TIPSv2-DPT Model with a monocular depth estimation head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Tipsv2DptImageProcessor](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptImageProcessor). See `Tipsv2DptImageProcessor.__call__()` for details (`processor_class` uses
  [Tipsv2DptImageProcessor](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptImageProcessor) for processing images).
- **labels** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.[DepthEstimatorOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.DepthEstimatorOutput) or `tuple(torch.FloatTensor)`A [DepthEstimatorOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.DepthEstimatorOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Tipsv2DptConfig](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptConfig)) and inputs.
The [Tipsv2DptForDepthEstimation](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptForDepthEstimation) forward method, overrides the `__call__` special method.

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
>>> import torch
>>> from transformers import AutoModelForDepthEstimation, AutoImageProcessor
>>> from transformers.image_utils import load_image

>>> model_id = "google/tipsv2-b14-dpt"
>>> model = AutoModelForDepthEstimation.from_pretrained(model_id, device_map="auto")
>>> image_processor = AutoImageProcessor.from_pretrained(model_id)

>>> image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/room.jpg")
>>> inputs = image_processor(images=image, return_tensors="pt").to(model.device)

>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> results = image_processor.post_process_depth_estimation(outputs, target_sizes=[(image.height, image.width)])
>>> predicted_depth = results[0]["predicted_depth"]  # (height, width) tensor with predicted depth in meters
```

## Tipsv2DptForNormalEstimation[[transformers.Tipsv2DptForNormalEstimation]]

- **config** ([Tipsv2DptConfig](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

TIPSv2-DPT Model with a surface normal estimation head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Tipsv2DptImageProcessor](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptImageProcessor). See `Tipsv2DptImageProcessor.__call__()` for details (`processor_class` uses
  [Tipsv2DptImageProcessor](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptImageProcessor) for processing images).
- **labels** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.`Tipsv2DptNormalEstimatorOutput` or `tuple(torch.FloatTensor)`A `Tipsv2DptNormalEstimatorOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Tipsv2DptConfig](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptConfig)) and inputs.
The [Tipsv2DptForNormalEstimation](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptForNormalEstimation) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Normal estimation loss.
- **normals** (`torch.FloatTensor` of shape `(batch_size, num_labels, height, width)`) -- Raw normal map predictions as output by the model (unnormalized).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage)
  of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states of the model at the output of
  each layer plus the initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one per layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`. Attentions weights after the attention softmax.

Example:

```python
>>> import torch
>>> from transformers import Tipsv2DptForNormalEstimation, AutoImageProcessor
>>> from transformers.image_utils import load_image

>>> model_id = "google/tipsv2-b14-dpt"
>>> model = Tipsv2DptForNormalEstimation.from_pretrained(model_id, device_map="auto")
>>> image_processor = AutoImageProcessor.from_pretrained(model_id)

>>> image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/room.jpg")
>>> inputs = image_processor(images=image, return_tensors="pt").to(model.device)

>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> results = image_processor.post_process_normal_estimation(outputs, target_sizes=[(image.height, image.width)])
>>> normals = results[0]["normals"]  # (3, height, width) tensor with normals in XYZ format (L2-normalized)
```

## Tipsv2DptForSemanticSegmentation[[transformers.Tipsv2DptForSemanticSegmentation]]

- **config** ([Tipsv2DptConfig](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

TIPSv2-DPT Model with a semantic segmentation head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Tipsv2DptImageProcessor](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptImageProcessor). See `Tipsv2DptImageProcessor.__call__()` for details (`processor_class` uses
  [Tipsv2DptImageProcessor](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptImageProcessor) for processing images).
- **labels** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Ground truth semantic segmentation maps for computing the loss. Indices should be in `[0, ...,
  config.num_labels - 1]`. If `config.num_labels > 1`, a classification loss is computed (Cross-Entropy).</paramsdesc><rettype>[SemanticSegmenterOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.SemanticSegmenterOutput) or `tuple(torch.FloatTensor)`A [SemanticSegmenterOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.SemanticSegmenterOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Tipsv2DptConfig](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptConfig)) and inputs.
The [Tipsv2DptForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/tipsv2_dpt#transformers.Tipsv2DptForSemanticSegmentation) forward method, overrides the `__call__` special method.

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

Example:

```python
>>> import torch
>>> from transformers import AutoModelForSemanticSegmentation, AutoImageProcessor
>>> from transformers.image_utils import load_image

>>> model_id = "google/tipsv2-b14-dpt"
>>> model = AutoModelForSemanticSegmentation.from_pretrained(model_id, device_map="auto")
>>> image_processor = AutoImageProcessor.from_pretrained(model_id)

>>> image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/room.jpg")
>>> inputs = image_processor(images=image, return_tensors="pt").to(model.device)

>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> results = image_processor.post_process_semantic_segmentation(outputs, target_sizes=[(image.height, image.width)])
>>> segmentation_map = results[0]  # (height, width) tensor with class ids
```
