# Qwen2.5-Omni

## Overview

The [Qwen2.5-Omni](https://qwenlm.github.io/blog/qwen2.5-omni/) model is a unified multiple modalities model proposed in [Qwen2.5-Omni Technical Report](https://huggingface.co/papers/2503.20215) from Qwen team, Alibaba Group.

The abstract from the technical report is the following:

*We present Qwen2.5-Omni, an end-to-end multimodal model designed to perceive diverse modalities, including text, images, audio, and video, while simultaneously generating text and natural speech responses in a streaming manner. To enable the streaming of multimodal information inputs, both audio and visual encoders utilize a block-wise processing approach. This strategy effectively decouples the handling of long sequences of multimodal data, assigning the perceptual responsibilities to the multimodal encoder and entrusting the modeling of extended sequences to a large language model. Such a division of labor enhances the fusion of different modalities via the shared attention mechanism. To synchronize the timestamps of video inputs with audio, we organized the audio and video sequentially in an interleaved manner and propose a novel position embedding approach, named TMRoPE (Time-aligned Multimodal RoPE). To concurrently generate text and speech while avoiding interference between the two modalities, we propose Thinker-Talker architecture. In this framework, Thinker functions as a large language model tasked with text generation, while Talker is a dual-track autoregressive model that directly utilizes the hidden representations from the Thinker to produce audio tokens as output. Both the Thinker and Talker models are designed to be trained and inferred in an end-to-end manner. For decoding audio tokens in a streaming manner, we introduce a sliding-window DiT that restricts the receptive field, aiming to reduce the initial package delay. Qwen2.5-Omni outperforms the similarly sized Qwen2-VL and Qwen2-Audio in both image and audio capabilities. Furthermore, Qwen2.5-Omni achieves state-of-the-art performance on multimodal benchmarks like Omni-Bench. Notably, Qwen2.5-Omni is the first open-source model to achieve a level of performance in end-to-end speech instruction following that is comparable to its capabilities with text inputs, as evidenced by benchmarks such as MMLU and GSM8K. As for speech generation, Qwen2.5-Omni's streaming Talker outperform most existing streaming and non-streaming alternatives in robustness and naturalness.*

## Notes

- Use [Qwen2_5OmniForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniForConditionalGeneration) to generate audio and text output. To generate only one output type, use [Qwen2_5OmniThinkerForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniThinkerForConditionalGeneration) for text-only and `Qwen2_5OmniTalkersForConditionalGeneration` for audio-only outputs.
- In case out-of-memory errors when working with video input, decrease `processor.max_pixels`. By default the maximum is set to a very large value and high resolution visuals will not be resized, unless resolution exceeds `processor.max_pixels`.
- The processor has its own [apply_chat_template()](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessorMixin.apply_chat_template) method to convert chat messages to model inputs.

## Usage example

`Qwen2.5-Omni` can be found on the [Huggingface Hub](https://huggingface.co/Qwen).

### Single Media inference

The model can accept text, images, audio and videos as input. Here's an example code for inference.

```python
import soundfile as sf

from transformers import Qwen2_5OmniForConditionalGeneration, Qwen2_5OmniProcessor

model = Qwen2_5OmniForConditionalGeneration.from_pretrained(
    "Qwen/Qwen2.5-Omni-7B",
    device_map="auto",
)
processor = Qwen2_5OmniProcessor.from_pretrained("Qwen/Qwen2.5-Omni-7B")

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
            {"type": "text", "text": "What can't you hear and see in this video?"},
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

    # kwargs to be passed to `Qwen2-5-OmniProcessor`
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

To generate only text output and save compute by not loading the audio generation model, we can use `Qwen2_5OmniThinkerForConditionalGeneration` model.  

```python
from transformers import Qwen2_5OmniProcessor, Qwen2_5OmniThinkerForConditionalGeneration

model = Qwen2_5OmniThinkerForConditionalGeneration.from_pretrained(
    "Qwen/Qwen2.5-Omni-7B",
    device_map="auto",
)
processor = Qwen2_5OmniProcessor.from_pretrained("Qwen/Qwen2.5-Omni-7B")

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
            {"type": "text", "text": "What can't you hear and see in this video?"},
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

    # kwargs to be passed to `Qwen2-5-OmniProcessor`
    padding=True,
    use_audio_in_video=True,
).to(model.device)

text_ids = model.generate(**inputs, use_audio_in_video=True)
text = processor.batch_decode(text_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)

print(text)
```

### Batch Mixed Media Inference

The model can batch inputs composed of mixed samples of various types such as text, images, audio and videos as input when using `Qwen2_5OmniThinkerForConditionalGeneration` model. Here is an example.

```python
from transformers import Qwen2_5OmniForConditionalGeneration, Qwen2_5OmniProcessor

model = Qwen2_5OmniForConditionalGeneration.from_pretrained(
    "Qwen/Qwen2.5-Omni-7B",
    device_map="auto"
)
processor = Qwen2_5OmniProcessor.from_pretrained("Qwen/Qwen2.5-Omni-7B")

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
            {"type": "text", "text": "What are the elements can you see and hear in these media?"},
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

    # kwargs to be passed to `Qwen2-5-OmniProcessor`
    padding=True,
    use_audio_in_video=True,
).to(model.thinker.device)

text_ids = model.generate(**inputs, use_audio_in_video=True)
text = processor.batch_decode(text_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)

print(text)
```

### Batch audio generation

[Qwen2_5OmniForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniForConditionalGeneration) supports batched audio output generation. For example, below for text-to-speech batch generation. With a batch of more than one sample, the generated audio is returned as a list with one waveform per sample, each already trimmed to its own length, so shorter samples are not padded with spurious audio. A single-sample call still returns one tensor.

```python
import soundfile as sf
from transformers import AutoModelForTextToWaveform, AutoProcessor
import torch

model_id = "Qwen/Qwen2.5-Omni-7B"
model = AutoModelForTextToWaveform.from_pretrained(model_id, device_map="auto")
processor = AutoProcessor.from_pretrained(model_id)
sampling_rate = 24000  # output sampling rate
max_new_tokens = 200  # maximum number of tokens to generate

system_text = (
    "You are Qwen, a virtual human developed by the Qwen Team, Alibaba Group, capable of "
    "perceiving auditory and visual inputs, as well as generating text and speech."
)
texts = [
    "Hello, I'm Qwen. How can I help you today?",
    "The weather is nice today. Let's go for a walk.",
]
inputs = processor.apply_chat_template(
    [
        [
            {"role": "system", "content": [{"type": "text", "text": system_text}]},
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": (
                            "Please read the following text aloud exactly as written, with no "
                            f"additional commentary:\n\n{text}"
                        ),
                    }
                ],
            },
        ]
        for text in texts
    ],
    tokenize=True,
    add_generation_prompt=True,
    return_dict=True,
    return_tensors="pt",
    processor_kwargs={"padding": True},
).to(model.device, dtype=model.dtype)

gen_kwargs = {
    "talker_do_sample": True,
    "speaker": "Ethan",  # Ethan, Chelsie
    "talker_max_new_tokens": max_new_tokens,
}
_, pred_waveform = model.generate(**inputs, **gen_kwargs)

for sample_id, audio in enumerate(pred_waveform):
    sf.write(
        f"qwen2_5_omni_output_{sample_id}.wav",
        audio.detach().to(torch.float32).cpu().numpy(),
        sampling_rate,
    )
    print(f"Saved audio to qwen2_5_omni_output_{sample_id}.wav")
```

### Usage Tips

#### Image Resolution trade-off

The model supports a wide range of resolution inputs. By default, it uses the native resolution for input, but higher resolutions can enhance performance at the cost of more computation. Users can set the minimum and maximum number of pixels to achieve an optimal configuration for their needs.

```python
min_pixels = 128*28*28
max_pixels = 768*28*28
processor = AutoProcessor.from_pretrained("Qwen/Qwen2.5-Omni-7B", min_pixels=min_pixels, max_pixels=max_pixels)
```

#### Prompt for audio output

If users need audio output, the system prompt must be set as "You are Qwen, a virtual human developed by the Qwen Team, Alibaba Group, capable of perceiving auditory and visual inputs, as well as generating text and speech.", otherwise the audio output may not work as expected.

```python
{
    "role": "system",
    "content": "You are Qwen, a virtual human developed by the Qwen Team, Alibaba Group, capable of perceiving auditory and visual inputs, as well as generating text and speech.",
}
```

#### Use audio output or not

The model supports both text and audio outputs, if users do not need audio outputs, they can set `enable_audio_output` in the `from_pretrained` function. This option will save about `~2GB` of GPU memory but the `return_audio` option for `generate` function will only allow to be set at `False`.

```python
model = Qwen2_5OmniForConditionalGeneration.from_pretrained(
    "Qwen/Qwen2.5-Omni-7B",
    device_map="auto",
    enable_audio_output=False,
)
```

In order to obtain a flexible experience, we recommend that users set `enable_audio_output` at `True` when initializing the model through `from_pretrained` function, and then decide whether to return audio when `generate` function is called. When `return_audio` is set to `False`, the model will only return text outputs to get text responses faster.

```python
model = Qwen2_5OmniForConditionalGeneration.from_pretrained(
    "Qwen/Qwen2.5-Omni-7B",
    device_map="auto",
    enable_audio_output=True,
)
...
text_ids = model.generate(**inputs, return_audio=False)
```

#### Change voice type of output audio

Qwen2.5-Omni supports the ability to change the voice of the output audio. Users can use the `speaker` parameter of `generate` function to specify the voice type. The `"Qwen/Qwen2.5-Omni-7B"` checkpoint support two voice types: `Chelsie` and `Ethan`, while `Chelsie` is a female voice and `Ethan` is a male voice. By default, if `speaker` is not specified, the default voice type is `Chelsie`.

```python
text_ids, audio = model.generate(**inputs, speaker="Chelsie")
```

```python
text_ids, audio = model.generate(**inputs, speaker="Ethan")
```

#### Flash-Attention 2 to speed up generation

First, make sure to install the latest version of Flash Attention 2:

```bash
pip install -U flash-attn --no-build-isolation
```

Also, you should have hardware that is compatible with FlashAttention 2. Read more about it in the official documentation of the [flash attention repository](https://github.com/Dao-AILab/flash-attention). FlashAttention-2 can only be used when a model is loaded in `torch.float16` or `torch.bfloat16`.

To load and run a model using FlashAttention-2, add `attn_implementation="flash_attention_2"` when loading the model:

```python
from transformers import Qwen2_5OmniForConditionalGeneration

model = Qwen2_5OmniForConditionalGeneration.from_pretrained(
    "Qwen/Qwen2.5-Omni-7B",
    device_map="auto",
    attn_implementation="flash_attention_2",
)
```

## Qwen2_5OmniConfig[[transformers.Qwen2_5OmniConfig]]

#### transformers.Qwen2_5OmniConfig[[transformers.Qwen2_5OmniConfig]]

```python
transformers.Qwen2_5OmniConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, thinker_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, talker_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, token2wav_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, enable_audio_output: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/configuration_qwen2_5_omni.py#L564)

**Parameters:**

thinker_config (`dict`, *optional*) : Configuration of the underlying thinker sub-model.

talker_config (`dict`, *optional*) : Configuration of the underlying talker sub-model.

token2wav_config (`dict`, *optional*) : Configuration of the underlying codec sub-model.

enable_audio_output (`bool`, *optional*, defaults to `True`) : Whether enable audio output and load talker and token2wav module.

This is the configuration class to store the configuration of a Qwen2 5 OmniModel. It is used to instantiate a Qwen2 5 Omni
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Qwen/Qwen2.5-Omni-7B](https://huggingface.co/Qwen/Qwen2.5-Omni-7B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import (
...     Qwen2_5OmniThinkerConfig,
...     Qwen2_5OmniTalkerConfig,
...     Qwen2_5OmniToken2WavConfig,
...     Qwen2_5OmniForConditionalGeneration,
...     Qwen2_5OmniConfig,
... )

>>> # Initializing sub-modules configurations.
>>> thinker_config = Qwen2_5OmniThinkerConfig()
>>> talker_config = Qwen2_5OmniTalkerConfig()
>>> token2wav_config = Qwen2_5OmniToken2WavConfig()

>>> # Initializing a module style configuration
>>> configuration = Qwen2_5OmniConfig(
...     thinker_config, talker_config, token2wav_config
... )

>>> # Initializing a model (with random weights)
>>> model = Qwen2_5OmniForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

#### get_text_config[[transformers.Qwen2_5OmniConfig.get_text_config]]

```python
get_text_config(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/configuration_qwen2_5_omni.py#L638)

**Parameters:**

decoder (`Optional[bool]`, *optional*, defaults to `False`) : If set to `True`, then only search for decoder config names.

Returns the config that is meant to be used with text IO. On most models, it is the original config instance
itself. On specific composite models, it is under a set of valid names.

## Qwen2_5OmniVisionEncoderConfig[[transformers.Qwen2_5OmniVisionEncoderConfig]]

#### transformers.Qwen2_5OmniVisionEncoderConfig[[transformers.Qwen2_5OmniVisionEncoderConfig]]

```python
transformers.Qwen2_5OmniVisionEncoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, depth: int = 32, hidden_size: int = 3584, hidden_act: str = 'silu', intermediate_size: int = 3420, num_heads: int = 16, in_channels: int = 3, patch_size: int | list[int] | tuple[int, int] = 14, spatial_merge_size: int = 2, temporal_patch_size: int | list[int] | tuple[int, int] = 2, window_size: int = 112, out_hidden_size: int = 3584, fullatt_block_indexes: list[int] | tuple[int, ...] = (7, 15, 23, 31), initializer_range: float = 0.02)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/configuration_qwen2_5_omni.py#L33)

**Parameters:**

depth (`int`, *optional*, defaults to `32`) : Number of Transformer layers in the vision encoder.

hidden_size (`int`, *optional*, defaults to `3584`) : Dimension of the hidden representations.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

intermediate_size (`int`, *optional*, defaults to `3420`) : Dimension of the MLP representations.

num_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer decoder.

in_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `14`) : The size (resolution) of each patch.

spatial_merge_size (`int`, *optional*, defaults to `2`) : The size of the spatial merge window used to reduce the number of visual tokens by merging neighboring patches.

temporal_patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `2`) : Temporal patch size used in the 3D patch embedding for video inputs.

window_size (`int`, *optional*, defaults to 11) : Size of windows.

out_hidden_size (`int`, *optional*, defaults to 3584) : The output hidden size of the vision model.

fullatt_block_indexes (`int`, *optional*, defaults to `[7, 15, 23, 31]`) : Indices of layers with full attention

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Qwen2 5 OmniModel. It is used to instantiate a Qwen2 5 Omni
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Qwen/Qwen2.5-Omni-7B](https://huggingface.co/Qwen/Qwen2.5-Omni-7B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Qwen2_5OmniVisionEncoderConfig, Qwen2_5OmniVisionEncoder

>>> # Initializing a Qwen2_5OmniVisionEncoderConfig
>>> configuration = Qwen2_5OmniVisionEncoderConfig()

>>> # Initializing a Qwen2_5OmniVisionEncoder (with random weights)
>>> model = Qwen2_5OmniVisionEncoder(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Qwen2_5OmniTextConfig[[transformers.Qwen2_5OmniTextConfig]]

#### transformers.Qwen2_5OmniTextConfig[[transformers.Qwen2_5OmniTextConfig]]

```python
transformers.Qwen2_5OmniTextConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 152064, hidden_size: int = 3584, intermediate_size: int = 18944, num_hidden_layers: int = 28, num_attention_heads: int = 28, num_key_value_heads: int | None = 4, hidden_act: str = 'silu', max_position_embeddings: int = 32768, initializer_range: float = 0.02, rms_norm_eps: float = 1e-06, use_cache: bool = True, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, use_sliding_window: bool = False, sliding_window: int | None = 32768, max_window_layers: int = 28, layer_types: list[str] | None = None, attention_dropout: float | int = 0.0, pad_token_id: int | None = None, bos_token_id: int | None = None, eos_token_id: int | list[int] | None = None, tie_word_embeddings: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/configuration_qwen2_5_omni.py#L128)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `152064`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `3584`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `18944`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `28`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `28`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `4`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `32768`) : The maximum sequence length that this model might ever be used with.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

rms_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the rms normalization layers.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

use_sliding_window (`bool`, *optional*, defaults to `False`) : Whether to use sliding window attention.

sliding_window (`int`, *optional*, defaults to `32768`) : Sliding window attention window size. If `None`, no sliding window is applied.

max_window_layers (`int`, *optional*, defaults to `28`) : The number of layers using full attention. The first `max_window_layers` layers will use full attention, while any additional layer afterwards will use SWA (Sliding Window Attention).

layer_types (`list[str]`, *optional*) : A list that explicitly maps each layer index with its layer type. If not provided, it will be automatically generated based on config values.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

pad_token_id (`int`, *optional*) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*) : Token id used for end-of-stream in the vocabulary.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Qwen2 5 OmniModel. It is used to instantiate a Qwen2 5 Omni
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Qwen/Qwen2.5-Omni-7B](https://huggingface.co/Qwen/Qwen2.5-Omni-7B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Qwen2_5OmniThinkerForConditionalGeneration, Qwen2_5OmniThinkerConfig, Qwen2_5OmniAudioEncoderConfig, Qwen2_5OmniVisionEncoderConfig

>>> # Initializing a Qwen2_5OmniAudioEncoder config
>>> audio_config = Qwen2_5OmniAudioEncoderConfig()

>>> # Initializing a Qwen2_5OmniVisionEncoder config
>>> vision_config = Qwen2_5OmniVisionEncoderConfig()

>>> # Initializing a Qwen2.5OmniThinker configuration
>>> configuration = Qwen2_5OmniThinkerConfig(audio_config, vision_config)

>>> # Initializing a model from the Qwen-Omni style configuration
>>> model = Qwen2_5OmniThinkerForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Qwen2_5OmniDiTConfig[[transformers.Qwen2_5OmniDiTConfig]]

#### transformers.Qwen2_5OmniDiTConfig[[transformers.Qwen2_5OmniDiTConfig]]

```python
transformers.Qwen2_5OmniDiTConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 1024, num_hidden_layers: int = 22, num_attention_heads: int = 16, ff_mult: int = 2, emb_dim: int = 512, head_dim: int = 64, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, max_position_embeddings: int = 32768, block_size: int = 24, look_ahead_layers: list[int] | tuple[int, ...] = (10,), look_backward_layers: list[int] | tuple[int, ...] = (0, 20), repeats: int = 2, num_embeds: int = 8193, mel_dim: int = 80, dropout: float | int = 0.1, enc_emb_dim: int = 192, enc_dim: int = 128, enc_channels: list[int] | tuple[int, ...] = (256, 256, 256, 256, 768), enc_kernel_sizes: list[int] | tuple[int, ...] = (5, 3, 3, 3, 1), enc_dilations: list[int] | tuple[int, ...] = (1, 2, 3, 4, 1), enc_attention_channels: int = 64, enc_res2net_scale: int = 2, enc_se_channels: int = 64)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/configuration_qwen2_5_omni.py#L409)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `1024`) : Dimension of the hidden representations.

num_hidden_layers (`int`, *optional*, defaults to `22`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer decoder.

ff_mult (`int`, *optional*, defaults to 2) : The multiplier for the feedforward layer in each transformer block.

emb_dim (`int`, *optional*, defaults to 512) : The dimension of the embedding layer.

head_dim (`int`, *optional*, defaults to `64`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

max_position_embeddings (`int`, *optional*, defaults to `32768`) : The maximum sequence length that this model might ever be used with.

block_size (`int`, *optional*, defaults to 64) : Number of tokens (frames) in each processing block.

look_ahead_layers (`list[int]`, *optional*, defaults to `[10]`) : Number of transformer layers that are permitted to attend to future blocks

look_backward_layers (`list[int]`, *optional*, defaults to `[0, 20]`) : Number of transformer layers that attend to past blocks beyond the current block boundary

repeats (`int`, *optional*, defaults to 2) : The number of times the codec embeddings are repeated.

num_embeds (`int`, *optional*, defaults to 8193) : The number of unique embeddings in the codec.

mel_dim (`int`, *optional*, defaults to 80) : The dimension of the mel-spectrogram.

dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The ratio for all dropout layers.

enc_emb_dim (`int`, *optional*, defaults to 192) : The dimension of the pre-trained speaker embedding.

enc_dim (`int`, *optional*, defaults to 128) : The dimension of the encoder output.

enc_channels (`list[int]`, *optional*, defaults to `[256, 256, 256, 256, 768]`) : A list of output channels for each TDNN/SERes2Net layer in the encoder.

enc_kernel_sizes (`list[int]`, *optional*, defaults to `[5, 3, 3, 3, 1]`) : A list of kernel sizes for each layer in the encoder.

enc_dilations (`list[int]`, *optional*, defaults to `[1, 2, 3, 4, 1]`) : A list of dilations for each layer in the encoder.

enc_attention_channels (`int`, *optional*, defaults to 64) : The number of attention channels in the SqueezeExcitationBlock.

enc_res2net_scale (`int`, *optional*, defaults to 2) : The scale of the Res2Net block in the encoder.

enc_se_channels (`int`, *optional*, defaults to 64) : The number of output channels after squeeze in the SqueezeExcitationBlock.

This is the configuration class to store the configuration of a Qwen2 5 OmniModel. It is used to instantiate a Qwen2 5 Omni
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Qwen/Qwen2.5-Omni-7B](https://huggingface.co/Qwen/Qwen2.5-Omni-7B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Qwen2_5OmniBigVGANConfig[[transformers.Qwen2_5OmniBigVGANConfig]]

#### transformers.Qwen2_5OmniBigVGANConfig[[transformers.Qwen2_5OmniBigVGANConfig]]

```python
transformers.Qwen2_5OmniBigVGANConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, mel_dim: int = 80, upsample_initial_channel: int = 1536, resblock_kernel_sizes: list[int] | tuple[int, ...] = (3, 7, 11), resblock_dilation_sizes: list | tuple = ((1, 3, 5), (1, 3, 5), (1, 3, 5)), upsample_rates: list[int] | tuple[int, ...] = (5, 3, 2, 2, 2, 2), upsample_kernel_sizes: list[int] | tuple[int, ...] = (11, 7, 4, 4, 4, 4))
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/configuration_qwen2_5_omni.py#L474)

**Parameters:**

mel_dim (`int`, *optional*, defaults to 80) : The dimension of the mel-spectrogram.

upsample_initial_channel (`int`, *optional*, defaults to 1536) : The number of channels in the initial upsampling layer.

resblock_kernel_sizes (`list[int]`, *optional*, defaults to `[3, 7, 11]`) : A list of kernel sizes for each residual block.

resblock_dilation_sizes (`list[list[int]]`, *optional*, defaults to `[[1, 3, 5], [1, 3, 5], [1, 3, 5]]`) : A list of dilation sizes for each residual block.

upsample_rates (`list[int]`, *optional*, defaults to `[5, 3, 2, 2, 2, 2]`) : A list of upsampling rates for each upsampling layer.

upsample_kernel_sizes (`list[int]`, *optional*, defaults to `[11, 7, 4, 4, 4, 4]`) : A list of kernel sizes for each upsampling layer.

This is the configuration class to store the configuration of a Qwen2 5 OmniModel. It is used to instantiate a Qwen2 5 Omni
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Qwen/Qwen2.5-Omni-7B](https://huggingface.co/Qwen/Qwen2.5-Omni-7B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Qwen2_5OmniAudioEncoderConfig[[transformers.Qwen2_5OmniAudioEncoderConfig]]

#### transformers.Qwen2_5OmniAudioEncoderConfig[[transformers.Qwen2_5OmniAudioEncoderConfig]]

```python
transformers.Qwen2_5OmniAudioEncoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, num_mel_bins: int = 128, encoder_layers: int = 32, encoder_attention_heads: int = 20, encoder_ffn_dim: int = 5120, d_model: int = 1280, dropout: float | int = 0.0, attention_dropout: float | int = 0.0, activation_function: str = 'gelu', activation_dropout: float | int = 0.0, scale_embedding: bool = False, initializer_range: float = 0.02, max_source_positions: int = 1500, n_window: int = 100, output_dim: int = 3584)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/configuration_qwen2_5_omni.py#L77)

**Parameters:**

num_mel_bins (`int`, *optional*, defaults to `128`) : Number of mel features used per input frame. Should correspond to the value used in the `AutoFeatureExtractor` class.

encoder_layers (`int`, *optional*, defaults to `32`) : Number of hidden layers in the Transformer encoder. Will use the same value as `num_layers` if not set.

encoder_attention_heads (`int`, *optional*, defaults to `20`) : Number of attention heads for each attention layer in the Transformer encoder.

encoder_ffn_dim (`int`, *optional*, defaults to `5120`) : Dimensionality of the "intermediate" (often named feed-forward) layer in encoder.

d_model (`int`, *optional*, defaults to `1280`) : Size of the encoder layers and the pooler layer.

dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The ratio for all dropout layers.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

activation_function (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

activation_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for activations inside the fully connected layer.

scale_embedding (`bool`, *optional*, defaults to `False`) : Whether to scale embeddings by dividing by sqrt(d_model).

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

max_source_positions (`int`, *optional*, defaults to 1500) : The maximum sequence length of log-mel filter-bank features that this model might ever be used with.

n_window (`int`, *optional*, defaults to 100) : The chunk for conv and flash attn in AudioEncoder.

output_dim (`int`, *optional*, defaults to 3584) : The output dimension of AudioEncoder.

This is the configuration class to store the configuration of a Qwen2 5 OmniModel. It is used to instantiate a Qwen2 5 Omni
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Qwen/Qwen2.5-Omni-7B](https://huggingface.co/Qwen/Qwen2.5-Omni-7B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Qwen2_5OmniAudioEncoderConfig, Qwen2_5OmniAudioEncoder

>>> # Initializing a Qwen2_5OmniAudioEncoderConfig
>>> configuration = Qwen2_5OmniAudioEncoderConfig()

>>> # Initializing a Qwen2_5OmniAudioEncoder (with random weights)
>>> model = Qwen2_5OmniAudioEncoder(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Qwen2_5OmniProcessor[[transformers.Qwen2_5OmniProcessor]]

#### transformers.Qwen2_5OmniProcessor[[transformers.Qwen2_5OmniProcessor]]

```python
transformers.Qwen2_5OmniProcessor(image_processor = None, video_processor = None, feature_extractor = None, tokenizer = None, chat_template = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/processing_qwen2_5_omni.py#L105)

**Parameters:**

image_processor (`Qwen2VLImageProcessor`) : The image processor is a required input.

video_processor (`Qwen2VLVideoProcessor`) : The video processor is a required input.

feature_extractor (`WhisperFeatureExtractor`) : The feature extractor is a required input.

tokenizer (`Qwen2Tokenizer`) : The tokenizer is a required input.

chat_template (`str`) : A Jinja template to convert lists of messages in a chat into a tokenizable string.

Constructs a Qwen2_5OmniProcessor which wraps a image processor, a video processor, a feature extractor, and a tokenizer into a single processor.

[Qwen2_5OmniProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniProcessor) offers all the functionalities of [Qwen2VLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLImageProcessor), [Qwen2VLVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLVideoProcessor), [WhisperFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/whisper#transformers.WhisperFeatureExtractor), and [Qwen2Tokenizer](/docs/transformers/v5.15.1/en/model_doc/qwen2#transformers.Qwen2Tokenizer). See the
[~Qwen2VLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLImageProcessor), [~Qwen2VLVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLVideoProcessor), [~WhisperFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/whisper#transformers.WhisperFeatureExtractor), and [~Qwen2Tokenizer](/docs/transformers/v5.15.1/en/model_doc/qwen2#transformers.Qwen2Tokenizer) for more information.

#### __call__[[transformers.Qwen2_5OmniProcessor.__call__]]

```python
__call__(text: str | list[str] | list[list[str]] = None, images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, videos: typing.Union[list['PIL.Image.Image'], numpy.ndarray, ForwardRef('torch.Tensor'), list[numpy.ndarray], list['torch.Tensor'], list[list['PIL.Image.Image']], list[list[numpy.ndarray]], list[list['torch.Tensor']], transformers.video_utils.URL, list[transformers.video_utils.URL], list[list[transformers.video_utils.URL]], transformers.video_utils.Path, list[transformers.video_utils.Path], list[list[transformers.video_utils.Path]], NoneType] = None, audio: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/processing_qwen2_5_omni.py#L120)

**Parameters:**

text (`Union[str, list[str], list[list[str]]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

videos (`Union[list[PIL.Image.Image], numpy.ndarray, torch.Tensor, list[numpy.ndarray], list[torch.Tensor], list[list[PIL.Image.Image]], list[list[numpy.ndarray]], list[list[torch.Tensor]], ~video_utils.URL, list[~video_utils.URL], list[list[~video_utils.URL]], ~video_utils.Path, list[~video_utils.Path], list[list[~video_utils.Path]]]`, *optional*) : Video to preprocess. Expects a single or batch of videos with pixel values ranging from 0 to 255. If passing in videos with pixel values between 0 and 1, set `do_rescale=False`.

audio (`Union[numpy.ndarray, torch.Tensor, list[numpy.ndarray], list[torch.Tensor]]`, *optional*) : The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor. In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels, and T is the sample length of the audio.

min_pixels (`int`, *kwargs*, *optional*) : Minimum number of pixels (height × width) for video frames after resizing. Frames smaller than this threshold will be upscaled to meet the minimum requirement.

max_pixels (`int`, *kwargs*, *optional*) : Maximum number of pixels (height × width) for video frames after resizing. Frames larger than this threshold will be downscaled to fit within the limit.

patch_size (`int`, *kwargs*, *optional*) : The spatial patch size used by the vision encoder. Video frames are divided into patches of this size in both height and width dimensions.

temporal_patch_size (`int`, *kwargs*, *optional*) : The temporal patch size used by the vision encoder. This determines how many consecutive frames are grouped together as a single temporal patch.

merge_size (`int`, *kwargs*, *optional*) : The merge size used for combining spatial patches. Multiple patches are merged together to reduce the sequence length while maintaining spatial information.

min_frames (`int`, *kwargs*, *optional*) : Minimum number of frames to extract from the video. Videos with fewer frames will be padded or repeated to meet this requirement.

max_frames (`int`, *kwargs*, *optional*) : Maximum number of frames to extract from the video. Longer videos will be truncated or sampled to fit within this limit.

use_audio_in_video (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to incorporate audio information when processing videos. When enabled, audio tokens are interleaved with video tokens based on temporal alignment, creating a unified multimodal representation.

seconds_per_chunk (`float`, *kwargs*, *optional*, defaults to `2.0`) : The duration (in seconds) of each video chunk when splitting long videos. This parameter controls how videos are divided into temporal segments for processing.

position_id_per_seconds (`int`, *kwargs* or `float`, *optional*, defaults to `25`) : The number of position IDs allocated per second of video. This parameter controls the temporal resolution of position embeddings and is used to align video tokens with audio tokens when `use_audio_in_video=True`.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) : Additional processing options for each modality (text, images, videos, audio). Model-specific parameters are listed above; see the TypedDict class for the complete list of supported arguments.

**Returns:** `~feature_extraction_utils.BatchFeature`

- **data** (`dict`, *optional*) -- Dictionary of lists/arrays/tensors returned by the __call__/pad methods ('input_values', 'attention_mask',
  etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.
- **skip_tensor_conversion** (`list[str]` or `set[str]`, *optional*) -- List or set of keys that should NOT be converted to tensors, even when `tensor_type` is specified.

## Qwen2_5OmniForConditionalGeneration[[transformers.Qwen2_5OmniForConditionalGeneration]]

#### transformers.Qwen2_5OmniForConditionalGeneration[[transformers.Qwen2_5OmniForConditionalGeneration]]

```python
transformers.Qwen2_5OmniForConditionalGeneration(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/modeling_qwen2_5_omni.py#L3622)

**Parameters:**

config ([Qwen2_5OmniForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniForConditionalGeneration)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The full Qwen2.5Omni model, a multimodal model composed of 3 sub-models:
- [Qwen2_5OmniThinkerForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniThinkerForConditionalGeneration):
a causal auto-regressive transformer takes text, audio, image, video as input and predict text tokens.
- [Qwen2_5OmniTalkerForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniTalkerForConditionalGeneration):
a causal auto-regressive transformer takes thinker hidden states and response as input and predict speech tokens.
- [Qwen2_5OmniToken2WavModel](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniToken2WavModel):
a DiT model take speech tokens as input and predict mel spectrogram and a BigVGAN vocoder take mel spectrogram as input and predict waveform.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Qwen2_5OmniForConditionalGeneration.forward]]

```python
forward(*args, **kwargs)
```

A mock value for a dotted path (e.g. `torch.float32`): attribute access chains,
calls behave as pass-through decorators, `repr` is the dotted path, and using it
as a base class substitutes a plain-`type` base (PEP 560 `__mro_entries__`), so
real subclasses keep a normal metaclass and `inspect.signature` reads their real
`__init__` instead of a mock's.

## Qwen2_5OmniPreTrainedModelForConditionalGeneration[[transformers.Qwen2_5OmniPreTrainedModelForConditionalGeneration]]

#### transformers.Qwen2_5OmniPreTrainedModelForConditionalGeneration[[transformers.Qwen2_5OmniPreTrainedModelForConditionalGeneration]]

```python
transformers.Qwen2_5OmniPreTrainedModelForConditionalGeneration(config: PreTrainedConfig, *inputs, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/modeling_qwen2_5_omni.py#L161)

#### get_chunked_index[[transformers.Qwen2_5OmniPreTrainedModelForConditionalGeneration.get_chunked_index]]

```python
get_chunked_index(token_indices: Tensor, tokens_per_chunk: int, remove_index: int)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/modeling_qwen2_5_omni.py#L184)

**Parameters:**

token_indices (`torch.Tensor` of shape `(seq_len, )`) : A monotonically increasing list of token index values.

t_ntoken_per_chunk (`int`) : Number of tokens per chunk (used as the chunk size threshold).

remove_index (`int`) An index id to subtract from `token_indices` before chunking --

**Returns:** `list[tuple[int, int]]`

A list of tuples, each representing the start (inclusive)
and end (exclusive) indices of a chunk in `token_indices`.

Splits token index list into chunks based on token value ranges.

Given a list of token indices, returns a list of (start, end) index tuples representing
slices of the list where the token values fall within successive ranges of `t_ntoken_per_chunk`.

For example, if `t_ntoken_per_chunk` is 1000, the function will create chunks such that:
- the first chunk contains token values < 1000,
- the second chunk contains values >= 1000 and < 2000, and so on.

#### get_rope_index[[transformers.Qwen2_5OmniPreTrainedModelForConditionalGeneration.get_rope_index]]

```python
get_rope_index(input_ids: typing.Optional[torch.LongTensor] = None, image_grid_thw: typing.Optional[torch.LongTensor] = None, video_grid_thw: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, use_audio_in_video: bool = False, audio_seqlens: typing.Optional[torch.LongTensor] = None, second_per_grids: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/modeling_qwen2_5_omni.py#L221)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default should you provide it.

image_grid_thw (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) : The temporal, height and width of feature shape of each image in LLM.

video_grid_thw (`torch.LongTensor` of shape `(num_videos, 3)`, *optional*) : The temporal, height and width of feature shape of each video in LLM.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.

use_audio_in_video (`bool`, *optional*) : If set to `True`, use the audio in video.

audio_seqlens (`torch.LongTensor` of shape `(num_audios)`, *optional*) : The length of feature shape of each audio in LLM.

second_per_grids (`torch.LongTensor` of shape `(num_videos)`, *optional*) : The time interval (in seconds) for each grid along the temporal dimension in the 3D position IDs.

**Returns:**

position_ids (`torch.LongTensor` of shape `(3, batch_size, sequence_length)`)
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
interval: The step size for the temporal position IDs, calculated as tokens_per_second * temporal_patch_size / fps. In this case, 25 * 2 / 1 = 50. This means that each temporal patch will have a difference of 50 in the temporal position IDs.
input_ids: [V V V V V V V V V V V V T T T T T], here V is for vision.
vision temporal position_ids: [0, 0, 0, 0, 50, 50, 50, 50, 100, 100, 100, 100]
vision height position_ids: [0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1]
vision width position_ids: [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]
text temporal position_ids: [101, 102, 103, 104, 105]
text height position_ids: [101, 102, 103, 104, 105]
text width position_ids: [101, 102, 103, 104, 105]
Here we calculate the text start position_ids as the max vision position_ids plus 1.

## Qwen2_5OmniThinkerConfig[[transformers.Qwen2_5OmniThinkerConfig]]

#### transformers.Qwen2_5OmniThinkerConfig[[transformers.Qwen2_5OmniThinkerConfig]]

```python
transformers.Qwen2_5OmniThinkerConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, audio_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, vision_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, audio_token_index: int = 151646, image_token_index: int = 151655, video_token_index: int = 151656, position_id_per_seconds: int = 25, seconds_per_chunk: int = 2, audio_start_token_id: int = 151647, audio_end_token_id: int = 151648, user_token_id: int = 872, initializer_range: float = 0.02, tie_word_embeddings: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/configuration_qwen2_5_omni.py#L212)

**Parameters:**

audio_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the audio backbone.

vision_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the vision backbone.

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

audio_token_index (`int`, *optional*, defaults to `151646`) : The audio token index used as a placeholder for input audio.

image_token_index (`int`, *optional*, defaults to `151655`) : The image token index used as a placeholder for input images.

video_token_index (`int`, *optional*, defaults to `151656`) : The video token index used as a placeholder for input videos.

position_id_per_seconds (`int`, *optional*, defaults to 25) : The increment of position id per second.

seconds_per_chunk (`int`, *optional*, defaults to 2) : The duration in seconds of the chunk of audio and video data.

audio_start_token_id (`int`, *optional*, defaults to 151647) : The audio start token index to encode the audio prompt.

audio_end_token_id (`int`, *optional*, defaults to 151648) : The audio end token index to encode the audio prompt.

user_token_id (`int, *optional*, defaults to 872) : The user token index to encode the user token.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Qwen2 5 OmniModel. It is used to instantiate a Qwen2 5 Omni
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Qwen/Qwen2.5-Omni-7B](https://huggingface.co/Qwen/Qwen2.5-Omni-7B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Qwen2_5OmniThinkerForConditionalGeneration, Qwen2_5OmniThinkerConfig, Qwen2_5OmniAudioEncoderConfig, Qwen2_5OmniVisionEncoderConfig

>>> # Initializing a Qwen2_5OmniAudioEncoder config
>>> audio_config = Qwen2_5OmniAudioEncoderConfig()

>>> # Initializing a Qwen2_5OmniVisionEncoder config
>>> vision_config = Qwen2_5OmniVisionEncoderConfig()

>>> # Initializing a Qwen2_5OmniTextConfig config
>>> text_config = Qwen2_5OmniTextConfig()

>>> # Initializing a Qwen2.5OmniThinker configuration
>>> configuration = Qwen2_5OmniThinkerConfig(audio_config, vision_config, text_config)

>>> # Initializing a model from the Qwen-Omni style configuration
>>> model = Qwen2_5OmniThinkerForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Qwen2_5OmniThinkerForConditionalGeneration[[transformers.Qwen2_5OmniThinkerForConditionalGeneration]]

#### transformers.Qwen2_5OmniThinkerForConditionalGeneration[[transformers.Qwen2_5OmniThinkerForConditionalGeneration]]

```python
transformers.Qwen2_5OmniThinkerForConditionalGeneration(config: Qwen2_5OmniThinkerConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/modeling_qwen2_5_omni.py#L1720)

**Parameters:**

config ([Qwen2_5OmniThinkerConfig](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniThinkerConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Qwen2.5OmniThinker model which consists of an audio backbone and a language model.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Qwen2_5OmniThinkerForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, input_features: typing.Optional[torch.FloatTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, pixel_values_videos: typing.Optional[torch.FloatTensor] = None, image_grid_thw: typing.Optional[torch.LongTensor] = None, video_grid_thw: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, feature_attention_mask: typing.Optional[torch.Tensor] = None, audio_feature_lengths: typing.Optional[torch.LongTensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, use_audio_in_video: bool | None = None, video_second_per_grid: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/modeling_qwen2_5_omni.py#L1865)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) : The tensors corresponding to the input audio features. Audio features can be obtained using [WhisperFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/whisper#transformers.WhisperFeatureExtractor). See [WhisperFeatureExtractor.__call__()](/docs/transformers/v5.15.1/en/model_doc/whisper#transformers.WhisperFeatureExtractor.__call__) for details ([Qwen2_5OmniProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniProcessor) uses [WhisperFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/whisper#transformers.WhisperFeatureExtractor) for processing audios).

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [Qwen2VLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLImageProcessor). See `Qwen2VLImageProcessor.__call__()` for details ([Qwen2_5OmniProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniProcessor) uses [Qwen2VLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLImageProcessor) for processing images).

pixel_values_videos (`torch.FloatTensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`, *optional*) : The tensors corresponding to the input video. Pixel values for videos can be obtained using [Qwen2VLVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLVideoProcessor). See `Qwen2VLVideoProcessor.__call__()` for details ([Qwen2_5OmniProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniProcessor) uses [Qwen2VLVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLVideoProcessor) for processing videos).

image_grid_thw (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) : The temporal, height and width of feature shape of each image in LLM.

video_grid_thw (`torch.LongTensor` of shape `(num_videos, 3)`, *optional*) : The temporal, height and width of feature shape of each video in LLM.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

feature_attention_mask (`torch.Tensor` of shape `(batch_size, feature_sequence_length)`, *optional*) : Mask to avoid performing attention on padding feature indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.

audio_feature_lengths (`torch.LongTensor` of shape `(num_audios)`, *optional*) : The length of feature shape of each audio in LLM.

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

use_audio_in_video (`bool`, *optional*) : Whether or not use audio track in video, should same as the parameter in `process_audio_info`.

video_second_per_grid (`torch.LongTensor` of shape `(num_videos)`, *optional*) : Number of seconds per grid for each video, used for temporal feature mapping.

**Returns:** `Qwen2_5OmniThinkerCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`

A `Qwen2_5OmniThinkerCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen2_5OmniConfig](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniConfig)) and inputs.

The [Qwen2_5OmniThinkerForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniThinkerForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
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
- **rope_deltas** (`torch.LongTensor` of shape `(batch_size, )`, *optional*) -- The rope index difference between sequence length and multimodal rope.
  The attribute is deprecated and will be removed in v5.20, use `model.base_model.rope_deltas` instead.

Example:

```python
>>> from io import BytesIO
>>> from urllib.request import urlopen
>>> import librosa
>>> from qwen_vl_utils import process_vision_info
>>> from transformers import Qwen2_5OmniProcessor, Qwen2_5OmniThinkerForConditionalGeneration

>>> thinker = Qwen2_5OmniThinkerForConditionalGeneration.from_pretrained("Qwen/Qwen2.5-Omni-7B")
>>> processor = Qwen2_5OmniProcessor.from_pretrained("Qwen/Qwen2.5-Omni-7B")

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

#### get_audio_features[[transformers.Qwen2_5OmniThinkerForConditionalGeneration.get_audio_features]]

```python
get_audio_features(input_features: FloatTensor, feature_attention_mask: typing.Optional[torch.LongTensor] = None, audio_feature_lengths: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/modeling_qwen2_5_omni.py#L1780)

**Parameters:**

input_features (`torch.FloatTensor`) : The tensors corresponding to the input audios.

feature_attention_mask (`torch.LongTensor`, *optional*) : Mask to avoid performing attention on padding feature indices. Mask values selected in `[0, 1]`:

audio_feature_lengths (`torch.LongTensor` of shape `(num_audios)`, *optional*) : The length of feature shape of each audio in LLM.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen2_5OmniConfig](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniConfig)) and inputs.

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
>>> from transformers import AutoProcessor, Qwen2_5OmniThinkerForConditionalGeneration

>>> model = Qwen2_5OmniThinkerForConditionalGeneration.from_pretrained("Qwen/Qwen2.5-Omni-7B")
>>> processor = AutoProcessor.from_pretrained("Qwen/Qwen2.5-Omni-7B")

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

#### get_image_features[[transformers.Qwen2_5OmniThinkerForConditionalGeneration.get_image_features]]

```python
get_image_features(pixel_values: FloatTensor, image_grid_thw: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/modeling_qwen2_5_omni.py#L1764)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [Qwen2VLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLImageProcessor). See `Qwen2VLImageProcessor.__call__()` for details ([Qwen2_5OmniProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniProcessor) uses [Qwen2VLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLImageProcessor) for processing images).

image_grid_thw (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) : The temporal, height and width of feature shape of each image in LLM.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen2_5OmniConfig](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniConfig)) and inputs.

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
>>> from transformers import AutoProcessor, Qwen2_5OmniThinkerForConditionalGeneration

>>> model = Qwen2_5OmniThinkerForConditionalGeneration.from_pretrained("Qwen/Qwen2.5-Omni-7B")
>>> processor = AutoProcessor.from_pretrained("Qwen/Qwen2.5-Omni-7B")

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

#### get_placeholder_mask[[transformers.Qwen2_5OmniThinkerForConditionalGeneration.get_placeholder_mask]]

```python
get_placeholder_mask(input_ids: LongTensor, inputs_embeds: FloatTensor, image_features: typing.Optional[torch.FloatTensor] = None, video_features: typing.Optional[torch.FloatTensor] = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/modeling_qwen2_5_omni.py#L1815)

Obtains multimodal placeholder mask from `input_ids` or `inputs_embeds`, and checks that the placeholder token count is
equal to the length of multimodal features. If the lengths are different, an error is raised.

#### get_video_features[[transformers.Qwen2_5OmniThinkerForConditionalGeneration.get_video_features]]

```python
get_video_features(pixel_values_videos: FloatTensor, video_grid_thw: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/modeling_qwen2_5_omni.py#L1748)

**Parameters:**

pixel_values_videos (`torch.FloatTensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`) : The tensors corresponding to the input video. Pixel values for videos can be obtained using [Qwen2VLVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLVideoProcessor). See `Qwen2VLVideoProcessor.__call__()` for details ([Qwen2_5OmniProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniProcessor) uses [Qwen2VLVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLVideoProcessor) for processing videos).

video_grid_thw (`torch.LongTensor` of shape `(num_videos, 3)`, *optional*) : The temporal, height and width of feature shape of each video in LLM.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen2_5OmniConfig](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniConfig)) and inputs.

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
>>> from transformers import AutoProcessor, Qwen2_5OmniThinkerForConditionalGeneration

>>> model = Qwen2_5OmniThinkerForConditionalGeneration.from_pretrained("Qwen/Qwen2.5-Omni-7B")
>>> processor = AutoProcessor.from_pretrained("Qwen/Qwen2.5-Omni-7B")

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

## Qwen2_5OmniThinkerTextModel[[transformers.Qwen2_5OmniThinkerTextModel]]

#### transformers.Qwen2_5OmniThinkerTextModel[[transformers.Qwen2_5OmniThinkerTextModel]]

```python
transformers.Qwen2_5OmniThinkerTextModel(config: Qwen2_5OmniTextConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/modeling_qwen2_5_omni.py#L1600)

**Parameters:**

config ([Qwen2_5OmniTextConfig](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniTextConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Qwen2 5 Omni Text Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Qwen2_5OmniThinkerTextModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/modeling_qwen2_5_omni.py#L1627)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen2_5OmniConfig](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniConfig)) and inputs.

The [Qwen2_5OmniThinkerTextModel](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniThinkerTextModel) forward method, overrides the `__call__` special method.

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

## Qwen2_5OmniTalkerConfig[[transformers.Qwen2_5OmniTalkerConfig]]

#### transformers.Qwen2_5OmniTalkerConfig[[transformers.Qwen2_5OmniTalkerConfig]]

```python
transformers.Qwen2_5OmniTalkerConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, audio_token_index: int = 151646, image_token_index: int = 151655, video_token_index: int = 151656, vocab_size: int = 8448, tts_text_start_token_id: int = 151860, tts_text_end_token_id: int = 151861, tts_text_pad_token_id: int = 151859, tts_codec_start_token_id: int = 8293, tts_codec_end_token_id: int = 8294, tts_codec_pad_token_id: int = 8292, tts_codec_mask_token_id: int = 8296, vision_start_token_id: int = 151652, vision_end_token_id: int = 151653, embedding_size: int = 3584, hidden_size: int = 3584, intermediate_size: int = 18944, num_hidden_layers: int = 28, num_attention_heads: int = 28, num_key_value_heads: int = 4, hidden_act: str = 'silu', max_position_embeddings: int = 32768, rms_norm_eps: float = 1e-06, head_dim: int = 128, use_cache: bool = True, tie_word_embeddings: bool = False, use_sliding_window: bool = False, sliding_window: int | None = 32768, max_window_layers: int = 28, attention_dropout: float | int = 0.0, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, position_id_per_seconds: int = 25, seconds_per_chunk: int = 2, audio_start_token_id: int = 151647, audio_end_token_id: int = 151648, initializer_range: float = 0.02, spatial_merge_size: int = 2, layer_types: list[str] | None = None, pad_token_id: int | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/configuration_qwen2_5_omni.py#L296)

**Parameters:**

audio_token_index (`int`, *optional*, defaults to `151646`) : The audio token index used as a placeholder for input audio.

image_token_index (`int`, *optional*, defaults to `151655`) : The image token index used as a placeholder for input images.

video_token_index (`int`, *optional*, defaults to `151656`) : The video token index used as a placeholder for input videos.

vocab_size (`int`, *optional*, defaults to `8448`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

tts_text_start_token_id (`int`, *optional*, defaults to 151860) : The tts text start token index to encode the start of tts text.

tts_text_end_token_id (`int`, *optional*, defaults to 151861) : The tts text end token index to encode the end of tts text.

tts_text_pad_token_id (`int`, *optional*, defaults to 151859) : The tts text pad token index to encode the pad of tts text.

tts_codec_start_token_id (`int`, *optional*, defaults to 8293) : The tts codec start token index to encode the start of tts codec.

tts_codec_end_token_id (`int`, *optional*, defaults to 8294) : The tts codec end token index to encode the end of tts codec.

tts_codec_pad_token_id (`int`, *optional*, defaults to 8292) : The tts codec pad token index to encode the pad of tts codec.

tts_codec_mask_token_id (`int`, *optional*, defaults to 8296) : The tts codec mask token index to encode the mask of tts codec.

vision_start_token_id (`int`, *optional*, defaults to `151652`) : Token ID that marks the start of a visual segment in the multimodal input sequence.

vision_end_token_id (`int`, *optional*, defaults to `151653`) : Token ID that marks the end of a visual segment in the multimodal input sequence.

embedding_size (`int`, *optional*, defaults to `3584`) : Dimensionality of the embeddings and hidden states.

hidden_size (`int`, *optional*, defaults to `3584`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `18944`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `28`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `28`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `4`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `32768`) : The maximum sequence length that this model might ever be used with.

rms_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the rms normalization layers.

head_dim (`int`, *optional*, defaults to `128`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

use_sliding_window (`bool`, *optional*, defaults to `False`) : Whether to use sliding window attention.

sliding_window (`int`, *optional*, defaults to `32768`) : Sliding window attention window size. If `None`, no sliding window is applied.

max_window_layers (`int`, *optional*, defaults to `28`) : The number of layers using full attention. The first `max_window_layers` layers will use full attention, while any additional layer afterwards will use SWA (Sliding Window Attention).

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

position_id_per_seconds (`int`, *optional*, defaults to 25) : The increment of position id per second.

seconds_per_chunk (`int`, *optional*, defaults to 2) : The duration in seconds of the chunk of audio and video data.

audio_start_token_id (`int`, *optional*, defaults to 151647) : The audio start token index to encode the audio prompt.

audio_end_token_id (`int`, *optional*, defaults to 151648) : The audio end token index to encode the audio prompt.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

spatial_merge_size (`int`, *optional*, defaults to `2`) : The size of the spatial merge window used to reduce the number of visual tokens by merging neighboring patches.

layer_types (`list[str]`, *optional*) : A list that explicitly maps each layer index with its layer type. If not provided, it will be automatically generated based on config values.

pad_token_id (`int`, *optional*) : Token id used for padding in the vocabulary.

This is the configuration class to store the configuration of a Qwen2 5 OmniModel. It is used to instantiate a Qwen2 5 Omni
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Qwen/Qwen2.5-Omni-7B](https://huggingface.co/Qwen/Qwen2.5-Omni-7B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Qwen2_5OmniTalkerForConditionalGeneration, Qwen2_5OmniThinkerConfig, Qwen2_5OmniAudioEncoderConfig, Qwen2_5OmniVisionEncoderConfig

>>> # Initializing a Qwen2_5OmniAudioEncoder config
>>> audio_config = Qwen2_5OmniAudioEncoderConfig()

>>> # Initializing a Qwen2 config
>>> text_config = Qwen2Config()

>>> # Initializing a Qwen2_5Omni configuration
>>> configuration = Qwen2_5OmniThinkerConfig(audio_config, text_config)

>>> # Initializing a model from the qwen2-audio style configuration
>>> model = Qwen2_5OmniTalkerForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Qwen2_5OmniTalkerForConditionalGeneration[[transformers.Qwen2_5OmniTalkerForConditionalGeneration]]

#### transformers.Qwen2_5OmniTalkerForConditionalGeneration[[transformers.Qwen2_5OmniTalkerForConditionalGeneration]]

```python
transformers.Qwen2_5OmniTalkerForConditionalGeneration(config: Qwen2_5OmniTalkerConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/modeling_qwen2_5_omni.py#L2166)

#### forward[[transformers.Qwen2_5OmniTalkerForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, thinker_reply_part: typing.Optional[torch.FloatTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, input_text_ids: typing.Optional[torch.LongTensor] = None, image_grid_thw: typing.Optional[torch.LongTensor] = None, video_grid_thw: typing.Optional[torch.LongTensor] = None, use_audio_in_video: bool | None = None, audio_feature_lengths: typing.Optional[torch.LongTensor] = None, video_second_per_grid: typing.Optional[torch.LongTensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/modeling_qwen2_5_omni.py#L2200)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

thinker_reply_part (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Hidden states from the thinker model's output that represent the text reply part to be processed.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

input_text_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Input token IDs for text-only content, used for position calculation in multimodal contexts.

image_grid_thw (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) : The temporal, height and width of feature shape of each image in LLM.

video_grid_thw (`torch.LongTensor` of shape `(num_videos, 3)`, *optional*) : The temporal, height and width of feature shape of each video in LLM.

use_audio_in_video (`bool`, *optional*) : Whether or not use audio track in video, should same as the parameter in `process_audio_info`.

audio_feature_lengths (`torch.LongTensor` of shape `(num_audios)`, *optional*) : The length of feature shape of each audio in LLM.

video_second_per_grid (`torch.LongTensor` of shape `(num_videos)`, *optional*) : Number of seconds per grid for each video, used for temporal feature mapping.

**Returns:** `Qwen2_5OmniTalkerCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`

A `Qwen2_5OmniTalkerCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen2_5OmniConfig](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniConfig)) and inputs.

The [Qwen2_5OmniTalkerForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniTalkerForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
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
- **rope_deltas** (`torch.LongTensor` of shape `(batch_size, )`, *optional*) -- The rope index difference between sequence length and multimodal rope.
  The attribute is deprecated and will be removed in v5.20, use `model.base_model.rope_deltas` instead.
- **thinker_reply_part** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Hidden states from the thinker model that are used as input for the talker model. These represent the encoded
  response that the talker model will use to generate speech tokens.

Example:

```python
>>> from io import BytesIO
>>> from urllib.request import urlopen
>>> import librosa
>>> from transformers import AutoProcessor, Qwen2_5OmniTalkerForConditionalGeneration

>>> model = Qwen2_5OmniTalkerForConditionalGeneration.from_pretrained("Qwen/Qwen2-Audio-7B")
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

## Qwen2_5OmniTalkerModel[[transformers.Qwen2_5OmniTalkerModel]]

#### transformers.Qwen2_5OmniTalkerModel[[transformers.Qwen2_5OmniTalkerModel]]

```python
transformers.Qwen2_5OmniTalkerModel(config: Qwen2_5OmniTalkerConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/modeling_qwen2_5_omni.py#L2051)

**Parameters:**

config ([Qwen2_5OmniTalkerConfig](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniTalkerConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Qwen2 5 Omni Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Qwen2_5OmniTalkerModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/modeling_qwen2_5_omni.py#L2078)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Qwen2_5OmniConfig](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniConfig)) and inputs.

The [Qwen2_5OmniTalkerModel](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniTalkerModel) forward method, overrides the `__call__` special method.

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

## Qwen2_5OmniToken2WavConfig[[transformers.Qwen2_5OmniToken2WavConfig]]

#### transformers.Qwen2_5OmniToken2WavConfig[[transformers.Qwen2_5OmniToken2WavConfig]]

```python
transformers.Qwen2_5OmniToken2WavConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, dit_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, bigvgan_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/configuration_qwen2_5_omni.py#L502)

**Parameters:**

dit_config (`DiT_Args`, *optional*) : Configuration class for the Diffusion Transformer (DiT) module responsible for generating mel-spectrograms.

bigvgan_config (`BigVGAN_Args`, *optional*) : Configuration class for the BigVGAN module responsible for converting mel-spectrograms to waveforms.

This is the configuration class to store the configuration of a Qwen2 5 OmniModel. It is used to instantiate a Qwen2 5 Omni
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Qwen/Qwen2.5-Omni-7B](https://huggingface.co/Qwen/Qwen2.5-Omni-7B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Qwen2_5OmniToken2WavModel, DiT_Args, BigVGAN_Args

>>> # Initialize DiT configuration
>>> dit_config = DiT_Args(
...     dim=1024,
...     depth=22,
...     heads=16,
...     ff_mult=2
... )

>>> # Initialize BigVGAN configuration
>>> bigvgan_config = BigVGAN_Args(
...     mel_dim=80,
...     upsample_rates=[5,3,2,2,2,2]
... )

>>> # Initialize main configuration
>>> config = Qwen2_5OmniToken2WavConfig(dit_config, bigvgan_config)

>>> # Initialize model with config
>>> model = Qwen2_5OmniToken2Wav(config)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Qwen2_5OmniToken2WavModel[[transformers.Qwen2_5OmniToken2WavModel]]

#### transformers.Qwen2_5OmniToken2WavModel[[transformers.Qwen2_5OmniToken2WavModel]]

```python
transformers.Qwen2_5OmniToken2WavModel(config: Qwen2_5OmniToken2WavConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/modeling_qwen2_5_omni.py#L3551)

**Parameters:**

config ([Qwen2_5OmniToken2WavConfig](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniToken2WavConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The full Qwen2.5Omni Token2Wav model. Consists a DiT model take speech tokens as input and predict mel spectrogram and a BigVGAN vocoder take mel spectrogram as input and predict waveform.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Qwen2_5OmniToken2WavModel.forward]]

```python
forward(code, conditioning, reference_mel, num_steps = 10, guidance_scale = 0.5, sway_coefficient = -1.0, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/modeling_qwen2_5_omni.py#L3580)

Generates a waveform from input code and conditioning parameters.

## Qwen2_5OmniToken2WavDiTModel[[transformers.Qwen2_5OmniToken2WavDiTModel]]

#### transformers.Qwen2_5OmniToken2WavDiTModel[[transformers.Qwen2_5OmniToken2WavDiTModel]]

```python
transformers.Qwen2_5OmniToken2WavDiTModel(config: Qwen2_5OmniDiTConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/modeling_qwen2_5_omni.py#L3380)

**Parameters:**

config ([Qwen2_5OmniDiTConfig](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniDiTConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The full Qwen2.5Omni Token2WavDiT model. Which take speech tokens as input and predict mel spectrogram.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

## Qwen2_5OmniToken2WavBigVGANModel[[transformers.Qwen2_5OmniToken2WavBigVGANModel]]

#### transformers.Qwen2_5OmniToken2WavBigVGANModel[[transformers.Qwen2_5OmniToken2WavBigVGANModel]]

```python
transformers.Qwen2_5OmniToken2WavBigVGANModel(config: Qwen2_5OmniBigVGANConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/qwen2_5_omni/modeling_qwen2_5_omni.py#L3242)

**Parameters:**

config ([Qwen2_5OmniBigVGANConfig](/docs/transformers/v5.15.1/en/model_doc/qwen2_5_omni#transformers.Qwen2_5OmniBigVGANConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The full Qwen2.5Omni Token2WavBigVGAN model. Which take mel spectrogram as input and predict waveform.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.
