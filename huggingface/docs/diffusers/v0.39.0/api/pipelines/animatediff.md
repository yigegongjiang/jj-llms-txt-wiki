# Text-to-Video Generation with AnimateDiff

  

## Overview

[AnimateDiff: Animate Your Personalized Text-to-Image Diffusion Models without Specific Tuning](https://huggingface.co/papers/2307.04725) by Yuwei Guo, Ceyuan Yang, Anyi Rao, Yaohui Wang, Yu Qiao, Dahua Lin, Bo Dai.

The abstract of the paper is the following:

*With the advance of text-to-image models (e.g., Stable Diffusion) and corresponding personalization techniques such as DreamBooth and LoRA, everyone can manifest their imagination into high-quality images at an affordable cost. Subsequently, there is a great demand for image animation techniques to further combine generated static images with motion dynamics. In this report, we propose a practical framework to animate most of the existing personalized text-to-image models once and for all, saving efforts in model-specific tuning. At the core of the proposed framework is to insert a newly initialized motion modeling module into the frozen text-to-image model and train it on video clips to distill reasonable motion priors. Once trained, by simply injecting this motion modeling module, all personalized versions derived from the same base T2I readily become text-driven models that produce diverse and personalized animated images. We conduct our evaluation on several public representative personalized text-to-image models across anime pictures and realistic photographs, and demonstrate that our proposed framework helps these models generate temporally smooth animation clips while preserving the domain and diversity of their outputs. Code and pre-trained weights will be publicly available at [this https URL](https://animatediff.github.io/).*

## Available Pipelines

| Pipeline | Tasks | Demo
|---|---|:---:|
| [AnimateDiffPipeline](https://github.com/huggingface/diffusers/blob/main/src/diffusers/pipelines/animatediff/pipeline_animatediff.py) | *Text-to-Video Generation with AnimateDiff* |
| [AnimateDiffControlNetPipeline](https://github.com/huggingface/diffusers/blob/main/src/diffusers/pipelines/animatediff/pipeline_animatediff_controlnet.py) | *Controlled Video-to-Video Generation with AnimateDiff using ControlNet* |
| [AnimateDiffSparseControlNetPipeline](https://github.com/huggingface/diffusers/blob/main/src/diffusers/pipelines/animatediff/pipeline_animatediff_sparsectrl.py) | *Controlled Video-to-Video Generation with AnimateDiff using SparseCtrl* |
| [AnimateDiffSDXLPipeline](https://github.com/huggingface/diffusers/blob/main/src/diffusers/pipelines/animatediff/pipeline_animatediff_sdxl.py) | *Video-to-Video Generation with AnimateDiff* |
| [AnimateDiffVideoToVideoPipeline](https://github.com/huggingface/diffusers/blob/main/src/diffusers/pipelines/animatediff/pipeline_animatediff_video2video.py) | *Video-to-Video Generation with AnimateDiff* |
| [AnimateDiffVideoToVideoControlNetPipeline](https://github.com/huggingface/diffusers/blob/main/src/diffusers/pipelines/animatediff/pipeline_animatediff_video2video_controlnet.py) | *Video-to-Video Generation with AnimateDiff using ControlNet* |

## Available checkpoints

Motion Adapter checkpoints can be found under [guoyww](https://huggingface.co/guoyww/). These checkpoints are meant to work with any model based on Stable Diffusion 1.4/1.5.

## Usage example

### AnimateDiffPipeline

AnimateDiff works with a MotionAdapter checkpoint and a Stable Diffusion model checkpoint. The MotionAdapter is a collection of Motion Modules that are responsible for adding coherent motion across image frames. These modules are applied after the Resnet and Attention blocks in Stable Diffusion UNet.

The following example demonstrates how to use a *MotionAdapter* checkpoint with Diffusers for inference based on StableDiffusion-1.4/1.5.

```python
import torch
from diffusers import AnimateDiffPipeline, DDIMScheduler, MotionAdapter
from diffusers.utils import export_to_gif

# Load the motion adapter
adapter = MotionAdapter.from_pretrained("guoyww/animatediff-motion-adapter-v1-5-2", torch_dtype=torch.float16)
# load SD 1.5 based finetuned model
model_id = "SG161222/Realistic_Vision_V5.1_noVAE"
pipe = AnimateDiffPipeline.from_pretrained(model_id, motion_adapter=adapter, torch_dtype=torch.float16)
scheduler = DDIMScheduler.from_pretrained(
    model_id,
    subfolder="scheduler",
    clip_sample=False,
    timestep_spacing="linspace",
    beta_schedule="linear",
    steps_offset=1,
)
pipe.scheduler = scheduler

# enable memory savings
pipe.enable_vae_slicing()
pipe.enable_model_cpu_offload()

output = pipe(
    prompt=(
        "masterpiece, bestquality, highlydetailed, ultradetailed, sunset, "
        "orange sky, warm lighting, fishing boats, ocean waves seagulls, "
        "rippling water, wharf, silhouette, serene atmosphere, dusk, evening glow, "
        "golden hour, coastal landscape, seaside scenery"
    ),
    negative_prompt="bad quality, worse quality",
    num_frames=16,
    guidance_scale=7.5,
    num_inference_steps=25,
    generator=torch.Generator("cpu").manual_seed(42),
)
frames = output.frames[0]
export_to_gif(frames, "animation.gif")
```

Here are some sample outputs:

    
        
        masterpiece, bestquality, sunset.
        
        <img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-realistic-doc.gif"
            alt="masterpiece, bestquality, sunset"
            style="width: 300px;" />
        
    

> [!TIP]
> AnimateDiff tends to work better with finetuned Stable Diffusion models. If you plan on using a scheduler that can clip samples, make sure to disable it by setting `clip_sample=False` in the scheduler as this can also have an adverse effect on generated samples. Additionally, the AnimateDiff checkpoints can be sensitive to the beta schedule of the scheduler. We recommend setting this to `linear`.

### AnimateDiffControlNetPipeline

AnimateDiff can also be used with ControlNets ControlNet was introduced in [Adding Conditional Control to Text-to-Image Diffusion Models](https://huggingface.co/papers/2302.05543) by Lvmin Zhang, Anyi Rao, and Maneesh Agrawala. With a ControlNet model, you can provide an additional control image to condition and control Stable Diffusion generation. For example, if you provide depth maps, the ControlNet model generates a video that'll preserve the spatial information from the depth maps. It is a more flexible and accurate way to control the video generation process.

```python
import torch
from diffusers import AnimateDiffControlNetPipeline, AutoencoderKL, ControlNetModel, MotionAdapter, LCMScheduler
from diffusers.utils import export_to_gif, load_video

# Additionally, you will need a preprocess videos before they can be used with the ControlNet
# HF maintains just the right package for it: `pip install controlnet_aux`
from controlnet_aux.processor import ZoeDetector

# Download controlnets from https://huggingface.co/lllyasviel/ControlNet-v1-1 to use .from_single_file
# Download Diffusers-format controlnets, such as https://huggingface.co/lllyasviel/sd-controlnet-depth, to use .from_pretrained()
controlnet = ControlNetModel.from_single_file("control_v11f1p_sd15_depth.pth", torch_dtype=torch.float16)

# We use AnimateLCM for this example but one can use the original motion adapters as well (for example, https://huggingface.co/guoyww/animatediff-motion-adapter-v1-5-3)
motion_adapter = MotionAdapter.from_pretrained("wangfuyun/AnimateLCM")

vae = AutoencoderKL.from_pretrained("stabilityai/sd-vae-ft-mse", torch_dtype=torch.float16)
pipe: AnimateDiffControlNetPipeline = AnimateDiffControlNetPipeline.from_pretrained(
    "SG161222/Realistic_Vision_V5.1_noVAE",
    motion_adapter=motion_adapter,
    controlnet=controlnet,
    vae=vae,
).to(device="cuda", dtype=torch.float16)
pipe.scheduler = LCMScheduler.from_config(pipe.scheduler.config, beta_schedule="linear")
pipe.load_lora_weights("wangfuyun/AnimateLCM", weight_name="AnimateLCM_sd15_t2v_lora.safetensors", adapter_name="lcm-lora")
pipe.set_adapters(["lcm-lora"], [0.8])

depth_detector = ZoeDetector.from_pretrained("lllyasviel/Annotators").to("cuda")
video = load_video("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-vid2vid-input-1.gif")
conditioning_frames = []

with pipe.progress_bar(total=len(video)) as progress_bar:
    for frame in video:
        conditioning_frames.append(depth_detector(frame))
        progress_bar.update()

prompt = "a panda, playing a guitar, sitting in a pink boat, in the ocean, mountains in background, realistic, high quality"
negative_prompt = "bad quality, worst quality"

video = pipe(
    prompt=prompt,
    negative_prompt=negative_prompt,
    num_frames=len(video),
    num_inference_steps=10,
    guidance_scale=2.0,
    conditioning_frames=conditioning_frames,
    generator=torch.Generator().manual_seed(42),
).frames[0]

export_to_gif(video, "animatediff_controlnet.gif", fps=8)
```

Here are some sample outputs:

    
      Source Video
      Output Video
    
    
        
          raccoon playing a guitar
          
          
        
        
          a panda, playing a guitar, sitting in a pink boat, in the ocean, mountains in background, realistic, high quality
          
          
        
    

### AnimateDiffSparseControlNetPipeline

[SparseCtrl: Adding Sparse Controls to Text-to-Video Diffusion Models](https://huggingface.co/papers/2311.16933) for achieving controlled generation in text-to-video diffusion models by Yuwei Guo, Ceyuan Yang, Anyi Rao, Maneesh Agrawala, Dahua Lin, and Bo Dai.

The abstract from the paper is:

*The development of text-to-video (T2V), i.e., generating videos with a given text prompt, has been significantly advanced in recent years. However, relying solely on text prompts often results in ambiguous frame composition due to spatial uncertainty. The research community thus leverages the dense structure signals, e.g., per-frame depth/edge sequences, to enhance controllability, whose collection accordingly increases the burden of inference. In this work, we present SparseCtrl to enable flexible structure control with temporally sparse signals, requiring only one or a few inputs, as shown in Figure 1. It incorporates an additional condition encoder to process these sparse signals while leaving the pre-trained T2V model untouched. The proposed approach is compatible with various modalities, including sketches, depth maps, and RGB images, providing more practical control for video generation and promoting applications such as storyboarding, depth rendering, keyframe animation, and interpolation. Extensive experiments demonstrate the generalization of SparseCtrl on both original and personalized T2V generators. Codes and models will be publicly available at [this https URL](https://guoyww.github.io/projects/SparseCtrl).*

SparseCtrl introduces the following checkpoints for controlled text-to-video generation:

- [SparseCtrl Scribble](https://huggingface.co/guoyww/animatediff-sparsectrl-scribble)
- [SparseCtrl RGB](https://huggingface.co/guoyww/animatediff-sparsectrl-rgb)

#### Using SparseCtrl Scribble

```python
import torch

from diffusers import AnimateDiffSparseControlNetPipeline
from diffusers.models import AutoencoderKL, MotionAdapter, SparseControlNetModel
from diffusers.schedulers import DPMSolverMultistepScheduler
from diffusers.utils import export_to_gif, load_image

model_id = "SG161222/Realistic_Vision_V5.1_noVAE"
motion_adapter_id = "guoyww/animatediff-motion-adapter-v1-5-3"
controlnet_id = "guoyww/animatediff-sparsectrl-scribble"
lora_adapter_id = "guoyww/animatediff-motion-lora-v1-5-3"
vae_id = "stabilityai/sd-vae-ft-mse"
device = "cuda"

motion_adapter = MotionAdapter.from_pretrained(motion_adapter_id, torch_dtype=torch.float16).to(device)
controlnet = SparseControlNetModel.from_pretrained(controlnet_id, torch_dtype=torch.float16).to(device)
vae = AutoencoderKL.from_pretrained(vae_id, torch_dtype=torch.float16).to(device)
scheduler = DPMSolverMultistepScheduler.from_pretrained(
    model_id,
    subfolder="scheduler",
    beta_schedule="linear",
    algorithm_type="dpmsolver++",
    use_karras_sigmas=True,
)
pipe = AnimateDiffSparseControlNetPipeline.from_pretrained(
    model_id,
    motion_adapter=motion_adapter,
    controlnet=controlnet,
    vae=vae,
    scheduler=scheduler,
    torch_dtype=torch.float16,
).to(device)
pipe.load_lora_weights(lora_adapter_id, adapter_name="motion_lora")
pipe.fuse_lora(lora_scale=1.0)

prompt = "an aerial view of a cyberpunk city, night time, neon lights, masterpiece, high quality"
negative_prompt = "low quality, worst quality, letterboxed"

image_files = [
    "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-scribble-1.png",
    "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-scribble-2.png",
    "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-scribble-3.png"
]
condition_frame_indices = [0, 8, 15]
conditioning_frames = [load_image(img_file) for img_file in image_files]

video = pipe(
    prompt=prompt,
    negative_prompt=negative_prompt,
    num_inference_steps=25,
    conditioning_frames=conditioning_frames,
    controlnet_conditioning_scale=1.0,
    controlnet_frame_indices=condition_frame_indices,
    generator=torch.Generator().manual_seed(1337),
).frames[0]
export_to_gif(video, "output.gif")
```

Here are some sample outputs:

    
        
          an aerial view of a cyberpunk city, night time, neon lights, masterpiece, high quality
        
    
    
        
          
            
          
        
        
          
            
          
        
        
          
            
          
        
    
    
        
          
            
          
        
    

#### Using SparseCtrl RGB

```python
import torch

from diffusers import AnimateDiffSparseControlNetPipeline
from diffusers.models import AutoencoderKL, MotionAdapter, SparseControlNetModel
from diffusers.schedulers import DPMSolverMultistepScheduler
from diffusers.utils import export_to_gif, load_image

model_id = "SG161222/Realistic_Vision_V5.1_noVAE"
motion_adapter_id = "guoyww/animatediff-motion-adapter-v1-5-3"
controlnet_id = "guoyww/animatediff-sparsectrl-rgb"
lora_adapter_id = "guoyww/animatediff-motion-lora-v1-5-3"
vae_id = "stabilityai/sd-vae-ft-mse"
device = "cuda"

motion_adapter = MotionAdapter.from_pretrained(motion_adapter_id, torch_dtype=torch.float16).to(device)
controlnet = SparseControlNetModel.from_pretrained(controlnet_id, torch_dtype=torch.float16).to(device)
vae = AutoencoderKL.from_pretrained(vae_id, torch_dtype=torch.float16).to(device)
scheduler = DPMSolverMultistepScheduler.from_pretrained(
    model_id,
    subfolder="scheduler",
    beta_schedule="linear",
    algorithm_type="dpmsolver++",
    use_karras_sigmas=True,
)
pipe = AnimateDiffSparseControlNetPipeline.from_pretrained(
    model_id,
    motion_adapter=motion_adapter,
    controlnet=controlnet,
    vae=vae,
    scheduler=scheduler,
    torch_dtype=torch.float16,
).to(device)
pipe.load_lora_weights(lora_adapter_id, adapter_name="motion_lora")

image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-firework.png")

video = pipe(
    prompt="closeup face photo of man in black clothes, night city street, bokeh, fireworks in background",
    negative_prompt="low quality, worst quality",
    num_inference_steps=25,
    conditioning_frames=image,
    controlnet_frame_indices=[0],
    controlnet_conditioning_scale=1.0,
    generator=torch.Generator().manual_seed(42),
).frames[0]
export_to_gif(video, "output.gif")
```

Here are some sample outputs:

    
        
          closeup face photo of man in black clothes, night city street, bokeh, fireworks in background
        
    
    
        
          
            
          
        
        
          
            
          
        
    

### AnimateDiffSDXLPipeline

AnimateDiff can also be used with SDXL models. This is currently an experimental feature as only a beta release of the motion adapter checkpoint is available.

```python
import torch
from diffusers.models import MotionAdapter
from diffusers import AnimateDiffSDXLPipeline, DDIMScheduler
from diffusers.utils import export_to_gif

adapter = MotionAdapter.from_pretrained("guoyww/animatediff-motion-adapter-sdxl-beta", torch_dtype=torch.float16)

model_id = "stabilityai/stable-diffusion-xl-base-1.0"
scheduler = DDIMScheduler.from_pretrained(
    model_id,
    subfolder="scheduler",
    clip_sample=False,
    timestep_spacing="linspace",
    beta_schedule="linear",
    steps_offset=1,
)
pipe = AnimateDiffSDXLPipeline.from_pretrained(
    model_id,
    motion_adapter=adapter,
    scheduler=scheduler,
    torch_dtype=torch.float16,
    variant="fp16",
).to("cuda")

# enable memory savings
pipe.enable_vae_slicing()
pipe.enable_vae_tiling()

output = pipe(
    prompt="a panda surfing in the ocean, realistic, high quality",
    negative_prompt="low quality, worst quality",
    num_inference_steps=20,
    guidance_scale=8,
    width=1024,
    height=1024,
    num_frames=16,
)

frames = output.frames[0]
export_to_gif(frames, "animation.gif")
```

### AnimateDiffVideoToVideoPipeline

AnimateDiff can also be used to generate visually similar videos or enable style/character/background or other edits starting from an initial video, allowing you to seamlessly explore creative possibilities.

```python
import imageio
import requests
import torch
from diffusers import AnimateDiffVideoToVideoPipeline, DDIMScheduler, MotionAdapter
from diffusers.utils import export_to_gif
from io import BytesIO
from PIL import Image

# Load the motion adapter
adapter = MotionAdapter.from_pretrained("guoyww/animatediff-motion-adapter-v1-5-2", torch_dtype=torch.float16)
# load SD 1.5 based finetuned model
model_id = "SG161222/Realistic_Vision_V5.1_noVAE"
pipe = AnimateDiffVideoToVideoPipeline.from_pretrained(model_id, motion_adapter=adapter, torch_dtype=torch.float16)
scheduler = DDIMScheduler.from_pretrained(
    model_id,
    subfolder="scheduler",
    clip_sample=False,
    timestep_spacing="linspace",
    beta_schedule="linear",
    steps_offset=1,
)
pipe.scheduler = scheduler

# enable memory savings
pipe.enable_vae_slicing()
pipe.enable_model_cpu_offload()

# helper function to load videos
def load_video(file_path: str):
    images = []

    if file_path.startswith(('http://', 'https://')):
        # If the file_path is a URL
        response = requests.get(file_path)
        response.raise_for_status()
        content = BytesIO(response.content)
        vid = imageio.get_reader(content)
    else:
        # Assuming it's a local file path
        vid = imageio.get_reader(file_path)

    for frame in vid:
        pil_image = Image.fromarray(frame)
        images.append(pil_image)

    return images

video = load_video("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-vid2vid-input-1.gif")

output = pipe(
    video = video,
    prompt="panda playing a guitar, on a boat, in the ocean, high quality",
    negative_prompt="bad quality, worse quality",
    guidance_scale=7.5,
    num_inference_steps=25,
    strength=0.5,
    generator=torch.Generator("cpu").manual_seed(42),
)
frames = output.frames[0]
export_to_gif(frames, "animation.gif")
```

Here are some sample outputs:

    
      Source Video
      Output Video
    
    
        
          raccoon playing a guitar
          
          <img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-vid2vid-input-1.gif"
              alt="racoon playing a guitar"
              style="width: 300px;" />
        
        
          panda playing a guitar
          
          <img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-vid2vid-output-1.gif"
              alt="panda playing a guitar"
              style="width: 300px;" />
        
    
    
        
          closeup of margot robbie, fireworks in the background, high quality
          
          <img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-vid2vid-input-2.gif"
              alt="closeup of margot robbie, fireworks in the background, high quality"
              style="width: 300px;" />
        
        
          closeup of tony stark, robert downey jr, fireworks
          
          <img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-vid2vid-output-2.gif"
              alt="closeup of tony stark, robert downey jr, fireworks"
              style="width: 300px;" />
        
    

### AnimateDiffVideoToVideoControlNetPipeline

AnimateDiff can be used together with ControlNets to enhance video-to-video generation by allowing for precise control over the output. ControlNet was introduced in [Adding Conditional Control to Text-to-Image Diffusion Models](https://huggingface.co/papers/2302.05543) by Lvmin Zhang, Anyi Rao, and Maneesh Agrawala, and allows you to condition Stable Diffusion with an additional control image to ensure that the spatial information is preserved throughout the video. 

This pipeline allows you to condition your generation both on the original video and on a sequence of control images.

```python
import torch
from PIL import Image
from tqdm.auto import tqdm

from controlnet_aux.processor import OpenposeDetector
from diffusers import AnimateDiffVideoToVideoControlNetPipeline
from diffusers.utils import export_to_gif, load_video
from diffusers import AutoencoderKL, ControlNetModel, MotionAdapter, LCMScheduler

# Load the ControlNet
controlnet = ControlNetModel.from_pretrained("lllyasviel/sd-controlnet-openpose", torch_dtype=torch.float16)
# Load the motion adapter
motion_adapter = MotionAdapter.from_pretrained("wangfuyun/AnimateLCM")
# Load SD 1.5 based finetuned model
vae = AutoencoderKL.from_pretrained("stabilityai/sd-vae-ft-mse", torch_dtype=torch.float16)
pipe = AnimateDiffVideoToVideoControlNetPipeline.from_pretrained(
    "SG161222/Realistic_Vision_V5.1_noVAE",
    motion_adapter=motion_adapter,
    controlnet=controlnet,
    vae=vae,
).to(device="cuda", dtype=torch.float16)

# Enable LCM to speed up inference
pipe.scheduler = LCMScheduler.from_config(pipe.scheduler.config, beta_schedule="linear")
pipe.load_lora_weights("wangfuyun/AnimateLCM", weight_name="AnimateLCM_sd15_t2v_lora.safetensors", adapter_name="lcm-lora")
pipe.set_adapters(["lcm-lora"], [0.8])

video = load_video("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/dance.gif")
video = [frame.convert("RGB") for frame in video]

prompt = "astronaut in space, dancing"
negative_prompt = "bad quality, worst quality, jpeg artifacts, ugly"

# Create controlnet preprocessor
open_pose = OpenposeDetector.from_pretrained("lllyasviel/Annotators").to("cuda")

# Preprocess controlnet images
conditioning_frames = []
for frame in tqdm(video):
    conditioning_frames.append(open_pose(frame))

strength = 0.8
with torch.inference_mode():
    video = pipe(
        video=video,
        prompt=prompt,
        negative_prompt=negative_prompt,
        num_inference_steps=10,
        guidance_scale=2.0,
        controlnet_conditioning_scale=0.75,
        conditioning_frames=conditioning_frames,
        strength=strength,
        generator=torch.Generator().manual_seed(42),
    ).frames[0]

video = [frame.resize(conditioning_frames[0].size) for frame in video]
export_to_gif(video, f"animatediff_vid2vid_controlnet.gif", fps=8)
```

Here are some sample outputs:

    
      Source Video
      Output Video
    
    
        
          anime girl, dancing
          
          
        
        
          astronaut in space, dancing
          
          
        
    

**The lights and composition were transferred from the Source Video.**

## Using Motion LoRAs

Motion LoRAs are a collection of LoRAs that work with the `guoyww/animatediff-motion-adapter-v1-5-2` checkpoint. These LoRAs are responsible for adding specific types of motion to the animations.

```python
import torch
from diffusers import AnimateDiffPipeline, DDIMScheduler, MotionAdapter
from diffusers.utils import export_to_gif

# Load the motion adapter
adapter = MotionAdapter.from_pretrained("guoyww/animatediff-motion-adapter-v1-5-2", torch_dtype=torch.float16)
# load SD 1.5 based finetuned model
model_id = "SG161222/Realistic_Vision_V5.1_noVAE"
pipe = AnimateDiffPipeline.from_pretrained(model_id, motion_adapter=adapter, torch_dtype=torch.float16)
pipe.load_lora_weights(
    "guoyww/animatediff-motion-lora-zoom-out", adapter_name="zoom-out"
)

scheduler = DDIMScheduler.from_pretrained(
    model_id,
    subfolder="scheduler",
    clip_sample=False,
    beta_schedule="linear",
    timestep_spacing="linspace",
    steps_offset=1,
)
pipe.scheduler = scheduler

# enable memory savings
pipe.enable_vae_slicing()
pipe.enable_model_cpu_offload()

output = pipe(
    prompt=(
        "masterpiece, bestquality, highlydetailed, ultradetailed, sunset, "
        "orange sky, warm lighting, fishing boats, ocean waves seagulls, "
        "rippling water, wharf, silhouette, serene atmosphere, dusk, evening glow, "
        "golden hour, coastal landscape, seaside scenery"
    ),
    negative_prompt="bad quality, worse quality",
    num_frames=16,
    guidance_scale=7.5,
    num_inference_steps=25,
    generator=torch.Generator("cpu").manual_seed(42),
)
frames = output.frames[0]
export_to_gif(frames, "animation.gif")
```

    
        
        masterpiece, bestquality, sunset.
        
        <img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-zoom-out-lora.gif"
            alt="masterpiece, bestquality, sunset"
            style="width: 300px;" />
        
    

## Using Motion LoRAs with PEFT

You can also leverage the [PEFT](https://github.com/huggingface/peft) backend to combine Motion LoRA's and create more complex animations.

First install PEFT with

```shell
pip install peft
```

Then you can use the following code to combine Motion LoRAs.

```python
import torch
from diffusers import AnimateDiffPipeline, DDIMScheduler, MotionAdapter
from diffusers.utils import export_to_gif

# Load the motion adapter
adapter = MotionAdapter.from_pretrained("guoyww/animatediff-motion-adapter-v1-5-2", torch_dtype=torch.float16)
# load SD 1.5 based finetuned model
model_id = "SG161222/Realistic_Vision_V5.1_noVAE"
pipe = AnimateDiffPipeline.from_pretrained(model_id, motion_adapter=adapter, torch_dtype=torch.float16)

pipe.load_lora_weights(
    "diffusers/animatediff-motion-lora-zoom-out", adapter_name="zoom-out",
)
pipe.load_lora_weights(
    "diffusers/animatediff-motion-lora-pan-left", adapter_name="pan-left",
)
pipe.set_adapters(["zoom-out", "pan-left"], adapter_weights=[1.0, 1.0])

scheduler = DDIMScheduler.from_pretrained(
    model_id,
    subfolder="scheduler",
    clip_sample=False,
    timestep_spacing="linspace",
    beta_schedule="linear",
    steps_offset=1,
)
pipe.scheduler = scheduler

# enable memory savings
pipe.enable_vae_slicing()
pipe.enable_model_cpu_offload()

output = pipe(
    prompt=(
        "masterpiece, bestquality, highlydetailed, ultradetailed, sunset, "
        "orange sky, warm lighting, fishing boats, ocean waves seagulls, "
        "rippling water, wharf, silhouette, serene atmosphere, dusk, evening glow, "
        "golden hour, coastal landscape, seaside scenery"
    ),
    negative_prompt="bad quality, worse quality",
    num_frames=16,
    guidance_scale=7.5,
    num_inference_steps=25,
    generator=torch.Generator("cpu").manual_seed(42),
)
frames = output.frames[0]
export_to_gif(frames, "animation.gif")
```

    
        
        masterpiece, bestquality, sunset.
        
        <img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-zoom-out-pan-left-lora.gif"
            alt="masterpiece, bestquality, sunset"
            style="width: 300px;" />
        
    

## Using FreeInit

[FreeInit: Bridging Initialization Gap in Video Diffusion Models](https://huggingface.co/papers/2312.07537) by Tianxing Wu, Chenyang Si, Yuming Jiang, Ziqi Huang, Ziwei Liu.

FreeInit is an effective method that improves temporal consistency and overall quality of videos generated using video-diffusion-models without any addition training. It can be applied to AnimateDiff, ModelScope, VideoCrafter and various other video generation models seamlessly at inference time, and works by iteratively refining the latent-initialization noise. More details can be found it the paper.

The following example demonstrates the usage of FreeInit.

```python
import torch
from diffusers import MotionAdapter, AnimateDiffPipeline, DDIMScheduler
from diffusers.utils import export_to_gif

adapter = MotionAdapter.from_pretrained("guoyww/animatediff-motion-adapter-v1-5-2")
model_id = "SG161222/Realistic_Vision_V5.1_noVAE"
pipe = AnimateDiffPipeline.from_pretrained(model_id, motion_adapter=adapter, torch_dtype=torch.float16).to("cuda")
pipe.scheduler = DDIMScheduler.from_pretrained(
    model_id,
    subfolder="scheduler",
    beta_schedule="linear",
    clip_sample=False,
    timestep_spacing="linspace",
    steps_offset=1
)

# enable memory savings
pipe.enable_vae_slicing()
pipe.enable_vae_tiling()

# enable FreeInit
# Refer to the enable_free_init documentation for a full list of configurable parameters
pipe.enable_free_init(method="butterworth", use_fast_sampling=True)

# run inference
output = pipe(
    prompt="a panda playing a guitar, on a boat, in the ocean, high quality",
    negative_prompt="bad quality, worse quality",
    num_frames=16,
    guidance_scale=7.5,
    num_inference_steps=20,
    generator=torch.Generator("cpu").manual_seed(666),
)

# disable FreeInit
pipe.disable_free_init()

frames = output.frames[0]
export_to_gif(frames, "animation.gif")
```

> [!WARNING]
> FreeInit is not really free - the improved quality comes at the cost of extra computation. It requires sampling a few extra times depending on the `num_iters` parameter that is set when enabling it. Setting the `use_fast_sampling` parameter to `True` can improve the overall performance (at the cost of lower quality compared to when `use_fast_sampling=False` but still better results than vanilla video generation models).

> [!TIP]
> Make sure to check out the Schedulers [guide](../../using-diffusers/schedulers) to learn how to explore the tradeoff between scheduler speed and quality, and see the [reuse components across pipelines](../../using-diffusers/loading#reuse-a-pipeline) section to learn how to efficiently load the same components into multiple pipelines.

    
      Without FreeInit enabled
      With FreeInit enabled
    
    
        
          panda playing a guitar
          
          <img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-no-freeinit.gif"
              alt="panda playing a guitar"
              style="width: 300px;" />
        
        
          panda playing a guitar
          
          <img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-freeinit.gif"
              alt="panda playing a guitar"
              style="width: 300px;" />
        
    

## Using AnimateLCM

[AnimateLCM](https://animatelcm.github.io/) is a motion module checkpoint and an [LCM LoRA](https://huggingface.co/docs/diffusers/using-diffusers/inference_with_lcm_lora) that have been created using a consistency learning strategy that decouples the distillation of the image generation priors and the motion generation priors.

```python
import torch
from diffusers import AnimateDiffPipeline, LCMScheduler, MotionAdapter
from diffusers.utils import export_to_gif

adapter = MotionAdapter.from_pretrained("wangfuyun/AnimateLCM")
pipe = AnimateDiffPipeline.from_pretrained("emilianJR/epiCRealism", motion_adapter=adapter)
pipe.scheduler = LCMScheduler.from_config(pipe.scheduler.config, beta_schedule="linear")

pipe.load_lora_weights("wangfuyun/AnimateLCM", weight_name="sd15_lora_beta.safetensors", adapter_name="lcm-lora")

pipe.enable_vae_slicing()
pipe.enable_model_cpu_offload()

output = pipe(
    prompt="A space rocket with trails of smoke behind it launching into space from the desert, 4k, high resolution",
    negative_prompt="bad quality, worse quality, low resolution",
    num_frames=16,
    guidance_scale=1.5,
    num_inference_steps=6,
    generator=torch.Generator("cpu").manual_seed(0),
)
frames = output.frames[0]
export_to_gif(frames, "animatelcm.gif")
```

    
        
        A space rocket, 4K.
        
        <img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatelcm-output.gif"
            alt="A space rocket, 4K"
            style="width: 300px;" />
        
    

AnimateLCM is also compatible with existing [Motion LoRAs](https://huggingface.co/collections/dn6/animatediff-motion-loras-654cb8ad732b9e3cf4d3c17e).

```python
import torch
from diffusers import AnimateDiffPipeline, LCMScheduler, MotionAdapter
from diffusers.utils import export_to_gif

adapter = MotionAdapter.from_pretrained("wangfuyun/AnimateLCM")
pipe = AnimateDiffPipeline.from_pretrained("emilianJR/epiCRealism", motion_adapter=adapter)
pipe.scheduler = LCMScheduler.from_config(pipe.scheduler.config, beta_schedule="linear")

pipe.load_lora_weights("wangfuyun/AnimateLCM", weight_name="sd15_lora_beta.safetensors", adapter_name="lcm-lora")
pipe.load_lora_weights("guoyww/animatediff-motion-lora-tilt-up", adapter_name="tilt-up")

pipe.set_adapters(["lcm-lora", "tilt-up"], [1.0, 0.8])
pipe.enable_vae_slicing()
pipe.enable_model_cpu_offload()

output = pipe(
    prompt="A space rocket with trails of smoke behind it launching into space from the desert, 4k, high resolution",
    negative_prompt="bad quality, worse quality, low resolution",
    num_frames=16,
    guidance_scale=1.5,
    num_inference_steps=6,
    generator=torch.Generator("cpu").manual_seed(0),
)
frames = output.frames[0]
export_to_gif(frames, "animatelcm-motion-lora.gif")
```

    
        
        A space rocket, 4K.
        
        <img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatelcm-motion-lora.gif"
            alt="A space rocket, 4K"
            style="width: 300px;" />
        
    

## Using FreeNoise

[FreeNoise: Tuning-Free Longer Video Diffusion via Noise Rescheduling](https://huggingface.co/papers/2310.15169) by Haonan Qiu, Menghan Xia, Yong Zhang, Yingqing He, Xintao Wang, Ying Shan, Ziwei Liu.

FreeNoise is a sampling mechanism that can generate longer videos with short-video generation models by employing noise-rescheduling, temporal attention over sliding windows, and weighted averaging of latent frames. It also can be used with multiple prompts to allow for interpolated video generations. More details are available in the paper.

The currently supported AnimateDiff pipelines that can be used with FreeNoise are:
- [AnimateDiffPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.AnimateDiffPipeline)
- [AnimateDiffControlNetPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.AnimateDiffControlNetPipeline)
- [AnimateDiffVideoToVideoPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.AnimateDiffVideoToVideoPipeline)
- [AnimateDiffVideoToVideoControlNetPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.AnimateDiffVideoToVideoControlNetPipeline)

In order to use FreeNoise, a single line needs to be added to the inference code after loading your pipelines.

```diff
+ pipe.enable_free_noise()
```

After this, either a single prompt could be used, or multiple prompts can be passed as a dictionary of integer-string pairs. The integer keys of the dictionary correspond to the frame index at which the influence of that prompt would be maximum. Each frame index should map to a single string prompt. The prompts for intermediate frame indices, that are not passed in the dictionary, are created by interpolating between the frame prompts that are passed. By default, simple linear interpolation is used. However, you can customize this behaviour with a callback to the `prompt_interpolation_callback` parameter when enabling FreeNoise.

Full example:

```python
import torch
from diffusers import AutoencoderKL, AnimateDiffPipeline, LCMScheduler, MotionAdapter
from diffusers.utils import export_to_video, load_image

# Load pipeline
dtype = torch.float16
motion_adapter = MotionAdapter.from_pretrained("wangfuyun/AnimateLCM", torch_dtype=dtype)
vae = AutoencoderKL.from_pretrained("stabilityai/sd-vae-ft-mse", torch_dtype=dtype)

pipe = AnimateDiffPipeline.from_pretrained("emilianJR/epiCRealism", motion_adapter=motion_adapter, vae=vae, torch_dtype=dtype)
pipe.scheduler = LCMScheduler.from_config(pipe.scheduler.config, beta_schedule="linear")

pipe.load_lora_weights(
    "wangfuyun/AnimateLCM", weight_name="AnimateLCM_sd15_t2v_lora.safetensors", adapter_name="lcm_lora"
)
pipe.set_adapters(["lcm_lora"], [0.8])

# Enable FreeNoise for long prompt generation
pipe.enable_free_noise(context_length=16, context_stride=4)
pipe.to("cuda")

# Can be a single prompt, or a dictionary with frame timesteps
prompt = {
    0: "A caterpillar on a leaf, high quality, photorealistic",
    40: "A caterpillar transforming into a cocoon, on a leaf, near flowers, photorealistic",
    80: "A cocoon on a leaf, flowers in the background, photorealistic",
    120: "A cocoon maturing and a butterfly being born, flowers and leaves visible in the background, photorealistic",
    160: "A beautiful butterfly, vibrant colors, sitting on a leaf, flowers in the background, photorealistic",
    200: "A beautiful butterfly, flying away in a forest, photorealistic",
    240: "A cyberpunk butterfly, neon lights, glowing",
}
negative_prompt = "bad quality, worst quality, jpeg artifacts"

# Run inference
output = pipe(
    prompt=prompt,
    negative_prompt=negative_prompt,
    num_frames=256,
    guidance_scale=2.5,
    num_inference_steps=10,
    generator=torch.Generator("cpu").manual_seed(0),
)

# Save video
frames = output.frames[0]
export_to_video(frames, "output.mp4", fps=16)
```

### FreeNoise memory savings

Since FreeNoise processes multiple frames together, there are parts in the modeling where the memory required exceeds that available on normal consumer GPUs. The main memory bottlenecks that we identified are spatial and temporal attention blocks, upsampling and downsampling blocks, resnet blocks and feed-forward layers. Since most of these blocks operate effectively only on the channel/embedding dimension, one can perform chunked inference across the batch dimensions. The batch dimension in AnimateDiff are either spatial (`[B x F, H x W, C]`) or temporal (`B x H x W, F, C`) in nature (note that it may seem counter-intuitive, but the batch dimension here are correct, because spatial blocks process across the `B x F` dimension while the temporal blocks process across the `B x H x W` dimension). We introduce a `SplitInferenceModule` that makes it easier to chunk across any dimension and perform inference. This saves a lot of memory but comes at the cost of requiring more time for inference.

```diff
# Load pipeline and adapters
# ...
+ pipe.enable_free_noise_split_inference()
+ pipe.unet.enable_forward_chunking(16)
```

The call to `pipe.enable_free_noise_split_inference` method accepts two parameters: `spatial_split_size` (defaults to `256`) and `temporal_split_size` (defaults to `16`). These can be configured based on how much VRAM you have available. A lower split size results in lower memory usage but slower inference, whereas a larger split size results in faster inference at the cost of more memory.

## Using `from_single_file` with the MotionAdapter

`diffusers>=0.30.0` supports loading the AnimateDiff checkpoints into the `MotionAdapter` in their original format via `from_single_file`

```python
from diffusers import MotionAdapter

ckpt_path = "https://huggingface.co/Lightricks/LongAnimateDiff/blob/main/lt_long_mm_32_frames.ckpt"

adapter = MotionAdapter.from_single_file(ckpt_path, torch_dtype=torch.float16)
pipe = AnimateDiffPipeline.from_pretrained("emilianJR/epiCRealism", motion_adapter=adapter)
```

## AnimateDiffPipeline[[diffusers.AnimateDiffPipeline]]

#### diffusers.AnimateDiffPipeline[[diffusers.AnimateDiffPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/animatediff/pipeline_animatediff.py#L78)

Pipeline for text-to-video generation.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

The pipeline also inherits the following loading methods:
- [load_textual_inversion()](/docs/diffusers/v0.39.0/en/api/loaders/textual_inversion#diffusers.loaders.TextualInversionLoaderMixin.load_textual_inversion) for loading textual inversion embeddings
- [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for loading LoRA weights
- [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for saving LoRA weights
- [load_ip_adapter()](/docs/diffusers/v0.39.0/en/api/loaders/ip_adapter#diffusers.loaders.IPAdapterMixin.load_ip_adapter) for loading IP Adapters

__call__diffusers.AnimateDiffPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/animatediff/pipeline_animatediff.py#L571[{"name": "prompt", "val": ": str | list[str] | None = None"}, {"name": "num_frames", "val": ": int | None = 16"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "guidance_scale", "val": ": float = 7.5"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "num_videos_per_prompt", "val": ": int | None = 1"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "ip_adapter_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "ip_adapter_image_embeds", "val": ": list[torch.Tensor] | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "clip_skip", "val": ": int | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "decode_chunk_size", "val": ": int = 16"}, {"name": "**kwargs", "val": ""}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide image generation. If not defined, you need to pass `prompt_embeds`.
- **height** (`int`, *optional*, defaults to `self.unet.config.sample_size * self.vae_scale_factor`) --
  The height in pixels of the generated video.
- **width** (`int`, *optional*, defaults to `self.unet.config.sample_size * self.vae_scale_factor`) --
  The width in pixels of the generated video.
- **num_frames** (`int`, *optional*, defaults to 16) --
  The number of video frames that are generated. Defaults to 16 frames which at 8 frames per seconds
  amounts to 2 seconds of video.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality videos at the
  expense of slower inference.
- **guidance_scale** (`float`, *optional*, defaults to 7.5) --
  A higher guidance scale value encourages the model to generate images closely linked to the text
  `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide what to not include in image generation. If not defined, you need to
  pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale 0[AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) or `tuple`If `return_dict` is `True`, [AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) is
returned, otherwise a `tuple` is returned where the first element is a list with the generated frames.

The call function to the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import MotionAdapter, AnimateDiffPipeline, DDIMScheduler
>>> from diffusers.utils import export_to_gif

>>> adapter = MotionAdapter.from_pretrained("guoyww/animatediff-motion-adapter-v1-5-2")
>>> pipe = AnimateDiffPipeline.from_pretrained("frankjoshua/toonyou_beta6", motion_adapter=adapter)
>>> pipe.scheduler = DDIMScheduler(beta_schedule="linear", steps_offset=1, clip_sample=False)
>>> output = pipe(prompt="A corgi walking in the park")
>>> frames = output.frames[0]
>>> export_to_gif(frames, "animation.gif")
```

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : Frozen text-encoder ([clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14)).

tokenizer (`CLIPTokenizer`) : A [CLIPTokenizer](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTokenizer) to tokenize text.

unet ([UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : A [UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel) used to create a UNetMotionModel to denoise the encoded video latents.

motion_adapter (`MotionAdapter`) : A `MotionAdapter` to be used in combination with `unet` to denoise the encoded video latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `unet` to denoise the encoded image latents. Can be one of [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), [LMSDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lms_discrete#diffusers.LMSDiscreteScheduler), or [PNDMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/pndm#diffusers.PNDMScheduler).

**Returns:**

`[AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) or `tuple``

If `return_dict` is `True`, [AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) is
returned, otherwise a `tuple` is returned where the first element is a list with the generated frames.
#### encode_prompt[[diffusers.AnimateDiffPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/animatediff/pipeline_animatediff.py#L154)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

do_classifier_free_guidance (`bool`) : whether to use classifier free guidance or not

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

lora_scale (`float`, *optional*) : A LoRA scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

clip_skip (`int`, *optional*) : Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that the output of the pre-final layer will be used for computing the prompt embeddings.

## AnimateDiffControlNetPipeline[[diffusers.AnimateDiffControlNetPipeline]]

#### diffusers.AnimateDiffControlNetPipeline[[diffusers.AnimateDiffControlNetPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/animatediff/pipeline_animatediff_controlnet.py#L120)

Pipeline for text-to-video generation with ControlNet guidance.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

The pipeline also inherits the following loading methods:
- [load_textual_inversion()](/docs/diffusers/v0.39.0/en/api/loaders/textual_inversion#diffusers.loaders.TextualInversionLoaderMixin.load_textual_inversion) for loading textual inversion embeddings
- [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for loading LoRA weights
- [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for saving LoRA weights
- [load_ip_adapter()](/docs/diffusers/v0.39.0/en/api/loaders/ip_adapter#diffusers.loaders.IPAdapterMixin.load_ip_adapter) for loading IP Adapters

__call__diffusers.AnimateDiffControlNetPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/animatediff/pipeline_animatediff_controlnet.py#L721[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "num_frames", "val": ": int | None = 16"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "guidance_scale", "val": ": float = 7.5"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "num_videos_per_prompt", "val": ": int | None = 1"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "ip_adapter_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "ip_adapter_image_embeds", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "conditioning_frames", "val": ": list[PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor]] | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "controlnet_conditioning_scale", "val": ": float | list[float] = 1.0"}, {"name": "guess_mode", "val": ": bool = False"}, {"name": "control_guidance_start", "val": ": float | list[float] = 0.0"}, {"name": "control_guidance_end", "val": ": float | list[float] = 1.0"}, {"name": "clip_skip", "val": ": int | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "decode_chunk_size", "val": ": int = 16"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide image generation. If not defined, you need to pass `prompt_embeds`.
- **height** (`int`, *optional*, defaults to `self.unet.config.sample_size * self.vae_scale_factor`) --
  The height in pixels of the generated video.
- **width** (`int`, *optional*, defaults to `self.unet.config.sample_size * self.vae_scale_factor`) --
  The width in pixels of the generated video.
- **num_frames** (`int`, *optional*, defaults to 16) --
  The number of video frames that are generated. Defaults to 16 frames which at 8 frames per seconds
  amounts to 2 seconds of video.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality videos at the
  expense of slower inference.
- **guidance_scale** (`float`, *optional*, defaults to 7.5) --
  A higher guidance scale value encourages the model to generate images closely linked to the text
  `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide what to not include in image generation. If not defined, you need to
  pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale 0[AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) or `tuple`If `return_dict` is `True`, [AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) is
returned, otherwise a `tuple` is returned where the first element is a list with the generated frames.

The call function to the pipeline for generation.

Examples:

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : Frozen text-encoder ([clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14)).

tokenizer (`CLIPTokenizer`) : A [CLIPTokenizer](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTokenizer) to tokenize text.

unet ([UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : A [UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel) used to create a UNetMotionModel to denoise the encoded video latents.

motion_adapter (`MotionAdapter`) : A `MotionAdapter` to be used in combination with `unet` to denoise the encoded video latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `unet` to denoise the encoded image latents. Can be one of [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), [LMSDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lms_discrete#diffusers.LMSDiscreteScheduler), or [PNDMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/pndm#diffusers.PNDMScheduler).

**Returns:**

`[AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) or `tuple``

If `return_dict` is `True`, [AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) is
returned, otherwise a `tuple` is returned where the first element is a list with the generated frames.
#### encode_prompt[[diffusers.AnimateDiffControlNetPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/animatediff/pipeline_animatediff_controlnet.py#L199)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

do_classifier_free_guidance (`bool`) : whether to use classifier free guidance or not

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

lora_scale (`float`, *optional*) : A LoRA scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

clip_skip (`int`, *optional*) : Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that the output of the pre-final layer will be used for computing the prompt embeddings.

## AnimateDiffSparseControlNetPipeline[[diffusers.AnimateDiffSparseControlNetPipeline]]

#### diffusers.AnimateDiffSparseControlNetPipeline[[diffusers.AnimateDiffSparseControlNetPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/animatediff/pipeline_animatediff_sparsectrl.py#L132)

Pipeline for controlled text-to-video generation using the method described in [SparseCtrl: Adding Sparse Controls
to Text-to-Video Diffusion Models](https://huggingface.co/papers/2311.16933).

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

The pipeline also inherits the following loading methods:
- [load_textual_inversion()](/docs/diffusers/v0.39.0/en/api/loaders/textual_inversion#diffusers.loaders.TextualInversionLoaderMixin.load_textual_inversion) for loading textual inversion embeddings
- [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for loading LoRA weights
- [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for saving LoRA weights
- [load_ip_adapter()](/docs/diffusers/v0.39.0/en/api/loaders/ip_adapter#diffusers.loaders.IPAdapterMixin.load_ip_adapter) for loading IP Adapters

__call__diffusers.AnimateDiffSparseControlNetPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/animatediff/pipeline_animatediff_sparsectrl.py#L712[{"name": "prompt", "val": ": str | list[str] | None = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_frames", "val": ": int = 16"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "guidance_scale", "val": ": float = 7.5"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "num_videos_per_prompt", "val": ": int = 1"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "ip_adapter_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "ip_adapter_image_embeds", "val": ": list[torch.Tensor] | None = None"}, {"name": "conditioning_frames", "val": ": list[PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor]] | None = None"}, {"name": "output_type", "val": ": str = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "controlnet_conditioning_scale", "val": ": float | list[float] = 1.0"}, {"name": "controlnet_frame_indices", "val": ": list = [0]"}, {"name": "guess_mode", "val": ": bool = False"}, {"name": "clip_skip", "val": ": int | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide image generation. If not defined, you need to pass `prompt_embeds`.
- **height** (`int`, *optional*, defaults to `self.unet.config.sample_size * self.vae_scale_factor`) --
  The height in pixels of the generated video.
- **width** (`int`, *optional*, defaults to `self.unet.config.sample_size * self.vae_scale_factor`) --
  The width in pixels of the generated video.
- **num_frames** (`int`, *optional*, defaults to 16) --
  The number of video frames that are generated. Defaults to 16 frames which at 8 frames per seconds
  amounts to 2 seconds of video.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality videos at the
  expense of slower inference.
- **guidance_scale** (`float`, *optional*, defaults to 7.5) --
  A higher guidance scale value encourages the model to generate images closely linked to the text
  `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide what to not include in image generation. If not defined, you need to
  pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale 0[AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) or `tuple`If `return_dict` is `True`, [AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) is
returned, otherwise a `tuple` is returned where the first element is a list with the generated frames.

The call function to the pipeline for generation.

Examples:
```python
>>> import torch
>>> from diffusers import AnimateDiffSparseControlNetPipeline
>>> from diffusers.models import AutoencoderKL, MotionAdapter, SparseControlNetModel
>>> from diffusers.schedulers import DPMSolverMultistepScheduler
>>> from diffusers.utils import export_to_gif, load_image

>>> model_id = "SG161222/Realistic_Vision_V5.1_noVAE"
>>> motion_adapter_id = "guoyww/animatediff-motion-adapter-v1-5-3"
>>> controlnet_id = "guoyww/animatediff-sparsectrl-scribble"
>>> lora_adapter_id = "guoyww/animatediff-motion-lora-v1-5-3"
>>> vae_id = "stabilityai/sd-vae-ft-mse"
>>> device = "cuda"

>>> motion_adapter = MotionAdapter.from_pretrained(motion_adapter_id, torch_dtype=torch.float16).to(device)
>>> controlnet = SparseControlNetModel.from_pretrained(controlnet_id, torch_dtype=torch.float16).to(device)
>>> vae = AutoencoderKL.from_pretrained(vae_id, torch_dtype=torch.float16).to(device)
>>> scheduler = DPMSolverMultistepScheduler.from_pretrained(
...     model_id,
...     subfolder="scheduler",
...     beta_schedule="linear",
...     algorithm_type="dpmsolver++",
...     use_karras_sigmas=True,
... )
>>> pipe = AnimateDiffSparseControlNetPipeline.from_pretrained(
...     model_id,
...     motion_adapter=motion_adapter,
...     controlnet=controlnet,
...     vae=vae,
...     scheduler=scheduler,
...     torch_dtype=torch.float16,
... ).to(device)
>>> pipe.load_lora_weights(lora_adapter_id, adapter_name="motion_lora")
>>> pipe.fuse_lora(lora_scale=1.0)

>>> prompt = "an aerial view of a cyberpunk city, night time, neon lights, masterpiece, high quality"
>>> negative_prompt = "low quality, worst quality, letterboxed"

>>> image_files = [
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-scribble-1.png",
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-scribble-2.png",
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-scribble-3.png",
... ]
>>> condition_frame_indices = [0, 8, 15]
>>> conditioning_frames = [load_image(img_file) for img_file in image_files]

>>> video = pipe(
...     prompt=prompt,
...     negative_prompt=negative_prompt,
...     num_inference_steps=25,
...     conditioning_frames=conditioning_frames,
...     controlnet_conditioning_scale=1.0,
...     controlnet_frame_indices=condition_frame_indices,
...     generator=torch.Generator().manual_seed(1337),
... ).frames[0]
>>> export_to_gif(video, "output.gif")
```

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : Frozen text-encoder ([clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14)).

tokenizer (`CLIPTokenizer`) : A [CLIPTokenizer](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTokenizer) to tokenize text.

unet ([UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : A [UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel) used to create a UNetMotionModel to denoise the encoded video latents.

motion_adapter (`MotionAdapter`) : A `MotionAdapter` to be used in combination with `unet` to denoise the encoded video latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `unet` to denoise the encoded image latents. Can be one of [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), [LMSDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lms_discrete#diffusers.LMSDiscreteScheduler), or [PNDMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/pndm#diffusers.PNDMScheduler).

**Returns:**

`[AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) or `tuple``

If `return_dict` is `True`, [AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) is
returned, otherwise a `tuple` is returned where the first element is a list with the generated frames.
#### encode_prompt[[diffusers.AnimateDiffSparseControlNetPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/animatediff/pipeline_animatediff_sparsectrl.py#L208)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

do_classifier_free_guidance (`bool`) : whether to use classifier free guidance or not

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

lora_scale (`float`, *optional*) : A LoRA scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

clip_skip (`int`, *optional*) : Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that the output of the pre-final layer will be used for computing the prompt embeddings.

## AnimateDiffSDXLPipeline[[diffusers.AnimateDiffSDXLPipeline]]

#### diffusers.AnimateDiffSDXLPipeline[[diffusers.AnimateDiffSDXLPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/animatediff/pipeline_animatediff_sdxl.py#L210)

Pipeline for text-to-video generation using Stable Diffusion XL.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

The pipeline also inherits the following loading methods:
- [load_textual_inversion()](/docs/diffusers/v0.39.0/en/api/loaders/textual_inversion#diffusers.loaders.TextualInversionLoaderMixin.load_textual_inversion) for loading textual inversion embeddings
- [from_single_file()](/docs/diffusers/v0.39.0/en/api/loaders/single_file#diffusers.loaders.FromSingleFileMixin.from_single_file) for loading `.ckpt` files
- [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionXLLoraLoaderMixin.load_lora_weights) for loading LoRA weights
- [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionXLLoraLoaderMixin.save_lora_weights) for saving LoRA weights
- [load_ip_adapter()](/docs/diffusers/v0.39.0/en/api/loaders/ip_adapter#diffusers.loaders.IPAdapterMixin.load_ip_adapter) for loading IP Adapters

__call__diffusers.AnimateDiffSDXLPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/animatediff/pipeline_animatediff_sdxl.py#L867[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "prompt_2", "val": ": str | list[str] | None = None"}, {"name": "num_frames", "val": ": int = 16"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "timesteps", "val": ": list = None"}, {"name": "sigmas", "val": ": list = None"}, {"name": "denoising_end", "val": ": float | None = None"}, {"name": "guidance_scale", "val": ": float = 5.0"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "negative_prompt_2", "val": ": str | list[str] | None = None"}, {"name": "num_videos_per_prompt", "val": ": int | None = 1"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "pooled_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_pooled_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "ip_adapter_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "ip_adapter_image_embeds", "val": ": list[torch.Tensor] | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "guidance_rescale", "val": ": float = 0.0"}, {"name": "original_size", "val": ": tuple[int, int] | None = None"}, {"name": "crops_coords_top_left", "val": ": tuple = (0, 0)"}, {"name": "target_size", "val": ": tuple[int, int] | None = None"}, {"name": "negative_original_size", "val": ": tuple[int, int] | None = None"}, {"name": "negative_crops_coords_top_left", "val": ": tuple = (0, 0)"}, {"name": "negative_target_size", "val": ": tuple[int, int] | None = None"}, {"name": "clip_skip", "val": ": int | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the video generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **prompt_2** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is
  used in both text-encoders
- **num_frames** --
  The number of video frames that are generated. Defaults to 16 frames which at 8 frames per seconds
  amounts to 2 seconds of video.
- **height** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The height in pixels of the generated video. This is set to 1024 by default for the best results.
  Anything below 512 pixels won't work well for
  [stabilityai/stable-diffusion-xl-base-1.0](https://huggingface.co/stabilityai/stable-diffusion-xl-base-1.0)
  and checkpoints that are not specifically fine-tuned on low resolutions.
- **width** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The width in pixels of the generated video. This is set to 1024 by default for the best results.
  Anything below 512 pixels won't work well for
  [stabilityai/stable-diffusion-xl-base-1.0](https://huggingface.co/stabilityai/stable-diffusion-xl-base-1.0)
  and checkpoints that are not specifically fine-tuned on low resolutions.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality video at the
  expense of slower inference.
- **timesteps** (`list[int]`, *optional*) --
  Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument
  in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is
  passed will be used. Must be in descending order.
- **sigmas** (`list[float]`, *optional*) --
  Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in
  their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed
  will be used.
- **denoising_end** (`float`, *optional*) --
  When specified, determines the fraction (between 0.0 and 1.0) of the total denoising process to be
  completed before it is intentionally prematurely terminated. As a result, the returned sample will
  still retain a substantial amount of noise as determined by the discrete timesteps selected by the
  scheduler. The denoising_end parameter should ideally be utilized when this pipeline forms a part of a
  "Mixture of Denoisers" multi-pipeline setup, as elaborated in [**Refining the Image
  Output**](https://huggingface.co/docs/diffusers/api/pipelines/stable_diffusion/stable_diffusion_xl#refining-the-image-output)
- **guidance_scale** (`float`, *optional*, defaults to 5.0) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2.
  of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting
  `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to
  the text `prompt`, usually at the expense of lower video quality.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the video generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is
  less than `1`).
- **negative_prompt_2** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the video generation to be sent to `tokenizer_2` and
  `text_encoder_2`. If not defined, `negative_prompt` is used in both text-encoders
- **num_videos_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of videos to generate per prompt.
- **eta** (`float`, *optional*, defaults to 0.0) --
  Corresponds to parameter eta (η) in the DDIM paper: https://huggingface.co/papers/2010.02502. Only
  applies to [schedulers.DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), will be ignored for others.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for video
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
  argument.
- **pooled_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting.
  If not provided, pooled text embeddings will be generated from `prompt` input argument.
- **negative_pooled_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, pooled negative_prompt_embeds will be generated from `negative_prompt`
  input argument.
- **ip_adapter_image** -- (`PipelineImageInput`, *optional*):
  Optional image input to work with IP Adapters.
- **ip_adapter_image_embeds** (`list[torch.Tensor]`, *optional*) --
  Pre-generated image embeddings for IP-Adapter. If not provided, embeddings are computed from the
  `ip_adapter_image` input argument.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generated video. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.stable_diffusion_xl.AnimateDiffPipelineOutput` instead of a
  plain tuple.
- **cross_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **guidance_rescale** (`float`, *optional*, defaults to 0.0) --
  Guidance rescale factor proposed by [Common Diffusion Noise Schedules and Sample Steps are
  Flawed](https://huggingface.co/papers/2305.08891) `guidance_scale` is defined as `φ` in equation 16. of
  [Common Diffusion Noise Schedules and Sample Steps are
  Flawed](https://huggingface.co/papers/2305.08891). Guidance rescale factor should fix overexposure when
  using zero terminal SNR.
- **original_size** (`tuple[int]`, *optional*, defaults to (1024, 1024)) --
  If `original_size` is not the same as `target_size` the image will appear to be down- or upsampled.
  `original_size` defaults to `(height, width)` if not specified. Part of SDXL's micro-conditioning as
  explained in section 2.2 of
  [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).
- **crops_coords_top_left** (`tuple[int]`, *optional*, defaults to (0, 0)) --
  `crops_coords_top_left` can be used to generate an image that appears to be "cropped" from the position
  `crops_coords_top_left` downwards. Favorable, well-centered images are usually achieved by setting
  `crops_coords_top_left` to (0, 0). Part of SDXL's micro-conditioning as explained in section 2.2 of
  [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).
- **target_size** (`tuple[int]`, *optional*, defaults to (1024, 1024)) --
  For most cases, `target_size` should be set to the desired height and width of the generated image. If
  not specified it will default to `(height, width)`. Part of SDXL's micro-conditioning as explained in
  section 2.2 of [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).
- **negative_original_size** (`tuple[int]`, *optional*, defaults to (1024, 1024)) --
  To negatively condition the generation process based on a specific image resolution. Part of SDXL's
  micro-conditioning as explained in section 2.2 of
  [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952). For more
  information, refer to this issue thread: https://github.com/huggingface/diffusers/issues/4208.
- **negative_crops_coords_top_left** (`tuple[int]`, *optional*, defaults to (0, 0)) --
  To negatively condition the generation process based on a specific crop coordinates. Part of SDXL's
  micro-conditioning as explained in section 2.2 of
  [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952). For more
  information, refer to this issue thread: https://github.com/huggingface/diffusers/issues/4208.
- **negative_target_size** (`tuple[int]`, *optional*, defaults to (1024, 1024)) --
  To negatively condition the generation process based on a target image resolution. It should be as same
  as the `target_size` for most cases. Part of SDXL's micro-conditioning as explained in section 2.2 of
  [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952). For more
  information, refer to this issue thread: https://github.com/huggingface/diffusers/issues/4208.
- **clip_skip** (`int`, *optional*) --
  Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that
  the output of the pre-final layer will be used for computing the prompt embeddings.
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`list`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.0[AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) or `tuple`If `return_dict` is `True`, [AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) is
returned, otherwise a `tuple` is returned where the first element is a list with the generated frames.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers.models import MotionAdapter
>>> from diffusers import AnimateDiffSDXLPipeline, DDIMScheduler
>>> from diffusers.utils import export_to_gif

>>> adapter = MotionAdapter.from_pretrained(
...     "a-r-r-o-w/animatediff-motion-adapter-sdxl-beta", torch_dtype=torch.float16
... )

>>> model_id = "stabilityai/stable-diffusion-xl-base-1.0"
>>> scheduler = DDIMScheduler.from_pretrained(
...     model_id,
...     subfolder="scheduler",
...     clip_sample=False,
...     timestep_spacing="linspace",
...     beta_schedule="linear",
...     steps_offset=1,
... )
>>> pipe = AnimateDiffSDXLPipeline.from_pretrained(
...     model_id,
...     motion_adapter=adapter,
...     scheduler=scheduler,
...     torch_dtype=torch.float16,
...     variant="fp16",
... ).to("cuda")

>>> # enable memory savings
>>> pipe.enable_vae_slicing()
>>> pipe.enable_vae_tiling()

>>> output = pipe(
...     prompt="a panda surfing in the ocean, realistic, high quality",
...     negative_prompt="low quality, worst quality",
...     num_inference_steps=20,
...     guidance_scale=8,
...     width=1024,
...     height=1024,
...     num_frames=16,
... )

>>> frames = output.frames[0]
>>> export_to_gif(frames, "animation.gif")
```

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : Frozen text-encoder. Stable Diffusion XL uses the text portion of [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

text_encoder_2 (` CLIPTextModelWithProjection`) : Second frozen text-encoder. Stable Diffusion XL uses the text and pool portion of [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModelWithProjection), specifically the [laion/CLIP-ViT-bigG-14-laion2B-39B-b160k](https://huggingface.co/laion/CLIP-ViT-bigG-14-laion2B-39B-b160k) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer_2 (`CLIPTokenizer`) : Second Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

unet ([UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the encoded image latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `unet` to denoise the encoded image latents. Can be one of [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), [LMSDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lms_discrete#diffusers.LMSDiscreteScheduler), or [PNDMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/pndm#diffusers.PNDMScheduler).

force_zeros_for_empty_prompt (`bool`, *optional*, defaults to `"True"`) : Whether the negative prompt embeddings shall be forced to always be set to 0. Also see the config of `stabilityai/stable-diffusion-xl-base-1-0`.

**Returns:**

`[AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) or `tuple``

If `return_dict` is `True`, [AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) is
returned, otherwise a `tuple` is returned where the first element is a list with the generated frames.
#### encode_prompt[[diffusers.AnimateDiffSDXLPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/animatediff/pipeline_animatediff_sdxl.py#L327)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is used in both text-encoders

device : (`torch.device`): torch device

num_videos_per_prompt (`int`) : number of images that should be generated per prompt

do_classifier_free_guidance (`bool`) : whether to use classifier free guidance or not

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

negative_prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `negative_prompt` is used in both text-encoders

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

pooled_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

negative_pooled_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled negative_prompt_embeds will be generated from `negative_prompt` input argument.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

clip_skip (`int`, *optional*) : Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that the output of the pre-final layer will be used for computing the prompt embeddings.
#### get_guidance_scale_embedding[[diffusers.AnimateDiffSDXLPipeline.get_guidance_scale_embedding]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/animatediff/pipeline_animatediff_sdxl.py#L802)

See https://github.com/google-research/vdm/blob/dc27b98a554f65cdc654b800da5aa1846545d41b/model_vdm.py#L298

**Parameters:**

w (`torch.Tensor`) : Generate embedding vectors with a specified guidance scale to subsequently enrich timestep embeddings.

embedding_dim (`int`, *optional*, defaults to 512) : Dimension of the embeddings to generate.

dtype (`torch.dtype`, *optional*, defaults to `torch.float32`) : Data type of the generated embeddings.

**Returns:**

``torch.Tensor``

Embedding vectors with shape `(len(w), embedding_dim)`.

## AnimateDiffVideoToVideoPipeline[[diffusers.AnimateDiffVideoToVideoPipeline]]

#### diffusers.AnimateDiffVideoToVideoPipeline[[diffusers.AnimateDiffVideoToVideoPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/animatediff/pipeline_animatediff_video2video.py#L181)

Pipeline for video-to-video generation.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

The pipeline also inherits the following loading methods:
- [load_textual_inversion()](/docs/diffusers/v0.39.0/en/api/loaders/textual_inversion#diffusers.loaders.TextualInversionLoaderMixin.load_textual_inversion) for loading textual inversion embeddings
- [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for loading LoRA weights
- [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for saving LoRA weights
- [load_ip_adapter()](/docs/diffusers/v0.39.0/en/api/loaders/ip_adapter#diffusers.loaders.IPAdapterMixin.load_ip_adapter) for loading IP Adapters

__call__diffusers.AnimateDiffVideoToVideoPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/animatediff/pipeline_animatediff_video2video.py#L744[{"name": "video", "val": ": list = None"}, {"name": "prompt", "val": ": str | list[str] | None = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "enforce_inference_steps", "val": ": bool = False"}, {"name": "timesteps", "val": ": list[int] | None = None"}, {"name": "sigmas", "val": ": list[float] | None = None"}, {"name": "guidance_scale", "val": ": float = 7.5"}, {"name": "strength", "val": ": float = 0.8"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "num_videos_per_prompt", "val": ": int | None = 1"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "ip_adapter_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "ip_adapter_image_embeds", "val": ": list[torch.Tensor] | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "clip_skip", "val": ": int | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "decode_chunk_size", "val": ": int = 16"}]- **video** (`list[PipelineImageInput]`) --
  The input video to condition the generation on. Must be a list of images/frames of the video.
- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide image generation. If not defined, you need to pass `prompt_embeds`.
- **height** (`int`, *optional*, defaults to `self.unet.config.sample_size * self.vae_scale_factor`) --
  The height in pixels of the generated video.
- **width** (`int`, *optional*, defaults to `self.unet.config.sample_size * self.vae_scale_factor`) --
  The width in pixels of the generated video.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality videos at the
  expense of slower inference.
- **enforce_inference_steps** (`bool`, *optional*, defaults to `False`) --
  Whether to enforce `num_inference_steps` denoising steps regardless of the `strength` parameter. When
  `False`, the effective number of inference steps is reduced according to `strength`.
- **timesteps** (`list[int]`, *optional*) --
  Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument
  in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is
  passed will be used. Must be in descending order.
- **sigmas** (`list[float]`, *optional*) --
  Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in
  their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed
  will be used.
- **strength** (`float`, *optional*, defaults to 0.8) --
  Higher strength leads to more differences between original video and generated video.
- **guidance_scale** (`float`, *optional*, defaults to 7.5) --
  A higher guidance scale value encourages the model to generate images closely linked to the text
  `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide what to not include in image generation. If not defined, you need to
  pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale 0[pipelines.animatediff.pipeline_output.AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) or `tuple`If `return_dict` is `True`, [pipelines.animatediff.pipeline_output.AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) is
returned, otherwise a `tuple` is returned where the first element is a list with the generated frames.

The call function to the pipeline for generation.

Examples:

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : Frozen text-encoder ([clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14)).

tokenizer (`CLIPTokenizer`) : A [CLIPTokenizer](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTokenizer) to tokenize text.

unet ([UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : A [UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel) used to create a UNetMotionModel to denoise the encoded video latents.

motion_adapter (`MotionAdapter`) : A `MotionAdapter` to be used in combination with `unet` to denoise the encoded video latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `unet` to denoise the encoded image latents. Can be one of [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), [LMSDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lms_discrete#diffusers.LMSDiscreteScheduler), or [PNDMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/pndm#diffusers.PNDMScheduler).

**Returns:**

`[pipelines.animatediff.pipeline_output.AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) or `tuple``

If `return_dict` is `True`, [pipelines.animatediff.pipeline_output.AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) is
returned, otherwise a `tuple` is returned where the first element is a list with the generated frames.
#### encode_prompt[[diffusers.AnimateDiffVideoToVideoPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/animatediff/pipeline_animatediff_video2video.py#L256)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

do_classifier_free_guidance (`bool`) : whether to use classifier free guidance or not

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

lora_scale (`float`, *optional*) : A LoRA scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

clip_skip (`int`, *optional*) : Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that the output of the pre-final layer will be used for computing the prompt embeddings.

## AnimateDiffVideoToVideoControlNetPipeline[[diffusers.AnimateDiffVideoToVideoControlNetPipeline]]

#### diffusers.AnimateDiffVideoToVideoControlNetPipeline[[diffusers.AnimateDiffVideoToVideoControlNetPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/animatediff/pipeline_animatediff_video2video_controlnet.py#L199)

Pipeline for video-to-video generation with ControlNet guidance.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

The pipeline also inherits the following loading methods:
- [load_textual_inversion()](/docs/diffusers/v0.39.0/en/api/loaders/textual_inversion#diffusers.loaders.TextualInversionLoaderMixin.load_textual_inversion) for loading textual inversion embeddings
- [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for loading LoRA weights
- [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for saving LoRA weights
- [load_ip_adapter()](/docs/diffusers/v0.39.0/en/api/loaders/ip_adapter#diffusers.loaders.IPAdapterMixin.load_ip_adapter) for loading IP Adapters

__call__diffusers.AnimateDiffVideoToVideoControlNetPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/animatediff/pipeline_animatediff_video2video_controlnet.py#L909[{"name": "video", "val": ": list = None"}, {"name": "prompt", "val": ": str | list[str] | None = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "enforce_inference_steps", "val": ": bool = False"}, {"name": "timesteps", "val": ": list[int] | None = None"}, {"name": "sigmas", "val": ": list[float] | None = None"}, {"name": "guidance_scale", "val": ": float = 7.5"}, {"name": "strength", "val": ": float = 0.8"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "num_videos_per_prompt", "val": ": int | None = 1"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "ip_adapter_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "ip_adapter_image_embeds", "val": ": list[torch.Tensor] | None = None"}, {"name": "conditioning_frames", "val": ": list[PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor]] | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "controlnet_conditioning_scale", "val": ": float | list[float] = 1.0"}, {"name": "guess_mode", "val": ": bool = False"}, {"name": "control_guidance_start", "val": ": float | list[float] = 0.0"}, {"name": "control_guidance_end", "val": ": float | list[float] = 1.0"}, {"name": "clip_skip", "val": ": int | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "decode_chunk_size", "val": ": int = 16"}]- **video** (`list[PipelineImageInput]`) --
  The input video to condition the generation on. Must be a list of images/frames of the video.
- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide image generation. If not defined, you need to pass `prompt_embeds`.
- **height** (`int`, *optional*, defaults to `self.unet.config.sample_size * self.vae_scale_factor`) --
  The height in pixels of the generated video.
- **width** (`int`, *optional*, defaults to `self.unet.config.sample_size * self.vae_scale_factor`) --
  The width in pixels of the generated video.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality videos at the
  expense of slower inference.
- **enforce_inference_steps** (`bool`, *optional*, defaults to `False`) --
  Whether to enforce `num_inference_steps` denoising steps regardless of the `strength` parameter. When
  `False`, the effective number of inference steps is reduced according to `strength`.
- **timesteps** (`list[int]`, *optional*) --
  Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument
  in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is
  passed will be used. Must be in descending order.
- **sigmas** (`list[float]`, *optional*) --
  Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in
  their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed
  will be used.
- **strength** (`float`, *optional*, defaults to 0.8) --
  Higher strength leads to more differences between original video and generated video.
- **guidance_scale** (`float`, *optional*, defaults to 7.5) --
  A higher guidance scale value encourages the model to generate images closely linked to the text
  `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide what to not include in image generation. If not defined, you need to
  pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale 0[pipelines.animatediff.pipeline_output.AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) or `tuple`If `return_dict` is `True`, [pipelines.animatediff.pipeline_output.AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) is
returned, otherwise a `tuple` is returned where the first element is a list with the generated frames.

The call function to the pipeline for generation.

Examples:

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : Frozen text-encoder ([clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14)).

tokenizer (`CLIPTokenizer`) : A [CLIPTokenizer](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTokenizer) to tokenize text.

unet ([UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : A [UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel) used to create a UNetMotionModel to denoise the encoded video latents.

motion_adapter (`MotionAdapter`) : A `MotionAdapter` to be used in combination with `unet` to denoise the encoded video latents.

controlnet ([ControlNetModel](/docs/diffusers/v0.39.0/en/api/models/controlnet#diffusers.ControlNetModel) or `list[ControlNetModel]` or `tuple[ControlNetModel]` or `MultiControlNetModel`) : Provides additional conditioning to the `unet` during the denoising process. If you set multiple ControlNets as a list, the outputs from each ControlNet are added together to create one combined additional conditioning.

scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `unet` to denoise the encoded image latents. Can be one of [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), [LMSDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lms_discrete#diffusers.LMSDiscreteScheduler), or [PNDMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/pndm#diffusers.PNDMScheduler).

**Returns:**

`[pipelines.animatediff.pipeline_output.AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) or `tuple``

If `return_dict` is `True`, [pipelines.animatediff.pipeline_output.AnimateDiffPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.pipelines.animatediff.AnimateDiffPipelineOutput) is
returned, otherwise a `tuple` is returned where the first element is a list with the generated frames.
#### encode_prompt[[diffusers.AnimateDiffVideoToVideoControlNetPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/animatediff/pipeline_animatediff_video2video_controlnet.py#L287)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

do_classifier_free_guidance (`bool`) : whether to use classifier free guidance or not

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

lora_scale (`float`, *optional*) : A LoRA scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

clip_skip (`int`, *optional*) : Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that the output of the pre-final layer will be used for computing the prompt embeddings.

## AnimateDiffPipelineOutput[[diffusers.pipelines.animatediff.AnimateDiffPipelineOutput]]

#### diffusers.pipelines.animatediff.AnimateDiffPipelineOutput[[diffusers.pipelines.animatediff.AnimateDiffPipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/animatediff/pipeline_output.py#L11)

Output class for AnimateDiff pipelines.

PIL image sequences of length `num_frames.` It can also be a NumPy array or Torch tensor of shape
`(batch_size, num_frames, channels, height, width)`

**Parameters:**

frames (`torch.Tensor`, `np.ndarray`, or list[list[PIL.Image.Image]]) : list of video outputs - It can be a nested list of length `batch_size,` with each sub-list containing denoised
