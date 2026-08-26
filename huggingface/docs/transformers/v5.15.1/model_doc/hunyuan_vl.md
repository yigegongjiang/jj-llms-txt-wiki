# HunYuanVL

## Overview

HunYuanVL is a vision-language model for image-text understanding and generation
proposed in [HunyuanOCR Technical Report
](https://huggingface.co/papers/2511.19575). The open-source `hunyuan_vl` integration in Transformers is a
dense-only image-text variant tailored for OCR and document understanding style workloads such as [`tencent/HunyuanOCR`]((https://huggingface.co/tencent/HunyuanOCR)).

The abstract from the paper is the following:

*This paper presents HunyuanOCR, a commercial-grade, open-source, and lightweight (1B parameters) Vision-Language Model
(VLM) dedicated to OCR tasks. The architecture comprises a Native Vision Transformer (ViT) and a lightweight LLM
connected via an MLP adapter. HunyuanOCR demonstrates superior performance, outperforming commercial APIs, traditional
pipelines, and larger models (e.g., Qwen3-VL-4B). Specifically, it surpasses current public solutions in perception
tasks (Text Spotting, Parsing) and excels in semantic tasks (IE, Text Image Translation), securing first place in the
ICDAR 2025 DIMT Challenge (Small Model Track). Furthermore, it achieves state-of-the-art (SOTA) results on OCRBench
among VLMs with fewer than 3B parameters.*

*HunyuanOCR achieves breakthroughs in three key aspects: 1) Unifying Versatility and Efficiency: We implement
comprehensive support for core capabilities, including spotting, parsing, IE, VQA, and translation within a lightweight
framework. This addresses the limitations of narrow "OCR expert models" and inefficient "General VLMs". 2) Streamlined
End-to-End Architecture: Adopting a pure end-to-end paradigm eliminates dependencies on pre-processing modules (e.g.,
layout analysis). This fundamentally resolves error propagation common in traditional pipelines and simplifies system
deployment. 3) Data-Driven and RL Strategies: We confirm the critical role of high-quality data and, for the first time
in the industry, demonstrate that Reinforcement Learning (RL) strategies yield significant performance gains in OCR
tasks.*

*HunyuanOCR is officially open-sourced on HuggingFace. We also provide a high-performance deployment solution based on
vLLM, placing its production efficiency in the top tier. We hope this model will advance frontier research and provide a
solid foundation for industrial applications.*

## Recommended checkpoints

- [tencent/HunyuanOCR](https://huggingface.co/tencent/HunyuanOCR) for OCR and document extraction workloads.

## Usage tips

This Transformers integration intentionally exposes the image-text path that is exercised by public OCR-style
checkpoints.

- Supported: dense-only text backbone, image-text prompting, OCR/document-understanding style generation.
- Not supported as part of this open-source variant: video inputs and runtime MoE execution paths.
- Compatibility note: some legacy Tencent-export configuration fields are still accepted so existing checkpoints can be
  loaded, but those fields do not imply that the open-source implementation enables extra runtime capabilities.
- For the currently validated OCR path, `attn_implementation="eager"` is the recommended starting point.
- `backend="pil"` is recommended when loading the processor for the current public OCR checkpoints.
- When batching variable-length prompts, pass `padding=True` if you need tensor outputs from the processor.

## Usage

```python
import torch
from transformers import AutoModelForImageTextToText, AutoProcessor

model_name_or_path = "tencent/HunyuanOCR"
processor = AutoProcessor.from_pretrained(model_name_or_path, backend="pil")
model = AutoModelForImageTextToText.from_pretrained(
    model_name_or_path,
    device_map="auto",
)

messages = [
    {
        "role": "user",
        "content": [
            {"type": "image", "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/p-blog/candy.JPG"},
            {"type": "text", "text": "What animal is on the candy?"}
        ]
    },
]
inputs = processor.apply_chat_template(
    messages,
    tokenize=True,
    add_generation_prompt=True,
    return_tensors="pt",
).to(model.device)

generated_ids = model.generate(**inputs, max_new_tokens=1024)

generated_ids_trimmed = generated_ids[0][len(inputs["input_ids"][0]) :]
output = processor.decode(generated_ids_trimmed, skip_special_tokens=True)
print(output)
```

## HunYuanVLProcessor[[transformers.HunYuanVLProcessor]]

#### transformers.HunYuanVLProcessor[[transformers.HunYuanVLProcessor]]

```python
transformers.HunYuanVLProcessor(image_processor = None, tokenizer = None, chat_template = None, cat_extra_token: bool = True, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/processing_hunyuan_vl.py#L36)

**Parameters:**

image_processor (`HunYuanVLImageProcessor`) : The image processor is a required input.

tokenizer (`Qwen2Tokenizer`) : The tokenizer is a required input.

chat_template (`str`) : A Jinja template to convert lists of messages in a chat into a tokenizable string.

cat_extra_token (`bool`, *optional*, defaults to `True`) : Whether to account for the two extra tokens that HunYuanVL inserts around each image span when computing the expanded image token sequence.

Constructs a HunYuanVLProcessor which wraps a image processor and a tokenizer into a single processor.

[HunYuanVLProcessor](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLProcessor) offers all the functionalities of [HunYuanVLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLImageProcessor) and [Qwen2Tokenizer](/docs/transformers/v5.15.1/en/model_doc/qwen2#transformers.Qwen2Tokenizer). See the
[~HunYuanVLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLImageProcessor) and [~Qwen2Tokenizer](/docs/transformers/v5.15.1/en/model_doc/qwen2#transformers.Qwen2Tokenizer) for more information.

#### __call__[[transformers.HunYuanVLProcessor.__call__]]

```python
__call__(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, text: str | list[str] | list[list[str]] | None = None, videos: typing.Union[list['PIL.Image.Image'], numpy.ndarray, ForwardRef('torch.Tensor'), list[numpy.ndarray], list['torch.Tensor'], list[list['PIL.Image.Image']], list[list[numpy.ndarray]], list[list['torch.Tensor']], transformers.video_utils.URL, list[transformers.video_utils.URL], list[list[transformers.video_utils.URL]], transformers.video_utils.Path, list[transformers.video_utils.Path], list[list[transformers.video_utils.Path]], NoneType] = None, audio: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor'], NoneType] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/processing_utils.py#L651)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

text (`Union[str, list[str], list[list[str]]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

videos (`Union[list[PIL.Image.Image], numpy.ndarray, torch.Tensor, list[numpy.ndarray], list[torch.Tensor], list[list[PIL.Image.Image]], list[list[numpy.ndarray]], list[list[torch.Tensor]], ~video_utils.URL, list[~video_utils.URL], list[list[~video_utils.URL]], ~video_utils.Path, list[~video_utils.Path], list[list[~video_utils.Path]]]`, *optional*) : Video to preprocess. Expects a single or batch of videos with pixel values ranging from 0 to 255. If passing in videos with pixel values between 0 and 1, set `do_rescale=False`.

audio (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`, *optional*) : The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor. In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels, and T is the sample length of the audio.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) : Additional processing options for each modality (text, images, videos, audio). Model-specific parameters are listed above; see the TypedDict class for the complete list of supported arguments.

## HunYuanVLImageProcessor[[transformers.HunYuanVLImageProcessor]]

#### transformers.HunYuanVLImageProcessor[[transformers.HunYuanVLImageProcessor]]

```python
transformers.HunYuanVLImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/image_processing_hunyuan_vl.py#L87)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'shortest_edge' : 262144, 'longest_edge': 4194304}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BICUBIC`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.48145466, 0.4578275, 0.40821073]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.26862954, 0.26130258, 0.27577711]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

min_pixels (`int`, *kwargs*, *optional*, defaults to `512 * 512`) : The min pixels of the image to resize the image.

max_pixels (`int`, *kwargs*, *optional*, defaults to `2048 * 2048`) : The max pixels of the image to resize the image.

patch_size (`int`, *kwargs*, *optional*, defaults to 16) : The spatial patch size of the vision encoder.

temporal_patch_size (`int`, *kwargs*, *optional*, defaults to 1) : The temporal patch size of the vision encoder.

merge_size (`int`, *kwargs*, *optional*, defaults to 2) : The merge size of the vision encoder to llm encoder.

Constructs a HunYuanVLImageProcessor image processor.

#### get_number_of_image_patches[[transformers.HunYuanVLImageProcessor.get_number_of_image_patches]]

```python
get_number_of_image_patches(height: int, width: int, images_kwargs: dict | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/image_processing_hunyuan_vl.py#L256)

**Parameters:**

height (`int`) : Height of the input image.

width (`int`) : Width of the input image.

images_kwargs (`dict`, *optional*) : Any kwargs to override defaults of the image processor.

**Returns:** `tuple[int, int]`

Number of image patches per image, as a `(height, width)` grid.

A utility that returns number of image patches for a given image size.

Note: Do not remove this method! It is used by vLLM to infer the number of patches and placeholders
without an image input.

#### patchify[[transformers.HunYuanVLImageProcessor.patchify]]

```python
patchify(images: torch.Tensor, patch_size: int, merge_size: int, temporal_patch_size: int)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/image_processing_hunyuan_vl.py#L164)

Patchifies each image into flat layout of shape (`seq_len`, `patch_dim`) so we can concat dynamically shaped pixels.

#### preprocess[[transformers.HunYuanVLImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/image_processing_hunyuan_vl.py#L127)

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

min_pixels (`int`, *kwargs*, *optional*, defaults to `512 * 512`) : The min pixels of the image to resize the image.

max_pixels (`int`, *kwargs*, *optional*, defaults to `2048 * 2048`) : The max pixels of the image to resize the image.

patch_size (`int`, *kwargs*, *optional*, defaults to 16) : The spatial patch size of the vision encoder.

temporal_patch_size (`int`, *kwargs*, *optional*, defaults to 1) : The temporal patch size of the vision encoder.

merge_size (`int`, *kwargs*, *optional*, defaults to 2) : The merge size of the vision encoder to llm encoder.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

#### resize[[transformers.HunYuanVLImageProcessor.resize]]

```python
resize(images: torch.Tensor, size: SizeDict, resample: PILImageResampling | tvF.InterpolationMode | int | None, factor: int, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/image_processing_hunyuan_vl.py#L135)

Resize dynamically based on input image aspect ratio.

## HunYuanVLImageProcessorPil[[transformers.HunYuanVLImageProcessorPil]]

#### transformers.HunYuanVLImageProcessorPil[[transformers.HunYuanVLImageProcessorPil]]

```python
transformers.HunYuanVLImageProcessorPil(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/image_processing_pil_hunyuan_vl.py#L87)

#### get_number_of_image_patches[[transformers.HunYuanVLImageProcessorPil.get_number_of_image_patches]]

```python
get_number_of_image_patches(height: int, width: int, images_kwargs: dict | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/image_processing_pil_hunyuan_vl.py#L252)

**Parameters:**

height (`int`) : Height of the input image.

width (`int`) : Width of the input image.

images_kwargs (`dict`, *optional*) : Any kwargs to override defaults of the image processor.

**Returns:** `tuple[int, int]`

Number of image patches per image, as a `(height, width)` grid.

A utility that returns number of image patches for a given image size.

Note: Do not remove this method! It is used by vLLM to infer the number of patches and placeholders
without an image input.

#### patchify[[transformers.HunYuanVLImageProcessorPil.patchify]]

```python
patchify(image: ndarray, patch_size: int, merge_size: int, temporal_patch_size: int)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/image_processing_pil_hunyuan_vl.py#L156)

Patchifies each image into flat layout of shape (`seq_len`, `patch_dim`) so we can concat dynamically shaped pixels.

#### preprocess[[transformers.HunYuanVLImageProcessorPil.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/image_processing_pil_hunyuan_vl.py#L193)

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

min_pixels (`int`, *kwargs*, *optional*, defaults to `512 * 512`) : The min pixels of the image to resize the image.

max_pixels (`int`, *kwargs*, *optional*, defaults to `2048 * 2048`) : The max pixels of the image to resize the image.

patch_size (`int`, *kwargs*, *optional*, defaults to 16) : The spatial patch size of the vision encoder.

temporal_patch_size (`int`, *kwargs*, *optional*, defaults to 1) : The temporal patch size of the vision encoder.

merge_size (`int`, *kwargs*, *optional*, defaults to 2) : The merge size of the vision encoder to llm encoder.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

#### resize[[transformers.HunYuanVLImageProcessorPil.resize]]

```python
resize(image: ndarray, size: SizeDict, resample: PILImageResampling | int | None, factor: int, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/image_processing_pil_hunyuan_vl.py#L127)

Resize dynamically based on input image aspect ratio.

`HunYuanVLForConditionalGeneration` is the main public entrypoint for image-text generation. `HunYuanVLModel` exposes
the multimodal base model without the language modeling head, while `HunYuanVLTextModel` exposes the lower-level text
backbone.

## HunYuanVLConfig[[transformers.HunYuanVLConfig]]

#### transformers.HunYuanVLConfig[[transformers.HunYuanVLConfig]]

```python
transformers.HunYuanVLConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, vision_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, image_token_id: int = 120120, tie_word_embeddings: bool = True, im_start_id: int = 120118, im_end_id: int = 120119, im_newline_id: int = 120121)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/configuration_hunyuan_vl.py#L235)

**Parameters:**

text_config (`HunYuanVLTextConfig` or `dict`, *optional*) : Configuration of the text backbone. When `None`, default values are used.

vision_config (`HunYuanVLVisionConfig` or `dict`, *optional*) : Configuration of the vision tower. When `None`, default values are used.

image_token_id (`int`, *optional*, defaults to `120120`) : The image token index used as a placeholder for input images.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

im_start_id (`int`, *optional*, defaults to 120118) : Token id marking the beginning of an image span in multimodal prompts.

im_end_id (`int`, *optional*, defaults to 120119) : Token id marking the end of an image span in multimodal prompts.

im_newline_id (`int`, *optional*, defaults to 120121) : Token id used for newline-style separators inserted inside serialized image regions.

Top-level configuration for the open-source HunYuanVL integration.

This configuration describes the dense-only, image-text-only variant used for OCR and document-understanding style
workloads. It mirrors the `Qwen2_5_VL` / `Qwen3_VL` family layout: the top-level config simply composes a
[HunYuanVLTextConfig](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLTextConfig) (text backbone) and a [HunYuanVLVisionConfig](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLVisionConfig) (vision tower) plus a few token ids that
delimit image spans in multimodal prompts.

Example:

```python
>>> from transformers import HunYuanVLConfig, HunYuanVLForConditionalGeneration
>>>
>>> configuration = HunYuanVLConfig()
>>> model = HunYuanVLForConditionalGeneration(configuration)
>>> configuration = model.config
```

## HunYuanVLVisionConfig[[transformers.HunYuanVLVisionConfig]]

#### transformers.HunYuanVLVisionConfig[[transformers.HunYuanVLVisionConfig]]

```python
transformers.HunYuanVLVisionConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_act: str = 'gelu', hidden_size: int = 1152, intermediate_size: int = 4304, interpolate_mode: str = 'bilinear', rms_norm_eps: float = 1e-05, attention_dropout: float = 0.0, num_attention_heads: int = 16, num_key_value_heads: int | None = None, num_channels: int = 3, num_hidden_layers: int = 27, out_hidden_size: int = 4096, patch_size: int = 16, spatial_merge_size: int = 2, temporal_patch_size: int = 1, img_max_token_num: int = 4096, max_image_size: int = 2048, min_image_size: int = 512, max_vit_seq_len: int = 16384, text_hidden_size: int = 3072)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/configuration_hunyuan_vl.py#L35)

**Parameters:**

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

hidden_size (`int`, *optional*, defaults to `1152`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `4304`) : Dimension of the MLP representations.

interpolate_mode (`str`, *optional*, defaults to `"bilinear"`) : Interpolation mode used when resizing learned patch positional embeddings to match the current image grid.

rms_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the rms normalization layers.

attention_dropout (`float`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

num_attention_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

num_hidden_layers (`int`, *optional*, defaults to `27`) : Number of hidden layers in the Transformer decoder.

out_hidden_size (`int`, *optional*, defaults to 4096) : Output hidden size produced by the vision tower before it is consumed by the text backbone.

patch_size (`int`, *optional*, defaults to `16`) : The size (resolution) of each patch.

spatial_merge_size (`int`, *optional*, defaults to `2`) : The size of the spatial merge window used to reduce the number of visual tokens by merging neighboring patches.

temporal_patch_size (`int`, *optional*, defaults to `1`) : Temporal patch size used in the 3D patch embedding for video inputs.

img_max_token_num (`int`, *optional*, defaults to 4096) : Maximum image token count expected by the vision stack.

max_image_size (`int`, *optional*, defaults to 2048) : Maximum supported image size for the current open-source vision configuration.

min_image_size (`int`, *optional*, defaults to 512) : Minimum supported image size for the current open-source vision configuration.

max_vit_seq_len (`int`, *optional*, defaults to 16384) : Maximum sequence length produced by the vision transformer.

text_hidden_size (`int`, *optional*, defaults to 3072) : Hidden size expected by the text backbone when consuming visual embeddings.

Vision backbone configuration for the dense-only, image-text HunYuanVL open-source variant.

Example:

```python
>>> from transformers import HunYuanVLVisionConfig
>>>
>>> configuration = HunYuanVLVisionConfig()
>>> configuration.hidden_size
1152
```

## HunYuanVLTextConfig[[transformers.HunYuanVLTextConfig]]

#### transformers.HunYuanVLTextConfig[[transformers.HunYuanVLTextConfig]]

```python
transformers.HunYuanVLTextConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 290943, hidden_size: int = 4096, intermediate_size: int = 11008, num_hidden_layers: int = 32, num_attention_heads: int = 32, num_key_value_heads: int | None = None, hidden_act: str = 'silu', max_position_embeddings: int = 2048, initializer_range: float = 0.02, rms_norm_eps: float = 1e-05, use_cache: bool = True, pad_token_id: int | None = 0, bos_token_id: int | None = 1, eos_token_id: int | list[int] | None = 2, eod_token_id: int | None = 3, pretraining_tp: int = 1, tie_word_embeddings: bool = True, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, attention_bias: bool = False, attention_dropout: float | int = 0.0, head_dim: int | None = None, sep_token_id: int | None = 4)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/configuration_hunyuan_vl.py#L106)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `290943`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `4096`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `11008`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `32`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `32`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `2048`) : The maximum sequence length that this model might ever be used with.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

rms_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the rms normalization layers.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

pad_token_id (`int`, *optional*, defaults to `0`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `1`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `2`) : Token id used for end-of-stream in the vocabulary.

eod_token_id (`int`, *optional*, defaults to 3) : Token id representing the end-of-document marker. Inherited from [HunYuanDenseV1Config](/docs/transformers/v5.15.1/en/model_doc/hunyuan_v1_dense#transformers.HunYuanDenseV1Config) and re-documented here so the auto-generated docstring stays in sync.

pretraining_tp (`int`, *optional*, defaults to `1`) : Experimental feature. Tensor parallelism rank used during pretraining. Please refer to [this document](https://huggingface.co/docs/transformers/main/perf_train_gpu_many#tensor-parallelism) to understand more about it. This value is necessary to ensure exact reproducibility of the pretraining results. Please refer to [this issue](https://github.com/pytorch/pytorch/issues/76232).

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

rope_parameters (`dict`, *optional*) : RoPE configuration inherited from [HunYuanDenseV1Config](/docs/transformers/v5.15.1/en/model_doc/hunyuan_v1_dense#transformers.HunYuanDenseV1Config). When `mrope_section` is present, it partitions half of each attention head across HunYuanVL's multimodal RoPE axes. The expected order is `(width, height, image_index)` for 3-axis multimodal RoPE and `(position, width, height, image_index)` for 4-axis multimodal RoPE. The `image_index` axis is the ordinal of the image/frame in the input sequence; all visual tokens from one image share the same value on that axis.

attention_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

head_dim (`int`, *optional*) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

sep_token_id (`int`, *optional*, defaults to 4) : Token id used as a separator marker by HunYuan tokenizers.

Text backbone configuration for the dense-only, image-text HunYuanVL open-source variant.

Inherits the standard fields from [HunYuanDenseV1Config](/docs/transformers/v5.15.1/en/model_doc/hunyuan_v1_dense#transformers.HunYuanDenseV1Config) and declares the canonical field names
(`pad_token_id`, `head_dim`, `vocab_size`) as the only public attributes. Legacy aliases that some Tencent
checkpoints persist on disk (`pad_id`, `attention_head_dim`, `org_vocab_size`) are mapped onto those canonical
fields via `attribute_map`, so the rest of the model only ever needs to read the canonical fields. Legacy RoPE
payloads persisted as `rope_scaling` / `rope_theta` are normalized by the base configuration class into
`rope_parameters`.

## HunYuanVLVisionTransformer[[transformers.HunYuanVLVisionTransformer]]

#### transformers.HunYuanVLVisionTransformer[[transformers.HunYuanVLVisionTransformer]]

```python
transformers.HunYuanVLVisionTransformer(config: HunYuanVLVisionConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/modeling_hunyuan_vl.py#L689)

HunYuanVL vision tower: patch embedding -> transformer blocks -> per-image patch merger.

Inputs are flat per-patch pixel tensors plus an `image_grid_thw` tensor describing the spatial layout of every
image in the batch. The output is the concatenation of merged image embeddings, ready to be scattered into the
language-model embedding stream.

#### forward[[transformers.HunYuanVLVisionTransformer.forward]]

```python
forward(pixel_values: Tensor, grid_thw: LongTensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/modeling_hunyuan_vl.py#L715)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(num_patches, num_channels * patch_size * patch_size)`) : Flat per-patch pixel features produced by the image processor.

grid_thw (`torch.LongTensor` of shape `(num_images, 3)`) : The temporal, height and width dimensions for each image. Each row contains `[t, h, w]` patch counts.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([HunYuanVLConfig](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLConfig)) and inputs.

The [HunYuanVLVisionTransformer](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLVisionTransformer) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

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

## HunYuanVLTextModel[[transformers.HunYuanVLTextModel]]

#### transformers.HunYuanVLTextModel[[transformers.HunYuanVLTextModel]]

```python
transformers.HunYuanVLTextModel(config: HunYuanVLTextConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/modeling_hunyuan_vl.py#L758)

**Parameters:**

config ([HunYuanVLTextConfig](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLTextConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Hunyuan Vl Text Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.HunYuanVLTextModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/modeling_hunyuan_vl.py#L780)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([HunYuanVLConfig](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLConfig)) and inputs.

The [HunYuanVLTextModel](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLTextModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and optionally if
  `config.is_encoder_decoder=True` in the cross-attention blocks) that can be used (see `past_key_values`
  input) to speed up sequential decoding.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

## HunYuanVLModel[[transformers.HunYuanVLModel]]

#### transformers.HunYuanVLModel[[transformers.HunYuanVLModel]]

```python
transformers.HunYuanVLModel(config: HunYuanVLConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/modeling_hunyuan_vl.py#L859)

**Parameters:**

config ([HunYuanVLConfig](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The HunYuanVL model which consists of a vision backbone and a language model, without a language modeling head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.HunYuanVLModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, pixel_values: typing.Optional[torch.FloatTensor] = None, image_grid_thw: typing.Optional[torch.LongTensor] = None, mm_token_type_ids: typing.Optional[torch.IntTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/modeling_hunyuan_vl.py#L1065)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [HunYuanVLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLImageProcessor). See `HunYuanVLImageProcessor.__call__()` for details ([HunYuanVLProcessor](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLProcessor) uses [HunYuanVLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLImageProcessor) for processing images).

image_grid_thw (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) : The temporal, height and width of feature shape of each image in LLM.

mm_token_type_ids (`torch.IntTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens matching each modality. For example text (0), image (1), video (2). Multimodal token type ids can be obtained using [AutoProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoProcessor). See [ProcessorMixin.__call__()](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessorMixin.__call__) for details.

**Returns:** `HunYuanVLModelOutputWithPast` or `tuple(torch.FloatTensor)`

A `HunYuanVLModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([HunYuanVLConfig](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLConfig)) and inputs.

The [HunYuanVLModel](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **image_hidden_states** (`torch.FloatTensor`, *optional*) -- Last image features produced by the vision tower and scattered into the language-model token stream.

#### get_image_features[[transformers.HunYuanVLModel.get_image_features]]

```python
get_image_features(pixel_values: FloatTensor, image_grid_thw: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/modeling_hunyuan_vl.py#L999)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [HunYuanVLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLImageProcessor). See `HunYuanVLImageProcessor.__call__()` for details ([HunYuanVLProcessor](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLProcessor) uses [HunYuanVLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLImageProcessor) for processing images).

image_grid_thw (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) : The temporal, height and width of feature shape of each image in LLM.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([HunYuanVLConfig](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLConfig)) and inputs.

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

## HunYuanVLForConditionalGeneration[[transformers.HunYuanVLForConditionalGeneration]]

#### transformers.HunYuanVLForConditionalGeneration[[transformers.HunYuanVLForConditionalGeneration]]

```python
transformers.HunYuanVLForConditionalGeneration(config: HunYuanVLConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/modeling_hunyuan_vl.py#L1127)

**Parameters:**

config ([HunYuanVLConfig](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Hunyuan Vl Model for token generation conditioned on other modalities (e.g. image-text-to-text generation).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.HunYuanVLForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, pixel_values: typing.Optional[torch.FloatTensor] = None, image_grid_thw: typing.Optional[torch.LongTensor] = None, mm_token_type_ids: typing.Optional[torch.IntTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/modeling_hunyuan_vl.py#L1148)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [HunYuanVLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLImageProcessor). See `HunYuanVLImageProcessor.__call__()` for details ([HunYuanVLProcessor](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLProcessor) uses [HunYuanVLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLImageProcessor) for processing images).

image_grid_thw (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) : The temporal, height and width of feature shape of each image in LLM.

mm_token_type_ids (`torch.IntTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens matching each modality. For example text (0), image (1), video (2). Multimodal token type ids can be obtained using [AutoProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoProcessor). See [ProcessorMixin.__call__()](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessorMixin.__call__) for details.

**Returns:** [CausalLMOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or `tuple(torch.FloatTensor)`

A [CausalLMOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([HunYuanVLConfig](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLConfig)) and inputs.

The [HunYuanVLForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/hunyuan_vl#transformers.HunYuanVLForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
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

Example:

```python
>>> from transformers import AutoProcessor, HunYuanVLForConditionalGeneration
>>> import torch

>>> model_id = "tencent/HunyuanOCR"
>>> processor = AutoProcessor.from_pretrained(model_id, backend="pil")
>>> model = HunYuanVLForConditionalGeneration.from_pretrained(
...     model_id, attn_implementation="eager", torch_dtype=torch.bfloat16, device_map="auto"
... )

>>> messages = [
...     {
...         "role": "user",
...         "content": [
...             {"type": "image", "image": "path/to/your/image.jpg"},
...             {"type": "text", "text": "Extract the text from the image."},
...         ],
...     }
... ]
>>> inputs = processor.apply_chat_template(
...     messages,
...     tokenize=True,
...     add_generation_prompt=True,
...     return_tensors="pt",
...     return_dict=True,
...     processor_kwargs={"padding": True},
... )

>>> with torch.no_grad():
...     generated_ids = model.generate(**inputs, max_new_tokens=128)
>>> generated_trimmed = generated_ids[0][inputs["input_ids"].shape[-1]:]
>>> print(processor.decode(generated_trimmed, skip_special_tokens=True))
```

#### get_image_features[[transformers.HunYuanVLForConditionalGeneration.get_image_features]]

```python
get_image_features(pixel_values: FloatTensor, image_grid_thw: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hunyuan_vl/modeling_hunyuan_vl.py#L1140)
