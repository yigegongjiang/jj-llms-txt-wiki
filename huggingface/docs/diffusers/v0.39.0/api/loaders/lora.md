# LoRA

LoRA is a fast and lightweight training method that inserts and trains a significantly smaller number of parameters instead of all the model parameters. This produces a smaller file (~100 MBs) and makes it easier to quickly train a model to learn a new concept. LoRA weights are typically loaded into the denoiser, text encoder or both. The denoiser usually corresponds to a UNet ([UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel), for example) or a Transformer ([SD3Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/sd3_transformer2d#diffusers.SD3Transformer2DModel), for example). There are several classes for loading LoRA weights:

- `StableDiffusionLoraLoaderMixin` provides functions for loading and unloading, fusing and unfusing, enabling and disabling, and more functions for managing LoRA weights. This class can be used with any model.
- `StableDiffusionXLLoraLoaderMixin` is a [Stable Diffusion (SDXL)](../../api/pipelines/stable_diffusion/stable_diffusion_xl) version of the `StableDiffusionLoraLoaderMixin` class for loading and saving LoRA weights. It can only be used with the SDXL model.
- `SD3LoraLoaderMixin` provides similar functions for [Stable Diffusion 3](https://huggingface.co/blog/sd3).
- `FluxLoraLoaderMixin` provides similar functions for [Flux](https://huggingface.co/docs/diffusers/main/en/api/pipelines/flux).
- `CogVideoXLoraLoaderMixin` provides similar functions for [CogVideoX](https://huggingface.co/docs/diffusers/main/en/api/pipelines/cogvideox).
- `Mochi1LoraLoaderMixin` provides similar functions for [Mochi](https://huggingface.co/docs/diffusers/main/en/api/pipelines/mochi).
- `AuraFlowLoraLoaderMixin` provides similar functions for [AuraFlow](https://huggingface.co/fal/AuraFlow).
- `LTXVideoLoraLoaderMixin` provides similar functions for [LTX-Video](https://huggingface.co/docs/diffusers/main/en/api/pipelines/ltx_video).
- `SanaLoraLoaderMixin` provides similar functions for [Sana](https://huggingface.co/docs/diffusers/main/en/api/pipelines/sana).
- `HeliosLoraLoaderMixin` provides similar functions for [HunyuanVideo](https://huggingface.co/docs/diffusers/main/en/api/pipelines/helios).
- `HunyuanVideoLoraLoaderMixin` provides similar functions for [HunyuanVideo](https://huggingface.co/docs/diffusers/main/en/api/pipelines/hunyuan_video).
- `Lumina2LoraLoaderMixin` provides similar functions for [Lumina2](https://huggingface.co/docs/diffusers/main/en/api/pipelines/lumina2).
- `WanLoraLoaderMixin` provides similar functions for [Wan](https://huggingface.co/docs/diffusers/main/en/api/pipelines/wan).
- `SkyReelsV2LoraLoaderMixin` provides similar functions for [SkyReels-V2](https://huggingface.co/docs/diffusers/main/en/api/pipelines/skyreels_v2).
- `CogView4LoraLoaderMixin` provides similar functions for [CogView4](https://huggingface.co/docs/diffusers/main/en/api/pipelines/cogview4).
- `AmusedLoraLoaderMixin` is for the `AmusedPipeline`.
- `AnimaLoraLoaderMixin` provides similar functions for [Anima](https://huggingface.co/docs/diffusers/main/en/api/pipelines/anima).
- `HiDreamImageLoraLoaderMixin` provides similar functions for [HiDream Image](https://huggingface.co/docs/diffusers/main/en/api/pipelines/hidream)
- `QwenImageLoraLoaderMixin` provides similar functions for [Qwen Image](https://huggingface.co/docs/diffusers/main/en/api/pipelines/qwen).
- `ZImageLoraLoaderMixin` provides similar functions for [Z-Image](https://huggingface.co/docs/diffusers/main/en/api/pipelines/zimage).
- `Flux2LoraLoaderMixin` provides similar functions for [Flux2](https://huggingface.co/docs/diffusers/main/en/api/pipelines/flux2).
- `ErnieImageLoraLoaderMixin` provides similar functions for [Ernie-Image](https://huggingface.co/docs/diffusers/main/en/api/pipelines/ernie_image).
- `LTX2LoraLoaderMixin` provides similar functions for [Flux2](https://huggingface.co/docs/diffusers/main/en/api/pipelines/ltx2).
- `LoraBaseMixin` provides a base class with several utility methods to fuse, unfuse, unload, LoRAs and more.

> [!TIP]
> To learn more about how to load LoRA weights, see the [LoRA](../../tutorials/using_peft_for_inference) loading guide.

## LoraBaseMixin[[diffusers.loaders.lora_base.LoraBaseMixin]]

#### diffusers.loaders.lora_base.LoraBaseMixin[[diffusers.loaders.lora_base.LoraBaseMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L479)

Utility class for handling LoRAs.

delete_adaptersdiffusers.loaders.lora_base.LoraBaseMixin.delete_adaptershttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L845[{"name": "adapter_names", "val": ": list[str] | str"}]- **adapter_names** (`list[str, str]`) --
  The names of the adapters to delete.0

Delete an adapter's LoRA layers from the pipeline.

Example:

```py
from diffusers import AutoPipelineForText2Image
import torch

pipeline = AutoPipelineForText2Image.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0", torch_dtype=torch.float16
).to("cuda")
pipeline.load_lora_weights(
    "jbilcke-hf/sdxl-cinematic-1", weight_name="pytorch_lora_weights.safetensors", adapter_names="cinematic"
)
pipeline.delete_adapters("cinematic")
```

**Parameters:**

adapter_names (`list[str, str]`) : The names of the adapters to delete.
#### disable_lora[[diffusers.loaders.lora_base.LoraBaseMixin.disable_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L785)

Disables the active LoRA layers of the pipeline.

Example:

```py
from diffusers import AutoPipelineForText2Image
import torch

pipeline = AutoPipelineForText2Image.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0", torch_dtype=torch.float16
).to("cuda")
pipeline.load_lora_weights(
    "jbilcke-hf/sdxl-cinematic-1", weight_name="pytorch_lora_weights.safetensors", adapter_name="cinematic"
)
pipeline.disable_lora()
```
#### enable_lora[[diffusers.loaders.lora_base.LoraBaseMixin.enable_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L815)

Enables the active LoRA layers of the pipeline.

Example:

```py
from diffusers import AutoPipelineForText2Image
import torch

pipeline = AutoPipelineForText2Image.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0", torch_dtype=torch.float16
).to("cuda")
pipeline.load_lora_weights(
    "jbilcke-hf/sdxl-cinematic-1", weight_name="pytorch_lora_weights.safetensors", adapter_name="cinematic"
)
pipeline.enable_lora()
```
#### enable_lora_hotswap[[diffusers.loaders.lora_base.LoraBaseMixin.enable_lora_hotswap]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L992)

Hotswap adapters without triggering recompilation of a model or if the ranks of the loaded adapters are
different.

**Parameters:**

target_rank (`int`) : The highest rank among all the adapters that will be loaded.

check_compiled (`str`, *optional*, defaults to `"error"`) : How to handle a model that is already compiled. The check can return the following messages: - "error" (default): raise an error - "warn": issue a warning - "ignore": do nothing
#### fuse_lora[[diffusers.loaders.lora_base.LoraBaseMixin.fuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L537)

Fuses the LoRA parameters into the original parameters of the corresponding blocks.

> [!WARNING] > This is an experimental API.

Example:

```py
from diffusers import DiffusionPipeline
import torch

pipeline = DiffusionPipeline.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0", torch_dtype=torch.float16
).to("cuda")
pipeline.load_lora_weights("nerijs/pixel-art-xl", weight_name="pixel-art-xl.safetensors", adapter_name="pixel")
pipeline.fuse_lora(lora_scale=0.7)
```

**Parameters:**

components : (`list[str]`): list of LoRA-injectable components to fuse the LoRAs into.

lora_scale (`float`, defaults to 1.0) : Controls how much to influence the outputs with the LoRA parameters.

safe_fusing (`bool`, defaults to `False`) : Whether to check fused weights for NaN values before fusing and if values are NaN not fusing them.

adapter_names (`list[str]`, *optional*) : Adapter names to be used for fusing. If nothing is passed, all active adapters will be fused.
#### get_active_adapters[[diffusers.loaders.lora_base.LoraBaseMixin.get_active_adapters]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L883)

Gets the list of the current active adapters.

Example:

```python
from diffusers import DiffusionPipeline

pipeline = DiffusionPipeline.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0",
).to("cuda")
pipeline.load_lora_weights("CiroN2022/toy-face", weight_name="toy_face_sdxl.safetensors", adapter_name="toy")
pipeline.get_active_adapters()
```
#### get_list_adapters[[diffusers.loaders.lora_base.LoraBaseMixin.get_list_adapters]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L916)

Gets the current list of all available adapters in the pipeline.
#### set_adapters[[diffusers.loaders.lora_base.LoraBaseMixin.set_adapters]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L682)

Set the currently active adapters for use in the pipeline.

Example:

```py
from diffusers import AutoPipelineForText2Image
import torch

pipeline = AutoPipelineForText2Image.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0", torch_dtype=torch.float16
).to("cuda")
pipeline.load_lora_weights(
    "jbilcke-hf/sdxl-cinematic-1", weight_name="pytorch_lora_weights.safetensors", adapter_name="cinematic"
)
pipeline.load_lora_weights("nerijs/pixel-art-xl", weight_name="pixel-art-xl.safetensors", adapter_name="pixel")
pipeline.set_adapters(["cinematic", "pixel"], adapter_weights=[0.5, 0.5])
```

**Parameters:**

adapter_names (`list[str]` or `str`) : The names of the adapters to use.

adapter_weights (`list[float, float]`, *optional*) : The adapter(s) weights to use with the UNet. If `None`, the weights are set to `1.0` for all the adapters.
#### set_lora_device[[diffusers.loaders.lora_base.LoraBaseMixin.set_lora_device]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L938)

Moves the LoRAs listed in `adapter_names` to a target device. Useful for offloading the LoRA to the CPU in case
you want to load multiple adapters and free some GPU memory.

After offloading the LoRA adapters to CPU, as long as the rest of the model is still on GPU, the LoRA adapters
can no longer be used for inference, as that would cause a device mismatch. Remember to set the device back to
GPU before using those LoRA adapters for inference.

```python
>>> pipe.load_lora_weights(path_1, adapter_name="adapter-1")
>>> pipe.load_lora_weights(path_2, adapter_name="adapter-2")
>>> pipe.set_adapters("adapter-1")
>>> image_1 = pipe(**kwargs)
>>> # switch to adapter-2, offload adapter-1
>>> pipeline.set_lora_device(adapter_names=["adapter-1"], device="cpu")
>>> pipeline.set_lora_device(adapter_names=["adapter-2"], device="cuda:0")
>>> pipe.set_adapters("adapter-2")
>>> image_2 = pipe(**kwargs)
>>> # switch back to adapter-1, offload adapter-2
>>> pipeline.set_lora_device(adapter_names=["adapter-2"], device="cpu")
>>> pipeline.set_lora_device(adapter_names=["adapter-1"], device="cuda:0")
>>> pipe.set_adapters("adapter-1")
>>> ...
```

**Parameters:**

adapter_names (`list[str]`) : list of adapters to send device to.

device (`torch.device | str | int`) : Device to send the adapters to. Can be either a torch device, a str or an integer.
#### unfuse_lora[[diffusers.loaders.lora_base.LoraBaseMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L626)

Reverses the effect of
[`pipe.fuse_lora()`](https://huggingface.co/docs/diffusers/main/en/api/loaders#diffusers.loaders.LoraBaseMixin.fuse_lora).

> [!WARNING] > This is an experimental API.

**Parameters:**

components (`list[str]`) : list of LoRA-injectable components to unfuse LoRA from.

unfuse_unet (`bool`, defaults to `True`) : Whether to unfuse the UNet LoRA parameters.

unfuse_text_encoder (`bool`, defaults to `True`) : Whether to unfuse the text encoder LoRA parameters. If the text encoder wasn't monkey-patched with the LoRA parameters then it won't have any effect.
#### unload_lora_weights[[diffusers.loaders.lora_base.LoraBaseMixin.unload_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L514)

Unloads the LoRA parameters.

Examples:

```python
>>> # Assuming `pipeline` is already loaded with the LoRA parameters.
>>> pipeline.unload_lora_weights()
>>> ...
```
#### write_lora_layers[[diffusers.loaders.lora_base.LoraBaseMixin.write_lora_layers]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L1015)

Writes the state dict of the LoRA layers (optionally with metadata) to disk.

## StableDiffusionLoraLoaderMixin[[diffusers.loaders.StableDiffusionLoraLoaderMixin]]

#### diffusers.loaders.StableDiffusionLoraLoaderMixin[[diffusers.loaders.StableDiffusionLoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L136)

Load LoRA layers into Stable Diffusion [UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel) and
[`CLIPTextModel`](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel).

load_lora_into_text_encoderdiffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_text_encoderhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L419[{"name": "state_dict", "val": ""}, {"name": "network_alphas", "val": ""}, {"name": "text_encoder", "val": ""}, {"name": "prefix", "val": " = None"}, {"name": "lora_scale", "val": " = 1.0"}, {"name": "adapter_name", "val": " = None"}, {"name": "_pipeline", "val": " = None"}, {"name": "low_cpu_mem_usage", "val": " = False"}, {"name": "hotswap", "val": ": bool = False"}, {"name": "metadata", "val": " = None"}]- **state_dict** (`dict`) --
  A standard state dict containing the lora layer parameters. The key should be prefixed with an
  additional `text_encoder` to distinguish between unet lora layers.
- **network_alphas** (`dict[str, float]`) --
  The value of the network alpha used for stable learning and preventing underflow. This value has the
  same meaning as the `--network_alpha` option in the kohya-ss trainer script. Refer to [this
  link](https://github.com/darkstorm2150/sd-scripts/blob/main/docs/train_network_README-en.md#execute-learning).
- **text_encoder** (`CLIPTextModel`) --
  The text encoder model to load the LoRA layers into.
- **prefix** (`str`) --
  Expected prefix of the `text_encoder` in the `state_dict`.
- **lora_scale** (`float`) --
  How much to scale the output of the lora linear layer before it is added with the output of the regular
  lora layer.
- **adapter_name** (`str`, *optional*) --
  Adapter name to be used for referencing the loaded adapter model. If not specified, it will use
  `default_{i}` where i is the total number of adapters being loaded.
- **low_cpu_mem_usage** (`bool`, *optional*) --
  Speed up model loading by only loading the pretrained LoRA weights and not initializing the random
  weights.
- **hotswap** (`bool`, *optional*) --
  See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights).
- **metadata** (`dict`) --
  Optional LoRA adapter metadata. When supplied, the `LoraConfig` arguments of `peft` won't be derived
  from the state dict.0

This will load the LoRA layers specified in `state_dict` into `text_encoder`

**Parameters:**

state_dict (`dict`) : A standard state dict containing the lora layer parameters. The key should be prefixed with an additional `text_encoder` to distinguish between unet lora layers.

network_alphas (`dict[str, float]`) : The value of the network alpha used for stable learning and preventing underflow. This value has the same meaning as the `--network_alpha` option in the kohya-ss trainer script. Refer to [this link](https://github.com/darkstorm2150/sd-scripts/blob/main/docs/train_network_README-en.md#execute-learning).

text_encoder (`CLIPTextModel`) : The text encoder model to load the LoRA layers into.

prefix (`str`) : Expected prefix of the `text_encoder` in the `state_dict`.

lora_scale (`float`) : How much to scale the output of the lora linear layer before it is added with the output of the regular lora layer.

adapter_name (`str`, *optional*) : Adapter name to be used for referencing the loaded adapter model. If not specified, it will use `default_{i}` where i is the total number of adapters being loaded.

low_cpu_mem_usage (`bool`, *optional*) : Speed up model loading by only loading the pretrained LoRA weights and not initializing the random weights.

hotswap (`bool`, *optional*) : See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights).

metadata (`dict`) : Optional LoRA adapter metadata. When supplied, the `LoraConfig` arguments of `peft` won't be derived from the state dict.
#### load_lora_into_unet[[diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L358)

This will load the LoRA layers specified in `state_dict` into `unet`.

**Parameters:**

state_dict (`dict`) : A standard state dict containing the lora layer parameters. The keys can either be indexed directly into the unet or prefixed with an additional `unet` which can be used to distinguish between text encoder lora layers.

network_alphas (`dict[str, float]`) : The value of the network alpha used for stable learning and preventing underflow. This value has the same meaning as the `--network_alpha` option in the kohya-ss trainer script. Refer to [this link](https://github.com/darkstorm2150/sd-scripts/blob/main/docs/train_network_README-en.md#execute-learning).

unet (`UNet2DConditionModel`) : The UNet model to load the LoRA layers into.

adapter_name (`str`, *optional*) : Adapter name to be used for referencing the loaded adapter model. If not specified, it will use `default_{i}` where i is the total number of adapters being loaded.

low_cpu_mem_usage (`bool`, *optional*) : Speed up model loading only loading the pretrained LoRA weights and not initializing the random weights.

hotswap (`bool`, *optional*) : See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights).

metadata (`dict`) : Optional LoRA adapter metadata. When supplied, the `LoraConfig` arguments of `peft` won't be derived from the state dict.
#### load_lora_weights[[diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L146)

Load LoRA weights specified in `pretrained_model_name_or_path_or_dict` into `self.unet` and
`self.text_encoder`.

All kwargs are forwarded to `self.lora_state_dict`.

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details on how the state dict is
loaded.

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details on how the state dict is
loaded into `self.unet`.

See [load_lora_into_text_encoder()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_text_encoder) for more details on how the state
dict is loaded into `self.text_encoder`.

**Parameters:**

pretrained_model_name_or_path_or_dict (`str` or `os.PathLike` or `dict`) : See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict).

adapter_name (`str`, *optional*) : Adapter name to be used for referencing the loaded adapter model. If not specified, it will use `default_{i}` where i is the total number of adapters being loaded.

low_cpu_mem_usage (`bool`, *optional*) : Speed up model loading by only loading the pretrained LoRA weights and not initializing the random weights.

hotswap (`bool`, *optional*) : Defaults to `False`. Whether to substitute an existing (LoRA) adapter with the newly loaded adapter in-place. This means that, instead of loading an additional adapter, this will take the existing adapter weights and replace them with the weights of the new adapter. This can be faster and more memory efficient. However, the main advantage of hotswapping is that when the model is compiled with torch.compile, loading the new adapter does not require recompilation of the model. When using hotswapping, the passed `adapter_name` should be the name of an already loaded adapter.  If the new adapter and the old adapter have different ranks and/or LoRA alphas (i.e. scaling), you need to call an additional method before loading the adapter:  ```py pipeline = ...  # load diffusers pipeline max_rank = ...  # the highest rank among all LoRAs that you want to load # call *before* compiling and loading the LoRA adapter pipeline.enable_lora_hotswap(target_rank=max_rank) pipeline.load_lora_weights(file_name) # optionally compile the model now ```  Note that hotswapping adapters of the text encoder is not yet supported. There are some further limitations to this technique, which are documented here: https://huggingface.co/docs/peft/main/en/package_reference/hotswap

kwargs (`dict`, *optional*) : See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict).
#### lora_state_dict[[diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L247)

Return state dict for lora weights and the network alphas.

> [!WARNING] > We support loading A1111 formatted LoRA checkpoints in a limited capacity. > > This function is
experimental and might change in the future.

**Parameters:**

pretrained_model_name_or_path_or_dict (`str` or `os.PathLike` or `dict`) : Can be either:  - A string, the *model id* (for example `google/ddpm-celebahq-256`) of a pretrained model hosted on the Hub. - A path to a *directory* (for example `./my_model_directory`) containing the model weights saved with [ModelMixin.save_pretrained()](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin.save_pretrained). - A [torch state dict](https://pytorch.org/tutorials/beginner/saving_loading_models.html#what-is-a-state-dict). 

cache_dir (`str | os.PathLike`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist. 

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

local_files_only (`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to `True`, the model won't be downloaded from the Hub.

token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

subfolder (`str`, *optional*, defaults to `""`) : The subfolder location of a model file within a larger model repository on the Hub or locally.

weight_name (`str`, *optional*, defaults to None) : Name of the serialized state dict file.

return_lora_metadata (`bool`, *optional*, defaults to False) : When enabled, additionally return the LoRA adapter metadata, typically found in the state dict.
#### save_lora_weights[[diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L477)

Save the LoRA parameters corresponding to the UNet and text encoder.

**Parameters:**

save_directory (`str` or `os.PathLike`) : Directory to save LoRA parameters to. Will be created if it doesn't exist.

unet_lora_layers (`dict[str, torch.nn.Module]` or `dict[str, torch.Tensor]`) : State dict of the LoRA layers corresponding to the `unet`.

text_encoder_lora_layers (`dict[str, torch.nn.Module]` or `dict[str, torch.Tensor]`) : State dict of the LoRA layers corresponding to the `text_encoder`. Must explicitly pass the text encoder LoRA state dict because it comes from 🤗 Transformers.

is_main_process (`bool`, *optional*, defaults to `True`) : Whether the process calling this is the main process or not. Useful during distributed training and you need to call this function on all processes. In this case, set `is_main_process=True` only on the main process to avoid race conditions.

save_function (`Callable`) : The function to use to save the state dictionary. Useful during distributed training when you need to replace `torch.save` with another method. Can be configured with the environment variable `DIFFUSERS_SAVE_MODE`.

safe_serialization (`bool`, *optional*, defaults to `True`) : Whether to save the model using `safetensors` or the traditional PyTorch way with `pickle`.

unet_lora_adapter_metadata : LoRA adapter metadata associated with the unet to be serialized with the state dict.

text_encoder_lora_adapter_metadata : LoRA adapter metadata associated with the text encoder to be serialized with the state dict.

## StableDiffusionXLLoraLoaderMixin[[diffusers.loaders.StableDiffusionXLLoraLoaderMixin]]

#### diffusers.loaders.StableDiffusionXLLoraLoaderMixin[[diffusers.loaders.StableDiffusionXLLoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L600)

Load LoRA layers into Stable Diffusion XL [UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel),
[`CLIPTextModel`](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), and
[`CLIPTextModelWithProjection`](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModelWithProjection).

fuse_loradiffusers.loaders.StableDiffusionXLLoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L966[{"name": "components", "val": ": list = ['unet', 'text_encoder', 'text_encoder_2']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_text_encoder[[diffusers.loaders.StableDiffusionXLLoraLoaderMixin.load_lora_into_text_encoder]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L859)

This will load the LoRA layers specified in `state_dict` into `text_encoder`

**Parameters:**

state_dict (`dict`) : A standard state dict containing the lora layer parameters. The key should be prefixed with an additional `text_encoder` to distinguish between unet lora layers.

network_alphas (`dict[str, float]`) : The value of the network alpha used for stable learning and preventing underflow. This value has the same meaning as the `--network_alpha` option in the kohya-ss trainer script. Refer to [this link](https://github.com/darkstorm2150/sd-scripts/blob/main/docs/train_network_README-en.md#execute-learning).

text_encoder (`CLIPTextModel`) : The text encoder model to load the LoRA layers into.

prefix (`str`) : Expected prefix of the `text_encoder` in the `state_dict`.

lora_scale (`float`) : How much to scale the output of the lora linear layer before it is added with the output of the regular lora layer.

adapter_name (`str`, *optional*) : Adapter name to be used for referencing the loaded adapter model. If not specified, it will use `default_{i}` where i is the total number of adapters being loaded.

low_cpu_mem_usage (`bool`, *optional*) : Speed up model loading by only loading the pretrained LoRA weights and not initializing the random weights.

hotswap (`bool`, *optional*) : See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights).

metadata (`dict`) : Optional LoRA adapter metadata. When supplied, the `LoraConfig` arguments of `peft` won't be derived from the state dict.
#### load_lora_into_unet[[diffusers.loaders.StableDiffusionXLLoraLoaderMixin.load_lora_into_unet]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L797)

This will load the LoRA layers specified in `state_dict` into `unet`.

**Parameters:**

state_dict (`dict`) : A standard state dict containing the lora layer parameters. The keys can either be indexed directly into the unet or prefixed with an additional `unet` which can be used to distinguish between text encoder lora layers.

network_alphas (`dict[str, float]`) : The value of the network alpha used for stable learning and preventing underflow. This value has the same meaning as the `--network_alpha` option in the kohya-ss trainer script. Refer to [this link](https://github.com/darkstorm2150/sd-scripts/blob/main/docs/train_network_README-en.md#execute-learning).

unet (`UNet2DConditionModel`) : The UNet model to load the LoRA layers into.

adapter_name (`str`, *optional*) : Adapter name to be used for referencing the loaded adapter model. If not specified, it will use `default_{i}` where i is the total number of adapters being loaded.

low_cpu_mem_usage (`bool`, *optional*) : Speed up model loading only loading the pretrained LoRA weights and not initializing the random weights.

hotswap (`bool`, *optional*) : See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights).

metadata (`dict`) : Optional LoRA adapter metadata. When supplied, the `LoraConfig` arguments of `peft` won't be derived from the state dict.
#### load_lora_weights[[diffusers.loaders.StableDiffusionXLLoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L611)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.StableDiffusionXLLoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L685)

Return state dict for lora weights and the network alphas.

> [!WARNING] > We support loading A1111 formatted LoRA checkpoints in a limited capacity. > > This function is
experimental and might change in the future.

**Parameters:**

pretrained_model_name_or_path_or_dict (`str` or `os.PathLike` or `dict`) : Can be either:  - A string, the *model id* (for example `google/ddpm-celebahq-256`) of a pretrained model hosted on the Hub. - A path to a *directory* (for example `./my_model_directory`) containing the model weights saved with [ModelMixin.save_pretrained()](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin.save_pretrained). - A [torch state dict](https://pytorch.org/tutorials/beginner/saving_loading_models.html#what-is-a-state-dict). 

cache_dir (`str | os.PathLike`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist. 

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

local_files_only (`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to `True`, the model won't be downloaded from the Hub.

token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

subfolder (`str`, *optional*, defaults to `""`) : The subfolder location of a model file within a larger model repository on the Hub or locally.

weight_name (`str`, *optional*, defaults to None) : Name of the serialized state dict file.

return_lora_metadata (`bool`, *optional*, defaults to False) : When enabled, additionally return the LoRA adapter metadata, typically found in the state dict.
#### save_lora_weights[[diffusers.loaders.StableDiffusionXLLoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L918)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.StableDiffusionXLLoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L985)

See `unfuse_lora()` for more details.

## SD3LoraLoaderMixin[[diffusers.loaders.SD3LoraLoaderMixin]]

#### diffusers.loaders.SD3LoraLoaderMixin[[diffusers.loaders.SD3LoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L992)

Load LoRA layers into [SD3Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/sd3_transformer2d#diffusers.SD3Transformer2DModel),
[`CLIPTextModel`](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), and
[`CLIPTextModelWithProjection`](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModelWithProjection).

Specific to [StableDiffusion3Pipeline](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/stable_diffusion_3#diffusers.StableDiffusion3Pipeline).

fuse_loradiffusers.loaders.SD3LoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1264[{"name": "components", "val": ": list = ['transformer', 'text_encoder', 'text_encoder_2']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_text_encoder[[diffusers.loaders.SD3LoraLoaderMixin.load_lora_into_text_encoder]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1155)

This will load the LoRA layers specified in `state_dict` into `text_encoder`

**Parameters:**

state_dict (`dict`) : A standard state dict containing the lora layer parameters. The key should be prefixed with an additional `text_encoder` to distinguish between unet lora layers.

network_alphas (`dict[str, float]`) : The value of the network alpha used for stable learning and preventing underflow. This value has the same meaning as the `--network_alpha` option in the kohya-ss trainer script. Refer to [this link](https://github.com/darkstorm2150/sd-scripts/blob/main/docs/train_network_README-en.md#execute-learning).

text_encoder (`CLIPTextModel`) : The text encoder model to load the LoRA layers into.

prefix (`str`) : Expected prefix of the `text_encoder` in the `state_dict`.

lora_scale (`float`) : How much to scale the output of the lora linear layer before it is added with the output of the regular lora layer.

adapter_name (`str`, *optional*) : Adapter name to be used for referencing the loaded adapter model. If not specified, it will use `default_{i}` where i is the total number of adapters being loaded.

low_cpu_mem_usage (`bool`, *optional*) : Speed up model loading by only loading the pretrained LoRA weights and not initializing the random weights.

hotswap (`bool`, *optional*) : See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights).

metadata (`dict`) : Optional LoRA adapter metadata. When supplied, the `LoraConfig` arguments of `peft` won't be derived from the state dict.
#### load_lora_into_transformer[[diffusers.loaders.SD3LoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1124)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.SD3LoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1059)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.SD3LoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1005)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.SD3LoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1214)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.SD3LoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1284)

See `unfuse_lora()` for more details.

## FluxLoraLoaderMixin[[diffusers.loaders.FluxLoraLoaderMixin]]

#### diffusers.loaders.FluxLoraLoaderMixin[[diffusers.loaders.FluxLoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1491)

Load LoRA layers into [FluxTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/flux_transformer#diffusers.FluxTransformer2DModel),
[`CLIPTextModel`](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel).

Specific to [FluxPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/flux#diffusers.FluxPipeline).

fuse_loradiffusers.loaders.FluxLoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1940[{"name": "components", "val": ": list = ['transformer']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### load_lora_into_text_encoder[[diffusers.loaders.FluxLoraLoaderMixin.load_lora_into_text_encoder]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1817)

This will load the LoRA layers specified in `state_dict` into `text_encoder`

**Parameters:**

state_dict (`dict`) : A standard state dict containing the lora layer parameters. The key should be prefixed with an additional `text_encoder` to distinguish between unet lora layers.

network_alphas (`dict[str, float]`) : The value of the network alpha used for stable learning and preventing underflow. This value has the same meaning as the `--network_alpha` option in the kohya-ss trainer script. Refer to [this link](https://github.com/darkstorm2150/sd-scripts/blob/main/docs/train_network_README-en.md#execute-learning).

text_encoder (`CLIPTextModel`) : The text encoder model to load the LoRA layers into.

prefix (`str`) : Expected prefix of the `text_encoder` in the `state_dict`.

lora_scale (`float`) : How much to scale the output of the lora linear layer before it is added with the output of the regular lora layer.

adapter_name (`str`, *optional*) : Adapter name to be used for referencing the loaded adapter model. If not specified, it will use `default_{i}` where i is the total number of adapters being loaded.

low_cpu_mem_usage (`bool`, *optional*) : Speed up model loading by only loading the pretrained LoRA weights and not initializing the random weights.

hotswap (`bool`, *optional*) : See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights).

metadata (`dict`) : Optional LoRA adapter metadata. When supplied, the `LoraConfig` arguments of `peft` won't be derived from the state dict.
#### load_lora_into_transformer[[diffusers.loaders.FluxLoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1731)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.FluxLoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1629)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.FluxLoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1504)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.FluxLoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1876)

Save the LoRA parameters corresponding to the UNet and text encoder.

**Parameters:**

save_directory (`str` or `os.PathLike`) : Directory to save LoRA parameters to. Will be created if it doesn't exist.

transformer_lora_layers (`dict[str, torch.nn.Module]` or `dict[str, torch.Tensor]`) : State dict of the LoRA layers corresponding to the `transformer`.

text_encoder_lora_layers (`dict[str, torch.nn.Module]` or `dict[str, torch.Tensor]`) : State dict of the LoRA layers corresponding to the `text_encoder`. Must explicitly pass the text encoder LoRA state dict because it comes from 🤗 Transformers.

is_main_process (`bool`, *optional*, defaults to `True`) : Whether the process calling this is the main process or not. Useful during distributed training and you need to call this function on all processes. In this case, set `is_main_process=True` only on the main process to avoid race conditions.

save_function (`Callable`) : The function to use to save the state dictionary. Useful during distributed training when you need to replace `torch.save` with another method. Can be configured with the environment variable `DIFFUSERS_SAVE_MODE`.

safe_serialization (`bool`, *optional*, defaults to `True`) : Whether to save the model using `safetensors` or the traditional PyTorch way with `pickle`.

transformer_lora_adapter_metadata : LoRA adapter metadata associated with the transformer to be serialized with the state dict.

text_encoder_lora_adapter_metadata : LoRA adapter metadata associated with the text encoder to be serialized with the state dict.
#### unfuse_lora[[diffusers.loaders.FluxLoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1972)

Reverses the effect of
[`pipe.fuse_lora()`](https://huggingface.co/docs/diffusers/main/en/api/loaders#diffusers.loaders.LoraBaseMixin.fuse_lora).

> [!WARNING] > This is an experimental API.

**Parameters:**

components (`list[str]`) : list of LoRA-injectable components to unfuse LoRA from.
#### unload_lora_weights[[diffusers.loaders.FluxLoraLoaderMixin.unload_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1989)

Unloads the LoRA parameters.

Examples:

```python
>>> # Assuming `pipeline` is already loaded with the LoRA parameters.
>>> pipeline.unload_lora_weights()
>>> ...
```

**Parameters:**

reset_to_overwritten_params (`bool`, defaults to `False`) : Whether to reset the LoRA-loaded modules to their original params. Refer to the [Flux documentation](https://huggingface.co/docs/diffusers/main/en/api/pipelines/flux) to learn more.

## Flux2LoraLoaderMixin[[diffusers.loaders.Flux2LoraLoaderMixin]]

#### diffusers.loaders.Flux2LoraLoaderMixin[[diffusers.loaders.Flux2LoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6014)

Load LoRA layers into [Flux2Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/flux2_transformer#diffusers.Flux2Transformer2DModel). Specific to [Flux2Pipeline](/docs/diffusers/v0.39.0/en/api/pipelines/flux2#diffusers.Flux2Pipeline).

fuse_loradiffusers.loaders.Flux2LoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6201[{"name": "components", "val": ": list = ['transformer']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_transformer[[diffusers.loaders.Flux2LoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6133)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.Flux2LoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6092)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.Flux2LoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6022)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.Flux2LoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6165)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.Flux2LoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6221)

See `unfuse_lora()` for more details.

## ErnieImageLoraLoaderMixin[[diffusers.loaders.ErnieImageLoraLoaderMixin]]

#### diffusers.loaders.ErnieImageLoraLoaderMixin[[diffusers.loaders.ErnieImageLoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6435)

Load LoRA layers into [ErnieImageTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/ernie_image_transformer2d#diffusers.ErnieImageTransformer2DModel). Specific to `ErnieImagePipeline`.

fuse_loradiffusers.loaders.ErnieImageLoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6619[{"name": "components", "val": ": list = ['transformer']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_transformer[[diffusers.loaders.ErnieImageLoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6551)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.ErnieImageLoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6510)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.ErnieImageLoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6443)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.ErnieImageLoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6583)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.ErnieImageLoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6639)

See `unfuse_lora()` for more details.

## LTX2LoraLoaderMixin[[diffusers.loaders.LTX2LoraLoaderMixin]]

#### diffusers.loaders.LTX2LoraLoaderMixin[[diffusers.loaders.LTX2LoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3020)

Load LoRA layers into [LTX2VideoTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/ltx2_video_transformer3d#diffusers.LTX2VideoTransformer3DModel). Specific to [LTX2Pipeline](/docs/diffusers/v0.39.0/en/api/pipelines/ltx2#diffusers.LTX2Pipeline).

fuse_loradiffusers.loaders.LTX2LoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3220[{"name": "components", "val": ": list = ['transformer']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_transformer[[diffusers.loaders.LTX2LoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3151)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.LTX2LoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3093)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.LTX2LoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3029)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.LTX2LoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3184)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.LTX2LoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3240)

See `unfuse_lora()` for more details.

## CogVideoXLoraLoaderMixin[[diffusers.loaders.CogVideoXLoraLoaderMixin]]

#### diffusers.loaders.CogVideoXLoraLoaderMixin[[diffusers.loaders.CogVideoXLoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2421)

Load LoRA layers into [CogVideoXTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/cogvideox_transformer3d#diffusers.CogVideoXTransformer3DModel). Specific to [CogVideoXPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/cogvideox#diffusers.CogVideoXPipeline).

fuse_loradiffusers.loaders.CogVideoXLoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2591[{"name": "components", "val": ": list = ['transformer']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_transformer[[diffusers.loaders.CogVideoXLoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2525)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.CogVideoXLoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2484)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.CogVideoXLoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2429)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.CogVideoXLoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2557)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.CogVideoXLoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2610)

See `unfuse_lora()` for more details.

## Mochi1LoraLoaderMixin[[diffusers.loaders.Mochi1LoraLoaderMixin]]

#### diffusers.loaders.Mochi1LoraLoaderMixin[[diffusers.loaders.Mochi1LoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2617)

Load LoRA layers into [MochiTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/mochi_transformer3d#diffusers.MochiTransformer3DModel). Specific to [MochiPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/mochi#diffusers.MochiPipeline).

fuse_loradiffusers.loaders.Mochi1LoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2790[{"name": "components", "val": ": list = ['transformer']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_transformer[[diffusers.loaders.Mochi1LoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2722)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.Mochi1LoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2681)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.Mochi1LoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2625)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.Mochi1LoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2754)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.Mochi1LoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2810)

See `unfuse_lora()` for more details.

## AuraFlowLoraLoaderMixin[[diffusers.loaders.AuraFlowLoraLoaderMixin]]

#### diffusers.loaders.AuraFlowLoraLoaderMixin[[diffusers.loaders.AuraFlowLoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1291)

Load LoRA layers into [AuraFlowTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/aura_flow_transformer2d#diffusers.AuraFlowTransformer2DModel) Specific to [AuraFlowPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/aura_flow#diffusers.AuraFlowPipeline).

fuse_loradiffusers.loaders.AuraFlowLoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1464[{"name": "components", "val": ": list = ['transformer']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_transformer[[diffusers.loaders.AuraFlowLoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1396)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.AuraFlowLoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1355)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.AuraFlowLoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1299)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.AuraFlowLoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1428)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.AuraFlowLoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L1484)

See `unfuse_lora()` for more details.

## LTXVideoLoraLoaderMixin[[diffusers.loaders.LTXVideoLoraLoaderMixin]]

#### diffusers.loaders.LTXVideoLoraLoaderMixin[[diffusers.loaders.LTXVideoLoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2817)

Load LoRA layers into [LTXVideoTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/ltx_video_transformer3d#diffusers.LTXVideoTransformer3DModel). Specific to [LTXPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/ltx_video#diffusers.LTXPipeline).

fuse_loradiffusers.loaders.LTXVideoLoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2993[{"name": "components", "val": ": list = ['transformer']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_transformer[[diffusers.loaders.LTXVideoLoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2925)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.LTXVideoLoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2884)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.LTXVideoLoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2825)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.LTXVideoLoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2957)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.LTXVideoLoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3013)

See `unfuse_lora()` for more details.

## SanaLoraLoaderMixin[[diffusers.loaders.SanaLoraLoaderMixin]]

#### diffusers.loaders.SanaLoraLoaderMixin[[diffusers.loaders.SanaLoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3247)

Load LoRA layers into [SanaTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/sana_transformer2d#diffusers.SanaTransformer2DModel). Specific to [SanaPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/sana#diffusers.SanaPipeline).

fuse_loradiffusers.loaders.SanaLoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3420[{"name": "components", "val": ": list = ['transformer']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_transformer[[diffusers.loaders.SanaLoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3352)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.SanaLoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3311)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.SanaLoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3255)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.SanaLoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3384)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.SanaLoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3440)

See `unfuse_lora()` for more details.

## HeliosLoraLoaderMixin[[diffusers.loaders.HeliosLoraLoaderMixin]]

#### diffusers.loaders.HeliosLoraLoaderMixin[[diffusers.loaders.HeliosLoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3447)

Load LoRA layers into [HeliosTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/helios_transformer3d#diffusers.HeliosTransformer3DModel). Specific to [HeliosPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/helios#diffusers.HeliosPipeline) and [HeliosPyramidPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/helios#diffusers.HeliosPyramidPipeline).

fuse_loradiffusers.loaders.HeliosLoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3621[{"name": "components", "val": ": list = ['transformer']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_transformer[[diffusers.loaders.HeliosLoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3553)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.HeliosLoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3513)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.HeliosLoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3455)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.HeliosLoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3585)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.HeliosLoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3641)

See `unfuse_lora()` for more details.

## HunyuanVideoLoraLoaderMixin[[diffusers.loaders.HunyuanVideoLoraLoaderMixin]]

#### diffusers.loaders.HunyuanVideoLoraLoaderMixin[[diffusers.loaders.HunyuanVideoLoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3648)

Load LoRA layers into [HunyuanVideoTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/hunyuan_video_transformer_3d#diffusers.HunyuanVideoTransformer3DModel). Specific to [HunyuanVideoPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/hunyuan_video#diffusers.HunyuanVideoPipeline).

fuse_loradiffusers.loaders.HunyuanVideoLoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3824[{"name": "components", "val": ": list = ['transformer']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_transformer[[diffusers.loaders.HunyuanVideoLoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3756)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.HunyuanVideoLoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3715)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.HunyuanVideoLoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3656)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.HunyuanVideoLoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3788)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.HunyuanVideoLoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3844)

See `unfuse_lora()` for more details.

## Lumina2LoraLoaderMixin[[diffusers.loaders.Lumina2LoraLoaderMixin]]

#### diffusers.loaders.Lumina2LoraLoaderMixin[[diffusers.loaders.Lumina2LoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3851)

Load LoRA layers into [Lumina2Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/lumina2_transformer2d#diffusers.Lumina2Transformer2DModel). Specific to `Lumina2Text2ImgPipeline`.

fuse_loradiffusers.loaders.Lumina2LoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4028[{"name": "components", "val": ": list = ['transformer']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_transformer[[diffusers.loaders.Lumina2LoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3960)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.Lumina2LoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3919)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.Lumina2LoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3859)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.Lumina2LoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L3992)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.Lumina2LoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4048)

See `unfuse_lora()` for more details.

## CogView4LoraLoaderMixin[[diffusers.loaders.CogView4LoraLoaderMixin]]

#### diffusers.loaders.CogView4LoraLoaderMixin[[diffusers.loaders.CogView4LoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4806)

Load LoRA layers into [WanTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/wan_transformer_3d#diffusers.WanTransformer3DModel). Specific to [CogView4Pipeline](/docs/diffusers/v0.39.0/en/api/pipelines/cogview4#diffusers.CogView4Pipeline).

fuse_loradiffusers.loaders.CogView4LoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4979[{"name": "components", "val": ": list = ['transformer']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_transformer[[diffusers.loaders.CogView4LoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4911)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.CogView4LoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4870)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.CogView4LoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4814)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.CogView4LoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4943)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.CogView4LoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4999)

See `unfuse_lora()` for more details.

## WanLoraLoaderMixin[[diffusers.loaders.WanLoraLoaderMixin]]

#### diffusers.loaders.WanLoraLoaderMixin[[diffusers.loaders.WanLoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4255)

Load LoRA layers into [WanTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/wan_transformer_3d#diffusers.WanTransformer3DModel). Specific to [WanPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/wan#diffusers.WanPipeline) and `[WanImageToVideoPipeline`].

fuse_loradiffusers.loaders.WanLoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4502[{"name": "components", "val": ": list = ['transformer']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_transformer[[diffusers.loaders.WanLoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4434)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.WanLoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4369)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.WanLoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4263)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.WanLoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4466)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.WanLoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4522)

See `unfuse_lora()` for more details.

## SkyReelsV2LoraLoaderMixin[[diffusers.loaders.SkyReelsV2LoraLoaderMixin]]

#### diffusers.loaders.SkyReelsV2LoraLoaderMixin[[diffusers.loaders.SkyReelsV2LoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4529)

Load LoRA layers into [SkyReelsV2Transformer3DModel](/docs/diffusers/v0.39.0/en/api/models/skyreels_v2_transformer_3d#diffusers.SkyReelsV2Transformer3DModel).

fuse_loradiffusers.loaders.SkyReelsV2LoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4779[{"name": "components", "val": ": list = ['transformer']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_transformer[[diffusers.loaders.SkyReelsV2LoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4711)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.SkyReelsV2LoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4646)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.SkyReelsV2LoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4537)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.SkyReelsV2LoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4743)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.SkyReelsV2LoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4799)

See `unfuse_lora()` for more details.

## AmusedLoraLoaderMixin[[diffusers.loaders.AmusedLoraLoaderMixin]]

#### diffusers.loaders.AmusedLoraLoaderMixin[[diffusers.loaders.AmusedLoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2269)

load_lora_into_transformerdiffusers.loaders.AmusedLoraLoaderMixin.load_lora_into_transformerhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2274[{"name": "state_dict", "val": ""}, {"name": "network_alphas", "val": ""}, {"name": "transformer", "val": ""}, {"name": "adapter_name", "val": " = None"}, {"name": "metadata", "val": " = None"}, {"name": "_pipeline", "val": " = None"}, {"name": "low_cpu_mem_usage", "val": " = False"}, {"name": "hotswap", "val": ": bool = False"}]

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### save_lora_weights[[diffusers.loaders.AmusedLoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L2366)

Save the LoRA parameters corresponding to the UNet and text encoder.

**Parameters:**

save_directory (`str` or `os.PathLike`) : Directory to save LoRA parameters to. Will be created if it doesn't exist.

unet_lora_layers (`dict[str, torch.nn.Module]` or `dict[str, torch.Tensor]`) : State dict of the LoRA layers corresponding to the `unet`.

text_encoder_lora_layers (`dict[str, torch.nn.Module]` or `dict[str, torch.Tensor]`) : State dict of the LoRA layers corresponding to the `text_encoder`. Must explicitly pass the text encoder LoRA state dict because it comes from 🤗 Transformers.

is_main_process (`bool`, *optional*, defaults to `True`) : Whether the process calling this is the main process or not. Useful during distributed training and you need to call this function on all processes. In this case, set `is_main_process=True` only on the main process to avoid race conditions.

save_function (`Callable`) : The function to use to save the state dictionary. Useful during distributed training when you need to replace `torch.save` with another method. Can be configured with the environment variable `DIFFUSERS_SAVE_MODE`.

safe_serialization (`bool`, *optional*, defaults to `True`) : Whether to save the model using `safetensors` or the traditional PyTorch way with `pickle`.

## AnimaLoraLoaderMixin[[diffusers.loaders.AnimaLoraLoaderMixin]]

#### diffusers.loaders.AnimaLoraLoaderMixin[[diffusers.loaders.AnimaLoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5826)

Load LoRA layers into [CosmosTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/cosmos_transformer3d#diffusers.CosmosTransformer3DModel) and [AnimaTextConditioner](/docs/diffusers/v0.39.0/en/api/pipelines/anima#diffusers.AnimaTextConditioner).

fuse_loradiffusers.loaders.AnimaLoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5988[{"name": "components", "val": ": list = ['transformer', 'text_conditioner']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_weights[[diffusers.loaders.AnimaLoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5891)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.AnimaLoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5835)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### unfuse_lora[[diffusers.loaders.AnimaLoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6007)

See `unfuse_lora()` for more details.

## HiDreamImageLoraLoaderMixin[[diffusers.loaders.HiDreamImageLoraLoaderMixin]]

#### diffusers.loaders.HiDreamImageLoraLoaderMixin[[diffusers.loaders.HiDreamImageLoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5006)

Load LoRA layers into [HiDreamImageTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/hidream_image_transformer#diffusers.HiDreamImageTransformer2DModel). Specific to [HiDreamImagePipeline](/docs/diffusers/v0.39.0/en/api/pipelines/hidream#diffusers.HiDreamImagePipeline).

fuse_loradiffusers.loaders.HiDreamImageLoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5182[{"name": "components", "val": ": list = ['transformer']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_transformer[[diffusers.loaders.HiDreamImageLoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5114)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.HiDreamImageLoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5073)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.HiDreamImageLoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5014)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.HiDreamImageLoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5146)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.HiDreamImageLoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5202)

See `unfuse_lora()` for more details.

## QwenImageLoraLoaderMixin[[diffusers.loaders.QwenImageLoraLoaderMixin]]

#### diffusers.loaders.QwenImageLoraLoaderMixin[[diffusers.loaders.QwenImageLoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5209)

Load LoRA layers into [QwenImageTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/qwenimage_transformer2d#diffusers.QwenImageTransformer2DModel). Specific to [QwenImagePipeline](/docs/diffusers/v0.39.0/en/api/pipelines/qwenimage#diffusers.QwenImagePipeline).

fuse_loradiffusers.loaders.QwenImageLoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5388[{"name": "components", "val": ": list = ['transformer']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_transformer[[diffusers.loaders.QwenImageLoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5320)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.QwenImageLoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5279)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.QwenImageLoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5217)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.QwenImageLoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5352)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.QwenImageLoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5408)

See `unfuse_lora()` for more details.

## ZImageLoraLoaderMixin[[diffusers.loaders.ZImageLoraLoaderMixin]]

#### diffusers.loaders.ZImageLoraLoaderMixin[[diffusers.loaders.ZImageLoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5620)

Load LoRA layers into [ZImageTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/z_image_transformer2d#diffusers.ZImageTransformer2DModel). Specific to [ZImagePipeline](/docs/diffusers/v0.39.0/en/api/pipelines/z_image#diffusers.ZImagePipeline).

fuse_loradiffusers.loaders.ZImageLoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5799[{"name": "components", "val": ": list = ['transformer']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_transformer[[diffusers.loaders.ZImageLoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5731)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.ZImageLoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5690)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.ZImageLoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5628)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.ZImageLoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5763)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.ZImageLoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5819)

See `unfuse_lora()` for more details.

## CosmosLoraLoaderMixin[[diffusers.loaders.CosmosLoraLoaderMixin]]

#### diffusers.loaders.CosmosLoraLoaderMixin[[diffusers.loaders.CosmosLoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6646)

Load LoRA layers into [CosmosTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/cosmos_transformer3d#diffusers.CosmosTransformer3DModel), Specific to [Cosmos2_5_PredictBasePipeline](/docs/diffusers/v0.39.0/en/api/pipelines/cosmos#diffusers.Cosmos2_5_PredictBasePipeline).

fuse_loradiffusers.loaders.CosmosLoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6820[{"name": "components", "val": ": list = ['transformer']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_transformer[[diffusers.loaders.CosmosLoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6752)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.CosmosLoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6711)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.CosmosLoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6655)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.CosmosLoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6784)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.CosmosLoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6840)

See `unfuse_lora()` for more details.

## KandinskyLoraLoaderMixin[[diffusers.loaders.KandinskyLoraLoaderMixin]]
#### diffusers.loaders.KandinskyLoraLoaderMixin[[diffusers.loaders.KandinskyLoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4055)

Load LoRA layers into `Kandinsky5Transformer3DModel`,

fuse_loradiffusers.loaders.KandinskyLoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4228[{"name": "components", "val": ": list = ['transformer']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_transformer[[diffusers.loaders.KandinskyLoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4160)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.KandinskyLoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4119)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.KandinskyLoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4063)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.KandinskyLoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4192)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.KandinskyLoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L4248)

See `unfuse_lora()` for more details.

## Ideogram4LoraLoaderMixin[[diffusers.loaders.Ideogram4LoraLoaderMixin]]

#### diffusers.loaders.Ideogram4LoraLoaderMixin[[diffusers.loaders.Ideogram4LoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6228)

Load LoRA layers into [Ideogram4Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/ideogram4_transformer2d#diffusers.Ideogram4Transformer2DModel). Specific to [Ideogram4Pipeline](/docs/diffusers/v0.39.0/en/api/pipelines/ideogram4#diffusers.Ideogram4Pipeline).

fuse_loradiffusers.loaders.Ideogram4LoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6408[{"name": "components", "val": ": list = ['transformer']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_transformer[[diffusers.loaders.Ideogram4LoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6340)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.Ideogram4LoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6299)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.Ideogram4LoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6236)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.Ideogram4LoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6372)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.Ideogram4LoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L6428)

See `unfuse_lora()` for more details.

## Krea2LoraLoaderMixin[[diffusers.loaders.Krea2LoraLoaderMixin]]

#### diffusers.loaders.Krea2LoraLoaderMixin[[diffusers.loaders.Krea2LoraLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5415)

Load LoRA layers into [Krea2Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/krea2_transformer2d#diffusers.Krea2Transformer2DModel). Specific to [Krea2Pipeline](/docs/diffusers/v0.39.0/en/api/pipelines/krea2#diffusers.Krea2Pipeline).

fuse_loradiffusers.loaders.Krea2LoraLoaderMixin.fuse_lorahttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5593[{"name": "components", "val": ": list = ['transformer']"}, {"name": "lora_scale", "val": ": float = 1.0"}, {"name": "safe_fusing", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}, {"name": "**kwargs", "val": ""}]

See `fuse_lora()` for more details.
#### load_lora_into_transformer[[diffusers.loaders.Krea2LoraLoaderMixin.load_lora_into_transformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5525)

See [load_lora_into_unet()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.
#### load_lora_weights[[diffusers.loaders.Krea2LoraLoaderMixin.load_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5484)

See [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.
#### lora_state_dict[[diffusers.loaders.Krea2LoraLoaderMixin.lora_state_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5423)

See [lora_state_dict()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.
#### save_lora_weights[[diffusers.loaders.Krea2LoraLoaderMixin.save_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5557)

See [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.
#### unfuse_lora[[diffusers.loaders.Krea2LoraLoaderMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_pipeline.py#L5613)

See `unfuse_lora()` for more details.

## LoraBaseMixin[[diffusers.loaders.lora_base.LoraBaseMixin]]

#### diffusers.loaders.lora_base.LoraBaseMixin[[diffusers.loaders.lora_base.LoraBaseMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L479)

Utility class for handling LoRAs.

delete_adaptersdiffusers.loaders.lora_base.LoraBaseMixin.delete_adaptershttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L845[{"name": "adapter_names", "val": ": list[str] | str"}]- **adapter_names** (`list[str, str]`) --
  The names of the adapters to delete.0

Delete an adapter's LoRA layers from the pipeline.

Example:

```py
from diffusers import AutoPipelineForText2Image
import torch

pipeline = AutoPipelineForText2Image.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0", torch_dtype=torch.float16
).to("cuda")
pipeline.load_lora_weights(
    "jbilcke-hf/sdxl-cinematic-1", weight_name="pytorch_lora_weights.safetensors", adapter_names="cinematic"
)
pipeline.delete_adapters("cinematic")
```

**Parameters:**

adapter_names (`list[str, str]`) : The names of the adapters to delete.
#### disable_lora[[diffusers.loaders.lora_base.LoraBaseMixin.disable_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L785)

Disables the active LoRA layers of the pipeline.

Example:

```py
from diffusers import AutoPipelineForText2Image
import torch

pipeline = AutoPipelineForText2Image.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0", torch_dtype=torch.float16
).to("cuda")
pipeline.load_lora_weights(
    "jbilcke-hf/sdxl-cinematic-1", weight_name="pytorch_lora_weights.safetensors", adapter_name="cinematic"
)
pipeline.disable_lora()
```
#### enable_lora[[diffusers.loaders.lora_base.LoraBaseMixin.enable_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L815)

Enables the active LoRA layers of the pipeline.

Example:

```py
from diffusers import AutoPipelineForText2Image
import torch

pipeline = AutoPipelineForText2Image.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0", torch_dtype=torch.float16
).to("cuda")
pipeline.load_lora_weights(
    "jbilcke-hf/sdxl-cinematic-1", weight_name="pytorch_lora_weights.safetensors", adapter_name="cinematic"
)
pipeline.enable_lora()
```
#### enable_lora_hotswap[[diffusers.loaders.lora_base.LoraBaseMixin.enable_lora_hotswap]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L992)

Hotswap adapters without triggering recompilation of a model or if the ranks of the loaded adapters are
different.

**Parameters:**

target_rank (`int`) : The highest rank among all the adapters that will be loaded.

check_compiled (`str`, *optional*, defaults to `"error"`) : How to handle a model that is already compiled. The check can return the following messages: - "error" (default): raise an error - "warn": issue a warning - "ignore": do nothing
#### fuse_lora[[diffusers.loaders.lora_base.LoraBaseMixin.fuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L537)

Fuses the LoRA parameters into the original parameters of the corresponding blocks.

> [!WARNING] > This is an experimental API.

Example:

```py
from diffusers import DiffusionPipeline
import torch

pipeline = DiffusionPipeline.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0", torch_dtype=torch.float16
).to("cuda")
pipeline.load_lora_weights("nerijs/pixel-art-xl", weight_name="pixel-art-xl.safetensors", adapter_name="pixel")
pipeline.fuse_lora(lora_scale=0.7)
```

**Parameters:**

components : (`list[str]`): list of LoRA-injectable components to fuse the LoRAs into.

lora_scale (`float`, defaults to 1.0) : Controls how much to influence the outputs with the LoRA parameters.

safe_fusing (`bool`, defaults to `False`) : Whether to check fused weights for NaN values before fusing and if values are NaN not fusing them.

adapter_names (`list[str]`, *optional*) : Adapter names to be used for fusing. If nothing is passed, all active adapters will be fused.
#### get_active_adapters[[diffusers.loaders.lora_base.LoraBaseMixin.get_active_adapters]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L883)

Gets the list of the current active adapters.

Example:

```python
from diffusers import DiffusionPipeline

pipeline = DiffusionPipeline.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0",
).to("cuda")
pipeline.load_lora_weights("CiroN2022/toy-face", weight_name="toy_face_sdxl.safetensors", adapter_name="toy")
pipeline.get_active_adapters()
```
#### get_list_adapters[[diffusers.loaders.lora_base.LoraBaseMixin.get_list_adapters]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L916)

Gets the current list of all available adapters in the pipeline.
#### set_adapters[[diffusers.loaders.lora_base.LoraBaseMixin.set_adapters]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L682)

Set the currently active adapters for use in the pipeline.

Example:

```py
from diffusers import AutoPipelineForText2Image
import torch

pipeline = AutoPipelineForText2Image.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0", torch_dtype=torch.float16
).to("cuda")
pipeline.load_lora_weights(
    "jbilcke-hf/sdxl-cinematic-1", weight_name="pytorch_lora_weights.safetensors", adapter_name="cinematic"
)
pipeline.load_lora_weights("nerijs/pixel-art-xl", weight_name="pixel-art-xl.safetensors", adapter_name="pixel")
pipeline.set_adapters(["cinematic", "pixel"], adapter_weights=[0.5, 0.5])
```

**Parameters:**

adapter_names (`list[str]` or `str`) : The names of the adapters to use.

adapter_weights (`list[float, float]`, *optional*) : The adapter(s) weights to use with the UNet. If `None`, the weights are set to `1.0` for all the adapters.
#### set_lora_device[[diffusers.loaders.lora_base.LoraBaseMixin.set_lora_device]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L938)

Moves the LoRAs listed in `adapter_names` to a target device. Useful for offloading the LoRA to the CPU in case
you want to load multiple adapters and free some GPU memory.

After offloading the LoRA adapters to CPU, as long as the rest of the model is still on GPU, the LoRA adapters
can no longer be used for inference, as that would cause a device mismatch. Remember to set the device back to
GPU before using those LoRA adapters for inference.

```python
>>> pipe.load_lora_weights(path_1, adapter_name="adapter-1")
>>> pipe.load_lora_weights(path_2, adapter_name="adapter-2")
>>> pipe.set_adapters("adapter-1")
>>> image_1 = pipe(**kwargs)
>>> # switch to adapter-2, offload adapter-1
>>> pipeline.set_lora_device(adapter_names=["adapter-1"], device="cpu")
>>> pipeline.set_lora_device(adapter_names=["adapter-2"], device="cuda:0")
>>> pipe.set_adapters("adapter-2")
>>> image_2 = pipe(**kwargs)
>>> # switch back to adapter-1, offload adapter-2
>>> pipeline.set_lora_device(adapter_names=["adapter-2"], device="cpu")
>>> pipeline.set_lora_device(adapter_names=["adapter-1"], device="cuda:0")
>>> pipe.set_adapters("adapter-1")
>>> ...
```

**Parameters:**

adapter_names (`list[str]`) : list of adapters to send device to.

device (`torch.device | str | int`) : Device to send the adapters to. Can be either a torch device, a str or an integer.
#### unfuse_lora[[diffusers.loaders.lora_base.LoraBaseMixin.unfuse_lora]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L626)

Reverses the effect of
[`pipe.fuse_lora()`](https://huggingface.co/docs/diffusers/main/en/api/loaders#diffusers.loaders.LoraBaseMixin.fuse_lora).

> [!WARNING] > This is an experimental API.

**Parameters:**

components (`list[str]`) : list of LoRA-injectable components to unfuse LoRA from.

unfuse_unet (`bool`, defaults to `True`) : Whether to unfuse the UNet LoRA parameters.

unfuse_text_encoder (`bool`, defaults to `True`) : Whether to unfuse the text encoder LoRA parameters. If the text encoder wasn't monkey-patched with the LoRA parameters then it won't have any effect.
#### unload_lora_weights[[diffusers.loaders.lora_base.LoraBaseMixin.unload_lora_weights]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L514)

Unloads the LoRA parameters.

Examples:

```python
>>> # Assuming `pipeline` is already loaded with the LoRA parameters.
>>> pipeline.unload_lora_weights()
>>> ...
```
#### write_lora_layers[[diffusers.loaders.lora_base.LoraBaseMixin.write_lora_layers]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/lora_base.py#L1015)

Writes the state dict of the LoRA layers (optionally with metadata) to disk.
