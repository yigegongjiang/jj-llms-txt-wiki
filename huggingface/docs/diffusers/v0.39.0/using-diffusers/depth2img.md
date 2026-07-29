# Text-guided depth-to-image generation

The [StableDiffusionDepth2ImgPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/depth2img#diffusers.StableDiffusionDepth2ImgPipeline) lets you pass a text prompt and an initial image to condition the generation of new images. In addition, you can also pass a `depth_map` to preserve the image structure. If no `depth_map` is provided, the pipeline automatically predicts the depth via an integrated [depth-estimation model](https://github.com/isl-org/MiDaS).

Start by creating an instance of the [StableDiffusionDepth2ImgPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/depth2img#diffusers.StableDiffusionDepth2ImgPipeline):

```python
import torch
from diffusers import StableDiffusionDepth2ImgPipeline
from diffusers.utils import load_image, make_image_grid

pipeline = StableDiffusionDepth2ImgPipeline.from_pretrained(
    "stabilityai/stable-diffusion-2-depth",
    torch_dtype=torch.float16,
    use_safetensors=True,
).to("cuda")
```

Now pass your prompt to the pipeline. You can also pass a `negative_prompt` to prevent certain words from guiding how an image is generated:

```python
url = "http://images.cocodataset.org/val2017/000000039769.jpg"
init_image = load_image(url)
prompt = "two tigers"
negative_prompt = "bad, deformed, ugly, bad anatomy"
image = pipeline(prompt=prompt, image=init_image, negative_prompt=negative_prompt, strength=0.7).images[0]
make_image_grid([init_image, image], rows=1, cols=2)
```

| Input                                                                           | Output                                                                                                                                |
|---------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------|
|  |  |
