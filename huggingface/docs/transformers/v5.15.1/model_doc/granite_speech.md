# Granite Speech

## Overview

The [Granite Speech](https://huggingface.co/papers/2505.08699) model ([blog post](https://www.ibm.com/new/announcements/ibm-granite-3-3-speech-recognition-refined-reasoning-rag-loras)) is a multimodal language model, consisting of a speech encoder, speech projector, large language model, and LoRA adapter(s). More details regarding each component for the current (Granite 3.2 Speech) model architecture may be found below.

1. Speech Encoder: A [Conformer](https://huggingface.co/papers/2005.08100) encoder trained with Connectionist Temporal Classification (CTC) on character-level targets on ASR corpora. The encoder uses block-attention and self-conditioned CTC from the middle layer.

2. Speech Projector: A query transformer (q-former) operating on the outputs of the last encoder block. The encoder and projector temporally downsample the audio features to be merged into the multimodal embeddings to be processed by the llm.

3. Large Language Model: The Granite Speech model leverages Granite LLMs, which were originally proposed in [this paper](https://huggingface.co/papers/2408.13359).

4. LoRA adapter(s): The Granite Speech model contains a modality specific LoRA, which will be enabled when audio features are provided, and disabled otherwise.

Note that most of the aforementioned components are implemented generically to enable compatibility and potential integration with other model architectures in transformers.

This model was contributed by [Alexander Brooks](https://huggingface.co/abrooks9944), [Avihu Dekel](https://huggingface.co/Avihu), and [George Saon](https://huggingface.co/gsaon).

## Usage tips

- This model bundles its own LoRA adapter, which will be automatically loaded and enabled/disabled as needed during inference calls. Be sure to install [PEFT](https://github.com/huggingface/peft) to ensure the LoRA is correctly applied!
- The model expects 16kHz sampling rate audio. The processor will automatically resample if needed.
- The LoRA adapter is automatically enabled when audio features are present and disabled for text-only inputs, so you don't need to manage it manually.

## Usage example

Granite Speech is a multimodal speech-to-text model that can transcribe audio and respond to text prompts. Here's how to use it:

### Basic Speech Transcription

```python
from datasets import Audio, load_dataset

from transformers import GraniteSpeechForConditionalGeneration, GraniteSpeechProcessor

# Load model and processor
model = GraniteSpeechForConditionalGeneration.from_pretrained(
    "ibm-granite/granite-3.2-8b-speech",
    device_map="auto"
)
processor = GraniteSpeechProcessor.from_pretrained("ibm-granite/granite-3.2-8b-speech")

# Load audio from dataset (16kHz sampling rate required)
ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))
audio = ds['audio'][0]['array']

# Process audio
inputs = processor(audio=audio, return_tensors="pt").to(model.device)

# Generate transcription
generated_ids = model.generate(**inputs, max_new_tokens=256)
transcription = processor.batch_decode(generated_ids, skip_special_tokens=True)[0]
print(transcription)
```

### Speech-to-Text with Chat Template

For instruction-following with audio, use the chat template with audio directly in the conversation format:

```python
from datasets import Audio, load_dataset

from transformers import GraniteSpeechForConditionalGeneration, GraniteSpeechProcessor

model = GraniteSpeechForConditionalGeneration.from_pretrained(
    "ibm-granite/granite-3.2-8b-speech",
    device_map="auto"
)
processor = GraniteSpeechProcessor.from_pretrained("ibm-granite/granite-3.2-8b-speech")

# Load audio from dataset
ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))
audio = ds['audio'][0]

# Prepare conversation with audio and text
conversation = [
    {
        "role": "user",
        "content": [
            {"type": "audio", "audio": audio},
            {"type": "text", "text": "Transcribe the following audio:"},
        ],
    }
]

# Apply chat template with audio - processor handles both tokenization and audio processing
inputs = processor.apply_chat_template(conversation, tokenize=True, return_tensors="pt").to(model.device)

# Generate transcription
generated_ids = model.generate(**inputs, max_new_tokens=512)
output_text = processor.batch_decode(generated_ids, skip_special_tokens=True)[0]
print(output_text)
```

### Batch Processing

Process multiple audio files efficiently:

```python
from datasets import Audio, load_dataset

from transformers import GraniteSpeechForConditionalGeneration, GraniteSpeechProcessor

model = GraniteSpeechForConditionalGeneration.from_pretrained(
    "ibm-granite/granite-3.2-8b-speech",
    device_map="auto"
)
processor = GraniteSpeechProcessor.from_pretrained("ibm-granite/granite-3.2-8b-speech")

# Load multiple audio samples from dataset
ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))
audio_samples = [ds['audio'][i]['array'] for i in range(3)]

# Process batch
inputs = processor(audio=audio_samples, return_tensors="pt", padding=True).to(model.device)

# Generate for all inputs
generated_ids = model.generate(**inputs, max_new_tokens=256)
transcriptions = processor.batch_decode(generated_ids, skip_special_tokens=True)

for i, transcription in enumerate(transcriptions):
    print(f"Audio {i+1}: {transcription}")
```

## GraniteSpeechConfig[[transformers.GraniteSpeechConfig]]

#### transformers.GraniteSpeechConfig[[transformers.GraniteSpeechConfig]]

```python
transformers.GraniteSpeechConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, encoder_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, projector_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, audio_token_index: int = 49155, initializer_range: float = 0.02, has_lora_adapter: bool = True, downsample_rate: int = 5, window_size: int = 15, tie_word_embeddings: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/granite_speech/configuration_granite_speech.py#L84)

**Parameters:**

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

encoder_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the encoder backbone.

projector_config (`Union[AutoConfig, dict]`, *optional*, defaults to `Blip2QFormerConfig`) : The config object or dictionary of the audio projector.

audio_token_index (`int`, *optional*, defaults to `49155`) : The audio token index used as a placeholder for input audio.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

has_lora_adapter (`bool`, *optional*, defaults to `True`) : Indicates whether or not the model has a lora adapter that should only be activate when processing audio inputs.

downsample_rate (`int`, *optional*, defaults to 5) : Downsample rate for the audio feature extractor.

window_size (`int`, *optional*, defaults to 15) : Window size for the audio feature projector.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a GraniteSpeechModel. It is used to instantiate a Granite Speech
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [ibm-granite/granite-speech-3.3-2b](https://huggingface.co/ibm-granite/granite-speech-3.3-2b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import GraniteSpeechConfig, GraniteSpeechForConditionalGeneration

>>> # Initializing a GraniteSpeechConfig
>>> configuration = GraniteSpeechConfig()

>>> # Initializing a GraniteSpeechForConditionalGeneration (with random weights)
>>> model = GraniteSpeechForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## GraniteSpeechEncoderConfig[[transformers.GraniteSpeechEncoderConfig]]

#### transformers.GraniteSpeechEncoderConfig[[transformers.GraniteSpeechEncoderConfig]]

```python
transformers.GraniteSpeechEncoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, input_dim: int = 160, num_layers: int = 10, hidden_dim: int = 1024, feedforward_mult: int = 4, num_heads: int = 8, dim_head: int | None = None, output_dim: int = 42, context_size: int = 200, max_pos_emb: int = 512, dropout: float | int = 0.1, conv_kernel_size: int = 15, conv_expansion_factor: int = 2)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/granite_speech/configuration_granite_speech.py#L25)

**Parameters:**

input_dim (`int`, *optional*, defaults to `160`) : Dimensionality of the input acoustic features (e.g., number of mel-filterbank channels).

num_layers (`int`, *optional*, defaults to `10`) : Number of hidden layers in the Transformer decoder.

hidden_dim (`int`, *optional*, defaults to `1024`) : Dimension of the hidden representations.

feedforward_mult (`int`, *optional*, defaults to 4) : Multiplier for the up/down projections in the encoder's feedforward layers; The projections will have intermediate dim of size `hidden_dim * feedforward_mult`.

num_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer decoder.

dim_head (`int`, *optional*) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

output_dim (`int`, *optional*, defaults to 42) : Intermediate dimension of the feedforward projections in the conformer to be added to every other encoder block's output.

context_size (`int`, *optional*, defaults to 200) : Context size to be used in conformer attention.

max_pos_emb (`int`, *optional*, defaults to 512) : Max pos embeds to be used in attention (shaw's relative positional encoding).

dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The ratio for all dropout layers.

conv_kernel_size (`int`, *optional*, defaults to `15`) : The size of the convolutional kernel.

conv_expansion_factor (`int`, *optional*, defaults to 2) : Intermediate dimension to be used in conformer convolutions.

This is the configuration class to store the configuration of a GraniteSpeechModel. It is used to instantiate a Granite Speech
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [ibm-granite/granite-speech-3.3-2b](https://huggingface.co/ibm-granite/granite-speech-3.3-2b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import GraniteSpeechEncoderConfig, GraniteSpeechCTCEncoder

>>> # Initializing a GraniteSpeechEncoderConfig
>>> configuration = GraniteSpeechEncoderConfig()

>>> # Initializing a GraniteSpeechCTCEncoder (with random weights)
>>> model = GraniteSpeechCTCEncoder(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## GraniteSpeechProcessor[[transformers.GraniteSpeechProcessor]]

#### transformers.GraniteSpeechProcessor[[transformers.GraniteSpeechProcessor]]

```python
transformers.GraniteSpeechProcessor(audio_processor, tokenizer, audio_token = '<|audio|>', chat_template = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/granite_speech/processing_granite_speech.py#L35)

**Parameters:**

audio_processor (`{audio_processor_class}`) : The audio processor is a required input.

tokenizer (`tokenizer_class`) : The tokenizer is a required input.

audio_token (`str`, *optional*, defaults to `"<|audio|>"`) : The special token used to represent audio in the text sequence. This token serves as a placeholder that will be replaced with multiple audio tokens based on the actual audio length. The number of audio tokens inserted depends on the audio feature dimensions extracted by the audio processor.

chat_template (`str`) : A Jinja template to convert lists of messages in a chat into a tokenizable string.

Constructs a GraniteSpeechProcessor which wraps a audio processor and a tokenizer into a single processor.

[GraniteSpeechProcessor](/docs/transformers/v5.15.1/en/model_doc/granite_speech#transformers.GraniteSpeechProcessor) offers all the functionalities of `{audio_processor_class}` and `tokenizer_class`. See the
`~{audio_processor_class}` and `~tokenizer_class` for more information.

#### __call__[[transformers.GraniteSpeechProcessor.__call__]]

```python
__call__(text: str | list[str] | list[list[str]], audio: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor'], NoneType] = None, device: str = 'cpu', **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/granite_speech/processing_granite_speech.py#L54)

**Parameters:**

text (`Union[str, list[str], list[list[str]]]`) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

audio (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`, *optional*) : The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor. In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels, and T is the sample length of the audio.

device (`str`, *optional*, defaults to `cpu`) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) : Additional processing options for each modality (text, images, videos, audio). Model-specific parameters are listed above; see the TypedDict class for the complete list of supported arguments.

**Returns:** `~feature_extraction_utils.BatchFeature`

- **data** (`dict`, *optional*) -- Dictionary of lists/arrays/tensors returned by the __call__/pad methods ('input_values', 'attention_mask',
  etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.
- **skip_tensor_conversion** (`list[str]` or `set[str]`, *optional*) -- List or set of keys that should NOT be converted to tensors, even when `tensor_type` is specified.

## GraniteSpeechFeatureExtractor[[transformers.GraniteSpeechFeatureExtractor]]

#### transformers.GraniteSpeechFeatureExtractor[[transformers.GraniteSpeechFeatureExtractor]]

```python
transformers.GraniteSpeechFeatureExtractor(sampling_rate: int = 16000, n_fft: int = 512, win_length: int = 400, hop_length: int = 160, n_mels: int = 80, projector_window_size: int = 15, projector_downsample_rate: int = 5, padding_value: float = 0.0, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/granite_speech/feature_extraction_granite_speech.py#L37)

## GraniteSpeechModel[[transformers.GraniteSpeechModel]]

#### transformers.GraniteSpeechModel[[transformers.GraniteSpeechModel]]

```python
transformers.GraniteSpeechModel(config: GraniteSpeechConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/granite_speech/modeling_granite_speech.py#L351)

**Parameters:**

config ([GraniteSpeechConfig](/docs/transformers/v5.15.1/en/model_doc/granite_speech#transformers.GraniteSpeechConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Granite Speech model, which consists of an audio encoder, projector, and language model,
without a language modeling head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.GraniteSpeechModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, input_features: typing.Optional[torch.FloatTensor] = None, input_features_mask: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/granite_speech/modeling_granite_speech.py#L424)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) : The tensors corresponding to the input audio features. Audio features can be obtained using [GraniteSpeechFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/granite_speech#transformers.GraniteSpeechFeatureExtractor). See `GraniteSpeechFeatureExtractor.__call__()` for details ([GraniteSpeechProcessor](/docs/transformers/v5.15.1/en/model_doc/granite_speech#transformers.GraniteSpeechProcessor) uses [GraniteSpeechFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/granite_speech#transformers.GraniteSpeechFeatureExtractor) for processing audios).

input_features_mask (`torch.Tensor`, *optional*) : Mask to be applied to audio features prior to scattering into the language embeddings.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** `GraniteSpeechModelOutputWithPast` or `tuple(torch.FloatTensor)`

A `GraniteSpeechModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([GraniteSpeechConfig](/docs/transformers/v5.15.1/en/model_doc/granite_speech#transformers.GraniteSpeechConfig)) and inputs.

The [GraniteSpeechModel](/docs/transformers/v5.15.1/en/model_doc/granite_speech#transformers.GraniteSpeechModel) forward method, overrides the `__call__` special method.

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
- **audio_hidden_states** (`torch.FloatTensor`, *optional*) -- Projected audio hidden states.

## GraniteSpeechForConditionalGeneration[[transformers.GraniteSpeechForConditionalGeneration]]

#### transformers.GraniteSpeechForConditionalGeneration[[transformers.GraniteSpeechForConditionalGeneration]]

```python
transformers.GraniteSpeechForConditionalGeneration(config: GraniteSpeechConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/granite_speech/modeling_granite_speech.py#L496)

**Parameters:**

config ([GraniteSpeechConfig](/docs/transformers/v5.15.1/en/model_doc/granite_speech#transformers.GraniteSpeechConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Granite Speech model, which consists of an audio encoder, projector, and language model.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.GraniteSpeechForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, input_features: typing.Optional[torch.FloatTensor] = None, input_features_mask: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/granite_speech/modeling_granite_speech.py#L517)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) : The tensors corresponding to the input audio features. Audio features can be obtained using [GraniteSpeechFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/granite_speech#transformers.GraniteSpeechFeatureExtractor). See `GraniteSpeechFeatureExtractor.__call__()` for details ([GraniteSpeechProcessor](/docs/transformers/v5.15.1/en/model_doc/granite_speech#transformers.GraniteSpeechProcessor) uses [GraniteSpeechFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/granite_speech#transformers.GraniteSpeechFeatureExtractor) for processing audios).

input_features_mask (`torch.Tensor`, *optional*) : Mask to be applied to audio features prior to scattering into the language embeddings.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** `GraniteSpeechCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`

A `GraniteSpeechCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([GraniteSpeechConfig](/docs/transformers/v5.15.1/en/model_doc/granite_speech#transformers.GraniteSpeechConfig)) and inputs.

The [GraniteSpeechForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/granite_speech#transformers.GraniteSpeechForConditionalGeneration) forward method, overrides the `__call__` special method.

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
- **audio_hidden_states** (`torch.FloatTensor`, *optional*) -- Projected audio hidden states.

Example:

```python
>>> from transformers import AutoProcessor, GraniteSpeechForConditionalGeneration
>>> from datasets import load_dataset
>>> import torch

>>> dataset = load_dataset("hf-internal-testing/librispeech_asr_demo", "clean", split="validation")
>>> dataset = dataset.sort("id")
>>> sampling_rate = dataset.features["audio"].sampling_rate

>>> processor = AutoProcessor.from_pretrained("ibm-granite/granite-speech-3.3-2b")
>>> model = GraniteSpeechForConditionalGeneration.from_pretrained("ibm-granite/granite-speech-3.3-2b")

>>> # audio file is decoded on the fly
>>> inputs = processor(dataset[0]["audio"]["array"], sampling_rate=sampling_rate, return_tensors="pt")
>>> with torch.no_grad():
...     logits = model(**inputs).logits
>>> predicted_ids = torch.argmax(logits, dim=-1)

>>> # transcribe speech
>>> transcription = processor.batch_decode(predicted_ids)
>>> transcription[0]
...

>>> inputs["labels"] = processor(text=dataset[0]["text"], return_tensors="pt").input_ids

>>> # compute loss
>>> loss = model(**inputs).loss
>>> round(loss.item(), 2)
...
```

#### get_audio_features[[transformers.GraniteSpeechForConditionalGeneration.get_audio_features]]

```python
get_audio_features(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/granite_speech/modeling_granite_speech.py#L514)
