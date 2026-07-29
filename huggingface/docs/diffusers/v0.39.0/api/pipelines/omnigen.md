#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
-->

# OmniGen

[OmniGen: Unified Image Generation](https://huggingface.co/papers/2409.11340) from BAAI, by Shitao Xiao, Yueze Wang, Junjie Zhou, Huaying Yuan, Xingrun Xing, Ruiran Yan, Chaofan Li, Shuting Wang, Tiejun Huang, Zheng Liu.

The abstract from the paper is:

*The emergence of Large Language Models (LLMs) has unified language  generation tasks and revolutionized human-machine interaction.  However, in the realm of image generation, a unified model capable of handling various tasks within a single framework remains largely unexplored. In this work, we introduce OmniGen, a new diffusion model for unified image generation. OmniGen is characterized by the following features: 1) Unification: OmniGen not only demonstrates text-to-image generation capabilities but also inherently supports various downstream tasks, such as image editing, subject-driven generation, and visual conditional generation. 2) Simplicity: The architecture of OmniGen is highly simplified, eliminating the need for additional plugins. Moreover, compared to existing diffusion models, it is more user-friendly and can complete complex tasks end-to-end through instructions without the need for extra intermediate steps, greatly simplifying the image generation workflow. 3) Knowledge Transfer: Benefit from learning in a unified format, OmniGen effectively transfers knowledge across different tasks, manages unseen tasks and domains, and exhibits novel capabilities. We also explore the model’s reasoning capabilities and potential applications of the chain-of-thought mechanism.  This work represents the first attempt at a general-purpose image generation model,  and we will release our resources at https://github.com/VectorSpaceLab/OmniGen to foster future advancements.*

> [!TIP]
> Make sure to check out the Schedulers [guide](../../using-diffusers/schedulers) to learn how to explore the tradeoff between scheduler speed and quality, and see the [reuse components across pipelines](../../using-diffusers/loading#reuse-a-pipeline) section to learn how to efficiently load the same components into multiple pipelines.

This pipeline was contributed by [staoxiao](https://github.com/staoxiao). The original codebase can be found [here](https://github.com/VectorSpaceLab/OmniGen). The original weights can be found under [hf.co/shitao](https://huggingface.co/Shitao/OmniGen-v1).

## Load model checkpoints

Model weights may be stored in separate subfolders on the Hub or locally, in which case, you should use the [from_pretrained()](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline.from_pretrained) method.

```python
import torch
from diffusers import OmniGenPipeline

pipe = OmniGenPipeline.from_pretrained("Shitao/OmniGen-v1-diffusers", torch_dtype=torch.bfloat16)
```

## Text-to-image

For text-to-image, pass a text prompt. By default, OmniGen generates a 1024x1024 image. 
You can try setting the `height` and `width` parameters to generate images with different size.

```python
import torch
from diffusers import OmniGenPipeline

pipe = OmniGenPipeline.from_pretrained(
    "Shitao/OmniGen-v1-diffusers",
    torch_dtype=torch.bfloat16
)
pipe.to("cuda")

prompt = "Realistic photo. A young woman sits on a sofa, holding a book and facing the camera. She wears delicate silver hoop earrings adorned with tiny, sparkling diamonds that catch the light, with her long chestnut hair cascading over her shoulders. Her eyes are focused and gentle, framed by long, dark lashes. She is dressed in a cozy cream sweater, which complements her warm, inviting smile. Behind her, there is a table with a cup of water in a sleek, minimalist blue mug. The background is a serene indoor setting with soft natural light filtering through a window, adorned with tasteful art and flowers, creating a cozy and peaceful ambiance. 4K, HD."
image = pipe(
    prompt=prompt,
    height=1024,
    width=1024,
    guidance_scale=3,
    generator=torch.Generator(device="cpu").manual_seed(111),
).images[0]
image.save("output.png")
```

    

## Image edit

OmniGen supports multimodal inputs. 
When the input includes an image, you need to add a placeholder `<img><|image_1|></img>` in the text prompt to represent the image. 
It is recommended to enable `use_input_image_size_as_output` to keep the edited image the same size as the original image.

```python
import torch
from diffusers import OmniGenPipeline
from diffusers.utils import load_image 

pipe = OmniGenPipeline.from_pretrained(
    "Shitao/OmniGen-v1-diffusers",
    torch_dtype=torch.bfloat16
)
pipe.to("cuda")

prompt="<img><|image_1|></img> Remove the woman's earrings. Replace the mug with a clear glass filled with sparkling iced cola."
input_images=[load_image("https://raw.githubusercontent.com/VectorSpaceLab/OmniGen/main/imgs/docs_img/t2i_woman_with_book.png")]
image = pipe(
    prompt=prompt, 
    input_images=input_images, 
    guidance_scale=2, 
    img_guidance_scale=1.6,
    use_input_image_size_as_output=True,
    generator=torch.Generator(device="cpu").manual_seed(222)
).images[0]
image.save("output.png")
```

  
    
    original image
  
  
    
    edited image
  

OmniGen has some interesting features, such as visual reasoning, as shown in the example below.

```python
prompt="If the woman is thirsty, what should she take? Find it in the image and highlight it in blue. <img><|image_1|></img>"
input_images=[load_image("https://raw.githubusercontent.com/VectorSpaceLab/OmniGen/main/imgs/docs_img/edit.png")]
image = pipe(
    prompt=prompt, 
    input_images=input_images, 
    guidance_scale=2, 
    img_guidance_scale=1.6,
    use_input_image_size_as_output=True,
    generator=torch.Generator(device="cpu").manual_seed(0)
).images[0]
image.save("output.png")
```

    

## Controllable generation

OmniGen can handle several classic computer vision tasks. As shown below, OmniGen can detect human skeletons in input images, which can be used as control conditions to generate new images.

```python
import torch
from diffusers import OmniGenPipeline
from diffusers.utils import load_image 

pipe = OmniGenPipeline.from_pretrained(
    "Shitao/OmniGen-v1-diffusers",
    torch_dtype=torch.bfloat16
)
pipe.to("cuda")

prompt="Detect the skeleton of human in this image: <img><|image_1|></img>"
input_images=[load_image("https://raw.githubusercontent.com/VectorSpaceLab/OmniGen/main/imgs/docs_img/edit.png")]
image1 = pipe(
    prompt=prompt, 
    input_images=input_images, 
    guidance_scale=2, 
    img_guidance_scale=1.6,
    use_input_image_size_as_output=True,
    generator=torch.Generator(device="cpu").manual_seed(333)
).images[0]
image1.save("image1.png")

prompt="Generate a new photo using the following picture and text as conditions: <img><|image_1|></img>\n A young boy is sitting on a sofa in the library, holding a book. His hair is neatly combed, and a faint smile plays on his lips, with a few freckles scattered across his cheeks. The library is quiet, with rows of shelves filled with books stretching out behind him."
input_images=[load_image("https://raw.githubusercontent.com/VectorSpaceLab/OmniGen/main/imgs/docs_img/skeletal.png")]
image2 = pipe(
    prompt=prompt, 
    input_images=input_images, 
    guidance_scale=2, 
    img_guidance_scale=1.6,
    use_input_image_size_as_output=True,
    generator=torch.Generator(device="cpu").manual_seed(333)
).images[0]
image2.save("image2.png")
```

  
    
    original image
  
  
    
    detected skeleton
  
  
    
    skeleton to image
  

OmniGen can also directly use relevant information from input images to generate new images.

```python
import torch
from diffusers import OmniGenPipeline
from diffusers.utils import load_image 

pipe = OmniGenPipeline.from_pretrained(
    "Shitao/OmniGen-v1-diffusers",
    torch_dtype=torch.bfloat16
)
pipe.to("cuda")

prompt="Following the pose of this image <img><|image_1|></img>, generate a new photo: A young boy is sitting on a sofa in the library, holding a book. His hair is neatly combed, and a faint smile plays on his lips, with a few freckles scattered across his cheeks. The library is quiet, with rows of shelves filled with books stretching out behind him."
input_images=[load_image("https://raw.githubusercontent.com/VectorSpaceLab/OmniGen/main/imgs/docs_img/edit.png")]
image = pipe(
    prompt=prompt, 
    input_images=input_images, 
    guidance_scale=2, 
    img_guidance_scale=1.6,
    use_input_image_size_as_output=True,
    generator=torch.Generator(device="cpu").manual_seed(0)
).images[0]
image.save("output.png")
```

  
    
    generated image
  

## ID and object preserving

OmniGen can generate multiple images based on the people and objects in the input image and supports inputting multiple images simultaneously. 
Additionally, OmniGen can extract desired objects from an image containing multiple objects based on instructions.

```python
import torch
from diffusers import OmniGenPipeline
from diffusers.utils import load_image 

pipe = OmniGenPipeline.from_pretrained(
    "Shitao/OmniGen-v1-diffusers",
    torch_dtype=torch.bfloat16
)
pipe.to("cuda")

prompt="A man and a woman are sitting at a classroom desk. The man is the man with yellow hair in <img><|image_1|></img>. The woman is the woman on the left of <img><|image_2|></img>"
input_image_1 = load_image("https://raw.githubusercontent.com/VectorSpaceLab/OmniGen/main/imgs/docs_img/3.png")
input_image_2 = load_image("https://raw.githubusercontent.com/VectorSpaceLab/OmniGen/main/imgs/docs_img/4.png")
input_images=[input_image_1, input_image_2]
image = pipe(
    prompt=prompt, 
    input_images=input_images, 
    height=1024,
    width=1024,
    guidance_scale=2.5, 
    img_guidance_scale=1.6,
    generator=torch.Generator(device="cpu").manual_seed(666)
).images[0]
image.save("output.png")
```

  
    
    input_image_1
  
  
    
    input_image_2
  
  
    
    generated image
  

```py
import torch
from diffusers import OmniGenPipeline
from diffusers.utils import load_image 

pipe = OmniGenPipeline.from_pretrained(
    "Shitao/OmniGen-v1-diffusers",
    torch_dtype=torch.bfloat16
)
pipe.to("cuda")

prompt="A woman is walking down the street, wearing a white long-sleeve blouse with lace details on the sleeves, paired with a blue pleated skirt. The woman is <img><|image_1|></img>. The long-sleeve blouse and a pleated skirt are <img><|image_2|></img>."
input_image_1 = load_image("https://raw.githubusercontent.com/VectorSpaceLab/OmniGen/main/imgs/docs_img/emma.jpeg")
input_image_2 = load_image("https://raw.githubusercontent.com/VectorSpaceLab/OmniGen/main/imgs/docs_img/dress.jpg")
input_images=[input_image_1, input_image_2]
image = pipe(
    prompt=prompt, 
    input_images=input_images, 
    height=1024,
    width=1024,
    guidance_scale=2.5, 
    img_guidance_scale=1.6,
    generator=torch.Generator(device="cpu").manual_seed(666)
).images[0]
image.save("output.png")
```

  
    
    person image
  
  
    
    clothe image
  
  
    
    generated image
  

## Optimization when using multiple images 

For text-to-image task, OmniGen requires minimal memory and time costs (9GB memory and 31s for a 1024x1024 image on A800 GPU). 
However, when using input images, the computational cost increases. 

Here are some guidelines to help you reduce computational costs when using multiple images. The experiments are conducted on an A800 GPU with two input images.

Like other pipelines, you can reduce memory usage by offloading the model: `pipe.enable_model_cpu_offload()` or `pipe.enable_sequential_cpu_offload() `. 
In OmniGen, you can also decrease computational overhead by reducing the `max_input_image_size`. 
The memory consumption for different image sizes is shown in the table below:

| Method                    | Memory Usage |
|---------------------------|--------------|
| max_input_image_size=1024 | 40GB         |
| max_input_image_size=512  | 17GB         |
| max_input_image_size=256  | 14GB         |

## OmniGenPipeline[[diffusers.OmniGenPipeline]]

#### diffusers.OmniGenPipeline[[diffusers.OmniGenPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/omnigen/pipeline_omnigen.py#L119)

The OmniGen pipeline for multimodal-to-image generation.

Reference: https://huggingface.co/papers/2409.11340

__call__diffusers.OmniGenPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/omnigen/pipeline_omnigen.py#L330[{"name": "prompt", "val": ": str | list[str]"}, {"name": "input_images", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | list[PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor]] = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "max_input_image_size", "val": ": int = 1024"}, {"name": "timesteps", "val": ": list = None"}, {"name": "guidance_scale", "val": ": float = 2.5"}, {"name": "img_guidance_scale", "val": ": float = 1.6"}, {"name": "use_input_image_size_as_output", "val": ": bool = False"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If the input includes images, need to add
  placeholders `` in the prompt to indicate the position of the i-th images.
- **input_images** (`PipelineImageInput` or `list[PipelineImageInput]`, *optional*) --
  The list of input images. We will replace the "" in prompt with the i-th image in list.
- **height** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The height in pixels of the generated image. This is set to 1024 by default for the best results.
- **width** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The width in pixels of the generated image. This is set to 1024 by default for the best results.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **max_input_image_size** (`int`, *optional*, defaults to 1024) --
  the maximum size of input image, which will be used to crop the input image to the maximum size
- **timesteps** (`list[int]`, *optional*) --
  Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument
  in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is
  passed will be used. Must be in descending order.
- **guidance_scale** (`float`, *optional*, defaults to 2.5) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2.
  of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting
  `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to
  the text `prompt`, usually at the expense of lower image quality.
- **img_guidance_scale** (`float`, *optional*, defaults to 1.6) --
  Defined as equation 3 in [Instrucpix2pix](https://huggingface.co/papers/2211.09800).
- **use_input_image_size_as_output** (bool, defaults to False) --
  whether to use the input image size as the output image size, which can be used for single-image input,
  e.g., image editing task
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.flux.FluxPipelineOutput` instead of a plain tuple.
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`list`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.0

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import OmniGenPipeline

>>> pipe = OmniGenPipeline.from_pretrained("Shitao/OmniGen-v1-diffusers", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> prompt = "A cat holding a sign that says hello world"
>>> # Depending on the variant being used, the pipeline call will slightly vary.
>>> # Refer to the pipeline documentation for more details.
>>> image = pipe(prompt, num_inference_steps=50, guidance_scale=2.5).images[0]
>>> image.save("output.png")
```

Returns: [ImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`:
If `return_dict` is `True`, [ImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) is returned, otherwise a `tuple` is returned
where the first element is a list with the generated images.

**Parameters:**

transformer ([OmniGenTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/omnigen_transformer#diffusers.OmniGenTransformer2DModel)) : Autoregressive Transformer architecture for OmniGen.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

tokenizer (`LlamaTokenizer`) : Text tokenizer of class. [LlamaTokenizer](https://huggingface.co/docs/transformers/main/model_doc/llama#transformers.LlamaTokenizer).
#### disable_vae_slicing[[diffusers.OmniGenPipeline.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/omnigen/pipeline_omnigen.py#L246)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### disable_vae_tiling[[diffusers.OmniGenPipeline.disable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/omnigen/pipeline_omnigen.py#L273)

Disable tiled VAE decoding. If `enable_vae_tiling` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_vae_slicing[[diffusers.OmniGenPipeline.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/omnigen/pipeline_omnigen.py#L233)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### enable_vae_tiling[[diffusers.OmniGenPipeline.enable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/omnigen/pipeline_omnigen.py#L259)

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.
#### encode_input_images[[diffusers.OmniGenPipeline.encode_input_images]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/omnigen/pipeline_omnigen.py#L171)

get the continue embedding of input images by VAE

Returns: torch.Tensor

**Parameters:**

input_pixel_values : normalized pixel of input images

device --
