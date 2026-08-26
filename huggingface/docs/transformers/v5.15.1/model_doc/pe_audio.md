# PE Audio

[PE Audio](https://huggingface.co/papers/2504.13181) is the audio branch of Meta's Perception Encoder family. It contrastively aligns raw waveforms with text into a shared embedding space, trained on paired audio–caption data for cross-modal retrieval and zero-shot audio classification.

Two heads are exposed on top of the same encoder. [PeAudioModel](/docs/transformers/v5.15.1/en/model_doc/pe_audio#transformers.PeAudioModel) returns one pooled embedding per clip for clip-level retrieval, while [PeAudioFrameLevelModel](/docs/transformers/v5.15.1/en/model_doc/pe_audio#transformers.PeAudioFrameLevelModel) returns one embedding every 40 ms for event localization and fine-grained temporal analysis.

You can find all the official PE Audio checkpoints under the [perception-encoder-audio-visual](https://huggingface.co/collections/facebook/perception-encoder-audio-visual) collection.

## Quickstart

```py
import torch
from datasets import load_dataset
from transformers import AutoProcessor, PeAudioModel

processor = AutoProcessor.from_pretrained("facebook/pe-av-large")
model = PeAudioModel.from_pretrained(
    "facebook/pe-av-large",
    device_map="auto",
)

ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
audio = ds[0]["audio"]["array"]
labels = ["a dog barking", "a person speaking", "music playing"]

audio_inputs = processor.feature_extractor(audio, sampling_rate=48_000, return_tensors="pt").to(model.device)
text_inputs = processor.tokenizer(labels, padding=True, return_tensors="pt").to(model.device)
inputs = {**audio_inputs, **text_inputs}

with torch.no_grad():
    outputs = model(**inputs)

probs = outputs.logits_audio_text.sigmoid()
print({label: p.item() for label, p in zip(labels, probs[0])})
```

## Usage tips and notes

- Audio must be mono (`feature_size=1`) and resampled to 48 kHz — the feature extractor warns but does not resample for you. Stereo input is not supported.
- Variable-length audio is handled with `padding_mask` (not the usual `attention_mask`). The mask is downsampled internally by `dac_config.hop_length` before it reaches the encoder, so pass the raw waveform-resolution mask that the feature extractor returns.
- [PeAudioModel](/docs/transformers/v5.15.1/en/model_doc/pe_audio#transformers.PeAudioModel) returns logits of shape `(n_audio, n_text)`. [PeAudioFrameLevelModel](/docs/transformers/v5.15.1/en/model_doc/pe_audio#transformers.PeAudioFrameLevelModel) returns `(n_audio, n_text, n_frames)` with one frame every 40 ms. Pick the class that matches the task — they share weights so swapping is cheap.
- The text tower is a shared encoder loaded via `AutoModel` from `config.text_config`. The tokenizer is attached to the processor via `AutoTokenizer`, not a dedicated class.

## PeAudioConfig[[transformers.PeAudioConfig]]

#### transformers.PeAudioConfig[[transformers.PeAudioConfig]]

```python
transformers.PeAudioConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, audio_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_audio/configuration_pe_audio.py#L90)

**Parameters:**

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

audio_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the audio backbone.

This is the configuration class to store the configuration of a PeAudioModel. It is used to instantiate a Pe Audio
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/pe-av-large](https://huggingface.co/facebook/pe-av-large)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import PeAudioModel, PeAudioConfig

>>> # Initializing a PeAudioModel style configuration
>>> configuration = PeAudioConfig()

>>> # Initializing a model from the pe-av-large style configuration
>>> model = PeAudioModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## PeAudioEncoderConfig[[transformers.PeAudioEncoderConfig]]

#### transformers.PeAudioEncoderConfig[[transformers.PeAudioEncoderConfig]]

```python
transformers.PeAudioEncoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, dac_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, hidden_size: int = 1792, intermediate_size: int = 4800, num_hidden_layers: int = 6, num_attention_heads: int = 14, num_key_value_heads: int | None = None, head_dim: int = 128, hidden_act: str = 'silu', max_position_embeddings: int = 10000, initializer_range: float = 0.02, rms_norm_eps: float = 1e-05, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, attention_bias: bool = False, attention_dropout: float | int = 0.0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_audio/configuration_pe_audio.py#L26)

**Parameters:**

dac_config (`Union[PreTrainedConfig, dict]`, *optional*) : Configuration for the DAC audio encoder used to tokenize the raw audio inputs. If a dictionary is passed, it will be used to instantiate a [DacConfig](/docs/transformers/v5.15.1/en/model_doc/dac#transformers.DacConfig) with default DAC hyperparameters.

hidden_size (`int`, *optional*, defaults to `1792`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `4800`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `6`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `14`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

head_dim (`int`, *optional*, defaults to `128`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `10000`) : The maximum sequence length that this model might ever be used with.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

rms_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the rms normalization layers.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

attention_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

This is the configuration class to store the configuration of a PeAudioModel. It is used to instantiate a Pe Audio
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/pe-av-large](https://huggingface.co/facebook/pe-av-large)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import PeAudioEncoder, PeAudioEncoderConfig

>>> # Initializing a PeAudioEncoder style configuration
>>> configuration = PeAudioEncoderConfig()

>>> # Initializing a model from the pe-av-large style configuration
>>> model = PeAudioEncoder(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## PeAudioFeatureExtractor[[transformers.PeAudioFeatureExtractor]]

#### transformers.PeAudioFeatureExtractor[[transformers.PeAudioFeatureExtractor]]

```python
transformers.PeAudioFeatureExtractor(feature_size: int = 1, sampling_rate: int = 48000, padding_value: float = 0.0, hop_length: int = 1920, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_audio/feature_extraction_pe_audio.py#L26)

**Parameters:**

feature_size (`int`, *optional*, defaults to 1) : The feature dimension of the extracted features. Use 1 for mono, 2 for stereo.

sampling_rate (`int`, *optional*, defaults to 48000) : The sampling rate at which the audio waveform should be digitalized, expressed in hertz (Hz).

padding_value (`float`, *optional*, defaults to 0.0) : The value that is used for padding.

hop_length (`int`, *optional*, defaults to 1920) : Overlap length between successive windows.

Constructs a PeAudioFeatureExtractor feature extractor.

This feature extractor inherits from [SequenceFeatureExtractor](/docs/transformers/v5.15.1/en/main_classes/feature_extractor#transformers.SequenceFeatureExtractor) which contains
most of the main methods. Users should refer to this superclass for more information regarding those methods.

#### __call__[[transformers.PeAudioFeatureExtractor.__call__]]

```python
__call__(raw_audio: numpy.ndarray | list[float] | list[numpy.ndarray] | list[list[float]] | str | list[str], padding: bool | str | transformers.utils.generic.PaddingStrategy | None = None, truncation: bool | None = False, max_length: int | None = None, return_tensors: str | transformers.utils.generic.TensorType | None = None, sampling_rate: int | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_audio/feature_extraction_pe_audio.py#L63)

## PeAudioProcessor[[transformers.PeAudioProcessor]]

#### transformers.PeAudioProcessor[[transformers.PeAudioProcessor]]

```python
transformers.PeAudioProcessor(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_audio/processing_pe_audio.py#L17)

## PeAudioEncoder[[transformers.PeAudioEncoder]]

#### transformers.PeAudioEncoder[[transformers.PeAudioEncoder]]

```python
transformers.PeAudioEncoder(config: PeAudioEncoderConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_audio/modeling_pe_audio.py#L616)

**Parameters:**

config ([PeAudioEncoderConfig](/docs/transformers/v5.15.1/en/model_doc/pe_audio#transformers.PeAudioEncoderConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The PeAudio Encoder model.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.PeAudioEncoder.forward]]

```python
forward(input_values: Tensor, padding_mask: typing.Optional[torch.Tensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_audio/modeling_pe_audio.py#L635)

**Parameters:**

input_values (`torch.Tensor` of shape `(batch_size, sequence_length)`) : Float values of input raw speech waveform. Values can be obtained by loading a `.flac` or `.wav` audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_values`, the [AutoProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoProcessor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`. See [PeAudioProcessor.__call__()](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessorMixin.__call__) for details.

padding_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding samples of `input_values`. Mask values selected in `[0, 1]`:  - 1 for samples that are **not masked**, - 0 for samples that are **masked**.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PeAudioConfig](/docs/transformers/v5.15.1/en/model_doc/pe_audio#transformers.PeAudioConfig)) and inputs.

The [PeAudioEncoder](/docs/transformers/v5.15.1/en/model_doc/pe_audio#transformers.PeAudioEncoder) forward method, overrides the `__call__` special method.

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

## PeAudioModel[[transformers.PeAudioModel]]

#### transformers.PeAudioModel[[transformers.PeAudioModel]]

```python
transformers.PeAudioModel(config: PeAudioConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_audio/modeling_pe_audio.py#L721)

#### forward[[transformers.PeAudioModel.forward]]

```python
forward(input_ids: Tensor, input_values: Tensor, attention_mask: typing.Optional[torch.Tensor] = None, padding_mask: typing.Optional[torch.Tensor] = None, return_loss: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_audio/modeling_pe_audio.py#L754)

**Parameters:**

input_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

input_values (`torch.Tensor` of shape `(batch_size, sequence_length)`) : Float values of input raw speech waveform. Values can be obtained by loading a `.flac` or `.wav` audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_values`, the [AutoProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoProcessor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`. See [PeAudioProcessor.__call__()](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessorMixin.__call__) for details.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

padding_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding samples of `input_values`. Mask values selected in `[0, 1]`:  - 1 for samples that are **not masked**, - 0 for samples that are **masked**.

return_loss (`bool`, *optional*) : Whether or not to return the loss.

**Returns:** `PeAudioOutput` or `tuple(torch.FloatTensor)`

A `PeAudioOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PeAudioConfig](/docs/transformers/v5.15.1/en/model_doc/pe_audio#transformers.PeAudioConfig)) and inputs.

The [PeAudioModel](/docs/transformers/v5.15.1/en/model_doc/pe_audio#transformers.PeAudioModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*) -- Contrastive loss computed between audio and text representations.
- **logits_audio_text** (`torch.FloatTensor` of shape `(batch_size, batch_size)`, *optional*) -- Similarity logits between audio and text embeddings. [PeAudioFrameLevelModel](/docs/transformers/v5.15.1/en/model_doc/pe_audio#transformers.PeAudioFrameLevelModel) returns per-frame logits of
  shape `(batch_size, batch_size, sequence_length)` instead.
- **text_audio_embeds** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`, *optional*) -- Text embeddings projected to the audio-text space.
- **audio_embeds** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`, *optional*) -- Audio embeddings projected to the audio-text space. [PeAudioFrameLevelModel](/docs/transformers/v5.15.1/en/model_doc/pe_audio#transformers.PeAudioFrameLevelModel) returns per-frame embeddings of
  shape `(batch_size, sequence_length, hidden_size)` instead.
- **text_outputs** (`BaseModelOutputWithPooling`, *optional*) -- Model outputs for the text encoder, including last hidden state and pooled output.
- **audio_outputs** (`BaseModelOutputWithPooling`, *optional*) -- Model outputs for the audio encoder, including last hidden state and pooled output.

## PeAudioFrameLevelModel[[transformers.PeAudioFrameLevelModel]]

#### transformers.PeAudioFrameLevelModel[[transformers.PeAudioFrameLevelModel]]

```python
transformers.PeAudioFrameLevelModel(config: PeAudioConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_audio/modeling_pe_audio.py#L810)

#### forward[[transformers.PeAudioFrameLevelModel.forward]]

```python
forward(input_ids: Tensor, input_values: Tensor, attention_mask: typing.Optional[torch.Tensor] = None, padding_mask: typing.Optional[torch.Tensor] = None, return_loss: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_audio/modeling_pe_audio.py#L821)

**Parameters:**

input_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

input_values (`torch.Tensor` of shape `(batch_size, sequence_length)`) : Float values of input raw speech waveform. Values can be obtained by loading a `.flac` or `.wav` audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_values`, the [AutoProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoProcessor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`. See [PeAudioProcessor.__call__()](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessorMixin.__call__) for details.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

padding_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding samples of `input_values`. Mask values selected in `[0, 1]`:  - 1 for samples that are **not masked**, - 0 for samples that are **masked**.

return_loss (`bool`, *optional*) : Whether or not to return the loss.

**Returns:** `PeAudioOutput` or `tuple(torch.FloatTensor)`

A `PeAudioOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PeAudioConfig](/docs/transformers/v5.15.1/en/model_doc/pe_audio#transformers.PeAudioConfig)) and inputs.

The [PeAudioFrameLevelModel](/docs/transformers/v5.15.1/en/model_doc/pe_audio#transformers.PeAudioFrameLevelModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*) -- Contrastive loss computed between audio and text representations.
- **logits_audio_text** (`torch.FloatTensor` of shape `(batch_size, batch_size)`, *optional*) -- Similarity logits between audio and text embeddings. [PeAudioFrameLevelModel](/docs/transformers/v5.15.1/en/model_doc/pe_audio#transformers.PeAudioFrameLevelModel) returns per-frame logits of
  shape `(batch_size, batch_size, sequence_length)` instead.
- **text_audio_embeds** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`, *optional*) -- Text embeddings projected to the audio-text space.
- **audio_embeds** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`, *optional*) -- Audio embeddings projected to the audio-text space. [PeAudioFrameLevelModel](/docs/transformers/v5.15.1/en/model_doc/pe_audio#transformers.PeAudioFrameLevelModel) returns per-frame embeddings of
  shape `(batch_size, sequence_length, hidden_size)` instead.
- **text_outputs** (`BaseModelOutputWithPooling`, *optional*) -- Model outputs for the text encoder, including last hidden state and pooled output.
- **audio_outputs** (`BaseModelOutputWithPooling`, *optional*) -- Model outputs for the audio encoder, including last hidden state and pooled output.
