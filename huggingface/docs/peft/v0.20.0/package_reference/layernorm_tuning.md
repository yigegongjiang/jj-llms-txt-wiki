# LayerNorm Tuning

LayerNorm Tuning ([LN Tuning](https://huggingface.co/papers/2312.11420)) is a PEFT method that only fine-tunes the parameters of the LayerNorm layers in a model.
The paper has tested the performance of this method on large language models and has shown that it can achieve strong performance with a significant reduction in the number of trainable parameters and GPU memory usage.
However, the method is not limited to language models and can be applied to any model that uses LayerNorm layers.
In this implementation, the default is that all layernorm layers inside a model is finetuned, but it could be used to target other layer types such as `MLP` or `Attention` layers, this can be done by specifying the `target_modules` in the `LNTuningConfig`.

The abstract from the paper is:

*This paper introduces an efficient strategy to transform Large Language Models (LLMs) into Multi-Modal Large Language Models (MLLMs). By conceptualizing this transformation as a domain adaptation process, i.e., transitioning from text understanding to embracing multiple modalities, we intriguingly note that, within each attention block, tuning LayerNorm suffices to yield strong performance. Moreover, when benchmarked against other tuning approaches like full parameter finetuning or LoRA, its benefits on efficiency are substantial. For example, when compared to LoRA on a 13B model scale, performance can be enhanced by an average of over 20% across five multi-modal tasks, and meanwhile, results in a significant reduction of trainable parameters by 41.9% and a decrease in GPU memory usage by 17.6%. On top of this LayerNorm strategy, we showcase that selectively tuning only with conversational data can improve efficiency further. Beyond these empirical outcomes, we provide a comprehensive analysis to explore the role of LayerNorm in adapting LLMs to the multi-modal domain and improving the expressive power of the model.*

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=LN_TUNING"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## LNTuningConfig[[peft.LNTuningConfig]]

#### peft.LNTuningConfig[[peft.LNTuningConfig]]

```python
peft.LNTuningConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, target_modules: Optional[Union[list[str], str]] = None, exclude_modules: Optional[Union[list[str], str]] = None, modules_to_save: Optional[Union[list[str], str]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/ln_tuning/config.py#L24)

**Parameters:**

target_modules (*Optional[Union[List[str], str]]*) : List of module names or regex expression of the module names to replace with LNTuning. For example, '.*decoder.*' or '.*encoder.*'. If this is not specified, modules will be chosen according to the model architecture. If the architecture is not known, an error will be raised -- in this case, you should specify the target modules manually.

exclude_modules (*Optional[Union[List[str], str]]*) : The names of the modules to not apply the adapter. When passing a string, a regex match will be performed. When passing a list of strings, either an exact match will be performed or it is checked if the name of the module ends with any of the passed strings.

modules_to_save (*Optional[Union[List[str], str]]*) : List of modules to be set as trainable and saved in the final checkpoint. For example, in Sequence Classification or Token Classification tasks, the final layer *classifier/score* are randomly initialized and as such need to be trainable and saved.

This is the configuration class to store the configuration of a [LNTuningModel](/docs/peft/v0.20.0/en/package_reference/layernorm_tuning#peft.LNTuningModel).

## LNTuningModel[[peft.LNTuningModel]]

#### peft.LNTuningModel[[peft.LNTuningModel]]

```python
peft.LNTuningModel(model, peft_config: Union[PeftConfig, dict[str, PeftConfig]], adapter_name: str, low_cpu_mem_usage: bool = False, state_dict: Optional[dict[str, torch.Tensor]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/ln_tuning/model.py#L28)

**Parameters:**

model (`torch.nn.Module`) : The model to be adapted.

config ([LNTuningConfig](/docs/peft/v0.20.0/en/package_reference/layernorm_tuning#peft.LNTuningConfig)) : The configuration of the LN tuning model.

adapter_name (`str`) : The name of the adapter, defaults to `"default"`.

low_cpu_mem_usage (`bool`, `optional`, defaults to `False`) : This option has no effect on LN tuning but exists for consistency with other PEFT methods.

**Returns:** `'torch.nn.Module'`

The adapted model with LayerNorm tuned on.

Creates LayerNorm tuning from a pretrained transformer model.

The method is described in detail in https://huggingface.co/papers/2312.11420.

Example:

```py
>>> from transformers import AutoModelForCausalLM
>>> from peft import get_peft_model, TaskType, LNTuningConfig

>>> peft_config = LNTuningConfig(
...     task_type=TaskType.CAUSAL_LM,
... )

>>> model = AutoModelForCausalLM.from_pretrained("meta-llama/Llama-2-7b-hf")
>>> model = get_peft_model(model, peft_config)
>>> model.print_trainable_parameters()
```

**Attributes**:
- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The model to be adapted.
- **peft_config** ([LNTuningConfig](/docs/peft/v0.20.0/en/package_reference/layernorm_tuning#peft.LNTuningConfig)): The configuration of the LN tuning model.
