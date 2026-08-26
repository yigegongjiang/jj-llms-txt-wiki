# Nemotron 3.5 ASR

## Overview

Nemotron 3.5 ASR is a 600M-parameter multilingual speech recognition model from NVIDIA, built for high-quality transcription in both low-latency streaming and high-throughput batch settings, with native punctuation and capitalization. For streaming, it offers configurable chunk sizes—80ms, 160ms, 560ms, and 1120ms, letting users trade off latency against accuracy to suit their application. Its cache-aware FastConformer-RNNT architecture is central to this capability: unlike traditional buffered streaming, which repeatedly reprocesses overlapping audio windows, the model processes only each new incoming chunk while reusing cached encoder context from prior chunks. This eliminates redundant computation, significantly improves efficiency, and minimizes end-to-end delay without sacrificing accuracy, making it well suited to real-time transcription workloads.

## Usage

### Offline transcription

```python
from transformers import pipeline

pipe = pipeline(
    "automatic-speech-recognition",
    model="nvidia/nemotron-3.5-asr-streaming-0.6b",
)
out = pipe("https://huggingface.co/datasets/hf-internal-testing/dummy-audio-samples/resolve/main/bcn_weather.mp3")
print(out)
```

> [!NOTE]
> The pipeline uses the default language prompt (index 0, `en-US`). For explicit language conditioning or automatic detection, pass the processor's `language` argument (see the AutoModel tab).

The language prompt is created by the processor, so the language travels with the inputs into `generate`.

```python
from transformers import AutoModelForRNNT, AutoProcessor
from transformers.audio_utils import load_audio

model_id = "nvidia/nemotron-3.5-asr-streaming-0.6b"
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForRNNT.from_pretrained(model_id, device_map="auto")

audio = load_audio(
    "https://huggingface.co/datasets/hf-internal-testing/dummy-audio-samples/resolve/main/bcn_weather.mp3",
    sampling_rate=processor.feature_extractor.sampling_rate,
)

# Condition on a known language ...
inputs = processor(audio, sampling_rate=processor.feature_extractor.sampling_rate, language="en-US")
inputs.to(model.device, dtype=model.dtype)
output = model.generate(**inputs, return_dict_in_generate=True)
print(processor.decode(output.sequences, skip_special_tokens=True))

# ... or let the model detect it and keep the emitted <xx-XX> language tag.
inputs = processor(audio, sampling_rate=processor.feature_extractor.sampling_rate) # equiv to ..., language="auto"
inputs.to(model.device, dtype=model.dtype)
output = model.generate(**inputs, return_dict_in_generate=True)
print(processor.decode(output.sequences, skip_special_tokens=False))
```

### Streaming transcription
> [!NOTE]
> This is an experimental feature and the API is subject to change.

For real-time transcription, audio is split into chunks following:

```python
from threading import Thread
from transformers import AutoModelForRNNT, AutoProcessor, TextIteratorStreamer
from transformers.audio_utils import load_audio

model_id = "nvidia/nemotron-3.5-asr-streaming-0.6b"
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForRNNT.from_pretrained(model_id, device_map="auto")

processor.set_num_lookahead_tokens(6)
print(f"Streaming latency: {processor.streaming_latency_ms} ms")

# The language prompt rides along on every chunk; use a locale (e.g. "de-DE") or "auto".
language = "en-US"

sampling_rate = processor.feature_extractor.sampling_rate
audio = load_audio(
    "https://huggingface.co/datasets/hf-internal-testing/dummy-audio-samples/resolve/main/obama.mp3",
    sampling_rate=sampling_rate,
)

first_chunk_inputs = processor(
    audio[: processor.num_samples_first_audio_chunk],
    sampling_rate=sampling_rate,
    is_streaming=True,
    is_first_audio_chunk=True,
    language=language,
    return_tensors="pt",
)
first_chunk_inputs = first_chunk_inputs.to(model.device, dtype=model.dtype)

def input_features_generator():
    yield first_chunk_inputs.input_features[:, : processor.num_mel_frames_first_audio_chunk, :]

    mel_frame_idx = processor.num_mel_frames_first_audio_chunk
    hop_length = processor.feature_extractor.hop_length
    n_fft = processor.feature_extractor.n_fft

    start_idx = mel_frame_idx * hop_length - n_fft // 2
    while (end_idx := start_idx + processor.num_samples_per_audio_chunk) < audio.shape[0]:
        inputs = processor(
            audio[start_idx:end_idx],
            sampling_rate=sampling_rate,
            is_streaming=True,
            is_first_audio_chunk=False,
            language=language,
            return_tensors="pt",
        )
        inputs = inputs.to(model.device, dtype=model.dtype)
        yield inputs.input_features

        mel_frame_idx += processor.num_mel_frames_per_audio_chunk
        start_idx = mel_frame_idx * hop_length - n_fft // 2

streamer = TextIteratorStreamer(processor.tokenizer, skip_special_tokens=True)
generate_kwargs = {
    **first_chunk_inputs,
    "input_features": input_features_generator(),
    "streamer": streamer,
}
thread = Thread(target=model.generate, kwargs=generate_kwargs)
thread.start()

# Iterate over the streamer to get text chunks as they are generated
print("Model output (streaming):", end=" ", flush=True)
for text_chunk in streamer:
    print(text_chunk, end="", flush=True)
thread.join()
```

#### Streaming latency

The latency is set by `num_lookahead_tokens`, the right attention context (lookahead, in subsampled encoder frames) each chunk waits for before it is emitted. A larger value lets each chunk see more future audio: better accuracy at the cost of higher latency. Inspect the supported trade-offs, select one, and read back the resulting latency:

```python
from transformers import AutoProcessor

processor = AutoProcessor.from_pretrained("nvidia/nemotron-3.5-asr-streaming-0.6b")

# Each supported `num_lookahead_tokens` mapped to its streaming latency in milliseconds:
print(processor.supported_streaming_latencies_ms)
# {3: 320, 0: 80, 6: 560, 13: 1120}

# Select a right attention context (this also re-derives the streaming chunk sizes used above):
processor.set_num_lookahead_tokens(6)

# Latency of the current selection:
print(processor.streaming_latency_ms)
# 560
```

`set_num_lookahead_tokens` sizes the chunks the processor emits, and the matching `num_lookahead_tokens` must reach `generate` (in the snippet above it travels through `**inputs`/`**first_chunk_inputs`, which carries `num_lookahead_tokens`). Streaming `generate` raises if it is omitted.

## Nemotron3_5AsrConfig[[transformers.Nemotron3_5AsrConfig]]

#### transformers.Nemotron3_5AsrConfig[[transformers.Nemotron3_5AsrConfig]]

```python
transformers.Nemotron3_5AsrConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, is_encoder_decoder: bool = True, vocab_size: int = 13088, decoder_hidden_size: int = 640, num_decoder_layers: int = 2, hidden_act: str = 'relu', max_symbols_per_step: int = 10, encoder_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, pad_token_id: int = 0, blank_token_id: int = 13087, num_prompts: int = 128, prompt_intermediate_size: int = 2048, default_prompt_id: int = 101)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nemotron3_5_asr/configuration_nemotron3_5_asr.py#L23)

**Parameters:**

is_encoder_decoder (`bool`, *optional*, defaults to `True`) : Whether the model is used as an encoder/decoder or not.

vocab_size (`int`, *optional*, defaults to 13088) : Vocabulary size of the joint network output (including the blank token).

decoder_hidden_size (`int`, *optional*, defaults to 640) : Hidden size of the LSTM prediction network (NeMo's `pred_hidden`).

num_decoder_layers (`int`, *optional*, defaults to 2) : Number of LSTM layers in the prediction network.

hidden_act (`str`, *optional*, defaults to `"relu"`) : Activation in the joint network.

max_symbols_per_step (`int`, *optional*, defaults to 10) : Maximum number of non-blank symbols emitted per encoder time step during greedy decoding.

encoder_config (`Union[dict, NemotronAsrStreamingEncoderConfig]`, *optional*) : The config object or dictionary of the encoder. Reuses [NemotronAsrStreamingEncoderConfig](/docs/transformers/v5.15.1/en/model_doc/nemotron_asr_streaming#transformers.NemotronAsrStreamingEncoderConfig) directly, since the encoder is identical to `NemotronAsrStreaming`'s.

pad_token_id (`int`, *optional*, defaults to `0`) : Token id used for padding in the vocabulary.

blank_token_id (`int`, *optional*, defaults to 13087) : Blank token id for RNN-T decoding.

num_prompts (`int`, *optional*, defaults to 128) : Number of language-prompt slots. The target language is encoded as a one-hot vector of this size, broadcast across the encoder time axis and concatenated with the encoder output before the `prompt_kernel` fusion MLP.

prompt_intermediate_size (`int`, *optional*, defaults to 2048) : Hidden size of the `prompt_kernel` fusion MLP (`Linear(hidden + num_prompts -> intermediate) -> ReLU -> Linear(intermediate -> hidden)`).

default_prompt_id (`int`, *optional*, defaults to 101) : Prompt index used to condition the model when `prompt_ids` is not provided. Defaults to the `auto` language-detection slot (index 101 in the NeMo prompt dictionary), matching NeMo's default of `target_lang="auto"`: the model detects the language itself and emits an `<xx-XX>` tag.

This is the configuration class to store the configuration of a Nemotron3_5AsrForRNNT. It is used to instantiate a Nemotron3 5 Asr
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [nvidia/nemotron-3.5-asr-streaming-0.6b](https://huggingface.co/nvidia/nemotron-3.5-asr-streaming-0.6b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import Nemotron3_5AsrForRNNT, Nemotron3_5AsrConfig

>>> configuration = Nemotron3_5AsrConfig()
>>> model = Nemotron3_5AsrForRNNT(configuration)
>>> configuration = model.config
```

## Nemotron3_5AsrProcessor[[transformers.Nemotron3_5AsrProcessor]]

#### transformers.Nemotron3_5AsrProcessor[[transformers.Nemotron3_5AsrProcessor]]

```python
transformers.Nemotron3_5AsrProcessor(feature_extractor, tokenizer, blank_token = '<blank>', decoder_type = None, supported_num_lookahead_tokens = None, default_num_lookahead_tokens = None, prompt_dictionary = None, num_prompts = 128)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nemotron3_5_asr/processing_nemotron3_5_asr.py#L187)

**Parameters:**

feature_extractor (`NemotronAsrStreamingFeatureExtractor`) : The feature extractor is a required input.

tokenizer (`ParakeetTokenizer`) : The tokenizer is a required input.

blank_token (`str`, *optional*, defaults to `"<blank>"`) : Blank token for RNN-T decoding.

decoder_type (`str`, *optional*) : Decoding/timestamp emission mode (e.g. `"ctc"`, `"rnnt"`, `"tdt"`). If `None` the decoder type is inferred automatically for backward compatibility.

supported_num_lookahead_tokens (`list[int]`, *optional*) : Supported right attention contexts (lookaheads, in subsampled encoder frames), mirroring `NemotronAsrStreamingEncoderConfig.supported_num_lookahead_tokens`. Used to validate `streaming_latency_ms` and to derive the returned `num_lookahead_tokens`.

default_num_lookahead_tokens (`int`, *optional*) : The right context used when `streaming_latency_ms` is not provided. Defaults to the first entry of `supported_num_lookahead_tokens`.

prompt_dictionary (`dict[str, int]`, *optional*) : Mapping from a target-language string (e.g. `"en-US"`, `"de-DE"`, the bare code `"de"`, or `"auto"`) to its prompt index. Defaults to the NeMo checkpoint's prompt dictionary.

num_prompts (`int`, *optional*, defaults to 128) : Number of language-prompt slots (size of the one-hot prompt vector), mirroring `Nemotron3_5AsrConfig.num_prompts`.

Constructs a Nemotron3_5AsrProcessor which wraps a feature extractor and a tokenizer into a single processor.

[Nemotron3_5AsrProcessor](/docs/transformers/v5.15.1/en/model_doc/nemotron3_5_asr#transformers.Nemotron3_5AsrProcessor) offers all the functionalities of [NemotronAsrStreamingFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/nemotron_asr_streaming#transformers.models.nemotron_asr_streaming.feature_extraction_nemotron_asr_streaming._LazyModule.__getattr__..Placeholder) and [ParakeetTokenizer](/docs/transformers/v5.15.1/en/model_doc/parakeet#transformers.ParakeetTokenizer). See the
[~NemotronAsrStreamingFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/nemotron_asr_streaming#transformers.models.nemotron_asr_streaming.feature_extraction_nemotron_asr_streaming._LazyModule.__getattr__..Placeholder) and [~ParakeetTokenizer](/docs/transformers/v5.15.1/en/model_doc/parakeet#transformers.ParakeetTokenizer) for more information.

#### decode[[transformers.Nemotron3_5AsrProcessor.decode]]

```python
decode(*args, durations = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nemotron3_5_asr/processing_nemotron3_5_asr.py#L325)

Forward arguments to [decode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.decode) and post-process the token-level timestamps (if
`durations` are provided) as in the NeMo library.

#### set_num_lookahead_tokens[[transformers.Nemotron3_5AsrProcessor.set_num_lookahead_tokens]]

```python
set_num_lookahead_tokens(num_lookahead_tokens: int)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nemotron3_5_asr/processing_nemotron3_5_asr.py#L384)

Select the right attention context (lookahead, in subsampled encoder frames) used for streaming.

Sets `default_num_lookahead_tokens`, so every derived streaming property
(`num_mel_frames_first_audio_chunk`, `num_mel_frames_per_audio_chunk`, `num_samples_first_audio_chunk`,
`num_samples_per_audio_chunk`) re-derives from the new value. `num_lookahead_tokens` must be one of
`supported_num_lookahead_tokens`.

Pass the same `num_lookahead_tokens` to `model.generate` so the attention right context used in the
forward matches the chunk sizes produced here; otherwise streaming `generate` raises.

## Nemotron3_5AsrRNNTOutput[[transformers.Nemotron3_5AsrRNNTOutput]]

#### transformers.Nemotron3_5AsrRNNTOutput[[transformers.Nemotron3_5AsrRNNTOutput]]

```python
transformers.Nemotron3_5AsrRNNTOutput(last_hidden_state: typing.Optional[torch.FloatTensor] = None, pooler_output: typing.Optional[torch.FloatTensor] = None, hidden_states: tuple[torch.FloatTensor, ...] | None = None, attentions: tuple[torch.FloatTensor, ...] | None = None, loss: typing.Optional[torch.FloatTensor] = None, logits: typing.Optional[torch.FloatTensor] = None, decoder_cache: transformers.models.nemotron3_5_asr.generation_nemotron3_5_asr.Nemotron3_5AsrRNNTDecoderCache | None = None, encoder_past_key_values: transformers.cache_utils.Cache | None = None, padding_cache: NemotronAsrStreamingEncoderCausalConvPaddingCache | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nemotron3_5_asr/modeling_nemotron3_5_asr.py#L45)

encoder_past_key_values (`Cache`, *optional*):
Updated encoder attention K/V sliding-window cache, returned when encoding audio with `use_cache=True`
(cache-aware streaming). Pass it to the next chunk's forward.
padding_cache (`NemotronAsrStreamingEncoderCausalConvPaddingCache`, *optional*):
Updated unified streaming conv cache (subsampling Conv2d + conformer depthwise Conv1d), returned when
encoding audio with `use_cache=True`. Pass it to the next chunk's forward.

## Nemotron3_5AsrForRNNT[[transformers.Nemotron3_5AsrForRNNT]]

#### transformers.Nemotron3_5AsrForRNNT[[transformers.Nemotron3_5AsrForRNNT]]

```python
transformers.Nemotron3_5AsrForRNNT(config: Nemotron3_5AsrConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nemotron3_5_asr/modeling_nemotron3_5_asr.py#L205)

**Parameters:**

config ([Nemotron3_5AsrConfig](/docs/transformers/v5.15.1/en/model_doc/nemotron3_5_asr#transformers.Nemotron3_5AsrConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Nemotron3_5Asr Encoder with an RNN-T (Recurrent Neural Network Transducer) head and language-ID
prompt conditioning.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Nemotron3_5AsrForRNNT.forward]]

```python
forward(input_features: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, decoder_input_ids: typing.Optional[torch.LongTensor] = None, decoder_cache: transformers.models.nemotron3_5_asr.generation_nemotron3_5_asr.Nemotron3_5AsrRNNTDecoderCache | None = None, use_decoder_cache: bool | None = None, encoder_outputs: transformers.modeling_outputs.BaseModelOutputWithPooling | None = None, labels: typing.Optional[torch.Tensor] = None, num_lookahead_tokens: int | None = None, prompt_ids: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nemotron3_5_asr/modeling_nemotron3_5_asr.py#L256)

**Parameters:**

input_features (`torch.Tensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) : The tensors corresponding to the input audio features. Audio features can be obtained using `feature_extractor_class`. See `feature_extractor_class.__call__` for details (`processor_class` uses `feature_extractor_class` for processing audios).

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

decoder_input_ids (`torch.LongTensor` of shape `(batch_size, 1)`, *optional*) : Decoder input token ids for single-step inference.

decoder_cache (`Nemotron3_5AsrRNNTDecoderCache`, *optional*) : Decoder LSTM cache. Reused on blank predictions to skip the LSTM step.

use_decoder_cache (`bool`, *optional*) : Whether to allocate and use a decoder cache when none is provided.

encoder_outputs (`tuple(torch.FloatTensor)`, *optional*) : Pre-computed encoder outputs (last_hidden_state, pooler_output, ...).

labels (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

num_lookahead_tokens (`int`, *optional*) : Right attention context (lookahead, in subsampled encoder frames) forwarded to the encoder. Defaults to `config.encoder_config.default_num_lookahead_tokens`.

prompt_ids (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Language-prompt indices for language-ID conditioning. Produced by the processor from `language`. Turned into the broadcast one-hot consumed by `prompt_projector`.

**Returns:** [Nemotron3_5AsrRNNTOutput](/docs/transformers/v5.15.1/en/model_doc/nemotron3_5_asr#transformers.Nemotron3_5AsrRNNTOutput) or `tuple(torch.FloatTensor)`

A [Nemotron3_5AsrRNNTOutput](/docs/transformers/v5.15.1/en/model_doc/nemotron3_5_asr#transformers.Nemotron3_5AsrRNNTOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.

The [Nemotron3_5AsrForRNNT](/docs/transformers/v5.15.1/en/model_doc/nemotron3_5_asr#transformers.Nemotron3_5AsrForRNNT) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **encoder_past_key_values** (`Cache`, *optional*) -- Updated encoder attention K/V sliding-window cache, returned when encoding audio with `use_cache=True`
  (cache-aware streaming). Pass it to the next chunk's forward.
- **padding_cache** (`NemotronAsrStreamingEncoderCausalConvPaddingCache`, *optional*) -- Updated unified streaming conv cache (subsampling Conv2d + conformer depthwise Conv1d), returned when
  encoding audio with `use_cache=True`. Pass it to the next chunk's forward.

Example:

```python
>>> from transformers import AutoProcessor, Nemotron3_5AsrForRNNT
>>> from datasets import load_dataset, Audio

>>> model_id = "nvidia/nemotron-3.5-asr-streaming-0.6b"
>>> processor = AutoProcessor.from_pretrained(model_id)
>>> model = Nemotron3_5AsrForRNNT.from_pretrained(model_id)

>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))

>>> inputs = processor(ds[0]["audio"]["array"], language="en-US")
>>> outputs = model(**inputs)
```

#### generate[[transformers.Nemotron3_5AsrForRNNT.generate]]

```python
generate(inputs = None, generation_config = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nemotron3_5_asr/generation_nemotron3_5_asr.py#L25)
