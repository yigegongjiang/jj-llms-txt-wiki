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

  
    
      
    
  

# ChronoEdit

[ChronoEdit: Towards Temporal Reasoning for Image Editing and World Simulation](https://huggingface.co/papers/2510.04290) from NVIDIA and University of Toronto, by Jay Zhangjie Wu, Xuanchi Ren, Tianchang Shen, Tianshi Cao, Kai He, Yifan Lu, Ruiyuan Gao, Enze Xie, Shiyi Lan, Jose M. Alvarez, Jun Gao, Sanja Fidler, Zian Wang, Huan Ling.

> **TL;DR:** ChronoEdit reframes image editing as a video generation task, using input and edited images as start/end frames to leverage pretrained video models with temporal consistency. A temporal reasoning stage introduces reasoning tokens to ensure physically plausible edits and visualize the editing trajectory.

*Recent advances in large generative models have greatly enhanced both image editing and in-context image generation, yet a critical gap remains in ensuring physical consistency, where edited objects must remain coherent. This capability is especially vital for world simulation related tasks. In this paper, we present ChronoEdit, a framework that reframes image editing as a video generation problem. First, ChronoEdit treats the input and edited images as the first and last frames of a video, allowing it to leverage large pretrained video generative models that capture not only object appearance but also the implicit physics of motion and interaction through learned temporal consistency. Second, ChronoEdit introduces a temporal reasoning stage that explicitly performs editing at inference time. Under this setting, target frame is jointly denoised with reasoning tokens to imagine a plausible editing trajectory that constrains the solution space to physically viable transformations. The reasoning tokens are then dropped after a few steps to avoid the high computational cost of rendering a full video. To validate ChronoEdit, we introduce PBench-Edit, a new benchmark of image-prompt pairs for contexts that require physical consistency, and demonstrate that ChronoEdit surpasses state-of-the-art baselines in both visual fidelity and physical plausibility. Project page for code and models: [this https URL](https://research.nvidia.com/labs/toronto-ai/chronoedit).*

The ChronoEdit pipeline is developed by the ChronoEdit Team. The original code is available on [GitHub](https://github.com/nv-tlabs/ChronoEdit), and pretrained models can be found in the [nvidia/ChronoEdit](https://huggingface.co/collections/nvidia/chronoedit) collection on Hugging Face.

Available Models/LoRAs:
- [nvidia/ChronoEdit-14B-Diffusers](https://huggingface.co/nvidia/ChronoEdit-14B-Diffusers)
- [nvidia/ChronoEdit-14B-Diffusers-Upscaler-Lora](https://huggingface.co/nvidia/ChronoEdit-14B-Diffusers-Upscaler-Lora)
- [nvidia/ChronoEdit-14B-Diffusers-Paint-Brush-Lora](https://huggingface.co/nvidia/ChronoEdit-14B-Diffusers-Paint-Brush-Lora)

### Image Editing

```py
import torch
import numpy as np
from diffusers import AutoencoderKLWan, ChronoEditTransformer3DModel, ChronoEditPipeline
from diffusers.utils import export_to_video, load_image
from transformers import CLIPVisionModel
from PIL import Image

model_id = "nvidia/ChronoEdit-14B-Diffusers"
image_encoder = CLIPVisionModel.from_pretrained(model_id, subfolder="image_encoder", dtype=torch.float32)
vae = AutoencoderKLWan.from_pretrained(model_id, subfolder="vae", dtype=torch.float32)
transformer = ChronoEditTransformer3DModel.from_pretrained(model_id, subfolder="transformer", dtype=torch.bfloat16)
pipe = ChronoEditPipeline.from_pretrained(model_id, image_encoder=image_encoder, transformer=transformer, vae=vae, dtype=torch.bfloat16)
pipe.to("cuda")

image = load_image(
    "https://huggingface.co/spaces/nvidia/ChronoEdit/resolve/main/examples/3.png"
)
max_area = 720 * 1280
aspect_ratio = image.height / image.width
mod_value = pipe.vae_scale_factor_spatial * pipe.transformer.config.patch_size[1]
height = round(np.sqrt(max_area * aspect_ratio)) // mod_value * mod_value
width = round(np.sqrt(max_area / aspect_ratio)) // mod_value * mod_value
print("width", width, "height", height)
image = image.resize((width, height))
prompt = (
    "The user wants to transform the image by adding a small, cute mouse sitting inside the floral teacup, enjoying a spa bath. The mouse should appear relaxed and cheerful, with a tiny white bath towel draped over its head like a turban. It should be positioned comfortably in the cup’s liquid, with gentle steam rising around it to blend with the cozy atmosphere. "
    "The mouse’s pose should be natural—perhaps sitting upright with paws resting lightly on the rim or submerged in the tea. The teacup’s floral design, gold trim, and warm lighting must remain unchanged to preserve the original aesthetic. The steam should softly swirl around the mouse, enhancing the spa-like, whimsical mood."
)

output = pipe(
    image=image,
    prompt=prompt,
    height=height,
    width=width,
    num_frames=5,
    num_inference_steps=50,
    guidance_scale=5.0,
    enable_temporal_reasoning=False,
    num_temporal_reasoning_steps=0,
).frames[0]
Image.fromarray((output[-1] * 255).clip(0, 255).astype("uint8")).save("output.png")
```

Optionally, enable **temporal reasoning** for improved physical consistency:
```py
output = pipe(
    image=image,
    prompt=prompt,
    height=height,
    width=width,
    num_frames=29,
    num_inference_steps=50,
    guidance_scale=5.0,
    enable_temporal_reasoning=True,
    num_temporal_reasoning_steps=50,
).frames[0]
export_to_video(output, "output.mp4", fps=16)
Image.fromarray((output[-1] * 255).clip(0, 255).astype("uint8")).save("output.png")
```

### Inference with 8-Step Distillation Lora

```py
import torch
import numpy as np
from diffusers import AutoencoderKLWan, ChronoEditTransformer3DModel, ChronoEditPipeline
from diffusers.schedulers import UniPCMultistepScheduler
from diffusers.utils import export_to_video, load_image
from transformers import CLIPVisionModel
from PIL import Image

model_id = "nvidia/ChronoEdit-14B-Diffusers"
image_encoder = CLIPVisionModel.from_pretrained(model_id, subfolder="image_encoder", dtype=torch.float32)
vae = AutoencoderKLWan.from_pretrained(model_id, subfolder="vae", dtype=torch.float32)
transformer = ChronoEditTransformer3DModel.from_pretrained(model_id, subfolder="transformer", dtype=torch.bfloat16)
pipe = ChronoEditPipeline.from_pretrained(model_id, image_encoder=image_encoder, transformer=transformer, vae=vae, dtype=torch.bfloat16)
pipe.load_lora_weights("nvidia/ChronoEdit-14B-Diffusers", weight_name="lora/chronoedit_distill_lora.safetensors", adapter_name="distill")
pipe.fuse_lora(adapter_names=["distill"], lora_scale=1.0)
pipe.scheduler = UniPCMultistepScheduler.from_config(pipe.scheduler.config, flow_shift=2.0)
pipe.to("cuda")

image = load_image(
    "https://huggingface.co/spaces/nvidia/ChronoEdit/resolve/main/examples/3.png"
)
max_area = 720 * 1280
aspect_ratio = image.height / image.width
mod_value = pipe.vae_scale_factor_spatial * pipe.transformer.config.patch_size[1]
height = round(np.sqrt(max_area * aspect_ratio)) // mod_value * mod_value
width = round(np.sqrt(max_area / aspect_ratio)) // mod_value * mod_value
print("width", width, "height", height)
image = image.resize((width, height))
prompt = (
    "The user wants to transform the image by adding a small, cute mouse sitting inside the floral teacup, enjoying a spa bath. The mouse should appear relaxed and cheerful, with a tiny white bath towel draped over its head like a turban. It should be positioned comfortably in the cup’s liquid, with gentle steam rising around it to blend with the cozy atmosphere. "
    "The mouse’s pose should be natural—perhaps sitting upright with paws resting lightly on the rim or submerged in the tea. The teacup’s floral design, gold trim, and warm lighting must remain unchanged to preserve the original aesthetic. The steam should softly swirl around the mouse, enhancing the spa-like, whimsical mood."
)

output = pipe(
    image=image,
    prompt=prompt,
    height=height,
    width=width,
    num_frames=5,
    num_inference_steps=8,
    guidance_scale=1.0,
    enable_temporal_reasoning=False,
    num_temporal_reasoning_steps=0,
).frames[0]
export_to_video(output, "output.mp4", fps=16)
Image.fromarray((output[-1] * 255).clip(0, 255).astype("uint8")).save("output.png")
```

### Inference with Multiple LoRAs

```py
import torch
import numpy as np
from diffusers import AutoencoderKLWan, ChronoEditTransformer3DModel, ChronoEditPipeline
from diffusers.schedulers import UniPCMultistepScheduler
from diffusers.utils import export_to_video, load_image
from transformers import CLIPVisionModel
from PIL import Image

model_id = "nvidia/ChronoEdit-14B-Diffusers"
image_encoder = CLIPVisionModel.from_pretrained(model_id, subfolder="image_encoder", dtype=torch.float32)
vae = AutoencoderKLWan.from_pretrained(model_id, subfolder="vae", dtype=torch.float32)
transformer = ChronoEditTransformer3DModel.from_pretrained(model_id, subfolder="transformer", dtype=torch.bfloat16)
pipe = ChronoEditPipeline.from_pretrained(model_id, image_encoder=image_encoder, transformer=transformer, vae=vae, dtype=torch.bfloat16)
pipe.load_lora_weights("nvidia/ChronoEdit-14B-Diffusers-Paint-Brush-Lora", weight_name="paintbrush_lora_diffusers.safetensors", adapter_name="paintbrush")
pipe.load_lora_weights("nvidia/ChronoEdit-14B-Diffusers", weight_name="lora/chronoedit_distill_lora.safetensors", adapter_name="distill")
pipe.fuse_lora(adapter_names=["paintbrush", "distill"], lora_scale=1.0)
pipe.scheduler = UniPCMultistepScheduler.from_config(pipe.scheduler.config, flow_shift=2.0)
pipe.to("cuda")

image = load_image(
    "https://raw.githubusercontent.com/nv-tlabs/ChronoEdit/refs/heads/main/assets/images/input_paintbrush.png"
)
max_area = 720 * 1280
aspect_ratio = image.height / image.width
mod_value = pipe.vae_scale_factor_spatial * pipe.transformer.config.patch_size[1]
height = round(np.sqrt(max_area * aspect_ratio)) // mod_value * mod_value
width = round(np.sqrt(max_area / aspect_ratio)) // mod_value * mod_value
print("width", width, "height", height)
image = image.resize((width, height))
prompt = (
    "Turn the pencil sketch in the image into an actual object that is consistent with the image’s content. The user wants to change the sketch to a crown and a hat."
)

output = pipe(
    image=image,
    prompt=prompt,
    height=height,
    width=width,
    num_frames=5,
    num_inference_steps=8,
    guidance_scale=1.0,
    enable_temporal_reasoning=False,
    num_temporal_reasoning_steps=0,
).frames[0]
export_to_video(output, "output.mp4", fps=16)
Image.fromarray((output[-1] * 255).clip(0, 255).astype("uint8")).save("output_1.png")
```

## ChronoEditPipeline[[diffusers.ChronoEditPipeline]]

#### diffusers.ChronoEditPipeline[[diffusers.ChronoEditPipeline]]

```python
diffusers.ChronoEditPipeline(tokenizer: AutoTokenizer, text_encoder: UMT5EncoderModel, image_encoder: CLIPVisionModel, image_processor: CLIPImageProcessorPil, transformer: ChronoEditTransformer3DModel, vae: AutoencoderKLWan, scheduler: FlowMatchEulerDiscreteScheduler)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/chronoedit/pipeline_chronoedit.py#L128)

**Parameters:**

tokenizer (`T5Tokenizer`) : Tokenizer from [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5Tokenizer), specifically the [google/umt5-xxl](https://huggingface.co/google/umt5-xxl) variant.

text_encoder (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/umt5-xxl](https://huggingface.co/google/umt5-xxl) variant.

image_encoder (`CLIPVisionModel`) : [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPVisionModel), specifically the [clip-vit-huge-patch14](https://github.com/mlfoundations/open_clip/blob/main/docs/PRETRAINED.md#vit-h14-xlm-roberta-large) variant.

transformer ([WanTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/wan_transformer_3d#diffusers.WanTransformer3DModel)) : Conditional Transformer to denoise the input latents.

scheduler ([UniPCMultistepScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/unipc#diffusers.UniPCMultistepScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLWan](/docs/diffusers/v0.40.0/en/api/models/autoencoder_kl_wan#diffusers.AutoencoderKLWan)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

Pipeline for image-to-video generation using Wan.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

#### __call__[[diffusers.ChronoEditPipeline.__call__]]

```python
__call__(image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]], prompt: str | list[str] = None, negative_prompt: str | list[str] = None, height: int = 480, width: int = 832, num_frames: int = 81, num_inference_steps: int = 50, guidance_scale: float = 5.0, num_videos_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, image_embeds: typing.Optional[torch.Tensor] = None, output_type: str | None = 'np', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int, NoneType], diffusers.callbacks.PipelineCallback | diffusers.callbacks.MultiPipelineCallbacks]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512, enable_temporal_reasoning: bool = False, num_temporal_reasoning_steps: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/chronoedit/pipeline_chronoedit.py#L467)

**Parameters:**

image (`PipelineImageInput`) : The input image to condition the generation on. Must be an image, a list of images or a `torch.Tensor`.

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

height (`int`, defaults to `480`) : The height of the generated video.

width (`int`, defaults to `832`) : The width of the generated video.

num_frames (`int`, defaults to `81`) : The number of frames in the generated video.

num_inference_steps (`int`, defaults to `50`) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, defaults to `5.0`) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://arxiv.org/abs/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://arxiv.org/pdf/2205.11487.pdf). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor is generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, text embeddings are generated from the `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, text embeddings are generated from the `negative_prompt` input argument.

image_embeds (`torch.Tensor`, *optional*) : Pre-generated image embeddings. Can be used to easily tweak image inputs (weighting). If not provided, image embeddings are generated from the `image` input argument.

output_type (`str`, *optional*, defaults to `"np"`) : The output format of the generated image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `ChronoEditPipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, `PipelineCallback`, `MultiPipelineCallbacks`, *optional*) : A function or a subclass of `PipelineCallback` or `MultiPipelineCallbacks` that is called at the end of each denoising step during the inference. with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`List`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int`, defaults to `512`) : The maximum sequence length of the text encoder. If the prompt is longer than this, it will be truncated. If the prompt is shorter, it will be padded to this length.

enable_temporal_reasoning (`bool`, *optional*, defaults to `False`) : Whether to enable temporal reasoning.

num_temporal_reasoning_steps (`int`, *optional*, defaults to `0`) : The number of steps to enable temporal reasoning.

**Returns:** `~ChronoEditPipelineOutput` or `tuple`

If `return_dict` is `True`, `ChronoEditPipelineOutput` is returned, otherwise a `tuple` is returned
where the first element is a list with the generated images and the second element is a list of `bool`s
indicating whether the corresponding generated image contains "not-safe-for-work" (nsfw) content.

The call function to the pipeline for generation.

Examples:
```python
>>> import torch
>>> import numpy as np
>>> from diffusers import AutoencoderKLWan, ChronoEditTransformer3DModel, ChronoEditPipeline
>>> from diffusers.utils import export_to_video, load_image
>>> from transformers import CLIPVisionModel

>>> # Available models: nvidia/ChronoEdit-14B-Diffusers
>>> model_id = "nvidia/ChronoEdit-14B-Diffusers"
>>> image_encoder = CLIPVisionModel.from_pretrained(
...     model_id, subfolder="image_encoder", torch_dtype=torch.float32
... )
>>> vae = AutoencoderKLWan.from_pretrained(model_id, subfolder="vae", torch_dtype=torch.float32)
>>> transformer = ChronoEditTransformer3DModel.from_pretrained(
...     model_id, subfolder="transformer", torch_dtype=torch.bfloat16
... )
>>> pipe = ChronoEditPipeline.from_pretrained(
...     model_id, vae=vae, image_encoder=image_encoder, transformer=transformer, torch_dtype=torch.bfloat16
... )
>>> pipe.to("cuda")

>>> image = load_image("https://huggingface.co/spaces/nvidia/ChronoEdit/resolve/main/examples/3.png")
>>> max_area = 720 * 1280
>>> aspect_ratio = image.height / image.width
>>> mod_value = pipe.vae_scale_factor_spatial * pipe.transformer.config.patch_size[1]
>>> height = round(np.sqrt(max_area * aspect_ratio)) // mod_value * mod_value
>>> width = round(np.sqrt(max_area / aspect_ratio)) // mod_value * mod_value
>>> image = image.resize((width, height))
>>> prompt = (
...     "The user wants to transform the image by adding a small, cute mouse sitting inside the floral teacup, enjoying a spa bath. The mouse should appear relaxed and cheerful, with a tiny white bath towel draped over its head like a turban. It should be positioned comfortably in the cup’s liquid, with gentle steam rising around it to blend with the cozy atmosphere. "
...     "The mouse’s pose should be natural—perhaps sitting upright with paws resting lightly on the rim or submerged in the tea. The teacup’s floral design, gold trim, and warm lighting must remain unchanged to preserve the original aesthetic. The steam should softly swirl around the mouse, enhancing the spa-like, whimsical mood."
... )

>>> output = pipe(
...     image=image,
...     prompt=prompt,
...     height=height,
...     width=width,
...     num_frames=5,
...     guidance_scale=5.0,
...     enable_temporal_reasoning=False,
...     num_temporal_reasoning_steps=0,
... ).frames[0]
>>> export_to_video(output, "output.mp4", fps=16)
```

#### encode_prompt[[diffusers.ChronoEditPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] | None = None, do_classifier_free_guidance: bool = True, num_videos_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 226, device: typing.Optional[torch.device] = None, dtype: typing.Optional[torch.dtype] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/chronoedit/pipeline_chronoedit.py#L239)

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

## ChronoEditPipelineOutput[[diffusers.pipelines.chronoedit.pipeline_output.ChronoEditPipelineOutput]]

#### diffusers.pipelines.chronoedit.pipeline_output.ChronoEditPipelineOutput[[diffusers.pipelines.chronoedit.pipeline_output.ChronoEditPipelineOutput]]

```python
diffusers.pipelines.chronoedit.pipeline_output.ChronoEditPipelineOutput(frames: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/chronoedit/pipeline_output.py#L9)

**Parameters:**

frames (`torch.Tensor`, `np.ndarray`, or list[list[PIL.Image.Image]]) : List of video outputs - It can be a nested list of length `batch_size,` with each sub-list containing denoised PIL image sequences of length `num_frames.` It can also be a NumPy array or Torch tensor of shape `(batch_size, num_frames, channels, height, width)`.

Output class for ChronoEdit pipelines.
