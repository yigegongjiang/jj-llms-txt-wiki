# Nougat

## Overview

The Nougat model was proposed in [Nougat: Neural Optical Understanding for Academic Documents](https://huggingface.co/papers/2308.13418) by
Lukas Blecher, Guillem Cucurull, Thomas Scialom, Robert Stojnic. Nougat uses the same architecture as [Donut](donut), meaning an image Transformer
encoder and an autoregressive text Transformer decoder to translate scientific PDFs to markdown, enabling easier access to them.

The abstract from the paper is the following:

*Scientific knowledge is predominantly stored in books and scientific journals, often in the form of PDFs. However, the PDF format leads to a loss of semantic information, particularly for mathematical expressions. We propose Nougat (Neural Optical Understanding for Academic Documents), a Visual Transformer model that performs an Optical Character Recognition (OCR) task for processing scientific documents into a markup language, and demonstrate the effectiveness of our model on a new dataset of scientific documents. The proposed approach offers a promising solution to enhance the accessibility of scientific knowledge in the digital age, by bridging the gap between human-readable documents and machine-readable text. We release the models and code to accelerate future work on scientific text recognition.*

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/nougat_architecture.jpg"
alt="drawing" width="600"/>

 Nougat high-level overview. Taken from the original paper. 

This model was contributed by [nielsr](https://huggingface.co/nielsr). The original code can be found
[here](https://github.com/facebookresearch/nougat).

## Usage tips

- The quickest way to get started with Nougat is by checking the [tutorial
  notebooks](https://github.com/NielsRogge/Transformers-Tutorials/tree/master/Nougat), which show how to use the model
  at inference time as well as fine-tuning on custom data.
- Nougat is always used within the [VisionEncoderDecoder](vision-encoder-decoder) framework. The model is identical to [Donut](donut) in terms of architecture.

## Inference

Nougat's `VisionEncoderDecoder` model accepts images as input and makes use of
[generate()](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationMixin.generate) to autoregressively generate text given the input image.

The [NougatImageProcessor](/docs/transformers/v5.15.1/en/model_doc/nougat#transformers.NougatImageProcessor) class is responsible for preprocessing the input image and
[NougatTokenizerFast](/docs/transformers/v5.15.1/en/model_doc/nougat#transformers.NougatTokenizer) decodes the generated target tokens to the target string. The
[NougatProcessor](/docs/transformers/v5.15.1/en/model_doc/nougat#transformers.NougatProcessor) wraps [NougatImageProcessor](/docs/transformers/v5.15.1/en/model_doc/nougat#transformers.NougatImageProcessor) and [NougatTokenizerFast](/docs/transformers/v5.15.1/en/model_doc/nougat#transformers.NougatTokenizer) classes
into a single instance to both extract the input features and decode the predicted token ids.

- Step-by-step PDF transcription

```python

from huggingface_hub import hf_hub_download
from PIL import Image

from transformers import AutoModelForImageTextToText, NougatProcessor

processor = NougatProcessor.from_pretrained("facebook/nougat-base")
model = AutoModelForImageTextToText.from_pretrained("facebook/nougat-base", device_map="auto")

model.to(model.device)  # doctest: +IGNORE_RESULT

# prepare PDF image for the model
filepath = hf_hub_download(repo_id="hf-internal-testing/fixtures_docvqa", filename="nougat_paper.png", repo_type="dataset")
image = Image.open(filepath)
pixel_values = processor(image, return_tensors="pt").to(model.device).pixel_values

# generate transcription (here we only generate 30 tokens)
outputs = model.generate(
    pixel_values.to(model.device),
    min_length=1,
    max_new_tokens=30,
    bad_words_ids=[[processor.tokenizer.unk_token_id]],
)

sequence = processor.batch_decode(outputs, skip_special_tokens=True)[0]
sequence = processor.post_process_generation(sequence, fix_markdown=False)
# note: we're using repr here such for the sake of printing the \n characters, feel free to just print the sequence
print(repr(sequence))
'\n\n# Nougat: Neural Optical Understanding for Academic Documents\n\n Lukas Blecher\n\nCorrespondence to: lblecher@'
```

See the [model hub](https://huggingface.co/models?filter=nougat) to look for Nougat checkpoints.

The model is identical to [Donut](donut) in terms of architecture.

## NougatConfig[[transformers.NougatConfig]]

#### transformers.NougatConfig[[transformers.NougatConfig]]

```python
transformers.NougatConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, is_encoder_decoder: bool = True, encoder: dict | transformers.configuration_utils.PreTrainedConfig | None = None, decoder: dict | transformers.configuration_utils.PreTrainedConfig | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nougat/configuration_nougat.py#L28)

**Parameters:**

is_encoder_decoder (`bool`, *optional*, defaults to `True`) : Whether the model is used as an encoder/decoder or not.

encoder (`dict | PreTrainedConfig`) : The config object or dictionary of the encoder backbone.

decoder (`dict | PreTrainedConfig`) : The config object or dictionary of the decoder backbone.

This is the configuration class to store the configuration of a NougatModel. It is used to instantiate a Nougat
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/nougat-base](https://huggingface.co/facebook/nougat-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import NougatConfig, VisionEncoderDecoderModel

>>> # Initializing a Nougat configuration
>>> config = NougatConfig()

>>> # Initializing a VisionEncoderDecoder model (with random weights) from a Nougat configurations
>>> model = VisionEncoderDecoderModel(config=config)
```

#### from_encoder_decoder_configs[[transformers.NougatConfig.from_encoder_decoder_configs]]

```python
from_encoder_decoder_configs(encoder_config: PreTrainedConfig, decoder_config: PreTrainedConfig, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nougat/configuration_nougat.py#L69)

**Returns:** [VisionEncoderDecoderConfig](/docs/transformers/v5.15.1/en/model_doc/vision-encoder-decoder#transformers.VisionEncoderDecoderConfig)

An instance of a configuration object

Instantiate a [VisionEncoderDecoderConfig](/docs/transformers/v5.15.1/en/model_doc/vision-encoder-decoder#transformers.VisionEncoderDecoderConfig) (or a derived class) from a pre-trained encoder model
configuration and decoder model configuration.

## NougatImageProcessor[[transformers.NougatImageProcessor]]

#### transformers.NougatImageProcessor[[transformers.NougatImageProcessor]]

```python
transformers.NougatImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nougat/image_processing_nougat.py#L57)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 896, 'width': 672}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BILINEAR`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.485, 0.456, 0.406]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.229, 0.224, 0.225]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

do_crop_margin (`bool`, *kwargs*, *optional*, defaults to `self.do_crop_margin`) : Whether to crop the image margins.

do_thumbnail (`bool`, *kwargs*, *optional*, defaults to `self.do_thumbnail`) : Whether to resize the image using thumbnail method.

do_align_long_axis (`bool`, *kwargs*, *optional*, defaults to `self.do_align_long_axis`) : Whether to align the long axis of the image with the long axis of `size` by rotating by 90 degrees.

Constructs a NougatImageProcessor image processor.

#### preprocess[[transformers.NougatImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nougat/image_processing_nougat.py#L74)

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

do_crop_margin (`bool`, *kwargs*, *optional*, defaults to `self.do_crop_margin`) : Whether to crop the image margins.

do_thumbnail (`bool`, *kwargs*, *optional*, defaults to `self.do_thumbnail`) : Whether to resize the image using thumbnail method.

do_align_long_axis (`bool`, *kwargs*, *optional*, defaults to `self.do_align_long_axis`) : Whether to align the long axis of the image with the long axis of `size` by rotating by 90 degrees.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## NougatImageProcessorPil[[transformers.NougatImageProcessorPil]]

#### transformers.NougatImageProcessorPil[[transformers.NougatImageProcessorPil]]

```python
transformers.NougatImageProcessorPil(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nougat/image_processing_pil_nougat.py#L59)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 896, 'width': 672}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BILINEAR`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.485, 0.456, 0.406]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.229, 0.224, 0.225]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

do_crop_margin (`bool`, *kwargs*, *optional*, defaults to `self.do_crop_margin`) : Whether to crop the image margins.

do_thumbnail (`bool`, *kwargs*, *optional*, defaults to `self.do_thumbnail`) : Whether to resize the image using thumbnail method.

do_align_long_axis (`bool`, *kwargs*, *optional*, defaults to `self.do_align_long_axis`) : Whether to align the long axis of the image with the long axis of `size` by rotating by 90 degrees.

Constructs a NougatImageProcessor image processor.

#### preprocess[[transformers.NougatImageProcessorPil.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nougat/image_processing_pil_nougat.py#L76)

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

do_crop_margin (`bool`, *kwargs*, *optional*, defaults to `self.do_crop_margin`) : Whether to crop the image margins.

do_thumbnail (`bool`, *kwargs*, *optional*, defaults to `self.do_thumbnail`) : Whether to resize the image using thumbnail method.

do_align_long_axis (`bool`, *kwargs*, *optional*, defaults to `self.do_align_long_axis`) : Whether to align the long axis of the image with the long axis of `size` by rotating by 90 degrees.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## NougatTokenizer[[transformers.NougatTokenizer]]

#### transformers.NougatTokenizer[[transformers.NougatTokenizer]]

```python
transformers.NougatTokenizer(errors: str = 'replace', unk_token: str = '<unk>', bos_token: str = '<s>', eos_token: str = '</s>', pad_token: str = '<pad>', vocab: str | dict | list | None = None, merges: str | list | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nougat/tokenization_nougat.py#L347)

**Parameters:**

vocab_file (`str`, *optional*) : Path to the vocabulary file.

merges_file (`str`, *optional*) : Path to the merges file.

tokenizer_file (`str`, *optional*) : [tokenizers](https://github.com/huggingface/tokenizers) file (generally has a .json extension) that contains everything needed to load the tokenizer. 

clean_up_tokenization_spaces (`str`, *optional*, defaults to `False`) : Whether to cleanup spaces after decoding, cleanup consists in removing potential artifacts like extra spaces. 

unk_token (`str`, *optional*, defaults to `"<unk>"`) : The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this token instead. 

bos_token (`str`, *optional*, defaults to `"<s>"`) : The beginning of sequence token that was used during pretraining. Can be used a sequence classifier token. 

eos_token (`str`, *optional*, defaults to `"</s>"`) : The end of sequence token. 

pad_token (`str`, *optional*, defaults to `"<pad>"`) : The token used for padding, for example when batching sequences of different lengths. 

vocab (`str`, `dict` or `list`, *optional*) : Custom vocabulary dictionary. If not provided, vocabulary is loaded from vocab_file. 

merges (`str` or `list`, *optional*) : Custom merges list. If not provided, merges are loaded from merges_file.

Tokenizer for Nougat (backed by HuggingFace tokenizers library).

This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should
refer to this superclass for more information regarding those methods. This class mainly adds Nougat-specific
methods for postprocessing the generated text.

#### correct_tables[[transformers.NougatTokenizer.correct_tables]]

```python
correct_tables(generation: str)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nougat/tokenization_nougat.py#L493)

**Parameters:**

generation (str) : The generated text to be postprocessed.

**Returns:** `str`

The postprocessed text.

Takes a generated string and fixes tables/tabulars to make them match the markdown format needed.

Example:

```python
correct_tables("\begin{table} \begin{tabular}{l l} & \ \end{tabular} \end{table}")
"\begin{table}
abular}{l l} & \ \end{tabular}
le}"
```

#### post_process_generation[[transformers.NougatTokenizer.post_process_generation]]

```python
post_process_generation(generation: str | list[str], fix_markdown: bool = True, num_workers: int | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nougat/tokenization_nougat.py#L623)

**Parameters:**

generation (Union[str, list[str]]) : The generated text or a list of generated texts.

fix_markdown (`bool`, *optional*, defaults to `True`) : Whether to perform Markdown formatting fixes.

num_workers (`int`, *optional*) : Optional number of workers to pass to leverage multiprocessing (postprocessing several texts in parallel).

**Returns:** Union[str, list[str]]

The postprocessed text or list of postprocessed texts.

Postprocess a generated text or a list of generated texts.

This function can be used to perform postprocessing on generated text, such as fixing Markdown formatting.

Postprocessing is quite slow so it is recommended to use multiprocessing to speed up the process.

#### post_process_single[[transformers.NougatTokenizer.post_process_single]]

```python
post_process_single(generation: str, fix_markdown: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nougat/tokenization_nougat.py#L528)

**Parameters:**

generation (str) : The generated text to be postprocessed.

fix_markdown (bool, optional) : Whether to perform Markdown formatting fixes. Default is True.

**Returns:** `str`

The postprocessed text.

Postprocess a single generated text. Regular expressions used here are taken directly from the Nougat article
authors. These expressions are commented for clarity and tested end-to-end in most cases.

#### remove_hallucinated_references[[transformers.NougatTokenizer.remove_hallucinated_references]]

```python
remove_hallucinated_references(text: str)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nougat/tokenization_nougat.py#L463)

**Parameters:**

text (`str`) : The input text containing references.

**Returns:** `str`

The text with hallucinated references removed.

Remove hallucinated or missing references from the text.

This function identifies and removes references that are marked as missing or hallucinated from the input text.

## NougatTokenizerFast[[transformers.NougatTokenizer]]

#### transformers.NougatTokenizer[[transformers.NougatTokenizer]]

```python
transformers.NougatTokenizer(errors: str = 'replace', unk_token: str = '<unk>', bos_token: str = '<s>', eos_token: str = '</s>', pad_token: str = '<pad>', vocab: str | dict | list | None = None, merges: str | list | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nougat/tokenization_nougat.py#L347)

**Parameters:**

vocab_file (`str`, *optional*) : Path to the vocabulary file.

merges_file (`str`, *optional*) : Path to the merges file.

tokenizer_file (`str`, *optional*) : [tokenizers](https://github.com/huggingface/tokenizers) file (generally has a .json extension) that contains everything needed to load the tokenizer. 

clean_up_tokenization_spaces (`str`, *optional*, defaults to `False`) : Whether to cleanup spaces after decoding, cleanup consists in removing potential artifacts like extra spaces. 

unk_token (`str`, *optional*, defaults to `"<unk>"`) : The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this token instead. 

bos_token (`str`, *optional*, defaults to `"<s>"`) : The beginning of sequence token that was used during pretraining. Can be used a sequence classifier token. 

eos_token (`str`, *optional*, defaults to `"</s>"`) : The end of sequence token. 

pad_token (`str`, *optional*, defaults to `"<pad>"`) : The token used for padding, for example when batching sequences of different lengths. 

vocab (`str`, `dict` or `list`, *optional*) : Custom vocabulary dictionary. If not provided, vocabulary is loaded from vocab_file. 

merges (`str` or `list`, *optional*) : Custom merges list. If not provided, merges are loaded from merges_file.

Tokenizer for Nougat (backed by HuggingFace tokenizers library).

This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should
refer to this superclass for more information regarding those methods. This class mainly adds Nougat-specific
methods for postprocessing the generated text.

#### correct_tables[[transformers.NougatTokenizer.correct_tables]]

```python
correct_tables(generation: str)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nougat/tokenization_nougat.py#L493)

**Parameters:**

generation (str) : The generated text to be postprocessed.

**Returns:** `str`

The postprocessed text.

Takes a generated string and fixes tables/tabulars to make them match the markdown format needed.

Example:

```python
correct_tables("\begin{table} \begin{tabular}{l l} & \ \end{tabular} \end{table}")
"\begin{table}
abular}{l l} & \ \end{tabular}
le}"
```

#### post_process_generation[[transformers.NougatTokenizer.post_process_generation]]

```python
post_process_generation(generation: str | list[str], fix_markdown: bool = True, num_workers: int | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nougat/tokenization_nougat.py#L623)

**Parameters:**

generation (Union[str, list[str]]) : The generated text or a list of generated texts.

fix_markdown (`bool`, *optional*, defaults to `True`) : Whether to perform Markdown formatting fixes.

num_workers (`int`, *optional*) : Optional number of workers to pass to leverage multiprocessing (postprocessing several texts in parallel).

**Returns:** Union[str, list[str]]

The postprocessed text or list of postprocessed texts.

Postprocess a generated text or a list of generated texts.

This function can be used to perform postprocessing on generated text, such as fixing Markdown formatting.

Postprocessing is quite slow so it is recommended to use multiprocessing to speed up the process.

#### post_process_single[[transformers.NougatTokenizer.post_process_single]]

```python
post_process_single(generation: str, fix_markdown: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nougat/tokenization_nougat.py#L528)

**Parameters:**

generation (str) : The generated text to be postprocessed.

fix_markdown (bool, optional) : Whether to perform Markdown formatting fixes. Default is True.

**Returns:** `str`

The postprocessed text.

Postprocess a single generated text. Regular expressions used here are taken directly from the Nougat article
authors. These expressions are commented for clarity and tested end-to-end in most cases.

#### remove_hallucinated_references[[transformers.NougatTokenizer.remove_hallucinated_references]]

```python
remove_hallucinated_references(text: str)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nougat/tokenization_nougat.py#L463)

**Parameters:**

text (`str`) : The input text containing references.

**Returns:** `str`

The text with hallucinated references removed.

Remove hallucinated or missing references from the text.

This function identifies and removes references that are marked as missing or hallucinated from the input text.

## NougatProcessor[[transformers.NougatProcessor]]

#### transformers.NougatProcessor[[transformers.NougatProcessor]]

```python
transformers.NougatProcessor(image_processor, tokenizer)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nougat/processing_nougat.py#L37)

**Parameters:**

image_processor (`NougatImageProcessor`) : The image processor is a required input.

tokenizer (`NougatTokenizer`) : The tokenizer is a required input.

Constructs a NougatProcessor which wraps a image processor and a tokenizer into a single processor.

[NougatProcessor](/docs/transformers/v5.15.1/en/model_doc/nougat#transformers.NougatProcessor) offers all the functionalities of [NougatImageProcessor](/docs/transformers/v5.15.1/en/model_doc/nougat#transformers.NougatImageProcessor) and [NougatTokenizer](/docs/transformers/v5.15.1/en/model_doc/nougat#transformers.NougatTokenizer). See the
[~NougatImageProcessor](/docs/transformers/v5.15.1/en/model_doc/nougat#transformers.NougatImageProcessor) and [~NougatTokenizer](/docs/transformers/v5.15.1/en/model_doc/nougat#transformers.NougatTokenizer) for more information.

#### __call__[[transformers.NougatProcessor.__call__]]

```python
__call__(images = None, text = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nougat/processing_nougat.py#L43)

**Parameters:**

images (``) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

text (``) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

#### from_pretrained[[transformers.NougatProcessor.from_pretrained]]

```python
from_pretrained(pretrained_model_name_or_path: str | os.PathLike, cache_dir: str | os.PathLike | None = None, force_download: bool = False, local_files_only: bool = False, token: str | bool | None = None, revision: str = 'main', **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/processing_utils.py#L1682)

**Parameters:**

pretrained_model_name_or_path (`str` or `os.PathLike`) : This can be either:  - a string, the *model id* of a pretrained feature_extractor hosted inside a model repo on huggingface.co. - a path to a *directory* containing a feature extractor file saved using the [save_pretrained()](/docs/transformers/v5.15.1/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin.save_pretrained) method, e.g., `./my_model_directory/`. - a path to a saved feature extractor JSON *file*, e.g., `./my_model_directory/preprocessor_config.json`.

- ****kwargs** : Additional keyword arguments passed along to both [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin.from_pretrained) and `~tokenization_utils_base.PreTrainedTokenizer.from_pretrained`.

Instantiate a processor associated with a pretrained model.

This class method is simply calling the feature extractor
[from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin.from_pretrained), image processor
[ImageProcessingMixin](/docs/transformers/v5.15.1/en/main_classes/image_processor#transformers.ImageProcessingMixin) and the tokenizer
`~tokenization_utils_base.PreTrainedTokenizer.from_pretrained` methods. Please refer to the docstrings of the
methods above for more information.

#### save_pretrained[[transformers.NougatProcessor.save_pretrained]]

```python
save_pretrained(save_directory, push_to_hub: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/processing_utils.py#L1107)

**Parameters:**

save_directory (`str` or `os.PathLike`) : Directory where the feature extractor JSON file and the tokenizer files will be saved (directory will be created if it does not exist).

push_to_hub (`bool`, *optional*, defaults to `False`) : Whether or not to push your model to the Hugging Face model hub after saving it. You can specify the repository you want to push to with `repo_id` (will default to the name of `save_directory` in your namespace).

kwargs (`dict[str, Any]`, *optional*) : Additional key word arguments passed along to the [push_to_hub()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.utils.PushToHubMixin.push_to_hub) method.

Saves the attributes of this processor (feature extractor, tokenizer...) in the specified directory so that it
can be reloaded using the [from_pretrained()](/docs/transformers/v5.15.1/en/model_doc/donut#transformers.DonutProcessor.from_pretrained) method.

This class method is simply calling [save_pretrained()](/docs/transformers/v5.15.1/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin.save_pretrained) and
[save_pretrained()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.save_pretrained). Please refer to the docstrings of the
methods above for more information.

#### batch_decode[[transformers.NougatProcessor.batch_decode]]

```python
batch_decode(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/processing_utils.py#L1930)

This method forwards all its arguments to PreTrainedTokenizer's [batch_decode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.batch_decode). Please
refer to the docstring of this method for more information.

#### decode[[transformers.NougatProcessor.decode]]

```python
decode(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/processing_utils.py#L1939)

This method forwards all its arguments to PreTrainedTokenizer's [decode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.decode). Please refer to
the docstring of this method for more information.

#### post_process_generation[[transformers.NougatProcessor.post_process_generation]]

```python
post_process_generation(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nougat/processing_nougat.py#L51)

This method forwards all its arguments to NougatTokenizer's `~PreTrainedTokenizer.post_process_generation`.
Please refer to the docstring of this method for more information.
