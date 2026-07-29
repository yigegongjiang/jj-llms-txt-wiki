# Grounding DINO

## Overview

The Grounding DINO model was proposed in [Grounding DINO: Marrying DINO with Grounded Pre-Training for Open-Set Object Detection](https://huggingface.co/papers/2303.05499) by Shilong Liu, Zhaoyang Zeng, Tianhe Ren, Feng Li, Hao Zhang, Jie Yang, Chunyuan Li, Jianwei Yang, Hang Su, Jun Zhu, Lei Zhang. Grounding DINO extends a closed-set object detection model with a text encoder, enabling open-set object detection. The model achieves remarkable results, such as 52.5 AP on COCO zero-shot.

The abstract from the paper is the following:

*In this paper, we present an open-set object detector, called Grounding DINO, by marrying Transformer-based detector DINO with grounded pre-training, which can detect arbitrary objects with human inputs such as category names or referring expressions. The key solution of open-set object detection is introducing language to a closed-set detector for open-set concept generalization. To effectively fuse language and vision modalities, we conceptually divide a closed-set detector into three phases and propose a tight fusion solution, which includes a feature enhancer, a language-guided query selection, and a cross-modality decoder for cross-modality fusion. While previous works mainly evaluate open-set object detection on novel categories, we propose to also perform evaluations on referring expression comprehension for objects specified with attributes. Grounding DINO performs remarkably well on all three settings, including benchmarks on COCO, LVIS, ODinW, and RefCOCO/+/g. Grounding DINO achieves a 52.5 AP on the COCO detection zero-shot transfer benchmark, i.e., without any training data from COCO. It sets a new record on the ODinW zero-shot benchmark with a mean 26.1 AP.*

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/grouding_dino_architecture.png"
alt="drawing" width="600"/>

 Grounding DINO overview. Taken from the original paper. 

This model was contributed by [EduardoPacheco](https://huggingface.co/EduardoPacheco) and [nielsr](https://huggingface.co/nielsr).
The original code can be found [here](https://github.com/IDEA-Research/GroundingDINO).

## Usage tips

- One can use [GroundingDinoProcessor](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoProcessor) to prepare image-text pairs for the model.
- To separate classes in the text use a period e.g. "a cat. a dog."
- When using multiple classes (e.g. `"a cat. a dog."`), use `post_process_grounded_object_detection` from [GroundingDinoProcessor](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoProcessor) to post process outputs. Since, the labels returned from `post_process_object_detection` represent the indices from the model dimension where prob > threshold.

Here's how to use the model for zero-shot object detection:

```python
import requests

import torch
from PIL import Image
from transformers import AutoProcessor, AutoModelForZeroShotObjectDetection

model_id = "IDEA-Research/grounding-dino-tiny"

processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForZeroShotObjectDetection.from_pretrained(model_id, device_map="auto")

image_url = "http://images.cocodataset.org/val2017/000000039769.jpg"
image = Image.open(requests.get(image_url, stream=True).raw)
# Check for cats and remote controls
text_labels = [["a cat", "a remote control"]]

inputs = processor(images=image, text=text_labels, return_tensors="pt").to(model.device)
with torch.no_grad():
    outputs = model(**inputs)

results = processor.post_process_grounded_object_detection(
    outputs,
    inputs.input_ids,
    threshold=0.4,
    text_threshold=0.3,
    target_sizes=[image.size[::-1]]
)

# Retrieve the first image result
result = results[0]
for box, score, labels in zip(result["boxes"], result["scores"], result["labels"]):
    box = [round(x, 2) for x in box.tolist()]
    print(f"Detected {labels} with confidence {round(score.item(), 3)} at location {box}")
Detected a cat with confidence 0.468 at location [344.78, 22.9, 637.3, 373.62]
Detected a cat with confidence 0.426 at location [11.74, 51.55, 316.51, 473.22]
```

## Grounded SAM

One can combine Grounding DINO with the [Segment Anything](sam) model for text-based mask generation as introduced in [Grounded SAM: Assembling Open-World Models for Diverse Visual Tasks](https://huggingface.co/papers/2401.14159). You can refer to this [demo notebook](https://github.com/NielsRogge/Transformers-Tutorials/blob/master/Grounding%20DINO/GroundingDINO_with_Segment_Anything.ipynb) 🌍 for details.

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/grounded_sam.png"
alt="drawing" width="900"/>

 Grounded SAM overview. Taken from the original repository. 

## Resources

A list of official Hugging Face and community (indicated by 🌎) resources to help you get started with Grounding DINO. If you're interested in submitting a resource to be included here, please feel free to open a Pull Request and we'll review it! The resource should ideally demonstrate something new instead of duplicating an existing resource.

- Demo notebooks regarding inference with Grounding DINO as well as combining it with [SAM](sam) can be found [here](https://github.com/NielsRogge/Transformers-Tutorials/tree/master/Grounding%20DINO). 🌎

## GroundingDinoImageProcessor[[transformers.GroundingDinoImageProcessor]]

- **format** (`str`, *kwargs*, *optional*, defaults to `AnnotationFormat.COCO_DETECTION`) --
  Data format of the annotations. One of "coco_detection" or "coco_panoptic".
- **do_convert_annotations** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Controls whether to convert the annotations to the format expected by the GROUNDING_DINO model. Converts the
  bounding boxes to the format `(center_x, center_y, width, height)` and in the range `[0, 1]`.
  Can be overridden by the `do_convert_annotations` parameter in the `preprocess` method.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a GroundingDinoImageProcessor image processor.

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
  Controls whether to convert the annotations to the format expected by the GROUNDING_DINO model. Converts the
  bounding boxes to the format `(center_x, center_y, width, height)` and in the range `[0, 1]`.
  Can be overridden by the `do_convert_annotations` parameter in the `preprocess` method.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## GroundingDinoImageProcessorPil[[transformers.GroundingDinoImageProcessorPil]]

- **format** (`str`, *kwargs*, *optional*, defaults to `AnnotationFormat.COCO_DETECTION`) --
  Data format of the annotations. One of "coco_detection" or "coco_panoptic".
- **do_convert_annotations** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Controls whether to convert the annotations to the format expected by the GROUNDING_DINO model. Converts the
  bounding boxes to the format `(center_x, center_y, width, height)` and in the range `[0, 1]`.
  Can be overridden by the `do_convert_annotations` parameter in the `preprocess` method.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a GroundingDinoImageProcessor image processor.

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
  Controls whether to convert the annotations to the format expected by the GROUNDING_DINO model. Converts the
  bounding boxes to the format `(center_x, center_y, width, height)` and in the range `[0, 1]`.
  Can be overridden by the `do_convert_annotations` parameter in the `preprocess` method.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** (`GroundingDinoObjectDetectionOutput`) --
  Raw outputs of the model.
- **threshold** (`float`, *optional*, defaults to 0.1) --
  Score threshold to keep object detection predictions.
- **target_sizes** (`torch.Tensor` or `list[tuple[int, int]]`, *optional*) --
  Tensor of shape `(batch_size, 2)` or list of tuples (`tuple[int, int]`) containing the target size
  `(height, width)` of each image in the batch. If unset, predictions will not be resized.`list[Dict]`A list of dictionaries, each dictionary containing the following keys:
- "scores": The confidence scores for each predicted box on the image.
- "labels": Indexes of the classes predicted by the model on the image.
- "boxes": Image bounding boxes in (top_left_x, top_left_y, bottom_right_x, bottom_right_y) format.

Converts the raw output of [GroundingDinoForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoForObjectDetection) into final bounding boxes in (top_left_x, top_left_y,
bottom_right_x, bottom_right_y) format.

## GroundingDinoProcessor[[transformers.GroundingDinoProcessor]]

- **image_processor** (`GroundingDinoImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`BertTokenizer`) --
  The tokenizer is a required input.
Constructs a GroundingDinoProcessor which wraps a image processor and a tokenizer into a single processor.

[GroundingDinoProcessor](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoProcessor) offers all the functionalities of [GroundingDinoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoImageProcessor) and [BertTokenizer](/docs/transformers/v5.14.0/en/model_doc/electra#transformers.BertTokenizer). See the
[~GroundingDinoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoImageProcessor) and [~BertTokenizer](/docs/transformers/v5.14.0/en/model_doc/electra#transformers.BertTokenizer) for more information.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **text** (`Union[str, list[str], list[list[str]]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.
- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) --
  Additional processing options for each modality (text, images, videos, audio). Model-specific parameters
  are listed above; see the TypedDict class for the complete list of supported arguments.`~tokenization_utils_base.BatchEncoding`- **data** (`dict`, *optional*) -- Dictionary of lists/arrays/tensors returned by the `__call__`/`encode_plus`/`batch_encode_plus` methods
  ('input_ids', 'attention_mask', etc.).
- **encoding** (`tokenizers.Encoding` or `Sequence[tokenizers.Encoding]`, *optional*) -- If the tokenizer is a fast tokenizer which outputs additional information like mapping from word/character
  space to token space the `tokenizers.Encoding` instance or list of instance (for batches) hold this
  information.
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.
- **prepend_batch_axis** (`bool`, *optional*, defaults to `False`) -- Whether or not to add a batch axis when converting to tensors (see `tensor_type` above). Note that this
  parameter has an effect if the parameter `tensor_type` is set, *otherwise has no effect*.
- **n_sequences** (`int`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** (`GroundingDinoObjectDetectionOutput`) --
  Raw outputs of the model.
- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  The token ids of the input text. If not provided will be taken from the model output.
- **threshold** (`float`, *optional*, defaults to 0.25) --
  Threshold to keep object detection predictions based on confidence score.
- **text_threshold** (`float`, *optional*, defaults to 0.25) --
  Score threshold to keep text detection predictions.
- **target_sizes** (`torch.Tensor` or `list[tuple[int, int]]`, *optional*) --
  Tensor of shape `(batch_size, 2)` or list of tuples (`tuple[int, int]`) containing the target size
  `(height, width)` of each image in the batch. If unset, predictions will not be resized.
- **text_labels** (`list[list[str]]`, *optional*) --
  List of candidate labels to be detected on each image. At the moment it's *NOT used*, but required
  to be in signature for the zero-shot object detection pipeline. Text labels are instead extracted
  from the `input_ids` tensor provided in `outputs`.`list[Dict]`A list of dictionaries, each dictionary containing the
- **scores**: tensor of confidence scores for detected objects
- **boxes**: tensor of bounding boxes in [x0, y0, x1, y1] format
- **labels**: list of text labels for each detected object (will be replaced with integer ids in v4.51.0)
- **text_labels**: list of text labels for detected objects

Converts the raw output of [GroundingDinoForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoForObjectDetection) into final bounding boxes in (top_left_x, top_left_y,
bottom_right_x, bottom_right_y) format and get the associated text label.

## GroundingDinoConfig[[transformers.GroundingDinoConfig]]

- **is_encoder_decoder** (`bool`, *optional*, defaults to `True`) --
  Whether the model is used as an encoder/decoder or not.
- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **num_queries** (`int`, *optional*, defaults to 900) --
  Number of object queries, i.e. detection slots. This is the maximal number of objects
  [GroundingDinoModel](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoModel) can detect in a single image.
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
- **auxiliary_loss** (`bool`, *optional*, defaults to `False`) --
  Whether auxiliary decoding losses (losses at each decoder layer) are to be used.
- **position_embedding_type** (`str`, *optional*, defaults to `"sine"`) --
  Type of position embeddings to be used on top of the image features. One of `"sine"` or `"learned"`.
- **num_feature_levels** (`int`, *optional*, defaults to 4) --
  The number of input feature levels.
- **encoder_n_points** (`int`, *optional*, defaults to 4) --
  The number of sampled keys in each feature level for each attention head in the encoder.
- **decoder_n_points** (`int`, *optional*, defaults to 4) --
  The number of sampled keys in each feature level for each attention head in the decoder.
- **two_stage** (`bool`, *optional*, defaults to `True`) --
  Whether to apply a two-stage deformable DETR, where the region proposals are also generated by a variant of
  Grounding DINO, which are further fed into the decoder for iterative bounding box refinement.
- **class_cost** (`float`, *optional*, defaults to `1.0`) --
  Relative weight of the classification error in the Hungarian matching cost.
- **bbox_cost** (`float`, *optional*, defaults to `5.0`) --
  Relative weight of the L1 bounding box error in the Hungarian matching cost.
- **giou_cost** (`float`, *optional*, defaults to `2.0`) --
  Relative weight of the generalized IoU loss in the Hungarian matching cost.
- **bbox_loss_coefficient** (`float`, *optional*, defaults to `5.0`) --
  Relative weight of the L1 bounding box loss in the panoptic segmentation loss.
- **giou_loss_coefficient** (`float`, *optional*, defaults to `2.0`) --
  Relative weight of the generalized IoU loss in the panoptic segmentation loss.
- **focal_alpha** (`float`, *optional*, defaults to `0.25`) --
  Alpha parameter in the focal loss.
- **disable_custom_kernels** (`bool`, *optional*, defaults to `False`) --
  Disable the use of custom CUDA and CPU kernels. This option is necessary for the ONNX export, as custom
  kernels are not supported by PyTorch ONNX export.
- **max_text_len** (`int`, *optional*, defaults to 256) --
  The maximum length of the text input.
- **text_enhancer_dropout** (`float`, *optional*, defaults to 0.0) --
  The dropout ratio for the text enhancer.
- **fusion_droppath** (`float`, *optional*, defaults to 0.1) --
  The droppath ratio for the fusion module.
- **fusion_dropout** (`float`, *optional*, defaults to 0.0) --
  The dropout ratio for the fusion module.
- **embedding_init_target** (`bool`, *optional*, defaults to `True`) --
  Whether to initialize the target with Embedding weights.
- **query_dim** (`int`, *optional*, defaults to 4) --
  The dimension of the query vector.
- **decoder_bbox_embed_share** (`bool`, *optional*, defaults to `True`) --
  Whether to share the bbox regression head for all decoder layers.
- **two_stage_bbox_embed_share** (`bool`, *optional*, defaults to `False`) --
  Whether to share the bbox embedding between the two-stage bbox generator and the region proposal
  generation.
- **positional_embedding_temperature** (`float`, *optional*, defaults to 20) --
  The temperature for Sine Positional Embedding that is used together with vision backbone.
- **init_std** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the layer normalization layers.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Grounding DinoModel. It is used to instantiate a Grounding Dino
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [IDEA-Research/grounding-dino-tiny](https://huggingface.co/IDEA-Research/grounding-dino-tiny)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import GroundingDinoConfig, GroundingDinoModel

>>> # Initializing a Grounding DINO IDEA-Research/grounding-dino-tiny style configuration
>>> configuration = GroundingDinoConfig()

>>> # Initializing a model (with random weights) from the IDEA-Research/grounding-dino-tiny style configuration
>>> model = GroundingDinoModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## GroundingDinoModel[[transformers.GroundingDinoModel]]

- **config** ([GroundingDinoConfig](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Grounding DINO Model (consisting of a backbone and encoder-decoder Transformer) outputting raw
hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "input_ids", "val": ": )>"}, {"name": "token_type_ids", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "pixel_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "encoder_outputs", "val": " = None"}, {"name": "output_attentions", "val": " = None"}, {"name": "output_hidden_states", "val": " = None"}, {"name": "return_dict", "val": " = None"}, {"name": "**kwargs", "val": ""}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [GroundingDinoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoImageProcessor). See `GroundingDinoImageProcessor.__call__()` for details ([GroundingDinoProcessor](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoProcessor) uses
  [GroundingDinoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoImageProcessor) for processing images).
- **input_ids** (`torch.LongTensor` of shape `(batch_size, text_sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default should you provide
  it.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [BertTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.
- **token_type_ids** (`torch.LongTensor` of shape `(batch_size, text_sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0,
  1]`: 0 corresponds to a `sentence A` token, 1 corresponds to a `sentence B` token

  [What are token type IDs?](../glossary#token-type-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **pixel_mask** (`torch.Tensor` of shape `(batch_size, height, width)`, *optional*) --
  Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:

  - 1 for pixels that are real (i.e. **not masked**),
  - 0 for pixels that are padding (i.e. **masked**).

  [What are attention masks?](../glossary#attention-mask)
- **encoder_outputs** (``) --
  Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`)
  `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of
  hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.
- **output_attentions** (``) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (``) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (``) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.`GroundingDinoModelOutput` or `tuple(torch.FloatTensor)`A `GroundingDinoModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([GroundingDinoConfig](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoConfig)) and inputs.
The [GroundingDinoModel](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_queries, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the decoder of the model.
- **init_reference_points** (`torch.FloatTensor` of shape  `(batch_size, num_queries, 4)`) -- Initial reference points sent through the Transformer decoder.
- **intermediate_hidden_states** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, hidden_size)`) -- Stacked intermediate hidden states (output of each layer of the decoder).
- **intermediate_reference_points** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, 4)`) -- Stacked intermediate reference points (reference points of each layer of the decoder).
- **decoder_hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the decoder at the output of each layer plus the initial embedding outputs.
- **decoder_attentions** (`tuple[tuple[torch.FloatTensor]]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **encoder_last_hidden_state_vision** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_last_hidden_state_text** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_vision_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the vision embeddings + one for the output of each
  layer) of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states of the vision encoder at the
  output of each layer plus the initial embedding outputs.
- **encoder_text_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the text embeddings + one for the output of each layer)
  of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states of the text encoder at the output of
  each layer plus the initial embedding outputs.
- **encoder_attentions** (`tuple(tuple(torch.FloatTensor))`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of tuples of `torch.FloatTensor` (one for attention for each layer) of shape `(batch_size, num_heads,
  sequence_length, sequence_length)`. Attentions weights after the attention softmax, used to compute the
  weighted average in the text-vision attention, vision-text attention, text-enhancer (self-attention) and
  multi-scale deformable attention heads. attention softmax, used to compute the weighted average in the
  bi-attention heads.
- **enc_outputs_class** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_labels)`, *optional*, returned when `config.two_stage=True`) -- Predicted bounding boxes scores where the top `config.num_queries` scoring bounding boxes are picked as
  region proposals in the first stage. Output of bounding box binary classification (i.e. foreground and
  background).
- **enc_outputs_coord_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, 4)`, *optional*, returned when `config.two_stage=True`) -- Logits of predicted bounding boxes coordinates in the first stage.
- **encoder_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_labels)`, *optional*, returned when `config.two_stage=True`) -- Logits of top `config.num_queries` scoring bounding boxes in the first stage.
- **encoder_pred_boxes** (`torch.FloatTensor` of shape `(batch_size, sequence_length, 4)`, *optional*, returned when `config.two_stage=True`) -- Coordinates of top `config.num_queries` scoring bounding boxes in the first stage.

Examples:

```python
>>> from transformers import AutoProcessor, AutoModel
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))
>>> text = "a cat."

>>> processor = AutoProcessor.from_pretrained("IDEA-Research/grounding-dino-tiny")
>>> model = AutoModel.from_pretrained("IDEA-Research/grounding-dino-tiny")

>>> inputs = processor(images=image, text=text, return_tensors="pt")
>>> outputs = model(**inputs)

>>> last_hidden_states = outputs.last_hidden_state
>>> list(last_hidden_states.shape)
[1, 900, 256]
```

## GroundingDinoForObjectDetection[[transformers.GroundingDinoForObjectDetection]]

- **config** ([GroundingDinoConfig](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Grounding DINO Model (consisting of a backbone and encoder-decoder Transformer) with object detection heads on top,
for tasks such as COCO detection.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [GroundingDinoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoImageProcessor). See `GroundingDinoImageProcessor.__call__()` for details ([GroundingDinoProcessor](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoProcessor) uses
  [GroundingDinoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoImageProcessor) for processing images).
- **input_ids** (`torch.LongTensor` of shape `(batch_size, text_sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default should you provide
  it.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [BertTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.
- **token_type_ids** (`torch.LongTensor` of shape `(batch_size, text_sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0,
  1]`: 0 corresponds to a `sentence A` token, 1 corresponds to a `sentence B` token

  [What are token type IDs?](../glossary#token-type-ids)
- **attention_mask** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **pixel_mask** (`torch.BoolTensor` of shape `(batch_size, height, width)`, *optional*) --
  Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:

  - 1 for pixels that are real (i.e. **not masked**),
  - 0 for pixels that are padding (i.e. **masked**).

  [What are attention masks?](../glossary#attention-mask)
- **encoder_outputs** (`Union[~models.grounding_dino.modeling_grounding_dino.GroundingDinoEncoderOutput, tuple]`, *optional*) --
  Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`)
  `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of
  hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.
- **labels** (`list[Dict]` of len `(batch_size,)`, *optional*) --
  Labels for computing the bipartite matching loss. List of dicts, each dictionary containing at least the
  following 2 keys: 'class_labels' and 'boxes' (the class labels and bounding boxes of an image in the batch
  respectively). The class labels themselves should be a `torch.LongTensor` of len `(number of bounding boxes
  in the image,)` and the boxes a `torch.FloatTensor` of shape `(number of bounding boxes in the image, 4)`.
The [GroundingDinoForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoForObjectDetection) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Examples:

```python
>>> import httpx
>>> from io import BytesIO

>>> import torch
>>> from PIL import Image
>>> from transformers import AutoProcessor, AutoModelForZeroShotObjectDetection

>>> model_id = "IDEA-Research/grounding-dino-tiny"
>>> device = "cuda"

>>> processor = AutoProcessor.from_pretrained(model_id)
>>> model = AutoModelForZeroShotObjectDetection.from_pretrained(model_id).to(device)

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))
>>> # Check for cats and remote controls
>>> text_labels = [["a cat", "a remote control"]]

>>> inputs = processor(images=image, text=text_labels, return_tensors="pt").to(device)
>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> results = processor.post_process_grounded_object_detection(
...     outputs,
...     threshold=0.4,
...     text_threshold=0.3,
...     target_sizes=[(image.height, image.width)]
... )
>>> # Retrieve the first image result
>>> result = results[0]
>>> for box, score, text_label in zip(result["boxes"], result["scores"], result["text_labels"]):
...     box = [round(x, 2) for x in box.tolist()]
...     print(f"Detected {text_label} with confidence {round(score.item(), 3)} at location {box}")
Detected a cat with confidence 0.479 at location [344.7, 23.11, 637.18, 374.28]
Detected a cat with confidence 0.438 at location [12.27, 51.91, 316.86, 472.44]
Detected a remote control with confidence 0.478 at location [38.57, 70.0, 176.78, 118.18]
```
