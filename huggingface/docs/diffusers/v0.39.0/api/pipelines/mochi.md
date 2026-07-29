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

# Mochi 1 Preview

  

> [!TIP]
> Only a research preview of the model weights is available at the moment.

[Mochi 1](https://huggingface.co/genmo/mochi-1-preview) is a video generation model by Genmo with a strong focus on prompt adherence and motion quality. The model features a 10B parameter Asmmetric Diffusion Transformer (AsymmDiT) architecture, and uses non-square QKV and output projection layers to reduce inference memory requirements. A single T5-XXL model is used to encode prompts.

*Mochi 1 preview is an open state-of-the-art video generation model with high-fidelity motion and strong prompt adherence in preliminary evaluation. This model dramatically closes the gap between closed and open video generation systems. The model is released under a permissive Apache 2.0 license.*

> [!TIP]
> Make sure to check out the Schedulers [guide](../../using-diffusers/schedulers) to learn how to explore the tradeoff between scheduler speed and quality, and see the [reuse components across pipelines](../../using-diffusers/loading#reuse-a-pipeline) section to learn how to efficiently load the same components into multiple pipelines.

## Quantization

Quantization helps reduce the memory requirements of very large models by storing model weights in a lower precision data type. However, quantization may have varying impact on video quality depending on the video model.

Refer to the [Quantization](../../quantization/overview) overview to learn more about supported quantization backends and selecting a quantization backend that supports your use case. The example below demonstrates how to load a quantized [MochiPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/mochi#diffusers.MochiPipeline) for inference with bitsandbytes.

```py
import torch
from diffusers import BitsAndBytesConfig as DiffusersBitsAndBytesConfig, MochiTransformer3DModel, MochiPipeline
from diffusers.utils import export_to_video
from transformers import BitsAndBytesConfig as BitsAndBytesConfig, T5EncoderModel

quant_config = BitsAndBytesConfig(load_in_8bit=True)
text_encoder_8bit = T5EncoderModel.from_pretrained(
    "genmo/mochi-1-preview",
    subfolder="text_encoder",
    quantization_config=quant_config,
    torch_dtype=torch.float16,
)

quant_config = DiffusersBitsAndBytesConfig(load_in_8bit=True)
transformer_8bit = MochiTransformer3DModel.from_pretrained(
    "genmo/mochi-1-preview",
    subfolder="transformer",
    quantization_config=quant_config,
    torch_dtype=torch.float16,
)

pipeline = MochiPipeline.from_pretrained(
    "genmo/mochi-1-preview",
    text_encoder=text_encoder_8bit,
    transformer=transformer_8bit,
    torch_dtype=torch.float16,
    device_map="balanced",
)

video = pipeline(
  "Close-up of a cats eye, with the galaxy reflected in the cats eye. Ultra high resolution 4k.",
  num_inference_steps=28,
  guidance_scale=3.5
).frames[0]
export_to_video(video, "cat.mp4")
```

## Generating videos with Mochi-1 Preview

The following example will download the full precision `mochi-1-preview` weights and produce the highest quality results but will require at least 42GB VRAM to run.

```python
import torch
from diffusers import MochiPipeline
from diffusers.utils import export_to_video

pipe = MochiPipeline.from_pretrained("genmo/mochi-1-preview")

# Enable memory savings
pipe.enable_model_cpu_offload()
pipe.enable_vae_tiling()

prompt = "Close-up of a chameleon's eye, with its scaly skin changing color. Ultra high resolution 4k."

with torch.autocast("cuda", torch.bfloat16, cache_enabled=False):
      frames = pipe(prompt, num_frames=85).frames[0]

export_to_video(frames, "mochi.mp4", fps=30)
```

## Using a lower precision variant to save memory

The following example will use the `bfloat16` variant of the model and requires 22GB VRAM to run. There is a slight drop in the quality of the generated video as a result.

```python
import torch
from diffusers import MochiPipeline
from diffusers.utils import export_to_video

pipe = MochiPipeline.from_pretrained("genmo/mochi-1-preview", variant="bf16", torch_dtype=torch.bfloat16)

# Enable memory savings
pipe.enable_model_cpu_offload()
pipe.enable_vae_tiling()

prompt = "Close-up of a chameleon's eye, with its scaly skin changing color. Ultra high resolution 4k."
frames = pipe(prompt, num_frames=85).frames[0]

export_to_video(frames, "mochi.mp4", fps=30)
```

## Reproducing the results from the Genmo Mochi repo

The [Genmo Mochi implementation](https://github.com/genmoai/mochi/tree/main) uses different precision values for each stage in the inference process. The text encoder and VAE use `torch.float32`, while the DiT uses `torch.bfloat16` with the [attention kernel](https://pytorch.org/docs/stable/generated/torch.nn.attention.sdpa_kernel.html#torch.nn.attention.sdpa_kernel) set to `EFFICIENT_ATTENTION`. Diffusers pipelines currently do not support setting different `dtypes` for different stages of the pipeline. In order to run inference in the same way as the original implementation, please refer to the following example.

> [!TIP]
> The original Mochi implementation zeros out empty prompts. However, enabling this option and placing the entire pipeline under autocast can lead to numerical overflows with the T5 text encoder.
>
> When enabling `force_zeros_for_empty_prompt`, it is recommended to run the text encoding step outside the autocast context in full precision.

> [!TIP]
> Decoding the latents in full precision is very memory intensive. You will need at least 70GB VRAM to generate the 163 frames in this example. To reduce memory, either reduce the number of frames or run the decoding step in `torch.bfloat16`.

```python
import torch
from torch.nn.attention import SDPBackend, sdpa_kernel

from diffusers import MochiPipeline
from diffusers.utils import export_to_video
from diffusers.video_processor import VideoProcessor

pipe = MochiPipeline.from_pretrained("genmo/mochi-1-preview", force_zeros_for_empty_prompt=True)
pipe.enable_vae_tiling()
pipe.enable_model_cpu_offload()

prompt =  "An aerial shot of a parade of elephants walking across the African savannah. The camera showcases the herd and the surrounding landscape."

with torch.no_grad():
    prompt_embeds, prompt_attention_mask, negative_prompt_embeds, negative_prompt_attention_mask = (
        pipe.encode_prompt(prompt=prompt)
    )

with torch.autocast("cuda", torch.bfloat16):
    with sdpa_kernel(SDPBackend.EFFICIENT_ATTENTION):
        frames = pipe(
            prompt_embeds=prompt_embeds,
            prompt_attention_mask=prompt_attention_mask,
            negative_prompt_embeds=negative_prompt_embeds,
            negative_prompt_attention_mask=negative_prompt_attention_mask,
            guidance_scale=4.5,
            num_inference_steps=64,
            height=480,
            width=848,
            num_frames=163,
            generator=torch.Generator("cuda").manual_seed(0),
            output_type="latent",
            return_dict=False,
        )[0]

video_processor = VideoProcessor(vae_scale_factor=8)
has_latents_mean = hasattr(pipe.vae.config, "latents_mean") and pipe.vae.config.latents_mean is not None
has_latents_std = hasattr(pipe.vae.config, "latents_std") and pipe.vae.config.latents_std is not None
if has_latents_mean and has_latents_std:
    latents_mean = (
        torch.tensor(pipe.vae.config.latents_mean).view(1, 12, 1, 1, 1).to(frames.device, frames.dtype)
    )
    latents_std = (
        torch.tensor(pipe.vae.config.latents_std).view(1, 12, 1, 1, 1).to(frames.device, frames.dtype)
    )
    frames = frames * latents_std / pipe.vae.config.scaling_factor + latents_mean
else:
    frames = frames / pipe.vae.config.scaling_factor

with torch.no_grad():
    video = pipe.vae.decode(frames.to(pipe.vae.dtype), return_dict=False)[0]

video = video_processor.postprocess_video(video)[0]
export_to_video(video, "mochi.mp4", fps=30)
```

## Running inference with multiple GPUs

It is possible to split the large Mochi transformer across multiple GPUs using the `device_map` and `max_memory` options in `from_pretrained`. In the following example we split the model across two GPUs, each with 24GB of VRAM.

```python
import torch
from diffusers import MochiPipeline, MochiTransformer3DModel
from diffusers.utils import export_to_video

model_id = "genmo/mochi-1-preview"
transformer = MochiTransformer3DModel.from_pretrained(
    model_id,
    subfolder="transformer",
    device_map="auto",
    max_memory={0: "24GB", 1: "24GB"}
)

pipe = MochiPipeline.from_pretrained(model_id,  transformer=transformer)
pipe.enable_model_cpu_offload()
pipe.enable_vae_tiling()

with torch.autocast(device_type="cuda", dtype=torch.bfloat16, cache_enabled=False):
    frames = pipe(
        prompt="Close-up of a chameleon's eye, with its scaly skin changing color. Ultra high resolution 4k.",
        negative_prompt="",
        height=480,
        width=848,
        num_frames=85,
        num_inference_steps=50,
        guidance_scale=4.5,
        num_videos_per_prompt=1,
        generator=torch.Generator(device="cuda").manual_seed(0),
        max_sequence_length=256,
        output_type="pil",
    ).frames[0]

export_to_video(frames, "output.mp4", fps=30)
```

## Using single file loading with the Mochi Transformer

You can use `from_single_file` to load the Mochi transformer in its original format.

> [!TIP]
> Diffusers currently doesn't support using the FP8 scaled versions of the Mochi single file checkpoints.

```python
import torch
from diffusers import MochiPipeline, MochiTransformer3DModel
from diffusers.utils import export_to_video

model_id = "genmo/mochi-1-preview"

ckpt_path = "https://huggingface.co/Comfy-Org/mochi_preview_repackaged/blob/main/split_files/diffusion_models/mochi_preview_bf16.safetensors"

transformer = MochiTransformer3DModel.from_pretrained(ckpt_path, torch_dtype=torch.bfloat16)

pipe = MochiPipeline.from_pretrained(model_id,  transformer=transformer)
pipe.enable_model_cpu_offload()
pipe.enable_vae_tiling()

with torch.autocast(device_type="cuda", dtype=torch.bfloat16, cache_enabled=False):
    frames = pipe(
        prompt="Close-up of a chameleon's eye, with its scaly skin changing color. Ultra high resolution 4k.",
        negative_prompt="",
        height=480,
        width=848,
        num_frames=85,
        num_inference_steps=50,
        guidance_scale=4.5,
        num_videos_per_prompt=1,
        generator=torch.Generator(device="cuda").manual_seed(0),
        max_sequence_length=256,
        output_type="pil",
    ).frames[0]

export_to_video(frames, "output.mp4", fps=30)
```

## MochiPipeline[[diffusers.MochiPipeline]]

#### diffusers.MochiPipeline[[diffusers.MochiPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/mochi/pipeline_mochi.py#L138)

The mochi pipeline for text-to-video generation.

Reference: https://github.com/genmoai/models

__call__diffusers.MochiPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/mochi/pipeline_mochi.py#L497[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_frames", "val": ": int = 19"}, {"name": "num_inference_steps", "val": ": int = 64"}, {"name": "timesteps", "val": ": list = None"}, {"name": "guidance_scale", "val": ": float = 4.5"}, {"name": "num_videos_per_prompt", "val": ": int | None = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 256"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale  1`. Higher guidance scale encourages to generate images that are closely linked to
  the text `prompt`, usually at the expense of lower image quality.
- **num_videos_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of videos to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **prompt_attention_mask** (`torch.Tensor`, *optional*) --
  Pre-generated attention mask for text embeddings.
- **negative_prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated negative text embeddings. For PixArt-Sigma this negative prompt should be "". If not
  provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.
- **negative_prompt_attention_mask** (`torch.FloatTensor`, *optional*) --
  Pre-generated attention mask for negative text embeddings.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.mochi.MochiPipelineOutput` instead of a plain tuple.
- **attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`list`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.
- **max_sequence_length** (`int` defaults to `256`) --
  Maximum sequence length to use with the `prompt`.0`~pipelines.mochi.MochiPipelineOutput` or `tuple`If `return_dict` is `True`, `~pipelines.mochi.MochiPipelineOutput` is returned, otherwise a `tuple`
is returned where the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import MochiPipeline
>>> from diffusers.utils import export_to_video

>>> pipe = MochiPipeline.from_pretrained("genmo/mochi-1-preview", torch_dtype=torch.bfloat16)
>>> pipe.enable_model_cpu_offload()
>>> pipe.enable_vae_tiling()
>>> prompt = "Close-up of a chameleon's eye, with its scaly skin changing color. Ultra high resolution 4k."
>>> frames = pipe(prompt, num_inference_steps=28, guidance_scale=3.5).frames[0]
>>> export_to_video(frames, "mochi.mp4")
```

**Parameters:**

transformer ([MochiTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/mochi_transformer3d#diffusers.MochiTransformer3DModel)) : Conditional Transformer architecture to denoise the encoded video latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLMochi](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl_mochi#diffusers.AutoencoderKLMochi)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

text_encoder (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer (`T5TokenizerFast`) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

**Returns:**

``~pipelines.mochi.MochiPipelineOutput` or `tuple``

If `return_dict` is `True`, `~pipelines.mochi.MochiPipelineOutput` is returned, otherwise a `tuple`
is returned where the first element is a list with the generated images.
#### disable_vae_slicing[[diffusers.MochiPipeline.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/mochi/pipeline_mochi.py#L403)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### disable_vae_tiling[[diffusers.MochiPipeline.disable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/mochi/pipeline_mochi.py#L430)

Disable tiled VAE decoding. If `enable_vae_tiling` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_vae_slicing[[diffusers.MochiPipeline.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/mochi/pipeline_mochi.py#L390)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### enable_vae_tiling[[diffusers.MochiPipeline.enable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/mochi/pipeline_mochi.py#L416)

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.
#### encode_prompt[[diffusers.MochiPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/mochi/pipeline_mochi.py#L254)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : Whether to use classifier free guidance or not.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : Number of videos that should be generated per prompt. torch device to place the resulting embeddings on

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

device : (`torch.device`, *optional*): torch device

dtype : (`torch.dtype`, *optional*): torch dtype

## MochiPipelineOutput[[diffusers.pipelines.mochi.pipeline_output.MochiPipelineOutput]]

#### diffusers.pipelines.mochi.pipeline_output.MochiPipelineOutput[[diffusers.pipelines.mochi.pipeline_output.MochiPipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/mochi/pipeline_output.py#L9)

Output class for Mochi pipelines.

**Parameters:**

frames (`torch.Tensor`, `np.ndarray`, or list[list[PIL.Image.Image]]) : list of video outputs - It can be a nested list of length `batch_size,` with each sub-list containing denoised PIL image sequences of length `num_frames.` It can also be a NumPy array or Torch tensor of shape `(batch_size, num_frames, channels, height, width)`.
