# VoxtralRealtime

VoxtralRealtime is a streaming speech-to-text model from [Mistral AI](https://mistral.ai), designed for real-time automatic speech recognition (ASR). Unlike the offline [Voxtral](./voxtral) model which processes complete audio files, VoxtralRealtime is architected for low-latency, incremental transcription by processing audio in chunks as they arrive.

The model combines an audio encoder with a Mistral-based language model decoder, using time conditioning embeddings and causal convolutions with padding caches to enable efficient streaming inference.

## Usage

### Offline Transcription

For transcribing complete audio files, use the processor and model directly. The generation length is automatically determined from the audio length.

```python
from datasets import load_dataset

from transformers import AutoProcessor, VoxtralRealtimeForConditionalGeneration

repo_id = "mistralai/Voxtral-Mini-4B-Realtime-2602"

processor = AutoProcessor.from_pretrained(repo_id)
model = VoxtralRealtimeForConditionalGeneration.from_pretrained(repo_id, device_map="auto")

ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
audio = ds[0]["audio"]["array"]

inputs = processor(audio, return_tensors="pt").to(model.device)
inputs = inputs.to(model.device, dtype=model.dtype)

outputs = model.generate(**inputs)
decoded_outputs = processor.batch_decode(outputs, skip_special_tokens=True)

print(decoded_outputs[0])
```

### Batched Offline Transcription

Multiple audio samples can be transcribed in a single forward pass:

```python
from datasets import load_dataset

from transformers import AutoProcessor, VoxtralRealtimeForConditionalGeneration

repo_id = "mistralai/Voxtral-Mini-4B-Realtime-2602"

processor = AutoProcessor.from_pretrained(repo_id)
model = VoxtralRealtimeForConditionalGeneration.from_pretrained(repo_id, device_map="auto")

ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
audio = [ds[i]["audio"]["array"] for i in range(2)]

inputs = processor(audio, return_tensors="pt").to(model.device)
inputs = inputs.to(model.device, dtype=model.dtype)

outputs = model.generate(**inputs)
decoded_outputs = processor.batch_decode(outputs, skip_special_tokens=True)

for decoded_output in decoded_outputs:
    print(decoded_output)
```

### Streaming Transcription
> [!NOTE]
> This is an experimental feature and the API is subject to change.

For real-time transcription, audio is split into chunks following:

```python
from threading import Thread

import numpy as np
import torch
from datasets import load_dataset

from transformers import (
    TextIteratorStreamer,
    VoxtralRealtimeForConditionalGeneration,
    VoxtralRealtimeProcessor,
)

model_id = "mistralai/Voxtral-Mini-4B-Realtime-2602"
processor = VoxtralRealtimeProcessor.from_pretrained(model_id)
model = VoxtralRealtimeForConditionalGeneration.from_pretrained(model_id, device_map=torch.accelerator.current_accelerator())

ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
audio = ds[0]["audio"]["array"]
# Manually pad the audio to account for right padding tokens required by the model
xaudio = np.pad(audio, (0, processor.num_right_pad_tokens * processor.raw_audio_length_per_tok))

first_chunk_inputs = processor(
    audio[:processor.num_samples_first_audio_chunk],
    is_streaming=True,
    is_first_audio_chunk=True,
    return_tensors="pt"
)
first_chunk_inputs.to(model.device, dtype=model.dtype)

def input_features_generator():
    yield first_chunk_inputs.input_features

    mel_frame_idx = processor.num_mel_frames_first_audio_chunk
    hop_length = processor.feature_extractor.hop_length
    win_length = processor.feature_extractor.win_length

    start_idx = mel_frame_idx * hop_length - win_length // 2
    end_idx = start_idx + processor.num_samples_per_audio_chunk

    while (end_idx:=start_idx + processor.num_samples_per_audio_chunk) < audio.shape[0]:
        inputs = processor(
            audio[start_idx:end_idx],
            is_streaming=True,
            is_first_audio_chunk=False,
            return_tensors="pt"
        )
        inputs.to(model.device, dtype=model.dtype)
        yield inputs.input_features

        mel_frame_idx += processor.audio_length_per_tok
        start_idx = mel_frame_idx * hop_length - win_length // 2

streamer = TextIteratorStreamer(processor.tokenizer, skip_special_tokens=True, clean_up_tokenization_spaces=True)
generate_kwargs = {
    "input_ids": first_chunk_inputs.input_ids,
    "input_features": input_features_generator(),
    "num_delay_tokens": first_chunk_inputs.num_delay_tokens,
    "streamer": streamer,
}
thread = Thread(target=model.generate, kwargs=generate_kwargs)
thread.start()

# Iterate over the streamer to get text chunks as they are generated
print("Model output (streaming):", end=" ", flush=True)
for text_chunk in streamer:
    print(text_chunk, end="", flush=True)
```

This model was contributed by [Eustache Le Bihan](https://huggingface.co/eustlb).

## VoxtralRealtimeConfig[[transformers.VoxtralRealtimeConfig]]

#### transformers.VoxtralRealtimeConfig[[transformers.VoxtralRealtimeConfig]]

```python
transformers.VoxtralRealtimeConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, audio_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, projector_hidden_act: str = 'gelu', audio_length_per_tok: int = 8, default_num_delay_tokens: int = 6, downsample_factor: int = 4, tie_word_embeddings: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/voxtral_realtime/configuration_voxtral_realtime.py#L124)

**Parameters:**

audio_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the audio backbone.

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

projector_hidden_act (`str`, *optional*, defaults to `gelu`) : The activation function used by the multimodal projector.

audio_length_per_tok (`int`, *optional*, defaults to 8) : The number of audio frames corresponding to each text token.

default_num_delay_tokens (`int`, *optional*, defaults to 6) : The default number of delay tokens used for streaming.

downsample_factor (`int`, *optional*, defaults to 4) : The downsampling factor applied to audio features before projection.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a VoxtralRealtimeModel. It is used to instantiate a Voxtral Realtime
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [mistralai/Voxtral-Mini-4B-Realtime-2602](https://huggingface.co/mistralai/Voxtral-Mini-4B-Realtime-2602)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import VoxtralRealtimeForConditionalGeneration, VoxtralRealtimeConfig

>>> # Initializing a VoxtralRealtime configuration
>>> configuration = VoxtralRealtimeConfig()

>>> # Initializing a model with random weights
>>> model = VoxtralRealtimeForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## VoxtralRealtimeEncoderConfig[[transformers.VoxtralRealtimeEncoderConfig]]

#### transformers.VoxtralRealtimeEncoderConfig[[transformers.VoxtralRealtimeEncoderConfig]]

```python
transformers.VoxtralRealtimeEncoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 131072, hidden_size: int = 1280, intermediate_size: int = 5120, num_hidden_layers: int = 32, num_attention_heads: int = 32, activation_function: str = 'gelu', num_mel_bins: int = 128, initializer_range: float = 0.02, attention_dropout: float | int = 0.0, hidden_act: str = 'silu', max_position_embeddings: int = 1500, rms_norm_eps: float = 1e-05, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, sliding_window: int = 750, head_dim: int = 64)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/voxtral_realtime/configuration_voxtral_realtime.py#L73)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `131072`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `1280`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `5120`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `32`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `32`) : Number of attention heads for each attention layer in the Transformer decoder.

activation_function (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

num_mel_bins (`int`, *optional*, defaults to `128`) : Number of mel features used per input frame. Should correspond to the value used in the `AutoFeatureExtractor` class.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `1500`) : The maximum sequence length that this model might ever be used with.

rms_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the rms normalization layers.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

sliding_window (`int`, *optional*, defaults to `750`) : Sliding window attention window size. If `None`, no sliding window is applied.

head_dim (`int`, *optional*, defaults to `64`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

This is the configuration class to store the configuration of a VoxtralRealtimeModel. It is used to instantiate a Voxtral Realtime
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [mistralai/Voxtral-Mini-4B-Realtime-2602](https://huggingface.co/mistralai/Voxtral-Mini-4B-Realtime-2602)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import VoxtralRealtimeEncoderConfig, VoxtralRealtimeEncoder

>>> # Initializing a VoxtralRealtimeEncoderConfig
>>> configuration = VoxtralRealtimeEncoderConfig()

>>> # Initializing a VoxtralRealtimeEncoder (with random weights)
>>> model = VoxtralRealtimeEncoder(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## VoxtralRealtimeTextConfig[[transformers.VoxtralRealtimeTextConfig]]

#### transformers.VoxtralRealtimeTextConfig[[transformers.VoxtralRealtimeTextConfig]]

```python
transformers.VoxtralRealtimeTextConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 32000, hidden_size: int = 4096, intermediate_size: int = 14336, num_hidden_layers: int = 32, num_attention_heads: int = 32, num_key_value_heads: int = 8, head_dim: int | None = None, hidden_act: str = 'silu', max_position_embeddings: int = 131072, initializer_range: float = 0.02, rms_norm_eps: float = 1e-06, use_cache: bool = True, pad_token_id: int | None = None, bos_token_id: int | None = 1, eos_token_id: int | list[int] | None = 2, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, sliding_window: int | None = 4096, attention_dropout: float | int = 0.0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/voxtral_realtime/configuration_voxtral_realtime.py#L26)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `32000`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `4096`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `14336`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `32`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `32`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `8`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

head_dim (`int`, *optional*) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `131072`) : The maximum sequence length that this model might ever be used with.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

rms_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the rms normalization layers.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

pad_token_id (`int`, *optional*) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `1`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `2`) : Token id used for end-of-stream in the vocabulary.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

sliding_window (`int`, *optional*, defaults to `4096`) : Sliding window attention window size. If `None`, no sliding window is applied.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

This is the configuration class to store the configuration of a VoxtralRealtimeModel. It is used to instantiate a Voxtral Realtime
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [mistralai/Voxtral-Mini-4B-Realtime-2602](https://huggingface.co/mistralai/Voxtral-Mini-4B-Realtime-2602)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## VoxtralRealtimeFeatureExtractor[[transformers.VoxtralRealtimeFeatureExtractor]]

#### transformers.VoxtralRealtimeFeatureExtractor[[transformers.VoxtralRealtimeFeatureExtractor]]

```python
transformers.VoxtralRealtimeFeatureExtractor(feature_size = 128, sampling_rate = 16000, hop_length = 160, n_fft = 400, win_length = 400, padding_value = 0.0, global_log_mel_max = 1.5, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/voxtral_realtime/feature_extraction_voxtral_realtime.py#L32)

**Parameters:**

feature_size (`int`, *optional*, defaults to 128) : The feature dimension of the extracted features.

sampling_rate (`int`, *optional*, defaults to 16000) : The sampling rate at which the audio files should be digitalized expressed in hertz (Hz).

hop_length (`int`, *optional*, defaults to 160) : Length of the overlapping windows for the STFT used to obtain the Mel Frequency coefficients.

n_fft (`int`, *optional*, defaults to 512) : Size of the Fourier transform.

win_length (`int`, *optional*, defaults to 400) : The window length for the STFT computation.

padding_value (`float`, *optional*, defaults to 0.0) : Padding value used to pad the audio. Should correspond to silences.

Constructs a VOXTRAL_REALTIME feature extractor.

This feature extractor inherits from [SequenceFeatureExtractor](/docs/transformers/v5.15.1/en/main_classes/feature_extractor#transformers.SequenceFeatureExtractor) which contains
most of the main methods. Users should refer to this superclass for more information regarding those methods.

This class extracts mel-filter bank features from raw speech using a custom numpy implementation of the `Short Time
Fourier Transform` which should match pytorch's `torch.stft` equivalent.

## VoxtralRealtimeProcessor[[transformers.models.voxtral_realtime.processing_voxtral_realtime._LazyModule.__getattr__..Placeholder]]

#### transformers.models.voxtral_realtime.processing_voxtral_realtime._LazyModule.__getattr__..Placeholder[[transformers.models.voxtral_realtime.processing_voxtral_realtime._LazyModule.__getattr__..Placeholder]]

```python
transformers.models.voxtral_realtime.processing_voxtral_realtime._LazyModule.__getattr__.<locals>.Placeholder(*args, **kwargs)
```

#### __call__[[transformers.models.voxtral_realtime.processing_voxtral_realtime._LazyModule.__getattr__..Placeholder.__call__]]

```python
__call__(*args, **kwargs)
```

Call self as a function.

## VoxtralRealtimeEncoder[[transformers.VoxtralRealtimeEncoder]]

#### transformers.VoxtralRealtimeEncoder[[transformers.VoxtralRealtimeEncoder]]

```python
transformers.VoxtralRealtimeEncoder(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/voxtral_realtime/modeling_voxtral_realtime.py#L512)

**Parameters:**

config ([VoxtralRealtimeEncoder](/docs/transformers/v5.15.1/en/model_doc/voxtral_realtime#transformers.VoxtralRealtimeEncoder)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The VoxtralRealtime encoder, which is a Whisper encoder.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.VoxtralRealtimeEncoder.forward]]

```python
forward(input_features: typing.Optional[torch.FloatTensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, padding_cache: transformers.models.voxtral_realtime.modeling_voxtral_realtime.VoxtralRealtimeConv1dPaddingCache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, use_padding_cache: bool | None = None, attention_mask: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/voxtral_realtime/modeling_voxtral_realtime.py#L543)

**Parameters:**

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) : The tensors corresponding to the input audio features. Audio features can be obtained using [VoxtralRealtimeFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/voxtral_realtime#transformers.VoxtralRealtimeFeatureExtractor). See `VoxtralRealtimeFeatureExtractor.__call__()` for details ([VoxtralRealtimeProcessor](/docs/transformers/v5.15.1/en/model_doc/voxtral_realtime#transformers.models.voxtral_realtime.processing_voxtral_realtime._LazyModule.__getattr__..Placeholder) uses [VoxtralRealtimeFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/voxtral_realtime#transformers.VoxtralRealtimeFeatureExtractor) for processing audios).

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

padding_cache (`VoxtralRealtimeConv1dPaddingCache`, *optional*) : Cache for padding in convolutional layers to maintain state across streaming chunks.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

use_padding_cache (`bool`, *optional*) : Whether to use the padding cache.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([VoxtralRealtimeConfig](/docs/transformers/v5.15.1/en/model_doc/voxtral_realtime#transformers.VoxtralRealtimeConfig)) and inputs.

The [VoxtralRealtimeEncoder](/docs/transformers/v5.15.1/en/model_doc/voxtral_realtime#transformers.VoxtralRealtimeEncoder) forward method, overrides the `__call__` special method.

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

## VoxtralRealtimeModel[[transformers.VoxtralRealtimeModel]]

#### transformers.VoxtralRealtimeModel[[transformers.VoxtralRealtimeModel]]

```python
transformers.VoxtralRealtimeModel(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/voxtral_realtime/modeling_voxtral_realtime.py#L876)

**Parameters:**

config ([VoxtralRealtimeModel](/docs/transformers/v5.15.1/en/model_doc/voxtral_realtime#transformers.VoxtralRealtimeModel)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The VoxtralRealtime model, which consists of a streaming Whisper-style encoder, a multi-modal projector,
a Mistral-based language model and a time embedding, without a language modeling head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.VoxtralRealtimeModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, input_features: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, encoder_past_key_values: transformers.cache_utils.Cache | None = None, padding_cache: transformers.models.voxtral_realtime.modeling_voxtral_realtime.VoxtralRealtimeConv1dPaddingCache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, encoder_inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, num_delay_tokens: typing.Union[int, torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/voxtral_realtime/modeling_voxtral_realtime.py#L933)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) : The tensors corresponding to the input audio features. Audio features can be obtained using [VoxtralRealtimeFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/voxtral_realtime#transformers.VoxtralRealtimeFeatureExtractor). See `VoxtralRealtimeFeatureExtractor.__call__()` for details ([VoxtralRealtimeProcessor](/docs/transformers/v5.15.1/en/model_doc/voxtral_realtime#transformers.models.voxtral_realtime.processing_voxtral_realtime._LazyModule.__getattr__..Placeholder) uses [VoxtralRealtimeFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/voxtral_realtime#transformers.VoxtralRealtimeFeatureExtractor) for processing audios).

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

encoder_past_key_values (`Cache`, *optional*) : Pre-computed hidden-states (key and value in the self-attention blocks) for the encoder.

padding_cache (`VoxtralRealtimeConv1dPaddingCache`, *optional*) : Cache for padding in convolutional layers to maintain state across streaming chunks.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

encoder_inputs_embeds (`torch.FloatTensor`, *optional*) : Optionally, instead of passing `input_features` you can choose to directly pass an embedded representation for the encoder.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

num_delay_tokens (`int` or `torch.Tensor`, *optional*) : Number of delay tokens used when preparing inputs.

**Returns:** `VoxtralRealtimeModelOutputWithPast` or `tuple(torch.FloatTensor)`

A `VoxtralRealtimeModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([VoxtralRealtimeConfig](/docs/transformers/v5.15.1/en/model_doc/voxtral_realtime#transformers.VoxtralRealtimeConfig)) and inputs.

The [VoxtralRealtimeModel](/docs/transformers/v5.15.1/en/model_doc/voxtral_realtime#transformers.VoxtralRealtimeModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **encoder_past_key_values** (`Cache`, *optional*) -- Pre-computed hidden-states (key and value in the self-attention blocks) for the audio encoder
  that can be used to speed up sequential decoding.
- **padding_cache** (`VoxtralRealtimeConv1dPaddingCache`, *optional*) -- Cache for padding in convolutional layers to maintain state across streaming chunks.
- **audio_hidden_states** (`torch.FloatTensor`, *optional*) -- Projected audio hidden states before they are added to the text embeddings.

## VoxtralRealtimeForConditionalGeneration[[transformers.VoxtralRealtimeForConditionalGeneration]]

#### transformers.VoxtralRealtimeForConditionalGeneration[[transformers.VoxtralRealtimeForConditionalGeneration]]

```python
transformers.VoxtralRealtimeForConditionalGeneration(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/voxtral_realtime/modeling_voxtral_realtime.py#L1023)

#### forward[[transformers.VoxtralRealtimeForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, input_features: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, encoder_past_key_values: transformers.cache_utils.Cache | None = None, padding_cache: transformers.models.voxtral_realtime.modeling_voxtral_realtime.VoxtralRealtimeConv1dPaddingCache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, encoder_inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, num_delay_tokens: typing.Union[int, torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/voxtral_realtime/modeling_voxtral_realtime.py#L1035)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) : The tensors corresponding to the input audio features. Audio features can be obtained using [VoxtralRealtimeFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/voxtral_realtime#transformers.VoxtralRealtimeFeatureExtractor). See `VoxtralRealtimeFeatureExtractor.__call__()` for details ([VoxtralRealtimeProcessor](/docs/transformers/v5.15.1/en/model_doc/voxtral_realtime#transformers.models.voxtral_realtime.processing_voxtral_realtime._LazyModule.__getattr__..Placeholder) uses [VoxtralRealtimeFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/voxtral_realtime#transformers.VoxtralRealtimeFeatureExtractor) for processing audios).

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

encoder_past_key_values (`Cache`, *optional*) : Pre-computed hidden-states (key and value in the self-attention blocks) for the encoder that can be used to speed up sequential decoding.

padding_cache (`VoxtralRealtimeConv1dPaddingCache`, *optional*) : Cache for padding in convolutional layers to maintain state across streaming chunks.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

encoder_inputs_embeds (`torch.FloatTensor`, *optional*) : Optionally, instead of passing `input_features` you can choose to directly pass an embedded representation for the encoder.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

num_delay_tokens (`int` or `torch.Tensor`, *optional*) : Number of delay tokens used when preparing inputs, see [~VoxtralRealtimeProcessor](/docs/transformers/v5.15.1/en/model_doc/voxtral_realtime#transformers.models.voxtral_realtime.processing_voxtral_realtime._LazyModule.__getattr__..Placeholder) for more details.

**Returns:** `VoxtralRealtimeCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`

A `VoxtralRealtimeCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([VoxtralRealtimeConfig](/docs/transformers/v5.15.1/en/model_doc/voxtral_realtime#transformers.VoxtralRealtimeConfig)) and inputs.

The [VoxtralRealtimeForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/voxtral_realtime#transformers.VoxtralRealtimeForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **encoder_past_key_values** (`Cache`, *optional*) -- Pre-computed hidden-states (key and value in the self-attention blocks) for the audio encoder
  that can be used to speed up sequential decoding.
- **padding_cache** (`VoxtralRealtimeConv1dPaddingCache`, *optional*) -- Cache for padding in convolutional layers to maintain state across streaming chunks.

Example:

```python
>>> import torch
>>> from transformers import VoxtralRealtimeForConditionalGeneration, AutoProcessor
>>> from datasets import load_dataset

>>> repo_id = "mistralai/Voxtral-Mini-4B-Realtime-2602"

>>> processor = AutoProcessor.from_pretrained(repo_id)
>>> model = VoxtralRealtimeForConditionalGeneration.from_pretrained(repo_id, dtype=torch.bfloat16, device_map="auto")

>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> audio = ds[0]["audio"]["array"]

>>> inputs = processor(audio, return_tensors="pt")
>>> inputs = inputs.to(model.device, dtype=model.dtype)

>>> outputs = model.generate(**inputs)
>>> processor.batch_decode(outputs, skip_special_tokens=True)
```

#### get_audio_features[[transformers.VoxtralRealtimeForConditionalGeneration.get_audio_features]]

```python
get_audio_features(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/voxtral_realtime/modeling_voxtral_realtime.py#L1032)
