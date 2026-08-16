# DeLoRA: Decoupled Low-rank Adaptation
[DeLoRA](https://huggingface.co/papers/2503.18225) is a parameter-efficient fine-tuning technique that implicitly maintains a Frobenius boundary with respect to the pretrained weights by normalizing and scaling learnable low-rank matrices. This effectively decouples the learning of directions (BA term) and magnitude (boundary term) of the weight updates, avoiding catastrophic shifts in the adapted weights and enhancing robustness to hyperparameter choices.

Note:
- use a learning rate 10-100x larger than for standard LoRA variants (typical values from 1e-3/1e-2/..)
- ensure the initial boundary parameter lambda is not too small (typical values around 10/15/..). Setting different lambdas to different layers is possible

DeLoRA currently has the following constraints:
- Only nn.Linear layers are supported.
- Quantized layers are not supported.

If these constraints don't work for your use case, consider other methods instead.

The abstract from the paper is:

> Parameter-Efficient FineTuning (PEFT) methods have recently gained significant popularity thanks to the widespread availability of large-scale pretrained models. These methods allow for quick adaptation to downstream tasks with minimal computational cost. However, popular finetuning methods such as LoRA exhibit limited robustness when it comes to hyperparameter choices or extended training regimes, preventing optimal out-of-the-box performance. In contrast, bounded approaches, such as ETHER, provide greater robustness but are limited to extremely low-rank adaptations and fixed-strength transformations, reducing their adaptation expressive power. In this work, we propose Decoupled Low-rank Adaptation (DeLoRA), a novel finetuning method that normalizes and scales learnable low-rank matrices. By bounding the distance of the transformation, DeLoRA effectively decouples the angular learning from the adaptation strength, enhancing robustness without compromising performance. Through evaluations on subject-driven image generation, natural language understanding, and instruction tuning, we show that DeLoRA matches or surpasses performance of competing PEFT methods, while exhibiting stronger robustness.

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=DELORA"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## DeloraConfig[[peft.DeloraConfig]]

#### peft.DeloraConfig[[peft.DeloraConfig]]

```python
peft.DeloraConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, r: int = 8, delora_lambda: int = 15, module_dropout: float = 0.0, target_modules: Optional[Union[list[str], str]] = None, exclude_modules: Optional[Union[list[str], str]] = None, bias: Literal['none', 'all', 'delora_only'] = 'none', init_weights: bool = True, layers_to_transform: Optional[Union[list[int], int]] = None, layers_pattern: Optional[Union[list[str], str]] = None, rank_pattern: Optional[dict] = <factory>, lambda_pattern: Optional[dict] = <factory>, modules_to_save: Optional[list[str]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/delora/config.py#L24)

**Parameters:**

r (`int`) : The rank of the DeLoRA adapter.

delora_lambda (`int`) : The initial value of the boundary of the DeLoRA adapter. This variable sets an upper bound to the Frobenius norm of the weight change, avoiding the finetuned model to deviate too much from the original model.

module_dropout (`float`) : The dropout probability for disabling DeLoRA modules during training.

target_modules (`Optional[Union[List[str], str]]`) : The names of the modules to apply the adapter to. If this is specified, only the modules with the specified names will be replaced. When passing a string, a regex match will be performed. When passing a list of strings, either an exact match will be performed or it is checked if the name of the module ends with any of the passed strings. If this is specified as 'all-linear', then all linear/Conv1D modules are chosen, excluding the output layer. If this is not specified, modules will be chosen according to the model architecture. If the architecture is not known, an error will be raised -- in this case, you should specify the target modules manually.

exclude_modules (`Optional[Union[List[str], str]]`) : The names of the modules to not apply the adapter. When passing a string, a regex match will be performed. When passing a list of strings, either an exact match will be performed or it is checked if the name of the module ends with any of the passed strings.

bias (`str`) : Bias type for DeLoRA. Can be 'none', 'all' or 'delora_only'. If 'all' or 'delora_only', the corresponding biases will be updated during training. Be aware that this means that, even when disabling the adapters, the model will not produce the same output as the base model would have without adaptation.

init_weights (`bool`) : Whether to perform initialization of adapter weights. If `True` (default): A is initialized with kaiming uniform initialization, while B is initialized with zeros. If `False`: A and B are both initialized with kaiming uniform, immediately contributing a non-zero delta. This is generally discouraged for normal use.

layers_to_transform (`Union[List[int], int]`) : The layer indices to transform. If a list of ints is passed, it will apply the adapter to the layer indices that are specified in this list. If a single integer is passed, it will apply the transformations on the layer at this index.

layers_pattern (`Optional[Union[List[str], str]]`) : The layer pattern name, used only if `layers_to_transform` is different from `None`. This should target the `nn.ModuleList` of the model, which is often called `'layers'` or `'h'`.

rank_pattern (`dict`) : The mapping from layer names or regexp expression to ranks which are different from the default rank specified by `r`. For example, `{'^model.decoder.layers.0.encoder_attn.k_proj': 16}`.

lambda_pattern (`dict`) : The mapping from layer names or regexp expression to lambdas which are different from the default lambda specified by `delora_lambda`. For example, `{'^model.decoder.layers.0.encoder_attn.k_proj': 16}`.

modules_to_save (`Optional[List[str]]`) : List of modules apart from adapter layers to be set as trainable and saved in the final checkpoint.

This is the configuration class to store the configuration of a [DeloraModel](/docs/peft/v0.20.0/en/package_reference/delora#peft.DeloraModel).

## DeloraModel[[peft.DeloraModel]]

#### peft.DeloraModel[[peft.DeloraModel]]

```python
peft.DeloraModel(model, peft_config: Union[PeftConfig, dict[str, PeftConfig]], adapter_name: str, low_cpu_mem_usage: bool = False, state_dict: Optional[dict[str, torch.Tensor]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/delora/model.py#L28)

**Parameters:**

model (`torch.nn.Module`) : The model to be adapted.

config ([DeloraConfig](/docs/peft/v0.20.0/en/package_reference/delora#peft.DeloraConfig)) : The configuration of the DeLoRA model.

adapter_name (`str`) : The name of the adapter, defaults to `"default"`.

**Returns:** `torch.nn.Module`

The DeLoRA model.

Creates DeLoRA model from a pretrained transformers model.

The method is described in detail in [DeLoRA: Decoupled Low-rank
Adaptation](https://huggingface.co/papers/2503.18225).

**Attributes**:
- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The model to be adapted.
- **peft_config** ([DeloraConfig](/docs/peft/v0.20.0/en/package_reference/delora#peft.DeloraConfig)): The configuration of the DeLoRA model.
