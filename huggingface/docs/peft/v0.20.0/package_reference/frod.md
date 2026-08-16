# FRoD: Full-Rank Efficient Fine-Tuning with Rotational Degrees

FRoD is a parameter-efficient fine-tuning method that combines a shared full-rank basis with sparse learnable
rotational degrees. The adapter update is expressed through fixed projection tensors and trainable coefficients, which
allows FRoD to apply full-rank updates while keeping the number of trained parameters small.

Paper: [Full-Rank Efficient Fine-Tuning with Rotational Degrees](https://doi.org/10.1609/aaai.v40i31.39813).

When saving the adapter parameters, it is possible to avoid storing the projection tensors by setting
`save_projection=False` on the `FrodConfig`. In that case, the projections are restored from the base model weights and
the fixed random seed from `projection_prng_key`. This reduces checkpoint size, but the default is
`save_projection=True` to make checkpoint loading independent of regeneration details.

Compared to LoRA, FRoD can express a full-rank update in each adapted linear layer while training only the diagonal
coefficients and a sparse set of off-diagonal rotation coefficients. This can be useful when a low-rank update is too
restrictive. The trade-off is that FRoD computes fixed projection tensors from the base weights during adapter
injection, which makes setup more expensive and the implementation less broadly supported than LoRA.

Projection initialization can be slow on large models because FRoD runs matrix decompositions over the target module
categories before injecting the adapters. A progress bar is shown by default and can be disabled with
`FrodConfig(progressbar=False)`.

For memory-constrained training, `runtime_offload_base_weight=True` keeps target base weights on CPU when the active
FRoD path does not need them. This is opt-in because PEFT methods usually keep all base parameters on the accelerator
after moving the model and after forward passes.

FRoD currently has the following constraint:

- Only `nn.Linear` and `transformers.pytorch_utils.Conv1D` layers are supported.

## Quickstart

```python
from transformers import AutoModelForSequenceClassification

from peft import FrodConfig, TaskType, get_peft_model

model = AutoModelForSequenceClassification.from_pretrained("google-bert/bert-base-uncased", num_labels=2)

peft_config = FrodConfig(
    task_type=TaskType.SEQ_CLS,
    target_modules=["query", "value"],
    modules_to_save=["classifier"],
    sparse_rate=0.02,
    frod_dropout=0.0,
    runtime_offload_base_weight=True,
)

model = get_peft_model(model, peft_config)
model.print_trainable_parameters()
```

## FrodConfig[[peft.FrodConfig]]

#### peft.FrodConfig[[peft.FrodConfig]]

```python
peft.FrodConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, target_modules: Optional[Union[list[str], str]] = None, projection_prng_key: int = 0, save_projection: bool = True, frod_dropout: float = 0.0, fan_in_fan_out: bool = False, bias: str = 'none', modules_to_save: Optional[list[str]] = None, init_weights: bool = True, layers_to_transform: Optional[Union[list[int], int]] = None, layers_pattern: Optional[Union[list[str], str]] = None, sparse_rate: float = 0.01, regularization_alpha: float = 0.001, progressbar: bool = True, runtime_offload_base_weight: bool = False)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/frod/config.py#L24)

**Parameters:**

target_modules (`Union[List[str], str]`) : The names of the modules to apply FRoD to. Only linear layers are supported.

projection_prng_key (`int`) : Random seed used when initializing the sparse FRoD COO pattern.

save_projection (`bool`) : Whether to save the FRoD projection tensors in the state dict. This increases checkpoint size but makes adapter reloading independent of local cache regeneration. Defaults to `True`.

frod_dropout (`float`) : The dropout probability for FRoD layers.

fan_in_fan_out (`bool`) : Set this to True if the layer to replace stores weight like (fan_in, fan_out). For example, gpt-2 uses `Conv1D` which stores weights like (fan_in, fan_out) and hence this should be set to `True`.

bias (`str`) : Bias type for FRoD. Can be 'none', 'all' or 'frod_only'. If 'all' or 'frod_only', the corresponding biases will be updated during training. Be aware that this means that, even when disabling the adapters, the model will not produce the same output as the base model would have without adaptation.

modules_to_save (`List[str]`) : List of modules apart from FRoD layers to be set as trainable and saved in the final checkpoint.

init_weights (`bool`) : Whether to initialize the weights of the FRoD layers with their default initialization. Don't change this setting, except if you know exactly what you're doing.

layers_to_transform (`Union[List[int],int]`) : The layer indexes to transform, if this argument is specified, it will apply the FRoD transformations on the layer indexes that are specified in this list. If a single integer is passed, it will apply the FRoD transformations on the layer at this index.

layers_pattern (`Optional[Union[List[str], str]]`) : The layer pattern name, used only if `layers_to_transform` is different from `None`. This should target the `nn.ModuleList` of the model, which is often called `'layers'` or `'h'`.

sparse_rate (`float`) : Fraction of off-diagonal entries in the sparse trainable rotation matrix. Higher values increase capacity and trainable parameters; lower values are cheaper. Defaults to `0.01`.

regularization_alpha (`float`) : Small positive value used while building the shared basis from base weights. It stabilizes the matrix inverse when layers in the same category have correlated weights. Defaults to `1e-3`.

progressbar (`bool`) : Whether to show a progress bar while building the FRoD projections. Projection initialization can be slow on large models because it runs matrix decompositions over the target module categories. Defaults to `True`.

runtime_offload_base_weight (`bool`) : Whether to keep target base weights on CPU when the active FRoD path does not need them. This can reduce GPU memory because FRoD reconstructs the adapted weight directly, but it changes the usual PEFT convention that all base parameters stay on the accelerator after moving the model or running forward. Defaults to `False`.

This is the configuration class to store the configuration of a [FrodModel](/docs/peft/v0.20.0/en/package_reference/frod#peft.FrodModel).

Paper: https://doi.org/10.1609/aaai.v40i31.39813.

## FrodModel[[peft.FrodModel]]

#### peft.FrodModel[[peft.FrodModel]]

```python
peft.FrodModel(model, peft_config: Union[PeftConfig, dict[str, PeftConfig]], adapter_name: str, low_cpu_mem_usage: bool = False, state_dict: Optional[dict[str, torch.Tensor]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/frod/model.py#L101)
