# ControlNet

[ControlNet](https://huggingface.co/papers/2302.05543) is an adapter that enables controllable generation such as generating an image of a cat in a *specific pose* or following the lines in a sketch of a *specific* cat. It works by adding a smaller network of "zero convolution" layers and progressively training these to avoid disrupting with the original model. The original model parameters are frozen to avoid retraining it.

A ControlNet is conditioned on extra visual information or "structural controls" (canny edge, depth maps, human pose, etc.) that can be combined with text prompts to generate images that are guided by the visual input.

> [!TIP]
> ControlNets are available to many models such as [Flux](../api/pipelines/controlnet_flux), [Hunyuan-DiT](../api/pipelines/controlnet_hunyuandit), [Stable Diffusion 3](../api/pipelines/controlnet_sd3), and more. The examples in this guide use Flux and Stable Diffusion XL.

Load a ControlNet conditioned on a specific control, such as canny edge, and pass it to the pipeline in [from_pretrained()](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline.from_pretrained).

Generate a canny image with [opencv-python](https://github.com/opencv/opencv-python).

```py
import cv2
import numpy as np
from PIL import Image
from diffusers.utils import load_image

original_image = load_image(
    "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/non-enhanced-prompt.png"
)

image = np.array(original_image)

low_threshold = 100
high_threshold = 200

image = cv2.Canny(image, low_threshold, high_threshold)
image = image[:, :, None]
image = np.concatenate([image, image, image], axis=2)
canny_image = Image.fromarray(image)
```

Pass the canny image to the pipeline. Use the `controlnet_conditioning_scale` parameter to determine how much weight to assign to the control.

```py
import torch
from diffusers.utils import load_image
from diffusers import FluxControlNetPipeline, FluxControlNetModel

controlnet = FluxControlNetModel.from_pretrained(
    "InstantX/FLUX.1-dev-Controlnet-Canny", dtype=torch.bfloat16
)
pipeline = FluxControlNetPipeline.from_pretrained(
    "black-forest-labs/FLUX.1-dev", controlnet=controlnet, dtype=torch.bfloat16
).to("cuda")

prompt = """
A photorealistic overhead image of a cat reclining sideways in a flamingo pool floatie holding a margarita. 
The cat is floating leisurely in the pool and completely relaxed and happy.
"""

pipeline(
    prompt, 
    control_image=canny_image,
    controlnet_conditioning_scale=0.5,
    num_inference_steps=50, 
    guidance_scale=3.5,
).images[0]
```

  
    
    original image
  
  
    
    canny image
  
  
    
    generated image
  

Generate a depth map with a depth estimation pipeline from Transformers.

```py
import torch
import numpy as np
from PIL import Image
from transformers import DPTImageProcessor, DPTForDepthEstimation
from diffusers import ControlNetModel, StableDiffusionXLControlNetImg2ImgPipeline, AutoencoderKL
from diffusers.utils import load_image

depth_estimator = DPTForDepthEstimation.from_pretrained("Intel/dpt-hybrid-midas").to("cuda")
feature_extractor = DPTImageProcessor.from_pretrained("Intel/dpt-hybrid-midas")

def get_depth_map(image):
    image = feature_extractor(images=image, return_tensors="pt").pixel_values.to("cuda")
    with torch.no_grad(), torch.autocast("cuda"):
        depth_map = depth_estimator(image).predicted_depth

    depth_map = torch.nn.functional.interpolate(
        depth_map.unsqueeze(1),
        size=(1024, 1024),
        mode="bicubic",
        align_corners=False,
    )
    depth_min = torch.amin(depth_map, dim=[1, 2, 3], keepdim=True)
    depth_max = torch.amax(depth_map, dim=[1, 2, 3], keepdim=True)
    depth_map = (depth_map - depth_min) / (depth_max - depth_min)
    image = torch.cat([depth_map] * 3, dim=1)
    image = image.permute(0, 2, 3, 1).cpu().numpy()[0]
    image = Image.fromarray((image * 255.0).clip(0, 255).astype(np.uint8))
    return image

depth_image = get_depth_map(image)
```

Pass the depth map to the pipeline. Use the `controlnet_conditioning_scale` parameter to determine how much weight to assign to the control.

```py
controlnet = ControlNetModel.from_pretrained(
    "diffusers/controlnet-depth-sdxl-1.0-small",
    dtype=torch.float16,
)
vae = AutoencoderKL.from_pretrained("madebyollin/sdxl-vae-fp16-fix", dtype=torch.float16)
pipeline = StableDiffusionXLControlNetImg2ImgPipeline.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0",
    controlnet=controlnet,
    vae=vae,
    dtype=torch.float16,
).to("cuda")

prompt = """
A photorealistic overhead image of a cat reclining sideways in a flamingo pool floatie holding a margarita. 
The cat is floating leisurely in the pool and completely relaxed and happy.
"""
image = load_image(
    "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/non-enhanced-prompt.png"
).resize((1024, 1024))
controlnet_conditioning_scale = 0.5 
pipeline(
    prompt,
    image=image,
    control_image=depth_image,
    controlnet_conditioning_scale=controlnet_conditioning_scale,
    strength=0.99,
    num_inference_steps=100,
).images[0]
```

  
    
    original image
  
  
    
    depth map
  
   
    
    generated image
  

Generate a mask image and convert it to a tensor to mark the pixels in the original image as masked if the corresponding pixel in the mask image is over a certain threshold.

```py
import cv2
import torch
import numpy as np
from PIL import Image
from diffusers.utils import load_image
from diffusers import StableDiffusionXLControlNetInpaintPipeline, ControlNetModel

init_image = load_image(
    "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/non-enhanced-prompt.png"
)
init_image = init_image.resize((1024, 1024))
mask_image = load_image(
    "/content/cat_mask.png"
)
mask_image = mask_image.resize((1024, 1024))

def make_canny_condition(image):
    image = np.array(image)
    image = cv2.Canny(image, 100, 200)
    image = image[:, :, None]
    image = np.concatenate([image, image, image], axis=2)
    image = Image.fromarray(image)
    return image

control_image = make_canny_condition(init_image)
```

Pass the mask and control image to the pipeline. Use the `controlnet_conditioning_scale` parameter to determine how much weight to assign to the control.

```py
controlnet = ControlNetModel.from_pretrained(
    "diffusers/controlnet-canny-sdxl-1.0", dtype=torch.float16
)
pipeline = StableDiffusionXLControlNetInpaintPipeline.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0", controlnet=controlnet, dtype=torch.float16
)
pipeline(
    "a cute and fluffy bunny rabbit",
    num_inference_steps=100,
    strength=0.99,
    controlnet_conditioning_scale=0.5,
    image=init_image,
    mask_image=mask_image,
    control_image=control_image,
).images[0]
```

  
    
    original image
  
  
    
    mask image
  
   
    
    generated image
  

## Multi-ControlNet

You can compose multiple ControlNet conditionings, such as canny image and a depth map, to create a *MultiControlNet*. For the best rersults, you should mask conditionings so they don't overlap and experiment with different `controlnet_conditioning_scale` parameters to adjust how much weight is assigned to each control input.

The example below composes a canny image and depth map.

Pass the ControlNets as a list to the pipeline and resize the images to the expected input size.

```py
import torch
from diffusers import StableDiffusionXLControlNetPipeline, ControlNetModel, AutoencoderKL

controlnets = [
    ControlNetModel.from_pretrained(
        "diffusers/controlnet-depth-sdxl-1.0-small", dtype=torch.float16
    ),
    ControlNetModel.from_pretrained(
        "diffusers/controlnet-canny-sdxl-1.0", dtype=torch.float16,
    ),
]

vae = AutoencoderKL.from_pretrained("madebyollin/sdxl-vae-fp16-fix", dtype=torch.float16)
pipeline = StableDiffusionXLControlNetPipeline.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0", controlnet=controlnets, vae=vae, dtype=torch.float16
).to("cuda")

prompt = """
a relaxed rabbit sitting on a striped towel next to a pool with a tropical drink nearby, 
bright sunny day, vacation scene, 35mm photograph, film, professional, 4k, highly detailed
"""
negative_prompt = "lowres, bad anatomy, worst quality, low quality, deformed, ugly"

images = [canny_image.resize((1024, 1024)), depth_image.resize((1024, 1024))]

pipeline(
    prompt,
    negative_prompt=negative_prompt,
    image=images,
    num_inference_steps=100,
    controlnet_conditioning_scale=[0.5, 0.5],
    strength=0.7,
).images[0]
```

  
    
    canny image
  
  
    
    depth map
  
   
    
    generated image
  

## guess_mode

[Guess mode](https://github.com/lllyasviel/ControlNet/discussions/188) generates an image from **only** the control input (canny edge, depth map, pose, etc.) and without guidance from a prompt. It adjusts the scale of the ControlNet's output residuals by a fixed ratio depending on block depth. The earlier `DownBlock` is only scaled by `0.1` and the `MidBlock` is fully scaled by `1.0`.

```py
import torch
from diffusers.utils import load_iamge
from diffusers import StableDiffusionXLControlNetPipeline, ControlNetModel

controlnet = ControlNetModel.from_pretrained(
  "diffusers/controlnet-canny-sdxl-1.0", dtype=torch.float16
)
pipeline = StableDiffusionXLControlNetPipeline.from_pretrained(
  "stabilityai/stable-diffusion-xl-base-1.0",
  controlnet=controlnet,
  dtype=torch.float16
).to("cuda")

canny_image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/canny-cat.png")
pipeline(
  "",
  image=canny_image,
  guess_mode=True
).images[0]
```

  
    
    canny image
  
  
    
    generated image
