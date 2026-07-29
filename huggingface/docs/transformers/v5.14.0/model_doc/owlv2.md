# OWLv2

## Overview

OWLv2 was proposed in [Scaling Open-Vocabulary Object Detection](https://huggingface.co/papers/2306.09683) by Matthias Minderer, Alexey Gritsenko, Neil Houlsby. OWLv2 scales up [OWL-ViT](owlvit) using self-training, which uses an existing detector to generate pseudo-box annotations on image-text pairs. This results in large gains over the previous state-of-the-art for zero-shot object detection.

The abstract from the paper is the following:

*Open-vocabulary object detection has benefited greatly from pretrained vision-language models, but is still limited by the amount of available detection training data. While detection training data can be expanded by using Web image-text pairs as weak supervision, this has not been done at scales comparable to image-level pretraining. Here, we scale up detection data with self-training, which uses an existing detector to generate pseudo-box annotations on image-text pairs. Major challenges in scaling self-training are the choice of label space, pseudo-annotation filtering, and training efficiency. We present the OWLv2 model and OWL-ST self-training recipe, which address these challenges. OWLv2 surpasses the performance of previous state-of-the-art open-vocabulary detectors already at comparable training scales (~10M examples). However, with OWL-ST, we can scale to over 1B examples, yielding further large improvement: With an L/14 architecture, OWL-ST improves AP on LVIS rare classes, for which the model has seen no human box annotations, from 31.2% to 44.6% (43% relative improvement). OWL-ST unlocks Web-scale training for open-world localization, similar to what has been seen for image classification and language modelling.*

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/owlv2_overview.png"
alt="drawing" width="600"/>

 OWLv2 high-level overview. Taken from the original paper. 

This model was contributed by [nielsr](https://huggingface.co/nielsr).
The original code can be found [here](https://github.com/google-research/scenic/tree/main/scenic/projects/owl_vit).

## Usage example

OWLv2 is, just like its predecessor [OWL-ViT](owlvit), a zero-shot text-conditioned object detection model. OWL-ViT uses [CLIP](clip) as its multi-modal backbone, with a ViT-like Transformer to get visual features and a causal language model to get the text features. To use CLIP for detection, OWL-ViT removes the final token pooling layer of the vision model and attaches a lightweight classification and box head to each transformer output token. Open-vocabulary classification is enabled by replacing the fixed classification layer weights with the class-name embeddings obtained from the text model. The authors first train CLIP from scratch and fine-tune it end-to-end with the classification and box heads on standard detection datasets using a bipartite matching loss. One or multiple text queries per image can be used to perform zero-shot text-conditioned object detection.

[Owlv2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ImageProcessor) can be used to resize (or rescale) and normalize images for the model and [CLIPTokenizer](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPTokenizer) is used to encode the text. [Owlv2Processor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2Processor) wraps [Owlv2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ImageProcessor) and [CLIPTokenizer](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPTokenizer) into a single instance to both encode the text and prepare the images. The following example shows how to perform object detection using [Owlv2Processor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2Processor) and [Owlv2ForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ForObjectDetection).

```python
import requests
from PIL import Image
import torch

from transformers import Owlv2Processor, Owlv2ForObjectDetection

processor = Owlv2Processor.from_pretrained("google/owlv2-base-patch16-ensemble")
model = Owlv2ForObjectDetection.from_pretrained("google/owlv2-base-patch16-ensemble", device_map="auto")

url = "http://images.cocodataset.org/val2017/000000039769.jpg"
image = Image.open(requests.get(url, stream=True).raw)
text_labels = [["a photo of a cat", "a photo of a dog"]]
inputs = processor(text=text_labels, images=image, return_tensors="pt").to(model.device)
outputs = model(**inputs)

# Target image sizes (height, width) to rescale box predictions [batch_size, 2]
target_sizes = torch.tensor([(image.height, image.width)])
# Convert outputs (bounding boxes and class logits) to Pascal VOC format (xmin, ymin, xmax, ymax)
results = processor.post_process_grounded_object_detection(
    outputs=outputs, target_sizes=target_sizes, threshold=0.1, text_labels=text_labels
)
# Retrieve predictions for the first image for the corresponding text queries
result = results[0]
boxes, scores, text_labels = result["boxes"], result["scores"], result["text_labels"]
for box, score, text_label in zip(boxes, scores, text_labels):
    box = [round(i, 2) for i in box.tolist()]
    print(f"Detected {text_label} with confidence {round(score.item(), 3)} at location {box}")
Detected a photo of a cat with confidence 0.614 at location [341.67, 23.39, 642.32, 371.35]
Detected a photo of a cat with confidence 0.665 at location [6.75, 51.96, 326.62, 473.13]
```

## Resources

- A demo notebook on using OWLv2 for zero- and one-shot (image-guided) object detection can be found [here](https://github.com/NielsRogge/Transformers-Tutorials/tree/master/OWLv2).
- [Zero-shot object detection task guide](../tasks/zero_shot_object_detection)

The architecture of OWLv2 is identical to [OWL-ViT](owlvit), however the object detection head now also includes an objectness classifier, which predicts the (query-agnostic) likelihood that a predicted box contains an object (as opposed to background). The objectness score can be used to rank or filter predictions independently of text queries.
Usage of OWLv2 is identical to [OWL-ViT](owlvit) with a new, updated image processor ([Owlv2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ImageProcessor)).

## Owlv2Config[[transformers.Owlv2Config]]

- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether to return a `ModelOutput` (dataclass) instead of a plain tuple.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **projection_dim** (`int`, *optional*, defaults to `512`) --
  Dimensionality of text and vision projection layers.
- **logit_scale_init_value** (`float`, *optional*, defaults to `2.6592`) --
  The initial value of the *logit_scale* parameter.
- **initializer_factor** (`float`, *optional*, defaults to `1.0`) --
  A factor for initializing all weight matrices (should be kept to 1, used internally for initialization
  testing).

This is the configuration class to store the configuration of a Owlv2Model. It is used to instantiate a Owlv2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/owlv2-base-patch16](https://huggingface.co/google/owlv2-base-patch16)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Owlv2TextConfig[[transformers.Owlv2TextConfig]]

- **vocab_size** (`int`, *optional*, defaults to `49408`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `512`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `2048`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `12`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **max_position_embeddings** (`int`, *optional*, defaults to `16`) --
  The maximum sequence length that this model might ever be used with.
- **hidden_act** (`str`, *optional*, defaults to `quick_gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the layer normalization layers.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **initializer_factor** (`float`, *optional*, defaults to `1.0`) --
  A factor for initializing all weight matrices (should be kept to 1, used internally for initialization
  testing).
- **pad_token_id** (`int`, *optional*, defaults to `0`) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `49406`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `49407`) --
  Token id used for end-of-stream in the vocabulary.

This is the configuration class to store the configuration of a Owlv2Model. It is used to instantiate a Owlv2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/owlv2-base-patch16](https://huggingface.co/google/owlv2-base-patch16)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Owlv2TextConfig, Owlv2TextModel

>>> # Initializing a Owlv2TextModel with google/owlv2-base-patch16 style configuration
>>> configuration = Owlv2TextConfig()

>>> # Initializing a Owlv2TextConfig from the google/owlv2-base-patch16 style configuration
>>> model = Owlv2TextModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Owlv2VisionConfig[[transformers.Owlv2VisionConfig]]

- **hidden_size** (`int`, *optional*, defaults to `768`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `3072`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `12`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `12`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `768`) --
  The size (resolution) of each image.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) --
  The size (resolution) of each patch.
- **hidden_act** (`str`, *optional*, defaults to `quick_gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the layer normalization layers.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **initializer_factor** (`float`, *optional*, defaults to `1.0`) --
  A factor for initializing all weight matrices (should be kept to 1, used internally for initialization
  testing).

This is the configuration class to store the configuration of a Owlv2Model. It is used to instantiate a Owlv2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/owlv2-base-patch16](https://huggingface.co/google/owlv2-base-patch16)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Owlv2VisionConfig, Owlv2VisionModel

>>> # Initializing a Owlv2VisionModel with google/owlv2-base-patch16 style configuration
>>> configuration = Owlv2VisionConfig()

>>> # Initializing a Owlv2VisionModel model from the google/owlv2-base-patch16 style configuration
>>> model = Owlv2VisionModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Owlv2ImageProcessor[[transformers.Owlv2ImageProcessor]]

- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a Owlv2ImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** (`Owlv2ObjectDetectionOutput`) --
  Raw outputs of the model.
- **threshold** (`float`, *optional*, defaults to 0.1) --
  Score threshold to keep object detection predictions.
- **target_sizes** (`torch.Tensor` or `list[tuple[int, int]]`, *optional*) --
  Tensor of shape `(batch_size, 2)` or list of tuples (`tuple[int, int]`) containing the target size
  `(height, width)` of each image in the batch. If unset, predictions will not be resized.`list[Dict]`A list of dictionaries, each dictionary containing the following keys:
- "scores": The confidence scores for each predicted box on the image.
- "labels": Indexes of the classes predicted by the model on the image.
- "boxes": Image bounding boxes in (top_left_x, top_left_y, bottom_right_x, bottom_right_y) format.

Converts the raw output of [Owlv2ForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ForObjectDetection) into final bounding boxes in (top_left_x, top_left_y,
bottom_right_x, bottom_right_y) format.

- **outputs** (`Owlv2ImageGuidedObjectDetectionOutput`) --
  Raw outputs of the model.
- **threshold** (`float`, *optional*, defaults to 0.0) --
  Minimum confidence threshold to use to filter out predicted boxes.
- **nms_threshold** (`float`, *optional*, defaults to 0.3) --
  IoU threshold for non-maximum suppression of overlapping boxes.
- **target_sizes** (`torch.Tensor`, *optional*) --
  Tensor of shape (batch_size, 2) where each entry is the (height, width) of the corresponding image in
  the batch. If set, predicted normalized bounding boxes are rescaled to the target sizes. If left to
  None, predictions will not be unnormalized.`list[Dict]`A list of dictionaries, each dictionary containing the scores, labels and boxes for an image
in the batch as predicted by the model. All labels are set to None as
`Owlv2ForObjectDetection.image_guided_detection` perform one-shot object detection.

Converts the output of [Owlv2ForObjectDetection.image_guided_detection()](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ForObjectDetection.image_guided_detection) into the format expected by the COCO
api.

## Owlv2ImageProcessorPil[[transformers.Owlv2ImageProcessorPil]]

- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a Owlv2ImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** (`Owlv2ObjectDetectionOutput`) --
  Raw outputs of the model.
- **threshold** (`float`, *optional*, defaults to 0.1) --
  Score threshold to keep object detection predictions.
- **target_sizes** (`torch.Tensor` or `list[tuple[int, int]]`, *optional*) --
  Tensor of shape `(batch_size, 2)` or list of tuples (`tuple[int, int]`) containing the target size
  `(height, width)` of each image in the batch. If unset, predictions will not be resized.`list[Dict]`A list of dictionaries, each dictionary containing the following keys:
- "scores": The confidence scores for each predicted box on the image.
- "labels": Indexes of the classes predicted by the model on the image.
- "boxes": Image bounding boxes in (top_left_x, top_left_y, bottom_right_x, bottom_right_y) format.

Converts the raw output of [Owlv2ForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ForObjectDetection) into final bounding boxes in (top_left_x, top_left_y,
bottom_right_x, bottom_right_y) format.

- **outputs** (`Owlv2ImageGuidedObjectDetectionOutput`) --
  Raw outputs of the model.
- **threshold** (`float`, *optional*, defaults to 0.0) --
  Minimum confidence threshold to use to filter out predicted boxes.
- **nms_threshold** (`float`, *optional*, defaults to 0.3) --
  IoU threshold for non-maximum suppression of overlapping boxes.
- **target_sizes** (`torch.Tensor`, *optional*) --
  Tensor of shape (batch_size, 2) where each entry is the (height, width) of the corresponding image in
  the batch. If set, predicted normalized bounding boxes are rescaled to the target sizes. If left to
  None, predictions will not be unnormalized.`list[Dict]`A list of dictionaries, each dictionary containing the scores, labels and boxes for an image
in the batch as predicted by the model. All labels are set to None as
`Owlv2ForObjectDetection.image_guided_detection` perform one-shot object detection.

Converts the output of [Owlv2ForObjectDetection.image_guided_detection()](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ForObjectDetection.image_guided_detection) into the format expected by the COCO
api.

## Owlv2Processor[[transformers.Owlv2Processor]]

- **image_processor** (`Owlv2ImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`CLIPTokenizer`) --
  The tokenizer is a required input.
Constructs a Owlv2Processor which wraps a image processor and a tokenizer into a single processor.

[Owlv2Processor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2Processor) offers all the functionalities of [Owlv2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ImageProcessor) and [CLIPTokenizer](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPTokenizer). See the
[~Owlv2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ImageProcessor) and [~CLIPTokenizer](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPTokenizer) for more information.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **text** (`Union[str, list[str], list[list[str]]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **query_images** (`ImageInput`, *kwargs*, *optional*) --
  Query images to use for image-guided object detection. When provided, these images serve as visual queries
  to find similar objects in the main `images`. The query images override any text prompts, and the model
  performs image-to-image matching instead of text-to-image matching.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.
- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) --
  Additional processing options for each modality (text, images, videos, audio). Model-specific parameters
  are listed above; see the TypedDict class for the complete list of supported arguments.[BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature)A [BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature) with the following fields:
- **input_ids** -- List of token ids to be fed to a model. Returned when `text` is not `None`.
- **attention_mask** -- List of indices specifying which tokens should be attended to by the model (when
  `return_attention_mask=True` or if *"attention_mask"* is in `self.model_input_names` and if `text` is not
  `None`).
- **pixel_values** -- Pixel values to be fed to a model. Returned when `images` is not `None`.
- **query_pixel_values** -- Pixel values of the query images to be fed to a model. Returned when `query_images` is not `None`.

- **outputs** (`Owlv2ObjectDetectionOutput`) --
  Raw outputs of the model.
- **threshold** (`float`, *optional*, defaults to 0.1) --
  Score threshold to keep object detection predictions.
- **target_sizes** (`torch.Tensor` or `list[tuple[int, int]]`, *optional*) --
  Tensor of shape `(batch_size, 2)` or list of tuples (`tuple[int, int]`) containing the target size
  `(height, width)` of each image in the batch. If unset, predictions will not be resized.
- **text_labels** (`list[list[str]]`, *optional*) --
  List of lists of text labels for each image in the batch. If unset, "text_labels" in output will be
  set to `None`.`list[Dict]`A list of dictionaries, each dictionary containing the following keys:
- "scores": The confidence scores for each predicted box on the image.
- "labels": Indexes of the classes predicted by the model on the image.
- "boxes": Image bounding boxes in (top_left_x, top_left_y, bottom_right_x, bottom_right_y) format.
- "text_labels": The text labels for each predicted bounding box on the image.

Converts the raw output of [Owlv2ForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ForObjectDetection) into final bounding boxes in (top_left_x, top_left_y,
bottom_right_x, bottom_right_y) format.

- **outputs** (`Owlv2ImageGuidedObjectDetectionOutput`) --
  Raw outputs of the model.
- **threshold** (`float`, *optional*, defaults to 0.0) --
  Minimum confidence threshold to use to filter out predicted boxes.
- **nms_threshold** (`float`, *optional*, defaults to 0.3) --
  IoU threshold for non-maximum suppression of overlapping boxes.
- **target_sizes** (`torch.Tensor`, *optional*) --
  Tensor of shape (batch_size, 2) where each entry is the (height, width) of the corresponding image in
  the batch. If set, predicted normalized bounding boxes are rescaled to the target sizes. If left to
  None, predictions will not be unnormalized.`list[Dict]`A list of dictionaries, each dictionary containing the following keys:
- "scores": The confidence scores for each predicted box on the image.
- "boxes": Image bounding boxes in (top_left_x, top_left_y, bottom_right_x, bottom_right_y) format.
- "labels": Set to `None`.

Converts the output of [Owlv2ForObjectDetection.image_guided_detection()](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ForObjectDetection.image_guided_detection) into the format expected by the COCO
api.

## Owlv2Model[[transformers.Owlv2Model]]

- **config** ([Owlv2Config](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Owlv2 Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Owlv2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ImageProcessor). See `Owlv2ImageProcessor.__call__()` for details ([Owlv2Processor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2Processor) uses
  [Owlv2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ImageProcessor) for processing images).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **return_loss** (`bool`, *optional*) --
  Whether or not to return the contrastive loss.
- **interpolate_pos_encoding** (`bool`, *optional*, defaults to `False`) --
  Whether to interpolate the pre-trained position encodings.
- **return_base_image_embeds** (`bool`, *optional*) --
  Whether or not to return the base image embeddings.`Owlv2Output` or `tuple(torch.FloatTensor)`A `Owlv2Output` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Owlv2Config](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2Config)) and inputs.
The [Owlv2Model](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2Model) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `return_loss` is `True`) -- Contrastive loss for image-text similarity.
- **logits_per_image** (`torch.FloatTensor` of shape `(image_batch_size, text_batch_size)`) -- The scaled dot product scores between `image_embeds` and `text_embeds`. This represents the image-text
  similarity scores.
- **logits_per_text** (`torch.FloatTensor` of shape `(text_batch_size, image_batch_size)`) -- The scaled dot product scores between `text_embeds` and `image_embeds`. This represents the text-image
  similarity scores.
- **text_embeds** (`torch.FloatTensor` of shape `(batch_size * num_max_text_queries, output_dim`) -- The text embeddings obtained by applying the projection layer to the pooled output of [Owlv2TextModel](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2TextModel).
- **image_embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim`) -- The image embeddings obtained by applying the projection layer to the pooled output of
  [Owlv2VisionModel](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2VisionModel).
- **text_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The output of the [Owlv2TextModel](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2TextModel).
- **vision_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The output of the [Owlv2VisionModel](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2VisionModel).

Examples:
```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoProcessor, Owlv2Model

>>> model = Owlv2Model.from_pretrained("google/owlv2-base-patch16-ensemble")
>>> processor = AutoProcessor.from_pretrained("google/owlv2-base-patch16-ensemble")
>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))
>>> inputs = processor(text=[["a photo of a cat", "a photo of a dog"]], images=image, return_tensors="pt")
>>> outputs = model(**inputs)
>>> logits_per_image = outputs.logits_per_image  # this is the image-text similarity score
>>> probs = logits_per_image.softmax(dim=1)  # we can take the softmax to get the label probabilities
```

)>"}, {"name": "attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **input_ids** (`torch.LongTensor` of shape `(batch_size * num_max_text_queries, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See
  [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details. [What are input
  IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Owlv2Config](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2Config)) and inputs.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Last layer hidden-state of the first token of the sequence (classification token) after further processing
  through the layers used for the auxiliary pretraining task. E.g. for BERT-family of models, this returns
  the classification token after processing through a linear layer and a tanh activation function. The linear
  layer weights are trained from the next sentence prediction (classification) objective during pretraining.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:
```python
>>> import torch
>>> from transformers import AutoProcessor, Owlv2Model

>>> model = Owlv2Model.from_pretrained("google/owlv2-base-patch16-ensemble")
>>> processor = AutoProcessor.from_pretrained("google/owlv2-base-patch16-ensemble")
>>> inputs = processor(
...     text=[["a photo of a cat", "a photo of a dog"], ["photo of a astranaut"]], return_tensors="pt"
... )
>>> with torch.inference_mode():
...     text_features = model.get_text_features(**inputs)
```

)>"}, {"name": "interpolate_pos_encoding", "val": ": bool = False"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Owlv2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ImageProcessor). See `Owlv2ImageProcessor.__call__()` for details ([Owlv2Processor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2Processor) uses
  [Owlv2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ImageProcessor) for processing images).
- **interpolate_pos_encoding** (`bool`, *optional*, defaults to `False`) --
  Whether to interpolate the pre-trained position encodings.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Owlv2Config](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2Config)) and inputs.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Last layer hidden-state of the first token of the sequence (classification token) after further processing
  through the layers used for the auxiliary pretraining task. E.g. for BERT-family of models, this returns
  the classification token after processing through a linear layer and a tanh activation function. The linear
  layer weights are trained from the next sentence prediction (classification) objective during pretraining.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:
```python
>>> import torch
>>> from transformers.image_utils import load_image
>>> from transformers import AutoProcessor, Owlv2Model

>>> model = Owlv2Model.from_pretrained("google/owlv2-base-patch16-ensemble")
>>> processor = AutoProcessor.from_pretrained("google/owlv2-base-patch16-ensemble")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> image = load_image(url)

>>> inputs = processor(images=image, return_tensors="pt")
>>> with torch.inference_mode():
...     image_features = model.get_image_features(**inputs)
```

## Owlv2TextModel[[transformers.Owlv2TextModel]]

- **input_ids** (`torch.LongTensor` of shape `(batch_size * num_max_text_queries, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See
  [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details. [What are input
  IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Owlv2Config](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2Config)) and inputs.
The [Owlv2TextModel](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2TextModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Last layer hidden-state of the first token of the sequence (classification token) after further processing
  through the layers used for the auxiliary pretraining task. E.g. for BERT-family of models, this returns
  the classification token after processing through a linear layer and a tanh activation function. The linear
  layer weights are trained from the next sentence prediction (classification) objective during pretraining.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:
```python
>>> from transformers import AutoProcessor, Owlv2TextModel

>>> model = Owlv2TextModel.from_pretrained("google/owlv2-base-patch16")
>>> processor = AutoProcessor.from_pretrained("google/owlv2-base-patch16")
>>> inputs = processor(
...     text=[["a photo of a cat", "a photo of a dog"], ["photo of a astranaut"]], return_tensors="pt"
... )
>>> outputs = model(**inputs)
>>> last_hidden_state = outputs.last_hidden_state
>>> pooled_output = outputs.pooler_output  # pooled (EOS token) states
```

## Owlv2VisionModel[[transformers.Owlv2VisionModel]]

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Owlv2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ImageProcessor). See `Owlv2ImageProcessor.__call__()` for details ([Owlv2Processor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2Processor) uses
  [Owlv2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ImageProcessor) for processing images).
- **interpolate_pos_encoding** (`bool`, *optional*, defaults to `False`) --
  Whether to interpolate the pre-trained position encodings.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Owlv2Config](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2Config)) and inputs.
The [Owlv2VisionModel](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2VisionModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Last layer hidden-state of the first token of the sequence (classification token) after further processing
  through the layers used for the auxiliary pretraining task. E.g. for BERT-family of models, this returns
  the classification token after processing through a linear layer and a tanh activation function. The linear
  layer weights are trained from the next sentence prediction (classification) objective during pretraining.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:
```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoProcessor, Owlv2VisionModel

>>> model = Owlv2VisionModel.from_pretrained("google/owlv2-base-patch16")
>>> processor = AutoProcessor.from_pretrained("google/owlv2-base-patch16")
>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> inputs = processor(images=image, return_tensors="pt")

>>> outputs = model(**inputs)
>>> last_hidden_state = outputs.last_hidden_state
>>> pooled_output = outputs.pooler_output  # pooled CLS states
```

## Owlv2ForObjectDetection[[transformers.Owlv2ForObjectDetection]]

)>"}, {"name": "pixel_values", "val": ": FloatTensor"}, {"name": "attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "interpolate_pos_encoding", "val": ": bool = False"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **input_ids** (`torch.LongTensor` of shape `(batch_size * num_max_text_queries, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See
  [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details. [What are input
  IDs?](../glossary#input-ids).
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Owlv2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ImageProcessor). See `Owlv2ImageProcessor.__call__()` for details ([Owlv2Processor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2Processor) uses
  [Owlv2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ImageProcessor) for processing images).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **interpolate_pos_encoding** (`bool`, *optional*, defaults to `False`) --
  Whether to interpolate the pre-trained position encodings.`Owlv2ObjectDetectionOutput` or `tuple(torch.FloatTensor)`A `Owlv2ObjectDetectionOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Owlv2Config](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2Config)) and inputs.
The [Owlv2ForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ForObjectDetection) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` are provided)) -- Total loss as a linear combination of a negative log-likelihood (cross-entropy) for class prediction and a
  bounding box loss. The latter is defined as a linear combination of the L1 loss and the generalized
  scale-invariant IoU loss.
- **loss_dict** (`Dict`, *optional*) -- A dictionary containing the individual losses. Useful for logging.
- **logits** (`torch.FloatTensor` of shape `(batch_size, num_patches, num_queries)`) -- Classification logits (including no-object) for all queries.
- **objectness_logits** (`torch.FloatTensor` of shape `(batch_size, num_patches, 1)`) -- The objectness logits of all image patches. OWL-ViT represents images as a set of image patches where the
  total number of patches is (image_size / patch_size)**2.
- **pred_boxes** (`torch.FloatTensor` of shape `(batch_size, num_patches, 4)`) -- Normalized boxes coordinates for all queries, represented as (center_x, center_y, width, height). These
  values are normalized in [0, 1], relative to the size of each individual image in the batch (disregarding
  possible padding). You can use [post_process_object_detection()](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ImageProcessor.post_process_object_detection) to retrieve the
  unnormalized bounding boxes.
- **text_embeds** (`torch.FloatTensor` of shape `(batch_size, num_max_text_queries, output_dim`) -- The text embeddings obtained by applying the projection layer to the pooled output of [Owlv2TextModel](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2TextModel).
- **image_embeds** (`torch.FloatTensor` of shape `(batch_size, patch_size, patch_size, output_dim`) -- Pooled output of [Owlv2VisionModel](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2VisionModel). OWLv2 represents images as a set of image patches and computes image
  embeddings for each patch.
- **class_embeds** (`torch.FloatTensor` of shape `(batch_size, num_patches, hidden_size)`) -- Class embeddings of all image patches. OWLv2 represents images as a set of image patches where the total
  number of patches is (image_size / patch_size)**2.
- **text_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The output of the [Owlv2TextModel](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2TextModel).
- **vision_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The output of the [Owlv2VisionModel](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2VisionModel).

Examples:
```python
>>> import httpx
>>> from io import BytesIO
>>> from PIL import Image
>>> import torch

>>> from transformers import Owlv2Processor, Owlv2ForObjectDetection

>>> processor = Owlv2Processor.from_pretrained("google/owlv2-base-patch16-ensemble")
>>> model = Owlv2ForObjectDetection.from_pretrained("google/owlv2-base-patch16-ensemble")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))
>>> text_labels = [["a photo of a cat", "a photo of a dog"]]
>>> inputs = processor(text=text_labels, images=image, return_tensors="pt")
>>> outputs = model(**inputs)

>>> # Target image sizes (height, width) to rescale box predictions [batch_size, 2]
>>> target_sizes = torch.tensor([(image.height, image.width)])
>>> # Convert outputs (bounding boxes and class logits) to Pascal VOC format (xmin, ymin, xmax, ymax)
>>> results = processor.post_process_grounded_object_detection(
...     outputs=outputs, target_sizes=target_sizes, threshold=0.1, text_labels=text_labels
... )
>>> # Retrieve predictions for the first image for the corresponding text queries
>>> result = results[0]
>>> boxes, scores, text_labels = result["boxes"], result["scores"], result["text_labels"]
>>> for box, score, text_label in zip(boxes, scores, text_labels):
...     box = [round(i, 2) for i in box.tolist()]
...     print(f"Detected {text_label} with confidence {round(score.item(), 3)} at location {box}")
Detected a photo of a cat with confidence 0.614 at location [341.67, 23.39, 642.32, 371.35]
Detected a photo of a cat with confidence 0.665 at location [6.75, 51.96, 326.62, 473.13]
```

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Owlv2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ImageProcessor). See `Owlv2ImageProcessor.__call__()` for details ([Owlv2Processor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2Processor) uses
  [Owlv2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ImageProcessor) for processing images).
- **query_pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) --
  Pixel values of query image(s) to be detected. Pass in one query image per target image.
- **interpolate_pos_encoding** (`bool`, *optional*, defaults to `False`) --
  Whether to interpolate the pre-trained position encodings.`Owlv2ImageGuidedObjectDetectionOutput` or `tuple(torch.FloatTensor)`A `Owlv2ImageGuidedObjectDetectionOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Owlv2Config](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2Config)) and inputs.

- **logits** (`torch.FloatTensor` of shape `(batch_size, num_patches, num_queries)`) -- Classification logits (including no-object) for all queries.
- **image_embeds** (`torch.FloatTensor` of shape `(batch_size, patch_size, patch_size, output_dim`) -- Pooled output of [Owlv2VisionModel](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2VisionModel). OWLv2 represents images as a set of image patches and computes
  image embeddings for each patch.
- **query_image_embeds** (`torch.FloatTensor` of shape `(batch_size, patch_size, patch_size, output_dim`) -- Pooled output of [Owlv2VisionModel](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2VisionModel). OWLv2 represents images as a set of image patches and computes
  image embeddings for each patch.
- **target_pred_boxes** (`torch.FloatTensor` of shape `(batch_size, num_patches, 4)`) -- Normalized boxes coordinates for all queries, represented as (center_x, center_y, width, height). These
  values are normalized in [0, 1], relative to the size of each individual target image in the batch
  (disregarding possible padding). You can use [post_process_object_detection()](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ImageProcessor.post_process_object_detection) to
  retrieve the unnormalized bounding boxes.
- **query_pred_boxes** (`torch.FloatTensor` of shape `(batch_size, num_patches, 4)`) -- Normalized boxes coordinates for all queries, represented as (center_x, center_y, width, height). These
  values are normalized in [0, 1], relative to the size of each individual query image in the batch
  (disregarding possible padding). You can use [post_process_object_detection()](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2ImageProcessor.post_process_object_detection) to
  retrieve the unnormalized bounding boxes.
- **class_embeds** (`torch.FloatTensor` of shape `(batch_size, num_patches, hidden_size)`) -- Class embeddings of all image patches. OWLv2 represents images as a set of image patches where the total
  number of patches is (image_size / patch_size)**2.
- **text_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The output of the [Owlv2TextModel](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2TextModel).
- **vision_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The output of the [Owlv2VisionModel](/docs/transformers/v5.14.0/en/model_doc/owlv2#transformers.Owlv2VisionModel).

Examples:
```python
>>> import httpx
>>> from io import BytesIO
>>> from PIL import Image
>>> import torch
>>> from transformers import AutoProcessor, Owlv2ForObjectDetection

>>> processor = AutoProcessor.from_pretrained("google/owlv2-base-patch16-ensemble")
>>> model = Owlv2ForObjectDetection.from_pretrained("google/owlv2-base-patch16-ensemble")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))
>>> query_url = "http://images.cocodataset.org/val2017/000000001675.jpg"
>>> with httpx.stream("GET", query_url) as response:
...     query_image = Image.open(BytesIO(response.read()))
>>> inputs = processor(images=image, query_images=query_image, return_tensors="pt")

>>> # forward pass
>>> with torch.no_grad():
...     outputs = model.image_guided_detection(**inputs)

>>> target_sizes = torch.Tensor([image.size[::-1]])

>>> # Convert outputs (bounding boxes and class logits) to Pascal VOC format (xmin, ymin, xmax, ymax)
>>> results = processor.post_process_image_guided_detection(
...     outputs=outputs, threshold=0.9, nms_threshold=0.3, target_sizes=target_sizes
... )
>>> i = 0  # Retrieve predictions for the first image
>>> boxes, scores = results[i]["boxes"], results[i]["scores"]
>>> for box, score in zip(boxes, scores):
...     box = [round(i, 2) for i in box.tolist()]
...     print(f"Detected similar object with confidence {round(score.item(), 3)} at location {box}")
Detected similar object with confidence 0.938 at location [327.31, 54.94, 547.39, 268.06]
Detected similar object with confidence 0.959 at location [5.78, 360.65, 619.12, 366.39]
Detected similar object with confidence 0.902 at location [2.85, 360.01, 627.63, 380.8]
Detected similar object with confidence 0.985 at location [176.98, -29.45, 672.69, 182.83]
Detected similar object with confidence 1.0 at location [6.53, 14.35, 624.87, 470.82]
Detected similar object with confidence 0.998 at location [579.98, 29.14, 615.49, 489.05]
Detected similar object with confidence 0.985 at location [206.15, 10.53, 247.74, 466.01]
Detected similar object with confidence 0.947 at location [18.62, 429.72, 646.5, 457.72]
Detected similar object with confidence 0.996 at location [523.88, 20.69, 586.84, 483.18]
Detected similar object with confidence 0.998 at location [3.39, 360.59, 617.29, 499.21]
Detected similar object with confidence 0.969 at location [4.47, 449.05, 614.5, 474.76]
Detected similar object with confidence 0.966 at location [31.44, 463.65, 654.66, 471.07]
Detected similar object with confidence 0.924 at location [30.93, 468.07, 635.35, 475.39]
```
