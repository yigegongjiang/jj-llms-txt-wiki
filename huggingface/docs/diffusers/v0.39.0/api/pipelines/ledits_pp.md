# LEDITS++

  

LEDITS++ was proposed in [LEDITS++: Limitless Image Editing using Text-to-Image Models](https://huggingface.co/papers/2311.16711) by Manuel Brack, Felix Friedrich, Katharina Kornmeier, Linoy Tsaban, Patrick Schramowski, Kristian Kersting, Apolinário Passos.

The abstract from the paper is:

*Text-to-image diffusion models have recently received increasing interest for their astonishing ability to produce high-fidelity images from solely text inputs. Subsequent research efforts aim to exploit and apply their capabilities to real image editing. However, existing image-to-image methods are often inefficient, imprecise, and of limited versatility. They either require time-consuming fine-tuning, deviate unnecessarily strongly from the input image, and/or lack support for multiple, simultaneous edits. To address these issues, we introduce LEDITS++, an efficient yet versatile and precise textual image manipulation technique. LEDITS++'s novel inversion approach requires no tuning nor optimization and produces high-fidelity results with a few diffusion steps. Second, our methodology supports multiple simultaneous edits and is architecture-agnostic. Third, we use a novel implicit masking technique that limits changes to relevant image regions. We propose the novel TEdBench++ benchmark as part of our exhaustive evaluation. Our results demonstrate the capabilities of LEDITS++ and its improvements over previous methods. The project page is available at https://leditsplusplus-project.static.hf.space .*

> [!TIP]
> You can find additional information about LEDITS++ on the [project page](https://leditsplusplus-project.static.hf.space/index.html) and try it out in a [demo](https://huggingface.co/spaces/editing-images/leditsplusplus).

> [!WARNING]
> Due to some backward compatibility issues with the current diffusers implementation of [DPMSolverMultistepScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/multistep_dpm_solver#diffusers.DPMSolverMultistepScheduler) this implementation of LEdits++ can no longer guarantee perfect inversion.
> This issue is unlikely to have any noticeable effects on applied use-cases. However, we provide an alternative implementation that guarantees perfect inversion in a dedicated [GitHub repo](https://github.com/ml-research/ledits_pp).

We provide two distinct pipelines based on different pre-trained models.

## LEditsPPPipelineStableDiffusion[[diffusers.LEditsPPPipelineStableDiffusion]]
#### diffusers.LEditsPPPipelineStableDiffusion[[diffusers.LEditsPPPipelineStableDiffusion]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ledits_pp/pipeline_leditspp_stable_diffusion.py#L269)

Pipeline for textual image editing using LEDits++ with Stable Diffusion.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline) and builds on the [StableDiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/text2img#diffusers.StableDiffusionPipeline). Check the superclass
documentation for the generic methods implemented for all pipelines (downloading, saving, running on a particular
device, etc.).

__call__diffusers.LEditsPPPipelineStableDiffusion.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ledits_pp/pipeline_leditspp_stable_diffusion.py#L773[{"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "editing_prompt", "val": ": str | list[str] | None = None"}, {"name": "editing_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "reverse_editing_direction", "val": ": bool | list[bool] | None = False"}, {"name": "edit_guidance_scale", "val": ": float | list[float] | None = 5"}, {"name": "edit_warmup_steps", "val": ": int | list[int] | None = 0"}, {"name": "edit_cooldown_steps", "val": ": int | list[int] | None = None"}, {"name": "edit_threshold", "val": ": float | list[float] | None = 0.9"}, {"name": "user_mask", "val": ": torch.Tensor | None = None"}, {"name": "sem_guidance", "val": ": list[torch.Tensor] | None = None"}, {"name": "use_cross_attn_mask", "val": ": bool = False"}, {"name": "use_intersect_mask", "val": ": bool = True"}, {"name": "attn_store_steps", "val": ": list[int] | None = []"}, {"name": "store_averaged_over_steps", "val": ": bool = True"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "guidance_rescale", "val": ": float = 0.0"}, {"name": "clip_skip", "val": ": int | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "**kwargs", "val": ""}]- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. Ignored when not using guidance (i.e., ignored
  if `guidance_scale` is less than `1`).
- **generator** (`torch.Generator`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a [LEditsPPDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ledits_pp#diffusers.pipelines.LEditsPPDiffusionPipelineOutput) instead of a plain
  tuple.
- **editing_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. The image is reconstructed by setting
  `editing_prompt = None`. Guidance direction of prompt should be specified via
  `reverse_editing_direction`.
- **editing_prompt_embeds** (`torch.Tensor>`, *optional*) --
  Pre-computed embeddings to use for guiding the image generation. Guidance direction of embedding should
  be specified via `reverse_editing_direction`.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs (prompt weighting). If
  not provided, `negative_prompt_embeds` are generated from the `negative_prompt` input argument.
- **reverse_editing_direction** (`bool` or `list[bool]`, *optional*, defaults to `False`) --
  Whether the corresponding prompt in `editing_prompt` should be increased or decreased.
- **edit_guidance_scale** (`float` or `list[float]`, *optional*, defaults to 5) --
  Guidance scale for guiding the image generation. If provided as list values should correspond to
  `editing_prompt`. `edit_guidance_scale` is defined as `s_e` of equation 12 of [LEDITS++
  Paper](https://huggingface.co/papers/2301.12247).
- **edit_warmup_steps** (`float` or `list[float]`, *optional*, defaults to 10) --
  Number of diffusion steps (for each prompt) for which guidance will not be applied.
- **edit_cooldown_steps** (`float` or `list[float]`, *optional*, defaults to `None`) --
  Number of diffusion steps (for each prompt) after which guidance will no longer be applied.
- **edit_threshold** (`float` or `list[float]`, *optional*, defaults to 0.9) --
  Masking threshold of guidance. Threshold should be proportional to the image region that is modified.
  'edit_threshold' is defined as 'λ' of equation 12 of [LEDITS++
  Paper](https://huggingface.co/papers/2301.12247).
- **user_mask** (`torch.Tensor`, *optional*) --
  User-provided mask for even better control over the editing process. This is helpful when LEDITS++'s
  implicit masks do not meet user preferences.
- **sem_guidance** (`list[torch.Tensor]`, *optional*) --
  list of pre-generated guidance vectors to be applied at generation. Length of the list has to
  correspond to `num_inference_steps`.
- **use_cross_attn_mask** (`bool`, defaults to `False`) --
  Whether cross-attention masks are used. Cross-attention masks are always used when use_intersect_mask
  is set to true. Cross-attention masks are defined as 'M^1' of equation 12 of [LEDITS++
  paper](https://huggingface.co/papers/2311.16711).
- **use_intersect_mask** (`bool`, defaults to `True`) --
  Whether the masking term is calculated as intersection of cross-attention masks and masks derived from
  the noise estimate. Cross-attention mask are defined as 'M^1' and masks derived from the noise estimate
  are defined as 'M^2' of equation 12 of [LEDITS++ paper](https://huggingface.co/papers/2311.16711).
- **attn_store_steps** (`list[int]`, *optional*) --
  Steps for which the attention maps are stored in the AttentionStore. Just for visualization purposes.
- **store_averaged_over_steps** (`bool`, defaults to `True`) --
  Whether the attention maps for the 'attn_store_steps' are stored averaged over the diffusion steps. If
  False, attention maps for each step are stores separately. Just for visualization purposes.
- **cross_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined in
  [`self.processor`](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **guidance_rescale** (`float`, *optional*, defaults to 0.0) --
  Guidance rescale factor from [Common Diffusion Noise Schedules and Sample Steps are
  Flawed](https://huggingface.co/papers/2305.08891). Guidance rescale factor should fix overexposure when
  using zero terminal SNR.
- **clip_skip** (`int`, *optional*) --
  Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that
  the output of the pre-final layer will be used for computing the prompt embeddings.
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`list`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.0[LEditsPPDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ledits_pp#diffusers.pipelines.LEditsPPDiffusionPipelineOutput) or `tuple`[LEditsPPDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ledits_pp#diffusers.pipelines.LEditsPPDiffusionPipelineOutput) if `return_dict` is True, otherwise a `tuple. When
returning a tuple, the first element is a list with the generated images, and the second element is a list
of `bool`s denoting whether the corresponding generated image likely represents "not-safe-for-work" (nsfw)
content, according to the `safety_checker`.

The call function to the pipeline for editing. The
[invert()](/docs/diffusers/v0.39.0/en/api/pipelines/ledits_pp#diffusers.LEditsPPPipelineStableDiffusion.invert) method has to be called beforehand. Edits will
always be performed for the last inverted image(s).

Examples:
```py
>>> import torch

>>> from diffusers import LEditsPPPipelineStableDiffusion
>>> from diffusers.utils import load_image

>>> pipe = LEditsPPPipelineStableDiffusion.from_pretrained(
...     "stable-diffusion-v1-5/stable-diffusion-v1-5", variant="fp16", torch_dtype=torch.float16
... )
>>> pipe.enable_vae_tiling()
>>> pipe = pipe.to("cuda")

>>> img_url = "https://www.aiml.informatik.tu-darmstadt.de/people/mbrack/cherry_blossom.png"
>>> image = load_image(img_url).resize((512, 512))

>>> _ = pipe.invert(image=image, num_inversion_steps=50, skip=0.1)

>>> edited_image = pipe(
...     editing_prompt=["cherry blossom"], edit_guidance_scale=10.0, edit_threshold=0.75
... ).images[0]
```

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder ([CLIPTextModel](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTextModel)) : Frozen text-encoder. Stable Diffusion uses the text portion of [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

tokenizer ([CLIPTokenizer](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTokenizer)) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

unet ([UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the encoded image latents.

scheduler ([DPMSolverMultistepScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/multistep_dpm_solver#diffusers.DPMSolverMultistepScheduler) or [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler)) : A scheduler to be used in combination with `unet` to denoise the encoded image latens. Can be one of [DPMSolverMultistepScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/multistep_dpm_solver#diffusers.DPMSolverMultistepScheduler) or [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler). If any other scheduler is passed it will automatically be set to [DPMSolverMultistepScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/multistep_dpm_solver#diffusers.DPMSolverMultistepScheduler).

safety_checker (`StableDiffusionSafetyChecker`) : Classification module that estimates whether generated images could be considered offensive or harmful. Please, refer to the [model card](https://huggingface.co/CompVis/stable-diffusion-v1-4) for details.

feature_extractor ([CLIPImageProcessor](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPImageProcessor)) : Model that extracts features from generated images to be used as inputs for the `safety_checker`.

**Returns:**

`[LEditsPPDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ledits_pp#diffusers.pipelines.LEditsPPDiffusionPipelineOutput) or `tuple``

[LEditsPPDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ledits_pp#diffusers.pipelines.LEditsPPDiffusionPipelineOutput) if `return_dict` is True, otherwise a `tuple. When
returning a tuple, the first element is a list with the generated images, and the second element is a list
of `bool`s denoting whether the corresponding generated image likely represents "not-safe-for-work" (nsfw)
content, according to the `safety_checker`.
#### invert[[diffusers.LEditsPPPipelineStableDiffusion.invert]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ledits_pp/pipeline_leditspp_stable_diffusion.py#L1277)

The function to the pipeline for image inversion as described by the [LEDITS++
Paper](https://huggingface.co/papers/2301.12247). If the scheduler is set to [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler) the
inversion proposed by [edit-friendly DPDM](https://huggingface.co/papers/2304.06140) will be performed instead.

**Parameters:**

image (`PipelineImageInput`) : Input for the image(s) that are to be edited. Multiple input images have to default to the same aspect ratio.

source_prompt (`str`, defaults to `""`) : Prompt describing the input image that will be used for guidance during inversion. Guidance is disabled if the `source_prompt` is `""`.

source_guidance_scale (`float`, defaults to `3.5`) : Strength of guidance during inversion.

num_inversion_steps (`int`, defaults to `30`) : Number of total performed inversion steps after discarding the initial `skip` steps.

skip (`float`, defaults to `0.15`) : Portion of initial steps that will be ignored for inversion and subsequent generation. Lower values will lead to stronger changes to the input image. `skip` has to be between `0` and `1`.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make inversion deterministic.

cross_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined in [`self.processor`](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

clip_skip (`int`, *optional*) : Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that the output of the pre-final layer will be used for computing the prompt embeddings.

height (`int`, *optional*, defaults to `None`) : The height in preprocessed image. If `None`, will use the `get_default_height_width()` to get default height.

width (`int`, *optional*`, defaults to `None`) : The width in preprocessed. If `None`, will use get_default_height_width()` to get the default width.

resize_mode (`str`, *optional*, defaults to `default`) : The resize mode, can be one of `default` or `fill`. If `default`, will resize the image to fit within the specified width and height, and it may not maintaining the original aspect ratio. If `fill`, will resize the image to fit within the specified width and height, maintaining the aspect ratio, and then center the image within the dimensions, filling empty with data from image. If `crop`, will resize the image to fit within the specified width and height, maintaining the aspect ratio, and then center the image within the dimensions, cropping the excess. Note that resize_mode `fill` and `crop` are only supported for PIL image input.

crops_coords (`list[tuple[int, int, int, int]]`, *optional*, defaults to `None`) : The crop coordinates for each image in the batch. If `None`, will not crop the image.

**Returns:**

`[LEditsPPInversionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ledits_pp#diffusers.pipelines.LEditsPPInversionPipelineOutput)`

Output will contain the resized input image(s)
and respective VAE reconstruction(s).
#### disable_vae_slicing[[diffusers.LEditsPPPipelineStableDiffusion.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ledits_pp/pipeline_leditspp_stable_diffusion.py#L733)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### disable_vae_tiling[[diffusers.LEditsPPPipelineStableDiffusion.disable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ledits_pp/pipeline_leditspp_stable_diffusion.py#L760)

Disable tiled VAE decoding. If `enable_vae_tiling` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_vae_slicing[[diffusers.LEditsPPPipelineStableDiffusion.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ledits_pp/pipeline_leditspp_stable_diffusion.py#L720)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### enable_vae_tiling[[diffusers.LEditsPPPipelineStableDiffusion.enable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ledits_pp/pipeline_leditspp_stable_diffusion.py#L746)

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.
#### encode_prompt[[diffusers.LEditsPPPipelineStableDiffusion.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ledits_pp/pipeline_leditspp_stable_diffusion.py#L521)

Encodes the prompt into text encoder hidden states.

**Parameters:**

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

enable_edit_guidance (`bool`) : whether to perform any editing or reconstruct the input image instead

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

editing_prompt (`str` or `list[str]`, *optional*) : Editing prompt(s) to be encoded. If not defined, one has to pass `editing_prompt_embeds` instead.

editing_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

lora_scale (`float`, *optional*) : A LoRA scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

clip_skip (`int`, *optional*) : Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that the output of the pre-final layer will be used for computing the prompt embeddings.

## LEditsPPPipelineStableDiffusionXL[[diffusers.LEditsPPPipelineStableDiffusionXL]]
#### diffusers.LEditsPPPipelineStableDiffusionXL[[diffusers.LEditsPPPipelineStableDiffusionXL]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ledits_pp/pipeline_leditspp_stable_diffusion_xl.py#L273)

Pipeline for textual image editing using LEDits++ with Stable Diffusion XL.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline) and builds on the [StableDiffusionXLPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/stable_diffusion_xl#diffusers.StableDiffusionXLPipeline). Check the
superclass documentation for the generic methods implemented for all pipelines (downloading, saving, running on a
particular device, etc.).

In addition the pipeline inherits the following loading methods:
- *LoRA*: [LEditsPPPipelineStableDiffusionXL.load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionXLLoraLoaderMixin.load_lora_weights)
- *Ckpt*: [loaders.FromSingleFileMixin.from_single_file()](/docs/diffusers/v0.39.0/en/api/loaders/single_file#diffusers.loaders.FromSingleFileMixin.from_single_file)

as well as the following saving methods:
- *LoRA*: `loaders.StableDiffusionXLPipeline.save_lora_weights`

__call__diffusers.LEditsPPPipelineStableDiffusionXL.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ledits_pp/pipeline_leditspp_stable_diffusion_xl.py#L836[{"name": "denoising_end", "val": ": float | None = None"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "negative_prompt_2", "val": ": str | list[str] | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_pooled_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "ip_adapter_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "guidance_rescale", "val": ": float = 0.0"}, {"name": "crops_coords_top_left", "val": ": tuple = (0, 0)"}, {"name": "target_size", "val": ": tuple[int, int] | None = None"}, {"name": "editing_prompt", "val": ": str | list[str] | None = None"}, {"name": "editing_prompt_embeddings", "val": ": torch.Tensor | None = None"}, {"name": "editing_pooled_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "reverse_editing_direction", "val": ": bool | list[bool] | None = False"}, {"name": "edit_guidance_scale", "val": ": float | list[float] | None = 5"}, {"name": "edit_warmup_steps", "val": ": int | list[int] | None = 0"}, {"name": "edit_cooldown_steps", "val": ": int | list[int] | None = None"}, {"name": "edit_threshold", "val": ": float | list[float] | None = 0.9"}, {"name": "sem_guidance", "val": ": list[torch.Tensor] | None = None"}, {"name": "use_cross_attn_mask", "val": ": bool = False"}, {"name": "use_intersect_mask", "val": ": bool = False"}, {"name": "user_mask", "val": ": torch.Tensor | None = None"}, {"name": "attn_store_steps", "val": ": list[int] | None = []"}, {"name": "store_averaged_over_steps", "val": ": bool = True"}, {"name": "clip_skip", "val": ": int | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "**kwargs", "val": ""}]- **denoising_end** (`float`, *optional*) --
  When specified, determines the fraction (between 0.0 and 1.0) of the total denoising process to be
  completed before it is intentionally prematurely terminated. As a result, the returned sample will
  still retain a substantial amount of noise as determined by the discrete timesteps selected by the
  scheduler. The denoising_end parameter should ideally be utilized when this pipeline forms a part of a
  "Mixture of Denoisers" multi-pipeline setup, as elaborated in [**Refining the Image
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is
  less than `1`).
- **negative_prompt_2** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation to be sent to `tokenizer_2` and
  `text_encoder_2`. If not defined, `negative_prompt` is used in both text-encoders
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
  argument.
- **negative_pooled_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, pooled negative_prompt_embeds will be generated from `negative_prompt`
  input argument.
- **ip_adapter_image** -- (`PipelineImageInput`, *optional*):
  Optional image input to work with IP Adapters.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.stable_diffusion_xl.StableDiffusionXLPipelineOutput` instead
  of a plain tuple.
- **cross_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **guidance_rescale** (`float`, *optional*, defaults to 0.7) --
  Guidance rescale factor proposed by [Common Diffusion Noise Schedules and Sample Steps are
  Flawed](https://huggingface.co/papers/2305.08891) `guidance_scale` is defined as `φ` in equation 16. of
  [Common Diffusion Noise Schedules and Sample Steps are
  Flawed](https://huggingface.co/papers/2305.08891). Guidance rescale factor should fix overexposure when
  using zero terminal SNR.
- **crops_coords_top_left** (`tuple[int]`, *optional*, defaults to (0, 0)) --
  `crops_coords_top_left` can be used to generate an image that appears to be "cropped" from the position
  `crops_coords_top_left` downwards. Favorable, well-centered images are usually achieved by setting
  `crops_coords_top_left` to (0, 0). Part of SDXL's micro-conditioning as explained in section 2.2 of
  [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).
- **target_size** (`tuple[int]`, *optional*, defaults to (1024, 1024)) --
  For most cases, `target_size` should be set to the desired height and width of the generated image. If
  not specified it will default to `(width, height)`. Part of SDXL's micro-conditioning as explained in
  section 2.2 of [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).
- **editing_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. The image is reconstructed by setting
  `editing_prompt = None`. Guidance direction of prompt should be specified via
  `reverse_editing_direction`.
- **editing_prompt_embeddings** (`torch.Tensor`, *optional*) --
  Pre-generated edit text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting.
  If not provided, editing_prompt_embeddings will be generated from `editing_prompt` input argument.
- **editing_pooled_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated pooled edit text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, editing_prompt_embeddings will be generated from `editing_prompt` input
  argument.
- **reverse_editing_direction** (`bool` or `list[bool]`, *optional*, defaults to `False`) --
  Whether the corresponding prompt in `editing_prompt` should be increased or decreased.
- **edit_guidance_scale** (`float` or `list[float]`, *optional*, defaults to 5) --
  Guidance scale for guiding the image generation. If provided as list values should correspond to
  `editing_prompt`. `edit_guidance_scale` is defined as `s_e` of equation 12 of [LEDITS++
  Paper](https://huggingface.co/papers/2301.12247).
- **edit_warmup_steps** (`float` or `list[float]`, *optional*, defaults to 10) --
  Number of diffusion steps (for each prompt) for which guidance is not applied.
- **edit_cooldown_steps** (`float` or `list[float]`, *optional*, defaults to `None`) --
  Number of diffusion steps (for each prompt) after which guidance is no longer applied.
- **edit_threshold** (`float` or `list[float]`, *optional*, defaults to 0.9) --
  Masking threshold of guidance. Threshold should be proportional to the image region that is modified.
  'edit_threshold' is defined as 'λ' of equation 12 of [LEDITS++
  Paper](https://huggingface.co/papers/2301.12247).
- **sem_guidance** (`list[torch.Tensor]`, *optional*) --
  list of pre-generated guidance vectors to be applied at generation. Length of the list has to
  correspond to `num_inference_steps`.
- **use_cross_attn_mask** --
  Whether cross-attention masks are used. Cross-attention masks are always used when use_intersect_mask
  is set to true. Cross-attention masks are defined as 'M^1' of equation 12 of [LEDITS++
  paper](https://huggingface.co/papers/2311.16711).
- **use_intersect_mask** --
  Whether the masking term is calculated as intersection of cross-attention masks and masks derived from
  the noise estimate. Cross-attention mask are defined as 'M^1' and masks derived from the noise estimate
  are defined as 'M^2' of equation 12 of [LEDITS++ paper](https://huggingface.co/papers/2311.16711).
- **user_mask** --
  User-provided mask for even better control over the editing process. This is helpful when LEDITS++'s
  implicit masks do not meet user preferences.
- **attn_store_steps** --
  Steps for which the attention maps are stored in the AttentionStore. Just for visualization purposes.
- **store_averaged_over_steps** --
  Whether the attention maps for the 'attn_store_steps' are stored averaged over the diffusion steps. If
  False, attention maps for each step are stores separately. Just for visualization purposes.
- **clip_skip** (`int`, *optional*) --
  Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that
  the output of the pre-final layer will be used for computing the prompt embeddings.
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`list`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.0[LEditsPPDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ledits_pp#diffusers.pipelines.LEditsPPDiffusionPipelineOutput) or `tuple`[LEditsPPDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ledits_pp#diffusers.pipelines.LEditsPPDiffusionPipelineOutput) if `return_dict` is True, otherwise a `tuple. When
returning a tuple, the first element is a list with the generated images.

The call function to the pipeline for editing. The
[invert()](/docs/diffusers/v0.39.0/en/api/pipelines/ledits_pp#diffusers.LEditsPPPipelineStableDiffusionXL.invert) method has to be called beforehand. Edits
will always be performed for the last inverted image(s).

Examples:
```py
>>> import torch

>>> from diffusers import LEditsPPPipelineStableDiffusionXL
>>> from diffusers.utils import load_image

>>> pipe = LEditsPPPipelineStableDiffusionXL.from_pretrained(
...     "stabilityai/stable-diffusion-xl-base-1.0", variant="fp16", torch_dtype=torch.float16
... )
>>> pipe.enable_vae_tiling()
>>> pipe = pipe.to("cuda")

>>> img_url = "https://www.aiml.informatik.tu-darmstadt.de/people/mbrack/tennis.jpg"
>>> image = load_image(img_url).resize((1024, 1024))

>>> _ = pipe.invert(image=image, num_inversion_steps=50, skip=0.2)

>>> edited_image = pipe(
...     editing_prompt=["tennis ball", "tomato"],
...     reverse_editing_direction=[True, False],
...     edit_guidance_scale=[5.0, 10.0],
...     edit_threshold=[0.9, 0.85],
... ).images[0]
```

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder ([CLIPTextModel](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTextModel)) : Frozen text-encoder. Stable Diffusion XL uses the text portion of [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

text_encoder_2 ([CLIPTextModelWithProjection](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTextModelWithProjection)) : Second frozen text-encoder. Stable Diffusion XL uses the text and pool portion of [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModelWithProjection), specifically the [laion/CLIP-ViT-bigG-14-laion2B-39B-b160k](https://huggingface.co/laion/CLIP-ViT-bigG-14-laion2B-39B-b160k) variant.

tokenizer ([CLIPTokenizer](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTokenizer)) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer_2 ([CLIPTokenizer](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTokenizer)) : Second Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

unet ([UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the encoded image latents.

scheduler ([DPMSolverMultistepScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/multistep_dpm_solver#diffusers.DPMSolverMultistepScheduler) or [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler)) : A scheduler to be used in combination with `unet` to denoise the encoded image latens. Can be one of [DPMSolverMultistepScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/multistep_dpm_solver#diffusers.DPMSolverMultistepScheduler) or [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler). If any other scheduler is passed it will automatically be set to [DPMSolverMultistepScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/multistep_dpm_solver#diffusers.DPMSolverMultistepScheduler).

force_zeros_for_empty_prompt (`bool`, *optional*, defaults to `"True"`) : Whether the negative prompt embeddings shall be forced to always be set to 0. Also see the config of `stabilityai/stable-diffusion-xl-base-1-0`.

add_watermarker (`bool`, *optional*) : Whether to use the [invisible_watermark library](https://github.com/ShieldMnt/invisible-watermark/) to watermark output images. If not defined, it will default to True if the package is installed, otherwise no watermarker will be used.

**Returns:**

`[LEditsPPDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ledits_pp#diffusers.pipelines.LEditsPPDiffusionPipelineOutput) or `tuple``

[LEditsPPDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ledits_pp#diffusers.pipelines.LEditsPPDiffusionPipelineOutput) if `return_dict` is True, otherwise a `tuple. When
returning a tuple, the first element is a list with the generated images.
#### invert[[diffusers.LEditsPPPipelineStableDiffusionXL.invert]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ledits_pp/pipeline_leditspp_stable_diffusion_xl.py#L1469)

The function to the pipeline for image inversion as described by the [LEDITS++
Paper](https://huggingface.co/papers/2301.12247). If the scheduler is set to [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler) the
inversion proposed by [edit-friendly DPDM](https://huggingface.co/papers/2304.06140) will be performed instead.

**Parameters:**

image (`PipelineImageInput`) : Input for the image(s) that are to be edited. Multiple input images have to default to the same aspect ratio.

source_prompt (`str`, defaults to `""`) : Prompt describing the input image that will be used for guidance during inversion. Guidance is disabled if the `source_prompt` is `""`.

source_guidance_scale (`float`, defaults to `3.5`) : Strength of guidance during inversion.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

negative_prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `negative_prompt` is used in both text-encoders

num_inversion_steps (`int`, defaults to `50`) : Number of total performed inversion steps after discarding the initial `skip` steps.

skip (`float`, defaults to `0.15`) : Portion of initial steps that will be ignored for inversion and subsequent generation. Lower values will lead to stronger changes to the input image. `skip` has to be between `0` and `1`.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make inversion deterministic.

crops_coords_top_left (`tuple[int]`, *optional*, defaults to (0, 0)) : `crops_coords_top_left` can be used to generate an image that appears to be "cropped" from the position `crops_coords_top_left` downwards. Favorable, well-centered images are usually achieved by setting `crops_coords_top_left` to (0, 0). Part of SDXL's micro-conditioning as explained in section 2.2 of [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).

num_zero_noise_steps (`int`, defaults to `3`) : Number of final diffusion steps that will not renoise the current image. If no steps are set to zero SD-XL in combination with [DPMSolverMultistepScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/multistep_dpm_solver#diffusers.DPMSolverMultistepScheduler) will produce noise artifacts.

cross_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

**Returns:**

`[LEditsPPInversionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ledits_pp#diffusers.pipelines.LEditsPPInversionPipelineOutput)`

Output will contain the resized input image(s)
and respective VAE reconstruction(s).
#### disable_vae_slicing[[diffusers.LEditsPPPipelineStableDiffusionXL.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ledits_pp/pipeline_leditspp_stable_diffusion_xl.py#L771)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### disable_vae_tiling[[diffusers.LEditsPPPipelineStableDiffusionXL.disable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ledits_pp/pipeline_leditspp_stable_diffusion_xl.py#L798)

Disable tiled VAE decoding. If `enable_vae_tiling` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_vae_slicing[[diffusers.LEditsPPPipelineStableDiffusionXL.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ledits_pp/pipeline_leditspp_stable_diffusion_xl.py#L758)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### enable_vae_tiling[[diffusers.LEditsPPPipelineStableDiffusionXL.enable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ledits_pp/pipeline_leditspp_stable_diffusion_xl.py#L784)

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.
#### encode_prompt[[diffusers.LEditsPPPipelineStableDiffusionXL.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ledits_pp/pipeline_leditspp_stable_diffusion_xl.py#L400)

Encodes the prompt into text encoder hidden states.

**Parameters:**

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead.

negative_prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `negative_prompt` is used in both text-encoders

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

negative_pooled_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled negative_prompt_embeds will be generated from `negative_prompt` input argument.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

clip_skip (`int`, *optional*) : Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that the output of the pre-final layer will be used for computing the prompt embeddings.

enable_edit_guidance (`bool`) : Whether to guide towards an editing prompt or not.

editing_prompt (`str` or `list[str]`, *optional*) : Editing prompt(s) to be encoded. If not defined and 'enable_edit_guidance' is True, one has to pass `editing_prompt_embeds` instead.

editing_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated edit text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided and 'enable_edit_guidance' is True, editing_prompt_embeds will be generated from `editing_prompt` input argument.

editing_pooled_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated edit pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled editing_pooled_prompt_embeds will be generated from `editing_prompt` input argument.
#### get_guidance_scale_embedding[[diffusers.LEditsPPPipelineStableDiffusionXL.get_guidance_scale_embedding]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ledits_pp/pipeline_leditspp_stable_diffusion_xl.py#L697)

See https://github.com/google-research/vdm/blob/dc27b98a554f65cdc654b800da5aa1846545d41b/model_vdm.py#L298

**Parameters:**

w (`torch.Tensor`) : Generate embedding vectors with a specified guidance scale to subsequently enrich timestep embeddings.

embedding_dim (`int`, *optional*, defaults to 512) : Dimension of the embeddings to generate.

dtype (`torch.dtype`, *optional*, defaults to `torch.float32`) : Data type of the generated embeddings.

**Returns:**

``torch.Tensor``

Embedding vectors with shape `(len(w), embedding_dim)`.

## LEditsPPDiffusionPipelineOutput[[diffusers.pipelines.LEditsPPDiffusionPipelineOutput]]
#### diffusers.pipelines.LEditsPPDiffusionPipelineOutput[[diffusers.pipelines.LEditsPPDiffusionPipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ledits_pp/pipeline_output.py#L10)

Output class for LEdits++ Diffusion pipelines.

**Parameters:**

images (`list[PIL.Image.Image]` or `np.ndarray`) : list of denoised PIL images of length `batch_size` or NumPy array of shape `(batch_size, height, width, num_channels)`.

nsfw_content_detected (`list[bool]`) : list indicating whether the corresponding generated image contains “not-safe-for-work” (nsfw) content or `None` if safety checking could not be performed.

## LEditsPPInversionPipelineOutput[[diffusers.pipelines.LEditsPPInversionPipelineOutput]]
#### diffusers.pipelines.LEditsPPInversionPipelineOutput[[diffusers.pipelines.LEditsPPInversionPipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ledits_pp/pipeline_output.py#L28)

Output class for LEdits++ Diffusion pipelines.

**Parameters:**

input_images (`list[PIL.Image.Image]` or `np.ndarray`) : list of the cropped and resized input images as PIL images of length `batch_size` or NumPy array of shape ` (batch_size, height, width, num_channels)`.

vae_reconstruction_images (`list[PIL.Image.Image]` or `np.ndarray`) : list of VAE reconstruction of all input images as PIL images of length `batch_size` or NumPy array of shape ` (batch_size, height, width, num_channels)`.
