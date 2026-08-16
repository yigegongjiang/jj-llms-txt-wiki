# RandLora: Full-rank parameter-efficient fine-tuning of large models
[RandLora](https://huggingface.co/papers/2502.00987) is a parameter-efficient fine-tuning technique that is similar to [LoRA](https://huggingface.co/papers/2106.09685) and [VeRA](https://huggingface.co/papers/2310.11454) but performs full rank updates to improve performance. RandLora can be particularly useful when adapting large model to hard tasks that require complex updates while preserving the parameter efficiency of LoRA. The full rank update of RandLora is achieved by linearly scaling random bases. The random bases are a collection of multiple low rank matrices such that the summation of their ranks if greater or equal to the full rank of the parameter matrices. The trainable parameters of RandLora are two diagonal matrices (vectors) that get multiplied with the right hand low rank random bases, in a similar way to VeRA's update. To maintain low memory usage, RandLora uses a custom function that prevents storing unnecessary bases in memory for backpropagation.

RandLora presents the noteworthy difference that contrary to other LoRA-like PEFT algorithm, increasing RandLora's random base ranks increases the amount of trainable parameters. Because number of bases x bases rank is constant in RandLora, reducing the rank will increase the number of random bases, hence the number of base-specific trainable diagonal bases.

Because reducing the rank of RandLora's random bases will increase their number, RandLora can become slower to train than LoRA for very small ranks where typically, ranks below 4 with result in a large training time increase. This does not affect inference though as the RandLora adapters can be merged into the pretrained weight matrices.

RandLora additionally supports training with sparse, ternary random bases (only containing -1, 0 and 1). These bases are as described in [Bingham et al.](https://cs-people.bu.edu/evimaria/cs565/kdd-rp.pdf) and [Ping et al.](https://hastie.su.domains/Papers/Ping/KDD06_rp.pdf) and could theoretically be used to reduce compute needs by performing aggregations instead of matrix multiplications to create the weight update. This is not currently supported. Although it does not currently reduce compute, using sparse random bases in RandLora can reduce overfitting in some cases. For users interested in using sparse ternary bases, the `sparse` option is recommended over the `very_sparse` one that can reduce performance.

Similarly to VeRA, when saving the RandLora's parameters, it's possible to eschew storing the low rank matrices by setting `save_projection=False` on the `VeraConfig`. In that case, these matrices will be restored based on the fixed random seed from the `projection_prng_key` argument. This cuts down on the size of the checkpoint, but we cannot guarantee reproducibility on all devices and for all future versions of PyTorch. If you want to ensure reproducibility, set `save_projection=True` (which is the default).

As in Vera and to handle different shapes of adapted layers, RandLora initializes shared A and B matrices with the largest required size for each dimension. During the forward pass, submatrices A and B for a given layer are sliced out from these shared matrices and used as described in the paper. For example, adapting two linear layers of shapes (100, 20) and (80, 50) will create A and B matrices of shapes (rank, 50) and (100, rank) respectively. Then, to adapt a layer of shape (100, 20), submatrices A and B of shapes (rank, 20) and (100, rank) will be extracted.

RandLora currently has the following constraint:

- Only `nn.Linear` layers are supported.

The abstract from the paper is:

> Low-Rank Adaptation (LoRA) and its variants have shown impressive results in reducing the number of trainable parameters and memory requirements of large transformer networks while maintaining fine-tuning performance. The low-rank nature of the weight update inherently limits the representation power of fine-tuned models, however, thus potentially compromising performance on complex tasks. This raises a critical question: when a performance gap between LoRA and standard fine-tuning is observed, is it due to the reduced number of trainable parameters or the rank deficiency?
This paper aims to answer this question by introducing RandLora, a parameter-efficient method that performs full-rank updates using a learned linear combinations of low-rank, non-trainable random matrices. Our method limits the number of trainable parameters by restricting optimization to diagonal scaling matrices applied to the fixed random matrices. This allows us to effectively overcome the low-rank limitations while maintaining parameter and memory efficiency during training. Through extensive experimentation across vision, language, and vision-language benchmarks, we systematically evaluate the limitations of LoRA and existing random basis methods. Our findings reveal that full-rank updates are beneficial across vision and language tasks individually, and even more so for vision-language tasks, where RandLora significantly reduces---and sometimes eliminates---the performance gap between standard fine-tuning and LoRA, demonstrating its efficacy.

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=RANDLORA"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## RandLoraConfig[[peft.RandLoraConfig]]

#### peft.RandLoraConfig[[peft.RandLoraConfig]]

```python
peft.RandLoraConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, r: int = 32, target_modules: typing.Union[str, list[str], NoneType] = None, projection_prng_key: int = 0, save_projection: bool = True, sparse: bool = False, very_sparse: bool = False, randlora_dropout: float = 0.0, fan_in_fan_out: bool = False, randlora_alpha: int = 640, bias: str = 'none', modules_to_save: typing.Optional[list[str]] = None, init_weights: bool = True, layers_to_transform: typing.Union[list[int], int, NoneType] = None, layers_pattern: typing.Optional[str] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/randlora/config.py#L24)

**Parameters:**

r (`int`, *optional*, defaults to `32`) : RandLora's random basis rank dimension. Contrary to Lora, this parameter is inversely proportional to the amount of trainable parameters as reducing it increases trainable parameters.

target_modules (`Union[list[str], str]`) : The names of the modules to apply RandLora to. Only linear layers are supported.

projection_prng_key (`int`) : RandLora PRNG init key. Used for initialising basis_A and basis_B for new models or when loading a checkpoint that did not include these projections. Defaults to `0`.

save_projection (`bool`) : Whether to save the global basis_A / basis_B random basis in the state dict alongside per layer lambda / gamma diagonal matrices. This will increase the size of the checkpoint, but guarantee that we can reload the checkpoint on all system configurations. Defaults to `True`.

sparse (`bool`) : Whether to use sparse random bases as described in the RandLora paper. The bases are ternary sparse bases (only containing -1, 0 and 1) where the attribution probability is 1/6 for -1 and 1 and 2/3 for 0. These sparse matrices aim to be used for matmul free computation in the future, see https://huggingface.co/papers/2406.02528v1 The current implementation is a proof of concept however where the sparseness is not used to improve speed or memory usage. Using sparse matrices typically does not reduce performance and can even help reduce overfitting. Defaults to `False`.

very_sparse (`bool`) : Whether to use highly sparse random bases as described in the RandLora paper. The very sparse bases are ternary sparse bases (only containing -1, 0 and 1) given a matrix with smallest dimension d, the attribution probability is 1/√D for -1 and 1 and 1- 2/√D for 0. Using these sparse matrices can further reduce overfitting over the `sparse` alternatives but will most likely decrease performance as a results. Use carefully. Defaults to `False`.

randlora_dropout (`float`) : The dropout probability for RandLora layers.

randlora_alpha (`float`) : The scaling coefficient for RandLora layers, this would typically be 20 times the rank. Because the `randlora_alpha` coefficient is large by default, it can lead to numerical instabilities especially when learning rates are high. If training is unstable, consider reducing the learning rate or the `randlora_alpha` coefficient.

fan_in_fan_out (`bool`) : Set this to True if the layer to replace stores weight like (fan_in, fan_out). For example, gpt-2 uses `Conv1D` which stores weights like (fan_in, fan_out) and hence this should be set to `True`.

bias (`str`) : Bias type. Can be 'none', 'all' or 'randlora_only'. If 'all' or 'randlora_only', the corresponding biases will be updated during training. Be aware that this means that, even when disabling the adapters, the model will not produce the same output as the base model would have without adaptation.

modules_to_save (`list[str]`) : list of modules apart from RandLora layers to be set as trainable and saved in the final checkpoint.

init_weights (`bool`) : Whether to initialize the weights of the RandLora layers with their default initialization. Don't change this setting, except if you know exactly what you're doing.

layers_to_transform (`Union[list[int],int]`) : The layer indexes to transform, if this argument is specified, it will apply the RandLora transformations on the layer indexes that are specified in this list. If a single integer is passed, it will apply the RandLora transformations on the layer at this index.

layers_pattern (`str`) : The layer pattern name, used only if `layers_to_transform` is different from `None` and if the layer pattern is not in the common layers pattern.

This is the configuration class to store the configuration of a [RandLoraModel](/docs/peft/v0.20.0/en/package_reference/randlora#peft.RandLoraModel).

Paper: https://huggingface.co/papers/2502.00987.

## RandLoraModel[[peft.RandLoraModel]]

#### peft.RandLoraModel[[peft.RandLoraModel]]

```python
peft.RandLoraModel(model, peft_config: Union[PeftConfig, dict[str, PeftConfig]], adapter_name: str, low_cpu_mem_usage: bool = False, state_dict: Optional[dict[str, torch.Tensor]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/randlora/model.py#L67)

**Parameters:**

model ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) : The model to be adapted.

config ([RandLoraConfig](/docs/peft/v0.20.0/en/package_reference/randlora#peft.RandLoraConfig)) : The configuration of the RandLora model.

adapter_name (`str`) : The name of the adapter, defaults to `"default"`.

low_cpu_mem_usage (`bool`, `optional`, defaults to `False`) : Create empty adapter weights on meta device. Useful to speed up the loading process.

**Returns:** `torch.nn.Module`

The RandLora model.

Creates a RandLoRA model from a pretrained transformers model.

Example:

```py
>>> from transformers import AutoModelForCausalLM
>>> from peft import RandLoraConfig, get_peft_model

>>> base_model = AutoModelForCausalLM.from_pretrained("facebook/opt-125m")
>>> config = RandLoraConfig(r=32)
>>> model = get_peft_model(base_model, config)
```

**Attributes**:
- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The model to be adapted.
- **peft_config** ([RandLoraConfig](/docs/peft/v0.20.0/en/package_reference/randlora#peft.RandLoraConfig)): The configuration of the RandLora model.
