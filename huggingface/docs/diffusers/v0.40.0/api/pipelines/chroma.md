# Chroma

  
  

Chroma is a text to image generation model based on Flux.

Original model checkpoints for Chroma can be found here:
* High-resolution finetune: [lodestones/Chroma1-HD](https://huggingface.co/lodestones/Chroma1-HD)
* Base model: [lodestones/Chroma1-Base](https://huggingface.co/lodestones/Chroma1-Base)
* Original repo with progress checkpoints: [lodestones/Chroma](https://huggingface.co/lodestones/Chroma) (loading this repo with `from_pretrained` will load a Diffusers-compatible version of the `unlocked-v37` checkpoint)

> [!TIP]
> Chroma can use all the same optimizations as Flux.

## Inference

```python
import torch
from diffusers import ChromaPipeline

pipe = ChromaPipeline.from_pretrained("lodestones/Chroma1-HD", dtype=torch.bfloat16)
pipe.enable_model_cpu_offload()

prompt = [
    "A high-fashion close-up portrait of a blonde woman in clear sunglasses. The image uses a bold teal and red color split for dramatic lighting. The background is a simple teal-green. The photo is sharp and well-composed, and is designed for viewing with anaglyph 3D glasses for optimal effect. It looks professionally done."
]
negative_prompt =  ["low quality, ugly, unfinished, out of focus, deformed, disfigure, blurry, smudged, restricted palette, flat colors"]

image = pipe(
    prompt=prompt,
    negative_prompt=negative_prompt,
    generator=torch.Generator("cpu").manual_seed(433),
    num_inference_steps=40,
    guidance_scale=3.0,
    num_images_per_prompt=1,
).images[0]
image.save("chroma.png")
```

## Loading from a single file

To use updated model checkpoints that are not in the Diffusers format, you can use the `ChromaTransformer2DModel` class to load the model from a single file in the original format. This is also useful when trying to load finetunes or quantized versions of the models that have been published by the community.

The following example demonstrates how to run Chroma from a single file.

Then run the following example

```python
import torch
from diffusers import ChromaTransformer2DModel, ChromaPipeline

model_id = "lodestones/Chroma1-HD"
dtype = torch.bfloat16

transformer = ChromaTransformer2DModel.from_single_file("https://huggingface.co/lodestones/Chroma1-HD/blob/main/Chroma1-HD.safetensors", dtype=dtype)

pipe = ChromaPipeline.from_pretrained(model_id, transformer=transformer, dtype=dtype)
pipe.enable_model_cpu_offload()

prompt = [
    "A high-fashion close-up portrait of a blonde woman in clear sunglasses. The image uses a bold teal and red color split for dramatic lighting. The background is a simple teal-green. The photo is sharp and well-composed, and is designed for viewing with anaglyph 3D glasses for optimal effect. It looks professionally done."
]
negative_prompt =  ["low quality, ugly, unfinished, out of focus, deformed, disfigure, blurry, smudged, restricted palette, flat colors"]

image = pipe(
    prompt=prompt,
    negative_prompt=negative_prompt,
    generator=torch.Generator("cpu").manual_seed(433),
    num_inference_steps=40,
    guidance_scale=3.0,
).images[0]

image.save("chroma-single-file.png")
```

## ChromaPipeline[[diffusers.ChromaPipeline]]

#### diffusers.ChromaPipeline[[diffusers.ChromaPipeline]]

```python
diffusers.ChromaPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKL, text_encoder: T5EncoderModel, tokenizer: T5Tokenizer, transformer: ChromaTransformer2DModel, image_encoder: CLIPVisionModelWithProjection = None, feature_extractor: CLIPImageProcessorPil = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/chroma/pipeline_chroma.py#L150)

**Parameters:**

transformer ([ChromaTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/chroma_transformer#diffusers.ChromaTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representation

text_encoder (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`T5TokenizerFast`) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

The Chroma pipeline for text-to-image generation.

Reference: https://huggingface.co/lodestones/Chroma1-HD/

#### __call__[[diffusers.ChromaPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, negative_prompt: str | list[str] = None, height: int | None = None, width: int | None = None, num_inference_steps: int = 35, sigmas: list[float] | None = None, guidance_scale: float = 5.0, num_images_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, ip_adapter_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, ip_adapter_image_embeds: list[torch.Tensor] | None = None, negative_ip_adapter_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, negative_ip_adapter_image_embeds: list[torch.Tensor] | None = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, joint_attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/chroma/pipeline_chroma.py#L587)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is not greater than `1`).

height (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The height in pixels of the generated image. This is set to 1024 by default for the best results.

width (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The width in pixels of the generated image. This is set to 1024 by default for the best results.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`list[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

guidance_scale (`float`, *optional*, defaults to 3.5) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

ip_adapter_image : (`PipelineImageInput`, *optional*): Optional image input to work with IP Adapters.

ip_adapter_image_embeds (`list[torch.Tensor]`, *optional*) : Pre-generated image embeddings for IP-Adapter. It should be a list of length same as number of IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. If not provided, embeddings are computed from the `ip_adapter_image` input argument.

negative_ip_adapter_image : (`PipelineImageInput`, *optional*): Optional image input to work with IP Adapters.

negative_ip_adapter_image_embeds (`list[torch.Tensor]`, *optional*) : Pre-generated image embeddings for IP-Adapter. It should be a list of length same as number of IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. If not provided, embeddings are computed from the `ip_adapter_image` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

prompt_attention_mask (torch.Tensor, *optional*) : Attention mask for the prompt embeddings. Used to mask out padding tokens in the prompt sequence. Chroma requires a single padding token remain unmasked. Please refer to https://huggingface.co/lodestones/Chroma#tldr-masking-t5-padding-tokens-enhanced-fidelity-and-increased-stability-during-training

negative_prompt_attention_mask (torch.Tensor, *optional*) : Attention mask for the negative prompt embeddings. Used to mask out padding tokens in the negative prompt sequence. Chroma requires a single padding token remain unmasked. PLease refer to https://huggingface.co/lodestones/Chroma#tldr-masking-t5-padding-tokens-enhanced-fidelity-and-increased-stability-during-training

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.flux.ChromaPipelineOutput` instead of a plain tuple.

joint_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int` defaults to 512) : Maximum sequence length to use with the `prompt`.

**Returns:** `~pipelines.chroma.ChromaPipelineOutput` or `tuple`

`~pipelines.chroma.ChromaPipelineOutput` if
`return_dict` is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the
generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import ChromaPipeline

>>> model_id = "lodestones/Chroma1-HD"
>>> ckpt_path = "https://huggingface.co/lodestones/Chroma1-HD/blob/main/Chroma1-HD.safetensors"
>>> transformer = ChromaTransformer2DModel.from_single_file(ckpt_path, torch_dtype=torch.bfloat16)
>>> pipe = ChromaPipeline.from_pretrained(
...     model_id,
...     transformer=transformer,
...     torch_dtype=torch.bfloat16,
... )
>>> pipe.enable_model_cpu_offload()
>>> prompt = [
...     "A high-fashion close-up portrait of a blonde woman in clear sunglasses. The image uses a bold teal and red color split for dramatic lighting. The background is a simple teal-green. The photo is sharp and well-composed, and is designed for viewing with anaglyph 3D glasses for optimal effect. It looks professionally done."
... ]
>>> negative_prompt = [
...     "low quality, ugly, unfinished, out of focus, deformed, disfigure, blurry, smudged, restricted palette, flat colors"
... ]
>>> image = pipe(prompt, negative_prompt=negative_prompt).images[0]
>>> image.save("chroma.png")
```

#### encode_prompt[[diffusers.ChromaPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] = None, device: typing.Optional[torch.device] = None, num_images_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, do_classifier_free_guidance: bool = True, max_sequence_length: int = 512, lora_scale: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/chroma/pipeline_chroma.py#L264)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

## ChromaImg2ImgPipeline[[diffusers.ChromaImg2ImgPipeline]]

#### diffusers.ChromaImg2ImgPipeline[[diffusers.ChromaImg2ImgPipeline]]

```python
diffusers.ChromaImg2ImgPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKL, text_encoder: T5EncoderModel, tokenizer: T5Tokenizer, transformer: ChromaTransformer2DModel, image_encoder: CLIPVisionModelWithProjection = None, feature_extractor: CLIPImageProcessorPil = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/chroma/pipeline_chroma_img2img.py#L162)

**Parameters:**

transformer ([ChromaTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/chroma_transformer#diffusers.ChromaTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representation

text_encoder (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`T5TokenizerFast`) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

The Chroma pipeline for image-to-image generation.

Reference: https://huggingface.co/lodestones/Chroma1-HD/

#### __call__[[diffusers.ChromaImg2ImgPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, negative_prompt: str | list[str] = None, image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, height: int | None = None, width: int | None = None, num_inference_steps: int = 35, sigmas: list[float] | None = None, guidance_scale: float = 5.0, strength: float = 0.9, num_images_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, ip_adapter_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, ip_adapter_image_embeds: list[torch.Tensor] | None = None, negative_ip_adapter_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, negative_ip_adapter_image_embeds: list[torch.Tensor] | None = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, joint_attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/chroma/pipeline_chroma_img2img.py#L646)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is not greater than `1`).

image (`PipelineImageInput`) : The image input for the pipeline.

height (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The height in pixels of the generated image. This is set to 1024 by default for the best results.

width (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The width in pixels of the generated image. This is set to 1024 by default for the best results.

num_inference_steps (`int`, *optional*, defaults to 35) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`list[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

guidance_scale (`float`, *optional*, defaults to 3.5) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

strength (`float, *optional*, defaults to 0.9) : Conceptually, indicates how much to transform the reference image. Must be between 0 and 1. image will be used as a starting point, adding more noise to it the larger the strength. The number of denoising steps depends on the amount of noise initially added. When strength is 1, added noise will be maximum and the denoising process will run for the full number of iterations specified in num_inference_steps. A value of 1, therefore, essentially ignores image.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

ip_adapter_image : (`PipelineImageInput`, *optional*): Optional image input to work with IP Adapters.

ip_adapter_image_embeds (`list[torch.Tensor]`, *optional*) : Pre-generated image embeddings for IP-Adapter. It should be a list of length same as number of IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. If not provided, embeddings are computed from the `ip_adapter_image` input argument.

negative_ip_adapter_image : (`PipelineImageInput`, *optional*): Optional image input to work with IP Adapters.

negative_ip_adapter_image_embeds (`list[torch.Tensor]`, *optional*) : Pre-generated image embeddings for IP-Adapter. It should be a list of length same as number of IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. If not provided, embeddings are computed from the `ip_adapter_image` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

prompt_attention_mask (torch.Tensor, *optional*) : Attention mask for the prompt embeddings. Used to mask out padding tokens in the prompt sequence. Chroma requires a single padding token remain unmasked. Please refer to https://huggingface.co/lodestones/Chroma#tldr-masking-t5-padding-tokens-enhanced-fidelity-and-increased-stability-during-training

negative_prompt_attention_mask (torch.Tensor, *optional*) : Attention mask for the negative prompt embeddings. Used to mask out padding tokens in the negative prompt sequence. Chroma requires a single padding token remain unmasked. PLease refer to https://huggingface.co/lodestones/Chroma#tldr-masking-t5-padding-tokens-enhanced-fidelity-and-increased-stability-during-training

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.flux.ChromaPipelineOutput` instead of a plain tuple.

joint_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int` defaults to 512) : Maximum sequence length to use with the `prompt`.

**Returns:** `~pipelines.chroma.ChromaPipelineOutput` or `tuple`

`~pipelines.chroma.ChromaPipelineOutput` if
`return_dict` is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the
generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import ChromaTransformer2DModel, ChromaImg2ImgPipeline

>>> model_id = "lodestones/Chroma1-HD"
>>> ckpt_path = "https://huggingface.co/lodestones/Chroma1-HD/blob/main/Chroma1-HD.safetensors"
>>> pipe = ChromaImg2ImgPipeline.from_pretrained(
...     model_id,
...     transformer=transformer,
...     torch_dtype=torch.bfloat16,
... )
>>> pipe.enable_model_cpu_offload()
>>> init_image = load_image(
...     "https://raw.githubusercontent.com/CompVis/stable-diffusion/main/assets/stable-samples/img2img/sketch-mountains-input.jpg"
... )
>>> prompt = "a scenic fastasy landscape with a river and mountains in the background, vibrant colors, detailed, high resolution"
>>> negative_prompt = "low quality, ugly, unfinished, out of focus, deformed, disfigure, blurry, smudged, restricted palette, flat colors"
>>> image = pipe(prompt, image=init_image, negative_prompt=negative_prompt).images[0]
>>> image.save("chroma-img2img.png")
```

#### encode_prompt[[diffusers.ChromaImg2ImgPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] = None, device: typing.Optional[torch.device] = None, num_images_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, do_classifier_free_guidance: bool = True, max_sequence_length: int = 512, lora_scale: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/chroma/pipeline_chroma_img2img.py#L291)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

## ChromaInpaintPipeline[[diffusers.ChromaInpaintPipeline]]

#### diffusers.ChromaInpaintPipeline[[diffusers.ChromaInpaintPipeline]]

```python
diffusers.ChromaInpaintPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKL, text_encoder: T5EncoderModel, tokenizer: T5Tokenizer, transformer: ChromaTransformer2DModel, image_encoder: CLIPVisionModelWithProjection = None, feature_extractor: CLIPImageProcessorPil = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/chroma/pipeline_chroma_inpainting.py#L169)

**Parameters:**

transformer ([ChromaTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/chroma_transformer#diffusers.ChromaTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

text_encoder_2 (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer_2 (`T5TokenizerFast`) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

The Flux pipeline for image inpainting.

Reference: https://blackforestlabs.ai/announcing-black-forest-labs/

#### __call__[[diffusers.ChromaInpaintPipeline.__call__]]

```python
__call__(prompt: typing.Union[str, typing.List[str]] = None, negative_prompt: typing.Union[str, typing.List[str]] = None, true_cfg_scale: float = 1.0, image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, mask_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, masked_image_latents: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, height: int | None = None, width: int | None = None, padding_mask_crop: int | None = None, strength: float = 0.6, num_inference_steps: int = 28, sigmas: list[float] | None = None, guidance_scale: float = 7.0, num_images_per_prompt: typing.Optional[int] = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.FloatTensor] = None, prompt_embeds: typing.Optional[torch.FloatTensor] = None, ip_adapter_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, ip_adapter_image_embeds: typing.Optional[typing.List[torch.Tensor]] = None, negative_ip_adapter_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, negative_ip_adapter_image_embeds: typing.Optional[typing.List[torch.Tensor]] = None, negative_prompt_embeds: typing.Optional[torch.FloatTensor] = None, output_type: str = 'pil', return_dict: bool = True, joint_attention_kwargs: typing.Optional[typing.Dict[str, typing.Any]] = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int, typing.Dict], NoneType]] = None, callback_on_step_end_tensor_inputs: typing.List[str] = ['latents'], max_sequence_length: int = 256, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/chroma/pipeline_chroma_inpainting.py#L764)

**Parameters:**

prompt (`str` or `List[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `List[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is not greater than `1`).

true_cfg_scale (`float`, *optional*, defaults to 1.0) : True classifier-free guidance (guidance scale) is enabled when `true_cfg_scale` > 1 and `negative_prompt` is provided.

image (`PipelineImageInput`) : The image input for the pipeline.

mask_image (`PipelineImageInput`) : `Image`, numpy array or tensor representing an image batch to mask `image`. White pixels in the mask are repainted while black pixels are preserved.

masked_image_latents (`torch.Tensor`, *optional*) : Pre-encoded latent representation of the masked image. If not provided, it will be computed from `mask_image` and `image`.

height (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The height in pixels of the generated image. This is set to 1024 by default for the best results.

width (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The width in pixels of the generated image. This is set to 1024 by default for the best results.

padding_mask_crop (`int`, *optional*, defaults to `None`) : The size of margin in the crop to be applied to the image and masking. If `None`, no crop is applied to image and mask_image. If `padding_mask_crop` is not `None`, it will first find a rectangular region with the same aspect ratio of the image and contains all masked area, and then expand that area based on `padding_mask_crop`. The image and mask_image will then be cropped based on the expanded area before resizing to the original image size for inpainting.

num_inference_steps (`int`, *optional*, defaults to 35) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`List[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

guidance_scale (`float`, *optional*, defaults to 3.5) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

strength (`float, *optional*, defaults to 0.9) : Conceptually, indicates how much to transform the reference image. Must be between 0 and 1. image will be used as a starting point, adding more noise to it the larger the strength. The number of denoising steps depends on the amount of noise initially added. When strength is 1, added noise will be maximum and the denoising process will run for the full number of iterations specified in num_inference_steps. A value of 1, therefore, essentially ignores image.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `List[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

ip_adapter_image : (`PipelineImageInput`, *optional*): Optional image input to work with IP Adapters.

ip_adapter_image_embeds (`List[torch.Tensor]`, *optional*) : Pre-generated image embeddings for IP-Adapter. It should be a list of length same as number of IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. If not provided, embeddings are computed from the `ip_adapter_image` input argument.

negative_ip_adapter_image : (`PipelineImageInput`, *optional*): Optional image input to work with IP Adapters.

negative_ip_adapter_image_embeds (`List[torch.Tensor]`, *optional*) : Pre-generated image embeddings for IP-Adapter. It should be a list of length same as number of IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. If not provided, embeddings are computed from the `ip_adapter_image` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

prompt_attention_mask (torch.Tensor, *optional*) : Attention mask for the prompt embeddings. Used to mask out padding tokens in the prompt sequence. Chroma requires a single padding token remain unmasked. Please refer to https://huggingface.co/lodestones/Chroma#tldr-masking-t5-padding-tokens-enhanced-fidelity-and-increased-stability-during-training

negative_prompt_attention_mask (torch.Tensor, *optional*) : Attention mask for the negative prompt embeddings. Used to mask out padding tokens in the negative prompt sequence. Chroma requires a single padding token remain unmasked. PLease refer to https://huggingface.co/lodestones/Chroma#tldr-masking-t5-padding-tokens-enhanced-fidelity-and-increased-stability-during-training

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.chroma.ChromaPipelineOutput` instead of a plain tuple.

joint_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`List`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int` defaults to 512) : Maximum sequence length to use with the `prompt`.

**Returns:** `~pipelines.chroma.ChromaPipelineOutput` or `tuple`

`~pipelines.chroma.ChromaPipelineOutput` if
`return_dict` is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the
generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import ChromaInpaintPipeline
>>> from diffusers.utils import load_image

>>> pipe = ChromaInpaintPipeline.from_pretrained("lodestones/Chroma1-HD", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")
>>> prompt = "Face of a yellow cat, high resolution, sitting on a park bench"
>>> img_url = "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
>>> mask_url = "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"
>>> source = load_image(img_url)
>>> mask = load_image(mask_url)
>>> image = pipe(prompt=prompt, image=source, mask_image=mask).images[0]
>>> image.save("chroma_inpainting.png")
```

#### encode_prompt[[diffusers.ChromaInpaintPipeline.encode_prompt]]

```python
encode_prompt(prompt: typing.Union[str, typing.List[str]], negative_prompt: typing.Union[str, typing.List[str], NoneType] = None, device: typing.Optional[torch.device] = None, num_images_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, do_classifier_free_guidance: bool = True, max_sequence_length: int = 256, lora_scale: bool | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/chroma/pipeline_chroma_inpainting.py#L293)

**Parameters:**

prompt (`str` or `List[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `List[str]`, *optional*) : The prompt not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.
