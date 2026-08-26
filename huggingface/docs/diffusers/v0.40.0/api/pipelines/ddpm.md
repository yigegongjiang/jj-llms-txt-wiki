# DDPM

[Denoising Diffusion Probabilistic Models](https://huggingface.co/papers/2006.11239) (DDPM) by Jonathan Ho, Ajay Jain and Pieter Abbeel proposes a diffusion based model of the same name. In the 🤗 Diffusers library, DDPM refers to the *discrete denoising scheduler* from the paper as well as the pipeline.

The abstract from the paper is:

*We present high quality image synthesis results using diffusion probabilistic models, a class of latent variable models inspired by considerations from nonequilibrium thermodynamics. Our best results are obtained by training on a weighted variational bound designed according to a novel connection between diffusion probabilistic models and denoising score matching with Langevin dynamics, and our models naturally admit a progressive lossy decompression scheme that can be interpreted as a generalization of autoregressive decoding. On the unconditional CIFAR10 dataset, we obtain an Inception score of 9.46 and a state-of-the-art FID score of 3.17. On 256x256 LSUN, we obtain sample quality similar to ProgressiveGAN.*

The original codebase can be found at [hohonathanho/diffusion](https://github.com/hojonathanho/diffusion).

> [!TIP]
> Make sure to check out the Schedulers [guide](../../using-diffusers/schedulers) to learn how to explore the tradeoff between scheduler speed and quality, and see the [reuse components across pipelines](../../using-diffusers/loading#reuse-a-pipeline) section to learn how to efficiently load the same components into multiple pipelines.

# DDPMPipeline[[diffusers.DDPMPipeline]]

#### diffusers.DDPMPipeline[[diffusers.DDPMPipeline]]

```python
diffusers.DDPMPipeline(unet: UNet2DModel, scheduler: DDPMScheduler)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ddpm/pipeline_ddpm.py#L33)

**Parameters:**

unet ([UNet2DModel](/docs/diffusers/v0.40.0/en/api/models/unet2d#diffusers.UNet2DModel)) : A `UNet2DModel` to denoise the encoded image latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `unet` to denoise the encoded image. Can be one of [DDPMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddpm#diffusers.DDPMScheduler), or [DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler).

Pipeline for image generation.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

#### __call__[[diffusers.DDPMPipeline.__call__]]

```python
__call__(batch_size: int = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, num_inference_steps: int = 1000, output_type: str | None = 'pil', return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ddpm/pipeline_ddpm.py#L54)

**Parameters:**

batch_size (`int`, *optional*, defaults to 1) : The number of images to generate.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

num_inference_steps (`int`, *optional*, defaults to 1000) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generated image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

If `return_dict` is `True`, [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) is returned, otherwise a `tuple` is
returned where the first element is a list with the generated images

The call function to the pipeline for generation.

Example:

```py
>>> from diffusers import DDPMPipeline

>>> # load model and scheduler
>>> pipe = DDPMPipeline.from_pretrained("google/ddpm-cat-256")

>>> # run pipeline in inference (sample random noise and denoise)
>>> image = pipe().images[0]

>>> # save image
>>> image.save("ddpm_generated_image.png")
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
