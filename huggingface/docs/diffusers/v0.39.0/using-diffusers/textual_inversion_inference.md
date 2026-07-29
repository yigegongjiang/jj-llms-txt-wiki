# Textual Inversion

[Textual Inversion](https://huggingface.co/papers/2208.01618) is a method for generating personalized images of a concept. It works by fine-tuning a models word embeddings on 3-5 images of the concept (for example, pixel art) that is associated with a unique token (`<sks>`). This allows you to use the `<sks>` token in your prompt to trigger the model to generate pixel art images.

Textual Inversion weights are very lightweight and typically only a few KBs because they're only word embeddings. However, this also means the word embeddings need to be loaded after loading a model with [from_pretrained()](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline.from_pretrained).

```py
import torch
from diffusers import AutoPipelineForText2Image

pipeline = AutoPipelineForText2Image.from_pretrained(
    "stable-diffusion-v1-5/stable-diffusion-v1-5",
    torch_dtype=torch.float16
).to("cuda")
```

Load the word embeddings with [load_textual_inversion()](/docs/diffusers/v0.39.0/en/api/loaders/textual_inversion#diffusers.loaders.TextualInversionLoaderMixin.load_textual_inversion) and include the unique token in the prompt to activate its generation.

```py
pipeline.load_textual_inversion("sd-concepts-library/gta5-artwork")
prompt = "A cute brown bear eating a slice of pizza, stunning color scheme, masterpiece, illustration, <gta5-artwork> style"
pipeline(prompt).images[0]
```

    

Textual Inversion can also be trained to learn *negative embeddings* to steer generation away from unwanted characteristics such as "blurry" or "ugly". It is useful for improving image quality.

EasyNegative is a widely used negative embedding that contains multiple learned negative concepts. Load the negative embeddings and specify the file name and token associated with the negative embeddings. Pass the token to `negative_prompt` in your pipeline to activate it.

```py
import torch
from diffusers import AutoPipelineForText2Image

pipeline = AutoPipelineForText2Image.from_pretrained(
    "stable-diffusion-v1-5/stable-diffusion-v1-5",
    torch_dtype=torch.float16
).to("cuda")
pipeline.load_textual_inversion(
    "EvilEngine/easynegative",
    weight_name="easynegative.safetensors",
    token="easynegative"
)
prompt = "A cute brown bear eating a slice of pizza, stunning color scheme, masterpiece, illustration"
negative_prompt = "easynegative"
pipeline(prompt, negative_prompt).images[0]
```
