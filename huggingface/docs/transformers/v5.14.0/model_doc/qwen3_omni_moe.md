# Qwen3-Omni-MOE

## Overview

The Qwen3-Omni-MOE model is a unified multiple modalities model proposed in [Qwen3-Omni Technical Report](https://huggingface.co/papers/2509.17765) from Qwen team, Alibaba Group.

The abstract from the technical report is the following:

*We present Qwen3-Omni, a single multimodal model that, for the first time, maintains state-of-the-art performance across text, image, audio, and video without any degradation relative to single-modal counterparts. Qwen3-Omni matches the performance of same-sized single-modal models within the Qwen series and excels particularly on audio tasks. Across 36 audio and audio-visual benchmarks, Qwen3-Omni achieves open-source SOTA on 32 benchmarks and overall SOTA on 22, outperforming strong closed-source models such as Gemini-2.5-Pro, Seed-ASR, and GPT-4o-Transcribe. Qwen3-Omni adopts a Thinker-Talker MoE architecture that unifies perception and generation across text, images, audio, and video, yielding fluent text and natural real-time speech. It supports text interaction in 119 languages, speech understanding in 19 languages, and speech generation in 10 languages. To reduce first-packet latency in streaming synthesis, Talker autoregressively predicts discrete speech codecs using a multi-codebook scheme. Leveraging the representational capacity of these codebooks, we replace computationally intensive block-wise diffusion with a lightweight causal ConvNet, enabling streaming from the first codec frame. In cold-start settings, Qwen3-Omni achieves a theoretical end-to-end first-packet latency of 234 ms. To further strengthen multimodal reasoning, we introduce a Thinking model that explicitly reasons over inputs from any modality. Since the research community currently lacks a general-purpose audio captioning model, we fine-tuned Qwen3-Omni-30B-A3B to obtain Qwen3-Omni-30B-A3B-Captioner, which produces detailed, low-hallucination captions for arbitrary audio inputs. Qwen3-Omni-30B-A3B, Qwen3-Omni-30B-A3B-Thinking, and Qwen3-Omni-30B-A3B-Captioner are publicly released under the Apache 2.0 license.

## Notes

- Use [Qwen3OmniMoeForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeForConditionalGeneration) to generate audio and text output. To generate only one output type, use [Qwen3OmniMoeThinkerForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeThinkerForConditionalGeneration) for text-only and [Qwen3OmniMoeTalkerForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeTalkerForConditionalGeneration) for audio-only outputs.
- Audio generation with [Qwen3OmniMoeForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeForConditionalGeneration) supports only single batch size at the moment.
- In case out out-of-memory errors hwen working with video input, decrease `processor.max_pixels`. By default the maximum is set to a very arge value and high resolution visuals will not be resized, unless resolution exceeds `processor.max_pixels`.
- The processor has its own [apply_chat_template()](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessorMixin.apply_chat_template) method to convert chat messages to model inputs.

## Usage example

`Qwen3-Omni` can be found on the [Huggingface Hub](https://huggingface.co/Qwen).

### Single Media inference

The model can accept text, images, audio and videos as input. Here's an example code for inference.

```python
import soundfile as sf

from transformers import Qwen3OmniMoeForConditionalGeneration, Qwen3OmniMoeProcessor

model = Qwen3OmniMoeForConditionalGeneration.from_pretrained(
    "Qwen/Qwen3-Omni-30B-A3B-Instruct",
    device_map="auto"
)
processor = Qwen3OmniMoeProcessor.from_pretrained("Qwen/Qwen3-Omni-30B-A3B-Instruct")

conversations = [
    {
        "role": "system",
        "content": [
            {"type": "text", "text": "You are Qwen, a virtual human developed by the Qwen Team, Alibaba Group, capable of perceiving auditory and visual inputs, as well as generating text and speech."}
        ],
    },
    {
        "role": "user",
        "content": [
            {"type": "video", "video": "/path/to/video.mp4"},
            {"type": "text", "text": "What cant you hear and see in this video?"},
        ],
    },
]

inputs = processor.apply_chat_template(
    conversations,
    load_audio_from_video=True,
    add_generation_prompt=True,
    tokenize=True,
    return_dict=True,
    return_tensors="pt",
    fps=1,

    # kwargs to be passed to `Qwen3OmniMoeProcessor`
    padding=True,
    use_audio_in_video=True,
).to(model.device)

# Generation params for audio or text can be different and have to be prefixed with `thinker_` or `talker_`
text_ids, audio = model.generate(**inputs, use_audio_in_video=True, thinker_do_sample=False, talker_do_sample=True)
text = processor.batch_decode(text_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)

sf.write(
    "output.wav",
    audio.reshape(-1).detach().cpu().numpy(),
    samplerate=24000,
)
print(text)
```

### Text-only generation

To generate only text output and save compute by not loading the audio generation model, we can use `Qwen3OmniMoeThinkerForConditionalGeneration` model.

```python
from transformers import Qwen3OmniMoeProcessor, Qwen3OmniMoeThinkerForConditionalGeneration

model = Qwen3OmniMoeThinkerForConditionalGeneration.from_pretrained(
    "Qwen/Qwen3-Omni-30B-A3B-Instruct",
    device_map="auto",
)
processor = Qwen3OmniMoeProcessor.from_pretrained("Qwen/Qwen3-Omni-30B-A3B-Instruct")

conversations = [
    {
        "role": "system",
        "content": [
            {"type": "text", "text": "You are Qwen, a virtual human developed by the Qwen Team, Alibaba Group, capable of perceiving auditory and visual inputs, as well as generating text and speech."}
        ],
    },
    {
        "role": "user",
        "content": [
            {"type": "video", "video": "/path/to/video.mp4"},
            {"type": "text", "text": "What cant you hear and see in this video?"},
        ],
    },
]

inputs = processor.apply_chat_template(
    conversations,
    load_audio_from_video=True,
    add_generation_prompt=True,
    tokenize=True,
    return_dict=True,
    return_tensors="pt",
    fps=1,

    # kwargs to be passed to `Qwen3OmniMoeProcessor`
    padding=True,
    use_audio_in_video=True,
).to(model.device)

text_ids = model.generate(**inputs, use_audio_in_video=True)
text = processor.batch_decode(text_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)

sf.write(
    "output.wav",
    audio.reshape(-1).detach().cpu().numpy(),
    samplerate=24000,
)
print(text)
```

### Batch Mixed Media Inference

The model can batch inputs composed of mixed samples of various types such as text, images, audio and videos as input when using `Qwen3OmniMoeThinkerForConditionalGeneration` model. Here is an example.

```python
import soundfile as sf
from transformers import Qwen3OmniMoeForConditionalGeneration, Qwen3OmniMoeProcessor

model = Qwen3OmniMoeForConditionalGeneration.from_pretrained(
    "Qwen/Qwen3-Omni-30B-A3B-Instruct"
    device_map="auto"
)
processor = Qwen3OmniMoeProcessor.from_pretrained("Qwen/Qwen3-Omni-30B-A3B-Instruct")

# Conversation with video only
conversation1 = [
    {
        "role": "system",
        "content": [
            {"type": "text", "text": "You are Qwen, a virtual human developed by the Qwen Team, Alibaba Group, capable of perceiving auditory and visual inputs, as well as generating text and speech."}
        ],
    },
    {
        "role": "user",
        "content": [
            {"type": "video", "path": "/path/to/video.mp4"},
        ]
    }
]

# Conversation with audio only
conversation2 = [
    {
        "role": "system",
        "content": [
            {"type": "text", "text": "You are Qwen, a virtual human developed by the Qwen Team, Alibaba Group, capable of perceiving auditory and visual inputs, as well as generating text and speech."}
        ],
    },
    {
        "role": "user",
        "content": [
            {"type": "audio", "path": "/path/to/audio.wav"},
        ]
    }
]

# Conversation with pure text
conversation3 = [
    {
        "role": "system",
        "content": [
            {"type": "text", "text": "You are Qwen, a virtual human developed by the Qwen Team, Alibaba Group, capable of perceiving auditory and visual inputs, as well as generating text and speech."}
        ],
    },
    {
        "role": "user",
        "content": [{"type": "text", "text": "who are you?"}],
    }
]

# Conversation with mixed media
conversation4 = [
    {
        "role": "system",
        "content": [
            {"type": "text", "text": "You are Qwen, a virtual human developed by the Qwen Team, Alibaba Group, capable of perceiving auditory and visual inputs, as well as generating text and speech."}
        ],
    },
    {
        "role": "user",
        "content": [
            {"type": "image", "path": "/path/to/image.jpg"},
            {"type": "video", "path": "/path/to/video.mp4"},
            {"type": "audio", "path": "/path/to/audio.wav"},
            {"type": "text", "text": "What are the elements can you see and hear in these medias?"},
        ],
    }
]

conversations = [conversation1, conversation2, conversation3, conversation4]

inputs = processor.apply_chat_template(
    conversations,
    load_audio_from_video=True,
    add_generation_prompt=True,
    tokenize=True,
    return_dict=True,
    return_tensors="pt",
    fps=1,

    # kwargs to be passed to `Qwen3OmniMoeProcessor`
    padding=True,
    use_audio_in_video=True,
).to(model.thinker.device)

text_ids = model.generate(**inputs, use_audio_in_video=True)
text = processor.batch_decode(text_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)

print(text)
```

### Usage Tips

#### Image Resolution trade-off

The model supports a wide range of resolution inputs. By default, it uses the native resolution for input, but higher resolutions can enhance performance at the cost of more computation. Users can set the minimum and maximum number of pixels to achieve an optimal configuration for their needs.

```python
min_pixels = 128*28*28
max_pixels = 768*28*28
processor = AutoProcessor.from_pretrained("Qwen/Qwen3-Omni-30B-A3B-Instruct", min_pixels=min_pixels, max_pixels=max_pixels)
```

#### Prompt for audio output

If users need audio output, the system prompt must be set as "You are Qwen, a virtual human developed by the Qwen Team, Alibaba Group, capable of perceiving auditory and visual inputs, as well as generating text and speech.", otherwise the audio output may not work as expected.

```json
{
    "role": "system",
    "content": "You are Qwen, a virtual human developed by the Qwen Team, Alibaba Group, capable of perceiving auditory and visual inputs, as well as generating text and speech.",
}
```

#### Use audio output or not

The model supports both text and audio outputs, if users do not need audio outputs, they can set `enable_audio_output` in the `from_pretrained` function. This option will save about `~2GB` of GPU memory but the `return_audio` option for `generate` function will only allow to be set at `False`.

```python
model = Qwen3OmniMoeForConditionalGeneration.from_pretrained(
    "Qwen/Qwen3-Omni-30B-A3B-Instruct",
    device_map="auto",
    enable_audio_output=False,
)
```

In order to obtain a flexible experience, we recommend that users set `enable_audio_output` at `True` when initializing the model through `from_pretrained` function, and then decide whether to return audio when `generate` function is called. When `return_audio` is set to `False`, the model will only return text outputs to get text responses faster.

```python
model = Qwen3OmniMoeForConditionalGeneration.from_pretrained(
    "Qwen/Qwen3-Omni-30B-A3B-Instruct",
    device_map="auto",
    enable_audio_output=True,
)
...
text_ids = model.generate(**inputs, return_audio=False)
```

#### Change voice type of output audio

Qwen3-Omni-MOE supports the ability to change the voice of the output audio. Users can use the `spk` parameter of `generate` function to specify the voice type. The `"Qwen/Qwen3-Omni-30B-A3B-Instruct"` checkpoint support two voice types: `Chelsie` and `Ethan`, while `Chelsie` is a female voice and `Ethan` is a male voice. By default, if `spk` is not specified, the default voice type is `Chelsie`.

```python
text_ids, audio = model.generate(**inputs, spk="Chelsie")
```

```python
text_ids, audio = model.generate(**inputs, spk="Ethan")
```

#### Flash-Attention 2 to speed up generation

First, make sure to install the latest version of Flash Attention 2:

```bash
pip install -U flash-attn --no-build-isolation
```

Also, you should have hardware that is compatible with FlashAttention 2. Read more about it in the official documentation of the [flash attention repository](https://github.com/Dao-AILab/flash-attention). FlashAttention-2 can only be used when a model is loaded in `torch.float16` or `torch.bfloat16`.

To load and run a model using FlashAttention-2, add `attn_implementation="flash_attention_2"` when loading the model:

```python
from transformers import Qwen3OmniMoeForConditionalGeneration

model = Qwen3OmniMoeForConditionalGeneration.from_pretrained(
    "Qwen/Qwen3-Omni-30B-A3B-Instruct",
    device_map="auto",
    attn_implementation="flash_attention_2",
)
```

## Qwen3OmniMoeConfig[[transformers.Qwen3OmniMoeConfig]]

- **thinker_config** (`dict`, *optional*) --
  Configuration of the underlying thinker sub-model.
- **talker_config** (`dict`, *optional*) --
  Configuration of the underlying talker sub-model.
- **code2wav_config** (`dict`, *optional*) --
  Configuration of the underlying code2wav sub-model.
- **enable_audio_output** (`bool`, *optional*, defaults to `True`) --
  Whether enable audio output and load talker and code2wav module.
- **im_start_token_id** (`int`, *optional*, defaults to 151644) --
  Token id for the start of image
- **im_end_token_id** (`int`, *optional*, defaults to 151645) --
  Token id for the end of image
- **tts_pad_token_id** (`int`, *optional*, defaults to 151671) --
  Token id for the padding in TTS
- **tts_bos_token_id** (`int`, *optional*, defaults to 151672) --
  Token id for the start of sequence in TTS
- **tts_eos_token_id** (`int`, *optional*, defaults to 151673) --
  Token id for the end of sequence in TTS of image
- **system_token_id** (`int`, *optional*, defaults to 8948) --
  Token id for the system prompt
- **user_token_id** (`int`, *optional*, defaults to 872) --
  Token id for the user prompt
- **assistant_token_id** (`int`, *optional*, defaults to 77091) --
  Token id for the assistant prompt
- **initializer_range** (`float`, *optional*) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Qwen3 Omni MoeModel. It is used to instantiate a Qwen3 Omni Moe
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Qwen/Qwen3-Omni-30B-A3B-Instruct](https://huggingface.co/Qwen/Qwen3-Omni-30B-A3B-Instruct)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import (
...     Qwen3OmniMoeThinkerConfig,
...     Qwen3OmniMoeTalkerConfig,
...     Qwen3OmniMoeCode2WavConfig,
...     Qwen3OmniMoeForConditionalGeneration,
...     Qwen3OmniMoeConfig,
... )

>>> # Initializing a Qwen3OmniMoe style configuration
>>> configuration = Qwen3OmniMoeConfig()

>>> # Initializing a model from the configuration
>>> model = Qwen3OmniMoeForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

- **decoder** (`Optional[bool]`, *optional*, defaults to `False`) --
  If set to `True`, then only search for decoder config names.

Returns the config that is meant to be used with text IO. On most models, it is the original config instance
itself. On specific composite models, it is under a set of valid names.

## Qwen3OmniMoeThinkerConfig[[transformers.Qwen3OmniMoeThinkerConfig]]

- **audio_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the audio backbone.
- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **position_id_per_seconds** (`int`, *optional*, defaults to 25) --
  The increment of position id per second.
- **audio_start_token_id** (`int`, *optional*, defaults to 151647) --
  The audio start token id to encode the audio prompt.
- **user_token_id** (`int`, *optional*, defaults to 872) --
  The user token id to encode the user token.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **audio_token_id** (`int`, *optional*, defaults to `151646`) --
  The audio token index used as a placeholder for input audio.
- **image_token_id** (`int`, *optional*, defaults to `151655`) --
  The image token index used as a placeholder for input images.
- **video_token_id** (`int`, *optional*, defaults to `151656`) --
  The video token index used as a placeholder for input videos.

This is the configuration class to store the configuration of a Qwen3 Omni MoeModel. It is used to instantiate a Qwen3 Omni Moe
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Qwen/Qwen3-Omni-30B-A3B-Instruct](https://huggingface.co/Qwen/Qwen3-Omni-30B-A3B-Instruct)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Qwen3OmniMoeThinkerModel, Qwen3OmniMoeThinkerConfig

>>> # Initializing a default Qwen3OmniMoeThinkerConfig
>>> configuration = Qwen3OmniMoeThinkerConfig()

>>> # Initializing a model (with random weights) from the default configuration
>>> model = Qwen3OmniMoeThinkerModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Qwen3OmniMoeTalkerConfig[[transformers.Qwen3OmniMoeTalkerConfig]]

- **code_predictor_config** (`dict`, *optional*) --
  A dictionary of configuration parameters used to initialize a [Qwen3OmniMoeTalkerCodePredictorConfig](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeTalkerCodePredictorConfig).
  If not provided, defaults will be used.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **num_code_groups** (`int`, *optional*, defaults to 32) --
  Number of codebook groups used in the predicted acoustic token sequence, corresponding to multi-codebook VQ representation.
- **thinker_hidden_size** (`int`, *optional*, defaults to 2048) --
  Hidden dimension size of the thinker module used for intermediate reasoning or latent planning before audio generation.
- **codec_eos_token_id** (`int`, *optional*, defaults to 4198) --
  Token ID representing the end-of-speech token in the codec-generated sequence.
- **accept_hidden_layer** (`int`, *optional*, defaults to 18) --
  Index of the hidden layer whose output is used for accepting or refining generated tokens during think-and-speak process.
- **codec_nothink_id** (`int`, *optional*, defaults to 4203) --
  Token ID indicating no thinking step is required during generation.
- **codec_think_bos_id** (`int`, *optional*, defaults to 4204) --
  Token ID marking the beginning of a thinking sequence.
- **codec_think_eos_id** (`int`, *optional*, defaults to 4205) --
  Token ID marking the end of a thinking sequence.
- **codec_pad_id** (`int`, *optional*, defaults to 4196) --
  Padding token ID used in codec input sequences.
- **codec_bos_id** (`int`, *optional*, defaults to 4197) --
  Beginning-of-speech token ID in codec sequences.
- **audio_token_id** (`int`, *optional*, defaults to `151646`) --
  The audio token index used as a placeholder for input audio.
- **image_token_id** (`int`, *optional*, defaults to `151655`) --
  The image token index used as a placeholder for input images.
- **video_token_id** (`int`, *optional*, defaults to `151656`) --
  The video token index used as a placeholder for input videos.
- **vision_start_token_id** (`int`, *optional*, defaults to `151652`) --
  Token ID that marks the start of a visual segment in the multimodal input sequence.
- **position_id_per_seconds** (`int`, *optional*, defaults to 25) --
  Number of position IDs allocated per second of audio content, used for temporal alignment in generation.
- **audio_start_token_id** (`int`, *optional*, defaults to 151669) --
  Token ID that indicates the start of an audio generation segment in the output.
- **speaker_id** (`dict`, *optional*) --
  Speaker name to speaker id dict.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Qwen3 Omni MoeModel. It is used to instantiate a Qwen3 Omni Moe
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Qwen/Qwen3-Omni-30B-A3B-Instruct](https://huggingface.co/Qwen/Qwen3-Omni-30B-A3B-Instruct)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Qwen3OmniMoeTalkerConfig, Qwen3OmniMoeTalker

>>> # Initialize a Qwen3OmniMoeTalkerConfig with default sub-configurations
>>> config = Qwen3OmniMoeTalkerConfig(
...     num_code_groups=32,
...     thinker_hidden_size=2048,
... )

>>> # Initialize the full Qwen3-Omni Talker model
>>> model = Qwen3OmniMoeTalker(config)

>>> # Access the model configuration
>>> config = model.config
>>> print(config.text_config)  # Access text decoder configuration
>>> print(config.code_predictor_config)  # Access code predictor configuration
```

## Qwen3OmniMoeTextConfig[[transformers.Qwen3OmniMoeTextConfig]]

- **vocab_size** (`int`, *optional*, defaults to `3584`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `2048`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `18944`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `28`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `28`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_key_value_heads** (`int`, *optional*, defaults to `4`) --
  This is the number of key_value heads that should be used to implement Grouped Query Attention. If
  `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if
  `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When
  converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed
  by meanpooling all the original heads within that group. For more details, check out [this
  paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to
  `num_attention_heads`.
- **hidden_act** (`str`, *optional*, defaults to `silu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **max_position_embeddings** (`int`, *optional*, defaults to `32768`) --
  The maximum sequence length that this model might ever be used with.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the rms normalization layers.
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **rope_parameters** (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **attention_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in the query, key, value and output projection layers during self-attention.
- **sliding_window** (`int`, *optional*) --
  Sliding window attention window size. If `None`, no sliding window is applied.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **decoder_sparse_step** (`int`, *optional*, defaults to 1) --
  The frequency of the MoE layer.
- **moe_intermediate_size** (`int`, *optional*, defaults to `768`) --
  Intermediate size of the routed expert MLPs.
- **num_experts_per_tok** (`int`, *optional*, defaults to `8`) --
  Number of experts to route each token to. This is the top-k value for the token-choice routing.
- **num_experts** (`int`, *optional*, defaults to `128`) --
  Number of routed experts in MoE layers.

- **norm_topk_prob** (`bool`, *optional*, defaults to `True`) --
  Whether to normalize the weights of the routed experts.

- **output_router_logits** (`bool`, *optional*, defaults to `False`) --
  Whether or not the router logits should be returned by the model. Enabling this will also allow the model
  to output the auxiliary loss, including load balancing loss and router z-loss.
- **router_aux_loss_coef** (`float`, *optional*, defaults to `0.001`) --
  Auxiliary load balancing loss coefficient. Used to penalize uneven expert routing in MoE models.
- **mlp_only_layers** (`list[int]`, *optional*, defaults to `[]`) --
  Indicate which layers use Qwen3OmniMoeTextMLP rather than Qwen3OmniMoeTextSparseMoeBlock
  The list contains layer index, from 0 to num_layers-1 if we have num_layers layers
  If `mlp_only_layers` is empty, `decoder_sparse_step` is used to determine the sparsity.
- **pad_token_id** (`int`, *optional*) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*) --
  Token id used for end-of-stream in the vocabulary.

This is the configuration class to store the configuration of a Qwen3 Omni MoeModel. It is used to instantiate a Qwen3 Omni Moe
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Qwen/Qwen3-Omni-30B-A3B-Instruct](https://huggingface.co/Qwen/Qwen3-Omni-30B-A3B-Instruct)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import Qwen3OmniMoeTextModel, Qwen3OmniMoeTextConfig

>>> # Initializing a Qwen3OmniMoeText style configuration
>>> configuration = Qwen3OmniMoeTextConfig()

>>> # Initializing a model from the Qwen3-15B-A2B" style configuration
>>> model = Qwen3OmniMoeTextModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Qwen3OmniMoeVisionEncoderConfig[[transformers.Qwen3OmniMoeVisionEncoderConfig]]

- **depth** (`int`, *optional*, defaults to `27`) --
  Number of Transformer layers in the vision encoder.
- **hidden_size** (`int`, *optional*, defaults to `1152`) --
  Dimension of the hidden representations.
- **hidden_act** (`str`, *optional*, defaults to `gelu_pytorch_tanh`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **intermediate_size** (`int`, *optional*, defaults to `4304`) --
  Dimension of the MLP representations.
- **num_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **in_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) --
  The size (resolution) of each patch.
- **spatial_merge_size** (`int`, *optional*, defaults to `2`) --
  The size of the spatial merge window used to reduce the number of visual tokens by merging neighboring patches.
- **temporal_patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `2`) --
  Temporal patch size used in the 3D patch embedding for video inputs.
- **out_hidden_size** (`int`, *optional*, defaults to 3584) --
  The output hidden size of the vision model.
- **num_position_embeddings** (`int`, *optional*, defaults to 2304) --
  The maximum sequence length that this model might ever be used with
- **deepstack_visual_indexes** (`list[int]`, *optional*, defaults to `[8, 16, 24]`) --
  Indexed of layers for deepstack embeddings.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Qwen3 Omni MoeModel. It is used to instantiate a Qwen3 Omni Moe
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Qwen/Qwen3-Omni-30B-A3B-Instruct](https://huggingface.co/Qwen/Qwen3-Omni-30B-A3B-Instruct)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Qwen3OmniMoeAudioEncoderConfig[[transformers.Qwen3OmniMoeAudioEncoderConfig]]

- **num_mel_bins** (`int`, *optional*, defaults to `128`) --
  Number of mel features used per input frame. Should correspond to the value used in the
  `AutoFeatureExtractor` class.
- **encoder_layers** (`int`, *optional*, defaults to `32`) --
  Number of hidden layers in the Transformer encoder. Will use the same value as `num_layers` if not set.
- **encoder_attention_heads** (`int`, *optional*, defaults to `20`) --
  Number of attention heads for each attention layer in the Transformer encoder.
- **encoder_ffn_dim** (`int`, *optional*, defaults to `5120`) --
  Dimensionality of the "intermediate" (often named feed-forward) layer in encoder.
- **d_model** (`int`, *optional*, defaults to `1280`) --
  Size of the encoder layers and the pooler layer.
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The ratio for all dropout layers.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **activation_function** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **activation_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for activations inside the fully connected layer.
- **scale_embedding** (`bool`, *optional*, defaults to `False`) --
  Whether to scale embeddings by dividing by sqrt(d_model).
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **max_source_positions** (`int`, *optional*, defaults to 1500) --
  Maximum sequence length for the inputs
- **n_window** (`int`, *optional*, defaults to 50) --
  Number of windows
- **output_dim** (`int`, *optional*, defaults to 3584) --
  Dimensionality of the output
- **n_window_infer** (`int`, *optional*, defaults to `800`) --
  Number of windows during inference
- **conv_chunksize** (`int`, *optional*, defaults to `500`) --
  Chunk size of each input to convolutional layer
- **downsample_hidden_size** (`int`, *optional*, defaults to `480`) --
  Hidden size in downsampling layer

This is the configuration class to store the configuration of a Qwen3 Omni MoeModel. It is used to instantiate a Qwen3 Omni Moe
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Qwen/Qwen3-Omni-30B-A3B-Instruct](https://huggingface.co/Qwen/Qwen3-Omni-30B-A3B-Instruct)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Qwen3OmniMoeTalkerTextConfig[[transformers.Qwen3OmniMoeTalkerTextConfig]]

- **vocab_size** (`int`, *optional*, defaults to `3072`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `1024`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `2048`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `20`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_key_value_heads** (`int`, *optional*, defaults to `2`) --
  This is the number of key_value heads that should be used to implement Grouped Query Attention. If
  `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if
  `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When
  converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed
  by meanpooling all the original heads within that group. For more details, check out [this
  paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to
  `num_attention_heads`.
- **hidden_act** (`str`, *optional*, defaults to `silu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **max_position_embeddings** (`int`, *optional*, defaults to `32768`) --
  The maximum sequence length that this model might ever be used with.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the rms normalization layers.
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **rope_parameters** (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **attention_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in the query, key, value and output projection layers during self-attention.
- **sliding_window** (`int`, *optional*) --
  Sliding window attention window size. If `None`, no sliding window is applied.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **decoder_sparse_step** (`int`, *optional*, defaults to 1) --
  The frequency of the MoE layer.
- **moe_intermediate_size** (`int`, *optional*, defaults to `384`) --
  Intermediate size of the routed expert MLPs.
- **num_experts_per_tok** (`int`, *optional*, defaults to `8`) --
  Number of experts to route each token to. This is the top-k value for the token-choice routing.
- **num_experts** (`int`, *optional*, defaults to `128`) --
  Number of routed experts in MoE layers.

- **norm_topk_prob** (`bool`, *optional*, defaults to `False`) --
  Whether to normalize the weights of the routed experts.

- **output_router_logits** (`bool`, *optional*, defaults to `False`) --
  Whether or not the router logits should be returned by the model. Enabling this will also allow the model
  to output the auxiliary loss, including load balancing loss and router z-loss.
- **router_aux_loss_coef** (`float`, *optional*, defaults to `0.001`) --
  Auxiliary load balancing loss coefficient. Used to penalize uneven expert routing in MoE models.
- **mlp_only_layers** (`list[int]`, *optional*, defaults to `[]`) --
  Indicate which layers use Qwen3OmniMoeTalkerTextMLP rather than Qwen3OmniMoeTalkerTextSparseMoeBlock
  The list contains layer index, from 0 to num_layers-1 if we have num_layers layers
  If `mlp_only_layers` is empty, `decoder_sparse_step` is used to determine the sparsity.
- **pad_token_id** (`int`, *optional*) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*) --
  Token id used for end-of-stream in the vocabulary.

This is the configuration class to store the configuration of a Qwen3 Omni MoeModel. It is used to instantiate a Qwen3 Omni Moe
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Qwen/Qwen3-Omni-30B-A3B-Instruct](https://huggingface.co/Qwen/Qwen3-Omni-30B-A3B-Instruct)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import Qwen3OmniMoeTalkerTextModel, Qwen3OmniMoeTalkerTextConfig

>>> # Initializing a Qwen3OmniMoeTalkerText style configuration
>>> configuration = Qwen3OmniMoeTalkerTextConfig()

>>> # Initializing a model from the Qwen3-15B-A2B" style configuration
>>> model = Qwen3OmniMoeTalkerTextModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Qwen3OmniMoeTalkerCodePredictorConfig[[transformers.Qwen3OmniMoeTalkerCodePredictorConfig]]

- **vocab_size** (`int`, *optional*, defaults to `2048`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `1024`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `3072`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `5`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_key_value_heads** (`int`, *optional*, defaults to `8`) --
  This is the number of key_value heads that should be used to implement Grouped Query Attention. If
  `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if
  `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When
  converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed
  by meanpooling all the original heads within that group. For more details, check out [this
  paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to
  `num_attention_heads`.
- **head_dim** (`int`, *optional*, defaults to `128`) --
  The attention head dimension. If None, it will default to hidden_size // num_attention_heads
- **hidden_act** (`str`, *optional*, defaults to `silu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **max_position_embeddings** (`int`, *optional*, defaults to `32768`) --
  The maximum sequence length that this model might ever be used with.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the rms normalization layers.
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **rope_parameters** (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **attention_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in the query, key, value and output projection layers during self-attention.
- **sliding_window** (`int`, *optional*) --
  Sliding window attention window size. If `None`, no sliding window is applied.
- **max_window_layers** (`int`, *optional*, defaults to `28`) --
  The number of layers using full attention. The first `max_window_layers` layers will use full attention, while any
  additional layer afterwards will use SWA (Sliding Window Attention).
- **layer_types** (`list[str]`, *optional*) --
  A list that explicitly maps each layer index with its layer type. If not provided, it will be automatically
  generated based on config values.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **pad_token_id** (`int`, *optional*) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*) --
  Token id used for end-of-stream in the vocabulary.
- **num_code_groups** (`int`, *optional*, defaults to 32) --
  Number of codebook groups used in the predicted acoustic token sequence, corresponding to multi-codebook VQ representation.

This is the configuration class to store the configuration of a Qwen3 Omni MoeModel. It is used to instantiate a Qwen3 Omni Moe
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Qwen/Qwen3-Omni-30B-A3B-Instruct](https://huggingface.co/Qwen/Qwen3-Omni-30B-A3B-Instruct)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Qwen3OmniMoeForConditionalGeneration[[transformers.Qwen3OmniMoeForConditionalGeneration]]

## Qwen3OmniMoeThinkerTextModel[[transformers.Qwen3OmniMoeThinkerTextModel]]

- **config** ([Qwen3OmniMoeTextConfig](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeTextConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
Text part of Qwen3OmniMoeThinker, not a pure text-only model, as DeepStack integrates visual features into the early hidden states.

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
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
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
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **visual_pos_masks** (`torch.Tensor` of shape `(batch_size, seqlen)`, *optional*) --
  The mask of the visual positions.
- **deepstack_visual_embeds** (`list[torch.Tensor]`, *optional*) --
  The deepstack visual embeddings. The shape is (num_layers, visual_seqlen, embed_dim).
  The feature is extracted from the different visual encoder layers, and fed to the decoder
  hidden states. It's from the paper DeepStack(https://arxiv.org/abs/2406.04334).`MoeModelOutputWithPast` or `tuple(torch.FloatTensor)`A `MoeModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen3OmniMoeConfig](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeConfig)) and inputs.
The [Qwen3OmniMoeThinkerTextModel](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeThinkerTextModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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
- **router_logits** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_router_probs=True` and `config.add_router_probs=True` is passed or when `config.output_router_probs=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, sequence_length, num_experts)`.

  Raw router logtis (post-softmax) that are computed by MoE routers, these terms are used to compute the auxiliary
  loss for Mixture of Experts models.

## Qwen3OmniMoeThinkerForConditionalGeneration[[transformers.Qwen3OmniMoeThinkerForConditionalGeneration]]

- **config** ([Qwen3OmniMoeThinkerForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeThinkerForConditionalGeneration)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Qwen2.5OmniThinker model which consists of a audio backbone and a language model.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **input_features** (`` of shape `(batch_size, sequence_length, feature_dim)`) --
  The tensors corresponding to the input audio features. Audio features can be obtained using
  [WhisperFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/whisper#transformers.WhisperFeatureExtractor). See [WhisperFeatureExtractor.__call__()](/docs/transformers/v5.14.0/en/model_doc/whisper#transformers.WhisperFeatureExtractor.__call__) for details ([Qwen3OmniMoeProcessor](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeProcessor) uses
  [WhisperFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/whisper#transformers.WhisperFeatureExtractor) for processing audios).
- **pixel_values** (`` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Qwen2VLImageProcessor](/docs/transformers/v5.14.0/en/model_doc/qwen2_vl#transformers.Qwen2VLImageProcessor). See `Qwen2VLImageProcessor.__call__()` for details ([Qwen3OmniMoeProcessor](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeProcessor) uses
  [Qwen2VLImageProcessor](/docs/transformers/v5.14.0/en/model_doc/qwen2_vl#transformers.Qwen2VLImageProcessor) for processing images).
- **pixel_values_videos** (`` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`) --
  The tensors corresponding to the input video. Pixel values for videos can be obtained using
  [Qwen2VLVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/qwen2_vl#transformers.Qwen2VLVideoProcessor). See `Qwen2VLVideoProcessor.__call__()` for details ([Qwen3OmniMoeProcessor](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeProcessor) uses
  [Qwen2VLVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/qwen2_vl#transformers.Qwen2VLVideoProcessor) for processing videos).
- **image_grid_thw** (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) --
  The temporal, height and width of feature shape of each image in LLM.
- **video_grid_thw** (`torch.LongTensor` of shape `(num_videos, 3)`, *optional*) --
  The temporal, height and width of feature shape of each video in LLM.
- **attention_mask** (`` of shape `(batch_size, sequence_length)`) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **feature_attention_mask** (`torch.Tensor` of shape `(batch_size, feature_sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding feature indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
- **audio_feature_lengths** (`torch.LongTensor` of shape `(num_audios)`, *optional*) --
  The length of feature shape of each audio in LLM.
- **position_ids** (`` of shape `(batch_size, sequence_length)`) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **past_key_values** (``) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **inputs_embeds** (`` of shape `(batch_size, sequence_length, hidden_size)`) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **use_cache** (``) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **output_router_logits** (`bool`, *optional*) --
  Whether or not to return the logits of all the routers. They are useful for computing the router loss, and
  should not be returned during inference.
- **use_audio_in_video** (`bool`, *optional*) --
  Whether or not use audio track in video, should same as the parameter in `process_audio_info`.
- **video_second_per_grid** (`torch.LongTensor` of shape `(num_videos)`, *optional*) --
  Number of seconds per grid for each video, used for temporal feature mapping.`Qwen3OmniMoeThinkerCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `Qwen3OmniMoeThinkerCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen3OmniMoeConfig](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeConfig)) and inputs.
The [Qwen3OmniMoeThinkerForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeThinkerForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **aux_loss** (`torch.FloatTensor`, *optional*, returned when `labels` is provided) -- aux_loss for the sparse modules.
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **router_logits** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_router_probs=True` and `config.add_router_probs=True` is passed or when `config.output_router_probs=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, sequence_length, num_experts)`.

  Raw router logtis (post-softmax) that are computed by MoE routers, these terms are used to compute the auxiliary
  loss for Mixture of Experts models.
- **rope_deltas** (`torch.LongTensor` of shape `(batch_size, )`, *optional*) -- The rope index difference between sequence length and multimodal rope.
  The attribute is deprecated and will be removed in v5.20, use `model.base_model.rope_deltas` instead.

Example:

```python
>>> from io import BytesIO
>>> from urllib.request import urlopen
>>> import librosa
>>> from qwen_vl_utils import process_vision_info
>>> from transformers import Qwen3OmniMoeProcessor, Qwen3OmniMoeThinkerForConditionalGeneration

>>> thinker = Qwen3OmniMoeThinkerForConditionalGeneration.from_pretrained("Qwen/Qwen2.5-Omni-7B")
>>> processor = Qwen3OmniMoeProcessor.from_pretrained("Qwen/Qwen2.5-Omni-7B")

>>> conversations = [
>>>         {'role': 'system', 'content': 'You are a helpful voice chat bot, and please respond to me in a casual conversation manner using random voice.'},
>>>         {"role": "user", "content": [
>>>             {"type": "image", "image_url": "https://www.ilankelman.org/stopsigns/australia.jpg"},
>>>             {"type": "audio", "audio_url": "https://qianwen-res.oss-cn-beijing.aliyuncs.com/Qwen2-Audio/audio/glass-breaking-151256.mp3"},
>>>         ]},
>>> ]

>>> text = processor.apply_chat_template(conversation, add_generation_prompt=True, tokenize=False)
>>> audios = [ librosa.load(BytesIO(urlopen( conversations[1]['content'][1]['audio_url'] ).read()), sr=self.processor.feature_extractor.sampling_rate) ]
>>> images, videos = process_vision_info(conversations)
>>> inputs = processor(text=text, audio=audios, images=images, videos=videos, return_tensors="pt", padding=True)

>>> # Generate
>>> inputs['use_audio_in_video'] = `True` or `False`
>>> generation = thinker.generate(**inputs, max_new_tokens=2048)
>>> generate_ids = generation[:, inputs.input_ids.size(1):]

>>> response = processor.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
```

- **input_features** (`torch.FloatTensor`) --
  The tensors corresponding to the input audios.
- **feature_attention_mask** (`torch.LongTensor`, *optional*) --
  Mask to avoid performing attention on padding feature indices. Mask values selected in `[0, 1]`:
- **audio_feature_lengths** (`torch.LongTensor` of shape `(num_audios)`, *optional*) --
  The length of feature shape of each audio in LLM.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen3OmniMoeConfig](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeConfig)) and inputs.

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

Example:

```python
>>> from PIL import Image
>>> from transformers import AutoProcessor, Qwen3OmniMoeThinkerForConditionalGeneration

>>> model = Qwen3OmniMoeThinkerForConditionalGeneration.from_pretrained("Qwen/Qwen3-Omni-30B-A3B-Instruct")
>>> processor = AutoProcessor.from_pretrained("Qwen/Qwen3-Omni-30B-A3B-Instruct")

>>> messages = [
...     {
...         "role": "user", "content": [
...             {"type": "image", "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"},
...             {"type": "text", "text": "Where is the cat standing?"},
...         ]
...     },
... ]

>>> inputs = processor.apply_chat_template(
...     messages,
...     tokenize=True,
...     return_dict=True,
...     return_tensors="pt",
...     add_generation_prompt=True
... )
>>> # Generate
>>> generate_ids = model.generate(**inputs)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True)[0]
```

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images.
- **image_grid_thw** (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) --
  The temporal, height and width of feature shape of each image in LLM.`BaseModelOutputWithDeepstackFeatures` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithDeepstackFeatures` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen3OmniMoeConfig](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeConfig)) and inputs.

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
- **deepstack_features** (`List[torch.FloatTensor]`, *optional*) -- List of hidden-states (feature maps) from deepstack layers.

Example:

```python
>>> from PIL import Image
>>> from transformers import AutoProcessor, Qwen3OmniMoeThinkerForConditionalGeneration

>>> model = Qwen3OmniMoeThinkerForConditionalGeneration.from_pretrained("Qwen/Qwen3-Omni-30B-A3B-Instruct")
>>> processor = AutoProcessor.from_pretrained("Qwen/Qwen3-Omni-30B-A3B-Instruct")

>>> messages = [
...     {
...         "role": "user", "content": [
...             {"type": "image", "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"},
...             {"type": "text", "text": "Where is the cat standing?"},
...         ]
...     },
... ]

>>> inputs = processor.apply_chat_template(
...     messages,
...     tokenize=True,
...     return_dict=True,
...     return_tensors="pt",
...     add_generation_prompt=True
... )
>>> # Generate
>>> generate_ids = model.generate(**inputs)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True)[0]
```

Obtains multimodal placeholder mask from `input_ids` or `inputs_embeds`, and checks that the placeholder token count is
equal to the length of multimodal features. If the lengths are different, an error is raised.

- **pixel_values_videos** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input videos.
- **video_grid_thw** (`torch.LongTensor` of shape `(num_videos, 3)`, *optional*) --
  The temporal, height and width of feature shape of each video in LLM.`BaseModelOutputWithDeepstackFeatures` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithDeepstackFeatures` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen3OmniMoeConfig](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeConfig)) and inputs.

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
- **deepstack_features** (`List[torch.FloatTensor]`, *optional*) -- List of hidden-states (feature maps) from deepstack layers.

Example:

```python
>>> from PIL import Image
>>> from transformers import AutoProcessor, Qwen3OmniMoeThinkerForConditionalGeneration

>>> model = Qwen3OmniMoeThinkerForConditionalGeneration.from_pretrained("Qwen/Qwen3-Omni-30B-A3B-Instruct")
>>> processor = AutoProcessor.from_pretrained("Qwen/Qwen3-Omni-30B-A3B-Instruct")

>>> messages = [
...     {
...         "role": "user", "content": [
...             {"type": "image", "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"},
...             {"type": "text", "text": "Where is the cat standing?"},
...         ]
...     },
... ]

>>> inputs = processor.apply_chat_template(
...     messages,
...     tokenize=True,
...     return_dict=True,
...     return_tensors="pt",
...     add_generation_prompt=True
... )
>>> # Generate
>>> generate_ids = model.generate(**inputs)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True)[0]
```

## Qwen3OmniMoeTalkerForConditionalGeneration[[transformers.Qwen3OmniMoeTalkerForConditionalGeneration]]

- **config** ([Qwen3OmniMoeTalkerConfig](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeTalkerConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Qwen3 Omni Moe Model for token generation conditioned on other modalities (e.g. image-text-to-text generation).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`` of shape `(batch_size, sequence_length)`) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **use_audio_in_video** (`bool`, *optional*) --
  If set to `True`, use the audio in video.
- **audio_feature_lengths** (`torch.LongTensor` of shape `(num_audios)`, *optional*) --
  The length of feature shape of each audio in LLM.
- **video_second_per_grid** (`torch.LongTensor` of shape `(num_videos)`, *optional*) --
  Number of seconds per grid for each video, used for temporal feature mapping.
- **image_grid_thw** (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) --
  The temporal, height and width of feature shape of each image in LLM.
- **video_grid_thw** (`torch.LongTensor` of shape `(num_videos, 3)`, *optional*) --
  The temporal, height and width of feature shape of each video in LLM.
- **position_ids** (`` of shape `(batch_size, sequence_length)`) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **past_key_values** (``) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **inputs_embeds** (`` of shape `(batch_size, sequence_length, hidden_size)`) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **labels** (`` of shape `(batch_size, sequence_length)`) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **use_cache** (``) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **output_router_logits** (``) --
  Whether or not to return the logits of all the routers. They are useful for computing the router loss, and
  should not be returned during inference.
- **residual_codes** (`torch.Tensor`) --
  The predicted residual codes of previous step.
- **trailing_text_hidden** (`torch.Tensor`) --
  Text hidden states from thinker after the first token.
- **tts_pad_embed** (`torch.Tensor`) --
  Embedding tensor of `tts_pad_token_id`.
- **generation_step** (`int`) --
  Generation step since prefill, used to sync with `trailing_text_hidden`.
- **talker_input_ids** (`torch.Tensor`) --
  Input ids from thinker, used to compute 3d RoPE.`MoeCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `MoeCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen3OmniMoeConfig](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeConfig)) and inputs.
The [Qwen3OmniMoeTalkerForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeTalkerForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **aux_loss** (`torch.FloatTensor`, *optional*, returned when `labels` is provided) -- aux_loss for the sparse modules.
- **router_logits** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_router_probs=True` and `config.add_router_probs=True` is passed or when `config.output_router_probs=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, sequence_length, num_experts)`.

  Raw router logtis (post-softmax) that are computed by MoE routers, these terms are used to compute the auxiliary
  loss for Mixture of Experts models.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

## Qwen3OmniMoePreTrainedModel[[transformers.Qwen3OmniMoePreTrainedModel]]

- **config** ([PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

## Qwen3OmniMoePreTrainedModelForConditionalGeneration[[transformers.Qwen3OmniMoePreTrainedModelForConditionalGeneration]]

- **token_indices** (`torch.Tensor` of shape `(seq_len, )`) -- A monotonically increasing list of
  token index values.
- **t_ntoken_per_chunk** (`int`) -- Number of tokens per chunk (used as the chunk size threshold).
- **remove_index** (`int`) An index id to subtract from `token_indices` before chunking --`list[tuple[int, int]]`A list of tuples, each representing the start (inclusive)
and end (exclusive) indices of a chunk in `token_indices`.

Splits token index list into chunks based on token value ranges.

Given a list of token indices, returns a list of (start, end) index tuples representing
slices of the list where the token values fall within successive ranges of `t_ntoken_per_chunk`.

For example, if `t_ntoken_per_chunk` is 1000, the function will create chunks such that:
- the first chunk contains token values < 1000,
- the second chunk contains values >= 1000 and < 2000, and so on.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default should you provide
  it.
- **image_grid_thw** (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) --
  The temporal, height and width of feature shape of each image in LLM.
- **video_grid_thw** (`torch.LongTensor` of shape `(num_videos, 3)`, *optional*) --
  The temporal, height and width of feature shape of each video in LLM.
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
- **use_audio_in_video** (`bool`, *optional*) --
  If set to `True`, use the audio in video.
- **audio_seqlens** (`torch.LongTensor` of shape `(num_audios)`, *optional*) --
  The length of feature shape of each audio in LLM.
- **second_per_grids** (`torch.LongTensor` of shape `(num_videos)`, *optional*) --
  The time interval (in seconds) for each grid along the temporal dimension in the 3D position IDs.position_ids (`torch.LongTensor` of shape `(3, batch_size, sequence_length)`)
mrope_position_deltas (`torch.Tensor` of shape `(batch_size)`)

Calculate the 3D rope index based on image and video's temporal, height and width in LLM.

Explanation:
Each embedding sequence contains vision embedding and text embedding or just contains text embedding.

For pure text embedding sequence, the rotary position embedding has no difference with modern LLMs.
Examples:
input_ids: [T T T T T], here T is for text.
temporal position_ids: [0, 1, 2, 3, 4]
height position_ids: [0, 1, 2, 3, 4]
width position_ids: [0, 1, 2, 3, 4]

For vision and text embedding sequence, we calculate 3D rotary position embedding for vision part
and 1D rotary position embedding for text part.
Examples:
Temporal (Time): 3 patches, representing different segments of the video in time.
Height: 2 patches, dividing each frame vertically.
Width: 2 patches, dividing each frame horizontally.
We also have some important parameters:
fps (Frames Per Second): The video's frame rate, set to 1. This means one frame is processed each second.
tokens_per_second: This is a crucial parameter. It dictates how many "time-steps" or "temporal tokens" are conceptually packed into a one-second interval of the video. In this case, we have 25 tokens per second. So each second of the video will be represented with 25 separate time points. It essentially defines the temporal granularity.
temporal_patch_size: The number of frames that compose one temporal patch. Here, it's 2 frames.
interval: The step size for the temporal position IDs, calculated as tokens_per_second * temporal_patch_size / fps. In this case, 25 * 2 / 1 = 50. This means that each temporal patch will be have a difference of 50 in the temporal position IDs.
input_ids: [V V V V V V V V V V V V T T T T T], here V is for vision.
vision temporal position_ids: [0, 0, 0, 0, 50, 50, 50, 50, 100, 100, 100, 100]
vision height position_ids: [0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1]
vision width position_ids: [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]
text temporal position_ids: [101, 102, 103, 104, 105]
text height position_ids: [101, 102, 103, 104, 105]
text width position_ids: [101, 102, 103, 104, 105]
Here we calculate the text start position_ids as the max vision position_ids plus 1.

## Qwen3OmniMoeTalkerModel[[transformers.Qwen3OmniMoeTalkerModel]]

- **config** ([Qwen3OmniMoeTalkerTextConfig](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeTalkerTextConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
Text part of Qwen3OmniMoe, not a pure text-only model, as DeepStack integrates visual features into the early hidden states.

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
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
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
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **visual_pos_masks** (`torch.Tensor` of shape `(batch_size, seqlen)`, *optional*) --
  The mask of the visual positions.
- **deepstack_visual_embeds** (`list[torch.Tensor]`, *optional*) --
  The deepstack visual embeddings. The shape is (num_layers, visual_seqlen, embed_dim).
  The feature is extracted from the different visual encoder layers, and fed to the decoder
  hidden states. It's from the paper DeepStack(https://arxiv.org/abs/2406.04334).`MoeModelOutputWithPast` or `tuple(torch.FloatTensor)`A `MoeModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen3OmniMoeConfig](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeConfig)) and inputs.
The [Qwen3OmniMoeTalkerModel](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeTalkerModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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
- **router_logits** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_router_probs=True` and `config.add_router_probs=True` is passed or when `config.output_router_probs=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, sequence_length, num_experts)`.

  Raw router logtis (post-softmax) that are computed by MoE routers, these terms are used to compute the auxiliary
  loss for Mixture of Experts models.

## Qwen3OmniMoeThinkerTextPreTrainedModel[[transformers.Qwen3OmniMoeThinkerTextPreTrainedModel]]

- **config** ([PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

## Qwen3OmniMoeProcessor[[transformers.Qwen3OmniMoeProcessor]]

- **image_processor** (`Qwen2VLImageProcessor`) --
  The image processor is a required input.
- **video_processor** (`Qwen2VLVideoProcessor`) --
  The video processor is a required input.
- **feature_extractor** (`WhisperFeatureExtractor`) --
  The feature extractor is a required input.
- **tokenizer** (`Qwen2Tokenizer`) --
  The tokenizer is a required input.
- **chat_template** (`str`) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
Constructs a Qwen3OmniMoeProcessor which wraps a image processor, a video processor, a feature extractor, and a tokenizer into a single processor.

[Qwen3OmniMoeProcessor](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeProcessor) offers all the functionalities of [Qwen2VLImageProcessor](/docs/transformers/v5.14.0/en/model_doc/qwen2_vl#transformers.Qwen2VLImageProcessor), [Qwen2VLVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/qwen2_vl#transformers.Qwen2VLVideoProcessor), [WhisperFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/whisper#transformers.WhisperFeatureExtractor), and [Qwen2Tokenizer](/docs/transformers/v5.14.0/en/model_doc/qwen2#transformers.Qwen2Tokenizer). See the
[~Qwen2VLImageProcessor](/docs/transformers/v5.14.0/en/model_doc/qwen2_vl#transformers.Qwen2VLImageProcessor), [~Qwen2VLVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/qwen2_vl#transformers.Qwen2VLVideoProcessor), [~WhisperFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/whisper#transformers.WhisperFeatureExtractor), and [~Qwen2Tokenizer](/docs/transformers/v5.14.0/en/model_doc/qwen2#transformers.Qwen2Tokenizer) for more information.

- **text** (`str`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **videos** (`Union[list[PIL.Image.Image], numpy.ndarray, torch.Tensor, list[numpy.ndarray], list[torch.Tensor], list[list[PIL.Image.Image]], list[list[numpy.ndarray]], list[list[torch.Tensor]], ~video_utils.URL, list[~video_utils.URL], list[list[~video_utils.URL]], ~video_utils.Path, list[~video_utils.Path], list[list[~video_utils.Path]]]`, *optional*) --
  Video to preprocess. Expects a single or batch of videos with pixel values ranging from 0 to 255. If
  passing in videos with pixel values between 0 and 1, set `do_rescale=False`.
- **audio** (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`, *optional*) --
  The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor.
  In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels,
  and T is the sample length of the audio.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.`~feature_extraction_utils.BatchFeature`- **data** (`dict`, *optional*) -- Dictionary of lists/arrays/tensors returned by the __call__/pad methods ('input_values', 'attention_mask',
  etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.
- **skip_tensor_conversion** (`list[str]` or `set[str]`, *optional*) -- List or set of keys that should NOT be converted to tensors, even when `tensor_type` is specified.

## Qwen3OmniMoeCode2Wav[[transformers.Qwen3OmniMoeCode2Wav]]

## Qwen3OmniMoeCode2WavDecoderBlock[[transformers.Qwen3OmniMoeCode2WavDecoderBlock]]

## Qwen3OmniMoeCode2WavTransformerModel[[transformers.Qwen3OmniMoeCode2WavTransformerModel]]

- **config** (`Qwen3OmniMoeCode2WavConfig`) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Qwen3 Omni Moe Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`` of shape `(batch_size, sequence_length)`) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`` of shape `(batch_size, sequence_length)`) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **past_key_values** (``) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **inputs_embeds** (`` of shape `(batch_size, sequence_length, hidden_size)`) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **use_cache** (``) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).[BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen3OmniMoeConfig](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeConfig)) and inputs.
The [Qwen3OmniMoeCode2WavTransformerModel](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeCode2WavTransformerModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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

## Qwen3OmniMoeTalkerCodePredictorModel[[transformers.Qwen3OmniMoeTalkerCodePredictorModel]]

- **config** ([Qwen3OmniMoeTalkerCodePredictorConfig](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeTalkerCodePredictorConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Qwen3 Omni Moe Model outputting raw hidden-states without any specific head on top.

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
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
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
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).[BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen3OmniMoeConfig](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeConfig)) and inputs.
The [Qwen3OmniMoeTalkerCodePredictorModel](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeTalkerCodePredictorModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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

## Qwen3OmniMoeTalkerCodePredictorModelForConditionalGeneration[[transformers.Qwen3OmniMoeTalkerCodePredictorModelForConditionalGeneration]]

- **config** ([Qwen3OmniMoeTalkerCodePredictorConfig](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeTalkerCodePredictorConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Qwen3 Omni Moe Model for token generation conditioned on other modalities (e.g. image-text-to-text generation).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`` of shape `(batch_size, sequence_length)`) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`` of shape `(batch_size, sequence_length)`) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **past_key_values** (``) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **inputs_embeds** (`` of shape `(batch_size, sequence_length, hidden_size)`) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **labels** (`` of shape `(batch_size, sequence_length)`) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **use_cache** (``) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **generation_steps** (`int`) --
  generation step of code predictor, 0..num_code_groups-1[CausalLMOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or `tuple(torch.FloatTensor)`A [CausalLMOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen3OmniMoeConfig](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeConfig)) and inputs.
The [Qwen3OmniMoeTalkerCodePredictorModelForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/qwen3_omni_moe#transformers.Qwen3OmniMoeTalkerCodePredictorModelForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
