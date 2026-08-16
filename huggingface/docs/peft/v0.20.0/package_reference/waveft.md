# WaveFT: Wavelet Fine-Tuning

[WaveFT](https://huggingface.co/papers/2505.12532) is a novel parameter-efficient fine-tuning (PEFT) method that introduces sparse updates in the **wavelet domain** of residual matrices. Unlike LoRA, which is constrained by discrete low-rank choices, WaveFT enables fine-grained control over the number of trainable parameters by directly learning a sparse set of coefficients in the transformed space. These coefficients are then mapped back to the weight domain via the Inverse Discrete Wavelet Transform (IDWT), producing high-rank updates without incurring inference overhead.

WaveFT currently has the following constraint:

- Only `nn.Linear` layers are supported.

The abstract from the paper is:

>Efficiently adapting large foundation models is critical, especially with tight compute and memory budgets. Parameter-Efficient Fine-Tuning (PEFT) methods such as LoRA offer limited granularity and effectiveness in few-parameter regimes. We propose Wavelet Fine-Tuning (WaveFT), a novel PEFT method that learns highly sparse updates in the wavelet domain of residual matrices. WaveFT allows precise control of trainable parameters, offering fine-grained capacity adjustment and excelling with remarkably low parameter count, potentially far fewer than LoRA’s minimum—ideal for extreme parameter-efficient scenarios. Evaluated on personalized text-to-image generation using Stable Diffusion XL as baseline, WaveFT significantly outperforms LoRA and other PEFT methods, especially at low parameter counts; achieving superior subject fidelity, prompt alignment, and image diversity.

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=WAVEFT"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## WaveFTConfig[[peft.WaveFTConfig]]

#### peft.WaveFTConfig[[peft.WaveFTConfig]]

```python
peft.WaveFTConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, n_frequency: int = 2592, scaling: float = 25.0, wavelet_family: str = 'db1', use_idwt: bool = True, random_loc_seed: int = 777, fan_in_fan_out: bool = False, target_modules: Optional[Union[list[str], str]] = None, exclude_modules: Optional[Union[list[str], str]] = None, bias: str = 'none', modules_to_save: Optional[list[str]] = None, layers_to_transform: Optional[Union[list[int], int]] = None, layers_pattern: Optional[Union[list[str], str]] = None, n_frequency_pattern: Optional[dict] = <factory>, proportional_parameters: bool = False, init_weights: bool = True)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/waveft/config.py#L27)

**Parameters:**

n_frequency (`int`) : Number of learnable wavelet coefficients for the Discrete Wavelet Transform (DWT). 'n_frequency' is an integer that is greater than 0 and less than or equal to the total number of elements in the original weight matrix (d_out * d_in). This parameter directly controls the number of trainable parameters for each adapted layer. A higher 'n_frequency' generally leads to better performance but also increases GPU memory usage, with a minor impact on training speed.

scaling (`float`) : The scaling factor applied to the reconstructed delta W matrix. This is a crucial hyperparameter, analogous to `lora_alpha` in LoRA. It can be tuned during hyperparameter search. Our default value for SDXL personalization is 25.

wavelet_family (`str`) : The wavelet family (e.g., 'db1', 'sym2', 'coif1') to use for the DWT and Inverse DWT (IDWT). Defaults to 'db1' (Haar wavelet). Different wavelet families have varying filter lengths which affect the training time substantially

use_idwt (`bool`) : Set to False for efficient adaptation. Whether to use the Inverse Discrete Wavelet Transform (IDWT) to reconstruct the delta weights from the learned wavelet coefficients. If `True` (default), the IDWT is applied. If `False`, the learned coefficients are directly used to form a sparse delta weight matrix, which is faster but performs worse for the SDXL personalization task.

random_loc_seed (`int`) : Seed for determining the random locations of the `n_frequency` learnable wavelet coefficients within the full wavelet coefficient matrix.

target_modules (`Union[list[str],str]`) : List of module names or a regex expression identifying the modules to be adapted with WaveFT. For example, `['q_proj', 'v_proj']` or `'.*decoder.*(SelfAttention|EncDecAttention).*(q|v)$'`. Currently, only linear layers (`torch.nn.Linear`) are supported.

exclude_modules (`Optional[Union[List[str], str]]`) : List of module names or a regex expression for modules to exclude from WaveFT adaptation.

fan_in_fan_out (`bool`) : Set to `True` if the weights of the layer to be replaced are stored in `(fan_in, fan_out)` format. Default is `False`.

bias (`str`) : Bias type for WaveFT. Can be 'none', 'all', or 'waveft_only'. ('fourier_only' was likely a typo and has been corrected to 'waveft_only' if it implies bias only on adapted parameters) If 'waveft_only', biases are added only to the WaveFT components. If 'all', biases are added to both base and WaveFT components. If 'none', no new biases are added.

modules_to_save (`list[str]`) : List of modules, in addition to WaveFT layers, that should be marked as trainable and saved in the final checkpoint. Useful for layers like classifiers in sequence or token classification tasks that are randomly initialized and need training.

layers_to_transform (`Union[list[int],int]`) : Specific layer indices to transform. If provided, PEFT will only adapt layers at these indices. If a single integer is given, only that layer is transformed.

layers_pattern (`Optional[Union[List[str], str]]`) : Pattern for layer names, used if `layers_to_transform` is specified and the layer pattern is not standard (e.g., not 'layers' or 'h'). This should target the `nn.ModuleList` attribute in the model.

n_frequency_pattern (`dict`) : A dictionary mapping layer names (or regex) to specific `n_frequency` values, overriding the global `n_frequency`. Example: `{"model.decoder.layers.0.encoder_attn.k_proj": 1000}`.

init_weights (`bool`) : Initialization strategy for the learnable wavelet coefficients (spectrum). If `True` (default), coefficients are initialized to zeros. If `False`, coefficients are initialized from a standard normal distribution scaled by a small factor.

proportional_parameters (`bool`) : If `True`, `n_frequency` is allocated proportionally to each layer's `input_dim * output_dim`. Default is `False`. Note: This option is included for experimental thoroughness to allow researchers to reproduce paper results, rather than for practical utility, as no beneficial scenarios have been identified.

This is the configuration class to store the configuration of a [WaveFTModel](/docs/peft/v0.20.0/en/package_reference/waveft#peft.WaveFTModel). It is used to define the
parameters for Wavelet-based Fine-Tuning (WaveFT), an approach that leverages the sparsity of wavelet transforms
for parameter-efficient fine-tuning of pretrained models.

## WaveFTModel[[peft.WaveFTModel]]

#### peft.WaveFTModel[[peft.WaveFTModel]]

```python
peft.WaveFTModel(model, peft_config: Union[PeftConfig, dict[str, PeftConfig]], adapter_name: str, low_cpu_mem_usage: bool = False, state_dict: Optional[dict[str, torch.Tensor]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/waveft/model.py#L30)
