# DEFT: Decompositional Efficient Fine-Tuning for Text-to-Image Models

[DEFT](https://proceedings.neurips.cc/paper_files/paper/2025/hash/93a34a7138bdad95e874018d5f491cc6-Abstract-Conference.html)
(Decompositional Efficient Fine-Tuning) is a parameter-efficient fine-tuning method for text-to-image models. It
decomposes the update of a frozen weight matrix `W` into two trainable components: a projection that removes a low-rank
subspace from `W`, and a low-rank update that injects new content into that subspace. This formulation is designed to
balance aligning with a target distribution, learning new concepts from a few images (personalization), and preserving
the pretrained model's instruction-following ability and editability.

Concretely, DEFT combines two trainable low-rank components: (1) a projection onto the complement of a low-rank
subspace spanned by a low-rank matrix, and (2) a low-rank update. The first low-rank matrix defines the subspace, while
the second enables flexible parameter adaptation within that subspace.

When to use DEFT: it is best suited to adapting a model to new data or concepts while **retaining and even improving the
base model's instruction-following ability** and keeping **forgetting of its previous capabilities to a minimum**.

Per target layer, DEFT learns a projection direction `P` (shape `out_features x r`) and an injection matrix `R` (shape
`r x in_features`). The effective weight is the residual projection

```
W' = (I - P_proj) @ W + Q_P @ R
```

The projector `P_proj` is derived from `P` according to `decomposition_method`:

- `"relu"` (default): `Q_P = P`, `P_proj = P @ relu(P).T` — a non-orthogonal projection.
- `"qr"`: `Q_P = qr(P)`, `P_proj = Q_P @ Q_P.T` — an orthogonal projection.

The `(I - P_proj) @ W` term removes a sub-space of the pretrained weight while `Q_P @ R` injects new content into it.
By default (`init_weights=True`) `R` is initialized so that the update is an exact identity at initialization
(`W' == W`), so training starts from the pretrained weights and learns the injection. The update is equivalent to a
low-rank additive delta `Q_P @ (R - right.T @ W)`, which is computed without ever forming the `out x out`
projection matrix and can be merged into the base weights for inference-free deployment.

Setting `para=True` selects the
[PaRa](https://proceedings.iclr.cc/paper_files/paper/2025/hash/f09e8dd9274cb7c2dd0dc65ffc6f427a-Abstract-Conference.html)
(Parameter Rank Reduction) variant: a removal-only update `W' = (I - P_proj) @ W` that keeps just the subspace-removal
term and drops the injection. Only the projection `P` is trained (no injection matrix `R`), so the adapter is not an
identity at initialization. PaRa was introduced for personalizing text-to-image diffusion models and is available here
as a special case of DEFT.

DEFT is currently implemented for `torch.nn.Linear` and `Conv1D` (e.g. gpt-2, via `fan_in_fan_out`) layers. The original implementation and the experiments from the
paper (Dreambooth, Dreambench Plus, InsDet, VisualCloze, on Stable Diffusion and a unified model) are available at
[github.com/MAXNORM8650/DEFT](https://github.com/MAXNORM8650/DEFT).

If you use DEFT in your work, please cite the paper:

```bibtex
@article{kumar2026deft,
  title={DEFT: Decompositional Efficient Fine-Tuning for Text-to-Image Models},
  author={Kumar, Komal and Anwer, Rao and Shahbaz Khan, Fahad and Khan, Salman and Laptev, Ivan and Cholakkal, Hisham},
  journal={Advances in Neural Information Processing Systems},
  volume={38},
  pages={102009--102035},
  year={2026}
}
```

If you use the PaRa variant (`para=True`), please also cite:

```bibtex
@inproceedings{chen2025personalizing,
  title={Para: Personalizing text-to-image diffusion via parameter rank reduction},
  author={Chen, Shangyu and Pan, Zizheng and Cai, Jianfei and Phung, Dinh},
  booktitle={International Conference on Learning Representations},
  year={2025}
}
```

## DeftConfig[[peft.DeftConfig]]

#### peft.DeftConfig[[peft.DeftConfig]]

```python
peft.DeftConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, r: int = 8, target_modules: Optional[Union[list[str], str]] = None, exclude_modules: Optional[Union[list[str], str]] = None, decomposition_method: Literal['relu', 'qr'] = 'relu', init_scale: float = 1.0, alpha: Optional[int] = None, para: bool = False, fan_in_fan_out: bool = False, deft_dropout: float = 0.0, init_weights: bool = True, layers_to_transform: Optional[Union[list[int], int]] = None, layers_pattern: Optional[Union[list[str], str]] = None, bias: str = 'none', modules_to_save: Optional[list[str]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/deft/config.py#L25)

**Parameters:**

r (`int`) : The rank of the DEFT projection/injection across layers.

target_modules (`Optional[Union[List[str], str]]`) : The names of the modules to apply the adapter to. If this is specified, only the modules with the specified names will be replaced. When passing a string, a regex match will be performed. When passing a list of strings, either an exact match will be performed or it is checked if the name of the module ends with any of the passed strings. If this is specified as 'all-linear', then all linear modules are chosen, excluding the output layer. If this is not specified, modules will be chosen according to the model architecture. If the architecture is not known, an error will be raised -- in this case, you should specify the target modules manually.

exclude_modules (`Optional[Union[List[str], str]]`) : The names of the modules to not apply the adapter. When passing a string, a regex match will be performed. When passing a list of strings, either an exact match will be performed or it is checked if the name of the module ends with any of the passed strings.

decomposition_method (`str`) : How the projector `P_proj` is derived from `P`. Either `"relu"` (default, non-orthogonal `P @ relu(P).T`) or `"qr"` (orthogonal `Q_P @ Q_P.T`).

init_scale (`float`) : Scaling applied to the standard deviation used to initialize the injection matrix `R` (only used when `init_weights=False`). Smaller values keep the injected update closer to zero at initialization. Defaults to `1.0`.

alpha (`Optional[int]`) : The scaling factor for the injection term, which is scaled by `alpha / r` (analogous to LoRA's alpha). If `None`, no scaling is applied (factor `1.0`). The subspace-removal term is unaffected.

para (`bool`) : Whether to use the PaRa method: pure subspace removal (`delta = -P_proj @ W`) with no injection term. When `True`, `R` is not created, `P` is the only trainable matrix, and the adapter cannot be an identity at init. Defaults to `False` (full DEFT).

fan_in_fan_out (`bool`) : Set this to `True` if the layer to replace stores weight like (fan_in, fan_out). For example, gpt-2 uses `Conv1D` which stores weights like (fan_in, fan_out) and hence this should be set to `True`.

deft_dropout (`float`) : The dropout probability applied to the layer input. Defaults to `0.0`.

init_weights (`bool`) : Whether to use DEFT's default (identity) initialization for the adapter weights, so the adapter is a no-op at the start of training. Don't change this setting, except if you know exactly what you're doing. Defaults to `True`.

layers_to_transform (`Union[List[int], int]`) : The layer indices to transform. If a list of ints is passed, it will apply the adapter to the layer indices that are specified in this list. If a single integer is passed, it will apply the transformations on the layer at this index.

layers_pattern (`Optional[Union[List[str], str]]`) : The layer pattern name, used only if `layers_to_transform` is different from `None`. This should target the `nn.ModuleList` of the model, which is often called `'layers'` or `'h'`.

bias (`str`) : Bias type for DEFT. Can be `'none'`, `'all'` or `'deft_only'`.

modules_to_save (`List[str]`) : List of modules apart from adapter layers to be set as trainable and saved in the final checkpoint.

This is the configuration class to store the configuration of a [DeftModel](/docs/peft/v0.20.0/en/package_reference/deft#peft.DeftModel).

DEFT (Decompositional Efficient Fine-Tuning) performs knowledge injection through a residual-projection update. For
a frozen base weight `W`, a low-rank projection direction `P` (shape `out_features x r`) and an injection matrix
`R` (shape `r x in_features`) are learned. The adapted weight is `W' = (I - P_proj) @ W + Q_P @ R`, where the
projector `P_proj` is derived from `P` according to `decomposition_method`:

- `"relu"` (default): `Q_P = P`, `P_proj = P @ relu(P).T` (non-orthogonal projection)
- `"qr"`: `Q_P = qr(P)`, `P_proj = Q_P @ Q_P.T` (orthogonal projection)

By default (`init_weights=True`) `R` is initialized so the update is an exact identity at init (the adapted weight
equals `W`), so training starts from the pretrained weights and learns the injection.

## DeftModel[[peft.DeftModel]]

#### peft.DeftModel[[peft.DeftModel]]

```python
peft.DeftModel(model, peft_config: Union[PeftConfig, dict[str, PeftConfig]], adapter_name: str, low_cpu_mem_usage: bool = False, state_dict: Optional[dict[str, torch.Tensor]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/deft/model.py#L27)

**Parameters:**

model (`torch.nn.Module`) : The model to which the adapter tuner layers will be attached.

config ([DeftConfig](/docs/peft/v0.20.0/en/package_reference/deft#peft.DeftConfig)) : The configuration of the DEFT model.

adapter_name (`str`) : The name of the adapter, defaults to `"default"`.

low_cpu_mem_usage (`bool`, `optional`, defaults to `False`) : Create empty adapter weights on meta device. Useful to speed up the loading process.

**Returns:** `torch.nn.Module`

The DEFT model.

Creates a DEFT (Decompositional Efficient Fine-Tuning) model from a pretrained model.

DEFT freezes the base weights and learns, per target module, a low-rank projection direction `P` and an injection
matrix `R`. The effective weight becomes `(I - P_proj) @ W + Q_P @ R`, replacing a sub-space of `W` with newly
injected content (see [DeftConfig](/docs/peft/v0.20.0/en/package_reference/deft#peft.DeftConfig) for the available `decomposition_method` variants).

**Attributes**:
- **model** (`~torch.nn.Module`) -- The model to be adapted.
- **peft_config** ([DeftConfig](/docs/peft/v0.20.0/en/package_reference/deft#peft.DeftConfig)): The configuration of the DEFT model.
