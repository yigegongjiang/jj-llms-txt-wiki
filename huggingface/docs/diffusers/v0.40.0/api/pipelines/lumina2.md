#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License. -->

# Lumina2

  

[Lumina Image 2.0: A Unified and Efficient Image Generative Model](https://huggingface.co/Alpha-VLLM/Lumina-Image-2.0) is a 2 billion parameter flow-based diffusion transformer capable of generating diverse images from text descriptions.

The abstract from the paper is:

*We introduce Lumina-Image 2.0, an advanced text-to-image model that surpasses previous state-of-the-art methods across multiple benchmarks, while also shedding light on its potential to evolve into a generalist vision intelligence model. Lumina-Image 2.0 exhibits three key properties: (1) Unification – it adopts a unified architecture that treats text and image tokens as a joint sequence, enabling natural cross-modal interactions and facilitating task expansion. Besides, since high-quality captioners can provide semantically better-aligned text-image training pairs, we introduce a unified captioning system, UniCaptioner, which generates comprehensive and precise captions for the model. This not only accelerates model convergence but also enhances prompt adherence, variable-length prompt handling, and task generalization via prompt templates. (2) Efficiency – to improve the efficiency of the unified architecture, we develop a set of optimization techniques that improve semantic learning and fine-grained texture generation during training while incorporating inference-time acceleration strategies without compromising image quality. (3) Transparency – we open-source all training details, code, and models to ensure full reproducibility, aiming to bridge the gap between well-resourced closed-source research teams and independent developers.*

> [!TIP]
> Make sure to check out the Schedulers [guide](../../using-diffusers/schedulers) to learn how to explore the tradeoff between scheduler speed and quality, and see the [reuse components across pipelines](../../using-diffusers/loading#reuse-a-pipeline) section to learn how to efficiently load the same components into multiple pipelines.

## Using Single File loading with Lumina Image 2.0

Single file loading for Lumina Image 2.0 is available for the `Lumina2Transformer2DModel`

```python
import torch
from diffusers import Lumina2Transformer2DModel, Lumina2Pipeline

ckpt_path = "https://huggingface.co/Alpha-VLLM/Lumina-Image-2.0/blob/main/consolidated.00-of-01.pth"
transformer = Lumina2Transformer2DModel.from_single_file(
    ckpt_path, dtype=torch.bfloat16
)

pipe = Lumina2Pipeline.from_pretrained(
    "Alpha-VLLM/Lumina-Image-2.0", transformer=transformer, dtype=torch.bfloat16
)
pipe.enable_model_cpu_offload()
image = pipe(
    "a cat holding a sign that says hello",
    generator=torch.Generator("cpu").manual_seed(0),
).images[0]
image.save("lumina-single-file.png")

```

## Using GGUF Quantized Checkpoints with Lumina Image 2.0

GGUF Quantized checkpoints for the `Lumina2Transformer2DModel` can be loaded via `from_single_file` with the `GGUFQuantizationConfig` 

```python
from diffusers import Lumina2Transformer2DModel, Lumina2Pipeline, GGUFQuantizationConfig 

ckpt_path = "https://huggingface.co/calcuis/lumina-gguf/blob/main/lumina2-q4_0.gguf"
transformer = Lumina2Transformer2DModel.from_single_file(
    ckpt_path,
    quantization_config=GGUFQuantizationConfig(compute_dtype=torch.bfloat16),
    dtype=torch.bfloat16,
)

pipe = Lumina2Pipeline.from_pretrained(
    "Alpha-VLLM/Lumina-Image-2.0", transformer=transformer, dtype=torch.bfloat16
)
pipe.enable_model_cpu_offload()
image = pipe(
    "a cat holding a sign that says hello",
    generator=torch.Generator("cpu").manual_seed(0),
).images[0]
image.save("lumina-gguf.png")
```

## Lumina2Pipeline[[diffusers.Lumina2Pipeline]]

#### diffusers.Lumina2Pipeline[[diffusers.Lumina2Pipeline]]

```python
diffusers.Lumina2Pipeline(transformer: Lumina2Transformer2DModel, scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKL, text_encoder: Gemma2PreTrainedModel, tokenizer: GemmaTokenizer)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/lumina2/pipeline_lumina2.py#L137)

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`Gemma2PreTrainedModel`) : Frozen Gemma2 text-encoder.

tokenizer (`GemmaTokenizer` or `GemmaTokenizerFast`) : Gemma tokenizer.

transformer ([Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel)) : A text conditioned `Transformer2DModel` to denoise the encoded image latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

Pipeline for text-to-image generation using Lumina-T2I.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.Lumina2Pipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, width: int | None = None, height: int | None = None, num_inference_steps: int = 30, guidance_scale: float = 4.0, negative_prompt: str | list[str] = None, sigmas: list = None, num_images_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], system_prompt: str | None = None, cfg_trunc_ratio: float = 1.0, cfg_normalization: bool = True, max_sequence_length: int = 256)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/lumina2/pipeline_lumina2.py#L471)

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

attention_kwargs : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

system_prompt (`str`, *optional*) : The system prompt to use for the image generation.

cfg_trunc_ratio (`float`, *optional*, defaults to `1.0`) : The ratio of the timestep interval to apply normalization-based guidance scale.

cfg_normalization (`bool`, *optional*, defaults to `True`) : Whether to apply normalization-based guidance scale.

max_sequence_length (`int`, defaults to `256`) : Maximum sequence length to use with the `prompt`.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

If `return_dict` is `True`, [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) is returned, otherwise a `tuple` is
returned where the first element is a list with the generated images

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import Lumina2Pipeline

>>> pipe = Lumina2Pipeline.from_pretrained("Alpha-VLLM/Lumina-Image-2.0", torch_dtype=torch.bfloat16)
>>> # Enable memory optimizations.
>>> pipe.enable_model_cpu_offload()

>>> prompt = "Upper body of a young woman in a Victorian-era outfit with brass goggles and leather straps. Background shows an industrial revolution cityscape with smoky skies and tall, metal structures"
>>> image = pipe(prompt).images[0]
```

#### encode_prompt[[diffusers.Lumina2Pipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], do_classifier_free_guidance: bool = True, negative_prompt: str | list[str] = None, num_images_per_prompt: int = 1, device: typing.Optional[torch.device] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, system_prompt: str | None = None, max_sequence_length: int = 256)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/lumina2/pipeline_lumina2.py#L238)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`). For Lumina-T2I, this should be "".

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : whether to use classifier free guidance or not

num_images_per_prompt (`int`, *optional*, defaults to 1) : number of images that should be generated per prompt

device : (`torch.device`, *optional*): torch device to place the resulting embeddings on

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. For Lumina-T2I, it's should be the embeddings of the "" string.

max_sequence_length (`int`, defaults to `256`) : Maximum sequence length to use for the prompt.

Encodes the prompt into text encoder hidden states.
