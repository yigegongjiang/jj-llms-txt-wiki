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
[generate()](/docs/transformers/v5.14.0/en/main_classes/text_generation#transformers.GenerationMixin.generate) to autoregressively generate text given the input image.

The [NougatImageProcessor](/docs/transformers/v5.14.0/en/model_doc/nougat#transformers.NougatImageProcessor) class is responsible for preprocessing the input image and
[NougatTokenizerFast](/docs/transformers/v5.14.0/en/model_doc/nougat#transformers.NougatTokenizer) decodes the generated target tokens to the target string. The
[NougatProcessor](/docs/transformers/v5.14.0/en/model_doc/nougat#transformers.NougatProcessor) wraps [NougatImageProcessor](/docs/transformers/v5.14.0/en/model_doc/nougat#transformers.NougatImageProcessor) and [NougatTokenizerFast](/docs/transformers/v5.14.0/en/model_doc/nougat#transformers.NougatTokenizer) classes
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

- **is_encoder_decoder** (`bool`, *optional*, defaults to `True`) --
  Whether the model is used as an encoder/decoder or not.
- **encoder** (`dict | PreTrainedConfig`) --
  The config object or dictionary of the encoder backbone.
- **decoder** (`dict | PreTrainedConfig`) --
  The config object or dictionary of the decoder backbone.

This is the configuration class to store the configuration of a NougatModel. It is used to instantiate a Nougat
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/nougat-base](https://huggingface.co/facebook/nougat-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import NougatConfig, VisionEncoderDecoderModel

>>> # Initializing a Nougat configuration
>>> config = NougatConfig()

>>> # Initializing a VisionEncoderDecoder model (with random weights) from a Nougat configurations
>>> model = VisionEncoderDecoderModel(config=config)
```

[VisionEncoderDecoderConfig](/docs/transformers/v5.14.0/en/model_doc/vision-encoder-decoder#transformers.VisionEncoderDecoderConfig)An instance of a configuration object

Instantiate a [VisionEncoderDecoderConfig](/docs/transformers/v5.14.0/en/model_doc/vision-encoder-decoder#transformers.VisionEncoderDecoderConfig) (or a derived class) from a pre-trained encoder model
configuration and decoder model configuration.

## NougatImageProcessor[[transformers.NougatImageProcessor]]

- **do_crop_margin** (`bool`, *kwargs*, *optional*, defaults to `self.do_crop_margin`) --
  Whether to crop the image margins.
- **do_thumbnail** (`bool`, *kwargs*, *optional*, defaults to `self.do_thumbnail`) --
  Whether to resize the image using thumbnail method.
- **do_align_long_axis** (`bool`, *kwargs*, *optional*, defaults to `self.do_align_long_axis`) --
  Whether to align the long axis of the image with the long axis of `size` by rotating by 90 degrees.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a NougatImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **do_crop_margin** (`bool`, *kwargs*, *optional*, defaults to `self.do_crop_margin`) --
  Whether to crop the image margins.
- **do_thumbnail** (`bool`, *kwargs*, *optional*, defaults to `self.do_thumbnail`) --
  Whether to resize the image using thumbnail method.
- **do_align_long_axis** (`bool`, *kwargs*, *optional*, defaults to `self.do_align_long_axis`) --
  Whether to align the long axis of the image with the long axis of `size` by rotating by 90 degrees.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## NougatImageProcessorPil[[transformers.NougatImageProcessorPil]]

- **do_crop_margin** (`bool`, *kwargs*, *optional*, defaults to `self.do_crop_margin`) --
  Whether to crop the image margins.
- **do_thumbnail** (`bool`, *kwargs*, *optional*, defaults to `self.do_thumbnail`) --
  Whether to resize the image using thumbnail method.
- **do_align_long_axis** (`bool`, *kwargs*, *optional*, defaults to `self.do_align_long_axis`) --
  Whether to align the long axis of the image with the long axis of `size` by rotating by 90 degrees.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a NougatImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **do_crop_margin** (`bool`, *kwargs*, *optional*, defaults to `self.do_crop_margin`) --
  Whether to crop the image margins.
- **do_thumbnail** (`bool`, *kwargs*, *optional*, defaults to `self.do_thumbnail`) --
  Whether to resize the image using thumbnail method.
- **do_align_long_axis** (`bool`, *kwargs*, *optional*, defaults to `self.do_align_long_axis`) --
  Whether to align the long axis of the image with the long axis of `size` by rotating by 90 degrees.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## NougatTokenizer[[transformers.NougatTokenizer]]

'"}, {"name": "bos_token", "val": ": str = ''"}, {"name": "eos_token", "val": ": str = ''"}, {"name": "pad_token", "val": ": str = ''"}, {"name": "vocab", "val": ": str | dict | list | None = None"}, {"name": "merges", "val": ": str | list | None = None"}, {"name": "**kwargs", "val": ""}]}>
- **vocab_file** (`str`, *optional*) --
  Path to the vocabulary file.
- **merges_file** (`str`, *optional*) --
  Path to the merges file.
- **tokenizer_file** (`str`, *optional*) --
  [tokenizers](https://github.com/huggingface/tokenizers) file (generally has a .json extension) that
  contains everything needed to load the tokenizer.

- **clean_up_tokenization_spaces** (`str`, *optional*, defaults to `False`) --
  Whether to cleanup spaces after decoding, cleanup consists in removing potential artifacts like extra
  spaces.

- **unk_token** (`str`, *optional*, defaults to `"<unk>"`) --
  The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this
  token instead.

- **bos_token** (`str`, *optional*, defaults to `"<s>"`) --
  The beginning of sequence token that was used during pretraining. Can be used a sequence classifier token.

- **eos_token** (`str`, *optional*, defaults to `"</s>"`) --
  The end of sequence token.

- **pad_token** (`str`, *optional*, defaults to `"<pad>"`) --
  The token used for padding, for example when batching sequences of different lengths.

- **vocab** (`str`, `dict` or `list`, *optional*) --
  Custom vocabulary dictionary. If not provided, vocabulary is loaded from vocab_file.

- **merges** (`str` or `list`, *optional*) --
  Custom merges list. If not provided, merges are loaded from merges_file.

Tokenizer for Nougat (backed by HuggingFace tokenizers library).

This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should
refer to this superclass for more information regarding those methods. This class mainly adds Nougat-specific
methods for postprocessing the generated text.

- **generation** (str) -- The generated text to be postprocessed.strThe postprocessed text.

Takes a generated string and fixes tables/tabulars to make them match the markdown format needed.

Example:

```python
correct_tables("\begin{table} \begin{tabular}{l l} & \ \end{tabular} \end{table}")
"\begin{table}
abular}{l l} & \ \end{tabular}
le}"
```

- **generation** (Union[str, list[str]]) --
  The generated text or a list of generated texts.
- **fix_markdown** (`bool`, *optional*, defaults to `True`) --
  Whether to perform Markdown formatting fixes.
- **num_workers** (`int`, *optional*) --
  Optional number of workers to pass to leverage multiprocessing (postprocessing several texts in
  parallel).Union[str, list[str]]The postprocessed text or list of postprocessed texts.

Postprocess a generated text or a list of generated texts.

This function can be used to perform postprocessing on generated text, such as fixing Markdown formatting.

Postprocessing is quite slow so it is recommended to use multiprocessing to speed up the process.

- **generation** (str) -- The generated text to be postprocessed.
- **fix_markdown** (bool, optional) -- Whether to perform Markdown formatting fixes. Default is True.strThe postprocessed text.

Postprocess a single generated text. Regular expressions used here are taken directly from the Nougat article
authors. These expressions are commented for clarity and tested end-to-end in most cases.

- **text** (`str`) --
  The input text containing references.`str`The text with hallucinated references removed.

Remove hallucinated or missing references from the text.

This function identifies and removes references that are marked as missing or hallucinated from the input text.

## NougatTokenizerFast[[transformers.NougatTokenizer]]

'"}, {"name": "bos_token", "val": ": str = ''"}, {"name": "eos_token", "val": ": str = ''"}, {"name": "pad_token", "val": ": str = ''"}, {"name": "vocab", "val": ": str | dict | list | None = None"}, {"name": "merges", "val": ": str | list | None = None"}, {"name": "**kwargs", "val": ""}]}>
- **vocab_file** (`str`, *optional*) --
  Path to the vocabulary file.
- **merges_file** (`str`, *optional*) --
  Path to the merges file.
- **tokenizer_file** (`str`, *optional*) --
  [tokenizers](https://github.com/huggingface/tokenizers) file (generally has a .json extension) that
  contains everything needed to load the tokenizer.

- **clean_up_tokenization_spaces** (`str`, *optional*, defaults to `False`) --
  Whether to cleanup spaces after decoding, cleanup consists in removing potential artifacts like extra
  spaces.

- **unk_token** (`str`, *optional*, defaults to `"<unk>"`) --
  The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this
  token instead.

- **bos_token** (`str`, *optional*, defaults to `"<s>"`) --
  The beginning of sequence token that was used during pretraining. Can be used a sequence classifier token.

- **eos_token** (`str`, *optional*, defaults to `"</s>"`) --
  The end of sequence token.

- **pad_token** (`str`, *optional*, defaults to `"<pad>"`) --
  The token used for padding, for example when batching sequences of different lengths.

- **vocab** (`str`, `dict` or `list`, *optional*) --
  Custom vocabulary dictionary. If not provided, vocabulary is loaded from vocab_file.

- **merges** (`str` or `list`, *optional*) --
  Custom merges list. If not provided, merges are loaded from merges_file.

Tokenizer for Nougat (backed by HuggingFace tokenizers library).

This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should
refer to this superclass for more information regarding those methods. This class mainly adds Nougat-specific
methods for postprocessing the generated text.

- **generation** (str) -- The generated text to be postprocessed.strThe postprocessed text.

Takes a generated string and fixes tables/tabulars to make them match the markdown format needed.

Example:

```python
correct_tables("\begin{table} \begin{tabular}{l l} & \ \end{tabular} \end{table}")
"\begin{table}
abular}{l l} & \ \end{tabular}
le}"
```

- **generation** (Union[str, list[str]]) --
  The generated text or a list of generated texts.
- **fix_markdown** (`bool`, *optional*, defaults to `True`) --
  Whether to perform Markdown formatting fixes.
- **num_workers** (`int`, *optional*) --
  Optional number of workers to pass to leverage multiprocessing (postprocessing several texts in
  parallel).Union[str, list[str]]The postprocessed text or list of postprocessed texts.

Postprocess a generated text or a list of generated texts.

This function can be used to perform postprocessing on generated text, such as fixing Markdown formatting.

Postprocessing is quite slow so it is recommended to use multiprocessing to speed up the process.

- **generation** (str) -- The generated text to be postprocessed.
- **fix_markdown** (bool, optional) -- Whether to perform Markdown formatting fixes. Default is True.strThe postprocessed text.

Postprocess a single generated text. Regular expressions used here are taken directly from the Nougat article
authors. These expressions are commented for clarity and tested end-to-end in most cases.

- **text** (`str`) --
  The input text containing references.`str`The text with hallucinated references removed.

Remove hallucinated or missing references from the text.

This function identifies and removes references that are marked as missing or hallucinated from the input text.

## NougatProcessor[[transformers.NougatProcessor]]

- **image_processor** (`NougatImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`NougatTokenizer`) --
  The tokenizer is a required input.
Constructs a NougatProcessor which wraps a image processor and a tokenizer into a single processor.

[NougatProcessor](/docs/transformers/v5.14.0/en/model_doc/nougat#transformers.NougatProcessor) offers all the functionalities of [NougatImageProcessor](/docs/transformers/v5.14.0/en/model_doc/nougat#transformers.NougatImageProcessor) and [NougatTokenizer](/docs/transformers/v5.14.0/en/model_doc/nougat#transformers.NougatTokenizer). See the
[~NougatImageProcessor](/docs/transformers/v5.14.0/en/model_doc/nougat#transformers.NougatImageProcessor) and [~NougatTokenizer](/docs/transformers/v5.14.0/en/model_doc/nougat#transformers.NougatTokenizer) for more information.

- **images** (``) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **text** (``) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **do_crop_margin** (`bool`, *optional*) --
  Whether to automatically crop white margins from document images. When enabled, the processor detects
  and removes white space around the edges of document pages, which is useful for processing scanned
  documents or PDFs with large margins.
- **do_resize** (`bool`, *optional*) --
  Whether to resize the image.
- **size** (`dict[str, int]`, *optional*) --
  Describes the maximum input dimensions to the model.
- **resample** (`PILImageResampling`, *optional*) --
  Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only
  has an effect if `do_resize` is set to `True`.
- **do_thumbnail** (`bool`, *optional*) --
  Whether to create a thumbnail version of the image. When enabled, a smaller version of the image is
  generated alongside the main processed image, which can be useful for preview or faster processing.
- **do_align_long_axis** (`bool`, *optional*) --
  Whether to automatically align images so that the longer axis is horizontal. When enabled, portrait
  images are rotated to landscape orientation, which is typically better for document processing tasks.
- **do_pad** (`bool`, *optional*) --
  Whether to pad the image. Padding is done either to the largest size in the batch
  or to a fixed square size per image. The exact padding strategy depends on the model.
- **do_rescale** (`bool`, *optional*) --
  Whether to rescale the image.
- **rescale_factor** (`Union[int, float]`, *optional*) --
  Rescale factor to rescale the image by if `do_rescale` is set to `True`.
- **do_normalize** (`bool`, *optional*) --
  Whether to normalize the image.
- **image_mean** (`Union[float, list[float]]`, *optional*) --
  Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.
- **image_std** (`Union[float, list[float]]`, *optional*) --
  Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to
  `True`.
- **data_format** (`ChannelDimension`, *optional*, defaults to `channels_first`) --
  Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.
- **input_data_format** (`Union[str, ChannelDimension]`, *optional*) --
  The channel dimension format for the input image. If unset, the channel dimension format is inferred
  from the input image. Can be one of:
  - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format.
  - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format.
  - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.
- **text_pair** (`str, list[str] or list[int]`, *optional*) --
  Optional second sequence to be encoded. This can be a string, a list of strings (tokenized string using
  the `tokenize` method) or a list of integers (tokenized string ids using the `convert_tokens_to_ids`
  method).
- **text_target** (`str, list[str] or list[list[str]]`, *optional*) --
  The sequence or batch of sequences to be encoded as target texts. Each sequence can be a string or a
  list of strings (pretokenized string). If you pass pretokenized input, set `is_split_into_words=True`
  to avoid ambiguity with batched inputs.
- **text_pair_target** (`str, list[str] or list[list[str]]`, *optional*) --
  The sequence or batch of sequences to be encoded as target texts. Each sequence can be a string or a
  list of strings (pretokenized string). If you pass pretokenized input, set `is_split_into_words=True`
  to avoid ambiguity with batched inputs.
- **add_special_tokens** (`bool`, *optional*, defaults to `True`) --
  Whether or not to add special tokens when encoding the sequences. This will use the underlying
  `PretrainedTokenizerBase.build_inputs_with_special_tokens` function, which defines which tokens are
  automatically added to the input ids. This is useful if you want to add `bos` or `eos` tokens
  automatically.
- **padding** (bool, str or [PaddingStrategy](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `False`) --
  Activates and controls padding. Accepts the following values:

  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single
    sequence is provided).
  - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum
    acceptable input length for the model if that argument is not provided.
  - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different
    lengths).
- **truncation** (bool, str or [TruncationStrategy](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.tokenization_utils_base.TruncationStrategy), *optional*) --
  Activates and controls truncation. Accepts the following values:

  - `True` or `'longest_first'`: Truncate to a maximum length specified with the argument `max_length` or
    to the maximum acceptable input length for the model if that argument is not provided. This will
    truncate token by token, removing a token from the longest sequence in the pair if a pair of
    sequences (or a batch of pairs) is provided.
  - `'only_first'`: Truncate to a maximum length specified with the argument `max_length` or to the
    maximum acceptable input length for the model if that argument is not provided. This will only
    truncate the first sequence of a pair if a pair of sequences (or a batch of pairs) is provided.
  - `'only_second'`: Truncate to a maximum length specified with the argument `max_length` or to the
    maximum acceptable input length for the model if that argument is not provided. This will only
    truncate the second sequence of a pair if a pair of sequences (or a batch of pairs) is provided.
  - `False` or `'do_not_truncate'` (default): No truncation (i.e., can output batch with sequence lengths
    greater than the model maximum admissible input size).
- **max_length** (`int`, *optional*) --
  Controls the maximum length to use by one of the truncation/padding parameters.

  If left unset or set to `None`, this will use the predefined model maximum length if a maximum length
  is required by one of the truncation/padding parameters. If the model has no specific maximum input
  length (like XLNet) truncation/padding to a maximum length will be deactivated.
- **stride** (`int`, *optional*, defaults to `0`) --
  If set to a number along with `max_length`, the overflowing tokens returned when
  `return_overflowing_tokens=True` will contain some tokens from the end of the truncated sequence
  returned to provide some overlap between truncated and overflowing sequences. The value of this
  argument defines the number of overlapping tokens.
- **is_split_into_words** (`bool`, *optional*, defaults to `False`) --
  Whether or not the input is already pre-tokenized (e.g., split into words). If set to `True`, the
  tokenizer assumes the input is already split into words (for instance, by splitting it on whitespace)
  which it will tokenize. This is useful for NER or token classification.
- **pad_to_multiple_of** (`int`, *optional*) --
  If set will pad the sequence to a multiple of the provided value. Requires `padding` to be activated.
  This is especially useful to enable using Tensor Cores on NVIDIA hardware with compute capability
  `>= 7.5` (Volta).
- **return_tensors** (`Union[str, ~utils.generic.TensorType]`, *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.
- **return_token_type_ids** (`bool`, *optional*) --
  Whether to return token type IDs. If left to the default, will return the token type IDs according to
  the specific tokenizer's default, defined by the `return_outputs` attribute.

  [What are token type IDs?](../glossary#token-type-ids)
- **return_attention_mask** (`bool`, *optional*) --
  Whether to return the attention mask. If left to the default, will return the attention mask according
  to the specific tokenizer's default, defined by the `return_outputs` attribute.

  [What are attention masks?](../glossary#attention-mask)
- **return_overflowing_tokens** (`bool`, *optional*, defaults to `False`) --
  Whether or not to return overflowing token sequences. If a pair of sequences of input ids (or a batch
  of pairs) is provided with `truncation_strategy = longest_first` or `True`, an error is raised instead
  of returning overflowing tokens.
- **return_special_tokens_mask** (`bool`, *optional*, defaults to `False`) --
  Whether or not to return special tokens mask information.
- **return_offsets_mapping** (`bool`, *optional*, defaults to `False`) --
  Whether or not to return `(char_start, char_end)` for each token.

  This is only available on fast tokenizers inheriting from [PreTrainedTokenizerFast](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend), if using
  Python's tokenizer, this method will raise `NotImplementedError`.
- **return_length** (`bool`, *optional*, defaults to `False`) --
  Whether or not to return the lengths of the encoded inputs.
- **verbose** (`bool`, *optional*, defaults to `True`) --
  Whether or not to print more information and warnings.

- **pretrained_model_name_or_path** (`str` or `os.PathLike`) --
  This can be either:

  - a string, the *model id* of a pretrained feature_extractor hosted inside a model repo on
    huggingface.co.
  - a path to a *directory* containing a feature extractor file saved using the
    [save_pretrained()](/docs/transformers/v5.14.0/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin.save_pretrained) method, e.g., `./my_model_directory/`.
  - a path to a saved feature extractor JSON *file*, e.g.,
    `./my_model_directory/preprocessor_config.json`.
- ****kwargs** --
  Additional keyword arguments passed along to both
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin.from_pretrained) and
  `~tokenization_utils_base.PreTrainedTokenizer.from_pretrained`.

Instantiate a processor associated with a pretrained model.

This class method is simply calling the feature extractor
[from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin.from_pretrained), image processor
[ImageProcessingMixin](/docs/transformers/v5.14.0/en/internal/image_processing_utils#transformers.ImageProcessingMixin) and the tokenizer
`~tokenization_utils_base.PreTrainedTokenizer.from_pretrained` methods. Please refer to the docstrings of the
methods above for more information.

- **save_directory** (`str` or `os.PathLike`) --
  Directory where the feature extractor JSON file and the tokenizer files will be saved (directory will
  be created if it does not exist).
- **push_to_hub** (`bool`, *optional*, defaults to `False`) --
  Whether or not to push your model to the Hugging Face model hub after saving it. You can specify the
  repository you want to push to with `repo_id` (will default to the name of `save_directory` in your
  namespace).
- **kwargs** (`dict[str, Any]`, *optional*) --
  Additional key word arguments passed along to the [push_to_hub()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.utils.PushToHubMixin.push_to_hub) method.

Saves the attributes of this processor (feature extractor, tokenizer...) in the specified directory so that it
can be reloaded using the [from_pretrained()](/docs/transformers/v5.14.0/en/model_doc/donut#transformers.DonutProcessor.from_pretrained) method.

This class method is simply calling [save_pretrained()](/docs/transformers/v5.14.0/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin.save_pretrained) and
[save_pretrained()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.save_pretrained). Please refer to the docstrings of the
methods above for more information.

This method forwards all its arguments to PreTrainedTokenizer's [batch_decode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.batch_decode). Please
refer to the docstring of this method for more information.

This method forwards all its arguments to PreTrainedTokenizer's [decode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.decode). Please refer to
the docstring of this method for more information.

This method forwards all its arguments to NougatTokenizer's `~PreTrainedTokenizer.post_process_generation`.
Please refer to the docstring of this method for more information.
