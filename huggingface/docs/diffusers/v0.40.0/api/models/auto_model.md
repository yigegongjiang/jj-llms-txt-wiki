# AutoModel

[AutoModel](/docs/diffusers/v0.40.0/en/api/models/auto_model#diffusers.AutoModel) automatically retrieves the correct model class from the checkpoint `config.json` file.

## AutoModel[[diffusers.AutoModel]]

#### diffusers.AutoModel[[diffusers.AutoModel]]

```python
diffusers.AutoModel(*args, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/auto_model.py#L27)

#### from_pretrained[[diffusers.AutoModel.from_pretrained]]

```python
from_pretrained(pretrained_model_or_path: str | os.PathLike | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/auto_model.py#L155)

**Parameters:**

pretrained_model_name_or_path (`str` or `os.PathLike`, *optional*) : Can be either:  - A string, the *model id* (for example `google/ddpm-celebahq-256`) of a pretrained model hosted on the Hub. - A path to a *directory* (for example `./my_model_directory`) containing the model weights saved with [save_pretrained()](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin.save_pretrained). 

cache_dir (`str | os.PathLike`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used.

dtype (`torch.dtype`, *optional*) : Override the default `torch.dtype` and load the model with another dtype.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist.

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

output_loading_info (`bool`, *optional*, defaults to `False`) : Whether or not to also return a dictionary containing missing keys, unexpected keys and error messages.

local_files_only(`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to `True`, the model won't be downloaded from the Hub.

token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

subfolder (`str`, *optional*, defaults to `""`) : The subfolder location of a model file within a larger model repository on the Hub or locally.

mirror (`str`, *optional*) : Mirror source to resolve accessibility issues if you're downloading a model in China. We do not guarantee the timeliness or safety of the source, and you should refer to the mirror site for more information.

device_map (`str` or `dict[str, int | str | torch.device]`, *optional*) : A map that specifies where each submodule should go. It doesn't need to be defined for each parameter/buffer name; once a given module name is inside, every submodule of it will be sent to the same device. Defaults to `None`, meaning that the model will be loaded on CPU.  Set `device_map="auto"` to have 🤗 Accelerate automatically compute the most optimized `device_map`. For more information about each option see [designing a device map](https://hf.co/docs/accelerate/main/en/usage_guides/big_modeling#designing-a-device-map).

max_memory (`Dict`, *optional*) : A dictionary device identifier for the maximum memory. Will default to the maximum memory available for each GPU and the available CPU RAM if unset.

offload_folder (`str` or `os.PathLike`, *optional*) : The path to offload weights if `device_map` contains the value `"disk"`.

offload_state_dict (`bool`, *optional*) : If `True`, temporarily offloads the CPU state dict to the hard drive to avoid running out of CPU RAM if the weight of the CPU state dict + the biggest shard of the checkpoint does not fit. Defaults to `True` when there is some disk offload.

low_cpu_mem_usage (`bool`, *optional*, defaults to `True` if torch version >= 1.9.0 else `False`) : Speed up model loading only loading the pretrained weights and not initializing the weights. This also tries to not use more than 1x model size in CPU memory (including peak memory) while loading the model. Only supported for PyTorch >= 1.9.0. If you are using an older version of PyTorch, setting this argument to `True` will raise an error.

variant (`str`, *optional*) : Load weights from a specified `variant` filename such as `"fp16"` or `"ema"`.

use_safetensors (`bool`, *optional*, defaults to `None`) : If set to `None`, the `safetensors` weights are downloaded if they're available **and** if the `safetensors` library is installed. If set to `True`, the model is forcibly loaded from `safetensors` weights. If set to `False`, `safetensors` weights are not loaded.

disable_mmap ('bool', *optional*, defaults to 'False') : Whether to disable mmap when loading a Safetensors model. This option can perform better when the model is on a network mount or hard drive, which may not handle the seeky-ness of mmap very well.

trust_remote_cocde (`bool`, *optional*, defaults to `False`) : Whether to trust remote code

Instantiate a pretrained PyTorch model from a pretrained model configuration.

The model is set in evaluation mode - `model.eval()` - by default, and dropout modules are deactivated. To
train the model, set it back in training mode with `model.train()`.

> [!TIP] > To use private or [gated models](https://huggingface.co/docs/hub/models-gated#gated-models), log-in
with `hf > auth login`. You can also activate the special >
["offline-mode"](https://huggingface.co/diffusers/installation.html#offline-mode) to use this method in a >
firewalled environment.

Example:

```py
from diffusers import AutoModel

unet = AutoModel.from_pretrained("stable-diffusion-v1-5/stable-diffusion-v1-5", subfolder="unet")
```

If you get the error message below, you need to finetune the weights for your downstream task:

```bash
Some weights of UNet2DConditionModel were not initialized from the model checkpoint at stable-diffusion-v1-5/stable-diffusion-v1-5 and are newly initialized because the shapes did not match:
- conv_in.weight: found shape torch.Size([320, 4, 3, 3]) in the checkpoint and torch.Size([320, 9, 3, 3]) in the model instantiated
You should probably TRAIN this model on a down-stream task to be able to use it for predictions and inference.
```

#### from_config[[diffusers.AutoModel.from_config]]

```python
from_config(pretrained_model_name_or_path_or_dict: str | os.PathLike | dict | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/auto_model.py#L38)

**Parameters:**

pretrained_model_name_or_path_or_dict (`str`, `os.PathLike`, or `dict`) : Can be either:  - A string, the *model id* (for example `google/ddpm-celebahq-256`) of a pretrained model configuration hosted on the Hub. - A path to a *directory* (for example `./my_model_directory`) containing a model configuration file. - A config dictionary. 

cache_dir (`Union[str, os.PathLike]`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model configuration, overriding the cached version if it exists.

proxies (`Dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint.

local_files_only(`bool`, *optional*, defaults to `False`) : Whether to only load local model configuration files or not.

token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use.

trust_remote_code (`bool`, *optional*, defaults to `False`) : Whether to trust remote code.

subfolder (`str`, *optional*, defaults to `""`) : The subfolder location of a model file within a larger model repository on the Hub or locally.

**Returns:**

A model object instantiated from the config with random weights.

Instantiate a model from a config dictionary or a pretrained model configuration file with random weights (no
pretrained weights are loaded).

Example:

```py
from diffusers import AutoModel

model = AutoModel.from_config("stable-diffusion-v1-5/stable-diffusion-v1-5", subfolder="unet")
```
