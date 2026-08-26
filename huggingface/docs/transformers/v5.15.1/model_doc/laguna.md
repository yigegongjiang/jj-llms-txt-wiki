# Laguna

Laguna is Poolside's mixture-of-experts language model family. The Laguna-specific
deltas vs a standard SwiGLU MoE transformer are:

- **Per-layer head counts** via `num_attention_heads_per_layer` — different decoder
  layers can have different query-head counts while sharing the same KV cache shape.
- **Sigmoid MoE router with auxiliary-loss-free load balancing**
  ([arXiv:2408.15664](https://huggingface.co/papers/2408.15664)) and optional logit
  soft-capping (`moe_router_logit_softcapping`) — router scores are the element-wise
  sigmoid of the gate logits plus a learned per-expert bias (`e_score_correction_bias`)
  that is added at selection time only.

## Usage

```python
from transformers import pipeline

pipe = pipeline(
    "text-generation",
    model="poolside/Laguna-XS.2",
    dtype="auto",
    device_map="auto",
)
print(pipe("The capital of France is", max_new_tokens=20, do_sample=False)[0]["generated_text"])
```

```python
import torch
from transformers import AutoModelForCausalLM, AutoTokenizer

model_id = "poolside/Laguna-XS.2"
tokenizer = AutoTokenizer.from_pretrained(model_id)
model = AutoModelForCausalLM.from_pretrained(
    model_id,
    dtype=torch.bfloat16,
    device_map="auto",
)

prompt = "The capital of France is"
inputs = tokenizer(prompt, return_tensors="pt").to(model.device)
generated = model.generate(**inputs, max_new_tokens=20, do_sample=False)
print(tokenizer.decode(generated[0], skip_special_tokens=True))
```

## Notes

- **Attention backends.** SDPA (default), FlashAttention-2, and flex attention are
  supported. Attention-output gating is applied outside the kernel call and
  therefore works with all backends.
- **`num_attention_heads_per_layer`.** When provided, its length must equal
  `num_hidden_layers`. Each entry must be divisible by `num_key_value_heads`.
- **`layer_types`.** Defaults to `["full_attention"] * num_hidden_layers` when left
  unset. To enable sliding-window attention, pass a list of
  `"full_attention"` / `"sliding_attention"` values.
- **`mlp_layer_types`.** Per-layer MLP type, values `"dense"` or `"sparse"`. Length must
  equal `num_hidden_layers`. Defaults to `["dense"] + ["sparse"] * (num_hidden_layers - 1)`
  (first layer dense, rest MoE) when left unset.
- **`moe_apply_router_weight_on_input=True`** is not currently supported alongside the
  fused experts kernel (`grouped_mm_experts_forward`); `validate_architecture` raises at
  config-construction time. Set it to `False` (the default).

## LagunaConfig[[transformers.LagunaConfig]]

#### transformers.LagunaConfig[[transformers.LagunaConfig]]

```python
transformers.LagunaConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 100352, hidden_size: int = 2048, intermediate_size: int = 8192, num_hidden_layers: int = 40, num_attention_heads: int = 48, num_key_value_heads: int = 8, hidden_act: str = 'silu', max_position_embeddings: int = 131072, initializer_range: float = 0.02, rms_norm_eps: float = 1e-06, use_cache: bool = True, tie_word_embeddings: bool = False, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, sliding_window: int = 512, attention_dropout: float | int = 0.0, moe_intermediate_size: int = 512, shared_expert_intermediate_size: int = 512, num_experts_per_tok: int = 8, num_experts: int = 256, output_router_logits: bool = False, router_aux_loss_coef: float = 0.001, layer_types: list[str] | None = None, pad_token_id: int | None = None, bos_token_id: int | None = None, eos_token_id: int | list[int] | None = None, head_dim: int = 128, attention_bias: bool = False, gating: bool | str = True, num_attention_heads_per_layer: list[int] | None = None, mlp_layer_types: list[str] | None = None, moe_routed_scaling_factor: float = 1.0, moe_apply_router_weight_on_input: bool = False, moe_router_logit_softcapping: float = 0.0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/laguna/configuration_laguna.py#L31)

**Parameters:**

vocab_size (*int*, *optional*, defaults to *100352*) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the *input_ids*.

hidden_size (*int*, *optional*, defaults to *2048*) : Dimension of the hidden representations.

intermediate_size (*int*, *optional*, defaults to *8192*) : Dimension of the MLP representations.

num_hidden_layers (*int*, *optional*, defaults to *40*) : Number of hidden layers in the Transformer decoder.

num_attention_heads (*int*, *optional*, defaults to *48*) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (*int*, *optional*, defaults to *8*) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If *num_key_value_heads=num_attention_heads*, the model will use Multi Head Attention (MHA), if *num_key_value_heads=1* the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to *num_attention_heads*.

hidden_act (*str*, *optional*, defaults to *silu*) : The non-linear activation function (function or string) in the decoder. For example, *"gelu"*, *"relu"*, *"silu"*, etc.

max_position_embeddings (*int*, *optional*, defaults to *131072*) : The maximum sequence length that this model might ever be used with.

initializer_range (*float*, *optional*, defaults to *0.02*) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

rms_norm_eps (*float*, *optional*, defaults to *1e-06*) : The epsilon used by the rms normalization layers.

use_cache (*bool*, *optional*, defaults to *True*) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if *config.is_decoder=True* or when the model is a decoder-only generative model.

tie_word_embeddings (*bool*, *optional*, defaults to *False*) : Whether to tie weight embeddings according to model's *tied_weights_keys* mapping.

rope_parameters (*Union[~modeling_rope_utils.RopeParameters, dict]*, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for *rope_theta* and optionally parameters used for scaling in case you want to use RoPE with longer *max_position_embeddings*.

sliding_window (*int*, *optional*, defaults to *512*) : Sliding window attention window size. If *None*, no sliding window is applied.

attention_dropout (*Union[float, int]*, *optional*, defaults to *0.0*) : The dropout ratio for the attention probabilities.

moe_intermediate_size (*int*, *optional*, defaults to *512*) : Intermediate size of the routed expert MLPs.

shared_expert_intermediate_size (*int*, *optional*, defaults to *512*) : Intermediate size of the shared expert MLPs.

num_experts_per_tok (*int*, *optional*, defaults to *8*) : Number of experts to route each token to. This is the top-k value for the token-choice routing.

num_experts (*int*, *optional*, defaults to *256*) : Number of routed experts in MoE layers. 

output_router_logits (*bool*, *optional*, defaults to *False*) : Whether or not the router logits should be returned by the model. Enabling this will also allow the model to output the auxiliary loss, including load balancing loss and router z-loss.

router_aux_loss_coef (*float*, *optional*, defaults to *0.001*) : Auxiliary load balancing loss coefficient. Used to penalize uneven expert routing in MoE models.

layer_types (*list[str]*, *optional*) : A list that explicitly maps each layer index with its layer type. If not provided, it will be automatically generated based on config values.

pad_token_id (*int*, *optional*) : Token id used for padding in the vocabulary.

bos_token_id (*int*, *optional*) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (*Union[int, list[int]]*, *optional*) : Token id used for end-of-stream in the vocabulary.

head_dim (*int*, *optional*, defaults to *128*) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

attention_bias (*bool*, *optional*, defaults to *False*) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

gating (*bool* or *str*, *optional*, defaults to *True*) : Softplus output-gate granularity. `True` or `"per-head"` applies one gate per head, broadcast across `head_dim`; `"per-element"` applies one gate per `(head, head_dim)` channel.

num_attention_heads_per_layer (*list[int]*, *optional*) : Per-layer override for `num_attention_heads`. Length must equal `num_hidden_layers`.

mlp_layer_types (*list[str]*, *optional*) : Per-layer MLP type — `"dense"` or `"sparse"`. Length must equal `num_hidden_layers`. Defaults to first layer dense, rest sparse.

moe_routed_scaling_factor (*float*, *optional*, defaults to 1.0) : Scalar applied to routed-expert output before combining with the shared-expert output.

moe_apply_router_weight_on_input (*bool*, *optional*, defaults to *False*) : Whether to apply router weights to the MoE input rather than the output. Not supported in transformers yet; `True` will raise a `NotImplementedError` for now.

moe_router_logit_softcapping (*float*, *optional*, defaults to 0.0) : Scaling factor when applying tanh softcapping on the logits of the MoE router logits.

This is the configuration class to store the configuration of a LagunaModel. It is used to instantiate a Laguna
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [poolside/laguna-XS.2](https://huggingface.co/poolside/laguna-XS.2)

Configuration objects inherit from [*PreTrainedConfig*] and can be used to control the model outputs. Read the
documentation from [*PreTrainedConfig*] for more information.

Example:

```python
>>> from transformers import LagunaModel, LagunaConfig

>>> configuration = LagunaConfig()
>>> model = LagunaModel(configuration)
>>> configuration = model.config
```

#### validate_architecture[[transformers.LagunaConfig.validate_architecture]]

```python
validate_architecture()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/laguna/configuration_laguna.py#L152)

Part of `@strict`-powered validation.

## LagunaModel[[transformers.LagunaModel]]

#### transformers.LagunaModel[[transformers.LagunaModel]]

```python
transformers.LagunaModel(config: LagunaConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/laguna/modeling_laguna.py#L507)

**Parameters:**

config ([LagunaConfig](/docs/transformers/v5.15.1/en/model_doc/laguna#transformers.LagunaConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Laguna Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.LagunaModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/laguna/modeling_laguna.py#L524)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** `MoeModelOutputWithPast` or `tuple(torch.FloatTensor)`

A `MoeModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LagunaConfig](/docs/transformers/v5.15.1/en/model_doc/laguna#transformers.LagunaConfig)) and inputs.

The [LagunaModel](/docs/transformers/v5.15.1/en/model_doc/laguna#transformers.LagunaModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and optionally if
  `config.is_encoder_decoder=True` in the cross-attention blocks) that can be used (see `past_key_values`
  input) to speed up sequential decoding.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **router_logits** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_router_probs=True` and `config.add_router_probs=True` is passed or when `config.output_router_probs=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, sequence_length, num_experts)`.

  Raw router logits (post-softmax) that are computed by MoE routers, these terms are used to compute the auxiliary
  loss for Mixture of Experts models.

## LagunaForCausalLM[[transformers.LagunaForCausalLM]]

#### transformers.LagunaForCausalLM[[transformers.LagunaForCausalLM]]

```python
transformers.LagunaForCausalLM(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/laguna/modeling_laguna.py#L673)

**Parameters:**

config ([LagunaForCausalLM](/docs/transformers/v5.15.1/en/model_doc/laguna#transformers.LagunaForCausalLM)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Laguna Model for causal language modeling.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.LagunaForCausalLM.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, output_router_logits: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/laguna/modeling_laguna.py#L692)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

output_router_logits (`bool`, *optional*) : Whether or not to return the logits of all the routers. They are useful for computing the router loss, and should not be returned during inference.

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** `MoeCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`

A `MoeCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LagunaConfig](/docs/transformers/v5.15.1/en/model_doc/laguna#transformers.LagunaConfig)) and inputs.

The [LagunaForCausalLM](/docs/transformers/v5.15.1/en/model_doc/laguna#transformers.LagunaForCausalLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **aux_loss** (`torch.FloatTensor`, *optional*, returned when `labels` is provided) -- aux_loss for the sparse modules.
- **router_logits** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_router_probs=True` and `config.add_router_probs=True` is passed or when `config.output_router_probs=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, sequence_length, num_experts)`.

  Raw router logits (post-softmax) that are computed by MoE routers, these terms are used to compute the auxiliary
  loss for Mixture of Experts models.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
```
