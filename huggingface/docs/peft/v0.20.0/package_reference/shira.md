# Sparse High Rank Adapters

Sparse High Rank Adapters or [SHiRA](https://huggingface.co/papers/2406.13175) is an alternate type of adapter and has been found to have significant advantages over the low rank adapters. Specifically, SHiRA achieves better accuracy than LoRA for a variety of vision and language tasks. It also offers simpler and higher quality multi-adapter fusion by significantly reducing concept loss, a common problem faced by low rank adapters. SHiRA directly finetunes a small number of the base model's parameters to finetune the model on any adaptation task.

SHiRA currently has the following constraint:

- Only `nn.Linear` layers are supported.

The abstract from the paper is:

> Low Rank Adaptation (LoRA) has gained massive attention in the recent generative AI research. One of the main advantages of LoRA is its ability to be fused with pretrained models, adding no overhead during inference. However, from a mobile deployment standpoint, we can either avoid inference overhead in the fused mode but lose the ability to switch adapters rapidly, or suffer significant (up to 30% higher) inference latency while enabling rapid switching in the unfused mode. LoRA also exhibits concept-loss when multiple adapters are used concurrently. In this paper, we propose Sparse High Rank Adapters (SHiRA), a new paradigm which incurs no inference overhead, enables rapid switching, and significantly reduces concept-loss. Specifically, SHiRA can be trained by directly tuning only 1-2% of the base model weights while leaving others unchanged. This results in a highly sparse adapter which can be switched directly in the fused mode. We further provide theoretical and empirical insights on how high sparsity in SHiRA can aid multi-adapter fusion by reducing concept loss. Our extensive experiments on LVMs and LLMs demonstrate that finetuning only a small fraction of the parameters in the base model significantly outperforms LoRA while enabling both rapid switching and multi-adapter fusion. Finally, we provide a latency- and memory-efficient SHiRA implementation based on Parameter-Efficient Finetuning (PEFT) Library which trains at nearly the same speed as LoRA while consuming up to 16% lower peak GPU memory, thus making SHiRA easy to adopt for practical use cases. To demonstrate rapid switching benefits during inference, we show that loading SHiRA on a base model can be 5x-16x faster than LoRA fusion on a CPU.

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=SHIRA"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## ShiraConfig[[peft.ShiraConfig]]

- **r** (`int`, *optional*, defaults to `32`) --
  For a given target module, the number of SHiRA parameters is computed as r(m+n), where the original tensor
  dimensions are m x n. This means the number of SHiRA parameters is the same as that for a LoRA adapter.
  SHiRA is a high rank adapter. Setting this r parameter does not restrict the rank to this value.
- **mask_type** (`str`, defaults to `random`) --
  Type of mask function. Defaults to a random sparse mask. An optional user-defined mask_fn to compute the
  mask value can also be supplied by instantiating `config = ShiraConfig(...)` and then setting
  `config.mask_fn = <your custom mask function>`. For a pretrained weight with shape m x n, the custom mask
  function must return only one mask (shape: m x n) which must be binary 0 or 1 with num_shira_parameters =
  r(m + n) for linear layers. Device and dtype of mask must be same as base layer's weight's device and
  dtype. Please see mask_functions.py for more details and to see the default random sparse mask
  implementation.
- **random_seed** (`int`, *optional*, defaults to `None`) --
  random seed for the torch generator for random_mask.
- **target_modules** (`Union[List[str], str]`) --
  List of module names or regex expression of the module names to replace with SHiRA. For example, ['q', 'v']
  or '.*decoder.*(SelfAttention|EncDecAttention).*(q|v)$'. Only linear layers are supported.
- **fan_in_fan_out** (`bool`) --
  Set this to True if the layer to replace stores weight like (fan_in, fan_out). For example, gpt-2 uses
  `Conv1D` which stores weights like (fan_in, fan_out) and hence this should be set to `True`.
- **init_weights** (`bool`, defaults to `True`) --
  Initialize SHiRA weight to have zero values. If set to False, SHiRA weights are initialized to randn values
  instead of zeros and this is used only for testing.
- **modules_to_save** (`List[str]`) --
  List of modules apart from SHiRA layers to be set as trainable and saved in the final checkpoint.

This is the configuration class to store the configuration of a [ShiraModel](/docs/peft/v0.20.0/en/package_reference/shira#peft.ShiraModel).

## ShiraModel[[peft.ShiraModel]]

- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The model to be adapted.
- **config** ([ShiraConfig](/docs/peft/v0.20.0/en/package_reference/shira#peft.ShiraConfig)) -- The configuration of the SHiRA model.
- **adapter_name** (`str`) -- The name of the adapter, defaults to `"default"`.`torch.nn.Module`The SHiRA model.

Creates a Sparse High Rank Adapter (SHiRA) Model from a pretrained model.

Example:

```py
>>> from transformers import AutoModelForCausalLM
>>> from peft import ShiraConfig, get_peft_model

>>> base_model = AutoModelForCausalLM.from_pretrained("facebook/opt-125m")
>>> config = ShiraConfig(r=32)
>>> model = get_peft_model(base_model, config)
```

**Attributes**:
- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The model to be adapted.
- **peft_config** ([ShiraConfig](/docs/peft/v0.20.0/en/package_reference/shira#peft.ShiraConfig)): The configuration of the SHiRA model.
