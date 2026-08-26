# Bria 3.2

Bria 3.2 is the next-generation commercial-ready text-to-image model. With just 4 billion parameters, it provides exceptional aesthetics and text rendering, evaluated to provide on par results to leading open-source models, and outperforming other licensed models.
In addition to being built entirely on licensed data, 3.2 provides several advantages for enterprise and commercial use:

- Efficient Compute - the model is X3 smaller than the equivalent models in the market (4B parameters vs 12B parameters other open source models)
- Architecture Consistency: Same architecture as 3.1—ideal for users looking to upgrade without disruption.
- Fine-tuning Speedup: 2x faster fine-tuning on L40S and A100.

Original model checkpoints for Bria 3.2 can be found [here](https://huggingface.co/briaai/BRIA-3.2).
Github repo for Bria 3.2 can be found [here](https://github.com/Bria-AI/BRIA-3.2).

If you want to learn more about the Bria platform, and get free traril access, please visit [bria.ai](https://bria.ai).

## Usage

_As the model is gated, before using it with diffusers you first need to go to the [Bria 3.2 Hugging Face page](https://huggingface.co/briaai/BRIA-3.2), fill in the form and accept the gate. Once you are in, you need to login so that your system knows you’ve accepted the gate._

Use the command below to log in:

```bash
hf auth login
```

## BriaPipeline[[diffusers.BriaPipeline]]

#### diffusers.BriaPipeline[[diffusers.BriaPipeline]]

```python
diffusers.BriaPipeline(transformer: BriaTransformer2DModel, scheduler: diffusers.schedulers.scheduling_flow_match_euler_discrete.FlowMatchEulerDiscreteScheduler | diffusers.schedulers.scheduling_utils.KarrasDiffusionSchedulers, vae: AutoencoderKL, text_encoder: T5EncoderModel, tokenizer: T5Tokenizer, image_encoder: CLIPVisionModelWithProjection = None, feature_extractor: CLIPImageProcessorPil = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/bria/pipeline_bria.py#L89)

**Parameters:**

transformer ([BriaTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/bria_transformer#diffusers.BriaTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`T5EncoderModel`) : Frozen text-encoder. Bria uses [T5](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5EncoderModel), specifically the [t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`T5TokenizerFast`) : Tokenizer of class [T5Tokenizer](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5Tokenizer).

Based on FluxPipeline with several changes:
- no pooled embeddings
- We use zero padding for prompts
- No guidance embedding since this is not a distilled version

#### __call__[[diffusers.BriaPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, height: int | None = None, width: int | None = None, num_inference_steps: int = 30, timesteps: list = None, guidance_scale: float = 5, negative_prompt: str | list[str] | None = None, num_images_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.FloatTensor] = None, prompt_embeds: typing.Optional[torch.FloatTensor] = None, negative_prompt_embeds: typing.Optional[torch.FloatTensor] = None, output_type: str | None = 'pil', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 128, clip_value: None | float = None, normalize: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/bria/pipeline_bria.py#L448)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

height (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The height in pixels of the generated image. This is set to 1024 by default for the best results.

width (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The width in pixels of the generated image. This is set to 1024 by default for the best results.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

timesteps (`list[int]`, *optional*) : Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used. Must be in descending order.

guidance_scale (`float`, *optional*, defaults to 5.0) --

**Returns:** `~pipelines.bria.BriaPipelineOutput` or `tuple`

`~pipelines.bria.BriaPipelineOutput` if
`return_dict` is True, otherwise a `tuple`. When returning a tuple, the first element is a list
with the generated images.

Function invoked when calling the pipeline for generation.

<<<<<<< HEAD
Guidance scale as defined in [Classifier-Free Diffusion
Guidance](https://arxiv.org/abs/2207.12598). `guidance_scale` is defined as `w` of equation 2.
of [Imagen Paper](https://arxiv.org/pdf/2205.11487.pdf). Guidance scale is enabled by setting
`guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely
linked to the text `prompt`, usually at the expense of lower image quality.
negative_prompt (`str` or `list[str]`, *optional*):
=======
Guidance scale as defined in [Classifier-Free Diffusion
Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of
equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is
enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images
that are closely linked to the text `prompt`, usually at the expense of lower image quality.
negative_prompt (`str` or `list[str]`, *optional*):
>>>>>>> main
The prompt or prompts not to guide the image generation. If not defined, one has to pass
`negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if
`guidance_scale` is less than `1`).
num_images_per_prompt (`int`, *optional*, defaults to 1):
The number of images to generate per prompt.
generator (`torch.Generator` or `list[torch.Generator]`, *optional*):
One or a list of [torch
generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make
generation deterministic.
latents (`torch.FloatTensor`, *optional*):
Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for
image generation. Can be used to tweak the same generation with different prompts. If not
provided, a latents tensor will be generated by sampling using the supplied random `generator`.
prompt_embeds (`torch.FloatTensor`, *optional*):
Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
weighting. If not provided, text embeddings will be generated from `prompt` input argument.
negative_prompt_embeds (`torch.FloatTensor`, *optional*):
Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt`
input argument.
output_type (`str`, *optional*, defaults to `"pil"`):
The output format of the generate image. Choose between
[PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
return_dict (`bool`, *optional*, defaults to `True`):
Whether or not to return a `~pipelines.bria.BriaPipelineOutput` instead of a plain tuple.
attention_kwargs (`dict`, *optional*):
A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined
under `self.processor` in
[diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
callback_on_step_end (`Callable`, *optional*):
A function that calls at the end of each denoising steps during the inference. The function is
called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int,
timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as
specified by `callback_on_step_end_tensor_inputs`.
callback_on_step_end_tensor_inputs (`list`, *optional*):
The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the
list will be passed as `callback_kwargs` argument. You will only be able to include variables
listed in the `._callback_tensor_inputs` attribute of your pipeline class.
max_sequence_length (`int` defaults to 256): Maximum sequence length to use with the `prompt`.
clip_value (`float`, *optional*):
If set, the predicted noise is clipped to the range `[-clip_value, clip_value]` at each
denoising step.
normalize (`bool`, *optional*, defaults to `False`):
Whether to normalize the predicted noise at each denoising step.

Examples:
```py
>>> import torch
>>> from diffusers import BriaPipeline

>>> pipe = BriaPipeline.from_pretrained("briaai/BRIA-3.2", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")
# BRIA's T5 text encoder is sensitive to precision. We need to cast it to bfloat16 and keep the final layer in float32.

>>> pipe.text_encoder = pipe.text_encoder.to(dtype=torch.bfloat16)
>>> for block in pipe.text_encoder.encoder.block:
...     block.layer[-1].DenseReluDense.wo.to(dtype=torch.float32)
# BRIA's VAE is not supported in mixed precision, so we use float32.

>>> if pipe.vae.config.shift_factor == 0:
...     pipe.vae.to(dtype=torch.float32)

>>> prompt = "Photorealistic food photography of a stack of fluffy pancakes on a white plate, with maple syrup being poured over them. On top of the pancakes are the words 'BRIA 3.2' in bold, yellow, 3D letters. The background is dark and out of focus."
>>> image = pipe(prompt).images[0]
>>> image.save("bria.png")
```

#### encode_prompt[[diffusers.BriaPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], device: typing.Optional[torch.device] = None, num_images_per_prompt: int = 1, do_classifier_free_guidance: bool = True, negative_prompt: str | list[str] | None = None, prompt_embeds: typing.Optional[torch.FloatTensor] = None, negative_prompt_embeds: typing.Optional[torch.FloatTensor] = None, max_sequence_length: int = 128, lora_scale: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/bria/pipeline_bria.py#L146)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

do_classifier_free_guidance (`bool`) : whether to use classifier free guidance or not

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.
