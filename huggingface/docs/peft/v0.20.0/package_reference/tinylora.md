# TinyLoRA: Learning to Reason in 13 Parameters

[TinyLoRA](https://huggingface.co/papers/2602.04118) is an extremely parameter-efficient fine-tuning technique that builds upon the [LoRA-XS](https://huggingface.co/papers/2405.17604) approach by using SVD decomposition of frozen weights and projecting a tiny trainable vector through fixed random tensors. When combined with reinforcement learning (RL) training methods like GRPO, TinyLoRA can achieve competitive performance with as few as 1-13 trainable parameters.

The key innovation of TinyLoRA is replacing the trainable low-rank matrix R with a weighted sum of fixed random projection matrices: `R = Σᵢ vᵢ Pᵢ`, where `v ∈ R^u` is a tiny trainable vector of dimension `u` and `Pᵢ` are fixed random matrices. This dramatically reduces the number of trainable parameters while maintaining competitive performance.

TinyLoRA supports weight tying through the `weight_tying` parameter, a ratio between 0.0 and 1.0 that controls how many modules share the same trainable vector `v`. Setting `weight_tying=0.0` (the default) means no sharing, while `weight_tying=1.0` means full sharing across all target modules — achieving extreme parameter efficiency with just a single vector of `u` trainable parameters for the entire model.

When saving the adapter parameters, it's possible to eschew storing the random projection matrices by setting `save_projection=False` on the `TinyLoraConfig`. In that case, these matrices will be restored based on the fixed random seed from the `projection_seed` argument. This cuts down on the size of the checkpoint, but we cannot guarantee reproducibility on all devices and for all future versions of PyTorch. If you want to ensure reproducibility, set `save_projection=True` (which is the default).

TinyLoRA currently has the following constraints:

- Only `nn.Linear`, `nn.Embedding`, and `transformers.pytorch_utils.Conv1D` layers are supported.

The abstract from the paper is:

> Recent research has shown that language models can learn to reason, often via reinforcement learning. Some work even trains low-rank parameterizations for reasoning, but conventional LoRA cannot scale below the model dimension. We question whether even rank=1 LoRA is necessary for learning to reason and propose TinyLoRA, a method for scaling low-rank adapters to sizes as small as one parameter. Within our new parameterization, we are able to train the 8B parameter size of Qwen2.5 to 91% accuracy on GSM8K with only 13 trained parameters in bf16 (26 total bytes). We find this trend holds in general: we are able to recover 90% of performance improvements while training 1000x fewer parameters across a suite of more difficult learning-to-reason benchmarks such as AIME, AMC, and MATH500. Notably, we are only able to achieve such strong performance with RL: models trained using SFT require 100-1000x larger updates to reach the same performance.

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=TINYLORA"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## TinyLoraConfig[[peft.TinyLoraConfig]]

- **r** (`int`, *optional*, defaults to `2`) --
  SVD rank for the frozen U, Sigma, V decomposition. The paper recommends r=2.
- **u** (`int`, *optional*, defaults to `64`) --
  Trainable vector dimension per group. This controls the expressivity of the adaptation. Can be as low as
  1-13 for extreme parameter efficiency.
- **weight_tying** (`float`, *optional*, defaults to `0.0`) --
  Degree of weight tying across target modules, as a ratio between 0.0 and 1.0. Controls how many modules
  share the same trainable vector v. 0.0 means no sharing (each module has its own v). 1.0 means full sharing
  (all modules share one v). Values in between give partial sharing.
- **projection_seed** (`int`, *optional*, defaults to `42`) --
  Random seed for generating the fixed projection matrices P.
- **save_projection** (`bool`, *optional*, defaults to `True`) --
  Whether to save the projection tensors P in the state dict. If False, they will be regenerated from the
  seed when loading.
- **init_v_bound** (`float`, *optional*, defaults to `0.02`) --
  Uniform initialization bound for the trainable vector v. Values are initialized in [-init_v_bound,
  init_v_bound].
- **target_modules** (`Union[List[str], str]`, *optional*) --
  The names of the modules to apply TinyLoRA to. This can be a list of module names (e.g. `['q_proj',
  'v_proj']`), a regex pattern (e.g. `'.*decoder.*(q|v)_proj$'`), or the special keyword `"all-linear"` to
  target all linear modules. Only `nn.Linear`, `nn.Embedding`, and `transformers.pytorch_utils.Conv1D` layers
  are supported.
- **tinylora_dropout** (`float`, *optional*, defaults to `0.0`) --
  The dropout probability for TinyLoRA layers.
- **fan_in_fan_out** (`bool`, *optional*, defaults to `False`) --
  Set this to True if the layer to replace stores weight like (fan_in, fan_out). For example, gpt-2 uses
  `Conv1D` which stores weights like (fan_in, fan_out).
- **bias** (`str`, *optional*, defaults to `"none"`) --
  Bias type for TinyLoRA. Can be 'none', 'all' or 'tinylora_only'.
- **modules_to_save** (`List[str]`, *optional*) --
  List of modules apart from TinyLoRA layers to be set as trainable and saved.
- **init_weights** (`bool` | `Literal["uniform"]`, *optional*, defaults to `True`) --
  How to initialize the trainable vector v. Passing `True` (default) initializes v to zeros, making the
  adapter a no-op (identity operation). Passing `"uniform"` initializes v with uniform random values in
  `[-init_v_bound, init_v_bound]`. Passing `False` leaves v uninitialized (for advanced use cases).
- **layers_to_transform** (`Union[List[int], int]`, *optional*) --
  The layer indexes to transform. If specified, only these layers will be adapted.
- **layers_pattern** (`Optional[Union[List[str], str]]`, *optional*) --
  The layer pattern name, used only if `layers_to_transform` is different from `None`.

This is the configuration class to store the configuration of a [TinyLoraModel](/docs/peft/v0.20.0/en/package_reference/tinylora#peft.TinyLoraModel).

TinyLoRA is an extremely parameter-efficient fine-tuning method based on the paper "Learning to Reason in 13
Parameters" (arXiv:2602.04118). It uses SVD decomposition of frozen weights and projects a tiny trainable vector
through fixed random tensors.

Paper: https://arxiv.org/abs/2602.04118

Example:
```python
from peft import get_peft_model, TinyLoraConfig

config = TinyLoraConfig(
    r=2,  # SVD rank (paper recommends 2)
    u=64,  # Trainable vector dimension
    weight_tying=0.0,  # No weight tying (0.0 = none, 1.0 = full)
    target_modules=["q_proj", "v_proj"],
    projection_seed=42,
)
model = get_peft_model(base_model, config)
```

## TinyLoraModel[[peft.TinyLoraModel]]

- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The model to be adapted.
- **config** ([TinyLoraConfig](/docs/peft/v0.20.0/en/package_reference/tinylora#peft.TinyLoraConfig)) -- The configuration of the TinyLoRA model.
- **adapter_name** (`str`) -- The name of the adapter, defaults to `"default"`.
- **low_cpu_mem_usage** (`bool`, *optional*, defaults to `False`) --
  Create empty adapter weights on meta device. Useful to speed up the loading process.`torch.nn.Module`The TinyLoRA model.

Creates TinyLoRA model from a pretrained transformers model.

TinyLoRA is an extremely parameter-efficient fine-tuning method that uses SVD decomposition of frozen weights and
projects a tiny trainable vector through fixed random tensors. Based on the paper "Learning to Reason in 13
Parameters" (arXiv:2602.04118).

Example:
```python
>>> from transformers import AutoModelForCausalLM
>>> from peft import TinyLoraConfig, get_peft_model

>>> base_model = AutoModelForCausalLM.from_pretrained("facebook/opt-125m")
>>> config = TinyLoraConfig(r=2, u=64, target_modules=["q_proj", "v_proj"])
>>> model = get_peft_model(base_model, config)
```

**Attributes**:
- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The model to be adapted.
- **peft_config** ([TinyLoraConfig](/docs/peft/v0.20.0/en/package_reference/tinylora#peft.TinyLoraConfig)): The configuration of the TinyLoRA model.

Delete an adapter and clean up the model-level shared v parameters.
