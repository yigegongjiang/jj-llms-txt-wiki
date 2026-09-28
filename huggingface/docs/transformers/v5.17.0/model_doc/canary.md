# Canary

## Overview

Canary-1B-v2 was proposed in [Canary-1B-v2 & Parakeet-TDT-0.6B-v3: Efficient and High-Performance Models for Multilingual ASR and AST](https://huggingface.co/papers/2509.14128) by Monica Sekoyan, Nithin Rao Koluguri, Nune Tadevosyan, Piotr Zelasko, Travis Bartley, Nikolay Karpov, Jagadeesh Balam, and Boris Ginsburg.

The abstract from the paper is the following:

*This report introduces Canary-1B-v2, a fast, robust multilingual model for Automatic Speech Recognition (ASR) and Speech-to-Text Translation (AST). Built with a FastConformer encoder and Transformer decoder, it supports 25 European languages. The model was trained on 1.7M hours of total data samples, including Granary and NeMo ASR Set 3.0, with non-speech audio added to reduce hallucinations for ASR and AST. We describe its two-stage pre-training and fine-tuning process with dynamic data balancing, as well as experiments with an nGPT encoder. Results show nGPT scales well with massive data, while FastConformer excels after fine-tuning. For timestamps, Canary-1B-v2 uses the NeMo Forced Aligner (NFA) with an auxiliary CTC model, providing reliable segment-level timestamps for ASR and AST. Evaluations show Canary-1B-v2 outperforms Whisper-large-v3 on English ASR while being 10× faster, and delivers competitive multilingual ASR and AST performance against larger models like Seamless-M4T-v2-large and LLM-based systems. We also release Parakeet-TDT-0.6B-v3, a successor to v2, offering multilingual ASR across the same 25 languages with just 600M parameters.*

Canary reuses the [Fast Conformer](https://huggingface.co/papers/2305.05084) encoder from [Parakeet](./parakeet) (loaded through [ParakeetEncoder](/docs/transformers/v5.17.0/en/model_doc/parakeet#transformers.ParakeetEncoder) / [ParakeetEncoderConfig](/docs/transformers/v5.17.0/en/model_doc/parakeet#transformers.ParakeetEncoderConfig)) and pairs it with a Transformer decoder that uses fixed sinusoidal positional embeddings, cross-attention to the encoder outputs and tied input/output embeddings. The task is selected through a decoder prompt prefix built by [CanaryProcessor](/docs/transformers/v5.17.0/en/model_doc/canary#transformers.CanaryProcessor) of the form `<|startofcontext|> <|startoftranscript|> <|emo:undefined|> <source_lang> <target_lang> <pnc|nopnc> <|noitn|> <|notimestamp|> <|nodiarize|>`, where `source_lang == target_lang` selects transcription and otherwise selects translation.

The original implementation can be found in [NVIDIA NeMo](https://github.com/NVIDIA/NeMo). A model checkpoint is available at [nvidia/canary-1b-v2](https://huggingface.co/nvidia/canary-1b-v2).

This model was contributed by [Harshal Janjani](https://huggingface.co/harshaljanjani).

> [!NOTE]
> Segment-level timestamps for Canary-1B-v2 are produced by the external NeMo Forced Aligner (NFA) with an auxiliary CTC model, not by the decoder, so they are not part of the `generate` output.

## Usage

### Transcription

The simplest way to transcribe audio is with `apply_transcription_request`, which builds the multitask decoder prompt for you (it is a convenience wrapper for `apply_chat_template`).

```python
from datasets import load_dataset, Audio
from transformers import AutoProcessor, AutoModelForSpeechSeq2Seq

processor = AutoProcessor.from_pretrained("nvidia/canary-1b-v2")
model = AutoModelForSpeechSeq2Seq.from_pretrained("nvidia/canary-1b-v2", device_map="auto")

ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))

inputs = processor.apply_transcription_request(audio=ds[0]["audio"]["array"], source_language="en").to(model.device)
generated_ids = model.generate(**inputs, max_new_tokens=128)
print(processor.decode(generated_ids, skip_special_tokens=True)[0])
```

### Translation

Set `target_language` to a different language than `source_language` for speech-to-text translation.

```python
...

inputs = processor.apply_transcription_request(
    audio=ds[0]["audio"]["array"], source_language="en", target_language="de"
).to(model.device)
generated_ids = model.generate(**inputs, max_new_tokens=128)
print(processor.decode(generated_ids, skip_special_tokens=True)[0])
```

### Batch inference

Pass a list of audios and, optionally, a list of `source_language` / `target_language`.

```python
...

audios = [ds[0]["audio"]["array"], ds[1]["audio"]["array"]]
# single entries get broadcasted to list
inputs = processor.apply_transcription_request(
    audio=audios, source_language="en", target_language=["en", "de"]
).to(model.device)
generated_ids = model.generate(**inputs, max_new_tokens=128)
for text in processor.decode(generated_ids, skip_special_tokens=True):
    print(text)
```

### Torch compile

For autoregressive transcription, `torch.compile` accelerates the per-token forward passes inside `generate` by providing a `CompileConfig` object.

```python
...

from transformers import CompileConfig

inputs = processor.apply_transcription_request(audio=ds[0]["audio"]["array"], source_language="en").to(model.device)
compile_config = CompileConfig()

# Warmup
for _ in range(3):
    _ = model.generate(**inputs, max_new_tokens=128, cache_implementation="static", compile_config=compile_config)

# Apply model
generated_ids = model.generate(**inputs, max_new_tokens=128, cache_implementation="static", compile_config=compile_config)
print(processor.decode(generated_ids, skip_special_tokens=True)[0])
```

### Training

Canary can be trained with the loss outputted by the model. Put the target transcript in the assistant turn and pass `output_labels=True`. Padding positions are masked automatically.

```python
...

model.train()
transcription = "mister Quilter is the apostle of the middle classes, and we are glad to welcome his gospel."

conversation = [
    [
        {
            "role": "user",
            "content": [
                {"type": "audio", "audio": ds[0]["audio"]["array"]},
                {"type": "text", "source_language": "en", "target_language": "en", "punctuation": True},
            ],
        },
        {"role": "assistant", "content": transcription},
    ]
]

inputs = processor.apply_chat_template(
    conversation,
    tokenize=True,
    return_dict=True,
    processor_kwargs={"output_labels": True},
).to(model.device)

outputs = model(**inputs)
outputs.loss.backward()
```

## CanaryConfig[[transformers.CanaryConfig]]

#### transformers.CanaryConfig[[transformers.CanaryConfig]]

```python
transformers.CanaryConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, is_encoder_decoder: bool = True, encoder_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, decoder_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, use_cache: bool = True, tie_word_embeddings: bool = True, pad_token_id: int | None = 2, bos_token_id: int | None = 4, eos_token_id: int | None = 3, decoder_start_token_id: int | None = 7, initializer_range: float = 0.02, vocab_size: int = 16384)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/canary/configuration_canary.py#L85)

**Parameters:**

is_encoder_decoder (`bool`, *optional*, defaults to `True`) : Whether the model is used as an encoder/decoder or not.

encoder_config (`Union[dict, ParakeetEncoderConfig]`, *optional*) : The config object or dictionary of the FastConformer encoder ([ParakeetEncoderConfig](/docs/transformers/v5.17.0/en/model_doc/parakeet#transformers.ParakeetEncoderConfig)).

decoder_config (`Union[dict, CanaryDecoderConfig]`, *optional*) : The config object or dictionary of the Transformer decoder ([CanaryDecoderConfig](/docs/transformers/v5.17.0/en/model_doc/canary#transformers.CanaryDecoderConfig)).

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

pad_token_id (`int`, *optional*, defaults to `2`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `4`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`int`, *optional*, defaults to `3`) : Token id used for end-of-stream in the vocabulary.

decoder_start_token_id (`int`, *optional*, defaults to 7) : The token id that starts decoding (`<|startofcontext|>`, the first token of the multitask prompt).

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

vocab_size (`int`, *optional*, defaults to `16384`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

This is the configuration class to store the configuration of a CanaryModel. It is used to instantiate a Canary
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [nvidia/canary-1b-v2](https://huggingface.co/nvidia/canary-1b-v2)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import CanaryForConditionalGeneration, CanaryConfig

>>> # Initializing a Canary configuration
>>> configuration = CanaryConfig()

>>> # Initializing a model from the configuration
>>> model = CanaryForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## CanaryDecoderConfig[[transformers.CanaryDecoderConfig]]

#### transformers.CanaryDecoderConfig[[transformers.CanaryDecoderConfig]]

```python
transformers.CanaryDecoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, is_encoder_decoder: bool = True, vocab_size: int = 16384, hidden_size: int = 1024, intermediate_size: int = 4096, num_hidden_layers: int = 8, num_attention_heads: int = 8, num_key_value_heads: int = 8, hidden_act: str = 'relu', max_position_embeddings: int = 1024, initializer_range: float = 0.02, use_cache: bool = True, pad_token_id: int | None = 2, bos_token_id: int | None = 4, eos_token_id: int | None = 3, attention_bias: bool = True, attention_dropout: int | float | None = 0.0, head_dim: int = 128)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/canary/configuration_canary.py#L30)

**Parameters:**

is_encoder_decoder (`bool`, *optional*, defaults to `True`) : Whether the model is used as an encoder/decoder or not.

vocab_size (`int`, *optional*, defaults to `16384`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `1024`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `4096`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `8`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `8`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

hidden_act (`str`, *optional*, defaults to `relu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `1024`) : The maximum sequence length that this model might ever be used with.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

pad_token_id (`int`, *optional*, defaults to `2`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `4`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`int`, *optional*, defaults to `3`) : Token id used for end-of-stream in the vocabulary.

attention_bias (`bool`, *optional*, defaults to `True`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

attention_dropout (`Union[int, float]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

head_dim (`int`, *optional*, defaults to `128`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

This is the configuration class to store the configuration of a CanaryModel. It is used to instantiate a Canary
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [nvidia/canary-1b-v2](https://huggingface.co/nvidia/canary-1b-v2)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import CanaryDecoderModel, CanaryDecoderConfig

>>> # Initializing a CanaryDecoder canary_decoder-7b style configuration
>>> configuration = CanaryDecoderConfig()

>>> # Initializing a model from the canary_decoder-7b style configuration
>>> model = CanaryDecoderModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## CanaryProcessor[[transformers.CanaryProcessor]]

#### transformers.CanaryProcessor[[transformers.CanaryProcessor]]

```python
transformers.CanaryProcessor(feature_extractor = None, tokenizer = None, chat_template = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/canary/processing_canary.py#L74)

**Parameters:**

feature_extractor (`ParakeetFeatureExtractor`) : The feature extractor is a required input.

tokenizer (`TokenizersBackend`) : The tokenizer is a required input.

chat_template (`str`) : A Jinja template to convert lists of messages in a chat into a tokenizable string.

Constructs a CanaryProcessor which wraps a feature extractor and a tokenizer into a single processor.

[CanaryProcessor](/docs/transformers/v5.17.0/en/model_doc/canary#transformers.CanaryProcessor) offers all the functionalities of [ParakeetFeatureExtractor](/docs/transformers/v5.17.0/en/model_doc/parakeet#transformers.models.parakeet.feature_extraction_parakeet._LazyModule.__getattr__..Placeholder) and [TokenizersBackend](/docs/transformers/v5.17.0/en/main_classes/tokenizer#transformers.TokenizersBackend). See the
[~ParakeetFeatureExtractor](/docs/transformers/v5.17.0/en/model_doc/parakeet#transformers.models.parakeet.feature_extraction_parakeet._LazyModule.__getattr__..Placeholder) and [~TokenizersBackend](/docs/transformers/v5.17.0/en/main_classes/tokenizer#transformers.TokenizersBackend) for more information.

#### apply_transcription_request[[transformers.CanaryProcessor.apply_transcription_request]]

```python
apply_transcription_request(audio: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor'], list[typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor']]]], source_language: str | list[str] = 'en', target_language: str | list[str] | None = None, punctuation: bool = True, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/canary/processing_canary.py#L111)

**Parameters:**

audio (`AudioInput` or `list[AudioInput]`) : Audio to transcribe or translate. Can be a URL string, local path, numpy array, or a list of these.

source_language (`str` or `list[str]`, *optional*, defaults to `"en"`) : The language of the input speech. Accepts ISO codes (e.g. `"en"`, `"de"`, `"fr"`) or full names (e.g. `"English"`, `"German"`, `"French"`).

target_language (`str` or `list[str]`, *optional*) : The language of the output text. Accepts ISO codes or full names. Defaults to `source_language` (transcription); set it to a different language for speech-to-text translation.

punctuation (`bool`, *optional*, defaults to `True`) : Whether to request punctuation and capitalization in the output.

- ****kwargs** : Additional keyword arguments forwarded to [apply_chat_template()](/docs/transformers/v5.17.0/en/main_classes/processors#transformers.ProcessorMixin.apply_chat_template).

**Returns:** [BatchFeature](/docs/transformers/v5.17.0/en/main_classes/image_processor#transformers.BatchFeature)

Processor outputs ready to be passed to
[CanaryForConditionalGeneration.generate()](/docs/transformers/v5.17.0/en/main_classes/text_generation#transformers.GenerationMixin.generate).

Prepare inputs for transcription or translation without manually writing the chat template.

## CanaryModel[[transformers.CanaryModel]]

#### transformers.CanaryModel[[transformers.CanaryModel]]

```python
transformers.CanaryModel(config: CanaryConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/canary/modeling_canary.py#L455)

**Parameters:**

config ([CanaryConfig](/docs/transformers/v5.17.0/en/model_doc/canary#transformers.CanaryConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Canary model (FastConformer encoder + Transformer decoder) outputting raw hidden-states without any
specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.CanaryModel.forward]]

```python
forward(input_features: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.LongTensor] = None, decoder_input_ids: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: tuple[tuple[torch.FloatTensor]] | None = None, past_key_values: transformers.cache_utils.EncoderDecoderCache | None = None, decoder_inputs_embeds: tuple[torch.FloatTensor] | None = None, decoder_position_ids: tuple[torch.LongTensor] | None = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/canary/modeling_canary.py#L483)

**Parameters:**

input_features (`torch.FloatTensor` of shape `(batch_size, audio_length)`) : Float values of the raw speech waveform. Raw speech waveform can be obtained by loading a `.flac` or `.wav` audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_features`, the [AutoFeatureExtractor](/docs/transformers/v5.17.0/en/model_doc/auto#transformers.AutoFeatureExtractor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`.

attention_mask (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

decoder_input_ids (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Indices of decoder input sequence tokens in the vocabulary.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.17.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are decoder input IDs?](../glossary#decoder-input-ids)

decoder_attention_mask (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Mask to avoid performing attention on certain token indices. By default, a causal mask will be used, to make sure the model can only look at previous inputs in order to predict the future.

encoder_outputs (`tuple[tuple[torch.FloatTensor]]`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

past_key_values (`~cache_utils.EncoderDecoderCache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

decoder_inputs_embeds (`tuple[torch.FloatTensor]` of shape `(batch_size, target_sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `decoder_input_ids` you can choose to directly pass an embedded representation. If `past_key_values` is used, optionally only the last `decoder_inputs_embeds` have to be input (see `past_key_values`). This is useful if you want more control over how to convert `decoder_input_ids` indices into associated vectors than the model's internal embedding lookup matrix.  If `decoder_input_ids` and `decoder_inputs_embeds` are both unset, `decoder_inputs_embeds` takes the value of `inputs_embeds`.

decoder_position_ids (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`) : Indices of positions of each input sequence tokens in the position embeddings. Used to calculate the position embeddings up to `config.decoder_config.max_position_embeddings`

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** [Seq2SeqModelOutput](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.Seq2SeqModelOutput) or `tuple(torch.FloatTensor)`

A [Seq2SeqModelOutput](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.Seq2SeqModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([CanaryConfig](/docs/transformers/v5.17.0/en/model_doc/canary#transformers.CanaryConfig)) and inputs.

The [CanaryModel](/docs/transformers/v5.17.0/en/model_doc/canary#transformers.CanaryModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the decoder of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [EncoderDecoderCache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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
>>> from transformers import AutoFeatureExtractor, CanaryModel
>>> from datasets import load_dataset

>>> model = CanaryModel.from_pretrained("UsefulSensors/canary-tiny")
>>> feature_extractor = AutoFeatureExtractor.from_pretrained("UsefulSensors/canary-tiny")
>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> inputs = feature_extractor(ds[0]["audio"]["array"], return_tensors="pt")
>>> input_features = inputs.input_features
>>> decoder_input_ids = torch.tensor([[1, 1]]) * model.config.decoder_start_token_id
>>> last_hidden_state = model(input_features, decoder_input_ids=decoder_input_ids).last_hidden_state
>>> list(last_hidden_state.shape)
[1, 2, 288]
```

## CanaryForConditionalGeneration[[transformers.CanaryForConditionalGeneration]]

#### transformers.CanaryForConditionalGeneration[[transformers.CanaryForConditionalGeneration]]

```python
transformers.CanaryForConditionalGeneration(config: CanaryConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/canary/modeling_canary.py#L578)

**Parameters:**

config ([CanaryConfig](/docs/transformers/v5.17.0/en/model_doc/canary#transformers.CanaryConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Canary model with a language modeling head. Can be used for multilingual automatic speech recognition and
speech-to-text translation.

This model inherits from [PreTrainedModel](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.CanaryForConditionalGeneration.forward]]

```python
forward(input_features: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.LongTensor] = None, decoder_input_ids: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: tuple[tuple[torch.FloatTensor]] | None = None, past_key_values: transformers.cache_utils.EncoderDecoderCache | None = None, decoder_inputs_embeds: tuple[torch.FloatTensor] | None = None, decoder_position_ids: tuple[torch.LongTensor] | None = None, use_cache: bool | None = None, labels: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/canary/modeling_canary.py#L598)

**Parameters:**

input_features (`torch.FloatTensor` of shape `(batch_size, audio_length)`) : Float values of the raw speech waveform. Raw speech waveform can be obtained by loading a `.flac` or `.wav` audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_features`, the [AutoFeatureExtractor](/docs/transformers/v5.17.0/en/model_doc/auto#transformers.AutoFeatureExtractor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`.

attention_mask (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

decoder_input_ids (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Indices of decoder input sequence tokens in the vocabulary.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.17.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are decoder input IDs?](../glossary#decoder-input-ids)

decoder_attention_mask (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Mask to avoid performing attention on certain token indices. By default, a causal mask will be used, to make sure the model can only look at previous inputs in order to predict the future.

encoder_outputs (`tuple[tuple[torch.FloatTensor]]`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

past_key_values (`~cache_utils.EncoderDecoderCache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

decoder_inputs_embeds (`tuple[torch.FloatTensor]` of shape `(batch_size, target_sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `decoder_input_ids` you can choose to directly pass an embedded representation. If `past_key_values` is used, optionally only the last `decoder_inputs_embeds` have to be input (see `past_key_values`). This is useful if you want more control over how to convert `decoder_input_ids` indices into associated vectors than the model's internal embedding lookup matrix.  If `decoder_input_ids` and `decoder_inputs_embeds` are both unset, `decoder_inputs_embeds` takes the value of `inputs_embeds`.

decoder_position_ids (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`) : Indices of positions of each input sequence tokens in the position embeddings. Used to calculate the position embeddings up to `config.decoder_config.max_position_embeddings`

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

**Returns:** [Seq2SeqLMOutput](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or `tuple(torch.FloatTensor)`

A [Seq2SeqLMOutput](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([CanaryConfig](/docs/transformers/v5.17.0/en/model_doc/canary#transformers.CanaryConfig)) and inputs.

The [CanaryForConditionalGeneration](/docs/transformers/v5.17.0/en/model_doc/canary#transformers.CanaryForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [EncoderDecoderCache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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
>>> from transformers import AutoProcessor, CanaryForConditionalGeneration
>>> from datasets import load_dataset

>>> processor = AutoProcessor.from_pretrained("UsefulSensors/canary-tiny")
>>> model = CanaryForConditionalGeneration.from_pretrained("UsefulSensors/canary-tiny")

>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")

>>> inputs = processor(ds[0]["audio"]["array"], return_tensors="pt")
>>> input_features = inputs.input_features

>>> generated_ids = model.generate(input_features, max_new_tokens=100)

>>> transcription = processor.batch_decode(generated_ids, skip_special_tokens=True)[0]
>>> transcription
'Mr. Quilter is the apostle of the middle classes, and we are glad to welcome his gospel.'
```
