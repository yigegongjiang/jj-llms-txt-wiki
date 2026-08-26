# KimiK-2.5, KimiK-2.6, KimiK-2.7

This model class supports all three different releases: KimiK-2.5,KimiK-2.6, KimiK-2.7

## Overview

Kimi K2.5 is an open-source, native multimodal agentic model that advances practical capabilities in long-horizon coding, coding-driven design, proactive autonomous execution, and swarm-based task orchestration. The model was proposed in [Kimi K2.5: Visual Agentic Intelligence](https://www.kimi.com/en/blog/kimi-k2-5) and further improved in [Kimi K2.6: Advancing Open-Source Coding](Kimi K2.5: Visual Agentic Intelligence).

Kimi K2.5 achieves significant improvements on complex, end-to-end coding tasks, generalizing robustly across programming languages (Rust, Go, Python) and domains spanning front-end, DevOps, and performance optimization. The model is capable of transforming simple prompts and visual inputs into production-ready interfaces and lightweight full-stack workflows, generating structured layouts, interactive elements, and rich animations with deliberate aesthetic precision.

This model was contributed by [RaushanTurganbay](https://huggingface.co/RaushanTurganbay).
The official checkpoints are [moonshotai/Kimi-K2.5](https://huggingface.co/moonshotai/Kimi-K2.5), [moonshotai/Kimi-K2.6](https://huggingface.co/moonshotai/Kimi-K2.6) and [moonshotai/Kimi-K2.7-Code](https://huggingface.co/moonshotai/Kimi-K2.7-Code).

## Usage examples

Note that the repositories don't yet have the correct fast tokenizer uploaded. You can get the converted processor and tokenizer from [RaushanTurganbay/kimi2.7-processor](https://huggingface.co/RaushanTurganbay/kimi2.7-processor)

 

```python
import os
import torch
from transformers import AutoProcessor, AutoTokenizer, AutoModelForImageTextToText
from transformers.distributed.configuration_utils import DistributedConfig

distributed_config = DistributedConfig(enable_expert_parallel=True)

processor = AutoProcessor.from_pretrained('moonshotai/Kimi-K2.6')
model = AutoModelForImageTextToText.from_pretrained(
    'moonshotai/Kimi-K2.6',
    distributed_config=distributed_config,
)

messages = [
    {
        "role": "user",
        "content": [
            {"type": "image", "image": "https://www.ilankelman.org/stopsigns/australia.jpg"},
            {"type": "text", "text": "What is shown in this image?"},
        ],
    }
]

inputs = processor.apply_chat_template(
    messages,
    tokenize=True,
    add_generation_prompt=True,
    return_tensors="pt",
    return_dict=True,
).to(device=model.device, dtype=model.dtype)

generated_ids = model.generate(**inputs, max_new_tokens=64)
generated_text = processor.batch_decode(generated_ids[:, inputs["input_ids"].shape[-1]:], skip_special_tokens=True)[0]
print(generated_text)

```

## Kimi_K25ImageProcessor[[transformers.Kimi_K25ImageProcessor]]

#### transformers.Kimi_K25ImageProcessor[[transformers.Kimi_K25ImageProcessor]]

```python
transformers.Kimi_K25ImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kimi_k25/image_processing_kimi_k25.py#L86)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'max_height' : 512, 'max_width': 512}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BICUBIC`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.5, 0.5, 0.5]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.5, 0.5, 0.5]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

max_patches (`int`, *kwargs*, *optional*, defaults to `16384`) : The max limit to resize the image.

patch_size (`int`, *kwargs*, *optional*, defaults to 14) : The spatial patch size of the vision encoder.

Constructs a Kimi_K25ImageProcessor image processor.

#### get_number_of_image_patches[[transformers.Kimi_K25ImageProcessor.get_number_of_image_patches]]

```python
get_number_of_image_patches(height: int, width: int, images_kwargs: dict | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kimi_k25/image_processing_kimi_k25.py#L218)

**Parameters:**

height (`int`) : Height of the input image.

width (`int`) : Width of the input image.

images_kwargs (`dict`, *optional*) : Any kwargs to override defaults of the image processor.

**Returns:** `int`

Number of image patches per image.

A utility that returns number of image patches for a given image size.

Note: Do not remove this method! It is used by vLLM to infer the number of patches and placeholders
without an image input.

#### patchify[[transformers.Kimi_K25ImageProcessor.patchify]]

```python
patchify(images: torch.Tensor, patch_size: int)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kimi_k25/image_processing_kimi_k25.py#L124)

Patchifies each image into flat layout of shape (`seq_len`, `patch_dim`) so we can concat dynamically shaped pixels.

#### preprocess[[transformers.Kimi_K25ImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kimi_k25/image_processing_kimi_k25.py#L116)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

max_patches (`int`, *kwargs*, *optional*, defaults to `16384`) : The max limit to resize the image.

patch_size (`int`, *kwargs*, *optional*, defaults to 14) : The spatial patch size of the vision encoder.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## Kimi_K25Processor[[transformers.Kimi_K25Processor]]

#### transformers.Kimi_K25Processor[[transformers.Kimi_K25Processor]]

```python
transformers.Kimi_K25Processor(image_processor = None, tokenizer = None, video_processor = None, chat_template = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kimi_k25/processing_kimi_k25.py#L42)

**Parameters:**

image_processor (`Kimi_K25ImageProcessor`) : The image processor is a required input.

tokenizer (`TokenizersBackend`) : The tokenizer is a required input.

video_processor (`Kimi_K25VideoProcessor`) : The video processor is a required input.

chat_template (`str`) : A Jinja template to convert lists of messages in a chat into a tokenizable string.

Constructs a Kimi_K25Processor which wraps a image processor, a tokenizer, and a video processor into a single processor.

[Kimi_K25Processor](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25Processor) offers all the functionalities of [Kimi_K25ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25ImageProcessor), [TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend), and [Kimi_K25VideoProcessor](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25VideoProcessor). See the
[~Kimi_K25ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25ImageProcessor), [~TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend), and [~Kimi_K25VideoProcessor](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25VideoProcessor) for more information.

#### post_process_image_text_to_text[[transformers.Kimi_K25Processor.post_process_image_text_to_text]]

```python
post_process_image_text_to_text(generated_outputs, skip_special_tokens = True, clean_up_tokenization_spaces = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kimi_k25/processing_kimi_k25.py#L142)

**Parameters:**

generated_outputs (`torch.Tensor` or `np.ndarray`) : The output of the model `generate` function. The output is expected to be a tensor of shape `(batch_size, sequence_length)` or `(sequence_length,)`.

skip_special_tokens (`bool`, *optional*, defaults to `True`) : Whether or not to remove special tokens in the output. Argument passed to the tokenizer's `batch_decode` method.

clean_up_tokenization_spaces (`bool`, *optional*, defaults to `False`) : Whether or not to clean up the tokenization spaces. Argument passed to the tokenizer's `batch_decode` method.

- ****kwargs** : Additional arguments to be passed to the tokenizer's `batch_decode method`.

**Returns:** `list[str]`

The decoded text.

Post-process the output of the model to decode the text.

## Kimi_K25VideoProcessor[[transformers.Kimi_K25VideoProcessor]]

#### transformers.Kimi_K25VideoProcessor[[transformers.Kimi_K25VideoProcessor]]

```python
transformers.Kimi_K25VideoProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kimi_k25/video_processing_kimi_k25.py#L90)

#### patchify[[transformers.Kimi_K25VideoProcessor.patchify]]

```python
patchify(videos: torch.Tensor, patch_size: int)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kimi_k25/video_processing_kimi_k25.py#L123)

Patchifies each video into flat layout of shape (`seq_len`, `patch_dim`) so we can concat dynamically shaped pixels.

## Kimi_K25Config[[transformers.Kimi_K25Config]]

#### transformers.Kimi_K25Config[[transformers.Kimi_K25Config]]

```python
transformers.Kimi_K25Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, vision_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, projection_hidden_size: int | None = 1152, projection_layer_norm_eps: float = 1e-05, image_token_id: int = 163605, video_token_id: int = 163840, vision_start_token_id: int = 163602, vision_end_token_id: int = 163604, tie_word_embeddings: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kimi_k25/configuration_kimi_k25.py#L59)

**Parameters:**

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

vision_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the vision backbone.

projection_hidden_size (`int`, *optional*, defaults to `1152`) : The output hidden size for multimodal projector.

projection_layer_norm_eps (`float`, *optional*, defaults to `1e-5`) : Layer norm epsilon for projector.

image_token_id (`int`, *optional*, defaults to `163605`) : The image token index used as a placeholder for input images.

video_token_id (`int`, *optional*, defaults to `163840`) : The video token index used as a placeholder for input videos.

vision_start_token_id (`int`, *optional*, defaults to `163602`) : Token ID that marks the start of a visual segment in the multimodal input sequence.

vision_end_token_id (`int`, *optional*, defaults to `163604`) : Token ID that marks the end of a visual segment in the multimodal input sequence.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Kimi_K25Model. It is used to instantiate a Kimi K25
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [moonshotai/Kimi-K2.6](https://huggingface.co/moonshotai/Kimi-K2.6)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Kimi_K25VisionConfig[[transformers.Kimi_K25VisionConfig]]

#### transformers.Kimi_K25VisionConfig[[transformers.Kimi_K25VisionConfig]]

```python
transformers.Kimi_K25VisionConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, patch_size: int = 14, pos_emb_height: int = 64, pos_emb_width: int = 64, pos_emb_time: int = 4, num_attention_heads: int = 16, num_hidden_layers: int = 27, hidden_size: int = 1152, intermediate_size: int = 4304, hidden_act: str = 'gelu_pytorch_tanh', merge_kernel_size: tuple[int, int] | list[int] = (2, 2), rope_parameters: dict | None = None, max_position_embeddings: int | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kimi_k25/configuration_kimi_k25.py#L29)

**Parameters:**

patch_size (`int`, *optional*, defaults to `14`) : The size (resolution) of each patch.

pos_emb_height (`int`, *optional*) : Initial position embedding height.

pos_emb_width (`int`, *optional*) : Initial position embedding width.

pos_emb_time (`int`, *optional*) : Initial position embedding time dimension.

num_attention_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer decoder.

num_hidden_layers (`int`, *optional*, defaults to `27`) : Number of hidden layers in the Transformer decoder.

hidden_size (`int`, *optional*, defaults to `1152`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `4304`) : Dimension of the MLP representations.

hidden_act (`str`, *optional*, defaults to `gelu_pytorch_tanh`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

merge_kernel_size (`tuple[int] | list[int]`, *optional*) : Kernel size for patch merging.

rope_parameters (`dict`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

max_position_embeddings (`int`, *optional*) : The maximum sequence length that this model might ever be used with.

This is the configuration class to store the configuration of a Kimi_K25Model. It is used to instantiate a Kimi K25
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [moonshotai/Kimi-K2.6](https://huggingface.co/moonshotai/Kimi-K2.6)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Kimi_K25PreTrainedModel[[transformers.Kimi_K25PreTrainedModel]]

#### transformers.Kimi_K25PreTrainedModel[[transformers.Kimi_K25PreTrainedModel]]

```python
transformers.Kimi_K25PreTrainedModel(config: PreTrainedConfig, *inputs, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kimi_k25/modeling_kimi_k25.py#L430)

**Parameters:**

config ([PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Kimi_K25PreTrainedModel.forward]]

```python
forward(*args, **kwargs)
```

A mock value for a dotted path (e.g. `torch.float32`): attribute access chains,
calls behave as pass-through decorators, `repr` is the dotted path, and using it
as a base class substitutes a plain-`type` base (PEP 560 `__mro_entries__`), so
real subclasses keep a normal metaclass and `inspect.signature` reads their real
`__init__` instead of a mock's.

## Kimi_K25VisionModel[[transformers.Kimi_K25VisionModel]]

#### transformers.Kimi_K25VisionModel[[transformers.Kimi_K25VisionModel]]

```python
transformers.Kimi_K25VisionModel(config: Kimi_K25VisionConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kimi_k25/modeling_kimi_k25.py#L478)

#### forward[[transformers.Kimi_K25VisionModel.forward]]

```python
forward(pixel_values: Tensor, grid_thw: Tensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kimi_k25/modeling_kimi_k25.py#L524)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [Kimi_K25ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25ImageProcessor). See `Kimi_K25ImageProcessor.__call__()` for details ([Kimi_K25Processor](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25Processor) uses [Kimi_K25ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25ImageProcessor) for processing images).

grid_thw (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) : The temporal, height and width of feature shape of each image in LLM.

**Returns:**

`torch.Tensor`

The [Kimi_K25VisionModel](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25VisionModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

#### temporal_patch_merger[[transformers.Kimi_K25VisionModel.temporal_patch_merger]]

```python
temporal_patch_merger(hidden_states: Tensor, grid_thw: Tensor)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kimi_k25/modeling_kimi_k25.py#L496)

Merges temporal frames by spatially pooling patch embeddings across time, returning
`(total_merged_patches, kernel_height * kernel_width, hidden_dim)`.

Kept for backward compatibility: `forward` now pools with the export-friendly
`get_vision_temporal_merge_index` gather instead of this Python per-clip loop.

## Kimi_K25Model[[transformers.Kimi_K25Model]]

#### transformers.Kimi_K25Model[[transformers.Kimi_K25Model]]

```python
transformers.Kimi_K25Model(config: Kimi_K25Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kimi_k25/modeling_kimi_k25.py#L585)

#### forward[[transformers.Kimi_K25Model.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, pixel_values: typing.Optional[torch.Tensor] = None, image_grid_thw: typing.Optional[torch.LongTensor] = None, pixel_values_videos: typing.Optional[torch.Tensor] = None, video_grid_thw: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kimi_k25/modeling_kimi_k25.py#L669)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [Kimi_K25ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25ImageProcessor). See `Kimi_K25ImageProcessor.__call__()` for details ([Kimi_K25Processor](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25Processor) uses [Kimi_K25ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25ImageProcessor) for processing images).

image_grid_thw (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) : The temporal, height and width of feature shape of each image in LLM.

pixel_values_videos (`torch.Tensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`, *optional*) : The tensors corresponding to the input video. Pixel values for videos can be obtained using [Kimi_K25VideoProcessor](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25VideoProcessor). See `Kimi_K25VideoProcessor.__call__()` for details ([Kimi_K25Processor](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25Processor) uses [Kimi_K25VideoProcessor](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25VideoProcessor) for processing videos).

video_grid_thw (`torch.LongTensor` of shape `(num_videos, 3)`, *optional*) : The temporal, height and width of feature shape of each video in LLM.

**Returns:** `Kimi_K25ModelOutputWithPast` or `tuple(torch.FloatTensor)`

A `Kimi_K25ModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Kimi_K25Config](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25Config)) and inputs.

The [Kimi_K25Model](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25Model) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **image_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(batch_size, num_images, sequence_length, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.

## Kimi_K25ForConditionalGeneration[[transformers.Kimi_K25ForConditionalGeneration]]

#### transformers.Kimi_K25ForConditionalGeneration[[transformers.Kimi_K25ForConditionalGeneration]]

```python
transformers.Kimi_K25ForConditionalGeneration(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kimi_k25/modeling_kimi_k25.py#L724)

#### forward[[transformers.Kimi_K25ForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, pixel_values: typing.Optional[torch.Tensor] = None, image_grid_thw: typing.Optional[torch.LongTensor] = None, pixel_values_videos: typing.Optional[torch.Tensor] = None, video_grid_thw: typing.Optional[torch.LongTensor] = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kimi_k25/modeling_kimi_k25.py#L762)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [Kimi_K25ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25ImageProcessor). See `Kimi_K25ImageProcessor.__call__()` for details ([Kimi_K25Processor](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25Processor) uses [Kimi_K25ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25ImageProcessor) for processing images).

image_grid_thw (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) : The temporal, height and width of feature shape of each image in LLM.

pixel_values_videos (`torch.Tensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`, *optional*) : The tensors corresponding to the input video. Pixel values for videos can be obtained using [Kimi_K25VideoProcessor](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25VideoProcessor). See `Kimi_K25VideoProcessor.__call__()` for details ([Kimi_K25Processor](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25Processor) uses [Kimi_K25VideoProcessor](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25VideoProcessor) for processing videos).

video_grid_thw (`torch.LongTensor` of shape `(num_videos, 3)`, *optional*) : The temporal, height and width of feature shape of each video in LLM.

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** `Kimi_K25CausalLMOutputWithPast` or `tuple(torch.FloatTensor)`

A `Kimi_K25CausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Kimi_K25Config](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25Config)) and inputs.

The [Kimi_K25ForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25ForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **image_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(batch_size, num_images, sequence_length, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.

Example:

```python
>>> from transformers import AutoProcessor, Kimi_K25ForConditionalGeneration

>>> model = Kimi_K25ForConditionalGeneration.from_pretrained("moonshotai/Kimi-K2.6")
>>> processor = AutoProcessor.from_pretrained("moonshotai/Kimi-K2.6")

>>> messages = [
    {
        "role": "user",
        "content": [
            {
                "type": "image",
                "image": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg",
            },
            {"type": "text", "text": "Describe the image."},
        ],
    }
]

>>> inputs = processor.apply_chat_template(
    messages,
    tokenize=True,
    add_generation_prompt=True,
    return_dict=True,
    return_tensors="pt",
)

>>> # Generate
>>> generated_ids = model.generate(**inputs, max_new_tokens=1024)
>>> generated_ids_trimmed = [out_ids[len(in_ids) :] for in_ids, out_ids in zip(inputs.input_ids, generated_ids)]
>>> output_text = processor.batch_decode(generated_ids_trimmed, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
>>> print(output_text)
```

#### get_image_features[[transformers.Kimi_K25ForConditionalGeneration.get_image_features]]

```python
get_image_features(pixel_values: FloatTensor, image_grid_thw: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kimi_k25/modeling_kimi_k25.py#L749)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images.

image_grid_thw (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) : The temporal, height and width of feature shape of each image in LLM.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Kimi_K25Config](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25Config)) and inputs.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Last layer hidden-state of the first token of the sequence (classification token) after further processing
  through the layers used for the auxiliary pretraining task. E.g. for BERT-family of models, this returns
  the classification token after processing through a linear layer and a tanh activation function. The linear
  layer weights are trained from the next sentence prediction (classification) objective during pretraining.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from PIL import Image
>>> from transformers import AutoProcessor, Kimi_K25ForConditionalGeneration

>>> model = Kimi_K25ForConditionalGeneration.from_pretrained("moonshotai/Kimi-K2.6")
>>> processor = AutoProcessor.from_pretrained("moonshotai/Kimi-K2.6")

>>> messages = [
...     {
...         "role": "user", "content": [
...             {"type": "image", "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"},
...             {"type": "text", "text": "Where is the cat standing?"},
...         ]
...     },
... ]

>>> inputs = processor.apply_chat_template(
...     messages,
...     tokenize=True,
...     return_dict=True,
...     return_tensors="pt",
...     add_generation_prompt=True
... )
>>> # Generate
>>> generate_ids = model.generate(**inputs)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True)[0]
```

#### get_video_features[[transformers.Kimi_K25ForConditionalGeneration.get_video_features]]

```python
get_video_features(pixel_values_videos: FloatTensor, video_grid_thw: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kimi_k25/modeling_kimi_k25.py#L736)

**Parameters:**

pixel_values_videos (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input videos.

video_grid_thw (`torch.LongTensor` of shape `(num_videos, 3)`, *optional*) : The temporal, height and width of feature shape of each video in LLM.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Kimi_K25Config](/docs/transformers/v5.15.1/en/model_doc/kimi_k25#transformers.Kimi_K25Config)) and inputs.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Last layer hidden-state of the first token of the sequence (classification token) after further processing
  through the layers used for the auxiliary pretraining task. E.g. for BERT-family of models, this returns
  the classification token after processing through a linear layer and a tanh activation function. The linear
  layer weights are trained from the next sentence prediction (classification) objective during pretraining.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from PIL import Image
>>> from transformers import AutoProcessor, Kimi_K25ForConditionalGeneration

>>> model = Kimi_K25ForConditionalGeneration.from_pretrained("moonshotai/Kimi-K2.6")
>>> processor = AutoProcessor.from_pretrained("moonshotai/Kimi-K2.6")

>>> messages = [
...     {
...         "role": "user", "content": [
...             {"type": "image", "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"},
...             {"type": "text", "text": "Where is the cat standing?"},
...         ]
...     },
... ]

>>> inputs = processor.apply_chat_template(
...     messages,
...     tokenize=True,
...     return_dict=True,
...     return_tensors="pt",
...     add_generation_prompt=True
... )
>>> # Generate
>>> generate_ids = model.generate(**inputs)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True)[0]
```
