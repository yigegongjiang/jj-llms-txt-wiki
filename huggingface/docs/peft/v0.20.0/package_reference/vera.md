# VeRA: Vector-based Random Matrix Adaptation

[VeRA](https://huggingface.co/papers/2310.11454) is a parameter-efficient fine-tuning technique that is similar to LoRA but requires even fewer extra parameters while promising similar or even better performance. As such, it is particularly useful when the parameter budget is very limited, e.g. when scaling to very large models. The reduction of the count of trainable parameters is achieved by sharing the same low-rank matrices across all layers, and only training two additional vectors per layer.

## How VeRA works

LoRA updates a frozen base model by learning two small, low-rank matrices for each adapted layer. The rank is the
inner dimension of these matrices, and it controls how much capacity the adapter has. In LoRA, increasing `r` or
adapting more layers increases the number of trainable parameters quickly because every adapted layer learns its own
pair of low-rank matrices.

VeRA keeps the same low-rank adaptation idea but changes which parameters are trained. Instead of learning separate
low-rank matrices for every layer, VeRA uses one frozen, randomly initialized pair of low-rank matrices, `A` and `B`,
that is shared across layers. Each adapted layer only learns two scaling vectors, `vera_lambda_d` and `vera_lambda_b`,
which rescale the shared matrices to produce a layer-specific update. Since `vera_lambda_d` has size `r`, VeRA's
trainable parameter count still increases with `r` and with the number of adapted layers. However, it grows much more
slowly than LoRA's because the large `A` and `B` matrices are shared and frozen instead of learned separately for each
layer.

This is why VeRA can use fewer trainable parameters than LoRA. In the simplified parameter count from the paper, LoRA
scales with `2 * L_tuned * d_model * r`, where `L_tuned` is the number of adapted layers, `d_model` is the model
dimension, and `r` is the rank. VeRA scales with `L_tuned * (d_model + r)` because the large low-rank matrices are
shared and frozen, while only the smaller per-layer vectors are trained.

When saving the adapter parameters, it's possible to eschew storing the low rank matrices by setting `save_projection=False` on the `VeraConfig`. In that case, these matrices will be restored based on the fixed random seed from the `projection_prng_key` argument. This cuts down on the size of the checkpoint, but we cannot guarantee reproducibility on all devices and for all future versions of PyTorch. If you want to ensure reproducibility, set `save_projection=True` (which is the default).

To handle different shapes of adapted layers, VeRA initializes shared A and B matrices with the largest required size for each dimension. During the forward pass, submatrices A and B for a given layer are sliced out from these shared matrices and used as described in the paper. For example, adapting two linear layers of shapes (100, 20) and (80, 50) will create A and B matrices of shapes (rank, 50) and (100, rank) respectively. Then, to adapt a layer of shape (100, 20), submatrices A and B of shapes (rank, 20) and (100, rank) will be extracted.

VeRA currently has the following constraint:

- Only `nn.Linear` layers are supported.

The abstract from the paper is:

> Low-rank adaptation (LoRA) is a popular method that reduces the number of trainable parameters when finetuning large language models, but still faces acute storage challenges when scaling to even larger models or deploying numerous per-user or per-task adapted models. In this work, we present Vector-based Random Matrix Adaptation (VeRA), which significantly reduces the number of trainable parameters compared to LoRA, yet maintains the same performance. It achieves this by using a single pair of low-rank matrices shared across all layers and learning small scaling vectors instead. We demonstrate its effectiveness on the GLUE and E2E benchmarks, image classification tasks, and show its application in instruction-tuning of 7B and 13B language models.

## When to use VeRA

VeRA is a good choice when:

- You want to minimize the number of trainable parameters while maintaining performance comparable to LoRA.
- You need to store or deploy many task-specific adapters, where smaller adapter checkpoints reduce storage requirements.
- You are fine-tuning very large language models under tight memory or parameter budgets.

## When not to use VeRA

VeRA may not be the best choice when:

- You require independent low-rank matrices for each adapted layer, providing greater flexibility in the learned adapter parameters.
- Your model requires adapting module types other than `nn.Linear`, since VeRA currently supports only linear layers.
- Your model contains adapted linear layers with widely different input and output dimensions. Because VeRA shares a single pair of projection matrices across all adapted layers, these matrices must be sized for the largest shape. Models with a large variation in layer shapes (for example, transformer up- and down-projection layers) can therefore require over-provisioning shared projection matrices, reducing some of VeRA's parameter-efficiency advantage.

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=VERA"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## VeRAConfig[[peft.VeraConfig]]

#### peft.VeraConfig[[peft.VeraConfig]]

```python
peft.VeraConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, r: int = 256, target_modules: Optional[Union[list[str], str]] = None, projection_prng_key: int = 0, save_projection: bool = True, vera_dropout: float = 0.0, d_initial: float = 0.1, fan_in_fan_out: bool = False, bias: str = 'none', modules_to_save: Optional[list[str]] = None, init_weights: bool = True, layers_to_transform: Optional[Union[list[int], int]] = None, layers_pattern: Optional[Union[list[str], str]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/vera/config.py#L25)

**Parameters:**

r (`int`, *optional*, defaults to `256`) : VeRA parameter dimension ("rank"). Choose higher values than LoRA ranks here, since VeRA uses far fewer parameters than LoRA (see Table 1).

target_modules (`Union[List[str], str]`) : The names of the modules to apply Vera to. Only linear layers are supported.

projection_prng_key (`int`) : Vera PRNG init key. Used for initialising vera_A and vera_B for new models or when loading a checkpoint that did not include these projections. Defaults to `0`.

save_projection (`bool`) : Whether to save the vera_A / vera_B projections in the state dict alongside per layer lambda_b / lambda_d weights. This will increase the size of the checkpoint, but guarantee that we can reload the checkpoint on all system configurations. Defaults to `True`.

vera_dropout (`float`) : The dropout probability for Vera layers.

d_initial (`float`, *optional*, defaults to `0.1`) : Initial init value for `vera_lambda_d` vector used when initializing the VeRA parameters. Small values (<=0.1) are recommended (see Table 6c in the paper).

fan_in_fan_out (`bool`) : Set this to True if the layer to replace stores weight like (fan_in, fan_out). For example, gpt-2 uses `Conv1D` which stores weights like (fan_in, fan_out) and hence this should be set to `True`.

bias (`str`) : Bias type for Vera. Can be 'none', 'all' or 'vera_only'. If 'all' or 'vera_only', the corresponding biases will be updated during training. Be aware that this means that, even when disabling the adapters, the model will not produce the same output as the base model would have without adaptation.

modules_to_save (`List[str]`) : List of modules apart from Vera layers to be set as trainable and saved in the final checkpoint.

init_weights (`bool`) : Whether to initialize the weights of the Vera layers with their default initialization. Don't change this setting, except if you know exactly what you're doing.

layers_to_transform (`Union[List[int],int]`) : The layer indexes to transform, if this argument is specified, it will apply the Vera transformations on the layer indexes that are specified in this list. If a single integer is passed, it will apply the Vera transformations on the layer at this index.

layers_pattern (`Optional[Union[List[str], str]]`) : The layer pattern name, used only if `layers_to_transform` is different from `None`. This should target the `nn.ModuleList` of the model, which is often called `'layers'` or `'h'`.

This is the configuration class to store the configuration of a [VeraModel](/docs/peft/v0.20.0/en/package_reference/vera#peft.VeraModel).

Paper: https://huggingface.co/papers/2310.11454.

## VeRAModel[[peft.VeraModel]]

#### peft.VeraModel[[peft.VeraModel]]

```python
peft.VeraModel(model, peft_config: Union[PeftConfig, dict[str, PeftConfig]], adapter_name: str, low_cpu_mem_usage: bool = False, state_dict: Optional[dict[str, torch.Tensor]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/vera/model.py#L78)

**Parameters:**

model ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) : The model to be adapted.

config ([VeraConfig](/docs/peft/v0.20.0/en/package_reference/vera#peft.VeraConfig)) : The configuration of the Vera model.

adapter_name (`str`) : The name of the adapter, defaults to `"default"`.

low_cpu_mem_usage (`bool`, `optional`, defaults to `False`) : Create empty adapter weights on meta device. Useful to speed up the loading process.

**Returns:** `torch.nn.Module`

The Vera model.

Creates Vector-based Random Matrix Adaptation (Vera) model from a pretrained transformers model.

Example:

```py
>>> from transformers import AutoModelForCausalLM
>>> from peft import VeraConfig, get_peft_model

>>> base_model = AutoModelForCausalLM.from_pretrained("facebook/opt-125m")
>>> config = VeraConfig(r=128)
>>> model = get_peft_model(base_model, config)
```

**Attributes**:
- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The model to be adapted.
- **peft_config** ([VeraConfig](/docs/peft/v0.20.0/en/package_reference/vera#peft.VeraConfig)): The configuration of the Vera model.
