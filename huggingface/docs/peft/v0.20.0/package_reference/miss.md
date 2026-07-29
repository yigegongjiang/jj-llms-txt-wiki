# MiSS

[MiSS (Matrix Shard Sharing)](https://arxiv.org/abs/2409.15371) is a PEFT method that achieves a good balance between model performance and computational efficiency. It requires only a single trainable matrix and introduces a shard-sharing mechanism distinct from LoRA.

The abstract from the paper is:

*Parameter-Efficient Fine-Tuning (PEFT) methods, particularly Low-Rank Adaptation (LoRA), effectively reduce the number of trainable parameters in Large Language Models (LLMs). However, as model scales continue to grow, the demand for computational resources remains a significant challenge. Existing LoRA variants often struggle to strike an optimal balance between adaptability (model performance and convergence speed) and efficiency (computational overhead, memory usage, and initialization time). This paper introduces MiSS (Matrix Shard Sharing), a novel PEFT approach that addresses this trade-off through a simple shard-sharing mechanism. MiSS leverages the insight that a low-rank adaptation can be achieved by decomposing the weight matrix into multiple fragment matrices and utilizing a shared, trainable common fragment. This method constructs the low-rank update matrix through the replication of these shared, partitioned shards. We also propose a hardware-efficient and broadly applicable implementation for MiSS. Extensive experiments conducted on a range of tasks, alongside a systematic analysis of computational performance, demonstrate MiSS's superiority. The results show that MiSS significantly outperforms standard LoRA and its prominent variants in both model performance metrics and computational efficiency, including initialization speed and training throughput. By effectively balancing expressive power and resource utilization, MiSS offers a compelling solution for efficiently adapting large-scale models.*

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=MISS"
	frameborder="0"
	width="850"
	height="1000"
>

## When to use MiSS

MiSS is a good choice when:

- You want faster initialization and higher training throughput than advanced LoRA initialization schemes that use expensive setups (such as PiSSA, LoRA-GA, or OLoRA).
- You want a drop-in alternative to LoRA with minimal configuration changes.

If you need stronger expressiveness at the cost of some efficiency, consider the `bat` initialization variant (see below).

## init_weights modes

MiSS supports three initialization modes via the `init_weights` parameter:

- `True` (default): Standard MiSS initialization. Best starting point for most use cases.
- `"bat"`: Enables nonlinear updates across different shards. Produces better results than standard MiSS but uses more memory and is approximately twice as slow. Use this when performance is the priority over efficiency.
- `"mini"`: Uses a smaller rank along the `out_features` dimension, controlled by `mini_r`. This reduces trainable parameters further. When using this mode, `mini_r` must be set and `out_features` must be divisible by `mini_r`.

## Quick start

```python
import torch
from peft import MissConfig, get_peft_model
from transformers import AutoModelForCausalLM, AutoTokenizer

model = AutoModelForCausalLM.from_pretrained(
    "meta-llama/Llama-2-7b-hf",
    torch_dtype=torch.bfloat16,
    device_map="auto"
)
tokenizer = AutoTokenizer.from_pretrained("meta-llama/Llama-2-7b-hf")
tokenizer.pad_token_id = tokenizer.eos_token_id

# Standard MiSS
config = MissConfig(
    r=64,
    miss_dropout=0.01,
    task_type="CAUSAL_LM"
)

# BAT variant — better performance, more memory
# config = MissConfig(
#     r=64,
#     init_weights="bat",
#     task_type="CAUSAL_LM"
# )

# Mini variant — fewer trainable parameters
# config = MissConfig(
#     r=64,
#     init_weights="mini",
#     mini_r=8,
#     task_type="CAUSAL_LM"
# )

model = get_peft_model(model, config)
model.print_trainable_parameters()
```

For a full fine-tuning example including training and inference, see the [MiSS fine-tuning example](https://github.com/huggingface/peft/tree/main/examples/miss_finetuning).

# API

## MissConfig[[peft.MissConfig]]

- **r** (`int`) --
  The rank of MiSS across different layers. It is best to set 'r' to an even number; otherwise, the default
  initialization method will not work. The rank of MiSS corresponds to a low-rank decomposition along the
  in_features dimension.
- **miss_dropout** (`float`) --
  The dropout probability for MiSS layers.
- **mini_r** (`int`) --
  The rank of MiSS corresponds to a low-rank decomposition along the out_features dimension. When you set
  `init_weights=mini`, you need to set `mini_r`. Please make sure that `out_features` is divisible by
  `mini_r`.
- **target_modules** (`Optional[Union[List[str], str]]`) --
  The names of the modules to apply the adapter to. If this is specified, only the modules with the specified
  names will be replaced. When passing a string, a regex match will be performed. When passing a list of
  strings, either an exact match will be performed or it is checked if the name of the module ends with any
  of the passed strings. If this is specified as 'all-linear', then all linear modules are chosen, excluding
  the output layer. If this is not specified, modules will be chosen according to the model architecture. If
  the architecture is not known, an error will be raised -- in this case, you should specify the target
  modules manually.
- **exclude_modules** (`Optional[Union[List[str], str]]`) --
  The names of the modules to not apply the adapter. When passing a string, a regex match will be performed.
  When passing a list of strings, either an exact match will be performed or it is checked if the name of the
  module ends with any of the passed strings.
- **init_weights** (bool | Literal["bat", "mini"]) --
  Different initializations correspond to different MiSS variants. By default(balance), the most efficient
  and general method in MiSS will be used. 'bat': In this mode, you can enable nonlinear updates across
  different shards. 'mini': In this mode, you can set a smaller rank to use fewer trainable parameters, but
  it is recommended to keep `out_features % mini_r == 0`.
- **layers_to_transform** (`Union[List[int], int]`) --
  The layer indices to transform. If a list of ints is passed, it will apply the adapter to the layer indices
  that are specified in this list. If a single integer is passed, it will apply the transformations on the
  layer at this index.
- **layers_pattern** (`str`) --
  The layer pattern name, used only if `layers_to_transform` is different from `None`.
- **bias** (`str`) --
  Bias type for MiSS. Can be `'none'`, `'all'` or `'MiSS_only'`.
- **modules_to_save** (`List[str]`) --
  List of modules apart from adapter layers to be set as trainable and saved in the final checkpoint.

This is the configuration class to store the configuration of a [MissModel](/docs/peft/v0.20.0/en/package_reference/miss#peft.MissModel).

## MissModel[[peft.MissModel]]

- **model** (`torch.nn.Module`) -- The model to which the adapter tuner layers will be attached.
- **config** ([MissConfig](/docs/peft/v0.20.0/en/package_reference/miss#peft.MissConfig)) -- The configuration of the MiSS model.
- **adapter_name** (`str`) -- The name of the adapter, defaults to `"default"`.
- **low_cpu_mem_usage** (`bool`, `optional`, defaults to `False`) --
  Create empty adapter weights on meta device. Useful to speed up the loading process.`torch.nn.Module`The MiSS model.

Creates Householder reflection adaptation (MiSS) model from a pretrained model. The method is described in
https://huggingface.co/papers/2409.15371

Example:
```py
>>> from diffusers import StableDiffusionPipeline
>>> from peft import MissModel, MissConfig

>>> config_te = MissConfig(
...     r=8,
...     target_modules=["k_proj", "q_proj", "v_proj", "out_proj", "fc1", "fc2"],
...     init_weights=True,
... )
>>> config_unet = MissConfig(
...     r=8,
...     target_modules=[
...         "proj_in",
...         "proj_out",
...         "to_k",
...         "to_q",
...         "to_v",
...         "to_out.0",
...         "ff.net.0.proj",
...         "ff.net.2",
...     ],
...     init_weights=True,
... )

>>> model = StableDiffusionPipeline.from_pretrained("runwayml/stable-diffusion-v1-5")
>>> model.text_encoder = MissModel(model.text_encoder, config_te, "default")
>>> model.unet = MissModel(model.unet, config_unet, "default")
```

**Attributes**:
- **model** (`~torch.nn.Module`) -- The model to be adapted.
- **peft_config** ([MissConfig](/docs/peft/v0.20.0/en/package_reference/miss#peft.MissConfig)): The configuration of the MiSS model.
