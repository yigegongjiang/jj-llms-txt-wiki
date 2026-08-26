# LoRA

LoRA is a fast and lightweight training method that inserts and trains a significantly smaller number of parameters instead of all the model parameters. This produces a smaller file (~100 MBs) and makes it easier to quickly train a model to learn a new concept. LoRA weights are typically loaded into the denoiser, text encoder or both. The denoiser usually corresponds to a UNet ([UNet2DConditionModel](/docs/diffusers/v0.40.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel), for example) or a Transformer ([SD3Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/sd3_transformer2d#diffusers.SD3Transformer2DModel), for example). There are several classes for loading LoRA weights:

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
- `AceStepLoraLoaderMixin` provides similar functions for [ACE-Step](https://huggingface.co/docs/diffusers/main/en/api/pipelines/ace_step).
- `HiDreamImageLoraLoaderMixin` provides similar functions for [HiDream Image](https://huggingface.co/docs/diffusers/main/en/api/pipelines/hidream)
- `QwenImageLoraLoaderMixin` provides similar functions for [Qwen Image](https://huggingface.co/docs/diffusers/main/en/api/pipelines/qwen).
- `ZImageLoraLoaderMixin` provides similar functions for [Z-Image](https://huggingface.co/docs/diffusers/main/en/api/pipelines/zimage).
- `Flux2LoraLoaderMixin` provides similar functions for [Flux2](https://huggingface.co/docs/diffusers/main/en/api/pipelines/flux2).
- `ErnieImageLoraLoaderMixin` provides similar functions for [Ernie-Image](https://huggingface.co/docs/diffusers/main/en/api/pipelines/ernie_image).
- `LTX2LoraLoaderMixin` provides similar functions for [Flux2](https://huggingface.co/docs/diffusers/main/en/api/pipelines/ltx2).
- `MiniMaxH3LoraLoaderMixin` provides similar functions for [MiniMax-H3](https://huggingface.co/docs/diffusers/main/en/api/pipelines/minimax_h3).
- `LoraBaseMixin` provides a base class with several utility methods to fuse, unfuse, unload, LoRAs and more.

> [!TIP]
> To learn more about how to load LoRA weights, see the [LoRA](../../tutorials/using_peft_for_inference) loading guide.

## LoraBaseMixin[[diffusers.loaders.lora_base.LoraBaseMixin]]

#### diffusers.loaders.lora_base.LoraBaseMixin[[diffusers.loaders.lora_base.LoraBaseMixin]]

```python
diffusers.loaders.lora_base.LoraBaseMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L486)

Utility class for handling LoRAs.

#### delete_adapters[[diffusers.loaders.lora_base.LoraBaseMixin.delete_adapters]]

```python
delete_adapters(adapter_names: list[str] | str)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L856)

**Parameters:**

adapter_names (`list[str, str]`) : The names of the adapters to delete.

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

#### disable_lora[[diffusers.loaders.lora_base.LoraBaseMixin.disable_lora]]

```python
disable_lora()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L796)

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

```python
enable_lora()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L826)

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

```python
enable_lora_hotswap(**kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L1003)

**Parameters:**

target_rank (`int`) : The highest rank among all the adapters that will be loaded.

check_compiled (`str`, *optional*, defaults to `"error"`) : How to handle a model that is already compiled. The check can return the following messages: - "error" (default): raise an error - "warn": issue a warning - "ignore": do nothing

Hotswap adapters without triggering recompilation of a model or if the ranks of the loaded adapters are
different.

#### fuse_lora[[diffusers.loaders.lora_base.LoraBaseMixin.fuse_lora]]

```python
fuse_lora(components: list[str] | None = None, lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L544)

**Parameters:**

components : (`list[str]`): list of LoRA-injectable components to fuse the LoRAs into.

lora_scale (`float`, defaults to 1.0) : Controls how much to influence the outputs with the LoRA parameters.

safe_fusing (`bool`, defaults to `False`) : Whether to check fused weights for NaN values before fusing and if values are NaN not fusing them.

adapter_names (`list[str]`, *optional*) : Adapter names to be used for fusing. If nothing is passed, all active adapters will be fused.

Fuses the LoRA parameters into the original parameters of the corresponding blocks.

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

#### get_active_adapters[[diffusers.loaders.lora_base.LoraBaseMixin.get_active_adapters]]

```python
get_active_adapters()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L894)

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

```python
get_list_adapters()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L927)

Gets the current list of all available adapters in the pipeline.

#### set_adapters[[diffusers.loaders.lora_base.LoraBaseMixin.set_adapters]]

```python
set_adapters(adapter_names: list[str] | str, adapter_weights: float | dict | list[float] | list[dict] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L693)

**Parameters:**

adapter_names (`list[str]` or `str`) : The names of the adapters to use.

adapter_weights (`list[float, float]`, *optional*) : The adapter(s) weights to use with the UNet. If `None`, the weights are set to `1.0` for all the adapters.

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

#### set_lora_device[[diffusers.loaders.lora_base.LoraBaseMixin.set_lora_device]]

```python
set_lora_device(adapter_names: list[str], device: torch.device | str | int)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L949)

**Parameters:**

adapter_names (`list[str]`) : list of adapters to send device to.

device (`torch.device | str | int`) : Device to send the adapters to. Can be either a torch device, a str or an integer.

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

#### unfuse_lora[[diffusers.loaders.lora_base.LoraBaseMixin.unfuse_lora]]

```python
unfuse_lora(components: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L631)

**Parameters:**

components (`list[str]`) : list of LoRA-injectable components to unfuse LoRA from.

unfuse_unet (`bool`, defaults to `True`) : Whether to unfuse the UNet LoRA parameters.

unfuse_text_encoder (`bool`, defaults to `True`) : Whether to unfuse the text encoder LoRA parameters. If the text encoder wasn't monkey-patched with the LoRA parameters then it won't have any effect.

Reverses the effect of
[`pipe.fuse_lora()`](https://huggingface.co/docs/diffusers/main/en/api/loaders#diffusers.loaders.LoraBaseMixin.fuse_lora).

#### unload_lora_weights[[diffusers.loaders.lora_base.LoraBaseMixin.unload_lora_weights]]

```python
unload_lora_weights()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L521)

Unloads the LoRA parameters.

Examples:

```python
>>> # Assuming `pipeline` is already loaded with the LoRA parameters.
>>> pipeline.unload_lora_weights()
>>> ...
```

#### write_lora_layers[[diffusers.loaders.lora_base.LoraBaseMixin.write_lora_layers]]

```python
write_lora_layers(state_dict: dict[str, torch.Tensor], save_directory: str, is_main_process: bool, weight_name: str, save_function: Callable, safe_serialization: bool, lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L1026)

Writes the state dict of the LoRA layers (optionally with metadata) to disk.

## StableDiffusionLoraLoaderMixin[[diffusers.loaders.StableDiffusionLoraLoaderMixin]]

#### diffusers.loaders.StableDiffusionLoraLoaderMixin[[diffusers.loaders.StableDiffusionLoraLoaderMixin]]

```python
diffusers.loaders.StableDiffusionLoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L133)

Load LoRA layers into Stable Diffusion [UNet2DConditionModel](/docs/diffusers/v0.40.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel) and
[`CLIPTextModel`](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel).

#### load_lora_into_text_encoder[[diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_text_encoder]]

```python
load_lora_into_text_encoder(state_dict, network_alphas, text_encoder, prefix = None, lora_scale = 1.0, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L403)

**Parameters:**

state_dict (`dict`) : A standard state dict containing the lora layer parameters. The key should be prefixed with an additional `text_encoder` to distinguish between unet lora layers.

network_alphas (`dict[str, float]`) : The value of the network alpha used for stable learning and preventing underflow. This value has the same meaning as the `--network_alpha` option in the kohya-ss trainer script. Refer to [this link](https://github.com/darkstorm2150/sd-scripts/blob/main/docs/train_network_README-en.md#execute-learning).

text_encoder (`CLIPTextModel`) : The text encoder model to load the LoRA layers into.

prefix (`str`) : Expected prefix of the `text_encoder` in the `state_dict`.

lora_scale (`float`) : How much to scale the output of the lora linear layer before it is added with the output of the regular lora layer.

adapter_name (`str`, *optional*) : Adapter name to be used for referencing the loaded adapter model. If not specified, it will use `default_{i}` where i is the total number of adapters being loaded.

low_cpu_mem_usage (`bool`, *optional*) : Speed up model loading by only loading the pretrained LoRA weights and not initializing the random weights.

hotswap (`bool`, *optional*) : See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights).

metadata (`dict`) : Optional LoRA adapter metadata. When supplied, the `LoraConfig` arguments of `peft` won't be derived from the state dict.

This will load the LoRA layers specified in `state_dict` into `text_encoder`

#### load_lora_into_unet[[diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet]]

```python
load_lora_into_unet(state_dict, network_alphas, unet, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L349)

**Parameters:**

state_dict (`dict`) : A standard state dict containing the lora layer parameters. The keys can either be indexed directly into the unet or prefixed with an additional `unet` which can be used to distinguish between text encoder lora layers.

network_alphas (`dict[str, float]`) : The value of the network alpha used for stable learning and preventing underflow. This value has the same meaning as the `--network_alpha` option in the kohya-ss trainer script. Refer to [this link](https://github.com/darkstorm2150/sd-scripts/blob/main/docs/train_network_README-en.md#execute-learning).

unet (`UNet2DConditionModel`) : The UNet model to load the LoRA layers into.

adapter_name (`str`, *optional*) : Adapter name to be used for referencing the loaded adapter model. If not specified, it will use `default_{i}` where i is the total number of adapters being loaded.

low_cpu_mem_usage (`bool`, *optional*) : Speed up model loading only loading the pretrained LoRA weights and not initializing the random weights.

hotswap (`bool`, *optional*) : See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights).

metadata (`dict`) : Optional LoRA adapter metadata. When supplied, the `LoraConfig` arguments of `peft` won't be derived from the state dict.

This will load the LoRA layers specified in `state_dict` into `unet`.

#### load_lora_weights[[diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L143)

**Parameters:**

pretrained_model_name_or_path_or_dict (`str` or `os.PathLike` or `dict`) : See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict).

adapter_name (`str`, *optional*) : Adapter name to be used for referencing the loaded adapter model. If not specified, it will use `default_{i}` where i is the total number of adapters being loaded.

low_cpu_mem_usage (`bool`, *optional*) : Speed up model loading by only loading the pretrained LoRA weights and not initializing the random weights.

hotswap (`bool`, *optional*) : Defaults to `False`. Whether to substitute an existing (LoRA) adapter with the newly loaded adapter in-place. This means that, instead of loading an additional adapter, this will take the existing adapter weights and replace them with the weights of the new adapter. This can be faster and more memory efficient. However, the main advantage of hotswapping is that when the model is compiled with torch.compile, loading the new adapter does not require recompilation of the model. When using hotswapping, the passed `adapter_name` should be the name of an already loaded adapter.  If the new adapter and the old adapter have different ranks and/or LoRA alphas (i.e. scaling), you need to call an additional method before loading the adapter:  ```py pipeline = ...  # load diffusers pipeline max_rank = ...  # the highest rank among all LoRAs that you want to load # call *before* compiling and loading the LoRA adapter pipeline.enable_lora_hotswap(target_rank=max_rank) pipeline.load_lora_weights(file_name) # optionally compile the model now ```  Note that hotswapping adapters of the text encoder is not yet supported. There are some further limitations to this technique, which are documented here: https://huggingface.co/docs/peft/main/en/package_reference/hotswap

kwargs (`dict`, *optional*) : See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict).

Load LoRA weights specified in `pretrained_model_name_or_path_or_dict` into `self.unet` and
`self.text_encoder`.

All kwargs are forwarded to `self.lora_state_dict`.

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details on how the state dict is
loaded.

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details on how the state dict is
loaded into `self.unet`.

See [load_lora_into_text_encoder()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_text_encoder) for more details on how the state
dict is loaded into `self.text_encoder`.

#### lora_state_dict[[diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L238)

**Parameters:**

pretrained_model_name_or_path_or_dict (`str` or `os.PathLike` or `dict`) : Can be either:  - A string, the *model id* (for example `google/ddpm-celebahq-256`) of a pretrained model hosted on the Hub. - A path to a *directory* (for example `./my_model_directory`) containing the model weights saved with [ModelMixin.save_pretrained()](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin.save_pretrained). - A [torch state dict](https://pytorch.org/tutorials/beginner/saving_loading_models.html#what-is-a-state-dict). 

cache_dir (`str | os.PathLike`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist. 

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

local_files_only (`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to `True`, the model won't be downloaded from the Hub.

token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

subfolder (`str`, *optional*, defaults to `""`) : The subfolder location of a model file within a larger model repository on the Hub or locally.

weight_name (`str`, *optional*, defaults to None) : Name of the serialized state dict file.

return_lora_metadata (`bool`, *optional*, defaults to False) : When enabled, additionally return the LoRA adapter metadata, typically found in the state dict.

Return state dict for lora weights and the network alphas.

> [!WARNING] > We support loading A1111 formatted LoRA checkpoints in a limited capacity. > > This function is
experimental and might change in the future.

#### save_lora_weights[[diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, unet_lora_layers: dict = None, text_encoder_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, unet_lora_adapter_metadata = None, text_encoder_lora_adapter_metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L461)

**Parameters:**

save_directory (`str` or `os.PathLike`) : Directory to save LoRA parameters to. Will be created if it doesn't exist.

unet_lora_layers (`dict[str, torch.nn.Module]` or `dict[str, torch.Tensor]`) : State dict of the LoRA layers corresponding to the `unet`.

text_encoder_lora_layers (`dict[str, torch.nn.Module]` or `dict[str, torch.Tensor]`) : State dict of the LoRA layers corresponding to the `text_encoder`. Must explicitly pass the text encoder LoRA state dict because it comes from 🤗 Transformers.

is_main_process (`bool`, *optional*, defaults to `True`) : Whether the process calling this is the main process or not. Useful during distributed training and you need to call this function on all processes. In this case, set `is_main_process=True` only on the main process to avoid race conditions.

save_function (`Callable`) : The function to use to save the state dictionary. Useful during distributed training when you need to replace `torch.save` with another method. Can be configured with the environment variable `DIFFUSERS_SAVE_MODE`.

safe_serialization (`bool`, *optional*, defaults to `True`) : Whether to save the model using `safetensors` or the traditional PyTorch way with `pickle`.

unet_lora_adapter_metadata : LoRA adapter metadata associated with the unet to be serialized with the state dict.

text_encoder_lora_adapter_metadata : LoRA adapter metadata associated with the text encoder to be serialized with the state dict.

Save the LoRA parameters corresponding to the UNet and text encoder.

## StableDiffusionXLLoraLoaderMixin[[diffusers.loaders.StableDiffusionXLLoraLoaderMixin]]

#### diffusers.loaders.StableDiffusionXLLoraLoaderMixin[[diffusers.loaders.StableDiffusionXLLoraLoaderMixin]]

```python
diffusers.loaders.StableDiffusionXLLoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L580)

Load LoRA layers into Stable Diffusion XL [UNet2DConditionModel](/docs/diffusers/v0.40.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel),
[`CLIPTextModel`](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), and
[`CLIPTextModelWithProjection`](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModelWithProjection).

#### fuse_lora[[diffusers.loaders.StableDiffusionXLLoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['unet', 'text_encoder', 'text_encoder_2'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L933)

See `fuse_lora()` for more details.

#### load_lora_into_text_encoder[[diffusers.loaders.StableDiffusionXLLoraLoaderMixin.load_lora_into_text_encoder]]

```python
load_lora_into_text_encoder(state_dict, network_alphas, text_encoder, prefix = None, lora_scale = 1.0, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L826)

**Parameters:**

state_dict (`dict`) : A standard state dict containing the lora layer parameters. The key should be prefixed with an additional `text_encoder` to distinguish between unet lora layers.

network_alphas (`dict[str, float]`) : The value of the network alpha used for stable learning and preventing underflow. This value has the same meaning as the `--network_alpha` option in the kohya-ss trainer script. Refer to [this link](https://github.com/darkstorm2150/sd-scripts/blob/main/docs/train_network_README-en.md#execute-learning).

text_encoder (`CLIPTextModel`) : The text encoder model to load the LoRA layers into.

prefix (`str`) : Expected prefix of the `text_encoder` in the `state_dict`.

lora_scale (`float`) : How much to scale the output of the lora linear layer before it is added with the output of the regular lora layer.

adapter_name (`str`, *optional*) : Adapter name to be used for referencing the loaded adapter model. If not specified, it will use `default_{i}` where i is the total number of adapters being loaded.

low_cpu_mem_usage (`bool`, *optional*) : Speed up model loading by only loading the pretrained LoRA weights and not initializing the random weights.

hotswap (`bool`, *optional*) : See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights).

metadata (`dict`) : Optional LoRA adapter metadata. When supplied, the `LoraConfig` arguments of `peft` won't be derived from the state dict.

This will load the LoRA layers specified in `state_dict` into `text_encoder`

#### load_lora_into_unet[[diffusers.loaders.StableDiffusionXLLoraLoaderMixin.load_lora_into_unet]]

```python
load_lora_into_unet(state_dict, network_alphas, unet, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L771)

**Parameters:**

state_dict (`dict`) : A standard state dict containing the lora layer parameters. The keys can either be indexed directly into the unet or prefixed with an additional `unet` which can be used to distinguish between text encoder lora layers.

network_alphas (`dict[str, float]`) : The value of the network alpha used for stable learning and preventing underflow. This value has the same meaning as the `--network_alpha` option in the kohya-ss trainer script. Refer to [this link](https://github.com/darkstorm2150/sd-scripts/blob/main/docs/train_network_README-en.md#execute-learning).

unet (`UNet2DConditionModel`) : The UNet model to load the LoRA layers into.

adapter_name (`str`, *optional*) : Adapter name to be used for referencing the loaded adapter model. If not specified, it will use `default_{i}` where i is the total number of adapters being loaded.

low_cpu_mem_usage (`bool`, *optional*) : Speed up model loading only loading the pretrained LoRA weights and not initializing the random weights.

hotswap (`bool`, *optional*) : See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights).

metadata (`dict`) : Optional LoRA adapter metadata. When supplied, the `LoraConfig` arguments of `peft` won't be derived from the state dict.

This will load the LoRA layers specified in `state_dict` into `unet`.

#### load_lora_weights[[diffusers.loaders.StableDiffusionXLLoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L591)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.StableDiffusionXLLoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L659)

**Parameters:**

pretrained_model_name_or_path_or_dict (`str` or `os.PathLike` or `dict`) : Can be either:  - A string, the *model id* (for example `google/ddpm-celebahq-256`) of a pretrained model hosted on the Hub. - A path to a *directory* (for example `./my_model_directory`) containing the model weights saved with [ModelMixin.save_pretrained()](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin.save_pretrained). - A [torch state dict](https://pytorch.org/tutorials/beginner/saving_loading_models.html#what-is-a-state-dict). 

cache_dir (`str | os.PathLike`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist. 

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

local_files_only (`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to `True`, the model won't be downloaded from the Hub.

token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

subfolder (`str`, *optional*, defaults to `""`) : The subfolder location of a model file within a larger model repository on the Hub or locally.

weight_name (`str`, *optional*, defaults to None) : Name of the serialized state dict file.

return_lora_metadata (`bool`, *optional*, defaults to False) : When enabled, additionally return the LoRA adapter metadata, typically found in the state dict.

Return state dict for lora weights and the network alphas.

> [!WARNING] > We support loading A1111 formatted LoRA checkpoints in a limited capacity. > > This function is
experimental and might change in the future.

#### save_lora_weights[[diffusers.loaders.StableDiffusionXLLoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, unet_lora_layers: dict = None, text_encoder_lora_layers: dict = None, text_encoder_2_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, unet_lora_adapter_metadata = None, text_encoder_lora_adapter_metadata = None, text_encoder_2_lora_adapter_metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L885)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.StableDiffusionXLLoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['unet', 'text_encoder', 'text_encoder_2'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L952)

See `unfuse_lora()` for more details.

## SD3LoraLoaderMixin[[diffusers.loaders.SD3LoraLoaderMixin]]

#### diffusers.loaders.SD3LoraLoaderMixin[[diffusers.loaders.SD3LoraLoaderMixin]]

```python
diffusers.loaders.SD3LoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L959)

Load LoRA layers into [SD3Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/sd3_transformer2d#diffusers.SD3Transformer2DModel),
[`CLIPTextModel`](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), and
[`CLIPTextModelWithProjection`](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModelWithProjection).

Specific to [StableDiffusion3Pipeline](/docs/diffusers/v0.40.0/en/api/pipelines/stable_diffusion/stable_diffusion_3#diffusers.StableDiffusion3Pipeline).

#### fuse_lora[[diffusers.loaders.SD3LoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer', 'text_encoder', 'text_encoder_2'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L1220)

See `fuse_lora()` for more details.

#### load_lora_into_text_encoder[[diffusers.loaders.SD3LoraLoaderMixin.load_lora_into_text_encoder]]

```python
load_lora_into_text_encoder(state_dict, network_alphas, text_encoder, prefix = None, lora_scale = 1.0, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L1111)

**Parameters:**

state_dict (`dict`) : A standard state dict containing the lora layer parameters. The key should be prefixed with an additional `text_encoder` to distinguish between unet lora layers.

network_alphas (`dict[str, float]`) : The value of the network alpha used for stable learning and preventing underflow. This value has the same meaning as the `--network_alpha` option in the kohya-ss trainer script. Refer to [this link](https://github.com/darkstorm2150/sd-scripts/blob/main/docs/train_network_README-en.md#execute-learning).

text_encoder (`CLIPTextModel`) : The text encoder model to load the LoRA layers into.

prefix (`str`) : Expected prefix of the `text_encoder` in the `state_dict`.

lora_scale (`float`) : How much to scale the output of the lora linear layer before it is added with the output of the regular lora layer.

adapter_name (`str`, *optional*) : Adapter name to be used for referencing the loaded adapter model. If not specified, it will use `default_{i}` where i is the total number of adapters being loaded.

low_cpu_mem_usage (`bool`, *optional*) : Speed up model loading by only loading the pretrained LoRA weights and not initializing the random weights.

hotswap (`bool`, *optional*) : See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights).

metadata (`dict`) : Optional LoRA adapter metadata. When supplied, the `LoraConfig` arguments of `peft` won't be derived from the state dict.

This will load the LoRA layers specified in `state_dict` into `text_encoder`

#### load_lora_into_transformer[[diffusers.loaders.SD3LoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L1085)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.SD3LoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L1026)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.SD3LoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L972)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.SD3LoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, text_encoder_lora_layers: dict = None, text_encoder_2_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata = None, text_encoder_lora_adapter_metadata = None, text_encoder_2_lora_adapter_metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L1170)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.SD3LoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer', 'text_encoder', 'text_encoder_2'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L1240)

See `unfuse_lora()` for more details.

## FluxLoraLoaderMixin[[diffusers.loaders.FluxLoraLoaderMixin]]

#### diffusers.loaders.FluxLoraLoaderMixin[[diffusers.loaders.FluxLoraLoaderMixin]]

```python
diffusers.loaders.FluxLoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L1436)

Load LoRA layers into [FluxTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/flux_transformer#diffusers.FluxTransformer2DModel),
[`CLIPTextModel`](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel).

Specific to [FluxPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/flux#diffusers.FluxPipeline).

#### fuse_lora[[diffusers.loaders.FluxLoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L1874)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### load_lora_into_text_encoder[[diffusers.loaders.FluxLoraLoaderMixin.load_lora_into_text_encoder]]

```python
load_lora_into_text_encoder(state_dict, network_alphas, text_encoder, prefix = None, lora_scale = 1.0, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L1751)

**Parameters:**

state_dict (`dict`) : A standard state dict containing the lora layer parameters. The key should be prefixed with an additional `text_encoder` to distinguish between unet lora layers.

network_alphas (`dict[str, float]`) : The value of the network alpha used for stable learning and preventing underflow. This value has the same meaning as the `--network_alpha` option in the kohya-ss trainer script. Refer to [this link](https://github.com/darkstorm2150/sd-scripts/blob/main/docs/train_network_README-en.md#execute-learning).

text_encoder (`CLIPTextModel`) : The text encoder model to load the LoRA layers into.

prefix (`str`) : Expected prefix of the `text_encoder` in the `state_dict`.

lora_scale (`float`) : How much to scale the output of the lora linear layer before it is added with the output of the regular lora layer.

adapter_name (`str`, *optional*) : Adapter name to be used for referencing the loaded adapter model. If not specified, it will use `default_{i}` where i is the total number of adapters being loaded.

low_cpu_mem_usage (`bool`, *optional*) : Speed up model loading by only loading the pretrained LoRA weights and not initializing the random weights.

hotswap (`bool`, *optional*) : See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights).

metadata (`dict`) : Optional LoRA adapter metadata. When supplied, the `LoraConfig` arguments of `peft` won't be derived from the state dict.

This will load the LoRA layers specified in `state_dict` into `text_encoder`

#### load_lora_into_transformer[[diffusers.loaders.FluxLoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, network_alphas, transformer, adapter_name = None, metadata = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L1670)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.FluxLoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L1574)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.FluxLoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], return_alphas: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L1449)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.FluxLoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, text_encoder_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata = None, text_encoder_lora_adapter_metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L1810)

**Parameters:**

save_directory (`str` or `os.PathLike`) : Directory to save LoRA parameters to. Will be created if it doesn't exist.

transformer_lora_layers (`dict[str, torch.nn.Module]` or `dict[str, torch.Tensor]`) : State dict of the LoRA layers corresponding to the `transformer`.

text_encoder_lora_layers (`dict[str, torch.nn.Module]` or `dict[str, torch.Tensor]`) : State dict of the LoRA layers corresponding to the `text_encoder`. Must explicitly pass the text encoder LoRA state dict because it comes from 🤗 Transformers.

is_main_process (`bool`, *optional*, defaults to `True`) : Whether the process calling this is the main process or not. Useful during distributed training and you need to call this function on all processes. In this case, set `is_main_process=True` only on the main process to avoid race conditions.

save_function (`Callable`) : The function to use to save the state dictionary. Useful during distributed training when you need to replace `torch.save` with another method. Can be configured with the environment variable `DIFFUSERS_SAVE_MODE`.

safe_serialization (`bool`, *optional*, defaults to `True`) : Whether to save the model using `safetensors` or the traditional PyTorch way with `pickle`.

transformer_lora_adapter_metadata : LoRA adapter metadata associated with the transformer to be serialized with the state dict.

text_encoder_lora_adapter_metadata : LoRA adapter metadata associated with the text encoder to be serialized with the state dict.

Save the LoRA parameters corresponding to the UNet and text encoder.

#### unfuse_lora[[diffusers.loaders.FluxLoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer', 'text_encoder'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L1906)

**Parameters:**

components (`list[str]`) : list of LoRA-injectable components to unfuse LoRA from.

Reverses the effect of
[`pipe.fuse_lora()`](https://huggingface.co/docs/diffusers/main/en/api/loaders#diffusers.loaders.LoraBaseMixin.fuse_lora).

#### unload_lora_weights[[diffusers.loaders.FluxLoraLoaderMixin.unload_lora_weights]]

```python
unload_lora_weights(reset_to_overwritten_params = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L1921)

**Parameters:**

reset_to_overwritten_params (`bool`, defaults to `False`) : Whether to reset the LoRA-loaded modules to their original params. Refer to the [Flux documentation](https://huggingface.co/docs/diffusers/main/en/api/pipelines/flux) to learn more.

Unloads the LoRA parameters.

Examples:

```python
>>> # Assuming `pipeline` is already loaded with the LoRA parameters.
>>> pipeline.unload_lora_weights()
>>> ...
```

## Flux2LoraLoaderMixin[[diffusers.loaders.Flux2LoraLoaderMixin]]

#### diffusers.loaders.Flux2LoraLoaderMixin[[diffusers.loaders.Flux2LoraLoaderMixin]]

```python
diffusers.loaders.Flux2LoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5763)

Load LoRA layers into [Flux2Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/flux2_transformer#diffusers.Flux2Transformer2DModel). Specific to [Flux2Pipeline](/docs/diffusers/v0.40.0/en/api/pipelines/flux2#diffusers.Flux2Pipeline).

#### fuse_lora[[diffusers.loaders.Flux2LoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5939)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.Flux2LoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5876)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.Flux2LoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5840)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.Flux2LoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5771)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.Flux2LoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5903)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.Flux2LoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5959)

See `unfuse_lora()` for more details.

## ErnieImageLoraLoaderMixin[[diffusers.loaders.ErnieImageLoraLoaderMixin]]

#### diffusers.loaders.ErnieImageLoraLoaderMixin[[diffusers.loaders.ErnieImageLoraLoaderMixin]]

```python
diffusers.loaders.ErnieImageLoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6162)

Load LoRA layers into [ErnieImageTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/ernie_image_transformer2d#diffusers.ErnieImageTransformer2DModel). Specific to `ErnieImagePipeline`.

#### fuse_lora[[diffusers.loaders.ErnieImageLoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6335)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.ErnieImageLoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6272)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.ErnieImageLoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6236)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.ErnieImageLoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6170)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.ErnieImageLoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6299)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.ErnieImageLoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6355)

See `unfuse_lora()` for more details.

## LTX2LoraLoaderMixin[[diffusers.loaders.LTX2LoraLoaderMixin]]

#### diffusers.loaders.LTX2LoraLoaderMixin[[diffusers.loaders.LTX2LoraLoaderMixin]]

```python
diffusers.loaders.LTX2LoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2914)

Load LoRA layers into [LTX2VideoTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/ltx2_video_transformer3d#diffusers.LTX2VideoTransformer3DModel). Specific to [LTX2Pipeline](/docs/diffusers/v0.40.0/en/api/pipelines/ltx2#diffusers.LTX2Pipeline).

#### fuse_lora[[diffusers.loaders.LTX2LoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3103)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.LTX2LoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None, prefix: str = 'transformer')
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3039)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.LTX2LoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2987)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.LTX2LoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2923)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.LTX2LoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3067)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.LTX2LoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3123)

See `unfuse_lora()` for more details.

## CogVideoXLoraLoaderMixin[[diffusers.loaders.CogVideoXLoraLoaderMixin]]

#### diffusers.loaders.CogVideoXLoraLoaderMixin[[diffusers.loaders.CogVideoXLoraLoaderMixin]]

```python
diffusers.loaders.CogVideoXLoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2348)

Load LoRA layers into [CogVideoXTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/cogvideox_transformer3d#diffusers.CogVideoXTransformer3DModel). Specific to [CogVideoXPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/cogvideox#diffusers.CogVideoXPipeline).

#### fuse_lora[[diffusers.loaders.CogVideoXLoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2507)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.CogVideoXLoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2446)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.CogVideoXLoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2411)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.CogVideoXLoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2356)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.CogVideoXLoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2473)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.CogVideoXLoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2526)

See `unfuse_lora()` for more details.

## Mochi1LoraLoaderMixin[[diffusers.loaders.Mochi1LoraLoaderMixin]]

#### diffusers.loaders.Mochi1LoraLoaderMixin[[diffusers.loaders.Mochi1LoraLoaderMixin]]

```python
diffusers.loaders.Mochi1LoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2533)

Load LoRA layers into [MochiTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/mochi_transformer3d#diffusers.MochiTransformer3DModel). Specific to [MochiPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/mochi#diffusers.MochiPipeline).

#### fuse_lora[[diffusers.loaders.Mochi1LoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2695)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.Mochi1LoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2632)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.Mochi1LoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2596)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.Mochi1LoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2541)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.Mochi1LoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2659)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.Mochi1LoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2715)

See `unfuse_lora()` for more details.

## AuraFlowLoraLoaderMixin[[diffusers.loaders.AuraFlowLoraLoaderMixin]]

#### diffusers.loaders.AuraFlowLoraLoaderMixin[[diffusers.loaders.AuraFlowLoraLoaderMixin]]

```python
diffusers.loaders.AuraFlowLoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L1247)

Load LoRA layers into [AuraFlowTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/aura_flow_transformer2d#diffusers.AuraFlowTransformer2DModel) Specific to [AuraFlowPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/aura_flow#diffusers.AuraFlowPipeline).

#### fuse_lora[[diffusers.loaders.AuraFlowLoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L1409)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.AuraFlowLoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L1346)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.AuraFlowLoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L1310)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.AuraFlowLoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L1255)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.AuraFlowLoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L1373)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.AuraFlowLoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer', 'text_encoder'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L1429)

See `unfuse_lora()` for more details.

## LTXVideoLoraLoaderMixin[[diffusers.loaders.LTXVideoLoraLoaderMixin]]

#### diffusers.loaders.LTXVideoLoraLoaderMixin[[diffusers.loaders.LTXVideoLoraLoaderMixin]]

```python
diffusers.loaders.LTXVideoLoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2722)

Load LoRA layers into [LTXVideoTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/ltx_video_transformer3d#diffusers.LTXVideoTransformer3DModel). Specific to [LTXPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/ltx_video#diffusers.LTXPipeline).

#### fuse_lora[[diffusers.loaders.LTXVideoLoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2887)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.LTXVideoLoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2824)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.LTXVideoLoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2788)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.LTXVideoLoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2730)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.LTXVideoLoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2851)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.LTXVideoLoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2907)

See `unfuse_lora()` for more details.

## SanaLoraLoaderMixin[[diffusers.loaders.SanaLoraLoaderMixin]]

#### diffusers.loaders.SanaLoraLoaderMixin[[diffusers.loaders.SanaLoraLoaderMixin]]

```python
diffusers.loaders.SanaLoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3130)

Load LoRA layers into [SanaTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/sana_transformer2d#diffusers.SanaTransformer2DModel). Specific to [SanaPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/sana#diffusers.SanaPipeline).

#### fuse_lora[[diffusers.loaders.SanaLoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3292)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.SanaLoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3229)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.SanaLoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3193)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.SanaLoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3138)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.SanaLoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3256)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.SanaLoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3312)

See `unfuse_lora()` for more details.

## HeliosLoraLoaderMixin[[diffusers.loaders.HeliosLoraLoaderMixin]]

#### diffusers.loaders.HeliosLoraLoaderMixin[[diffusers.loaders.HeliosLoraLoaderMixin]]

```python
diffusers.loaders.HeliosLoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3319)

Load LoRA layers into [HeliosTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/helios_transformer3d#diffusers.HeliosTransformer3DModel). Specific to [HeliosPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/helios#diffusers.HeliosPipeline) and [HeliosPyramidPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/helios#diffusers.HeliosPyramidPipeline).

#### fuse_lora[[diffusers.loaders.HeliosLoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3482)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.HeliosLoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3419)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.HeliosLoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3385)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.HeliosLoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3327)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.HeliosLoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3446)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.HeliosLoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3502)

See `unfuse_lora()` for more details.

## HunyuanVideoLoraLoaderMixin[[diffusers.loaders.HunyuanVideoLoraLoaderMixin]]

#### diffusers.loaders.HunyuanVideoLoraLoaderMixin[[diffusers.loaders.HunyuanVideoLoraLoaderMixin]]

```python
diffusers.loaders.HunyuanVideoLoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3509)

Load LoRA layers into [HunyuanVideoTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/hunyuan_video_transformer_3d#diffusers.HunyuanVideoTransformer3DModel). Specific to [HunyuanVideoPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/hunyuan_video#diffusers.HunyuanVideoPipeline).

#### fuse_lora[[diffusers.loaders.HunyuanVideoLoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3674)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.HunyuanVideoLoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3611)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.HunyuanVideoLoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3575)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.HunyuanVideoLoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3517)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.HunyuanVideoLoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3638)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.HunyuanVideoLoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3694)

See `unfuse_lora()` for more details.

## Lumina2LoraLoaderMixin[[diffusers.loaders.Lumina2LoraLoaderMixin]]

#### diffusers.loaders.Lumina2LoraLoaderMixin[[diffusers.loaders.Lumina2LoraLoaderMixin]]

```python
diffusers.loaders.Lumina2LoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3701)

Load LoRA layers into [Lumina2Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/lumina2_transformer2d#diffusers.Lumina2Transformer2DModel). Specific to `Lumina2Text2ImgPipeline`.

#### fuse_lora[[diffusers.loaders.Lumina2LoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3867)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.Lumina2LoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3804)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.Lumina2LoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3768)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.Lumina2LoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3709)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.Lumina2LoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3831)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.Lumina2LoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3887)

See `unfuse_lora()` for more details.

## CogView4LoraLoaderMixin[[diffusers.loaders.CogView4LoraLoaderMixin]]

#### diffusers.loaders.CogView4LoraLoaderMixin[[diffusers.loaders.CogView4LoraLoaderMixin]]

```python
diffusers.loaders.CogView4LoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4612)

Load LoRA layers into [WanTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/wan_transformer_3d#diffusers.WanTransformer3DModel). Specific to [CogView4Pipeline](/docs/diffusers/v0.40.0/en/api/pipelines/cogview4#diffusers.CogView4Pipeline).

#### fuse_lora[[diffusers.loaders.CogView4LoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4774)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.CogView4LoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4711)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.CogView4LoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4675)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.CogView4LoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4620)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.CogView4LoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4738)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.CogView4LoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4794)

See `unfuse_lora()` for more details.

## WanLoraLoaderMixin[[diffusers.loaders.WanLoraLoaderMixin]]

#### diffusers.loaders.WanLoraLoaderMixin[[diffusers.loaders.WanLoraLoaderMixin]]

```python
diffusers.loaders.WanLoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4083)

Load LoRA layers into [WanTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/wan_transformer_3d#diffusers.WanTransformer3DModel). Specific to [WanPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/wan#diffusers.WanPipeline) and `[WanImageToVideoPipeline`].

#### fuse_lora[[diffusers.loaders.WanLoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4319)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.WanLoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4256)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.WanLoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4197)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.WanLoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4091)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.WanLoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4283)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.WanLoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4339)

See `unfuse_lora()` for more details.

## SkyReelsV2LoraLoaderMixin[[diffusers.loaders.SkyReelsV2LoraLoaderMixin]]

#### diffusers.loaders.SkyReelsV2LoraLoaderMixin[[diffusers.loaders.SkyReelsV2LoraLoaderMixin]]

```python
diffusers.loaders.SkyReelsV2LoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4346)

Load LoRA layers into [SkyReelsV2Transformer3DModel](/docs/diffusers/v0.40.0/en/api/models/skyreels_v2_transformer_3d#diffusers.SkyReelsV2Transformer3DModel).

#### fuse_lora[[diffusers.loaders.SkyReelsV2LoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4585)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.SkyReelsV2LoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4522)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.SkyReelsV2LoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4462)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.SkyReelsV2LoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4354)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.SkyReelsV2LoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4549)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.SkyReelsV2LoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4605)

See `unfuse_lora()` for more details.

## AmusedLoraLoaderMixin[[diffusers.loaders.AmusedLoraLoaderMixin]]

#### diffusers.loaders.AmusedLoraLoaderMixin[[diffusers.loaders.AmusedLoraLoaderMixin]]

```python
diffusers.loaders.AmusedLoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2201)

#### load_lora_into_transformer[[diffusers.loaders.AmusedLoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, network_alphas, transformer, adapter_name = None, metadata = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2206)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### save_lora_weights[[diffusers.loaders.AmusedLoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, text_encoder_lora_layers: dict = None, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L2293)

**Parameters:**

save_directory (`str` or `os.PathLike`) : Directory to save LoRA parameters to. Will be created if it doesn't exist.

unet_lora_layers (`dict[str, torch.nn.Module]` or `dict[str, torch.Tensor]`) : State dict of the LoRA layers corresponding to the `unet`.

text_encoder_lora_layers (`dict[str, torch.nn.Module]` or `dict[str, torch.Tensor]`) : State dict of the LoRA layers corresponding to the `text_encoder`. Must explicitly pass the text encoder LoRA state dict because it comes from 🤗 Transformers.

is_main_process (`bool`, *optional*, defaults to `True`) : Whether the process calling this is the main process or not. Useful during distributed training and you need to call this function on all processes. In this case, set `is_main_process=True` only on the main process to avoid race conditions.

save_function (`Callable`) : The function to use to save the state dictionary. Useful during distributed training when you need to replace `torch.save` with another method. Can be configured with the environment variable `DIFFUSERS_SAVE_MODE`.

safe_serialization (`bool`, *optional*, defaults to `True`) : Whether to save the model using `safetensors` or the traditional PyTorch way with `pickle`.

Save the LoRA parameters corresponding to the UNet and text encoder.

## AnimaLoraLoaderMixin[[diffusers.loaders.AnimaLoraLoaderMixin]]

#### diffusers.loaders.AnimaLoraLoaderMixin[[diffusers.loaders.AnimaLoraLoaderMixin]]

```python
diffusers.loaders.AnimaLoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5577)

Load LoRA layers into [CosmosTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/cosmos_transformer3d#diffusers.CosmosTransformer3DModel) and [AnimaTextConditioner](/docs/diffusers/v0.40.0/en/api/pipelines/anima#diffusers.AnimaTextConditioner).

#### fuse_lora[[diffusers.loaders.AnimaLoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer', 'text_conditioner'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5737)

See `fuse_lora()` for more details.

#### load_lora_weights[[diffusers.loaders.AnimaLoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5642)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.AnimaLoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5586)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### unfuse_lora[[diffusers.loaders.AnimaLoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer', 'text_conditioner'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5756)

See `unfuse_lora()` for more details.

## AceStepLoraLoaderMixin[[diffusers.loaders.AceStepLoraLoaderMixin]]

#### diffusers.loaders.AceStepLoraLoaderMixin[[diffusers.loaders.AceStepLoraLoaderMixin]]

```python
diffusers.loaders.AceStepLoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6552)

Load LoRA layers into [AceStepTransformer1DModel](/docs/diffusers/v0.40.0/en/api/models/ace_step_transformer#diffusers.AceStepTransformer1DModel). Specific to [AceStepPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/ace_step#diffusers.AceStepPipeline).

#### fuse_lora[[diffusers.loaders.AceStepLoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6716)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.AceStepLoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6653)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.AceStepLoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6617)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.AceStepLoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6560)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.AceStepLoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6680)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.AceStepLoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6736)

See `unfuse_lora()` for more details.

## HiDreamImageLoraLoaderMixin[[diffusers.loaders.HiDreamImageLoraLoaderMixin]]

#### diffusers.loaders.HiDreamImageLoraLoaderMixin[[diffusers.loaders.HiDreamImageLoraLoaderMixin]]

```python
diffusers.loaders.HiDreamImageLoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4801)

Load LoRA layers into [HiDreamImageTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/hidream_image_transformer#diffusers.HiDreamImageTransformer2DModel). Specific to [HiDreamImagePipeline](/docs/diffusers/v0.40.0/en/api/pipelines/hidream#diffusers.HiDreamImagePipeline).

#### fuse_lora[[diffusers.loaders.HiDreamImageLoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4966)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.HiDreamImageLoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4903)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.HiDreamImageLoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4867)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.HiDreamImageLoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4809)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.HiDreamImageLoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4930)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.HiDreamImageLoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4986)

See `unfuse_lora()` for more details.

## QwenImageLoraLoaderMixin[[diffusers.loaders.QwenImageLoraLoaderMixin]]

#### diffusers.loaders.QwenImageLoraLoaderMixin[[diffusers.loaders.QwenImageLoraLoaderMixin]]

```python
diffusers.loaders.QwenImageLoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4993)

Load LoRA layers into [QwenImageTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/qwenimage_transformer2d#diffusers.QwenImageTransformer2DModel). Specific to [QwenImagePipeline](/docs/diffusers/v0.40.0/en/api/pipelines/qwenimage#diffusers.QwenImagePipeline).

#### fuse_lora[[diffusers.loaders.QwenImageLoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5161)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.QwenImageLoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5098)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.QwenImageLoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5062)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.QwenImageLoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5001)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.QwenImageLoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5125)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.QwenImageLoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5181)

See `unfuse_lora()` for more details.

## ZImageLoraLoaderMixin[[diffusers.loaders.ZImageLoraLoaderMixin]]

#### diffusers.loaders.ZImageLoraLoaderMixin[[diffusers.loaders.ZImageLoraLoaderMixin]]

```python
diffusers.loaders.ZImageLoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5382)

Load LoRA layers into [ZImageTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/z_image_transformer2d#diffusers.ZImageTransformer2DModel). Specific to [ZImagePipeline](/docs/diffusers/v0.40.0/en/api/pipelines/z_image#diffusers.ZImagePipeline).

#### fuse_lora[[diffusers.loaders.ZImageLoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5550)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.ZImageLoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5487)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.ZImageLoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5451)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.ZImageLoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5390)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.ZImageLoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5514)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.ZImageLoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5570)

See `unfuse_lora()` for more details.

## CosmosLoraLoaderMixin[[diffusers.loaders.CosmosLoraLoaderMixin]]

#### diffusers.loaders.CosmosLoraLoaderMixin[[diffusers.loaders.CosmosLoraLoaderMixin]]

```python
diffusers.loaders.CosmosLoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6362)

Load LoRA layers into [CosmosTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/cosmos_transformer3d#diffusers.CosmosTransformer3DModel), Specific to [Cosmos2_5_PredictBasePipeline](/docs/diffusers/v0.40.0/en/api/pipelines/cosmos#diffusers.Cosmos2_5_PredictBasePipeline).

#### fuse_lora[[diffusers.loaders.CosmosLoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6525)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.CosmosLoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6462)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.CosmosLoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6426)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.CosmosLoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6371)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.CosmosLoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6489)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.CosmosLoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6545)

See `unfuse_lora()` for more details.

## KandinskyLoraLoaderMixin[[diffusers.loaders.KandinskyLoraLoaderMixin]]

#### diffusers.loaders.KandinskyLoraLoaderMixin[[diffusers.loaders.KandinskyLoraLoaderMixin]]

```python
diffusers.loaders.KandinskyLoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3894)

Load LoRA layers into `Kandinsky5Transformer3DModel`,

#### fuse_lora[[diffusers.loaders.KandinskyLoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4056)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.KandinskyLoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3993)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.KandinskyLoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3957)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.KandinskyLoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L3902)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.KandinskyLoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4020)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.KandinskyLoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L4076)

See `unfuse_lora()` for more details.

## Ideogram4LoraLoaderMixin[[diffusers.loaders.Ideogram4LoraLoaderMixin]]

#### diffusers.loaders.Ideogram4LoraLoaderMixin[[diffusers.loaders.Ideogram4LoraLoaderMixin]]

```python
diffusers.loaders.Ideogram4LoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5966)

Load LoRA layers into [Ideogram4Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/ideogram4_transformer2d#diffusers.Ideogram4Transformer2DModel). Specific to [Ideogram4Pipeline](/docs/diffusers/v0.40.0/en/api/pipelines/ideogram4#diffusers.Ideogram4Pipeline).

#### fuse_lora[[diffusers.loaders.Ideogram4LoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6135)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.Ideogram4LoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6072)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.Ideogram4LoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6036)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.Ideogram4LoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5974)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.Ideogram4LoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6099)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.Ideogram4LoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6155)

See `unfuse_lora()` for more details.

## Krea2LoraLoaderMixin[[diffusers.loaders.Krea2LoraLoaderMixin]]

#### diffusers.loaders.Krea2LoraLoaderMixin[[diffusers.loaders.Krea2LoraLoaderMixin]]

```python
diffusers.loaders.Krea2LoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5188)

Load LoRA layers into [Krea2Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/krea2_transformer2d#diffusers.Krea2Transformer2DModel). Specific to [Krea2Pipeline](/docs/diffusers/v0.40.0/en/api/pipelines/krea2#diffusers.Krea2Pipeline).

#### fuse_lora[[diffusers.loaders.Krea2LoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5355)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.Krea2LoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5292)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_weights[[diffusers.loaders.Krea2LoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5256)

See [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.Krea2LoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5196)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.Krea2LoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5319)

See [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.Krea2LoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L5375)

See `unfuse_lora()` for more details.

## MiniMaxH3LoraLoaderMixin[[diffusers.loaders.MiniMaxH3LoraLoaderMixin]]

#### diffusers.loaders.MiniMaxH3LoraLoaderMixin[[diffusers.loaders.MiniMaxH3LoraLoaderMixin]]

```python
diffusers.loaders.MiniMaxH3LoraLoaderMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6743)

Load LoRA layers into [MiniMaxH3Transformer3DModel](/docs/diffusers/v0.40.0/en/api/models/minimax_h3_transformer3d#diffusers.MiniMaxH3Transformer3DModel). Specific to [MiniMaxH3ModularPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/minimax_h3#diffusers.MiniMaxH3ModularPipeline).

MiniMax-H3 ships two independent DiT partitions with identical module names (a LoRA for one loads into the other
and degrades output), so routing is explicit: converted state dicts target `transformer.`; reach `transformer_ref`
with a `transformer_ref.`-prefixed file or `load_into_transformer_ref=True`.

DiffSynth-Studio LoRAs (e.g.
[DiffSynth-Studio/MiniMax-H3-LoRA-LineartAnime](https://huggingface.co/DiffSynth-Studio/MiniMax-H3-LoRA-LineartAnime))
are trained against the raw checkpoint's per-head-interleaved fused QKV and are de-interleaved on conversion. Their
fp32 factors make the unfused LoRA path compute in fp32; `.to(torch.bfloat16)` on the model after loading restores
the bf16 memory budget.

LoRAs trained against a pruned checkpoint (the `*_pruned_*` files in
[Comfy-Org/MiniMax-H3](https://huggingface.co/Comfy-Org/MiniMax-H3);
[joyfox/MiniMax-H3-Turbo](https://huggingface.co/joyfox/MiniMax-H3-Turbo) is one) fail with a size mismatch.
Alpha-less files load at `alpha == rank` (scale 1.0, the convention stated by e.g.
[larryvrh/MiniMax-H3-Turbo-Lora](https://huggingface.co/larryvrh/MiniMax-H3-Turbo-Lora)); a `__metadata__` `alpha`
entry (e.g. [lightx2v/Minimax-h3-Turbo](https://huggingface.co/lightx2v/Minimax-h3-Turbo)'s 8-step file), when
present, is honored instead.

#### fuse_lora[[diffusers.loaders.MiniMaxH3LoraLoaderMixin.fuse_lora]]

```python
fuse_lora(components: list = ['transformer', 'transformer_ref'], lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L7100)

See `fuse_lora()` for more details.

#### load_lora_into_transformer[[diffusers.loaders.MiniMaxH3LoraLoaderMixin.load_lora_into_transformer]]

```python
load_lora_into_transformer(state_dict, transformer, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L7000)

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details.

#### load_lora_into_transformer_ref[[diffusers.loaders.MiniMaxH3LoraLoaderMixin.load_lora_into_transformer_ref]]

```python
load_lora_into_transformer_ref(state_dict, transformer_ref, prefix, adapter_name = None, _pipeline = None, low_cpu_mem_usage = False, hotswap: bool = False, metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L7027)

Load LoRA layers into the `transformer_ref` partition. `prefix` is the component name the keys carry, which is
`transformer_ref` for a file that names the partition and `transformer` for one routed here by
`load_into_transformer_ref=True`. See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more
details.

#### load_lora_weights[[diffusers.loaders.MiniMaxH3LoraLoaderMixin.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, load_into_transformer_ref: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6885)

**Parameters:**

load_into_transformer_ref (`bool`, defaults to `False`) : Load the `transformer.`-prefixed layers into the `transformer_ref` partition — the one the `ref2va` workflow denoises with — instead of `transformer`. Only needed when both partitions are loaded: a pipeline that holds `transformer_ref` alone routes there on its own.

Load LoRA layers into `transformer` or, with `load_into_transformer_ref=True`, into `transformer_ref`. See
[load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for more details.

#### lora_state_dict[[diffusers.loaders.MiniMaxH3LoraLoaderMixin.lora_state_dict]]

```python
lora_state_dict(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L6770)

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details.

#### save_lora_weights[[diffusers.loaders.MiniMaxH3LoraLoaderMixin.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, transformer_lora_layers: dict = None, transformer_ref_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, transformer_lora_adapter_metadata: dict | None = None, transformer_ref_lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L7057)

Save the LoRA layers of one or both MiniMax-H3 partitions. Which partition a LoRA belongs to is not recoverable
from its keys, so this is the only way to publish an H3 LoRA that records it. See
[save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for more information.

#### unfuse_lora[[diffusers.loaders.MiniMaxH3LoraLoaderMixin.unfuse_lora]]

```python
unfuse_lora(components: list = ['transformer', 'transformer_ref'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L7119)

See `unfuse_lora()` for more details.

## LoraBaseMixin[[diffusers.loaders.lora_base.LoraBaseMixin]]

#### diffusers.loaders.lora_base.LoraBaseMixin[[diffusers.loaders.lora_base.LoraBaseMixin]]

```python
diffusers.loaders.lora_base.LoraBaseMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L486)

Utility class for handling LoRAs.

#### delete_adapters[[diffusers.loaders.lora_base.LoraBaseMixin.delete_adapters]]

```python
delete_adapters(adapter_names: list[str] | str)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L856)

**Parameters:**

adapter_names (`list[str, str]`) : The names of the adapters to delete.

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

#### disable_lora[[diffusers.loaders.lora_base.LoraBaseMixin.disable_lora]]

```python
disable_lora()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L796)

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

```python
enable_lora()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L826)

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

```python
enable_lora_hotswap(**kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L1003)

**Parameters:**

target_rank (`int`) : The highest rank among all the adapters that will be loaded.

check_compiled (`str`, *optional*, defaults to `"error"`) : How to handle a model that is already compiled. The check can return the following messages: - "error" (default): raise an error - "warn": issue a warning - "ignore": do nothing

Hotswap adapters without triggering recompilation of a model or if the ranks of the loaded adapters are
different.

#### fuse_lora[[diffusers.loaders.lora_base.LoraBaseMixin.fuse_lora]]

```python
fuse_lora(components: list[str] | None = None, lora_scale: float = 1.0, safe_fusing: bool = False, adapter_names: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L544)

**Parameters:**

components : (`list[str]`): list of LoRA-injectable components to fuse the LoRAs into.

lora_scale (`float`, defaults to 1.0) : Controls how much to influence the outputs with the LoRA parameters.

safe_fusing (`bool`, defaults to `False`) : Whether to check fused weights for NaN values before fusing and if values are NaN not fusing them.

adapter_names (`list[str]`, *optional*) : Adapter names to be used for fusing. If nothing is passed, all active adapters will be fused.

Fuses the LoRA parameters into the original parameters of the corresponding blocks.

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

#### get_active_adapters[[diffusers.loaders.lora_base.LoraBaseMixin.get_active_adapters]]

```python
get_active_adapters()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L894)

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

```python
get_list_adapters()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L927)

Gets the current list of all available adapters in the pipeline.

#### set_adapters[[diffusers.loaders.lora_base.LoraBaseMixin.set_adapters]]

```python
set_adapters(adapter_names: list[str] | str, adapter_weights: float | dict | list[float] | list[dict] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L693)

**Parameters:**

adapter_names (`list[str]` or `str`) : The names of the adapters to use.

adapter_weights (`list[float, float]`, *optional*) : The adapter(s) weights to use with the UNet. If `None`, the weights are set to `1.0` for all the adapters.

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

#### set_lora_device[[diffusers.loaders.lora_base.LoraBaseMixin.set_lora_device]]

```python
set_lora_device(adapter_names: list[str], device: torch.device | str | int)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L949)

**Parameters:**

adapter_names (`list[str]`) : list of adapters to send device to.

device (`torch.device | str | int`) : Device to send the adapters to. Can be either a torch device, a str or an integer.

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

#### unfuse_lora[[diffusers.loaders.lora_base.LoraBaseMixin.unfuse_lora]]

```python
unfuse_lora(components: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L631)

**Parameters:**

components (`list[str]`) : list of LoRA-injectable components to unfuse LoRA from.

unfuse_unet (`bool`, defaults to `True`) : Whether to unfuse the UNet LoRA parameters.

unfuse_text_encoder (`bool`, defaults to `True`) : Whether to unfuse the text encoder LoRA parameters. If the text encoder wasn't monkey-patched with the LoRA parameters then it won't have any effect.

Reverses the effect of
[`pipe.fuse_lora()`](https://huggingface.co/docs/diffusers/main/en/api/loaders#diffusers.loaders.LoraBaseMixin.fuse_lora).

#### unload_lora_weights[[diffusers.loaders.lora_base.LoraBaseMixin.unload_lora_weights]]

```python
unload_lora_weights()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L521)

Unloads the LoRA parameters.

Examples:

```python
>>> # Assuming `pipeline` is already loaded with the LoRA parameters.
>>> pipeline.unload_lora_weights()
>>> ...
```

#### write_lora_layers[[diffusers.loaders.lora_base.LoraBaseMixin.write_lora_layers]]

```python
write_lora_layers(state_dict: dict[str, torch.Tensor], save_directory: str, is_main_process: bool, weight_name: str, save_function: Callable, safe_serialization: bool, lora_adapter_metadata: dict | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_base.py#L1026)

Writes the state dict of the LoRA layers (optionally with metadata) to disk.
