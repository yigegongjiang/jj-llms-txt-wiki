# Ernie-Image

  

[ERNIE-Image] is a powerful and highly efficient image generation model with 8B parameters. Currently there's only two models to be released:

|Model|Hugging Face|
|---|---|
|ERNIE-Image|https://huggingface.co/baidu/ERNIE-Image|
|ERNIE-Image-Turbo|https://huggingface.co/baidu/ERNIE-Image-Turbo|

## ERNIE-Image

ERNIE-Image is designed with a relatively compact architecture and solid instruction-following capability, emphasizing parameter efficiency. Based on an 8B DiT backbone, it provides performance that is comparable in some scenarios to larger (20B+) models, while maintaining reasonable parameter efficiency. It offers a relatively stable level of performance in instruction understanding and execution, text generation (e.g., English / Chinese / Japanese), and overall stability.

## ERNIE-Image-Turbo

ERNIE-Image-Turbo is a distilled variant of ERNIE-Image, requiring only 8 NFEs (Number of Function Evaluations) and offering a more efficient alternative with relatively comparable performance to the full model in certain cases.

## ErnieImagePipeline

Use [ErnieImagePipeline] to generate images from text prompts. The pipeline supports Prompt Enhancer (PE) by default, which enhances the user’s raw prompt to improve output quality, though it may reduce instruction-following accuracy.

We provide a pretrained 3B-parameter PE model; however, using larger language models (e.g., Gemini or ChatGPT) for prompt enhancement may yield better results. The system prompt template is available at: https://huggingface.co/baidu/ERNIE-Image/blob/main/pe/chat_template.jinja.

If you prefer not to use PE, set use_pe=False.

```python
import torch
from diffusers import ErnieImagePipeline
from diffusers.utils import load_image

pipe = ErnieImagePipeline.from_pretrained("baidu/ERNIE-Image", dtype=torch.bfloat16)
pipe.to("cuda")
# If you are running low on GPU VRAM, you can enable offloading
pipe.enable_model_cpu_offload()

prompt = "一只黑白相间的中华田园犬"
images = pipe(
    prompt=prompt,
    height=1024,
    width=1024,
    num_inference_steps=50,
    guidance_scale=4.0,
    generator=torch.Generator("cuda").manual_seed(42),
    use_pe=True,
).images
images[0].save("ernie-image-output.png")
```

```python
import torch
from diffusers import ErnieImagePipeline
from diffusers.utils import load_image

pipe = ErnieImagePipeline.from_pretrained("baidu/ERNIE-Image-Turbo", dtype=torch.bfloat16)
pipe.to("cuda")
# If you are running low on GPU VRAM, you can enable offloading
pipe.enable_model_cpu_offload()

prompt = "一只黑白相间的中华田园犬"
images = pipe(
    prompt=prompt,
    height=1024,
    width=1024,
    num_inference_steps=8,
    guidance_scale=1.0,
    generator=torch.Generator("cuda").manual_seed(42),
    use_pe=True,
).images
images[0].save("ernie-image-turbo-output.png")
```
