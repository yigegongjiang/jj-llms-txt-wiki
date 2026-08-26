# DeepSeek-V4

[DeepSeek-V4](https://huggingface.co/deepseek-ai) is the next-generation MoE language model from DeepSeek
([paper](https://huggingface.co/deepseek-ai/DeepSeek-V4-Flash/blob/main/DeepSeek_V4.pdf)). The architecture replaces
DeepSeek-V3's Multi-head Latent Attention (MLA) with a hybrid local + long-range design, swaps residual connections
for Manifold-Constrained Hyper-Connections (mHC), and bootstraps the first few MoE layers with a static
token-id → expert-id hash table.

This implementation covers `DeepSeek-V4-Flash`, `DeepSeek-V4-Pro`, and their `-Base` pretrained siblings. All four
share the same architecture; they differ only in width / depth / expert count and weights.

## Architecture (paper §2)

### Hybrid attention (§2.3)

Each decoder block is one of three attention types, dispatched by `config.layer_types[i]`:

* **Sliding-window full attention** (`"sliding_attention"`): only the local window of `sliding_window` tokens, no
  long-range branch. Matches V3's "Full Attention" style for the bootstrap layers.
* **Compressed Sparse Attention** (`"compressed_sparse_attention"`, **CSA** — paper §2.3.1): a low-compression
  pool (`compress_rate_csa`, default `m=4`) with overlapping windows, plus a **Lightning Indexer** (eqs. 13–17)
  that scores queries against the pool and gathers the top `index_topk` blocks per query before they reach core
  attention.
* **Heavily Compressed Attention** (`"heavily_compressed_attention"`, **HCA** — paper §2.3.2): a high-compression
  pool (`compress_rate_hca`, default `m'=128`) with non-overlapping windows. No indexer — every pooled entry
  contributes to attention.

All three types share the same backbone:

* **Shared K=V Multi-Query Attention**: `num_key_value_heads = 1`; `kv_proj` produces a single KV head and the same
  tensor is read as both key and value.
* **Partial RoPE** (interleaved-pair, paper §2.3.3 "Partial Rotary Positional Embedding") on the trailing
  `qk_rope_head_dim = head_dim * partial_rotary_factor` channels of each head. The same rotation is applied with
  position `-i` to the attention output's rope slice (eq. 26) so the contribution of each KV entry stays a function
  of the *relative* distance to the query.
* **Per-head learnable attention sink** (eq. 27).
* **Grouped low-rank output projection** (§2.3.1 "Grouped Output Projection"): `o_groups` head-groups → `o_lora_rank`
  per group → `hidden_size`, computed by `DeepseekV4GroupedLinear` (`o_a_proj`) followed by `o_b_proj`. Cuts the
  per-token cost of the wide attention output without losing expressivity.
* **Shared sliding-window K=V branch** of size `sliding_window` ("Additional Branch of Sliding Window Attention",
  §2.3.1) preserves local fine-grained dependencies; the long-range compressor's output is concatenated with this
  branch's KVs before core attention.

### Manifold-Constrained Hyper-Connections (§2.2)

Residual connections are replaced by mHC (Xie et al., 2026): `hc_mult` parallel residual streams kept in shape
`[B, S, hc_mult, D]` throughout each block. Two `DeepseekV4HyperConnection` modules — `attn_hc` and `ffn_hc` — mix
streams in and out around the attention / MLP sublayers via a `(pre, post, comb)` triplet. The `comb` matrix is a
doubly-stochastic projection produced by `hc_sinkhorn_iters` Sinkhorn–Knopp iterations on the manifold, making
signal propagation non-expansive across deep stacks. A final `DeepseekV4HyperHead` collapses the `hc_mult`
streams down to a single sequence before the model norm.

### MoE schedule (§2.1)

Routing is configured per layer by `config.mlp_layer_types`, with values from `{"hash_moe", "moe"}`:

* `"hash_moe"`: expert indices come from a frozen `tid2eid[input_ids]` lookup populated from the V4 checkpoint.
  The learned gate `weight` still produces the per-expert scores that weight the selected experts; only
  *which-experts* is static. Used for the first few bootstrap layers (default 3, override via legacy
  `num_hash_layers`).
* `"moe"`: standard top-k routed MoE. The expert affinity uses **Sqrt(Softplus(·))** instead of V3's Sigmoid
  ("we change the activation function that computes the affinity scores from Sigmoid(·) into Sqrt(Softplus(·))",
  paper §2.1), and V3's `n_group` / `topk_group` constraint is dropped. The auxiliary-loss-free strategy
  (DeepSeek's `noaux_tc`) is preserved via the `e_score_correction_bias` buffer that biases the top-k argmax
  without flowing gradients.

Routed experts use a **clamped SwiGLU** (`gate.clamp(max=swiglu_limit)`, `up.clamp(min=-swiglu_limit, max=swiglu_limit)`,
then `act_fn(gate) * up`) on top of the standard Mixtral `[num_experts, 2 * moe_intermediate_size, hidden_size]`
expert weight layout. A single shared expert (a plain SwiGLU MLP at `moe_intermediate_size` width) runs in parallel
on every token.

### Attention mask layout

Each `DeepseekV4Attention` layer extends the standard sliding-window-causal mask along the key axis with a
`block_bias` returned by its compressor, then feeds the concatenated mask to `eager_attention_forward`. The
sliding-section (left, `[S, S]`) is the same for every layer type; the compressor-section (right) differs by
layer type and is the actual "novel" piece introduced by V4.

The diagrams below were produced with a tiny config (`sliding_window=8`, CSA `m=4`, HCA `m'=8`, `index_topk=2`)
on a 16-token input so the full per-layer-type mask fits on screen. Green = the query/key diagonal in the
sliding section, dark = a visible standard KV position, light = masked, amber = a compressor / indexer slot
the query is allowed to attend to. Columns past the dashed line are appended by the compressor via
`cat([sliding_causal_mask, block_bias], dim=-1)`.

**Sliding-only layer (`"sliding_attention"`).** No compressor, no right-padding — the mask is the plain
sliding-window-causal mask of shape `[S, S]` (window = 8). For `i ≥ window` the lower-left triangle is cut
off, recovering the local-only attention pattern.

**CSA layer (`"compressed_sparse_attention"`).** The compressor flattens its per-query gathered output to
`[B, 1, S·k, D]` and right-pads the mask by `S·k` columns. For query `t`, only the `k` slots at columns
`[S + t·k, S + (t+1)·k)` carry the indexer's picks; all other compressor columns are `-inf`. Queries before
the first window has closed (`t < m − 1`) get nothing — the indexer's `-1` sentinel propagates straight to
the mask. As `t` grows, more compressed entries are ready and the indexer can fill all `k` slots.

**HCA layer (`"heavily_compressed_attention"`).** No indexer — every cached compressed entry is potentially
visible. Right-padded by `T_total = entry_count["compressor"]` columns. Query `t` may only see entry `w` once
its source window has closed, i.e. `w < (t + 1) // m`. With `m=8` here, entries 0 (covers positions `0..7`)
and 1 (covers `8..15`) only become visible at `t ≥ 7` and `t ≥ 15` respectively.

These diagrams are reproducible end-to-end via:

```bash
python docs/source/en/imgs/deepseek_v4/visualize_attention_masks.py \
    --svg docs/source/en/imgs/deepseek_v4
```

The script runs a forward pass on this tiny config, wraps each attention layer to capture the exact
post-`cat([attention_mask, block_bias])` mask, remaps CSA's `[S, S·k]` flat-slot mask back to a
`[S, T_entries]` entry-visibility view (so each `C_w` column is a compressed *entry*, not a gather slot),
and writes the three SVGs above. It also prints an ANSI grid to stdout for quick terminal inspection and
dumps the indexer's per-query top-k picks so warm-up sentinels and pick choices are auditable.

### Cache layers

Each non-sliding attention block needs to thread compressor / indexer state across forward calls. V4 ships two
cache layer types that auto-register with `LAYER_TYPE_CACHE_MAPPING`:

* `DeepseekV4HCACache`: sliding-window K=V + HCA compressor buffer / pool / count (no overlap, no indexer).
* `DeepseekV4CSACache`: sliding-window K=V + CSA compressor (with overlap state) + parallel indexer
  buffer / pool / count / overlap at `index_head_dim`.

`DynamicCache(config=…)` builds the right cache layer per `config.layer_types[i]`.

## DeepseekV4Config[[transformers.DeepseekV4Config]]

#### transformers.DeepseekV4Config[[transformers.DeepseekV4Config]]

```python
transformers.DeepseekV4Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 129280, hidden_size: int = 4096, moe_intermediate_size: int = 2048, num_hidden_layers: int = 43, num_attention_heads: int = 64, num_key_value_heads: int = 1, head_dim: int = 512, q_lora_rank: int = 1024, num_experts_per_tok: int = 6, n_routed_experts: int = 256, n_shared_experts: int = 1, scoring_func: str = 'sqrtsoftplus', norm_topk_prob: bool = True, routed_scaling_factor: float = 1.5, max_position_embeddings: int = 1048576, rope_theta: float | int = 10000.0, layer_types: list[str] | None = None, compress_rates: dict | None = None, compress_rope_theta: float | int = 160000.0, hc_mult: int = 4, hc_sinkhorn_iters: int = 20, hc_eps: float = 1e-06, mlp_layer_types: list[str] | None = None, swiglu_limit: float = 10.0, sliding_window: int = 128, o_groups: int = 8, o_lora_rank: int = 1024, index_n_heads: int = 64, index_head_dim: int = 128, index_topk: int = 512, num_nextn_predict_layers: int = 1, output_router_logits: bool = False, router_aux_loss_coef: float = 0.001, router_jitter_noise: float = 0.0, hidden_act: str = 'silu', initializer_range: float = 0.02, rms_norm_eps: float = 1e-06, use_cache: bool = True, pad_token_id: int | None = None, bos_token_id: int | None = 0, eos_token_id: int | list[int] | None = 1, tie_word_embeddings: bool = False, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, partial_rotary_factor: float | None = None, attention_bias: bool = False, mlp_bias: bool = False, attention_dropout: float = 0.0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_v4/configuration_deepseek_v4.py#L40)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `129280`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `4096`) : Dimension of the hidden representations.

moe_intermediate_size (`int`, *optional*, defaults to `2048`) : Intermediate size of the routed expert MLPs.

num_hidden_layers (`int`, *optional*, defaults to `43`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `64`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `1`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

head_dim (`int`, *optional*, defaults to `512`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

q_lora_rank (`int`, *optional*, defaults to `1024`) : Rank of the LoRA matrices for query projections.

num_experts_per_tok (`int`, *optional*, defaults to `6`) : Number of experts to route each token to. This is the top-k value for the token-choice routing.

n_routed_experts (`int`, *optional*, defaults to `256`) : Number of routed experts.

n_shared_experts (`int`, *optional*, defaults to `1`) : Number of shared experts.

scoring_func (`str`, *optional*, defaults to `sqrtsoftplus`) : Router activation — `sqrtsoftplus`, `softmax`, or `sigmoid`.

norm_topk_prob (`bool`, *optional*, defaults to `True`) : Whether to normalize the weights of the routed experts. 

routed_scaling_factor (`float`, *optional*, defaults to `1.5`) : Scaling factor or routed experts.

max_position_embeddings (`int`, *optional*, defaults to `1048576`) : The maximum sequence length that this model might ever be used with.

rope_theta (`Union[float, int]`, *optional*, defaults to `10000.0`) : RoPE base for the main self-attention rotary.

layer_types (`list[str]`, *optional*) : Per-layer attention schedule with values from `{"compressed_sparse_attention", "heavily_compressed_attention"}`. V4-Pro default: 2× HCA bootstrap + interleaved CSA / HCA.

compress_rates (`dict[str, int]`) : Per-layer-type compression rate. Default `{"compressed_sparse_attention": 4, "heavily_compressed_attention": 128}` (m=4 for CSA, m'=128 for HCA, paper §2.3.1 / §2.3.2). BC: configs that ship `compress_rate_csa` / `compress_rate_hca` as top-level kwargs are folded in at `__post_init__` time.

compress_rope_theta (`Union[float, int]`, *optional*, defaults to `160000.0`) : RoPE base for the compressed branches (paired with `rope_scaling` for YaRN).

hc_mult (`int`, *optional*, defaults to `4`) : Manifold-Constrained Hyper-Connection (mHC) expansion factor n_hc (always active; Section 2.2).

hc_sinkhorn_iters (`int`, *optional*, defaults to `20`) : Sinkhorn-Knopp iterations t_max for the mHC residual mapping projection onto doubly-stochastic matrices.

hc_eps (`float`, *optional*, defaults to `1e-06`) : Numerical floor for the Sinkhorn-Knopp normalization.

mlp_layer_types (`list[str]`, *optional*) : Per-layer MoE schedule with values from `{"hash_moe", "moe"}`. `hash_moe` routes via a frozen `tid2eid[input_ids]` lookup (paper §2.1, "Hash-MoE bootstrap"); `moe` is the standard top-k routed MoE. Default: 3× `hash_moe` then `moe` for the rest. BC: legacy configs that ship `num_hash_layers` as a top-level kwarg are folded in at `__post_init__` time.

swiglu_limit (`float`, *optional*, defaults to `10.0`) : Clip routed experts' gate/up pre-activations.

sliding_window (`int`, *optional*, defaults to `128`) : Local window size n_win used in every attention block's sliding-window branch.

o_groups (`int`, *optional*, defaults to `8`) : Number of head-groups g in the grouped output projection (paper §2.3.1, "Grouped Output Projection").

o_lora_rank (`int`, *optional*, defaults to `1024`) : Per-group intermediate dim d_g in the grouped output projection.

index_n_heads (`int`, *optional*, defaults to `64`) : Number of indexer query heads n_h^I (paper §2.3.1, eq. 14).

index_head_dim (`int`, *optional*, defaults to `128`) : Indexer head dim c^I (paper §2.3.1).

index_topk (`int`, *optional*, defaults to `512`) : Number of compressed entries per query the Lightning Indexer keeps via top-k (paper §2.3.1, eq. 17).

num_nextn_predict_layers (`int`, *optional*, defaults to `1`) : MTP layer count in the upstream checkpoint (not instantiated here).

output_router_logits (`bool`, *optional*, defaults to `False`) : Whether or not the router logits should be returned by the model. Enabling this will also allow the model to output the auxiliary loss, including load balancing loss and router z-loss.

router_aux_loss_coef (`float`, *optional*, defaults to `0.001`) : Auxiliary load balancing loss coefficient. Used to penalize uneven expert routing in MoE models.

router_jitter_noise (`float`, *optional*, defaults to `0.0`) : Amount of noise to add to the router logits during training for better load balancing.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

rms_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the rms normalization layers.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

pad_token_id (`int`, *optional*) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `0`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `1`) : Token id used for end-of-stream in the vocabulary.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

partial_rotary_factor (`float`, *optional*) : Fraction of head_dim that gets RoPE. Defaults to `qk_rope_head_dim / head_dim` so cos/sin sizes to `qk_rope_head_dim`.

attention_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

mlp_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in up_proj, down_proj and gate_proj layers in the MLP layers.

attention_dropout (`float`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

This is the configuration class to store the configuration of a DeepseekV4Model. It is used to instantiate a Deepseek V4
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [deepseek-ai/DeepSeek-V4-Flash-Base](https://huggingface.co/deepseek-ai/DeepSeek-V4-Flash-Base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

#### validate_layer_type[[transformers.DeepseekV4Config.validate_layer_type]]

```python
validate_layer_type()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_v4/configuration_deepseek_v4.py#L225)

V4 narrows the global `ALLOWED_LAYER_TYPES` to the three attention-block
types and two MLP-block types it actually ships with, on top of the standard
length / type-membership checks.

## DeepseekV4Model[[transformers.DeepseekV4Model]]

#### transformers.DeepseekV4Model[[transformers.DeepseekV4Model]]

```python
transformers.DeepseekV4Model(config: DeepseekV4Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_v4/modeling_deepseek_v4.py#L1256)

**Parameters:**

config ([DeepseekV4Config](/docs/transformers/v5.15.1/en/model_doc/deepseek_v4#transformers.DeepseekV4Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Deepseek V4 Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.DeepseekV4Model.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_v4/modeling_deepseek_v4.py#L1274)

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
elements depending on the configuration ([DeepseekV4Config](/docs/transformers/v5.15.1/en/model_doc/deepseek_v4#transformers.DeepseekV4Config)) and inputs.

The [DeepseekV4Model](/docs/transformers/v5.15.1/en/model_doc/deepseek_v4#transformers.DeepseekV4Model) forward method, overrides the `__call__` special method.

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

## DeepseekV4ForCausalLM[[transformers.DeepseekV4ForCausalLM]]

#### transformers.DeepseekV4ForCausalLM[[transformers.DeepseekV4ForCausalLM]]

```python
transformers.DeepseekV4ForCausalLM(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_v4/modeling_deepseek_v4.py#L1414)

**Parameters:**

config ([DeepseekV4ForCausalLM](/docs/transformers/v5.15.1/en/model_doc/deepseek_v4#transformers.DeepseekV4ForCausalLM)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Deepseek V4 Model for causal language modeling.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.DeepseekV4ForCausalLM.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, output_router_logits: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_v4/modeling_deepseek_v4.py#L1432)

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
elements depending on the configuration ([DeepseekV4Config](/docs/transformers/v5.15.1/en/model_doc/deepseek_v4#transformers.DeepseekV4Config)) and inputs.

The [DeepseekV4ForCausalLM](/docs/transformers/v5.15.1/en/model_doc/deepseek_v4#transformers.DeepseekV4ForCausalLM) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoTokenizer, DeepseekV4ForCausalLM

>>> model = DeepseekV4ForCausalLM.from_pretrained("mistralai/DeepseekV4-8x7B-v0.1")
>>> tokenizer = AutoTokenizer.from_pretrained("mistralai/DeepseekV4-8x7B-v0.1")

>>> prompt = "Hey, are you conscious? Can you talk to me?"
>>> inputs = tokenizer(prompt, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(inputs.input_ids, max_length=30)
>>> tokenizer.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"Hey, are you conscious? Can you talk to me?\nI'm not conscious, but I can talk to you."
```
