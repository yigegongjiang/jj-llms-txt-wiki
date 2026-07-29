# SLANet

## Overview

**SLANet** and **SLANet_plus** are part of a series of dedicated lightweight models for table structure recognition, focusing on accurately recognizing table structures in documents and natural scenes. For more details about the SLANet series model, please refer to the [official documentation](https://www.paddleocr.ai/latest/en/version3.x/module_usage/table_structure_recognition.html).

## Model Architecture

SLANet is a table structure recognition model developed by Baidu PaddlePaddle Vision Team. The model significantly improves the accuracy and inference speed of table structure recognition by adopting a CPU-friendly lightweight backbone network PP-LCNet, a high-low-level feature fusion module CSP-PAN, and a feature decoding module SLA Head that aligns structural and positional information.

## Usage

### Single input inference

The example below demonstrates how to detect text with SLANet using the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel).

```python
from io import BytesIO

import httpx
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForTableRecognition

model_path="PaddlePaddle/SLANet_plus_safetensors"
model = AutoModelForTableRecognition.from_pretrained(model_path, device_map="auto")
image_processor = AutoImageProcessor.from_pretrained(model_path)

image = Image.open(BytesIO(httpx.get(image_url).content))
inputs = image_processor(images=image, return_tensors="pt").to(model.device)
outputs = model(**inputs)

results = image_processor.post_process_table_recognition(outputs)

print(result['structure'])
print(result['structure_score'])
```

## SLANetConfig[[transformers.SLANetConfig]]

- **post_conv_out_channels** (`int`, *optional*, defaults to 96) --
  Number of output channels for the post-encoder convolution layer.
- **out_channels** (`int`, *optional*, defaults to 50) --
  Vocabulary size for the table structure token prediction head, i.e., the number of distinct structure
  tokens the model can predict.
- **hidden_size** (`int`, *optional*, defaults to 256) --
  Dimensionality of the hidden states in the attention GRU cell and the structure/location prediction heads.
- **max_text_length** (`int`, *optional*, defaults to 500) --
  Maximum number of autoregressive decoding steps (tokens) for the structure and location decoder.
- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **hidden_act** (`str`, *optional*, defaults to `hardswish`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **csp_kernel_size** (`int`, *optional*, defaults to 5) --
  The kernel size of the Cross Stage Partial (CSP) layer.
- **csp_num_blocks** (`int`, *optional*, defaults to 1) --
  Number of blocks within the Cross Stage Partial (CSP) layer.

This is the configuration class to store the configuration of a SlanetModel. It is used to instantiate a Slanet
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/SLANet_plus_safetensors](https://huggingface.co/PaddlePaddle/SLANet_plus_safetensors)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## SLANetForTableRecognition[[transformers.SLANetForTableRecognition]]

- **config** ([SLANetConfig](/docs/transformers/v5.14.0/en/model_doc/slanet#transformers.SLANetConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

SLANet Table Recognition model for table recognition tasks. Wraps the core SLANetPreTrainedModel
and returns outputs compatible with the Transformers table recognition API.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [SLANeXtImageProcessor](/docs/transformers/v5.14.0/en/model_doc/slanext#transformers.SLANeXtImageProcessor). See `SLANeXtImageProcessor.__call__()` for details (`processor_class` uses
  [SLANeXtImageProcessor](/docs/transformers/v5.14.0/en/model_doc/slanext#transformers.SLANeXtImageProcessor) for processing images).`SLANetForTableRecognitionOutput` or `tuple(torch.FloatTensor)`A `SLANetForTableRecognitionOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SLANetConfig](/docs/transformers/v5.14.0/en/model_doc/slanet#transformers.SLANetConfig)) and inputs.
The [SLANetForTableRecognition](/docs/transformers/v5.14.0/en/model_doc/slanet#transformers.SLANetForTableRecognition) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, num_channels, height, width)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **head_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Hidden-states of the SLANetSLAHead at each prediction step, varies up to max `self.config.max_text_length` states (depending on early exits).
- **head_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Attentions of the SLANetSLAHead at each prediction step, varies up to max `self.config.max_text_length` attentions (depending on early exits).

## SLANetBackbone[[transformers.SLANetBackbone]]

- **hidden_states** (`torch.FloatTensor`) -- input to the layer of shape `(batch, seq_len, embed_dim)</paramsdesc><rettype>`BaseModelOutputWithNoAttention` or `tuple(torch.FloatTensor)`</rettype><retdesc>A `BaseModelOutputWithNoAttention` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SLANetConfig](/docs/transformers/v5.14.0/en/model_doc/slanet#transformers.SLANetConfig)) and inputs.
The [SLANetBackbone](/docs/transformers/v5.14.0/en/model_doc/slanet#transformers.SLANetBackbone) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, num_channels, height, width)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

## SLANetSLAHead[[transformers.SLANetSLAHead]]
