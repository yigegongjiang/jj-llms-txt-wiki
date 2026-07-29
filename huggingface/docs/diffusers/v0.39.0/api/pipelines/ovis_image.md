# Ovis-Image

![concepts](https://github.com/AIDC-AI/Ovis-Image/blob/main/docs/imgs/ovis_image_case.png)

Ovis-Image is a 7B text-to-image model specifically optimized for high-quality text rendering, designed to operate efficiently under stringent computational constraints.

[Ovis-Image Technical Report](https://arxiv.org/abs/2511.22982) from Alibaba Group, by Guo-Hua Wang, Liangfu Cao, Tianyu Cui, Minghao Fu, Xiaohao Chen, Pengxin Zhan, Jianshan Zhao, Lan Li, Bowen Fu, Jiaqi Liu, Qing-Guo Chen.

The abstract from the paper is:

*We introduce Ovis-Image, a 7B text-to-image model specifically optimized for high-quality text rendering, designed to operate efficiently under stringent computational constraints. Built upon our previous Ovis-U1 framework, Ovis-Image integrates a diffusion-based visual decoder with the stronger Ovis 2.5 multimodal backbone, leveraging a text-centric training pipeline that combines large-scale pre-training with carefully tailored post-training refinements. Despite its compact architecture, Ovis-Image achieves text rendering performance on par with significantly larger open models such as Qwen-Image and approaches closed-source systems like Seedream and GPT4o. Crucially, the model remains deployable on a single high-end GPU with moderate memory, narrowing the gap between frontier-level text rendering and practical deployment. Our results indicate that combining a strong multimodal backbone with a carefully designed, text-focused training recipe is sufficient to achieve reliable bilingual text rendering without resorting to oversized or proprietary models.*

**Highlights**: 

*   **Strong text rendering at a compact 7B scale**: Ovis-Image is a 7B text-to-image model that delivers text rendering quality comparable to much larger 20B-class systems such as Qwen-Image and competitive with leading closed-source models like GPT4o in text-centric scenarios, while remaining small enough to run on widely accessible hardware.
*   **High fidelity on text-heavy, layout-sensitive prompts**: The model excels on prompts that demand tight alignment between linguistic content and rendered typography (e.g., posters, banners, logos, UI mockups, infographics), producing legible, correctly spelled, and semantically consistent text across diverse fonts, sizes, and aspect ratios without compromising overall visual quality.
*   **Efficiency and deployability**: With its 7B parameter budget and streamlined architecture, Ovis-Image fits on a single high-end GPU with moderate memory, supports low-latency interactive use, and scales to batch production serving, bringing near–frontier text rendering to applications where tens-of-billions–parameter models are impractical.

This pipeline was contributed by Ovis-Image Team. The original codebase can be found [here](https://github.com/AIDC-AI/Ovis-Image).

Available models:

| Model | Recommended dtype |
|:-----:|:-----------------:|
| [`AIDC-AI/Ovis-Image-7B`](https://huggingface.co/AIDC-AI/Ovis-Image-7B) | `torch.bfloat16` |

Refer to [this](https://huggingface.co/collections/AIDC-AI/ovis-image) collection for more information.

## OvisImagePipeline[[diffusers.OvisImagePipeline]]

#### diffusers.OvisImagePipeline[[diffusers.OvisImagePipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ovis_image/pipeline_ovis_image.py#L129)

The Ovis-Image pipeline for text-to-image generation.

Reference: https://github.com/AIDC-AI/Ovis-Image

__call__diffusers.OvisImagePipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ovis_image/pipeline_ovis_image.py#L457[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str | list[str] = ''"}, {"name": "guidance_scale", "val": ": float = 5.0"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "sigmas", "val": ": list[float] | None = None"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.FloatTensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.FloatTensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.FloatTensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "joint_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 256"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is
  not greater than `1`).
- **guidance_scale** (`float`, *optional*, defaults to 1.0) --
  True classifier-free guidance (guidance scale) is enabled when `guidance_scale` > 1 and
  `negative_prompt` is provided.
- **height** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The height in pixels of the generated image. This is set to 1024 by default for the best results.
- **width** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The width in pixels of the generated image. This is set to 1024 by default for the best results.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **sigmas** (`list[float]`, *optional*) --
  Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in
  their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed
  will be used.
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.FloatTensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **negative_prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
  argument.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.flux.FluxPipelineOutput` instead of a plain tuple.
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
- **max_sequence_length** (`int` defaults to 512) -- Maximum sequence length to use with the `prompt`.0[OvisImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ovis_image#diffusers.pipelines.ovis_image.OvisImagePipelineOutput) or `tuple`[OvisImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ovis_image#diffusers.pipelines.ovis_image.OvisImagePipelineOutput) if `return_dict` is True, otherwise a `tuple`. When
returning a tuple, the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import OvisImagePipeline

>>> pipe = OvisImagePipeline.from_pretrained("AIDC-AI/Ovis-Image-7B", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")
>>> prompt = 'A creative 3D artistic render where the text "OVIS-IMAGE" is written in a bold, expressive handwritten brush style using thick, wet oil paint. The paint is a mix of vibrant rainbow colors (red, blue, yellow) swirling together like toothpaste or impasto art. You can see the ridges of the brush bristles and the glossy, wet texture of the paint. The background is a clean artist's canvas. Dynamic lighting creates soft shadows behind the floating paint strokes. Colorful, expressive, tactile texture, 4k detail.'
>>> image = pipe(prompt, negative_prompt="", num_inference_steps=50, guidance_scale=5.0).images[0]
>>> image.save("ovis_image.png")
```

**Parameters:**

transformer ([OvisImageTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/ovisimage_transformer2d#diffusers.OvisImageTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`Qwen3Model`) : Text encoder of class [Qwen3Model](https://huggingface.co/docs/transformers/en/model_doc/qwen3#transformers.Qwen3Model).

tokenizer (`Qwen2TokenizerFast`) : Tokenizer of class [Qwen2TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/qwen2#transformers.Qwen2TokenizerFast).

**Returns:**

`[OvisImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ovis_image#diffusers.pipelines.ovis_image.OvisImagePipelineOutput) or `tuple``

[OvisImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ovis_image#diffusers.pipelines.ovis_image.OvisImagePipelineOutput) if `return_dict` is True, otherwise a `tuple`. When
returning a tuple, the first element is a list with the generated images.
#### encode_prompt[[diffusers.OvisImagePipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ovis_image/pipeline_ovis_image.py#L269)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. Used only when `do_classifier_free_guidance` is `True`. If not defined, an empty string is used.

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : Whether to also encode the `negative_prompt` for classifier-free guidance.

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

max_sequence_length (`int`, *optional*, defaults to 256) : Maximum sequence length to use for the `prompt`.

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. If not provided, they are generated from `negative_prompt`.

## OvisImagePipelineOutput[[diffusers.pipelines.ovis_image.OvisImagePipelineOutput]]

#### diffusers.pipelines.ovis_image.OvisImagePipelineOutput[[diffusers.pipelines.ovis_image.OvisImagePipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ovis_image/pipeline_output.py#L24)

Output class for Ovis-Image pipelines.

**Parameters:**

images (`list[PIL.Image.Image]` or `np.ndarray`) : List of denoised PIL images of length `batch_size` or numpy array of shape `(batch_size, height, width, num_channels)`. PIL images or numpy array present the denoised images of the diffusion pipeline.
