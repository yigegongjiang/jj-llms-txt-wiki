# PP-OCRv5_mobile_det

## Overview

**PP-OCRv5_mobile_det** is a dedicated lightweight model for text detection, focusing specifically on efficient detection and understanding of text elements in multi-language documents and natural scenes.

## Model Architecture

PP-OCRv5_mobile_det is one of the PP-OCRv5_det series, the latest generation of text detection models developed by the PaddleOCR team. It aims to efficiently and accurately supports the detection of text in diverse scenarios—including handwriting, vertical, rotated, and curved text—across multiple languages such as Simplified Chinese, Traditional Chinese, English, and Japanese. Key features include robust handling of complex layouts, varying text sizes, and challenging backgrounds, making it suitable for practical applications like document analysis, license plate recognition, and scene text detection. 

## Usage

### Single input inference

The example below demonstrates how to detect text with PP-OCRV5_Mobile_Det using the [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel).

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
    model="PaddlePaddle/PP-OCRV5_mobile_det_safetensors",
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

model_path="PaddlePaddle/PP-OCRv5_mobile_det_safetensors"
model = AutoModelForObjectDetection.from_pretrained(model_path, device_map="auto")
image_processor = AutoImageProcessor.from_pretrained(model_path)

image = Image.open(requests.get("https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/general_ocr_001.png", stream=True).raw).convert("RGB")
inputs = image_processor(images=image, return_tensors="pt").to(model.device)
outputs = model(**inputs)

results = image_processor.post_process_object_detection(outputs, target_sizes=inputs["target_sizes"])

for result in results:
    print(result["boxes"])
    print(result["scores"])
```

### Batched inference

Here is how you can do it with PP-OCRV5_Mobile_Det using the [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel):

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
    model="PaddlePaddle/PP-OCRV5_mobile_det_safetensors",
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

model_path="PaddlePaddle/PP-OCRv5_mobile_det_safetensors"
model = AutoModelForObjectDetection.from_pretrained(model_path, device_map="auto")
image_processor = AutoImageProcessor.from_pretrained(model_path)

image = Image.open(requests.get("https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/general_ocr_001.png", stream=True).raw).convert("RGB")
inputs = image_processor(images=[image, image], return_tensors="pt").to(model.device)
outputs = model(**inputs)

results = image_processor.post_process_object_detection(outputs, target_sizes=inputs["target_sizes"])

for result in results:
    print(result["boxes"])
    print(result["scores"])
```

## PPOCRV5MobileDetForObjectDetection[[transformers.PPOCRV5MobileDetForObjectDetection]]

- **config** ([PPOCRV5MobileDetConfig](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_mobile_det#transformers.PPOCRV5MobileDetConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

PPOCRV5 Mobile Det model for object (text) detection tasks. Wraps the core PPOCRV5MobileDetModel
and returns outputs compatible with the Transformers object detection API.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

## PPOCRV5MobileDetConfig[[transformers.PPOCRV5MobileDetConfig]]

- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **reduction** (`int`, *optional*, defaults to 4) --
  The reduction factor for feature channel dimensions, used to reduce the number of model parameters and
  computational complexity while maintaining feature representability.
- **neck_out_channels** (`int`, *optional*, defaults to 96) --
  The number of output channels from the neck network, which is responsible for feature fusion and
  refinement before passing features to the head network.
- **interpolate_mode** (`str`, *optional*, defaults to `"nearest"`) --
  The interpolation mode used for upsampling or downsampling feature maps in the neck network. Supported
  modes include `"nearest"` (nearest neighbor interpolation) and `"bilinear"`.
- **kernel_list** (`List[int]`, *optional*, defaults to `[3, 2, 2]`) --
  The list of kernel sizes for convolutional layers in the head network, used for multi-scale feature
  extraction to detect text regions of different sizes.
- **layer_list_out_channels** (`List[int]`, *optional*, defaults to `[12, 18, 42, 360]`) --
  The list of output channels for each backbone stage, used to configure the input channels of the RSE layers
  in the neck network for multi-scale feature fusion.

This is the configuration class to store the configuration of a Pp Ocrv5 Mobile DetModel. It is used to instantiate a Pp Ocrv5 Mobile Det
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/PP-OCRv5_mobile_det_safetensors](https://huggingface.co/PaddlePaddle/PP-OCRv5_mobile_det_safetensors)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## PPOCRV5MobileDetModel[[transformers.PPOCRV5MobileDetModel]]

- **config** ([PPOCRV5MobileDetConfig](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_mobile_det#transformers.PPOCRV5MobileDetConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Core PP-OCRv5_mobile_det, consisting of Backbone, Neck, and Head networks.
Generates binary text segmentation maps for text detection tasks.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.
