# VB-LoRA: Extreme Parameter Efficient Fine-Tuning with Vector Banks

## Overview

[VB-LoRA](https://huggingface.co/papers/2405.15179) is a parameter-efficient fine-tuning technique that extends LoRA by learning a fine-grained parameter-sharing scheme at the sub-vector level, achieving significantly higher parameter efficiency. This makes VB-LoRA especially useful in scenarios where storage and transmission costs are critical. It works by decomposing low-rank matrices—from different layers and modules such as K, Q, V, and FFN—into sub-vectors, which are then globally shared through a vector bank.

The abstract from the paper is:

*As the adoption of large language models increases and the need for per-user or per-task model customization grows, the parameter-efficient fine-tuning (PEFT) methods, such as low-rank adaptation (LoRA) and its variants, incur substantial storage and transmission costs. To further reduce stored parameters, we introduce a "divide-and-share" paradigm that breaks the barriers of low-rank decomposition across matrix dimensions, modules and layers by sharing parameters globally via a vector bank. As an instantiation of the paradigm to LoRA, our proposed VB-LoRA composites all the low-rank matrices of LoRA from a shared vector bank with a differentiable top-k admixture module. VB-LoRA achieves extreme parameter efficiency while maintaining comparable or better performance compared to state-of-the-art PEFT methods. Extensive experiments demonstrate the effectiveness of VB-LoRA on natural language understanding, natural language generation, and instruction tuning tasks. When fine-tuning the Llama2-13B model, VB-LoRA only uses 0.4% of LoRA's stored parameters, yet achieves superior results.*

## Usage Tips

- VB-LoRA utilizes a sparse top-k module to learn the sharing mechanism. When saving adapter parameters, you can either save only the top-k weights and their indices by setting `save_only_topk_weights = True` in `VBLoRAConfig`, or save all the trainable logits by setting it to `False`. Enabling `save_only_topk_weights = True` significantly reduces storage space; for instance, in Llama2-7B, the storage file size decreases from 308MB to 2.5MB. Note that models saved with `save_only_topk_weights = True` are intended for merging or inference only and cannot be used to resume training.

- VB-LoRA has two sets of training parameters: vector bank parameters and logit parameters. In practice, we found that logit parameters require a higher learning rate, while vector bank parameters require a lower learning rate. When using the AdamW optimizer, typical learning rates are 0.01 for logits and 0.001 for vector bank parameters.

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=VBLORA"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## VBLoRAConfig[[peft.VBLoRAConfig]]

#### peft.VBLoRAConfig[[peft.VBLoRAConfig]]

```python
peft.VBLoRAConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, r: int = 4, num_vectors: int = 256, vector_length: int = 256, topk: int = 2, target_modules: Optional[Union[list[str], str]] = None, exclude_modules: Optional[Union[list[str], str]] = None, save_only_topk_weights: bool = False, vblora_dropout: float = 0.0, fan_in_fan_out: bool = False, bias: str = 'none', modules_to_save: Optional[list[str]] = None, init_vector_bank_bound: float = 0.02, init_logits_std: float = 0.1, layers_to_transform: Optional[Union[list[int], int]] = None, layers_pattern: Optional[Union[list[str], str]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/vblora/config.py#L25)

**Parameters:**

r (`int`) : The rank of incremental matrices.

num_vectors (`int`) : Number of vectors in the vector bank. Use higher values when the model size increases.

vector_length (`int`) : The length of the vectors in the vector bank. The length of the vectors should be divisible by the hidden dimension of the model.

topk (`int`) : The K value for top-K selection. A larger value of K increases the size of the saved model. In practice, setting K=2 typically provides the best performance and parameter efficiency. For more details, refer to the discussion in the paper.

target_modules (`Union[List[str], str]`) : The names of the modules to apply the adapter to. If this is specified, only the modules with the specified names will be replaced. When passing a string, a regex match will be performed. When passing a list of strings, either an exact match will be performed or it is checked if the name of the module ends with any of the passed strings. If this is specified as 'all-linear', then all linear/Conv1D modules are chosen, excluding the output layer. If this is not specified, modules will be chosen according to the model architecture. If the architecture is not known, an error will be raised -- in this case, you should specify the target modules manually.

exclude_modules (`Optional[Union[List[str], str]]`) : The names of the modules to not apply the adapter. When passing a string, a regex match will be performed. When passing a list of strings, either an exact match will be performed or it is checked if the name of the module ends with any of the passed strings.

save_only_topk_weights (`bool`) : Whether to only save the topk weights. Setting `save_only_topk_weights = True` significantly reduces storage space. However, models saved in this mode can be used for merging or inference only, not for resuming training.

vblora_dropout (`float`) : The dropout probability for VBLoRA layers.

fan_in_fan_out (`bool`) : Set this to True if the layer to replace stores weight like (fan_in, fan_out). For example, gpt-2 uses `Conv1D` which stores weights like (fan_in, fan_out) and hence this should be set to `True`.

bias (`str`) : Bias type for VBLoRA. Can be 'none', 'all' or 'vblora_only'. If 'all' or 'vblora_only', the corresponding biases will be updated during training. Be aware that this means that, even when disabling the adapters, the model will not produce the same output as the base model would have without adaptation.

modules_to_save (`List[str]`) : List of modules apart from VBLoRA layers to be set as trainable and saved in the final checkpoint.

init_vector_bank_bound (`float`) : The vector bank is initialized with a uniform distribution between -init_vector_bank_bound and init_vector_bank_bound. Avoid initializing the vector bank with all zeros to prevent zero gradients. A small value, such as 0.02, is typically effective. Initializing with a large value may cause training instability.

init_logits_std (`float`) : The logits are initialized with a normal distribution with a standard deviation of init_logits_std. Default is 0.1.

layers_to_transform (`Union[List[int],int]`) : The layer indices to transform. If a list of ints is passed, it will apply the adapter to the layer indices that are specified in this list. If a single integer is passed, it will apply the transformations on the layer at this index.

layers_pattern (`Optional[Union[List[str], str]]`) : The layer pattern name, used only if `layers_to_transform` is different from `None`. This should target the `nn.ModuleList` of the model, which is often called `'layers'` or `'h'`.

This is the configuration class to store the configuration of a [VBLoRAModel](/docs/peft/v0.20.0/en/package_reference/vblora#peft.VBLoRAModel).

Paper: https://huggingface.co/papers/2405.15179

## VBLoRAModel[[peft.VBLoRAModel]]

#### peft.VBLoRAModel[[peft.VBLoRAModel]]

```python
peft.VBLoRAModel(model, peft_config: Union[PeftConfig, dict[str, PeftConfig]], adapter_name: str, low_cpu_mem_usage: bool = False, state_dict: Optional[dict[str, torch.Tensor]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/vblora/model.py#L29)

**Parameters:**

model ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) : The model to be adapted.

config ([VBLoRAConfig](/docs/peft/v0.20.0/en/package_reference/vblora#peft.VBLoRAConfig)) : The configuration of the VBLoRA model.

adapter_name (`str`) : The name of the adapter, defaults to `"default"`.

low_cpu_mem_usage (`bool`, `optional`, defaults to `False`) : Create empty adapter weights on meta device. Useful to speed up the loading process.

**Returns:** `torch.nn.Module`

The VBLoRA model.

Creates VBLoRA model from a pretrained transformers model.

The method is described in detail in https://huggingface.co/papers/2405.15179.

Example:

```py
>>> from transformers import AutoModelForCausalLM
>>> from peft import VBLoRAConfig, get_peft_model

>>> base_model = AutoModelForCausalLM.from_pretrained("facebook/opt-125m")
>>> config = VBLoRAConfig(
...     task_type="SEQ_CLS",
...     r=4,
...     target_modules=["fc1", "fc2", "k_proj", "out_proj", "q_proj", "v_proj"],
...     num_vectors=60,
...     vector_length=256,
...     save_only_topk_weights=True,
... )
>>> model = get_peft_model(base_model, config)
```

**Attributes**:
- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The model to be adapted.
- **peft_config** ([VBLoRAConfig](/docs/peft/v0.20.0/en/package_reference/vblora#peft.VBLoRAConfig)): The configuration of the VBLoRAConfig model.

#### get_nb_savable_parameters[[peft.VBLoRAModel.get_nb_savable_parameters]]

```python
get_nb_savable_parameters(adapter = 'default')
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/vblora/model.py#L157)

Returns the number of savable VB-LoRA parameters and other savable parameters.

#### print_savable_parameters[[peft.VBLoRAModel.print_savable_parameters]]

```python
print_savable_parameters()
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/vblora/model.py#L193)

Prints the number of savable VB-LoRA parameters and total savable parameters.
