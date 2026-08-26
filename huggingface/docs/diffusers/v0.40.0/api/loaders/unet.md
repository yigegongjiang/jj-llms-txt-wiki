# UNet

Some training methods - like LoRA and Custom Diffusion - typically target the UNet's attention layers, but these training methods can also target other non-attention layers. Instead of training all of a model's parameters, only a subset of the parameters are trained, which is faster and more efficient. This class is useful if you're *only* loading weights into a UNet. If you need to load weights into the text encoder or a text encoder and UNet, try using the [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) function instead.

The `UNet2DConditionLoadersMixin` class provides functions for loading and saving weights, fusing and unfusing LoRAs, disabling and enabling LoRAs, and setting and deleting adapters.

> [!TIP]
> To learn more about how to load LoRA weights, see the [LoRA](../../tutorials/using_peft_for_inference) guide.

## UNet2DConditionLoadersMixin[[diffusers.loaders.UNet2DConditionLoadersMixin]]

#### diffusers.loaders.UNet2DConditionLoadersMixin[[diffusers.loaders.UNet2DConditionLoadersMixin]]

```python
diffusers.loaders.UNet2DConditionLoadersMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/unet.py#L54)

Load LoRA layers into a `UNet2DCondtionModel`.

#### load_attn_procs[[diffusers.loaders.UNet2DConditionLoadersMixin.load_attn_procs]]

```python
load_attn_procs(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/unet.py#L62)

**Parameters:**

pretrained_model_name_or_path_or_dict (`str` or `os.PathLike` or `dict`) : Can be either:  - A string, the model id (for example `google/ddpm-celebahq-256`) of a pretrained model hosted on the Hub. - A path to a directory (for example `./my_model_directory`) containing the model weights saved with [ModelMixin.save_pretrained()](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin.save_pretrained). - A [torch state dict](https://pytorch.org/tutorials/beginner/saving_loading_models.html#what-is-a-state-dict). 

cache_dir (`str | os.PathLike`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist. 

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

local_files_only (`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to `True`, the model won't be downloaded from the Hub.

token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

subfolder (`str`, *optional*, defaults to `""`) : The subfolder location of a model file within a larger model repository on the Hub or locally.

weight_name (`str`, *optional*, defaults to None) : Name of the serialized state dict file.

Load pretrained Custom Diffusion attention processor layers into [UNet2DConditionModel](/docs/diffusers/v0.40.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel). Attention processor
layers have to be defined in
[`attention_processor.py`](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py)
and be a `torch.nn.Module` class. To load LoRA layers, use [load_lora_adapter()](/docs/diffusers/v0.40.0/en/api/loaders/peft#diffusers.loaders.PeftAdapterMixin.load_lora_adapter)
instead.

Example:

```py
import torch
from diffusers import DiffusionPipeline

pipeline = DiffusionPipeline.from_pretrained(
    "CompVis/stable-diffusion-v1-4",
    torch_dtype=torch.float16,
).to("cuda")
pipeline.unet.load_attn_procs("path-to-save-model", weight_name="pytorch_custom_diffusion_weights.bin")
```

#### save_attn_procs[[diffusers.loaders.UNet2DConditionLoadersMixin.save_attn_procs]]

```python
save_attn_procs(save_directory: str | os.PathLike, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/unet.py#L253)

**Parameters:**

save_directory (`str` or `os.PathLike`) : Directory to save an attention processor to (will be created if it doesn't exist).

is_main_process (`bool`, *optional*, defaults to `True`) : Whether the process calling this is the main process or not. Useful during distributed training and you need to call this function on all processes. In this case, set `is_main_process=True` only on the main process to avoid race conditions.

save_function (`Callable`) : The function to use to save the state dictionary. Useful during distributed training when you need to replace `torch.save` with another method. Can be configured with the environment variable `DIFFUSERS_SAVE_MODE`.

safe_serialization (`bool`, *optional*, defaults to `True`) : Whether to save the model using `safetensors` or with `pickle`.

Save Custom Diffusion attention processor layers to a directory so that it can be reloaded with the
[load_attn_procs()](/docs/diffusers/v0.40.0/en/api/loaders/unet#diffusers.loaders.UNet2DConditionLoadersMixin.load_attn_procs) method. To save LoRA layers, use
[save_lora_adapter()](/docs/diffusers/v0.40.0/en/api/loaders/peft#diffusers.loaders.PeftAdapterMixin.save_lora_adapter) instead.

Example:

```py
import torch
from diffusers import DiffusionPipeline

pipeline = DiffusionPipeline.from_pretrained(
    "CompVis/stable-diffusion-v1-4",
    torch_dtype=torch.float16,
).to("cuda")
pipeline.unet.load_attn_procs("path-to-save-model", weight_name="pytorch_custom_diffusion_weights.bin")
pipeline.unet.save_attn_procs("path-to-save-model", weight_name="pytorch_custom_diffusion_weights.bin")
```
