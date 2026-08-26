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

# LTX-2

  

[LTX-2](https://hf.co/papers/2601.03233) is a DiT-based foundation model designed to generate synchronized video and audio within a single model. It brings together the core building blocks of modern video generation, with open weights and a focus on practical, local execution.

You can find all the original LTX-Video checkpoints under the [Lightricks](https://huggingface.co/Lightricks) organization.

The original codebase for LTX-2 can be found [here](https://github.com/Lightricks/LTX-2).

## Two-stages Generation

The shared `LTX2Pipeline` / `LTX2ImageToVideoPipeline` `__call__` defaults match the LTX-2.5 reference (`num_inference_steps=30`; `num_frames` is optional when a `duration_head` is present, otherwise it falls back to `121`). The examples below use those defaults for LTX-2.0/2.3 as well.

Recommended pipeline to achieve production quality generation, this pipeline is composed of two stages:

- Stage 1: Generate a video at the target resolution using diffusion sampling with classifier-free guidance (CFG). This stage produces a coherent low-noise video sequence that respects the text/image conditioning.
- Stage 2: Upsample the Stage 1 output by 2 and refine details using a distilled LoRA model to improve fidelity and visual quality. Stage 2 may apply lighter CFG to preserve the structure from Stage 1 while enhancing texture and sharpness.

Sample usage of text-to-video two stages pipeline

```py
import torch
from diffusers import FlowMatchEulerDiscreteScheduler
from diffusers.pipelines.ltx2 import LTX2Pipeline, LTX2LatentUpsamplePipeline
from diffusers.pipelines.ltx2.latent_upsampler import LTX2LatentUpsamplerModel
from diffusers.pipelines.ltx2.utils import STAGE_2_DISTILLED_SIGMA_VALUES
from diffusers.utils import encode_video

device = "cuda:0"
width = 768
height = 512

pipe = LTX2Pipeline.from_pretrained(
    "Lightricks/LTX-2", dtype=torch.bfloat16
)
pipe.enable_sequential_cpu_offload(device=device)

prompt = "A beautiful sunset over the ocean"
negative_prompt = "shaky, glitchy, low quality, worst quality, deformed, distorted, disfigured, motion smear, motion artifacts, fused fingers, bad anatomy, weird hand, ugly, transition, static."

# Stage 1 default (non-distilled) inference
frame_rate = 24.0
video_latent, audio_latent = pipe(
    prompt=prompt,
    negative_prompt=negative_prompt,
    width=width,
    height=height,
    num_frames=121,
    frame_rate=frame_rate,
    num_inference_steps=30,
    sigmas=None,
    guidance_scale=3.0,
    output_type="latent",
    return_dict=False,
)

latent_upsampler = LTX2LatentUpsamplerModel.from_pretrained(
    "Lightricks/LTX-2",
    subfolder="latent_upsampler",
    dtype=torch.bfloat16,
)
upsample_pipe = LTX2LatentUpsamplePipeline(vae=pipe.vae, latent_upsampler=latent_upsampler)
upsample_pipe.enable_model_cpu_offload(device=device)
upscaled_video_latent = upsample_pipe(
    latents=video_latent,
    output_type="latent",
    return_dict=False,
)[0]

# Load Stage 2 distilled LoRA
pipe.load_lora_weights(
    "Lightricks/LTX-2", adapter_name="stage_2_distilled", weight_name="ltx-2-19b-distilled-lora-384.safetensors"
)
pipe.set_adapters("stage_2_distilled", 1.0)
# VAE tiling is usually necessary to avoid OOM error when VAE decoding
pipe.vae.enable_tiling()
# Change scheduler to use Stage 2 distilled sigmas as is
new_scheduler = FlowMatchEulerDiscreteScheduler.from_config(
    pipe.scheduler.config, use_dynamic_shifting=False, shift_terminal=None
)
pipe.scheduler = new_scheduler
# Stage 2 inference with distilled LoRA and sigmas
video, audio = pipe(
    latents=upscaled_video_latent,
    audio_latents=audio_latent,
    prompt=prompt,
    negative_prompt=negative_prompt,
    num_inference_steps=3,
    noise_scale=STAGE_2_DISTILLED_SIGMA_VALUES[0], # renoise with first sigma value https://github.com/Lightricks/LTX-2/blob/main/packages/ltx-pipelines/src/ltx_pipelines/ti2vid_two_stages.py#L218
    sigmas=STAGE_2_DISTILLED_SIGMA_VALUES,
    guidance_scale=1.0,
    output_type="np",
    return_dict=False,
)

encode_video(
    video[0],
    fps=frame_rate,
    audio=audio[0].float().cpu(),
    audio_sample_rate=pipe.vocoder.config.output_sampling_rate,
    output_path="ltx2_lora_distilled_sample.mp4",
)
```

## Distilled checkpoint generation
Fastest two-stages generation pipeline using a distilled checkpoint.

```py
import torch
from diffusers.pipelines.ltx2 import LTX2Pipeline, LTX2LatentUpsamplePipeline
from diffusers.pipelines.ltx2.latent_upsampler import LTX2LatentUpsamplerModel
from diffusers.pipelines.ltx2.utils import DISTILLED_SIGMA_VALUES, STAGE_2_DISTILLED_SIGMA_VALUES
from diffusers.utils import encode_video

device = "cuda"
width = 768
height = 512
random_seed = 42
generator = torch.Generator(device).manual_seed(random_seed)
model_path = "rootonchair/LTX-2-19b-distilled"

pipe = LTX2Pipeline.from_pretrained(
    model_path, dtype=torch.bfloat16
)
pipe.enable_sequential_cpu_offload(device=device)

prompt = "A beautiful sunset over the ocean"
negative_prompt = "shaky, glitchy, low quality, worst quality, deformed, distorted, disfigured, motion smear, motion artifacts, fused fingers, bad anatomy, weird hand, ugly, transition, static."

frame_rate = 24.0
video_latent, audio_latent = pipe(
    prompt=prompt,
    negative_prompt=negative_prompt,
    width=width,
    height=height,
    num_frames=121,
    frame_rate=frame_rate,
    num_inference_steps=8,
    sigmas=DISTILLED_SIGMA_VALUES,
    guidance_scale=1.0,
    generator=generator,
    output_type="latent",
    return_dict=False,
)

latent_upsampler = LTX2LatentUpsamplerModel.from_pretrained(
    model_path,
    subfolder="latent_upsampler",
    dtype=torch.bfloat16,
)
upsample_pipe = LTX2LatentUpsamplePipeline(vae=pipe.vae, latent_upsampler=latent_upsampler)
upsample_pipe.enable_model_cpu_offload(device=device)
upscaled_video_latent = upsample_pipe(
    latents=video_latent,
    output_type="latent",
    return_dict=False,
)[0]

video, audio = pipe(
    latents=upscaled_video_latent,
    audio_latents=audio_latent,
    prompt=prompt,
    negative_prompt=negative_prompt,
    num_inference_steps=3,
    noise_scale=STAGE_2_DISTILLED_SIGMA_VALUES[0], # renoise with first sigma value https://github.com/Lightricks/LTX-2/blob/main/packages/ltx-pipelines/src/ltx_pipelines/distilled.py#L178
    sigmas=STAGE_2_DISTILLED_SIGMA_VALUES,
    generator=generator,
    guidance_scale=1.0,
    output_type="np",
    return_dict=False,
)

encode_video(
    video[0],
    fps=frame_rate,
    audio=audio[0].float().cpu(),
    audio_sample_rate=pipe.vocoder.config.output_sampling_rate,
    output_path="ltx2_distilled_sample.mp4",
)
```

## Condition Pipeline Generation

You can use `LTX2ConditionPipeline` to specify image and/or video conditions at arbitrary latent indices. For example, we can specify both a first-frame and last-frame condition to perform first-last-frame-to-video (FLF2V) generation:

```py
import torch
from diffusers import LTX2ConditionPipeline, LTX2LatentUpsamplePipeline
from diffusers.pipelines.ltx2.latent_upsampler import LTX2LatentUpsamplerModel
from diffusers.pipelines.ltx2.pipeline_ltx2_condition import LTX2VideoCondition
from diffusers.pipelines.ltx2.utils import DISTILLED_SIGMA_VALUES, STAGE_2_DISTILLED_SIGMA_VALUES
from diffusers.utils import encode_video
from diffusers.utils import load_image

device = "cuda"
width = 768
height = 512
random_seed = 42
generator = torch.Generator(device).manual_seed(random_seed)
model_path = "rootonchair/LTX-2-19b-distilled"

pipe = LTX2ConditionPipeline.from_pretrained(model_path, dtype=torch.bfloat16)
pipe.enable_sequential_cpu_offload(device=device)
pipe.vae.enable_tiling()

prompt = (
    "CG animation style, a small blue bird takes off from the ground, flapping its wings. The bird's feathers are "
    "delicate, with a unique pattern on its chest. The background shows a blue sky with white clouds under bright "
    "sunshine. The camera follows the bird upward, capturing its flight and the vastness of the sky from a close-up, "
    "low-angle perspective."
)

first_image = load_image(
    "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/flf2v_input_first_frame.png",
)
last_image = load_image(
    "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/flf2v_input_last_frame.png",
)
first_cond = LTX2VideoCondition(frames=first_image, index=0, strength=1.0)
last_cond = LTX2VideoCondition(frames=last_image, index=-1, strength=1.0)
conditions = [first_cond, last_cond]

frame_rate = 24.0
video_latent, audio_latent = pipe(
    conditions=conditions,
    prompt=prompt,
    width=width,
    height=height,
    num_frames=121,
    frame_rate=frame_rate,
    num_inference_steps=8,
    sigmas=DISTILLED_SIGMA_VALUES,
    guidance_scale=1.0,
    generator=generator,
    output_type="latent",
    return_dict=False,
)

latent_upsampler = LTX2LatentUpsamplerModel.from_pretrained(
    model_path,
    subfolder="latent_upsampler",
    dtype=torch.bfloat16,
)
upsample_pipe = LTX2LatentUpsamplePipeline(vae=pipe.vae, latent_upsampler=latent_upsampler)
upsample_pipe.enable_model_cpu_offload(device=device)
upscaled_video_latent = upsample_pipe(
    latents=video_latent,
    output_type="latent",
    return_dict=False,
)[0]

video, audio = pipe(
    latents=upscaled_video_latent,
    audio_latents=audio_latent,
    prompt=prompt,
    width=width * 2,
    height=height * 2,
    num_inference_steps=3,
    sigmas=STAGE_2_DISTILLED_SIGMA_VALUES,
    generator=generator,
    guidance_scale=1.0,
    output_type="np",
    return_dict=False,
)

encode_video(
    video[0],
    fps=frame_rate,
    audio=audio[0].float().cpu(),
    audio_sample_rate=pipe.vocoder.config.output_sampling_rate,
    output_path="ltx2_distilled_flf2v.mp4",
)
```

You can use both image and video conditions:

```py
import torch
from diffusers import LTX2ConditionPipeline
from diffusers.pipelines.ltx2.pipeline_ltx2_condition import LTX2VideoCondition
from diffusers.utils import encode_video
from diffusers.pipelines.ltx2.utils import DEFAULT_NEGATIVE_PROMPT
from diffusers.utils import load_image, load_video

device = "cuda"
width = 768
height = 512
random_seed = 42
generator = torch.Generator(device).manual_seed(random_seed)
model_path = "rootonchair/LTX-2-19b-distilled"

pipe = LTX2ConditionPipeline.from_pretrained(model_path, dtype=torch.bfloat16)
pipe.enable_sequential_cpu_offload(device=device)
pipe.vae.enable_tiling()

prompt = (
    "The video depicts a long, straight highway stretching into the distance, flanked by metal guardrails. The road is "
    "divided into multiple lanes, with a few vehicles visible in the far distance. The surrounding landscape features "
    "dry, grassy fields on one side and rolling hills on the other. The sky is mostly clear with a few scattered "
    "clouds, suggesting a bright, sunny day. And then the camera switch to a winding mountain road covered in snow, "
    "with a single vehicle traveling along it. The road is flanked by steep, rocky cliffs and sparse vegetation. The "
    "landscape is characterized by rugged terrain and a river visible in the distance. The scene captures the "
    "solitude and beauty of a winter drive through a mountainous region."
)

cond_video = load_video(
    "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/cosmos/cosmos-video2world-input-vid.mp4"
)
cond_image = load_image(
    "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/cosmos/cosmos-video2world-input.jpg"
)
video_cond = LTX2VideoCondition(frames=cond_video, index=0, strength=1.0)
image_cond = LTX2VideoCondition(frames=cond_image, index=8, strength=1.0)
conditions = [video_cond, image_cond]

frame_rate = 24.0
video, audio = pipe(
    conditions=conditions,
    prompt=prompt,
    negative_prompt=DEFAULT_NEGATIVE_PROMPT,
    width=width,
    height=height,
    num_frames=121,
    frame_rate=frame_rate,
    num_inference_steps=30,
    guidance_scale=3.0,
    generator=generator,
    output_type="np",
    return_dict=False,
)

encode_video(
    video[0],
    fps=frame_rate,
    audio=audio[0].float().cpu(),
    audio_sample_rate=pipe.vocoder.config.output_sampling_rate,
    output_path="ltx2_cond_video.mp4",
)
```

Because the conditioning is done via latent frames, the 8 data space frames corresponding to the specified latent frame for an image condition will tend to be static.

## Multimodal Guidance

LTX-2.X pipelines support multimodal guidance. It is composed of three terms, all using a CFG-style update rule:

1. Classifier-Free Guidance (CFG): standard [CFG](https://huggingface.co/papers/2207.12598) where the perturbed ("weaker") output is generated using the negative prompt.
2. Spatio-Temporal Guidance (STG): [STG](https://huggingface.co/papers/2411.18664) moves away from a perturbed output created from short-cutting self-attention operations and substitutes in the attention values instead. The idea is that this creates sharper videos and better spatiotemporal consistency.
3. Modality Isolation Guidance: moves away from a perturbed output created from disabling cross-modality (audio-to-video and video-to-audio) cross attention. This guidance is more specific to [LTX-2.X](https://huggingface.co/papers/2601.03233) models, with the idea that this produces better consistency between the generated audio and video.

These are controlled by the `guidance_scale`, `stg_scale`, and `modality_scale` arguments and can be set separately for video and audio. Additionally, for STG the transformer block indices where self-attention is skipped needs to be specified via the `spatio_temporal_guidance_blocks` argument. The LTX-2.X pipelines also support [guidance rescaling](https://huggingface.co/papers/2305.08891) to help reduce over-exposure, which can be a problem when the guidance scales are set to high values.

```py
import torch
from diffusers import LTX2ImageToVideoPipeline
from diffusers.utils import encode_video
from diffusers.pipelines.ltx2.utils import DEFAULT_NEGATIVE_PROMPT
from diffusers.utils import load_image

device = "cuda"
width = 768
height = 512
random_seed = 42
frame_rate = 24.0
generator = torch.Generator(device).manual_seed(random_seed)
model_path = "diffusers/LTX-2.3-Diffusers"

pipe = LTX2ImageToVideoPipeline.from_pretrained(model_path, dtype=torch.bfloat16)
pipe.enable_sequential_cpu_offload(device=device)
pipe.vae.enable_tiling()

prompt = (
    "An astronaut hatches from a fragile egg on the surface of the Moon, the shell cracking and peeling apart in "
    "gentle low-gravity motion. Fine lunar dust lifts and drifts outward with each movement, floating in slow arcs "
    "before settling back onto the ground. The astronaut pushes free in a deliberate, weightless motion, small "
    "fragments of the egg tumbling and spinning through the air. In the background, the deep darkness of space subtly "
    "shifts as stars glide with the camera's movement, emphasizing vast depth and scale. The camera performs a "
    "smooth, cinematic slow push-in, with natural parallax between the foreground dust, the astronaut, and the "
    "distant starfield. Ultra-realistic detail, physically accurate low-gravity motion, cinematic lighting, and a "
    "breath-taking, movie-like shot."
)

image = load_image(
    "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/astronaut.jpg",
)

video, audio = pipe(
    image=image,
    prompt=prompt,
    negative_prompt=DEFAULT_NEGATIVE_PROMPT,
    width=width,
    height=height,
    num_frames=121,
    frame_rate=frame_rate,
    num_inference_steps=30,
    guidance_scale=3.0,  # Recommended LTX-2.3 guidance parameters
    stg_scale=1.0,  # Note that 0.0 (not 1.0) means that STG is disabled (all other guidance is disabled at 1.0)
    modality_scale=3.0,
    guidance_rescale=0.7,
    audio_guidance_scale=7.0,  # Note that a higher CFG guidance scale is recommended for audio
    audio_stg_scale=1.0,
    audio_modality_scale=3.0,
    audio_guidance_rescale=0.7,
    spatio_temporal_guidance_blocks=[28],
    use_cross_timestep=True,
    generator=generator,
    output_type="np",
    return_dict=False,
)

encode_video(
    video[0],
    fps=frame_rate,
    audio=audio[0].float().cpu(),
    audio_sample_rate=pipe.vocoder.config.output_sampling_rate,
    output_path="ltx2_3_i2v_stage_1.mp4",
)
```

## Prompt Enhancement

The LTX-2.X models are sensitive to prompting style. Refer to the [official prompting guide](https://ltx.io/model/model-blog/prompting-guide-for-ltx-2) for recommendations on how to write a good prompt. Using prompt enhancement, where the supplied prompts are enhanced using the pipeline's text encoder (by default a [Gemma 3](https://huggingface.co/google/gemma-3-12b-it-qat-q4_0-unquantized) model) given a system prompt, can also improve sample quality. The optional `processor` pipeline component needs to be present to use prompt enhancement. Enable it with `enable_prompt_enhancement=True` and a `system_prompt` (opt-in, matching the Lightricks reference pipelines):

```py
import torch
from transformers import Gemma3Processor
from diffusers import LTX2Pipeline
from diffusers.utils import encode_video
from diffusers.pipelines.ltx2.utils import DEFAULT_NEGATIVE_PROMPT, T2V_DEFAULT_SYSTEM_PROMPT

device = "cuda"
width = 768
height = 512
random_seed = 42
frame_rate = 24.0
generator = torch.Generator(device).manual_seed(random_seed)
model_path = "diffusers/LTX-2.3-Diffusers"

pipe = LTX2Pipeline.from_pretrained(model_path, dtype=torch.bfloat16)
pipe.enable_model_cpu_offload(device=device)
pipe.vae.enable_tiling()
if getattr(pipe, "processor", None) is None:
    processor = Gemma3Processor.from_pretrained("google/gemma-3-12b-it-qat-q4_0-unquantized")
    pipe.processor = processor

prompt = (
    "An astronaut hatches from a fragile egg on the surface of the Moon, the shell cracking and peeling apart in "
    "gentle low-gravity motion. Fine lunar dust lifts and drifts outward with each movement, floating in slow arcs "
    "before settling back onto the ground. The astronaut pushes free in a deliberate, weightless motion, small "
    "fragments of the egg tumbling and spinning through the air. In the background, the deep darkness of space subtly "
    "shifts as stars glide with the camera's movement, emphasizing vast depth and scale. The camera performs a "
    "smooth, cinematic slow push-in, with natural parallax between the foreground dust, the astronaut, and the "
    "distant starfield. Ultra-realistic detail, physically accurate low-gravity motion, cinematic lighting, and a "
    "breath-taking, movie-like shot."
)

video, audio = pipe(
    prompt=prompt,
    negative_prompt=DEFAULT_NEGATIVE_PROMPT,
    width=width,
    height=height,
    num_frames=121,
    frame_rate=frame_rate,
    num_inference_steps=30,
    guidance_scale=3.0,
    stg_scale=1.0,
    modality_scale=3.0,
    guidance_rescale=0.7,
    audio_guidance_scale=7.0,
    audio_stg_scale=1.0,
    audio_modality_scale=3.0,
    audio_guidance_rescale=0.7,
    spatio_temporal_guidance_blocks=[28],
    use_cross_timestep=True,
    enable_prompt_enhancement=True,
    system_prompt=T2V_DEFAULT_SYSTEM_PROMPT,
    generator=generator,
    output_type="np",
    return_dict=False,
)

encode_video(
    video[0],
    fps=frame_rate,
    audio=audio[0].float().cpu(),
    audio_sample_rate=pipe.vocoder.config.output_sampling_rate,
    output_path="ltx2_3_t2v_stage_1.mp4",
)
```

## LTX-2.5

LTX-2.5 reuses the same `LTX2Pipeline`/`LTX2VideoTransformer3DModel`/`AutoencoderKLLTX2Video`/etc. classes as LTX-2.3 — there is no separate pipeline class for it. The user-visible difference is the text encoder: LTX-2.5 is paired with a Gemma 4 (`gemma4_unified`) checkpoint instead of Gemma 3. This is loaded automatically when you call `from_pretrained` on a converted LTX-2.5 checkpoint (via the `transformers` `Auto*` classes), so no extra setup is needed at inference time — just point `from_pretrained` at an LTX-2.5 repo instead of an LTX-2.3 one.

[`Lightricks/LTX-2.5-Diffusers`](https://huggingface.co/Lightricks/LTX-2.5-Diffusers) ships both transformers: the **distilled** DiT in `transformer/`, which is what `model_index.json` points at, and the full/SFT DiT in `transformer_full/`, which has to be loaded explicitly (see [Full / SFT transformer](#full--sft-transformer)). The repo's `scheduler/` is configured for the distilled checkpoint (`use_dynamic_shifting=False`, `shift_terminal=None`) so that its sigma schedule is used exactly as given. Everything [two-stage generation](#two-stage-generation-for-ltx-25) needs is shipped there too: a `latent_upsampler/` subfolder and the stage 2 distilled LoRA, `ltx-2.5-22b-distilled-lora-450-bf16.safetensors`, at the root of the repo.

Distilled inference is driven by an explicit sigma schedule rather than a step count, and runs unguided (`guidance_scale=1.0`, so `negative_prompt` is unused). Passing `num_inference_steps` instead would hand the model a generic linear schedule and quietly cost quality:

```py
import torch
from diffusers import LTX2Pipeline
from diffusers.utils import encode_video
from diffusers.pipelines.ltx2.utils import DISTILLED_SIGMA_VALUES

device = "cuda"
width = 768
height = 512
random_seed = 42
frame_rate = 24.0
generator = torch.Generator(device).manual_seed(random_seed)
model_path = "Lightricks/LTX-2.5-Diffusers"

pipe = LTX2Pipeline.from_pretrained(model_path, dtype=torch.bfloat16)
pipe.enable_sequential_cpu_offload(device=device)
pipe.vae.enable_tiling()

prompt = "A cinematic shot of a red fox walking through a snowy forest at dawn, golden light filtering through pine trees."

video, audio = pipe(
    prompt=prompt,
    width=width,
    height=height,
    num_frames=121,
    frame_rate=frame_rate,
    sigmas=DISTILLED_SIGMA_VALUES,
    guidance_scale=1.0,
    audio_guidance_scale=1.0,
    generator=generator,
    output_type="np",
    return_dict=False,
)

encode_video(
    video[0],
    fps=frame_rate,
    audio=audio[0].float().cpu(),
    audio_sample_rate=pipe.vocoder.config.output_sampling_rate,
    output_path="ltx2_5_t2v.mp4",
)
```

### Two-stage generation for LTX-2.5

LTX-2.5 supports both two-stage variants, and `DISTILLED_SIGMA_VALUES` / `STAGE_2_DISTILLED_SIGMA_VALUES` are its reference schedules:

- **Distilled checkpoint, both stages** — the reference recipe for the default `transformer/`, and the one shown below. No stage 2 LoRA is involved, since the transformer is already distilled; this is [Distilled checkpoint generation](#distilled-checkpoint-generation) with LTX-2.5 weights.
- **Full/SFT stage 1 + distilled LoRA stage 2** — [Two-stages Generation](#two-stages-generation) as described at the top of this page, using `transformer_full/` and the shipped LoRA. See [below](#stage-2-with-the-distilled-lora) for what changes.

Stage 1 runs at half the target resolution, the upsampler doubles it, and stage 2 refines at full resolution — video *and* audio, both reseeded from the stage 1 latents at `noise_scale=STAGE_2_DISTILLED_SIGMA_VALUES[0]`. Height and width must be divisible by 64, since stage 1 halves each axis and still has to land on the VAE's spatial grid.

```py
import torch
from diffusers.pipelines.ltx2 import LTX2Pipeline, LTX2LatentUpsamplePipeline
from diffusers.pipelines.ltx2.latent_upsampler import LTX2LatentUpsamplerModel
from diffusers.pipelines.ltx2.utils import DISTILLED_SIGMA_VALUES, STAGE_2_DISTILLED_SIGMA_VALUES
from diffusers.utils import encode_video

device = "cuda"
width = 1536
height = 1024
num_frames = 121
frame_rate = 24.0
model_path = "Lightricks/LTX-2.5-Diffusers"

# One generator for the whole call, threaded through both stages, so stage 2 continues the noise
# stream instead of repeating stage 1's draw.
generator = torch.Generator(device).manual_seed(42)

pipe = LTX2Pipeline.from_pretrained(model_path, dtype=torch.bfloat16)
pipe.enable_sequential_cpu_offload(device=device)

prompt = "A cinematic shot of a red fox walking through a snowy forest at dawn, golden light filtering through pine trees."

# Stage 1: half resolution, 8 distilled sigmas
video_latent, audio_latent = pipe(
    prompt=prompt,
    width=width // 2,
    height=height // 2,
    num_frames=num_frames,
    frame_rate=frame_rate,
    sigmas=DISTILLED_SIGMA_VALUES,
    guidance_scale=1.0,
    audio_guidance_scale=1.0,
    generator=generator,
    output_type="latent",
    return_dict=False,
)

latent_upsampler = LTX2LatentUpsamplerModel.from_pretrained(
    model_path,
    subfolder="latent_upsampler",
    dtype=torch.bfloat16,
)
upsample_pipe = LTX2LatentUpsamplePipeline(vae=pipe.vae, latent_upsampler=latent_upsampler)
upsample_pipe.enable_model_cpu_offload(device=device)
# `latents_normalized=False`: `output_type="latent"` already applied the latent statistics, and the
# upsampler is trained on denormalized latents. Stage 2 renormalizes them in `prepare_latents`.
upscaled_video_latent = upsample_pipe(
    latents=video_latent,
    latents_normalized=False,
    output_type="latent",
    return_dict=False,
)[0]

# Stage 2: full resolution, 3 sigmas, reseeded from stage 1. Pass `num_frames` explicitly here --
# omitting it would run the duration head a second time instead of using the stage 1 length.
pipe.vae.enable_tiling()
video, audio = pipe(
    prompt=prompt,
    latents=upscaled_video_latent,
    audio_latents=audio_latent,
    width=width,
    height=height,
    num_frames=num_frames,
    frame_rate=frame_rate,
    sigmas=STAGE_2_DISTILLED_SIGMA_VALUES,
    noise_scale=STAGE_2_DISTILLED_SIGMA_VALUES[0],  # renoise with the stage 2 entry sigma
    guidance_scale=1.0,
    audio_guidance_scale=1.0,
    generator=generator,
    output_type="np",
    return_dict=False,
)

encode_video(
    video[0],
    fps=frame_rate,
    audio=audio[0].float().cpu(),
    audio_sample_rate=pipe.vocoder.config.output_sampling_rate,
    output_path="ltx2_5_t2v_two_stages.mp4",
)
```

When the length comes from the [duration head](#automatic-duration-for-ltx-25) rather than an explicit `num_frames`, let stage 1 decide and recover the realized length from its latents (`[B, C, F, H, W]`) before stage 2 runs, instead of predicting a second time:

```py
num_frames = (video_latent.shape[2] - 1) * pipe.vae_temporal_compression_ratio + 1
```

#### Stage 2 with the distilled LoRA

To run [Two-stages Generation](#two-stages-generation) instead — full/SFT DiT for stage 1, distilled LoRA for stage 2 — build the pipeline as in [Full / SFT transformer](#full--sft-transformer) and generate stage 1 latents with that guidance stack. Two things then differ from LTX-2.0/2.3. The LoRA lives in the diffusers repo itself rather than alongside the original weights, and the scheduler flip goes the other way round: LTX-2.5 ships the *distilled* scheduler config, so stage 1 is what turned dynamic shifting on, and stage 2 turns it back off.

```py
pipe.load_lora_weights(
    "Lightricks/LTX-2.5-Diffusers",
    adapter_name="stage_2_distilled",
    weight_name="ltx-2.5-22b-distilled-lora-450-bf16.safetensors",
)
pipe.set_adapters("stage_2_distilled", 1.0)
pipe.vae.enable_tiling()

pipe.scheduler = FlowMatchEulerDiscreteScheduler.from_config(
    pipe.scheduler.config, use_dynamic_shifting=False, shift_terminal=None
)
```

The upsample step and the stage 2 call itself are unchanged from the distilled recipe above: same `sigmas=STAGE_2_DISTILLED_SIGMA_VALUES`, same `noise_scale`, and `guidance_scale=1.0`, since stage 2 is running a distilled model either way.

### Convolutional and diffusion decoding

LTX-2.5 ships two video decoders over the same latent space, so latents are interchangeable between them:

- `vae/` — the convolutional VAE ([AutoencoderKLLTX2Video](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl_ltx_2#diffusers.AutoencoderKLLTX2Video)). It is what the pipelines decode with, so every snippet above already uses it, and it is the only one of the two that tiles (`pipe.vae.enable_tiling()`), which is usually what makes a high resolution fit.
- `diffusion_decoder/` — [LTX2VideoDiffusionDecoderModel](/docs/diffusers/v0.40.0/en/api/models/ltx2_diffusion_decoder#diffusers.LTX2VideoDiffusionDecoderModel). It is a diffusion model in its own right rather than a pipeline component, so it is not passed as a `vae`: run the pipeline with `output_type="latent"` and hand the latents to [LTX2VideoDiffusionDecodePipeline](/docs/diffusers/v0.40.0/en/api/pipelines/ltx2#diffusers.LTX2VideoDiffusionDecodePipeline).

Encoding always goes through `vae/`, so image and video conditioning are unaffected by the choice.

Two things change when you decode with the diffusion decoder. `output_type="latent"` also skips the vocoder, so the audio comes back as latents and has to be finished by hand, and the NATTEN processor is effectively required at video resolutions:

```py
import torch
from diffusers import LTX2Pipeline, LTX2VideoDiffusionDecodePipeline, LTX2VideoDiffusionDecoderModel
from diffusers.models.autoencoders.ltx2_diffusion_decoder import LTX2VideoVaeNeighborhoodNattenProcessor
from diffusers.pipelines.ltx2.utils import DISTILLED_SIGMA_VALUES
from diffusers.utils import encode_video

device = "cuda"
frame_rate = 24.0
generator = torch.Generator(device).manual_seed(42)
model_path = "Lightricks/LTX-2.5-Diffusers"

pipe = LTX2Pipeline.from_pretrained(model_path, dtype=torch.bfloat16)
pipe.enable_model_cpu_offload(device=device)

prompt = "A cinematic shot of a red fox walking through a snowy forest at dawn, golden light filtering through pine trees."

latents, audio_latents = pipe(
    prompt=prompt,
    width=960,
    height=544,
    num_frames=121,
    frame_rate=frame_rate,
    sigmas=DISTILLED_SIGMA_VALUES,
    guidance_scale=1.0,
    audio_guidance_scale=1.0,
    generator=generator,
    output_type="latent",
    return_dict=False,
)

# `output_type="latent"` skips the vocoder, so finish the audio by hand. These latents are already
# denormalized, which is what `audio_vae.decode` expects.
mel = pipe.audio_vae.decode(audio_latents.to(pipe.audio_vae.dtype), return_dict=False)[0]
audio = pipe.vocoder(mel)

decoder = LTX2VideoDiffusionDecoderModel.from_pretrained(
    model_path, subfolder="diffusion_decoder", dtype=torch.bfloat16
).to(device)
# The decoder runs on the `flex` backend by default, and uncompiled `flex_attention` materializes the
# full score matrix -- tens of GB at video resolutions. NATTEN's kernels are what the original
# implementation uses; they are fetched from the Hub by `kernels` (`pip install kernels`), not from a
# local NATTEN build. Switching the attention *backend* instead raises: only `flex` takes the BlockMask.
decoder.set_attn_processor(LTX2VideoVaeNeighborhoodNattenProcessor())
# Decode in overlapping tiles so peak memory scales with the tile size rather than the video size.
decoder.enable_tiling()

decode_pipe = LTX2VideoDiffusionDecodePipeline(diffusion_decoder=decoder, scheduler=pipe.scheduler)

# `denormalize=False`: `output_type="latent"` already applied the latent statistics, so applying them
# again would rescale every channel by its std a second time. The decoder draws the noise it denoises,
# so pass a generator to make decoding reproducible.
video = decode_pipe(
    latents, generator=generator, output_type="np", denormalize=False, return_dict=False
)[0]

encode_video(
    video[0],
    fps=frame_rate,
    audio=audio[0].float().cpu(),
    audio_sample_rate=pipe.vocoder.config.output_sampling_rate,
    output_path="ltx2_5_t2v_diffusion_decode.mp4",
)
```

To combine this with [two-stage generation](#two-stage-generation-for-ltx-25), ask *stage 2* for `output_type="latent"` and decode that.

`decoder.enable_tiling()` is what keeps a high resolution decode in memory, the same way `pipe.vae.enable_tiling()` does for the convolutional VAE. The memory-dominant part of the decode — the last upsampling stage and the diffusion stage — then runs on overlapping tiles that are blended back together, so peak memory is bounded by the tile size instead of the video size. Tiling only kicks in once the latent exceeds one tile, and the tile and overlap sizes can be tuned via the `tile_sample_min_*` / `tile_sample_stride_*` arguments (defaults match the reference implementation). Since the diffusion stage denoises each tile separately, a tiled decode does not reproduce the untiled result exactly.

On a single card it is also worth moving the pipeline out of the way before decoding (`pipe.to("cpu")` and `torch.cuda.empty_cache()`, after capturing `pipe.scheduler` and the vocoder's `output_sampling_rate`), since the decoder needs its own headroom. See [LTX2VideoDiffusionDecoderModel](/docs/diffusers/v0.40.0/en/api/models/ltx2_diffusion_decoder#diffusers.LTX2VideoDiffusionDecoderModel) for the attention backends, the tiling details, and the rest of the decoder's behaviour.

### Full / SFT transformer

`transformer_full/` is not referenced by `model_index.json`, so load it explicitly. It also needs a different scheduler and a real guidance stack: the shipped `scheduler/` is configured for the distilled checkpoint, and the guidance defaults are LTX-2.0-era generics that leave an LTX-2.5 SFT run visibly under-guided without raising anything. The [Multimodal Guidance](#multimodal-guidance) recommendations apply here unchanged, including STG on block `28`:

```py
import torch
from diffusers import FlowMatchEulerDiscreteScheduler, LTX2Pipeline, LTX2VideoTransformer3DModel
from diffusers.pipelines.ltx2.utils import DEFAULT_NEGATIVE_PROMPT

device = "cuda"
model_path = "Lightricks/LTX-2.5-Diffusers"

# Passing `transformer=` keeps `from_pretrained` from fetching the distilled folder as well.
transformer = LTX2VideoTransformer3DModel.from_pretrained(
    model_path, subfolder="transformer_full", dtype=torch.bfloat16
)
pipe = LTX2Pipeline.from_pretrained(model_path, transformer=transformer, dtype=torch.bfloat16)
pipe.enable_sequential_cpu_offload(device=device)
pipe.vae.enable_tiling()

# Re-enable dynamic shifting and the terminal shift, which the distilled configuration turns off.
pipe.scheduler = FlowMatchEulerDiscreteScheduler.from_config(
    pipe.scheduler.config, use_dynamic_shifting=True, shift_terminal=0.1
)

video, audio = pipe(
    prompt="A cinematic shot of a red fox walking through a snowy forest at dawn, golden light filtering through pine trees.",
    negative_prompt=DEFAULT_NEGATIVE_PROMPT,
    width=768,
    height=512,
    num_frames=121,
    frame_rate=24.0,
    num_inference_steps=30,
    guidance_scale=3.0,
    stg_scale=1.0,
    modality_scale=3.0,
    guidance_rescale=0.7,
    audio_guidance_scale=7.0,
    audio_stg_scale=1.0,
    audio_modality_scale=3.0,
    audio_guidance_rescale=0.7,
    spatio_temporal_guidance_blocks=[28],
    use_cross_timestep=True,
    generator=torch.Generator(device).manual_seed(42),
    output_type="np",
    return_dict=False,
)
```

Drop `sigmas` here — the full DiT takes its schedule from the scheduler.

### Prompt Enhancement for LTX-2.5

**Using prompt enhancement is strongly recommended for LTX-2.5; pass `enable_prompt_enhancement=True` to opt in** (same as the Lightricks reference pipelines). Unlike LTX-2.0/2.3, where the same text encoder checkpoint doubles as the enhancer (see [Prompt Enhancement](#prompt-enhancement) above), LTX-2.5's fine-tuned text encoder was not trained for enhancement. Instead, enhancement uses a separate, off-the-shelf `google/gemma-4-E2B-it` checkpoint. Load it into the pipeline's optional `prompt_enhancer`/`processor` components, then enable enhancement — the pipeline defaults to `LTX2_5_T2V_DEFAULT_SYSTEM_PROMPT` and the Gemma 4 recipe (`do_sample=False`, `no_repeat_ngram_size=5`, `max_new_tokens=600`). Pass an explicit `system_prompt=` to override:

```py
import torch
from transformers import AutoModelForImageTextToText, AutoProcessor
from diffusers import LTX2Pipeline
from diffusers.utils import encode_video
from diffusers.pipelines.ltx2.utils import DISTILLED_SIGMA_VALUES

device = "cuda"
width = 768
height = 512
random_seed = 42
frame_rate = 24.0
generator = torch.Generator(device).manual_seed(random_seed)
model_path = "Lightricks/LTX-2.5-Diffusers"
enhancer_model_id = "google/gemma-4-E2B-it"

pipe = LTX2Pipeline.from_pretrained(model_path, dtype=torch.bfloat16)
pipe.enable_model_cpu_offload(device=device)
pipe.vae.enable_tiling()
if getattr(pipe, "prompt_enhancer", None) is None:
    pipe.prompt_enhancer = AutoModelForImageTextToText.from_pretrained(enhancer_model_id)
    pipe.processor = AutoProcessor.from_pretrained(enhancer_model_id)

prompt = "A cinematic shot of a red fox walking through a snowy forest at dawn, golden light filtering through pine trees."

video, audio = pipe(
    prompt=prompt,
    width=width,
    height=height,
    num_frames=121,
    frame_rate=frame_rate,
    sigmas=DISTILLED_SIGMA_VALUES,
    guidance_scale=1.0,
    audio_guidance_scale=1.0,
    enable_prompt_enhancement=True,
    # No `system_prompt=` needed -- defaults to `LTX2_5_T2V_DEFAULT_SYSTEM_PROMPT` when `prompt_enhancer` is set.
    generator=generator,
    output_type="np",
    return_dict=False,
)

encode_video(
    video[0],
    fps=frame_rate,
    audio=audio[0].float().cpu(),
    audio_sample_rate=pipe.vocoder.config.output_sampling_rate,
    output_path="ltx2_5_t2v_enhanced.mp4",
)
```

The same applies to image-to-video with `LTX2ImageToVideoPipeline`: set `pipe.prompt_enhancer`/`pipe.processor` the same way and pass `enable_prompt_enhancement=True` (using `LTX2_5_I2V_DEFAULT_SYSTEM_PROMPT`, conditioning on both the reference image and the text prompt) — again, no `system_prompt=` needed unless you want to override it.

### Automatic duration for LTX-2.5

LTX-2.5 checkpoints ship a small `duration_head` that predicts how long the described shot should be, from the same text-connector output the transformer is conditioned on. When the loaded pipeline has one, **`num_frames` is auto-predicted by default** — omit it and the model chooses the length:

```py
video, audio = pipe(prompt=prompt, output_type="np", return_dict=False)
```

To set the length yourself, pass `num_frames` explicitly. An integer always wins over the head:

```py
video, audio = pipe(prompt=prompt, num_frames=121, output_type="np", return_dict=False)
```

Pipelines loaded from LTX-2.0 or LTX-2.3 checkpoints have no duration head and keep the previous default of 121 frames, so this changes nothing for them.

Pass `min_seconds` / `max_seconds` to constrain the prediction. The raw prediction is clamped into the range, then converted to frames:

```py
video, audio = pipe(
    prompt=prompt,
    min_seconds=2.0,
    max_seconds=10.0,
    frame_rate=frame_rate,
    output_type="np",
    return_dict=False,
)
```

Predicted frame counts are snapped to the VAE's causal temporal grid (`8k + 1`), so the realized duration is quantized — about 0.33s per step at 24 fps — and it shifts with `frame_rate`, since the head predicts seconds rather than frames. `min_seconds` must be strictly less than `max_seconds`. These bounds are ignored when `num_frames` is set explicitly.

Bounds narrower than one grid step may not be satisfiable exactly: at 24 fps `[1.0s, 1.02s]` converts to `[24, 24]` frames, and 24 is not `8k + 1`. The nearest grid point is used and a warning is logged, so the returned length can fall just outside bounds that tight.

To inspect a prediction without generating a video, call the head directly. Everything it needs is public:

```py
prompt_embeds, prompt_attention_mask, _, _ = pipe.encode_prompt(prompt, do_classifier_free_guidance=False)
video_tokens, audio_tokens, _ = pipe.connectors(prompt_embeds, prompt_attention_mask)

num_frames = pipe.duration_head.predict_num_frames(
    video_tokens,
    audio_tokens,
    frame_rate=24.0,
    temporal_compression_ratio=pipe.vae_temporal_compression_ratio,
)
seconds = pipe.duration_head(video_tokens, audio_tokens).item()  # raw, before clamping
print(f"predicted {seconds:.2f}s -> {num_frames} frames")
```

Converting a 2.5 checkpoint picks the head up automatically with `--full_pipeline`, or on its own with `--duration_head`. Checkpoints predating 2.5 have no such weights, and conversion skips the component rather than failing.

## LTX2Pipeline[[diffusers.LTX2Pipeline]]

#### diffusers.LTX2Pipeline[[diffusers.LTX2Pipeline]]

```python
diffusers.LTX2Pipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKLLTX2Video, audio_vae: AutoencoderKLLTX2Audio, text_encoder: transformers.models.gemma3.modeling_gemma3.Gemma3ForConditionalGeneration | transformers.models.gemma4_unified.modeling_gemma4_unified.Gemma4UnifiedForConditionalGeneration, tokenizer: GemmaTokenizer, connectors: LTX2TextConnectors, transformer: LTX2VideoTransformer3DModel, vocoder: diffusers.pipelines.ltx2.vocoder.LTX2Vocoder | diffusers.pipelines.ltx2.vocoder.LTX2VocoderWithBWE, processor: transformers.processing_utils.ProcessorMixin | None = None, prompt_enhancer: transformers.models.gemma4.modeling_gemma4.Gemma4ForConditionalGeneration | None = None, duration_head: diffusers.pipelines.ltx2.duration_head.LTX2DurationHead | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_ltx2.py#L206)

**Parameters:**

transformer ([LTXVideoTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/ltx_video_transformer3d#diffusers.LTXVideoTransformer3DModel)) : Conditional Transformer architecture to denoise the encoded video latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLLTXVideo](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl_ltx_video#diffusers.AutoencoderKLLTXVideo)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer (`T5TokenizerFast`) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

connectors (`LTX2TextConnectors`) : Text connector stack used to adapt text encoder hidden states for the video and audio branches.

Pipeline for text-to-video generation.

Reference: https://github.com/Lightricks/LTX-Video

#### __call__[[diffusers.LTX2Pipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, negative_prompt: str | list[str] | None = None, height: int = 512, width: int = 768, num_frames: int | None = None, min_seconds: float = 1.0, max_seconds: float = 20.0, frame_rate: float = 24.0, num_inference_steps: int = 30, sigmas: list[float] | None = None, timesteps: list = None, guidance_scale: float = 3.0, stg_scale: float = 1.0, modality_scale: float = 3.0, guidance_rescale: float = 0.7, audio_guidance_scale: float | None = 7.0, audio_stg_scale: float | None = 1.0, audio_modality_scale: float | None = 3.0, audio_guidance_rescale: float | None = 0.7, spatio_temporal_guidance_blocks: list[int] | None = [28], noise_scale: float = 0.0, num_videos_per_prompt: int = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, audio_latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, decode_timestep: float | list[float] = 0.0, decode_noise_scale: float | list[float] | None = None, use_cross_timestep: bool = True, system_prompt: str | None = None, enable_prompt_enhancement: bool = False, prompt_max_new_tokens: int | None = None, prompt_enhancement_kwargs: dict[str, typing.Any] | None = None, prompt_enhancement_seed: int = 10, output_type: str = 'pil', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 1024)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_ltx2.py#L926)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale < 1`).

height (`int`, *optional*, defaults to `512`) : The height in pixels of the generated image. This is set to 480 by default for the best results.

width (`int`, *optional*, defaults to `768`) : The width in pixels of the generated image. This is set to 848 by default for the best results.

num_frames (`int`, *optional*) : The number of video frames to generate. If not supplied, defaults to an auto-predicted duration when this pipeline has a `duration_head` component (LTX-2.5 checkpoints and later), and to `121` otherwise. Pass an integer to set the length explicitly. Auto-predicted counts are snapped to the VAE's causal temporal grid, so the realized duration is quantized (roughly 0.33s at 24 fps).

min_seconds (`float`, *optional*, defaults to `1.0`) : Lower bound on the auto-predicted duration when `num_frames` is omitted and a `duration_head` is present. Ignored when `num_frames` is set explicitly.

max_seconds (`float`, *optional*, defaults to `20.0`) : Upper bound on the auto-predicted duration when `num_frames` is omitted and a `duration_head` is present. Ignored when `num_frames` is set explicitly. Must be strictly greater than `min_seconds`.

frame_rate (`float`, *optional*, defaults to `24.0`) : The frames per second (FPS) of the generated video.

num_inference_steps (`int`, *optional*, defaults to 30) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`List[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

timesteps (`list[int]`, *optional*) : Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used. Must be in descending order.

guidance_scale (`float`, *optional*, defaults to `4.0`) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality. Used for the video modality (there is a separate value `audio_guidance_scale` for the audio modality).

stg_scale (`float`, *optional*, defaults to `0.0`) : Video guidance scale for Spatio-Temporal Guidance (STG), proposed in [Spatiotemporal Skip Guidance for Enhanced Video Diffusion Sampling](https://arxiv.org/abs/2411.18664). STG uses a CFG-like estimate where we move the sample away from a weak sample from a perturbed version of the denoising model. Enabling STG will result in an additional denoising model forward pass; the default value of `0.0` means that STG is disabled.

modality_scale (`float`, *optional*, defaults to `1.0`) : Video guidance scale for LTX-2.X modality isolation guidance, where we move the sample away from a weaker sample generated by the denoising model withy cross-modality (audio-to-video and video-to-audio) cross attention disabled using a CFG-like estimate. Enabling modality guidance will result in an additional denoising model forward pass; the default value of `1.0` means that modality guidance is disabled.

guidance_rescale (`float`, *optional*, defaults to 0.0) : Guidance rescale factor proposed by [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891) `guidance_scale` is defined as `φ` in equation 16. of [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891). Guidance rescale factor should fix overexposure when using zero terminal SNR. Used for the video modality.

audio_guidance_scale (`float`, *optional* defaults to `None`) : Audio guidance scale for CFG with respect to the negative prompt. The CFG update rule is the same for video and audio, but they can use different values for the guidance scale. The LTX-2.X authors suggest that the `audio_guidance_scale` should be higher relative to the video `guidance_scale` (e.g. for LTX-2.3 they suggest 3.0 for video and 7.0 for audio). If `None`, defaults to the video value `guidance_scale`.

audio_stg_scale (`float`, *optional*, defaults to `None`) : Audio guidance scale for STG. As with CFG, the STG update rule is otherwise the same for video and audio. For LTX-2.3, a value of 1.0 is suggested for both video and audio. If `None`, defaults to the video value `stg_scale`.

audio_modality_scale (`float`, *optional*, defaults to `None`) : Audio guidance scale for LTX-2.X modality isolation guidance. As with CFG, the modality guidance rule is otherwise the same for video and audio. For LTX-2.3, a value of 3.0 is suggested for both video and audio. If `None`, defaults to the video value `modality_scale`.

audio_guidance_rescale (`float`, *optional*, defaults to `None`) : A separate guidance rescale factor for the audio modality. If `None`, defaults to the video value `guidance_rescale`.

spatio_temporal_guidance_blocks (`list[int]`, *optional*, defaults to `None`) : The zero-indexed transformer block indices at which to apply STG. Must be supplied if STG is used (`stg_scale` or `audio_stg_scale` is greater than `0`). A value of `[29]` is recommended for LTX-2.0 and `[28]` is recommended for LTX-2.3.

noise_scale (`float`, *optional*, defaults to `0.0`) : The interpolation factor between random noise and denoised latents at each timestep. Applying noise to the `latents` and `audio_latents` before continue denoising.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : The number of videos to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for video generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

audio_latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for audio generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

prompt_attention_mask (`torch.Tensor`, *optional*) : Pre-generated attention mask for text embeddings.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. For PixArt-Sigma this negative prompt should be "". If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

negative_prompt_attention_mask (`torch.FloatTensor`, *optional*) : Pre-generated attention mask for negative text embeddings.

decode_timestep (`float`, defaults to `0.0`) : The timestep at which generated video is decoded.

decode_noise_scale (`float`, defaults to `None`) : The interpolation factor between random noise and denoised latents at the decode timestep.

use_cross_timestep (`bool` *optional*, defaults to `True`) : Whether to use the cross modality (audio is the cross modality of video, and vice versa) sigma when calculating the cross attention modulation parameters. `True` is the LTX-2.3/2.5 behavior; `False` is the legacy LTX-2.0 behavior.

system_prompt (`str`, *optional*, defaults to `None`) : Optional system prompt to use for prompt enhancement. The system prompt will be used by the prompt enhancer (a Gemma conditional-generation model -- the dedicated `prompt_enhancer` component if one is configured, otherwise the main `text_encoder`) to generate an enhanced prompt from the original `prompt` to condition generation. If not supplied and a dedicated `prompt_enhancer` is configured (LTX-2.5), defaults to `LTX2_5_T2V_DEFAULT_SYSTEM_PROMPT` (from `diffusers.pipelines.ltx2.utils`) -- see `enable_prompt_enhancement`.

enable_prompt_enhancement (`bool`, *optional*, defaults to `False`) : Whether to run prompt enhancement. Opt-in, matching the Lightricks reference pipelines. When `True` and `system_prompt` is omitted, LTX-2.5 uses `LTX2_5_T2V_DEFAULT_SYSTEM_PROMPT` if a dedicated `prompt_enhancer` is configured; LTX-2.0/2.3 require an explicit `system_prompt`.

prompt_max_new_tokens (`int`, *optional*, defaults to `None`) : The maximum number of new tokens to generate when performing prompt enhancement. If not supplied, uses 600 for a dedicated Gemma 4 `prompt_enhancer` (LTX-2.5) or 512 for the Gemma 3 `text_encoder` fallback (LTX-2.0/2.3).

prompt_enhancement_kwargs (`dict[str, Any]`, *optional*, defaults to `None`) : Keyword arguments for the prompt enhancer's `.generate` call. If not supplied, always matches whichever model is doing the enhancing: `do_sample=False, no_repeat_ngram_size=5` (greedy) when using a dedicated `prompt_enhancer` (LTX-2.5), or `do_sample=True, temperature=0.7` for the `text_encoder` fallback (LTX-2.0/2.3). See https://huggingface.co/docs/transformers/main/en/main_classes/text_generation#transformers.GenerationMixin.generate for more details.

prompt_enhancement_seed (`int`, *optional*, defaults to `10`) : Random seed for any random operations during prompt enhancement.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.ltx.LTX2PipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`List`, *optional*, defaults to `["latents"]`) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int`, *optional*, defaults to `1024`) : Maximum sequence length to use with the `prompt`.

**Returns:** `~pipelines.ltx.LTX2PipelineOutput` or `tuple`

If `return_dict` is `True`, `~pipelines.ltx.LTX2PipelineOutput` is returned, otherwise a `tuple` is
returned where the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import LTX2Pipeline
>>> from diffusers.utils import encode_video

>>> pipe = LTX2Pipeline.from_pretrained("Lightricks/LTX-2", torch_dtype=torch.bfloat16)
>>> pipe.enable_model_cpu_offload()

>>> prompt = "A woman with long brown hair and light skin smiles at another woman with long blonde hair. The woman with brown hair wears a black jacket and has a small, barely noticeable mole on her right cheek. The camera angle is a close-up, focused on the woman with brown hair's face. The lighting is warm and natural, likely from the setting sun, casting a soft glow on the scene. The scene appears to be real-life footage"
>>> negative_prompt = "worst quality, inconsistent motion, blurry, jittery, distorted"

>>> frame_rate = 24.0
>>> video, audio = pipe(
...     prompt=prompt,
...     negative_prompt=negative_prompt,
...     width=768,
...     height=512,
...     num_frames=121,
...     frame_rate=frame_rate,
...     num_inference_steps=30,
...     guidance_scale=3.0,
...     output_type="np",
...     return_dict=False,
... )

>>> encode_video(
...     video[0],
...     fps=frame_rate,
...     audio=audio[0].float().cpu(),
...     audio_sample_rate=pipe.vocoder.config.output_sampling_rate,  # should be 24000
...     output_path="video.mp4",
... )
```

#### encode_prompt[[diffusers.LTX2Pipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] | None = None, do_classifier_free_guidance: bool = True, num_videos_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 1024, scale_factor: int = 8, device: typing.Optional[torch.device] = None, dtype: typing.Optional[torch.dtype] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_ltx2.py#L364)

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

#### enhance_prompt[[diffusers.LTX2Pipeline.enhance_prompt]]

```python
enhance_prompt(prompt: str, system_prompt: str, max_new_tokens: int | None = None, seed: int = 10, generator: typing.Optional[torch.Generator] = None, generation_kwargs: dict[str, typing.Any] | None = None, device: typing.Union[torch.device, str, NoneType] = None, image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_ltx2.py#L571)

Enhances the supplied `prompt` by generating a new prompt using the prompt enhancer (a Gemma
conditional-generation model) from it and a system prompt. When `image` is supplied, the enhancer is also
conditioned on that reference frame (I2V / keyframe-style enhancement). Uses the dedicated `prompt_enhancer`
component if one is configured (e.g. LTX-2.5, whose text encoder isn't trained for enhancement), otherwise
falls back to the main `text_encoder` (LTX-2.0/2.3, which double as their own enhancer).

Message templates, decoding kwargs, response cleaning, and image long-side prep match `ltx-core` /
`ltx-pipelines` (`enhance_t2v` / `enhance_i2v` / `generate_enhanced_prompt`).

## LTX2ImageToVideoPipeline[[diffusers.LTX2ImageToVideoPipeline]]

#### diffusers.LTX2ImageToVideoPipeline[[diffusers.LTX2ImageToVideoPipeline]]

```python
diffusers.LTX2ImageToVideoPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKLLTX2Video, audio_vae: AutoencoderKLLTX2Audio, text_encoder: transformers.models.gemma3.modeling_gemma3.Gemma3ForConditionalGeneration | transformers.models.gemma4_unified.modeling_gemma4_unified.Gemma4UnifiedForConditionalGeneration, tokenizer: GemmaTokenizer, connectors: LTX2TextConnectors, transformer: LTX2VideoTransformer3DModel, vocoder: diffusers.pipelines.ltx2.vocoder.LTX2Vocoder | diffusers.pipelines.ltx2.vocoder.LTX2VocoderWithBWE, processor: transformers.processing_utils.ProcessorMixin | None = None, prompt_enhancer: transformers.models.gemma4.modeling_gemma4.Gemma4ForConditionalGeneration | None = None, duration_head: diffusers.pipelines.ltx2.duration_head.LTX2DurationHead | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_ltx2_image2video.py#L226)

Pipeline for image-to-video generation.

Reference: https://github.com/Lightricks/LTX-Video

TODO

#### __call__[[diffusers.LTX2ImageToVideoPipeline.__call__]]

```python
__call__(image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, prompt: str | list[str] = None, negative_prompt: str | list[str] | None = None, height: int = 512, width: int = 768, num_frames: int | None = None, min_seconds: float = 1.0, max_seconds: float = 20.0, frame_rate: float = 24.0, num_inference_steps: int = 30, sigmas: list[float] | None = None, timesteps: list[int] | None = None, guidance_scale: float = 3.0, stg_scale: float = 1.0, modality_scale: float = 3.0, guidance_rescale: float = 0.7, audio_guidance_scale: float | None = 7.0, audio_stg_scale: float | None = 1.0, audio_modality_scale: float | None = 3.0, audio_guidance_rescale: float | None = 0.7, spatio_temporal_guidance_blocks: list[int] | None = [28], noise_scale: float = 0.0, num_videos_per_prompt: int = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, audio_latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, decode_timestep: float | list[float] = 0.0, decode_noise_scale: float | list[float] | None = None, use_cross_timestep: bool = True, system_prompt: str | None = None, enable_prompt_enhancement: bool = False, prompt_max_new_tokens: int | None = None, prompt_enhancement_kwargs: dict[str, typing.Any] | None = None, prompt_enhancement_seed: int = 10, image_crf: int | None = None, output_type: str = 'pil', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 1024)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_ltx2_image2video.py#L980)

**Parameters:**

image (`PipelineImageInput`) : The input image to condition the generation on. Must be an image, a list of images or a `torch.Tensor`.

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale < 1`).

height (`int`, *optional*, defaults to `512`) : The height in pixels of the generated image. This is set to 480 by default for the best results.

width (`int`, *optional*, defaults to `768`) : The width in pixels of the generated image. This is set to 848 by default for the best results.

num_frames (`int`, *optional*) : The number of video frames to generate. If not supplied, defaults to an auto-predicted duration when this pipeline has a `duration_head` component (LTX-2.5 checkpoints and later), and to `121` otherwise. Pass an integer to set the length explicitly. Auto-predicted counts are snapped to the VAE's causal temporal grid, so the realized duration is quantized (roughly 0.33s at 24 fps).

min_seconds (`float`, *optional*, defaults to `1.0`) : Lower bound on the auto-predicted duration when `num_frames` is omitted and a `duration_head` is present. Ignored when `num_frames` is set explicitly.

max_seconds (`float`, *optional*, defaults to `20.0`) : Upper bound on the auto-predicted duration when `num_frames` is omitted and a `duration_head` is present. Ignored when `num_frames` is set explicitly. Must be strictly greater than `min_seconds`.

frame_rate (`float`, *optional*, defaults to `24.0`) : The frames per second (FPS) of the generated video.

num_inference_steps (`int`, *optional*, defaults to 30) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`List[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

timesteps (`List[int]`, *optional*) : Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used. Must be in descending order.

guidance_scale (`float`, *optional*, defaults to `4.0`) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality. Used for the video modality (there is a separate value `audio_guidance_scale` for the audio modality).

stg_scale (`float`, *optional*, defaults to `0.0`) : Video guidance scale for Spatio-Temporal Guidance (STG), proposed in [Spatiotemporal Skip Guidance for Enhanced Video Diffusion Sampling](https://arxiv.org/abs/2411.18664). STG uses a CFG-like estimate where we move the sample away from a weak sample from a perturbed version of the denoising model. Enabling STG will result in an additional denoising model forward pass; the default value of `0.0` means that STG is disabled.

modality_scale (`float`, *optional*, defaults to `1.0`) : Video guidance scale for LTX-2.X modality isolation guidance, where we move the sample away from a weaker sample generated by the denoising model withy cross-modality (audio-to-video and video-to-audio) cross attention disabled using a CFG-like estimate. Enabling modality guidance will result in an additional denoising model forward pass; the default value of `1.0` means that modality guidance is disabled.

guidance_rescale (`float`, *optional*, defaults to 0.0) : Guidance rescale factor proposed by [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891) `guidance_scale` is defined as `φ` in equation 16. of [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891). Guidance rescale factor should fix overexposure when using zero terminal SNR. Used for the video modality.

audio_guidance_scale (`float`, *optional* defaults to `None`) : Audio guidance scale for CFG with respect to the negative prompt. The CFG update rule is the same for video and audio, but they can use different values for the guidance scale. The LTX-2.X authors suggest that the `audio_guidance_scale` should be higher relative to the video `guidance_scale` (e.g. for LTX-2.3 they suggest 3.0 for video and 7.0 for audio). If `None`, defaults to the video value `guidance_scale`.

audio_stg_scale (`float`, *optional*, defaults to `None`) : Audio guidance scale for STG. As with CFG, the STG update rule is otherwise the same for video and audio. For LTX-2.3, a value of 1.0 is suggested for both video and audio. If `None`, defaults to the video value `stg_scale`.

audio_modality_scale (`float`, *optional*, defaults to `None`) : Audio guidance scale for LTX-2.X modality isolation guidance. As with CFG, the modality guidance rule is otherwise the same for video and audio. For LTX-2.3, a value of 3.0 is suggested for both video and audio. If `None`, defaults to the video value `modality_scale`.

audio_guidance_rescale (`float`, *optional*, defaults to `None`) : A separate guidance rescale factor for the audio modality. If `None`, defaults to the video value `guidance_rescale`.

spatio_temporal_guidance_blocks (`list[int]`, *optional*, defaults to `None`) : The zero-indexed transformer block indices at which to apply STG. Must be supplied if STG is used (`stg_scale` or `audio_stg_scale` is greater than `0`). A value of `[29]` is recommended for LTX-2.0 and `[28]` is recommended for LTX-2.3.

noise_scale (`float`, *optional*, defaults to `0.0`) : The interpolation factor between random noise and denoised latents at each timestep. Applying noise to the `latents` and `audio_latents` before continue denoising.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : The number of videos to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for video generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

audio_latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for audio generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

prompt_attention_mask (`torch.Tensor`, *optional*) : Pre-generated attention mask for text embeddings.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. For PixArt-Sigma this negative prompt should be "". If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

negative_prompt_attention_mask (`torch.FloatTensor`, *optional*) : Pre-generated attention mask for negative text embeddings.

decode_timestep (`float`, defaults to `0.0`) : The timestep at which generated video is decoded.

decode_noise_scale (`float`, defaults to `None`) : The interpolation factor between random noise and denoised latents at the decode timestep.

use_cross_timestep (`bool` *optional*, defaults to `True`) : Whether to use the cross modality (audio is the cross modality of video, and vice versa) sigma when calculating the cross attention modulation parameters. `True` is the LTX-2.3/2.5 behavior; `False` is the legacy LTX-2.0 behavior.

system_prompt (`str`, *optional*, defaults to `None`) : Optional system prompt to use for prompt enhancement. The system prompt will be used by the prompt enhancer (a Gemma conditional-generation model -- the dedicated `prompt_enhancer` component if one is configured, otherwise the main `text_encoder`) to generate an enhanced prompt from the original `prompt` and the first `image` to condition generation. If not supplied and a dedicated `prompt_enhancer` is configured (LTX-2.5), defaults to `LTX2_5_I2V_DEFAULT_SYSTEM_PROMPT` (from `diffusers.pipelines.ltx2.utils`) -- see `enable_prompt_enhancement`.

enable_prompt_enhancement (`bool`, *optional*, defaults to `False`) : Whether to run prompt enhancement. Opt-in, matching the Lightricks reference pipelines. When `True` and `system_prompt` is omitted, LTX-2.5 uses `LTX2_5_I2V_DEFAULT_SYSTEM_PROMPT` if a dedicated `prompt_enhancer` is configured; LTX-2.0/2.3 require an explicit `system_prompt`.

prompt_max_new_tokens (`int`, *optional*, defaults to `None`) : The maximum number of new tokens to generate when performing prompt enhancement. If not supplied, uses 600 for a dedicated Gemma 4 `prompt_enhancer` (LTX-2.5) or 512 for the Gemma 3 `text_encoder` fallback (LTX-2.0/2.3).

prompt_enhancement_kwargs (`dict[str, Any]`, *optional*, defaults to `None`) : Keyword arguments for the prompt enhancer's `.generate` call. If not supplied, always matches whichever model is doing the enhancing: `do_sample=False, no_repeat_ngram_size=3` (greedy) when using a dedicated `prompt_enhancer` (LTX-2.5), or `do_sample=True, temperature=0.7` for the `text_encoder` fallback (LTX-2.0/2.3). See https://huggingface.co/docs/transformers/main/en/main_classes/text_generation#transformers.GenerationMixin.generate for more details.

prompt_enhancement_seed (`int`, *optional*, defaults to `10`) : Random seed for any random operations during prompt enhancement.

image_crf (`int`, *optional*, defaults to `None`) : H.264 CRF used to re-compress the conditioning `image` before VAE encode, matching the compression the model was trained against. `None` means "use the model default" (33 through LTX-2.3, 18 for LTX-2.5). Pass `0` to skip re-compression. Requires a `PIL.Image.Image` when re-compression runs.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.ltx.LTX2PipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`List`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int`, *optional*, defaults to `1024`) : Maximum sequence length to use with the `prompt`.

**Returns:** `~pipelines.ltx.LTX2PipelineOutput` or `tuple`

If `return_dict` is `True`, `~pipelines.ltx.LTX2PipelineOutput` is returned, otherwise a `tuple` is
returned where the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import LTX2ImageToVideoPipeline
>>> from diffusers.utils import encode_video
>>> from diffusers.utils import load_image

>>> pipe = LTX2ImageToVideoPipeline.from_pretrained("Lightricks/LTX-2", torch_dtype=torch.bfloat16)
>>> pipe.enable_model_cpu_offload()

>>> image = load_image(
...     "https://huggingface.co/datasets/a-r-r-o-w/tiny-meme-dataset-captioned/resolve/main/images/8.png"
... )
>>> prompt = "A young girl stands calmly in the foreground, looking directly at the camera, as a house fire rages in the background."
>>> negative_prompt = "worst quality, inconsistent motion, blurry, jittery, distorted"

>>> frame_rate = 24.0
>>> video, audio = pipe(
...     image=image,
...     prompt=prompt,
...     negative_prompt=negative_prompt,
...     width=768,
...     height=512,
...     num_frames=121,
...     frame_rate=frame_rate,
...     num_inference_steps=30,
...     guidance_scale=3.0,
...     output_type="np",
...     return_dict=False,
... )

>>> encode_video(
...     video[0],
...     fps=frame_rate,
...     audio=audio[0].float().cpu(),
...     audio_sample_rate=pipe.vocoder.config.output_sampling_rate,  # should be 24000
...     output_path="video.mp4",
... )
```

#### encode_prompt[[diffusers.LTX2ImageToVideoPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] | None = None, do_classifier_free_guidance: bool = True, num_videos_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 1024, scale_factor: int = 8, device: typing.Optional[torch.device] = None, dtype: typing.Optional[torch.dtype] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_ltx2_image2video.py#L369)

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

#### enhance_prompt[[diffusers.LTX2ImageToVideoPipeline.enhance_prompt]]

```python
enhance_prompt(prompt: str, system_prompt: str, max_new_tokens: int | None = None, seed: int = 10, generator: typing.Optional[torch.Generator] = None, generation_kwargs: dict[str, typing.Any] | None = None, device: typing.Union[torch.device, str, NoneType] = None, image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_ltx2_image2video.py#L577)

Enhances the supplied `prompt` by generating a new prompt using the prompt enhancer (a Gemma
conditional-generation model) from it and a system prompt. When `image` is supplied, the enhancer is also
conditioned on that reference frame (I2V / keyframe-style enhancement). Uses the dedicated `prompt_enhancer`
component if one is configured (e.g. LTX-2.5, whose text encoder isn't trained for enhancement), otherwise
falls back to the main `text_encoder` (LTX-2.0/2.3, which double as their own enhancer).

Message templates, decoding kwargs, response cleaning, and image long-side prep match `ltx-core` /
`ltx-pipelines` (`enhance_t2v` / `enhance_i2v` / `generate_enhanced_prompt`).

## LTX2ConditionPipeline[[diffusers.LTX2ConditionPipeline]]

#### diffusers.LTX2ConditionPipeline[[diffusers.LTX2ConditionPipeline]]

```python
diffusers.LTX2ConditionPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKLLTX2Video, audio_vae: AutoencoderKLLTX2Audio, text_encoder: transformers.models.gemma3.modeling_gemma3.Gemma3ForConditionalGeneration | transformers.models.gemma4_unified.modeling_gemma4_unified.Gemma4UnifiedForConditionalGeneration, tokenizer: GemmaTokenizer, connectors: LTX2TextConnectors, transformer: LTX2VideoTransformer3DModel, vocoder: diffusers.pipelines.ltx2.vocoder.LTX2Vocoder | diffusers.pipelines.ltx2.vocoder.LTX2VocoderWithBWE, audio_scheduler: diffusers.schedulers.scheduling_flow_match_euler_discrete.FlowMatchEulerDiscreteScheduler | None = None, processor: transformers.processing_utils.ProcessorMixin | None = None, prompt_enhancer: transformers.models.gemma4.modeling_gemma4.Gemma4ForConditionalGeneration | None = None, duration_head: diffusers.pipelines.ltx2.duration_head.LTX2DurationHead | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_ltx2_condition.py#L263)

Pipeline for video generation which allows image conditions to be inserted at arbitary parts of the video.

Reference: https://github.com/Lightricks/LTX-Video

TODO

#### __call__[[diffusers.LTX2ConditionPipeline.__call__]]

```python
__call__(conditions: diffusers.pipelines.ltx2.pipeline_ltx2_condition.LTX2VideoCondition | list[diffusers.pipelines.ltx2.pipeline_ltx2_condition.LTX2VideoCondition] | None = None, prompt: str | list[str] = None, negative_prompt: str | list[str] | None = None, height: int = 512, width: int = 768, num_frames: int | None = None, min_seconds: float = 1.0, max_seconds: float = 20.0, frame_rate: float = 24.0, num_inference_steps: int = 30, sigmas: list[float] | None = None, timesteps: list[float] | None = None, guidance_scale: float = 3.0, stg_scale: float = 1.0, modality_scale: float = 3.0, guidance_rescale: float = 0.7, audio_guidance_scale: float | None = 7.0, audio_stg_scale: float | None = 1.0, audio_modality_scale: float | None = 3.0, audio_guidance_rescale: float | None = 0.7, spatio_temporal_guidance_blocks: list[int] | None = [28], noise_scale: float | None = None, num_videos_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, audio_latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, decode_timestep: float | list[float] = 0.0, decode_noise_scale: float | list[float] | None = None, use_cross_timestep: bool = True, system_prompt: str | None = None, enable_prompt_enhancement: bool = False, prompt_max_new_tokens: int | None = None, prompt_enhancement_kwargs: dict[str, typing.Any] | None = None, prompt_enhancement_seed: int = 10, output_type: str = 'pil', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 1024)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_ltx2_condition.py#L1345)

**Parameters:**

conditions (`List[LTXVideoCondition], *optional*`) : The list of frame-conditioning items for the video generation.

prompt (`str` or `List[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `List[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale < 1`).

height (`int`, *optional*, defaults to `512`) : The height in pixels of the generated image. This is set to 480 by default for the best results.

width (`int`, *optional*, defaults to `768`) : The width in pixels of the generated image. This is set to 848 by default for the best results.

num_frames (`int`, *optional*) : The number of video frames to generate. If not supplied, defaults to an auto-predicted duration when this pipeline has a `duration_head` component (LTX-2.5 checkpoints and later), and to `121` otherwise. Pass an integer to set the length explicitly. Auto-predicted counts are snapped to the VAE's causal temporal grid, so the realized duration is quantized (roughly 0.33s at 24 fps).

min_seconds (`float`, *optional*, defaults to `1.0`) : Lower bound on the auto-predicted duration when `num_frames` is omitted and a `duration_head` is present. Ignored when `num_frames` is set explicitly.

max_seconds (`float`, *optional*, defaults to `20.0`) : Upper bound on the auto-predicted duration when `num_frames` is omitted and a `duration_head` is present. Ignored when `num_frames` is set explicitly. Must be strictly greater than `min_seconds`.

frame_rate (`float`, *optional*, defaults to `24.0`) : The frames per second (FPS) of the generated video.

num_inference_steps (`int`, *optional*, defaults to 30) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`List[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

timesteps (`List[int]`, *optional*) : Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used. Must be in descending order.

guidance_scale (`float`, *optional*, defaults to `4.0`) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality. Used for the video modality (there is a separate value `audio_guidance_scale` for the audio modality).

stg_scale (`float`, *optional*, defaults to `0.0`) : Video guidance scale for Spatio-Temporal Guidance (STG), proposed in [Spatiotemporal Skip Guidance for Enhanced Video Diffusion Sampling](https://arxiv.org/abs/2411.18664). STG uses a CFG-like estimate where we move the sample away from a weak sample from a perturbed version of the denoising model. Enabling STG will result in an additional denoising model forward pass; the default value of `0.0` means that STG is disabled.

modality_scale (`float`, *optional*, defaults to `1.0`) : Video guidance scale for LTX-2.X modality isolation guidance, where we move the sample away from a weaker sample generated by the denoising model withy cross-modality (audio-to-video and video-to-audio) cross attention disabled using a CFG-like estimate. Enabling modality guidance will result in an additional denoising model forward pass; the default value of `1.0` means that modality guidance is disabled.

guidance_rescale (`float`, *optional*, defaults to 0.0) : Guidance rescale factor proposed by [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891) `guidance_scale` is defined as `φ` in equation 16. of [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891). Guidance rescale factor should fix overexposure when using zero terminal SNR. Used for the video modality.

audio_guidance_scale (`float`, *optional* defaults to `None`) : Audio guidance scale for CFG with respect to the negative prompt. The CFG update rule is the same for video and audio, but they can use different values for the guidance scale. The LTX-2.X authors suggest that the `audio_guidance_scale` should be higher relative to the video `guidance_scale` (e.g. for LTX-2.3 they suggest 3.0 for video and 7.0 for audio). If `None`, defaults to the video value `guidance_scale`.

audio_stg_scale (`float`, *optional*, defaults to `None`) : Audio guidance scale for STG. As with CFG, the STG update rule is otherwise the same for video and audio. For LTX-2.3, a value of 1.0 is suggested for both video and audio. If `None`, defaults to the video value `stg_scale`.

audio_modality_scale (`float`, *optional*, defaults to `None`) : Audio guidance scale for LTX-2.X modality isolation guidance. As with CFG, the modality guidance rule is otherwise the same for video and audio. For LTX-2.3, a value of 3.0 is suggested for both video and audio. If `None`, defaults to the video value `modality_scale`.

audio_guidance_rescale (`float`, *optional*, defaults to `None`) : A separate guidance rescale factor for the audio modality. If `None`, defaults to the video value `guidance_rescale`.

spatio_temporal_guidance_blocks (`list[int]`, *optional*, defaults to `None`) : The zero-indexed transformer block indices at which to apply STG. Must be supplied if STG is used (`stg_scale` or `audio_stg_scale` is greater than `0`). A value of `[29]` is recommended for LTX-2.0 and `[28]` is recommended for LTX-2.3.

noise_scale (`float`, *optional*, defaults to `None`) : The interpolation factor between random noise and denoised latents at each timestep. Applying noise to the `latents` and `audio_latents` before continue denoising. If not set, will be inferred from the sigma schedule.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : The number of videos to generate per prompt.

generator (`torch.Generator` or `List[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for video generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

audio_latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for audio generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

prompt_attention_mask (`torch.Tensor`, *optional*) : Pre-generated attention mask for text embeddings.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. For PixArt-Sigma this negative prompt should be "". If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

negative_prompt_attention_mask (`torch.FloatTensor`, *optional*) : Pre-generated attention mask for negative text embeddings.

decode_timestep (`float`, defaults to `0.0`) : The timestep at which generated video is decoded.

decode_noise_scale (`float`, defaults to `None`) : The interpolation factor between random noise and denoised latents at the decode timestep.

use_cross_timestep (`bool` *optional*, defaults to `True`) : Whether to use the cross modality (audio is the cross modality of video, and vice versa) sigma when calculating the cross attention modulation parameters. `True` is the LTX-2.3/2.5 behavior; `False` is the legacy LTX-2.0 behavior.

system_prompt (`str`, *optional*, defaults to `None`) : Optional system prompt to use for prompt enhancement. The system prompt will be used by the prompt enhancer (a Gemma conditional-generation model -- the dedicated `prompt_enhancer` component if one is configured, otherwise the main `text_encoder`) to generate an enhanced prompt from the original `prompt` (and a conditioning image when one is available) to condition generation. If not supplied and a dedicated `prompt_enhancer` is configured (LTX-2.5), defaults to `LTX2_5_I2V_DEFAULT_SYSTEM_PROMPT` when a conditioning image is available, otherwise `LTX2_5_T2V_DEFAULT_SYSTEM_PROMPT` -- see `enable_prompt_enhancement`.

enable_prompt_enhancement (`bool`, *optional*, defaults to `False`) : Whether to run prompt enhancement. Opt-in, matching the Lightricks reference pipelines. When `True` and `system_prompt` is omitted, LTX-2.5 picks `LTX2_5_I2V_DEFAULT_SYSTEM_PROMPT` / `LTX2_5_T2V_DEFAULT_SYSTEM_PROMPT` based on whether a conditioning image is available.

prompt_max_new_tokens (`int`, *optional*, defaults to `None`) : The maximum number of new tokens to generate when performing prompt enhancement. If not supplied, uses 600 for a dedicated Gemma 4 `prompt_enhancer` (LTX-2.5) or 512 for the Gemma 3 `text_encoder` fallback (LTX-2.0/2.3).

prompt_enhancement_kwargs (`dict[str, Any]`, *optional*, defaults to `None`) : Keyword arguments for the prompt enhancer's `.generate` call. If not supplied, always matches whichever model is doing the enhancing: `do_sample=False, no_repeat_ngram_size=5` (greedy) when using a dedicated `prompt_enhancer` (LTX-2.5), or `do_sample=True, temperature=0.7` for the `text_encoder` fallback (LTX-2.0/2.3). See https://huggingface.co/docs/transformers/main/en/main_classes/text_generation#transformers.GenerationMixin.generate for more details.

prompt_enhancement_seed (`int`, *optional*, defaults to `10`) : Random seed for any random operations during prompt enhancement.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.ltx.LTX2PipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`List`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int`, *optional*, defaults to `1024`) : Maximum sequence length to use with the `prompt`.

**Returns:** `~pipelines.ltx.LTX2PipelineOutput` or `tuple`

If `return_dict` is `True`, `~pipelines.ltx.LTX2PipelineOutput` is returned, otherwise a `tuple` is
returned where the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import LTX2ConditionPipeline
>>> from diffusers.utils import encode_video
>>> from diffusers.pipelines.ltx2.pipeline_ltx2_condition import LTX2VideoCondition
>>> from diffusers.utils import load_image

>>> pipe = LTX2ConditionPipeline.from_pretrained("Lightricks/LTX-2", torch_dtype=torch.bfloat16)
>>> pipe.enable_model_cpu_offload()

>>> first_image = load_image(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/flf2v_input_first_frame.png"
... )
>>> last_image = load_image(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/flf2v_input_last_frame.png"
... )
>>> first_cond = LTX2VideoCondition(frames=first_image, index=0, strength=1.0)
>>> last_cond = LTX2VideoCondition(frames=last_image, index=-1, strength=1.0)
>>> conditions = [first_cond, last_cond]
>>> prompt = "CG animation style, a small blue bird takes off from the ground, flapping its wings."
>>> negative_prompt = "worst quality, inconsistent motion, blurry, jittery, distorted, static"

>>> frame_rate = 24.0
>>> video = pipe(
...     conditions=conditions,
...     prompt=prompt,
...     negative_prompt=negative_prompt,
...     width=768,
...     height=512,
...     num_frames=121,
...     frame_rate=frame_rate,
...     num_inference_steps=30,
...     guidance_scale=3.0,
...     output_type="np",
...     return_dict=False,
... )
>>> video = (video * 255).round().astype("uint8")
>>> video = torch.from_numpy(video)

>>> encode_video(
...     video[0],
...     fps=frame_rate,
...     audio=audio[0].float().cpu(),
...     audio_sample_rate=pipe.vocoder.config.output_sampling_rate,  # should be 24000
...     output_path="video.mp4",
... )
```

#### apply_first_frame_conditioning[[diffusers.LTX2ConditionPipeline.apply_first_frame_conditioning]]

```python
apply_first_frame_conditioning(latents: Tensor, conditioning_mask: Tensor, condition_latents: list, condition_strengths: list, condition_indices: list, latent_height: int, latent_width: int)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_ltx2_condition.py#L961)

**Parameters:**

latents (`torch.Tensor`) : Initial packed (patchified) latents of shape [batch_size, patch_seq_len, hidden_dim].

conditioning_mask (`torch.Tensor`) : Initial packed (patchified) conditioning mask of shape [batch_size, patch_seq_len, 1] with values in [0, 1] where 0 means the denoising model output will be fully used and 1 means the condition will be fully used.

**Returns:** `Tuple[torch.Tensor, torch.Tensor, torch.Tensor]`

Returns a 3-tuple of tensors where:
1. The packed video latents with first-frame conditions applied.
2. The packed conditioning mask with first-frame strengths applied.
3. The clean conditioning latents at first-frame positions (zeros elsewhere).

Apply first-frame visual conditioning by overwriting tokens at the first-frame positions.

Only conditions with `latent_idx == 0` are applied here (matching `VideoConditionByLatentIndex` in the
reference implementation). Conditions at non-zero latent indices are appended as separate keyframe tokens via
`prepare_keyframe_extras` (matching `VideoConditionByKeyframeIndex`) and are skipped here.

#### encode_prompt[[diffusers.LTX2ConditionPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] | None = None, do_classifier_free_guidance: bool = True, num_videos_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 1024, scale_factor: int = 8, device: typing.Optional[torch.device] = None, dtype: typing.Optional[torch.dtype] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_ltx2_condition.py#L416)

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

#### enhance_prompt[[diffusers.LTX2ConditionPipeline.enhance_prompt]]

```python
enhance_prompt(prompt: str, system_prompt: str, max_new_tokens: int | None = None, seed: int = 10, generator: typing.Optional[torch.Generator] = None, generation_kwargs: dict[str, typing.Any] | None = None, device: typing.Union[torch.device, str, NoneType] = None, image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_ltx2_condition.py#L624)

Enhances the supplied `prompt` by generating a new prompt using the prompt enhancer (a Gemma
conditional-generation model) from it and a system prompt. When `image` is supplied, the enhancer is also
conditioned on that reference frame (I2V / keyframe-style enhancement). Uses the dedicated `prompt_enhancer`
component if one is configured (e.g. LTX-2.5, whose text encoder isn't trained for enhancement), otherwise
falls back to the main `text_encoder` (LTX-2.0/2.3, which double as their own enhancer).

Message templates, decoding kwargs, response cleaning, and image long-side prep match `ltx-core` /
`ltx-pipelines` (`enhance_t2v` / `enhance_i2v` / `generate_enhanced_prompt`).

#### prepare_latents[[diffusers.LTX2ConditionPipeline.prepare_latents]]

```python
prepare_latents(conditions: diffusers.pipelines.ltx2.pipeline_ltx2_condition.LTX2VideoCondition | list[diffusers.pipelines.ltx2.pipeline_ltx2_condition.LTX2VideoCondition] | None = None, batch_size: int = 1, num_channels_latents: int = 128, height: int = 512, width: int = 768, num_frames: int = 121, frame_rate: float = 24.0, noise_scale: float = 1.0, dtype: typing.Optional[torch.dtype] = None, device: typing.Optional[torch.device] = None, generator: typing.Optional[torch.Generator] = None, latents: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_ltx2_condition.py#L1068)

Prepare noisy video latents, applying frame conditions.

First-frame conditions (`latent_idx == 0`) are applied by overwriting tokens at the first-frame positions
(`VideoConditionByLatentIndex` semantics). Non-first-frame conditions (`latent_idx > 0`) are concatenated onto
the main latent sequence with per-token `conditioning_mask = strength` (`VideoConditionByKeyframeIndex`
semantics) — the denoising loop's existing timestep formula `t * (1 - conditioning_mask)` and post-process
blend `denoised * (1 - conditioning_mask) + clean * conditioning_mask` then drive them across steps.

Returns a 4-tuple:
- `latents`: packed noisy latents (base tokens + any keyframe tokens cat'd onto the sequence dim).
- `conditioning_mask`: packed conditioning mask with values in `[0, 1]` — `1` at first-frame positions,
  `strength` at keyframe positions, `0` elsewhere.
- `clean_latents`: clean condition values at conditioned positions (zeros elsewhere); same shape as
  `latents`.
- `keyframe_coords`: `[B, 3, num_keyframe_patches, 2]` positional coordinates to append to `video_coords`,
  or `None` if there are no non-first-frame conditions.

#### preprocess_conditions[[diffusers.LTX2ConditionPipeline.preprocess_conditions]]

```python
preprocess_conditions(conditions: diffusers.pipelines.ltx2.pipeline_ltx2_condition.LTX2VideoCondition | list[diffusers.pipelines.ltx2.pipeline_ltx2_condition.LTX2VideoCondition] | None = None, height: int = 512, width: int = 768, num_frames: int = 121, device: typing.Optional[torch.device] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_ltx2_condition.py#L843)

**Parameters:**

conditions (`LTX2VideoCondition` or `List[LTX2VideoCondition]`, *optional*, defaults to `None`) : A list of image/video condition instances.

height (`int`, *optional*, defaults to `512`) : The desired height in pixels.

width (`int`, *optional*, defaults to `768`) : The desired width in pixels.

num_frames (`int`, *optional*, defaults to `121`) : The desired number of frames in the generated video.

device (`torch.device`, *optional*, defaults to `None`) : The device on which to put the preprocessed image/video tensors.

**Returns:** `Tuple[List[torch.Tensor], List[float], List[int], List[int]]`

Returns a 4-tuple of lists of length `len(conditions)` as follows:
1. The first list is a list of preprocessed video tensors of shape [batch_size=1, num_channels,
   num_frames, height, width].
2. The second list is a list of conditioning strengths.
3. The third list is a list of latent-space indices for each condition.
4. The fourth list is a list of (trimmed) pixel-space frame counts per condition. This is needed
   for keyframe coord semantics (single-pixel-frame keyframes have a clamped temporal extent).

Preprocesses the condition images/videos to torch tensors.

#### trim_conditioning_sequence[[diffusers.LTX2ConditionPipeline.trim_conditioning_sequence]]

```python
trim_conditioning_sequence(start_frame: int, sequence_num_frames: int, target_num_frames: int)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_ltx2_condition.py#L826)

**Parameters:**

start_frame (int) : The target frame number of the first frame in the sequence.

sequence_num_frames (int) : The number of frames in the sequence.

target_num_frames (int) : The target number of frames in the generated video.

**Returns:** `int`

updated sequence length

Trim a conditioning sequence to the allowed number of frames.

## LTX2LatentUpsamplePipeline[[diffusers.LTX2LatentUpsamplePipeline]]

#### diffusers.LTX2LatentUpsamplePipeline[[diffusers.LTX2LatentUpsamplePipeline]]

```python
diffusers.LTX2LatentUpsamplePipeline(vae: AutoencoderKLLTX2Video, latent_upsampler: LTX2LatentUpsamplerModel)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_ltx2_latent_upsample.py#L104)

#### __call__[[diffusers.LTX2LatentUpsamplePipeline.__call__]]

```python
__call__(video: list[typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]] | None = None, height: int = 512, width: int = 768, num_frames: int = 121, spatial_patch_size: int = 1, temporal_patch_size: int = 1, latents: typing.Optional[torch.Tensor] = None, latents_normalized: bool = False, decode_timestep: float | list[float] = 0.0, decode_noise_scale: float | list[float] | None = None, adain_factor: float = 0.0, tone_map_compression_ratio: float = 0.0, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, output_type: str | None = 'pil', return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_ltx2_latent_upsample.py#L264)

**Parameters:**

video (`list[PipelineImageInput]`, *optional*) : The video to be upsampled (such as a LTX 2.0 first stage output). If not supplied, `latents` should be supplied.

height (`int`, *optional*, defaults to `512`) : The height in pixels of the input video (not the generated video, which will have a larger resolution).

width (`int`, *optional*, defaults to `768`) : The width in pixels of the input video (not the generated video, which will have a larger resolution).

num_frames (`int`, *optional*, defaults to `121`) : The number of frames in the input video.

spatial_patch_size (`int`, *optional*, defaults to `1`) : The spatial patch size of the video latents. Used when `latents` is supplied if unpacking is necessary.

temporal_patch_size (`int`, *optional*, defaults to `1`) : The temporal patch size of the video latents. Used when `latents` is supplied if unpacking is necessary.

latents (`torch.Tensor`, *optional*) : Pre-generated video latents. This can be supplied in place of the `video` argument. Can either be a patch sequence of shape `(batch_size, seq_len, hidden_dim)` or a video latent of shape `(batch_size, latent_channels, latent_frames, latent_height, latent_width)`.

latents_normalized (`bool`, *optional*, defaults to `False`) : If `latents` are supplied, whether the `latents` are normalized using the VAE latent mean and std. If `True`, the `latents` will be denormalized before being supplied to the latent upsampler.

decode_timestep (`float`, defaults to `0.0`) : The timestep at which generated video is decoded.

decode_noise_scale (`float`, defaults to `None`) : The interpolation factor between random noise and denoised latents at the decode timestep.

adain_factor (`float`, *optional*, defaults to `0.0`) : Adaptive Instance Normalization (AdaIN) blending factor between the upsampled and original latents. Should be in [-10.0, 10.0]; supplying 0.0 (the default) means that AdaIN is not performed.

tone_map_compression_ratio (`float`, *optional*, defaults to `0.0`) : The compression strength for tone mapping, which will reduce the dynamic range of the latent values. This is useful for regularizing high-variance latents or for conditioning outputs during generation. Should be in [0, 1], where 0.0 (the default) means tone mapping is not applied and 1.0 corresponds to the full compression effect.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.ltx.LTXPipelineOutput` instead of a plain tuple.

**Returns:** `~pipelines.ltx.LTXPipelineOutput` or `tuple`

If `return_dict` is `True`, `~pipelines.ltx.LTXPipelineOutput` is returned, otherwise a `tuple` is
returned where the first element is the upsampled video.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import LTX2ImageToVideoPipeline, LTX2LatentUpsamplePipeline
>>> from diffusers.utils import encode_video
>>> from diffusers.pipelines.ltx2.latent_upsampler import LTX2LatentUpsamplerModel
>>> from diffusers.utils import load_image

>>> pipe = LTX2ImageToVideoPipeline.from_pretrained("Lightricks/LTX-2", torch_dtype=torch.bfloat16)
>>> pipe.enable_model_cpu_offload()

>>> image = load_image(
...     "https://huggingface.co/datasets/a-r-r-o-w/tiny-meme-dataset-captioned/resolve/main/images/8.png"
... )
>>> prompt = "A young girl stands calmly in the foreground, looking directly at the camera, as a house fire rages in the background."
>>> negative_prompt = "worst quality, inconsistent motion, blurry, jittery, distorted"

>>> frame_rate = 24.0
>>> video, audio = pipe(
...     image=image,
...     prompt=prompt,
...     negative_prompt=negative_prompt,
...     width=768,
...     height=512,
...     num_frames=121,
...     frame_rate=frame_rate,
...     num_inference_steps=40,
...     guidance_scale=4.0,
...     output_type="pil",
...     return_dict=False,
... )

>>> latent_upsampler = LTX2LatentUpsamplerModel.from_pretrained(
...     "Lightricks/LTX-2", subfolder="latent_upsampler", torch_dtype=torch.bfloat16
... )
>>> upsample_pipe = LTX2LatentUpsamplePipeline(vae=pipe.vae, latent_upsampler=latent_upsampler)
>>> upsample_pipe.vae.enable_tiling()
>>> upsample_pipe.to(device="cuda", dtype=torch.bfloat16)

>>> video = upsample_pipe(
...     video=video,
...     width=768,
...     height=512,
...     output_type="np",
...     return_dict=False,
... )[0]

>>> encode_video(
...     video[0],
...     fps=frame_rate,
...     audio=audio[0].float().cpu(),
...     audio_sample_rate=pipe.vocoder.config.output_sampling_rate,  # should be 24000
...     output_path="video.mp4",
... )
```

#### adain_filter_latent[[diffusers.LTX2LatentUpsamplePipeline.adain_filter_latent]]

```python
adain_filter_latent(latents: Tensor, reference_latents: Tensor, factor: float = 1.0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_ltx2_latent_upsample.py#L168)

**Parameters:**

latent (`torch.Tensor`) : Input latents to normalize

reference_latents (`torch.Tensor`) : The reference latents providing style statistics.

factor (`float`) : Blending factor between original and transformed latent. Range: -10.0 to 10.0, Default: 1.0

**Returns:** `torch.Tensor`

The transformed latent tensor

Applies Adaptive Instance Normalization (AdaIN) to a latent tensor based on statistics from a reference latent
tensor.

#### tone_map_latents[[diffusers.LTX2LatentUpsamplePipeline.tone_map_latents]]

```python
tone_map_latents(latents: Tensor, compression: float)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_ltx2_latent_upsample.py#L196)

**Parameters:**

latents : torch.Tensor Input latent tensor with arbitrary shape. Expected to be roughly in [-1, 1] or [0, 1] range.

compression : float Compression strength in the range [0, 1]. - 0.0: No tone-mapping (identity transform) - 1.0: Full compression effect

**Returns:**

torch.Tensor
The tone-mapped latent tensor of the same shape as input.

Applies a non-linear tone-mapping function to latent values to reduce their dynamic range in a perceptually
smooth way using a sigmoid-based compression.

This is useful for regularizing high-variance latents or for conditioning outputs during generation, especially
when controlling dynamic behavior with a `compression` factor.

## LTX2VideoDiffusionDecodePipeline[[diffusers.LTX2VideoDiffusionDecodePipeline]]

#### diffusers.LTX2VideoDiffusionDecodePipeline[[diffusers.LTX2VideoDiffusionDecodePipeline]]

```python
diffusers.LTX2VideoDiffusionDecodePipeline(diffusion_decoder: LTX2VideoDiffusionDecoderModel, scheduler, vae: AutoencoderKLLTX2Video = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_ltx2_diffusion_decode.py#L27)

**Parameters:**

diffusion_decoder ([LTX2VideoDiffusionDecoderModel](/docs/diffusers/v0.40.0/en/api/models/ltx2_diffusion_decoder#diffusers.LTX2VideoDiffusionDecoderModel)) : The diffusion video decoder.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : Scheduler driving the decoder's denoising steps.

vae ([AutoencoderKLLTX2Video](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl_ltx_2#diffusers.AutoencoderKLLTX2Video), *optional*) : Only consulted for the latent statistics used to denormalize. When omitted the pipeline falls back to the LTX-2 defaults, so a decode-only workflow does not have to load a second autoencoder.

Decode LTX-2 video latents with the diffusion decoder introduced in LTX-2.5.

Unlike a convolutional decoder this one is itself a small diffusion model: it denoises pixels conditioned on a
context volume built from the latents, so it needs a scheduler and a generator. Pair it with any LTX-2 pipeline run
with `output_type="latent"`, passing `denormalize=False` since that path already applied the latent statistics.

#### __call__[[diffusers.LTX2VideoDiffusionDecodePipeline.__call__]]

```python
__call__(latents: Tensor, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, output_type: str = 'pil', return_dict: bool = True, denormalize: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_ltx2_diffusion_decode.py#L79)

**Parameters:**

latents (`torch.Tensor`) : Latents of shape `(B, C, F, H, W)`. Note that an LTX-2 pipeline run with `output_type="latent"` returns latents that are *already* denormalized, so pass `denormalize=False` for those.

generator (`torch.Generator`, *optional*) : The decoder samples the noise it denoises, so pass a generator to make decoding reproducible.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the decoded video. Choose between `"pil"`, `"np"`, `"pt"` and `"latent"`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a `LTX2VideoDecodeOutput` instead of a plain tuple.

denormalize (`bool`, *optional*, defaults to `True`) : Whether to apply the latent statistics before decoding. Set to `False` if the latents are already denormalized.

**Returns:** `LTX2VideoDecodeOutput` or `tuple`

## LTX2DurationHead[[diffusers.pipelines.ltx2.LTX2DurationHead]]

#### diffusers.pipelines.ltx2.LTX2DurationHead[[diffusers.pipelines.ltx2.LTX2DurationHead]]

```python
diffusers.pipelines.ltx2.LTX2DurationHead(video_cross_attention_dim: int = 4096, audio_cross_attention_dim: int = 2048, pooler_hidden_dim: int = 256, num_queries: int = 1, num_pooler_heads: int = 4, mlp_hidden_dim: int = 256)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/duration_head.py#L81)

**Parameters:**

video_cross_attention_dim (`int`, defaults to `4096`) : Width of the video connector output.

audio_cross_attention_dim (`int`, defaults to `2048`) : Width of the audio connector output.

pooler_hidden_dim (`int`, defaults to `256`) : Shared hidden dimension both modalities are projected into.

num_queries (`int`, defaults to `1`) : Number of learnable pooling queries.

num_pooler_heads (`int`, defaults to `4`) : Attention heads used by the pooler.

mlp_hidden_dim (`int`, defaults to `256`) : Hidden width of the output MLP. Named with a `_dim` suffix to avoid colliding with the `mlp_hidden` submodule, which `ConfigMixin.__getattr__` would otherwise shadow with this config value.

Predicts the natural duration of the shot implied by a caption, from the LTX-2 text connector outputs.

The head is modality-agnostic: pass either or both of the video and audio connector outputs. Modality-specific
input projections map each stream into a shared pooler dimension, learnable modality embeddings tag the streams so
the pooler can tell them apart, and a small MLP turns the pooled vector into a log-duration. The regression target
is trained in log-seconds, so `forward` exponentiates and callers always get seconds.

Ships from LTX-2.5 checkpoints onward.

#### forward[[diffusers.pipelines.ltx2.LTX2DurationHead.forward]]

```python
forward(video_tokens: typing.Optional[torch.Tensor] = None, audio_tokens: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/duration_head.py#L134)

**Parameters:**

video_tokens (`torch.Tensor` of shape `(batch_size, seq_len, video_cross_attention_dim)`, *optional*) : Video connector output.

audio_tokens (`torch.Tensor` of shape `(batch_size, seq_len, audio_cross_attention_dim)`, *optional*) : Audio connector output.

**Returns:** `torch.Tensor` of shape `(batch_size,)`

the predicted duration in seconds.

#### predict_num_frames[[diffusers.pipelines.ltx2.LTX2DurationHead.predict_num_frames]]

```python
predict_num_frames(video_tokens: typing.Optional[torch.Tensor] = None, audio_tokens: typing.Optional[torch.Tensor] = None, frame_rate: float, temporal_compression_ratio: int, min_seconds: float = 1.0, max_seconds: float = 20.0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/duration_head.py#L172)

**Parameters:**

video_tokens (`torch.Tensor`, *optional*) : Video connector output for a single prompt.

audio_tokens (`torch.Tensor`, *optional*) : Audio connector output for a single prompt.

frame_rate (`float`) : Frames per second used to convert the predicted duration into a frame count.

temporal_compression_ratio (`int`) : The VAE's temporal compression ratio, which defines the frame grid.

min_seconds (`float`, defaults to `1.0`) : Lower bound on the prediction.

max_seconds (`float`, defaults to `20.0`) : Upper bound on the prediction.

**Returns:** `int`

a frame count lying on the VAE's temporal grid.

Predicts a frame count from connector tokens, clamped to `[min_seconds, max_seconds]` and snapped to the VAE's
causal temporal grid (`k * temporal_compression_ratio + 1`).

The clamp is applied before snapping: a clamped frame count is not necessarily grid-aligned, so snapping first
would give a different result. Because snapping floors, it can land below the minimum; when that happens the
result is snapped up to the next grid point instead, so the frame count stays within bounds.

Narrow bounds can convert to a frame window containing no grid point at all -- at 24 fps, `[1.0s, 1.02s]`
rounds to `[24, 24]`, and 24 is not `8k + 1`. The nearest grid point is used and a warning is logged, since
overshooting by under one grid step beats refusing to generate. The returned count is therefore always on the
grid, but may fall just outside the requested bounds in this case.

## LTX2PipelineOutput[[diffusers.pipelines.ltx2.pipeline_output.LTX2PipelineOutput]]

#### diffusers.pipelines.ltx2.pipeline_output.LTX2PipelineOutput[[diffusers.pipelines.ltx2.pipeline_output.LTX2PipelineOutput]]

```python
diffusers.pipelines.ltx2.pipeline_output.LTX2PipelineOutput(frames: Tensor, audio: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx2/pipeline_output.py#L9)

**Parameters:**

frames (`torch.Tensor`, `np.ndarray`, or list[list[PIL.Image.Image]]) : List of video outputs - It can be a nested list of length `batch_size,` with each sub-list containing denoised PIL image sequences of length `num_frames.` It can also be a NumPy array or Torch tensor of shape `(batch_size, num_frames, channels, height, width)`.

audio (`torch.Tensor`, `np.ndarray`) : TODO

Output class for LTX pipelines.
