# VibeVoice Acoustic Tokenizer

## Overview

[VibeVoice](https://huggingface.co/papers/2508.19205) is a novel framework for synthesizing high-fidelity, long-form speech with multiple speakers by employing a next-token diffusion approach within a Large Language Model (LLM) structure. It's designed to capture the authentic conversational "vibe" and is particularly suited for generating audio content like podcasts and multi-participant audiobooks.

One key feature of VibeVoice is the use of two continuous audio tokenizers, one for extracting acoustic features and another for semantic features.

A model checkpoint is available at [microsoft/VibeVoice-AcousticTokenizer](https://huggingface.co/microsoft/VibeVoice-AcousticTokenizer)

This model was contributed by [Eric Bezzam](https://huggingface.co/bezzam).

## Architecture

The architecture is a mirror-symmetric encoder-decoder structure. The encoder employs a hierarchical design with 7 stages of ConvNeXt-like blocks, which use 1D depth-wise causal convolutionsfor efficient streaming processing. Six downsampling layers achieve a cumulative 3200X downsampling rate from a 24kHz input, yielding 7.5 tokens/frames per second. Each encoder/decoder component has approximately 340M parameters, for a total of around 680M parameters The training objective follows that of [DAC](./dac), including its discriminator and loss designs.

Acoustic Tokenizer adopts the principles of a Variational Autoencoder (VAE). The encoder maps the input audio to the parameters of a latent distribution, namely the mean. Along with a fixed standard deviation, a latent vector is then sampled using the reparameterization trick. Please refer to the [technical report](https://huggingface.co/papers/2508.19205) for further details.

## Usage

Below is example usage to encode and decode audio:

```python
import torch
from scipy.io import wavfile

from transformers import AutoFeatureExtractor, VibeVoiceAcousticTokenizerModel
from transformers.audio_utils import load_audio_librosa

model_id = "microsoft/VibeVoice-AcousticTokenizer"

# load model
feature_extractor = AutoFeatureExtractor.from_pretrained(model_id)
model = VibeVoiceAcousticTokenizerModel.from_pretrained(model_id, device_map="auto")
print("Model loaded on device:", model.device)
print("Model dtype:", model.dtype)

# load audio
audio = load_audio_librosa(
    "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/voices/en-Alice_woman.wav",
    sampling_rate=feature_extractor.sampling_rate,
)

# preprocess audio
inputs = feature_extractor(
    audio,
    sampling_rate=feature_extractor.sampling_rate,
    pad_to_multiple_of=3200,
).to(model.device, model.dtype)
print("Input audio shape:", inputs.input_values.shape)
# Input audio shape: torch.Size([1, 1, 224000])

with torch.no_grad():
    # set VAE sampling to False for deterministic output
    encoded_outputs = model.encode(inputs.input_values, sample=False)
    print("Latent shape:", encoded_outputs.latents.shape)
    # Latent shape: torch.Size([1, 70, 64])

    decoded_outputs = model.decode(**encoded_outputs)
    print("Reconstructed audio shape:", decoded_outputs.audio.shape)
    # Reconstructed audio shape: torch.Size([1, 1, 224000])

# Save audio
output_fp = "vibevoice_acoustic_tokenizer_reconstructed.wav"
wavfile.write(output_fp, feature_extractor.sampling_rate, decoded_outputs.audio.squeeze().float().cpu().numpy())
print(f"Reconstructed audio saved to : {output_fp}")
```

## Streaming

For streaming ASR or TTS, where cached states need to be tracked, the `use_cache` parameter can be used when encoding or decoding audio:

```python
import torch
from scipy.io import wavfile

from transformers import AutoFeatureExtractor, VibeVoiceAcousticTokenizerModel
from transformers.audio_utils import load_audio_librosa

model_id = "microsoft/VibeVoice-AcousticTokenizer"

# load model
feature_extractor = AutoFeatureExtractor.from_pretrained(model_id)
model = VibeVoiceAcousticTokenizerModel.from_pretrained(model_id, device_map="auto")
print("Model loaded on device:", model.device)
print("Model dtype:", model.dtype)

# load audio
audio = load_audio_librosa(
    "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/voices/en-Alice_woman.wav",
    sampling_rate=feature_extractor.sampling_rate,
)

# preprocess audio
inputs = feature_extractor(
    audio,
    sampling_rate=feature_extractor.sampling_rate,
    pad_to_multiple_of=3200,
).to(model.device, model.dtype)
print("Input audio shape:", inputs.input_values.shape)
# Input audio shape: torch.Size([1, 1, 224000])

# chache will be initialized after a first pass
encoder_cache = None
decoder_cache = None
with torch.no_grad():
    # set VAE sampling to False for deterministic output
    encoded_outputs = model.encode(inputs.input_values, sample=False, padding_cache=encoder_cache, use_cache=True)
    print("Latent shape:", encoded_outputs.latents.shape)
    # Latent shape: torch.Size([1, 70, 64])

    decoded_outputs = model.decode(encoded_outputs.latents, padding_cache=decoder_cache, use_cache=True)
    print("Reconstructed audio shape:", decoded_outputs.audio.shape)
    # Reconstructed audio shape: torch.Size([1, 1, 224000])

    # `padding_cache` can be extracted from the outputs for subsequent passes
    encoder_cache = encoded_outputs.padding_cache
    print("Number of cached encoder layers:", len(encoder_cache.per_layer_in_channels))
    # Number of cached encoder layers: 34
    decoder_cache = decoded_outputs.padding_cache
    print("Number of cached decoder layers:", len(decoder_cache.per_layer_in_channels))
    # Number of cached decoder layers: 34

# Save audio
output_fp = "vibevoice_acoustic_tokenizer_reconstructed.wav"
wavfile.write(output_fp, feature_extractor.sampling_rate, decoded_outputs.audio.squeeze().float().cpu().numpy())
print(f"Reconstructed audio saved to : {output_fp}")
```

## VibeVoiceAcousticTokenizerConfig[[transformers.VibeVoiceAcousticTokenizerConfig]]

- **channels** (`int`, *optional*, defaults to 1) --
  Number of input channels.
- **hidden_size** (`int`, *optional*, defaults to 64) --
  Dimensionality of latent representations.
- **kernel_size** (`int`, *optional*, defaults to 7) --
  Kernel size for convolutional layers.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the rms normalization layers.
- **layer_scale_init_value** (`float`, *optional*, defaults to `1e-06`) --
  Scale to use in the self-attention layers. 0.1 for base, 1e-6 for large. Set 0 to disable layer scale.
- **initializer_range** (`float`, *optional*, defaults to `0.01`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **num_filters** (`int`, *optional*, defaults to 32) --
  Number of filters in initial convolutional layer, and doubles after each downsampling.
- **downsampling_ratios** (`List[int]`, *optional*, defaults to `[2, 2, 4, 5, 5, 8]`) --
  Downsampling ratios for each layer.
- **depths** (`List[int]`, *optional*, defaults to `[3, 3, 3, 3, 3, 3, 8]`) --
  Number of ConvNeXt blocks at each stage.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **ffn_expansion** (`int`, *optional*, defaults to 4) --
  Expansion factor for feed-forward networks.
- **vae_std** (`float`, *optional*, defaults to 0.625) --
  Standard deviation used for VAE sampling after encoder.

This is the configuration class to store the configuration of a VibeVoiceAcousticTokenizerModel. It is used to instantiate a Vibevoice Acoustic Tokenizer
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [microsoft/VibeVoice-1.5B](https://huggingface.co/microsoft/VibeVoice-1.5B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import VibeVoiceAcousticTokenizerModel, VibeVoiceAcousticTokenizerConfig

>>> # Initializing a VibeVoice Acoustic Tokenizer configuration
>>> configuration = VibeVoiceAcousticTokenizerConfig()

>>> # Initializing a model (with random weights)
>>> model = VibeVoiceAcousticTokenizerModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## VibeVoiceAcousticTokenizerEncoderConfig[[transformers.VibeVoiceAcousticTokenizerEncoderConfig]]

- **channels** (`int`, *optional*, defaults to 1) --
  Number of input channels.
- **hidden_size** (`int`, *optional*, defaults to 64) --
  Dimensionality of latent representations.
- **kernel_size** (`int`, *optional*, defaults to 7) --
  Kernel size for convolutional layers.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the rms normalization layers.
- **layer_scale_init_value** (`float`, *optional*, defaults to `1e-06`) --
  Scale to use in the self-attention layers. 0.1 for base, 1e-6 for large. Set 0 to disable layer scale.
- **initializer_range** (`float`, *optional*, defaults to `0.01`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **num_filters** (`int`, *optional*, defaults to 32) --
  Number of filters in initial convolutional layer, and doubles after each downsampling.
- **downsampling_ratios** (`List[int]`, *optional*, defaults to `[2, 2, 4, 5, 5, 8]`) --
  Downsampling ratios for each layer.
- **depths** (`List[int]`, *optional*, defaults to `[3, 3, 3, 3, 3, 3, 8]`) --
  Number of ConvNeXt blocks at each stage.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **ffn_expansion** (`int`, *optional*, defaults to 4) --
  Expansion factor for feed-forward networks.
- **vae_std** (`float`, *optional*, defaults to 0.625) --
  Standard deviation used for VAE sampling after encoder.

This is the configuration class to store the configuration of a VibeVoiceAcousticTokenizerModel. It is used to instantiate a Vibevoice Acoustic Tokenizer
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [microsoft/VibeVoice-1.5B](https://huggingface.co/microsoft/VibeVoice-1.5B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## VibeVoiceAcousticTokenizerDecoderConfig[[transformers.VibeVoiceAcousticTokenizerDecoderConfig]]

- **channels** (`int`, *optional*, defaults to 1) --
  Number of input channels.
- **hidden_size** (`int`, *optional*, defaults to 64) --
  Dimensionality of latent representations.
- **kernel_size** (`int`, *optional*, defaults to 7) --
  Kernel size for convolutional layers.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the rms normalization layers.
- **layer_scale_init_value** (`float`, *optional*, defaults to `1e-06`) --
  Scale to use in the self-attention layers. 0.1 for base, 1e-6 for large. Set 0 to disable layer scale.
- **initializer_range** (`float`, *optional*, defaults to `0.01`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **num_filters** (`int`, *optional*, defaults to 32) --
  Number of filters in initial convolutional layer, and doubles after each downsampling.
- **downsampling_ratios** (`List[int]`, *optional*, defaults to `[2, 2, 4, 5, 5, 8]`) --
  Downsampling ratios for each layer.
- **depths** (`List[int]`, *optional*, defaults to `[3, 3, 3, 3, 3, 3, 8]`) --
  Number of ConvNeXt blocks at each stage.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **ffn_expansion** (`int`, *optional*, defaults to 4) --
  Expansion factor for feed-forward networks.
- **vae_std** (`float`, *optional*, defaults to 0.625) --
  Standard deviation used for VAE sampling after encoder.

This is the configuration class to store the configuration of a VibeVoiceAcousticTokenizerModel. It is used to instantiate a Vibevoice Acoustic Tokenizer
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [microsoft/VibeVoice-1.5B](https://huggingface.co/microsoft/VibeVoice-1.5B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## VibeVoiceAcousticTokenizerFeatureExtractor[[transformers.VibeVoiceAcousticTokenizerFeatureExtractor]]

- **feature_size** (`int`, *optional*, defaults to 1) --
  The number of channels.
- **sampling_rate** (`int`, *optional*, defaults to 24000) --
  The sampling rate at which the audio waveform should be digitalized, expressed in hertz (Hz).
- **padding_value** (`float`, *optional*, defaults to 0.0) --
  The value that is used for padding.
- **normalize_audio** (`bool`, *optional*, defaults to `True`) --
  Whether to normalize audio to a target dB FS.
- **target_dB_FS** (`float`, *optional*, defaults to -25) --
  Target dB FS for normalization.
- **eps** (`float`, *optional*, defaults to 1e-06) --
  A small value to avoid division by zero when normalizing.

Constructs a VibeVoiceAcousticTokenizer feature extractor.

- **audio** (`np.ndarray`, `torch.Tensor`, `list[np.ndarray]`, `list[torch.Tensor]` --
  The sequence or batch of sequences to be processed. Each sequence can be a numpy array, a torch tensor,
  a list of numpy arrays or a list of torch tensors.
- **sampling_rate** (`int`, *optional*) --
  The sampling rate at which the `audio` input was sampled. It is strongly recommended to pass
  `sampling_rate` at the forward call to prevent silent errors.
- **padding** (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `True`) --
  Select a strategy to pad the returned sequences (according to the model's padding side and padding
  index) among:

  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single
    sequence if provided).
  - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum
    acceptable input length for the model if that argument is not provided.
  - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different
    lengths).
- **pad_to_multiple_of** (`int`, *optional*) --
  If set will pad the sequence to a multiple of the provided value.

## VibeVoiceAcousticTokenizerModel[[transformers.VibeVoiceAcousticTokenizerModel]]

- **config** ([VibeVoiceAcousticTokenizerModel](/docs/transformers/v5.14.0/en/model_doc/vibevoice_acoustic_tokenizer#transformers.VibeVoiceAcousticTokenizerModel)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

VibeVoice acoustic tokenizer with an encoder and decoder for continuous acoustic tokens.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_values** (`torch.FloatTensor` of shape `(batch_size, channels, sequence_length)`) --
  Input audio waveform to be encoded into latent representation.
- **padding_cache** (`VibeVoiceAcousticTokenizerConv1dPaddingCache`, *optional*) --
  Cache object for streaming mode to maintain convolution states across layers.
- **use_cache** (`bool`, *optional*) --
  Whether to use caching for convolution states.
- **sample** (`bool`, *optional*) --
  Whether to sample from the VAE. If False, no noise is added.

- **latents** (`torch.FloatTensor` of shape `(batch_size, channels, sequence_length)`) --
  Input latent representation to be decoded back into audio.
- **padding_cache** (`VibeVoiceAcousticTokenizerConv1dPaddingCache`, *optional*) --
  Cache object for streaming mode to maintain convolution states across layers.
- **use_cache** (`bool`, *optional*) --
  Whether to use caching for convolution states.

- **input_values** (`torch.FloatTensor` of shape `(batch_size, channels, sequence_length)`) --
  Input audio waveform to be encoded into latent representation.
- **padding_cache** (`VibeVoiceAcousticTokenizerConv1dPaddingCache`, *optional*) --
  Cache object for streaming mode to maintain convolution states across layers. Note only used by decoder.
- **use_cache** (`bool`, *optional*) --
  Whether to use caching for convolution states.
- **sample** (`bool`, *optional*) --
  Whether to sample from the VAE latent distribution. If False, no noise is added to the latents.
The [VibeVoiceAcousticTokenizerModel](/docs/transformers/v5.14.0/en/model_doc/vibevoice_acoustic_tokenizer#transformers.VibeVoiceAcousticTokenizerModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

## VibeVoiceAcousticTokenizerEncoderModel[[transformers.VibeVoiceAcousticTokenizerEncoderModel]]

## VibeVoiceAcousticTokenizerDecoderModel[[transformers.VibeVoiceAcousticTokenizerDecoderModel]]
