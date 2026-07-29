# Fully Sharded Data Parallel utilities

## enable_fsdp_ram_efficient_loading[[accelerate.utils.enable_fsdp_ram_efficient_loading]]

#### accelerate.utils.enable_fsdp_ram_efficient_loading[[accelerate.utils.enable_fsdp_ram_efficient_loading]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/fsdp_utils.py#L39)

Enables RAM efficient loading of Hugging Face models for FSDP in the environment.

## disable_fsdp_ram_efficient_loading[[accelerate.utils.disable_fsdp_ram_efficient_loading]]

#### accelerate.utils.disable_fsdp_ram_efficient_loading[[accelerate.utils.disable_fsdp_ram_efficient_loading]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/fsdp_utils.py#L49)

Disables RAM efficient loading of Hugging Face models for FSDP in the environment.

## merge_fsdp_weights[[accelerate.utils.merge_fsdp_weights]]

#### accelerate.utils.merge_fsdp_weights[[accelerate.utils.merge_fsdp_weights]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/fsdp_utils.py#L366)

Merge the weights from sharded FSDP model checkpoints into a single combined checkpoint. Should be used if
`SHARDED_STATE_DICT` was used for the model. Weights will be saved to `{output_path}/model.safetensors` if
`safe_serialization` else `pytorch_model.bin`.

Note: this is a CPU-bound process.

**Parameters:**

checkpoint_dir (`str`) : The directory containing the FSDP checkpoints (can be either the model or optimizer).

output_path (`str`) : The path to save the merged checkpoint.

safe_serialization (`bool`, *optional*, defaults to `True`) : Whether to save the merged weights with safetensors (recommended).

remove_checkpoint_dir (`bool`, *optional*, defaults to `False`) : Whether to remove the checkpoint directory after merging.

## FullyShardedDataParallelPlugin[[accelerate.FullyShardedDataParallelPlugin]]

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

## fsdp2_load_full_state_dict[[accelerate.utils.fsdp2_load_full_state_dict]]

#### accelerate.utils.fsdp2_load_full_state_dict[[accelerate.utils.fsdp2_load_full_state_dict]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/fsdp_utils.py#L467)

Loads the full state dict (could be only on rank 0) into the sharded model. This is done by broadcasting the
parameters from rank 0 to all other ranks. This function modifies the model in-place.

**Parameters:**

accelerator (`Accelerator`) : The accelerator instance

model (`torch.nn.Module`) : The model to load the state dict into, expected to be on meta device or a VRAM spike can occur

full_sd (`dict`) : The full state dict to load, can only be on rank 0

cpu_offload (`bool`, defaults to `False`) : If True, move sharded parameters to CPU after distribution. Required when FSDP CPU offloading is enabled.

## fsdp2_switch_optimizer_parameters[[accelerate.utils.fsdp2_switch_optimizer_parameters]]

#### accelerate.utils.fsdp2_switch_optimizer_parameters[[accelerate.utils.fsdp2_switch_optimizer_parameters]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/fsdp_utils.py#L563)

Switches the parameters of the optimizer to new ones (sharded parameters in usual case). This function modifies the
optimizer in-place.

**Parameters:**

optimizer (`torch.optim.Optimizer`) : Optimizer instance which contains the original model parameters

mapping (`dict`) : Mapping from the original parameter (specified by `data_ptr`) to the sharded parameter

## fsdp2_prepare_model[[accelerate.utils.fsdp2_prepare_model]]

#### accelerate.utils.fsdp2_prepare_model[[accelerate.utils.fsdp2_prepare_model]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/fsdp_utils.py#L645)

Prepares the model for FSDP2 in-place. Also returns the model to avoid misuse of the original model.

**Parameters:**

accelerator (`Accelerator`) : The accelerator instance

model (`torch.nn.Module`) : The model to prepare

**Returns:**

``torch.nn.Module``

Prepared model

## fsdp2_prepare_auto_wrap_policy
