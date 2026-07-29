# Super-resolution

  

The Stable Diffusion upscaler diffusion model was created by the researchers and engineers from [CompVis](https://github.com/CompVis), [Stability AI](https://stability.ai/), and [LAION](https://laion.ai/). It is used to enhance the resolution of input images by a factor of 4.

> [!TIP]
> Make sure to check out the Stable Diffusion [Tips](overview#tips) section to learn how to explore the tradeoff between scheduler speed and quality, and how to reuse pipeline components efficiently!
>
> If you're interested in using one of the official checkpoints for a task, explore the [CompVis](https://huggingface.co/CompVis) and [Stability AI](https://huggingface.co/stabilityai) Hub organizations!

## StableDiffusionUpscalePipeline[[diffusers.StableDiffusionUpscalePipeline]]

#### diffusers.StableDiffusionUpscalePipeline[[diffusers.StableDiffusionUpscalePipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_diffusion/pipeline_stable_diffusion_upscale.py#L78)

Pipeline for text-guided image super-resolution using Stable Diffusion 2.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

The pipeline also inherits the following loading methods:
- [load_textual_inversion()](/docs/diffusers/v0.39.0/en/api/loaders/textual_inversion#diffusers.loaders.TextualInversionLoaderMixin.load_textual_inversion) for loading textual inversion embeddings
- [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for loading LoRA weights
- [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for saving LoRA weights
- [from_single_file()](/docs/diffusers/v0.39.0/en/api/loaders/single_file#diffusers.loaders.FromSingleFileMixin.from_single_file) for loading `.ckpt` files

__call__diffusers.StableDiffusionUpscalePipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_diffusion/pipeline_stable_diffusion_upscale.py#L535[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] = None"}, {"name": "num_inference_steps", "val": ": int = 75"}, {"name": "guidance_scale", "val": ": float = 9.0"}, {"name": "noise_level", "val": ": int = 20"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "callback", "val": ": typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None"}, {"name": "callback_steps", "val": ": int = 1"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "clip_skip", "val": ": int = None"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide image generation. If not defined, you need to pass `prompt_embeds`.
- **image** (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) --
  `Image` or tensor representing an image batch to be upscaled.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **guidance_scale** (`float`, *optional*, defaults to 7.5) --
  A higher guidance scale value encourages the model to generate images closely linked to the text
  `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.
- **noise_level** (`int`, *optional*, defaults to 20) --
  The amount of noise to add to the upscaled input image. Must be in the range `[0, max_noise_level]`
  where `max_noise_level` is defined by the scheduler. A higher `noise_level` adds more noise to the
  input, increasing variation but reducing fidelity to the source image.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide what to not include in image generation. If not defined, you need to
  pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale 0[StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) or `tuple`If `return_dict` is `True`, [StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated images and the
second element is a list of `bool`s indicating whether the corresponding generated image contains
"not-safe-for-work" (nsfw) content.

The call function to the pipeline for generation.

Examples:
```py
>>> import requests
>>> from PIL import Image
>>> from io import BytesIO
>>> from diffusers import StableDiffusionUpscalePipeline
>>> import torch

>>> # load model and scheduler
>>> model_id = "stabilityai/stable-diffusion-x4-upscaler"
>>> pipeline = StableDiffusionUpscalePipeline.from_pretrained(
...     model_id, variant="fp16", torch_dtype=torch.float16
... )
>>> pipeline = pipeline.to("cuda")

>>> # let's download an  image
>>> url = "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main/sd2-upscale/low_res_cat.png"
>>> response = requests.get(url)
>>> low_res_img = Image.open(BytesIO(response.content)).convert("RGB")
>>> low_res_img = low_res_img.resize((128, 128))
>>> prompt = "a white cat"

>>> upscaled_image = pipeline(prompt=prompt, image=low_res_img).images[0]
>>> upscaled_image.save("upsampled_cat.png")
```

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) model to encode and decode images to and from latent representations.

text_encoder ([CLIPTextModel](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTextModel)) : Frozen text-encoder ([clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14)).

tokenizer ([CLIPTokenizer](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTokenizer)) : A `CLIPTokenizer` to tokenize text.

unet ([UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : A `UNet2DConditionModel` to denoise the encoded image latents.

low_res_scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler used to add initial noise to the low resolution conditioning image. It must be an instance of [DDPMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddpm#diffusers.DDPMScheduler).

scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `unet` to denoise the encoded image latents. Can be one of [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), [LMSDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lms_discrete#diffusers.LMSDiscreteScheduler), or [PNDMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/pndm#diffusers.PNDMScheduler).

**Returns:**

`[StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) or `tuple``

If `return_dict` is `True`, [StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated images and the
second element is a list of `bool`s indicating whether the corresponding generated image contains
"not-safe-for-work" (nsfw) content.
#### enable_attention_slicing[[diffusers.StableDiffusionUpscalePipeline.enable_attention_slicing]]

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
#### disable_attention_slicing[[diffusers.StableDiffusionUpscalePipeline.disable_attention_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2084)

Disable sliced attention computation. If `enable_attention_slicing` was previously called, attention is
computed in one step.
#### enable_xformers_memory_efficient_attention[[diffusers.StableDiffusionUpscalePipeline.enable_xformers_memory_efficient_attention]]

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
#### disable_xformers_memory_efficient_attention[[diffusers.StableDiffusionUpscalePipeline.disable_xformers_memory_efficient_attention]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2023)

Disable memory efficient attention from [xFormers](https://facebookresearch.github.io/xformers/).
#### encode_prompt[[diffusers.StableDiffusionUpscalePipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_diffusion/pipeline_stable_diffusion_upscale.py#L217)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

do_classifier_free_guidance (`bool`) : whether to use classifier free guidance or not

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

lora_scale (`float`, *optional*) : A LoRA scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

clip_skip (`int`, *optional*) : Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that the output of the pre-final layer will be used for computing the prompt embeddings.

## StableDiffusionPipelineOutput[[diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput]]

#### diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput[[diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_diffusion/pipeline_output.py#L10)

Output class for Stable Diffusion pipelines.

**Parameters:**

images (`list[PIL.Image.Image]` or `np.ndarray`) : list of denoised PIL images of length `batch_size` or NumPy array of shape `(batch_size, height, width, num_channels)`.

nsfw_content_detected (`list[bool]`) : list indicating whether the corresponding generated image contains "not-safe-for-work" (nsfw) content or `None` if safety checking could not be performed.
