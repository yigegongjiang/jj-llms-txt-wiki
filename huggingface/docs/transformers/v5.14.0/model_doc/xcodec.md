# X-Codec

## Overview

The X-Codec model was proposed in [Codec Does Matter: Exploring the Semantic Shortcoming of Codec for Audio Language Model](https://huggingface.co/papers/2408.17175) by Zhen Ye, Peiwen Sun, Jiahe Lei, Hongzhan Lin, Xu Tan, Zheqi Dai, Qiuqiang Kong, Jianyi Chen, Jiahao Pan, Qifeng Liu, Yike Guo, Wei Xue.

The X-Codec model is a neural audio codec that integrates semantic information from self-supervised models (e.g., HuBERT) alongside traditional acoustic information. This enables:

- **Music continuation**: Better modeling of musical semantics yields more coherent continuations.
- **Text-to-Sound Synthesis**: X-Codec captures semantic alignment between text prompts and generated audio.
- **Semantic aware audio tokenization**: X-Codec is used as an audio tokenizer in the YuE lyrics to song generation model.

The abstract of the paper states the following:

*Recent advancements in audio generation have been significantly propelled by the capabilities of Large Language Models (LLMs). The existing research on audio LLM has primarily focused on enhancing the architecture and scale of audio language models, as well as leveraging larger datasets, and generally, acoustic codecs, such as EnCodec, are used for audio tokenization. However, these codecs were originally designed for audio compression, which may lead to suboptimal performance in the context of audio LLM. Our research aims to address the shortcomings of current audio LLM codecs, particularly their challenges in maintaining semantic integrity in generated audio. For instance, existing methods like VALL-E, which condition acoustic token generation on text transcriptions, often suffer from content inaccuracies and elevated word error rates (WER) due to semantic misinterpretations of acoustic tokens, resulting in word skipping and errors. To overcome these issues, we propose a straightforward yet effective approach called X-Codec. X-Codec incorporates semantic features from a pre-trained semantic encoder before the Residual Vector Quantization (RVQ) stage and introduces a semantic reconstruction loss after RVQ. By enhancing the semantic ability of the codec, X-Codec significantly reduces WER in speech synthesis tasks and extends these benefits to non-speech applications, including music and sound generation. Our experiments in text-to-speech, music continuation, and text-to-sound tasks demonstrate that integrating semantic information substantially improves the overall performance of language models in audio generation.*

Model cards:

- [xcodec-hubert-librispeech](https://huggingface.co/hf-audio/xcodec-hubert-librispeech) (for speech)
- [xcodec-wavlm-mls](https://huggingface.co/hf-audio/xcodec-wavlm-mls) (for speech)
- [xcodec-wavlm-more-data](https://huggingface.co/hf-audio/xcodec-wavlm-more-data) (for speech)
- [xcodec-hubert-general](https://huggingface.co/hf-audio/xcodec-hubert-general) (for general audio)
- [xcodec-hubert-general-balanced](https://huggingface.co/hf-audio/xcodec-hubert-general-balanced) (for general audio)

This model was contributed by [Manal El Aidouni](https://huggingface.co/Manel). The original code can be found [here](https://github.com/zhenye234/xcodec) and original checkpoints for the five different models [here](https://github.com/zhenye234/xcodec?tab=readme-ov-file#available-models).

Demos can be found on this [page](https://x-codec-audio.github.io/).

## Usage example

Here is a quick example of how to encode and decode an audio using this model:

```python
from datasets import Audio, load_dataset

from transformers import AutoFeatureExtractor, XcodecModel

dummy_dataset = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")

# load model and feature extractor
model_id = "hf-audio/xcodec-hubert-librispeech"
model = XcodecModel.from_pretrained(model_id, device_map="auto")
feature_extractor = AutoFeatureExtractor.from_pretrained(model_id)

# load audio sample
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

To listen to the original and reconstructed audio, run the snippet below and then open the generated `original.wav` and `reconstruction.wav` files in your music player to compare.

```python
import soundfile as sf

original = audio_sample
reconstruction = audio_values[0].cpu().detach().numpy()
sampling_rate = feature_extractor.sampling_rate

sf.write("original.wav", original, sampling_rate)
sf.write("reconstruction.wav", reconstruction.T, sampling_rate)
```

## XcodecConfig[[transformers.XcodecConfig]]

- **target_bandwidths** (`List[float]`, *optional*, defaults to `[0.5, 1, 1.5, 2, 4]`) --
  The range of different bandwidths (in kbps) the model can encode audio with.
- **sample_rate** (`int`, *optional*, defaults to `16000`) --
  The sampling rate at which the audio files should be digitalized expressed in hertz (Hz).
- **kernel_size** (`int`, *optional*, defaults to `3`) --
  The size of the convolutional kernel.
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
- **codebook_dim** (`int`, *optional*) --
  Dimensionality of each codebook embedding vector.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **acoustic_model_config** (`Union[Dict, DacConfig]`, *optional*) --
  An instance of the configuration for the acoustic (DAC) model.
- **semantic_model_config** (`Union[Dict, HubertConfig, WavLMConfig]`, *optional*) --
  An instance of the configuration object for the semantic (HuBERT) model.

This is the configuration class to store the configuration of a XcodecModel. It is used to instantiate a Xcodec
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Manel/X-Codec](https://huggingface.co/Manel/X-Codec)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import XcodecModel, XcodecConfig

>>> # Initializing configuration
>>> configuration = XcodecConfig()

>>> # Initializing a model (with random weights) from the configuration
>>> model = XcodecModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## XcodecModel[[transformers.XcodecModel]]

- **config** ([XcodecModel](/docs/transformers/v5.14.0/en/model_doc/xcodec#transformers.XcodecModel)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
The Xcodec neural audio codec model.

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
Xcodec.

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
  Target bandwidth in kbps. Must be one of `config.target_bandwidths`. Defaults to the highest available bandwidth.`XcodecOutput` or tuple `(audio_codes, audio_values)`- `audio_codes` of shape `(batch_size, num_quantizers, codes_length)`: the quantized discrete codes.
- `audio_values` of shape `(batch_size, channels, num_samples)`: the reconstructed audio waveform given the codes.
The [XcodecModel](/docs/transformers/v5.14.0/en/model_doc/xcodec#transformers.XcodecModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Example:

```python
>>> from datasets import load_dataset
>>> from transformers import AutoFeatureExtractor, XcodecModel

>>> model_id = "hf-audio/xcodec-hubert-librispeech"
>>> model = XcodecModel.from_pretrained(model_id)
>>> feature_extractor = AutoFeatureExtractor.from_pretrained(model_id)

>>> dataset = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> dataset = dataset.cast_column("audio", Audio(sampling_rate=feature_extractor.sampling_rate))
>>> audio_sample = dataset[0]['audio']['array']

>>> inputs = feature_extractor(raw_audio=audio_sample, return_tensors="pt")

>>> outputs = model(**inputs)
>>> audio_codes = outputs.audio_codes
>>> audio_values = outputs.audio_values
```
