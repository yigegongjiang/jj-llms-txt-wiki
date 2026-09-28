# GraniteSpeech5

## Overview

Granite Speech 5.0 Turbo CTC is a lightweight (~470M parameters) conformer encoder for automatic speech recognition, trained with Connectionist Temporal Classification (CTC) on BPE targets. It is a fast, encoder-only member of the [Granite Speech](https://huggingface.co/papers/2505.08699) family: transcription requires a single forward pass followed by greedy CTC decoding, with no autoregressive decoder.

Architecturally, it extends the Granite Speech conformer CTC encoder with:

1. **Frame stacking + block-wise time subsampling**: the feature extractor stacks pairs of log-mel(+delta) frames (2x), and the first two conformer blocks each subsample time by 2 through a stride-2 depthwise convolution (with a mean-pooled residual), for a total 8x time reduction at 10 ms mel hop.

2. **Block attention with Shaw's relative positional embeddings**: attention is computed over fixed-size blocks (the sequence is right-padded to a whole number of blocks, with padded frames masked out), using separate bias-free query/key/value projections.

3. **Self-conditioned CTC**: the CTC posteriors of the middle layer are projected and fed back into the hidden states, and the CTC head is shared between this mid-layer self-conditioning and the final prediction.

This model was contributed by [Eustache Le Bihan](https://huggingface.co/eustlb).

## Usage

### `GraniteSpeech5ForCTC` usage

```python
from transformers import pipeline

pipe = pipeline("automatic-speech-recognition", model="ibm-granite/granite-speech-5.0-470m-turboctc")
out = pipe("https://huggingface.co/datasets/hf-internal-testing/dummy-audio-samples/resolve/main/bcn_weather.mp3")
print(out)
# {'text': 'yesterday it was 35 degrees in barcelona but today the temperature will go down to -20 degrees'}
```

```python
from datasets import Audio, load_dataset
from transformers import AutoModelForCTC, AutoProcessor

model_id = "ibm-granite/granite-speech-5.0-470m-turboctc"
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForCTC.from_pretrained(model_id, device_map="auto")

ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))
speech_samples = [el['array'] for el in ds["audio"][:5]]

# `device` computes the log-mel front-end on the model's accelerator, saving a host-to-device copy
inputs = processor(
    speech_samples, sampling_rate=processor.feature_extractor.sampling_rate, device=model.device
)
inputs.to(model.device, dtype=model.dtype)
outputs = model.generate(**inputs)
print(processor.batch_decode(outputs, skip_special_tokens=True))
# ['mister quilter is the apostle of the middle classes and we are glad to welcome his gospel', ...]
```

## GraniteSpeech5CTCConfig[[transformers.GraniteSpeech5CTCConfig]]

#### transformers.GraniteSpeech5CTCConfig[[transformers.GraniteSpeech5CTCConfig]]

```python
transformers.GraniteSpeech5CTCConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 16384, ctc_loss_reduction: str = 'mean', ctc_zero_infinity: bool = True, encoder_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, pad_token_id: int | None = 0, tie_word_embeddings: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/granite_speech5/configuration_granite_speech5.py#L97)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `16384`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

ctc_loss_reduction (`str`, *optional*, defaults to `"mean"`) : Specifies the reduction to apply to the output of `torch.nn.CTCLoss`. Only relevant when training an instance of [GraniteSpeech5ForCTC](/docs/transformers/v5.17.0/en/model_doc/granite_speech5#transformers.GraniteSpeech5ForCTC).

ctc_zero_infinity (`bool`, *optional*, defaults to `True`) : Whether to zero infinite losses and the associated gradients of `torch.nn.CTCLoss`. Infinite losses mainly occur when the inputs are too short to be aligned to the targets. Only relevant when training an instance of [GraniteSpeech5ForCTC](/docs/transformers/v5.17.0/en/model_doc/granite_speech5#transformers.GraniteSpeech5ForCTC).

encoder_config (`Union[dict, GraniteSpeech5EncoderConfig]`, *optional*) : The config object or dictionary of the encoder.

pad_token_id (`int`, *optional*, defaults to `0`) : Token id used for padding in the vocabulary.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Granite Speech5Model. It is used to instantiate a Granite Speech5
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [ibm-granite/granite-speech-5.0-470m-turboctc](https://huggingface.co/ibm-granite/granite-speech-5.0-470m-turboctc)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import GraniteSpeech5ForCTC, GraniteSpeech5CTCConfig
>>> # Initializing a GraniteSpeech5 configuration
>>> configuration = GraniteSpeech5CTCConfig()
>>> # Initializing a model from the configuration
>>> model = GraniteSpeech5ForCTC(configuration)
>>> # Accessing the model configuration
>>> configuration = model.config
```

## GraniteSpeech5EncoderConfig[[transformers.GraniteSpeech5EncoderConfig]]

#### transformers.GraniteSpeech5EncoderConfig[[transformers.GraniteSpeech5EncoderConfig]]

```python
transformers.GraniteSpeech5EncoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 16384, hidden_size: int = 1024, intermediate_size: int = 4096, num_hidden_layers: int = 16, num_attention_heads: int = 8, num_key_value_heads: int | None = None, num_mel_bins: int = 80, head_dim: int | None = None, hidden_act: str = 'silu', max_position_embeddings: int = 512, context_size: int = 128, conv_kernel_size: int = 7, conv_expansion_factor: int = 2, subsample_layers: list[int] | None = None, attention_bias: bool = True, attention_dropout: float | int = 0.0, activation_dropout: float | int = 0.0, initializer_range: float = 0.02)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/granite_speech5/configuration_granite_speech5.py#L29)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `16384`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `1024`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `4096`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `16`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

num_mel_bins (`int`, *optional*, defaults to `80`) : Number of mel features used per input frame. Should correspond to the value used in the `AutoFeatureExtractor` class.

head_dim (`int`, *optional*) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to 512) : Maximum relative position index of Shaw's relative positional encoding; the embedding table holds `2 * max_position_embeddings + 1` entries.

context_size (`int`, *optional*, defaults to 128) : Context size for block-wise conformer attention.

conv_kernel_size (`int`, *optional*, defaults to 7) : Kernel size of the depthwise convolution in the conformer convolution module.

conv_expansion_factor (`int`, *optional*, defaults to 2) : Expansion factor for the conformer convolution module.

subsample_layers (`list[int]`, *optional*, defaults to `[0, 1]`) : Indices of the conformer blocks that subsample time by 2 (stride-2 depthwise convolution with a mean-pooled residual).

attention_bias (`bool`, *optional*, defaults to `True`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

activation_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for activations inside the fully connected layer.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Granite Speech5Model. It is used to instantiate a Granite Speech5
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [ibm-granite/granite-speech-5.0-470m-turboctc](https://huggingface.co/ibm-granite/granite-speech-5.0-470m-turboctc)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import GraniteSpeech5EncoderConfig, GraniteSpeech5Encoder

>>> # Initializing a GraniteSpeech5EncoderConfig
>>> configuration = GraniteSpeech5EncoderConfig()

>>> # Initializing a GraniteSpeech5Encoder (with random weights)
>>> model = GraniteSpeech5Encoder(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## GraniteSpeech5FeatureExtractor[[transformers.GraniteSpeech5FeatureExtractor]]

#### transformers.GraniteSpeech5FeatureExtractor[[transformers.GraniteSpeech5FeatureExtractor]]

```python
transformers.GraniteSpeech5FeatureExtractor(num_mel_bins: int = 80, sampling_rate: int = 16000, n_fft: int = 512, win_length: int = 400, hop_length: int = 160, delta_win_length: int = 3, logmel_floor_db: float = 8.0, padding_value: float = 0.0, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/granite_speech5/feature_extraction_granite_speech5.py#L33)

**Parameters:**

num_mel_bins (`int`, *optional*, defaults to 80) : Number of mel filter banks.

sampling_rate (`int`, *optional*, defaults to 16000) : The sampling rate at which the audio files should be digitalized expressed in hertz (Hz).

n_fft (`int`, *optional*, defaults to 512) : Size of the Fourier transform.

win_length (`int`, *optional*, defaults to 400) : Window length in samples.

hop_length (`int`, *optional*, defaults to 160) : Length of the overlapping windows for the STFT used to obtain the mel spectrogram, in samples.

delta_win_length (`int`, *optional*, defaults to 3) : Window length used to compute the delta features.

logmel_floor_db (`float`, *optional*, defaults to 8.0) : The log-mel features are floored at this many dB below the per-sample maximum.

padding_value (`float`, *optional*, defaults to 0.0) : Padding value used to pad the audio.

Constructs a Granite Speech 5.0 feature extractor.

This feature extractor inherits from [SequenceFeatureExtractor](/docs/transformers/v5.17.0/en/main_classes/feature_extractor#transformers.SequenceFeatureExtractor) which contains
most of the main methods. Users should refer to this superclass for more information regarding those methods.

## GraniteSpeech5Processor[[transformers.GraniteSpeech5Processor]]

#### transformers.GraniteSpeech5Processor[[transformers.GraniteSpeech5Processor]]

```python
transformers.GraniteSpeech5Processor(feature_extractor, tokenizer)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/granite_speech5/processing_granite_speech5.py#L32)

**Parameters:**

feature_extractor (`feature_extractor_class`) : The feature extractor is a required input.

tokenizer (`tokenizer_class`) : The tokenizer is a required input.

Constructs a GraniteSpeech5Processor which wraps a feature extractor and a tokenizer into a single processor.

[GraniteSpeech5Processor](/docs/transformers/v5.17.0/en/model_doc/granite_speech5#transformers.GraniteSpeech5Processor) offers all the functionalities of `feature_extractor_class` and `tokenizer_class`. See the
`~feature_extractor_class` and `~tokenizer_class` for more information.

## GraniteSpeech5Encoder[[transformers.GraniteSpeech5Encoder]]

#### transformers.GraniteSpeech5Encoder[[transformers.GraniteSpeech5Encoder]]

```python
transformers.GraniteSpeech5Encoder(config: GraniteSpeech5EncoderConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/granite_speech5/modeling_granite_speech5.py#L368)

**Parameters:**

config ([GraniteSpeech5EncoderConfig](/docs/transformers/v5.17.0/en/model_doc/granite_speech5#transformers.GraniteSpeech5EncoderConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Granite Speech 5.0 conformer encoder, adapted from the [Granite Speech CTC encoder](https://huggingface.co/papers/2505.08699)
with block-wise time subsampling and self-conditioned CTC from the middle layer.

This model inherits from [PreTrainedModel](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.GraniteSpeech5Encoder.forward]]

```python
forward(input_features: Tensor, attention_mask: typing.Optional[torch.Tensor] = None, output_attention_mask: bool = True, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/granite_speech5/modeling_granite_speech5.py#L395)

**Parameters:**

input_features (`torch.Tensor` of shape `(batch_size, sequence_length, feature_dim)`) : The tensors corresponding to the input audio features. Audio features can be obtained using `feature_extractor_class`. See `feature_extractor_class.__call__` for details (`processor_class` uses `feature_extractor_class` for processing audios).

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

output_attention_mask (`bool`, *optional*, defaults to `True`) : Whether to return the output attention mask. Only effective when `attention_mask` is provided.

**Returns:** `GraniteSpeech5EncoderModelOutput` or `tuple(torch.FloatTensor)`

A `GraniteSpeech5EncoderModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.

The [GraniteSpeech5Encoder](/docs/transformers/v5.17.0/en/model_doc/granite_speech5#transformers.GraniteSpeech5Encoder) forward method, overrides the `__call__` special method.

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
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) -- Mask to avoid performing attention on padding token indices after sequence compression. Returned because the
  sequence length may differ from the input sequence length. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

Example:

```python
>>> from transformers import AutoProcessor, GraniteSpeech5Encoder
>>> from datasets import load_dataset, Audio

>>> model_id = "ibm-granite/granite-speech-5.0-470m-turboctc"
>>> processor = AutoProcessor.from_pretrained(model_id)
>>> encoder = GraniteSpeech5Encoder.from_pretrained(model_id)

>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))

>>> inputs = processor(ds[0]["audio"]["array"])
>>> encoder_outputs = encoder(**inputs)

>>> print(encoder_outputs.last_hidden_state.shape)
```

## GraniteSpeech5ForCTC[[transformers.GraniteSpeech5ForCTC]]

#### transformers.GraniteSpeech5ForCTC[[transformers.GraniteSpeech5ForCTC]]

```python
transformers.GraniteSpeech5ForCTC(config: GraniteSpeech5CTCConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/granite_speech5/modeling_granite_speech5.py#L496)

**Parameters:**

config ([GraniteSpeech5CTCConfig](/docs/transformers/v5.17.0/en/model_doc/granite_speech5#transformers.GraniteSpeech5CTCConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Granite Speech 5.0 encoder with a Connectionist Temporal Classification (CTC) head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.GraniteSpeech5ForCTC.forward]]

```python
forward(input_features: Tensor, attention_mask: typing.Optional[torch.Tensor] = None, labels: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/granite_speech5/modeling_granite_speech5.py#L511)

**Parameters:**

input_features (`torch.Tensor` of shape `(batch_size, sequence_length, feature_dim)`) : The tensors corresponding to the input audio features. Audio features can be obtained using `feature_extractor_class`. See `feature_extractor_class.__call__` for details (`processor_class` uses `feature_extractor_class` for processing audios).

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

labels (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

**Returns:** [CausalLMOutput](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutput) or `tuple(torch.FloatTensor)`

A [CausalLMOutput](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.

The [GraniteSpeech5ForCTC](/docs/transformers/v5.17.0/en/model_doc/granite_speech5#transformers.GraniteSpeech5ForCTC) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoProcessor, GraniteSpeech5ForCTC
>>> from datasets import load_dataset, Audio

>>> model_id = "nvidia/granite_speech5-ctc-1.1b"
>>> processor = AutoProcessor.from_pretrained(model_id)
>>> model = GraniteSpeech5ForCTC.from_pretrained(model_id)

>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))

>>> inputs = processor(ds[0]["audio"]["array"], text=ds[0]["text"])
>>> outputs = model(**inputs)

>>> print(outputs.loss)
```

#### generate[[transformers.GraniteSpeech5ForCTC.generate]]

```python
generate(input_features: Tensor, attention_mask: typing.Optional[torch.Tensor] = None, return_dict_in_generate: bool = False, compile_config: transformers.generation.configuration_utils.CompileConfig | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/granite_speech5/modeling_granite_speech5.py#L581)

compile_config ([CompileConfig](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.CompileConfig), *optional*):
If provided, `torch.compile` will be applied to the forward calls in the decoding loop.

Example:

```python
>>> from transformers import AutoProcessor, GraniteSpeech5ForCTC
>>> from datasets import load_dataset, Audio

>>> model_id = "ibm-granite/granite-speech-5.0-470m-turboctc"
>>> processor = AutoProcessor.from_pretrained(model_id)
>>> model = GraniteSpeech5ForCTC.from_pretrained(model_id)

>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))

>>> inputs = processor(ds[0]["audio"]["array"])
>>> predicted_ids = model.generate(**inputs)
>>> transcription = processor.batch_decode(predicted_ids, skip_special_tokens=True)

>>> print(transcription)
```
