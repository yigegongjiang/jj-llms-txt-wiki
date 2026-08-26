# Qwen3 ASR

## Overview

Qwen3 ASR is an automatic speech recognition model from Alibaba's Qwen team that combines a Whisper-style audio encoder with a Qwen3 language model decoder for speech-to-text transcription. The model supports automatic language detection and multilingual transcription.

A forced aligner model is also included. It can be used to timestamp a provided transcript and its audio. It uses the same audio encoder model with a classification head that predicts a word's length. This model can be used with the transcript from any ASR model (see the example below with Parakeet CTC).

Available checkpoints:
- [Qwen/Qwen3-ASR-1.7B-hf](https://huggingface.co/Qwen/Qwen3-ASR-1.7B-hf)
- [Qwen/Qwen3-ASR-0.6B-hf](https://huggingface.co/Qwen/Qwen3-ASR-0.6B-hf)
- [Qwen/Qwen3-ForcedAligner-0.6B-hf](https://huggingface.co/Qwen/Qwen3-ForcedAligner-0.6B-hf)

The following languages are supported:
- `Qwen3-ASR-1.7B` and `Qwen3-ASR-0.6B`: Chinese (zh), English (en), Cantonese (yue), Arabic (ar), German (de), French (fr), Spanish (es), Portuguese (pt), Indonesian (id), Italian (it), Korean (ko), Russian (ru), Thai (th), Vietnamese (vi), Japanese (ja), Turkish (tr), Hindi (hi), Malay (ms), Dutch (nl), Swedish (sv), Danish (da), Finnish (fi), Polish (pl), Czech (cs), Filipino (fil), Persian (fa), Greek (el), Hungarian (hu), Macedonian (mk), Romanian (ro).
- `Qwen3-ForcedAligner-0.6B`: Chinese (zh), English (en), Cantonese (yue), French (fr), German (de), Italian (it), Japanese (ja), Korean (ko), Portuguese (pt), Russian (ru), Spanish (es).

See the original repository at [QwenLM/Qwen3-ASR](https://github.com/QwenLM/Qwen3-ASR) and the [report](https://huggingface.co/papers/2601.21337) for more details.

This model was contributed by [Eric Bezzam](https://huggingface.co/bezzam) and [Muhammed Tariq](https://huggingface.co/mbtariq82).

## Usage

### Simple transcription

The simplest way to transcribe audio is with `apply_transcription_request`, which handles the chat template formatting for you, namely it is a convenience wrapper for `apply_chat_template` (see [Chat template](#chat-template) below).

```python
from transformers import AutoProcessor, AutoModelForMultimodalLM

model_id = "Qwen/Qwen3-ASR-1.7B-hf"
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForMultimodalLM.from_pretrained(model_id, device_map="auto")
print(f"Model loaded on {model.device} with dtype {model.dtype}")

inputs = processor.apply_transcription_request(
    audio="https://huggingface.co/datasets/bezzam/audio_samples/resolve/main/librispeech_mr_quilter.wav",
).to(model.device, model.dtype)

output_ids = model.generate(**inputs, max_new_tokens=256)
generated_ids = output_ids[:, inputs["input_ids"].shape[1]:]

# Raw output includes language tag and <asr_text> marker
raw = processor.decode(generated_ids)[0]
print(f"Raw: {raw}")

# Parsed output: dict with "language" and "transcription"
parsed = processor.decode(generated_ids, return_format="parsed")[0]
print(f"Parsed: {parsed}")

# Extract only the transcription text
transcription = processor.decode(generated_ids, return_format="transcription_only")[0]
print(f"Transcription: {transcription}")

"""
Raw: language English<asr_text>Mr. Quilter is the apostle of the middle classes, and we are glad to welcome his gospel.
Parsed: {'language': 'English', 'transcription': 'Mr. Quilter is the apostle of the middle classes, and we are glad to welcome his gospel.'}
Transcription: Mr. Quilter is the apostle of the middle classes, and we are glad to welcome his gospel.
"""
```

### Forcing the language

You can force the transcription language as shown below.

```python
from transformers import AutoProcessor, AutoModelForMultimodalLM

model_id = "Qwen/Qwen3-ASR-1.7B-hf"
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForMultimodalLM.from_pretrained(model_id, device_map="auto")

# Without language hint (auto-detect)
inputs = processor.apply_transcription_request(
    audio="https://qianwen-res.oss-cn-beijing.aliyuncs.com/Qwen3-ASR-Repo/asr_zh.wav",
).to(model.device, model.dtype)
output_ids = model.generate(**inputs, max_new_tokens=256)
generated_ids = output_ids[:, inputs["input_ids"].shape[1]:]
print(f"Auto-detect: {processor.decode(generated_ids, return_format='transcription_only')[0]}")

# With forced language
inputs = processor.apply_transcription_request(
    audio="https://qianwen-res.oss-cn-beijing.aliyuncs.com/Qwen3-ASR-Repo/asr_zh.wav",
    language="Chinese",  # or language code "zh"
).to(model.device, model.dtype)
output_ids = model.generate(**inputs, max_new_tokens=256)
generated_ids = output_ids[:, inputs["input_ids"].shape[1]:]
print(f"Forced:      {processor.decode(generated_ids, return_format='transcription_only')[0]}")
```

### Context / hotwords

You can pass free-form context (e.g. domain-specific vocabulary, names, or background information) via `prompt` to bias the transcription.

```python
from transformers import AutoProcessor, AutoModelForMultimodalLM

model_id = "Qwen/Qwen3-ASR-1.7B-hf"
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForMultimodalLM.from_pretrained(model_id, device_map="auto")

inputs = processor.apply_transcription_request(
    audio="https://huggingface.co/datasets/bezzam/audio_samples/resolve/main/librispeech_mr_quilter.wav",
    prompt="Vocabulary: Quilter, apostle, gospel.",
    language="English",
).to(model.device, model.dtype)

output_ids = model.generate(**inputs, max_new_tokens=256)
generated_ids = output_ids[:, inputs["input_ids"].shape[1]:]
print(processor.decode(generated_ids, return_format="transcription_only")[0])
```

### Batch inference

Batch inference is possible by passing a list of audios and, if provided, a list of languages and/or prompts.

```python
from transformers import AutoProcessor, AutoModelForMultimodalLM

model_id = "Qwen/Qwen3-ASR-1.7B-hf"
audio = [
    "https://huggingface.co/datasets/bezzam/audio_samples/resolve/main/librispeech_mr_quilter.wav",
    "https://qianwen-res.oss-cn-beijing.aliyuncs.com/Qwen3-ASR-Repo/asr_zh.wav",
]

processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForMultimodalLM.from_pretrained(model_id, device_map="auto")

inputs = processor.apply_transcription_request(
    audio,
    prompt=["Vocabulary: Quilter, apostle, gospel.", None],
    language=[None, "zh"],  # language codes ("zh") and full names ("Chinese") are both accepted
).to(model.device, model.dtype)

output_ids = model.generate(**inputs, max_new_tokens=256)
generated_ids = output_ids[:, inputs["input_ids"].shape[1]:]
transcriptions = processor.decode(generated_ids, return_format="transcription_only")

for i, text in enumerate(transcriptions):
    print(f"Audio {i + 1}: {text}")
```

### Chat template

Qwen3 ASR also accepts chat template inputs. The `apply_transcription_request` usage [above](#simple-transcription) is a convenience wrapper for `apply_chat_template`.

The language can be forced through the chat template by *prefilling* the assistant turn with `language <NAME><asr_text>` and passing `continue_final_message=True`, which is what `apply_transcription_request` does under the hood. Note that if forcing the language, a prefill should be set for all audio in a batch (as shown below).

```python
from transformers import AutoProcessor, Qwen3ASRForConditionalGeneration

model_id = "Qwen/Qwen3-ASR-1.7B-hf"
processor = AutoProcessor.from_pretrained(model_id)
model = Qwen3ASRForConditionalGeneration.from_pretrained(model_id, device_map="auto")

chat_template = [
    [
        # Context/hotwords as system message
        {"role": "system", "content": [{"type": "text", "text": "Vocabulary: Quilter, apostle, gospel."}]},
        {
            "role": "user",
            "content": [
                {
                    "type": "audio",
                    "path": "https://huggingface.co/datasets/bezzam/audio_samples/resolve/main/librispeech_mr_quilter.wav",
                },
            ],
        },
        # empty prefill since forcing language in the other sample
        {"role": "assistant", "content": [{"type": "text", "text": ""}]},
    ],
    [
        {
            "role": "user",
            "content": [
                {
                    "type": "audio",
                    "path": "https://qianwen-res.oss-cn-beijing.aliyuncs.com/Qwen3-ASR-Repo/asr_zh.wav",
                },
            ],
        },
        {"role": "assistant", "content": [{"type": "text", "text": "language Chinese<asr_text>"}]},
    ],
]

inputs = processor.apply_chat_template(
    chat_template, tokenize=True, return_dict=True, continue_final_message=True,
).to(model.device, model.dtype)

output_ids = model.generate(**inputs, max_new_tokens=256)
generated_ids = output_ids[:, inputs["input_ids"].shape[1]:]
transcriptions = processor.decode(generated_ids, return_format="transcription_only")
for text in transcriptions:
    print(text)
```

### Training

Qwen3 ASR can be trained with the loss outputted by the model. Put the target transcript in the assistant turn — in the model's output format `language <NAME><asr_text>...` to preserve the pretrained behavior — and pass `output_labels=True`. Audio and padding positions are masked automatically.

```python
from transformers import AutoProcessor, Qwen3ASRForConditionalGeneration

model_id = "Qwen/Qwen3-ASR-1.7B-hf"
processor = AutoProcessor.from_pretrained(model_id)
model = Qwen3ASRForConditionalGeneration.from_pretrained(model_id, device_map="auto")
model.train()

transcript = "Mr. Quilter is the apostle of the middle classes, and we are glad to welcome his gospel."
conversation = [
    [
        {
            "role": "user",
            "content": [
                {
                    "type": "audio",
                    "path": "https://huggingface.co/datasets/bezzam/audio_samples/resolve/main/librispeech_mr_quilter.wav",
                },
            ],
        },
        {"role": "assistant", "content": [{"type": "text", "text": f"language English<asr_text>{transcript}"}]},
    ],
]

inputs = processor.apply_chat_template(
    conversation, tokenize=True, return_dict=True, processor_kwargs={"output_labels": True},
).to(model.device, model.dtype)

loss = model(**inputs).loss
print("Loss:", loss.item())
loss.backward()
```

### Forced alignment (word-level timestamping)

Use `Qwen3ASRForTokenClassification` to obtain word-level timestamps from a transcript. First transcribe with the ASR model, then align with the forced aligner.

The following languages are supported: Chinese (zh), English (en), Cantonese (yue), French (fr), German (de), Italian (it), Japanese (ja), Korean (ko), Portuguese (pt), Russian (ru), Spanish (es).

Japanese requires the `nagisa` library, while Korean requires the `soynlp` library:
```
pip install nagisa soynlp
```

#### With Qwen3 ASR

```python
import torch
from transformers import AutoProcessor, AutoModelForMultimodalLM, AutoModelForTokenClassification

asr_model_id = "Qwen/Qwen3-ASR-0.6B-hf"
aligner_model_id = "Qwen/Qwen3-ForcedAligner-0.6B-hf"

asr_processor = AutoProcessor.from_pretrained(asr_model_id)
asr_model = AutoModelForMultimodalLM.from_pretrained(asr_model_id, device_map="auto")

aligner_processor = AutoProcessor.from_pretrained(aligner_model_id)
aligner_model = AutoModelForTokenClassification.from_pretrained(
    aligner_model_id, dtype=torch.bfloat16, device_map="auto"
)

audio_url = "https://huggingface.co/datasets/bezzam/audio_samples/resolve/main/librispeech_mr_quilter.wav"

# Step 1: Transcribe
inputs = asr_processor.apply_transcription_request(audio=audio_url)
inputs = inputs.to(asr_model.device, asr_model.dtype)
output_ids = asr_model.generate(**inputs, max_new_tokens=256)
generated_ids = output_ids[:, inputs["input_ids"].shape[1]:]
parsed = asr_processor.decode(generated_ids, return_format="parsed")[0]
transcript = parsed["transcription"]
language = parsed["language"] or "English"

# Step 2: Prepare alignment inputs
aligner_inputs, word_lists = aligner_processor.prepare_forced_aligner_inputs(
    audio=audio_url, transcript=transcript, language=language,
)
aligner_inputs = aligner_inputs.to(aligner_model.device, aligner_model.dtype)

# Step 3: Run forced aligner
with torch.inference_mode():
    outputs = aligner_model(**aligner_inputs)

# Step 4: Decode timestamps
timestamps = aligner_processor.decode_forced_alignment(
    logits=outputs.logits,
    input_ids=aligner_inputs["input_ids"],
    word_lists=word_lists,
    timestamp_token_id=aligner_model.config.timestamp_token_id,
)[0]

for item in timestamps:
    print(f"{item['text']:<20} {item['start_time']:>8.3f}s → {item['end_time']:>8.3f}s")

"""
Word                  Start (s)    End (s)
------------------------------------------
Mr                        0.560      0.800
Quilter                   0.800      1.280
is                        1.280      1.440
the                       1.440      1.520
apostle                   1.520      2.080
...
"""
```

#### With another ASR model

The forced aligner is model-agnostic, meaning the transcripts from any ASR system can be provided. Below is a batch inference example using [NVIDIA Parakeet CTC](https://huggingface.co/nvidia/parakeet-ctc-1.1b) for transcription.

```python
import torch
from datasets import Audio, load_dataset
from transformers import AutoModelForCTC, AutoProcessor, AutoModelForTokenClassification

parakeet_processor = AutoProcessor.from_pretrained("nvidia/parakeet-ctc-1.1b")
parakeet_model = AutoModelForCTC.from_pretrained(
    "nvidia/parakeet-ctc-1.1b", dtype="auto", device_map="auto",
)

aligner_model_id = "Qwen/Qwen3-ForcedAligner-0.6B-hf"
aligner_processor = AutoProcessor.from_pretrained(aligner_model_id)
aligner_model = AutoModelForTokenClassification.from_pretrained(
    aligner_model_id, dtype=torch.bfloat16, device_map="auto",
)

ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
ds = ds.cast_column("audio", Audio(sampling_rate=parakeet_processor.feature_extractor.sampling_rate))
audio_arrays = [ds[i]["audio"]["array"] for i in range(3)]
sr = ds[0]["audio"]["sampling_rate"]

# Batch transcribe with Parakeet
inputs = parakeet_processor(audio_arrays, sampling_rate=sr, return_tensors="pt", padding=True).to(
    parakeet_model.device, dtype=parakeet_model.dtype
)
with torch.inference_mode():
    outputs = parakeet_model.generate(**inputs)
transcripts = parakeet_processor.decode(outputs)

# Batch align with Qwen3 Forced Aligner
aligner_inputs, word_lists = aligner_processor.prepare_forced_aligner_inputs(
    audio=audio_arrays, transcript=transcripts, language="English",
)
aligner_inputs = aligner_inputs.to(aligner_model.device, aligner_model.dtype)

with torch.inference_mode():
    aligner_outputs = aligner_model(**aligner_inputs)

batch_timestamps = aligner_processor.decode_forced_alignment(
    logits=aligner_outputs.logits,
    input_ids=aligner_inputs["input_ids"],
    word_lists=word_lists,
    timestamp_token_id=aligner_model.config.timestamp_token_id,
)

for i, (transcript, timestamps) in enumerate(zip(transcripts, batch_timestamps)):
    print(f"\n[Sample {i}] {transcript}")
    for item in timestamps[:5]:
        print(f"  {item['text']:<20} {item['start_time']:>8.3f}s → {item['end_time']:>8.3f}s")
    if len(timestamps) > 5:
        print(f"  ... ({len(timestamps) - 5} more words)")
```

### Torch compile

Both the ASR and forced aligner models support `torch.compile` for faster inference. The forced aligner is an especially good fit for compilation because it runs a single forward pass (no autoregressive decoding). This makes it ideal for **bulk audio timestamping**: transcribe with any ASR model, then batch-align with the compiled forced aligner for maximum throughput.

#### Forced aligner

On an A100, we observed a speed-up of ~1.2 for a batch size of 4 ([script](https://gist.github.com/ebezzam/3e0551708631784aeb684e0e838299f3#file-benchmark_compile_forced_alignment-py)).

```python
import torch
from transformers import AutoProcessor, AutoModelForTokenClassification

model_id = "Qwen/Qwen3-ForcedAligner-0.6B-hf"
num_warmup = 5
batch_size = 4

processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForTokenClassification.from_pretrained(model_id, dtype=torch.bfloat16).to("cuda")

# Prepare a batch of 4 samples
audio_url = "https://huggingface.co/datasets/bezzam/audio_samples/resolve/main/librispeech_mr_quilter.wav"
transcript = "Mr. Quilter is the apostle of the middle classes."

aligner_inputs, word_lists = processor.prepare_forced_aligner_inputs(
    audio=[audio_url] * batch_size,
    transcript=[transcript] * batch_size,
    language=["English"] * batch_size,
)
aligner_inputs = aligner_inputs.to(model.device, torch.bfloat16)

# Warm-up and apply model
model = torch.compile(model)
with torch.no_grad():
    for _ in range(num_warmup):
        _ = model(**aligner_inputs)
with torch.no_grad():
    _ = model(**aligner_inputs)
```

#### ASR model (generate)

For autoregressive transcription, `torch.compile` accelerates the per-token forward passes inside `generate` setting providing a `CompileConfig` object.

On an A100, we observed a speed-up of ~3.8 for a batch size of 4 ([script](https://gist.github.com/ebezzam/3e0551708631784aeb684e0e838299f3#file-benchmark_compile_generate-py)).

```python
import torch
from transformers import AutoProcessor, AutoModelForMultimodalLM, CompileConfig

model_id = "Qwen/Qwen3-ASR-1.7B-hf"
num_warmup = 3
max_new_tokens = 256

processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForMultimodalLM.from_pretrained(model_id, dtype=torch.bfloat16).to("cuda").eval()

audio_url = "https://huggingface.co/datasets/bezzam/audio_samples/resolve/main/librispeech_mr_quilter.wav"
inputs = processor.apply_transcription_request(
    audio=[audio_url] * 4,  # batch of 4
).to(model.device, torch.bfloat16)

compile_config = CompileConfig()

# Warmup
with torch.inference_mode():
    for _ in range(num_warmup):
        _ = model.generate(
            **inputs, max_new_tokens=max_new_tokens, do_sample=False,
            cache_implementation="static", compile_config=compile_config,
        )
torch.cuda.synchronize()

# Apply model
with torch.inference_mode():
    output_ids = model.generate(
        **inputs, max_new_tokens=max_new_tokens, do_sample=False,
        cache_implementation="static", compile_config=compile_config,
    )
generated_ids = output_ids[:, inputs["input_ids"].shape[1] :]
text_compiled = processor.decode(generated_ids, return_format="transcription_only")[0]
print(f"Output:  {text_compiled}")
```

### Pipeline usage

```python
from transformers import pipeline

model_id = "Qwen/Qwen3-ASR-1.7B-hf"
pipe = pipeline("any-to-any", model=model_id, device_map="auto")

chat_template = [
    {
        "role": "user",
        "content": [
            {
                "type": "audio",
                "path": "https://huggingface.co/datasets/bezzam/audio_samples/resolve/main/librispeech_mr_quilter.wav",
            },
        ],
    }
]
outputs = pipe(text=chat_template, return_full_text=False)
raw_text = outputs[0]["generated_text"]
print(f"Raw: {raw_text}")

# Use processor helper to extract transcription
transcription = pipe.processor.extract_transcription(raw_text)
print(f"Transcription: {transcription}")
```

## Qwen3ASRConfig[[transformers.Qwen3ASRConfig]]

#### transformers.Qwen3ASRConfig[[transformers.Qwen3ASRConfig]]

```python
transformers.Qwen3ASRConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, audio_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, audio_token_id: int = 151676, timestamp_token_id: int = 151705, pad_token_id: int = 151645, eos_token_id: list[int] | tuple[int, ...] | int = (151643, 151645), initializer_range: float = 0.02, tie_word_embeddings: bool = True, token_classification_bias: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen3_asr/configuration_qwen3_asr.py#L73)

**Parameters:**

audio_config (*Union[dict, ~configuration_utils.PreTrainedConfig]*, *optional*) : The config object or dictionary of the audio backbone.

text_config (*Union[dict, ~configuration_utils.PreTrainedConfig]*, *optional*) : The config object or dictionary of the text backbone.

audio_token_id (*int*, *optional*, defaults to 151676) : The audio token id to encode the audio prompt.

timestamp_token_id (*int*, *optional*, defaults to 151705) : Token ID of the `&amp;lt;timestamp>` marker in the tokenizer vocabulary. These markers delimit word boundaries in the forced-alignment input sequence.

pad_token_id (*int*, *optional*, defaults to *151645*) : Token id used for padding in the vocabulary.

eos_token_id (*Union[list[int], tuple[int, ...], int]*, *optional*, defaults to *(151643, 151645)*) : Token id used for end-of-stream in the vocabulary.

initializer_range (*float*, *optional*, defaults to *0.02*) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

tie_word_embeddings (*bool*, *optional*, defaults to *True*) : Whether to tie weight embeddings according to model's *tied_weights_keys* mapping.

token_classification_bias (*bool*, *optional*, defaults to False) : Whether the token classification head for forced alignment should have a bias term.

This is the configuration class to store the configuration of a Qwen3ASRModel. It is used to instantiate a Qwen3 Asr
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Qwen/Qwen3-ASR-1.7B-hf](https://huggingface.co/Qwen/Qwen3-ASR-1.7B-hf)

Configuration objects inherit from [*PreTrainedConfig*] and can be used to control the model outputs. Read the
documentation from [*PreTrainedConfig*] for more information.

Example:

```python
>>> from transformers import Qwen3ASRForConditionalGeneration, Qwen3ASRConfig

>>> # Initializing a Qwen3ASR style configuration
>>> configuration = Qwen3ASRConfig()

>>> # Initializing a model from the configuration
>>> model = Qwen3ASRForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Qwen3ASREncoderConfig[[transformers.Qwen3ASREncoderConfig]]

#### transformers.Qwen3ASREncoderConfig[[transformers.Qwen3ASREncoderConfig]]

```python
transformers.Qwen3ASREncoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, num_mel_bins: int = 128, encoder_layers: int = 24, encoder_attention_heads: int = 16, encoder_ffn_dim: int = 4096, d_model: int = 1024, dropout: float | int = 0.0, attention_dropout: float | int = 0.0, activation_function: str = 'gelu', activation_dropout: float | int = 0.0, scale_embedding: bool = False, initializer_range: float = 0.02, n_window: int = 50, output_dim: int = 3584, n_window_infer: int = 800, downsample_hidden_size: int = 480, max_position_embeddings: int = 13)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen3_asr/configuration_qwen3_asr.py#L30)

**Parameters:**

num_mel_bins (*int*, *optional*, defaults to *128*) : Number of mel features used per input frame. Should correspond to the value used in the *AutoFeatureExtractor* class.

encoder_layers (*int*, *optional*, defaults to *24*) : Number of hidden layers in the Transformer encoder. Will use the same value as *num_layers* if not set.

encoder_attention_heads (*int*, *optional*, defaults to *16*) : Number of attention heads for each attention layer in the Transformer encoder.

encoder_ffn_dim (*int*, *optional*, defaults to *4096*) : Dimensionality of the "intermediate" (often named feed-forward) layer in encoder.

d_model (*int*, *optional*, defaults to *1024*) : Size of the encoder layers and the pooler layer.

dropout (*Union[float, int]*, *optional*, defaults to *0.0*) : The ratio for all dropout layers.

attention_dropout (*Union[float, int]*, *optional*, defaults to *0.0*) : The dropout ratio for the attention probabilities.

activation_function (*str*, *optional*, defaults to *gelu*) : The non-linear activation function (function or string) in the decoder. For example, *"gelu"*, *"relu"*, *"silu"*, etc.

activation_dropout (*Union[float, int]*, *optional*, defaults to *0.0*) : The dropout ratio for activations inside the fully connected layer.

scale_embedding (*bool*, *optional*, defaults to *False*) : Whether to scale embeddings by dividing by sqrt(d_model).

initializer_range (*float*, *optional*, defaults to *0.02*) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

n_window (*int*, *optional*, defaults to 50) : Half the number of mel frames in one encoder chunk. Each chunk processed by the conv stack has `2 * n_window` mel frames (1 second of audio at 16 kHz with a 10 ms hop).

output_dim (*int*, *optional*, defaults to 3584) : Dimensionality of the output.

n_window_infer (*int*, *optional*, defaults to 800) : Number of mel frames worth of audio over which each attention window spans. Must be a multiple of `n_window * 2` so attention windows align with encoder chunks.

downsample_hidden_size (*int*, *optional*, defaults to 480) : Hidden size of the convolutional downsampling stack.

max_position_embeddings (*int*, *optional*, defaults to *13*) : The maximum sequence length that this model might ever be used with.

This is the configuration class to store the configuration of a Qwen3ASRModel. It is used to instantiate a Qwen3 Asr
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Qwen/Qwen3-ASR-1.7B-hf](https://huggingface.co/Qwen/Qwen3-ASR-1.7B-hf)

Configuration objects inherit from [*PreTrainedConfig*] and can be used to control the model outputs. Read the
documentation from [*PreTrainedConfig*] for more information.

## Qwen3ASRFeatureExtractor[[transformers.Qwen3ASRFeatureExtractor]]

#### transformers.Qwen3ASRFeatureExtractor[[transformers.Qwen3ASRFeatureExtractor]]

```python
transformers.Qwen3ASRFeatureExtractor(feature_size = 128, sampling_rate = 16000, hop_length = 160, chunk_length = 30, n_fft = 400, padding_value = 0.0, dither = 0.0, return_attention_mask = True, n_window = 50, min_length = 8000, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen3_asr/feature_extraction_qwen3_asr.py#L31)

**Parameters:**

feature_size (*int*, *optional*, defaults to 128) : Number of mel filter banks.

sampling_rate (*int*, *optional*, defaults to 16000) : Audio sampling rate in Hz.

hop_length (*int*, *optional*, defaults to 160) : Length of the overlapping windows for the STFT used to obtain the Mel Frequency coefficients.

chunk_length (*int*, *optional*, defaults to 30) : Maximum audio length (in seconds) used to trim/pad when `padding="max_length"`.

n_fft (*int*, *optional*, defaults to 400) : Size of the Fourier transform.

padding_value (*float*, *optional*, defaults to 0.0) : Padding value used to pad the raw audio.

dither (*float*, *optional*, defaults to 0.0) : If non-zero, adds Gaussian noise (*std = dither*) to each STFT frame.

return_attention_mask (*bool*, *optional*, defaults to *True*) : Whether to return the attention mask corresponding to the padded mel frames.

n_window (*int*, *optional*, defaults to 50) : Half the mel-frame chunk size used for padding. The log-mel time axis is right-padded to a multiple of `2 * n_window`.

min_length (*int*, *optional*, defaults to 8000) : Minimum number of samples for each audio clip. Clips shorter than this are zero-padded, matching the original Qwen3-ASR library behaviour.

Constructs a Qwen3 ASR feature extractor.

Extracts 128-bin log-mel features from raw speech, then right-pads the mel time axis to a multiple of `2 * n_window`.

#### __call__[[transformers.Qwen3ASRFeatureExtractor.__call__]]

```python
__call__(raw_speech: numpy.ndarray | list[float] | list[numpy.ndarray] | list[list[float]], truncation: bool = False, pad_to_multiple_of: int | None = None, return_tensors: str | None = 'pt', return_attention_mask: bool | None = None, padding: str | None = 'max_length', max_length: int | None = None, sampling_rate: int | None = None, n_window: int | None = None, device: str | None = 'cpu', **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen3_asr/feature_extraction_qwen3_asr.py#L129)

**Parameters:**

raw_speech (*np.ndarray*, *list[float]*, *list[np.ndarray]*, *list[list[float]]*) : The sequence or batch of sequences to be padded. Mono-channel audio only.

pad_to_multiple_of (*int*, *optional*) : If set, pads the raw audio to a multiple of this value (in samples). Separate from `n_window`, which applies to the mel-frame axis.

n_window (*int*, *optional*) : Override the instance's `n_window` for this call. The mel axis is padded to a multiple of `2 * n_window`. Set to `0` to skip mel-axis padding entirely.

device (*str*, *optional*, defaults to *"cpu"*) : Device used to compute the log-mel spectrogram.

Prepare log-mel features from one or several audio sequences.

## Qwen3ASRProcessor[[transformers.Qwen3ASRProcessor]]

#### transformers.Qwen3ASRProcessor[[transformers.Qwen3ASRProcessor]]

```python
transformers.Qwen3ASRProcessor(feature_extractor = None, tokenizer = None, chat_template = None, timestamp_segment_time: float = 80)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen3_asr/processing_qwen3_asr.py#L401)

**Parameters:**

feature_extractor (`Qwen3ASRFeatureExtractor`) : The feature extractor is a required input.

tokenizer (`Qwen2Tokenizer`) : The tokenizer is a required input.

chat_template (`str`) : A Jinja template to convert lists of messages in a chat into a tokenizable string.

timestamp_segment_time (`float`, *optional*) : Milliseconds per timestamp class. Defaults to 80 ms.

Constructs a Qwen3ASRProcessor which wraps a feature extractor and a tokenizer into a single processor.

[Qwen3ASRProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRProcessor) offers all the functionalities of [Qwen3ASRFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRFeatureExtractor) and [Qwen2Tokenizer](/docs/transformers/v5.15.1/en/model_doc/qwen2#transformers.Qwen2Tokenizer). See the
[~Qwen3ASRFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRFeatureExtractor) and [~Qwen2Tokenizer](/docs/transformers/v5.15.1/en/model_doc/qwen2#transformers.Qwen2Tokenizer) for more information.

#### __call__[[transformers.Qwen3ASRProcessor.__call__]]

```python
__call__(text: str | list[str], audio: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor']], output_labels: bool | None = False, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen3_asr/processing_qwen3_asr.py#L424)

**Parameters:**

text (`Union[str, list[str]]`) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

audio (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`) : The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor. In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels, and T is the sample length of the audio.

output_labels (`bool`, *optional*, default=False) : Whether to return labels for training.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) : Additional processing options for each modality (text, images, videos, audio). Model-specific parameters are listed above; see the TypedDict class for the complete list of supported arguments.

**Returns:** [BatchFeature](/docs/transformers/v5.15.1/en/main_classes/image_processor#transformers.BatchFeature)

A dictionary with tokenized text (`input_ids`, `attention_mask`) and
audio features (`input_features`, `input_features_mask`).

#### apply_transcription_request[[transformers.Qwen3ASRProcessor.apply_transcription_request]]

```python
apply_transcription_request(audio: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor'], list[typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor']]]], language: str | list[str] | None = None, prompt: str | list[str] | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen3_asr/processing_qwen3_asr.py#L492)

**Parameters:**

audio (*AudioInput* or *list[AudioInput]*) : Audio to transcribe. Can be a URL string, local path, numpy array, or a list of these.

language (*str* or *list[str]*, *optional*) : Language(s) to force for transcription. Accepts full names (e.g. `"English"`, `"Chinese"`) or ISO codes (e.g. `"en"`, `"zh"`). A list must be the same length as the audio batch. Following the original implementation, the language is forced by appending `language &amp;lt;NAME>&amp;lt;asr_text>` after the generation prompt, so the model generates only the transcription text. When `None`, the model performs automatic language detection and outputs `language &amp;lt;NAME>&amp;lt;asr_text>...` itself.

prompt (*str* or *list[str]*, *optional*) : Context/hotwords to include as the system prompt, e.g. domain-specific words or phrases to bias the transcription towards. A list must be the same length as the audio batch. When *None*, the system prompt is left empty.

- ****kwargs** : Additional keyword arguments forwarded to [*~Qwen3ASRProcessor.apply_chat_template*] and the underlying processor call (for example *text_kwargs*, *audio_kwargs*, ...).

**Returns:** [*BatchFeature*]

Processor outputs ready to be passed to
[*Qwen3ASRForConditionalGeneration.generate*].

Prepare inputs for automatic speech recognition without manually writing the chat template.

#### prepare_forced_aligner_inputs[[transformers.Qwen3ASRProcessor.prepare_forced_aligner_inputs]]

```python
prepare_forced_aligner_inputs(audio: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor']], transcript: str | list[str], language: str | list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen3_asr/processing_qwen3_asr.py#L689)

**Parameters:**

audio (*AudioInput*) : Audio input(s).  Accepts paths, URLs, numpy arrays, or a list of these.

transcript (*str* or *list[str]*) : Transcript(s) to align against the audio.

language (*str*, *list[str]*, or *None*, *optional*) : Language hint(s). Currently unused in tokenization but reserved for language-specific tokenizers (e.g. Japanese, Korean).

- ****kwargs** : Additional keyword arguments forwarded to [*~Qwen3ASRProcessor.apply_chat_template*].

**Returns:** *tuple[BatchFeature, list[list[str]]]*

- `inputs`: A [*BatchFeature*] with `input_ids`, `attention_mask`,
  `input_features`, and `input_features_mask` ready for the forced
  aligner model.
- `word_lists`: A list (one per sample) of word-level token lists used
  to build the input. Pass these to
  [*~Qwen3ASRProcessor.decode_forced_alignment*] to pair timestamps
  with words.

Prepare inputs for the forced aligner model.

#### decode_forced_alignment[[transformers.Qwen3ASRProcessor.decode_forced_alignment]]

```python
decode_forced_alignment(logits, input_ids, word_lists: list, timestamp_token_id: int, timestamp_segment_time: float | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen3_asr/processing_qwen3_asr.py#L760)

**Parameters:**

logits (*torch.Tensor* of shape *(batch_size, seq_len, num_labels)*) : Classification logits from [*Qwen3ASRForTokenClassification*].

input_ids (*torch.LongTensor* of shape *(batch_size, seq_len)*) : Input token IDs used for the forward pass.

word_lists (*list[list[str]]*) : Word-level token lists as returned by [*~Qwen3ASRProcessor.prepare_forced_aligner_inputs*].

timestamp_token_id (*int*) : Token ID of the `&amp;lt;timestamp>` marker (from `model.config.timestamp_token_id`).

timestamp_segment_time (*float*, *optional*) : Milliseconds per timestamp class. If not provided, uses *self.timestamp_segment_time*.

**Returns:** *list[list[dict]]*

One list per sample.  Each inner list contains dicts
with keys `"text"` (*str*), `"start_time"` (*float*, seconds), and
`"end_time"` (*float*, seconds).

Decode forced aligner model outputs into word-level timestamps.

#### decode[[transformers.Qwen3ASRProcessor.decode]]

```python
decode(*args, return_format = 'raw', **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen3_asr/processing_qwen3_asr.py#L555)

**Parameters:**

return_format (*str*, *optional*, defaults to *"raw"*) : Options:  - `"raw"`: Return raw decoded strings from the tokenizer. - `"parsed"`: Return a dict (or list of dicts) with `"language"` and `"transcription"` keys. - `"transcription_only"`: Extract only the transcribed text (after `&amp;lt;asr_text>`).  `skip_special_tokens` is hard-set to `True` for `"parsed"` and `"transcription_only"`.

Forward arguments to the tokenizer's decode and optionally parse the ASR output.

Qwen3 ASR outputs transcription in the format: `language &amp;lt;LANG>&amp;lt;asr_text>transcribed text`

## Qwen3ASREncoder[[transformers.Qwen3ASREncoder]]

#### transformers.Qwen3ASREncoder[[transformers.Qwen3ASREncoder]]

```python
transformers.Qwen3ASREncoder(config: Qwen3ASREncoderConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen3_asr/modeling_qwen3_asr.py#L323)

**Parameters:**

config ([Qwen3ASREncoderConfig](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASREncoderConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The audio model for Qwen3 ASR without any head or projection on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Qwen3ASREncoder.forward]]

```python
forward(input_features: Tensor, input_features_mask: Tensor, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen3_asr/modeling_qwen3_asr.py#L367)

**Parameters:**

input_features (`torch.Tensor` of shape `(batch_size, sequence_length, feature_dim)`) : The tensors corresponding to the input audio features. Audio features can be obtained using [Qwen3ASRFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRFeatureExtractor). See [Qwen3ASRFeatureExtractor.__call__()](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRFeatureExtractor.__call__) for details ([Qwen3ASRProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRProcessor) uses [Qwen3ASRFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRFeatureExtractor) for processing audios).

input_features_mask (`torch.LongTensor` of shape `(batch_size, padded_feature_length)`) : 1 for valid mel frames and 0 for padding.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen3ASRConfig](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRConfig)) and inputs.

The [Qwen3ASREncoder](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASREncoder) forward method, overrides the `__call__` special method.

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

## Qwen3ASRModel[[transformers.Qwen3ASRModel]]

#### transformers.Qwen3ASRModel[[transformers.Qwen3ASRModel]]

```python
transformers.Qwen3ASRModel(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen3_asr/modeling_qwen3_asr.py#L469)

**Parameters:**

config ([Qwen3ASRModel](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRModel)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Qwen3ASR model (fine-tuned Whisper encoder, multi-modal projector, Qwen2 language model),
without a language modeling head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Qwen3ASRModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, input_features: typing.Optional[torch.FloatTensor] = None, input_features_mask: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen3_asr/modeling_qwen3_asr.py#L527)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) : The tensors corresponding to the input audio features. Audio features can be obtained using [Qwen3ASRFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRFeatureExtractor). See [Qwen3ASRFeatureExtractor.__call__()](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRFeatureExtractor.__call__) for details ([Qwen3ASRProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRProcessor) uses [Qwen3ASRFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRFeatureExtractor) for processing audios).

input_features_mask (`torch.Tensor` of shape `(batch_size, feature_sequence_length)`) : Mask to avoid performing attention on padding feature indices.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** `Qwen3ASRModelOutputWithPast` or `tuple(torch.FloatTensor)`

A `Qwen3ASRModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen3ASRConfig](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRConfig)) and inputs.

The [Qwen3ASRModel](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **audio_hidden_states** (`torch.FloatTensor`, *optional*) -- Projected audio hidden states.

#### get_audio_features[[transformers.Qwen3ASRModel.get_audio_features]]

```python
get_audio_features(input_features: FloatTensor, input_features_mask: LongTensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen3_asr/modeling_qwen3_asr.py#L481)

**Parameters:**

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`) : The tensors corresponding to the input audio features. Audio features can be obtained using [Qwen3ASRFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRFeatureExtractor). See [Qwen3ASRFeatureExtractor.__call__()](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRFeatureExtractor.__call__) for details ([Qwen3ASRProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRProcessor) uses [Qwen3ASRFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRFeatureExtractor) for processing audios).

input_features_mask (`torch.LongTensor` of shape `(batch_size, padded_feature_length)`) : 1 for valid mel frames and 0 for padding.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen3ASRConfig](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRConfig)) and inputs.

This method is used to get the audio embeddings from input features (a log mel spectrogram).

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

#### get_placeholder_mask[[transformers.Qwen3ASRModel.get_placeholder_mask]]

```python
get_placeholder_mask(input_ids: LongTensor, inputs_embeds: FloatTensor, audio_features: FloatTensor)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen3_asr/modeling_qwen3_asr.py#L503)

Obtains multimodal placeholder mask from `input_ids` or `inputs_embeds`, and checks that the placeholder token count is
equal to the length of multimodal features. If the lengths are different, an error is raised.

## Qwen3ASRForConditionalGeneration[[transformers.Qwen3ASRForConditionalGeneration]]

#### transformers.Qwen3ASRForConditionalGeneration[[transformers.Qwen3ASRForConditionalGeneration]]

```python
transformers.Qwen3ASRForConditionalGeneration(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen3_asr/modeling_qwen3_asr.py#L609)

**Parameters:**

config ([Qwen3ASRForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRForConditionalGeneration)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Qwen3ASR model which consists of an audio encoder and a language model.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Qwen3ASRForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, input_features: typing.Optional[torch.FloatTensor] = None, input_features_mask: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen3_asr/modeling_qwen3_asr.py#L621)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) : The tensors corresponding to the input audio features. Audio features can be obtained using [Qwen3ASRFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRFeatureExtractor). See [Qwen3ASRFeatureExtractor.__call__()](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRFeatureExtractor.__call__) for details ([Qwen3ASRProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRProcessor) uses [Qwen3ASRFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRFeatureExtractor) for processing audios).

input_features_mask (`torch.Tensor` of shape `(batch_size, feature_sequence_length)`) : Mask to avoid performing attention on padding feature indices.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** `Qwen3ASRCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`

A `Qwen3ASRCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen3ASRConfig](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRConfig)) and inputs.

The [Qwen3ASRForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/qwen3_asr#transformers.Qwen3ASRForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance.
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
>>> from transformers import Qwen3ASRForConditionalGeneration, AutoProcessor

>>> model_id = "Qwen/Qwen3-ASR-1.7B-hf"
>>> processor = AutoProcessor.from_pretrained(model_id)
>>> model = Qwen3ASRForConditionalGeneration.from_pretrained(model_id, device_map="auto")
```

#### get_audio_features[[transformers.Qwen3ASRForConditionalGeneration.get_audio_features]]

```python
get_audio_features(input_features, input_features_mask, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen3_asr/modeling_qwen3_asr.py#L618)

## Qwen3ASRForTokenClassification[[transformers.Qwen3ASRForTokenClassification]]

#### transformers.Qwen3ASRForTokenClassification[[transformers.Qwen3ASRForTokenClassification]]

```python
transformers.Qwen3ASRForTokenClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen3_asr/modeling_qwen3_asr.py#L689)

**Parameters:**

config ([``GenericForTokenClassification``]) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Qwen3 ASR model with a token classification head for timestamp prediction (forced alignment).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Qwen3ASRForTokenClassification.forward]]

```python
forward(input_ids: torch.LongTensor | None = None, attention_mask: torch.Tensor | None = None, position_ids: torch.LongTensor | None = None, past_key_values: Cache | None = None, inputs_embeds: torch.FloatTensor | None = None, labels: torch.LongTensor | None = None, use_cache: bool | None = None, **kwargs: Unpack[TransformersKwargs])
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_layers.py#L278)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** `TokenClassifierOutput`

The `GenericForTokenClassification` forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.
