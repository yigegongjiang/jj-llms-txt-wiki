# Nemotron ASR Streaming

## Overview

Nemotron ASR Streaming is a 600M-parameter English speech recognition model from NVIDIA, built for high-quality transcription in both low-latency streaming and high-throughput batch settings, with native punctuation and capitalization. For streaming, it offers configurable chunk sizes—80ms, 160ms, 560ms, and 1120ms, letting users trade off latency against accuracy to suit their application. Its cache-aware FastConformer-RNNT architecture is central to this capability: unlike traditional buffered streaming, which repeatedly reprocesses overlapping audio windows, the model processes only each new incoming chunk while reusing cached encoder context from prior chunks. This eliminates redundant computation, significantly improves efficiency, and minimizes end-to-end delay without sacrificing accuracy, making it well suited to real-time transcription workloads.

## Usage

### Offline transcription

```python
from transformers import pipeline

pipe = pipeline(
    "automatic-speech-recognition",
    model="nvidia/nemotron-speech-streaming-en-0.6b",
)
out = pipe("https://huggingface.co/datasets/hf-internal-testing/dummy-audio-samples/resolve/main/bcn_weather.mp3")
print(out)
```

```python
from transformers import AutoModelForRNNT, AutoProcessor
from transformers.audio_utils import load_audio

model_id = "nvidia/nemotron-speech-streaming-en-0.6b"
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForRNNT.from_pretrained(model_id, device_map="auto")

audio = load_audio(
    "https://huggingface.co/datasets/hf-internal-testing/dummy-audio-samples/resolve/main/bcn_weather.mp3",
    sampling_rate=processor.feature_extractor.sampling_rate,
)

inputs = processor(audio, sampling_rate=processor.feature_extractor.sampling_rate)
inputs.to(model.device, dtype=model.dtype)
output = model.generate(**inputs, return_dict_in_generate=True)
print(processor.decode(output.sequences, skip_special_tokens=True))
```

### Streaming transcription
> [!NOTE]
> This is an experimental feature and the API is subject to change.

For real-time transcription, audio is split into chunks following:

```python
from threading import Thread
from transformers import AutoModelForRNNT, AutoProcessor, TextIteratorStreamer
from transformers.audio_utils import load_audio

model_id = "nvidia/nemotron-speech-streaming-en-0.6b"
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForRNNT.from_pretrained(model_id, device_map="auto")

processor.set_num_lookahead_tokens(6)
print(f"Streaming latency: {processor.streaming_latency_ms} ms")

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

processor = AutoProcessor.from_pretrained("nvidia/nemotron-speech-streaming-en-0.6b")

# Each supported `num_lookahead_tokens` mapped to its streaming latency in milliseconds:
print(processor.supported_streaming_latencies_ms)
# {13: 1120, 6: 560, 1: 160, 0: 80}

# Select a right attention context (this also re-derives the streaming chunk sizes used above):
processor.set_num_lookahead_tokens(6)

# Latency of the current selection:
print(processor.streaming_latency_ms)
# 560
```

`set_num_lookahead_tokens` sizes the chunks the processor emits, and the matching `num_lookahead_tokens` must reach `generate` (in the snippet above it travels through `**inputs`/`**first_chunk_inputs`, which carries `num_lookahead_tokens`). Streaming `generate` raises if it is omitted.

## NemotronAsrStreamingConfig[[transformers.NemotronAsrStreamingConfig]]

- **is_encoder_decoder** (`bool`, *optional*, defaults to `True`) --
  Whether the model is used as an encoder/decoder or not.
- **vocab_size** (`int`, *optional*, defaults to `1025`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **decoder_hidden_size** (`int`, *optional*, defaults to 640) --
  Hidden size of the LSTM prediction network (NeMo's `pred_hidden`). The joint network projects both
  encoder and decoder outputs to this size (NeMo's `joint_hidden`, which all known checkpoints set equal
  to `pred_hidden`).
- **num_decoder_layers** (`int`, *optional*, defaults to 2) --
  Number of LSTM layers in the prediction network.
- **hidden_act** (`str`, *optional*, defaults to `"relu"`) --
  Activation in the joint network.
- **max_symbols_per_step** (`int`, *optional*, defaults to 10) --
  Maximum number of non-blank symbols emitted per encoder time step during greedy decoding.
- **encoder_config** (`Union[dict, NemotronAsrStreamingEncoderConfig]`, *optional*) --
  The config object or dictionary of the encoder.
- **pad_token_id** (`int`, *optional*, defaults to `0`) --
  Token id used for padding in the vocabulary.
- **blank_token_id** (`int`, *optional*, defaults to 1024) --
  Blank token id. Different from `pad_token_id` for RNN-T.

This is the configuration class to store the configuration of a NemotronAsrStreamingForRNNT. It is used to instantiate a Nemotron Asr Streaming
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [nvidia/nemotron-speech-streaming-en-0.6b](https://huggingface.co/nvidia/nemotron-speech-streaming-en-0.6b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import NemotronAsrStreamingForRNNT, NemotronAsrStreamingConfig

>>> # Initializing a NemotronAsrStreaming RNN-T configuration
>>> configuration = NemotronAsrStreamingConfig()

>>> # Initializing a model from the configuration
>>> model = NemotronAsrStreamingForRNNT(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## NemotronAsrStreamingEncoderConfig[[transformers.NemotronAsrStreamingEncoderConfig]]

- **hidden_size** (`int`, *optional*, defaults to `1024`) --
  Dimension of the hidden representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `24`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **intermediate_size** (`int`, *optional*, defaults to `4096`) --
  Dimension of the MLP representations.
- **hidden_act** (`str`, *optional*, defaults to `silu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **attention_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to use a bias in the query, key, value and output projection layers during self-attention.
- **convolution_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to use bias in convolutions of the conformer's convolution module.
- **conv_kernel_size** (`int`, *optional*, defaults to 9) --
  The kernel size of the convolution layers in the Conformer block.
- **subsampling_factor** (`int`, *optional*, defaults to 8) --
  The factor by which the input sequence is subsampled.
- **subsampling_conv_channels** (`int`, *optional*, defaults to 256) --
  The number of channels in the subsampling convolution layers.
- **num_mel_bins** (`int`, *optional*, defaults to 80) --
  Number of mel features.
- **subsampling_conv_kernel_size** (`int`, *optional*, defaults to 3) --
  The kernel size of the subsampling convolution layers.
- **subsampling_conv_stride** (`int`, *optional*, defaults to 2) --
  The stride of the subsampling convolution layers.
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The ratio for all dropout layers.
- **dropout_positions** (`float`, *optional*, defaults to 0.0) --
  The dropout ratio for the positions in the input sequence.
- **layerdrop** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The LayerDrop probability. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556) for
  more details.
- **activation_dropout** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout ratio for activations inside the fully connected layer.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout ratio for the attention probabilities.
- **max_position_embeddings** (`int`, *optional*, defaults to `5000`) --
  The maximum sequence length that this model might ever be used with.
- **scale_input** (`bool`, *optional*, defaults to `True`) --
  Whether to scale the input embeddings.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **sliding_window** (`int`, *optional*, defaults to 71) --
  Size of the K/V attention sliding window (in subsampled encoder frames). It equals
  `left_context + 1` (the current frame plus the left context), so the left attention context is
  `sliding_window - 1` — the same across all supported lookaheads.
- **default_num_lookahead_tokens** (`int`, *optional*, defaults to 13) --
  The right attention context (lookahead, in subsampled encoder frames) used when none is passed to the
  forward. The supported set the model was trained with lives on [NemotronAsrStreamingProcessor](/docs/transformers/v5.14.0/en/model_doc/nemotron_asr_streaming#transformers.NemotronAsrStreamingProcessor).

This is the configuration class to store the configuration of a NemotronAsrStreamingForRNNT. It is used to instantiate a Nemotron Asr Streaming
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [nvidia/nemotron-speech-streaming-en-0.6b](https://huggingface.co/nvidia/nemotron-speech-streaming-en-0.6b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import NemotronAsrStreamingEncoder, NemotronAsrStreamingEncoderConfig

>>> # Initializing a `NemotronAsrStreamingEncoder` configuration
>>> configuration = NemotronAsrStreamingEncoderConfig()

>>> # Initializing a model from the configuration
>>> model = NemotronAsrStreamingEncoder(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## NemotronAsrStreamingFeatureExtractor[[transformers.models.nemotron_asr_streaming.feature_extraction_nemotron_asr_streaming._LazyModule.__getattr__..Placeholder]]

.Placeholder"} anchor={"transformers.models.nemotron_asr_streaming.feature_extraction_nemotron_asr_streaming._LazyModule.__getattr__..Placeholder"} parameters={[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]}>

## NemotronAsrStreamingProcessor[[transformers.NemotronAsrStreamingProcessor]]

'"}, {"name": "supported_num_lookahead_tokens", "val": " = None"}, {"name": "default_num_lookahead_tokens", "val": " = None"}]}>
- **feature_extractor** (`NemotronAsrStreamingFeatureExtractor`) --
  The feature extractor is a required input.
- **tokenizer** (`ParakeetTokenizer`) --
  The tokenizer is a required input.
- **blank_token** (`str`, *optional*, defaults to `"<blank>"`) --
  Blank token for RNN-T decoding.
- **supported_num_lookahead_tokens** (`list[int]`, *optional*) --
  Right attention contexts (lookaheads, in subsampled encoder frames) the model was trained with.
  The processor is the single source of truth for this set: `set_num_lookahead_tokens()`
  validates against it. Defaults to the NeMo cache-aware set `[13, 6, 1, 0]`.
- **default_num_lookahead_tokens** (`int`, *optional*) --
  The right context used to size streaming chunks and emitted by [__call__()](/docs/transformers/v5.14.0/en/model_doc/nemotron_asr_streaming#transformers.NemotronAsrStreamingProcessor.__call__);
  change it with `set_num_lookahead_tokens()`. Defaults to the first entry of
  `supported_num_lookahead_tokens`.
Constructs a NemotronAsrStreamingProcessor which wraps a feature extractor and a tokenizer into a single processor.

[NemotronAsrStreamingProcessor](/docs/transformers/v5.14.0/en/model_doc/nemotron_asr_streaming#transformers.NemotronAsrStreamingProcessor) offers all the functionalities of [NemotronAsrStreamingFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/nemotron_asr_streaming#transformers.models.nemotron_asr_streaming.feature_extraction_nemotron_asr_streaming._LazyModule.__getattr__..Placeholder) and [ParakeetTokenizer](/docs/transformers/v5.14.0/en/model_doc/parakeet#transformers.ParakeetTokenizer). See the
[~NemotronAsrStreamingFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/nemotron_asr_streaming#transformers.models.nemotron_asr_streaming.feature_extraction_nemotron_asr_streaming._LazyModule.__getattr__..Placeholder) and [~ParakeetTokenizer](/docs/transformers/v5.14.0/en/model_doc/parakeet#transformers.ParakeetTokenizer) for more information.

- **audio** (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`) --
  The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor.
  In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels,
  and T is the sample length of the audio.
- **text** (`Union[str, list[str], list[list[str]]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **sampling_rate** (`int`, *optional*) --
  The sampling rate of the input audio in Hz. This should match the sampling rate expected by the feature
  extractor (defaults to 16000 Hz). If provided, it will be validated against the processor's expected
  sampling rate, and an error will be raised if they don't match. If not provided, a warning will be
  issued and the default sampling rate will be assumed.
- **is_streaming** (`bool`, *optional*, defaults to `False`) --
  Whether to process audio in streaming mode. When `True`, audio can be passed in chunks, using
  `is_first_audio_chunk` to distinguish the first chunk from subsequent ones.
- **is_first_audio_chunk** (`bool`, *optional*, defaults to `True`) --
  Whether the current audio is the first chunk of a streaming session. The feature extractor uses
  `center=True` for the first chunk (and for offline use) and `center=False` for subsequent chunks,
  so that the per-chunk STFT reproduces, frame-for-frame, a single full-utterance pass. Must be
  `True` when `is_streaming=False`.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.
- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) --
  Additional processing options for each modality (text, images, videos, audio). Model-specific parameters
  are listed above; see the TypedDict class for the complete list of supported arguments.[BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature)the feature-extractor (and optional tokenizer) outputs, augmented with:

- **num_lookahead_tokens** -- The right attention context (lookahead, in subsampled encoder frames),
  i.e. `default_num_lookahead_tokens` (set via `set_num_lookahead_tokens()`).
  Pass it to the model/encoder forward (or `generate`); it plays the role of Voxtral Realtime's
  `num_delay_tokens`.

Forward arguments to [decode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.decode) and post-process the token-level timestamps (if
`durations` are provided) as in the NeMo library.

## NemotronAsrStreamingEncoderModelOutput[[transformers.NemotronAsrStreamingEncoderModelOutput]]

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) --
  Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) --
  Last layer hidden-state of the first token of the sequence (classification token) after further processing
  through the layers used for the auxiliary pretraining task. E.g. for BERT-family of models, this returns
  the classification token after processing through a linear layer and a tanh activation function. The linear
  layer weights are trained from the next sentence prediction (classification) objective during pretraining.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) --
  Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) --
  Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices after sequence compression. Returned because the
  sequence length may differ from the input sequence length. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
- **past_key_values** (`Cache`, *optional*) --
  Updated attention K/V sliding-window cache from the encoder. Pass to the next chunk call.
- **padding_cache** (`NemotronAsrStreamingEncoderCausalConvPaddingCache`, *optional*) --
  Unified streaming cache backing the subsampling Conv2d layers and the conformer depthwise Conv1d.

Extends `ParakeetEncoderModelOutput` with optional streaming caches. Caches are only populated for
cache-aware models when `use_cache=True`.

## NemotronAsrStreamingRNNTOutput[[transformers.NemotronAsrStreamingRNNTOutput]]

encoder_past_key_values (`Cache`, *optional*):
Updated encoder attention K/V sliding-window cache, returned when encoding audio with `use_cache=True`
(cache-aware streaming). Pass it to the next chunk's forward.
padding_cache (`NemotronAsrStreamingEncoderCausalConvPaddingCache`, *optional*):
Updated unified streaming conv cache (subsampling Conv2d + conformer depthwise Conv1d), returned when
encoding audio with `use_cache=True`. Pass it to the next chunk's forward.

## NemotronAsrStreamingEncoder[[transformers.NemotronAsrStreamingEncoder]]

- **config** ([NemotronAsrStreamingEncoderConfig](/docs/transformers/v5.14.0/en/model_doc/nemotron_asr_streaming#transformers.NemotronAsrStreamingEncoderConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The NemotronAsrStreaming Encoder model, based on the [Fast Conformer architecture](https://huggingface.co/papers/2305.05084).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "position_ids", "val": ": typing.Optional[torch.LongTensor] = None"}, {"name": "past_key_values", "val": ": transformers.cache_utils.Cache | None = None"}, {"name": "output_attention_mask", "val": ": bool = True"}, {"name": "use_cache", "val": ": bool | None = None"}, {"name": "padding_cache", "val": ": transformers.models.nemotron_asr_streaming.modeling_nemotron_asr_streaming.NemotronAsrStreamingEncoderCausalConvPaddingCache | None = None"}, {"name": "num_lookahead_tokens", "val": ": int | None = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **input_features** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, sequence_length, feature_dim)`) --
  The tensors corresponding to the input audio features. Audio features can be obtained using
  `feature_extractor_class`. See `feature_extractor_class.__call__` for details (`processor_class` uses
  `feature_extractor_class` for processing audios).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **past_key_values** (`Cache`, *optional*) --
  Sliding-window K/V cache (`DynamicCache` built from `config.sliding_window`) for cache-aware
  streaming attention.
- **output_attention_mask** (`bool`, *optional*, defaults to `True`) --
  Whether to return the output attention mask. Only effective when `attention_mask` is provided.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **padding_cache** (`NemotronAsrStreamingEncoderCausalConvPaddingCache`, *optional*) --
  Unified streaming cache backing the subsampling Conv2d layers and the conformer depthwise Conv1d.
- **num_lookahead_tokens** (`int`, *optional*) --
  Override of the right attention context (lookahead, in subsampled encoder frames) for this
  forward pass. Combined with the left context `config.sliding_window - 1`. Defaults to
  `config.default_num_lookahead_tokens`.[BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or `tuple(torch.FloatTensor)`A [BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.
The [NemotronAsrStreamingEncoder](/docs/transformers/v5.14.0/en/model_doc/nemotron_asr_streaming#transformers.NemotronAsrStreamingEncoder) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoProcessor, NemotronAsrStreamingEncoder
>>> from datasets import load_dataset, Audio

>>> model_id = "nvidia/nemotron-speech-streaming-en-0.6b"
>>> processor = AutoProcessor.from_pretrained(model_id)
>>> encoder = NemotronAsrStreamingEncoder.from_pretrained(model_id)

>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))

>>> inputs = processor(ds[0]["audio"]["array"])
>>> encoder_outputs = encoder(**inputs)

>>> print(encoder_outputs.last_hidden_state.shape)
```

## NemotronAsrStreamingForRNNT[[transformers.NemotronAsrStreamingForRNNT]]

- **config** ([NemotronAsrStreamingConfig](/docs/transformers/v5.14.0/en/model_doc/nemotron_asr_streaming#transformers.NemotronAsrStreamingConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

NemotronAsrStreaming Encoder with an RNN-T (Recurrent Neural Network Transducer) head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_features** (`torch.Tensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) --
  The tensors corresponding to the input audio features. Audio features can be obtained using
  `feature_extractor_class`. See `feature_extractor_class.__call__` for details (`processor_class` uses
  `feature_extractor_class` for processing audios).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **decoder_input_ids** (`torch.LongTensor` of shape `(batch_size, 1)`, *optional*) --
  Decoder input token ids for single-step inference.
- **decoder_cache** (`NemotronAsrStreamingRNNTDecoderCache`, *optional*) --
  Decoder LSTM cache. Reused on blank predictions to skip the LSTM step.
- **use_decoder_cache** (`bool`, *optional*) --
  Whether to allocate and use a decoder cache when none is provided.
- **encoder_outputs** (`NemotronAsrStreamingEncoderModelOutput`, *optional*) --
  Pre-computed encoder outputs (last_hidden_state, pooler_output, ...).
- **labels** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **num_lookahead_tokens** (`int`, *optional*) --
  Right attention context (lookahead, in subsampled encoder frames) forwarded to the encoder.
  Defaults to `config.encoder_config.default_num_lookahead_tokens`.[NemotronAsrStreamingRNNTOutput](/docs/transformers/v5.14.0/en/model_doc/nemotron_asr_streaming#transformers.NemotronAsrStreamingRNNTOutput) or `tuple(torch.FloatTensor)`A [NemotronAsrStreamingRNNTOutput](/docs/transformers/v5.14.0/en/model_doc/nemotron_asr_streaming#transformers.NemotronAsrStreamingRNNTOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.
The [NemotronAsrStreamingForRNNT](/docs/transformers/v5.14.0/en/model_doc/nemotron_asr_streaming#transformers.NemotronAsrStreamingForRNNT) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **encoder_past_key_values** (`Cache`, *optional*) -- Updated encoder attention K/V sliding-window cache, returned when encoding audio with `use_cache=True`
  (cache-aware streaming). Pass it to the next chunk's forward.
- **padding_cache** (`NemotronAsrStreamingEncoderCausalConvPaddingCache`, *optional*) -- Updated unified streaming conv cache (subsampling Conv2d + conformer depthwise Conv1d), returned when
  encoding audio with `use_cache=True`. Pass it to the next chunk's forward.

Example:

```python
>>> from transformers import AutoProcessor, NemotronAsrStreamingForRNNT
>>> from datasets import load_dataset, Audio

>>> model_id = "nvidia/nemotron-speech-streaming-en-0.6b"
>>> processor = AutoProcessor.from_pretrained(model_id)
>>> model = NemotronAsrStreamingForRNNT.from_pretrained(model_id)

>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))

>>> inputs = processor(ds[0]["audio"]["array"])
>>> outputs = model(**inputs)
```
