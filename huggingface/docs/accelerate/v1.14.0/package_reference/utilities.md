# Utility functions and classes

Below are a variety of utility functions that 🤗 Accelerate provides, broken down by use-case. 

## Constants

Constants used throughout 🤗 Accelerate for reference

The following are constants used when utilizing [Accelerator.save_state()](/docs/accelerate/v1.14.0/en/package_reference/accelerator#accelerate.Accelerator.save_state)

`utils.MODEL_NAME`: `"pytorch_model"`
`utils.OPTIMIZER_NAME`: `"optimizer"`
`utils.RNG_STATE_NAME`: `"random_states"`
`utils.SCALER_NAME`: `"scaler.pt`
`utils.SCHEDULER_NAME`: `"scheduler`

The following are constants used when utilizing [Accelerator.save_model()](/docs/accelerate/v1.14.0/en/package_reference/accelerator#accelerate.Accelerator.save_model)

`utils.WEIGHTS_NAME`: `"pytorch_model.bin"`
`utils.SAFE_WEIGHTS_NAME`: `"model.safetensors"`
`utils.WEIGHTS_INDEX_NAME`: `"pytorch_model.bin.index.json"`
`utils.SAFE_WEIGHTS_INDEX_NAME`: `"model.safetensors.index.json"`

## Data Classes

These are basic dataclasses used throughout 🤗 Accelerate and they can be passed in as parameters.

### Standalone[[accelerate.utils.ComputeEnvironment]]

These are standalone dataclasses used for checks, such as the type of distributed system being used

#### accelerate.utils.ComputeEnvironment[[accelerate.utils.ComputeEnvironment]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L675)

Represents a type of the compute environment.

Values:

- **LOCAL_MACHINE** -- private/custom cluster hardware.
- **AMAZON_SAGEMAKER** -- Amazon SageMaker as compute environment.

#### accelerate.DistributedType[[accelerate.DistributedType]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L602)

Represents a type of distributed environment.

Values:

- **NO** -- Not a distributed environment, just a single process.
- **MULTI_CPU** -- Distributed on multiple CPU nodes.
- **MULTI_GPU** -- Distributed on multiple GPUs.
- **MULTI_MLU** -- Distributed on multiple MLUs.
- **MULTI_SDAA** -- Distributed on multiple SDAAs.
- **MULTI_MUSA** -- Distributed on multiple MUSAs.
- **MULTI_NPU** -- Distributed on multiple NPUs.
- **MULTI_XPU** -- Distributed on multiple XPUs.
- **MULTI_HPU** -- Distributed on multiple HPUs.
- **MULTI_NEURON** -- Distributed on multiple Neuron cores.
- **DEEPSPEED** -- Using DeepSpeed.
- **FSDP** -- Using Fully Sharded Data Parallelism (FSDP).
- **XLA** -- Using TorchXLA.
- **MEGATRON_LM** -- Using Megatron-LM.

#### accelerate.utils.DynamoBackend[[accelerate.utils.DynamoBackend]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L690)

Represents a dynamo backend (see https://pytorch.org/docs/stable/torch.compiler.html).

Values:

- **NO** -- Do not use torch dynamo.
- **EAGER** -- Uses PyTorch to run the extracted GraphModule. This is quite useful in debugging TorchDynamo
  issues.
- **AOT_EAGER** -- Uses AotAutograd with no compiler, i.e, just using PyTorch eager for the AotAutograd's
  extracted forward and backward graphs. This is useful for debugging, and unlikely to give speedups.
- **INDUCTOR** -- Uses TorchInductor backend with AotAutograd and cudagraphs by leveraging codegened Triton
  kernels. [Read
  more](https://dev-discuss.pytorch.org/t/torchinductor-a-pytorch-native-compiler-with-define-by-run-ir-and-symbolic-shapes/747)
- **AOT_TS_NVFUSER** -- nvFuser with AotAutograd/TorchScript. [Read
  more](https://dev-discuss.pytorch.org/t/tracing-with-primitives-update-1-nvfuser-and-its-primitives/593)
- **NVPRIMS_NVFUSER** -- nvFuser with PrimTorch. [Read
  more](https://dev-discuss.pytorch.org/t/tracing-with-primitives-update-1-nvfuser-and-its-primitives/593)
- **CUDAGRAPHS** -- cudagraphs with AotAutograd. [Read more](https://github.com/pytorch/torchdynamo/pull/757)
- **OFI** -- Uses Torchscript optimize_for_inference. Inference only. [Read
  more](https://pytorch.org/docs/stable/generated/torch.jit.optimize_for_inference.html)
- **FX2TRT** -- Uses Nvidia TensorRT for inference optimizations. Inference only. [Read
  more](https://github.com/pytorch/TensorRT/blob/master/docsrc/tutorials/getting_started_with_fx_path.rst)
- **ONNXRT** -- Uses ONNXRT for inference on CPU/GPU. Inference only. [Read more](https://onnxruntime.ai/)
- **TENSORRT** -- Uses ONNXRT to run TensorRT for inference optimizations. [Read
  more](https://github.com/onnx/onnx-tensorrt)
- **AOT_TORCHXLA_TRACE_ONCE** -- Uses Pytorch/XLA with TorchDynamo optimization, for training. [Read
  more](https://github.com/pytorch/xla/blob/r2.0/docs/dynamo.md)
- **TORCHXLA_TRACE_ONCE** -- Uses Pytorch/XLA with TorchDynamo optimization, for inference. [Read
  more](https://github.com/pytorch/xla/blob/r2.0/docs/dynamo.md)
- **TVM** -- Uses Apache TVM for inference optimizations. [Read more](https://tvm.apache.org/)
- **HPU_BACKEND** -- Uses HPU backend for inference optimizations.

#### accelerate.utils.LoggerType[[accelerate.utils.LoggerType]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L743)

Represents a type of supported experiment tracker

Values:

- **ALL** -- all available trackers in the environment that are supported
- **TENSORBOARD** -- TensorBoard as an experiment tracker
- **WANDB** -- wandb as an experiment tracker
- **TRACKIO** -- trackio as an experiment tracker
- **COMETML** -- comet_ml as an experiment tracker
- **MLFLOW** -- mlflow as an experiment tracker
- **CLEARML** -- clearml as an experiment tracker
- **DVCLIVE** -- dvclive as an experiment tracker
- **SWANLAB** -- swanlab as an experiment tracker

#### accelerate.utils.PrecisionType[[accelerate.utils.PrecisionType]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L771)

Represents a type of precision used on floating point values

Values:

- **NO** -- using full precision (FP32)
- **FP16** -- using half precision
- **BF16** -- using brain floating point precision

#### accelerate.utils.RNGType[[accelerate.utils.RNGType]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L787)

An enumeration.

#### accelerate.utils.SageMakerDistributedType[[accelerate.utils.SageMakerDistributedType]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L641)

Represents a type of distributed environment.

Values:

- **NO** -- Not a distributed environment, just a single process.
- **DATA_PARALLEL** -- using sagemaker distributed data parallelism.
- **MODEL_PARALLEL** -- using sagemaker distributed model parallelism.

### Kwargs[[accelerate.AutocastKwargs]]

These are configurable arguments for specific interactions throughout the PyTorch ecosystem that Accelerate handles under the hood.

#### accelerate.AutocastKwargs[[accelerate.AutocastKwargs]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L115)

Use this object in your [Accelerator](/docs/accelerate/v1.14.0/en/package_reference/accelerator#accelerate.Accelerator) to customize how `torch.autocast` behaves. Please refer to the
documentation of this [context manager](https://pytorch.org/docs/stable/amp.html#torch.autocast) for more
information on each argument.

Example:

```python
from accelerate import Accelerator
from accelerate.utils import AutocastKwargs

kwargs = AutocastKwargs(cache_enabled=True)
accelerator = Accelerator(kwargs_handlers=[kwargs])
```

#### accelerate.DistributedDataParallelKwargs[[accelerate.DistributedDataParallelKwargs]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L157)

Use this object in your [Accelerator](/docs/accelerate/v1.14.0/en/package_reference/accelerator#accelerate.Accelerator) to customize how your model is wrapped in a
`torch.nn.parallel.DistributedDataParallel`. Please refer to the documentation of this
[wrapper](https://pytorch.org/docs/stable/generated/torch.nn.parallel.DistributedDataParallel.html) for more
information on each argument.

`gradient_as_bucket_view` is only available in PyTorch 1.7.0 and later versions.

`static_graph` is only available in PyTorch 1.11.0 and later versions.

Example:

```python
from accelerate import Accelerator
from accelerate.utils import DistributedDataParallelKwargs

kwargs = DistributedDataParallelKwargs(find_unused_parameters=True)
accelerator = Accelerator(kwargs_handlers=[kwargs])
```

#### accelerate.utils.FP8RecipeKwargs[[accelerate.utils.FP8RecipeKwargs]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L457)

Deprecated. Please use one of the proper FP8 recipe kwargs classes such as `TERecipeKwargs` or `MSAMPRecipeKwargs`
instead.

#### accelerate.GradScalerKwargs[[accelerate.GradScalerKwargs]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L243)

Use this object in your [Accelerator](/docs/accelerate/v1.14.0/en/package_reference/accelerator#accelerate.Accelerator) to customize the behavior of mixed precision, specifically how the
`torch.amp.GradScaler` or `torch.cuda.amp.GradScaler` used is created. Please refer to the documentation of this
[scaler](https://pytorch.org/docs/stable/amp.html?highlight=gradscaler) for more information on each argument.

`torch.cuda.amp.GradScaler` is only available in PyTorch 1.5.0 and later versions, and `torch.amp.GradScaler` is
only available in PyTorch 2.4.0 and later versions.

Example:

```python
from accelerate import Accelerator
from accelerate.utils import GradScalerKwargs

kwargs = GradScalerKwargs(backoff_factor=0.25)
accelerator = Accelerator(kwargs_handlers=[kwargs])
```

#### accelerate.InitProcessGroupKwargs[[accelerate.InitProcessGroupKwargs]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L275)

Use this object in your [Accelerator](/docs/accelerate/v1.14.0/en/package_reference/accelerator#accelerate.Accelerator) to customize the initialization of the distributed processes. Please refer
to the documentation of this
[method](https://pytorch.org/docs/stable/distributed.html#torch.distributed.init_process_group) for more
information on each argument.

Note: If `timeout` is set to `None`, the default will be based upon how `backend` is set.

```python
from datetime import timedelta
from accelerate import Accelerator
from accelerate.utils import InitProcessGroupKwargs

kwargs = InitProcessGroupKwargs(timeout=timedelta(seconds=800))
accelerator = Accelerator(kwargs_handlers=[kwargs])
```

#### accelerate.utils.KwargsHandler[[accelerate.utils.KwargsHandler]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L70)

Internal mixin that implements a `to_kwargs()` method for a dataclass.

to_kwargsaccelerate.utils.KwargsHandler.to_kwargshttps://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L78[]

Returns a dictionary containing the attributes with values different from the default of this class.

## Plugins[[accelerate.DeepSpeedPlugin]]

These are plugins that can be passed to the [Accelerator](/docs/accelerate/v1.14.0/en/package_reference/accelerator#accelerate.Accelerator) object. While they are defined elsewhere in the documentation, 
for convenience all of them are available to see here:

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

#### accelerate.FullyShardedDataParallelPlugin[[accelerate.FullyShardedDataParallelPlugin]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L1584)

This plugin is used to enable fully sharded data parallelism.

set_auto_wrap_policyaccelerate.FullyShardedDataParallelPlugin.set_auto_wrap_policyhttps://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L2056[{"name": "model", "val": ""}]

Given `model`, creates an `auto_wrap_policy` based on the passed in policy and if we can use the
`transformer_cls_to_wrap`

**Parameters:**

fsdp_version (`int`, defaults to `1`) : The version of FSDP to use. Defaults to 1. If set to 2, launcher expects the config to be converted to FSDP2 format.

sharding_strategy (`Union[str, torch.distributed.fsdp.ShardingStrategy]`, defaults to `'FULL_SHARD'`) : Sharding strategy to use. Should be either a `str` or an instance of `torch.distributed.fsdp.fully_sharded_data_parallel.ShardingStrategy`. Is deprecated in favor of `reshard_after_forward`.

reshard_after_forward (`Union[str, torch.distributed.fsdp.ShardingStrategy, bool]`, defaults to `'FULL_SHARD'` for `fsdp_version=1` and `True` for `fsdp_version=2`) : Sharding strategy to use. Should be a bool if `fsdp_version` is set to 2 else a `str` or an instance of `torch.distributed.fsdp.fully_sharded_data_parallel.ShardingStrategy`.

backward_prefetch (`Union[str, torch.distributed.fsdp.BackwardPrefetch]`, defaults to `'NO_PREFETCH'`) : Backward prefetch strategy to use. Should be either a `str` or an instance of `torch.distributed.fsdp.fully_sharded_data_parallel.BackwardPrefetch`.

mixed_precision_policy (`Optional[Union[dict, str, torch.distributed.fsdp.MixedPrecision, torch.distributed.fsdp.MixedPrecisionPolicy]]`, defaults to `None`) : A config to enable mixed precision training with FullyShardedDataParallel. If passing in a `dict`, it should have the following keys: `param_dtype`, `reduce_dtype`, and `buffer_dtype`, can be an instance of `torch.distributed.fsdp.MixedPrecisionPolicy` if `fsdp_version` is set to 2. If passing in a `str`, it should be one of the following values: fp8, fp16, bf16, fp32, and used to set `param_dtype`, `reduce_dtype`, and `buffer_dtype`.

auto_wrap_policy (`Optional(Union[Callable, Literal["transformer_based_wrap", "size_based_wrap", "no_wrap"]]), defaults to `NO_WRAP`) : A callable or string specifying a policy to recursively wrap layers with FSDP. If a string, it must be one of `transformer_based_wrap`, `size_based_wrap`, or `no_wrap`. See `torch.distributed.fsdp.wrap.size_based_wrap_policy` for a direction on what it should look like.

cpu_offload (`Union[bool, torch.distributed.fsdp.CPUOffload, torch.distributed.fsdp.CPUOffloadPolicy]`, defaults to `False`) : Whether to offload parameters to CPU. Should be either a `bool` or an instance of `torch.distributed.fsdp.fully_sharded_data_parallel.CPUOffload` or `torch.distributed.fsdp.fully_sharded_data_parallel.CPUOffloadPolicy` if `fsdp_version` is set to 2.

ignored_modules (`Optional[Union[Iterable[torch.nn.Module], str]]`, defaults to `None`) : A list of modules to ignore when wrapping with FSDP. When passing a string, will match the modules by name using regex fullmatch. If `fsdp_version` is set to 2, the modules are converted to parameters and used.

state_dict_type (`Union[str, torch.distributed.fsdp.StateDictType]`, defaults to `'FULL_STATE_DICT'`) : State dict type to use. If a string, it must be one of `full_state_dict`, `local_state_dict`, or `sharded_state_dict`.

state_dict_config (`Optional[Union[torch.distributed.fsdp.FullStateDictConfig, torch.distributed.fsdp.ShardedStateDictConfig]`, defaults to `None`) : State dict config to use. Is determined based on the `state_dict_type` if not passed in.

optim_state_dict_config (`Optional[Union[torch.distributed.fsdp.FullOptimStateDictConfig, torch.distributed.fsdp.ShardedOptimStateDictConfig]`, defaults to `None`) : Optim state dict config to use. Is determined based on the `state_dict_type` if not passed in.

limit_all_gathers (`bool`, defaults to `True`) : Whether to have FSDP explicitly synchronizes the CPU thread to prevent too many in-flight all-gathers. This bool only affects the sharded strategies that schedule all-gathers. Enabling this can help lower the number of CUDA malloc retries.

use_orig_params (`bool`, defaults to `False`) : Whether to use the original parameters for the optimizer.

param_init_fn (`Optional[Callable[[torch.nn.Module], None]`, defaults to `None`) : A `Callable[torch.nn.Module] -> None` that specifies how modules that are currently on the meta device should be initialized onto an actual device. Only applicable when `sync_module_states` is `True`. By default is a `lambda` which calls `to_empty` on the module.

sync_module_states (`bool`, defaults to `False`) : Whether each individually wrapped FSDP unit should broadcast module parameters from rank 0 to ensure they are the same across all ranks after initialization. Defaults to `False` unless `cpu_ram_efficient_loading` is `True`, then will be forcibly enabled.

forward_prefetch (`bool`, defaults to `False`) : Whether to have FSDP explicitly prefetches the next upcoming all-gather while executing in the forward pass. only use with Static graphs.

activation_checkpointing (`bool`, defaults to `False`) : A technique to reduce memory usage by clearing activations of certain layers and recomputing them during a backward pass. Effectively, this trades extra computation time for reduced memory usage.

cpu_ram_efficient_loading (`bool`, defaults to `None`) : If True, only the first process loads the pretrained model checkpoint while all other processes have empty weights. Only applicable for Transformers. When using this, `sync_module_states` needs to be `True`.

transformer_cls_names_to_wrap (`Optional[List[str]]`, defaults to `None`) : A list of transformer layer class names to wrap. Only applicable when `auto_wrap_policy` is `transformer_based_wrap`.

min_num_params (`Optional[int]`, defaults to `None`) : The minimum number of parameters a module must have to be wrapped. Only applicable when `auto_wrap_policy` is `size_based_wrap`.
#### set_mixed_precision[[accelerate.FullyShardedDataParallelPlugin.set_mixed_precision]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L2090)

Sets the mixed precision policy for FSDP
#### set_state_dict_type[[accelerate.FullyShardedDataParallelPlugin.set_state_dict_type]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L2011)

Set the state dict config based on the `StateDictType`.
#### validate_mixed_precision_policy[[accelerate.FullyShardedDataParallelPlugin.validate_mixed_precision_policy]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L2142)

Validates the mixed precision policy, abstracted away to not bring in the imports if not needed.

#### accelerate.utils.GradientAccumulationPlugin[[accelerate.utils.GradientAccumulationPlugin]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L979)

A plugin to configure gradient accumulation behavior. You can only pass one of `gradient_accumulation_plugin` or
`gradient_accumulation_steps` to [Accelerator](/docs/accelerate/v1.14.0/en/package_reference/accelerator#accelerate.Accelerator). Passing both raises an error.

Example:

```python
from accelerate.utils import GradientAccumulationPlugin

gradient_accumulation_plugin = GradientAccumulationPlugin(num_steps=2)
accelerator = Accelerator(gradient_accumulation_plugin=gradient_accumulation_plugin)
```

**Parameters:**

num_steps (`int`) : The number of steps to accumulate gradients for.

adjust_scheduler (`bool`, *optional*, defaults to `True`) : Whether to adjust the scheduler steps to account for the number of steps being accumulated. Should be `True` if the used scheduler was not adjusted for gradient accumulation.

sync_with_dataloader (`bool`, *optional*, defaults to `True`) : Whether to synchronize setting the gradients when at the end of the dataloader.

sync_each_batch (`bool`, *optional*) : Whether to synchronize setting the gradients at each data batch. Setting to `True` may reduce memory requirements when using gradient accumulation with distributed training, at expense of speed.

#### accelerate.utils.MegatronLMPlugin[[accelerate.utils.MegatronLMPlugin]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L2316)

Plugin for Megatron-LM to enable tensor, pipeline, sequence and data parallelism. Also to enable selective
activation recomputation and optimized fused kernels.

**Parameters:**

tp_degree (`int`, defaults to `None`) : Tensor parallelism degree.

pp_degree (`int`, defaults to `None`) : Pipeline parallelism degree.

num_micro_batches (`int`, defaults to `None`) : Number of micro-batches.

gradient_clipping (`float`, defaults to `None`) : Gradient clipping value based on global L2 Norm (0 to disable).

sequence_parallelism (`bool`, defaults to `None`) : Enable sequence parallelism.

recompute_activations (`bool`, defaults to `None`) : Enable selective activation recomputation.

use_distributed_optimizr (`bool`, defaults to `None`) : Enable distributed optimizer.

pipeline_model_parallel_split_rank (`int`, defaults to `None`) : Rank where encoder and decoder should be split.

num_layers_per_virtual_pipeline_stage (`int`, defaults to `None`) : Number of layers per virtual pipeline stage.

is_train_batch_min (`str`, defaults to `True`) : If both tran & eval dataloaders are specified, this will decide the `micro_batch_size`.

train_iters (`int`, defaults to `None`) : Total number of samples to train over all training runs. Note that either train-iters or train-samples should be provided when using `MegatronLMDummyScheduler`.

train_samples (`int`, defaults to `None`) : Total number of samples to train over all training runs. Note that either train-iters or train-samples should be provided when using `MegatronLMDummyScheduler`.

weight_decay_incr_style (`str`, defaults to `'constant'`) : Weight decay increment function. choices=["constant", "linear", "cosine"].

start_weight_decay (`float`, defaults to `None`) : Initial weight decay coefficient for L2 regularization.

end_weight_decay (`float`, defaults to `None`) : End of run weight decay coefficient for L2 regularization.

lr_decay_style (`str`, defaults to `'linear'`) : Learning rate decay function. choices=['constant', 'linear', 'cosine'].

lr_decay_iters (`int`, defaults to `None`) : Number of iterations for learning rate decay. If None defaults to `train_iters`.

lr_decay_samples (`int`, defaults to `None`) : Number of samples for learning rate decay. If None defaults to `train_samples`.

lr_warmup_iters (`int`, defaults to `None`) : Number of iterations to linearly warmup learning rate over.

lr_warmup_samples (`int`, defaults to `None`) : Number of samples to linearly warmup learning rate over.

lr_warmup_fraction (`float`, defaults to `None`) : Fraction of lr-warmup-(iters/samples) to linearly warmup learning rate over.

min_lr (`float`, defaults to `0`) : Minimum value for learning rate. The scheduler clip values below this threshold.

consumed_samples (`List`, defaults to `None`) : Number of samples consumed in the same order as the dataloaders to `accelerator.prepare` call.

no_wd_decay_cond (`Optional`, defaults to `None`) : Condition to disable weight decay.

scale_lr_cond (`Optional`, defaults to `None`) : Condition to scale learning rate.

lr_mult (`float`, defaults to `1.0`) : Learning rate multiplier.

megatron_dataset_flag (`bool`, defaults to `False`) : Whether the format of dataset follows Megatron-LM Indexed/Cached/MemoryMapped format.

seq_length (`int`, defaults to `None`) : Maximum sequence length to process.

encoder_seq_length (`int`, defaults to `None`) : Maximum sequence length to process for the encoder.

decoder_seq_length (`int`, defaults to `None`) : Maximum sequence length to process for the decoder.

tensorboard_dir (`str`, defaults to `None`) : Path to save tensorboard logs.

set_all_logging_options (`bool`, defaults to `False`) : Whether to set all logging options.

eval_iters (`int`, defaults to `100`) : Number of iterations to run for evaluation validation/test for.

eval_interval (`int`, defaults to `1000`) : Interval between running evaluation on validation set.

return_logits (`bool`, defaults to `False`) : Whether to return logits from the model.

custom_train_step_class (`Optional`, defaults to `None`) : Custom train step class.

custom_train_step_kwargs (`Optional`, defaults to `None`) : Custom train step kwargs.

custom_model_provider_function (`Optional`, defaults to `None`) : Custom model provider function.

custom_prepare_model_function (`Optional`, defaults to `None`) : Custom prepare model function.

custom_megatron_datasets_provider_function (`Optional`, defaults to `None`) : Custom megatron train_valid_test datasets provider function.

custom_get_batch_function (`Optional`, defaults to `None`) : Custom get batch function.

custom_loss_function (`Optional`, defaults to `None`) : Custom loss function.

other_megatron_args (`Optional`, defaults to `None`) : Other Megatron-LM arguments. Please refer Megatron-LM.

#### accelerate.utils.TorchDynamoPlugin[[accelerate.utils.TorchDynamoPlugin]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L1031)

This plugin is used to compile a model with PyTorch 2.0

**Parameters:**

backend (`DynamoBackend`, defaults to `None`) : A valid Dynamo backend. See https://pytorch.org/docs/stable/torch.compiler.html for more details.

mode (`str`, defaults to `None`) : Possible options are 'default', 'reduce-overhead' or 'max-autotune'.

fullgraph (`bool`, defaults to `None`) : Whether it is ok to break model into several subgraphs.

dynamic (`bool`, defaults to `None`) : Whether to use dynamic shape for tracing.

options (`Any`, defaults to `None`) : A dictionary of options to pass to the backend.

disable (`bool`, defaults to `False`) : Turn torch.compile() into a no-op for testing

use_regional_compilation (`bool`, defaults to `None`) : Use it to reduce the cold start compilation time of torch.compile() by targeting repeated blocks of the same class and compiling them sequentially to hit the compiler's cache. For example, in `GPT2LMHeadModel`, the repeated block/class is `GPT2Block`, and can be accessed as `model.transformer.h[0]`. The rest of the model (e.g model.lm_head) is compiled separately.

## Configurations[[accelerate.utils.BnbQuantizationConfig]]

These are classes which can be configured and passed through to the appropriate integration

#### accelerate.utils.BnbQuantizationConfig[[accelerate.utils.BnbQuantizationConfig]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L3055)

A plugin to enable BitsAndBytes 4bit and 8bit quantization

**Parameters:**

load_in_8bit (`bool`, defaults to `False`) : Enable 8bit quantization.

llm_int8_threshold (`float`, defaults to `6.0`) : Value of the outliner threshold. Only relevant when `load_in_8bit=True`.

load_in_4bit (`bool`, defaults to `False`) : Enable 4bit quantization.

bnb_4bit_quant_type (`str`, defaults to `fp4`) : Set the quantization data type in the `bnb.nn.Linear4Bit` layers. Options are {'fp4','np4'}.

bnb_4bit_use_double_quant (`bool`, defaults to `False`) : Enable nested quantization where the quantization constants from the first quantization are quantized again.

bnb_4bit_compute_dtype (`bool`, defaults to `fp16`) : This sets the computational type which might be different than the input time. For example, inputs might be fp32, but computation can be set to bf16 for speedups. Options are {'fp32','fp16','bf16'}.

torch_dtype (`torch.dtype`, defaults to `None`) : This sets the dtype of the remaining non quantized layers. `bitsandbytes` library suggests to set the value to `torch.float16` for 8 bit model and use the same dtype as the compute dtype for 4 bit model.

skip_modules (`List[str]`, defaults to `None`) : An explicit list of the modules that we don't quantize. The dtype of these modules will be `torch_dtype`.

keep_in_fp32_modules (`List`, defaults to `None`) : An explicit list of the modules that we don't quantize. We keep them in `torch.float32`.

#### accelerate.DataLoaderConfiguration[[accelerate.DataLoaderConfiguration]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L821)

Configuration for dataloader-related items when calling `accelerator.prepare`.

**Parameters:**

split_batches (`bool`, defaults to `False`) : Whether or not the accelerator should split the batches yielded by the dataloaders across the devices. If `True`, the actual batch size used will be the same on any kind of distributed processes, but it must be a round multiple of `num_processes` you are using. If `False`, actual batch size used will be the one set in your script multiplied by the number of processes.

dispatch_batches (`bool`, defaults to `None`) : If set to `True`, the dataloader prepared by the Accelerator is only iterated through on the main process and then the batches are split and broadcast to each process. Will default to `True` for `DataLoader` whose underlying dataset is an `IterableDataset`, `False` otherwise.

even_batches (`bool`, defaults to `True`) : If set to `True`, in cases where the total batch size across all processes does not exactly divide the dataset, samples at the start of the dataset will be duplicated so the batch can be divided equally among all workers.

use_seedable_sampler (`bool`, defaults to `False`) : Whether or not use a fully seedable random sampler (`data_loader.SeedableRandomSampler`). Ensures training results are fully reproducible using a different sampling technique. While seed-to-seed results may differ, on average the differences are negligible when using multiple different seeds to compare. Should also be ran with [set_seed()](/docs/accelerate/v1.14.0/en/package_reference/utilities#accelerate.utils.set_seed) for the best results.

data_seed (`int`, defaults to `None`) : The seed to use for the underlying generator when using `use_seedable_sampler`. If `None`, the generator will use the current default seed from torch.

non_blocking (`bool`, defaults to `False`) : If set to `True`, the dataloader prepared by the Accelerator will utilize non-blocking host-to-device transfers, allowing for better overlap between dataloader communication and computation. Recommended that the prepared dataloader has `pin_memory` set to `True` to work properly.

use_stateful_dataloader (`bool`, defaults to `False`) : If set to `True`, the dataloader prepared by the Accelerator will be backed by [torchdata.StatefulDataLoader](https://github.com/pytorch/data/tree/main/torchdata/stateful_dataloader). This requires `torchdata` version 0.8.0 or higher that supports StatefulDataLoader to be installed.

#### accelerate.utils.ProjectConfiguration[[accelerate.utils.ProjectConfiguration]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L916)

Configuration for the Accelerator object based on inner-project needs.

set_directoriesaccelerate.utils.ProjectConfiguration.set_directorieshttps://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L968[{"name": "project_dir", "val": ": typing.Optional[str] = None"}]
Sets `self.project_dir` and `self.logging_dir` to the appropriate values.

**Parameters:**

project_dir (`str`, defaults to `None`) : A path to a directory for storing data.

logging_dir (`str`, defaults to `None`) : A path to a directory for storing logs of locally-compatible loggers. If None, defaults to `project_dir`.

automatic_checkpoint_naming (`bool`, defaults to `False`) : Whether saved states should be automatically iteratively named.

total_limit (`int`, defaults to `None`) : The maximum number of total saved states to keep.

iteration (`int`, defaults to `0`) : The current save iteration.

save_on_each_node (`bool`, defaults to `False`) : When doing multi-node distributed training, whether to save models and checkpoints on each node, or only on the main one.

## Environmental Variables

These are environmental variables that can be enabled for different use cases

* `ACCELERATE_DEBUG_MODE` (`str`): Whether to run accelerate in debug mode. More info available [here](../usage_guides/debug).

## Data Manipulation and Operations[[accelerate.utils.broadcast]]

These include data operations that mimic the same `torch` ops but can be used on distributed processes.

#### accelerate.utils.broadcast[[accelerate.utils.broadcast]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/operations.py#L600)

Recursively broadcast tensor in a nested list/tuple/dictionary of tensors to all devices.

**Parameters:**

tensor (nested list/tuple/dictionary of `torch.Tensor`) : The data to gather.

from_process (`int`, *optional*, defaults to 0) : The process from which to send the data

**Returns:**

The same data structure as `tensor` with all tensors broadcasted to the proper device.

#### accelerate.utils.broadcast_object_list[[accelerate.utils.broadcast_object_list]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/operations.py#L675)

Broadcast a list of picklable objects from one process to the others.

**Parameters:**

object_list (list of picklable objects) : The list of objects to broadcast. This list will be modified inplace.

from_process (`int`, *optional*, defaults to 0) : The process from which to send the data.

**Returns:**

The same list containing the objects from process 0.

#### accelerate.utils.concatenate[[accelerate.utils.concatenate]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/operations.py#L719)

Recursively concatenate the tensors in a nested list/tuple/dictionary of lists of tensors with the same shape.
If there is only a single batch of data, it is returned as-is.

**Parameters:**

data (nested list/tuple/dictionary of lists of tensors `torch.Tensor`) : The data to concatenate.

dim (`int`, *optional*, defaults to 0) : The dimension on which to concatenate.

**Returns:**

The same data structure as `data` with all the tensors concatenated.

#### accelerate.utils.convert_outputs_to_fp32[[accelerate.utils.convert_outputs_to_fp32]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/operations.py#L939)

#### accelerate.utils.convert_to_fp32[[accelerate.utils.convert_to_fp32]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/operations.py#L889)

Recursively converts the elements nested list/tuple/dictionary of tensors in FP16/BF16 precision to FP32.

**Parameters:**

tensor (nested list/tuple/dictionary of `torch.Tensor`) : The data to convert from FP16/BF16 to FP32.

**Returns:**

The same data structure as `tensor` with all tensors that were in FP16/BF16 precision converted to FP32.

#### accelerate.utils.gather[[accelerate.utils.gather]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/operations.py#L424)

Recursively gather tensor in a nested list/tuple/dictionary of tensors from all devices.

**Parameters:**

tensor (nested list/tuple/dictionary of `torch.Tensor`) : The data to gather.

**Returns:**

The same data structure as `tensor` with all tensors sent to the proper device.

#### accelerate.utils.gather_object[[accelerate.utils.gather_object]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/operations.py#L505)

Recursively gather object in a nested list/tuple/dictionary of objects from all devices.

**Parameters:**

object (nested list/tuple/dictionary of picklable object) : The data to gather.

**Returns:**

The same data structure as `object` with all the objects sent to every device.

#### accelerate.utils.get_grad_scaler[[accelerate.utils.get_grad_scaler]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/modeling.py#L2110)

A generic helper which will initialize the correct `GradScaler` implementation based on the environment and return
it.

**Parameters:**

distributed_type (`DistributedType`, *optional*, defaults to None) : The type of distributed environment.

kwargs : Additional arguments for the utilized `GradScaler` constructor.

#### accelerate.utils.get_mixed_precision_context_manager[[accelerate.utils.get_mixed_precision_context_manager]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/modeling.py#L2066)

Return a context manager for autocasting mixed precision

**Parameters:**

native_amp (`bool`, *optional*, defaults to False) : Whether mixed precision is actually enabled.

cache_enabled (`bool`, *optional*, defaults to True) : Whether the weight cache inside autocast should be enabled.

#### accelerate.utils.listify[[accelerate.utils.listify]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/operations.py#L284)

Recursively finds tensors in a nested list/tuple/dictionary and converts them to a list of numbers.

**Parameters:**

data (nested list/tuple/dictionary of `torch.Tensor`) : The data from which to convert to regular numbers.

**Returns:**

The same data structure as `data` with lists of numbers instead of `torch.Tensor`.

#### accelerate.utils.pad_across_processes[[accelerate.utils.pad_across_processes]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/operations.py#L749)

Recursively pad the tensors in a nested list/tuple/dictionary of tensors from all devices to the same size so they
can safely be gathered.

**Parameters:**

tensor (nested list/tuple/dictionary of `torch.Tensor`) : The data to gather.

dim (`int`, *optional*, defaults to 0) : The dimension on which to pad.

pad_index (`int`, *optional*, defaults to 0) : The value with which to pad.

pad_first (`bool`, *optional*, defaults to `False`) : Whether to pad at the beginning or the end.

#### accelerate.utils.recursively_apply[[accelerate.utils.recursively_apply]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/operations.py#L85)

Recursively apply a function on a data structure that is a nested list/tuple/dictionary of a given base type.

**Parameters:**

func (`callable`) : The function to recursively apply.

data (nested list/tuple/dictionary of `main_type`) : The data on which to apply `func`

- ***args** : Positional arguments that will be passed to `func` when applied on the unpacked data.

main_type (`type`, *optional*, defaults to `torch.Tensor`) : The base type of the objects to which apply `func`.

error_on_other_type (`bool`, *optional*, defaults to `False`) : Whether to return an error or not if after unpacking `data`, we get on an object that is not of type `main_type`. If `False`, the function will leave objects of types different than `main_type` unchanged.

- ****kwargs** (additional keyword arguments, *optional*) : Keyword arguments that will be passed to `func` when applied on the unpacked data.

**Returns:**

The same data structure as `data` with `func` applied to every object of type `main_type`.

#### accelerate.utils.reduce[[accelerate.utils.reduce]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/operations.py#L845)

Recursively reduce the tensors in a nested list/tuple/dictionary of lists of tensors across all processes by the
mean of a given operation.

**Parameters:**

tensor (nested list/tuple/dictionary of `torch.Tensor`) : The data to reduce.

reduction (`str`, *optional*, defaults to `"mean"`) : A reduction method. Can be of "mean", "sum", "max", or "none"

scale (`float`, *optional*) : A default scaling value to be applied after the reduce, only valid on XLA.

**Returns:**

The same data structure as `data` with all the tensors reduced.

#### accelerate.utils.send_to_device[[accelerate.utils.send_to_device]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/operations.py#L136)

Recursively sends the elements in a nested list/tuple/dictionary of tensors to a given device.

**Parameters:**

tensor (nested list/tuple/dictionary of `torch.Tensor`) : The data to send to a given device.

device (`torch.device`) : The device to send the data to.

non_blocking (`bool`, *optional*, defaults to `False`) : If `True`, the transfer to the device is performed asynchronously, which can overlap data movement with computation. Only effective when the device supports it (e.g. CUDA).

skip_keys (`str` or `List[str]`, *optional*) : A key or list of keys in a dictionary `tensor` whose values should not be sent to the given `device`. Entries with these keys are left on their original device.

**Returns:**

The same data structure as `tensor` with all tensors sent to the proper device.

#### accelerate.utils.slice_tensors[[accelerate.utils.slice_tensors]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/operations.py#L699)

Recursively takes a slice in a nested list/tuple/dictionary of tensors.

**Parameters:**

data (nested list/tuple/dictionary of `torch.Tensor`) : The data to slice.

tensor_slice (`slice`) : The slice to take.

**Returns:**

The same data structure as `data` with all the tensors slices.

## Environment Checks[[accelerate.utils.is_bf16_available]]

These functionalities check the state of the current working environment including information about the operating system itself, what it can support, and if particular dependencies are installed. 

#### accelerate.utils.is_bf16_available[[accelerate.utils.is_bf16_available]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/imports.py#L164)

Checks if bf16 is supported, optionally ignoring the TPU

#### accelerate.utils.is_mps_available[[accelerate.utils.is_mps_available]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/imports.py#L334)

Checks if MPS device is available. The minimum version required is 1.12.

#### accelerate.utils.is_npu_available[[accelerate.utils.is_npu_available]]

Checks if `torch_npu` is installed and potentially if a NPU is in the environment

#### accelerate.utils.is_torch_version[[accelerate.utils.is_torch_version]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/versions.py#L46)

Compares the current PyTorch version to a given reference with an operation.

**Parameters:**

operation (`str`) : A string representation of an operator, such as `">"` or `"<="`

version (`str`) : A string version of PyTorch

#### accelerate.utils.is_torch_xla_available[[accelerate.utils.is_torch_xla_available]]

Check if `torch_xla` is available. To train a native pytorch job in an environment with torch xla installed, set
the USE_TORCH_XLA to false.

#### accelerate.utils.is_xpu_available[[accelerate.utils.is_xpu_available]]

Checks if XPU acceleration is available via stock PyTorch (>=2.7) and
potentially if a XPU is in the environment

## Environment Manipulation[[accelerate.utils.patch_environment]]

#### accelerate.utils.patch_environment[[accelerate.utils.patch_environment]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/environment.py#L416)

A context manager that will add each keyword argument passed to `os.environ` and remove them when exiting.

Will convert the values in `kwargs` to strings and upper-case all the keys.

Example:

```python
>>> import os
>>> from accelerate.utils import patch_environment

>>> with patch_environment(FOO="bar"):
...     print(os.environ["FOO"])  # prints "bar"
>>> print(os.environ["FOO"])  # raises KeyError
```

#### accelerate.utils.clear_environment[[accelerate.utils.clear_environment]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/environment.py#L381)

A context manager that will temporarily clear environment variables.

When this context exits, the previous environment variables will be back.

Example:

```python
>>> import os
>>> from accelerate.utils import clear_environment

>>> os.environ["FOO"] = "bar"
>>> with clear_environment():
...     print(os.environ)
...     os.environ["FOO"] = "new_bar"
...     print(os.environ["FOO"])
{}
new_bar

>>> print(os.environ["FOO"])
bar
```

#### accelerate.commands.config.default.write_basic_config[[accelerate.commands.config.default.write_basic_config]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/commands/config/default.py#L37)

Creates and saves a basic cluster config to be used on a local machine with potentially multiple GPUs. Will also
set CPU if it is a CPU-only machine.

**Parameters:**

mixed_precision (`str`, *optional*, defaults to "no") : Mixed Precision to use. Should be one of "no", "fp16", or "bf16"

save_location (`str`, *optional*, defaults to `default_json_config_file`) : Optional custom save location. Should be passed to `--config_file` when using `accelerate launch`. Default location is inside the huggingface cache folder (`~/.cache/huggingface`) but can be overridden by setting the `HF_HOME` environmental variable, followed by `accelerate/default_config.yaml`.

When setting up 🤗 Accelerate for the first time, rather than running `accelerate config` [~utils.write_basic_config] can be used as an alternative for quick configuration.

#### accelerate.utils.set_numa_affinity[[accelerate.utils.set_numa_affinity]]

Assigns the current process to a specific NUMA node. Ideally most efficient when having at least 2 cpus per node.

This result is cached between calls. If you want to override it, please use
`accelerate.utils.environment.override_numa_afifnity`.

**Parameters:**

local_process_index (int) : The index of the current process on the current server.

verbose (bool, *optional*) : Whether to print the new cpu cores assignment for each process. If `ACCELERATE_DEBUG_MODE` is enabled, will default to True.

#### accelerate.utils.environment.override_numa_affinity[[accelerate.utils.environment.override_numa_affinity]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/environment.py#L298)

Overrides whatever NUMA affinity is set for the current process. This is very taxing and requires recalculating the
affinity to set, ideally you should use `utils.environment.set_numa_affinity` instead.

**Parameters:**

local_process_index (int) : The index of the current process on the current server.

verbose (bool, *optional*) : Whether to log out the assignment of each CPU. If `ACCELERATE_DEBUG_MODE` is enabled, will default to True.

#### accelerate.utils.purge_accelerate_environment[[accelerate.utils.purge_accelerate_environment]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/environment.py#L453)

Decorator to clean up accelerate environment variables set by the decorated class or function.

In some circumstances, calling certain classes or functions can result in accelerate env vars being set and not
being cleaned up afterwards. As an example, when calling:

TrainingArguments(fp16=True, ...)

The following env var will be set:

ACCELERATE_MIXED_PRECISION=fp16

This can affect subsequent code, since the env var takes precedence over TrainingArguments(fp16=False). This is
especially relevant for unit testing, where we want to avoid the individual tests to have side effects on one
another. Decorate the unit test function or whole class with this decorator to ensure that after each test, the env
vars are cleaned up. This works for both unittest.TestCase and normal classes (pytest); it also works when
decorating the parent class.

## Memory[[accelerate.find_executable_batch_size]]

#### accelerate.find_executable_batch_size[[accelerate.find_executable_batch_size]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/memory.py#L119)

A basic decorator that will try to execute `function`. If it fails from exceptions related to out-of-memory or
CUDNN, the batch size is multiplied by 0.9 and passed to `function`

`function` must take in a `batch_size` parameter as its first argument.

Example:

```python
>>> from accelerate.utils import find_executable_batch_size

>>> @find_executable_batch_size(starting_batch_size=128)
... def train(batch_size, model, optimizer):
...     ...

>>> train(model, optimizer)
```

**Parameters:**

function (`callable`, *optional*) : A function to wrap

starting_batch_size (`int`, *optional*) : The batch size to try and fit into memory

reduce_batch_size_fn (`callable`, *optional*) : A function to determine the new batch size after an out-of-memory error. If not provided, the batch size is multiplied by 0.9 on each failure. The function takes no arguments and should return the new (reduced) batch size as an `int`.

## Modeling[[accelerate.utils.calculate_maximum_sizes]]

These utilities relate to interacting with PyTorch models

#### accelerate.utils.calculate_maximum_sizes[[accelerate.utils.calculate_maximum_sizes]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/modeling.py#L1067)

Computes the total size of the model and its largest layer

#### accelerate.utils.compute_module_sizes[[accelerate.utils.compute_module_sizes]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/modeling.py#L664)

Compute the size of each submodule of a given model.

#### accelerate.utils.extract_model_from_parallel[[accelerate.utils.extract_model_from_parallel]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/other.py#L248)

Extract a model from its distributed containers.

**Parameters:**

model (`torch.nn.Module`) : The model to extract.

keep_fp32_wrapper (`bool`, *optional*) : Whether to remove mixed precision hooks from the model.

keep_torch_compile (`bool`, *optional*) : Whether to unwrap compiled model.

recursive (`bool`, *optional*, defaults to `False`) : Whether to recursively extract all cases of `module.module` from `model` as well as unwrap child sublayers recursively, not just the top-level distributed containers.

**Returns:**

``torch.nn.Module``

The extracted model.

#### accelerate.utils.get_balanced_memory[[accelerate.utils.get_balanced_memory]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/modeling.py#L931)

Compute a `max_memory` dictionary for [infer_auto_device_map()](/docs/accelerate/v1.14.0/en/package_reference/utilities#accelerate.infer_auto_device_map) that will balance the use of each available GPU.

All computation is done analyzing sizes and dtypes of the model parameters. As a result, the model can be on the
meta device (as it would if initialized within the `init_empty_weights` context manager).

**Parameters:**

model (`torch.nn.Module`) : The model to analyze.

max_memory (`Dict`, *optional*) : A dictionary device identifier to maximum memory. Will default to the maximum memory available if unset. Example: `max_memory={0: "1GB"}`.

no_split_module_classes (`List[str]`, *optional*) : A list of layer class names that should never be split across device (for instance any layer that has a residual connection).

dtype (`str` or `torch.dtype`, *optional*) : If provided, the weights will be converted to that type when loaded.

special_dtypes (`Dict[str, Union[str, torch.device]]`, *optional*) : If provided, special dtypes to consider for some specific weights (will override dtype used as default for all weights).

low_zero (`bool`, *optional*) : Minimizes the number of weights on GPU 0, which is convenient when it's used for other operations (like the Transformers generate function).

#### accelerate.utils.get_max_layer_size[[accelerate.utils.get_max_layer_size]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/modeling.py#L718)

Utility function that will scan a list of named modules and return the maximum size used by one full layer. The
definition of a layer being:
- a module with no direct children (just parameters and buffers)
- a module whose class name is in the list `no_split_module_classes`

**Parameters:**

modules (`List[Tuple[str, torch.nn.Module]]`) : The list of named modules where we want to determine the maximum layer size.

module_sizes (`Dict[str, int]`) : A dictionary mapping each layer name to its size (as generated by `compute_module_sizes`).

no_split_module_classes (`List[str]`) : A list of class names for layers we don't want to be split.

**Returns:**

``Tuple[int, List[str]]``

The maximum size of a layer with the list of layer names realizing that maximum size.

#### accelerate.infer_auto_device_map[[accelerate.infer_auto_device_map]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/modeling.py#L1295)

Compute a device map for a given model giving priority to GPUs, then offload on CPU and finally offload to disk,
such that:
- we don't exceed the memory available of any of the GPU.
- if offload to the CPU is needed, there is always room left on GPU 0 to put back the layer offloaded on CPU that
  has the largest size.
- if offload to the CPU is needed,we don't exceed the RAM available on the CPU.
- if offload to the disk is needed, there is always room left on the CPU to put back the layer offloaded on disk
  that has the largest size.

All computation is done analyzing sizes and dtypes of the model parameters. As a result, the model can be on the
meta device (as it would if initialized within the `init_empty_weights` context manager).

**Parameters:**

model (`torch.nn.Module`) : The model to analyze.

max_memory (`Dict`, *optional*) : A dictionary device identifier to maximum memory. Will default to the maximum memory available if unset. Example: `max_memory={0: "1GB"}`.

no_split_module_classes (`List[str]`, *optional*) : A list of layer class names that should never be split across device (for instance any layer that has a residual connection).

dtype (`str` or `torch.dtype`, *optional*) : If provided, the weights will be converted to that type when loaded.

special_dtypes (`Dict[str, Union[str, torch.device]]`, *optional*) : If provided, special dtypes to consider for some specific weights (will override dtype used as default for all weights).

verbose (`bool`, *optional*, defaults to `False`) : Whether or not to provide debugging statements as the function builds the device_map.

clean_result (`bool`, *optional*, defaults to `True`) : Clean the resulting device_map by grouping all submodules that go on the same device together.

offload_buffers (`bool`, *optional*, defaults to `False`) : In the layers that are offloaded on the CPU or the hard drive, whether or not to offload the buffers as well as the parameters.

fallback_allocation (`bool`, *optional*, defaults to `False`) : When regular allocation fails, try to allocate a module that fits in the size limit using BFS.

#### accelerate.load_checkpoint_in_model[[accelerate.load_checkpoint_in_model]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/modeling.py#L1805)

Loads a (potentially sharded) checkpoint inside a model, potentially sending weights to a given device as they are
loaded.

Once loaded across devices, you still need to call [dispatch_model()](/docs/accelerate/v1.14.0/en/package_reference/big_modeling#accelerate.dispatch_model) on your model to make it able to run. To
group the checkpoint loading and dispatch in one single call, use [load_checkpoint_and_dispatch()](/docs/accelerate/v1.14.0/en/package_reference/big_modeling#accelerate.load_checkpoint_and_dispatch).

**Parameters:**

model (`torch.nn.Module`) : The model in which we want to load a checkpoint.

checkpoint (`str` or `os.PathLike`) : The folder checkpoint to load. It can be: - a path to a file containing a whole model state dict - a path to a `.json` file containing the index to a sharded checkpoint - a path to a folder containing a unique `.index.json` file and the shards of a checkpoint. - a path to a folder containing a unique pytorch_model.bin or a model.safetensors file.

device_map (`Dict[str, Union[int, str, torch.device]]`, *optional*) : A map that specifies where each submodule should go. It doesn't need to be refined to each parameter/buffer name, once a given module name is inside, every submodule of it will be sent to the same device.

offload_folder (`str` or `os.PathLike`, *optional*) : If the `device_map` contains any value `"disk"`, the folder where we will offload weights.

dtype (`str` or `torch.dtype`, *optional*) : If provided, the weights will be converted to that type when loaded.

offload_state_dict (`bool`, *optional*, defaults to `False`) : If `True`, will temporarily offload the CPU state dict on the hard drive to avoid getting out of CPU RAM if the weight of the CPU state dict + the biggest shard does not fit.

offload_buffers (`bool`, *optional*, defaults to `False`) : Whether or not to include the buffers in the weights offloaded to disk.

keep_in_fp32_modules(`List[str]`, *optional*) : A list of the modules that we keep in `torch.float32` dtype.

offload_8bit_bnb (`bool`, *optional*) : Whether or not to enable offload of 8-bit modules on cpu/disk.

strict (`bool`, *optional*, defaults to `False`) : Whether to strictly enforce that the keys in the checkpoint state_dict match the keys of the model's state_dict.

full_state_dict (`bool`, *optional*, defaults to `True`) : if this is set to `True`, all the tensors in the loaded state_dict will be gathered. No ShardedTensor and DTensor will be in the loaded state_dict.

broadcast_from_rank0 (`False`, *optional*, defaults to `False`) : when the option is `True`, a distributed `ProcessGroup` must be initialized. rank0 should receive a full state_dict and will broadcast the tensors in the state_dict one by one to other ranks. Other ranks will receive the tensors and shard (if applicable) according to the local shards in the model.

#### accelerate.utils.load_offloaded_weights[[accelerate.utils.load_offloaded_weights]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/modeling.py#L890)

Loads the weights from the offload folder into the model.

**Parameters:**

model (`torch.nn.Module`) : The model to load the weights into.

index (`dict`) : A dictionary containing the parameter name and its metadata for each parameter that was offloaded from the model.

offload_folder (`str`) : The folder where the offloaded weights are stored.

#### accelerate.utils.load_state_dict[[accelerate.utils.load_state_dict]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/modeling.py#L1637)

Load a checkpoint from a given file. If the checkpoint is in the safetensors format and a device map is passed, the
weights can be fast-loaded directly on the GPU.

**Parameters:**

checkpoint_file (`str`) : The path to the checkpoint to load.

device_map (`Dict[str, Union[int, str, torch.device]]`, *optional*) : A map that specifies where each submodule should go. It doesn't need to be refined to each parameter/buffer name, once a given module name is inside, every submodule of it will be sent to the same device.

#### accelerate.utils.offload_state_dict[[accelerate.utils.offload_state_dict]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/offload.py#L85)

Offload a state dict in a given folder.

**Parameters:**

save_dir (`str` or `os.PathLike`) : The directory in which to offload the state dict.

state_dict (`Dict[str, torch.Tensor]`) : The dictionary of tensors to offload.

#### accelerate.utils.retie_parameters[[accelerate.utils.retie_parameters]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/modeling.py#L622)

Reties tied parameters in a given model if the link was broken (for instance when adding hooks).

**Parameters:**

model (`torch.nn.Module`) : The model in which to retie parameters.

tied_params (`List[List[str]]`) : A mapping parameter name to tied parameter name as obtained by `find_tied_parameters`.

#### accelerate.utils.set_module_tensor_to_device[[accelerate.utils.set_module_tensor_to_device]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/modeling.py#L227)

A helper function to set a given tensor (parameter of buffer) of a module on a specific device (note that doing
`param.to(device)` creates a new tensor not linked to the parameter, which is why we need this function).

**Parameters:**

module (`torch.nn.Module`) : The module in which the tensor we want to move lives.

tensor_name (`str`) : The full name of the parameter/buffer.

device (`int`, `str` or `torch.device`) : The device on which to set the tensor.

value (`torch.Tensor`, *optional*) : The value of the tensor (useful when going from the meta device to any other device).

dtype (`torch.dtype`, *optional*) : If passed along the value of the parameter will be cast to this `dtype`. Otherwise, `value` will be cast to the dtype of the existing parameter in the model.

fp16_statistics (`torch.HalfTensor`, *optional*) : The list of fp16 statistics to set on the module, used for 8 bit model serialization.

tied_params_map (Dict[int, Dict[torch.device, torch.Tensor]], *optional*, defaults to `None`) : A map of current data pointers to dictionaries of devices to already dispatched tied weights. For a given execution device, this parameter is useful to reuse the first available pointer of a shared weight on the device for all others, instead of duplicating memory.

non_blocking (`bool`, *optional*, defaults to `False`) : If `True`, the device transfer will be asynchronous with respect to the host, if possible.

clear_cache (`bool`, *optional*, defaults to `True`) : Whether or not to clear the device cache after setting the tensor on the device.

#### accelerate.utils.get_module_children_bottom_up[[accelerate.utils.get_module_children_bottom_up]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/other.py#L566)

Traverse the model in bottom-up order and return the children modules in that order.

**Parameters:**

model (`torch.nn.Module`) : the model to get the children of

**Returns:**

``list[torch.nn.Module]``

a list of children modules of `model` in bottom-up order. The last element is the
`model` itself.

## Parallel[[accelerate.utils.extract_model_from_parallel]]

These include general utilities that should be used when working in parallel.

#### accelerate.utils.extract_model_from_parallel[[accelerate.utils.extract_model_from_parallel]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/other.py#L248)

Extract a model from its distributed containers.

**Parameters:**

model (`torch.nn.Module`) : The model to extract.

keep_fp32_wrapper (`bool`, *optional*) : Whether to remove mixed precision hooks from the model.

keep_torch_compile (`bool`, *optional*) : Whether to unwrap compiled model.

recursive (`bool`, *optional*, defaults to `False`) : Whether to recursively extract all cases of `module.module` from `model` as well as unwrap child sublayers recursively, not just the top-level distributed containers.

**Returns:**

``torch.nn.Module``

The extracted model.

#### accelerate.utils.save[[accelerate.utils.save]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/other.py#L384)

Save the data to disk. Use in place of `torch.save()`.

**Parameters:**

obj : The data to save

f : The file (or file-like object) to use to save the data

save_on_each_node (`bool`, *optional*, defaults to `False`) : Whether to only save on the global main process

safe_serialization (`bool`, *optional*, defaults to `False`) : Whether to save `obj` using `safetensors` or the traditional PyTorch way (that uses `pickle`).

#### accelerate.utils.load[[accelerate.utils.load]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/other.py#L434)

Compatible drop-in replacement of `torch.load()` which allows for `weights_only` to be used if `torch` version is
2.4.0 or higher. Otherwise will ignore the kwarg.

Will also add (and then remove) an exception for numpy arrays

**Parameters:**

f : The file (or file-like object) to use to load the data

map_location : a function, `torch.device`, string or a dict specifying how to remap storage locations

- ****kwargs** : Additional keyword arguments to pass to `torch.load()`.

#### accelerate.utils.wait_for_everyone[[accelerate.utils.wait_for_everyone]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/other.py#L336)

Introduces a blocking point in the script, making sure all processes have reached this point before continuing.

Make sure all processes will reach this instruction otherwise one of your processes will hang forever.

## Random[[accelerate.utils.set_seed]]

These utilities relate to setting and synchronizing of all the random states.

#### accelerate.utils.set_seed[[accelerate.utils.set_seed]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/random.py#L40)

Helper function for reproducible behavior to set the seed in `random`, `numpy`, `torch`.

**Parameters:**

seed (`int`) : The seed to set.

device_specific (`bool`, *optional*, defaults to `False`) : Whether to differ the seed on each device slightly with `self.process_index`.

deterministic (`bool`, *optional*, defaults to `False`) : Whether to use deterministic algorithms where available. Can slow down training.

#### accelerate.utils.synchronize_rng_state[[accelerate.utils.synchronize_rng_state]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/random.py#L81)

#### accelerate.synchronize_rng_states[[accelerate.synchronize_rng_states]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/random.py#L163)

## PyTorch XLA[[accelerate.utils.install_xla]]

These include utilities that are useful while using PyTorch with XLA.

#### accelerate.utils.install_xla[[accelerate.utils.install_xla]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/torch_xla.py#L20)

Helper function to install appropriate xla wheels based on the `torch` version in Google Colaboratory.

Example:

```python
>>> from accelerate.utils import install_xla

>>> install_xla(upgrade=True)
```

**Parameters:**

upgrade (`bool`, *optional*, defaults to `False`) : Whether to upgrade `torch` and install the latest `torch_xla` wheels.

## Loading model weights[[accelerate.load_checkpoint_in_model]]

These include utilities that are useful to load checkpoints.

#### accelerate.load_checkpoint_in_model[[accelerate.load_checkpoint_in_model]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/modeling.py#L1805)

Loads a (potentially sharded) checkpoint inside a model, potentially sending weights to a given device as they are
loaded.

Once loaded across devices, you still need to call [dispatch_model()](/docs/accelerate/v1.14.0/en/package_reference/big_modeling#accelerate.dispatch_model) on your model to make it able to run. To
group the checkpoint loading and dispatch in one single call, use [load_checkpoint_and_dispatch()](/docs/accelerate/v1.14.0/en/package_reference/big_modeling#accelerate.load_checkpoint_and_dispatch).

**Parameters:**

model (`torch.nn.Module`) : The model in which we want to load a checkpoint.

checkpoint (`str` or `os.PathLike`) : The folder checkpoint to load. It can be: - a path to a file containing a whole model state dict - a path to a `.json` file containing the index to a sharded checkpoint - a path to a folder containing a unique `.index.json` file and the shards of a checkpoint. - a path to a folder containing a unique pytorch_model.bin or a model.safetensors file.

device_map (`Dict[str, Union[int, str, torch.device]]`, *optional*) : A map that specifies where each submodule should go. It doesn't need to be refined to each parameter/buffer name, once a given module name is inside, every submodule of it will be sent to the same device.

offload_folder (`str` or `os.PathLike`, *optional*) : If the `device_map` contains any value `"disk"`, the folder where we will offload weights.

dtype (`str` or `torch.dtype`, *optional*) : If provided, the weights will be converted to that type when loaded.

offload_state_dict (`bool`, *optional*, defaults to `False`) : If `True`, will temporarily offload the CPU state dict on the hard drive to avoid getting out of CPU RAM if the weight of the CPU state dict + the biggest shard does not fit.

offload_buffers (`bool`, *optional*, defaults to `False`) : Whether or not to include the buffers in the weights offloaded to disk.

keep_in_fp32_modules(`List[str]`, *optional*) : A list of the modules that we keep in `torch.float32` dtype.

offload_8bit_bnb (`bool`, *optional*) : Whether or not to enable offload of 8-bit modules on cpu/disk.

strict (`bool`, *optional*, defaults to `False`) : Whether to strictly enforce that the keys in the checkpoint state_dict match the keys of the model's state_dict.

full_state_dict (`bool`, *optional*, defaults to `True`) : if this is set to `True`, all the tensors in the loaded state_dict will be gathered. No ShardedTensor and DTensor will be in the loaded state_dict.

broadcast_from_rank0 (`False`, *optional*, defaults to `False`) : when the option is `True`, a distributed `ProcessGroup` must be initialized. rank0 should receive a full state_dict and will broadcast the tensors in the state_dict one by one to other ranks. Other ranks will receive the tensors and shard (if applicable) according to the local shards in the model.

## Quantization[[accelerate.utils.load_and_quantize_model]]

These include utilities that are useful to quantize model.

#### accelerate.utils.load_and_quantize_model[[accelerate.utils.load_and_quantize_model]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/bnb.py#L44)

This function will quantize the input model with the associated config passed in `bnb_quantization_config`. If the
model is in the meta device, we will load and dispatch the weights according to the `device_map` passed. If the
model is already loaded, we will quantize the model and put the model on the GPU,

**Parameters:**

model (`torch.nn.Module`) : Input model. The model can be already loaded or on the meta device

bnb_quantization_config (`BnbQuantizationConfig`) : The bitsandbytes quantization parameters

weights_location (`str` or `os.PathLike`) : The folder weights_location to load. It can be: - a path to a file containing a whole model state dict - a path to a `.json` file containing the index to a sharded checkpoint - a path to a folder containing a unique `.index.json` file and the shards of a checkpoint. - a path to a folder containing a unique pytorch_model.bin file.

device_map (`Dict[str, Union[int, str, torch.device]]`, *optional*) : A map that specifies where each submodule should go. It doesn't need to be refined to each parameter/buffer name, once a given module name is inside, every submodule of it will be sent to the same device.

no_split_module_classes (`List[str]`, *optional*) : A list of layer class names that should never be split across device (for instance any layer that has a residual connection).

max_memory (`Dict`, *optional*) : A dictionary device identifier to maximum memory. Will default to the maximum memory available if unset.

offload_folder (`str` or `os.PathLike`, *optional*) : If the `device_map` contains any value `"disk"`, the folder where we will offload weights.

offload_state_dict (`bool`, *optional*, defaults to `False`) : If `True`, will temporarily offload the CPU state dict on the hard drive to avoid getting out of CPU RAM if the weight of the CPU state dict + the biggest shard does not fit.

**Returns:**

``torch.nn.Module``

The quantized model
