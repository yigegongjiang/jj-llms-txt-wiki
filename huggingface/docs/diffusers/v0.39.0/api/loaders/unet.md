# UNet

Some training methods - like LoRA and Custom Diffusion - typically target the UNet's attention layers, but these training methods can also target other non-attention layers. Instead of training all of a model's parameters, only a subset of the parameters are trained, which is faster and more efficient. This class is useful if you're *only* loading weights into a UNet. If you need to load weights into the text encoder or a text encoder and UNet, try using the [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) function instead.

The `UNet2DConditionLoadersMixin` class provides functions for loading and saving weights, fusing and unfusing LoRAs, disabling and enabling LoRAs, and setting and deleting adapters.

> [!TIP]
> To learn more about how to load LoRA weights, see the [LoRA](../../tutorials/using_peft_for_inference) guide.

## UNet2DConditionLoadersMixin[[diffusers.loaders.UNet2DConditionLoadersMixin]]

#### diffusers.loaders.UNet2DConditionLoadersMixin[[diffusers.loaders.UNet2DConditionLoadersMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/unet.py#L60)

Load LoRA layers into a `UNet2DCondtionModel`.

load_attn_procsdiffusers.loaders.UNet2DConditionLoadersMixin.load_attn_procshttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/unet.py#L68[{"name": "pretrained_model_name_or_path_or_dict", "val": ": str | dict[str, torch.Tensor]"}, {"name": "**kwargs", "val": ""}]- **pretrained_model_name_or_path_or_dict** (`str` or `os.PathLike` or `dict`) --
  Can be either:

  - A string, the model id (for example `google/ddpm-celebahq-256`) of a pretrained model hosted on
    the Hub.
  - A path to a directory (for example `./my_model_directory`) containing the model weights saved
    with [ModelMixin.save_pretrained()](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin.save_pretrained).
  - A [torch state
    dict](https://pytorch.org/tutorials/beginner/saving_loading_models.html#what-is-a-state-dict).

- **cache_dir** (`str | os.PathLike`, *optional*) --
  Path to a directory where a downloaded pretrained model configuration is cached if the standard cache
  is not used.
- **force_download** (`bool`, *optional*, defaults to `False`) --
  Whether or not to force the (re-)download of the model weights and configuration files, overriding the
  cached versions if they exist.

- **proxies** (`dict[str, str]`, *optional*) --
  A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128',
  'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.
- **local_files_only** (`bool`, *optional*, defaults to `False`) --
  Whether to only load local model weights and configuration files or not. If set to `True`, the model
  won't be downloaded from the Hub.
- **token** (`str` or *bool*, *optional*) --
  The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from
  `diffusers-cli login` (stored in `~/.huggingface`) is used.
- **revision** (`str`, *optional*, defaults to `"main"`) --
  The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier
  allowed by Git.
- **subfolder** (`str`, *optional*, defaults to `""`) --
  The subfolder location of a model file within a larger model repository on the Hub or locally.
- **network_alphas** (`dict[str, float]`) --
  The value of the network alpha used for stable learning and preventing underflow. This value has the
  same meaning as the `--network_alpha` option in the kohya-ss trainer script. Refer to [this
  link](https://github.com/darkstorm2150/sd-scripts/blob/main/docs/train_network_README-en.md#execute-learning).
- **adapter_name** (`str`, *optional*, defaults to None) --
  Adapter name to be used for referencing the loaded adapter model. If not specified, it will use
  `default_{i}` where i is the total number of adapters being loaded.
- **weight_name** (`str`, *optional*, defaults to None) --
  Name of the serialized state dict file.
- **low_cpu_mem_usage** (`bool`, *optional*) --
  Speed up model loading by only loading the pretrained LoRA weights and not initializing the random
  weights.0

Load pretrained attention processor layers into [UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel). Attention processor layers have to be
defined in
[`attention_processor.py`](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py)
and be a `torch.nn.Module` class. Currently supported: LoRA, Custom Diffusion. For LoRA, one must install
`peft`: `pip install -U peft`.

Example:

```py
from diffusers import AutoPipelineForText2Image
import torch

pipeline = AutoPipelineForText2Image.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0", torch_dtype=torch.float16
).to("cuda")
pipeline.unet.load_attn_procs(
    "jbilcke-hf/sdxl-cinematic-1", weight_name="pytorch_lora_weights.safetensors", adapter_name="cinematic"
)
```

**Parameters:**

pretrained_model_name_or_path_or_dict (`str` or `os.PathLike` or `dict`) : Can be either:  - A string, the model id (for example `google/ddpm-celebahq-256`) of a pretrained model hosted on the Hub. - A path to a directory (for example `./my_model_directory`) containing the model weights saved with [ModelMixin.save_pretrained()](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin.save_pretrained). - A [torch state dict](https://pytorch.org/tutorials/beginner/saving_loading_models.html#what-is-a-state-dict). 

cache_dir (`str | os.PathLike`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist. 

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

local_files_only (`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to `True`, the model won't be downloaded from the Hub.

token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

subfolder (`str`, *optional*, defaults to `""`) : The subfolder location of a model file within a larger model repository on the Hub or locally.

network_alphas (`dict[str, float]`) : The value of the network alpha used for stable learning and preventing underflow. This value has the same meaning as the `--network_alpha` option in the kohya-ss trainer script. Refer to [this link](https://github.com/darkstorm2150/sd-scripts/blob/main/docs/train_network_README-en.md#execute-learning).

adapter_name (`str`, *optional*, defaults to None) : Adapter name to be used for referencing the loaded adapter model. If not specified, it will use `default_{i}` where i is the total number of adapters being loaded.

weight_name (`str`, *optional*, defaults to None) : Name of the serialized state dict file.

low_cpu_mem_usage (`bool`, *optional*) : Speed up model loading by only loading the pretrained LoRA weights and not initializing the random weights.
#### save_attn_procs[[diffusers.loaders.UNet2DConditionLoadersMixin.save_attn_procs]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/unet.py#L413)

Save attention processor layers to a directory so that it can be reloaded with the
[load_attn_procs()](/docs/diffusers/v0.39.0/en/api/loaders/unet#diffusers.loaders.UNet2DConditionLoadersMixin.load_attn_procs) method.

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

**Parameters:**

save_directory (`str` or `os.PathLike`) : Directory to save an attention processor to (will be created if it doesn't exist).

is_main_process (`bool`, *optional*, defaults to `True`) : Whether the process calling this is the main process or not. Useful during distributed training and you need to call this function on all processes. In this case, set `is_main_process=True` only on the main process to avoid race conditions.

save_function (`Callable`) : The function to use to save the state dictionary. Useful during distributed training when you need to replace `torch.save` with another method. Can be configured with the environment variable `DIFFUSERS_SAVE_MODE`.

safe_serialization (`bool`, *optional*, defaults to `True`) : Whether to save the model using `safetensors` or with `pickle`.
