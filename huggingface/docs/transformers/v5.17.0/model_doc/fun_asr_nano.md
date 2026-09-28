# Fun-ASR-Nano

## Overview

Fun-ASR-Nano is an 800M-parameter end-to-end speech recognition model developed by Alibaba DAMO Academy's FunAudioLLM team. It achieves state-of-the-art performance on Chinese, English, and Japanese ASR benchmarks while being significantly smaller than comparable models.

The model was proposed in [Fun-ASR: An Industrial-Grade Speech Recognition System](https://huggingface.co/papers/2509.12508).

### Architecture

Fun-ASR-Nano consists of three components:

1. **Audio Encoder** (SenseVoiceEncoderSmall): A 70-layer SANM (Self-Attention with FSMN Memory) encoder that combines multi-head self-attention with Feedforward Sequential Memory Networks for efficient speech feature extraction.

2. **Audio Adaptor**: A 2-layer Transformer that projects encoder outputs (512-dim) to the LLM dimension (1024-dim).

3. **Language Model** (Qwen3-0.6B): A 28-layer causal language model that generates transcription text autoregressively.

### Key Features

- **Chinese, English, and Japanese**, including 7 Chinese dialects and 26 regional accents
- **Hotword customization** for domain-specific vocabulary
- **Native punctuation** output (no separate punctuation model needed)

## Usage

### Single inference

```python
from transformers import AutoModelForSpeechSeq2Seq, AutoProcessor

model_id = "FunAudioLLM/Fun-ASR-Nano-2512-hf"
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForSpeechSeq2Seq.from_pretrained(model_id, device_map="auto")

audio_url = "https://huggingface.co/FunAudioLLM/Fun-ASR-Nano-2512/resolve/main/example/en.mp3"
inputs = processor.apply_transcription_request(audio=audio_url).to(model.device)

generated_ids = model.generate(**inputs, max_new_tokens=200)
generated_ids = generated_ids[:, inputs.input_ids.shape[1]:]
print(processor.decode(generated_ids, skip_special_tokens=True)[0])
```

### Batch inference

```python
from transformers import AutoModelForSpeechSeq2Seq, AutoProcessor

model_id = "FunAudioLLM/Fun-ASR-Nano-2512-hf"
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForSpeechSeq2Seq.from_pretrained(model_id, device_map="auto")

audio_urls = [
    "https://huggingface.co/FunAudioLLM/Fun-ASR-Nano-2512/resolve/main/example/zh.mp3",
    "https://huggingface.co/FunAudioLLM/Fun-ASR-Nano-2512/resolve/main/example/en.mp3",
]
languages = ["zh", "en"]
inputs = processor.apply_transcription_request(audio=audio_urls, language=languages).to(model.device)

generated_ids = model.generate(**inputs, max_new_tokens=200)
generated_ids = generated_ids[:, inputs.input_ids.shape[1]:]
print(processor.decode(generated_ids, skip_special_tokens=True))
```

### Custom prompts and hotwords

Pass contextual information with `prompt` and hotwords with `keywords`; the checkpoint chat template builds the full
transcription instruction. `language` accepts Chinese, English, and Japanese as full names or ISO codes.

```python
from transformers import AutoModelForSpeechSeq2Seq, AutoProcessor

model_id = "FunAudioLLM/Fun-ASR-Nano-2512-hf"
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForSpeechSeq2Seq.from_pretrained(model_id, device_map="auto")

audio_url = "https://huggingface.co/FunAudioLLM/Fun-ASR-Nano-2512/resolve/main/example/en.mp3"
inputs = processor.apply_transcription_request(
    audio=audio_url,
    prompt="A tribal story involving a chieftain and a boy.",
    keywords=["tribal chieftain", "fifty pieces of gold"],
).to(model.device)

generated_ids = model.generate(**inputs, max_new_tokens=200)
generated_ids = generated_ids[:, inputs.input_ids.shape[1]:]
print(processor.decode(generated_ids, skip_special_tokens=True)[0])
```

### Training

```python
from transformers import AutoModelForSpeechSeq2Seq, AutoProcessor

model_id = "FunAudioLLM/Fun-ASR-Nano-2512-hf"
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForSpeechSeq2Seq.from_pretrained(model_id, device_map="auto")
model.train()

audio_url = "https://huggingface.co/FunAudioLLM/Fun-ASR-Nano-2512/resolve/main/example/en.mp3"
conversation = [
    {
        "role": "user",
        "content": [
            {"type": "audio", "path": audio_url},
        ],
    },
    {
        "role": "assistant",
        "content": [
            {
                "type": "text",
                "text": "The tribal chieftain called for the boy, and presented him with fifty pieces of gold.",
            }
        ],
    },
]
inputs = processor.apply_chat_template(
    conversation,
    tokenize=True,
    return_dict=True,
    processor_kwargs={"output_labels": True},
).to(model.device)

loss = model(**inputs).loss
loss.backward()
```

### Inference with `torch.compile`

```python
import torch
from transformers import AutoModelForSpeechSeq2Seq, AutoProcessor, CompileConfig

model_id = "FunAudioLLM/Fun-ASR-Nano-2512-hf"
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForSpeechSeq2Seq.from_pretrained(model_id, device_map="auto")

audio_url = "https://huggingface.co/FunAudioLLM/Fun-ASR-Nano-2512/resolve/main/example/en.mp3"
inputs = processor.apply_transcription_request(audio=audio_url).to(model.device)

with torch.inference_mode():
    generated_ids = model.generate(
        **inputs,
        max_new_tokens=200,
        compile_config=CompileConfig(),
    )
generated_ids = generated_ids[:, inputs.input_ids.shape[1]:]
print(processor.decode(generated_ids, skip_special_tokens=True)[0])
```

## FunAsrNanoConfig[[transformers.FunAsrNanoConfig]]

#### transformers.FunAsrNanoConfig[[transformers.FunAsrNanoConfig]]

```python
transformers.FunAsrNanoConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, adaptor_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, audio_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, audio_token_id: int = 151646, initializer_range: float = 0.02, tie_word_embeddings: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/fun_asr_nano/configuration_fun_asr_nano.py#L77)

**Parameters:**

adaptor_config (`dict` or `FunAsrNanoAdaptorConfig`, *optional*) : Configuration for the bidirectional audio adaptor.

audio_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the audio backbone.

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

audio_token_id (`int`, *optional*, defaults to `151646`) : The audio token index used as a placeholder for input audio.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a FunAsrNanoModel. It is used to instantiate a Fun Asr Nano
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [FunAudioLLM/Fun-ASR-Nano-2512-hf](https://huggingface.co/FunAudioLLM/Fun-ASR-Nano-2512-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## FunAsrNanoEncoderConfig[[transformers.FunAsrNanoEncoderConfig]]

#### transformers.FunAsrNanoEncoderConfig[[transformers.FunAsrNanoEncoderConfig]]

```python
transformers.FunAsrNanoEncoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, num_mel_bins: int = 80, hidden_size: int = 512, num_attention_heads: int = 4, intermediate_size: int = 2048, num_hidden_layers: int = 70, hidden_dropout: float = 0.1, attention_dropout: float = 0.1, layer_norm_eps: float = 1e-05, hidden_act: str = 'relu', max_position_embeddings: int = 2049, num_stacked_frames: int = 7, num_timestamp_prediction_layers: int = 20, fsmn_kernel_size: int = 11)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/fun_asr_nano/configuration_fun_asr_nano.py#L25)

**Parameters:**

num_mel_bins (`int`, *optional*, defaults to `80`) : Number of mel features used per input frame. Should correspond to the value used in the `AutoFeatureExtractor` class.

hidden_size (`int`, *optional*, defaults to `512`) : Dimension of the hidden representations.

num_attention_heads (`int`, *optional*, defaults to `4`) : Number of attention heads for each attention layer in the Transformer decoder.

intermediate_size (`int`, *optional*, defaults to `2048`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to 70) : Total number of encoder layers, including both the transcription layers and the subsequent timestamp prediction layers.

hidden_dropout (`float`, *optional*, defaults to `0.1`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

attention_dropout (`float`, *optional*, defaults to `0.1`) : The dropout ratio for the attention probabilities.

layer_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

hidden_act (`str`, *optional*, defaults to `relu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `2049`) : The maximum sequence length that this model might ever be used with.

num_stacked_frames (`int`, *optional*, defaults to 7) : Number of consecutive mel frames stacked by low-frame-rate feature extraction.

num_timestamp_prediction_layers (`int`, *optional*, defaults to 20) : Number of encoder layers (final ones) dedicated to timestamp prediction. The transcription output is taken after layer `num_hidden_layers - num_timestamp_prediction_layers - 1`.

fsmn_kernel_size (`int`, *optional*, defaults to 11) : Kernel size for the feedforward sequential memory network (FSMN) convolution.

This is the configuration class to store the configuration of a FunAsrNanoModel. It is used to instantiate a Fun Asr Nano
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [FunAudioLLM/Fun-ASR-Nano-2512-hf](https://huggingface.co/FunAudioLLM/Fun-ASR-Nano-2512-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## FunAsrNanoAdaptorConfig[[transformers.FunAsrNanoAdaptorConfig]]

#### transformers.FunAsrNanoAdaptorConfig[[transformers.FunAsrNanoAdaptorConfig]]

```python
transformers.FunAsrNanoAdaptorConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 1024, num_attention_heads: int = 8, intermediate_size: int = 256, num_hidden_layers: int = 2, hidden_dropout: float = 0.0, attention_dropout: float = 0.0, layer_norm_eps: float = 1e-05, hidden_act: str = 'relu', projector_hidden_act: str = 'relu', projector_hidden_size: int = 2048)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/fun_asr_nano/configuration_fun_asr_nano.py#L62)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `1024`) : Dimension of the hidden representations.

num_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer decoder.

intermediate_size (`int`, *optional*, defaults to `256`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `2`) : Number of hidden layers in the Transformer decoder.

hidden_dropout (`float`, *optional*, defaults to `0.0`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

attention_dropout (`float`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

layer_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

hidden_act (`str`, *optional*, defaults to `relu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

projector_hidden_act (`str`, *optional*, defaults to `relu`) : The activation function used by the multimodal projector.

projector_hidden_size (`int`, *optional*, defaults to `2048`) : Dimensionality of text and vision projection layers.

This is the configuration class to store the configuration of a FunAsrNanoModel. It is used to instantiate a Fun Asr Nano
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [FunAudioLLM/Fun-ASR-Nano-2512-hf](https://huggingface.co/FunAudioLLM/Fun-ASR-Nano-2512-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## FunAsrNanoFeatureExtractor[[transformers.FunAsrNanoFeatureExtractor]]

#### transformers.FunAsrNanoFeatureExtractor[[transformers.FunAsrNanoFeatureExtractor]]

```python
transformers.FunAsrNanoFeatureExtractor(feature_size: int = 80, sampling_rate: int = 16000, frame_length: int = 25, frame_shift: int = 10, num_frames_lfr: int = 7, stride_lfr: int = 6, window: str = 'hamming', padding_value: float = 0.0, return_attention_mask: bool = True, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/fun_asr_nano/feature_extraction_fun_asr_nano.py#L33)

**Parameters:**

feature_size (`int`, *optional*, defaults to 80) : Number of mel frequency bins.

sampling_rate (`int`, *optional*, defaults to 16000) : Sampling rate of the audio.

frame_length (`int`, *optional*, defaults to 25) : Frame length in milliseconds for the STFT.

frame_shift (`int`, *optional*, defaults to 10) : Frame shift (hop length) in milliseconds for the STFT.

num_frames_lfr (`int`, *optional*, defaults to 7) : Number of consecutive frames to stack (LFR stacking factor).

stride_lfr (`int`, *optional*, defaults to 6) : Subsampling stride for LFR (take every `stride_lfr`-th stacked frame).

window (`str`, *optional*, defaults to `"hamming"`) : Window function for the STFT.

padding_value (`float`, *optional*, defaults to 0.0) : Value used for padding shorter sequences.

return_attention_mask (`bool`, *optional*, defaults to `True`) : Whether to return an attention mask.

Constructs a Fun-ASR-Nano feature extractor.

This feature extractor inherits from [SequenceFeatureExtractor](/docs/transformers/v5.17.0/en/main_classes/feature_extractor#transformers.SequenceFeatureExtractor) which contains
most of the main methods. Users should refer to this superclass for more information regarding those methods.

It extracts Kaldi-compatible mel-filterbank features (via `torchaudio.compliance.kaldi.fbank`, matching the
original FunASR front-end) and then applies Low Frame Rate (LFR) processing by stacking `num_frames_lfr`
consecutive frames and subsampling with stride `stride_lfr`.

Example:

```python
>>> import numpy as np
>>> from transformers import FunAsrNanoFeatureExtractor

>>> feature_extractor = FunAsrNanoFeatureExtractor()
>>> audio = np.random.randn(16000)  # 1 second of audio at 16kHz
>>> features = feature_extractor(audio, sampling_rate=16000, return_tensors="pt")
>>> features.input_features.shape  # (1, num_frames_after_lfr, 560)
```

#### __call__[[transformers.FunAsrNanoFeatureExtractor.__call__]]

```python
__call__(raw_speech: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor']], sampling_rate: int | None = None, return_tensors: str | transformers.utils.generic.TensorType | None = 'pt', padding: bool | str = True, max_length: int | None = None, truncation: bool = False, pad_to_multiple_of: int | None = None, return_attention_mask: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/fun_asr_nano/feature_extraction_fun_asr_nano.py#L146)

**Parameters:**

raw_speech (`np.ndarray`, `list[float]`, `list[np.ndarray]`, `list[list[float]]`) : The sequence or batch of mono waveforms to featurize.

sampling_rate (`int`, *optional*) : Sampling rate of the input audio. Must match `self.sampling_rate` (16000 Hz).

return_tensors (`str` or [TensorType](/docs/transformers/v5.17.0/en/internal/file_utils#transformers.TensorType), *optional*) : If set, returns tensors of the given framework (`"pt"`, `"np"`, ...).

padding (`bool` or `str`, *optional*, defaults to `True`) : Padding strategy forwarded to [pad()](/docs/transformers/v5.17.0/en/main_classes/feature_extractor#transformers.SequenceFeatureExtractor.pad).

max_length (`int`, *optional*) : Maximum LFR sequence length for padding/truncation.

truncation (`bool`, *optional*, defaults to `False`) : Whether to truncate sequences longer than `max_length`.

pad_to_multiple_of (`int`, *optional*) : If set, pads the LFR sequence length to a multiple of this value.

return_attention_mask (`bool`, *optional*) : Whether to return `input_features_mask`. Defaults to `self.return_attention_mask`.

**Returns:**

[BatchFeature](/docs/transformers/v5.17.0/en/main_classes/image_processor#transformers.BatchFeature) with `input_features` of shape
`(batch, max_lfr_frames, feature_size * num_frames_lfr)` and the frame-level `input_features_mask`.

Extract Kaldi mel-filterbank + LFR features from one or several raw audio waveforms.

## FunAsrNanoProcessor[[transformers.FunAsrNanoProcessor]]

#### transformers.FunAsrNanoProcessor[[transformers.FunAsrNanoProcessor]]

```python
transformers.FunAsrNanoProcessor(feature_extractor, tokenizer, chat_template = None, audio_token = '<|object_ref_start|>')
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/fun_asr_nano/processing_fun_asr_nano.py#L62)

**Parameters:**

feature_extractor (`FunAsrNanoFeatureExtractor`) : The feature extractor is a required input.

tokenizer (`Qwen2Tokenizer`) : The tokenizer is a required input.

chat_template (`str`) : A Jinja template to convert lists of messages in a chat into a tokenizable string.

audio_token (`str`, *optional*, defaults to `"<|object_ref_start|>"`) : The token used as a placeholder for audio in the text.

Constructs a FunAsrNanoProcessor which wraps a feature extractor and a tokenizer into a single processor.

[FunAsrNanoProcessor](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoProcessor) offers all the functionalities of [FunAsrNanoFeatureExtractor](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoFeatureExtractor) and [Qwen2Tokenizer](/docs/transformers/v5.17.0/en/model_doc/qwen2#transformers.Qwen2Tokenizer). See the
[~FunAsrNanoFeatureExtractor](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoFeatureExtractor) and [~Qwen2Tokenizer](/docs/transformers/v5.17.0/en/model_doc/qwen2#transformers.Qwen2Tokenizer) for more information.

#### __call__[[transformers.FunAsrNanoProcessor.__call__]]

```python
__call__(text: str | list[str], audio: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor'], NoneType] = None, output_labels: bool | None = False, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/fun_asr_nano/processing_fun_asr_nano.py#L80)

**Parameters:**

text (`Union[str, list[str]]`) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

audio (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`, *optional*) : The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor. In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels, and T is the sample length of the audio.

output_labels (`bool`, *optional*, default=False) : Whether to return labels for training.

return_tensors (`str` or [TensorType](/docs/transformers/v5.17.0/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.17.0/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) : Additional processing options for each modality (text, images, videos, audio). Model-specific parameters are listed above; see the TypedDict class for the complete list of supported arguments.

**Returns:** [BatchFeature](/docs/transformers/v5.17.0/en/main_classes/image_processor#transformers.BatchFeature)

A dictionary with tokenized text (`input_ids`, `attention_mask`) and
audio features (`input_features`, `input_features_mask`).

#### apply_transcription_request[[transformers.FunAsrNanoProcessor.apply_transcription_request]]

```python
apply_transcription_request(audio: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor'], list[typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor']]]], language: str | list[str] | None = None, prompt: str | list[str] | None = None, keywords: str | list[str] | list[list[str]] | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/fun_asr_nano/processing_fun_asr_nano.py#L144)

**Parameters:**

audio (`AudioInput` or `list[AudioInput]`) : Audio to transcribe. Can be a URL, local path, NumPy array, PyTorch tensor, or a list of these.

language (`str` or `list[str]`, *optional*) : Target language. Accepts Chinese, English, or Japanese as full English names, ISO codes (`"zh"`, `"en"`, `"ja"`), or the checkpoint's Chinese language names (`"中文"`, `"英文"`, `"日文"`). A single value is broadcast across the batch.

prompt (`str` or `list[str]`, *optional*) : Contextual information that may improve transcription. A list must match the audio batch size.

keywords (`str`, `list[str]`, or `list[list[str]]`, *optional*) : Hotwords to bias recognition. A string or flat list is shared across the batch; a nested list supplies separate hotwords for each audio sample.

- ****kwargs** : Additional keyword arguments forwarded to [apply_chat_template()](/docs/transformers/v5.17.0/en/main_classes/processors#transformers.ProcessorMixin.apply_chat_template) and the underlying processor call (for example `text_kwargs`, `audio_kwargs`, ...).

**Returns:** [BatchFeature](/docs/transformers/v5.17.0/en/main_classes/image_processor#transformers.BatchFeature)

Processor outputs ready to be passed to
[FunAsrNanoForConditionalGeneration.generate()](/docs/transformers/v5.17.0/en/main_classes/text_generation#transformers.GenerationMixin.generate).

Prepare inputs for ASR using the checkpoint's structured transcription chat template.

## FunAsrNanoEncoder[[transformers.FunAsrNanoEncoder]]

#### transformers.FunAsrNanoEncoder[[transformers.FunAsrNanoEncoder]]

```python
transformers.FunAsrNanoEncoder(config: FunAsrNanoEncoderConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/fun_asr_nano/modeling_fun_asr_nano.py#L279)

**Parameters:**

config ([FunAsrNanoEncoderConfig](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoEncoderConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Fun-ASR-Nano audio encoder (SenseVoice SAN-M architecture), without any head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.FunAsrNanoEncoder.forward]]

```python
forward(input_features: Tensor, input_features_mask: Tensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/fun_asr_nano/modeling_fun_asr_nano.py#L313)

**Parameters:**

input_features (`torch.Tensor` of shape `(batch_size, sequence_length, feature_dim)`) : The tensors corresponding to the input audio features. Audio features can be obtained using [FunAsrNanoFeatureExtractor](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoFeatureExtractor). See [FunAsrNanoFeatureExtractor.__call__()](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoFeatureExtractor.__call__) for details ([FunAsrNanoProcessor](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoProcessor) uses [FunAsrNanoFeatureExtractor](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoFeatureExtractor) for processing audios).

input_features_mask (`torch.LongTensor` of shape `(batch_size, padded_feature_length)`) : 1 for valid mel frames and 0 for padding.

**Returns:** [BaseModelOutput](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or `tuple(torch.FloatTensor)`

A [BaseModelOutput](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([FunAsrNanoConfig](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoConfig)) and inputs.

The [FunAsrNanoEncoder](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoEncoder) forward method, overrides the `__call__` special method.

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

## FunAsrNanoModel[[transformers.FunAsrNanoModel]]

#### transformers.FunAsrNanoModel[[transformers.FunAsrNanoModel]]

```python
transformers.FunAsrNanoModel(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/fun_asr_nano/modeling_fun_asr_nano.py#L393)

**Parameters:**

config ([FunAsrNanoModel](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoModel)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Fun-ASR-Nano model (SenseVoice SAN-M audio encoder, a Transformer adaptor and a Qwen3 language model),
without a language modeling head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.FunAsrNanoModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, input_features: typing.Optional[torch.FloatTensor] = None, input_features_mask: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/fun_asr_nano/modeling_fun_asr_nano.py#L468)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.17.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) : The tensors corresponding to the input audio features. Audio features can be obtained using [FunAsrNanoFeatureExtractor](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoFeatureExtractor). See [FunAsrNanoFeatureExtractor.__call__()](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoFeatureExtractor.__call__) for details ([FunAsrNanoProcessor](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoProcessor) uses [FunAsrNanoFeatureExtractor](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoFeatureExtractor) for processing audios).

input_features_mask (`torch.Tensor` of shape `(batch_size, feature_sequence_length)`) : Mask to avoid performing attention on padding feature indices.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** `FunAsrNanoModelOutputWithPast` or `tuple(torch.FloatTensor)`

A `FunAsrNanoModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([FunAsrNanoConfig](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoConfig)) and inputs.

The [FunAsrNanoModel](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **audio_hidden_states** (`torch.FloatTensor`, *optional*) -- Projected audio hidden states.

## FunAsrNanoForConditionalGeneration[[transformers.FunAsrNanoForConditionalGeneration]]

#### transformers.FunAsrNanoForConditionalGeneration[[transformers.FunAsrNanoForConditionalGeneration]]

```python
transformers.FunAsrNanoForConditionalGeneration(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/fun_asr_nano/modeling_fun_asr_nano.py#L551)

**Parameters:**

config ([FunAsrNanoForConditionalGeneration](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoForConditionalGeneration)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Fun-ASR-Nano model for speech recognition: a SenseVoice SAN-M audio encoder, a Transformer adaptor and a
Qwen3 language model with a language modeling head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.FunAsrNanoForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, input_features: typing.Optional[torch.FloatTensor] = None, input_features_mask: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/fun_asr_nano/modeling_fun_asr_nano.py#L563)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.17.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) : The tensors corresponding to the input audio features. Audio features can be obtained using [FunAsrNanoFeatureExtractor](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoFeatureExtractor). See [FunAsrNanoFeatureExtractor.__call__()](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoFeatureExtractor.__call__) for details ([FunAsrNanoProcessor](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoProcessor) uses [FunAsrNanoFeatureExtractor](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoFeatureExtractor) for processing audios).

input_features_mask (`torch.Tensor` of shape `(batch_size, feature_sequence_length)`) : Mask to avoid performing attention on padding feature indices.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** `FunAsrNanoCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`

A `FunAsrNanoCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([FunAsrNanoConfig](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoConfig)) and inputs.

The [FunAsrNanoForConditionalGeneration](/docs/transformers/v5.17.0/en/model_doc/fun_asr_nano#transformers.FunAsrNanoForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.Cache) instance.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **audio_hidden_states** (`torch.FloatTensor`, *optional*) -- Hidden states of the audio encoder after projection.

Example:

```python
>>> from transformers import FunAsrNanoForConditionalGeneration, AutoProcessor

>>> model_id = "FunAudioLLM/Fun-ASR-Nano-2512-hf"
>>> processor = AutoProcessor.from_pretrained(model_id)
>>> model = FunAsrNanoForConditionalGeneration.from_pretrained(model_id, device_map="auto")
```
