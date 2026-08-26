# Hunyuan-DiT
![chinese elements understanding](https://github.com/gnobitab/diffusers-hunyuan/assets/1157982/39b99036-c3cb-4f16-bb1a-40ec25eda573)

[Hunyuan-DiT : A Powerful Multi-Resolution Diffusion Transformer with Fine-Grained Chinese Understanding](https://huggingface.co/papers/2405.08748) from Tencent Hunyuan.

The abstract from the paper is:

*We present Hunyuan-DiT, a text-to-image diffusion transformer with fine-grained understanding of both English and Chinese. To construct Hunyuan-DiT, we carefully design the transformer structure, text encoder, and positional encoding. We also build from scratch a whole data pipeline to update and evaluate data for iterative model optimization. For fine-grained language understanding, we train a Multimodal Large Language Model to refine the captions of the images. Finally, Hunyuan-DiT can perform multi-turn multimodal dialogue with users, generating and refining images according to the context. Through our holistic human evaluation protocol with more than 50 professional human evaluators, Hunyuan-DiT sets a new state-of-the-art in Chinese-to-image generation compared with other open-source models.*

You can find the original codebase at [Tencent/HunyuanDiT](https://github.com/Tencent/HunyuanDiT) and all the available checkpoints at [Tencent-Hunyuan](https://huggingface.co/Tencent-Hunyuan/HunyuanDiT).

**Highlights**: HunyuanDiT supports Chinese/English-to-image, multi-resolution generation.

HunyuanDiT has the following components:
* It uses a diffusion transformer as the backbone
* It combines two text encoders, a bilingual CLIP and a multilingual T5 encoder

> [!TIP]
> Make sure to check out the Schedulers [guide](../../using-diffusers/schedulers) to learn how to explore the tradeoff between scheduler speed and quality, and see the [reuse components across pipelines](../../using-diffusers/loading#reuse-a-pipeline) section to learn how to efficiently load the same components into multiple pipelines.

> [!TIP]
> You can further improve generation quality by passing the generated image from `HungyuanDiTPipeline` to the [SDXL refiner](./stable_diffusion/stable_diffusion_xl#base-to-refiner-model) model.

## Optimization

You can optimize the pipeline's runtime and memory consumption with torch.compile and feed-forward chunking. To learn about other optimization methods, check out the [Speed up inference](../../optimization/fp16) and [Reduce memory usage](../../optimization/memory) guides.

### Inference

Use [`torch.compile`](https://huggingface.co/docs/diffusers/main/en/tutorials/fast_diffusion#torchcompile) to reduce the inference latency.

First, load the pipeline:

```python
from diffusers import HunyuanDiTPipeline
import torch

pipeline = HunyuanDiTPipeline.from_pretrained(
	"Tencent-Hunyuan/HunyuanDiT-Diffusers", dtype=torch.float16
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

image = pipeline(prompt="一个宇航员在骑马").images[0]
```

The [benchmark](https://gist.github.com/sayakpaul/29d3a14905cfcbf611fe71ebd22e9b23) results on a 80GB A100 machine are:

```bash
With torch.compile(): Average inference time: 12.470 seconds.
Without torch.compile(): Average inference time: 20.570 seconds.
```

### Memory optimization

By loading the T5 text encoder in 8 bits, you can run the pipeline in just under 6 GBs of GPU VRAM. Refer to [this script](https://gist.github.com/sayakpaul/3154605f6af05b98a41081aaba5ca43e) for details.

Furthermore, you can use the [enable_forward_chunking()](/docs/diffusers/v0.40.0/en/api/models/hunyuan_transformer2d#diffusers.HunyuanDiT2DModel.enable_forward_chunking) method to reduce memory usage. Feed-forward chunking runs the feed-forward layers in a transformer block in a loop instead of all at once. This gives you a trade-off between memory consumption and inference runtime.

```diff
+ pipeline.transformer.enable_forward_chunking(chunk_size=1, dim=1)
```

## HunyuanDiTPipeline[[diffusers.HunyuanDiTPipeline]]

#### diffusers.HunyuanDiTPipeline[[diffusers.HunyuanDiTPipeline]]

```python
diffusers.HunyuanDiTPipeline(vae: AutoencoderKL, text_encoder: BertModel, tokenizer: BertTokenizer, transformer: HunyuanDiT2DModel, scheduler: DDPMScheduler, safety_checker: StableDiffusionSafetyChecker, feature_extractor: CLIPImageProcessorPil, requires_safety_checker: bool = True, text_encoder_2: transformers.models.t5.modeling_t5.T5EncoderModel | None = None, tokenizer_2: transformers.models.t5.tokenization_t5.T5Tokenizer | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/hunyuandit/pipeline_hunyuandit.py#L149)

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations. We use `sdxl-vae-fp16-fix`.

text_encoder (`~transformers.BertModel`, `~transformers.CLIPTextModel` | None) : Frozen text-encoder ([clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14)). HunyuanDiT uses a fine-tuned [bilingual CLIP].

tokenizer (`~transformers.BertTokenizer`, `~transformers.CLIPTokenizer` | None) : A `BertTokenizer` or `CLIPTokenizer` to tokenize text.

transformer ([HunyuanDiT2DModel](/docs/diffusers/v0.40.0/en/api/models/hunyuan_transformer2d#diffusers.HunyuanDiT2DModel)) : The HunyuanDiT model designed by Tencent Hunyuan.

text_encoder_2 (`T5EncoderModel`) : The mT5 embedder. Specifically, it is 't5-v1_1-xxl'.

tokenizer_2 (`T5Tokenizer`) : The tokenizer for the mT5 embedder.

scheduler ([DDPMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddpm#diffusers.DDPMScheduler)) : A scheduler to be used in combination with HunyuanDiT to denoise the encoded image latents.

Pipeline for English/Chinese-to-image generation using HunyuanDiT.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

HunyuanDiT uses two text encoders: [mT5](https://huggingface.co/google/mt5-base) and [bilingual CLIP](fine-tuned by
ourselves)

#### __call__[[diffusers.HunyuanDiTPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, height: int | None = None, width: int | None = None, num_inference_steps: int | None = 50, guidance_scale: float | None = 5.0, negative_prompt: str | list[str] | None = None, num_images_per_prompt: int | None = 1, eta: float | None = 0.0, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_embeds_2: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds_2: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, prompt_attention_mask_2: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask_2: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, callback_on_step_end: typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None, callback_on_step_end_tensor_inputs: list = ['latents'], guidance_rescale: float = 0.0, original_size: tuple[int, int] | None = (1024, 1024), target_size: tuple[int, int] | None = None, crops_coords_top_left: tuple = (0, 0), use_resolution_binning: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/hunyuandit/pipeline_hunyuandit.py#L568)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide image generation. If not defined, you need to pass `prompt_embeds`.

height (`int`) : The height in pixels of the generated image.

width (`int`) : The width in pixels of the generated image.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference. This parameter is modulated by `strength`.

guidance_scale (`float`, *optional*, defaults to 7.5) : A higher guidance scale value encourages the model to generate images closely linked to the text `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide what to not include in image generation. If not defined, you need to pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale < 1`).

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

eta (`float`, *optional*, defaults to 0.0) : Corresponds to parameter eta (η) from the [DDIM](https://huggingface.co/papers/2010.02502) paper. Only applies to the [DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), and is ignored in other schedulers.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor is generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, text embeddings are generated from the `prompt` input argument.

prompt_embeds_2 (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, text embeddings are generated from the `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, `negative_prompt_embeds` are generated from the `negative_prompt` input argument.

negative_prompt_embeds_2 (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, `negative_prompt_embeds` are generated from the `negative_prompt` input argument.

prompt_attention_mask (`torch.Tensor`, *optional*) : Attention mask for the prompt. Required when `prompt_embeds` is passed directly.

prompt_attention_mask_2 (`torch.Tensor`, *optional*) : Attention mask for the prompt. Required when `prompt_embeds_2` is passed directly.

negative_prompt_attention_mask (`torch.Tensor`, *optional*) : Attention mask for the negative prompt. Required when `negative_prompt_embeds` is passed directly.

negative_prompt_attention_mask_2 (`torch.Tensor`, *optional*) : Attention mask for the negative prompt. Required when `negative_prompt_embeds_2` is passed directly.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generated image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [StableDiffusionPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/stable_diffusion/image_variation#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) instead of a plain tuple.

callback_on_step_end (`Callable[[int, int], None]`, `PipelineCallback`, `MultiPipelineCallbacks`, *optional*) : A callback function or a list of callback functions to be called at the end of each denoising step.

callback_on_step_end_tensor_inputs (`list[str]`, *optional*) : A list of tensor inputs that should be passed to the callback function. If not defined, all tensor inputs will be passed.

guidance_rescale (`float`, *optional*, defaults to 0.0) : Rescale the noise_cfg according to `guidance_rescale`. Based on findings of [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891). See Section 3.4

original_size (`tuple[int, int]`, *optional*, defaults to `(1024, 1024)`) : The original size of the image. Used to calculate the time ids.

target_size (`tuple[int, int]`, *optional*) : The target size of the image. Used to calculate the time ids.

crops_coords_top_left (`tuple[int, int]`, *optional*, defaults to `(0, 0)`) : The top left coordinates of the crop. Used to calculate the time ids.

use_resolution_binning (`bool`, *optional*, defaults to `True`) : Whether to use resolution binning or not. If `True`, the input resolution will be mapped to the closest standard resolution. Supported resolutions are 1024x1024, 1280x1280, 1024x768, 1152x864, 1280x960, 768x1024, 864x1152, 960x1280, 1280x768, and 768x1280. It is recommended to set this to `True`.

**Returns:** [StableDiffusionPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/stable_diffusion/image_variation#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) or `tuple`

If `return_dict` is `True`, [StableDiffusionPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/stable_diffusion/image_variation#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated images and the
second element is a list of `bool`s indicating whether the corresponding generated image contains
"not-safe-for-work" (nsfw) content.

The call function to the pipeline for generation with HunyuanDiT.

Examples:
```py
>>> import torch
>>> from diffusers import HunyuanDiTPipeline

>>> pipe = HunyuanDiTPipeline.from_pretrained(
...     "Tencent-Hunyuan/HunyuanDiT-Diffusers", torch_dtype=torch.float16
... )
>>> pipe.to("cuda")

>>> # You may also use English prompt as HunyuanDiT supports both English and Chinese
>>> # prompt = "An astronaut riding a horse"
>>> prompt = "一个宇航员在骑马"
>>> image = pipe(prompt).images[0]
```

#### encode_prompt[[diffusers.HunyuanDiTPipeline.encode_prompt]]

```python
encode_prompt(prompt: str, device: device = None, dtype: dtype = None, num_images_per_prompt: int = 1, do_classifier_free_guidance: bool = True, negative_prompt: str | None = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, max_sequence_length: int | None = None, text_encoder_index: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/hunyuandit/pipeline_hunyuandit.py#L248)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

device : (`torch.device`): torch device

dtype (`torch.dtype`) : torch dtype

num_images_per_prompt (`int`) : number of images that should be generated per prompt

do_classifier_free_guidance (`bool`) : whether to use classifier free guidance or not

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

prompt_attention_mask (`torch.Tensor`, *optional*) : Attention mask for the prompt. Required when `prompt_embeds` is passed directly.

negative_prompt_attention_mask (`torch.Tensor`, *optional*) : Attention mask for the negative prompt. Required when `negative_prompt_embeds` is passed directly.

max_sequence_length (`int`, *optional*) : maximum sequence length to use for the prompt.

text_encoder_index (`int`, *optional*) : Index of the text encoder to use. `0` for clip and `1` for T5.

Encodes the prompt into text encoder hidden states.
