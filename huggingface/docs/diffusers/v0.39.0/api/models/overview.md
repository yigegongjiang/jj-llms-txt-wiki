# Models

🤗 Diffusers provides pretrained models for popular algorithms and modules to create custom diffusion systems. The primary function of models is to denoise an input sample as modeled by the distribution  \\(p_{\theta}(x_{t-1}|x_{t})\\).

All models are built from the base [ModelMixin](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin) class which is a [`torch.nn.Module`](https://pytorch.org/docs/stable/generated/torch.nn.Module.html) providing basic functionality for saving and loading models, locally and from the Hugging Face Hub.

## ModelMixin[[diffusers.ModelMixin]]
#### diffusers.ModelMixin[[diffusers.ModelMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_utils.py#L232)

Base class for all models.

[ModelMixin](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin) takes care of storing the model configuration and provides methods for loading, downloading and
saving models.

- **config_name** (`str`) -- Filename to save a model to when calling [save_pretrained()](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin.save_pretrained).

compile_repeated_blocksdiffusers.ModelMixin.compile_repeated_blockshttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_utils.py#L1586[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]

Compiles *only* the frequently repeated sub-modules of a model (e.g. the Transformer layers) instead of
compiling the entire model. This technique—often called **regional compilation** (see the PyTorch recipe
https://docs.pytorch.org/tutorials/recipes/regional_compilation.html) can reduce end-to-end compile time
substantially, while preserving the runtime speed-ups you would expect from a full `torch.compile`.

The set of sub-modules to compile is discovered by the presence of **`_repeated_blocks`** attribute in the
model definition. Define this attribute on your model subclass as a list/tuple of class names (strings). Every
module whose class name matches will be compiled.

Once discovered, each matching sub-module is compiled by calling `submodule.compile(*args, **kwargs)`. Any
positional or keyword arguments you supply to `compile_repeated_blocks` are forwarded verbatim to
`torch.compile`.
#### dequantize[[diffusers.ModelMixin.dequantize]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_utils.py#L862)

Potentially dequantize the model in case it has been quantized by a quantization method that support
dequantization.
#### disable_gradient_checkpointing[[diffusers.ModelMixin.disable_gradient_checkpointing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_utils.py#L315)

Deactivates gradient checkpointing for the current model (may be referred to as *activation checkpointing* or
*checkpoint activations* in other frameworks).
#### disable_npu_flash_attention[[diffusers.ModelMixin.disable_npu_flash_attention]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_utils.py#L346)

disable npu flash attention from torch_npu
#### disable_xformers_memory_efficient_attention[[diffusers.ModelMixin.disable_xformers_memory_efficient_attention]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_utils.py#L429)

Disable memory efficient attention from [xFormers](https://facebookresearch.github.io/xformers/).
#### disable_xla_flash_attention[[diffusers.ModelMixin.disable_xla_flash_attention]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_utils.py#L376)

Disable the flash attention pallals kernel for torch_xla.
#### enable_gradient_checkpointing[[diffusers.ModelMixin.enable_gradient_checkpointing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_utils.py#L285)

Activates gradient checkpointing for the current model (may be referred to as *activation checkpointing* or
*checkpoint activations* in other frameworks).

**Parameters:**

gradient_checkpointing_func (`Callable`, *optional*) : The function to use for gradient checkpointing. If `None`, the default PyTorch checkpointing function is used (`torch.utils.checkpoint.checkpoint`).
#### enable_group_offload[[diffusers.ModelMixin.enable_group_offload]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_utils.py#L520)

Activates group offloading for the current model.

See [apply_group_offloading()](/docs/diffusers/v0.39.0/en/api/utilities#diffusers.hooks.apply_group_offloading) for more information.

Example:

```python
>>> from diffusers import CogVideoXTransformer3DModel

>>> transformer = CogVideoXTransformer3DModel.from_pretrained(
...     "THUDM/CogVideoX-5b", subfolder="transformer", torch_dtype=torch.bfloat16
... )

>>> transformer.enable_group_offload(
...     onload_device=torch.device("cuda"),
...     offload_device=torch.device("cpu"),
...     offload_type="leaf_level",
...     use_stream=True,
... )
```
#### enable_layerwise_casting[[diffusers.ModelMixin.enable_layerwise_casting]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_utils.py#L435)

Activates layerwise casting for the current model.

Layerwise casting is a technique that casts the model weights to a lower precision dtype for storage but
upcasts them on-the-fly to a higher precision dtype for computation. This process can significantly reduce the
memory footprint from model weights, but may lead to some quality degradation in the outputs. Most degradations
are negligible, mostly stemming from weight casting in normalization and modulation layers.

By default, most models in diffusers set the `_skip_layerwise_casting_patterns` attribute to ignore patch
embedding, positional embedding and normalization layers. This is because these layers are most likely
precision-critical for quality. If you wish to change this behavior, you can set the
`_skip_layerwise_casting_patterns` attribute to `None`, or call
[apply_layerwise_casting()](/docs/diffusers/v0.39.0/en/api/utilities#diffusers.hooks.apply_layerwise_casting) with custom arguments.

Example:

Using [enable_layerwise_casting()](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin.enable_layerwise_casting):

```python
>>> from diffusers import CogVideoXTransformer3DModel

>>> transformer = CogVideoXTransformer3DModel.from_pretrained(
...     "THUDM/CogVideoX-5b", subfolder="transformer", torch_dtype=torch.bfloat16
... )

>>> # Enable layerwise casting via the model, which ignores certain modules by default
>>> transformer.enable_layerwise_casting(storage_dtype=torch.float8_e4m3fn, compute_dtype=torch.bfloat16)
```

**Parameters:**

storage_dtype (`torch.dtype`) : The dtype to which the model should be cast for storage.

compute_dtype (`torch.dtype`) : The dtype to which the model weights should be cast during the forward pass.

skip_modules_pattern (`tuple[str, ...]`, *optional*) : A list of patterns to match the names of the modules to skip during the layerwise casting process. If set to `None`, default skip patterns are used to ignore certain internal layers of modules and PEFT layers.

skip_modules_classes (`tuple[Type[torch.nn.Module], ...]`, *optional*) : A list of module classes to skip during the layerwise casting process.

non_blocking (`bool`, *optional*, defaults to `False`) : If `True`, the weight casting operations are non-blocking.
#### enable_npu_flash_attention[[diffusers.ModelMixin.enable_npu_flash_attention]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_utils.py#L339)

Enable npu flash attention from torch_npu
#### enable_xformers_memory_efficient_attention[[diffusers.ModelMixin.enable_xformers_memory_efficient_attention]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_utils.py#L397)

Enable memory efficient attention from [xFormers](https://facebookresearch.github.io/xformers/).

When this option is enabled, you should observe lower GPU memory usage and a potential speed up during
inference. Speed up during training is not guaranteed.

> [!WARNING] > ⚠️ When memory efficient attention and sliced attention are both enabled, memory efficient
attention takes > precedent.

Examples:

```py
>>> import torch
>>> from diffusers import UNet2DConditionModel
>>> from xformers.ops import MemoryEfficientAttentionFlashAttentionOp

>>> model = UNet2DConditionModel.from_pretrained(
...     "stabilityai/stable-diffusion-2-1", subfolder="unet", torch_dtype=torch.float16
... )
>>> model = model.to("cuda")
>>> model.enable_xformers_memory_efficient_attention(attention_op=MemoryEfficientAttentionFlashAttentionOp)
```

**Parameters:**

attention_op (`Callable`, *optional*) : Override the default `None` operator for use as `op` argument to the [`memory_efficient_attention()`](https://facebookresearch.github.io/xformers/components/ops.html#xformers.ops.memory_efficient_attention) function of xFormers.
#### enable_xla_flash_attention[[diffusers.ModelMixin.enable_xla_flash_attention]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_utils.py#L370)

Enable the flash attention pallals kernel for torch_xla.
#### from_pretrained[[diffusers.ModelMixin.from_pretrained]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_utils.py#L874)

Instantiate a pretrained PyTorch model from a pretrained model configuration.

The model is set in evaluation mode - `model.eval()` - by default, and dropout modules are deactivated. To
train the model, set it back in training mode with `model.train()`.

> [!TIP] > To use private or [gated models](https://huggingface.co/docs/hub/models-gated#gated-models), log-in
with `hf > auth login`. You can also activate the special >
["offline-mode"](https://huggingface.co/diffusers/installation.html#offline-mode) to use this method in a >
firewalled environment.

Example:

```py
from diffusers import UNet2DConditionModel

unet = UNet2DConditionModel.from_pretrained("stable-diffusion-v1-5/stable-diffusion-v1-5", subfolder="unet")
```

If you get the error message below, you need to finetune the weights for your downstream task:

```bash
Some weights of UNet2DConditionModel were not initialized from the model checkpoint at stable-diffusion-v1-5/stable-diffusion-v1-5 and are newly initialized because the shapes did not match:
- conv_in.weight: found shape torch.Size([320, 4, 3, 3]) in the checkpoint and torch.Size([320, 9, 3, 3]) in the model instantiated
You should probably TRAIN this model on a down-stream task to be able to use it for predictions and inference.
```

**Parameters:**

pretrained_model_name_or_path (`str` or `os.PathLike`, *optional*) : Can be either:  - A string, the *model id* (for example `google/ddpm-celebahq-256`) of a pretrained model hosted on the Hub. - A path to a *directory* (for example `./my_model_directory`) containing the model weights saved with [save_pretrained()](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin.save_pretrained). 

cache_dir (`str | os.PathLike`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used.

torch_dtype (`torch.dtype`, *optional*) : Override the default `torch.dtype` and load the model with another dtype.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist.

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

output_loading_info (`bool`, *optional*, defaults to `False`) : Whether or not to also return a dictionary containing missing keys, unexpected keys and error messages.

local_files_only(`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to `True`, the model won't be downloaded from the Hub.

token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

from_flax (`bool`, *optional*, defaults to `False`) : Load the model weights from a Flax checkpoint save file.

subfolder (`str`, *optional*, defaults to `""`) : The subfolder location of a model file within a larger model repository on the Hub or locally.

mirror (`str`, *optional*) : Mirror source to resolve accessibility issues if you're downloading a model in China. We do not guarantee the timeliness or safety of the source, and you should refer to the mirror site for more information.

device_map (`int | str | torch.device` or `dict[str, int | str | torch.device]`, *optional*) : A map that specifies where each submodule should go. It doesn't need to be defined for each parameter/buffer name; once a given module name is inside, every submodule of it will be sent to the same device. Defaults to `None`, meaning that the model will be loaded on CPU.  Examples:  ```py >>> from diffusers import AutoModel >>> import torch  >>> # This works. >>> model = AutoModel.from_pretrained( ...     "stabilityai/stable-diffusion-xl-base-1.0", subfolder="unet", device_map="cuda" ... ) >>> # This also works (integer accelerator device ID). >>> model = AutoModel.from_pretrained( ...     "stabilityai/stable-diffusion-xl-base-1.0", subfolder="unet", device_map=0 ... ) >>> # Specifying a supported offloading strategy like "auto" also works. >>> model = AutoModel.from_pretrained( ...     "stabilityai/stable-diffusion-xl-base-1.0", subfolder="unet", device_map="auto" ... ) >>> # Specifying a dictionary as `device_map` also works. >>> model = AutoModel.from_pretrained( ...     "stabilityai/stable-diffusion-xl-base-1.0", ...     subfolder="unet", ...     device_map={"": torch.device("cuda")}, ... ) ```  Set `device_map="auto"` to have 🤗 Accelerate automatically compute the most optimized `device_map`. For more information about each option see [designing a device map](https://huggingface.co/docs/accelerate/en/concept_guides/big_model_inference#the-devicemap). You can also refer to the [Diffusers-specific documentation](https://huggingface.co/docs/diffusers/main/en/training/distributed_inference#model-sharding) for more concrete examples.

max_memory (`Dict`, *optional*) : A dictionary device identifier for the maximum memory. Will default to the maximum memory available for each GPU and the available CPU RAM if unset.

offload_folder (`str` or `os.PathLike`, *optional*) : The path to offload weights if `device_map` contains the value `"disk"`.

offload_state_dict (`bool`, *optional*) : If `True`, temporarily offloads the CPU state dict to the hard drive to avoid running out of CPU RAM if the weight of the CPU state dict + the biggest shard of the checkpoint does not fit. Defaults to `True` when there is some disk offload.

low_cpu_mem_usage (`bool`, *optional*, defaults to `True` if torch version >= 1.9.0 else `False`) : Speed up model loading only loading the pretrained weights and not initializing the weights. This also tries to not use more than 1x model size in CPU memory (including peak memory) while loading the model. Only supported for PyTorch >= 1.9.0. If you are using an older version of PyTorch, setting this argument to `True` will raise an error.

variant (`str`, *optional*) : Load weights from a specified `variant` filename such as `"fp16"` or `"ema"`. This is ignored when loading `from_flax`.

use_safetensors (`bool`, *optional*, defaults to `None`) : If set to `None`, the `safetensors` weights are downloaded if they're available **and** if the `safetensors` library is installed. If set to `True`, the model is forcibly loaded from `safetensors` weights. If set to `False`, `safetensors` weights are not loaded.

disable_mmap ('bool', *optional*, defaults to 'False') : Whether to disable mmap when loading a Safetensors model. This option can perform better when the model is on a network mount or hard drive, which may not handle the seeky-ness of mmap very well.

use_flashpack (`bool`, *optional*, defaults to `False`) : If set to `True`, the model is loaded from `flashpack` weights.

flashpack_kwargs(`dict[str, Any]`, *optional*, defaults to `{}`) : Kwargs passed to [`flashpack.deserialization.assign_from_file`](https://github.com/fal-ai/flashpack/blob/f1aa91c5cd9532a3dbf5bcc707ab9b01c274b76c/src/flashpack/deserialization.py#L408-L422)
#### get_memory_footprint[[diffusers.ModelMixin.get_memory_footprint]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_utils.py#L2013)

Get the memory footprint of a model. This will return the memory footprint of the current model in bytes.
Useful to benchmark the memory footprint of the current model and design some tests. Solution inspired from the
PyTorch discussions: https://discuss.pytorch.org/t/gpu-memory-that-model-uses/56822/2

**Parameters:**

return_buffers (`bool`, *optional*, defaults to `True`) : Whether to return the size of the buffer tensors in the computation of the memory footprint. Buffers are tensors that do not require gradients and not registered as parameters. E.g. mean and std in batch norm layers. Please see: https://discuss.pytorch.org/t/what-pytorch-means-by-buffers/120266/2
#### num_parameters[[diffusers.ModelMixin.num_parameters]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_utils.py#L1949)

Get number of (trainable or non-embedding) parameters in the module.

Example:

```py
from diffusers import UNet2DConditionModel

model_id = "stable-diffusion-v1-5/stable-diffusion-v1-5"
unet = UNet2DConditionModel.from_pretrained(model_id, subfolder="unet")
unet.num_parameters(only_trainable=True)
859520964
```

**Parameters:**

only_trainable (`bool`, *optional*, defaults to `False`) : Whether or not to return only the number of trainable parameters.

exclude_embeddings (`bool`, *optional*, defaults to `False`) : Whether or not to return only the number of non-embedding parameters.

**Returns:**

``int``

The number of parameters.
#### reset_attention_backend[[diffusers.ModelMixin.reset_attention_backend]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_utils.py#L650)

Resets the attention backend for the model. Following calls to `forward` will use the environment default, if
set, or the torch native scaled dot product attention.
#### save_pretrained[[diffusers.ModelMixin.save_pretrained]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_utils.py#L669)

Save a model and its configuration file to a directory so that it can be reloaded using the
[from_pretrained()](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin.from_pretrained) class method.

**Parameters:**

save_directory (`str` or `os.PathLike`) : Directory to save a model and its configuration file to. Will be created if it doesn't exist.

is_main_process (`bool`, *optional*, defaults to `True`) : Whether the process calling this is the main process or not. Useful during distributed training and you need to call this function on all processes. In this case, set `is_main_process=True` only on the main process to avoid race conditions.

save_function (`Callable`) : The function to use to save the state dictionary. Useful during distributed training when you need to replace `torch.save` with another method. Can be configured with the environment variable `DIFFUSERS_SAVE_MODE`.

safe_serialization (`bool`, *optional*, defaults to `True`) : Whether to save the model using `safetensors` or the traditional PyTorch way with `pickle`.

variant (`str`, *optional*) : If specified, weights are saved in the format `pytorch_model.<variant>.bin`.

max_shard_size (`int` or `str`, defaults to `"10GB"`) : The maximum size for a checkpoint before being sharded. Checkpoints shard will then be each of size lower than this size. If expressed as a string, needs to be digits followed by a unit (like `"5GB"`). If expressed as an integer, the unit is bytes. Note that this limit will be decreased after a certain period of time (starting from Oct 2024) to allow users to upgrade to the latest version of `diffusers`. This is to establish a common default size for this argument across different libraries in the Hugging Face ecosystem (`transformers`, and `accelerate`, for example).

push_to_hub (`bool`, *optional*, defaults to `False`) : Whether or not to push your model to the Hugging Face Hub after saving it. You can specify the repository you want to push to with `repo_id` (will default to the name of `save_directory` in your namespace).

kwargs (`dict[str, Any]`, *optional*) : Additional keyword arguments passed along to the [push_to_hub()](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.utils.PushToHubMixin.push_to_hub) method.
#### set_attention_backend[[diffusers.ModelMixin.set_attention_backend]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_utils.py#L588)

Set the attention backend for the model.

**Parameters:**

backend (`str`) : The name of the backend to set. Must be one of the available backends defined in `AttentionBackendName`. Available backends can be found in `diffusers.attention_dispatch.AttentionBackendName`. Defaults to torch native scaled dot product attention as backend.
#### set_use_npu_flash_attention[[diffusers.ModelMixin.set_use_npu_flash_attention]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_utils.py#L323)

Set the switch for the npu flash attention.

## PushToHubMixin[[diffusers.utils.PushToHubMixin]]

#### diffusers.utils.PushToHubMixin[[diffusers.utils.PushToHubMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/hub_utils.py#L483)

A Mixin to push a model, scheduler, or pipeline to the Hugging Face Hub.

push_to_hubdiffusers.utils.PushToHubMixin.push_to_hubhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/hub_utils.py#L518[{"name": "repo_id", "val": ": str"}, {"name": "commit_message", "val": ": str | None = None"}, {"name": "private", "val": ": bool | None = None"}, {"name": "token", "val": ": str | None = None"}, {"name": "create_pr", "val": ": bool = False"}, {"name": "safe_serialization", "val": ": bool = True"}, {"name": "variant", "val": ": str | None = None"}, {"name": "subfolder", "val": ": str | None = None"}]- **repo_id** (`str`) --
  The name of the repository you want to push your model, scheduler, or pipeline files to. It should
  contain your organization name when pushing to an organization. `repo_id` can also be a path to a local
  directory.
- **commit_message** (`str`, *optional*) --
  Message to commit while pushing. Default to `"Upload {object}"`.
- **private** (`bool`, *optional*) --
  Whether to make the repo private. If `None` (default), the repo will be public unless the
  organization's default is private. This value is ignored if the repo already exists.
- **token** (`str`, *optional*) --
  The token to use as HTTP bearer authorization for remote files. The token generated when running `hf
  auth login` (stored in `~/.huggingface`).
- **create_pr** (`bool`, *optional*, defaults to `False`) --
  Whether or not to create a PR with the uploaded files or directly commit.
- **safe_serialization** (`bool`, *optional*, defaults to `True`) --
  Whether or not to convert the model weights to the `safetensors` format.
- **variant** (`str`, *optional*) --
  If specified, weights are saved in the format `pytorch_model..bin`.0

Upload model, scheduler, or pipeline files to the 🤗 Hugging Face Hub.

Examples:

```python
from diffusers import UNet2DConditionModel

unet = UNet2DConditionModel.from_pretrained("stabilityai/stable-diffusion-2", subfolder="unet")

# Push the `unet` to your namespace with the name "my-finetuned-unet".
unet.push_to_hub("my-finetuned-unet")

# Push the `unet` to an organization with the name "my-finetuned-unet".
unet.push_to_hub("your-org/my-finetuned-unet")
```

**Parameters:**

repo_id (`str`) : The name of the repository you want to push your model, scheduler, or pipeline files to. It should contain your organization name when pushing to an organization. `repo_id` can also be a path to a local directory.

commit_message (`str`, *optional*) : Message to commit while pushing. Default to `"Upload {object}"`.

private (`bool`, *optional*) : Whether to make the repo private. If `None` (default), the repo will be public unless the organization's default is private. This value is ignored if the repo already exists.

token (`str`, *optional*) : The token to use as HTTP bearer authorization for remote files. The token generated when running `hf auth login` (stored in `~/.huggingface`).

create_pr (`bool`, *optional*, defaults to `False`) : Whether or not to create a PR with the uploaded files or directly commit.

safe_serialization (`bool`, *optional*, defaults to `True`) : Whether or not to convert the model weights to the `safetensors` format.

variant (`str`, *optional*) : If specified, weights are saved in the format `pytorch_model.<variant>.bin`.
