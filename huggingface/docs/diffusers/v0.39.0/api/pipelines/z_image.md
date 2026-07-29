# Z-Image

  

[Z-Image](https://huggingface.co/papers/2511.22699) is a powerful and highly efficient image generation model with 6B parameters. Currently there's only one model with two more to be released:

|Model|Hugging Face|
|---|---|
|Z-Image-Turbo|https://huggingface.co/Tongyi-MAI/Z-Image-Turbo|

## Z-Image-Turbo

Z-Image-Turbo is a distilled version of Z-Image that matches or exceeds leading competitors with only 8 NFEs (Number of Function Evaluations). It offers sub-second inference latency on enterprise-grade H800 GPUs and fits comfortably within 16G VRAM consumer devices. It excels in photorealistic image generation, bilingual text rendering (English & Chinese), and robust instruction adherence.

## Image-to-image

Use [ZImageImg2ImgPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/z_image#diffusers.ZImageImg2ImgPipeline) to transform an existing image based on a text prompt.

```python
import torch
from diffusers import ZImageImg2ImgPipeline
from diffusers.utils import load_image

pipe = ZImageImg2ImgPipeline.from_pretrained("Tongyi-MAI/Z-Image-Turbo", torch_dtype=torch.bfloat16)
pipe.to("cuda")

url = "https://raw.githubusercontent.com/CompVis/stable-diffusion/main/assets/stable-samples/img2img/sketch-mountains-input.jpg"
init_image = load_image(url).resize((1024, 1024))

prompt = "A fantasy landscape with mountains and a river, detailed, vibrant colors"
image = pipe(
    prompt,
    image=init_image,
    strength=0.6,
    num_inference_steps=8,
    guidance_scale=0.0,
    generator=torch.Generator("cuda").manual_seed(42),
).images[0]
image.save("zimage_img2img.png")
```

## Inpainting

Use [ZImageInpaintPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/z_image#diffusers.ZImageInpaintPipeline) to inpaint specific regions of an image based on a text prompt and mask.

```python
import torch
import numpy as np
from PIL import Image
from diffusers import ZImageInpaintPipeline
from diffusers.utils import load_image

pipe = ZImageInpaintPipeline.from_pretrained("Tongyi-MAI/Z-Image-Turbo", torch_dtype=torch.bfloat16)
pipe.to("cuda")

url = "https://raw.githubusercontent.com/CompVis/stable-diffusion/main/assets/stable-samples/img2img/sketch-mountains-input.jpg"
init_image = load_image(url).resize((1024, 1024))

# Create a mask (white = inpaint, black = preserve)
mask = np.zeros((1024, 1024), dtype=np.uint8)
mask[256:768, 256:768] = 255  # Inpaint center region
mask_image = Image.fromarray(mask)

prompt = "A beautiful lake with mountains in the background"
image = pipe(
    prompt,
    image=init_image,
    mask_image=mask_image,
    strength=1.0,
    num_inference_steps=8,
    guidance_scale=0.0,
    generator=torch.Generator("cuda").manual_seed(42),
).images[0]
image.save("zimage_inpaint.png")
```

## ZImagePipeline[[diffusers.ZImagePipeline]]

#### diffusers.ZImagePipeline[[diffusers.ZImagePipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/z_image/pipeline_z_image.py#L141)

__call__diffusers.ZImagePipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/z_image/pipeline_z_image.py#L297[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "sigmas", "val": ": list[float] | None = None"}, {"name": "guidance_scale", "val": ": float = 5.0"}, {"name": "cfg_normalization", "val": ": bool = False"}, {"name": "cfg_truncation", "val": ": float = 1.0"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.FloatTensor | None = None"}, {"name": "prompt_embeds", "val": ": list[torch.FloatTensor] | None = None"}, {"name": "negative_prompt_embeds", "val": ": list[torch.FloatTensor] | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "joint_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **height** (`int`, *optional*, defaults to 1024) --
  The height in pixels of the generated image.
- **width** (`int`, *optional*, defaults to 1024) --
  The width in pixels of the generated image.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **sigmas** (`list[float]`, *optional*) --
  Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in
  their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed
  will be used.
- **guidance_scale** (`float`, *optional*, defaults to 5.0) --
  Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://arxiv.org/abs/2207.12598).
  `guidance_scale` is defined as `w` of equation 2. of [Imagen
  Paper](https://arxiv.org/pdf/2205.11487.pdf). Guidance scale is enabled by setting `guidance_scale >
  1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`,
  usually at the expense of lower image quality.
- **cfg_normalization** (`bool`, *optional*, defaults to False) --
  Whether to apply configuration normalization.
- **cfg_truncation** (`float`, *optional*, defaults to 1.0) --
  The truncation value for configuration.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is
  less than `1`).
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.FloatTensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`list[torch.FloatTensor]`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **negative_prompt_embeds** (`list[torch.FloatTensor]`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
  argument.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.stable_diffusion.ZImagePipelineOutput` instead of a plain
  tuple.
- **joint_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`List`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.
- **max_sequence_length** (`int`, *optional*, defaults to 512) --
  Maximum sequence length to use with the `prompt`.0`ZImagePipelineOutput` or `tuple``ZImagePipelineOutput` if
`return_dict` is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the
generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import ZImagePipeline

>>> pipe = ZImagePipeline.from_pretrained("Z-a-o/Z-Image-Turbo", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> # Optionally, set the attention backend to flash-attn 2 or 3, default is SDPA in PyTorch.
>>> # (1) Use flash attention 2
>>> # pipe.transformer.set_attention_backend("flash")
>>> # (2) Use flash attention 3
>>> # pipe.transformer.set_attention_backend("_flash_3")

>>> prompt = "一幅为名为“造相「Z-IMAGE-TURBO」”的项目设计的创意海报。画面巧妙地将文字概念视觉化：一辆复古蒸汽小火车化身为巨大的拉链头，正拉开厚厚的冬日积雪，展露出一个生机盎然的春天。"
>>> image = pipe(
...     prompt,
...     height=1024,
...     width=1024,
...     num_inference_steps=8,
...     guidance_scale=0.0,
...     generator=torch.Generator("cuda").manual_seed(42),
... ).images[0]
>>> image.save("zimage.png")
```

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

height (`int`, *optional*, defaults to 1024) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to 1024) : The width in pixels of the generated image.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`list[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

guidance_scale (`float`, *optional*, defaults to 5.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://arxiv.org/abs/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://arxiv.org/pdf/2205.11487.pdf). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

cfg_normalization (`bool`, *optional*, defaults to False) : Whether to apply configuration normalization.

cfg_truncation (`float`, *optional*, defaults to 1.0) : The truncation value for configuration.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.FloatTensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`list[torch.FloatTensor]`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`list[torch.FloatTensor]`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.stable_diffusion.ZImagePipelineOutput` instead of a plain tuple.

joint_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`List`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int`, *optional*, defaults to 512) : Maximum sequence length to use with the `prompt`.

**Returns:**

``ZImagePipelineOutput` or `tuple``

`ZImagePipelineOutput` if
`return_dict` is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the
generated images.

## ZImageImg2ImgPipeline[[diffusers.ZImageImg2ImgPipeline]]

#### diffusers.ZImageImg2ImgPipeline[[diffusers.ZImageImg2ImgPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/z_image/pipeline_z_image_img2img.py#L154)

The ZImage pipeline for image-to-image generation.

__call__diffusers.ZImageImg2ImgPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/z_image/pipeline_z_image_img2img.py#L369[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] = None"}, {"name": "strength", "val": ": float = 0.6"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "sigmas", "val": ": list[float] | None = None"}, {"name": "guidance_scale", "val": ": float = 5.0"}, {"name": "cfg_normalization", "val": ": bool = False"}, {"name": "cfg_truncation", "val": ": float = 1.0"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.FloatTensor | None = None"}, {"name": "prompt_embeds", "val": ": list[torch.FloatTensor] | None = None"}, {"name": "negative_prompt_embeds", "val": ": list[torch.FloatTensor] | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "joint_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **image** (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) --
  `Image`, numpy array or tensor representing an image batch to be used as the starting point. For both
  numpy array and pytorch tensor, the expected value range is between `[0, 1]`. If it's a tensor or a
  list of tensors, the expected shape should be `(B, C, H, W)` or `(C, H, W)`. If it is a numpy array or
  a list of arrays, the expected shape should be `(B, H, W, C)` or `(H, W, C)`.
- **strength** (`float`, *optional*, defaults to 0.6) --
  Indicates extent to transform the reference `image`. Must be between 0 and 1. `image` is used as a
  starting point and more noise is added the higher the `strength`. The number of denoising steps depends
  on the amount of noise initially added. When `strength` is 1, added noise is maximum and the denoising
  process runs for the full number of iterations specified in `num_inference_steps`. A value of 1
  essentially ignores `image`.
- **height** (`int`, *optional*, defaults to 1024) --
  The height in pixels of the generated image. If not provided, uses the input image height.
- **width** (`int`, *optional*, defaults to 1024) --
  The width in pixels of the generated image. If not provided, uses the input image width.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **sigmas** (`list[float]`, *optional*) --
  Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in
  their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed
  will be used.
- **guidance_scale** (`float`, *optional*, defaults to 5.0) --
  Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://arxiv.org/abs/2207.12598).
  `guidance_scale` is defined as `w` of equation 2. of [Imagen
  Paper](https://arxiv.org/pdf/2205.11487.pdf). Guidance scale is enabled by setting `guidance_scale >
  1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`,
  usually at the expense of lower image quality.
- **cfg_normalization** (`bool`, *optional*, defaults to False) --
  Whether to apply configuration normalization.
- **cfg_truncation** (`float`, *optional*, defaults to 1.0) --
  The truncation value for configuration.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is
  less than `1`).
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.FloatTensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`list[torch.FloatTensor]`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **negative_prompt_embeds** (`list[torch.FloatTensor]`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
  argument.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.stable_diffusion.ZImagePipelineOutput` instead of a plain
  tuple.
- **joint_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`List`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.
- **max_sequence_length** (`int`, *optional*, defaults to 512) --
  Maximum sequence length to use with the `prompt`.0`ZImagePipelineOutput` or `tuple``ZImagePipelineOutput` if
`return_dict` is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the
generated images.

Function invoked when calling the pipeline for image-to-image generation.

Examples:
```py
>>> import torch
>>> from diffusers import ZImageImg2ImgPipeline
>>> from diffusers.utils import load_image

>>> pipe = ZImageImg2ImgPipeline.from_pretrained("Z-a-o/Z-Image-Turbo", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> url = "https://raw.githubusercontent.com/CompVis/stable-diffusion/main/assets/stable-samples/img2img/sketch-mountains-input.jpg"
>>> init_image = load_image(url).resize((1024, 1024))

>>> prompt = "A fantasy landscape with mountains and a river, detailed, vibrant colors"
>>> image = pipe(
...     prompt,
...     image=init_image,
...     strength=0.6,
...     num_inference_steps=8,
...     guidance_scale=0.0,
...     generator=torch.Generator("cuda").manual_seed(42),
... ).images[0]
>>> image.save("zimage_img2img.png")
```

**Parameters:**

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`PreTrainedModel`) : A text encoder model to encode text prompts.

tokenizer (`AutoTokenizer`) : A tokenizer to tokenize text prompts.

transformer ([ZImageTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/z_image_transformer2d#diffusers.ZImageTransformer2DModel)) : A ZImage transformer model to denoise the encoded image latents.

**Returns:**

``ZImagePipelineOutput` or `tuple``

`ZImagePipelineOutput` if
`return_dict` is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the
generated images.

## ZImageInpaintPipeline[[diffusers.ZImageInpaintPipeline]]

#### diffusers.ZImageInpaintPipeline[[diffusers.ZImageInpaintPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/z_image/pipeline_z_image_inpaint.py#L170)

The ZImage pipeline for inpainting.

__call__diffusers.ZImageInpaintPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/z_image/pipeline_z_image_inpaint.py#L535[{"name": "prompt", "val": ": typing.Union[str, typing.List[str]] = None"}, {"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] = None"}, {"name": "mask_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] = None"}, {"name": "masked_image_latents", "val": ": typing.Optional[torch.FloatTensor] = None"}, {"name": "strength", "val": ": float = 1.0"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "sigmas", "val": ": list[float] | None = None"}, {"name": "guidance_scale", "val": ": float = 5.0"}, {"name": "cfg_normalization", "val": ": bool = False"}, {"name": "cfg_truncation", "val": ": float = 1.0"}, {"name": "negative_prompt", "val": ": typing.Union[str, typing.List[str], NoneType] = None"}, {"name": "num_images_per_prompt", "val": ": typing.Optional[int] = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": typing.Optional[torch.FloatTensor] = None"}, {"name": "prompt_embeds", "val": ": typing.Optional[typing.List[torch.FloatTensor]] = None"}, {"name": "negative_prompt_embeds", "val": ": typing.Optional[typing.List[torch.FloatTensor]] = None"}, {"name": "output_type", "val": ": str = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "joint_attention_kwargs", "val": ": typing.Optional[typing.Dict[str, typing.Any]] = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int, typing.Dict], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": typing.List[str] = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}]- **prompt** (`str` or `List[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **image** (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `List[torch.Tensor]`, `List[PIL.Image.Image]`, or `List[np.ndarray]`) --
  `Image`, numpy array or tensor representing an image batch to be used as the starting point. For both
  numpy array and pytorch tensor, the expected value range is between `[0, 1]`. If it's a tensor or a
  list of tensors, the expected shape should be `(B, C, H, W)` or `(C, H, W)`. If it is a numpy array or
  a list of arrays, the expected shape should be `(B, H, W, C)` or `(H, W, C)`.
- **mask_image** (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `List[torch.Tensor]`, `List[PIL.Image.Image]`, or `List[np.ndarray]`) --
  `Image`, numpy array or tensor representing a mask image for inpainting. White pixels (value 1) in the
  mask will be inpainted, black pixels (value 0) will be preserved from the original image.
- **masked_image_latents** (`torch.FloatTensor`, *optional*) --
  Pre-encoded masked image latents. If provided, the masked image encoding step will be skipped.
- **strength** (`float`, *optional*, defaults to 1.0) --
  Indicates extent to transform the reference `image`. Must be between 0 and 1. `image` is used as a
  starting point and more noise is added the higher the `strength`. The number of denoising steps depends
  on the amount of noise initially added. When `strength` is 1, added noise is maximum and the denoising
  process runs for the full number of iterations specified in `num_inference_steps`. A value of 1
  essentially ignores `image` in the masked region.
- **height** (`int`, *optional*, defaults to 1024) --
  The height in pixels of the generated image. If not provided, uses the input image height.
- **width** (`int`, *optional*, defaults to 1024) --
  The width in pixels of the generated image. If not provided, uses the input image width.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **sigmas** (`List[float]`, *optional*) --
  Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in
  their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed
  will be used.
- **guidance_scale** (`float`, *optional*, defaults to 5.0) --
  Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://arxiv.org/abs/2207.12598).
  `guidance_scale` is defined as `w` of equation 2. of [Imagen
  Paper](https://arxiv.org/pdf/2205.11487.pdf). Guidance scale is enabled by setting `guidance_scale >
  1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`,
  usually at the expense of lower image quality.
- **cfg_normalization** (`bool`, *optional*, defaults to False) --
  Whether to apply configuration normalization.
- **cfg_truncation** (`float`, *optional*, defaults to 1.0) --
  The truncation value for configuration.
- **negative_prompt** (`str` or `List[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is
  less than `1`).
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `List[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.FloatTensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`List[torch.FloatTensor]`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **negative_prompt_embeds** (`List[torch.FloatTensor]`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
  argument.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.stable_diffusion.ZImagePipelineOutput` instead of a plain
  tuple.
- **joint_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`List`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.
- **max_sequence_length** (`int`, *optional*, defaults to 512) --
  Maximum sequence length to use with the `prompt`.0`ZImagePipelineOutput` or `tuple``ZImagePipelineOutput` if
`return_dict` is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the
generated images.

Function invoked when calling the pipeline for inpainting.

Examples:
```py
>>> import torch
>>> from diffusers import ZImageInpaintPipeline
>>> from diffusers.utils import load_image

>>> pipe = ZImageInpaintPipeline.from_pretrained("Tongyi-MAI/Z-Image-Turbo", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> url = "https://raw.githubusercontent.com/CompVis/stable-diffusion/main/assets/stable-samples/img2img/sketch-mountains-input.jpg"
>>> init_image = load_image(url).resize((1024, 1024))

>>> # Create a mask (white = inpaint, black = preserve)
>>> import numpy as np
>>> from PIL import Image

>>> mask = np.zeros((1024, 1024), dtype=np.uint8)
>>> mask[256:768, 256:768] = 255  # Inpaint center region
>>> mask_image = Image.fromarray(mask)

>>> prompt = "A beautiful lake with mountains in the background"
>>> image = pipe(
...     prompt,
...     image=init_image,
...     mask_image=mask_image,
...     strength=1.0,
...     num_inference_steps=8,
...     guidance_scale=0.0,
...     generator=torch.Generator("cuda").manual_seed(42),
... ).images[0]
>>> image.save("zimage_inpaint.png")
```

**Parameters:**

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`PreTrainedModel`) : A text encoder model to encode text prompts.

tokenizer (`AutoTokenizer`) : A tokenizer to tokenize text prompts.

transformer ([ZImageTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/z_image_transformer2d#diffusers.ZImageTransformer2DModel)) : A ZImage transformer model to denoise the encoded image latents.

**Returns:**

``ZImagePipelineOutput` or `tuple``

`ZImagePipelineOutput` if
`return_dict` is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the
generated images.
#### prepare_latents[[diffusers.ZImageInpaintPipeline.prepare_latents]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/z_image/pipeline_z_image_inpaint.py#L386)

Prepare latents for inpainting, returning noise and image_latents for blending.

**Returns:**

`Tuple of (latents, noise, image_latents) where`

- latents: Noised image latents for denoising
- noise: The noise tensor used for blending
- image_latents: Clean image latents for blending
#### prepare_mask_latents[[diffusers.ZImageInpaintPipeline.prepare_mask_latents]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/z_image/pipeline_z_image_inpaint.py#L318)

Prepare mask and masked image latents for inpainting.

**Parameters:**

mask : Binary mask tensor where 1 = inpaint region, 0 = preserve region.

masked_image : Original image with masked regions zeroed out.

batch_size : Number of images to generate.

height : Output image height.

width : Output image width.

dtype : Data type for the tensors.

device : Device to place tensors on.

generator : Random generator for reproducibility.

**Returns:**

Tuple of (mask, masked_image_latents) prepared for the denoising loop.
