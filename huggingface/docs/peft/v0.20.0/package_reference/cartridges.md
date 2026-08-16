# Cartridges

Cartridges are a prompt-learning method that stores a compressed long-context representation as a parameterized KV-cache
prefix. The core idea comes from the paper
[Cartridges: Lightweight and general-purpose long context representations via self-study](https://huggingface.co/papers/2506.06266).

For a high-level overview and motivation, see the blog post
[Cartridges: Storing long contexts in tiny caches with self-study](https://hazyresearch.stanford.edu/blog/2025-06-08-cartridges).

## How Cartridges differ from Prefix Tuning

Both Prefix Tuning and Cartridges are served by injecting `past_key_values` (a prefix KV cache) into the base model.

- Prefix Tuning learns virtual token embeddings (and optionally an MLP projection) and produces a KV prefix.
- Cartridges learn the KV prefix itself directly (the per-layer key/value vectors for `p` virtual tokens), and are
  designed to be initialized from real prefill KV (for example, the first `p` tokens of a corpus/system prompt).

The paper also recommends freezing the first token as an attention sink for stability (`num_frozen_tokens=1` is the
default).

## Usage (inference)

Load a trained CARTRIDGE adapter and run generation:

```py
from transformers import AutoModelForCausalLM, AutoTokenizer

from peft import PeftModel

model_id = "Qwen/Qwen2.5-0.5B-Instruct"
adapter_path = "path/to/cartridge_adapter"

base = AutoModelForCausalLM.from_pretrained(model_id)
model = PeftModel.from_pretrained(base, adapter_path)

tok = AutoTokenizer.from_pretrained(model_id)
if tok.pad_token is None:
    tok.pad_token = tok.eos_token

out = model.generate(**tok("Question about the corpus:", return_tensors="pt"), max_new_tokens=64)
print(tok.decode(out[0], skip_special_tokens=True))
```

If you need to create and initialize a cartridge before training, see the initialization options below.

## Initialization options

The paper discusses a few practical initialization strategies:

- Random KV (default): create a `CartridgeConfig` and start training. This initializes the KV prefix randomly.
- KV from the first tokens of a prompt/corpus: use `initialize_kv_prefix_from_text(model, tokenizer, text=...)`. This
  runs a prefill on `text` and copies the resulting KV cache for the first `num_virtual_tokens` into the adapter.
- KV from an existing cache: use `initialize_kv_prefix_from_past_key_values(model, past_key_values=...)` if you already
  have a `past_key_values` object from a base-model prefill.

## Training

The Cartridges paper proposes a SELF-STUDY distillation objective (a frozen base model provides teacher logits; the
CARTRIDGE adapter is trained so the student matches the teacher’s next-token distribution over the target segment).
PEFT keeps training logic out of the core library; see
`https://github.com/huggingface/peft/tree/main/examples/cartridge_self_study` for a reference workflow.
The example scripts use the frozen base model as the teacher and the adapted model as the student, so both share the
same underlying checkpoint.

## Composition

To concatenate independently trained cartridges into a single adapter, use `compose_cartridge_adapters(...)`.

# API

## CartridgeConfig[[peft.CartridgeConfig]]

#### peft.CartridgeConfig[[peft.CartridgeConfig]]

```python
peft.CartridgeConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, num_virtual_tokens: int = None, token_dim: int = None, num_transformer_submodules: Optional[int] = None, num_attention_heads: Optional[int] = None, num_layers: Optional[int] = None, modules_to_save: Optional[list[str]] = None, num_frozen_tokens: int = 1)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/cartridge/config.py#L22)

**Parameters:**

num_frozen_tokens (`int`, defaults to 1) : Number of *prefix* tokens at the start of the cartridge to keep frozen (no gradients). The Cartridges paper recommends freezing the first token as an attention sink for stability (set this to `1`), as many LLMs use early tokens as attention sinks and changing them can harm training.

Configuration for CARTRIDGE, a KV-cache-parameterized prefix adapter.

This is similar to prefix-tuning in how it is served (as `past_key_values`), but it stores the KV cache directly as
trainable parameters instead of learning it via an MLP projection.

Initialization:
The Cartridges paper discusses multiple initialization options. In PEFT, initialization is a *separate* step
from constructing the adapter config:

- **Random KV initialization (paper option 2)**: Create the adapter via `get_peft_model(...)`. The CARTRIDGE
  prompt encoder parameters are randomly initialized by PyTorch.

- **KV derived from the first tokens of a prompt/corpus (paper option 3)**: Run a no-grad prefill on the *base
  model* and copy the first `num_virtual_tokens` cached KV tokens into the adapter. PEFT provides utilities for
  this (importable from `peft` or from `peft.tuners.cartridge.utils`):

  - `initialize_kv_prefix_from_text(model, tokenizer, text=...)`
  - `initialize_kv_prefix_from_past_key_values(model, past_key_values=...)`

  If you already have a flattened KV-prefix tensor, you can load it directly via the prompt encoder’s
  `load_prompt_embeddings(...)` method.

## CartridgeEncoder[[peft.CartridgeEncoder]]

#### peft.CartridgeEncoder[[peft.CartridgeEncoder]]

```python
peft.CartridgeEncoder(config)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/cartridge/model.py#L20)

A parameterized prefix KV cache.

The parameters are stored in the same flattened layout as `PrefixEncoder` output: `[num_virtual_tokens, num_layers
* 2 * token_dim]`, where `token_dim` is per-head hidden size times number of heads (after any GQA adjustment
performed by `_prepare_prompt_learning_config`).

If `num_frozen_tokens > 0`, the first `num_frozen_tokens` virtual tokens are stored as a non-trainable parameter,
and the remaining tokens are trainable.

#### load_prompt_embeddings[[peft.CartridgeEncoder.load_prompt_embeddings]]

```python
load_prompt_embeddings(prompt_embeddings: torch.Tensor)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/cartridge/model.py#L89)

Load the flattened prompt embeddings saved by PEFT (`prompt_embeddings`).

PEFT saves prompt-learning adapters as a single `prompt_embeddings` tensor. For CARTRIDGE, we split that tensor
into frozen and trainable segments according to `self.num_frozen_tokens`.
