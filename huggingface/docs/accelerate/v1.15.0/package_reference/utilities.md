# Utility functions and classes

Below are a variety of utility functions that 🤗 Accelerate provides, broken down by use-case. 

## Constants

Constants used throughout 🤗 Accelerate for reference

The following are constants used when utilizing [Accelerator.save_state()](/docs/accelerate/v1.15.0/en/package_reference/accelerator#accelerate.Accelerator.save_state)

`utils.MODEL_NAME`: `"pytorch_model"`
`utils.OPTIMIZER_NAME`: `"optimizer"`
`utils.RNG_STATE_NAME`: `"random_states"`
`utils.SCALER_NAME`: `"scaler.pt`
`utils.SCHEDULER_NAME`: `"scheduler`

The following are constants used when utilizing [Accelerator.save_model()](/docs/accelerate/v1.15.0/en/package_reference/accelerator#accelerate.Accelerator.save_model)

`utils.WEIGHTS_NAME`: `"pytorch_model.bin"`
`utils.SAFE_WEIGHTS_NAME`: `"model.safetensors"`
`utils.WEIGHTS_INDEX_NAME`: `"pytorch_model.bin.index.json"`
`utils.SAFE_WEIGHTS_INDEX_NAME`: `"model.safetensors.index.json"`

## Data Classes

These are basic dataclasses used throughout 🤗 Accelerate and they can be passed in as parameters.

### Standalone[[accelerate.utils.ComputeEnvironment]]

These are standalone dataclasses used for checks, such as the type of distributed system being used

#### accelerate.utils.ComputeEnvironment[[accelerate.utils.ComputeEnvironment]]

```python
accelerate.utils.ComputeEnvironment(value, names = None, module = None, qualname = None, type = None, start = 1)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L675)

Represents a type of the compute environment.

Values:

- **LOCAL_MACHINE** -- private/custom cluster hardware.
- **AMAZON_SAGEMAKER** -- Amazon SageMaker as compute environment.

#### accelerate.DistributedType[[accelerate.DistributedType]]

```python
accelerate.DistributedType(value, names = None, module = None, qualname = None, type = None, start = 1)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L602)

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

```python
accelerate.utils.DynamoBackend(value, names = None, module = None, qualname = None, type = None, start = 1)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L690)

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
- **NEURON** -- Uses AWS Neuron backend for Trainium/Inferentia.

#### accelerate.utils.LoggerType[[accelerate.utils.LoggerType]]

```python
accelerate.utils.LoggerType(value, names = None, module = None, qualname = None, type = None, start = 1)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L745)

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

```python
accelerate.utils.PrecisionType(value, names = None, module = None, qualname = None, type = None, start = 1)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L773)

Represents a type of precision used on floating point values

Values:

- **NO** -- using full precision (FP32)
- **FP16** -- using half precision
- **BF16** -- using brain floating point precision

#### accelerate.utils.RNGType[[accelerate.utils.RNGType]]

```python
accelerate.utils.RNGType(value, names = None, module = None, qualname = None, type = None, start = 1)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L789)

An enumeration.

#### accelerate.utils.SageMakerDistributedType[[accelerate.utils.SageMakerDistributedType]]

```python
accelerate.utils.SageMakerDistributedType(value, names = None, module = None, qualname = None, type = None, start = 1)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L641)

Represents a type of distributed environment.

Values:

- **NO** -- Not a distributed environment, just a single process.
- **DATA_PARALLEL** -- using sagemaker distributed data parallelism.
- **MODEL_PARALLEL** -- using sagemaker distributed model parallelism.

### Kwargs[[accelerate.AutocastKwargs]]

These are configurable arguments for specific interactions throughout the PyTorch ecosystem that Accelerate handles under the hood.

#### accelerate.AutocastKwargs[[accelerate.AutocastKwargs]]

```python
accelerate.AutocastKwargs(enabled: bool = True, cache_enabled: typing.Optional[bool] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L115)

Use this object in your [Accelerator](/docs/accelerate/v1.15.0/en/package_reference/accelerator#accelerate.Accelerator) to customize how `torch.autocast` behaves. Please refer to the
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

```python
accelerate.DistributedDataParallelKwargs(dim: int = 0, broadcast_buffers: bool = True, bucket_cap_mb: int = 25, find_unused_parameters: bool = False, check_reduction: bool = False, gradient_as_bucket_view: bool = False, static_graph: bool = False, comm_hook: DDPCommunicationHookType = <DDPCommunicationHookType.NO: 'no'>, comm_wrapper: typing.Literal[<DDPCommunicationHookType.NO: 'no'>, <DDPCommunicationHookType.FP16: 'fp16'>, <DDPCommunicationHookType.BF16: 'bf16'>] = <DDPCommunicationHookType.NO: 'no'>, comm_state_option: dict = <factory>)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L157)

Use this object in your [Accelerator](/docs/accelerate/v1.15.0/en/package_reference/accelerator#accelerate.Accelerator) to customize how your model is wrapped in a
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

```python
accelerate.utils.FP8RecipeKwargs(opt_level: typing.Literal['O1', 'O2'] = None, use_autocast_during_eval: typing.Optional[bool] = None, margin: typing.Optional[int] = None, interval: typing.Optional[int] = None, fp8_format: typing.Literal['HYBRID', 'E4M3', 'E5M2'] = None, amax_history_len: typing.Optional[int] = None, amax_compute_algo: typing.Literal['max', 'most_recent'] = None, override_linear_precision: tuple = None, use_mxfp8_block_scaling: typing.Optional[bool] = None, backend: typing.Literal['MSAMP', 'TE'] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L457)

Deprecated. Please use one of the proper FP8 recipe kwargs classes such as `TERecipeKwargs` or `MSAMPRecipeKwargs`
instead.

#### accelerate.GradScalerKwargs[[accelerate.GradScalerKwargs]]

```python
accelerate.GradScalerKwargs(init_scale: float = 65536.0, growth_factor: float = 2.0, backoff_factor: float = 0.5, growth_interval: int = 2000, enabled: bool = True)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L243)

Use this object in your [Accelerator](/docs/accelerate/v1.15.0/en/package_reference/accelerator#accelerate.Accelerator) to customize the behavior of mixed precision, specifically how the
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

```python
accelerate.InitProcessGroupKwargs(backend: typing.Optional[str] = 'nccl', init_method: typing.Optional[str] = None, timeout: typing.Optional[datetime.timedelta] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L275)

Use this object in your [Accelerator](/docs/accelerate/v1.15.0/en/package_reference/accelerator#accelerate.Accelerator) to customize the initialization of the distributed processes. Please refer
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

```python
accelerate.utils.KwargsHandler()
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L70)

Internal mixin that implements a `to_kwargs()` method for a dataclass.

#### to_kwargs[[accelerate.utils.KwargsHandler.to_kwargs]]

```python
to_kwargs()
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L78)

Returns a dictionary containing the attributes with values different from the default of this class.

## Plugins[[accelerate.DeepSpeedPlugin]]

These are plugins that can be passed to the [Accelerator](/docs/accelerate/v1.15.0/en/package_reference/accelerator#accelerate.Accelerator) object. While they are defined elsewhere in the documentation, 
for convenience all of them are available to see here:

#### accelerate.DeepSpeedPlugin[[accelerate.DeepSpeedPlugin]]

```python
accelerate.DeepSpeedPlugin(hf_ds_config: typing.Any = None, gradient_accumulation_steps: int = None, gradient_clipping: float = None, zero_stage: int = None, is_train_batch_min: bool = True, offload_optimizer_device: str = None, offload_param_device: str = None, offload_optimizer_nvme_path: str = None, offload_param_nvme_path: str = None, zero3_init_flag: bool = None, zero3_save_16bit_model: bool = None, transformer_moe_cls_names: str = None, enable_msamp: bool = None, msamp_opt_level: typing.Optional[typing.Literal['O1', 'O2']] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L1122)

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

msamp_opt_level (`Optional[Literal["O1", "O2"]]`, defaults to `None`) : Optimization level for MS-AMP (defaults to 'O1'). Only applicable if `enable_msamp` is True. Should be one of ['O1' or 'O2'].

This plugin is used to integrate DeepSpeed.

#### deepspeed_config_process[[accelerate.DeepSpeedPlugin.deepspeed_config_process]]

```python
deepspeed_config_process(prefix = '', mismatches = None, config = None, must_match = True, **kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L1392)

Process the DeepSpeed config with the values from the kwargs.

#### select[[accelerate.DeepSpeedPlugin.select]]

```python
select(_from_accelerator_state: bool = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L1554)

Sets the HfDeepSpeedWeakref to use the current deepspeed plugin configuration

#### accelerate.FullyShardedDataParallelPlugin[[accelerate.FullyShardedDataParallelPlugin]]

```python
accelerate.FullyShardedDataParallelPlugin(fsdp_version: int = None, sharding_strategy: typing.Union[str, ForwardRef('torch.distributed.fsdp.ShardingStrategy')] = None, reshard_after_forward: typing.Union[str, ForwardRef('torch.distributed.fsdp.ShardingStrategy'), bool] = None, backward_prefetch: typing.Union[str, ForwardRef('torch.distributed.fsdp.BackwardPrefetch'), NoneType] = None, mixed_precision_policy: typing.Union[dict, str, ForwardRef('torch.distributed.fsdp.MixedPrecision'), ForwardRef('torch.distributed.fsdp.MixedPrecisionPolicy'), NoneType] = None, auto_wrap_policy: typing.Union[typing.Callable, typing.Literal['transformer_based_wrap', 'size_based_wrap', 'no_wrap'], NoneType] = None, cpu_offload: typing.Union[bool, ForwardRef('torch.distributed.fsdp.CPUOffload'), ForwardRef('torch.distributed.fsdp.CPUOffloadPolicy')] = None, ignored_modules: typing.Union[collections.abc.Iterable[torch.nn.Module], str, NoneType] = None, state_dict_type: typing.Union[str, ForwardRef('torch.distributed.fsdp.StateDictType')] = None, state_dict_config: typing.Union[ForwardRef('torch.distributed.fsdp.FullStateDictConfig'), ForwardRef('torch.distributed.fsdp.ShardedStateDictConfig'), NoneType] = None, optim_state_dict_config: typing.Union[ForwardRef('torch.distributed.fsdp.FullOptimStateDictConfig'), ForwardRef('torch.distributed.fsdp.ShardedOptimStateDictConfig'), NoneType] = None, limit_all_gathers: bool = True, use_orig_params: typing.Optional[bool] = None, param_init_fn: typing.Optional[typing.Callable[[torch.nn.Module], NoneType]] = None, sync_module_states: typing.Optional[bool] = None, forward_prefetch: bool = None, activation_checkpointing: bool = None, activation_checkpointing_offload: bool = None, cpu_ram_efficient_loading: bool = None, transformer_cls_names_to_wrap: typing.Optional[list[str]] = None, min_num_params: typing.Optional[int] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L1586)

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

This plugin is used to enable fully sharded data parallelism.

#### set_auto_wrap_policy[[accelerate.FullyShardedDataParallelPlugin.set_auto_wrap_policy]]

```python
set_auto_wrap_policy(model)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L2075)

Given `model`, creates an `auto_wrap_policy` based on the passed in policy and if we can use the
`transformer_cls_to_wrap`

#### set_mixed_precision[[accelerate.FullyShardedDataParallelPlugin.set_mixed_precision]]

```python
set_mixed_precision(mixed_precision, buffer_autocast = False, override = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L2109)

Sets the mixed precision policy for FSDP

#### set_state_dict_type[[accelerate.FullyShardedDataParallelPlugin.set_state_dict_type]]

```python
set_state_dict_type(state_dict_type = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L2030)

Set the state dict config based on the `StateDictType`.

#### validate_mixed_precision_policy[[accelerate.FullyShardedDataParallelPlugin.validate_mixed_precision_policy]]

```python
validate_mixed_precision_policy()
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L2161)

Validates the mixed precision policy, abstracted away to not bring in the imports if not needed.

#### accelerate.utils.GradientAccumulationPlugin[[accelerate.utils.GradientAccumulationPlugin]]

```python
accelerate.utils.GradientAccumulationPlugin(num_steps: int = None, adjust_scheduler: bool = True, sync_with_dataloader: bool = True, sync_each_batch: bool = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L981)

**Parameters:**

num_steps (`int`) : The number of steps to accumulate gradients for.

adjust_scheduler (`bool`, *optional*, defaults to `True`) : Whether to adjust the scheduler steps to account for the number of steps being accumulated. Should be `True` if the used scheduler was not adjusted for gradient accumulation.

sync_with_dataloader (`bool`, *optional*, defaults to `True`) : Whether to synchronize setting the gradients when at the end of the dataloader.

sync_each_batch (`bool`, *optional*) : Whether to synchronize setting the gradients at each data batch. Setting to `True` may reduce memory requirements when using gradient accumulation with distributed training, at expense of speed.

A plugin to configure gradient accumulation behavior. You can only pass one of `gradient_accumulation_plugin` or
`gradient_accumulation_steps` to [Accelerator](/docs/accelerate/v1.15.0/en/package_reference/accelerator#accelerate.Accelerator). Passing both raises an error.

Example:

```python
from accelerate.utils import GradientAccumulationPlugin

gradient_accumulation_plugin = GradientAccumulationPlugin(num_steps=2)
accelerator = Accelerator(gradient_accumulation_plugin=gradient_accumulation_plugin)
```

#### accelerate.utils.MegatronLMPlugin[[accelerate.utils.MegatronLMPlugin]]

```python
accelerate.utils.MegatronLMPlugin(tp_degree: int = None, pp_degree: int = None, use_custom_fsdp: bool = None, overlap_cpu_optimizer_d2h_h2d: bool = None, no_load_optim: bool = None, eod_mask_loss: bool = None, no_save_optim: bool = None, optimizer_cpu_offload: bool = None, use_precision_aware_optimizer: bool = None, decoder_last_pipeline_num_layers: int = None, recompute_granularity: str = None, recompute_method: str = None, recompute_num_layers: int = None, attention_backend: bool = None, expert_model_parallel_size: int = None, context_parallel_size: int = None, attention_dropout: float = None, hidden_dropout: float = None, attention_softmax_in_fp32: bool = None, expert_tensor_parallel_size: int = None, calculate_per_token_loss: bool = None, use_rotary_position_embeddings: bool = None, num_micro_batches: int = None, gradient_clipping: float = None, sequence_parallelism: bool = None, recompute_activations: bool = None, use_distributed_optimizer: bool = None, pipeline_model_parallel_split_rank: int = None, num_layers_per_virtual_pipeline_stage: int = None, is_train_batch_min: str = True, train_iters: int = None, train_samples: int = None, weight_decay_incr_style: str = 'constant', start_weight_decay: float = None, end_weight_decay: float = None, lr_decay_style: str = 'linear', lr_decay_iters: int = None, lr_decay_samples: int = None, lr_warmup_iters: int = None, lr_warmup_samples: int = None, lr_warmup_fraction: float = None, min_lr: float = 0, consumed_samples: list = None, no_wd_decay_cond: typing.Optional[typing.Callable] = None, scale_lr_cond: typing.Optional[typing.Callable] = None, lr_mult: float = 1.0, megatron_dataset_flag: bool = False, seq_length: int = None, encoder_seq_length: int = None, decoder_seq_length: int = None, tensorboard_dir: str = None, set_all_logging_options: bool = False, eval_iters: int = 100, eval_interval: int = 1000, return_logits: bool = False, custom_train_step_class: typing.Optional[typing.Any] = None, custom_train_step_kwargs: typing.Optional[dict[str, typing.Any]] = None, custom_model_provider_function: typing.Optional[typing.Callable] = None, custom_prepare_model_function: typing.Optional[typing.Callable] = None, custom_megatron_datasets_provider_function: typing.Optional[typing.Callable] = None, custom_get_batch_function: typing.Optional[typing.Callable] = None, custom_loss_function: typing.Optional[typing.Callable] = None, other_megatron_args: typing.Optional[dict[str, typing.Any]] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L2335)

**Parameters:**

tp_degree (`int`, defaults to `None`) : Tensor parallelism degree.

pp_degree (`int`, defaults to `None`) : Pipeline parallelism degree.

num_micro_batches (`int`, defaults to `None`) : Number of micro-batches.

gradient_clipping (`float`, defaults to `None`) : Gradient clipping value based on global L2 Norm (0 to disable).

sequence_parallelism (`bool`, defaults to `None`) : Enable sequence parallelism.

recompute_activations (`bool`, defaults to `None`) : Enable selective activation recomputation.

use_distributed_optimizer (`bool`, defaults to `None`) : Enable distributed optimizer.

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

Plugin for Megatron-LM to enable tensor, pipeline, sequence and data parallelism. Also to enable selective
activation recomputation and optimized fused kernels.

#### accelerate.utils.TorchDynamoPlugin[[accelerate.utils.TorchDynamoPlugin]]

```python
accelerate.utils.TorchDynamoPlugin(backend: DynamoBackend = None, mode: str = None, fullgraph: bool = None, dynamic: bool = None, options: typing.Any = None, disable: bool = False, use_regional_compilation: bool = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L1033)

**Parameters:**

backend (`DynamoBackend`, defaults to `None`) : A valid Dynamo backend. See https://pytorch.org/docs/stable/torch.compiler.html for more details.

mode (`str`, defaults to `None`) : Possible options are 'default', 'reduce-overhead' or 'max-autotune'.

fullgraph (`bool`, defaults to `None`) : Whether it is ok to break model into several subgraphs.

dynamic (`bool`, defaults to `None`) : Whether to use dynamic shape for tracing.

options (`Any`, defaults to `None`) : A dictionary of options to pass to the backend.

disable (`bool`, defaults to `False`) : Turn torch.compile() into a no-op for testing

use_regional_compilation (`bool`, defaults to `None`) : Use it to reduce the cold start compilation time of torch.compile() by targeting repeated blocks of the same class and compiling them sequentially to hit the compiler's cache. For example, in `GPT2LMHeadModel`, the repeated block/class is `GPT2Block`, and can be accessed as `model.transformer.h[0]`. The rest of the model (e.g model.lm_head) is compiled separately.

This plugin is used to compile a model with PyTorch 2.0

## Configurations[[accelerate.utils.BnbQuantizationConfig]]

These are classes which can be configured and passed through to the appropriate integration

#### accelerate.utils.BnbQuantizationConfig[[accelerate.utils.BnbQuantizationConfig]]

```python
accelerate.utils.BnbQuantizationConfig(load_in_8bit: bool = False, llm_int8_threshold: float = 6.0, load_in_4bit: bool = False, bnb_4bit_quant_type: str = 'fp4', bnb_4bit_use_double_quant: bool = False, bnb_4bit_compute_dtype: str = 'fp16', torch_dtype: dtype = None, skip_modules: list = None, keep_in_fp32_modules: list = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L3074)

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

A plugin to enable BitsAndBytes 4bit and 8bit quantization

#### accelerate.DataLoaderConfiguration[[accelerate.DataLoaderConfiguration]]

```python
accelerate.DataLoaderConfiguration(split_batches: bool = False, dispatch_batches: bool = None, even_batches: bool = True, use_seedable_sampler: bool = False, data_seed: int = None, non_blocking: bool = False, use_stateful_dataloader: bool = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L823)

**Parameters:**

split_batches (`bool`, defaults to `False`) : Whether or not the accelerator should split the batches yielded by the dataloaders across the devices. If `True`, the actual batch size used will be the same on any kind of distributed processes, but it must be a round multiple of `num_processes` you are using. If `False`, actual batch size used will be the one set in your script multiplied by the number of processes.

dispatch_batches (`bool`, defaults to `None`) : If set to `True`, the dataloader prepared by the Accelerator is only iterated through on the main process and then the batches are split and broadcast to each process. Will default to `True` for `DataLoader` whose underlying dataset is an `IterableDataset`, `False` otherwise.

even_batches (`bool`, defaults to `True`) : If set to `True`, in cases where the total batch size across all processes does not exactly divide the dataset, samples at the start of the dataset will be duplicated so the batch can be divided equally among all workers.

use_seedable_sampler (`bool`, defaults to `False`) : Whether or not use a fully seedable random sampler (`data_loader.SeedableRandomSampler`). Ensures training results are fully reproducible using a different sampling technique. While seed-to-seed results may differ, on average the differences are negligible when using multiple different seeds to compare. Should also be ran with [set_seed()](/docs/accelerate/v1.15.0/en/package_reference/utilities#accelerate.utils.set_seed) for the best results.

data_seed (`int`, defaults to `None`) : The seed to use for the underlying generator when using `use_seedable_sampler`. If `None`, the generator will use the current default seed from torch.

non_blocking (`bool`, defaults to `False`) : If set to `True`, the dataloader prepared by the Accelerator will utilize non-blocking host-to-device transfers, allowing for better overlap between dataloader communication and computation. Recommended that the prepared dataloader has `pin_memory` set to `True` to work properly.

use_stateful_dataloader (`bool`, defaults to `False`) : If set to `True`, the dataloader prepared by the Accelerator will be backed by [torchdata.StatefulDataLoader](https://github.com/pytorch/data/tree/main/torchdata/stateful_dataloader). This requires `torchdata` version 0.8.0 or higher that supports StatefulDataLoader to be installed.

Configuration for dataloader-related items when calling `accelerator.prepare`.

#### accelerate.utils.ProjectConfiguration[[accelerate.utils.ProjectConfiguration]]

```python
accelerate.utils.ProjectConfiguration(project_dir: str = None, logging_dir: str = None, automatic_checkpoint_naming: bool = False, total_limit: int = None, iteration: int = 0, save_on_each_node: bool = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L918)

**Parameters:**

project_dir (`str`, defaults to `None`) : A path to a directory for storing data.

logging_dir (`str`, defaults to `None`) : A path to a directory for storing logs of locally-compatible loggers. If None, defaults to `project_dir`.

automatic_checkpoint_naming (`bool`, defaults to `False`) : Whether saved states should be automatically iteratively named.

total_limit (`int`, defaults to `None`) : The maximum number of total saved states to keep.

iteration (`int`, defaults to `0`) : The current save iteration.

save_on_each_node (`bool`, defaults to `False`) : When doing multi-node distributed training, whether to save models and checkpoints on each node, or only on the main one.

Configuration for the Accelerator object based on inner-project needs.

#### set_directories[[accelerate.utils.ProjectConfiguration.set_directories]]

```python
set_directories(project_dir: typing.Optional[str] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L970)

Sets `self.project_dir` and `self.logging_dir` to the appropriate values.

## Environmental Variables

These are environmental variables that can be enabled for different use cases

* `ACCELERATE_DEBUG_MODE` (`str`): Whether to run accelerate in debug mode. More info available [here](../basic_tutorials/troubleshooting#hanging-code-and-timeout-errors).

## Data Manipulation and Operations[[accelerate.utils.broadcast]]

These include data operations that mimic the same `torch` ops but can be used on distributed processes.

#### accelerate.utils.broadcast[[accelerate.utils.broadcast]]

```python
accelerate.utils.broadcast(tensor, from_process: int = 0)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/operations.py#L600)

**Parameters:**

tensor (nested list/tuple/dictionary of `torch.Tensor`) : The data to gather.

from_process (`int`, *optional*, defaults to 0) : The process from which to send the data

**Returns:**

The same data structure as `tensor` with all tensors broadcasted to the proper device.

Recursively broadcast tensor in a nested list/tuple/dictionary of tensors to all devices.

#### accelerate.utils.broadcast_object_list[[accelerate.utils.broadcast_object_list]]

```python
accelerate.utils.broadcast_object_list(object_list, from_process: int = 0)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/operations.py#L675)

**Parameters:**

object_list (list of picklable objects) : The list of objects to broadcast. This list will be modified inplace.

from_process (`int`, *optional*, defaults to 0) : The process from which to send the data.

**Returns:**

The same list containing the objects from process 0.

Broadcast a list of picklable objects from one process to the others.

#### accelerate.utils.concatenate[[accelerate.utils.concatenate]]

```python
accelerate.utils.concatenate(data, dim = 0)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/operations.py#L719)

**Parameters:**

data (nested list/tuple/dictionary of lists of tensors `torch.Tensor`) : The data to concatenate.

dim (`int`, *optional*, defaults to 0) : The dimension on which to concatenate.

**Returns:**

The same data structure as `data` with all the tensors concatenated.

Recursively concatenate the tensors in a nested list/tuple/dictionary of lists of tensors with the same shape.
If there is only a single batch of data, it is returned as-is.

#### accelerate.utils.convert_outputs_to_fp32[[accelerate.utils.convert_outputs_to_fp32]]

```python
accelerate.utils.convert_outputs_to_fp32(model_forward)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/operations.py#L939)

#### accelerate.utils.convert_to_fp32[[accelerate.utils.convert_to_fp32]]

```python
accelerate.utils.convert_to_fp32(tensor)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/operations.py#L889)

**Parameters:**

tensor (nested list/tuple/dictionary of `torch.Tensor`) : The data to convert from FP16/BF16 to FP32.

**Returns:**

The same data structure as `tensor` with all tensors that were in FP16/BF16 precision converted to FP32.

Recursively converts the elements nested list/tuple/dictionary of tensors in FP16/BF16 precision to FP32.

#### accelerate.utils.gather[[accelerate.utils.gather]]

```python
accelerate.utils.gather(tensor)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/operations.py#L424)

**Parameters:**

tensor (nested list/tuple/dictionary of `torch.Tensor`) : The data to gather.

**Returns:**

The same data structure as `tensor` with all tensors sent to the proper device.

Recursively gather tensor in a nested list/tuple/dictionary of tensors from all devices.

#### accelerate.utils.gather_object[[accelerate.utils.gather_object]]

```python
accelerate.utils.gather_object(object: typing.Any)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/operations.py#L505)

**Parameters:**

object (nested list/tuple/dictionary of picklable object) : The data to gather.

**Returns:**

The same data structure as `object` with all the objects sent to every device.

Recursively gather object in a nested list/tuple/dictionary of objects from all devices.

#### accelerate.utils.get_grad_scaler[[accelerate.utils.get_grad_scaler]]

```python
accelerate.utils.get_grad_scaler(distributed_type: DistributedType = None, **kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/modeling.py#L2120)

**Parameters:**

distributed_type (`DistributedType`, *optional*, defaults to None) : The type of distributed environment.

kwargs : Additional arguments for the utilized `GradScaler` constructor.

A generic helper which will initialize the correct `GradScaler` implementation based on the environment and return
it.

#### accelerate.utils.get_mixed_precision_context_manager[[accelerate.utils.get_mixed_precision_context_manager]]

```python
accelerate.utils.get_mixed_precision_context_manager(native_amp: bool = False, autocast_kwargs: AutocastKwargs = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/modeling.py#L2078)

**Parameters:**

native_amp (`bool`, *optional*, defaults to False) : Whether mixed precision is actually enabled.

Return a context manager for autocasting mixed precision

#### accelerate.utils.listify[[accelerate.utils.listify]]

```python
accelerate.utils.listify(data)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/operations.py#L284)

**Parameters:**

data (nested list/tuple/dictionary of `torch.Tensor`) : The data from which to convert to regular numbers.

**Returns:**

The same data structure as `data` with lists of numbers instead of `torch.Tensor`.

Recursively finds tensors in a nested list/tuple/dictionary and converts them to a list of numbers.

#### accelerate.utils.pad_across_processes[[accelerate.utils.pad_across_processes]]

```python
accelerate.utils.pad_across_processes(tensor, dim = 0, pad_index = 0, pad_first = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/operations.py#L749)

**Parameters:**

tensor (nested list/tuple/dictionary of `torch.Tensor`) : The data to gather.

dim (`int`, *optional*, defaults to 0) : The dimension on which to pad.

pad_index (`int`, *optional*, defaults to 0) : The value with which to pad.

pad_first (`bool`, *optional*, defaults to `False`) : Whether to pad at the beginning or the end.

Recursively pad the tensors in a nested list/tuple/dictionary of tensors from all devices to the same size so they
can safely be gathered.

#### accelerate.utils.recursively_apply[[accelerate.utils.recursively_apply]]

```python
accelerate.utils.recursively_apply(func, data, *args, test_type = <function is_torch_tensor at 0x7f64df3f9e10>, error_on_other_type = False, **kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/operations.py#L85)

**Parameters:**

func (`callable`) : The function to recursively apply.

data (nested list/tuple/dictionary of objects accepted by `test_type`) : The data on which to apply `func`

- ***args** : Positional arguments that will be passed to `func` when applied on the unpacked data.

test_type (`Callable[[Any], bool]`, *optional*, defaults to `is_torch_tensor`) : The predicate deciding whether `func` should be applied to a given leaf object.

error_on_other_type (`bool`, *optional*, defaults to `False`) : Whether to return an error or not if after unpacking `data`, we get on an object for which `test_type` returns `False`. If `False`, the function will leave such objects unchanged.

- ****kwargs** (additional keyword arguments, *optional*) : Keyword arguments that will be passed to `func` when applied on the unpacked data.

**Returns:**

The same data structure as `data` with `func` applied to every object for which `test_type` returns `True`.

Recursively apply a function on a data structure that is a nested list/tuple/dictionary of a given base type.

#### accelerate.utils.reduce[[accelerate.utils.reduce]]

```python
accelerate.utils.reduce(tensor, reduction = 'mean', scale = 1.0)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/operations.py#L845)

**Parameters:**

tensor (nested list/tuple/dictionary of `torch.Tensor`) : The data to reduce.

reduction (`str`, *optional*, defaults to `"mean"`) : A reduction method. Can be of "mean", "sum", "max", or "none"

scale (`float`, *optional*) : A default scaling value to be applied after the reduce, only valid on XLA.

**Returns:**

The same data structure as `data` with all the tensors reduced.

Recursively reduce the tensors in a nested list/tuple/dictionary of lists of tensors across all processes by the
mean of a given operation.

#### accelerate.utils.send_to_device[[accelerate.utils.send_to_device]]

```python
accelerate.utils.send_to_device(tensor, device, non_blocking = False, skip_keys = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/operations.py#L136)

**Parameters:**

tensor (nested list/tuple/dictionary of `torch.Tensor`) : The data to send to a given device.

device (`torch.device`) : The device to send the data to.

non_blocking (`bool`, *optional*, defaults to `False`) : If `True`, the transfer to the device is performed asynchronously, which can overlap data movement with computation. Only effective when the device supports it (e.g. CUDA).

skip_keys (`str` or `List[str]`, *optional*) : A key or list of keys in a dictionary `tensor` whose values should not be sent to the given `device`. Entries with these keys are left on their original device.

**Returns:**

The same data structure as `tensor` with all tensors sent to the proper device.

Recursively sends the elements in a nested list/tuple/dictionary of tensors to a given device.

#### accelerate.utils.slice_tensors[[accelerate.utils.slice_tensors]]

```python
accelerate.utils.slice_tensors(data, tensor_slice, process_index = None, num_processes = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/operations.py#L699)

**Parameters:**

data (nested list/tuple/dictionary of `torch.Tensor`) : The data to slice.

tensor_slice (`slice`) : The slice to take.

**Returns:**

The same data structure as `data` with all the tensors slices.

Recursively takes a slice in a nested list/tuple/dictionary of tensors.

## Environment Checks[[accelerate.utils.is_bf16_available]]

These functionalities check the state of the current working environment including information about the operating system itself, what it can support, and if particular dependencies are installed. 

#### accelerate.utils.is_bf16_available[[accelerate.utils.is_bf16_available]]

```python
accelerate.utils.is_bf16_available(ignore_tpu = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/imports.py#L164)

Checks if bf16 is supported, optionally ignoring the TPU

#### accelerate.utils.is_mps_available[[accelerate.utils.is_mps_available]]

```python
accelerate.utils.is_mps_available(min_version = '1.12')
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/imports.py#L334)

Checks if MPS device is available. The minimum version required is 1.12.

#### accelerate.utils.is_npu_available[[accelerate.utils.is_npu_available]]

```python
accelerate.utils.is_npu_available(check_device = False)
```

Checks if `torch_npu` is installed and potentially if a NPU is in the environment

#### accelerate.utils.is_torch_version[[accelerate.utils.is_torch_version]]

```python
accelerate.utils.is_torch_version(operation: str, version: str)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/versions.py#L46)

**Parameters:**

operation (`str`) : A string representation of an operator, such as `">"` or `"<="`

version (`str`) : A string version of PyTorch

Compares the current PyTorch version to a given reference with an operation.

#### accelerate.utils.is_torch_xla_available[[accelerate.utils.is_torch_xla_available]]

```python
accelerate.utils.is_torch_xla_available(check_is_tpu = False, check_is_gpu = False)
```

Check if `torch_xla` is available. To train a native pytorch job in an environment with torch xla installed, set
the USE_TORCH_XLA to false.

#### accelerate.utils.is_xpu_available[[accelerate.utils.is_xpu_available]]

```python
accelerate.utils.is_xpu_available(check_device = False)
```

Checks if XPU acceleration is available via stock PyTorch (>=2.7) and
potentially if a XPU is in the environment

## Environment Manipulation[[accelerate.utils.patch_environment]]

#### accelerate.utils.patch_environment[[accelerate.utils.patch_environment]]

```python
accelerate.utils.patch_environment(**kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/environment.py#L416)

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

```python
accelerate.utils.clear_environment()
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/environment.py#L381)

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

```python
accelerate.commands.config.default.write_basic_config(mixed_precision = 'no', save_location: str = '/home/runner/.cache/huggingface/accelerate/default_config.yaml')
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/commands/config/default.py#L37)

**Parameters:**

mixed_precision (`str`, *optional*, defaults to "no") : Mixed Precision to use. Should be one of "no", "fp16", or "bf16"

save_location (`str`, *optional*, defaults to `default_json_config_file`) : Optional custom save location. Should be passed to `--config_file` when using `accelerate launch`. Default location is inside the huggingface cache folder (`~/.cache/huggingface`) but can be overridden by setting the `HF_HOME` environmental variable, followed by `accelerate/default_config.yaml`.

Creates and saves a basic cluster config to be used on a local machine with potentially multiple GPUs. Will also
set CPU if it is a CPU-only machine.

When setting up 🤗 Accelerate for the first time, rather than running `accelerate config` [~utils.write_basic_config] can be used as an alternative for quick configuration.

#### accelerate.utils.set_numa_affinity[[accelerate.utils.set_numa_affinity]]

```python
accelerate.utils.set_numa_affinity(local_process_index: int, verbose: typing.Optional[bool] = None)
```

**Parameters:**

local_process_index (int) : The index of the current process on the current server.

verbose (bool, *optional*) : Whether to print the new cpu cores assignment for each process. If `ACCELERATE_DEBUG_MODE` is enabled, will default to True.

Assigns the current process to a specific NUMA node. Ideally most efficient when having at least 2 cpus per node.

This result is cached between calls. If you want to override it, please use
`accelerate.utils.environment.override_numa_afifnity`.

#### accelerate.utils.environment.override_numa_affinity[[accelerate.utils.environment.override_numa_affinity]]

```python
accelerate.utils.environment.override_numa_affinity(local_process_index: int, verbose: typing.Optional[bool] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/environment.py#L298)

**Parameters:**

local_process_index (int) : The index of the current process on the current server.

verbose (bool, *optional*) : Whether to log out the assignment of each CPU. If `ACCELERATE_DEBUG_MODE` is enabled, will default to True.

Overrides whatever NUMA affinity is set for the current process. This is very taxing and requires recalculating the
affinity to set, ideally you should use `utils.environment.set_numa_affinity` instead.

#### accelerate.utils.purge_accelerate_environment[[accelerate.utils.purge_accelerate_environment]]

```python
accelerate.utils.purge_accelerate_environment(func_or_cls)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/environment.py#L453)

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

```python
accelerate.find_executable_batch_size(function: typing.Optional[<built-in function callable>] = None, starting_batch_size: int = 128, reduce_batch_size_fn: typing.Optional[<built-in function callable>] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/memory.py#L120)

**Parameters:**

function (`callable`, *optional*) : A function to wrap

starting_batch_size (`int`, *optional*) : The batch size to try and fit into memory

reduce_batch_size_fn (`callable`, *optional*) : A function to determine the new batch size after an out-of-memory error. If not provided, the batch size is multiplied by 0.9 on each failure. The function takes no arguments and should return the new (reduced) batch size as an `int`.

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

## Modeling[[accelerate.utils.calculate_maximum_sizes]]

These utilities relate to interacting with PyTorch models

#### accelerate.utils.calculate_maximum_sizes[[accelerate.utils.calculate_maximum_sizes]]

```python
accelerate.utils.calculate_maximum_sizes(model: Module)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/modeling.py#L1079)

Computes the total size of the model and its largest layer

#### accelerate.utils.compute_module_sizes[[accelerate.utils.compute_module_sizes]]

```python
accelerate.utils.compute_module_sizes(model: Module, dtype: typing.Union[str, torch.device, NoneType] = None, special_dtypes: typing.Optional[dict[str, typing.Union[str, torch.device]]] = None, buffers_only: bool = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/modeling.py#L664)

Compute the size of each submodule of a given model.

#### accelerate.utils.extract_model_from_parallel[[accelerate.utils.extract_model_from_parallel]]

```python
accelerate.utils.extract_model_from_parallel(model, keep_fp32_wrapper: bool = True, keep_torch_compile: bool = True, recursive: bool = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/other.py#L274)

**Parameters:**

model (`torch.nn.Module`) : The model to extract.

keep_fp32_wrapper (`bool`, *optional*) : Whether to remove mixed precision hooks from the model.

keep_torch_compile (`bool`, *optional*) : Whether to unwrap compiled model.

recursive (`bool`, *optional*, defaults to `False`) : Whether to recursively extract all cases of `module.module` from `model` as well as unwrap child sublayers recursively, not just the top-level distributed containers.

**Returns:** `torch.nn.Module`

The extracted model.

Extract a model from its distributed containers.

#### accelerate.utils.get_balanced_memory[[accelerate.utils.get_balanced_memory]]

```python
accelerate.utils.get_balanced_memory(model: Module, max_memory: typing.Optional[dict[typing.Union[int, str], typing.Union[int, str]]] = None, no_split_module_classes: typing.Optional[list[str]] = None, dtype: typing.Union[str, torch.dtype, NoneType] = None, special_dtypes: typing.Optional[dict[str, typing.Union[str, torch.device]]] = None, low_zero: bool = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/modeling.py#L934)

**Parameters:**

model (`torch.nn.Module`) : The model to analyze.

max_memory (`Dict`, *optional*) : A dictionary device identifier to maximum memory. Will default to the maximum memory available if unset. Example: `max_memory={0: "1GB"}`.

no_split_module_classes (`List[str]`, *optional*) : A list of layer class names that should never be split across device (for instance any layer that has a residual connection).

dtype (`str` or `torch.dtype`, *optional*) : If provided, the weights will be converted to that type when loaded.

special_dtypes (`Dict[str, Union[str, torch.device]]`, *optional*) : If provided, special dtypes to consider for some specific weights (will override dtype used as default for all weights).

low_zero (`bool`, *optional*) : Minimizes the number of weights on GPU 0, which is convenient when it's used for other operations (like the Transformers generate function).

Compute a `max_memory` dictionary for [infer_auto_device_map()](/docs/accelerate/v1.15.0/en/package_reference/utilities#accelerate.infer_auto_device_map) that will balance the use of each available GPU.

All computation is done analyzing sizes and dtypes of the model parameters. As a result, the model can be on the
meta device (as it would if initialized within the `init_empty_weights` context manager).

#### accelerate.utils.get_max_layer_size[[accelerate.utils.get_max_layer_size]]

```python
accelerate.utils.get_max_layer_size(modules: list, module_sizes: dict, no_split_module_classes: list)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/modeling.py#L718)

**Parameters:**

modules (`List[Tuple[str, torch.nn.Module]]`) : The list of named modules where we want to determine the maximum layer size.

module_sizes (`Dict[str, int]`) : A dictionary mapping each layer name to its size (as generated by `compute_module_sizes`).

no_split_module_classes (`List[str]`) : A list of class names for layers we don't want to be split.

**Returns:** `Tuple[int, List[str]]`

The maximum size of a layer with the list of layer names realizing that maximum size.

Utility function that will scan a list of named modules and return the maximum size used by one full layer. The
definition of a layer being:
- a module with no direct children (just parameters and buffers)
- a module whose class name is in the list `no_split_module_classes`

#### accelerate.infer_auto_device_map[[accelerate.infer_auto_device_map]]

```python
accelerate.infer_auto_device_map(model: Module, max_memory: typing.Optional[dict[typing.Union[int, str], typing.Union[int, str]]] = None, no_split_module_classes: typing.Optional[list[str]] = None, dtype: typing.Union[str, torch.dtype, NoneType] = None, special_dtypes: typing.Optional[dict[str, typing.Union[str, torch.dtype]]] = None, verbose: bool = False, clean_result: bool = True, offload_buffers: bool = False, fallback_allocation: bool = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/modeling.py#L1307)

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

#### accelerate.load_checkpoint_in_model[[accelerate.load_checkpoint_in_model]]

```python
accelerate.load_checkpoint_in_model(model: Module, checkpoint: typing.Union[str, os.PathLike], device_map: typing.Optional[dict[str, typing.Union[int, str, torch.device]]] = None, offload_folder: typing.Union[str, os.PathLike, NoneType] = None, dtype: typing.Union[str, torch.dtype, NoneType] = None, offload_state_dict: bool = False, offload_buffers: bool = False, keep_in_fp32_modules: typing.Optional[list[str]] = None, offload_8bit_bnb: bool = False, strict: bool = False, full_state_dict: bool = True, broadcast_from_rank0: bool = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/modeling.py#L1817)

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

Loads a (potentially sharded) checkpoint inside a model, potentially sending weights to a given device as they are
loaded.

Once loaded across devices, you still need to call [dispatch_model()](/docs/accelerate/v1.15.0/en/package_reference/big_modeling#accelerate.dispatch_model) on your model to make it able to run. To
group the checkpoint loading and dispatch in one single call, use [load_checkpoint_and_dispatch()](/docs/accelerate/v1.15.0/en/package_reference/big_modeling#accelerate.load_checkpoint_and_dispatch).

#### accelerate.utils.load_offloaded_weights[[accelerate.utils.load_offloaded_weights]]

```python
accelerate.utils.load_offloaded_weights(model, index, offload_folder)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/modeling.py#L893)

**Parameters:**

model (`torch.nn.Module`) : The model to load the weights into.

index (`dict`) : A dictionary containing the parameter name and its metadata for each parameter that was offloaded from the model.

offload_folder (`str`) : The folder where the offloaded weights are stored.

Loads the weights from the offload folder into the model.

#### accelerate.utils.load_state_dict[[accelerate.utils.load_state_dict]]

```python
accelerate.utils.load_state_dict(checkpoint_file, device_map = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/modeling.py#L1649)

**Parameters:**

checkpoint_file (`str`) : The path to the checkpoint to load.

device_map (`Dict[str, Union[int, str, torch.device]]`, *optional*) : A map that specifies where each submodule should go. It doesn't need to be refined to each parameter/buffer name, once a given module name is inside, every submodule of it will be sent to the same device.

Load a checkpoint from a given file. If the checkpoint is in the safetensors format and a device map is passed, the
weights can be fast-loaded directly on the GPU.

#### accelerate.utils.offload_state_dict[[accelerate.utils.offload_state_dict]]

```python
accelerate.utils.offload_state_dict(save_dir: typing.Union[str, os.PathLike], state_dict: dict)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/offload.py#L94)

**Parameters:**

save_dir (`str` or `os.PathLike`) : The directory in which to offload the state dict.

state_dict (`Dict[str, torch.Tensor]`) : The dictionary of tensors to offload.

Offload a state dict in a given folder.

#### accelerate.utils.retie_parameters[[accelerate.utils.retie_parameters]]

```python
accelerate.utils.retie_parameters(model, tied_params)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/modeling.py#L622)

**Parameters:**

model (`torch.nn.Module`) : The model in which to retie parameters.

tied_params (`List[List[str]]`) : A mapping parameter name to tied parameter name as obtained by `find_tied_parameters`.

Reties tied parameters in a given model if the link was broken (for instance when adding hooks).

#### accelerate.utils.set_module_tensor_to_device[[accelerate.utils.set_module_tensor_to_device]]

```python
accelerate.utils.set_module_tensor_to_device(module: Module, tensor_name: str, device: typing.Union[int, str, torch.device], value: typing.Optional[torch.Tensor] = None, dtype: typing.Union[str, torch.dtype, NoneType] = None, fp16_statistics: typing.Optional[torch.HalfTensor] = None, tied_params_map: typing.Optional[dict[int, dict[torch.device, torch.Tensor]]] = None, non_blocking: bool = False, clear_cache: bool = True)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/modeling.py#L227)

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

A helper function to set a given tensor (parameter of buffer) of a module on a specific device (note that doing
`param.to(device)` creates a new tensor not linked to the parameter, which is why we need this function).

#### accelerate.utils.get_module_children_bottom_up[[accelerate.utils.get_module_children_bottom_up]]

```python
accelerate.utils.get_module_children_bottom_up(model: Module, return_fqns: bool = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/other.py#L592)

**Parameters:**

model (`torch.nn.Module`) : the model to get the children of

**Returns:** `list[torch.nn.Module]`

a list of children modules of `model` in bottom-up order. The last element is the
`model` itself.

Traverse the model in bottom-up order and return the children modules in that order.

## Parallel[[accelerate.utils.extract_model_from_parallel]]

These include general utilities that should be used when working in parallel.

#### accelerate.utils.extract_model_from_parallel[[accelerate.utils.extract_model_from_parallel]]

```python
accelerate.utils.extract_model_from_parallel(model, keep_fp32_wrapper: bool = True, keep_torch_compile: bool = True, recursive: bool = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/other.py#L274)

**Parameters:**

model (`torch.nn.Module`) : The model to extract.

keep_fp32_wrapper (`bool`, *optional*) : Whether to remove mixed precision hooks from the model.

keep_torch_compile (`bool`, *optional*) : Whether to unwrap compiled model.

recursive (`bool`, *optional*, defaults to `False`) : Whether to recursively extract all cases of `module.module` from `model` as well as unwrap child sublayers recursively, not just the top-level distributed containers.

**Returns:** `torch.nn.Module`

The extracted model.

Extract a model from its distributed containers.

#### accelerate.utils.save[[accelerate.utils.save]]

```python
accelerate.utils.save(obj, f, save_on_each_node: bool = False, safe_serialization: bool = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/other.py#L410)

**Parameters:**

obj : The data to save

f : The file (or file-like object) to use to save the data

save_on_each_node (`bool`, *optional*, defaults to `False`) : Whether to only save on the global main process

safe_serialization (`bool`, *optional*, defaults to `False`) : Whether to save `obj` using `safetensors` or the traditional PyTorch way (that uses `pickle`).

Save the data to disk. Use in place of `torch.save()`.

#### accelerate.utils.load[[accelerate.utils.load]]

```python
accelerate.utils.load(f, map_location = None, **kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/other.py#L460)

**Parameters:**

f : The file (or file-like object) to use to load the data

map_location : a function, `torch.device`, string or a dict specifying how to remap storage locations

- ****kwargs** : Additional keyword arguments to pass to `torch.load()`.

Compatible drop-in replacement of `torch.load()` which allows for `weights_only` to be used if `torch` version is
2.4.0 or higher. Otherwise will ignore the kwarg.

Will also add (and then remove) an exception for numpy arrays

#### accelerate.utils.wait_for_everyone[[accelerate.utils.wait_for_everyone]]

```python
accelerate.utils.wait_for_everyone()
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/other.py#L362)

Introduces a blocking point in the script, making sure all processes have reached this point before continuing.

Make sure all processes will reach this instruction otherwise one of your processes will hang forever.

## Random[[accelerate.utils.set_seed]]

These utilities relate to setting and synchronizing of all the random states.

#### accelerate.utils.set_seed[[accelerate.utils.set_seed]]

```python
accelerate.utils.set_seed(seed: int, device_specific: bool = False, deterministic: bool = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/random.py#L40)

**Parameters:**

seed (`int`) : The seed to set.

device_specific (`bool`, *optional*, defaults to `False`) : Whether to differ the seed on each device slightly with `self.process_index`.

deterministic (`bool`, *optional*, defaults to `False`) : Whether to use deterministic algorithms where available. Can slow down training.

Helper function for reproducible behavior to set the seed in `random`, `numpy`, `torch`.

#### accelerate.utils.synchronize_rng_state[[accelerate.utils.synchronize_rng_state]]

```python
accelerate.utils.synchronize_rng_state(rng_type: typing.Optional[accelerate.utils.dataclasses.RNGType] = None, generator: typing.Optional[torch.Generator] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/random.py#L81)

#### accelerate.synchronize_rng_states[[accelerate.synchronize_rng_states]]

```python
accelerate.synchronize_rng_states(rng_types: list, generator: typing.Optional[torch.Generator] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/random.py#L163)

## PyTorch XLA[[accelerate.utils.install_xla]]

These include utilities that are useful while using PyTorch with XLA.

#### accelerate.utils.install_xla[[accelerate.utils.install_xla]]

```python
accelerate.utils.install_xla(upgrade: bool = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/torch_xla.py#L20)

**Parameters:**

upgrade (`bool`, *optional*, defaults to `False`) : Whether to upgrade `torch` and install the latest `torch_xla` wheels.

Helper function to install appropriate xla wheels based on the `torch` version in Google Colaboratory.

Example:

```python
>>> from accelerate.utils import install_xla

>>> install_xla(upgrade=True)
```

## Loading model weights[[accelerate.load_checkpoint_in_model]]

These include utilities that are useful to load checkpoints.

#### accelerate.load_checkpoint_in_model[[accelerate.load_checkpoint_in_model]]

```python
accelerate.load_checkpoint_in_model(model: Module, checkpoint: typing.Union[str, os.PathLike], device_map: typing.Optional[dict[str, typing.Union[int, str, torch.device]]] = None, offload_folder: typing.Union[str, os.PathLike, NoneType] = None, dtype: typing.Union[str, torch.dtype, NoneType] = None, offload_state_dict: bool = False, offload_buffers: bool = False, keep_in_fp32_modules: typing.Optional[list[str]] = None, offload_8bit_bnb: bool = False, strict: bool = False, full_state_dict: bool = True, broadcast_from_rank0: bool = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/modeling.py#L1817)

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

Loads a (potentially sharded) checkpoint inside a model, potentially sending weights to a given device as they are
loaded.

Once loaded across devices, you still need to call [dispatch_model()](/docs/accelerate/v1.15.0/en/package_reference/big_modeling#accelerate.dispatch_model) on your model to make it able to run. To
group the checkpoint loading and dispatch in one single call, use [load_checkpoint_and_dispatch()](/docs/accelerate/v1.15.0/en/package_reference/big_modeling#accelerate.load_checkpoint_and_dispatch).

## Quantization[[accelerate.utils.load_and_quantize_model]]

These include utilities that are useful to quantize model.

#### accelerate.utils.load_and_quantize_model[[accelerate.utils.load_and_quantize_model]]

```python
accelerate.utils.load_and_quantize_model(model: Module, bnb_quantization_config: BnbQuantizationConfig, weights_location: typing.Union[str, os.PathLike, NoneType] = None, device_map: typing.Optional[dict[str, typing.Union[int, str, torch.device]]] = None, no_split_module_classes: typing.Optional[list[str]] = None, max_memory: typing.Optional[dict[typing.Union[int, str], typing.Union[int, str]]] = None, offload_folder: typing.Union[str, os.PathLike, NoneType] = None, offload_state_dict: bool = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/bnb.py#L44)

**Parameters:**

model (`torch.nn.Module`) : Input model. The model can be already loaded or on the meta device

bnb_quantization_config (`BnbQuantizationConfig`) : The bitsandbytes quantization parameters

weights_location (`str` or `os.PathLike`) : The folder weights_location to load. It can be: - a path to a file containing a whole model state dict - a path to a `.json` file containing the index to a sharded checkpoint - a path to a folder containing a unique `.index.json` file and the shards of a checkpoint. - a path to a folder containing a unique pytorch_model.bin file.

device_map (`Dict[str, Union[int, str, torch.device]]`, *optional*) : A map that specifies where each submodule should go. It doesn't need to be refined to each parameter/buffer name, once a given module name is inside, every submodule of it will be sent to the same device.

no_split_module_classes (`List[str]`, *optional*) : A list of layer class names that should never be split across device (for instance any layer that has a residual connection).

max_memory (`Dict`, *optional*) : A dictionary device identifier to maximum memory. Will default to the maximum memory available if unset.

offload_folder (`str` or `os.PathLike`, *optional*) : If the `device_map` contains any value `"disk"`, the folder where we will offload weights.

offload_state_dict (`bool`, *optional*, defaults to `False`) : If `True`, will temporarily offload the CPU state dict on the hard drive to avoid getting out of CPU RAM if the weight of the CPU state dict + the biggest shard does not fit.

**Returns:** `torch.nn.Module`

The quantized model

This function will quantize the input model with the associated config passed in `bnb_quantization_config`. If the
model is in the meta device, we will load and dispatch the weights according to the `device_map` passed. If the
model is already loaded, we will quantize the model and put the model on the GPU,
