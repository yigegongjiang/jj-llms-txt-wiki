# VibeVoice

## Overview

[VibeVoice](https://huggingface.co/papers/2508.19205) is a novel framework for synthesizing high-fidelity, long-form speech with multiple speakers by employing a next-token diffusion approach within a Large Language Model (LLM) structure. It's designed to capture the authentic conversational "vibe" and is particularly suited for generating audio content like podcasts and multi-participant audiobooks.

Two model checkpoints are available at:
- [vibevoice/VibeVoice-1.5B-hf](https://huggingface.co/vibevoice/VibeVoice-1.5B-hf)
- [vibevoice/VibeVoice-7B-hf](https://huggingface.co/vibevoice/VibeVoice-7B-hf)

This model was contributed by [Eric Bezzam](https://huggingface.co/bezzam).

## Architecture

    

The VibeVoice framework integrates three key components:
1. **Continuous Speech Tokenizers:** Specialized [acoustic](./vibevoice_acoustic_tokenizer) and [semantic](./vibevoice_semantic_tokenizer) tokenizers, where the acoustic tokenizer uses a $\sigma$-VAE to achieve ultra-low compression (7.5 tokens/sec, 3200x) for scalability and fidelity, and the semantic tokenizer uses an ASR proxy task for content-centric feature extraction.
2. **Large Language Model (LLM):** Uses Qwen2.5 (in 1.5B and 7B versions) as its core sequence model.
3. **Token-Level Diffusion Head:** conditioned on the LLM's hidden state and responsible for predicting the continuous VAE features in a streaming fashion.

The original VibeVoice-1.5B checkpoint is available under the [Microsoft](https://huggingface.co/microsoft/VibeVoice-1.5B) organization on Hugging Face.

## Key Features

- **Long-Form Synthesis**: Can synthesize up to 90 minutes multi-speaker conversational speech.
- **Multi-Speaker Dialogue**: Capable of synthesizing audio with a maximum of 4 speakers.
- **State-of-the-Art Quality**: Outperforms baselines on both subjective and objective metrics.
- **High Compression**: Achieved by a novel acoustic tokenizer operating at an ultra-low 7.5 Hz frame rate.
- **Scalable LLM**: Scaling the core LLM from 1.5B to 7B significantly improves perceptual quality.

## Usage

### Setup 

A noise scheduler is needed as audio generation relies on a diffusion process. The easiest approach (and as done by the model developers) is to use a noise scheduler from the `diffusers` library. By default, the model will create a noise scheduler with `diffusers` internally.
```
pip install diffusers
pip install soundfile   # for saving audio
```

### Loading the model

```python
from transformers import AutoProcessor, AutoModelForTextToWaveform

model_id = "vibevoice/VibeVoice-1.5B-hf"  # "vibevoice/VibeVoice-7B-hf"
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForTextToWaveform.from_pretrained(model_id)
```

### Text-to-speech (TTS)

```python
import os
from transformers import AutoProcessor, AutoModelForTextToWaveform

model_id = "vibevoice/VibeVoice-1.5B-hf"   # "vibevoice/VibeVoice-7B-hf"
text = "Hello, nice to meet you. How are you?"

# Load model
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForTextToWaveform.from_pretrained(model_id, device_map="auto")

# Prepare input
conversation = [{"role": "0", "content": [{"type": "text", "text": text}]}]
inputs = processor.apply_chat_template(
    conversation, return_dict=True, tokenize=True, add_generation_prompt=True,
).to(model.device, model.dtype)

# Generate!
audio = model.generate(**inputs)

# Save to file
file_name = f"{os.path.basename(model_id)}_tts.wav"
processor.save_audio(audio, file_name)
print(f"Saved output to {file_name}")
```

### TTS voice cloning

A voice can be cloned by providing a reference audio alongside the text within the chat template dictionary.

```python
import os
from transformers import AutoProcessor, AutoModelForTextToWaveform, set_seed

model_id = "vibevoice/VibeVoice-1.5B-hf"   # "vibevoice/VibeVoice-7B-hf"
text = "Hello, nice to meet you. How are you?"
set_seed(42)  # for deterministic results

# Load model
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForTextToWaveform.from_pretrained(model_id, device_map="auto")
sampling_rate = processor.feature_extractor.sampling_rate

# Prepare input
conversation = [
    {
        "role": "0",
        "content": [
            {"type": "text", "text": text},
            {
                "type": "audio",
                "url": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/voices/en-Alice_woman.wav",
            },
        ],
    }
]
inputs = processor.apply_chat_template(
    conversation, return_dict=True, tokenize=True, add_generation_prompt=True,
).to(model.device, model.dtype)

# Generate!
audio = model.generate(**inputs)

# Save to file
fn = f"{os.path.basename(model_id)}_tts_clone.wav"
processor.save_audio(audio, fn)
print(f"Saved output to {fn}")
```

### Generating a podcast from a script

Below is an example to generate a conversation between two speakers, whose voices are cloned by providing a reference audio for each unique role ID in the chat template.

The example below also used the `monitor_progress` option to track the generation progress.

```python
import os
import time
from transformers import AutoProcessor, AutoModelForTextToWaveform

model_id = "vibevoice/VibeVoice-1.5B-hf"   # "vibevoice/VibeVoice-7B-hf"
max_new_tokens = 400  # `None` to ensure full generation

# create conversation with an audio for the first time a speaker appears to clone that particular voice
conversation = [
    {
        "role": "0",
        "content": [
            {
                "type": "text", "text": "Hello everyone, and welcome to the VibeVoice podcast. I'm your host, Linda, and today we're getting into one of the biggest debates in all of sports: who's the greatest basketball player of all time? I'm so excited to have Thomas here to talk about it with me.",
            },
            {
                "type": "audio", "url": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/voices/en-Alice_woman.wav",
            },
        ],
    },
    {
        "role": "1",
        "content": [
            {
                "type": "text", "text": "Thanks so much for having me, Linda. You're absolutely right—this question always brings out some seriously strong feelings.",
            },
            {
                "type": "audio", "url": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/voices/en-Frank_man.wav",
            },
        ],
    },
    {
        "role": "0",
        "content": [
            {
                "type": "text", "text": "Okay, so let's get right into it. For me, it has to be Michael Jordan. Six trips to the Finals, six championships. That kind of perfection is just incredible.",
            },
        ],
    },
    {
        "role": "1",
        "content": [
            {
                "type": "text", "text": "Oh man, the first thing that always pops into my head is that shot against the Cleveland Cavaliers back in '89. Jordan just rises, hangs in the air forever, and just sinks it",
            },
        ],
    },
]

# Load model
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForTextToWaveform.from_pretrained(model_id, device_map="auto")

# prepare inputs
inputs = processor.apply_chat_template(
    conversation, return_dict=True, tokenize=True, add_generation_prompt=True,
).to(model.device, model.dtype)

# Generate audio with a progress bar to track generation
model.generation_config.max_new_tokens = max_new_tokens
start_time = time.time()
audio = model.generate(**inputs, monitor_progress=True)
generation_time = time.time() - start_time
print(f"Generation time: {generation_time:.2f} seconds")

# Save audio
fn = f"{os.path.basename(model_id)}_script.wav"
processor.save_audio(audio, fn)
print(f"Saved output to {fn}")
```

### Batched inference

For batch processing, a list of conversations can be passed to `processor.apply_chat_template`: 

```python
import os
import time
from transformers import AutoProcessor, AutoModelForTextToWaveform

model_id = "vibevoice/VibeVoice-1.5B-hf"   # "vibevoice/VibeVoice-7B-hf"
max_new_tokens = 400  # `None` to ensure full generation

conversation = [
    [
        {
            "role": "0",
            "content": [
                {
                    "type": "text", "text": "Hello everyone, and welcome to the VibeVoice podcast. I'm your host, Linda, and today we're getting into one of the biggest debates in all of sports: who's the greatest basketball player of all time? I'm so excited to have Thomas here to talk about it with me.",
                },
                {
                    "type": "audio",
                    "url": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/voices/en-Alice_woman.wav",
                },
            ],
        },
        {
            "role": "1",
            "content": [
                {
                    "type": "text", "text": "Thanks so much for having me, Linda.",
                },
                {
                    "type": "audio", "url": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/voices/en-Frank_man.wav",
                },
            ],
        },
    ],
    [
        {
            "role": "0",
            "content": [
                {
                    "type": "text", "text": "Hello and welcome to Planet in Peril. I'm your host, Alice. We're here today to discuss a really sobering new report that looks back at the last ten years of climate change. I'm joined by our expert panel. Welcome Carter, Frank, and Maya.",
                },
                {
                    "type": "audio", "url": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/voices/en-Alice_woman.wav",
                },
            ],
        },
        {
            "role": "1",
            "content": [
                {"type": "text", "text": "Hi Alice, it's great to be here. I'm Carter."},
                {
                    "type": "audio", "url": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/voices/en-Carter_man.wav",
                },
            ],
        },
        {
            "role": "2",
            "content": [
                {"type": "text", "text": "Hello, uh, I'm Frank. Good to be on."},
                {
                    "type": "audio", "url": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/voices/en-Frank_man.wav",
                },
            ],
        },
        {
            "role": "3",
            "content": [
                {"type": "text", "text": "And I'm Maya. Thanks for having me."},
                {
                    "type": "audio", "url": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/voices/en-Maya_woman.wav",
                },
            ],
        },
    ],
]

# Load model
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForTextToWaveform.from_pretrained(model_id, device_map="auto")

# prepare inputs
inputs = processor.apply_chat_template(
    conversation, return_dict=True, tokenize=True, add_generation_prompt=True,
).to(model.device, model.dtype)

# Generate audio with a progress bar to track generation
model.generation_config.max_new_tokens = max_new_tokens
start_time = time.time()
audio = model.generate(**inputs, monitor_progress=True)
generation_time = time.time() - start_time
print(f"Generation time: {generation_time:.2f} seconds")

# Save audio
output_dir = f"{os.path.basename(model_id)}_batch"
processor.save_audio(audio, output_dir)
print(f"Saved output to {output_dir}")
```

### Pipeline usage

VibeVoice can also be loaded as a pipeline. We also show below how the diffusion parameters can be adjusted.

```python
import os
import soundfile as sf
from transformers import pipeline

model_id = "vibevoice/VibeVoice-1.5B-hf"   # "vibevoice/VibeVoice-7B-hf"
text = "Hello, nice to meet you. How are you?"
pipe = pipeline("text-to-speech", model=model_id)

# Generate!
conversation = [
    {
        "role": "0",
        "content": [
            {"type": "text", "text": text},
            {
                "type": "audio", "url": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/voices/en-Alice_woman.wav",
            },
        ],
    }
]
# optional kwargs for generation
generate_kwargs = {"guidance_scale": 1.3, "num_diffusion_steps": 10}
output = pipe(conversation, generate_kwargs=generate_kwargs)

# Save to file
fn = f"{os.path.basename(model_id)}_pipeline.wav"
sf.write(fn, output["audio"], output["sampling_rate"])
print(f"Saved output to {fn}")
```

### Training

VibeVoice can be trained with the loss outputted by the model.

```python
from transformers import AutoProcessor, AutoModelForTextToWaveform

model_id = "vibevoice/VibeVoice-1.5B-hf"   # "vibevoice/VibeVoice-7B-hf"

# Load model and processor
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForTextToWaveform.from_pretrained(
    model_id,
    diffusion_loss_weight=0.75,  # by default, equal weighting (0.5) of language modeling loss (CE) and diffusion loss is applied
    device_map="auto"
)
model.train()

# Prepare batch of 2
conversation = [
    [
        {
            "role": "0",
            "content": [
                {
                    "type": "text", "text": "VibeVoice is this novel framework designed for generating expressive, long-form, multi-speaker, conversational audio.",
                },
                {
                    "type": "audio", "url": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/realtime_model/vibevoice_tts_german.wav",
                },
            ],
        }
    ],
    # NOTE: multiple speakers not supported yet
    [
        {
            "role": "0",
            "content": [
                {
                    "type": "text", "text": "Hello everyone and welcome to the VibeVoice podcast. I'm your host, Alex, and today we're getting into one of the biggest debates in all of sports: who's the greatest basketball player of all time? I'm so excited to have Sam here to talk about it with me. Thanks so much for having me, Alex. And you're absolutely right. This question always brings out some seriously strong feelings. Okay, so let's get right into it. For me, it has to be Michael Jordan. Six trips to the finals, six championships. That kind of perfection is just incredible. Oh man, the first thing that always pops into my head is that shot against the Cleveland Cavaliers back in '89. Jordan just rises, hangs in the air forever, and just sinks it.",
                },
                {
                    "type": "audio",
                    "url": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/example_output/VibeVoice-1.5B_output.wav",
                },
            ],
        }
    ],
]

# Process with apply_chat_template and output_labels=True for training
inputs = processor.apply_chat_template(
    conversation,
    tokenize=True,
    return_dict=True,
    processor_kwargs={"output_labels": True},
).to(model.device, model.dtype)

# Forward pass
outputs = model(**inputs, ddpm_batch_multiplier=2, num_diffusion_steps=2)
print(f"Total loss: {outputs.loss.item():.4f}")

# Backward pass
outputs.loss.backward()
```

### Torch compile

The model can be compiled with `torch.compile` for faster inference. A few warmup runs are needed before the compiled model reaches full speed.

On an A100 with batch size 4, we observed a ~1.5x speed-up between compiled vs. non-compiled inference, see [this script](https://gist.github.com/ebezzam/c45b9fdee65f3029e17d566e30c59399).

```python
import os
import time
import torch
from transformers import AutoModelForTextToWaveform, AutoProcessor, CompileConfig

model_id = "vibevoice/VibeVoice-1.5B-hf"   # "vibevoice/VibeVoice-7B-hf"
num_warmup = 5
max_new_tokens = 128

torch.set_float32_matmul_precision("high")

# Load processor + model
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForTextToWaveform.from_pretrained(model_id, dtype=torch.bfloat16, device_map="auto").eval()

# Prepare inputs
conversation = [
    [
        {
            "role": "0",
            "content": [
                {"type": "text", "text": "VibeVoice is a novel framework for generating expressive audio."},
                {
                    "type": "audio", "url": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/realtime_model/vibevoice_tts_german.wav",
                },
            ],
        }
    ],
] * 4  # batch size 4
inputs = processor.apply_chat_template(
    conversation, tokenize=True, return_dict=True, add_generation_prompt=True,
).to(model.device, model.dtype)

compile_config = CompileConfig(mode="default", dynamic=False)

generate_kwargs = dict(
    **inputs,
    max_new_tokens=max_new_tokens,
    cache_implementation="static",
    compile_config=compile_config,
)

# Warmup
print("Warming up...")
warmup_start = time.time()
with torch.inference_mode():
    for _ in range(num_warmup):
        torch.compiler.cudagraph_mark_step_begin()
        _ = model.generate(**generate_kwargs)
torch.cuda.synchronize()
print(f"Warmup complete in {time.time() - warmup_start:.2f}s. Ready!")

# Apply model
with torch.inference_mode():
    torch.compiler.cudagraph_mark_step_begin()
    audio = model.generate(**generate_kwargs)
output_folder = f"{os.path.basename(model_id)}_compiled_output"
processor.save_audio(audio, output_folder)
print(f"Saved output to {output_folder}")
```

## VibeVoiceConfig[[transformers.VibeVoiceConfig]]

#### transformers.VibeVoiceConfig[[transformers.VibeVoiceConfig]]

```python
transformers.VibeVoiceConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, audio_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, semantic_model_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, diffusion_head_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, pad_token_id: int = 151643, eos_token_id: int = 151643, audio_bos_token_id: int = 151652, audio_eos_token_id: int = 151653, audio_token_id: int = 151654, diffusion_loss_weight: float = 0.5)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/vibevoice/configuration_vibevoice.py#L47)

**Parameters:**

audio_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the audio backbone.

semantic_model_config (`Union[AutoConfig, dict]`, *optional*) : The config object or dictionary of the semantic tokenizer encoder. This tokenizer extracts semantic features from audio.

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

diffusion_head_config (`Union[VibeVoiceDiffusionHeadConfig, dict]`, *optional*) : The config object or dictionary of the diffusion head used to synthesize acoustic latents.

pad_token_id (`int`, *optional*, defaults to `151643`) : Token id used for padding in the vocabulary.

eos_token_id (`int`, *optional*, defaults to `151643`) : Token id used for end-of-stream in the vocabulary.

audio_bos_token_id (`int`, *optional*, defaults to 151652) : The token ID indicating the start of audio tokens.

audio_eos_token_id (`int`, *optional*, defaults to 151653) : The token ID indicating the end of audio tokens.

audio_token_id (`int`, *optional*, defaults to `151654`) : The audio token index used as a placeholder for input audio.

diffusion_loss_weight (`float`, *optional*, defaults to 0.5) : The weight of the diffusion loss in the overall loss computation. The cross entropy loss for the language modeling head is weighted by `(1 - diffusion_loss_weight)`.

This is the configuration class to store the configuration of a VibeVoiceForConditionalGeneration. It is used to instantiate a Vibevoice
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [vibevoice/VibeVoice-1.5B-hf](https://huggingface.co/vibevoice/VibeVoice-1.5B-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import VibeVoiceForConditionalGeneration, VibeVoiceConfig

>>> # Initializing a VibeVoice configuration
>>> configuration = VibeVoiceConfig()

>>> # Initializing a 1.5B model with random weights
>>> model = VibeVoiceForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## VibeVoiceDiffusionHeadConfig[[transformers.VibeVoiceDiffusionHeadConfig]]

#### transformers.VibeVoiceDiffusionHeadConfig[[transformers.VibeVoiceDiffusionHeadConfig]]

```python
transformers.VibeVoiceDiffusionHeadConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 1536, latent_size: int = 64, num_hidden_layers: int = 4, intermediate_size: int = 4608, rms_norm_eps: float = 1e-05, hidden_act: str = 'silu', frequency_embedding_size: int = 256, diffusion_max_period: int = 10000, mlp_bias: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/vibevoice/configuration_vibevoice.py#L24)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `1536`) : Dimension of the hidden representations.

latent_size (`int`, *optional*, defaults to 64) : Dimensionality of the acoustic latents the head denoises.

num_hidden_layers (`int`, *optional*, defaults to `4`) : Number of hidden layers in the Transformer decoder.

intermediate_size (`int`, *optional*, defaults to `4608`) : Dimension of the MLP representations.

rms_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the rms normalization layers.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

frequency_embedding_size (`int`, *optional*, defaults to 256) : The size of the sinusoidal frequency embedding for timestep encoding in the diffusion head.

diffusion_max_period (`int`, *optional*, defaults to 10000) : The maximum period for the sinusoidal frequency embedding in the diffusion head.

mlp_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in up_proj, down_proj and gate_proj layers in the MLP layers.

This is the configuration class to store the configuration of a VibeVoiceForConditionalGeneration. It is used to instantiate a Vibevoice
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [vibevoice/VibeVoice-1.5B-hf](https://huggingface.co/vibevoice/VibeVoice-1.5B-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## VibeVoiceProcessor[[transformers.VibeVoiceProcessor]]

#### transformers.VibeVoiceProcessor[[transformers.VibeVoiceProcessor]]

```python
transformers.VibeVoiceProcessor(feature_extractor, tokenizer, chat_template = None, audio_bos_token = '<|vision_start|>', audio_eos_token = '<|vision_end|>', audio_token = '<|vision_pad|>')
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/vibevoice/processing_vibevoice.py#L46)

**Parameters:**

feature_extractor (`VibeVoiceAcousticTokenizerFeatureExtractor`) : The feature extractor is a required input.

tokenizer (`Qwen2TokenizerFast`) : The tokenizer is a required input.

chat_template (`str`) : A Jinja template to convert lists of messages in a chat into a tokenizable string.

audio_bos_token (`str`, *optional*, defaults to `"<|vision_start|>"`) : The token used to indicate the beginning of audio generation.

audio_eos_token (`str`, *optional*, defaults to `"<|vision_end|>"`) : The token used to indicate the end of audio generation.

audio_token (`str`, *optional*, defaults to `"<|vision_pad|>"`) : The token used to indicate to continue generating audio.

Constructs a VibeVoiceProcessor which wraps a feature extractor and a tokenizer into a single processor.

[VibeVoiceProcessor](/docs/transformers/v5.17.0/en/model_doc/vibevoice#transformers.VibeVoiceProcessor) offers all the functionalities of [VibeVoiceAcousticTokenizerFeatureExtractor](/docs/transformers/v5.17.0/en/model_doc/vibevoice_acoustic_tokenizer#transformers.VibeVoiceAcousticTokenizerFeatureExtractor) and [Qwen2TokenizerFast](/docs/transformers/v5.17.0/en/model_doc/qwen2#transformers.Qwen2Tokenizer). See the
[~VibeVoiceAcousticTokenizerFeatureExtractor](/docs/transformers/v5.17.0/en/model_doc/vibevoice_acoustic_tokenizer#transformers.VibeVoiceAcousticTokenizerFeatureExtractor) and [~Qwen2TokenizerFast](/docs/transformers/v5.17.0/en/model_doc/qwen2#transformers.Qwen2Tokenizer) for more information.

#### __call__[[transformers.VibeVoiceProcessor.__call__]]

```python
__call__(text: str | list[str] | list[list[str]], audio: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor'], NoneType] = None, output_labels: bool | None = False, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/vibevoice/processing_vibevoice.py#L98)

**Parameters:**

text (`str`, `List[str]`) : The input text(s) to process, typically prepared by apply_chat_template with audio token placeholders.

audio (`List[Union[str, np.ndarray]]`, *optional*) : Audio samples for speaker voice cloning. Should match the number of audio token placeholders in text.

output_labels (`bool`, *optional*, default=False) : Whether to return labels for training.

- ****kwargs** : Additional keyword arguments passed to the tokenizer and feature extractor.

return_tensors (`str` or [TensorType](/docs/transformers/v5.17.0/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.17.0/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) : Additional processing options for each modality (text, images, videos, audio). Model-specific parameters are listed above; see the TypedDict class for the complete list of supported arguments.

**Returns:** [BatchFeature](/docs/transformers/v5.17.0/en/main_classes/image_processor#transformers.BatchFeature)

A [BatchFeature](/docs/transformers/v5.17.0/en/main_classes/image_processor#transformers.BatchFeature) with the following fields:
- **input_ids** -- List of token ids to be fed to the model.
- **attention_mask** -- List of indices specifying which tokens should be attended to by the model (when
  `return_attention_mask=True`).
- **input_values** -- List of audio values to be fed to the model. Returned when `audio` is not `None`.
- **padding_mask** -- List of indices specifying which audio frames should be attended to by the model.
  Returned when `audio` is not `None`.
- **labels** -- Labels for language model training. Padding and audio diffusion tokens are masked with
  -100 (audio diffusion token embeddings are replaced by audio features, so their token identity is not a
  meaningful target). Audio bos/eos and text eos tokens are kept as targets so the LM learns when to start
  and stop generating audio. Returned when `output_labels=True`.
- **acoustic_loss_mask** -- Boolean mask for positions where diffusion loss is computed. True at audio
  diffusion token positions. Returned when `output_labels=True`.

## VibeVoiceForConditionalGeneration[[transformers.VibeVoiceForConditionalGeneration]]

#### transformers.VibeVoiceForConditionalGeneration[[transformers.VibeVoiceForConditionalGeneration]]

```python
transformers.VibeVoiceForConditionalGeneration(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/vibevoice/modeling_vibevoice.py#L354)

**Parameters:**

config ([VibeVoiceForConditionalGeneration](/docs/transformers/v5.17.0/en/model_doc/vibevoice#transformers.VibeVoiceForConditionalGeneration)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The VibeVoice model, which consists of a language model, audio tokenizers, connectors, and a diffusion head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.VibeVoiceForConditionalGeneration.forward]]

```python
forward(input_ids: LongTensor = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, logits_to_keep: int | slice = 0, input_values: typing.Optional[torch.FloatTensor] = None, padding_mask: typing.Optional[torch.BoolTensor] = None, acoustic_loss_mask: typing.Optional[torch.BoolTensor] = None, noise_scheduler: object | None = None, ddpm_batch_multiplier: int = 4, num_diffusion_steps: int = 10, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/vibevoice/modeling_vibevoice.py#L409)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.17.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

logits_to_keep (`Union[int, slice]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

input_values (`torch.FloatTensor`, *optional*) : Preprocessed audio waveform for voice cloning.

padding_mask (`torch.BoolTensor`, *optional*) : Masks indicating valid input frames.

acoustic_loss_mask (`torch.BoolTensor`, *optional*) : Mask to compute diffusion loss only on specific acoustic tokens.

noise_scheduler (`object`, *optional*) : Needed for training to compute noise targets for the diffusion loss. By default, uses the noise scheduler configuration specified in the model's `generation_config`.

ddpm_batch_multiplier (`int`, *optional*, defaults to 4) : For training, number of noise samples to generate per audio token for diffusion loss computation, which can help stabilize training.

num_diffusion_steps (`int`, *optional*, defaults to 10) : For training, the number of diffusion steps to use. Defaults to 10 if not provided.

**Returns:** `VibeVoiceCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`

A `VibeVoiceCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([VibeVoiceConfig](/docs/transformers/v5.17.0/en/model_doc/vibevoice#transformers.VibeVoiceConfig)) and inputs.

The [VibeVoiceForConditionalGeneration](/docs/transformers/v5.17.0/en/model_doc/vibevoice#transformers.VibeVoiceForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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
- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **audio_features** (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_size)`, *optional*) -- Extracted audio features that can be used for conditioning the language model.

Example:

```python
>>> from transformers import AutoProcessor, VibeVoiceForConditionalGeneration

>>> model = VibeVoiceForConditionalGeneration.from_pretrained("vibevoice/VibeVoice-1.5B-hf")

>>> processor = AutoProcessor.from_pretrained("vibevoice/VibeVoice-1.5B-hf")
>>> inputs = processor(text="Hello, my dog is cute", return_tensors="pt")

>>> # generate speech
>>> speech = model(inputs["input_ids"])
```

#### generate[[transformers.VibeVoiceForConditionalGeneration.generate]]

```python
generate(inputs: typing.Optional[torch.Tensor] = None, generation_config: transformers.generation.configuration_utils.GenerationConfig | None = None, logits_processor: transformers.generation.logits_process.LogitsProcessorList | None = None, stopping_criteria: transformers.generation.stopping_criteria.StoppingCriteriaList | None = None, prefix_allowed_tokens_fn: collections.abc.Callable[[int, torch.Tensor], list[int]] | None = None, synced_gpus: bool | None = None, assistant_model: typing.Optional[ForwardRef('PreTrainedModel')] = None, streamer: typing.Optional[ForwardRef('BaseStreamer')] = None, negative_prompt_ids: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, custom_generate: str | collections.abc.Callable | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/generation/utils.py#L2394)

**Parameters:**

inputs (`torch.Tensor` of varying shape depending on the modality, *optional*) : The sequence used as a prompt for the generation or as model inputs to the encoder. If `None` the method initializes it with `bos_token_id` and a batch size of 1. For decoder-only models `inputs` should be in the format of `input_ids`. For encoder-decoder models *inputs* can represent any of `input_ids`, `input_values`, `input_features`, or `pixel_values`.

generation_config ([GenerationConfig](/docs/transformers/v5.17.0/en/main_classes/text_generation#transformers.GenerationConfig), *optional*) : The generation configuration to be used as base parametrization for the generation call. `**kwargs` passed to generate matching the attributes of `generation_config` will override them. If `generation_config` is not provided, the default will be used, which has the following loading priority: 1) from the `generation_config.json` model file, if it exists; 2) from the model configuration. Please note that unspecified parameters will inherit [GenerationConfig](/docs/transformers/v5.17.0/en/main_classes/text_generation#transformers.GenerationConfig)'s default values, whose documentation should be checked to parameterize generation.

logits_processor (`LogitsProcessorList`, *optional*) : Custom logits processors that complement the default logits processors built from arguments and generation config. If a logit processor is passed that is already created with the arguments or a generation config an error is thrown. This feature is intended for advanced users.

stopping_criteria (`StoppingCriteriaList`, *optional*) : Custom stopping criteria that complements the default stopping criteria built from arguments and a generation config. If a stopping criteria is passed that is already created with the arguments or a generation config an error is thrown. If your stopping criteria depends on the `scores` input, make sure you pass `return_dict_in_generate=True, output_scores=True` to `generate`. This feature is intended for advanced users.

prefix_allowed_tokens_fn (`Callable[[int, torch.Tensor], list[int]]`, *optional*) : If provided, this function constraints the beam search to allowed tokens only at each step. If not provided no constraint is applied. This function takes 2 arguments: the batch ID `batch_id` and `input_ids`. It has to return a list with the allowed tokens for the next generation step conditioned on the batch ID `batch_id` and the previously generated tokens `inputs_ids`. This argument is useful for constrained generation conditioned on the prefix, as described in [Autoregressive Entity Retrieval](https://huggingface.co/papers/2010.00904).

synced_gpus (`bool`, *optional*) : Whether to continue running the while loop until max_length. Unless overridden, this flag will be set to `True` if using `FullyShardedDataParallel` or DeepSpeed ZeRO Stage 3 with multiple GPUs to avoid deadlocking if one GPU finishes generating before other GPUs. Otherwise, defaults to `False`.

assistant_model (`PreTrainedModel`, *optional*) : An assistant model that can be used to accelerate generation. The assistant model must have the exact same tokenizer. The acceleration is achieved when forecasting candidate tokens with the assistant model is much faster than running generation with the model you're calling generate from. As such, the assistant model should be much smaller.

streamer (`BaseStreamer`, *optional*) : Streamer object that will be used to stream the generated sequences. Generated tokens are passed through `streamer.put(token_ids)` and the streamer is responsible for any further processing.

negative_prompt_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : The negative prompt needed for some processors such as CFG. The batch size must match the input batch size. This is an experimental feature, subject to breaking API changes in future versions.

negative_prompt_attention_mask (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Attention_mask for `negative_prompt_ids`.

custom_generate (`str` or `Callable`, *optional*) : One of the following: - `str` (Hugging Face Hub repository name): runs the custom `generate` function defined at `custom_generate/generate.py` in that repository instead of the standard `generate` method. The repository fully replaces the generation logic, and the return type may differ. - `str` (local repository path): same as above but from a local path. Local directories also require `trust_remote_code=True` because the local `custom_generate/generate.py` is executed. - `Callable`: `generate` will perform the usual input preparation steps, then call the provided callable to run the decoding loop. For more information, see [the docs](../../generation_strategies#custom-generation-methods).

kwargs (`dict[str, Any]`, *optional*) : Ad hoc parametrization of `generation_config` and/or additional model-specific kwargs that will be forwarded to the `forward` function of the model. If the model is an encoder-decoder model, encoder specific kwargs should not be prefixed and decoder specific kwargs should be prefixed with *decoder_*.

**Returns:** [ModelOutput](/docs/transformers/v5.17.0/en/main_classes/output#transformers.utils.ModelOutput) or `torch.LongTensor`

A [ModelOutput](/docs/transformers/v5.17.0/en/main_classes/output#transformers.utils.ModelOutput) (if `return_dict_in_generate=True`
or when `config.return_dict_in_generate=True`) or a `torch.LongTensor`.

If the model is *not* an encoder-decoder model (`model.config.is_encoder_decoder=False`), the possible
[ModelOutput](/docs/transformers/v5.17.0/en/main_classes/output#transformers.utils.ModelOutput) types are:

- [GenerateDecoderOnlyOutput](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.generation.GenerateDecoderOnlyOutput),
- [GenerateBeamDecoderOnlyOutput](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.generation.GenerateBeamDecoderOnlyOutput)

If the model is an encoder-decoder model (`model.config.is_encoder_decoder=True`), the possible
[ModelOutput](/docs/transformers/v5.17.0/en/main_classes/output#transformers.utils.ModelOutput) types are:

- [GenerateEncoderDecoderOutput](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.generation.GenerateEncoderDecoderOutput),
- [GenerateBeamEncoderDecoderOutput](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.generation.GenerateBeamEncoderDecoderOutput)

Generates sequences of token ids for models with a language modeling head.

Most generation-controlling parameters are set in `generation_config` which, if not passed, will be set to the
model's default generation configuration. You can override any `generation_config` by passing the corresponding
parameters to generate(), e.g. `.generate(inputs, num_beams=4, do_sample=True)`.

For an overview of generation strategies and code examples, check out the [following
guide](../generation_strategies).

## VibeVoiceModel[[transformers.VibeVoiceModel]]

#### transformers.VibeVoiceModel[[transformers.VibeVoiceModel]]

```python
transformers.VibeVoiceModel(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/vibevoice/modeling_vibevoice.py#L241)

**Parameters:**

config ([VibeVoiceModel](/docs/transformers/v5.17.0/en/model_doc/vibevoice#transformers.VibeVoiceModel)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The VibeVoice model which consists of audio tokenizers and an LLM backbone, without a language modeling head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.VibeVoiceModel.forward]]

```python
forward(input_ids: LongTensor = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, input_values: typing.Optional[torch.FloatTensor] = None, padding_mask: typing.Optional[torch.BoolTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/vibevoice/modeling_vibevoice.py#L316)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.17.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

input_values (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) : Float values of input raw speech waveform. Values can be obtained by loading a `.flac` or `.wav` audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_values`, the [AutoProcessor](/docs/transformers/v5.17.0/en/model_doc/auto#transformers.AutoProcessor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`. See [VibeVoiceProcessor.__call__()](/docs/transformers/v5.17.0/en/model_doc/vibevoice#transformers.VibeVoiceProcessor.__call__) for details.

padding_mask (`torch.Tensor` of shape `(batch_size, padded_audio_length)`) : Padding mask to remove padded parts of audio.

**Returns:** [BaseModelOutputWithPast](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPast](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([VibeVoiceConfig](/docs/transformers/v5.17.0/en/model_doc/vibevoice#transformers.VibeVoiceConfig)) and inputs.

The [VibeVoiceModel](/docs/transformers/v5.17.0/en/model_doc/vibevoice#transformers.VibeVoiceModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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

#### get_audio_features[[transformers.VibeVoiceModel.get_audio_features]]

```python
get_audio_features(input_values: FloatTensor, padding_mask: Tensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/vibevoice/modeling_vibevoice.py#L258)

**Parameters:**

input_values (`torch.FloatTensor`) : Float values of (normalized) audio waveform.

padding_mask (`torch.Tensor` of shape `(batch_size, padded_audio_length)`) : Padding mask to remove padded parts of audio.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([VibeVoiceConfig](/docs/transformers/v5.17.0/en/model_doc/vibevoice#transformers.VibeVoiceConfig)) and inputs.

This method is used to get the audio embeddings (that replace placeholder audio tokens in the input sequence) and the acoustic features (used as diffusion target) from the input audio waveform.

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

#### get_placeholder_mask[[transformers.VibeVoiceModel.get_placeholder_mask]]

```python
get_placeholder_mask(input_ids: LongTensor, inputs_embeds: FloatTensor, audio_features: FloatTensor)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/vibevoice/modeling_vibevoice.py#L292)

Obtains multimodal placeholder mask from `input_ids` or `inputs_embeds`, and checks that the placeholder token count is
equal to the length of multimodal features. If the lengths are different, an error is raised.
