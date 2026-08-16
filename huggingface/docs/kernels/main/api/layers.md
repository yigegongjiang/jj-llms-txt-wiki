# Layers API Reference

## Making layers kernel-aware

### use_kernel_forward_from_hub[[kernels.use_kernel_forward_from_hub]]

#### kernels.use_kernel_forward_from_hub[[kernels.use_kernel_forward_from_hub]]

```python
kernels.use_kernel_forward_from_hub(layer_name: str)
```

[Source](https://github.com/huggingface/kernels/blob/main/kernels/src/kernels/layer/layer.py#L272)

**Parameters:**

layer_name (`str`) : The name of the layer to use for kernel lookup in registered mappings.

**Returns:** `Callable`

A decorator function that can be applied to layer classes.

Decorator factory that makes a layer extensible using the specified layer name.

This decorator which prepares a layer class to use kernel layers from the Hugging
Face Hub.

When applied to a function, the function is converted into a layer (`nn.Module`),
made extensible using the given layer name, and then the class is instantiated.
Since `nn.Module` also implements the `__call__` method, the module can still be
used as if it was a function. Note that a decorated function is only visible to
[kernelize()](/docs/kernels/main/en/api/layers#kernels.kernelize) if it is attached to a module using [use_kernelized_func()](/docs/kernels/main/en/api/layers#kernels.use_kernelized_func).

Example:
```python
import torch
import torch.nn as nn

from kernels import use_kernel_forward_from_hub
from kernels import use_kernelized_func
from kernels import Mode, kernelize

@use_kernel_forward_from_hub("MyCustomLayer")
class MyCustomLayer(nn.Module):
    def __init__(self, hidden_size):
        super().__init__()
        self.hidden_size = hidden_size

    def forward(self, x: torch.Tensor):
        # original implementation
        return x

model = MyCustomLayer(768)

# The layer can now be kernelized:
# model = kernelize(model, mode=Mode.TRAINING | Mode.TORCH_COMPILE, device="cuda")

# Use on a function (converts the function to `nn.Module`). The function
# can then be replaced with a layer mapping for `MyCustomLayer`.
@use_kernel_forward_from_hub("MyCustomLayer")
def identity(x: torch.Tensor) -> torch.Tensor:
    return x

@use_kernelized_func(identity)
class LayerUsingIdentity(nn.Module):
    def forward(self, x: torch.Tensor) -> torch.Tensor:
        return identity(x)

```

### use_kernel_func_from_hub[[kernels.use_kernel_func_from_hub]]

#### kernels.use_kernel_func_from_hub[[kernels.use_kernel_func_from_hub]]

```python
kernels.use_kernel_func_from_hub(func_name: str)
```

[Source](https://github.com/huggingface/kernels/blob/main/kernels/src/kernels/layer/func.py#L198)

**Parameters:**

func_name (`str`) : The name of the function name to use for kernel lookup in registered mappings.

**Returns:** `Callable`

A decorator function that can be applied to layer classes.

Decorator that makes a function extensible using the specified function name.

This is a decorator factory that returns a decorator which prepares a function to use kernels from the
Hugging Face Hub.

The function will be exposed as an instance of `torch.nn.Module` in which
the function is called in `forward`. For the function to be properly
kernelized, it **must** be a member of another `torch.nn.Module` that is
part of the model (see the example).

> [!WARNING]
> `use_kernel_func_from_hub` is deprecated and will be removed in kernels 0.17.
> Use [use_kernel_forward_from_hub()](/docs/kernels/main/en/api/layers#kernels.use_kernel_forward_from_hub) instead.

Example:
```python
import torch
import torch.nn as nn

from kernels import use_kernel_func_from_hub
from kernels import Mode, kernelize

@use_kernel_func_from_hub("my_custom_func")
def my_custom_func(x: torch.Tensor):
    # Original implementation
    return x

class MyModel(torch.nn.Module):
    def __init__(self):
        super().__init__()
        self.fn = my_custom_func

    def forward(self, x):
        return self.fn(x)

model = MyModel()

# The layer can now be kernelized:
# model = kernelize(model, mode=Mode.TRAINING | Mode.TORCH_COMPILE, device="cuda")
```

### use_kernelized_func[[kernels.use_kernelized_func]]

#### kernels.use_kernelized_func[[kernels.use_kernelized_func]]

```python
kernels.use_kernelized_func(*args: Callable)
```

[Source](https://github.com/huggingface/kernels/blob/main/kernels/src/kernels/layer/layer.py#L344)

**Parameters:**

- ***args** (`Callable`) : Kernel functions to attach to the module.

**Returns:** `Callable`

A decorator function that can be applied to modules.

This decorator attaches the target function within the module as a plain
attribute (not as a submodule). This makes the function visible to
[kernelize()](/docs/kernels/main/en/api/layers#kernels.kernelize).

Example:
```python
import torch
import torch.nn as nn

from kernels import use_kernel_forward_from_hub
from kernels import use_kernelized_func
from kernels import Mode, kernelize

# Use on a function (converts the function to `nn.Module`). The function
# can then be replaced with a layer mapping for `MyCustomLayer`.
@use_kernel_forward_from_hub("MyCustomLayer")
def identity(x: torch.Tensor) -> torch.Tensor:
    return x

@use_kernelized_func(identity)
class LayerUsingIdentity(nn.Module):
    def forward(self, x: torch.Tensor) -> torch.Tensor:
        return identity(x)

model = LayerUsingIdentity()

# The layer can now be kernelized:
# model = kernelize(model, mode=Mode.TRAINING | Mode.TORCH_COMPILE, device="cuda")
```

### replace_kernel_forward_from_hub[[kernels.replace_kernel_forward_from_hub]]

#### kernels.replace_kernel_forward_from_hub[[kernels.replace_kernel_forward_from_hub]]

```python
kernels.replace_kernel_forward_from_hub(layer_name: str)
```

[Source](https://github.com/huggingface/kernels/blob/main/kernels/src/kernels/layer/layer.py#L249)

Function that prepares a layer class to use kernels from the Hugging Face Hub.

It is recommended to use [use_kernel_forward_from_hub()](/docs/kernels/main/en/api/layers#kernels.use_kernel_forward_from_hub) decorator instead.
This function should only be used as a last resort to extend third-party layers,
it is inherently fragile since the member variables and `forward` signature
of such a layer can change.

Example:
```python
from kernels import replace_kernel_forward_from_hub
import torch.nn as nn

replace_kernel_forward_from_hub(nn.LayerNorm, "LayerNorm")
```

## Registering kernel mappings

### use_kernel_mapping[[kernels.use_kernel_mapping]]

#### kernels.use_kernel_mapping[[kernels.use_kernel_mapping]]

```python
kernels.use_kernel_mapping(mapping: dict[str, dict[Device | str, RepositoryProtocol | dict[Mode, RepositoryProtocol]]], inherit_mapping: bool = True)
```

[Source](https://github.com/huggingface/kernels/blob/main/kernels/src/kernels/layer/kernelize.py#L17)

**Parameters:**

mapping (`dict[str, dict[Union[Device, str], Union[LayerRepositoryProtocol, dict[Mode, LayerRepositoryProtocol]]]]`) : The kernel mapping to apply. Maps layer names to device-specific kernel configurations.

inherit_mapping (`bool`, *optional*, defaults to `True`) : When `True`, the current mapping will be extended by `mapping` inside the context. When `False`, only `mapping` is used inside the context.

**Returns:**

Context manager that handles the temporary kernel mapping.

Context manager that sets a kernel mapping for the duration of the context.

This function allows temporary kernel mappings to be applied within a specific context, enabling different
kernel configurations for different parts of your code.

Example:
```python
import torch
import torch.nn as nn
from torch.nn import functional as F

from kernels import use_kernel_forward_from_hub
from kernels import use_kernel_mapping, LayerRepository, Device
from kernels import Mode, kernelize

# Define a mapping
mapping = {
    "SiluAndMul": {
        "cuda": LayerRepository(
            repo_id="kernels-community/activation",
            layer_name="SiluAndMul",
            version=1
        )
    }
}

@use_kernel_forward_from_hub("SiluAndMul")
class SiluAndMul(nn.Module):
    def forward(self, x: torch.Tensor) -> torch.Tensor:
        d = x.shape[-1] // 2
        return F.silu(x[..., :d]) * x[..., d:]

model = SiluAndMul()

# Use the mapping for the duration of the context.
with use_kernel_mapping(mapping):
    # kernelize uses the temporary mapping
    model = kernelize(model, mode=Mode.TRAINING | Mode.TORCH_COMPILE, device="cuda")

# Outside the context, original mappings are restored
```

### register_kernel_mapping[[kernels.register_kernel_mapping]]

#### kernels.register_kernel_mapping[[kernels.register_kernel_mapping]]

```python
kernels.register_kernel_mapping(mapping: dict[str, dict[Device | str, RepositoryProtocol | dict[Mode, RepositoryProtocol]]], inherit_mapping: bool = True)
```

[Source](https://github.com/huggingface/kernels/blob/main/kernels/src/kernels/layer/kernelize.py#L97)

**Parameters:**

mapping (`dict[str, dict[Union[Device, str], Union[RepositoryProtocol, dict[Mode, RepositoryProtocol]]]]`) : The kernel mapping to register globally. Maps layer names to device-specific kernels. The mapping can specify different kernels for different modes (training, inference, etc.).

inherit_mapping (`bool`, *optional*, defaults to `True`) : When `True`, the current mapping will be extended by `mapping`. When `False`, the existing mappings are erased before adding `mapping`.

Register a global mapping between layer names and their corresponding kernel implementations.

This function allows you to register a mapping between a layer name and the corresponding kernel(s) to use,
depending on the device and mode. This should be used in conjunction with [kernelize()](/docs/kernels/main/en/api/layers#kernels.kernelize).

Example:
```python
from kernels import LayerRepository, register_kernel_mapping, Mode

# Simple mapping for a single kernel per device
kernel_layer_mapping = {
    "LlamaRMSNorm": {
        "cuda": LayerRepository(
            repo_id="kernels-community/layer_norm",
            layer_name="LlamaRMSNorm",
            version=1,
        ),
    },
}
register_kernel_mapping(kernel_layer_mapping)

# Advanced mapping with mode-specific kernels
advanced_mapping = {
    "MultiHeadAttention": {
        "cuda": {
            Mode.TRAINING: LayerRepository(
                repo_id="kernels-community/training-kernels",
                layer_name="TrainingAttention",
                version=1,
            ),
            Mode.INFERENCE: LayerRepository(
                repo_id="kernels-community/inference-kernels",
                layer_name="FastAttention",
                version=1,
            ),
        }
    }
}
register_kernel_mapping(advanced_mapping)
```

## Kernelizing a model

### kernelize[[kernels.kernelize]]

#### kernels.kernelize[[kernels.kernelize]]

```python
kernels.kernelize(model: 'nn.Module', mode: Mode, device: str | 'torch.device' | None = None, use_fallback: bool = True)
```

[Source](https://github.com/huggingface/kernels/blob/main/kernels/src/kernels/layer/kernelize.py#L175)

**Parameters:**

model (`nn.Module`) : The PyTorch model to kernelize.

mode ([Mode](/docs/kernels/main/en/api/layers#kernels.Mode)) : The mode that the kernel is going to be used in. For example, `Mode.TRAINING | Mode.TORCH_COMPILE` kernelizes the model for training with `torch.compile`.

device (`Union[str, torch.device]`, *optional*) : The device type to load kernels for. Supported device types are: "cuda", "mps", "npu", "rocm", "tpu", "xpu". The device type will be inferred from the model parameters when not provided.

use_fallback (`bool`, *optional*, defaults to `True`) : Whether to use the original forward method of modules when no compatible kernel could be found. If set to `False`, an exception will be raised in such cases.

**Returns:** `nn.Module`

The kernelized model with optimized kernel implementations.

Replace layer forward methods with optimized kernel implementations.

This function iterates over all modules in the model and replaces the `forward` method of extensible layers
for which kernels are registered using [register_kernel_mapping()](/docs/kernels/main/en/api/layers#kernels.register_kernel_mapping) or [use_kernel_mapping()](/docs/kernels/main/en/api/layers#kernels.use_kernel_mapping).

Example:
```python
import torch
import torch.nn as nn

from kernels import kernelize, Mode, use_kernel_mapping, LayerRepository
from kernels import use_kernel_forward_from_hub

@use_kernel_forward_from_hub("SiluAndMul")
class SiluAndMul(nn.Module):
    def forward(self, x: torch.Tensor) -> torch.Tensor:
        d = x.shape[-1] // 2
        return F.silu(x[..., :d]) * x[..., d:]

mapping = {
    "SiluAndMul": {
        "cuda": LayerRepository(
            repo_id="kernels-community/activation",
            layer_name="SiluAndMul",
            version=1,
        )
    }
}

# Create and kernelize a model
model = nn.Sequential(
    nn.Linear(1024, 2048, device="cuda"),
    SiluAndMul(),
)

# Kernelize for inference
with use_kernel_mapping(mapping):
    kernelized_model = kernelize(model, mode=Mode.TRAINING | Mode.TORCH_COMPILE)
```

## Classes

### Device[[kernels.Device]]

#### kernels.Device[[kernels.Device]]

```python
kernels.Device(type: str, properties: kernels.layer.device.CUDAProperties | kernels.layer.device.ROCMProperties | None = None)
```

[Source](https://github.com/huggingface/kernels/blob/main/kernels/src/kernels/layer/device.py#L106)

**Parameters:**

type (`str`) : The device type (e.g., "cuda", "mps", "npu", "rocm", "xpu").

properties ([CUDAProperties](/docs/kernels/main/en/api/layers#kernels.CUDAProperties), *optional*) : Device-specific properties. Currently only [CUDAProperties](/docs/kernels/main/en/api/layers#kernels.CUDAProperties) is supported for CUDA devices.

Represents a compute device with optional properties.

This class encapsulates device information including device type and optional device-specific properties
like CUDA capabilities.

Example:
```python
from kernels import Device, CUDAProperties

# Basic CUDA device
cuda_device = Device(type="cuda")

# CUDA device with specific capability requirements
cuda_device_with_props = Device(
    type="cuda",
    properties=CUDAProperties(min_capability=75, max_capability=90)
)

# MPS device for Apple Silicon
mps_device = Device(type="mps")

# XPU device (e.g., Intel(R) Data Center GPU Max 1550)
xpu_device = Device(type="xpu")

# NPU device (Huawei Ascend)
npu_device = Device(type="npu")
```

#### validate[[kernels.Device.validate]]

```python
validate()
```

[Source](https://github.com/huggingface/kernels/blob/main/kernels/src/huggingface_hub/dataclasses.py#L247)

Run class validators on the instance.

### CUDAProperties[[kernels.CUDAProperties]]

#### kernels.CUDAProperties[[kernels.CUDAProperties]]

```python
kernels.CUDAProperties(min_capability: int, max_capability: int)
```

[Source](https://github.com/huggingface/kernels/blob/main/kernels/src/kernels/layer/device.py#L8)

**Parameters:**

min_capability (`int`) : Minimum CUDA compute capability required (e.g., 75 for compute capability 7.5).

max_capability (`int`) : Maximum CUDA compute capability supported (e.g., 90 for compute capability 9.0).

CUDA-specific device properties for capability-based kernel selection.

This class defines CUDA compute capability constraints for kernel selection, allowing kernels to specify
minimum and maximum CUDA compute capabilities they support.

Example:
```python
from kernels import CUDAProperties, Device

# Define CUDA properties for modern GPUs (compute capability 7.5 to 9.0)
cuda_props = CUDAProperties(min_capability=75, max_capability=90)

# Create a device with these properties
device = Device(type="cuda", properties=cuda_props)
```

Note:
CUDA compute capabilities are represented as integers where the major and minor versions are concatenated.
For example, compute capability 7.5 is represented as 75, and 8.6 is represented as 86.

#### validate[[kernels.CUDAProperties.validate]]

```python
validate()
```

[Source](https://github.com/huggingface/kernels/blob/main/kernels/src/huggingface_hub/dataclasses.py#L247)

Run class validators on the instance.

### ROCMProperties[[kernels.ROCMProperties]]

#### kernels.ROCMProperties[[kernels.ROCMProperties]]

```python
kernels.ROCMProperties(min_capability: int, max_capability: int)
```

[Source](https://github.com/huggingface/kernels/blob/main/kernels/src/kernels/layer/device.py#L57)

**Parameters:**

min_capability (`int`) : Minimum ROCM compute capability required (e.g., 75 for compute capability 7.5).

max_capability (`int`) : Maximum ROCM compute capability supported (e.g., 90 for compute capability 9.0).

ROCM-specific device properties for capability-based kernel selection.

This class defines ROCM compute capability constraints for kernel selection, allowing kernels to specify
minimum and maximum ROCM compute capabilities they support.

Example:
```python
from kernels import ROCMProperties, Device

# Define ROCM properties for modern GPUs (compute capability 7.5 to 9.0)
rocm_props = ROCMProperties(min_capability=75, max_capability=90)

# Create a device with these properties
device = Device(type="rocm", properties=rocm_props)
```

Note:
ROCM compute capabilities are represented as integers where the major and minor versions are concatenated.
For example, compute capability 7.5 is represented as 75, and 8.6 is represented as 86.

#### validate[[kernels.ROCMProperties.validate]]

```python
validate()
```

[Source](https://github.com/huggingface/kernels/blob/main/kernels/src/huggingface_hub/dataclasses.py#L247)

Run class validators on the instance.

### Mode[[kernels.Mode]]

#### kernels.Mode[[kernels.Mode]]

```python
kernels.Mode(value, names = None, module = None, qualname = None, type = None, start = 1)
```

[Source](https://github.com/huggingface/kernels/blob/main/kernels/src/kernels/layer/mode.py#L4)

**Parameters:**

INFERENCE : The kernel is used for inference.

TRAINING : The kernel is used for training.

TORCH_COMPILE : The kernel is used with `torch.compile`.

FALLBACK : In a kernel mapping, this kernel is used when no other mode matches.

Kernelize mode

The `Mode` flag is used by [kernelize()](/docs/kernels/main/en/api/layers#kernels.kernelize) to select kernels for the given mode. Mappings can be registered for
specific modes.

Note:
Different modes can be combined. For instance, `INFERENCE | TORCH_COMPILE` should be used for layers that
are used for inference *with* `torch.compile`.

### FuncRepository[[kernels.FuncRepository]]

#### kernels.FuncRepository[[kernels.FuncRepository]]

```python
kernels.FuncRepository(repo_id: str, func_name: str, revision: str | None = None, version: int | None = None, trust_remote_code: bool | list[str] = False)
```

[Source](https://github.com/huggingface/kernels/blob/main/kernels/src/kernels/layer/func.py#L28)

**Parameters:**

repo_id (`str`) : The Hub repository containing the layer.

func_name (`str`) : The name of the function within the kernel repository.

revision (`str`, *optional*) : The specific revision (branch, tag, or commit) to download. Cannot be used together with `version`.

version (`int`, *optional*) : The kernel version to download. Cannot be used together with `revision`. Either `version` or `revision` must be specified.

Repository and name of a function for kernel mapping.

> [!WARNING]
> `FuncRepository` is deprecated and will be removed in kernels 0.17.
> Use [LayerRepository](/docs/kernels/main/en/api/layers#kernels.LayerRepository) instead.

Example:
```python
from kernels import FuncRepository

# Reference a specific layer by revision
layer_repo = FuncRepository(
    repo_id="kernels-community/activation",
    func_name="silu_and_mul",
    revision="main",
)

# Reference a layer by version
layer_repo_versioned = FuncRepository(
    repo_id="kernels-community/relu",
    func_name="relu",
    version=1
)
```

### LayerRepository[[kernels.LayerRepository]]

#### kernels.LayerRepository[[kernels.LayerRepository]]

```python
kernels.LayerRepository(repo_id: str, layer_name: str, revision: str | None = None, version: int | None = None, trust_remote_code: bool | list[str] = False)
```

[Source](https://github.com/huggingface/kernels/blob/main/kernels/src/kernels/layer/layer.py#L35)

**Parameters:**

repo_id (`str`) : The Hub repository containing the layer.

layer_name (`str`) : The name of the layer within the kernel repository.

revision (`str`, *optional*) : The specific revision (branch, tag, or commit) to download. Cannot be used together with `version`.

version (`int`, *optional*) : The kernel version to download. Cannot be used together with `revision`. Either `version` or `revision` must be specified.

trust_remote_code (`bool | list[str]`, *optional*, defaults to `False`) : Whether to allow loading kernels from untrusted organisations. A list of signing identities can be provided for future verification support; until then it warns and falls back to the default trust check.

Repository and name of a layer for kernel mapping.

Example:
```python
from kernels import LayerRepository

# Reference a specific layer by version
layer_repo = LayerRepository(
    repo_id="kernels-community/activation",
    layer_name="SiluAndMul",
    version=1,
)
```

### LocalFuncRepository[[kernels.LocalFuncRepository]]

#### kernels.LocalFuncRepository[[kernels.LocalFuncRepository]]

```python
kernels.LocalFuncRepository(repo_path: Path, func_name: str)
```

[Source](https://github.com/huggingface/kernels/blob/main/kernels/src/kernels/layer/func.py#L137)

**Parameters:**

repo_path (`Path`) : The local repository containing the layer.

func_name (`str`) : The name of the function within the kernel repository.

Repository and function name from a local directory for kernel mapping.

> [!WARNING]
> `LocalFuncRepository` is deprecated and will be removed in kernels 0.17.
> Use [LocalLayerRepository](/docs/kernels/main/en/api/layers#kernels.LocalLayerRepository) instead.

Example:
```python
from pathlib import Path

from kernels import LocalFuncRepository

# Reference a specific layer by revision
layer_repo = LocalFuncRepository(
    repo_path=Path("/home/daniel/kernels/activation"),
    func_name="silu_and_mul",
)
```

### LocalLayerRepository[[kernels.LocalLayerRepository]]

#### kernels.LocalLayerRepository[[kernels.LocalLayerRepository]]

```python
kernels.LocalLayerRepository(repo_path: Path, layer_name: str)
```

[Source](https://github.com/huggingface/kernels/blob/main/kernels/src/kernels/layer/layer.py#L131)

**Parameters:**

repo_path (`Path`) : The local repository containing the layer.

layer_name (`str`) : The name of the layer within the kernel repository.

Repository from a local directory for kernel mapping.

Example:
```python
from pathlib import Path

from kernels import LocalLayerRepository

# Reference a specific layer by revision
layer_repo = LocalLayerRepository(
    repo_path=Path("/home/daniel/kernels/activation"),
    layer_name="SiluAndMul",
)
```

### LockedFuncRepository[[kernels.LockedFuncRepository]]

#### kernels.LockedFuncRepository[[kernels.LockedFuncRepository]]

```python
kernels.LockedFuncRepository(repo_id: str, lockfile: pathlib.Path | None = None, func_name: str, trust_remote_code: bool | list[str] = False)
```

[Source](https://github.com/huggingface/kernels/blob/main/kernels/src/kernels/layer/func.py#L257)

**Parameters:**

repo_id (`str`) : The Hub repository containing the function.

lockfile (`Path`, *optional*) : Path to the lockfile. If not provided, the lockfile will be inferred from the caller's context.

func_name (`str`) : The name of the function within the kernel repository.

trust_remote_code (`bool`, *optional*, defaults to `False`) : Whether to allow loading kernels from untrusted organisations.

Repository and name of a function.

In contrast to `FuncRepository`, this class uses repositories that
are locked inside a project.

> [!WARNING]
> `LockedFuncRepository` is deprecated and will be removed in kernels 0.17.
> Use [LockedLayerRepository](/docs/kernels/main/en/api/layers#kernels.LockedLayerRepository) instead.

### LockedLayerRepository[[kernels.LockedLayerRepository]]

#### kernels.LockedLayerRepository[[kernels.LockedLayerRepository]]

```python
kernels.LockedLayerRepository(repo_id: str, lockfile: Path | None = None, layer_name: str, trust_remote_code: bool | list[str] = False)
```

[Source](https://github.com/huggingface/kernels/blob/main/kernels/src/kernels/layer/layer.py#L182)

Repository and name of a layer.

In contrast to `LayerRepository`, this class uses repositories that
are locked inside a project.
