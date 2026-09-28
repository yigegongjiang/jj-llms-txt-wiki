# Hy4-Preview

## Overview

Hy4-Preview is a 780B-parameter mixture-of-experts language model that activates 49B parameters per
token. Each MoE layer holds 256 routed experts plus one always-active shared expert and routes every
token to 8 of them. The context window is 1M tokens.

The architecture combines four features:

- **Multi-head Latent Attention (MLA)** compresses keys and values into a low-rank latent
  (`kv_lora_rank`) that `kv_b_proj` expands back to one key/value per query head.
- **DeepSeek Sparse Attention (DSA)** selects `index_topk` keys per query with a lightweight indexer.
  Following [IndexShare](https://huggingface.co/papers/2603.12201), only the layers marked `"full"`
  in `indexer_types` run an indexer; `"shared"` layers reuse the previous full layer's selection.
- **Gated MLA with learnable attention sinks**, where each head owns a sink logit that participates
  in the softmax and contributes no value, as in [GPT-OSS](./gpt_oss).
- **Independent Hyper-Connections (iHC)** replace the plain residual path with `hc_mult` parallel
  residual streams that are collapsed before, and redistributed after, every sublayer.

The implementation does not execute the multi-token prediction (MTP) layers. Released checkpoints
keep those weights so that other runtimes can use them for speculative decoding; they are ignored
at load time.

## Usage example

```python
import torch
from transformers import AutoModelForCausalLM, AutoTokenizer

model_id = "tencent/Hy4-Preview"
tokenizer = AutoTokenizer.from_pretrained(model_id)
model = AutoModelForCausalLM.from_pretrained(model_id, dtype=torch.bfloat16, device_map="auto")

messages = [{"role": "user", "content": "Explain in one sentence why the sky is usually blue."}]
inputs = tokenizer.apply_chat_template(
    messages, add_generation_prompt=True, return_tensors="pt", return_dict=True
).to(model.device)

outputs = model.generate(**inputs, max_new_tokens=64, do_sample=False)
print(tokenizer.decode(outputs[0, inputs["input_ids"].shape[1]:], skip_special_tokens=True))
```

The full checkpoint does not fit on a single accelerator. Shard it with tensor parallelism, or place
each expert group on its own device with expert parallelism:

```python
from transformers import AutoModelForCausalLM, DistributedConfig

# Tensor parallel: launch with `torchrun --nproc-per-node <world_size>`.
model = AutoModelForCausalLM.from_pretrained(
    model_id, dtype=torch.bfloat16, distributed_config=DistributedConfig(tp_size=16)
)

# Expert parallel: routed experts are split along the expert axis.
model = AutoModelForCausalLM.from_pretrained(
    model_id,
    dtype=torch.bfloat16,
    distributed_config=DistributedConfig(tp_size=16, enable_expert_parallel=True),
)
```

Expert parallelism is inference-only, because the routed-expert all-reduce has no backward pass.

## HYV4Config[[transformers.HYV4Config]]

#### transformers.HYV4Config[[transformers.HYV4Config]]

```python
transformers.HYV4Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 120832, hidden_size: int = 2816, intermediate_size: int = 6912, moe_intermediate_size: int = 768, num_hidden_layers: int = 34, num_attention_heads: int = 32, num_key_value_heads: int = 32, head_dim: int = 256, hidden_act: str = 'silu', max_position_embeddings: int = 262144, initializer_range: float = 0.006, rms_norm_eps: float = 1e-05, use_cache: bool = True, pad_token_id: int | None = 120002, bos_token_id: int | None = 120000, eos_token_id: int | list[int] | None = 120025, tie_word_embeddings: bool = False, attention_bias: bool = False, attention_dropout: float = 0.0, n_routed_experts: int = 256, n_shared_experts: int = 1, num_experts_per_tok: int = 8, routed_scaling_factor: float = 2.827, norm_topk_prob: bool = True, n_group: int = 1, topk_group: int = 1, q_lora_rank: int = 1536, kv_lora_rank: int = 512, qk_nope_head_dim: int = 192, qk_rope_head_dim: int = 64, v_head_dim: int = 256, mlp_layer_types: list[str] | None = None, layer_types: list[str] | None = None, index_topk: int = 2048, index_head_dim: int = 128, index_n_heads: int = 16, indexer_types: list[str] | None = None, hc_mult: int = 4, hc_magnitude: float = 2.0, hc_eps: float = 1e-06, learnable_sink_init: float = 0.0, swiglu_limit: float = 10.0, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/hy_v4/configuration_hy_v4.py#L29)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `120832`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `2816`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `6912`) : Dimension of the MLP representations.

moe_intermediate_size (`int`, *optional*, defaults to `768`) : Intermediate size of the routed expert MLPs.

num_hidden_layers (`int`, *optional*, defaults to `34`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `32`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `32`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

head_dim (`int`, *optional*, defaults to `256`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `262144`) : The maximum sequence length that this model might ever be used with.

initializer_range (`float`, *optional*, defaults to `0.006`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

rms_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the rms normalization layers.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

pad_token_id (`int`, *optional*, defaults to `120002`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `120000`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `120025`) : Token id used for end-of-stream in the vocabulary.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

attention_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

attention_dropout (`float`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

n_routed_experts (`int`, *optional*, defaults to `256`) : Number of routed experts.

n_shared_experts (`int`, *optional*, defaults to `1`) : Number of shared experts.

num_experts_per_tok (`int`, *optional*, defaults to `8`) : Number of experts to route each token to. This is the top-k value for the token-choice routing.

routed_scaling_factor (`float`, *optional*, defaults to `2.827`) : Scaling factor or routed experts.

norm_topk_prob (`bool`, *optional*, defaults to `True`) : Whether to normalize the weights of the routed experts. 

n_group (`int`, *optional*, defaults to 1) : Number of expert groups for routing. HYV4 selects experts globally, so this is 1 (one group holding every expert).

topk_group (`int`, *optional*, defaults to 1) : Number of expert groups kept during routing. With `n_group=1` this reuses `Glm4MoeLiteTopkRouter` as a plain global top-k.

q_lora_rank (`int`, *optional*, defaults to `1536`) : Rank of the LoRA matrices for query projections.

kv_lora_rank (`int`, *optional*, defaults to `512`) : Rank of the LoRA matrices for key and value projections.

qk_nope_head_dim (`int`, *optional*, defaults to `192`) : Dimension of the query/key heads that don't use rotary position embeddings.

qk_rope_head_dim (`int`, *optional*, defaults to `64`) : Dimension of the query/key heads that use rotary position embeddings.

v_head_dim (`int`, *optional*, defaults to `256`) : Dimension of the value heads.

mlp_layer_types (`list[str]`, *optional*) : Per-layer MLP type, either `"dense"` or `"sparse"`. Defaults to one dense layer followed by sparse MoE layers.

layer_types (`list[str]`, *optional*) : A list that explicitly maps each layer index with its layer type. If not provided, it will be automatically generated based on config values.

index_topk (`int`, *optional*, defaults to 2048) : Maximum number of key positions selected by each DSA query.

index_head_dim (`int`, *optional*, defaults to 128) : Hidden dimension of each DSA indexer head.

index_n_heads (`int`, *optional*, defaults to 16) : Number of DSA indexer heads.

indexer_types (`list[str]`, *optional*) : Per-layer DSA indexer type, either `"full"` or `"shared"`. A shared layer reuses the most recent full indexer in the same forward request.

hc_mult (`int`, *optional*, defaults to 4) : Number of hidden-state channels maintained by iHC.

hc_magnitude (`float`, *optional*, defaults to 2.0) : Scale applied to the iHC post-gating branch.

hc_eps (`float`, *optional*, defaults to 1e-6) : Numerical epsilon added to iHC sigmoid gates.

learnable_sink_init (`float`, *optional*, defaults to 0.0) : Initial value of each learned attention-sink logit.

swiglu_limit (`float`, *optional*, defaults to 10.0) : Magnitude of the routed-expert SwiGLU clamp. Values at or below zero disable the clamp.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

This is the configuration class to store the configuration of a HYV4Model. It is used to instantiate a Hy V4
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [tencent/Hy4-preview](https://huggingface.co/tencent/Hy4-preview)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## HYV4Model[[transformers.HYV4Model]]

#### transformers.HYV4Model[[transformers.HYV4Model]]

```python
transformers.HYV4Model(config: HYV4Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/hy_v4/modeling_hy_v4.py#L806)

**Parameters:**

config ([HYV4Config](/docs/transformers/v5.17.0/en/model_doc/hy_v4#transformers.HYV4Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Hy V4 Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.HYV4Model.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/hy_v4/modeling_hy_v4.py#L824)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.17.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** [BaseModelOutputWithPast](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPast](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([HYV4Config](/docs/transformers/v5.17.0/en/model_doc/hy_v4#transformers.HYV4Config)) and inputs.

The [HYV4Model](/docs/transformers/v5.17.0/en/model_doc/hy_v4#transformers.HYV4Model) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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

## HYV4ForCausalLM[[transformers.HYV4ForCausalLM]]

#### transformers.HYV4ForCausalLM[[transformers.HYV4ForCausalLM]]

```python
transformers.HYV4ForCausalLM(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/hy_v4/modeling_hy_v4.py#L891)

**Parameters:**

config ([HYV4ForCausalLM](/docs/transformers/v5.17.0/en/model_doc/hy_v4#transformers.HYV4ForCausalLM)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Hy V4 Model for causal language modeling.

This model inherits from [PreTrainedModel](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.HYV4ForCausalLM.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/hy_v4/modeling_hy_v4.py#L920)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.17.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** [CausalLMOutputWithPast](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or `tuple(torch.FloatTensor)`

A [CausalLMOutputWithPast](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([HYV4Config](/docs/transformers/v5.17.0/en/model_doc/hy_v4#transformers.HYV4Config)) and inputs.

The [HYV4ForCausalLM](/docs/transformers/v5.17.0/en/model_doc/hy_v4#transformers.HYV4ForCausalLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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
>>> from transformers import AutoTokenizer, HYV4ForCausalLM

>>> model = HYV4ForCausalLM.from_pretrained("meta-hy_v4/HYV4-2-7b-hf")
>>> tokenizer = AutoTokenizer.from_pretrained("meta-hy_v4/HYV4-2-7b-hf")

>>> prompt = "Hey, are you conscious? Can you talk to me?"
>>> inputs = tokenizer(prompt, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(inputs.input_ids, max_length=30)
>>> tokenizer.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"Hey, are you conscious? Can you talk to me?\nI'm not conscious, but I can talk to you."
```
