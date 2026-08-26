# MuseGlimmerAssistant

## Overview

The MuseGlimmer model was proposed in []() by .

The abstract from the paper is the following:

Tips:

This model was contributed by [INSERT YOUR HF USERNAME HERE](https://huggingface.co/).
The original code can be found [here]().

## Usage examples

## MuseGlimmerAssistantConfig[[transformers.MuseGlimmerAssistantConfig]]

#### transformers.MuseGlimmerAssistantConfig[[transformers.MuseGlimmerAssistantConfig]]

```python
transformers.MuseGlimmerAssistantConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 6656, intermediate_size: int = 19968, num_hidden_layers: int = 5, num_attention_heads: int = 32, num_key_value_heads: int = 8, head_dim: int = 128, rms_norm_eps: float = 1e-05, rope_parameters: dict | None = None, max_position_embeddings: int = 131072, sliding_window: int = 2048, layer_types: list[str] | None = None, attention_dropout: float | int = 0, hidden_act: str = 'silu', bos_token_id: int | None = 200000, eos_token_id: int | None = 200001, pad_token_id: int | None = 200018, block_size: int = 16, mask_token_id: int = 201818, target_layer_ids: list[int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/muse_glimmer_assistant/configuration_muse_glimmer_assistant.py#L27)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `6656`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `19968`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `5`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `32`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `8`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

head_dim (`int`, *optional*, defaults to `128`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

rms_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the rms normalization layers.

rope_parameters (`dict`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

max_position_embeddings (`int`, *optional*, defaults to `131072`) : The maximum sequence length that this model might ever be used with.

sliding_window (`int`, *optional*, defaults to `2048`) : Sliding window attention window size. If `None`, no sliding window is applied.

layer_types (`list[str]`, *optional*) : A list that explicitly maps each layer index with its layer type. If not provided, it will be automatically generated based on config values.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0`) : The dropout ratio for the attention probabilities.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

bos_token_id (`int`, *optional*, defaults to `200000`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`int`, *optional*, defaults to `200001`) : Token id used for end-of-stream in the vocabulary.

pad_token_id (`int`, *optional*, defaults to `200018`) : Token id used for padding in the vocabulary.

block_size (`int`, *optional*) : The block size of noise inputs that will be denoised.

mask_token_id (`int`, *optional*) : Mask token ids used as noisey input to model.

target_layer_ids (`list[int]`, *optional*) : Zero indexed layer ids whose hidden states are concatenated as context for the model.

This is the configuration class to store the configuration of a MuseGlimmerAssistantModel. It is used to instantiate a Muse Glimmer Assistant
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [meta-models/Muse-Glimmer-30B-assistant](https://huggingface.co/meta-models/Muse-Glimmer-30B-assistant)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import MuseGlimmerAssistantConfig, MuseGlimmerAssistantModel

>>> # Initializing a Muse Glimmer Assistant config similar to `meta-models/Muse-Glimmer-30B-assistant`.
>>> configuration = MuseGlimmerAssistantConfig(text_config)

>>> # Initializing a model from the `meta-models/Muse-Glimmer-30B-assistant` configuration.
>>> model = MuseGlimmerAssistantModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## MuseGlimmerAssistantPreTrainedModel[[transformers.MuseGlimmerAssistantPreTrainedModel]]

#### transformers.MuseGlimmerAssistantPreTrainedModel[[transformers.MuseGlimmerAssistantPreTrainedModel]]

```python
transformers.MuseGlimmerAssistantPreTrainedModel(config: PreTrainedConfig, *inputs, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/muse_glimmer_assistant/modeling_muse_glimmer_assistant.py#L276)

**Parameters:**

config ([PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

## MuseGlimmerAssistantModel[[transformers.MuseGlimmerAssistantModel]]

#### transformers.MuseGlimmerAssistantModel[[transformers.MuseGlimmerAssistantModel]]

```python
transformers.MuseGlimmerAssistantModel(config: MuseGlimmerAssistantConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/muse_glimmer_assistant/modeling_muse_glimmer_assistant.py#L372)

**Parameters:**

config ([MuseGlimmerAssistantConfig](/docs/transformers/v5.15.1/en/model_doc/muse_glimmer_assistant#transformers.MuseGlimmerAssistantConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Muse Glimmer Assistant Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MuseGlimmerAssistantModel.forward]]

```python
forward(noise_embeds: FloatTensor, context_hidden_states: FloatTensor, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.DFlashCache | None = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/muse_glimmer_assistant/modeling_muse_glimmer_assistant.py#L386)

noise_embeds (`torch.FloatTensor` of shape `[batch_size, config.block_size, dim]`):
Input embedding for the last generated anchor token and mask tokens to be denoised.
context_hidden_states (`torch.FloatTensor` of shape `[batch_size, number_of_previous_accepted_tokens, dim * len(config.target_layer_ids)]`):
Context hidden states from target model's selected layer ids concatenated in the last dim.
attention_mask (`torch.Tensor` of shape `[batch_size, number_of_previous_accepted_tokens + config.block_size]`):
Similar to the usual attention_mask, but note that it has length `number_of_previous_accepted_tokens + config.block_size`,
because the Attention will first concatenate `context_hidden_states` and the hidden states derived from `noise_embeds`, so that
k/v states do not have the same length as q_states, even before the `cache.update()` call. Thus the kv_seq_len dimension of
the attention mask needs to span the additional positions.
position_ids (`torch.Tensor` of shape `[batch_size, number_of_previous_accepted_tokens + config.block_size]`):
Similar to the usual position_ids, but note that it has length `number_of_previous_accepted_tokens + config.block_size`,
because the Attention will first concatenate `context_hidden_states` and the hidden states derived from `noise_embeds`, so that
k/v states do not have the same length as q_states, even before the `cache.update()` call. Thus the `position_ids` and the derived
`position_embeddings` need to span all the additional positions.
