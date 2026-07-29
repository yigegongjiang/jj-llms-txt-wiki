# VibeVoice ASR

## Overview

VibeVoice ASR is an automatic speech recognition model from Microsoft that combines acoustic and semantic audio tokenizers with a causal language model for robust speech-to-text transcription. The model uses VibeVoice's acoustic and semantic tokenizers that process audio at 24kHz, paired with a Qwen2-based language decoder for generating transcriptions. See the [technical report](https://huggingface.co/papers/2601.18184) for more details.

The model checkpoint is available at: [microsoft/VibeVoice-ASR-HF](https://huggingface.co/microsoft/VibeVoice-ASR-HF)

Highlights:

- **🕒 60-minute Single-Pass Processing**:
  Unlike conventional ASR models that slice audio into short chunks (often losing global context), VibeVoice ASR accepts up to **60 minutes** of continuous audio input within 64K token length. This ensures consistent speaker tracking and semantic coherence across the entire hour.

- **👤 Customized Hotwords**:
  Users can provide customized hotwords (e.g., specific names, technical terms, or background info) to guide the recognition process, significantly improving accuracy on domain-specific content.

- **📝 Rich Transcription (Who, When, What)**:
  The model jointly performs ASR, diarization, and timestamping, producing a structured output that indicates *who* said *what* and *when*.
  
- **🌍 Multilingual & Code-Switching Support**:
  It supports over 50 languages, requires no explicit language setting, and natively handles code-switching within and across utterances. Language distribution can be found [here](#language-distribution).

This model was contributed by [Eric Bezzam](https://huggingface.co/bezzam).

## Usage

The model supports various automatic speech recognition functionalities.

### Speaker-timestamped transcription

A notable feature of VibeVoice ASR is its ability to transcribe multi-speaker content, denoting who spoke and when.

```python
from transformers import AutoProcessor, VibeVoiceAsrForConditionalGeneration

model_id = "microsoft/VibeVoice-ASR-HF"
processor = AutoProcessor.from_pretrained(model_id)
model = VibeVoiceAsrForConditionalGeneration.from_pretrained(model_id, device_map="auto")
print(f"Model loaded on {model.device} with dtype {model.dtype}")

# Prepare inputs using `apply_transcription_request`
inputs = processor.apply_transcription_request(
    audio="https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/example_output/VibeVoice-1.5B_output.wav",
).to(model.device, model.dtype)

# Apply model
output_ids = model.generate(**inputs)
generated_ids = output_ids[:, inputs["input_ids"].shape[1] :]
transcription = processor.decode(generated_ids)[0]
print("\n" + "=" * 60)
print("RAW OUTPUT")
print("=" * 60)
print(transcription)

transcription = processor.decode(generated_ids, return_format="parsed")[0]
print("\n" + "=" * 60)
print("TRANSCRIPTION (list of dicts)")
print("=" * 60)
for speaker_transcription in transcription:
    print(speaker_transcription)

# Remove speaker labels, only get raw transcription
transcription = processor.decode(generated_ids, return_format="transcription_only")[0]
print("\n" + "=" * 60)
print("TRANSCRIPTION ONLY")
print("=" * 60)
print(transcription)

"""
============================================================
RAW OUTPUT
============================================================
<|im_start|>assistant
[{"Start":0,"End":15.43,"Speaker":0,"Content":"Hello everyone and welcome to the Vibe Voice podcast. I'm your host, Alex, and today we're getting into one of the biggest debates in all of sports: who's the greatest basketball player of all time? I'm so excited to have Sam here to talk about it with me."},{"Start":15.43,"End":21.05,"Speaker":1,"Content":"Thanks so much for having me, Alex. And you're absolutely right. This question always brings out some seriously strong feelings."},{"Start":21.05,"End":31.66,"Speaker":0,"Content":"Okay, so let's get right into it. For me, it has to be Michael Jordan. Six trips to the finals, six championships. That kind of perfection is just incredible."},{"Start":31.66,"End":40.93,"Speaker":1,"Content":"Oh man, the first thing that always pops into my head is that shot against the Cleveland Cavaliers back in '89. Jordan just rises, hangs in the air forever, and just sinks it."}]<|im_end|>
<|endoftext|>

============================================================
TRANSCRIPTION (list of dicts)
============================================================
{'Start': 0, 'End': 15.43, 'Speaker': 0, 'Content': "Hello everyone and welcome to the Vibe Voice podcast. I'm your host, Alex, and today we're getting into one of the biggest debates in all of sports: who's the greatest basketball player of all time? I'm so excited to have Sam here to talk about it with me."}
{'Start': 15.43, 'End': 21.05, 'Speaker': 1, 'Content': "Thanks so much for having me, Alex. And you're absolutely right. This question always brings out some seriously strong feelings."}
{'Start': 21.05, 'End': 31.66, 'Speaker': 0, 'Content': "Okay, so let's get right into it. For me, it has to be Michael Jordan. Six trips to the finals, six championships. That kind of perfection is just incredible."}
{'Start': 31.66, 'End': 40.93, 'Speaker': 1, 'Content': "Oh man, the first thing that always pops into my head is that shot against the Cleveland Cavaliers back in '89. Jordan just rises, hangs in the air forever, and just sinks it."}

============================================================
TRANSCRIPTION ONLY
============================================================
Hello everyone and welcome to the Vibe Voice podcast. I'm your host, Alex, and today we're getting into one of the biggest debates in all of sports: who's the greatest basketball player of all time? I'm so excited to have Sam here to talk about it with me. Thanks so much for having me, Alex. And you're absolutely right. This question always brings out some seriously strong feelings. Okay, so let's get right into it. For me, it has to be Michael Jordan. Six trips to the finals, six championships. That kind of perfection is just incredible. Oh man, the first thing that always pops into my head is that shot against the Cleveland Cavaliers back in '89. Jordan just rises, hangs in the air forever, and just sinks it.
"""
```

The VibeVoice ASR model is trained to generate a string that resembles a JSON structure. The flag `return_format="parsed"` tries to return the generated output as a list of dicts, while `return_format="transcription_only"` tries to extract only the transcribed audio. If they fail, the generated output is returned as-is.

### Providing context

It is also possible to provide context. This can be useful if certain words cannot be transcribed correctly, such as proper nouns.

Below we transcribe an audio where the speaker (with a German accent) talks about VibeVoice, comparing with and without the context "About VibeVoice".

```python
from transformers import AutoProcessor, VibeVoiceAsrForConditionalGeneration

model_id = "microsoft/VibeVoice-ASR-HF"
processor = AutoProcessor.from_pretrained(model_id)
model = VibeVoiceAsrForConditionalGeneration.from_pretrained(model_id, device_map="auto")
print(f"Model loaded on {model.device} with dtype {model.dtype}")

# Without context
inputs = processor.apply_transcription_request(
    audio="https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/realtime_model/vibevoice_tts_german.wav",
).to(model.device, model.dtype)
output_ids = model.generate(**inputs)
generated_ids = output_ids[:, inputs["input_ids"].shape[1] :]
transcription = processor.decode(generated_ids, return_format="transcription_only")[0]
print(f"WITHOUT CONTEXT: {transcription}")

# With context
inputs = processor.apply_transcription_request(
    audio="https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/realtime_model/vibevoice_tts_german.wav",
    prompt="About VibeVoice",
).to(model.device, model.dtype)
output_ids = model.generate(**inputs)
generated_ids = output_ids[:, inputs["input_ids"].shape[1] :]
transcription = processor.decode(generated_ids, return_format="transcription_only")[0]
print(f"WITH CONTEXT   : {transcription}")

"""
WITHOUT CONTEXT: Revevoices is a novel framework designed for generating expressive, long-form, multi-speaker conversational audio.
WITH CONTEXT   : VibeVoice is this novel framework designed for generating expressive, long-form, multi-speaker, conversational audio.
"""
```

### Batch inference

Batch inference is possible by passing a list of audio and, if provided, a list of prompts. The number of audio inputs and prompts should match (for prompts, you can set an entry to `None` if not needed for a given audio).

```python
from transformers import AutoProcessor, VibeVoiceAsrForConditionalGeneration

model_id = "microsoft/VibeVoice-ASR-HF"
audio = [
    "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/realtime_model/vibevoice_tts_german.wav",
    "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/example_output/VibeVoice-1.5B_output.wav"
]
prompts = ["About VibeVoice", None]

processor = AutoProcessor.from_pretrained(model_id)
model = VibeVoiceAsrForConditionalGeneration.from_pretrained(model_id, device_map="auto")
print(f"Model loaded on {model.device} with dtype {model.dtype}")

inputs = processor.apply_transcription_request(audio, prompt=prompts).to(model.device, model.dtype)
output_ids = model.generate(**inputs)
generated_ids = output_ids[:, inputs["input_ids"].shape[1] :]
transcription = processor.decode(generated_ids, return_format="transcription_only")

print(transcription)
```

### Adjusting tokenizer chunk (e.g. if out-of-memory)

A key feature of VibeVoice ASR is that it can transcribe up to 60 minutes of continuous audio. This is done by chunking audio into 60-second segments (1440000 samples at 24kHz) and caching the convolution states between each segment.

However, if chunks of 60 seconds are too large for your device, the `acoustic_tokenizer_chunk_size` argument passed to `generate` can be adjusted. *Note it should be a multiple of the hop length (3200 for the original acoustic tokenizer).*

```python
from transformers import AutoProcessor, VibeVoiceAsrForConditionalGeneration

acoustic_tokenizer_chunk_size = 64000    # default is 1440000 (60s @ 24kHz)
model_id = "microsoft/VibeVoice-ASR-HF"
audio = [
    "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/realtime_model/vibevoice_tts_german.wav",
    "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/example_output/VibeVoice-1.5B_output.wav"
]
prompts = ["About VibeVoice", None]

processor = AutoProcessor.from_pretrained(model_id)
model = VibeVoiceAsrForConditionalGeneration.from_pretrained(model_id, device_map="auto")
print(f"Model loaded on {model.device} with dtype {model.dtype}")

inputs = processor.apply_transcription_request(audio, prompt=prompts).to(model.device, model.dtype)
output_ids = model.generate(**inputs, acoustic_tokenizer_chunk_size=acoustic_tokenizer_chunk_size)
generated_ids = output_ids[:, inputs["input_ids"].shape[1] :]
transcription = processor.decode(generated_ids, return_format="transcription_only")
print(transcription)
```

### Chat template

VibeVoice ASR also accepts chat template inputs (`apply_transcription_request` is actually a wrapper for `apply_chat_template` for convenience):
```python
from transformers import AutoProcessor, VibeVoiceAsrForConditionalGeneration

model_id = "microsoft/VibeVoice-ASR-HF"
processor = AutoProcessor.from_pretrained(model_id)
model = VibeVoiceAsrForConditionalGeneration.from_pretrained(model_id, device_map="auto")

chat_template = [
    [
        {
            "role": "user",
            "content": [
                {"type": "text", "text": "About VibeVoice"},
                {
                    "type": "audio",
                    "path": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/realtime_model/vibevoice_tts_german.wav",
                },
            ],
        }
    ],
    [
        {
            "role": "user",
            "content": [
                {
                    "type": "audio",
                    "path": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/example_output/VibeVoice-1.5B_output.wav",
                },
            ],
        }
    ],
]

inputs = processor.apply_chat_template(
    chat_template,
    tokenize=True,
    return_dict=True,
).to(model.device, model.dtype)

output_ids = model.generate(**inputs)
generated_ids = output_ids[:, inputs["input_ids"].shape[1] :]
transcription = processor.decode(generated_ids, return_format="transcription_only")
print(transcription)
```

### Training

VibeVoice ASR can be trained with the loss outputted by the model.

```python
from transformers import AutoProcessor, VibeVoiceAsrForConditionalGeneration

model_id = "microsoft/VibeVoice-ASR-HF"
processor = AutoProcessor.from_pretrained(model_id)
model = VibeVoiceAsrForConditionalGeneration.from_pretrained(model_id, device_map="auto")
model.train()

# Prepare batch of 2
# -- NOTE: the original model is trained to output transcription, speaker ID, and timestamps in JSON-like format. Below we are only using the transcription text as the label
chat_template = [
    [
        {
            "role": "user",
            "content": [
                {"type": "text", "text": "VibeVoice is this novel framework designed for generating expressive, long-form, multi-speaker, conversational audio."},
                {
                    "type": "audio",
                    "path": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/realtime_model/vibevoice_tts_german.wav",
                },
            ],
        }
    ],
    [
        {
            "role": "user",
            "content": [
                {"type": "text", "text": "Hello everyone and welcome to the VibeVoice podcast. I'm your host, Alex, and today we're getting into one of the biggest debates in all of sports: who's the greatest basketball player of all time? I'm so excited to have Sam here to talk about it with me. Thanks so much for having me, Alex. And you're absolutely right. This question always brings out some seriously strong feelings. Okay, so let's get right into it. For me, it has to be Michael Jordan. Six trips to the finals, six championships. That kind of perfection is just incredible. Oh man, the first thing that always pops into my head is that shot against the Cleveland Cavaliers back in '89. Jordan just rises, hangs in the air forever, and just sinks it."},
                {
                    "type": "audio",
                    "path": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/example_output/VibeVoice-1.5B_output.wav",
                },
            ],
        }
    ],
]
inputs = processor.apply_chat_template(
    chat_template,
    tokenize=True,
    return_dict=True,
    output_labels=True,
).to(model.device, model.dtype)

loss = model(**inputs).loss
print("Loss:", loss.item())
loss.backward()
```

### Torch compile

The model can be compiled for faster inference/training.
```python
import time

import torch

from transformers import AutoProcessor, VibeVoiceAsrForConditionalGeneration

model_id = "microsoft/VibeVoice-ASR-HF"

num_warmup = 5
num_runs = 20

# Load processor + model
processor = AutoProcessor.from_pretrained(model_id)
model = VibeVoiceAsrForConditionalGeneration.from_pretrained(model_id, device_map="cuda")

# Prepare static inputs
chat_template = [
    [
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "VibeVoice is this novel framework designed for generating expressive, long-form, multi-speaker, conversational audio.",
                },
                {
                    "type": "audio",
                    "path": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/realtime_model/vibevoice_tts_german.wav",
                },
            ],
        }
    ],
] * 4  # batch size 4
inputs = processor.apply_chat_template(
    chat_template,
    tokenize=True,
    return_dict=True,
).to("cuda", torch.bfloat16)

# Benchmark without compile
print("Warming up without compile...")
with torch.no_grad():
    for _ in range(num_warmup):
        _ = model(**inputs)

torch.cuda.synchronize()

print("\nBenchmarking without torch.compile...")
torch.cuda.synchronize()
start = time.time()
with torch.no_grad():
    for _ in range(num_runs):
        _ = model(**inputs)
torch.cuda.synchronize()
no_compile_time = (time.time() - start) / num_runs
print(f"Average time without compile: {no_compile_time:.4f}s")

# Benchmark with compile
print("\nCompiling model...")
model = torch.compile(model)

print("Warming up with compile (includes graph capture)...")
with torch.no_grad():
    for _ in range(num_warmup):
        _ = model(**inputs)

torch.cuda.synchronize()

print("\nBenchmarking with torch.compile...")
torch.cuda.synchronize()
start = time.time()
with torch.no_grad():
    for _ in range(num_runs):
        _ = model(**inputs)
torch.cuda.synchronize()
compile_time = (time.time() - start) / num_runs
print(f"Average time with compile: {compile_time:.4f}s")

speedup = no_compile_time / compile_time
print(f"\nSpeedup: {speedup:.2f}x")
```

### Pipeline usage

The model can be used as a pipeline, but you will have to define your own methods for parsing the raw output.

```python
from transformers import pipeline

model_id = "microsoft/VibeVoice-ASR-HF"
pipe = pipeline("any-to-any", model=model_id, device_map="auto")
chat_template = [
    {
        "role": "user",
        "content": [
            {"type": "text", "text": "About VibeVoice"},
            {
                "type": "audio",
                "path": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/realtime_model/vibevoice_tts_german.wav",
            },
        ],
    }
]
outputs = pipe(text=chat_template, return_full_text=False)

print("\n" + "=" * 60)
print("RAW PIPELINE OUTPUT")
print("=" * 60)
print(outputs[0]["generated_text"])

print("\n" + "=" * 60)
print("DICT OUTPUT")
print("=" * 60)
dict_output = pipe.processor.extract_speaker_dict(outputs[0]["generated_text"])
print(dict_output)

print("\n" + "=" * 60)
print("TRANSCRIPT OUTPUT")
print("=" * 60)
transcription = pipe.processor.extract_transcription(outputs[0]["generated_text"])
print(transcription)
```

## VibeVoiceAsrConfig[[transformers.VibeVoiceAsrConfig]]

- **acoustic_tokenizer_encoder_config** (`Union[VibeVoiceAcousticTokenizerConfig, dict]`, *optional*) --
  The config object or dictionary of the acoustic tokenizer. This tokenizer extracts acoustic features from audio.
- **semantic_tokenizer_encoder_config** (`Union[VibeVoiceAcousticTokenizerConfig, dict]`, *optional*) --
  The config object or dictionary of the semantic tokenizer. This tokenizer extracts semantic features from audio.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **audio_token_id** (`int`, *optional*, defaults to `151648`) --
  The audio token index used as a placeholder for input audio.
- **audio_bos_token_id** (`int`, *optional*, defaults to 151646) --
  The audio begin-of-sequence token index.
- **audio_eos_token_id** (`int`, *optional*, defaults to 151647) --
  The audio end-of-sequence token index.
- **acoustic_tokenizer_chunk_size** (`int`, *optional*, defaults to 1440000) --
  The chunk size (in number of samples) to use when tokenizer audio inputs. Default corresponds to 60 seconds at 24kHz.

This is the configuration class to store the configuration of a VibeVoiceAsrModel. It is used to instantiate a Vibevoice Asr
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [microsoft/VibeVoice-ASR-HF](https://huggingface.co/microsoft/VibeVoice-ASR-HF)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import VibeVoiceAsrForConditionalGeneration, VibeVoiceAsrConfig, VibeVoiceAcousticTokenizerEncoderConfig, Qwen2Config

>>> # Initializing VibeVoice acoustic and semantic encoder configs
>>> acoustic_config = VibeVoiceAcousticTokenizerEncoderConfig()
>>> semantic_config = VibeVoiceAcousticTokenizerEncoderConfig(hidden_size=128)

>>> # Initializing a Qwen2 config
>>> text_config = Qwen2Config()

>>> # Initializing a VibeVoice ASR configuration
>>> configuration = VibeVoiceAsrConfig(acoustic_config, semantic_config, text_config)

>>> # Initializing a model from the vibevoice_asr style configuration
>>> model = VibeVoiceAsrForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## VibeVoiceAsrProcessor[[transformers.VibeVoiceAsrProcessor]]

'"}, {"name": "audio_bos_token", "val": " = '<|object_ref_start|>'"}, {"name": "audio_eos_token", "val": " = '<|object_ref_end|>'"}, {"name": "audio_duration_token", "val": " = '<|AUDIO_DURATION|>'"}]}>
- **feature_extractor** (`VibeVoiceAcousticTokenizerFeatureExtractor`) --
  The feature extractor for audio processing.
- **tokenizer** (`Qwen2TokenizerFast`) --
  The tokenizer for text processing.
- **chat_template** (`str`, *optional*) --
  A Jinja template which will be used to convert lists of messages in a chat into a tokenizable string.
- **audio_token** (`str`, *optional*, defaults to `"<|box_start|>"`) --
  The audio token placeholder to use in the chat template.
- **audio_bos_token** (`str`, *optional*, defaults to `"<|object_ref_start|>"`) --
  The audio begin-of-sequence token placeholder to use in the chat template.
- **audio_eos_token** (`str`, *optional*, defaults to `"<|object_ref_end|>"`) --
  The audio end-of-sequence token placeholder to use in the chat template.
- **audio_duration_token** (`str`, *optional*, defaults to `"<|AUDIO_DURATION|>"`) --
  The audio duration token placeholder to use in the chat template.

Constructs a VibeVoice ASR processor which wraps [VibeVoiceAcousticTokenizerFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/vibevoice_acoustic_tokenizer#transformers.VibeVoiceAcousticTokenizerFeatureExtractor) and
[Qwen2TokenizerFast](/docs/transformers/v5.14.0/en/model_doc/qwen2#transformers.Qwen2Tokenizer) into a single processor that inherits both the audio feature extraction and
tokenizer functionalities.

See the [__call__()](/docs/transformers/v5.14.0/en/model_doc/vibevoice_asr#transformers.VibeVoiceAsrProcessor.__call__) for more information.

- **text** (`str`, `List[str]`) --
  The input text(s) to process, typically prepared by apply_chat_template with audio token placeholders.
- **audio** (`List[Union[str, np.ndarray]]`) --
  Audio samples for transcription. Should match the number of audio token placeholders in text.
- **output_labels** (bool, *optional*, default=False) --
  Whether to return labels for training.
- ****kwargs** --
  Additional keyword arguments passed to the tokenizer and feature extractor.[BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature)A dictionary with tokenized text (`input_ids`, `attention_mask`) and
audio features (`input_features`, `input_features_mask`).

Main method to process text inputs with optional audio samples for ASR.

This method processes text inputs (typically prepared by apply_chat_template) and optional audio samples
for transcription. It replaces the audio duration placeholder and expands audio token placeholders based
on the actual audio length.

- **audio** (`str`, `list[str]`, `np.ndarray`, `torch.Tensor`, `list[np.ndarray]`, `list[torch.Tensor]`) --
  Audio to transcribe. Strings are interpreted as local paths or URLs and will be loaded automatically by
  the chat template loader; NumPy arrays and PyTorch tensors are forwarded directly.
- **prompt** (`str` or `list[str]`, *optional*) --
  Custom prompt(s) to include in the user turn as extra context. A list must be the same length as the
  batch. When `None`, no additional context is provided.
- ****kwargs** --
  Additional keyword arguments forwarded to [apply_chat_template()](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessorMixin.apply_chat_template) (for example
  `text_kwargs`, `audio_kwargs`, ...).[BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature)Processor outputs ready to be passed to [VibeVoiceAsrForConditionalGeneration.generate()](/docs/transformers/v5.14.0/en/main_classes/text_generation#transformers.GenerationMixin.generate).

Prepare inputs for automatic speech recognition without manually writing the chat template.

- **return_format** (`str`, *optional*, defaults to `"raw"`) --
  Options are:
  - `"raw"`: Return a list of raw decoded strings from the tokenizer, without any parsing.
  - `"parsed"`: Return a list of list of parsed dictionary objects for each speaker utterance with timestamps.
  - `"transcription_only"`: Return a list of extracted transcription strings.

  `skip_special_tokens` is automatically enforced (hard-set) to `True` for `"parsed"` and `"transcription_only"`.

Forward arguments to [decode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.decode) and optionally parse the dict-like output.

VibeVoice ASR outputs transcriptions in a dictionary-like format, e.g.:
```
[
    'assistant
":0.0,"End":7.56,"Speaker":0,"Content":"text"}]

    'assistant
":0,"End":5.20,"Speaker":0,"Content":"text"}]

]
```

## VibeVoiceAsrModel[[transformers.VibeVoiceAsrModel]]

- **config** ([VibeVoiceAsrConfig](/docs/transformers/v5.14.0/en/model_doc/vibevoice_asr#transformers.VibeVoiceAsrConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The VibeVoice ASR model (acoustic tokenizer + semantic tokenizer + multi-modal projector + language model),
without a language modeling head.

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
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
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
- **input_values** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Float values of input raw speech waveform. Values can be obtained by loading a `.flac` or `.wav` audio file
  into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library
  (`pip install torchcodec`) or the soundfile library (`pip install soundfile`).
  To prepare the array into `input_values`, the [AutoProcessor](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoProcessor) should be used for padding and conversion
  into a tensor of type `torch.FloatTensor`. See [VibeVoiceAsrProcessor.__call__()](/docs/transformers/v5.14.0/en/model_doc/vibevoice_asr#transformers.VibeVoiceAsrProcessor.__call__) for details.
- **padding_mask** (`torch.BoolTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing operations on padding feature indices.
- **acoustic_tokenizer_chunk_size** (`int`, *optional*) --
  Size of audio chunks processed by the acoustic and semantic tokenizers.`VibeVoiceAsrModelOutputWithPast` or `tuple(torch.FloatTensor)`A `VibeVoiceAsrModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([VibeVoiceAsrConfig](/docs/transformers/v5.14.0/en/model_doc/vibevoice_asr#transformers.VibeVoiceAsrConfig)) and inputs.
The [VibeVoiceAsrModel](/docs/transformers/v5.14.0/en/model_doc/vibevoice_asr#transformers.VibeVoiceAsrModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **audio_hidden_states** (`torch.FloatTensor`, *optional*) -- Projected audio hidden states.

## VibeVoiceAsrForConditionalGeneration[[transformers.VibeVoiceAsrForConditionalGeneration]]

- **config** ([VibeVoiceAsrConfig](/docs/transformers/v5.14.0/en/model_doc/vibevoice_asr#transformers.VibeVoiceAsrConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The VibeVoice ASR model with pre-trained acoustic tokenizers and a language model.

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
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
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
- **input_values** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Float values of input raw speech waveform. Values can be obtained by loading a `.flac` or `.wav` audio file
  into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library
  (`pip install torchcodec`) or the soundfile library (`pip install soundfile`).
  To prepare the array into `input_values`, the [AutoProcessor](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoProcessor) should be used for padding and conversion
  into a tensor of type `torch.FloatTensor`. See [VibeVoiceAsrProcessor.__call__()](/docs/transformers/v5.14.0/en/model_doc/vibevoice_asr#transformers.VibeVoiceAsrProcessor.__call__) for details.
- **padding_mask** (`torch.BoolTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing operations on padding feature indices.
- **acoustic_tokenizer_chunk_size** (`int`, *optional*) --
  Size of audio chunks processed by the acoustic and semantic tokenizers.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).`VibeVoiceAsrCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `VibeVoiceAsrCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([VibeVoiceAsrConfig](/docs/transformers/v5.14.0/en/model_doc/vibevoice_asr#transformers.VibeVoiceAsrConfig)) and inputs.
The [VibeVoiceAsrForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/vibevoice_asr#transformers.VibeVoiceAsrForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores.
- **past_key_values** (`Cache`, *optional*) -- Cache instance.
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
>>> from transformers import VibeVoiceAsrForConditionalGeneration, AutoProcessor

>>> model_id = "microsoft/VibeVoice-ASR-HF"
>>> processor = AutoProcessor.from_pretrained(model_id)
>>> model = VibeVoiceAsrForConditionalGeneration.from_pretrained(model_id, dtype="auto", device_map="auto")
```
