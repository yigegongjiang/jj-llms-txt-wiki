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

  
    
      
    
    
  

# LTX-Video

[LTX-Video](https://huggingface.co/Lightricks/LTX-Video) is a diffusion transformer designed for fast and real-time generation of high-resolution videos from text and images. The main feature of LTX-Video is the Video-VAE. The Video-VAE has a higher pixel to latent compression ratio (1:192) which enables more efficient video data processing and faster generation speed. To support and prevent finer details from being lost during generation, the Video-VAE decoder performs the latent to pixel conversion *and* the last denoising step.

You can find all the original LTX-Video checkpoints under the [Lightricks](https://huggingface.co/Lightricks) organization.

> [!TIP]
> Click on the LTX-Video models in the right sidebar for more examples of other video generation tasks.

The example below demonstrates how to generate a video optimized for memory or inference speed.

Refer to the [Reduce memory usage](../../optimization/memory) guide for more details about the various memory saving techniques.

The LTX-Video model below requires ~10GB of VRAM.

```py
import torch
from diffusers import LTXPipeline, AutoModel
from diffusers.hooks import apply_group_offloading
from diffusers.utils import export_to_video

# fp8 layerwise weight-casting
transformer = AutoModel.from_pretrained(
    "Lightricks/LTX-Video",
    subfolder="transformer",
    dtype=torch.bfloat16
)
transformer.enable_layerwise_casting(
    storage_dtype=torch.float8_e4m3fn, compute_dtype=torch.bfloat16
)

pipeline = LTXPipeline.from_pretrained("Lightricks/LTX-Video", transformer=transformer, dtype=torch.bfloat16)

# group-offloading
onload_device = torch.device("cuda")
offload_device = torch.device("cpu")
pipeline.transformer.enable_group_offload(onload_device=onload_device, offload_device=offload_device, offload_type="leaf_level", use_stream=True)
apply_group_offloading(pipeline.text_encoder, onload_device=onload_device, offload_type="block_level", num_blocks_per_group=2)
apply_group_offloading(pipeline.vae, onload_device=onload_device, offload_type="leaf_level")

prompt = """
A woman with long brown hair and light skin smiles at another woman with long blonde hair.
The woman with brown hair wears a black jacket and has a small, barely noticeable mole on her right cheek.
The camera angle is a close-up, focused on the woman with brown hair's face. The lighting is warm and 
natural, likely from the setting sun, casting a soft glow on the scene. The scene appears to be real-life footage
"""
negative_prompt = "worst quality, inconsistent motion, blurry, jittery, distorted"

video = pipeline(
    prompt=prompt,
    negative_prompt=negative_prompt,
    width=768,
    height=512,
    num_frames=161,
    decode_timestep=0.03,
    decode_noise_scale=0.025,
    num_inference_steps=50,
).frames[0]
export_to_video(video, "output.mp4", fps=24)
```

[Compilation](../../optimization/fp16#torchcompile) is slow the first time but subsequent calls to the pipeline are faster. [Caching](../../optimization/cache) may also speed up inference by storing and reusing intermediate outputs.

```py
import torch
from diffusers import LTXPipeline
from diffusers.utils import export_to_video

pipeline = LTXPipeline.from_pretrained(
    "Lightricks/LTX-Video", dtype=torch.bfloat16
)

# torch.compile
pipeline.transformer.to(memory_format=torch.channels_last)
pipeline.transformer = torch.compile(
    pipeline.transformer, mode="max-autotune", fullgraph=True
)

prompt = """
A woman with long brown hair and light skin smiles at another woman with long blonde hair.
The woman with brown hair wears a black jacket and has a small, barely noticeable mole on her right cheek.
The camera angle is a close-up, focused on the woman with brown hair's face. The lighting is warm and 
natural, likely from the setting sun, casting a soft glow on the scene. The scene appears to be real-life footage
"""
negative_prompt = "worst quality, inconsistent motion, blurry, jittery, distorted"

video = pipeline(
    prompt=prompt,
    negative_prompt=negative_prompt,
    width=768,
    height=512,
    num_frames=161,
    decode_timestep=0.03,
    decode_noise_scale=0.025,
    num_inference_steps=50,
).frames[0]
export_to_video(video, "output.mp4", fps=24)
```

## Notes

- Refer to the following recommended settings for generation from the [LTX-Video](https://github.com/Lightricks/LTX-Video) repository.

  - The recommended dtype for the transformer, VAE, and text encoder is `torch.bfloat16`. The VAE and text encoder can also be `torch.float32` or `torch.float16`.
  - For guidance-distilled variants of LTX-Video, set `guidance_scale` to `1.0`. The `guidance_scale` for any other model should be set higher, like `5.0`, for good generation quality.
  - For timestep-aware VAE variants (LTX-Video 0.9.1 and above), set `decode_timestep` to `0.05` and `image_cond_noise_scale` to `0.025`.
  - For variants that support interpolation between multiple conditioning images and videos (LTX-Video 0.9.5 and above), use similar images and videos for the best results. Divergence from the conditioning inputs may lead to abrupt transitions in the generated video.

- LTX-Video 0.9.7 includes a spatial latent upscaler and a 13B parameter transformer. During inference, a low resolution video is quickly generated first and then upscaled and refined.

  
  Show example code

  ```py
  import torch
  from diffusers import LTXConditionPipeline, LTXLatentUpsamplePipeline
  from diffusers.pipelines.ltx.pipeline_ltx_condition import LTXVideoCondition
  from diffusers.utils import export_to_video, load_video

  pipeline = LTXConditionPipeline.from_pretrained("Lightricks/LTX-Video-0.9.7-dev", dtype=torch.bfloat16)
  pipeline_upsample = LTXLatentUpsamplePipeline.from_pretrained("Lightricks/ltxv-spatial-upscaler-0.9.7", vae=pipeline.vae, dtype=torch.bfloat16)
  pipeline.to("cuda")
  pipe_upsample.to("cuda")
  pipeline.vae.enable_tiling()

  def round_to_nearest_resolution_acceptable_by_vae(height, width):
      height = height - (height % pipeline.vae_temporal_compression_ratio)
      width = width - (width % pipeline.vae_temporal_compression_ratio)
      return height, width

  video = load_video(
      "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/cosmos/cosmos-video2world-input-vid.mp4"
  )[:21]  # only use the first 21 frames as conditioning
  condition1 = LTXVideoCondition(video=video, frame_index=0)

  prompt = """
  The video depicts a winding mountain road covered in snow, with a single vehicle 
  traveling along it. The road is flanked by steep, rocky cliffs and sparse vegetation. 
  The landscape is characterized by rugged terrain and a river visible in the distance. 
  The scene captures the solitude and beauty of a winter drive through a mountainous region.
  """
  negative_prompt = "worst quality, inconsistent motion, blurry, jittery, distorted"
  expected_height, expected_width = 768, 1152
  downscale_factor = 2 / 3
  num_frames = 161

  # 1. Generate video at smaller resolution
  # Text-only conditioning is also supported without the need to pass `conditions`
  downscaled_height, downscaled_width = int(expected_height * downscale_factor), int(expected_width * downscale_factor)
  downscaled_height, downscaled_width = round_to_nearest_resolution_acceptable_by_vae(downscaled_height, downscaled_width)
  latents = pipeline(
      conditions=[condition1],
      prompt=prompt,
      negative_prompt=negative_prompt,
      width=downscaled_width,
      height=downscaled_height,
      num_frames=num_frames,
      num_inference_steps=30,
      decode_timestep=0.05,
      decode_noise_scale=0.025,
      image_cond_noise_scale=0.0,
      guidance_scale=5.0,
      guidance_rescale=0.7,
      generator=torch.Generator().manual_seed(0),
      output_type="latent",
  ).frames

  # 2. Upscale generated video using latent upsampler with fewer inference steps
  # The available latent upsampler upscales the height/width by 2x
  upscaled_height, upscaled_width = downscaled_height * 2, downscaled_width * 2
  upscaled_latents = pipe_upsample(
      latents=latents,
      output_type="latent"
  ).frames

  # 3. Denoise the upscaled video with few steps to improve texture (optional, but recommended)
  video = pipeline(
      conditions=[condition1],
      prompt=prompt,
      negative_prompt=negative_prompt,
      width=upscaled_width,
      height=upscaled_height,
      num_frames=num_frames,
      denoise_strength=0.4,  # Effectively, 4 inference steps out of 10
      num_inference_steps=10,
      latents=upscaled_latents,
      decode_timestep=0.05,
      decode_noise_scale=0.025,
      image_cond_noise_scale=0.0,
      guidance_scale=5.0,
      guidance_rescale=0.7,
      generator=torch.Generator().manual_seed(0),
      output_type="pil",
  ).frames[0]

  # 4. Downscale the video to the expected resolution
  video = [frame.resize((expected_width, expected_height)) for frame in video]

  export_to_video(video, "output.mp4", fps=24)
  ```

  

- LTX-Video 0.9.7 distilled model is guidance and timestep-distilled to speedup generation. It requires `guidance_scale` to be set to `1.0` and `num_inference_steps` should be set between `4` and `10` for good generation quality. You should also use the following custom timesteps for the best results.

  - Base model inference to prepare for upscaling: `[1000, 993, 987, 981, 975, 909, 725, 0.03]`.
  - Upscaling: `[1000, 909, 725, 421, 0]`.

  
  Show example code

  ```py
  import torch
  from diffusers import LTXConditionPipeline, LTXLatentUpsamplePipeline
  from diffusers.pipelines.ltx.pipeline_ltx_condition import LTXVideoCondition
  from diffusers.utils import export_to_video, load_video

  pipeline = LTXConditionPipeline.from_pretrained("Lightricks/LTX-Video-0.9.7-distilled", dtype=torch.bfloat16)
  pipe_upsample = LTXLatentUpsamplePipeline.from_pretrained("Lightricks/ltxv-spatial-upscaler-0.9.7", vae=pipeline.vae, dtype=torch.bfloat16)
  pipeline.to("cuda")
  pipe_upsample.to("cuda")
  pipeline.vae.enable_tiling()

  def round_to_nearest_resolution_acceptable_by_vae(height, width):
      height = height - (height % pipeline.vae_spatial_compression_ratio)
      width = width - (width % pipeline.vae_spatial_compression_ratio)
      return height, width

  prompt = """
  artistic anatomical 3d render, utlra quality, human half full male body with transparent 
  skin revealing structure instead of organs, muscular, intricate creative patterns, 
  monochromatic with backlighting, lightning mesh, scientific concept art, blending biology 
  with botany, surreal and ethereal quality, unreal engine 5, ray tracing, ultra realistic, 
  16K UHD, rich details. camera zooms out in a rotating fashion
  """
  negative_prompt = "worst quality, inconsistent motion, blurry, jittery, distorted"
  expected_height, expected_width = 768, 1152
  downscale_factor = 2 / 3
  num_frames = 161

  # 1. Generate video at smaller resolution
  downscaled_height, downscaled_width = int(expected_height * downscale_factor), int(expected_width * downscale_factor)
  downscaled_height, downscaled_width = round_to_nearest_resolution_acceptable_by_vae(downscaled_height, downscaled_width)
  latents = pipeline(
      prompt=prompt,
      negative_prompt=negative_prompt,
      width=downscaled_width,
      height=downscaled_height,
      num_frames=num_frames,
      timesteps=[1000, 993, 987, 981, 975, 909, 725, 0.03],
      decode_timestep=0.05,
      decode_noise_scale=0.025,
      image_cond_noise_scale=0.0,
      guidance_scale=1.0,
      guidance_rescale=0.7,
      generator=torch.Generator().manual_seed(0),
      output_type="latent",
  ).frames

  # 2. Upscale generated video using latent upsampler with fewer inference steps
  # The available latent upsampler upscales the height/width by 2x
  upscaled_height, upscaled_width = downscaled_height * 2, downscaled_width * 2
  upscaled_latents = pipe_upsample(
      latents=latents,
      adain_factor=1.0,
      output_type="latent"
  ).frames

  # 3. Denoise the upscaled video with few steps to improve texture (optional, but recommended)
  video = pipeline(
      prompt=prompt,
      negative_prompt=negative_prompt,
      width=upscaled_width,
      height=upscaled_height,
      num_frames=num_frames,
      denoise_strength=0.999,  # Effectively, 4 inference steps out of 5
      timesteps=[1000, 909, 725, 421, 0],
      latents=upscaled_latents,
      decode_timestep=0.05,
      decode_noise_scale=0.025,
      image_cond_noise_scale=0.0,
      guidance_scale=1.0,
      guidance_rescale=0.7,
      generator=torch.Generator().manual_seed(0),
      output_type="pil",
  ).frames[0]

  # 4. Downscale the video to the expected resolution
  video = [frame.resize((expected_width, expected_height)) for frame in video]

  export_to_video(video, "output.mp4", fps=24)
  ```

  

- LTX-Video 0.9.8 distilled model is similar to the 0.9.7 variant. It is guidance and timestep-distilled, and similar inference code can be used as above. An improvement of this version is that it supports generating very long videos. Additionally, it supports using tone mapping to improve the quality of the generated video using the `tone_map_compression_ratio` parameter. The default value of `0.6` is recommended.

  
  Show example code

  ```python
  import torch
  from diffusers import LTXConditionPipeline, LTXLatentUpsamplePipeline
  from diffusers.pipelines.ltx.pipeline_ltx_condition import LTXVideoCondition
  from diffusers.pipelines.ltx.modeling_latent_upsampler import LTXLatentUpsamplerModel
  from diffusers.utils import export_to_video, load_video

  pipeline = LTXConditionPipeline.from_pretrained("Lightricks/LTX-Video-0.9.8-13B-distilled", dtype=torch.bfloat16)
  # TODO: Update the checkpoint here once updated in LTX org
  upsampler = LTXLatentUpsamplerModel.from_pretrained("a-r-r-o-w/LTX-0.9.8-Latent-Upsampler", dtype=torch.bfloat16)
  pipe_upsample = LTXLatentUpsamplePipeline(vae=pipeline.vae, latent_upsampler=upsampler).to(torch.bfloat16)
  pipeline.to("cuda")
  pipe_upsample.to("cuda")
  pipeline.vae.enable_tiling()

  def round_to_nearest_resolution_acceptable_by_vae(height, width):
      height = height - (height % pipeline.vae_spatial_compression_ratio)
      width = width - (width % pipeline.vae_spatial_compression_ratio)
      return height, width

  prompt = """The camera pans over a snow-covered mountain range, revealing a vast expanse of snow-capped peaks and valleys.The mountains are covered in a thick layer of snow, with some areas appearing almost white while others have a slightly darker, almost grayish hue. The peaks are jagged and irregular, with some rising sharply into the sky while others are more rounded. The valleys are deep and narrow, with steep slopes that are also covered in snow. The trees in the foreground are mostly bare, with only a few leaves remaining on their branches. The sky is overcast, with thick clouds obscuring the sun. The overall impression is one of peace and tranquility, with the snow-covered mountains standing as a testament to the power and beauty of nature."""
  # prompt = """A woman walks away from a white Jeep parked on a city street at night, then ascends a staircase and knocks on a door. The woman, wearing a dark jacket and jeans, walks away from the Jeep parked on the left side of the street, her back to the camera; she walks at a steady pace, her arms swinging slightly by her sides; the street is dimly lit, with streetlights casting pools of light on the wet pavement; a man in a dark jacket and jeans walks past the Jeep in the opposite direction; the camera follows the woman from behind as she walks up a set of stairs towards a building with a green door; she reaches the top of the stairs and turns left, continuing to walk towards the building; she reaches the door and knocks on it with her right hand; the camera remains stationary, focused on the doorway; the scene is captured in real-life footage."""
  negative_prompt = "bright colors, symbols, graffiti, watermarks, worst quality, inconsistent motion, blurry, jittery, distorted"
  expected_height, expected_width = 480, 832
  downscale_factor = 2 / 3
  # num_frames = 161
  num_frames = 361

  # 1. Generate video at smaller resolution
  downscaled_height, downscaled_width = int(expected_height * downscale_factor), int(expected_width * downscale_factor)
  downscaled_height, downscaled_width = round_to_nearest_resolution_acceptable_by_vae(downscaled_height, downscaled_width)
  latents = pipeline(
      prompt=prompt,
      negative_prompt=negative_prompt,
      width=downscaled_width,
      height=downscaled_height,
      num_frames=num_frames,
      timesteps=[1000, 993, 987, 981, 975, 909, 725, 0.03],
      decode_timestep=0.05,
      decode_noise_scale=0.025,
      image_cond_noise_scale=0.0,
      guidance_scale=1.0,
      guidance_rescale=0.7,
      generator=torch.Generator().manual_seed(0),
      output_type="latent",
  ).frames

  # 2. Upscale generated video using latent upsampler with fewer inference steps
  # The available latent upsampler upscales the height/width by 2x
  upscaled_height, upscaled_width = downscaled_height * 2, downscaled_width * 2
  upscaled_latents = pipe_upsample(
      latents=latents,
      adain_factor=1.0,
      tone_map_compression_ratio=0.6,
      output_type="latent"
  ).frames

  # 3. Denoise the upscaled video with few steps to improve texture (optional, but recommended)
  video = pipeline(
      prompt=prompt,
      negative_prompt=negative_prompt,
      width=upscaled_width,
      height=upscaled_height,
      num_frames=num_frames,
      denoise_strength=0.999,  # Effectively, 4 inference steps out of 5
      timesteps=[1000, 909, 725, 421, 0],
      latents=upscaled_latents,
      decode_timestep=0.05,
      decode_noise_scale=0.025,
      image_cond_noise_scale=0.0,
      guidance_scale=1.0,
      guidance_rescale=0.7,
      generator=torch.Generator().manual_seed(0),
      output_type="pil",
  ).frames[0]

  # 4. Downscale the video to the expected resolution
  video = [frame.resize((expected_width, expected_height)) for frame in video]

  export_to_video(video, "output.mp4", fps=24)
  ```

  

- LTX-Video supports LoRAs with [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.LTXVideoLoraLoaderMixin.load_lora_weights).

  
  Show example code

  ```py
  import torch
  from diffusers import LTXConditionPipeline
  from diffusers.utils import export_to_video, load_image

  pipeline = LTXConditionPipeline.from_pretrained(
      "Lightricks/LTX-Video-0.9.5", dtype=torch.bfloat16
  )

  pipeline.load_lora_weights("Lightricks/LTX-Video-Cakeify-LoRA", adapter_name="cakeify")
  pipeline.set_adapters("cakeify")

  # use "CAKEIFY" to trigger the LoRA
  prompt = "CAKEIFY a person using a knife to cut a cake shaped like a Pikachu plushie"
  image = load_image("https://huggingface.co/Lightricks/LTX-Video-Cakeify-LoRA/resolve/main/assets/images/pikachu.png")

  video = pipeline(
      prompt=prompt,
      image=image,
      width=576,
      height=576,
      num_frames=161,
      decode_timestep=0.03,
      decode_noise_scale=0.025,
      num_inference_steps=50,
  ).frames[0]
  export_to_video(video, "output.mp4", fps=26)
  ```

  

- LTX-Video supports loading from single files, such as [GGUF checkpoints](../../quantization/gguf), with [loaders.FromOriginalModelMixin.from_single_file()](/docs/diffusers/v0.40.0/en/api/loaders/single_file#diffusers.loaders.FromOriginalModelMixin.from_single_file) or [loaders.FromSingleFileMixin.from_single_file()](/docs/diffusers/v0.40.0/en/api/loaders/single_file#diffusers.loaders.FromSingleFileMixin.from_single_file).

  
  Show example code

  ```py
  import torch
  from diffusers.utils import export_to_video
  from diffusers import LTXPipeline, AutoModel, GGUFQuantizationConfig

  transformer = AutoModel.from_single_file(
      "https://huggingface.co/city96/LTX-Video-gguf/blob/main/ltx-video-2b-v0.9-Q3_K_S.gguf"
      quantization_config=GGUFQuantizationConfig(compute_dtype=torch.bfloat16),
      dtype=torch.bfloat16
  )
  pipeline = LTXPipeline.from_pretrained(
      "Lightricks/LTX-Video",
      transformer=transformer,
      dtype=torch.bfloat16
  )
  ```

  

## LTXI2VLongMultiPromptPipeline[[diffusers.LTXI2VLongMultiPromptPipeline]]

#### diffusers.LTXI2VLongMultiPromptPipeline[[diffusers.LTXI2VLongMultiPromptPipeline]]

```python
diffusers.LTXI2VLongMultiPromptPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKLLTXVideo, text_encoder: T5EncoderModel, tokenizer: T5TokenizerFast, transformer: LTXVideoTransformer3DModel)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx/pipeline_ltx_i2v_long_multi_prompt.py#L389)

**Parameters:**

transformer ([LTXVideoTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/ltx_video_transformer3d#diffusers.LTXVideoTransformer3DModel)) : Conditional Transformer architecture to denoise the encoded video latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler) or `LTXEulerAncestralRFScheduler`) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLLTXVideo](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl_ltx_video#diffusers.AutoencoderKLLTXVideo)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`T5TokenizerFast`) : Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

Long-duration I2V (image-to-video) multi-prompt pipeline with ComfyUI parity.

Key features:
- Temporal sliding-window sampling only (no spatial H/W sharding); autoregressive fusion across windows.
- Multi-prompt segmentation per window with smooth transitions at window heads.
- First-frame hard conditioning via per-token mask for I2V.
- VRAM control via temporal windowing and VAE tiled decoding.

Reference: https://github.com/Lightricks/LTX-Video

#### __call__[[diffusers.LTXI2VLongMultiPromptPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, negative_prompt: str | list[str] | None = None, prompt_segments: list[dict[str, Any]] | None = None, height: int = 512, width: int = 704, num_frames: int = 161, frame_rate: float = 25, guidance_scale: float = 1.0, guidance_rescale: float = 0.0, num_inference_steps: int | None = 8, sigmas: list[float, torch.Tensor] | None = None, generator: torch.Generator | list[torch.Generator] | None = None, seed: int | None = 0, cond_image: 'PIL.Image.Image' | torch.Tensor | None = None, cond_strength: float = 0.5, latents: torch.Tensor | None = None, temporal_tile_size: int = 80, temporal_overlap: int = 24, temporal_overlap_cond_strength: float = 0.5, adain_factor: float = 0.25, guidance_latents: torch.Tensor | None = None, guiding_strength: float = 1.0, negative_index_latents: torch.Tensor | None = None, negative_index_strength: float = 1.0, skip_steps_sigma_threshold: float | None = 1, decode_timestep: float | None = 0.05, decode_noise_scale: float | None = 0.025, decode_horizontal_tiles: int = 4, decode_vertical_tiles: int = 4, decode_overlap: int = 3, output_type: str | None = 'latent', return_dict: bool = True, attention_kwargs: dict[str, Any] | None = None, callback_on_step_end: Callable[[int, int], None] | None = None, callback_on_step_end_tensor_inputs: list[str] = ['latents'], max_sequence_length: int = 128)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx/pipeline_ltx_i2v_long_multi_prompt.py#L935)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : Positive text prompt(s) per window. If a single string contains '|', parts are split by bars.

negative_prompt (`str` or `list[str]`, *optional*) : Negative prompt(s) to suppress undesired content.

prompt_segments (`list[dict]`, *optional*) : Segment mapping with {"start_window", "end_window", "text"} to override prompts per window.

height (`int`, defaults to `512`) : Output image height in pixels; must be divisible by 32.

width (`int`, defaults to `704`) : Output image width in pixels; must be divisible by 32.

num_frames (`int`, defaults to `161`) : Number of output frames (in decoded pixel space).

frame_rate (`float`, defaults to `25`) : Frames-per-second; used to normalize temporal coordinates in `video_coords`.

guidance_scale (`float`, defaults to `1.0`) : CFG scale; values > 1 enable classifier-free guidance.

guidance_rescale (`float`, defaults to `0.0`) : Optional rescale to mitigate overexposure under CFG (see `rescale_noise_cfg`).

num_inference_steps (`int`, *optional*, defaults to `8`) : Denoising steps per window. Ignored if `sigmas` is provided.

sigmas (`list[float]` or `torch.Tensor`, *optional*) : Explicit sigma schedule per window; if set, overrides `num_inference_steps`.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : Controls stochasticity; list accepted but first element is used (batch=1).

seed (`int`, *optional*, defaults to `0`) : If provided, seeds the shared generator for global latents and derives a window-local generator with `seed + w_start` per temporal window.

cond_image (`PIL.Image.Image` or `torch.Tensor`, *optional*) : Conditioning image; fixes frame 0 via per-token mask when `cond_strength > 0`.

cond_strength (`float`, defaults to `0.5`) : Strength of first-frame hard conditioning (smaller cond_mask ⇒ stronger preservation).

latents (`torch.Tensor`, *optional*) : Initial latents [B, C_lat, F_lat, H_lat, W_lat]; if None, sampled with `randn_tensor`.

temporal_tile_size (`int`, defaults to `80`) : Temporal window size (in decoded frames); internally scaled by VAE temporal compression.

temporal_overlap (`int`, defaults to `24`) : Overlap between consecutive windows (in decoded frames); internally scaled by compression.

temporal_overlap_cond_strength (`float`, defaults to `0.5`) : Strength for injecting previous window tail latents at new window head.

adain_factor (`float`, defaults to `0.25`) : AdaIN normalization strength for cross-window consistency (0 disables).

guidance_latents (`torch.Tensor`, *optional*) : Reference latents injected at window head; length trimmed by overlap for subsequent windows.

guiding_strength (`float`, defaults to `1.0`) : Injection strength for `guidance_latents`.

negative_index_latents (`torch.Tensor`, *optional*) : A single-frame latent appended at window head for "negative index" semantics.

negative_index_strength (`float`, defaults to `1.0`) : Injection strength for `negative_index_latents`.

skip_steps_sigma_threshold (`float`, *optional*, defaults to `1`) : Skip steps whose sigma exceeds this threshold.

decode_timestep (`float`, *optional*, defaults to `0.05`) : Decode-time timestep (if VAE supports timestep_conditioning).

decode_noise_scale (`float`, *optional*, defaults to `0.025`) : Decode-time noise mix scale (if VAE supports timestep_conditioning).

decode_horizontal_tiles (`int`, defaults to `4`) : Number of horizontal tiles during VAE decoding.

decode_vertical_tiles (`int`, defaults to `4`) : Number of vertical tiles during VAE decoding.

decode_overlap (`int`, defaults to `3`) : Overlap (in latent pixels) between tiles during VAE decoding.

output_type (`str`, *optional*, defaults to `"latent"`) : The output format of the generated video. Choose between "latent", "pt", "np", or "pil". If "latent", returns latents without decoding.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.ltx.LTXPipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : Extra attention parameters forwarded to the transformer.

callback_on_step_end (`PipelineCallback` or `MultiPipelineCallbacks`, *optional*) : Per-step callback hook.

callback_on_step_end_tensor_inputs (`list[str]`, defaults to `["latents"]`) : Keys from locals() to pass into the callback.

max_sequence_length (`int`, defaults to `128`) : Tokenizer max length for prompt encoding.

**Returns:** `~pipelines.ltx.LTXPipelineOutput` or `tuple`

If `return_dict` is `True`, `~pipelines.ltx.LTXPipelineOutput` is returned, otherwise a `tuple` is
returned where the first element is a list with the generated frames. The output format depends on
`output_type`:
- "latent"/"pt": `torch.Tensor` [B, C, F, H, W]; "latent" is in normalized latent space, "pt" is VAE
  output space.
- "np": `np.ndarray` post-processed.
- "pil": `list[PIL.Image.Image]` list of PIL images.

Generate an image-to-video sequence via temporal sliding windows and multi-prompt scheduling.

Examples:
```py
>>> import torch
>>> from diffusers import LTXEulerAncestralRFScheduler, LTXI2VLongMultiPromptPipeline

>>> pipe = LTXI2VLongMultiPromptPipeline.from_pretrained("LTX-Video-0.9.8-13B-distilled")
>>> # For ComfyUI parity, swap in the RF scheduler (keeps the original config).
>>> pipe.scheduler = LTXEulerAncestralRFScheduler.from_config(pipe.scheduler.config)
>>> pipe = pipe.to("cuda").to(dtype=torch.bfloat16)
>>> # Example A: get decoded frames (PIL)
>>> out = pipe(
...     prompt="a chimpanzee walks | a chimpanzee eats",
...     num_frames=161,
...     height=512,
...     width=704,
...     temporal_tile_size=80,
...     temporal_overlap=24,
...     output_type="pil",
...     return_dict=True,
... )
>>> frames = out.frames[0]  # list of PIL.Image.Image
>>> # Example B: get latent video and decode later (saves VRAM during sampling)
>>> out_latent = pipe(prompt="a chimpanzee walking", output_type="latent", return_dict=True).frames
>>> frames = pipe.vae_decode_tiled(out_latent, output_type="pil")[0]
```

Shapes:
Latent sizes (when auto-generated):
- F_lat = (num_frames - 1) // vae_temporal_compression_ratio + 1
- H_lat = height // vae_spatial_compression_ratio
- W_lat = width // vae_spatial_compression_ratio

Notes:
- Seeding: when `seed` is provided, each temporal window uses a local generator seeded with `seed +
  w_start`, while the shared generator is seeded once for global latents if no generator is passed;
  otherwise the passed-in generator is reused.
- CFG: unified `noise_pred = uncond + w * (text - uncond)` with optional `guidance_rescale`.
- Memory: denoising performs full-frame predictions (no spatial tiling); decoding can be tiled to avoid
  OOM.

#### encode_prompt[[diffusers.LTXI2VLongMultiPromptPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] | None = None, do_classifier_free_guidance: bool = True, num_videos_per_prompt: int = 1, prompt_embeds: torch.Tensor | None = None, negative_prompt_embeds: torch.Tensor | None = None, prompt_attention_mask: torch.Tensor | None = None, negative_prompt_attention_mask: torch.Tensor | None = None, max_sequence_length: int = 128, device: torch.device | None = None, dtype: torch.dtype | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx/pipeline_ltx_i2v_long_multi_prompt.py#L552)

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

#### prepare_latents[[diffusers.LTXI2VLongMultiPromptPipeline.prepare_latents]]

```python
prepare_latents(batch_size: int, num_channels_latents: int, height: int, width: int, num_frames: int, device: torch.device, generator: torch.Generator | None, dtype: torch.dtype = torch.float32, latents: torch.Tensor | None = None, cond_latents: torch.Tensor | None = None, cond_strength: float = 0.0, negative_index_latents: torch.Tensor | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx/pipeline_ltx_i2v_long_multi_prompt.py#L694)

**Returns:**

latents, negative_index_latents, latent_num_frames, latent_height, latent_width

Prepare base latents and optionally inject first-frame conditioning latents.

#### vae_decode_tiled[[diffusers.LTXI2VLongMultiPromptPipeline.vae_decode_tiled]]

```python
vae_decode_tiled(latents: torch.Tensor, decode_timestep: float | None = None, decode_noise_scale: float | None = None, horizontal_tiles: int = 4, vertical_tiles: int = 4, overlap: int = 3, last_frame_fix: bool = True, generator: torch.Generator | None = None, output_type: str = 'pt', auto_denormalize: bool = True, compute_dtype: torch.dtype = torch.float32, enable_vae_tiling: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx/pipeline_ltx_i2v_long_multi_prompt.py#L738)

**Parameters:**

latents : [B, C_latent, F_latent, H_latent, W_latent]

decode_timestep : Optional decode timestep (effective only if VAE supports timestep_conditioning)

decode_noise_scale : Optional decode noise interpolation (effective only if VAE supports timestep_conditioning)

horizontal_tiles, vertical_tiles : Number of tiles horizontally/vertically (>= 1)

overlap : Overlap in latent space (in latent pixels, >= 0)

last_frame_fix : Whether to enable the "repeat last frame" fix

generator : Random generator (used for decode_noise_scale noise)

output_type : "latent" | "pt" | "np" | "pil" - "latent": return latents unchanged (useful for downstream processing) - "pt": return tensor in VAE output space - "np"/"pil": post-processed outputs via VideoProcessor.postprocess_video

auto_denormalize : If True, apply LTX de-normalization to `latents` internally (recommended)

compute_dtype : Precision used during tile fusion (float32 default; significantly reduces seam blur)

enable_vae_tiling : If True, delegate tiling to VAE's built-in `tiled_decode` (sets `vae.use_tiling`).

**Returns:** `- If output_type="latent"`

returns input `latents` unchanged
- If output_type="pt": returns [B, C, F, H, W] (values roughly in [-1, 1])
- If output_type="np"/"pil": returns post-processed outputs via postprocess_video

VAE-based spatial tiled decoding (ComfyUI parity) implemented in Diffusers style.
- Linearly feather and blend overlapping tiles to avoid seams.
- Optional last_frame_fix: duplicate the last latent frame before decoding, then drop time_scale_factor frames
  at the end.
- Supports timestep_conditioning and decode_noise_scale injection.
- By default, "normalized latents" (the denoising output) are de-normalized internally (auto_denormalize=True).
- Tile fusion is computed in compute_dtype (float32 by default) to reduce blur and color shifts.

## LTXPipeline[[diffusers.LTXPipeline]]

#### diffusers.LTXPipeline[[diffusers.LTXPipeline]]

```python
diffusers.LTXPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKLLTXVideo, text_encoder: T5EncoderModel, tokenizer: T5Tokenizer, transformer: LTXVideoTransformer3DModel)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx/pipeline_ltx.py#L170)

**Parameters:**

transformer ([LTXVideoTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/ltx_video_transformer3d#diffusers.LTXVideoTransformer3DModel)) : Conditional Transformer architecture to denoise the encoded video latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLLTXVideo](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl_ltx_video#diffusers.AutoencoderKLLTXVideo)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer (`T5TokenizerFast`) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

Pipeline for text-to-video generation.

Reference: https://github.com/Lightricks/LTX-Video

#### __call__[[diffusers.LTXPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, negative_prompt: str | list[str] | None = None, height: int = 512, width: int = 704, num_frames: int = 161, frame_rate: int = 25, num_inference_steps: int = 50, timesteps: list = None, guidance_scale: float = 3, guidance_rescale: float = 0.0, num_videos_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, decode_timestep: float | list[float] = 0.0, decode_noise_scale: float | list[float] | None = None, output_type: str | None = 'pil', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 128)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx/pipeline_ltx.py#L535)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale < 1`).

height (`int`, defaults to `512`) : The height in pixels of the generated image. This is set to 480 by default for the best results.

width (`int`, defaults to `704`) : The width in pixels of the generated image. This is set to 848 by default for the best results.

num_frames (`int`, defaults to `161`) : The number of video frames to generate

frame_rate (`int`, defaults to `25`) : Target frame rate of the generated video.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

timesteps (`list[int]`, *optional*) : Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used. Must be in descending order.

guidance_scale (`float`, defaults to `3 `) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

guidance_rescale (`float`, *optional*, defaults to 0.0) : Guidance rescale factor proposed by [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891) `guidance_scale` is defined as `φ` in equation 16. of [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891). Guidance rescale factor should fix overexposure when using zero terminal SNR.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : The number of videos to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

prompt_attention_mask (`torch.Tensor`, *optional*) : Pre-generated attention mask for text embeddings.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. For PixArt-Sigma this negative prompt should be "". If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

negative_prompt_attention_mask (`torch.FloatTensor`, *optional*) : Pre-generated attention mask for negative text embeddings.

decode_timestep (`float`, defaults to `0.0`) : The timestep at which generated video is decoded.

decode_noise_scale (`float`, defaults to `None`) : The interpolation factor between random noise and denoised latents at the decode timestep.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.ltx.LTXPipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int` defaults to `128 `) : Maximum sequence length to use with the `prompt`.

**Returns:** `~pipelines.ltx.LTXPipelineOutput` or `tuple`

If `return_dict` is `True`, `~pipelines.ltx.LTXPipelineOutput` is returned, otherwise a `tuple` is
returned where the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import LTXPipeline
>>> from diffusers.utils import export_to_video

>>> pipe = LTXPipeline.from_pretrained("Lightricks/LTX-Video", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> prompt = "A woman with long brown hair and light skin smiles at another woman with long blonde hair. The woman with brown hair wears a black jacket and has a small, barely noticeable mole on her right cheek. The camera angle is a close-up, focused on the woman with brown hair's face. The lighting is warm and natural, likely from the setting sun, casting a soft glow on the scene. The scene appears to be real-life footage"
>>> negative_prompt = "worst quality, inconsistent motion, blurry, jittery, distorted"

>>> video = pipe(
...     prompt=prompt,
...     negative_prompt=negative_prompt,
...     width=704,
...     height=480,
...     num_frames=161,
...     num_inference_steps=50,
... ).frames[0]
>>> export_to_video(video, "output.mp4", fps=24)
```

#### encode_prompt[[diffusers.LTXPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] | None = None, do_classifier_free_guidance: bool = True, num_videos_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 128, device: typing.Optional[torch.device] = None, dtype: typing.Optional[torch.dtype] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx/pipeline_ltx.py#L283)

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

## LTXImageToVideoPipeline[[diffusers.LTXImageToVideoPipeline]]

#### diffusers.LTXImageToVideoPipeline[[diffusers.LTXImageToVideoPipeline]]

```python
diffusers.LTXImageToVideoPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKLLTXVideo, text_encoder: T5EncoderModel, tokenizer: T5Tokenizer, transformer: LTXVideoTransformer3DModel)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx/pipeline_ltx_image2video.py#L189)

**Parameters:**

transformer ([LTXVideoTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/ltx_video_transformer3d#diffusers.LTXVideoTransformer3DModel)) : Conditional Transformer architecture to denoise the encoded video latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLLTXVideo](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl_ltx_video#diffusers.AutoencoderKLLTXVideo)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer (`T5TokenizerFast`) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

Pipeline for image-to-video generation.

Reference: https://github.com/Lightricks/LTX-Video

#### __call__[[diffusers.LTXImageToVideoPipeline.__call__]]

```python
__call__(image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, prompt: str | list[str] = None, negative_prompt: str | list[str] | None = None, height: int = 512, width: int = 704, num_frames: int = 161, frame_rate: int = 25, num_inference_steps: int = 50, timesteps: list = None, guidance_scale: float = 3, guidance_rescale: float = 0.0, num_videos_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, decode_timestep: float | list[float] = 0.0, decode_noise_scale: float | list[float] | None = None, output_type: str | None = 'pil', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 128)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx/pipeline_ltx_image2video.py#L596)

**Parameters:**

image (`PipelineImageInput`) : The input image to condition the generation on. Must be an image, a list of images or a `torch.Tensor`.

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale < 1`).

height (`int`, defaults to `512`) : The height in pixels of the generated image. This is set to 480 by default for the best results.

width (`int`, defaults to `704`) : The width in pixels of the generated image. This is set to 848 by default for the best results.

num_frames (`int`, defaults to `161`) : The number of video frames to generate

frame_rate (`int`, defaults to `25`) : Target frame rate of the generated video.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

timesteps (`list[int]`, *optional*) : Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used. Must be in descending order.

guidance_scale (`float`, defaults to `3 `) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

guidance_rescale (`float`, *optional*, defaults to 0.0) : Guidance rescale factor proposed by [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891) `guidance_scale` is defined as `φ` in equation 16. of [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891). Guidance rescale factor should fix overexposure when using zero terminal SNR.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : The number of videos to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

prompt_attention_mask (`torch.Tensor`, *optional*) : Pre-generated attention mask for text embeddings.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. For PixArt-Sigma this negative prompt should be "". If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

negative_prompt_attention_mask (`torch.FloatTensor`, *optional*) : Pre-generated attention mask for negative text embeddings.

decode_timestep (`float`, defaults to `0.0`) : The timestep at which generated video is decoded.

decode_noise_scale (`float`, defaults to `None`) : The interpolation factor between random noise and denoised latents at the decode timestep.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.ltx.LTXPipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int` defaults to `128 `) : Maximum sequence length to use with the `prompt`.

**Returns:** `~pipelines.ltx.LTXPipelineOutput` or `tuple`

If `return_dict` is `True`, `~pipelines.ltx.LTXPipelineOutput` is returned, otherwise a `tuple` is
returned where the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import LTXImageToVideoPipeline
>>> from diffusers.utils import export_to_video, load_image

>>> pipe = LTXImageToVideoPipeline.from_pretrained("Lightricks/LTX-Video", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> image = load_image(
...     "https://huggingface.co/datasets/a-r-r-o-w/tiny-meme-dataset-captioned/resolve/main/images/8.png"
... )
>>> prompt = "A young girl stands calmly in the foreground, looking directly at the camera, as a house fire rages in the background. Flames engulf the structure, with smoke billowing into the air. Firefighters in protective gear rush to the scene, a fire truck labeled '38' visible behind them. The girl's neutral expression contrasts sharply with the chaos of the fire, creating a poignant and emotionally charged scene."
>>> negative_prompt = "worst quality, inconsistent motion, blurry, jittery, distorted"

>>> video = pipe(
...     image=image,
...     prompt=prompt,
...     negative_prompt=negative_prompt,
...     width=704,
...     height=480,
...     num_frames=161,
...     num_inference_steps=50,
... ).frames[0]
>>> export_to_video(video, "output.mp4", fps=24)
```

#### encode_prompt[[diffusers.LTXImageToVideoPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] | None = None, do_classifier_free_guidance: bool = True, num_videos_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 128, device: typing.Optional[torch.device] = None, dtype: typing.Optional[torch.dtype] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx/pipeline_ltx_image2video.py#L306)

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

## LTXConditionPipeline[[diffusers.LTXConditionPipeline]]

#### diffusers.LTXConditionPipeline[[diffusers.LTXConditionPipeline]]

```python
diffusers.LTXConditionPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKLLTXVideo, text_encoder: T5EncoderModel, tokenizer: T5Tokenizer, transformer: LTXVideoTransformer3DModel)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx/pipeline_ltx_condition.py#L252)

**Parameters:**

transformer ([LTXVideoTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/ltx_video_transformer3d#diffusers.LTXVideoTransformer3DModel)) : Conditional Transformer architecture to denoise the encoded video latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLLTXVideo](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl_ltx_video#diffusers.AutoencoderKLLTXVideo)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer (`T5TokenizerFast`) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

Pipeline for text/image/video-to-video generation.

Reference: https://github.com/Lightricks/LTX-Video

#### __call__[[diffusers.LTXConditionPipeline.__call__]]

```python
__call__(conditions: diffusers.pipelines.ltx.pipeline_ltx_condition.LTXVideoCondition | list[diffusers.pipelines.ltx.pipeline_ltx_condition.LTXVideoCondition] = None, image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], list[typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]]] = None, video: list = None, frame_index: int | list[int] = 0, strength: float | list[float] = 1.0, denoise_strength: float = 1.0, prompt: str | list[str] = None, negative_prompt: str | list[str] | None = None, height: int = 512, width: int = 704, num_frames: int = 161, frame_rate: int = 25, num_inference_steps: int = 50, timesteps: list = None, guidance_scale: float = 3, guidance_rescale: float = 0.0, image_cond_noise_scale: float = 0.15, num_videos_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, decode_timestep: float | list[float] = 0.0, decode_noise_scale: float | list[float] | None = None, output_type: str | None = 'pil', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 256)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx/pipeline_ltx_condition.py#L848)

**Parameters:**

conditions (`list[LTXVideoCondition], *optional*`) : The list of frame-conditioning items for the video generation.If not provided, conditions will be created using `image`, `video`, `frame_index` and `strength`.

image (`PipelineImageInput` or `list[PipelineImageInput]`, *optional*) : The image or images to condition the video generation. If not provided, one has to pass `video` or `conditions`.

video (`list[PipelineImageInput]`, *optional*) : The video to condition the video generation. If not provided, one has to pass `image` or `conditions`.

frame_index (`int` or `list[int]`, *optional*) : The frame index or frame indices at which the image or video will conditionally effect the video generation. If not provided, one has to pass `conditions`.

strength (`float` or `list[float]`, *optional*) : The strength or strengths of the conditioning effect. If not provided, one has to pass `conditions`.

denoise_strength (`float`, defaults to `1.0`) : The strength of the noise added to the latents for editing. Higher strength leads to more noise added to the latents, therefore leading to more differences between original video and generated video. This is useful for video-to-video editing.

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale < 1`).

height (`int`, defaults to `512`) : The height in pixels of the generated image. This is set to 480 by default for the best results.

width (`int`, defaults to `704`) : The width in pixels of the generated image. This is set to 848 by default for the best results.

num_frames (`int`, defaults to `161`) : The number of video frames to generate

frame_rate (`int`, defaults to `25`) : Target frame rate of the generated video.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

timesteps (`list[int]`, *optional*) : Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used. Must be in descending order.

guidance_scale (`float`, defaults to `3 `) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

guidance_rescale (`float`, *optional*, defaults to 0.0) : Guidance rescale factor proposed by [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891) `guidance_scale` is defined as `φ` in equation 16. of [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891). Guidance rescale factor should fix overexposure when using zero terminal SNR.

image_cond_noise_scale (`float`, defaults to `0.15`) : Scale of noise added to the conditioning image latents.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : The number of videos to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

prompt_attention_mask (`torch.Tensor`, *optional*) : Pre-generated attention mask for text embeddings.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. For PixArt-Sigma this negative prompt should be "". If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

negative_prompt_attention_mask (`torch.FloatTensor`, *optional*) : Pre-generated attention mask for negative text embeddings.

decode_timestep (`float`, defaults to `0.0`) : The timestep at which generated video is decoded.

decode_noise_scale (`float`, defaults to `None`) : The interpolation factor between random noise and denoised latents at the decode timestep.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.ltx.LTXPipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int` defaults to `128 `) : Maximum sequence length to use with the `prompt`.

**Returns:** `~pipelines.ltx.LTXPipelineOutput` or `tuple`

If `return_dict` is `True`, `~pipelines.ltx.LTXPipelineOutput` is returned, otherwise a `tuple` is
returned where the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers.pipelines.ltx.pipeline_ltx_condition import LTXConditionPipeline, LTXVideoCondition
>>> from diffusers.utils import export_to_video, load_video, load_image

>>> pipe = LTXConditionPipeline.from_pretrained("Lightricks/LTX-Video-0.9.5", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> # Load input image and video
>>> video = load_video(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/cosmos/cosmos-video2world-input-vid.mp4"
... )
>>> image = load_image(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/cosmos/cosmos-video2world-input.jpg"
... )

>>> # Create conditioning objects
>>> condition1 = LTXVideoCondition(
...     image=image,
...     frame_index=0,
... )
>>> condition2 = LTXVideoCondition(
...     video=video,
...     frame_index=80,
... )

>>> prompt = "The video depicts a long, straight highway stretching into the distance, flanked by metal guardrails. The road is divided into multiple lanes, with a few vehicles visible in the far distance. The surrounding landscape features dry, grassy fields on one side and rolling hills on the other. The sky is mostly clear with a few scattered clouds, suggesting a bright, sunny day. And then the camera switch to a winding mountain road covered in snow, with a single vehicle traveling along it. The road is flanked by steep, rocky cliffs and sparse vegetation. The landscape is characterized by rugged terrain and a river visible in the distance. The scene captures the solitude and beauty of a winter drive through a mountainous region."
>>> negative_prompt = "worst quality, inconsistent motion, blurry, jittery, distorted"

>>> # Generate video
>>> generator = torch.Generator("cuda").manual_seed(0)
>>> # Text-only conditioning is also supported without the need to pass `conditions`
>>> video = pipe(
...     conditions=[condition1, condition2],
...     prompt=prompt,
...     negative_prompt=negative_prompt,
...     width=768,
...     height=512,
...     num_frames=161,
...     num_inference_steps=40,
...     generator=generator,
... ).frames[0]

>>> export_to_video(video, "output.mp4", fps=24)
```

#### add_noise_to_image_conditioning_latents[[diffusers.LTXConditionPipeline.add_noise_to_image_conditioning_latents]]

```python
add_noise_to_image_conditioning_latents(t: float, init_latents: Tensor, latents: Tensor, noise_scale: float, conditioning_mask: Tensor, generator, eps = 1e-06)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx/pipeline_ltx_condition.py#L646)

Add timestep-dependent noise to the hard-conditioning latents. This helps with motion continuity, especially
when conditioned on a single frame.

#### encode_prompt[[diffusers.LTXConditionPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] | None = None, do_classifier_free_guidance: bool = True, num_videos_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 256, device: typing.Optional[torch.device] = None, dtype: typing.Optional[torch.dtype] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx/pipeline_ltx_condition.py#L369)

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

#### trim_conditioning_sequence[[diffusers.LTXConditionPipeline.trim_conditioning_sequence]]

```python
trim_conditioning_sequence(start_frame: int, sequence_num_frames: int, target_num_frames: int)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx/pipeline_ltx_condition.py#L629)

**Parameters:**

start_frame (int) : The target frame number of the first frame in the sequence.

sequence_num_frames (int) : The number of frames in the sequence.

target_num_frames (int) : The target number of frames in the generated video.

**Returns:** `int`

updated sequence length

Trim a conditioning sequence to the allowed number of frames.

## LTXLatentUpsamplePipeline[[diffusers.LTXLatentUpsamplePipeline]]

#### diffusers.LTXLatentUpsamplePipeline[[diffusers.LTXLatentUpsamplePipeline]]

```python
diffusers.LTXLatentUpsamplePipeline(vae: AutoencoderKLLTXVideo, latent_upsampler: LTXLatentUpsamplerModel)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx/pipeline_ltx_latent_upsample.py#L44)

#### __call__[[diffusers.LTXLatentUpsamplePipeline.__call__]]

```python
__call__(video: list[typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]] | None = None, height: int = 512, width: int = 704, latents: typing.Optional[torch.Tensor] = None, decode_timestep: float | list[float] = 0.0, decode_noise_scale: float | list[float] | None = None, adain_factor: float = 0.0, tone_map_compression_ratio: float = 0.0, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, output_type: str | None = 'pil', return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx/pipeline_ltx_latent_upsample.py#L188)

**Parameters:**

video (`list[PipelineImageInput]`, *optional*) : The input video frames to upsample. Mutually exclusive with `latents`.

height (`int`, defaults to `512`) : The height in pixels of the upsampled output.

width (`int`, defaults to `704`) : The width in pixels of the upsampled output.

latents (`torch.Tensor`, *optional*) : Pre-encoded video latents to upsample. Mutually exclusive with `video`.

decode_timestep (`float` or `list[float]`, defaults to `0.0`) : The timestep at which the upsampled latents are decoded.

decode_noise_scale (`float` or `list[float]`, *optional*) : Interpolation factor between random noise and denoised latents at the decode timestep.

adain_factor (`float`, defaults to `0.0`) : Strength of AdaIN statistical matching applied to the upsampled latents.

tone_map_compression_ratio (`float`, defaults to `0.0`) : Compression ratio used for tone mapping the upsampled latents. Must be in the range [0, 1].

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generated video. Choose between `PIL.Image`, `np.array`, or `latent`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.ltx.LTXPipelineOutput` instead of a plain tuple.

Function invoked when calling the pipeline for latent upsampling.

#### adain_filter_latent[[diffusers.LTXLatentUpsamplePipeline.adain_filter_latent]]

```python
adain_filter_latent(latents: Tensor, reference_latents: Tensor, factor: float = 1.0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx/pipeline_ltx_latent_upsample.py#L94)

**Parameters:**

latent (`torch.Tensor`) : Input latents to normalize

reference_latents (`torch.Tensor`) : The reference latents providing style statistics.

factor (`float`) : Blending factor between original and transformed latent. Range: -10.0 to 10.0, Default: 1.0

**Returns:** `torch.Tensor`

The transformed latent tensor

Applies Adaptive Instance Normalization (AdaIN) to a latent tensor based on statistics from a reference latent
tensor.

#### tone_map_latents[[diffusers.LTXLatentUpsamplePipeline.tone_map_latents]]

```python
tone_map_latents(latents: Tensor, compression: float)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx/pipeline_ltx_latent_upsample.py#L122)

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

## LTXPipelineOutput[[diffusers.pipelines.ltx.pipeline_output.LTXPipelineOutput]]

#### diffusers.pipelines.ltx.pipeline_output.LTXPipelineOutput[[diffusers.pipelines.ltx.pipeline_output.LTXPipelineOutput]]

```python
diffusers.pipelines.ltx.pipeline_output.LTXPipelineOutput(frames: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/ltx/pipeline_output.py#L9)

**Parameters:**

frames (`torch.Tensor`, `np.ndarray`, or list[list[PIL.Image.Image]]) : list of video outputs - It can be a nested list of length `batch_size,` with each sub-list containing denoised PIL image sequences of length `num_frames.` It can also be a NumPy array or Torch tensor of shape `(batch_size, num_frames, channels, height, width)`.

Output class for LTX pipelines.
