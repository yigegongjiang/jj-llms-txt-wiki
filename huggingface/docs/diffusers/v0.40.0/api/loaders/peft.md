# PEFT

Diffusers supports loading adapters such as [LoRA](../../tutorials/using_peft_for_inference) with the [PEFT](https://huggingface.co/docs/peft/index) library with the [PeftAdapterMixin](/docs/diffusers/v0.40.0/en/api/loaders/peft#diffusers.loaders.PeftAdapterMixin) class. This allows modeling classes in Diffusers like [UNet2DConditionModel](/docs/diffusers/v0.40.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel), [SD3Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/sd3_transformer2d#diffusers.SD3Transformer2DModel) to operate with an adapter.

> [!TIP]
> Refer to the [Inference with PEFT](../../tutorials/using_peft_for_inference) tutorial for an overview of how to use PEFT in Diffusers for inference.

## PeftAdapterMixin[[diffusers.loaders.PeftAdapterMixin]]

#### diffusers.loaders.PeftAdapterMixin[[diffusers.loaders.PeftAdapterMixin]]

```python
diffusers.loaders.PeftAdapterMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/peft.py#L56)

A class containing all functions for loading and using adapters weights that are supported in PEFT library. For
more details about adapters and injecting them in a base model, check out the PEFT
[documentation](https://huggingface.co/docs/peft/index).

Install the latest version of PEFT, and use this mixin to:

- Attach new adapters in the model.
- Attach multiple adapters and iteratively activate/deactivate them.
- Activate/deactivate all adapters from the model.
- Get a list of the active adapters.

#### active_adapters[[diffusers.loaders.PeftAdapterMixin.active_adapters]]

```python
active_adapters()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/peft.py#L625)

Gets the current list of active adapters of the model.

If you are not familiar with adapters and PEFT methods, we invite you to read more about them on the PEFT
[documentation](https://huggingface.co/docs/peft).

#### add_adapter[[diffusers.loaders.PeftAdapterMixin.add_adapter]]

```python
add_adapter(adapter_config, adapter_name: str = 'default')
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/peft.py#L493)

**Parameters:**

adapter_config (`[~peft.PeftConfig]`) : The configuration of the adapter to add; supported adapters are non-prefix tuning and adaption prompt methods.

adapter_name (`str`, *optional*, defaults to `"default"`) : The name of the adapter to add. If no name is passed, a default name is assigned to the adapter.

Adds a new adapter to the current model for training. If no adapter name is passed, a default name is assigned
to the adapter to follow the convention of the PEFT library.

If you are not familiar with adapters and PEFT methods, we invite you to read more about them in the PEFT
[documentation](https://huggingface.co/docs/peft).

#### delete_adapters[[diffusers.loaders.PeftAdapterMixin.delete_adapters]]

```python
delete_adapters(adapter_names: list[str] | str)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/peft.py#L748)

**Parameters:**

adapter_names (`list[str, str]`) : The names (single string or list of strings) of the adapter to delete.

Delete an adapter's LoRA layers from the underlying model.

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
pipeline.unet.delete_adapters("cinematic")
```

#### disable_adapters[[diffusers.loaders.PeftAdapterMixin.disable_adapters]]

```python
disable_adapters()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/peft.py#L580)

Disable all adapters attached to the model and fallback to inference with the base model only.

If you are not familiar with adapters and PEFT methods, we invite you to read more about them on the PEFT
[documentation](https://huggingface.co/docs/peft).

#### disable_lora[[diffusers.loaders.PeftAdapterMixin.disable_lora]]

```python
disable_lora()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/peft.py#L702)

Disables the active LoRA layers of the underlying model.

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
pipeline.unet.disable_lora()
```

#### enable_adapters[[diffusers.loaders.PeftAdapterMixin.enable_adapters]]

```python
enable_adapters()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/peft.py#L602)

Enable adapters that are attached to the model. The model uses `self.active_adapters()` to retrieve the list of
adapters to enable.

If you are not familiar with adapters and PEFT methods, we invite you to read more about them on the PEFT
[documentation](https://huggingface.co/docs/peft).

#### enable_lora[[diffusers.loaders.PeftAdapterMixin.enable_lora]]

```python
enable_lora()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/peft.py#L725)

Enables the active LoRA layers of the underlying model.

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
pipeline.unet.enable_lora()
```

#### enable_lora_hotswap[[diffusers.loaders.PeftAdapterMixin.enable_lora_hotswap]]

```python
enable_lora_hotswap(target_rank: int = 128, check_compiled: typing.Literal['error', 'warn', 'ignore'] = 'error')
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/peft.py#L786)

**Parameters:**

target_rank (`int`, *optional*, defaults to `128`) : The highest rank among all the adapters that will be loaded. 

check_compiled (`str`, *optional*, defaults to `"error"`) : How to handle the case when the model is already compiled, which should generally be avoided. The options are: - "error" (default): raise an error - "warn": issue a warning - "ignore": do nothing

Enables the possibility to hotswap LoRA adapters.

Calling this method is only required when hotswapping adapters and if the model is compiled or if the ranks of
the loaded adapters differ.

#### load_lora_adapter[[diffusers.loaders.PeftAdapterMixin.load_lora_adapter]]

```python
load_lora_adapter(pretrained_model_name_or_path_or_dict, prefix = 'transformer', hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/peft.py#L79)

**Parameters:**

pretrained_model_name_or_path_or_dict (`str` or `os.PathLike` or `dict`) : Can be either:  - A string, the *model id* (for example `google/ddpm-celebahq-256`) of a pretrained model hosted on the Hub. - A path to a *directory* (for example `./my_model_directory`) containing the model weights saved with [ModelMixin.save_pretrained()](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin.save_pretrained). - A [torch state dict](https://pytorch.org/tutorials/beginner/saving_loading_models.html#what-is-a-state-dict). 

prefix (`str`, *optional*) : Prefix to filter the state dict. 

cache_dir (`str | os.PathLike`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist.

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

local_files_only (`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to `True`, the model won't be downloaded from the Hub.

token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

subfolder (`str`, *optional*, defaults to `""`) : The subfolder location of a model file within a larger model repository on the Hub or locally.

network_alphas (`dict[str, float]`) : The value of the network alpha used for stable learning and preventing underflow. This value has the same meaning as the `--network_alpha` option in the kohya-ss trainer script. Refer to [this link](https://github.com/darkstorm2150/sd-scripts/blob/main/docs/train_network_README-en.md#execute-learning).

low_cpu_mem_usage (`bool`, *optional*) : Speed up model loading by only loading the pretrained LoRA weights and not initializing the random weights.

hotswap : (`bool`, *optional*) Defaults to `False`. Whether to substitute an existing (LoRA) adapter with the newly loaded adapter in-place. This means that, instead of loading an additional adapter, this will take the existing adapter weights and replace them with the weights of the new adapter. This can be faster and more memory efficient. However, the main advantage of hotswapping is that when the model is compiled with torch.compile, loading the new adapter does not require recompilation of the model. When using hotswapping, the passed `adapter_name` should be the name of an already loaded adapter.  If the new adapter and the old adapter have different ranks and/or LoRA alphas (i.e. scaling), you need to call an additional method before loading the adapter:  ```py pipeline = ...  # load diffusers pipeline max_rank = ...  # the highest rank among all LoRAs that you want to load # call *before* compiling and loading the LoRA adapter pipeline.enable_lora_hotswap(target_rank=max_rank) pipeline.load_lora_weights(file_name) # optionally compile the model now ```  Note that hotswapping adapters of the text encoder is not yet supported. There are some further limitations to this technique, which are documented here: https://huggingface.co/docs/peft/main/en/package_reference/hotswap

metadata : LoRA adapter metadata. When supplied, the metadata inferred through the state dict isn't used to initialize `LoraConfig`.

Loads a LoRA adapter into the underlying model.

#### save_lora_adapter[[diffusers.loaders.PeftAdapterMixin.save_lora_adapter]]

```python
save_lora_adapter(save_directory, adapter_name: str = 'default', upcast_before_saving: bool = False, safe_serialization: bool = True, weight_name: str | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/peft.py#L369)

**Parameters:**

save_directory (`str` or `os.PathLike`) : Directory to save LoRA parameters to. Will be created if it doesn't exist.

adapter_name : (`str`, defaults to "default"): The name of the adapter to serialize. Useful when the underlying model has multiple adapters loaded.

upcast_before_saving (`bool`, defaults to `False`) : Whether to cast the underlying model to `torch.float32` before serialization.

safe_serialization (`bool`, *optional*, defaults to `True`) : Whether to save the model using `safetensors` or the traditional PyTorch way with `pickle`.

weight_name : (`str`, *optional*, defaults to `None`): Name of the file to serialize the state dict with.

Save the LoRA parameters corresponding to the underlying model.

#### set_adapter[[diffusers.loaders.PeftAdapterMixin.set_adapter]]

```python
set_adapter(adapter_name: str | list[str])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/peft.py#L531)

**Parameters:**

adapter_name (str | list[str])) : The list of adapters to set or the adapter name in the case of a single adapter.

Sets a specific adapter by forcing the model to only use that adapter and disables the other adapters.

If you are not familiar with adapters and PEFT methods, we invite you to read more about them on the PEFT
[documentation](https://huggingface.co/docs/peft).

#### set_adapters[[diffusers.loaders.PeftAdapterMixin.set_adapters]]

```python
set_adapters(adapter_names: list[str] | str, weights: float | dict | list[float] | list[dict] | list[None] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/peft.py#L437)

**Parameters:**

adapter_names (`list[str]` or `str`) : The names of the adapters to use.

weights (`Union[List[float], float]`, *optional*) : The adapter(s) weights to use with the UNet. If `None`, the weights are set to `1.0` for all the adapters.

Set the currently active adapters for use in the diffusion network (e.g. unet, transformer, etc.).

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
pipeline.unet.set_adapters(["cinematic", "pixel"], weights=[0.5, 0.5])
```
