# GlmAsr

## Overview

**GLM-ASR-Nano-2512** is a robust, open-source speech recognition model with **1.5B parameters**. Designed for
real-world complexity, it outperforms OpenAI Whisper V3 on multiple benchmarks while maintaining a compact size.

Key capabilities include:

* **Exceptional Dialect Support**
  Beyond standard Mandarin and English, the model is highly optimized for **Cantonese (粤语)** and other dialects,
  effectively bridging the gap in dialectal speech recognition.

* **Low-Volume Speech Robustness**
  Specifically trained for **"Whisper/Quiet Speech"** scenarios. It captures and accurately transcribes extremely
  low-volume audio that traditional models often miss.

* **SOTA Performance**
  Achieves the **lowest average error rate (4.10)** among comparable open-source models, showing significant advantages
  in Chinese benchmarks (Wenet Meeting, Aishell-1, etc..).

This model was contributed by [Eustache Le Bihan](https://huggingface.co/eustlb) and [Yuxuan Zhang](https://huggingface.co/ZHANGYUXUAN-zR).
you can check the [model card](https://huggingface.co/zai-org/GLM-ASR-Nano-2512) for more details and our
[github repo](https://github.com/zai-org/GLM-ASR).

## Usage

### Basic usage

```py
from transformers import AutoModelForSeq2SeqLM, AutoProcessor

processor = AutoProcessor.from_pretrained("zai-org/GLM-ASR-Nano-2512")
model = AutoModelForSeq2SeqLM.from_pretrained("zai-org/GLM-ASR-Nano-2512", device_map="auto")

inputs = processor.apply_transcription_request("https://huggingface.co/datasets/hf-internal-testing/dummy-audio-samples/resolve/main/bcn_weather.mp3")

inputs = inputs.to(model.device, dtype=model.dtype)
outputs = model.generate(**inputs, do_sample=False, max_new_tokens=500)

decoded_outputs = processor.batch_decode(outputs[:, inputs.input_ids.shape[1] :], skip_special_tokens=True)
assert len(decoded_outputs) == 1  # nodoc
print(decoded_outputs)
```

### Advanced usage

The processor's `apply_transcription_request` is equivalent to using the chat template in the following manner:

```py
from transformers import AutoProcessor, GlmAsrForConditionalGeneration

processor = AutoProcessor.from_pretrained("zai-org/GLM-ASR-Nano-2512")
model = GlmAsrForConditionalGeneration.from_pretrained("zai-org/GLM-ASR-Nano-2512", device_map="auto")

inputs = processor.apply_transcription_request("https://huggingface.co/datasets/hf-internal-testing/dummy-audio-samples/resolve/main/bcn_weather.mp3")

# which is equivalent to
conversation = [
    {
        "role": "user",
        "content": [
            {
                "type": "audio",
                "url": "https://huggingface.co/datasets/hf-internal-testing/dummy-audio-samples/resolve/main/bcn_weather.mp3",
            },
            {"type": "text", "text": "Please transcribe this audio into text"},
        ],
    },
]

inputs = processor.apply_chat_template(
    conversation,
    tokenize=True,
    add_generation_prompt=True,
    return_dict=True,
)

inputs = inputs.to(model.device, dtype=model.dtype)
outputs = model.generate(**inputs, do_sample=False, max_new_tokens=500)

decoded_outputs = processor.batch_decode(outputs[:, inputs.input_ids.shape[1] :], skip_special_tokens=True)
print(decoded_outputs)
```

One can also use audio arrays directly:

```python
from datasets import Audio, load_dataset

from transformers import AutoProcessor, GlmAsrForConditionalGeneration

processor = AutoProcessor.from_pretrained("zai-org/GLM-ASR-Nano-2512")
model = GlmAsrForConditionalGeneration.from_pretrained("zai-org/GLM-ASR-Nano-2512", device_map="auto")

# loading audio directly from dataset
ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))
audio_array = ds[0]["audio"]["array"]

inputs = processor.apply_transcription_request(audio_array)

inputs = inputs.to(model.device, dtype=model.dtype)
outputs = model.generate(**inputs, do_sample=False, max_new_tokens=500)

decoded_outputs = processor.batch_decode(outputs[:, inputs.input_ids.shape[1] :], skip_special_tokens=True)
print(decoded_outputs)
```

### Batched inference

You can process multiple audio files at once:

```python
import torch

from transformers import AutoProcessor, GlmAsrForConditionalGeneration

checkpoint_name = "zai-org/GLM-ASR-Nano-2512"
processor = AutoProcessor.from_pretrained(checkpoint_name)

conversation = [
    [
        {
            "role": "user",
            "content": [
                {
                    "type": "audio",
                    "url": "https://huggingface.co/datasets/eustlb/audio-samples/resolve/main/bcn_weather.mp3",
                },
                {"type": "text", "text": "Please transcribe this audio into text"},
            ],
        },
    ],
    [
        {
            "role": "user",
            "content": [
                {
                    "type": "audio",
                    "url": "https://huggingface.co/datasets/eustlb/audio-samples/resolve/main/obama2.mp3",
                },
                {"type": "text", "text": "Please transcribe this audio into text"},
            ],
        },
    ],
]

model = GlmAsrForConditionalGeneration.from_pretrained(checkpoint_name, device_map="auto")

inputs = processor.apply_chat_template(
    conversation, tokenize=True, add_generation_prompt=True, return_dict=True
).to(model.device, dtype=model.dtype)

inputs_transcription = processor.apply_transcription_request(
    [
        "https://huggingface.co/datasets/eustlb/audio-samples/resolve/main/bcn_weather.mp3",
        "https://huggingface.co/datasets/eustlb/audio-samples/resolve/main/obama2.mp3",
    ],
).to(model.device, dtype=model.dtype)

for key in inputs:
    assert torch.equal(inputs[key], inputs_transcription[key])

outputs = model.generate(**inputs, do_sample=False, max_new_tokens=500)

decoded_outputs = processor.batch_decode(
    outputs[:, inputs.input_ids.shape[1] :], skip_special_tokens=True
)

EXPECTED_OUTPUT = [
    "Yesterday it was thirty five degrees in Barcelona, but today the temperature will go down to minus twenty degrees.",
    "This week, I traveled to Chicago to deliver my final farewell address to the nation, following in the tradition of presidents before me. It was an opportunity to say thank you. Whether we've seen eye to eye or rarely agreed at all, my conversations with you, the American people, in living rooms and schools, at farms and on factory floors, at diners and on distant military outposts, all these conversations are what have kept me honest, kept me inspired, and kept me going. Every day, I learned from you. You made me a better president, and you made me a better man. Over the",
]
assert decoded_outputs == EXPECTED_OUTPUT
```

## GlmAsrEncoderConfig[[transformers.GlmAsrEncoderConfig]]

- **hidden_size** (`int`, *optional*, defaults to `1280`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `5120`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `32`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `20`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_key_value_heads** (`int`, *optional*) --
  This is the number of key_value heads that should be used to implement Grouped Query Attention. If
  `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if
  `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When
  converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed
  by meanpooling all the original heads within that group. For more details, check out [this
  paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to
  `num_attention_heads`.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **max_position_embeddings** (`int`, *optional*, defaults to `1500`) --
  The maximum sequence length that this model might ever be used with.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **rope_parameters** (`dict`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **num_mel_bins** (`int`, *optional*, defaults to `128`) --
  Number of mel features used per input frame. Should correspond to the value used in the
  `AutoFeatureExtractor` class.

This is the configuration class to store the configuration of a GlmAsrModel. It is used to instantiate a Glmasr
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [zai-org/GLM-ASR-Nano-2512](https://huggingface.co/zai-org/GLM-ASR-Nano-2512)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import GlmAsrEncoderConfig, GlmAsrEncoder

>>> # Initializing a GlmAsrEncoderConfig
>>> configuration = GlmAsrEncoderConfig()

>>> # Initializing a GlmAsrEncoder (with random weights)
>>> model = GlmAsrEncoder(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## GlmAsrConfig[[transformers.GlmAsrConfig]]

- **audio_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the audio backbone.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **audio_token_id** (`int`, *optional*, defaults to `59260`) --
  The audio token index used as a placeholder for input audio.
- **projector_hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The activation function used by the multimodal projector.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a GlmAsrModel. It is used to instantiate a Glmasr
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [zai-org/GLM-ASR-Nano-2512](https://huggingface.co/zai-org/GLM-ASR-Nano-2512)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import GlmAsrForConditionalGeneration, GlmAsrConfig

>>> # Initializing a glmasr configuration
>>> configuration = GlmAsrConfig()

>>> # Initializing a GLM-ASR-Nano-2512 model with random weights
>>> model = GlmAsrForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## GlmAsrPreTrainedModel[[transformers.GlmAsrPreTrainedModel]]

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

A mock value for a dotted path (e.g. `torch.float32`): attribute access chains,
calls behave as pass-through decorators, `repr` is the dotted path, and using it
as a base class substitutes a plain-`type` base (PEP 560 `__mro_entries__`), so
real subclasses keep a normal metaclass and `inspect.signature` reads their real
`__init__` instead of a mock's.

## GlmAsrProcessor[[transformers.GlmAsrProcessor]]

'"}, {"name": "default_transcription_prompt", "val": " = 'Please transcribe this audio into text'"}, {"name": "max_audio_len", "val": " = 655"}]}>
- **feature_extractor** (`WhisperFeatureExtractor`) --
  The feature extractor is a required input.
- **tokenizer** (`TokenizersBackend`) --
  The tokenizer is a required input.
- **chat_template** (`str`) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
- **audio_token** (`Optional[str]`, *optional*, defaults to `"<|pad|>`") --
  Special token used to represent audio inputs in the chat template.
- **default_transcription_prompt** (`str`, *optional*, defaults to `"Please transcribe this audio into text"`) --
  Default prompt to use for transcription tasks when applying transcription requests.
- **max_audio_len** (`int`, *optional*, defaults to 655) --
  Maximum length of audio sequences in seconds. Audio longer than this will be truncated.
  655 gives approximately 8192 tokens, corresponding to the maximum sequence length of the text model.
Constructs a GlmAsrProcessor which wraps a feature extractor and a tokenizer into a single processor.

[GlmAsrProcessor](/docs/transformers/v5.14.0/en/model_doc/glmasr#transformers.GlmAsrProcessor) offers all the functionalities of [WhisperFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/whisper#transformers.WhisperFeatureExtractor) and [TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend). See the
[~WhisperFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/whisper#transformers.WhisperFeatureExtractor) and [~TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend) for more information.

- **text** (`Union[str, list[str]]`) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **audio** (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`, *optional*) --
  The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor.
  In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels,
  and T is the sample length of the audio.
- **output_labels** (`bool`, *optional*, default=False) --
  Whether to return labels for training.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.
- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) --
  Additional processing options for each modality (text, images, videos, audio). Model-specific parameters
  are listed above; see the TypedDict class for the complete list of supported arguments.[BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature)A dictionary with tokenized text (`input_ids`, `attention_mask`) and
audio features (`input_features`, `input_features_mask`).

## GlmAsrEncoder[[transformers.GlmAsrEncoder]]

- **input_features** (`` of shape `(batch_size, sequence_length, feature_dim)`) --
  The tensors corresponding to the input audio features. Audio features can be obtained using
  [WhisperFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/whisper#transformers.WhisperFeatureExtractor). See [WhisperFeatureExtractor.__call__()](/docs/transformers/v5.14.0/en/model_doc/whisper#transformers.WhisperFeatureExtractor.__call__) for details ([GlmAsrProcessor](/docs/transformers/v5.14.0/en/model_doc/glmasr#transformers.GlmAsrProcessor) uses
  [WhisperFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/whisper#transformers.WhisperFeatureExtractor) for processing audios).
The [GlmAsrEncoder](/docs/transformers/v5.14.0/en/model_doc/glmasr#transformers.GlmAsrEncoder) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

## GlmAsrModel[[transformers.GlmAsrModel]]

- **config** ([GlmAsrModel](/docs/transformers/v5.14.0/en/model_doc/glmasr#transformers.GlmAsrModel)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The GlmAsr model which consists of a fine-tuned Whisper encoder, a multi-modal projector and a Llama language model.

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
- **input_features** (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) --
  The tensors corresponding to the input audio features. Audio features can be obtained using
  [WhisperFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/whisper#transformers.WhisperFeatureExtractor). See [WhisperFeatureExtractor.__call__()](/docs/transformers/v5.14.0/en/model_doc/whisper#transformers.WhisperFeatureExtractor.__call__) for details ([GlmAsrProcessor](/docs/transformers/v5.14.0/en/model_doc/glmasr#transformers.GlmAsrProcessor) uses
  [WhisperFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/whisper#transformers.WhisperFeatureExtractor) for processing audios).
- **input_features_mask** (`torch.Tensor` of shape `(batch_size, feature_sequence_length)`) --
  Mask to avoid performing attention on padding feature indices.
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
  `past_key_values`).`GlmAsrModelOutputWithPast` or `tuple(torch.FloatTensor)`A `GlmAsrModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([GlmAsrConfig](/docs/transformers/v5.14.0/en/model_doc/glmasr#transformers.GlmAsrConfig)) and inputs.
The [GlmAsrModel](/docs/transformers/v5.14.0/en/model_doc/glmasr#transformers.GlmAsrModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **audio_hidden_states** (`torch.FloatTensor`, *optional*) -- Projected audio hidden states.

## GlmAsrForConditionalGeneration[[transformers.GlmAsrForConditionalGeneration]]

- **config** ([GlmAsrForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/glmasr#transformers.GlmAsrForConditionalGeneration)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The GlmAsr model which consists of a fine-tuned Whisper encoder, a multi-modal projector and a Llama language model.

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
- **input_features** (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) --
  The tensors corresponding to the input audio features. Audio features can be obtained using
  [WhisperFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/whisper#transformers.WhisperFeatureExtractor). See [WhisperFeatureExtractor.__call__()](/docs/transformers/v5.14.0/en/model_doc/whisper#transformers.WhisperFeatureExtractor.__call__) for details ([GlmAsrProcessor](/docs/transformers/v5.14.0/en/model_doc/glmasr#transformers.GlmAsrProcessor) uses
  [WhisperFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/whisper#transformers.WhisperFeatureExtractor) for processing audios).
- **input_features_mask** (`torch.Tensor` of shape `(batch_size, feature_sequence_length)`) --
  Mask to avoid performing attention on padding feature indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
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
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).[CausalLMOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or `tuple(torch.FloatTensor)`A [CausalLMOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([GlmAsrConfig](/docs/transformers/v5.14.0/en/model_doc/glmasr#transformers.GlmAsrConfig)) and inputs.
The [GlmAsrForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/glmasr#transformers.GlmAsrForConditionalGeneration) forward method, overrides the `__call__` special method.

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

Example:

```python
>>> from transformers import GlmAsrForConditionalGeneration, AutoProcessor

>>> model_id = "zai-org/GLM-ASR-Nano-2512"
>>> processor = AutoProcessor.from_pretrained(model_id)
>>> model = GlmAsrForConditionalGeneration.from_pretrained(model_id, dtype="auto", device_map="auto")
>>> inputs = processor.apply_transcription_request("https://huggingface.co/datasets/hf-internal-testing/dummy-audio-samples/resolve/main/bcn_weather.mp3")

>>> inputs = inputs.to(model.device, dtype=model.dtype)

>>> outputs = model.generate(**inputs, do_sample=False, max_new_tokens=500)

>>> decoded_outputs = processor.batch_decode(outputs[:, inputs.input_ids.shape[1] :], skip_special_tokens=True)
>>> print(decoded_outputs)
```
