# DDIM

[Denoising Diffusion Implicit Models](https://huggingface.co/papers/2010.02502) (DDIM) by Jiaming Song, Chenlin Meng and Stefano Ermon.

The abstract from the paper is:

*Denoising diffusion probabilistic models (DDPMs) have achieved high quality image generation without adversarial training, yet they require simulating a Markov chain for many steps to produce a sample. To accelerate sampling, we present denoising diffusion implicit models (DDIMs), a more efficient class of iterative implicit probabilistic models with the same training procedure as DDPMs. In DDPMs, the generative process is defined as the reverse of a Markovian diffusion process. We construct a class of non-Markovian diffusion processes that lead to the same training objective, but whose reverse process can be much faster to sample from. We empirically demonstrate that DDIMs can produce high quality samples 10× to 50× faster in terms of wall-clock time compared to DDPMs, allow us to trade off computation for sample quality, and can perform semantically meaningful image interpolation directly in the latent space.*

The original codebase can be found at [ermongroup/ddim](https://github.com/ermongroup/ddim).

## DDIMPipeline[[diffusers.DDIMPipeline]]

#### diffusers.DDIMPipeline[[diffusers.DDIMPipeline]]

```python
diffusers.DDIMPipeline(unet: UNet2DModel, scheduler: DDIMScheduler)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ddim/pipeline_ddim.py#L37)

**Parameters:**

unet ([UNet2DModel](/docs/diffusers/v0.40.0/en/api/models/unet2d#diffusers.UNet2DModel)) : A `UNet2DModel` to denoise the encoded image latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `unet` to denoise the encoded image. Can be one of [DDPMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddpm#diffusers.DDPMScheduler), or [DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler).

Pipeline for image generation.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

#### __call__[[diffusers.DDIMPipeline.__call__]]

```python
__call__(batch_size: int = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, eta: float = 0.0, num_inference_steps: int = 50, use_clipped_model_output: bool | None = None, output_type: str | None = 'pil', return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ddim/pipeline_ddim.py#L62)

**Parameters:**

batch_size (`int`, *optional*, defaults to 1) : The number of images to generate.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

eta (`float`, *optional*, defaults to 0.0) : Corresponds to parameter eta (η) from the [DDIM](https://huggingface.co/papers/2010.02502) paper. Only applies to the [DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), and is ignored in other schedulers. A value of `0` corresponds to DDIM and `1` corresponds to DDPM.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

use_clipped_model_output (`bool`, *optional*, defaults to `None`) : If `True` or `False`, see documentation for [DDIMScheduler.step()](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler.step). If `None`, nothing is passed downstream to the scheduler (use `None` for schedulers which don't support this argument).

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generated image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

If `return_dict` is `True`, [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) is returned, otherwise a `tuple` is
returned where the first element is a list with the generated images

The call function to the pipeline for generation.

Example:

```py
>>> from diffusers import DDIMPipeline
>>> import PIL.Image
>>> import numpy as np

>>> # load model and scheduler
>>> pipe = DDIMPipeline.from_pretrained("fusing/ddim-lsun-bedroom")

>>> # run pipeline in inference (sample random noise and denoise)
>>> image = pipe(eta=0.0, num_inference_steps=50)

>>> # process image to PIL
>>> image_processed = image.cpu().permute(0, 2, 3, 1)
>>> image_processed = (image_processed + 1.0) * 127.5
>>> image_processed = image_processed.numpy().astype(np.uint8)
>>> image_pil = PIL.Image.fromarray(image_processed[0])

>>> # save image
>>> image_pil.save("test.png")
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
