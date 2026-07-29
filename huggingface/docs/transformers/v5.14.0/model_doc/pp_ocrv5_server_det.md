# PP-OCRv5_server_det

## Overview

**PP-OCRv5_server_det** is a high-performance text detection model optimized for server-side applications, focusing on accurate detection of multi-language text in documents and natural scenes.

## Model Architecture

PP-OCRv5_server_det is one of the PP-OCRv5_det series, the latest generation of text detection models developed by the PaddleOCR team. Designed for high-performance applications, it supports the detection of text in diverse scenarios—including handwriting, vertical, rotated, and curved text—across multiple languages such as Simplified Chinese, Traditional Chinese, English, and Japanese. Key features include robust handling of complex layouts, varying text sizes, and challenging backgrounds, making it suitable for practical applications like document analysis, license plate recognition, and scene text detection.

## Usage

### Single input inference

The example below demonstrates how to detect text with PP-OCRV5_Server_Det using [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel).

```python
import requests
from PIL import Image

from transformers import pipeline

image = Image.open(
    requests.get(
        "https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/general_ocr_001.png", stream=True
    ).raw)
detector = pipeline(
    task="object-detection",
    model="PaddlePaddle/PP-OCRV5_server_det_safetensors",
    device_map="auto",
)
results = detector(image)

for result in results:
    print(result)
```

```python
import requests
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForObjectDetection

model_path = "PaddlePaddle/PP-OCRV5_server_det_safetensors"
model = AutoModelForObjectDetection.from_pretrained(
    model_path,
    device_map="auto"
)
image_processor = AutoImageProcessor.from_pretrained(model_path)

image = Image.open(requests.get("https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/general_ocr_001.png", stream=True).raw).convert("RGB")
inputs = image_processor(images=image, return_tensors="pt").to(model.device)
outputs = model(**inputs)

results = image_processor.post_process_object_detection(outputs, target_sizes=inputs["target_sizes"])

for result in results:
    print(result)
```

### Batched inference

Here is how you can do it with PP-OCRV5_Server_Det using [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel).

```python
import requests
from PIL import Image

from transformers import pipeline

image = Image.open(
    requests.get(
        "https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/general_ocr_001.png", stream=True
    ).raw)
detector = pipeline(
    task="object-detection",
    model="PaddlePaddle/PP-OCRV5_server_det_safetensors",
    device_map="auto",
)
results = detector([image, image])

for result in results:
    print(result)
```

```python
import requests
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForObjectDetection

model_path = "PaddlePaddle/PP-OCRV5_server_det_safetensors"
model = AutoModelForObjectDetection.from_pretrained(
    model_path,
    device_map="auto",
)
image_processor = AutoImageProcessor.from_pretrained(model_path)

image = Image.open(requests.get("https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/general_ocr_001.png", stream=True).raw).convert("RGB")
inputs = image_processor(images=[image, image], return_tensors="pt").to(model.device)
outputs = model(**inputs)

results = image_processor.post_process_object_detection(outputs, target_sizes=inputs["target_sizes"])

for result in results:
    print(result)
```

## PPOCRV5ServerDetForObjectDetection[[transformers.PPOCRV5ServerDetForObjectDetection]]

- **config** ([PPOCRV5ServerDetConfig](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_server_det#transformers.PPOCRV5ServerDetConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

PPOCRV5 Server Det model for object (text) detection tasks. Wraps the core PPOCRV5ServerDetModel
and returns outputs compatible with the Transformers object detection API.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

## PPOCRV5ServerDetConfig[[transformers.PPOCRV5ServerDetConfig]]

- **id2label** (`Union[dict[int, str], dict[str, str]]`, *optional*) --
  A map from index (for instance prediction index, or target index) to label.
- **interpolate_mode** (`str`, *optional*, defaults to `"nearest"`) --
  The interpolation mode used for upsampling or downsampling feature maps in the neck network.
- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **neck_out_channels** (`int`, *optional*, defaults to 256) --
  The number of output channels from the neck network, responsible for feature fusion and refinement.
- **reduce_factor** (`int`, *optional*, defaults to 2) --
  The channel reduction factor used in the neck blocks to balance performance and complexity.
- **intraclass_block_number** (`int`, *optional*, defaults to 4) --
  The number of Intra-Class Block modules used for enhancing feature representation.
- **intraclass_block_config** (`dict`, *optional*, defaults to `None`) --
  Configuration for the Intra-Class Block modules, if any, used for enhancing feature representation.
- **scale_factor** (`int`, *optional*, defaults to 2) --
  The scaling factor used for spatial resolution adjustments in the feature maps.
- **scale_factor_list** (`list[int]`, *optional*, defaults to `None`) --
  A list of scaling factors used for spatial resolution adjustments in the feature maps.
- **hidden_act** (`str`, *optional*, defaults to `relu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **kernel_list** (`list[int]`, *optional*, defaults to `[3, 2, 2]`) --
  The list of kernel sizes for convolutional layers in the head network for multi-scale feature extraction.

This is the configuration class to store the configuration of a Pp Ocrv5 Server DetModel. It is used to instantiate a Pp Ocrv5 Server Det
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/PP-OCRv5_server_det_safetensors](https://huggingface.co/PaddlePaddle/PP-OCRv5_server_det_safetensors)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## PPOCRV5ServerDetModel[[transformers.PPOCRV5ServerDetModel]]

- **config** ([PPOCRV5ServerDetConfig](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_server_det#transformers.PPOCRV5ServerDetConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Pp Ocrv5 Server Det Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [PPOCRV5ServerDetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_server_det#transformers.PPOCRV5ServerDetImageProcessor). See `PPOCRV5ServerDetImageProcessor.__call__()` for details (`processor_class` uses
  [PPOCRV5ServerDetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_server_det#transformers.PPOCRV5ServerDetImageProcessor) for processing images).`BaseModelOutputWithNoAttention` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithNoAttention` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PPOCRV5ServerDetConfig](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_server_det#transformers.PPOCRV5ServerDetConfig)) and inputs.
The [PPOCRV5ServerDetModel](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_server_det#transformers.PPOCRV5ServerDetModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, num_channels, height, width)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

## PPOCRV5ServerDetImageProcessor[[transformers.PPOCRV5ServerDetImageProcessor]]

- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a PPOCRV5ServerDetImageProcessor image processor.

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

- **predictions** -- Model outputs with `logits` attribute (probability maps of shape `(batch_size, 1, H, W)`).
- **threshold** (float) -- Binarization threshold.
- **target_sizes** -- Original image sizes (height, width) per image.
- **box_threshold** (float) -- Box score threshold.
- **max_candidates** (int) -- Maximum number of boxes.
- **min_size** (int) -- Minimum box size.
- **unclip_ratio** (float) -- Expansion ratio.list[dict]List of detection results per image. Each dict contains:
- "boxes": `torch.Tensor` of shape `(N, 4)` in corners format (xmin, ymin, xmax, ymax)
- "scores": `torch.Tensor` of shape `(N,)`
- "labels": `torch.Tensor` of shape `(N,)` (class id 0 for text)

Converts model outputs into detected text boxes in corners format (xmin, ymin, xmax, ymax).
