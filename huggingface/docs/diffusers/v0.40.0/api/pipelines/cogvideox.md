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
# limitations under the License.
-->

  
    
      
    
  

# CogVideoX

[CogVideoX](https://huggingface.co/papers/2408.06072) is a large diffusion transformer model - available in 2B and 5B parameters - designed to generate longer and more consistent videos from text. This model uses a 3D causal variational autoencoder to more efficiently process video data by reducing sequence length (and associated training compute) and preventing flickering in generated videos. An "expert" transformer with adaptive LayerNorm improves alignment between text and video, and 3D full attention helps accurately capture motion and time in generated videos.

You can find all the original CogVideoX checkpoints under the [CogVideoX](https://huggingface.co/collections/THUDM/cogvideo-66c08e62f1685a3ade464cce) collection.

> [!TIP]
> Click on the CogVideoX models in the right sidebar for more examples of other video generation tasks.

The example below demonstrates how to generate a video optimized for memory or inference speed.

Refer to the [Reduce memory usage](../../optimization/memory) guide for more details about the various memory saving techniques.

The quantized CogVideoX 5B model below requires ~16GB of VRAM.

```py
import torch
from diffusers import CogVideoXPipeline, AutoModel, TorchAoConfig
from diffusers.quantizers import PipelineQuantizationConfig
from diffusers.hooks import apply_group_offloading
from diffusers.utils import export_to_video
from torchao.quantization import Int8WeightOnlyConfig

# quantize weights to int8 with torchao
pipeline_quant_config = PipelineQuantizationConfig(
  quant_mapping={"transformer": TorchAoConfig(Int8WeightOnlyConfig())}
)

# fp8 layerwise weight-casting
transformer = AutoModel.from_pretrained(
    "THUDM/CogVideoX-5b",
    subfolder="transformer",
    dtype=torch.bfloat16
)
transformer.enable_layerwise_casting(
    storage_dtype=torch.float8_e4m3fn, compute_dtype=torch.bfloat16
)

pipeline = CogVideoXPipeline.from_pretrained(
    "THUDM/CogVideoX-5b",
    transformer=transformer,
    quantization_config=pipeline_quant_config,
    dtype=torch.bfloat16
)
pipeline.to("cuda")

# model-offloading
pipeline.enable_model_cpu_offload()

prompt = """
A detailed wooden toy ship with intricately carved masts and sails is seen gliding smoothly over a plush, blue carpet that mimics the waves of the sea. 
The ship's hull is painted a rich brown, with tiny windows. The carpet, soft and textured, provides a perfect backdrop, resembling an oceanic expanse. 
Surrounding the ship are various other toys and children's items, hinting at a playful environment. The scene captures the innocence and imagination of childhood, 
with the toy ship's journey symbolizing endless adventures in a whimsical, indoor setting.
"""

video = pipeline(
    prompt=prompt,
    guidance_scale=6,
    num_inference_steps=50
).frames[0]
export_to_video(video, "output.mp4", fps=8)
```

[Compilation](../../optimization/fp16#torchcompile) is slow the first time but subsequent calls to the pipeline are faster.

The average inference time with torch.compile on a 80GB A100 is 76.27 seconds compared to 96.89 seconds for an uncompiled model.

```py
import torch
from diffusers import CogVideoXPipeline
from diffusers.utils import export_to_video

pipeline = CogVideoXPipeline.from_pretrained(
    "THUDM/CogVideoX-2b",
    dtype=torch.float16
).to("cuda")

# torch.compile
pipeline.transformer.to(memory_format=torch.channels_last)
pipeline.transformer = torch.compile(
    pipeline.transformer, mode="max-autotune", fullgraph=True
)

prompt = """
A detailed wooden toy ship with intricately carved masts and sails is seen gliding smoothly over a plush, blue carpet that mimics the waves of the sea. 
The ship's hull is painted a rich brown, with tiny windows. The carpet, soft and textured, provides a perfect backdrop, resembling an oceanic expanse. 
Surrounding the ship are various other toys and children's items, hinting at a playful environment. The scene captures the innocence and imagination of childhood, 
with the toy ship's journey symbolizing endless adventures in a whimsical, indoor setting.
"""

video = pipeline(
    prompt=prompt,
    guidance_scale=6,
    num_inference_steps=50
).frames[0]
export_to_video(video, "output.mp4", fps=8)
```

## Notes

- CogVideoX supports LoRAs with [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.CogVideoXLoraLoaderMixin.load_lora_weights).

  
  Show example code

  ```py
  import torch
  from diffusers import CogVideoXPipeline
  from diffusers.hooks import apply_group_offloading
  from diffusers.utils import export_to_video

  pipeline = CogVideoXPipeline.from_pretrained(
      "THUDM/CogVideoX-5b",
      dtype=torch.bfloat16
  )
  pipeline.to("cuda")

  # load LoRA weights
  pipeline.load_lora_weights("finetrainers/CogVideoX-1.5-crush-smol-v0", adapter_name="crush-lora")
  pipeline.set_adapters("crush-lora", 0.9)

  # model-offloading
  pipeline.enable_model_cpu_offload()

  prompt = """
  PIKA_CRUSH A large metal cylinder is seen pressing down on a pile of Oreo cookies, flattening them as if they were under a hydraulic press.
  """
  negative_prompt = "inconsistent motion, blurry motion, worse quality, degenerate outputs, deformed outputs"

  video = pipeline(
      prompt=prompt, 
      negative_prompt=negative_prompt, 
      num_frames=81, 
      height=480,
      width=768,
      num_inference_steps=50
  ).frames[0]
  export_to_video(video, "output.mp4", fps=16)
  ```

  

- The text-to-video (T2V) checkpoints work best with a resolution of 1360x768 because that was the resolution it was pretrained on.

- The image-to-video (I2V) checkpoints work with multiple resolutions. The width can vary from 768 to 1360, but the height must be 758. Both height and width must be divisible by 16.

- Both T2V and I2V checkpoints work best with 81 and 161 frames. It is recommended to export the generated video at 16fps.

- Refer to the table below to view memory usage when various memory-saving techniques are enabled.

  | method | memory usage (enabled) | memory usage (disabled) |
  |---|---|---|
  | enable_model_cpu_offload | 19GB | 33GB |
  | enable_sequential_cpu_offload | <4GB | ~33GB (very slow inference speed) |
  | enable_tiling | 11GB (with enable_model_cpu_offload) | --- |
 
## CogVideoXPipeline[[diffusers.CogVideoXPipeline]]

#### diffusers.CogVideoXPipeline[[diffusers.CogVideoXPipeline]]

```python
diffusers.CogVideoXPipeline(tokenizer: T5Tokenizer, text_encoder: T5EncoderModel, vae: AutoencoderKLCogVideoX, transformer: CogVideoXTransformer3DModel, scheduler: diffusers.schedulers.scheduling_ddim_cogvideox.CogVideoXDDIMScheduler | diffusers.schedulers.scheduling_dpm_cogvideox.CogVideoXDPMScheduler)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cogvideo/pipeline_cogvideox.py#L147)

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

text_encoder (`T5EncoderModel`) : Frozen text-encoder. CogVideoX uses [T5](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5EncoderModel); specifically the [t5-v1_1-xxl](https://huggingface.co/PixArt-alpha/PixArt-alpha/tree/main/t5-v1_1-xxl) variant.

tokenizer (`T5Tokenizer`) : Tokenizer of class [T5Tokenizer](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5Tokenizer).

transformer ([CogVideoXTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/cogvideox_transformer3d#diffusers.CogVideoXTransformer3DModel)) : A text conditioned `CogVideoXTransformer3DModel` to denoise the encoded video latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `transformer` to denoise the encoded video latents.

Pipeline for text-to-video generation using CogVideoX.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.CogVideoXPipeline.__call__]]

```python
__call__(prompt: str | list[str] | None = None, negative_prompt: str | list[str] | None = None, height: int | None = None, width: int | None = None, num_frames: int | None = None, num_inference_steps: int = 50, timesteps: list[int] | None = None, guidance_scale: float = 6, use_dynamic_cfg: bool = False, num_videos_per_prompt: int = 1, eta: float = 0.0, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.FloatTensor] = None, prompt_embeds: typing.Optional[torch.FloatTensor] = None, negative_prompt_embeds: typing.Optional[torch.FloatTensor] = None, output_type: str = 'pil', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 226)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cogvideo/pipeline_cogvideox.py#L505)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

height (`int`, *optional*, defaults to self.transformer.config.sample_height * self.vae_scale_factor_spatial) : The height in pixels of the generated image. This is set to 480 by default for the best results.

width (`int`, *optional*, defaults to self.transformer.config.sample_height * self.vae_scale_factor_spatial) : The width in pixels of the generated image. This is set to 720 by default for the best results.

num_frames (`int`, defaults to `48`) : Number of frames to generate. Must be divisible by self.vae_scale_factor_temporal. Generated video will contain 1 extra frame because CogVideoX is conditioned with (num_seconds * fps + 1) frames where num_seconds is 6 and fps is 8. However, since videos can be saved at any fps, the only condition that needs to be satisfied is that of divisibility mentioned above.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

timesteps (`list[int]`, *optional*) : Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used. Must be in descending order.

guidance_scale (`float`, *optional*, defaults to 7.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

use_dynamic_cfg (`bool`, *optional*, defaults to `False`) : If True, dynamically adjusts the guidance scale during inference.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : The number of videos to generate per prompt.

eta (`float`, *optional*, defaults to 0.0) : Corresponds to parameter eta (η) from the [DDIM](https://arxiv.org/abs/2010.02502) paper. Only applies to [DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), and is ignored in other schedulers.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.FloatTensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.stable_diffusion_xl.StableDiffusionXLPipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int`, defaults to `226`) : Maximum sequence length in encoded prompt. Must be consistent with `self.transformer.config.max_text_seq_length` otherwise may lead to poor results.

**Returns:** [CogVideoXPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/cogvideox#diffusers.pipelines.cogvideo.pipeline_output.CogVideoXPipelineOutput) or `tuple`

[CogVideoXPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/cogvideox#diffusers.pipelines.cogvideo.pipeline_output.CogVideoXPipelineOutput) if `return_dict` is True, otherwise a
`tuple`. When returning a tuple, the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```python
>>> import torch
>>> from diffusers import CogVideoXPipeline
>>> from diffusers.utils import export_to_video

>>> # Models: "THUDM/CogVideoX-2b" or "THUDM/CogVideoX-5b"
>>> pipe = CogVideoXPipeline.from_pretrained("THUDM/CogVideoX-2b", torch_dtype=torch.float16).to("cuda")
>>> prompt = (
...     "A panda, dressed in a small, red jacket and a tiny hat, sits on a wooden stool in a serene bamboo forest. "
...     "The panda's fluffy paws strum a miniature acoustic guitar, producing soft, melodic tunes. Nearby, a few other "
...     "pandas gather, watching curiously and some clapping in rhythm. Sunlight filters through the tall bamboo, "
...     "casting a gentle glow on the scene. The panda's face is expressive, showing concentration and joy as it plays. "
...     "The background includes a small, flowing stream and vibrant green foliage, enhancing the peaceful and magical "
...     "atmosphere of this unique musical performance."
... )
>>> video = pipe(prompt=prompt, guidance_scale=6, num_inference_steps=50).frames[0]
>>> export_to_video(video, "output.mp4", fps=8)
```

#### encode_prompt[[diffusers.CogVideoXPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] | None = None, do_classifier_free_guidance: bool = True, num_videos_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 226, device: typing.Optional[torch.device] = None, dtype: typing.Optional[torch.dtype] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cogvideo/pipeline_cogvideox.py#L244)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : Whether to use classifier free guidance or not.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : Number of videos that should be generated per prompt. torch device to place the resulting embeddings on

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

device : (`torch.device`, *optional*): torch device

dtype : (`torch.dtype`, *optional*): torch dtype

Encodes the prompt into text encoder hidden states.

#### fuse_qkv_projections[[diffusers.CogVideoXPipeline.fuse_qkv_projections]]

```python
fuse_qkv_projections()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cogvideo/pipeline_cogvideox.py#L428)

Enables fused QKV projections.

#### unfuse_qkv_projections[[diffusers.CogVideoXPipeline.unfuse_qkv_projections]]

```python
unfuse_qkv_projections()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cogvideo/pipeline_cogvideox.py#L433)

Disable QKV projection fusion if enabled.

## CogVideoXImageToVideoPipeline[[diffusers.CogVideoXImageToVideoPipeline]]

#### diffusers.CogVideoXImageToVideoPipeline[[diffusers.CogVideoXImageToVideoPipeline]]

```python
diffusers.CogVideoXImageToVideoPipeline(tokenizer: T5Tokenizer, text_encoder: T5EncoderModel, vae: AutoencoderKLCogVideoX, transformer: CogVideoXTransformer3DModel, scheduler: diffusers.schedulers.scheduling_ddim_cogvideox.CogVideoXDDIMScheduler | diffusers.schedulers.scheduling_dpm_cogvideox.CogVideoXDPMScheduler)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cogvideo/pipeline_cogvideox_image2video.py#L160)

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

text_encoder (`T5EncoderModel`) : Frozen text-encoder. CogVideoX uses [T5](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5EncoderModel); specifically the [t5-v1_1-xxl](https://huggingface.co/PixArt-alpha/PixArt-alpha/tree/main/t5-v1_1-xxl) variant.

tokenizer (`T5Tokenizer`) : Tokenizer of class [T5Tokenizer](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5Tokenizer).

transformer ([CogVideoXTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/cogvideox_transformer3d#diffusers.CogVideoXTransformer3DModel)) : A text conditioned `CogVideoXTransformer3DModel` to denoise the encoded video latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `transformer` to denoise the encoded video latents.

Pipeline for image-to-video generation using CogVideoX.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.CogVideoXImageToVideoPipeline.__call__]]

```python
__call__(image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]], prompt: str | list[str] | None = None, negative_prompt: str | list[str] | None = None, height: int | None = None, width: int | None = None, num_frames: int = 49, num_inference_steps: int = 50, timesteps: list[int] | None = None, guidance_scale: float = 6, use_dynamic_cfg: bool = False, num_videos_per_prompt: int = 1, eta: float = 0.0, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.FloatTensor] = None, prompt_embeds: typing.Optional[torch.FloatTensor] = None, negative_prompt_embeds: typing.Optional[torch.FloatTensor] = None, output_type: str = 'pil', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 226)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cogvideo/pipeline_cogvideox_image2video.py#L598)

**Parameters:**

image (`PipelineImageInput`) : The input image to condition the generation on. Must be an image, a list of images or a `torch.Tensor`.

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

height (`int`, *optional*, defaults to self.transformer.config.sample_height * self.vae_scale_factor_spatial) : The height in pixels of the generated image. This is set to 480 by default for the best results.

width (`int`, *optional*, defaults to self.transformer.config.sample_height * self.vae_scale_factor_spatial) : The width in pixels of the generated image. This is set to 720 by default for the best results.

num_frames (`int`, defaults to `48`) : Number of frames to generate. Must be divisible by self.vae_scale_factor_temporal. Generated video will contain 1 extra frame because CogVideoX is conditioned with (num_seconds * fps + 1) frames where num_seconds is 6 and fps is 8. However, since videos can be saved at any fps, the only condition that needs to be satisfied is that of divisibility mentioned above.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

timesteps (`list[int]`, *optional*) : Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used. Must be in descending order.

guidance_scale (`float`, *optional*, defaults to 7.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

use_dynamic_cfg (`bool`, *optional*, defaults to `False`) : If True, dynamically adjusts the guidance scale during inference.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : The number of videos to generate per prompt.

eta (`float`, *optional*, defaults to 0.0) : Corresponds to parameter eta (η) from the [DDIM](https://arxiv.org/abs/2010.02502) paper. Only applies to [DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), and is ignored in other schedulers.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.FloatTensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.stable_diffusion_xl.StableDiffusionXLPipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int`, defaults to `226`) : Maximum sequence length in encoded prompt. Must be consistent with `self.transformer.config.max_text_seq_length` otherwise may lead to poor results.

**Returns:** [CogVideoXPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/cogvideox#diffusers.pipelines.cogvideo.pipeline_output.CogVideoXPipelineOutput) or `tuple`

[CogVideoXPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/cogvideox#diffusers.pipelines.cogvideo.pipeline_output.CogVideoXPipelineOutput) if `return_dict` is True, otherwise a
`tuple`. When returning a tuple, the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import CogVideoXImageToVideoPipeline
>>> from diffusers.utils import export_to_video, load_image

>>> pipe = CogVideoXImageToVideoPipeline.from_pretrained("THUDM/CogVideoX-5b-I2V", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> prompt = "An astronaut hatching from an egg, on the surface of the moon, the darkness and depth of space realised in the background. High quality, ultrarealistic detail and breath-taking movie-like camera shot."
>>> image = load_image(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/astronaut.jpg"
... )
>>> video = pipe(image, prompt, use_dynamic_cfg=True)
>>> export_to_video(video.frames[0], "output.mp4", fps=8)
```

#### encode_prompt[[diffusers.CogVideoXImageToVideoPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] | None = None, do_classifier_free_guidance: bool = True, num_videos_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 226, device: typing.Optional[torch.device] = None, dtype: typing.Optional[torch.dtype] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cogvideo/pipeline_cogvideox_image2video.py#L263)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : Whether to use classifier free guidance or not.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : Number of videos that should be generated per prompt. torch device to place the resulting embeddings on

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

device : (`torch.device`, *optional*): torch device

dtype : (`torch.dtype`, *optional*): torch dtype

Encodes the prompt into text encoder hidden states.

#### fuse_qkv_projections[[diffusers.CogVideoXImageToVideoPipeline.fuse_qkv_projections]]

```python
fuse_qkv_projections()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cogvideo/pipeline_cogvideox_image2video.py#L519)

Enables fused QKV projections.

#### unfuse_qkv_projections[[diffusers.CogVideoXImageToVideoPipeline.unfuse_qkv_projections]]

```python
unfuse_qkv_projections()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cogvideo/pipeline_cogvideox_image2video.py#L525)

Disable QKV projection fusion if enabled.

## CogVideoXVideoToVideoPipeline[[diffusers.CogVideoXVideoToVideoPipeline]]

#### diffusers.CogVideoXVideoToVideoPipeline[[diffusers.CogVideoXVideoToVideoPipeline]]

```python
diffusers.CogVideoXVideoToVideoPipeline(tokenizer: T5Tokenizer, text_encoder: T5EncoderModel, vae: AutoencoderKLCogVideoX, transformer: CogVideoXTransformer3DModel, scheduler: diffusers.schedulers.scheduling_ddim_cogvideox.CogVideoXDDIMScheduler | diffusers.schedulers.scheduling_dpm_cogvideox.CogVideoXDPMScheduler)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cogvideo/pipeline_cogvideox_video2video.py#L169)

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

text_encoder (`T5EncoderModel`) : Frozen text-encoder. CogVideoX uses [T5](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5EncoderModel); specifically the [t5-v1_1-xxl](https://huggingface.co/PixArt-alpha/PixArt-alpha/tree/main/t5-v1_1-xxl) variant.

tokenizer (`T5Tokenizer`) : Tokenizer of class [T5Tokenizer](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5Tokenizer).

transformer ([CogVideoXTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/cogvideox_transformer3d#diffusers.CogVideoXTransformer3DModel)) : A text conditioned `CogVideoXTransformer3DModel` to denoise the encoded video latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `transformer` to denoise the encoded video latents.

Pipeline for video-to-video generation using CogVideoX.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.CogVideoXVideoToVideoPipeline.__call__]]

```python
__call__(video: list = None, prompt: str | list[str] | None = None, negative_prompt: str | list[str] | None = None, height: int | None = None, width: int | None = None, num_inference_steps: int = 50, timesteps: list[int] | None = None, strength: float = 0.8, guidance_scale: float = 6, use_dynamic_cfg: bool = False, num_videos_per_prompt: int = 1, eta: float = 0.0, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.FloatTensor] = None, prompt_embeds: typing.Optional[torch.FloatTensor] = None, negative_prompt_embeds: typing.Optional[torch.FloatTensor] = None, output_type: str = 'pil', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 226)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cogvideo/pipeline_cogvideox_video2video.py#L575)

**Parameters:**

video (`list[PIL.Image.Image]`) : The input video to condition the generation on. Must be a list of images/frames of the video.

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

height (`int`, *optional*, defaults to self.transformer.config.sample_height * self.vae_scale_factor_spatial) : The height in pixels of the generated image. This is set to 480 by default for the best results.

width (`int`, *optional*, defaults to self.transformer.config.sample_height * self.vae_scale_factor_spatial) : The width in pixels of the generated image. This is set to 720 by default for the best results.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

timesteps (`list[int]`, *optional*) : Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used. Must be in descending order.

strength (`float`, *optional*, defaults to 0.8) : Higher strength leads to more differences between original video and generated video.

guidance_scale (`float`, *optional*, defaults to 7.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

use_dynamic_cfg (`bool`, *optional*, defaults to `False`) : If True, dynamically adjusts the guidance scale during inference.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : The number of videos to generate per prompt.

eta (`float`, *optional*, defaults to 0.0) : Corresponds to parameter eta (η) from the [DDIM](https://arxiv.org/abs/2010.02502) paper. Only applies to [DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), and is ignored in other schedulers.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.FloatTensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.stable_diffusion_xl.StableDiffusionXLPipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int`, defaults to `226`) : Maximum sequence length in encoded prompt. Must be consistent with `self.transformer.config.max_text_seq_length` otherwise may lead to poor results.

**Returns:** [CogVideoXPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/cogvideox#diffusers.pipelines.cogvideo.pipeline_output.CogVideoXPipelineOutput) or `tuple`

[CogVideoXPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/cogvideox#diffusers.pipelines.cogvideo.pipeline_output.CogVideoXPipelineOutput) if `return_dict` is True, otherwise a
`tuple`. When returning a tuple, the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```python
>>> import torch
>>> from diffusers import CogVideoXDPMScheduler, CogVideoXVideoToVideoPipeline
>>> from diffusers.utils import export_to_video, load_video

>>> # Models: "THUDM/CogVideoX-2b" or "THUDM/CogVideoX-5b"
>>> pipe = CogVideoXVideoToVideoPipeline.from_pretrained("THUDM/CogVideoX-5b", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")
>>> pipe.scheduler = CogVideoXDPMScheduler.from_config(pipe.scheduler.config)

>>> input_video = load_video(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/hiker.mp4"
... )
>>> prompt = (
...     "An astronaut stands triumphantly at the peak of a towering mountain. Panorama of rugged peaks and "
...     "valleys. Very futuristic vibe and animated aesthetic. Highlights of purple and golden colors in "
...     "the scene. The sky is looks like an animated/cartoonish dream of galaxies, nebulae, stars, planets, "
...     "moons, but the remainder of the scene is mostly realistic."
... )

>>> video = pipe(
...     video=input_video, prompt=prompt, strength=0.8, guidance_scale=6, num_inference_steps=50
... ).frames[0]
>>> export_to_video(video, "output.mp4", fps=8)
```

#### encode_prompt[[diffusers.CogVideoXVideoToVideoPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] | None = None, do_classifier_free_guidance: bool = True, num_videos_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 226, device: typing.Optional[torch.device] = None, dtype: typing.Optional[torch.dtype] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cogvideo/pipeline_cogvideox_video2video.py#L269)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : Whether to use classifier free guidance or not.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : Number of videos that should be generated per prompt. torch device to place the resulting embeddings on

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

device : (`torch.device`, *optional*): torch device

dtype : (`torch.dtype`, *optional*): torch dtype

Encodes the prompt into text encoder hidden states.

#### fuse_qkv_projections[[diffusers.CogVideoXVideoToVideoPipeline.fuse_qkv_projections]]

```python
fuse_qkv_projections()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cogvideo/pipeline_cogvideox_video2video.py#L496)

Enables fused QKV projections.

#### unfuse_qkv_projections[[diffusers.CogVideoXVideoToVideoPipeline.unfuse_qkv_projections]]

```python
unfuse_qkv_projections()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cogvideo/pipeline_cogvideox_video2video.py#L502)

Disable QKV projection fusion if enabled.

## CogVideoXFunControlPipeline[[diffusers.CogVideoXFunControlPipeline]]

#### diffusers.CogVideoXFunControlPipeline[[diffusers.CogVideoXFunControlPipeline]]

```python
diffusers.CogVideoXFunControlPipeline(tokenizer: T5Tokenizer, text_encoder: T5EncoderModel, vae: AutoencoderKLCogVideoX, transformer: CogVideoXTransformer3DModel, scheduler: KarrasDiffusionSchedulers)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cogvideo/pipeline_cogvideox_fun_control.py#L154)

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

text_encoder (`T5EncoderModel`) : Frozen text-encoder. CogVideoX uses [T5](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5EncoderModel); specifically the [t5-v1_1-xxl](https://huggingface.co/PixArt-alpha/PixArt-alpha/tree/main/t5-v1_1-xxl) variant.

tokenizer (`T5Tokenizer`) : Tokenizer of class [T5Tokenizer](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5Tokenizer).

transformer ([CogVideoXTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/cogvideox_transformer3d#diffusers.CogVideoXTransformer3DModel)) : A text conditioned `CogVideoXTransformer3DModel` to denoise the encoded video latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `transformer` to denoise the encoded video latents.

Pipeline for controlled text-to-video generation using CogVideoX Fun.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.CogVideoXFunControlPipeline.__call__]]

```python
__call__(prompt: str | list[str] | None = None, negative_prompt: str | list[str] | None = None, control_video: list[PIL.Image.Image] | None = None, height: int | None = None, width: int | None = None, num_inference_steps: int = 50, timesteps: list[int] | None = None, guidance_scale: float = 6, use_dynamic_cfg: bool = False, num_videos_per_prompt: int = 1, eta: float = 0.0, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, control_video_latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, output_type: str = 'pil', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 226)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cogvideo/pipeline_cogvideox_fun_control.py#L551)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

control_video (`list[PIL.Image.Image]`) : The control video to condition the generation on. Must be a list of images/frames of the video. If not provided, `control_video_latents` must be provided.

height (`int`, *optional*, defaults to self.transformer.config.sample_height * self.vae_scale_factor_spatial) : The height in pixels of the generated image. This is set to 480 by default for the best results.

width (`int`, *optional*, defaults to self.transformer.config.sample_height * self.vae_scale_factor_spatial) : The width in pixels of the generated image. This is set to 720 by default for the best results.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

timesteps (`list[int]`, *optional*) : Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used. Must be in descending order.

guidance_scale (`float`, *optional*, defaults to 6.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

use_dynamic_cfg (`bool`, *optional*, defaults to `False`) : If True, dynamically adjusts the guidance scale during inference.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : The number of videos to generate per prompt.

eta (`float`, *optional*, defaults to 0.0) : Corresponds to parameter eta (η) from the [DDIM](https://arxiv.org/abs/2010.02502) paper. Only applies to [DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), and is ignored in other schedulers.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for video generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

control_video_latents (`torch.Tensor`, *optional*) : Pre-generated control latents, sampled from a Gaussian distribution, to be used as inputs for controlled video generation. If not provided, `control_video` must be provided.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.stable_diffusion_xl.StableDiffusionXLPipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int`, defaults to `226`) : Maximum sequence length in encoded prompt. Must be consistent with `self.transformer.config.max_text_seq_length` otherwise may lead to poor results.

**Returns:** [CogVideoXPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/cogvideox#diffusers.pipelines.cogvideo.pipeline_output.CogVideoXPipelineOutput) or `tuple`

[CogVideoXPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/cogvideox#diffusers.pipelines.cogvideo.pipeline_output.CogVideoXPipelineOutput) if `return_dict` is True, otherwise a
`tuple`. When returning a tuple, the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```python
>>> import torch
>>> from diffusers import CogVideoXFunControlPipeline, DDIMScheduler
>>> from diffusers.utils import export_to_video, load_video

>>> pipe = CogVideoXFunControlPipeline.from_pretrained(
...     "alibaba-pai/CogVideoX-Fun-V1.1-5b-Pose", torch_dtype=torch.bfloat16
... )
>>> pipe.scheduler = DDIMScheduler.from_config(pipe.scheduler.config)
>>> pipe.to("cuda")

>>> control_video = load_video(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/hiker.mp4"
... )
>>> prompt = (
...     "An astronaut stands triumphantly at the peak of a towering mountain. Panorama of rugged peaks and "
...     "valleys. Very futuristic vibe and animated aesthetic. Highlights of purple and golden colors in "
...     "the scene. The sky is looks like an animated/cartoonish dream of galaxies, nebulae, stars, planets, "
...     "moons, but the remainder of the scene is mostly realistic."
... )

>>> video = pipe(prompt=prompt, control_video=control_video).frames[0]
>>> export_to_video(video, "output.mp4", fps=8)
```

#### encode_prompt[[diffusers.CogVideoXFunControlPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] | None = None, do_classifier_free_guidance: bool = True, num_videos_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 226, device: typing.Optional[torch.device] = None, dtype: typing.Optional[torch.dtype] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cogvideo/pipeline_cogvideox_fun_control.py#L253)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : Whether to use classifier free guidance or not.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : Number of videos that should be generated per prompt. torch device to place the resulting embeddings on

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

device : (`torch.device`, *optional*): torch device

dtype : (`torch.dtype`, *optional*): torch dtype

Encodes the prompt into text encoder hidden states.

#### fuse_qkv_projections[[diffusers.CogVideoXFunControlPipeline.fuse_qkv_projections]]

```python
fuse_qkv_projections()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cogvideo/pipeline_cogvideox_fun_control.py#L473)

Enables fused QKV projections.

#### unfuse_qkv_projections[[diffusers.CogVideoXFunControlPipeline.unfuse_qkv_projections]]

```python
unfuse_qkv_projections()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cogvideo/pipeline_cogvideox_fun_control.py#L478)

Disable QKV projection fusion if enabled.

## CogVideoXPipelineOutput[[diffusers.pipelines.cogvideo.pipeline_output.CogVideoXPipelineOutput]]

#### diffusers.pipelines.cogvideo.pipeline_output.CogVideoXPipelineOutput[[diffusers.pipelines.cogvideo.pipeline_output.CogVideoXPipelineOutput]]

```python
diffusers.pipelines.cogvideo.pipeline_output.CogVideoXPipelineOutput(frames: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cogvideo/pipeline_output.py#L9)

**Parameters:**

frames (`torch.Tensor`, `np.ndarray`, or list[list[PIL.Image.Image]]) : list of video outputs - It can be a nested list of length `batch_size,` with each sub-list containing denoised PIL image sequences of length `num_frames.` It can also be a NumPy array or Torch tensor of shape `(batch_size, num_frames, channels, height, width)`.

Output class for CogVideo pipelines.
