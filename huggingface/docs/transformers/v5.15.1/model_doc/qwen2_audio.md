# Qwen2Audio

## Overview

The Qwen2-Audio is the new model series of large audio-language models from the Qwen team. Qwen2-Audio is capable of accepting various audio signal inputs and performing audio analysis or direct textual responses with regard to speech instructions. We introduce two distinct audio interaction modes:

* voice chat: users can freely engage in voice interactions with Qwen2-Audio without text input
* audio analysis: users could provide audio and text instructions for analysis during the interaction

It was proposed in [Qwen2-Audio Technical Report](https://huggingface.co/papers/2407.10759) by Yunfei Chu, Jin Xu, Qian Yang, Haojie Wei, Xipin Wei, Zhifang Guo, Yichong Leng, Yuanjun Lv, Jinzheng He, Junyang Lin, Chang Zhou, Jingren Zhou.

The abstract from the paper is the following:

*We introduce the latest progress of Qwen-Audio, a large-scale audio-language model called Qwen2-Audio, which is capable of accepting various audio signal inputs and performing audio analysis or direct textual responses with regard to speech instructions. In contrast to complex hierarchical tags, we have simplified the pre-training process by utilizing natural language prompts for different data and tasks, and have further expanded the data volume. We have boosted the instruction-following capability of Qwen2-Audio and implemented two distinct audio interaction modes for voice chat and audio analysis. In the voice chat mode, users can freely engage in voice interactions with Qwen2-Audio without text input. In the audio analysis mode, users could provide audio and text instructions for analysis during the interaction. Note that we do not use any system prompts to switch between voice chat and audio analysis modes. Qwen2-Audio is capable of intelligently comprehending the content within audio and following voice commands to respond appropriately. For instance, in an audio segment that simultaneously contains sounds, multi-speaker conversations, and a voice command, Qwen2-Audio can directly understand the command and provide an interpretation and response to the audio. Additionally, DPO has optimized the model's performance in terms of factuality and adherence to desired behavior. According to the evaluation results from AIR-Bench, Qwen2-Audio outperformed previous SOTAs, such as Gemini-1.5-pro, in tests focused on audio-centric instruction-following capabilities. Qwen2-Audio is open-sourced with the aim of fostering the advancement of the multi-modal language community.*

## Usage tips

`Qwen2-Audio-7B` and `Qwen2-Audio-7B-Instruct` can be found on the [Huggingface Hub](https://huggingface.co/Qwen)

### Inference

```python
from io import BytesIO
from urllib.request import urlopen

import librosa

from transformers import AutoProcessor, Qwen2AudioForConditionalGeneration

model = Qwen2AudioForConditionalGeneration.from_pretrained("Qwen/Qwen2-Audio-7B", trust_remote_code=True, device_map="auto")
processor = AutoProcessor.from_pretrained("Qwen/Qwen2-Audio-7B", trust_remote_code=True)

prompt = "<|audio_bos|><|AUDIO|><|audio_eos|>Generate the caption in English:"
url = "https://qianwen-res.oss-cn-beijing.aliyuncs.com/Qwen-Audio/glass-breaking-151256.mp3"
audio, sr = librosa.load(BytesIO(urlopen(url).read()), sr=processor.feature_extractor.sampling_rate)
inputs = processor(text=prompt, audio=audio, return_tensors="pt").to(model.device)

generate_ids = model.generate(**inputs, max_length=256)
generate_ids = generate_ids[:, inputs.input_ids.size(1):]

response = processor.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]

# We can also omit the audio_bos and audio_eos tokens
prompt = "<|AUDIO|>Generate the caption in English:"
inputs = processor(text=prompt, audio=audio, return_tensors="pt").to(model.device)

generate_ids = model.generate(**inputs, max_length=256)
generate_ids = generate_ids[:, inputs.input_ids.size(1):]

response = processor.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
```

In the following, we demonstrate how to use `Qwen2-Audio-7B-Instruct` for the inference, supporting both voice chat and audio analysis modes. Note that we have used the ChatML format for dialog, in this demo we show how to leverage `apply_chat_template` for this purpose.

### Voice Chat Inference

In the voice chat mode, users can freely engage in voice interactions with Qwen2-Audio without text input:

```python
from io import BytesIO
from urllib.request import urlopen

import librosa

from transformers import AutoProcessor, Qwen2AudioForConditionalGeneration

processor = AutoProcessor.from_pretrained("Qwen/Qwen2-Audio-7B-Instruct")
model = Qwen2AudioForConditionalGeneration.from_pretrained("Qwen/Qwen2-Audio-7B-Instruct", device_map="auto")

conversation = [
    {"role": "user", "content": [
        {"type": "audio", "audio_url": "https://qianwen-res.oss-cn-beijing.aliyuncs.com/Qwen2-Audio/audio/guess_age_gender.wav"},
    ]},
    {"role": "assistant", "content": "Yes, the speaker is female and in her twenties."},
    {"role": "user", "content": [
        {"type": "audio", "audio_url": "https://qianwen-res.oss-cn-beijing.aliyuncs.com/Qwen2-Audio/audio/translate_to_chinese.wav"},
    ]},
]
text = processor.apply_chat_template(conversation, add_generation_prompt=True, tokenize=False)
audios = []
for message in conversation:
    if isinstance(message["content"], list):
        for ele in message["content"]:
            if ele["type"] == "audio":
                audios.append(librosa.load(
                    BytesIO(urlopen(ele['audio_url']).read()),
                    sr=processor.feature_extractor.sampling_rate)[0]
                )

inputs = processor(text=text, audio=audios, return_tensors="pt", padding=True).to(model.device)
inputs.input_ids = inputs.input_ids.to(model.device)

generate_ids = model.generate(**inputs, max_length=256)
generate_ids = generate_ids[:, inputs.input_ids.size(1):]

response = processor.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
```

### Audio Analysis Inference

In the audio analysis, users could provide both audio and text instructions for analysis:

```python
from io import BytesIO
from urllib.request import urlopen

import librosa

from transformers import AutoProcessor, Qwen2AudioForConditionalGeneration

processor = AutoProcessor.from_pretrained("Qwen/Qwen2-Audio-7B-Instruct")
model = Qwen2AudioForConditionalGeneration.from_pretrained("Qwen/Qwen2-Audio-7B-Instruct", device_map="auto")

conversation = [
    {'role': 'system', 'content': 'You are a helpful assistant.'},
    {"role": "user", "content": [
        {"type": "audio", "audio_url": "https://qianwen-res.oss-cn-beijing.aliyuncs.com/Qwen2-Audio/audio/glass-breaking-151256.mp3"},
        {"type": "text", "text": "What's that sound?"},
    ]},
    {"role": "assistant", "content": "It is the sound of glass shattering."},
    {"role": "user", "content": [
        {"type": "text", "text": "What can you do when you hear that?"},
    ]},
    {"role": "assistant", "content": "Stay alert and cautious, and check if anyone is hurt or if there is any damage to property."},
    {"role": "user", "content": [
        {"type": "audio", "audio_url": "https://qianwen-res.oss-cn-beijing.aliyuncs.com/Qwen2-Audio/audio/1272-128104-0000.flac"},
        {"type": "text", "text": "What does the person say?"},
    ]},
]
text = processor.apply_chat_template(conversation, add_generation_prompt=True, tokenize=False)
audios = []
for message in conversation:
    if isinstance(message["content"], list):
        for ele in message["content"]:
            if ele["type"] == "audio":
                audios.append(
                    librosa.load(
                        BytesIO(urlopen(ele['audio_url']).read()),
                        sr=processor.feature_extractor.sampling_rate)[0]
                )

inputs = processor(text=text, audio=audios, return_tensors="pt", padding=True).to(model.device)
inputs.input_ids = inputs.input_ids.to(model.device)

generate_ids = model.generate(**inputs, max_length=256)
generate_ids = generate_ids[:, inputs.input_ids.size(1):]

response = processor.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
```

### Batch Inference

We also support batch inference:

```python
from io import BytesIO
from urllib.request import urlopen

import librosa

from transformers import AutoProcessor, Qwen2AudioForConditionalGeneration

processor = AutoProcessor.from_pretrained("Qwen/Qwen2-Audio-7B-Instruct")
model = Qwen2AudioForConditionalGeneration.from_pretrained("Qwen/Qwen2-Audio-7B-Instruct", device_map="auto")

conversation1 = [
    {"role": "user", "content": [
        {"type": "audio", "audio_url": "https://qianwen-res.oss-cn-beijing.aliyuncs.com/Qwen2-Audio/audio/glass-breaking-151256.mp3"},
        {"type": "text", "text": "What's that sound?"},
    ]},
    {"role": "assistant", "content": "It is the sound of glass shattering."},
    {"role": "user", "content": [
        {"type": "audio", "audio_url": "https://qianwen-res.oss-cn-beijing.aliyuncs.com/Qwen2-Audio/audio/f2641_0_throatclearing.wav"},
        {"type": "text", "text": "What can you hear?"},
    ]}
]

conversation2 = [
    {"role": "user", "content": [
        {"type": "audio", "audio_url": "https://qianwen-res.oss-cn-beijing.aliyuncs.com/Qwen2-Audio/audio/1272-128104-0000.flac"},
        {"type": "text", "text": "What does the person say?"},
    ]},
]

conversations = [conversation1, conversation2]

text = [processor.apply_chat_template(conversation, add_generation_prompt=True, tokenize=False) for conversation in conversations]

audios = []
for conversation in conversations:
    for message in conversation:
        if isinstance(message["content"], list):
            for ele in message["content"]:
                if ele["type"] == "audio":
                    audios.append(
                        librosa.load(
                            BytesIO(urlopen(ele['audio_url']).read()),
                            sr=processor.feature_extractor.sampling_rate)[0]
                    )

inputs = processor(text=text, audio=audios, return_tensors="pt", padding=True).to(model.device)
inputs['input_ids'] = inputs['input_ids'].to(model.device)
inputs.input_ids = inputs.input_ids.to(model.device)

generate_ids = model.generate(**inputs, max_length=256)
generate_ids = generate_ids[:, inputs.input_ids.size(1):]

response = processor.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)
```

## Qwen2AudioConfig[[transformers.Qwen2AudioConfig]]

#### transformers.Qwen2AudioConfig[[transformers.Qwen2AudioConfig]]

```python
transformers.Qwen2AudioConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, audio_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, audio_token_index: int = 151646)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_audio/configuration_qwen2_audio.py#L69)

**Parameters:**

audio_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the audio backbone.

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

audio_token_index (`int`, *optional*, defaults to `151646`) : The audio token index used as a placeholder for input audio.

This is the configuration class to store the configuration of a Qwen2AudioModel. It is used to instantiate a Qwen2 Audio
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Qwen/Qwen2-Audio-7B](https://huggingface.co/Qwen/Qwen2-Audio-7B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Qwen2AudioForConditionalGeneration, Qwen2AudioConfig, Qwen2AudioEncoderConfig, Qwen2Config

>>> # Initializing a Qwen2AudioEncoder config
>>> audio_config = Qwen2AudioEncoderConfig()

>>> # Initializing a Qwen2 config
>>> text_config = Qwen2Config()

>>> # Initializing a Qwen2Audio configuration
>>> configuration = Qwen2AudioConfig(audio_config, text_config)

>>> # Initializing a model from the qwen2-audio style configuration
>>> model = Qwen2AudioForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Qwen2AudioEncoderConfig[[transformers.Qwen2AudioEncoderConfig]]

#### transformers.Qwen2AudioEncoderConfig[[transformers.Qwen2AudioEncoderConfig]]

```python
transformers.Qwen2AudioEncoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, num_mel_bins: int = 128, encoder_layers: int = 32, encoder_attention_heads: int = 20, encoder_ffn_dim: int = 5120, encoder_layerdrop: float | int = 0.0, d_model: int = 1280, dropout: float | int = 0.0, attention_dropout: float | int = 0.0, activation_function: str = 'gelu', activation_dropout: float | int = 0.0, scale_embedding: bool = False, initializer_range: float = 0.02, max_source_positions: int = 1500)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_audio/configuration_qwen2_audio.py#L24)

**Parameters:**

num_mel_bins (`int`, *optional*, defaults to `128`) : Number of mel features used per input frame. Should correspond to the value used in the `AutoFeatureExtractor` class.

encoder_layers (`int`, *optional*, defaults to `32`) : Number of hidden layers in the Transformer encoder. Will use the same value as `num_layers` if not set.

encoder_attention_heads (`int`, *optional*, defaults to `20`) : Number of attention heads for each attention layer in the Transformer encoder.

encoder_ffn_dim (`int`, *optional*, defaults to `5120`) : Dimensionality of the "intermediate" (often named feed-forward) layer in encoder.

encoder_layerdrop (`Union[float, int]`, *optional*, defaults to `0.0`) : The LayerDrop probability for the encoder. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556) for more details.

d_model (`int`, *optional*, defaults to `1280`) : Size of the encoder layers and the pooler layer.

dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The ratio for all dropout layers.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

activation_function (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

activation_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for activations inside the fully connected layer.

scale_embedding (`bool`, *optional*, defaults to `False`) : Whether to scale embeddings by dividing by sqrt(d_model).

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

max_source_positions (`int`, *optional*, defaults to 1500) : The maximum sequence length of log-mel filter-bank features that this model might ever be used with.

This is the configuration class to store the configuration of a Qwen2AudioModel. It is used to instantiate a Qwen2 Audio
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Qwen/Qwen2-Audio-7B](https://huggingface.co/Qwen/Qwen2-Audio-7B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Qwen2AudioEncoderConfig, Qwen2AudioEncoder

>>> # Initializing a Qwen2AudioEncoderConfig
>>> configuration = Qwen2AudioEncoderConfig()

>>> # Initializing a Qwen2AudioEncoder (with random weights)
>>> model = Qwen2AudioEncoder(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Qwen2AudioProcessor[[transformers.Qwen2AudioProcessor]]

#### transformers.Qwen2AudioProcessor[[transformers.Qwen2AudioProcessor]]

```python
transformers.Qwen2AudioProcessor(feature_extractor = None, tokenizer = None, chat_template = None, audio_token = '<|AUDIO|>', audio_bos_token = '<|audio_bos|>', audio_eos_token = '<|audio_eos|>')
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_audio/processing_qwen2_audio.py#L36)

**Parameters:**

feature_extractor (`WhisperFeatureExtractor`) : The feature extractor is a required input.

tokenizer (`Qwen2Tokenizer`) : The tokenizer is a required input.

chat_template (`str`) : A Jinja template to convert lists of messages in a chat into a tokenizable string.

audio_token (`str`, *optional*, defaults to `"<|AUDIO|>"`) : The token to use for audio tokens.

audio_bos_token (`str`, *optional*, defaults to `"<|audio_bos|>"`) : The token to use for audio bos tokens.

audio_eos_token (`str`, *optional*, defaults to `"<|audio_eos|>"`) : The token to use for audio eos tokens.

Constructs a Qwen2AudioProcessor which wraps a feature extractor and a tokenizer into a single processor.

[Qwen2AudioProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_audio#transformers.Qwen2AudioProcessor) offers all the functionalities of [WhisperFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/whisper#transformers.WhisperFeatureExtractor) and [Qwen2Tokenizer](/docs/transformers/v5.15.1/en/model_doc/qwen2#transformers.Qwen2Tokenizer). See the
[~WhisperFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/whisper#transformers.WhisperFeatureExtractor) and [~Qwen2Tokenizer](/docs/transformers/v5.15.1/en/model_doc/qwen2#transformers.Qwen2Tokenizer) for more information.

## Qwen2AudioEncoder[[transformers.Qwen2AudioEncoder]]

#### transformers.Qwen2AudioEncoder[[transformers.Qwen2AudioEncoder]]

```python
transformers.Qwen2AudioEncoder(config: Qwen2AudioEncoderConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_audio/modeling_qwen2_audio.py#L289)

**Parameters:**

config ([Qwen2AudioEncoderConfig](/docs/transformers/v5.15.1/en/model_doc/qwen2_audio#transformers.Qwen2AudioEncoderConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The audio model from Qwen2Audio without any head or projection on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Qwen2AudioEncoder.forward]]

```python
forward(input_features, attention_mask = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_audio/modeling_qwen2_audio.py#L341)

**Parameters:**

attention_mask (`torch.Tensor`)`, *optional*) : Qwen2Audio does not support masking of the `input_features`, this argument is preserved for compatibility, but it is not used. By default the silence in the input log mel spectrogram are ignored.

## Qwen2AudioModel[[transformers.Qwen2AudioModel]]

#### transformers.Qwen2AudioModel[[transformers.Qwen2AudioModel]]

```python
transformers.Qwen2AudioModel(config: Qwen2AudioConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_audio/modeling_qwen2_audio.py#L424)

**Parameters:**

config ([Qwen2AudioConfig](/docs/transformers/v5.15.1/en/model_doc/qwen2_audio#transformers.Qwen2AudioConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Qwen2Audio model which consists of an audio backbone and a language model, without a language modeling head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Qwen2AudioModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, input_features: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, feature_attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_audio/modeling_qwen2_audio.py#L646)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) : The tensors corresponding to the input audio features. Audio features can be obtained using [WhisperFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/whisper#transformers.WhisperFeatureExtractor). See [WhisperFeatureExtractor.__call__()](/docs/transformers/v5.15.1/en/model_doc/whisper#transformers.WhisperFeatureExtractor.__call__) for details ([Qwen2AudioProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_audio#transformers.Qwen2AudioProcessor) uses [WhisperFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/whisper#transformers.WhisperFeatureExtractor) for processing audios).

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

feature_attention_mask (`torch.Tensor` of shape `(batch_size, feature_sequence_length)`) : Mask to avoid performing attention on padding feature indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels kept in the signature for the legacy merge path that may re-align them with audio tokens. The loss is not computed here; `Qwen2AudioForConditionalGeneration` is responsible for that.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** `Qwen2AudioModelOutputWithPast` or `tuple(torch.FloatTensor)`

A `Qwen2AudioModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen2AudioConfig](/docs/transformers/v5.15.1/en/model_doc/qwen2_audio#transformers.Qwen2AudioConfig)) and inputs.

The [Qwen2AudioModel](/docs/transformers/v5.15.1/en/model_doc/qwen2_audio#transformers.Qwen2AudioModel) forward method, overrides the `__call__` special method.

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
- **attention_mask** (`torch.FloatTensor`, *optional*) -- Attention mask, potentially updated by the audio merging logic so that audio tokens are unmasked.
- **labels** (`torch.LongTensor`, *optional*) -- Labels, potentially re-aligned by the legacy audio merging logic. Returned so the language-modeling
  head can compute the loss against the expanded sequence.

## Qwen2AudioForConditionalGeneration[[transformers.Qwen2AudioForConditionalGeneration]]

#### transformers.Qwen2AudioForConditionalGeneration[[transformers.Qwen2AudioForConditionalGeneration]]

```python
transformers.Qwen2AudioForConditionalGeneration(config: Qwen2AudioConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_audio/modeling_qwen2_audio.py#L768)

**Parameters:**

config ([Qwen2AudioConfig](/docs/transformers/v5.15.1/en/model_doc/qwen2_audio#transformers.Qwen2AudioConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The QWEN2AUDIO model which consists of an audio backbone and a language model.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Qwen2AudioForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, input_features: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, feature_attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_audio/modeling_qwen2_audio.py#L783)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) : The tensors corresponding to the input audio features. Audio features can be obtained using [WhisperFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/whisper#transformers.WhisperFeatureExtractor). See [WhisperFeatureExtractor.__call__()](/docs/transformers/v5.15.1/en/model_doc/whisper#transformers.WhisperFeatureExtractor.__call__) for details ([Qwen2AudioProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_audio#transformers.Qwen2AudioProcessor) uses [WhisperFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/whisper#transformers.WhisperFeatureExtractor) for processing audios).

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

feature_attention_mask (`torch.Tensor` of shape `(batch_size, feature_sequence_length)`) : Mask to avoid performing attention on padding feature indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** `Qwen2AudioCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`

A `Qwen2AudioCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen2AudioConfig](/docs/transformers/v5.15.1/en/model_doc/qwen2_audio#transformers.Qwen2AudioConfig)) and inputs.

The [Qwen2AudioForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/qwen2_audio#transformers.Qwen2AudioForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- Pre-computed hidden-states that can be used to speed up auto-regressive (sequential) decoding. There are
  two sets of pre-computed hidden-states: key and values states in the self-attention blocks.
  The `past_key_values` are returned when `use_cache=True` is passed or when `config.use_cache=True`.
  It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance.

  If `past_key_values` are used, the user can optionally input only the last `input_ids` (those
  that don't have their past key value states given to this model) of shape `(batch_size, 1)` instead of
  all `input_ids` of shape `(batch_size, sequence_length)`.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **attention_mask** (`torch.FloatTensor`, *optional*) -- Attentions mask, used to update attention mask and position_ids.

Example:

```python
>>> from io import BytesIO
>>> from urllib.request import urlopen
>>> import librosa
>>> from transformers import AutoProcessor, Qwen2AudioForConditionalGeneration

>>> model = Qwen2AudioForConditionalGeneration.from_pretrained("Qwen/Qwen2-Audio-7B")
>>> processor = AutoProcessor.from_pretrained("Qwen/Qwen2-Audio-7B")

>>> prompt = "<|audio_bos|><|AUDIO|><|audio_eos|>Generate the caption in English:"
>>> url = "https://qianwen-res.oss-cn-beijing.aliyuncs.com/Qwen2-Audio/audio/glass-breaking-151256.mp3"
>>> audio, _ = librosa.load(BytesIO(urlopen(url).read()), sr=self.processor.feature_extractor.sampling_rate)

>>> inputs = processor(text=prompt, audio=audio, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(**inputs, max_length=30)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"Generate the caption in English: Glass is breaking."
```
