# CohereAsr

## Overview

Cohere ASR, [released](https://cohere.com/blog/transcribe) by Cohere on March 26th, 2026, is a 2B parameter Conformer-based encoder-decoder speech recognition model.

This model was contributed by [Eustache Le Bihan](https://huggingface.co/eustlb).

## Usage

### Short-form transcription

```python
from transformers import AutoProcessor, CohereAsrForConditionalGeneration
from transformers.audio_utils import load_audio

revision = "refs/pr/6"
processor = AutoProcessor.from_pretrained("CohereLabs/cohere-transcribe-03-2026", revision=revision)
model = CohereAsrForConditionalGeneration.from_pretrained("CohereLabs/cohere-transcribe-03-2026", device_map="auto", revision=revision)

audio = load_audio(
    "https://huggingface.co/datasets/hf-internal-testing/dummy-audio-samples/resolve/main/bcn_weather.mp3",
    sampling_rate=16000,
)

inputs = processor(audio, sampling_rate=16000, return_tensors="pt", language="en").to(model.device)
inputs.to(model.device, dtype=model.dtype)

outputs = model.generate(**inputs, max_new_tokens=256)
text = processor.decode(outputs, skip_special_tokens=True)
print(text)
```

### Punctuation control

Pass `punctuation=False` to obtain lower-cased output without punctuation marks.

```python
inputs_pnc = processor(audio, sampling_rate=16000, return_tensors="pt", language="en", punctuation=True).to(model.device)
inputs_nopnc = processor(audio, sampling_rate=16000, return_tensors="pt", language="en", punctuation=False).to(model.device)
```

### Long-form transcription

For audio longer than the feature extractor's `max_audio_clip_s`, the feature extractor automatically splits the waveform into chunks.
The processor reassembles the per-chunk transcriptions using the returned `audio_chunk_index`.

```python
audio_long = load_audio(
    "https://huggingface.co/datasets/hf-internal-testing/dummy-audio-samples/resolve/main/obama_first_45_secs.mp3",
    sampling_rate=16000,
)

inputs = processor(audio=audio_long, return_tensors="pt", language="en", sampling_rate=16000).to(model.device)
audio_chunk_index = inputs.get("audio_chunk_index")
inputs.to(model.device, dtype=model.dtype)

outputs = model.generate(**inputs, max_new_tokens=256)
text = processor.decode(outputs, skip_special_tokens=True, audio_chunk_index=audio_chunk_index, language="en")
print(text)
```

### Batched inference

Multiple audio files can be processed in a single call. When the batch mixes short-form and long-form audio, the
processor handles chunking and reassembly.

```python
audio_short = load_audio(
    "https://huggingface.co/datasets/hf-internal-testing/dummy-audio-samples/resolve/main/bcn_weather.mp3",
    sampling_rate=16000,
)
audio_long = load_audio(
    "https://huggingface.co/datasets/hf-internal-testing/dummy-audio-samples/resolve/main/obama_first_45_secs.mp3",
    sampling_rate=16000,
)

inputs = processor([audio_short, audio_long], sampling_rate=16000, return_tensors="pt", language="en").to(model.device)
audio_chunk_index = inputs.get("audio_chunk_index")
inputs.to(model.device, dtype=model.dtype)

outputs = model.generate(**inputs, max_new_tokens=256)
text = processor.decode(
    outputs, skip_special_tokens=True, audio_chunk_index=audio_chunk_index, language="en"
)
print(text)
```

### Non-English transcription

Specify the language code to transcribe in any of the 14 supported languages.

```python
audio_es = load_audio(
    "https://huggingface.co/datasets/hf-internal-testing/dummy-audio-samples/resolve/main/fleur_es_sample.wav",
    sampling_rate=16000,
)

inputs = processor(audio_es, sampling_rate=16000, return_tensors="pt", language="es", punctuation=True).to(model.device)
inputs.to(model.device, dtype=model.dtype)

outputs = model.generate(**inputs, max_new_tokens=256)
text = processor.decode(outputs, skip_special_tokens=True)
print(text)
```

## CohereAsrConfig[[transformers.CohereAsrConfig]]

#### transformers.CohereAsrConfig[[transformers.CohereAsrConfig]]

```python
transformers.CohereAsrConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, is_encoder_decoder: bool = True, encoder_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, vocab_size: int = 16384, hidden_size: int = 1024, num_hidden_layers: int = 8, num_attention_heads: int = 8, num_key_value_heads: int | None = None, intermediate_size: int = 4096, hidden_act: str = 'relu', max_position_embeddings: int = 1024, pad_token_id: int | None = 2, eos_token_id: int | None = 3, bos_token_id: int | None = 4, use_cache: bool = True, initializer_range: float = 0.02, attention_dropout: float | int = 0.0, attention_bias: bool = True, decoder_start_token_id: int | None = None, tie_word_embeddings: bool = False, head_dim: int | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/cohere_asr/configuration_cohere_asr.py#L25)

**Parameters:**

is_encoder_decoder (`bool`, *optional*, defaults to `True`) : Whether the model is used as an encoder/decoder or not.

encoder_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the encoder backbone.

vocab_size (`int`, *optional*, defaults to `16384`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `1024`) : Dimension of the hidden representations.

num_hidden_layers (`int`, *optional*, defaults to `8`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

intermediate_size (`int`, *optional*, defaults to `4096`) : Dimension of the MLP representations.

hidden_act (`str`, *optional*, defaults to `relu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `1024`) : The maximum sequence length that this model might ever be used with.

pad_token_id (`int`, *optional*, defaults to `2`) : Token id used for padding in the vocabulary.

eos_token_id (`int`, *optional*, defaults to `3`) : Token id used for end-of-stream in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `4`) : Token id used for beginning-of-stream in the vocabulary.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

attention_bias (`bool`, *optional*, defaults to `True`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

decoder_start_token_id (`int`, *optional*) : If an encoder-decoder model starts decoding with a different token than `bos`, the id of that token.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

head_dim (`int`, *optional*) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

This is the configuration class to store the configuration of a CohereAsrModel. It is used to instantiate a Cohere Asr
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [CohereLabs/cohere-transcribe-03-2026](https://huggingface.co/CohereLabs/cohere-transcribe-03-2026)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import CohereAsrForConditionalGeneration, CohereAsrConfig

>>> configuration = CohereAsrConfig()
>>> model = CohereAsrForConditionalGeneration(configuration)
>>> configuration = model.config
```

## CohereAsrFeatureExtractor[[transformers.models.cohere_asr.feature_extraction_cohere_asr._LazyModule.__getattr__..Placeholder]]

#### transformers.models.cohere_asr.feature_extraction_cohere_asr._LazyModule.__getattr__..Placeholder[[transformers.models.cohere_asr.feature_extraction_cohere_asr._LazyModule.__getattr__..Placeholder]]

```python
transformers.models.cohere_asr.feature_extraction_cohere_asr._LazyModule.__getattr__.<locals>.Placeholder(*args, **kwargs)
```

#### __call__[[transformers.models.cohere_asr.feature_extraction_cohere_asr._LazyModule.__getattr__..Placeholder.__call__]]

```python
__call__(*args, **kwargs)
```

Call self as a function.

## CohereAsrProcessor[[transformers.CohereAsrProcessor]]

#### transformers.CohereAsrProcessor[[transformers.CohereAsrProcessor]]

```python
transformers.CohereAsrProcessor(feature_extractor, tokenizer)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/cohere_asr/processing_cohere_asr.py#L51)

**Parameters:**

feature_extractor (`CohereAsrFeatureExtractor`) : The feature extractor is a required input.

tokenizer (`TokenizersBackend`) : The tokenizer is a required input.

Constructs a CohereAsrProcessor which wraps a feature extractor and a tokenizer into a single processor.

[CohereAsrProcessor](/docs/transformers/v5.15.1/en/model_doc/cohere_asr#transformers.CohereAsrProcessor) offers all the functionalities of [CohereAsrFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/cohere_asr#transformers.models.cohere_asr.feature_extraction_cohere_asr._LazyModule.__getattr__..Placeholder) and [TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend). See the
[~CohereAsrFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/cohere_asr#transformers.models.cohere_asr.feature_extraction_cohere_asr._LazyModule.__getattr__..Placeholder) and [~TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend) for more information.

#### __call__[[transformers.CohereAsrProcessor.__call__]]

```python
__call__(audio: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor']], language: str, text: str | list[str] | list[list[str]] | None = None, punctuation: bool = True, sampling_rate: int | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/cohere_asr/processing_cohere_asr.py#L79)

**Parameters:**

audio (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`) : The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor. In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels, and T is the sample length of the audio.

language (`str`) : Language code (e.g. `"en"`, `"es"`, `"fr"`) used to build the decoder prompt. The processor constructs the full decoder prompt and returns `decoder_input_ids` alongside the audio features.

text (`Union[str, list[str], list[list[str]]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

punctuation (`bool`, defaults to `True`) : Whether to enable punctuation in the decoder prompt.

sampling_rate (`int`, *optional*) : The sampling rate of the input audio in Hz. This should match the sampling rate expected by the feature extractor (defaults to 16000 Hz). If provided, it will be validated against the processor's expected sampling rate, and an error will be raised if they don't match. If not provided, a warning will be issued and the default sampling rate will be assumed.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) : Additional processing options for each modality (text, images, videos, audio). Model-specific parameters are listed above; see the TypedDict class for the complete list of supported arguments.

## CohereAsrPreTrainedModel[[transformers.CohereAsrPreTrainedModel]]

#### transformers.CohereAsrPreTrainedModel[[transformers.CohereAsrPreTrainedModel]]

```python
transformers.CohereAsrPreTrainedModel(config: PreTrainedConfig, *inputs, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/cohere_asr/modeling_cohere_asr.py#L297)

**Parameters:**

config ([PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.CohereAsrPreTrainedModel.forward]]

```python
forward(*args, **kwargs)
```

A mock value for a dotted path (e.g. `torch.float32`): attribute access chains,
calls behave as pass-through decorators, `repr` is the dotted path, and using it
as a base class substitutes a plain-`type` base (PEP 560 `__mro_entries__`), so
real subclasses keep a normal metaclass and `inspect.signature` reads their real
`__init__` instead of a mock's.

## CohereAsrModel[[transformers.CohereAsrModel]]

#### transformers.CohereAsrModel[[transformers.CohereAsrModel]]

```python
transformers.CohereAsrModel(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/cohere_asr/modeling_cohere_asr.py#L426)

**Parameters:**

config ([CohereAsrModel](/docs/transformers/v5.15.1/en/model_doc/cohere_asr#transformers.CohereAsrModel)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Cohere Asr Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.CohereAsrModel.forward]]

```python
forward(input_features: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.LongTensor] = None, decoder_input_ids: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: tuple[tuple[torch.FloatTensor]] | None = None, past_key_values: transformers.cache_utils.EncoderDecoderCache | None = None, decoder_inputs_embeds: tuple[torch.FloatTensor] | None = None, decoder_position_ids: tuple[torch.LongTensor] | None = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/cohere_asr/modeling_cohere_asr.py#L454)

**Parameters:**

input_features (`torch.FloatTensor` of shape `(batch_size, audio_length)`) : Float values of the raw speech waveform. Raw speech waveform can be obtained by loading a `.flac` or `.wav` audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_features`, the [AutoFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoFeatureExtractor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`.

attention_mask (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

decoder_input_ids (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Indices of decoder input sequence tokens in the vocabulary.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are decoder input IDs?](../glossary#decoder-input-ids)

decoder_attention_mask (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Mask to avoid performing attention on certain token indices. By default, a causal mask will be used, to make sure the model can only look at previous inputs in order to predict the future.

encoder_outputs (`tuple[tuple[torch.FloatTensor]]`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

past_key_values (`~cache_utils.EncoderDecoderCache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

decoder_inputs_embeds (`tuple[torch.FloatTensor]` of shape `(batch_size, target_sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `decoder_input_ids` you can choose to directly pass an embedded representation. If `past_key_values` is used, optionally only the last `decoder_inputs_embeds` have to be input (see `past_key_values`). This is useful if you want more control over how to convert `decoder_input_ids` indices into associated vectors than the model's internal embedding lookup matrix.  If `decoder_input_ids` and `decoder_inputs_embeds` are both unset, `decoder_inputs_embeds` takes the value of `inputs_embeds`.

decoder_position_ids (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`) : Indices of positions of each input sequence tokens in the position embeddings. Used to calculate the position embeddings up to `config.decoder_config.max_position_embeddings`

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** [Seq2SeqModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqModelOutput) or `tuple(torch.FloatTensor)`

A [Seq2SeqModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([CohereAsrConfig](/docs/transformers/v5.15.1/en/model_doc/cohere_asr#transformers.CohereAsrConfig)) and inputs.

The [CohereAsrModel](/docs/transformers/v5.15.1/en/model_doc/cohere_asr#transformers.CohereAsrModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the decoder of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [EncoderDecoderCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.
- **decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the decoder at the output of each layer plus the optional initial embedding outputs.
- **decoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **cross_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the encoder at the output of each layer plus the optional initial embedding outputs.
- **encoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.

Example:

```python
>>> import torch
>>> from transformers import AutoFeatureExtractor, CohereAsrModel
>>> from datasets import load_dataset

>>> model = CohereAsrModel.from_pretrained("UsefulSensors/cohere_asr-tiny")
>>> feature_extractor = AutoFeatureExtractor.from_pretrained("UsefulSensors/cohere_asr-tiny")
>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> inputs = feature_extractor(ds[0]["audio"]["array"], return_tensors="pt")
>>> input_features = inputs.input_features
>>> decoder_input_ids = torch.tensor([[1, 1]]) * model.config.decoder_start_token_id
>>> last_hidden_state = model(input_features, decoder_input_ids=decoder_input_ids).last_hidden_state
>>> list(last_hidden_state.shape)
[1, 2, 288]
```

## CohereAsrForConditionalGeneration[[transformers.CohereAsrForConditionalGeneration]]

#### transformers.CohereAsrForConditionalGeneration[[transformers.CohereAsrForConditionalGeneration]]

```python
transformers.CohereAsrForConditionalGeneration(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/cohere_asr/modeling_cohere_asr.py#L548)

**Parameters:**

config ([CohereAsrForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/cohere_asr#transformers.CohereAsrForConditionalGeneration)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The CohereAsr Model with a language modeling head. Can be used for automatic speech recognition.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.CohereAsrForConditionalGeneration.forward]]

```python
forward(input_features: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.LongTensor] = None, decoder_input_ids: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: tuple[tuple[torch.FloatTensor]] | None = None, past_key_values: transformers.cache_utils.EncoderDecoderCache | None = None, decoder_inputs_embeds: tuple[torch.FloatTensor] | None = None, decoder_position_ids: tuple[torch.LongTensor] | None = None, use_cache: bool | None = None, labels: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/cohere_asr/modeling_cohere_asr.py#L568)

**Parameters:**

input_features (`torch.FloatTensor` of shape `(batch_size, audio_length)`) : Float values of the raw speech waveform. Raw speech waveform can be obtained by loading a `.flac` or `.wav` audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_features`, the [AutoFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoFeatureExtractor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`.

attention_mask (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

decoder_input_ids (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Indices of decoder input sequence tokens in the vocabulary.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are decoder input IDs?](../glossary#decoder-input-ids)

decoder_attention_mask (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Mask to avoid performing attention on certain token indices. By default, a causal mask will be used, to make sure the model can only look at previous inputs in order to predict the future.

encoder_outputs (`tuple[tuple[torch.FloatTensor]]`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

past_key_values (`~cache_utils.EncoderDecoderCache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

decoder_inputs_embeds (`tuple[torch.FloatTensor]` of shape `(batch_size, target_sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `decoder_input_ids` you can choose to directly pass an embedded representation. If `past_key_values` is used, optionally only the last `decoder_inputs_embeds` have to be input (see `past_key_values`). This is useful if you want more control over how to convert `decoder_input_ids` indices into associated vectors than the model's internal embedding lookup matrix.  If `decoder_input_ids` and `decoder_inputs_embeds` are both unset, `decoder_inputs_embeds` takes the value of `inputs_embeds`.

decoder_position_ids (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`) : Indices of positions of each input sequence tokens in the position embeddings. Used to calculate the position embeddings up to `config.decoder_config.max_position_embeddings`

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

**Returns:** [Seq2SeqLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or `tuple(torch.FloatTensor)`

A [Seq2SeqLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([CohereAsrConfig](/docs/transformers/v5.15.1/en/model_doc/cohere_asr#transformers.CohereAsrConfig)) and inputs.

The [CohereAsrForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/cohere_asr#transformers.CohereAsrForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [EncoderDecoderCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.
- **decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the decoder at the output of each layer plus the initial embedding outputs.
- **decoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **cross_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the encoder at the output of each layer plus the initial embedding outputs.
- **encoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.

Example:

```python
>>> import torch
>>> from transformers import AutoProcessor, CohereAsrForConditionalGeneration
>>> from datasets import load_dataset

>>> processor = AutoProcessor.from_pretrained("UsefulSensors/cohere_asr-tiny")
>>> model = CohereAsrForConditionalGeneration.from_pretrained("UsefulSensors/cohere_asr-tiny")

>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")

>>> inputs = processor(ds[0]["audio"]["array"], return_tensors="pt")
>>> input_features = inputs.input_features

>>> generated_ids = model.generate(input_features, max_new_tokens=100)

>>> transcription = processor.batch_decode(generated_ids, skip_special_tokens=True)[0]
>>> transcription
'Mr. Quilter is the apostle of the middle classes, and we are glad to welcome his gospel.'
```
