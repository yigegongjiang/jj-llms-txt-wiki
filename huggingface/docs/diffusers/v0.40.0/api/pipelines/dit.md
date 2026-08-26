# DiT

[Scalable Diffusion Models with Transformers](https://huggingface.co/papers/2212.09748) (DiT) is by William Peebles and Saining Xie.

The abstract from the paper is:

*We explore a new class of diffusion models based on the transformer architecture. We train latent diffusion models of images, replacing the commonly-used U-Net backbone with a transformer that operates on latent patches. We analyze the scalability of our Diffusion Transformers (DiTs) through the lens of forward pass complexity as measured by Gflops. We find that DiTs with higher Gflops -- through increased transformer depth/width or increased number of input tokens -- consistently have lower FID. In addition to possessing good scalability properties, our largest DiT-XL/2 models outperform all prior diffusion models on the class-conditional ImageNet 512x512 and 256x256 benchmarks, achieving a state-of-the-art FID of 2.27 on the latter.*

The original codebase can be found at [facebookresearch/dit](https://github.com/facebookresearch/dit).

> [!TIP]
> Make sure to check out the Schedulers [guide](../../using-diffusers/schedulers) to learn how to explore the tradeoff between scheduler speed and quality, and see the [reuse components across pipelines](../../using-diffusers/loading#reuse-a-pipeline) section to learn how to efficiently load the same components into multiple pipelines.

## DiTPipeline[[diffusers.DiTPipeline]]

#### diffusers.DiTPipeline[[diffusers.DiTPipeline]]

```python
diffusers.DiTPipeline(transformer: DiTTransformer2DModel, vae: AutoencoderKL, scheduler: KarrasDiffusionSchedulers, id2label: dict[int, str] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/dit/pipeline_dit.py#L38)

**Parameters:**

transformer ([DiTTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/dit_transformer2d#diffusers.DiTTransformer2DModel)) : A class conditioned `DiTTransformer2DModel` to denoise the encoded image latents. Initially published as [`Transformer2DModel`](https://huggingface.co/facebook/DiT-XL-2-256/blob/main/transformer/config.json#L2) in the config, but the mismatch can be ignored.

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) model to encode and decode images to and from latent representations.

scheduler ([DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

Pipeline for image generation based on a Transformer backbone instead of a UNet.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

#### __call__[[diffusers.DiTPipeline.__call__]]

```python
__call__(class_labels: list, guidance_scale: float = 4.0, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, num_inference_steps: int = 50, output_type: str | None = 'pil', return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/dit/pipeline_dit.py#L101)

**Parameters:**

class_labels (list[int]) : list of ImageNet class labels for the images to be generated.

guidance_scale (`float`, *optional*, defaults to 4.0) : A higher guidance scale value encourages the model to generate images closely linked to the text `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

num_inference_steps (`int`, *optional*, defaults to 250) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generated image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

If `return_dict` is `True`, [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) is returned, otherwise a `tuple` is
returned where the first element is a list with the generated images

The call function to the pipeline for generation.

Examples:

```py
>>> from diffusers import DiTPipeline, DPMSolverMultistepScheduler
>>> import torch

>>> pipe = DiTPipeline.from_pretrained("facebook/DiT-XL-2-256", torch_dtype=torch.float16)
>>> pipe.scheduler = DPMSolverMultistepScheduler.from_config(pipe.scheduler.config)
>>> pipe = pipe.to("cuda")

>>> # pick words from Imagenet class labels
>>> pipe.labels  # to print all available words

>>> # pick words that exist in ImageNet
>>> words = ["white shark", "umbrella"]

>>> class_ids = pipe.get_label_ids(words)

>>> generator = torch.manual_seed(33)
>>> output = pipe(class_labels=class_ids, num_inference_steps=25, generator=generator)

>>> image = output.images[0]  # label 'white shark'
```

#### get_label_ids[[diffusers.DiTPipeline.get_label_ids]]

```python
get_label_ids(label: str | list[str])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/dit/pipeline_dit.py#L76)

**Parameters:**

label (`str` or `dict` of `str`) : Label strings to be mapped to class ids.

**Returns:** `list` of `int`

Class ids to be processed by pipeline.

Map label strings from ImageNet to corresponding class ids.

## ImagePipelineOutput[[diffusers.ImagePipelineOutput]]

#### diffusers.ImagePipelineOutput[[diffusers.ImagePipelineOutput]]

```python
diffusers.ImagePipelineOutput(images: list[PIL.Image.Image] | numpy.ndarray)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/pipeline_utils.py#L135)

**Parameters:**

images (`List[PIL.Image.Image]` or `np.ndarray`) : List of denoised PIL images of length `batch_size` or NumPy array of shape `(batch_size, height, width, num_channels)`.

Output class for image pipelines.
