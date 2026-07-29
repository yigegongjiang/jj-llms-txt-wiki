# PE Audio Video

[PE Audio Video](https://huggingface.co/papers/2504.13181) is the joint audio–video branch of Meta's Perception Encoder family. It encodes audio and video streams together with a shared text tower, producing contrastive embeddings for every pairwise combination, audio-text, video-text, audio-video, and audio+text-video, from a single forward pass.

Internally the model aligns the video feature sequence to the audio's temporal resolution via nearest-neighbor interpolation, so clips with different frame rates from sample rates stay in lockstep. The text encoder weights are tied across the audio and video branches.

You can find all the official PE Audio Video checkpoints under the [perception-encoder-audio-visual](https://huggingface.co/collections/facebook/perception-encoder-audio-visual) collection.

## Quickstart

```py
import torch
from datasets import load_dataset
from transformers import AutoProcessor, PeAudioVideoModel
from transformers.video_utils import load_video

processor = AutoProcessor.from_pretrained("facebook/pe-av-large")
model = PeAudioVideoModel.from_pretrained(
    "facebook/pe-av-large",
    device_map="auto",
)

ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
audio = ds[0]["audio"]["array"]
video, _ = load_video("https://huggingface.co/datasets/hf-internal-testing/fixtures_videos/resolve/main/tennis.mp4")
labels = ["a person playing tennis with background crowd", "a dog barking in a park"]

audio_inputs = processor.feature_extractor(audio, sampling_rate=48_000, return_tensors="pt").to(model.device)
video_inputs = processor.video_processor(video, num_frames=16, return_tensors="pt").to(model.device)
text_inputs = processor.tokenizer(labels, padding=True, return_tensors="pt").to(model.device)
inputs = {**audio_inputs, **video_inputs, **text_inputs}

with torch.no_grad():
    outputs = model(**inputs)

print("audio-text:", outputs.logits_audio_text.sigmoid().tolist())
print("video-text:", outputs.logits_video_text.sigmoid().tolist())
print("audio-video:", outputs.logits_audio_video.sigmoid().tolist())
```

## Usage tips and notes

- [PeAudioVideoModel](/docs/transformers/v5.14.0/en/model_doc/pe_audio_video#transformers.PeAudioVideoModel) requires at least two of `input_ids`, `input_values`, `pixel_values_videos` — if only two are provided it dispatches to the audio-only or video-only sub-model. Passing all three triggers the joint audio-video-text path and the full set of logit matrices in `PeAudioVideoOutput`.
- Audio uses `padding_mask` and video uses `padding_mask_videos` simultaneously. They are independent masks; do not conflate them with `attention_mask`, which is reserved for the text tower.
- Audio–video alignment runs per-batch-element inside `_align_video_hidden_state`, so batches with very different audio/video lengths iterate rather than vectorizing. Keep batch items roughly balanced for throughput.
- The text tower's weights are tied across branches via `_tied_weights_keys` — do not try to load separate text encoders for the audio and video halves.

## PeAudioVideoConfig[[transformers.PeAudioVideoConfig]]

- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **audio_video_config** (`dict` or `PreTrainedConfig`, *optional*) --
  Configuration for the audio-video encoder component.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a PeAudioVideoModel. It is used to instantiate a Pe Audio Video
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/pe-av-large](https://huggingface.co/facebook/pe-av-large)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import PeAudioVideoModel, PeAudioVideoConfig

>>> # Initializing a PeAudioVideoModel style configuration
>>> configuration = PeAudioVideoConfig()

>>> # Initializing a model from the pe-av-large style configuration
>>> model = PeAudioModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## PeAudioVideoEncoderConfig[[transformers.PeAudioVideoEncoderConfig]]

- **audio_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the audio backbone.
- **video_config** (`Union[PreTrainedConfig, dict]`, *optional*) --
  Configuration for the video encoder. If a dictionary is provided, it is used to instantiate
  [PeVideoEncoderConfig](/docs/transformers/v5.14.0/en/model_doc/pe_video#transformers.PeVideoEncoderConfig).
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

This is the configuration class to store the configuration of a PeAudioVideoModel. It is used to instantiate a Pe Audio Video
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/pe-av-large](https://huggingface.co/facebook/pe-av-large)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import PeAudioVideoEncoder, PeAudioVideoEncoderConfig

>>> # Initializing a PeAudioVideoEncoder style configuration
>>> configuration = PeAudioVideoEncoderConfig()

>>> # Initializing a model from the pe-av-large style configuration
>>> model = PeAudioVideoEncoder(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## PeAudioVideoProcessor[[transformers.PeAudioVideoProcessor]]

## PeAudioVideoEncoder[[transformers.PeAudioVideoEncoder]]

- **config** ([PeAudioVideoEncoderConfig](/docs/transformers/v5.14.0/en/model_doc/pe_audio_video#transformers.PeAudioVideoEncoderConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The PeAudioVideo Encoder model.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

## PeAudioVideoModel[[transformers.PeAudioVideoModel]]
