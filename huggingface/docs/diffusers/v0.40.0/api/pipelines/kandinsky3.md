# Kandinsky 3

  

Kandinsky 3 is created by [Vladimir Arkhipkin](https://github.com/oriBetelgeuse),[Anastasia Maltseva](https://github.com/NastyaMittseva),[Igor Pavlov](https://github.com/boomb0om),[Andrei Filatov](https://github.com/anvilarth),[Arseniy Shakhmatov](https://github.com/cene555),[Andrey Kuznetsov](https://github.com/kuznetsoffandrey),[Denis Dimitrov](https://github.com/denndimitrov), [Zein Shaheen](https://github.com/zeinsh)

The description from it's GitHub page:

*Kandinsky 3.0 is an open-source text-to-image diffusion model built upon the Kandinsky2-x model family. In comparison to its predecessors, enhancements have been made to the text understanding and visual quality of the model, achieved by increasing the size of the text encoder and Diffusion U-Net models, respectively.*

Its architecture includes 3 main components:
1. [FLAN-UL2](https://huggingface.co/google/flan-ul2), which is an encoder decoder model based on the T5 architecture.
2. New U-Net architecture featuring BigGAN-deep blocks doubles depth while maintaining the same number of parameters.
3. Sber-MoVQGAN is a decoder proven to have superior results in image restoration.

The original codebase can be found at [ai-forever/Kandinsky-3](https://github.com/ai-forever/Kandinsky-3).

> [!TIP]
> Check out the [Kandinsky Community](https://huggingface.co/kandinsky-community) organization on the Hub for the official model checkpoints for tasks like text-to-image, image-to-image, and inpainting.

> [!TIP]
> Make sure to check out the schedulers [guide](../../using-diffusers/schedulers) to learn how to explore the tradeoff between scheduler speed and quality, and see the [reuse components across pipelines](../../using-diffusers/loading#reuse-a-pipeline) section to learn how to efficiently load the same components into multiple pipelines.

## Kandinsky3Pipeline[[diffusers.Kandinsky3Pipeline]]

#### diffusers.Kandinsky3Pipeline[[diffusers.Kandinsky3Pipeline]]

```python
diffusers.Kandinsky3Pipeline(tokenizer: T5Tokenizer, text_encoder: T5EncoderModel, unet: Kandinsky3UNet, scheduler: DDPMScheduler, movq: VQModel)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky3/pipeline_kandinsky3.py#L59)

#### __call__[[diffusers.Kandinsky3Pipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, num_inference_steps: int = 25, guidance_scale: float = 3.0, negative_prompt: str | list[str] | None = None, num_images_per_prompt: int | None = 1, height: int | None = 1024, width: int | None = 1024, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, negative_attention_mask: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, latents = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky3/pipeline_kandinsky3.py#L334)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

num_inference_steps (`int`, *optional*, defaults to 25) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, *optional*, defaults to 3.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

height (`int`, *optional*, defaults to self.unet.config.sample_size) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to self.unet.config.sample_size) : The width in pixels of the generated image.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

attention_mask (`torch.Tensor`, *optional*) : Pre-generated attention mask. Must provide if passing `prompt_embeds` directly.

negative_attention_mask (`torch.Tensor`, *optional*) : Pre-generated negative attention mask. Must provide if passing `negative_prompt_embeds` directly.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.stable_diffusion.IFPipelineOutput` instead of a plain tuple.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> from diffusers import AutoPipelineForText2Image
>>> import torch

>>> pipe = AutoPipelineForText2Image.from_pretrained(
...     "kandinsky-community/kandinsky-3", variant="fp16", torch_dtype=torch.float16
... )
>>> pipe.enable_model_cpu_offload()

>>> prompt = "A photograph of the inside of a subway train. There are raccoons sitting on the seats. One of them is reading a newspaper. The window shows the city in the background."

>>> generator = torch.Generator(device="cpu").manual_seed(0)
>>> image = pipe(prompt, num_inference_steps=25, generator=generator).images[0]
```

#### encode_prompt[[diffusers.Kandinsky3Pipeline.encode_prompt]]

```python
encode_prompt(prompt, do_classifier_free_guidance = True, num_images_per_prompt = 1, device = None, negative_prompt = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, _cut_context = False, attention_mask: typing.Optional[torch.Tensor] = None, negative_attention_mask: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky3/pipeline_kandinsky3.py#L91)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

device : (`torch.device`, *optional*): torch device to place the resulting embeddings on

num_images_per_prompt (`int`, *optional*, defaults to 1) : number of images that should be generated per prompt

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : whether to use classifier free guidance or not

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds`. instead. If not defined, one has to pass `negative_prompt_embeds`. instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

attention_mask (`torch.Tensor`, *optional*) : Pre-generated attention mask. Must provide if passing `prompt_embeds` directly.

negative_attention_mask (`torch.Tensor`, *optional*) : Pre-generated negative attention mask. Must provide if passing `negative_prompt_embeds` directly.

Encodes the prompt into text encoder hidden states.

## Kandinsky3Img2ImgPipeline[[diffusers.Kandinsky3Img2ImgPipeline]]

#### diffusers.Kandinsky3Img2ImgPipeline[[diffusers.Kandinsky3Img2ImgPipeline]]

```python
diffusers.Kandinsky3Img2ImgPipeline(tokenizer: T5Tokenizer, text_encoder: T5EncoderModel, unet: Kandinsky3UNet, scheduler: DDPMScheduler, movq: VQModel)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky3/pipeline_kandinsky3_img2img.py#L56)

#### __call__[[diffusers.Kandinsky3Img2ImgPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, image: typing.Union[torch.Tensor, list[torch.Tensor], PIL.Image.Image, list[PIL.Image.Image]] = None, strength: float = 0.3, num_inference_steps: int = 25, guidance_scale: float = 3.0, negative_prompt: str | list[str] | None = None, num_images_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, negative_attention_mask: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky3/pipeline_kandinsky3_img2img.py#L400)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) : `Image`, or tensor representing an image batch, that will be used as the starting point for the process.

strength (`float`, *optional*, defaults to 0.8) : Indicates extent to transform the reference `image`. Must be between 0 and 1. `image` is used as a starting point and more noise is added the higher the `strength`. The number of denoising steps depends on the amount of noise initially added. When `strength` is 1, added noise is maximum and the denoising process runs for the full number of iterations specified in `num_inference_steps`. A value of 1 essentially ignores `image`.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, *optional*, defaults to 3.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

attention_mask (`torch.Tensor`, *optional*) : Pre-generated attention mask. Must provide if passing `prompt_embeds` directly.

negative_attention_mask (`torch.Tensor`, *optional*) : Pre-generated negative attention mask. Must provide if passing `negative_prompt_embeds` directly.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.stable_diffusion.IFPipelineOutput` instead of a plain tuple.

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> from diffusers import AutoPipelineForImage2Image
>>> from diffusers.utils import load_image
>>> import torch

>>> pipe = AutoPipelineForImage2Image.from_pretrained(
...     "kandinsky-community/kandinsky-3", variant="fp16", torch_dtype=torch.float16
... )
>>> pipe.enable_model_cpu_offload()

>>> prompt = "A painting of the inside of a subway train with tiny raccoons."
>>> image = load_image(
...     "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main/kandinsky3/t2i.png"
... )

>>> generator = torch.Generator(device="cpu").manual_seed(0)
>>> image = pipe(prompt, image=image, strength=0.75, num_inference_steps=25, generator=generator).images[0]
```

#### encode_prompt[[diffusers.Kandinsky3Img2ImgPipeline.encode_prompt]]

```python
encode_prompt(prompt, do_classifier_free_guidance = True, num_images_per_prompt = 1, device = None, negative_prompt = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, _cut_context = True, attention_mask: typing.Optional[torch.Tensor] = None, negative_attention_mask: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky3/pipeline_kandinsky3_img2img.py#L106)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

Encodes the prompt into text encoder hidden states.

device: (`torch.device`, *optional*):
torch device to place the resulting embeddings on
num_images_per_prompt (`int`, *optional*, defaults to 1):
number of images that should be generated per prompt
do_classifier_free_guidance (`bool`, *optional*, defaults to `True`):
whether to use classifier free guidance or not
negative_prompt (`str` or `list[str]`, *optional*):
The prompt or prompts not to guide the image generation. If not defined, one has to pass
`negative_prompt_embeds`. instead. If not defined, one has to pass `negative_prompt_embeds`. instead.
Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).
prompt_embeds (`torch.Tensor`, *optional*):
Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
provided, text embeddings will be generated from `prompt` input argument.
negative_prompt_embeds (`torch.Tensor`, *optional*):
Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
argument.
attention_mask (`torch.Tensor`, *optional*):
Pre-generated attention mask. Must provide if passing `prompt_embeds` directly.
negative_attention_mask (`torch.Tensor`, *optional*):
Pre-generated negative attention mask. Must provide if passing `negative_prompt_embeds` directly.
