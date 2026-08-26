# Hiera

## Overview

Hiera was proposed in [Hiera: A Hierarchical Vision Transformer without the Bells-and-Whistles](https://huggingface.co/papers/2306.00989) by Chaitanya Ryali, Yuan-Ting Hu, Daniel Bolya, Chen Wei, Haoqi Fan, Po-Yao Huang, Vaibhav Aggarwal, Arkabandhu Chowdhury, Omid Poursaeed, Judy Hoffman, Jitendra Malik, Yanghao Li, Christoph Feichtenhofer

The paper introduces "Hiera," a hierarchical Vision Transformer that simplifies the architecture of modern hierarchical vision transformers by removing unnecessary components without compromising on accuracy or efficiency. Unlike traditional transformers that add complex vision-specific components to improve supervised classification performance, Hiera demonstrates that such additions, often termed "bells-and-whistles," are not essential for high accuracy. By leveraging a strong visual pretext task (MAE) for pretraining, Hiera retains simplicity and achieves superior accuracy and speed both in inference and training across various image and video recognition tasks. The approach suggests that spatial biases required for vision tasks can be effectively learned through proper pretraining, eliminating the need for added architectural complexity.

The abstract from the paper is the following:

*Modern hierarchical vision transformers have added several vision-specific components in the pursuit of supervised classification performance. While these components lead to effective accuracies and attractive FLOP counts, the added complexity actually makes these transformers slower than their vanilla ViT counterparts. In this paper, we argue that this additional bulk is unnecessary. By pretraining with a strong visual pretext task (MAE), we can strip out all the bells-and-whistles from a state-of-the-art multi-stage vision transformer without losing accuracy. In the process, we create Hiera, an extremely simple hierarchical vision transformer that is more accurate than previous models while being significantly faster both at inference and during training. We evaluate Hiera on a variety of tasks for image and video recognition. Our code and models are available at https://github.com/facebookresearch/hiera.*

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/hiera_overview.png"
alt="drawing" width="600"/>

 Hiera architecture. Taken from the original paper. 

This model was a joint contribution by [EduardoPacheco](https://huggingface.co/EduardoPacheco) and [namangarg110](https://huggingface.co/namangarg110). The original code can be found [here] (https://github.com/facebookresearch/hiera).

## Resources

A list of official Hugging Face and community (indicated by 🌎) resources to help you get started with Hiera. If you're interested in submitting a resource to be included here, please feel free to open a Pull Request and we'll review it! The resource should ideally demonstrate something new instead of duplicating an existing resource.

- [HieraForImageClassification](/docs/transformers/v5.15.1/en/model_doc/hiera#transformers.HieraForImageClassification) is supported by this [example script](https://github.com/huggingface/transformers/tree/main/examples/pytorch/image-classification) and [notebook](https://colab.research.google.com/github/huggingface/notebooks/blob/main/examples/image_classification.ipynb).
- See also: [Image classification task guide](../tasks/image_classification)

## HieraConfig[[transformers.HieraConfig]]

#### transformers.HieraConfig[[transformers.HieraConfig]]

```python
transformers.HieraConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, embed_dim: int = 96, image_size: list[int] | tuple[int, ...] = (224, 224), patch_size: list[int] | tuple[int, ...] = (7, 7), patch_stride: list[int] | tuple[int, ...] = (4, 4), patch_padding: list[int] | tuple[int, ...] = (3, 3), mlp_ratio: float = 4.0, depths: list[int] | tuple[int, ...] = (2, 3, 16, 3), num_heads: list[int] | tuple[int, ...] = (1, 2, 4, 8), embed_dim_multiplier: float | int = 2.0, num_query_pool: int = 3, query_stride: list[int] | tuple[int, ...] = (2, 2), masked_unit_size: list[int] | tuple[int, ...] = (8, 8), masked_unit_attention: list[bool] | tuple[bool, ...] = (True, True, False, False), drop_path_rate: float | int = 0.0, num_channels: int = 3, hidden_act: str = 'gelu', initializer_range: float = 0.02, layer_norm_init: float = 1.0, layer_norm_eps: float = 1e-06, decoder_hidden_size: int | None = None, decoder_depth: int | None = None, decoder_num_heads: int | None = None, normalize_pixel_loss: bool | None = True, mask_ratio: float = 0.6, _out_features: list[str] | None = None, _out_indices: list[int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hiera/configuration_hiera.py#L25)

**Parameters:**

embed_dim (`int`, *optional*, defaults to `96`) : Dimensionality of the embeddings and hidden states.

image_size (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(224, 224)`) : The size (resolution) of each image.

patch_size (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(7, 7)`) : The size (resolution) of each patch.

patch_stride (`list(int)`, *optional*, defaults to `[4, 4]`) : The stride of the patch.

patch_padding (`list(int)`, *optional*, defaults to `[3, 3]`) : The padding of the patch.

mlp_ratio (`float`, *optional*, defaults to `4.0`) : Ratio of the MLP hidden dim to the embedding dim.

depths (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(2, 3, 16, 3)`) : Depth of each layer in the Transformer.

num_heads (`list(int)`, *optional*, defaults to `[1, 2, 4, 8]`) : Number of attention heads in each layer of the Transformer encoder.

embed_dim_multiplier (`float`, *optional*, defaults to 2.0) : The multiplier to the dimensionality of patch embedding in each layer of the Transformer encoder.

num_query_pool (`int`, *optional*, defaults to 3) : The number of query pool stages.

query_stride (`list(int)`, *optional*, defaults to `[2, 2]`) : The stride of the query pool.

masked_unit_size (`list(int)`, *optional*, defaults to `[8, 8]`) : The size of the masked unit.

masked_unit_attention (`list(bool)`, *optional*, defaults to `[True, True, False, False]`) : Whether to use masked unit attention in each layer of the Transformer encoder.

drop_path_rate (`Union[float, int]`, *optional*, defaults to `0.0`) : Drop path rate for the patch fusion.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_init (`float`, *optional*, defaults to 1.0) : The initial weight value for layer normalization layers.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

decoder_hidden_size (`int`, *optional*) : Dimension of the hidden representations.

decoder_depth (`int`, *optional*) : Depth of the decoder for MAE pretraining.

decoder_num_heads (`int`, *optional*) : Number of attention heads for each attention layer in the Transformer decoder.

normalize_pixel_loss (`bool`, *optional*, defaults to `True`) : Whether to normalize the pixel loss by the number of pixels.

mask_ratio (`float`, *optional*, defaults to 0.6) : The ratio of masked tokens in the input.

This is the configuration class to store the configuration of a HieraModel. It is used to instantiate a Hiera
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/hiera-base-224](https://huggingface.co/facebook/hiera-base-224)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import HieraConfig, HieraModel

>>> # Initializing a Hiera hiera-base-patch16-224 style configuration
>>> configuration = HieraConfig()

>>> # Initializing a model (with random weights) from the hiera-base-patch16-224 style configuration
>>> model = HieraModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## HieraModel[[transformers.HieraModel]]

#### transformers.HieraModel[[transformers.HieraModel]]

```python
transformers.HieraModel(config: HieraConfig, add_pooling_layer: bool = True, is_mae: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hiera/modeling_hiera.py#L793)

**Parameters:**

config ([HieraConfig](/docs/transformers/v5.15.1/en/model_doc/hiera#transformers.HieraConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

add_pooling_layer (`bool`, *optional*, defaults to `True`) : Whether or not to apply pooling layer.

is_mae (`bool`, *optional*, defaults to `False`) : Whether or not to run the model on MAE mode.

The bare Hiera Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.HieraModel.forward]]

```python
forward(pixel_values: typing.Optional[torch.Tensor] = None, noise: typing.Optional[torch.FloatTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, interpolate_pos_encoding: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hiera/modeling_hiera.py#L817)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [BitImageProcessor](/docs/transformers/v5.15.1/en/model_doc/bit#transformers.BitImageProcessor). See `BitImageProcessor.__call__()` for details (`processor_class` uses [BitImageProcessor](/docs/transformers/v5.15.1/en/model_doc/bit#transformers.BitImageProcessor) for processing images).

noise (`torch.FloatTensor` of shape `(batch_size, num_mask_units)`, *optional*) : Mainly used for testing purposes to control randomness and maintain the reproducibility

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

interpolate_pos_encoding (`bool`, *optional*) : Whether to interpolate the pre-trained position encodings.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([HieraConfig](/docs/transformers/v5.15.1/en/model_doc/hiera#transformers.HieraConfig)) and inputs.

The [HieraModel](/docs/transformers/v5.15.1/en/model_doc/hiera#transformers.HieraModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Last layer hidden-state of the first token of the sequence (classification token) after further processing
  through the layers used for the auxiliary pretraining task. E.g. for BERT-family of models, this returns
  the classification token after processing through a linear layer and a tanh activation function. The linear
  layer weights are trained from the next sentence prediction (classification) objective during pretraining.
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

## HieraForPreTraining[[transformers.HieraForPreTraining]]

#### transformers.HieraForPreTraining[[transformers.HieraForPreTraining]]

```python
transformers.HieraForPreTraining(config: HieraConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hiera/modeling_hiera.py#L1061)

**Parameters:**

config ([HieraConfig](/docs/transformers/v5.15.1/en/model_doc/hiera#transformers.HieraConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Hiera Model transformer with the decoder on top for self-supervised pre-training.

Note that we provide a script to pre-train this model on custom data in our [examples
directory](https://github.com/huggingface/transformers/tree/main/examples/pytorch/image-pretraining).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.HieraForPreTraining.forward]]

```python
forward(pixel_values: typing.Optional[torch.Tensor] = None, noise: typing.Optional[torch.FloatTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, interpolate_pos_encoding: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hiera/modeling_hiera.py#L1102)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [BitImageProcessor](/docs/transformers/v5.15.1/en/model_doc/bit#transformers.BitImageProcessor). See `BitImageProcessor.__call__()` for details (`processor_class` uses [BitImageProcessor](/docs/transformers/v5.15.1/en/model_doc/bit#transformers.BitImageProcessor) for processing images).

noise (`torch.FloatTensor` of shape `(batch_size, num_mask_units)`, *optional*) : Mainly used for testing purposes to control randomness and maintain the reproducibility

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

interpolate_pos_encoding (`bool`, *optional*) : Whether to interpolate the pre-trained position encodings.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `HieraForPreTrainingOutput` or `tuple(torch.FloatTensor)`

A `HieraForPreTrainingOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([HieraConfig](/docs/transformers/v5.15.1/en/model_doc/hiera#transformers.HieraConfig)) and inputs.

The [HieraForPreTraining](/docs/transformers/v5.15.1/en/model_doc/hiera#transformers.HieraForPreTraining) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`) -- Pixel reconstruction loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, patch_size ** 2 * num_channels)`) -- Pixel reconstruction logits.
- **bool_masked_pos** (`torch.BoolTensor` of shape `(batch_size, sequence_length)`) -- Tensor indicating which patches are masked (0) and which are not (1).
- **ids_restore** (`torch.LongTensor` of shape `(batch_size, sequence_length)`) -- Tensor containing the original index of the (shuffled) masked patches.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **reshaped_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer) of
  shape `(batch_size, height, width, hidden_size)`. Hidden-states of the model at the output of each layer
  plus the initial embedding outputs reshaped to include the spatial dimensions.

Examples:
```python
>>> from transformers import AutoImageProcessor, HieraForPreTraining
>>> import torch
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("facebook/hiera-tiny-224-mae-hf")
>>> model = HieraForPreTraining.from_pretrained("facebook/hiera-tiny-224-mae-hf")

>>> inputs = image_processor(images=image, return_tensors="pt")

>>> outputs = model(**inputs)
>>> logits = outputs.logits
>>> loss = outputs.loss
>>> print(list(logits.shape))
[1, 196, 768]
```

## HieraForImageClassification[[transformers.HieraForImageClassification]]

#### transformers.HieraForImageClassification[[transformers.HieraForImageClassification]]

```python
transformers.HieraForImageClassification(config: HieraConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hiera/modeling_hiera.py#L1207)

**Parameters:**

config ([HieraConfig](/docs/transformers/v5.15.1/en/model_doc/hiera#transformers.HieraConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Hiera Model transformer with an image classification head on top (a linear layer on top of the final hidden state with
average pooling) e.g. for ImageNet.

Note that it's possible to fine-tune Hiera on higher resolution images than the ones it has been trained on, by
setting `interpolate_pos_encoding` to `True` in the forward of the model. This will interpolate the pre-trained
position embeddings to the higher resolution.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.HieraForImageClassification.forward]]

```python
forward(pixel_values, labels: typing.Optional[torch.Tensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, interpolate_pos_encoding: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hiera/modeling_hiera.py#L1222)

**Parameters:**

pixel_values (`` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [BitImageProcessor](/docs/transformers/v5.15.1/en/model_doc/bit#transformers.BitImageProcessor). See `BitImageProcessor.__call__()` for details (`processor_class` uses [BitImageProcessor](/docs/transformers/v5.15.1/en/model_doc/bit#transformers.BitImageProcessor) for processing images).

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the image classification/regression loss. Indices should be in `[0, ..., config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If `config.num_labels > 1` a classification loss is computed (Cross-Entropy).

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

interpolate_pos_encoding (`bool`, *optional*) : Whether to interpolate the pre-trained position encodings.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `HieraForImageClassificationOutput` or `tuple(torch.FloatTensor)`

A `HieraForImageClassificationOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([HieraConfig](/docs/transformers/v5.15.1/en/model_doc/hiera#transformers.HieraConfig)) and inputs.

The [HieraForImageClassification](/docs/transformers/v5.15.1/en/model_doc/hiera#transformers.HieraForImageClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Classification (or regression if config.num_labels==1) scores (before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each stage) of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states
  (also called feature maps) of the model at the output of each stage.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, patch_size,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **reshaped_hidden_states** (`tuple(torch.FloatTensor)`, `optional`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, height, width, hidden_size)`. These are the reshaped and re-rolled hidden states of the model.

  Hidden-states of the model at the output of each layer plus the initial embedding outputs reshaped to
  include the spatial dimensions.

Example:

```python
>>> from transformers import AutoImageProcessor, HieraForImageClassification
>>> import torch
>>> from datasets import load_dataset

>>> dataset = load_dataset("huggingface/cats-image")
>>> image = dataset["test"]["image"][0]

>>> image_processor = AutoImageProcessor.from_pretrained("facebook/hiera-base-224")
>>> model = HieraForImageClassification.from_pretrained("facebook/hiera-base-224")

>>> inputs = image_processor(image, return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # model predicts one of the 1000 ImageNet classes
>>> predicted_label = logits.argmax(-1).item()
>>> print(model.config.id2label[predicted_label])
...
```
