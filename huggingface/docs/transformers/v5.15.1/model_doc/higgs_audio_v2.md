# Higgs Audio V2

    
        
        
    

## Overview

Higgs Audio V2 is a powerful audio foundation model developed by [Boson AI](https://www.boson.ai/). 
The model was pretrained on over 10 million hours of audio data and a diverse set of text data. 
Despite having no post-training or fine-tuning, Higgs Audio v2 excels in expressive audio generation, thanks to its deep language and acoustic understanding.

**Model Architecture:**

    

Higgs Audio v2 adopts the "generation variant" depicted in the architecture figure above. Its strong performance is driven by three key technical innovations:

- Developed an automated annotation pipeline that leverages multiple ASR models, sound event classification models, and our in-house audio understanding model. Using this pipeline, we cleaned and annotated 10 million hours audio data, which we refer to as AudioVerse. The in-house understanding model is finetuned on top of Higgs Audio v1 Understanding, which adopts the "understanding variant" shown in the architecture figure.
- Trained a unified audio tokenizer from scratch that captures both semantic and acoustic features.
- Proposed DualFFN architecture, which enhances the LLM’s ability to model acoustics tokens with minimal computational overhead.

## Usage

All of the snippets below mirror the integration tests in `test_higgs_audio.py`, ensuring the doc stays in sync with the officially supported workflows.

### Single-speaker smart voice

```python
from transformers import AutoProcessor, HiggsAudioV2ForConditionalGeneration

model_id = "eustlb/higgs-audio-v2-generation-3B-base"
processor = AutoProcessor.from_pretrained(model_id, device_map="auto")
model = HiggsAudioV2ForConditionalGeneration.from_pretrained(model_id, device_map="auto")

conversation = [
    {
        "role": "system",
        "content": [
            {
                "type": "text",
                "text": "Generate audio following instruction."
            }
        ],
    },
    {
        "role": "scene",
        "content": [
            {
                "type": "text",
                "text": "Audio is recorded from a quiet room."
            }
        ]
    },
    {
        "role": "user",
        "content": [
            {
                "type": "text",
                "text": "The sun rises in the east and sets in the west. This simple fact has been observed by humans for thousands of years."
            }
        ]
    }
]

inputs = processor.apply_chat_template(
    conversation,
    add_generation_prompt=True,
    tokenize=True,
    return_dict=True,
    sampling_rate=24000,
    return_tensors="pt"
).to(model.device)

outputs = model.generate(**inputs, max_new_tokens=1000, do_sample=False)
decoded = processor.batch_decode(outputs)
processor.save_audio(decoded, "output_single_speaker_smart_voice.wav")
```

### Multi-speaker smart voice

```python
from transformers import AutoProcessor, HiggsAudioV2ForConditionalGeneration

model_id = "eustlb/higgs-audio-v2-generation-3B-base"
processor = AutoProcessor.from_pretrained(model_id, device_map="auto")
model = HiggsAudioV2ForConditionalGeneration.from_pretrained(model_id, device_map="auto")

system_message = """You are an AI assistant designed to convert text into speech.
If the user's message includes a [SPEAKER*] tag, do not read out the tag and generate speech for the following text, using the specified voice.
If no speaker tag is present, select a suitable voice on your own."""

user_message = """[SPEAKER0] I can't believe you did that without even asking me first!
[SPEAKER1] Oh, come on! It wasn't a big deal, and I knew you would overreact like this.
[SPEAKER0] Overreact? You made a decision that affects both of us without even considering my opinion!
[SPEAKER1] Because I didn't have time to sit around waiting for you to make up your mind! Someone had to act."""

conversation = [
    {
        "role": "system",
        "content": [
            {
                "type": "text",
                "text": system_message
            }
        ]
    },
    {
        "role": "scene",
        "content": [
            {
                "type": "text",
                "text": "Audio is recorded from a quiet room."
            },
            {
                "type": "text",
                "text": "SPEAKER0: feminine"
            },
            {
                "type": "text",
                "text": "SPEAKER1: masculine"
            },
        ]
    },
    {
        "role": "user",
        "content": [
            {
                "type": "text",
                "text": user_message
            }
        ]
    }
]

inputs = processor.apply_chat_template(
    conversation,
    add_generation_prompt=True,
    tokenize=True,
    return_dict=True,
    sampling_rate=24000,
    return_tensors="pt"
).to(model.device)

outputs = model.generate(**inputs, max_new_tokens=50, do_sample=False)
decoded = processor.batch_decode(outputs)
processor.save_audio(decoded, "output_multi_speaker_smart_voice.wav")
```

### Zero-shot voice cloning

```python
from transformers import AutoProcessor, HiggsAudioV2ForConditionalGeneration

model_id = "eustlb/higgs-audio-v2-generation-3B-base"
processor = AutoProcessor.from_pretrained(model_id, device_map="auto")
model = HiggsAudioV2ForConditionalGeneration.from_pretrained(model_id, device_map="auto")

conversation = [
    {
        "role": "system",
        "content": [
            {
                "type": "text",
                "text": "Generate audio following instruction."
            }
        ]
    },
    {
        "role": "scene",
        "content": [
            {
                "type": "text",
                "text": "Audio is recorded from a quiet room."
            }
        ]
    },
    {
        "role": "user",
        "content": [
            {
                "type": "text",
                "text": "It was the night before my birthday. Hooray! It's almost here! It may not be a holiday, but it's the best day of the year."
            }
        ]
    },
    {
        "role": "assistant",
        "content": [
            {
                "type": "audio",
                "url": "https://huggingface.co/datasets/eustlb/dummy-audio-samples-higgs/resolve/main/belinda.wav"
            }
        ]
    },
    {
        "role": "user",
        "content": [
            {
                "type": "text",
                "text": "The sun rises in the east and sets in the west. This simple fact has been observed by humans for thousands of years."
            }
        ]
    }
]

inputs = processor.apply_chat_template(
    conversation,
    add_generation_prompt=True,
    tokenize=True,
    return_dict=True,
    sampling_rate=24000,
    return_tensors="pt"
).to(model.device)

outputs = model.generate(**inputs, max_new_tokens=1000, do_sample=False)
decoded = processor.batch_decode(outputs)
processor.save_audio(decoded, "output_zero_shot_voice_cloning.wav")
```

### Multi-speaker voice cloning

```python
from transformers import AutoProcessor, HiggsAudioV2ForConditionalGeneration

model_id = "eustlb/higgs-audio-v2-generation-3B-base"
processor = AutoProcessor.from_pretrained(model_id, device_map="auto")
model = HiggsAudioV2ForConditionalGeneration.from_pretrained(model_id, device_map="auto")

user_message = """[SPEAKER0] I can't believe you did that without even asking me first!
[SPEAKER1] Oh, come on! It wasn't a big deal, and I knew you would overreact like this.
[SPEAKER0] Overreact? You made a decision that affects both of us without even considering my opinion!
[SPEAKER1] Because I didn't have time to sit around waiting for you to make up your mind! Someone had to act."""

conversation = [
    {
        "role": "system",
        "content": [
            {
                "type": "text",
                "text": "Generate audio following instruction."
            }
        ]
    },
    {
        "role": "scene",
        "content": [
            {
                "type": "text",
                "text": "Audio is recorded from a quiet room."
            },
            {
                "type": "text",
                "text": "SPEAKER0:"
            },
            {
                "type": "audio",
                "url": "https://qianwen-res.oss-cn-beijing.aliyuncs.com/Qwen2-Audio/audio/guess_age_gender.wav"
            },
            {
                "type": "text",
                "text": "SPEAKER1:"
            },
            {
                "type": "audio",
                "url": "https://qianwen-res.oss-cn-beijing.aliyuncs.com/Qwen2-Audio/audio/1272-128104-0000.flac"
            },
        ]
    },
    {
        "role": "user",
        "content": [
            {
                "type": "text",
                "text": user_message
            }
        ]
    },
]

inputs = processor.apply_chat_template(
    conversation,
    add_generation_prompt=True,
    tokenize=True,
    return_dict=True,
    sampling_rate=24000,
    return_tensors="pt"
).to(model.device)

outputs = model.generate(**inputs, max_new_tokens=1000, do_sample=False)
decoded = processor.batch_decode(outputs)
processor.save_audio(decoded, "output_multi_speaker_voice_cloning.wav")
```

### Batched inference

```python
from transformers import AutoProcessor, HiggsAudioV2ForConditionalGeneration

model_id = "eustlb/higgs-audio-v2-generation-3B-base"
processor = AutoProcessor.from_pretrained(model_id, device_map="auto")
model = HiggsAudioV2ForConditionalGeneration.from_pretrained(model_id, device_map="auto")

conversation1 = [
    {
        "role": "system",
        "content": [
            {
                "type": "text",
                "text": "Generate audio following instruction."
            }
        ]
    },
    {
        "role": "scene",
        "content": [
            {
                "type": "text",
                "text": "Audio is recorded from a quiet room."
            }
        ]
    },
    {
        "role": "user",
        "content": [
            {
                "type": "text",
                "text": "It was the night before my birthday. Hooray! It's almost here! It may not be a holiday, but it's the best day of the year."
            }
        ]
    },
    {
        "role": "assistant",
        "content": [
            {
                "type": "audio",
                "url": "https://huggingface.co/datasets/eustlb/dummy-audio-samples-higgs/resolve/main/belinda.wav"
            }
        ]
    },
    {
        "role": "user",
        "content": [
            {
                "type": "text",
                "text": "The sun rises in the east and sets in the west. This simple fact has been observed by humans for thousands of years."
            }
        ]
    }
]

conversation2 = [
    {
        "role": "system",
        "content": [
            {
                "type": "text",
                "text": "Generate audio following instruction."
            }
        ]
    },
    {
        "role": "scene",
        "content": [
            {
                "type": "text",
                "text": "Audio is recorded from a quiet room."
            }
        ]
    },
    {
        "role": "user",
        "content": [
            {
                "type": "text",
                "text": " It's super important to assess fairly the fact that our former model is over. And this is not a question of adjustment. This is not the same world, 2024, 2025. And on top of that, we are making the same mistakes, on top of the key elements I mentioned. We are over-regulating and under-investing. So just if, in the two to three years to come, if we follow our classical agenda, we will be out of the market. I have no doubts."
            }
        ]
    },
    {
        "role": "assistant",
        "content": [
            {
                "type": "audio",
                "url": "https://huggingface.co/datasets/eustlb/dummy-audio-samples-higgs/resolve/main/macron.wav"
            }
        ]
    },
    {
        "role": "user",
        "content": [
            {
                "type": "text",
                "text": "Hey, here is a clone from the given voice."
            }
        ]
    }
]

inputs = processor.apply_chat_template(
    [conversation1, conversation2],
    add_generation_prompt=True,
    tokenize=True,
    return_dict=True,
    sampling_rate=24000,
    return_tensors="pt"
).to(model.device)

outputs = model.generate(**inputs, max_new_tokens=1000, do_sample=False)
decoded = processor.batch_decode(outputs)
processor.save_audio(decoded, ["output_batched_1.wav", "output_batched_2.wav"])
```

### Training

> [!TIP]
> By default, the model does not load the text language modeling head to save memory (~1.5GiB reduction), as it's not required for generation.
> However, when training the model, you need the text head to compute loss on text tokens. To enable it, set `use_text_head=True` when instantiating the model (see example below).

```python
from transformers import AutoProcessor, HiggsAudioV2ForConditionalGeneration

model_id = "eustlb/higgs-audio-v2-generation-3B-base"
processor = AutoProcessor.from_pretrained(model_id, device_map="auto")
model = HiggsAudioV2ForConditionalGeneration.from_pretrained(model_id, device_map="auto", use_text_head=True)

conversation1 = [
    {
        "role": "system",
        "content": [
            {
                "type": "text",
                "text": "Generate audio following instruction."
            }
        ]
    },
    {
        "role": "scene",
        "content": [
            {
                "type": "text",
                "text": "Audio is recorded from a quiet room."
            }
        ]
    },
    {
        "role": "user",
        "content": [
            {
                "type": "text",
                "text": "It was the night before my birthday. Hooray! It's almost here! It may not be a holiday, but it's the best day of the year."
            }
        ]
    },
    {
        "role": "assistant",
        "content": [
            {
                "type": "audio",
                "url": "https://huggingface.co/datasets/eustlb/dummy-audio-samples-higgs/resolve/main/belinda.wav"
            }
        ]
    }
]

conversation2 = [
    {
        "role": "system",
        "content": [
            {
                "type": "text",
                "text": "Generate audio following instruction."
            }
        ]
    },
    {
        "role": "scene",
        "content": [
            {
                "type": "text",
                "text": "Audio is recorded from a quiet room."
            }
        ]
    },
    {
        "role": "user",
        "content": [
            {
                "type": "text",
                "text": " I would imagine so. A wand with a dragon heartstring core is capable of dazzling magic, and the bond between you and your wand should only grow stronger. Do not be surprised at your new wand's ability to perceive your intentions, particularly in a moment of need"
            }
        ]
    },
    {
        "role": "assistant",
        "content": [
            {
                "type": "audio",
                "url": "https://huggingface.co/datasets/eustlb/dummy-audio-samples-higgs/resolve/main/broom_salesman.wav"
            }
        ]
    }
]

inputs = processor.apply_chat_template(
    [conversation1, conversation2],
    add_generation_prompt=True,
    tokenize=True,
    return_dict=True,
    sampling_rate=24000,
    return_tensors="pt",
    output_labels=True,
).to(model.device)

outputs = model(**inputs)
outputs.loss.backward()
```

This model was contributed by [Shuai Zheng](https://huggingface.co/szhengac) and [Eustache Le Bihan](https://huggingface.co/eustlb). The original code can be found [here](https://github.com/boson-ai/higgs-audio).

## HiggsAudioV2Config[[transformers.HiggsAudioV2Config]]

#### transformers.HiggsAudioV2Config[[transformers.HiggsAudioV2Config]]

```python
transformers.HiggsAudioV2Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 128256, hidden_size: int = 3072, intermediate_size: int = 8192, num_hidden_layers: int = 28, num_attention_heads: int = 24, num_key_value_heads: int = 8, hidden_act: str = 'silu', max_position_embeddings: int = 2048, initializer_range: float = 0.02, rms_norm_eps: float = 1e-05, use_cache: bool = True, pad_token_id: int | None = 128001, bos_token_id: int | None = 1, eos_token_id: int | list[int] | None = 128009, pretraining_tp: int | None = 1, tie_word_embeddings: bool = False, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, attention_bias: bool = False, attention_dropout: int | float | None = 0.0, mlp_bias: bool = False, head_dim: int | None = 128, num_codebooks: int = 8, codebook_size: int = 1024, audio_token_id: int = 128016, audio_bos_token_id: int = 128013, audio_delay_token_id: int = 128014, audio_stream_bos_id: int = 1024, audio_stream_eos_id: int = 1025)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/higgs_audio_v2/configuration_higgs_audio_v2.py#L32)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `128256`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `3072`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `8192`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `28`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `24`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `8`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `2048`) : The maximum sequence length that this model might ever be used with.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

rms_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the rms normalization layers.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

pad_token_id (`int`, *optional*, defaults to `128001`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `1`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `128009`) : Token id used for end-of-stream in the vocabulary.

pretraining_tp (`int`, *optional*, defaults to `1`) : Experimental feature. Tensor parallelism rank used during pretraining. Please refer to [this document](https://huggingface.co/docs/transformers/main/perf_train_gpu_many#tensor-parallelism) to understand more about it. This value is necessary to ensure exact reproducibility of the pretraining results. Please refer to [this issue](https://github.com/pytorch/pytorch/issues/76232).

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

attention_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

attention_dropout (`Union[int, float]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

mlp_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in up_proj, down_proj and gate_proj layers in the MLP layers.

head_dim (`int`, *optional*, defaults to `128`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

num_codebooks (`int`, *optional*, defaults to `8`) : The number of parallel codebooks used by the model.

codebook_size (`int`, *optional*, defaults to `1024`) : The number of parallel codebooks used by the model.

audio_token_id (`int`, *optional*, defaults to `128016`) : The audio token index used as a placeholder for input audio.

audio_bos_token_id (`int`, *optional*, defaults to 128013) : The token ID for the beginning-of-sequence token for audio output.

audio_delay_token_id (`int`, *optional*, defaults to 128014) : The token ID used for audio delay pattern in multi-codebook generation.

audio_stream_bos_id (`int`, *optional*, defaults to 1024) : The ID for the beginning-of-stream token in audio sequences.

audio_stream_eos_id (`int`, *optional*, defaults to 1025) : The ID for the end-of-stream token in audio sequences.

This is the configuration class to store the configuration of a HiggsAudioV2ForConditionalGeneration. It is used to instantiate a Higgs Audio V2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [bosonai/higgs-audio-v2-generation-3B-base](https://huggingface.co/bosonai/higgs-audio-v2-generation-3B-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import HiggsAudioV2Model, HiggsAudioV2Config

>>> # Initializing a HiggsAudioV2 style configuration
>>> configuration = HiggsAudioV2Config()

>>> # Initializing a model from the configuration
>>> model = HiggsAudioV2Model(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## HiggsAudioV2Processor[[transformers.HiggsAudioV2Processor]]

#### transformers.HiggsAudioV2Processor[[transformers.HiggsAudioV2Processor]]

```python
transformers.HiggsAudioV2Processor(feature_extractor, tokenizer, audio_tokenizer, chat_template = None, audio_token = '<|AUDIO_OUT|>', audio_bos_token = '<|audio_out_bos|>', audio_eos_token = '<|audio_eos|>', audio_delay_token = '<|reserved_special_token_6|>', audio_stream_bos_id = 1024, audio_stream_eos_id = 1025)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/higgs_audio_v2/processing_higgs_audio_v2.py#L53)

**Parameters:**

feature_extractor (`DacFeatureExtractor`) : An instance of [DacFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/dac#transformers.DacFeatureExtractor). The feature extractor is a required input.

tokenizer (`AutoTokenizer`) : An instance of [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). The tokenizer is a required input.

audio_tokenizer (`HiggsAudioV2TokenizerModel`) : An instance of [HiggsAudioV2TokenizerModel](/docs/transformers/v5.15.1/en/model_doc/higgs_audio_v2_tokenizer#transformers.HiggsAudioV2TokenizerModel). The audio tokenizer is a required input.

chat_template (`str`, *optional*) : A template string for chat formatting when combining text and audio interactions.

audio_token (`str`, *optional*, defaults to `"<|AUDIO_OUT|>"`) : The token used to represent audio output in the text sequence.

audio_bos_token (`str`, *optional*, defaults to `"<|audio_out_bos|>"`) : The beginning-of-sequence token for audio output.

audio_eos_token (`str`, *optional*, defaults to `"<|audio_eos|>"`) : The end-of-sequence token for audio output.

audio_delay_token (`str`, *optional*, defaults to `"<|reserved_special_token_6|>"`) : The token used for audio delay pattern in multi-codebook generation.

audio_stream_bos_id (`int`, *optional*, defaults to 1024) : The ID for the beginning-of-stream token in audio sequences.

audio_stream_eos_id (`int`, *optional*, defaults to 1025) : The ID for the end-of-stream token in audio sequences.

Constructs a Higgs Audio processor which wraps a [DacFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/dac#transformers.DacFeatureExtractor), a [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer),
and a [HiggsAudioV2TokenizerModel](/docs/transformers/v5.15.1/en/model_doc/higgs_audio_v2_tokenizer#transformers.HiggsAudioV2TokenizerModel) into a single processor. It inherits, the audio feature extraction, tokenizer,
and audio encode/decode functionalities.
See [__call__()](/docs/transformers/v5.15.1/en/model_doc/higgs_audio_v2#transformers.HiggsAudioV2Processor.__call__) and [decode()](/docs/transformers/v5.15.1/en/model_doc/higgs_audio_v2#transformers.HiggsAudioV2Processor.decode) for more information.

#### __call__[[transformers.HiggsAudioV2Processor.__call__]]

```python
__call__(text: str | list[str] | list[list[str]] | None = None, audio: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor'], NoneType] = None, output_labels: bool | None = False, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/higgs_audio_v2/processing_higgs_audio_v2.py#L129)

#### decode[[transformers.HiggsAudioV2Processor.decode]]

```python
decode(audio_input_ids)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/higgs_audio_v2/processing_higgs_audio_v2.py#L287)

## HiggsAudioV2Model[[transformers.HiggsAudioV2Model]]

#### transformers.HiggsAudioV2Model[[transformers.HiggsAudioV2Model]]

```python
transformers.HiggsAudioV2Model(config: HiggsAudioV2Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/higgs_audio_v2/modeling_higgs_audio_v2.py#L395)

**Parameters:**

config ([HiggsAudioV2Config](/docs/transformers/v5.15.1/en/model_doc/higgs_audio_v2#transformers.HiggsAudioV2Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Higgs Audio V2 Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.HiggsAudioV2Model.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, audio_input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.LongTensor] = None, audio_input_ids_mask: typing.Optional[torch.BoolTensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/higgs_audio_v2/modeling_higgs_audio_v2.py#L413)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

audio_input_ids (`torch.LongTensor` of shape `(batch_size, num_audio_frames, num_codebooks)`, *optional*) : Indices of audio codebook tokens.  Indices can be obtained using [HiggsAudioV2TokenizerModel.encode()](/docs/transformers/v5.15.1/en/model_doc/higgs_audio_v2_tokenizer#transformers.HiggsAudioV2TokenizerModel.encode).

attention_mask (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

audio_input_ids_mask (`torch.BoolTensor` of shape `(batch_size, num_audio_frames)`, *optional*) : Indicates which audio frames in `audio_input_ids` are valid.

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** `~models.modeling_outputs.BaseModelOutputWithPast`

Usual decoder outputs with the placeholder positions already substituted by their corresponding
audio embeddings.

The [HiggsAudioV2Model](/docs/transformers/v5.15.1/en/model_doc/higgs_audio_v2#transformers.HiggsAudioV2Model) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Example:

```python
>>> from transformers import AutoProcessor, HiggsAudioV2Model
>>> import torch
>>> device = "cuda" if torch.cuda.is_available() else "cpu"
>>> processor = AutoProcessor.from_pretrained("eustlb/higgs-audio-v2-generation-3B-base", device_map=device)
>>> model = HiggsAudioV2Model.from_pretrained("eustlb/higgs-audio-v2-generation-3B-base", device_map=device)
>>> conversation = [
...     {
...         "role": "system",
...         "content": [
...             {
...                 "type": "text",
...                 "text": "Generate audio following instruction."
...             }
...         ]
...     },
...     {
...         "role": "scene",
...         "content": [
...             {
...                 "type": "text",
...                 "text": "Audio is recorded from a quiet room."
...             }
...         ]
...     },
...     {
...         "role": "user",
...         "content": [
...             {
...                 "type": "text",
...                 "text": "It was the night before my birthday. Hooray! It's almost here! It may not be a holiday, but it's the best day of the year."
...             }
...         ]
...     },
...     {
...         "role": "assistant",
...         "content": [
...             {
...                 "type": "audio",
...                 "url": "https://huggingface.co/datasets/eustlb/dummy-audio-samples-higgs/resolve/main/belinda.wav"
...             }
...         ]
...     },
...     {
...         "role": "user",
...         "content": [
...             {
...                 "type": "text",
...                 "text": "The sun rises in the east and sets in the west. This simple fact has been observed by humans for thousands of years."
...             }
...         ]
...     }
... ]
>>> inputs = processor.apply_chat_template(conversation, return_dict=True, tokenize=True, sampling_rate=24000, return_tensors="pt")
>>> inputs = inputs.to(model.device)
>>> outputs = model(**inputs)
```

## HiggsAudioV2ForConditionalGeneration[[transformers.HiggsAudioV2ForConditionalGeneration]]

#### transformers.HiggsAudioV2ForConditionalGeneration[[transformers.HiggsAudioV2ForConditionalGeneration]]

```python
transformers.HiggsAudioV2ForConditionalGeneration(config: HiggsAudioV2Config, use_text_head: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/higgs_audio_v2/modeling_higgs_audio_v2.py#L594)

**Parameters:**

config ([HiggsAudioV2Config](/docs/transformers/v5.15.1/en/model_doc/higgs_audio_v2#transformers.HiggsAudioV2Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

use_text_head (`bool`, *optional*, defaults to False) : Whether to use a text language model head. Such head is not required for generation, but can be used to compute the text loss when training.

The Higgs Audio model, a llama-like auto-regressive transformer model with dual-FFN.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.HiggsAudioV2ForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.BoolTensor] = None, audio_input_ids: typing.Optional[torch.LongTensor] = None, audio_input_ids_mask: typing.Optional[torch.LongTensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, audio_labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/higgs_audio_v2/modeling_higgs_audio_v2.py#L643)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.BoolTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

audio_input_ids (`torch.LongTensor` of shape `(batch_size, num_audio_frames, num_codebooks)`, *optional*) : Indices of audio codebook tokens.  Indices can be obtained using [HiggsAudioV2TokenizerModel.encode()](/docs/transformers/v5.15.1/en/model_doc/higgs_audio_v2_tokenizer#transformers.HiggsAudioV2TokenizerModel.encode).

audio_input_ids_mask (`torch.BoolTensor` of shape `(batch_size, num_audio_frames)`, *optional*) : Indicates which audio frames in `audio_input_ids` are valid.

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

audio_labels (`torch.LongTensor` of shape `(batch_size, num_audio_frames, num_codebooks)`, *optional*) : Labels for the audio codebook tokens for computing the masked language modeling loss. Indices should either be in `[0, ..., config.codebook_size]. Token with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.codebook_size]`. Can be obtained using `output_labels=True` when calling [HiggsAudioV2Processor](/docs/transformers/v5.15.1/en/model_doc/higgs_audio_v2#transformers.HiggsAudioV2Processor).

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** `~models.modeling_outputs.CausalLMOutputWithPast`

A `~models.modeling_outputs.CausalLMOutputWithPast` containing the logits, loss (if labels are provided),
and other outputs from the model.

The [HiggsAudioV2ForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/higgs_audio_v2#transformers.HiggsAudioV2ForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Example:

```python
>>> from transformers import AutoProcessor, HiggsAudioV2ForConditionalGeneration
>>> model_id = "eustlb/higgs-audio-v2-generation-3B-base"
>>> processor = AutoProcessor.from_pretrained(model_id, device_map="auto")
>>> model = HiggsAudioV2ForConditionalGeneration.from_pretrained(model_id, device_map="auto")
>>> conversation = [
...     {
...         "role": "system",
...         "content": [
...             {
...                 "type": "text",
...                 "text": "Generate audio following instruction."
...             }
...         ]
...     },
...     {
...         "role": "scene",
...         "content": [
...             {
...                 "type": "text",
...                 "text": "Audio is recorded from a quiet room."
...             }
...         ]
...     },
...     {
...         "role": "user",
...         "content": [
...             {
...                 "type": "text",
...                 "text": "It was the night before my birthday. Hooray! It's almost here! It may not be a holiday, but it's the best day of the year."
...             }
...         ]
...     },
...     {
...         "role": "assistant",
...         "content": [
...             {
...                 "type": "audio",
...                 "url": "https://huggingface.co/datasets/eustlb/dummy-audio-samples-higgs/resolve/main/belinda.wav"
...             }
...         ]
...     },
...     {
...         "role": "user",
...         "content": [
...             {
...                 "type": "text",
...                 "text": "The sun rises in the east and sets in the west. This simple fact has been observed by humans for thousands of years."
...             }
...         ]
...     }
... ]
>>> inputs = processor.apply_chat_template(conversation, return_dict=True, tokenize=True, sampling_rate=24000, return_tensors="pt")
>>> inputs = inputs.to(model.device)
>>> outputs = model(**inputs)
```

#### generate[[transformers.HiggsAudioV2ForConditionalGeneration.generate]]

```python
generate(inputs: typing.Optional[torch.Tensor] = None, generation_config: transformers.generation.configuration_utils.GenerationConfig | None = None, logits_processor: transformers.generation.logits_process.LogitsProcessorList | None = None, stopping_criteria: transformers.generation.stopping_criteria.StoppingCriteriaList | None = None, prefix_allowed_tokens_fn: collections.abc.Callable[[int, torch.Tensor], list[int]] | None = None, synced_gpus: bool | None = None, assistant_model: typing.Optional[ForwardRef('PreTrainedModel')] = None, streamer: typing.Optional[ForwardRef('BaseStreamer')] = None, negative_prompt_ids: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, custom_generate: str | collections.abc.Callable | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/utils.py#L2260)

**Parameters:**

inputs (`torch.Tensor` of varying shape depending on the modality, *optional*) : The sequence used as a prompt for the generation or as model inputs to the encoder. If `None` the method initializes it with `bos_token_id` and a batch size of 1. For decoder-only models `inputs` should be in the format of `input_ids`. For encoder-decoder models *inputs* can represent any of `input_ids`, `input_values`, `input_features`, or `pixel_values`.

generation_config ([GenerationConfig](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationConfig), *optional*) : The generation configuration to be used as base parametrization for the generation call. `**kwargs` passed to generate matching the attributes of `generation_config` will override them. If `generation_config` is not provided, the default will be used, which has the following loading priority: 1) from the `generation_config.json` model file, if it exists; 2) from the model configuration. Please note that unspecified parameters will inherit [GenerationConfig](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationConfig)'s default values, whose documentation should be checked to parameterize generation.

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

**Returns:** [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) or `torch.LongTensor`

A [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) (if `return_dict_in_generate=True`
or when `config.return_dict_in_generate=True`) or a `torch.LongTensor`.

If the model is *not* an encoder-decoder model (`model.config.is_encoder_decoder=False`), the possible
[ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) types are:

- [GenerateDecoderOnlyOutput](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.generation.GenerateDecoderOnlyOutput),
- [GenerateBeamDecoderOnlyOutput](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.generation.GenerateBeamDecoderOnlyOutput)

If the model is an encoder-decoder model (`model.config.is_encoder_decoder=True`), the possible
[ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) types are:

- [GenerateEncoderDecoderOutput](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.generation.GenerateEncoderDecoderOutput),
- [GenerateBeamEncoderDecoderOutput](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.generation.GenerateBeamEncoderDecoderOutput)

Generates sequences of token ids for models with a language modeling head.

Most generation-controlling parameters are set in `generation_config` which, if not passed, will be set to the
model's default generation configuration. You can override any `generation_config` by passing the corresponding
parameters to generate(), e.g. `.generate(inputs, num_beams=4, do_sample=True)`.

For an overview of generation strategies and code examples, check out the [following
guide](../generation_strategies).
