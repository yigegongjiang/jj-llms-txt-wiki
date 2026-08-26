# Higgs Audio V2 Tokenizer

## Overview

- Low Frame Rate: At 25 fps, our tokenizer halves the frame rate of many baselines when still maintaining high audio quality.
- Unified 24 kHz Training: We mix speech, music, and sound-event clips in one model, capturing both semantic and acoustic details, hugely facilitating the training of audio language models.
- Fast Inference: By avoiding diffusion steps, our encoder/decoder processes batches quickly, making it practical for real-time or large-scale tasks.

**Model Architecture:**

    

## Usage

```python
from datasets import Audio, load_dataset

from transformers import AutoFeatureExtractor, HiggsAudioV2TokenizerModel

# load model and feature extractor
model_id = "eustlb/higgs-audio-v2-tokenizer"
feature_extractor = AutoFeatureExtractor.from_pretrained(model_id)
model = HiggsAudioV2TokenizerModel.from_pretrained(model_id, device_map="auto")

# load audio sample
dummy_dataset = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
dummy_dataset = dummy_dataset.cast_column("audio", Audio(sampling_rate=feature_extractor.sampling_rate))
audio_sample = dummy_dataset[-1]["audio"]["array"]
inputs = feature_extractor(raw_audio=audio_sample, sampling_rate=feature_extractor.sampling_rate, return_tensors="pt").to(model.device)

# encode and decode
encoder_outputs = model.encode(inputs["input_values"])
decoder_outputs = model.decode(encoder_outputs.audio_codes)
audio_values = decoder_outputs.audio_values

# or the equivalent with a forward pass
audio_values = model(inputs["input_values"]).audio_values
```

## HiggsAudioV2TokenizerConfig[[transformers.HiggsAudioV2TokenizerConfig]]

#### transformers.HiggsAudioV2TokenizerConfig[[transformers.HiggsAudioV2TokenizerConfig]]

```python
transformers.HiggsAudioV2TokenizerConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, target_bandwidths: list[int | float] | tuple[int | float, ...] = (0.5, 1, 1.5, 2, 4), sample_rate: int = 24000, kernel_size: int = 3, channel_ratios: list[int] | tuple[int, ...] = (1, 1), strides: list[int] | tuple[int, ...] = (1, 1), block_dilations: list[int] | tuple[int, ...] = (1, 1), unit_kernel_size: int = 3, codebook_size: int = 1024, codebook_dim: int = 64, initializer_range: float = 0.02, acoustic_model_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, semantic_model_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, semantic_sample_rate: int = 16000, downsample_factor: int = 320)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/higgs_audio_v2_tokenizer/configuration_higgs_audio_v2_tokenizer.py#L34)

**Parameters:**

target_bandwidths (`List[float]`, *optional*, defaults to `[0.5, 1, 1.5, 2]`) : The range of different bandwidths (in kbps) the model can encode audio with.

sample_rate (`int`, *optional*, defaults to `24000`) : The sampling rate at which the audio files should be digitalized expressed in hertz (Hz).

kernel_size (`int`, *optional*, defaults to 3) : Kernel size for the initial semantic convolution.

channel_ratios (`List[float]`, *optional*, defaults to `[1, 1]`) : Expansion factors for the number of output channels in each semantic block.

strides (`List[int]`, *optional*, defaults to `[1, 1]`) : Strides for each semantic encoder block.

block_dilations (`List[int]`, *optional*, defaults to `[1, 1]`) : Dilation factors for the residual units in semantic blocks.

unit_kernel_size (`int`, *optional*, defaults to 3) : Kernel size inside each ResidualUnit in semantic blocks.

codebook_size (`int`, *optional*, defaults to `1024`) : The number of parallel codebooks used by the model.

codebook_dim (`int`, *optional*, defaults to `64`) : Dimensionality of each codebook embedding vector.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

acoustic_model_config (`Union[Dict, AutoConfig]`, *optional*) : An instance of the configuration for the acoustic (DAC) model.

semantic_model_config (`Union[Dict, AutoConfig]`, *optional*) : An instance of the configuration object for the semantic (HuBERT) model.

semantic_sample_rate (`int`, *optional*, defaults to 16000) : The sampling rate at which the semantic model expects audio input, in hertz (Hz).

downsample_factor (`int`, *optional*, defaults to 320) : Downsampling factor for the semantic features.

This is the configuration class to store the configuration of a HiggsAudioV2TokenizerModel. It is used to instantiate a Higgs Audio V2 Tokenizer
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [bosonai/higgs-audio-v2-tokenizer](https://huggingface.co/bosonai/higgs-audio-v2-tokenizer)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import HiggsAudioV2TokenizerModel, HiggsAudioV2TokenizerConfig

>>> # Initializing configuration
>>> configuration = HiggsAudioV2TokenizerConfig()

>>> # Initializing a model (with random weights) from the configuration
>>> model = HiggsAudioV2TokenizerModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## HiggsAudioV2TokenizerModel[[transformers.HiggsAudioV2TokenizerModel]]

#### transformers.HiggsAudioV2TokenizerModel[[transformers.HiggsAudioV2TokenizerModel]]

```python
transformers.HiggsAudioV2TokenizerModel(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/higgs_audio_v2_tokenizer/modeling_higgs_audio_v2_tokenizer.py#L456)

**Parameters:**

config ([HiggsAudioV2TokenizerModel](/docs/transformers/v5.15.1/en/model_doc/higgs_audio_v2_tokenizer#transformers.HiggsAudioV2TokenizerModel)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The HiggsAudioV2Tokenizer neural audio codec model.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### decode[[transformers.HiggsAudioV2TokenizerModel.decode]]

```python
decode(audio_codes: Tensor, return_dict: bool | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/higgs_audio_v2_tokenizer/modeling_higgs_audio_v2_tokenizer.py#L564)

**Parameters:**

audio_codes (`torch.LongTensor`  of shape `(batch_size, num_quantizers, codes_length)`) : Discrete code indices computed using `model.encode`.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput)

**Returns:**

Decoded audio values of shape `(batch_size, channels, num_samples)` obtained using the decoder part of
HiggsAudioV2Tokenizer.

#### encode[[transformers.HiggsAudioV2TokenizerModel.encode]]

```python
encode(input_values: Tensor, bandwidth: float | None = None, return_dict: bool | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/higgs_audio_v2_tokenizer/modeling_higgs_audio_v2_tokenizer.py#L512)

**Parameters:**

input_values (`torch.FloatTensor` of shape `(batch_size, channels, num_samples)`) : Float values of the input audio waveform.

bandwidth (`float`, *optional*) : The target bandwidth in (kbps) supports only values in `config.target_bandwidths`. Defaults to the highest available bandwidth `4.0` kbps.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput).

**Returns:**

`torch.LongTensor` of shape `(batch_size, num_quantizers, codes_length)` containing the discrete encoded audio codes.

#### forward[[transformers.HiggsAudioV2TokenizerModel.forward]]

```python
forward(input_values: Tensor, audio_codes: typing.Optional[torch.Tensor] = None, bandwidth: float | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/higgs_audio_v2_tokenizer/modeling_higgs_audio_v2_tokenizer.py#L592)

**Parameters:**

input_values (`torch.FloatTensor` of shape `(batch_size, channels, num_samples)`) : The raw float values of the input audio waveform.

audio_codes (`torch.LongTensor`  of shape `(batch_size, num_quantizers, codes_length)` : Discrete code indices computed using `model.encode`.

bandwidth (`float`, *optional*) : Target bandwidth in kbps. Must be one of `config.target_bandwidths`. Defaults to the highest available bandwidth.

bandwidth (`float`, *optional*) : Target bandwidth in kbps. Must be one of `config.target_bandwidths`. Defaults to the highest available bandwidth.

**Returns:** `HiggsAudioV2TokenizerOutput` or tuple `(audio_codes, audio_values)`

- `audio_codes` of shape `(batch_size, num_quantizers, codes_length)`: the quantized discrete codes.
- `audio_values` of shape `(batch_size, channels, num_samples)`: the reconstructed audio waveform given the codes.

The [HiggsAudioV2TokenizerModel](/docs/transformers/v5.15.1/en/model_doc/higgs_audio_v2_tokenizer#transformers.HiggsAudioV2TokenizerModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Example:

```python
>>> from datasets import load_dataset
>>> from transformers import AutoFeatureExtractor, HiggsAudioV2TokenizerModel

>>> model_id = "hf-audio/higgs_audio_v2_tokenizer-hubert-librispeech"
>>> model = HiggsAudioV2TokenizerModel.from_pretrained(model_id)
>>> feature_extractor = AutoFeatureExtractor.from_pretrained(model_id)

>>> dataset = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> dataset = dataset.cast_column("audio", Audio(sampling_rate=feature_extractor.sampling_rate))
>>> audio_sample = dataset[0]['audio']['array']

>>> inputs = feature_extractor(raw_audio=audio_sample, return_tensors="pt")

>>> outputs = model(**inputs)
>>> audio_codes = outputs.audio_codes
>>> audio_values = outputs.audio_values
```
