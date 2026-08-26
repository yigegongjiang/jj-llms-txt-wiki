# Flux2

  
  

Flux.2 is the recent series of image generation models from Black Forest Labs, preceded by the [Flux.1](./flux) series. It is an entirely new model with a new architecture and pre-training done from scratch!

Original model checkpoints for Flux can be found [here](https://huggingface.co/black-forest-labs). Original inference code can be found [here](https://github.com/black-forest-labs/flux2).

> [!TIP]
> Flux2 can be quite expensive to run on consumer hardware devices. However, you can perform a suite of optimizations to run it faster and in a more memory-friendly manner. Check out [this section](https://huggingface.co/blog/sd3#memory-optimizations-for-sd3) for more details. Additionally, Flux can benefit from quantization for memory efficiency with a trade-off in inference latency. Refer to [this blog post](https://huggingface.co/blog/quanto-diffusers) to learn more.
>
> [Caching](../../optimization/cache) may also speed up inference by storing and reusing intermediate outputs.

## Caption upsampling

Flux.2 can potentially generate better better outputs with better prompts. We can "upsample"
an input prompt by setting the `caption_upsample_temperature` argument in the pipeline call arguments.
The [official implementation](https://github.com/black-forest-labs/flux2/blob/5a5d316b1b42f6b59a8c9194b77c8256be848432/src/flux2/text_encoder.py#L140) recommends this value to be 0.15.

## Flux2Pipeline[[diffusers.Flux2Pipeline]]

#### diffusers.Flux2Pipeline[[diffusers.Flux2Pipeline]]

```python
diffusers.Flux2Pipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKLFlux2, text_encoder: Mistral3ForConditionalGeneration, tokenizer: AutoProcessor, transformer: Flux2Transformer2DModel)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux2/pipeline_flux2.py#L251)

**Parameters:**

transformer ([Flux2Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/flux2_transformer#diffusers.Flux2Transformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae (`AutoencoderKLFlux2`) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`Mistral3ForConditionalGeneration`) : [Mistral3ForConditionalGeneration](https://huggingface.co/docs/transformers/en/model_doc/mistral3#transformers.Mistral3ForConditionalGeneration)

tokenizer (`AutoProcessor`) : Tokenizer of class [PixtralProcessor](https://huggingface.co/docs/transformers/en/model_doc/pixtral#transformers.PixtralProcessor).

The Flux2 pipeline for text-to-image generation.

Reference: [https://bfl.ai/blog/flux-2](https://bfl.ai/blog/flux-2)

#### __call__[[diffusers.Flux2Pipeline.__call__]]

```python
__call__(image: PIL.Image.Image | list[PIL.Image.Image] | None = None, prompt: str | list[str] = None, height: int | None = None, width: int | None = None, num_inference_steps: int = 50, sigmas: list[float] | None = None, guidance_scale: float | None = 4.0, num_images_per_prompt: int = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512, text_encoder_out_layers: tuple = (10, 20, 30), caption_upsample_temperature: float = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux2/pipeline_flux2.py#L745)

**Parameters:**

image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) : `Image`, numpy array or tensor representing an image batch to be used as the starting point. For both numpy array and pytorch tensor, the expected value range is between `[0, 1]` If it's a tensor or a list or tensors, the expected shape should be `(B, C, H, W)` or `(C, H, W)`. If it is a numpy array or a list of arrays, the expected shape should be `(B, H, W, C)` or `(H, W, C)` It can also accept image latents as `image`, but if passing latents directly it is not encoded again.

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

guidance_scale (`float`, *optional*, defaults to 1.0) : Embedded guiddance scale is enabled by setting `guidance_scale` > 1. Higher `guidance_scale` encourages a model to generate images more aligned with `prompt` at the expense of lower image quality.  Guidance-distilled models approximates true classifer-free guidance for `guidance_scale` > 1. Refer to the [paper](https://huggingface.co/papers/2210.03142) to learn more.

height (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The height in pixels of the generated image. This is set to 1024 by default for the best results.

width (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The width in pixels of the generated image. This is set to 1024 by default for the best results.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`list[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.qwenimage.QwenImagePipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`List`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int` defaults to 512) : Maximum sequence length to use with the `prompt`.

text_encoder_out_layers (`tuple[int]`) : Layer indices to use in the `text_encoder` to derive the final prompt embeddings.

caption_upsample_temperature (`float`) : When specified, we will try to perform caption upsampling for potentially improved outputs. We recommend setting it to 0.15 if caption upsampling is to be performed.

**Returns:** `~pipelines.flux2.Flux2PipelineOutput` or `tuple`

`~pipelines.flux2.Flux2PipelineOutput` if
`return_dict` is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the
generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import Flux2Pipeline

>>> pipe = Flux2Pipeline.from_pretrained("black-forest-labs/FLUX.2-dev", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")
>>> prompt = "A cat holding a sign that says hello world"
>>> # Depending on the variant being used, the pipeline call will slightly vary.
>>> # Refer to the pipeline documentation for more details.
>>> image = pipe(prompt, num_inference_steps=50, guidance_scale=2.5).images[0]
>>> image.save("flux.png")
```

## Flux2KleinPipeline[[diffusers.Flux2KleinPipeline]]

#### diffusers.Flux2KleinPipeline[[diffusers.Flux2KleinPipeline]]

```python
diffusers.Flux2KleinPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKLFlux2, text_encoder: Qwen3ForCausalLM, tokenizer: Qwen2Tokenizer, transformer: Flux2Transformer2DModel, is_distilled: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux2/pipeline_flux2_klein.py#L155)

**Parameters:**

transformer ([Flux2Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/flux2_transformer#diffusers.Flux2Transformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae (`AutoencoderKLFlux2`) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`Qwen3ForCausalLM`) : [Qwen3ForCausalLM](https://huggingface.co/docs/transformers/en/model_doc/qwen3#transformers.Qwen3ForCausalLM)

tokenizer (`Qwen2TokenizerFast`) : Tokenizer of class [Qwen2TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/qwen2#transformers.Qwen2TokenizerFast).

The Flux2 Klein pipeline for text-to-image generation.

Reference:
[https://bfl.ai/blog/flux2-klein-towards-interactive-visual-intelligence](https://bfl.ai/blog/flux2-klein-towards-interactive-visual-intelligence)

#### __call__[[diffusers.Flux2KleinPipeline.__call__]]

```python
__call__(image: list[PIL.Image.Image] | PIL.Image.Image | None = None, prompt: str | list[str] = None, height: int | None = None, width: int | None = None, num_inference_steps: int = 50, sigmas: list[float] | None = None, guidance_scale: float = 4.0, num_images_per_prompt: int = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: str | list[str] | None = None, output_type: str = 'pil', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int, dict], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512, text_encoder_out_layers: tuple = (9, 18, 27))
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux2/pipeline_flux2_klein.py#L612)

**Parameters:**

image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `List[torch.Tensor]`, `List[PIL.Image.Image]`, or `List[np.ndarray]`) : `Image`, numpy array or tensor representing an image batch to be used as the starting point. For both numpy array and pytorch tensor, the expected value range is between `[0, 1]` If it's a tensor or a list or tensors, the expected shape should be `(B, C, H, W)` or `(C, H, W)`. If it is a numpy array or a list of arrays, the expected shape should be `(B, H, W, C)` or `(H, W, C)` It can also accept image latents as `image`, but if passing latents directly it is not encoded again.

prompt (`str` or `List[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality. For step-wise distilled models, `guidance_scale` is ignored.

height (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The height in pixels of the generated image. This is set to 1024 by default for the best results.

width (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The width in pixels of the generated image. This is set to 1024 by default for the best results.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`List[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `List[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Note that "" is used as the negative prompt in this pipeline. If not provided, will be generated from "".

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.qwenimage.QwenImagePipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`List`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int` defaults to 512) : Maximum sequence length to use with the `prompt`.

text_encoder_out_layers (`tuple[int]`) : Layer indices to use in the `text_encoder` to derive the final prompt embeddings.

**Returns:** `~pipelines.flux2.Flux2PipelineOutput` or `tuple`

`~pipelines.flux2.Flux2PipelineOutput` if
`return_dict` is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the
generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import Flux2KleinPipeline

>>> pipe = Flux2KleinPipeline.from_pretrained(
...     "black-forest-labs/FLUX.2-klein-base-9B", torch_dtype=torch.bfloat16
... )
>>> pipe.to("cuda")
>>> prompt = "A cat holding a sign that says hello world"
>>> # Depending on the variant being used, the pipeline call will slightly vary.
>>> # Refer to the pipeline documentation for more details.
>>> image = pipe(prompt, num_inference_steps=50, guidance_scale=4.0).images[0]
>>> image.save("flux2_output.png")
```

## Flux2KleinKVPipeline[[diffusers.Flux2KleinKVPipeline]]

#### diffusers.Flux2KleinKVPipeline[[diffusers.Flux2KleinKVPipeline]]

```python
diffusers.Flux2KleinKVPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKLFlux2, text_encoder: Qwen3ForCausalLM, tokenizer: Qwen2Tokenizer, transformer: Flux2Transformer2DModel, is_distilled: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux2/pipeline_flux2_klein_kv.py#L155)

**Parameters:**

transformer ([Flux2Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/flux2_transformer#diffusers.Flux2Transformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae (`AutoencoderKLFlux2`) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`Qwen3ForCausalLM`) : [Qwen3ForCausalLM](https://huggingface.co/docs/transformers/en/model_doc/qwen3#transformers.Qwen3ForCausalLM)

tokenizer (`Qwen2TokenizerFast`) : Tokenizer of class [Qwen2TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/qwen2#transformers.Qwen2TokenizerFast).

The Flux2 Klein KV pipeline for text-to-image generation with KV-cached reference image conditioning.

On the first denoising step, reference image tokens are included in the forward pass and their attention K/V
projections are cached. On subsequent steps, the cached K/V are reused without recomputing, providing faster
inference when using reference images.

Reference:
[https://bfl.ai/blog/flux2-klein-towards-interactive-visual-intelligence](https://bfl.ai/blog/flux2-klein-towards-interactive-visual-intelligence)

#### __call__[[diffusers.Flux2KleinKVPipeline.__call__]]

```python
__call__(image: list[PIL.Image.Image] | PIL.Image.Image | None = None, prompt: str | list[str] = None, height: int | None = None, width: int | None = None, num_inference_steps: int = 4, sigmas: list[float] | None = None, num_images_per_prompt: int = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, output_type: str = 'pil', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int, dict], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512, text_encoder_out_layers: tuple = (9, 18, 27))
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux2/pipeline_flux2_klein_kv.py#L609)

**Parameters:**

image (`PIL.Image.Image` or `List[PIL.Image.Image]`, *optional*) : Reference image(s) for conditioning. On the first denoising step, reference tokens are included in the forward pass and their attention K/V are cached. On subsequent steps, the cached K/V are reused without recomputing.

prompt (`str` or `List[str]`, *optional*) : The prompt or prompts to guide the image generation.

height (`int`, *optional*) : The height in pixels of the generated image.

width (`int`, *optional*) : The width in pixels of the generated image.

num_inference_steps (`int`, *optional*, defaults to 4) : The number of denoising steps.

sigmas (`List[float]`, *optional*) : Custom sigmas for the denoising schedule.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `List[torch.Generator]`, *optional*) : Generator(s) for deterministic generation.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings.

output_type (`str`, *optional*, defaults to `"pil"`) : Output format: `"pil"` or `"np"`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a `Flux2PipelineOutput` or a plain tuple.

attention_kwargs (`dict`, *optional*) : Extra kwargs passed to attention processors.

callback_on_step_end (`Callable`, *optional*) : Callback function called at the end of each denoising step.

callback_on_step_end_tensor_inputs (`List`, *optional*) : Tensor inputs for the callback function.

max_sequence_length (`int`, defaults to 512) : Maximum sequence length for the prompt.

text_encoder_out_layers (`tuple[int]`) : Layer indices for text encoder hidden state extraction.

**Returns:**

`~pipelines.flux2.Flux2PipelineOutput` or `tuple`.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from PIL import Image
>>> from diffusers import Flux2KleinKVPipeline

>>> pipe = Flux2KleinKVPipeline.from_pretrained(
...     "black-forest-labs/FLUX.2-klein-9b-kv", torch_dtype=torch.bfloat16
... )
>>> pipe.to("cuda")
>>> ref_image = Image.open("reference.png")
>>> image = pipe("A cat dressed like a wizard", image=ref_image, num_inference_steps=4).images[0]
>>> image.save("flux2_kv_output.png")
```
