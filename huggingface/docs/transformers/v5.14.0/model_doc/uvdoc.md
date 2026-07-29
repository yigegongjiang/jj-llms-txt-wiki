# UVDoc

## Overview

**UVDoc** The main purpose of text image correction is to carry out geometric transformation on the image to correct the document distortion, inclination, perspective deformation and other problems in the image.

## Usage

### Single input inference

The example below demonstrates how to rectify a document image with UVDoc using the [AutoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoImageProcessor) and [UVDocModel](/docs/transformers/v5.14.0/en/model_doc/uvdoc#transformers.UVDocModel).

```python
import requests
from PIL import Image

from transformers import AutoImageProcessor, AutoModel

model_path = "PaddlePaddle/UVDoc_safetensors"
model = AutoModel.from_pretrained(
    model_path,
    device_map="auto",
)
image_processor = AutoImageProcessor.from_pretrained(model_path)

image = Image.open(requests.get("https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/doc_test.jpg", stream=True).raw)

inputs = image_processor(images=image, return_tensors="pt").to(model.device)
outputs = model(**inputs)

result = image_processor.post_process_document_rectification(outputs.last_hidden_state, inputs["original_images"])
print(result)
```

### Batched inference

Here is how to perform batched document rectification with UVDoc:

```py
import requests
from PIL import Image
from transformers import AutoImageProcessor, AutoModel

model_path = "PaddlePaddle/UVDoc_safetensors"
model = AutoModel.from_pretrained(
    model_path
    device_map="auto",
)
image_processor = AutoImageProcessor.from_pretrained(model_path)

image = Image.open(requests.get("https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/doc_test.jpg", stream=True).raw)

inputs = image_processor(images=[image, image], return_tensors="pt").to(model.device)
outputs = model(**inputs)

result = image_processor.post_process_document_rectification(outputs.last_hidden_state, inputs["original_images"])
print(result)
```

## UVDocConfig[[transformers.UVDocConfig]]

- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **hidden_act** (`str`, *optional*, defaults to `prelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **padding_mode** (`str`, *optional*, defaults to `"reflect"`) --
  Padding mode for convolutional layers. Supported modes are `"reflect"`, `"constant"`, and `"replicate"`.
- **kernel_size** (`int`, *optional*, defaults to 5) --
  Kernel size for convolutional layers in the backbone network.
- **bridge_connector** (`list[int] | tuple[int, ...]`, *optional*, defaults to `(128, 128)`) --
  Configuration for the bridge connector in format [in_channels, out_channels].
- **out_point_positions2D** (`Sequence[list[int] | tuple[int, ...]]`, *optional*, defaults to `((128, 32), (32, 2))`) --
  Configuration for the output point positions 2D layer in format [in_channels, out_channels].

This is the configuration class to store the configuration of a UVDocModel. It is used to instantiate a Uvdoc
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/UVDoc_safetensors](https://huggingface.co/PaddlePaddle/UVDoc_safetensors)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## UVDocModel[[transformers.UVDocModel]]

- **config** ([UVDocConfig](/docs/transformers/v5.14.0/en/model_doc/uvdoc#transformers.UVDocConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The model takes raw document images (pixel values) as input, processes them through the UVDoc backbone to predict spatial transformation parameters,
and outputs the rectified (corrected) document image tensor.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [UVDocImageProcessor](/docs/transformers/v5.14.0/en/model_doc/uvdoc#transformers.UVDocImageProcessor). See `UVDocImageProcessor.__call__()` for details (`processor_class` uses
  [UVDocImageProcessor](/docs/transformers/v5.14.0/en/model_doc/uvdoc#transformers.UVDocImageProcessor) for processing images).`BaseModelOutputWithNoAttention` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithNoAttention` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([UVDocConfig](/docs/transformers/v5.14.0/en/model_doc/uvdoc#transformers.UVDocConfig)) and inputs.
The [UVDocModel](/docs/transformers/v5.14.0/en/model_doc/uvdoc#transformers.UVDocModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, num_channels, height, width)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

## UVDocBackboneConfig[[transformers.UVDocBackboneConfig]]

- **resnet_head** (`Sequence[list[int] | tuple[int, ...]]`, *optional*, defaults to `((3, 32), (32, 32))`) --
  Configuration for the ResNet head layers in format [in_channels, out_channels].
- **resnet_configs** (`Sequence[Sequence[tuple[int, int, int, bool] | list[int | bool]]]`, *optional*, defaults to `(((32, 32, 1, False), --
  (32, 32, 3, False), (32, 32, 3, False)), ((32, 64, 1, True), (64, 64, 3, False), (64, 64, 3, False), (64, 64, 3, False)), ((64, 128, 1, True),
  (128, 128, 3, False), (128, 128, 3, False), (128, 128, 3, False), (128, 128, 3, False), (128, 128, 3, False)))`):
  Configuration for the ResNet stages in format [in_channels, out_channels, dilation_value, downsample].
- **stage_configs** (`Sequence[Sequence[tuple[int`, ...] | list[int]]], *optional*, defaults to `(((128, 1),), ((128, 2),), --
  ((128, 5),), ((128, 8),(128, 3),(128, 2),), ((128, 12), (128, 7), (128, 4),), ((128, 18), (128, 12), (128, 6),),)`):
  Configuration for the bridge module stages in format [in_channels, dilation_value].
  Each inner sequence corresponds to a single bridge block, and the outer sequence groups blocks by bridge stage.
- **kernel_size** (`int`, *optional*, defaults to `5`) --
  The size of the convolutional kernel.

This is the configuration class to store the configuration of a UVDocModel. It is used to instantiate a Uvdoc
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/UVDoc_safetensors](https://huggingface.co/PaddlePaddle/UVDoc_safetensors)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## UVDocBackbone[[transformers.UVDocBackbone]]

- **config** ([UVDocBackboneConfig](/docs/transformers/v5.14.0/en/model_doc/uvdoc#transformers.UVDocBackboneConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

UVDoc backbone model for feature extraction.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [UVDocImageProcessor](/docs/transformers/v5.14.0/en/model_doc/uvdoc#transformers.UVDocImageProcessor). See `UVDocImageProcessor.__call__()` for details (`processor_class` uses
  [UVDocImageProcessor](/docs/transformers/v5.14.0/en/model_doc/uvdoc#transformers.UVDocImageProcessor) for processing images).`BackboneOutput` or `tuple(torch.FloatTensor)`A `BackboneOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([UVDocConfig](/docs/transformers/v5.14.0/en/model_doc/uvdoc#transformers.UVDocConfig)) and inputs.
The [UVDocBackbone](/docs/transformers/v5.14.0/en/model_doc/uvdoc#transformers.UVDocBackbone) forward method, overrides the `__call__` special method.

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

## UVDocBridge[[transformers.UVDocBridge]]

## UVDocImageProcessor[[transformers.UVDocImageProcessor]]

- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a UVDocImageProcessor image processor.

)>"}, {"name": "original_images", "val": ": list"}, {"name": "scale", "val": ": float = 255.0"}]}>
- **prediction** -- Predicted 2D Bezier mesh coordinates, shape (B, 2, H, W)
- **original_images** -- List of original input tensors, each of shape (C, H_i, W_i). Images may have different sizes.
- **scale** -- Scaling factor for output images (default: 255.0)List of dictionaries containing rectified images. Each dictionary has- "images": Rectified image tensor of shape (H, W, 3) with dtype torch.uint8
  and BGR channel order (suitable for OpenCV visualization)

Post-process document rectification predictions to convert them into rectified images.
