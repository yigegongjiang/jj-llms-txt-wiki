# LLaVA-NeXT

[LLaVA‑NeXT](https://llava-vl.github.io/blog/2024-01-30-llava-next/) improves on [Llava](./llava) by increasing the input image resolution by 4x more pixels and supporting 3 aspect ratios (up to 672x672, 336x1344, 1344x336) to better grasp visual details. It is also trained on an improved visual instruction tuning dataset covering more scenarios and applications to improve OCR and common sense reasoning.

You can find all the original LLaVA‑NeXT checkpoints under the [LLaVA-NeXT](https://huggingface.co/collections/llava-hf/llava-next-65f75c4afac77fd37dbbe6cf) collection.

> [!TIP]
> This model was contributed by [nielsr](https://huggingface.co/nielsr).
>
> Click on the LLaVA‑NeXT models in the right sidebar for more examples of how to apply Llava-NeXT to different multimodal tasks.

The example below demonstrates how to generate text based on an image with [Pipeline](/docs/transformers/v5.15.1/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline

pipeline = pipeline(
    task="image-text-to-text",
    model="llava-hf/llava-v1.6-mistral-7b-hf",
    device=0,
)
messages = [
    {
        "role": "user",
        "content": [
            {
                "type": "image",
                "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg",
            },
            { "type": "text", "text": "Describe this image."},
        ]
    }
]
pipeline(text=messages, max_new_tokens=20, return_full_text=False)
```

```python
import requests
from PIL import Image

from transformers import AutoProcessor, LlavaNextForConditionalGeneration

processor = AutoProcessor.from_pretrained("llava-hf/llava-v1.6-mistral-7b-hf")
model = LlavaNextForConditionalGeneration.from_pretrained("llava-hf/llava-v1.6-mistral-7b-hf", device_map="auto")

url = "https://github.com/haotian-liu/LLaVA/blob/1a91fc274d7c35a9b50b3cb29c4247ae5837ce39/images/llava_v1_5_radar.jpg?raw=true"
image = Image.open(requests.get(url, stream=True).raw)

conversation = [
    {
        "role": "user",
        "content": [
            {"type": "image"},
            {"type": "text", "text": "What is shown in this image?"},
        ],
    },
]
prompt = processor.apply_chat_template(conversation, add_generation_prompt=True)
inputs = processor(image, prompt, return_tensors="pt").to(model.device)
output = model.generate(**inputs, max_new_tokens=100)
print(processor.decode(output[0], skip_special_tokens=True))
```

Quantization reduces the memory burden of large models by representing the weights in a lower precision. Refer to the [Quantization](../quantization/overview) overview for more available quantization backends.

The example below uses [bitsandbytes](../quantization/bitsandbytes) to only quantize the weights to int4.

```python
import requests
import torch
from PIL import Image

from transformers import AutoModelForImageTextToText, AutoProcessor, BitsAndBytesConfig

quant_config = BitsAndBytesConfig(
    load_in_4bit=True,
    bnb_4bit_quant_type="nf4"
)

processor = AutoProcessor.from_pretrained("llava-hf/llava-v1.6-mistral-7b-hf")
model = AutoModelForImageTextToText.from_pretrained("llava-hf/llava-v1.6-mistral-7b-hf", quantization_config=quant_config, device_map="auto")

url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/llava_next_ocr.png"
image = Image.open(requests.get(url, stream=True).raw)

conversation = [
    {
        "role": "user",
        "content": [
            {"type": "image"},
            {"type": "text", "text": "What does this chart show?"},
        ],
    },
]
prompt = processor.apply_chat_template(conversation, add_generation_prompt=True)
inputs = processor(image, prompt, return_tensors="pt").to(model.device)

with torch.inference_mode():
    output = model.generate(**inputs, max_new_tokens=100)
print(processor.decode(output[0], skip_special_tokens=True))
```

## Notes

* Different checkpoints (Mistral, Vicuna, etc.) require a specific prompt format depending on the underlying LLM. Always use [apply_chat_template()](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessorMixin.apply_chat_template) to ensure correct formatting. Refer to the [Templates](../chat_templating) guide for more details.

* Set `padding_side="left"` during batched generation for more accurate results.

```py
processor.tokenizer.padding_side = "left"
```

* LLaVA-NeXT uses different numbers of patches for images and pads the inputs inside the modeling code except when padding is done during processing. The default setting is *left-padding* if the model is in `eval()` mode, otherwise it is *right-padding*.

* LLaVA models after v4.46 raises warnings about adding `processor.patch_size = {{patch_size}}`, `processor.num_additional_image_tokens = {{num_additional_image_tokens}}`, and `processor.vision_feature_select_strategy = {{vision_feature_select_strategy}}`. It is strongly recommended to add these attributes to the processor if you own the model checkpoint or open a PR if it isn't.

  Adding these attributes means LLaVA will try to infer the number of image tokens required per image and expand the text with the same number of `<image>` token placeholders. There are usually ~500 tokens per image, so make sure the text is not truncated because it will cause a failure when merging the embeddings. The attributes can be found in `model.config.vision_config.patch_size` or `model.config.vision_feature_select_strategy`.

  The `num_additional_image_tokens` should be `1` if the vision backbone adds a `CLS` token or `0` if nothing extra is added.

* The example below demonstrates inference with multiple input images.

```python
import requests
from PIL import Image

from transformers import LlavaNextForConditionalGeneration, LlavaNextProcessor

processor = LlavaNextProcessor.from_pretrained("llava-hf/llava-v1.6-mistral-7b-hf")
model = LlavaNextForConditionalGeneration.from_pretrained(
    "llava-hf/llava-v1.6-mistral-7b-hf", device_map="auto"
)

# Load multiple images
url1 = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/llava_next_ocr.png"
url2 = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/llava_next_comparison.png"

image1 = Image.open(requests.get(url1, stream=True).raw)
image2 = Image.open(requests.get(url2, stream=True).raw)

conversation = [
    {"role": "user", "content": [{"type": "image"}, {"type": "image"}, {"type": "text", "text": "Compare these two images and describe the differences."}]}
]
prompt = processor.apply_chat_template(conversation, add_generation_prompt=True)
inputs = processor([image1, image2], prompt, return_tensors="pt").to(model.device)

output = model.generate(**inputs, max_new_tokens=100)
print(processor.decode(output[0], skip_special_tokens=True))
```

## LlavaNextConfig[[transformers.LlavaNextConfig]]

#### transformers.LlavaNextConfig[[transformers.LlavaNextConfig]]

```python
transformers.LlavaNextConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vision_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, image_token_index: int = 32000, projector_hidden_act: str = 'gelu', vision_feature_select_strategy: typing.Literal['default', 'full'] = 'default', vision_feature_layer: int | list[int] = -2, multimodal_projector_bias: bool = True, tie_word_embeddings: bool = False, image_grid_pinpoints: list | None = None, image_seq_length: int = 576)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next/configuration_llava_next.py#L26)

**Parameters:**

vision_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the vision backbone.

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

image_token_index (`int`, *optional*, defaults to `32000`) : The image token index used as a placeholder for input images.

projector_hidden_act (`str`, *optional*, defaults to `gelu`) : The activation function used by the multimodal projector.

vision_feature_select_strategy (`Literal[default, full]`, *optional*, defaults to `default`) : The feature selection strategy used to select the vision feature from the vision backbone.

vision_feature_layer (`Union[int, list[int]]`, *optional*, defaults to `-2`) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

multimodal_projector_bias (`bool`, *optional*, defaults to `True`) : Whether to use bias in the multimodal projector.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

image_grid_pinpoints (`List`, *optional*, defaults to `[[336, 672], [672, 336], [672, 672], [1008, 336], [336, 1008]]`) : A list of possible resolutions to use for processing high resolution images. Each item in the list should be a tuple or list of the form `(height, width)`.

image_seq_length (`int`, *optional*, defaults to `576`) : Sequence length of one image embedding.

This is the configuration class to store the configuration of a LlavaNextModel. It is used to instantiate a Llava Next
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [llava-hf/llava-v1.6-mistral-7b-hf](https://huggingface.co/llava-hf/llava-v1.6-mistral-7b-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import LlavaNextForConditionalGeneration, LlavaNextConfig, CLIPVisionConfig, LlamaConfig

>>> # Initializing a CLIP-vision config
>>> vision_config = CLIPVisionConfig()

>>> # Initializing a Llama config
>>> text_config = LlamaConfig()

>>> # Initializing a Llava-Next llava-hf/llava-v1.6-mistral-7b-hf style configuration
>>> configuration = LlavaNextConfig(vision_config, text_config)

>>> # Initializing a model from the llava-hf/llava-v1.6-mistral-7b-hf style configuration
>>> model = LlavaNextForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## LlavaNextImageProcessor[[transformers.LlavaNextImageProcessor]]

#### transformers.LlavaNextImageProcessor[[transformers.LlavaNextImageProcessor]]

```python
transformers.LlavaNextImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next/image_processing_llava_next.py#L53)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'shortest_edge' : 224}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 224, 'width': 224}`): Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BICUBIC`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.48145466, 0.4578275, 0.40821073]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.26862954, 0.26130258, 0.27577711]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

image_grid_pinpoints (`list[list[int]]`, *kwargs*, *optional*) : A list of possible resolutions to use for processing high resolution images. The best resolution is selected based on the original size of the image. Can be overridden by `image_grid_pinpoints` in the `preprocess` method.

Constructs a LlavaNextImageProcessor image processor.

#### preprocess[[transformers.LlavaNextImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], list[typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']]]], *args, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next/image_processing_llava_next.py#L74)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], list[Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

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

image_grid_pinpoints (`list[list[int]]`, *kwargs*, *optional*) : A list of possible resolutions to use for processing high resolution images. The best resolution is selected based on the original size of the image. Can be overridden by `image_grid_pinpoints` in the `preprocess` method.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## LlavaNextImageProcessorPil[[transformers.LlavaNextImageProcessorPil]]

#### transformers.LlavaNextImageProcessorPil[[transformers.LlavaNextImageProcessorPil]]

```python
transformers.LlavaNextImageProcessorPil(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next/image_processing_pil_llava_next.py#L50)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'shortest_edge' : 224}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 224, 'width': 224}`): Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BICUBIC`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.48145466, 0.4578275, 0.40821073]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.26862954, 0.26130258, 0.27577711]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

image_grid_pinpoints (`list[list[int]]`, *kwargs*, *optional*) : A list of possible resolutions to use for processing high resolution images. The best resolution is selected based on the original size of the image. Can be overridden by `image_grid_pinpoints` in the `preprocess` method.

Constructs a LlavaNextImageProcessor image processor.

#### preprocess[[transformers.LlavaNextImageProcessorPil.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], list[typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']]]], *args, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next/image_processing_pil_llava_next.py#L71)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], list[Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

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

image_grid_pinpoints (`list[list[int]]`, *kwargs*, *optional*) : A list of possible resolutions to use for processing high resolution images. The best resolution is selected based on the original size of the image. Can be overridden by `image_grid_pinpoints` in the `preprocess` method.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## LlavaNextProcessor[[transformers.LlavaNextProcessor]]

#### transformers.LlavaNextProcessor[[transformers.LlavaNextProcessor]]

```python
transformers.LlavaNextProcessor(image_processor = None, tokenizer = None, patch_size = None, vision_feature_select_strategy = None, chat_template = None, image_token = '<image>', num_additional_image_tokens = 0, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next/processing_llava_next.py#L44)

**Parameters:**

image_processor (`LlavaNextImageProcessor`) : The image processor is a required input.

tokenizer (`TokenizersBackend`) : The tokenizer is a required input.

patch_size (`int`, *optional*) : Patch size from the vision tower.

vision_feature_select_strategy (`str`, *optional*) : The feature selection strategy used to select the vision feature from the vision backbone. Should be same as in model's config

chat_template (`str`) : A Jinja template to convert lists of messages in a chat into a tokenizable string.

image_token (`str`, *optional*, defaults to `"<image>"`) : Special token used to denote image location.

num_additional_image_tokens (`int`, *optional*, defaults to 0) : Number of additional tokens added to the image embeddings, such as CLS (+1). If the backbone has no CLS or other extra tokens appended, no need to set this arg.

Constructs a LlavaNextProcessor which wraps a image processor and a tokenizer into a single processor.

[LlavaNextProcessor](/docs/transformers/v5.15.1/en/model_doc/granitevision#transformers.LlavaNextProcessor) offers all the functionalities of [LlavaNextImageProcessor](/docs/transformers/v5.15.1/en/model_doc/granitevision#transformers.LlavaNextImageProcessor) and [TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend). See the
[~LlavaNextImageProcessor](/docs/transformers/v5.15.1/en/model_doc/granitevision#transformers.LlavaNextImageProcessor) and [~TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend) for more information.

#### __call__[[transformers.LlavaNextProcessor.__call__]]

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

## LlavaNextModel[[transformers.LlavaNextModel]]

#### transformers.LlavaNextModel[[transformers.LlavaNextModel]]

```python
transformers.LlavaNextModel(config: LlavaNextConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next/modeling_llava_next.py#L250)

**Parameters:**

config ([LlavaNextConfig](/docs/transformers/v5.15.1/en/model_doc/granitevision#transformers.LlavaNextConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Llava-Next model which consists of a vision backbone and a language model without language modeling head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.LlavaNextModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, image_sizes: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, vision_feature_layer: int | list[int] | None = None, vision_feature_select_strategy: str | None = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next/modeling_llava_next.py#L430)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [LlavaNextImageProcessor](/docs/transformers/v5.15.1/en/model_doc/granitevision#transformers.LlavaNextImageProcessor). See `LlavaNextImageProcessor.__call__()` for details ([LlavaNextProcessor](/docs/transformers/v5.15.1/en/model_doc/granitevision#transformers.LlavaNextProcessor) uses [LlavaNextImageProcessor](/docs/transformers/v5.15.1/en/model_doc/granitevision#transformers.LlavaNextImageProcessor) for processing images).

image_sizes (`torch.LongTensor` of shape `(batch_size, 2)`, *optional*) : The sizes of the images in the batch, being (height, width) for each image.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

vision_feature_layer (`Union[int, list[int]]`, *optional*) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

vision_feature_select_strategy (`str`, *optional*, defaults to `"default"`) : The feature selection strategy used to select the vision feature from the vision backbone. Can be one of `"default"` or `"full"`. If `"default"`, the CLS token is removed from the vision features. If `"full"`, the full vision features are used.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** `LlavaNextModelOutputWithPast` or `tuple(torch.FloatTensor)`

A `LlavaNextModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LlavaNextConfig](/docs/transformers/v5.15.1/en/model_doc/granitevision#transformers.LlavaNextConfig)) and inputs.

The [LlavaNextModel](/docs/transformers/v5.15.1/en/model_doc/llava_next#transformers.LlavaNextModel) forward method, overrides the `__call__` special method.

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

#### get_image_features[[transformers.LlavaNextModel.get_image_features]]

```python
get_image_features(pixel_values: FloatTensor, image_sizes: Tensor, vision_feature_layer: int | list[int] | None = None, vision_feature_select_strategy: str | None = None, output_hidden_states: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next/modeling_llava_next.py#L332)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_patches, channels, height, width)`) : The tensors corresponding to the input images.

image_sizes (`torch.Tensor` of shape `(num_images, 2)`) : Actual image size of each images (H, W).

vision_feature_layer (`Union[int, list[int]]`, *optional*) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

image_sizes (`torch.Tensor` of shape `(batch_size, 2)`) : The sizes of the images in the batch, being (height, width) for each image.

vision_feature_layer (`Union[int, list[int]]`, *optional*) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

vision_feature_select_strategy (`str`, *optional*) : The feature selection strategy used to select the vision feature from the vision backbone. Can be one of `"default"` or `"full"`

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LlavaNextConfig](/docs/transformers/v5.15.1/en/model_doc/granitevision#transformers.LlavaNextConfig)) and inputs.

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

#### get_placeholder_mask[[transformers.LlavaNextModel.get_placeholder_mask]]

```python
get_placeholder_mask(input_ids: LongTensor, inputs_embeds: FloatTensor, image_features: FloatTensor)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next/modeling_llava_next.py#L407)

Obtains multimodal placeholder mask from `input_ids` or `inputs_embeds`, and checks that the placeholder token count is
equal to the length of multimodal features. If the lengths are different, an error is raised.

#### pack_image_features[[transformers.LlavaNextModel.pack_image_features]]

```python
pack_image_features(image_features, image_sizes, vision_feature_select_strategy, image_newline = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next/modeling_llava_next.py#L265)

**Parameters:**

image_features (`list[torch.Tensor]` of length num_images, each of shape `(num_patches, image_length, embed_dim)`) : List of image feature tensor, each contains all the visual feature of all patches.

image_sizes (`torch.Tensor` of shape `(num_images, 2)`) : Actual image size of each images (H, W).

vision_feature_select_strategy (`str`) : The feature selection strategy used to select the vision feature from the vision backbone.

image_newline (`torch.Tensor` of shape `(embed_dim)`) : New line embedding vector.

**Returns:**

image_features (`torch.Tensor` of shape `(all_feat_len, embed_dim)`)
feature_lens (`list[int]`)
token length of each image in image_features

Reshape, unpad and then pack each image_feature into a single image_features tensor containing all visual vectors.

## LlavaNextForConditionalGeneration[[transformers.LlavaNextForConditionalGeneration]]

#### transformers.LlavaNextForConditionalGeneration[[transformers.LlavaNextForConditionalGeneration]]

```python
transformers.LlavaNextForConditionalGeneration(config: LlavaNextConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next/modeling_llava_next.py#L496)

**Parameters:**

config ([LlavaNextConfig](/docs/transformers/v5.15.1/en/model_doc/granitevision#transformers.LlavaNextConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The LLAVA-NeXT model which consists of a vision backbone and a language model.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.LlavaNextForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, image_sizes: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, vision_feature_layer: int | list[int] | None = None, vision_feature_select_strategy: str | None = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next/modeling_llava_next.py#L548)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [LlavaNextImageProcessor](/docs/transformers/v5.15.1/en/model_doc/granitevision#transformers.LlavaNextImageProcessor). See `LlavaNextImageProcessor.__call__()` for details ([LlavaNextProcessor](/docs/transformers/v5.15.1/en/model_doc/granitevision#transformers.LlavaNextProcessor) uses [LlavaNextImageProcessor](/docs/transformers/v5.15.1/en/model_doc/granitevision#transformers.LlavaNextImageProcessor) for processing images).

image_sizes (`torch.LongTensor` of shape `(batch_size, 2)`, *optional*) : The sizes of the images in the batch, being (height, width) for each image.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

vision_feature_layer (`Union[int, list[int]]`, *optional*) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

vision_feature_select_strategy (`str`, *optional*, defaults to `"default"`) : The feature selection strategy used to select the vision feature from the vision backbone. Can be one of `"default"` or `"full"`. If `"default"`, the CLS token is removed from the vision features. If `"full"`, the full vision features are used.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** `LlavaNextCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`

A `LlavaNextCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LlavaNextConfig](/docs/transformers/v5.15.1/en/model_doc/granitevision#transformers.LlavaNextConfig)) and inputs.

The [LlavaNextForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/granitevision#transformers.LlavaNextForConditionalGeneration) forward method, overrides the `__call__` special method.

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
- **image_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size (batch_size * num_patches, num_images, sequence_length, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.

Example:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoProcessor, LlavaNextForConditionalGeneration

>>> model = LlavaNextForConditionalGeneration.from_pretrained("llava-hf/llava-v1.6-mistral-7b-hf")
>>> processor = AutoProcessor.from_pretrained("llava-hf/llava-v1.6-mistral-7b-hf")

>>> prompt = "[INST] <image>\nWhat is shown in this image? [/INST]"
>>> url = "https://www.ilankelman.org/stopsigns/australia.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> inputs = processor(images=image, text=prompt, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(**inputs, max_length=30)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"[INST]  \nWhat is shown in this image? [/INST] The image appears to be a radar chart, which is a type of multi-dimensional plot (...)"
```

#### get_image_features[[transformers.LlavaNextForConditionalGeneration.get_image_features]]

```python
get_image_features(pixel_values: FloatTensor, image_sizes: Tensor, vision_feature_layer: int | list[int] | None = None, vision_feature_select_strategy: str | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next/modeling_llava_next.py#L516)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_patches, channels, height, width)`) : The tensors corresponding to the input images.

image_sizes (`torch.Tensor` of shape `(num_images, 2)`) : Actual image size of each images (H, W).

vision_feature_layer (`Union[int, list[int]]`, *optional*) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

image_sizes (`torch.Tensor` of shape `(batch_size, 2)`) : The sizes of the images in the batch, being (height, width) for each image.

vision_feature_layer (`Union[int, list[int]]`, *optional*) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

vision_feature_select_strategy (`str`, *optional*) : The feature selection strategy used to select the vision feature from the vision backbone. Can be one of `"default"` or `"full"`

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LlavaNextConfig](/docs/transformers/v5.15.1/en/model_doc/granitevision#transformers.LlavaNextConfig)) and inputs.

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
>>> from transformers import AutoProcessor, LlavaNextForConditionalGeneration

>>> model = LlavaNextForConditionalGeneration.from_pretrained("llava-hf/llava-v1.6-mistral-7b-hf")
>>> processor = AutoProcessor.from_pretrained("llava-hf/llava-v1.6-mistral-7b-hf")

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
