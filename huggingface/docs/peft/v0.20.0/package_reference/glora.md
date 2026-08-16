# GLoRA

Generalized Low-Rank Adaptation ([GLoRA](https://huggingface.co/papers/2306.07967)) is a PEFT method that generalizes LoRA and related approaches. GLoRA decomposes updates into configurable paths (A, B, C, D, E), where each path can use low-rank, vector, constant, or disabled parameterization depending on the path.

Each path supports one of four parameterization modes. They trade off **parameter count** against **expressiveness** (how rich the update can be):

- `"lora"`: Low-rank decomposition (like standard LoRA). Uses `r * (out + in)` parameters and can express rank-`r` corrections. Most expressive, most parameters.
- `"vector"`: A single vector (e.g. shape `(out, 1)`), broadcast across the matrix. Uses `O(out)` parameters; only per-channel scaling or shifts.
- `"constant"`: A single scalar shared across all elements. Uses 1 parameter; least expressive among the trainable options.
- `"none"`: Zeros with no trainable parameters; disables that path entirely.

Not every path accepts every mode (for example, `config_D_E` does not support `"lora"`). Choosing `"lora"` on more paths increases capacity and trainable parameters; `"vector"`, `"constant"`, or `"none"` reduce both.

GLoRA is especially useful for research and advanced applications where you want to experiment with structured update patterns and combine multiple adaptation mechanisms in a single layer.

At a high level, GLoRA modifies a frozen linear layer with:

$$
W_{\mathrm{eff}} = W_0 + W_0 \odot A + B
$$

$$
b_{\mathrm{eff}} = b_0 + b_0 \odot D + E + W_0 C
$$

where each path is independently parameterized.

## GloraConfig[[peft.GloraConfig]]

#### peft.GloraConfig[[peft.GloraConfig]]

```python
peft.GloraConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, r: int = 8, target_modules: typing.Union[str, list[str], NoneType] = None, bias: str = 'none', modules_to_save: typing.Optional[list[str]] = None, config_A_B: typing.Literal['lora', 'vector', 'constant', 'none'] = 'lora', config_C: typing.Literal['lora', 'vector', 'none'] = 'lora', config_D_E: typing.Literal['vector', 'constant', 'none'] = 'constant', init_weights: bool = True)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/glora/config.py#L23)

**Parameters:**

r (`int`) : Rank of the low-rank decomposition used when a config is set to `lora`.

target_modules (`Optional[Union[List[str], str]]`) : The names of the modules to apply Glora to.

config_A_B (`str`) : Parameterization for the A and B matrices (weight multiplicative and additive corrections). Valid values: `lora`, `vector`, `constant`, `none`.

config_C (`str`) : Parameterization for the C matrix (weight-to-bias coupling: b += W0 @ C). Valid values: `lora`, `vector`, `none`.

config_D_E (`str`) : Parameterization for the D and E scalars (bias multiplicative and additive corrections). Does not support `lora` since D and E are bias-sized vectors, not matrices. Valid values: `vector`, `constant`, `none`.

init_weights (`bool`) : If True (default), initialize GLoRA as a no-op (zeros). If False, use kaiming initialization so the adapter is not a no-op.

This is the configuration class to store the configuration of a [GloraModel](/docs/peft/v0.20.0/en/package_reference/glora#peft.GloraModel).

Glora modifies a frozen linear layer W0 as: `W_eff = W0 + W0 * A + B` and `b_eff = b0 + b0 * D + E + W0 @ C`.

Each matrix (A, B, C, D, E) can be parameterized independently. The config values control how many parameters are
used and what shapes they can express:

- `lora`: Low-rank decomposition `Xd @ Xu` with shapes `(out, r)` and `(r, in)`. Uses `r * (out + in)` parameters
  and can express any rank-r correction. Like standard LoRA.
- `vector`: A single column vector of shape `(out, 1)`, broadcast across the full matrix. Uses `out` parameters;
  only per-output-channel scaling or shifts.
- `constant`: A single scalar shared across all elements. Uses 1 parameter; most constrained.
- `none`: Zeros, no trainable parameters. Effectively disables this path.

### Key Configuration Options
- `r`: Rank used when a path is configured as `"lora"` (default: `8`).
- `target_modules`: List or regex of module names to adapt (e.g., `["q_proj", "v_proj"]`).
- `config_A_B`: Path type for A and B ("lora", "vector", "constant", "none").
- `config_C`: Path type for C ("lora", "vector", "none").
- `config_D_E`: Path type for D and E ("constant", "vector", "none").
- `bias`: Bias handling (`"none"`, `"all"`, or `"glora_only"`).
- `init_weights`: If `True` (default), GLoRA is initialized as a no-op. If `False`, uses kaiming initialization.

Notes:
- `config_D_E` does not support `"lora"`.
- `target_modules` can be omitted for supported model types (PEFT default mappings are used).

## GloraModel[[peft.GloraModel]]

#### peft.GloraModel[[peft.GloraModel]]

```python
peft.GloraModel(model, peft_config: Union[PeftConfig, dict[str, PeftConfig]], adapter_name: str, low_cpu_mem_usage: bool = False, state_dict: Optional[dict[str, torch.Tensor]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/glora/model.py#L34)

Creates Generalized Low Rank Adapter (GLoRA) model from a pretrained transformers model.

- Wraps a base model and injects GLoRA adapters into the specified modules.
- Supports multiple adapters, adapter switching, merging/unmerging, and mixed-batch inference.
- Use `set_adapter`, `merge_and_unload`, and related methods for adapter management.

## GloraLayer and GloraLinear[[peft.tuners.glora.GloraLayer]]

#### peft.tuners.glora.GloraLayer[[peft.tuners.glora.GloraLayer]]

```python
peft.tuners.glora.GloraLayer(base_layer: nn.Module, **kwargs)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/glora/layer.py#L103)

#### peft.tuners.glora.GloraLinear[[peft.tuners.glora.GloraLinear]]

```python
peft.tuners.glora.GloraLinear(base_layer: nn.Module, adapter_name: str, config, **kwargs)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/glora/layer.py#L298)

GLoRA adapter wrapping a dense `~torch.nn.Linear` `base_layer`.

- `GloraLayer` is the core logic for generalized low-rank adaptation, supporting multiple adapters and flexible path configs.
- `GloraLinear` is a drop-in replacement for `nn.Linear` with GLoRA support.
- GLoRA currently supports plain `torch.nn.Linear` base layers.

## Example Usage

```python
from transformers import AutoModelForCausalLM
from peft import GloraConfig, get_peft_model

model = AutoModelForCausalLM.from_pretrained("your-model-id")
glora_config = GloraConfig(
    r=8,
    target_modules=["q_proj", "v_proj"],
    config_A_B="lora",
    config_C="vector",
    config_D_E="constant",
    task_type="CAUSAL_LM",
)
model = get_peft_model(model, glora_config)
model.print_trainable_parameters()

# Switch adapters, merge, etc.
model.set_adapter("default")
model.merge_and_unload()
```

## Notes
- GLoRA is a superset of LoRA: setting all paths to "lora" recovers standard LoRA.
- You can use different path types for A/B/C/D/E to experiment with new adaptation strategies.
- GLoRA supports all standard PEFT adapter management features (add, delete, switch, merge, etc).

## See Also
- [Adapter conceptual guide](../conceptual_guides/adapter)
- [LoRA reference](./lora)
- [Paper: https://huggingface.co/papers/2306.07967](https://huggingface.co/papers/2306.07967)
