# SegFormer

[SegFormer: Simple and Efficient Design for Semantic Segmentation with Transformers](https://huggingface.co/papers/2105.15203) is a semantic segmentation model that combines a hierarchical Transformer encoder (Mix Transformer, MiT) with a lightweight all-MLP decoder. It avoids positional encodings and complex decoders and achieves state-of-the-art performance on benchmarks like ADE20K and Cityscapes. This simple and lightweight design is more efficient and scalable.

The figure below illustrates the architecture of SegFormer.

You can find all the original SegFormer checkpoints under the [NVIDIA](https://huggingface.co/nvidia/models?search=segformer) organization.

> [!TIP]
> This model was contributed by [nielsr](https://huggingface.co/nielsr).
>
> Click on the SegFormer models in the right sidebar for more examples of how to apply SegFormer to different vision tasks.

The example below demonstrates semantic segmentation with [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline

pipeline = pipeline(task="image-segmentation", model="nvidia/segformer-b0-finetuned-ade-512-512")
pipeline("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg")
```

```python
import requests
from PIL import Image

from transformers import AutoModelForSemanticSegmentation, AutoProcessor

url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
image = Image.open(requests.get(url, stream=True).raw)

processor = AutoProcessor.from_pretrained("nvidia/segformer-b0-finetuned-ade-512-512")
model = AutoModelForSemanticSegmentation.from_pretrained("nvidia/segformer-b0-finetuned-ade-512-512", device_map="auto")

inputs = processor(images=image, return_tensors="pt").to(model.device)
outputs = model(**inputs)
logits = outputs.logits # shape [batch, num_labels, height, width]
```

## Notes

- SegFormer works with **any input size**, padding inputs to be divisible by `config.patch_sizes`.
- The most important preprocessing step is to randomly crop and pad all images to the same size (such as 512x512 or 640x640) and normalize afterwards.
- Some datasets (ADE20k) uses the `0` index in the annotated segmentation as the background, but doesn't include the "background" class in its labels. The `do_reduce_labels` argument in `SegformerForImageProcessor` is used to reduce all labels by `1`. To make sure no loss is computed for the background class, it replaces `0` in the annotated maps by `255`, which is the `ignore_index` of the loss function.

   Other datasets may include a background class and label though, in which case, `do_reduce_labels` should be `False`.

```python
from transformers import SegformerImageProcessor

processor = SegformerImageProcessor(do_reduce_labels=True)
```

## Resources

- [Original SegFormer code (NVlabs)](https://github.com/NVlabs/SegFormer)  
- [Fine-tuning blog post](https://huggingface.co/blog/fine-tune-segformer)  
- [Tutorial notebooks (Niels Rogge)](https://github.com/NielsRogge/Transformers-Tutorials/tree/master/SegFormer)  
- [Hugging Face demo space](https://huggingface.co/spaces/chansung/segformer-tf-transformers)  

## SegformerConfig[[transformers.SegformerConfig]]

- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **num_encoder_blocks** (`int`, *optional*, defaults to 4) --
  The number of encoder blocks (i.e. stages in the Mix Transformer encoder).
- **depths** (`list[int]`, *optional*, defaults to `[2, 2, 2, 2]`) --
  The number of layers in each encoder block.
- **sr_ratios** (`list[int]`, *optional*, defaults to `[8, 4, 2, 1]`) --
  Sequence reduction ratios in each encoder block.
- **hidden_sizes** (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(32, 64, 160, 256)`) --
  Dimensionality (hidden size) at each stage of the model.
- **patch_sizes** (`list[int]`, *optional*, defaults to `[7, 3, 3, 3]`) --
  Patch size before each encoder block.
- **strides** (`list[int]`, *optional*, defaults to `[4, 2, 2, 2]`) --
  Stride before each encoder block.
- **num_attention_heads** (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(1, 2, 5, 8)`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **mlp_ratios** (`list[int]`, *optional*, defaults to `[4, 4, 4, 4]`) --
  Ratio of the size of the hidden layer compared to the size of the input layer of the Mix FFNs in the
  encoder blocks.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **hidden_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **attention_probs_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **classifier_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout ratio for classifier.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **drop_path_rate** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  Drop path rate for the patch fusion.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **decoder_hidden_size** (`int`, *optional*, defaults to `256`) --
  Dimension of the hidden representations.
- **semantic_loss_ignore_index** (`int`, *optional*, defaults to `255`) --
  The index that is ignored by the loss function of the semantic segmentation model.
- **reshape_last_stage** (`bool`, *optional*, defaults to True) --
  Whether to reshape the last stage outputs

This is the configuration class to store the configuration of a SegformerModel. It is used to instantiate a Segformer
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [ByteDance-Seed/Seed-OSS-36B-Instruct](https://huggingface.co/ByteDance-Seed/Seed-OSS-36B-Instruct)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import SegformerModel, SegformerConfig

>>> # Initializing a SegFormer nvidia/segformer-b0-finetuned-ade-512-512 style configuration
>>> configuration = SegformerConfig()

>>> # Initializing a model from the nvidia/segformer-b0-finetuned-ade-512-512 style configuration
>>> model = SegformerModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## SegformerImageProcessor[[transformers.SegformerImageProcessor]]

- **do_reduce_labels** (`bool`, *kwargs*, *optional*, defaults to `self.do_reduce_labels`) --
  Whether or not to reduce all label values of segmentation maps by 1. Usually used for datasets where 0
  is used for background, and background itself is not included in all classes of a dataset (e.g.
  ADE20k). The background label will be replaced by 255.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a SegformerImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **segmentation_maps** (`ImageInput`, *optional*) --
  The segmentation maps to preprocess.
- **do_reduce_labels** (`bool`, *kwargs*, *optional*, defaults to `self.do_reduce_labels`) --
  Whether or not to reduce all label values of segmentation maps by 1. Usually used for datasets where 0
  is used for background, and background itself is not included in all classes of a dataset (e.g.
  ADE20k). The background label will be replaced by 255.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** ([SegformerForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/segformer#transformers.SegformerForSemanticSegmentation)) --
  Raw outputs of the model.
- **target_sizes** (`list[Tuple]` of length `batch_size`, *optional*) --
  List of tuples corresponding to the requested final size (height, width) of each prediction. If unset,
  predictions will not be resized.
- **return_segmentation_scores** (`bool`, *optional*, defaults to `False`) --
  Whether to return segmentation scores alongside the segmentation map. When `True`, each element of
  the returned list is a `SemanticSegmentationPostProcessorOutput` with fields `segmentation`
  (class IDs, shape `(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`).`list[torch.Tensor]` or `list[SemanticSegmentationPostProcessorOutput]`When
`return_segmentation_scores=False` (default), a list of length `batch_size` where each item is a
segmentation map of shape `(height, width)` with class IDs. When `return_segmentation_scores=True`,
a list of `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape
`(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`). In both cases,
`(height, width)` corresponds to the target size (if `target_sizes` is specified).

Converts the output of [SegformerForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/segformer#transformers.SegformerForSemanticSegmentation) into semantic segmentation maps.

## SegformerImageProcessorPil[[transformers.SegformerImageProcessorPil]]

PIL backend for Segformer with reduce_label support.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **segmentation_maps** (`ImageInput`, *optional*) --
  The segmentation maps to preprocess.
- **do_reduce_labels** (`bool`, *kwargs*, *optional*, defaults to `self.do_reduce_labels`) --
  Whether or not to reduce all label values of segmentation maps by 1. Usually used for datasets where 0
  is used for background, and background itself is not included in all classes of a dataset (e.g.
  ADE20k). The background label will be replaced by 255.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** ([SegformerForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/segformer#transformers.SegformerForSemanticSegmentation)) --
  Raw outputs of the model.
- **target_sizes** (`list[Tuple]` of length `batch_size`, *optional*) --
  List of tuples corresponding to the requested final size (height, width) of each prediction. If unset,
  predictions will not be resized.
- **return_segmentation_scores** (`bool`, *optional*, defaults to `False`) --
  Whether to return segmentation scores alongside the segmentation map. When `True`, each element of
  the returned list is a `SemanticSegmentationPostProcessorOutput` with fields `segmentation`
  (class IDs, shape `(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`).`list[torch.Tensor]` or `list[SemanticSegmentationPostProcessorOutput]`When
`return_segmentation_scores=False` (default), a list of length `batch_size` where each item is a
segmentation map of shape `(height, width)` with class IDs. When `return_segmentation_scores=True`,
a list of `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape
`(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`). In both cases,
`(height, width)` corresponds to the target size (if `target_sizes` is specified).

Converts the output of [SegformerForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/segformer#transformers.SegformerForSemanticSegmentation) into semantic segmentation maps.

## SegformerModel[[transformers.SegformerModel]]

- **config** ([SegformerModel](/docs/transformers/v5.14.0/en/model_doc/segformer#transformers.SegformerModel)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Segformer Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [SegformerImageProcessor](/docs/transformers/v5.14.0/en/model_doc/segformer#transformers.SegformerImageProcessor). See `SegformerImageProcessor.__call__()` for details (`processor_class` uses
  [SegformerImageProcessor](/docs/transformers/v5.14.0/en/model_doc/segformer#transformers.SegformerImageProcessor) for processing images).[BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or `tuple(torch.FloatTensor)`A [BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SegformerConfig](/docs/transformers/v5.14.0/en/model_doc/segformer#transformers.SegformerConfig)) and inputs.
The [SegformerModel](/docs/transformers/v5.14.0/en/model_doc/segformer#transformers.SegformerModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
```

## SegformerDecodeHead[[transformers.SegformerDecodeHead]]

## SegformerForImageClassification[[transformers.SegformerForImageClassification]]

- **config** ([SegformerForImageClassification](/docs/transformers/v5.14.0/en/model_doc/segformer#transformers.SegformerForImageClassification)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

SegFormer Model transformer with an image classification head on top (a linear layer on top of the final hidden
states) e.g. for ImageNet.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [SegformerImageProcessor](/docs/transformers/v5.14.0/en/model_doc/segformer#transformers.SegformerImageProcessor). See `SegformerImageProcessor.__call__()` for details (`processor_class` uses
  [SegformerImageProcessor](/docs/transformers/v5.14.0/en/model_doc/segformer#transformers.SegformerImageProcessor) for processing images).
- **labels** (`torch.LongTensor` of shape `(batch_size,)`, *optional*) --
  Labels for computing the image classification/regression loss. Indices should be in `[0, ...,
  config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If
  `config.num_labels > 1` a classification loss is computed (Cross-Entropy).`SegFormerImageClassifierOutput` or `tuple(torch.FloatTensor)`A `SegFormerImageClassifierOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SegformerConfig](/docs/transformers/v5.14.0/en/model_doc/segformer#transformers.SegformerConfig)) and inputs.
The [SegformerForImageClassification](/docs/transformers/v5.14.0/en/model_doc/segformer#transformers.SegformerForImageClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Classification (or regression if config.num_labels==1) scores (before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each stage) of shape `(batch_size, num_channels, height, width)`. Hidden-states (also
  called feature maps) of the model at the output of each stage.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, patch_size,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoImageProcessor, SegformerForImageClassification
>>> import torch
>>> from datasets import load_dataset

>>> dataset = load_dataset("huggingface/cats-image")
>>> image = dataset["test"]["image"][0]

>>> image_processor = AutoImageProcessor.from_pretrained("ByteDance-Seed/Seed-OSS-36B-Instruct")
>>> model = SegformerForImageClassification.from_pretrained("ByteDance-Seed/Seed-OSS-36B-Instruct")

>>> inputs = image_processor(image, return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # model predicts one of the 1000 ImageNet classes
>>> predicted_label = logits.argmax(-1).item()
>>> print(model.config.id2label[predicted_label])
...
```

## SegformerForSemanticSegmentation[[transformers.SegformerForSemanticSegmentation]]

- **config** ([SegformerForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/segformer#transformers.SegformerForSemanticSegmentation)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

SegFormer Model transformer with an all-MLP decode head on top e.g. for ADE20k, CityScapes.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [SegformerImageProcessor](/docs/transformers/v5.14.0/en/model_doc/segformer#transformers.SegformerImageProcessor). See `SegformerImageProcessor.__call__()` for details (`processor_class` uses
  [SegformerImageProcessor](/docs/transformers/v5.14.0/en/model_doc/segformer#transformers.SegformerImageProcessor) for processing images).
- **labels** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Ground truth semantic segmentation maps for computing the loss. Indices should be in `[0, ...,
  config.num_labels - 1]`. If `config.num_labels > 1`, a classification loss is computed (Cross-Entropy).</paramsdesc><rettype>[SemanticSegmenterOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.SemanticSegmenterOutput) or `tuple(torch.FloatTensor)`A [SemanticSegmenterOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.SemanticSegmenterOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SegformerConfig](/docs/transformers/v5.14.0/en/model_doc/segformer#transformers.SegformerConfig)) and inputs.
The [SegformerForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/segformer#transformers.SegformerForSemanticSegmentation) forward method, overrides the `__call__` special method.

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

Examples:

```python
>>> from transformers import AutoImageProcessor, SegformerForSemanticSegmentation
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> image_processor = AutoImageProcessor.from_pretrained("nvidia/segformer-b0-finetuned-ade-512-512")
>>> model = SegformerForSemanticSegmentation.from_pretrained("nvidia/segformer-b0-finetuned-ade-512-512")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> inputs = image_processor(images=image, return_tensors="pt")
>>> outputs = model(**inputs)
>>> logits = outputs.logits  # shape (batch_size, num_labels, height/4, width/4)
>>> list(logits.shape)
[1, 150, 128, 128]
```
