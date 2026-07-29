# Stable Cascade

This model is built upon the [Würstchen](https://openreview.net/forum?id=gU58d5QeGv) architecture and its main
difference to other models like Stable Diffusion is that it is working at a much smaller latent space. Why is this
important? The smaller the latent space, the **faster** you can run inference and the **cheaper** the training becomes.
How small is the latent space? Stable Diffusion uses a compression factor of 8, resulting in a 1024x1024 image being
encoded to 128x128. Stable Cascade achieves a compression factor of 42, meaning that it is possible to encode a
1024x1024 image to 24x24, while maintaining crisp reconstructions. The text-conditional model is then trained in the
highly compressed latent space. Previous versions of this architecture, achieved a 16x cost reduction over Stable
Diffusion 1.5.

Therefore, this kind of model is well suited for usages where efficiency is important. Furthermore, all known extensions
like finetuning, LoRA, ControlNet, IP-Adapter, LCM etc. are possible with this method as well.

The original codebase can be found at [Stability-AI/StableCascade](https://github.com/Stability-AI/StableCascade).

## Model Overview
Stable Cascade consists of three models: Stage A, Stage B and Stage C, representing a cascade to generate images,
hence the name "Stable Cascade".

Stage A & B are used to compress images, similar to what the job of the VAE is in Stable Diffusion.
However, with this setup, a much higher compression of images can be achieved. While the Stable Diffusion models use a
spatial compression factor of 8, encoding an image with resolution of 1024 x 1024 to 128 x 128, Stable Cascade achieves
a compression factor of 42. This encodes a 1024 x 1024 image to 24 x 24, while being able to accurately decode the
image. This comes with the great benefit of cheaper training and inference. Furthermore, Stage C is responsible
for generating the small 24 x 24 latents given a text prompt.

The Stage C model operates on the small 24 x 24 latents and denoises the latents conditioned on text prompts. The model is also the largest component in the Cascade pipeline and is meant to be used with the `StableCascadePriorPipeline`

The Stage B and Stage A models are used with the `StableCascadeDecoderPipeline` and are responsible for generating the final image given the small 24 x 24 latents.

> [!WARNING]
> There are some restrictions on data types that can be used with the Stable Cascade models. The official checkpoints for the  `StableCascadePriorPipeline` do not support the `torch.float16` data type. Please use `torch.bfloat16` instead.
>
> In order to use the `torch.bfloat16` data type with the `StableCascadeDecoderPipeline` you need to have PyTorch 2.2.0 or higher installed. This also means that using the `StableCascadeCombinedPipeline` with `torch.bfloat16` requires PyTorch 2.2.0 or higher, since it calls the `StableCascadeDecoderPipeline` internally.
>
> If it is not possible to install PyTorch 2.2.0 or higher in your environment, the `StableCascadeDecoderPipeline` can be used on its own with the `torch.float16` data type. You can download the full precision or `bf16` variant weights for the pipeline and cast the weights to `torch.float16`.

## Usage example

```python
import torch
from diffusers import StableCascadeDecoderPipeline, StableCascadePriorPipeline

prompt = "an image of a shiba inu, donning a spacesuit and helmet"
negative_prompt = ""

prior = StableCascadePriorPipeline.from_pretrained("stabilityai/stable-cascade-prior", variant="bf16", torch_dtype=torch.bfloat16)
decoder = StableCascadeDecoderPipeline.from_pretrained("stabilityai/stable-cascade", variant="bf16", torch_dtype=torch.float16)

prior.enable_model_cpu_offload()
prior_output = prior(
    prompt=prompt,
    height=1024,
    width=1024,
    negative_prompt=negative_prompt,
    guidance_scale=4.0,
    num_images_per_prompt=1,
    num_inference_steps=20
)

decoder.enable_model_cpu_offload()
decoder_output = decoder(
    image_embeddings=prior_output.image_embeddings.to(torch.float16),
    prompt=prompt,
    negative_prompt=negative_prompt,
    guidance_scale=0.0,
    output_type="pil",
    num_inference_steps=10
).images[0]
decoder_output.save("cascade.png")
```

## Using the Lite Versions of the Stage B and Stage C models

```python
import torch
from diffusers import (
    StableCascadeDecoderPipeline,
    StableCascadePriorPipeline,
    StableCascadeUNet,
)

prompt = "an image of a shiba inu, donning a spacesuit and helmet"
negative_prompt = ""

prior_unet = StableCascadeUNet.from_pretrained("stabilityai/stable-cascade-prior", subfolder="prior_lite")
decoder_unet = StableCascadeUNet.from_pretrained("stabilityai/stable-cascade", subfolder="decoder_lite")

prior = StableCascadePriorPipeline.from_pretrained("stabilityai/stable-cascade-prior", prior=prior_unet)
decoder = StableCascadeDecoderPipeline.from_pretrained("stabilityai/stable-cascade", decoder=decoder_unet)

prior.enable_model_cpu_offload()
prior_output = prior(
    prompt=prompt,
    height=1024,
    width=1024,
    negative_prompt=negative_prompt,
    guidance_scale=4.0,
    num_images_per_prompt=1,
    num_inference_steps=20
)

decoder.enable_model_cpu_offload()
decoder_output = decoder(
    image_embeddings=prior_output.image_embeddings,
    prompt=prompt,
    negative_prompt=negative_prompt,
    guidance_scale=0.0,
    output_type="pil",
    num_inference_steps=10
).images[0]
decoder_output.save("cascade.png")
```

## Loading original checkpoints with `from_single_file`

Loading the original format checkpoints is supported via `from_single_file` method in the StableCascadeUNet.

```python
import torch
from diffusers import (
    StableCascadeDecoderPipeline,
    StableCascadePriorPipeline,
    StableCascadeUNet,
)

prompt = "an image of a shiba inu, donning a spacesuit and helmet"
negative_prompt = ""

prior_unet = StableCascadeUNet.from_single_file(
    "https://huggingface.co/stabilityai/stable-cascade/resolve/main/stage_c_bf16.safetensors",
    torch_dtype=torch.bfloat16
)
decoder_unet = StableCascadeUNet.from_single_file(
    "https://huggingface.co/stabilityai/stable-cascade/blob/main/stage_b_bf16.safetensors",
    torch_dtype=torch.bfloat16
)

prior = StableCascadePriorPipeline.from_pretrained("stabilityai/stable-cascade-prior", prior=prior_unet, torch_dtype=torch.bfloat16)
decoder = StableCascadeDecoderPipeline.from_pretrained("stabilityai/stable-cascade", decoder=decoder_unet, torch_dtype=torch.bfloat16)

prior.enable_model_cpu_offload()
prior_output = prior(
    prompt=prompt,
    height=1024,
    width=1024,
    negative_prompt=negative_prompt,
    guidance_scale=4.0,
    num_images_per_prompt=1,
    num_inference_steps=20
)

decoder.enable_model_cpu_offload()
decoder_output = decoder(
    image_embeddings=prior_output.image_embeddings,
    prompt=prompt,
    negative_prompt=negative_prompt,
    guidance_scale=0.0,
    output_type="pil",
    num_inference_steps=10
).images[0]
decoder_output.save("cascade-single-file.png")
```

## Uses

### Direct Use

The model is intended for research purposes for now. Possible research areas and tasks include

- Research on generative models.
- Safe deployment of models which have the potential to generate harmful content.
- Probing and understanding the limitations and biases of generative models.
- Generation of artworks and use in design and other artistic processes.
- Applications in educational or creative tools.

Excluded uses are described below.

### Out-of-Scope Use

The model was not trained to be factual or true representations of people or events,
and therefore using the model to generate such content is out-of-scope for the abilities of this model.
The model should not be used in any way that violates Stability AI's [Acceptable Use Policy](https://stability.ai/use-policy).

## Limitations and Bias

### Limitations
- Faces and people in general may not be generated properly.
- The autoencoding part of the model is lossy.

## StableCascadeCombinedPipeline[[diffusers.StableCascadeCombinedPipeline]]

#### diffusers.StableCascadeCombinedPipeline[[diffusers.StableCascadeCombinedPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_cascade/pipeline_stable_cascade_combined.py#L45)

Combined Pipeline for text-to-image generation using Stable Cascade.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

__call__diffusers.StableCascadeCombinedPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_cascade/pipeline_stable_cascade_combined.py#L158[{"name": "prompt", "val": ": str | list[str] | None = None"}, {"name": "images", "val": ": torch.Tensor | PIL.Image.Image | list[torch.Tensor] | list[PIL.Image.Image] = None"}, {"name": "height", "val": ": int = 512"}, {"name": "width", "val": ": int = 512"}, {"name": "prior_num_inference_steps", "val": ": int = 60"}, {"name": "prior_guidance_scale", "val": ": float = 4.0"}, {"name": "num_inference_steps", "val": ": int = 12"}, {"name": "decoder_guidance_scale", "val": ": float = 0.0"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds_pooled", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds_pooled", "val": ": torch.Tensor | None = None"}, {"name": "num_images_per_prompt", "val": ": int = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "prior_callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "prior_callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}]- **prompt** (`str` or `list[str]`) --
  The prompt or prompts to guide the image generation for the prior and decoder.
- **images** (`torch.Tensor`, `PIL.Image.Image`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, *optional*) --
  The images to guide the image generation for the prior.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. Ignored when not using guidance (i.e., ignored
  if `guidance_scale` is less than `1`).
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings for the prior. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, text embeddings will be generated from `prompt` input argument.
- **prompt_embeds_pooled** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings for the prior. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, text embeddings will be generated from `prompt` input argument.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings for the prior. Can be used to easily tweak text inputs, *e.g.*
  prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt`
  input argument.
- **negative_prompt_embeds_pooled** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings for the prior. Can be used to easily tweak text inputs, *e.g.*
  prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt`
  input argument.
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **height** (`int`, *optional*, defaults to 512) --
  The height in pixels of the generated image.
- **width** (`int`, *optional*, defaults to 512) --
  The width in pixels of the generated image.
- **prior_guidance_scale** (`float`, *optional*, defaults to 4.0) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `prior_guidance_scale` is defined as `w` of
  equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by
  setting `prior_guidance_scale > 1`. Higher guidance scale encourages to generate images that are
  closely linked to the text `prompt`, usually at the expense of lower image quality.
- **prior_num_inference_steps** (`int | dict[float, int]`, *optional*, defaults to 60) --
  The number of prior denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference. For more specific timestep spacing, you can pass customized
  `prior_timesteps`
- **num_inference_steps** (`int`, *optional*, defaults to 12) --
  The number of decoder denoising steps. More denoising steps usually lead to a higher quality image at
  the expense of slower inference. For more specific timestep spacing, you can pass customized
  `timesteps`
- **decoder_guidance_scale** (`float`, *optional*, defaults to 0.0) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2.
  of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting
  `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to
  the text `prompt`, usually at the expense of lower image quality.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between: `"pil"` (`PIL.Image.Image`), `"np"`
  (`np.array`) or `"pt"` (`torch.Tensor`).
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.
- **prior_callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `prior_callback_on_step_end(self: DiffusionPipeline, step: int, timestep:
  int, callback_kwargs: Dict)`.
- **prior_callback_on_step_end_tensor_inputs** (`list`, *optional*) --
  The list of tensor inputs for the `prior_callback_on_step_end` function. The tensors specified in the
  list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in
  the `._callback_tensor_inputs` attribute of your pipeline class.
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`list`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.0[ImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple` [ImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) if `return_dict` is True,
otherwise a `tuple`. When returning a tuple, the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import StableCascadeCombinedPipeline

>>> pipe = StableCascadeCombinedPipeline.from_pretrained(
...     "stabilityai/stable-cascade", variant="bf16", torch_dtype=torch.bfloat16
... )
>>> pipe.enable_model_cpu_offload()
>>> prompt = "an image of a shiba inu, donning a spacesuit and helmet"
>>> images = pipe(prompt=prompt)
```

**Parameters:**

tokenizer (`CLIPTokenizer`) : The decoder tokenizer to be used for text inputs.

text_encoder (`CLIPTextModelWithProjection`) : The decoder text encoder to be used for text inputs.

decoder (`StableCascadeUNet`) : The decoder model to be used for decoder image generation pipeline.

scheduler (`DDPMWuerstchenScheduler`) : The scheduler to be used for decoder image generation pipeline.

vqgan (`PaellaVQModel`) : The VQGAN model to be used for decoder image generation pipeline.

prior_prior (`StableCascadeUNet`) : The prior model to be used for prior pipeline.

prior_text_encoder (`CLIPTextModelWithProjection`) : The prior text encoder to be used for text inputs.

prior_tokenizer (`CLIPTokenizer`) : The prior tokenizer to be used for text inputs.

prior_scheduler (`DDPMWuerstchenScheduler`) : The scheduler to be used for prior pipeline.

prior_feature_extractor ([CLIPImageProcessor](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPImageProcessor)) : Model that extracts features from generated images to be used as inputs for the `image_encoder`.

prior_image_encoder (`CLIPVisionModelWithProjection`) : Frozen CLIP image-encoder ([clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14)).

**Returns:**

[ImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple` [ImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) if `return_dict` is True,
otherwise a `tuple`. When returning a tuple, the first element is a list with the generated images.
#### enable_model_cpu_offload[[diffusers.StableCascadeCombinedPipeline.enable_model_cpu_offload]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_cascade/pipeline_stable_cascade_combined.py#L130)

Offloads all models to CPU using accelerate, reducing memory usage with a low impact on performance. Compared
to `enable_sequential_cpu_offload`, this method moves one whole model at a time to the GPU when its `forward`
method is called, and the model remains in GPU until the next model runs. Memory savings are lower than with
`enable_sequential_cpu_offload`, but performance is much better due to the iterative execution of the `unet`.
#### enable_sequential_cpu_offload[[diffusers.StableCascadeCombinedPipeline.enable_sequential_cpu_offload]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_cascade/pipeline_stable_cascade_combined.py#L140)

Offloads all models (`unet`, `text_encoder`, `vae`, and `safety checker` state dicts) to CPU using 🤗
Accelerate, significantly reducing memory usage. Models are moved to a `torch.device('meta')` and loaded on a
GPU only when their specific submodule's `forward` method is called. Offloading happens on a submodule basis.
Memory savings are higher than using `enable_model_cpu_offload`, but performance is lower.

## StableCascadePriorPipeline[[diffusers.StableCascadePriorPipeline]]

#### diffusers.StableCascadePriorPipeline[[diffusers.StableCascadePriorPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_cascade/pipeline_stable_cascade_prior.py#L80)

Pipeline for generating image prior for Stable Cascade.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

__call__diffusers.StableCascadePriorPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_cascade/pipeline_stable_cascade_prior.py#L375[{"name": "prompt", "val": ": str | list[str] | None = None"}, {"name": "images", "val": ": torch.Tensor | PIL.Image.Image | list[torch.Tensor] | list[PIL.Image.Image] = None"}, {"name": "height", "val": ": int = 1024"}, {"name": "width", "val": ": int = 1024"}, {"name": "num_inference_steps", "val": ": int = 20"}, {"name": "timesteps", "val": ": list = None"}, {"name": "guidance_scale", "val": ": float = 4.0"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds_pooled", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds_pooled", "val": ": torch.Tensor | None = None"}, {"name": "image_embeds", "val": ": torch.Tensor | None = None"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pt'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}]- **prompt** (`str` or `list[str]`) --
  The prompt or prompts to guide the image generation.
- **images** (`torch.Tensor`, `PIL.Image.Image`, `list[torch.Tensor]` or `list[PIL.Image.Image]`, *optional*) --
  Reference image(s) used to condition the prior generation. When provided, image embeddings are derived
  from the image and combined with the text prompt.
- **height** (`int`, *optional*, defaults to 1024) --
  The height in pixels of the generated image.
- **width** (`int`, *optional*, defaults to 1024) --
  The width in pixels of the generated image.
- **num_inference_steps** (`int`, *optional*, defaults to 60) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **timesteps** (`list[float]`, *optional*) --
  Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument
  in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is
  passed will be used. Must be in descending order.
- **guidance_scale** (`float`, *optional*, defaults to 8.0) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `decoder_guidance_scale` is defined as `w` of
  equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by
  setting `decoder_guidance_scale > 1`. Higher guidance scale encourages to generate images that are
  closely linked to the text `prompt`, usually at the expense of lower image quality.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. Ignored when not using guidance (i.e., ignored
  if `decoder_guidance_scale` is less than `1`).
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **prompt_embeds_pooled** (`torch.Tensor`, *optional*) --
  Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting.
  If not provided, pooled text embeddings will be generated from `prompt` input argument.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
  argument.
- **negative_prompt_embeds_pooled** (`torch.Tensor`, *optional*) --
  Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds_pooled will be generated from `negative_prompt`
  input argument.
- **image_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated image embeddings. Can be used to easily tweak image inputs, *e.g.* prompt weighting. If
  not provided, image embeddings will be generated from `image` input argument if existing.
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between: `"pil"` (`PIL.Image.Image`), `"np"`
  (`np.array`) or `"pt"` (`torch.Tensor`).
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`list`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.0`StableCascadePriorPipelineOutput` or `tuple` `StableCascadePriorPipelineOutput` if `return_dict` is
True, otherwise a `tuple`. When returning a tuple, the first element is a list with the generated image
embeddings.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import StableCascadePriorPipeline

>>> prior_pipe = StableCascadePriorPipeline.from_pretrained(
...     "stabilityai/stable-cascade-prior", torch_dtype=torch.bfloat16
... ).to("cuda")

>>> prompt = "an image of a shiba inu, donning a spacesuit and helmet"
>>> prior_output = pipe(prompt)
```

**Parameters:**

prior (`StableCascadeUNet`) : The Stable Cascade prior to approximate the image embedding from the text and/or image embedding.

text_encoder (`CLIPTextModelWithProjection`) : Frozen text-encoder ([laion/CLIP-ViT-bigG-14-laion2B-39B-b160k](https://huggingface.co/laion/CLIP-ViT-bigG-14-laion2B-39B-b160k)).

feature_extractor ([CLIPImageProcessor](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPImageProcessor)) : Model that extracts features from generated images to be used as inputs for the `image_encoder`.

image_encoder (`CLIPVisionModelWithProjection`) : Frozen CLIP image-encoder ([clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14)).

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

scheduler (`DDPMWuerstchenScheduler`) : A scheduler to be used in combination with `prior` to generate image embedding.

resolution_multiple ('float', *optional*, defaults to 42.67) : Default resolution for multiple images generated.

**Returns:**

`StableCascadePriorPipelineOutput` or `tuple` `StableCascadePriorPipelineOutput` if `return_dict` is
True, otherwise a `tuple`. When returning a tuple, the first element is a list with the generated image
embeddings.

## StableCascadePriorPipelineOutput[[diffusers.pipelines.stable_cascade.pipeline_stable_cascade_prior.StableCascadePriorPipelineOutput]]

#### diffusers.pipelines.stable_cascade.pipeline_stable_cascade_prior.StableCascadePriorPipelineOutput[[diffusers.pipelines.stable_cascade.pipeline_stable_cascade_prior.StableCascadePriorPipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_cascade/pipeline_stable_cascade_prior.py#L60)

Output class for WuerstchenPriorPipeline.

**Parameters:**

image_embeddings (`torch.Tensor` or `np.ndarray`) : Prior image embeddings for text prompt

prompt_embeds (`torch.Tensor`) : Text embeddings for the prompt.

negative_prompt_embeds (`torch.Tensor`) : Text embeddings for the negative prompt.

## StableCascadeDecoderPipeline[[diffusers.StableCascadeDecoderPipeline]]

#### diffusers.StableCascadeDecoderPipeline[[diffusers.StableCascadeDecoderPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_cascade/pipeline_stable_cascade.py#L58)

Pipeline for generating images from the Stable Cascade model.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

__call__diffusers.StableCascadeDecoderPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_cascade/pipeline_stable_cascade.py#L304[{"name": "image_embeddings", "val": ": torch.Tensor | list[torch.Tensor]"}, {"name": "prompt", "val": ": str | list[str] = None"}, {"name": "num_inference_steps", "val": ": int = 10"}, {"name": "guidance_scale", "val": ": float = 0.0"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds_pooled", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds_pooled", "val": ": torch.Tensor | None = None"}, {"name": "num_images_per_prompt", "val": ": int = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}]- **image_embeddings** (`torch.Tensor` or `list[torch.Tensor]`) --
  Image Embeddings either extracted from an image or generated by a Prior Model.
- **prompt** (`str` or `list[str]`) --
  The prompt or prompts to guide the image generation.
- **num_inference_steps** (`int`, *optional*, defaults to 12) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **guidance_scale** (`float`, *optional*, defaults to 0.0) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `decoder_guidance_scale` is defined as `w` of
  equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by
  setting `decoder_guidance_scale > 1`. Higher guidance scale encourages to generate images that are
  closely linked to the text `prompt`, usually at the expense of lower image quality.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. Ignored when not using guidance (i.e., ignored
  if `decoder_guidance_scale` is less than `1`).
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **prompt_embeds_pooled** (`torch.Tensor`, *optional*) --
  Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting.
  If not provided, pooled text embeddings will be generated from `prompt` input argument.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
  argument.
- **negative_prompt_embeds_pooled** (`torch.Tensor`, *optional*) --
  Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds_pooled will be generated from `negative_prompt`
  input argument.
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between: `"pil"` (`PIL.Image.Image`), `"np"`
  (`np.array`) or `"pt"` (`torch.Tensor`).
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`list`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.0[ImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple` [ImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) if `return_dict` is True,
otherwise a `tuple`. When returning a tuple, the first element is a list with the generated image
embeddings.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import StableCascadePriorPipeline, StableCascadeDecoderPipeline

>>> prior_pipe = StableCascadePriorPipeline.from_pretrained(
...     "stabilityai/stable-cascade-prior", torch_dtype=torch.bfloat16
... ).to("cuda")
>>> gen_pipe = StableCascadeDecoderPipeline.from_pretrain(
...     "stabilityai/stable-cascade", torch_dtype=torch.float16
... ).to("cuda")

>>> prompt = "an image of a shiba inu, donning a spacesuit and helmet"
>>> prior_output = pipe(prompt)
>>> images = gen_pipe(prior_output.image_embeddings, prompt=prompt)
```

**Parameters:**

tokenizer (`CLIPTokenizer`) : The CLIP tokenizer.

text_encoder (`CLIPTextModelWithProjection`) : The CLIP text encoder.

decoder (`StableCascadeUNet`) : The Stable Cascade decoder unet.

vqgan (`PaellaVQModel`) : The VQGAN model.

scheduler (`DDPMWuerstchenScheduler`) : A scheduler to be used in combination with `prior` to generate image embedding.

latent_dim_scale (float, `optional`, defaults to 10.67) : Multiplier to determine the VQ latent space size from the image embeddings. If the image embeddings are height=24 and width=24, the VQ latent shape needs to be height=int(24*10.67)=256 and width=int(24*10.67)=256 in order to match the training conditions.

**Returns:**

[ImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple` [ImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) if `return_dict` is True,
otherwise a `tuple`. When returning a tuple, the first element is a list with the generated image
embeddings.
