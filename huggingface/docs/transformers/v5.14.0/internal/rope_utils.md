# Utilities for Rotary Embedding

This page explains how the Rotary Embedding is computed and applied in Transformers and what types of RoPE are supported.

## Overview

Rotary Position Embeddings are a technique used to inject positional information into attention mechanisms without relying on explicit position encodings.  
Instead of adding position vectors to token embeddings, RoPE rotates query and key vectors in the complex plane according to their positions enabling relative positional awareness and better extrapolation to unseen sequence lengths.

The Transformers library provides a flexible and extensible implementation of various RoPE types defined in ``~modeling_rope_utils.ROPE_VALIDATION_FUNCTIONS``, including both the default and scaled variants:

| Rope Type | Description |
|------------|-------------|
| `"default"` | Standard rotary embedding as in LLaMA. |
| `"linear"` | Linear-scaled RoPE which allows longer context windows. |
| `"dynamic"` | NTK-aware scaling computed by rescaling frequency base (`θ`) for longer context. |
| `"yarn"` | YaRN scaling variant providing smoother extrapolation and stability. |
| `"longrope"` | [LongRoPE](https://github.com/microsoft/LongRoPE) scaling as in Phi-2 model series. |
| `"llama3"` | RoPE scaling as in Llama3.1. |

## Configuration in Model Configs

To enable and customize rotary embeddings, add a `rope_parameters` field to your model’s configuration file (`config.json`). This field controls the RoPE behavior across model layers. Note that each RoPE variant defines its own set of expected keys and missing keys will raise an error. See the example below which creates a llama config with default RoPE parameters:

```python
from transformers import LlamaConfig

config = LlamaConfig()
config.rope_parameters = {
    "rope_type": "default", # type of RoPE to use
    # rope_theta is optional — omitting it uses the model’s default_theta (typically 10000.0)
}

# If we want to apply a scaled RoPE type, we need to pass extra parameters
config.rope_parameters = {
    "rope_type": "linear",
    "rope_theta": 10000.0,  # can be omitted to fall back to default_theta
    "factor": 8.0  # scale factor for context extension
}
```

## Per-Layer-Type RoPE Configuration

Some models such as Gemma-3 use different layer types with different attention mechanisms, i.e. "full attention" in some blocks and "sliding-window attention" in others. Transformers supports specifying distinct RoPE parameters per layer type for these models. In this case, `rope_parameters` should be a nested dictionary, where top-level keys correspond to `config.layer_types` and values are per-type RoPE parameters. During model initialization, each decoder layer will automatically look up the matching RoPE configuration based on its declared layer type.

```python
from transformers import Gemma3Config

config = Gemma3Config()
config.rope_parameters = {
    "full_attention": {
        "rope_type": "dynamic",
        "rope_theta": 1000000.0,
        "factor": 8.0,
        "original_max_position_embeddings": 8096,
    },
    "sliding_attention": {
        "rope_type": "default",
        "rope_theta": 10000.0,
    }
}
```

## Utilities[[transformers.RopeParameters]]

- **rope_theta** (`float`, *optional*, defaults to `RotaryEmbeddingConfigMixin.default_theta`) --
  The base period of the RoPE embeddings. Optional in serialized configs — if omitted,
  the model's `default_theta` (typically 10000.0) is used.
- **rope_type** (`str`, *optional*, defaults to "default") --
  The sub-variant of RoPE to use. Can be one of ['default', 'linear', 'dynamic', 'yarn', 'longrope',
  'llama3'], with 'default' being the original RoPE implementation.
- **partial_rotary_factor** (`float`, *optional*) --
  The percentage of the query and key head embedding on which RoPE will be applied.
- **factor** (`float`, *optional*) --
  Used with all rope types except 'default'. The scaling factor to apply to the RoPE embeddings. In
  most scaling types, a `factor` of x will enable the model to handle sequences of length x *
  original maximum pre-trained length.
- **original_max_position_embeddings** (`int`, *optional*) --
  Used with 'yarn', 'longrope' and 'llama3'. The original max position embeddings used during
  pretraining.
- **attention_factor** (`float`, *optional*) --
  Used with 'yarn' and 'longrope'. The scaling factor to be applied on the attention
  computation. If unspecified, it defaults to value recommended by the implementation, using the
  `factor` field to infer the suggested value.
- **beta_fast** (`float`, *optional*) --
  Only used with 'yarn'. Parameter to set the boundary for extrapolation (only) in the linear
  ramp function. If unspecified, it defaults to 32.
- **beta_slow** (`float`, *optional*) --
  Only used with 'yarn'. Parameter to set the boundary for interpolation (only) in the linear
  ramp function. If unspecified, it defaults to 1.
- **short_factor** (`list[float]`, *optional*) --
  Only used with 'longrope'. The scaling factor to be applied to short contexts (<
  `original_max_position_embeddings`). Must be a list of numbers with the same length as the hidden
  size divided by the number of attention heads divided by 2
- **long_factor** (`list[float]`, *optional*) --
  Only used with 'longrope'. The scaling factor to be applied to long contexts (<
  `original_max_position_embeddings`). Must be a list of numbers with the same length as the hidden
  size divided by the number of attention heads divided by 2
- **low_freq_factor** (`float`, *optional*) --
  Only used with 'llama3'. Scaling factor applied to low frequency components of the RoPE
- **high_freq_factor** (`float`, *optional*) --
  Only used with 'llama3'. Scaling factor applied to high frequency components of the RoPE

dict() -> new empty dictionary
dict(mapping) -> new dictionary initialized from a mapping object's
(key, value) pairs
dict(iterable) -> new dictionary initialized as if via:
d = {}
for k, v in iterable:
d[k] = v
dict(**kwargs) -> new dictionary initialized with the name=value pairs
in the keyword argument list.  For example:  dict(one=1, two=2)
