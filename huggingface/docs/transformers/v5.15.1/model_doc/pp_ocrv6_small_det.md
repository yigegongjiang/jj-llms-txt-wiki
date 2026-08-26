# PP-OCRv6_small_det

## Overview

PP-OCRv6_small_det is the small model in the PP-OCRv6 detection series developed by the PaddleOCR team. It uses LCNetV4 as the backbone and RepLKFPN as the feature pyramid neck, providing accurate text localization across diverse scenarios including handwritten, printed, rotated, curved, and artistic text in multiple languages. The model contains 2.48M parameters.

## Model Architecture

## Usage

### Single input inference

The example below demonstrates how to detect text with PP-OCRv6_small_det using the [AutoModel](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel).

```python
from io import BytesIO

import httpx
from PIL import Image
from transformers import AutoImageProcessor, AutoModelForObjectDetection
from transformers.image_utils import load_image

model_path = "PaddlePaddle/PP-OCRv6_small_det_safetensors" # or "PaddlePaddle/PP-OCRv6_tiny_det_safetensors"
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

Here is how you can do it with PP-OCRv6_small_det using the [AutoModel](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel):

```python
from io import BytesIO

import httpx
from PIL import Image
from transformers import AutoImageProcessor, AutoModelForObjectDetection
from transformers.image_utils import load_image

model_path = "PaddlePaddle/PP-OCRv6_small_det_safetensors" # or "PaddlePaddle/PP-OCRv6_tiny_det_safetensors"
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

## PPOCRV6SmallDetForObjectDetection[[transformers.PPOCRV6SmallDetForObjectDetection]]

#### transformers.PPOCRV6SmallDetForObjectDetection[[transformers.PPOCRV6SmallDetForObjectDetection]]

```python
transformers.PPOCRV6SmallDetForObjectDetection(config: PPOCRV6SmallDetConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_ocrv6_small_det/modeling_pp_ocrv6_small_det.py#L314)

**Parameters:**

config ([PPOCRV6SmallDetConfig](/docs/transformers/v5.15.1/en/model_doc/pp_ocrv6_small_det#transformers.PPOCRV6SmallDetConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

PPOCR6SmallRec model for text recognition tasks.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

## PPOCRV6SmallDetConfig[[transformers.PPOCRV6SmallDetConfig]]

#### transformers.PPOCRV6SmallDetConfig[[transformers.PPOCRV6SmallDetConfig]]

```python
transformers.PPOCRV6SmallDetConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, backbone_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, reduction: int = 4, neck_out_channels: int = 96, interpolate_mode: str = 'nearest', kernel_list: list[int] | tuple[int, ...] = (3, 2, 2), layer_list_out_channels: list[int] | tuple[int, ...] = (12, 18, 42, 360), dilated_kernel_size: int = 7)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_ocrv6_small_det/configuration_pp_ocrv6_small_det.py#L31)

**Parameters:**

backbone_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The configuration of the backbone model.

reduction (`int`, *optional*, defaults to 4) : The reduction factor for feature channel dimensions, used to reduce the number of model parameters and computational complexity while maintaining feature representability.

neck_out_channels (`int`, *optional*, defaults to 96) : The number of output channels from the neck network, which is responsible for feature fusion and refinement before passing features to the head network.

interpolate_mode (`str`, *optional*, defaults to `"nearest"`) : The interpolation mode used for upsampling or downsampling feature maps in the neck network. Supported modes include `"nearest"` (nearest neighbor interpolation) and `"bilinear"`.

kernel_list (`List[int]`, *optional*, defaults to `[3, 2, 2]`) : The list of kernel sizes for convolutional layers in the head network, used for multi-scale feature extraction to detect text regions of different sizes.

layer_list_out_channels (`List[int]`, *optional*, defaults to `[12, 18, 42, 360]`) : The list of output channels for each backbone stage, used to configure the input channels of the RSE layers in the neck network for multi-scale feature fusion.

dilated_kernel_size (`int`, *optional*, defaults to 7) : The kernel size of the dilated convolutional layer in the input conv path, used for capturing long-range dependencies in the feature maps.

This is the configuration class to store the configuration of a PPOCRV6SmallDetModel. It is used to instantiate a Pp Ocrv6 Small Det
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/PP-OCRv6_small_det_safetensors](https://huggingface.co/PaddlePaddle/PP-OCRv6_small_det_safetensors)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## PPOCRV6SmallDetModel[[transformers.PPOCRV6SmallDetModel]]

#### transformers.PPOCRV6SmallDetModel[[transformers.PPOCRV6SmallDetModel]]

```python
transformers.PPOCRV6SmallDetModel(config: PPOCRV6SmallDetConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_ocrv6_small_det/modeling_pp_ocrv6_small_det.py#L289)

**Parameters:**

config ([PPOCRV6SmallDetConfig](/docs/transformers/v5.15.1/en/model_doc/pp_ocrv6_small_det#transformers.PPOCRV6SmallDetConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Pp Ocrv6 Small Det Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.PPOCRV6SmallDetModel.forward]]

```python
forward(pixel_values: FloatTensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_ocrv6_small_det/modeling_pp_ocrv6_small_det.py#L296)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [PPOCRV5ServerDetImageProcessor](/docs/transformers/v5.15.1/en/model_doc/pp_ocrv5_server_det#transformers.PPOCRV5ServerDetImageProcessor). See `PPOCRV5ServerDetImageProcessor.__call__()` for details (`processor_class` uses [PPOCRV5ServerDetImageProcessor](/docs/transformers/v5.15.1/en/model_doc/pp_ocrv5_server_det#transformers.PPOCRV5ServerDetImageProcessor) for processing images).

**Returns:** `BaseModelOutputWithNoAttention` or `tuple(torch.FloatTensor)`

A `BaseModelOutputWithNoAttention` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PPOCRV6SmallDetConfig](/docs/transformers/v5.15.1/en/model_doc/pp_ocrv6_small_det#transformers.PPOCRV6SmallDetConfig)) and inputs.

The [PPOCRV6SmallDetModel](/docs/transformers/v5.15.1/en/model_doc/pp_ocrv6_small_det#transformers.PPOCRV6SmallDetModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, num_channels, height, width)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
