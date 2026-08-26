# OneFormer

## Overview

The OneFormer model was proposed in [OneFormer: One Transformer to Rule Universal Image Segmentation](https://huggingface.co/papers/2211.06220) by Jitesh Jain, Jiachen Li, MangTik Chiu, Ali Hassani, Nikita Orlov, Humphrey Shi. OneFormer is a universal image segmentation framework that can be trained on a single panoptic dataset to perform semantic, instance, and panoptic segmentation tasks. OneFormer uses a task token to condition the model on the task in focus, making the architecture task-guided for training, and task-dynamic for inference.

The abstract from the paper is the following:

*Universal Image Segmentation is not a new concept. Past attempts to unify image segmentation in the last decades include scene parsing, panoptic segmentation, and, more recently, new panoptic architectures. However, such panoptic architectures do not truly unify image segmentation because they need to be trained individually on the semantic, instance, or panoptic segmentation to achieve the best performance. Ideally, a truly universal framework should be trained only once and achieve SOTA performance across all three image segmentation tasks. To that end, we propose OneFormer, a universal image segmentation framework that unifies segmentation with a multi-task train-once design. We first propose a task-conditioned joint training strategy that enables training on ground truths of each domain (semantic, instance, and panoptic segmentation) within a single multi-task training process. Secondly, we introduce a task token to condition our model on the task at hand, making our model task-dynamic to support multi-task training and inference. Thirdly, we propose using a query-text contrastive loss during training to establish better inter-task and inter-class distinctions. Notably, our single OneFormer model outperforms specialized Mask2Former models across all three segmentation tasks on ADE20k, CityScapes, and COCO, despite the latter being trained on each of the three tasks individually with three times the resources. With new ConvNeXt and DiNAT backbones, we observe even more performance improvement. We believe OneFormer is a significant step towards making image segmentation more universal and accessible.*

The figure below illustrates the architecture of OneFormer. Taken from the [original paper](https://huggingface.co/papers/2211.06220).

This model was contributed by [Jitesh Jain](https://huggingface.co/praeclarumjj3). The original code can be found [here](https://github.com/SHI-Labs/OneFormer).

## Usage tips

- OneFormer requires two inputs during inference: *image* and *task token*.
- During training, OneFormer only uses panoptic annotations.
- If you want to train the model in a distributed environment across multiple nodes, then one should update the
  `get_num_masks` function inside in the `OneFormerLoss` class of `modeling_oneformer.py`. When training on multiple nodes, this should be
  set to the average number of target masks across all nodes, as can be seen in the original implementation [here](https://github.com/SHI-Labs/OneFormer/blob/33ebb56ed34f970a30ae103e786c0cb64c653d9a/oneformer/modeling/criterion.py#L287).
- One can use [OneFormerProcessor](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerProcessor) to prepare input images and task inputs for the model and optional targets for the model. [OneFormerProcessor](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerProcessor) wraps [OneFormerImageProcessor](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerImageProcessor) and [CLIPTokenizer](/docs/transformers/v5.15.1/en/model_doc/clip#transformers.CLIPTokenizer) into a single instance to both prepare the images and encode the task inputs.
- To get the final segmentation, depending on the task, you can call `post_process_semantic_segmentation()` or [post_process_instance_segmentation()](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerImageProcessor.post_process_instance_segmentation) or [post_process_panoptic_segmentation()](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerImageProcessor.post_process_panoptic_segmentation). All three tasks can be solved using [OneFormerForUniversalSegmentation](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerForUniversalSegmentation) output, panoptic segmentation accepts an optional `label_ids_to_fuse` argument to fuse instances of the target object/s (e.g. sky) together.

## Resources

A list of official Hugging Face and community (indicated by 🌎) resources to help you get started with OneFormer.

- Demo notebooks regarding inference + fine-tuning on custom data can be found [here](https://github.com/NielsRogge/Transformers-Tutorials/tree/master/OneFormer).

If you're interested in submitting a resource to be included here, please feel free to open a Pull Request and we will review it.
The resource should ideally demonstrate something new instead of duplicating an existing resource.

## OneFormer specific outputs[[transformers.models.oneformer.modeling_oneformer.OneFormerModelOutput]]

#### transformers.models.oneformer.modeling_oneformer.OneFormerModelOutput[[transformers.models.oneformer.modeling_oneformer.OneFormerModelOutput]]

```python
transformers.models.oneformer.modeling_oneformer.OneFormerModelOutput(encoder_hidden_states: tuple[torch.FloatTensor] | None = None, pixel_decoder_hidden_states: tuple[torch.FloatTensor] | None = None, transformer_decoder_hidden_states: typing.Optional[torch.FloatTensor] = None, transformer_decoder_object_queries: typing.Optional[torch.FloatTensor] = None, transformer_decoder_contrastive_queries: typing.Optional[torch.FloatTensor] = None, transformer_decoder_mask_predictions: typing.Optional[torch.FloatTensor] = None, transformer_decoder_class_predictions: typing.Optional[torch.FloatTensor] = None, transformer_decoder_auxiliary_predictions: tuple[dict[str, torch.FloatTensor]] | None = None, text_queries: typing.Optional[torch.FloatTensor] = None, task_token: typing.Optional[torch.FloatTensor] = None, attentions: tuple[torch.FloatTensor] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/oneformer/modeling_oneformer.py#L820)

**Parameters:**

encoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the encoder model at the output of each stage.

pixel_decoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the pixel decoder model at the output of each stage.

transformer_decoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states (also called feature maps) of the transformer decoder at the output of each stage.

transformer_decoder_object_queries (`torch.FloatTensor` of shape `(batch_size, num_queries, hidden_dim)`) : Output object queries from the last layer in the transformer decoder.

transformer_decoder_contrastive_queries (`torch.FloatTensor` of shape `(batch_size, num_queries, hidden_dim)`) : Contrastive queries from the transformer decoder.

transformer_decoder_mask_predictions (`torch.FloatTensor` of shape `(batch_size, num_queries, height, width)`) : Mask Predictions from the last layer in the transformer decoder.

transformer_decoder_class_predictions (`torch.FloatTensor` of shape `(batch_size, num_queries, num_classes+1)`) : Class Predictions from the last layer in the transformer decoder.

transformer_decoder_auxiliary_predictions (`Tuple` of Dict of `str, torch.FloatTensor`, *optional*) : Tuple of class and mask predictions from each layer of the transformer decoder.

text_queries (`torch.FloatTensor`, *optional* of shape `(batch_size, num_queries, hidden_dim)`) : Text queries derived from the input text list used for calculating contrastive loss during training.

task_token (`torch.FloatTensor` of shape `(batch_size, hidden_dim)`) : 1D task token to condition the queries.

attentions (`tuple(tuple(torch.FloatTensor))`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `tuple(torch.FloatTensor)` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`. Self and Cross Attentions weights from transformer decoder.

Class for outputs of [OneFormerModel](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerModel). This class returns all the needed hidden states to compute the logits.

#### transformers.models.oneformer.modeling_oneformer.OneFormerForUniversalSegmentationOutput[[transformers.models.oneformer.modeling_oneformer.OneFormerForUniversalSegmentationOutput]]

```python
transformers.models.oneformer.modeling_oneformer.OneFormerForUniversalSegmentationOutput(loss: typing.Optional[torch.FloatTensor] = None, class_queries_logits: typing.Optional[torch.FloatTensor] = None, masks_queries_logits: typing.Optional[torch.FloatTensor] = None, auxiliary_predictions: list = None, encoder_hidden_states: tuple[torch.FloatTensor] | None = None, pixel_decoder_hidden_states: list[torch.FloatTensor] | None = None, transformer_decoder_hidden_states: typing.Optional[torch.FloatTensor] = None, transformer_decoder_object_queries: typing.Optional[torch.FloatTensor] = None, transformer_decoder_contrastive_queries: typing.Optional[torch.FloatTensor] = None, transformer_decoder_mask_predictions: typing.Optional[torch.FloatTensor] = None, transformer_decoder_class_predictions: typing.Optional[torch.FloatTensor] = None, transformer_decoder_auxiliary_predictions: list[dict[str, torch.FloatTensor]] | None = None, text_queries: typing.Optional[torch.FloatTensor] = None, task_token: typing.Optional[torch.FloatTensor] = None, attentions: tuple[tuple[torch.FloatTensor]] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/oneformer/modeling_oneformer.py#L877)

**Parameters:**

loss (`torch.Tensor`, *optional*) : The computed loss, returned when labels are present.

class_queries_logits (`torch.FloatTensor`, *optional*) : A tensor of shape `(batch_size, num_queries, num_labels + 1)` representing the proposed classes for each query. Note the `+ 1` is needed because we incorporate the null class.

masks_queries_logits (`torch.FloatTensor`, *optional*) : A tensor of shape `(batch_size, num_queries, height, width)` representing the proposed masks for each query.

auxiliary_predictions (`List` of Dict of `str, torch.FloatTensor`, *optional*) : List of class and mask predictions from each layer of the transformer decoder.

encoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the encoder model at the output of each stage.

pixel_decoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the pixel decoder model at the output of each stage.

transformer_decoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states (also called feature maps) of the transformer decoder at the output of each stage.

transformer_decoder_object_queries (`torch.FloatTensor` of shape `(batch_size, num_queries, hidden_dim)`) : Output object queries from the last layer in the transformer decoder.

transformer_decoder_contrastive_queries (`torch.FloatTensor` of shape `(batch_size, num_queries, hidden_dim)`) : Contrastive queries from the transformer decoder.

transformer_decoder_mask_predictions (`torch.FloatTensor` of shape `(batch_size, num_queries, height, width)`) : Mask Predictions from the last layer in the transformer decoder.

transformer_decoder_class_predictions (`torch.FloatTensor` of shape `(batch_size, num_queries, num_classes+1)`) : Class Predictions from the last layer in the transformer decoder.

transformer_decoder_auxiliary_predictions (`List` of Dict of `str, torch.FloatTensor`, *optional*) : List of class and mask predictions from each layer of the transformer decoder.

text_queries (`torch.FloatTensor`, *optional* of shape `(batch_size, num_queries, hidden_dim)`) : Text queries derived from the input text list used for calculating contrastive loss during training.

task_token (`torch.FloatTensor` of shape `(batch_size, hidden_dim)`) : 1D task token to condition the queries.

attentions (`tuple(tuple(torch.FloatTensor))`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `tuple(torch.FloatTensor)` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`. Self and Cross Attentions weights from transformer decoder.

Class for outputs of `OneFormerForUniversalSegmentationOutput`.

This output can be directly passed to [post_process_semantic_segmentation()](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerImageProcessor.post_process_semantic_segmentation) or
[post_process_instance_segmentation()](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerImageProcessor.post_process_instance_segmentation) or
[post_process_panoptic_segmentation()](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerImageProcessor.post_process_panoptic_segmentation) depending on the task. Please, see
[`~OneFormerImageProcessor] for details regarding usage.

## OneFormerConfig[[transformers.OneFormerConfig]]

#### transformers.OneFormerConfig[[transformers.OneFormerConfig]]

```python
transformers.OneFormerConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, backbone_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, ignore_value: int = 255, num_queries: int = 150, no_object_weight: float = 0.1, class_weight: float = 2.0, mask_weight: float = 5.0, dice_weight: float = 5.0, contrastive_weight: float = 0.5, contrastive_temperature: float = 0.07, train_num_points: int = 12544, oversample_ratio: float = 3.0, importance_sample_ratio: float = 0.75, init_std: float = 0.02, init_xavier_std: float = 1.0, layer_norm_eps: float = 1e-05, is_training: bool = False, use_auxiliary_loss: bool = True, output_auxiliary_logits: bool = True, strides: list[int] | tuple[int, ...] = (4, 8, 16, 32), task_seq_len: int = 77, text_encoder_width: int = 256, text_encoder_context_length: int = 77, text_encoder_num_layers: int = 6, text_encoder_vocab_size: int = 49408, text_encoder_proj_layers: int = 2, text_encoder_n_ctx: int = 16, conv_dim: int = 256, mask_dim: int = 256, hidden_dim: int = 256, encoder_feedforward_dim: int = 1024, norm: str = 'GN', encoder_layers: int = 6, decoder_layers: int = 10, use_task_norm: bool = True, num_attention_heads: int = 8, dropout: float | int = 0.1, dim_feedforward: int = 2048, pre_norm: bool = False, enforce_input_proj: bool = False, query_dec_layers: int = 2, common_stride: int = 4)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/oneformer/configuration_oneformer.py#L26)

**Parameters:**

backbone_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The configuration of the backbone model.

ignore_value (`int`, *optional*, defaults to 255) : Values to be ignored in GT label while calculating loss.

num_queries (`int`, *optional*, defaults to 150) : Number of object queries.

no_object_weight (`float`, *optional*, defaults to `0.1`) : Relative classification weight of the no-object class in the object detection loss.

class_weight (`float`, *optional*, defaults to `2.0`) : Relative weight of the classification error in the Hungarian matching cost.

mask_weight (`float`, *optional*, defaults to `5.0`) : Relative weight of the focal loss in the panoptic segmentation loss.

dice_weight (`float`, *optional*, defaults to `5.0`) : Relative weight of the dice loss in the panoptic segmentation loss.

contrastive_weight (`float`, *optional*, defaults to 0.5) : Weight for contrastive loss.

contrastive_temperature (`float`, *optional*, defaults to 0.07) : Initial value for scaling the contrastive logits.

train_num_points (`int`, *optional*, defaults to 12544) : Number of points to sample while calculating losses on mask predictions.

oversample_ratio (`float`, *optional*, defaults to 3.0) : Ratio to decide how many points to oversample.

importance_sample_ratio (`float`, *optional*, defaults to 0.75) : Ratio of points that are sampled via importance sampling..

init_std (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

init_xavier_std (`float`, *optional*, defaults to `1.0`) : The scaling factor used for the Xavier initialization of the cross-attention weights.

layer_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

is_training (`bool`, *optional*, defaults to `False`) : Whether to run in training or inference mode.

use_auxiliary_loss (`bool`, *optional*, defaults to `True`) : Whether to calculate loss using intermediate predictions from transformer decoder.

output_auxiliary_logits (`bool`, *optional*, defaults to `True`) : Whether to return intermediate predictions from transformer decoder.

strides (`list`, *optional*, defaults to `[4, 8, 16, 32]`) : List containing the strides for feature maps in the encoder.

task_seq_len (`int`, *optional*, defaults to 77) : Sequence length for tokenizing text list input.

text_encoder_width (`int`, *optional*, defaults to 256) : Hidden size for text encoder.

text_encoder_context_length (`int`, *optional*, defaults to 77) : Input sequence length for text encoder.

text_encoder_num_layers (`int`, *optional*, defaults to 6) : Number of layers for transformer in text encoder.

text_encoder_vocab_size (`int`, *optional*, defaults to 49408) : Vocabulary size for tokenizer.

text_encoder_proj_layers (`int`, *optional*, defaults to 2) : Number of layers in MLP for project text queries.

text_encoder_n_ctx (`int`, *optional*, defaults to 16) : Number of learnable text context queries.

conv_dim (`int`, *optional*, defaults to 256) : Feature map dimension to map outputs from the backbone.

mask_dim (`int`, *optional*, defaults to 256) : Dimension for feature maps in pixel decoder.

hidden_dim (`int`, *optional*, defaults to 256) : Dimension for hidden states in transformer decoder.

encoder_feedforward_dim (`int`, *optional*, defaults to 1024) : Dimension for FFN layer in pixel decoder.

norm (`str`, *optional*, defaults to `"GN"`) : Type of normalization.

encoder_layers (`int`, *optional*, defaults to `6`) : Number of hidden layers in the Transformer encoder. Will use the same value as `num_layers` if not set.

decoder_layers (`int`, *optional*, defaults to `10`) : Number of hidden layers in the Transformer decoder. Will use the same value as `num_layers` if not set.

use_task_norm (`bool`, *optional*, defaults to `True`) : Whether to normalize the task token.

num_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer decoder.

dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The ratio for all dropout layers.

dim_feedforward (`int`, *optional*, defaults to 2048) : Dimension for FFN layer in transformer decoder.

pre_norm (`bool`, *optional*, defaults to `False`) : Whether to normalize hidden states before attention layers in transformer decoder.

enforce_input_proj (`bool`, *optional*, defaults to `False`) : Whether to project hidden states in transformer decoder.

query_dec_layers (`int`, *optional*, defaults to 2) : Number of layers in query transformer.

common_stride (`int`, *optional*, defaults to 4) : Common stride used for features in pixel decoder.

This is the configuration class to store the configuration of a OneFormerModel. It is used to instantiate a Oneformer
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [shi-labs/oneformer_ade20k_swin_tiny](https://huggingface.co/shi-labs/oneformer_ade20k_swin_tiny)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:
```python
>>> from transformers import OneFormerConfig, OneFormerModel

>>> # Initializing a OneFormer shi-labs/oneformer_ade20k_swin_tiny configuration
>>> configuration = OneFormerConfig()
>>> # Initializing a model (with random weights) from the shi-labs/oneformer_ade20k_swin_tiny style configuration
>>> model = OneFormerModel(configuration)
>>> # Accessing the model configuration
>>> configuration = model.config
```

## OneFormerImageProcessor[[transformers.OneFormerImageProcessor]]

#### transformers.OneFormerImageProcessor[[transformers.OneFormerImageProcessor]]

```python
transformers.OneFormerImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/oneformer/image_processing_oneformer.py#L275)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to convert the image to RGB.

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

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

repo_path (`str`, *kwargs*, *optional*, defaults to `shi-labs/oneformer_demo`) : Path to a local directory or HuggingFace Hub repository containing model metadata.

class_info_file (`str`, *kwargs*, *optional*) : Path to the JSON file within the repository that contains class metadata.

num_text (`int`, *kwargs*, *optional*) : Number of text queries for the text encoder, used as task-guiding prompts.

num_labels (`int`, *kwargs*, *optional*) : Number of semantic classes for segmentation, determining the output layer's size.

ignore_index (`int`, *kwargs*, *optional*) : Label to ignore in segmentation maps, often used for padding.

do_reduce_labels (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to decrement all label values by 1, mapping the background class to `ignore_index`.

Constructs a OneFormerImageProcessor image processor.

#### preprocess[[transformers.OneFormerImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], task_inputs: list[str] | None = None, segmentation_maps: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, instance_id_to_semantic_id: list[dict[int, int]] | dict[int, int] | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/oneformer/image_processing_oneformer.py#L300)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

task_inputs (`list[str]`, *optional*) : List of tasks (`"panoptic"`, `"instance"`, `"semantic"`) for each image in the batch.

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

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

repo_path (`str`, *kwargs*, *optional*, defaults to `shi-labs/oneformer_demo`) : Path to a local directory or HuggingFace Hub repository containing model metadata.

class_info_file (`str`, *kwargs*, *optional*) : Path to the JSON file within the repository that contains class metadata.

num_text (`int`, *kwargs*, *optional*) : Number of text queries for the text encoder, used as task-guiding prompts.

num_labels (`int`, *kwargs*, *optional*) : Number of semantic classes for segmentation, determining the output layer's size.

ignore_index (`int`, *kwargs*, *optional*) : Label to ignore in segmentation maps, often used for padding.

do_reduce_labels (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to decrement all label values by 1, mapping the background class to `ignore_index`.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

#### post_process_semantic_segmentation[[transformers.OneFormerImageProcessor.post_process_semantic_segmentation]]

```python
post_process_semantic_segmentation(outputs, target_sizes: list[tuple[int, int]] | None = None, return_segmentation_scores: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/oneformer/image_processing_oneformer.py#L690)

**Parameters:**

outputs ([OneFormerForUniversalSegmentation](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerForUniversalSegmentation)) : Raw outputs of the model.

target_sizes (`list[tuple[int, int]]`, *optional*) : List of length (batch_size), where each list item (`Tuple[int, int]]`) corresponds to the requested final size (height, width) of each prediction. If left to None, predictions will not be resized.

return_segmentation_scores (`bool`, *optional*, defaults to `False`) : Whether to return segmentation scores alongside the segmentation map. When `True`, each element of the returned list is a `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape `(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`).

**Returns:** `list[torch.Tensor]` or `list[SemanticSegmentationPostProcessorOutput]`

When
`return_segmentation_scores=False` (default), a list of length `batch_size` where each item is a
segmentation map of shape `(height, width)` with class IDs. When `return_segmentation_scores=True`,
a list of `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape
`(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`). In both cases,
`(height, width)` corresponds to the target size (if `target_sizes` is specified).

Converts the output of [OneFormerForUniversalSegmentation](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerForUniversalSegmentation) into semantic segmentation maps. Only supports
PyTorch.

#### post_process_instance_segmentation[[transformers.OneFormerImageProcessor.post_process_instance_segmentation]]

```python
post_process_instance_segmentation(outputs, task_type: str = 'instance', is_demo: bool = True, threshold: float = 0.5, mask_threshold: float = 0.5, overlap_mask_area_threshold: float = 0.8, target_sizes: list[tuple[int, int]] | None = None, return_coco_annotation: bool | None = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/oneformer/image_processing_oneformer.py#L766)

**Parameters:**

outputs (`OneFormerForUniversalSegmentationOutput`) : The outputs from `OneFormerForUniversalSegmentationOutput`.

task_type (`str`, *optional*, defaults to "instance") : The post processing depends on the task token input. If the `task_type` is "panoptic", we need to ignore the stuff predictions.

is_demo (`bool`, *optional*, defaults to `True`) : Whether the model is in demo mode. If true, use threshold to predict final masks.

threshold (`float`, *optional*, defaults to 0.5) : The probability score threshold to keep predicted instance masks.

mask_threshold (`float`, *optional*, defaults to 0.5) : Threshold to use when turning the predicted masks into binary values.

overlap_mask_area_threshold (`float`, *optional*, defaults to 0.8) : The overlap mask area threshold to merge or discard small disconnected parts within each binary instance mask.

target_sizes (`List[Tuple]`, *optional*) : List of length (batch_size), where each list item (`Tuple[int, int]]`) corresponds to the requested final size (height, width) of each prediction in batch. If left to None, predictions will not be resized.

return_coco_annotation (`bool`, *optional*, defaults to `False`) : Whether to return predictions in COCO format.

**Returns:** `List[Dict]`

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

Converts the output of `OneFormerForUniversalSegmentationOutput` into image instance segmentation
predictions. Only supports PyTorch.

#### post_process_panoptic_segmentation[[transformers.OneFormerImageProcessor.post_process_panoptic_segmentation]]

```python
post_process_panoptic_segmentation(outputs, threshold: float = 0.5, mask_threshold: float = 0.5, overlap_mask_area_threshold: float = 0.8, label_ids_to_fuse: set[int] | None = None, target_sizes: list[tuple[int, int]] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/oneformer/image_processing_oneformer.py#L886)

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

## OneFormerImageProcessorPil[[transformers.OneFormerImageProcessorPil]]

#### transformers.OneFormerImageProcessorPil[[transformers.OneFormerImageProcessorPil]]

```python
transformers.OneFormerImageProcessorPil(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/oneformer/image_processing_pil_oneformer.py#L293)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to convert the image to RGB.

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

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

repo_path (`str`, *kwargs*, *optional*, defaults to `shi-labs/oneformer_demo`) : Path to a local directory or HuggingFace Hub repository containing model metadata.

class_info_file (`str`, *kwargs*, *optional*) : Path to the JSON file within the repository that contains class metadata.

num_text (`int`, *kwargs*, *optional*) : Number of text queries for the text encoder, used as task-guiding prompts.

num_labels (`int`, *kwargs*, *optional*) : Number of semantic classes for segmentation, determining the output layer's size.

ignore_index (`int`, *kwargs*, *optional*) : Label to ignore in segmentation maps, often used for padding.

do_reduce_labels (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to decrement all label values by 1, mapping the background class to `ignore_index`.

Constructs a OneFormerImageProcessor image processor.

#### preprocess[[transformers.OneFormerImageProcessorPil.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], task_inputs: list[str] | None = None, segmentation_maps: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, instance_id_to_semantic_id: list[dict[int, int]] | dict[int, int] | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/oneformer/image_processing_pil_oneformer.py#L318)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

task_inputs (`list[str]`, *optional*) : List of tasks (`"panoptic"`, `"instance"`, `"semantic"`) for each image in the batch.

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

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

repo_path (`str`, *kwargs*, *optional*, defaults to `shi-labs/oneformer_demo`) : Path to a local directory or HuggingFace Hub repository containing model metadata.

class_info_file (`str`, *kwargs*, *optional*) : Path to the JSON file within the repository that contains class metadata.

num_text (`int`, *kwargs*, *optional*) : Number of text queries for the text encoder, used as task-guiding prompts.

num_labels (`int`, *kwargs*, *optional*) : Number of semantic classes for segmentation, determining the output layer's size.

ignore_index (`int`, *kwargs*, *optional*) : Label to ignore in segmentation maps, often used for padding.

do_reduce_labels (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to decrement all label values by 1, mapping the background class to `ignore_index`.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

#### post_process_semantic_segmentation[[transformers.OneFormerImageProcessorPil.post_process_semantic_segmentation]]

```python
post_process_semantic_segmentation(outputs, target_sizes: list[tuple[int, int]] | None = None, return_segmentation_scores: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/oneformer/image_processing_pil_oneformer.py#L746)

**Parameters:**

outputs ([OneFormerForUniversalSegmentation](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerForUniversalSegmentation)) : Raw outputs of the model.

target_sizes (`list[tuple[int, int]]`, *optional*) : List of length (batch_size), where each list item (`Tuple[int, int]]`) corresponds to the requested final size (height, width) of each prediction. If left to None, predictions will not be resized.

return_segmentation_scores (`bool`, *optional*, defaults to `False`) : Whether to return segmentation scores alongside the segmentation map. When `True`, each element of the returned list is a `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape `(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`).

**Returns:** `list[torch.Tensor]` or `list[SemanticSegmentationPostProcessorOutput]`

When
`return_segmentation_scores=False` (default), a list of length `batch_size` where each item is a
segmentation map of shape `(height, width)` with class IDs. When `return_segmentation_scores=True`,
a list of `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape
`(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`). In both cases,
`(height, width)` corresponds to the target size (if `target_sizes` is specified).

Converts the output of [OneFormerForUniversalSegmentation](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerForUniversalSegmentation) into semantic segmentation maps. Only supports
PyTorch.

#### post_process_instance_segmentation[[transformers.OneFormerImageProcessorPil.post_process_instance_segmentation]]

```python
post_process_instance_segmentation(outputs, task_type: str = 'instance', is_demo: bool = True, threshold: float = 0.5, mask_threshold: float = 0.5, overlap_mask_area_threshold: float = 0.8, target_sizes: list[tuple[int, int]] | None = None, return_coco_annotation: bool | None = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/oneformer/image_processing_pil_oneformer.py#L820)

**Parameters:**

outputs (`OneFormerForUniversalSegmentationOutput`) : The outputs from `OneFormerForUniversalSegmentationOutput`.

task_type (`str`, *optional*, defaults to "instance") : The post processing depends on the task token input. If the `task_type` is "panoptic", we need to ignore the stuff predictions.

is_demo (`bool`, *optional*, defaults to `True`) : Whether the model is in demo mode. If true, use threshold to predict final masks.

threshold (`float`, *optional*, defaults to 0.5) : The probability score threshold to keep predicted instance masks.

mask_threshold (`float`, *optional*, defaults to 0.5) : Threshold to use when turning the predicted masks into binary values.

overlap_mask_area_threshold (`float`, *optional*, defaults to 0.8) : The overlap mask area threshold to merge or discard small disconnected parts within each binary instance mask.

target_sizes (`List[Tuple]`, *optional*) : List of length (batch_size), where each list item (`Tuple[int, int]]`) corresponds to the requested final size (height, width) of each prediction in batch. If left to None, predictions will not be resized.

return_coco_annotation (`bool`, *optional*, defaults to `False`) : Whether to return predictions in COCO format.

**Returns:** `List[Dict]`

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

Converts the output of `OneFormerForUniversalSegmentationOutput` into image instance segmentation
predictions. Only supports PyTorch.

#### post_process_panoptic_segmentation[[transformers.OneFormerImageProcessorPil.post_process_panoptic_segmentation]]

```python
post_process_panoptic_segmentation(outputs, threshold: float = 0.5, mask_threshold: float = 0.5, overlap_mask_area_threshold: float = 0.8, label_ids_to_fuse: set[int] | None = None, target_sizes: list[tuple[int, int]] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/oneformer/image_processing_pil_oneformer.py#L940)

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

## OneFormerProcessor[[transformers.OneFormerProcessor]]

#### transformers.OneFormerProcessor[[transformers.OneFormerProcessor]]

```python
transformers.OneFormerProcessor(image_processor = None, tokenizer = None, max_seq_length: int = 77, task_seq_length: int = 77, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/oneformer/processing_oneformer.py#L27)

**Parameters:**

image_processor (`OneFormerImageProcessor`) : The image processor is a required input.

tokenizer (`CLIPTokenizer`) : The tokenizer is a required input.

max_seq_length (`int`, *optional*, defaults to `77`) : Maximum sequence length for encoding class names and text inputs. This parameter controls the maximum number of tokens used when tokenizing class names and other text inputs for the model.

task_seq_length (`int`, *optional*, defaults to `77`) : Maximum sequence length specifically for encoding task descriptions. Task descriptions (e.g., "the task is semantic") are tokenized with this length limit, which may differ from the general text sequence length.

Constructs a OneFormerProcessor which wraps a image processor and a tokenizer into a single processor.

[OneFormerProcessor](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerProcessor) offers all the functionalities of [OneFormerImageProcessor](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerImageProcessor) and [CLIPTokenizer](/docs/transformers/v5.15.1/en/model_doc/clip#transformers.CLIPTokenizer). See the
[~OneFormerImageProcessor](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerImageProcessor) and [~CLIPTokenizer](/docs/transformers/v5.15.1/en/model_doc/clip#transformers.CLIPTokenizer) for more information.

#### __call__[[transformers.OneFormerProcessor.__call__]]

```python
__call__(images = None, task_inputs = None, segmentation_maps = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/oneformer/processing_oneformer.py#L61)

**Parameters:**

images (``) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

task_inputs (`str` or `list[str]`, *required*) : The task type(s) for segmentation. Must be one or more of `"semantic"`, `"instance"`, or `"panoptic"`. Can be a single string for a single image, or a list of strings (one per image) for batch processing. The task type determines which type of segmentation the model will perform on the input images.

segmentation_maps (`ImageInput`, *optional*) : The corresponding semantic segmentation maps with the pixel-wise annotations.  (`bool`, *optional*, defaults to `True`): Whether or not to pad images up to the largest image in a batch and create a pixel mask.  If left to the default, will return a pixel mask that is:  - 1 for pixels that are real (i.e. **not masked**), - 0 for pixels that are padding (i.e. **masked**).

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

**Returns:** [BatchFeature](/docs/transformers/v5.15.1/en/main_classes/image_processor#transformers.BatchFeature)

A [BatchFeature](/docs/transformers/v5.15.1/en/main_classes/image_processor#transformers.BatchFeature) with the following fields:
- **task_inputs** -- List of token ids to be fed to a model. Returned when `text` is not `None`.
- **pixel_values** -- Pixel values to be fed to a model. Returned when `images` is not `None`.

## OneFormerModel[[transformers.OneFormerModel]]

#### transformers.OneFormerModel[[transformers.OneFormerModel]]

```python
transformers.OneFormerModel(config: OneFormerConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/oneformer/modeling_oneformer.py#L2845)

**Parameters:**

config ([OneFormerConfig](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Oneformer Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.OneFormerModel.forward]]

```python
forward(pixel_values: Tensor, task_inputs: Tensor, text_inputs: typing.Optional[torch.Tensor] = None, pixel_mask: typing.Optional[torch.Tensor] = None, output_hidden_states: bool | None = None, output_attentions: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/oneformer/modeling_oneformer.py#L2862)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [OneFormerImageProcessor](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerImageProcessor). See `OneFormerImageProcessor.__call__()` for details ([OneFormerProcessor](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerProcessor) uses [OneFormerImageProcessor](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerImageProcessor) for processing images).

task_inputs (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) : Task inputs. Task inputs can be obtained using [AutoImageProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoImageProcessor). See [OneFormerProcessor.__call__()](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerProcessor.__call__) for details.

text_inputs (`list[torch.Tensor]`, *optional*) : Tensor of shape `(num_queries, sequence_length)` to be fed to a model

pixel_mask (`torch.Tensor` of shape `(batch_size, height, width)`, *optional*) : Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:  - 1 for pixels that are real (i.e. **not masked**), - 0 for pixels that are padding (i.e. **masked**).  [What are attention masks?](../glossary#attention-mask)

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [OneFormerModelOutput](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.models.oneformer.modeling_oneformer.OneFormerModelOutput) or `tuple(torch.FloatTensor)`

A [OneFormerModelOutput](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.models.oneformer.modeling_oneformer.OneFormerModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([OneFormerConfig](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerConfig)) and inputs.

The [OneFormerModel](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **encoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the encoder
  model at the output of each stage.
- **pixel_decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the pixel
  decoder model at the output of each stage.
- **transformer_decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, sequence_length, hidden_size)`. Hidden-states (also called feature maps) of the
  transformer decoder at the output of each stage.
- **transformer_decoder_object_queries** (`torch.FloatTensor` of shape `(batch_size, num_queries, hidden_dim)`) -- Output object queries from the last layer in the transformer decoder.
- **transformer_decoder_contrastive_queries** (`torch.FloatTensor` of shape `(batch_size, num_queries, hidden_dim)`) -- Contrastive queries from the transformer decoder.
- **transformer_decoder_mask_predictions** (`torch.FloatTensor` of shape `(batch_size, num_queries, height, width)`) -- Mask Predictions from the last layer in the transformer decoder.
- **transformer_decoder_class_predictions** (`torch.FloatTensor` of shape `(batch_size, num_queries, num_classes+1)`) -- Class Predictions from the last layer in the transformer decoder.
- **transformer_decoder_auxiliary_predictions** (`Tuple` of Dict of `str, torch.FloatTensor`, *optional*) -- Tuple of class and mask predictions from each layer of the transformer decoder.
- **text_queries** (`torch.FloatTensor`, *optional* of shape `(batch_size, num_queries, hidden_dim)`) -- Text queries derived from the input text list used for calculating contrastive loss during training.
- **task_token** (`torch.FloatTensor` of shape `(batch_size, hidden_dim)`) -- 1D task token to condition the queries.
- **attentions** (`tuple(tuple(torch.FloatTensor))`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `tuple(torch.FloatTensor)` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`. Self and Cross Attentions weights from transformer decoder.

Example:

```python
>>> import torch
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import OneFormerProcessor, OneFormerModel

>>> # download test image
>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> # load processor for preprocessing the inputs
>>> processor = OneFormerProcessor.from_pretrained("shi-labs/oneformer_ade20k_swin_tiny")
>>> model = OneFormerModel.from_pretrained("shi-labs/oneformer_ade20k_swin_tiny")
>>> inputs = processor(image, ["semantic"], return_tensors="pt")

>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> mask_predictions = outputs.transformer_decoder_mask_predictions
>>> class_predictions = outputs.transformer_decoder_class_predictions

>>> f"👉 Mask Predictions Shape: {list(mask_predictions.shape)}, Class Predictions Shape: {list(class_predictions.shape)}"
'👉 Mask Predictions Shape: [1, 150, 128, 171], Class Predictions Shape: [1, 150, 151]'
```

## OneFormerForUniversalSegmentation[[transformers.OneFormerForUniversalSegmentation]]

#### transformers.OneFormerForUniversalSegmentation[[transformers.OneFormerForUniversalSegmentation]]

```python
transformers.OneFormerForUniversalSegmentation(config: OneFormerConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/oneformer/modeling_oneformer.py#L2981)

**Parameters:**

config ([OneFormerConfig](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

OneFormer Model for instance, semantic and panoptic image segmentation.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.OneFormerForUniversalSegmentation.forward]]

```python
forward(pixel_values: Tensor, task_inputs: Tensor, text_inputs: typing.Optional[torch.Tensor] = None, mask_labels: list[torch.Tensor] | None = None, class_labels: list[torch.Tensor] | None = None, pixel_mask: typing.Optional[torch.Tensor] = None, output_auxiliary_logits: bool | None = None, output_hidden_states: bool | None = None, output_attentions: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/oneformer/modeling_oneformer.py#L3048)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [OneFormerImageProcessor](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerImageProcessor). See `OneFormerImageProcessor.__call__()` for details ([OneFormerProcessor](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerProcessor) uses [OneFormerImageProcessor](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerImageProcessor) for processing images).

task_inputs (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) : Task inputs. Task inputs can be obtained using [AutoImageProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoImageProcessor). See [OneFormerProcessor.__call__()](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerProcessor.__call__) for details.

text_inputs (`list[torch.Tensor]`, *optional*) : Tensor of shape `(num_queries, sequence_length)` to be fed to a model

mask_labels (`list[torch.Tensor]`, *optional*) : List of mask labels of shape `(num_labels, height, width)` to be fed to a model

class_labels (`list[torch.LongTensor]`, *optional*) : list of target class labels of shape `(num_labels, height, width)` to be fed to a model. They identify the labels of `mask_labels`, e.g. the label of `mask_labels[i][j]` if `class_labels[i][j]`.

pixel_mask (`torch.Tensor` of shape `(batch_size, height, width)`, *optional*) : Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:  - 1 for pixels that are real (i.e. **not masked**), - 0 for pixels that are padding (i.e. **masked**).  [What are attention masks?](../glossary#attention-mask)

output_auxiliary_logits (`bool`, *optional*) : Whether or not to output auxiliary logits.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [OneFormerForUniversalSegmentationOutput](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.models.oneformer.modeling_oneformer.OneFormerForUniversalSegmentationOutput) or `tuple(torch.FloatTensor)`

A [OneFormerForUniversalSegmentationOutput](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.models.oneformer.modeling_oneformer.OneFormerForUniversalSegmentationOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([OneFormerConfig](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerConfig)) and inputs.

The [OneFormerForUniversalSegmentation](/docs/transformers/v5.15.1/en/model_doc/oneformer#transformers.OneFormerForUniversalSegmentation) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.Tensor`, *optional*) -- The computed loss, returned when labels are present.
- **class_queries_logits** (`torch.FloatTensor`, *optional*) -- A tensor of shape `(batch_size, num_queries, num_labels + 1)` representing the proposed classes for each
  query. Note the `+ 1` is needed because we incorporate the null class.
- **masks_queries_logits** (`torch.FloatTensor`, *optional*) -- A tensor of shape `(batch_size, num_queries, height, width)` representing the proposed masks for each
  query.
- **auxiliary_predictions** (`List` of Dict of `str, torch.FloatTensor`, *optional*) -- List of class and mask predictions from each layer of the transformer decoder.
- **encoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the encoder
  model at the output of each stage.
- **pixel_decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the pixel
  decoder model at the output of each stage.
- **transformer_decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, sequence_length, hidden_size)`. Hidden-states (also called feature maps) of the
  transformer decoder at the output of each stage.
- **transformer_decoder_object_queries** (`torch.FloatTensor` of shape `(batch_size, num_queries, hidden_dim)`) -- Output object queries from the last layer in the transformer decoder.
- **transformer_decoder_contrastive_queries** (`torch.FloatTensor` of shape `(batch_size, num_queries, hidden_dim)`) -- Contrastive queries from the transformer decoder.
- **transformer_decoder_mask_predictions** (`torch.FloatTensor` of shape `(batch_size, num_queries, height, width)`) -- Mask Predictions from the last layer in the transformer decoder.
- **transformer_decoder_class_predictions** (`torch.FloatTensor` of shape `(batch_size, num_queries, num_classes+1)`) -- Class Predictions from the last layer in the transformer decoder.
- **transformer_decoder_auxiliary_predictions** (`List` of Dict of `str, torch.FloatTensor`, *optional*) -- List of class and mask predictions from each layer of the transformer decoder.
- **text_queries** (`torch.FloatTensor`, *optional* of shape `(batch_size, num_queries, hidden_dim)`) -- Text queries derived from the input text list used for calculating contrastive loss during training.
- **task_token** (`torch.FloatTensor` of shape `(batch_size, hidden_dim)`) -- 1D task token to condition the queries.
- **attentions** (`tuple(tuple(torch.FloatTensor))`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `tuple(torch.FloatTensor)` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`. Self and Cross Attentions weights from transformer decoder.

Example:

Universal segmentation example:

```python
>>> from transformers import OneFormerProcessor, OneFormerForUniversalSegmentation
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> import torch

>>> # load OneFormer fine-tuned on ADE20k for universal segmentation
>>> processor = OneFormerProcessor.from_pretrained("shi-labs/oneformer_ade20k_swin_tiny")
>>> model = OneFormerForUniversalSegmentation.from_pretrained("shi-labs/oneformer_ade20k_swin_tiny")

>>> url = (
...     "https://huggingface.co/datasets/hf-internal-testing/fixtures_ade20k/resolve/main/ADE_val_00000001.jpg"
... )
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> # Semantic Segmentation
>>> inputs = processor(image, ["semantic"], return_tensors="pt")

>>> with torch.no_grad():
...     outputs = model(**inputs)
>>> # model predicts class_queries_logits of shape `(batch_size, num_queries)`
>>> # and masks_queries_logits of shape `(batch_size, num_queries, height, width)`
>>> class_queries_logits = outputs.class_queries_logits
>>> masks_queries_logits = outputs.masks_queries_logits

>>> # you can pass them to processor for semantic postprocessing
>>> predicted_semantic_map = processor.post_process_semantic_segmentation(
...     outputs, target_sizes=[(image.height, image.width)]
... )[0]
>>> f"👉 Semantic Predictions Shape: {list(predicted_semantic_map.shape)}"
'👉 Semantic Predictions Shape: [512, 683]'

>>> # Instance Segmentation
>>> inputs = processor(image, ["instance"], return_tensors="pt")

>>> with torch.no_grad():
...     outputs = model(**inputs)
>>> # model predicts class_queries_logits of shape `(batch_size, num_queries)`
>>> # and masks_queries_logits of shape `(batch_size, num_queries, height, width)`
>>> class_queries_logits = outputs.class_queries_logits
>>> masks_queries_logits = outputs.masks_queries_logits

>>> # you can pass them to processor for instance postprocessing
>>> predicted_instance_map = processor.post_process_instance_segmentation(
...     outputs, target_sizes=[(image.height, image.width)]
... )[0]["segmentation"]
>>> f"👉 Instance Predictions Shape: {list(predicted_instance_map.shape)}"
'👉 Instance Predictions Shape: [512, 683]'

>>> # Panoptic Segmentation
>>> inputs = processor(image, ["panoptic"], return_tensors="pt")

>>> with torch.no_grad():
...     outputs = model(**inputs)
>>> # model predicts class_queries_logits of shape `(batch_size, num_queries)`
>>> # and masks_queries_logits of shape `(batch_size, num_queries, height, width)`
>>> class_queries_logits = outputs.class_queries_logits
>>> masks_queries_logits = outputs.masks_queries_logits

>>> # you can pass them to processor for panoptic postprocessing
>>> predicted_panoptic_map = processor.post_process_panoptic_segmentation(
...     outputs, target_sizes=[(image.height, image.width)]
... )[0]["segmentation"]
>>> f"👉 Panoptic Predictions Shape: {list(predicted_panoptic_map.shape)}"
'👉 Panoptic Predictions Shape: [512, 683]'
```
