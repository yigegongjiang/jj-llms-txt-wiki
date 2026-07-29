# PP-OCRv6_medium_det

## Overview

PP-OCRv6_medium_det is the largest model in the PP-OCRv6 detection series developed by the PaddleOCR team. It uses LCNetV4 as the backbone and RepLKFPN as the feature pyramid neck, providing accurate text localization across diverse scenarios including handwritten, printed, rotated, curved, and artistic text in multiple languages. The model contains 15.5M parameters.

## Model Architecture

## Usage

### Single input inference

The example below demonstrates how to detect text with PP-OCRv6_medium_det using the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel).

```python
from io import BytesIO

import httpx
from PIL import Image
from transformers import AutoImageProcessor, AutoModelForObjectDetection
from transformers.image_utils import load_image

model_path = "PaddlePaddle/PP-OCRv6_medium_det_safetensors"
model = AutoModelForObjectDetection.from_pretrained(model_path, device_map="auto")
image_processor = AutoImageProcessor.from_pretrained(model_path)

image_url = "https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/general_ocr_001.png"
image = load_image(image_url)
inputs = image_processor(images=image, return_tensors="pt").to(model.device)
outputs = model(**inputs)

results = image_processor.post_process_object_detection(
    outputs, 
    target_sizes=inputs["target_sizes"],
    threshold=0.2,
    box_threshold=0.45,
    max_candidates=3000,
    unclip_ratio=1.4,
)

for result in results:
    print(result)
```

### Batched inference

Here is how you can do it with PP-OCRv6_medium_det using the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel):

```python
from io import BytesIO

import httpx
from PIL import Image
from transformers import AutoImageProcessor, AutoModelForObjectDetection
from transformers.image_utils import load_image

model_path = "PaddlePaddle/PP-OCRv6_medium_det_safetensors"
model = AutoModelForObjectDetection.from_pretrained(model_path, device_map="auto")
image_processor = AutoImageProcessor.from_pretrained(model_path)

image_url = "https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/general_ocr_001.png"
image = load_image(image_url)
inputs = image_processor(images=[image, image], return_tensors="pt").to(model.device)
outputs = model(**inputs)

results = image_processor.post_process_object_detection(
    outputs, 
    target_sizes=inputs["target_sizes"],
    threshold=0.2,
    box_threshold=0.45,
    max_candidates=3000,
    unclip_ratio=1.4,
)

for result in results:
    print(result)
```

## PPOCRV6MediumDetForObjectDetection[[transformers.PPOCRV6MediumDetForObjectDetection]]

- **config** ([PPOCRV6MediumDetConfig](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv6_medium_det#transformers.PPOCRV6MediumDetConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
PPOCRV6MediumDet model for text detection tasks.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

## PPOCRV6MediumDetConfig[[transformers.PPOCRV6MediumDetConfig]]

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
- **kernel_list** (`list[int]`, *optional*, defaults to `[3, 2, 2]`) --
  The list of kernel sizes for convolutional layers in the head network for multi-scale feature extraction.

This is the configuration class to store the configuration of a PPOCRV6MediumDetModel. It is used to instantiate a Pp Ocrv6 Medium Det
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/PP-OCRv6_medium_det_safetensors](https://huggingface.co/PaddlePaddle/PP-OCRv6_medium_det_safetensors)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## PPOCRV6MediumDetModel[[transformers.PPOCRV6MediumDetModel]]

- **config** ([PPOCRV6MediumDetConfig](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv6_medium_det#transformers.PPOCRV6MediumDetConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Pp Ocrv6 Medium Det Model outputting raw hidden-states without any specific head on top.

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
elements depending on the configuration ([PPOCRV6MediumDetConfig](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv6_medium_det#transformers.PPOCRV6MediumDetConfig)) and inputs.
The [PPOCRV6MediumDetModel](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv6_medium_det#transformers.PPOCRV6MediumDetModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, num_channels, height, width)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
