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

# Wan-Animate-2

[Wan-Animate-2](https://github.com/Wan-Video/Wan2.2) by the Alibaba Wan Team animates a reference character image with the motion of a driving video. The driving video is processed in fixed-length segments: each segment runs a reference-extraction pass that caches the driving segment's K/V in every transformer layer, denoises against that cache, and is decoded inside the loop because the next segment conditions on the previous segment's decoded tail frames.

Two presets are available: the base checkpoint samples with classifier-free guidance, and the distilled checkpoint samples in few steps without it (its guider is pinned to `guidance_scale=1.0`).

```python
import torch
from diffusers import ModularPipeline
from diffusers.utils import export_to_video, load_image, load_video

pipe = ModularPipeline.from_pretrained("Wan-AI/Wan2.2-Animate-2-14B-Diffusers")
pipe.load_components(dtype=torch.bfloat16)

# The transformer weights and the per-segment reference KV cache do not co-reside on one 80 GB
# card at the default resolution, so stream the transformer's blocks. Compiling the blocks is
# required as the in-context attention runs on the flex backend
pipe.transformer.enable_group_offload(
    onload_device=torch.device("cuda"),
    offload_device=torch.device("cpu"),
    offload_type="block_level",
    use_stream=True,
)
pipe.text_encoder.to("cuda")
pipe.image_encoder.to("cuda")
pipe.vae.to("cuda")
pipe.transformer.compile_repeated_blocks(fullgraph=False)

# The first demo from the official repository: https://github.com/Wan-Video/Wan-Animate-2
demo = "https://raw.githubusercontent.com/Wan-Video/Wan-Animate-2/main/examples/demo1"
image = load_image(f"{demo}/reference.png")
driving_video, driving_video_fps = load_video(f"{demo}/template.mp4", return_fps=True)
prompt = "人物外观描述：一只银灰色虎斑纹的小猫，拥有圆润的脸庞、竖立的耳朵和巨大的圆形眼睛。它身穿一套深蓝色的制服套装，包括一件带有金色纽扣的西装外套和一条百褶裙。外套里面搭配着白色衬衫，领口处系着一个红色的蝴蝶结，袖口露出白色的衬衫边缘。背景描述：背景为纯白色，光线均匀明亮，无其他杂物或装饰。"

videos = pipe(
    image=image,
    driving_video=driving_video,
    driving_video_fps=driving_video_fps,
    prompt=prompt,
    output="videos",
)
export_to_video(videos[0], "output.mp4", fps=24)
```

For the distilled checkpoint, load `Wan-AI/Wan2.2-Animate-2-14B-Distilled-Diffusers` the same way — nothing else changes. Each preset carries its own sampling defaults (40 steps for the base checkpoint, 10 for the distilled one), and no `guidance_scale` argument exists anywhere: guidance is owned by the pipeline's guider component (classifier-free guidance at 3.0 for the base preset, disabled for the distilled one).

`height` and `width` (defaults 800 and 640) set the target *area* of the generated video; the actual frame size keeps the reference image's aspect ratio, and the driving frames are letterboxed to it. Inputs that already sit at the target letterbox size pass through the preprocessing untouched, so preprocessing can also be done entirely outside the pipeline.

## WanAnimate2ModularPipeline[[diffusers.WanAnimate2ModularPipeline]]

#### diffusers.WanAnimate2ModularPipeline[[diffusers.WanAnimate2ModularPipeline]]

```python
diffusers.WanAnimate2ModularPipeline(blocks: diffusers.modular_pipelines.modular_pipeline.ModularPipelineBlocks | None = None, pretrained_model_name_or_path: str | os.PathLike | None = None, components_manager: diffusers.modular_pipelines.components_manager.ComponentsManager | None = None, collection: str | None = None, workflow: str | None = None, modular_config_dict: dict[str, typing.Any] | None = None, config_dict: dict[str, typing.Any] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/wan_animate_2/modular_pipeline.py#L23)

A ModularPipeline for Wan-Animate-2 character animation.

## WanAnimate2DistilledModularPipeline[[diffusers.WanAnimate2DistilledModularPipeline]]

#### diffusers.WanAnimate2DistilledModularPipeline[[diffusers.WanAnimate2DistilledModularPipeline]]

```python
diffusers.WanAnimate2DistilledModularPipeline(blocks: diffusers.modular_pipelines.modular_pipeline.ModularPipelineBlocks | None = None, pretrained_model_name_or_path: str | os.PathLike | None = None, components_manager: diffusers.modular_pipelines.components_manager.ComponentsManager | None = None, collection: str | None = None, workflow: str | None = None, modular_config_dict: dict[str, typing.Any] | None = None, config_dict: dict[str, typing.Any] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/wan_animate_2/modular_pipeline.py#L62)

A ModularPipeline for the distilled Wan-Animate-2 model, which samples in few steps without classifier-free
guidance.

## WanAnimate2Blocks[[diffusers.WanAnimate2Blocks]]

#### diffusers.WanAnimate2Blocks[[diffusers.WanAnimate2Blocks]]

```python
diffusers.WanAnimate2Blocks()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/wan_animate_2/modular_blocks_wan_animate_2.py#L252)

Modular pipeline blocks for Wan-Animate-2 character animation: a reference character image and a driving video
produce a video of the character following the driving motion.

Components:
text_encoder (`UMT5EncoderModel`) tokenizer (`AutoTokenizer`) image_processor (`WanAnimate2VideoProcessor`)
image_encoder (`CLIPVisionModel`) video_processor (`WanAnimate2VideoProcessor`) vae (`AutoencoderKLWan`)
transformer (`WanAnimate2Transformer3DModel`) scheduler (`SchedulerMixin`) guider (`ClassifierFreeGuidance`)

Inputs:
prompt (`str`):
The prompt or prompts to guide image generation.
negative_prompt (`str`, *optional*):
The prompt or prompts not to guide the image generation.
prompt_ref (`str`, *optional*, defaults to 人物动作的参考视频):
The reference prompt for the driving video context
max_sequence_length (`int`, *optional*, defaults to 512):
Maximum sequence length for prompt encoding.
image (`Image | list`):
The reference image holding the character to animate.
height (`int`, *optional*, defaults to 800):
Together with `width`, the target *area* of the generated video; the aspect ratio comes from `image`.
Overwritten with the resolved frame height.
width (`int`, *optional*, defaults to 640):
See `height`. Overwritten with the resolved frame width.
driving_video (`list`):
The driving video that provides the motion, in any format accepted by `VideoProcessor.preprocess_video`.
driving_video_fps (`float`, *optional*):
The frame rate `driving_video` was captured at — `load_video(..., return_fps=True)` reports it. When set,
the driving frames are resampled from it to `fps`; when `None` they are used as-is.
fps (`int`, *optional*, defaults to 24):
The frame rate the model generates at
segment_frame_length (`int`, *optional*, defaults to 81):
The number of frames in each inference segment
prev_segment_conditioning_frames (`int`, *optional*, defaults to 1):
The number of conditioning frames carried over from the previous segment
generator (`Generator`, *optional*):
Torch generator for deterministic generation.
num_inference_steps (`int`, *optional*, defaults to 40):
The number of denoising steps.
**denoiser_input_fields (`None`, *optional*):
conditional model inputs for the denoiser: e.g. prompt_embeds, negative_prompt_embeds, etc.
output_type (`str`, *optional*, defaults to np):
The output type of the decoded videos

Outputs:
videos (`list`):
The generated videos.

## WanAnimate2DistilledBlocks[[diffusers.WanAnimate2DistilledBlocks]]

#### diffusers.WanAnimate2DistilledBlocks[[diffusers.WanAnimate2DistilledBlocks]]

```python
diffusers.WanAnimate2DistilledBlocks()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/wan_animate_2/modular_blocks_wan_animate_2_distilled.py#L254)

Modular pipeline blocks for distilled Wan-Animate-2 character animation, sampling in few steps without
classifier-free guidance.

Components:
text_encoder (`UMT5EncoderModel`) tokenizer (`AutoTokenizer`) image_processor (`WanAnimate2VideoProcessor`)
image_encoder (`CLIPVisionModel`) video_processor (`WanAnimate2VideoProcessor`) vae (`AutoencoderKLWan`)
transformer (`WanAnimate2Transformer3DModel`) scheduler (`SchedulerMixin`) guider (`ClassifierFreeGuidance`)

Inputs:
prompt (`str`):
The prompt or prompts to guide image generation.
negative_prompt (`str`, *optional*):
The prompt or prompts not to guide the image generation.
prompt_ref (`str`, *optional*, defaults to 人物动作的参考视频):
The reference prompt for the driving video context
max_sequence_length (`int`, *optional*, defaults to 512):
Maximum sequence length for prompt encoding.
image (`Image | list`):
The reference image holding the character to animate.
height (`int`, *optional*, defaults to 800):
Together with `width`, the target *area* of the generated video; the aspect ratio comes from `image`.
Overwritten with the resolved frame height.
width (`int`, *optional*, defaults to 640):
See `height`. Overwritten with the resolved frame width.
driving_video (`list`):
The driving video that provides the motion, in any format accepted by `VideoProcessor.preprocess_video`.
driving_video_fps (`float`, *optional*):
The frame rate `driving_video` was captured at — `load_video(..., return_fps=True)` reports it. When set,
the driving frames are resampled from it to `fps`; when `None` they are used as-is.
fps (`int`, *optional*, defaults to 24):
The frame rate the model generates at
segment_frame_length (`int`, *optional*, defaults to 81):
The number of frames in each inference segment
prev_segment_conditioning_frames (`int`, *optional*, defaults to 1):
The number of conditioning frames carried over from the previous segment
generator (`Generator`, *optional*):
Torch generator for deterministic generation.
num_inference_steps (`int`, *optional*, defaults to 10):
The number of denoising steps.
**denoiser_input_fields (`None`, *optional*):
conditional model inputs for the denoiser: e.g. prompt_embeds, negative_prompt_embeds, etc.
output_type (`str`, *optional*, defaults to np):
The output type of the decoded videos

Outputs:
videos (`list`):
The generated videos.
