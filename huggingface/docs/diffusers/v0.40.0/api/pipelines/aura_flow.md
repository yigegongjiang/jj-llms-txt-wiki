# AuraFlow

AuraFlow is inspired by [Stable Diffusion 3](../pipelines/stable_diffusion/stable_diffusion_3) and is by far the largest text-to-image generation model that comes with an Apache 2.0 license. This model achieves state-of-the-art results on the [GenEval](https://github.com/djghosh13/geneval) benchmark.

It was developed by the Fal team and more details about it can be found in [this blog post](https://blog.fal.ai/auraflow/).

> [!TIP]
> AuraFlow can be quite expensive to run on consumer hardware devices. However, you can perform a suite of optimizations to run it faster and in a more memory-friendly manner. Check out [this section](https://huggingface.co/blog/sd3#memory-optimizations-for-sd3) for more details.

## Quantization

Quantization helps reduce the memory requirements of very large models by storing model weights in a lower precision data type. However, quantization may have varying impact on video quality depending on the video model.

Refer to the [Quantization](../../quantization/overview) overview to learn more about supported quantization backends and selecting a quantization backend that supports your use case. The example below demonstrates how to load a quantized [AuraFlowPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/aura_flow#diffusers.AuraFlowPipeline) for inference with bitsandbytes.

```py
import torch
from diffusers import BitsAndBytesConfig as DiffusersBitsAndBytesConfig, AuraFlowTransformer2DModel, AuraFlowPipeline
from transformers import BitsAndBytesConfig as BitsAndBytesConfig, T5EncoderModel

quant_config = BitsAndBytesConfig(load_in_8bit=True)
text_encoder_8bit = T5EncoderModel.from_pretrained(
    "fal/AuraFlow",
    subfolder="text_encoder",
    quantization_config=quant_config,
    dtype=torch.float16,
)

quant_config = DiffusersBitsAndBytesConfig(load_in_8bit=True)
transformer_8bit = AuraFlowTransformer2DModel.from_pretrained(
    "fal/AuraFlow",
    subfolder="transformer",
    quantization_config=quant_config,
    dtype=torch.float16,
)

pipeline = AuraFlowPipeline.from_pretrained(
    "fal/AuraFlow",
    text_encoder=text_encoder_8bit,
    transformer=transformer_8bit,
    dtype=torch.float16,
    device_map="balanced",
)

prompt = "a tiny astronaut hatching from an egg on the moon"
image = pipeline(prompt).images[0]
image.save("auraflow.png")
```

Loading [GGUF checkpoints](https://huggingface.co/docs/diffusers/quantization/gguf) are also supported:

```py
import torch
from diffusers import (
    AuraFlowPipeline,
    GGUFQuantizationConfig,
    AuraFlowTransformer2DModel,
)

transformer = AuraFlowTransformer2DModel.from_single_file(
    "https://huggingface.co/city96/AuraFlow-v0.3-gguf/blob/main/aura_flow_0.3-Q2_K.gguf",
    quantization_config=GGUFQuantizationConfig(compute_dtype=torch.bfloat16),
    dtype=torch.bfloat16,
)

pipeline = AuraFlowPipeline.from_pretrained(
    "fal/AuraFlow-v0.3",
    transformer=transformer,
    dtype=torch.bfloat16,
)

prompt = "a cute pony in a field of flowers"
image = pipeline(prompt).images[0]
image.save("auraflow.png")
```

## Support for `torch.compile()`

AuraFlow can be compiled with `torch.compile()` to speed up inference latency even for different resolutions. First, install PyTorch nightly following the instructions from [here](https://pytorch.org/). The snippet below shows the changes needed to enable this:

```diff
+ torch.fx.experimental._config.use_duck_shape = False
+ pipeline.transformer = torch.compile(
    pipeline.transformer, fullgraph=True, dynamic=True
)
```

Specifying `use_duck_shape` to be `False` instructs the compiler if it should use the same symbolic variable to represent input sizes that are the same. For more details, check out [this comment](https://github.com/huggingface/diffusers/pull/11327#discussion_r2047659790).

This enables from 100% (on low resolutions) to a 30% (on 1536x1536 resolution) speed improvements.

Thanks to [AstraliteHeart](https://github.com/huggingface/diffusers/pull/11297/) who helped us rewrite the [AuraFlowTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/aura_flow_transformer2d#diffusers.AuraFlowTransformer2DModel) class so that the above works for different resolutions ([PR](https://github.com/huggingface/diffusers/pull/11297/)).

## AuraFlowPipeline[[diffusers.AuraFlowPipeline]]

#### diffusers.AuraFlowPipeline[[diffusers.AuraFlowPipeline]]

```python
diffusers.AuraFlowPipeline(tokenizer: T5Tokenizer, text_encoder: UMT5EncoderModel, vae: AutoencoderKL, transformer: AuraFlowTransformer2DModel, scheduler: FlowMatchEulerDiscreteScheduler)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/aura_flow/pipeline_aura_flow.py#L123)

**Parameters:**

tokenizer (`T5TokenizerFast`) : Tokenizer of class [T5Tokenizer](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5Tokenizer).

text_encoder (`T5EncoderModel`) : Frozen text-encoder. AuraFlow uses [T5](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5EncoderModel), specifically the [EleutherAI/pile-t5-xl](https://huggingface.co/EleutherAI/pile-t5-xl) variant.

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

transformer ([AuraFlowTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/aura_flow_transformer2d#diffusers.AuraFlowTransformer2DModel)) : Conditional Transformer (MMDiT and DiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

#### __call__[[diffusers.AuraFlowPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, negative_prompt: str | list[str] = None, num_inference_steps: int = 50, sigmas: list = None, guidance_scale: float = 3.5, num_images_per_prompt: int | None = 1, height: int | None = 1024, width: int | None = 1024, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 256, output_type: str | None = 'pil', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None, callback_on_step_end_tensor_inputs: list = ['latents'])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/aura_flow/pipeline_aura_flow.py#L428)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

height (`int`, *optional*, defaults to self.transformer.config.sample_size * self.vae_scale_factor) : The height in pixels of the generated image. This is set to 1024 by default for best results.

width (`int`, *optional*, defaults to self.transformer.config.sample_size * self.vae_scale_factor) : The width in pixels of the generated image. This is set to 1024 by default for best results.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`list[float]`, *optional*) : Custom sigmas used to override the timestep spacing strategy of the scheduler. If `sigmas` is passed, `num_inference_steps` and `timesteps` must be `None`.

guidance_scale (`float`, *optional*, defaults to 5.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.FloatTensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

prompt_attention_mask (`torch.Tensor`, *optional*) : Pre-generated attention mask for text embeddings.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

negative_prompt_attention_mask (`torch.Tensor`, *optional*) : Pre-generated attention mask for negative text embeddings.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.stable_diffusion_xl.StableDiffusionXLPipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int` defaults to 256) : Maximum sequence length to use with the `prompt`.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

If `return_dict` is `True`, [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) is returned, otherwise a `tuple` is
returned where the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import AuraFlowPipeline

>>> pipe = AuraFlowPipeline.from_pretrained("fal/AuraFlow", torch_dtype=torch.float16)
>>> pipe = pipe.to("cuda")
>>> prompt = "A cat holding a sign that says hello world"
>>> image = pipe(prompt).images[0]
>>> image.save("aura_flow.png")
```

#### encode_prompt[[diffusers.AuraFlowPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] = None, do_classifier_free_guidance: bool = True, num_images_per_prompt: int = 1, device: typing.Optional[torch.device] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 256, lora_scale: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/aura_flow/pipeline_aura_flow.py#L232)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : whether to use classifier free guidance or not

num_images_per_prompt (`int`, *optional*, defaults to 1) : number of images that should be generated per prompt

device : (`torch.device`, *optional*): torch device to place the resulting embeddings on

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

prompt_attention_mask (`torch.Tensor`, *optional*) : Pre-generated attention mask for text embeddings.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings.

negative_prompt_attention_mask (`torch.Tensor`, *optional*) : Pre-generated attention mask for negative text embeddings.

max_sequence_length (`int`, defaults to 256) : Maximum sequence length to use for the prompt.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

Encodes the prompt into text encoder hidden states.
