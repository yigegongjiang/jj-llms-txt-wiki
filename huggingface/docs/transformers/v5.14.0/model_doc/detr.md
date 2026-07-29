# DETR

[DETR](https://huggingface.co/papers/2005.12872) consists of a convolutional backbone followed by an encoder-decoder Transformer which can be trained end-to-end for object detection. It greatly simplifies a lot of the complexity of models like Faster-R-CNN and Mask-R-CNN, which use things like region proposals, non-maximum suppression procedure and anchor generation. Moreover, DETR can also be naturally extended to perform panoptic segmentation, by simply adding a mask head on top of the decoder outputs.

You can find all the original DETR checkpoints under the [AI at Meta](https://huggingface.co/facebook/models?search=detr) organization.

> [!TIP]
> This model was contributed by [nielsr](https://huggingface.co/nielsr).
>
> Click on the DETR models in the right sidebar for more examples of how to apply DETR to different object detection and segmentation tasks.

The example below demonstrates how to perform object detection with the [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) class.

```python

from transformers import pipeline

pipeline = pipeline(
    "object-detection",
    model="facebook/detr-resnet-50",
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

image_processor = AutoImageProcessor.from_pretrained("facebook/detr-resnet-50")
model = AutoModelForObjectDetection.from_pretrained("facebook/detr-resnet-50", device_map="auto")

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

How DETR works

Here's a TLDR explaining how [DetrForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForObjectDetection) works:

First, an image is sent through a pre-trained convolutional backbone (in the paper, the authors use ResNet-50/ResNet-101). Let's assume we also add a batch dimension. This means that the input to the backbone is a tensor of shape `(batch_size, 3, height, width)`, assuming the image has 3 color channels (RGB). The CNN backbone outputs a new lower-resolution feature map, typically of shape `(batch_size, 2048, height/32, width/32)`. This is then projected to match the hidden dimension of the Transformer of DETR, which is `256` by default, using a `nn.Conv2D` layer. So now, we have a tensor of shape `(batch_size, 256, height/32, width/32).` Next, the feature map is flattened and transposed to obtain a tensor of shape `(batch_size, seq_len, d_model)` = `(batch_size, width/32*height/32, 256)`. So a difference with NLP models is that the sequence length is actually longer than usual, but with a smaller `d_model` (which in NLP is typically 768 or higher).

Next, this is sent through the encoder, outputting `encoder_hidden_states` of the same shape (you can consider these as image features). Next, so-called **object queries** are sent through the decoder. This is a tensor of shape `(batch_size, num_queries, d_model)`, with `num_queries` typically set to 100 and initialized with zeros. These input embeddings are learnt positional encodings that the authors refer to as object queries, and similarly to the encoder, they are added to the input of each attention layer. Each object query will look for a particular object in the image. The decoder updates these embeddings through multiple self-attention and encoder-decoder attention layers to output `decoder_hidden_states` of the same shape: `(batch_size, num_queries, d_model)`. Next, two heads are added on top for object detection: a linear layer for classifying each object query into one of the objects or "no object", and a MLP to predict bounding boxes for each query.

The model is trained using a **bipartite matching loss**: so what we actually do is compare the predicted classes + bounding boxes of each of the N = 100 object queries to the ground truth annotations, padded up to the same length N (so if an image only contains 4 objects, 96 annotations will just have a "no object" as class and "no bounding box" as bounding box). The [Hungarian matching algorithm](https://en.wikipedia.org/wiki/Hungarian_algorithm) is used to find an optimal one-to-one mapping of each of the N queries to each of the N annotations. Next, standard cross-entropy (for the classes) and a linear combination of the L1 and [generalized IoU loss](https://giou.stanford.edu/) (for the bounding boxes) are used to optimize the parameters of the model.

DETR can be naturally extended to perform panoptic segmentation (which unifies semantic segmentation and instance segmentation). [DetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForSegmentation) adds a segmentation mask head on top of [DetrForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForObjectDetection). The mask head can be trained either jointly, or in a two steps process, where one first trains a [DetrForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForObjectDetection) model to detect bounding boxes around both "things" (instances) and "stuff" (background things like trees, roads, sky), then freeze all the weights and train only the mask head for 25 epochs. Experimentally, these two approaches give similar results. Note that predicting boxes is required for the training to be possible, since the Hungarian matching is computed using distances between boxes.

## Notes

- DETR uses so-called **object queries** to detect objects in an image. The number of queries determines the maximum number of objects that can be detected in a single image, and is set to 100 by default (see parameter `num_queries` of [DetrConfig](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrConfig)). Note that it's good to have some slack (in COCO, the authors used 100, while the maximum number of objects in a COCO image is ~70).
- The decoder of DETR updates the query embeddings in parallel. This is different from language models like GPT-2, which use autoregressive decoding instead of parallel. Hence, no causal attention mask is used.
- DETR adds position embeddings to the hidden states at each self-attention and cross-attention layer before projecting to queries and keys. For the position embeddings of the image, one can choose between fixed sinusoidal or learned absolute position embeddings. By default, the parameter `position_embedding_type` of [DetrConfig](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrConfig) is set to `"sine"`.
- During training, the authors of DETR did find it helpful to use auxiliary losses in the decoder, especially to help the model output the correct number of objects of each class. If you set the parameter `auxiliary_loss` of [DetrConfig](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrConfig) to `True`, then prediction feedforward neural networks and Hungarian losses are added after each decoder layer (with the FFNs sharing parameters).
- If you want to train the model in a distributed environment across multiple nodes, then one should update the *num_boxes* variable in the *DetrLoss* class of *modeling_detr.py*. When training on multiple nodes, this should be set to the average number of target boxes across all nodes, as can be seen in the original implementation [here](https://github.com/facebookresearch/detr/blob/a54b77800eb8e64e3ad0d8237789fcbf2f8350c5/models/detr.py#L227-L232).
- [DetrForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForObjectDetection) and [DetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForSegmentation) can be initialized with any convolutional backbone available in the [timm library](https://github.com/rwightman/pytorch-image-models). Initializing with a MobileNet backbone for example can be done by setting the `backbone` attribute of [DetrConfig](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrConfig) to `"tf_mobilenetv3_small_075"`, and then initializing the model with that config.
- DETR resizes the input images such that the shortest side is at least a certain amount of pixels while the longest is at most 1333 pixels. At training time, scale augmentation is used such that the shortest side is randomly set to at least 480 and at most 800 pixels. At inference time, the shortest side is set to 800. One can use [DetrImageProcessor](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrImageProcessor) to prepare images (and optional annotations in COCO format) for the model. Due to this resizing, images in a batch can have different sizes. DETR solves this by padding images up to the largest size in a batch, and by creating a pixel mask that indicates which pixels are real/which are padding. Alternatively, one can also define a custom `collate_fn` in order to batch images together, using `~transformers.DetrImageProcessor.pad_and_create_pixel_mask`.
- The size of the images will determine the amount of memory being used, and will thus determine the `batch_size`. It is advised to use a batch size of 2 per GPU. See [this Github thread](https://github.com/facebookresearch/detr/issues/150) for more info.

There are three other ways to instantiate a DETR model (depending on what you prefer):

- Option 1: Instantiate DETR with pre-trained weights for entire model

```python
from transformers import DetrForObjectDetection

model = DetrForObjectDetection.from_pretrained("facebook/detr-resnet-50", device_map="auto")
```

- Option 2: Instantiate DETR with randomly initialized weights for Transformer, but pre-trained weights for backbone

```python
from transformers import DetrConfig, DetrForObjectDetection

config = DetrConfig()
model = DetrForObjectDetection(config)
```

- Option 3: Instantiate DETR with randomly initialized weights for backbone + Transformer

```python
config = DetrConfig()
model = DetrForObjectDetection(config)
```

As a summary, consider the following table:

| Task | Object detection | Instance segmentation | Panoptic segmentation |
|------|------------------|-----------------------|-----------------------|
| **Description** | Predicting bounding boxes and class labels around objects in an image | Predicting masks around objects (i.e. instances) in an image | Predicting masks around both objects (i.e. instances) as well as "stuff" (i.e. background things like trees and roads) in an image |
| **Model** | [DetrForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForObjectDetection) | [DetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForSegmentation) | [DetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForSegmentation) |
| **Example dataset** | COCO detection | COCO detection, COCO panoptic | COCO panoptic                           |
| **Format of annotations to provide to**  [DetrImageProcessor](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrImageProcessor) | {'image_id': `int`, 'annotations': `list[Dict]`} each Dict being a COCO object annotation  | {'image_id': `int`, 'annotations': `list[Dict]`}  (in case of COCO detection) or {'file_name': `str`, 'image_id': `int`, 'segments_info': `list[Dict]`} (in case of COCO panoptic) | {'file_name': `str`, 'image_id': `int`, 'segments_info': `list[Dict]`} and masks_path (path to directory containing PNG files of the masks) |
| **Postprocessing** (i.e. converting the output of the model to Pascal VOC format) | `~transformers.DetrImageProcessor.post_process` | `~transformers.DetrImageProcessor.post_process_segmentation` | `~transformers.DetrImageProcessor.post_process_segmentation`, `~transformers.DetrImageProcessor.post_process_panoptic` |
| **evaluators** | `CocoEvaluator` with `iou_types="bbox"` | `CocoEvaluator` with `iou_types="bbox"` or `"segm"` | `CocoEvaluator` with `iou_types="bbox"` or `"segm"`, `PanopticEvaluator` |

- In short, one should prepare the data either in COCO detection or COCO panoptic format, then use [DetrImageProcessor](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrImageProcessor) to create `pixel_values`, `pixel_mask` and optional `labels`, which can then be used to train (or fine-tune) a model.
- For evaluation, one should first convert the outputs of the model using one of the postprocessing methods of [DetrImageProcessor](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrImageProcessor). These can be provided to either `CocoEvaluator` or `PanopticEvaluator`, which allow you to calculate metrics like mean Average Precision (mAP) and Panoptic Quality (PQ). The latter objects are implemented in the [original repository](https://github.com/facebookresearch/detr). See the [example notebooks](https://github.com/NielsRogge/Transformers-Tutorials/tree/master/DETR) for more info regarding evaluation.

## Resources

- Refer to these [notebooks](https://github.com/NielsRogge/Transformers-Tutorials/tree/master/DETR) for examples of fine-tuning [DetrForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForObjectDetection) and [DetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForSegmentation) on a custom dataset.

## DetrConfig[[transformers.DetrConfig]]

- **is_encoder_decoder** (`bool`, *optional*, defaults to `True`) --
  Whether the model is used as an encoder/decoder or not.
- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **num_queries** (`int`, *optional*, defaults to 100) --
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
  Whether auxiliary decoding losses (losses at each decoder layer) are to be used.
- **position_embedding_type** (`str`, *optional*, defaults to `"sine"`) --
  Type of position embeddings to be used on top of the image features. One of `"sine"` or `"learned"`.
- **dilation** (`bool`, *optional*, defaults to `False`) --
  Whether to replace stride with dilation in the last convolutional block (DC5). Only supported when
  `use_timm_backbone` = `True`.
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

This is the configuration class to store the configuration of a DetrModel. It is used to instantiate a Detr
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/detr-resnet-50](https://huggingface.co/facebook/detr-resnet-50)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import DetrConfig, DetrModel

>>> # Initializing a DETR facebook/detr-resnet-50 style configuration
>>> configuration = DetrConfig()

>>> # Initializing a model (with random weights) from the facebook/detr-resnet-50 style configuration
>>> model = DetrModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## DetrImageProcessor[[transformers.DetrImageProcessor]]

- **format** (`str`, *kwargs*, *optional*, defaults to `AnnotationFormat.COCO_DETECTION`) --
  Data format of the annotations. One of "coco_detection" or "coco_panoptic".
- **do_convert_annotations** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Controls whether to convert the annotations to the format expected by the DETR model. Converts the
  bounding boxes to the format `(center_x, center_y, width, height)` and in the range `[0, 1]`.
  Can be overridden by the `do_convert_annotations` parameter in the `preprocess` method.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a DetrImageProcessor image processor.

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
  Controls whether to convert the annotations to the format expected by the DETR model. Converts the
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
  `(height, width)` of each image in the batch. If unset, predictions will not be resized.`list[Dict]`A list of dictionaries, each dictionary containing the scores, labels and boxes for an image
in the batch as predicted by the model.

Converts the raw output of [DetrForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForObjectDetection) into final bounding boxes in (top_left_x, top_left_y,
bottom_right_x, bottom_right_y) format. Only supports PyTorch.

- **outputs** ([DetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForSegmentation)) --
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

Converts the output of [DetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForSegmentation) into semantic segmentation maps. Only supports PyTorch.

- **outputs** ([DetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForSegmentation)) --
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

Converts the output of [DetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForSegmentation) into instance segmentation predictions. Only supports PyTorch.

- **outputs** ([DetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForSegmentation)) --
  The outputs from [DetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForSegmentation).
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

Converts the output of [DetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForSegmentation) into image panoptic segmentation predictions. Only supports
PyTorch.

## DetrImageProcessorPil[[transformers.DetrImageProcessorPil]]

- **format** (`str`, *kwargs*, *optional*, defaults to `AnnotationFormat.COCO_DETECTION`) --
  Data format of the annotations. One of "coco_detection" or "coco_panoptic".
- **do_convert_annotations** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Controls whether to convert the annotations to the format expected by the DETR model. Converts the
  bounding boxes to the format `(center_x, center_y, width, height)` and in the range `[0, 1]`.
  Can be overridden by the `do_convert_annotations` parameter in the `preprocess` method.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a DetrImageProcessor image processor.

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
  Controls whether to convert the annotations to the format expected by the DETR model. Converts the
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
  `(height, width)` of each image in the batch. If unset, predictions will not be resized.`list[Dict]`A list of dictionaries, each dictionary containing the scores, labels and boxes for an image
in the batch as predicted by the model.

Converts the raw output of [DetrForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForObjectDetection) into final bounding boxes in (top_left_x, top_left_y,
bottom_right_x, bottom_right_y) format. Only supports PyTorch.

- **outputs** ([DetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForSegmentation)) --
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

Converts the output of [DetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForSegmentation) into semantic segmentation maps. Only supports PyTorch.

- **outputs** ([DetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForSegmentation)) --
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

Converts the output of [DetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForSegmentation) into instance segmentation predictions. Only supports PyTorch.

- **outputs** ([DetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForSegmentation)) --
  The outputs from [DetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForSegmentation).
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

Converts the output of [DetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForSegmentation) into image panoptic segmentation predictions. Only supports
PyTorch.

## DETR specific outputs[[transformers.models.detr.modeling_detr.DetrModelOutput]]

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) --
  Sequence of hidden-states at the output of the last layer of the decoder of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) --
  It is a [EncoderDecoderCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.
- **decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) --
  Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the decoder at the output of each layer plus the optional initial embedding outputs.
- **decoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) --
  Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **cross_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) --
  Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) --
  Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the encoder at the output of each layer plus the optional initial embedding outputs.
- **encoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) --
  Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **intermediate_hidden_states** (`torch.FloatTensor` of shape `(config.decoder_layers, batch_size, sequence_length, hidden_size)`, *optional*, returned when `config.auxiliary_loss=True`) --
  Intermediate decoder activations, i.e. the output of each decoder layer, each of them gone through a
  layernorm.

Base class for outputs of the DETR encoder-decoder model. This class adds one attribute to Seq2SeqModelOutput,
namely an optional stack of intermediate decoder activations, i.e. the output of each decoder layer, each of them
gone through a layernorm. This is useful when training the model with auxiliary decoding losses.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` are provided)) --
  Total loss as a linear combination of a negative log-likelihood (cross-entropy) for class prediction and a
  bounding box loss. The latter is defined as a linear combination of the L1 loss and the generalized
  scale-invariant IoU loss.
- **loss_dict** (`Dict`, *optional*) --
  A dictionary containing the individual losses. Useful for logging.
- **logits** (`torch.FloatTensor` of shape `(batch_size, num_queries, num_classes + 1)`) --
  Classification logits (including no-object) for all queries.
- **pred_boxes** (`torch.FloatTensor` of shape `(batch_size, num_queries, 4)`) --
  Normalized boxes coordinates for all queries, represented as (center_x, center_y, width, height). These
  values are normalized in [0, 1], relative to the size of each individual image in the batch (disregarding
  possible padding). You can use [post_process_object_detection()](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrImageProcessor.post_process_object_detection) to retrieve the
  unnormalized bounding boxes.
- **auxiliary_outputs** (`list[Dict]`, *optional*) --
  Optional, only returned when auxiliary losses are activated (i.e. `config.auxiliary_loss` is set to `True`)
  and labels are provided. It is a list of dictionaries containing the two above keys (`logits` and
  `pred_boxes`) for each decoder layer.
- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Sequence of hidden-states at the output of the last layer of the decoder of the model.
- **decoder_hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) --
  Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the decoder at the output of each layer plus the initial embedding outputs.
- **decoder_attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) --
  Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **cross_attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) --
  Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) --
  Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the encoder at the output of each layer plus the initial embedding outputs.
- **encoder_attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) --
  Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.

Output type of [DetrForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForObjectDetection).

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` are provided)) --
  Total loss as a linear combination of a negative log-likelihood (cross-entropy) for class prediction and a
  bounding box loss. The latter is defined as a linear combination of the L1 loss and the generalized
  scale-invariant IoU loss.
- **loss_dict** (`Dict`, *optional*) --
  A dictionary containing the individual losses. Useful for logging.
- **logits** (`torch.FloatTensor` of shape `(batch_size, num_queries, num_classes + 1)`) --
  Classification logits (including no-object) for all queries.
- **pred_boxes** (`torch.FloatTensor` of shape `(batch_size, num_queries, 4)`) --
  Normalized boxes coordinates for all queries, represented as (center_x, center_y, width, height). These
  values are normalized in [0, 1], relative to the size of each individual image in the batch (disregarding
  possible padding). You can use [post_process_object_detection()](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrImageProcessor.post_process_object_detection) to retrieve the
  unnormalized bounding boxes.
- **pred_masks** (`torch.FloatTensor` of shape `(batch_size, num_queries, height/4, width/4)`) --
  Segmentation masks logits for all queries. See also
  [post_process_semantic_segmentation()](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrImageProcessor.post_process_semantic_segmentation) or
  [post_process_instance_segmentation()](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrImageProcessor.post_process_instance_segmentation)
  [post_process_panoptic_segmentation()](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrImageProcessor.post_process_panoptic_segmentation) to evaluate semantic, instance and panoptic
  segmentation masks respectively.
- **auxiliary_outputs** (`list[Dict]`, *optional*) --
  Optional, only returned when auxiliary losses are activated (i.e. `config.auxiliary_loss` is set to `True`)
  and labels are provided. It is a list of dictionaries containing the two above keys (`logits` and
  `pred_boxes`) for each decoder layer.
- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Sequence of hidden-states at the output of the last layer of the decoder of the model.
- **decoder_hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) --
  Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the decoder at the output of each layer plus the initial embedding outputs.
- **decoder_attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) --
  Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **cross_attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) --
  Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) --
  Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the encoder at the output of each layer plus the initial embedding outputs.
- **encoder_attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) --
  Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.

Output type of [DetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForSegmentation).

## DetrModel[[transformers.DetrModel]]

- **config** ([DetrConfig](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare DETR Model (consisting of a backbone and encoder-decoder Transformer) outputting raw hidden-states without
any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
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
  Optionally, instead of passing the flattened feature map (output of the backbone + projection layer), you
  can choose to directly pass a flattened representation of an image. Useful for bypassing the vision backbone.
- **decoder_inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, num_queries, hidden_size)`, *optional*) --
  Optionally, instead of initializing the queries with a tensor of zeros, you can choose to directly pass an
  embedded representation. Useful for tasks that require custom query initialization.[DetrModelOutput](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.models.detr.modeling_detr.DetrModelOutput) or `tuple(torch.FloatTensor)`A [DetrModelOutput](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.models.detr.modeling_detr.DetrModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.
The [DetrModel](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrModel) forward method, overrides the `__call__` special method.

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

Examples:

```python
>>> from transformers import AutoImageProcessor, DetrModel
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("facebook/detr-resnet-50")
>>> model = DetrModel.from_pretrained("facebook/detr-resnet-50")

>>> # prepare image for the model
>>> inputs = image_processor(images=image, return_tensors="pt")

>>> # forward pass
>>> outputs = model(**inputs)

>>> # the last hidden states are the final query embeddings of the Transformer decoder
>>> # these are of shape (batch_size, num_queries, hidden_size)
>>> last_hidden_states = outputs.last_hidden_state
>>> list(last_hidden_states.shape)
[1, 100, 256]
```

## DetrForObjectDetection[[transformers.DetrForObjectDetection]]

- **config** ([DetrConfig](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

DETR Model (consisting of a backbone and encoder-decoder Transformer) with object detection heads on top, for tasks
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
  Mask to avoid performing attention on certain object queries in the decoder. Mask values selected in `[0, 1]`:

  - 1 for queries that are **not masked**,
  - 0 for queries that are **masked**.
- **encoder_outputs** (`torch.FloatTensor`, *optional*) --
  Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`)
  `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of
  hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing the flattened feature map (output of the backbone + projection layer), you
  can choose to directly pass a flattened representation of an image. Useful for bypassing the vision backbone.
- **decoder_inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, num_queries, hidden_size)`, *optional*) --
  Optionally, instead of initializing the queries with a tensor of zeros, you can choose to directly pass an
  embedded representation. Useful for tasks that require custom query initialization.
- **labels** (`list[Dict]` of len `(batch_size,)`, *optional*) --
  Labels for computing the bipartite matching loss. List of dicts, each dictionary containing at least the
  following 2 keys: 'class_labels' and 'boxes' (the class labels and bounding boxes of an image in the batch
  respectively). The class labels themselves should be a `torch.LongTensor` of len `(number of bounding boxes
  in the image,)` and the boxes a `torch.FloatTensor` of shape `(number of bounding boxes in the image, 4)`.</paramsdesc><rettype>[DetrObjectDetectionOutput](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.models.detr.modeling_detr.DetrObjectDetectionOutput) or `tuple(torch.FloatTensor)`A [DetrObjectDetectionOutput](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.models.detr.modeling_detr.DetrObjectDetectionOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.
The [DetrForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForObjectDetection) forward method, overrides the `__call__` special method.

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
  possible padding). You can use [post_process_object_detection()](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrImageProcessor.post_process_object_detection) to retrieve the
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
>>> from transformers import AutoImageProcessor, DetrForObjectDetection
>>> import torch
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("facebook/detr-resnet-50")
>>> model = DetrForObjectDetection.from_pretrained("facebook/detr-resnet-50")

>>> inputs = image_processor(images=image, return_tensors="pt")
>>> outputs = model(**inputs)

>>> # convert outputs (bounding boxes and class logits) to Pascal VOC format (xmin, ymin, xmax, ymax)
>>> target_sizes = torch.tensor([image.size[::-1]])
>>> results = image_processor.post_process_object_detection(outputs, threshold=0.9, target_sizes=target_sizes)[
...     0
... ]

>>> for score, label, box in zip(results["scores"], results["labels"], results["boxes"]):
...     box = [round(i, 2) for i in box.tolist()]
...     print(
...         f"Detected {model.config.id2label[label.item()]} with confidence "
...         f"{round(score.item(), 3)} at location {box}"
...     )
Detected remote with confidence 0.998 at location [40.16, 70.81, 175.55, 117.98]
Detected remote with confidence 0.996 at location [333.24, 72.55, 368.33, 187.66]
Detected couch with confidence 0.995 at location [-0.02, 1.15, 639.73, 473.76]
Detected cat with confidence 0.999 at location [13.24, 52.05, 314.02, 470.93]
Detected cat with confidence 0.999 at location [345.4, 23.85, 640.37, 368.72]
```

## DetrForSegmentation[[transformers.DetrForSegmentation]]

- **config** ([DetrConfig](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

DETR Model (consisting of a backbone and encoder-decoder Transformer) with a segmentation head on top, for tasks
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
  `torch.FloatTensor` of shape `(number of bounding boxes in the image, height, width)`.[DetrSegmentationOutput](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.models.detr.modeling_detr.DetrSegmentationOutput) or `tuple(torch.FloatTensor)`A [DetrSegmentationOutput](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.models.detr.modeling_detr.DetrSegmentationOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.
The [DetrForSegmentation](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrForSegmentation) forward method, overrides the `__call__` special method.

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
  possible padding). You can use [post_process_object_detection()](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrImageProcessor.post_process_object_detection) to retrieve the
  unnormalized bounding boxes.
- **pred_masks** (`torch.FloatTensor` of shape `(batch_size, num_queries, height/4, width/4)`) -- Segmentation masks logits for all queries. See also
  [post_process_semantic_segmentation()](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrImageProcessor.post_process_semantic_segmentation) or
  [post_process_instance_segmentation()](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrImageProcessor.post_process_instance_segmentation)
  [post_process_panoptic_segmentation()](/docs/transformers/v5.14.0/en/model_doc/detr#transformers.DetrImageProcessor.post_process_panoptic_segmentation) to evaluate semantic, instance and panoptic
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

>>> from transformers import AutoImageProcessor, DetrForSegmentation
>>> from transformers.image_transforms import rgb_to_id

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("facebook/detr-resnet-50-panoptic")
>>> model = DetrForSegmentation.from_pretrained("facebook/detr-resnet-50-panoptic")

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
