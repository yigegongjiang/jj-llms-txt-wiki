# Lumina-T2X
![concepts](https://github.com/Alpha-VLLM/Lumina-T2X/assets/54879512/9f52eabb-07dc-4881-8257-6d8a5f2a0a5a)

[Lumina-Next : Making Lumina-T2X Stronger and Faster with Next-DiT](https://github.com/Alpha-VLLM/Lumina-T2X/blob/main/assets/lumina-next.pdf) from Alpha-VLLM, OpenGVLab, Shanghai AI Laboratory.

The abstract from the paper is:

*Lumina-T2X is a nascent family of Flow-based Large Diffusion Transformers (Flag-DiT) that establishes a unified framework for transforming noise into various modalities, such as images and videos, conditioned on text instructions. Despite its promising capabilities, Lumina-T2X still encounters challenges including training instability, slow inference, and extrapolation artifacts. In this paper, we present Lumina-Next, an improved version of Lumina-T2X, showcasing stronger generation performance with increased training and inference efficiency. We begin with a comprehensive analysis of the Flag-DiT architecture and identify several suboptimal components, which we address by introducing the Next-DiT architecture with 3D RoPE and sandwich normalizations. To enable better resolution extrapolation, we thoroughly compare different context extrapolation methods applied to text-to-image generation with 3D RoPE, and propose Frequency- and Time-Aware Scaled RoPE tailored for diffusion transformers. Additionally, we introduce a sigmoid time discretization schedule to reduce sampling steps in solving the Flow ODE and the Context Drop method to merge redundant visual tokens for faster network evaluation, effectively boosting the overall sampling speed. Thanks to these improvements, Lumina-Next not only improves the quality and efficiency of basic text-to-image generation but also demonstrates superior resolution extrapolation capabilities and multilingual generation using decoder-based LLMs as the text encoder, all in a zero-shot manner. To further validate Lumina-Next as a versatile generative framework, we instantiate it on diverse tasks including visual recognition, multi-view, audio, music, and point cloud generation, showcasing strong performance across these domains. By releasing all codes and model weights at https://github.com/Alpha-VLLM/Lumina-T2X, we aim to advance the development of next-generation generative AI capable of universal modeling.*

**Highlights**: Lumina-Next is a next-generation Diffusion Transformer that significantly enhances text-to-image generation, multilingual generation, and multitask performance by introducing the Next-DiT architecture, 3D RoPE, and frequency- and time-aware RoPE, among other improvements.

Lumina-Next has the following components:
* It improves sampling efficiency with fewer and faster Steps.
* It uses a Next-DiT as a transformer backbone with Sandwichnorm 3D RoPE, and Grouped-Query Attention.
* It uses a Frequency- and Time-Aware Scaled RoPE.

---

[Lumina-T2X: Transforming Text into Any Modality, Resolution, and Duration via Flow-based Large Diffusion Transformers](https://huggingface.co/papers/2405.05945) from Alpha-VLLM, OpenGVLab, Shanghai AI Laboratory.

The abstract from the paper is:

*Sora unveils the potential of scaling Diffusion Transformer for generating photorealistic images and videos at arbitrary resolutions, aspect ratios, and durations, yet it still lacks sufficient implementation details. In this technical report, we introduce the Lumina-T2X family - a series of Flow-based Large Diffusion Transformers (Flag-DiT) equipped with zero-initialized attention, as a unified framework designed to transform noise into images, videos, multi-view 3D objects, and audio clips conditioned on text instructions. By tokenizing the latent spatial-temporal space and incorporating learnable placeholders such as [nextline] and [nextframe] tokens, Lumina-T2X seamlessly unifies the representations of different modalities across various spatial-temporal resolutions. This unified approach enables training within a single framework for different modalities and allows for flexible generation of multimodal data at any resolution, aspect ratio, and length during inference. Advanced techniques like RoPE, RMSNorm, and flow matching enhance the stability, flexibility, and scalability of Flag-DiT, enabling models of Lumina-T2X to scale up to 7 billion parameters and extend the context window to 128K tokens. This is particularly beneficial for creating ultra-high-definition images with our Lumina-T2I model and long 720p videos with our Lumina-T2V model. Remarkably, Lumina-T2I, powered by a 5-billion-parameter Flag-DiT, requires only 35% of the training computational costs of a 600-million-parameter naive DiT. Our further comprehensive analysis underscores Lumina-T2X's preliminary capability in resolution extrapolation, high-resolution editing, generating consistent 3D views, and synthesizing videos with seamless transitions. We expect that the open-sourcing of Lumina-T2X will further foster creativity, transparency, and diversity in the generative AI community.*

You can find the original codebase at [Alpha-VLLM](https://github.com/Alpha-VLLM/Lumina-T2X) and all the available checkpoints at [Alpha-VLLM Lumina Family](https://huggingface.co/collections/Alpha-VLLM/lumina-family-66423205bedb81171fd0644b).

**Highlights**: Lumina-T2X supports Any Modality, Resolution, and Duration.

Lumina-T2X has the following components:
* It uses a Flow-based Large Diffusion Transformer as the backbone
* It supports different any modalities with one backbone and corresponding encoder, decoder.

This pipeline was contributed by [PommesPeter](https://github.com/PommesPeter). The original codebase can be found [here](https://github.com/Alpha-VLLM/Lumina-T2X). The original weights can be found under [hf.co/Alpha-VLLM](https://huggingface.co/Alpha-VLLM).

> [!TIP]
> Make sure to check out the Schedulers [guide](../../using-diffusers/schedulers) to learn how to explore the tradeoff between scheduler speed and quality, and see the [reuse components across pipelines](../../using-diffusers/loading#reuse-a-pipeline) section to learn how to efficiently load the same components into multiple pipelines.

### Inference (Text-to-Image)

Use [`torch.compile`](https://huggingface.co/docs/diffusers/main/en/tutorials/fast_diffusion#torchcompile) to reduce the inference latency.

First, load the pipeline:

```python
from diffusers import LuminaPipeline
import torch

pipeline = LuminaPipeline.from_pretrained(
	"Alpha-VLLM/Lumina-Next-SFT-diffusers", dtype=torch.bfloat16
).to("cuda")
```

Then change the memory layout of the pipelines `transformer` and `vae` components to `torch.channels-last`:

```python
pipeline.transformer.to(memory_format=torch.channels_last)
pipeline.vae.to(memory_format=torch.channels_last)
```

Finally, compile the components and run inference:

```python
pipeline.transformer = torch.compile(pipeline.transformer, mode="max-autotune", fullgraph=True)
pipeline.vae.decode = torch.compile(pipeline.vae.decode, mode="max-autotune", fullgraph=True)

image = pipeline(prompt="Upper body of a young woman in a Victorian-era outfit with brass goggles and leather straps. Background shows an industrial revolution cityscape with smoky skies and tall, metal structures").images[0]
```

## Quantization

Quantization helps reduce the memory requirements of very large models by storing model weights in a lower precision data type. However, quantization may have varying impact on video quality depending on the video model.

Refer to the [Quantization](../../quantization/overview) overview to learn more about supported quantization backends and selecting a quantization backend that supports your use case. The example below demonstrates how to load a quantized [LuminaPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/lumina#diffusers.LuminaPipeline) for inference with bitsandbytes.

```py
import torch
from diffusers import BitsAndBytesConfig as DiffusersBitsAndBytesConfig, Transformer2DModel, LuminaPipeline
from transformers import BitsAndBytesConfig as BitsAndBytesConfig, T5EncoderModel

quant_config = BitsAndBytesConfig(load_in_8bit=True)
text_encoder_8bit = T5EncoderModel.from_pretrained(
    "Alpha-VLLM/Lumina-Next-SFT-diffusers",
    subfolder="text_encoder",
    quantization_config=quant_config,
    dtype=torch.float16,
)

quant_config = DiffusersBitsAndBytesConfig(load_in_8bit=True)
transformer_8bit = Transformer2DModel.from_pretrained(
    "Alpha-VLLM/Lumina-Next-SFT-diffusers",
    subfolder="transformer",
    quantization_config=quant_config,
    dtype=torch.float16,
)

pipeline = LuminaPipeline.from_pretrained(
    "Alpha-VLLM/Lumina-Next-SFT-diffusers",
    text_encoder=text_encoder_8bit,
    transformer=transformer_8bit,
    dtype=torch.float16,
    device_map="balanced",
)

prompt = "a tiny astronaut hatching from an egg on the moon"
image = pipeline(prompt).images[0]
image.save("lumina.png")
```

## LuminaPipeline[[diffusers.LuminaPipeline]]

#### diffusers.LuminaPipeline[[diffusers.LuminaPipeline]]

```python
diffusers.LuminaPipeline(transformer: LuminaNextDiT2DModel, scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKL, text_encoder: GemmaPreTrainedModel, tokenizer: GemmaTokenizer)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/lumina/pipeline_lumina.py#L136)

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`GemmaPreTrainedModel`) : Frozen Gemma text-encoder.

tokenizer (`GemmaTokenizer` or `GemmaTokenizerFast`) : Gemma tokenizer.

transformer ([Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel)) : A text conditioned `Transformer2DModel` to denoise the encoded image latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

Pipeline for text-to-image generation using Lumina-T2I.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.LuminaPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, width: int | None = None, height: int | None = None, num_inference_steps: int = 30, guidance_scale: float = 4.0, negative_prompt: str | list[str] = None, sigmas: list = None, num_images_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, clean_caption: bool = True, max_sequence_length: int = 256, scaling_watershed: float | None = 1.0, proportional_attn: bool | None = True, callback_on_step_end: typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None, callback_on_step_end_tensor_inputs: list = ['latents'])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/lumina/pipeline_lumina.py#L632)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

num_inference_steps (`int`, *optional*, defaults to 30) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`list[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

height (`int`, *optional*, defaults to self.unet.config.sample_size) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to self.unet.config.sample_size) : The width in pixels of the generated image.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

prompt_attention_mask (`torch.Tensor`, *optional*) : Pre-generated attention mask for text embeddings.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. For Lumina-T2I this negative prompt should be "". If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

negative_prompt_attention_mask (`torch.Tensor`, *optional*) : Pre-generated attention mask for negative text embeddings.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.stable_diffusion.IFPipelineOutput` instead of a plain tuple.

clean_caption (`bool`, *optional*, defaults to `True`) : Whether or not to clean the caption before creating embeddings. Requires `beautifulsoup4` and `ftfy` to be installed. If the dependencies are not installed, the embeddings will be created from the raw prompt.

max_sequence_length (`int` defaults to 120) : Maximum sequence length to use with the `prompt`.

scaling_watershed (`float`, *optional*, defaults to 1.0) : Resolution scaling threshold used by Lumina to switch between standard and extended-context attention.

proportional_attn (`bool`, *optional*, defaults to True) : Whether to scale attention proportionally for high-resolution generation.

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

If `return_dict` is `True`, [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) is returned, otherwise a `tuple` is
returned where the first element is a list with the generated images

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import LuminaPipeline

>>> pipe = LuminaPipeline.from_pretrained("Alpha-VLLM/Lumina-Next-SFT-diffusers", torch_dtype=torch.bfloat16)
>>> # Enable memory optimizations.
>>> pipe.enable_model_cpu_offload()

>>> prompt = "Upper body of a young woman in a Victorian-era outfit with brass goggles and leather straps. Background shows an industrial revolution cityscape with smoky skies and tall, metal structures"
>>> image = pipe(prompt).images[0]
```

#### encode_prompt[[diffusers.LuminaPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], do_classifier_free_guidance: bool = True, negative_prompt: str | list[str] = None, num_images_per_prompt: int = 1, device: typing.Optional[torch.device] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, clean_caption: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/lumina/pipeline_lumina.py#L262)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`). For Lumina-T2I, this should be "".

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : whether to use classifier free guidance or not

num_images_per_prompt (`int`, *optional*, defaults to 1) : number of images that should be generated per prompt

device : (`torch.device`, *optional*): torch device to place the resulting embeddings on

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. For Lumina-T2I, it's should be the embeddings of the "" string.

clean_caption (`bool`, defaults to `False`) : If `True`, the function will preprocess and clean the provided caption before encoding.

max_sequence_length (`int`, defaults to 256) : Maximum sequence length to use for the prompt.

Encodes the prompt into text encoder hidden states.
