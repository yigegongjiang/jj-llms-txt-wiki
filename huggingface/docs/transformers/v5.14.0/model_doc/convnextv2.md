# ConvNeXt V2

## Overview

The ConvNeXt V2 model was proposed in [ConvNeXt V2: Co-designing and Scaling ConvNets with Masked Autoencoders](https://huggingface.co/papers/2301.00808) by Sanghyun Woo, Shoubhik Debnath, Ronghang Hu, Xinlei Chen, Zhuang Liu, In So Kweon, Saining Xie.
ConvNeXt V2 is a pure convolutional model (ConvNet), inspired by the design of Vision Transformers, and a successor of [ConvNeXT](convnext).

The abstract from the paper is the following:

*Driven by improved architectures and better representation learning frameworks, the field of visual recognition has enjoyed rapid modernization and performance boost in the early 2020s. For example, modern ConvNets, represented by ConvNeXt, have demonstrated strong performance in various scenarios. While these models were originally designed for supervised learning with ImageNet labels, they can also potentially benefit from self-supervised learning techniques such as masked  autoencoders (MAE). However, we found that simply combining these two approaches leads to subpar performance. In this paper, we propose a fully convolutional masked autoencoder framework and a new Global Response Normalization (GRN) layer that can be added to the ConvNeXt architecture to enhance inter-channel feature competition. This co-design of self-supervised learning techniques and architectural improvement results in a new model family called ConvNeXt V2, which significantly improves the performance of pure ConvNets on various recognition benchmarks, including ImageNet classification, COCO detection, and ADE20K segmentation. We also provide pre-trained ConvNeXt V2 models of various sizes, ranging from an efficient 3.7M-parameter Atto model with 76.7% top-1 accuracy on ImageNet, to a 650M Huge model that achieves a state-of-the-art 88.9% accuracy using only public training data.*

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/convnextv2_architecture.png"
alt="drawing" width="600"/>

 ConvNeXt V2 architecture. Taken from the original paper.

This model was contributed by [adirik](https://huggingface.co/adirik). The original code can be found [here](https://github.com/facebookresearch/ConvNeXt-V2).

## Resources

A list of official Hugging Face and community (indicated by 🌎) resources to help you get started with ConvNeXt V2.

- [ConvNextV2ForImageClassification](/docs/transformers/v5.14.0/en/model_doc/convnextv2#transformers.ConvNextV2ForImageClassification) is supported by this [example script](https://github.com/huggingface/transformers/tree/main/examples/pytorch/image-classification) and [notebook](https://colab.research.google.com/github/huggingface/notebooks/blob/main/examples/image_classification.ipynb).

If you're interested in submitting a resource to be included here, please feel free to open a Pull Request and we'll review it! The resource should ideally demonstrate something new instead of duplicating an existing resource.

## ConvNextV2Config[[transformers.ConvNextV2Config]]

- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `4`) --
  The size (resolution) of each patch.
- **num_stages** (`int`, *optional*, defaults to 4) --
  The number of stages in the model.
- **hidden_sizes** (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(96, 192, 384, 768)`) --
  Dimensionality (hidden size) at each stage of the model.
- **depths** (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(3, 3, 9, 3)`) --
  Depth of each layer in the Transformer.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-12`) --
  The epsilon used by the layer normalization layers.
- **drop_path_rate** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  Drop path rate for the patch fusion.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `224`) --
  The size (resolution) of each image.

This is the configuration class to store the configuration of a ConvNextV2Model. It is used to instantiate a Convnextv2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/convnextv2-tiny-1k-224](https://huggingface.co/facebook/convnextv2-tiny-1k-224)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import ConvNeXTV2Config, ConvNextV2Model

>>> # Initializing a ConvNeXTV2 convnextv2-tiny-1k-224 style configuration
>>> configuration = ConvNeXTV2Config()

>>> # Initializing a model (with random weights) from the convnextv2-tiny-1k-224 style configuration
>>> model = ConvNextV2Model(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## ConvNextV2Model[[transformers.ConvNextV2Model]]

- **config** ([ConvNextV2Model](/docs/transformers/v5.14.0/en/model_doc/convnextv2#transformers.ConvNextV2Model)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Convnextv2 Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [ConvNextImageProcessor](/docs/transformers/v5.14.0/en/model_doc/convnext#transformers.ConvNextImageProcessor). See `ConvNextImageProcessor.__call__()` for details (`processor_class` uses
  [ConvNextImageProcessor](/docs/transformers/v5.14.0/en/model_doc/convnext#transformers.ConvNextImageProcessor) for processing images).`BaseModelOutputWithPoolingAndNoAttention` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithPoolingAndNoAttention` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ConvNextV2Config](/docs/transformers/v5.14.0/en/model_doc/convnextv2#transformers.ConvNextV2Config)) and inputs.
The [ConvNextV2Model](/docs/transformers/v5.14.0/en/model_doc/convnextv2#transformers.ConvNextV2Model) forward method, overrides the `__call__` special method.

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

## ConvNextV2ForImageClassification[[transformers.ConvNextV2ForImageClassification]]

- **config** ([ConvNextV2ForImageClassification](/docs/transformers/v5.14.0/en/model_doc/convnextv2#transformers.ConvNextV2ForImageClassification)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

ConvNextV2 Model with an image classification head on top (a linear layer on top of the pooled features), e.g. for
ImageNet.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [ConvNextImageProcessor](/docs/transformers/v5.14.0/en/model_doc/convnext#transformers.ConvNextImageProcessor). See `ConvNextImageProcessor.__call__()` for details (`processor_class` uses
  [ConvNextImageProcessor](/docs/transformers/v5.14.0/en/model_doc/convnext#transformers.ConvNextImageProcessor) for processing images).
- **labels** (`torch.LongTensor` of shape `(batch_size,)`, *optional*) --
  Labels for computing the image classification/regression loss. Indices should be in `[0, ...,
  config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If
  `config.num_labels > 1` a classification loss is computed (Cross-Entropy).[ImageClassifierOutputWithNoAttention](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutputWithNoAttention) or `tuple(torch.FloatTensor)`A [ImageClassifierOutputWithNoAttention](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutputWithNoAttention) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ConvNextV2Config](/docs/transformers/v5.14.0/en/model_doc/convnextv2#transformers.ConvNextV2Config)) and inputs.
The [ConvNextV2ForImageClassification](/docs/transformers/v5.14.0/en/model_doc/convnextv2#transformers.ConvNextV2ForImageClassification) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoImageProcessor, ConvNextV2ForImageClassification
>>> import torch
>>> from datasets import load_dataset

>>> dataset = load_dataset("huggingface/cats-image")
>>> image = dataset["test"]["image"][0]

>>> image_processor = AutoImageProcessor.from_pretrained("facebook/convnextv2-tiny-1k-224")
>>> model = ConvNextV2ForImageClassification.from_pretrained("facebook/convnextv2-tiny-1k-224")

>>> inputs = image_processor(image, return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # model predicts one of the 1000 ImageNet classes
>>> predicted_label = logits.argmax(-1).item()
>>> print(model.config.id2label[predicted_label])
...
```
