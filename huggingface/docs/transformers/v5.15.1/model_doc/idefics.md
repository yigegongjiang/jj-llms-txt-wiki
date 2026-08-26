# IDEFICS

## Overview

The IDEFICS model was proposed in [OBELICS: An Open Web-Scale Filtered Dataset of Interleaved Image-Text Documents](https://huggingface.co/papers/2306.16527) by Hugo Laurençon, Lucile Saulnier, Léo Tronchon, Stas Bekman, Amanpreet Singh, Anton Lozhkov, Thomas Wang, Siddharth Karamcheti, Alexander M. Rush, Douwe Kiela, Matthieu Cord, Victor Sanh

The abstract from the paper is the following:

*Large multimodal models trained on natural documents, which interleave images and text, outperform models trained on image-text pairs on various multimodal benchmarks that require reasoning over one or multiple images to generate a text. However, the datasets used to train these models have not been released, and the collection process has not been fully specified. We introduce the OBELICS dataset, an open web-scale filtered dataset of interleaved image-text documents comprising 141 million web pages extracted from Common Crawl, 353 million associated images, and 115 billion text tokens. We describe the dataset creation process, present comprehensive filtering rules, and provide an analysis of the dataset's content. To show the viability of OBELISC, we train an 80 billion parameters vision and language model on the dataset and obtain competitive performance on various multimodal benchmarks. We release the code to reproduce the dataset along with the dataset itself.*

This model was contributed by [HuggingFaceM4](https://huggingface.co/HuggingFaceM4). The original code can be found [here](). (TODO: don't have a public link yet).

IDEFICS modeling code in Transformers is for finetuning and inferencing the pre-trained IDEFICS models.

To train a new IDEFICS model from scratch use the m4 codebase (a link will be provided once it's made public)

## IdeficsConfig[[transformers.IdeficsConfig]]

#### transformers.IdeficsConfig[[transformers.IdeficsConfig]]

```python
transformers.IdeficsConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 32000, additional_vocab_size: int = 0, hidden_size: int = 4096, intermediate_size: int = 11008, num_hidden_layers: int = 32, num_attention_heads: int = 32, dropout: float | int = 0.0, hidden_act: str = 'silu', initializer_range: float = 0.02, alpha_initializer: str = 'zeros', alphas_initializer_range: float = 0.0, alpha_type: str = 'float', rms_norm_eps: float = 1e-06, use_cache: bool = True, pad_token_id: int | None = 0, bos_token_id: int | None = 1, eos_token_id: int | list[int] | None = 2, tie_word_embeddings: bool = False, cross_layer_interval: int = 1, qk_layer_norms: bool = False, freeze_text_layers: bool = True, freeze_text_module_exceptions: list | tuple = (), freeze_lm_head: bool = False, freeze_vision_layers: bool = True, freeze_vision_module_exceptions: list | tuple = (), use_resampler: bool = False, vision_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, perceiver_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/idefics/configuration_idefics.py#L77)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `32000`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

additional_vocab_size (`int`, *optional*, defaults to 0) : Additional vocabulary size of the model, typically for the special "" token. Additional vocab tokens are always trainable whereas regular vocab tokens can be frozen or not.

hidden_size (`int`, *optional*, defaults to `4096`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `11008`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `32`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `32`) : Number of attention heads for each attention layer in the Transformer decoder.

dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The ratio for all dropout layers.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

alpha_initializer (`str`, *optional*, defaults to `"zeros"`) : Initialization type for the alphas.

alphas_initializer_range (`float`, *optional*, defaults to 0.0) : The standard deviation of the truncated_normal_initializer for initializing the alphas in the Gated Cross Attention.

alpha_type (`str`, *optional*, defaults to `"float"`) : Whether the gating alphas should be vectors or single floats.

rms_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the rms normalization layers.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

pad_token_id (`int`, *optional*, defaults to `0`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `1`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `2`) : Token id used for end-of-stream in the vocabulary.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

cross_layer_interval (`int`, *optional*, default to 1) : Interval for cross attention (from text to image) layers.

qk_layer_norms (`bool`, *optional*, defaults to `False`) : Whether to add layer norm after q and k

freeze_text_layers (`bool`, *optional*, defaults to `True`) : Whether to freeze text layers

freeze_text_module_exceptions (`bool`, *optional*, defaults to `[]`) : Exceptions to freezing text layers when `freeze_text_layers` is `True`

freeze_lm_head (`bool`, *optional*, defaults to `False`) : Whether to freeze lm head

freeze_vision_layers (`bool`, *optional*, defaults to `True`) : Whether to freeze vision layers

freeze_vision_module_exceptions (`bool`, *optional*, defaults to `[]`) : Exceptions to freezing vision layers when `freeze_vision_layers` is `True`

use_resampler (`bool`, *optional*, defaults to `False`) : Whether to use the Resampler

vision_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the vision backbone.

perceiver_config (`IdeficsPerceiverConfig`,  *optional*) : Custom perceiver config or dict

This is the configuration class to store the configuration of a IdeficsModel. It is used to instantiate a Idefics
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [HuggingFaceM4/idefics-9b](https://huggingface.co/HuggingFaceM4/idefics-9b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import IdeficsModel, IdeficsConfig

>>> # Initializing a Idefics idefics-9b style configuration
>>> configuration = IdeficsConfig()

>>> # Initializing a model from the idefics-9b style configuration
>>> model = IdeficsModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## IdeficsVisionConfig[[transformers.IdeficsVisionConfig]]

#### transformers.IdeficsVisionConfig[[transformers.IdeficsVisionConfig]]

```python
transformers.IdeficsVisionConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, embed_dim: int = 768, image_size: int | list[int] | tuple[int, int] = 224, intermediate_size: int = 5120, patch_size: int | list[int] | tuple[int, int] = 14, num_hidden_layers: int = 32, num_attention_heads: int = 16, num_channels: int = 3, hidden_act: str = 'gelu', layer_norm_eps: float = 1e-05, attention_dropout: float | int = 0.0, initializer_range: float = 0.02, initializer_factor: float = 1.0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/idefics/configuration_idefics.py#L29)

**Parameters:**

embed_dim (`int`, *optional*, defaults to `768`) : Dimensionality of the embeddings and hidden states.

image_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `224`) : The size (resolution) of each image.

intermediate_size (`int`, *optional*, defaults to `5120`) : Dimension of the MLP representations.

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `14`) : The size (resolution) of each patch.

num_hidden_layers (`int`, *optional*, defaults to `32`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer decoder.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

layer_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

initializer_factor (`float`, *optional*, defaults to `1.0`) : A factor for initializing all weight matrices (should be kept to 1, used internally for initialization testing).

This is the configuration class to store the configuration of a IdeficsModel. It is used to instantiate a Idefics
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [HuggingFaceM4/idefics-9b](https://huggingface.co/HuggingFaceM4/idefics-9b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## IdeficsPerceiverConfig[[transformers.IdeficsPerceiverConfig]]

#### transformers.IdeficsPerceiverConfig[[transformers.IdeficsPerceiverConfig]]

```python
transformers.IdeficsPerceiverConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, use_resampler: bool = False, resampler_n_latents: int = 64, resampler_depth: int = 6, resampler_n_heads: int = 16, resampler_head_dim: int = 96, qk_layer_norms_perceiver: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/idefics/configuration_idefics.py#L49)

**Parameters:**

use_resampler (`bool`, *optional*, defaults to `False`) : Whether or not to use the resampler

resampler_n_latents (`int`, *optional*, defaults to 64) : Number of latent embeddings to resample ("compress") the input sequence to (usually < 128).

resampler_depth (`int`, *optional*, defaults to 6) : Depth of the Perceiver Resampler (Transformer w/ cross attention). Should be shallow (< 3).

resampler_n_heads (`int`, *optional*, defaults to 16) : Number of heads in each Transformer block (for multi-headed self-attention).

resampler_head_dim (`int`, *optional*, defaults to 96) : Dimensionality of each head projection in the Transformer block.

qk_layer_norms_perceiver (`bool`, *optional*, defaults to `False`) : Whether or not to use qk layer norms in perceiver

This is the configuration class to store the configuration of a IdeficsModel. It is used to instantiate a Idefics
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [HuggingFaceM4/idefics-9b](https://huggingface.co/HuggingFaceM4/idefics-9b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## IdeficsModel[[transformers.IdeficsModel]]

#### transformers.IdeficsModel[[transformers.IdeficsModel]]

```python
transformers.IdeficsModel(config: IdeficsConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/idefics/modeling_idefics.py#L856)

**Parameters:**

config ([IdeficsConfig](/docs/transformers/v5.15.1/en/model_doc/idefics#transformers.IdeficsConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Idefics Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.IdeficsModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, image_encoder_embeddings: typing.Optional[torch.FloatTensor] = None, perceiver_embeddings: typing.Optional[torch.FloatTensor] = None, image_attention_mask: typing.Optional[torch.Tensor] = None, use_cache: bool | None = None, interpolate_pos_encoding: bool | None = False, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/idefics/modeling_idefics.py#L931)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [IdeficsImageProcessor](/docs/transformers/v5.15.1/en/model_doc/idefics#transformers.IdeficsImageProcessor). See `IdeficsImageProcessor.__call__()` for details ([IdeficsProcessor](/docs/transformers/v5.15.1/en/model_doc/idefics#transformers.IdeficsProcessor) uses [IdeficsImageProcessor](/docs/transformers/v5.15.1/en/model_doc/idefics#transformers.IdeficsImageProcessor) for processing images).

image_encoder_embeddings (`torch.FloatTensor`, *optional*) : The output of the image encoder.

perceiver_embeddings (`torch.FloatTensor`, *optional*) : The output of the perceiver resampler.

image_attention_mask (`torch.LongTensor`, *optional*) : The attention mask for the image encoder.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

interpolate_pos_encoding (`bool`, *optional*, defaults to `False`) : Whether to interpolate the pre-trained position encodings.

**Returns:** `IdeficsBaseModelOutputWithPast` or `tuple(torch.FloatTensor)`

A `IdeficsBaseModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([IdeficsConfig](/docs/transformers/v5.15.1/en/model_doc/idefics#transformers.IdeficsConfig)) and inputs.

The [IdeficsModel](/docs/transformers/v5.15.1/en/model_doc/idefics#transformers.IdeficsModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`~cache_utils.Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and optionally if
  `config.is_encoder_decoder=True` in the cross-attention blocks) that can be used (see `past_key_values`
  input) to speed up sequential decoding.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **image_hidden_states** (`tuple(torch.FloatTensor)`, *optional*) -- Tuple of `torch.FloatTensor` (one for the output of the image embeddings, `(batch_size, num_images,
  sequence_length, hidden_size)`.

  image_hidden_states of the model produced by the vision encoder, and optionally by the perceiver

## IdeficsForVisionText2Text[[transformers.IdeficsForVisionText2Text]]

#### transformers.IdeficsForVisionText2Text[[transformers.IdeficsForVisionText2Text]]

```python
transformers.IdeficsForVisionText2Text(config, vision_model = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/idefics/modeling_idefics.py#L1084)

#### forward[[transformers.IdeficsForVisionText2Text.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, image_encoder_embeddings: typing.Optional[torch.FloatTensor] = None, perceiver_embeddings: typing.Optional[torch.FloatTensor] = None, image_attention_mask: typing.Optional[torch.Tensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, interpolate_pos_encoding: bool | None = False, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/idefics/modeling_idefics.py#L1107)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [IdeficsImageProcessor](/docs/transformers/v5.15.1/en/model_doc/idefics#transformers.IdeficsImageProcessor). See `IdeficsImageProcessor.__call__()` for details ([IdeficsProcessor](/docs/transformers/v5.15.1/en/model_doc/idefics#transformers.IdeficsProcessor) uses [IdeficsImageProcessor](/docs/transformers/v5.15.1/en/model_doc/idefics#transformers.IdeficsImageProcessor) for processing images).

image_encoder_embeddings (`torch.FloatTensor`, *optional*) : The output of the image encoder.

perceiver_embeddings (`torch.FloatTensor`, *optional*) : The output of the perceiver resampler.

image_attention_mask (`torch.LongTensor`, *optional*) : The attention mask for the image encoder.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

interpolate_pos_encoding (`bool`, *optional*, defaults to `False`) : Whether to interpolate the pre-trained position encodings.

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** `IdeficsCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`

A `IdeficsCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([IdeficsConfig](/docs/transformers/v5.15.1/en/model_doc/idefics#transformers.IdeficsConfig)) and inputs.

The [IdeficsForVisionText2Text](/docs/transformers/v5.15.1/en/model_doc/idefics#transformers.IdeficsForVisionText2Text) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **image_hidden_states** (`tuple(torch.FloatTensor)`, *optional*) -- Tuple of `torch.FloatTensor` (one for the output of the image embeddings, `(batch_size, num_images,
  sequence_length, hidden_size)`.

  image_hidden_states of the model produced by the vision encoder, and optionally by the perceiver

Example:

```python
>>> from transformers import AutoProcessor, IdeficsForVisionText2Text

>>> model = IdeficsForVisionText2Text.from_pretrained("HuggingFaceM4/idefics-9b")
>>> processor = AutoProcessor.from_pretrained("HuggingFaceM4/idefics-9b")

>>> dogs_image_url_1 = "https://huggingface.co/datasets/hf-internal-testing/fixtures_nlvr2/raw/main/image1.jpeg"
>>> dogs_image_url_2 = "https://huggingface.co/datasets/hf-internal-testing/fixtures_nlvr2/raw/main/image2.jpeg"

>>> prompts = [
...     [
...         "User:",
...         dogs_image_url_1,
...         "Describe this image.\nAssistant: An image of two dogs.\n",
...         "User:",
...         dogs_image_url_2,
...         "Describe this image.\nAssistant:",
...     ]
... ]
>>> inputs = processor(prompts, return_tensors="pt")
>>> generate_ids = model.generate(**inputs, max_new_tokens=6)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True)
```

## IdeficsImageProcessor[[transformers.IdeficsImageProcessor]]

#### transformers.IdeficsImageProcessor[[transformers.IdeficsImageProcessor]]

```python
transformers.IdeficsImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/idefics/image_processing_idefics.py#L52)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 224, 'width': 224}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BICUBIC`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.48145466, 0.4578275, 0.40821073]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.26862954, 0.26130258, 0.27577711]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

transform (`Callable`, *kwargs*, *optional*, defaults to `None`) : A custom transform function that accepts a single image can be passed for training. For example, `torchvision.Compose` can be used to compose multiple transforms. If `None` - an inference mode is assumed - and then a preset of inference-specific transforms will be applied to the images.

image_size (`int`, *kwargs*, *optional*, defaults to `self.image_size`) : Resize to image size. This is a backward-compatible alias for `size`. When provided, it overrides `size` and sets it to `{"height": image_size, "width": image_size}`.

image_num_channels (`int`, *kwargs*, *optional*, defaults to `3`) : The number of channels of the image.

Constructs a IdeficsImageProcessor image processor.

#### preprocess[[transformers.IdeficsImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/idefics/image_processing_idefics.py#L71)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

transform (`Callable`, *kwargs*, *optional*, defaults to `None`) : A custom transform function that accepts a single image can be passed for training. For example, `torchvision.Compose` can be used to compose multiple transforms. If `None` - an inference mode is assumed - and then a preset of inference-specific transforms will be applied to the images.

image_size (`int`, *kwargs*, *optional*, defaults to `self.image_size`) : Resize to image size. This is a backward-compatible alias for `size`. When provided, it overrides `size` and sets it to `{"height": image_size, "width": image_size}`.

image_num_channels (`int`, *kwargs*, *optional*, defaults to `3`) : The number of channels of the image.

**Returns:**

`TensorType | BatchFeature`

## IdeficsImageProcessorPil[[transformers.IdeficsImageProcessorPil]]

#### transformers.IdeficsImageProcessorPil[[transformers.IdeficsImageProcessorPil]]

```python
transformers.IdeficsImageProcessorPil(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/idefics/image_processing_pil_idefics.py#L51)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 224, 'width': 224}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BICUBIC`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.48145466, 0.4578275, 0.40821073]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.26862954, 0.26130258, 0.27577711]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

transform (`Callable`, *kwargs*, *optional*, defaults to `None`) : A custom transform function that accepts a single image can be passed for training. For example, `torchvision.Compose` can be used to compose multiple transforms. If `None` - an inference mode is assumed - and then a preset of inference-specific transforms will be applied to the images.

image_size (`int`, *kwargs*, *optional*, defaults to `self.image_size`) : Resize to image size. This is a backward-compatible alias for `size`. When provided, it overrides `size` and sets it to `{"height": image_size, "width": image_size}`.

image_num_channels (`int`, *kwargs*, *optional*, defaults to `3`) : The number of channels of the image.

Constructs a IdeficsImageProcessor image processor.

#### preprocess[[transformers.IdeficsImageProcessorPil.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/idefics/image_processing_pil_idefics.py#L70)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

transform (`Callable`, *kwargs*, *optional*, defaults to `None`) : A custom transform function that accepts a single image can be passed for training. For example, `torchvision.Compose` can be used to compose multiple transforms. If `None` - an inference mode is assumed - and then a preset of inference-specific transforms will be applied to the images.

image_size (`int`, *kwargs*, *optional*, defaults to `self.image_size`) : Resize to image size. This is a backward-compatible alias for `size`. When provided, it overrides `size` and sets it to `{"height": image_size, "width": image_size}`.

image_num_channels (`int`, *kwargs*, *optional*, defaults to `3`) : The number of channels of the image.

## IdeficsProcessor[[transformers.IdeficsProcessor]]

#### transformers.IdeficsProcessor[[transformers.IdeficsProcessor]]

```python
transformers.IdeficsProcessor(image_processor, tokenizer = None, image_size = 224, add_end_of_utterance_token = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/idefics/processing_idefics.py#L146)

**Parameters:**

image_processor (`IdeficsImageProcessor`) : The image processor is a required input.

tokenizer (`LlamaTokenizer`) : The tokenizer is a required input.

image_size (`int`, *optional*, defaults to 224) : The size of the image to be processed.

add_end_of_utterance_token (`bool`, *optional*, defaults to None) : Whether to add the end of utterance token to the text.

Constructs a IdeficsProcessor which wraps a image processor and a tokenizer into a single processor.

[IdeficsProcessor](/docs/transformers/v5.15.1/en/model_doc/idefics#transformers.IdeficsProcessor) offers all the functionalities of [IdeficsImageProcessor](/docs/transformers/v5.15.1/en/model_doc/idefics#transformers.IdeficsImageProcessor) and [LlamaTokenizer](/docs/transformers/v5.15.1/en/model_doc/llama2#transformers.LlamaTokenizer). See the
[~IdeficsImageProcessor](/docs/transformers/v5.15.1/en/model_doc/idefics#transformers.IdeficsImageProcessor) and [~LlamaTokenizer](/docs/transformers/v5.15.1/en/model_doc/llama2#transformers.LlamaTokenizer) for more information.

#### __call__[[transformers.IdeficsProcessor.__call__]]

```python
__call__(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], list[typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']]], str, list[str], list[list[str]]] = None, text: str | list[str] | list[list[str]] | list[list[list[str]]] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/idefics/processing_idefics.py#L173)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], list[Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]], str, list[str], list[list[str]]]`, *optional*) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

text (`Union[str, list[str], list[list[str]], list[list[list[str]]]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

add_eos_token (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to add an end-of-sequence token at the end of the text input. When enabled, an EOS token is appended to mark the end of the text sequence, which is useful for generation tasks.

add_end_of_utterance_token (`bool`, *kwargs*, *optional*) : Whether to add an end-of-utterance token to mark the end of a user's message in conversational contexts. This token helps the model distinguish between different utterances in a multi-turn conversation and is particularly important for chat-based models.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) : Additional processing options for each modality (text, images, videos, audio). Model-specific parameters are listed above; see the TypedDict class for the complete list of supported arguments.

**Returns:** `a dict with entries`

`input_ids`, `attention_mask`, `pixel_values`, `image_attention_mask` which can be
directly passed to `model.generate`

Detailed explanation:

Each entry in `text` is either a text to be passed as is or an image that will be processed.

An image can be either an image object (`PIL.Image`) or a url from which the image can be retrieved.

When the processor encounters an image it'll inject `<fake_token_around_image><image><fake_token_around_image>`
entry into the prompt.

Example:

```python
checkpoint = "HuggingFaceM4/idefics-9b"
processor = AutoProcessor.from_pretrained(checkpoint)
url = "https://hips.hearstapps.com/hmg-prod/images/cute-photos-of-cats-in-grass-1593184777.jpg"
img = processor.image_processor.fetch_images([url])[0]

prompts = [
    "User:",
    img,
    "Describe this image.\nAssistant: An image of two kittens in grass.\n",
    "User:",
    "https://hips.hearstapps.com/hmg-prod/images/dog-puns-1581708208.jpg",
    "Describe this image.\nAssistant:",
]

inputs = processor(text=prompts, return_tensors="pt")
generated_ids = model.generate(**inputs, max_length=100)
generated_text = processor.batch_decode(generated_ids, skip_special_tokens=True)[0]
```

In this example the `prompts` will be converted into:

```
<s>User:<fake_token_around_image><image><fake_token_around_image>Describe this image.
Assistant: An image of two kittens in grass.
User:<fake_token_around_image><image><fake_token_around_image>Describe this image.
Assistant:'
```

and the two images will be massaged using `IdeficsImageProcessor.__call__()` method and placed inside the
`pixel_values` dict entry of the return value.

This example also exemplifies that images can be passed as objects or as text urls. It can be seen that the
first image is passed as object and the second one as a url.

To do training do:

```python
image_transform = transforms.Compose(
    [
        transforms.RandomResizedCrop(
            (w, h), scale=(0.9, 1.0), interpolation=transforms.InterpolationMode.BICUBIC
        ),
        transforms.ToTensor(),
        transforms.Normalize(mean=self.image_mean, std=self.image_std),
    ]
)
inputs = processor(text=prompts, transform=image_transform, return_tensors="pt")
```

In order to help debug prompt generation enable `debug=True` which will show you what's happening.
