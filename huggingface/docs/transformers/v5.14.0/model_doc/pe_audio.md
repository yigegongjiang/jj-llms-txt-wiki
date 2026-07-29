# PE Audio

[PE Audio](https://huggingface.co/papers/2504.13181) is the audio branch of Meta's Perception Encoder family. It contrastively aligns raw waveforms with text into a shared embedding space, trained on paired audio–caption data for cross-modal retrieval and zero-shot audio classification.

Two heads are exposed on top of the same encoder. [PeAudioModel](/docs/transformers/v5.14.0/en/model_doc/pe_audio#transformers.PeAudioModel) returns one pooled embedding per clip for clip-level retrieval, while [PeAudioFrameLevelModel](/docs/transformers/v5.14.0/en/model_doc/pe_audio#transformers.PeAudioFrameLevelModel) returns one embedding every 40 ms for event localization and fine-grained temporal analysis.

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
- [PeAudioModel](/docs/transformers/v5.14.0/en/model_doc/pe_audio#transformers.PeAudioModel) returns logits of shape `(n_audio, n_text)`. [PeAudioFrameLevelModel](/docs/transformers/v5.14.0/en/model_doc/pe_audio#transformers.PeAudioFrameLevelModel) returns `(n_audio, n_text, n_frames)` with one frame every 40 ms. Pick the class that matches the task — they share weights so swapping is cheap.
- The text tower is a shared encoder loaded via `AutoModel` from `config.text_config`. The tokenizer is attached to the processor via `AutoTokenizer`, not a dedicated class.

## PeAudioConfig[[transformers.PeAudioConfig]]

- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **audio_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the audio backbone.

This is the configuration class to store the configuration of a PeAudioModel. It is used to instantiate a Pe Audio
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/pe-av-large](https://huggingface.co/facebook/pe-av-large)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

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

- **dac_config** (`Union[PreTrainedConfig, dict]`, *optional*) --
  Configuration for the DAC audio encoder used to tokenize the raw audio inputs. If a dictionary is passed, it
  will be used to instantiate a [DacConfig](/docs/transformers/v5.14.0/en/model_doc/dac#transformers.DacConfig) with default DAC hyperparameters.
- **hidden_size** (`int`, *optional*, defaults to `1792`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `4800`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `6`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `14`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_key_value_heads** (`int`, *optional*) --
  This is the number of key_value heads that should be used to implement Grouped Query Attention. If
  `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if
  `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When
  converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed
  by meanpooling all the original heads within that group. For more details, check out [this
  paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to
  `num_attention_heads`.
- **head_dim** (`int`, *optional*, defaults to `128`) --
  The attention head dimension. If None, it will default to hidden_size // num_attention_heads
- **hidden_act** (`str`, *optional*, defaults to `silu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **max_position_embeddings** (`int`, *optional*, defaults to `10000`) --
  The maximum sequence length that this model might ever be used with.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the rms normalization layers.
- **rope_parameters** (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **attention_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in the query, key, value and output projection layers during self-attention.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.

This is the configuration class to store the configuration of a PeAudioModel. It is used to instantiate a Pe Audio
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/pe-av-large](https://huggingface.co/facebook/pe-av-large)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

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

- **feature_size** (`int`, *optional*, defaults to 1) --
  The feature dimension of the extracted features. Use 1 for mono, 2 for stereo.
- **sampling_rate** (`int`, *optional*, defaults to 48000) --
  The sampling rate at which the audio waveform should be digitalized, expressed in hertz (Hz).
- **padding_value** (`float`, *optional*, defaults to 0.0) --
  The value that is used for padding.
- **hop_length** (`int`, *optional*, defaults to 1920) --
  Overlap length between successive windows.

Constructs a PeAudioFeatureExtractor feature extractor.

This feature extractor inherits from [SequenceFeatureExtractor](/docs/transformers/v5.14.0/en/main_classes/feature_extractor#transformers.SequenceFeatureExtractor) which contains
most of the main methods. Users should refer to this superclass for more information regarding those methods.

## PeAudioProcessor[[transformers.PeAudioProcessor]]

## PeAudioEncoder[[transformers.PeAudioEncoder]]

- **config** ([PeAudioEncoderConfig](/docs/transformers/v5.14.0/en/model_doc/pe_audio#transformers.PeAudioEncoderConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The PeAudio Encoder model.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "padding_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "**kwargs", "val": ""}]}>

## PeAudioModel[[transformers.PeAudioModel]]

)>"}, {"name": "input_values", "val": ": )>"}, {"name": "attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "padding_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "return_loss", "val": ": bool | None = None"}, {"name": "**kwargs", "val": ""}]}>

## PeAudioFrameLevelModel[[transformers.PeAudioFrameLevelModel]]

)>"}, {"name": "input_values", "val": ": )>"}, {"name": "attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "padding_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "return_loss", "val": ": bool | None = None"}, {"name": "**kwargs", "val": ""}]}>
