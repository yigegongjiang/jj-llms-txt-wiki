# FourierFT: Discrete Fourier Transformation Fine-Tuning

[FourierFT](https://huggingface.co/papers/2405.03003) is a parameter-efficient fine-tuning technique that leverages Discrete Fourier Transform to compress the model's tunable weights. This method outperforms LoRA in the GLUE benchmark and common ViT classification tasks using much less parameters.

FourierFT currently has the following constraints:

- Only `nn.Linear` layers are supported.
- Quantized layers are not supported.

If these constraints don't work for your use case, consider other methods instead.

The abstract from the paper is:

> Low-rank adaptation (LoRA) has recently gained much interest in fine-tuning foundation models. It effectively reduces the number of trainable parameters by incorporating low-rank matrices A and B to represent the weight change, i.e., Delta W=BA. Despite LoRA's progress, it faces storage challenges when handling extensive customization adaptations or larger base models. In this work, we aim to further compress trainable parameters by enjoying the powerful expressiveness of the Fourier transform. Specifically, we introduce FourierFT, which treats Delta W as a matrix in the spatial domain and learns only a small fraction of its spectral coefficients. With the trained spectral coefficients, we implement the inverse discrete Fourier transform to recover Delta W. Empirically, our FourierFT method shows comparable or better performance with fewer parameters than LoRA on various tasks, including natural language understanding, natural language generation, instruction tuning, and image classification. For example, when performing instruction tuning on the LLaMA2-7B model, FourierFT surpasses LoRA with only 0.064M trainable parameters, compared to LoRA's 33.5M.

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=FOURIERFT"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## FourierFTConfig[[peft.FourierFTConfig]]

#### peft.FourierFTConfig[[peft.FourierFTConfig]]

```python
peft.FourierFTConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, n_frequency: int = 1000, scaling: float = 150.0, random_loc_seed: Optional[int] = 777, fan_in_fan_out: bool = False, target_modules: Optional[Union[list[str], str]] = None, exclude_modules: Optional[Union[list[str], str]] = None, bias: str = 'none', modules_to_save: Optional[list[str]] = None, layers_to_transform: Optional[Union[list[int], int]] = None, layers_pattern: Optional[Union[list[str], str]] = None, n_frequency_pattern: Optional[dict] = <factory>, init_weights: bool = False)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/fourierft/config.py#L25)

**Parameters:**

n_frequency (`int`) : Num of learnable frequencies for the Discrete Fourier Transform. 'n_frequency' is an integer that is greater than 0 and less than or equal to d^2 (assuming the weight W has dimensions of d by d). Additionally, it is the number of trainable parameters required to update each delta W weight. 'n_frequency' will affect the performance and efficiency for PEFT. Specifically, it has little impact on training speed, but higher values of it (typically) result in larger GPU memory costs and better accuracy. With the same `target_modules`, the number of parameters of LoRA is (2*d*r/n_frequency) times that of FourierFT. The following examples of settings regarding 'n_frequency' can be used as reference for users. For NLU tasks with the RoBERTa-large model, adopting 'n_frequency': 1000 can almost achieve similar results as 'r': 8 in LoRA. At this time, the number of parameters of LoRA is about 16 times that of FourierFT. For image classification tasks with Vit-large models, adopting 'n_frequency': 3000 can almost achieve similar results as 'r': 16 in LoRA, where the number of parameters of LoRA is about 11 times that of FourierFT.

scaling (`float`) : The scaling value for the delta W matrix. This is an important hyperparameter used for scaling, similar to the 'lora_alpha' parameter in the LoRA method. 'scaling' can be determined during the hyperparameter search process. However, if users want to skip this process, one can refer to the settings in the following scenarios. This parameter can be set to 100.0 or 150.0 for both RoBERTa-base and RoBERTa-large models across all NLU (GLUE) tasks. This parameter can be set to 300.0 for both LLaMA family models for all instruction tuning. This parameter can be set to 300.0 for both ViT-base and ViT-large models across all image classification tasks.

random_loc_seed (`int`) : Seed for the random location of the frequencies, i.e., the spectral entry matrix.

target_modules (`Union[list[str],str]`) : List of module names or regex expression of the module names to replace with FourierFT. For example, ['q', 'v'] or '.*decoder.*(SelfAttention|EncDecAttention).*(q|v)$'. Only linear layers are supported.

exclude_modules (`Optional[Union[List[str], str]]`) : The names of the modules to not apply the adapter. When passing a string, a regex match will be performed. When passing a list of strings, either an exact match will be performed or it is checked if the name of the module ends with any of the passed strings.

fan_in_fan_out (`bool`) : Set this to True if the layer to replace stores weight like (fan_in, fan_out).

bias (`str`) : Bias type for FourierFT. Can be 'none', 'all' or 'fourier_only'.

modules_to_save (`list[str]`) : List of modules apart from FourierFT layers to be set as trainable and saved in the final checkpoint. For example, in Sequence Classification or Token Classification tasks, the final layer `classifier/score` are randomly initialized and as such need to be trainable and saved.

layers_to_transform (`Union[list[int],int]`) : The layer indexes to transform, is this argument is specified, PEFT will transform only the layers indexes that are specified inside this list. If a single integer is passed, PEFT will transform only the layer at this index.

layers_pattern (`Optional[Union[List[str], str]]`) : The layer pattern name, used only if `layers_to_transform` is different to None and if the layer pattern is not in the common layers pattern. This should target the `nn.ModuleList` of the model, which is often called `'layers'` or `'h'`.

n_frequency_pattern (`dict`) : The mapping from layer names or regexp expression to n_frequency which are different from the default specified. For example, `{model.decoder.layers.0.encoder_attn.k_proj: 1000`}.

init_weights (`bool`) : The initialization of the Fourier weights. Set this to False (the default) if the spectrum are initialized to a standard normal distribution. Set this to True if the spectrum are initialized to zeros.

This is the configuration class to store the configuration of a [FourierFTModel](/docs/peft/v0.20.0/en/package_reference/fourierft#peft.FourierFTModel).

## FourierFTModel[[peft.FourierFTModel]]

#### peft.FourierFTModel[[peft.FourierFTModel]]

```python
peft.FourierFTModel(model, peft_config: Union[PeftConfig, dict[str, PeftConfig]], adapter_name: str, low_cpu_mem_usage: bool = False, state_dict: Optional[dict[str, torch.Tensor]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/fourierft/model.py#L31)

**Parameters:**

model (`torch.nn.Module`) : The model to be adapted.

config ([FourierFTConfig](/docs/peft/v0.20.0/en/package_reference/fourierft#peft.FourierFTConfig)) : The configuration of the FourierFT model.

adapter_name (`str`) : The name of the adapter, defaults to `"default"`.

low_cpu_mem_usage (`bool`, `optional`, defaults to `False`) : Create empty adapter weights on meta device. Useful to speed up the loading process.

**Returns:** `torch.nn.Module`

The FourierFT model.

Creates FourierFT model from a pretrained transformers model.

The method is described in detail in https://huggingface.co/papers/2405.03003.

**Attributes**:
- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The model to be adapted.
- **peft_config** ([FourierFTConfig](/docs/peft/v0.20.0/en/package_reference/fourierft#peft.FourierFTConfig)): The configuration of the Fourier model.
