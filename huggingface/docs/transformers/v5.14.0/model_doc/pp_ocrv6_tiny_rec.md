# PP-OCRv6_tiny_rec

## Overview

PP-OCRv6_tiny_rec is the most lightweight recognition model in the PP-OCRv6 series. It uses LCNetV4 as the backbone with a direct reshape projection instead of an encoder neck, and a CTC+NRTR multi-head decoder. The model supports 49 languages and contains 1.1M parameters.

## Model Architecture

## Usage

### Single input inference

The example below demonstrates how to detect text with PP-OCRv6_tiny_rec using the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel).

```python
from io import BytesIO

import httpx
from PIL import Image
from transformers import AutoImageProcessor, AutoModelForTextRecognition
from transformers.image_utils import load_image

model_path = "PaddlePaddle/PP-OCRv6_tiny_rec_safetensors"
model = AutoModelForTextRecognition.from_pretrained(model_path, device_map="auto")
image_processor = AutoImageProcessor.from_pretrained(model_path)

image_url = "https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/general_ocr_rec_001.png"
image = load_image(image_url)
inputs = image_processor(images=image, return_tensors="pt").to(model.device)
outputs = model(**inputs)

results = image_processor.post_process_text_recognition(outputs)
for result in results:
    print(result)
```

### Batched inference

Here is how you can do it with PP-OCRv6_tiny_rec using the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel):

```python
from io import BytesIO

import httpx
from PIL import Image
from transformers import AutoImageProcessor, AutoModelForTextRecognition
from transformers.image_utils import load_image

model_path = "PaddlePaddle/PP-OCRv6_tiny_rec_safetensors"
model = AutoModelForTextRecognition.from_pretrained(model_path, device_map="auto")
image_processor = AutoImageProcessor.from_pretrained(model_path)

image_url = "https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/general_ocr_rec_001.png"
image = load_image(image_url)
inputs = image_processor(images=[image, image], return_tensors="pt").to(model.device)
outputs = model(**inputs)

results = image_processor.post_process_text_recognition(outputs)
for result in results:
    print(result)
```

## PPOCRV6TinyRecForTextRecognition[[transformers.PPOCRV6TinyRecForTextRecognition]]

- **config** ([PPOCRV6TinyRecConfig](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv6_tiny_rec#transformers.PPOCRV6TinyRecConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
PPOCR6TinyRec model for text recognition tasks.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [PPOCRV6SmallRecImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv6_tiny_rec#transformers.PPOCRV6SmallRecImageProcessor). See `PPOCRV6SmallRecImageProcessor.__call__()` for details (`processor_class` uses
  [PPOCRV6SmallRecImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv6_tiny_rec#transformers.PPOCRV6SmallRecImageProcessor) for processing images).`BaseModelOutputWithNoAttention` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithNoAttention` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PPOCRV6TinyRecConfig](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv6_tiny_rec#transformers.PPOCRV6TinyRecConfig)) and inputs.
The [PPOCRV6TinyRecForTextRecognition](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv6_tiny_rec#transformers.PPOCRV6TinyRecForTextRecognition) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, num_channels, height, width)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

## PPOCRV6TinyRecConfig[[transformers.PPOCRV6TinyRecConfig]]

- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **hidden_size** (`int`, *optional*, defaults to `120`) --
  Dimension of the hidden representations.
- **head_out_channels** (`int`, *optional*, defaults to 18714) --
  The number of output channels from the PPOCRV6TinyRecHead, responsible for final classification.

This is the configuration class to store the configuration of a PPOCRV6TinyRecModel. It is used to instantiate a Pp Ocrv6 Tiny Rec
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/PP-OCRv6_tiny_rec_safetensors](https://huggingface.co/PaddlePaddle/PP-OCRv6_tiny_rec_safetensors)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## PPOCRV6TinyRecModel[[transformers.PPOCRV6TinyRecModel]]

- **config** ([PPOCRV6TinyRecConfig](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv6_tiny_rec#transformers.PPOCRV6TinyRecConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
PPOCRV6TinyRec model, consisting of Backbone and Head networks.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [PPOCRV6SmallRecImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv6_tiny_rec#transformers.PPOCRV6SmallRecImageProcessor). See `PPOCRV6SmallRecImageProcessor.__call__()` for details (`processor_class` uses
  [PPOCRV6SmallRecImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv6_tiny_rec#transformers.PPOCRV6SmallRecImageProcessor) for processing images).`BaseModelOutputWithNoAttention` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithNoAttention` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PPOCRV6TinyRecConfig](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv6_tiny_rec#transformers.PPOCRV6TinyRecConfig)) and inputs.
The [PPOCRV6TinyRecModel](/docs/transformers/v5.14.0/en/model_doc/pp_ocrv6_tiny_rec#transformers.PPOCRV6TinyRecModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, num_channels, height, width)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

## PPOCRV6SmallRecImageProcessor[[transformers.PPOCRV6SmallRecImageProcessor]]

- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a PPOCRV6SmallRecImageProcessor image processor.

Calculate the width and height from the widest image in the batch.

- **predictions** -- Model outputs with `logits` attribute (probability maps of shape `(batch_size, height, vocab_size)`).A list of dictionaries, where each dictionary corresponds to an image in the batch.
Each dictionary contains:
- "text" (str): The decoded text string.
- "score" (float): The average confidence score of the characters in the decoded text.

Post-processes raw model logits to decode the recognized text and its confidence score.
