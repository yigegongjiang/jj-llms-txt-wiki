# RF-DETR

[RF-DETR](https://github.com/roboflow/rf-detr) proposes a Receptive Field Detection Transformer (DETR) architecture
designed to compete with and surpass the dominant YOLO series for real-time object detection. It achieves a new
state-of-the-art balance between speed (latency) and accuracy (mAP) by combining recent transformer advances with
efficient design choices.

The RF-DETR architecture is characterized by its simple and efficient structure: a DINOv2 Backbone, a Projector, and a
shallow DETR Decoder.
It enhances the DETR architecture for efficiency and speed using the following core modifications:

1. **DINOv2 Backbone**: Uses a powerful DINOv2 backbone for robust feature extraction.
2. **Group DETR Training**: Utilizes Group-Wise One-to-Many Assignment during training to accelerate convergence.
3. **Richer Input**: Aggregates multi-level features from the backbone and uses a C2f Projector (similarly to YOLOv8) to
   pass multi-scale features.
4. **Faster Decoder**: Employs a shallow 3-layer DETR decoder with deformable cross-attention for lower latency.
5. **Optimized Queries**: Uses a mixed-query scheme combining learnable content queries and generated spatial queries.

You can find all the available RF-DETR checkpoints under the [Roboflow](https://huggingface.co/Roboflow)
organization.
The original code can be found [here](https://github.com/roboflow/rf-detr).

Thanks to the weight conversion mapping, RfDetr is compatible with models from the original
[rf-detr](https://github.com/roboflow/rf-detr) library as well as models that you trained using the
[Roboflow](https://roboflow.com/) platform. This means you can use Roboflow platform to train your model and use
`RfDetr` in `transformers` to import the weights and deploy your model anywhere.

> [!TIP]
>
> Click on the RF-DETR models in the right sidebar for more examples of how to apply RF-DETR to different object
> detection tasks.

The example below demonstrates how to perform object detection with the [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) and the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline
import torch

pipeline = pipeline("object-detection", model="Roboflow/rf-detr-medium", device_map="auto")

pipeline("http://images.cocodataset.org/val2017/000000039769.jpg")
```

```python
from transformers import AutoImageProcessor, AutoModelForObjectDetection
from PIL import Image
import httpx
from io import BytesIO
import torch

url = "http://images.cocodataset.org/val2017/000000039769.jpg"
image = Image.open(BytesIO(httpx.get(url).content))

image_processor = AutoImageProcessor.from_pretrained("Roboflow/rf-detr-medium")
model = AutoModelForObjectDetection.from_pretrained("Roboflow/rf-detr-medium", device_map="auto")

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

### Visualizing results with supervision

You can use the [supervision](https://github.com/roboflow/supervision) library to visualize detection and segmentation results. Install it with `pip install supervision`.

```python
from transformers import AutoImageProcessor, AutoModelForObjectDetection
from PIL import Image
import supervision as sv

import httpx
from io import BytesIO
import torch

url = "http://images.cocodataset.org/val2017/000000039769.jpg"
image = Image.open(BytesIO(httpx.get(url).content))

image_processor = AutoImageProcessor.from_pretrained("Roboflow/rf-detr-medium")
model = AutoModelForObjectDetection.from_pretrained("Roboflow/rf-detr-medium", device_map="auto")

inputs = image_processor(images=image, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

results = image_processor.post_process_object_detection(
    outputs, target_sizes=torch.tensor([image.height, image.width]), threshold=0.3
)[0]

detections = sv.Detections.from_transformers(
    transformers_results=results, id2label=model.config.id2label
)

box_annotator = sv.BoxAnnotator()
label_annotator = sv.LabelAnnotator()

annotated_image = image.copy()
annotated_image = box_annotator.annotate(annotated_image, detections)
annotated_image = label_annotator.annotate(annotated_image, detections)

sv.plot_image(annotated_image)
```

```python
from transformers import AutoImageProcessor, AutoModelForInstanceSegmentation
from PIL import Image
import supervision as sv
import httpx
from io import BytesIO
import torch

url = "http://images.cocodataset.org/val2017/000000039769.jpg"
image = Image.open(BytesIO(httpx.get(url).content))

image_processor = AutoImageProcessor.from_pretrained("Roboflow/rf-detr-seg-medium")
model = AutoModelForInstanceSegmentation.from_pretrained("Roboflow/rf-detr-seg-medium", device_map="auto")

inputs = image_processor(images=image, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

results = image_processor.post_process_instance_segmentation(
    outputs, target_sizes=[image.size[::-1]], threshold=0.3
)[0]

detections = sv.Detections.from_transformers(
    transformers_results=results, id2label=model.config.id2label
)

mask_annotator = sv.MaskAnnotator()
label_annotator = sv.LabelAnnotator()

annotated_image = image.copy()
annotated_image = mask_annotator.annotate(annotated_image, detections)
annotated_image = label_annotator.annotate(annotated_image, detections)

sv.plot_image(annotated_image)
```

## Resources

- Scripts for finetuning [RfDetrForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/rf_detr#transformers.RfDetrForObjectDetection) with [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer)
  or [Accelerate](https://huggingface.co/docs/accelerate/index) can be
  found [here](https://github.com/huggingface/transformers/tree/main/examples/pytorch/object-detection).
- See also: [Object detection task guide](../tasks/object_detection).

## RfDetrConfig[[transformers.RfDetrConfig]]

- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **hidden_expansion** (`float`, *optional*, defaults to 0.5) --
  Expansion factor for hidden dimensions in the projector layers.
- **c2f_num_blocks** (`int`, *optional*, defaults to 3) --
  Number of blocks in the C2F layer.
- **activation_function** (`str`, *optional*, defaults to `"silu"`) --
  The non-linear activation function in the projector. Supported values are `"silu"`, `"relu"`, `"gelu"`.
- **dropout** (`float`, *optional*, defaults to `0.1`) --
  The ratio for all dropout layers.
- **decoder_ffn_dim** (`int`, *optional*, defaults to `2048`) --
  Dimensionality of the "intermediate" (often named feed-forward) layer in decoder.
- **decoder_n_points** (`int`, *optional*, defaults to 4) --
  The number of sampled keys in each feature level for each attention head in the decoder.
- **decoder_layers** (`int`, *optional*, defaults to 3) --
  Number of decoder layers in the transformer.
- **decoder_self_attention_heads** (`int`, *optional*, defaults to 8) --
  Number of attention heads for each attention layer in the decoder self-attention.
- **decoder_cross_attention_heads** (`int`, *optional*, defaults to 16) --
  Number of attention heads for each attention layer in the decoder cross-attention.
- **decoder_activation_function** (`str`, *optional*, defaults to `"relu"`) --
  The non-linear activation function in the decoder. Supported values are `"relu"`, `"silu"`, `"gelu"`.
- **num_queries** (`int`, *optional*, defaults to 300) --
  Number of object queries, i.e. detection slots. This is the maximal number of objects
  [RfDetrModel](/docs/transformers/v5.14.0/en/model_doc/rf_detr#transformers.RfDetrModel) can detect in a single image.
- **attention_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to use a bias in the query, key, value and output projection layers during self-attention.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **activation_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for activations inside the fully connected layer.
- **group_detr** (`int`, *optional*, defaults to 13) --
  Number of groups for Group DETR attention mechanism, which helps reduce computational complexity.
- **init_std** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **disable_custom_kernels** (`bool`, *optional*, defaults to `True`) --
  Disable the use of custom CUDA and CPU kernels. This option is necessary for the ONNX export, as custom
  kernels are not supported by PyTorch ONNX export.
- **class_cost** (`Union[int, float]`, *optional*, defaults to `2`) --
  Relative weight of the classification error in the Hungarian matching cost.
- **bbox_cost** (`Union[int, float]`, *optional*, defaults to `5`) --
  Relative weight of the L1 bounding box error in the Hungarian matching cost.
- **giou_cost** (`Union[int, float]`, *optional*, defaults to `2`) --
  Relative weight of the generalized IoU loss in the Hungarian matching cost.
- **class_loss_coefficient** (`float`, *optional*, defaults to 1) --
  Relative weight of the classification loss in the Hungarian matching cost.
- **dice_loss_coefficient** (`float`, *optional*, defaults to 1) --
  Relative weight of the DICE/F-1 loss in the object detection loss.
- **bbox_loss_coefficient** (`float`, *optional*, defaults to 5) --
  Relative weight of the L1 bounding box loss in the object detection loss.
- **giou_loss_coefficient** (`float`, *optional*, defaults to 2) --
  Relative weight of the generalized IoU loss in the object detection loss.
- **eos_coefficient** (`float`, *optional*, defaults to `0.1`) --
  Relative classification weight of the 'no-object' class in the object detection loss.
- **focal_alpha** (`float`, *optional*, defaults to `0.25`) --
  Alpha parameter in the focal loss.
- **auxiliary_loss** (`bool`, *optional*, defaults to `True`) --
  Whether auxiliary decoding losses (losses at each decoder layer) are to be used.
- **d_model** (`int`, *optional*, defaults to `256`) --
  Size of the encoder layers and the pooler layer.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the layer normalization layers.
- **num_feature_levels** (`int`, *optional*, defaults to 1) --
  Number of feature levels used in the multiscale deformable attention.
- **mask_loss_coefficient** (`float`, *optional*, defaults to 1) --
  Relative weight of the Focal loss in the instance segmentation mask loss.
- **mask_point_sample_ratio** (`int`, *optional*, defaults to 16) --
  The ratio of points to sample for the mask loss calculation.
- **mask_downsample_ratio** (`int`, *optional*, defaults to 4) --
  The downsample ratio for the segmentation masks compared to the input image resolution.
- **mask_class_loss_coefficient** (`float`, *optional*, defaults to 5.0) --
  Relative weight of the Focal loss in the instance segmentation loss.
- **mask_dice_loss_coefficient** (`float`, *optional*, defaults to 5.0) --
  Relative weight of the DICE/F-1 loss in the instance segmentation loss.
- **segmentation_head_activation_function** (`str`, *optional*, defaults to `"gelu"`) --
  The non-linear activation function in the segmentation head. Supported values are `"relu"`, `"silu"`, `"gelu"`.
- **intermediate_size** (`int`, *optional*, defaults to `1024`) --
  Dimension of the MLP representations.

This is the configuration class to store the configuration of a RfDetrModel. It is used to instantiate a Rf Detr
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Roboflow/rf-detr-base](https://huggingface.co/Roboflow/rf-detr-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import RfDetrConfig, RfDetrModel

>>> # Initializing a RF-DETR roboflow/rf-detr-base style configuration
>>> configuration = RfDetrConfig()

>>> # Initializing a model (with random weights) from the Roboflow/rf-detr-base style configuration
>>> model = RfDetrModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## RfDetrDinov2Config[[transformers.RfDetrDinov2Config]]

- **hidden_size** (`int`, *optional*, defaults to `768`) --
  Dimension of the hidden representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `12`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `12`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **mlp_ratio** (`int`, *optional*, defaults to `4`) --
  Ratio of the MLP hidden dim to the embedding dim.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **hidden_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **attention_probs_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `224`) --
  The size (resolution) of each image.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `14`) --
  The size (resolution) of each patch.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **qkv_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to add a bias to the queries, keys and values.
- **layerscale_value** (`float`, *optional*, defaults to 1.0) --
  Initial value to use for layer scale.
- **drop_path_rate** (`float`, *optional*, defaults to 0.0) --
  Stochastic depth rate per sample (when applied in the main path of residual layers).
- **use_swiglu_ffn** (`bool`, *optional*, defaults to `False`) --
  Whether to use the SwiGLU feedforward neural network.
- **apply_layernorm** (`bool`, *optional*, defaults to `True`) --
  Whether to apply layer normalization to the feature maps in case the model is used as backbone.
- **reshape_hidden_states** (`bool`, *optional*, defaults to `True`) --
  Whether to reshape the feature maps to 4D tensors of shape `(batch_size, d_model, height, width)` in
  case the model is used as backbone. If `False`, the feature maps will be 3D tensors of shape `(batch_size,
  seq_len, d_model)`.
- **use_mask_token** (`bool`, *optional*, defaults to `True`) --
  Whether to use mask_token in embeddings.
- **num_windows** (`int`, *optional*, defaults to 4) --
  Number of windows to use for windowed attention. If 1, no windowed attention is used.

This is the configuration class to store the configuration of a RfDetrModel. It is used to instantiate a Rf Detr
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Roboflow/rf-detr-base](https://huggingface.co/Roboflow/rf-detr-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import RfDetrDinov2Config, RfDetrDinov2Backbone

>>> # Initializing a RfDetrDinov2 base style configuration
>>> configuration = RfDetrDinov2Config()

>>> # Initializing a model (with random weights) from the base style configuration
>>> model = RfDetrDinov2Backbone(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## RfDetrImageProcessor[[transformers.RfDetrImageProcessor]]

- **format** (`str`, *kwargs*, *optional*, defaults to `AnnotationFormat.COCO_DETECTION`) --
  Data format of the annotations. One of "coco_detection" or "coco_panoptic".
- **do_convert_annotations** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Controls whether to convert the annotations to the format expected by the RF_DETR model. Converts the
  bounding boxes to the format `(center_x, center_y, width, height)` and in the range `[0, 1]`.
  Can be overridden by the `do_convert_annotations` parameter in the `preprocess` method.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a RfDetrImageProcessor image processor.

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
  Controls whether to convert the annotations to the format expected by the RF_DETR model. Converts the
  bounding boxes to the format `(center_x, center_y, width, height)` and in the range `[0, 1]`.
  Can be overridden by the `do_convert_annotations` parameter in the `preprocess` method.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** (`RfDetrObjectDetectionOutput`) --
  Raw outputs of the model.
- **threshold** (`float`, *optional*) --
  Score threshold to keep object detection predictions.
- **target_sizes** (`torch.Tensor` or `list[tuple[int, int]]`, *optional*) --
  Tensor of shape `(batch_size, 2)` or list of tuples (`tuple[int, int]`) containing the target size
  `(height, width)` of each image in the batch. If unset, predictions will not be resized.`list[Dict]`A list of dictionaries, each dictionary containing the scores, labels and boxes for an image
in the batch as predicted by the model.

Converts the raw output of [RfDetrForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/rf_detr#transformers.RfDetrForObjectDetection) into final bounding boxes in (top_left_x, top_left_y,
bottom_right_x, bottom_right_y) format. Only supports PyTorch.

- **outputs** ([*RfDetrInstanceSegmentationOutput*]) --
  Raw outputs of the model.
- **threshold** (*float*, *optional*, defaults to 0.5) --
  Score threshold to keep predicted instance masks.
- **mask_threshold** (*float*, *optional*, defaults to 0.0) --
  Threshold to binarize predicted masks.
- **target_sizes** (*list[tuple[int, int]]*, *optional*) --
  Target `(height, width)` for each image. If unset, masks are not resized.
- **return_coco_annotation** (*bool*, *optional*, defaults to *False*) --
  If *True*, return segmentation maps as COCO run-length encoding instead of tensors.
  Mutually exclusive with *return_binary_maps*.
- **return_binary_maps** (*bool*, *optional*, defaults to *False*) --
  If *True*, return segmentation maps as a stacked tensor of binary instance masks
  (one per detected instance), without overlap resolution. This matches the output
  format of the original `rfdetr` package. Mutually exclusive with
  *return_coco_annotation*.
- **top_k** (*int*, *optional*) --
  Maximum number of candidate queries evaluated before score thresholding.
  Defaults to the total number of queries.*list[dict]*One dict per image with keys:
- **segmentation** -- *Tensor[H, W]* of `int32` segment ids (`-1` = background),
  *Tensor[num_instances, H, W]* of bool binary masks when *return_binary_maps=True*,
  or a list of RLE encodings when *return_coco_annotation=True*.
- **segments_info** -- List of dicts with keys `id`, `label_id`, and `score`.

Converts the output of [*RfDetrForInstanceSegmentation*] into instance segmentation predictions.

## RfDetrModel[[transformers.RfDetrModel]]

- **config** ([RfDetrConfig](/docs/transformers/v5.14.0/en/model_doc/rf_detr#transformers.RfDetrConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare LW Detr Model (consisting of a backbone and decoder Transformer) outputting raw
hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [RfDetrImageProcessor](/docs/transformers/v5.14.0/en/model_doc/rf_detr#transformers.RfDetrImageProcessor). See `RfDetrImageProcessor.__call__()` for details (`processor_class` uses
  [RfDetrImageProcessor](/docs/transformers/v5.14.0/en/model_doc/rf_detr#transformers.RfDetrImageProcessor) for processing images).
- **pixel_mask** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:

  - 1 for pixels that are real (i.e. **not masked**),
  - 0 for pixels that are padding (i.e. **masked**).

  [What are attention masks?](../glossary#attention-mask)`RfDetrModelOutput` or `tuple(torch.FloatTensor)`A `RfDetrModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([RfDetrConfig](/docs/transformers/v5.14.0/en/model_doc/rf_detr#transformers.RfDetrConfig)) and inputs.

Forward pass of the RF-DETR model. The pipeline proceeds as follows:

1. Generate an initial set of object query embeddings and spatial location proposals from the
   backbone's flattened output.
2. Initialize storage for refined encoder-stage predictions (accommodating multi-group query
   structures) and iteratively refine object queries and their coordinates for each query group
   to capture the highest-confidence candidates from the encoder stage.
3. Initialize learnable query features and spatial reference points (restricting to the primary
   group during inference for efficiency).
4. Project the base reference points across the batch, refine them with the predicted coordinate
   refinements (shifting attention to the discovered object locations before decoding), and expand
   the target query features to match the batch dimensions.
5. Pass the refined queries and updated reference points through the transformer decoder to
   aggregate detailed spatial context from the multi-scale features.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the model.
- **init_reference_points** (`torch.FloatTensor` of shape  `(batch_size, num_queries, 4)`) -- Initial reference points sent through the Transformer decoder.
- **intermediate_hidden_states** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, d_model)`) -- Stacked intermediate hidden states (output of each layer of the decoder).
- **intermediate_reference_points** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, 4)`) -- Stacked intermediate reference points (reference points of each layer of the decoder).
- **enc_outputs_class** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_labels)`, *optional*, returned when `config.with_box_refine=True` and `config.two_stage=True`) -- Predicted bounding boxes scores where the top `config.two_stage_num_proposals` scoring bounding boxes are
  picked as region proposals in the first stage. Output of bounding box binary classification (i.e.
  foreground and background).
- **enc_outputs_coord_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, 4)`, *optional*, returned when `config.with_box_refine=True` and `config.two_stage=True`) -- Logits of predicted bounding boxes coordinates in the first stage.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **cross_attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **backbone_features** (`list` of `torch.FloatTensor` of shape `(batch_size, config.num_channels, config.image_size, config.image_size)`) -- Features from the backbone.

Examples:

```python
>>> from transformers import AutoImageProcessor, RfDetrModel
>>> from PIL import Image
>>> import requests

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> image = Image.open(requests.get(url, stream=True).raw)

>>> image_processor = AutoImageProcessor.from_pretrained("Roboflow/rf-detr-base")
>>> model = RfDetrModel.from_pretrained("Roboflow/rf-detr-base")

>>> inputs = image_processor(images=image, return_tensors="pt")

>>> outputs = model(**inputs)

>>> last_hidden_states = outputs.last_hidden_state
>>> list(last_hidden_states.shape)
[1, 200, 256]
```

## RfDetrForObjectDetection[[transformers.RfDetrForObjectDetection]]

- **config** ([RfDetrConfig](/docs/transformers/v5.14.0/en/model_doc/rf_detr#transformers.RfDetrConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

LW DETR Model (consisting of a backbone and decoder Transformer) with object detection heads on
top, for tasks such as COCO detection.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [RfDetrImageProcessor](/docs/transformers/v5.14.0/en/model_doc/rf_detr#transformers.RfDetrImageProcessor). See `RfDetrImageProcessor.__call__()` for details (`processor_class` uses
  [RfDetrImageProcessor](/docs/transformers/v5.14.0/en/model_doc/rf_detr#transformers.RfDetrImageProcessor) for processing images).
- **pixel_mask** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:

  - 1 for pixels that are real (i.e. **not masked**),
  - 0 for pixels that are padding (i.e. **masked**).

  [What are attention masks?](../glossary#attention-mask)
- **labels** (`list[Dict]` of len `(batch_size,)`, *optional*) --
  Labels for computing the bipartite matching loss. List of dicts, each dictionary containing at least the
  following 2 keys: 'class_labels' and 'boxes' (the class labels and bounding boxes of an image in the batch
  respectively). The class labels themselves should be a `torch.LongTensor` of len `(number of bounding boxes
  in the image,)` and the boxes a `torch.FloatTensor` of shape `(number of bounding boxes in the image, 4)`.</paramsdesc><rettype>`RfDetrObjectDetectionOutput` or `tuple(torch.FloatTensor)`</rettype><retdesc>A `RfDetrObjectDetectionOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([RfDetrConfig](/docs/transformers/v5.14.0/en/model_doc/rf_detr#transformers.RfDetrConfig)) and inputs.

The forward pass proceeds as follows:

1. Process the visual input through the base RF-DETR model to obtain the transformer's last hidden state and
   the final sequence of reference points.
2. First stage: Generate classification logits from the encoder's proposed object query embeddings.
3. Second stage: Predict the final classification labels and refined bounding boxes using the decoder's last hidden state
   and the most recent reference points.

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
- **init_reference_points** (`torch.FloatTensor` of shape  `(batch_size, num_queries, 4)`) -- Initial reference points sent through the Transformer decoder.
- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the model.
- **intermediate_hidden_states** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, d_model)`) -- Stacked intermediate hidden states (output of each layer of the decoder).
- **intermediate_reference_points** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, 4)`) -- Stacked intermediate reference points (reference points of each layer of the decoder).
- **enc_outputs_class** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_labels)`, *optional*, returned when `config.with_box_refine=True` and `config.two_stage=True`) -- Predicted bounding boxes scores where the top `config.two_stage_num_proposals` scoring bounding boxes are
  picked as region proposals in the first stage. Output of bounding box binary classification (i.e.
  foreground and background).
- **enc_outputs_coord_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, 4)`, *optional*, returned when `config.with_box_refine=True` and `config.two_stage=True`) -- Logits of predicted bounding boxes coordinates in the first stage.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **cross_attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **backbone_features** (`list` of `torch.FloatTensor` of shape `(batch_size, config.num_channels, config.image_size, config.image_size)`) -- Features from the backbone.

Examples:

```python
>>> from transformers import AutoImageProcessor, RfDetrForObjectDetection
>>> from PIL import Image
>>> import requests

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> image = Image.open(requests.get(url, stream=True).raw)

>>> image_processor = AutoImageProcessor.from_pretrained("Roboflow/rf-detr-base")
>>> model = RfDetrForObjectDetection.from_pretrained("Roboflow/rf-detr-base")

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

## RfDetrForInstanceSegmentation[[transformers.RfDetrForInstanceSegmentation]]

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [RfDetrImageProcessor](/docs/transformers/v5.14.0/en/model_doc/rf_detr#transformers.RfDetrImageProcessor). See `RfDetrImageProcessor.__call__()` for details (`processor_class` uses
  [RfDetrImageProcessor](/docs/transformers/v5.14.0/en/model_doc/rf_detr#transformers.RfDetrImageProcessor) for processing images).
- **pixel_mask** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:

  - 1 for pixels that are real (i.e. **not masked**),
  - 0 for pixels that are padding (i.e. **masked**).

  [What are attention masks?](../glossary#attention-mask)
- **labels** (`list[Dict]` of len `(batch_size,)`, *optional*) --
  Labels for computing the bipartite matching loss. List of dicts, each dictionary containing at least the
  following 2 keys: 'class_labels' and 'boxes' (the class labels and bounding boxes of an image in the batch
  respectively). The class labels themselves should be a `torch.LongTensor` of len `(number of bounding boxes
  in the image,)` and the boxes a `torch.FloatTensor` of shape `(number of bounding boxes in the image, 4)`.</paramsdesc><retdesc>`dict[str, doc_builder.mock_imports.torch.Tensor]`

Forward pass of the RF-DETR model for instance segmentation. The pipeline proceeds as follows:

1. Process the visual input through the base RF-DETR model to obtain multi-scale spatial features,
   query embeddings, and their transformation history.
2. Generate classification logits and initial segmentation masks from the encoder's proposed
   object query embeddings (first stage).
3. Predict the final classification labels and refined bounding boxes using the decoder's last
   hidden state (second stage).
4. Pass the high-resolution spatial features and query hidden states through the segmentation
   head to produce the final, detailed instance masks.

## RfDetrDinov2Backbone[[transformers.RfDetrDinov2Backbone]]

- **config** ([RfDetrDinov2Backbone](/docs/transformers/v5.14.0/en/model_doc/rf_detr#transformers.RfDetrDinov2Backbone)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

RfDetrDinov2 backbone, to be used with frameworks like DETR and MaskFormer.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [RfDetrImageProcessor](/docs/transformers/v5.14.0/en/model_doc/rf_detr#transformers.RfDetrImageProcessor). See `RfDetrImageProcessor.__call__()` for details (`processor_class` uses
  [RfDetrImageProcessor](/docs/transformers/v5.14.0/en/model_doc/rf_detr#transformers.RfDetrImageProcessor) for processing images).`BackboneOutput` or `tuple(torch.FloatTensor)`A `BackboneOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([RfDetrConfig](/docs/transformers/v5.14.0/en/model_doc/rf_detr#transformers.RfDetrConfig)) and inputs.
The [RfDetrDinov2Backbone](/docs/transformers/v5.14.0/en/model_doc/rf_detr#transformers.RfDetrDinov2Backbone) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **feature_maps** (`tuple(torch.FloatTensor)` of shape `(batch_size, num_channels, height, width)`) -- Feature maps of the stages.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer) of
  shape `(batch_size, sequence_length, hidden_size)` or `(batch_size, num_channels, height, width)`,
  depending on the backbone.

  Hidden-states of the model at the output of each stage plus the initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`. Only applicable if the backbone uses attention.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import AutoImageProcessor, AutoBackbone
>>> import torch
>>> from PIL import Image
>>> import requests

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> image = Image.open(requests.get(url, stream=True).raw)

>>> processor = AutoImageProcessor.from_pretrained("facebook/dinov2-base")
>>> model = AutoBackbone.from_pretrained(
...     "facebook/dinov2-base", out_features=["stage2", "stage5", "stage8", "stage11"]
... )

>>> inputs = processor(image, return_tensors="pt")

>>> outputs = model(**inputs)
>>> feature_maps = outputs.feature_maps
>>> list(feature_maps[-1].shape)
[1, 768, 16, 16]
```
