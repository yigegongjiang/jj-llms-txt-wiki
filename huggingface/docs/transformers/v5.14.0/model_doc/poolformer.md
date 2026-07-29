# PoolFormer

## Overview

The PoolFormer model was proposed in [MetaFormer is Actually What You Need for Vision](https://huggingface.co/papers/2111.11418)  by Sea AI Labs. Instead of designing complicated token mixer to achieve SOTA performance, the target of this work is to demonstrate the competence of transformer models largely stem from the general architecture MetaFormer.

The abstract from the paper is the following:

*Transformers have shown great potential in computer vision tasks. A common belief is their attention-based token mixer module contributes most to their competence. However, recent works show the attention-based module in transformers can be replaced by spatial MLPs and the resulted models still perform quite well. Based on this observation, we hypothesize that the general architecture of the transformers, instead of the specific token mixer module, is more essential to the model's performance. To verify this, we deliberately replace the attention module in transformers with an embarrassingly simple spatial pooling operator to conduct only the most basic token mixing. Surprisingly, we observe that the derived model, termed as PoolFormer, achieves competitive performance on multiple computer vision tasks. For example, on ImageNet-1K, PoolFormer achieves 82.1% top-1 accuracy, surpassing well-tuned vision transformer/MLP-like baselines DeiT-B/ResMLP-B24 by 0.3%/1.1% accuracy with 35%/52% fewer parameters and 48%/60% fewer MACs. The effectiveness of PoolFormer verifies our hypothesis and urges us to initiate the concept of "MetaFormer", a general architecture abstracted from transformers without specifying the token mixer. Based on the extensive experiments, we argue that MetaFormer is the key player in achieving superior results for recent transformer and MLP-like models on vision tasks. This work calls for more future research dedicated to improving MetaFormer instead of focusing on the token mixer modules. Additionally, our proposed PoolFormer could serve as a starting baseline for future MetaFormer architecture design.*

The figure below illustrates the architecture of PoolFormer. Taken from the [original paper](https://huggingface.co/papers/2111.11418).

This model was contributed by [heytanay](https://huggingface.co/heytanay). The original code can be found [here](https://github.com/sail-sg/poolformer).

## Usage tips

- PoolFormer has a hierarchical architecture, where instead of Attention, a simple Average Pooling layer is present. All checkpoints of the model can be found on the [hub](https://huggingface.co/models?other=poolformer).
- One can use [PoolFormerImageProcessor](/docs/transformers/v5.14.0/en/model_doc/poolformer#transformers.PoolFormerImageProcessor) to prepare images for the model.
- As most models, PoolFormer comes in different sizes, the details of which can be found in the table below.

| **Model variant** | **Depths**    | **Hidden sizes**    | **Params (M)** | **ImageNet-1k Top 1** |
| :---------------: | ------------- | ------------------- | :------------: | :-------------------: |
| s12               | [2, 2, 6, 2]  | [64, 128, 320, 512] | 12             | 77.2                  |
| s24               | [4, 4, 12, 4] | [64, 128, 320, 512] | 21             | 80.3                  |
| s36               | [6, 6, 18, 6] | [64, 128, 320, 512] | 31             | 81.4                  |
| m36               | [6, 6, 18, 6] | [96, 192, 384, 768] | 56             | 82.1                  |
| m48               | [8, 8, 24, 8] | [96, 192, 384, 768] | 73             | 82.5                  |

## Resources

A list of official Hugging Face and community (indicated by 🌎) resources to help you get started with PoolFormer.

- [PoolFormerForImageClassification](/docs/transformers/v5.14.0/en/model_doc/poolformer#transformers.PoolFormerForImageClassification) is supported by this [example script](https://github.com/huggingface/transformers/tree/main/examples/pytorch/image-classification) and [notebook](https://colab.research.google.com/github/huggingface/notebooks/blob/main/examples/image_classification.ipynb).
- See also: [Image classification task guide](../tasks/image_classification)

If you're interested in submitting a resource to be included here, please feel free to open a Pull Request and we'll review it! The resource should ideally demonstrate something new instead of duplicating an existing resource.

## PoolFormerConfig[[transformers.PoolFormerConfig]]

- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) --
  The size (resolution) of each patch.
- **stride** (`int`, *optional*, defaults to 16) --
  The stride of the input patch.
- **pool_size** (`int`, *optional*, defaults to 3) --
  The size of the pooling window.
- **mlp_ratio** (`float`, *optional*, defaults to `4.0`) --
  Ratio of the MLP hidden dim to the embedding dim.
- **depths** (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(2, 2, 6, 2)`) --
  Depth of each layer in the Transformer.
- **hidden_sizes** (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(64, 128, 320, 512)`) --
  Dimensionality (hidden size) at each stage of the model.
- **patch_sizes** (`list`, *optional*, defaults to `[7, 3, 3, 3]`) --
  The size of the input patch for each encoder block.
- **strides** (`list`, *optional*, defaults to `[4, 2, 2, 2]`) --
  The stride of the input patch for each encoder block.
- **padding** (`list`, *optional*, defaults to `[2, 1, 1, 1]`) --
  The padding of the input patch for each encoder block.
- **num_encoder_blocks** (`int`, *optional*, defaults to 4) --
  The number of encoder blocks.
- **drop_path_rate** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  Drop path rate for the patch fusion.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **use_layer_scale** (`bool`, *optional*, defaults to `True`) --
  Whether to use layer scale.
- **layer_scale_init_value** (`float`, *optional*, defaults to `1e-05`) --
  Scale to use in the self-attention layers. 0.1 for base, 1e-6 for large. Set 0 to disable layer scale.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a PoolFormerModel. It is used to instantiate a Poolformer
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [sail/poolformer_s12](https://huggingface.co/sail/poolformer_s12)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import PoolFormerConfig, PoolFormerModel

>>> # Initializing a PoolFormer sail/poolformer_s12 style configuration
>>> configuration = PoolFormerConfig()

>>> # Initializing a model (with random weights) from the sail/poolformer_s12 style configuration
>>> model = PoolFormerModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## PoolFormerImageProcessor[[transformers.PoolFormerImageProcessor]]

- **crop_pct** (`float`, *kwargs*, *optional*, defaults to `self.crop_pct`) --
  Percentage of the image to crop. Only has an effect if `do_resize` is set to `True`.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a PoolFormerImageProcessor image processor.

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

## PoolFormerImageProcessorPil[[transformers.PoolFormerImageProcessorPil]]

- **crop_pct** (`float`, *kwargs*, *optional*, defaults to `self.crop_pct`) --
  Percentage of the image to crop. Only has an effect if `do_resize` is set to `True`.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a PoolFormerImageProcessor image processor.

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

## PoolFormerModel[[transformers.PoolFormerModel]]

- **config** ([PoolFormerModel](/docs/transformers/v5.14.0/en/model_doc/poolformer#transformers.PoolFormerModel)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Poolformer Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [PoolFormerImageProcessor](/docs/transformers/v5.14.0/en/model_doc/poolformer#transformers.PoolFormerImageProcessor). See `PoolFormerImageProcessor.__call__()` for details (`processor_class` uses
  [PoolFormerImageProcessor](/docs/transformers/v5.14.0/en/model_doc/poolformer#transformers.PoolFormerImageProcessor) for processing images).
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.`BaseModelOutputWithNoAttention` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithNoAttention` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PoolFormerConfig](/docs/transformers/v5.14.0/en/model_doc/poolformer#transformers.PoolFormerConfig)) and inputs.
The [PoolFormerModel](/docs/transformers/v5.14.0/en/model_doc/poolformer#transformers.PoolFormerModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, num_channels, height, width)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

Example:

```python
```

## PoolFormerForImageClassification[[transformers.PoolFormerForImageClassification]]

- **config** ([PoolFormerForImageClassification](/docs/transformers/v5.14.0/en/model_doc/poolformer#transformers.PoolFormerForImageClassification)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

PoolFormer Model transformer with an image classification head on top

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [PoolFormerImageProcessor](/docs/transformers/v5.14.0/en/model_doc/poolformer#transformers.PoolFormerImageProcessor). See `PoolFormerImageProcessor.__call__()` for details (`processor_class` uses
  [PoolFormerImageProcessor](/docs/transformers/v5.14.0/en/model_doc/poolformer#transformers.PoolFormerImageProcessor) for processing images).
- **labels** (`torch.LongTensor` of shape `(batch_size,)`, *optional*) --
  Labels for computing the image classification/regression loss. Indices should be in `[0, ...,
  config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If
  `config.num_labels > 1` a classification loss is computed (Cross-Entropy).
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.[ImageClassifierOutputWithNoAttention](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutputWithNoAttention) or `tuple(torch.FloatTensor)`A [ImageClassifierOutputWithNoAttention](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutputWithNoAttention) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PoolFormerConfig](/docs/transformers/v5.14.0/en/model_doc/poolformer#transformers.PoolFormerConfig)) and inputs.
The [PoolFormerForImageClassification](/docs/transformers/v5.14.0/en/model_doc/poolformer#transformers.PoolFormerForImageClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Classification (or regression if config.num_labels==1) scores (before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each stage) of shape `(batch_size, num_channels, height, width)`. Hidden-states (also
  called feature maps) of the model at the output of each stage.

Example:

```python
>>> from transformers import AutoImageProcessor, PoolFormerForImageClassification
>>> import torch
>>> from datasets import load_dataset

>>> dataset = load_dataset("huggingface/cats-image")
>>> image = dataset["test"]["image"][0]

>>> image_processor = AutoImageProcessor.from_pretrained("sail/poolformer_s12")
>>> model = PoolFormerForImageClassification.from_pretrained("sail/poolformer_s12")

>>> inputs = image_processor(image, return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # model predicts one of the 1000 ImageNet classes
>>> predicted_label = logits.argmax(-1).item()
>>> print(model.config.id2label[predicted_label])
...
```
