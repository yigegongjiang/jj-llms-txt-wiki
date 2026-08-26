# Dilated Neighborhood Attention Transformer

## Overview

DiNAT was proposed in [Dilated Neighborhood Attention Transformer](https://huggingface.co/papers/2209.15001)
by Ali Hassani and Humphrey Shi.

It extends [NAT](nat) by adding a Dilated Neighborhood Attention pattern to capture global context,
and shows significant performance improvements over it.

The abstract from the paper is the following:

*Transformers are quickly becoming one of the most heavily applied deep learning architectures across modalities,
domains, and tasks. In vision, on top of ongoing efforts into plain transformers, hierarchical transformers have
also gained significant attention, thanks to their performance and easy integration into existing frameworks.
These models typically employ localized attention mechanisms, such as the sliding-window Neighborhood Attention (NA)
or Swin Transformer's Shifted Window Self Attention. While effective at reducing self attention's quadratic complexity,
local attention weakens two of the most desirable properties of self attention: long range inter-dependency modeling,
and global receptive field. In this paper, we introduce Dilated Neighborhood Attention (DiNA), a natural, flexible and
efficient extension to NA that can capture more global context and expand receptive fields exponentially at no
additional cost. NA's local attention and DiNA's sparse global attention complement each other, and therefore we
introduce Dilated Neighborhood Attention Transformer (DiNAT), a new hierarchical vision transformer built upon both.
DiNAT variants enjoy significant improvements over strong baselines such as NAT, Swin, and ConvNeXt.
Our large model is faster and ahead of its Swin counterpart by 1.5% box AP in COCO object detection,
1.3% mask AP in COCO instance segmentation, and 1.1% mIoU in ADE20K semantic segmentation.
Paired with new frameworks, our large variant is the new state of the art panoptic segmentation model on COCO (58.2 PQ)
and ADE20K (48.5 PQ), and instance segmentation model on Cityscapes (44.5 AP) and ADE20K (35.4 AP) (no extra data).
It also matches the state of the art specialized semantic segmentation models on ADE20K (58.2 mIoU),
and ranks second on Cityscapes (84.5 mIoU) (no extra data).*

<img
src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/dilated-neighborhood-attention-pattern.jpg"
alt="drawing" width="600"/>

 Neighborhood Attention with different dilation values.
Taken from the original paper.

This model was contributed by [Ali Hassani](https://huggingface.co/alihassanijr).
The original code can be found [here](https://github.com/SHI-Labs/Neighborhood-Attention-Transformer).

## Usage tips

DiNAT can be used as a *backbone*. When `output_hidden_states = True`,
it will output both `hidden_states` and `reshaped_hidden_states`. The `reshaped_hidden_states` have a shape of `(batch, num_channels, height, width)` rather than `(batch_size, height, width, num_channels)`.

Notes:

- DiNAT depends on [NATTEN](https://github.com/SHI-Labs/NATTEN/)'s implementation of Neighborhood Attention and Dilated Neighborhood Attention.
You can install it with pre-built wheels for Linux by referring to [shi-labs.com/natten](https://shi-labs.com/natten), or build on your system by running `pip install natten`.
Note that the latter will likely take time to compile. NATTEN does not support Windows devices yet.
- Patch size of 4 is only supported at the moment.

## Resources

A list of official Hugging Face and community (indicated by 🌎) resources to help you get started with DiNAT.

- [DinatForImageClassification](/docs/transformers/v5.15.1/en/model_doc/dinat#transformers.DinatForImageClassification) is supported by this [example script](https://github.com/huggingface/transformers/tree/main/examples/pytorch/image-classification) and [notebook](https://colab.research.google.com/github/huggingface/notebooks/blob/main/examples/image_classification.ipynb).
- See also: [Image classification task guide](../tasks/image_classification)

If you're interested in submitting a resource to be included here, please feel free to open a Pull Request and we'll review it! The resource should ideally demonstrate something new instead of duplicating an existing resource.

## DinatConfig[[transformers.DinatConfig]]

#### transformers.DinatConfig[[transformers.DinatConfig]]

```python
transformers.DinatConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, patch_size: int | list[int] | tuple[int, int] = 4, num_channels: int = 3, embed_dim: int = 64, depths: list[int] | tuple[int, ...] = (3, 4, 6, 5), num_heads: list[int] | tuple[int, ...] = (2, 4, 8, 16), kernel_size: int = 7, dilations: list | tuple | None = None, mlp_ratio: float = 3.0, qkv_bias: bool = True, hidden_dropout_prob: float | int = 0.0, attention_probs_dropout_prob: float | int = 0.0, drop_path_rate: float | int = 0.1, hidden_act: str = 'gelu', initializer_range: float = 0.02, layer_norm_eps: float = 1e-05, layer_scale_init_value: float = 0.0, _out_features: list[str] | None = None, _out_indices: list[int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dinat/configuration_dinat.py#L25)

**Parameters:**

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `4`) : The size (resolution) of each patch.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

embed_dim (`int`, *optional*, defaults to `64`) : Dimensionality of the embeddings and hidden states.

depths (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(3, 4, 6, 5)`) : Depth of each layer in the Transformer.

num_heads (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(2, 4, 8, 16)`) : Number of attention heads for each attention layer in the Transformer decoder.

kernel_size (`int`, *optional*, defaults to `7`) : The size of the convolutional kernel.

dilations (`list[list[int]]`, *optional*, defaults to `[[1, 8, 1], [1, 4, 1, 4], [1, 2, 1, 2, 1, 2], [1, 1, 1, 1, 1]]`) : Dilation value of each NA layer in the Transformer encoder.

mlp_ratio (`float`, *optional*, defaults to `3.0`) : Ratio of the MLP hidden dim to the embedding dim.

qkv_bias (`bool`, *optional*, defaults to `True`) : Whether to add a bias to the queries, keys and values.

hidden_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

attention_probs_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

drop_path_rate (`Union[float, int]`, *optional*, defaults to `0.1`) : Drop path rate for the patch fusion.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

layer_scale_init_value (`float`, *optional*, defaults to `0.0`) : Scale to use in the self-attention layers. 0.1 for base, 1e-6 for large. Set 0 to disable layer scale.

This is the configuration class to store the configuration of a DinatModel. It is used to instantiate a Dinat
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [shi-labs/dinat-mini-in1k-224](https://huggingface.co/shi-labs/dinat-mini-in1k-224)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import DinatConfig, DinatModel

>>> # Initializing a Dinat shi-labs/dinat-mini-in1k-224 style configuration
>>> configuration = DinatConfig()

>>> # Initializing a model (with random weights) from the shi-labs/dinat-mini-in1k-224 style configuration
>>> model = DinatModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## DinatModel[[transformers.DinatModel]]

#### transformers.DinatModel[[transformers.DinatModel]]

```python
transformers.DinatModel(config, add_pooling_layer = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dinat/modeling_dinat.py#L541)

**Parameters:**

config ([DinatModel](/docs/transformers/v5.15.1/en/model_doc/dinat#transformers.DinatModel)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

add_pooling_layer (`bool`, *optional*, defaults to `True`) : Whether to add a pooling layer

The bare Dinat Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.DinatModel.forward]]

```python
forward(pixel_values: typing.Optional[torch.FloatTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dinat/modeling_dinat.py#L567)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [ViTImageProcessor](/docs/transformers/v5.15.1/en/model_doc/vit#transformers.ViTImageProcessor). See `ViTImageProcessor.__call__()` for details (`processor_class` uses [ViTImageProcessor](/docs/transformers/v5.15.1/en/model_doc/vit#transformers.ViTImageProcessor) for processing images).

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `DinatModelOutput` or `tuple(torch.FloatTensor)`

A `DinatModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DinatConfig](/docs/transformers/v5.15.1/en/model_doc/dinat#transformers.DinatConfig)) and inputs.

The [DinatModel](/docs/transformers/v5.15.1/en/model_doc/dinat#transformers.DinatModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`, *optional*, returned when `add_pooling_layer=True` is passed) -- Average pooling of the last layer hidden-state.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **reshaped_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, hidden_size, height, width)`.

  Hidden-states of the model at the output of each layer plus the initial embedding outputs reshaped to
  include the spatial dimensions.

Example:

```python
```

## DinatForImageClassification[[transformers.DinatForImageClassification]]

#### transformers.DinatForImageClassification[[transformers.DinatForImageClassification]]

```python
transformers.DinatForImageClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dinat/modeling_dinat.py#L622)

**Parameters:**

config ([DinatForImageClassification](/docs/transformers/v5.15.1/en/model_doc/dinat#transformers.DinatForImageClassification)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Dinat Model transformer with an image classification head on top (a linear layer on top of the final hidden state
of the [CLS] token) e.g. for ImageNet.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.DinatForImageClassification.forward]]

```python
forward(pixel_values: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dinat/modeling_dinat.py#L639)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [ViTImageProcessor](/docs/transformers/v5.15.1/en/model_doc/vit#transformers.ViTImageProcessor). See `ViTImageProcessor.__call__()` for details (`processor_class` uses [ViTImageProcessor](/docs/transformers/v5.15.1/en/model_doc/vit#transformers.ViTImageProcessor) for processing images).

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the image classification/regression loss. Indices should be in `[0, ..., config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If `config.num_labels > 1` a classification loss is computed (Cross-Entropy).

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `DinatImageClassifierOutput` or `tuple(torch.FloatTensor)`

A `DinatImageClassifierOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DinatConfig](/docs/transformers/v5.15.1/en/model_doc/dinat#transformers.DinatConfig)) and inputs.

The [DinatForImageClassification](/docs/transformers/v5.15.1/en/model_doc/dinat#transformers.DinatForImageClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Classification (or regression if config.num_labels==1) scores (before SoftMax).
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **reshaped_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, hidden_size, height, width)`.

  Hidden-states of the model at the output of each layer plus the initial embedding outputs reshaped to
  include the spatial dimensions.

Example:

```python
>>> from transformers import AutoImageProcessor, DinatForImageClassification
>>> import torch
>>> from datasets import load_dataset

>>> dataset = load_dataset("huggingface/cats-image")
>>> image = dataset["test"]["image"][0]

>>> image_processor = AutoImageProcessor.from_pretrained("shi-labs/dinat-mini-in1k-224")
>>> model = DinatForImageClassification.from_pretrained("shi-labs/dinat-mini-in1k-224")

>>> inputs = image_processor(image, return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # model predicts one of the 1000 ImageNet classes
>>> predicted_label = logits.argmax(-1).item()
>>> print(model.config.id2label[predicted_label])
...
```
