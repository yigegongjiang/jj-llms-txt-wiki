# Shap-E

The Shap-E model was proposed in [Shap-E: Generating Conditional 3D Implicit Functions](https://huggingface.co/papers/2305.02463) by Alex Nichol and Heewoo Jun from [OpenAI](https://github.com/openai).

The abstract from the paper is:

*We present Shap-E, a conditional generative model for 3D assets. Unlike recent work on 3D generative models which produce a single output representation, Shap-E directly generates the parameters of implicit functions that can be rendered as both textured meshes and neural radiance fields. We train Shap-E in two stages: first, we train an encoder that deterministically maps 3D assets into the parameters of an implicit function; second, we train a conditional diffusion model on outputs of the encoder. When trained on a large dataset of paired 3D and text data, our resulting models are capable of generating complex and diverse 3D assets in a matter of seconds. When compared to Point-E, an explicit generative model over point clouds, Shap-E converges faster and reaches comparable or better sample quality despite modeling a higher-dimensional, multi-representation output space.*

The original codebase can be found at [openai/shap-e](https://github.com/openai/shap-e).

> [!TIP]
> See the [reuse components across pipelines](../../using-diffusers/loading#reuse-a-pipeline) section to learn how to efficiently load the same components into multiple pipelines.

Make sure you have the following libraries installed.

```py
# uncomment to install the necessary libraries in Colab
#!pip install -q diffusers transformers accelerate trimesh
```

## Text-to-3D

To generate a gif of a 3D object, pass a text prompt to the [ShapEPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/shap_e#diffusers.ShapEPipeline). The pipeline generates a list of image frames which are used to create the 3D object.

```py
import torch
from diffusers import ShapEPipeline

device = torch.device("cuda" if torch.cuda.is_available() else "cpu")

pipe = ShapEPipeline.from_pretrained("openai/shap-e", dtype=torch.float16, variant="fp16")
pipe = pipe.to(device)

guidance_scale = 15.0
prompt = ["A firecracker", "A birthday cupcake"]

images = pipe(
    prompt,
    guidance_scale=guidance_scale,
    num_inference_steps=64,
    frame_size=256,
).images
```

Now use the [export_to_gif()](/docs/diffusers/v0.40.0/en/api/utilities#diffusers.utils.export_to_gif) function to convert the list of image frames to a gif of the 3D object.

```py
from diffusers.utils import export_to_gif

export_to_gif(images[0], "firecracker_3d.gif")
export_to_gif(images[1], "cake_3d.gif")
```

  
    
    prompt = "A firecracker"
  
  
    
    prompt = "A birthday cupcake"
  

## Image-to-3D

To generate a 3D object from another image, use the [ShapEImg2ImgPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/shap_e#diffusers.ShapEImg2ImgPipeline). You can use an existing image or generate an entirely new one. Let's use the [Kandinsky 2.1](./kandinsky) model to generate a new image.

```py
from diffusers import DiffusionPipeline
import torch

prior_pipeline = DiffusionPipeline.from_pretrained("kandinsky-community/kandinsky-2-1-prior", dtype=torch.float16, use_safetensors=True).to("cuda")
pipeline = DiffusionPipeline.from_pretrained("kandinsky-community/kandinsky-2-1", dtype=torch.float16, use_safetensors=True).to("cuda")

prompt = "A cheeseburger, white background"

image_embeds, negative_image_embeds = prior_pipeline(prompt, guidance_scale=1.0).to_tuple()
image = pipeline(
    prompt,
    image_embeds=image_embeds,
    negative_image_embeds=negative_image_embeds,
).images[0]

image.save("burger.png")
```

Pass the cheeseburger to the [ShapEImg2ImgPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/shap_e#diffusers.ShapEImg2ImgPipeline) to generate a 3D representation of it.

```py
from PIL import Image
from diffusers import ShapEImg2ImgPipeline
from diffusers.utils import export_to_gif

pipe = ShapEImg2ImgPipeline.from_pretrained("openai/shap-e-img2img", dtype=torch.float16, variant="fp16").to("cuda")

guidance_scale = 3.0
image = Image.open("burger.png").resize((256, 256))

images = pipe(
    image,
    guidance_scale=guidance_scale,
    num_inference_steps=64,
    frame_size=256,
).images

gif_path = export_to_gif(images[0], "burger_3d.gif")
```

  
    
    cheeseburger
  
  
    
    3D cheeseburger
  

## Generate mesh

Shap-E is a flexible model that can also generate textured mesh outputs to be rendered for downstream applications. In this example, you'll convert the output into a `glb` file because the 🤗 Datasets library supports mesh visualization of `glb` files which can be rendered by the [Dataset viewer](https://huggingface.co/docs/hub/datasets-viewer#dataset-preview).

You can generate mesh outputs for both the [ShapEPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/shap_e#diffusers.ShapEPipeline) and [ShapEImg2ImgPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/shap_e#diffusers.ShapEImg2ImgPipeline) by specifying the `output_type` parameter as `"mesh"`:

```py
import torch
from diffusers import ShapEPipeline

device = torch.device("cuda" if torch.cuda.is_available() else "cpu")

pipe = ShapEPipeline.from_pretrained("openai/shap-e", dtype=torch.float16, variant="fp16")
pipe = pipe.to(device)

guidance_scale = 15.0
prompt = "A birthday cupcake"

images = pipe(prompt, guidance_scale=guidance_scale, num_inference_steps=64, frame_size=256, output_type="mesh").images
```

Use the `export_to_ply()` function to save the mesh output as a `ply` file:

> [!TIP]
> You can optionally save the mesh output as an `obj` file with the `export_to_obj()` function. The ability to save the mesh output in a variety of formats makes it more flexible for downstream usage!

```py
from diffusers.utils import export_to_ply

ply_path = export_to_ply(images[0], "3d_cake.ply")
print(f"Saved to folder: {ply_path}")
```

Then you can convert the `ply` file to a `glb` file with the trimesh library:

```py
import trimesh

mesh = trimesh.load("3d_cake.ply")
mesh_export = mesh.export("3d_cake.glb", file_type="glb")
```

By default, the mesh output is focused from the bottom viewpoint but you can change the default viewpoint by applying a rotation transform:

```py
import trimesh
import numpy as np

mesh = trimesh.load("3d_cake.ply")
rot = trimesh.transformations.rotation_matrix(-np.pi / 2, [1, 0, 0])
mesh = mesh.apply_transform(rot)
mesh_export = mesh.export("3d_cake.glb", file_type="glb")
```

Upload the mesh file to your dataset repository to visualize it with the Dataset viewer!

    

## ShapEPipeline[[diffusers.ShapEPipeline]]

#### diffusers.ShapEPipeline[[diffusers.ShapEPipeline]]

```python
diffusers.ShapEPipeline(prior: PriorTransformer, text_encoder: CLIPTextModelWithProjection, tokenizer: CLIPTokenizer, scheduler: HeunDiscreteScheduler, shap_e_renderer: ShapERenderer)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/shap_e/pipeline_shap_e.py#L87)

**Parameters:**

prior ([PriorTransformer](/docs/diffusers/v0.40.0/en/api/models/prior_transformer#diffusers.PriorTransformer)) : The canonical unCLIP prior to approximate the image embedding from the text embedding.

text_encoder ([CLIPTextModelWithProjection](https://huggingface.co/docs/transformers/v5.15.1/en/model_doc/clip#transformers.CLIPTextModelWithProjection)) : Frozen text-encoder.

tokenizer ([CLIPTokenizer](https://huggingface.co/docs/transformers/v5.15.1/en/model_doc/clip#transformers.CLIPTokenizer)) : A `CLIPTokenizer` to tokenize text.

scheduler ([HeunDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/heun#diffusers.HeunDiscreteScheduler)) : A scheduler to be used in combination with the `prior` model to generate image embedding.

shap_e_renderer (`ShapERenderer`) : Shap-E renderer projects the generated latents into parameters of a MLP to create 3D objects with the NeRF rendering method.

Pipeline for generating latent representation of a 3D asset and rendering with the NeRF method.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

#### __call__[[diffusers.ShapEPipeline.__call__]]

```python
__call__(prompt: str, num_images_per_prompt: int = 1, num_inference_steps: int = 25, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, guidance_scale: float = 4.0, frame_size: int = 64, output_type: str | None = 'pil', return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/shap_e/pipeline_shap_e.py#L190)

**Parameters:**

prompt (`str` or `list[str]`) : The prompt or prompts to guide the image generation.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

num_inference_steps (`int`, *optional*, defaults to 25) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor is generated by sampling using the supplied random `generator`.

guidance_scale (`float`, *optional*, defaults to 4.0) : A higher guidance scale value encourages the model to generate images closely linked to the text `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.

frame_size (`int`, *optional*, default to 64) : The width and height of each image frame of the generated 3D output.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generated image. Choose between `"pil"` (`PIL.Image.Image`), `"np"` (`np.array`), `"latent"` (`torch.Tensor`), or mesh (`MeshDecoderOutput`).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ShapEPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/shap_e#diffusers.pipelines.shap_e.pipeline_shap_e.ShapEPipelineOutput) instead of a plain tuple.

**Returns:** [ShapEPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/shap_e#diffusers.pipelines.shap_e.pipeline_shap_e.ShapEPipelineOutput) or `tuple`

If `return_dict` is `True`, [ShapEPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/shap_e#diffusers.pipelines.shap_e.pipeline_shap_e.ShapEPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated images.

The call function to the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import DiffusionPipeline
>>> from diffusers.utils import export_to_gif

>>> device = torch.device("cuda" if torch.cuda.is_available() else "cpu")

>>> repo = "openai/shap-e"
>>> pipe = DiffusionPipeline.from_pretrained(repo, torch_dtype=torch.float16)
>>> pipe = pipe.to(device)

>>> guidance_scale = 15.0
>>> prompt = "a shark"

>>> images = pipe(
...     prompt,
...     guidance_scale=guidance_scale,
...     num_inference_steps=64,
...     frame_size=256,
... ).images

>>> gif_path = export_to_gif(images[0], "shark_3d.gif")
```

## ShapEImg2ImgPipeline[[diffusers.ShapEImg2ImgPipeline]]

#### diffusers.ShapEImg2ImgPipeline[[diffusers.ShapEImg2ImgPipeline]]

```python
diffusers.ShapEImg2ImgPipeline(prior: PriorTransformer, image_encoder: CLIPVisionModel, image_processor: CLIPImageProcessorPil, scheduler: HeunDiscreteScheduler, shap_e_renderer: ShapERenderer)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/shap_e/pipeline_shap_e_img2img.py#L88)

**Parameters:**

prior ([PriorTransformer](/docs/diffusers/v0.40.0/en/api/models/prior_transformer#diffusers.PriorTransformer)) : The canonical unCLIP prior to approximate the image embedding from the text embedding.

image_encoder ([CLIPVisionModel](https://huggingface.co/docs/transformers/v5.15.1/en/model_doc/clip#transformers.CLIPVisionModel)) : Frozen image-encoder.

image_processor ([CLIPImageProcessor](https://huggingface.co/docs/transformers/v5.15.1/en/model_doc/clip#transformers.CLIPImageProcessor)) : A `CLIPImageProcessor` to process images.

scheduler ([HeunDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/heun#diffusers.HeunDiscreteScheduler)) : A scheduler to be used in combination with the `prior` model to generate image embedding.

shap_e_renderer (`ShapERenderer`) : Shap-E renderer projects the generated latents into parameters of a MLP to create 3D objects with the NeRF rendering method.

Pipeline for generating latent representation of a 3D asset and rendering with the NeRF method from an image.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

#### __call__[[diffusers.ShapEImg2ImgPipeline.__call__]]

```python
__call__(image: PIL.Image.Image | list[PIL.Image.Image], num_images_per_prompt: int = 1, num_inference_steps: int = 25, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, guidance_scale: float = 4.0, frame_size: int = 64, output_type: str | None = 'pil', return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/shap_e/pipeline_shap_e_img2img.py#L172)

**Parameters:**

image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) : `Image` or tensor representing an image batch to be used as the starting point. Can also accept image latents as image, but if passing latents directly it is not encoded again.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

num_inference_steps (`int`, *optional*, defaults to 25) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor is generated by sampling using the supplied random `generator`.

guidance_scale (`float`, *optional*, defaults to 4.0) : A higher guidance scale value encourages the model to generate images closely linked to the text `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.

frame_size (`int`, *optional*, default to 64) : The width and height of each image frame of the generated 3D output.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generated image. Choose between `"pil"` (`PIL.Image.Image`), `"np"` (`np.array`), `"latent"` (`torch.Tensor`), or mesh (`MeshDecoderOutput`).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [ShapEPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/shap_e#diffusers.pipelines.shap_e.pipeline_shap_e.ShapEPipelineOutput) instead of a plain tuple.

**Returns:** [ShapEPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/shap_e#diffusers.pipelines.shap_e.pipeline_shap_e.ShapEPipelineOutput) or `tuple`

If `return_dict` is `True`, [ShapEPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/shap_e#diffusers.pipelines.shap_e.pipeline_shap_e.ShapEPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated images.

The call function to the pipeline for generation.

Examples:
```py
>>> from PIL import Image
>>> import torch
>>> from diffusers import DiffusionPipeline
>>> from diffusers.utils import export_to_gif, load_image

>>> device = torch.device("cuda" if torch.cuda.is_available() else "cpu")

>>> repo = "openai/shap-e-img2img"
>>> pipe = DiffusionPipeline.from_pretrained(repo, torch_dtype=torch.float16)
>>> pipe = pipe.to(device)

>>> guidance_scale = 3.0
>>> image_url = "https://hf.co/datasets/diffusers/docs-images/resolve/main/shap-e/corgi.png"
>>> image = load_image(image_url).convert("RGB")

>>> images = pipe(
...     image,
...     guidance_scale=guidance_scale,
...     num_inference_steps=64,
...     frame_size=256,
... ).images

>>> gif_path = export_to_gif(images[0], "corgi_3d.gif")
```

## ShapEPipelineOutput[[diffusers.pipelines.shap_e.pipeline_shap_e.ShapEPipelineOutput]]

#### diffusers.pipelines.shap_e.pipeline_shap_e.ShapEPipelineOutput[[diffusers.pipelines.shap_e.pipeline_shap_e.ShapEPipelineOutput]]

```python
diffusers.pipelines.shap_e.pipeline_shap_e.ShapEPipelineOutput(images: list[list[PIL.Image.Image]] | list[list[numpy.ndarray]])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/shap_e/pipeline_shap_e.py#L75)

**Parameters:**

images (`torch.Tensor`) : A list of images for 3D rendering.

Output class for [ShapEPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/shap_e#diffusers.ShapEPipeline) and [ShapEImg2ImgPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/shap_e#diffusers.ShapEImg2ImgPipeline).
