# Consistency Models

Consistency Models were proposed in [Consistency Models](https://huggingface.co/papers/2303.01469) by Yang Song, Prafulla Dhariwal, Mark Chen, and Ilya Sutskever.

The abstract from the paper is:

*Diffusion models have significantly advanced the fields of image, audio, and video generation, but they depend on an iterative sampling process that causes slow generation. To overcome this limitation, we propose consistency models, a new family of models that generate high quality samples by directly mapping noise to data. They support fast one-step generation by design, while still allowing multistep sampling to trade compute for sample quality. They also support zero-shot data editing, such as image inpainting, colorization, and super-resolution, without requiring explicit training on these tasks. Consistency models can be trained either by distilling pre-trained diffusion models, or as standalone generative models altogether. Through extensive experiments, we demonstrate that they outperform existing distillation techniques for diffusion models in one- and few-step sampling, achieving the new state-of-the-art FID of 3.55 on CIFAR-10 and 6.20 on ImageNet 64x64 for one-step generation. When trained in isolation, consistency models become a new family of generative models that can outperform existing one-step, non-adversarial generative models on standard benchmarks such as CIFAR-10, ImageNet 64x64 and LSUN 256x256.*

The original codebase can be found at [openai/consistency_models](https://github.com/openai/consistency_models), and additional checkpoints are available at [openai](https://huggingface.co/openai).

The pipeline was contributed by [dg845](https://github.com/dg845) and [ayushtues](https://huggingface.co/ayushtues). ❤️

## Tips

For an additional speed-up, use `torch.compile` to generate multiple images in <1 second:

```diff
  import torch
  from diffusers import ConsistencyModelPipeline

  device = "cuda"
  # Load the cd_bedroom256_lpips checkpoint.
  model_id_or_path = "openai/diffusers-cd_bedroom256_lpips"
  pipe = ConsistencyModelPipeline.from_pretrained(model_id_or_path, dtype=torch.float16)
  pipe.to(device)

+ pipe.unet = torch.compile(pipe.unet, mode="reduce-overhead", fullgraph=True)

  # Multistep sampling
  # Timesteps can be explicitly specified; the particular timesteps below are from the original GitHub repo:
  # https://github.com/openai/consistency_models/blob/main/scripts/launch.sh#L83
  for _ in range(10):
      image = pipe(timesteps=[17, 0]).images[0]
      image.show()
```

## ConsistencyModelPipeline[[diffusers.ConsistencyModelPipeline]]

#### diffusers.ConsistencyModelPipeline[[diffusers.ConsistencyModelPipeline]]

```python
diffusers.ConsistencyModelPipeline(unet: UNet2DModel, scheduler: CMStochasticIterativeScheduler)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/consistency_models/pipeline_consistency_models.py#L67)

**Parameters:**

unet ([UNet2DModel](/docs/diffusers/v0.40.0/en/api/models/unet2d#diffusers.UNet2DModel)) : A `UNet2DModel` to denoise the encoded image latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `unet` to denoise the encoded image latents. Currently only compatible with [CMStochasticIterativeScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/cm_stochastic_iterative#diffusers.CMStochasticIterativeScheduler).

Pipeline for unconditional or class-conditional image generation.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

#### __call__[[diffusers.ConsistencyModelPipeline.__call__]]

```python
__call__(batch_size: int = 1, class_labels: typing.Union[torch.Tensor, list[int], int, NoneType] = None, num_inference_steps: int = 1, timesteps: list = None, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, callback: typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None, callback_steps: int = 1)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/consistency_models/pipeline_consistency_models.py#L171)

**Parameters:**

batch_size (`int`, *optional*, defaults to 1) : The number of images to generate.

class_labels (`torch.Tensor` or `list[int]` or `int`, *optional*) : Optional class labels for conditioning class-conditional consistency models. Not used if the model is not class-conditional.

num_inference_steps (`int`, *optional*, defaults to 1) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

timesteps (`list[int]`, *optional*) : Custom timesteps to use for the denoising process. If not defined, equal spaced `num_inference_steps` timesteps are used. Must be in descending order.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor is generated by sampling using the supplied random `generator`.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generated image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.

callback (`Callable`, *optional*) : A function that calls every `callback_steps` steps during inference. The function is called with the following arguments: `callback(step: int, timestep: int, latents: torch.Tensor)`.

callback_steps (`int`, *optional*, defaults to 1) : The frequency at which the `callback` function is called. If not specified, the callback is called at every step.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

If `return_dict` is `True`, [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) is returned, otherwise a `tuple` is
returned where the first element is a list with the generated images.

Examples:
```py
>>> import torch

>>> from diffusers import ConsistencyModelPipeline

>>> device = "cuda"
>>> # Load the cd_imagenet64_l2 checkpoint.
>>> model_id_or_path = "openai/diffusers-cd_imagenet64_l2"
>>> pipe = ConsistencyModelPipeline.from_pretrained(model_id_or_path, torch_dtype=torch.float16)
>>> pipe.to(device)

>>> # Onestep Sampling
>>> image = pipe(num_inference_steps=1).images[0]
>>> image.save("cd_imagenet64_l2_onestep_sample.png")

>>> # Onestep sampling, class-conditional image generation
>>> # ImageNet-64 class label 145 corresponds to king penguins
>>> image = pipe(num_inference_steps=1, class_labels=145).images[0]
>>> image.save("cd_imagenet64_l2_onestep_sample_penguin.png")

>>> # Multistep sampling, class-conditional image generation
>>> # Timesteps can be explicitly specified; the particular timesteps below are from the original GitHub repo:
>>> # https://github.com/openai/consistency_models/blob/main/scripts/launch.sh#L77
>>> image = pipe(num_inference_steps=None, timesteps=[22, 0], class_labels=145).images[0]
>>> image.save("cd_imagenet64_l2_multistep_sample_penguin.png")
```

## ImagePipelineOutput[[diffusers.ImagePipelineOutput]]

#### diffusers.ImagePipelineOutput[[diffusers.ImagePipelineOutput]]

```python
diffusers.ImagePipelineOutput(images: list[PIL.Image.Image] | numpy.ndarray)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/pipeline_utils.py#L135)

**Parameters:**

images (`List[PIL.Image.Image]` or `np.ndarray`) : List of denoised PIL images of length `batch_size` or NumPy array of shape `(batch_size, height, width, num_channels)`.

Output class for image pipelines.
