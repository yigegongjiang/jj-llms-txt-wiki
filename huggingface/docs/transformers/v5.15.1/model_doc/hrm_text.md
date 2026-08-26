# HRM-Text

## Overview

HRM-Text is the improved autoregressive language-modeling variant of the Hierarchical Reasoning Model
(HRM, [Hierarchical Reasoning Model](https://huggingface.co/papers/2506.21734)) by the Sapient AI team.
It is a base model that uses a *hierarchical recurrent* forward — two transformer stacks (`H` for slow,
abstract planning, and `L` for fast, detailed computation) are reused inside a nested recurrence:

```
for h in range(H_cycles):
    for l in range(L_cycles):
        z_L = L(z_L + z_H)
    z_H = H(z_H + z_L)
```

Architectural traits:

- **PrefixLM attention**: instruction tokens attend bidirectionally, response tokens attend
  causally. Controlled by `config.prefix_lm` (default `True`); see [4D-masks blog](https://huggingface.co/blog/poedator/4d-masks) /
  [FlexAttention blog](https://pytorch.org/blog/flexattention/) for the canonical form.
- **Per-head sigmoid output gate** applied to the attention output before `o_proj`
  (Qwen3-Next-style; see [`Qwen3NextAttention`](./qwen3_next)). Legacy checkpoints stored as
  a single fused `gqkv_proj` are split into `gate_proj` / `q_proj` / `k_proj` / `v_proj` at
  load time by the registered HRM-Text checkpoint conversion mapping.
- **Parameterless RMSNorm** — `F.rms_norm` with no learnable scale.
- **`L_bp_cycles`** — the *k-step grad trick* from HRM. At training time, only the trailing
  `L_bp_cycles[i]` of the `L_cycles` low-level iterations propagate gradients;
  earlier iterations run under `torch.no_grad()` so their activations are not
  stored. No effect at inference.

## Usage

HRM-Text-1B is a **base language model**. It does not ship a `chat_template` and
`apply_chat_template` is intentionally not supported for this release — the prompt
format used during pre-training is still evolving, and an instruction-tuned variant with
a stable chat template will follow in a separate release. Drive the base model through
plain `AutoTokenizer` + `AutoModelForCausalLM.generate(...)`:

```python
from transformers import AutoTokenizer, AutoModelForCausalLM

tokenizer = AutoTokenizer.from_pretrained("sapientinc/HRM-Text-1B")
model = AutoModelForCausalLM.from_pretrained(
    "sapientinc/HRM-Text-1B", device_map="auto",
)

inputs = tokenizer("The quick brown fox", return_tensors="pt").to(model.device)
out = model.generate(**inputs, max_new_tokens=16, do_sample=False)
print(tokenizer.decode(out[0], skip_special_tokens=True))
```

### Attention backends

`"sdpa"` is the default, and is the right choice for most workloads. `"flex_attention"`
is supported and pays off at long context — but it carries a fixed BlockMask construction
cost per forward that does not amortise to the win you might expect from HRM-Text's
recurrent stack reuse. Indicative prefill latency on a single H100 with the released
1.2B base checkpoint and the default `H_cycles=2`, `L_cycles=3`:

| seq_len | sdpa     | flex_attention | recommendation |
|---------|----------|----------------|----------------|
|   64    |  41 ms   |  70 ms         | sdpa           |
|  256    |  41 ms   |  70 ms         | sdpa           |
| 1024    |  42 ms   |  69 ms         | sdpa           |
| 2048    |  85 ms   |  78 ms         | flex (≈ 1.1x)  |

So pick the backend by the workload:

```python
# Default — short / medium context
model = AutoModelForCausalLM.from_pretrained("sapientinc/HRM-Text-1B", device_map="auto")

# Long context (≥ 2K tokens) — FlexAttention's per-block sparsity overtakes SDPA
model = AutoModelForCausalLM.from_pretrained(
    "sapientinc/HRM-Text-1B", device_map="auto", attn_implementation="flex_attention",
)
```

Both backends produce equivalent logits (verified top-1 100% match end-to-end against
the torch reference). `"eager"` is supported and produces the same logits, but is rarely
the fastest option on modern hardware. Its main use is `output_attentions=True` —
SDPA / FlexAttention do not return per-head attention weights, so passes that need them
for analysis or visualisation should run with `attn_implementation="eager"`.

> [!WARNING]
> Any FlashAttention variation — FA 2/3/4 and HF Hub kernel implementations that may
> not follow the `flash_attention_*` naming convention — is rejected by [HrmTextModel](/docs/transformers/v5.15.1/en/model_doc/hrm_text#transformers.HrmTextModel)
> at init whenever `config.prefix_lm=True` (the default). FA backends only accept causal
> vs. non-causal masks and cannot represent the PrefixLM 4-D overlay. Use `"sdpa"`
> (default) or `"flex_attention"` for PrefixLM. Setting `config.prefix_lm=False` makes
> the mask pure causal and re-enables FA — useful for causal-only fine-tuning or
> inference paths where FA is the fastest option.

### PrefixLM training

For supervised fine-tuning that respects the instruction / response boundary, emit
`token_type_ids` from the data collator alongside `input_ids` — positions inside the
instruction get `1`, response and padding get `0`. The model treats every position with
`token_type_ids == 1` as part of a single bidirectional block; everything else stays
causal:

```python
import torch

def collate_prefixlm(batch, pad_token_id=0, ignore_label_id=-100):
    """`batch[i] = {"instruction_ids": [...], "response_ids": [...]}`."""
    full_ids = [b["instruction_ids"] + b["response_ids"] for b in batch]
    prefix_lens = [len(b["instruction_ids"]) for b in batch]
    max_len = max(len(ids) for ids in full_ids)

    input_ids = torch.full((len(batch), max_len), pad_token_id, dtype=torch.long)
    token_type_ids = torch.zeros_like(input_ids)
    labels = torch.full_like(input_ids, ignore_label_id)
    attention_mask = torch.zeros_like(input_ids)

    for i, (ids, plen) in enumerate(zip(full_ids, prefix_lens)):
        input_ids[i, : len(ids)] = torch.tensor(ids)
        token_type_ids[i, :plen] = 1                      # bidirectional prefix
        labels[i, plen : len(ids)] = input_ids[i, plen : len(ids)]  # loss on response only
        attention_mask[i, : len(ids)] = 1
    return {
        "input_ids": input_ids,
        "token_type_ids": token_type_ids,
        "attention_mask": attention_mask,
        "labels": labels,
    }
```

See [HrmTextModel.forward()](/docs/transformers/v5.15.1/en/model_doc/hrm_text#transformers.HrmTextModel.forward) for the accepted shape.

## HrmTextConfig[[transformers.HrmTextConfig]]

#### transformers.HrmTextConfig[[transformers.HrmTextConfig]]

```python
transformers.HrmTextConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 151808, hidden_size: int = 1536, intermediate_size: int = 4096, num_hidden_layers: int = 16, num_attention_heads: int = 12, hidden_act: str = 'silu', max_position_embeddings: int = 2048, initializer_range: float = 0.02, rms_norm_eps: float = 1e-06, use_cache: bool = True, pad_token_id: int | None = None, bos_token_id: int | None = None, eos_token_id: int | list[int] | None = None, tie_word_embeddings: bool = False, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, attention_bias: bool = False, attention_dropout: int | float | None = 0.0, mlp_bias: bool = False, head_dim: int = 128, H_cycles: int = 2, L_cycles: int = 3, L_bp_cycles: list[int] | None = None, embedding_scale: float | None = None, prefix_lm: bool = True, num_layers_per_stack: int | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hrm_text/configuration_hrm_text.py#L32)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `151808`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `1536`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `4096`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `16`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer decoder.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `2048`) : The maximum sequence length that this model might ever be used with.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

rms_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the rms normalization layers.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

pad_token_id (`int`, *optional*) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*) : Token id used for end-of-stream in the vocabulary.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

attention_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

attention_dropout (`Union[int, float]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

mlp_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in up_proj, down_proj and gate_proj layers in the MLP layers.

head_dim (`int`, *optional*, defaults to `128`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

H_cycles (`int`, *optional*, defaults to 2) : Number of high-level cycles.

L_cycles (`int`, *optional*, defaults to 3) : Number of low-level cycles per H-cycle.

L_bp_cycles (`list[int]`, *optional*, defaults to `[2]`) : Training-time gradient-routing list; left-padded with `1`s up to `L_cycles` inside the model. Inference-time no-op.

embedding_scale (`float`, *optional*) : Token-embedding multiplier. If `None`, defaults to `1 / initializer_range`.

prefix_lm (`bool`, *optional*, defaults to `True`) : Instruction tokens attend bidirectionally, response tokens attend causally.

num_layers_per_stack (`int`, *optional*) : Real number of transformer blocks inside each of the H / L stacks. Set automatically on first construction: the value passed as `num_hidden_layers` is remembered here and `num_hidden_layers` is then rewritten to `num_layers_per_stack * H_cycles * (L_cycles + 1)` so that `DynamicCache(config=...)` pre-allocates one slot per unique attention invocation under the recurrent forward. Do not set this directly on first construction — pass the real per-stack count as `num_hidden_layers` and let `__post_init__` split it.

This is the configuration class to store the configuration of a HrmTextModel. It is used to instantiate a Hrm Text
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [sapientinc/HRM-Text-1B](https://huggingface.co/sapientinc/HRM-Text-1B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## HrmTextModel[[transformers.HrmTextModel]]

#### transformers.HrmTextModel[[transformers.HrmTextModel]]

```python
transformers.HrmTextModel(config: HrmTextConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hrm_text/modeling_hrm_text.py#L410)

**Parameters:**

config ([HrmTextConfig](/docs/transformers/v5.15.1/en/model_doc/hrm_text#transformers.HrmTextConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Hrm Text Text Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.HrmTextModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, token_type_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hrm_text/modeling_hrm_text.py#L434)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

token_type_ids (`torch.LongTensor` of shape `(batch, seq_len)`, *optional*) : Per-position bidirectional/causal indicator. Tokens with `token_type_ids == 1` form a single bidirectional block; all other positions are causal.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([HrmTextConfig](/docs/transformers/v5.15.1/en/model_doc/hrm_text#transformers.HrmTextConfig)) and inputs.

The [HrmTextModel](/docs/transformers/v5.15.1/en/model_doc/hrm_text#transformers.HrmTextModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
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

## HrmTextForCausalLM[[transformers.HrmTextForCausalLM]]

#### transformers.HrmTextForCausalLM[[transformers.HrmTextForCausalLM]]

```python
transformers.HrmTextForCausalLM(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hrm_text/modeling_hrm_text.py#L548)

**Parameters:**

config ([HrmTextForCausalLM](/docs/transformers/v5.15.1/en/model_doc/hrm_text#transformers.HrmTextForCausalLM)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Hrm Text Model for causal language modeling.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.HrmTextForCausalLM.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, token_type_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/hrm_text/modeling_hrm_text.py#L563)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

token_type_ids (`torch.LongTensor` of shape `(batch, seq_len)`, *optional*) : Per-position bidirectional/causal indicator. Tokens with `token_type_ids == 1` form a single bidirectional block; all other positions are causal.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** [CausalLMOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or `tuple(torch.FloatTensor)`

A [CausalLMOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([HrmTextConfig](/docs/transformers/v5.15.1/en/model_doc/hrm_text#transformers.HrmTextConfig)) and inputs.

The [HrmTextForCausalLM](/docs/transformers/v5.15.1/en/model_doc/hrm_text#transformers.HrmTextForCausalLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
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
