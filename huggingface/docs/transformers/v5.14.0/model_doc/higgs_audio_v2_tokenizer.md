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

- **target_bandwidths** (`List[float]`, *optional*, defaults to `[0.5, 1, 1.5, 2]`) --
  The range of different bandwidths (in kbps) the model can encode audio with.
- **sample_rate** (`int`, *optional*, defaults to `24000`) --
  The sampling rate at which the audio files should be digitalized expressed in hertz (Hz).
- **kernel_size** (`int`, *optional*, defaults to 3) --
  Kernel size for the initial semantic convolution.
- **channel_ratios** (`List[float]`, *optional*, defaults to `[1, 1]`) --
  Expansion factors for the number of output channels in each semantic block.
- **strides** (`List[int]`, *optional*, defaults to `[1, 1]`) --
  Strides for each semantic encoder block.
- **block_dilations** (`List[int]`, *optional*, defaults to `[1, 1]`) --
  Dilation factors for the residual units in semantic blocks.
- **unit_kernel_size** (`int`, *optional*, defaults to 3) --
  Kernel size inside each ResidualUnit in semantic blocks.
- **codebook_size** (`int`, *optional*, defaults to `1024`) --
  The number of parallel codebooks used by the model.
- **codebook_dim** (`int`, *optional*, defaults to `64`) --
  Dimensionality of each codebook embedding vector.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **acoustic_model_config** (`Union[Dict, AutoConfig]`, *optional*) --
  An instance of the configuration for the acoustic (DAC) model.
- **semantic_model_config** (`Union[Dict, AutoConfig]`, *optional*) --
  An instance of the configuration object for the semantic (HuBERT) model.
- **semantic_sample_rate** (`int`, *optional*, defaults to 16000) --
  The sampling rate at which the semantic model expects audio input, in hertz (Hz).
- **downsample_factor** (`int`, *optional*, defaults to 320) --
  Downsampling factor for the semantic features.

This is the configuration class to store the configuration of a HiggsAudioV2TokenizerModel. It is used to instantiate a Higgs Audio V2 Tokenizer
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [bosonai/higgs-audio-v2-tokenizer](https://huggingface.co/bosonai/higgs-audio-v2-tokenizer)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

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

- **config** ([HiggsAudioV2TokenizerModel](/docs/transformers/v5.14.0/en/model_doc/higgs_audio_v2_tokenizer#transformers.HiggsAudioV2TokenizerModel)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
The HiggsAudioV2Tokenizer neural audio codec model.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "return_dict", "val": ": bool | None = None"}]}>
- **audio_codes** (`torch.LongTensor`  of shape `(batch_size, num_quantizers, codes_length)`) --
  Discrete code indices computed using `model.encode`.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput)Decoded audio values of shape `(batch_size, channels, num_samples)` obtained using the decoder part of
HiggsAudioV2Tokenizer.

)>"}, {"name": "bandwidth", "val": ": float | None = None"}, {"name": "return_dict", "val": ": bool | None = None"}]}>
- **input_values** (`torch.FloatTensor` of shape `(batch_size, channels, num_samples)`) --
  Float values of the input audio waveform.
- **bandwidth** (`float`, *optional*) --
  The target bandwidth in (kbps) supports only values in `config.target_bandwidths`.
  Defaults to the highest available bandwidth `4.0` kbps.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput).`torch.LongTensor` of shape `(batch_size, num_quantizers, codes_length)` containing the discrete encoded audio codes.

)>"}, {"name": "audio_codes", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "bandwidth", "val": ": float | None = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **input_values** (`torch.FloatTensor` of shape `(batch_size, channels, num_samples)`) --
  The raw float values of the input audio waveform.
- **audio_codes** (`torch.LongTensor`  of shape `(batch_size, num_quantizers, codes_length)` --
  Discrete code indices computed using `model.encode`.
- **bandwidth** (`float`, *optional*) --
  Target bandwidth in kbps. Must be one of `config.target_bandwidths`. Defaults to the highest available bandwidth.
- **bandwidth** (`float`, *optional*) --
  Target bandwidth in kbps. Must be one of `config.target_bandwidths`. Defaults to the highest available bandwidth.`HiggsAudioV2TokenizerOutput` or tuple `(audio_codes, audio_values)`- `audio_codes` of shape `(batch_size, num_quantizers, codes_length)`: the quantized discrete codes.
- `audio_values` of shape `(batch_size, channels, num_samples)`: the reconstructed audio waveform given the codes.
The [HiggsAudioV2TokenizerModel](/docs/transformers/v5.14.0/en/model_doc/higgs_audio_v2_tokenizer#transformers.HiggsAudioV2TokenizerModel) forward method, overrides the `__call__` special method.

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
