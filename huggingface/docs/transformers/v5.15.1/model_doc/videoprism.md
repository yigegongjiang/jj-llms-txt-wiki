# VideoPrism

The VideoPrism model was proposed in the paper [VideoPrism: A Foundational Visual Encoder for Video Understanding](https://huggingface.co/papers/2402.13217) by Google DeepMind ([blog post](https://research.google/blog/videoprism-a-foundational-visual-encoder-for-video-understanding/)).

VideoPrism is a general-purpose video encoder that tackles diverse video understanding tasks with a single frozen model. The model is pretrained on a large-scale heterogeneous corpus containing 36M high-quality video-caption pairs and 582M video clips with noisy parallel text (e.g., ASR transcripts). The pretraining approach improves upon masked autoencoding through global-local distillation of semantic video embeddings and a token shuffling scheme, enabling the model to focus primarily on the video modality while leveraging text associated with videos. VideoPrism achieves state-of-the-art performance on 31 out of 33 video understanding benchmarks across four broad task groups, from web video question answering to computer vision for science.

    

You can find all original VideoPrism checkpoints under the [VideoPrism](https://huggingface.co/collections/google/videoprism) collection.

Notes:

- VideoPrism uses a factorized spatio-temporal encoder architecture, processing videos through separate spatial and temporal transformers.
- The model supports video-text contrastive learning through `VideoPrismClipModel`, which combines a video encoder and a text encoder. `VideoPrismConfig` must be used with this model.
- For video classification tasks, use `VideoPrismForVideoClassification` which adds a classification head on top of the video encoder. `VideoPrismVisionConfig` must be used with this model.
- The vision encoder can be used standalone via `VideoPrismVisionModel` for extracting video features. `VideoPrismVisionConfig` must be used with this model.
- The default input resolution is 288x288 pixels with 16 frames per video clip for the base models and 8 frames for the large models. Set interpolate_pos_encoding=True to use the models with custom resolution and frames per clip.

This model was contributed by [MHRDYN7](https://github.com/MHRDYN7) and reviewed by [vasqu](https://github.com/vasqu) & [zucchini-nlp](https://github.com/zucchini-nlp).
The original code can be found [here](https://github.com/google-deepmind/videoprism).

## Usage example

The snippet below shows how to load the VideoPrismVisionModel for feature extraction using the `AutoModel` class.

```py
import torch
from transformers import AutoModel, AutoVideoProcessor

processor = AutoVideoProcessor.from_pretrained("google/videoprism-base-f16r288", revision="refs/pr/4")
model = AutoModel.from_pretrained(
    "google/videoprism-base-f16r288",
    revision="refs/pr/4",
    device_map="auto",
    # use "flash_attention_2" for faster inference on supported hardware
    # attn_implementation="flash_attention_2" 
)

video_url = "https://huggingface.co/datasets/nateraw/kinetics-mini/resolve/main/val/archery/-Qz25rXdMjE_000014_000024.mp4"

# when do_sample_frames=True, 16/8 frames will be sampled by default depending on the checkpoint size base/large.
processed_video_inputs = processor(videos=[video_url], return_metadata=True, do_sample_frames=True)
video_metadata = processed_video_inputs["video_metadata"]
video_inputs = processed_video_inputs["pixel_values_videos"].to(model.device)
outputs = model(video_inputs)

# VideoPrism encoder outputs
encoder_outputs = outputs.last_hidden_state

```

## VideoPrismVisionConfig[[transformers.VideoPrismVisionConfig]]

#### transformers.VideoPrismVisionConfig[[transformers.VideoPrismVisionConfig]]

```python
transformers.VideoPrismVisionConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, image_size: int | list[int] | tuple[int, int] = 288, num_frames: int = 16, tubelet_size: list[int] | tuple[int, ...] = (1, 18, 18), num_channels: int = 3, hidden_size: int = 768, num_attention_heads: int = 12, intermediate_size: int = 3072, hidden_act: str = 'gelu_python', hidden_dropout_prob: float | int = 0.0, attention_probs_dropout_prob: float | int = 0.0, initializer_range: float = 0.02, layer_norm_eps: float = 1e-06, qkv_bias: bool = True, num_spatial_layers: int = 12, num_temporal_layers: int = 4, attn_logit_softcapping: float = 50.0, num_auxiliary_layers: int = 2, apply_l2norm: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/videoprism/configuration_videoprism.py#L33)

**Parameters:**

image_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `288`) : The size (resolution) of each image.

num_frames (`int`, *optional*, defaults to 16) : The number of frames in the input video.

tubelet_size (`List[int]`, *optional*, defaults to `[1, 18, 18]`) : The size of the tubelet patch.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

num_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer decoder.

intermediate_size (`int`, *optional*, defaults to `3072`) : Dimension of the MLP representations.

hidden_act (`str`, *optional*, defaults to `gelu_python`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

hidden_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

attention_probs_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

qkv_bias (`bool`, *optional*, defaults to `True`) : Whether to add a bias to the queries, keys and values.

num_spatial_layers (`int`, *optional*, defaults to 12) : Number of spatial transformer blocks.

num_temporal_layers (`int`, *optional*, defaults to 4) : Number of temporal transformer blocks.

attn_logit_softcapping (`float`, *optional*, defaults to 50.0) : Softcapping constant for attention logits.

num_auxiliary_layers (`int`, *optional*, defaults to 2) : Number of auxiliary layers. This is used in the VideoPrismVideoModel that is a part of VideoPrismClipModel.

apply_l2norm (`bool`, *optional*, defaults to `True`) : Whether to apply L2 normalization to the output. This is used in the VideoPrismVideoModel that is a part of VideoPrismClipModel.

This is the configuration class to store the configuration of a VideoPrismClipModel. It is used to instantiate a Videoprism
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/videoprism-base-f16r288](https://huggingface.co/google/videoprism-base-f16r288)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## VideoPrismTextConfig[[transformers.VideoPrismTextConfig]]

#### transformers.VideoPrismTextConfig[[transformers.VideoPrismTextConfig]]

```python
transformers.VideoPrismTextConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 32000, hidden_size: int = 768, intermediate_size: int = 3072, num_hidden_layers: int = 12, num_attention_heads: int = 12, max_position_embeddings: int = 64, hidden_act: str = 'relu', layer_norm_eps: float = 1e-06, pad_token_id: int | None = 0, bos_token_id: int | None = None, eos_token_id: int | list[int] | None = None, attention_probs_dropout_prob: float | int = 0.0, apply_l2norm: bool = True, qkv_bias: bool = True, hidden_dropout_prob: float = 0.0, initializer_range: float = 0.02, attn_logit_softcapping: float = 50.0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/videoprism/configuration_videoprism.py#L75)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `32000`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `3072`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer decoder.

max_position_embeddings (`int`, *optional*, defaults to `64`) : The maximum sequence length that this model might ever be used with.

hidden_act (`str`, *optional*, defaults to `relu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

pad_token_id (`int`, *optional*, defaults to `0`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*) : Token id used for end-of-stream in the vocabulary.

attention_probs_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

apply_l2norm (`bool`, *optional*, defaults to `True`) : Whether to apply L2 normalization to the output of VideoPrismTextEncoder.

qkv_bias (`bool`, *optional*, defaults to `True`) : Whether to add a bias to the queries, keys and values.

hidden_dropout_prob (`float`, *optional*, defaults to `0.0`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

attn_logit_softcapping (`float`, *optional*, defaults to 50.0) : Softcapping constant for attention logits.

This is the configuration class to store the configuration of a VideoPrismClipModel. It is used to instantiate a Videoprism
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/videoprism-lvt-base-f16r288](https://huggingface.co/google/videoprism-lvt-base-f16r288)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## VideoPrismConfig[[transformers.VideoPrismConfig]]

#### transformers.VideoPrismConfig[[transformers.VideoPrismConfig]]

```python
transformers.VideoPrismConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, vision_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/videoprism/configuration_videoprism.py#L108)

**Parameters:**

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

vision_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the vision backbone.

This is the configuration class to store the configuration of a VideoPrismClipModel. It is used to instantiate a Videoprism
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/videoprism-lvt-base-f16r288](https://huggingface.co/google/videoprism-lvt-base-f16r288)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import VideoPrismClipModel, VideoPrismConfig

>>> # Initializing a VideoPrismConfig with default values
>>> configuration = VideoPrismConfig()

>>> # Initializing a VideoPrismClipModel with the configuration
>>> model = VideoPrismClipModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## VideoPrismTokenizer[[transformers.VideoPrismTokenizer]]

#### transformers.VideoPrismTokenizer[[transformers.VideoPrismTokenizer]]

```python
transformers.VideoPrismTokenizer(vocab: str | list[tuple[str, float]] | None = None, eos_token = '</s>', unk_token = '<unk>', pad_token = '<pad>', _spm_precompiled_charsmap = None, extra_ids = 100, additional_special_tokens = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/videoprism/tokenization_videoprism.py#L33)

Constructs a VideoPrism tokenizer, which is essentially a T5 tokenizer without its postprocessor
(appending an EOS token at the end of the sequence).

This tokenizer inherits from [T5Tokenizer](/docs/transformers/v5.15.1/en/model_doc/t5#transformers.T5Tokenizer) which contains most of the main methods. Users should refer to this
superclass for more information regarding those methods.

#### get_sentinel_token_ids[[transformers.VideoPrismTokenizer.get_sentinel_token_ids]]

```python
get_sentinel_token_ids()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/videoprism/tokenization_videoprism.py#L121)

Get the token IDs for sentinel tokens.

#### get_sentinel_tokens[[transformers.VideoPrismTokenizer.get_sentinel_tokens]]

```python
get_sentinel_tokens()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/videoprism/tokenization_videoprism.py#L115)

Get the list of sentinel tokens (extra_id tokens) from additional_special_tokens.

## VideoPrismProcessor[[transformers.VideoPrismProcessor]]

#### transformers.VideoPrismProcessor[[transformers.VideoPrismProcessor]]

```python
transformers.VideoPrismProcessor(video_processor = None, tokenizer = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/videoprism/processing_videoprism.py#L42)

**Parameters:**

video_processor (`LlavaOnevisionVideoProcessor`) : The video processor is a required input.

tokenizer (`VideoPrismTokenizer`) : The tokenizer is a required input.

Constructs a VideoPrismProcessor which wraps a video processor and a tokenizer into a single processor.

[VideoPrismProcessor](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismProcessor) offers all the functionalities of [LlavaOnevisionVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/llava_onevision#transformers.LlavaOnevisionVideoProcessor) and [VideoPrismTokenizer](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismTokenizer). See the
[~LlavaOnevisionVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/llava_onevision#transformers.LlavaOnevisionVideoProcessor) and [~VideoPrismTokenizer](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismTokenizer) for more information.

## VideoPrismVisionModel[[transformers.VideoPrismVisionModel]]

#### transformers.VideoPrismVisionModel[[transformers.VideoPrismVisionModel]]

```python
transformers.VideoPrismVisionModel(config: VideoPrismVisionConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/videoprism/modeling_videoprism.py#L527)

**Parameters:**

config ([VideoPrismVisionConfig](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismVisionConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare VideoPrism vision encoder outputting raw hidden-states without any specific head on top. This model is the backbone encoder used in VideoPrismVideoModel.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.VideoPrismVisionModel.forward]]

```python
forward(pixel_values_videos: typing.Optional[torch.FloatTensor] = None, interpolate_pos_encoding: bool | None = False, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/videoprism/modeling_videoprism.py#L548)

**Parameters:**

pixel_values_videos (`torch.FloatTensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`, *optional*) : The tensors corresponding to the input video. Pixel values for videos can be obtained using [LlavaOnevisionVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/llava_onevision#transformers.LlavaOnevisionVideoProcessor). See `LlavaOnevisionVideoProcessor.__call__()` for details ([VideoPrismProcessor](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismProcessor) uses [LlavaOnevisionVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/llava_onevision#transformers.LlavaOnevisionVideoProcessor) for processing videos).

interpolate_pos_encoding (`bool`, *optional*, defaults to `False`) : Whether to interpolate the pre-trained position encodings.

**Returns:** `BaseModelOutputWithSpatialAndTemporalStates` or `tuple(torch.FloatTensor)`

A `BaseModelOutputWithSpatialAndTemporalStates` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([VideoPrismConfig](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismConfig)) and inputs.

The [VideoPrismVisionModel](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismVisionModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **last_temporal_hidden_state** (`torch.FloatTensor`, *optional*) -- The last hidden state of the temporal encoder, typically of shape
  `(batch_size * num_patches, num_frames, hidden_size)`.
- **last_spatial_hidden_state** (`torch.FloatTensor`, *optional*) -- The last hidden state of the spatial encoder, typically of shape
  `(batch_size * num_frames, num_patches, hidden_size)`.

Example:

```python
```

## VideoPrismVideoModel[[transformers.VideoPrismVideoModel]]

#### transformers.VideoPrismVideoModel[[transformers.VideoPrismVideoModel]]

```python
transformers.VideoPrismVideoModel(config: VideoPrismVisionConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/videoprism/modeling_videoprism.py#L716)

**Parameters:**

config ([VideoPrismVisionConfig](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismVisionConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

VideoPrism video model consisting of the vision encoder backbone with auxiliary encoder layers and an attention pooling head on top. This model is used in VideoPrismClipModel.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.VideoPrismVideoModel.forward]]

```python
forward(pixel_values_videos: FloatTensor, interpolate_pos_encoding: bool | None = False, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/videoprism/modeling_videoprism.py#L733)

**Parameters:**

pixel_values_videos (`torch.FloatTensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`) : The tensors corresponding to the input video. Pixel values for videos can be obtained using [LlavaOnevisionVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/llava_onevision#transformers.LlavaOnevisionVideoProcessor). See `LlavaOnevisionVideoProcessor.__call__()` for details ([VideoPrismProcessor](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismProcessor) uses [LlavaOnevisionVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/llava_onevision#transformers.LlavaOnevisionVideoProcessor) for processing videos).

interpolate_pos_encoding (`bool`, *optional*, defaults to `False`) : Whether to interpolate the pre-trained position encodings.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([VideoPrismConfig](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismConfig)) and inputs.

The [VideoPrismVideoModel](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismVideoModel) forward method, overrides the `__call__` special method.

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

## VideoPrismTextModel[[transformers.VideoPrismTextModel]]

#### transformers.VideoPrismTextModel[[transformers.VideoPrismTextModel]]

```python
transformers.VideoPrismTextModel(config: VideoPrismTextConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/videoprism/modeling_videoprism.py#L657)

**Parameters:**

config ([VideoPrismTextConfig](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismTextConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare VideoPrism text encoder outputting last hidden states without any specific head on top. This model is used in VideoPrismClipModel.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.VideoPrismTextModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, inputs_embeds: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/videoprism/modeling_videoprism.py#L672)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

inputs_embeds (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

position_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([VideoPrismConfig](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismConfig)) and inputs.

The [VideoPrismTextModel](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismTextModel) forward method, overrides the `__call__` special method.

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

## VideoPrismClipModel[[transformers.VideoPrismClipModel]]

#### transformers.VideoPrismClipModel[[transformers.VideoPrismClipModel]]

```python
transformers.VideoPrismClipModel(config: VideoPrismConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/videoprism/modeling_videoprism.py#L766)

**Parameters:**

config ([VideoPrismConfig](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

VideoPrism model for video-text contrastive learning. This model consists of a VideoPrismVideoModel and a VideoPrismTextModel, and computes similarity scores between video and text inputs.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.VideoPrismClipModel.forward]]

```python
forward(pixel_values_videos: FloatTensor, input_ids: Tensor, attention_mask: typing.Optional[torch.Tensor] = None, interpolate_pos_encoding: bool | None = False, temperature: float | None = None, return_loss: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/videoprism/modeling_videoprism.py#L829)

**Parameters:**

pixel_values_videos (`torch.FloatTensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`) : The tensors corresponding to the input video. Pixel values for videos can be obtained using [LlavaOnevisionVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/llava_onevision#transformers.LlavaOnevisionVideoProcessor). See `LlavaOnevisionVideoProcessor.__call__()` for details ([VideoPrismProcessor](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismProcessor) uses [LlavaOnevisionVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/llava_onevision#transformers.LlavaOnevisionVideoProcessor) for processing videos).

input_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

interpolate_pos_encoding (`bool`, *optional*, defaults to `False`) : Whether to interpolate the pre-trained position encodings.

temperature (`float`, *optional*) : A temperature scalar to scale the similarity scores. If not provided, no scaling is applied.

return_loss (`bool`, *optional*) : Whether or not to return the contrastive loss.

**Returns:** `VideoPrismClipOutput` or `tuple(torch.FloatTensor)`

A `VideoPrismClipOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([VideoPrismConfig](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismConfig)) and inputs.

The [VideoPrismClipModel](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismClipModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **logits_per_video** (`torch.FloatTensor` of shape `(video_batch_size, text_batch_size)`) -- The scaled dot product scores between `video_embeds` and `text_embeds`. This represents the video-text
  similarity scores.
- **logits_per_text** (`torch.FloatTensor` of shape `(text_batch_size, video_batch_size)`) -- The scaled dot product scores between `text_embeds` and `video_embeds`. This represents the text-video
  similarity scores.
- **video_embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim)`) -- The video embeddings obtained by applying the projection layer to the pooled output of [VideoPrismVideoModel](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismVideoModel).
- **text_embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim)`) -- The text embeddings obtained by applying the projection layer to the pooled output of [VideoPrismTextModel](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismTextModel).
- **video_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The output of [VideoPrismVideoModel](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismVideoModel).
- **text_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The output of the [VideoPrismTextModel](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismTextModel).
- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `return_loss` is `True`) -- Contrastive loss for video-text similarity.

Example:

```python
```

## VideoPrismForVideoClassification[[transformers.VideoPrismForVideoClassification]]

#### transformers.VideoPrismForVideoClassification[[transformers.VideoPrismForVideoClassification]]

```python
transformers.VideoPrismForVideoClassification(config: VideoPrismVisionConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/videoprism/modeling_videoprism.py#L896)

**Parameters:**

config ([VideoPrismVisionConfig](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismVisionConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

VideoPrism Model transformer with a video classification head on top (a linear layer on top of the attention pooler).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.VideoPrismForVideoClassification.forward]]

```python
forward(pixel_values_videos: FloatTensor, labels: typing.Optional[torch.LongTensor] = None, interpolate_pos_encoding: bool | None = False, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/videoprism/modeling_videoprism.py#L915)

**Parameters:**

pixel_values_videos (`torch.FloatTensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`) : The tensors corresponding to the input video. Pixel values for videos can be obtained using [LlavaOnevisionVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/llava_onevision#transformers.LlavaOnevisionVideoProcessor). See `LlavaOnevisionVideoProcessor.__call__()` for details ([VideoPrismProcessor](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismProcessor) uses [LlavaOnevisionVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/llava_onevision#transformers.LlavaOnevisionVideoProcessor) for processing videos).

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

interpolate_pos_encoding (`bool`, *optional*, defaults to `False`) : Whether to interpolate the pre-trained position encodings.

**Returns:** [ImageClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutput) or `tuple(torch.FloatTensor)`

A [ImageClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([VideoPrismConfig](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismConfig)) and inputs.

The [VideoPrismForVideoClassification](/docs/transformers/v5.15.1/en/model_doc/videoprism#transformers.VideoPrismForVideoClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Classification (or regression if config.num_labels==1) scores (before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each stage) of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states
  (also called feature maps) of the model at the output of each stage.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, patch_size,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
```
