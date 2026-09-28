# Working with large models

## Dispatch and offload

### init_empty_weights[[accelerate.init_empty_weights]]

#### accelerate.init_empty_weights[[accelerate.init_empty_weights]]

```python
accelerate.init_empty_weights(include_buffers: typing.Optional[bool] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/big_modeling.py#L61)

**Parameters:**

include_buffers (`bool`, *optional*) : Whether or not to also put all buffers on the meta device while initializing.

A context manager under which models are initialized with all parameters on the meta device, therefore creating an
empty model. Useful when just initializing the model would blow the available RAM.

Example:

```python
import torch.nn as nn
from accelerate import init_empty_weights

# Initialize a model with 100 billions parameters in no time and without using any RAM.
with init_empty_weights():
    tst = nn.Sequential(*[nn.Linear(10000, 10000) for _ in range(1000)])
```

Any model created under this context manager has no weights. As such you can't do something like
`model.to(some_device)` with it. To load weights inside your empty model, see [load_checkpoint_and_dispatch()](/docs/accelerate/v1.15.0/en/package_reference/big_modeling#accelerate.load_checkpoint_and_dispatch).
Make sure to overwrite the default device_map param for [load_checkpoint_and_dispatch()](/docs/accelerate/v1.15.0/en/package_reference/big_modeling#accelerate.load_checkpoint_and_dispatch), otherwise dispatch is not
called.

### cpu_offload[[accelerate.cpu_offload]]

#### accelerate.cpu_offload[[accelerate.cpu_offload]]

```python
accelerate.cpu_offload(model: Module, execution_device: typing.Optional[torch.device] = None, offload_buffers: bool = False, state_dict: typing.Optional[dict[str, torch.Tensor]] = None, preload_module_classes: typing.Optional[list[str]] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/big_modeling.py#L179)

**Parameters:**

model (`torch.nn.Module`) : The model to offload.

execution_device (`torch.device`, *optional*) : The device on which the forward pass of the model will be executed (should be a GPU). Will default to the model first parameter device.

offload_buffers (`bool`, *optional*, defaults to `False`) : Whether or not to offload the buffers with the model parameters.

state_dict (`Dict[str, torch.Tensor]`, *optional*) : The state dict of the model that will be kept on CPU.

preload_module_classes (`List[str]`, *optional*) : A list of classes whose instances should load all their weights (even in the submodules) at the beginning of the forward. This should only be used for classes that have submodules which are registered but not called directly during the forward, for instance if a `dense` linear layer is registered, but at forward, `dense.weight` and `dense.bias` are used in some operations instead of calling `dense` directly.

Activates full CPU offload for a model. As a result, all parameters of the model will be offloaded and only one
copy of the state dict of the model will be kept. During the forward pass, parameters will be extracted from that
state dict and put on the execution device passed as they are needed, then offloaded again.

### cpu_offload_with_hook[[accelerate.cpu_offload_with_hook]]

#### accelerate.cpu_offload_with_hook[[accelerate.cpu_offload_with_hook]]

```python
accelerate.cpu_offload_with_hook(model: Module, execution_device: typing.Union[int, str, torch.device, NoneType] = None, prev_module_hook: typing.Optional[accelerate.hooks.UserCpuOffloadHook] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/big_modeling.py#L225)

**Parameters:**

model (`torch.nn.Module`) : The model to offload.

execution_device(`str`, `int` or `torch.device`, *optional*) : The device on which the model should be executed. Will default to the MPS device if it's available, then device 0 if there is an accelerator device, and finally to the CPU.

prev_module_hook (`UserCpuOffloadHook`, *optional*) : The hook sent back by this function for a previous model in the pipeline you are running. If passed, its offload method will be called just before the forward of the model to which this hook is attached.

Offloads a model on the CPU and puts it back to an execution device when executed. The difference with
[cpu_offload()](/docs/accelerate/v1.15.0/en/package_reference/big_modeling#accelerate.cpu_offload) is that the model stays on the execution device after the forward and is only offloaded again when
the `offload` method of the returned `hook` is called. Useful for pipelines running a model in a loop.

Example:

```py
model_1, hook_1 = cpu_offload_with_hook(model_1, device)
model_2, hook_2 = cpu_offload_with_hook(model_2, device, prev_module_hook=hook_1)
model_3, hook_3 = cpu_offload_with_hook(model_3, device, prev_module_hook=hook_2)

hid_1 = model_1(input)
for i in range(50):
    # model1 is offloaded on the CPU at the first iteration, model 2 stays on the GPU for this whole loop.
    hid_2 = model_2(hid_1)
# model2 is offloaded to the CPU just before this forward.
hid_3 = model_3(hid_3)

# For model3, you need to manually call the hook offload method.
hook_3.offload()
```

### disk_offload[[accelerate.disk_offload]]

#### accelerate.disk_offload[[accelerate.disk_offload]]

```python
accelerate.disk_offload(model: Module, offload_dir: typing.Union[str, os.PathLike], execution_device: typing.Optional[torch.device] = None, offload_buffers: bool = False, preload_module_classes: typing.Optional[list[str]] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/big_modeling.py#L269)

**Parameters:**

model (`torch.nn.Module`) : The model to offload.

offload_dir (`str` or `os.PathLike`) : The folder in which to offload the model weights (or where the model weights are already offloaded).

execution_device (`torch.device`, *optional*) : The device on which the forward pass of the model will be executed (should be a GPU). Will default to the model's first parameter device.

offload_buffers (`bool`, *optional*, defaults to `False`) : Whether or not to offload the buffers with the model parameters.

preload_module_classes (`List[str]`, *optional*) : A list of classes whose instances should load all their weights (even in the submodules) at the beginning of the forward. This should only be used for classes that have submodules which are registered but not called directly during the forward, for instance if a `dense` linear layer is registered, but at forward, `dense.weight` and `dense.bias` are used in some operations instead of calling `dense` directly.

Activates full disk offload for a model. As a result, all parameters of the model will be offloaded as
memory-mapped array in a given folder. During the forward pass, parameters will be accessed from that folder and
put on the execution device passed as they are needed, then offloaded again.

### dispatch_model[[accelerate.dispatch_model]]

#### accelerate.dispatch_model[[accelerate.dispatch_model]]

```python
accelerate.dispatch_model(model: Module, device_map: dict, main_device: typing.Optional[torch.device] = None, state_dict: typing.Optional[dict[str, torch.Tensor]] = None, offload_dir: typing.Union[str, os.PathLike, NoneType] = None, offload_index: typing.Optional[dict[str, str]] = None, offload_buffers: bool = False, skip_keys: typing.Union[str, list[str], NoneType] = None, preload_module_classes: typing.Optional[list[str]] = None, force_hooks: bool = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/big_modeling.py#L315)

**Parameters:**

model (`torch.nn.Module`) : The model to dispatch.

device_map (`Dict[str, Union[str, int, torch.device]]`) : A dictionary mapping module names in the models `state_dict` to the device they should go to. Note that `"disk"` is accepted even if it's not a proper value for `torch.device`.

main_device (`str`, `int` or `torch.device`, *optional*) : The main execution device. Will default to the first device in the `device_map` different from `"cpu"` or `"disk"`.

state_dict (`Dict[str, torch.Tensor]`, *optional*) : The state dict of the part of the model that will be kept on CPU.

offload_dir (`str` or `os.PathLike`) : The folder in which to offload the model weights (or where the model weights are already offloaded).

offload_index (`Dict`, *optional*) : A dictionary from weight name to their information (`dtype`/ `shape` or safetensors filename). Will default to the index saved in `save_folder`.

offload_buffers (`bool`, *optional*, defaults to `False`) : Whether or not to offload the buffers with the model parameters.

skip_keys (`str` or `List[str]`, *optional*) : A list of keys to ignore when moving inputs or outputs between devices.

preload_module_classes (`List[str]`, *optional*) : A list of classes whose instances should load all their weights (even in the submodules) at the beginning of the forward. This should only be used for classes that have submodules which are registered but not called directly during the forward, for instance if a `dense` linear layer is registered, but at forward, `dense.weight` and `dense.bias` are used in some operations instead of calling `dense` directly.

force_hooks (`bool`, *optional*, defaults to `False`) : Whether or not to force device hooks to be attached to the model even if all layers are dispatched to a single device.

Dispatches a model according to a given device map. Layers of the model might be spread across GPUs, offloaded on
the CPU or even the disk.

### load_checkpoint_and_dispatch[[accelerate.load_checkpoint_and_dispatch]]

#### accelerate.load_checkpoint_and_dispatch[[accelerate.load_checkpoint_and_dispatch]]

```python
accelerate.load_checkpoint_and_dispatch(model: Module, checkpoint: typing.Union[str, os.PathLike], device_map: typing.Union[str, dict[str, typing.Union[int, str, torch.device]], NoneType] = None, max_memory: typing.Optional[dict[typing.Union[int, str], typing.Union[int, str]]] = None, no_split_module_classes: typing.Optional[list[str]] = None, offload_folder: typing.Union[str, os.PathLike, NoneType] = None, offload_buffers: bool = False, dtype: typing.Union[str, torch.dtype, NoneType] = None, offload_state_dict: typing.Optional[bool] = None, skip_keys: typing.Union[str, list[str], NoneType] = None, preload_module_classes: typing.Optional[list[str]] = None, force_hooks: bool = False, strict: bool = False, full_state_dict: bool = True, broadcast_from_rank0: bool = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/big_modeling.py#L520)

**Parameters:**

model (`torch.nn.Module`) : The model in which we want to load a checkpoint.

checkpoint (`str` or `os.PathLike`) : The folder checkpoint to load. It can be: - a path to a file containing a whole model state dict - a path to a `.json` file containing the index to a sharded checkpoint - a path to a folder containing a unique `.index.json` file and the shards of a checkpoint.

device_map (`Dict[str, Union[int, str, torch.device]]`, *optional*) : A map that specifies where each submodule should go. It doesn't need to be refined to each parameter/buffer name, once a given module name is inside, every submodule of it will be sent to the same device.  To have Accelerate compute the most optimized `device_map` automatically, set `device_map="auto"`. For more information about each option see [here](../concept_guides/big_model_inference#designing-a-device-map). Defaults to None, which means [dispatch_model()](/docs/accelerate/v1.15.0/en/package_reference/big_modeling#accelerate.dispatch_model) will not be called.

max_memory (`Dict`, *optional*) : A dictionary device identifier to maximum memory. Will default to the maximum memory available for each GPU and the available CPU RAM if unset.

no_split_module_classes (`List[str]`, *optional*) : A list of layer class names that should never be split across device (for instance any layer that has a residual connection).

offload_folder (`str` or `os.PathLike`, *optional*) : If the `device_map` contains any value `"disk"`, the folder where we will offload weights.

offload_buffers (`bool`, *optional*, defaults to `False`) : In the layers that are offloaded on the CPU or the hard drive, whether or not to offload the buffers as well as the parameters.

dtype (`str` or `torch.dtype`, *optional*) : If provided, the weights will be converted to that type when loaded.

offload_state_dict (`bool`, *optional*) : If `True`, will temporarily offload the CPU state dict on the hard drive to avoid getting out of CPU RAM if the weight of the CPU state dict + the biggest shard does not fit. Will default to `True` if the device map picked contains `"disk"` values.

skip_keys (`str` or `List[str]`, *optional*) : A list of keys to ignore when moving inputs or outputs between devices.

preload_module_classes (`List[str]`, *optional*) : A list of classes whose instances should load all their weights (even in the submodules) at the beginning of the forward. This should only be used for classes that have submodules which are registered but not called directly during the forward, for instance if a `dense` linear layer is registered, but at forward, `dense.weight` and `dense.bias` are used in some operations instead of calling `dense` directly.

force_hooks (`bool`, *optional*, defaults to `False`) : Whether or not to force device hooks to be attached to the model even if all layers are dispatched to a single device.

strict (`bool`, *optional*, defaults to `False`) : Whether to strictly enforce that the keys in the checkpoint state_dict match the keys of the model's state_dict.

full_state_dict (`bool`, *optional*, defaults to `True`) : if this is set to `True`, all the tensors in the loaded state_dict will be gathered. No ShardedTensor and DTensor will be in the loaded state_dict.

broadcast_from_rank0 (`False`, *optional*, defaults to `False`) : when the option is `True`, a distributed `ProcessGroup` must be initialized. rank0 should receive a full state_dict and will broadcast the tensors in the state_dict one by one to other ranks. Other ranks will receive the tensors and shard (if applicable) according to the local shards in the model.

Loads a (potentially sharded) checkpoint inside a model, potentially sending weights to a given device as they are
loaded and adds the various hooks that will make this model run properly (even if split across devices).

Example:

```python
>>> from accelerate import init_empty_weights, load_checkpoint_and_dispatch
>>> from huggingface_hub import hf_hub_download
>>> from transformers import AutoConfig, AutoModelForCausalLM

>>> # Download the Weights
>>> checkpoint = "EleutherAI/gpt-j-6B"
>>> weights_location = hf_hub_download(checkpoint, "pytorch_model.bin")

>>> # Create a model and initialize it with empty weights
>>> config = AutoConfig.from_pretrained(checkpoint)
>>> with init_empty_weights():
...     model = AutoModelForCausalLM.from_config(config)

>>> # Load the checkpoint and dispatch it to the right devices
>>> model = load_checkpoint_and_dispatch(
...     model, weights_location, device_map="auto", no_split_module_classes=["GPTJBlock"]
... )
```

### load_checkpoint_in_model[[accelerate.load_checkpoint_in_model]]

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

### infer_auto_device_map[[accelerate.infer_auto_device_map]]

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

## Hooks

### ModelHook[[accelerate.hooks.ModelHook]]

#### accelerate.hooks.ModelHook[[accelerate.hooks.ModelHook]]

```python
accelerate.hooks.ModelHook()
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/hooks.py#L58)

A hook that contains callbacks to be executed just before and after the forward method of a model. The difference
with PyTorch existing hooks is that they get passed along the kwargs.

Class attribute:
- **no_grad** (`bool`, *optional*, defaults to `False`) -- Whether or not to execute the actual forward pass under
  the `torch.no_grad()` context manager.

#### detach_hook[[accelerate.hooks.ModelHook.detach_hook]]

```python
detach_hook(module)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/hooks.py#L106)

**Parameters:**

module (`torch.nn.Module`) : The module detached from this hook.

To be executed when the hook is detached from a module.

#### init_hook[[accelerate.hooks.ModelHook.init_hook]]

```python
init_hook(module)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/hooks.py#L70)

**Parameters:**

module (`torch.nn.Module`) : The module attached to this hook.

To be executed when the hook is attached to the module.

#### post_forward[[accelerate.hooks.ModelHook.post_forward]]

```python
post_forward(module, output)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/hooks.py#L93)

**Parameters:**

module (`torch.nn.Module`) : The module whose forward pass been executed just before this event.

output (`Any`) : The output of the module.

**Returns:** `Any`

The processed `output`.

To be executed just after the forward method of the model.

#### pre_forward[[accelerate.hooks.ModelHook.pre_forward]]

```python
pre_forward(module, *args, **kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/hooks.py#L79)

**Parameters:**

module (`torch.nn.Module`) : The module whose forward pass will be executed just after this event.

args (`Tuple[Any]`) : The positional arguments passed to the module.

kwargs (`Dict[Str, Any]`) : The keyword arguments passed to the module.

**Returns:** `Tuple[Tuple[Any], Dict[Str, Any]]`

A tuple with the treated `args` and `kwargs`.

To be executed just before the forward method of the model.

### AlignDevicesHook[[accelerate.hooks.AlignDevicesHook]]

#### accelerate.hooks.AlignDevicesHook[[accelerate.hooks.AlignDevicesHook]]

```python
accelerate.hooks.AlignDevicesHook(execution_device: typing.Union[int, str, torch.device, NoneType] = None, offload: bool = False, io_same_device: bool = False, weights_map: typing.Optional[collections.abc.Mapping] = None, offload_buffers: bool = False, place_submodules: bool = False, skip_keys: typing.Union[str, list[str], NoneType] = None, tied_params_map: typing.Optional[dict[int, dict[torch.device, torch.Tensor]]] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/hooks.py#L242)

**Parameters:**

execution_device (`torch.device`, *optional*) : The device on which inputs and model weights should be placed before the forward pass.

offload (`bool`, *optional*, defaults to `False`) : Whether or not the weights should be offloaded after the forward pass.

io_same_device (`bool`, *optional*, defaults to `False`) : Whether or not the output should be placed on the same device as the input was.

weights_map (`Mapping[str, torch.Tensor]`, *optional*) : When the model weights are offloaded, a (potentially lazy) map from param names to the tensor values.

offload_buffers (`bool`, *optional*, defaults to `False`) : Whether or not to include the associated module's buffers when offloading.

place_submodules (`bool`, *optional*, defaults to `False`) : Whether to place the submodules on `execution_device` during the `init_hook` event.

A generic `ModelHook` that ensures inputs and model weights are on the same device for the forward pass of the
associated module, potentially offloading the weights after the forward pass.

### SequentialHook[[accelerate.hooks.SequentialHook]]

#### accelerate.hooks.SequentialHook[[accelerate.hooks.SequentialHook]]

```python
accelerate.hooks.SequentialHook(*hooks)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/hooks.py#L116)

A hook that can contain several hooks and iterates through them at each event.

### LayerwiseCastingHook[[accelerate.hooks.LayerwiseCastingHook]]

#### accelerate.hooks.LayerwiseCastingHook[[accelerate.hooks.LayerwiseCastingHook]]

```python
accelerate.hooks.LayerwiseCastingHook(storage_dtype: dtype, compute_dtype: dtype, non_blocking: bool)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/hooks.py#L784)

A hook that casts the weights of a module to a high precision dtype for computation, and to a low precision dtype
for storage. This process may lead to quality loss in the output, but can significantly reduce the memory
footprint.

## Adding Hooks

### add_hook_to_module[[accelerate.hooks.add_hook_to_module]]

#### accelerate.hooks.add_hook_to_module[[accelerate.hooks.add_hook_to_module]]

```python
accelerate.hooks.add_hook_to_module(module: Module, hook: ModelHook, append: bool = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/hooks.py#L147)

**Parameters:**

module (`torch.nn.Module`) : The module to attach a hook to.

hook (`ModelHook`) : The hook to attach.

append (`bool`, *optional*, defaults to `False`) : Whether the hook should be chained with an existing one (if module already contains a hook) or not.

**Returns:** `torch.nn.Module`

The same module, with the hook attached (the module is modified in place, so the result can
be discarded).

Adds a hook to a given module. This will rewrite the `forward` method of the module to include the hook, to remove
this behavior and restore the original `forward` method, use `remove_hook_from_module`.

If the module already contains a hook, this will replace it with the new hook passed by default. To chain two hooks
together, pass `append=True`, so it chains the current and new hook into an instance of the `SequentialHook` class.

### attach_execution_device_hook[[accelerate.hooks.attach_execution_device_hook]]

#### accelerate.hooks.attach_execution_device_hook[[accelerate.hooks.attach_execution_device_hook]]

```python
accelerate.hooks.attach_execution_device_hook(module: Module, execution_device: typing.Union[int, str, torch.device], skip_keys: typing.Union[str, list[str], NoneType] = None, preload_module_classes: typing.Optional[list[str]] = None, tied_params_map: typing.Optional[dict[int, dict[torch.device, torch.Tensor]]] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/hooks.py#L443)

**Parameters:**

module (`torch.nn.Module`) : The module where we want to attach the hooks.

execution_device (`int`, `str` or `torch.device`) : The device on which inputs and model weights should be placed before the forward pass.

skip_keys (`str` or `List[str]`, *optional*) : A list of keys to ignore when moving inputs or outputs between devices.

preload_module_classes (`List[str]`, *optional*) : A list of classes whose instances should load all their weights (even in the submodules) at the beginning of the forward. This should only be used for classes that have submodules which are registered but not called directly during the forward, for instance if a `dense` linear layer is registered, but at forward, `dense.weight` and `dense.bias` are used in some operations instead of calling `dense` directly.

tied_params_map (Optional[Dict[int, Dict[torch.device, torch.Tensor]]], *optional*, defaults to `None`) : A map of data pointers to dictionaries of devices to already dispatched tied weights. For a given execution device, this parameter is useful to reuse the first available pointer of a shared weight for all others, instead of duplicating memory.

Recursively attaches `AlignDevicesHook` to all submodules of a given model to make sure they have the right
execution device

### attach_align_device_hook[[accelerate.hooks.attach_align_device_hook]]

#### accelerate.hooks.attach_align_device_hook[[accelerate.hooks.attach_align_device_hook]]

```python
accelerate.hooks.attach_align_device_hook(module: Module, execution_device: typing.Optional[torch.device] = None, offload: bool = False, weights_map: typing.Optional[collections.abc.Mapping] = None, offload_buffers: bool = False, module_name: str = '', skip_keys: typing.Union[str, list[str], NoneType] = None, preload_module_classes: typing.Optional[list[str]] = None, tied_params_map: typing.Optional[dict[int, dict[torch.device, torch.Tensor]]] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/hooks.py#L491)

**Parameters:**

module (`torch.nn.Module`) : The module where we want to attach the hooks.

execution_device (`torch.device`, *optional*) : The device on which inputs and model weights should be placed before the forward pass.

offload (`bool`, *optional*, defaults to `False`) : Whether or not the weights should be offloaded after the forward pass.

weights_map (`Mapping[str, torch.Tensor]`, *optional*) : When the model weights are offloaded, a (potentially lazy) map from param names to the tensor values.

offload_buffers (`bool`, *optional*, defaults to `False`) : Whether or not to include the associated module's buffers when offloading.

module_name (`str`, *optional*, defaults to `""`) : The name of the module.

skip_keys (`str` or `List[str]`, *optional*) : A list of keys to ignore when moving inputs or outputs between devices.

preload_module_classes (`List[str]`, *optional*) : A list of classes whose instances should load all their weights (even in the submodules) at the beginning of the forward. This should only be used for classes that have submodules which are registered but not called directly during the forward, for instance if a `dense` linear layer is registered, but at forward, `dense.weight` and `dense.bias` are used in some operations instead of calling `dense` directly.

tied_params_map (Optional[Dict[int, Dict[torch.device, torch.Tensor]]], *optional*, defaults to `None`) : A map of data pointers to dictionaries of devices to already dispatched tied weights. For a given execution device, this parameter is useful to reuse the first available pointer of a shared weight for all others, instead of duplicating memory.

Recursively attaches `AlignDevicesHook` to all submodules of a given model that have direct parameters and/or
buffers.

### attach_align_device_hook_on_blocks[[accelerate.hooks.attach_align_device_hook_on_blocks]]

#### accelerate.hooks.attach_align_device_hook_on_blocks[[accelerate.hooks.attach_align_device_hook_on_blocks]]

```python
accelerate.hooks.attach_align_device_hook_on_blocks(module: Module, execution_device: typing.Union[torch.device, dict[str, torch.device], NoneType] = None, offload: typing.Union[bool, dict[str, bool]] = False, weights_map: typing.Optional[collections.abc.Mapping] = None, offload_buffers: bool = False, module_name: str = '', skip_keys: typing.Union[str, list[str], NoneType] = None, preload_module_classes: typing.Optional[list[str]] = None, tied_params_map: typing.Optional[dict[int, dict[torch.device, torch.Tensor]]] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/hooks.py#L586)

**Parameters:**

module (`torch.nn.Module`) : The module where we want to attach the hooks.

execution_device (`torch.device` or `Dict[str, torch.device]`, *optional*) : The device on which inputs and model weights should be placed before the forward pass. It can be one device for the whole module, or a dictionary mapping module name to device.

offload (`bool`, *optional*, defaults to `False`) : Whether or not the weights should be offloaded after the forward pass. It can be one boolean for the whole module, or a dictionary mapping module name to boolean.

weights_map (`Mapping[str, torch.Tensor]`, *optional*) : When the model weights are offloaded, a (potentially lazy) map from param names to the tensor values.

offload_buffers (`bool`, *optional*, defaults to `False`) : Whether or not to include the associated module's buffers when offloading.

module_name (`str`, *optional*, defaults to `""`) : The name of the module.

skip_keys (`str` or `List[str]`, *optional*) : A list of keys to ignore when moving inputs or outputs between devices.

preload_module_classes (`List[str]`, *optional*) : A list of classes whose instances should load all their weights (even in the submodules) at the beginning of the forward. This should only be used for classes that have submodules which are registered but not called directly during the forward, for instance if a `dense` linear layer is registered, but at forward, `dense.weight` and `dense.bias` are used in some operations instead of calling `dense` directly.

tied_params_map (Optional[Dict[int, Dict[torch.device, torch.Tensor]]], *optional*, defaults to `None`) : A map of data pointers to dictionaries of devices to already dispatched tied weights. For a given execution device, this parameter is useful to reuse the first available pointer of a shared weight for all others, instead of duplicating memory.

Attaches `AlignDevicesHook` to all blocks of a given model as needed.

### attach_layerwise_casting_hooks[[accelerate.big_modeling.attach_layerwise_casting_hooks]]

#### accelerate.big_modeling.attach_layerwise_casting_hooks[[accelerate.big_modeling.attach_layerwise_casting_hooks]]

```python
accelerate.big_modeling.attach_layerwise_casting_hooks(module: Module, storage_dtype: dtype, compute_dtype: dtype, skip_modules_pattern: typing.Union[str, tuple[str, ...], NoneType] = None, skip_modules_classes: typing.Optional[tuple[type[torch.nn.Module], ...]] = None, non_blocking: bool = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/big_modeling.py#L661)

**Parameters:**

module (`torch.nn.Module`) : The module whose leaf modules will be cast to a high precision dtype for computation, and to a low precision dtype for storage.

storage_dtype (`torch.dtype`) : The dtype to cast the module to before/after the forward pass for storage.

compute_dtype (`torch.dtype`) : The dtype to cast the module to during the forward pass for computation.

skip_modules_pattern (`tuple[str, ...]`, defaults to `None`) : A list of patterns to match the names of the modules to skip during the layerwise casting process. If set to `None` alongside `skip_modules_classes` being `None`, the layerwise casting is applied directly to the module instead of its internal submodules.

skip_modules_classes (`tuple[type[torch.nn.Module], ...]`, defaults to `None`) : A list of module classes to skip during the layerwise casting process.

non_blocking (`bool`, defaults to `False`) : If `True`, the weight casting operations are non-blocking.

Applies layerwise casting to a given module. The module expected here is a PyTorch `nn.Module`. This is helpful for
reducing memory requirements when one doesn't want to fully quantize a model. Model params can be kept in say,
`torch.float8_e4m3fn` and upcasted to a higher precision like `torch.bfloat16` during forward pass and downcasted
back to `torch.float8_e4m3fn` to realize memory savings.

Example:

```python
>>> from accelerate.hooks import attach_layerwise_casting_hooks
>>> from transformers import AutoModelForCausalLM
>>> import torch

>>> # Model
>>> checkpoint = "EleutherAI/gpt-j-6B"
>>> model = AutoModelForCausalLM.from_pretrained(checkpoint)

>>> # Attach hooks and perform inference
>>> attach_layerwise_casting_hooks(model, storage_dtype=torch.float8_e4m3fn, compute_dtype=torch.bfloat16)
>>> with torch.no_grad():
...     model(...)
```

Users can also pass modules they want to avoid from getting downcasted.

```py
>>> attach_layerwise_casting_hooks(
...     model, storage_dtype=torch.float8_e4m3fn, compute_dtype=torch.bfloat16, skip_modules_pattern=["norm"]
... )
```

## Removing Hooks

### remove_hook_from_module[[accelerate.hooks.remove_hook_from_module]]

#### accelerate.hooks.remove_hook_from_module[[accelerate.hooks.remove_hook_from_module]]

```python
accelerate.hooks.remove_hook_from_module(module: Module, recurse = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/hooks.py#L205)

**Parameters:**

module (`torch.nn.Module`) : The module to attach a hook to.

recurse (`bool`, **optional**) : Whether to remove the hooks recursively

**Returns:** `torch.nn.Module`

The same module, with the hook detached (the module is modified in place, so the result can
be discarded).

Removes any hook attached to a module via `add_hook_to_module`.

### remove_hook_from_submodules[[accelerate.hooks.remove_hook_from_submodules]]

#### accelerate.hooks.remove_hook_from_submodules[[accelerate.hooks.remove_hook_from_submodules]]

```python
accelerate.hooks.remove_hook_from_submodules(module: Module)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/hooks.py#L574)

**Parameters:**

module (`torch.nn.Module`) : The module on which to remove all hooks.

Recursively removes all hooks attached on the submodules of a given model.

## Utilities

### has_offloaded_params[[accelerate.utils.has_offloaded_params]]

#### accelerate.utils.has_offloaded_params[[accelerate.utils.has_offloaded_params]]

```python
accelerate.utils.has_offloaded_params(module: Module)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/modeling.py#L2162)

**Parameters:**

module (`torch.nn.Module`) : The module to check for an offload hook.

**Returns:** `bool`

`True` if the module has an offload hook and offloading is enabled, `False` otherwise.

Checks if a module has offloaded parameters by checking if the given module has a AlignDevicesHook attached with
offloading enabled

### align_module_device[[accelerate.utils.align_module_device]]

#### accelerate.utils.align_module_device[[accelerate.utils.align_module_device]]

```python
accelerate.utils.align_module_device(module: Module, execution_device: typing.Optional[torch.device] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/modeling.py#L2178)

**Parameters:**

module (`torch.nn.Module`) : Module with parameters to align.

execution_device (`torch.device`, *optional*) : If provided, overrides the module's execution device within the context. Otherwise, use hook execution device or pass

Context manager that moves a module's parameters to the specified execution device.
