# SLANeXt

## Overview

**SLANeXt_wired** and **SLANeXt_wireless** are part of a series of dedicated lightweight models for table structure recognition, focusing on accurately recognizing table structures in documents and natural scenes. For more details about the SLANeXt series model, please refer to the [official documentation](https://www.paddleocr.ai/latest/en/version3.x/module_usage/table_structure_recognition.html).

## Model Architecture

The SLANeXt series is a new generation of table structure recognition models independently developed by the Baidu PaddlePaddle Vision Team. SLANeXt focuses on table structure recognition, and trains dedicated weights for wired and wireless tables separately. The recognition ability for all types of tables has been significantly improved, especially for wired tables.

## Usage

### Single input inference

The example below demonstrates how to detect text with PP-OCRV5_Mobile_Det using the [AutoModel](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel).

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

#### transformers.SLANeXtConfig[[transformers.SLANeXtConfig]]

```python
transformers.SLANeXtConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vision_config: dict | transformers.models.slanext.configuration_slanext.SLANeXtVisionConfig | None = None, post_conv_in_channels: int = 256, post_conv_out_channels: int = 512, out_channels: int = 50, hidden_size: int = 512, max_text_length: int = 500)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/slanext/configuration_slanext.py#L68)

**Parameters:**

vision_config (`dict` or `SLANeXtVisionConfig`, *optional*) : Configuration for the vision encoder. If `None`, a default `SLANeXtVisionConfig` is used.

post_conv_in_channels (`int`, *optional*, defaults to 256) : Number of input channels for the post-encoder convolution layer.

post_conv_out_channels (`int`, *optional*, defaults to 512) : Number of output channels for the post-encoder convolution layer.

out_channels (`int`, *optional*, defaults to 50) : Vocabulary size for the table structure token prediction head, i.e., the number of distinct structure tokens the model can predict.

hidden_size (`int`, *optional*, defaults to 512) : Dimensionality of the hidden states in the attention GRU cell and the structure/location prediction heads.

max_text_length (`int`, *optional*, defaults to 500) : Maximum number of autoregressive decoding steps (tokens) for the structure and location decoder.

This is the configuration class to store the configuration of a SlanextModel. It is used to instantiate a Slanext
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/SLANeXt_wired_safetensors](https://huggingface.co/PaddlePaddle/SLANeXt_wired_safetensors)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## SLANeXtForTableRecognition[[transformers.SLANeXtForTableRecognition]]

#### transformers.SLANeXtForTableRecognition[[transformers.SLANeXtForTableRecognition]]

```python
transformers.SLANeXtForTableRecognition(config: SLANeXtConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/slanext/modeling_slanext.py#L620)

**Parameters:**

config ([SLANeXtConfig](/docs/transformers/v5.15.1/en/model_doc/slanext#transformers.SLANeXtConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

SLANeXt Table Recognition model for table recognition tasks. Wraps the core SLANeXtPreTrainedModel
and returns outputs compatible with the Transformers table recognition API.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.SLANeXtForTableRecognition.forward]]

```python
forward(pixel_values: FloatTensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/slanext/modeling_slanext.py#L627)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [SLANeXtImageProcessor](/docs/transformers/v5.15.1/en/model_doc/slanext#transformers.SLANeXtImageProcessor). See `SLANeXtImageProcessor.__call__()` for details (`processor_class` uses [SLANeXtImageProcessor](/docs/transformers/v5.15.1/en/model_doc/slanext#transformers.SLANeXtImageProcessor) for processing images).

**Returns:** `SLANeXtForTableRecognitionOutput` or `tuple(torch.FloatTensor)`

A `SLANeXtForTableRecognitionOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SLANeXtConfig](/docs/transformers/v5.15.1/en/model_doc/slanext#transformers.SLANeXtConfig)) and inputs.

The [SLANeXtForTableRecognition](/docs/transformers/v5.15.1/en/model_doc/slanext#transformers.SLANeXtForTableRecognition) forward method, overrides the `__call__` special method.

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

#### transformers.SLANeXtBackbone[[transformers.SLANeXtBackbone]]

```python
transformers.SLANeXtBackbone(config: dict | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/slanext/modeling_slanext.py#L524)

## SLANeXtSLAHead[[transformers.SLANeXtSLAHead]]

#### transformers.SLANeXtSLAHead[[transformers.SLANeXtSLAHead]]

```python
transformers.SLANeXtSLAHead(config: dict | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/slanext/modeling_slanext.py#L548)

## SLANeXtImageProcessor[[transformers.SLANeXtImageProcessor]]

#### transformers.SLANeXtImageProcessor[[transformers.SLANeXtImageProcessor]]

```python
transformers.SLANeXtImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/slanext/image_processing_slanext.py#L40)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 512, 'width': 512}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `2`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.485, 0.456, 0.406]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.229, 0.224, 0.225]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 512, 'width': 512}`): The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

Constructs a SLANeXtImageProcessor image processor.

#### init_decoder[[transformers.SLANeXtImageProcessor.init_decoder]]

```python
init_decoder()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/slanext/image_processing_slanext.py#L172)

Initialize the decoder vocabulary for table structure recognition.

Builds a character dictionary mapping HTML table structure tokens (e.g., `<thead>`, `<tr>`, `<td>`, colspan/
rowspan attributes) to integer indices. The dictionary includes special `"sos"` (start-of-sequence) and
`"eos"` (end-of-sequence) tokens. Merged `<td></td>` tokens are used in place of standalone `<td>` tokens
when applicable.

#### post_process_table_recognition[[transformers.SLANeXtImageProcessor.post_process_table_recognition]]

```python
post_process_table_recognition(outputs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/slanext/image_processing_slanext.py#L208)

**Parameters:**

outputs (`SLANeXtForTableRecognitionOutput`) : Raw outputs from the SLANeXt model. The `last_hidden_state` field contains the predicted probability distributions over the structure vocabulary at each decoding step, with shape `(batch_size, max_text_length, num_classes)`.

**Returns:** `dict`

A dictionary containing:
- **structure** (`list[str]`): The predicted HTML table structure as a list of tokens, wrapped with
  `<html>`, `<body>`, and `<table>` tags.
- **structure_score** (`float`): The mean confidence score across all predicted tokens.

Post-process the raw model outputs to decode the predicted table structure into an HTML token sequence.

Converts the model's predicted probability distributions over the structure vocabulary into a sequence of
HTML tokens representing the table structure. The decoded tokens are wrapped with `<html>`, `<body>`, and
`<table>` tags to form a complete HTML table structure.
