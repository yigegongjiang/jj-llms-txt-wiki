# PE Video

[PE Video](https://huggingface.co/papers/2504.13181) is the video branch of Meta's Perception Encoder family. It contrastively aligns video clips with text into a shared embedding space, enabling zero-shot video classification and video–text retrieval from a single pretrained backbone.

The encoder's rotary embeddings and patch embedder treat the temporal axis as a first-class dimension, so variable-length clips can be encoded without tiling each frame independently.

You can find all the official PE Audio checkpoints under the [perception-encoder-audio-visual](https://huggingface.co/collections/facebook/perception-encoder-audio-visual) collection.

## Quickstart

```py
import torch
from transformers import AutoProcessor, PeVideoModel
from transformers.video_utils import load_video

processor = AutoProcessor.from_pretrained("facebook/pe-av-large")
model = PeVideoModel.from_pretrained(
    "facebook/pe-av-large",
    device_map="auto",
)

video, _ = load_video("https://huggingface.co/datasets/hf-internal-testing/fixtures_videos/resolve/main/tennis.mp4")
labels = ["a person playing tennis", "a person cooking", "a cat sleeping"]

video_inputs = processor.video_processor(video, num_frames=16, return_tensors="pt").to(model.device)
text_inputs = processor.tokenizer(labels, padding=True, return_tensors="pt").to(model.device)
inputs = {**video_inputs, **text_inputs}

with torch.no_grad():
    outputs = model(**inputs)

probs = outputs.logits_video_text.sigmoid()
print({label: p.item() for label, p in zip(labels, probs[0])})
```

## Usage tips and notes

- Variable-length videos use `padding_mask_videos` (not `attention_mask`). The video processor only pads and returns this mask when `return_tensors` is set — without it you get a list of per-clip tensors and no mask.
- Pass `num_frames` to the video processor for fixed-length uniform sampling across `[0, total_frames-1]`. Omit it to fall back to fps-based sampling from the base class. Checkpoints are usually trained at a specific frame count, so match what the checkpoint expects.
- Encoder input is `pixel_values_videos`. The encoder's `main_input_name` is `"pixel_values_videos"` while the full model's is `"input_ids"`, which matters when routing through generic utilities that inspect `main_input_name`.

## PeVideoConfig[[transformers.PeVideoConfig]]

- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **video_config** (`dict` or `PreTrainedConfig`, *optional*) --
  Configuration for the video encoder component.

This is the configuration class to store the configuration of a PeVideoModel. It is used to instantiate a Pe Video
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/pe-av-large](https://huggingface.co/facebook/pe-av-large)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import PeVideoModel, PeVideoConfig

>>> # Initializing a PeVideoModel style configuration
>>> configuration = PeVideoConfig()

>>> # Initializing a model from the pe-av-large style configuration
>>> model = PeVideoModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## PeVideoEncoderConfig[[transformers.PeVideoEncoderConfig]]

- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
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

This is the configuration class to store the configuration of a PeVideoModel. It is used to instantiate a Pe Video
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/pe-av-large](https://huggingface.co/facebook/pe-av-large)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import PeAudioEncoder, PeAudioEncoderConfig

>>> # Initializing a PeAudioEncoder style configuration
>>> configuration = PeAudioEncoderConfig()

>>> # Initializing a model from the pe-av-large style configuration
>>> model = PeAudioEncoder(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## PeVideoVideoProcessor[[transformers.PeVideoVideoProcessor]]

## PeVideoProcessor[[transformers.PeVideoProcessor]]

## PeVideoEncoder[[transformers.PeVideoEncoder]]

- **config** ([PeVideoEncoderConfig](/docs/transformers/v5.14.0/en/model_doc/pe_video#transformers.PeVideoEncoderConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The PeVideo Encoder model.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "padding_mask_videos", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "**kwargs", "val": ""}]}>

## PeVideoModel[[transformers.PeVideoModel]]

)>"}, {"name": "pixel_values_videos", "val": ": )>"}, {"name": "attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "padding_mask_videos", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "return_loss", "val": ": bool | None = None"}, {"name": "**kwargs", "val": ""}]}>
