# C3A: Parameter-Efficient Fine-Tuning via Circular Convolution

[C3A](https://huggingface.co/papers/2407.19342) is a parameter-efficient fine-tuning technique that leverages Circular Convolution to achieve high rank adaptation within reasonable resource limits.

Note that you should use a much larger learning rate (LR) for C3A than for other methods. For example, a LR of 1e-1 for C3A is a good starting point. Besides, a much smaller weight decay should be used. You can refer to the `method_comparison` folder for more details.

For the `block_size`, it affects tunable parameters and performance. To start with, you can choose a $\mathrm{gcd}(d_1,d_2)$ near $\frac{\sqrt{d_1 \times d_2}}{r}$, where $r$ is the rank for LoRA you would use for this task.

C3A currently has the following constraints:

- Only `nn.Linear` layers are supported.
- Quantized layers are not supported.
- The block size should be a common divisor of both the input and output sizes of target layers.

If these constraints don't work for your use case, consider other methods instead.

The abstract from the paper is:

> Low-Rank Adaptation (LoRA) has gained popularity for fine-tuning large foundation models, leveraging low-rank matrices $\mathbf{A}$ and $\mathbf{B}$ to represent weight changes (i.e., $\Delta \mathbf{W} = \mathbf{B} \mathbf{A}$). This method reduces trainable parameters and mitigates heavy memory consumption associated with full delta matrices by sequentially multiplying $\mathbf{A}$ and $\mathbf{B}$ with the activation. Despite its success, the intrinsic low-rank characteristic may limit its performance. Although several variants have been proposed to address this issue, they often overlook the crucial computational and memory efficiency brought by LoRA. In this paper, we propose Circular Convolution Adaptation (C3A), which not only achieves high-rank adaptation with enhanced performance but also excels in both computational power and memory utilization. Extensive experiments demonstrate that C3A consistently outperforms LoRA and its variants across various fine-tuning tasks.

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=C3A"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## C3AConfig[[peft.C3AConfig]]

#### peft.C3AConfig[[peft.C3AConfig]]

```python
peft.C3AConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, block_size: int = 256, target_modules: Optional[Union[list[str], str]] = None, bias: str = 'none', modules_to_save: Optional[list[str]] = None, layers_to_transform: Optional[Union[list[int], int]] = None, layers_pattern: Optional[Union[list[str], str]] = None, block_size_pattern: Optional[dict] = <factory>, init_weights: Optional[Union[bool, Literal['gaussian', 'kaiming_uniform', 'xavier_uniform']]] = 'xavier_uniform')
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/c3a/config.py#L25)

**Parameters:**

block_size (`int`) : block size for C3A, must be divisible by both the input size and the output size of the target layer. If you have no idea what block_size you should use, set it to the greatest common divisor of all input & output sizes of your target layers. Increasing this would result in less parameters.

target_modules (`Union[list[str],str]`) : The names of the modules to apply C3A to.

bias (`str`) : Bias type for C3A. Can be 'none', 'all' or 'c3a_only'. If 'all' or 'c3a_only', the corresponding biases will be updated during training. Be aware that this means that, even when disabling the adapters, the model will not produce the same output as the base model would have without adaptation.

modules_to_save (`list[str]`) --list of modules apart from C3A layers to be set as trainable and saved in the final checkpoint.

layers_to_transform (`Union[list[int],int]`) : The layer indexes to transform, if this argument is specified, it will apply C3A on the layer indexes that are specified in this list. If a single integer is passed, it will apply C3A on the layer at this index.

layers_pattern (`str`) : The layer pattern name, used only if `layers_to_transform` is different from `None` and if the layer pattern is not in the common layers pattern.

block_size_pattern (`dict`) : The mapping from layer names or regexp expression to block_size which are different from the default specified. For example, `{"model.decoder.layers.0.encoder_attn.k_proj": 1280`}

init_weights (`Union[bool, Literal["gaussian", "kaiming_uniform", "xavier_uniform"]]`) : Defaults to 'xavier_uniform'. Setting this to `False` also uses 'xavier_uniform'. To set the weights to zeros (thus making C3A a no-op), set the value to `True`.

This is the configuration class to store the configuration of a [C3AModel](/docs/peft/v0.20.0/en/package_reference/c3a#peft.C3AModel).

## C3AModel[[peft.C3AModel]]

#### peft.C3AModel[[peft.C3AModel]]

```python
peft.C3AModel(model, peft_config: Union[PeftConfig, dict[str, PeftConfig]], adapter_name: str, low_cpu_mem_usage: bool = False, state_dict: Optional[dict[str, torch.Tensor]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/c3a/model.py#L29)

**Parameters:**

model (`torch.nn.Module`) : The model to be adapted.

config ([C3AConfig](/docs/peft/v0.20.0/en/package_reference/c3a#peft.C3AConfig)) : The configuration of the C3A model.

adapter_name (`str`) : The name of the adapter, defaults to `"default"`.

**Returns:** `torch.nn.Module`

The C3A model.

Creates C3A model from a pretrained transformers model.

The method is described in detail in https://huggingface.co/papers/2407.19342.

**Attributes**:
- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The model to be adapted.
- **peft_config** ([C3AConfig](/docs/peft/v0.20.0/en/package_reference/c3a#peft.C3AConfig)): The configuration of the C3A model.
