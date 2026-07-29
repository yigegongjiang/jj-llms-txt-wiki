# ViTMatte

## Overview

The ViTMatte model was proposed in [Boosting Image Matting with Pretrained Plain Vision Transformers](https://huggingface.co/papers/2305.15272) by Jingfeng Yao, Xinggang Wang, Shusheng Yang, Baoyuan Wang.
ViTMatte leverages plain [Vision Transformers](vit) for the task of image matting, which is the process of accurately estimating the foreground object in images and videos.

The abstract from the paper is the following:

*Recently, plain vision Transformers (ViTs) have shown impressive performance on various computer vision tasks, thanks to their strong modeling capacity and large-scale pretraining. However, they have not yet conquered the problem of image matting. We hypothesize that image matting could also be boosted by ViTs and present a new efficient and robust ViT-based matting system, named ViTMatte. Our method utilizes (i) a hybrid attention mechanism combined with a convolution neck to help ViTs achieve an excellent performance-computation trade-off in matting tasks. (ii) Additionally, we introduce the detail capture module, which just consists of simple lightweight convolutions to complement the detailed information required by matting. To the best of our knowledge, ViTMatte is the first work to unleash the potential of ViT on image matting with concise adaptation. It inherits many superior properties from ViT to matting, including various pretraining strategies, concise architecture design, and flexible inference strategies. We evaluate ViTMatte on Composition-1k and Distinctions-646, the most commonly used benchmark for image matting, our method achieves state-of-the-art performance and outperforms prior matting works by a large margin.*

This model was contributed by [nielsr](https://huggingface.co/nielsr).
The original code can be found [here](https://github.com/hustvl/ViTMatte).

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/vitmatte_architecture.png"
alt="drawing" width="600"/>

 ViTMatte high-level overview. Taken from the original paper. 

## Resources

A list of official Hugging Face and community (indicated by 🌎) resources to help you get started with ViTMatte.

- A demo notebook regarding inference with [VitMatteForImageMatting](/docs/transformers/v5.14.0/en/model_doc/vitmatte#transformers.VitMatteForImageMatting), including background replacement, can be found [here](https://github.com/NielsRogge/Transformers-Tutorials/tree/master/ViTMatte).

The model expects both the image and trimap (concatenated) as input. Use `ViTMatteImageProcessor` for this purpose.

## VitMatteConfig[[transformers.VitMatteConfig]]

- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **hidden_size** (`int`, *optional*, defaults to `384`) --
  Dimension of the hidden representations.
- **batch_norm_eps** (`float`, *optional*, defaults to 1e-05) --
  The epsilon used by the batch norm layers.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **convstream_hidden_sizes** (`list[int]`, *optional*, defaults to `[48, 96, 192]`) --
  The output channels of the ConvStream module.
- **fusion_hidden_sizes** (`list[int]`, *optional*, defaults to `[256, 128, 64, 32]`) --
  The output channels of the Fusion blocks.

This is the configuration class to store the configuration of a VitmatteModel. It is used to instantiate a Vitmatte
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [hustvl/vitmatte-small-composition-1k](https://huggingface.co/hustvl/vitmatte-small-composition-1k)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import VitMatteConfig, VitMatteForImageMatting

>>> # Initializing a ViTMatte hustvl/vitmatte-small-composition-1k style configuration
>>> configuration = VitMatteConfig()

>>> # Initializing a model (with random weights) from the hustvl/vitmatte-small-composition-1k style configuration
>>> model = VitMatteForImageMatting(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## VitMatteImageProcessor[[transformers.VitMatteImageProcessor]]

- **size_divisor** (`int`, *kwargs*, *optional*, defaults to `self.size_divisor`) --
  The width and height of the image will be padded to be divisible by this number.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a VitMatteImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **trimaps** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  The trimaps to preprocess.
- **size_divisor** (`int`, *kwargs*, *optional*, defaults to `self.size_divisor`) --
  The width and height of the image will be padded to be divisible by this number.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## VitMatteImageProcessorPil[[transformers.VitMatteImageProcessorPil]]

- **size_divisor** (`int`, *kwargs*, *optional*, defaults to `self.size_divisor`) --
  The width and height of the image will be padded to be divisible by this number.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a VitMatteImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **trimaps** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  The trimaps to preprocess.
- **size_divisor** (`int`, *kwargs*, *optional*, defaults to `self.size_divisor`) --
  The width and height of the image will be padded to be divisible by this number.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## VitMatteForImageMatting[[transformers.VitMatteForImageMatting]]

- **config** ([VitMatteForImageMatting](/docs/transformers/v5.14.0/en/model_doc/vitmatte#transformers.VitMatteForImageMatting)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

ViTMatte framework leveraging any vision backbone e.g. for ADE20k, CityScapes.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [VitMatteImageProcessor](/docs/transformers/v5.14.0/en/model_doc/vitmatte#transformers.VitMatteImageProcessor). See `VitMatteImageProcessor.__call__()` for details (`processor_class` uses
  [VitMatteImageProcessor](/docs/transformers/v5.14.0/en/model_doc/vitmatte#transformers.VitMatteImageProcessor) for processing images).
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **labels** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Ground truth image matting for computing the loss.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.
The [VitMatteForImageMatting](/docs/transformers/v5.14.0/en/model_doc/vitmatte#transformers.VitMatteForImageMatting) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Examples:

```python
>>> from transformers import VitMatteImageProcessor, VitMatteForImageMatting
>>> import torch
>>> from PIL import Image
>>> from huggingface_hub import hf_hub_download

>>> processor = VitMatteImageProcessor.from_pretrained("hustvl/vitmatte-small-composition-1k")
>>> model = VitMatteForImageMatting.from_pretrained("hustvl/vitmatte-small-composition-1k")

>>> filepath = hf_hub_download(
...     repo_id="hf-internal-testing/image-matting-fixtures", filename="image.png", repo_type="dataset"
... )
>>> image = Image.open(filepath).convert("RGB")
>>> filepath = hf_hub_download(
...     repo_id="hf-internal-testing/image-matting-fixtures", filename="trimap.png", repo_type="dataset"
... )
>>> trimap = Image.open(filepath).convert("L")

>>> # prepare image + trimap for the model
>>> inputs = processor(images=image, trimaps=trimap, return_tensors="pt")

>>> with torch.no_grad():
...     alphas = model(**inputs).alphas
>>> print(alphas.shape)
torch.Size([1, 1, 640, 960])
```
