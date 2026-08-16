# PEANuT: Parameter-Efficient Adaptation with Weight-aware Neural Tweakers

[PEANuT](https://arxiv.org/abs/2410.01870) is a parameter-efficient fine-tuning technique that introduces
weight-aware neural tweakers to generate adapter updates from the frozen pretrained weights themselves. Instead of
learning a purely linear low-rank update as in LoRA, PEANuT conditions the adapter transformation on the base weight,
which makes the update rule more expressive while keeping the number of trainable parameters small.

PEANuT uses an input projection `A`, an output projection `B`, and optional intermediate residual encoder/decoder
pairs with non-linear activations. This makes it possible to model more complex update patterns than weight-agnostic
linear adapters while still remaining within the PEFT setting.

PEANuT currently has the following tradeoffs:

Pros:
- Higher theoretical expressiveness than linear low-rank updates.
- Better performance than LoRA on a range of tasks under similar budgets.
- Works well in very low-parameter regimes, for example around `0.2M` trainable parameters.

Cons:
- Higher memory usage than LoRA, because `ΔW` is explicitly constructed before being applied.
- Slower training and inference than LoRA, and deeper intermediate layers increase the overhead further.
- The non-linearity can require more careful hyperparameter tuning, especially learning rate and related optimization settings.

If these tradeoffs do not fit your use case, consider other PEFT methods such as LoRA.

The abstract from the paper is:

> Fine-tuning large pre-trained foundation models often yields excellent downstream performance but is prohibitively expensive when updating all parameters. Parameter-efficient fine-tuning (PEFT) methods such as LoRA alleviate this by introducing lightweight update modules, yet they commonly rely on weight-agnostic linear approximations, limiting their expressiveness. In this work, we propose PEANuT, a novel PEFT framework that introduces weight-aware neural tweakers, compact neural modules that generate task-adaptive updates conditioned on frozen pre-trained weights. PEANuT provides a flexible yet efficient way to capture complex update patterns without full model tuning. We theoretically show that PEANuT achieves equivalent or greater expressivity than existing linear PEFT methods with comparable or fewer parameters. Extensive experiments across four benchmarks with over twenty datasets demonstrate that PEANuT consistently outperforms strong baselines in both NLP and vision tasks, while maintaining low computational overhead.

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=PEANUT"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## PeanutConfig[[peft.PeanutConfig]]

#### peft.PeanutConfig[[peft.PeanutConfig]]

```python
peft.PeanutConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, r: int = 32, depth: int = 0, act_fn: str = 'relu', scaling: float = 1.0, target_modules: Optional[Union[list[str], str]] = None, exclude_modules: Optional[Union[list[str], str]] = None, modules_to_save: Optional[list[str]] = None, layers_to_transform: Optional[Union[list[int], int]] = None, layers_pattern: Optional[Union[list[str], str]] = None, init_weights: bool = True)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/peanut/config.py#L26)

**Parameters:**

r (`int`) : PEANuT rank. This is the hidden dimension used by the adapters. Similar to LoRA rank, larger `r` increases adapter capacity and trainable parameters.

depth (`int`) : Number of hidden adapter layers per encoder/decoder side in PEANuT. The input projection `A` and output projection `B` are always present in addition to these hidden layers. Therefore, `depth` must be a non-negative integer.  - `depth=0`: `A`, `B`. - `depth=1`: `A`, one encoder, one decoder, `B`. - `depth=2`: `A`, two encoders, two decoders, `B`. - `depth=3`: `A`, three encoders, three decoders, `B`, etc.

act_fn (`str`) : Non-linear activation applied in the PEANuT network. This corresponds to `non_linear` in the vanilla PyTorch implementation. Default is `"relu"`. Any activation key available in `transformers.activations.ACT2FN` is supported and may perform better on different tasks.

scaling (`float`) : A scalar multiplier applied to the PEANuT output before adding it to the frozen base layer output. The final adapter contribution is `scaling * (x @ delta_w)`.

target_modules (`Union[List[str], str]`, *optional*) : The names of the modules to apply PEANuT to. Can be a list of module name strings (e.g. `['q_proj', 'v_proj']`) or a regex pattern.

modules_to_save (`List[str]`, *optional*) : List of modules apart from PEANuT layers to be set as trainable and saved in the final checkpoint.

exclude_modules (`Union[List[str], str]`, *optional*) : The names of the modules to not apply the adapter. When passing a string, a regex match will be performed. When passing a list of strings, either an exact match will be performed or it is checked if the name of the module ends with any of the passed strings.

layers_to_transform (`Union[list[int], int]`, *optional*) : The layer indexes to transform. If this argument is specified, PEFT will transform only the layer indexes that are specified in this list. If a single integer is passed, PEFT will transform only the layer at this index.

layers_pattern (`Optional[Union[List[str], str]]`, *optional*) : The layer pattern name, used only if `layers_to_transform` is not None and if the layer pattern is not in the common layers pattern.

init_weights (`bool`) : Whether to initialize PEANuT adapter weights using the default initialization scheme:  - If `True`: all weights except `B` are initialized with Kaiming uniform, and `B` is initialized to zero. - If `False`: all weights (including `B`) are initialized with Kaiming uniform.  Initializing `B` to zero makes the adapter start as an exact no-op.

This is the configuration class to store the configuration of a [PeanutModel](/docs/peft/v0.20.0/en/package_reference/peanut#peft.PeanutModel).

Notes:
PEANuT uses a weight-aware pathway, where the delta weight is conditioned on the base weight. The `A` adapter
is applied over the base weight's output dimension, so `A` has shape `(out_dim -> r)` rather than the usual
`(in_dim -> r)` used by LoRA-like methods.

## PeanutModel[[peft.PeanutModel]]

#### peft.PeanutModel[[peft.PeanutModel]]

```python
peft.PeanutModel(model, peft_config: Union[PeftConfig, dict[str, PeftConfig]], adapter_name: str, low_cpu_mem_usage: bool = False, state_dict: Optional[dict[str, torch.Tensor]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/peanut/model.py#L28)

**Parameters:**

model (`torch.nn.Module`) : The model to be adapted.

config ([PeanutConfig](/docs/peft/v0.20.0/en/package_reference/peanut#peft.PeanutConfig)) : The configuration of the PEANuT model.

adapter_name (`str`) : The name of the adapter, defaults to `"default"`.

**Returns:** `torch.nn.Module`

The PEANuT PEFT model.

Creates a PEANuT model from a pretrained transformers model.

The method is described in detail in https://arxiv.org/abs/2410.01870.

**Attributes**:
- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The model to be adapted.
- **peft_config** ([PeanutConfig](/docs/peft/v0.20.0/en/package_reference/peanut#peft.PeanutConfig)): The configuration of the PEANuT model.
