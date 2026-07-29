# Mllama

## Overview

The [Llama 3.2-Vision](https://ai.meta.com/blog/llama-3-2-connect-2024-vision-edge-mobile-devices/) collection of multimodal large language models (LLMs) is a collection of pretrained and instruction-tuned image reasoning generative models in 11B and 90B sizes (text \+ images in / text out). The Llama 3.2-Vision instruction-tuned models are optimized for visual recognition, image reasoning, captioning, and answering general questions about an image.

**Model Architecture:** Llama 3.2-Vision is built on top of Llama 3.1 text-only model, which is an auto-regressive language model that uses an optimized transformer architecture. The tuned versions use supervised fine-tuning (SFT) and reinforcement learning with human feedback (RLHF) to align with human preferences for helpfulness and safety. To support image recognition tasks, the Llama 3.2-Vision model uses a separately trained vision adapter that integrates with the pre-trained Llama 3.1 language model. The adapter consists of a series of cross-attention layers that feed image encoder representations into the core LLM.

## Usage Tips

- For image+text and text inputs use `MllamaForConditionalGeneration`.
- For text-only inputs use `MllamaForCausalLM` for generation to avoid loading vision tower.
- Each sample can contain multiple images, and the number of images can vary between samples. The processor will pad the inputs to the maximum number of images across samples and to a maximum number of tiles within each image.
- The text passed to the processor should have the `"<|image|>"` tokens where the images should be inserted.
- The processor has its own `apply_chat_template` method to convert chat messages to text that can then be passed as text to the processor. If you're using `transformers>=4.49.0`, you can also get a vectorized output from `apply_chat_template`. See the **Usage Examples** below for more details on how to use it.

Mllama has an extra token used as a placeholder for image positions in the text. It means that input ids and an input embedding layer will have an extra token. But since the weights for input and output embeddings are not tied, the `lm_head` layer has one less token and will fail if you want to calculate loss on image tokens or apply some logit processors. In case you are training, make sure to mask out special `"<|image|>"` tokens in the `labels` as the model should not be trained on predicting them.

Otherwise if you see CUDA-side index errors when generating, use the below code to expand the `lm_head` by one more token.

```python
old_embeddings = model.get_output_embeddings()

num_tokens = model.vocab_size + 1
resized_embeddings = model._get_resized_lm_head(old_embeddings, new_num_tokens=num_tokens, mean_resizing=True)
resized_embeddings.requires_grad_(old_embeddings.weight.requires_grad)
model.set_output_embeddings(resized_embeddings)
```

## Usage Example

### Instruct model

```python
from transformers import AutoProcessor, MllamaForConditionalGeneration

model_id = "meta-llama/Llama-3.2-11B-Vision-Instruct"
model = MllamaForConditionalGeneration.from_pretrained(model_id, device_map="auto")
processor = AutoProcessor.from_pretrained(model_id)

messages = [
    [
        {
            "role": "user",
            "content": [
                {"type": "image", "url": "https://llava-vl.github.io/static/images/view.jpg"},
                {"type": "text", "text": "What does the image show?"}
            ]
        }
    ],
]
inputs = processor.apply_chat_template(messages, add_generation_prompt=True, tokenize=True, return_dict=True, return_tensors="pt").to(model.device)
output = model.generate(**inputs, max_new_tokens=25)
print(processor.decode(output[0]))
```

### Base model

```python
import requests
from PIL import Image

from transformers import AutoProcessor, MllamaForConditionalGeneration

model_id = "meta-llama/Llama-3.2-11B-Vision"
model = MllamaForConditionalGeneration.from_pretrained(model_id, device_map="auto")
processor = AutoProcessor.from_pretrained(model_id)

prompt = "<|image|>If I had to write a haiku for this one"
url = "https://llava-vl.github.io/static/images/view.jpg"
raw_image = Image.open(requests.get(url, stream=True).raw)

inputs = processor(text=prompt, images=raw_image, return_tensors="pt").to(model.device)
output = model.generate(**inputs, do_sample=False, max_new_tokens=25)
print(processor.decode(output[0], skip_special_tokens=True))
```

## MllamaConfig[[transformers.MllamaConfig]]

- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **image_token_index** (`int`, *optional*, defaults to `128256`) --
  The image token index used as a placeholder for input images.

This is the configuration class to store the configuration of a MllamaModel. It is used to instantiate a Mllama
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [meta-llama/Llama-3.2-11B-Vision](https://huggingface.co/meta-llama/Llama-3.2-11B-Vision)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import MllamaForConditionalGeneration, MllamaConfig, MllamaVisionConfig, MllamaTextConfig

>>> # Initializing a CLIP-vision config
>>> vision_config = MllamaVisionConfig()

>>> # Initializing a Llama config
>>> text_config = MllamaTextConfig()

>>> # Initializing a mllama-11b style configuration
>>> configuration = MllamaConfig(vision_config, text_config)

>>> # Initializing a model from the mllama-11b style configuration
>>> model = MllamaForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## MllamaTextConfig[[transformers.MllamaTextConfig]]

- **vocab_size** (`int`, *optional*, defaults to `128256`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `4096`) --
  Dimension of the hidden representations.
- **hidden_act** (`str`, *optional*, defaults to `silu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **num_hidden_layers** (`int`, *optional*, defaults to `40`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `32`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_key_value_heads** (`int`, *optional*, defaults to `8`) --
  This is the number of key_value heads that should be used to implement Grouped Query Attention. If
  `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if
  `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When
  converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed
  by meanpooling all the original heads within that group. For more details, check out [this
  paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to
  `num_attention_heads`.
- **intermediate_size** (`int`, *optional*, defaults to `14336`) --
  Dimension of the MLP representations.
- **rope_parameters** (`dict`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the rms normalization layers.
- **max_position_embeddings** (`int`, *optional*, defaults to `131072`) --
  The maximum sequence length that this model might ever be used with.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **cross_attention_layers** (`list[int]`, *optional*) --
  Indices of the cross attention layers. If not specified, will default to [3, 8, 13, 18, 23, 28, 33, 38].
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The ratio for all dropout layers.
- **bos_token_id** (`int`, *optional*, defaults to `128000`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `128001`) --
  Token id used for end-of-stream in the vocabulary.
- **pad_token_id** (`int`, *optional*, defaults to `128004`) --
  Token id used for padding in the vocabulary.

This is the configuration class to store the configuration of a MllamaModel. It is used to instantiate a Mllama
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [meta-llama/Llama-3.2-11B-Vision](https://huggingface.co/meta-llama/Llama-3.2-11B-Vision)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import MllamaTextModel, MllamaTextConfig

>>> # Initializing a Mllama text config
>>> config = MllamaTextConfig()

>>> # Initializing a model from the Mllama text configuration
>>> model = MllamaTextModel(config)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## MllamaVisionConfig[[transformers.MllamaVisionConfig]]

- **hidden_size** (`int`, *optional*, defaults to `1280`) --
  Dimension of the hidden representations.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **num_hidden_layers** (`int`, *optional*, defaults to `32`) --
  Number of hidden layers in the Transformer decoder.
- **num_global_layers** (`int`, *optional*, defaults to 8) --
  Number of global layers in the Transformer encoder. Vision model has a second transformer encoder, called global.
- **attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **intermediate_size** (`int`, *optional*, defaults to `5120`) --
  Dimension of the MLP representations.
- **vision_output_dim** (`int`, *optional*, defaults to 7680) --
  Dimensionality of the vision model output. Includes output of transformer
  encoder with intermediate layers and global transformer encoder.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `448`) --
  The size (resolution) of each image.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `14`) --
  The size (resolution) of each patch.
- **norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the layer normalization layers.
- **max_num_tiles** (`int`, *optional*, defaults to 4) --
  Maximum number of tiles for image splitting.
- **intermediate_layers_indices** (`list[int]`, *optional*, defaults to [3, 7, 15, 23, 30]) --
  Indices of intermediate layers of transformer encoder from which to extract and output features.
  These output features are concatenated with final hidden state of transformer encoder.
- **supported_aspect_ratios** (`list[list[int]]`, *optional*) --
  List of supported aspect ratios for image splitting. If not specified, the default supported aspect ratios
  are [[1, 1], [1, 2], [1, 3], [1, 4], [2, 1], [2, 2], [3, 1], [4, 1]] for `max_num_tiles=4`.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a MllamaModel. It is used to instantiate a Mllama
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [meta-llama/Llama-3.2-11B-Vision](https://huggingface.co/meta-llama/Llama-3.2-11B-Vision)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import MllamaVisionConfig, MllamaVisionModel

>>> # Initializing a Llama config
>>> config = MllamaVisionConfig()

>>> # Initializing a vision model from the mllama-11b style configuration
>>> model = MllamaVisionModel(config)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## MllamaProcessor[[transformers.MllamaProcessor]]

- **image_processor** (`MllamaImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`tokenizer_class`) --
  The tokenizer is a required input.
- **chat_template** (`str`) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
Constructs a MllamaProcessor which wraps a image processor and a tokenizer into a single processor.

[MllamaProcessor](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaProcessor) offers all the functionalities of [MllamaImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaImageProcessor) and `tokenizer_class`. See the
[~MllamaImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaImageProcessor) and `~tokenizer_class` for more information.

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
TODO: add aspect_ratio_ids and aspect_ratio_mask and cross_attention_mask

## MllamaImageProcessor[[transformers.MllamaImageProcessor]]

- **max_image_tiles** (`int`, *kwargs*, *optional*) --
  The maximum number of tiles allowed.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a MllamaImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **max_image_tiles** (`int`, *kwargs*, *optional*) --
  The maximum number of tiles allowed.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## MllamaImageProcessorPil[[transformers.MllamaImageProcessorPil]]

- **max_image_tiles** (`int`, *kwargs*, *optional*) --
  The maximum number of tiles allowed.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a MllamaImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **max_image_tiles** (`int`, *kwargs*, *optional*) --
  The maximum number of tiles allowed.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## MllamaForConditionalGeneration[[transformers.MllamaForConditionalGeneration]]

- **config** ([MllamaConfig](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Mllama model which consists of a vision encoder and a language model.

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
  [MllamaImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaImageProcessor). See `MllamaImageProcessor.__call__()` for details ([MllamaProcessor](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaProcessor) uses
  [MllamaImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaImageProcessor) for processing images).
- **aspect_ratio_mask** (`torch.Tensor` of shape `(batch_size, max_num_images, max_num_tiles)`, *optional*) --
  Mask to avoid performing attention on padding tiles. Mask values selected in `[0, 1]`:

  - 1 for tiles that are **not masked**,
  - 0 for tiles that are **masked**.
- **aspect_ratio_ids** (`torch.Tensor` of shape `(batch_size, max_num_images)`, *optional*) --
  Aspect ratio ids used to select the appropriate precomputed tile embeddings based on the aspect ratio of each input image.
  These ids correspond to indices in the model's list of supported aspect ratios, offset by 1.

  For example, if the model supports aspect ratios [[1, 1], [1, 2], [2, 1]]:
  - An image with aspect ratio [1, 1] would have ID 1
  - An image with aspect ratio [1, 2] would have ID 2
  - An image with aspect ratio [2, 1] would have ID 3

  The id 0 is reserved for padding (i.e., no image).

  If an image has aspect ratio [1, 2], that means it was split into 2 tiles horizontally, and its `aspect_ratio_id` would be 2.
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **cross_attention_mask** (`torch.Tensor` of shape `(batch_size, seq_length, max_num_images, max_num_tiles)`, *optional*) --
  Cross-attention mask to control the interaction between text tokens and image tiles.
  This 4D tensor defines which image tiles each text token should attend to.

  For each text token (in seq_length):
  - 1 indicates the token **should attend** to the corresponding image tile
  - 0 indicates the token **should not attend** to the corresponding image tile
- **cross_attention_states** (`torch.FloatTensor`, *optional*) --
  Output of the vision model, used for cross-attention. This tensor contains the processed image features that
  the language model will attend to.
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
  This is useful when using packed tensor format (single dimension for batch and sequence length).[CausalLMOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or `tuple(torch.FloatTensor)`A [CausalLMOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MllamaConfig](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaConfig)) and inputs.
The [MllamaForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoProcessor, MllamaForConditionalGeneration

>>> checkpoint = "meta-llama/Llama-3.2-11B-Vision"
>>> model = MllamaForConditionalGeneration.from_pretrained(checkpoint)
>>> processor = AutoProcessor.from_pretrained(checkpoint)

>>> prompt = "<|image|>If I had to write a haiku for this one"
>>> url = "https://www.ilankelman.org/stopsigns/australia.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> inputs = processor(text=prompt, images=image, return_tensors="pt")

>>> # Generate
>>> output = model.generate(**inputs, max_new_tokens=15)

>>> prompt_len = inputs.input_ids.shape[-1]
>>> generated_ids = output[:, prompt_len:]
>>> generated_text = processor.batch_decode(generated_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)
>>> print(generated_text)
[', it would be:.\\nA stop sign in Chinatown.\\n']
```

## MllamaForCausalLM[[transformers.MllamaForCausalLM]]

- **config** ([MllamaForCausalLM](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaForCausalLM)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Mllama Text Model with a language modeling head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>, )>] | None = None"}, {"name": "past_key_values", "val": ": transformers.cache_utils.Cache | None = None"}, {"name": "inputs_embeds", "val": ": typing.Optional[torch.FloatTensor] = None"}, {"name": "labels", "val": ": typing.Optional[torch.LongTensor] = None"}, {"name": "use_cache", "val": ": bool | None = None"}, {"name": "logits_to_keep", "val": ": typing.Union[int, torch.Tensor] = 0"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **cross_attention_states** (`torch.FloatTensor`, *optional*) --
  Output of the vision model, used for cross-attention. This tensor contains the processed image features that
  the language model will attend to.
- **cross_attention_mask** (`torch.Tensor` of shape `(batch_size, seq_length, max_num_images, max_num_tiles)`, *optional*) --
  Cross-attention mask to control the interaction between text tokens and image tiles.
  This 4D tensor defines which image tiles each text token should attend to.

  For each text token (in seq_length):
  - 1 indicates the token **should attend** to the corresponding image tile
  - 0 indicates the token **should not attend** to the corresponding image tile
- **full_text_row_masked_out_mask** (`tuple[torch.Tensor, torch.Tensor]`, *optional*) --
  A tuple containing two tensors that mask out rows in the cross-attention mechanism:
  - The first tensor has shape `(batch_size, 1, seq_length, 1)` and contains values of 0 or 1.
    A value of 0 indicates that the corresponding text token's entire row in the cross-attention
    matrix should be masked out (all image tokens ignored).
  - The second tensor has the same shape and is used internally to apply the masking during
    the forward pass of cross-attention layers.
  This mask is derived from the cross_attention_mask and is used to handle cases where a text token
  should not attend to any image token.
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
  This is useful when using packed tensor format (single dimension for batch and sequence length).[CausalLMOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or `tuple(torch.FloatTensor)`A [CausalLMOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MllamaConfig](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaConfig)) and inputs.
The [MllamaForCausalLM](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaForCausalLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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
>>> from transformers import AutoTokenizer, MllamaForCausalLM

>>> model = MllamaForCausalLM.from_pretrained("Llama-3.2-11B-Vision")
>>> tokenizer = AutoTokenizer.from_pretrained("Llama-3.2-11B-Vision")

>>> prompt = "If I had to write a haiku, it would be:"
>>> inputs = tokenizer(prompt, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(inputs.input_ids, max_length=40, do_sample=True, temperature=0.6)
>>> result = tokenizer.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
>>> print(result)
If I had to write a haiku, it would be: "Snowflakes gently fall" - simple, yet peaceful.
I love the idea of snowflakes gently falling, each one
```

## MllamaTextModel[[transformers.MllamaTextModel]]

- **config** ([MllamaTextConfig](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaTextConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Mllama Text Model which consists of transformer with self and cross attention layers.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>, )>] | None = None"}, {"name": "past_key_values", "val": ": transformers.cache_utils.Cache | None = None"}, {"name": "inputs_embeds", "val": ": typing.Optional[torch.FloatTensor] = None"}, {"name": "use_cache", "val": ": bool | None = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **cross_attention_states** (`torch.FloatTensor`, *optional*) --
  Output of the vision model, used for cross-attention. This tensor contains the processed image features that
  the language model will attend to.
- **cross_attention_mask** (`torch.Tensor` of shape `(batch_size, seq_length, max_num_images, max_num_tiles)`, *optional*) --
  Cross-attention mask to control the interaction between text tokens and image tiles.
  This 4D tensor defines which image tiles each text token should attend to.

  For each text token (in seq_length):
  - 1 indicates the token **should attend** to the corresponding image tile
  - 0 indicates the token **should not attend** to the corresponding image tile
- **full_text_row_masked_out_mask** (`tuple[torch.Tensor, torch.Tensor]`, *optional*) --
  A tuple containing two tensors that mask out rows in the cross-attention mechanism:
  - The first tensor has shape `(batch_size, 1, seq_length, 1)` and contains values of 0 or 1.
    A value of 0 indicates that the corresponding text token's entire row in the cross-attention
    matrix should be masked out (all image tokens ignored).
  - The second tensor has the same shape and is used internally to apply the masking during
    the forward pass of cross-attention layers.
  This mask is derived from the cross_attention_mask and is used to handle cases where a text token
  should not attend to any image token.
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
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).[BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MllamaConfig](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaConfig)) and inputs.
The [MllamaTextModel](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaTextModel) forward method, overrides the `__call__` special method.

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

Example:

```python
>>> from transformers import AutoProcessor, MllamaTextModel

>>> checkpoint = "meta-llama/Llama-3.2-11B-Vision"
>>> model = MllamaTextModel.from_pretrained(checkpoint)
>>> processor = AutoProcessor.from_pretrained(checkpoint)

>>> text = "<|image|>If I had to write a haiku for this one"
>>> inputs = processor(text=text, return_tensors="pt")

>>> output = model(**inputs)

>>> print(output.last_hidden_state.shape)
torch.Size([1, 13, 4096])
```

## MllamaModel[[transformers.MllamaModel]]

- **config** ([MllamaConfig](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Mllama model which consists of a vision encoder and a language model without language modeling head.

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
  [MllamaImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaImageProcessor). See `MllamaImageProcessor.__call__()` for details ([MllamaProcessor](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaProcessor) uses
  [MllamaImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaImageProcessor) for processing images).
- **aspect_ratio_mask** (`torch.Tensor` of shape `(batch_size, max_num_images, max_num_tiles)`, *optional*) --
  Mask to avoid performing attention on padding tiles. Mask values selected in `[0, 1]`:

  - 1 for tiles that are **not masked**,
  - 0 for tiles that are **masked**.
- **aspect_ratio_ids** (`torch.Tensor` of shape `(batch_size, max_num_images)`, *optional*) --
  Aspect ratio ids used to select the appropriate precomputed tile embeddings based on the aspect ratio of each input image.
  These ids correspond to indices in the model's list of supported aspect ratios, offset by 1.

  For example, if the model supports aspect ratios [[1, 1], [1, 2], [2, 1]]:
  - An image with aspect ratio [1, 1] would have ID 1
  - An image with aspect ratio [1, 2] would have ID 2
  - An image with aspect ratio [2, 1] would have ID 3

  The id 0 is reserved for padding (i.e., no image).

  If an image has aspect ratio [1, 2], that means it was split into 2 tiles horizontally, and its `aspect_ratio_id` would be 2.
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **cross_attention_mask** (`torch.Tensor` of shape `(batch_size, seq_length, max_num_images, max_num_tiles)`, *optional*) --
  Cross-attention mask to control the interaction between text tokens and image tiles.
  This 4D tensor defines which image tiles each text token should attend to.

  For each text token (in seq_length):
  - 1 indicates the token **should attend** to the corresponding image tile
  - 0 indicates the token **should not attend** to the corresponding image tile
- **cross_attention_states** (`torch.FloatTensor`, *optional*) --
  Output of the vision model, used for cross-attention. This tensor contains the processed image features that
  the language model will attend to.
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
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).[BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MllamaConfig](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaConfig)) and inputs.
The [MllamaModel](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaModel) forward method, overrides the `__call__` special method.

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

## MllamaVisionModel[[transformers.MllamaVisionModel]]

- **config** ([MllamaVisionConfig](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaVisionConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Mllama Vision Model which consists of two vision encoders.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "aspect_ratio_ids", "val": ": )>"}, {"name": "aspect_ratio_mask", "val": ": )>"}, {"name": "**kwargs", "val": ""}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [MllamaImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaImageProcessor). See `MllamaImageProcessor.__call__()` for details ([MllamaProcessor](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaProcessor) uses
  [MllamaImageProcessor](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaImageProcessor) for processing images).
- **aspect_ratio_ids** (`torch.Tensor` of shape `(batch_size, max_num_images)`, *optional*) --
  Aspect ratio ids used to select the appropriate precomputed tile embeddings based on the aspect ratio of each input image.
  These ids correspond to indices in the model's list of supported aspect ratios, offset by 1.

  For example, if the model supports aspect ratios [[1, 1], [1, 2], [2, 1]]:
  - An image with aspect ratio [1, 1] would have ID 1
  - An image with aspect ratio [1, 2] would have ID 2
  - An image with aspect ratio [2, 1] would have ID 3

  The id 0 is reserved for padding (i.e., no image).

  If an image has aspect ratio [1, 2], that means it was split into 2 tiles horizontally, and its `aspect_ratio_id` would be 2.
- **aspect_ratio_mask** (`torch.Tensor` of shape `(batch_size, max_num_images, max_num_tiles)`, *optional*) --
  Mask to avoid performing attention on padding tiles. Mask values selected in `[0, 1]`:

  - 1 for tiles that are **not masked**,
  - 0 for tiles that are **masked**.[BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or `tuple(torch.FloatTensor)`A [BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MllamaConfig](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaConfig)) and inputs.
The [MllamaVisionModel](/docs/transformers/v5.14.0/en/model_doc/mllama#transformers.MllamaVisionModel) forward method, overrides the `__call__` special method.

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

Example:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoProcessor, MllamaVisionModel

>>> checkpoint = "meta-llama/Llama-3.2-11B-Vision"
>>> model = MllamaVisionModel.from_pretrained(checkpoint)
>>> processor = AutoProcessor.from_pretrained(checkpoint)

>>> url = "https://www.ilankelman.org/stopsigns/australia.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))
>>> inputs = processor(images=image, return_tensors="pt")

>>> output = model(**inputs)

>>> print(output.last_hidden_state.shape)
torch.Size([1, 1, 4, 1025, 7680])
```
