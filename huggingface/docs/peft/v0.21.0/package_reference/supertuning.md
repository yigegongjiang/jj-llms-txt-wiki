# Super-Tuning

[Super-Tuning](https://huggingface.co/papers/2607.09287) is a sparse fine-tuning method that freezes the base weight and trains only a sparse support of scalar entries selected by weight magnitude. Setting `r` additionally allocates a LoRA-style low-rank adapter composed additively on top of the sparse support (the paper's "Supra" hybrid).

Default scoring is magnitude-only and data-free. The paper's 8B ablation reports `magnitude-topk` at 79.02% average outperforming Wanda-weighted saliency at 78.66% while requiring no calibration pass. Wanda-style activation-weighted scoring is not offered by this implementation.

Super-Tuning currently has the following constraint:

- Only `nn.Linear` layers are supported.

The abstract from the paper is:

> Fine-tuning large language models with parameter-efficient methods has become standard practice, but existing approaches like LoRA restrict the trainable subspace to a low-rank decomposition. We introduce Super-Tuning, a sparse fine-tuning approach that instead selects a small support of individual scalar weight entries — an unrestricted-rank trainable set at a fixed parameter budget. Selection is guided by pruning-inspired saliency signals: magnitude-only scoring (PaFi-style) or activation-weighted scoring (Wanda-style). We show that on Llama-3.2-1B and Meta-Llama-3-8B fine-tunes evaluated on Math17K, magnitude-based Super-Tuning matches or exceeds LoRA at comparable parameter budgets, and that a hybrid variant "Supra" — combining sparse support with a low-rank component — further improves downstream accuracy.

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=SUPERTUNING"
	frameborder="0"
	width="850"
	height="1000"
>

## Usage

**Pure Super (magnitude scoring, data-free):**

```python
from peft import SupertuningConfig, get_peft_model

config = SupertuningConfig(target_modules=["q_proj", "v_proj"], sparsity=0.99)
model = get_peft_model(base_model, config)
```

**Supra hybrid** (sparse support + LoRA composed additively):

```python
config = SupertuningConfig(
    target_modules=["q_proj", "v_proj"], sparsity=0.99,
    r=8, lora_alpha=16,   # lora_alpha defaults to 2 * r when omitted
)
model = get_peft_model(base_model, config)
```

## SupertuningConfig[[peft.SupertuningConfig]]

#### peft.SupertuningConfig[[peft.SupertuningConfig]]

```python
peft.SupertuningConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, target_modules: Optional[Union[list[str], str]] = None, modules_to_save: Optional[list[str]] = None, sparsity: float = 0.99, select_top: bool = True, r: Optional[int] = None, lora_alpha: Optional[float] = None, lora_dropout: float = 0.0, init_weights: bool = True, save_precomputed_indices: bool = True)
```

[Source](https://github.com/huggingface/peft/blob/v0.21.0/src/peft/tuners/supertuning/config.py#L26)

**Parameters:**

target_modules (`Optional[Union[List[str], str]]`) : The names of the modules to apply the adapter to. String → regex match; list → suffix / exact match; None → model-architecture default.

modules_to_save (`Optional[List[str]]`) : Modules outside the Supertuning layers that should also be trainable and saved in the final checkpoint (e.g. randomly-initialized classifier heads).

sparsity (`float`) : Target sparsity ratio in `[0.0, 1.0)`. `0.99` = 1% of weight entries are trainable. Defaults to `0.99`.

select_top (`bool`) : Which end of the magnitude score to keep as the trainable support. `True` keeps the largest-magnitude entries (paper's Super / Supra); `False` keeps the smallest (paper's `-bottom` variants). The paper reports that the best direction is model- and task-dependent. Defaults to `True`.

r (`Optional[int]`) : LoRA rank for the "Supra" hybrid. When `None` (default), only the sparse support is trainable (Super mode). When set to a positive integer, additionally allocates LoRA `A` (`[r, in_features]`) and `B` (`[out_features, r]`) parameters whose contribution is added to the sparse support in the forward pass.

lora_alpha (`Optional[float]`) : LoRA scaling factor for Supra mode. If `None` and `r` is set, defaults to `2 * r`. Ignored when `r is None`.

lora_dropout (`float`) : LoRA dropout probability for Supra mode. Defaults to `0.0`. Ignored when `r is None`.

init_weights (`bool`) : When `True` (default), the sparse `values` are zero-initialised and LoRA `B` is zero-initialised — the adapter is an identity update at construction. When `False`, both are Kaiming-uniform (used by tests to exercise a non-trivial adapter). LoRA `A` uses Kaiming-uniform in both cases.

save_precomputed_indices (`bool`) : Whether to save the sparse-support indices in the state dict. Defaults to `True`. Set to `False` to trim checkpoint size — indices will be reconstructed deterministically from the base weight magnitudes at load time. Reconstruction assumes the base model weights are identical to those used at training time; small numerical drift can cause topk tie-breaks to differ.

Configuration class for [SupertuningModel](/docs/peft/v0.21.0/en/package_reference/supertuning#peft.SupertuningModel).

Super-Tuning (arXiv:2607.09287) freezes the base weight and trains only a sparse support of scalar entries,
selected by weight magnitude. Setting `r` additionally allocates a LoRA-style low-rank adapter on top of the sparse
support (the paper's "Supra" hybrid).

The default (magnitude scoring, `r=None`) reproduces the paper's best-reported single-mechanism configuration: on
Meta-Llama-3-8B, `magnitude-topk` at 79.02% average beats `Wanda` at 78.66% AND requires no calibration pass.
Wanda-style activation-weighted scoring is not offered by this implementation.

Paper: https://arxiv.org/abs/2607.09287

## SupertuningModel[[peft.SupertuningModel]]

#### peft.SupertuningModel[[peft.SupertuningModel]]

```python
peft.SupertuningModel(model, peft_config: Union[PeftConfig, dict[str, PeftConfig]], adapter_name: str, low_cpu_mem_usage: bool = False, state_dict: Optional[dict[str, torch.Tensor]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.21.0/src/peft/tuners/supertuning/model.py#L25)

**Parameters:**

model ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel)) : The base model to adapt.

config ([SupertuningConfig](/docs/peft/v0.21.0/en/package_reference/supertuning#peft.SupertuningConfig)) : The Supertuning configuration.

adapter_name (`str`) : The adapter name. Defaults to `"default"`.

low_cpu_mem_usage (`bool`, *optional*) : Create empty adapter weights on the meta device to speed up loading.

**Returns:** `torch.nn.Module`

The Supertuning-wrapped model.

Super-Tuning tuner (arXiv:2607.09287).

Freezes the base weights and trains only a sparse support of scalar entries selected by weight magnitude (paper's
best single-mechanism configuration; data-free). When `config.r` is set, additionally allocates LoRA A/B parameters
composed additively with the sparse support — the paper's Supra hybrid.

Example (pure Super):

```py
>>> from transformers import AutoModelForCausalLM
>>> from peft import SupertuningConfig, get_peft_model

>>> base = AutoModelForCausalLM.from_pretrained("meta-llama/Llama-3.2-1B")
>>> config = SupertuningConfig(target_modules=["q_proj", "v_proj"], sparsity=0.99)
>>> model = get_peft_model(base, config)
```

Example (Supra hybrid):

```py
>>> config = SupertuningConfig(
...     target_modules=["q_proj", "v_proj"],
...     sparsity=0.99,
...     r=8,
...     lora_alpha=16,
... )
>>> model = get_peft_model(base, config)
```

Paper: https://arxiv.org/abs/2607.09287
