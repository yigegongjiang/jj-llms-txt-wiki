# Stable unCLIP

  

Stable unCLIP checkpoints are finetuned from [Stable Diffusion 2.1](./stable_diffusion/stable_diffusion_2) checkpoints to condition on CLIP image embeddings.
Stable unCLIP still conditions on text embeddings. Given the two separate conditionings, stable unCLIP can be used
for text guided image variation. When combined with an unCLIP prior, it can also be used for full text to image generation.

The abstract from the paper is:

*Contrastive models like CLIP have been shown to learn robust representations of images that capture both semantics and style. To leverage these representations for image generation, we propose a two-stage model: a prior that generates a CLIP image embedding given a text caption, and a decoder that generates an image conditioned on the image embedding. We show that explicitly generating image representations improves image diversity with minimal loss in photorealism and caption similarity. Our decoders conditioned on image representations can also produce variations of an image that preserve both its semantics and style, while varying the non-essential details absent from the image representation. Moreover, the joint embedding space of CLIP enables language-guided image manipulations in a zero-shot fashion. We use diffusion models for the decoder and experiment with both autoregressive and diffusion models for the prior, finding that the latter are computationally more efficient and produce higher-quality samples.*

## Tips

Stable unCLIP takes  `noise_level` as input during inference which determines how much noise is added to the image embeddings. A higher `noise_level` increases variation in the final un-noised images. By default, we do not add any additional noise to the image embeddings (`noise_level = 0`).

### Text-to-Image Generation
Stable unCLIP can be leveraged for text-to-image generation by pipelining it with the prior model of KakaoBrain's open source DALL-E 2 replication [Karlo](https://huggingface.co/kakaobrain/karlo-v1-alpha):

```python
import torch
from diffusers import UnCLIPScheduler, DDPMScheduler, StableUnCLIPPipeline
from diffusers.models import PriorTransformer
from transformers import CLIPTokenizer, CLIPTextModelWithProjection

prior_model_id = "kakaobrain/karlo-v1-alpha"
data_type = torch.float16
prior = PriorTransformer.from_pretrained(prior_model_id, subfolder="prior", torch_dtype=data_type)

prior_text_model_id = "openai/clip-vit-large-patch14"
prior_tokenizer = CLIPTokenizer.from_pretrained(prior_text_model_id)
prior_text_model = CLIPTextModelWithProjection.from_pretrained(prior_text_model_id, torch_dtype=data_type)
prior_scheduler = UnCLIPScheduler.from_pretrained(prior_model_id, subfolder="prior_scheduler")
prior_scheduler = DDPMScheduler.from_config(prior_scheduler.config)

stable_unclip_model_id = "stabilityai/stable-diffusion-2-1-unclip-small"

pipe = StableUnCLIPPipeline.from_pretrained(
    stable_unclip_model_id,
    torch_dtype=data_type,
    variant="fp16",
    prior_tokenizer=prior_tokenizer,
    prior_text_encoder=prior_text_model,
    prior=prior,
    prior_scheduler=prior_scheduler,
)

pipe = pipe.to("cuda")
wave_prompt = "dramatic wave, the Oceans roar, Strong wave spiral across the oceans as the waves unfurl into roaring crests; perfect wave form; perfect wave shape; dramatic wave shape; wave shape unbelievable; wave; wave shape spectacular"

image = pipe(prompt=wave_prompt).images[0]
image
```
> [!WARNING]
> For text-to-image we use `stabilityai/stable-diffusion-2-1-unclip-small` as it was trained on CLIP ViT-L/14 embedding, the same as the Karlo model prior. [stabilityai/stable-diffusion-2-1-unclip](https://hf.co/stabilityai/stable-diffusion-2-1-unclip) was trained on OpenCLIP ViT-H, so we don't recommend its use.

### Text guided Image-to-Image Variation

```python
from diffusers import StableUnCLIPImg2ImgPipeline
from diffusers.utils import load_image
import torch

pipe = StableUnCLIPImg2ImgPipeline.from_pretrained(
    "stabilityai/stable-diffusion-2-1-unclip", torch_dtype=torch.float16, variation="fp16"
)
pipe = pipe.to("cuda")

url = "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main/stable_unclip/tarsila_do_amaral.png"
init_image = load_image(url)

images = pipe(init_image).images
images[0].save("variation_image.png")
```

Optionally, you can also pass a prompt to `pipe` such as:

```python
prompt = "A fantasy landscape, trending on artstation"

image = pipe(init_image, prompt=prompt).images[0]
image
```

> [!TIP]
> Make sure to check out the Schedulers [guide](../../using-diffusers/schedulers) to learn how to explore the tradeoff between scheduler speed and quality, and see the [reuse components across pipelines](../../using-diffusers/loading#reuse-a-pipeline) section to learn how to efficiently load the same components into multiple pipelines.

## StableUnCLIPPipeline[[diffusers.StableUnCLIPPipeline]]

#### diffusers.StableUnCLIPPipeline[[diffusers.StableUnCLIPPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_diffusion/pipeline_stable_unclip.py#L70)

Pipeline for text-to-image generation using stable unCLIP.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

The pipeline also inherits the following loading methods:
- [load_textual_inversion()](/docs/diffusers/v0.39.0/en/api/loaders/textual_inversion#diffusers.loaders.TextualInversionLoaderMixin.load_textual_inversion) for loading textual inversion embeddings
- [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for loading LoRA weights
- [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for saving LoRA weights

__call__diffusers.StableUnCLIPPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_diffusion/pipeline_stable_unclip.py#L645[{"name": "prompt", "val": ": str | list[str] | None = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 20"}, {"name": "guidance_scale", "val": ": float = 10.0"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "callback", "val": ": typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None"}, {"name": "callback_steps", "val": ": int = 1"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "noise_level", "val": ": int = 0"}, {"name": "prior_num_inference_steps", "val": ": int = 25"}, {"name": "prior_guidance_scale", "val": ": float = 4.0"}, {"name": "prior_latents", "val": ": torch.Tensor | None = None"}, {"name": "clip_skip", "val": ": int | None = None"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide image generation. If not defined, you need to pass `prompt_embeds`.
- **height** (`int`, *optional*, defaults to `self.unet.config.sample_size * self.vae_scale_factor`) --
  The height in pixels of the generated image.
- **width** (`int`, *optional*, defaults to `self.unet.config.sample_size * self.vae_scale_factor`) --
  The width in pixels of the generated image.
- **num_inference_steps** (`int`, *optional*, defaults to 20) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **guidance_scale** (`float`, *optional*, defaults to 10.0) --
  A higher guidance scale value encourages the model to generate images closely linked to the text
  `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide what to not include in image generation. If not defined, you need to
  pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale  1`.
- **prior_latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for image
  embedding generation in the prior denoising process. Can be used to tweak the same generation with
  different prompts. If not provided, a latents tensor is generated by sampling using the supplied random
  `generator`.
- **clip_skip** (`int`, *optional*) --
  Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that
  the output of the pre-final layer will be used for computing the prompt embeddings.0[ImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple``~ pipeline_utils.ImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When returning
a tuple, the first element is a list with the generated images.

The call function to the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import StableUnCLIPPipeline

>>> pipe = StableUnCLIPPipeline.from_pretrained(
...     "fusing/stable-unclip-2-1-l", torch_dtype=torch.float16
... )  # TODO update model path
>>> pipe = pipe.to("cuda")

>>> prompt = "a photo of an astronaut riding a horse on mars"
>>> images = pipe(prompt).images
>>> images[0].save("astronaut_horse.png")
```

**Parameters:**

prior_tokenizer (`CLIPTokenizer`) : A `CLIPTokenizer`.

prior_text_encoder (`CLIPTextModelWithProjection`) : Frozen `CLIPTextModelWithProjection` text-encoder.

prior ([PriorTransformer](/docs/diffusers/v0.39.0/en/api/models/prior_transformer#diffusers.PriorTransformer)) : The canonical unCLIP prior to approximate the image embedding from the text embedding.

prior_scheduler (`KarrasDiffusionSchedulers`) : Scheduler used in the prior denoising process.

image_normalizer (`StableUnCLIPImageNormalizer`) : Used to normalize the predicted image embeddings before the noise is applied and un-normalize the image embeddings after the noise has been applied.

image_noising_scheduler (`KarrasDiffusionSchedulers`) : Noise schedule for adding noise to the predicted image embeddings. The amount of noise to add is determined by the `noise_level`.

tokenizer (`CLIPTokenizer`) : A `CLIPTokenizer`.

text_encoder (`CLIPTextModel`) : Frozen `CLIPTextModel` text-encoder.

unet ([UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : A [UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel) to denoise the encoded image latents.

scheduler (`KarrasDiffusionSchedulers`) : A scheduler to be used in combination with `unet` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

**Returns:**

`[ImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple``

`~ pipeline_utils.ImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When returning
a tuple, the first element is a list with the generated images.
#### enable_attention_slicing[[diffusers.StableUnCLIPPipeline.enable_attention_slicing]]

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
#### disable_attention_slicing[[diffusers.StableUnCLIPPipeline.disable_attention_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2084)

Disable sliced attention computation. If `enable_attention_slicing` was previously called, attention is
computed in one step.
#### enable_vae_slicing[[diffusers.StableUnCLIPPipeline.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2267)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### disable_vae_slicing[[diffusers.StableUnCLIPPipeline.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2280)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_xformers_memory_efficient_attention[[diffusers.StableUnCLIPPipeline.enable_xformers_memory_efficient_attention]]

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
#### disable_xformers_memory_efficient_attention[[diffusers.StableUnCLIPPipeline.disable_xformers_memory_efficient_attention]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2023)

Disable memory efficient attention from [xFormers](https://facebookresearch.github.io/xformers/).
#### encode_prompt[[diffusers.StableUnCLIPPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_diffusion/pipeline_stable_unclip.py#L297)

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
#### noise_image_embeddings[[diffusers.StableUnCLIPPipeline.noise_image_embeddings]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_diffusion/pipeline_stable_unclip.py#L599)

Add noise to the image embeddings. The amount of noise is controlled by a `noise_level` input. A higher
`noise_level` increases the variance in the final un-noised images.

The noise is applied in two ways:
1. A noise schedule is applied directly to the embeddings.
2. A vector of sinusoidal time embeddings are appended to the output.

In both cases, the amount of noise is controlled by the same `noise_level`.

The embeddings are normalized before the noise is applied and un-normalized after the noise is applied.

## StableUnCLIPImg2ImgPipeline[[diffusers.StableUnCLIPImg2ImgPipeline]]

#### diffusers.StableUnCLIPImg2ImgPipeline[[diffusers.StableUnCLIPImg2ImgPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_diffusion/pipeline_stable_unclip_img2img.py#L81)

Pipeline for text-guided image-to-image generation using stable unCLIP.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

The pipeline also inherits the following loading methods:
- [load_textual_inversion()](/docs/diffusers/v0.39.0/en/api/loaders/textual_inversion#diffusers.loaders.TextualInversionLoaderMixin.load_textual_inversion) for loading textual inversion embeddings
- [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for loading LoRA weights
- [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for saving LoRA weights

__call__diffusers.StableUnCLIPImg2ImgPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_diffusion/pipeline_stable_unclip_img2img.py#L624[{"name": "image", "val": ": torch.Tensor | PIL.Image.Image = None"}, {"name": "prompt", "val": ": str | list[str] = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 20"}, {"name": "guidance_scale", "val": ": float = 10"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "callback", "val": ": typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None"}, {"name": "callback_steps", "val": ": int = 1"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "noise_level", "val": ": int = 0"}, {"name": "image_embeds", "val": ": torch.Tensor | None = None"}, {"name": "clip_skip", "val": ": int | None = None"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, either `prompt_embeds` will be
  used or prompt is initialized to `""`.
- **image** (`torch.Tensor` or `PIL.Image.Image`) --
  `Image` or tensor representing an image batch. The image is encoded to its CLIP embedding which the
  `unet` is conditioned on. The image is _not_ encoded by the `vae` and then used as the latents in the
  denoising process like it is in the standard Stable Diffusion text-guided image variation process.
- **height** (`int`, *optional*, defaults to `self.unet.config.sample_size * self.vae_scale_factor`) --
  The height in pixels of the generated image.
- **width** (`int`, *optional*, defaults to `self.unet.config.sample_size * self.vae_scale_factor`) --
  The width in pixels of the generated image.
- **num_inference_steps** (`int`, *optional*, defaults to 20) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **guidance_scale** (`float`, *optional*, defaults to 10.0) --
  A higher guidance scale value encourages the model to generate images closely linked to the text
  `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide what to not include in image generation. If not defined, you need to
  pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale 0[ImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple``~ pipeline_utils.ImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When returning
a tuple, the first element is a list with the generated images.

The call function to the pipeline for generation.

Examples:
```py
>>> import requests
>>> import torch
>>> from PIL import Image
>>> from io import BytesIO

>>> from diffusers import StableUnCLIPImg2ImgPipeline

>>> pipe = StableUnCLIPImg2ImgPipeline.from_pretrained(
...     "stabilityai/stable-diffusion-2-1-unclip-small", torch_dtype=torch.float16
... )
>>> pipe = pipe.to("cuda")

>>> url = "https://raw.githubusercontent.com/CompVis/stable-diffusion/main/assets/stable-samples/img2img/sketch-mountains-input.jpg"

>>> response = requests.get(url)
>>> init_image = Image.open(BytesIO(response.content)).convert("RGB")
>>> init_image = init_image.resize((768, 512))

>>> prompt = "A fantasy landscape, trending on artstation"

>>> images = pipe(init_image, prompt).images
>>> images[0].save("fantasy_landscape.png")
```

**Parameters:**

feature_extractor (`CLIPImageProcessor`) : Feature extractor for image pre-processing before being encoded.

image_encoder (`CLIPVisionModelWithProjection`) : CLIP vision model for encoding images.

image_normalizer (`StableUnCLIPImageNormalizer`) : Used to normalize the predicted image embeddings before the noise is applied and un-normalize the image embeddings after the noise has been applied.

image_noising_scheduler (`KarrasDiffusionSchedulers`) : Noise schedule for adding noise to the predicted image embeddings. The amount of noise to add is determined by the `noise_level`.

tokenizer (`~transformers.CLIPTokenizer`) : A [`~transformers.CLIPTokenizer`)].

text_encoder ([CLIPTextModel](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTextModel)) : Frozen [CLIPTextModel](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTextModel) text-encoder.

unet ([UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : A [UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel) to denoise the encoded image latents.

scheduler (`KarrasDiffusionSchedulers`) : A scheduler to be used in combination with `unet` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

**Returns:**

`[ImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple``

`~ pipeline_utils.ImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When returning
a tuple, the first element is a list with the generated images.
#### enable_attention_slicing[[diffusers.StableUnCLIPImg2ImgPipeline.enable_attention_slicing]]

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
#### disable_attention_slicing[[diffusers.StableUnCLIPImg2ImgPipeline.disable_attention_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2084)

Disable sliced attention computation. If `enable_attention_slicing` was previously called, attention is
computed in one step.
#### enable_vae_slicing[[diffusers.StableUnCLIPImg2ImgPipeline.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2267)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### disable_vae_slicing[[diffusers.StableUnCLIPImg2ImgPipeline.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2280)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_xformers_memory_efficient_attention[[diffusers.StableUnCLIPImg2ImgPipeline.enable_xformers_memory_efficient_attention]]

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
#### disable_xformers_memory_efficient_attention[[diffusers.StableUnCLIPImg2ImgPipeline.disable_xformers_memory_efficient_attention]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2023)

Disable memory efficient attention from [xFormers](https://facebookresearch.github.io/xformers/).
#### encode_prompt[[diffusers.StableUnCLIPImg2ImgPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_diffusion/pipeline_stable_unclip_img2img.py#L259)

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
#### noise_image_embeddings[[diffusers.StableUnCLIPImg2ImgPipeline.noise_image_embeddings]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_diffusion/pipeline_stable_unclip_img2img.py#L578)

Add noise to the image embeddings. The amount of noise is controlled by a `noise_level` input. A higher
`noise_level` increases the variance in the final un-noised images.

The noise is applied in two ways:
1. A noise schedule is applied directly to the embeddings.
2. A vector of sinusoidal time embeddings are appended to the output.

In both cases, the amount of noise is controlled by the same `noise_level`.

The embeddings are normalized before the noise is applied and un-normalized after the noise is applied.

## ImagePipelineOutput[[diffusers.ImagePipelineOutput]]
#### diffusers.ImagePipelineOutput[[diffusers.ImagePipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L122)

Output class for image pipelines.

**Parameters:**

images (`List[PIL.Image.Image]` or `np.ndarray`) : List of denoised PIL images of length `batch_size` or NumPy array of shape `(batch_size, height, width, num_channels)`.
