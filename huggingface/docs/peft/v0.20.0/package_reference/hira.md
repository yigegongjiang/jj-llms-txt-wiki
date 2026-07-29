# HiRA

High-Rank Adaptation ([HiRA](https://openreview.net/pdf?id=TwJrTz9cRS)) is a PEFT method that extends the LoRA approach by applying an element-wise modulation on the original weight matrix. Instead of adding a low-rank update directly, HiRA computes:

$$
W' = W_0 + W_0 \odot (B A)
$$

where $W_0$ is the base weight, and $A, B$ are low-rank factors with rank $r \ll \min(	\text{in_features}, \text{out_features})$. This formulation allows HiRA to adapt existing weights with a multiplicative, input-dependent modulation, often improving fine-tuning efficiency on downstream tasks.

The abstract from the HiRA paper is:

> *We propose Hadamard High-Rank Adaptation (HiRA), a parameter-efficient fine-tuning (PEFT) method that enhances the adaptability of Large Language Models (LLMs). While Low-rank Adaptation (LoRA) is widely used to reduce resource demands, its low-rank updates may limit its expressiveness for new tasks. HiRA addresses this by using a Hadamard product to retain high-rank update parameters, improving the model capacity. Empirically, HiRA outperforms LoRA and its variants on several tasks, with extensive ablation studies validating its effectiveness.*

## Examples

```python
from transformers import AutoModelForCausalLM, AutoTokenizer
from peft import get_peft_model
from peft.tuners.hira import HiraConfig

# Example 1: HiRA on opt-125m for causal language modeling
model_id = "facebook/opt-125m"
base_model = AutoModelForCausalLM.from_pretrained(model_id)
tokenizer = AutoTokenizer.from_pretrained(model_id)

# Define HiRA configuration: apply to the MLP dense layers in each transformer block
hira_config = HiraConfig(
    r=32,
    target_modules=["k_proj", "q_proj", "v_proj", "fc1", "fc2"],
    hira_dropout=0.0,
    init_weights=True,
)
peft_model = get_peft_model(base_model, hira_config)

peft_model.print_trainable_parameters()
# trainable params: 4,718,592 || all params: 129,957,888 || trainable%: 3.6309
```

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=HIRA"
	frameborder="0"
	width="850"
	height="1000"
>

## Citation:

If you found HiRA is useful, please cite HiRA as:
```
@inproceedings{
huang2025hira,
title={Hi{RA}: Parameter-Efficient Hadamard High-Rank Adaptation for Large Language Models},
author={Qiushi Huang and Tom Ko and Zhan Zhuang and Lilian Tang and Yu Zhang},
booktitle={The Thirteenth International Conference on Learning Representations},
year={2025},
url={https://openreview.net/forum?id=TwJrTz9cRS}
}
```

# API

## HiraConfig[[peft.HiraConfig]]

"}]}>
- **r** (`int`) --
  Rank of the low-rank component in HiRA. Although HiRA achieves a high-rank adaptation through Hadamard
  fusion, this value defines the dimension of the underlying low-rank factorization (matrices A and B).
- **target_modules** (`Optional[Union[List[str], str]]`) --
  The names of the modules to apply the adapter to. If this is specified, only the modules with the specified
  names will be replaced. When passing a string, a regex match will be performed. When passing a list of
  strings, either an exact match will be performed or it is checked if the name of the module ends with any
  of the passed strings. If this is specified as 'all-linear', then all linear/Conv1D modules are chosen (if
  the model is a PreTrainedModel, the output layer excluded). If this is not specified, modules will be
  chosen according to the model architecture. If the architecture is not known, an error will be raised -- in
  this case, you should specify the target modules manually.
- **exclude_modules** (`Optional[Union[List[str], str]]`) --
  The names of the modules to not apply the adapter. When passing a string, a regex match will be performed.
  When passing a list of strings, either an exact match will be performed or it is checked if the name of the
  module ends with any of the passed strings.
- **hira_dropout** (`float`) --
  The dropout probability for HiRA layers.
- **fan_in_fan_out** (`bool`) --
  Set this to True if the layer to replace stores weight like (fan_in, fan_out). For example, gpt-2 uses
  `Conv1D` which stores weights like (fan_in, fan_out) and hence this should be set to `True`.
- **modules_to_save** (`List[str]`) --
  List of modules apart from adapter layers to be set as trainable and saved in the final checkpoint.
- **init_weights** (`bool` | `Literal["gaussian"]`) --
  How to initialize the weights of the HiRA layers. Passing True (default) results in the default
  initialization, with the HiRA B weight being set to 0. This means that without further training, the HiRA
  adapter will be a no-op. Setting the initialization to False leads to random initialization of HiRA A and
  B, meaning that HiRA is not a no-op before training; this setting is intended for debugging purposes.
  Passing `'gaussian'` results in Gaussian initialization scaled by the HiRA rank for linear and layers.
- **layers_to_transform** (`Union[List[int], int]`) --
  The layer indices to transform. If a list of ints is passed, it will apply the adapter to the layer indices
  that are specified in this list. If a single integer is passed, it will apply the transformations on the
  layer at this index.
- **layers_pattern** (`Optional[Union[List[str], str]]`) --
  The layer pattern name, used only if `layers_to_transform` is different from `None`. This should target the
  `nn.ModuleList` of the model, which is often called `'layers'` or `'h'`.
- **rank_pattern** (`dict`) --
  The mapping from layer names or regexp expression to ranks which are different from the default r specified
  by `r`. For example, `{'^model.decoder.layers.0.encoder_attn.k_proj': 16}`.

This is the configuration class to store the configuration of a `HiraModel`.

## Core Layers

### HiraLayer[[peft.tuners.hira.HiraLayer]]

### Linear Adapter[[peft.tuners.hira.Linear]]

- **adapter** (str) --
  The name of the adapter for which the delta weight should be computed.

Compute the delta weight for the given adapter.

- **safe_merge** (`bool`, *optional*) --
  If True, the merge operation will be performed in a copy of the original weights and check for NaNs
  before merging the weights. This is useful if you want to check if the merge operation will produce
  NaNs. Defaults to `False`.
- **adapter_names** (`list[str]`, *optional*) --
  The list of adapter names that should be merged. If None, all active adapters will be merged. Defaults
  to `None`.

Merge the active adapter weights into the base weights

This method unmerges all merged adapter layers from the base weights.

### Embedding Adapter[[peft.tuners.hira.Embedding]]

HiRA forward for Embedding layer. Supports mixed adapters per batch or single adapter.

- **adapter** (str) --
  The name of the adapter for which the delta weight should be computed.

Compute the delta weight for the given adapter.

- **safe_merge** (`bool`, *optional*) --
  If True, the merge operation will be performed in a copy of the original weights and check for NaNs
  before merging the weights. This is useful if you want to check if the merge operation will produce
  NaNs. Defaults to `False`.
- **adapter_names** (`list[str]`, *optional*) --
  The list of adapter names that should be merged. If None, all active adapters will be merged. Defaults
  to `None`.

Merge the active adapter weights into the base weights

This method unmerges all merged adapter layers from the base weights.

### Convolutional Adapters

[[autodoc]] tuners.hira.layer.Conv1d [[autodoc]] tuners.hira.layer.Conv2d [[autodoc]] tuners.hira.layer.ConvNd
