# Kyutai Speech-To-Text

## Overview

[Kyutai STT](https://kyutai.org/next/stt) is a speech-to-text model architecture based on the [Mimi codec](https://huggingface.co/docs/transformers/en/model_doc/mimi), which encodes audio into discrete tokens in a streaming fashion, and a [Moshi-like](https://huggingface.co/docs/transformers/en/model_doc/moshi) autoregressive decoder. Kyutai's lab has released two model checkpoints:

- [kyutai/stt-1b-en_fr](https://huggingface.co/kyutai/stt-1b-en_fr): a 1B-parameter model capable of transcribing both English and French
- [kyutai/stt-2.6b-en](https://huggingface.co/kyutai/stt-2.6b-en): a 2.6B-parameter model focused solely on English, optimized for maximum transcription accuracy

    

## Usage Tips

### Inference

```python
from datasets import Audio, load_dataset

from transformers import KyutaiSpeechToTextForConditionalGeneration, KyutaiSpeechToTextProcessor

# 1. load the model and the processor
model_id = "kyutai/stt-2.6b-en-trfs"

processor = KyutaiSpeechToTextProcessor.from_pretrained(model_id)
model = KyutaiSpeechToTextForConditionalGeneration.from_pretrained(model_id, device_map="auto")

# 2. load audio samples
ds = load_dataset(
    "hf-internal-testing/librispeech_asr_dummy", "clean", split="validation"
)
ds = ds.cast_column("audio", Audio(sampling_rate=24000))

# 3. prepare the model inputs
inputs = processor(
    ds[0]["audio"]["array"],
)
inputs.to(model.device)

# 4. infer the model
output_tokens = model.generate(**inputs)

# 5. decode the generated tokens
print(processor.batch_decode(output_tokens, skip_special_tokens=True))
```

### Batched Inference

```python
from datasets import Audio, load_dataset

from transformers import KyutaiSpeechToTextForConditionalGeneration, KyutaiSpeechToTextProcessor

# 1. load the model and the processor
model_id = "kyutai/stt-2.6b-en-trfs"

processor = KyutaiSpeechToTextProcessor.from_pretrained(model_id)
model = KyutaiSpeechToTextForConditionalGeneration.from_pretrained(model_id, device_map="auto")

# 2. load audio samples
ds = load_dataset(
    "hf-internal-testing/librispeech_asr_dummy", "clean", split="validation"
)
ds = ds.cast_column("audio", Audio(sampling_rate=24000))

# 3. prepare the model inputs
audio_arrays = [ds[i]["audio"]["array"] for i in range(4)]
inputs = processor(audio_arrays, return_tensors="pt", padding=True).to(model.device)
inputs = inputs.to(model.device)

# 4. infer the model
output_tokens = model.generate(**inputs)

# 5. decode the generated tokens
decoded_outputs = processor.batch_decode(output_tokens, skip_special_tokens=True)
for output in decoded_outputs:
    print(output)
```

This model was contributed by [Eustache Le Bihan](https://huggingface.co/eustlb).
The original code can be found [here](https://github.com/kyutai-labs/moshi).

## KyutaiSpeechToTextConfig[[transformers.KyutaiSpeechToTextConfig]]

#### transformers.KyutaiSpeechToTextConfig[[transformers.KyutaiSpeechToTextConfig]]

```python
transformers.KyutaiSpeechToTextConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, codebook_vocab_size: int = 2049, vocab_size: int = 4001, hidden_size: int = 2048, num_hidden_layers: int = 48, num_attention_heads: int = 32, num_key_value_heads: int | None = None, max_position_embeddings: int = 750, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, hidden_act: str = 'silu', head_dim: int | None = None, initializer_range: float = 0.02, use_cache: bool = True, sliding_window: int = 375, attention_dropout: float | int = 0.0, ffn_dim: int = 11264, rms_norm_eps: float = 1e-08, num_codebooks: int = 32, audio_bos_token_id: int | None = 2048, audio_pad_token_id: int | None = 69569, tie_word_embeddings: bool = False, pad_token_id: int | None = 3, bos_token_id: int | None = 48000, eos_token_id: int | list[int] | None = None, codec_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kyutai_speech_to_text/configuration_kyutai_speech_to_text.py#L29)

**Parameters:**

codebook_vocab_size (`int`, *optional*, defaults to 2049) : Vocabulary size of the codebook. Defines the number of different audio tokens that can be represented by each codebook.

vocab_size (`int`, *optional*, defaults to `4001`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `2048`) : Dimension of the hidden representations.

num_hidden_layers (`int`, *optional*, defaults to `48`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `32`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

max_position_embeddings (`int`, *optional*, defaults to `750`) : The maximum sequence length that this model might ever be used with.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

head_dim (`int`, *optional*) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

sliding_window (`int`, *optional*, defaults to `375`) : Sliding window attention window size. If `None`, no sliding window is applied.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

ffn_dim (`int`, *optional*, defaults to `11264`) : Dimension of the MLP representations.

rms_norm_eps (`float`, *optional*, defaults to `1e-08`) : The epsilon used by the rms normalization layers.

num_codebooks (`int`, *optional*, defaults to `32`) : The number of parallel codebooks used by the model.

audio_bos_token_id (`int`, *optional*, defaults to 2048) : Beginning of stream token id for codebook tokens.

audio_pad_token_id (`int`, *optional*, defaults to 69569) : Padding token id for codebook tokens.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

pad_token_id (`int`, *optional*, defaults to `3`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `48000`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*) : Token id used for end-of-stream in the vocabulary.

codec_config (`PreTrainedConfig`, *optional*) : Configuration for the codec.

This is the configuration class to store the configuration of a KyutaiSpeechToTextModel. It is used to instantiate a Kyutai Speech To Text
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [kyutai/stt-2.6b-en-trfs](https://huggingface.co/kyutai/stt-2.6b-en-trfs)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import KyutaiSpeechToTextConfig, KyutaiSpeechToTextForConditionalGeneration

>>> # Initializing a KyutaiSpeechToTextConfig
>>> configuration = KyutaiSpeechToTextConfig()

>>> # Initializing a model
>>> model = KyutaiSpeechToTextForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## KyutaiSpeechToTextProcessor[[transformers.KyutaiSpeechToTextProcessor]]

#### transformers.KyutaiSpeechToTextProcessor[[transformers.KyutaiSpeechToTextProcessor]]

```python
transformers.KyutaiSpeechToTextProcessor(feature_extractor, tokenizer)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kyutai_speech_to_text/processing_kyutai_speech_to_text.py#L30)

**Parameters:**

feature_extractor (`KyutaiSpeechToTextFeatureExtractor`) : The feature extractor is a required input.

tokenizer (`tokenizer_class`) : The tokenizer is a required input.

Constructs a KyutaiSpeechToTextProcessor which wraps a feature extractor and a tokenizer into a single processor.

[KyutaiSpeechToTextProcessor](/docs/transformers/v5.15.1/en/model_doc/kyutai_speech_to_text#transformers.KyutaiSpeechToTextProcessor) offers all the functionalities of [KyutaiSpeechToTextFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/kyutai_speech_to_text#transformers.KyutaiSpeechToTextFeatureExtractor) and `tokenizer_class`. See the
[~KyutaiSpeechToTextFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/kyutai_speech_to_text#transformers.KyutaiSpeechToTextFeatureExtractor) and `~tokenizer_class` for more information.

#### __call__[[transformers.KyutaiSpeechToTextProcessor.__call__]]

```python
__call__(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, text: str | list[str] | list[list[str]] | None = None, videos: typing.Union[list['PIL.Image.Image'], numpy.ndarray, ForwardRef('torch.Tensor'), list[numpy.ndarray], list['torch.Tensor'], list[list['PIL.Image.Image']], list[list[numpy.ndarray]], list[list['torch.Tensor']], transformers.video_utils.URL, list[transformers.video_utils.URL], list[list[transformers.video_utils.URL]], transformers.video_utils.Path, list[transformers.video_utils.Path], list[list[transformers.video_utils.Path]], NoneType] = None, audio: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor'], NoneType] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/processing_utils.py#L651)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

text (`Union[str, list[str], list[list[str]]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

videos (`Union[list[PIL.Image.Image], numpy.ndarray, torch.Tensor, list[numpy.ndarray], list[torch.Tensor], list[list[PIL.Image.Image]], list[list[numpy.ndarray]], list[list[torch.Tensor]], ~video_utils.URL, list[~video_utils.URL], list[list[~video_utils.URL]], ~video_utils.Path, list[~video_utils.Path], list[list[~video_utils.Path]]]`, *optional*) : Video to preprocess. Expects a single or batch of videos with pixel values ranging from 0 to 255. If passing in videos with pixel values between 0 and 1, set `do_rescale=False`.

audio (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`, *optional*) : The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor. In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels, and T is the sample length of the audio.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) : Additional processing options for each modality (text, images, videos, audio). Model-specific parameters are listed above; see the TypedDict class for the complete list of supported arguments.

## KyutaiSpeechToTextFeatureExtractor[[transformers.KyutaiSpeechToTextFeatureExtractor]]

#### transformers.KyutaiSpeechToTextFeatureExtractor[[transformers.KyutaiSpeechToTextFeatureExtractor]]

```python
transformers.KyutaiSpeechToTextFeatureExtractor(feature_size: int = 1, sampling_rate: int = 24000, padding_value: float = 0.0, chunk_length_s: float | None = None, overlap: float | None = None, audio_delay_seconds: float | None = 0.0, audio_silence_prefix_seconds: float | None = 0.0, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kyutai_speech_to_text/feature_extraction_kyutai_speech_to_text.py#L32)

**Parameters:**

feature_size (`int`, *optional*, defaults to 1) : The feature dimension of the extracted features. Use 1 for mono, 2 for stereo.

sampling_rate (`int`, *optional*, defaults to 24000) : The sampling rate at which the audio waveform should be digitalized expressed in hertz (Hz).

padding_value (`float`, *optional*, defaults to 0.0) : The value that is used to fill the padding values.

chunk_length_s (`float`, *optional*) : If defined the audio is pre-processed into chunks of lengths `chunk_length_s` and then encoded.

overlap (`float`, *optional*) : Defines the overlap between each chunk. It is used to compute the `chunk_stride` using the following formulae : `int((1.0 - self.overlap) * self.chunk_length)`.

audio_delay_seconds (`float`, *optional*, defaults to 0.0) : The delay in seconds to add after the audio (right padding).

audio_silence_prefix_seconds (`float`, *optional*, defaults to 0.0) : The silence prefix in seconds to add before the audio (left padding).

Constructs an KyutaiSpeechToText feature extractor.

This feature extractor inherits from [SequenceFeatureExtractor](/docs/transformers/v5.15.1/en/main_classes/feature_extractor#transformers.SequenceFeatureExtractor) which contains
most of the main methods. Users should refer to this superclass for more information regarding those methods.

## KyutaiSpeechToTextForConditionalGeneration[[transformers.KyutaiSpeechToTextForConditionalGeneration]]

#### transformers.KyutaiSpeechToTextForConditionalGeneration[[transformers.KyutaiSpeechToTextForConditionalGeneration]]

```python
transformers.KyutaiSpeechToTextForConditionalGeneration(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kyutai_speech_to_text/modeling_kyutai_speech_to_text.py#L682)

**Parameters:**

config ([KyutaiSpeechToTextForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/kyutai_speech_to_text#transformers.KyutaiSpeechToTextForConditionalGeneration)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Kyutai Speech To Text Model for token generation conditioned on other modalities (e.g. image-text-to-text generation).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.KyutaiSpeechToTextForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kyutai_speech_to_text/modeling_kyutai_speech_to_text.py#L705)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

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
elements depending on the configuration ([KyutaiSpeechToTextConfig](/docs/transformers/v5.15.1/en/model_doc/kyutai_speech_to_text#transformers.KyutaiSpeechToTextConfig)) and inputs.

The [KyutaiSpeechToTextForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/kyutai_speech_to_text#transformers.KyutaiSpeechToTextForConditionalGeneration) forward method, overrides the `__call__` special method.

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

Example:

```python
>>> import torch
>>> from datasets import load_dataset, Audio
>>> from transformers import KyutaiSpeechToTextProcessor, KyutaiSpeechToTextForConditionalGeneration

>>> torch_device = "cuda" if torch.cuda.is_available() else "cpu"
>>> model_id = "kyutai/stt-2.6b-en-trfs"

>>> processor = KyutaiSpeechToTextProcessor.from_pretrained(model_id)
>>> model = KyutaiSpeechToTextForConditionalGeneration.from_pretrained(model_id, device_map=torch_device)

>>> ds = load_dataset(
...     "hf-internal-testing/librispeech_asr_dummy", "clean", split="validation"
... )

>>> ds = ds.cast_column("audio", Audio(sampling_rate=24000))
>>> inputs = processor(
...     ds[0]["audio"]["array"],
... )
>>> inputs.to(torch_device)

>>> output_tokens = model.generate(**inputs)
>>> print(processor.batch_decode(output_tokens, skip_special_tokens=True))
```

#### generate[[transformers.KyutaiSpeechToTextForConditionalGeneration.generate]]

```python
generate(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kyutai_speech_to_text/modeling_kyutai_speech_to_text.py#L963)

This method forwards all its arguments to GenerationMixin's [generate()](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationMixin.generate). Please refer to the docstring of this method for more information.

## KyutaiSpeechToTextModel[[transformers.KyutaiSpeechToTextModel]]

#### transformers.KyutaiSpeechToTextModel[[transformers.KyutaiSpeechToTextModel]]

```python
transformers.KyutaiSpeechToTextModel(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kyutai_speech_to_text/modeling_kyutai_speech_to_text.py#L572)

**Parameters:**

config ([KyutaiSpeechToTextModel](/docs/transformers/v5.15.1/en/model_doc/kyutai_speech_to_text#transformers.KyutaiSpeechToTextModel)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Kyutai Speech To Text Text Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.KyutaiSpeechToTextModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/kyutai_speech_to_text/modeling_kyutai_speech_to_text.py#L590)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([KyutaiSpeechToTextConfig](/docs/transformers/v5.15.1/en/model_doc/kyutai_speech_to_text#transformers.KyutaiSpeechToTextConfig)) and inputs.

The [KyutaiSpeechToTextModel](/docs/transformers/v5.15.1/en/model_doc/kyutai_speech_to_text#transformers.KyutaiSpeechToTextModel) forward method, overrides the `__call__` special method.

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
