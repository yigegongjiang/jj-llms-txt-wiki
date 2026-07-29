# Mask2Former

## Overview

The Mask2Former model was proposed in [Masked-attention Mask Transformer for Universal Image Segmentation](https://huggingface.co/papers/2112.01527) by Bowen Cheng, Ishan Misra, Alexander G. Schwing, Alexander Kirillov, Rohit Girdhar. Mask2Former is a unified framework for panoptic, instance and semantic segmentation and features significant performance and efficiency improvements over [MaskFormer](maskformer).

The abstract from the paper is the following:

*Image segmentation groups pixels with different semantics, e.g., category or instance membership. Each choice
of semantics defines a task. While only the semantics of each task differ, current research focuses on designing specialized architectures for each task. We present Masked-attention Mask Transformer (Mask2Former), a new architecture capable of addressing any image segmentation task (panoptic, instance or semantic). Its key components include masked attention, which extracts localized features by constraining cross-attention within predicted mask regions. In addition to reducing the research effort by at least three times, it outperforms the best specialized architectures by a significant margin on four popular datasets. Most notably, Mask2Former sets a new state-of-the-art for panoptic segmentation (57.8 PQ on COCO), instance segmentation (50.1 AP on COCO) and semantic segmentation (57.7 mIoU on ADE20K).*

 Mask2Former architecture. Taken from the original paper. 

This model was contributed by [Shivalika Singh](https://huggingface.co/shivi) and [Alara Dirik](https://huggingface.co/adirik). The original code can be found [here](https://github.com/facebookresearch/Mask2Former).

## Usage tips

- Mask2Former uses the same preprocessing and postprocessing steps as [MaskFormer](maskformer). Use [Mask2FormerImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerImageProcessor) or [AutoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoImageProcessor) to prepare images and optional targets for the model.
- To get the final segmentation, depending on the task, you can call [post_process_semantic_segmentation()](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerImageProcessor.post_process_semantic_segmentation) or [post_process_instance_segmentation()](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerImageProcessor.post_process_instance_segmentation) or [post_process_panoptic_segmentation()](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerImageProcessor.post_process_panoptic_segmentation). All three tasks can be solved using [Mask2FormerForUniversalSegmentation](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerForUniversalSegmentation) output, panoptic segmentation accepts an optional `label_ids_to_fuse` argument to fuse instances of the target object/s (e.g. sky) together.

## Resources

A list of official Hugging Face and community (indicated by 🌎) resources to help you get started with Mask2Former.

- Demo notebooks regarding inference + fine-tuning Mask2Former on custom data can be found [here](https://github.com/NielsRogge/Transformers-Tutorials/tree/master/Mask2Former).
- Scripts for finetuning `Mask2Former` with [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer) or [Accelerate](https://huggingface.co/docs/accelerate/index) can be found [here](https://github.com/huggingface/transformers/tree/main/examples/pytorch/instance-segmentation).

If you're interested in submitting a resource to be included here, please feel free to open a Pull Request and we will review it.
The resource should ideally demonstrate something new instead of duplicating an existing resource.

## Mask2FormerConfig[[transformers.Mask2FormerConfig]]

- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **feature_size** (`int`, *optional*, defaults to 256) --
  The features (channels) of the resulting feature maps.
- **mask_feature_size** (`int`, *optional*, defaults to 256) --
  The masks' features size, this value will also be used to specify the Feature Pyramid Network features'
  size.
- **hidden_dim** (`int`, *optional*, defaults to `256`) --
  Dimension of the hidden representations.
- **encoder_feedforward_dim** (`int`, *optional*, defaults to 1024) --
  Dimension of feedforward network for deformable detr encoder used as part of pixel decoder.
- **activation_function** (`str`, *optional*, defaults to `relu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **encoder_layers** (`int`, *optional*, defaults to `6`) --
  Number of hidden layers in the Transformer encoder. Will use the same value as `num_layers` if not set.
- **decoder_layers** (`int`, *optional*, defaults to `10`) --
  Number of hidden layers in the Transformer decoder. Will use the same value as `num_layers` if not set.
- **num_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The ratio for all dropout layers.
- **dim_feedforward** (`int`, *optional*, defaults to 2048) --
  Feature dimension in feedforward network for transformer decoder.
- **pre_norm** (`bool`, *optional*, defaults to `False`) --
  Whether to use pre-LayerNorm or not for transformer decoder.
- **enforce_input_projection** (`bool`, *optional*, defaults to `False`) --
  Whether to add an input projection 1x1 convolution even if the input channels and hidden dim are identical
  in the Transformer decoder.
- **common_stride** (`int`, *optional*, defaults to 4) --
  Parameter used for determining number of FPN levels used as part of pixel decoder.
- **ignore_value** (`int`, *optional*, defaults to 255) --
  Category id to be ignored during training.
- **num_queries** (`int`, *optional*, defaults to 100) --
  Number of queries for the decoder.
- **no_object_weight** (`float`, *optional*, defaults to `0.1`) --
  Relative classification weight of the no-object class in the object detection loss.
- **class_weight** (`float`, *optional*, defaults to `2.0`) --
  Relative weight of the classification error in the Hungarian matching cost.
- **mask_weight** (`float`, *optional*, defaults to `5.0`) --
  Relative weight of the focal loss in the panoptic segmentation loss.
- **dice_weight** (`float`, *optional*, defaults to `5.0`) --
  Relative weight of the dice loss in the panoptic segmentation loss.
- **train_num_points** (`str` or `function`, *optional*, defaults to 12544) --
  Number of points used for sampling during loss calculation.
- **oversample_ratio** (`float`, *optional*, defaults to 3.0) --
  Oversampling parameter used for calculating no. of sampled points
- **importance_sample_ratio** (`float`, *optional*, defaults to 0.75) --
  Ratio of points that are sampled via importance sampling.
- **init_std** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **init_xavier_std** (`float`, *optional*, defaults to `1.0`) --
  The scaling factor used for the Xavier initialization of the cross-attention weights.
- **use_auxiliary_loss** (`bool`, *optional*, defaults to `True`) --
  Whether to calculate loss using intermediate predictions from transformer decoder.
- **feature_strides** (`list[int]`, *optional*, defaults to `[4, 8, 16, 32]`) --
  Feature strides corresponding to features generated from backbone network.
- **output_auxiliary_logits** (`bool`, *optional*) --
  Should the model output its `auxiliary_logits` or not.

This is the configuration class to store the configuration of a Mask2FormerModel. It is used to instantiate a Mask2Former
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/mask2former-swin-small-coco-instance](https://huggingface.co/facebook/mask2former-swin-small-coco-instance)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import Mask2FormerConfig, Mask2FormerModel

>>> # Initializing a Mask2Former facebook/mask2former-swin-small-coco-instance configuration
>>> configuration = Mask2FormerConfig()

>>> # Initializing a model (with random weights) from the facebook/mask2former-swin-small-coco-instance style configuration
>>> model = Mask2FormerModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## MaskFormer specific outputs[[transformers.models.mask2former.modeling_mask2former.Mask2FormerModelOutput]]

- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`, *optional*) --
  Last hidden states (final feature map) of the last stage of the encoder model (backbone). Returned when
  `output_hidden_states=True` is passed.
- **pixel_decoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`, *optional*) --
  Last hidden states (final feature map) of the last stage of the pixel decoder model.
- **transformer_decoder_last_hidden_state** (`tuple(torch.FloatTensor)`) --
  Final output of the transformer decoder `(batch_size, sequence_length, hidden_size)`.
- **encoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*) --
  Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the encoder
  model at the output of each stage. Returned when `output_hidden_states=True` is passed.
- **pixel_decoder_hidden_states** (`tuple(torch.FloatTensor)`, , *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) --
  Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the pixel
  decoder model at the output of each stage. Returned when `output_hidden_states=True` is passed.
- **transformer_decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*) --
  Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, sequence_length, hidden_size)`. Hidden-states (also called feature maps) of the
  transformer decoder at the output of each stage. Returned when `output_hidden_states=True` is passed.
- **transformer_decoder_intermediate_states** (`tuple(torch.FloatTensor)` of shape `(num_queries, 1, hidden_size)`) --
  Intermediate decoder activations, i.e. the output of each decoder layer, each of them gone through a
  layernorm.
- **masks_queries_logits** (`tuple(torch.FloatTensor)` of shape `(batch_size, num_queries, height, width)`) --
  Mask Predictions from each layer in the transformer decoder.
- **attentions** (`tuple(tuple(torch.FloatTensor))`, *optional*, returned when `output_attentions=True` is passed) --
  Tuple of `tuple(torch.FloatTensor)` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`. Self attentions weights from transformer decoder.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) --
  Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Class for outputs of [Mask2FormerModel](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerModel). This class returns all the needed hidden states to compute the logits.

- **loss** (`torch.Tensor`, *optional*) --
  The computed loss, returned when labels are present.
- **class_queries_logits** (`torch.FloatTensor`, *optional*) --
  A tensor of shape `(batch_size, num_queries, num_labels + 1)` representing the proposed classes for each
  query. Note the `+ 1` is needed because we incorporate the null class.
- **masks_queries_logits** (`torch.FloatTensor`, *optional*) --
  A tensor of shape `(batch_size, num_queries, height, width)` representing the proposed masks for each
  query.
- **auxiliary_logits** (`list[Dict(str, torch.FloatTensor)]`, *optional*) --
  List of class and mask predictions from each layer of the transformer decoder.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) --
  Last hidden states (final feature map) of the last stage of the encoder model (backbone).
- **pixel_decoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) --
  Last hidden states (final feature map) of the last stage of the pixel decoder model.
- **transformer_decoder_last_hidden_state** (`tuple(torch.FloatTensor)`) --
  Final output of the transformer decoder `(batch_size, sequence_length, hidden_size)`.
- **encoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) --
  Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the encoder
  model at the output of each stage.
- **pixel_decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) --
  Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the pixel
  decoder model at the output of each stage.
- **transformer_decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) --
  Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, sequence_length, hidden_size)`. Hidden-states (also called feature maps) of the
  transformer decoder at the output of each stage.
- **attentions** (`tuple(tuple(torch.FloatTensor))`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) --
  Tuple of `tuple(torch.FloatTensor)` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`. Self and Cross Attentions weights from transformer decoder.

Class for outputs of `Mask2FormerForUniversalSegmentationOutput`.

This output can be directly passed to [post_process_semantic_segmentation()](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerImageProcessor.post_process_semantic_segmentation) or
[post_process_instance_segmentation()](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerImageProcessor.post_process_instance_segmentation) or
[post_process_panoptic_segmentation()](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerImageProcessor.post_process_panoptic_segmentation) to compute final segmentation maps. Please, see
[`~Mask2FormerImageProcessor] for details regarding usage.

## Mask2FormerModel[[transformers.Mask2FormerModel]]

- **config** ([Mask2FormerConfig](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Mask2Former Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "pixel_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "output_hidden_states", "val": ": bool | None = None"}, {"name": "output_attentions", "val": ": bool | None = None"}, {"name": "return_dict", "val": ": bool | None = None"}, {"name": "**kwargs", "val": ""}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Mask2FormerImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerImageProcessor). See `Mask2FormerImageProcessor.__call__()` for details (`processor_class` uses
  [Mask2FormerImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerImageProcessor) for processing images).
- **pixel_mask** (`torch.Tensor` of shape `(batch_size, height, width)`, *optional*) --
  Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:

  - 1 for pixels that are real (i.e. **not masked**),
  - 0 for pixels that are padding (i.e. **masked**).

  [What are attention masks?](../glossary#attention-mask)
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.[Mask2FormerModelOutput](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.models.mask2former.modeling_mask2former.Mask2FormerModelOutput) or `tuple(torch.FloatTensor)`A [Mask2FormerModelOutput](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.models.mask2former.modeling_mask2former.Mask2FormerModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Mask2FormerConfig](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerConfig)) and inputs.
The [Mask2FormerModel](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`, *optional*) -- Last hidden states (final feature map) of the last stage of the encoder model (backbone). Returned when
  `output_hidden_states=True` is passed.
- **pixel_decoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`, *optional*) -- Last hidden states (final feature map) of the last stage of the pixel decoder model.
- **transformer_decoder_last_hidden_state** (`tuple(torch.FloatTensor)`) -- Final output of the transformer decoder `(batch_size, sequence_length, hidden_size)`.
- **encoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the encoder
  model at the output of each stage. Returned when `output_hidden_states=True` is passed.
- **pixel_decoder_hidden_states** (`tuple(torch.FloatTensor)`, , *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the pixel
  decoder model at the output of each stage. Returned when `output_hidden_states=True` is passed.
- **transformer_decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, sequence_length, hidden_size)`. Hidden-states (also called feature maps) of the
  transformer decoder at the output of each stage. Returned when `output_hidden_states=True` is passed.
- **transformer_decoder_intermediate_states** (`tuple(torch.FloatTensor)` of shape `(num_queries, 1, hidden_size)`) -- Intermediate decoder activations, i.e. the output of each decoder layer, each of them gone through a
  layernorm.
- **masks_queries_logits** (`tuple(torch.FloatTensor)` of shape `(batch_size, num_queries, height, width)`)
  Mask Predictions from each layer in the transformer decoder.
  attentions (`tuple(tuple(torch.FloatTensor))`, *optional*, returned when `output_attentions=True` is passed) -- Tuple of `tuple(torch.FloatTensor)` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`. Self attentions weights from transformer decoder.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

## Mask2FormerForUniversalSegmentation[[transformers.Mask2FormerForUniversalSegmentation]]

- **config** ([Mask2FormerConfig](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Mask2Former Model with heads on top for instance/semantic/panoptic segmentation.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "mask_labels", "val": ": list[)>] | None = None"}, {"name": "class_labels", "val": ": list[)>] | None = None"}, {"name": "pixel_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "output_hidden_states", "val": ": bool | None = None"}, {"name": "output_auxiliary_logits", "val": ": bool | None = None"}, {"name": "output_attentions", "val": ": bool | None = None"}, {"name": "return_dict", "val": ": bool | None = None"}, {"name": "**kwargs", "val": ""}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Mask2FormerImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerImageProcessor). See `Mask2FormerImageProcessor.__call__()` for details (`processor_class` uses
  [Mask2FormerImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerImageProcessor) for processing images).
- **mask_labels** (`list[torch.Tensor]`, *optional*) --
  List of mask labels of shape `(num_labels, height, width)` to be fed to a model
- **class_labels** (`list[torch.LongTensor]`, *optional*) --
  list of target class labels of shape `(num_labels, height, width)` to be fed to a model. They identify the
  labels of `mask_labels`, e.g. the label of `mask_labels[i][j]` if `class_labels[i][j]`.
- **pixel_mask** (`torch.Tensor` of shape `(batch_size, height, width)`, *optional*) --
  Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:

  - 1 for pixels that are real (i.e. **not masked**),
  - 0 for pixels that are padding (i.e. **masked**).

  [What are attention masks?](../glossary#attention-mask)
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **output_auxiliary_logits** (`bool`, *optional*) --
  Whether or not to output auxiliary logits.
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.[Mask2FormerForUniversalSegmentationOutput](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.models.mask2former.modeling_mask2former.Mask2FormerForUniversalSegmentationOutput) or `tuple(torch.FloatTensor)`A [Mask2FormerForUniversalSegmentationOutput](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.models.mask2former.modeling_mask2former.Mask2FormerForUniversalSegmentationOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Mask2FormerConfig](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerConfig)) and inputs.
The [Mask2FormerForUniversalSegmentation](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerForUniversalSegmentation) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.Tensor`, *optional*) -- The computed loss, returned when labels are present.
- **class_queries_logits** (`torch.FloatTensor`, *optional*) -- A tensor of shape `(batch_size, num_queries, num_labels + 1)` representing the proposed classes for each
  query. Note the `+ 1` is needed because we incorporate the null class.
- **masks_queries_logits** (`torch.FloatTensor`, *optional*) -- A tensor of shape `(batch_size, num_queries, height, width)` representing the proposed masks for each
  query.
- **auxiliary_logits** (`list[Dict(str, torch.FloatTensor)]`, *optional*) -- List of class and mask predictions from each layer of the transformer decoder.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Last hidden states (final feature map) of the last stage of the encoder model (backbone).
- **pixel_decoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Last hidden states (final feature map) of the last stage of the pixel decoder model.
- **transformer_decoder_last_hidden_state** (`tuple(torch.FloatTensor)`) -- Final output of the transformer decoder `(batch_size, sequence_length, hidden_size)`.
- **encoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the encoder
  model at the output of each stage.
- **pixel_decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the pixel
  decoder model at the output of each stage.
- **transformer_decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, sequence_length, hidden_size)`. Hidden-states (also called feature maps) of the
  transformer decoder at the output of each stage.
- **attentions** (`tuple(tuple(torch.FloatTensor))`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `tuple(torch.FloatTensor)` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`. Self and Cross Attentions weights from transformer decoder.

Examples:

Instance segmentation example:

```python
>>> from transformers import AutoImageProcessor, Mask2FormerForUniversalSegmentation
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> import torch

>>> # Load Mask2Former trained on COCO instance segmentation dataset
>>> image_processor = AutoImageProcessor.from_pretrained("facebook/mask2former-swin-small-coco-instance")
>>> model = Mask2FormerForUniversalSegmentation.from_pretrained(
...     "facebook/mask2former-swin-small-coco-instance"
... )

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))
>>> inputs = image_processor(image, return_tensors="pt")

>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> # Model predicts class_queries_logits of shape `(batch_size, num_queries)`
>>> # and masks_queries_logits of shape `(batch_size, num_queries, height, width)`
>>> class_queries_logits = outputs.class_queries_logits
>>> masks_queries_logits = outputs.masks_queries_logits

>>> # Perform post-processing to get instance segmentation map
>>> pred_instance_map = image_processor.post_process_instance_segmentation(
...     outputs, target_sizes=[(image.height, image.width)]
... )[0]
>>> print(pred_instance_map.shape)
torch.Size([480, 640])
```

Semantic segmentation example:
```python
>>> from transformers import AutoImageProcessor, Mask2FormerForUniversalSegmentation
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> import torch

>>> # Load Mask2Former trained on ADE20k semantic segmentation dataset
>>> image_processor = AutoImageProcessor.from_pretrained("facebook/mask2former-swin-small-ade-semantic")
>>> model = Mask2FormerForUniversalSegmentation.from_pretrained("facebook/mask2former-swin-small-ade-semantic")

>>> url = (
...     "https://huggingface.co/datasets/hf-internal-testing/fixtures_ade20k/resolve/main/ADE_val_00000001.jpg"
... )
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))
>>> inputs = image_processor(image, return_tensors="pt")

>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> # Model predicts class_queries_logits of shape `(batch_size, num_queries)`
>>> # and masks_queries_logits of shape `(batch_size, num_queries, height, width)`
>>> class_queries_logits = outputs.class_queries_logits
>>> masks_queries_logits = outputs.masks_queries_logits

>>> # Perform post-processing to get semantic segmentation map
>>> pred_semantic_map = image_processor.post_process_semantic_segmentation(
...     outputs, target_sizes=[(image.height, image.width)]
... )[0]
>>> print(pred_semantic_map.shape)
torch.Size([512, 683])
```

Panoptic segmentation example:

```python
>>> from transformers import AutoImageProcessor, Mask2FormerForUniversalSegmentation
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> import torch

>>> # Load Mask2Former trained on CityScapes panoptic segmentation dataset
>>> image_processor = AutoImageProcessor.from_pretrained("facebook/mask2former-swin-small-cityscapes-panoptic")
>>> model = Mask2FormerForUniversalSegmentation.from_pretrained(
...     "facebook/mask2former-swin-small-cityscapes-panoptic"
... )

>>> url = "https://cdn-media.huggingface.co/Inference-API/Sample-results-on-the-Cityscapes-dataset-The-above-images-show-how-our-method-can-handle.png"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))
>>> inputs = image_processor(image, return_tensors="pt")

>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> # Model predicts class_queries_logits of shape `(batch_size, num_queries)`
>>> # and masks_queries_logits of shape `(batch_size, num_queries, height, width)`
>>> class_queries_logits = outputs.class_queries_logits
>>> masks_queries_logits = outputs.masks_queries_logits

>>> # Perform post-processing to get panoptic segmentation map
>>> pred_panoptic_map = image_processor.post_process_panoptic_segmentation(
...     outputs, target_sizes=[(image.height, image.width)]
... )[0]["segmentation"]
>>> print(pred_panoptic_map.shape)
torch.Size([338, 676])
```

## Mask2FormerImageProcessor[[transformers.Mask2FormerImageProcessor]]

- **pad_size** (`SizeDict`, *kwargs*, *optional*) --
  The size to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size`
  is not provided, images will be padded to the largest height and width in the batch.
- **ignore_index** (`int`, *kwargs*, *optional*) --
  Label to be assigned to background pixels in segmentation maps. If provided, segmentation map pixels
  denoted with 0 (background) will be replaced with `ignore_index`.
- **do_reduce_labels** (`bool`, *kwargs*, *optional*, defaults to `False`) --
  Whether or not to decrement all label values of segmentation maps by 1. Usually used for datasets where 0
  is used for background, and background itself is not included in all classes of a dataset (e.g. ADE20k).
  The background label will be replaced by `ignore_index`.
- **num_labels** (`int`, *kwargs*, *optional*) --
  The number of labels in the segmentation map.
- **size_divisor** (`int`, *kwargs*, *optional*, defaults to `32`) --
  Some backbones need images divisible by a certain number. If not passed, it defaults to the value used in
  Swin Transformer.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a Mask2FormerImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **segmentation_maps** (`ImageInput`, *optional*) --
  The segmentation maps.
- **instance_id_to_semantic_id** (`Union[list[dict[int, int]], dict[int, int]]`, *optional*) --
  A mapping from instance IDs to semantic IDs.
- **pad_size** (`SizeDict`, *kwargs*, *optional*) --
  The size to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size`
  is not provided, images will be padded to the largest height and width in the batch.
- **ignore_index** (`int`, *kwargs*, *optional*) --
  Label to be assigned to background pixels in segmentation maps. If provided, segmentation map pixels
  denoted with 0 (background) will be replaced with `ignore_index`.
- **do_reduce_labels** (`bool`, *kwargs*, *optional*, defaults to `False`) --
  Whether or not to decrement all label values of segmentation maps by 1. Usually used for datasets where 0
  is used for background, and background itself is not included in all classes of a dataset (e.g. ADE20k).
  The background label will be replaced by `ignore_index`.
- **num_labels** (`int`, *kwargs*, *optional*) --
  The number of labels in the segmentation map.
- **size_divisor** (`int`, *kwargs*, *optional*, defaults to `32`) --
  Some backbones need images divisible by a certain number. If not passed, it defaults to the value used in
  Swin Transformer.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** ([Mask2FormerForUniversalSegmentation](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerForUniversalSegmentation)) --
  Raw outputs of the model.
- **target_sizes** (`list[tuple[int, int]]`, *optional*) --
  List of length (batch_size), where each list item (`tuple[int, int]]`) corresponds to the requested
  final size (height, width) of each prediction. If left to None, predictions will not be resized.
- **return_segmentation_scores** (`bool`, *optional*, defaults to `False`) --
  Whether to return segmentation scores alongside the segmentation map. When `True`, each element of
  the returned list is a `SemanticSegmentationPostProcessorOutput` with fields `segmentation`
  (class IDs, shape `(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`).`list[torch.Tensor]` or `list[SemanticSegmentationPostProcessorOutput]`When
`return_segmentation_scores=False` (default), a list of length `batch_size` where each item is a
segmentation map of shape `(height, width)` with class IDs. When `return_segmentation_scores=True`,
a list of `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape
`(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`). In both cases,
`(height, width)` corresponds to the target size (if `target_sizes` is specified).

Converts the output of [Mask2FormerForUniversalSegmentation](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerForUniversalSegmentation) into semantic segmentation maps. Only supports
PyTorch.

- **outputs** ([Mask2FormerForUniversalSegmentation](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerForUniversalSegmentation)) --
  Raw outputs of the model.
- **threshold** (`float`, *optional*, defaults to 0.5) --
  The probability score threshold to keep predicted instance masks.
- **mask_threshold** (`float`, *optional*, defaults to 0.5) --
  Threshold to use when turning the predicted masks into binary values.
- **overlap_mask_area_threshold** (`float`, *optional*, defaults to 0.8) --
  The overlap mask area threshold to merge or discard small disconnected parts within each binary
  instance mask.
- **target_sizes** (`List[Tuple]`, *optional*) --
  List of length (batch_size), where each list item (`Tuple[int, int]]`) corresponds to the requested
  final size (height, width) of each prediction. If left to None, predictions will not be resized.
- **return_coco_annotation** (`bool`, *optional*, defaults to `False`) --
  If set to `True`, segmentation maps are returned in COCO run-length encoding (RLE) format.
- **return_binary_maps** (`bool`, *optional*, defaults to `False`) --
  If set to `True`, segmentation maps are returned as a concatenated tensor of binary segmentation maps
  (one per detected instance).`List[Dict]`A list of dictionaries, one per image, each dictionary containing two keys:
- **segmentation** -- A tensor of shape `(height, width)` where each pixel represents a `segment_id`, or
  `List[List]` run-length encoding (RLE) of the segmentation map if return_coco_annotation is set to
  `True`, or a tensor of shape `(num_instances, height, width)` if return_binary_maps is set to `True`.
  Set to `None` if no mask if found above `threshold`.
- **segments_info** -- A dictionary that contains additional information on each segment.
  - **id** -- An integer representing the `segment_id`.
  - **label_id** -- An integer representing the label / semantic class id corresponding to `segment_id`.
  - **score** -- Prediction score of segment with `segment_id`.

Converts the output of `Mask2FormerForUniversalSegmentationOutput` into instance segmentation predictions.
Only supports PyTorch. If instances could overlap, set either return_coco_annotation or return_binary_maps
to `True` to get the correct segmentation result.

- **outputs** (`Mask2FormerForUniversalSegmentationOutput`) --
  The outputs from [Mask2FormerForUniversalSegmentation](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerForUniversalSegmentation).
- **threshold** (`float`, *optional*, defaults to 0.5) --
  The probability score threshold to keep predicted instance masks.
- **mask_threshold** (`float`, *optional*, defaults to 0.5) --
  Threshold to use when turning the predicted masks into binary values.
- **overlap_mask_area_threshold** (`float`, *optional*, defaults to 0.8) --
  The overlap mask area threshold to merge or discard small disconnected parts within each binary
  instance mask.
- **label_ids_to_fuse** (`Set[int]`, *optional*) --
  The labels in this state will have all their instances be fused together. For instance we could say
  there can only be one sky in an image, but several persons, so the label ID for sky would be in that
  set, but not the one for person.
- **target_sizes** (`List[Tuple]`, *optional*) --
  List of length (batch_size), where each list item (`Tuple[int, int]]`) corresponds to the requested
  final size (height, width) of each prediction in batch. If left to None, predictions will not be
  resized.`List[Dict]`A list of dictionaries, one per image, each dictionary containing two keys:
- **segmentation** -- a tensor of shape `(height, width)` where each pixel represents a `segment_id`, set
  to `None` if no mask if found above `threshold`. If `target_sizes` is specified, segmentation is resized
  to the corresponding `target_sizes` entry.
- **segments_info** -- A dictionary that contains additional information on each segment.
  - **id** -- an integer representing the `segment_id`.
  - **label_id** -- An integer representing the label / semantic class id corresponding to `segment_id`.
  - **was_fused** -- a boolean, `True` if `label_id` was in `label_ids_to_fuse`, `False` otherwise.
    Multiple instances of the same class / label were fused and assigned a single `segment_id`.
  - **score** -- Prediction score of segment with `segment_id`.

Converts the output of `Mask2FormerForUniversalSegmentationOutput` into image panoptic segmentation
predictions. Only supports PyTorch.

## Mask2FormerImageProcessorPil[[transformers.Mask2FormerImageProcessorPil]]

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **segmentation_maps** (`ImageInput`, *optional*) --
  The segmentation maps.
- **instance_id_to_semantic_id** (`Union[list[dict[int, int]], dict[int, int]]`, *optional*) --
  A mapping from instance IDs to semantic IDs.
- **pad_size** (`SizeDict`, *kwargs*, *optional*) --
  The size to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size`
  is not provided, images will be padded to the largest height and width in the batch.
- **ignore_index** (`int`, *kwargs*, *optional*) --
  Label to be assigned to background pixels in segmentation maps. If provided, segmentation map pixels
  denoted with 0 (background) will be replaced with `ignore_index`.
- **do_reduce_labels** (`bool`, *kwargs*, *optional*, defaults to `False`) --
  Whether or not to decrement all label values of segmentation maps by 1. Usually used for datasets where 0
  is used for background, and background itself is not included in all classes of a dataset (e.g. ADE20k).
  The background label will be replaced by `ignore_index`.
- **num_labels** (`int`, *kwargs*, *optional*) --
  The number of labels in the segmentation map.
- **size_divisor** (`int`, *kwargs*, *optional*, defaults to `32`) --
  Some backbones need images divisible by a certain number. If not passed, it defaults to the value used in
  Swin Transformer.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** ([Mask2FormerForUniversalSegmentation](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerForUniversalSegmentation)) --
  Raw outputs of the model.
- **target_sizes** (`list[tuple[int, int]]`, *optional*) --
  List of length (batch_size), where each list item (`tuple[int, int]]`) corresponds to the requested
  final size (height, width) of each prediction. If left to None, predictions will not be resized.
- **return_segmentation_scores** (`bool`, *optional*, defaults to `False`) --
  Whether to return segmentation scores alongside the segmentation map. When `True`, each element of
  the returned list is a `SemanticSegmentationPostProcessorOutput` with fields `segmentation`
  (class IDs, shape `(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`).`list[torch.Tensor]` or `list[SemanticSegmentationPostProcessorOutput]`When
`return_segmentation_scores=False` (default), a list of length `batch_size` where each item is a
segmentation map of shape `(height, width)` with class IDs. When `return_segmentation_scores=True`,
a list of `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape
`(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`). In both cases,
`(height, width)` corresponds to the target size (if `target_sizes` is specified).

Converts the output of [Mask2FormerForUniversalSegmentation](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerForUniversalSegmentation) into semantic segmentation maps. Only supports
PyTorch.

- **outputs** ([Mask2FormerForUniversalSegmentation](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerForUniversalSegmentation)) --
  Raw outputs of the model.
- **threshold** (`float`, *optional*, defaults to 0.5) --
  The probability score threshold to keep predicted instance masks.
- **mask_threshold** (`float`, *optional*, defaults to 0.5) --
  Threshold to use when turning the predicted masks into binary values.
- **overlap_mask_area_threshold** (`float`, *optional*, defaults to 0.8) --
  The overlap mask area threshold to merge or discard small disconnected parts within each binary
  instance mask.
- **target_sizes** (`List[Tuple]`, *optional*) --
  List of length (batch_size), where each list item (`Tuple[int, int]]`) corresponds to the requested
  final size (height, width) of each prediction. If left to None, predictions will not be resized.
- **return_coco_annotation** (`bool`, *optional*, defaults to `False`) --
  If set to `True`, segmentation maps are returned in COCO run-length encoding (RLE) format.
- **return_binary_maps** (`bool`, *optional*, defaults to `False`) --
  If set to `True`, segmentation maps are returned as a concatenated tensor of binary segmentation maps
  (one per detected instance).`List[Dict]`A list of dictionaries, one per image, each dictionary containing two keys:
- **segmentation** -- A tensor of shape `(height, width)` where each pixel represents a `segment_id`, or
  `List[List]` run-length encoding (RLE) of the segmentation map if return_coco_annotation is set to
  `True`, or a tensor of shape `(num_instances, height, width)` if return_binary_maps is set to `True`.
  Set to `None` if no mask if found above `threshold`.
- **segments_info** -- A dictionary that contains additional information on each segment.
  - **id** -- An integer representing the `segment_id`.
  - **label_id** -- An integer representing the label / semantic class id corresponding to `segment_id`.
  - **score** -- Prediction score of segment with `segment_id`.

Converts the output of `Mask2FormerForUniversalSegmentationOutput` into instance segmentation predictions.
Only supports PyTorch. If instances could overlap, set either return_coco_annotation or return_binary_maps
to `True` to get the correct segmentation result.

- **outputs** (`Mask2FormerForUniversalSegmentationOutput`) --
  The outputs from [Mask2FormerForUniversalSegmentation](/docs/transformers/v5.14.0/en/model_doc/mask2former#transformers.Mask2FormerForUniversalSegmentation).
- **threshold** (`float`, *optional*, defaults to 0.5) --
  The probability score threshold to keep predicted instance masks.
- **mask_threshold** (`float`, *optional*, defaults to 0.5) --
  Threshold to use when turning the predicted masks into binary values.
- **overlap_mask_area_threshold** (`float`, *optional*, defaults to 0.8) --
  The overlap mask area threshold to merge or discard small disconnected parts within each binary
  instance mask.
- **label_ids_to_fuse** (`Set[int]`, *optional*) --
  The labels in this state will have all their instances be fused together. For instance we could say
  there can only be one sky in an image, but several persons, so the label ID for sky would be in that
  set, but not the one for person.
- **target_sizes** (`List[Tuple]`, *optional*) --
  List of length (batch_size), where each list item (`Tuple[int, int]]`) corresponds to the requested
  final size (height, width) of each prediction in batch. If left to None, predictions will not be
  resized.`List[Dict]`A list of dictionaries, one per image, each dictionary containing two keys:
- **segmentation** -- a tensor of shape `(height, width)` where each pixel represents a `segment_id`, set
  to `None` if no mask if found above `threshold`. If `target_sizes` is specified, segmentation is resized
  to the corresponding `target_sizes` entry.
- **segments_info** -- A dictionary that contains additional information on each segment.
  - **id** -- an integer representing the `segment_id`.
  - **label_id** -- An integer representing the label / semantic class id corresponding to `segment_id`.
  - **was_fused** -- a boolean, `True` if `label_id` was in `label_ids_to_fuse`, `False` otherwise.
    Multiple instances of the same class / label were fused and assigned a single `segment_id`.
  - **score** -- Prediction score of segment with `segment_id`.

Converts the output of `Mask2FormerForUniversalSegmentationOutput` into image panoptic segmentation
predictions. Only supports PyTorch.
