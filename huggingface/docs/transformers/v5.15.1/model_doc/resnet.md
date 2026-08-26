# ResNet

## Overview

The ResNet model was proposed in [Deep Residual Learning for Image Recognition](https://huggingface.co/papers/1512.03385) by Kaiming He, Xiangyu Zhang, Shaoqing Ren and Jian Sun. Our implementation follows the small changes made by [Nvidia](https://catalog.ngc.nvidia.com/orgs/nvidia/resources/resnet_50_v1_5_for_pytorch), we apply the `stride=2` for downsampling in bottleneck's `3x3` conv and not in the first `1x1`. This is generally known as "ResNet v1.5".

ResNet introduced residual connections, they allow to train networks with an unseen number of layers (up to 1000). ResNet won the 2015 ILSVRC & COCO competition, one important milestone in deep computer vision.

The abstract from the paper is the following:

*Deeper neural networks are more difficult to train. We present a residual learning framework to ease the training of networks that are substantially deeper than those used previously. We explicitly reformulate the layers as learning residual functions with reference to the layer inputs, instead of learning unreferenced functions. We provide comprehensive empirical evidence showing that these residual networks are easier to optimize, and can gain accuracy from considerably increased depth. On the ImageNet dataset we evaluate residual nets with a depth of up to 152 layers---8x deeper than VGG nets but still having lower complexity. An ensemble of these residual nets achieves 3.57% error on the ImageNet test set. This result won the 1st place on the ILSVRC 2015 classification task. We also present analysis on CIFAR-10 with 100 and 1000 layers.
The depth of representations is of central importance for many visual recognition tasks. Solely due to our extremely deep representations, we obtain a 28% relative improvement on the COCO object detection dataset. Deep residual nets are foundations of our submissions to ILSVRC & COCO 2015 competitions, where we also won the 1st places on the tasks of ImageNet detection, ImageNet localization, COCO detection, and COCO segmentation.*

The figure below illustrates the architecture of ResNet. Taken from the [original paper](https://huggingface.co/papers/1512.03385).

This model was contributed by [Francesco](https://huggingface.co/Francesco). The original code can be found [here](https://github.com/KaimingHe/deep-residual-networks).

## Resources

A list of official Hugging Face and community (indicated by 🌎) resources to help you get started with ResNet.

- [ResNetForImageClassification](/docs/transformers/v5.15.1/en/model_doc/resnet#transformers.ResNetForImageClassification) is supported by this [example script](https://github.com/huggingface/transformers/tree/main/examples/pytorch/image-classification) and [notebook](https://colab.research.google.com/github/huggingface/notebooks/blob/main/examples/image_classification.ipynb).
- See also: [Image classification task guide](../tasks/image_classification)

If you're interested in submitting a resource to be included here, please feel free to open a Pull Request and we'll review it! The resource should ideally demonstrate something new instead of duplicating an existing resource.

## ResNetConfig[[transformers.ResNetConfig]]

#### transformers.ResNetConfig[[transformers.ResNetConfig]]

```python
transformers.ResNetConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, num_channels: int = 3, embedding_size: int = 64, hidden_sizes: list[int] | tuple[int, ...] | None = (256, 512, 1024, 2048), depths: list[int] | tuple[int, ...] | None = (3, 4, 6, 3), layer_type: str = 'bottleneck', hidden_act: str = 'relu', downsample_in_first_stage: bool = False, downsample_in_bottleneck: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/resnet/configuration_resnet.py#L27)

**Parameters:**

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

embedding_size (`int`, *optional*, defaults to `64`) : Dimensionality of the embeddings and hidden states.

hidden_sizes (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(256, 512, 1024, 2048)`) : Dimensionality (hidden size) at each stage of the model.

depths (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(3, 4, 6, 3)`) : Depth of each layer in the Transformer.

layer_type (`str`, *optional*, defaults to `"bottleneck"`) : The layer to use, it can be either `"basic"` (used for smaller models, like resnet-18 or resnet-34) or `"bottleneck"` (used for larger models like resnet-50 and above).

hidden_act (`str`, *optional*, defaults to `relu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

downsample_in_first_stage (`bool`, *optional*, defaults to `False`) : If `True`, the first stage will downsample the inputs using a `stride` of 2.

downsample_in_bottleneck (`bool`, *optional*, defaults to `False`) : If `True`, the first conv 1x1 in ResNetBottleNeckLayer will downsample the inputs using a `stride` of 2.

This is the configuration class to store the configuration of a ResNetModel. It is used to instantiate a Resnet
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [microsoft/resnet-50](https://huggingface.co/microsoft/resnet-50)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import ResNetConfig, ResNetModel

>>> # Initializing a ResNet resnet-50 style configuration
>>> configuration = ResNetConfig()

>>> # Initializing a model (with random weights) from the resnet-50 style configuration
>>> model = ResNetModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

#### validate_layer_type[[transformers.ResNetConfig.validate_layer_type]]

```python
validate_layer_type()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/resnet/configuration_resnet.py#L72)

Check that `layer_types` is correctly defined.

## ResNetModel[[transformers.ResNetModel]]

#### transformers.ResNetModel[[transformers.ResNetModel]]

```python
transformers.ResNetModel(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/resnet/modeling_resnet.py#L291)

**Parameters:**

config ([ResNetModel](/docs/transformers/v5.15.1/en/model_doc/resnet#transformers.ResNetModel)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Resnet Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.ResNetModel.forward]]

```python
forward(pixel_values: Tensor, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/resnet/modeling_resnet.py#L301)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [ConvNextImageProcessor](/docs/transformers/v5.15.1/en/model_doc/convnext#transformers.ConvNextImageProcessor). See `ConvNextImageProcessor.__call__()` for details (`processor_class` uses [ConvNextImageProcessor](/docs/transformers/v5.15.1/en/model_doc/convnext#transformers.ConvNextImageProcessor) for processing images).

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `BaseModelOutputWithPoolingAndNoAttention` or `tuple(torch.FloatTensor)`

A `BaseModelOutputWithPoolingAndNoAttention` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ResNetConfig](/docs/transformers/v5.15.1/en/model_doc/resnet#transformers.ResNetConfig)) and inputs.

The [ResNetModel](/docs/transformers/v5.15.1/en/model_doc/resnet#transformers.ResNetModel) forward method, overrides the `__call__` special method.

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

## ResNetForImageClassification[[transformers.ResNetForImageClassification]]

#### transformers.ResNetForImageClassification[[transformers.ResNetForImageClassification]]

```python
transformers.ResNetForImageClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/resnet/modeling_resnet.py#L340)

**Parameters:**

config ([ResNetForImageClassification](/docs/transformers/v5.15.1/en/model_doc/resnet#transformers.ResNetForImageClassification)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

ResNet Model with an image classification head on top (a linear layer on top of the pooled features), e.g. for
ImageNet.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.ResNetForImageClassification.forward]]

```python
forward(pixel_values: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/resnet/modeling_resnet.py#L353)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [ConvNextImageProcessor](/docs/transformers/v5.15.1/en/model_doc/convnext#transformers.ConvNextImageProcessor). See `ConvNextImageProcessor.__call__()` for details (`processor_class` uses [ConvNextImageProcessor](/docs/transformers/v5.15.1/en/model_doc/convnext#transformers.ConvNextImageProcessor) for processing images).

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the image classification/regression loss. Indices should be in `[0, ..., config.num_labels - 1]`. If `config.num_labels > 1` a classification loss is computed (Cross-Entropy).

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [ImageClassifierOutputWithNoAttention](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutputWithNoAttention) or `tuple(torch.FloatTensor)`

A [ImageClassifierOutputWithNoAttention](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutputWithNoAttention) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ResNetConfig](/docs/transformers/v5.15.1/en/model_doc/resnet#transformers.ResNetConfig)) and inputs.

The [ResNetForImageClassification](/docs/transformers/v5.15.1/en/model_doc/resnet#transformers.ResNetForImageClassification) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoImageProcessor, ResNetForImageClassification
>>> import torch
>>> from datasets import load_dataset

>>> dataset = load_dataset("huggingface/cats-image")
>>> image = dataset["test"]["image"][0]

>>> image_processor = AutoImageProcessor.from_pretrained("microsoft/resnet-50")
>>> model = ResNetForImageClassification.from_pretrained("microsoft/resnet-50")

>>> inputs = image_processor(image, return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # model predicts one of the 1000 ImageNet classes
>>> predicted_label = logits.argmax(-1).item()
>>> print(model.config.id2label[predicted_label])
...
```
