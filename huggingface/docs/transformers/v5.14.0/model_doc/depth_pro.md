# DepthPro

## Overview

The DepthPro model was proposed in [Depth Pro: Sharp Monocular Metric Depth in Less Than a Second](https://huggingface.co/papers/2410.02073) by Aleksei Bochkovskii, Amaël Delaunoy, Hugo Germain, Marcel Santos, Yichao Zhou, Stephan R. Richter, Vladlen Koltun.

DepthPro is a foundation model for zero-shot metric monocular depth estimation, designed to generate high-resolution depth maps with remarkable sharpness and fine-grained details. It employs a multi-scale Vision Transformer (ViT)-based architecture, where images are downsampled, divided into patches, and processed using a shared Dinov2 encoder. The extracted patch-level features are merged, upsampled, and refined using a DPT-like fusion stage, enabling precise depth estimation.

The abstract from the paper is the following:

*We present a foundation model for zero-shot metric monocular depth estimation. Our model, Depth Pro, synthesizes high-resolution depth maps with unparalleled sharpness and high-frequency details. The predictions are metric, with absolute scale, without relying on the availability of metadata such as camera intrinsics. And the model is fast, producing a 2.25-megapixel depth map in 0.3 seconds on a standard GPU. These characteristics are enabled by a number of technical contributions, including an efficient multi-scale vision transformer for dense prediction, a training protocol that combines real and synthetic datasets to achieve high metric accuracy alongside fine boundary tracing, dedicated evaluation metrics for boundary accuracy in estimated depth maps, and state-of-the-art focal length estimation from a single image. Extensive experiments analyze specific design choices and demonstrate that Depth Pro outperforms prior work along multiple dimensions.*

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/depth_pro_teaser.png"
alt="drawing" width="600"/>

 DepthPro Outputs. Taken from the official code. 

This model was contributed by [geetu040](https://github.com/geetu040). The original code can be found [here](https://github.com/apple/ml-depth-pro).

## Usage Tips

The DepthPro model processes an input image by first downsampling it at multiple scales and splitting each scaled version into patches. These patches are then encoded using a shared Vision Transformer (ViT)-based Dinov2 patch encoder, while the full image is processed by a separate image encoder. The extracted patch features are merged into feature maps, upsampled, and fused using a DPT-like decoder to generate the final depth estimation. If enabled, an additional Field of View (FOV) encoder processes the image for estimating the camera's field of view, aiding in depth accuracy.

```python
import requests
import torch
from PIL import Image

from transformers import DepthProForDepthEstimation, DepthProImageProcessor

url = 'http://images.cocodataset.org/val2017/000000039769.jpg'
image = Image.open(requests.get(url, stream=True).raw)

image_processor = DepthProImageProcessor.from_pretrained("apple/DepthPro-hf")
model = DepthProForDepthEstimation.from_pretrained("apple/DepthPro-hf", device_map="auto")

inputs = image_processor(images=image, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

post_processed_output = image_processor.post_process_depth_estimation(
    outputs, target_sizes=[(image.height, image.width)],
)

field_of_view = post_processed_output[0]["field_of_view"]
focal_length = post_processed_output[0]["focal_length"]
depth = post_processed_output[0]["predicted_depth"]
depth = (depth - depth.min()) / depth.max()
depth = depth * 255.
depth = depth.detach().cpu().numpy()
depth = Image.fromarray(depth.astype("uint8"))
```

### Architecture and Configuration

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/depth_pro_architecture.png"
alt="drawing" width="600"/>

 DepthPro architecture. Taken from the original paper. 

The `DepthProForDepthEstimation` model uses a `DepthProEncoder`, for encoding the input image and a `FeatureFusionStage` for fusing the output features from encoder.

The `DepthProEncoder` further uses two encoders:

- `patch_encoder`
  - Input image is scaled with multiple ratios, as specified in the `scaled_images_ratios` configuration.
  - Each scaled image is split into smaller **patches** of size `patch_size` with overlapping areas determined by `scaled_images_overlap_ratios`.
  - These patches are processed by the **`patch_encoder`**
- `image_encoder`
  - Input image is also rescaled to `patch_size` and processed by the **`image_encoder`**

Both these encoders can be configured via `patch_model_config` and `image_model_config` respectively, both of which are separate `Dinov2Model` by default.

Outputs from both encoders (`last_hidden_state`) and selected intermediate states (`hidden_states`) from **`patch_encoder`** are fused by a `DPT`-based `FeatureFusionStage` for depth estimation.

### Field-of-View (FOV) Prediction

The network is supplemented with a focal length estimation head. A small convolutional head ingests frozen features from the depth estimation network and task-specific features from a separate ViT image encoder to predict the horizontal angular field-of-view.

The `use_fov_model` parameter in `DepthProConfig` controls whether **FOV prediction** is enabled. By default, it is set to `False` to conserve memory and computation. When enabled, the **FOV encoder** is instantiated based on the `fov_model_config` parameter, which defaults to a `Dinov2Model`. The `use_fov_model` parameter can also be passed when initializing the `DepthProForDepthEstimation` model.

The pretrained model at checkpoint `apple/DepthPro-hf` uses the FOV encoder. To use the pretrained-model without FOV encoder, set `use_fov_model=False` when loading the model, which saves computation.

```python
from transformers import DepthProForDepthEstimation

model = DepthProForDepthEstimation.from_pretrained("apple/DepthPro-hf", use_fov_model=False, device_map="auto")
```

To instantiate a new model with FOV encoder, set `use_fov_model=True` in the config.

```python
from transformers import DepthProConfig, DepthProForDepthEstimation

config = DepthProConfig(use_fov_model=True)
model = DepthProForDepthEstimation(config)
```

Or set `use_fov_model=True` when initializing the model, which overrides the value in config.

```python
from transformers import DepthProConfig, DepthProForDepthEstimation

config = DepthProConfig()
model = DepthProForDepthEstimation(config, use_fov_model=True)
```

### Using Scaled Dot Product Attention (SDPA)

PyTorch includes a native scaled dot-product attention (SDPA) operator as part of `torch.nn.functional`. This function
encompasses several implementations that can be applied depending on the inputs and the hardware in use. See the
[official documentation](https://pytorch.org/docs/stable/generated/torch.nn.functional.scaled_dot_product_attention.html)
or the [GPU Inference](https://huggingface.co/docs/transformers/main/en/perf_infer_gpu_one#pytorch-scaled-dot-product-attention)
page for more information.

SDPA is used by default for `torch>=2.1.1` when an implementation is available, but you may also set
`attn_implementation="sdpa"` in `from_pretrained()` to explicitly request SDPA to be used.

```python
from transformers import DepthProForDepthEstimation

model = DepthProForDepthEstimation.from_pretrained("apple/DepthPro-hf", attn_implementation="sdpa", device_map="auto")
```

For the best speedups, we recommend loading the model in half-precision (e.g. `torch.float16` or `torch.bfloat16`).

On a local benchmark (A100-40GB, PyTorch 2.3.0, OS Ubuntu 22.04) with `float32` and `google/vit-base-patch16-224` model, we saw the following speedups during inference.

|   Batch size |   Average inference time (ms), eager mode |   Average inference time (ms), sdpa model |   Speed up, Sdpa / Eager (x) |
|--------------|-------------------------------------------|-------------------------------------------|------------------------------|
|            1 |                                         7 |                                         6 |                      1.17 |
|            2 |                                         8 |                                         6 |                      1.33 |
|            4 |                                         8 |                                         6 |                      1.33 |
|            8 |                                         8 |                                         6 |                      1.33 |

## Resources

A list of official Hugging Face and community (indicated by 🌎) resources to help you get started with DepthPro:

- Research Paper: [Depth Pro: Sharp Monocular Metric Depth in Less Than a Second](https://huggingface.co/papers/2410.02073)
- Official Implementation: [apple/ml-depth-pro](https://github.com/apple/ml-depth-pro)
- DepthPro Inference Notebook: [DepthPro Inference](https://github.com/qubvel/transformers-notebooks/blob/main/notebooks/DepthPro_inference.ipynb)
- DepthPro for Super Resolution and Image Segmentation
  - Read blog on Medium: [Depth Pro: Beyond Depth](https://medium.com/@raoarmaghanshakir040/depth-pro-beyond-depth-9d822fc557ba)
  - Code on Github: [geetu040/depthpro-beyond-depth](https://github.com/geetu040/depthpro-beyond-depth)

If you're interested in submitting a resource to be included here, please feel free to open a Pull Request and we'll review it! The resource should ideally demonstrate something new instead of duplicating an existing resource.

## DepthProConfig[[transformers.DepthProConfig]]

- **fusion_hidden_size** (`int`, *optional*, defaults to 256) --
  The number of channels before fusion.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `384`) --
  The size (resolution) of each patch.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **intermediate_hook_ids** (`list[int]`, *optional*, defaults to `[11, 5]`) --
  Indices of the intermediate hidden states from the patch encoder to use for fusion.
- **intermediate_feature_dims** (`list[int]`, *optional*, defaults to `[256, 256]`) --
  Hidden state dimensions during upsampling for each intermediate hidden state in `intermediate_hook_ids`.
- **scaled_images_ratios** (`list[float]`, *optional*, defaults to `[0.25, 0.5, 1]`) --
  Ratios of scaled images to be used by the patch encoder.
- **scaled_images_overlap_ratios** (`list[float]`, *optional*, defaults to `[0.0, 0.5, 0.25]`) --
  Overlap ratios between patches for each scaled image in `scaled_images_ratios`.
- **scaled_images_feature_dims** (`list[int]`, *optional*, defaults to `[1024, 1024, 512]`) --
  Hidden state dimensions during upsampling for each scaled image in `scaled_images_ratios`.
- **merge_padding_value** (`int`, *optional*, defaults to 3) --
  When merging smaller patches back to the image size, overlapping sections of this size are removed.
- **use_batch_norm_in_fusion_residual** (`bool`, *optional*, defaults to `False`) --
  Whether to use batch normalization in the pre-activate residual units of the fusion blocks.
- **use_bias_in_fusion_residual** (`bool`, *optional*, defaults to `True`) --
  Whether to use bias in the pre-activate residual units of the fusion blocks.
- **use_fov_model** (`bool`, *optional*, defaults to `False`) --
  Whether to use `DepthProFovModel` to generate the field of view.
- **num_fov_head_layers** (`int`, *optional*, defaults to 2) --
  Number of convolution layers in the head of `DepthProFovModel`.
- **image_model_config** (`Union[dict[str, Any], PreTrainedConfig]`, *optional*) --
  The configuration of the image encoder model, which is loaded using the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) API.
  By default, Dinov2 model is used as backbone.
- **patch_model_config** (`Union[dict[str, Any], PreTrainedConfig]`, *optional*) --
  The configuration of the patch encoder model, which is loaded using the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) API.
  By default, Dinov2 model is used as backbone.
- **fov_model_config** (`Union[dict[str, Any], PreTrainedConfig]`, *optional*) --
  The configuration of the fov encoder model, which is loaded using the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) API.
  By default, Dinov2 model is used as backbone.

This is the configuration class to store the configuration of a DepthProModel. It is used to instantiate a Depth Pro
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [apple/DepthPro](https://huggingface.co/apple/DepthPro)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import DepthProConfig, DepthProModel

>>> # Initializing a DepthPro apple/DepthPro style configuration
>>> configuration = DepthProConfig()

>>> # Initializing a model (with random weights) from the apple/DepthPro style configuration
>>> model = DepthProModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## DepthProImageProcessor[[transformers.DepthProImageProcessor]]

- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a DepthPro image processor.

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

Post-processes the raw depth predictions from the model.

## DepthProModel[[transformers.DepthProModel]]

- **config** ([DepthProModel](/docs/transformers/v5.14.0/en/model_doc/depth_pro#transformers.DepthProModel)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Depth Pro Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [DepthProImageProcessor](/docs/transformers/v5.14.0/en/model_doc/depth_pro#transformers.DepthProImageProcessor). See `DepthProImageProcessor.__call__()` for details (`processor_class` uses
  [DepthProImageProcessor](/docs/transformers/v5.14.0/en/model_doc/depth_pro#transformers.DepthProImageProcessor) for processing images).
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.`DepthProOutput` or `tuple(torch.FloatTensor)`A `DepthProOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DepthProConfig](/docs/transformers/v5.14.0/en/model_doc/depth_pro#transformers.DepthProConfig)) and inputs.
The [DepthProModel](/docs/transformers/v5.14.0/en/model_doc/depth_pro#transformers.DepthProModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, n_patches_per_batch, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **features** (`Union[torch.FloatTensor, List[torch.FloatTensor]]`, *optional*) -- Features from encoders. Can be a single feature or a list of features.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> import torch
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoProcessor, DepthProModel

>>> url = "https://www.ilankelman.org/stopsigns/australia.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> checkpoint = "apple/DepthPro-hf"
>>> processor = AutoProcessor.from_pretrained(checkpoint)
>>> model = DepthProModel.from_pretrained(checkpoint)

>>> # prepare image for the model
>>> inputs = processor(images=image, return_tensors="pt")

>>> with torch.no_grad():
...     output = model(**inputs)

>>> output.last_hidden_state.shape
torch.Size([1, 35, 577, 1024])
```

## DepthProForDepthEstimation[[transformers.DepthProForDepthEstimation]]

- **config** ([DepthProForDepthEstimation](/docs/transformers/v5.14.0/en/model_doc/depth_pro#transformers.DepthProForDepthEstimation)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
- **use_fov_model** (`bool`, *optional*) --
  Whether to use the field of view model.

DepthPro Model with a depth estimation head on top (consisting of 3 convolutional layers).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [DepthProImageProcessor](/docs/transformers/v5.14.0/en/model_doc/depth_pro#transformers.DepthProImageProcessor). See `DepthProImageProcessor.__call__()` for details (`processor_class` uses
  [DepthProImageProcessor](/docs/transformers/v5.14.0/en/model_doc/depth_pro#transformers.DepthProImageProcessor) for processing images).
- **labels** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Ground truth depth estimation maps for computing the loss.
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.`DepthProDepthEstimatorOutput` or `tuple(torch.FloatTensor)`A `DepthProDepthEstimatorOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DepthProConfig](/docs/transformers/v5.14.0/en/model_doc/depth_pro#transformers.DepthProConfig)) and inputs.
The [DepthProForDepthEstimation](/docs/transformers/v5.14.0/en/model_doc/depth_pro#transformers.DepthProForDepthEstimation) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **predicted_depth** (`torch.FloatTensor` of shape `(batch_size, height, width)`, *optional*) -- Predicted depth for each pixel.
- **field_of_view** (`torch.FloatTensor` of shape `(batch_size,)`, *optional*, returned when `use_fov_model` is provided) -- Field of View Scaler.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import AutoImageProcessor, DepthProForDepthEstimation
>>> import torch
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> checkpoint = "apple/DepthPro-hf"
>>> processor = AutoImageProcessor.from_pretrained(checkpoint)
>>> model = DepthProForDepthEstimation.from_pretrained(checkpoint)

>>> device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
>>> model.to(device)

>>> # prepare image for the model
>>> inputs = processor(images=image, return_tensors="pt").to(device)

>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> # interpolate to original size
>>> post_processed_output = processor.post_process_depth_estimation(
...     outputs, target_sizes=[(image.height, image.width)],
... )

>>> # get the field of view (fov) predictions
>>> field_of_view = post_processed_output[0]["field_of_view"]
>>> focal_length = post_processed_output[0]["focal_length"]

>>> # visualize the prediction
>>> predicted_depth = post_processed_output[0]["predicted_depth"]
>>> depth = predicted_depth * 255 / predicted_depth.max()
>>> depth = depth.detach().cpu().numpy()
>>> depth = Image.fromarray(depth.astype("uint8"))
```
