# ControlNet with Flux.1

  

FluxControlNetPipeline is an implementation of ControlNet for Flux.1.

ControlNet was introduced in [Adding Conditional Control to Text-to-Image Diffusion Models](https://huggingface.co/papers/2302.05543) by Lvmin Zhang, Anyi Rao, and Maneesh Agrawala.

With a ControlNet model, you can provide an additional control image to condition and control Stable Diffusion generation. For example, if you provide a depth map, the ControlNet model generates an image that'll preserve the spatial information from the depth map. It is a more flexible and accurate way to control the image generation process.

The abstract from the paper is:

*We present ControlNet, a neural network architecture to add spatial conditioning controls to large, pretrained text-to-image diffusion models. ControlNet locks the production-ready large diffusion models, and reuses their deep and robust encoding layers pretrained with billions of images as a strong backbone to learn a diverse set of conditional controls. The neural architecture is connected with "zero convolutions" (zero-initialized convolution layers) that progressively grow the parameters from zero and ensure that no harmful noise could affect the finetuning. We test various conditioning controls, eg, edges, depth, segmentation, human pose, etc, with Stable Diffusion, using single or multiple conditions, with or without prompts. We show that the training of ControlNets is robust with small (<50k) and large (>1m) datasets. Extensive results show that ControlNet may facilitate wider applications to control image diffusion models.*

This controlnet code is implemented by [The InstantX Team](https://huggingface.co/InstantX). You can find pre-trained checkpoints for Flux-ControlNet in the table below:

| ControlNet type | Developer | Link |
| -------- | ---------- | ---- |
| Canny | [The InstantX Team](https://huggingface.co/InstantX) | [Link](https://huggingface.co/InstantX/FLUX.1-dev-Controlnet-Canny) |
| Depth | [The InstantX Team](https://huggingface.co/InstantX) | [Link](https://huggingface.co/Shakker-Labs/FLUX.1-dev-ControlNet-Depth) |
| Union | [The InstantX Team](https://huggingface.co/InstantX) | [Link](https://huggingface.co/InstantX/FLUX.1-dev-Controlnet-Union) |

XLabs ControlNets are also supported, which was contributed by the [XLabs team](https://huggingface.co/XLabs-AI).

| ControlNet type | Developer | Link |
| -------- | ---------- | ---- |
| Canny | [The XLabs Team](https://huggingface.co/XLabs-AI) | [Link](https://huggingface.co/XLabs-AI/flux-controlnet-canny-diffusers) |
| Depth | [The XLabs Team](https://huggingface.co/XLabs-AI) | [Link](https://huggingface.co/XLabs-AI/flux-controlnet-depth-diffusers) |
| HED | [The XLabs Team](https://huggingface.co/XLabs-AI) | [Link](https://huggingface.co/XLabs-AI/flux-controlnet-hed-diffusers) |

> [!TIP]
> Make sure to check out the Schedulers [guide](../../using-diffusers/schedulers) to learn how to explore the tradeoff between scheduler speed and quality, and see the [reuse components across pipelines](../../using-diffusers/loading#reuse-a-pipeline) section to learn how to efficiently load the same components into multiple pipelines.

## FluxControlNetPipeline[[diffusers.FluxControlNetPipeline]]

#### diffusers.FluxControlNetPipeline[[diffusers.FluxControlNetPipeline]]

```python
diffusers.FluxControlNetPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKL, text_encoder: CLIPTextModel, tokenizer: CLIPTokenizer, text_encoder_2: T5EncoderModel, tokenizer_2: T5Tokenizer, transformer: FluxTransformer2DModel, controlnet: diffusers.models.controlnets.controlnet_flux.FluxControlNetModel | list[diffusers.models.controlnets.controlnet_flux.FluxControlNetModel] | tuple[diffusers.models.controlnets.controlnet_flux.FluxControlNetModel] | diffusers.models.controlnets.controlnet_flux.FluxMultiControlNetModel, image_encoder: CLIPVisionModelWithProjection = None, feature_extractor: CLIPImageProcessorPil = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_controlnet.py#L177)

**Parameters:**

transformer ([FluxTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/flux_transformer#diffusers.FluxTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

text_encoder_2 (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer_2 (`T5TokenizerFast`) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

The Flux pipeline for text-to-image generation.

Reference: https://blackforestlabs.ai/announcing-black-forest-labs/

#### __call__[[diffusers.FluxControlNetPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, prompt_2: str | list[str] | None = None, negative_prompt: str | list[str] = None, negative_prompt_2: str | list[str] | None = None, true_cfg_scale: float = 1.0, height: int | None = None, width: int | None = None, num_inference_steps: int = 28, sigmas: list[float] | None = None, guidance_scale: float = 7.0, control_guidance_start: float | list[float] = 0.0, control_guidance_end: float | list[float] = 1.0, control_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, control_mode: int | list[int] | None = None, controlnet_conditioning_scale: float | list[float] = 1.0, num_images_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.FloatTensor] = None, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, ip_adapter_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, ip_adapter_image_embeds: list[torch.Tensor] | None = None, negative_ip_adapter_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, negative_ip_adapter_image_embeds: list[torch.Tensor] | None = None, negative_prompt_embeds: typing.Optional[torch.FloatTensor] = None, negative_pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, output_type: str | None = 'pil', return_dict: bool = True, joint_attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_controlnet.py#L678)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is will be used instead

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `true_cfg_scale` is not greater than `1`).

negative_prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `negative_prompt` is used in all the text-encoders.

true_cfg_scale (`float`, *optional*, defaults to 1.0) : True classifier-free guidance (guidance scale) is enabled when `true_cfg_scale` > 1 and `negative_prompt` is provided.

height (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The height in pixels of the generated image. This is set to 1024 by default for the best results.

width (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The width in pixels of the generated image. This is set to 1024 by default for the best results.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`list[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

guidance_scale (`float`, *optional*, defaults to 7.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

control_guidance_start (`float` or `list[float]`, *optional*, defaults to 0.0) : The percentage of total steps at which the ControlNet starts applying.

control_guidance_end (`float` or `list[float]`, *optional*, defaults to 1.0) : The percentage of total steps at which the ControlNet stops applying.

control_image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, `list[np.ndarray]`, : `list[list[torch.Tensor]]`, `list[list[np.ndarray]]` or `list[list[PIL.Image.Image]]`): The ControlNet input condition to provide guidance to the `unet` for generation. If the type is specified as `torch.Tensor`, it is passed to ControlNet as is. `PIL.Image.Image` can also be accepted as an image. The dimensions of the output image defaults to `image`'s dimensions. If height and/or width are passed, `image` is resized accordingly. If multiple ControlNets are specified in `init`, images must be passed as a list such that each element of the list can be correctly batched for input to a single ControlNet.

controlnet_conditioning_scale (`float` or `list[float]`, *optional*, defaults to 1.0) : The outputs of the ControlNet are multiplied by `controlnet_conditioning_scale` before they are added to the residual in the original `unet`. If multiple ControlNets are specified in `init`, you can set the corresponding scale as a list.

control_mode (`int` or `list[int]`,, *optional*, defaults to None) : The control mode when applying ControlNet-Union.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.FloatTensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

negative_pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled negative_prompt_embeds will be generated from `negative_prompt` input argument.

ip_adapter_image : (`PipelineImageInput`, *optional*): Optional image input to work with IP Adapters.

ip_adapter_image_embeds (`list[torch.Tensor]`, *optional*) : Pre-generated image embeddings for IP-Adapter. It should be a list of length same as number of IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. If not provided, embeddings are computed from the `ip_adapter_image` input argument.

negative_ip_adapter_image : (`PipelineImageInput`, *optional*): Optional image input to work with IP Adapters.

negative_ip_adapter_image_embeds (`list[torch.Tensor]`, *optional*) : Pre-generated image embeddings for IP-Adapter. It should be a list of length same as number of IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. If not provided, embeddings are computed from the `ip_adapter_image` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.flux.FluxPipelineOutput` instead of a plain tuple.

joint_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int` defaults to 512) : Maximum sequence length to use with the `prompt`.

**Returns:** `~pipelines.flux.FluxPipelineOutput` or `tuple`

`~pipelines.flux.FluxPipelineOutput` if `return_dict`
is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the generated
images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers.utils import load_image
>>> from diffusers import FluxControlNetPipeline
>>> from diffusers import FluxControlNetModel

>>> base_model = "black-forest-labs/FLUX.1-dev"
>>> controlnet_model = "InstantX/FLUX.1-dev-controlnet-canny"
>>> controlnet = FluxControlNetModel.from_pretrained(controlnet_model, torch_dtype=torch.bfloat16)
>>> pipe = FluxControlNetPipeline.from_pretrained(
...     base_model, controlnet=controlnet, torch_dtype=torch.bfloat16
... )
>>> pipe.to("cuda")
>>> control_image = load_image("https://huggingface.co/InstantX/SD3-Controlnet-Canny/resolve/main/canny.jpg")
>>> prompt = "A girl in city, 25 years old, cool, futuristic"
>>> image = pipe(
...     prompt,
...     control_image=control_image,
...     control_guidance_start=0.2,
...     control_guidance_end=0.8,
...     controlnet_conditioning_scale=1.0,
...     num_inference_steps=28,
...     guidance_scale=3.5,
... ).images[0]
>>> image.save("flux.png")
```

#### encode_prompt[[diffusers.FluxControlNetPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], prompt_2: str | list[str] | None = None, device: typing.Optional[torch.device] = None, num_images_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, max_sequence_length: int = 512, lora_scale: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_controlnet.py#L342)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is used in all text-encoders

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

clip_skip (`int`, *optional*) : Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that the output of the pre-final layer will be used for computing the prompt embeddings.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

## FluxPipelineOutput[[diffusers.pipelines.flux.pipeline_output.FluxPipelineOutput]]

#### diffusers.pipelines.flux.pipeline_output.FluxPipelineOutput[[diffusers.pipelines.flux.pipeline_output.FluxPipelineOutput]]

```python
diffusers.pipelines.flux.pipeline_output.FluxPipelineOutput(images: list[PIL.Image.Image] | numpy.ndarray)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_output.py#L11)

**Parameters:**

images (`list[PIL.Image.Image]` or `torch.Tensor` or `np.ndarray`) : list of denoised PIL images of length `batch_size` or numpy array or torch tensor of shape `(batch_size, height, width, num_channels)`. PIL images or numpy array present the denoised images of the diffusion pipeline. Torch tensors can represent either the denoised images or the intermediate latents ready to be passed to the decoder.

Output class for Flux image generation pipelines.
