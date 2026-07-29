# IP-Adapter

[IP-Adapter](https://hf.co/papers/2308.06721) is a lightweight adapter that enables prompting a diffusion model with an image. This method decouples the cross-attention layers of the image and text features. The image features are generated from an image encoder.

> [!TIP]
> Learn how to load and use an IP-Adapter checkpoint and image in the [IP-Adapter](../../using-diffusers/ip_adapter) guide,.

## IPAdapterMixin[[diffusers.loaders.IPAdapterMixin]]

#### diffusers.loaders.IPAdapterMixin[[diffusers.loaders.IPAdapterMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/ip_adapter.py#L54)

Mixin for handling IP Adapters.

load_ip_adapterdiffusers.loaders.IPAdapterMixin.load_ip_adapterhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/ip_adapter.py#L57[{"name": "pretrained_model_name_or_path_or_dict", "val": ": str | list[str] | dict[str, torch.Tensor]"}, {"name": "subfolder", "val": ": str | list[str]"}, {"name": "weight_name", "val": ": str | list[str]"}, {"name": "image_encoder_folder", "val": ": str | None = 'image_encoder'"}, {"name": "**kwargs", "val": ""}]- **pretrained_model_name_or_path_or_dict** (`str` or `list[str]` or `os.PathLike` or `list[os.PathLike]` or `dict` or `list[dict]`) --
  Can be either:

  - A string, the *model id* (for example `google/ddpm-celebahq-256`) of a pretrained model hosted on
    the Hub.
  - A path to a *directory* (for example `./my_model_directory`) containing the model weights saved
    with [ModelMixin.save_pretrained()](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin.save_pretrained).
  - A [torch state
    dict](https://pytorch.org/tutorials/beginner/saving_loading_models.html#what-is-a-state-dict).
- **subfolder** (`str` or `list[str]`) --
  The subfolder location of a model file within a larger model repository on the Hub or locally. If a
  list is passed, it should have the same length as `weight_name`.
- **weight_name** (`str` or `list[str]`) --
  The name of the weight file to load. If a list is passed, it should have the same length as
  `subfolder`.
- **image_encoder_folder** (`str`, *optional*, defaults to `image_encoder`) --
  The subfolder location of the image encoder within a larger model repository on the Hub or locally.
  Pass `None` to not load the image encoder. If the image encoder is located in a folder inside
  `subfolder`, you only need to pass the name of the folder that contains image encoder weights, e.g.
  `image_encoder_folder="image_encoder"`. If the image encoder is located in a folder other than
  `subfolder`, you should pass the path to the folder that contains image encoder weights, for example,
  `image_encoder_folder="different_subfolder/image_encoder"`.
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
- **low_cpu_mem_usage** (`bool`, *optional*, defaults to `True` if torch version >= 1.9.0 else `False`) --
  Speed up model loading only loading the pretrained weights and not initializing the weights. This also
  tries to not use more than 1x model size in CPU memory (including peak memory) while loading the model.
  Only supported for PyTorch >= 1.9.0. If you are using an older version of PyTorch, setting this
  argument to `True` will raise an error.0

**Parameters:**

pretrained_model_name_or_path_or_dict (`str` or `list[str]` or `os.PathLike` or `list[os.PathLike]` or `dict` or `list[dict]`) : Can be either:  - A string, the *model id* (for example `google/ddpm-celebahq-256`) of a pretrained model hosted on the Hub. - A path to a *directory* (for example `./my_model_directory`) containing the model weights saved with [ModelMixin.save_pretrained()](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin.save_pretrained). - A [torch state dict](https://pytorch.org/tutorials/beginner/saving_loading_models.html#what-is-a-state-dict).

subfolder (`str` or `list[str]`) : The subfolder location of a model file within a larger model repository on the Hub or locally. If a list is passed, it should have the same length as `weight_name`.

weight_name (`str` or `list[str]`) : The name of the weight file to load. If a list is passed, it should have the same length as `subfolder`.

image_encoder_folder (`str`, *optional*, defaults to `image_encoder`) : The subfolder location of the image encoder within a larger model repository on the Hub or locally. Pass `None` to not load the image encoder. If the image encoder is located in a folder inside `subfolder`, you only need to pass the name of the folder that contains image encoder weights, e.g. `image_encoder_folder="image_encoder"`. If the image encoder is located in a folder other than `subfolder`, you should pass the path to the folder that contains image encoder weights, for example, `image_encoder_folder="different_subfolder/image_encoder"`.

cache_dir (`str | os.PathLike`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist. 

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

local_files_only (`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to `True`, the model won't be downloaded from the Hub.

token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

low_cpu_mem_usage (`bool`, *optional*, defaults to `True` if torch version >= 1.9.0 else `False`) : Speed up model loading only loading the pretrained weights and not initializing the weights. This also tries to not use more than 1x model size in CPU memory (including peak memory) while loading the model. Only supported for PyTorch >= 1.9.0. If you are using an older version of PyTorch, setting this argument to `True` will raise an error.
#### set_ip_adapter_scale[[diffusers.loaders.IPAdapterMixin.set_ip_adapter_scale]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/ip_adapter.py#L252)

Set IP-Adapter scales per-transformer block. Input `scale` could be a single config or a list of configs for
granular control over each IP-Adapter behavior. A config can be a float or a dictionary.

Example:

```py
# To use original IP-Adapter
scale = 1.0
pipeline.set_ip_adapter_scale(scale)

# To use style block only
scale = {
    "up": {"block_0": [0.0, 1.0, 0.0]},
}
pipeline.set_ip_adapter_scale(scale)

# To use style+layout blocks
scale = {
    "down": {"block_2": [0.0, 1.0]},
    "up": {"block_0": [0.0, 1.0, 0.0]},
}
pipeline.set_ip_adapter_scale(scale)

# To use style and layout from 2 reference images
scales = [{"down": {"block_2": [0.0, 1.0]}}, {"up": {"block_0": [0.0, 1.0, 0.0]}}]
pipeline.set_ip_adapter_scale(scales)
```
#### unload_ip_adapter[[diffusers.loaders.IPAdapterMixin.unload_ip_adapter]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/ip_adapter.py#L305)

Unloads the IP Adapter weights

Examples:

```python
>>> # Assuming `pipeline` is already loaded with the IP Adapter weights.
>>> pipeline.unload_ip_adapter()
>>> ...
```

## SD3IPAdapterMixin[[diffusers.loaders.SD3IPAdapterMixin]]

#### diffusers.loaders.SD3IPAdapterMixin[[diffusers.loaders.SD3IPAdapterMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/ip_adapter.py#L897)

Mixin for handling StableDiffusion 3 IP Adapters.

is_ip_adapter_activediffusers.loaders.SD3IPAdapterMixin.is_ip_adapter_activehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/ip_adapter.py#L900[]`bool`True when IP-Adapter is loaded and any layer has scale > 0.
Checks if IP-Adapter is loaded and scale > 0.

IP-Adapter scale controls the influence of the image prompt versus text prompt. When this value is set to 0,
the image context is irrelevant.

**Returns:**

``bool``

True when IP-Adapter is loaded and any layer has scale > 0.
#### load_ip_adapter[[diffusers.loaders.SD3IPAdapterMixin.load_ip_adapter]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/ip_adapter.py#L918)

**Parameters:**

pretrained_model_name_or_path_or_dict (`str` or `os.PathLike` or `dict`) : Can be either: - A string, the *model id* (for example `google/ddpm-celebahq-256`) of a pretrained model hosted on the Hub. - A path to a *directory* (for example `./my_model_directory`) containing the model weights saved with [ModelMixin.save_pretrained()](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin.save_pretrained). - A [torch state dict](https://pytorch.org/tutorials/beginner/saving_loading_models.html#what-is-a-state-dict).

weight_name (`str`, defaults to "ip-adapter.safetensors") : The name of the weight file to load. If a list is passed, it should have the same length as `subfolder`.

subfolder (`str`, *optional*) : The subfolder location of a model file within a larger model repository on the Hub or locally. If a list is passed, it should have the same length as `weight_name`.

image_encoder_folder (`str`, *optional*, defaults to `image_encoder`) : The subfolder location of the image encoder within a larger model repository on the Hub or locally. Pass `None` to not load the image encoder. If the image encoder is located in a folder inside `subfolder`, you only need to pass the name of the folder that contains image encoder weights, e.g. `image_encoder_folder="image_encoder"`. If the image encoder is located in a folder other than `subfolder`, you should pass the path to the folder that contains image encoder weights, for example, `image_encoder_folder="different_subfolder/image_encoder"`.

cache_dir (`str | os.PathLike`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist.

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

local_files_only (`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to `True`, the model won't be downloaded from the Hub.

token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

low_cpu_mem_usage (`bool`, *optional*, defaults to `True` if torch version >= 1.9.0 else `False`) : Speed up model loading only loading the pretrained weights and not initializing the weights. This also tries to not use more than 1x model size in CPU memory (including peak memory) while loading the model. Only supported for PyTorch >= 1.9.0. If you are using an older version of PyTorch, setting this argument to `True` will raise an error.
#### set_ip_adapter_scale[[diffusers.loaders.SD3IPAdapterMixin.set_ip_adapter_scale]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/ip_adapter.py#L1066)

Set IP-Adapter scale, which controls image prompt conditioning. A value of 1.0 means the model is only
conditioned on the image prompt, and 0.0 only conditioned by the text prompt. Lowering this value encourages
the model to produce more diverse images, but they may not be as aligned with the image prompt.

Example:

```python
>>> # Assuming `pipeline` is already loaded with the IP Adapter weights.
>>> pipeline.set_ip_adapter_scale(0.6)
>>> ...
```

**Parameters:**

scale (float) : IP-Adapter scale to be set.
#### unload_ip_adapter[[diffusers.loaders.SD3IPAdapterMixin.unload_ip_adapter]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/ip_adapter.py#L1089)

Unloads the IP Adapter weights.

Example:

```python
>>> # Assuming `pipeline` is already loaded with the IP Adapter weights.
>>> pipeline.unload_ip_adapter()
>>> ...
```

## IPAdapterMaskProcessor[[diffusers.IPAdapterMaskProcessor]]

#### diffusers.IPAdapterMaskProcessor[[diffusers.IPAdapterMaskProcessor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L1270)

Image processor for IP Adapter image masks.

downsamplediffusers.IPAdapterMaskProcessor.downsamplehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L1311[{"name": "mask", "val": ": Tensor"}, {"name": "batch_size", "val": ": int"}, {"name": "num_queries", "val": ": int"}, {"name": "value_embed_dim", "val": ": int"}]- **mask** (`torch.Tensor`) --
  The input mask tensor generated with `IPAdapterMaskProcessor.preprocess()`.
- **batch_size** (`int`) --
  The batch size.
- **num_queries** (`int`) --
  The number of queries.
- **value_embed_dim** (`int`) --
  The dimensionality of the value embeddings.0`torch.Tensor`The downsampled mask tensor.

Downsamples the provided mask tensor to match the expected dimensions for scaled dot-product attention. If the
aspect ratio of the mask does not match the aspect ratio of the output image, a warning is issued.

**Parameters:**

do_resize (`bool`, *optional*, defaults to `True`) : Whether to downscale the image's (height, width) dimensions to multiples of `vae_scale_factor`.

vae_scale_factor (`int`, *optional*, defaults to `8`) : VAE scale factor. If `do_resize` is `True`, the image is automatically resized to multiples of this factor.

resample (`str`, *optional*, defaults to `lanczos`) : Resampling filter to use when resizing the image.

do_normalize (`bool`, *optional*, defaults to `False`) : Whether to normalize the image to [-1,1].

do_binarize (`bool`, *optional*, defaults to `True`) : Whether to binarize the image to 0/1.

do_convert_grayscale (`bool`, *optional*, defaults to be `True`) : Whether to convert the images to grayscale format.

**Returns:**

``torch.Tensor``

The downsampled mask tensor.
