# Granite Vision

## Overview

The [Granite Vision](https://www.ibm.com/new/announcements/ibm-granite-3-1-powerful-performance-long-context-and-more) model is a variant of [LLaVA-NeXT](llava_next), leveraging a [Granite](granite) language model alongside a [SigLIP](siglip) visual encoder. It utilizes multiple concatenated vision hidden states as its image features, similar to [VipLlava](vipllava). It also uses a larger set of image grid pinpoints than the original LlaVa-NeXT models to support additional aspect ratios.

Tips:

- This model is loaded into Transformers as an instance of LlaVA-Next. The usage and tips from [LLaVA-NeXT](llava_next) apply to this model as well.

- You can apply the chat template on the tokenizer / processor in the same way as well. Example chat format:

```bash
"<|user|>\nWhat’s shown in this image?\n<|assistant|>\nThis image shows a red stop sign.<|end_of_text|><|user|>\nDescribe the image in more details.\n<|assistant|>\n"
```

Sample inference:

```python
from transformers import LlavaNextForConditionalGeneration, LlavaNextProcessor

model_path = "ibm-granite/granite-vision-3.1-2b-preview"
processor = LlavaNextProcessor.from_pretrained(model_path)

model = LlavaNextForConditionalGeneration.from_pretrained(model_path, device_map="auto")

# prepare image and text prompt, using the appropriate prompt template
url = "https://github.com/haotian-liu/LLaVA/blob/1a91fc274d7c35a9b50b3cb29c4247ae5837ce39/images/llava_v1_5_radar.jpg?raw=true"

conversation = [
    {
        "role": "user",
        "content": [
            {"type": "image", "url": url},
            {"type": "text", "text": "What is shown in this image?"},
        ],
    },
]
inputs = processor.apply_chat_template(
    conversation,
    add_generation_prompt=True,
    tokenize=True,
    return_dict=True,
    return_tensors="pt"
).to(model.device)

# autoregressively complete prompt
output = model.generate(**inputs, max_new_tokens=100)

print(processor.decode(output[0], skip_special_tokens=True))
```

This model was contributed by [Alexander Brooks](https://huggingface.co/abrooks9944).

## LlavaNextConfig[[transformers.LlavaNextConfig]]

- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **image_token_index** (`int`, *optional*, defaults to `32000`) --
  The image token index used as a placeholder for input images.
- **projector_hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The activation function used by the multimodal projector.
- **vision_feature_select_strategy** (`Literal[default, full]`, *optional*, defaults to `default`) --
  The feature selection strategy used to select the vision feature from the vision backbone.
- **vision_feature_layer** (`Union[int, list[int]]`, *optional*, defaults to `-2`) --
  The index of the layer to select the vision feature. If multiple indices are provided,
  the vision feature of the corresponding indices will be concatenated to form the
  vision features.
- **multimodal_projector_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to use bias in the multimodal projector.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **image_grid_pinpoints** (`List`, *optional*, defaults to `[[336, 672], [672, 336], [672, 672], [1008, 336], [336, 1008]]`) --
  A list of possible resolutions to use for processing high resolution images. Each item in the list should be a tuple or list
  of the form `(height, width)`.
- **image_seq_length** (`int`, *optional*, defaults to `576`) --
  Sequence length of one image embedding.

This is the configuration class to store the configuration of a LlavaNextModel. It is used to instantiate a Llava Next
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [llava-hf/llava-v1.6-mistral-7b-hf](https://huggingface.co/llava-hf/llava-v1.6-mistral-7b-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

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

- **image_grid_pinpoints** (`list[list[int]]`, *kwargs*, *optional*) --
  A list of possible resolutions to use for processing high resolution images. The best resolution is selected
  based on the original size of the image. Can be overridden by `image_grid_pinpoints` in the `preprocess`
  method.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a LlavaNextImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], list[Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **image_grid_pinpoints** (`list[list[int]]`, *kwargs*, *optional*) --
  A list of possible resolutions to use for processing high resolution images. The best resolution is selected
  based on the original size of the image. Can be overridden by `image_grid_pinpoints` in the `preprocess`
  method.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## LlavaNextProcessor[[transformers.LlavaNextProcessor]]

'"}, {"name": "num_additional_image_tokens", "val": " = 0"}, {"name": "**kwargs", "val": ""}]}>
- **image_processor** (`LlavaNextImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`TokenizersBackend`) --
  The tokenizer is a required input.
- **patch_size** (`int`, *optional*) --
  Patch size from the vision tower.
- **vision_feature_select_strategy** (`str`, *optional*) --
  The feature selection strategy used to select the vision feature from the vision backbone.
  Should be same as in model's config
- **chat_template** (`str`) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
- **image_token** (`str`, *optional*, defaults to `"<image>"`) --
  Special token used to denote image location.
- **num_additional_image_tokens** (`int`, *optional*, defaults to 0) --
  Number of additional tokens added to the image embeddings, such as CLS (+1). If the backbone has no CLS or other
  extra tokens appended, no need to set this arg.
Constructs a LlavaNextProcessor which wraps a image processor and a tokenizer into a single processor.

[LlavaNextProcessor](/docs/transformers/v5.14.0/en/model_doc/llava_next#transformers.LlavaNextProcessor) offers all the functionalities of [LlavaNextImageProcessor](/docs/transformers/v5.14.0/en/model_doc/llava_next#transformers.LlavaNextImageProcessor) and [TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend). See the
[~LlavaNextImageProcessor](/docs/transformers/v5.14.0/en/model_doc/llava_next#transformers.LlavaNextImageProcessor) and [~TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend) for more information.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **text** (`Union[str, list[str], list[list[str]]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.
- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) --
  Additional processing options for each modality (text, images, videos, audio). Model-specific parameters
  are listed above; see the TypedDict class for the complete list of supported arguments.[BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature)A [BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature) with the following fields:

- **input_ids** -- List of token ids to be fed to a model. Returned when `text` is not `None`.
- **attention_mask** -- List of indices specifying which tokens should be attended to by the model (when
  `return_attention_mask=True` or if *"attention_mask"* is in `self.model_input_names` and if `text` is not
  `None`).
- **pixel_values** -- Pixel values to be fed to a model. Returned when `images` is not `None`.

## LlavaNextForConditionalGeneration[[transformers.LlavaNextForConditionalGeneration]]

- **config** ([LlavaNextConfig](/docs/transformers/v5.14.0/en/model_doc/llava_next#transformers.LlavaNextConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The LLAVA-NeXT model which consists of a vision backbone and a language model.

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
  [LlavaNextImageProcessor](/docs/transformers/v5.14.0/en/model_doc/llava_next#transformers.LlavaNextImageProcessor). See `LlavaNextImageProcessor.__call__()` for details ([LlavaNextProcessor](/docs/transformers/v5.14.0/en/model_doc/llava_next#transformers.LlavaNextProcessor) uses
  [LlavaNextImageProcessor](/docs/transformers/v5.14.0/en/model_doc/llava_next#transformers.LlavaNextImageProcessor) for processing images).
- **image_sizes** (`torch.LongTensor` of shape `(batch_size, 2)`, *optional*) --
  The sizes of the images in the batch, being (height, width) for each image.
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
- **vision_feature_select_strategy** (`str`, *optional*, defaults to `"default"`) --
  The feature selection strategy used to select the vision feature from the vision backbone.
  Can be one of `"default"` or `"full"`. If `"default"`, the CLS token is removed from the vision features.
  If `"full"`, the full vision features are used.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).`LlavaNextCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `LlavaNextCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LlavaNextConfig](/docs/transformers/v5.14.0/en/model_doc/llava_next#transformers.LlavaNextConfig)) and inputs.
The [LlavaNextForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/llava_next#transformers.LlavaNextForConditionalGeneration) forward method, overrides the `__call__` special method.

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
