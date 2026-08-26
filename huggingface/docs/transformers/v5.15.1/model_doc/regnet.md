# RegNet

## Overview

The RegNet model was proposed in [Designing Network Design Spaces](https://huggingface.co/papers/2003.13678) by Ilija Radosavovic, Raj Prateek Kosaraju, Ross Girshick, Kaiming He, Piotr Dollár.

The authors design search spaces to perform Neural Architecture Search (NAS). They first start from a high dimensional search space and iteratively reduce the search space by empirically applying constraints based on the best-performing models sampled by the current search space.

The abstract from the paper is the following:

*In this work, we present a new network design paradigm. Our goal is to help advance the understanding of network design and discover design principles that generalize across settings. Instead of focusing on designing individual network instances, we design network design spaces that parametrize populations of networks. The overall process is analogous to classic manual design of networks, but elevated to the design space level. Using our methodology we explore the structure aspect of network design and arrive at a low-dimensional design space consisting of simple, regular networks that we call RegNet. The core insight of the RegNet parametrization is surprisingly simple: widths and depths of good networks can be explained by a quantized linear function. We analyze the RegNet design space and arrive at interesting findings that do not match the current practice of network design. The RegNet design space provides simple and fast networks that work well across a wide range of flop regimes. Under comparable training settings and flops, the RegNet models outperform the popular EfficientNet models while being up to 5x faster on GPUs.*

This model was contributed by [Francesco](https://huggingface.co/Francesco). The original code can be found [here](https://github.com/facebookresearch/pycls).

The huge 10B model from [Self-supervised Pretraining of Visual Features in the Wild](https://huggingface.co/papers/2103.01988),
trained on  one billion Instagram images, is available on the [hub](https://huggingface.co/facebook/regnet-y-10b-seer)

## Resources

A list of official Hugging Face and community (indicated by 🌎) resources to help you get started with RegNet.

- [RegNetForImageClassification](/docs/transformers/v5.15.1/en/model_doc/regnet#transformers.RegNetForImageClassification) is supported by this [example script](https://github.com/huggingface/transformers/tree/main/examples/pytorch/image-classification) and [notebook](https://colab.research.google.com/github/huggingface/notebooks/blob/main/examples/image_classification.ipynb).
- See also: [Image classification task guide](../tasks/image_classification)

If you're interested in submitting a resource to be included here, please feel free to open a Pull Request and we'll review it! The resource should ideally demonstrate something new instead of duplicating an existing resource.

## RegNetConfig[[transformers.RegNetConfig]]

#### transformers.RegNetConfig[[transformers.RegNetConfig]]

```python
transformers.RegNetConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, num_channels: int = 3, embedding_size: int = 32, hidden_sizes: list[int] | tuple[int, ...] = (128, 192, 512, 1088), depths: list[int] | tuple[int, ...] = (2, 6, 12, 2), groups_width: int = 64, layer_type: str = 'y', hidden_act: str = 'relu', downsample_in_first_stage: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/regnet/configuration_regnet.py#L24)

**Parameters:**

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

embedding_size (`int`, *optional*, defaults to `32`) : Dimensionality of the embeddings and hidden states.

hidden_sizes (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(128, 192, 512, 1088)`) : Dimensionality (hidden size) at each stage of the model.

depths (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(2, 6, 12, 2)`) : Depth of each layer in the Transformer.

groups_width (`int`, *optional*, defaults to 64) : Width of group for each stage.

layer_type (`str`, *optional*, defaults to `"y"`) : The layer to use, it can be either `"x" or `"y"`. An `x` layer is a ResNet's BottleNeck layer with `reduction` fixed to `1`. While a `y` layer is a `x` but with squeeze and excitation. Please refer to the paper for a detailed explanation of how these layers were constructed.

hidden_act (`str`, *optional*, defaults to `relu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

downsample_in_first_stage (`bool`, *optional*, defaults to `False`) : If `True`, the first stage will downsample the inputs using a `stride` of 2.

This is the configuration class to store the configuration of a RegNetModel. It is used to instantiate a Regnet
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/regnet-y-040](https://huggingface.co/facebook/regnet-y-040)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import RegNetConfig, RegNetModel

>>> # Initializing a RegNet regnet-y-40 style configuration
>>> configuration = RegNetConfig()
>>> # Initializing a model from the regnet-y-40 style configuration
>>> model = RegNetModel(configuration)
>>> # Accessing the model configuration
>>> configuration = model.config
```

## RegNetModel[[transformers.RegNetModel]]

#### transformers.RegNetModel[[transformers.RegNetModel]]

```python
transformers.RegNetModel(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/regnet/modeling_regnet.py#L282)

**Parameters:**

config ([RegNetModel](/docs/transformers/v5.15.1/en/model_doc/regnet#transformers.RegNetModel)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Regnet Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.RegNetModel.forward]]

```python
forward(pixel_values: Tensor, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/regnet/modeling_regnet.py#L292)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [ConvNextImageProcessor](/docs/transformers/v5.15.1/en/model_doc/convnext#transformers.ConvNextImageProcessor). See `ConvNextImageProcessor.__call__()` for details (`processor_class` uses [ConvNextImageProcessor](/docs/transformers/v5.15.1/en/model_doc/convnext#transformers.ConvNextImageProcessor) for processing images).

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `BaseModelOutputWithPoolingAndNoAttention` or `tuple(torch.FloatTensor)`

A `BaseModelOutputWithPoolingAndNoAttention` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([RegNetConfig](/docs/transformers/v5.15.1/en/model_doc/regnet#transformers.RegNetConfig)) and inputs.

The [RegNetModel](/docs/transformers/v5.15.1/en/model_doc/regnet#transformers.RegNetModel) forward method, overrides the `__call__` special method.

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

## RegNetForImageClassification[[transformers.RegNetForImageClassification]]

#### transformers.RegNetForImageClassification[[transformers.RegNetForImageClassification]]

```python
transformers.RegNetForImageClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/regnet/modeling_regnet.py#L332)

**Parameters:**

config ([RegNetForImageClassification](/docs/transformers/v5.15.1/en/model_doc/regnet#transformers.RegNetForImageClassification)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

RegNet Model with an image classification head on top (a linear layer on top of the pooled features), e.g. for
ImageNet.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.RegNetForImageClassification.forward]]

```python
forward(pixel_values: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/regnet/modeling_regnet.py#L345)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [ConvNextImageProcessor](/docs/transformers/v5.15.1/en/model_doc/convnext#transformers.ConvNextImageProcessor). See `ConvNextImageProcessor.__call__()` for details (`processor_class` uses [ConvNextImageProcessor](/docs/transformers/v5.15.1/en/model_doc/convnext#transformers.ConvNextImageProcessor) for processing images).

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the image classification/regression loss. Indices should be in `[0, ..., config.num_labels - 1]`. If `config.num_labels > 1` a classification loss is computed (Cross-Entropy).

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [ImageClassifierOutputWithNoAttention](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutputWithNoAttention) or `tuple(torch.FloatTensor)`

A [ImageClassifierOutputWithNoAttention](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutputWithNoAttention) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([RegNetConfig](/docs/transformers/v5.15.1/en/model_doc/regnet#transformers.RegNetConfig)) and inputs.

The [RegNetForImageClassification](/docs/transformers/v5.15.1/en/model_doc/regnet#transformers.RegNetForImageClassification) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoImageProcessor, RegNetForImageClassification
>>> import torch
>>> from datasets import load_dataset

>>> dataset = load_dataset("huggingface/cats-image")
>>> image = dataset["test"]["image"][0]

>>> image_processor = AutoImageProcessor.from_pretrained("facebook/regnet-y-040")
>>> model = RegNetForImageClassification.from_pretrained("facebook/regnet-y-040")

>>> inputs = image_processor(image, return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # model predicts one of the 1000 ImageNet classes
>>> predicted_label = logits.argmax(-1).item()
>>> print(model.config.id2label[predicted_label])
...
```
