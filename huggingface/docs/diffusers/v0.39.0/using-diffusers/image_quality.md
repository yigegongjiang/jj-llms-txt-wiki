# FreeU

[FreeU](https://hf.co/papers/2309.11497) improves image details by rebalancing the UNet's backbone and skip connection weights. The skip connections can cause the model to overlook some of the backbone semantics which may lead to unnatural image details in the generated image. This technique does not require any additional training and can be applied on the fly during inference for tasks like image-to-image and text-to-video.

Use the [enable_freeu()](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.StableDiffusionMixin.enable_freeu) method on your pipeline and configure the scaling factors for the backbone (`b1` and `b2`) and skip connections (`s1` and `s2`). The number after each scaling factor corresponds to the stage in the UNet where the factor is applied. Take a look at the [FreeU](https://github.com/ChenyangSi/FreeU#parameters) repository for reference hyperparameters for different models.

```py
import torch
from diffusers import DiffusionPipeline

pipeline = DiffusionPipeline.from_pretrained(
    "stable-diffusion-v1-5/stable-diffusion-v1-5", torch_dtype=torch.float16, safety_checker=None
).to("cuda")
pipeline.enable_freeu(s1=0.9, s2=0.2, b1=1.5, b2=1.6)
generator = torch.Generator(device="cpu").manual_seed(33)
prompt = ""
image = pipeline(prompt, generator=generator).images[0]
image
```

  
    
    FreeU disabled
  
  
    
    FreeU enabled
  

```py
import torch
from diffusers import DiffusionPipeline

pipeline = DiffusionPipeline.from_pretrained(
    "stabilityai/stable-diffusion-2-1", torch_dtype=torch.float16, safety_checker=None
).to("cuda")
pipeline.enable_freeu(s1=0.9, s2=0.2, b1=1.4, b2=1.6)
generator = torch.Generator(device="cpu").manual_seed(80)
prompt = "A squirrel eating a burger"
image = pipeline(prompt, generator=generator).images[0]
image
```

  
    
    FreeU disabled
  
  
    
    FreeU enabled
  

```py
import torch
from diffusers import DiffusionPipeline

pipeline = DiffusionPipeline.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0", torch_dtype=torch.float16,
).to("cuda")
pipeline.enable_freeu(s1=0.9, s2=0.2, b1=1.3, b2=1.4)
generator = torch.Generator(device="cpu").manual_seed(13)
prompt = "A squirrel eating a burger"
image = pipeline(prompt, generator=generator).images[0]
image
```

  
    
    FreeU disabled
  
  
    
    FreeU enabled
  

```py
import torch
from diffusers import DiffusionPipeline
from diffusers.utils import export_to_video

pipeline = DiffusionPipeline.from_pretrained(
    "damo-vilab/text-to-video-ms-1.7b", torch_dtype=torch.float16
).to("cuda")
# values come from https://github.com/lyn-rgb/FreeU_Diffusers#video-pipelines
pipeline.enable_freeu(b1=1.2, b2=1.4, s1=0.9, s2=0.2)
prompt = "Confident teddy bear surfer rides the wave in the tropics"
generator = torch.Generator(device="cpu").manual_seed(47)
video_frames = pipeline(prompt, generator=generator).frames[0]
export_to_video(video_frames, "teddy_bear.mp4", fps=10)
```

  
    
    FreeU disabled
  
  
    
    FreeU enabled
  

Call the [disable_freeu()](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.StableDiffusionMixin.disable_freeu) method to disable FreeU.

```py
pipeline.disable_freeu()
```
