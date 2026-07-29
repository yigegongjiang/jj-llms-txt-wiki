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

  
    
      
    
  

# Helios

[Helios: Real Real-Time Long Video Generation Model](https://huggingface.co/papers/2603.04379) from Peking University & ByteDance & etc, by Shenghai Yuan, Yuanyang Yin, Zongjian Li, Xinwei Huang, Xiao Yang, Li Yuan.

*  We introduce Helios, the first 14B video generation model that runs at 17 FPS on a single NVIDIA H100 GPU and supports minute-scale generation while matching a strong baseline in quality. We make breakthroughs along three key dimensions: (1) robustness to long-video drifting without commonly used anti-drift heuristics such as self-forcing, error banks, or keyframe sampling; (2) real-time generation without standard acceleration techniques such as KV-cache, causal masking, or sparse attention; and (3) training without parallelism or sharding frameworks, enabling image-diffusion-scale batch sizes while fitting up to four 14B models within 80 GB of GPU memory. Specifically, Helios is a 14B autoregressive diffusion model with a unified input representation that natively supports T2V, I2V, and V2V tasks. To mitigate drifting in long-video generation, we characterize its typical failure modes and propose simple yet effective training strategies that explicitly simulate drifting during training, while eliminating repetitive motion at its source. For efficiency, we heavily compress the historical and noisy context and reduce the number of sampling steps, yielding computational costs comparable to—or lower than—those of 1.3B video generative models. Moreover, we introduce infrastructure-level optimizations that accelerate both inference and training while reducing memory consumption. Extensive experiments demonstrate that Helios consistently outperforms prior methods on both short- and long-video generation. All the code and models are available at [this https URL](https://pku-yuangroup.github.io/Helios-Page).

The following Helios models are supported in Diffusers:

- [Helios-Base](https://huggingface.co/BestWishYsh/Helios-Base): Best Quality, with v-prediction, standard CFG and custom HeliosScheduler.
- [Helios-Mid](https://huggingface.co/BestWishYsh/Helios-Mid): Intermediate Weight, with v-prediction, CFG-Zero* and custom HeliosScheduler.
- [Helios-Distilled](https://huggingface.co/BestWishYsh/Helios-Distilled): Best Efficiency, with x0-prediction and custom HeliosDMDScheduler.

> [!TIP]
> Click on the Helios models in the right sidebar for more examples of video generation.

### Optimizing Memory and Inference Speed

The example below demonstrates how to generate a video from text optimized for memory or inference speed.

Refer to the [Reduce memory usage](../../optimization/memory) guide for more details about the various memory saving techniques.

The Helios model below requires ~6GB of VRAM.

```py
import torch
from diffusers import AutoModel, HeliosPipeline
from diffusers.hooks.group_offloading import apply_group_offloading
from diffusers.utils import export_to_video

vae = AutoModel.from_pretrained("BestWishYsh/Helios-Base", subfolder="vae", torch_dtype=torch.float32)

# group-offloading
pipeline = HeliosPipeline.from_pretrained(
    "BestWishYsh/Helios-Base",
    vae=vae,
    torch_dtype=torch.bfloat16
)
pipeline.enable_group_offload(
    onload_device=torch.device("cuda"),
    offload_device=torch.device("cpu"),
    offload_type="leaf_level",
    use_stream=True,
    record_stream=True,
)

prompt = """
A vibrant tropical fish swimming gracefully among colorful coral reefs in a clear, turquoise ocean. The fish has bright blue 
and yellow scales with a small, distinctive orange spot on its side, its fins moving fluidly. The coral reefs are alive with 
a variety of marine life, including small schools of colorful fish and sea turtles gliding by. The water is crystal clear, 
allowing for a view of the sandy ocean floor below. The reef itself is adorned with a mix of hard and soft corals in shades 
of red, orange, and green. The photo captures the fish from a slightly elevated angle, emphasizing its lively movements and 
the vivid colors of its surroundings. A close-up shot with dynamic movement.
"""
negative_prompt = """
Bright tones, overexposed, static, blurred details, subtitles, style, works, paintings, images, static, overall gray, worst quality,
low quality, JPEG compression residue, ugly, incomplete, extra fingers, poorly drawn hands, poorly drawn faces, deformed, disfigured,
misshapen limbs, fused fingers, still picture, messy background, three legs, many people in the background, walking backwards
"""

output = pipeline(
    prompt=prompt,
    negative_prompt=negative_prompt,
    num_frames=99,
    num_inference_steps=50,
    guidance_scale=5.0,
    generator=torch.Generator("cuda").manual_seed(42),
).frames[0]
export_to_video(output, "helios_base_t2v_output.mp4", fps=24)
```

[Compilation](../../optimization/fp16#torchcompile) is slow the first time but subsequent calls to the pipeline are faster. [Attention Backends](../../optimization/attention_backends) such as FlashAttention and SageAttention can significantly increase speed by optimizing the computation of the attention mechanism. [Context Parallelism](../../training/distributed_inference#context-parallelism) splits the input sequence across multiple devices to enable processing of long contexts in parallel, reducing memory pressure and latency. [Caching](../../optimization/cache) may also speed up inference by storing and reusing intermediate outputs.

```py
import torch
from diffusers import AutoModel, HeliosPipeline
from diffusers.utils import export_to_video

vae = AutoModel.from_pretrained("BestWishYsh/Helios-Base", subfolder="vae", torch_dtype=torch.float32)

pipeline = HeliosPipeline.from_pretrained(
    "BestWishYsh/Helios-Base",
    vae=vae,
    torch_dtype=torch.bfloat16
)
pipeline.to("cuda")

# attention backend
# pipeline.transformer.set_attention_backend("flash")
pipeline.transformer.set_attention_backend("_flash_3_hub") # For Hopper GPUs

# torch.compile
torch.backends.cudnn.benchmark = True
pipeline.text_encoder.compile(mode="max-autotune-no-cudagraphs", dynamic=False)
pipeline.vae.compile(mode="max-autotune-no-cudagraphs", dynamic=False)
pipeline.transformer.compile(mode="max-autotune-no-cudagraphs", dynamic=False)

prompt = """
A vibrant tropical fish swimming gracefully among colorful coral reefs in a clear, turquoise ocean. The fish has bright blue 
and yellow scales with a small, distinctive orange spot on its side, its fins moving fluidly. The coral reefs are alive with 
a variety of marine life, including small schools of colorful fish and sea turtles gliding by. The water is crystal clear, 
allowing for a view of the sandy ocean floor below. The reef itself is adorned with a mix of hard and soft corals in shades 
of red, orange, and green. The photo captures the fish from a slightly elevated angle, emphasizing its lively movements and 
the vivid colors of its surroundings. A close-up shot with dynamic movement.
"""
negative_prompt = """
Bright tones, overexposed, static, blurred details, subtitles, style, works, paintings, images, static, overall gray, worst quality,
low quality, JPEG compression residue, ugly, incomplete, extra fingers, poorly drawn hands, poorly drawn faces, deformed, disfigured,
misshapen limbs, fused fingers, still picture, messy background, three legs, many people in the background, walking backwards
"""

output = pipeline(
    prompt=prompt,
    negative_prompt=negative_prompt,
    num_frames=99,
    num_inference_steps=50,
    guidance_scale=5.0,
    generator=torch.Generator("cuda").manual_seed(42),
).frames[0]
export_to_video(output, "helios_base_t2v_output.mp4", fps=24)
```

### Generation with Helios-Base

The example below demonstrates how to use Helios-Base to generate video based on text, image or video.

```python
import torch
from diffusers import AutoModel, HeliosPipeline
from diffusers.utils import export_to_video, load_video, load_image

vae = AutoModel.from_pretrained("BestWishYsh/Helios-Base", subfolder="vae", torch_dtype=torch.float32)

pipeline = HeliosPipeline.from_pretrained(
    "BestWishYsh/Helios-Base",
    vae=vae,
    torch_dtype=torch.bfloat16
)
pipeline.to("cuda")

negative_prompt = """
Bright tones, overexposed, static, blurred details, subtitles, style, works, paintings, images, static, overall gray, worst quality,
low quality, JPEG compression residue, ugly, incomplete, extra fingers, poorly drawn hands, poorly drawn faces, deformed, disfigured,
misshapen limbs, fused fingers, still picture, messy background, three legs, many people in the background, walking backwards
"""

# For Text-to-Video
prompt = """
A vibrant tropical fish swimming gracefully among colorful coral reefs in a clear, turquoise ocean. The fish has bright blue 
and yellow scales with a small, distinctive orange spot on its side, its fins moving fluidly. The coral reefs are alive with 
a variety of marine life, including small schools of colorful fish and sea turtles gliding by. The water is crystal clear, 
allowing for a view of the sandy ocean floor below. The reef itself is adorned with a mix of hard and soft corals in shades 
of red, orange, and green. The photo captures the fish from a slightly elevated angle, emphasizing its lively movements and 
the vivid colors of its surroundings. A close-up shot with dynamic movement.
"""

output = pipeline(
    prompt=prompt,
    negative_prompt=negative_prompt,
    num_frames=99,
    num_inference_steps=50,
    guidance_scale=5.0,
    generator=torch.Generator("cuda").manual_seed(42),
).frames[0]
export_to_video(output, "helios_base_t2v_output.mp4", fps=24)

# For Image-to-Video
prompt = """
A towering emerald wave surges forward, its crest curling with raw power and energy. Sunlight glints off the translucent water, 
illuminating the intricate textures and deep green hues within the wave’s body. A thick spray erupts from the breaking crest, 
casting a misty veil that dances above the churning surface. As the perspective widens, the immense scale of the wave becomes 
apparent, revealing the restless expanse of the ocean stretching beyond. The scene captures the ocean’s untamed beauty and 
relentless force, with every droplet and ripple shimmering in the light. The dynamic motion and vivid colors evoke both awe and 
respect for nature’s might.
"""
image_path = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/helios/wave.jpg"

output = pipeline(
    prompt=prompt,
    negative_prompt=negative_prompt,
    image=load_image(image_path).resize((640, 384)),
    num_frames=99,
    num_inference_steps=50,
    guidance_scale=5.0,
    generator=torch.Generator("cuda").manual_seed(42),
).frames[0]
export_to_video(output, "helios_base_i2v_output.mp4", fps=24)

# For Video-to-Video
prompt = """
A bright yellow Lamborghini Huracn Tecnica speeds along a curving mountain road, surrounded by lush green trees 
under a partly cloudy sky. The car's sleek design and vibrant color stand out against the natural backdrop, 
emphasizing its dynamic movement. The road curves gently, with a guardrail visible on one side, adding depth to 
the scene. The motion blur captures the sense of speed and energy, creating a thrilling and exhilarating atmosphere. 
A front-facing shot from a slightly elevated angle, highlighting the car's aggressive stance and the surrounding greenery.
"""
video_path = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/helios/car.mp4"

output = pipeline(
    prompt=prompt,
    negative_prompt=negative_prompt,
    video=load_video(video_path),
    num_frames=99,
    num_inference_steps=50,
    guidance_scale=5.0,
    generator=torch.Generator("cuda").manual_seed(42),
).frames[0]
export_to_video(output, "helios_base_v2v_output.mp4", fps=24)
```

### Generation with Helios-Mid

The example below demonstrates how to use Helios-Mid to generate video based on text, image or video.

```python
import torch
from diffusers import AutoModel, HeliosPyramidPipeline
from diffusers.utils import export_to_video, load_video, load_image

vae = AutoModel.from_pretrained("BestWishYsh/Helios-Mid", subfolder="vae", torch_dtype=torch.float32)

pipeline = HeliosPyramidPipeline.from_pretrained(
    "BestWishYsh/Helios-Mid",
    vae=vae,
    torch_dtype=torch.bfloat16
)
pipeline.to("cuda")

negative_prompt = """
Bright tones, overexposed, static, blurred details, subtitles, style, works, paintings, images, static, overall gray, worst quality,
low quality, JPEG compression residue, ugly, incomplete, extra fingers, poorly drawn hands, poorly drawn faces, deformed, disfigured,
misshapen limbs, fused fingers, still picture, messy background, three legs, many people in the background, walking backwards
"""

# For Text-to-Video
prompt = """
A vibrant tropical fish swimming gracefully among colorful coral reefs in a clear, turquoise ocean. The fish has bright blue 
and yellow scales with a small, distinctive orange spot on its side, its fins moving fluidly. The coral reefs are alive with 
a variety of marine life, including small schools of colorful fish and sea turtles gliding by. The water is crystal clear, 
allowing for a view of the sandy ocean floor below. The reef itself is adorned with a mix of hard and soft corals in shades 
of red, orange, and green. The photo captures the fish from a slightly elevated angle, emphasizing its lively movements and 
the vivid colors of its surroundings. A close-up shot with dynamic movement.
"""

output = pipeline(
    prompt=prompt,
    negative_prompt=negative_prompt,
    num_frames=99,
    pyramid_num_inference_steps_list=[20, 20, 20],
    guidance_scale=5.0,
    use_zero_init=True,
    zero_steps=1,
    generator=torch.Generator("cuda").manual_seed(42),
).frames[0]
export_to_video(output, "helios_pyramid_t2v_output.mp4", fps=24)

# For Image-to-Video
prompt = """
A towering emerald wave surges forward, its crest curling with raw power and energy. Sunlight glints off the translucent water, 
illuminating the intricate textures and deep green hues within the wave’s body. A thick spray erupts from the breaking crest, 
casting a misty veil that dances above the churning surface. As the perspective widens, the immense scale of the wave becomes 
apparent, revealing the restless expanse of the ocean stretching beyond. The scene captures the ocean’s untamed beauty and 
relentless force, with every droplet and ripple shimmering in the light. The dynamic motion and vivid colors evoke both awe and 
respect for nature’s might.
"""
image_path = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/helios/wave.jpg"

output = pipeline(
    prompt=prompt,
    negative_prompt=negative_prompt,
    image=load_image(image_path).resize((640, 384)),
    num_frames=99,
    pyramid_num_inference_steps_list=[20, 20, 20],
    guidance_scale=5.0,
    use_zero_init=True,
    zero_steps=1,
    generator=torch.Generator("cuda").manual_seed(42),
).frames[0]
export_to_video(output, "helios_pyramid_i2v_output.mp4", fps=24)

# For Video-to-Video
prompt = """
A bright yellow Lamborghini Huracn Tecnica speeds along a curving mountain road, surrounded by lush green trees 
under a partly cloudy sky. The car's sleek design and vibrant color stand out against the natural backdrop, 
emphasizing its dynamic movement. The road curves gently, with a guardrail visible on one side, adding depth to 
the scene. The motion blur captures the sense of speed and energy, creating a thrilling and exhilarating atmosphere. 
A front-facing shot from a slightly elevated angle, highlighting the car's aggressive stance and the surrounding greenery.
"""
video_path = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/helios/car.mp4"

output = pipeline(
    prompt=prompt,
    negative_prompt=negative_prompt,
    video=load_video(video_path),
    num_frames=99,
    pyramid_num_inference_steps_list=[20, 20, 20],
    guidance_scale=5.0,
    use_zero_init=True,
    zero_steps=1,
    generator=torch.Generator("cuda").manual_seed(42),
).frames[0]
export_to_video(output, "helios_pyramid_v2v_output.mp4", fps=24)
```

### Generation with Helios-Distilled

The example below demonstrates how to use Helios-Distilled to generate video based on text, image or video.

```python
import torch
from diffusers import AutoModel, HeliosPyramidPipeline
from diffusers.utils import export_to_video, load_video, load_image

vae = AutoModel.from_pretrained("BestWishYsh/Helios-Distilled", subfolder="vae", torch_dtype=torch.float32)

pipeline = HeliosPyramidPipeline.from_pretrained(
    "BestWishYsh/Helios-Distilled",
    vae=vae,
    torch_dtype=torch.bfloat16
)
pipeline.to("cuda")

negative_prompt = """
Bright tones, overexposed, static, blurred details, subtitles, style, works, paintings, images, static, overall gray, worst quality,
low quality, JPEG compression residue, ugly, incomplete, extra fingers, poorly drawn hands, poorly drawn faces, deformed, disfigured,
misshapen limbs, fused fingers, still picture, messy background, three legs, many people in the background, walking backwards
"""

# For Text-to-Video
prompt = """
A vibrant tropical fish swimming gracefully among colorful coral reefs in a clear, turquoise ocean. The fish has bright blue 
and yellow scales with a small, distinctive orange spot on its side, its fins moving fluidly. The coral reefs are alive with 
a variety of marine life, including small schools of colorful fish and sea turtles gliding by. The water is crystal clear, 
allowing for a view of the sandy ocean floor below. The reef itself is adorned with a mix of hard and soft corals in shades 
of red, orange, and green. The photo captures the fish from a slightly elevated angle, emphasizing its lively movements and 
the vivid colors of its surroundings. A close-up shot with dynamic movement.
"""

output = pipeline(
    prompt=prompt,
    negative_prompt=negative_prompt,
    num_frames=240,
    pyramid_num_inference_steps_list=[2, 2, 2],
    guidance_scale=1.0,
    is_amplify_first_chunk=True,
    generator=torch.Generator("cuda").manual_seed(42),
).frames[0]
export_to_video(output, "helios_distilled_t2v_output.mp4", fps=24)

# For Image-to-Video
prompt = """
A towering emerald wave surges forward, its crest curling with raw power and energy. Sunlight glints off the translucent water, 
illuminating the intricate textures and deep green hues within the wave’s body. A thick spray erupts from the breaking crest, 
casting a misty veil that dances above the churning surface. As the perspective widens, the immense scale of the wave becomes 
apparent, revealing the restless expanse of the ocean stretching beyond. The scene captures the ocean’s untamed beauty and 
relentless force, with every droplet and ripple shimmering in the light. The dynamic motion and vivid colors evoke both awe and 
respect for nature’s might.
"""
image_path = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/helios/wave.jpg"

output = pipeline(
    prompt=prompt,
    negative_prompt=negative_prompt,
    image=load_image(image_path).resize((640, 384)),
    num_frames=240,
    pyramid_num_inference_steps_list=[2, 2, 2],
    guidance_scale=1.0,
    is_amplify_first_chunk=True,
    generator=torch.Generator("cuda").manual_seed(42),
).frames[0]
export_to_video(output, "helios_distilled_i2v_output.mp4", fps=24)

# For Video-to-Video
prompt = """
A bright yellow Lamborghini Huracn Tecnica speeds along a curving mountain road, surrounded by lush green trees 
under a partly cloudy sky. The car's sleek design and vibrant color stand out against the natural backdrop, 
emphasizing its dynamic movement. The road curves gently, with a guardrail visible on one side, adding depth to 
the scene. The motion blur captures the sense of speed and energy, creating a thrilling and exhilarating atmosphere. 
A front-facing shot from a slightly elevated angle, highlighting the car's aggressive stance and the surrounding greenery.
"""
video_path = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/helios/car.mp4"

output = pipeline(
    prompt=prompt,
    negative_prompt=negative_prompt,
    video=load_video(video_path),
    num_frames=240,
    pyramid_num_inference_steps_list=[2, 2, 2],
    guidance_scale=1.0,
    is_amplify_first_chunk=True,
    generator=torch.Generator("cuda").manual_seed(42),
).frames[0]
export_to_video(output, "helios_distilled_v2v_output.mp4", fps=24)
```

## Text-to-Video Showcases

  
    Prompt
    Generated Video
  
  
    A Viking warrior driving a modern city bus filled with passengers. The Viking has long blonde hair tied back, a beard, and is adorned with a fur-lined helmet and armor. He wears a traditional tunic and trousers, but also sports a seatbelt as he focuses on navigating the busy streets. The interior of the bus is typical, with rows of seats occupied by diverse passengers going about their daily routines. The exterior shots show the bustling urban environment, including tall buildings and traffic. Medium shot focusing on the Viking at the wheel, with occasional close-ups of his determined expression.
    
    
      
        
      
    
  
  
    A documentary-style nature photography shot from a camera truck moving to the left, capturing a crab quickly scurrying into its burrow. The crab has a hard, greenish-brown shell and long claws, moving with determined speed across the sandy ground. Its body is slightly arched as it burrows into the sand, leaving a small trail behind. The background shows a shallow beach with scattered rocks and seashells, and the horizon features a gentle curve of the coastline. The photo has a natural and realistic texture, emphasizing the crab's natural movement and the texture of the sand. A close-up shot from a slightly elevated angle.
    
    
      
        
      
    
  

## Image-to-Video Showcases

  
    Image
    Prompt
    Generated Video
  
  
    
    A sleek red Kia car speeds along a rural road under a cloudy sky, its modern design and dynamic movement emphasized by the blurred motion of the surrounding fields and trees stretching into the distance. The car's glossy exterior reflects the overcast sky, highlighting its aerodynamic shape and sporty stance. The license plate reads "KIA 626," and the vehicle's headlights are on, adding to the sense of motion and energy. The road curves gently, with the car positioned slightly off-center, creating a sense of forward momentum. A dynamic front three-quarter view captures the car's powerful presence against the serene backdrop of rolling hills and scattered trees.
    
    
      
        
      
    
  
  
    
    A close-up captures a fluffy orange cat with striking green eyes and white whiskers, gazing intently towards the camera. The cat's fur is soft and well-groomed, with a mix of warm orange and cream tones. Its large, expressive eyes are a vivid green, reflecting curiosity and alertness. The cat's nose is small and pink, and its mouth is slightly open, revealing a hint of its pink tongue. The background is softly blurred, suggesting a cozy indoor setting with neutral tones. The photo has a shallow depth of field, focusing sharply on the cat's face while the background remains out of focus. A close-up shot from a slightly elevated perspective.
    
    
      
        
      
    
  

## Interactive-Video Showcases

  
    Prompt
    Generated Video
  
  
    The prompt can be found here
    
      
        
      
    
  
  
    The prompt can be found here
    
      
        
      
    
  

## Resources

Learn more about Helios with the following resources.
- Watch [video1](https://www.youtube.com/watch?v=vd_AgHtOUFQ) and [video2](https://www.youtube.com/watch?v=1GeIU2Dn7UY) for a demonstration of Helios's key features.
- The research paper, [Helios: Real Real-Time Long Video Generation Model](https://huggingface.co/papers/2603.04379) for more details.

## HeliosPipeline[[diffusers.HeliosPipeline]]

#### diffusers.HeliosPipeline[[diffusers.HeliosPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/helios/pipeline_helios.py#L108)

Pipeline for text-to-video / image-to-video / video-to-video generation using Helios.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

__call__diffusers.HeliosPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/helios/pipeline_helios.py#L445[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str | list[str] = None"}, {"name": "height", "val": ": int = 384"}, {"name": "width", "val": ": int = 640"}, {"name": "num_frames", "val": ": int = 132"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "sigmas", "val": ": list = None"}, {"name": "guidance_scale", "val": ": float = 5.0"}, {"name": "num_videos_per_prompt", "val": ": int | None = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'np'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}, {"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "image_latents", "val": ": torch.Tensor | None = None"}, {"name": "fake_image_latents", "val": ": torch.Tensor | None = None"}, {"name": "add_noise_to_image_latents", "val": ": bool = True"}, {"name": "image_noise_sigma_min", "val": ": float = 0.111"}, {"name": "image_noise_sigma_max", "val": ": float = 0.135"}, {"name": "video", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "video_latents", "val": ": torch.Tensor | None = None"}, {"name": "add_noise_to_video_latents", "val": ": bool = True"}, {"name": "video_noise_sigma_min", "val": ": float = 0.111"}, {"name": "video_noise_sigma_max", "val": ": float = 0.135"}, {"name": "history_sizes", "val": ": list = [16, 2, 1]"}, {"name": "num_latent_frames_per_chunk", "val": ": int = 9"}, {"name": "keep_first_frame", "val": ": bool = True"}, {"name": "is_skip_first_chunk", "val": ": bool = False"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, pass `prompt_embeds` instead.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to avoid during image generation. If not defined, pass `negative_prompt_embeds`
  instead. Ignored when not using guidance (`guidance_scale`  1`. Higher guidance scale encourages to generate images that are closely linked to
  the text `prompt`, usually at the expense of lower image quality.
- **num_videos_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make
  generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor is generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not
  provided, text embeddings are generated from the `prompt` input argument.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings. If not provided, they are generated from `negative_prompt`.
- **output_type** (`str`, *optional*, defaults to `"np"`) --
  The output format of the generated image. Choose between `PIL.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `HeliosPipelineOutput` instead of a plain tuple.
- **attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **callback_on_step_end** (`Callable`, `PipelineCallback`, `MultiPipelineCallbacks`, *optional*) --
  A function or a subclass of `PipelineCallback` or `MultiPipelineCallbacks` that is called at the end of
  each denoising step during the inference. with the following arguments: `callback_on_step_end(self:
  DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a
  list of all tensors as specified by `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`list`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.
- **max_sequence_length** (`int`, defaults to `512`) --
  The maximum sequence length of the text encoder. If the prompt is longer than this, it will be
  truncated. If the prompt is shorter, it will be padded to this length.
- **image** (`PipelineImageInput`, *optional*) --
  Input image used for image-to-video conditioning.
- **image_latents** (`torch.Tensor`, *optional*) --
  Pre-encoded image latents to use instead of `image`.
- **fake_image_latents** (`torch.Tensor`, *optional*) --
  Optional fake image latents used during conditioning.
- **add_noise_to_image_latents** (`bool`, *optional*, defaults to `True`) --
  Whether to add noise to the image latents prior to denoising.
- **image_noise_sigma_min** (`float`, *optional*, defaults to `0.111`) --
  Minimum sigma value for noise added to image latents.
- **image_noise_sigma_max** (`float`, *optional*, defaults to `0.135`) --
  Maximum sigma value for noise added to image latents.
- **video** (`PipelineImageInput`, *optional*) --
  Input video used for video-to-video conditioning.
- **video_latents** (`torch.Tensor`, *optional*) --
  Pre-encoded video latents to use instead of `video`.
- **add_noise_to_video_latents** (`bool`, *optional*, defaults to `True`) --
  Whether to add noise to the video latents prior to denoising.
- **video_noise_sigma_min** (`float`, *optional*, defaults to `0.111`) --
  Minimum sigma value for noise added to video latents.
- **video_noise_sigma_max** (`float`, *optional*, defaults to `0.135`) --
  Maximum sigma value for noise added to video latents.
- **history_sizes** (`list`, *optional*, defaults to `[16, 2, 1]`) --
  History window sizes used for autoregressive chunked generation.
- **num_latent_frames_per_chunk** (`int`, *optional*, defaults to `9`) --
  Number of latent frames produced per chunk during autoregressive generation.
- **keep_first_frame** (`bool`, *optional*, defaults to `True`) --
  Whether to retain the first frame across chunks.
- **is_skip_first_chunk** (`bool`, *optional*, defaults to `False`) --
  Whether to skip generation of the first chunk.0`~HeliosPipelineOutput` or `tuple`If `return_dict` is `True`, `HeliosPipelineOutput` is returned, otherwise a `tuple` is returned where
the first element is a list with the generated images and the second element is a list of `bool`s
indicating whether the corresponding generated image contains "not-safe-for-work" (nsfw) content.

The call function to the pipeline for generation.

Examples:
```python
>>> import torch
>>> from diffusers.utils import export_to_video
>>> from diffusers import AutoencoderKLWan, HeliosPipeline

>>> # Available models: BestWishYsh/Helios-Base, BestWishYsh/Helios-Mid, BestWishYsh/Helios-Distilled
>>> model_id = "BestWishYsh/Helios-Base"
>>> vae = AutoencoderKLWan.from_pretrained(model_id, subfolder="vae", torch_dtype=torch.float32)
>>> pipe = HeliosPipeline.from_pretrained(model_id, vae=vae, torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> prompt = "A cat and a dog baking a cake together in a kitchen. The cat is carefully measuring flour, while the dog is stirring the batter with a wooden spoon. The kitchen is cozy, with sunlight streaming through the window."
>>> negative_prompt = "Bright tones, overexposed, static, blurred details, subtitles, style, works, paintings, images, static, overall gray, worst quality, low quality, JPEG compression residue, ugly, incomplete, extra fingers, poorly drawn hands, poorly drawn faces, deformed, disfigured, misshapen limbs, fused fingers, still picture, messy background, three legs, many people in the background, walking backwards"

>>> output = pipe(
...     prompt=prompt,
...     negative_prompt=negative_prompt,
...     height=384,
...     width=640,
...     num_frames=132,
...     guidance_scale=5.0,
... ).frames[0]
>>> export_to_video(output, "output.mp4", fps=24)
```

**Parameters:**

tokenizer (`T5Tokenizer`) : Tokenizer from [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5Tokenizer), specifically the [google/umt5-xxl](https://huggingface.co/google/umt5-xxl) variant.

text_encoder (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/umt5-xxl](https://huggingface.co/google/umt5-xxl) variant.

transformer ([HeliosTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/helios_transformer3d#diffusers.HeliosTransformer3DModel)) : Conditional Transformer to denoise the input latents.

scheduler ([HeliosScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/helios#diffusers.HeliosScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLWan](/docs/diffusers/v0.39.0/en/api/models/autoencoder_kl_wan#diffusers.AutoencoderKLWan)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

**Returns:**

``~HeliosPipelineOutput` or `tuple``

If `return_dict` is `True`, `HeliosPipelineOutput` is returned, otherwise a `tuple` is returned where
the first element is a list with the generated images and the second element is a list of `bool`s
indicating whether the corresponding generated image contains "not-safe-for-work" (nsfw) content.
#### encode_prompt[[diffusers.HeliosPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/helios/pipeline_helios.py#L196)

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

## HeliosPyramidPipeline[[diffusers.HeliosPyramidPipeline]]

#### diffusers.HeliosPyramidPipeline[[diffusers.HeliosPyramidPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/helios/pipeline_helios_pyramid.py#L121)

Pipeline for text-to-video / image-to-video / video-to-video generation using Helios.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

__call__diffusers.HeliosPyramidPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/helios/pipeline_helios_pyramid.py#L508[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str | list[str] = None"}, {"name": "height", "val": ": int = 384"}, {"name": "width", "val": ": int = 640"}, {"name": "num_frames", "val": ": int = 132"}, {"name": "sigmas", "val": ": list = None"}, {"name": "guidance_scale", "val": ": float = 5.0"}, {"name": "num_videos_per_prompt", "val": ": int | None = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'np'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}, {"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "image_latents", "val": ": torch.Tensor | None = None"}, {"name": "fake_image_latents", "val": ": torch.Tensor | None = None"}, {"name": "add_noise_to_image_latents", "val": ": bool = True"}, {"name": "image_noise_sigma_min", "val": ": float = 0.111"}, {"name": "image_noise_sigma_max", "val": ": float = 0.135"}, {"name": "video", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "video_latents", "val": ": torch.Tensor | None = None"}, {"name": "add_noise_to_video_latents", "val": ": bool = True"}, {"name": "video_noise_sigma_min", "val": ": float = 0.111"}, {"name": "video_noise_sigma_max", "val": ": float = 0.135"}, {"name": "history_sizes", "val": ": list = [16, 2, 1]"}, {"name": "num_latent_frames_per_chunk", "val": ": int = 9"}, {"name": "keep_first_frame", "val": ": bool = True"}, {"name": "is_skip_first_chunk", "val": ": bool = False"}, {"name": "pyramid_num_inference_steps_list", "val": ": list = [10, 10, 10]"}, {"name": "use_zero_init", "val": ": bool | None = True"}, {"name": "zero_steps", "val": ": int | None = 1"}, {"name": "is_amplify_first_chunk", "val": ": bool = False"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, pass `prompt_embeds` instead.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to avoid during image generation. If not defined, pass `negative_prompt_embeds`
  instead. Ignored when not using guidance (`guidance_scale`  1`. Higher guidance scale encourages to generate images that are closely linked to
  the text `prompt`, usually at the expense of lower image quality.
- **num_videos_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make
  generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor is generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not
  provided, text embeddings are generated from the `prompt` input argument.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings. If not provided, they are generated from `negative_prompt`.
- **output_type** (`str`, *optional*, defaults to `"np"`) --
  The output format of the generated image. Choose between `PIL.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `HeliosPipelineOutput` instead of a plain tuple.
- **attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **callback_on_step_end** (`Callable`, `PipelineCallback`, `MultiPipelineCallbacks`, *optional*) --
  A function or a subclass of `PipelineCallback` or `MultiPipelineCallbacks` that is called at the end of
  each denoising step during the inference. with the following arguments: `callback_on_step_end(self:
  DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a
  list of all tensors as specified by `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`list`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.
- **max_sequence_length** (`int`, defaults to `512`) --
  The maximum sequence length of the text encoder. If the prompt is longer than this, it will be
  truncated. If the prompt is shorter, it will be padded to this length.
- **image** (`PipelineImageInput`, *optional*) --
  Input image used for image-to-video conditioning.
- **image_latents** (`torch.Tensor`, *optional*) --
  Pre-encoded image latents to use instead of `image`.
- **fake_image_latents** (`torch.Tensor`, *optional*) --
  Optional fake image latents used during conditioning.
- **add_noise_to_image_latents** (`bool`, *optional*, defaults to `True`) --
  Whether to add noise to the image latents prior to denoising.
- **image_noise_sigma_min** (`float`, *optional*, defaults to `0.111`) --
  Minimum sigma value for noise added to image latents.
- **image_noise_sigma_max** (`float`, *optional*, defaults to `0.135`) --
  Maximum sigma value for noise added to image latents.
- **video** (`PipelineImageInput`, *optional*) --
  Input video used for video-to-video conditioning.
- **video_latents** (`torch.Tensor`, *optional*) --
  Pre-encoded video latents to use instead of `video`.
- **add_noise_to_video_latents** (`bool`, *optional*, defaults to `True`) --
  Whether to add noise to the video latents prior to denoising.
- **video_noise_sigma_min** (`float`, *optional*, defaults to `0.111`) --
  Minimum sigma value for noise added to video latents.
- **video_noise_sigma_max** (`float`, *optional*, defaults to `0.135`) --
  Maximum sigma value for noise added to video latents.
- **history_sizes** (`list`, *optional*, defaults to `[16, 2, 1]`) --
  History window sizes used for autoregressive chunked generation.
- **num_latent_frames_per_chunk** (`int`, *optional*, defaults to `9`) --
  Number of latent frames produced per chunk during autoregressive generation.
- **keep_first_frame** (`bool`, *optional*, defaults to `True`) --
  Whether to retain the first frame across chunks.
- **is_skip_first_chunk** (`bool`, *optional*, defaults to `False`) --
  Whether to skip generation of the first chunk.
- **pyramid_num_inference_steps_list** (`list`, *optional*, defaults to `[10, 10, 10]`) --
  Number of inference steps for each pyramid stage during Stage 2 generation.
- **use_zero_init** (`bool`, *optional*, defaults to `True`) --
  Whether to apply CFG zero-init at the start of denoising.
- **zero_steps** (`int`, *optional*, defaults to `1`) --
  Number of initial steps that use CFG zero-init.
- **is_amplify_first_chunk** (`bool`, *optional*, defaults to `False`) --
  Whether to amplify guidance on the first chunk (DMD-related).0`~HeliosPipelineOutput` or `tuple`If `return_dict` is `True`, `HeliosPipelineOutput` is returned, otherwise a `tuple` is returned where
the first element is a list with the generated images and the second element is a list of `bool`s
indicating whether the corresponding generated image contains "not-safe-for-work" (nsfw) content.

The call function to the pipeline for generation.

Examples:
```python
>>> import torch
>>> from diffusers.utils import export_to_video
>>> from diffusers import AutoencoderKLWan, HeliosPyramidPipeline

>>> # Available models: BestWishYsh/Helios-Base, BestWishYsh/Helios-Mid, BestWishYsh/Helios-Distilled
>>> model_id = "BestWishYsh/Helios-Base"
>>> vae = AutoencoderKLWan.from_pretrained(model_id, subfolder="vae", torch_dtype=torch.float32)
>>> pipe = HeliosPyramidPipeline.from_pretrained(model_id, vae=vae, torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> prompt = "A cat and a dog baking a cake together in a kitchen. The cat is carefully measuring flour, while the dog is stirring the batter with a wooden spoon. The kitchen is cozy, with sunlight streaming through the window."
>>> negative_prompt = "Bright tones, overexposed, static, blurred details, subtitles, style, works, paintings, images, static, overall gray, worst quality, low quality, JPEG compression residue, ugly, incomplete, extra fingers, poorly drawn hands, poorly drawn faces, deformed, disfigured, misshapen limbs, fused fingers, still picture, messy background, three legs, many people in the background, walking backwards"

>>> output = pipe(
...     prompt=prompt,
...     negative_prompt=negative_prompt,
...     height=384,
...     width=640,
...     num_frames=132,
...     guidance_scale=5.0,
... ).frames[0]
>>> export_to_video(output, "output.mp4", fps=24)
```

**Parameters:**

tokenizer (`T5Tokenizer`) : Tokenizer from [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5Tokenizer), specifically the [google/umt5-xxl](https://huggingface.co/google/umt5-xxl) variant.

text_encoder (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/umt5-xxl](https://huggingface.co/google/umt5-xxl) variant.

transformer ([HeliosTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/helios_transformer3d#diffusers.HeliosTransformer3DModel)) : Conditional Transformer to denoise the input latents.

scheduler ([`HeliosScheduler`, `HeliosDMDScheduler`]) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLWan](/docs/diffusers/v0.39.0/en/api/models/autoencoder_kl_wan#diffusers.AutoencoderKLWan)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

**Returns:**

``~HeliosPipelineOutput` or `tuple``

If `return_dict` is `True`, `HeliosPipelineOutput` is returned, otherwise a `tuple` is returned where
the first element is a list with the generated images and the second element is a list of `bool`s
indicating whether the corresponding generated image contains "not-safe-for-work" (nsfw) content.
#### encode_prompt[[diffusers.HeliosPyramidPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/helios/pipeline_helios_pyramid.py#L214)

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

## HeliosPipelineOutput[[diffusers.pipelines.helios.pipeline_output.HeliosPipelineOutput]]

#### diffusers.pipelines.helios.pipeline_output.HeliosPipelineOutput[[diffusers.pipelines.helios.pipeline_output.HeliosPipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/helios/pipeline_output.py#L9)

Output class for Helios pipelines.

**Parameters:**

frames (`torch.Tensor`, `np.ndarray`, or List[List[PIL.Image.Image]]) : List of video outputs - It can be a nested list of length `batch_size,` with each sub-list containing denoised PIL image sequences of length `num_frames.` It can also be a NumPy array or Torch tensor of shape `(batch_size, num_frames, channels, height, width)`.
