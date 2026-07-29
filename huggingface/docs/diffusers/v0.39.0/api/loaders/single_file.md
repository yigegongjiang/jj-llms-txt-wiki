# Single files

The [from_single_file()](/docs/diffusers/v0.39.0/en/api/loaders/single_file#diffusers.loaders.FromSingleFileMixin.from_single_file) method allows you to load:

* a model stored in a single file, which is useful if you're working with models from the diffusion ecosystem, like Automatic1111, and commonly rely on a single-file layout to store and share models
* a model stored in their originally distributed layout, which is useful if you're working with models finetuned with other services, and want to load it directly into Diffusers model objects and pipelines

> [!TIP]
> Read the [Model files and layouts](../../using-diffusers/other-formats) guide to learn more about the Diffusers-multifolder layout versus the single-file layout, and how to load models stored in these different layouts.

## Supported pipelines

- [StableDiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/text2img#diffusers.StableDiffusionPipeline)
- [StableDiffusionImg2ImgPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/img2img#diffusers.StableDiffusionImg2ImgPipeline)
- [StableDiffusionInpaintPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.StableDiffusionInpaintPipeline)
- [StableDiffusionControlNetPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/controlnet#diffusers.StableDiffusionControlNetPipeline)
- [StableDiffusionControlNetImg2ImgPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/controlnet#diffusers.StableDiffusionControlNetImg2ImgPipeline)
- [StableDiffusionControlNetInpaintPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/controlnet#diffusers.StableDiffusionControlNetInpaintPipeline)
- [StableDiffusionUpscalePipeline](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/upscale#diffusers.StableDiffusionUpscalePipeline)
- [StableDiffusionXLPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/stable_diffusion_xl#diffusers.StableDiffusionXLPipeline)
- [StableDiffusionXLImg2ImgPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/stable_diffusion_xl#diffusers.StableDiffusionXLImg2ImgPipeline)
- [StableDiffusionXLInpaintPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/stable_diffusion_xl#diffusers.StableDiffusionXLInpaintPipeline)
- [StableDiffusionXLInstructPix2PixPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/pix2pix#diffusers.StableDiffusionXLInstructPix2PixPipeline)
- [StableDiffusionXLControlNetPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/controlnet_sdxl#diffusers.StableDiffusionXLControlNetPipeline)
- `StableDiffusionXLKDiffusionPipeline`
- [StableDiffusion3Pipeline](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/stable_diffusion_3#diffusers.StableDiffusion3Pipeline)
- [LatentConsistencyModelPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/latent_consistency_models#diffusers.LatentConsistencyModelPipeline)
- [LatentConsistencyModelImg2ImgPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/latent_consistency_models#diffusers.LatentConsistencyModelImg2ImgPipeline)
- `StableDiffusionControlNetXSPipeline`
- `StableDiffusionXLControlNetXSPipeline`
- [LEditsPPPipelineStableDiffusion](/docs/diffusers/v0.39.0/en/api/pipelines/ledits_pp#diffusers.LEditsPPPipelineStableDiffusion)
- [LEditsPPPipelineStableDiffusionXL](/docs/diffusers/v0.39.0/en/api/pipelines/ledits_pp#diffusers.LEditsPPPipelineStableDiffusionXL)
- `PIAPipeline`

## Supported models

- [UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)
- `StableCascadeUNet`
- [AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)
- [ControlNetModel](/docs/diffusers/v0.39.0/en/api/models/controlnet#diffusers.ControlNetModel)
- [SD3Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/sd3_transformer2d#diffusers.SD3Transformer2DModel)
- [FluxTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/flux_transformer#diffusers.FluxTransformer2DModel)

## FromSingleFileMixin[[diffusers.loaders.FromSingleFileMixin]]

#### diffusers.loaders.FromSingleFileMixin[[diffusers.loaders.FromSingleFileMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/single_file.py#L266)

Load model weights saved in the `.ckpt` format into a [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline).

from_single_filediffusers.loaders.FromSingleFileMixin.from_single_filehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/single_file.py#L271[{"name": "pretrained_model_link_or_path", "val": ""}, {"name": "**kwargs", "val": ""}]- **pretrained_model_link_or_path** (`str` or `os.PathLike`, *optional*) --
  Can be either:
  - A link to the `.ckpt` file (for example
    `"https://huggingface.co//blob/main/.ckpt"`) on the Hub.
  - A path to a *file* containing all pipeline weights.
- **torch_dtype** (`str` or `torch.dtype`, *optional*) --
  Override the default `torch.dtype` and load the model with another dtype.
- **force_download** (`bool`, *optional*, defaults to `False`) --
  Whether or not to force the (re-)download of the model weights and configuration files, overriding the
  cached versions if they exist.
- **cache_dir** (`str | os.PathLike`, *optional*) --
  Path to a directory where a downloaded pretrained model configuration is cached if the standard cache
  is not used.

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
- **original_config_file** (`str`, *optional*) --
  The path to the original config file that was used to train the model. If not provided, the config file
  will be inferred from the checkpoint file.
- **config** (`str`, *optional*) --
  Can be either:
  - A string, the *repo id* (for example `CompVis/ldm-text2im-large-256`) of a pretrained pipeline
    hosted on the Hub.
  - A path to a *directory* (for example `./my_pipeline_directory/`) containing the pipeline
    component configs in Diffusers format.
- **disable_mmap** ('bool', *optional*, defaults to 'False') --
  Whether to disable mmap when loading a Safetensors model. This option can perform better when the model
  is on a network mount or hard drive.
- **kwargs** (remaining dictionary of keyword arguments, *optional*) --
  Can be used to overwrite load and saveable variables (the pipeline components of the specific pipeline
  class). The overwritten components are passed directly to the pipelines `__init__` method. See example
  below for more information.0

Instantiate a [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline) from pretrained pipeline weights saved in the `.ckpt` or `.safetensors`
format. The pipeline is set in evaluation mode (`model.eval()`) by default.

Examples:

```py
>>> from diffusers import StableDiffusionPipeline

>>> # Download pipeline from huggingface.co and cache.
>>> pipeline = StableDiffusionPipeline.from_single_file(
...     "https://huggingface.co/WarriorMama777/OrangeMixs/blob/main/Models/AbyssOrangeMix/AbyssOrangeMix.safetensors"
... )

>>> # Download pipeline from local file
>>> # file is downloaded under ./v1-5-pruned-emaonly.ckpt
>>> pipeline = StableDiffusionPipeline.from_single_file("./v1-5-pruned-emaonly.ckpt")

>>> # Enable float16 and move to GPU
>>> pipeline = StableDiffusionPipeline.from_single_file(
...     "https://huggingface.co/stable-diffusion-v1-5/stable-diffusion-v1-5/blob/main/v1-5-pruned-emaonly.ckpt",
...     torch_dtype=torch.float16,
... )
>>> pipeline.to("cuda")
```

**Parameters:**

pretrained_model_link_or_path (`str` or `os.PathLike`, *optional*) : Can be either: - A link to the `.ckpt` file (for example `"https://huggingface.co/<repo_id>/blob/main/<path_to_file>.ckpt"`) on the Hub. - A path to a *file* containing all pipeline weights.

torch_dtype (`str` or `torch.dtype`, *optional*) : Override the default `torch.dtype` and load the model with another dtype.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist.

cache_dir (`str | os.PathLike`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used. 

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

local_files_only (`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to `True`, the model won't be downloaded from the Hub.

token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

original_config_file (`str`, *optional*) : The path to the original config file that was used to train the model. If not provided, the config file will be inferred from the checkpoint file.

config (`str`, *optional*) : Can be either: - A string, the *repo id* (for example `CompVis/ldm-text2im-large-256`) of a pretrained pipeline hosted on the Hub. - A path to a *directory* (for example `./my_pipeline_directory/`) containing the pipeline component configs in Diffusers format.

disable_mmap ('bool', *optional*, defaults to 'False') : Whether to disable mmap when loading a Safetensors model. This option can perform better when the model is on a network mount or hard drive.

kwargs (remaining dictionary of keyword arguments, *optional*) : Can be used to overwrite load and saveable variables (the pipeline components of the specific pipeline class). The overwritten components are passed directly to the pipelines `__init__` method. See example below for more information.

## FromOriginalModelMixin[[diffusers.loaders.FromOriginalModelMixin]]

#### diffusers.loaders.FromOriginalModelMixin[[diffusers.loaders.FromOriginalModelMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/single_file_model.py#L251)

Load pretrained weights saved in the `.ckpt` or `.safetensors` format into a model.

from_single_filediffusers.loaders.FromOriginalModelMixin.from_single_filehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/single_file_model.py#L256[{"name": "pretrained_model_link_or_path_or_dict", "val": ": str | None = None"}, {"name": "**kwargs", "val": ""}]- **pretrained_model_link_or_path_or_dict** (`str`, *optional*) --
  Can be either:
  - A link to the `.safetensors` or `.ckpt` file (for example
    `"https://huggingface.co//blob/main/.safetensors"`) on the Hub.
  - A path to a local *file* containing the weights of the component model.
  - A state dict containing the component model weights.
- **config** (`str`, *optional*) --
  - A string, the *repo id* (for example `CompVis/ldm-text2im-large-256`) of a pretrained pipeline hosted
    on the Hub.
  - A path to a *directory* (for example `./my_pipeline_directory/`) containing the pipeline component
    configs in Diffusers format.
- **subfolder** (`str`, *optional*, defaults to `""`) --
  The subfolder location of a model file within a larger model repository on the Hub or locally.
- **original_config** (`str`, *optional*) --
  Dict or path to a yaml file containing the configuration for the model in its original format.
  If a dict is provided, it will be used to initialize the model configuration.
- **torch_dtype** (`torch.dtype`, *optional*) --
  Override the default `torch.dtype` and load the model with another dtype.
- **force_download** (`bool`, *optional*, defaults to `False`) --
  Whether or not to force the (re-)download of the model weights and configuration files, overriding the
  cached versions if they exist.
- **cache_dir** (`str | os.PathLike`, *optional*) --
  Path to a directory where a downloaded pretrained model configuration is cached if the standard cache
  is not used.

- **proxies** (`dict[str, str]`, *optional*) --
  A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128',
  'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.
- **local_files_only** (`bool`, *optional*, defaults to `False`) --
  Whether to only load local model weights and configuration files or not. If set to True, the model
  won't be downloaded from the Hub.
- **token** (`str` or *bool*, *optional*) --
  The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from
  `diffusers-cli login` (stored in `~/.huggingface`) is used.
- **revision** (`str`, *optional*, defaults to `"main"`) --
  The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier
  allowed by Git.
- **low_cpu_mem_usage** (`bool`, *optional*, defaults to `True` if torch version >= 1.9.0 and --
  is_accelerate_available() else `False`): Speed up model loading only loading the pretrained weights and
  not initializing the weights. This also tries to not use more than 1x model size in CPU memory
  (including peak memory) while loading the model. Only supported for PyTorch >= 1.9.0. If you are using
  an older version of PyTorch, setting this argument to `True` will raise an error.
- **disable_mmap** ('bool', *optional*, defaults to 'False') --
  Whether to disable mmap when loading a Safetensors model. This option can perform better when the model
  is on a network mount or hard drive, which may not handle the seeky-ness of mmap very well.
- **kwargs** (remaining dictionary of keyword arguments, *optional*) --
  Can be used to overwrite load and saveable variables (for example the pipeline components of the
  specific pipeline class). The overwritten components are directly passed to the pipelines `__init__`
  method. See example below for more information.0

Instantiate a model from pretrained weights saved in the original `.ckpt` or `.safetensors` format. The model
is set in evaluation mode (`model.eval()`) by default.

```py
>>> from diffusers import StableCascadeUNet

>>> ckpt_path = "https://huggingface.co/stabilityai/stable-cascade/blob/main/stage_b_lite.safetensors"
>>> model = StableCascadeUNet.from_single_file(ckpt_path)
```

**Parameters:**

pretrained_model_link_or_path_or_dict (`str`, *optional*) : Can be either: - A link to the `.safetensors` or `.ckpt` file (for example `"https://huggingface.co/<repo_id>/blob/main/<path_to_file>.safetensors"`) on the Hub. - A path to a local *file* containing the weights of the component model. - A state dict containing the component model weights.

config (`str`, *optional*) : - A string, the *repo id* (for example `CompVis/ldm-text2im-large-256`) of a pretrained pipeline hosted on the Hub. - A path to a *directory* (for example `./my_pipeline_directory/`) containing the pipeline component configs in Diffusers format.

subfolder (`str`, *optional*, defaults to `""`) : The subfolder location of a model file within a larger model repository on the Hub or locally.

original_config (`str`, *optional*) : Dict or path to a yaml file containing the configuration for the model in its original format. If a dict is provided, it will be used to initialize the model configuration.

torch_dtype (`torch.dtype`, *optional*) : Override the default `torch.dtype` and load the model with another dtype.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist.

cache_dir (`str | os.PathLike`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used. 

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

local_files_only (`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to True, the model won't be downloaded from the Hub.

token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

low_cpu_mem_usage (`bool`, *optional*, defaults to `True` if torch version >= 1.9.0 and : is_accelerate_available() else `False`): Speed up model loading only loading the pretrained weights and not initializing the weights. This also tries to not use more than 1x model size in CPU memory (including peak memory) while loading the model. Only supported for PyTorch >= 1.9.0. If you are using an older version of PyTorch, setting this argument to `True` will raise an error.

disable_mmap ('bool', *optional*, defaults to 'False') : Whether to disable mmap when loading a Safetensors model. This option can perform better when the model is on a network mount or hard drive, which may not handle the seeky-ness of mmap very well.

kwargs (remaining dictionary of keyword arguments, *optional*) : Can be used to overwrite load and saveable variables (for example the pipeline components of the specific pipeline class). The overwritten components are directly passed to the pipelines `__init__` method. See example below for more information.
