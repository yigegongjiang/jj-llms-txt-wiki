# T2I-Adapter

[T2I-Adapter: Learning Adapters to Dig out More Controllable Ability for Text-to-Image Diffusion Models](https://huggingface.co/papers/2302.08453) by Chong Mou, Xintao Wang, Liangbin Xie, Jian Zhang, Zhongang Qi, Ying Shan, Xiaohu Qie.

Using the pretrained models we can provide control images (for example, a depth map) to control Stable Diffusion text-to-image generation so that it follows the structure of the depth image and fills in the details.

The abstract of the paper is the following:

*The incredible generative ability of large-scale text-to-image (T2I) models has demonstrated strong power of learning complex structures and meaningful semantics. However, relying solely on text prompts cannot fully take advantage of the knowledge learned by the model, especially when flexible and accurate controlling (e.g., color and structure) is needed. In this paper, we aim to ``dig out" the capabilities that T2I models have implicitly learned, and then explicitly use them to control the generation more granularly. Specifically, we propose to learn simple and lightweight T2I-Adapters to align internal knowledge in T2I models with external control signals, while freezing the original large T2I models. In this way, we can train various adapters according to different conditions, achieving rich control and editing effects in the color and structure of the generation results. Further, the proposed T2I-Adapters have attractive properties of practical value, such as composability and generalization ability. Extensive experiments demonstrate that our T2I-Adapter has promising generation quality and a wide range of applications.*

This model was contributed by the community contributor [HimariO](https://github.com/HimariO) ❤️ .

## StableDiffusionAdapterPipeline[[diffusers.StableDiffusionAdapterPipeline]]

#### diffusers.StableDiffusionAdapterPipeline[[diffusers.StableDiffusionAdapterPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/t2i_adapter/pipeline_stable_diffusion_adapter.py#L191)

Pipeline for text-to-image generation using Stable Diffusion augmented with T2I-Adapter
https://huggingface.co/papers/2302.08453

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

__call__diffusers.StableDiffusionAdapterPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/t2i_adapter/pipeline_stable_diffusion_adapter.py#L689[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "image", "val": ": torch.Tensor | PIL.Image.Image | list[PIL.Image.Image] = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "timesteps", "val": ": list = None"}, {"name": "sigmas", "val": ": list = None"}, {"name": "guidance_scale", "val": ": float = 7.5"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "callback", "val": ": typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None"}, {"name": "callback_steps", "val": ": int = 1"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "adapter_conditioning_scale", "val": ": float | list[float] = 1.0"}, {"name": "clip_skip", "val": ": int | None = None"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **image** (`torch.Tensor`, `PIL.Image.Image`, `list[torch.Tensor]` or `list[PIL.Image.Image]` or `list[list[PIL.Image.Image]]`) --
  The Adapter input condition. Adapter uses this input condition to generate guidance to Unet. If the
  type is specified as `torch.Tensor`, it is passed to Adapter as is. PIL.Image.Image` can also be
  accepted as an image. The control image is automatically resized to fit the output image.
- **height** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The height in pixels of the generated image.
- **width** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The width in pixels of the generated image.
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
- **guidance_scale** (`float`, *optional*, defaults to 7.5) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2.
  of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting
  `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to
  the text `prompt`, usually at the expense of lower image quality.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, one has to pass
  `negative_prompt_embeds`. instead. If not defined, one has to pass `negative_prompt_embeds`. instead.
  Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).
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
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.stable_diffusion.StableDiffusionAdapterPipelineOutput` instead
  of a plain tuple.
- **callback** (`Callable`, *optional*) --
  A function that will be called every `callback_steps` steps during inference. The function will be
  called with the following arguments: `callback(step: int, timestep: int, latents: torch.Tensor)`.
- **callback_steps** (`int`, *optional*, defaults to 1) --
  The frequency at which the `callback` function will be called. If not specified, the callback will be
  called at every step.
- **cross_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttnProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **adapter_conditioning_scale** (`float` or `list[float]`, *optional*, defaults to 1.0) --
  The outputs of the adapter are multiplied by `adapter_conditioning_scale` before they are added to the
  residual in the original unet. If multiple adapters are specified in init, you can set the
  corresponding scale as a list.
- **clip_skip** (`int`, *optional*) --
  Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that
  the output of the pre-final layer will be used for computing the prompt embeddings.0`~pipelines.stable_diffusion.StableDiffusionAdapterPipelineOutput` or `tuple``~pipelines.stable_diffusion.StableDiffusionAdapterPipelineOutput` if `return_dict` is True, otherwise a
`tuple. When returning a tuple, the first element is a list with the generated images, and the second
element is a list of `bool`s denoting whether the corresponding generated image likely represents
"not-safe-for-work" (nsfw) content, according to the `safety_checker`.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> from PIL import Image
>>> from diffusers.utils import load_image
>>> import torch
>>> from diffusers import StableDiffusionAdapterPipeline, T2IAdapter

>>> image = load_image(
...     "https://huggingface.co/datasets/diffusers/docs-images/resolve/main/t2i-adapter/color_ref.png"
... )

>>> color_palette = image.resize((8, 8))
>>> color_palette = color_palette.resize((512, 512), resample=Image.Resampling.NEAREST)

>>> adapter = T2IAdapter.from_pretrained("TencentARC/t2iadapter_color_sd14v1", torch_dtype=torch.float16)
>>> pipe = StableDiffusionAdapterPipeline.from_pretrained(
...     "CompVis/stable-diffusion-v1-4",
...     adapter=adapter,
...     torch_dtype=torch.float16,
... )

>>> pipe.to("cuda")

>>> out_image = pipe(
...     "At night, glowing cubes in front of the beach",
...     image=color_palette,
... ).images[0]
```

**Parameters:**

adapter (`T2IAdapter` or `MultiAdapter` or `list[T2IAdapter]`) : Provides additional conditioning to the unet during the denoising process. If you set multiple Adapter as a list, the outputs from each Adapter are added together to create one combined additional conditioning.

adapter_weights (`list[float]`, *optional*, defaults to None) : list of floats representing the weight which will be multiply to each adapter's output before adding them together.

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : Frozen text-encoder. Stable Diffusion uses the text portion of [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

unet ([UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the encoded image latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `unet` to denoise the encoded image latents. Can be one of [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), [LMSDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lms_discrete#diffusers.LMSDiscreteScheduler), or [PNDMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/pndm#diffusers.PNDMScheduler).

safety_checker (`StableDiffusionSafetyChecker`) : Classification module that estimates whether generated images could be considered offensive or harmful. Please, refer to the [model card](https://huggingface.co/stable-diffusion-v1-5/stable-diffusion-v1-5) for details.

feature_extractor (`CLIPImageProcessor`) : Model that extracts features from generated images to be used as inputs for the `safety_checker`.

**Returns:**

``~pipelines.stable_diffusion.StableDiffusionAdapterPipelineOutput` or `tuple``

`~pipelines.stable_diffusion.StableDiffusionAdapterPipelineOutput` if `return_dict` is True, otherwise a
`tuple. When returning a tuple, the first element is a list with the generated images, and the second
element is a list of `bool`s denoting whether the corresponding generated image likely represents
"not-safe-for-work" (nsfw) content, according to the `safety_checker`.
#### enable_attention_slicing[[diffusers.StableDiffusionAdapterPipeline.enable_attention_slicing]]

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
#### disable_attention_slicing[[diffusers.StableDiffusionAdapterPipeline.disable_attention_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2084)

Disable sliced attention computation. If `enable_attention_slicing` was previously called, attention is
computed in one step.
#### enable_vae_slicing[[diffusers.StableDiffusionAdapterPipeline.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2267)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### disable_vae_slicing[[diffusers.StableDiffusionAdapterPipeline.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2280)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_xformers_memory_efficient_attention[[diffusers.StableDiffusionAdapterPipeline.enable_xformers_memory_efficient_attention]]

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
#### disable_xformers_memory_efficient_attention[[diffusers.StableDiffusionAdapterPipeline.disable_xformers_memory_efficient_attention]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2023)

Disable memory efficient attention from [xFormers](https://facebookresearch.github.io/xformers/).
#### encode_prompt[[diffusers.StableDiffusionAdapterPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/t2i_adapter/pipeline_stable_diffusion_adapter.py#L311)

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
#### get_guidance_scale_embedding[[diffusers.StableDiffusionAdapterPipeline.get_guidance_scale_embedding]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/t2i_adapter/pipeline_stable_diffusion_adapter.py#L648)

See https://github.com/google-research/vdm/blob/dc27b98a554f65cdc654b800da5aa1846545d41b/model_vdm.py#L298

**Parameters:**

w (`torch.Tensor`) : Generate embedding vectors with a specified guidance scale to subsequently enrich timestep embeddings.

embedding_dim (`int`, *optional*, defaults to 512) : Dimension of the embeddings to generate.

dtype (`torch.dtype`, *optional*, defaults to `torch.float32`) : Data type of the generated embeddings.

**Returns:**

``torch.Tensor``

Embedding vectors with shape `(len(w), embedding_dim)`.

## StableDiffusionXLAdapterPipeline[[diffusers.StableDiffusionXLAdapterPipeline]]

#### diffusers.StableDiffusionXLAdapterPipeline[[diffusers.StableDiffusionXLAdapterPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/t2i_adapter/pipeline_stable_diffusion_xl_adapter.py#L213)

Pipeline for text-to-image generation using Stable Diffusion augmented with T2I-Adapter
https://huggingface.co/papers/2302.08453

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

The pipeline also inherits the following loading methods:
- [load_textual_inversion()](/docs/diffusers/v0.39.0/en/api/loaders/textual_inversion#diffusers.loaders.TextualInversionLoaderMixin.load_textual_inversion) for loading textual inversion embeddings
- [from_single_file()](/docs/diffusers/v0.39.0/en/api/loaders/single_file#diffusers.loaders.FromSingleFileMixin.from_single_file) for loading `.ckpt` files
- [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionXLLoraLoaderMixin.load_lora_weights) for loading LoRA weights
- [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionXLLoraLoaderMixin.save_lora_weights) for saving LoRA weights
- [load_ip_adapter()](/docs/diffusers/v0.39.0/en/api/loaders/ip_adapter#diffusers.loaders.IPAdapterMixin.load_ip_adapter) for loading IP Adapters

__call__diffusers.StableDiffusionXLAdapterPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/t2i_adapter/pipeline_stable_diffusion_xl_adapter.py#L856[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "prompt_2", "val": ": str | list[str] | None = None"}, {"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "timesteps", "val": ": list = None"}, {"name": "sigmas", "val": ": list = None"}, {"name": "denoising_end", "val": ": float | None = None"}, {"name": "guidance_scale", "val": ": float = 5.0"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "negative_prompt_2", "val": ": str | list[str] | None = None"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "pooled_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_pooled_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "ip_adapter_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "ip_adapter_image_embeds", "val": ": list[torch.Tensor] | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "callback", "val": ": typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None"}, {"name": "callback_steps", "val": ": int = 1"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "guidance_rescale", "val": ": float = 0.0"}, {"name": "original_size", "val": ": tuple[int, int] | None = None"}, {"name": "crops_coords_top_left", "val": ": tuple = (0, 0)"}, {"name": "target_size", "val": ": tuple[int, int] | None = None"}, {"name": "negative_original_size", "val": ": tuple[int, int] | None = None"}, {"name": "negative_crops_coords_top_left", "val": ": tuple = (0, 0)"}, {"name": "negative_target_size", "val": ": tuple[int, int] | None = None"}, {"name": "adapter_conditioning_scale", "val": ": float | list[float] = 1.0"}, {"name": "adapter_conditioning_factor", "val": ": float = 1.0"}, {"name": "clip_skip", "val": ": int | None = None"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **prompt_2** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is
  used in both text-encoders
- **image** (`torch.Tensor`, `PIL.Image.Image`, `list[torch.Tensor]` or `list[PIL.Image.Image]` or `list[list[PIL.Image.Image]]`) --
  The Adapter input condition. Adapter uses this input condition to generate guidance to Unet. If the
  type is specified as `torch.Tensor`, it is passed to Adapter as is. PIL.Image.Image` can also be
  accepted as an image. The control image is automatically resized to fit the output image.
- **height** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The height in pixels of the generated image. Anything below 512 pixels won't work well for
  [stabilityai/stable-diffusion-xl-base-1.0](https://huggingface.co/stabilityai/stable-diffusion-xl-base-1.0)
  and checkpoints that are not specifically fine-tuned on low resolutions.
- **width** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
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
  Whether or not to return a `~pipelines.stable_diffusion_xl.StableDiffusionAdapterPipelineOutput`
  instead of a plain tuple.
- **callback** (`Callable`, *optional*) --
  A function that will be called every `callback_steps` steps during inference. The function will be
  called with the following arguments: `callback(step: int, timestep: int, latents: torch.Tensor)`.
- **callback_steps** (`int`, *optional*, defaults to 1) --
  The frequency at which the `callback` function will be called. If not specified, the callback will be
  called at every step.
- **cross_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **guidance_rescale** (`float`, *optional*, defaults to 0.0) --
  Guidance rescale factor proposed by [Common Diffusion Noise Schedules and Sample Steps are
  Flawed](https://huggingface.co/papers/2305.08891) `guidance_scale` is defined as `φ` in equation 16. of
  [Common Diffusion Noise Schedules and Sample Steps are
  Flawed](https://huggingface.co/papers/2305.08891). Guidance rescale factor should fix overexposure when
  using zero terminal SNR.
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
- **adapter_conditioning_scale** (`float` or `list[float]`, *optional*, defaults to 1.0) --
  The outputs of the adapter are multiplied by `adapter_conditioning_scale` before they are added to the
  residual in the original unet. If multiple adapters are specified in init, you can set the
  corresponding scale as a list.
- **adapter_conditioning_factor** (`float`, *optional*, defaults to 1.0) --
  The fraction of timesteps for which adapter should be applied. If `adapter_conditioning_factor` is
  `0.0`, adapter is not applied at all. If `adapter_conditioning_factor` is `1.0`, adapter is applied for
  all timesteps. If `adapter_conditioning_factor` is `0.5`, adapter is applied for half of the timesteps.
- **clip_skip** (`int`, *optional*) --
  Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that
  the output of the pre-final layer will be used for computing the prompt embeddings.0`~pipelines.stable_diffusion.StableDiffusionAdapterPipelineOutput` or `tuple``~pipelines.stable_diffusion.StableDiffusionAdapterPipelineOutput` if `return_dict` is True, otherwise a
`tuple`. When returning a tuple, the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import T2IAdapter, StableDiffusionXLAdapterPipeline, DDPMScheduler
>>> from diffusers.utils import load_image

>>> sketch_image = load_image("https://huggingface.co/Adapter/t2iadapter/resolve/main/sketch.png").convert("L")

>>> model_id = "stabilityai/stable-diffusion-xl-base-1.0"

>>> adapter = T2IAdapter.from_pretrained(
...     "Adapter/t2iadapter",
...     subfolder="sketch_sdxl_1.0",
...     torch_dtype=torch.float16,
...     adapter_type="full_adapter_xl",
... )
>>> scheduler = DDPMScheduler.from_pretrained(model_id, subfolder="scheduler")

>>> pipe = StableDiffusionXLAdapterPipeline.from_pretrained(
...     model_id, adapter=adapter, torch_dtype=torch.float16, variant="fp16", scheduler=scheduler
... ).to("cuda")

>>> generator = torch.manual_seed(42)
>>> sketch_image_out = pipe(
...     prompt="a photo of a dog in real world, high quality",
...     negative_prompt="extra digit, fewer digits, cropped, worst quality, low quality",
...     image=sketch_image,
...     generator=generator,
...     guidance_scale=7.5,
... ).images[0]
```

**Parameters:**

adapter (`T2IAdapter` or `MultiAdapter` or `list[T2IAdapter]`) : Provides additional conditioning to the unet during the denoising process. If you set multiple Adapter as a list, the outputs from each Adapter are added together to create one combined additional conditioning.

adapter_weights (`list[float]`, *optional*, defaults to None) : list of floats representing the weight which will be multiply to each adapter's output before adding them together.

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : Frozen text-encoder. Stable Diffusion uses the text portion of [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

unet ([UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the encoded image latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `unet` to denoise the encoded image latents. Can be one of [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), [LMSDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lms_discrete#diffusers.LMSDiscreteScheduler), or [PNDMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/pndm#diffusers.PNDMScheduler).

safety_checker (`StableDiffusionSafetyChecker`) : Classification module that estimates whether generated images could be considered offensive or harmful. Please, refer to the [model card](https://huggingface.co/stable-diffusion-v1-5/stable-diffusion-v1-5) for details.

feature_extractor (`CLIPImageProcessor`) : Model that extracts features from generated images to be used as inputs for the `safety_checker`.

**Returns:**

``~pipelines.stable_diffusion.StableDiffusionAdapterPipelineOutput` or `tuple``

`~pipelines.stable_diffusion.StableDiffusionAdapterPipelineOutput` if `return_dict` is True, otherwise a
`tuple`. When returning a tuple, the first element is a list with the generated images.
#### enable_attention_slicing[[diffusers.StableDiffusionXLAdapterPipeline.enable_attention_slicing]]

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
#### disable_attention_slicing[[diffusers.StableDiffusionXLAdapterPipeline.disable_attention_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2084)

Disable sliced attention computation. If `enable_attention_slicing` was previously called, attention is
computed in one step.
#### enable_vae_slicing[[diffusers.StableDiffusionXLAdapterPipeline.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2267)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### disable_vae_slicing[[diffusers.StableDiffusionXLAdapterPipeline.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2280)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_xformers_memory_efficient_attention[[diffusers.StableDiffusionXLAdapterPipeline.enable_xformers_memory_efficient_attention]]

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
#### disable_xformers_memory_efficient_attention[[diffusers.StableDiffusionXLAdapterPipeline.disable_xformers_memory_efficient_attention]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2023)

Disable memory efficient attention from [xFormers](https://facebookresearch.github.io/xformers/).
#### encode_prompt[[diffusers.StableDiffusionXLAdapterPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/t2i_adapter/pipeline_stable_diffusion_xl_adapter.py#L311)

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
#### get_guidance_scale_embedding[[diffusers.StableDiffusionXLAdapterPipeline.get_guidance_scale_embedding]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/t2i_adapter/pipeline_stable_diffusion_xl_adapter.py#L815)

See https://github.com/google-research/vdm/blob/dc27b98a554f65cdc654b800da5aa1846545d41b/model_vdm.py#L298

**Parameters:**

w (`torch.Tensor`) : Generate embedding vectors with a specified guidance scale to subsequently enrich timestep embeddings.

embedding_dim (`int`, *optional*, defaults to 512) : Dimension of the embeddings to generate.

dtype (`torch.dtype`, *optional*, defaults to `torch.float32`) : Data type of the generated embeddings.

**Returns:**

``torch.Tensor``

Embedding vectors with shape `(len(w), embedding_dim)`.
