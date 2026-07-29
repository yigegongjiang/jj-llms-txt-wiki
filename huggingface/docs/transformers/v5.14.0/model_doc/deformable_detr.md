# Deformable DETR

[Deformable DETR](https://huggingface.co/papers/2010.04159) improves on the original [DETR](./detr) by using a deformable attention module. This mechanism selectively attends to a small set of key sampling points around a reference. It improves training speed and improves accuracy.

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/deformable_detr_architecture.png"
alt="drawing" width="600"/>

 Deformable DETR architecture. Taken from the original paper.

You can find all the available Deformable DETR checkpoints under the [SenseTime](https://huggingface.co/SenseTime) organization.

> [!TIP]
> This model was contributed by [nielsr](https://huggingface.co/nielsr).
>
> Click on the Deformable DETR models in the right sidebar for more examples of how to apply Deformable DETR to different object detection and segmentation tasks.

The example below demonstrates how to perform object detection with the [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) and the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) class.

```python

from transformers import pipeline

pipeline = pipeline(
    "object-detection",
    model="SenseTime/deformable-detr",
    device_map=0
)

pipeline("http://images.cocodataset.org/val2017/000000039769.jpg")
```

```python
import requests
import torch
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForObjectDetection

url = "http://images.cocodataset.org/val2017/000000039769.jpg"
image = Image.open(requests.get(url, stream=True).raw)

image_processor = AutoImageProcessor.from_pretrained("SenseTime/deformable-detr")
model = AutoModelForObjectDetection.from_pretrained("SenseTime/deformable-detr", device_map="auto")

# prepare image for the model
inputs = image_processor(images=image, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

results = image_processor.post_process_object_detection(outputs, target_sizes=torch.tensor([image.size[::-1]]), threshold=0.3)

for result in results:
    for score, label_id, box in zip(result["scores"], result["labels"], result["boxes"]):
        score, label = score.item(), label_id.item()
        box = [round(i, 2) for i in box.tolist()]
        print(f"{model.config.id2label[label]}: {score:.2f} {box}")
```

## Resources

- Refer to this set of [notebooks](https://github.com/NielsRogge/Transformers-Tutorials/tree/master/Deformable-DETR) for inference and fine-tuning [DeformableDetrForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/deformable_detr#transformers.DeformableDetrForObjectDetection) on a custom dataset.

## DeformableDetrImageProcessor[[transformers.DeformableDetrImageProcessor]]

- **format** (`str`, *kwargs*, *optional*, defaults to `AnnotationFormat.COCO_DETECTION`) --
  Data format of the annotations. One of "coco_detection" or "coco_panoptic".
- **do_convert_annotations** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Controls whether to convert the annotations to the format expected by the DEFORMABLE_DETR model. Converts the
  bounding boxes to the format `(center_x, center_y, width, height)` and in the range `[0, 1]`.
  Can be overridden by the `do_convert_annotations` parameter in the `preprocess` method.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a DeformableDetrImageProcessor image processor.

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
  Controls whether to convert the annotations to the format expected by the DEFORMABLE_DETR model. Converts the
  bounding boxes to the format `(center_x, center_y, width, height)` and in the range `[0, 1]`.
  Can be overridden by the `do_convert_annotations` parameter in the `preprocess` method.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** (`DetrObjectDetectionOutput`) --
  Raw outputs of the model.
- **threshold** (`float`, *optional*) --
  Score threshold to keep object detection predictions.
- **target_sizes** (`torch.Tensor` or `list[tuple[int, int]]`, *optional*) --
  Tensor of shape `(batch_size, 2)` or list of tuples (`tuple[int, int]`) containing the target size
  (height, width) of each image in the batch. If left to None, predictions will not be resized.
- **top_k** (`int`, *optional*, defaults to 100) --
  Keep only top k bounding boxes before filtering by thresholding.`list[Dict]`A list of dictionaries, each dictionary containing the scores, labels and boxes for an image
in the batch as predicted by the model.

Converts the raw output of [DeformableDetrForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/deformable_detr#transformers.DeformableDetrForObjectDetection) into final bounding boxes in (top_left_x,
top_left_y, bottom_right_x, bottom_right_y) format. Only supports PyTorch.

## DeformableDetrImageProcessorPil[[transformers.DeformableDetrImageProcessorPil]]

- **format** (`str`, *kwargs*, *optional*, defaults to `AnnotationFormat.COCO_DETECTION`) --
  Data format of the annotations. One of "coco_detection" or "coco_panoptic".
- **do_convert_annotations** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Controls whether to convert the annotations to the format expected by the DEFORMABLE_DETR model. Converts the
  bounding boxes to the format `(center_x, center_y, width, height)` and in the range `[0, 1]`.
  Can be overridden by the `do_convert_annotations` parameter in the `preprocess` method.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a DeformableDetrImageProcessor image processor.

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
  Controls whether to convert the annotations to the format expected by the DEFORMABLE_DETR model. Converts the
  bounding boxes to the format `(center_x, center_y, width, height)` and in the range `[0, 1]`.
  Can be overridden by the `do_convert_annotations` parameter in the `preprocess` method.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** (`DetrObjectDetectionOutput`) --
  Raw outputs of the model.
- **threshold** (`float`, *optional*) --
  Score threshold to keep object detection predictions.
- **target_sizes** (`torch.Tensor` or `list[tuple[int, int]]`, *optional*) --
  Tensor of shape `(batch_size, 2)` or list of tuples (`tuple[int, int]`) containing the target size
  (height, width) of each image in the batch. If left to None, predictions will not be resized.
- **top_k** (`int`, *optional*, defaults to 100) --
  Keep only top k bounding boxes before filtering by thresholding.`list[Dict]`A list of dictionaries, each dictionary containing the scores, labels and boxes for an image
in the batch as predicted by the model.

Converts the raw output of [DeformableDetrForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/deformable_detr#transformers.DeformableDetrForObjectDetection) into final bounding boxes in (top_left_x,
top_left_y, bottom_right_x, bottom_right_y) format. Only supports PyTorch.

## DeformableDetrConfig[[transformers.DeformableDetrConfig]]

- **is_encoder_decoder** (`bool`, *optional*, defaults to `True`) --
  Whether the model is used as an encoder/decoder or not.
- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **num_queries** (`int`, *optional*, defaults to 300) --
  Number of object queries, i.e. detection slots. This is the maximal number of objects
  [DeformableDetrModel](/docs/transformers/v5.14.0/en/model_doc/deformable_detr#transformers.DeformableDetrModel) can detect in a single image. In case `two_stage` is set to `True`, we use
  `two_stage_num_proposals` instead.
- **max_position_embeddings** (`int`, *optional*, defaults to `1024`) --
  The maximum sequence length that this model might ever be used with.
- **encoder_layers** (`int`, *optional*, defaults to `6`) --
  Number of hidden layers in the Transformer encoder. Will use the same value as `num_layers` if not set.
- **encoder_ffn_dim** (`int`, *optional*, defaults to `1024`) --
  Dimensionality of the "intermediate" (often named feed-forward) layer in encoder.
- **encoder_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer encoder.
- **decoder_layers** (`int`, *optional*, defaults to `6`) --
  Number of hidden layers in the Transformer decoder. Will use the same value as `num_layers` if not set.
- **decoder_ffn_dim** (`int`, *optional*, defaults to `1024`) --
  Dimensionality of the "intermediate" (often named feed-forward) layer in decoder.
- **decoder_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **encoder_layerdrop** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The LayerDrop probability for the encoder. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556)
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
- **return_intermediate** (`bool`, *optional*, defaults to True) --
  Whether to return the intermediate state or not
- **auxiliary_loss** (`bool`, *optional*, defaults to `False`) --
  Whether auxiliary decoding losses (losses at each decoder layer) are to be used.
- **position_embedding_type** (`str`, *optional*, defaults to `"sine"`) --
  Type of position embeddings to be used on top of the image features. One of `"sine"` or `"learned"`.
- **dilation** (`bool`, *optional*, defaults to `False`) --
  Whether to replace stride with dilation in the last convolutional block (DC5). Only supported when
  `use_timm_backbone` = `True`.
- **num_feature_levels** (`int`, *optional*, defaults to 4) --
  The number of input feature levels.
- **encoder_n_points** (`int`, *optional*, defaults to 4) --
  The number of sampled keys in each feature level for each attention head in the encoder.
- **decoder_n_points** (`int`, *optional*, defaults to 4) --
  The number of sampled keys in each feature level for each attention head in the decoder.
- **two_stage** (`bool`, *optional*, defaults to `False`) --
  Whether to apply a two-stage deformable DETR, where the region proposals are also generated by a variant of
  Deformable DETR, which are further fed into the decoder for iterative bounding box refinement.
- **two_stage_num_proposals** (`int`, *optional*, defaults to 300) --
  The number of region proposals to be generated, in case `two_stage` is set to `True`.
- **with_box_refine** (`bool`, *optional*, defaults to `False`) --
  Whether to apply iterative bounding box refinement, where each decoder layer refines the bounding boxes
  based on the predictions from the previous layer.
- **class_cost** (`int`, *optional*, defaults to `1`) --
  Relative weight of the classification error in the Hungarian matching cost.
- **bbox_cost** (`int`, *optional*, defaults to `5`) --
  Relative weight of the L1 bounding box error in the Hungarian matching cost.
- **giou_cost** (`int`, *optional*, defaults to `2`) --
  Relative weight of the generalized IoU loss in the Hungarian matching cost.
- **mask_loss_coefficient** (`int`, *optional*, defaults to `1`) --
  Relative weight of the focal loss in the panoptic segmentation loss.
- **dice_loss_coefficient** (`int`, *optional*, defaults to `1`) --
  Relative weight of the dice loss in the panoptic segmentation loss.
- **bbox_loss_coefficient** (`int`, *optional*, defaults to `5`) --
  Relative weight of the L1 bounding box loss in the panoptic segmentation loss.
- **giou_loss_coefficient** (`int`, *optional*, defaults to `2`) --
  Relative weight of the generalized IoU loss in the panoptic segmentation loss.
- **eos_coefficient** (`float`, *optional*, defaults to `0.1`) --
  Relative classification weight of the 'no-object' class in the object detection loss.
- **focal_alpha** (`float`, *optional*, defaults to `0.25`) --
  Alpha parameter in the focal loss.
- **disable_custom_kernels** (`bool`, *optional*, defaults to `False`) --
  Disable the use of custom CUDA and CPU kernels. This option is necessary for the ONNX export, as custom
  kernels are not supported by PyTorch ONNX export.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a DeformableDetrModel. It is used to instantiate a Deformable Detr
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [SenseTime/deformable-detr](https://huggingface.co/SenseTime/deformable-detr)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import DeformableDetrConfig, DeformableDetrModel

>>> # Initializing a Deformable DETR SenseTime/deformable-detr style configuration
>>> configuration = DeformableDetrConfig()

>>> # Initializing a model (with random weights) from the SenseTime/deformable-detr style configuration
>>> model = DeformableDetrModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## DeformableDetrModel[[transformers.DeformableDetrModel]]

- **config** ([DeformableDetrConfig](/docs/transformers/v5.14.0/en/model_doc/deformable_detr#transformers.DeformableDetrConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Deformable DETR Model (consisting of a backbone and encoder-decoder Transformer) outputting raw
hidden-states without any specific head on top.

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
  embedded representation.`DeformableDetrModelOutput` or `tuple(torch.FloatTensor)`A `DeformableDetrModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.
The [DeformableDetrModel](/docs/transformers/v5.14.0/en/model_doc/deformable_detr#transformers.DeformableDetrModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **init_reference_points** (`torch.FloatTensor` of shape  `(batch_size, num_queries, 4)`) -- Initial reference points sent through the Transformer decoder.
- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_queries, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the decoder of the model.
- **intermediate_hidden_states** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, hidden_size)`) -- Stacked intermediate hidden states (output of each layer of the decoder).
- **intermediate_reference_points** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, 4)`) -- Stacked intermediate reference points (reference points of each layer of the decoder).
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
- **enc_outputs_class** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_labels)`, *optional*, returned when `config.with_box_refine=True` and `config.two_stage=True`) -- Predicted bounding boxes scores where the top `config.two_stage_num_proposals` scoring bounding boxes are
  picked as region proposals in the first stage. Output of bounding box binary classification (i.e.
  foreground and background).
- **enc_outputs_coord_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, 4)`, *optional*, returned when `config.with_box_refine=True` and `config.two_stage=True`) -- Logits of predicted bounding boxes coordinates in the first stage.

Examples:

```python
>>> from transformers import AutoImageProcessor, DeformableDetrModel
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("SenseTime/deformable-detr")
>>> model = DeformableDetrModel.from_pretrained("SenseTime/deformable-detr")

>>> inputs = image_processor(images=image, return_tensors="pt")

>>> outputs = model(**inputs)

>>> last_hidden_states = outputs.last_hidden_state
>>> list(last_hidden_states.shape)
[1, 300, 256]
```

## DeformableDetrForObjectDetection[[transformers.DeformableDetrForObjectDetection]]

- **config** ([DeformableDetrConfig](/docs/transformers/v5.14.0/en/model_doc/deformable_detr#transformers.DeformableDetrConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Deformable DETR Model (consisting of a backbone and encoder-decoder Transformer) with object detection heads on
top, for tasks such as COCO detection.

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
  in the image,)` and the boxes a `torch.FloatTensor` of shape `(number of bounding boxes in the image, 4)`.</paramsdesc><rettype>`DeformableDetrObjectDetectionOutput` or `tuple(torch.FloatTensor)`</rettype><retdesc>A `DeformableDetrObjectDetectionOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.
The [DeformableDetrForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/deformable_detr#transformers.DeformableDetrForObjectDetection) forward method, overrides the `__call__` special method.

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
  possible padding). You can use `~DeformableDetrProcessor.post_process_object_detection` to retrieve the
  unnormalized bounding boxes.
- **auxiliary_outputs** (`list[Dict]`, *optional*) -- Optional, only returned when auxiliary losses are activated (i.e. `config.auxiliary_loss` is set to `True`)
  and labels are provided. It is a list of dictionaries containing the two above keys (`logits` and
  `pred_boxes`) for each decoder layer.
- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_queries, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the decoder of the model.
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
- **init_reference_points** (`torch.FloatTensor` of shape  `(batch_size, num_queries, 4)`) -- Initial reference points sent through the Transformer decoder.
- **intermediate_hidden_states** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, hidden_size)`) -- Stacked intermediate hidden states (output of each layer of the decoder).
- **intermediate_reference_points** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, 4)`) -- Stacked intermediate reference points (reference points of each layer of the decoder).
- **enc_outputs_class** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_labels)`, *optional*, returned when `config.with_box_refine=True` and `config.two_stage=True`) -- Predicted bounding boxes scores where the top `config.two_stage_num_proposals` scoring bounding boxes are
  picked as region proposals in the first stage. Output of bounding box binary classification (i.e.
  foreground and background).
- **enc_outputs_coord_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, 4)`, *optional*, returned when `config.with_box_refine=True` and `config.two_stage=True`) -- Logits of predicted bounding boxes coordinates in the first stage.

Examples:

```python
>>> from transformers import AutoImageProcessor, DeformableDetrForObjectDetection
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("SenseTime/deformable-detr")
>>> model = DeformableDetrForObjectDetection.from_pretrained("SenseTime/deformable-detr")

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
Detected cat with confidence 0.8 at location [16.5, 52.84, 318.25, 470.78]
Detected cat with confidence 0.789 at location [342.19, 24.3, 640.02, 372.25]
Detected remote with confidence 0.633 at location [40.79, 72.78, 176.76, 117.25]
```
