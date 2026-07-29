# ZoeDepth

[ZoeDepth](https://huggingface.co/papers/2302.12288) is a depth estimation model that combines the generalization performance of relative depth estimation (how far objects are from each other) and metric depth estimation (precise depth measurement on metric scale) from a single image. It is pre-trained on 12 datasets using relative depth and 2 datasets (NYU Depth v2 and KITTI) for metric accuracy. A lightweight head with a metric bin module for each domain is used, and during inference, it automatically selects the appropriate head for each input image with a latent classifier.

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/zoedepth_architecture_bis.png"
alt="drawing" width="600"/>

You can find all the original ZoeDepth checkpoints under the [Intel](https://huggingface.co/Intel?search=zoedepth) organization.

The example below demonstrates how to estimate depth with [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) class.

```python
import requests
from PIL import Image

from transformers import pipeline

url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
image = Image.open(requests.get(url, stream=True).raw)
pipeline = pipeline(
    task="depth-estimation",
    model="Intel/zoedepth-nyu-kitti",
    device=0
)
results = pipeline(image)
results["depth"]
```

```python
import requests
import torch
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForDepthEstimation

image_processor = AutoImageProcessor.from_pretrained(
    "Intel/zoedepth-nyu-kitti"
)
model = AutoModelForDepthEstimation.from_pretrained(
    "Intel/zoedepth-nyu-kitti",
    device_map="auto"
)
url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
image = Image.open(requests.get(url, stream=True).raw)
inputs = image_processor(image, return_tensors="pt").to(model.device)

with torch.no_grad():
  outputs = model(inputs)

# interpolate to original size and visualize the prediction
## ZoeDepth dynamically pads the input image, so pass the original image size as argument
## to `post_process_depth_estimation` to remove the padding and resize to original dimensions.
post_processed_output = image_processor.post_process_depth_estimation(
    outputs,
    source_sizes=[(image.height, image.width)],
)

predicted_depth = post_processed_output[0]["predicted_depth"]
depth = (predicted_depth - predicted_depth.min()) / (predicted_depth.max() - predicted_depth.min())
depth = depth.detach().cpu().numpy() * 255
Image.fromarray(depth.astype("uint8"))
```

## Notes

- In the [original implementation](https://github.com/isl-org/ZoeDepth/blob/edb6daf45458569e24f50250ef1ed08c015f17a7/zoedepth/models/depth_model.py#L131) ZoeDepth performs inference on both the original and flipped images and averages the results. The `post_process_depth_estimation` function handles this by passing the flipped outputs to the optional `outputs_flipped` argument as shown below.

   ```py
    with torch.no_grad():
        outputs = model(pixel_values)
        outputs_flipped = model(pixel_values=torch.flip(inputs.pixel_values, dims=[3]))
        post_processed_output = image_processor.post_process_depth_estimation(
            outputs,
            source_sizes=[(image.height, image.width)],
            outputs_flipped=outputs_flipped,
        )
   ```

## Resources

- Refer to this [notebook](https://github.com/NielsRogge/Transformers-Tutorials/tree/master/ZoeDepth) for an inference example.

## ZoeDepthConfig[[transformers.ZoeDepthConfig]]

- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **batch_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the batch normalization layers.
- **readout_type** (`str`, *optional*, defaults to `"project"`) --
  The readout type to use when processing the readout token (CLS token) of the intermediate hidden states of
  the ViT backbone. Can be one of [`"ignore"`, `"add"`, `"project"`].
  - "ignore" simply ignores the CLS token.
  - "add" passes the information from the CLS token to all other tokens by adding the representations.
  - "project" passes information to the other tokens by concatenating the readout to all other tokens before
    projecting the
  representation to the original feature dimension D using a linear layer followed by a GELU non-linearity.
- **reassemble_factors** (`list[int]`, *optional*, defaults to `[4, 2, 1, 0.5]`) --
  The up/downsampling factors of the reassemble layers.
- **neck_hidden_sizes** (`list[str]`, *optional*, defaults to `[96, 192, 384, 768]`) --
  The hidden sizes to project to for the feature maps of the backbone.
- **fusion_hidden_size** (`int`, *optional*, defaults to 256) --
  The number of channels before fusion.
- **head_in_index** (`int`, *optional*, defaults to -1) --
  The index of the features to use in the heads.
- **use_batch_norm_in_fusion_residual** (`bool`, *optional*, defaults to `False`) --
  Whether to use batch normalization in the pre-activate residual units of the fusion blocks.
- **use_bias_in_fusion_residual** (`bool`, *optional*, defaults to `True`) --
  Whether to use bias in the pre-activate residual units of the fusion blocks.
- **num_relative_features** (`int`, *optional*, defaults to 32) --
  The number of features to use in the relative depth estimation head.
- **add_projection** (`bool`, *optional*, defaults to `False`) --
  Whether to add a projection layer before the depth estimation head.
- **bottleneck_features** (`int`, *optional*, defaults to 256) --
  The number of features in the bottleneck layer.
- **num_attractors** (`list[int], *optional*, defaults to `[16, 8, 4, 1]`) --
  The number of attractors to use in each stage.
- **bin_embedding_dim** (`int`, *optional*, defaults to 128) --
  The dimension of the bin embeddings.
- **attractor_alpha** (`int`, *optional*, defaults to 1000) --
  The alpha value to use in the attractor.
- **attractor_gamma** (`int`, *optional*, defaults to 2) --
  The gamma value to use in the attractor.
- **attractor_kind** (`str`, *optional*, defaults to `"mean"`) --
  The kind of attractor to use. Can be one of [`"mean"`, `"sum"`].
- **min_temp** (`float`, *optional*, defaults to 0.0212) --
  The minimum temperature value to consider.
- **max_temp** (`float`, *optional*, defaults to 50.0) --
  The maximum temperature value to consider.
- **bin_centers_type** (`str`, *optional*, defaults to `"softplus"`) --
  Activation type used for bin centers. Can be "normed" or "softplus". For "normed" bin centers, linear normalization trick
  is applied. This results in bounded bin centers. For "softplus", softplus activation is used and thus are unbounded.
- **bin_configurations** (`list[dict]`, *optional*, defaults to `[{'n_bins' -- 64, 'min_depth': 0.001, 'max_depth': 10.0}]`):
  Configuration for each of the bin heads.
  Each configuration should consist of the following keys:
  - name (`str`): The name of the bin head - only required in case of multiple bin configurations.
  - `n_bins` (`int`): The number of bins to use.
  - `min_depth` (`float`): The minimum depth value to consider.
  - `max_depth` (`float`): The maximum depth value to consider.
  In case only a single configuration is passed, the model will use a single head with the specified configuration.
  In case multiple configurations are passed, the model will use multiple heads with the specified configurations.
- **num_patch_transformer_layers** (`int`, *optional*) --
  The number of transformer layers to use in the patch transformer. Only used in case of multiple bin configurations.
- **patch_transformer_hidden_size** (`int`, *optional*) --
  The hidden size to use in the patch transformer. Only used in case of multiple bin configurations.
- **patch_transformer_intermediate_size** (`int`, *optional*) --
  The intermediate size to use in the patch transformer. Only used in case of multiple bin configurations.
- **patch_transformer_num_attention_heads** (`int`, *optional*) --
  The number of attention heads to use in the patch transformer. Only used in case of multiple bin configurations.

This is the configuration class to store the configuration of a ZoedepthModel. It is used to instantiate a Zoedepth
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Intel/zoedepth-nyu](https://huggingface.co/Intel/zoedepth-nyu)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import ZoeDepthConfig, ZoeDepthForDepthEstimation

>>> # Initializing a ZoeDepth zoedepth-large style configuration
>>> configuration = ZoeDepthConfig()

>>> # Initializing a model from the zoedepth-large style configuration
>>> model = ZoeDepthForDepthEstimation(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## ZoeDepthImageProcessor[[transformers.ZoeDepthImageProcessor]]

- **keep_aspect_ratio** (`bool`, *kwargs*, *optional*, defaults to `self.keep_aspect_ratio`) --
  If `True`, the image is resized by choosing the smaller of the height and width scaling factors and using it
  for both dimensions. This ensures that the image is scaled down as little as possible while still fitting
  within the desired output size. In case `ensure_multiple_of` is also set, the image is further resized to a
  size that is a multiple of this value by flooring the height and width to the nearest multiple of this value.
  Can be overridden by `keep_aspect_ratio` in `preprocess`.
- **ensure_multiple_of** (`int`, *kwargs*, *optional*, defaults to `self.ensure_multiple_of`) --
  If `do_resize` is `True`, the image is resized to a size that is a multiple of this value. Works by flooring
  the height and width to the nearest multiple of this value.
  Works both with and without `keep_aspect_ratio` being set to `True`.
  Can be overridden by `ensure_multiple_of` in `preprocess`.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a ZoeDepthImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **keep_aspect_ratio** (`bool`, *kwargs*, *optional*, defaults to `self.keep_aspect_ratio`) --
  If `True`, the image is resized by choosing the smaller of the height and width scaling factors and using it
  for both dimensions. This ensures that the image is scaled down as little as possible while still fitting
  within the desired output size. In case `ensure_multiple_of` is also set, the image is further resized to a
  size that is a multiple of this value by flooring the height and width to the nearest multiple of this value.
  Can be overridden by `keep_aspect_ratio` in `preprocess`.
- **ensure_multiple_of** (`int`, *kwargs*, *optional*, defaults to `self.ensure_multiple_of`) --
  If `do_resize` is `True`, the image is resized to a size that is a multiple of this value. Works by flooring
  the height and width to the nearest multiple of this value.
  Works both with and without `keep_aspect_ratio` being set to `True`.
  Can be overridden by `ensure_multiple_of` in `preprocess`.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## ZoeDepthImageProcessorPil[[transformers.ZoeDepthImageProcessorPil]]

- **keep_aspect_ratio** (`bool`, *kwargs*, *optional*, defaults to `self.keep_aspect_ratio`) --
  If `True`, the image is resized by choosing the smaller of the height and width scaling factors and using it
  for both dimensions. This ensures that the image is scaled down as little as possible while still fitting
  within the desired output size. In case `ensure_multiple_of` is also set, the image is further resized to a
  size that is a multiple of this value by flooring the height and width to the nearest multiple of this value.
  Can be overridden by `keep_aspect_ratio` in `preprocess`.
- **ensure_multiple_of** (`int`, *kwargs*, *optional*, defaults to `self.ensure_multiple_of`) --
  If `do_resize` is `True`, the image is resized to a size that is a multiple of this value. Works by flooring
  the height and width to the nearest multiple of this value.
  Works both with and without `keep_aspect_ratio` being set to `True`.
  Can be overridden by `ensure_multiple_of` in `preprocess`.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a ZoeDepthImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **keep_aspect_ratio** (`bool`, *kwargs*, *optional*, defaults to `self.keep_aspect_ratio`) --
  If `True`, the image is resized by choosing the smaller of the height and width scaling factors and using it
  for both dimensions. This ensures that the image is scaled down as little as possible while still fitting
  within the desired output size. In case `ensure_multiple_of` is also set, the image is further resized to a
  size that is a multiple of this value by flooring the height and width to the nearest multiple of this value.
  Can be overridden by `keep_aspect_ratio` in `preprocess`.
- **ensure_multiple_of** (`int`, *kwargs*, *optional*, defaults to `self.ensure_multiple_of`) --
  If `do_resize` is `True`, the image is resized to a size that is a multiple of this value. Works by flooring
  the height and width to the nearest multiple of this value.
  Works both with and without `keep_aspect_ratio` being set to `True`.
  Can be overridden by `ensure_multiple_of` in `preprocess`.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## ZoeDepthForDepthEstimation[[transformers.ZoeDepthForDepthEstimation]]

- **config** ([ZoeDepthForDepthEstimation](/docs/transformers/v5.14.0/en/model_doc/zoedepth#transformers.ZoeDepthForDepthEstimation)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

ZoeDepth model with one or multiple metric depth estimation head(s) on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [ZoeDepthImageProcessor](/docs/transformers/v5.14.0/en/model_doc/zoedepth#transformers.ZoeDepthImageProcessor). See `ZoeDepthImageProcessor.__call__()` for details (`processor_class` uses
  [ZoeDepthImageProcessor](/docs/transformers/v5.14.0/en/model_doc/zoedepth#transformers.ZoeDepthImageProcessor) for processing images).
- **labels** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Ground truth depth estimation maps for computing the loss.
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.[DepthEstimatorOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.DepthEstimatorOutput) or `tuple(torch.FloatTensor)`A [DepthEstimatorOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.DepthEstimatorOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ZoeDepthConfig](/docs/transformers/v5.14.0/en/model_doc/zoedepth#transformers.ZoeDepthConfig)) and inputs.
The [ZoeDepthForDepthEstimation](/docs/transformers/v5.14.0/en/model_doc/zoedepth#transformers.ZoeDepthForDepthEstimation) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoImageProcessor, ZoeDepthForDepthEstimation
>>> import torch
>>> import numpy as np
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("Intel/zoedepth-nyu-kitti")
>>> model = ZoeDepthForDepthEstimation.from_pretrained("Intel/zoedepth-nyu-kitti")

>>> # prepare image for the model
>>> inputs = image_processor(images=image, return_tensors="pt")

>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> # interpolate to original size
>>> post_processed_output = image_processor.post_process_depth_estimation(
...     outputs,
...     source_sizes=[(image.height, image.width)],
... )

>>> # visualize the prediction
>>> predicted_depth = post_processed_output[0]["predicted_depth"]
>>> depth = predicted_depth * 255 / predicted_depth.max()
>>> depth = depth.detach().cpu().numpy()
>>> depth = Image.fromarray(depth.astype("uint8"))
```
