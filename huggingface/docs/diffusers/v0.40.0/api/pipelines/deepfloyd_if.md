# DeepFloyd IF

  
  

## Overview

DeepFloyd IF is a novel state-of-the-art open-source text-to-image model with a high degree of photorealism and language understanding.
The model is a modular composed of a frozen text encoder and three cascaded pixel diffusion modules:
- Stage 1: a base model that generates 64x64 px image based on text prompt,
- Stage 2: a 64x64 px => 256x256 px super-resolution model, and
- Stage 3: a 256x256 px => 1024x1024 px super-resolution model
Stage 1 and Stage 2 utilize a frozen text encoder based on the T5 transformer to extract text embeddings, which are then fed into a UNet architecture enhanced with cross-attention and attention pooling.
Stage 3 is [Stability AI's x4 Upscaling model](https://huggingface.co/stabilityai/stable-diffusion-x4-upscaler).
The result is a highly efficient model that outperforms current state-of-the-art models, achieving a zero-shot FID score of 6.66 on the COCO dataset.
Our work underscores the potential of larger UNet architectures in the first stage of cascaded diffusion models and depicts a promising future for text-to-image synthesis.

## Usage

Before you can use IF, you need to accept its usage conditions. To do so:
1. Make sure to have a [Hugging Face account](https://huggingface.co/join) and be logged in.
2. Accept the license on the model card of [DeepFloyd/IF-I-XL-v1.0](https://huggingface.co/DeepFloyd/IF-I-XL-v1.0). Accepting the license on the stage I model card will auto accept for the other IF models.
3. Make sure to login locally. Install `huggingface_hub`:
```sh
pip install huggingface_hub --upgrade
```

run the login function in a Python shell:

```py
from huggingface_hub import login

login()
```

and enter your [Hugging Face Hub access token](https://huggingface.co/docs/hub/security-tokens#what-are-user-access-tokens).

Next we install `diffusers` and dependencies:

```sh
pip install -q diffusers accelerate transformers
```

The following sections give more in-detail examples of how to use IF. Specifically:

- [Text-to-Image Generation](#text-to-image-generation)
- [Image-to-Image Generation](#text-guided-image-to-image-generation)
- [Inpainting](#text-guided-inpainting-generation)
- [Reusing model weights](#converting-between-different-pipelines)
- [Speed optimization](#optimizing-for-speed)
- [Memory optimization](#optimizing-for-memory)

**Available checkpoints**
- *Stage-1*
  - [DeepFloyd/IF-I-XL-v1.0](https://huggingface.co/DeepFloyd/IF-I-XL-v1.0)
  - [DeepFloyd/IF-I-L-v1.0](https://huggingface.co/DeepFloyd/IF-I-L-v1.0)
  - [DeepFloyd/IF-I-M-v1.0](https://huggingface.co/DeepFloyd/IF-I-M-v1.0)

- *Stage-2*
  - [DeepFloyd/IF-II-L-v1.0](https://huggingface.co/DeepFloyd/IF-II-L-v1.0)
  - [DeepFloyd/IF-II-M-v1.0](https://huggingface.co/DeepFloyd/IF-II-M-v1.0)

- *Stage-3*
  - [stabilityai/stable-diffusion-x4-upscaler](https://huggingface.co/stabilityai/stable-diffusion-x4-upscaler)

**Google Colab**
[![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/huggingface/notebooks/blob/main/diffusers/deepfloyd_if_free_tier_google_colab.ipynb)

### Text-to-Image Generation

By default diffusers makes use of [model cpu offloading](../../optimization/memory#model-offloading) to run the whole IF pipeline with as little as 14 GB of VRAM.

```python
from diffusers import DiffusionPipeline
from diffusers.utils import pt_to_pil, make_image_grid
import torch

# stage 1
stage_1 = DiffusionPipeline.from_pretrained("DeepFloyd/IF-I-XL-v1.0", variant="fp16", dtype=torch.float16)
stage_1.enable_model_cpu_offload()

# stage 2
stage_2 = DiffusionPipeline.from_pretrained(
    "DeepFloyd/IF-II-L-v1.0", text_encoder=None, variant="fp16", dtype=torch.float16
)
stage_2.enable_model_cpu_offload()

# stage 3
safety_modules = {
    "feature_extractor": stage_1.feature_extractor,
    "safety_checker": stage_1.safety_checker,
    "watermarker": stage_1.watermarker,
}
stage_3 = DiffusionPipeline.from_pretrained(
    "stabilityai/stable-diffusion-x4-upscaler", **safety_modules, dtype=torch.float16
)
stage_3.enable_model_cpu_offload()

prompt = 'a photo of a kangaroo wearing an orange hoodie and blue sunglasses standing in front of the eiffel tower holding a sign that says "very deep learning"'
generator = torch.manual_seed(1)

# text embeds
prompt_embeds, negative_embeds = stage_1.encode_prompt(prompt)

# stage 1
stage_1_output = stage_1(
    prompt_embeds=prompt_embeds, negative_prompt_embeds=negative_embeds, generator=generator, output_type="pt"
).images
#pt_to_pil(stage_1_output)[0].save("./if_stage_I.png")

# stage 2
stage_2_output = stage_2(
    image=stage_1_output,
    prompt_embeds=prompt_embeds,
    negative_prompt_embeds=negative_embeds,
    generator=generator,
    output_type="pt",
).images
#pt_to_pil(stage_2_output)[0].save("./if_stage_II.png")

# stage 3
stage_3_output = stage_3(prompt=prompt, image=stage_2_output, noise_level=100, generator=generator).images
#stage_3_output[0].save("./if_stage_III.png")
make_image_grid([pt_to_pil(stage_1_output)[0], pt_to_pil(stage_2_output)[0], stage_3_output[0]], rows=1, rows=3)
```

### Text Guided Image-to-Image Generation

The same IF model weights can be used for text-guided image-to-image translation or image variation.
In this case just make sure to load the weights using the [IFImg2ImgPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/deepfloyd_if#diffusers.IFImg2ImgPipeline) and [IFImg2ImgSuperResolutionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/deepfloyd_if#diffusers.IFImg2ImgSuperResolutionPipeline) pipelines.

**Note**: You can also directly move the weights of the text-to-image pipelines to the image-to-image pipelines
without loading them twice by making use of the [components](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline.components) argument as explained [here](#converting-between-different-pipelines).

```python
from diffusers import IFImg2ImgPipeline, IFImg2ImgSuperResolutionPipeline, DiffusionPipeline
from diffusers.utils import pt_to_pil, load_image, make_image_grid
import torch

# download image
url = "https://raw.githubusercontent.com/CompVis/stable-diffusion/main/assets/stable-samples/img2img/sketch-mountains-input.jpg"
original_image = load_image(url)
original_image = original_image.resize((768, 512))

# stage 1
stage_1 = IFImg2ImgPipeline.from_pretrained("DeepFloyd/IF-I-XL-v1.0", variant="fp16", dtype=torch.float16)
stage_1.enable_model_cpu_offload()

# stage 2
stage_2 = IFImg2ImgSuperResolutionPipeline.from_pretrained(
    "DeepFloyd/IF-II-L-v1.0", text_encoder=None, variant="fp16", dtype=torch.float16
)
stage_2.enable_model_cpu_offload()

# stage 3
safety_modules = {
    "feature_extractor": stage_1.feature_extractor,
    "safety_checker": stage_1.safety_checker,
    "watermarker": stage_1.watermarker,
}
stage_3 = DiffusionPipeline.from_pretrained(
    "stabilityai/stable-diffusion-x4-upscaler", **safety_modules, dtype=torch.float16
)
stage_3.enable_model_cpu_offload()

prompt = "A fantasy landscape in style minecraft"
generator = torch.manual_seed(1)

# text embeds
prompt_embeds, negative_embeds = stage_1.encode_prompt(prompt)

# stage 1
stage_1_output = stage_1(
    image=original_image,
    prompt_embeds=prompt_embeds,
    negative_prompt_embeds=negative_embeds,
    generator=generator,
    output_type="pt",
).images
#pt_to_pil(stage_1_output)[0].save("./if_stage_I.png")

# stage 2
stage_2_output = stage_2(
    image=stage_1_output,
    original_image=original_image,
    prompt_embeds=prompt_embeds,
    negative_prompt_embeds=negative_embeds,
    generator=generator,
    output_type="pt",
).images
#pt_to_pil(stage_2_output)[0].save("./if_stage_II.png")

# stage 3
stage_3_output = stage_3(prompt=prompt, image=stage_2_output, generator=generator, noise_level=100).images
#stage_3_output[0].save("./if_stage_III.png")
make_image_grid([original_image, pt_to_pil(stage_1_output)[0], pt_to_pil(stage_2_output)[0], stage_3_output[0]], rows=1, rows=4)
```

### Text Guided Inpainting Generation

The same IF model weights can be used for text-guided image-to-image translation or image variation.
In this case just make sure to load the weights using the [IFInpaintingPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/deepfloyd_if#diffusers.IFInpaintingPipeline) and [IFInpaintingSuperResolutionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/deepfloyd_if#diffusers.IFInpaintingSuperResolutionPipeline) pipelines.

**Note**: You can also directly move the weights of the text-to-image pipelines to the image-to-image pipelines
without loading them twice by making use of the `~DiffusionPipeline.components()` function as explained [here](#converting-between-different-pipelines).

```python
from diffusers import IFInpaintingPipeline, IFInpaintingSuperResolutionPipeline, DiffusionPipeline
from diffusers.utils import pt_to_pil, load_image, make_image_grid
import torch

# download image
url = "https://huggingface.co/datasets/diffusers/docs-images/resolve/main/if/person.png"
original_image = load_image(url)

# download mask
url = "https://huggingface.co/datasets/diffusers/docs-images/resolve/main/if/glasses_mask.png"
mask_image = load_image(url)

# stage 1
stage_1 = IFInpaintingPipeline.from_pretrained("DeepFloyd/IF-I-XL-v1.0", variant="fp16", dtype=torch.float16)
stage_1.enable_model_cpu_offload()

# stage 2
stage_2 = IFInpaintingSuperResolutionPipeline.from_pretrained(
    "DeepFloyd/IF-II-L-v1.0", text_encoder=None, variant="fp16", dtype=torch.float16
)
stage_2.enable_model_cpu_offload()

# stage 3
safety_modules = {
    "feature_extractor": stage_1.feature_extractor,
    "safety_checker": stage_1.safety_checker,
    "watermarker": stage_1.watermarker,
}
stage_3 = DiffusionPipeline.from_pretrained(
    "stabilityai/stable-diffusion-x4-upscaler", **safety_modules, dtype=torch.float16
)
stage_3.enable_model_cpu_offload()

prompt = "blue sunglasses"
generator = torch.manual_seed(1)

# text embeds
prompt_embeds, negative_embeds = stage_1.encode_prompt(prompt)

# stage 1
stage_1_output = stage_1(
    image=original_image,
    mask_image=mask_image,
    prompt_embeds=prompt_embeds,
    negative_prompt_embeds=negative_embeds,
    generator=generator,
    output_type="pt",
).images
#pt_to_pil(stage_1_output)[0].save("./if_stage_I.png")

# stage 2
stage_2_output = stage_2(
    image=stage_1_output,
    original_image=original_image,
    mask_image=mask_image,
    prompt_embeds=prompt_embeds,
    negative_prompt_embeds=negative_embeds,
    generator=generator,
    output_type="pt",
).images
#pt_to_pil(stage_1_output)[0].save("./if_stage_II.png")

# stage 3
stage_3_output = stage_3(prompt=prompt, image=stage_2_output, generator=generator, noise_level=100).images
#stage_3_output[0].save("./if_stage_III.png")
make_image_grid([original_image, mask_image, pt_to_pil(stage_1_output)[0], pt_to_pil(stage_2_output)[0], stage_3_output[0]], rows=1, rows=5)
```

### Converting between different pipelines

In addition to being loaded with `from_pretrained`, Pipelines can also be loaded directly from each other.

```python
from diffusers import IFPipeline, IFSuperResolutionPipeline

pipe_1 = IFPipeline.from_pretrained("DeepFloyd/IF-I-XL-v1.0")
pipe_2 = IFSuperResolutionPipeline.from_pretrained("DeepFloyd/IF-II-L-v1.0")

from diffusers import IFImg2ImgPipeline, IFImg2ImgSuperResolutionPipeline

pipe_1 = IFImg2ImgPipeline(**pipe_1.components)
pipe_2 = IFImg2ImgSuperResolutionPipeline(**pipe_2.components)

from diffusers import IFInpaintingPipeline, IFInpaintingSuperResolutionPipeline

pipe_1 = IFInpaintingPipeline(**pipe_1.components)
pipe_2 = IFInpaintingSuperResolutionPipeline(**pipe_2.components)
```

### Optimizing for speed

The simplest optimization to run IF faster is to move all model components to the GPU.

```py
pipe = DiffusionPipeline.from_pretrained("DeepFloyd/IF-I-XL-v1.0", variant="fp16", dtype=torch.float16)
pipe.to("cuda")
```

You can also run the diffusion process for a shorter number of timesteps.

This can either be done with the `num_inference_steps` argument:

```py
pipe("<prompt>", num_inference_steps=30)
```

Or with the `timesteps` argument:

```py
from diffusers.pipelines.deepfloyd_if import fast27_timesteps

pipe("<prompt>", timesteps=fast27_timesteps)
```

When doing image variation or inpainting, you can also decrease the number of timesteps
with the strength argument. The strength argument is the amount of noise to add to the input image which also determines how many steps to run in the denoising process.
A smaller number will vary the image less but run faster.

```py
pipe = IFImg2ImgPipeline.from_pretrained("DeepFloyd/IF-I-XL-v1.0", variant="fp16", dtype=torch.float16)
pipe.to("cuda")

image = pipe(image=image, prompt="<prompt>", strength=0.3).images
```

You can also use [`torch.compile`](../../optimization/fp16#torchcompile). Note that we have not exhaustively tested `torch.compile`
with IF and it might not give expected results.

```py
from diffusers import DiffusionPipeline
import torch

pipe = DiffusionPipeline.from_pretrained("DeepFloyd/IF-I-XL-v1.0", variant="fp16", dtype=torch.float16)
pipe.to("cuda")

pipe.text_encoder = torch.compile(pipe.text_encoder, mode="reduce-overhead", fullgraph=True)
pipe.unet = torch.compile(pipe.unet, mode="reduce-overhead", fullgraph=True)
```

### Optimizing for memory

When optimizing for GPU memory, we can use the standard diffusers CPU offloading APIs.

Either the model based CPU offloading,

```py
pipe = DiffusionPipeline.from_pretrained("DeepFloyd/IF-I-XL-v1.0", variant="fp16", dtype=torch.float16)
pipe.enable_model_cpu_offload()
```

or the more aggressive layer based CPU offloading.

```py
pipe = DiffusionPipeline.from_pretrained("DeepFloyd/IF-I-XL-v1.0", variant="fp16", dtype=torch.float16)
pipe.enable_sequential_cpu_offload()
```

Additionally, T5 can be loaded in 8bit precision

```py
from transformers import T5EncoderModel

text_encoder = T5EncoderModel.from_pretrained(
    "DeepFloyd/IF-I-XL-v1.0", subfolder="text_encoder", device_map="auto", load_in_8bit=True, variant="8bit"
)

from diffusers import DiffusionPipeline

pipe = DiffusionPipeline.from_pretrained(
    "DeepFloyd/IF-I-XL-v1.0",
    text_encoder=text_encoder,  # pass the previously instantiated 8bit text encoder
    unet=None,
    device_map="auto",
)

prompt_embeds, negative_embeds = pipe.encode_prompt("<prompt>")
```

For CPU RAM constrained machines like Google Colab free tier where we can't load all model components to the CPU at once, we can manually only load the pipeline with
the text encoder or UNet when the respective model components are needed.

```py
from diffusers import IFPipeline, IFSuperResolutionPipeline
import torch
import gc
from transformers import T5EncoderModel
from diffusers.utils import pt_to_pil, make_image_grid

text_encoder = T5EncoderModel.from_pretrained(
    "DeepFloyd/IF-I-XL-v1.0", subfolder="text_encoder", device_map="auto", load_in_8bit=True, variant="8bit"
)

# text to image
pipe = DiffusionPipeline.from_pretrained(
    "DeepFloyd/IF-I-XL-v1.0",
    text_encoder=text_encoder,  # pass the previously instantiated 8bit text encoder
    unet=None,
    device_map="auto",
)

prompt = 'a photo of a kangaroo wearing an orange hoodie and blue sunglasses standing in front of the eiffel tower holding a sign that says "very deep learning"'
prompt_embeds, negative_embeds = pipe.encode_prompt(prompt)

# Remove the pipeline so we can re-load the pipeline with the unet
del text_encoder
del pipe
gc.collect()
torch.cuda.empty_cache()

pipe = IFPipeline.from_pretrained(
    "DeepFloyd/IF-I-XL-v1.0", text_encoder=None, variant="fp16", dtype=torch.float16, device_map="auto"
)

generator = torch.Generator().manual_seed(0)
stage_1_output = pipe(
    prompt_embeds=prompt_embeds,
    negative_prompt_embeds=negative_embeds,
    output_type="pt",
    generator=generator,
).images

#pt_to_pil(stage_1_output)[0].save("./if_stage_I.png")

# Remove the pipeline so we can load the super-resolution pipeline
del pipe
gc.collect()
torch.cuda.empty_cache()

# First super resolution

pipe = IFSuperResolutionPipeline.from_pretrained(
    "DeepFloyd/IF-II-L-v1.0", text_encoder=None, variant="fp16", dtype=torch.float16, device_map="auto"
)

generator = torch.Generator().manual_seed(0)
stage_2_output = pipe(
    image=stage_1_output,
    prompt_embeds=prompt_embeds,
    negative_prompt_embeds=negative_embeds,
    output_type="pt",
    generator=generator,
).images

#pt_to_pil(stage_2_output)[0].save("./if_stage_II.png")
make_image_grid([pt_to_pil(stage_1_output)[0], pt_to_pil(stage_2_output)[0]], rows=1, rows=2)
```

## Available Pipelines:

| Pipeline | Tasks | Colab
|---|---|:---:|
| [pipeline_if.py](https://github.com/huggingface/diffusers/blob/main/src/diffusers/pipelines/deepfloyd_if/pipeline_if.py) | *Text-to-Image Generation* | - |
| [pipeline_if_superresolution.py](https://github.com/huggingface/diffusers/blob/main/src/diffusers/pipelines/deepfloyd_if/pipeline_if_superresolution.py) | *Text-to-Image Generation* | - |
| [pipeline_if_img2img.py](https://github.com/huggingface/diffusers/blob/main/src/diffusers/pipelines/deepfloyd_if/pipeline_if_img2img.py) | *Image-to-Image Generation* | - |
| [pipeline_if_img2img_superresolution.py](https://github.com/huggingface/diffusers/blob/main/src/diffusers/pipelines/deepfloyd_if/pipeline_if_img2img_superresolution.py) | *Image-to-Image Generation* | - |
| [pipeline_if_inpainting.py](https://github.com/huggingface/diffusers/blob/main/src/diffusers/pipelines/deepfloyd_if/pipeline_if_inpainting.py) | *Image-to-Image Generation* | - |
| [pipeline_if_inpainting_superresolution.py](https://github.com/huggingface/diffusers/blob/main/src/diffusers/pipelines/deepfloyd_if/pipeline_if_inpainting_superresolution.py) | *Image-to-Image Generation* | - |

## IFPipeline[[diffusers.IFPipeline]]

#### diffusers.IFPipeline[[diffusers.IFPipeline]]

```python
diffusers.IFPipeline(tokenizer: T5Tokenizer, text_encoder: T5EncoderModel, unet: UNet2DConditionModel, scheduler: DDPMScheduler, safety_checker: diffusers.pipelines.deepfloyd_if.safety_checker.IFSafetyChecker | None, feature_extractor: transformers.models.clip.image_processing_pil_clip.CLIPImageProcessorPil | None, watermarker: diffusers.pipelines.deepfloyd_if.watermark.IFWatermarker | None, requires_safety_checker: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/deepfloyd_if/pipeline_if.py#L96)

#### __call__[[diffusers.IFPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, num_inference_steps: int = 100, timesteps: list = None, guidance_scale: float = 7.0, negative_prompt: str | list[str] | None = None, num_images_per_prompt: int | None = 1, height: int | None = None, width: int | None = None, eta: float = 0.0, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, callback: typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None, callback_steps: int = 1, clean_caption: bool = True, cross_attention_kwargs: dict[str, typing.Any] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/deepfloyd_if/pipeline_if.py#L547)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

timesteps (`list[int]`, *optional*) : Custom timesteps to use for the denoising process. If not defined, equal spaced `num_inference_steps` timesteps are used. Must be in descending order.

guidance_scale (`float`, *optional*, defaults to 7.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

height (`int`, *optional*, defaults to self.unet.config.sample_size) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to self.unet.config.sample_size) : The width in pixels of the generated image.

eta (`float`, *optional*, defaults to 0.0) : Corresponds to parameter eta (η) in the DDIM paper: https://huggingface.co/papers/2010.02502. Only applies to [schedulers.DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), will be ignored for others.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.stable_diffusion.IFPipelineOutput` instead of a plain tuple.

callback (`Callable`, *optional*) : A function that will be called every `callback_steps` steps during inference. The function will be called with the following arguments: `callback(step: int, timestep: int, latents: torch.Tensor)`.

callback_steps (`int`, *optional*, defaults to 1) : The frequency at which the `callback` function will be called. If not specified, the callback will be called at every step.

clean_caption (`bool`, *optional*, defaults to `True`) : Whether or not to clean the caption before creating embeddings. Requires `beautifulsoup4` and `ftfy` to be installed. If the dependencies are not installed, the embeddings will be created from the raw prompt.

cross_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

**Returns:** `~pipelines.stable_diffusion.IFPipelineOutput` or `tuple`

`~pipelines.stable_diffusion.IFPipelineOutput` if `return_dict` is True, otherwise a `tuple. When
returning a tuple, the first element is a list with the generated images, and the second element is a list
of `bool`s denoting whether the corresponding generated image likely represents "not-safe-for-work" (nsfw)
or watermarked content, according to the `safety_checker`.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> from diffusers import IFPipeline, IFSuperResolutionPipeline, DiffusionPipeline
>>> from diffusers.utils import pt_to_pil
>>> import torch

>>> pipe = IFPipeline.from_pretrained("DeepFloyd/IF-I-XL-v1.0", variant="fp16", torch_dtype=torch.float16)
>>> pipe.enable_model_cpu_offload()

>>> prompt = 'a photo of a kangaroo wearing an orange hoodie and blue sunglasses standing in front of the eiffel tower holding a sign that says "very deep learning"'
>>> prompt_embeds, negative_embeds = pipe.encode_prompt(prompt)

>>> image = pipe(prompt_embeds=prompt_embeds, negative_prompt_embeds=negative_embeds, output_type="pt").images

>>> # save intermediate image
>>> pil_image = pt_to_pil(image)
>>> pil_image[0].save("./if_stage_I.png")

>>> super_res_1_pipe = IFSuperResolutionPipeline.from_pretrained(
...     "DeepFloyd/IF-II-L-v1.0", text_encoder=None, variant="fp16", torch_dtype=torch.float16
... )
>>> super_res_1_pipe.enable_model_cpu_offload()

>>> image = super_res_1_pipe(
...     image=image, prompt_embeds=prompt_embeds, negative_prompt_embeds=negative_embeds, output_type="pt"
... ).images

>>> # save intermediate image
>>> pil_image = pt_to_pil(image)
>>> pil_image[0].save("./if_stage_I.png")

>>> safety_modules = {
...     "feature_extractor": pipe.feature_extractor,
...     "safety_checker": pipe.safety_checker,
...     "watermarker": pipe.watermarker,
... }
>>> super_res_2_pipe = DiffusionPipeline.from_pretrained(
...     "stabilityai/stable-diffusion-x4-upscaler", **safety_modules, torch_dtype=torch.float16
... )
>>> super_res_2_pipe.enable_model_cpu_offload()

>>> image = super_res_2_pipe(
...     prompt=prompt,
...     image=image,
... ).images
>>> image[0].save("./if_stage_II.png")
```

#### encode_prompt[[diffusers.IFPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], do_classifier_free_guidance: bool = True, num_images_per_prompt: int = 1, device: typing.Optional[torch.device] = None, negative_prompt: str | list[str] | None = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, clean_caption: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/deepfloyd_if/pipeline_if.py#L168)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : whether to use classifier free guidance or not

num_images_per_prompt (`int`, *optional*, defaults to 1) : number of images that should be generated per prompt

device : (`torch.device`, *optional*): torch device to place the resulting embeddings on

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds`. instead. If not defined, one has to pass `negative_prompt_embeds`. instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

clean_caption (bool, defaults to `False`) : If `True`, the function will preprocess and clean the provided caption before encoding.

Encodes the prompt into text encoder hidden states.

## IFSuperResolutionPipeline[[diffusers.IFSuperResolutionPipeline]]

#### diffusers.IFSuperResolutionPipeline[[diffusers.IFSuperResolutionPipeline]]

```python
diffusers.IFSuperResolutionPipeline(tokenizer: T5Tokenizer, text_encoder: T5EncoderModel, unet: UNet2DConditionModel, scheduler: DDPMScheduler, image_noising_scheduler: DDPMScheduler, safety_checker: diffusers.pipelines.deepfloyd_if.safety_checker.IFSafetyChecker | None, feature_extractor: transformers.models.clip.image_processing_pil_clip.CLIPImageProcessorPil | None, watermarker: diffusers.pipelines.deepfloyd_if.watermark.IFWatermarker | None, requires_safety_checker: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/deepfloyd_if/pipeline_if_superresolution.py#L82)

#### __call__[[diffusers.IFSuperResolutionPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, height: int = None, width: int = None, image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor] = None, num_inference_steps: int = 50, timesteps: list = None, guidance_scale: float = 4.0, negative_prompt: str | list[str] | None = None, num_images_per_prompt: int | None = 1, eta: float = 0.0, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, callback: typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None, callback_steps: int = 1, cross_attention_kwargs: dict[str, typing.Any] | None = None, noise_level: int = 250, clean_caption: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/deepfloyd_if/pipeline_if_superresolution.py#L614)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

height (`int`, *optional*, defaults to None) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to None) : The width in pixels of the generated image.

image (`PIL.Image.Image`, `np.ndarray`, `torch.Tensor`) : The image to be upscaled.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

timesteps (`list[int]`, *optional*, defaults to None) : Custom timesteps to use for the denoising process. If not defined, equal spaced `num_inference_steps` timesteps are used. Must be in descending order.

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

eta (`float`, *optional*, defaults to 0.0) : Corresponds to parameter eta (η) in the DDIM paper: https://huggingface.co/papers/2010.02502. Only applies to [schedulers.DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), will be ignored for others.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.stable_diffusion.IFPipelineOutput` instead of a plain tuple.

callback (`Callable`, *optional*) : A function that will be called every `callback_steps` steps during inference. The function will be called with the following arguments: `callback(step: int, timestep: int, latents: torch.Tensor)`.

callback_steps (`int`, *optional*, defaults to 1) : The frequency at which the `callback` function will be called. If not specified, the callback will be called at every step.

cross_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

noise_level (`int`, *optional*, defaults to 250) : The amount of noise to add to the upscaled image. Must be in the range `[0, 1000)`

clean_caption (`bool`, *optional*, defaults to `True`) : Whether or not to clean the caption before creating embeddings. Requires `beautifulsoup4` and `ftfy` to be installed. If the dependencies are not installed, the embeddings will be created from the raw prompt.

**Returns:** `~pipelines.stable_diffusion.IFPipelineOutput` or `tuple`

`~pipelines.stable_diffusion.IFPipelineOutput` if `return_dict` is True, otherwise a `tuple. When
returning a tuple, the first element is a list with the generated images, and the second element is a list
of `bool`s denoting whether the corresponding generated image likely represents "not-safe-for-work" (nsfw)
or watermarked content, according to the `safety_checker`.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> from diffusers import IFPipeline, IFSuperResolutionPipeline, DiffusionPipeline
>>> from diffusers.utils import pt_to_pil
>>> import torch

>>> pipe = IFPipeline.from_pretrained("DeepFloyd/IF-I-XL-v1.0", variant="fp16", torch_dtype=torch.float16)
>>> pipe.enable_model_cpu_offload()

>>> prompt = 'a photo of a kangaroo wearing an orange hoodie and blue sunglasses standing in front of the eiffel tower holding a sign that says "very deep learning"'
>>> prompt_embeds, negative_embeds = pipe.encode_prompt(prompt)

>>> image = pipe(prompt_embeds=prompt_embeds, negative_prompt_embeds=negative_embeds, output_type="pt").images

>>> # save intermediate image
>>> pil_image = pt_to_pil(image)
>>> pil_image[0].save("./if_stage_I.png")

>>> super_res_1_pipe = IFSuperResolutionPipeline.from_pretrained(
...     "DeepFloyd/IF-II-L-v1.0", text_encoder=None, variant="fp16", torch_dtype=torch.float16
... )
>>> super_res_1_pipe.enable_model_cpu_offload()

>>> image = super_res_1_pipe(
...     image=image, prompt_embeds=prompt_embeds, negative_prompt_embeds=negative_embeds
... ).images
>>> image[0].save("./if_stage_II.png")
```

#### encode_prompt[[diffusers.IFSuperResolutionPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], do_classifier_free_guidance: bool = True, num_images_per_prompt: int = 1, device: typing.Optional[torch.device] = None, negative_prompt: str | list[str] | None = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, clean_caption: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/deepfloyd_if/pipeline_if_superresolution.py#L302)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : whether to use classifier free guidance or not

num_images_per_prompt (`int`, *optional*, defaults to 1) : number of images that should be generated per prompt

device : (`torch.device`, *optional*): torch device to place the resulting embeddings on

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds`. instead. If not defined, one has to pass `negative_prompt_embeds`. instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

clean_caption (bool, defaults to `False`) : If `True`, the function will preprocess and clean the provided caption before encoding.

Encodes the prompt into text encoder hidden states.

## IFImg2ImgPipeline[[diffusers.IFImg2ImgPipeline]]

#### diffusers.IFImg2ImgPipeline[[diffusers.IFImg2ImgPipeline]]

```python
diffusers.IFImg2ImgPipeline(tokenizer: T5Tokenizer, text_encoder: T5EncoderModel, unet: UNet2DConditionModel, scheduler: DDPMScheduler, safety_checker: diffusers.pipelines.deepfloyd_if.safety_checker.IFSafetyChecker | None, feature_extractor: transformers.models.clip.image_processing_pil_clip.CLIPImageProcessorPil | None, watermarker: diffusers.pipelines.deepfloyd_if.watermark.IFWatermarker | None, requires_safety_checker: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/deepfloyd_if/pipeline_if_img2img.py#L120)

#### __call__[[diffusers.IFImg2ImgPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, image: typing.Union[PIL.Image.Image, torch.Tensor, numpy.ndarray, list[PIL.Image.Image], list[torch.Tensor], list[numpy.ndarray]] = None, strength: float = 0.7, num_inference_steps: int = 80, timesteps: list = None, guidance_scale: float = 10.0, negative_prompt: str | list[str] | None = None, num_images_per_prompt: int | None = 1, eta: float = 0.0, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, callback: typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None, callback_steps: int = 1, clean_caption: bool = True, cross_attention_kwargs: dict[str, typing.Any] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/deepfloyd_if/pipeline_if_img2img.py#L661)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

image (`torch.Tensor` or `PIL.Image.Image`) : `Image`, or tensor representing an image batch, that will be used as the starting point for the process.

strength (`float`, *optional*, defaults to 0.7) : Conceptually, indicates how much to transform the reference `image`. Must be between 0 and 1. `image` will be used as a starting point, adding more noise to it the larger the `strength`. The number of denoising steps depends on the amount of noise initially added. When `strength` is 1, added noise will be maximum and the denoising process will run for the full number of iterations specified in `num_inference_steps`. A value of 1, therefore, essentially ignores `image`.

num_inference_steps (`int`, *optional*, defaults to 80) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

timesteps (`list[int]`, *optional*) : Custom timesteps to use for the denoising process. If not defined, equal spaced `num_inference_steps` timesteps are used. Must be in descending order.

guidance_scale (`float`, *optional*, defaults to 10.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

eta (`float`, *optional*, defaults to 0.0) : Corresponds to parameter eta (η) in the DDIM paper: https://huggingface.co/papers/2010.02502. Only applies to [schedulers.DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), will be ignored for others.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.stable_diffusion.IFPipelineOutput` instead of a plain tuple.

callback (`Callable`, *optional*) : A function that will be called every `callback_steps` steps during inference. The function will be called with the following arguments: `callback(step: int, timestep: int, latents: torch.Tensor)`.

callback_steps (`int`, *optional*, defaults to 1) : The frequency at which the `callback` function will be called. If not specified, the callback will be called at every step.

clean_caption (`bool`, *optional*, defaults to `True`) : Whether or not to clean the caption before creating embeddings. Requires `beautifulsoup4` and `ftfy` to be installed. If the dependencies are not installed, the embeddings will be created from the raw prompt.

cross_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

**Returns:** `~pipelines.stable_diffusion.IFPipelineOutput` or `tuple`

`~pipelines.stable_diffusion.IFPipelineOutput` if `return_dict` is True, otherwise a `tuple. When
returning a tuple, the first element is a list with the generated images, and the second element is a list
of `bool`s denoting whether the corresponding generated image likely represents "not-safe-for-work" (nsfw)
or watermarked content, according to the `safety_checker`.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> from diffusers import IFImg2ImgPipeline, IFImg2ImgSuperResolutionPipeline, DiffusionPipeline
>>> from diffusers.utils import pt_to_pil
>>> import torch
>>> from PIL import Image
>>> import requests
>>> from io import BytesIO

>>> url = "https://raw.githubusercontent.com/CompVis/stable-diffusion/main/assets/stable-samples/img2img/sketch-mountains-input.jpg"
>>> response = requests.get(url)
>>> original_image = Image.open(BytesIO(response.content)).convert("RGB")
>>> original_image = original_image.resize((768, 512))

>>> pipe = IFImg2ImgPipeline.from_pretrained(
...     "DeepFloyd/IF-I-XL-v1.0",
...     variant="fp16",
...     torch_dtype=torch.float16,
... )
>>> pipe.enable_model_cpu_offload()

>>> prompt = "A fantasy landscape in style minecraft"
>>> prompt_embeds, negative_embeds = pipe.encode_prompt(prompt)

>>> image = pipe(
...     image=original_image,
...     prompt_embeds=prompt_embeds,
...     negative_prompt_embeds=negative_embeds,
...     output_type="pt",
... ).images

>>> # save intermediate image
>>> pil_image = pt_to_pil(image)
>>> pil_image[0].save("./if_stage_I.png")

>>> super_res_1_pipe = IFImg2ImgSuperResolutionPipeline.from_pretrained(
...     "DeepFloyd/IF-II-L-v1.0",
...     text_encoder=None,
...     variant="fp16",
...     torch_dtype=torch.float16,
... )
>>> super_res_1_pipe.enable_model_cpu_offload()

>>> image = super_res_1_pipe(
...     image=image,
...     original_image=original_image,
...     prompt_embeds=prompt_embeds,
...     negative_prompt_embeds=negative_embeds,
... ).images
>>> image[0].save("./if_stage_II.png")
```

#### encode_prompt[[diffusers.IFImg2ImgPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], do_classifier_free_guidance: bool = True, num_images_per_prompt: int = 1, device: typing.Optional[torch.device] = None, negative_prompt: str | list[str] | None = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, clean_caption: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/deepfloyd_if/pipeline_if_img2img.py#L192)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : whether to use classifier free guidance or not

num_images_per_prompt (`int`, *optional*, defaults to 1) : number of images that should be generated per prompt

device : (`torch.device`, *optional*): torch device to place the resulting embeddings on

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds`. instead. If not defined, one has to pass `negative_prompt_embeds`. instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

clean_caption (bool, defaults to `False`) : If `True`, the function will preprocess and clean the provided caption before encoding.

Encodes the prompt into text encoder hidden states.

## IFImg2ImgSuperResolutionPipeline[[diffusers.IFImg2ImgSuperResolutionPipeline]]

#### diffusers.IFImg2ImgSuperResolutionPipeline[[diffusers.IFImg2ImgSuperResolutionPipeline]]

```python
diffusers.IFImg2ImgSuperResolutionPipeline(tokenizer: T5Tokenizer, text_encoder: T5EncoderModel, unet: UNet2DConditionModel, scheduler: DDPMScheduler, image_noising_scheduler: DDPMScheduler, safety_checker: diffusers.pipelines.deepfloyd_if.safety_checker.IFSafetyChecker | None, feature_extractor: transformers.models.clip.image_processing_pil_clip.CLIPImageProcessorPil | None, watermarker: diffusers.pipelines.deepfloyd_if.watermark.IFWatermarker | None, requires_safety_checker: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/deepfloyd_if/pipeline_if_img2img_superresolution.py#L124)

#### __call__[[diffusers.IFImg2ImgSuperResolutionPipeline.__call__]]

```python
__call__(image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor], original_image: typing.Union[PIL.Image.Image, torch.Tensor, numpy.ndarray, list[PIL.Image.Image], list[torch.Tensor], list[numpy.ndarray]] = None, strength: float = 0.8, prompt: str | list[str] = None, num_inference_steps: int = 50, timesteps: list = None, guidance_scale: float = 4.0, negative_prompt: str | list[str] | None = None, num_images_per_prompt: int | None = 1, eta: float = 0.0, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, callback: typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None, callback_steps: int = 1, cross_attention_kwargs: dict[str, typing.Any] | None = None, noise_level: int = 250, clean_caption: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/deepfloyd_if/pipeline_if_img2img_superresolution.py#L744)

**Parameters:**

image (`torch.Tensor` or `PIL.Image.Image`) : `Image`, or tensor representing an image batch, that will be used as the starting point for the process.

original_image (`torch.Tensor` or `PIL.Image.Image`) : The original image that `image` was varied from.

strength (`float`, *optional*, defaults to 0.8) : Conceptually, indicates how much to transform the reference `image`. Must be between 0 and 1. `image` will be used as a starting point, adding more noise to it the larger the `strength`. The number of denoising steps depends on the amount of noise initially added. When `strength` is 1, added noise will be maximum and the denoising process will run for the full number of iterations specified in `num_inference_steps`. A value of 1, therefore, essentially ignores `image`.

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

timesteps (`list[int]`, *optional*) : Custom timesteps to use for the denoising process. If not defined, equal spaced `num_inference_steps` timesteps are used. Must be in descending order.

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

eta (`float`, *optional*, defaults to 0.0) : Corresponds to parameter eta (η) in the DDIM paper: https://huggingface.co/papers/2010.02502. Only applies to [schedulers.DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), will be ignored for others.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.stable_diffusion.IFPipelineOutput` instead of a plain tuple.

callback (`Callable`, *optional*) : A function that will be called every `callback_steps` steps during inference. The function will be called with the following arguments: `callback(step: int, timestep: int, latents: torch.Tensor)`.

callback_steps (`int`, *optional*, defaults to 1) : The frequency at which the `callback` function will be called. If not specified, the callback will be called at every step.

cross_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

noise_level (`int`, *optional*, defaults to 250) : The amount of noise to add to the upscaled image. Must be in the range `[0, 1000)`

clean_caption (`bool`, *optional*, defaults to `True`) : Whether or not to clean the caption before creating embeddings. Requires `beautifulsoup4` and `ftfy` to be installed. If the dependencies are not installed, the embeddings will be created from the raw prompt.

**Returns:** `~pipelines.stable_diffusion.IFPipelineOutput` or `tuple`

`~pipelines.stable_diffusion.IFPipelineOutput` if `return_dict` is True, otherwise a `tuple. When
returning a tuple, the first element is a list with the generated images, and the second element is a list
of `bool`s denoting whether the corresponding generated image likely represents "not-safe-for-work" (nsfw)
or watermarked content, according to the `safety_checker`.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> from diffusers import IFImg2ImgPipeline, IFImg2ImgSuperResolutionPipeline, DiffusionPipeline
>>> from diffusers.utils import pt_to_pil
>>> import torch
>>> from PIL import Image
>>> import requests
>>> from io import BytesIO

>>> url = "https://raw.githubusercontent.com/CompVis/stable-diffusion/main/assets/stable-samples/img2img/sketch-mountains-input.jpg"
>>> response = requests.get(url)
>>> original_image = Image.open(BytesIO(response.content)).convert("RGB")
>>> original_image = original_image.resize((768, 512))

>>> pipe = IFImg2ImgPipeline.from_pretrained(
...     "DeepFloyd/IF-I-XL-v1.0",
...     variant="fp16",
...     torch_dtype=torch.float16,
... )
>>> pipe.enable_model_cpu_offload()

>>> prompt = "A fantasy landscape in style minecraft"
>>> prompt_embeds, negative_embeds = pipe.encode_prompt(prompt)

>>> image = pipe(
...     image=original_image,
...     prompt_embeds=prompt_embeds,
...     negative_prompt_embeds=negative_embeds,
...     output_type="pt",
... ).images

>>> # save intermediate image
>>> pil_image = pt_to_pil(image)
>>> pil_image[0].save("./if_stage_I.png")

>>> super_res_1_pipe = IFImg2ImgSuperResolutionPipeline.from_pretrained(
...     "DeepFloyd/IF-II-L-v1.0",
...     text_encoder=None,
...     variant="fp16",
...     torch_dtype=torch.float16,
... )
>>> super_res_1_pipe.enable_model_cpu_offload()

>>> image = super_res_1_pipe(
...     image=image,
...     original_image=original_image,
...     prompt_embeds=prompt_embeds,
...     negative_prompt_embeds=negative_embeds,
... ).images
>>> image[0].save("./if_stage_II.png")
```

#### encode_prompt[[diffusers.IFImg2ImgSuperResolutionPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], do_classifier_free_guidance: bool = True, num_images_per_prompt: int = 1, device: typing.Optional[torch.device] = None, negative_prompt: str | list[str] | None = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, clean_caption: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/deepfloyd_if/pipeline_if_img2img_superresolution.py#L344)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : whether to use classifier free guidance or not

num_images_per_prompt (`int`, *optional*, defaults to 1) : number of images that should be generated per prompt

device : (`torch.device`, *optional*): torch device to place the resulting embeddings on

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds`. instead. If not defined, one has to pass `negative_prompt_embeds`. instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

clean_caption (bool, defaults to `False`) : If `True`, the function will preprocess and clean the provided caption before encoding.

Encodes the prompt into text encoder hidden states.

## IFInpaintingPipeline[[diffusers.IFInpaintingPipeline]]

#### diffusers.IFInpaintingPipeline[[diffusers.IFInpaintingPipeline]]

```python
diffusers.IFInpaintingPipeline(tokenizer: T5Tokenizer, text_encoder: T5EncoderModel, unet: UNet2DConditionModel, scheduler: DDPMScheduler, safety_checker: diffusers.pipelines.deepfloyd_if.safety_checker.IFSafetyChecker | None, feature_extractor: transformers.models.clip.image_processing_pil_clip.CLIPImageProcessorPil | None, watermarker: diffusers.pipelines.deepfloyd_if.watermark.IFWatermarker | None, requires_safety_checker: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/deepfloyd_if/pipeline_if_inpainting.py#L123)

#### __call__[[diffusers.IFInpaintingPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, image: typing.Union[PIL.Image.Image, torch.Tensor, numpy.ndarray, list[PIL.Image.Image], list[torch.Tensor], list[numpy.ndarray]] = None, mask_image: typing.Union[PIL.Image.Image, torch.Tensor, numpy.ndarray, list[PIL.Image.Image], list[torch.Tensor], list[numpy.ndarray]] = None, strength: float = 1.0, num_inference_steps: int = 50, timesteps: list = None, guidance_scale: float = 7.0, negative_prompt: str | list[str] | None = None, num_images_per_prompt: int | None = 1, eta: float = 0.0, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, callback: typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None, callback_steps: int = 1, clean_caption: bool = True, cross_attention_kwargs: dict[str, typing.Any] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/deepfloyd_if/pipeline_if_inpainting.py#L753)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

image (`torch.Tensor` or `PIL.Image.Image`) : `Image`, or tensor representing an image batch, that will be used as the starting point for the process.

mask_image (`PIL.Image.Image`) : `Image`, or tensor representing an image batch, to mask `image`. White pixels in the mask will be repainted, while black pixels will be preserved. If `mask_image` is a PIL image, it will be converted to a single channel (luminance) before use. If it's a tensor, it should contain one color channel (L) instead of 3, so the expected shape would be `(B, H, W, 1)`.

strength (`float`, *optional*, defaults to 1.0) : Conceptually, indicates how much to transform the reference `image`. Must be between 0 and 1. `image` will be used as a starting point, adding more noise to it the larger the `strength`. The number of denoising steps depends on the amount of noise initially added. When `strength` is 1, added noise will be maximum and the denoising process will run for the full number of iterations specified in `num_inference_steps`. A value of 1, therefore, essentially ignores `image`.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

timesteps (`list[int]`, *optional*) : Custom timesteps to use for the denoising process. If not defined, equal spaced `num_inference_steps` timesteps are used. Must be in descending order.

guidance_scale (`float`, *optional*, defaults to 7.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

eta (`float`, *optional*, defaults to 0.0) : Corresponds to parameter eta (η) in the DDIM paper: https://huggingface.co/papers/2010.02502. Only applies to [schedulers.DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), will be ignored for others.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.stable_diffusion.IFPipelineOutput` instead of a plain tuple.

callback (`Callable`, *optional*) : A function that will be called every `callback_steps` steps during inference. The function will be called with the following arguments: `callback(step: int, timestep: int, latents: torch.Tensor)`.

callback_steps (`int`, *optional*, defaults to 1) : The frequency at which the `callback` function will be called. If not specified, the callback will be called at every step.

clean_caption (`bool`, *optional*, defaults to `True`) : Whether or not to clean the caption before creating embeddings. Requires `beautifulsoup4` and `ftfy` to be installed. If the dependencies are not installed, the embeddings will be created from the raw prompt.

cross_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

**Returns:** `~pipelines.stable_diffusion.IFPipelineOutput` or `tuple`

`~pipelines.stable_diffusion.IFPipelineOutput` if `return_dict` is True, otherwise a `tuple. When
returning a tuple, the first element is a list with the generated images, and the second element is a list
of `bool`s denoting whether the corresponding generated image likely represents "not-safe-for-work" (nsfw)
or watermarked content, according to the `safety_checker`.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> from diffusers import IFInpaintingPipeline, IFInpaintingSuperResolutionPipeline, DiffusionPipeline
>>> from diffusers.utils import pt_to_pil
>>> import torch
>>> from PIL import Image
>>> import requests
>>> from io import BytesIO

>>> url = "https://huggingface.co/datasets/diffusers/docs-images/resolve/main/if/person.png"
>>> response = requests.get(url)
>>> original_image = Image.open(BytesIO(response.content)).convert("RGB")
>>> original_image = original_image

>>> url = "https://huggingface.co/datasets/diffusers/docs-images/resolve/main/if/glasses_mask.png"
>>> response = requests.get(url)
>>> mask_image = Image.open(BytesIO(response.content))
>>> mask_image = mask_image

>>> pipe = IFInpaintingPipeline.from_pretrained(
...     "DeepFloyd/IF-I-XL-v1.0", variant="fp16", torch_dtype=torch.float16
... )
>>> pipe.enable_model_cpu_offload()

>>> prompt = "blue sunglasses"
>>> prompt_embeds, negative_embeds = pipe.encode_prompt(prompt)

>>> image = pipe(
...     image=original_image,
...     mask_image=mask_image,
...     prompt_embeds=prompt_embeds,
...     negative_prompt_embeds=negative_embeds,
...     output_type="pt",
... ).images

>>> # save intermediate image
>>> pil_image = pt_to_pil(image)
>>> pil_image[0].save("./if_stage_I.png")

>>> super_res_1_pipe = IFInpaintingSuperResolutionPipeline.from_pretrained(
...     "DeepFloyd/IF-II-L-v1.0", text_encoder=None, variant="fp16", torch_dtype=torch.float16
... )
>>> super_res_1_pipe.enable_model_cpu_offload()

>>> image = super_res_1_pipe(
...     image=image,
...     mask_image=mask_image,
...     original_image=original_image,
...     prompt_embeds=prompt_embeds,
...     negative_prompt_embeds=negative_embeds,
... ).images
>>> image[0].save("./if_stage_II.png")
```

#### encode_prompt[[diffusers.IFInpaintingPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], do_classifier_free_guidance: bool = True, num_images_per_prompt: int = 1, device: typing.Optional[torch.device] = None, negative_prompt: str | list[str] | None = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, clean_caption: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/deepfloyd_if/pipeline_if_inpainting.py#L195)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : whether to use classifier free guidance or not

num_images_per_prompt (`int`, *optional*, defaults to 1) : number of images that should be generated per prompt

device : (`torch.device`, *optional*): torch device to place the resulting embeddings on

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds`. instead. If not defined, one has to pass `negative_prompt_embeds`. instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

clean_caption (bool, defaults to `False`) : If `True`, the function will preprocess and clean the provided caption before encoding.

Encodes the prompt into text encoder hidden states.

## IFInpaintingSuperResolutionPipeline[[diffusers.IFInpaintingSuperResolutionPipeline]]

#### diffusers.IFInpaintingSuperResolutionPipeline[[diffusers.IFInpaintingSuperResolutionPipeline]]

```python
diffusers.IFInpaintingSuperResolutionPipeline(tokenizer: T5Tokenizer, text_encoder: T5EncoderModel, unet: UNet2DConditionModel, scheduler: DDPMScheduler, image_noising_scheduler: DDPMScheduler, safety_checker: diffusers.pipelines.deepfloyd_if.safety_checker.IFSafetyChecker | None, feature_extractor: transformers.models.clip.image_processing_pil_clip.CLIPImageProcessorPil | None, watermarker: diffusers.pipelines.deepfloyd_if.watermark.IFWatermarker | None, requires_safety_checker: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/deepfloyd_if/pipeline_if_inpainting_superresolution.py#L126)

#### __call__[[diffusers.IFInpaintingSuperResolutionPipeline.__call__]]

```python
__call__(image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor], original_image: typing.Union[PIL.Image.Image, torch.Tensor, numpy.ndarray, list[PIL.Image.Image], list[torch.Tensor], list[numpy.ndarray]] = None, mask_image: typing.Union[PIL.Image.Image, torch.Tensor, numpy.ndarray, list[PIL.Image.Image], list[torch.Tensor], list[numpy.ndarray]] = None, strength: float = 0.8, prompt: str | list[str] = None, num_inference_steps: int = 100, timesteps: list = None, guidance_scale: float = 4.0, negative_prompt: str | list[str] | None = None, num_images_per_prompt: int | None = 1, eta: float = 0.0, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, callback: typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None, callback_steps: int = 1, cross_attention_kwargs: dict[str, typing.Any] | None = None, noise_level: int = 0, clean_caption: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/deepfloyd_if/pipeline_if_inpainting_superresolution.py#L832)

**Parameters:**

image (`torch.Tensor` or `PIL.Image.Image`) : `Image`, or tensor representing an image batch, that will be used as the starting point for the process.

original_image (`torch.Tensor` or `PIL.Image.Image`) : The original image that `image` was varied from.

mask_image (`PIL.Image.Image`) : `Image`, or tensor representing an image batch, to mask `image`. White pixels in the mask will be repainted, while black pixels will be preserved. If `mask_image` is a PIL image, it will be converted to a single channel (luminance) before use. If it's a tensor, it should contain one color channel (L) instead of 3, so the expected shape would be `(B, H, W, 1)`.

strength (`float`, *optional*, defaults to 0.8) : Conceptually, indicates how much to transform the reference `image`. Must be between 0 and 1. `image` will be used as a starting point, adding more noise to it the larger the `strength`. The number of denoising steps depends on the amount of noise initially added. When `strength` is 1, added noise will be maximum and the denoising process will run for the full number of iterations specified in `num_inference_steps`. A value of 1, therefore, essentially ignores `image`.

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

timesteps (`list[int]`, *optional*) : Custom timesteps to use for the denoising process. If not defined, equal spaced `num_inference_steps` timesteps are used. Must be in descending order.

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

eta (`float`, *optional*, defaults to 0.0) : Corresponds to parameter eta (η) in the DDIM paper: https://huggingface.co/papers/2010.02502. Only applies to [schedulers.DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), will be ignored for others.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.stable_diffusion.IFPipelineOutput` instead of a plain tuple.

callback (`Callable`, *optional*) : A function that will be called every `callback_steps` steps during inference. The function will be called with the following arguments: `callback(step: int, timestep: int, latents: torch.Tensor)`.

callback_steps (`int`, *optional*, defaults to 1) : The frequency at which the `callback` function will be called. If not specified, the callback will be called at every step.

cross_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

noise_level (`int`, *optional*, defaults to 0) : The amount of noise to add to the upscaled image. Must be in the range `[0, 1000)`

clean_caption (`bool`, *optional*, defaults to `True`) : Whether or not to clean the caption before creating embeddings. Requires `beautifulsoup4` and `ftfy` to be installed. If the dependencies are not installed, the embeddings will be created from the raw prompt.

**Returns:** `~pipelines.stable_diffusion.IFPipelineOutput` or `tuple`

`~pipelines.stable_diffusion.IFPipelineOutput` if `return_dict` is True, otherwise a `tuple. When
returning a tuple, the first element is a list with the generated images, and the second element is a list
of `bool`s denoting whether the corresponding generated image likely represents "not-safe-for-work" (nsfw)
or watermarked content, according to the `safety_checker`.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> from diffusers import IFInpaintingPipeline, IFInpaintingSuperResolutionPipeline, DiffusionPipeline
>>> from diffusers.utils import pt_to_pil
>>> import torch
>>> from PIL import Image
>>> import requests
>>> from io import BytesIO

>>> url = "https://huggingface.co/datasets/diffusers/docs-images/resolve/main/if/person.png"
>>> response = requests.get(url)
>>> original_image = Image.open(BytesIO(response.content)).convert("RGB")
>>> original_image = original_image

>>> url = "https://huggingface.co/datasets/diffusers/docs-images/resolve/main/if/glasses_mask.png"
>>> response = requests.get(url)
>>> mask_image = Image.open(BytesIO(response.content))
>>> mask_image = mask_image

>>> pipe = IFInpaintingPipeline.from_pretrained(
...     "DeepFloyd/IF-I-XL-v1.0", variant="fp16", torch_dtype=torch.float16
... )
>>> pipe.enable_model_cpu_offload()

>>> prompt = "blue sunglasses"

>>> prompt_embeds, negative_embeds = pipe.encode_prompt(prompt)
>>> image = pipe(
...     image=original_image,
...     mask_image=mask_image,
...     prompt_embeds=prompt_embeds,
...     negative_prompt_embeds=negative_embeds,
...     output_type="pt",
... ).images

>>> # save intermediate image
>>> pil_image = pt_to_pil(image)
>>> pil_image[0].save("./if_stage_I.png")

>>> super_res_1_pipe = IFInpaintingSuperResolutionPipeline.from_pretrained(
...     "DeepFloyd/IF-II-L-v1.0", text_encoder=None, variant="fp16", torch_dtype=torch.float16
... )
>>> super_res_1_pipe.enable_model_cpu_offload()

>>> image = super_res_1_pipe(
...     image=image,
...     mask_image=mask_image,
...     original_image=original_image,
...     prompt_embeds=prompt_embeds,
...     negative_prompt_embeds=negative_embeds,
... ).images
>>> image[0].save("./if_stage_II.png")
```

#### encode_prompt[[diffusers.IFInpaintingSuperResolutionPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], do_classifier_free_guidance: bool = True, num_images_per_prompt: int = 1, device: typing.Optional[torch.device] = None, negative_prompt: str | list[str] | None = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, clean_caption: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/deepfloyd_if/pipeline_if_inpainting_superresolution.py#L346)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : whether to use classifier free guidance or not

num_images_per_prompt (`int`, *optional*, defaults to 1) : number of images that should be generated per prompt

device : (`torch.device`, *optional*): torch device to place the resulting embeddings on

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds`. instead. If not defined, one has to pass `negative_prompt_embeds`. instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

clean_caption (bool, defaults to `False`) : If `True`, the function will preprocess and clean the provided caption before encoding.

Encodes the prompt into text encoder hidden states.
