# QianfanOCR

## Overview

**Qianfan-OCR** is a 4B-parameter end-to-end document intelligence model developed by the Baidu Qianfan Team. It was proposed in [Qianfan-OCR: A Unified End-to-End Model for Document Intelligence](https://huggingface.co/papers/2603.13398) by Daxiang Dong et al.

Unlike traditional multi-stage OCR pipelines, Qianfan-OCR performs **direct image-to-text conversion** and supports a broad range of prompt-driven tasks — from structured document parsing and table extraction to chart understanding, document question answering, and key information extraction — all within one model.

The model adopts a multimodal bridging architecture consisting of three components:
- **Vision Encoder**: Qianfan-ViT with AnyResolution design (up to 4K), 256 visual tokens per 448×448 tile, max 4,096 tokens per image
- **Language Model**: Qwen3-4B with 32K context (extendable to 131K)
- **Cross-Modal Adapter**: 2-layer MLP with GELU activation

A key innovation is **Layout-as-Thought**: an optional thinking phase triggered by `<think>` tokens, where the model generates structured layout representations (bounding boxes, element types, reading order) before producing final outputs. This is particularly useful for heterogeneous pages with mixed element types (exam papers, technical reports, newspapers).

The model achieves state-of-the-art results on several benchmarks:
- **#1 end-to-end model on OmniDocBench v1.5** with an overall score of 93.12
- **#1 end-to-end model on OlmOCR Bench** with a score of 79.8
- **#1 on Key Information Extraction** with a mean score of 87.9 across five public KIE benchmarks

This model was contributed by the [Baidu Qianfan Team](https://github.com/baidubce/Qianfan-VL).

## Usage example

### Document parsing

```python
from transformers import AutoModelForImageTextToText, AutoProcessor

model = AutoModelForImageTextToText.from_pretrained("baidu/Qianfan-OCR", device_map="auto")
processor = AutoProcessor.from_pretrained("baidu/Qianfan-OCR")

image = "https://huggingface.co/datasets/hf-internal-testing/fixtures_got_ocr/resolve/main/image_ocr.jpg"
messages = [{"role": "user", "content": [{"type": "image", "url": image}, {"type": "text", "text": "Parse this document to Markdown."}]}]

inputs = processor.apply_chat_template(messages, add_generation_prompt=True, tokenize=True, return_tensors="pt").to(model.device)

generate_ids = model.generate(**inputs, max_new_tokens=64)
processor.decode(generate_ids[0, inputs["input_ids"].shape[1]:], skip_special_tokens=True)
```

### Layout-as-Thought (thinking mode)

For documents with complex layouts, cluttered elements, or non-standard reading orders, enable thinking mode by setting `enable_thinking=True` in `apply_chat_template`. The model will first generate structured layout analysis (bounding boxes, element types, reading order), then produce the final output.

```python
from transformers import AutoModelForImageTextToText, AutoProcessor

model = AutoModelForImageTextToText.from_pretrained("baidu/Qianfan-OCR", device_map="auto")
processor = AutoProcessor.from_pretrained("baidu/Qianfan-OCR")

image = "https://huggingface.co/datasets/hf-internal-testing/fixtures_got_ocr/resolve/main/image_ocr.jpg"
messages = [{"role": "user", "content": [{"type": "image", "url": image}, {"type": "text", "text": "Parse this document to Markdown."}]}]

inputs = processor.apply_chat_template(messages, add_generation_prompt=True, tokenize=True, return_tensors="pt", enable_thinking=True).to(model.device)

generate_ids = model.generate(**inputs, max_new_tokens=128)
processor.decode(generate_ids[0, inputs["input_ids"].shape[1]:], skip_special_tokens=True)
```

### Batched inference

```python
from transformers import AutoModelForImageTextToText, AutoProcessor

model = AutoModelForImageTextToText.from_pretrained("baidu/Qianfan-OCR", device_map="auto")
processor = AutoProcessor.from_pretrained("baidu/Qianfan-OCR")

image1 = "https://huggingface.co/datasets/hf-internal-testing/fixtures_got_ocr/resolve/main/image_ocr.jpg"
image2 = "https://huggingface.co/datasets/hf-internal-testing/fixtures_got_ocr/resolve/main/multi_box.png"
messages = [
    [{"role": "user", "content": [{"type": "image", "url": image1}, {"type": "text", "text": "Parse this document to Markdown."}]}],
    [{"role": "user", "content": [{"type": "image", "url": image2}, {"type": "text", "text": "OCR the text in the image."}]}],
]

inputs = processor.apply_chat_template(messages, add_generation_prompt=True, tokenize=True, return_tensors="pt", padding=True).to(model.device)

generate_ids = model.generate(**inputs, max_new_tokens=64)
processor.batch_decode(generate_ids[:, inputs["input_ids"].shape[1]:], skip_special_tokens=True)
```

## QianfanOCRConfig[[transformers.QianfanOCRConfig]]

- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **image_token_id** (`int`, *optional*, defaults to `151667`) --
  The image token index used as a placeholder for input images.
- **image_seq_length** (`int`, *optional*, defaults to `256`) --
  Sequence length of one image embedding.
- **downsample_ratio** (`float`, *optional*, defaults to 0.5) --
  Factor by which to downsample the image.
- **projector_hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The activation function used by the multimodal projector.
- **vision_feature_layer** (`Union[int, list[int]]`, *optional*, defaults to `-1`) --
  The index of the layer to select the vision feature. If multiple indices are provided,
  the vision feature of the corresponding indices will be concatenated to form the
  vision features.
- **vision_feature_select_strategy** (`str`, *optional*, defaults to `default`) --
  The feature selection strategy used to select the vision feature from the vision backbone.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a QianfanOCRModel. It is used to instantiate a Qianfan Ocr
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [baidu/Qianfan-OCR](https://huggingface.co/baidu/Qianfan-OCR)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> # Initializing a QianfanOCR style configuration
>>> configuration = QianfanOCRConfig()

>>> # Initializing a model from the configuration
>>> model = QianfanOCRForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## QianfanOCRVisionConfig[[transformers.QianfanOCRVisionConfig]]

- **hidden_size** (`int`, *optional*, defaults to `1024`) --
  Dimension of the hidden representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `24`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **attention_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to use a bias in the query, key, value and output projection layers during self-attention.
- **use_qk_norm** (`bool`, *optional*, defaults to `False`) --
  Whether to use query-key normalization in the attention.
- **intermediate_size** (`int`, *optional*, defaults to `4096`) --
  Dimension of the MLP representations.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **hidden_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **projection_dropout** (`float`, *optional*, defaults to 0.0) --
  Dropout probability for the projection layer.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **norm_type** (`str`, *optional*, defaults to `"layer_norm"`) --
  The type of normalization to use in the encoder. Can be `"layer_norm"` or `"rms_norm"`.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **image_size** (`Union[int, list[int], tuple[int, ...]]`, *optional*, defaults to `(448, 448)`) --
  The size (resolution) of each image.
- **patch_size** (`Union[int, list[int], tuple[int, ...]]`, *optional*, defaults to `(14, 14)`) --
  The size (resolution) of each patch.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **use_mask_token** (`bool`, *optional*, defaults to `False`) --
  Whether to use a mask token for masked image modeling.
- **use_absolute_position_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to use absolute position embeddings.
- **layer_scale_init_value** (`float`, *optional*, defaults to `0.1`) --
  Scale to use in the self-attention layers. 0.1 for base, 1e-6 for large. Set 0 to disable layer scale.
- **use_mean_pooling** (`bool`, *optional*, defaults to `True`) --
  Whether to mean pool the final hidden states of the patches instead of using the final hidden state of the
  CLS token, before applying the classification head.
- **drop_path_rate** (`float`, *optional*, defaults to 0.1) --
  Dropout rate for stochastic depth.

This is the configuration class to store the configuration of a QianfanOCRModel. It is used to instantiate a Qianfan Ocr
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [baidu/Qianfan-OCR](https://huggingface.co/baidu/Qianfan-OCR)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> # Initializing a QianfanOCR vision style configuration
>>> configuration = QianfanOCRVisionConfig()

>>> # Initializing a model from the configuration
>>> model = QianfanOCRVisionModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## QianfanOCRProcessor[[transformers.QianfanOCRProcessor]]

'"}, {"name": "**kwargs", "val": ""}]}>
- **image_processor** (*GotOcr2ImageProcessor*) --
  The image processor is a required input.
- **tokenizer** (*Qwen2Tokenizer*) --
  The tokenizer is a required input.
- **image_seq_length** (*int*, *optional*, defaults to *256*) --
  The number of image tokens to be used for each image in the input.
  Added for backward compatibility but this should be set as a processor attribute in future models.
- **chat_template** (*str*) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
- **image_placeholder_token** (*str*, *optional*, defaults to *"&amp;lt;image>"*) --
  The token emitted by the chat template to mark image positions.
  It is replaced by the full `<img>&amp;lt;IMG_CONTEXT>...&amp;lt;IMG_CONTEXT>&amp;lt;/img>`
  sequence during processing.
Constructs a QianfanOCRProcessor which wraps a image processor and a tokenizer into a single processor.

[*QianfanOCRProcessor*] offers all the functionalities of [*GotOcr2ImageProcessor*] and [*Qwen2Tokenizer*]. See the
[*~GotOcr2ImageProcessor*] and [*~Qwen2Tokenizer*] for more information.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **text** (`Union[str, list[str], list[list[str]]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **videos** (``) --
  Video to preprocess. Expects a single or batch of videos with pixel values ranging from 0 to 255. If
  passing in videos with pixel values between 0 and 1, set `do_rescale=False`.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.[BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature)A [BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature) with the following fields:

- **input_ids** -- List of token ids to be fed to a model. Returned when `text` is not `None`.
- **attention_mask** -- List of indices specifying which tokens should be attended to by the model (when
  `return_attention_mask=True` or if *"attention_mask"* is in `self.model_input_names` and if `text` is not
  `None`).
- **pixel_values** -- Pixel values to be fed to a model. Returned when `images` is not `None`.

## QianfanOCRVisionModel[[transformers.QianfanOCRVisionModel]]

- **config** ([QianfanOCRVisionConfig](/docs/transformers/v5.14.0/en/model_doc/qianfan_ocr#transformers.QianfanOCRVisionConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Qianfan Ocr Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "bool_masked_pos", "val": ": typing.Optional[torch.BoolTensor] = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor). See `GotOcr2ImageProcessor.__call__()` for details ([QianfanOCRProcessor](/docs/transformers/v5.14.0/en/model_doc/qianfan_ocr#transformers.QianfanOCRProcessor) uses
  [GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor) for processing images).
- **bool_masked_pos** (`torch.BoolTensor` of shape `(batch_size, num_patches)`, *optional*) --
  Boolean masked positions. Indicates which patches are masked (1) and which aren't (0).`QianfanOCRVisionModelOutputWithPooling` or `tuple(torch.FloatTensor)`A `QianfanOCRVisionModelOutputWithPooling` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([QianfanOCRConfig](/docs/transformers/v5.14.0/en/model_doc/qianfan_ocr#transformers.QianfanOCRConfig)) and inputs.
The [QianfanOCRVisionModel](/docs/transformers/v5.14.0/en/model_doc/qianfan_ocr#transformers.QianfanOCRVisionModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Average of the last layer hidden states of the patch tokens (excluding the *[CLS]* token) if
  *config.use_mean_pooling* is set to True. If set to False, then the final hidden state of the *[CLS]* token
  will be returned.

## QianfanOCRModel[[transformers.QianfanOCRModel]]

- **config** ([QianfanOCRConfig](/docs/transformers/v5.14.0/en/model_doc/qianfan_ocr#transformers.QianfanOCRConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The QianfanOCR model which consists of a vision backbone and a language model, without a language modeling head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor). See `GotOcr2ImageProcessor.__call__()` for details ([QianfanOCRProcessor](/docs/transformers/v5.14.0/en/model_doc/qianfan_ocr#transformers.QianfanOCRProcessor) uses
  [GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor) for processing images).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **past_key_values** (`~cache_utils.Cache`, *optional*) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **vision_feature_layer** (`Union[int, list[int]]`, *optional*) --
  The index of the layer to select the vision feature. If multiple indices are provided,
  the vision feature of the corresponding indices will be concatenated to form the
  vision features.
- **vision_feature_select_strategy** (`str`, *optional*) --
  The feature selection strategy used to select the vision feature from the vision backbone.
  Can be one of `"default"` or `"full"`.`QianfanOCRModelOutputWithPast` or `tuple(torch.FloatTensor)`A `QianfanOCRModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([QianfanOCRConfig](/docs/transformers/v5.14.0/en/model_doc/qianfan_ocr#transformers.QianfanOCRConfig)) and inputs.
The [QianfanOCRModel](/docs/transformers/v5.14.0/en/model_doc/qianfan_ocr#transformers.QianfanOCRModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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
- **image_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(batch_size, num_images, sequence_length, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.

## QianfanOCRForConditionalGeneration[[transformers.QianfanOCRForConditionalGeneration]]

- **config** ([QianfanOCRConfig](/docs/transformers/v5.14.0/en/model_doc/qianfan_ocr#transformers.QianfanOCRConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The QIANFAN_OCR model which consists of a vision backbone and a language model.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor). See `GotOcr2ImageProcessor.__call__()` for details ([QianfanOCRProcessor](/docs/transformers/v5.14.0/en/model_doc/qianfan_ocr#transformers.QianfanOCRProcessor) uses
  [GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor) for processing images).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **past_key_values** (`~cache_utils.Cache`, *optional*) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **vision_feature_layer** (`Union[int, list[int]]`, *optional*) --
  The index of the layer to select the vision feature. If multiple indices are provided,
  the vision feature of the corresponding indices will be concatenated to form the
  vision features.
- **vision_feature_select_strategy** (`str`, *optional*) --
  The feature selection strategy used to select the vision feature from the vision backbone.
  Can be one of `"default"` or `"full"`.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).
- **image_sizes** (`torch.Tensor` of shape `(batch_size, 2)`, *optional*) --
  The sizes of the images in the batch, being (height, width) for each image.`QianfanOCRCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `QianfanOCRCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([QianfanOCRConfig](/docs/transformers/v5.14.0/en/model_doc/qianfan_ocr#transformers.QianfanOCRConfig)) and inputs.
The [QianfanOCRForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/qianfan_ocr#transformers.QianfanOCRForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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
>>> import torch
>>> from transformers import AutoProcessor, AutoModelForImageTextToText
>>> torch_device = "cuda"
>>> processor = AutoProcessor.from_pretrained("baidu/Qianfan-OCR")
>>> model = AutoModelForImageTextToText.from_pretrained(
...     "baidu/Qianfan-OCR", dtype=torch.bfloat16, device_map=torch_device
... )
>>> messages = [
...     {
...         "role": "user",
...         "content": [
...             {"type": "image", "url": "https://example.com/image.jpg"},
...             {"type": "text", "text": "Describe this image."},
...         ],
...     },
... ]
>>> inputs = processor.apply_chat_template(messages, add_generation_prompt=True, tokenize=True, return_dict=True, return_tensors="pt").to(torch_device)
>>> generate_ids = model.generate(**inputs, max_new_tokens=200)
>>> print(processor.decode(generate_ids[0, inputs["input_ids"].shape[1] :], skip_special_tokens=True))
```
