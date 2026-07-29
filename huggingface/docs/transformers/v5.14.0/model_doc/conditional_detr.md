# Conditional DETR

## Overview

The Conditional DETR model was proposed in [Conditional DETR for Fast Training Convergence](https://huggingface.co/papers/2108.06152) by Depu Meng, Xiaokang Chen, Zejia Fan, Gang Zeng, Houqiang Li, Yuhui Yuan, Lei Sun, Jingdong Wang. Conditional DETR presents a conditional cross-attention mechanism for fast DETR training. Conditional DETR converges 6.7× to 10× faster than DETR.

The abstract from the paper is the following:

*The recently-developed DETR approach applies the transformer encoder and decoder architecture to object detection and achieves promising performance. In this paper, we handle the critical issue, slow training convergence, and present a conditional cross-attention mechanism for fast DETR training. Our approach is motivated by that the cross-attention in DETR relies highly on the content embeddings for localizing the four extremities and predicting the box, which increases the need for high-quality content embeddings and thus the training difficulty. Our approach, named conditional DETR, learns a conditional spatial query from the decoder embedding for decoder multi-head cross-attention. The benefit is that through the conditional spatial query, each cross-attention head is able to attend to a band containing a distinct region, e.g., one object extremity or a region inside the object box. This narrows down the spatial range for localizing the distinct regions for object classification and box regression, thus relaxing the dependence on the content embeddings and easing the training. Empirical results show that conditional DETR converges 6.7× faster for the backbones R50 and R101 and 10× faster for stronger backbones DC5-R50 and DC5-R101. Code is available at https://github.com/Atten4Vis/ConditionalDETR.*

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/conditional_detr_curve.jpg"
alt="drawing" width="600"/>

 Conditional DETR shows much faster convergence compared to the original DETR. Taken from the original paper.

This model was contributed by [DepuMeng](https://huggingface.co/DepuMeng). The original code can be found [here](https://github.com/Atten4Vis/ConditionalDETR).

## Resources

- Scripts for finetuning [ConditionalDetrForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/conditional_detr#transformers.ConditionalDetrForObjectDetection) with [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer) or [Accelerate](https://huggingface.co/docs/accelerate/index) can be found [here](https://github.com/huggingface/transformers/tree/main/examples/pytorch/object-detection).
- See also: [Object detection task guide](../tasks/object_detection).

## ConditionalDetrConfig[[transformers.ConditionalDetrConfig]]

- **is_encoder_decoder** (`bool`, *optional*, defaults to `True`) --
  Whether the model is used as an encoder/decoder or not.
- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **num_queries** (`int`, *optional*, defaults to 300) --
  Number of object queries, i.e. detection slots. This is the maximal number of objects
  [ConditionalDetrModel](/docs/transformers/v5.14.0/en/model_doc/conditional_detr#transformers.ConditionalDetrModel) can detect in a single image. For COCO, we recommend 100 queries.
- **encoder_layers** (`int`, *optional*, defaults to `6`) --
  Number of hidden layers in the Transformer encoder. Will use the same value as `num_layers` if not set.
- **encoder_ffn_dim** (`int`, *optional*, defaults to `2048`) --
  Dimensionality of the "intermediate" (often named feed-forward) layer in encoder.
- **encoder_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer encoder.
- **decoder_layers** (`int`, *optional*, defaults to `6`) --
  Number of hidden layers in the Transformer decoder. Will use the same value as `num_layers` if not set.
- **decoder_ffn_dim** (`int`, *optional*, defaults to `2048`) --
  Dimensionality of the "intermediate" (often named feed-forward) layer in decoder.
- **decoder_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **encoder_layerdrop** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The LayerDrop probability for the encoder. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556)
  for more details.
- **decoder_layerdrop** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The LayerDrop probability for the decoder. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556)
  for more details.
- **activation_function** (`str`, *optional*, defaults to `relu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **d_model** (`int`, *optional*, defaults to `256`) --
  Size of the encoder layers and the pooler layer.
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The ratio for all dropout layers.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **activation_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for activations inside the fully connected layer.
- **init_std** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **init_xavier_std** (`float`, *optional*, defaults to `1.0`) --
  The scaling factor used for the Xavier initialization of the cross-attention weights.
- **auxiliary_loss** (`bool`, *optional*, defaults to `False`) --
  Whether auxiliary decoding losses (loss at each decoder layer) are to be used.
- **position_embedding_type** (`str`, *optional*, defaults to `"sine"`) --
  Type of position embeddings to be used on top of the image features. One of `"sine"` or `"learned"`.
- **dilation** (`bool`, *optional*, defaults to `False`) --
  Whether to replace stride with dilation in the last convolutional block (DC5). Only supported when
  `use_timm_backbone` = `True`.
- **class_cost** (`int`, *optional*, defaults to `2`) --
  Relative weight of the classification error in the Hungarian matching cost.
- **bbox_cost** (`int`, *optional*, defaults to `5`) --
  Relative weight of the L1 bounding box error in the Hungarian matching cost.
- **giou_cost** (`int`, *optional*, defaults to `2`) --
  Relative weight of the generalized IoU loss in the Hungarian matching cost.
- **mask_loss_coefficient** (`int`, *optional*, defaults to `1`) --
  Relative weight of the focal loss in the panoptic segmentation loss.
- **dice_loss_coefficient** (`int`, *optional*, defaults to `1`) --
  Relative weight of the dice loss in the panoptic segmentation loss.
- **cls_loss_coefficient** (`int`, *optional*, defaults to `2`) --
  Relative weight of the classification loss in the panoptic segmentation loss.
- **bbox_loss_coefficient** (`int`, *optional*, defaults to `5`) --
  Relative weight of the L1 bounding box loss in the panoptic segmentation loss.
- **giou_loss_coefficient** (`int`, *optional*, defaults to `2`) --
  Relative weight of the generalized IoU loss in the panoptic segmentation loss.
- **focal_alpha** (`float`, *optional*, defaults to `0.25`) --
  Alpha parameter in the focal loss.

This is the configuration class to store the configuration of a ConditionalDetrModel. It is used to instantiate a Conditional Detr
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [microsoft/conditional-detr-resnet-50](https://huggingface.co/microsoft/conditional-detr-resnet-50)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import ConditionalDetrConfig, ConditionalDetrModel

>>> # Initializing a Conditional DETR microsoft/conditional-detr-resnet-50 style configuration
>>> configuration = ConditionalDetrConfig()

>>> # Initializing a model (with random weights) from the microsoft/conditional-detr-resnet-50 style configuration
>>> model = ConditionalDetrModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## ConditionalDetrImageProcessor[[transformers.ConditionalDetrImageProcessor]]

- **format** (`str`, *kwargs*, *optional*, defaults to `AnnotationFormat.COCO_DETECTION`) --
  Data format of the annotations. One of "coco_detection" or "coco_panoptic".
- **do_convert_annotations** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Controls whether to convert the annotations to the format expected by the CONDITIONAL_DETR model. Converts the
  bounding boxes to the format `(center_x, center_y, width, height)` and in the range `[0, 1]`.
  Can be overridden by the `do_convert_annotations` parameter in the `preprocess` method.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a ConditionalDetrImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **annotations** (`AnnotationType` or `list[AnnotationType]`, *optional*) --
  Annotations to transform according to the padding that is applied to the images.
- **return_segmentation_masks** (`bool`, *optional*, defaults to `self.return_segmentation_masks`) --
  Whether to return segmentation masks.
- **masks_path** (`str` or `pathlib.Path`, *optional*) --
  Path to the directory containing the segmentation masks.
- **format** (`str`, *kwargs*, *optional*, defaults to `AnnotationFormat.COCO_DETECTION`) --
  Data format of the annotations. One of "coco_detection" or "coco_panoptic".
- **do_convert_annotations** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Controls whether to convert the annotations to the format expected by the CONDITIONAL_DETR model. Converts the
  bounding boxes to the format `(center_x, center_y, width, height)` and in the range `[0, 1]`.
  Can be overridden by the `do_convert_annotations` parameter in the `preprocess` method.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## ConditionalDetrImageProcessorPil[[transformers.ConditionalDetrImageProcessorPil]]

- **format** (`str`, *kwargs*, *optional*, defaults to `AnnotationFormat.COCO_DETECTION`) --
  Data format of the annotations. One of "coco_detection" or "coco_panoptic".
- **do_convert_annotations** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Controls whether to convert the annotations to the format expected by the CONDITIONAL_DETR model. Converts the
  bounding boxes to the format `(center_x, center_y, width, height)` and in the range `[0, 1]`.
  Can be overridden by the `do_convert_annotations` parameter in the `preprocess` method.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a ConditionalDetrImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **annotations** (`AnnotationType` or `list[AnnotationType]`, *optional*) --
  Annotations to transform according to the padding that is applied to the images.
- **return_segmentation_masks** (`bool`, *optional*, defaults to `self.return_segmentation_masks`) --
  Whether to return segmentation masks.
- **masks_path** (`str` or `pathlib.Path`, *optional*) --
  Path to the directory containing the segmentation masks.
- **format** (`str`, *kwargs*, *optional*, defaults to `AnnotationFormat.COCO_DETECTION`) --
  Data format of the annotations. One of "coco_detection" or "coco_panoptic".
- **do_convert_annotations** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Controls whether to convert the annotations to the format expected by the CONDITIONAL_DETR model. Converts the
  bounding boxes to the format `(center_x, center_y, width, height)` and in the range `[0, 1]`.
  Can be overridden by the `do_convert_annotations` parameter in the `preprocess` method.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** (`ConditionalDetrObjectDetectionOutput`) --
  Raw outputs of the model.
- **threshold** (`float`, *optional*) --
  Score threshold to keep object detection predictions.
- **target_sizes** (`torch.Tensor` or `list[tuple[int, int]]`, *optional*) --
  Tensor of shape `(batch_size, 2)` or list of tuples (`tuple[int, int]`) containing the target size
  (height, width) of each image in the batch. If left to None, predictions will not be resized.
- **top_k** (`int`, *optional*, defaults to 100) --
  Keep only top k bounding boxes before filtering by thresholding.`list[Dict]`A list of dictionaries, each dictionary containing the scores, labels and boxes for an image
in the batch as predicted by the model.

Converts the raw output of [ConditionalDetrForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/conditional_detr#transformers.ConditionalDetrForObjectDetection) into final bounding boxes in (top_left_x,
top_left_y, bottom_right_x, bottom_right_y) format. Only supports PyTorch.

- **outputs** ([ConditionalDetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/conditional_detr#transformers.ConditionalDetrForSegmentation)) --
  Raw outputs of the model.
- **threshold** (`float`, *optional*, defaults to 0.5) --
  The probability score threshold to keep predicted instance masks.
- **mask_threshold** (`float`, *optional*, defaults to 0.5) --
  Threshold to use when turning the predicted masks into binary values.
- **overlap_mask_area_threshold** (`float`, *optional*, defaults to 0.8) --
  The overlap mask area threshold to merge or discard small disconnected parts within each binary
  instance mask.
- **target_sizes** (`list[Tuple]`, *optional*) --
  List of length (batch_size), where each list item (`tuple[int, int]]`) corresponds to the requested
  final size (height, width) of each prediction. If unset, predictions will not be resized.
- **return_coco_annotation** (`bool`, *optional*) --
  Defaults to `False`. If set to `True`, segmentation maps are returned in COCO run-length encoding (RLE)
  format.`list[Dict]`A list of dictionaries, one per image, each dictionary containing two keys:
- **segmentation** -- A tensor of shape `(height, width)` where each pixel represents a `segment_id` or
  `list[List]` run-length encoding (RLE) of the segmentation map if return_coco_annotation is set to
  `True`. Set to `None` if no mask if found above `threshold`.
- **segments_info** -- A dictionary that contains additional information on each segment.
  - **id** -- An integer representing the `segment_id`.
  - **label_id** -- An integer representing the label / semantic class id corresponding to `segment_id`.
  - **score** -- Prediction score of segment with `segment_id`.

Converts the output of [ConditionalDetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/conditional_detr#transformers.ConditionalDetrForSegmentation) into instance segmentation predictions. Only supports PyTorch.

- **outputs** ([ConditionalDetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/conditional_detr#transformers.ConditionalDetrForSegmentation)) --
  Raw outputs of the model.
- **target_sizes** (`list[tuple[int, int]]`, *optional*) --
  A list of tuples (`tuple[int, int]`) containing the target size (height, width) of each image in the
  batch. If unset, predictions will not be resized.
- **return_segmentation_scores** (`bool`, *optional*, defaults to `False`) --
  Whether to return segmentation scores alongside the segmentation map. When `True`, each element of
  the returned list is a `SemanticSegmentationPostProcessorOutput` with fields `segmentation`
  (class IDs, shape `(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`).`list[torch.Tensor]` or `list[SemanticSegmentationPostProcessorOutput]`When
`return_segmentation_scores=False` (default), a list of length `batch_size` where each item is a
segmentation map of shape `(height, width)` with class IDs. When `return_segmentation_scores=True`,
a list of `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape
`(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`). In both cases,
`(height, width)` corresponds to the target size (if `target_sizes` is specified).

Converts the output of [ConditionalDetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/conditional_detr#transformers.ConditionalDetrForSegmentation) into semantic segmentation maps. Only supports PyTorch.

- **outputs** ([ConditionalDetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/conditional_detr#transformers.ConditionalDetrForSegmentation)) --
  The outputs from [ConditionalDetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/conditional_detr#transformers.ConditionalDetrForSegmentation).
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
- **target_sizes** (`list[Tuple]`, *optional*) --
  List of length (batch_size), where each list item (`tuple[int, int]]`) corresponds to the requested
  final size (height, width) of each prediction in batch. If unset, predictions will not be resized.`list[Dict]`A list of dictionaries, one per image, each dictionary containing two keys:
- **segmentation** -- a tensor of shape `(height, width)` where each pixel represents a `segment_id` or
  `None` if no mask if found above `threshold`. If `target_sizes` is specified, segmentation is resized to
  the corresponding `target_sizes` entry.
- **segments_info** -- A dictionary that contains additional information on each segment.
  - **id** -- an integer representing the `segment_id`.
  - **label_id** -- An integer representing the label / semantic class id corresponding to `segment_id`.
  - **was_fused** -- a boolean, `True` if `label_id` was in `label_ids_to_fuse`, `False` otherwise.
    Multiple instances of the same class / label were fused and assigned a single `segment_id`.
  - **score** -- Prediction score of segment with `segment_id`.

Converts the output of [ConditionalDetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/conditional_detr#transformers.ConditionalDetrForSegmentation) into image panoptic segmentation predictions. Only supports
PyTorch.

## ConditionalDetrModel[[transformers.ConditionalDetrModel]]

- **config** ([ConditionalDetrConfig](/docs/transformers/v5.14.0/en/model_doc/conditional_detr#transformers.ConditionalDetrConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare CONDITIONAL_DETR Model (consisting of a backbone and encoder-decoder Transformer) outputting raw hidden-states without
any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses
  `image_processor_class` for processing images).
- **pixel_mask** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:

  - 1 for pixels that are real (i.e. **not masked**),
  - 0 for pixels that are padding (i.e. **masked**).

  [What are attention masks?](../glossary#attention-mask)
- **decoder_attention_mask** (`torch.FloatTensor` of shape `(batch_size, num_queries)`, *optional*) --
  Not used by default. Can be used to mask object queries.
- **encoder_outputs** (`torch.FloatTensor`, *optional*) --
  Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`)
  `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of
  hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing the flattened feature map (output of the backbone + projection layer), you
  can choose to directly pass a flattened representation of an image.
- **decoder_inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, num_queries, hidden_size)`, *optional*) --
  Optionally, instead of initializing the queries with a tensor of zeros, you can choose to directly pass an
  embedded representation.`ConditionalDetrModelOutput` or `tuple(torch.FloatTensor)`A `ConditionalDetrModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.
The [ConditionalDetrModel](/docs/transformers/v5.14.0/en/model_doc/conditional_detr#transformers.ConditionalDetrModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the decoder of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [EncoderDecoderCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.
- **decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the decoder at the output of each layer plus the optional initial embedding outputs.
- **decoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **cross_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the encoder at the output of each layer plus the optional initial embedding outputs.
- **encoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **intermediate_hidden_states** (`torch.FloatTensor` of shape `(config.decoder_layers, batch_size, sequence_length, hidden_size)`, *optional*, returned when `config.auxiliary_loss=True`) -- Intermediate decoder activations, i.e. the output of each decoder layer, each of them gone through a
  layernorm.
- **reference_points** (`torch.FloatTensor` of shape `(config.decoder_layers, batch_size, num_queries, 2 (anchor points))`) -- Reference points (reference points of each layer of the decoder).

Examples:

```python
>>> from transformers import AutoImageProcessor, AutoModel
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("microsoft/conditional-detr-resnet-50")
>>> model = AutoModel.from_pretrained("microsoft/conditional-detr-resnet-50")

>>> # prepare image for the model
>>> inputs = image_processor(images=image, return_tensors="pt")

>>> # forward pass
>>> outputs = model(**inputs)

>>> # the last hidden states are the final query embeddings of the Transformer decoder
>>> # these are of shape (batch_size, num_queries, hidden_size)
>>> last_hidden_states = outputs.last_hidden_state
>>> list(last_hidden_states.shape)
[1, 300, 256]
```

## ConditionalDetrForObjectDetection[[transformers.ConditionalDetrForObjectDetection]]

- **config** ([ConditionalDetrConfig](/docs/transformers/v5.14.0/en/model_doc/conditional_detr#transformers.ConditionalDetrConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

CONDITIONAL_DETR Model (consisting of a backbone and encoder-decoder Transformer) with object detection heads on top, for tasks
such as COCO detection.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses
  `image_processor_class` for processing images).
- **pixel_mask** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:

  - 1 for pixels that are real (i.e. **not masked**),
  - 0 for pixels that are padding (i.e. **masked**).

  [What are attention masks?](../glossary#attention-mask)
- **decoder_attention_mask** (`torch.FloatTensor` of shape `(batch_size, num_queries)`, *optional*) --
  Not used by default. Can be used to mask object queries.
- **encoder_outputs** (`torch.FloatTensor`, *optional*) --
  Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`)
  `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of
  hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing the flattened feature map (output of the backbone + projection layer), you
  can choose to directly pass a flattened representation of an image.
- **decoder_inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, num_queries, hidden_size)`, *optional*) --
  Optionally, instead of initializing the queries with a tensor of zeros, you can choose to directly pass an
  embedded representation.
- **labels** (`list[Dict]` of len `(batch_size,)`, *optional*) --
  Labels for computing the bipartite matching loss. List of dicts, each dictionary containing at least the
  following 2 keys: 'class_labels' and 'boxes' (the class labels and bounding boxes of an image in the batch
  respectively). The class labels themselves should be a `torch.LongTensor` of len `(number of bounding boxes
  in the image,)` and the boxes a `torch.FloatTensor` of shape `(number of bounding boxes in the image, 4)`.</paramsdesc><rettype>`ConditionalDetrObjectDetectionOutput` or `tuple(torch.FloatTensor)`</rettype><retdesc>A `ConditionalDetrObjectDetectionOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.
The [ConditionalDetrForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/conditional_detr#transformers.ConditionalDetrForObjectDetection) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` are provided)) -- Total loss as a linear combination of a negative log-likelihood (cross-entropy) for class prediction and a
  bounding box loss. The latter is defined as a linear combination of the L1 loss and the generalized
  scale-invariant IoU loss.
- **loss_dict** (`Dict`, *optional*) -- A dictionary containing the individual losses. Useful for logging.
- **logits** (`torch.FloatTensor` of shape `(batch_size, num_queries, num_classes + 1)`) -- Classification logits (including no-object) for all queries.
- **pred_boxes** (`torch.FloatTensor` of shape `(batch_size, num_queries, 4)`) -- Normalized boxes coordinates for all queries, represented as (center_x, center_y, width, height). These
  values are normalized in [0, 1], relative to the size of each individual image in the batch (disregarding
  possible padding). You can use `post_process_object_detection()` to retrieve the
  unnormalized bounding boxes.
- **auxiliary_outputs** (`list[Dict]`, *optional*) -- Optional, only returned when auxiliary losses are activated (i.e. `config.auxiliary_loss` is set to `True`)
  and labels are provided. It is a list of dictionaries containing the two above keys (`logits` and
  `pred_boxes`) for each decoder layer.
- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the decoder of the model.
- **decoder_hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the decoder at the output of each layer plus the initial embedding outputs.
- **decoder_attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **cross_attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the encoder at the output of each layer plus the initial embedding outputs.
- **encoder_attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.

Examples:

```python
>>> from transformers import AutoImageProcessor, AutoModelForObjectDetection
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("microsoft/conditional-detr-resnet-50")
>>> model = AutoModelForObjectDetection.from_pretrained("microsoft/conditional-detr-resnet-50")

>>> inputs = image_processor(images=image, return_tensors="pt")

>>> outputs = model(**inputs)

>>> # convert outputs (bounding boxes and class logits) to Pascal VOC format (xmin, ymin, xmax, ymax)
>>> target_sizes = torch.tensor([image.size[::-1]])
>>> results = image_processor.post_process_object_detection(outputs, threshold=0.5, target_sizes=target_sizes)[
...     0
... ]
>>> for score, label, box in zip(results["scores"], results["labels"], results["boxes"]):
...     box = [round(i, 2) for i in box.tolist()]
...     print(
...         f"Detected {model.config.id2label[label.item()]} with confidence "
...         f"{round(score.item(), 3)} at location {box}"
...     )
Detected remote with confidence 0.833 at location [38.31, 72.1, 177.63, 118.45]
Detected cat with confidence 0.831 at location [9.2, 51.38, 321.13, 469.0]
Detected cat with confidence 0.804 at location [340.3, 16.85, 642.93, 370.95]
Detected remote with confidence 0.683 at location [334.48, 73.49, 366.37, 190.01]
Detected couch with confidence 0.535 at location [0.52, 1.19, 640.35, 475.1]
```

## ConditionalDetrForSegmentation[[transformers.ConditionalDetrForSegmentation]]

- **config** ([ConditionalDetrConfig](/docs/transformers/v5.14.0/en/model_doc/conditional_detr#transformers.ConditionalDetrConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

CONDITIONAL_DETR Model (consisting of a backbone and encoder-decoder Transformer) with a segmentation head on top, for tasks
such as COCO panoptic.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses
  `image_processor_class` for processing images).
- **pixel_mask** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:

  - 1 for pixels that are real (i.e. **not masked**),
  - 0 for pixels that are padding (i.e. **masked**).

  [What are attention masks?](../glossary#attention-mask)
- **decoder_attention_mask** (`torch.FloatTensor` of shape `(batch_size, num_queries)`, *optional*) --
  Mask to avoid performing attention on certain object queries in the decoder. Mask values selected in `[0, 1]`:

  - 1 for queries that are **not masked**,
  - 0 for queries that are **masked**.
- **encoder_outputs** (`torch.FloatTensor`, *optional*) --
  Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`)
  `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of
  hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Kept for backward compatibility, but cannot be used for segmentation, as segmentation requires
  multi-scale features from the backbone that are not available when bypassing it with inputs_embeds.
- **decoder_inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, num_queries, hidden_size)`, *optional*) --
  Optionally, instead of initializing the queries with a tensor of zeros, you can choose to directly pass an
  embedded representation. Useful for tasks that require custom query initialization.
- **labels** (`list[Dict]` of len `(batch_size,)`, *optional*) --
  Labels for computing the bipartite matching loss, DICE/F-1 loss and Focal loss. List of dicts, each
  dictionary containing at least the following 3 keys: 'class_labels', 'boxes' and 'masks' (the class labels,
  bounding boxes and segmentation masks of an image in the batch respectively). The class labels themselves
  should be a `torch.LongTensor` of len `(number of bounding boxes in the image,)`, the boxes a
  `torch.FloatTensor` of shape `(number of bounding boxes in the image, 4)` and the masks a
  `torch.FloatTensor` of shape `(number of bounding boxes in the image, height, width)`.`ConditionalDetrSegmentationOutput` or `tuple(torch.FloatTensor)`A `ConditionalDetrSegmentationOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.
The [ConditionalDetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/conditional_detr#transformers.ConditionalDetrForSegmentation) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` are provided)) -- Total loss as a linear combination of a negative log-likelihood (cross-entropy) for class prediction and a
  bounding box loss. The latter is defined as a linear combination of the L1 loss and the generalized
  scale-invariant IoU loss.
- **loss_dict** (`Dict`, *optional*) -- A dictionary containing the individual losses. Useful for logging.
- **logits** (`torch.FloatTensor` of shape `(batch_size, num_queries, num_classes + 1)`) -- Classification logits (including no-object) for all queries.
- **pred_boxes** (`torch.FloatTensor` of shape `(batch_size, num_queries, 4)`) -- Normalized boxes coordinates for all queries, represented as (center_x, center_y, width, height). These
  values are normalized in [0, 1], relative to the size of each individual image in the batch (disregarding
  possible padding). You can use `post_process_object_detection()` to retrieve the
  unnormalized bounding boxes.
- **pred_masks** (`torch.FloatTensor` of shape `(batch_size, num_queries, height/4, width/4)`) -- Segmentation masks logits for all queries. See also
  `post_process_semantic_segmentation()` or
  `post_process_instance_segmentation()`
  `post_process_panoptic_segmentation()` to evaluate semantic, instance and panoptic
  segmentation masks respectively.
- **auxiliary_outputs** (`list[Dict]`, *optional*) -- Optional, only returned when auxiliary losses are activated (i.e. `config.auxiliary_loss` is set to `True`)
  and labels are provided. It is a list of dictionaries containing the two above keys (`logits` and
  `pred_boxes`) for each decoder layer.
- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the decoder of the model.
- **decoder_hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the decoder at the output of each layer plus the initial embedding outputs.
- **decoder_attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **cross_attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the encoder at the output of each layer plus the initial embedding outputs.
- **encoder_attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.

Examples:

```python
>>> import io
>>> import httpx
>>> from io import BytesIO
>>> from PIL import Image
>>> import torch
>>> import numpy

>>> from transformers import AutoImageProcessor, ConditionalDetrForSegmentation
>>> from transformers.image_transforms import rgb_to_id

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("facebook/conditional_detr-resnet-50-panoptic")
>>> model = ConditionalDetrForSegmentation.from_pretrained("facebook/conditional_detr-resnet-50-panoptic")

>>> # prepare image for the model
>>> inputs = image_processor(images=image, return_tensors="pt")

>>> # forward pass
>>> outputs = model(**inputs)

>>> # Use the `post_process_panoptic_segmentation` method of the `image_processor` to retrieve post-processed panoptic segmentation maps
>>> # Segmentation results are returned as a list of dictionaries
>>> result = image_processor.post_process_panoptic_segmentation(outputs, target_sizes=[(300, 500)])

>>> # A tensor of shape (height, width) where each value denotes a segment id, filled with -1 if no segment is found
>>> panoptic_seg = result[0]["segmentation"]
>>> panoptic_seg.shape
torch.Size([300, 500])
>>> # Get prediction score and segment_id to class_id mapping of each segment
>>> panoptic_segments_info = result[0]["segments_info"]
>>> len(panoptic_segments_info)
5
```
