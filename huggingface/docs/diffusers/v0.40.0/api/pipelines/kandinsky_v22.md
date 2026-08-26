# Kandinsky 2.2

Kandinsky 2.2 is created by [Arseniy Shakhmatov](https://github.com/cene555), [Anton Razzhigaev](https://github.com/razzant), [Aleksandr Nikolich](https://github.com/AlexWortega), [Vladimir Arkhipkin](https://github.com/oriBetelgeuse), [Igor Pavlov](https://github.com/boomb0om), [Andrey Kuznetsov](https://github.com/kuznetsoffandrey), and [Denis Dimitrov](https://github.com/denndimitrov).

The description from it's GitHub page is:

*Kandinsky 2.2 brings substantial improvements upon its predecessor, Kandinsky 2.1, by introducing a new, more powerful image encoder - CLIP-ViT-G and the ControlNet support. The switch to CLIP-ViT-G as the image encoder significantly increases the model's capability to generate more aesthetic pictures and better understand text, thus enhancing the model's overall performance. The addition of the ControlNet mechanism allows the model to effectively control the process of generating images. This leads to more accurate and visually appealing outputs and opens new possibilities for text-guided image manipulation.*

The original codebase can be found at [ai-forever/Kandinsky-2](https://github.com/ai-forever/Kandinsky-2).

> [!TIP]
> Check out the [Kandinsky Community](https://huggingface.co/kandinsky-community) organization on the Hub for the official model checkpoints for tasks like text-to-image, image-to-image, and inpainting.

> [!TIP]
> Make sure to check out the schedulers [guide](../../using-diffusers/schedulers) to learn how to explore the tradeoff between scheduler speed and quality, and see the [reuse components across pipelines](../../using-diffusers/loading#reuse-a-pipeline) section to learn how to efficiently load the same components into multiple pipelines.

## KandinskyV22PriorPipeline[[diffusers.KandinskyV22PriorPipeline]]

#### diffusers.KandinskyV22PriorPipeline[[diffusers.KandinskyV22PriorPipeline]]

```python
diffusers.KandinskyV22PriorPipeline(prior: PriorTransformer, image_encoder: CLIPVisionModelWithProjection, text_encoder: CLIPTextModelWithProjection, tokenizer: CLIPTokenizer, scheduler: UnCLIPScheduler, image_processor: CLIPImageProcessorPil)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_prior.py#L89)

**Parameters:**

prior ([PriorTransformer](/docs/diffusers/v0.40.0/en/api/models/prior_transformer#diffusers.PriorTransformer)) : The canonical unCLIP prior to approximate the image embedding from the text embedding.

image_encoder (`CLIPVisionModelWithProjection`) : Frozen image-encoder.

text_encoder (`CLIPTextModelWithProjection`) : Frozen text-encoder.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

scheduler (`UnCLIPScheduler`) : A scheduler to be used in combination with `prior` to generate image embedding.

image_processor (`CLIPImageProcessor`) : A image_processor to be used to preprocess image from clip.

Pipeline for generating image prior for Kandinsky

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.KandinskyV22PriorPipeline.__call__]]

```python
__call__(prompt: str | list[str], negative_prompt: str | list[str] | None = None, num_images_per_prompt: int = 1, num_inference_steps: int = 25, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, guidance_scale: float = 4.0, output_type: str | None = 'pt', return_dict: bool = True, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_prior.py#L375)

**Parameters:**

prompt (`str` or `list[str]`) : The prompt or prompts to guide the image generation.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

output_type (`str`, *optional*, defaults to `"pt"`) : The output format of the generate image. Choose between: `"np"` (`np.array`) or `"pt"` (`torch.Tensor`).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

**Returns:** `KandinskyPriorPipelineOutput` or `tuple`

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> from diffusers import KandinskyV22Pipeline, KandinskyV22PriorPipeline
>>> import torch

>>> pipe_prior = KandinskyV22PriorPipeline.from_pretrained("kandinsky-community/kandinsky-2-2-prior")
>>> pipe_prior.to("cuda")
>>> prompt = "red cat, 4k photo"
>>> image_emb, negative_image_emb = pipe_prior(prompt).to_tuple()

>>> pipe = KandinskyV22Pipeline.from_pretrained("kandinsky-community/kandinsky-2-2-decoder")
>>> pipe.to("cuda")
>>> image = pipe(
...     image_embeds=image_emb,
...     negative_image_embeds=negative_image_emb,
...     height=768,
...     width=768,
...     num_inference_steps=50,
... ).images
>>> image[0].save("cat.png")
```

#### interpolate[[diffusers.KandinskyV22PriorPipeline.interpolate]]

```python
interpolate(images_and_prompts: list, weights: list, num_images_per_prompt: int = 1, num_inference_steps: int = 25, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, negative_prior_prompt: str | None = None, negative_prompt: str = '', guidance_scale: float = 4.0, device = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_prior.py#L136)

**Parameters:**

images_and_prompts (`list[str | PIL.Image.Image | torch.Tensor]`) : list of prompts and images to guide the image generation.

weights : (`list[float]`): list of weights for each condition in `images_and_prompts`

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

negative_prior_prompt (`str`, *optional*) : The prompt not to guide the prior diffusion process. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

negative_prompt (`str` or `list[str]`, *optional*) : The prompt not to guide the image generation. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

**Returns:** `KandinskyPriorPipelineOutput` or `tuple`

Function invoked when using the prior pipeline for interpolation.

Examples:
```py
>>> from diffusers import KandinskyV22PriorPipeline, KandinskyV22Pipeline
>>> from diffusers.utils import load_image
>>> import PIL
>>> import torch
>>> from torchvision import transforms

>>> pipe_prior = KandinskyV22PriorPipeline.from_pretrained(
...     "kandinsky-community/kandinsky-2-2-prior", torch_dtype=torch.float16
... )
>>> pipe_prior.to("cuda")
>>> img1 = load_image(
...     "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main"
...     "/kandinsky/cat.png"
... )
>>> img2 = load_image(
...     "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main"
...     "/kandinsky/starry_night.jpeg"
... )
>>> images_texts = ["a cat", img1, img2]
>>> weights = [0.3, 0.3, 0.4]
>>> out = pipe_prior.interpolate(images_texts, weights)
>>> pipe = KandinskyV22Pipeline.from_pretrained(
...     "kandinsky-community/kandinsky-2-2-decoder", torch_dtype=torch.float16
... )
>>> pipe.to("cuda")
>>> image = pipe(
...     image_embeds=out.image_embeds,
...     negative_image_embeds=out.negative_image_embeds,
...     height=768,
...     width=768,
...     num_inference_steps=50,
... ).images[0]
>>> image.save("starry_cat.png")
```

## KandinskyV22Pipeline[[diffusers.KandinskyV22Pipeline]]

#### diffusers.KandinskyV22Pipeline[[diffusers.KandinskyV22Pipeline]]

```python
diffusers.KandinskyV22Pipeline(unet: UNet2DConditionModel, scheduler: DDPMScheduler, movq: VQModel)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2.py#L72)

**Parameters:**

scheduler (`DDIMScheduler` | `DDPMScheduler`) : A scheduler to be used in combination with `unet` to generate image latents.

unet ([UNet2DConditionModel](/docs/diffusers/v0.40.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the image embedding.

movq ([VQModel](/docs/diffusers/v0.40.0/en/api/models/vq#diffusers.VQModel)) : MoVQ Decoder to generate the image from the latents.

Pipeline for text-to-image generation using Kandinsky

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.KandinskyV22Pipeline.__call__]]

```python
__call__(image_embeds: typing.Union[torch.Tensor, list[torch.Tensor]], negative_image_embeds: typing.Union[torch.Tensor, list[torch.Tensor]], height: int = 512, width: int = 512, num_inference_steps: int = 100, guidance_scale: float = 4.0, num_images_per_prompt: int = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2.py#L130)

**Parameters:**

image_embeds (`torch.Tensor` or `list[torch.Tensor]`) : The clip image embeddings for text prompt, that will be used to condition the image generation.

negative_image_embeds (`torch.Tensor` or `list[torch.Tensor]`) : The clip image embeddings for negative text prompt, will be used to condition the image generation.

height (`int`, *optional*, defaults to 512) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to 512) : The width in pixels of the generated image.

num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between: `"pil"` (`PIL.Image.Image`), `"np"` (`np.array`) or `"pt"` (`torch.Tensor`).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> from diffusers import KandinskyV22Pipeline, KandinskyV22PriorPipeline
>>> import torch

>>> pipe_prior = KandinskyV22PriorPipeline.from_pretrained("kandinsky-community/kandinsky-2-2-prior")
>>> pipe_prior.to("cuda")
>>> prompt = "red cat, 4k photo"
>>> out = pipe_prior(prompt)
>>> image_emb = out.image_embeds
>>> zero_image_emb = out.negative_image_embeds
>>> pipe = KandinskyV22Pipeline.from_pretrained("kandinsky-community/kandinsky-2-2-decoder")
>>> pipe.to("cuda")
>>> image = pipe(
...     image_embeds=image_emb,
...     negative_image_embeds=zero_image_emb,
...     height=768,
...     width=768,
...     num_inference_steps=50,
... ).images
>>> image[0].save("cat.png")
```

## KandinskyV22CombinedPipeline[[diffusers.KandinskyV22CombinedPipeline]]

#### diffusers.KandinskyV22CombinedPipeline[[diffusers.KandinskyV22CombinedPipeline]]

```python
diffusers.KandinskyV22CombinedPipeline(unet: UNet2DConditionModel, scheduler: DDPMScheduler, movq: VQModel, prior_prior: PriorTransformer, prior_image_encoder: CLIPVisionModelWithProjection, prior_text_encoder: CLIPTextModelWithProjection, prior_tokenizer: CLIPTokenizer, prior_scheduler: UnCLIPScheduler, prior_image_processor: CLIPImageProcessorPil)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_combined.py#L107)

**Parameters:**

scheduler (`DDIMScheduler` | `DDPMScheduler`) : A scheduler to be used in combination with `unet` to generate image latents.

unet ([UNet2DConditionModel](/docs/diffusers/v0.40.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the image embedding.

movq ([VQModel](/docs/diffusers/v0.40.0/en/api/models/vq#diffusers.VQModel)) : MoVQ Decoder to generate the image from the latents.

prior_prior ([PriorTransformer](/docs/diffusers/v0.40.0/en/api/models/prior_transformer#diffusers.PriorTransformer)) : The canonical unCLIP prior to approximate the image embedding from the text embedding.

prior_image_encoder (`CLIPVisionModelWithProjection`) : Frozen image-encoder.

prior_text_encoder (`CLIPTextModelWithProjection`) : Frozen text-encoder.

prior_tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

prior_scheduler (`UnCLIPScheduler`) : A scheduler to be used in combination with `prior` to generate image embedding.

prior_image_processor (`CLIPImageProcessor`) : A image_processor to be used to preprocess image from clip.

Combined Pipeline for text-to-image generation using Kandinsky

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.KandinskyV22CombinedPipeline.__call__]]

```python
__call__(prompt: str | list[str], negative_prompt: str | list[str] | None = None, num_inference_steps: int = 100, guidance_scale: float = 4.0, num_images_per_prompt: int = 1, height: int = 512, width: int = 512, prior_guidance_scale: float = 4.0, prior_num_inference_steps: int = 25, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', callback: typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None, callback_steps: int = 1, return_dict: bool = True, prior_callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, prior_callback_on_step_end_tensor_inputs: list = ['latents'], callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_combined.py#L202)

**Parameters:**

prompt (`str` or `list[str]`) : The prompt or prompts to guide the image generation.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

height (`int`, *optional*, defaults to 512) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to 512) : The width in pixels of the generated image.

prior_guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

prior_num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between: `"pil"` (`PIL.Image.Image`), `"np"` (`np.array`) or `"pt"` (`torch.Tensor`).

callback (`Callable`, *optional*) : A function that calls every `callback_steps` steps during inference. The function is called with the following arguments: `callback(step: int, timestep: int, latents: torch.Tensor)`.

callback_steps (`int`, *optional*, defaults to 1) : The frequency at which the `callback` function is called. If not specified, the callback is called at every step.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.

prior_callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference of the prior pipeline. The function is called with the following arguments: `prior_callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`.

prior_callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `prior_callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your prior pipeline class.

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference of the decoder pipeline. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

Function invoked when calling the pipeline for generation.

Examples:
```py
from diffusers import AutoPipelineForText2Image
import torch

pipe = AutoPipelineForText2Image.from_pretrained(
    "kandinsky-community/kandinsky-2-2-decoder", torch_dtype=torch.float16
)
pipe.enable_model_cpu_offload()

prompt = "A lion in galaxies, spirals, nebulae, stars, smoke, iridescent, intricate detail, octane render, 8k"

image = pipe(prompt=prompt, num_inference_steps=25).images[0]
```

#### enable_sequential_cpu_offload[[diffusers.KandinskyV22CombinedPipeline.enable_sequential_cpu_offload]]

```python
enable_sequential_cpu_offload(gpu_id: int | None = None, device: typing.Union[torch.device, str] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_combined.py#L182)

Offloads all models to CPU using accelerate, significantly reducing memory usage. When called, unet,
text_encoder, vae and safety checker have their state dicts saved to CPU and then are moved to a
`torch.device('meta') and loaded to GPU only when their specific submodule has its `forward` method called.
Note that offloading happens on a submodule basis. Memory savings are higher than with
`enable_model_cpu_offload`, but performance is lower.

## KandinskyV22ControlnetPipeline[[diffusers.KandinskyV22ControlnetPipeline]]

#### diffusers.KandinskyV22ControlnetPipeline[[diffusers.KandinskyV22ControlnetPipeline]]

```python
diffusers.KandinskyV22ControlnetPipeline(unet: UNet2DConditionModel, scheduler: DDPMScheduler, movq: VQModel)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_controlnet.py#L115)

**Parameters:**

scheduler ([DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler)) : A scheduler to be used in combination with `unet` to generate image latents.

unet ([UNet2DConditionModel](/docs/diffusers/v0.40.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the image embedding.

movq ([VQModel](/docs/diffusers/v0.40.0/en/api/models/vq#diffusers.VQModel)) : MoVQ Decoder to generate the image from the latents.

Pipeline for text-to-image generation using Kandinsky

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.KandinskyV22ControlnetPipeline.__call__]]

```python
__call__(image_embeds: typing.Union[torch.Tensor, list[torch.Tensor]], negative_image_embeds: typing.Union[torch.Tensor, list[torch.Tensor]], hint: Tensor, height: int = 512, width: int = 512, num_inference_steps: int = 100, guidance_scale: float = 4.0, num_images_per_prompt: int = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', callback: typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None, callback_steps: int = 1, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_controlnet.py#L160)

**Parameters:**

hint (`torch.Tensor`) : The controlnet condition.

image_embeds (`torch.Tensor` or `list[torch.Tensor]`) : The clip image embeddings for text prompt, that will be used to condition the image generation.

negative_image_embeds (`torch.Tensor` or `list[torch.Tensor]`) : The clip image embeddings for negative text prompt, will be used to condition the image generation.

height (`int`, *optional*, defaults to 512) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to 512) : The width in pixels of the generated image.

num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between: `"pil"` (`PIL.Image.Image`), `"np"` (`np.array`) or `"pt"` (`torch.Tensor`).

callback (`Callable`, *optional*) : A function that calls every `callback_steps` steps during inference. The function is called with the following arguments: `callback(step: int, timestep: int, latents: torch.Tensor)`.

callback_steps (`int`, *optional*, defaults to 1) : The frequency at which the `callback` function is called. If not specified, the callback is called at every step.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

Function invoked when calling the pipeline for generation.

Examples:

## KandinskyV22PriorEmb2EmbPipeline[[diffusers.KandinskyV22PriorEmb2EmbPipeline]]

#### diffusers.KandinskyV22PriorEmb2EmbPipeline[[diffusers.KandinskyV22PriorEmb2EmbPipeline]]

```python
diffusers.KandinskyV22PriorEmb2EmbPipeline(prior: PriorTransformer, image_encoder: CLIPVisionModelWithProjection, text_encoder: CLIPTextModelWithProjection, tokenizer: CLIPTokenizer, scheduler: UnCLIPScheduler, image_processor: CLIPImageProcessorPil)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_prior_emb2emb.py#L105)

**Parameters:**

prior ([PriorTransformer](/docs/diffusers/v0.40.0/en/api/models/prior_transformer#diffusers.PriorTransformer)) : The canonical unCLIP prior to approximate the image embedding from the text embedding.

image_encoder (`CLIPVisionModelWithProjection`) : Frozen image-encoder.

text_encoder (`CLIPTextModelWithProjection`) : Frozen text-encoder.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

scheduler (`UnCLIPScheduler`) : A scheduler to be used in combination with `prior` to generate image embedding.

Pipeline for generating image prior for Kandinsky

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.KandinskyV22PriorEmb2EmbPipeline.__call__]]

```python
__call__(prompt: str | list[str], image: typing.Union[torch.Tensor, list[torch.Tensor], PIL.Image.Image, list[PIL.Image.Image]], strength: float = 0.3, negative_prompt: str | list[str] | None = None, num_images_per_prompt: int = 1, num_inference_steps: int = 25, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, guidance_scale: float = 4.0, output_type: str | None = 'pt', return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_prior_emb2emb.py#L399)

**Parameters:**

prompt (`str` or `list[str]`) : The prompt or prompts to guide the image generation.

image (`torch.Tensor`, `PIL.Image.Image`, `list[torch.Tensor]` or `list[PIL.Image.Image]`) : `Image`, or tensor representing an image batch, that will be used as the starting point for the image embedding. Can also accept image latents as `image`, if passing latents directly, it will not be encoded again.

strength (`float`, *optional*, defaults to 0.8) : Conceptually, indicates how much to transform the reference `image`. Must be between 0 and 1. `image` will be used as a starting point, adding more noise to it the larger the `strength`. The number of denoising steps depends on the amount of noise initially added.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

output_type (`str`, *optional*, defaults to `"pt"`) : The output format of the generate image. Choose between: `"np"` (`np.array`) or `"pt"` (`torch.Tensor`).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.

**Returns:** `KandinskyPriorPipelineOutput` or `tuple`

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> from diffusers import KandinskyV22Pipeline, KandinskyV22PriorEmb2EmbPipeline
>>> import torch

>>> pipe_prior = KandinskyPriorPipeline.from_pretrained(
...     "kandinsky-community/kandinsky-2-2-prior", torch_dtype=torch.float16
... )
>>> pipe_prior.to("cuda")

>>> prompt = "red cat, 4k photo"
>>> img = load_image(
...     "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main"
...     "/kandinsky/cat.png"
... )
>>> image_emb, nagative_image_emb = pipe_prior(prompt, image=img, strength=0.2).to_tuple()

>>> pipe = KandinskyPipeline.from_pretrained(
...     "kandinsky-community/kandinsky-2-2-decoder, torch_dtype=torch.float16"
... )
>>> pipe.to("cuda")

>>> image = pipe(
...     image_embeds=image_emb,
...     negative_image_embeds=negative_image_emb,
...     height=768,
...     width=768,
...     num_inference_steps=100,
... ).images

>>> image[0].save("cat.png")
```

#### interpolate[[diffusers.KandinskyV22PriorEmb2EmbPipeline.interpolate]]

```python
interpolate(images_and_prompts: list, weights: list, num_images_per_prompt: int = 1, num_inference_steps: int = 25, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, negative_prior_prompt: str | None = None, negative_prompt: str = '', guidance_scale: float = 4.0, device = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_prior_emb2emb.py#L158)

**Parameters:**

images_and_prompts (`list[str | PIL.Image.Image | torch.Tensor]`) : list of prompts and images to guide the image generation.

weights : (`list[float]`): list of weights for each condition in `images_and_prompts`

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

negative_prior_prompt (`str`, *optional*) : The prompt not to guide the prior diffusion process. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

negative_prompt (`str` or `list[str]`, *optional*) : The prompt not to guide the image generation. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

**Returns:** `KandinskyPriorPipelineOutput` or `tuple`

Function invoked when using the prior pipeline for interpolation.

Examples:
```py
>>> from diffusers import KandinskyV22PriorEmb2EmbPipeline, KandinskyV22Pipeline
>>> from diffusers.utils import load_image
>>> import PIL

>>> import torch
>>> from torchvision import transforms

>>> pipe_prior = KandinskyV22PriorPipeline.from_pretrained(
...     "kandinsky-community/kandinsky-2-2-prior", torch_dtype=torch.float16
... )
>>> pipe_prior.to("cuda")

>>> img1 = load_image(
...     "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main"
...     "/kandinsky/cat.png"
... )

>>> img2 = load_image(
...     "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main"
...     "/kandinsky/starry_night.jpeg"
... )

>>> images_texts = ["a cat", img1, img2]
>>> weights = [0.3, 0.3, 0.4]
>>> image_emb, zero_image_emb = pipe_prior.interpolate(images_texts, weights)

>>> pipe = KandinskyV22Pipeline.from_pretrained(
...     "kandinsky-community/kandinsky-2-2-decoder", torch_dtype=torch.float16
... )
>>> pipe.to("cuda")

>>> image = pipe(
...     image_embeds=image_emb,
...     negative_image_embeds=zero_image_emb,
...     height=768,
...     width=768,
...     num_inference_steps=150,
... ).images[0]

>>> image.save("starry_cat.png")
```

## KandinskyV22Img2ImgPipeline[[diffusers.KandinskyV22Img2ImgPipeline]]

#### diffusers.KandinskyV22Img2ImgPipeline[[diffusers.KandinskyV22Img2ImgPipeline]]

```python
diffusers.KandinskyV22Img2ImgPipeline(unet: UNet2DConditionModel, scheduler: DDPMScheduler, movq: VQModel)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_img2img.py#L78)

**Parameters:**

scheduler ([DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler)) : A scheduler to be used in combination with `unet` to generate image latents.

unet ([UNet2DConditionModel](/docs/diffusers/v0.40.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the image embedding.

movq ([VQModel](/docs/diffusers/v0.40.0/en/api/models/vq#diffusers.VQModel)) : MoVQ Decoder to generate the image from the latents.

Pipeline for image-to-image generation using Kandinsky

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.KandinskyV22Img2ImgPipeline.__call__]]

```python
__call__(image_embeds: typing.Union[torch.Tensor, list[torch.Tensor]], image: typing.Union[torch.Tensor, list[torch.Tensor], PIL.Image.Image, list[PIL.Image.Image]], negative_image_embeds: typing.Union[torch.Tensor, list[torch.Tensor]], height: int = 512, width: int = 512, num_inference_steps: int = 100, guidance_scale: float = 4.0, strength: float = 0.3, num_images_per_prompt: int = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, output_type: str | None = 'pil', return_dict: bool = True, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_img2img.py#L183)

**Parameters:**

image_embeds (`torch.Tensor` or `list[torch.Tensor]`) : The clip image embeddings for text prompt, that will be used to condition the image generation.

image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) : `Image`, or tensor representing an image batch, that will be used as the starting point for the process. Can also accept image latents as `image`, if passing latents directly, it will not be encoded again.

strength (`float`, *optional*, defaults to 0.8) : Conceptually, indicates how much to transform the reference `image`. Must be between 0 and 1. `image` will be used as a starting point, adding more noise to it the larger the `strength`. The number of denoising steps depends on the amount of noise initially added. When `strength` is 1, added noise will be maximum and the denoising process will run for the full number of iterations specified in `num_inference_steps`. A value of 1, therefore, essentially ignores `image`.

negative_image_embeds (`torch.Tensor` or `list[torch.Tensor]`) : The clip image embeddings for negative text prompt, will be used to condition the image generation.

height (`int`, *optional*, defaults to 512) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to 512) : The width in pixels of the generated image.

num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between: `"pil"` (`PIL.Image.Image`), `"np"` (`np.array`) or `"pt"` (`torch.Tensor`).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

Function invoked when calling the pipeline for generation.

Examples:

## KandinskyV22Img2ImgCombinedPipeline[[diffusers.KandinskyV22Img2ImgCombinedPipeline]]

#### diffusers.KandinskyV22Img2ImgCombinedPipeline[[diffusers.KandinskyV22Img2ImgCombinedPipeline]]

```python
diffusers.KandinskyV22Img2ImgCombinedPipeline(unet: UNet2DConditionModel, scheduler: DDPMScheduler, movq: VQModel, prior_prior: PriorTransformer, prior_image_encoder: CLIPVisionModelWithProjection, prior_text_encoder: CLIPTextModelWithProjection, prior_tokenizer: CLIPTokenizer, prior_scheduler: UnCLIPScheduler, prior_image_processor: CLIPImageProcessorPil)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_combined.py#L341)

**Parameters:**

scheduler (`DDIMScheduler` | `DDPMScheduler`) : A scheduler to be used in combination with `unet` to generate image latents.

unet ([UNet2DConditionModel](/docs/diffusers/v0.40.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the image embedding.

movq ([VQModel](/docs/diffusers/v0.40.0/en/api/models/vq#diffusers.VQModel)) : MoVQ Decoder to generate the image from the latents.

prior_prior ([PriorTransformer](/docs/diffusers/v0.40.0/en/api/models/prior_transformer#diffusers.PriorTransformer)) : The canonical unCLIP prior to approximate the image embedding from the text embedding.

prior_image_encoder (`CLIPVisionModelWithProjection`) : Frozen image-encoder.

prior_text_encoder (`CLIPTextModelWithProjection`) : Frozen text-encoder.

prior_tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

prior_scheduler (`UnCLIPScheduler`) : A scheduler to be used in combination with `prior` to generate image embedding.

prior_image_processor (`CLIPImageProcessor`) : A image_processor to be used to preprocess image from clip.

Combined Pipeline for image-to-image generation using Kandinsky

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.KandinskyV22Img2ImgCombinedPipeline.__call__]]

```python
__call__(prompt: str | list[str], image: typing.Union[torch.Tensor, list[torch.Tensor], PIL.Image.Image, list[PIL.Image.Image]], negative_prompt: str | list[str] | None = None, num_inference_steps: int = 100, guidance_scale: float = 4.0, strength: float = 0.3, num_images_per_prompt: int = 1, height: int = 512, width: int = 512, prior_guidance_scale: float = 4.0, prior_num_inference_steps: int = 25, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', callback: typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None, callback_steps: int = 1, return_dict: bool = True, prior_callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, prior_callback_on_step_end_tensor_inputs: list = ['latents'], callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_combined.py#L446)

**Parameters:**

prompt (`str` or `list[str]`) : The prompt or prompts to guide the image generation.

image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) : `Image`, or tensor representing an image batch, that will be used as the starting point for the process. Can also accept image latents as `image`, if passing latents directly, it will not be encoded again.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

strength (`float`, *optional*, defaults to 0.3) : Conceptually, indicates how much to transform the reference `image`. Must be between 0 and 1. `image` will be used as a starting point, adding more noise to it the larger the `strength`. The number of denoising steps depends on the amount of noise initially added. When `strength` is 1, added noise will be maximum and the denoising process will run for the full number of iterations specified in `num_inference_steps`. A value of 1, therefore, essentially ignores `image`.

num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

height (`int`, *optional*, defaults to 512) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to 512) : The width in pixels of the generated image.

prior_guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

prior_num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between: `"pil"` (`PIL.Image.Image`), `"np"` (`np.array`) or `"pt"` (`torch.Tensor`).

callback (`Callable`, *optional*) : A function that calls every `callback_steps` steps during inference. The function is called with the following arguments: `callback(step: int, timestep: int, latents: torch.Tensor)`.

callback_steps (`int`, *optional*, defaults to 1) : The frequency at which the `callback` function is called. If not specified, the callback is called at every step.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.

prior_callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference of the prior pipeline. The function is called with the following arguments: `prior_callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`.

prior_callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `prior_callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your prior pipeline class.

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference of the decoder pipeline. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

Function invoked when calling the pipeline for generation.

Examples:
```py
from diffusers import AutoPipelineForImage2Image
import torch
import requests
from io import BytesIO
from PIL import Image
import os

pipe = AutoPipelineForImage2Image.from_pretrained(
    "kandinsky-community/kandinsky-2-2-decoder", torch_dtype=torch.float16
)
pipe.enable_model_cpu_offload()

prompt = "A fantasy landscape, Cinematic lighting"
negative_prompt = "low quality, bad quality"

url = "https://raw.githubusercontent.com/CompVis/stable-diffusion/main/assets/stable-samples/img2img/sketch-mountains-input.jpg"

response = requests.get(url)
image = Image.open(BytesIO(response.content)).convert("RGB")
image.thumbnail((768, 768))

image = pipe(prompt=prompt, image=original_image, num_inference_steps=25).images[0]
```

#### enable_model_cpu_offload[[diffusers.KandinskyV22Img2ImgCombinedPipeline.enable_model_cpu_offload]]

```python
enable_model_cpu_offload(gpu_id: int | None = None, device: typing.Union[torch.device, str] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_combined.py#L416)

Offloads all models to CPU using accelerate, reducing memory usage with a low impact on performance. Compared
to `enable_sequential_cpu_offload`, this method moves one whole model at a time to the GPU when its `forward`
method is called, and the model remains in GPU until the next model runs. Memory savings are lower than with
`enable_sequential_cpu_offload`, but performance is much better due to the iterative execution of the `unet`.

#### enable_sequential_cpu_offload[[diffusers.KandinskyV22Img2ImgCombinedPipeline.enable_sequential_cpu_offload]]

```python
enable_sequential_cpu_offload(gpu_id: int | None = None, device: typing.Union[torch.device, str] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_combined.py#L426)

Offloads all models to CPU using accelerate, significantly reducing memory usage. When called, unet,
text_encoder, vae and safety checker have their state dicts saved to CPU and then are moved to a
`torch.device('meta') and loaded to GPU only when their specific submodule has its `forward` method called.
Note that offloading happens on a submodule basis. Memory savings are higher than with
`enable_model_cpu_offload`, but performance is lower.

## KandinskyV22ControlnetImg2ImgPipeline[[diffusers.KandinskyV22ControlnetImg2ImgPipeline]]

#### diffusers.KandinskyV22ControlnetImg2ImgPipeline[[diffusers.KandinskyV22ControlnetImg2ImgPipeline]]

```python
diffusers.KandinskyV22ControlnetImg2ImgPipeline(unet: UNet2DConditionModel, scheduler: DDPMScheduler, movq: VQModel)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_controlnet_img2img.py#L107)

**Parameters:**

scheduler ([DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler)) : A scheduler to be used in combination with `unet` to generate image latents.

unet ([UNet2DConditionModel](/docs/diffusers/v0.40.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the image embedding.

movq ([VQModel](/docs/diffusers/v0.40.0/en/api/models/vq#diffusers.VQModel)) : MoVQ Decoder to generate the image from the latents.

Pipeline for image-to-image generation using Kandinsky

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.KandinskyV22ControlnetImg2ImgPipeline.__call__]]

```python
__call__(image_embeds: typing.Union[torch.Tensor, list[torch.Tensor]], image: typing.Union[torch.Tensor, list[torch.Tensor], PIL.Image.Image, list[PIL.Image.Image]], negative_image_embeds: typing.Union[torch.Tensor, list[torch.Tensor]], hint: Tensor, height: int = 512, width: int = 512, num_inference_steps: int = 100, guidance_scale: float = 4.0, strength: float = 0.3, num_images_per_prompt: int = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, output_type: str | None = 'pil', callback: typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None, callback_steps: int = 1, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_controlnet_img2img.py#L200)

**Parameters:**

image_embeds (`torch.Tensor` or `list[torch.Tensor]`) : The clip image embeddings for text prompt, that will be used to condition the image generation.

image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) : `Image`, or tensor representing an image batch, that will be used as the starting point for the process. Can also accept image latents as `image`, if passing latents directly, it will not be encoded again.

strength (`float`, *optional*, defaults to 0.8) : Conceptually, indicates how much to transform the reference `image`. Must be between 0 and 1. `image` will be used as a starting point, adding more noise to it the larger the `strength`. The number of denoising steps depends on the amount of noise initially added. When `strength` is 1, added noise will be maximum and the denoising process will run for the full number of iterations specified in `num_inference_steps`. A value of 1, therefore, essentially ignores `image`.

hint (`torch.Tensor`) : The controlnet condition.

negative_image_embeds (`torch.Tensor` or `list[torch.Tensor]`) : The clip image embeddings for negative text prompt, will be used to condition the image generation.

height (`int`, *optional*, defaults to 512) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to 512) : The width in pixels of the generated image.

num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between: `"pil"` (`PIL.Image.Image`), `"np"` (`np.array`) or `"pt"` (`torch.Tensor`).

callback (`Callable`, *optional*) : A function that calls every `callback_steps` steps during inference. The function is called with the following arguments: `callback(step: int, timestep: int, latents: torch.Tensor)`.

callback_steps (`int`, *optional*, defaults to 1) : The frequency at which the `callback` function is called. If not specified, the callback is called at every step.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

Function invoked when calling the pipeline for generation.

Examples:

## KandinskyV22InpaintPipeline[[diffusers.KandinskyV22InpaintPipeline]]

#### diffusers.KandinskyV22InpaintPipeline[[diffusers.KandinskyV22InpaintPipeline]]

```python
diffusers.KandinskyV22InpaintPipeline(unet: UNet2DConditionModel, scheduler: DDPMScheduler, movq: VQModel)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_inpainting.py#L243)

**Parameters:**

scheduler ([DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler)) : A scheduler to be used in combination with `unet` to generate image latents.

unet ([UNet2DConditionModel](/docs/diffusers/v0.40.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the image embedding.

movq ([VQModel](/docs/diffusers/v0.40.0/en/api/models/vq#diffusers.VQModel)) : MoVQ Decoder to generate the image from the latents.

Pipeline for text-guided image inpainting using Kandinsky2.1

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.KandinskyV22InpaintPipeline.__call__]]

```python
__call__(image_embeds: typing.Union[torch.Tensor, list[torch.Tensor]], image: typing.Union[torch.Tensor, PIL.Image.Image], mask_image: typing.Union[PIL.Image.Image, torch.Tensor, numpy.ndarray], negative_image_embeds: typing.Union[torch.Tensor, list[torch.Tensor]], height: int = 512, width: int = 512, num_inference_steps: int = 100, guidance_scale: float = 4.0, num_images_per_prompt: int = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_inpainting.py#L302)

**Parameters:**

image_embeds (`torch.Tensor` or `list[torch.Tensor]`) : The clip image embeddings for text prompt, that will be used to condition the image generation.

image (`PIL.Image.Image`) : `Image`, or tensor representing an image batch which will be inpainted, *i.e.* parts of the image will be masked out with `mask_image` and repainted according to `prompt`.

mask_image (`np.array`) : Tensor representing an image batch, to mask `image`. White pixels in the mask will be repainted, while black pixels will be preserved. If `mask_image` is a PIL image, it will be converted to a single channel (luminance) before use. If it's a tensor, it should contain one color channel (L) instead of 3, so the expected shape would be `(B, H, W, 1)`.

negative_image_embeds (`torch.Tensor` or `list[torch.Tensor]`) : The clip image embeddings for negative text prompt, will be used to condition the image generation.

height (`int`, *optional*, defaults to 512) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to 512) : The width in pixels of the generated image.

num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between: `"pil"` (`PIL.Image.Image`), `"np"` (`np.array`) or `"pt"` (`torch.Tensor`).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

Function invoked when calling the pipeline for generation.

Examples:

## KandinskyV22InpaintCombinedPipeline[[diffusers.KandinskyV22InpaintCombinedPipeline]]

#### diffusers.KandinskyV22InpaintCombinedPipeline[[diffusers.KandinskyV22InpaintCombinedPipeline]]

```python
diffusers.KandinskyV22InpaintCombinedPipeline(unet: UNet2DConditionModel, scheduler: DDPMScheduler, movq: VQModel, prior_prior: PriorTransformer, prior_image_encoder: CLIPVisionModelWithProjection, prior_text_encoder: CLIPTextModelWithProjection, prior_tokenizer: CLIPTokenizer, prior_scheduler: UnCLIPScheduler, prior_image_processor: CLIPImageProcessorPil)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_combined.py#L607)

**Parameters:**

scheduler (`DDIMScheduler` | `DDPMScheduler`) : A scheduler to be used in combination with `unet` to generate image latents.

unet ([UNet2DConditionModel](/docs/diffusers/v0.40.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the image embedding.

movq ([VQModel](/docs/diffusers/v0.40.0/en/api/models/vq#diffusers.VQModel)) : MoVQ Decoder to generate the image from the latents.

prior_prior ([PriorTransformer](/docs/diffusers/v0.40.0/en/api/models/prior_transformer#diffusers.PriorTransformer)) : The canonical unCLIP prior to approximate the image embedding from the text embedding.

prior_image_encoder (`CLIPVisionModelWithProjection`) : Frozen image-encoder.

prior_text_encoder (`CLIPTextModelWithProjection`) : Frozen text-encoder.

prior_tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

prior_scheduler (`UnCLIPScheduler`) : A scheduler to be used in combination with `prior` to generate image embedding.

prior_image_processor (`CLIPImageProcessor`) : A image_processor to be used to preprocess image from clip.

Combined Pipeline for inpainting generation using Kandinsky

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.KandinskyV22InpaintCombinedPipeline.__call__]]

```python
__call__(prompt: str | list[str], image: typing.Union[torch.Tensor, list[torch.Tensor], PIL.Image.Image, list[PIL.Image.Image]], mask_image: typing.Union[torch.Tensor, list[torch.Tensor], PIL.Image.Image, list[PIL.Image.Image]], negative_prompt: str | list[str] | None = None, num_inference_steps: int = 100, guidance_scale: float = 4.0, num_images_per_prompt: int = 1, height: int = 512, width: int = 512, prior_guidance_scale: float = 4.0, prior_num_inference_steps: int = 25, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, prior_callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, prior_callback_on_step_end_tensor_inputs: list = ['latents'], callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_combined.py#L702)

**Parameters:**

prompt (`str` or `list[str]`) : The prompt or prompts to guide the image generation.

image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) : `Image`, or tensor representing an image batch, that will be used as the starting point for the process. Can also accept image latents as `image`, if passing latents directly, it will not be encoded again.

mask_image (`np.array`) : Tensor representing an image batch, to mask `image`. White pixels in the mask will be repainted, while black pixels will be preserved. If `mask_image` is a PIL image, it will be converted to a single channel (luminance) before use. If it's a tensor, it should contain one color channel (L) instead of 3, so the expected shape would be `(B, H, W, 1)`.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

height (`int`, *optional*, defaults to 512) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to 512) : The width in pixels of the generated image.

prior_guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

prior_num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between: `"pil"` (`PIL.Image.Image`), `"np"` (`np.array`) or `"pt"` (`torch.Tensor`).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.

prior_callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `prior_callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`.

prior_callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `prior_callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

Function invoked when calling the pipeline for generation.

Examples:
```py
from diffusers import AutoPipelineForInpainting
from diffusers.utils import load_image
import torch
import numpy as np

pipe = AutoPipelineForInpainting.from_pretrained(
    "kandinsky-community/kandinsky-2-2-decoder-inpaint", torch_dtype=torch.float16
)
pipe.enable_model_cpu_offload()

prompt = "A fantasy landscape, Cinematic lighting"
negative_prompt = "low quality, bad quality"

original_image = load_image(
    "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main" "/kandinsky/cat.png"
)

mask = np.zeros((768, 768), dtype=np.float32)
# Let's mask out an area above the cat's head
mask[:250, 250:-250] = 1

image = pipe(prompt=prompt, image=original_image, mask_image=mask, num_inference_steps=25).images[0]
```

#### enable_sequential_cpu_offload[[diffusers.KandinskyV22InpaintCombinedPipeline.enable_sequential_cpu_offload]]

```python
enable_sequential_cpu_offload(gpu_id: int | None = None, device: typing.Union[torch.device, str] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky2_2/pipeline_kandinsky2_2_combined.py#L682)

Offloads all models to CPU using accelerate, significantly reducing memory usage. When called, unet,
text_encoder, vae and safety checker have their state dicts saved to CPU and then are moved to a
`torch.device('meta') and loaded to GPU only when their specific submodule has its `forward` method called.
Note that offloading happens on a submodule basis. Memory savings are higher than with
`enable_model_cpu_offload`, but performance is lower.
