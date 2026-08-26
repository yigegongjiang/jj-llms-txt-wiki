# LFM2-VL

## Overview

[LFM2-VL](https://www.liquid.ai/blog/lfm2-vl-efficient-vision-language-models) first series of vision-language foundation models developed by [Liquid AI](https://liquid.ai/). These multimodal models are designed for low-latency and device-aware deployment. LFM2-VL extends the LFM2 family of open-weight Liquid Foundation Models (LFMs) into the vision-language space, supporting both text and image inputs with variable resolutions.

## Architecture

LFM2-VL consists of three main components: a language model backbone, a vision encoder, and a multimodal projector. LFM2-VL builds upon the LFM2 backbone, inheriting from either LFM2-1.2B (for LFM2-VL-1.6B) or LFM2-350M (for LFM2-VL-450M). For the vision tower, LFM2-VL uses SigLIP2 NaFlex encoders to convert input images into token sequences. Two variants are implemented:

* Shape-optimized (400M) for more fine-grained vision capabilities for LFM2-VL-1.6B
* Base (86M) for fast image processing for LFM2-VL-450M

The encoder processes images at their native resolution up to 512×512 pixels, efficiently handling smaller images without upscaling and supporting non-standard aspect ratios without distortion. Larger images are split into non-overlapping square patches of 512×512 each, preserving detail. In LFM2-VL-1.6B, the model also receives a thumbnail (a small, downscaled version of the original image capturing the overall scene) to enhance global context understanding and alignment. Special tokens mark each patch’s position and indicate the thumbnail’s start. The multimodal connector is a 2-layer MLP connector with pixel unshuffle to reduce image token count.

## Example

The following example shows how to generate an answer using the `AutoModelForImageTextToText` class.

```python
from transformers import AutoModelForImageTextToText, AutoProcessor

\
# Load model and processor
model_id = "LiquidAI/LFM2-VL-1.6B"
model = AutoModelForImageTextToText.from_pretrained(
    model_id,
    device_map="auto",
    dtype="bfloat16",
)
processor = AutoProcessor.from_pretrained(model_id)

# Load image and create conversation
conversation = [
    {
        "role": "user",
        "content": [
            {"type": "image", "image": "https://www.ilankelman.org/stopsigns/australia.jpg"},
            {"type": "text", "text": "What is in this image?"},
        ],
    },
]

# Generate answer
inputs = processor.apply_chat_template(
    conversation,
    add_generation_prompt=True,
    return_tensors="pt",
    return_dict=True,
    tokenize=True,
).to(model.device)

outputs = model.generate(**inputs, max_new_tokens=64)
processor.batch_decode(outputs, skip_special_tokens=True)[0]
```

## Lfm2VlImageProcessor[[transformers.Lfm2VlImageProcessor]]

#### transformers.Lfm2VlImageProcessor[[transformers.Lfm2VlImageProcessor]]

```python
transformers.Lfm2VlImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lfm2_vl/image_processing_lfm2_vl.py#L218)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 512, 'width': 512}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BILINEAR`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.5, 0.5, 0.5]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.5, 0.5, 0.5]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, defaults to `True`) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

downsample_factor (`int`, *kwargs*, *optional*, defaults to `2`) : The downsampling factor for images used when resizing the image.

do_image_splitting (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to split large images into a grid of smaller tiles. When enabled, images exceeding the maximum token limit are divided into multiple tiles based on `min_tiles` and `max_tiles` constraints.

min_tiles (`int`, *kwargs*, *optional*, defaults to `2`) : Minimum number of tiles (width × height) to use when splitting an image into a grid. The grid configuration is chosen to maintain the original aspect ratio while staying within the `min_tiles` and `max_tiles` range.

max_tiles (`int`, *kwargs*, *optional*, defaults to `10`) : Maximum number of tiles (width × height) to use when splitting an image into a grid. The grid configuration is chosen to maintain the original aspect ratio while staying within the `min_tiles` and `max_tiles` range.

use_thumbnail (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to include a thumbnail version of the image when splitting into tiles. The thumbnail provides a low-resolution overview of the entire image and is added as an additional patch when the grid has more than one tile.

min_image_tokens (`int`, *kwargs*, *optional*, defaults to `64`) : Minimum number of image tokens (patches) to generate for an image. Images smaller than this threshold will be upscaled to meet the minimum token requirement.

max_image_tokens (`int`, *kwargs*, *optional*, defaults to `256`) : Maximum number of image tokens (patches) allowed for a single image. Images exceeding this limit will be split into multiple tiles or downscaled accordingly.

encoder_patch_size (`int`, *kwargs*, *optional*, defaults to `16`) : The patch size used by the vision encoder. Images are divided into patches of this size, and both height and width must be divisible by this value (after accounting for the downsampling factor).

tile_size (`int`, *kwargs*, *optional*, defaults to `512`) : The size of each tile when splitting large images into a grid. Each tile will be resized to this dimension before being processed into patches.

max_pixels_tolerance (`float`, *kwargs*, *optional*, defaults to `2.0`) : Tolerance factor for determining if an image is too large. An image is considered too large if its pixel count exceeds `max_image_tokens * encoder_patch_size^2 * downsample_factor^2 * max_pixels_tolerance`.

return_row_col_info (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to return row and column information for each image in the batch. When enabled, the output includes `image_rows`, `image_cols`, and `image_sizes` fields indicating the grid layout and dimensions of processed images.

Constructs a Lfm2VlImageProcessor image processor.

#### preprocess[[transformers.Lfm2VlImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], *args, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_utils.py#L382)

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

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## Lfm2VlProcessor[[transformers.Lfm2VlProcessor]]

#### transformers.Lfm2VlProcessor[[transformers.Lfm2VlProcessor]]

```python
transformers.Lfm2VlProcessor(image_processor, tokenizer, chat_template: str | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lfm2_vl/processing_lfm2_vl.py#L58)

**Parameters:**

image_processor (`Lfm2VlImageProcessor`) : The image processor is a required input.

tokenizer (`tokenizer_class`) : The tokenizer is a required input.

chat_template (`str`, *optional*) : A Jinja template to convert lists of messages in a chat into a tokenizable string.

Constructs a Lfm2VlProcessor which wraps a image processor and a tokenizer into a single processor.

[Lfm2VlProcessor](/docs/transformers/v5.15.1/en/model_doc/lfm2_vl#transformers.Lfm2VlProcessor) offers all the functionalities of [Lfm2VlImageProcessor](/docs/transformers/v5.15.1/en/model_doc/lfm2_vl#transformers.Lfm2VlImageProcessor) and `tokenizer_class`. See the
[~Lfm2VlImageProcessor](/docs/transformers/v5.15.1/en/model_doc/lfm2_vl#transformers.Lfm2VlImageProcessor) and `~tokenizer_class` for more information.

#### __call__[[transformers.Lfm2VlProcessor.__call__]]

```python
__call__(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, text: str | list[str] | list[list[str]] | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lfm2_vl/processing_lfm2_vl.py#L79)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

text (`Union[str, list[str], list[list[str]]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) : Additional processing options for each modality (text, images, videos, audio). Model-specific parameters are listed above; see the TypedDict class for the complete list of supported arguments.

## Lfm2VlConfig[[transformers.Lfm2VlConfig]]

#### transformers.Lfm2VlConfig[[transformers.Lfm2VlConfig]]

```python
transformers.Lfm2VlConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vision_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, image_token_id: int = 396, projector_hidden_act: str = 'gelu', projector_hidden_size: int = 2560, projector_bias: bool = True, projector_use_layernorm: bool = True, downsample_factor: int = 2, tie_word_embeddings: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lfm2_vl/configuration_lfm2_vl.py#L25)

**Parameters:**

vision_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the vision backbone.

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

image_token_id (`int`, *optional*, defaults to `396`) : The image token index used as a placeholder for input images.

projector_hidden_act (`str`, *optional*, defaults to `gelu`) : The activation function used by the multimodal projector.

projector_hidden_size (`int`, *optional*, defaults to `2560`) : Dimensionality of text and vision projection layers.

projector_bias (`bool`, *optional*, defaults to `True`) : Whether to use bias in the multimodal projector.

projector_use_layernorm (`bool`, *optional*, defaults to `True`) : Whether to use layernorm in the multimodal projector.

downsample_factor (`int`, *optional*, defaults to 2) : The downsample_factor factor of the vision backbone.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Lfm2VlModel. It is used to instantiate a Lfm2 Vl
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [LiquidAI/LFM2-VL-1.6B](https://huggingface.co/LiquidAI/LFM2-VL-1.6B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Lfm2VlModel[[transformers.Lfm2VlModel]]

#### transformers.Lfm2VlModel[[transformers.Lfm2VlModel]]

```python
transformers.Lfm2VlModel(config: Lfm2VlConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lfm2_vl/modeling_lfm2_vl.py#L147)

**Parameters:**

config ([Lfm2VlConfig](/docs/transformers/v5.15.1/en/model_doc/lfm2_vl#transformers.Lfm2VlConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Lfm2Vl model which consists of a vision backbone and a language model, without a language modeling head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Lfm2VlModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, spatial_shapes: typing.Optional[torch.Tensor] = None, pixel_attention_mask: typing.Optional[torch.Tensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lfm2_vl/modeling_lfm2_vl.py#L230)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [Lfm2VlImageProcessor](/docs/transformers/v5.15.1/en/model_doc/lfm2_vl#transformers.Lfm2VlImageProcessor). See `Lfm2VlImageProcessor.__call__()` for details ([Lfm2VlProcessor](/docs/transformers/v5.15.1/en/model_doc/lfm2_vl#transformers.Lfm2VlProcessor) uses [Lfm2VlImageProcessor](/docs/transformers/v5.15.1/en/model_doc/lfm2_vl#transformers.Lfm2VlImageProcessor) for processing images).

spatial_shapes (`torch.Tensor` of shape `(batch_size, 2)`, *optional*) : The spatial shapes of the input images.

pixel_attention_mask (`torch.Tensor` of shape `(batch_size, height, width)`, *optional*) : The pixel attention mask of the input images.

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** `Lfm2VlModelOutputWithPast` or `tuple(torch.FloatTensor)`

A `Lfm2VlModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Lfm2VlConfig](/docs/transformers/v5.15.1/en/model_doc/lfm2_vl#transformers.Lfm2VlConfig)) and inputs.

The [Lfm2VlModel](/docs/transformers/v5.15.1/en/model_doc/lfm2_vl#transformers.Lfm2VlModel) forward method, overrides the `__call__` special method.

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

#### get_image_features[[transformers.Lfm2VlModel.get_image_features]]

```python
get_image_features(pixel_values: FloatTensor, spatial_shapes: Tensor, pixel_attention_mask: Tensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lfm2_vl/modeling_lfm2_vl.py#L156)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, channels, height, width)`) : The tensors corresponding to the input images.

spatial_shapes (`torch.Tensor` of shape `(batch_size, 2)`) : The spatial shapes of the input images.

pixel_attention_mask (`torch.Tensor` of shape `(batch_size, height, width)`) : The pixel attention mask of the input images.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Lfm2VlConfig](/docs/transformers/v5.15.1/en/model_doc/lfm2_vl#transformers.Lfm2VlConfig)) and inputs.

Obtains image last hidden states from the vision tower and apply multimodal projection.

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

## Lfm2VlForConditionalGeneration[[transformers.Lfm2VlForConditionalGeneration]]

#### transformers.Lfm2VlForConditionalGeneration[[transformers.Lfm2VlForConditionalGeneration]]

```python
transformers.Lfm2VlForConditionalGeneration(config: Lfm2VlConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lfm2_vl/modeling_lfm2_vl.py#L296)

**Parameters:**

config ([Lfm2VlConfig](/docs/transformers/v5.15.1/en/model_doc/lfm2_vl#transformers.Lfm2VlConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The LFM2_VL model which consists of a vision backbone and a language model.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Lfm2VlForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, spatial_shapes: typing.Optional[torch.Tensor] = None, pixel_attention_mask: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lfm2_vl/modeling_lfm2_vl.py#L331)

pixel_values (`torch.FloatTensor` of shape `(batch_size, channels, height, width)`, *optional*):
The input image tensors.
spatial_shapes (`torch.Tensor` of shape `(batch_size, 2)`, *optional*):
The spatial shapes of the input images.
pixel_attention_mask (`torch.Tensor` of shape `(batch_size, height, width)`, *optional*):
The pixel attention mask of the input images.
labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*):
Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
(masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

Example:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoProcessor, AutoModelForImageTextToText
>>> from transformers.image_utils import load_image

>>> model = AutoModelForImageTextToText.from_pretrained(
...     "LiquidAI/LFM2-VL-1.6B",
... )
>>> processor = AutoProcessor.from_pretrained(
...     "LiquidAI/LFM2-VL-1.6B",
... )

>>> url = "https://www.ilankelman.org/stopsigns/australia.jpg"
>>> image = load_image(url)

>>> conversation = [
...     {
...         "role": "user",
...         "content": [
...             {"type": "image", "image": image},
...             {"type": "text", "text": "What is in this image?"},
...         ],
...     },
... ]

>>> inputs = processor.apply_chat_template(
...     conversation,
...     add_generation_prompt=True,
...     tokenize=True,
...     return_dict=True,
...     return_tensors="pt"
... )

>>> # Generate
>>> outputs = model.generate(**inputs, max_new_tokens=45)
>>> processor.batch_decode(outputs, skip_special_tokens=True)[0]
'This image depicts a vibrant street scene in what appears to be a Chinatown or similar cultural area. The focal point is a large red stop sign with white lettering, mounted on a pole.'
```

#### get_image_features[[transformers.Lfm2VlForConditionalGeneration.get_image_features]]

```python
get_image_features(pixel_values: FloatTensor, spatial_shapes: Tensor, pixel_attention_mask: Tensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lfm2_vl/modeling_lfm2_vl.py#L308)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, channels, height, width)`) : The tensors corresponding to the input images.

spatial_shapes (`torch.Tensor` of shape `(batch_size, 2)`) : The spatial shapes of the input images.

pixel_attention_mask (`torch.Tensor` of shape `(batch_size, height, width)`) : The pixel attention mask of the input images.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Lfm2VlConfig](/docs/transformers/v5.15.1/en/model_doc/lfm2_vl#transformers.Lfm2VlConfig)) and inputs.

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
>>> from transformers import AutoProcessor, Lfm2VlForConditionalGeneration

>>> model = Lfm2VlForConditionalGeneration.from_pretrained("LiquidAI/LFM2-VL-1.6B")
>>> processor = AutoProcessor.from_pretrained("LiquidAI/LFM2-VL-1.6B")

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
