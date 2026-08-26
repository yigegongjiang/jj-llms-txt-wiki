# LASR

## Overview

LASR is the architecture behind MedASR, a speech-to-text model from Google Health AI pre-trained for medical dictation. It's based on the [Conformer architecture](https://huggingface.co/papers/2005.08100) and designed as a starting point for developers building dictation tools with medical terminology, like radiology dictation. MedASR performs well on medical audio but can struggle with terms outside its training data, such as non-standard medication names or temporal references (dates, times, or durations).

## Usage

### Basic usage

```python
from transformers import pipeline

pipe = pipeline("automatic-speech-recognition", model="google/medasr")
out = pipe("path/to/audio.mp3")
print(out)
```

```python
from datasets import Audio, load_dataset

from transformers import AutoModelForCTC, AutoProcessor

processor = AutoProcessor.from_pretrained("google/medasr")
model = AutoModelForCTC.from_pretrained("google/medasr", device_map="auto")

ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))
speech_samples = [el['array'] for el in ds["audio"][:5]]

inputs = processor(speech_samples, sampling_rate=processor.feature_extractor.sampling_rate)
inputs.to(model.device, dtype=model.dtype)
outputs = model.generate(**inputs)
print(processor.batch_decode(outputs))
```

### Training

The example below prepares a batch of audio and text, passes it through the LASR/MedASR model, and computes the training loss.

```python
from datasets import Audio, load_dataset

from transformers import AutoModelForCTC, AutoProcessor

# Load processor and model
processor = AutoProcessor.from_pretrained("google/medasr")
model = AutoModelForCTC.from_pretrained("google/medasr", device_map="auto")

# Load a small example dataset and prepare batch
ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))
speech_samples = [el["array"] for el in ds["audio"][:5]]
text_samples = [el for el in ds["text"][:5]]

# Passing `text` to the processor will prepare the `labels`
inputs = processor(audio=speech_samples, text=text_samples, sampling_rate=processor.feature_extractor.sampling_rate)
inputs.to(device, dtype=model.dtype)

outputs = model(**inputs)
outputs.loss.backward()
```

## LasrTokenizer[[transformers.LasrTokenizer]]

#### transformers.LasrTokenizer[[transformers.LasrTokenizer]]

```python
transformers.LasrTokenizer(eos_token = '</s>', unk_token = '<unk>', pad_token = '<pad>', _spm_precompiled_charsmap = None, extra_ids = 100, additional_special_tokens = None, vocab = None, vocab_file = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lasr/tokenization_lasr.py#L33)

**Parameters:**

vocab_file (`str`, *optional*) : [SentencePiece](https://github.com/google/sentencepiece) file (generally has a *.spm* extension) that contains the vocabulary necessary to instantiate a tokenizer.

eos_token (`str`, *optional*, defaults to `"</s>"`) : The end of sequence token.    When building a sequence using special tokens, this is not the token that is used for the end of sequence. The token used is the `sep_token`.   

unk_token (`str`, *optional*, defaults to `"<unk>"`) : The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this token instead.

pad_token (`str`, *optional*, defaults to `"<pad>"`) : The token used for padding, for example when batching sequences of different lengths.

extra_ids (`int`, *optional*, defaults to 100) : Add a number of extra ids added to the vocabulary for use as sentinels. These tokens are accessible as "" where "{%d}" is a number between 0 and extra_ids-1. These tokens can be retrieved by calling get_sentinel_tokens method and token ids can be by calling get_sentinel_token_ids method

additional_special_tokens (`list[str]`, *optional*) : Additional special tokens used by the tokenizer.

vocab (`str`, `dict` or `list`, *optional*) : Custom vocabulary dict. If not provided, a minimal vocabulary is created using the special tokens.

Construct a LASR tokenizer (backed by HuggingFace's *tokenizers* library). Based on
[Unigram](https://huggingface.co/docs/tokenizers/python/latest/components.html?highlight=unigram#models).

This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should
refer to this superclass for more information regarding those methods.

#### get_sentinel_token_ids[[transformers.LasrTokenizer.get_sentinel_token_ids]]

```python
get_sentinel_token_ids()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lasr/tokenization_lasr.py#L157)

Get the token IDs for sentinel tokens.

#### get_sentinel_tokens[[transformers.LasrTokenizer.get_sentinel_tokens]]

```python
get_sentinel_tokens()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lasr/tokenization_lasr.py#L151)

Get the list of sentinel tokens (extra_id tokens) from additional_special_tokens.

## LasrFeatureExtractor[[transformers.LasrFeatureExtractor]]

#### transformers.LasrFeatureExtractor[[transformers.LasrFeatureExtractor]]

```python
transformers.LasrFeatureExtractor(feature_size = 128, sampling_rate = 16000, hop_length = 160, n_fft = 512, win_length = 400, padding_value = 0.0, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lasr/feature_extraction_lasr.py#L70)

**Parameters:**

feature_size (`int`, *optional*, defaults to 128) : The feature dimension of the extracted features.

sampling_rate (`int`, *optional*, defaults to 16000) : The sampling rate at which the audio files should be digitalized expressed in hertz (Hz).

hop_length (`int`, *optional*, defaults to 160) : Length of the overlapping windows for the STFT used to obtain the Mel Frequency coefficients.

n_fft (`int`, *optional*, defaults to 512) : Size of the Fourier transform.

win_length (`int`, *optional*, defaults to 400) : The window length for the STFT computation.

padding_value (`float`, *optional*, defaults to 0.0) : Padding value used to pad the audio. Should correspond to silences.

Constructs a LASR feature extractor.

This feature extractor inherits from [SequenceFeatureExtractor](/docs/transformers/v5.15.1/en/main_classes/feature_extractor#transformers.SequenceFeatureExtractor) which contains
most of the main methods. Users should refer to this superclass for more information regarding those methods.

This class extracts mel-filter bank features from raw speech using a custom numpy implementation of the `Short Time
Fourier Transform` which should match pytorch's `torch.stft` equivalent.

#### __call__[[transformers.LasrFeatureExtractor.__call__]]

```python
__call__(raw_speech: numpy.ndarray | list[float] | list[numpy.ndarray] | list[list[float]], truncation: bool = False, pad_to_multiple_of: int | None = None, return_tensors: str | transformers.utils.generic.TensorType | None = None, return_attention_mask: bool | None = None, padding: str | None = 'longest', max_length: int | None = None, sampling_rate: int | None = None, do_normalize: bool | None = None, device: str | None = 'cpu', return_token_timestamps: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lasr/feature_extraction_lasr.py#L142)

**Parameters:**

raw_speech (`np.ndarray`, `list[float]`, `list[np.ndarray]`, `list[list[float]]`) : The sequence or batch of sequences to be padded. Each sequence can be a numpy array, a list of float values, a list of numpy arrays or a list of list of float values. Must be mono channel audio, not stereo, i.e. single float per timestep.

truncation (`bool`, *optional*, default to `True`) : Activates truncation to cut input sequences longer than *max_length* to *max_length*.

pad_to_multiple_of (`int`, *optional*, defaults to None) : If set will pad the sequence to a multiple of the provided value.  This is especially useful to enable the use of Tensor Cores on NVIDIA hardware with compute capability `>= 7.5` (Volta), or on TPUs which benefit from having sequence lengths be a multiple of 128.

return_attention_mask (`bool`, *optional*) : Whether to return the attention mask. If left to the default, will return the attention mask according to the specific feature_extractor's default.  [What are attention masks?](../glossary#attention-mask)    For Parakeet models, `attention_mask` should always be passed for batched inference, to avoid subtle bugs.   

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors instead of list of python integers. Acceptable values are:  - `'tf'`: Return TensorFlow `tf.constant` objects. - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return Numpy `np.ndarray` objects.

sampling_rate (`int`, *optional*) : The sampling rate at which the `raw_speech` input was sampled. It is strongly recommended to pass `sampling_rate` at the forward call to prevent silent errors and allow automatic speech recognition pipeline.

padding_value (`float`, *optional*, defaults to 0.0) : The value that is used to fill the padding values / vectors.

do_normalize (`bool`, *optional*, defaults to `False`) : Whether or not to zero-mean unit-variance normalize the input. Normalizing can help to significantly improve the performance of the model.

device (`str`, *optional*, defaults to `'cpu'`) : Specifies the device for computation of the log-mel spectrogram of audio signals in the `_torch_extract_fbank_features` method. (e.g., "cpu", "cuda")

return_token_timestamps (`bool`, *optional*, defaults to `None`) : Deprecated. Use `return_attention_mask` instead from which the number of frames can be inferred.  Whether or not to return the number of frames of the input raw_speech. These num_frames can be used by the model to compute word level timestamps.

Main method to featurize and prepare for the model one or several sequence(s). Implementation uses PyTorch for
the STFT computation if available, otherwise a slower NumPy based one.

## LasrProcessor[[transformers.LasrProcessor]]

#### transformers.LasrProcessor[[transformers.LasrProcessor]]

```python
transformers.LasrProcessor(feature_extractor, tokenizer)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lasr/processing_lasr.py#L48)

**Parameters:**

feature_extractor (`feature_extractor_class`) : The feature extractor is a required input.

tokenizer (`tokenizer_class`) : The tokenizer is a required input.

Constructs a LasrProcessor which wraps a feature extractor and a tokenizer into a single processor.

[LasrProcessor](/docs/transformers/v5.15.1/en/model_doc/lasr#transformers.LasrProcessor) offers all the functionalities of `feature_extractor_class` and `tokenizer_class`. See the
`~feature_extractor_class` and `~tokenizer_class` for more information.

#### __call__[[transformers.LasrProcessor.__call__]]

```python
__call__(audio: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor']], text: str | list[str] | list[list[str]] | None = None, sampling_rate: int | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lasr/processing_lasr.py#L54)

**Parameters:**

audio (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`) : The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor. In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels, and T is the sample length of the audio.

text (`Union[str, list[str], list[list[str]]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

sampling_rate (`int`, *optional*) : The sampling rate of the input audio in Hz. This should match the sampling rate expected by the feature extractor (defaults to 16000 Hz). If provided, it will be validated against the processor's expected sampling rate, and an error will be raised if they don't match. If not provided, a warning will be issued and the default sampling rate will be assumed.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) : Additional processing options for each modality (text, images, videos, audio). Model-specific parameters are listed above; see the TypedDict class for the complete list of supported arguments.

#### batch_decode[[transformers.LasrProcessor.batch_decode]]

```python
batch_decode(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/processing_utils.py#L1930)

This method forwards all its arguments to PreTrainedTokenizer's [batch_decode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.batch_decode). Please
refer to the docstring of this method for more information.

#### decode[[transformers.LasrProcessor.decode]]

```python
decode(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/processing_utils.py#L1939)

This method forwards all its arguments to PreTrainedTokenizer's [decode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.decode). Please refer to
the docstring of this method for more information.

## LasrEncoderConfig[[transformers.LasrEncoderConfig]]

#### transformers.LasrEncoderConfig[[transformers.LasrEncoderConfig]]

```python
transformers.LasrEncoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 512, num_hidden_layers: int = 17, num_attention_heads: int = 8, intermediate_size: int = 2048, hidden_act: str = 'silu', attention_bias: bool = False, convolution_bias: bool = False, conv_kernel_size: int = 32, subsampling_conv_channels: int = 256, num_mel_bins: int = 128, subsampling_conv_kernel_size: int = 5, subsampling_conv_stride: int = 2, dropout: float | int = 0.1, dropout_positions: float | int = 0.0, layerdrop: float | int = 0.1, activation_dropout: float | int = 0.1, attention_dropout: float | int = 0.1, max_position_embeddings: int = 10000, initializer_range: float = 0.02, layer_norm_eps: float = 1e-06, feed_forward_residual_weights: list[float] | tuple[float, ...] = (1.5, 0.5), conv_residual_weights: list[float] | tuple[float, ...] = (2.0, 1.0), batch_norm_momentum: float = 0.01, rope_parameters: dict | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lasr/configuration_lasr.py#L29)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `512`) : Dimension of the hidden representations.

num_hidden_layers (`int`, *optional*, defaults to `17`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer decoder.

intermediate_size (`int`, *optional*, defaults to `2048`) : Dimension of the MLP representations.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

attention_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

convolution_bias (`bool`, *optional*, defaults to `False`) : Whether to use bias in convolutions of the conformer's convolution module.

conv_kernel_size (`int`, *optional*, defaults to 32) : The kernel size of the convolution layers in the Conformer block.

subsampling_conv_channels (`int`, *optional*, defaults to 256) : The number of channels in the subsampling convolution layers.

num_mel_bins (`int`, *optional*, defaults to `128`) : Number of mel features used per input frame. Should correspond to the value used in the `AutoFeatureExtractor` class.

subsampling_conv_kernel_size (`int`, *optional*, defaults to 5) : The kernel size of the subsampling convolution layers.

subsampling_conv_stride (`int`, *optional*, defaults to 2) : The stride of the subsampling convolution layers.

dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The ratio for all dropout layers.

dropout_positions (`float`, *optional*, defaults to 0.0) : The dropout ratio for the positions in the input sequence.

layerdrop (`Union[float, int]`, *optional*, defaults to `0.1`) : The LayerDrop probability. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556) for more details.

activation_dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout ratio for activations inside the fully connected layer.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout ratio for the attention probabilities.

max_position_embeddings (`int`, *optional*, defaults to `10000`) : The maximum sequence length that this model might ever be used with.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

feed_forward_residual_weights (`tuple[float, float]`, *optional*, defaults to `[1.5, 0.5]`) : The residual weights for the feed forward layers.

conv_residual_weights (`tuple[float, float]`, *optional*, defaults to `[2.0, 1.0]`) : The residual weights for the convolution layers.

batch_norm_momentum (`float`, *optional*, defaults to 0.01) : The momentum for the batch normalization layers

rope_parameters (`dict`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

This is the configuration class to store the configuration of a LasrModel. It is used to instantiate a Lasr
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/medasr](https://huggingface.co/google/medasr)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import LasrEncoderModel, LasrEncoderConfig

>>> # Initializing a `LasrEncoder` configuration
>>> configuration = LasrEncoderConfig()

>>> # Initializing a model from the configuration
>>> model = LasrEncoderModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

This configuration class is based on the LasrEncoder architecture from Google Health AI. You can find more details
and pre-trained models at [google/medasr](https://huggingface.co/google/medasr).

## LasrCTCConfig[[transformers.LasrCTCConfig]]

#### transformers.LasrCTCConfig[[transformers.LasrCTCConfig]]

```python
transformers.LasrCTCConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 512, ctc_loss_reduction: str = 'mean', ctc_zero_infinity: bool = True, encoder_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, pad_token_id: int = 0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lasr/configuration_lasr.py#L103)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `512`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

ctc_loss_reduction (`str`, *optional*, defaults to `"mean"`) : Specifies the reduction to apply to the output of `torch.nn.CTCLoss`. Only relevant when training an instance of [LasrForCTC](/docs/transformers/v5.15.1/en/model_doc/lasr#transformers.LasrForCTC).

ctc_zero_infinity (`bool`, *optional*, defaults to `True`) : Whether to zero infinite losses and the associated gradients of `torch.nn.CTCLoss`. Infinite losses mainly occur when the inputs are too short to be aligned to the targets. Only relevant when training an instance of [LasrForCTC](/docs/transformers/v5.15.1/en/model_doc/lasr#transformers.LasrForCTC).

encoder_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the encoder backbone.

pad_token_id (`int`, *optional*, defaults to `0`) : Token id used for padding in the vocabulary.

This is the configuration class to store the configuration of a LasrModel. It is used to instantiate a Lasr
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/medasr](https://huggingface.co/google/medasr)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import LasrForCTC, LasrCTCConfig
>>> # Initializing a Lasr configuration
>>> configuration = LasrCTCConfig()
>>> # Initializing a model from the configuration
>>> model = LasrForCTC(configuration)
>>> # Accessing the model configuration
>>> configuration = model.config
```

This configuration class is based on the Lasr CTC architecture from Google Health AI. You can find more details
and pre-trained models at [google/medasr](https://huggingface.co/google/medasr).

## LasrEncoder[[transformers.LasrEncoder]]

#### transformers.LasrEncoder[[transformers.LasrEncoder]]

```python
transformers.LasrEncoder(config: LasrEncoderConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lasr/modeling_lasr.py#L478)

**Parameters:**

config ([LasrEncoderConfig](/docs/transformers/v5.15.1/en/model_doc/lasr#transformers.LasrEncoderConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The LasrEncoder model, based on the Conformer architecture](https://arxiv.org/abs/2005.08100).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.LasrEncoder.forward]]

```python
forward(input_features: Tensor, attention_mask: typing.Optional[torch.Tensor] = None, output_attention_mask: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lasr/modeling_lasr.py#L499)

**Parameters:**

input_features (`torch.Tensor` of shape `(batch_size, sequence_length, feature_dim)`) : The tensors corresponding to the input audio features. Audio features can be obtained using `feature_extractor_class`. See `feature_extractor_class.__call__` for details (`processor_class` uses `feature_extractor_class` for processing audios).

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

output_attention_mask (`bool`, *optional*) : Whether to return the output attention mask.

**Returns:** `LasrEncoderModelOutput` or `tuple(torch.FloatTensor)`

A `LasrEncoderModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.

The [LasrEncoder](/docs/transformers/v5.15.1/en/model_doc/lasr#transformers.LasrEncoder) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoProcessor, LasrEncoder
>>> from datasets import load_dataset, Audio

>>> model_id = "google/medasr"
>>> processor = AutoProcessor.from_pretrained(model_id)
>>> encoder = ParakeetEncoder.from_pretrained(model_id)

>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))

>>> inputs = processor(ds[0]["audio"]["array"])
>>> encoder_outputs = encoder(**inputs)

>>> print(encoder_outputs.last_hidden_state.shape)
```

## LasrForCTC[[transformers.LasrForCTC]]

#### transformers.LasrForCTC[[transformers.LasrForCTC]]

```python
transformers.LasrForCTC(config: LasrCTCConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lasr/modeling_lasr.py#L609)

**Parameters:**

config ([LasrCTCConfig](/docs/transformers/v5.15.1/en/model_doc/lasr#transformers.LasrCTCConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Lasr Encoder with a Connectionist Temporal Classification (CTC) head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.LasrForCTC.forward]]

```python
forward(input_features: Tensor, attention_mask: typing.Optional[torch.Tensor] = None, labels: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lasr/modeling_lasr.py#L620)

**Parameters:**

input_features (`torch.Tensor` of shape `(batch_size, sequence_length, feature_dim)`) : The tensors corresponding to the input audio features. Audio features can be obtained using `feature_extractor_class`. See `feature_extractor_class.__call__` for details (`processor_class` uses `feature_extractor_class` for processing audios).

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

labels (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

**Returns:** [CausalLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutput) or `tuple(torch.FloatTensor)`

A [CausalLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.

The [LasrForCTC](/docs/transformers/v5.15.1/en/model_doc/lasr#transformers.LasrForCTC) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoProcessor, LasrForCTC
>>> from datasets import load_dataset, Audio

>>> model_id = "nvidia/lasr-ctc-1.1b"
>>> processor = AutoProcessor.from_pretrained(model_id)
>>> model = LasrForCTC.from_pretrained(model_id)

>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))

>>> inputs = processor(ds[0]["audio"]["array"], text=ds[0]["text"])
>>> outputs = model(**inputs)

>>> print(outputs.loss)
```

#### generate[[transformers.LasrForCTC.generate]]

```python
generate(input_features: Tensor, attention_mask: typing.Optional[torch.Tensor] = None, return_dict_in_generate: bool = False, compile_config: transformers.generation.configuration_utils.CompileConfig | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/lasr/modeling_lasr.py#L690)

Example:

```python
>>> from transformers import AutoProcessor, LasrForCTC
>>> from datasets import load_dataset, Audio

>>> model_id = "google/medasr"
>>> processor = AutoProcessor.from_pretrained(model_id)
>>> model = LasrForCTC.from_pretrained(model_id)

>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))

>>> inputs = processor(ds[0]["audio"]["array"], text=ds[0]["text"])
>>> predicted_ids = model.generate(**inputs)
>>> transcription = processor.batch_decode(predicted_ids, skip_special_tokens=True)

>>> print(transcription)
```
