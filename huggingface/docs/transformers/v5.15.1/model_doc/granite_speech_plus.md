# Granite Speech Plus

## Overview

Granite Speech Plus is a variant of [Granite Speech](./granite_speech) whose projector consumes the concatenation of
the encoder's final hidden states with an arbitrary subset of its intermediate hidden states (along the feature
dimension). The selected intermediate layers are controlled by the `cat_hidden_layers` config field on
[GraniteSpeechPlusEncoderConfig](/docs/transformers/v5.15.1/en/model_doc/granite_speech_plus#transformers.GraniteSpeechPlusEncoderConfig); when it is `None`, the model behaves identically to Granite Speech. When it is set, the
projector's `encoder_hidden_size` must equal `encoder_config.hidden_dim * (len(cat_hidden_layers) + 1)`.

The rest of the architecture — speech encoder, query transformer projector, language model, and optional LoRA adapter
— is inherited unchanged from Granite Speech. See the [Granite Speech documentation](./granite_speech) for usage
examples; the same [GraniteSpeechProcessor](/docs/transformers/v5.15.1/en/model_doc/granite_speech#transformers.GraniteSpeechProcessor) and [GraniteSpeechFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/granite_speech#transformers.GraniteSpeechFeatureExtractor) are used here.

## Usage

Granite Speech Plus is a multimodal speech-to-text model that can transcribe audio, provide speaker annotation and word level timestamps by responding to text prompts. Here's how to use the different functions:

**Setup** — load the model and a test audio clip:

```python
import re
import torch
from datasets import Audio, load_dataset
from transformers import AutoModelForSpeechSeq2Seq, AutoProcessor

SAMPLE_RATE = 16000
MODEL_NAME = "ibm-granite/granite-speech-4.1-2b-plus"
```

Define the prompts used for the different tasks:

```python
SYSTEM_PROMPT = "Knowledge Cutoff Date: April 2024.\nToday's Date: December 19, 2024.\nYou are Granite, developed by IBM. You are a helpful AI assistant"
ASR_PROMPT = "<|audio|> can you transcribe the speech into a written format?"
SAA_PROMPT = "<|audio|> Speaker attribution: Transcribe and denote who is speaking by adding [Speaker 1]: and [Speaker 2]: tags before speaker turns."
TS_PROMPT = "<|audio|> Timestamps: Transcribe the speech. After each word, add a timestamp tag showing the end time in centiseconds, e.g. hello [T:45] world [T:82]"
```

Load the model and define a general function for decoding the audio:

```python
processor = AutoProcessor.from_pretrained(MODEL_NAME)
model = AutoModelForSpeechSeq2Seq.from_pretrained(MODEL_NAME, device_map="auto")

@torch.inference_mode()
def transcribe(audio, prompt, max_new_tokens=2000, prefix_text=None):
    chat = [{"role": "system", "content": SYSTEM_PROMPT}, {"role": "user", "content": prompt}]
    extra = {"prefix_text": prefix_text} if prefix_text is not None else {}
    prompt_text = processor.apply_chat_template(chat, tokenize=False, add_generation_prompt=True, **extra)
    inputs = processor(prompt_text, audio, device=device, return_tensors="pt").to(device)
    outputs = model.generate(**inputs, max_new_tokens=max_new_tokens, do_sample=False, num_beams=1)
    new_tokens = outputs[0, inputs["input_ids"].shape[-1]:]
    output_text = processor.decode(new_tokens, add_special_tokens=False, skip_special_tokens=True)
    return output_text
```

Load some example audio data from the AMI dataset

```python
ds = load_dataset("diarizers-community/ami", "ihm", split="test")
ds = ds.cast_column("audio", Audio(sampling_rate=SAMPLE_RATE, num_channels=1))

TEST_SAMPLE = 0
START_TIME, END_TIME = 5 * 60, 6 * 60
audio = ds["audio"][TEST_SAMPLE].get_samples_played_in_range(START_TIME, END_TIME)
```

**Task 1: ASR** — plain speech-to-text transcription:

```python
asr_text = transcribe(audio.data, ASR_PROMPT)
print(asr_text)
```

**Task 2: Speaker Attributed ASR** — transcription with speaker labels:

```python
saa_text = transcribe(audio.data, SAA_PROMPT)
for segment in re.split(r"(\[Speaker \d+\]:)", saa_text):
    print(segment.strip())
```

**Task 3: Word-level timestamps** — transcription with per-word timing:

The timestamps are given in centiseconds and are modulo 1000 (=10 seconds)
so we need to unwrap them by adding multiples of 10 seconds.

```python
ts_text = transcribe(audio.data, TS_PROMPT, max_new_tokens=10000)
ts_words = re.split(r"\[T:(\d+)\]", ts_text)
last_word_end_time = 0
offset_time = 0
for word, ts in zip(ts_words[::2], ts_words[1::2]):
    word_end_time = float(ts) / 100
    while word_end_time + offset_time < last_word_end_time:
        offset_time += 10
    last_word_end_time = word_end_time + offset_time
    print(f"{word}\t{last_word_end_time:.2f}s")
```

**Task 4: Incremental decoding** — transcribe segments while accumulating audio context:

```python
NUM_SEGMENTS = 3
previous_transcript = ""
all_audio = None

for k in range(NUM_SEGMENTS):
    t1 = START_TIME + (END_TIME - START_TIME) * k / NUM_SEGMENTS
    t2 = START_TIME + (END_TIME - START_TIME) * (k + 1) / NUM_SEGMENTS
    new_audio = ds["audio"][TEST_SAMPLE].get_samples_played_in_range(t1, t2)
    all_audio = new_audio.data if all_audio is None else torch.cat([all_audio, new_audio.data], dim=-1)
    saa_text = transcribe(all_audio, SAA_PROMPT, prefix_text=previous_transcript)
    print(f"{t1:06.2f}-{t2:06.2f}:\t{saa_text}")
    previous_transcript = (previous_transcript + " " + saa_text).strip()
```

## GraniteSpeechPlusConfig[[transformers.GraniteSpeechPlusConfig]]

#### transformers.GraniteSpeechPlusConfig[[transformers.GraniteSpeechPlusConfig]]

```python
transformers.GraniteSpeechPlusConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, encoder_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, projector_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, audio_token_index: int = 49155, initializer_range: float = 0.02, has_lora_adapter: bool = True, downsample_rate: int = 5, window_size: int = 15, tie_word_embeddings: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/granite_speech_plus/configuration_granite_speech_plus.py#L95)

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

This is the configuration class to store the configuration of a GraniteSpeechPlusModel. It is used to instantiate a Granite Speech Plus
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [ibm-granite/granite-speech-4.1-2b-plus](https://huggingface.co/ibm-granite/granite-speech-4.1-2b-plus)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import GraniteSpeechPlusConfig, GraniteSpeechPlusForConditionalGeneration

>>> # Initializing a GraniteSpeechPlusConfig
>>> configuration = GraniteSpeechPlusConfig()

>>> # Initializing a GraniteSpeechPlusForConditionalGeneration (with random weights)
>>> model = GraniteSpeechPlusForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## GraniteSpeechPlusEncoderConfig[[transformers.GraniteSpeechPlusEncoderConfig]]

#### transformers.GraniteSpeechPlusEncoderConfig[[transformers.GraniteSpeechPlusEncoderConfig]]

```python
transformers.GraniteSpeechPlusEncoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, input_dim: int = 160, num_layers: int = 10, hidden_dim: int = 1024, feedforward_mult: int = 4, num_heads: int = 8, dim_head: int | None = None, output_dim: int = 42, context_size: int = 200, max_pos_emb: int = 512, dropout: float | int = 0.1, conv_kernel_size: int = 15, conv_expansion_factor: int = 2, cat_hidden_layers: list[int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/granite_speech_plus/configuration_granite_speech_plus.py#L29)

**Parameters:**

input_dim (*int*, *optional*, defaults to *160*) : Dimensionality of the input acoustic features (e.g., number of mel-filterbank channels).

num_layers (*int*, *optional*, defaults to *10*) : Number of hidden layers in the Transformer decoder.

hidden_dim (*int*, *optional*, defaults to *1024*) : Dimension of the hidden representations.

feedforward_mult (*int*, *optional*, defaults to 4) : Multiplier for the up/down projections in the encoder's feedforward layers; The projections will have intermediate dim of size *hidden_dim * feedforward_mult*.

num_heads (*int*, *optional*, defaults to *8*) : Number of attention heads for each attention layer in the Transformer decoder.

dim_head (*int*, *optional*) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

output_dim (*int*, *optional*, defaults to 42) : Intermediate dimension of the feedforward projections in the conformer to be added to every other encoder block's output.

context_size (*int*, *optional*, defaults to 200) : Context size to be used in conformer attention.

max_pos_emb (*int*, *optional*, defaults to 512) : Max pos embeds to be used in attention (shaw's relative positional encoding).

dropout (*Union[float, int]*, *optional*, defaults to *0.1*) : The ratio for all dropout layers.

conv_kernel_size (*int*, *optional*, defaults to *15*) : The size of the convolutional kernel.

conv_expansion_factor (*int*, *optional*, defaults to 2) : Intermediate dimension to be used in conformer convolutions.

cat_hidden_layers (*list[int]*, *optional*) : Indices of encoder conformer layers whose outputs are concatenated with the final encoder output (along the feature dimension) before being passed to the projector. When set, the projector's `encoder_hidden_size` must equal `encoder_config.hidden_dim * (len(cat_hidden_layers) + 1)`.

This is the configuration class to store the configuration of a GraniteSpeechPlusModel. It is used to instantiate a Granite Speech Plus
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [ibm-granite/granite-speech-4.1-2b-plus](https://huggingface.co/ibm-granite/granite-speech-4.1-2b-plus)

Configuration objects inherit from [*PreTrainedConfig*] and can be used to control the model outputs. Read the
documentation from [*PreTrainedConfig*] for more information.

Example:

```python
>>> from transformers import GraniteSpeechPlusEncoderConfig, GraniteSpeechPlusCTCEncoder

>>> # Initializing a GraniteSpeechPlusEncoderConfig
>>> configuration = GraniteSpeechPlusEncoderConfig()

>>> # Initializing a GraniteSpeechPlusCTCEncoder (with random weights)
>>> model = GraniteSpeechPlusCTCEncoder(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## GraniteSpeechPlusModel[[transformers.GraniteSpeechPlusModel]]

#### transformers.GraniteSpeechPlusModel[[transformers.GraniteSpeechPlusModel]]

```python
transformers.GraniteSpeechPlusModel(config: GraniteSpeechPlusConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/granite_speech_plus/modeling_granite_speech_plus.py#L131)

**Parameters:**

config ([GraniteSpeechPlusConfig](/docs/transformers/v5.15.1/en/model_doc/granite_speech_plus#transformers.GraniteSpeechPlusConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Granite Speech model, which consists of an audio encoder, projector, and language model,
without a language modeling head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.GraniteSpeechPlusModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, input_features: typing.Optional[torch.FloatTensor] = None, input_features_mask: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/granite_speech_plus/modeling_granite_speech_plus.py#L204)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) : The tensors corresponding to the input audio features. Audio features can be obtained using [GraniteSpeechFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/granite_speech#transformers.GraniteSpeechFeatureExtractor). See `GraniteSpeechFeatureExtractor.__call__()` for details ([GraniteSpeechProcessor](/docs/transformers/v5.15.1/en/model_doc/granite_speech#transformers.GraniteSpeechProcessor) uses [GraniteSpeechFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/granite_speech#transformers.GraniteSpeechFeatureExtractor) for processing audios).

input_features_mask (`torch.Tensor`, *optional*) : Mask to be applied to audio features prior to scattering into the language embeddings.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** `GraniteSpeechPlusModelOutputWithPast` or `tuple(torch.FloatTensor)`

A `GraniteSpeechPlusModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([GraniteSpeechPlusConfig](/docs/transformers/v5.15.1/en/model_doc/granite_speech_plus#transformers.GraniteSpeechPlusConfig)) and inputs.

The [GraniteSpeechPlusModel](/docs/transformers/v5.15.1/en/model_doc/granite_speech_plus#transformers.GraniteSpeechPlusModel) forward method, overrides the `__call__` special method.

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

## GraniteSpeechPlusForConditionalGeneration[[transformers.GraniteSpeechPlusForConditionalGeneration]]

#### transformers.GraniteSpeechPlusForConditionalGeneration[[transformers.GraniteSpeechPlusForConditionalGeneration]]

```python
transformers.GraniteSpeechPlusForConditionalGeneration(config: GraniteSpeechPlusConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/granite_speech_plus/modeling_granite_speech_plus.py#L517)

**Parameters:**

config ([GraniteSpeechPlusConfig](/docs/transformers/v5.15.1/en/model_doc/granite_speech_plus#transformers.GraniteSpeechPlusConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Granite Speech Plus model, a Granite Speech variant whose projector consumes the concatenation of the
encoder's final hidden states with an arbitrary subset of its intermediate hidden states.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.GraniteSpeechPlusForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, input_features: typing.Optional[torch.FloatTensor] = None, input_features_mask: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/granite_speech_plus/modeling_granite_speech_plus.py#L538)

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

**Returns:** `GraniteSpeechPlusCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`

A `GraniteSpeechPlusCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([GraniteSpeechPlusConfig](/docs/transformers/v5.15.1/en/model_doc/granite_speech_plus#transformers.GraniteSpeechPlusConfig)) and inputs.

The [GraniteSpeechPlusForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/granite_speech_plus#transformers.GraniteSpeechPlusForConditionalGeneration) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoProcessor, GraniteSpeechPlusForConditionalGeneration
>>> from datasets import load_dataset
>>> import torch

>>> dataset = load_dataset("hf-internal-testing/librispeech_asr_demo", "clean", split="validation")
>>> dataset = dataset.sort("id")
>>> sampling_rate = dataset.features["audio"].sampling_rate

>>> processor = AutoProcessor.from_pretrained("ibm-granite/granite-speech-4.1-2b-plus")
>>> model = GraniteSpeechPlusForConditionalGeneration.from_pretrained("ibm-granite/granite-speech-4.1-2b-plus")

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

#### get_audio_features[[transformers.GraniteSpeechPlusForConditionalGeneration.get_audio_features]]

```python
get_audio_features(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/granite_speech_plus/modeling_granite_speech_plus.py#L535)
