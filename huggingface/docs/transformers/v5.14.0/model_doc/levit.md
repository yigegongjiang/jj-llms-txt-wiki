# LeViT

## Overview

The LeViT model was proposed in [LeViT: Introducing Convolutions to Vision Transformers](https://huggingface.co/papers/2104.01136) by Ben Graham, Alaaeldin El-Nouby, Hugo Touvron, Pierre Stock, Armand Joulin, Hervé Jégou, Matthijs Douze. LeViT improves the [Vision Transformer (ViT)](vit) in performance and efficiency by a few architectural differences such as activation maps with decreasing resolutions in Transformers and the introduction of an attention bias to integrate positional information.

The abstract from the paper is the following:

*We design a family of image classification architectures that optimize the trade-off between accuracy
and efficiency in a high-speed regime. Our work exploits recent findings in attention-based architectures,
which are competitive on highly parallel processing hardware. We revisit principles from the extensive
literature on convolutional neural networks to apply them to transformers, in particular activation maps
with decreasing resolutions. We also introduce the attention bias, a new way to integrate positional information
in vision transformers. As a result, we propose LeVIT: a hybrid neural network for fast inference image classification.
We consider different measures of efficiency on different hardware platforms, so as to best reflect a wide range of
application scenarios. Our extensive experiments empirically validate our technical choices and show they are suitable
to most architectures. Overall, LeViT significantly outperforms existing convnets and vision transformers with respect
to the speed/accuracy tradeoff. For example, at 80% ImageNet top-1 accuracy, LeViT is 5 times faster than EfficientNet on CPU.*

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/levit_architecture.png"
alt="drawing" width="600"/>

 LeViT Architecture. Taken from the original paper.

This model was contributed by [anugunj](https://huggingface.co/anugunj). The original code can be found [here](https://github.com/facebookresearch/LeViT).

## Usage tips

- Compared to ViT, LeViT models use an additional distillation head to effectively learn from a teacher (which, in the LeViT paper, is a ResNet like-model). The distillation head is learned through backpropagation under supervision of a ResNet like-model. They also draw inspiration from convolution neural networks to use activation maps with decreasing resolutions to increase the efficiency.
- There are 2 ways to fine-tune distilled models, either (1) in a classic way, by only placing a prediction head on top
  of the final hidden state and not using the distillation head, or (2) by placing both a prediction head and distillation
  head on top of the final hidden state. In that case, the prediction head is trained using regular cross-entropy between
  the prediction of the head and the ground-truth label, while the distillation prediction head is trained using hard distillation
  (cross-entropy between the prediction of the distillation head and the label predicted by the teacher). At inference time,
  one takes the average prediction between both heads as final prediction. (2) is also called "fine-tuning with distillation",
  because one relies on a teacher that has already been fine-tuned on the downstream dataset. In terms of models, (1) corresponds
  to [LevitForImageClassification](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitForImageClassification) and (2) corresponds to [LevitForImageClassificationWithTeacher](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitForImageClassificationWithTeacher).
- All released checkpoints were pre-trained and fine-tuned on  [ImageNet-1k](https://huggingface.co/datasets/ILSVRC/imagenet-1k)
  (also referred to as ILSVRC 2012, a collection of 1.3 million images and 1,000 classes). only. No external data was used. This is in
  contrast with the original ViT model, which used external data like the JFT-300M dataset/Imagenet-21k for
  pre-training.
- The authors of LeViT released 5 trained LeViT models, which you can directly plug into [LevitModel](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitModel) or [LevitForImageClassification](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitForImageClassification).
  Techniques like data augmentation, optimization, and regularization were used in order to simulate training on a much larger dataset
  (while only using ImageNet-1k for pre-training). The 5 variants available are (all trained on images of size 224x224):
  *facebook/levit-128S*, *facebook/levit-128*, *facebook/levit-192*, *facebook/levit-256* and
  *facebook/levit-384*. Note that one should use [LevitImageProcessor](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitImageProcessor) in order to
  prepare images for the model.
- [LevitForImageClassificationWithTeacher](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitForImageClassificationWithTeacher) currently supports only inference and not training or fine-tuning.
- You can check out demo notebooks regarding inference as well as fine-tuning on custom data [here](https://github.com/NielsRogge/Transformers-Tutorials/tree/master/VisionTransformer)
  (you can just replace [ViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTImageProcessor) by [LevitImageProcessor](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitImageProcessor) and [ViTForImageClassification](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTForImageClassification) by [LevitForImageClassification](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitForImageClassification) or [LevitForImageClassificationWithTeacher](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitForImageClassificationWithTeacher)).

## Resources

A list of official Hugging Face and community (indicated by 🌎) resources to help you get started with LeViT.

- [LevitForImageClassification](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitForImageClassification) is supported by this [example script](https://github.com/huggingface/transformers/tree/main/examples/pytorch/image-classification) and [notebook](https://colab.research.google.com/github/huggingface/notebooks/blob/main/examples/image_classification.ipynb).
- See also: [Image classification task guide](../tasks/image_classification)

If you're interested in submitting a resource to be included here, please feel free to open a Pull Request and we'll review it! The resource should ideally demonstrate something new instead of duplicating an existing resource.

## LevitConfig[[transformers.LevitConfig]]

- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `224`) --
  The size (resolution) of each image.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **kernel_size** (`int`, *optional*, defaults to `3`) --
  The size of the convolutional kernel.
- **stride** (`int`, *optional*, defaults to 2) --
  The stride size for the initial convolution layers of patch embedding.
- **padding** (`int`, *optional*, defaults to 1) --
  The padding size for the initial convolution layers of patch embedding.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) --
  The size (resolution) of each patch.
- **hidden_sizes** (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(128, 256, 384)`) --
  Dimensionality (hidden size) at each stage of the model.
- **num_attention_heads** (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(4, 8, 12)`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **depths** (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(4, 4, 4)`) --
  Depth of each layer in the Transformer.
- **key_dim** (`list[int]`, *optional*, defaults to `[16, 16, 16]`) --
  The size of key in each of the encoder blocks.
- **drop_path_rate** (`int`, *optional*, defaults to `0`) --
  Drop path rate for the patch fusion.
- **mlp_ratio** (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(2, 2, 2)`) --
  Ratio of the MLP hidden dim to the embedding dim.
- **attention_ratio** (`list[int]`, *optional*, defaults to `[2, 2, 2]`) --
  Ratio of the size of the output dimension compared to input dimension of attention layers.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a LevitModel. It is used to instantiate a Levit
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/levit-128S](https://huggingface.co/facebook/levit-128S)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import LevitConfig, LevitModel

>>> # Initializing a LeViT levit-128S style configuration
>>> configuration = LevitConfig()

>>> # Initializing a model (with random weights) from the levit-128S style configuration
>>> model = LevitModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## LevitImageProcessor[[transformers.LevitImageProcessor]]

- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a LevitImageProcessor image processor.

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

## LevitImageProcessorPil[[transformers.LevitImageProcessorPil]]

- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a LevitImageProcessor image processor.

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

## LevitModel[[transformers.LevitModel]]

- **config** ([LevitModel](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitModel)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Levit Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [LevitImageProcessor](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitImageProcessor). See `LevitImageProcessor.__call__()` for details (`processor_class` uses
  [LevitImageProcessor](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitImageProcessor) for processing images).
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.`BaseModelOutputWithPoolingAndNoAttention` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithPoolingAndNoAttention` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LevitConfig](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitConfig)) and inputs.
The [LevitModel](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Last layer hidden-state after a pooling operation on the spatial dimensions.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, num_channels, height, width)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

Example:

```python
```

## LevitForImageClassification[[transformers.LevitForImageClassification]]

- **config** ([LevitForImageClassification](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitForImageClassification)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Levit Model with an image classification head on top (a linear layer on top of the pooled features), e.g. for
ImageNet.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [LevitImageProcessor](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitImageProcessor). See `LevitImageProcessor.__call__()` for details (`processor_class` uses
  [LevitImageProcessor](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitImageProcessor) for processing images).
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
elements depending on the configuration ([LevitConfig](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitConfig)) and inputs.
The [LevitForImageClassification](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitForImageClassification) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoImageProcessor, LevitForImageClassification
>>> import torch
>>> from datasets import load_dataset

>>> dataset = load_dataset("huggingface/cats-image")
>>> image = dataset["test"]["image"][0]

>>> image_processor = AutoImageProcessor.from_pretrained("facebook/levit-128S")
>>> model = LevitForImageClassification.from_pretrained("facebook/levit-128S")

>>> inputs = image_processor(image, return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # model predicts one of the 1000 ImageNet classes
>>> predicted_label = logits.argmax(-1).item()
>>> print(model.config.id2label[predicted_label])
...
```

## LevitForImageClassificationWithTeacher[[transformers.LevitForImageClassificationWithTeacher]]

- **config** ([LevitForImageClassificationWithTeacher](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitForImageClassificationWithTeacher)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

LeViT Model transformer with image classification heads on top (a linear layer on top of the final hidden state and
a linear layer on top of the final hidden state of the distillation token) e.g. for ImageNet. .. warning::
This model supports inference-only. Fine-tuning with distillation (i.e. with a teacher) is not yet
supported.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [LevitImageProcessor](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitImageProcessor). See `LevitImageProcessor.__call__()` for details (`processor_class` uses
  [LevitImageProcessor](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitImageProcessor) for processing images).
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.`LevitForImageClassificationWithTeacherOutput` or `tuple(torch.FloatTensor)`A `LevitForImageClassificationWithTeacherOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LevitConfig](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitConfig)) and inputs.
The [LevitForImageClassificationWithTeacher](/docs/transformers/v5.14.0/en/model_doc/levit#transformers.LevitForImageClassificationWithTeacher) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Prediction scores as the average of the `cls_logits` and `distillation_logits`.
- **cls_logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Prediction scores of the classification head (i.e. the linear layer on top of the final hidden state of the
  class token).
- **distillation_logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Prediction scores of the distillation head (i.e. the linear layer on top of the final hidden state of the
  distillation token).
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

Example:

```python
>>> from transformers import AutoImageProcessor, LevitForImageClassificationWithTeacher
>>> import torch
>>> from datasets import load_dataset

>>> dataset = load_dataset("huggingface/cats-image")
>>> image = dataset["test"]["image"][0]

>>> image_processor = AutoImageProcessor.from_pretrained("facebook/levit-128S")
>>> model = LevitForImageClassificationWithTeacher.from_pretrained("facebook/levit-128S")

>>> inputs = image_processor(image, return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # model predicts one of the 1000 ImageNet classes
>>> predicted_label = logits.argmax(-1).item()
>>> print(model.config.id2label[predicted_label])
...
```
