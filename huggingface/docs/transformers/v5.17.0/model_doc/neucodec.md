# NeuCodec

## Overview

The NeuCodec model was proposed in [Finite Scalar Quantization Enables Redundant and Transmission-Robust Neural Audio Compression at Low Bit-rates](https://huggingface.co/papers/2509.09550).

NeuCodec is a neural audio codec extending on XCodec2. It takes advantage of the following features:

- Finite Scalar Quantization (FSQ) quantisation resulting in a **single codebook**, making it ideal for downstream modeling with Speech Language Models.
- Trained with CC data such that there are **no Non-Commercial data restrictions**.
- At 50 tokens/sec and 16 bits per token, the overall bit-rate is **0.8kbps**.
- The codec takes in 16kHz input and outputs **24kHz** using an **upsampling decoder**.
- The FSQ encoding scheme allows for bit-level error resistance suitable for unreliable and noisy channels.

The original modelling code can be found [here](https://github.com/neuphonic/neucodec).

## Usage example 

Here is a quick example of how to encode and decode an audio using this model:

```python 
from datasets import Audio, load_dataset
from transformers import AutoFeatureExtractor, AutoModel

model_id = "neuphonic/neucodec"
model = AutoModel.from_pretrained(model_id, device_map="auto")
feature_extractor = AutoFeatureExtractor.from_pretrained(model_id)

dataset = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
dataset = dataset.cast_column("audio", Audio(sampling_rate=feature_extractor.sampling_rate))
audio = dataset[0]["audio"]["array"]
inputs = feature_extractor(audio=audio, sampling_rate=feature_extractor.sampling_rate, return_tensors="pt").to(
    model.device, model.dtype
)
print("Input waveform shape:", inputs["input_values"].shape)
# Input waveform shape: torch.Size([1, 1, 93760])

# encoder and decoder
audio_codes = model.encode(**inputs).audio_codes
print("Audio codes shape:", audio_codes.shape)
# Audio codes shape: torch.Size([1, 1, 292])
audio_values = model.decode(audio_codes).audio_values
print("Audio values shape:", audio_values.shape)

# Equivalently, you can do encoding and decoding in one step
model_output = model(**inputs)
audio_codes = model_output.audio_codes
audio_values = model_output.audio_values
```

### Batch processing

This implementation also supports batched input!

```python
from datasets import Audio, load_dataset
from transformers import AutoFeatureExtractor, AutoModel

batch_size = 2
model_id = "neuphonic/neucodec"
model = AutoModel.from_pretrained(model_id, device_map="auto")
feature_extractor = AutoFeatureExtractor.from_pretrained(model_id)

dataset = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
dataset = dataset.cast_column("audio", Audio(sampling_rate=feature_extractor.sampling_rate))
audios = [dataset[i]["audio"]["array"] for i in range(batch_size)]
inputs = feature_extractor(audio=audios, sampling_rate=feature_extractor.sampling_rate, return_tensors="pt").to(
    model.device, model.dtype
)
print("Input waveform shape:", inputs["input_values"].shape)
# Input waveform shape: torch.Size([2, 1, 93760])

# encoder and decoder
encoder_output = model.encode(**inputs)
audio_codes = encoder_output.audio_codes
print("Audio codes shape:", audio_codes.shape)
# Audio codes shape: torch.Size([2, 1, 292])
audio_values = model.decode(audio_codes).audio_values
print("Audio values shape:", audio_values.shape)

# Equivalently, you can do encoding and decoding in one step
model_output = model(**inputs)
audio_codes = model_output.audio_codes
audio_values = model_output.audio_values
```

### Speed-up with `torch.compile`

You can speed up inference with [`torch.compile`](https://pytorch.org/docs/stable/generated/torch.compile.html). The first few calls will be slower due to compilation overhead, but subsequent calls will be faster.

```python
import torch
from datasets import Audio, load_dataset
from transformers import AutoFeatureExtractor, AutoModel

batch_size = 4
model_id = "neuphonic/neucodec"
model = AutoModel.from_pretrained(model_id, device_map="auto")
feature_extractor = AutoFeatureExtractor.from_pretrained(model_id)

dataset = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
dataset = dataset.cast_column("audio", Audio(sampling_rate=feature_extractor.sampling_rate))
audios = [dataset[i]["audio"]["array"] for i in range(batch_size)]
inputs = feature_extractor(
    audio=audios, sampling_rate=feature_extractor.sampling_rate, padding=True, return_tensors="pt"
).to(model.device, model.dtype)

compiled_model = torch.compile(model, fullgraph=True)

# Warmup (includes compilation on first call)
for _ in range(10):
    with torch.inference_mode():
        _ = compiled_model(**inputs)

with torch.inference_mode():
    output = compiled_model(**inputs)
print("Audio values shape:", output.audio_values.shape)
```

## NeuCodecConfig[[transformers.NeuCodecConfig]]

#### transformers.NeuCodecConfig[[transformers.NeuCodecConfig]]

```python
transformers.NeuCodecConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 1024, intermediate_size: int = 4096, num_hidden_layers: int = 12, num_attention_heads: int = 16, num_key_value_heads: int = 16, hidden_act: str = 'silu', max_position_embeddings: int = 4096, initializer_range: float = 0.02, rms_norm_eps: float = 1e-06, pad_token_id: int | None = None, tie_word_embeddings: bool = False, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, attention_bias: bool = False, attention_dropout: int | float | None = 0.0, head_dim: int = 64, encoder_hidden_size: int = 48, downsampling_ratios: list[int] | tuple[int, ...] = (2, 2, 4, 4, 5), semantic_model_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, activation_dropout: float = 0.1, quantization_dim: int = 2048, quantization_levels: list[int] | tuple[int, ...] = (4, 4, 4, 4, 4, 4, 4, 4), input_sampling_rate: int = 16000, output_sampling_rate: int = 24000)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/neucodec/configuration_neucodec.py#L33)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `1024`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `4096`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `16`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `4096`) : The maximum sequence length that this model might ever be used with.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

rms_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the rms normalization layers.

pad_token_id (`int`, *optional*) : Token id used for padding in the vocabulary.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

attention_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

attention_dropout (`Union[int, float]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

head_dim (`int`, *optional*, defaults to `64`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

encoder_hidden_size (`int`, *optional*, defaults to `48`) : Dimension of the hidden representations.

downsampling_ratios (`list[int]`, *optional*, defaults to `[2, 2, 4, 4, 5]`) : Ratios for downsampling in the encoder.

semantic_model_config (`Union[Dict, Wav2Vec2BertConfig]`, *optional*) : An instance of the configuration object for the semantic (Wav2Vec2BertConfig) model.

activation_dropout (`float`, *optional*, defaults to `0.1`) : The dropout ratio for activations inside the fully connected layer.

quantization_dim (`int`, *optional*, defaults to 2048) : Dimension for the vector quantization codebook.

quantization_levels (`list[int]`, *optional*, defaults to `[4, 4, 4, 4, 4, 4, 4, 4]`) : Levels for the vector quantization codebook.

input_sampling_rate (`int`, *optional*, defaults to 16000) : Sampling rate, in hertz (Hz), of the encoder's input audio waveform.

output_sampling_rate (`int`, *optional*, defaults to 24000) : Sampling rate, in hertz (Hz), of the decoder's output audio waveform.

This is the configuration class to store the configuration of a NeuCodecModel. It is used to instantiate a Neucodec
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [neuphonic/neucodec](https://huggingface.co/neuphonic/neucodec)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import NeuCodecConfig, NeuCodecModel

>>> # Initializing configuration
>>> configuration = NeuCodecConfig()

>>> # Initializing a model (with random weights) from the configuration
>>> model = NeuCodecModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## NeuCodecFeatureExtractor[[transformers.NeuCodecFeatureExtractor]]

#### transformers.NeuCodecFeatureExtractor[[transformers.NeuCodecFeatureExtractor]]

```python
transformers.NeuCodecFeatureExtractor(feature_size = 80, sampling_rate = 16000, padding_value = 1.0, hop_length = 320, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/neucodec/feature_extraction_neucodec.py#L42)

**Parameters:**

feature_size (`int`, *optional*, defaults to 80) : The feature dimension of the extracted features.

sampling_rate (`int`, *optional*, defaults to 16000) : The sample rate at which the audio files should be digitalized expressed in hertz (Hz).

padding_value (`float`, *optional*, defaults to 1.0) : The value that is used to fill the padding vectors for the mel spectrogram.

hop_length (`int`, *optional*, defaults to 320) : Number of audio samples encoded per frame. Equivalent to product of downsampling ratios. Needed for acoustic encoder input padding.

Constructs a NeuCodec feature extractor, which computes mel-filter bank features for the semantic encoder and padded
audio for the acoustic encoder.

This feature extractor inherits from [SequenceFeatureExtractor](/docs/transformers/v5.17.0/en/main_classes/feature_extractor#transformers.SequenceFeatureExtractor) which contains most of the main methods. Users
should refer to this superclass for more information regarding those methods.

#### __call__[[transformers.NeuCodecFeatureExtractor.__call__]]

```python
__call__(audio: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor']], padding: bool | str | transformers.utils.generic.PaddingStrategy = True, max_length: int | None = None, truncation: bool = False, return_tensors: str | transformers.utils.generic.TensorType | None = None, sampling_rate: int | None = None, device: str = 'cpu', **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/neucodec/feature_extraction_neucodec.py#L90)

**Parameters:**

audio (`np.ndarray`, `torch.Tensor`, `list[np.ndarray]`, `list[torch.Tensor]`) : Numpy array or torch tensor with shape (num_channels, sequence_length). A list of such arrays or tensors can also be provided for a batch of inputs.

padding (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.17.0/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `True`) : Select a strategy to pad the returned sequences (according to the model's padding side and padding index) among:  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single sequence if provided). - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different lengths).

max_length (`int`, *optional*) : Maximum length of the returned list and optionally padding length (see above).

truncation (`bool`) : Activates truncation to cut input sequences longer than *max_length* to *max_length*.

return_tensors (`str` or [TensorType](/docs/transformers/v5.17.0/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors instead of list of python integers. Acceptable values are:  - `'tf'`: Return TensorFlow `tf.constant` objects. - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return Numpy `np.ndarray` objects.

sampling_rate (`int`, *optional*) : The sample rate at which the `audio` input was sampled. It is strongly recommended to pass `sampling_rate` at the forward call to prevent silent errors.

device (`str`, *optional*, defaults to `"cpu"`) : Device for PyTorch tensors during mel-filter bank feature extraction.

kwargs (*optional*) : Remaining dictionary of keyword arguments that will be passed to the tokenizer or the feature extractor.

## NeuCodecModel[[transformers.NeuCodecModel]]

#### transformers.NeuCodecModel[[transformers.NeuCodecModel]]

```python
transformers.NeuCodecModel(config: NeuCodecConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/neucodec/modeling_neucodec.py#L959)

**Parameters:**

config ([NeuCodecConfig](/docs/transformers/v5.17.0/en/model_doc/neucodec#transformers.NeuCodecConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

NeuCodec neural audio codec model.

This model inherits from [PreTrainedModel](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### decode[[transformers.NeuCodecModel.decode]]

```python
decode(audio_codes: typing.Optional[torch.Tensor] = None, latents: typing.Optional[torch.Tensor] = None, audio_codes_mask: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/neucodec/modeling_neucodec.py#L1043)

**Parameters:**

audio_codes (`torch.LongTensor`  of shape `(batch_size, 1, codes_length)`) : Discrete code indices computed using `model.encode`.

latents (`torch.Tensor` of shape `(batch_size, dimension, time_steps)`, *optional*) : Quantized continuous representation of input.

audio_codes_mask (`torch.Tensor` of shape `(batch_size, codes_length)`, *optional*) : Mask marking valid (non-padding) positions, as returned by `model.encode`. Needed for correct batched decoding of variable-length audio.

**Returns:** `NeuCodecDecoderOutput` or `tuple(torch.FloatTensor)`

A `NeuCodecDecoderOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.

- **audio_values** (`torch.FloatTensor` of shape `(batch_size, 1, segment_length)`, *optional*) -- Decoded audio waveform values in the time domain, obtained by converting
  the discrete codes back into continuous audio signals. This represents
  the reconstructed audio that can be played back.

#### encode[[transformers.NeuCodecModel.encode]]

```python
encode(input_values: Tensor, input_features: Tensor, padding_mask: typing.Optional[torch.Tensor] = None, input_features_mask: typing.Optional[torch.Tensor] = None, output_latents: bool = False, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/neucodec/modeling_neucodec.py#L981)

**Parameters:**

input_values (`torch.Tensor` of shape `(batch_size, 1, sequence_length)`) : Input audio waveform.

input_features (`torch.Tensor` of shape `(batch_size, mel_bins, time_steps)`) : Input audio mel spectrogram for semantic encoding.

padding_mask (`torch.Tensor` of shape `(batch_size, 1, sequence_length)`) : Padding mask used to pad `input_values`.

input_features_mask (`torch.Tensor` of shape `(batch_size, time_steps)`, *optional*) : Attention mask for the spectrogram input to the semantic encoder. `1` for valid frames, `0` for padding.

output_latents (`bool`, *optional*, defaults to `False`) : Whether to return the continuous latent representation from the quantizer.

**Returns:** `NeuCodecEncoderOutput` or `tuple(torch.FloatTensor)`

A `NeuCodecEncoderOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.

- **audio_codes** (`torch.LongTensor` of shape `(batch_size, 1, codes_length)`, *optional*) -- Discrete code embeddings computed using `model.encode`. These represent
  the compressed, quantized form of the input audio signal that can be
  used for storage, transmission, or generation.
- **latents** (`torch.Tensor` of shape `(batch_size, dimension, time_steps)`) -- Quantized continuous representation of input's embedding.
- **audio_codes_mask** (`torch.int32` of shape `(batch_size, 1, codes_length)`, *optional*) -- Downsampled `padding_mask` for indicating valid audio codes in `audio_codes`.

#### forward[[transformers.NeuCodecModel.forward]]

```python
forward(input_values: Tensor, input_features: Tensor, padding_mask: typing.Optional[torch.Tensor] = None, input_features_mask: typing.Optional[torch.Tensor] = None, output_latents: bool = False, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/neucodec/modeling_neucodec.py#L1079)

**Parameters:**

input_values (`torch.Tensor` of shape `(batch_size, 1, sequence_length)`) : Input audio waveform, sampled at `config.input_sampling_rate`.

input_features (`torch.Tensor` of shape `(batch_size, mel_bins, time_steps)`) : Input audio mel spectrogram for semantic encoding.

padding_mask (`torch.Tensor` of shape `(batch_size, 1, sequence_length)`) : Padding mask used to pad `input_values`.

input_features_mask (`torch.Tensor` of shape `(batch_size, time_steps)`, *optional*) : Attention mask for the spectrogram input to the semantic encoder. `1` for valid frames, `0` for padding.

output_latents (`bool`, *optional*, defaults to `False`) : Whether to return the continuous latent representation from the quantizer.

**Returns:** `NeuCodecOutput` or `tuple(torch.FloatTensor)`

A `NeuCodecOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.

The [NeuCodecModel](/docs/transformers/v5.17.0/en/model_doc/neucodec#transformers.NeuCodecModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **audio_values** (`torch.FloatTensor` of shape `(batch_size, 1, sequence_length)`, *optional*) -- Decoded audio waveform values in the time domain, obtained using the decoder
  part of NeuCodec. These represent the reconstructed audio signal.
- **audio_codes** (`torch.LongTensor` of shape `(batch_size, 1, codes_length)`, *optional*) -- Discrete code embeddings computed using `model.encode`. These are the quantized
  representations of the input audio used for further processing or generation.
- **latents** (`torch.Tensor` of shape `(batch_size, dimension, time_steps)`) -- Quantized continuous representation of input's embedding.
- **audio_codes_mask** (`torch.int32` of shape `(batch_size, 1, codes_length)`, *optional*) -- Downsampled `padding_mask` for indicating valid audio codes in `audio_codes`.

Examples:

```python
>>> from datasets import load_dataset
>>> from transformers import AutoFeatureExtractor, NeuCodecModel

>>> dataset = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> audio = dataset["train"]["audio"][0]["array"]

>>> model_id = "neuphonic/neucodec"
>>> model = NeuCodecModel.from_pretrained(model_id)
>>> feature_extractor = AutoFeatureExtractor.from_pretrained(model_id)

>>> inputs = feature_extractor(audio=audio, sampling_rate=feature_extractor.sampling_rate, return_tensors="pt")

>>> outputs = model(**inputs)
>>> audio_codes = outputs.audio_codes
>>> audio_values = outputs.audio_values  # sampled at 24kHz
```
