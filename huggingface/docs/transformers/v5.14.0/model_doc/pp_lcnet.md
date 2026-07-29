# PP-LCNet

## Overview

**PP-LCNet** PP-LCNet is a family of efficient, lightweight convolutional neural networks designed for real-world document understanding and OCR tasks. It balances accuracy, speed, and model size, making it ideal for both server-side and edge deployment. To address different document processing requirements, PP-LCNet has three main variants, each optimized for a specific task.

## Model Architecture

1. The Document Image Orientation Classification Module is primarily designed to distinguish the orientation of document images and correct them through post-processing. During processes such as document scanning or ID photo capturing, the device might be rotated to achieve clearer images, resulting in images with various orientations. Standard OCR pipelines may not handle these images effectively. By leveraging image classification techniques, the orientation of documents or IDs containing text regions can be pre-determined and adjusted, thereby improving the accuracy of OCR processing.

2. The Table Classification Module is a key component in computer vision systems, responsible for classifying input table images. The performance of this module directly affects the accuracy and efficiency of the entire table recognition process. The Table Classification Module typically receives table images as input and, using deep learning algorithms, classifies them into predefined categories based on the characteristics and content of the images, such as wired and wireless tables. The classification results from the Table Classification Module serve as output for use in table recognition pipelines.

3. The text line orientation classification module primarily distinguishes the orientation of text lines and corrects them using post-processing. In processes such as document scanning and license/certificate photography, to capture clearer images, the capture device may be rotated, resulting in text lines in various orientations. Standard OCR pipelines cannot handle such data well. By utilizing image classification technology, the orientation of text lines can be predetermined and adjusted, thereby enhancing the accuracy of OCR processing.

## Usage

### Single input inference

The example below demonstrates how to classify image with PP-LCNet using [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel).

```python
import requests
from PIL import Image

from transformers import pipeline

model_path = "PaddlePaddle/PP-LCNet_x1_0_doc_ori_safetensors"
image_classifier = pipeline("image-classification", model=model_path, function_to_apply="none", device_map="auto")

image = Image.open(requests.get("https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/img_rot180_demo.jpg", stream=True).raw)
result = image_classifier(image)
print(result)
```

```python
import requests
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForImageClassification

model_path = "PaddlePaddle/PP-LCNet_x1_0_doc_ori_safetensors"
model = AutoModelForImageClassification.from_pretrained(model_path, device_map="auto")
image_processor = AutoImageProcessor.from_pretrained(model_path)

image = Image.open(requests.get("https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/img_rot180_demo.jpg", stream=True).raw)
inputs = image_processor(images=image, return_tensors="pt").to(model.device)
outputs = model(**inputs)
predicted_label = outputs.logits.argmax(-1).item()
print(model.config.id2label[predicted_label])
```

### Batched inference

Here is how you can do it with PP-LCNet using [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel):

```python
import requests
from PIL import Image

from transformers import pipeline

model_path = "PaddlePaddle/PP-LCNet_x1_0_doc_ori_safetensors"
image_classifier = pipeline("image-classification", model=model_path, function_to_apply="none", device_map="auto")

image = Image.open(requests.get("https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/img_rot180_demo.jpg", stream=True).raw)
result = image_classifier([image, image])
print(result)
```

```python
import requests
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForImageClassification

model_path = "PaddlePaddle/PP-LCNet_x1_0_doc_ori_safetensors"
model = AutoModelForImageClassification.from_pretrained(model_path, device_map="auto")
image_processor = AutoImageProcessor.from_pretrained(model_path)

image = Image.open(requests.get("https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/img_rot180_demo.jpg", stream=True).raw)
inputs = image_processor(images=[image, image], return_tensors="pt").to(model.device)
outputs = model(**inputs)

predicted_labels = outputs.logits.argmax(-1)

for label_id in predicted_labels:
    label_id_scalar = label_id.item()
    label = model.config.id2label[label_id_scalar]
    print(label)
```

## PPLCNetForImageClassification[[transformers.PPLCNetForImageClassification]]

- **config** ([PPLCNetConfig](/docs/transformers/v5.14.0/en/model_doc/pp_lcnet#transformers.PPLCNetConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Pp Lcnet Model with an image classification head on top e.g. for ImageNet.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [PPLCNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_lcnet#transformers.PPLCNetImageProcessor). See `PPLCNetImageProcessor.__call__()` for details (`processor_class` uses
  [PPLCNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_lcnet#transformers.PPLCNetImageProcessor) for processing images).`BaseModelOutputWithNoAttention` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithNoAttention` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PPLCNetConfig](/docs/transformers/v5.14.0/en/model_doc/pp_lcnet#transformers.PPLCNetConfig)) and inputs.
The [PPLCNetForImageClassification](/docs/transformers/v5.14.0/en/model_doc/pp_lcnet#transformers.PPLCNetForImageClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, num_channels, height, width)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

Examples:

```python
>>> import httpx
>>> from io import BytesIO
>>> from PIL import Image
>>> from transformers import AutoModelForImageClassification, AutoImageProcessor

>>> model_path = "PaddlePaddle/PP-LCNet_x1_0_table_cls_safetensors"
>>> model = AutoModelForImageClassification.from_pretrained(model_path)
>>> image_processor = AutoImageProcessor.from_pretrained(model_path)

>>> url = "https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/img_rot180_demo.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> inputs = image_processor(images=image, return_tensors="pt")
>>> outputs = model(**inputs)
>>> predicted_label = outputs.last_hidden_state.argmax(-1).item()
>>> print(model.config.id2label[predicted_label])
wireless_table
```

## PPLCNetConfig[[transformers.PPLCNetConfig]]

- **scale** (`float`, *optional*, defaults to 1.0) --
  The scaling factor for the model's channel dimensions, used to adjust the model size and computational cost
  without changing the overall architecture (e.g., 0.25, 0.5, 1.0, 1.5).
- **block_configs** (`list[list[tuple]]`, *optional*, defaults to `None`) --
  Configuration for each block in each stage. Each tuple contains:
  (kernel_size, in_channels, out_channels, stride, use_squeeze_excitation).
  If `None`, uses the default PP-LCNet configuration.
- **stem_channels** (`int`, *optional*, defaults to 16) --
  The number of output channels for the stem layer.
- **stem_stride** (`int`, *optional*, defaults to 2) --
  The stride for the stem convolution layer.
- **reduction** (`int`, *optional*, defaults to 4) --
  The reduction factor for feature channel dimensions in the squeeze-and-excitation (SE) blocks, used to
  reduce the number of model parameters and computational complexity while maintaining feature representability.
- **class_expand** (`int`, *optional*, defaults to 1280) --
  The number of hidden units in the expansion layer of the classification head, used to enhance the model's
  feature representation capability before the final classification layer.
- **divisor** (`int`, *optional*, defaults to 8) --
  The divisor used to ensure that various model parameters (e.g., channel dimensions, kernel sizes) are
  multiples of this value, promoting efficient model implementation and resource utilization.
- **hidden_act** (`str`, *optional*, defaults to `hardswish`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **hidden_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.2`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

This is the configuration class to store the configuration of a Pp LcnetModel. It is used to instantiate a Pp Lcnet
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/PP-LCNet_x1_0_doc_ori_safetensors](https://huggingface.co/PaddlePaddle/PP-LCNet_x1_0_doc_ori_safetensors)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## PPLCNetBackbone[[transformers.PPLCNetBackbone]]

- **config** ([PPLCNetConfig](/docs/transformers/v5.14.0/en/model_doc/pp_lcnet#transformers.PPLCNetConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

PPLCNet backbone model for feature extraction.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [PPLCNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_lcnet#transformers.PPLCNetImageProcessor). See `PPLCNetImageProcessor.__call__()` for details (`processor_class` uses
  [PPLCNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_lcnet#transformers.PPLCNetImageProcessor) for processing images).`BackboneOutput` or `tuple(torch.FloatTensor)`A `BackboneOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PPLCNetConfig](/docs/transformers/v5.14.0/en/model_doc/pp_lcnet#transformers.PPLCNetConfig)) and inputs.
The [PPLCNetBackbone](/docs/transformers/v5.14.0/en/model_doc/pp_lcnet#transformers.PPLCNetBackbone) forward method, overrides the `__call__` special method.

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
>>> from transformers import PPLCNetConfig, PPLCNetBackbone
>>> import torch

>>> config = PPLCNetConfig()
>>> model = PPLCNetBackbone(config)

>>> pixel_values = torch.randn(1, 3, 224, 224)

>>> with torch.no_grad():
...     outputs = model(pixel_values)

>>> feature_maps = outputs.feature_maps
>>> list(feature_maps[-1].shape)
```

## PPLCNetImageProcessor[[transformers.PPLCNetImageProcessor]]

- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a PPLCNetImageProcessor image processor.

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
