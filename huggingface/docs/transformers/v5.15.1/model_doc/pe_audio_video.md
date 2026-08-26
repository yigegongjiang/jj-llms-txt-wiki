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

- [PeAudioVideoModel](/docs/transformers/v5.15.1/en/model_doc/pe_audio_video#transformers.PeAudioVideoModel) requires at least two of `input_ids`, `input_values`, `pixel_values_videos` — if only two are provided it dispatches to the audio-only or video-only sub-model. Passing all three triggers the joint audio-video-text path and the full set of logit matrices in `PeAudioVideoOutput`.
- Audio uses `padding_mask` and video uses `padding_mask_videos` simultaneously. They are independent masks; do not conflate them with `attention_mask`, which is reserved for the text tower.
- Audio–video alignment runs per-batch-element inside `_align_video_hidden_state`, so batches with very different audio/video lengths iterate rather than vectorizing. Keep batch items roughly balanced for throughput.
- The text tower's weights are tied across branches via `_tied_weights_keys` — do not try to load separate text encoders for the audio and video halves.

## PeAudioVideoConfig[[transformers.PeAudioVideoConfig]]

#### transformers.PeAudioVideoConfig[[transformers.PeAudioVideoConfig]]

```python
transformers.PeAudioVideoConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, audio_video_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, tie_word_embeddings: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_audio_video/configuration_pe_audio_video.py#L89)

**Parameters:**

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

audio_video_config (`dict` or `PreTrainedConfig`, *optional*) : Configuration for the audio-video encoder component.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a PeAudioVideoModel. It is used to instantiate a Pe Audio Video
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/pe-av-large](https://huggingface.co/facebook/pe-av-large)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

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

#### transformers.PeAudioVideoEncoderConfig[[transformers.PeAudioVideoEncoderConfig]]

```python
transformers.PeAudioVideoEncoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, audio_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, video_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, hidden_size: int = 1792, intermediate_size: int = 4800, num_hidden_layers: int = 6, num_attention_heads: int = 14, num_key_value_heads: int | None = None, head_dim: int = 128, hidden_act: str = 'silu', max_position_embeddings: int = 10000, initializer_range: float = 0.02, rms_norm_eps: float = 1e-05, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, attention_bias: bool = False, attention_dropout: float | int = 0.0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_audio_video/configuration_pe_audio_video.py#L26)

**Parameters:**

audio_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the audio backbone.

video_config (`Union[PreTrainedConfig, dict]`, *optional*) : Configuration for the video encoder. If a dictionary is provided, it is used to instantiate [PeVideoEncoderConfig](/docs/transformers/v5.15.1/en/model_doc/pe_video#transformers.PeVideoEncoderConfig).

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

This is the configuration class to store the configuration of a PeAudioVideoModel. It is used to instantiate a Pe Audio Video
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/pe-av-large](https://huggingface.co/facebook/pe-av-large)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

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

#### transformers.PeAudioVideoProcessor[[transformers.PeAudioVideoProcessor]]

```python
transformers.PeAudioVideoProcessor(feature_extractor = None, video_processor = None, tokenizer = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_audio_video/processing_pe_audio_video.py#L17)

## PeAudioVideoEncoder[[transformers.PeAudioVideoEncoder]]

#### transformers.PeAudioVideoEncoder[[transformers.PeAudioVideoEncoder]]

```python
transformers.PeAudioVideoEncoder(config: PeAudioVideoEncoderConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_audio_video/modeling_pe_audio_video.py#L564)

**Parameters:**

config ([PeAudioVideoEncoderConfig](/docs/transformers/v5.15.1/en/model_doc/pe_audio_video#transformers.PeAudioVideoEncoderConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The PeAudioVideo Encoder model.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.PeAudioVideoEncoder.forward]]

```python
forward(input_values: typing.Optional[torch.Tensor] = None, pixel_values_videos: typing.Optional[torch.Tensor] = None, padding_mask: typing.Optional[torch.Tensor] = None, padding_mask_videos: typing.Optional[torch.Tensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_audio_video/modeling_pe_audio_video.py#L583)

**Parameters:**

input_values (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Float values of input raw speech waveform. Values can be obtained by loading a `.flac` or `.wav` audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_values`, the [AutoProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoProcessor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`. See [PeAudioVideoProcessor.__call__()](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessorMixin.__call__) for details.

pixel_values_videos (`torch.Tensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`, *optional*) : The tensors corresponding to the input video. Pixel values for videos can be obtained using [PeVideoVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/pe_video#transformers.PeVideoVideoProcessor). See `PeVideoVideoProcessor.__call__()` for details ([PeAudioVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/pe_audio_video#transformers.PeAudioVideoProcessor) uses [PeVideoVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/pe_video#transformers.PeVideoVideoProcessor) for processing videos).

padding_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding samples of `input_values`. Mask values selected in `[0, 1]`:  - 1 for samples that are **not masked**, - 0 for samples that are **masked**.

padding_mask_videos (`torch.Tensor` of shape `(batch_size, num_frames)`, *optional*) : Mask to avoid performing attention on padding video frames. Mask values selected in `[0, 1]`:  - 1 for frames that are **not masked**, - 0 for frames that are **masked**.

**Returns:** `PeAudioVideoEncoderOutput` or `tuple(torch.FloatTensor)`

A `PeAudioVideoEncoderOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PeAudioVideoConfig](/docs/transformers/v5.15.1/en/model_doc/pe_audio_video#transformers.PeAudioVideoConfig)) and inputs.

The [PeAudioVideoEncoder](/docs/transformers/v5.15.1/en/model_doc/pe_audio_video#transformers.PeAudioVideoEncoder) forward method, overrides the `__call__` special method.

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
- **audio_model_output** (`BaseModelOutputWithPooling`, *optional*) -- Output of the audio encoder, containing the last hidden state, pooled output, and optional hidden states
  and attentions. See [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) for details.
- **video_model_output** (`BaseModelOutputWithPooling`, *optional*) -- Output of the video encoder, containing the last hidden state, pooled output, and optional hidden states
  and attentions. See [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) for details.

## PeAudioVideoModel[[transformers.PeAudioVideoModel]]

#### transformers.PeAudioVideoModel[[transformers.PeAudioVideoModel]]

```python
transformers.PeAudioVideoModel(config: PeAudioVideoConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_audio_video/modeling_pe_audio_video.py#L748)

#### forward[[transformers.PeAudioVideoModel.forward]]

```python
forward(input_ids: typing.Optional[torch.Tensor] = None, pixel_values_videos: typing.Optional[torch.Tensor] = None, input_values: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, padding_mask_videos: typing.Optional[torch.Tensor] = None, padding_mask: typing.Optional[torch.Tensor] = None, return_loss = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pe_audio_video/modeling_pe_audio_video.py#L889)

**Parameters:**

input_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

pixel_values_videos (`torch.Tensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`, *optional*) : The tensors corresponding to the input video. Pixel values for videos can be obtained using [PeVideoVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/pe_video#transformers.PeVideoVideoProcessor). See `PeVideoVideoProcessor.__call__()` for details ([PeAudioVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/pe_audio_video#transformers.PeAudioVideoProcessor) uses [PeVideoVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/pe_video#transformers.PeVideoVideoProcessor) for processing videos).

input_values (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Float values of input raw speech waveform. Values can be obtained by loading a `.flac` or `.wav` audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_values`, the [AutoProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoProcessor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`. See [PeAudioVideoProcessor.__call__()](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessorMixin.__call__) for details.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

padding_mask_videos (`torch.Tensor` of shape `(batch_size, num_frames)`, *optional*) : Mask to avoid performing attention on padding video frames. Mask values selected in `[0, 1]`:  - 1 for frames that are **not masked**, - 0 for frames that are **masked**.

padding_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding samples of `input_values`. Mask values selected in `[0, 1]`:  - 1 for samples that are **not masked**, - 0 for samples that are **masked**.

return_loss (`bool`, *optional*) : Whether or not to return the loss.

**Returns:** `PeAudioVideoOutput` or `tuple(torch.FloatTensor)`

A `PeAudioVideoOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PeAudioVideoConfig](/docs/transformers/v5.15.1/en/model_doc/pe_audio_video#transformers.PeAudioVideoConfig)) and inputs.

The [PeAudioVideoModel](/docs/transformers/v5.15.1/en/model_doc/pe_audio_video#transformers.PeAudioVideoModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **audio_embeds** (`torch.FloatTensor`, *optional*) -- Audio modality embeddings. Shape `(batch_size, sequence_length, hidden_size)`.
- **video_embeds** (`torch.FloatTensor`, *optional*) -- Video modality embeddings. Shape `(batch_size, sequence_length, hidden_size)`.
- **audio_video_embeds** (`torch.FloatTensor`, *optional*) -- Joint audio-video embeddings produced by a fusion module. Shape `(batch_size, sequence_length, hidden_size)`.
- **text_audio_embeds** (`torch.FloatTensor`, *optional*) -- Joint text-audio embeddings. Shape `(batch_size, sequence_length, hidden_size)`.
- **text_video_embeds** (`torch.FloatTensor`, *optional*) -- Joint text-video embeddings. Shape `(batch_size, sequence_length, hidden_size)`.
- **text_audio_video_embeds** (`torch.FloatTensor`, *optional*) -- Joint text-audio-video embeddings combining all three modalities. Shape `(batch_size, sequence_length, hidden_size)`.
- **audio_plus_text_embeds** (`torch.FloatTensor`, *optional*) -- Combined audio and text embeddings (e.g., concatenation or additive fusion). Shape `(batch_size, sequence_length, hidden_size)`.
- **video_plus_text_embeds** (`torch.FloatTensor`, *optional*) -- Combined video and text embeddings. Shape `(batch_size, sequence_length, hidden_size)`.
- **text_outputs** (`MaskedLMOutput`, *optional*) -- Model outputs for the text encoder. Includes hidden states, attentions, and optionally loss.
- **audio_outputs** (`BaseModelOutputWithPooling`, *optional*) -- Model outputs for the audio encoder, including last hidden state and pooled output.
- **video_outputs** (`BaseModelOutputWithPooling`, *optional*) -- Model outputs for the video encoder, including last hidden state and pooled output.
- **audio_video_outputs** (`BaseModelOutputWithPooling`, *optional*) -- Model outputs for the joint audio-video encoder.
- **logits_audio_text** (`torch.FloatTensor`, *optional*) -- Similarity logits between audio and text embeddings. Shape `(batch_size, batch_size)`.
- **logits_video_text** (`torch.FloatTensor`, *optional*) -- Similarity logits between video and text embeddings. Shape `(batch_size, batch_size)`.
- **logits_audio_video** (`torch.FloatTensor`, *optional*) -- Similarity logits between audio and video embeddings. Shape `(batch_size, batch_size)`.
- **logits_audio_video_text** (`torch.FloatTensor`, *optional*) -- Similarity logits across audio, video, and text modalities.
- **logits_audio_plus_text_video** (`torch.FloatTensor`, *optional*) -- Similarity logits between fused (audio + text) embeddings and video embeddings.
- **logits_video_plus_text_audio** (`torch.FloatTensor`, *optional*) -- Similarity logits between fused (video + text) embeddings and audio embeddings.
- **audio_text_loss** (`torch.FloatTensor`, *optional*) -- Contrastive loss computed between audio and text representations.
- **video_text_loss** (`torch.FloatTensor`, *optional*) -- Contrastive loss computed between video and text representations.
- **audio_video_loss** (`torch.FloatTensor`, *optional*) -- Contrastive loss computed between audio and video representations.
- **audio_video_text_loss** (`torch.FloatTensor`, *optional*) -- Joint loss over audio, video, and text modalities.
- **audio_plus_text_video_loss** (`torch.FloatTensor`, *optional*) -- Loss between fused (audio + text) representations and video.
- **video_plus_text_audio_loss** (`torch.FloatTensor`, *optional*) -- Loss between fused (video + text) representations and audio.
- **loss** (`torch.FloatTensor`, *optional*) -- Combined loss for all modality-wise losses.
