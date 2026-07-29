# Pipelines

Pipelines provide a simple way to run state-of-the-art diffusion models in inference by bundling all of the necessary components (multiple independently-trained models, schedulers, and processors) into a single end-to-end class. Pipelines are flexible and they can be adapted to use different schedulers or even model components.

All pipelines are built from the base [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline) class which provides basic functionality for loading, downloading, and saving all the components. Specific pipeline types (for example [StableDiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/text2img#diffusers.StableDiffusionPipeline)) loaded with [from_pretrained()](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline.from_pretrained) are automatically detected and the pipeline components are loaded and passed to the `__init__` function of the pipeline.

> [!WARNING]
> You shouldn't use the [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline) class for training. Individual components (for example, [UNet2DModel](/docs/diffusers/v0.39.0/en/api/models/unet2d#diffusers.UNet2DModel) and [UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) of diffusion pipelines are usually trained individually, so we suggest directly working with them instead.
>
> 
>
> Pipelines do not offer any training functionality. You'll notice PyTorch's autograd is disabled by decorating the `__call__()` method with a [`torch.no_grad`](https://pytorch.org/docs/stable/generated/torch.no_grad.html) decorator because pipelines should not be used for training. If you're interested in training, please take a look at the [Training](../../training/overview) guides instead!

The table below lists all the pipelines currently available in 🤗 Diffusers and the tasks they support. Click on a pipeline to view its abstract and published paper.

| Pipeline | Tasks |
|---|---|
| [AnimateDiff](animatediff) | text2video |
| [AudioLDM2](audioldm2) | text2audio |
| [LongCat-AudioDiT](longcat_audio_dit) | text2audio |
| [AuraFlow](aura_flow) | text2image |
| [Bria 3.2](bria_3_2) | text2image |
| [CogVideoX](cogvideox) | text2video |
| [Consistency Models](consistency_models) | unconditional image generation |
| [ControlNet](controlnet) | text2image, image2image, inpainting |
| [ControlNet with Flux.1](controlnet_flux) | text2image |
| [ControlNet with Hunyuan-DiT](controlnet_hunyuandit) | text2image |
| [ControlNet with Stable Diffusion 3](controlnet_sd3) | text2image |
| [ControlNet with Stable Diffusion XL](controlnet_sdxl) | text2image |
| [DDIM](ddim) | unconditional image generation |
| [DDPM](ddpm) | unconditional image generation |
| [DeepFloyd IF](deepfloyd_if) | text2image, image2image, inpainting, super-resolution |
| [DiT](dit) | text2image |
| [Flux](flux) | text2image |
| [Hunyuan-DiT](hunyuandit) | text2image |
| [InstructPix2Pix](pix2pix) | image editing |
| [Kandinsky 2.1](kandinsky) | text2image, image2image, inpainting, interpolation |
| [Kandinsky 2.2](kandinsky_v22) | text2image, image2image, inpainting |
| [Kandinsky 3](kandinsky3) | text2image, image2image |
| [Kolors](kolors) | text2image |
| [Latent Consistency Models](latent_consistency_models) | text2image |
| [Latent Diffusion](latent_diffusion) | text2image, super-resolution |
| [Latte](latte) | text2image |
| [LEDITS++](ledits_pp) | image editing |
| [LLaDA2](llada2) | text2text |
| [Lumina-T2X](lumina) | text2image |
| [Marigold](marigold) | depth-estimation, normals-estimation, intrinsic-decomposition |
| [Motif-Video](motif_video) | text2video, image2video |
| [PAG](pag) | text2image |
| [PixArt-α](pixart) | text2image |
| [PixArt-Σ](pixart_sigma) | text2image |
| [Shap-E](shap_e) | text-to-3D, image-to-3D |
| [Stable Audio](stable_audio) | text2audio |
| [Stable Cascade](stable_cascade) | text2image |
| [Stable Diffusion](stable_diffusion/overview) | text2image, image2image, depth2image, inpainting, image variation, latent upscaler, super-resolution |
| [Stable Diffusion XL](stable_diffusion/stable_diffusion_xl) | text2image, image2image, inpainting |
| [Stable Diffusion XL Turbo](stable_diffusion/sdxl_turbo) | text2image, image2image, inpainting |
| [Stable unCLIP](stable_unclip) | text2image, image variation |
| [T2I-Adapter](stable_diffusion/adapter) | text2image |
| [Value-guided planning](value_guided_sampling) | value guided sampling |
| [VisualCloze](visualcloze) | text2image, image2image, subject driven generation, inpainting, style transfer, image restoration, image editing, [depth,normal,edge,pose]2image, [depth,normal,edge,pose]-estimation, virtual try-on, image relighting |

## DiffusionPipeline[[diffusers.DiffusionPipeline]]

#### diffusers.DiffusionPipeline[[diffusers.DiffusionPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L185)

Base class for all pipelines.

[DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline) stores all components (models, schedulers, and processors) for diffusion pipelines and
provides methods for loading, downloading and saving models. It also includes methods to:

- move all PyTorch modules to the device of your choice
- enable/disable the progress bar for the denoising iteration

Class attributes:

- **config_name** (`str`) -- The configuration filename that stores the class and module names of all the
  diffusion pipeline's components.
- **_optional_components** (`List[str]`) -- List of all optional components that don't have to be passed to the
  pipeline to function (should be overridden by subclasses).

__call__diffusers.DiffusionPipeline.__call__[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]
Call self as a function.
#### device[[diffusers.DiffusionPipeline.device]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L587)

**Returns:**

``torch.device``

The torch device on which the pipeline is located.
#### to[[diffusers.DiffusionPipeline.to]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L384)

Performs Pipeline dtype and/or device conversion. A torch.dtype and torch.device are inferred from the
arguments of `self.to(*args, **kwargs).`

> [!TIP] > If the pipeline already has the correct torch.dtype and torch.device, then it is returned as is.
Otherwise, > the returned pipeline is a copy of self with the desired torch.dtype and torch.device.

Here are the ways to call `to`:

- `to(dtype, silence_dtype_warnings=False) → DiffusionPipeline` to return a pipeline with the specified
  [`dtype`](https://pytorch.org/docs/stable/tensor_attributes.html#torch.dtype)
- `to(device, silence_dtype_warnings=False) → DiffusionPipeline` to return a pipeline with the specified
  [`device`](https://pytorch.org/docs/stable/tensor_attributes.html#torch.device)
- `to(device=None, dtype=None, silence_dtype_warnings=False) → DiffusionPipeline` to return a pipeline with the
  specified [`device`](https://pytorch.org/docs/stable/tensor_attributes.html#torch.device) and
  [`dtype`](https://pytorch.org/docs/stable/tensor_attributes.html#torch.dtype)

**Parameters:**

dtype (`torch.dtype`, *optional*) : Returns a pipeline with the specified [`dtype`](https://pytorch.org/docs/stable/tensor_attributes.html#torch.dtype)

device (`torch.Device`, *optional*) : Returns a pipeline with the specified [`device`](https://pytorch.org/docs/stable/tensor_attributes.html#torch.device)

silence_dtype_warnings (`str`, *optional*, defaults to `False`) : Whether to omit warnings if the target `dtype` is not compatible with the target `device`.

**Returns:**

`[DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline)`

The pipeline converted to specified `dtype` and/or `dtype`.
#### components[[diffusers.DiffusionPipeline.components]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L1924)

The `self.components` property can be useful to run different pipelines with the same weights and
configurations without reallocating additional memory.

Returns (`dict`):
A dictionary containing all the modules needed to initialize the pipeline.

Examples:

```py
>>> from diffusers import (
...     StableDiffusionPipeline,
...     StableDiffusionImg2ImgPipeline,
...     StableDiffusionInpaintPipeline,
... )

>>> text2img = StableDiffusionPipeline.from_pretrained("stable-diffusion-v1-5/stable-diffusion-v1-5")
>>> img2img = StableDiffusionImg2ImgPipeline(**text2img.components)
>>> inpaint = StableDiffusionInpaintPipeline(**text2img.components)
```
#### disable_attention_slicing[[diffusers.DiffusionPipeline.disable_attention_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2084)

Disable sliced attention computation. If `enable_attention_slicing` was previously called, attention is
computed in one step.
#### disable_xformers_memory_efficient_attention[[diffusers.DiffusionPipeline.disable_xformers_memory_efficient_attention]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2023)

Disable memory efficient attention from [xFormers](https://facebookresearch.github.io/xformers/).
#### download[[diffusers.DiffusionPipeline.download]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L1515)

Download and cache a PyTorch diffusion pipeline from pretrained pipeline weights.

> [!TIP] > To use private or [gated models](https://huggingface.co/docs/hub/models-gated#gated-models), log-in
with `hf > auth login

**Parameters:**

pretrained_model_name (`str` or `os.PathLike`, *optional*) : A string, the *repository id* (for example `CompVis/ldm-text2im-large-256`) of a pretrained pipeline hosted on the Hub.

custom_pipeline (`str`, *optional*) : Can be either:  - A string, the *repository id* (for example `CompVis/ldm-text2im-large-256`) of a pretrained pipeline hosted on the Hub. The repository must contain a file called `pipeline.py` that defines the custom pipeline.  - A string, the *file name* of a community pipeline hosted on GitHub under [Community](https://github.com/huggingface/diffusers/tree/main/examples/community). Valid file names must match the file name and not the pipeline script (`clip_guided_stable_diffusion` instead of `clip_guided_stable_diffusion.py`). Community pipelines are always loaded from the current `main` branch of GitHub.  - A path to a *directory* (`./my_pipeline_directory/`) containing a custom pipeline. The directory must contain a file called `pipeline.py` that defines the custom pipeline.  > [!WARNING] > 🧪 This is an experimental feature and may change in the future.  For more information on how to load and create custom pipelines, take a look at [How to contribute a community pipeline](https://huggingface.co/docs/diffusers/main/en/using-diffusers/contribute_pipeline). 

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist. 

proxies (`Dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

output_loading_info(`bool`, *optional*, defaults to `False`) : Whether or not to also return a dictionary containing missing keys, unexpected keys and error messages.

local_files_only (`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to `True`, the model won't be downloaded from the Hub.

token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

custom_revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, or a commit id similar to `revision` when loading a custom pipeline from the Hub. It can be a 🤗 Diffusers version when loading a custom pipeline from GitHub, otherwise it defaults to `"main"` when loading from the Hub.

mirror (`str`, *optional*) : Mirror source to resolve accessibility issues if you're downloading a model in China. We do not guarantee the timeliness or safety of the source, and you should refer to the mirror site for more information.

variant (`str`, *optional*) : Load weights from a specified variant filename such as `"fp16"` or `"ema"`. This is ignored when loading `from_flax`.

dduf_file(`str`, *optional*) : Load weights from the specified DDUF file.

use_safetensors (`bool`, *optional*, defaults to `None`) : If set to `None`, the safetensors weights are downloaded if they're available **and** if the safetensors library is installed. If set to `True`, the model is forcibly loaded from safetensors weights. If set to `False`, safetensors weights are not loaded.

use_onnx (`bool`, *optional*, defaults to `False`) : If set to `True`, ONNX weights will always be downloaded if present. If set to `False`, ONNX weights will never be downloaded. By default `use_onnx` defaults to the `_is_onnx` class attribute which is `False` for non-ONNX pipelines and `True` for ONNX pipelines. ONNX weights include both files ending with `.onnx` and `.pb`.

trust_remote_code (`bool`, *optional*, defaults to `False`) : Whether or not to allow for custom pipelines and components defined on the Hub in their own files. This option should only be set to `True` for repositories you trust and in which you have read the code, as it will execute code present on the Hub on your local machine.

use_flashpack (`bool`, *optional*, defaults to `False`) : If set to `True`, FlashPack weights will always be downloaded if present. If set to `False`, FlashPack weights will never be downloaded.

**Returns:**

``os.PathLike``

A path to the downloaded pipeline.
#### enable_attention_slicing[[diffusers.DiffusionPipeline.enable_attention_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2047)

Enable sliced attention computation. When this option is enabled, the attention module splits the input tensor
in slices to compute attention in several steps. For more than one attention head, the computation is performed
sequentially over each head. This is useful to save some memory in exchange for a small speed decrease.

> [!WARNING] > ⚠️ Don't enable attention slicing if you're already using `scaled_dot_product_attention` (SDPA)
from PyTorch > 2.0 or xFormers. These attention computations are already very memory efficient so you won't
need to enable > this function. If you enable attention slicing with SDPA or xFormers, it can lead to serious
slow downs!

Examples:

```py
>>> import torch
>>> from diffusers import StableDiffusionPipeline

>>> pipe = StableDiffusionPipeline.from_pretrained(
...     "stable-diffusion-v1-5/stable-diffusion-v1-5",
...     torch_dtype=torch.float16,
...     use_safetensors=True,
... )

>>> prompt = "a photo of an astronaut riding a horse on mars"
>>> pipe.enable_attention_slicing()
>>> image = pipe(prompt).images[0]
```

**Parameters:**

slice_size (`str` or `int`, *optional*, defaults to `"auto"`) : When `"auto"`, halves the input to the attention heads, so attention will be computed in two steps. If `"max"`, maximum amount of memory will be saved by running only one slice at a time. If a number is provided, uses as many slices as `attention_head_dim // slice_size`. In this case, `attention_head_dim` must be a multiple of `slice_size`.
#### enable_group_offload[[diffusers.DiffusionPipeline.enable_group_offload]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L1375)

Applies group offloading to the internal layers of a torch.nn.Module. To understand what group offloading is,
and where it is beneficial, we need to first provide some context on how other supported offloading methods
work.

Typically, offloading is done at two levels:
- Module-level: In Diffusers, this can be enabled using the `ModelMixin::enable_model_cpu_offload()` method. It
works by offloading each component of a pipeline to the CPU for storage, and onloading to the accelerator
device when needed for computation. This method is more memory-efficient than keeping all components on the
accelerator, but the memory requirements are still quite high. For this method to work, one needs memory
equivalent to size of the model in runtime dtype + size of largest intermediate activation tensors to be able
to complete the forward pass.
- Leaf-level: In Diffusers, this can be enabled using the `ModelMixin::enable_sequential_cpu_offload()` method.
  It
works by offloading the lowest leaf-level parameters of the computation graph to the CPU for storage, and
onloading only the leafs to the accelerator device for computation. This uses the lowest amount of accelerator
memory, but can be slower due to the excessive number of device synchronizations.

Group offloading is a middle ground between the two methods. It works by offloading groups of internal layers,
(either `torch.nn.ModuleList` or `torch.nn.Sequential`). This method uses lower memory than module-level
offloading. It is also faster than leaf-level/sequential offloading, as the number of device synchronizations
is reduced.

Another supported feature (for CUDA devices with support for asynchronous data transfer streams) is the ability
to overlap data transfer and computation to reduce the overall execution time compared to sequential
offloading. This is enabled using layer prefetching with streams, i.e., the layer that is to be executed next
starts onloading to the accelerator device while the current layer is being executed - this increases the
memory requirements slightly. Note that this implementation also supports leaf-level offloading but can be made
much faster when using streams.

Example:
```python
>>> from diffusers import DiffusionPipeline
>>> import torch

>>> pipe = DiffusionPipeline.from_pretrained("Qwen/Qwen-Image", torch_dtype=torch.bfloat16)

>>> pipe.enable_group_offload(
...     onload_device=torch.device("cuda"),
...     offload_device=torch.device("cpu"),
...     offload_type="leaf_level",
...     use_stream=True,
... )
>>> image = pipe("a beautiful sunset").images[0]
```

**Parameters:**

onload_device (`torch.device`) : The device to which the group of modules are onloaded.

offload_device (`torch.device`, defaults to `torch.device("cpu")`) : The device to which the group of modules are offloaded. This should typically be the CPU. Default is CPU.

offload_type (`str` or `GroupOffloadingType`, defaults to "block_level") : The type of offloading to be applied. Can be one of "block_level" or "leaf_level". Default is "block_level".

offload_to_disk_path (`str`, *optional*, defaults to `None`) : The path to the directory where parameters will be offloaded. Setting this option can be useful in limited RAM environment settings where a reasonable speed-memory trade-off is desired.

num_blocks_per_group (`int`, *optional*) : The number of blocks per group when using offload_type="block_level". This is required when using offload_type="block_level".

non_blocking (`bool`, defaults to `False`) : If True, offloading and onloading is done with non-blocking data transfer.

use_stream (`bool`, defaults to `False`) : If True, offloading and onloading is done asynchronously using a CUDA stream. This can be useful for overlapping computation and data transfer.

record_stream (`bool`, defaults to `False`) : When enabled with `use_stream`, it marks the current tensor as having been used by this stream. It is faster at the expense of slightly more memory usage. Refer to the [PyTorch official docs](https://pytorch.org/docs/stable/generated/torch.Tensor.record_stream.html) more details.

low_cpu_mem_usage (`bool`, defaults to `False`) : If True, the CPU memory usage is minimized by pinning tensors on-the-fly instead of pre-pinning them. This option only matters when using streamed CPU offloading (i.e. `use_stream=True`). This can be useful when the CPU memory is a bottleneck but may counteract the benefits of using streams.

exclude_modules (`Union[str, List[str]]`, defaults to `None`) : List of modules to exclude from offloading.
#### enable_model_cpu_offload[[diffusers.DiffusionPipeline.enable_model_cpu_offload]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L1190)

Offloads all models to CPU using accelerate, reducing memory usage with a low impact on performance. Compared
to `enable_sequential_cpu_offload`, this method moves one whole model at a time to the accelerator when its
`forward` method is called, and the model remains in accelerator until the next model runs. Memory savings are
lower than with `enable_sequential_cpu_offload`, but performance is much better due to the iterative execution
of the `unet`.

**Parameters:**

gpu_id (`int`, *optional*) : The ID of the accelerator that shall be used in inference. If not specified, it will default to 0.

device (`torch.Device` or `str`, *optional*, defaults to None) : The PyTorch device type of the accelerator that shall be used in inference. If not specified, it will automatically detect the available accelerator and use.
#### enable_sequential_cpu_offload[[diffusers.DiffusionPipeline.enable_sequential_cpu_offload]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L1308)

Offloads all models to CPU using 🤗 Accelerate, significantly reducing memory usage. When called, the state
dicts of all `torch.nn.Module` components (except those in `self._exclude_from_cpu_offload`) are saved to CPU
and then moved to `torch.device('meta')` and loaded to accelerator only when their specific submodule has its
`forward` method called. Offloading happens on a submodule basis. Memory savings are higher than with
`enable_model_cpu_offload`, but performance is lower.

**Parameters:**

gpu_id (`int`, *optional*) : The ID of the accelerator that shall be used in inference. If not specified, it will default to 0.

device (`torch.Device` or `str`, *optional*, defaults to None) : The PyTorch device type of the accelerator that shall be used in inference. If not specified, it will automatically detect the available accelerator and use.
#### enable_xformers_memory_efficient_attention[[diffusers.DiffusionPipeline.enable_xformers_memory_efficient_attention]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L1992)

Enable memory efficient attention from [xFormers](https://facebookresearch.github.io/xformers/). When this
option is enabled, you should observe lower GPU memory usage and a potential speed up during inference. Speed
up during training is not guaranteed.

> [!WARNING] > ⚠️ When memory efficient attention and sliced attention are both enabled, memory efficient
attention takes > precedent.

Examples:

```py
>>> import torch
>>> from diffusers import DiffusionPipeline
>>> from xformers.ops import MemoryEfficientAttentionFlashAttentionOp

>>> pipe = DiffusionPipeline.from_pretrained("stabilityai/stable-diffusion-2-1", torch_dtype=torch.float16)
>>> pipe = pipe.to("cuda")
>>> pipe.enable_xformers_memory_efficient_attention(attention_op=MemoryEfficientAttentionFlashAttentionOp)
>>> # Workaround for not accepting attention shape using VAE for Flash Attention
>>> pipe.vae.enable_xformers_memory_efficient_attention(attention_op=None)
```

**Parameters:**

attention_op (`Callable`, *optional*) : Override the default `None` operator for use as `op` argument to the [`memory_efficient_attention()`](https://facebookresearch.github.io/xformers/components/ops.html#xformers.ops.memory_efficient_attention) function of xFormers.
#### from_pipe[[diffusers.DiffusionPipeline.from_pipe]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2100)

Create a new pipeline from a given pipeline. This method is useful to create a new pipeline from the existing
pipeline components without reallocating additional memory.

Examples:

```py
>>> from diffusers import StableDiffusionPipeline, StableDiffusionSAGPipeline

>>> pipe = StableDiffusionPipeline.from_pretrained("stable-diffusion-v1-5/stable-diffusion-v1-5")
>>> new_pipe = StableDiffusionSAGPipeline.from_pipe(pipe)
```

**Parameters:**

pipeline (`DiffusionPipeline`) : The pipeline from which to create a new pipeline.

**Returns:**

``DiffusionPipeline``

A new pipeline with the same weights and configurations as `pipeline`.
#### from_pretrained[[diffusers.DiffusionPipeline.from_pretrained]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L617)

Instantiate a PyTorch diffusion pipeline from pretrained pipeline weights.

The pipeline is set in evaluation mode (`model.eval()`) by default.

If you get the error message below, you need to finetune the weights for your downstream task:

```
Some weights of UNet2DConditionModel were not initialized from the model checkpoint at stable-diffusion-v1-5/stable-diffusion-v1-5 and are newly initialized because the shapes did not match:
- conv_in.weight: found shape torch.Size([320, 4, 3, 3]) in the checkpoint and torch.Size([320, 9, 3, 3]) in the model instantiated
You should probably TRAIN this model on a down-stream task to be able to use it for predictions and inference.
```

> [!TIP] > To use private or [gated](https://huggingface.co/docs/hub/models-gated#gated-models) models, log-in
with `hf > auth login`.

Examples:

```py
>>> from diffusers import DiffusionPipeline

>>> # Download pipeline from huggingface.co and cache.
>>> pipeline = DiffusionPipeline.from_pretrained("CompVis/ldm-text2im-large-256")

>>> # Download pipeline that requires an authorization token
>>> # For more information on access tokens, please refer to this section
>>> # of the documentation](https://huggingface.co/docs/hub/security-tokens)
>>> pipeline = DiffusionPipeline.from_pretrained("stable-diffusion-v1-5/stable-diffusion-v1-5")

>>> # Use a different scheduler
>>> from diffusers import LMSDiscreteScheduler

>>> scheduler = LMSDiscreteScheduler.from_config(pipeline.scheduler.config)
>>> pipeline.scheduler = scheduler
```

**Parameters:**

pretrained_model_name_or_path (`str` or `os.PathLike`, *optional*) : Can be either:  - A string, the *repo id* (for example `CompVis/ldm-text2im-large-256`) of a pretrained pipeline hosted on the Hub. - A path to a *directory* (for example `./my_pipeline_directory/`) containing pipeline weights saved using [save_pretrained()](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline.save_pretrained). - A path to a *directory* (for example `./my_pipeline_directory/`) containing a dduf file

torch_dtype (`torch.dtype` or `dict[str, Union[str, torch.dtype]]`, *optional*) : Override the default `torch.dtype` and load the model with another dtype. To load submodels with different dtype pass a `dict` (for example `{'transformer': torch.bfloat16, 'vae': torch.float16}`). Set the default dtype for unspecified components with `default` (for example `{'transformer': torch.bfloat16, 'default': torch.float16}`). If a component is not specified and no default is set, `torch.float32` is used.

custom_pipeline (`str`, *optional*) : > [!WARNING] > 🧪 This is an experimental feature and may change in the future.  Can be either:  - A string, the *repo id* (for example `hf-internal-testing/diffusers-dummy-pipeline`) of a custom pipeline hosted on the Hub. The repository must contain a file called pipeline.py that defines the custom pipeline. - A string, the *file name* of a community pipeline hosted on GitHub under [Community](https://github.com/huggingface/diffusers/tree/main/examples/community). Valid file names must match the file name and not the pipeline script (`clip_guided_stable_diffusion` instead of `clip_guided_stable_diffusion.py`). Community pipelines are always loaded from the current main branch of GitHub. - A path to a directory (`./my_pipeline_directory/`) containing a custom pipeline. The directory must contain a file called `pipeline.py` that defines the custom pipeline.  For more information on how to load and create custom pipelines, please have a look at [Loading and Adding Custom Pipelines](https://huggingface.co/docs/diffusers/using-diffusers/custom_pipeline_overview)

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist.

cache_dir (`Union[str, os.PathLike]`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used. 

proxies (`Dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

output_loading_info(`bool`, *optional*, defaults to `False`) : Whether or not to also return a dictionary containing missing keys, unexpected keys and error messages.

local_files_only (`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to `True`, the model won't be downloaded from the Hub.

token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

custom_revision (`str`, *optional*) : The specific model version to use. It can be a branch name, a tag name, or a commit id similar to `revision` when loading a custom pipeline from the Hub. Defaults to the latest stable 🤗 Diffusers version.

mirror (`str`, *optional*) : Mirror source to resolve accessibility issues if you’re downloading a model in China. We do not guarantee the timeliness or safety of the source, and you should refer to the mirror site for more information.

device_map (`str`, *optional*) : Strategy that dictates how the different components of a pipeline should be placed on available devices. Currently, only "balanced" `device_map` is supported. Check out [this](https://huggingface.co/docs/diffusers/main/en/tutorials/inference_with_big_models#device-placement) to know more.

max_memory (`Dict`, *optional*) : A dictionary device identifier for the maximum memory. Will default to the maximum memory available for each GPU and the available CPU RAM if unset.

offload_folder (`str` or `os.PathLike`, *optional*) : The path to offload weights if device_map contains the value `"disk"`.

offload_state_dict (`bool`, *optional*) : If `True`, temporarily offloads the CPU state dict to the hard drive to avoid running out of CPU RAM if the weight of the CPU state dict + the biggest shard of the checkpoint does not fit. Defaults to `True` when there is some disk offload.

low_cpu_mem_usage (`bool`, *optional*, defaults to `True` if torch version >= 1.9.0 else `False`) : Speed up model loading only loading the pretrained weights and not initializing the weights. This also tries to not use more than 1x model size in CPU memory (including peak memory) while loading the model. Only supported for PyTorch >= 1.9.0. If you are using an older version of PyTorch, setting this argument to `True` will raise an error.

use_safetensors (`bool`, *optional*, defaults to `None`) : If set to `None`, the safetensors weights are downloaded if they're available **and** if the safetensors library is installed. If set to `True`, the model is forcibly loaded from safetensors weights. If set to `False`, safetensors weights are not loaded.

use_onnx (`bool`, *optional*, defaults to `None`) : If set to `True`, ONNX weights will always be downloaded if present. If set to `False`, ONNX weights will never be downloaded. By default `use_onnx` defaults to the `_is_onnx` class attribute which is `False` for non-ONNX pipelines and `True` for ONNX pipelines. ONNX weights include both files ending with `.onnx` and `.pb`.

kwargs (remaining dictionary of keyword arguments, *optional*) : Can be used to overwrite load and saveable variables (the pipeline components of the specific pipeline class). The overwritten components are passed directly to the pipelines `__init__` method. See example below for more information.

variant (`str`, *optional*) : Load weights from a specified variant filename such as `"fp16"` or `"ema"`. This is ignored when loading `from_flax`.

dduf_file(`str`, *optional*) : Load weights from the specified dduf file.

disable_mmap ('bool', *optional*, defaults to 'False') : Whether to disable mmap when loading a Safetensors model. This option can perform better when the model is on a network mount or hard drive, which may not handle the seeky-ness of mmap very well.
#### maybe_free_model_hooks[[diffusers.DiffusionPipeline.maybe_free_model_hooks]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L1285)

Method that performs the following:
- Offloads all components.
- Removes all model hooks that were added when using `enable_model_cpu_offload`, and then applies them again.
  In case the model has not been offloaded, this function is a no-op.
- Resets stateful diffusers hooks of denoiser components if they were added with
  `register_hook()`.

Make sure to add this function to the end of the `__call__` function of your pipeline so that it functions
correctly when applying `enable_model_cpu_offload`.
#### numpy_to_pil[[diffusers.DiffusionPipeline.numpy_to_pil]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L1962)

Convert a NumPy image or a batch of images to a PIL image.
#### remove_all_hooks[[diffusers.DiffusionPipeline.remove_all_hooks]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L1181)

Removes all hooks that were added when using `enable_sequential_cpu_offload` or `enable_model_cpu_offload`.
#### reset_device_map[[diffusers.DiffusionPipeline.reset_device_map]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L1502)

Resets the device maps (if any) to None.
#### save_pretrained[[diffusers.DiffusionPipeline.save_pretrained]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L241)

Save all saveable variables of the pipeline to a directory. A pipeline variable can be saved and loaded if its
class implements both a save and loading method. The pipeline is easily reloaded using the
[from_pretrained()](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline.from_pretrained) class method.

**Parameters:**

save_directory (`str` or `os.PathLike`) : Directory to save a pipeline to. Will be created if it doesn't exist.

safe_serialization (`bool`, *optional*, defaults to `True`) : Whether to save the model using `safetensors` or the traditional PyTorch way with `pickle`.

variant (`str`, *optional*) : If specified, weights are saved in the format `pytorch_model.<variant>.bin`.

max_shard_size (`int` or `str`, defaults to `None`) : The maximum size for a checkpoint before being sharded. Checkpoints shard will then be each of size lower than this size. If expressed as a string, needs to be digits followed by a unit (like `"5GB"`). If expressed as an integer, the unit is bytes. Note that this limit will be decreased after a certain period of time (starting from Oct 2024) to allow users to upgrade to the latest version of `diffusers`. This is to establish a common default size for this argument across different libraries in the Hugging Face ecosystem (`transformers`, and `accelerate`, for example).

push_to_hub (`bool`, *optional*, defaults to `False`) : Whether or not to push your model to the Hugging Face model hub after saving it. You can specify the repository you want to push to with `repo_id` (will default to the name of `save_directory` in your namespace). 

kwargs (`Dict[str, Any]`, *optional*) : Additional keyword arguments passed along to the [push_to_hub()](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.utils.PushToHubMixin.push_to_hub) method.

#### diffusers.StableDiffusionMixin.enable_freeu[[diffusers.StableDiffusionMixin.enable_freeu]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2320)

Enables the FreeU mechanism as in https://huggingface.co/papers/2309.11497.

The suffixes after the scaling factors represent the stages where they are being applied.

Please refer to the [official repository](https://github.com/ChenyangSi/FreeU) for combinations of the values
that are known to work well for different pipelines such as Stable Diffusion v1, v2, and Stable Diffusion XL.

**Parameters:**

s1 (`float`) : Scaling factor for stage 1 to attenuate the contributions of the skip features. This is done to mitigate "oversmoothing effect" in the enhanced denoising process.

s2 (`float`) : Scaling factor for stage 2 to attenuate the contributions of the skip features. This is done to mitigate "oversmoothing effect" in the enhanced denoising process.

b1 (`float`) : Scaling factor for stage 1 to amplify the contributions of backbone features.

b2 (`float`) : Scaling factor for stage 2 to amplify the contributions of backbone features.

#### diffusers.StableDiffusionMixin.disable_freeu[[diffusers.StableDiffusionMixin.disable_freeu]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2342)

Disables the FreeU mechanism if enabled.

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

## Callbacks[[diffusers.callbacks.PipelineCallback]]

#### diffusers.callbacks.PipelineCallback[[diffusers.callbacks.PipelineCallback]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/callbacks.py#L7)

Base class for all the official callbacks used in a pipeline. This class provides a structure for implementing
custom callbacks and ensures that all callbacks have a consistent interface.

Please implement the following:
`tensor_inputs`: This should return a list of tensor inputs specific to your callback. You will only be able to
include
variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.
`callback_fn`: This method defines the core functionality of your callback.

#### diffusers.callbacks.SDCFGCutoffCallback[[diffusers.callbacks.SDCFGCutoffCallback]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/callbacks.py#L69)

Callback function for Stable Diffusion Pipelines. After certain number of steps (set by `cutoff_step_ratio` or
`cutoff_step_index`), this callback will disable the CFG.

Note: This callback mutates the pipeline by changing the `_guidance_scale` attribute to 0.0 after the cutoff step.

#### diffusers.callbacks.SDXLCFGCutoffCallback[[diffusers.callbacks.SDXLCFGCutoffCallback]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/callbacks.py#L98)

Callback function for the base Stable Diffusion XL Pipelines. After certain number of steps (set by
`cutoff_step_ratio` or `cutoff_step_index`), this callback will disable the CFG.

Note: This callback mutates the pipeline by changing the `_guidance_scale` attribute to 0.0 after the cutoff step.

#### diffusers.callbacks.SDXLControlnetCFGCutoffCallback[[diffusers.callbacks.SDXLControlnetCFGCutoffCallback]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/callbacks.py#L140)

Callback function for the Controlnet Stable Diffusion XL Pipelines. After certain number of steps (set by
`cutoff_step_ratio` or `cutoff_step_index`), this callback will disable the CFG.

Note: This callback mutates the pipeline by changing the `_guidance_scale` attribute to 0.0 after the cutoff step.

#### diffusers.callbacks.IPAdapterScaleCutoffCallback[[diffusers.callbacks.IPAdapterScaleCutoffCallback]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/callbacks.py#L188)

Callback function for any pipeline that inherits `IPAdapterMixin`. After certain number of steps (set by
`cutoff_step_ratio` or `cutoff_step_index`), this callback will set the IP Adapter scale to `0.0`.

Note: This callback mutates the IP Adapter attention processors by setting the scale to 0.0 after the cutoff step.

#### diffusers.callbacks.SD3CFGCutoffCallback[[diffusers.callbacks.SD3CFGCutoffCallback]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/callbacks.py#L212)

Callback function for Stable Diffusion 3 Pipelines. After certain number of steps (set by `cutoff_step_ratio` or
`cutoff_step_index`), this callback will disable the CFG.

Note: This callback mutates the pipeline by changing the `_guidance_scale` attribute to 0.0 after the cutoff step.
