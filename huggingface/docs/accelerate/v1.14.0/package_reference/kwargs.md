# Kwargs handlers

The following objects can be passed to the main [Accelerator](/docs/accelerate/v1.14.0/en/package_reference/accelerator#accelerate.Accelerator) to customize how some PyTorch objects
related to distributed training or mixed precision are created.

## AutocastKwargs[[accelerate.AutocastKwargs]]

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

## DistributedDataParallelKwargs[[accelerate.DistributedDataParallelKwargs]]

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

## FP8RecipeKwargs[[accelerate.utils.FP8RecipeKwargs]]

#### accelerate.utils.FP8RecipeKwargs[[accelerate.utils.FP8RecipeKwargs]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L457)

Deprecated. Please use one of the proper FP8 recipe kwargs classes such as `TERecipeKwargs` or `MSAMPRecipeKwargs`
instead.

## ProfileKwargs[[accelerate.ProfileKwargs]]

#### accelerate.ProfileKwargs[[accelerate.ProfileKwargs]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L486)

Use this object in your [Accelerator](/docs/accelerate/v1.14.0/en/package_reference/accelerator#accelerate.Accelerator) to customize the initialization of the profiler. Please refer to the
documentation of this [context manager](https://pytorch.org/docs/stable/profiler.html#torch.profiler.profile) for
more information on each argument.

`torch.profiler` is only available in PyTorch 1.8.1 and later versions.

Example:

```python
from accelerate import Accelerator
from accelerate.utils import ProfileKwargs

kwargs = ProfileKwargs(activities=["cpu", "cuda"])
accelerator = Accelerator(kwargs_handlers=[kwargs])
```

buildaccelerate.ProfileKwargs.buildhttps://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L576[]torch.profiler.profileThe profiler object.

Build a profiler object with the current configuration.

**Parameters:**

activities (`List[str]`, *optional*, default to `None`) : The list of activity groups to use in profiling. Must be one of `"cpu"`, `"xpu"`, `"mtia"`, "hpu" or `"cuda"`.

schedule_option (`Dict[str, int]`, *optional*, default to `None`) : The schedule option to use for the profiler. Available keys are `wait`, `warmup`, `active`, `repeat` and `skip_first`. The profiler will skip the first `skip_first` steps, then wait for `wait` steps, then do the warmup for the next `warmup` steps, then do the active recording for the next `active` steps and then repeat the cycle starting with `wait` steps. The optional number of cycles is specified with the `repeat` parameter, the zero value means that the cycles will continue until the profiling is finished.

on_trace_ready (`Callable`, *optional*, default to `None`) : Callable that is called at each step when schedule returns `ProfilerAction.RECORD_AND_SAVE` during the profiling.

record_shapes (`bool`, *optional*, default to `False`) : Save information about operator’s input shapes.

profile_memory (`bool`, *optional*, default to `False`) : Track tensor memory allocation/deallocation

with_stack (`bool`, *optional*, default to `False`) : Record source information (file and line number) for the ops.

with_flops (`bool`, *optional*, default to `False`) : Use formula to estimate the FLOPS of specific operators

with_modules (`bool`, *optional*, default to `False`) : Record module hierarchy (including function names) corresponding to the callstack of the op.

output_trace_dir (`str`, *optional*, default to `None`) : Exports the collected trace in Chrome JSON format. Chrome use 'chrome://tracing' view json file. Defaults to None, which means profiling does not store json files.

**Returns:**

`torch.profiler.profile`

The profiler object.

## GradScalerKwargs[[accelerate.GradScalerKwargs]]

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

## InitProcessGroupKwargs[[accelerate.InitProcessGroupKwargs]]

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

## KwargsHandler[[accelerate.utils.KwargsHandler]]

#### accelerate.utils.KwargsHandler[[accelerate.utils.KwargsHandler]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L70)

Internal mixin that implements a `to_kwargs()` method for a dataclass.

to_kwargsaccelerate.utils.KwargsHandler.to_kwargshttps://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L78[]

Returns a dictionary containing the attributes with values different from the default of this class.
