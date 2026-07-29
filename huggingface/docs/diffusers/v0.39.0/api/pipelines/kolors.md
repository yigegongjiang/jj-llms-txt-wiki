# Kolors: Effective Training of Diffusion Model for Photorealistic Text-to-Image Synthesis

  
  

![](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/kolors/kolors_header_collage.png)

Kolors is a large-scale text-to-image generation model based on latent diffusion, developed by [the Kuaishou Kolors team](https://github.com/Kwai-Kolors/Kolors). Trained on billions of text-image pairs, Kolors exhibits significant advantages over both open-source and closed-source models in visual quality, complex semantic accuracy, and text rendering for both Chinese and English characters. Furthermore, Kolors supports both Chinese and English inputs, demonstrating strong performance in understanding and generating Chinese-specific content. For more details, please refer to this [technical report](https://github.com/Kwai-Kolors/Kolors/blob/master/imgs/Kolors_paper.pdf).

The abstract from the technical report is:

*We present Kolors, a latent diffusion model for text-to-image synthesis, characterized by its profound understanding of both English and Chinese, as well as an impressive degree of photorealism. There are three key insights contributing to the development of Kolors. Firstly, unlike large language model T5 used in Imagen and Stable Diffusion 3, Kolors is built upon the General Language Model (GLM), which enhances its comprehension capabilities in both English and Chinese. Moreover, we employ a multimodal large language model to recaption the extensive training dataset for fine-grained text understanding. These strategies significantly improve Kolors’ ability to comprehend intricate semantics, particularly those involving multiple entities, and enable its advanced text rendering capabilities. Secondly, we divide the training of Kolors into two phases: the concept learning phase with broad knowledge and the quality improvement phase with specifically curated high-aesthetic data. Furthermore, we investigate the critical role of the noise schedule and introduce a novel schedule to optimize high-resolution image generation. These strategies collectively enhance the visual appeal of the generated high-resolution images. Lastly, we propose a category-balanced benchmark KolorsPrompts, which serves as a guide for the training and evaluation of Kolors. Consequently, even when employing the commonly used U-Net backbone, Kolors has demonstrated remarkable performance in human evaluations, surpassing the existing open-source models and achieving Midjourney-v6 level performance, especially in terms of visual appeal. We will release the code and weights of Kolors at , and hope that it will benefit future research and applications in the visual generation community.*

## Usage Example

```python
import torch

from diffusers import DPMSolverMultistepScheduler, KolorsPipeline

pipe = KolorsPipeline.from_pretrained("Kwai-Kolors/Kolors-diffusers", torch_dtype=torch.float16, variant="fp16")
pipe.to("cuda")
pipe.scheduler = DPMSolverMultistepScheduler.from_config(pipe.scheduler.config, use_karras_sigmas=True)

image = pipe(
    prompt='一张瓢虫的照片，微距，变焦，高质量，电影，拿着一个牌子，写着"可图"',
    negative_prompt="",
    guidance_scale=6.5,
    num_inference_steps=25,
).images[0]

image.save("kolors_sample.png")
```

### IP Adapter

Kolors needs a different IP Adapter to work, and it uses [Openai-CLIP-336](https://huggingface.co/openai/clip-vit-large-patch14-336) as an image encoder.

> [!TIP]
> Using an IP Adapter with Kolors requires more than 24GB of VRAM. To use it, we recommend using [enable_model_cpu_offload()](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline.enable_model_cpu_offload) on consumer GPUs.

> [!TIP]
> While Kolors is integrated in Diffusers, you need to load the image encoder from a revision to use the safetensor files. You can still use the main branch of the original repository if you're comfortable loading pickle checkpoints.

```python
import torch
from transformers import CLIPVisionModelWithProjection

from diffusers import DPMSolverMultistepScheduler, KolorsPipeline
from diffusers.utils import load_image

image_encoder = CLIPVisionModelWithProjection.from_pretrained(
    "Kwai-Kolors/Kolors-IP-Adapter-Plus",
    subfolder="image_encoder",
    low_cpu_mem_usage=True,
    torch_dtype=torch.float16,
    revision="refs/pr/4",
)

pipe = KolorsPipeline.from_pretrained(
    "Kwai-Kolors/Kolors-diffusers", image_encoder=image_encoder, torch_dtype=torch.float16, variant="fp16"
)
pipe.scheduler = DPMSolverMultistepScheduler.from_config(pipe.scheduler.config, use_karras_sigmas=True)

pipe.load_ip_adapter(
    "Kwai-Kolors/Kolors-IP-Adapter-Plus",
    subfolder="",
    weight_name="ip_adapter_plus_general.safetensors",
    revision="refs/pr/4",
    image_encoder_folder=None,
)
pipe.enable_model_cpu_offload()

ipa_image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/kolors/cat_square.png")

image = pipe(
    prompt="best quality, high quality",
    negative_prompt="",
    guidance_scale=6.5,
    num_inference_steps=25,
    ip_adapter_image=ipa_image,
).images[0]

image.save("kolors_ipa_sample.png")
```

## KolorsPipeline[[diffusers.KolorsPipeline]]

#### diffusers.KolorsPipeline[[diffusers.KolorsPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kolors/pipeline_kolors.py#L123)

Pipeline for text-to-image generation using Kolors.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

The pipeline also inherits the following loading methods:
- [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for loading LoRA weights
- [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for saving LoRA weights
- [load_ip_adapter()](/docs/diffusers/v0.39.0/en/api/loaders/ip_adapter#diffusers.loaders.IPAdapterMixin.load_ip_adapter) for loading IP Adapters

encode_promptdiffusers.KolorsPipeline.encode_prompthttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kolors/pipeline_kolors.py#L199[{"name": "prompt", "val": ""}, {"name": "device", "val": ": torch.device | None = None"}, {"name": "num_images_per_prompt", "val": ": int = 1"}, {"name": "do_classifier_free_guidance", "val": ": bool = True"}, {"name": "negative_prompt", "val": " = None"}, {"name": "prompt_embeds", "val": ": torch.FloatTensor | None = None"}, {"name": "pooled_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.FloatTensor | None = None"}, {"name": "negative_pooled_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "max_sequence_length", "val": ": int = 256"}]- **prompt** (`str` or `list[str]`, *optional*) --
  prompt to be encoded
- **device** -- (`torch.device`):
  torch device
- **num_images_per_prompt** (`int`) --
  number of images that should be generated per prompt
- **do_classifier_free_guidance** (`bool`) --
  whether to use classifier free guidance or not
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is
  less than `1`).
- **prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **pooled_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting.
  If not provided, pooled text embeddings will be generated from `prompt` input argument.
- **negative_prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
  argument.
- **negative_pooled_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, pooled negative_prompt_embeds will be generated from `negative_prompt`
  input argument.
- **max_sequence_length** (`int` defaults to 256) -- Maximum sequence length to use with the `prompt`.0

Encodes the prompt into text encoder hidden states.

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`ChatGLMModel`) : Frozen text-encoder. Kolors uses [ChatGLM3-6B](https://huggingface.co/THUDM/chatglm3-6b).

tokenizer (`ChatGLMTokenizer`) : Tokenizer of class [ChatGLMTokenizer](https://huggingface.co/THUDM/chatglm3-6b/blob/main/tokenization_chatglm.py).

unet ([UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the encoded image latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `unet` to denoise the encoded image latents. Can be one of [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), [LMSDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lms_discrete#diffusers.LMSDiscreteScheduler), or [PNDMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/pndm#diffusers.PNDMScheduler).

force_zeros_for_empty_prompt (`bool`, *optional*, defaults to `"False"`) : Whether the negative prompt embeddings shall be forced to always be set to 0. Also see the config of `Kwai-Kolors/Kolors-diffusers`.
#### get_guidance_scale_embedding[[diffusers.KolorsPipeline.get_guidance_scale_embedding]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kolors/pipeline_kolors.py#L590)

See https://github.com/google-research/vdm/blob/dc27b98a554f65cdc654b800da5aa1846545d41b/model_vdm.py#L298

**Parameters:**

w (`torch.Tensor`) : Generate embedding vectors with a specified guidance scale to subsequently enrich timestep embeddings.

embedding_dim (`int`, *optional*, defaults to 512) : Dimension of the embeddings to generate.

dtype (`torch.dtype`, *optional*, defaults to `torch.float32`) : Data type of the generated embeddings.

**Returns:**

``torch.Tensor``

Embedding vectors with shape `(len(w), embedding_dim)`.

- all
- __call__

## KolorsImg2ImgPipeline[[diffusers.KolorsImg2ImgPipeline]]

#### diffusers.KolorsImg2ImgPipeline[[diffusers.KolorsImg2ImgPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kolors/pipeline_kolors_img2img.py#L142)

Pipeline for text-to-image generation using Kolors.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

The pipeline also inherits the following loading methods:
- [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionXLLoraLoaderMixin.load_lora_weights) for loading LoRA weights
- [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionXLLoraLoaderMixin.save_lora_weights) for saving LoRA weights
- [load_ip_adapter()](/docs/diffusers/v0.39.0/en/api/loaders/ip_adapter#diffusers.loaders.IPAdapterMixin.load_ip_adapter) for loading IP Adapters

encode_promptdiffusers.KolorsImg2ImgPipeline.encode_prompthttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kolors/pipeline_kolors_img2img.py#L219[{"name": "prompt", "val": ""}, {"name": "device", "val": ": torch.device | None = None"}, {"name": "num_images_per_prompt", "val": ": int = 1"}, {"name": "do_classifier_free_guidance", "val": ": bool = True"}, {"name": "negative_prompt", "val": " = None"}, {"name": "prompt_embeds", "val": ": torch.FloatTensor | None = None"}, {"name": "pooled_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.FloatTensor | None = None"}, {"name": "negative_pooled_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "max_sequence_length", "val": ": int = 256"}]- **prompt** (`str` or `list[str]`, *optional*) --
  prompt to be encoded
- **device** -- (`torch.device`):
  torch device
- **num_images_per_prompt** (`int`) --
  number of images that should be generated per prompt
- **do_classifier_free_guidance** (`bool`) --
  whether to use classifier free guidance or not
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is
  less than `1`).
- **prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **pooled_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting.
  If not provided, pooled text embeddings will be generated from `prompt` input argument.
- **negative_prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
  argument.
- **negative_pooled_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, pooled negative_prompt_embeds will be generated from `negative_prompt`
  input argument.
- **max_sequence_length** (`int` defaults to 256) -- Maximum sequence length to use with the `prompt`.0

Encodes the prompt into text encoder hidden states.

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`ChatGLMModel`) : Frozen text-encoder. Kolors uses [ChatGLM3-6B](https://huggingface.co/THUDM/chatglm3-6b).

tokenizer (`ChatGLMTokenizer`) : Tokenizer of class [ChatGLMTokenizer](https://huggingface.co/THUDM/chatglm3-6b/blob/main/tokenization_chatglm.py).

unet ([UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the encoded image latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `unet` to denoise the encoded image latents. Can be one of [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), [LMSDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lms_discrete#diffusers.LMSDiscreteScheduler), or [PNDMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/pndm#diffusers.PNDMScheduler).

force_zeros_for_empty_prompt (`bool`, *optional*, defaults to `"False"`) : Whether the negative prompt embeddings shall be forced to always be set to 0. Also see the config of `Kwai-Kolors/Kolors-diffusers`.
#### get_guidance_scale_embedding[[diffusers.KolorsImg2ImgPipeline.get_guidance_scale_embedding]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kolors/pipeline_kolors_img2img.py#L718)

See https://github.com/google-research/vdm/blob/dc27b98a554f65cdc654b800da5aa1846545d41b/model_vdm.py#L298

**Parameters:**

w (`torch.Tensor`) : Generate embedding vectors with a specified guidance scale to subsequently enrich timestep embeddings.

embedding_dim (`int`, *optional*, defaults to 512) : Dimension of the embeddings to generate.

dtype (`torch.dtype`, *optional*, defaults to `torch.float32`) : Data type of the generated embeddings.

**Returns:**

``torch.Tensor``

Embedding vectors with shape `(len(w), embedding_dim)`.

- all
- __call__
