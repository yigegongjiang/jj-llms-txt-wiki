# AutoPipeline

The `AutoPipeline` is designed to make it easy to load a checkpoint for a task without needing to know the specific pipeline class. Based on the task, the `AutoPipeline` automatically retrieves the correct pipeline class from the checkpoint `model_index.json` file.

> [!TIP]
> Check out the [AutoPipeline](../../tutorials/autopipeline) tutorial to learn how to use this API!

## AutoPipelineForText2Image[[diffusers.AutoPipelineForText2Image]]

#### diffusers.AutoPipelineForText2Image[[diffusers.AutoPipelineForText2Image]]

```python
diffusers.AutoPipelineForText2Image(*args, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/auto_pipeline.py#L373)

[AutoPipelineForText2Image](/docs/diffusers/v0.40.0/en/api/pipelines/auto_pipeline#diffusers.AutoPipelineForText2Image) is a generic pipeline class that instantiates a text-to-image pipeline class. The
specific underlying pipeline class is automatically selected from either the
[from_pretrained()](/docs/diffusers/v0.40.0/en/api/pipelines/auto_pipeline#diffusers.AutoPipelineForText2Image.from_pretrained) or [from_pipe()](/docs/diffusers/v0.40.0/en/api/pipelines/auto_pipeline#diffusers.AutoPipelineForText2Image.from_pipe) methods.

This class cannot be instantiated using `__init__()` (throws an error).

Class attributes:

- **config_name** (`str`) -- The configuration filename that stores the class and module names of all the
  diffusion pipeline's components.

#### from_pretrained[[diffusers.AutoPipelineForText2Image.from_pretrained]]

```python
from_pretrained(pretrained_model_or_path, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/auto_pipeline.py#L398)

**Parameters:**

pretrained_model_or_path (`str` or `os.PathLike`, *optional*) : Can be either:  - A string, the *repo id* (for example `CompVis/ldm-text2im-large-256`) of a pretrained pipeline hosted on the Hub. - A path to a *directory* (for example `./my_pipeline_directory/`) containing pipeline weights saved using [save_pretrained()](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline.save_pretrained).

dtype (`torch.dtype`, *optional*) : Override the default `torch.dtype` and load the model with another dtype.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist.

cache_dir (`str | os.PathLike`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used. 

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

output_loading_info(`bool`, *optional*, defaults to `False`) : Whether or not to also return a dictionary containing missing keys, unexpected keys and error messages.

local_files_only (`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to `True`, the model won't be downloaded from the Hub.

token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

custom_revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, or a commit id similar to `revision` when loading a custom pipeline from the Hub. It can be a 🤗 Diffusers version when loading a custom pipeline from GitHub, otherwise it defaults to `"main"` when loading from the Hub.

mirror (`str`, *optional*) : Mirror source to resolve accessibility issues if you’re downloading a model in China. We do not guarantee the timeliness or safety of the source, and you should refer to the mirror site for more information.

device_map (`str` or `dict[str, int | str | torch.device]`, *optional*) : A map that specifies where each submodule should go. It doesn’t need to be defined for each parameter/buffer name; once a given module name is inside, every submodule of it will be sent to the same device.  Set `device_map="auto"` to have 🤗 Accelerate automatically compute the most optimized `device_map`. For more information about each option see [designing a device map](https://hf.co/docs/accelerate/main/en/usage_guides/big_modeling#designing-a-device-map).

max_memory (`Dict`, *optional*) : A dictionary device identifier for the maximum memory. Will default to the maximum memory available for each GPU and the available CPU RAM if unset.

offload_folder (`str` or `os.PathLike`, *optional*) : The path to offload weights if device_map contains the value `"disk"`.

offload_state_dict (`bool`, *optional*) : If `True`, temporarily offloads the CPU state dict to the hard drive to avoid running out of CPU RAM if the weight of the CPU state dict + the biggest shard of the checkpoint does not fit. Defaults to `True` when there is some disk offload.

low_cpu_mem_usage (`bool`, *optional*, defaults to `True` if torch version >= 1.9.0 else `False`) : Speed up model loading only loading the pretrained weights and not initializing the weights. This also tries to not use more than 1x model size in CPU memory (including peak memory) while loading the model. Only supported for PyTorch >= 1.9.0. If you are using an older version of PyTorch, setting this argument to `True` will raise an error.

use_safetensors (`bool`, *optional*, defaults to `None`) : If set to `None`, the safetensors weights are downloaded if they're available **and** if the safetensors library is installed. If set to `True`, the model is forcibly loaded from safetensors weights. If set to `False`, safetensors weights are not loaded.

kwargs (remaining dictionary of keyword arguments, *optional*) : Can be used to overwrite load and saveable variables (the pipeline components of the specific pipeline class). The overwritten components are passed directly to the pipelines `__init__` method. See example below for more information.

variant (`str`, *optional*) : Load weights from a specified variant filename such as `"fp16"` or `"ema"`.

Instantiates a text-to-image Pytorch diffusion pipeline from pretrained pipeline weight.

The from_pretrained() method takes care of returning the correct pipeline class instance by:
1. Detect the pipeline class of the pretrained_model_or_path based on the _class_name property of its
   config object
2. Find the text-to-image pipeline linked to the pipeline class using pattern matching on pipeline class
   name.

If a `controlnet` argument is passed, it will instantiate a [StableDiffusionControlNetPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/controlnet#diffusers.StableDiffusionControlNetPipeline) object.

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
>>> from diffusers import AutoPipelineForText2Image

>>> pipeline = AutoPipelineForText2Image.from_pretrained("stable-diffusion-v1-5/stable-diffusion-v1-5")
>>> image = pipeline(prompt).images[0]
```

#### from_pipe[[diffusers.AutoPipelineForText2Image.from_pipe]]

```python
from_pipe(pipeline, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/auto_pipeline.py#L545)

**Parameters:**

pipeline (`DiffusionPipeline`) : an instantiated `DiffusionPipeline` object

Instantiates a text-to-image Pytorch diffusion pipeline from another instantiated diffusion pipeline class.

The from_pipe() method takes care of returning the correct pipeline class instance by finding the text-to-image
pipeline linked to the pipeline class using pattern matching on pipeline class name.

All the modules the pipeline contains will be used to initialize the new pipeline without reallocating
additional memory.

The pipeline is set in evaluation mode (`model.eval()`) by default.

```py
>>> from diffusers import AutoPipelineForText2Image, AutoPipelineForImage2Image

>>> pipe_i2i = AutoPipelineForImage2Image.from_pretrained(
...     "stable-diffusion-v1-5/stable-diffusion-v1-5", requires_safety_checker=False
... )

>>> pipe_t2i = AutoPipelineForText2Image.from_pipe(pipe_i2i)
>>> image = pipe_t2i(prompt).images[0]
```

## AutoPipelineForImage2Image[[diffusers.AutoPipelineForImage2Image]]

#### diffusers.AutoPipelineForImage2Image[[diffusers.AutoPipelineForImage2Image]]

```python
diffusers.AutoPipelineForImage2Image(*args, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/auto_pipeline.py#L662)

[AutoPipelineForImage2Image](/docs/diffusers/v0.40.0/en/api/pipelines/auto_pipeline#diffusers.AutoPipelineForImage2Image) is a generic pipeline class that instantiates an image-to-image pipeline class. The
specific underlying pipeline class is automatically selected from either the
[from_pretrained()](/docs/diffusers/v0.40.0/en/api/pipelines/auto_pipeline#diffusers.AutoPipelineForImage2Image.from_pretrained) or [from_pipe()](/docs/diffusers/v0.40.0/en/api/pipelines/auto_pipeline#diffusers.AutoPipelineForImage2Image.from_pipe) methods.

This class cannot be instantiated using `__init__()` (throws an error).

Class attributes:

- **config_name** (`str`) -- The configuration filename that stores the class and module names of all the
  diffusion pipeline's components.

#### from_pretrained[[diffusers.AutoPipelineForImage2Image.from_pretrained]]

```python
from_pretrained(pretrained_model_or_path, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/auto_pipeline.py#L687)

**Parameters:**

pretrained_model_or_path (`str` or `os.PathLike`, *optional*) : Can be either:  - A string, the *repo id* (for example `CompVis/ldm-text2im-large-256`) of a pretrained pipeline hosted on the Hub. - A path to a *directory* (for example `./my_pipeline_directory/`) containing pipeline weights saved using [save_pretrained()](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline.save_pretrained).

dtype (`str` or `torch.dtype`, *optional*) : Override the default `torch.dtype` and load the model with another dtype.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist.

cache_dir (`str | os.PathLike`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used. 

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

output_loading_info(`bool`, *optional*, defaults to `False`) : Whether or not to also return a dictionary containing missing keys, unexpected keys and error messages.

local_files_only (`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to `True`, the model won't be downloaded from the Hub.

token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

custom_revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, or a commit id similar to `revision` when loading a custom pipeline from the Hub. It can be a 🤗 Diffusers version when loading a custom pipeline from GitHub, otherwise it defaults to `"main"` when loading from the Hub.

mirror (`str`, *optional*) : Mirror source to resolve accessibility issues if you’re downloading a model in China. We do not guarantee the timeliness or safety of the source, and you should refer to the mirror site for more information.

device_map (`str` or `dict[str, int | str | torch.device]`, *optional*) : A map that specifies where each submodule should go. It doesn’t need to be defined for each parameter/buffer name; once a given module name is inside, every submodule of it will be sent to the same device.  Set `device_map="auto"` to have 🤗 Accelerate automatically compute the most optimized `device_map`. For more information about each option see [designing a device map](https://hf.co/docs/accelerate/main/en/usage_guides/big_modeling#designing-a-device-map).

max_memory (`Dict`, *optional*) : A dictionary device identifier for the maximum memory. Will default to the maximum memory available for each GPU and the available CPU RAM if unset.

offload_folder (`str` or `os.PathLike`, *optional*) : The path to offload weights if device_map contains the value `"disk"`.

offload_state_dict (`bool`, *optional*) : If `True`, temporarily offloads the CPU state dict to the hard drive to avoid running out of CPU RAM if the weight of the CPU state dict + the biggest shard of the checkpoint does not fit. Defaults to `True` when there is some disk offload.

low_cpu_mem_usage (`bool`, *optional*, defaults to `True` if torch version >= 1.9.0 else `False`) : Speed up model loading only loading the pretrained weights and not initializing the weights. This also tries to not use more than 1x model size in CPU memory (including peak memory) while loading the model. Only supported for PyTorch >= 1.9.0. If you are using an older version of PyTorch, setting this argument to `True` will raise an error.

use_safetensors (`bool`, *optional*, defaults to `None`) : If set to `None`, the safetensors weights are downloaded if they're available **and** if the safetensors library is installed. If set to `True`, the model is forcibly loaded from safetensors weights. If set to `False`, safetensors weights are not loaded.

kwargs (remaining dictionary of keyword arguments, *optional*) : Can be used to overwrite load and saveable variables (the pipeline components of the specific pipeline class). The overwritten components are passed directly to the pipelines `__init__` method. See example below for more information.

variant (`str`, *optional*) : Load weights from a specified variant filename such as `"fp16"` or `"ema"`.

Instantiates a image-to-image Pytorch diffusion pipeline from pretrained pipeline weight.

The from_pretrained() method takes care of returning the correct pipeline class instance by:
1. Detect the pipeline class of the pretrained_model_or_path based on the _class_name property of its
   config object
2. Find the image-to-image pipeline linked to the pipeline class using pattern matching on pipeline class
   name.

If a `controlnet` argument is passed, it will instantiate a [StableDiffusionControlNetImg2ImgPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/controlnet#diffusers.StableDiffusionControlNetImg2ImgPipeline)
object.

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
>>> from diffusers import AutoPipelineForImage2Image

>>> pipeline = AutoPipelineForImage2Image.from_pretrained("stable-diffusion-v1-5/stable-diffusion-v1-5")
>>> image = pipeline(prompt, image).images[0]
```

#### from_pipe[[diffusers.AutoPipelineForImage2Image.from_pipe]]

```python
from_pipe(pipeline, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/auto_pipeline.py#L845)

**Parameters:**

pipeline (`DiffusionPipeline`) : an instantiated `DiffusionPipeline` object

Instantiates a image-to-image Pytorch diffusion pipeline from another instantiated diffusion pipeline class.

The from_pipe() method takes care of returning the correct pipeline class instance by finding the
image-to-image pipeline linked to the pipeline class using pattern matching on pipeline class name.

All the modules the pipeline contains will be used to initialize the new pipeline without reallocating
additional memory.

The pipeline is set in evaluation mode (`model.eval()`) by default.

Examples:

```py
>>> from diffusers import AutoPipelineForText2Image, AutoPipelineForImage2Image

>>> pipe_t2i = AutoPipelineForText2Image.from_pretrained(
...     "stable-diffusion-v1-5/stable-diffusion-v1-5", requires_safety_checker=False
... )

>>> pipe_i2i = AutoPipelineForImage2Image.from_pipe(pipe_t2i)
>>> image = pipe_i2i(prompt, image).images[0]
```

## AutoPipelineForInpainting[[diffusers.AutoPipelineForInpainting]]

#### diffusers.AutoPipelineForInpainting[[diffusers.AutoPipelineForInpainting]]

```python
diffusers.AutoPipelineForInpainting(*args, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/auto_pipeline.py#L967)

[AutoPipelineForInpainting](/docs/diffusers/v0.40.0/en/api/pipelines/auto_pipeline#diffusers.AutoPipelineForInpainting) is a generic pipeline class that instantiates an inpainting pipeline class. The
specific underlying pipeline class is automatically selected from either the
[from_pretrained()](/docs/diffusers/v0.40.0/en/api/pipelines/auto_pipeline#diffusers.AutoPipelineForInpainting.from_pretrained) or [from_pipe()](/docs/diffusers/v0.40.0/en/api/pipelines/auto_pipeline#diffusers.AutoPipelineForInpainting.from_pipe) methods.

This class cannot be instantiated using `__init__()` (throws an error).

Class attributes:

- **config_name** (`str`) -- The configuration filename that stores the class and module names of all the
  diffusion pipeline's components.

#### from_pretrained[[diffusers.AutoPipelineForInpainting.from_pretrained]]

```python
from_pretrained(pretrained_model_or_path, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/auto_pipeline.py#L992)

**Parameters:**

pretrained_model_or_path (`str` or `os.PathLike`, *optional*) : Can be either:  - A string, the *repo id* (for example `CompVis/ldm-text2im-large-256`) of a pretrained pipeline hosted on the Hub. - A path to a *directory* (for example `./my_pipeline_directory/`) containing pipeline weights saved using [save_pretrained()](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline.save_pretrained).

dtype (`str` or `torch.dtype`, *optional*) : Override the default `torch.dtype` and load the model with another dtype.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist.

cache_dir (`str | os.PathLike`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used. 

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

output_loading_info(`bool`, *optional*, defaults to `False`) : Whether or not to also return a dictionary containing missing keys, unexpected keys and error messages.

local_files_only (`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to `True`, the model won't be downloaded from the Hub.

token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

custom_revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, or a commit id similar to `revision` when loading a custom pipeline from the Hub. It can be a 🤗 Diffusers version when loading a custom pipeline from GitHub, otherwise it defaults to `"main"` when loading from the Hub.

mirror (`str`, *optional*) : Mirror source to resolve accessibility issues if you’re downloading a model in China. We do not guarantee the timeliness or safety of the source, and you should refer to the mirror site for more information.

device_map (`str` or `dict[str, int | str | torch.device]`, *optional*) : A map that specifies where each submodule should go. It doesn’t need to be defined for each parameter/buffer name; once a given module name is inside, every submodule of it will be sent to the same device.  Set `device_map="auto"` to have 🤗 Accelerate automatically compute the most optimized `device_map`. For more information about each option see [designing a device map](https://hf.co/docs/accelerate/main/en/usage_guides/big_modeling#designing-a-device-map).

max_memory (`Dict`, *optional*) : A dictionary device identifier for the maximum memory. Will default to the maximum memory available for each GPU and the available CPU RAM if unset.

offload_folder (`str` or `os.PathLike`, *optional*) : The path to offload weights if device_map contains the value `"disk"`.

offload_state_dict (`bool`, *optional*) : If `True`, temporarily offloads the CPU state dict to the hard drive to avoid running out of CPU RAM if the weight of the CPU state dict + the biggest shard of the checkpoint does not fit. Defaults to `True` when there is some disk offload.

low_cpu_mem_usage (`bool`, *optional*, defaults to `True` if torch version >= 1.9.0 else `False`) : Speed up model loading only loading the pretrained weights and not initializing the weights. This also tries to not use more than 1x model size in CPU memory (including peak memory) while loading the model. Only supported for PyTorch >= 1.9.0. If you are using an older version of PyTorch, setting this argument to `True` will raise an error.

use_safetensors (`bool`, *optional*, defaults to `None`) : If set to `None`, the safetensors weights are downloaded if they're available **and** if the safetensors library is installed. If set to `True`, the model is forcibly loaded from safetensors weights. If set to `False`, safetensors weights are not loaded.

kwargs (remaining dictionary of keyword arguments, *optional*) : Can be used to overwrite load and saveable variables (the pipeline components of the specific pipeline class). The overwritten components are passed directly to the pipelines `__init__` method. See example below for more information.

variant (`str`, *optional*) : Load weights from a specified variant filename such as `"fp16"` or `"ema"`.

Instantiates a inpainting Pytorch diffusion pipeline from pretrained pipeline weight.

The from_pretrained() method takes care of returning the correct pipeline class instance by:
1. Detect the pipeline class of the pretrained_model_or_path based on the _class_name property of its
   config object
2. Find the inpainting pipeline linked to the pipeline class using pattern matching on pipeline class name.

If a `controlnet` argument is passed, it will instantiate a [StableDiffusionControlNetInpaintPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/controlnet#diffusers.StableDiffusionControlNetInpaintPipeline)
object.

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
>>> from diffusers import AutoPipelineForInpainting

>>> pipeline = AutoPipelineForInpainting.from_pretrained("stable-diffusion-v1-5/stable-diffusion-v1-5")
>>> image = pipeline(prompt, image=init_image, mask_image=mask_image).images[0]
```

#### from_pipe[[diffusers.AutoPipelineForInpainting.from_pipe]]

```python
from_pipe(pipeline, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/auto_pipeline.py#L1147)

**Parameters:**

pipeline (`DiffusionPipeline`) : an instantiated `DiffusionPipeline` object

Instantiates a inpainting Pytorch diffusion pipeline from another instantiated diffusion pipeline class.

The from_pipe() method takes care of returning the correct pipeline class instance by finding the inpainting
pipeline linked to the pipeline class using pattern matching on pipeline class name.

All the modules the pipeline class contain will be used to initialize the new pipeline without reallocating
additional memory.

The pipeline is set in evaluation mode (`model.eval()`) by default.

Examples:

```py
>>> from diffusers import AutoPipelineForText2Image, AutoPipelineForInpainting

>>> pipe_t2i = AutoPipelineForText2Image.from_pretrained(
...     "DeepFloyd/IF-I-XL-v1.0", requires_safety_checker=False
... )

>>> pipe_inpaint = AutoPipelineForInpainting.from_pipe(pipe_t2i)
>>> image = pipe_inpaint(prompt, image=init_image, mask_image=mask_image).images[0]
```

## AutoPipelineForText2Audio[[diffusers.AutoPipelineForText2Audio]]

#### diffusers.AutoPipelineForText2Audio[[diffusers.AutoPipelineForText2Audio]]

```python
diffusers.AutoPipelineForText2Audio(*args, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/auto_pipeline.py#L1266)

[AutoPipelineForText2Audio](/docs/diffusers/v0.40.0/en/api/pipelines/auto_pipeline#diffusers.AutoPipelineForText2Audio) is a generic pipeline class that instantiates a text-to-audio pipeline class. The
specific underlying pipeline class is automatically selected from either the
[from_pretrained()](/docs/diffusers/v0.40.0/en/api/pipelines/auto_pipeline#diffusers.AutoPipelineForText2Audio.from_pretrained) or [from_pipe()](/docs/diffusers/v0.40.0/en/api/pipelines/auto_pipeline#diffusers.AutoPipelineForText2Audio.from_pipe) methods.

This class cannot be instantiated using `__init__()` (throws an error).

Class attributes:

- **config_name** (`str`) -- The configuration filename that stores the class and module names of all the
  diffusion pipeline's components.

#### from_pretrained[[diffusers.AutoPipelineForText2Audio.from_pretrained]]

```python
from_pretrained(pretrained_model_or_path, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/auto_pipeline.py#L1291)

**Parameters:**

pretrained_model_or_path (`str` or `os.PathLike`, *optional*) : Can be either:  - A string, the *repo id* (for example `stabilityai/stable-audio-open-1.0`) of a pretrained pipeline hosted on the Hub. - A path to a *directory* (for example `./my_pipeline_directory/`) containing pipeline weights saved using [save_pretrained()](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline.save_pretrained).

dtype (`torch.dtype`, *optional*) : Override the default `torch.dtype` and load the model with another dtype.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist.

cache_dir (`str | os.PathLike`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used. 

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

output_loading_info(`bool`, *optional*, defaults to `False`) : Whether or not to also return a dictionary containing missing keys, unexpected keys and error messages.

local_files_only (`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to `True`, the model won't be downloaded from the Hub.

token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

custom_revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, or a commit id similar to `revision` when loading a custom pipeline from the Hub. It can be a 🤗 Diffusers version when loading a custom pipeline from GitHub, otherwise it defaults to `"main"` when loading from the Hub.

mirror (`str`, *optional*) : Mirror source to resolve accessibility issues if you're downloading a model in China. We do not guarantee the timeliness or safety of the source, and you should refer to the mirror site for more information.

device_map (`str` or `dict[str, int | str | torch.device]`, *optional*) : A map that specifies where each submodule should go. It doesn't need to be defined for each parameter/buffer name; once a given module name is inside, every submodule of it will be sent to the same device.  Set `device_map="auto"` to have 🤗 Accelerate automatically compute the most optimized `device_map`. For more information about each option see [designing a device map](https://hf.co/docs/accelerate/main/en/usage_guides/big_modeling#designing-a-device-map).

max_memory (`Dict`, *optional*) : A dictionary device identifier for the maximum memory. Will default to the maximum memory available for each GPU and the available CPU RAM if unset.

offload_folder (`str` or `os.PathLike`, *optional*) : The path to offload weights if device_map contains the value `"disk"`.

offload_state_dict (`bool`, *optional*) : If `True`, temporarily offloads the CPU state dict to the hard drive to avoid running out of CPU RAM if the weight of the CPU state dict + the biggest shard of the checkpoint does not fit. Defaults to `True` when there is some disk offload.

low_cpu_mem_usage (`bool`, *optional*, defaults to `True` if torch version >= 1.9.0 else `False`) : Speed up model loading only loading the pretrained weights and not initializing the weights. This also tries to not use more than 1x model size in CPU memory (including peak memory) while loading the model. Only supported for PyTorch >= 1.9.0. If you are using an older version of PyTorch, setting this argument to `True` will raise an error.

use_safetensors (`bool`, *optional*, defaults to `None`) : If set to `None`, the safetensors weights are downloaded if they're available **and** if the safetensors library is installed. If set to `True`, the model is forcibly loaded from safetensors weights. If set to `False`, safetensors weights are not loaded.

kwargs (remaining dictionary of keyword arguments, *optional*) : Can be used to overwrite load and saveable variables (the pipeline components of the specific pipeline class). The overwritten components are passed directly to the pipelines `__init__` method. See example below for more information.

variant (`str`, *optional*) : Load weights from a specified variant filename such as `"fp16"` or `"ema"`.

Instantiates a text-to-audio Pytorch diffusion pipeline from pretrained pipeline weight.

The from_pretrained() method takes care of returning the correct pipeline class instance by:
1. Detect the pipeline class of the pretrained_model_or_path based on the _class_name property of its
   config object
2. Find the text-to-audio pipeline linked to the pipeline class using pattern matching on pipeline class
   name.

The pipeline is set in evaluation mode (`model.eval()`) by default.

> [!TIP] > To use private or [gated](https://huggingface.co/docs/hub/models-gated#gated-models) models, log-in
with `hf > auth login`.

Examples:

```py
>>> import torch
>>> import soundfile as sf
>>> from diffusers import AutoPipelineForText2Audio

>>> pipeline = AutoPipelineForText2Audio.from_pretrained(
...     "stabilityai/stable-audio-open-1.0", torch_dtype=torch.float16
... )
>>> pipeline = pipeline.to("cuda")

>>> output = pipeline(
...     "Generate a male voice reading a paragraph",
...     num_inference_steps=200,
...     audio_end_in_s=10.0,
... )
>>> audio = output.audios[0].T.float().cpu().numpy()
>>> sf.write("audio.wav", audio, pipeline.vae.sampling_rate)
```

#### from_pipe[[diffusers.AutoPipelineForText2Audio.from_pipe]]

```python
from_pipe(pipeline, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/auto_pipeline.py#L1426)

**Parameters:**

pipeline (`DiffusionPipeline`) : an instantiated `DiffusionPipeline` object

Instantiates a text-to-audio Pytorch diffusion pipeline from another instantiated diffusion pipeline class.

The from_pipe() method takes care of returning the correct pipeline class instance by finding the text-to-audio
pipeline linked to the pipeline class using pattern matching on pipeline class name.

All the modules the pipeline contains will be used to initialize the new pipeline without reallocating
additional memory.

The pipeline is set in evaluation mode (`model.eval()`) by default.

```py
>>> import torch
>>> import soundfile as sf
>>> from diffusers import AutoPipelineForText2Audio, StableAudioPipeline

>>> pipe = StableAudioPipeline.from_pretrained("stabilityai/stable-audio-open-1.0", torch_dtype=torch.float16)

>>> pipe_audio = AutoPipelineForText2Audio.from_pipe(pipe)
>>> output = pipe_audio(
...     "Generate a sound",
...     num_inference_steps=200,
...     audio_end_in_s=10.0,
... )
>>> audio = output.audios[0].T.float().cpu().numpy()
>>> sf.write("audio.wav", audio, pipe_audio.vae.sampling_rate)
```
