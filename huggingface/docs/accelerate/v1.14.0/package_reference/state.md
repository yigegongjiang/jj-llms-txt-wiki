# Stateful Classes

Below are variations of a [singleton class](https://en.wikipedia.org/wiki/Singleton_pattern) in the sense that all
instances share the same state, which is initialized on the first instantiation.

These classes are immutable and store information about certain configurations or 
states.

## PartialState[[accelerate.PartialState]]

#### accelerate.PartialState[[accelerate.PartialState]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/state.py#L123)

Singleton class that has information about the current training environment and functions to help with process
control. Designed to be used when only process control and device execution states are needed. Does *not* need to
be initialized from `Accelerator`.

**Available attributes:**

- **device** (`torch.device`) -- The device to use.
- **distributed_type** ([DistributedType](/docs/accelerate/v1.14.0/en/package_reference/utilities#accelerate.DistributedType)) -- The type of distributed environment currently
  in use.
- **local_process_index** (`int`) -- The index of the current process on the current server.
- **mixed_precision** (`str`) -- Whether or not the current script will use mixed precision, and if so the type
  of mixed precision being performed. (Choose from 'no','fp16','bf16 or 'fp8').
- **num_processes** (`int`) -- The number of processes currently launched in parallel.
- **process_index** (`int`) -- The index of the current process.
- **is_last_process** (`bool`) -- Whether or not the current process is the last one.
- **is_main_process** (`bool`) -- Whether or not the current process is the main one.
- **is_local_main_process** (`bool`) -- Whether or not the current process is the main one on the local node.
- **debug** (`bool`) -- Whether or not the current script is being run in debug mode.

Example:
```python
from accelerate.utils import InitProcessGroupKwargs

# To include `InitProcessGroupKwargs`, init then call `.to_kwargs()`
kwargs = InitProcessGroupKwargs(...).to_kwargs()
state = PartialState(**kwargs)
```

destroy_process_groupaccelerate.PartialState.destroy_process_grouphttps://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/state.py#L845[{"name": "group", "val": " = None"}]

Destroys the process group. If one is not specified, the default process group is destroyed.

**Parameters:**

cpu (`bool`, *optional*) : Whether or not to force the script to execute on CPU. Will ignore any accelerators available if set to `True` and force the execution on the CPU.

kwargs (additional keyword arguments, *optional*) : Additional keyword arguments to pass to the relevant `init_process_group` function. Valid `kwargs` can be found in [utils.InitProcessGroupKwargs](/docs/accelerate/v1.14.0/en/package_reference/kwargs#accelerate.InitProcessGroupKwargs). See the example section for detailed usage.
#### local_main_process_first[[accelerate.PartialState.local_main_process_first]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/state.py#L534)

Lets the local main process go inside a with block.

The other processes will enter the with block after the main process exits.

Example:

```python
>>> from accelerate.state import PartialState

>>> state = PartialState()
>>> with state.local_main_process_first():
...     # This will be printed first by local process 0 then in a seemingly
...     # random order by the other processes.
...     print(f"This will be printed by process {state.local_process_index}")
```
#### main_process_first[[accelerate.PartialState.main_process_first]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/state.py#L513)

Lets the main process go first inside a with block.

The other processes will enter the with block after the main process exits.

Example:

```python
>>> from accelerate import Accelerator

>>> accelerator = Accelerator()
>>> with accelerator.main_process_first():
...     # This will be printed first by process 0 then in a seemingly
...     # random order by the other processes.
...     print(f"This will be printed by process {accelerator.process_index}")
```
#### on_last_process[[accelerate.PartialState.on_last_process]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/state.py#L616)

Decorator that only runs the decorated function on the last process.

Example:
```python
# Assume we have 4 processes.
from accelerate.state import PartialState

state = PartialState()

@state.on_last_process
def print_something():
    print(f"Printed on process {state.process_index}")

print_something()
"Printed on process 3"
```

**Parameters:**

function (`Callable`) : The function to decorate.
#### on_local_main_process[[accelerate.PartialState.on_local_main_process]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/state.py#L585)

Decorator that only runs the decorated function on the local main process.

Example:
```python
# Assume we have 2 servers with 4 processes each.
from accelerate.state import PartialState

state = PartialState()

@state.on_local_main_process
def print_something():
    print("This will be printed by process 0 only on each server.")

print_something()
# On server 1:
"This will be printed by process 0 only"
# On server 2:
"This will be printed by process 0 only"
```

**Parameters:**

function (`Callable`) : The function to decorate.
#### on_local_process[[accelerate.PartialState.on_local_process]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/state.py#L677)

Decorator that only runs the decorated function on the process with the given index on the current node.

Example:
```python
# Assume we have 2 servers with 4 processes each.
from accelerate import Accelerator

accelerator = Accelerator()

@accelerator.on_local_process(local_process_index=2)
def print_something():
    print(f"Printed on process {accelerator.local_process_index}")

print_something()
# On server 1:
"Printed on process 2"
# On server 2:
"Printed on process 2"
```

**Parameters:**

function (`Callable`, *optional*) : The function to decorate.

local_process_index (`int`, *optional*) : The index of the local process on which to run the function.
#### on_main_process[[accelerate.PartialState.on_main_process]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/state.py#L555)

Decorator that only runs the decorated function on the main process.

Example:

```python
>>> from accelerate.state import PartialState

>>> state = PartialState()

>>> @state.on_main_process
... def print_something():
...     print("This will be printed by process 0 only.")

>>> print_something()
"This will be printed by process 0 only"
```

**Parameters:**

function (`Callable`) : The function to decorate.
#### on_process[[accelerate.PartialState.on_process]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/state.py#L644)

Decorator that only runs the decorated function on the process with the given index.

Example:
```python
# Assume we have 4 processes.
from accelerate.state import PartialState

state = PartialState()

@state.on_process(process_index=2)
def print_something():
    print(f"Printed on process {state.process_index}")

print_something()
"Printed on process 2"
```

**Parameters:**

function (`Callable`, `optional`) : The function to decorate.

process_index (`int`, `optional`) : The index of the process on which to run the function.
#### set_device[[accelerate.PartialState.set_device]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/state.py#L819)

Sets the device in `self.device` to the current distributed environment.
#### split_between_processes[[accelerate.PartialState.split_between_processes]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/state.py#L425)

Splits `input` between `self.num_processes` quickly and can be then used on that process. Useful when doing
distributed inference, such as with different prompts.

Note that when using a `dict`, all keys need to have the same number of elements.

Example:

```python
# Assume there are two processes
from accelerate import PartialState

state = PartialState()
with state.split_between_processes(["A", "B", "C"]) as inputs:
    print(inputs)
# Process 0
["A", "B"]
# Process 1
["C"]

with state.split_between_processes(["A", "B", "C"], apply_padding=True) as inputs:
    print(inputs)
# Process 0
["A", "B"]
# Process 1
["C", "C"]
```

**Parameters:**

inputs (`list`, `tuple`, `torch.Tensor`, `dict` of `list`/`tuple`/`torch.Tensor`, or `datasets.Dataset`) : The input to split between processes.

apply_padding (`bool`, `optional`, defaults to `False`) : Whether to apply padding by repeating the last element of the input so that all processes have the same number of elements. Useful when trying to perform actions such as `gather()` on the outputs or passing in less inputs than there are processes. If so, just remember to drop the padded elements afterwards.
#### wait_for_everyone[[accelerate.PartialState.wait_for_everyone]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/state.py#L377)

Will stop the execution of the current process until every other process has reached that point (so this does
nothing when the script is only run in one process). Useful to do before saving a model.

Example:

```python
>>> # Assuming two GPU processes
>>> import time
>>> from accelerate.state import PartialState

>>> state = PartialState()
>>> if state.is_main_process:
...     time.sleep(2)
>>> else:
...     print("I'm waiting for the main process to finish its sleep...")
>>> state.wait_for_everyone()
>>> # Should print on every process at the same time
>>> print("Everyone is here")
```

## AcceleratorState[[accelerate.state.AcceleratorState]]

#### accelerate.state.AcceleratorState[[accelerate.state.AcceleratorState]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/state.py#L868)

Singleton class that has information about the current training environment.

**Available attributes:**

- **device** (`torch.device`) -- The device to use.
- **distributed_type** ([DistributedType](/docs/accelerate/v1.14.0/en/package_reference/utilities#accelerate.DistributedType)) -- The type of distributed environment currently
  in use.
- **parallelism_config** (`ParallelismConfig`) -- The parallelism configuration for the
  current training environment. This is used to configure the distributed training environment.
- **initialized** (`bool`) -- Whether or not the `AcceleratorState` has been initialized from `Accelerator`.
- **local_process_index** (`int`) -- The index of the current process on the current server.
- **mixed_precision** (`str`) -- Whether or not the current script will use mixed precision, and if so the type
  of mixed precision being performed. (Choose from 'no','fp16','bf16 or 'fp8').
- **num_processes** (`int`) -- The number of processes currently launched in parallel.
- **process_index** (`int`) -- The index of the current process.
- **is_last_process** (`bool`) -- Whether or not the current process is the last one.
- **is_main_process** (`bool`) -- Whether or not the current process is the main one.
- **is_local_main_process** (`bool`) -- Whether or not the current process is the main one on the local node.
- **debug** (`bool`) -- Whether or not the current script is being run in debug mode.

destroy_process_groupaccelerate.state.AcceleratorState.destroy_process_grouphttps://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/state.py#L1081[{"name": "group", "val": " = None"}]

Destroys the process group. If one is not specified, the default process group is destroyed.

If `self.fork_launched` is `True` and `group` is `None`, nothing happens.
#### get_deepspeed_plugin[[accelerate.state.AcceleratorState.get_deepspeed_plugin]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/state.py#L1198)

Returns the DeepSpeedPlugin with the given plugin_key.
#### local_main_process_first[[accelerate.state.AcceleratorState.local_main_process_first]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/state.py#L1174)

Lets the local main process go inside a with block.

The other processes will enter the with block after the main process exits.
#### main_process_first[[accelerate.state.AcceleratorState.main_process_first]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/state.py#L1164)

Lets the main process go first inside a with block.

The other processes will enter the with block after the main process exits.
#### select_deepspeed_plugin[[accelerate.state.AcceleratorState.select_deepspeed_plugin]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/state.py#L1205)

Activates the DeepSpeedPlugin with the given `name`, and will disable all other plugins.
#### split_between_processes[[accelerate.state.AcceleratorState.split_between_processes]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/state.py#L1122)

Splits `input` between `self.num_processes` quickly and can be then used on that process. Useful when doing
distributed inference, such as with different prompts.

Note that when using a `dict`, all keys need to have the same number of elements.

Example:

```python
# Assume there are two processes
from accelerate.state import AcceleratorState

state = AcceleratorState()
with state.split_between_processes(["A", "B", "C"]) as inputs:
    print(inputs)
# Process 0
["A", "B"]
# Process 1
["C"]

with state.split_between_processes(["A", "B", "C"], apply_padding=True) as inputs:
    print(inputs)
# Process 0
["A", "B"]
# Process 1
["C", "C"]
```

**Parameters:**

inputs (`list`, `tuple`, `torch.Tensor`, or `dict` of `list`/`tuple`/`torch.Tensor`) : The input to split between processes.

apply_padding (`bool`, `optional`, defaults to `False`) : Whether to apply padding by repeating the last element of the input so that all processes have the same number of elements. Useful when trying to perform actions such as `gather()` on the outputs or passing in less inputs than there are processes. If so, just remember to drop the padded elements afterwards.

## GradientState[[accelerate.state.GradientState]]

#### accelerate.state.GradientState[[accelerate.state.GradientState]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/state.py#L1231)

Singleton class that has information related to gradient synchronization for gradient accumulation

**Available attributes:**

- **end_of_dataloader** (`bool`) -- Whether we have reached the end the current dataloader
- **remainder** (`int`) -- The number of extra samples that were added from padding the dataloader
- **sync_gradients** (`bool`) -- Whether the gradients should be synced across all devices
- **active_dataloader** (`Optional[DataLoader]`) -- The dataloader that is currently being iterated over
- **dataloader_references** (`List[Optional[DataLoader]]`) -- A list of references to the dataloaders that are
  being iterated over
- **num_steps** (`int`) -- The number of steps to accumulate over
- **adjust_scheduler** (`bool`) -- Whether the scheduler should be adjusted to account for the gradient
  accumulation
- **sync_with_dataloader** (`bool`) -- Whether the gradients should be synced at the end of the dataloader
  iteration and the number of total steps reset
- **is_xla_gradients_synced** (`bool`) -- Whether the XLA gradients have been synchronized. It is initialized
  as false. Once gradients have been reduced before the optimizer step, this flag is set to true. Subsequently,
  after each step, the flag is reset to false. FSDP will always synchronize the gradients, hence
  is_xla_gradients_synced is always true.
