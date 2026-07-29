# SLANeXt

## Overview

**SLANeXt_wired** and **SLANeXt_wireless** are part of a series of dedicated lightweight models for table structure recognition, focusing on accurately recognizing table structures in documents and natural scenes. For more details about the SLANeXt series model, please refer to the [official documentation](https://www.paddleocr.ai/latest/en/version3.x/module_usage/table_structure_recognition.html).

## Model Architecture

The SLANeXt series is a new generation of table structure recognition models independently developed by the Baidu PaddlePaddle Vision Team. SLANeXt focuses on table structure recognition, and trains dedicated weights for wired and wireless tables separately. The recognition ability for all types of tables has been significantly improved, especially for wired tables.

## Usage

### Single input inference

The example below demonstrates how to detect text with PP-OCRV5_Mobile_Det using the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel).

```python
import requests
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForTableRecognition

model_path="PaddlePaddle/SLANeXt_wired_safetensors"
model = AutoModelForTableRecognition.from_pretrained(model_path, device_map="auto")
image_processor = AutoImageProcessor.from_pretrained(model_path)

image = Image.open(requests.get("https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/table_recognition.jpg", stream=True).raw)
inputs = image_processor(images=image, return_tensors="pt").to(model.device)
outputs = model(**inputs)

results = image_processor.post_process_table_recognition(outputs)

print(result['structure'])
print(result['structure_score'])
```

## SLANeXtConfig[[transformers.SLANeXtConfig]]

- **vision_config** (`dict` or `SLANeXtVisionConfig`, *optional*) --
  Configuration for the vision encoder. If `None`, a default `SLANeXtVisionConfig` is used.
- **post_conv_in_channels** (`int`, *optional*, defaults to 256) --
  Number of input channels for the post-encoder convolution layer.
- **post_conv_out_channels** (`int`, *optional*, defaults to 512) --
  Number of output channels for the post-encoder convolution layer.
- **out_channels** (`int`, *optional*, defaults to 50) --
  Vocabulary size for the table structure token prediction head, i.e., the number of distinct structure
  tokens the model can predict.
- **hidden_size** (`int`, *optional*, defaults to 512) --
  Dimensionality of the hidden states in the attention GRU cell and the structure/location prediction heads.
- **max_text_length** (`int`, *optional*, defaults to 500) --
  Maximum number of autoregressive decoding steps (tokens) for the structure and location decoder.

This is the configuration class to store the configuration of a SlanextModel. It is used to instantiate a Slanext
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/SLANeXt_wired_safetensors](https://huggingface.co/PaddlePaddle/SLANeXt_wired_safetensors)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## SLANeXtForTableRecognition[[transformers.SLANeXtForTableRecognition]]

- **config** ([SLANeXtConfig](/docs/transformers/v5.14.0/en/model_doc/slanext#transformers.SLANeXtConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

SLANeXt Table Recognition model for table recognition tasks. Wraps the core SLANeXtPreTrainedModel
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
  [SLANeXtImageProcessor](/docs/transformers/v5.14.0/en/model_doc/slanext#transformers.SLANeXtImageProcessor) for processing images).`SLANeXtForTableRecognitionOutput` or `tuple(torch.FloatTensor)`A `SLANeXtForTableRecognitionOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SLANeXtConfig](/docs/transformers/v5.14.0/en/model_doc/slanext#transformers.SLANeXtConfig)) and inputs.
The [SLANeXtForTableRecognition](/docs/transformers/v5.14.0/en/model_doc/slanext#transformers.SLANeXtForTableRecognition) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **head_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Hidden-states of the SLANeXtSLAHead at each prediction step, varies up to max `self.config.max_text_length` states (depending on early exits).
- **head_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Attentions of the SLANeXtSLAHead at each prediction step, varies up to max `self.config.max_text_length` attentions (depending on early exits).

## SLANeXtBackbone[[transformers.SLANeXtBackbone]]

## SLANeXtSLAHead[[transformers.SLANeXtSLAHead]]

## SLANeXtImageProcessor[[transformers.SLANeXtImageProcessor]]

- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a SLANeXtImageProcessor image processor.

Initialize the decoder vocabulary for table structure recognition.

Builds a character dictionary mapping HTML table structure tokens (e.g., `<thead>`, `<tr>`, `<td>`, colspan/
rowspan attributes) to integer indices. The dictionary includes special `"sos"` (start-of-sequence) and
`"eos"` (end-of-sequence) tokens. Merged `<td></td>` tokens are used in place of standalone `<td>` tokens
when applicable.

- **outputs** (`SLANeXtForTableRecognitionOutput`) --
  Raw outputs from the SLANeXt model. The `last_hidden_state` field contains the predicted probability
  distributions over the structure vocabulary at each decoding step, with shape
  `(batch_size, max_text_length, num_classes)`.`dict`A dictionary containing:
- **structure** (`list[str]`): The predicted HTML table structure as a list of tokens, wrapped with
  `<html>`, `<body>`, and `<table>` tags.
- **structure_score** (`float`): The mean confidence score across all predicted tokens.

Post-process the raw model outputs to decode the predicted table structure into an HTML token sequence.

Converts the model's predicted probability distributions over the structure vocabulary into a sequence of
HTML tokens representing the table structure. The decoded tokens are wrapped with `<html>`, `<body>`, and
`<table>` tags to form a complete HTML table structure.
