# Csm

## Overview

The Conversational Speech Model (CSM) is the first open-source contextual text-to-speech model [released by Sesame](https://www.sesame.com/research/crossing_the_uncanny_valley_of_voice). It is designed to generate natural-sounding speech with or without conversational context. This context typically consists of multi-turn dialogue between speakers, represented as sequences of text and corresponding spoken audio.

**Model Architecture:**
CSM is composed of two LLaMA-style auto-regressive transformer decoders: a backbone decoder that predicts the first codebook token and a depth decoder that generates the remaining tokens. It uses the pretrained codec model [Mimi](./mimi), introduced by Kyutai, to encode speech into discrete codebook tokens and decode them back into audio.

The original csm-1b checkpoint is available under the [Sesame](https://huggingface.co/sesame/csm-1b) organization on Hugging Face.

    

## Usage Tips

### Without Conversational Context

CSM can be used to simply generate speech from a text prompt:

```python
from transformers import AutoProcessor, CsmForConditionalGeneration

model_id = "sesame/csm-1b"

# load the model and the processor
processor = AutoProcessor.from_pretrained(model_id)
model = CsmForConditionalGeneration.from_pretrained(model_id, device_map="auto")

# prepare the inputs
text = "[0]The past is just a story we tell ourselves." # `[0]` for speaker id 0
inputs = processor(text, add_special_tokens=True).to(model.device)

# another equivalent way to prepare the inputs
conversation = [
    {"role": "0", "content": [{"type": "text", "text": "The past is just a story we tell ourselves."}]},
]
inputs = processor.apply_chat_template(
    conversation,
    tokenize=True,
    return_dict=True,
).to(model.device)

# infer the model
audio = model.generate(**inputs, output_audio=True)
processor.save_audio(audio, "example_without_context.wav")
```

### With Conversational Context

CSM can be used to generate speech given a conversation, allowing consistency in the voices and content-aware generation:

```python
from datasets import Audio, load_dataset

from transformers import AutoProcessor, CsmForConditionalGeneration

model_id = "sesame/csm-1b"

# load the model and the processor
processor = AutoProcessor.from_pretrained(model_id)
model = CsmForConditionalGeneration.from_pretrained(model_id, device_map="auto")

# prepare the inputs
ds = load_dataset("hf-internal-testing/dailytalk-dummy", split="train")
# ensure the audio is 24kHz
ds = ds.cast_column("audio", Audio(sampling_rate=24000))
conversation = []

# 1. context
for text, audio, speaker_id in zip(ds[:4]["text"], ds[:4]["audio"], ds[:4]["speaker_id"]):
    conversation.append(
        {
            "role": f"{speaker_id}",
            "content": [{"type": "text", "text": text}, {"type": "audio", "path": audio["array"]}],
        }
    )

# 2. text prompt
conversation.append({"role": f"{ds[4]['speaker_id']}", "content": [{"type": "text", "text": ds[4]["text"]}]})

inputs = processor.apply_chat_template(
    conversation,
    tokenize=True,
    return_dict=True,
).to(model.device)

# infer the model
audio = model.generate(**inputs, output_audio=True)
processor.save_audio(audio, "example_with_context.wav")
```

### Batched Inference

CSM supports batched inference!

```python
from datasets import Audio, load_dataset

from transformers import AutoProcessor, CsmForConditionalGeneration

model_id = "sesame/csm-1b"

# load the model and the processor
processor = AutoProcessor.from_pretrained(model_id)
model = CsmForConditionalGeneration.from_pretrained(model_id, device_map="auto")

# prepare the inputs
ds = load_dataset("hf-internal-testing/dailytalk-dummy", split="train")
# ensure the audio is 24kHz
ds = ds.cast_column("audio", Audio(sampling_rate=24000))
# here a batch with two prompts
conversation = [
    [
        {
            "role": f"{ds[0]['speaker_id']}",
            "content": [
                {"type": "text", "text": ds[0]["text"]},
                {"type": "audio", "path": ds[0]["audio"]["array"]},
            ],
        },
        {
            "role": f"{ds[1]['speaker_id']}",
            "content": [
                {"type": "text", "text": ds[1]["text"]},
            ],
        },
    ],
    [
        {
            "role": f"{ds[0]['speaker_id']}",
            "content": [
                {"type": "text", "text": ds[0]["text"]},
            ],
        }
    ],
]
inputs = processor.apply_chat_template(
    conversation,
    tokenize=True,
    return_dict=True,
).to(model.device)

audio = model.generate(**inputs, output_audio=True)
processor.save_audio(audio, [f"speech_batch_idx_{i}.wav" for i in range(len(audio))])
```

### Making The Model Go Brrr

CSM supports full-graph compilation with CUDA graphs!

```python

import torch
from datasets import load_dataset

from transformers import AutoProcessor, CsmForConditionalGeneration

model_id = "sesame/csm-1b"

# set logs to ensure no recompilation and graph breaks
torch._logging.set_logs(graph_breaks=True, recompiles=True, cudagraphs=True)

# load the model and the processor
processor = AutoProcessor.from_pretrained(model_id)
model = CsmForConditionalGeneration.from_pretrained(model_id, device_map="auto")

# use static cache, enabling automatically torch compile with fullgraph and reduce-overhead
model.generation_config.max_length = 250 # big enough to avoid recompilation
model.generation_config.max_new_tokens = None # would take precedence over max_length
model.generation_config.cache_implementation = "static"
model.depth_decoder.generation_config.cache_implementation = "static"

# generation kwargs
gen_kwargs = {
    "do_sample": False,
    "depth_decoder_do_sample": False,
    "temperature": 1.0,
    "depth_decoder_temperature": 1.0,
}

# Define a timing decorator
class TimerContext:
    def __init__(self, name="Execution"):
        self.name = name
        self.start_event = None
        self.end_event = None

    def __enter__(self):
        # Use CUDA events for more accurate GPU timing
        self.start_event = torch.cuda.Event(enable_timing=True)
        self.end_event = torch.cuda.Event(enable_timing=True)
        self.start_event.record()
        return self

    def __exit__(self, *args):
        self.end_event.record()
        torch.cuda.synchronize()
        elapsed_time = self.start_event.elapsed_time(self.end_event) / 1000.0
        print(f"{self.name} time: {elapsed_time:.4f} seconds")

# prepare the inputs
ds = load_dataset("hf-internal-testing/dailytalk-dummy", split="train")

conversation = [
    {
        "role": f"{ds[0]['speaker_id']}",
        "content": [
            {"type": "text", "text": ds[0]["text"]},
            {"type": "audio", "path": ds[0]["audio"]["array"]},
        ],
    },
    {
        "role": f"{ds[1]['speaker_id']}",
        "content": [
            {"type": "text", "text": ds[1]["text"]},
            {"type": "audio", "path": ds[1]["audio"]["array"]},
        ],
    },
    {
        "role": f"{ds[2]['speaker_id']}",
        "content": [
            {"type": "text", "text": ds[2]["text"]},
        ],
    },
]

padded_inputs_1 = processor.apply_chat_template(
    conversation,
    tokenize=True,
    return_dict=True,
).to(model.device)

print("\n" + "="*50)
print("First generation - compiling and recording CUDA graphs...")
with TimerContext("First generation"):
    _ = model.generate(**padded_inputs_1, **gen_kwargs)
print("="*50)

print("\n" + "="*50)
print("Second generation - fast !!!")
with TimerContext("Second generation"):
    _ = model.generate(**padded_inputs_1, **gen_kwargs)
print("="*50)

# now with different inputs
conversation = [
    {
        "role": f"{ds[0]['speaker_id']}",
        "content": [
            {"type": "text", "text": ds[2]["text"]},
            {"type": "audio", "path": ds[2]["audio"]["array"]},
        ],
    },
    {
        "role": f"{ds[1]['speaker_id']}",
        "content": [
            {"type": "text", "text": ds[3]["text"]},
            {"type": "audio", "path": ds[3]["audio"]["array"]},
        ],
    },
    {
        "role": f"{ds[2]['speaker_id']}",
        "content": [
            {"type": "text", "text": ds[4]["text"]},
        ],
    },
]
padded_inputs_2 = processor.apply_chat_template(
    conversation,
    tokenize=True,
    return_dict=True,
).to(model.device)

print("\n" + "="*50)
print("Generation with other inputs!")
with TimerContext("Generation with different inputs"):
    _ = model.generate(**padded_inputs_2, **gen_kwargs)
print("="*50)
```

### Training

CSM Transformers integration supports training!

```python
from datasets import Audio, load_dataset

from transformers import AutoProcessor, CsmForConditionalGeneration

model_id = "sesame/csm-1b"

# load the model and the processor
processor = AutoProcessor.from_pretrained(model_id)
model = CsmForConditionalGeneration.from_pretrained(model_id, device_map="auto")
model.train()
model.codec_model.eval()

ds = load_dataset("hf-internal-testing/dailytalk-dummy", split="train")
# ensure the audio is 24kHz
ds = ds.cast_column("audio", Audio(sampling_rate=24000))
conversation = []

# context
for text, audio, speaker_id in zip(ds[:4]["text"], ds[:4]["audio"], ds[:4]["speaker_id"]):
    conversation.append(
        {
            "role": f"{speaker_id}",
            "content": [{"type": "text", "text": text}, {"type": "audio", "path": audio["array"]}],
        }
    )

inputs = processor.apply_chat_template(
    conversation,
    tokenize=True,
    return_dict=True,
    output_labels=True,
).to(model.device)

out = model(**inputs)
out.loss.backward()
```

This model was contributed by [Eustache Le Bihan](https://huggingface.co/eustlb).
The original code can be found [here](https://github.com/SesameAILabs/csm).

## CsmConfig[[transformers.CsmConfig]]

#### transformers.CsmConfig[[transformers.CsmConfig]]

```python
transformers.CsmConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, num_codebooks: int | None = 32, vocab_size: int = 2051, text_vocab_size: int = 128256, hidden_size: int = 2048, intermediate_size: int = 8192, num_hidden_layers: int = 16, num_attention_heads: int = 32, num_key_value_heads: int | None = 8, hidden_act: str = 'silu', max_position_embeddings: int = 2048, initializer_range: float = 0.02, rms_norm_eps: float = 1e-05, use_cache: bool = True, pad_token_id: int | None = 128002, codebook_pad_token_id: int | None = 2050, codebook_eos_token_id: int | list[int] | None = 0, bos_token_id: int | None = 128000, eos_token_id: int | list[int] | None = None, audio_token_id: int | None = 128002, audio_eos_token_id: int | list[int] | None = 128003, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, attention_bias: bool = False, attention_dropout: float | int | None = 0.0, mlp_bias: bool = False, head_dim: int | None = None, tie_codebooks_embeddings: bool | None = True, depth_decoder_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, codec_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/csm/configuration_csm.py#L90)

**Parameters:**

num_codebooks (`int`, *optional*, defaults to `32`) : The number of parallel codebooks used by the model.

vocab_size (`int`, *optional*, defaults to `2051`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

text_vocab_size (`int`, *optional*, defaults to `128256`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `2048`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `8192`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `16`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `32`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `8`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `2048`) : The maximum sequence length that this model might ever be used with.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

rms_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the rms normalization layers.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

pad_token_id (`int`, *optional*, defaults to `128002`) : Token id used for padding in the vocabulary.

codebook_pad_token_id (`int`, *optional*, defaults to 2050) : Padding token id for codebook tokens.

codebook_eos_token_id (`int`, *optional*, defaults to 0) : End of stream token id for codebook tokens.

bos_token_id (`int`, *optional*, defaults to `128000`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*) : Token id used for end-of-stream in the vocabulary.

audio_token_id (`int`, *optional*, defaults to 128002) : Audio token id in the text input.

audio_eos_token_id (`int`, *optional*, defaults to 128003) : End of stream token id for audio in the text input.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

attention_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

mlp_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in up_proj, down_proj and gate_proj layers in the MLP layers.

head_dim (`int`, *optional*) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

tie_codebooks_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie the codebook tokens embeddings of the backbone model to the codebook tokens embeddings of the depth decoder.

depth_decoder_config (`CsmDepthDecoderConfig`, *optional*) : Configuration for the depth decoder.

codec_config (`PreTrainedConfig`, *optional*) : Configuration for the codec.

This is the configuration class to store the configuration of a CsmForConditionalGeneration. It is used to instantiate a Csm
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [sesame/csm-1b](https://huggingface.co/sesame/csm-1b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import CsmForConditionalGeneration, CsmConfig

>>> # Initializing a CsmConfig
>>> configuration = CsmConfig()

>>> # Initializing a model
>>> model = CsmForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## CsmDepthDecoderConfig[[transformers.CsmDepthDecoderConfig]]

#### transformers.CsmDepthDecoderConfig[[transformers.CsmDepthDecoderConfig]]

```python
transformers.CsmDepthDecoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, num_codebooks: int | None = 32, backbone_hidden_size: int = 2048, vocab_size: int = 2051, hidden_size: int = 1024, intermediate_size: int = 8192, num_hidden_layers: int = 4, num_attention_heads: int = 8, num_key_value_heads: int | None = 2, hidden_act: str = 'silu', max_position_embeddings: int = 33, initializer_range: float = 0.02, rms_norm_eps: float = 1e-05, use_cache: bool = True, pad_token_id: int | None = None, bos_token_id: int | None = None, eos_token_id: int | list[int] | None = None, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, attention_bias: bool = False, attention_dropout: float | int | None = 0.0, mlp_bias: bool = False, head_dim: int | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/csm/configuration_csm.py#L29)

**Parameters:**

num_codebooks (`int`, *optional*, defaults to `32`) : The number of parallel codebooks used by the model.

backbone_hidden_size (`int`, *optional*, defaults to 2048) : Dimension of the hidden representations of the backbone model used with this depth decoder.

vocab_size (`int`, *optional*, defaults to `2051`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `1024`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `8192`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `4`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `2`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `33`) : The maximum sequence length that this model might ever be used with.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

rms_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the rms normalization layers.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

pad_token_id (`int`, *optional*) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*) : Token id used for end-of-stream in the vocabulary.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

attention_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

mlp_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in up_proj, down_proj and gate_proj layers in the MLP layers.

head_dim (`int`, *optional*) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

This is the configuration class to store the configuration of a CsmForConditionalGeneration. It is used to instantiate a Csm
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [sesame/csm-1b](https://huggingface.co/sesame/csm-1b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import CsmDepthDecoder, CsmDepthDecoderConfig

>>> # Initializing a CsmDepthDecoder
>>> configuration = CsmDepthDecoderConfig()
>>> model = CsmDepthDecoderModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## CsmProcessor[[transformers.CsmProcessor]]

    

#### transformers.CsmProcessor[[transformers.CsmProcessor]]

```python
transformers.CsmProcessor(feature_extractor, tokenizer, chat_template = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/csm/processing_csm.py#L72)

**Parameters:**

feature_extractor (`EncodecFeatureExtractor`) : The feature extractor is a required input.

tokenizer (`tokenizer_class`) : The tokenizer is a required input.

chat_template (`str`) : A Jinja template to convert lists of messages in a chat into a tokenizable string.

Constructs a CsmProcessor which wraps a feature extractor and a tokenizer into a single processor.

[CsmProcessor](/docs/transformers/v5.15.1/en/model_doc/csm#transformers.CsmProcessor) offers all the functionalities of [EncodecFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/encodec#transformers.EncodecFeatureExtractor) and `tokenizer_class`. See the
[~EncodecFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/encodec#transformers.EncodecFeatureExtractor) and `~tokenizer_class` for more information.

#### __call__[[transformers.CsmProcessor.__call__]]

```python
__call__(text: str | list[str] | list[list[str]] | None, audio: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor'], NoneType] = None, output_labels: bool | None = False, depth_decoder_labels_ratio: float | None = 1.0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/csm/processing_csm.py#L166)

**Parameters:**

text (`Union[str, list[str], list[list[str]]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

audio (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`, *optional*) : The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor. In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels, and T is the sample length of the audio.

output_labels (`bool`, *optional*, default=False) : Whether to return labels for training. Indices will be in `[config.audio_token_id, -100, -101]`. - `config.audio_token_id` indicates an audio frame (considering sequence length elements as frames) - `-100` will be ignored in the loss computation - `-101` indicates the audio frame will be used only for the backbone model (using the first codebook token as labels)

depth_decoder_labels_ratio (`float`, *optional*, default=1.0) : The ratio of audio frames to keep for the depth decoder labels.

encoded_length_kwargs (`dict[str, *kwargs*, Any]`, *optional*) : Dictionary of keyword arguments used to compute the encoded audio sequence length. This includes parameters such as `kernel_sizes`, `strides`, `dilations`, and `use_causal_conv` that define the convolutional layers used in audio encoding. The encoded length is used to determine how many audio tokens to generate for each audio input in the text sequence.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) : Additional processing options for each modality (text, images, videos, audio). Model-specific parameters are listed above; see the TypedDict class for the complete list of supported arguments.

**Returns:** [BatchFeature](/docs/transformers/v5.15.1/en/main_classes/image_processor#transformers.BatchFeature)

A [BatchFeature](/docs/transformers/v5.15.1/en/main_classes/image_processor#transformers.BatchFeature) with the following fields:

- **input_ids** -- List of token ids to be fed to a model. Returned when `text` is not `None`.
- **input_values** -- List of audio values to be fed to a model. Returned when `audio` is not `None`.
- **attention_mask** -- List of indices specifying which tokens should be attended to by the model (when
  `return_attention_mask=True` or if *"attention_mask"* is in `self.model_input_names` and if `text` is not
  `None`).
- **labels** -- List of labels for the audio frames. Returned when `output_labels=True`.

## CsmForConditionalGeneration[[transformers.CsmForConditionalGeneration]]

#### transformers.CsmForConditionalGeneration[[transformers.CsmForConditionalGeneration]]

```python
transformers.CsmForConditionalGeneration(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/csm/modeling_csm.py#L752)

**Parameters:**

config ([CsmForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/csm#transformers.CsmForConditionalGeneration)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Csm model consists of two llama-like auto-regressive transformer models: a backbone model that predicts the first codebook token and a depth decoder that predicts the other codebook tokens.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.CsmForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, input_values: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, input_values_cutoffs: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/csm/modeling_csm.py#L923)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length, num_codebooks) or (batch_size, sequence_length)`) : 1. (batch_size, sequence_length): corresponds to the input sequence prepared with the processor from the text prompt. Such input requires `input_values` to be provided so that audio can be encoded in codebook tokens and then merged with the text tokens.  2. (batch_size, sequence_length, num_codebooks): codebook tokens generated during the autoregressive decoding. Such input is not meant to be used by end users.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

input_values (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Float values of input raw speech waveform. Values can be obtained by loading a `.flac` or `.wav` audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_values`, the [AutoProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoProcessor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`. See [CsmProcessor.__call__()](/docs/transformers/v5.15.1/en/model_doc/csm#transformers.CsmProcessor.__call__) for details.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

input_values_cutoffs (`torch.Tensor` of shape `(batch_size, max_num_audio)`, *optional*) : Specify the end positions of audio segments within each batch entry, relative to the concatenated audio input. If a batch entry has fewer segments than the maximum, it is padded with -1. For example, in a batch of 2 sequences where the first contains 2 audio segments of length l1, and the second contains 1 audio segment of length l2, the input_values_cutoffs would be: [[l1, 2 * l1], [l2, -1]].

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should be in `[config.audio_token_id, -100, -101]`. Requires targeted `input_values` to be provided as audio tokens will be inferred from it using the `codec_model`. - `config.audio_token_id` indicates an audio frames (considering sequence length elements as frames) - `-100` will be ignored in the loss computation - `-101` indicates the audio frame will be used only for the backbone model (using the first codebook token as labels)  Such labels can be prepared using `output_labels=True` when calling [CsmProcessor](/docs/transformers/v5.15.1/en/model_doc/csm#transformers.CsmProcessor).

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

logits_to_keep (`int` or `torch.Tensor`, *optional*) : Kept for compatibility. Does not support another value than: 1. `0`, which is equivalent to keeping all logits, used in the training regime 2. `1`, which is equivalent to keeping only the last logit, used in the generation regime

**Returns:** `CsmOutputWithPast` or `tuple(torch.FloatTensor)`

A `CsmOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([CsmConfig](/docs/transformers/v5.15.1/en/model_doc/csm#transformers.CsmConfig)) and inputs.

The [CsmForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/csm#transformers.CsmForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **depth_decoder_loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction) of the depth decoder model.
- **depth_decoder_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the depth decoder (scores for each vocabulary token before SoftMax).
- **depth_decoder_past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
- **depth_decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **depth_decoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.
- **backbone_loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction) of the backbone model.

Example:

```python
>>> import torch
>>> from transformers import CsmForConditionalGeneration, AutoProcessor
>>> from datasets import load_dataset, Audio

>>> model_id = "sesame/csm-1b"
>>> torch_device = "cuda" if torch.cuda.is_available() else "cpu"

>>> processor = AutoProcessor.from_pretrained(model_id)

>>> ds = load_dataset("hf-internal-testing/dailytalk-dummy", split="train")
>>> # ensure the audio is 24kHz
>>> ds = ds.cast_column("audio", Audio(sampling_rate=24000))

>>> conversation = []
>>> # prepare a conversation with text and corresponding audio
>>> for text, audio, speaker_id in zip(ds[:4]["text"], ds[:4]["audio"], ds[:4]["speaker_id"]):
...     conversation.append(
...         {
...             "role": f"{speaker_id}",
...             "content": [{"type": "text", "text": text}, {"type": "audio", "path": audio["array"]}],
...         }
...     )

>>> inputs = processor.apply_chat_template(
...     conversation,
...     tokenize=True,
...     return_dict=True,
...     output_labels=True,
... ).to(torch_device)

>>> model = CsmForConditionalGeneration.from_pretrained(model_id, device_map=torch_device)
>>> output = model(**inputs)
>>> output.loss.backward()
```

#### generate[[transformers.CsmForConditionalGeneration.generate]]

```python
generate(input_ids: typing.Optional[torch.Tensor] = None, input_values: typing.Optional[torch.Tensor] = None, input_values_cutoffs: typing.Optional[torch.Tensor] = None, generation_config: transformers.generation.configuration_utils.GenerationConfig | None = None, logits_processor: transformers.generation.logits_process.LogitsProcessorList | None = None, stopping_criteria: transformers.generation.stopping_criteria.StoppingCriteriaList | None = None, synced_gpus: bool | None = None, streamer: typing.Optional[ForwardRef('BaseStreamer')] = None, output_audio: bool | None = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/csm/generation_csm.py#L334)

**Parameters:**

inputs_ids (`torch.Tensor` of shape (batch_size, seq_length), *optional*) : The sequence used as a prompt for the backbone model.

input_values (`torch.Tensor` of shape (batch_size, channels, max_concatenated_audio_length), *optional*) : The batched audio input values, where each batch entry contains the concatenation of all audio segments for that entry. These values will be encoded into codebook tokens using the codec model and merged with the text input ids provided in `input_ids`.

input_values_cutoffs (`torch.Tensor` of shape (batch_size, max_num_audio), *optional*) : Specify the end positions of audio segments within each batch entry, relative to the concatenated audio input. If a batch entry has fewer segments than the maximum, it is padded with -1. For example, in a batch of 2 sequences where the first contains 2 audio segments of length l1, and the second contains 1 audio segment of length l2, the input_values_cutoffs would be: [[l1, 2 * l1], [l2, -1]].

generation_config ([GenerationConfig](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationConfig), *optional*) : The generation configuration to be used as base parametrization for the generation call. `**kwargs` passed to generate matching the attributes of `generation_config` will override them. If `generation_config` is not provided, the default will be used, which has the following loading priority: 1) from the `generation_config.json` model file, if it exists; 2) from the model configuration. Please note that unspecified parameters will inherit [GenerationConfig](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationConfig)'s default values, whose documentation should be checked to parameterize generation.

logits_processor (`LogitsProcessorList`, *optional*) : Custom logits processors that complement the default logits processors built from arguments and generation config. If a logit processor is passed that is already created with the arguments or a generation config an error is thrown. This feature is intended for advanced users.

stopping_criteria (`StoppingCriteriaList`, *optional*) : Custom stopping criteria that complements the default stopping criteria built from arguments and a generation config. If a stopping criteria is passed that is already created with the arguments or a generation config an error is thrown. If your stopping criteria depends on the `scores` input, make sure you pass `return_dict_in_generate=True, output_scores=True` to `generate`. This feature is intended for advanced users.

synced_gpus (`bool`, *optional*) : Whether to continue running the while loop until max_length. Unless overridden, this flag will be set to `True` if using `FullyShardedDataParallel` or DeepSpeed ZeRO Stage 3 with multiple GPUs to avoid deadlocking if one GPU finishes generating before other GPUs. Otherwise, defaults to `False`.

streamer (`BaseStreamer`, *optional*) : Streamer object that will be used to stream the generated sequences. Generated tokens are passed through `streamer.put(token_ids)` and the streamer is responsible for any further processing.

output_audio (`bool`, *optional*) : Whether to return the generated audio.

kwargs (`dict[str, Any]`, *optional*) : Ad hoc parametrization of `generation_config` and/or additional model-specific kwargs that will be forwarded to the `forward` function of the model. Depth decoder specific kwargs should be prefixed with *depth_decoder_*.

**Returns:** `CsmGenerateOutput` or `torch.LongTensor` or `list[torch.FloatTensor]`

A `CsmGenerateOutput`
(if `return_dict_in_generate=True` or when `config.return_dict_in_generate=True`) or a `torch.LongTensor` when `output_audio=False`
or a `list[torch.FloatTensor]` otherwise.

This method overrides [generate()](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationMixin.generate) to match the specifics of the Csm model.
Indeed, Csm model requires a custom generation sampling step:
1. Infer the backbone model to sample the first codebook token
2. Call generate on the depth decoder with the first codebook token as `input_ids` to sample the next codebook tokens
3. Use these generated codebook tokens as `input_ids` to sample the next first codebook token using the backbone model
4. Repeat until stopping criteria is met

Most generation-controlling parameters are set in `generation_config` which, if not passed, will be set to the
model's default generation configuration. You can override any `generation_config` by passing the corresponding
parameters to generate(), e.g. `.generate(inputs, do_sample=True)`.

Example:

```python
>>> from transformers import CsmProcessor, CsmForConditionalGeneration
>>> from datasets import load_dataset, Audio

>>> model_id = "sesame/csm-1b"
>>> torch_device = "cuda" if torch.cuda.is_available() else "cpu"

>>> processor = AutoProcessor.from_pretrained(model_id)

>>> ds = load_dataset("hf-internal-testing/dailytalk-dummy", split="train")
>>> # ensure the audio is 24kHz
>>> ds = ds.cast_column("audio", Audio(sampling_rate=24000))

>>> conversation = []
>>> # prepare a conversation with text and corresponding audio
>>> for text, audio, speaker_id in zip(ds[:4]["text"], ds[:4]["audio"], ds[:4]["speaker_id"]):
...     conversation.append(
...         {
...             "role": f"{speaker_id}",
...             "content": [{"type": "text", "text": text}, {"type": "audio", "path": audio["array"]}],
...         }
...     )

>>> # text prompt
>>> conversation.append({"role": f"{ds[4]['speaker_id']}", "content": [{"type": "text", "text": ds[4]["text"]}]})

>>> inputs = processor.apply_chat_template(
...     conversation,
...     tokenize=True,
...     return_dict=True,
... ).to(torch_device)

>>> model = CsmForConditionalGeneration.from_pretrained(model_id, device_map=torch_device)
>>> audio = model.generate(**inputs, output_audio=True)
>>> processor.save_audio(audio, "output.wav")
```

## CsmDepthDecoderForCausalLM[[transformers.CsmDepthDecoderForCausalLM]]

#### transformers.CsmDepthDecoderForCausalLM[[transformers.CsmDepthDecoderForCausalLM]]

```python
transformers.CsmDepthDecoderForCausalLM(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/csm/modeling_csm.py#L541)

**Parameters:**

config ([CsmDepthDecoderForCausalLM](/docs/transformers/v5.15.1/en/model_doc/csm#transformers.CsmDepthDecoderForCausalLM)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The CsmDepthDecoder Model transformer, with a `CsmCodebooksHead` on top,
which can be seen a position-specific language modeling head, allowing to use a different linear layer for each codebook
(e.g. position 0 is the first codebook and uses the first codebook head, etc.)

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.CsmDepthDecoderForCausalLM.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, backbone_last_hidden_state: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/csm/modeling_csm.py#L556)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

backbone_last_hidden_state (`torch.FloatTensor` of shape `(batch_size, backbone_hidden_size)`, *optional*) : The last hidden state of the backbone model. Such input is required when the first codebook token (the one generated by the backbone model) is provided in the `input_ids` argument.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** [CausalLMOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or `tuple(torch.FloatTensor)`

A [CausalLMOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([CsmConfig](/docs/transformers/v5.15.1/en/model_doc/csm#transformers.CsmConfig)) and inputs.

The [CsmDepthDecoderForCausalLM](/docs/transformers/v5.15.1/en/model_doc/csm#transformers.CsmDepthDecoderForCausalLM) forward method, overrides the `__call__` special method.

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

## CsmDepthDecoderModel[[transformers.CsmDepthDecoderModel]]

#### transformers.CsmDepthDecoderModel[[transformers.CsmDepthDecoderModel]]

```python
transformers.CsmDepthDecoderModel(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/csm/modeling_csm.py#L411)

**Parameters:**

config ([CsmDepthDecoderModel](/docs/transformers/v5.15.1/en/model_doc/csm#transformers.CsmDepthDecoderModel)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Csm Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.CsmDepthDecoderModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, backbone_last_hidden_state: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/csm/modeling_csm.py#L430)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

backbone_last_hidden_state (`torch.FloatTensor` of shape `(batch_size, backbone_hidden_size)`, *optional*) : The last hidden state of the backbone model. Such input is required when the first codebook token (the one generated by the backbone model) is provided in the `input_ids` argument.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([CsmConfig](/docs/transformers/v5.15.1/en/model_doc/csm#transformers.CsmConfig)) and inputs.

The [CsmDepthDecoderModel](/docs/transformers/v5.15.1/en/model_doc/csm#transformers.CsmDepthDecoderModel) forward method, overrides the `__call__` special method.

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

## CsmBackboneModel[[transformers.CsmBackboneModel]]

#### transformers.CsmBackboneModel[[transformers.CsmBackboneModel]]

```python
transformers.CsmBackboneModel(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/csm/modeling_csm.py#L663)

**Parameters:**

config ([CsmBackboneModel](/docs/transformers/v5.15.1/en/model_doc/csm#transformers.CsmBackboneModel)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Csm Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.CsmBackboneModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/csm/modeling_csm.py#L679)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length, num_codebooks) or (batch_size, sequence_length)`) : 1. (batch_size, sequence_length): corresponds to the input sequence prepared with the processor from the text prompt. Such input requires `input_values` to be provided so that audio can be encoded in codebook tokens and then merged with the text tokens.  2. (batch_size, sequence_length, num_codebooks): codebook tokens generated during the autoregressive decoding. Such input is not meant to be used by end users.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([CsmConfig](/docs/transformers/v5.15.1/en/model_doc/csm#transformers.CsmConfig)) and inputs.

The [CsmBackboneModel](/docs/transformers/v5.15.1/en/model_doc/csm#transformers.CsmBackboneModel) forward method, overrides the `__call__` special method.

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
