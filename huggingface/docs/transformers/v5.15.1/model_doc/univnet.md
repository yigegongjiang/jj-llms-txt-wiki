# UnivNet

## Overview

The UnivNet model was proposed in [UnivNet: A Neural Vocoder with Multi-Resolution Spectrogram Discriminators for High-Fidelity Waveform Generation](https://huggingface.co/papers/2106.07889) by Won Jang, Dan Lim, Jaesam Yoon, Bongwan Kin, and Juntae Kim.
The UnivNet model is a generative adversarial network (GAN) trained to synthesize high fidelity speech waveforms. The UnivNet model shared in `transformers` is the *generator*, which maps a conditioning log-mel spectrogram and optional noise sequence to a speech waveform (e.g. a vocoder). Only the generator is required for inference. The *discriminator* used to train the `generator` is not implemented.

The abstract from the paper is the following:

*Most neural vocoders employ band-limited mel-spectrograms to generate waveforms. If full-band spectral features are used as the input, the vocoder can be provided with as much acoustic information as possible. However, in some models employing full-band mel-spectrograms, an over-smoothing problem occurs as part of which non-sharp spectrograms are generated. To address this problem, we propose UnivNet, a neural vocoder that synthesizes high-fidelity waveforms in real time. Inspired by works in the field of voice activity detection, we added a multi-resolution spectrogram discriminator that employs multiple linear spectrogram magnitudes computed using various parameter sets. Using full-band mel-spectrograms as input, we expect to generate high-resolution signals by adding a discriminator that employs spectrograms of multiple resolutions as the input. In an evaluation on a dataset containing information on hundreds of speakers, UnivNet obtained the best objective and subjective results among competing models for both seen and unseen speakers. These results, including the best subjective score for text-to-speech, demonstrate the potential for fast adaptation to new speakers without a need for training from scratch.*

Tips:

- The `noise_sequence` argument for [UnivNetModel.forward()](/docs/transformers/v5.15.1/en/model_doc/univnet#transformers.UnivNetModel.forward) should be standard Gaussian noise (such as from `torch.randn`) of shape `([batch_size], noise_length, model.config.model_in_channels)`, where `noise_length` should match the length dimension (dimension 1) of the `input_features` argument. If not supplied, it will be randomly generated; a `torch.Generator` can be supplied to the `generator` argument so that the forward pass can be reproduced. (Note that [UnivNetFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/univnet#transformers.UnivNetFeatureExtractor) will return generated noise by default, so it shouldn't be necessary to generate `noise_sequence` manually.)
- Padding added by [UnivNetFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/univnet#transformers.UnivNetFeatureExtractor) can be removed from the [UnivNetModel](/docs/transformers/v5.15.1/en/model_doc/univnet#transformers.UnivNetModel) output through the `UnivNetFeatureExtractor.batch_decode()` method, as shown in the usage example below.
- Padding the end of each waveform with silence can reduce artifacts at the end of the generated audio sample. This can be done by supplying `pad_end = True` to [UnivNetFeatureExtractor.__call__()](/docs/transformers/v5.15.1/en/model_doc/univnet#transformers.UnivNetFeatureExtractor.__call__). See [this issue](https://github.com/seungwonpark/melgan/issues/8) for more details.

Usage Example:

```python
import torch
from datasets import Audio, load_dataset
from scipy.io.wavfile import write

from transformers import UnivNetFeatureExtractor, UnivNetModel

model_id_or_path = "dg845/univnet-dev"
model = UnivNetModel.from_pretrained(model_id_or_path, device_map="auto")
feature_extractor = UnivNetFeatureExtractor.from_pretrained(model_id_or_path)

ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
# Resample the audio to the model and feature extractor's sampling rate.
ds = ds.cast_column("audio", Audio(sampling_rate=feature_extractor.sampling_rate))
# Pad the end of the converted waveforms to reduce artifacts at the end of the output audio samples.
inputs = feature_extractor(
    ds[0]["audio"]["array"], sampling_rate=ds[0]["audio"]["sampling_rate"], pad_end=True, return_tensors="pt"
)

with torch.no_grad():
    audio = model(**inputs)

# Remove the extra padding at the end of the output.
audio = feature_extractor.batch_decode(**audio)[0]
# Convert to wav file
write("sample_audio.wav", feature_extractor.sampling_rate, audio)
```

This model was contributed by [dg845](https://huggingface.co/dg845).
To the best of my knowledge, there is no official code release, but an unofficial implementation can be found at [maum-ai/univnet](https://github.com/maum-ai/univnet) with pretrained checkpoints [here](https://github.com/maum-ai/univnet#pre-trained-model).

## UnivNetConfig[[transformers.UnivNetConfig]]

#### transformers.UnivNetConfig[[transformers.UnivNetConfig]]

```python
transformers.UnivNetConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, model_in_channels: int = 64, model_hidden_channels: int = 32, num_mel_bins: int = 100, resblock_kernel_sizes: list[int] | tuple[int, ...] = (3, 3, 3), resblock_stride_sizes: list[int] | tuple[int, ...] = (8, 8, 4), resblock_dilation_sizes: list | tuple = ((1, 3, 9, 27), (1, 3, 9, 27), (1, 3, 9, 27)), kernel_predictor_num_blocks: int = 3, kernel_predictor_hidden_channels: int = 64, kernel_predictor_conv_size: int = 3, kernel_predictor_dropout: float | int = 0.0, initializer_range: float = 0.01, leaky_relu_slope: float = 0.2)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/univnet/configuration_univnet.py#L24)

**Parameters:**

model_in_channels (`int`, *optional*, defaults to 64) : The number of input channels for the UnivNet residual network. This should correspond to `noise_sequence.shape[1]` and the value used in the [UnivNetFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/univnet#transformers.UnivNetFeatureExtractor) class.

model_hidden_channels (`int`, *optional*, defaults to 32) : The number of hidden channels of each residual block in the UnivNet residual network.

num_mel_bins (`int`, *optional*, defaults to 100) : The number of frequency bins in the conditioning log-mel spectrogram. This should correspond to the value used in the [UnivNetFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/univnet#transformers.UnivNetFeatureExtractor) class.

resblock_kernel_sizes (`tuple[int]` or `list[int]`, *optional*, defaults to `[3, 3, 3]`) : A tuple of integers defining the kernel sizes of the 1D convolutional layers in the UnivNet residual network. The length of `resblock_kernel_sizes` defines the number of resnet blocks and should match that of `resblock_stride_sizes` and `resblock_dilation_sizes`.

resblock_stride_sizes (`tuple[int]` or `list[int]`, *optional*, defaults to `[8, 8, 4]`) : A tuple of integers defining the stride sizes of the 1D convolutional layers in the UnivNet residual network. The length of `resblock_stride_sizes` should match that of `resblock_kernel_sizes` and `resblock_dilation_sizes`.

resblock_dilation_sizes (`tuple[tuple[int]]` or `list[list[int]]`, *optional*, defaults to `[[1, 3, 9, 27], [1, 3, 9, 27], [1, 3, 9, 27]]`) : A nested tuple of integers defining the dilation rates of the dilated 1D convolutional layers in the UnivNet residual network. The length of `resblock_dilation_sizes` should match that of `resblock_kernel_sizes` and `resblock_stride_sizes`. The length of each nested list in `resblock_dilation_sizes` defines the number of convolutional layers per resnet block.

kernel_predictor_num_blocks (`int`, *optional*, defaults to 3) : The number of residual blocks in the kernel predictor network, which calculates the kernel and bias for each location variable convolution layer in the UnivNet residual network.

kernel_predictor_hidden_channels (`int`, *optional*, defaults to 64) : The number of hidden channels for each residual block in the kernel predictor network.

kernel_predictor_conv_size (`int`, *optional*, defaults to 3) : The kernel size of each 1D convolutional layer in the kernel predictor network.

kernel_predictor_dropout (`float`, *optional*, defaults to 0.0) : The dropout probability for each residual block in the kernel predictor network.

initializer_range (`float`, *optional*, defaults to `0.01`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

leaky_relu_slope (`float`, *optional*, defaults to 0.2) : The angle of the negative slope used by the leaky ReLU activation.

This is the configuration class to store the configuration of a UnivNetModel. It is used to instantiate a Univnet
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [dg845/univnet-dev](https://huggingface.co/dg845/univnet-dev)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import UnivNetModel, UnivNetConfig

>>> # Initializing a Tortoise TTS style configuration
>>> configuration = UnivNetConfig()

>>> # Initializing a model (with random weights) from the Tortoise TTS style configuration
>>> model = UnivNetModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## UnivNetFeatureExtractor[[transformers.UnivNetFeatureExtractor]]

#### transformers.UnivNetFeatureExtractor[[transformers.UnivNetFeatureExtractor]]

```python
transformers.UnivNetFeatureExtractor(feature_size: int = 1, sampling_rate: int = 24000, padding_value: float = 0.0, do_normalize: bool = False, num_mel_bins: int = 100, hop_length: int = 256, win_length: int = 1024, win_function: str = 'hann_window', filter_length: int | None = 1024, max_length_s: int = 10, fmin: float = 0.0, fmax: float | None = None, mel_floor: float = 1e-09, center: bool = False, compression_factor: float = 1.0, compression_clip_val: float = 1e-05, normalize_min: float = -11.512925148010254, normalize_max: float = 2.3143386840820312, model_in_channels: int = 64, pad_end_length: int = 10, return_attention_mask = True, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/univnet/feature_extraction_univnet.py#L29)

**Parameters:**

feature_size (`int`, *optional*, defaults to 1) : The feature dimension of the extracted features.

sampling_rate (`int`, *optional*, defaults to 24000) : The sampling rate at which the audio files should be digitalized expressed in hertz (Hz).

padding_value (`float`, *optional*, defaults to 0.0) : The value to pad with when applying the padding strategy defined by the `padding` argument to [UnivNetFeatureExtractor.__call__()](/docs/transformers/v5.15.1/en/model_doc/univnet#transformers.UnivNetFeatureExtractor.__call__). Should correspond to audio silence. The `pad_end` argument to `__call__` will also use this padding value.

do_normalize (`bool`, *optional*, defaults to `False`) : Whether to perform Tacotron 2 normalization on the input. Normalizing can help to significantly improve the performance for some models.

num_mel_bins (`int`, *optional*, defaults to 100) : The number of mel-frequency bins in the extracted spectrogram features. This should match `UnivNetModel.config.num_mel_bins`.

hop_length (`int`, *optional*, defaults to 256) : The direct number of samples between sliding windows. Otherwise referred to as "shift" in many papers. Note that this is different from other audio feature extractors such as [SpeechT5FeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5FeatureExtractor) which take the `hop_length` in ms.

win_length (`int`, *optional*, defaults to 1024) : The direct number of samples for each sliding window. Note that this is different from other audio feature extractors such as [SpeechT5FeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5FeatureExtractor) which take the `win_length` in ms.

win_function (`str`, *optional*, defaults to `"hann_window"`) : Name for the window function used for windowing, must be accessible via `torch.{win_function}`

filter_length (`int`, *optional*, defaults to 1024) : The number of FFT components to use. If `None`, this is determined using `transformers.audio_utils.optimal_fft_length`.

max_length_s (`int`, *optional*, defaults to 10) : The maximum input length of the model in seconds. This is used to pad the audio.

fmin (`float`, *optional*, defaults to 0.0) : Minimum mel frequency in Hz.

fmax (`float`, *optional*) : Maximum mel frequency in Hz. If not set, defaults to `sampling_rate / 2`.

mel_floor (`float`, *optional*, defaults to 1e-09) : Minimum value of mel frequency banks. Note that the way [UnivNetFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/univnet#transformers.UnivNetFeatureExtractor) uses `mel_floor` is different than in [transformers.audio_utils.spectrogram()](/docs/transformers/v5.15.1/en/internal/audio_utils#transformers.audio_utils.spectrogram).

center (`bool`, *optional*, defaults to `False`) : Whether to pad the waveform so that frame `t` is centered around time `t * hop_length`. If `False`, frame `t` will start at time `t * hop_length`.

compression_factor (`float`, *optional*, defaults to 1.0) : The multiplicative compression factor for dynamic range compression during spectral normalization.

compression_clip_val (`float`, *optional*, defaults to 1e-05) : The clip value applied to the waveform before applying dynamic range compression during spectral normalization.

normalize_min (`float`, *optional*, defaults to -11.512925148010254) : The min value used for Tacotron 2-style linear normalization. The default is the original value from the Tacotron 2 implementation.

normalize_max (`float`, *optional*, defaults to 2.3143386840820312) : The max value used for Tacotron 2-style linear normalization. The default is the original value from the Tacotron 2 implementation.

model_in_channels (`int`, *optional*, defaults to 64) : The number of input channels to the [UnivNetModel](/docs/transformers/v5.15.1/en/model_doc/univnet#transformers.UnivNetModel) model. This should match `UnivNetModel.config.model_in_channels`.

pad_end_length (`int`, *optional*, defaults to 10) : If padding the end of each waveform, the number of spectrogram frames worth of samples to append. The number of appended samples will be `pad_end_length * hop_length`.

return_attention_mask (`bool`, *optional*, defaults to `True`) : Whether or not [__call__()](/docs/transformers/v5.15.1/en/model_doc/univnet#transformers.UnivNetFeatureExtractor.__call__) should return `attention_mask`.

Constructs a UnivNet feature extractor.

This class extracts log-mel-filter bank features from raw speech using the short time Fourier Transform (STFT). The
STFT implementation follows that of TacoTron 2 and Hifi-GAN.

This feature extractor inherits from [SequenceFeatureExtractor](/docs/transformers/v5.15.1/en/main_classes/feature_extractor#transformers.SequenceFeatureExtractor) which contains
most of the main methods. Users should refer to this superclass for more information regarding those methods.

#### __call__[[transformers.UnivNetFeatureExtractor.__call__]]

```python
__call__(raw_speech: numpy.ndarray | list[float] | list[numpy.ndarray] | list[list[float]], sampling_rate: int | None = None, padding: bool | str | transformers.utils.generic.PaddingStrategy = True, max_length: int | None = None, truncation: bool = True, pad_to_multiple_of: int | None = None, return_noise: bool = True, generator: numpy.random._generator.Generator | None = None, pad_end: bool = False, pad_length: int | None = None, do_normalize: str | None = None, return_attention_mask: bool | None = None, return_tensors: str | transformers.utils.generic.TensorType | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/univnet/feature_extraction_univnet.py#L286)

**Parameters:**

raw_speech (`np.ndarray`, `list[float]`, `list[np.ndarray]`, `list[list[float]]`) : The sequence or batch of sequences to be padded. Each sequence can be a numpy array, a list of float values, a list of numpy arrays or a list of list of float values. Must be mono channel audio, not stereo, i.e. single float per timestep.

sampling_rate (`int`, *optional*) : The sampling rate at which the `raw_speech` input was sampled. It is strongly recommended to pass `sampling_rate` at the forward call to prevent silent errors and allow automatic speech recognition pipeline.

padding (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `True`) : Select a strategy to pad the input `raw_speech` waveforms (according to the model's padding side and padding index) among:  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single sequence if provided). - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different lengths).  If `pad_end = True`, that padding will occur before the `padding` strategy is applied.

max_length (`int`, *optional*) : Maximum length of the returned list and optionally padding length (see above).

truncation (`bool`, *optional*, defaults to `True`) : Activates truncation to cut input sequences longer than `max_length` to `max_length`.

pad_to_multiple_of (`int`, *optional*) : If set will pad the sequence to a multiple of the provided value.  This is especially useful to enable the use of Tensor Cores on NVIDIA hardware with compute capability `>= 7.5` (Volta), or on TPUs which benefit from having sequence lengths be a multiple of 128.

return_noise (`bool`, *optional*, defaults to `True`) : Whether to generate and return a noise waveform for use in [UnivNetModel.forward()](/docs/transformers/v5.15.1/en/model_doc/univnet#transformers.UnivNetModel.forward).

generator (`numpy.random.Generator`, *optional*, defaults to `None`) : An optional `numpy.random.Generator` random number generator to use when generating noise.

pad_end (`bool`, *optional*, defaults to `False`) : Whether to pad the end of each waveform with silence. This can help reduce artifacts at the end of the generated audio sample; see https://github.com/seungwonpark/melgan/issues/8 for more details. This padding will be done before the padding strategy specified in `padding` is performed.

pad_length (`int`, *optional*, defaults to `None`) : If padding the end of each waveform, the length of the padding in spectrogram frames. If not set, this will default to `self.config.pad_end_length`.

do_normalize (`bool`, *optional*) : Whether to perform Tacotron 2 normalization on the input. Normalizing can help to significantly improve the performance for some models. If not set, this will default to `self.config.do_normalize`.

return_attention_mask (`bool`, *optional*) : Whether to return the attention mask. If left to the default, will return the attention mask according to the specific feature_extractor's default.  [What are attention masks?](../glossary#attention-mask) 

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors instead of list of python integers. Acceptable values are:  - `'pt'`: Return PyTorch `torch.np.array` objects. - `'np'`: Return Numpy `np.ndarray` objects.

Main method to featurize and prepare for the model one or several sequence(s).

## UnivNetModel[[transformers.UnivNetModel]]

#### transformers.UnivNetModel[[transformers.UnivNetModel]]

```python
transformers.UnivNetModel(config: UnivNetConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/univnet/modeling_univnet.py#L425)

**Parameters:**

config ([UnivNetConfig](/docs/transformers/v5.15.1/en/model_doc/univnet#transformers.UnivNetConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Univnet Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.UnivNetModel.forward]]

```python
forward(input_features: FloatTensor, noise_sequence: typing.Optional[torch.FloatTensor] = None, padding_mask: typing.Optional[torch.FloatTensor] = None, generator: typing.Optional[torch.Generator] = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/univnet/modeling_univnet.py#L469)

**Parameters:**

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`) : The tensors corresponding to the input audio features. Audio features can be obtained using [UnivNetFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/univnet#transformers.UnivNetFeatureExtractor). See [UnivNetFeatureExtractor.__call__()](/docs/transformers/v5.15.1/en/model_doc/univnet#transformers.UnivNetFeatureExtractor.__call__) for details (`processor_class` uses [UnivNetFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/univnet#transformers.UnivNetFeatureExtractor) for processing audios).

noise_sequence (`torch.FloatTensor`, *optional*) : Tensor containing a noise sequence of standard Gaussian noise. Can be batched and of shape `(batch_size, sequence_length, config.model_in_channels)`, or un-batched and of shape (sequence_length, config.model_in_channels)`. If not supplied, will be randomly generated.

padding_mask (`torch.BoolTensor`, *optional*) : Mask indicating which parts of each sequence are padded. Mask values are selected in `[0, 1]`:  - 1 for tokens that are **not masked** - 0 for tokens that are **masked**  The mask can be batched and of shape `(batch_size, sequence_length)` or un-batched and of shape `(sequence_length,)`.

generator (`torch.Generator`, *optional*) : A [torch generator](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic. return_dict: Whether to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) subclass instead of a plain tuple.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `UnivNetModelOutput` or `tuple(torch.FloatTensor)`

A `UnivNetModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([UnivNetConfig](/docs/transformers/v5.15.1/en/model_doc/univnet#transformers.UnivNetConfig)) and inputs.

The [UnivNetModel](/docs/transformers/v5.15.1/en/model_doc/univnet#transformers.UnivNetModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **waveforms** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) -- Batched 1D (mono-channel) output audio waveforms.
- **waveform_lengths** (`torch.FloatTensor` of shape `(batch_size,)`) -- The batched length in samples of each unpadded waveform in `waveforms`.

Example:

```python
>>> from transformers import UnivNetFeatureExtractor, UnivNetModel
>>> from datasets import load_dataset, Audio

>>> model = UnivNetModel.from_pretrained("dg845/univnet-dev")
>>> feature_extractor = UnivNetFeatureExtractor.from_pretrained("dg845/univnet-dev")

>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> # Resample the audio to the feature extractor's sampling rate.
>>> ds = ds.cast_column("audio", Audio(sampling_rate=feature_extractor.sampling_rate))
>>> inputs = feature_extractor(
...     ds[0]["audio"]["array"], sampling_rate=ds[0]["audio"]["sampling_rate"], return_tensors="pt"
... )
>>> audio = model(**inputs).waveforms
>>> list(audio.shape)
[1, 140288]
```
