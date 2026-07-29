# Latent Consistency Models

  

Latent Consistency Models (LCMs) were proposed in [Latent Consistency Models: Synthesizing High-Resolution Images with Few-Step Inference](https://huggingface.co/papers/2310.04378) by Simian Luo, Yiqin Tan, Longbo Huang, Jian Li, and Hang Zhao.

The abstract of the paper is as follows:

*Latent Diffusion models (LDMs) have achieved remarkable results in synthesizing high-resolution images. However, the iterative sampling process is computationally intensive and leads to slow generation. Inspired by Consistency Models (song et al.), we propose Latent Consistency Models (LCMs), enabling swift inference with minimal steps on any pre-trained LDMs, including Stable Diffusion (rombach et al). Viewing the guided reverse diffusion process as solving an augmented probability flow ODE (PF-ODE), LCMs are designed to directly predict the solution of such ODE in latent space, mitigating the need for numerous iterations and allowing rapid, high-fidelity sampling. Efficiently distilled from pre-trained classifier-free guided diffusion models, a high-quality 768 x 768 2~4-step LCM takes only 32 A100 GPU hours for training. Furthermore, we introduce Latent Consistency Fine-tuning (LCF), a novel method that is tailored for fine-tuning LCMs on customized image datasets. Evaluation on the LAION-5B-Aesthetics dataset demonstrates that LCMs achieve state-of-the-art text-to-image generation performance with few-step inference. Project Page: [this https URL](https://latent-consistency-models.github.io/).*

A demo for the [SimianLuo/LCM_Dreamshaper_v7](https://huggingface.co/SimianLuo/LCM_Dreamshaper_v7) checkpoint can be found [here](https://huggingface.co/spaces/SimianLuo/Latent_Consistency_Model).

The pipelines were contributed by [luosiallen](https://luosiallen.github.io/), [nagolinc](https://github.com/nagolinc), and [dg845](https://github.com/dg845).

> [!TIP]
> LCMs and LCM-LoRAs are available for Stable Diffusion v1.5, Stable Diffusion XL, and the SSD-1B model. You can find their checkpoints on the [Latent Consistency](https://hf.co/collections/latent-consistency/latent-consistency-models-weights-654ce61a95edd6dffccef6a8) Collections.

## Text-to-image

To use LCMs, you need to load the LCM checkpoint for your supported model into [UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel) and replace the scheduler with the [LCMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lcm#diffusers.LCMScheduler). Then you can use the pipeline as usual, and pass a text prompt to generate an image in just 4 steps.

A couple of notes to keep in mind when using LCMs are:

* Typically, batch size is doubled inside the pipeline for classifier-free guidance. But LCM applies guidance with guidance embeddings and doesn't need to double the batch size, which leads to faster inference. The downside is that negative prompts don't work with LCM because they don't have any effect on the denoising process.
* The ideal range for `guidance_scale` is [3., 13.] because that is what the UNet was trained with. However, disabling `guidance_scale` with a value of 1.0 is also effective in most cases.

```python
from diffusers import StableDiffusionXLPipeline, UNet2DConditionModel, LCMScheduler
import torch

unet = UNet2DConditionModel.from_pretrained(
    "latent-consistency/lcm-sdxl",
    torch_dtype=torch.float16,
    variant="fp16",
)
pipe = StableDiffusionXLPipeline.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0", unet=unet, torch_dtype=torch.float16, variant="fp16",
).to("cuda")
pipe.scheduler = LCMScheduler.from_config(pipe.scheduler.config)

prompt = "Self-portrait oil painting, a beautiful cyborg with golden hair, 8k"
generator = torch.manual_seed(0)
image = pipe(
    prompt=prompt, num_inference_steps=4, generator=generator, guidance_scale=8.0
).images[0]
image
```

    

To use LCM-LoRAs, you need to replace the scheduler with the [LCMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lcm#diffusers.LCMScheduler) and load the LCM-LoRA weights with the [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) method. Then you can use the pipeline as usual, and pass a text prompt to generate an image in just 4 steps.

A couple of notes to keep in mind when using LCM-LoRAs are:

* Typically, batch size is doubled inside the pipeline for classifier-free guidance. But LCM applies guidance with guidance embeddings and doesn't need to double the batch size, which leads to faster inference. The downside is that negative prompts don't work with LCM because they don't have any effect on the denoising process.
* You could use guidance with LCM-LoRAs, but it is very sensitive to high `guidance_scale` values and can lead to artifacts in the generated image. The best values we've found are between [1.0, 2.0].
* Replace [stabilityai/stable-diffusion-xl-base-1.0](https://hf.co/stabilityai/stable-diffusion-xl-base-1.0) with any finetuned model. For example, try using the [animagine-xl](https://huggingface.co/Linaqruf/animagine-xl) checkpoint to generate anime images with SDXL.

```py
import torch
from diffusers import DiffusionPipeline, LCMScheduler

pipe = DiffusionPipeline.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0",
    variant="fp16",
    torch_dtype=torch.float16
).to("cuda")
pipe.scheduler = LCMScheduler.from_config(pipe.scheduler.config)
pipe.load_lora_weights("latent-consistency/lcm-lora-sdxl")

prompt = "Self-portrait oil painting, a beautiful cyborg with golden hair, 8k"
generator = torch.manual_seed(42)
image = pipe(
    prompt=prompt, num_inference_steps=4, generator=generator, guidance_scale=1.0
).images[0]
image
```

    

## Image-to-image

To use LCMs for image-to-image, you need to load the LCM checkpoint for your supported model into [UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel) and replace the scheduler with the [LCMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lcm#diffusers.LCMScheduler). Then you can use the pipeline as usual, and pass a text prompt and initial image to generate an image in just 4 steps.

> [!TIP]
> Experiment with different values for `num_inference_steps`, `strength`, and `guidance_scale` to get the best results.

```python
import torch
from diffusers import AutoPipelineForImage2Image, UNet2DConditionModel, LCMScheduler
from diffusers.utils import load_image

unet = UNet2DConditionModel.from_pretrained(
    "SimianLuo/LCM_Dreamshaper_v7",
    subfolder="unet",
    torch_dtype=torch.float16,
)

pipe = AutoPipelineForImage2Image.from_pretrained(
    "Lykon/dreamshaper-7",
    unet=unet,
    torch_dtype=torch.float16,
    variant="fp16",
).to("cuda")
pipe.scheduler = LCMScheduler.from_config(pipe.scheduler.config)

init_image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/img2img-init.png")
prompt = "Astronauts in a jungle, cold color palette, muted colors, detailed, 8k"
generator = torch.manual_seed(0)
image = pipe(
    prompt,
    image=init_image,
    num_inference_steps=4,
    guidance_scale=7.5,
    strength=0.5,
    generator=generator
).images[0]
image
```

  
    
    initial image
  
  
    
    generated image
  

To use LCM-LoRAs for image-to-image, you need to replace the scheduler with the [LCMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lcm#diffusers.LCMScheduler) and load the LCM-LoRA weights with the [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) method. Then you can use the pipeline as usual, and pass a text prompt and initial image to generate an image in just 4 steps.

> [!TIP]
> Experiment with different values for `num_inference_steps`, `strength`, and `guidance_scale` to get the best results.

```py
import torch
from diffusers import AutoPipelineForImage2Image, LCMScheduler
from diffusers.utils import make_image_grid, load_image

pipe = AutoPipelineForImage2Image.from_pretrained(
    "Lykon/dreamshaper-7",
    torch_dtype=torch.float16,
    variant="fp16",
).to("cuda")

pipe.scheduler = LCMScheduler.from_config(pipe.scheduler.config)

pipe.load_lora_weights("latent-consistency/lcm-lora-sdv1-5")

init_image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/img2img-init.png")
prompt = "Astronauts in a jungle, cold color palette, muted colors, detailed, 8k"

generator = torch.manual_seed(0)
image = pipe(
    prompt,
    image=init_image,
    num_inference_steps=4,
    guidance_scale=1,
    strength=0.6,
    generator=generator
).images[0]
image
```

  
    
    initial image
  
  
    
    generated image
  

## Inpainting

To use LCM-LoRAs for inpainting, you need to replace the scheduler with the [LCMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lcm#diffusers.LCMScheduler) and load the LCM-LoRA weights with the [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) method. Then you can use the pipeline as usual, and pass a text prompt, initial image, and mask image to generate an image in just 4 steps.

```py
import torch
from diffusers import AutoPipelineForInpainting, LCMScheduler
from diffusers.utils import load_image, make_image_grid

pipe = AutoPipelineForInpainting.from_pretrained(
    "stable-diffusion-v1-5/stable-diffusion-inpainting",
    torch_dtype=torch.float16,
    variant="fp16",
).to("cuda")

pipe.scheduler = LCMScheduler.from_config(pipe.scheduler.config)

pipe.load_lora_weights("latent-consistency/lcm-lora-sdv1-5")

init_image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/inpaint.png")
mask_image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/inpaint_mask.png")

prompt = "concept art digital painting of an elven castle, inspired by lord of the rings, highly detailed, 8k"
generator = torch.manual_seed(0)
image = pipe(
    prompt=prompt,
    image=init_image,
    mask_image=mask_image,
    generator=generator,
    num_inference_steps=4,
    guidance_scale=4,
).images[0]
image
```

  
    
    initial image
  
  
    
    generated image
  

## Adapters

LCMs are compatible with adapters like LoRA, ControlNet, T2I-Adapter, and AnimateDiff. You can bring the speed of LCMs to these adapters to generate images in a certain style or condition the model on another input like a canny image.

### LoRA

[LoRA](../../tutorials/using_peft_for_inference) adapters can be rapidly finetuned to learn a new style from just a few images and plugged into a pretrained model to generate images in that style.

Load the LCM checkpoint for your supported model into [UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel) and replace the scheduler with the [LCMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lcm#diffusers.LCMScheduler). Then you can use the [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) method to load the LoRA weights into the LCM and generate a styled image in a few steps.

```python
from diffusers import StableDiffusionXLPipeline, UNet2DConditionModel, LCMScheduler
import torch

unet = UNet2DConditionModel.from_pretrained(
    "latent-consistency/lcm-sdxl",
    torch_dtype=torch.float16,
    variant="fp16",
)
pipe = StableDiffusionXLPipeline.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0", unet=unet, torch_dtype=torch.float16, variant="fp16",
).to("cuda")
pipe.scheduler = LCMScheduler.from_config(pipe.scheduler.config)
pipe.load_lora_weights("TheLastBen/Papercut_SDXL", weight_name="papercut.safetensors", adapter_name="papercut")

prompt = "papercut, a cute fox"
generator = torch.manual_seed(0)
image = pipe(
    prompt=prompt, num_inference_steps=4, generator=generator, guidance_scale=8.0
).images[0]
image
```

    

Replace the scheduler with the [LCMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lcm#diffusers.LCMScheduler). Then you can use the [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) method to load the LCM-LoRA weights and the style LoRA you want to use. Combine both LoRA adapters with the `~loaders.UNet2DConditionLoadersMixin.set_adapters` method and generate a styled image in a few steps.

```py
import torch
from diffusers import DiffusionPipeline, LCMScheduler

pipe = DiffusionPipeline.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0",
    variant="fp16",
    torch_dtype=torch.float16
).to("cuda")

pipe.scheduler = LCMScheduler.from_config(pipe.scheduler.config)

pipe.load_lora_weights("latent-consistency/lcm-lora-sdxl", adapter_name="lcm")
pipe.load_lora_weights("TheLastBen/Papercut_SDXL", weight_name="papercut.safetensors", adapter_name="papercut")

pipe.set_adapters(["lcm", "papercut"], adapter_weights=[1.0, 0.8])

prompt = "papercut, a cute fox"
generator = torch.manual_seed(0)
image = pipe(prompt, num_inference_steps=4, guidance_scale=1, generator=generator).images[0]
image
```

    

### ControlNet

[ControlNet](./controlnet) are adapters that can be trained on a variety of inputs like canny edge, pose estimation, or depth. The ControlNet can be inserted into the pipeline to provide additional conditioning and control to the model for more accurate generation.

You can find additional ControlNet models trained on other inputs in [lllyasviel's](https://hf.co/lllyasviel) repository.

Load a ControlNet model trained on canny images and pass it to the [ControlNetModel](/docs/diffusers/v0.39.0/en/api/models/controlnet#diffusers.ControlNetModel). Then you can load a LCM model into [StableDiffusionControlNetPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/controlnet#diffusers.StableDiffusionControlNetPipeline) and replace the scheduler with the [LCMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lcm#diffusers.LCMScheduler). Now pass the canny image to the pipeline and generate an image.

> [!TIP]
> Experiment with different values for `num_inference_steps`, `controlnet_conditioning_scale`, `cross_attention_kwargs`, and `guidance_scale` to get the best results.

```python
import torch
import cv2
import numpy as np
from PIL import Image

from diffusers import StableDiffusionControlNetPipeline, ControlNetModel, LCMScheduler
from diffusers.utils import load_image, make_image_grid

image = load_image(
    "https://hf.co/datasets/huggingface/documentation-images/resolve/main/diffusers/input_image_vermeer.png"
).resize((512, 512))

image = np.array(image)

low_threshold = 100
high_threshold = 200

image = cv2.Canny(image, low_threshold, high_threshold)
image = image[:, :, None]
image = np.concatenate([image, image, image], axis=2)
canny_image = Image.fromarray(image)

controlnet = ControlNetModel.from_pretrained("lllyasviel/sd-controlnet-canny", torch_dtype=torch.float16)
pipe = StableDiffusionControlNetPipeline.from_pretrained(
    "SimianLuo/LCM_Dreamshaper_v7",
    controlnet=controlnet,
    torch_dtype=torch.float16,
    safety_checker=None,
).to("cuda")
pipe.scheduler = LCMScheduler.from_config(pipe.scheduler.config)

generator = torch.manual_seed(0)
image = pipe(
    "the mona lisa",
    image=canny_image,
    num_inference_steps=4,
    generator=generator,
).images[0]
make_image_grid([canny_image, image], rows=1, cols=2)
```

    

Load a ControlNet model trained on canny images and pass it to the [ControlNetModel](/docs/diffusers/v0.39.0/en/api/models/controlnet#diffusers.ControlNetModel). Then you can load a Stable Diffusion v1.5 model into [StableDiffusionControlNetPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/controlnet#diffusers.StableDiffusionControlNetPipeline) and replace the scheduler with the [LCMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lcm#diffusers.LCMScheduler). Use the [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) method to load the LCM-LoRA weights, and pass the canny image to the pipeline and generate an image.

> [!TIP]
> Experiment with different values for `num_inference_steps`, `controlnet_conditioning_scale`, `cross_attention_kwargs`, and `guidance_scale` to get the best results.

```py
import torch
import cv2
import numpy as np
from PIL import Image

from diffusers import StableDiffusionControlNetPipeline, ControlNetModel, LCMScheduler
from diffusers.utils import load_image

image = load_image(
    "https://hf.co/datasets/huggingface/documentation-images/resolve/main/diffusers/input_image_vermeer.png"
).resize((512, 512))

image = np.array(image)

low_threshold = 100
high_threshold = 200

image = cv2.Canny(image, low_threshold, high_threshold)
image = image[:, :, None]
image = np.concatenate([image, image, image], axis=2)
canny_image = Image.fromarray(image)

controlnet = ControlNetModel.from_pretrained("lllyasviel/sd-controlnet-canny", torch_dtype=torch.float16)
pipe = StableDiffusionControlNetPipeline.from_pretrained(
    "stable-diffusion-v1-5/stable-diffusion-v1-5",
    controlnet=controlnet,
    torch_dtype=torch.float16,
    safety_checker=None,
    variant="fp16"
).to("cuda")

pipe.scheduler = LCMScheduler.from_config(pipe.scheduler.config)

pipe.load_lora_weights("latent-consistency/lcm-lora-sdv1-5")

generator = torch.manual_seed(0)
image = pipe(
    "the mona lisa",
    image=canny_image,
    num_inference_steps=4,
    guidance_scale=1.5,
    controlnet_conditioning_scale=0.8,
    cross_attention_kwargs={"scale": 1},
    generator=generator,
).images[0]
image
```

    

### T2I-Adapter

[T2I-Adapter](../../using-diffusers/t2i_adapter) is an even more lightweight adapter than ControlNet, that provides an additional input to condition a pretrained model with. It is faster than ControlNet but the results may be slightly worse.

You can find additional T2I-Adapter checkpoints trained on other inputs in [TencentArc's](https://hf.co/TencentARC) repository.

Load a T2IAdapter trained on canny images and pass it to the [StableDiffusionXLAdapterPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/adapter#diffusers.StableDiffusionXLAdapterPipeline). Then load a LCM checkpoint into [UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel) and replace the scheduler with the [LCMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lcm#diffusers.LCMScheduler). Now pass the canny image to the pipeline and generate an image.

```python
import torch
import cv2
import numpy as np
from PIL import Image

from diffusers import StableDiffusionXLAdapterPipeline, UNet2DConditionModel, T2IAdapter, LCMScheduler
from diffusers.utils import load_image, make_image_grid

# detect the canny map in low resolution to avoid high-frequency details
image = load_image(
    "https://hf.co/datasets/huggingface/documentation-images/resolve/main/diffusers/input_image_vermeer.png"
).resize((384, 384))

image = np.array(image)

low_threshold = 100
high_threshold = 200

image = cv2.Canny(image, low_threshold, high_threshold)
image = image[:, :, None]
image = np.concatenate([image, image, image], axis=2)
canny_image = Image.fromarray(image).resize((1024, 1216))

adapter = T2IAdapter.from_pretrained("TencentARC/t2i-adapter-canny-sdxl-1.0", torch_dtype=torch.float16, variant="fp16").to("cuda")

unet = UNet2DConditionModel.from_pretrained(
    "latent-consistency/lcm-sdxl",
    torch_dtype=torch.float16,
    variant="fp16",
)
pipe = StableDiffusionXLAdapterPipeline.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0",
    unet=unet,
    adapter=adapter,
    torch_dtype=torch.float16,
    variant="fp16",
).to("cuda")

pipe.scheduler = LCMScheduler.from_config(pipe.scheduler.config)

prompt = "the mona lisa, 4k picture, high quality"
negative_prompt = "extra digit, fewer digits, cropped, worst quality, low quality, glitch, deformed, mutated, ugly, disfigured"

generator = torch.manual_seed(0)
image = pipe(
    prompt=prompt,
    negative_prompt=negative_prompt,
    image=canny_image,
    num_inference_steps=4,
    guidance_scale=5,
    adapter_conditioning_scale=0.8,
    adapter_conditioning_factor=1,
    generator=generator,
).images[0]
```

    

Load a T2IAdapter trained on canny images and pass it to the [StableDiffusionXLAdapterPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/adapter#diffusers.StableDiffusionXLAdapterPipeline). Replace the scheduler with the [LCMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lcm#diffusers.LCMScheduler), and use the [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) method to load the LCM-LoRA weights. Pass the canny image to the pipeline and generate an image.

```py
import torch
import cv2
import numpy as np
from PIL import Image

from diffusers import StableDiffusionXLAdapterPipeline, UNet2DConditionModel, T2IAdapter, LCMScheduler
from diffusers.utils import load_image, make_image_grid

# detect the canny map in low resolution to avoid high-frequency details
image = load_image(
    "https://hf.co/datasets/huggingface/documentation-images/resolve/main/diffusers/input_image_vermeer.png"
).resize((384, 384))

image = np.array(image)

low_threshold = 100
high_threshold = 200

image = cv2.Canny(image, low_threshold, high_threshold)
image = image[:, :, None]
image = np.concatenate([image, image, image], axis=2)
canny_image = Image.fromarray(image).resize((1024, 1024))

adapter = T2IAdapter.from_pretrained("TencentARC/t2i-adapter-canny-sdxl-1.0", torch_dtype=torch.float16, variant="fp16").to("cuda")

pipe = StableDiffusionXLAdapterPipeline.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0",
    adapter=adapter,
    torch_dtype=torch.float16,
    variant="fp16",
).to("cuda")

pipe.scheduler = LCMScheduler.from_config(pipe.scheduler.config)

pipe.load_lora_weights("latent-consistency/lcm-lora-sdxl")

prompt = "the mona lisa, 4k picture, high quality"
negative_prompt = "extra digit, fewer digits, cropped, worst quality, low quality, glitch, deformed, mutated, ugly, disfigured"

generator = torch.manual_seed(0)
image = pipe(
    prompt=prompt,
    negative_prompt=negative_prompt,
    image=canny_image,
    num_inference_steps=4,
    guidance_scale=1.5,
    adapter_conditioning_scale=0.8,
    adapter_conditioning_factor=1,
    generator=generator,
).images[0]
```

    

### AnimateDiff

[AnimateDiff](./animatediff) is an adapter that adds motion to an image. It can be used with most Stable Diffusion models, effectively turning them into "video generation" models. Generating good results with a video model usually requires generating multiple frames (16-24), which can be very slow with a regular Stable Diffusion model. LCM-LoRA can speed up this process by only taking 4-8 steps for each frame.

Load a [AnimateDiffPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/animatediff#diffusers.AnimateDiffPipeline) and pass a `MotionAdapter` to it. Then replace the scheduler with the [LCMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lcm#diffusers.LCMScheduler), and combine both LoRA adapters with the `~loaders.UNet2DConditionLoadersMixin.set_adapters` method. Now you can pass a prompt to the pipeline and generate an animated image.

```py
import torch
from diffusers import MotionAdapter, AnimateDiffPipeline, DDIMScheduler, LCMScheduler
from diffusers.utils import export_to_gif

adapter = MotionAdapter.from_pretrained("guoyww/animatediff-motion-adapter-v1-5")
pipe = AnimateDiffPipeline.from_pretrained(
    "frankjoshua/toonyou_beta6",
    motion_adapter=adapter,
).to("cuda")

# set scheduler
pipe.scheduler = LCMScheduler.from_config(pipe.scheduler.config)

# load LCM-LoRA
pipe.load_lora_weights("latent-consistency/lcm-lora-sdv1-5", adapter_name="lcm")
pipe.load_lora_weights("guoyww/animatediff-motion-lora-zoom-in", weight_name="diffusion_pytorch_model.safetensors", adapter_name="motion-lora")

pipe.set_adapters(["lcm", "motion-lora"], adapter_weights=[0.55, 1.2])

prompt = "best quality, masterpiece, 1girl, looking at viewer, blurry background, upper body, contemporary, dress"
generator = torch.manual_seed(0)
frames = pipe(
    prompt=prompt,
    num_inference_steps=5,
    guidance_scale=1.25,
    cross_attention_kwargs={"scale": 1},
    num_frames=24,
    generator=generator
).frames[0]
export_to_gif(frames, "animation.gif")
```

    

## LatentConsistencyModelPipeline[[diffusers.LatentConsistencyModelPipeline]]

#### diffusers.LatentConsistencyModelPipeline[[diffusers.LatentConsistencyModelPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/latent_consistency_models/pipeline_latent_consistency_text2img.py#L133)

Pipeline for text-to-image generation using a latent consistency model.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

The pipeline also inherits the following loading methods:
- [load_textual_inversion()](/docs/diffusers/v0.39.0/en/api/loaders/textual_inversion#diffusers.loaders.TextualInversionLoaderMixin.load_textual_inversion) for loading textual inversion embeddings
- [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for loading LoRA weights
- [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for saving LoRA weights
- [from_single_file()](/docs/diffusers/v0.39.0/en/api/loaders/single_file#diffusers.loaders.FromSingleFileMixin.from_single_file) for loading `.ckpt` files
- [load_ip_adapter()](/docs/diffusers/v0.39.0/en/api/loaders/ip_adapter#diffusers.loaders.IPAdapterMixin.load_ip_adapter) for loading IP Adapters

__call__diffusers.LatentConsistencyModelPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/latent_consistency_models/pipeline_latent_consistency_text2img.py#L640[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 4"}, {"name": "original_inference_steps", "val": ": int = None"}, {"name": "timesteps", "val": ": list = None"}, {"name": "guidance_scale", "val": ": float = 8.5"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "ip_adapter_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "ip_adapter_image_embeds", "val": ": list[torch.Tensor] | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "clip_skip", "val": ": int | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "**kwargs", "val": ""}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide image generation. If not defined, you need to pass `prompt_embeds`.
- **height** (`int`, *optional*, defaults to `self.unet.config.sample_size * self.vae_scale_factor`) --
  The height in pixels of the generated image.
- **width** (`int`, *optional*, defaults to `self.unet.config.sample_size * self.vae_scale_factor`) --
  The width in pixels of the generated image.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **original_inference_steps** (`int`, *optional*) --
  The original number of inference steps use to generate a linearly-spaced timestep schedule, from which
  we will draw `num_inference_steps` evenly spaced timesteps from as our final timestep schedule,
  following the Skipping-Step method in the paper (see Section 4.3). If not set this will default to the
  scheduler's `original_inference_steps` attribute.
- **timesteps** (`list[int]`, *optional*) --
  Custom timesteps to use for the denoising process. If not defined, equal spaced `num_inference_steps`
  timesteps on the original LCM training/distillation timestep schedule are used. Must be in descending
  order.
- **guidance_scale** (`float`, *optional*, defaults to 7.5) --
  A higher guidance scale value encourages the model to generate images closely linked to the text
  `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.
  Note that the original latent consistency models paper uses a different CFG formulation where the
  guidance scales are decreased by 1 (so in the paper formulation CFG is enabled when `guidance_scale >
  0`).
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
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
- **ip_adapter_image** -- (`PipelineImageInput`, *optional*):
  Optional image input to work with IP Adapters.
- **ip_adapter_image_embeds** (`list[torch.Tensor]`, *optional*) --
  Pre-generated image embeddings for IP-Adapter. It should be a list of length same as number of
  IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. It should
  contain the negative image embedding if `do_classifier_free_guidance` is set to `True`. If not
  provided, embeddings are computed from the `ip_adapter_image` input argument.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generated image. Choose between `PIL.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a [StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) instead of a
  plain tuple.
- **cross_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined in
  [`self.processor`](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
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
  `._callback_tensor_inputs` attribute of your pipeline class.0[StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) or `tuple`If `return_dict` is `True`, [StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated images and the
second element is a list of `bool`s indicating whether the corresponding generated image contains
"not-safe-for-work" (nsfw) content.

The call function to the pipeline for generation.

Examples:
```py
>>> from diffusers import DiffusionPipeline
>>> import torch

>>> pipe = DiffusionPipeline.from_pretrained("SimianLuo/LCM_Dreamshaper_v7")
>>> # To save GPU memory, torch.float16 can be used, but it may compromise image quality.
>>> pipe.to(torch_device="cuda", torch_dtype=torch.float32)

>>> prompt = "Self-portrait oil painting, a beautiful cyborg with golden hair, 8k"

>>> # Can be set to 1~50 steps. LCM support fast inference even >> num_inference_steps = 4
>>> images = pipe(prompt=prompt, num_inference_steps=num_inference_steps, guidance_scale=8.0).images
>>> images[0].save("image.png")
```

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) model to encode and decode images to and from latent representations.

text_encoder ([CLIPTextModel](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTextModel)) : Frozen text-encoder ([clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14)).

tokenizer ([CLIPTokenizer](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTokenizer)) : A `CLIPTokenizer` to tokenize text.

unet ([UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : A `UNet2DConditionModel` to denoise the encoded image latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `unet` to denoise the encoded image latents. Currently only supports [LCMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lcm#diffusers.LCMScheduler).

safety_checker (`StableDiffusionSafetyChecker`) : Classification module that estimates whether generated images could be considered offensive or harmful. Please refer to the [model card](https://huggingface.co/stable-diffusion-v1-5/stable-diffusion-v1-5) for more details about a model's potential harms.

feature_extractor ([CLIPImageProcessor](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPImageProcessor)) : A `CLIPImageProcessor` to extract features from generated images; used as inputs to the `safety_checker`.

requires_safety_checker (`bool`, *optional*, defaults to `True`) : Whether the pipeline requires a safety checker component.

**Returns:**

`[StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) or `tuple``

If `return_dict` is `True`, [StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated images and the
second element is a list of `bool`s indicating whether the corresponding generated image contains
"not-safe-for-work" (nsfw) content.
#### enable_freeu[[diffusers.LatentConsistencyModelPipeline.enable_freeu]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2320)

Enables the FreeU mechanism as in https://huggingface.co/papers/2309.11497.

The suffixes after the scaling factors represent the stages where they are being applied.

Please refer to the [official repository](https://github.com/ChenyangSi/FreeU) for combinations of the values
that are known to work well for different pipelines such as Stable Diffusion v1, v2, and Stable Diffusion XL.

**Parameters:**

s1 (`float`) : Scaling factor for stage 1 to attenuate the contributions of the skip features. This is done to mitigate "oversmoothing effect" in the enhanced denoising process.

s2 (`float`) : Scaling factor for stage 2 to attenuate the contributions of the skip features. This is done to mitigate "oversmoothing effect" in the enhanced denoising process.

b1 (`float`) : Scaling factor for stage 1 to amplify the contributions of backbone features.

b2 (`float`) : Scaling factor for stage 2 to amplify the contributions of backbone features.
#### disable_freeu[[diffusers.LatentConsistencyModelPipeline.disable_freeu]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2342)

Disables the FreeU mechanism if enabled.
#### enable_vae_slicing[[diffusers.LatentConsistencyModelPipeline.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2267)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### disable_vae_slicing[[diffusers.LatentConsistencyModelPipeline.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2280)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_vae_tiling[[diffusers.LatentConsistencyModelPipeline.enable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2293)

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.
#### disable_vae_tiling[[diffusers.LatentConsistencyModelPipeline.disable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2307)

Disable tiled VAE decoding. If `enable_vae_tiling` was previously enabled, this method will go back to
computing decoding in one step.
#### encode_prompt[[diffusers.LatentConsistencyModelPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/latent_consistency_models/pipeline_latent_consistency_text2img.py#L226)

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
#### get_guidance_scale_embedding[[diffusers.LatentConsistencyModelPipeline.get_guidance_scale_embedding]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/latent_consistency_models/pipeline_latent_consistency_text2img.py#L517)

See https://github.com/google-research/vdm/blob/dc27b98a554f65cdc654b800da5aa1846545d41b/model_vdm.py#L298

**Parameters:**

w (`torch.Tensor`) : Generate embedding vectors with a specified guidance scale to subsequently enrich timestep embeddings.

embedding_dim (`int`, *optional*, defaults to 512) : Dimension of the embeddings to generate.

dtype (`torch.dtype`, *optional*, defaults to `torch.float32`) : Data type of the generated embeddings.

**Returns:**

``torch.Tensor``

Embedding vectors with shape `(len(w), embedding_dim)`.

## LatentConsistencyModelImg2ImgPipeline[[diffusers.LatentConsistencyModelImg2ImgPipeline]]

#### diffusers.LatentConsistencyModelImg2ImgPipeline[[diffusers.LatentConsistencyModelImg2ImgPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/latent_consistency_models/pipeline_latent_consistency_img2img.py#L154)

Pipeline for image-to-image generation using a latent consistency model.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

The pipeline also inherits the following loading methods:
- [load_textual_inversion()](/docs/diffusers/v0.39.0/en/api/loaders/textual_inversion#diffusers.loaders.TextualInversionLoaderMixin.load_textual_inversion) for loading textual inversion embeddings
- [load_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for loading LoRA weights
- [save_lora_weights()](/docs/diffusers/v0.39.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for saving LoRA weights
- [from_single_file()](/docs/diffusers/v0.39.0/en/api/loaders/single_file#diffusers.loaders.FromSingleFileMixin.from_single_file) for loading `.ckpt` files
- [load_ip_adapter()](/docs/diffusers/v0.39.0/en/api/loaders/ip_adapter#diffusers.loaders.IPAdapterMixin.load_ip_adapter) for loading IP Adapters

__call__diffusers.LatentConsistencyModelImg2ImgPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/latent_consistency_models/pipeline_latent_consistency_img2img.py#L709[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] = None"}, {"name": "num_inference_steps", "val": ": int = 4"}, {"name": "strength", "val": ": float = 0.8"}, {"name": "original_inference_steps", "val": ": int = None"}, {"name": "timesteps", "val": ": list = None"}, {"name": "guidance_scale", "val": ": float = 8.5"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "ip_adapter_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "ip_adapter_image_embeds", "val": ": list[torch.Tensor] | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "clip_skip", "val": ": int | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "**kwargs", "val": ""}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide image generation. If not defined, you need to pass `prompt_embeds`.
- **image** (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) --
  `Image` or tensor representing an image batch to be used as the starting point. Can also accept image
  latents as `image`, but if passing latents directly it is not encoded again.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **strength** (`float`, *optional*, defaults to 0.8) --
  Indicates extent to transform the reference `image`. Must be between 0 and 1. `image` is used as a
  starting point and more noise is added the higher the `strength`. The number of denoising steps depends
  on the amount of noise initially added. When `strength` is 1, added noise is maximum and the denoising
  process runs for the full number of iterations specified in `num_inference_steps`. A value of 1
  essentially ignores `image`.
- **original_inference_steps** (`int`, *optional*) --
  The original number of inference steps use to generate a linearly-spaced timestep schedule, from which
  we will draw `num_inference_steps` evenly spaced timesteps from as our final timestep schedule,
  following the Skipping-Step method in the paper (see Section 4.3). If not set this will default to the
  scheduler's `original_inference_steps` attribute.
- **timesteps** (`list[int]`, *optional*) --
  Custom timesteps to use for the denoising process. If not defined, equal spaced `num_inference_steps`
  timesteps on the original LCM training/distillation timestep schedule are used. Must be in descending
  order.
- **guidance_scale** (`float`, *optional*, defaults to 7.5) --
  A higher guidance scale value encourages the model to generate images closely linked to the text
  `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.
  Note that the original latent consistency models paper uses a different CFG formulation where the
  guidance scales are decreased by 1 (so in the paper formulation CFG is enabled when `guidance_scale >
  0`).
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
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
- **ip_adapter_image** -- (`PipelineImageInput`, *optional*):
  Optional image input to work with IP Adapters.
- **ip_adapter_image_embeds** (`list[torch.Tensor]`, *optional*) --
  Pre-generated image embeddings for IP-Adapter. It should be a list of length same as number of
  IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. It should
  contain the negative image embedding if `do_classifier_free_guidance` is set to `True`. If not
  provided, embeddings are computed from the `ip_adapter_image` input argument.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generated image. Choose between `PIL.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a [StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) instead of a
  plain tuple.
- **cross_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined in
  [`self.processor`](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
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
  `._callback_tensor_inputs` attribute of your pipeline class.0[StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) or `tuple`If `return_dict` is `True`, [StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated images and the
second element is a list of `bool`s indicating whether the corresponding generated image contains
"not-safe-for-work" (nsfw) content.

The call function to the pipeline for generation.

Examples:
```py
>>> from diffusers import AutoPipelineForImage2Image
>>> import torch
>>> import PIL

>>> pipe = AutoPipelineForImage2Image.from_pretrained("SimianLuo/LCM_Dreamshaper_v7")
>>> # To save GPU memory, torch.float16 can be used, but it may compromise image quality.
>>> pipe.to(torch_device="cuda", torch_dtype=torch.float32)

>>> prompt = "High altitude snowy mountains"
>>> image = PIL.Image.open("./snowy_mountains.png")

>>> # Can be set to 1~50 steps. LCM support fast inference even >> num_inference_steps = 4
>>> images = pipe(
...     prompt=prompt, image=image, num_inference_steps=num_inference_steps, guidance_scale=8.0
... ).images

>>> images[0].save("image.png")
```

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) model to encode and decode images to and from latent representations.

text_encoder ([CLIPTextModel](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTextModel)) : Frozen text-encoder ([clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14)).

tokenizer ([CLIPTokenizer](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTokenizer)) : A `CLIPTokenizer` to tokenize text.

unet ([UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : A `UNet2DConditionModel` to denoise the encoded image latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `unet` to denoise the encoded image latents. Currently only supports [LCMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lcm#diffusers.LCMScheduler).

safety_checker (`StableDiffusionSafetyChecker`) : Classification module that estimates whether generated images could be considered offensive or harmful. Please refer to the [model card](https://huggingface.co/stable-diffusion-v1-5/stable-diffusion-v1-5) for more details about a model's potential harms.

feature_extractor ([CLIPImageProcessor](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPImageProcessor)) : A `CLIPImageProcessor` to extract features from generated images; used as inputs to the `safety_checker`.

requires_safety_checker (`bool`, *optional*, defaults to `True`) : Whether the pipeline requires a safety checker component.

**Returns:**

`[StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) or `tuple``

If `return_dict` is `True`, [StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated images and the
second element is a list of `bool`s indicating whether the corresponding generated image contains
"not-safe-for-work" (nsfw) content.
#### enable_freeu[[diffusers.LatentConsistencyModelImg2ImgPipeline.enable_freeu]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2320)

Enables the FreeU mechanism as in https://huggingface.co/papers/2309.11497.

The suffixes after the scaling factors represent the stages where they are being applied.

Please refer to the [official repository](https://github.com/ChenyangSi/FreeU) for combinations of the values
that are known to work well for different pipelines such as Stable Diffusion v1, v2, and Stable Diffusion XL.

**Parameters:**

s1 (`float`) : Scaling factor for stage 1 to attenuate the contributions of the skip features. This is done to mitigate "oversmoothing effect" in the enhanced denoising process.

s2 (`float`) : Scaling factor for stage 2 to attenuate the contributions of the skip features. This is done to mitigate "oversmoothing effect" in the enhanced denoising process.

b1 (`float`) : Scaling factor for stage 1 to amplify the contributions of backbone features.

b2 (`float`) : Scaling factor for stage 2 to amplify the contributions of backbone features.
#### disable_freeu[[diffusers.LatentConsistencyModelImg2ImgPipeline.disable_freeu]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2342)

Disables the FreeU mechanism if enabled.
#### enable_vae_slicing[[diffusers.LatentConsistencyModelImg2ImgPipeline.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2267)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### disable_vae_slicing[[diffusers.LatentConsistencyModelImg2ImgPipeline.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2280)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_vae_tiling[[diffusers.LatentConsistencyModelImg2ImgPipeline.enable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2293)

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.
#### disable_vae_tiling[[diffusers.LatentConsistencyModelImg2ImgPipeline.disable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L2307)

Disable tiled VAE decoding. If `enable_vae_tiling` was previously enabled, this method will go back to
computing decoding in one step.
#### encode_prompt[[diffusers.LatentConsistencyModelImg2ImgPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/latent_consistency_models/pipeline_latent_consistency_img2img.py#L241)

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
#### get_guidance_scale_embedding[[diffusers.LatentConsistencyModelImg2ImgPipeline.get_guidance_scale_embedding]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/latent_consistency_models/pipeline_latent_consistency_img2img.py#L576)

See https://github.com/google-research/vdm/blob/dc27b98a554f65cdc654b800da5aa1846545d41b/model_vdm.py#L298

**Parameters:**

w (`torch.Tensor`) : Generate embedding vectors with a specified guidance scale to subsequently enrich timestep embeddings.

embedding_dim (`int`, *optional*, defaults to 512) : Dimension of the embeddings to generate.

dtype (`torch.dtype`, *optional*, defaults to `torch.float32`) : Data type of the generated embeddings.

**Returns:**

``torch.Tensor``

Embedding vectors with shape `(len(w), embedding_dim)`.

## StableDiffusionPipelineOutput[[diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput]]

#### diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput[[diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_diffusion/pipeline_output.py#L10)

Output class for Stable Diffusion pipelines.

**Parameters:**

images (`list[PIL.Image.Image]` or `np.ndarray`) : list of denoised PIL images of length `batch_size` or NumPy array of shape `(batch_size, height, width, num_channels)`.

nsfw_content_detected (`list[bool]`) : list indicating whether the corresponding generated image contains "not-safe-for-work" (nsfw) content or `None` if safety checking could not be performed.
