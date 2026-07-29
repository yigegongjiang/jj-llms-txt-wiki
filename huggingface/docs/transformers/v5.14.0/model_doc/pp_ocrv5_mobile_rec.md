# PP-OCRv5_mobile_rec

## Overview

**PP-OCRv5_mobile_rec** is a dedicated lightweight model for text recognition, focusing specifically on efficient recognition and understanding of text elements in multi-language documents and natural scenes.

## Model Architecture

PP-OCRv5_mobile_rec is one of the PP-OCRv5_rec series, the latest generation of text recognition models developed by the PaddleOCR team. It is designed to efficiently and accurately support the recognition of Simplified Chinese, Traditional Chinese, English, Japanese, as well as complex text scenarios such as handwriting, vertical text, pinyin, and rare characters with a single model. While maintaining recognition performance, it also balances inference speed and model robustness, providing efficient and accurate technical support for document understanding in various scenarios. 

## Usage

### Single input inference

The example below demonstrates how to detect text with PP-OCRv5_mobile_rec using the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel).

```python
import requests
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForTextRecognition

model_path="PaddlePaddle/PP-OCRv5_mobile_rec_safetensors"
model = AutoModelForTextRecognition.from_pretrained(model_path, device_map="auto")
image_processor = AutoImageProcessor.from_pretrained(model_path)

image = Image.open(requests.get("https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/general_ocr_rec_001.png", stream=True).raw).convert("RGB")
inputs = image_processor(images=image, return_tensors="pt").to(model.device)
outputs = model(**inputs)

results = image_processor.post_process_text_recognition(outputs)

for result in results:
    print(result)
```

### Batched inference

Here is how you can do it with PP-OCRv5_mobile_rec using the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel):

```python
import requests
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForTextRecognition

model_path = "PaddlePaddle/PP-OCRv5_mobile_rec_safetensors"
model = AutoModelForTextRecognition.from_pretrained(model_path, device_map="auto")
image_processor = AutoImageProcessor.from_pretrained(model_path)

image = Image.open(requests.get("https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/general_ocr_rec_001.png", stream=True).raw).convert("RGB")
inputs = image_processor(images=[image, image], return_tensors="pt").to(model.device)
outputs = model(**inputs)

results = image_processor.post_process_text_recognition(outputs)
for result in results:
    print(result)
```

## PPOCRV5MobileRecForTextRecognition[[transformers.PPOCRV5MobileRecForTextRecognition]]

- **config** ([PPOCRV5MobileRecConfig](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_mobile_rec#transformers.PPOCRV5MobileRecConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
PPOCRV5MobileRec model for text recognition tasks.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [PPOCRV5ServerRecImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_server_rec#transformers.PPOCRV5ServerRecImageProcessor). See `PPOCRV5ServerRecImageProcessor.__call__()` for details (`processor_class` uses
  [PPOCRV5ServerRecImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_server_rec#transformers.PPOCRV5ServerRecImageProcessor) for processing images).`BaseModelOutputWithNoAttention` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithNoAttention` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PPOCRV5MobileRecConfig](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_mobile_rec#transformers.PPOCRV5MobileRecConfig)) and inputs.
The [PPOCRV5MobileRecForTextRecognition](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_mobile_rec#transformers.PPOCRV5MobileRecForTextRecognition) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, num_channels, height, width)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

## PPOCRV5MobileRecConfig[[transformers.PPOCRV5MobileRecConfig]]

- **hidden_act** (`str`, *optional*, defaults to `silu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **hidden_size** (`int`, *optional*, defaults to `120`) --
  Dimension of the hidden representations.
- **mlp_ratio** (`float`, *optional*, defaults to `2.0`) --
  Ratio of the MLP hidden dim to the embedding dim.
- **depth** (`int`, *optional*, defaults to `2`) --
  Number of Transformer layers in the vision encoder.
- **head_out_channels** (`int`, *optional*, defaults to 18385) --
  The number of output channels from the PPOCRV5MobileRecHead, responsible for final classification.
- **conv_kernel_size** (`list`, *optional*) --
  The size of the convolutional kernel.
- **qkv_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to add a bias to the queries, keys and values.
- **num_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.

This is the configuration class to store the configuration of a PPOCRV5MobileRecModel. It is used to instantiate a Pp Ocrv5 Mobile Rec
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/PP-OCRv5_mobile_rec_safetensors](https://huggingface.co/PaddlePaddle/PP-OCRv5_mobile_rec_safetensors)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## PPOCRV5MobileRecModel[[transformers.PPOCRV5MobileRecModel]]

- **config** ([PPOCRV5MobileRecConfig](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_mobile_rec#transformers.PPOCRV5MobileRecConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
PPOCRV5MobileRec model, consisting of Backbone and Head networks.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [PPOCRV5ServerRecImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_server_rec#transformers.PPOCRV5ServerRecImageProcessor). See `PPOCRV5ServerRecImageProcessor.__call__()` for details (`processor_class` uses
  [PPOCRV5ServerRecImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_server_rec#transformers.PPOCRV5ServerRecImageProcessor) for processing images).`BaseModelOutputWithNoAttention` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithNoAttention` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PPOCRV5MobileRecConfig](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_mobile_rec#transformers.PPOCRV5MobileRecConfig)) and inputs.
The [PPOCRV5MobileRecModel](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_mobile_rec#transformers.PPOCRV5MobileRecModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, num_channels, height, width)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

## PPOCRV5MobileRecModel[[transformers.PPOCRV5MobileRecModel]]

- **config** ([PPOCRV5MobileRecConfig](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_mobile_rec#transformers.PPOCRV5MobileRecConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
PPOCRV5MobileRec model, consisting of Backbone and Head networks.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [PPOCRV5ServerRecImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_server_rec#transformers.PPOCRV5ServerRecImageProcessor). See `PPOCRV5ServerRecImageProcessor.__call__()` for details (`processor_class` uses
  [PPOCRV5ServerRecImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_server_rec#transformers.PPOCRV5ServerRecImageProcessor) for processing images).`BaseModelOutputWithNoAttention` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithNoAttention` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PPOCRV5MobileRecConfig](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_mobile_rec#transformers.PPOCRV5MobileRecConfig)) and inputs.
The [PPOCRV5MobileRecModel](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv5_mobile_rec#transformers.PPOCRV5MobileRecModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, num_channels, height, width)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
