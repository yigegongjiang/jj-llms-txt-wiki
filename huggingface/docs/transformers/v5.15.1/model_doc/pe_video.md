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

#### transformers.PeVideoConfig[[transformers.PeVideoConfig]]

```python
transformers.PeVideoConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, video_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_video/configuration_pe_video.py#L91)

**Parameters:**

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

video_config (`dict` or `PreTrainedConfig`, *optional*) : Configuration for the video encoder component.

This is the configuration class to store the configuration of a PeVideoModel. It is used to instantiate a Pe Video
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/pe-av-large](https://huggingface.co/facebook/pe-av-large)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

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

#### transformers.PeVideoEncoderConfig[[transformers.PeVideoEncoderConfig]]

```python
transformers.PeVideoEncoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vision_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, hidden_size: int = 1792, intermediate_size: int = 4800, num_hidden_layers: int = 6, num_attention_heads: int = 14, num_key_value_heads: int | None = None, head_dim: int = 128, hidden_act: str = 'silu', max_position_embeddings: int = 10000, initializer_range: float = 0.02, rms_norm_eps: float = 1e-05, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, attention_bias: bool = False, attention_dropout: float | int = 0.0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_video/configuration_pe_video.py#L27)

**Parameters:**

vision_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the vision backbone.

hidden_size (`int`, *optional*, defaults to `1792`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `4800`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `6`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `14`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

head_dim (`int`, *optional*, defaults to `128`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `10000`) : The maximum sequence length that this model might ever be used with.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

rms_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the rms normalization layers.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

attention_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

This is the configuration class to store the configuration of a PeVideoModel. It is used to instantiate a Pe Video
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/pe-av-large](https://huggingface.co/facebook/pe-av-large)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

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

#### transformers.PeVideoVideoProcessor[[transformers.PeVideoVideoProcessor]]

```python
transformers.PeVideoVideoProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_video/video_processing_pe_video.py#L24)

## PeVideoProcessor[[transformers.PeVideoProcessor]]

#### transformers.PeVideoProcessor[[transformers.PeVideoProcessor]]

```python
transformers.PeVideoProcessor(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_video/processing_pe_video.py#L4)

## PeVideoEncoder[[transformers.PeVideoEncoder]]

#### transformers.PeVideoEncoder[[transformers.PeVideoEncoder]]

```python
transformers.PeVideoEncoder(config: PeVideoEncoderConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_video/modeling_pe_video.py#L511)

**Parameters:**

config ([PeVideoEncoderConfig](/docs/transformers/v5.15.1/en/model_doc/pe_video#transformers.PeVideoEncoderConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The PeVideo Encoder model.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.PeVideoEncoder.forward]]

```python
forward(pixel_values_videos: Tensor, padding_mask_videos: typing.Optional[torch.Tensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_video/modeling_pe_video.py#L530)

**Parameters:**

pixel_values_videos (`torch.Tensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`) : The tensors corresponding to the input video. Pixel values for videos can be obtained using [PeVideoVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/pe_video#transformers.PeVideoVideoProcessor). See `PeVideoVideoProcessor.__call__()` for details ([PeVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/pe_video#transformers.PeVideoProcessor) uses [PeVideoVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/pe_video#transformers.PeVideoVideoProcessor) for processing videos).

padding_mask_videos (`torch.Tensor` of shape `(batch_size, num_frames)`, *optional*) : Mask to avoid performing attention on padding video frames. Mask values selected in `[0, 1]`:  - 1 for frames that are **not masked**, - 0 for frames that are **masked**.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PeVideoConfig](/docs/transformers/v5.15.1/en/model_doc/pe_video#transformers.PeVideoConfig)) and inputs.

The [PeVideoEncoder](/docs/transformers/v5.15.1/en/model_doc/pe_video#transformers.PeVideoEncoder) forward method, overrides the `__call__` special method.

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

## PeVideoModel[[transformers.PeVideoModel]]

#### transformers.PeVideoModel[[transformers.PeVideoModel]]

```python
transformers.PeVideoModel(config: PeVideoConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_video/modeling_pe_video.py#L577)

#### forward[[transformers.PeVideoModel.forward]]

```python
forward(input_ids: Tensor, pixel_values_videos: Tensor, attention_mask: typing.Optional[torch.Tensor] = None, padding_mask_videos: typing.Optional[torch.Tensor] = None, return_loss: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_video/modeling_pe_video.py#L627)

**Parameters:**

input_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

pixel_values_videos (`torch.Tensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`) : The tensors corresponding to the input video. Pixel values for videos can be obtained using [PeVideoVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/pe_video#transformers.PeVideoVideoProcessor). See `PeVideoVideoProcessor.__call__()` for details ([PeVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/pe_video#transformers.PeVideoProcessor) uses [PeVideoVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/pe_video#transformers.PeVideoVideoProcessor) for processing videos).

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

padding_mask_videos (`torch.Tensor` of shape `(batch_size, num_frames)`, *optional*) : Mask to avoid performing attention on padding video frames. Mask values selected in `[0, 1]`:  - 1 for frames that are **not masked**, - 0 for frames that are **masked**.

return_loss (`bool`, *optional*) : Whether or not to return the loss.

**Returns:** `PeVideoOutput` or `tuple(torch.FloatTensor)`

A `PeVideoOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PeVideoConfig](/docs/transformers/v5.15.1/en/model_doc/pe_video#transformers.PeVideoConfig)) and inputs.

The [PeVideoModel](/docs/transformers/v5.15.1/en/model_doc/pe_video#transformers.PeVideoModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*) -- Contrastive loss computed between video and text representations.
- **logits_video_text** (`torch.FloatTensor` of shape `(batch_size, batch_size)`, *optional*) -- Similarity logits between video and text embeddings.
- **text_video_embeds** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`, *optional*) -- Text embeddings projected to the video-text space.
- **video_embeds** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`, *optional*) -- Video embeddings projected to the video-text space.
- **text_outputs** (`BaseModelOutputWithPooling`, *optional*) -- Model outputs for the text encoder, including last hidden state and pooled output.
- **video_outputs** (`BaseModelOutputWithPooling`, *optional*) -- Model outputs for the video encoder, including last hidden state and pooled output.
