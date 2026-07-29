# Kernels API Reference

## Main Functions

### get_kernel[[kernels.get_kernel]]

- **repo_id** (*str*) --
  The Hub repository containing the kernel.
- **revision** (*str*, *optional*) --
  The specific revision (branch, tag, or commit) to download. Cannot be used together with *version*.
- **version** (*int*, *optional*) --
  The kernel version to download. Cannot be used together with *revision*.
  Either *version* or *revision* must be specified.
- **backend** (*str*, *optional*) --
  The backend to load the kernel for. Can only be *cpu* or the backend that Torch is compiled for.
  The backend will be detected automatically if not provided.
- **user_agent** (*Union[str, dict]*, *optional*) --
  The *user_agent* info to pass to *snapshot_download()* for internal telemetry.
- **trust_remote_code** (*bool | list[str]*, *optional*, defaults to *False*) --
  Whether to allow loading kernels from untrusted organisations. When `False`,
  only kernels from trusted organisations are allowed. When `True`, all
  repositories are allowed. A list of strings will be used to verify signing
  identities in a future release; for now it emits a warning and falls
  back to the default trust check.*ModuleType*The imported kernel module.

Load a kernel from the kernel hub.

This function downloads a kernel to the local Hugging Face Hub cache directory (if it was not downloaded before)
and then loads the kernel.

Example:
```python
import torch
from kernels import get_kernel

activation = get_kernel("kernels-community/relu", version=1)
x = torch.randn(10, 20, device="cuda")
out = torch.empty_like(x)
result = activation.relu(out, x)
```

### get_local_kernel[[kernels.get_local_kernel]]

- **repo_path** (`Path`) --
  The local path to the kernel repository.
- **backend** (`str`, *optional*) --
  The backend to load the kernel for. Can only be `cpu` or the backend that Torch is compiled for.
  The backend will be detected automatically if not provided.`ModuleType`The imported kernel module.

Import a kernel from a local kernel repository path.

### has_kernel[[kernels.has_kernel]]

- **repo_id** (`str`) --
  The Hub repository containing the kernel.
- **revision** (`str`, *optional*) --
  The specific revision (branch, tag, or commit) to download. Cannot be used together with `version`.
- **version** (`int`, *optional*) --
  The kernel version to download. Cannot be used together with `revision`.
  Either `version` or `revision` must be specified.
- **backend** (`str`, *optional*) --
  The backend to load the kernel for. Can only be `cpu` or the backend that Torch is compiled for.
  The backend will be detected automatically if not provided.`bool``True` if a kernel is available for the current environment.

Check whether a kernel build exists for the current environment (Torch version and compute framework).

### get_kernel_variants[[kernels.get_kernel_variants]]

- **repo_id** (`str`) --
  The Hub repository containing the kernel.
- **revision** (`str`, *optional*) --
  The specific revision (branch, tag, or commit) to inspect. Cannot be used together with `version`.
- **version** (`int`, *optional*) --
  The kernel version to inspect. Cannot be used together with `revision`.
  Either `version` or `revision` must be specified.
- **backend** (`str`, *optional*) --
  The backend to resolve variants for. Can only be `cpu` or the backend that Torch is compiled for.
  The backend will be detected automatically if not provided.`list[Decision]`One `VariantAccepted` or `VariantRejected` per build variant
in the repository, compatible variants first.

Resolve all build variants of a kernel against the current environment.

The decisions are sorted with compatible variants first, the most preferred
variant leading.

Example:
```python
from kernels import get_kernel_variants, VariantAccepted

for decision in get_kernel_variants("kernels-community/activation", version=1):
    name = decision.variant.variant_str
    if isinstance(decision, VariantAccepted):
        print(f"{name}: compatible")
    else:
        print(f"{name}: rejected ({decision.reason})")

```

### get_loaded_kernels[[kernels.get_loaded_kernels]]

`list[LoadedKernel]`One [LoadedKernel](/docs/kernels/main/en/api/kernels#kernels.LoadedKernel) per distinct kernel variant path
loaded in this process.

Return a snapshot of every kernel that has been loaded into the current process.

The returned list is a new list; mutating it does not affect the registry.

Example:
```python
from kernels import get_kernel, get_loaded_kernels

get_kernel("kernels-community/activation", version=1)
for loaded in get_loaded_kernels():
    print(loaded.metadata.name, loaded.repo_info)
```

## Loading locked kernels

### load_kernel[[kernels.load_kernel]]

- **repo_id** (`str`) --
  The Hub repository containing the kernel.
- **lockfile** (`Path`, *optional*) --
  Path to the lockfile. If not provided, the lockfile will be loaded from the caller's package metadata.
- **backend** (`str`, *optional*) --
  The backend to load the kernel for. Can only be `cpu` or the backend that Torch is compiled for.
  The backend will be detected automatically if not provided.
- **revision** (`str`, *optional*) --
  The specific revision (branch, tag, or commit) to download. Cannot be used together with `version`.`ModuleType`The imported kernel module.

Get a pre-downloaded, locked kernel.

If `lockfile` is not specified, the lockfile will be loaded from the caller's package metadata.

### get_locked_kernel[[kernels.get_locked_kernel]]

- **repo_id** (`str`) --
  The Hub repository containing the kernel.
- **local_files_only** (`bool`, *optional*, defaults to `False`) --
  Whether to only use local files and not download from the Hub.`ModuleType`The imported kernel module.

Get a kernel using a lock file.

## Classes

### LoadedKernel[[kernels.LoadedKernel]]

This dataclass provides information about a loaded kernel:

- `metadata` (`Metadata`): kernel metadata.
- `module` (`ModuleType`): the imported kernel module.
- `repo_info` (`kernels.utils.RepoInfo | None`): populated only for
  kernels loaded via `get_kernel`. Loaders that work from a local path
  (`get_local_kernel`) or a lockfile (`get_locked_kernel`, `load_kernel`)
  leave this as `None`.

The metadata includes the following properties that describe a kernel:

- `id` (`str`): kernel identifier that is unique to the kernel version + backend.
- `name` (`str`): the name of the kernel.
- `version` (`int`): the version of the kernel.
- `license` (`str`): the license of the kernel.
- `upstream` (`str | None`): the original upstream repository of the kernel.
- `source` (`str | None`): the kernel-builder formatted source repository.
- `python_depends` (`list[str]`): required Python dependencies.
- `backend`: information about the kernel's backend.

### RepoInfo[[kernels.RepoInfo]]

This dataclass stores the origin of the kernel.

The following fields are available:

- `repo_id` (`str`): the Hub repository containing the kernel.
- `revision` (`str`): the specific revision of the kernel.
