# Kandinsky 5.0 Image

[Kandinsky 5.0](https://arxiv.org/abs/2511.14993) is a family of diffusion models for Video & Image generation. 

Kandinsky 5.0 Image Lite is a lightweight image generation model (6B parameters).

The model introduces several key innovations:
- **Latent diffusion pipeline** with **Flow Matching** for improved training stability
- **Diffusion Transformer (DiT)** as the main generative backbone with cross-attention to text embeddings
- Dual text encoding using **Qwen2.5-VL** and **CLIP** for comprehensive text understanding
- **Flux VAE** for efficient image encoding and decoding

The original codebase can be found at [kandinskylab/Kandinsky-5](https://github.com/kandinskylab/Kandinsky-5).

> [!TIP]
> Check out the [Kandinsky Lab](https://huggingface.co/kandinskylab) organization on the Hub for the official model checkpoints for text-to-video generation, including pretrained, SFT, no-CFG, and distilled variants.

## Available Models

Kandinsky 5.0 Image Lite:

| model_id | Description | Use Cases |
|------------|-------------|-----------|
| [**kandinskylab/Kandinsky-5.0-T2I-Lite-sft-Diffusers**](https://huggingface.co/kandinskylab/Kandinsky-5.0-T2I-Lite-sft-Diffusers) | 6B image Supervised Fine-Tuned model | Highest generation quality |
| [**kandinskylab/Kandinsky-5.0-I2I-Lite-sft-Diffusers**](https://huggingface.co/kandinskylab/Kandinsky-5.0-I2I-Lite-sft-Diffusers) | 6B image editing Supervised Fine-Tuned model | Highest generation quality |
| [**kandinskylab/Kandinsky-5.0-T2I-Lite-pretrain-Diffusers**](https://huggingface.co/kandinskylab/Kandinsky-5.0-T2I-Lite-pretrain-Diffusers) | 6B image Base pretrained model | Research and fine-tuning |
| [**kandinskylab/Kandinsky-5.0-I2I-Lite-pretrain-Diffusers**](https://huggingface.co/kandinskylab/Kandinsky-5.0-I2I-Lite-pretrain-Diffusers) | 6B image editing Base pretrained model | Research and fine-tuning |

## Usage Examples

### Basic Text-to-Image Generation

```python
import torch
from diffusers import Kandinsky5T2IPipeline

# Load the pipeline
model_id = "kandinskylab/Kandinsky-5.0-T2I-Lite-sft-Diffusers"
pipe = Kandinsky5T2IPipeline.from_pretrained(model_id)
_ = pipe.to(device='cuda',dtype=torch.bfloat16)

# Generate image
prompt = "A fluffy, expressive cat wearing a bright red hat with a soft, slightly textured fabric. The hat should look cozy and well-fitted on the cat’s head. On the front of the hat, add clean, bold white text that reads “SWEET”, clearly visible and neatly centered. Ensure the overall lighting highlights the hat’s color and the cat’s fur details."

output = pipe(
    prompt=prompt,
    negative_prompt="",
    height=1024,
    width=1024,
    num_inference_steps=50,
    guidance_scale=3.5,
).image[0]
```

### Basic Image-to-Image Generation

```python
import torch
from diffusers import Kandinsky5I2IPipeline
from diffusers.utils import load_image 
# Load the pipeline
model_id = "kandinskylab/Kandinsky-5.0-I2I-Lite-sft-Diffusers"
pipe = Kandinsky5I2IPipeline.from_pretrained(model_id)

_ = pipe.to(device='cuda',dtype=torch.bfloat16)
pipe.enable_model_cpu_offload()                                               # <--- Enable CPU offloading for single GPU inference

# Edit the input image
image = load_image(
    "https://huggingface.co/kandinsky-community/kandinsky-3/resolve/main/assets/title.jpg?download=true"
)

prompt = "Change the background from a winter night scene to a bright summer day. Place the character on a sandy beach with clear blue sky, soft sunlight, and gentle waves in the distance. Replace the winter clothing with a light short-sleeved T-shirt (in soft pastel colors) and casual shorts. Ensure the character’s fur reflects warm daylight instead of cold winter tones. Add small beach details such as seashells, footprints in the sand, and a few scattered beach toys nearby. Keep the oranges in the scene, but place them naturally on the sand."
negative_prompt = ""

output = pipe(
    image=image,
    prompt=prompt,
    negative_prompt=negative_prompt,
    guidance_scale=3.5,
).image[0]
```

## Kandinsky5T2IPipeline[[diffusers.Kandinsky5T2IPipeline]]

#### diffusers.Kandinsky5T2IPipeline[[diffusers.Kandinsky5T2IPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky_t2i.py#L120)

Pipeline for text-to-image generation using Kandinsky 5.0.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

__call__diffusers.Kandinsky5T2IPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky_t2i.py#L534[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "height", "val": ": int = 1024"}, {"name": "width", "val": ": int = 1024"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "guidance_scale", "val": ": float = 3.5"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds_qwen", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds_clip", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds_qwen", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds_clip", "val": ": torch.Tensor | None = None"}, {"name": "prompt_cu_seqlens", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_cu_seqlens", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int, NoneType], diffusers.callbacks.PipelineCallback | diffusers.callbacks.MultiPipelineCallbacks]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, pass `prompt_embeds` instead.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to avoid during image generation. If not defined, pass `negative_prompt_embeds`
  instead. Ignored when not using guidance (`guidance_scale` 0`~KandinskyImagePipelineOutput` or `tuple`If `return_dict` is `True`, `KandinskyImagePipelineOutput` is returned, otherwise a `tuple` is
returned where the first element is a list with the generated images.

The call function to the pipeline for text-to-image generation.

Examples:

```python
>>> import torch
>>> from diffusers import Kandinsky5T2IPipeline

>>> # Available models:
>>> # kandinskylab/Kandinsky-5.0-T2I-Lite-sft-Diffusers
>>> # kandinskylab/Kandinsky-5.0-T2I-Lite-pretrain-Diffusers

>>> model_id = "kandinskylab/Kandinsky-5.0-T2I-Lite-sft-Diffusers"
>>> pipe = Kandinsky5T2IPipeline.from_pretrained(model_id, torch_dtype=torch.bfloat16)
>>> pipe = pipe.to("cuda")

>>> prompt = "A cat and a dog baking a cake together in a kitchen."

>>> output = pipe(
...     prompt=prompt,
...     negative_prompt="",
...     height=1024,
...     width=1024,
...     num_inference_steps=50,
...     guidance_scale=3.5,
... ).frames[0]
```

**Parameters:**

transformer (`Kandinsky5Transformer3DModel`) : Conditional Transformer to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder Model [black-forest-labs/FLUX.1-dev (vae)](https://huggingface.co/black-forest-labs/FLUX.1-dev) to encode and decode videos to and from latent representations.

text_encoder (`Qwen2_5_VLForConditionalGeneration`) : Frozen text-encoder [Qwen2.5-VL](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct).

tokenizer (`AutoProcessor`) : Tokenizer for Qwen2.5-VL.

text_encoder_2 (`CLIPTextModel`) : Frozen [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

tokenizer_2 (`CLIPTokenizer`) : Tokenizer for CLIP.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

**Returns:**

``~KandinskyImagePipelineOutput` or `tuple``

If `return_dict` is `True`, `KandinskyImagePipelineOutput` is returned, otherwise a `tuple` is
returned where the first element is a list with the generated images.
#### check_inputs[[diffusers.Kandinsky5T2IPipeline.check_inputs]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky_t2i.py#L380)

Validate input parameters for the pipeline.

**Parameters:**

prompt : Input prompt

negative_prompt : Negative prompt for guidance

height : Image height

width : Image width

prompt_embeds_qwen : Pre-computed Qwen prompt embeddings

prompt_embeds_clip : Pre-computed CLIP prompt embeddings

negative_prompt_embeds_qwen : Pre-computed Qwen negative prompt embeddings

negative_prompt_embeds_clip : Pre-computed CLIP negative prompt embeddings

prompt_cu_seqlens : Pre-computed cumulative sequence lengths for Qwen positive prompt

negative_prompt_cu_seqlens : Pre-computed cumulative sequence lengths for Qwen negative prompt

callback_on_step_end_tensor_inputs : Callback tensor inputs
#### encode_prompt[[diffusers.Kandinsky5T2IPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky_t2i.py#L289)

Encodes a single prompt (positive or negative) into text encoder hidden states.

This method combines embeddings from both Qwen2.5-VL and CLIP text encoders to create comprehensive text
representations for image generation.

**Parameters:**

prompt (`str` or `list[str]`) : Prompt to be encoded.

num_images_per_prompt (`int`, *optional*, defaults to 1) : Number of images to generate per prompt.

max_sequence_length (`int`, *optional*, defaults to 512) : Maximum sequence length for text encoding. Must be less than 1024

device (`torch.device`, *optional*) : Torch device.

dtype (`torch.dtype`, *optional*) : Torch dtype.

**Returns:**

`tuple[torch.Tensor, torch.Tensor, torch.Tensor]`

- Qwen text embeddings of shape (batch_size * num_images_per_prompt, sequence_length, embedding_dim)
- CLIP pooled embeddings of shape (batch_size * num_images_per_prompt, clip_embedding_dim)
- Cumulative sequence lengths (`cu_seqlens`) for Qwen embeddings of shape (batch_size *
  num_images_per_prompt + 1,)
#### prepare_latents[[diffusers.Kandinsky5T2IPipeline.prepare_latents]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky_t2i.py#L469)

Prepare initial latent variables for text-to-image generation.

This method creates random noise latents

**Parameters:**

batch_size (int) : Number of images to generate

num_channels_latents (int) : Number of channels in latent space

height (int) : Height of generated image

width (int) : Width of generated image

dtype (torch.dtype) : Data type for latents

device (torch.device) : Device to create latents on

generator (torch.Generator) : Random number generator

latents (torch.Tensor) : Pre-existing latents to use

**Returns:**

`torch.Tensor`

Prepared latent tensor

## Kandinsky5I2IPipeline[[diffusers.Kandinsky5I2IPipeline]]

#### diffusers.Kandinsky5I2IPipeline[[diffusers.Kandinsky5I2IPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky_i2i.py#L120)

Pipeline for image-to-image generation using Kandinsky 5.0.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

__call__diffusers.Kandinsky5I2IPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky_i2i.py#L567[{"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor]"}, {"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "guidance_scale", "val": ": float = 3.5"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds_qwen", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds_clip", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds_qwen", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds_clip", "val": ": torch.Tensor | None = None"}, {"name": "prompt_cu_seqlens", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_cu_seqlens", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int, NoneType], diffusers.callbacks.PipelineCallback | diffusers.callbacks.MultiPipelineCallbacks]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 1024"}]- **image** (`PipelineImageInput`) --
  The input image to condition the generation on. Must be an image, a list of images or a `torch.Tensor`.
- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, pass `prompt_embeds` instead.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to avoid during image generation. If not defined, pass `negative_prompt_embeds`
  instead. Ignored when not using guidance (`guidance_scale` 0`~KandinskyImagePipelineOutput` or `tuple`If `return_dict` is `True`, `KandinskyImagePipelineOutput` is returned, otherwise a `tuple` is
returned where the first element is a list with the generated images.

The call function to the pipeline for image-to-image generation.

Examples:

```python
>>> import torch
>>> from diffusers import Kandinsky5I2IPipeline

>>> # Available models:
>>> # kandinskylab/Kandinsky-5.0-I2I-Lite-sft-Diffusers
>>> # kandinskylab/Kandinsky-5.0-I2I-Lite-pretrain-Diffusers

>>> model_id = "kandinskylab/Kandinsky-5.0-I2I-Lite-sft-Diffusers"
>>> pipe = Kandinsky5I2IPipeline.from_pretrained(model_id, torch_dtype=torch.bfloat16)
>>> pipe = pipe.to("cuda")

>>> prompt = "A cat and a dog baking a cake together in a kitchen."

>>> output = pipe(
...     prompt=prompt,
...     negative_prompt="",
...     height=1024,
...     width=1024,
...     num_inference_steps=50,
...     guidance_scale=3.5,
... ).frames[0]
```

**Parameters:**

transformer (`Kandinsky5Transformer3DModel`) : Conditional Transformer to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder Model [black-forest-labs/FLUX.1-dev (vae)](https://huggingface.co/black-forest-labs/FLUX.1-dev) to encode and decode videos to and from latent representations.

text_encoder (`Qwen2_5_VLForConditionalGeneration`) : Frozen text-encoder [Qwen2.5-VL](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct).

tokenizer (`AutoProcessor`) : Tokenizer for Qwen2.5-VL.

text_encoder_2 (`CLIPTextModel`) : Frozen [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

tokenizer_2 (`CLIPTokenizer`) : Tokenizer for CLIP.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

**Returns:**

``~KandinskyImagePipelineOutput` or `tuple``

If `return_dict` is `True`, `KandinskyImagePipelineOutput` is returned, otherwise a `tuple` is
returned where the first element is a list with the generated images.
#### check_inputs[[diffusers.Kandinsky5I2IPipeline.check_inputs]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky_i2i.py#L388)

Validate input parameters for the pipeline.

**Parameters:**

prompt : Input prompt

negative_prompt : Negative prompt for guidance

image : Input image for conditioning

height : Image height

width : Image width

prompt_embeds_qwen : Pre-computed Qwen prompt embeddings

prompt_embeds_clip : Pre-computed CLIP prompt embeddings

negative_prompt_embeds_qwen : Pre-computed Qwen negative prompt embeddings

negative_prompt_embeds_clip : Pre-computed CLIP negative prompt embeddings

prompt_cu_seqlens : Pre-computed cumulative sequence lengths for Qwen positive prompt

negative_prompt_cu_seqlens : Pre-computed cumulative sequence lengths for Qwen negative prompt

callback_on_step_end_tensor_inputs : Callback tensor inputs
#### encode_prompt[[diffusers.Kandinsky5I2IPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky_i2i.py#L295)

Encodes a single prompt (positive or negative) into text encoder hidden states.

This method combines embeddings from both Qwen2.5-VL and CLIP text encoders to create comprehensive text
representations for image generation.

**Parameters:**

prompt (`str` or `list[str]`) : Prompt to be encoded.

num_images_per_prompt (`int`, *optional*, defaults to 1) : Number of images to generate per prompt.

max_sequence_length (`int`, *optional*, defaults to 1024) : Maximum sequence length for text encoding. Must be less than 1024

device (`torch.device`, *optional*) : Torch device.

dtype (`torch.dtype`, *optional*) : Torch dtype.

**Returns:**

`tuple[torch.Tensor, torch.Tensor, torch.Tensor]`

- Qwen text embeddings of shape (batch_size * num_images_per_prompt, sequence_length, embedding_dim)
- CLIP pooled embeddings of shape (batch_size * num_images_per_prompt, clip_embedding_dim)
- Cumulative sequence lengths (`cu_seqlens`) for Qwen embeddings of shape (batch_size *
  num_images_per_prompt + 1,)
#### prepare_latents[[diffusers.Kandinsky5I2IPipeline.prepare_latents]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky_i2i.py#L482)

Prepare initial latent variables for image-to-image generation.

This method creates random noise latents with encoded image,

**Parameters:**

image (PipelineImageInput) : Input image to condition the generation on

batch_size (int) : Number of images to generate

num_channels_latents (int) : Number of channels in latent space

height (int) : Height of generated image

width (int) : Width of generated image

dtype (torch.dtype) : Data type for latents

device (torch.device) : Device to create latents on

generator (torch.Generator) : Random number generator

latents (torch.Tensor) : Pre-existing latents to use

**Returns:**

`torch.Tensor`

Prepared latent tensor with encoded image

## Citation
```bibtex
@misc{kandinsky2025,
    author = {Alexander Belykh and Alexander Varlamov and Alexey Letunovskiy and Anastasia Aliaskina and Anastasia Maltseva and Anastasiia Kargapoltseva and Andrey Shutkin and Anna Averchenkova and Anna Dmitrienko and Bulat Akhmatov and Denis Dimitrov and Denis Koposov and Denis Parkhomenko and Dmitrii and Ilya Vasiliev and Ivan Kirillov and Julia Agafonova and Kirill Chernyshev and Kormilitsyn Semen and Lev Novitskiy and Maria Kovaleva and Mikhail Mamaev and Mikhailov and Nikita Kiselev and Nikita Osterov and Nikolai Gerasimenko and Nikolai Vaulin and Olga Kim and Olga Vdovchenko and Polina Gavrilova and Polina Mikhailova and Tatiana Nikulina and Viacheslav Vasilev and Vladimir Arkhipkin and Vladimir Korviakov and Vladimir Polovnikov and Yury Kolabushin},
    title = {Kandinsky 5.0: A family of diffusion models for Video & Image generation},
    howpublished = {\url{https://github.com/kandinskylab/Kandinsky-5}},
    year = 2025
}
```
