# LongCat-AudioDiT

LongCat-AudioDiT is a text-to-audio diffusion model from Meituan LongCat. The diffusers integration exposes a standard [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline) interface for text-conditioned audio generation.

This pipeline was adapted from the LongCat-AudioDiT reference implementation: https://github.com/meituan-longcat/LongCat-AudioDiT

This pipeline supports loading from a local directory or Hugging Face Hub repository in diffusers format (containing `text_encoder/`, `transformer/`, `vae/`, `tokenizer/`, and `scheduler/` subfolders).

## Usage

```py
import soundfile as sf
import torch
from diffusers import LongCatAudioDiTPipeline

pipeline = LongCatAudioDiTPipeline.from_pretrained(
    "ruixiangma/LongCat-AudioDiT-1B-Diffusers",
    torch_dtype=torch.float16,
)
pipeline = pipeline.to("cuda")

prompt = "A calm ocean wave ambience with soft wind in the background."
audio = pipeline(
    prompt,
    audio_duration_s=5.0,
    num_inference_steps=16,
    guidance_scale=4.0,
    generator=torch.Generator("cuda").manual_seed(42),
).audios[0, 0]

sf.write("longcat.wav", audio, pipeline.sample_rate)
```

## Tips

- `audio_duration_s` is the most direct way to control output duration.
- Use `generator=torch.Generator("cuda").manual_seed(42)` to make generation reproducible.
- Output shape is `(batch, channels, samples)` - use `.audios[0, 0]` to get a single audio sample.
- The pipeline outputs mono audio (1 channel). If you need stereo, you can duplicate the channel: `audio.unsqueeze(0).repeat(1, 2, 1)`.

## LongCatAudioDiTPipeline[[diffusers.LongCatAudioDiTPipeline]]

#### diffusers.LongCatAudioDiTPipeline[[diffusers.LongCatAudioDiTPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/longcat_audio_dit/pipeline_longcat_audio_dit.py#L99)

__call__diffusers.LongCatAudioDiTPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/longcat_audio_dit/pipeline_longcat_audio_dit.py#L219[{"name": "prompt", "val": ": str | list[str]"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "audio_duration_s", "val": ": float | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "num_inference_steps", "val": ": int = 16"}, {"name": "guidance_scale", "val": ": float = 4.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "output_type", "val": ": str = 'np'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}]- **prompt** (`str` or `list[str]`) -- Prompt or prompts that guide audio generation.
- **negative_prompt** (`str` or `list[str]`, *optional*) -- Negative prompt(s) for classifier-free guidance.
- **audio_duration_s** (`float`, *optional*) --
  Target audio duration in seconds. Ignored when `latents` is provided.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents of shape `(batch_size, duration, latent_dim)`.
- **num_inference_steps** (`int`, defaults to 16) -- Number of denoising steps.
- **guidance_scale** (`float`, defaults to 4.0) -- Guidance scale for classifier-free guidance.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) -- Random generator(s).
- **output_type** (`str`, defaults to `"np"`) -- Output format: `"np"`, `"pt"`, or `"latent"`.
- **return_dict** (`bool`, defaults to `True`) -- Whether to return `AudioPipelineOutput`.
- **callback_on_step_end** (`Callable`, *optional*) --
  A function called at the end of each denoising step with the pipeline, step index, timestep, and tensor
  inputs specified by `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`list`, defaults to `["latents"]`) --
  Tensor inputs passed to `callback_on_step_end`.0

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import soundfile as sf
>>> import torch
>>> from diffusers import LongCatAudioDiTPipeline

>>> pipe = LongCatAudioDiTPipeline.from_pretrained("ruixiangma/LongCat-AudioDiT-1B-Diffusers")
>>> pipe.to("cuda")

>>> prompt = "A calm ocean wave ambience with soft wind in the background."
>>> audio = pipe(
...     prompt,
...     audio_duration_s=5.0,
...     num_inference_steps=20,
...     guidance_scale=4.0,
...     generator=torch.Generator("cuda").manual_seed(42),
... ).audios[0, 0]
>>> sf.write("output.wav", audio, pipe.sample_rate)
```

**Parameters:**

prompt (`str` or `list[str]`) : Prompt or prompts that guide audio generation.

negative_prompt (`str` or `list[str]`, *optional*) : Negative prompt(s) for classifier-free guidance.

audio_duration_s (`float`, *optional*) : Target audio duration in seconds. Ignored when `latents` is provided.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents of shape `(batch_size, duration, latent_dim)`.

num_inference_steps (`int`, defaults to 16) : Number of denoising steps.

guidance_scale (`float`, defaults to 4.0) : Guidance scale for classifier-free guidance.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : Random generator(s).

output_type (`str`, defaults to `"np"`) : Output format: `"np"`, `"pt"`, or `"latent"`.

return_dict (`bool`, defaults to `True`) : Whether to return `AudioPipelineOutput`.

callback_on_step_end (`Callable`, *optional*) : A function called at the end of each denoising step with the pipeline, step index, timestep, and tensor inputs specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, defaults to `["latents"]`) : Tensor inputs passed to `callback_on_step_end`.
#### from_pretrained[[diffusers.LongCatAudioDiTPipeline.from_pretrained]]

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
