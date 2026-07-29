# PVeRA: Probabilistic Vector-Based Random Matrix Adaptation

[PVeRA](https://huggingface.co/papers/2512.07703) is a parameter-efficient fine-tuning technique that is base on VeRA, in the family of the LoRA-based adapters. It keeps the very low parameter budget of VeRA, but increases the performance by learning a distribution of latent adaptations. This also enables models adapted with PVeRA to generate Monte Carlo confidence interval estimates, by sampling from the learned distribution at inference.

When saving the adapter parameters, it's possible to eschew storing the low rank matrices by setting `save_projection=False` on the `PveraConfig`. In that case, these matrices will be restored based on the fixed random seed from the `projection_prng_key` argument. This cuts down on the size of the checkpoint, but we cannot guarantee reproducibility on all devices and for all future versions of PyTorch. If you want to ensure reproducibility, set `save_projection=True` (which is the default).

To handle different shapes of adapted layers, PVeRA initializes shared A and B matrices with the largest required size for each dimension. During the forward pass, submatrices A and B for a given layer are sliced out from these shared matrices and used as described in the paper. For example, adapting two linear layers of shapes (100, 20) and (80, 50) will create A and B matrices of shapes (rank, 50) and (100, rank) respectively. Then, to adapt a layer of shape (100, 20), submatrices A and B of shapes (rank, 20) and (100, rank) will be extracted.

PVeRA currently has the following constraint:

- Only `nn.Linear` layers are supported.
- The latent representation is not easily accessible, for training using the KL divergence.

The abstract from the paper is:

> Large foundation models have emerged in the last years and are pushing performance boundaries for a variety of tasks. Training or even finetuning such models demands vast datasets and computational resources, which are often scarce and costly. Adaptation methods provide a computationally efficient solution to address these limitations by allowing such models to be finetuned on small amounts of data and computing power. This is achieved by appending new trainable modules to frozen backbones with only a fraction of the trainable parameters and fitting only these modules on novel tasks. Recently, the VeRA adapter was shown to excel in parameter-efficient adaptations by utilizing a pair of frozen random low-rank matrices shared across all layers. In this paper, we propose PVeRA, a probabilistic version of the VeRA adapter, which modifies the low-rank matrices of VeRA in a probabilistic manner. This modification naturally allows handling inherent ambiguities in the input and allows for different sampling configurations during training and testing. A comprehensive evaluation was performed on the VTAB-1k benchmark and seven adapters, with PVeRA outperforming VeRA and other adapters.

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=PVERA"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## PveraConfig[[peft.PveraConfig]]

- **r** (`int`, *optional*, defaults to `256`) --
  PVeRA parameter dimension ("rank"). Choose higher values than LoRA ranks here, since PVeRA shares
  parameters across layers and therefore uses far fewer parameters than LoRA.
- **target_modules** (`Union[List[str], str]`) --
  The names of the modules to apply PVeRA to. Only linear layers are supported. When passing a string, a
  regex match will be performed. If this is specified as 'all-linear', then all linear/Conv1D modules are
  chosen. If this is not specified, modules will bechosen according to the model architecture. If the
  architecture is not known, an error will be raised.
- **projection_prng_key** (`int`) --
  PVeRA PRNG init key. Used for initialising pvera_A and pvera_B for new models or when loading a checkpoint
  that did not include these projections. Defaults to `0`.
- **save_projection** (`bool`) --
  Whether to save the pvera_A / pvera_B projections in the state dict alongside per layer lambda_b / lambda_d
  weights. This will increase the size of the checkpoint, but guarantee that we can reload the checkpoint on
  all system configurations. Defaults to `True`.
- **pvera_dropout** (`float`) --
  The dropout probability for PVeRA layers.
- **d_initial** (`float`, *optional*, defaults to `0.1`) --
  Initial value for `pvera_lambda_d` vector used when initializing the PVeRA parameters. Small values (<=0.1)
  are recommended.
- **fan_in_fan_out** (`bool`) --
  Set this to True if the layer to replace stores weight like (fan_in, fan_out). For example, gpt-2 uses
  `Conv1D` which stores weights like (fan_in, fan_out) and hence this should be set to `True`.
- **bias** (`str`) --
  Bias type for PVeRA. Can be 'none', 'all' or 'pvera_only'. If 'all' or 'pvera_only', the corresponding
  biases will be updated during training. Be aware that this means that, even when disabling the adapters,
  the model will not produce the same output as the base model would have without adaptation.
- **modules_to_save** (`List[str]`) --
  List of modules apart from PVeRA layers to be set as trainable and saved in the final checkpoint.
- **init_weights** (`bool`) --
  Whether to initialize the weights of the PVeRA layers with their default initialization. Don't change this
  setting, except if you know exactly what you're doing.
- **layers_to_transform** (`Union[List[int],int]`) --
  The layer indexes to transform, if this argument is specified, it will apply the PVeRA transformations on
  the layer indexes that are specified in this list. If a single integer is passed, it will apply the PVeRA
  transformations on the layer at this index.
- **layers_pattern** (`Optional[Union[List[str], str]]`) --
  The layer pattern name, used only if `layers_to_transform` is different from `None`. This should target the
  `nn.ModuleList` of the model, which is often called `'layers'` or `'h'`.
- **sample_at_inference** (`bool` | `dict`, defaults to `False`) --
  Whether to sample from the learned PVeRA distribution at inference. If false, the learned mean is used. The
  default is False (indicating false for all adapters). If True is provided, then the value will be true for
  all adapters. If a dict is provided, then a specific value can be specified per adapter (with False by
  default for non-specified adapters). For example
  `sample_at_inference={'encoder.layer.0.attention.attention.query': True}` will only sample at inference for
  one specific adapter.
- **generator_seed** (`int`, defaults to None) --
  Random seed for the generator for sampling from the learned distribution.

This is the configuration class to store the configuration of a [PveraModel](/docs/peft/v0.20.0/en/package_reference/pvera#peft.PveraModel).

Paper: https://www.arxiv.org/abs/2512.07703.

## PveraModel[[peft.PveraModel]]

- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The model to be adapted.
- **config** ([PveraConfig](/docs/peft/v0.20.0/en/package_reference/pvera#peft.PveraConfig)) -- The configuration of the PVeRA model.
- **adapter_name** (`str`) -- The name of the adapter, defaults to `"default"`.
- **low_cpu_mem_usage** (`bool`, `optional`, defaults to `False`) --
  Create empty adapter weights on meta device. Useful to speed up the loading process.`torch.nn.Module`The PVeRA model.

Creates Probabilistic Vector-based Random Matrix Adaptation (PVeRA) model from a pretrained transformers model.

Example:

```py
>>> from transformers import AutoModel
>>> from peft import PveraConfig, get_peft_model

>>> base_model = AutoModel.from_pretrained("facebook/dinov2-base")
>>> config = PveraConfig(r=128, sample_at_inference=False)
>>> model = get_peft_model(base_model, config)
```

**Attributes**:
- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The model to be adapted.
- **peft_config** ([PveraConfig](/docs/peft/v0.20.0/en/package_reference/pvera#peft.PveraConfig)): The configuration of the PVeRA model.
