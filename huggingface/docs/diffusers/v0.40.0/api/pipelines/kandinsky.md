# Kandinsky 2.1

Kandinsky 2.1 is created by [Arseniy Shakhmatov](https://github.com/cene555), [Anton Razzhigaev](https://github.com/razzant), [Aleksandr Nikolich](https://github.com/AlexWortega), [Vladimir Arkhipkin](https://github.com/oriBetelgeuse), [Igor Pavlov](https://github.com/boomb0om), [Andrey Kuznetsov](https://github.com/kuznetsoffandrey), and [Denis Dimitrov](https://github.com/denndimitrov).

The description from it's GitHub page is:

*Kandinsky 2.1 inherits best practicies from Dall-E 2 and Latent diffusion, while introducing some new ideas. As text and image encoder it uses CLIP model and diffusion image prior (mapping) between latent spaces of CLIP modalities. This approach increases the visual performance of the model and unveils new horizons in blending images and text-guided image manipulation.*

The original codebase can be found at [ai-forever/Kandinsky-2](https://github.com/ai-forever/Kandinsky-2).

> [!TIP]
> Check out the [Kandinsky Community](https://huggingface.co/kandinsky-community) organization on the Hub for the official model checkpoints for tasks like text-to-image, image-to-image, and inpainting.

> [!TIP]
> Make sure to check out the Schedulers [guide](../../using-diffusers/schedulers) to learn how to explore the tradeoff between scheduler speed and quality, and see the [reuse components across pipelines](../../using-diffusers/loading#reuse-a-pipeline) section to learn how to efficiently load the same components into multiple pipelines.

Make sure you have the following libraries installed.

```py
# uncomment to install the necessary libraries in Colab
#!pip install -q diffusers transformers accelerate
```

> [!WARNING]
> Kandinsky 2.1 and 2.2 usage is very similar! The only difference is Kandinsky 2.2 doesn't accept `prompt` as an input when decoding the latents. Instead, Kandinsky 2.2 only accepts `image_embeds` during decoding.
>
> 
>
> Kandinsky 3 has a more concise architecture and it doesn't require a prior model. This means it's usage is identical to other diffusion models like [Stable Diffusion XL](./stable_diffusion/stable_diffusion_xl).

## Text-to-image

To use the Kandinsky models for any task, you always start by setting up the prior pipeline to encode the prompt and generate the image embeddings. The prior pipeline also generates `negative_image_embeds` that correspond to the negative prompt `""`. For better results, you can pass an actual `negative_prompt` to the prior pipeline, but this'll increase the effective batch size of the prior pipeline by 2x.

```py
from diffusers import KandinskyPriorPipeline, KandinskyPipeline
import torch

prior_pipeline = KandinskyPriorPipeline.from_pretrained("kandinsky-community/kandinsky-2-1-prior", dtype=torch.float16).to("cuda")
pipeline = KandinskyPipeline.from_pretrained("kandinsky-community/kandinsky-2-1", dtype=torch.float16).to("cuda")

prompt = "A alien cheeseburger creature eating itself, claymation, cinematic, moody lighting"
negative_prompt = "low quality, bad quality" # optional to include a negative prompt, but results are usually better
image_embeds, negative_image_embeds = prior_pipeline(prompt, negative_prompt, guidance_scale=1.0).to_tuple()
```

Now pass all the prompts and embeddings to the [KandinskyPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/kandinsky#diffusers.KandinskyPipeline) to generate an image:

```py
image = pipeline(prompt, image_embeds=image_embeds, negative_prompt=negative_prompt, negative_image_embeds=negative_image_embeds, height=768, width=768).images[0]
image
```

    

```py
from diffusers import KandinskyV22PriorPipeline, KandinskyV22Pipeline
import torch

prior_pipeline = KandinskyV22PriorPipeline.from_pretrained("kandinsky-community/kandinsky-2-2-prior", dtype=torch.float16).to("cuda")
pipeline = KandinskyV22Pipeline.from_pretrained("kandinsky-community/kandinsky-2-2-decoder", dtype=torch.float16).to("cuda")

prompt = "A alien cheeseburger creature eating itself, claymation, cinematic, moody lighting"
negative_prompt = "low quality, bad quality" # optional to include a negative prompt, but results are usually better
image_embeds, negative_image_embeds = prior_pipeline(prompt, guidance_scale=1.0).to_tuple()
```

Pass the `image_embeds` and `negative_image_embeds` to the [KandinskyV22Pipeline](/docs/diffusers/v0.40.0/en/api/pipelines/kandinsky_v22#diffusers.KandinskyV22Pipeline) to generate an image:

```py
image = pipeline(image_embeds=image_embeds, negative_image_embeds=negative_image_embeds, height=768, width=768).images[0]
image
```

    

Kandinsky 3 doesn't require a prior model so you can directly load the [Kandinsky3Pipeline](/docs/diffusers/v0.40.0/en/api/pipelines/kandinsky3#diffusers.Kandinsky3Pipeline) and pass a prompt to generate an image:

```py
from diffusers import Kandinsky3Pipeline
import torch

pipeline = Kandinsky3Pipeline.from_pretrained("kandinsky-community/kandinsky-3", variant="fp16", dtype=torch.float16)
pipeline.enable_model_cpu_offload()

prompt = "A alien cheeseburger creature eating itself, claymation, cinematic, moody lighting"
image = pipeline(prompt).images[0]
image
```

🤗 Diffusers also provides an end-to-end API with the [KandinskyCombinedPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/kandinsky#diffusers.KandinskyCombinedPipeline) and [KandinskyV22CombinedPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/kandinsky_v22#diffusers.KandinskyV22CombinedPipeline), meaning you don't have to separately load the prior and text-to-image pipeline. The combined pipeline automatically loads both the prior model and the decoder. You can still set different values for the prior pipeline with the `prior_guidance_scale` and `prior_num_inference_steps` parameters if you want.

Use the [AutoPipelineForText2Image](/docs/diffusers/v0.40.0/en/api/pipelines/auto_pipeline#diffusers.AutoPipelineForText2Image) to automatically call the combined pipelines under the hood:

```py
from diffusers import AutoPipelineForText2Image
import torch

pipeline = AutoPipelineForText2Image.from_pretrained("kandinsky-community/kandinsky-2-1", dtype=torch.float16)
pipeline.enable_model_cpu_offload()

prompt = "A alien cheeseburger creature eating itself, claymation, cinematic, moody lighting"
negative_prompt = "low quality, bad quality"

image = pipeline(prompt=prompt, negative_prompt=negative_prompt, prior_guidance_scale=1.0, guidance_scale=4.0, height=768, width=768).images[0]
image
```

```py
from diffusers import AutoPipelineForText2Image
import torch

pipeline = AutoPipelineForText2Image.from_pretrained("kandinsky-community/kandinsky-2-2-decoder", dtype=torch.float16)
pipeline.enable_model_cpu_offload()

prompt = "A alien cheeseburger creature eating itself, claymation, cinematic, moody lighting"
negative_prompt = "low quality, bad quality"

image = pipeline(prompt=prompt, negative_prompt=negative_prompt, prior_guidance_scale=1.0, guidance_scale=4.0, height=768, width=768).images[0]
image
```

## Image-to-image

For image-to-image, pass the initial image and text prompt to condition the image to the pipeline. Start by loading the prior pipeline:

```py
import torch
from diffusers import KandinskyImg2ImgPipeline, KandinskyPriorPipeline

prior_pipeline = KandinskyPriorPipeline.from_pretrained("kandinsky-community/kandinsky-2-1-prior", dtype=torch.float16, use_safetensors=True).to("cuda")
pipeline = KandinskyImg2ImgPipeline.from_pretrained("kandinsky-community/kandinsky-2-1", dtype=torch.float16, use_safetensors=True).to("cuda")
```

```py
import torch
from diffusers import KandinskyV22Img2ImgPipeline, KandinskyPriorPipeline

prior_pipeline = KandinskyPriorPipeline.from_pretrained("kandinsky-community/kandinsky-2-2-prior", dtype=torch.float16, use_safetensors=True).to("cuda")
pipeline = KandinskyV22Img2ImgPipeline.from_pretrained("kandinsky-community/kandinsky-2-2-decoder", dtype=torch.float16, use_safetensors=True).to("cuda")
```

Kandinsky 3 doesn't require a prior model so you can directly load the image-to-image pipeline:

```py
from diffusers import Kandinsky3Img2ImgPipeline
from diffusers.utils import load_image
import torch

pipeline = Kandinsky3Img2ImgPipeline.from_pretrained("kandinsky-community/kandinsky-3", variant="fp16", dtype=torch.float16)
pipeline.enable_model_cpu_offload()
```

Download an image to condition on:

```py
from diffusers.utils import load_image

# download image
url = "https://raw.githubusercontent.com/CompVis/stable-diffusion/main/assets/stable-samples/img2img/sketch-mountains-input.jpg"
original_image = load_image(url)
original_image = original_image.resize((768, 512))
```

    

Generate the `image_embeds` and `negative_image_embeds` with the prior pipeline:

```py
prompt = "A fantasy landscape, Cinematic lighting"
negative_prompt = "low quality, bad quality"

image_embeds, negative_image_embeds = prior_pipeline(prompt, negative_prompt).to_tuple()
```

Now pass the original image, and all the prompts and embeddings to the pipeline to generate an image:

```py
from diffusers.utils import make_image_grid

image = pipeline(prompt, negative_prompt=negative_prompt, image=original_image, image_embeds=image_embeds, negative_image_embeds=negative_image_embeds, height=768, width=768, strength=0.3).images[0]
make_image_grid([original_image.resize((512, 512)), image.resize((512, 512))], rows=1, cols=2)
```

    

```py
from diffusers.utils import make_image_grid

image = pipeline(image=original_image, image_embeds=image_embeds, negative_image_embeds=negative_image_embeds, height=768, width=768, strength=0.3).images[0]
make_image_grid([original_image.resize((512, 512)), image.resize((512, 512))], rows=1, cols=2)
```

    

```py
image = pipeline(prompt, negative_prompt=negative_prompt, image=image, strength=0.75, num_inference_steps=25).images[0]
image
```

🤗 Diffusers also provides an end-to-end API with the [KandinskyImg2ImgCombinedPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/kandinsky#diffusers.KandinskyImg2ImgCombinedPipeline) and [KandinskyV22Img2ImgCombinedPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/kandinsky_v22#diffusers.KandinskyV22Img2ImgCombinedPipeline), meaning you don't have to separately load the prior and image-to-image pipeline. The combined pipeline automatically loads both the prior model and the decoder. You can still set different values for the prior pipeline with the `prior_guidance_scale` and `prior_num_inference_steps` parameters if you want.

Use the [AutoPipelineForImage2Image](/docs/diffusers/v0.40.0/en/api/pipelines/auto_pipeline#diffusers.AutoPipelineForImage2Image) to automatically call the combined pipelines under the hood:

```py
from diffusers import AutoPipelineForImage2Image
from diffusers.utils import make_image_grid, load_image
import torch

pipeline = AutoPipelineForImage2Image.from_pretrained("kandinsky-community/kandinsky-2-1", dtype=torch.float16, use_safetensors=True)
pipeline.enable_model_cpu_offload()

prompt = "A fantasy landscape, Cinematic lighting"
negative_prompt = "low quality, bad quality"

url = "https://raw.githubusercontent.com/CompVis/stable-diffusion/main/assets/stable-samples/img2img/sketch-mountains-input.jpg"
original_image = load_image(url)

original_image.thumbnail((768, 768))

image = pipeline(prompt=prompt, negative_prompt=negative_prompt, image=original_image, strength=0.3).images[0]
make_image_grid([original_image.resize((512, 512)), image.resize((512, 512))], rows=1, cols=2)
```

```py
from diffusers import AutoPipelineForImage2Image
from diffusers.utils import make_image_grid, load_image
import torch

pipeline = AutoPipelineForImage2Image.from_pretrained("kandinsky-community/kandinsky-2-2-decoder", dtype=torch.float16)
pipeline.enable_model_cpu_offload()

prompt = "A fantasy landscape, Cinematic lighting"
negative_prompt = "low quality, bad quality"

url = "https://raw.githubusercontent.com/CompVis/stable-diffusion/main/assets/stable-samples/img2img/sketch-mountains-input.jpg"
original_image = load_image(url)

original_image.thumbnail((768, 768))

image = pipeline(prompt=prompt, negative_prompt=negative_prompt, image=original_image, strength=0.3).images[0]
make_image_grid([original_image.resize((512, 512)), image.resize((512, 512))], rows=1, cols=2)
```

## Inpainting

> [!WARNING]
> ⚠️ The Kandinsky models use ⬜️ **white pixels** to represent the masked area now instead of black pixels. If you are using [KandinskyInpaintPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/kandinsky#diffusers.KandinskyInpaintPipeline) in production, you need to change the mask to use white pixels:
>
> ```py
> # For PIL input
> import PIL.ImageOps
> mask = PIL.ImageOps.invert(mask)
>
> # For PyTorch and NumPy input
> mask = 1 - mask
> ```

For inpainting, you'll need the original image, a mask of the area to replace in the original image, and a text prompt of what to inpaint. Load the prior pipeline:

```py
from diffusers import KandinskyInpaintPipeline, KandinskyPriorPipeline
from diffusers.utils import load_image, make_image_grid
import torch
import numpy as np
from PIL import Image

prior_pipeline = KandinskyPriorPipeline.from_pretrained("kandinsky-community/kandinsky-2-1-prior", dtype=torch.float16, use_safetensors=True).to("cuda")
pipeline = KandinskyInpaintPipeline.from_pretrained("kandinsky-community/kandinsky-2-1-inpaint", dtype=torch.float16, use_safetensors=True).to("cuda")
```

```py
from diffusers import KandinskyV22InpaintPipeline, KandinskyV22PriorPipeline
from diffusers.utils import load_image, make_image_grid
import torch
import numpy as np
from PIL import Image

prior_pipeline = KandinskyV22PriorPipeline.from_pretrained("kandinsky-community/kandinsky-2-2-prior", dtype=torch.float16, use_safetensors=True).to("cuda")
pipeline = KandinskyV22InpaintPipeline.from_pretrained("kandinsky-community/kandinsky-2-2-decoder-inpaint", dtype=torch.float16, use_safetensors=True).to("cuda")
```

Load an initial image and create a mask:

```py
init_image = load_image("https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main/kandinsky/cat.png")
mask = np.zeros((768, 768), dtype=np.float32)
# mask area above cat's head
mask[:250, 250:-250] = 1
```

Generate the embeddings with the prior pipeline:

```py
prompt = "a hat"
prior_output = prior_pipeline(prompt)
```

Now pass the initial image, mask, and prompt and embeddings to the pipeline to generate an image:

```py
output_image = pipeline(prompt, image=init_image, mask_image=mask, **prior_output, height=768, width=768, num_inference_steps=150).images[0]
mask = Image.fromarray((mask*255).astype('uint8'), 'L')
make_image_grid([init_image, mask, output_image], rows=1, cols=3)
```

    

```py
output_image = pipeline(image=init_image, mask_image=mask, **prior_output, height=768, width=768, num_inference_steps=150).images[0]
mask = Image.fromarray((mask*255).astype('uint8'), 'L')
make_image_grid([init_image, mask, output_image], rows=1, cols=3)
```

    

You can also use the end-to-end [KandinskyInpaintCombinedPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/kandinsky#diffusers.KandinskyInpaintCombinedPipeline) and [KandinskyV22InpaintCombinedPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/kandinsky_v22#diffusers.KandinskyV22InpaintCombinedPipeline) to call the prior and decoder pipelines together under the hood. Use the [AutoPipelineForInpainting](/docs/diffusers/v0.40.0/en/api/pipelines/auto_pipeline#diffusers.AutoPipelineForInpainting) for this:

```py
import torch
import numpy as np
from PIL import Image
from diffusers import AutoPipelineForInpainting
from diffusers.utils import load_image, make_image_grid

pipe = AutoPipelineForInpainting.from_pretrained("kandinsky-community/kandinsky-2-1-inpaint", dtype=torch.float16)
pipe.enable_model_cpu_offload()

init_image = load_image("https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main/kandinsky/cat.png")
mask = np.zeros((768, 768), dtype=np.float32)
# mask area above cat's head
mask[:250, 250:-250] = 1
prompt = "a hat"

output_image = pipe(prompt=prompt, image=init_image, mask_image=mask).images[0]
mask = Image.fromarray((mask*255).astype('uint8'), 'L')
make_image_grid([init_image, mask, output_image], rows=1, cols=3)
```

```py
import torch
import numpy as np
from PIL import Image
from diffusers import AutoPipelineForInpainting
from diffusers.utils import load_image, make_image_grid

pipe = AutoPipelineForInpainting.from_pretrained("kandinsky-community/kandinsky-2-2-decoder-inpaint", dtype=torch.float16)
pipe.enable_model_cpu_offload()

init_image = load_image("https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main/kandinsky/cat.png")
mask = np.zeros((768, 768), dtype=np.float32)
# mask area above cat's head
mask[:250, 250:-250] = 1
prompt = "a hat"

output_image = pipe(prompt=prompt, image=original_image, mask_image=mask).images[0]
mask = Image.fromarray((mask*255).astype('uint8'), 'L')
make_image_grid([init_image, mask, output_image], rows=1, cols=3)
```

## Interpolation

Interpolation allows you to explore the latent space between the image and text embeddings which is a cool way to see some of the prior model's intermediate outputs. Load the prior pipeline and two images you'd like to interpolate:

```py
from diffusers import KandinskyPriorPipeline, KandinskyPipeline
from diffusers.utils import load_image, make_image_grid
import torch

prior_pipeline = KandinskyPriorPipeline.from_pretrained("kandinsky-community/kandinsky-2-1-prior", dtype=torch.float16, use_safetensors=True).to("cuda")
img_1 = load_image("https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main/kandinsky/cat.png")
img_2 = load_image("https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main/kandinsky/starry_night.jpeg")
make_image_grid([img_1.resize((512,512)), img_2.resize((512,512))], rows=1, cols=2)
```

```py
from diffusers import KandinskyV22PriorPipeline, KandinskyV22Pipeline
from diffusers.utils import load_image, make_image_grid
import torch

prior_pipeline = KandinskyV22PriorPipeline.from_pretrained("kandinsky-community/kandinsky-2-2-prior", dtype=torch.float16, use_safetensors=True).to("cuda")
img_1 = load_image("https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main/kandinsky/cat.png")
img_2 = load_image("https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main/kandinsky/starry_night.jpeg")
make_image_grid([img_1.resize((512,512)), img_2.resize((512,512))], rows=1, cols=2)
```

  
    
    a cat
  
  
    
    Van Gogh's Starry Night painting
  

Specify the text or images to interpolate, and set the weights for each text or image. Experiment with the weights to see how they affect the interpolation!

```py
images_texts = ["a cat", img_1, img_2]
weights = [0.3, 0.3, 0.4]
```

Call the `interpolate` function to generate the embeddings, and then pass them to the pipeline to generate the image:

```py
# prompt can be left empty
prompt = ""
prior_out = prior_pipeline.interpolate(images_texts, weights)

pipeline = KandinskyPipeline.from_pretrained("kandinsky-community/kandinsky-2-1", dtype=torch.float16, use_safetensors=True).to("cuda")

image = pipeline(prompt, **prior_out, height=768, width=768).images[0]
image
```

    

```py
# prompt can be left empty
prompt = ""
prior_out = prior_pipeline.interpolate(images_texts, weights)

pipeline = KandinskyV22Pipeline.from_pretrained("kandinsky-community/kandinsky-2-2-decoder", dtype=torch.float16, use_safetensors=True).to("cuda")

image = pipeline(prompt, **prior_out, height=768, width=768).images[0]
image
```

    

## ControlNet

> [!WARNING]
> ⚠️ ControlNet is only supported for Kandinsky 2.2!

ControlNet enables conditioning large pretrained diffusion models with additional inputs such as a depth map or edge detection. For example, you can condition Kandinsky 2.2 with a depth map so the model understands and preserves the structure of the depth image.

Let's load an image and extract it's depth map:

```py
from diffusers.utils import load_image

img = load_image(
    "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main/kandinskyv22/cat.png"
).resize((768, 768))
img
```

    

Then you can use the `depth-estimation` [Pipeline](https://huggingface.co/docs/transformers/v5.15.1/en/main_classes/pipelines#transformers.Pipeline) from 🤗 Transformers to process the image and retrieve the depth map:

```py
import torch
import numpy as np

from transformers import pipeline

def make_hint(image, depth_estimator):
    image = depth_estimator(image)["depth"]
    image = np.array(image)
    image = image[:, :, None]
    image = np.concatenate([image, image, image], axis=2)
    detected_map = torch.from_numpy(image).float() / 255.0
    hint = detected_map.permute(2, 0, 1)
    return hint

depth_estimator = pipeline("depth-estimation")
hint = make_hint(img, depth_estimator).unsqueeze(0).half().to("cuda")
```

### Text-to-image [[controlnet-text-to-image]]

Load the prior pipeline and the [KandinskyV22ControlnetPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/kandinsky_v22#diffusers.KandinskyV22ControlnetPipeline):

```py
from diffusers import KandinskyV22PriorPipeline, KandinskyV22ControlnetPipeline

prior_pipeline = KandinskyV22PriorPipeline.from_pretrained(
    "kandinsky-community/kandinsky-2-2-prior", dtype=torch.float16, use_safetensors=True
).to("cuda")

pipeline = KandinskyV22ControlnetPipeline.from_pretrained(
    "kandinsky-community/kandinsky-2-2-controlnet-depth", dtype=torch.float16
).to("cuda")
```

Generate the image embeddings from a prompt and negative prompt:

```py
prompt = "A robot, 4k photo"
negative_prior_prompt = "lowres, text, error, cropped, worst quality, low quality, jpeg artifacts, ugly, duplicate, morbid, mutilated, out of frame, extra fingers, mutated hands, poorly drawn hands, poorly drawn face, mutation, deformed, blurry, dehydrated, bad anatomy, bad proportions, extra limbs, cloned face, disfigured, gross proportions, malformed limbs, missing arms, missing legs, extra arms, extra legs, fused fingers, too many fingers, long neck, username, watermark, signature"

generator = torch.Generator(device="cuda").manual_seed(43)

image_emb, zero_image_emb = prior_pipeline(
    prompt=prompt, negative_prompt=negative_prior_prompt, generator=generator
).to_tuple()
```

Finally, pass the image embeddings and the depth image to the [KandinskyV22ControlnetPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/kandinsky_v22#diffusers.KandinskyV22ControlnetPipeline) to generate an image:

```py
image = pipeline(image_embeds=image_emb, negative_image_embeds=zero_image_emb, hint=hint, num_inference_steps=50, generator=generator, height=768, width=768).images[0]
image
```

    

### Image-to-image [[controlnet-image-to-image]]

For image-to-image with ControlNet, you'll need to use the:

- [KandinskyV22PriorEmb2EmbPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/kandinsky_v22#diffusers.KandinskyV22PriorEmb2EmbPipeline) to generate the image embeddings from a text prompt and an image
- [KandinskyV22ControlnetImg2ImgPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/kandinsky_v22#diffusers.KandinskyV22ControlnetImg2ImgPipeline) to generate an image from the initial image and the image embeddings

Process and extract a depth map of an initial image of a cat with the `depth-estimation` [Pipeline](https://huggingface.co/docs/transformers/v5.15.1/en/main_classes/pipelines#transformers.Pipeline) from 🤗 Transformers:

```py
import torch
import numpy as np

from diffusers import KandinskyV22PriorEmb2EmbPipeline, KandinskyV22ControlnetImg2ImgPipeline
from diffusers.utils import load_image
from transformers import pipeline

img = load_image(
    "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main/kandinskyv22/cat.png"
).resize((768, 768))

def make_hint(image, depth_estimator):
    image = depth_estimator(image)["depth"]
    image = np.array(image)
    image = image[:, :, None]
    image = np.concatenate([image, image, image], axis=2)
    detected_map = torch.from_numpy(image).float() / 255.0
    hint = detected_map.permute(2, 0, 1)
    return hint

depth_estimator = pipeline("depth-estimation")
hint = make_hint(img, depth_estimator).unsqueeze(0).half().to("cuda")
```

Load the prior pipeline and the [KandinskyV22ControlnetImg2ImgPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/kandinsky_v22#diffusers.KandinskyV22ControlnetImg2ImgPipeline):

```py
prior_pipeline = KandinskyV22PriorEmb2EmbPipeline.from_pretrained(
    "kandinsky-community/kandinsky-2-2-prior", dtype=torch.float16, use_safetensors=True
).to("cuda")

pipeline = KandinskyV22ControlnetImg2ImgPipeline.from_pretrained(
    "kandinsky-community/kandinsky-2-2-controlnet-depth", dtype=torch.float16
).to("cuda")
```

Pass a text prompt and the initial image to the prior pipeline to generate the image embeddings:

```py
prompt = "A robot, 4k photo"
negative_prior_prompt = "lowres, text, error, cropped, worst quality, low quality, jpeg artifacts, ugly, duplicate, morbid, mutilated, out of frame, extra fingers, mutated hands, poorly drawn hands, poorly drawn face, mutation, deformed, blurry, dehydrated, bad anatomy, bad proportions, extra limbs, cloned face, disfigured, gross proportions, malformed limbs, missing arms, missing legs, extra arms, extra legs, fused fingers, too many fingers, long neck, username, watermark, signature"

generator = torch.Generator(device="cuda").manual_seed(43)

img_emb = prior_pipeline(prompt=prompt, image=img, strength=0.85, generator=generator)
negative_emb = prior_pipeline(prompt=negative_prior_prompt, image=img, strength=1, generator=generator)
```

Now you can run the [KandinskyV22ControlnetImg2ImgPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/kandinsky_v22#diffusers.KandinskyV22ControlnetImg2ImgPipeline) to generate an image from the initial image and the image embeddings:

```py
image = pipeline(image=img, strength=0.5, image_embeds=img_emb.image_embeds, negative_image_embeds=negative_emb.image_embeds, hint=hint, num_inference_steps=50, generator=generator, height=768, width=768).images[0]
make_image_grid([img.resize((512, 512)), image.resize((512, 512))], rows=1, cols=2)
```

    

## Optimizations

Kandinsky is unique because it requires a prior pipeline to generate the mappings, and a second pipeline to decode the latents into an image. Optimization efforts should be focused on the second pipeline because that is where the bulk of the computation is done. Here are some tips to improve Kandinsky during inference.

1. Enable [xFormers](../../optimization/xformers) if you're using PyTorch < 2.0:

```diff
  from diffusers import DiffusionPipeline
  import torch

  pipe = DiffusionPipeline.from_pretrained("kandinsky-community/kandinsky-2-1", dtype=torch.float16)
+ pipe.enable_xformers_memory_efficient_attention()
```

2. Enable `torch.compile` if you're using PyTorch >= 2.0 to automatically use scaled dot-product attention (SDPA):

```diff
  pipe.unet.to(memory_format=torch.channels_last)
+ pipe.unet = torch.compile(pipe.unet, mode="reduce-overhead", fullgraph=True)
```

This is the same as explicitly setting the attention processor to use [AttnAddedKVProcessor2_0](/docs/diffusers/v0.40.0/en/api/attnprocessor#diffusers.models.attention_processor.AttnAddedKVProcessor2_0):

```py
from diffusers.models.attention_processor import AttnAddedKVProcessor2_0

pipe.unet.set_attn_processor(AttnAddedKVProcessor2_0())
```

3. Offload the model to the CPU with [enable_model_cpu_offload()](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline.enable_model_cpu_offload) to avoid out-of-memory errors:

```diff
  from diffusers import DiffusionPipeline
  import torch

  pipe = DiffusionPipeline.from_pretrained("kandinsky-community/kandinsky-2-1", dtype=torch.float16)
+ pipe.enable_model_cpu_offload()
```

4. By default, the text-to-image pipeline uses the [DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler) but you can replace it with another scheduler like [DDPMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddpm#diffusers.DDPMScheduler) to see how that affects the tradeoff between inference speed and image quality:

```py
from diffusers import DDPMScheduler
from diffusers import DiffusionPipeline

scheduler = DDPMScheduler.from_pretrained("kandinsky-community/kandinsky-2-1", subfolder="ddpm_scheduler")
pipe = DiffusionPipeline.from_pretrained("kandinsky-community/kandinsky-2-1", scheduler=scheduler, dtype=torch.float16, use_safetensors=True).to("cuda")
```

## KandinskyPriorPipeline[[diffusers.KandinskyPriorPipeline]]

#### diffusers.KandinskyPriorPipeline[[diffusers.KandinskyPriorPipeline]]

```python
diffusers.KandinskyPriorPipeline(prior: PriorTransformer, image_encoder: CLIPVisionModelWithProjection, text_encoder: CLIPTextModelWithProjection, tokenizer: CLIPTokenizer, scheduler: UnCLIPScheduler, image_processor: CLIPImageProcessorPil)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky/pipeline_kandinsky_prior.py#L136)

**Parameters:**

prior ([PriorTransformer](/docs/diffusers/v0.40.0/en/api/models/prior_transformer#diffusers.PriorTransformer)) : The canonical unCLIP prior to approximate the image embedding from the text embedding.

image_encoder (`CLIPVisionModelWithProjection`) : Frozen image-encoder.

text_encoder (`CLIPTextModelWithProjection`) : Frozen text-encoder.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

scheduler (`UnCLIPScheduler`) : A scheduler to be used in combination with `prior` to generate image embedding.

Pipeline for generating image prior for Kandinsky

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.KandinskyPriorPipeline.__call__]]

```python
__call__(prompt: str | list[str], negative_prompt: str | list[str] | None = None, num_images_per_prompt: int = 1, num_inference_steps: int = 25, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, guidance_scale: float = 4.0, output_type: str | None = 'pt', return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky/pipeline_kandinsky_prior.py#L405)

**Parameters:**

prompt (`str` or `list[str]`) : The prompt or prompts to guide the image generation.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

num_inference_steps (`int`, *optional*, defaults to 25) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

output_type (`str`, *optional*, defaults to `"pt"`) : The output format of the generate image. Choose between: `"np"` (`np.array`) or `"pt"` (`torch.Tensor`).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.

**Returns:** `KandinskyPriorPipelineOutput` or `tuple`

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> from diffusers import KandinskyPipeline, KandinskyPriorPipeline
>>> import torch

>>> pipe_prior = KandinskyPriorPipeline.from_pretrained("kandinsky-community/kandinsky-2-1-prior")
>>> pipe_prior.to("cuda")

>>> prompt = "red cat, 4k photo"
>>> out = pipe_prior(prompt)
>>> image_emb = out.image_embeds
>>> negative_image_emb = out.negative_image_embeds

>>> pipe = KandinskyPipeline.from_pretrained("kandinsky-community/kandinsky-2-1")
>>> pipe.to("cuda")

>>> image = pipe(
...     prompt,
...     image_embeds=image_emb,
...     negative_image_embeds=negative_image_emb,
...     height=768,
...     width=768,
...     num_inference_steps=100,
... ).images

>>> image[0].save("cat.png")
```

#### interpolate[[diffusers.KandinskyPriorPipeline.interpolate]]

```python
interpolate(images_and_prompts: list, weights: list, num_images_per_prompt: int = 1, num_inference_steps: int = 25, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, negative_prior_prompt: str | None = None, negative_prompt: str = '', guidance_scale: float = 4.0, device = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky/pipeline_kandinsky_prior.py#L180)

**Parameters:**

images_and_prompts (`list[str | PIL.Image.Image | torch.Tensor]`) : list of prompts and images to guide the image generation.

weights : (`list[float]`): list of weights for each condition in `images_and_prompts`

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

num_inference_steps (`int`, *optional*, defaults to 25) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

negative_prior_prompt (`str`, *optional*) : The prompt not to guide the prior diffusion process. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

negative_prompt (`str` or `list[str]`, *optional*) : The prompt not to guide the image generation. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

**Returns:** `KandinskyPriorPipelineOutput` or `tuple`

Function invoked when using the prior pipeline for interpolation.

Examples:
```py
>>> from diffusers import KandinskyPriorPipeline, KandinskyPipeline
>>> from diffusers.utils import load_image
>>> import PIL

>>> import torch
>>> from torchvision import transforms

>>> pipe_prior = KandinskyPriorPipeline.from_pretrained(
...     "kandinsky-community/kandinsky-2-1-prior", torch_dtype=torch.float16
... )
>>> pipe_prior.to("cuda")

>>> img1 = load_image(
...     "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main"
...     "/kandinsky/cat.png"
... )

>>> img2 = load_image(
...     "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main"
...     "/kandinsky/starry_night.jpeg"
... )

>>> images_texts = ["a cat", img1, img2]
>>> weights = [0.3, 0.3, 0.4]
>>> image_emb, zero_image_emb = pipe_prior.interpolate(images_texts, weights)

>>> pipe = KandinskyPipeline.from_pretrained("kandinsky-community/kandinsky-2-1", torch_dtype=torch.float16)
>>> pipe.to("cuda")

>>> image = pipe(
...     "",
...     image_embeds=image_emb,
...     negative_image_embeds=zero_image_emb,
...     height=768,
...     width=768,
...     num_inference_steps=150,
... ).images[0]

>>> image.save("starry_cat.png")
```

## KandinskyPipeline[[diffusers.KandinskyPipeline]]

#### diffusers.KandinskyPipeline[[diffusers.KandinskyPipeline]]

```python
diffusers.KandinskyPipeline(text_encoder: MultilingualCLIP, tokenizer: XLMRobertaTokenizer, unet: UNet2DConditionModel, scheduler: diffusers.schedulers.scheduling_ddim.DDIMScheduler | diffusers.schedulers.scheduling_ddpm.DDPMScheduler, movq: VQModel)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky/pipeline_kandinsky.py#L81)

**Parameters:**

text_encoder (`MultilingualCLIP`) : Frozen text-encoder.

tokenizer (`XLMRobertaTokenizer`) : Tokenizer of class

scheduler (`DDIMScheduler` | `DDPMScheduler`) : A scheduler to be used in combination with `unet` to generate image latents.

unet ([UNet2DConditionModel](/docs/diffusers/v0.40.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the image embedding.

movq ([VQModel](/docs/diffusers/v0.40.0/en/api/models/vq#diffusers.VQModel)) : MoVQ Decoder to generate the image from the latents.

Pipeline for text-to-image generation using Kandinsky

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.KandinskyPipeline.__call__]]

```python
__call__(prompt: str | list[str], image_embeds: typing.Union[torch.Tensor, list[torch.Tensor]], negative_image_embeds: typing.Union[torch.Tensor, list[torch.Tensor]], negative_prompt: str | list[str] | None = None, height: int = 512, width: int = 512, num_inference_steps: int = 100, guidance_scale: float = 4.0, num_images_per_prompt: int = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', callback: typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None, callback_steps: int = 1, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky/pipeline_kandinsky.py#L236)

**Parameters:**

prompt (`str` or `list[str]`) : The prompt or prompts to guide the image generation.

image_embeds (`torch.Tensor` or `list[torch.Tensor]`) : The clip image embeddings for text prompt, that will be used to condition the image generation.

negative_image_embeds (`torch.Tensor` or `list[torch.Tensor]`) : The clip image embeddings for negative text prompt, will be used to condition the image generation.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

height (`int`, *optional*, defaults to 512) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to 512) : The width in pixels of the generated image.

num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between: `"pil"` (`PIL.Image.Image`), `"np"` (`np.array`) or `"pt"` (`torch.Tensor`).

callback (`Callable`, *optional*) : A function that calls every `callback_steps` steps during inference. The function is called with the following arguments: `callback(step: int, timestep: int, latents: torch.Tensor)`.

callback_steps (`int`, *optional*, defaults to 1) : The frequency at which the `callback` function is called. If not specified, the callback is called at every step.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> from diffusers import KandinskyPipeline, KandinskyPriorPipeline
>>> import torch

>>> pipe_prior = KandinskyPriorPipeline.from_pretrained("kandinsky-community/Kandinsky-2-1-prior")
>>> pipe_prior.to("cuda")

>>> prompt = "red cat, 4k photo"
>>> out = pipe_prior(prompt)
>>> image_emb = out.image_embeds
>>> negative_image_emb = out.negative_image_embeds

>>> pipe = KandinskyPipeline.from_pretrained("kandinsky-community/kandinsky-2-1")
>>> pipe.to("cuda")

>>> image = pipe(
...     prompt,
...     image_embeds=image_emb,
...     negative_image_embeds=negative_image_emb,
...     height=768,
...     width=768,
...     num_inference_steps=100,
... ).images

>>> image[0].save("cat.png")
```

## KandinskyCombinedPipeline[[diffusers.KandinskyCombinedPipeline]]

#### diffusers.KandinskyCombinedPipeline[[diffusers.KandinskyCombinedPipeline]]

```python
diffusers.KandinskyCombinedPipeline(text_encoder: MultilingualCLIP, tokenizer: XLMRobertaTokenizer, unet: UNet2DConditionModel, scheduler: diffusers.schedulers.scheduling_ddim.DDIMScheduler | diffusers.schedulers.scheduling_ddpm.DDPMScheduler, movq: VQModel, prior_prior: PriorTransformer, prior_image_encoder: CLIPVisionModelWithProjection, prior_text_encoder: CLIPTextModelWithProjection, prior_tokenizer: CLIPTokenizer, prior_scheduler: UnCLIPScheduler, prior_image_processor: CLIPImageProcessorPil)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky/pipeline_kandinsky_combined.py#L113)

**Parameters:**

text_encoder (`MultilingualCLIP`) : Frozen text-encoder.

tokenizer (`XLMRobertaTokenizer`) : Tokenizer of class

scheduler (`DDIMScheduler` | `DDPMScheduler`) : A scheduler to be used in combination with `unet` to generate image latents.

unet ([UNet2DConditionModel](/docs/diffusers/v0.40.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the image embedding.

movq ([VQModel](/docs/diffusers/v0.40.0/en/api/models/vq#diffusers.VQModel)) : MoVQ Decoder to generate the image from the latents.

prior_prior ([PriorTransformer](/docs/diffusers/v0.40.0/en/api/models/prior_transformer#diffusers.PriorTransformer)) : The canonical unCLIP prior to approximate the image embedding from the text embedding.

prior_image_encoder (`CLIPVisionModelWithProjection`) : Frozen image-encoder.

prior_text_encoder (`CLIPTextModelWithProjection`) : Frozen text-encoder.

prior_tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

prior_scheduler (`UnCLIPScheduler`) : A scheduler to be used in combination with `prior` to generate image embedding.

Combined Pipeline for text-to-image generation using Kandinsky

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.KandinskyCombinedPipeline.__call__]]

```python
__call__(prompt: str | list[str], negative_prompt: str | list[str] | None = None, num_inference_steps: int = 100, guidance_scale: float = 4.0, num_images_per_prompt: int = 1, height: int = 512, width: int = 512, prior_guidance_scale: float = 4.0, prior_num_inference_steps: int = 25, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', callback: typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None, callback_steps: int = 1, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky/pipeline_kandinsky_combined.py#L215)

**Parameters:**

prompt (`str` or `list[str]`) : The prompt or prompts to guide the image generation.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

height (`int`, *optional*, defaults to 512) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to 512) : The width in pixels of the generated image.

prior_guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

prior_num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between: `"pil"` (`PIL.Image.Image`), `"np"` (`np.array`) or `"pt"` (`torch.Tensor`).

callback (`Callable`, *optional*) : A function that calls every `callback_steps` steps during inference. The function is called with the following arguments: `callback(step: int, timestep: int, latents: torch.Tensor)`.

callback_steps (`int`, *optional*, defaults to 1) : The frequency at which the `callback` function is called. If not specified, the callback is called at every step.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

Function invoked when calling the pipeline for generation.

Examples:
```py
from diffusers import AutoPipelineForText2Image
import torch

pipe = AutoPipelineForText2Image.from_pretrained(
    "kandinsky-community/kandinsky-2-1", torch_dtype=torch.float16
)
pipe.enable_model_cpu_offload()

prompt = "A lion in galaxies, spirals, nebulae, stars, smoke, iridescent, intricate detail, octane render, 8k"

image = pipe(prompt=prompt, num_inference_steps=25).images[0]
```

#### enable_sequential_cpu_offload[[diffusers.KandinskyCombinedPipeline.enable_sequential_cpu_offload]]

```python
enable_sequential_cpu_offload(gpu_id: int | None = None, device: typing.Union[torch.device, str] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky/pipeline_kandinsky_combined.py#L196)

Offloads all models (`unet`, `text_encoder`, `vae`, and `safety checker` state dicts) to CPU using 🤗
Accelerate, significantly reducing memory usage. Models are moved to a `torch.device('meta')` and loaded on a
GPU only when their specific submodule's `forward` method is called. Offloading happens on a submodule basis.
Memory savings are higher than using `enable_model_cpu_offload`, but performance is lower.

## KandinskyImg2ImgPipeline[[diffusers.KandinskyImg2ImgPipeline]]

#### diffusers.KandinskyImg2ImgPipeline[[diffusers.KandinskyImg2ImgPipeline]]

```python
diffusers.KandinskyImg2ImgPipeline(text_encoder: MultilingualCLIP, movq: VQModel, tokenizer: XLMRobertaTokenizer, unet: UNet2DConditionModel, scheduler: DDIMScheduler)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky/pipeline_kandinsky_img2img.py#L93)

**Parameters:**

text_encoder (`MultilingualCLIP`) : Frozen text-encoder.

tokenizer (`XLMRobertaTokenizer`) : Tokenizer of class

scheduler ([DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler)) : A scheduler to be used in combination with `unet` to generate image latents.

unet ([UNet2DConditionModel](/docs/diffusers/v0.40.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the image embedding.

movq ([VQModel](/docs/diffusers/v0.40.0/en/api/models/vq#diffusers.VQModel)) : MoVQ image encoder and decoder

Pipeline for image-to-image generation using Kandinsky

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.KandinskyImg2ImgPipeline.__call__]]

```python
__call__(prompt: str | list[str], image: typing.Union[torch.Tensor, list[torch.Tensor], PIL.Image.Image, list[PIL.Image.Image]], image_embeds: Tensor, negative_image_embeds: Tensor, negative_prompt: str | list[str] | None = None, height: int = 512, width: int = 512, num_inference_steps: int = 100, strength: float = 0.3, guidance_scale: float = 7.0, num_images_per_prompt: int = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, output_type: str | None = 'pil', callback: typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None, callback_steps: int = 1, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky/pipeline_kandinsky_img2img.py#L297)

**Parameters:**

prompt (`str` or `list[str]`) : The prompt or prompts to guide the image generation.

image (`torch.Tensor`, `PIL.Image.Image`) : `Image`, or tensor representing an image batch, that will be used as the starting point for the process.

image_embeds (`torch.Tensor` or `list[torch.Tensor]`) : The clip image embeddings for text prompt, that will be used to condition the image generation.

negative_image_embeds (`torch.Tensor` or `list[torch.Tensor]`) : The clip image embeddings for negative text prompt, will be used to condition the image generation.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

height (`int`, *optional*, defaults to 512) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to 512) : The width in pixels of the generated image.

num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

strength (`float`, *optional*, defaults to 0.3) : Conceptually, indicates how much to transform the reference `image`. Must be between 0 and 1. `image` will be used as a starting point, adding more noise to it the larger the `strength`. The number of denoising steps depends on the amount of noise initially added. When `strength` is 1, added noise will be maximum and the denoising process will run for the full number of iterations specified in `num_inference_steps`. A value of 1, therefore, essentially ignores `image`.

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between: `"pil"` (`PIL.Image.Image`), `"np"` (`np.array`) or `"pt"` (`torch.Tensor`).

callback (`Callable`, *optional*) : A function that calls every `callback_steps` steps during inference. The function is called with the following arguments: `callback(step: int, timestep: int, latents: torch.Tensor)`.

callback_steps (`int`, *optional*, defaults to 1) : The frequency at which the `callback` function is called. If not specified, the callback is called at every step.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> from diffusers import KandinskyImg2ImgPipeline, KandinskyPriorPipeline
>>> from diffusers.utils import load_image
>>> import torch

>>> pipe_prior = KandinskyPriorPipeline.from_pretrained(
...     "kandinsky-community/kandinsky-2-1-prior", torch_dtype=torch.float16
... )
>>> pipe_prior.to("cuda")

>>> prompt = "A red cartoon frog, 4k"
>>> image_emb, zero_image_emb = pipe_prior(prompt, return_dict=False)

>>> pipe = KandinskyImg2ImgPipeline.from_pretrained(
...     "kandinsky-community/kandinsky-2-1", torch_dtype=torch.float16
... )
>>> pipe.to("cuda")

>>> init_image = load_image(
...     "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main"
...     "/kandinsky/frog.png"
... )

>>> image = pipe(
...     prompt,
...     image=init_image,
...     image_embeds=image_emb,
...     negative_image_embeds=zero_image_emb,
...     height=768,
...     width=768,
...     num_inference_steps=100,
...     strength=0.2,
... ).images

>>> image[0].save("red_frog.png")
```

## KandinskyImg2ImgCombinedPipeline[[diffusers.KandinskyImg2ImgCombinedPipeline]]

#### diffusers.KandinskyImg2ImgCombinedPipeline[[diffusers.KandinskyImg2ImgCombinedPipeline]]

```python
diffusers.KandinskyImg2ImgCombinedPipeline(text_encoder: MultilingualCLIP, tokenizer: XLMRobertaTokenizer, unet: UNet2DConditionModel, scheduler: diffusers.schedulers.scheduling_ddim.DDIMScheduler | diffusers.schedulers.scheduling_ddpm.DDPMScheduler, movq: VQModel, prior_prior: PriorTransformer, prior_image_encoder: CLIPVisionModelWithProjection, prior_text_encoder: CLIPTextModelWithProjection, prior_tokenizer: CLIPTokenizer, prior_scheduler: UnCLIPScheduler, prior_image_processor: CLIPImageProcessorPil)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky/pipeline_kandinsky_combined.py#L331)

**Parameters:**

text_encoder (`MultilingualCLIP`) : Frozen text-encoder.

tokenizer (`XLMRobertaTokenizer`) : Tokenizer of class

scheduler (`DDIMScheduler` | `DDPMScheduler`) : A scheduler to be used in combination with `unet` to generate image latents.

unet ([UNet2DConditionModel](/docs/diffusers/v0.40.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the image embedding.

movq ([VQModel](/docs/diffusers/v0.40.0/en/api/models/vq#diffusers.VQModel)) : MoVQ Decoder to generate the image from the latents.

prior_prior ([PriorTransformer](/docs/diffusers/v0.40.0/en/api/models/prior_transformer#diffusers.PriorTransformer)) : The canonical unCLIP prior to approximate the image embedding from the text embedding.

prior_image_encoder (`CLIPVisionModelWithProjection`) : Frozen image-encoder.

prior_text_encoder (`CLIPTextModelWithProjection`) : Frozen text-encoder.

prior_tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

prior_scheduler (`UnCLIPScheduler`) : A scheduler to be used in combination with `prior` to generate image embedding.

Combined Pipeline for image-to-image generation using Kandinsky

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.KandinskyImg2ImgCombinedPipeline.__call__]]

```python
__call__(prompt: str | list[str], image: typing.Union[torch.Tensor, list[torch.Tensor], PIL.Image.Image, list[PIL.Image.Image]], negative_prompt: str | list[str] | None = None, num_inference_steps: int = 100, guidance_scale: float = 4.0, num_images_per_prompt: int = 1, strength: float = 0.3, height: int = 512, width: int = 512, prior_guidance_scale: float = 4.0, prior_num_inference_steps: int = 25, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', callback: typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None, callback_steps: int = 1, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky/pipeline_kandinsky_combined.py#L434)

**Parameters:**

prompt (`str` or `list[str]`) : The prompt or prompts to guide the image generation.

image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) : `Image`, or tensor representing an image batch, that will be used as the starting point for the process. Can also accept image latents as `image`, if passing latents directly, it will not be encoded again.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

height (`int`, *optional*, defaults to 512) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to 512) : The width in pixels of the generated image.

strength (`float`, *optional*, defaults to 0.3) : Conceptually, indicates how much to transform the reference `image`. Must be between 0 and 1. `image` will be used as a starting point, adding more noise to it the larger the `strength`. The number of denoising steps depends on the amount of noise initially added. When `strength` is 1, added noise will be maximum and the denoising process will run for the full number of iterations specified in `num_inference_steps`. A value of 1, therefore, essentially ignores `image`.

prior_guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

prior_num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between: `"pil"` (`PIL.Image.Image`), `"np"` (`np.array`) or `"pt"` (`torch.Tensor`).

callback (`Callable`, *optional*) : A function that calls every `callback_steps` steps during inference. The function is called with the following arguments: `callback(step: int, timestep: int, latents: torch.Tensor)`.

callback_steps (`int`, *optional*, defaults to 1) : The frequency at which the `callback` function is called. If not specified, the callback is called at every step.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

Function invoked when calling the pipeline for generation.

Examples:
```py
from diffusers import AutoPipelineForImage2Image
import torch
import requests
from io import BytesIO
from PIL import Image
import os

pipe = AutoPipelineForImage2Image.from_pretrained(
    "kandinsky-community/kandinsky-2-1", torch_dtype=torch.float16
)
pipe.enable_model_cpu_offload()

prompt = "A fantasy landscape, Cinematic lighting"
negative_prompt = "low quality, bad quality"

url = "https://raw.githubusercontent.com/CompVis/stable-diffusion/main/assets/stable-samples/img2img/sketch-mountains-input.jpg"

response = requests.get(url)
image = Image.open(BytesIO(response.content)).convert("RGB")
image.thumbnail((768, 768))

image = pipe(prompt=prompt, image=original_image, num_inference_steps=25).images[0]
```

#### enable_sequential_cpu_offload[[diffusers.KandinskyImg2ImgCombinedPipeline.enable_sequential_cpu_offload]]

```python
enable_sequential_cpu_offload(gpu_id: int | None = None, device: typing.Union[torch.device, str] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky/pipeline_kandinsky_combined.py#L414)

Offloads all models to CPU using accelerate, significantly reducing memory usage. When called, unet,
text_encoder, vae and safety checker have their state dicts saved to CPU and then are moved to a
`torch.device('meta') and loaded to GPU only when their specific submodule has its `forward` method called.
Note that offloading happens on a submodule basis. Memory savings are higher than with
`enable_model_cpu_offload`, but performance is lower.

## KandinskyInpaintPipeline[[diffusers.KandinskyInpaintPipeline]]

#### diffusers.KandinskyInpaintPipeline[[diffusers.KandinskyInpaintPipeline]]

```python
diffusers.KandinskyInpaintPipeline(text_encoder: MultilingualCLIP, movq: VQModel, tokenizer: XLMRobertaTokenizer, unet: UNet2DConditionModel, scheduler: DDIMScheduler)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky/pipeline_kandinsky_inpaint.py#L245)

**Parameters:**

text_encoder (`MultilingualCLIP`) : Frozen text-encoder.

tokenizer (`XLMRobertaTokenizer`) : Tokenizer of class

scheduler ([DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler)) : A scheduler to be used in combination with `unet` to generate image latents.

unet ([UNet2DConditionModel](/docs/diffusers/v0.40.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the image embedding.

movq ([VQModel](/docs/diffusers/v0.40.0/en/api/models/vq#diffusers.VQModel)) : MoVQ image encoder and decoder

Pipeline for text-guided image inpainting using Kandinsky2.1

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.KandinskyInpaintPipeline.__call__]]

```python
__call__(prompt: str | list[str], image: typing.Union[torch.Tensor, PIL.Image.Image], mask_image: typing.Union[PIL.Image.Image, torch.Tensor, numpy.ndarray], image_embeds: Tensor, negative_image_embeds: Tensor, negative_prompt: str | list[str] | None = None, height: int = 512, width: int = 512, num_inference_steps: int = 100, guidance_scale: float = 4.0, num_images_per_prompt: int = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', callback: typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None, callback_steps: int = 1, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky/pipeline_kandinsky_inpaint.py#L401)

**Parameters:**

prompt (`str` or `list[str]`) : The prompt or prompts to guide the image generation.

image (`torch.Tensor`, `PIL.Image.Image` or `np.ndarray`) : `Image`, or tensor representing an image batch, that will be used as the starting point for the process.

mask_image (`PIL.Image.Image`,`torch.Tensor` or `np.ndarray`) : `Image`, or a tensor representing an image batch, to mask `image`. White pixels in the mask will be repainted, while black pixels will be preserved. You can pass a pytorch tensor as mask only if the image you passed is a pytorch tensor, and it should contain one color channel (L) instead of 3, so the expected shape would be either `(B, 1, H, W,)`, `(B, H, W)`, `(1, H, W)` or `(H, W)` If image is an PIL image or numpy array, mask should also be a either PIL image or numpy array. If it is a PIL image, it will be converted to a single channel (luminance) before use. If it is a nummpy array, the expected shape is `(H, W)`.

image_embeds (`torch.Tensor` or `list[torch.Tensor]`) : The clip image embeddings for text prompt, that will be used to condition the image generation.

negative_image_embeds (`torch.Tensor` or `list[torch.Tensor]`) : The clip image embeddings for negative text prompt, will be used to condition the image generation.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

height (`int`, *optional*, defaults to 512) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to 512) : The width in pixels of the generated image.

num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between: `"pil"` (`PIL.Image.Image`), `"np"` (`np.array`) or `"pt"` (`torch.Tensor`).

callback (`Callable`, *optional*) : A function that calls every `callback_steps` steps during inference. The function is called with the following arguments: `callback(step: int, timestep: int, latents: torch.Tensor)`.

callback_steps (`int`, *optional*, defaults to 1) : The frequency at which the `callback` function is called. If not specified, the callback is called at every step.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> from diffusers import KandinskyInpaintPipeline, KandinskyPriorPipeline
>>> from diffusers.utils import load_image
>>> import torch
>>> import numpy as np

>>> pipe_prior = KandinskyPriorPipeline.from_pretrained(
...     "kandinsky-community/kandinsky-2-1-prior", torch_dtype=torch.float16
... )
>>> pipe_prior.to("cuda")

>>> prompt = "a hat"
>>> image_emb, zero_image_emb = pipe_prior(prompt, return_dict=False)

>>> pipe = KandinskyInpaintPipeline.from_pretrained(
...     "kandinsky-community/kandinsky-2-1-inpaint", torch_dtype=torch.float16
... )
>>> pipe.to("cuda")

>>> init_image = load_image(
...     "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main"
...     "/kandinsky/cat.png"
... )

>>> mask = np.zeros((768, 768), dtype=np.float32)
>>> mask[:250, 250:-250] = 1

>>> out = pipe(
...     prompt,
...     image=init_image,
...     mask_image=mask,
...     image_embeds=image_emb,
...     negative_image_embeds=zero_image_emb,
...     height=768,
...     width=768,
...     num_inference_steps=50,
... )

>>> image = out.images[0]
>>> image.save("cat_with_hat.png")
```

## KandinskyInpaintCombinedPipeline[[diffusers.KandinskyInpaintCombinedPipeline]]

#### diffusers.KandinskyInpaintCombinedPipeline[[diffusers.KandinskyInpaintCombinedPipeline]]

```python
diffusers.KandinskyInpaintCombinedPipeline(text_encoder: MultilingualCLIP, tokenizer: XLMRobertaTokenizer, unet: UNet2DConditionModel, scheduler: diffusers.schedulers.scheduling_ddim.DDIMScheduler | diffusers.schedulers.scheduling_ddpm.DDPMScheduler, movq: VQModel, prior_prior: PriorTransformer, prior_image_encoder: CLIPVisionModelWithProjection, prior_text_encoder: CLIPTextModelWithProjection, prior_tokenizer: CLIPTokenizer, prior_scheduler: UnCLIPScheduler, prior_image_processor: CLIPImageProcessorPil)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky/pipeline_kandinsky_combined.py#L572)

**Parameters:**

text_encoder (`MultilingualCLIP`) : Frozen text-encoder.

tokenizer (`XLMRobertaTokenizer`) : Tokenizer of class

scheduler (`DDIMScheduler` | `DDPMScheduler`) : A scheduler to be used in combination with `unet` to generate image latents.

unet ([UNet2DConditionModel](/docs/diffusers/v0.40.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : Conditional U-Net architecture to denoise the image embedding.

movq ([VQModel](/docs/diffusers/v0.40.0/en/api/models/vq#diffusers.VQModel)) : MoVQ Decoder to generate the image from the latents.

prior_prior ([PriorTransformer](/docs/diffusers/v0.40.0/en/api/models/prior_transformer#diffusers.PriorTransformer)) : The canonical unCLIP prior to approximate the image embedding from the text embedding.

prior_image_encoder (`CLIPVisionModelWithProjection`) : Frozen image-encoder.

prior_text_encoder (`CLIPTextModelWithProjection`) : Frozen text-encoder.

prior_tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

prior_scheduler (`UnCLIPScheduler`) : A scheduler to be used in combination with `prior` to generate image embedding.

Combined Pipeline for generation using Kandinsky

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.KandinskyInpaintCombinedPipeline.__call__]]

```python
__call__(prompt: str | list[str], image: typing.Union[torch.Tensor, list[torch.Tensor], PIL.Image.Image, list[PIL.Image.Image]], mask_image: typing.Union[torch.Tensor, list[torch.Tensor], PIL.Image.Image, list[PIL.Image.Image]], negative_prompt: str | list[str] | None = None, num_inference_steps: int = 100, guidance_scale: float = 4.0, num_images_per_prompt: int = 1, height: int = 512, width: int = 512, prior_guidance_scale: float = 4.0, prior_num_inference_steps: int = 25, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', callback: typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None, callback_steps: int = 1, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky/pipeline_kandinsky_combined.py#L675)

**Parameters:**

prompt (`str` or `list[str]`) : The prompt or prompts to guide the image generation.

image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) : `Image`, or tensor representing an image batch, that will be used as the starting point for the process. Can also accept image latents as `image`, if passing latents directly, it will not be encoded again.

mask_image (`np.array`) : Tensor representing an image batch, to mask `image`. White pixels in the mask will be repainted, while black pixels will be preserved. If `mask_image` is a PIL image, it will be converted to a single channel (luminance) before use. If it's a tensor, it should contain one color channel (L) instead of 3, so the expected shape would be `(B, H, W, 1)`.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

height (`int`, *optional*, defaults to 512) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to 512) : The width in pixels of the generated image.

prior_guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

prior_num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between: `"pil"` (`PIL.Image.Image`), `"np"` (`np.array`) or `"pt"` (`torch.Tensor`).

callback (`Callable`, *optional*) : A function that calls every `callback_steps` steps during inference. The function is called with the following arguments: `callback(step: int, timestep: int, latents: torch.Tensor)`.

callback_steps (`int`, *optional*, defaults to 1) : The frequency at which the `callback` function is called. If not specified, the callback is called at every step.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) instead of a plain tuple.

**Returns:** [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`

Function invoked when calling the pipeline for generation.

Examples:
```py
from diffusers import AutoPipelineForInpainting
from diffusers.utils import load_image
import torch
import numpy as np

pipe = AutoPipelineForInpainting.from_pretrained(
    "kandinsky-community/kandinsky-2-1-inpaint", torch_dtype=torch.float16
)
pipe.enable_model_cpu_offload()

prompt = "A fantasy landscape, Cinematic lighting"
negative_prompt = "low quality, bad quality"

original_image = load_image(
    "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main" "/kandinsky/cat.png"
)

mask = np.zeros((768, 768), dtype=np.float32)
# Let's mask out an area above the cat's head
mask[:250, 250:-250] = 1

image = pipe(prompt=prompt, image=original_image, mask_image=mask, num_inference_steps=25).images[0]
```

#### enable_sequential_cpu_offload[[diffusers.KandinskyInpaintCombinedPipeline.enable_sequential_cpu_offload]]

```python
enable_sequential_cpu_offload(gpu_id: int | None = None, device: typing.Union[torch.device, str] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/kandinsky/pipeline_kandinsky_combined.py#L655)

Offloads all models to CPU using accelerate, significantly reducing memory usage. When called, unet,
text_encoder, vae and safety checker have their state dicts saved to CPU and then are moved to a
`torch.device('meta') and loaded to GPU only when their specific submodule has its `forward` method called.
Note that offloading happens on a submodule basis. Memory savings are higher than with
`enable_model_cpu_offload`, but performance is lower.
