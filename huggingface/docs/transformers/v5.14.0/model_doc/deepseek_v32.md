# DeepSeek-V3.2

## Overview

[DeepSeek-V3.2-Exp](https://huggingface.co/deepseek-ai/DeepSeek-V3.2-Exp) is an experimental release from DeepSeek-AI that introduces **DeepSeek Sparse Attention (DSA)**, a trainable, fine-grained sparse attention mechanism designed to improve training and inference efficiency in long-context scenarios. It is built directly on top of [DeepSeek-V3.1-Terminus](https://huggingface.co/deepseek-ai/DeepSeek-V3.1-Terminus): the model keeps the same 685B-parameter Mixture-of-Experts (MoE) backbone and Multi-head Latent Attention (MLA), and is obtained through continued training that adds the sparse-attention indexer while deliberately aligning the training distribution with V3.1-Terminus so the two models can be compared head-to-head.

The work was later extended in the [DeepSeek-V3.2 technical report](https://huggingface.co/papers/2512.02556), *DeepSeek-V3.2: Pushing the Frontier of Open Large Language Models*, which pairs DSA with a scalable reinforcement-learning framework and reports gold-medal level results on competition math (IMO) and competitive programming (IOI) benchmarks.

The abstract from the DeepSeek-V3.2-Exp release is the following:

*We introduce DeepSeek-V3.2-Exp, an experimental version of our model that incorporates DeepSeek Sparse Attention (DSA) to explore and validate optimizations for training and inference efficiency in long-context scenarios. DeepSeek Sparse Attention achieves fine-grained sparse attention for the first time with minimal impact on model output quality. Built upon DeepSeek-V3.1-Terminus, DeepSeek-V3.2-Exp delivers substantially improved efficiency in both training and inference, especially in long-context settings, while maintaining virtually identical benchmark performance.*

### DeepSeek Sparse Attention (DSA)

DSA reduces the quadratic cost of attention over long sequences by attending only to a selected subset of past tokens. It has two components:

1. **Lightning indexer.** A lightweight, low-head-count scoring module computes an *index score* between each query and every preceding key. In the reference implementation it runs in FP8 with a Hadamard (`rotate_activation`) transform; because the transform is orthogonal (`Hq·Hk = q·k`) and FP8 is only a precision optimization, the transformers port computes the same scores directly in bf16/fp32, keeping the indexer cheap relative to the main attention.
2. **Fine-grained token selection.** For each query the indexer keeps the top-`index_topk` (2048 by default) tokens, and main MLA attention is then computed only over those tokens via an additive mask. This turns the per-query attention cost from `O(L)` to `O(index_topk)` for long sequences when using `flash_mla`, which is not supported yet 😉.

The indexer keeps its own small per-token key cache (single-head, `index_head_dim`) alongside the main K/V cache. In transformers this lives on a dedicated cache layer — `DynamicIndexedLayer` for growing caches and `StaticIndexedLayer` for static / `torch.compile` caches — and is updated through `past_key_values.update_indexer()`.

In DeepSeek-V3.2 **every layer runs its own indexer** — there is no cross-layer top-k sharing.

> [!NOTE]
> **The MLA query LoRA path (`q_lora_rank`) is required.** The indexer scores queries from the low-rank query latent `q_a_layernorm(q_a_proj(x))` (its `wq_b` projection is sized by `q_lora_rank`), so the model always uses the LoRA query path and `q_lora_rank` must be set — the released checkpoint uses `1536`. The optional non-LoRA `q_proj` path that [DeepSeek-V3](./deepseek_v3) exposes for `q_lora_rank=None` is **not supported** here: without the query latent there is nothing for the indexer to consume.

## Usage examples

DeepSeek-V3.2-Exp is distributed as an FP8 checkpoint. The indexer projections are kept out of FP8 quantization, since the checkpoint stores them in bf16/fp32:

```python
from transformers import FineGrainedFP8Config, AutoModelForCausalLM, AutoTokenizer
import torch

model_name = "deepseek-ai/DeepSeek-V3.2-Exp"
quantization_config = FineGrainedFP8Config(
    modules_to_not_convert=["model.layers.*.mlp.gate.*", "*.self_attn.indexer.weights_proj.*"],
    weight_block_size=(128, 128),
)
model = AutoModelForCausalLM.from_pretrained(
    model_name,
    torch_dtype="auto",
    device_map="auto",
    quantization_config=quantization_config,
)
tokenizer = AutoTokenizer.from_pretrained(model_name)

inputs = tokenizer("What are we having for dinner?", return_tensors="pt").to(model.device)
outputs = model.generate(**inputs, max_new_tokens=20)
print(tokenizer.decode(outputs[0], skip_special_tokens=True))
```

The original code can be found [here](https://github.com/deepseek-ai/DeepSeek-V3.2-Exp).

## DeepseekV32Config[[transformers.DeepseekV32Config]]

- **vocab_size** (`int`, *optional*, defaults to `129280`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `7168`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `18432`) --
  Dimension of the MLP representations.
- **moe_intermediate_size** (`int`, *optional*, defaults to `2048`) --
  Intermediate size of the routed expert MLPs.
- **num_hidden_layers** (`int`, *optional*, defaults to `61`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `128`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_key_value_heads** (`int`, *optional*, defaults to `128`) --
  This is the number of key_value heads that should be used to implement Grouped Query Attention. If
  `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if
  `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When
  converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed
  by meanpooling all the original heads within that group. For more details, check out [this
  paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to
  `num_attention_heads`.
- **n_shared_experts** (`int`, *optional*, defaults to `1`) --
  Number of shared experts.
- **n_routed_experts** (`int`, *optional*, defaults to `256`) --
  Number of routed experts.
- **routed_scaling_factor** (`float`, *optional*, defaults to `2.5`) --
  Scaling factor or routed experts.
- **kv_lora_rank** (`int`, *optional*, defaults to `512`) --
  Rank of the LoRA matrices for key and value projections.
- **q_lora_rank** (`int`, *optional*, defaults to `1536`) --
  Rank of the LoRA matrices for query projections.
- **qk_rope_head_dim** (`int`, *optional*, defaults to `64`) --
  Dimension of the query/key heads that use rotary position embeddings.
- **v_head_dim** (`int`, *optional*, defaults to `128`) --
  Dimension of the value heads.
- **qk_nope_head_dim** (`int`, *optional*, defaults to `128`) --
  Dimension of the query/key heads that don't use rotary position embeddings.
- **n_group** (`int`, *optional*, defaults to 1) --
  Number of groups for routed experts.
- **topk_group** (`int`, *optional*, defaults to `4`) --
  Number of selected groups for each token (for each token, ensuring the selected experts is only within `topk_group` groups).
- **num_experts_per_tok** (`int`, *optional*, defaults to `8`) --
  Number of experts to route each token to. This is the top-k value for the token-choice routing.
- **norm_topk_prob** (`bool`, *optional*, defaults to `True`) --
  Whether to normalize the weights of the routed experts.

- **hidden_act** (`str`, *optional*, defaults to `silu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **max_position_embeddings** (`int`, *optional*, defaults to `163840`) --
  The maximum sequence length that this model might ever be used with.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the rms normalization layers.
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **pad_token_id** (`int`, *optional*) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `0`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `1`) --
  Token id used for end-of-stream in the vocabulary.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **rope_parameters** (`dict`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **mlp_layer_types** (`list`, *optional*) --
  MLP type pattern for each layer (`"dense"` or `"sparse"`). Defaults to 3 dense + rest sparse.
- **attention_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in the query, key, value and output projection layers during self-attention.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **index_topk** (`int`, *optional*, defaults to 2048) --
  Number of top tokens selected by the indexer for sparse attention.
- **index_head_dim** (`int`, *optional*, defaults to 128) --
  Head dimension for the indexer projections (DSA).
- **index_n_heads** (`int`, *optional*, defaults to 64) --
  Number of heads for the indexer projections (DSA).
- **mlp_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in up_proj, down_proj and gate_proj layers in the MLP layers.
- **head_dim** (`int`, *optional*, defaults to `64`) --
  The attention head dimension. If None, it will default to hidden_size // num_attention_heads
- **first_k_dense_replace** (`int`, *optional*, defaults to 3) --
  Number of leading layers that use a dense MLP; the rest use the MoE block.
- **layer_types** (`list[str]`, *optional*) --
  A list that explicitly maps each layer index with its layer type. If not provided, it will be automatically
  generated based on config values.

This is the configuration class to store the configuration of a DeepseekV32Model. It is used to instantiate a Deepseek V32
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [deepseek-ai/DeepSeek-V3.2-Exp](https://huggingface.co/deepseek-ai/DeepSeek-V3.2-Exp)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import DeepseekV32Config, DeepseekV32Model

>>> # Initializing a DeepSeek-V3.2 configuration
>>> configuration = DeepseekV32Config()

>>> # Initializing a model from the configuration
>>> model = DeepseekV32Model(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## DeepseekV32PreTrainedModel[[transformers.DeepseekV32PreTrainedModel]]

- **config** ([PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

A mock value for a dotted path (e.g. `torch.float32`): attribute access chains,
calls behave as pass-through decorators, `repr` is the dotted path, and using it
as a base class substitutes a plain-`type` base (PEP 560 `__mro_entries__`), so
real subclasses keep a normal metaclass and `inspect.signature` reads their real
`__init__` instead of a mock's.

## DeepseekV32Model[[transformers.DeepseekV32Model]]

- **config** ([DeepseekV32Config](/docs/transformers/v5.14.0/en/model_doc/deepseek_v32#transformers.DeepseekV32Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Deepseek V32 Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **past_key_values** (`~cache_utils.Cache`, *optional*) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).[BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DeepseekV32Config](/docs/transformers/v5.14.0/en/model_doc/deepseek_v32#transformers.DeepseekV32Config)) and inputs.
The [DeepseekV32Model](/docs/transformers/v5.14.0/en/model_doc/deepseek_v32#transformers.DeepseekV32Model) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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

## DeepseekV32ForCausalLM[[transformers.DeepseekV32ForCausalLM]]

- **config** ([DeepseekV32ForCausalLM](/docs/transformers/v5.14.0/en/model_doc/deepseek_v32#transformers.DeepseekV32ForCausalLM)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Deepseek V32 Model for causal language modeling.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **past_key_values** (`~cache_utils.Cache`, *optional*) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).[CausalLMOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or `tuple(torch.FloatTensor)`A [CausalLMOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DeepseekV32Config](/docs/transformers/v5.14.0/en/model_doc/deepseek_v32#transformers.DeepseekV32Config)) and inputs.
The [DeepseekV32ForCausalLM](/docs/transformers/v5.14.0/en/model_doc/deepseek_v32#transformers.DeepseekV32ForCausalLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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
>>> from transformers import AutoTokenizer, DeepseekV32ForCausalLM

>>> model = DeepseekV32ForCausalLM.from_pretrained("meta-deepseek_v32/DeepseekV32-2-7b-hf")
>>> tokenizer = AutoTokenizer.from_pretrained("meta-deepseek_v32/DeepseekV32-2-7b-hf")

>>> prompt = "Hey, are you conscious? Can you talk to me?"
>>> inputs = tokenizer(prompt, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(inputs.input_ids, max_length=30)
>>> tokenizer.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"Hey, are you conscious? Can you talk to me?\nI'm not conscious, but I can talk to you."
```
