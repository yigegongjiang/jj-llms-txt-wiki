# Decision Transformer

## Overview

The Decision Transformer model was proposed in [Decision Transformer: Reinforcement Learning via Sequence Modeling](https://huggingface.co/papers/2106.01345)  
by Lili Chen, Kevin Lu, Aravind Rajeswaran, Kimin Lee, Aditya Grover, Michael Laskin, Pieter Abbeel, Aravind Srinivas, Igor Mordatch.

The abstract from the paper is the following:

*We introduce a framework that abstracts Reinforcement Learning (RL) as a sequence modeling problem.
This allows us to draw upon the simplicity and scalability of the Transformer architecture, and associated advances
 in language modeling such as GPT-x and BERT. In particular, we present Decision Transformer, an architecture that
 casts the problem of RL as conditional sequence modeling. Unlike prior approaches to RL that fit value functions or
 compute policy gradients, Decision Transformer simply outputs the optimal actions by leveraging a causally masked
 Transformer. By conditioning an autoregressive model on the desired return (reward), past states, and actions, our
 Decision Transformer model can generate future actions that achieve the desired return. Despite its simplicity,
 Decision Transformer matches or exceeds the performance of state-of-the-art model-free offline RL baselines on
 Atari, OpenAI Gym, and Key-to-Door tasks.*

This version of the model is for tasks where the state is a vector.

This model was contributed by [edbeeching](https://huggingface.co/edbeeching). The original code can be found [here](https://github.com/kzl/decision-transformer).

## DecisionTransformerConfig[[transformers.DecisionTransformerConfig]]

#### transformers.DecisionTransformerConfig[[transformers.DecisionTransformerConfig]]

```python
transformers.DecisionTransformerConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, state_dim: int = 17, act_dim: int = 4, hidden_size: int = 128, max_ep_len: int = 4096, action_tanh: bool = True, vocab_size: int = 1, n_positions: int = 1024, n_layer: int = 3, n_head: int = 1, n_inner: int | None = None, activation_function: str = 'relu', resid_pdrop: float | int = 0.1, embd_pdrop: float | int = 0.1, attn_pdrop: float | int = 0.1, layer_norm_epsilon: float = 1e-05, initializer_range: float = 0.02, scale_attn_weights: bool = True, use_cache: bool = True, bos_token_id: int | None = 50256, eos_token_id: int | list[int] | None = 50256, scale_attn_by_inverse_layer_idx: bool = False, reorder_and_upcast_attn: bool = False, add_cross_attention: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/decision_transformer/configuration_decision_transformer.py#L24)

**Parameters:**

state_dim (`int`, *optional*, defaults to 17) : The state size for the RL environment

act_dim (`int`, *optional*, defaults to 4) : The size of the output action space

hidden_size (`int`, *optional*, defaults to `128`) : Dimension of the hidden representations.

max_ep_len (`int`, *optional*, defaults to 4096) : The maximum length of an episode in the environment

action_tanh (`bool`, *optional*, defaults to True) : Whether to use a tanh activation on action prediction

vocab_size (`int`, *optional*, defaults to `1`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

n_positions (`int`, *optional*, defaults to `1024`) : The maximum sequence length that this model might ever be used with.

n_layer (`int`, *optional*, defaults to `3`) : Number of hidden layers in the Transformer decoder.

n_head (`int`, *optional*, defaults to `1`) : Number of attention heads for each attention layer in the Transformer decoder.

n_inner (`int`, *optional*) : Dimension of the MLP representations.

activation_function (`str`, *optional*, defaults to `relu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

resid_pdrop (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

embd_pdrop (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout ratio for the embeddings.

attn_pdrop (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout ratio for the attention probabilities.

layer_norm_epsilon (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

scale_attn_weights (`bool`, *optional*, defaults to `True`) : Scale attention weights by dividing by sqrt(hidden_size)..

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

bos_token_id (`int`, *optional*, defaults to `50256`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `50256`) : Token id used for end-of-stream in the vocabulary.

scale_attn_by_inverse_layer_idx (`bool`, *optional*, defaults to `False`) : Whether to additionally scale attention weights by `1 / layer_idx + 1`.

reorder_and_upcast_attn (`bool`, *optional*, defaults to `False`) : Whether to scale keys (K) prior to computing attention (dot-product) and upcast attention dot-product/softmax to float() when training with mixed precision.

add_cross_attention (`bool`, *optional*, defaults to `False`) : Whether cross-attention layers should be added to the model.

This is the configuration class to store the configuration of a DecisionTransformerModel. It is used to instantiate a Decision Transformer
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [](https://huggingface.co/)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import DecisionTransformerConfig, DecisionTransformerModel

>>> # Initializing a DecisionTransformer configuration
>>> configuration = DecisionTransformerConfig()

>>> # Initializing a model (with random weights) from the configuration
>>> model = DecisionTransformerModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## DecisionTransformerGPT2Model[[transformers.DecisionTransformerGPT2Model]]

#### transformers.DecisionTransformerGPT2Model[[transformers.DecisionTransformerGPT2Model]]

```python
transformers.DecisionTransformerGPT2Model(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/decision_transformer/modeling_decision_transformer.py#L344)

#### forward[[transformers.DecisionTransformerGPT2Model.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, attention_mask: typing.Optional[torch.FloatTensor] = None, token_type_ids: typing.Optional[torch.LongTensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, encoder_hidden_states: typing.Optional[torch.Tensor] = None, encoder_attention_mask: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/decision_transformer/modeling_decision_transformer.py#L370)

## DecisionTransformerModel[[transformers.DecisionTransformerModel]]

#### transformers.DecisionTransformerModel[[transformers.DecisionTransformerModel]]

```python
transformers.DecisionTransformerModel(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/decision_transformer/modeling_decision_transformer.py#L502)

**Parameters:**

config ([DecisionTransformerModel](/docs/transformers/v5.15.1/en/model_doc/decision_transformer#transformers.DecisionTransformerModel)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Decision Transformer Model

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.DecisionTransformerModel.forward]]

```python
forward(states: typing.Optional[torch.FloatTensor] = None, actions: typing.Optional[torch.FloatTensor] = None, rewards: typing.Optional[torch.FloatTensor] = None, returns_to_go: typing.Optional[torch.FloatTensor] = None, timesteps: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.FloatTensor] = None, output_hidden_states: bool | None = None, output_attentions: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/decision_transformer/modeling_decision_transformer.py#L535)

**Parameters:**

states (`torch.FloatTensor` of shape `(batch_size, episode_length, state_dim)`) : The states for each step in the trajectory

actions (`torch.FloatTensor` of shape `(batch_size, episode_length, act_dim)`) : The actions taken by the "expert" policy for the current state, these are masked for auto regressive prediction

rewards (`torch.FloatTensor` of shape `(batch_size, episode_length, 1)`) : The rewards for each state, action

returns_to_go (`torch.FloatTensor` of shape `(batch_size, episode_length, 1)`) : The returns for each state in the trajectory

timesteps (`torch.LongTensor` of shape `(batch_size, episode_length)`) : The timestep for each step in the trajectory

attention_mask (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `DecisionTransformerOutput` or `tuple(torch.FloatTensor)`

A `DecisionTransformerOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DecisionTransformerConfig](/docs/transformers/v5.15.1/en/model_doc/decision_transformer#transformers.DecisionTransformerConfig)) and inputs.

The [DecisionTransformerModel](/docs/transformers/v5.15.1/en/model_doc/decision_transformer#transformers.DecisionTransformerModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **state_preds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, state_dim)`) -- Environment state predictions
- **action_preds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, action_dim)`) -- Model action predictions
- **return_preds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, 1)`) -- Predicted returns for each state
- **hidden_states** (`torch.FloatTensor`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`torch.FloatTensor`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the model.

Examples:

```python
>>> from transformers import DecisionTransformerModel
>>> import torch

>>> model = DecisionTransformerModel.from_pretrained("edbeeching/decision-transformer-gym-hopper-medium")
>>> # evaluation
>>> model = model.to(device)
>>> model.eval()

>>> env = gym.make("Hopper-v3")
>>> state_dim = env.observation_space.shape[0]
>>> act_dim = env.action_space.shape[0]

>>> state = env.reset()
>>> states = torch.from_numpy(state).reshape(1, 1, state_dim).to(device=device, dtype=torch.float32)
>>> actions = torch.zeros((1, 1, act_dim), device=device, dtype=torch.float32)
>>> rewards = torch.zeros(1, 1, device=device, dtype=torch.float32)
>>> target_return = torch.tensor(TARGET_RETURN, dtype=torch.float32).reshape(1, 1)
>>> timesteps = torch.tensor(0, device=device, dtype=torch.long).reshape(1, 1)
>>> attention_mask = torch.zeros(1, 1, device=device, dtype=torch.float32)

>>> # forward pass
>>> with torch.no_grad():
...     state_preds, action_preds, return_preds = model(
...         states=states,
...         actions=actions,
...         rewards=rewards,
...         returns_to_go=target_return,
...         timesteps=timesteps,
...         attention_mask=attention_mask,
...         return_dict=False,
...     )
```
