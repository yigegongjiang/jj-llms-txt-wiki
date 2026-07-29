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

# VisualCloze

[VisualCloze: A Universal Image Generation Framework via Visual In-Context Learning](https://huggingface.co/papers/2504.07960) is an innovative in-context learning based universal image generation framework that offers key capabilities:
1. Support for various in-domain tasks
2. Generalization to unseen tasks through in-context learning
3. Unify multiple tasks into one step and generate both target image and intermediate results
4. Support reverse-engineering conditions from target images

## Overview

The abstract from the paper is:

*Recent progress in diffusion models significantly advances various image generation tasks. However, the current mainstream approach remains focused on building task-specific models, which have limited efficiency when supporting a wide range of different needs. While universal models attempt to address this limitation, they face critical challenges, including generalizable task instruction, appropriate task distributions, and unified architectural design. To tackle these challenges, we propose VisualCloze, a universal image generation framework, which supports a wide range of in-domain tasks, generalization to unseen ones, unseen unification of multiple tasks, and reverse generation. Unlike existing methods that rely on language-based task instruction, leading to task ambiguity and weak generalization, we integrate visual in-context learning, allowing models to identify tasks from visual demonstrations. Meanwhile, the inherent sparsity of visual task distributions hampers the learning of transferable knowledge across tasks. To this end, we introduce Graph200K, a graph-structured dataset that establishes various interrelated tasks, enhancing task density and transferable knowledge. Furthermore, we uncover that our unified image generation formulation shared a consistent objective with image infilling, enabling us to leverage the strong generative priors of pre-trained infilling models without modifying the architectures. The codes, dataset, and models are available at https://visualcloze.github.io.*

## Inference

### Model loading

VisualCloze is a two-stage cascade pipeline, containing `VisualClozeGenerationPipeline` and `VisualClozeUpsamplingPipeline`.
- In `VisualClozeGenerationPipeline`, each image is downsampled before concatenating images into a grid layout, avoiding excessively high resolutions. VisualCloze releases two models suitable for diffusers, i.e., [VisualClozePipeline-384](https://huggingface.co/VisualCloze/VisualClozePipeline-384) and [VisualClozePipeline-512](https://huggingface.co/VisualCloze/VisualClozePipeline-384), which downsample images to resolutions of 384 and 512, respectively. 
- `VisualClozeUpsamplingPipeline` uses [SDEdit](https://huggingface.co/papers/2108.01073) to enable high-resolution image synthesis.

The `VisualClozePipeline` integrates both stages to support convenient end-to-end sampling, while also allowing users to utilize each pipeline independently as needed.

### Input Specifications

#### Task and Content Prompts
- Task prompt: Required to describe the generation task intention
- Content prompt: Optional description or caption of the target image
- When content prompt is not needed, pass `None`
- For batch inference, pass `List[str|None]`

#### Image Input Format
- Format: `List[List[Image|None]]`
- Structure:
  - All rows except the last represent in-context examples
  - Last row represents the current query (target image set to `None`)
- For batch inference, pass `List[List[List[Image|None]]]`

#### Resolution Control
- Default behavior:
  - Initial generation in the first stage: area of ${pipe.resolution}^2$
  - Upsampling in the second stage: 3x factor
- Custom resolution: Adjust using `upsampling_height` and `upsampling_width` parameters

### Examples

For comprehensive examples covering a wide range of tasks, please refer to the [Online Demo](https://huggingface.co/spaces/VisualCloze/VisualCloze) and [GitHub Repository](https://github.com/lzyhha/VisualCloze). Below are simple examples for three cases: mask-to-image conversion, edge detection, and subject-driven generation.

#### Example for mask2image

```python
import torch
from diffusers import VisualClozePipeline
from diffusers.utils import load_image

pipe = VisualClozePipeline.from_pretrained("VisualCloze/VisualClozePipeline-384", resolution=384, torch_dtype=torch.bfloat16)
pipe.to("cuda")

# Load in-context images (make sure the paths are correct and accessible)
image_paths = [
    # in-context examples
    [
        load_image('https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_mask2image_incontext-example-1_mask.jpg'),
        load_image('https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_mask2image_incontext-example-1_image.jpg'),
    ],
    # query with the target image
    [
        load_image('https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_mask2image_query_mask.jpg'),
        None, # No image needed for the target image
    ],
]

# Task and content prompt
task_prompt = "In each row, a logical task is demonstrated to achieve [IMAGE2] an aesthetically pleasing photograph based on [IMAGE1] sam 2-generated masks with rich color coding."
content_prompt = """Majestic photo of a golden eagle perched on a rocky outcrop in a mountainous landscape. 
The eagle is positioned in the right foreground, facing left, with its sharp beak and keen eyes prominently visible. 
Its plumage is a mix of dark brown and golden hues, with intricate feather details. 
The background features a soft-focus view of snow-capped mountains under a cloudy sky, creating a serene and grandiose atmosphere. 
The foreground includes rugged rocks and patches of green moss. Photorealistic, medium depth of field, 
soft natural lighting, cool color palette, high contrast, sharp focus on the eagle, blurred background, 
tranquil, majestic, wildlife photography."""

# Run the pipeline
image_result = pipe(
    task_prompt=task_prompt,
    content_prompt=content_prompt,
    image=image_paths,
    upsampling_width=1344,
    upsampling_height=768,
    upsampling_strength=0.4,
    guidance_scale=30,
    num_inference_steps=30,
    max_sequence_length=512,
    generator=torch.Generator("cpu").manual_seed(0)
).images[0][0]

# Save the resulting image
image_result.save("visualcloze.png")
```

#### Example for edge-detection

```python
import torch
from diffusers import VisualClozePipeline
from diffusers.utils import load_image

pipe = VisualClozePipeline.from_pretrained("VisualCloze/VisualClozePipeline-384", resolution=384, torch_dtype=torch.bfloat16)
pipe.to("cuda")

# Load in-context images (make sure the paths are correct and accessible)
image_paths = [
    # in-context examples
    [
        load_image('https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_edgedetection_incontext-example-1_image.jpg'),
        load_image('https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_edgedetection_incontext-example-1_edge.jpg'),
    ],
    [
        load_image('https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_edgedetection_incontext-example-2_image.jpg'),
        load_image('https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_edgedetection_incontext-example-2_edge.jpg'),
    ],
    # query with the target image
    [
        load_image('https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_edgedetection_query_image.jpg'),
        None, # No image needed for the target image
    ],
]

# Task and content prompt
task_prompt = "Each row illustrates a pathway from [IMAGE1] a sharp and beautifully composed photograph to [IMAGE2] edge map with natural well-connected outlines using a clear logical task."
content_prompt = ""

# Run the pipeline
image_result = pipe(
    task_prompt=task_prompt,
    content_prompt=content_prompt,
    image=image_paths,
    upsampling_width=864,
    upsampling_height=1152,
    upsampling_strength=0.4,
    guidance_scale=30,
    num_inference_steps=30,
    max_sequence_length=512,
    generator=torch.Generator("cpu").manual_seed(0)
).images[0][0]

# Save the resulting image
image_result.save("visualcloze.png")
```

#### Example for subject-driven generation

```python
import torch
from diffusers import VisualClozePipeline
from diffusers.utils import load_image

pipe = VisualClozePipeline.from_pretrained("VisualCloze/VisualClozePipeline-384", resolution=384, torch_dtype=torch.bfloat16)
pipe.to("cuda")

# Load in-context images (make sure the paths are correct and accessible)
image_paths = [
    # in-context examples
    [
        load_image('https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_subjectdriven_incontext-example-1_reference.jpg'),
        load_image('https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_subjectdriven_incontext-example-1_depth.jpg'),
        load_image('https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_subjectdriven_incontext-example-1_image.jpg'),
    ],
    [
        load_image('https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_subjectdriven_incontext-example-2_reference.jpg'),
        load_image('https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_subjectdriven_incontext-example-2_depth.jpg'),
        load_image('https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_subjectdriven_incontext-example-2_image.jpg'),
    ],
    # query with the target image
    [
        load_image('https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_subjectdriven_query_reference.jpg'),
        load_image('https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_subjectdriven_query_depth.jpg'),
        None, # No image needed for the target image
    ],
]

# Task and content prompt
task_prompt = """Each row describes a process that begins with [IMAGE1] an image containing the key object, 
[IMAGE2] depth map revealing gray-toned spatial layers and results in 
[IMAGE3] an image with artistic qualitya high-quality image with exceptional detail."""
content_prompt = """A vintage porcelain collector's item. Beneath a blossoming cherry tree in early spring, 
this treasure is photographed up close, with soft pink petals drifting through the air and vibrant blossoms framing the scene."""

# Run the pipeline
image_result = pipe(
    task_prompt=task_prompt,
    content_prompt=content_prompt,
    image=image_paths,
    upsampling_width=1024,
    upsampling_height=1024,
    upsampling_strength=0.2,
    guidance_scale=30,
    num_inference_steps=30,
    max_sequence_length=512,
    generator=torch.Generator("cpu").manual_seed(0)
).images[0][0]

# Save the resulting image
image_result.save("visualcloze.png")
```

#### Utilize each pipeline independently 

```python
import torch
from diffusers import VisualClozeGenerationPipeline, FluxFillPipeline as VisualClozeUpsamplingPipeline
from diffusers.utils import load_image
from PIL import Image

pipe = VisualClozeGenerationPipeline.from_pretrained(
    "VisualCloze/VisualClozePipeline-384", resolution=384, torch_dtype=torch.bfloat16
)
pipe.to("cuda")

image_paths = [
    # in-context examples
    [
        load_image(
            "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_mask2image_incontext-example-1_mask.jpg"
        ),
        load_image(
            "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_mask2image_incontext-example-1_image.jpg"
        ),
    ],
    # query with the target image
    [
        load_image(
            "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_mask2image_query_mask.jpg"
        ),
        None,  # No image needed for the target image
    ],
]
task_prompt = "In each row, a logical task is demonstrated to achieve [IMAGE2] an aesthetically pleasing photograph based on [IMAGE1] sam 2-generated masks with rich color coding."
content_prompt = "Majestic photo of a golden eagle perched on a rocky outcrop in a mountainous landscape. The eagle is positioned in the right foreground, facing left, with its sharp beak and keen eyes prominently visible. Its plumage is a mix of dark brown and golden hues, with intricate feather details. The background features a soft-focus view of snow-capped mountains under a cloudy sky, creating a serene and grandiose atmosphere. The foreground includes rugged rocks and patches of green moss. Photorealistic, medium depth of field, soft natural lighting, cool color palette, high contrast, sharp focus on the eagle, blurred background, tranquil, majestic, wildlife photography."

# Stage 1: Generate initial image
image = pipe(
    task_prompt=task_prompt,
    content_prompt=content_prompt,
    image=image_paths,
    guidance_scale=30,
    num_inference_steps=30,
    max_sequence_length=512,
    generator=torch.Generator("cpu").manual_seed(0),
).images[0][0]

# Stage 2 (optional): Upsample the generated image
pipe_upsample = VisualClozeUpsamplingPipeline.from_pipe(pipe)
pipe_upsample.to("cuda")

mask_image = Image.new("RGB", image.size, (255, 255, 255))

image = pipe_upsample(
    image=image,
    mask_image=mask_image,
    prompt=content_prompt,
    width=1344,
    height=768,
    strength=0.4,
    guidance_scale=30,
    num_inference_steps=30,
    max_sequence_length=512,
    generator=torch.Generator("cpu").manual_seed(0),
).images[0]

image.save("visualcloze.png")
```

## VisualClozePipeline[[diffusers.VisualClozePipeline]]

#### diffusers.VisualClozePipeline[[diffusers.VisualClozePipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/visualcloze/pipeline_visualcloze_combined.py#L89)

The VisualCloze pipeline for image generation with visual context. Reference:
https://github.com/lzyhha/VisualCloze/tree/main. This pipeline is designed to generate images based on visual
in-context examples.

__call__diffusers.VisualClozePipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/visualcloze/pipeline_visualcloze_combined.py#L249[{"name": "task_prompt", "val": ": str | list[str] = None"}, {"name": "content_prompt", "val": ": str | list[str] = None"}, {"name": "image", "val": ": torch.FloatTensor | None = None"}, {"name": "upsampling_height", "val": ": int | None = None"}, {"name": "upsampling_width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "sigmas", "val": ": list[float] | None = None"}, {"name": "guidance_scale", "val": ": float = 30.0"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.FloatTensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.FloatTensor | None = None"}, {"name": "pooled_prompt_embeds", "val": ": torch.FloatTensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "joint_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}, {"name": "upsampling_strength", "val": ": float = 1.0"}]- **task_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to define the task intention.
- **content_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to define the content or caption of the target image to be generated.
- **image** (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) --
  `Image`, numpy array or tensor representing an image batch to be used as the starting point. For both
  numpy array and pytorch tensor, the expected value range is between `[0, 1]` If it's a tensor or a list
  or tensors, the expected shape should be `(B, C, H, W)` or `(C, H, W)`. If it is a numpy array or a
  list of arrays, the expected shape should be `(B, H, W, C)` or `(H, W, C)`.
- **upsampling_height** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The height in pixels of the generated image (i.e., output image) after upsampling via SDEdit. By
  default, the image is upsampled by a factor of three, and the base resolution is determined by the
  resolution parameter of the pipeline. When only one of `upsampling_height` or `upsampling_width` is
  specified, the other will be automatically set based on the aspect ratio.
- **upsampling_width** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The width in pixels of the generated image (i.e., output image) after upsampling via SDEdit. By
  default, the image is upsampled by a factor of three, and the base resolution is determined by the
  resolution parameter of the pipeline. When only one of `upsampling_height` or `upsampling_width` is
  specified, the other will be automatically set based on the aspect ratio.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **sigmas** (`list[float]`, *optional*) --
  Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in
  their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed
  will be used.
- **guidance_scale** (`float`, *optional*, defaults to 30.0) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2.
  of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting
  `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to
  the text `prompt`, usually at the expense of lower image quality.
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.FloatTensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **pooled_prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting.
  If not provided, pooled text embeddings will be generated from `prompt` input argument.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.flux.FluxPipelineOutput` instead of a plain tuple.
- **joint_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`list`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.
- **max_sequence_length** (`int` defaults to 512) -- Maximum sequence length to use with the `prompt`.
- **upsampling_strength** (`float`, *optional*, defaults to 1.0) --
  Indicates extent to transform the reference `image` when upsampling the results. Must be between 0 and
  1. The generated image is used as a starting point and more noise is added the higher the
  `upsampling_strength`. The number of denoising steps depends on the amount of noise initially added.
  When `upsampling_strength` is 1, added noise is maximum and the denoising process runs for the full
  number of iterations specified in `num_inference_steps`. A value of 0 skips the upsampling step and
  output the results at the resolution of `self.resolution`.0`~pipelines.flux.FluxPipelineOutput` or `tuple``~pipelines.flux.FluxPipelineOutput` if `return_dict`
is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the generated
images.

Function invoked when calling the VisualCloze pipeline for generation.

Examples:
```python
>>> import torch
>>> from diffusers import VisualClozePipeline
>>> from diffusers.utils import load_image

>>> image_paths = [
...     # in-context examples
...     [
...         load_image(
...             "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_mask2image_incontext-example-1_mask.jpg"
...         ),
...         load_image(
...             "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_mask2image_incontext-example-1_image.jpg"
...         ),
...     ],
...     # query with the target image
...     [
...         load_image(
...             "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_mask2image_query_mask.jpg"
...         ),
...         None,  # No image needed for the target image
...     ],
... ]
>>> task_prompt = "In each row, a logical task is demonstrated to achieve [IMAGE2] an aesthetically pleasing photograph based on [IMAGE1] sam 2-generated masks with rich color coding."
>>> content_prompt = "Majestic photo of a golden eagle perched on a rocky outcrop in a mountainous landscape. The eagle is positioned in the right foreground, facing left, with its sharp beak and keen eyes prominently visible. Its plumage is a mix of dark brown and golden hues, with intricate feather details. The background features a soft-focus view of snow-capped mountains under a cloudy sky, creating a serene and grandiose atmosphere. The foreground includes rugged rocks and patches of green moss. Photorealistic, medium depth of field, soft natural lighting, cool color palette, high contrast, sharp focus on the eagle, blurred background, tranquil, majestic, wildlife photography."
>>> pipe = VisualClozePipeline.from_pretrained(
...     "VisualCloze/VisualClozePipeline-384", resolution=384, torch_dtype=torch.bfloat16
... )
>>> pipe.to("cuda")

>>> image = pipe(
...     task_prompt=task_prompt,
...     content_prompt=content_prompt,
...     image=image_paths,
...     upsampling_width=1344,
...     upsampling_height=768,
...     upsampling_strength=0.4,
...     guidance_scale=30,
...     num_inference_steps=30,
...     max_sequence_length=512,
...     generator=torch.Generator("cpu").manual_seed(0),
... ).images[0][0]
>>> image.save("visualcloze.png")
```

**Parameters:**

transformer ([FluxTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/flux_transformer#diffusers.FluxTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

text_encoder_2 (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer_2 (`T5TokenizerFast`) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

resolution (`int`, *optional*, defaults to 384) : The resolution of each image when concatenating images from the query and in-context examples.

**Returns:**

``~pipelines.flux.FluxPipelineOutput` or `tuple``

`~pipelines.flux.FluxPipelineOutput` if `return_dict`
is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the generated
images.

## VisualClozeGenerationPipeline[[diffusers.VisualClozeGenerationPipeline]]

#### diffusers.VisualClozeGenerationPipeline[[diffusers.VisualClozeGenerationPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/visualcloze/pipeline_visualcloze_generation.py#L119)

The VisualCloze pipeline for image generation with visual context. Reference:
https://github.com/lzyhha/VisualCloze/tree/main This pipeline is designed to generate images based on visual
in-context examples.

__call__diffusers.VisualClozeGenerationPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/visualcloze/pipeline_visualcloze_generation.py#L708[{"name": "task_prompt", "val": ": str | list[str] = None"}, {"name": "content_prompt", "val": ": str | list[str] = None"}, {"name": "image", "val": ": torch.FloatTensor | None = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "sigmas", "val": ": list[float] | None = None"}, {"name": "guidance_scale", "val": ": float = 30.0"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.FloatTensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.FloatTensor | None = None"}, {"name": "pooled_prompt_embeds", "val": ": torch.FloatTensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "joint_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}]- **task_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to define the task intention.
- **content_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to define the content or caption of the target image to be generated.
- **image** (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) --
  `Image`, numpy array or tensor representing an image batch to be used as the starting point. For both
  numpy array and pytorch tensor, the expected value range is between `[0, 1]` If it's a tensor or a list
  or tensors, the expected shape should be `(B, C, H, W)` or `(C, H, W)`. If it is a numpy array or a
  list of arrays, the expected shape should be `(B, H, W, C)` or `(H, W, C)`.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **sigmas** (`list[float]`, *optional*) --
  Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in
  their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed
  will be used.
- **guidance_scale** (`float`, *optional*, defaults to 30.0) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2.
  of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting
  `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to
  the text `prompt`, usually at the expense of lower image quality.
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.FloatTensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **pooled_prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting.
  If not provided, pooled text embeddings will be generated from `prompt` input argument.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.flux.FluxPipelineOutput` instead of a plain tuple.
- **joint_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`list`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.
- **max_sequence_length** (`int` defaults to 512) -- Maximum sequence length to use with the `prompt`.0`~pipelines.flux.FluxPipelineOutput` or `tuple``~pipelines.flux.FluxPipelineOutput` if `return_dict`
is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the generated
images.

Function invoked when calling the VisualCloze pipeline for generation.

Examples:
```python
>>> import torch
>>> from diffusers import VisualClozeGenerationPipeline, FluxFillPipeline as VisualClozeUpsamplingPipeline
>>> from diffusers.utils import load_image
>>> from PIL import Image

>>> image_paths = [
...     # in-context examples
...     [
...         load_image(
...             "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_mask2image_incontext-example-1_mask.jpg"
...         ),
...         load_image(
...             "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_mask2image_incontext-example-1_image.jpg"
...         ),
...     ],
...     # query with the target image
...     [
...         load_image(
...             "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/visualcloze/visualcloze_mask2image_query_mask.jpg"
...         ),
...         None,  # No image needed for the target image
...     ],
... ]
>>> task_prompt = "In each row, a logical task is demonstrated to achieve [IMAGE2] an aesthetically pleasing photograph based on [IMAGE1] sam 2-generated masks with rich color coding."
>>> content_prompt = "Majestic photo of a golden eagle perched on a rocky outcrop in a mountainous landscape. The eagle is positioned in the right foreground, facing left, with its sharp beak and keen eyes prominently visible. Its plumage is a mix of dark brown and golden hues, with intricate feather details. The background features a soft-focus view of snow-capped mountains under a cloudy sky, creating a serene and grandiose atmosphere. The foreground includes rugged rocks and patches of green moss. Photorealistic, medium depth of field, soft natural lighting, cool color palette, high contrast, sharp focus on the eagle, blurred background, tranquil, majestic, wildlife photography."
>>> pipe = VisualClozeGenerationPipeline.from_pretrained(
...     "VisualCloze/VisualClozePipeline-384", resolution=384, torch_dtype=torch.bfloat16
... )
>>> pipe.to("cuda")

>>> image = pipe(
...     task_prompt=task_prompt,
...     content_prompt=content_prompt,
...     image=image_paths,
...     guidance_scale=30,
...     num_inference_steps=30,
...     max_sequence_length=512,
...     generator=torch.Generator("cpu").manual_seed(0),
... ).images[0][0]

>>> # optional, upsampling the generated image
>>> pipe_upsample = VisualClozeUpsamplingPipeline.from_pipe(pipe)
>>> pipe_upsample.to("cuda")

>>> mask_image = Image.new("RGB", image.size, (255, 255, 255))

>>> image = pipe_upsample(
...     image=image,
...     mask_image=mask_image,
...     prompt=content_prompt,
...     width=1344,
...     height=768,
...     strength=0.4,
...     guidance_scale=30,
...     num_inference_steps=30,
...     max_sequence_length=512,
...     generator=torch.Generator("cpu").manual_seed(0),
... ).images[0]

>>> image.save("visualcloze.png")
```

**Parameters:**

transformer ([FluxTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/flux_transformer#diffusers.FluxTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

text_encoder_2 (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer_2 (`T5TokenizerFast`) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

resolution (`int`, *optional*, defaults to 384) : The resolution of each image when concatenating images from the query and in-context examples.

**Returns:**

``~pipelines.flux.FluxPipelineOutput` or `tuple``

`~pipelines.flux.FluxPipelineOutput` if `return_dict`
is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the generated
images.
#### disable_vae_slicing[[diffusers.VisualClozeGenerationPipeline.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/visualcloze/pipeline_visualcloze_generation.py#L536)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### disable_vae_tiling[[diffusers.VisualClozeGenerationPipeline.disable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/visualcloze/pipeline_visualcloze_generation.py#L563)

Disable tiled VAE decoding. If `enable_vae_tiling` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_vae_slicing[[diffusers.VisualClozeGenerationPipeline.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/visualcloze/pipeline_visualcloze_generation.py#L523)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### enable_vae_tiling[[diffusers.VisualClozeGenerationPipeline.enable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/visualcloze/pipeline_visualcloze_generation.py#L549)

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.
#### encode_prompt[[diffusers.VisualClozeGenerationPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/visualcloze/pipeline_visualcloze_generation.py#L288)

**Parameters:**

layout_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to define the number of in-context examples and the number of images involved in the task.

task_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to define the task intention.

content_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to define the content or caption of the target image to be generated.

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.
