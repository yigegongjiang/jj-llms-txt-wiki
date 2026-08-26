# Backbone

A backbone is a model used for feature extraction for higher level computer vision tasks such as object detection and image classification. Transformers provides an [AutoBackbone](/docs/transformers/v5.15.1/en/main_classes/backbones#transformers.AutoBackbone) class for initializing a Transformers backbone from pretrained model weights, and two utility classes:

* [BackboneMixin](/docs/transformers/v5.15.1/en/main_classes/backbones#transformers.BackboneMixin) enables initializing a backbone from Transformers or [timm](https://hf.co/docs/timm/index) and includes functions for returning the output features and indices.
* [BackboneConfigMixin](/docs/transformers/v5.15.1/en/main_classes/backbones#transformers.BackboneConfigMixin) sets the output features and indices of the backbone configuration.

[timm](https://hf.co/docs/timm/index) models are loaded with the [TimmBackbone](/docs/transformers/v5.15.1/en/main_classes/backbones#transformers.TimmBackbone) and [TimmBackboneConfig](/docs/transformers/v5.15.1/en/main_classes/backbones#transformers.TimmBackboneConfig) classes.

Backbones are supported for the following models:

* [BEiT](../model_doc/beit)
* [BiT](../model_doc/bit)
* [ConvNext](../model_doc/convnext)
* [ConvNextV2](../model_doc/convnextv2)
* [DiNAT](../model_doc/dinat)
* [DINOV2](../model_doc/dinov2)
* [FocalNet](../model_doc/focalnet)
* [MaskFormer](../model_doc/maskformer)
* [NAT](../model_doc/nat)
* [ResNet](../model_doc/resnet)
* [Swin Transformer](../model_doc/swin)
* [Swin Transformer v2](../model_doc/swinv2)
* [ViTDet](../model_doc/vitdet)

## AutoBackbone[[transformers.AutoBackbone]]

#### transformers.AutoBackbone[[transformers.AutoBackbone]]

```python
transformers.AutoBackbone(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/auto/modeling_auto.py#L2479)

## BackboneMixin[[transformers.BackboneMixin]]

#### transformers.BackboneMixin[[transformers.BackboneMixin]]

```python
transformers.BackboneMixin(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/backbone_utils.py#L181)

#### post_init[[transformers.BackboneMixin.post_init]]

```python
post_init()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/backbone_utils.py#L207)

Override `post_init` to always install capturing hooks, as backbone will ALWAYS capture outputs. We need to do
it in `post_init`, as modules need to be already instantiated.
It avoids some mixups with `torch.compile`, as the first hook installation will need/create a graph break,
which can clash with external user call such as `model = torch.compile(model...)`.

## BackboneConfigMixin[[transformers.BackboneConfigMixin]]

#### transformers.BackboneConfigMixin[[transformers.BackboneConfigMixin]]

```python
transformers.BackboneConfigMixin()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/backbone_utils.py#L33)

A Mixin to support handling the `out_features` and `out_indices` attributes for the backbone configurations.

#### set_output_features_output_indices[[transformers.BackboneConfigMixin.set_output_features_output_indices]]

```python
set_output_features_output_indices(out_features: list | None, out_indices: list | None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/backbone_utils.py#L38)

**Parameters:**

out_features (`list[str]`, *optional*) : The names of the features for the backbone to output. Defaults to `config._out_features` if not provided.

out_indices (`list[int]` or `tuple[int]`, *optional*) : The indices of the features for the backbone to output. Defaults to `config._out_indices` if not provided.

Sets output indices and features to new values and aligns them with the given `stage_names`.
If one of the inputs is not given, find the corresponding `out_features` or `out_indices`
for the given `stage_names`.

#### to_dict[[transformers.BackboneConfigMixin.to_dict]]

```python
to_dict()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/backbone_utils.py#L147)

Serializes this instance to a Python dictionary. Override the default `to_dict()` from `PreTrainedConfig` to
include the `out_features` and `out_indices` attributes.

#### verify_out_features_out_indices[[transformers.BackboneConfigMixin.verify_out_features_out_indices]]

```python
verify_out_features_out_indices()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/backbone_utils.py#L74)

Verify that out_indices and out_features are valid for the given stage_names.

## TimmBackbone[[transformers.TimmBackbone]]

#### transformers.TimmBackbone[[transformers.TimmBackbone]]

```python
transformers.TimmBackbone(config, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/timm_backbone/modeling_timm_backbone.py#L32)

Wrapper class for timm models to be used as backbones. This enables using the timm models interchangeably with the
other models in the library keeping the same API.

## TimmBackboneConfig[[transformers.TimmBackboneConfig]]

#### transformers.TimmBackboneConfig[[transformers.TimmBackboneConfig]]

```python
transformers.TimmBackboneConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, backbone: str | None = None, num_channels: int = 3, features_only: bool = True, _out_indices: list[int] | None = None, freeze_batch_norm_2d: bool = False, output_stride: int | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/timm_backbone/configuration_timm_backbone.py#L26)

**Parameters:**

backbone (`str`, *optional*) : The timm checkpoint to load.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

features_only (`bool`, *optional*, defaults to `True`) : Whether to output only the features or also the logits.

freeze_batch_norm_2d (`bool`, *optional*, defaults to `False`) : Converts all `BatchNorm2d` and `SyncBatchNorm` layers of provided module into `FrozenBatchNorm2d`.

output_stride (`int`, *optional*) : The ratio between the spatial resolution of the input and output feature maps.

This is the configuration class to store the configuration of a TimmBackbone. It is used to instantiate a Timm Backbone
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [](https://huggingface.co/)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import TimmBackboneConfig, TimmBackbone

>>> # Initializing a timm backbone
>>> configuration = TimmBackboneConfig("resnet50")

>>> # Initializing a model from the configuration
>>> model = TimmBackbone(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```
