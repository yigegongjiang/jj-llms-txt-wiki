# DeepSpeed utilities

## DeepSpeedPlugin

## get_active_deepspeed_plugin[[accelerate.utils.get_active_deepspeed_plugin]]

#### accelerate.utils.get_active_deepspeed_plugin[[accelerate.utils.get_active_deepspeed_plugin]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/deepspeed.py#L100)

Returns the currently active DeepSpeedPlugin.

#### accelerate.DeepSpeedPlugin[[accelerate.DeepSpeedPlugin]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L1120)

This plugin is used to integrate DeepSpeed.

deepspeed_config_processaccelerate.DeepSpeedPlugin.deepspeed_config_processhttps://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L1390[{"name": "prefix", "val": " = ''"}, {"name": "mismatches", "val": " = None"}, {"name": "config", "val": " = None"}, {"name": "must_match", "val": " = True"}, {"name": "**kwargs", "val": ""}]
Process the DeepSpeed config with the values from the kwargs.

**Parameters:**

hf_ds_config (`Any`, defaults to `None`) : Path to DeepSpeed config file or dict or an object of class `accelerate.utils.deepspeed.HfDeepSpeedConfig`.

gradient_accumulation_steps (`int`, defaults to `None`) : Number of steps to accumulate gradients before updating optimizer states. If not set, will use the value from the `Accelerator` directly.

gradient_clipping (`float`, defaults to `None`) : Enable gradient clipping with value.

zero_stage (`int`, defaults to `None`) : Possible options are 0, 1, 2, 3. Default will be taken from environment variable.

is_train_batch_min (`bool`, defaults to `True`) : If both train & eval dataloaders are specified, this will decide the `train_batch_size`.

offload_optimizer_device (`str`, defaults to `None`) : Possible options are none|cpu|nvme. Only applicable with ZeRO Stages 2 and 3.

offload_param_device (`str`, defaults to `None`) : Possible options are none|cpu|nvme. Only applicable with ZeRO Stage 3.

offload_optimizer_nvme_path (`str`, defaults to `None`) : Possible options are /nvme|/local_nvme. Only applicable with ZeRO Stage 3.

offload_param_nvme_path (`str`, defaults to `None`) : Possible options are /nvme|/local_nvme. Only applicable with ZeRO Stage 3.

zero3_init_flag (`bool`, defaults to `None`) : Flag to indicate whether to save 16-bit model. Only applicable with ZeRO Stage-3.

zero3_save_16bit_model (`bool`, defaults to `None`) : Flag to indicate whether to save 16-bit model. Only applicable with ZeRO Stage-3.

transformer_moe_cls_names (`str`, defaults to `None`) : Comma-separated list of Transformers MoE layer class names (case-sensitive). For example, `MixtralSparseMoeBlock`, `Qwen2MoeSparseMoeBlock`, `JetMoEAttention`, `JetMoEBlock`, etc.

enable_msamp (`bool`, defaults to `None`) : Flag to indicate whether to enable MS-AMP backend for FP8 training.

msasmp_opt_level (`Optional[Literal["O1", "O2"]]`, defaults to `None`) : Optimization level for MS-AMP (defaults to 'O1'). Only applicable if `enable_msamp` is True. Should be one of ['O1' or 'O2'].
#### select[[accelerate.DeepSpeedPlugin.select]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L1552)

Sets the HfDeepSpeedWeakref to use the current deepspeed plugin configuration

#### accelerate.utils.DummyScheduler[[accelerate.utils.DummyScheduler]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/deepspeed.py#L362)

Dummy scheduler presents model parameters or param groups, this is primarily used to follow conventional training
loop when scheduler config is specified in the deepspeed config file.

**Parameters:**

optimizer (`torch.optim.optimizer.Optimizer`) : The optimizer to wrap.

total_num_steps (int, *optional*) : Total number of steps.

warmup_num_steps (int, *optional*) : Number of steps for warmup.

lr_scheduler_callable (callable, *optional*) : A callable function that creates an LR Scheduler. It accepts only one argument `optimizer`.

- ****kwargs** (additional keyword arguments, *optional*) : Other arguments.

## DeepSpeedEnginerWrapper[[accelerate.utils.DeepSpeedEngineWrapper]]

#### accelerate.utils.DeepSpeedEngineWrapper[[accelerate.utils.DeepSpeedEngineWrapper]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/deepspeed.py#L253)

Internal wrapper for deepspeed.runtime.engine.DeepSpeedEngine. This is used to follow conventional training loop.

get_global_grad_normaccelerate.utils.DeepSpeedEngineWrapper.get_global_grad_normhttps://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/deepspeed.py#L286[]
Get the global gradient norm from DeepSpeed engine.

**Parameters:**

engine (deepspeed.runtime.engine.DeepSpeedEngine) : deepspeed engine to wrap

## DeepSpeedOptimizerWrapper[[accelerate.utils.DeepSpeedOptimizerWrapper]]

#### accelerate.utils.DeepSpeedOptimizerWrapper[[accelerate.utils.DeepSpeedOptimizerWrapper]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/deepspeed.py#L295)

Internal wrapper around a deepspeed optimizer.

**Parameters:**

optimizer (`torch.optim.optimizer.Optimizer`) : The optimizer to wrap.

## DeepSpeedSchedulerWrapper[[accelerate.utils.DeepSpeedSchedulerWrapper]]

#### accelerate.utils.DeepSpeedSchedulerWrapper[[accelerate.utils.DeepSpeedSchedulerWrapper]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/deepspeed.py#L322)

Internal wrapper around a deepspeed scheduler.

**Parameters:**

scheduler (`torch.optim.lr_scheduler.LambdaLR`) : The scheduler to wrap.

optimizers (one or a list of `torch.optim.Optimizer`) --

## DummyOptim[[accelerate.utils.DummyOptim]]

#### accelerate.utils.DummyOptim[[accelerate.utils.DummyOptim]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/deepspeed.py#L339)

Dummy optimizer presents model parameters or param groups, this is primarily used to follow conventional training
loop when optimizer config is specified in the deepspeed config file.

**Parameters:**

lr (float) : Learning rate.

params (iterable) : iterable of parameters to optimize or dicts defining parameter groups

weight_decay (float) : Weight decay.

- ****kwargs** (additional keyword arguments, *optional*) : Other arguments.

## DummyScheduler
