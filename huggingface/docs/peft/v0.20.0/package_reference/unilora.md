# UniLoRA

[Uni-LoRA](https://huggingface.co/papers/2506.00799) is a PEFT method that shares a compact trainable
vector bank across low-rank adapter weights. Instead of learning every LoRA matrix element independently, UniLoRA
deterministically projects entries into shared `theta_d` values and learns the shared parameters used by the adapter
update.

## Quick Start

```python
from peft import UniLoraConfig, get_peft_model
from transformers import AutoModelForCausalLM

model = AutoModelForCausalLM.from_pretrained("meta-llama/Llama-3.2-3B")

config = UniLoraConfig(
    r=32,
    theta_d_length=256,
    proj_seed=42,
    target_modules=["q_proj", "v_proj"],
    unilora_dropout=0.0,
    init_weights=True,
    task_type="CAUSAL_LM",
)

peft_model = get_peft_model(model, config)
peft_model.print_trainable_parameters()
```

## Important Parameters

`r` controls the low-rank adapter dimension. Larger values increase adapter capacity and memory use.

`theta_d_length` controls the length of the shared UniLoRA vector bank. This is the main trainable storage shared by
the projected adapter entries.

`proj_seed` controls deterministic index generation for the fixed projections into `theta_d`. Reusing the same seed and
configuration makes the generated adapter indices reproducible.

`target_modules` selects which modules receive UniLoRA adapters. Use module suffixes such as `["q_proj", "v_proj"]`, a
regex string, or `"all-linear"` when supported by the model architecture.

`unilora_dropout` applies dropout inside UniLoRA adapter layers during training.

`init_weights` controls UniLoRA parameter initialization. Set it to `False` to keep a random `theta_d`
initialization when you need to manage initialization manually.

`save_indices` controls whether UniLoRA checkpoints save the generated index and scale tensors together with the
shared `theta_d` parameters. Keeping this disabled gives smaller checkpoints and regenerates indices from
`proj_seed`; enabling it makes saved adapters independent from future index-generation changes.

## Benchmark overview

<iframe
src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=UNILORA"
frameborder="0"
width="850"
height="1000"
>

# API

## UniLoraConfig[[peft.UniLoraConfig]]

#### peft.UniLoraConfig[[peft.UniLoraConfig]]

```python
peft.UniLoraConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, r: int = 4, proj_seed: int = 42, theta_d_length: int = 256, target_modules: typing.Union[str, list[str], NoneType] = None, unilora_dropout: float = 0.0, fan_in_fan_out: bool = False, bias: str = 'none', modules_to_save: typing.Optional[list[str]] = None, init_theta_d_bound: float = 0.02, init_weights: bool = True, save_indices: bool = False, layers_to_transform: typing.Union[list[int], int, NoneType] = None, layers_pattern: typing.Union[str, list[str], NoneType] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/unilora/config.py#L23)

**Parameters:**

r (`int`) : Rank of the low-rank adaptation. This controls the expressive capacity of the UniLora update.

proj_seed (`int`) : Random seed used to generate the fixed index assignment. This ensures reproducibility across runs.

theta_d_length (`int`) : Length of the shared UniLora vector `theta_d`.

target_modules (`Union[list[str], str]`, *optional*) : Names or patterns of modules to which UniLora adapters are applied. - If a string is provided, it is treated as a regular expression. - If a list is provided, modules are matched by exact name or suffix. - The special value 'all-linear' applies UniLora to all Linear / Conv1D layers except the output layer. If not specified, modules are inferred from the model architecture. An error is raised if the architecture is unsupported.

unilora_dropout (`float`) : Dropout probability applied within UniLora layers.

fan_in_fan_out (`bool`) : Whether the replaced layer stores weights in (fan_in, fan_out) format. This should be set to True for models such as GPT-2 that use Conv1D layers.

bias (`str`) : Specifies which bias terms are trainable: - 'none': no bias parameters are updated - 'all': all bias parameters are updated - 'unilora_only': only biases inside UniLora layers are updated Note: enabling bias updates changes model outputs even when adapters are disabled.

modules_to_save (`list[str]`, *optional*) : Additional modules (outside UniLora layers) that should remain trainable and be saved in the final checkpoint. This is commonly used for task-specific heads such as classifiers.

init_theta_d_bound (`float`) : Initialization bound for the UniLora vector bank. Vectors are sampled uniformly from [-init_theta_d_bound, init_theta_d_bound]. Initializing with zeros is avoided to prevent vanishing gradients. Small values (e.g., 0.02) are recommended for stable training.

init_weights (`bool`) : Whether to initialize `theta_d` with the default UniLora initialization. If set to `False`, `theta_d` keeps a random initialization.

save_indices (`bool`) : Whether to save the generated UniLora index and scale buffers alongside `theta_d`. This increases checkpoint size, but makes saved adapters independent from future changes to the index generation routine.

layers_to_transform (`Union[list[int], int]`, *optional*) : Indices of transformer layers to which UniLora is applied. If specified, only these layers are modified. This option is valid only when `target_modules` is a list.

layers_pattern (`Union[list[str], str]`, *optional*) : Custom layer name pattern used together with `layers_to_transform` when the model does not follow standard layer naming conventions. This option is valid only when `target_modules` is a list.

Configuration class for UniLora adapters.

This class defines all hyperparameters required to initialize and apply UniLora layers within the PEFT framework.
The configuration is intentionally minimal and only includes parameters that are actively used by the current
UniLora implementation.

Reference:
Uni-LoRA: One Vector Is All You Need https://arxiv.org/abs/2506.00799

## UniLoraModel[[peft.UniLoraModel]]

#### peft.UniLoraModel[[peft.UniLoraModel]]

```python
peft.UniLoraModel(model, config, adapter_name, low_cpu_mem_usage: bool = False, state_dict: dict[str, torch.Tensor] | None = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/unilora/model.py#L31)

Creates a UniLora adapter around a pretrained model.

#### generate_index[[peft.UniLoraModel.generate_index]]

```python
generate_index(lora_param_count: int, theta_d_length: int, proj_seed: int)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/unilora/model.py#L127)

Assign deterministic `theta_d` indices to the flattened UniLora parameter space.

A plain `np.random.choice(np.arange(theta_d_length), size=lora_param_count)` samples each position
independently, which can leave some `theta_d` entries unused for smaller adapters. UniLora instead uses a
balanced deterministic assignment: each index appears either `floor(D / d)` or `ceil(D / d)` times, where `D`
is the flattened LoRA parameter count and `d` is `theta_d_length`. This keeps per-index normalization stable
while still shuffling the assignment with `proj_seed`.
