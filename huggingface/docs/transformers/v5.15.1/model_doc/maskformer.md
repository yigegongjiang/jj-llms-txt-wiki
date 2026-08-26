# MaskFormer

This is a recently introduced model so the API hasn't been tested extensively. There may be some bugs or slight
breaking changes to fix it in the future. If you see something strange, file a [Github Issue](https://github.com/huggingface/transformers/issues/new?assignees=&labels=&template=bug-report.md&title).

## Overview

The MaskFormer model was proposed in [Per-Pixel Classification is Not All You Need for Semantic Segmentation](https://huggingface.co/papers/2107.06278) by Bowen Cheng, Alexander G. Schwing, Alexander Kirillov. MaskFormer addresses semantic segmentation with a mask classification paradigm instead of performing classic pixel-level classification.

The abstract from the paper is the following:

*Modern approaches typically formulate semantic segmentation as a per-pixel classification task, while instance-level segmentation is handled with an alternative mask classification. Our key insight: mask classification is sufficiently general to solve both semantic- and instance-level segmentation tasks in a unified manner using the exact same model, loss, and training procedure. Following this observation, we propose MaskFormer, a simple mask classification model which predicts a set of binary masks, each associated with a single global class label prediction. Overall, the proposed mask classification-based method simplifies the landscape of effective approaches to semantic and panoptic segmentation tasks and shows excellent empirical results. In particular, we observe that MaskFormer outperforms per-pixel classification baselines when the number of classes is large. Our mask classification-based method outperforms both current state-of-the-art semantic (55.6 mIoU on ADE20K) and panoptic segmentation (52.7 PQ on COCO) models.*

The figure below illustrates the architecture of MaskFormer. Taken from the [original paper](https://huggingface.co/papers/2107.06278).

This model was contributed by [francesco](https://huggingface.co/francesco). The original code can be found [here](https://github.com/facebookresearch/MaskFormer).

## Usage tips

- MaskFormer's Transformer decoder is identical to the decoder of [DETR](detr). During training, the authors of DETR did find it helpful to use auxiliary losses in the decoder, especially to help the model output the correct number of objects of each class. If you set the parameter `use_auxiliary_loss` of [MaskFormerConfig](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerConfig) to `True`, then prediction feedforward neural networks and Hungarian losses are added after each decoder layer (with the FFNs sharing parameters).
- If you want to train the model in a distributed environment across multiple nodes, then one should update the
  `get_num_masks` function inside in the `MaskFormerLoss` class of `modeling_maskformer.py`. When training on multiple nodes, this should be
  set to the average number of target masks across all nodes, as can be seen in the original implementation [here](https://github.com/facebookresearch/MaskFormer/blob/da3e60d85fdeedcb31476b5edd7d328826ce56cc/mask_former/modeling/criterion.py#L169).
- One can use [MaskFormerImageProcessor](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerImageProcessor) to prepare images for the model and optional targets for the model.
- To get the final segmentation, depending on the task, you can call [post_process_semantic_segmentation()](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerImageProcessor.post_process_semantic_segmentation) or [post_process_panoptic_segmentation()](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerImageProcessor.post_process_panoptic_segmentation). Both tasks can be solved using [MaskFormerForInstanceSegmentation](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerForInstanceSegmentation) output, panoptic segmentation accepts an optional `label_ids_to_fuse` argument to fuse instances of the target object/s (e.g. sky) together.

## Resources

- All notebooks that illustrate inference as well as fine-tuning on custom data with MaskFormer can be found [here](https://github.com/NielsRogge/Transformers-Tutorials/tree/master/MaskFormer).
- Scripts for finetuning `MaskFormer` with [Trainer](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.Trainer) or [Accelerate](https://huggingface.co/docs/accelerate/index) can be found [here](https://github.com/huggingface/transformers/tree/main/examples/pytorch/instance-segmentation).

## MaskFormer specific outputs[[transformers.models.maskformer.modeling_maskformer.MaskFormerModelOutput]]

#### transformers.models.maskformer.modeling_maskformer.MaskFormerModelOutput[[transformers.models.maskformer.modeling_maskformer.MaskFormerModelOutput]]

```python
transformers.models.maskformer.modeling_maskformer.MaskFormerModelOutput(encoder_last_hidden_state: typing.Optional[torch.FloatTensor] = None, pixel_decoder_last_hidden_state: typing.Optional[torch.FloatTensor] = None, transformer_decoder_last_hidden_state: typing.Optional[torch.FloatTensor] = None, encoder_hidden_states: tuple[torch.FloatTensor] | None = None, pixel_decoder_hidden_states: tuple[torch.FloatTensor] | None = None, transformer_decoder_hidden_states: tuple[torch.FloatTensor] | None = None, hidden_states: tuple[torch.FloatTensor] | None = None, attentions: tuple[torch.FloatTensor] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/maskformer/modeling_maskformer.py#L136)

**Parameters:**

encoder_last_hidden_state (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) : Last hidden states (final feature map) of the last stage of the encoder model (backbone).

pixel_decoder_last_hidden_state (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) : Last hidden states (final feature map) of the last stage of the pixel decoder model (FPN).

transformer_decoder_last_hidden_state (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) : Last hidden states (final feature map) of the last stage of the transformer decoder model.

encoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the encoder model at the output of each stage.

pixel_decoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the pixel decoder model at the output of each stage.

transformer_decoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states (also called feature maps) of the transformer decoder at the output of each stage.

hidden_states `tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` containing `encoder_hidden_states`, `pixel_decoder_hidden_states` and `decoder_hidden_states`

hidden_states (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

attentions (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

Class for outputs of [MaskFormerModel](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerModel). This class returns all the needed hidden states to compute the logits.

#### transformers.models.maskformer.modeling_maskformer.MaskFormerForInstanceSegmentationOutput[[transformers.models.maskformer.modeling_maskformer.MaskFormerForInstanceSegmentationOutput]]

```python
transformers.models.maskformer.modeling_maskformer.MaskFormerForInstanceSegmentationOutput(loss: typing.Optional[torch.FloatTensor] = None, class_queries_logits: typing.Optional[torch.FloatTensor] = None, masks_queries_logits: typing.Optional[torch.FloatTensor] = None, auxiliary_logits: typing.Optional[torch.FloatTensor] = None, encoder_last_hidden_state: typing.Optional[torch.FloatTensor] = None, pixel_decoder_last_hidden_state: typing.Optional[torch.FloatTensor] = None, transformer_decoder_last_hidden_state: typing.Optional[torch.FloatTensor] = None, encoder_hidden_states: tuple[torch.FloatTensor] | None = None, pixel_decoder_hidden_states: tuple[torch.FloatTensor] | None = None, transformer_decoder_hidden_states: tuple[torch.FloatTensor] | None = None, hidden_states: tuple[torch.FloatTensor] | None = None, attentions: tuple[torch.FloatTensor] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/maskformer/modeling_maskformer.py#L182)

**Parameters:**

loss (`torch.Tensor`, *optional*) : The computed loss, returned when labels are present.

class_queries_logits (`torch.FloatTensor`, *optional*) : A tensor of shape `(batch_size, num_queries, num_labels + 1)` representing the proposed classes for each query. Note the `+ 1` is needed because we incorporate the null class.

masks_queries_logits (`torch.FloatTensor`, *optional*) : A tensor of shape `(batch_size, num_queries, height, width)` representing the proposed masks for each query.

auxiliary_logits (`Dict[str, torch.FloatTensor]`, *optional*, returned when `output_auxiliary_logits=True`) : Dictionary containing auxiliary predictions for each decoder layer when auxiliary losses are enabled.

encoder_last_hidden_state (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) : Last hidden states (final feature map) of the last stage of the encoder model (backbone).

pixel_decoder_last_hidden_state (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) : Last hidden states (final feature map) of the last stage of the pixel decoder model (FPN).

transformer_decoder_last_hidden_state (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) : Last hidden states (final feature map) of the last stage of the transformer decoder model.

encoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the encoder model at the output of each stage.

pixel_decoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the pixel decoder model at the output of each stage.

transformer_decoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states of the transformer decoder at the output of each stage.

hidden_states `tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` containing `encoder_hidden_states`, `pixel_decoder_hidden_states` and `decoder_hidden_states`.

hidden_states (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

attentions (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

Class for outputs of [MaskFormerForInstanceSegmentation](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerForInstanceSegmentation).

This output can be directly passed to [post_process_semantic_segmentation()](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerImageProcessor.post_process_semantic_segmentation) or
[post_process_instance_segmentation()](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerImageProcessor.post_process_instance_segmentation) or
[post_process_panoptic_segmentation()](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerImageProcessor.post_process_panoptic_segmentation) depending on the task. Please, see
[`~MaskFormerImageProcessor] for details regarding usage.

## MaskFormerDetrConfig[[transformers.MaskFormerDetrConfig]]

#### transformers.MaskFormerDetrConfig[[transformers.MaskFormerDetrConfig]]

```python
transformers.MaskFormerDetrConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, is_encoder_decoder: bool = True, backbone_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, num_channels: int = 3, num_queries: int = 100, encoder_layers: int = 6, encoder_ffn_dim: int = 2048, encoder_attention_heads: int = 8, decoder_layers: int = 6, decoder_ffn_dim: int = 2048, decoder_attention_heads: int = 8, encoder_layerdrop: float | int = 0.0, decoder_layerdrop: float | int = 0.0, activation_function: str = 'relu', d_model: int = 256, dropout: float | int = 0.1, attention_dropout: float | int = 0.0, activation_dropout: float | int = 0.0, init_std: float = 0.02, init_xavier_std: float = 1.0, auxiliary_loss: bool = False, position_embedding_type: str = 'sine', dilation: bool = False, class_cost: int = 1, bbox_cost: int = 5, giou_cost: int = 2, mask_loss_coefficient: int = 1, dice_loss_coefficient: int = 1, bbox_loss_coefficient: int = 5, giou_loss_coefficient: int = 2, eos_coefficient: float = 0.1)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/maskformer/configuration_maskformer.py#L33)

**Parameters:**

is_encoder_decoder (`bool`, *optional*, defaults to `True`) : Whether the model is used as an encoder/decoder or not.

backbone_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The configuration of the backbone model.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

num_queries (`int`, *optional*, defaults to 100) : Number of object queries, i.e. detection slots. This is the maximal number of objects [ConditionalDetrModel](/docs/transformers/v5.15.1/en/model_doc/conditional_detr#transformers.ConditionalDetrModel) can detect in a single image. For COCO, we recommend 100 queries.

encoder_layers (`int`, *optional*, defaults to `6`) : Number of hidden layers in the Transformer encoder. Will use the same value as `num_layers` if not set.

encoder_ffn_dim (`int`, *optional*, defaults to `2048`) : Dimensionality of the "intermediate" (often named feed-forward) layer in encoder.

encoder_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer encoder.

decoder_layers (`int`, *optional*, defaults to `6`) : Number of hidden layers in the Transformer decoder. Will use the same value as `num_layers` if not set.

decoder_ffn_dim (`int`, *optional*, defaults to `2048`) : Dimensionality of the "intermediate" (often named feed-forward) layer in decoder.

decoder_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer decoder.

encoder_layerdrop (`Union[float, int]`, *optional*, defaults to `0.0`) : The LayerDrop probability for the encoder. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556) for more details.

decoder_layerdrop (`Union[float, int]`, *optional*, defaults to `0.0`) : The LayerDrop probability for the decoder. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556) for more details.

activation_function (`str`, *optional*, defaults to `relu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

d_model (`int`, *optional*, defaults to `256`) : Size of the encoder layers and the pooler layer.

dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The ratio for all dropout layers.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

activation_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for activations inside the fully connected layer.

init_std (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

init_xavier_std (`float`, *optional*, defaults to `1.0`) : The scaling factor used for the Xavier initialization of the cross-attention weights.

auxiliary_loss (`bool`, *optional*, defaults to `False`) : Whether auxiliary decoding losses (losses at each decoder layer) are to be used.

position_embedding_type (`str`, *optional*, defaults to `"sine"`) : Type of position embeddings to be used on top of the image features. One of `"sine"` or `"learned"`.

dilation (`bool`, *optional*, defaults to `False`) : Whether to replace stride with dilation in the last convolutional block (DC5). Only supported when `use_timm_backbone` = `True`.

class_cost (`int`, *optional*, defaults to `1`) : Relative weight of the classification error in the Hungarian matching cost.

bbox_cost (`int`, *optional*, defaults to `5`) : Relative weight of the L1 bounding box error in the Hungarian matching cost.

giou_cost (`int`, *optional*, defaults to `2`) : Relative weight of the generalized IoU loss in the Hungarian matching cost.

mask_loss_coefficient (`int`, *optional*, defaults to `1`) : Relative weight of the focal loss in the panoptic segmentation loss.

dice_loss_coefficient (`int`, *optional*, defaults to `1`) : Relative weight of the dice loss in the panoptic segmentation loss.

bbox_loss_coefficient (`int`, *optional*, defaults to `5`) : Relative weight of the L1 bounding box loss in the panoptic segmentation loss.

giou_loss_coefficient (`int`, *optional*, defaults to `2`) : Relative weight of the generalized IoU loss in the panoptic segmentation loss.

eos_coefficient (`float`, *optional*, defaults to `0.1`) : Relative classification weight of the 'no-object' class in the object detection loss.

This is the configuration class to store the configuration of a MaskFormerModel. It is used to instantiate a Maskformer
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/maskformer-swin-base-ade](https://huggingface.co/facebook/maskformer-swin-base-ade)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import MaskFormerDetrConfig, MaskFormerDetrModel

>>> # Initializing a MASK_FORMER_DETR facebook/mask_former_detr-resnet-50 style configuration
>>> configuration = MaskFormerDetrConfig()

>>> # Initializing a model (with random weights) from the facebook/mask_former_detr-resnet-50 style configuration
>>> model = MaskFormerDetrModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## MaskFormerConfig[[transformers.MaskFormerConfig]]

#### transformers.MaskFormerConfig[[transformers.MaskFormerConfig]]

```python
transformers.MaskFormerConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, fpn_feature_size: int = 256, mask_feature_size: int = 256, no_object_weight: float = 0.1, use_auxiliary_loss: bool = False, backbone_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, decoder_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, init_std: float = 0.02, init_xavier_std: float = 1.0, dice_weight: float = 1.0, cross_entropy_weight: float = 1.0, mask_weight: float = 20.0, output_auxiliary_logits: bool | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/maskformer/configuration_maskformer.py#L123)

**Parameters:**

fpn_feature_size (`int`, *optional*, defaults to 256) : The Feature Pyramid Network's features size.

mask_feature_size (`int`, *optional*, defaults to 256) : The masks' features size, this value will also be used to specify the Feature Pyramid Network features' size.

no_object_weight (`float`, *optional*, defaults to `0.1`) : Relative classification weight of the no-object class in the object detection loss.

use_auxiliary_loss (`bool`, *optional*, defaults to `False`) : Whether to calculate loss using intermediate predictions from transformer decoder.

backbone_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The configuration of the backbone model.

decoder_config (`Dict`, *optional*) : The configuration passed to the transformer decoder model, if unset the base config for `detr-resnet-50` will be used.

init_std (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

init_xavier_std (`float`, *optional*, defaults to `1.0`) : The scaling factor used for the Xavier initialization of the cross-attention weights.

dice_weight (`float`, *optional*, defaults to `1.0`) : Relative weight of the dice loss in the panoptic segmentation loss.

cross_entropy_weight (`float`, *optional*, defaults to 1.0) : The weight for the cross entropy loss.

mask_weight (`float`, *optional*, defaults to `20.0`) : Relative weight of the focal loss in the panoptic segmentation loss.

output_auxiliary_logits (`bool`, *optional*) : Should the model output its `auxiliary_logits` or not.

**Raises:** ``ValueError``

- ``ValueError`` -- 
  Raised if the backbone model type selected is not in `["swin"]` or the decoder model type selected is not
  in `["detr"]`

This is the configuration class to store the configuration of a MaskFormerModel. It is used to instantiate a Maskformer
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/maskformer-swin-base-ade](https://huggingface.co/facebook/maskformer-swin-base-ade)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import MaskFormerConfig, MaskFormerModel

>>> # Initializing a MaskFormer facebook/maskformer-swin-base-ade configuration
>>> configuration = MaskFormerConfig()

>>> # Initializing a model (with random weights) from the facebook/maskformer-swin-base-ade style configuration
>>> model = MaskFormerModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## MaskFormerImageProcessor[[transformers.MaskFormerImageProcessor]]

#### transformers.MaskFormerImageProcessor[[transformers.MaskFormerImageProcessor]]

```python
transformers.MaskFormerImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/maskformer/image_processing_maskformer.py#L257)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'shortest_edge' : 800, 'longest_edge': 1333}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BILINEAR`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.485, 0.456, 0.406]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.229, 0.224, 0.225]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`SizeDict`, *kwargs*, *optional*) : The size to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch.

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

ignore_index (`int`, *kwargs*, *optional*) : Label to be assigned to background pixels in segmentation maps. If provided, segmentation map pixels denoted with 0 (background) will be replaced with `ignore_index`.

do_reduce_labels (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether or not to decrement all label values of segmentation maps by 1. Usually used for datasets where 0 is used for background, and background itself is not included in all classes of a dataset (e.g. ADE20k). The background label will be replaced by `ignore_index`.

num_labels (`int`, *kwargs*, *optional*) : The number of labels in the segmentation map.

size_divisor (`int`, *kwargs*, *optional*, defaults to `32`) : Some backbones need images divisible by a certain number. If not passed, it defaults to the value used in Swin Transformer.

Constructs a MaskFormerImageProcessor image processor.

#### preprocess[[transformers.MaskFormerImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], segmentation_maps: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, instance_id_to_semantic_id: list[dict[int, int]] | dict[int, int] | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/maskformer/image_processing_maskformer.py#L380)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

segmentation_maps (`ImageInput`, *optional*) : The segmentation maps.

instance_id_to_semantic_id (`Union[list[dict[int, int]], dict[int, int]]`, *optional*) : A mapping from instance IDs to semantic IDs.

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`SizeDict`, *kwargs*, *optional*) : The size to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch.

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

ignore_index (`int`, *kwargs*, *optional*) : Label to be assigned to background pixels in segmentation maps. If provided, segmentation map pixels denoted with 0 (background) will be replaced with `ignore_index`.

do_reduce_labels (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether or not to decrement all label values of segmentation maps by 1. Usually used for datasets where 0 is used for background, and background itself is not included in all classes of a dataset (e.g. ADE20k). The background label will be replaced by `ignore_index`.

num_labels (`int`, *kwargs*, *optional*) : The number of labels in the segmentation map.

size_divisor (`int`, *kwargs*, *optional*, defaults to `32`) : Some backbones need images divisible by a certain number. If not passed, it defaults to the value used in Swin Transformer.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

#### post_process_semantic_segmentation[[transformers.MaskFormerImageProcessor.post_process_semantic_segmentation]]

```python
post_process_semantic_segmentation(outputs, target_sizes: list[tuple[int, int]] | None = None, return_segmentation_scores: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/maskformer/image_processing_maskformer.py#L546)

**Parameters:**

outputs ([MaskFormerForInstanceSegmentation](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerForInstanceSegmentation)) : Raw outputs of the model.

target_sizes (`list[tuple[int, int]]`, *optional*) : List of length (batch_size), where each list item (`tuple[int, int]]`) corresponds to the requested final size (height, width) of each prediction. If left to None, predictions will not be resized.

return_segmentation_scores (`bool`, *optional*, defaults to `False`) : Whether to return segmentation scores alongside the segmentation map. When `True`, each element of the returned list is a `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape `(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`).

**Returns:** `list[torch.Tensor]` or `list[SemanticSegmentationPostProcessorOutput]`

When
`return_segmentation_scores=False` (default), a list of length `batch_size` where each item is a
segmentation map of shape `(height, width)` with class IDs. When `return_segmentation_scores=True`,
a list of `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape
`(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`). In both cases,
`(height, width)` corresponds to the target size (if `target_sizes` is specified).

Converts the output of [MaskFormerForInstanceSegmentation](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerForInstanceSegmentation) into semantic segmentation maps. Only supports
PyTorch.

#### post_process_instance_segmentation[[transformers.MaskFormerImageProcessor.post_process_instance_segmentation]]

```python
post_process_instance_segmentation(outputs, threshold: float = 0.5, mask_threshold: float = 0.5, overlap_mask_area_threshold: float = 0.8, target_sizes: list[tuple[int, int]] | None = None, return_coco_annotation: bool | None = False, return_binary_maps: bool | None = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/maskformer/image_processing_maskformer.py#L616)

**Parameters:**

outputs ([MaskFormerForInstanceSegmentation](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerForInstanceSegmentation)) : Raw outputs of the model.

threshold (`float`, *optional*, defaults to 0.5) : The probability score threshold to keep predicted instance masks.

mask_threshold (`float`, *optional*, defaults to 0.5) : Threshold to use when turning the predicted masks into binary values.

overlap_mask_area_threshold (`float`, *optional*, defaults to 0.8) : The overlap mask area threshold to merge or discard small disconnected parts within each binary instance mask.

target_sizes (`list[Tuple]`, *optional*) : List of length (batch_size), where each list item (`tuple[int, int]]`) corresponds to the requested final size (height, width) of each prediction. If left to None, predictions will not be resized.

return_coco_annotation (`bool`, *optional*, defaults to `False`) : If set to `True`, segmentation maps are returned in COCO run-length encoding (RLE) format.

return_binary_maps (`bool`, *optional*, defaults to `False`) : If set to `True`, segmentation maps are returned as a concatenated tensor of binary segmentation maps (one per detected instance).

**Returns:** `list[Dict]`

A list of dictionaries, one per image, each dictionary containing two keys:
- **segmentation** -- A tensor of shape `(height, width)` where each pixel represents a `segment_id`, or
  `list[List]` run-length encoding (RLE) of the segmentation map if return_coco_annotation is set to
  `True`, or a tensor of shape `(num_instances, height, width)` if return_binary_maps is set to `True`.
  Set to `None` if no mask if found above `threshold`.
- **segments_info** -- A dictionary that contains additional information on each segment.
  - **id** -- An integer representing the `segment_id`.
  - **label_id** -- An integer representing the label / semantic class id corresponding to `segment_id`.
  - **score** -- Prediction score of segment with `segment_id`.

Converts the output of `MaskFormerForInstanceSegmentationOutput` into instance segmentation predictions. Only
supports PyTorch. If instances could overlap, set either return_coco_annotation or return_binary_maps
to `True` to get the correct segmentation result.

#### post_process_panoptic_segmentation[[transformers.MaskFormerImageProcessor.post_process_panoptic_segmentation]]

```python
post_process_panoptic_segmentation(outputs, threshold: float = 0.5, mask_threshold: float = 0.5, overlap_mask_area_threshold: float = 0.8, label_ids_to_fuse: set[int] | None = None, target_sizes: list[tuple[int, int]] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/maskformer/image_processing_maskformer.py#L733)

**Parameters:**

outputs (`MaskFormerForInstanceSegmentationOutput`) : The outputs from [MaskFormerForInstanceSegmentation](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerForInstanceSegmentation).

threshold (`float`, *optional*, defaults to 0.5) : The probability score threshold to keep predicted instance masks.

mask_threshold (`float`, *optional*, defaults to 0.5) : Threshold to use when turning the predicted masks into binary values.

overlap_mask_area_threshold (`float`, *optional*, defaults to 0.8) : The overlap mask area threshold to merge or discard small disconnected parts within each binary instance mask.

label_ids_to_fuse (`Set[int]`, *optional*) : The labels in this state will have all their instances be fused together. For instance we could say there can only be one sky in an image, but several persons, so the label ID for sky would be in that set, but not the one for person.

target_sizes (`list[Tuple]`, *optional*) : List of length (batch_size), where each list item (`tuple[int, int]]`) corresponds to the requested final size (height, width) of each prediction in batch. If left to None, predictions will not be resized.

**Returns:** `list[Dict]`

A list of dictionaries, one per image, each dictionary containing two keys:
- **segmentation** -- a tensor of shape `(height, width)` where each pixel represents a `segment_id`, set
  to `None` if no mask if found above `threshold`. If `target_sizes` is specified, segmentation is resized
  to the corresponding `target_sizes` entry.
- **segments_info** -- A dictionary that contains additional information on each segment.
  - **id** -- an integer representing the `segment_id`.
  - **label_id** -- An integer representing the label / semantic class id corresponding to `segment_id`.
  - **was_fused** -- a boolean, `True` if `label_id` was in `label_ids_to_fuse`, `False` otherwise.
    Multiple instances of the same class / label were fused and assigned a single `segment_id`.
  - **score** -- Prediction score of segment with `segment_id`.

Converts the output of `MaskFormerForInstanceSegmentationOutput` into image panoptic segmentation
predictions. Only supports PyTorch.

## MaskFormerImageProcessorPil[[transformers.MaskFormerImageProcessorPil]]

#### transformers.MaskFormerImageProcessorPil[[transformers.MaskFormerImageProcessorPil]]

```python
transformers.MaskFormerImageProcessorPil(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/maskformer/image_processing_pil_maskformer.py#L270)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'shortest_edge' : 800, 'longest_edge': 1333}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BILINEAR`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.485, 0.456, 0.406]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.229, 0.224, 0.225]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`SizeDict`, *kwargs*, *optional*) : The size to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch.

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

ignore_index (`int`, *kwargs*, *optional*) : Label to be assigned to background pixels in segmentation maps. If provided, segmentation map pixels denoted with 0 (background) will be replaced with `ignore_index`.

do_reduce_labels (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether or not to decrement all label values of segmentation maps by 1. Usually used for datasets where 0 is used for background, and background itself is not included in all classes of a dataset (e.g. ADE20k). The background label will be replaced by `ignore_index`.

num_labels (`int`, *kwargs*, *optional*) : The number of labels in the segmentation map.

size_divisor (`int`, *kwargs*, *optional*, defaults to `32`) : Some backbones need images divisible by a certain number. If not passed, it defaults to the value used in Swin Transformer.

Constructs a MaskFormerImageProcessor image processor.

#### preprocess[[transformers.MaskFormerImageProcessorPil.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], segmentation_maps: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, instance_id_to_semantic_id: list[dict[int, int]] | dict[int, int] | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/maskformer/image_processing_pil_maskformer.py#L438)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

segmentation_maps (`ImageInput`, *optional*) : The segmentation maps.

instance_id_to_semantic_id (`Union[list[dict[int, int]], dict[int, int]]`, *optional*) : A mapping from instance IDs to semantic IDs.

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`SizeDict`, *kwargs*, *optional*) : The size to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch.

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

ignore_index (`int`, *kwargs*, *optional*) : Label to be assigned to background pixels in segmentation maps. If provided, segmentation map pixels denoted with 0 (background) will be replaced with `ignore_index`.

do_reduce_labels (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether or not to decrement all label values of segmentation maps by 1. Usually used for datasets where 0 is used for background, and background itself is not included in all classes of a dataset (e.g. ADE20k). The background label will be replaced by `ignore_index`.

num_labels (`int`, *kwargs*, *optional*) : The number of labels in the segmentation map.

size_divisor (`int`, *kwargs*, *optional*, defaults to `32`) : Some backbones need images divisible by a certain number. If not passed, it defaults to the value used in Swin Transformer.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

#### post_process_semantic_segmentation[[transformers.MaskFormerImageProcessorPil.post_process_semantic_segmentation]]

```python
post_process_semantic_segmentation(outputs, target_sizes: list[tuple[int, int]] | None = None, return_segmentation_scores: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/maskformer/image_processing_pil_maskformer.py#L583)

**Parameters:**

outputs ([MaskFormerForInstanceSegmentation](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerForInstanceSegmentation)) : Raw outputs of the model.

target_sizes (`list[tuple[int, int]]`, *optional*) : List of length (batch_size), where each list item (`tuple[int, int]]`) corresponds to the requested final size (height, width) of each prediction. If left to None, predictions will not be resized.

return_segmentation_scores (`bool`, *optional*, defaults to `False`) : Whether to return segmentation scores alongside the segmentation map. When `True`, each element of the returned list is a `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape `(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`).

**Returns:** `list[torch.Tensor]` or `list[SemanticSegmentationPostProcessorOutput]`

When
`return_segmentation_scores=False` (default), a list of length `batch_size` where each item is a
segmentation map of shape `(height, width)` with class IDs. When `return_segmentation_scores=True`,
a list of `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape
`(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`). In both cases,
`(height, width)` corresponds to the target size (if `target_sizes` is specified).

Converts the output of [MaskFormerForInstanceSegmentation](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerForInstanceSegmentation) into semantic segmentation maps. Only supports
PyTorch.

#### post_process_instance_segmentation[[transformers.MaskFormerImageProcessorPil.post_process_instance_segmentation]]

```python
post_process_instance_segmentation(outputs, threshold: float = 0.5, mask_threshold: float = 0.5, overlap_mask_area_threshold: float = 0.8, target_sizes: list[tuple[int, int]] | None = None, return_coco_annotation: bool | None = False, return_binary_maps: bool | None = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/maskformer/image_processing_pil_maskformer.py#L654)

**Parameters:**

outputs ([MaskFormerForInstanceSegmentation](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerForInstanceSegmentation)) : Raw outputs of the model.

threshold (`float`, *optional*, defaults to 0.5) : The probability score threshold to keep predicted instance masks.

mask_threshold (`float`, *optional*, defaults to 0.5) : Threshold to use when turning the predicted masks into binary values.

overlap_mask_area_threshold (`float`, *optional*, defaults to 0.8) : The overlap mask area threshold to merge or discard small disconnected parts within each binary instance mask.

target_sizes (`list[Tuple]`, *optional*) : List of length (batch_size), where each list item (`tuple[int, int]]`) corresponds to the requested final size (height, width) of each prediction. If left to None, predictions will not be resized.

return_coco_annotation (`bool`, *optional*, defaults to `False`) : If set to `True`, segmentation maps are returned in COCO run-length encoding (RLE) format.

return_binary_maps (`bool`, *optional*, defaults to `False`) : If set to `True`, segmentation maps are returned as a concatenated tensor of binary segmentation maps (one per detected instance).

**Returns:** `list[Dict]`

A list of dictionaries, one per image, each dictionary containing two keys:
- **segmentation** -- A tensor of shape `(height, width)` where each pixel represents a `segment_id`, or
  `list[List]` run-length encoding (RLE) of the segmentation map if return_coco_annotation is set to
  `True`, or a tensor of shape `(num_instances, height, width)` if return_binary_maps is set to `True`.
  Set to `None` if no mask if found above `threshold`.
- **segments_info** -- A dictionary that contains additional information on each segment.
  - **id** -- An integer representing the `segment_id`.
  - **label_id** -- An integer representing the label / semantic class id corresponding to `segment_id`.
  - **score** -- Prediction score of segment with `segment_id`.

Converts the output of `MaskFormerForInstanceSegmentationOutput` into instance segmentation predictions. Only
supports PyTorch. If instances could overlap, set either return_coco_annotation or return_binary_maps
to `True` to get the correct segmentation result.

#### post_process_panoptic_segmentation[[transformers.MaskFormerImageProcessorPil.post_process_panoptic_segmentation]]

```python
post_process_panoptic_segmentation(outputs, threshold: float = 0.5, mask_threshold: float = 0.5, overlap_mask_area_threshold: float = 0.8, label_ids_to_fuse: set[int] | None = None, target_sizes: list[tuple[int, int]] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/maskformer/image_processing_pil_maskformer.py#L773)

**Parameters:**

outputs (`MaskFormerForInstanceSegmentationOutput`) : The outputs from [MaskFormerForInstanceSegmentation](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerForInstanceSegmentation).

threshold (`float`, *optional*, defaults to 0.5) : The probability score threshold to keep predicted instance masks.

mask_threshold (`float`, *optional*, defaults to 0.5) : Threshold to use when turning the predicted masks into binary values.

overlap_mask_area_threshold (`float`, *optional*, defaults to 0.8) : The overlap mask area threshold to merge or discard small disconnected parts within each binary instance mask.

label_ids_to_fuse (`Set[int]`, *optional*) : The labels in this state will have all their instances be fused together. For instance we could say there can only be one sky in an image, but several persons, so the label ID for sky would be in that set, but not the one for person.

target_sizes (`list[Tuple]`, *optional*) : List of length (batch_size), where each list item (`tuple[int, int]]`) corresponds to the requested final size (height, width) of each prediction in batch. If left to None, predictions will not be resized.

**Returns:** `list[Dict]`

A list of dictionaries, one per image, each dictionary containing two keys:
- **segmentation** -- a tensor of shape `(height, width)` where each pixel represents a `segment_id`, set
  to `None` if no mask if found above `threshold`. If `target_sizes` is specified, segmentation is resized
  to the corresponding `target_sizes` entry.
- **segments_info** -- A dictionary that contains additional information on each segment.
  - **id** -- an integer representing the `segment_id`.
  - **label_id** -- An integer representing the label / semantic class id corresponding to `segment_id`.
  - **was_fused** -- a boolean, `True` if `label_id` was in `label_ids_to_fuse`, `False` otherwise.
    Multiple instances of the same class / label were fused and assigned a single `segment_id`.
  - **score** -- Prediction score of segment with `segment_id`.

Converts the output of `MaskFormerForInstanceSegmentationOutput` into image panoptic segmentation
predictions. Only supports PyTorch.

## MaskFormerModel[[transformers.MaskFormerModel]]

#### transformers.MaskFormerModel[[transformers.MaskFormerModel]]

```python
transformers.MaskFormerModel(config: MaskFormerConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/maskformer/modeling_maskformer.py#L1739)

**Parameters:**

config ([MaskFormerConfig](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Maskformer Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MaskFormerModel.forward]]

```python
forward(pixel_values: Tensor, pixel_mask: typing.Optional[torch.Tensor] = None, output_hidden_states: bool | None = None, output_attentions: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/maskformer/modeling_maskformer.py#L1749)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [MaskFormerImageProcessor](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerImageProcessor). See `MaskFormerImageProcessor.__call__()` for details (`processor_class` uses [MaskFormerImageProcessor](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerImageProcessor) for processing images).

pixel_mask (`torch.Tensor` of shape `(batch_size, height, width)`, *optional*) : Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:  - 1 for pixels that are real (i.e. **not masked**), - 0 for pixels that are padding (i.e. **masked**).  [What are attention masks?](../glossary#attention-mask)

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [MaskFormerModelOutput](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.models.maskformer.modeling_maskformer.MaskFormerModelOutput) or `tuple(torch.FloatTensor)`

A [MaskFormerModelOutput](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.models.maskformer.modeling_maskformer.MaskFormerModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MaskFormerConfig](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerConfig)) and inputs.

The [MaskFormerModel](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Last hidden states (final feature map) of the last stage of the encoder model (backbone).
- **pixel_decoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Last hidden states (final feature map) of the last stage of the pixel decoder model (FPN).
- **transformer_decoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Last hidden states (final feature map) of the last stage of the transformer decoder model.
- **encoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the encoder
  model at the output of each stage.
- **pixel_decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the pixel
  decoder model at the output of each stage.
- **transformer_decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, sequence_length, hidden_size)`. Hidden-states (also called feature maps) of the
  transformer decoder at the output of each stage.
  hidden_states `tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`):
  Tuple of `torch.FloatTensor` containing `encoder_hidden_states`, `pixel_decoder_hidden_states` and
  `decoder_hidden_states`
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import AutoImageProcessor, MaskFormerModel
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> # load MaskFormer fine-tuned on ADE20k semantic segmentation
>>> image_processor = AutoImageProcessor.from_pretrained("facebook/maskformer-swin-base-ade")
>>> model = MaskFormerModel.from_pretrained("facebook/maskformer-swin-base-ade")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> inputs = image_processor(image, return_tensors="pt")

>>> # forward pass
>>> outputs = model(**inputs)

>>> # the decoder of MaskFormer outputs hidden states of shape (batch_size, num_queries, hidden_size)
>>> transformer_decoder_last_hidden_state = outputs.transformer_decoder_last_hidden_state
>>> list(transformer_decoder_last_hidden_state.shape)
[1, 100, 256]
```

## MaskFormerForInstanceSegmentation[[transformers.MaskFormerForInstanceSegmentation]]

#### transformers.MaskFormerForInstanceSegmentation[[transformers.MaskFormerForInstanceSegmentation]]

```python
transformers.MaskFormerForInstanceSegmentation(config: MaskFormerConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/maskformer/modeling_maskformer.py#L1838)

#### forward[[transformers.MaskFormerForInstanceSegmentation.forward]]

```python
forward(pixel_values: Tensor, mask_labels: list[torch.Tensor] | None = None, class_labels: list[torch.Tensor] | None = None, pixel_mask: typing.Optional[torch.Tensor] = None, output_auxiliary_logits: bool | None = None, output_hidden_states: bool | None = None, output_attentions: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/maskformer/modeling_maskformer.py#L1920)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [MaskFormerImageProcessor](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerImageProcessor). See `MaskFormerImageProcessor.__call__()` for details (`processor_class` uses [MaskFormerImageProcessor](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerImageProcessor) for processing images).

mask_labels (`list[torch.Tensor]`, *optional*) : List of mask labels of shape `(num_labels, height, width)` to be fed to a model

class_labels (`list[torch.LongTensor]`, *optional*) : list of target class labels of shape `(num_labels, height, width)` to be fed to a model. They identify the labels of `mask_labels`, e.g. the label of `mask_labels[i][j]` if `class_labels[i][j]`.

pixel_mask (`torch.Tensor` of shape `(batch_size, height, width)`, *optional*) : Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:  - 1 for pixels that are real (i.e. **not masked**), - 0 for pixels that are padding (i.e. **masked**).  [What are attention masks?](../glossary#attention-mask)

output_auxiliary_logits (`bool`, *optional*) : Whether or not to output auxiliary logits.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [MaskFormerForInstanceSegmentationOutput](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.models.maskformer.modeling_maskformer.MaskFormerForInstanceSegmentationOutput) or `tuple(torch.FloatTensor)`

A [MaskFormerForInstanceSegmentationOutput](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.models.maskformer.modeling_maskformer.MaskFormerForInstanceSegmentationOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MaskFormerConfig](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerConfig)) and inputs.

The [MaskFormerForInstanceSegmentation](/docs/transformers/v5.15.1/en/model_doc/maskformer#transformers.MaskFormerForInstanceSegmentation) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.Tensor`, *optional*) -- The computed loss, returned when labels are present.
- **class_queries_logits** (`torch.FloatTensor`, *optional*) -- A tensor of shape `(batch_size, num_queries, num_labels + 1)` representing the proposed classes for each
  query. Note the `+ 1` is needed because we incorporate the null class.
- **masks_queries_logits** (`torch.FloatTensor`, *optional*) -- A tensor of shape `(batch_size, num_queries, height, width)` representing the proposed masks for each
  query.
- **auxiliary_logits** (`Dict[str, torch.FloatTensor]`, *optional*, returned when `output_auxiliary_logits=True`) -- Dictionary containing auxiliary predictions for each decoder layer when auxiliary losses are enabled.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Last hidden states (final feature map) of the last stage of the encoder model (backbone).
- **pixel_decoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Last hidden states (final feature map) of the last stage of the pixel decoder model (FPN).
- **transformer_decoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Last hidden states (final feature map) of the last stage of the transformer decoder model.
- **encoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the encoder
  model at the output of each stage.
- **pixel_decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the pixel
  decoder model at the output of each stage.
- **transformer_decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, sequence_length, hidden_size)`. Hidden-states of the transformer decoder at the output
  of each stage.
  hidden_states `tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`):
  Tuple of `torch.FloatTensor` containing `encoder_hidden_states`, `pixel_decoder_hidden_states` and
  `decoder_hidden_states`.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

Semantic segmentation example:

```python
>>> from transformers import AutoImageProcessor, MaskFormerForInstanceSegmentation
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> # load MaskFormer fine-tuned on ADE20k semantic segmentation
>>> image_processor = AutoImageProcessor.from_pretrained("facebook/maskformer-swin-base-ade")
>>> model = MaskFormerForInstanceSegmentation.from_pretrained("facebook/maskformer-swin-base-ade")

>>> url = (
...     "https://huggingface.co/datasets/hf-internal-testing/fixtures_ade20k/resolve/main/ADE_val_00000001.jpg"
... )
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))
>>> inputs = image_processor(images=image, return_tensors="pt")

>>> outputs = model(**inputs)
>>> # model predicts class_queries_logits of shape `(batch_size, num_queries)`
>>> # and masks_queries_logits of shape `(batch_size, num_queries, height, width)`
>>> class_queries_logits = outputs.class_queries_logits
>>> masks_queries_logits = outputs.masks_queries_logits

>>> # you can pass them to image_processor for postprocessing
>>> predicted_semantic_map = image_processor.post_process_semantic_segmentation(
...     outputs, target_sizes=[(image.height, image.width)]
... )[0]

>>> # we refer to the demo notebooks for visualization (see "Resources" section in the MaskFormer docs)
>>> list(predicted_semantic_map.shape)
[512, 683]
```

Panoptic segmentation example:

```python
>>> from transformers import AutoImageProcessor, MaskFormerForInstanceSegmentation
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> # load MaskFormer fine-tuned on COCO panoptic segmentation
>>> image_processor = AutoImageProcessor.from_pretrained("facebook/maskformer-swin-base-coco")
>>> model = MaskFormerForInstanceSegmentation.from_pretrained("facebook/maskformer-swin-base-coco")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))
>>> inputs = image_processor(images=image, return_tensors="pt")

>>> outputs = model(**inputs)
>>> # model predicts class_queries_logits of shape `(batch_size, num_queries)`
>>> # and masks_queries_logits of shape `(batch_size, num_queries, height, width)`
>>> class_queries_logits = outputs.class_queries_logits
>>> masks_queries_logits = outputs.masks_queries_logits

>>> # you can pass them to image_processor for postprocessing
>>> result = image_processor.post_process_panoptic_segmentation(outputs, target_sizes=[(image.height, image.width)])[0]

>>> # we refer to the demo notebooks for visualization (see "Resources" section in the MaskFormer docs)
>>> predicted_panoptic_map = result["segmentation"]
>>> list(predicted_panoptic_map.shape)
[480, 640]
```
