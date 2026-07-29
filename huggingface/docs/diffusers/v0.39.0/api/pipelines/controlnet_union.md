# ControlNetUnion

  

ControlNetUnionModel is an implementation of ControlNet for Stable Diffusion XL.

The ControlNet model was introduced in [ControlNetPlus](https://github.com/xinsir6/ControlNetPlus) by xinsir6. It supports multiple conditioning inputs without increasing computation.

*We design a new architecture that can support 10+ control types in condition text-to-image generation and can generate high resolution images visually comparable with midjourney. The network is based on the original ControlNet architecture, we propose two new modules to: 1 Extend the original ControlNet to support different image conditions using the same network parameter. 2 Support multiple conditions input without increasing computation offload, which is especially important for designers who want to edit image in detail, different conditions use the same condition encoder, without adding extra computations or parameters.*

## StableDiffusionXLControlNetUnionPipeline[[diffusers.StableDiffusionXLControlNetUnionPipeline]]
#### diffusers.StableDiffusionXLControlNetUnionPipeline[[diffusers.StableDiffusionXLControlNetUnionPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/controlnet/pipeline_controlnet_union_sd_xl.py#L175)

Pipeline for text-to-image generation using Stable Diffusion XL with ControlNet guidance.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

The pipeline also inherits the following loading methods:
- [load_textual_inversion()](/docs/diffusers/v0.39.0/en/api/loaders/textual_inversion#diffusers.loaders.TextualInversionLoaderMixin.load_textual_inversion) for loading textual inversion embeddings
- [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionXLLoraLoaderMixin.load_lora_weights) for loading LoRA weights
- [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionXLLoraLoaderMixin.save_lora_weights) for saving LoRA weights
- [from_single_file()](/docs/diffusers/v0.39.0/en/api/loaders/single_file#diffusers.loaders.FromSingleFileMixin.from_single_file) for loading `.ckpt` files
- [load_ip_adapter()](/docs/diffusers/v0.39.0/en/api/loaders/ip_adapter#diffusers.loaders.IPAdapterMixin.load_ip_adapter) for loading IP Adapters

__call__diffusers.StableDiffusionXLControlNetUnionPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/controlnet/pipeline_controlnet_union_sd_xl.py#L974[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "prompt_2", "val": ": str | list[str] | None = None"}, {"name": "control_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | list[PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor]] = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "timesteps", "val": ": list = None"}, {"name": "sigmas", "val": ": list = None"}, {"name": "denoising_end", "val": ": float | None = None"}, {"name": "guidance_scale", "val": ": float = 5.0"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "negative_prompt_2", "val": ": str | list[str] | None = None"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "pooled_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_pooled_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "ip_adapter_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "ip_adapter_image_embeds", "val": ": list[torch.Tensor] | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "controlnet_conditioning_scale", "val": ": float | list[float] = 1.0"}, {"name": "guess_mode", "val": ": bool = False"}, {"name": "control_guidance_start", "val": ": float | list[float] = 0.0"}, {"name": "control_guidance_end", "val": ": float | list[float] = 1.0"}, {"name": "control_mode", "val": ": int | list[int] | list[list[int]] | None = None"}, {"name": "original_size", "val": ": tuple = None"}, {"name": "crops_coords_top_left", "val": ": tuple = (0, 0)"}, {"name": "target_size", "val": ": tuple = None"}, {"name": "negative_original_size", "val": ": tuple[int, int] | None = None"}, {"name": "negative_crops_coords_top_left", "val": ": tuple = (0, 0)"}, {"name": "negative_target_size", "val": ": tuple[int, int] | None = None"}, {"name": "clip_skip", "val": ": int | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide image generation. If not defined, you need to pass `prompt_embeds`.
- **prompt_2** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is
  used in both text-encoders.
- **control_image** (`PipelineImageInput` or `list[PipelineImageInput]`, *optional*) --
  The ControlNet input condition to provide guidance to the `unet` for generation. If the type is
  specified as `torch.Tensor`, it is passed to ControlNet as is. `PIL.Image.Image` can also be accepted
  as an image. The dimensions of the output image defaults to `image`'s dimensions. If height and/or
  width are passed, `image` is resized accordingly. If multiple ControlNets are specified in `init`,
  images must be passed as a list such that each element of the list can be correctly batched for input
  to a single ControlNet.
- **height** (`int`, *optional*, defaults to `self.unet.config.sample_size * self.vae_scale_factor`) --
  The height in pixels of the generated image. Anything below 512 pixels won't work well for
  [stabilityai/stable-diffusion-xl-base-1.0](https://huggingface.co/stabilityai/stable-diffusion-xl-base-1.0)
  and checkpoints that are not specifically fine-tuned on low resolutions.
- **width** (`int`, *optional*, defaults to `self.unet.config.sample_size * self.vae_scale_factor`) --
  The width in pixels of the generated image. Anything below 512 pixels won't work well for
  [stabilityai/stable-diffusion-xl-base-1.0](https://huggingface.co/stabilityai/stable-diffusion-xl-base-1.0)
  and checkpoints that are not specifically fine-tuned on low resolutions.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **timesteps** (`list[int]`, *optional*) --
  Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument
  in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is
  passed will be used. Must be in descending order.
- **sigmas** (`list[float]`, *optional*) --
  Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in
  their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed
  will be used.
- **denoising_end** (`float`, *optional*) --
  When specified, determines the fraction (between 0.0 and 1.0) of the total denoising process to be
  completed before it is intentionally prematurely terminated. As a result, the returned sample will
  still retain a substantial amount of noise as determined by the discrete timesteps selected by the
  scheduler. The denoising_end parameter should ideally be utilized when this pipeline forms a part of a
  "Mixture of Denoisers" multi-pipeline setup, as elaborated in [**Refining the Image
  Output**](https://huggingface.co/docs/diffusers/api/pipelines/stable_diffusion/stable_diffusion_xl#refining-the-image-output)
- **guidance_scale** (`float`, *optional*, defaults to 5.0) --
  A higher guidance scale value encourages the model to generate images closely linked to the text
  `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide what to not include in image generation. If not defined, you need to
  pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale 0[StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) or `tuple`If `return_dict` is `True`, [StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) is returned,
otherwise a `tuple` is returned containing the output images.

The call function to the pipeline for generation.

Examples:
```py
>>> # !pip install controlnet_aux
>>> from controlnet_aux import LineartAnimeDetector
>>> from diffusers import StableDiffusionXLControlNetUnionPipeline, ControlNetUnionModel, AutoencoderKL
>>> from diffusers.utils import load_image
>>> import torch

>>> prompt = "A cat"
>>> # download an image
>>> image = load_image(
...     "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main/kandinsky/cat.png"
... ).resize((1024, 1024))
>>> # initialize the models and pipeline
>>> controlnet = ControlNetUnionModel.from_pretrained(
...     "xinsir/controlnet-union-sdxl-1.0", torch_dtype=torch.float16
... )
>>> vae = AutoencoderKL.from_pretrained("madebyollin/sdxl-vae-fp16-fix", torch_dtype=torch.float16)
>>> pipe = StableDiffusionXLControlNetUnionPipeline.from_pretrained(
...     "stabilityai/stable-diffusion-xl-base-1.0",
...     controlnet=controlnet,
...     vae=vae,
...     torch_dtype=torch.float16,
...     variant="fp16",
... )
>>> pipe.enable_model_cpu_offload()
>>> # prepare image
>>> processor = LineartAnimeDetector.from_pretrained("lllyasviel/Annotators")
>>> controlnet_img = processor(image, output_type="pil")
>>> # generate image
>>> image = pipe(prompt, control_image=[controlnet_img], control_mode=[3], height=1024, width=1024).images[0]
```

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) model to encode and decode images to and from latent representations.

text_encoder ([CLIPTextModel](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTextModel)) : Frozen text-encoder ([clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14)).

text_encoder_2 ([CLIPTextModelWithProjection](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTextModelWithProjection)) : Second frozen text-encoder ([laion/CLIP-ViT-bigG-14-laion2B-39B-b160k](https://huggingface.co/laion/CLIP-ViT-bigG-14-laion2B-39B-b160k)).

tokenizer ([CLIPTokenizer](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTokenizer)) : A `CLIPTokenizer` to tokenize text.

tokenizer_2 ([CLIPTokenizer](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTokenizer)) : A `CLIPTokenizer` to tokenize text.

unet ([UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : A `UNet2DConditionModel` to denoise the encoded image latents.

controlnet ([ControlNetUnionModel](/docs/diffusers/v0.39.0/en/api/models/controlnet_union#diffusers.ControlNetUnionModel)`) : Provides additional conditioning to the `unet` during the denoising process.

scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `unet` to denoise the encoded image latents. Can be one of [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), [LMSDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lms_discrete#diffusers.LMSDiscreteScheduler), or [PNDMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/pndm#diffusers.PNDMScheduler).

force_zeros_for_empty_prompt (`bool`, *optional*, defaults to `"True"`) : Whether the negative prompt embeddings should always be set to 0. Also see the config of `stabilityai/stable-diffusion-xl-base-1-0`.

add_watermarker (`bool`, *optional*) : Whether to use the [invisible_watermark](https://github.com/ShieldMnt/invisible-watermark/) library to watermark output images. If not defined, it defaults to `True` if the package is installed; otherwise no watermarker is used.

**Returns:**

`[StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) or `tuple``

If `return_dict` is `True`, [StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) is returned,
otherwise a `tuple` is returned containing the output images.
#### encode_prompt[[diffusers.StableDiffusionXLControlNetUnionPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/controlnet/pipeline_controlnet_union_sd_xl.py#L291)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is used in both text-encoders

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

do_classifier_free_guidance (`bool`) : whether to use classifier free guidance or not

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

negative_prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `negative_prompt` is used in both text-encoders

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

pooled_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

negative_pooled_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled negative_prompt_embeds will be generated from `negative_prompt` input argument.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

clip_skip (`int`, *optional*) : Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that the output of the pre-final layer will be used for computing the prompt embeddings.
#### get_guidance_scale_embedding[[diffusers.StableDiffusionXLControlNetUnionPipeline.get_guidance_scale_embedding]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/controlnet/pipeline_controlnet_union_sd_xl.py#L913)

See https://github.com/google-research/vdm/blob/dc27b98a554f65cdc654b800da5aa1846545d41b/model_vdm.py#L298

**Parameters:**

w (`torch.Tensor`) : Generate embedding vectors with a specified guidance scale to subsequently enrich timestep embeddings.

embedding_dim (`int`, *optional*, defaults to 512) : Dimension of the embeddings to generate.

dtype (`torch.dtype`, *optional*, defaults to `torch.float32`) : Data type of the generated embeddings.

**Returns:**

``torch.Tensor``

Embedding vectors with shape `(len(w), embedding_dim)`.

## StableDiffusionXLControlNetUnionImg2ImgPipeline[[diffusers.StableDiffusionXLControlNetUnionImg2ImgPipeline]]
#### diffusers.StableDiffusionXLControlNetUnionImg2ImgPipeline[[diffusers.StableDiffusionXLControlNetUnionImg2ImgPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/controlnet/pipeline_controlnet_union_sd_xl_img2img.py#L188)

Pipeline for image-to-image generation using Stable Diffusion XL with ControlNet guidance.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

The pipeline also inherits the following loading methods:
- [load_textual_inversion()](/docs/diffusers/v0.39.0/en/api/loaders/textual_inversion#diffusers.loaders.TextualInversionLoaderMixin.load_textual_inversion) for loading textual inversion embeddings
- [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionXLLoraLoaderMixin.load_lora_weights) for loading LoRA weights
- [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionXLLoraLoaderMixin.save_lora_weights) for saving LoRA weights
- [load_ip_adapter()](/docs/diffusers/v0.39.0/en/api/loaders/ip_adapter#diffusers.loaders.IPAdapterMixin.load_ip_adapter) for loading IP Adapters

__call__diffusers.StableDiffusionXLControlNetUnionImg2ImgPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/controlnet/pipeline_controlnet_union_sd_xl_img2img.py#L1066[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "prompt_2", "val": ": str | list[str] | None = None"}, {"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] = None"}, {"name": "control_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | list[PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor]] = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "strength", "val": ": float = 0.8"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "guidance_scale", "val": ": float = 5.0"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "negative_prompt_2", "val": ": str | list[str] | None = None"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "pooled_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_pooled_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "ip_adapter_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "ip_adapter_image_embeds", "val": ": list[torch.Tensor] | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "controlnet_conditioning_scale", "val": ": float | list[float] = 0.8"}, {"name": "guess_mode", "val": ": bool = False"}, {"name": "control_guidance_start", "val": ": float | list[float] = 0.0"}, {"name": "control_guidance_end", "val": ": float | list[float] = 1.0"}, {"name": "control_mode", "val": ": int | list[int] | list[list[int]] | None = None"}, {"name": "original_size", "val": ": tuple = None"}, {"name": "crops_coords_top_left", "val": ": tuple = (0, 0)"}, {"name": "target_size", "val": ": tuple = None"}, {"name": "negative_original_size", "val": ": tuple[int, int] | None = None"}, {"name": "negative_crops_coords_top_left", "val": ": tuple = (0, 0)"}, {"name": "negative_target_size", "val": ": tuple[int, int] | None = None"}, {"name": "aesthetic_score", "val": ": float = 6.0"}, {"name": "negative_aesthetic_score", "val": ": float = 2.5"}, {"name": "clip_skip", "val": ": int | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "**kwargs", "val": ""}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **prompt_2** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is
  used in both text-encoders
- **image** (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, `list[np.ndarray]`, --
  `list[list[torch.Tensor]]`, `list[list[np.ndarray]]` or `list[list[PIL.Image.Image]]`):
  The initial image will be used as the starting point for the image generation process. Can also accept
  image latents as `image`, if passing latents directly, it will not be encoded again.
- **control_image** (`PipelineImageInput` or `list[PipelineImageInput]`, *optional*) --
  The ControlNet input condition to provide guidance to the `unet` for generation. If the type is
  specified as `torch.Tensor`, it is passed to ControlNet as is. `PIL.Image.Image` can also be accepted
  as an image. The dimensions of the output image defaults to `image`'s dimensions. If height and/or
  width are passed, `image` is resized accordingly. If multiple ControlNets are specified in `init`,
  images must be passed as a list such that each element of the list can be correctly batched for input
  to a single ControlNet.
- **height** (`int`, *optional*, defaults to the size of control_image) --
  The height in pixels of the generated image. Anything below 512 pixels won't work well for
  [stabilityai/stable-diffusion-xl-base-1.0](https://huggingface.co/stabilityai/stable-diffusion-xl-base-1.0)
  and checkpoints that are not specifically fine-tuned on low resolutions.
- **width** (`int`, *optional*, defaults to the size of control_image) --
  The width in pixels of the generated image. Anything below 512 pixels won't work well for
  [stabilityai/stable-diffusion-xl-base-1.0](https://huggingface.co/stabilityai/stable-diffusion-xl-base-1.0)
  and checkpoints that are not specifically fine-tuned on low resolutions.
- **strength** (`float`, *optional*, defaults to 0.8) --
  Indicates extent to transform the reference `image`. Must be between 0 and 1. `image` is used as a
  starting point and more noise is added the higher the `strength`. The number of denoising steps depends
  on the amount of noise initially added. When `strength` is 1, added noise is maximum and the denoising
  process runs for the full number of iterations specified in `num_inference_steps`. A value of 1
  essentially ignores `image`.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **guidance_scale** (`float`, *optional*, defaults to 7.5) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2.
  of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting
  `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to
  the text `prompt`, usually at the expense of lower image quality.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is
  less than `1`).
- **negative_prompt_2** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation to be sent to `tokenizer_2` and
  `text_encoder_2`. If not defined, `negative_prompt` is used in both text-encoders
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **eta** (`float`, *optional*, defaults to 0.0) --
  Corresponds to parameter eta (η) in the DDIM paper: https://huggingface.co/papers/2010.02502. Only
  applies to [schedulers.DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), will be ignored for others.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
  argument.
- **pooled_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting.
  If not provided, pooled text embeddings will be generated from `prompt` input argument.
- **negative_pooled_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, pooled negative_prompt_embeds will be generated from `negative_prompt`
  input argument.
- **ip_adapter_image** -- (`PipelineImageInput`, *optional*): Optional image input to work with IP Adapters.
- **ip_adapter_image_embeds** (`list[torch.Tensor]`, *optional*) --
  Pre-generated image embeddings for IP-Adapter. It should be a list of length same as number of
  IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. It should
  contain the negative image embedding if `do_classifier_free_guidance` is set to `True`. If not
  provided, embeddings are computed from the `ip_adapter_image` input argument.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a [StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) instead of a
  plain tuple.
- **cross_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **controlnet_conditioning_scale** (`float` or `list[float]`, *optional*, defaults to 1.0) --
  The outputs of the ControlNet are multiplied by `controlnet_conditioning_scale` before they are added
  to the residual in the original `unet`. If multiple ControlNets are specified in `init`, you can set
  the corresponding scale as a list.
- **guess_mode** (`bool`, *optional*, defaults to `False`) --
  In this mode, the ControlNet encoder will try best to recognize the content of the input image even if
  you remove all prompts. The `guidance_scale` between 3.0 and 5.0 is recommended.
- **control_guidance_start** (`float` or `list[float]`, *optional*, defaults to 0.0) --
  The percentage of total steps at which the ControlNet starts applying.
- **control_guidance_end** (`float` or `list[float]`, *optional*, defaults to 1.0) --
  The percentage of total steps at which the ControlNet stops applying.
- **control_mode** (`int` or `list[int]` or `list[list[int]], *optional*) --
  The control condition types for the ControlNet. See the ControlNet's model card forinformation on the
  available control modes. If multiple ControlNets are specified in `init`, control_mode should be a list
  where each ControlNet should have its corresponding control mode list. Should reflect the order of
  conditions in control_image
- **original_size** (`tuple[int]`, *optional*, defaults to (1024, 1024)) --
  If `original_size` is not the same as `target_size` the image will appear to be down- or upsampled.
  `original_size` defaults to `(height, width)` if not specified. Part of SDXL's micro-conditioning as
  explained in section 2.2 of
  [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).
- **crops_coords_top_left** (`tuple[int]`, *optional*, defaults to (0, 0)) --
  `crops_coords_top_left` can be used to generate an image that appears to be "cropped" from the position
  `crops_coords_top_left` downwards. Favorable, well-centered images are usually achieved by setting
  `crops_coords_top_left` to (0, 0). Part of SDXL's micro-conditioning as explained in section 2.2 of
  [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).
- **target_size** (`tuple[int]`, *optional*, defaults to (1024, 1024)) --
  For most cases, `target_size` should be set to the desired height and width of the generated image. If
  not specified it will default to `(height, width)`. Part of SDXL's micro-conditioning as explained in
  section 2.2 of [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).
- **negative_original_size** (`tuple[int]`, *optional*, defaults to (1024, 1024)) --
  To negatively condition the generation process based on a specific image resolution. Part of SDXL's
  micro-conditioning as explained in section 2.2 of
  [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952). For more
  information, refer to this issue thread: https://github.com/huggingface/diffusers/issues/4208.
- **negative_crops_coords_top_left** (`tuple[int]`, *optional*, defaults to (0, 0)) --
  To negatively condition the generation process based on a specific crop coordinates. Part of SDXL's
  micro-conditioning as explained in section 2.2 of
  [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952). For more
  information, refer to this issue thread: https://github.com/huggingface/diffusers/issues/4208.
- **negative_target_size** (`tuple[int]`, *optional*, defaults to (1024, 1024)) --
  To negatively condition the generation process based on a target image resolution. It should be as same
  as the `target_size` for most cases. Part of SDXL's micro-conditioning as explained in section 2.2 of
  [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952). For more
  information, refer to this issue thread: https://github.com/huggingface/diffusers/issues/4208.
- **aesthetic_score** (`float`, *optional*, defaults to 6.0) --
  Used to simulate an aesthetic score of the generated image by influencing the positive text condition.
  Part of SDXL's micro-conditioning as explained in section 2.2 of
  [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).
- **negative_aesthetic_score** (`float`, *optional*, defaults to 2.5) --
  Part of SDXL's micro-conditioning as explained in section 2.2 of
  [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952). Can be used to
  simulate an aesthetic score of the generated image by influencing the negative text condition.
- **clip_skip** (`int`, *optional*) --
  Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that
  the output of the pre-final layer will be used for computing the prompt embeddings.
- **callback_on_step_end** (`Callable`, `PipelineCallback`, `MultiPipelineCallbacks`, *optional*) --
  A function or a subclass of `PipelineCallback` or `MultiPipelineCallbacks` that is called at the end of
  each denoising step during the inference. with the following arguments: `callback_on_step_end(self:
  DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a
  list of all tensors as specified by `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`list`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.0[StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) or `tuple`[StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) if `return_dict` is True, otherwise a `tuple`
containing the output images.

Function invoked when calling the pipeline for generation.

Examples:
```py
# !pip install controlnet_aux
from diffusers import (
    StableDiffusionXLControlNetUnionImg2ImgPipeline,
    ControlNetUnionModel,
    AutoencoderKL,
)
from diffusers.utils import load_image
import torch
from PIL import Image
import numpy as np

prompt = "A cat"
# download an image
image = load_image(
    "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main/kandinsky/cat.png"
)
# initialize the models and pipeline
controlnet = ControlNetUnionModel.from_pretrained(
    "brad-twinkl/controlnet-union-sdxl-1.0-promax", torch_dtype=torch.float16
)
vae = AutoencoderKL.from_pretrained("madebyollin/sdxl-vae-fp16-fix", torch_dtype=torch.float16)
pipe = StableDiffusionXLControlNetUnionImg2ImgPipeline.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0",
    controlnet=controlnet,
    vae=vae,
    torch_dtype=torch.float16,
    variant="fp16",
).to("cuda")
# `enable_model_cpu_offload` is not recommended due to multiple generations
height = image.height
width = image.width
ratio = np.sqrt(1024.0 * 1024.0 / (width * height))
# 3 * 3 upscale correspond to 16 * 3 multiply, 2 * 2 correspond to 16 * 2 multiply and so on.
scale_image_factor = 3
base_factor = 16
factor = scale_image_factor * base_factor
W, H = int(width * ratio) // factor * factor, int(height * ratio) // factor * factor
image = image.resize((W, H))
target_width = W // scale_image_factor
target_height = H // scale_image_factor
images = []
crops_coords_list = [
    (0, 0),
    (0, width // 2),
    (height // 2, 0),
    (width // 2, height // 2),
    0,
    0,
    0,
    0,
    0,
]
for i in range(scale_image_factor):
    for j in range(scale_image_factor):
        left = j * target_width
        top = i * target_height
        right = left + target_width
        bottom = top + target_height
        cropped_image = image.crop((left, top, right, bottom))
        cropped_image = cropped_image.resize((W, H))
        images.append(cropped_image)
# set ControlNetUnion input
result_images = []
for sub_img, crops_coords in zip(images, crops_coords_list):
    new_width, new_height = W, H
    out = pipe(
        prompt=[prompt] * 1,
        image=sub_img,
        control_image=[sub_img],
        control_mode=[6],
        width=new_width,
        height=new_height,
        num_inference_steps=30,
        crops_coords_top_left=(W, H),
        target_size=(W, H),
        original_size=(W * 2, H * 2),
    )
    result_images.append(out.images[0])
new_im = Image.new("RGB", (new_width * scale_image_factor, new_height * scale_image_factor))
new_im.paste(result_images[0], (0, 0))
new_im.paste(result_images[1], (new_width, 0))
new_im.paste(result_images[2], (new_width * 2, 0))
new_im.paste(result_images[3], (0, new_height))
new_im.paste(result_images[4], (new_width, new_height))
new_im.paste(result_images[5], (new_width * 2, new_height))
new_im.paste(result_images[6], (0, new_height * 2))
new_im.paste(result_images[7], (new_width, new_height * 2))
new_im.paste(result_images[8], (new_width * 2, new_height * 2))
```

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : Frozen text-encoder. Stable Diffusion uses the text portion of [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

text_encoder_2 (` CLIPTextModelWithProjection`) : Second frozen text-encoder. Stable Diffusion XL uses the text and pool portion of [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModelWithProjection), specifically the [laion/CLIP-ViT-bigG-14-laion2B-39B-b160k](https://huggingface.co/laion/CLIP-ViT-bigG-14-laion2B-39B-b160k) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer_2 (`CLIPTokenizer`) : Second Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

unet ([UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the encoded image latents.

controlnet ([ControlNetUnionModel](/docs/diffusers/v0.39.0/en/api/models/controlnet_union#diffusers.ControlNetUnionModel)) : Provides additional conditioning to the unet during the denoising process.

scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `unet` to denoise the encoded image latents. Can be one of [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), [LMSDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lms_discrete#diffusers.LMSDiscreteScheduler), or [PNDMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/pndm#diffusers.PNDMScheduler).

requires_aesthetics_score (`bool`, *optional*, defaults to `"False"`) : Whether the `unet` requires an `aesthetic_score` condition to be passed during inference. Also see the config of `stabilityai/stable-diffusion-xl-refiner-1-0`.

force_zeros_for_empty_prompt (`bool`, *optional*, defaults to `"True"`) : Whether the negative prompt embeddings shall be forced to always be set to 0. Also see the config of `stabilityai/stable-diffusion-xl-base-1-0`.

add_watermarker (`bool`, *optional*) : Whether to use the [invisible_watermark library](https://github.com/ShieldMnt/invisible-watermark/) to watermark output images. If not defined, it will default to True if the package is installed, otherwise no watermarker will be used.

feature_extractor ([CLIPImageProcessor](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPImageProcessor)) : A `CLIPImageProcessor` to extract features from generated images; used as inputs to the `safety_checker`.

**Returns:**

`[StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) or `tuple``

[StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) if `return_dict` is True, otherwise a `tuple`
containing the output images.
#### encode_prompt[[diffusers.StableDiffusionXLControlNetUnionImg2ImgPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/controlnet/pipeline_controlnet_union_sd_xl_img2img.py#L310)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is used in both text-encoders

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

do_classifier_free_guidance (`bool`) : whether to use classifier free guidance or not

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

negative_prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `negative_prompt` is used in both text-encoders

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

pooled_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

negative_pooled_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled negative_prompt_embeds will be generated from `negative_prompt` input argument.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

clip_skip (`int`, *optional*) : Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that the output of the pre-final layer will be used for computing the prompt embeddings.

## StableDiffusionXLControlNetUnionInpaintPipeline[[diffusers.StableDiffusionXLControlNetUnionInpaintPipeline]]
#### diffusers.StableDiffusionXLControlNetUnionInpaintPipeline[[diffusers.StableDiffusionXLControlNetUnionInpaintPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/controlnet/pipeline_controlnet_union_inpaint_sd_xl.py#L160)

Pipeline for text-to-image generation using Stable Diffusion XL.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

The pipeline also inherits the following loading methods:
- [load_textual_inversion()](/docs/diffusers/v0.39.0/en/api/loaders/textual_inversion#diffusers.loaders.TextualInversionLoaderMixin.load_textual_inversion) for loading textual inversion embeddings
- [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionXLLoraLoaderMixin.load_lora_weights) for loading LoRA weights
- [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionXLLoraLoaderMixin.save_lora_weights) for saving LoRA weights
- [from_single_file()](/docs/diffusers/v0.39.0/en/api/loaders/single_file#diffusers.loaders.FromSingleFileMixin.from_single_file) for loading `.ckpt` files
- [load_ip_adapter()](/docs/diffusers/v0.39.0/en/api/loaders/ip_adapter#diffusers.loaders.IPAdapterMixin.load_ip_adapter) for loading IP Adapters

__call__diffusers.StableDiffusionXLControlNetUnionInpaintPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/controlnet/pipeline_controlnet_union_inpaint_sd_xl.py#L1145[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "prompt_2", "val": ": str | list[str] | None = None"}, {"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] = None"}, {"name": "mask_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] = None"}, {"name": "control_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | list[PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor]] = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "padding_mask_crop", "val": ": int | None = None"}, {"name": "strength", "val": ": float = 0.9999"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "denoising_start", "val": ": float | None = None"}, {"name": "denoising_end", "val": ": float | None = None"}, {"name": "guidance_scale", "val": ": float = 5.0"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "negative_prompt_2", "val": ": str | list[str] | None = None"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "ip_adapter_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "ip_adapter_image_embeds", "val": ": list[torch.Tensor] | None = None"}, {"name": "pooled_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_pooled_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "controlnet_conditioning_scale", "val": ": float | list[float] = 1.0"}, {"name": "guess_mode", "val": ": bool = False"}, {"name": "control_guidance_start", "val": ": float | list[float] = 0.0"}, {"name": "control_guidance_end", "val": ": float | list[float] = 1.0"}, {"name": "control_mode", "val": ": int | list[int] | list[list[int]] | None = None"}, {"name": "guidance_rescale", "val": ": float = 0.0"}, {"name": "original_size", "val": ": tuple = None"}, {"name": "crops_coords_top_left", "val": ": tuple = (0, 0)"}, {"name": "target_size", "val": ": tuple = None"}, {"name": "aesthetic_score", "val": ": float = 6.0"}, {"name": "negative_aesthetic_score", "val": ": float = 2.5"}, {"name": "clip_skip", "val": ": int | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "**kwargs", "val": ""}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **prompt_2** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is
  used in both text-encoders
- **image** (`PIL.Image.Image`) --
  `Image`, or tensor representing an image batch which will be inpainted, *i.e.* parts of the image will
  be masked out with `mask_image` and repainted according to `prompt`.
- **mask_image** (`PIL.Image.Image`) --
  `Image`, or tensor representing an image batch, to mask `image`. White pixels in the mask will be
  repainted, while black pixels will be preserved. If `mask_image` is a PIL image, it will be converted
  to a single channel (luminance) before use. If it's a tensor, it should contain one color channel (L)
  instead of 3, so the expected shape would be `(B, H, W, 1)`.
- **control_image** (`PipelineImageInput` or `list[PipelineImageInput]`, *optional*) --
  The ControlNet input condition to provide guidance to the `unet` for generation. If the type is
  specified as `torch.Tensor`, it is passed to ControlNet as is. `PIL.Image.Image` can also be accepted
  as an image. The dimensions of the output image defaults to `image`'s dimensions. If height and/or
  width are passed, `image` is resized accordingly. If multiple ControlNets are specified in `init`,
  images must be passed as a list such that each element of the list can be correctly batched for input
  to a single ControlNet.
- **height** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The height in pixels of the generated image.
- **width** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The width in pixels of the generated image.
- **padding_mask_crop** (`int`, *optional*, defaults to `None`) --
  The size of margin in the crop to be applied to the image and masking. If `None`, no crop is applied to
  image and mask_image. If `padding_mask_crop` is not `None`, it will first find a rectangular region
  with the same aspect ration of the image and contains all masked area, and then expand that area based
  on `padding_mask_crop`. The image and mask_image will then be cropped based on the expanded area before
  resizing to the original image size for inpainting. This is useful when the masked area is small while
  the image is large and contain information irrelevant for inpainting, such as background.
- **strength** (`float`, *optional*, defaults to 0.9999) --
  Conceptually, indicates how much to transform the masked portion of the reference `image`. Must be
  between 0 and 1. `image` will be used as a starting point, adding more noise to it the larger the
  `strength`. The number of denoising steps depends on the amount of noise initially added. When
  `strength` is 1, added noise will be maximum and the denoising process will run for the full number of
  iterations specified in `num_inference_steps`. A value of 1, therefore, essentially ignores the masked
  portion of the reference `image`. Note that in the case of `denoising_start` being declared as an
  integer, the value of `strength` will be ignored.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **denoising_start** (`float`, *optional*) --
  When specified, indicates the fraction (between 0.0 and 1.0) of the total denoising process to be
  bypassed before it is initiated. Consequently, the initial part of the denoising process is skipped and
  it is assumed that the passed `image` is a partly denoised image. Note that when this is specified,
  strength will be ignored. The `denoising_start` parameter is particularly beneficial when this pipeline
  is integrated into a "Mixture of Denoisers" multi-pipeline setup, as detailed in [**Refining the Image
  Output**](https://huggingface.co/docs/diffusers/api/pipelines/stable_diffusion/stable_diffusion_xl#refining-the-image-output).
- **denoising_end** (`float`, *optional*) --
  When specified, determines the fraction (between 0.0 and 1.0) of the total denoising process to be
  completed before it is intentionally prematurely terminated. As a result, the returned sample will
  still retain a substantial amount of noise (ca. final 20% of timesteps still needed) and should be
  denoised by a successor pipeline that has `denoising_start` set to 0.8 so that it only denoises the
  final 20% of the scheduler. The denoising_end parameter should ideally be utilized when this pipeline
  forms a part of a "Mixture of Denoisers" multi-pipeline setup, as elaborated in [**Refining the Image
  Output**](https://huggingface.co/docs/diffusers/api/pipelines/stable_diffusion/stable_diffusion_xl#refining-the-image-output).
- **guidance_scale** (`float`, *optional*, defaults to 7.5) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2.
  of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting
  `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to
  the text `prompt`, usually at the expense of lower image quality.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is
  less than `1`).
- **negative_prompt_2** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation to be sent to `tokenizer_2` and
  `text_encoder_2`. If not defined, `negative_prompt` is used in both text-encoders
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
  argument.
- **ip_adapter_image** -- (`PipelineImageInput`, *optional*): Optional image input to work with IP Adapters.
- **ip_adapter_image_embeds** (`list[torch.Tensor]`, *optional*) --
  Pre-generated image embeddings for IP-Adapter. It should be a list of length same as number of
  IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. It should
  contain the negative image embedding if `do_classifier_free_guidance` is set to `True`. If not
  provided, embeddings are computed from the `ip_adapter_image` input argument.
- **pooled_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting.
  If not provided, pooled text embeddings will be generated from `prompt` input argument.
- **negative_pooled_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, pooled negative_prompt_embeds will be generated from `negative_prompt`
  input argument.
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **eta** (`float`, *optional*, defaults to 0.0) --
  Corresponds to parameter eta (η) in the DDIM paper: https://huggingface.co/papers/2010.02502. Only
  applies to [schedulers.DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), will be ignored for others.
- **generator** (`torch.Generator`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a [StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) instead of a
  plain tuple.
- **cross_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **controlnet_conditioning_scale** (`float` or `list[float]`, *optional*, defaults to 1.0) --
  The outputs of the ControlNet are multiplied by `controlnet_conditioning_scale` before they are added
  to the residual in the original `unet`. If multiple ControlNets are specified in `init`, you can set
  the corresponding scale as a list.
- **guess_mode** (`bool`, *optional*, defaults to `False`) --
  The ControlNet encoder tries to recognize the content of the input image even if you remove all
  prompts. A `guidance_scale` value between 3.0 and 5.0 is recommended.
- **control_guidance_start** (`float` or `list[float]`, *optional*, defaults to 0.0) --
  The percentage of total steps at which the ControlNet starts applying.
- **control_guidance_end** (`float` or `list[float]`, *optional*, defaults to 1.0) --
  The percentage of total steps at which the ControlNet stops applying.
- **control_mode** (`int` or `list[int]` or `list[list[int]], *optional*) --
  The control condition types for the ControlNet. See the ControlNet's model card forinformation on the
  available control modes. If multiple ControlNets are specified in `init`, control_mode should be a list
  where each ControlNet should have its corresponding control mode list. Should reflect the order of
  conditions in control_image.
- **guidance_rescale** (`float`, *optional*, defaults to 0.0) --
  Guidance rescale factor from [Common Diffusion Noise Schedules and Sample Steps are
  Flawed](https://arxiv.org/pdf/2305.08891.pdf).
- **original_size** (`tuple[int]`, *optional*, defaults to (1024, 1024)) --
  If `original_size` is not the same as `target_size` the image will appear to be down- or upsampled.
  `original_size` defaults to `(width, height)` if not specified. Part of SDXL's micro-conditioning as
  explained in section 2.2 of
  [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).
- **crops_coords_top_left** (`tuple[int]`, *optional*, defaults to (0, 0)) --
  `crops_coords_top_left` can be used to generate an image that appears to be "cropped" from the position
  `crops_coords_top_left` downwards. Favorable, well-centered images are usually achieved by setting
  `crops_coords_top_left` to (0, 0). Part of SDXL's micro-conditioning as explained in section 2.2 of
  [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).
- **target_size** (`tuple[int]`, *optional*, defaults to (1024, 1024)) --
  For most cases, `target_size` should be set to the desired height and width of the generated image. If
  not specified it will default to `(width, height)`. Part of SDXL's micro-conditioning as explained in
  section 2.2 of [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).
- **aesthetic_score** (`float`, *optional*, defaults to 6.0) --
  Used to simulate an aesthetic score of the generated image by influencing the positive text condition.
  Part of SDXL's micro-conditioning as explained in section 2.2 of
  [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).
- **negative_aesthetic_score** (`float`, *optional*, defaults to 2.5) --
  Part of SDXL's micro-conditioning as explained in section 2.2 of
  [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952). Can be used to
  simulate an aesthetic score of the generated image by influencing the negative text condition.
- **clip_skip** (`int`, *optional*) --
  Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that
  the output of the pre-final layer will be used for computing the prompt embeddings.
- **callback_on_step_end** (`Callable`, `PipelineCallback`, `MultiPipelineCallbacks`, *optional*) --
  A function or a subclass of `PipelineCallback` or `MultiPipelineCallbacks` that is called at the end of
  each denoising step during the inference. with the following arguments: `callback_on_step_end(self:
  DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a
  list of all tensors as specified by `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`list`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.0`~pipelines.stable_diffusion.StableDiffusionXLPipelineOutput` or `tuple``~pipelines.stable_diffusion.StableDiffusionXLPipelineOutput` if `return_dict` is True, otherwise a
`tuple. `tuple. When returning a tuple, the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
from diffusers import StableDiffusionXLControlNetUnionInpaintPipeline, ControlNetUnionModel, AutoencoderKL
from diffusers.utils import load_image
import torch
import numpy as np
from PIL import Image

prompt = "A cat"
# download an image
image = load_image(
    "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main/in_paint/overture-creations-5sI6fQgYIuo.png"
).resize((1024, 1024))
mask = load_image(
    "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main/in_paint/overture-creations-5sI6fQgYIuo_mask.png"
).resize((1024, 1024))
# initialize the models and pipeline
controlnet = ControlNetUnionModel.from_pretrained(
    "brad-twinkl/controlnet-union-sdxl-1.0-promax", torch_dtype=torch.float16
)
vae = AutoencoderKL.from_pretrained("madebyollin/sdxl-vae-fp16-fix", torch_dtype=torch.float16)
pipe = StableDiffusionXLControlNetUnionInpaintPipeline.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0",
    controlnet=controlnet,
    vae=vae,
    torch_dtype=torch.float16,
    variant="fp16",
)
pipe.enable_model_cpu_offload()
controlnet_img = image.copy()
controlnet_img_np = np.array(controlnet_img)
mask_np = np.array(mask)
controlnet_img_np[mask_np > 0] = 0
controlnet_img = Image.fromarray(controlnet_img_np)
# generate image
image = pipe(prompt, image=image, mask_image=mask, control_image=[controlnet_img], control_mode=[7]).images[0]
image.save("inpaint.png")
```

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : Frozen text-encoder. Stable Diffusion XL uses the text portion of [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

text_encoder_2 (` CLIPTextModelWithProjection`) : Second frozen text-encoder. Stable Diffusion XL uses the text and pool portion of [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModelWithProjection), specifically the [laion/CLIP-ViT-bigG-14-laion2B-39B-b160k](https://huggingface.co/laion/CLIP-ViT-bigG-14-laion2B-39B-b160k) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer_2 (`CLIPTokenizer`) : Second Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

unet ([UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the encoded image latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `unet` to denoise the encoded image latents. Can be one of [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), [LMSDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lms_discrete#diffusers.LMSDiscreteScheduler), or [PNDMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/pndm#diffusers.PNDMScheduler).

**Returns:**

``~pipelines.stable_diffusion.StableDiffusionXLPipelineOutput` or `tuple``

`~pipelines.stable_diffusion.StableDiffusionXLPipelineOutput` if `return_dict` is True, otherwise a
`tuple. `tuple. When returning a tuple, the first element is a list with the generated images.
#### encode_prompt[[diffusers.StableDiffusionXLControlNetUnionInpaintPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/controlnet/pipeline_controlnet_union_inpaint_sd_xl.py#L281)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is used in both text-encoders

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

do_classifier_free_guidance (`bool`) : whether to use classifier free guidance or not

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

negative_prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `negative_prompt` is used in both text-encoders

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

pooled_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

negative_pooled_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled negative_prompt_embeds will be generated from `negative_prompt` input argument.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

clip_skip (`int`, *optional*) : Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that the output of the pre-final layer will be used for computing the prompt embeddings.
