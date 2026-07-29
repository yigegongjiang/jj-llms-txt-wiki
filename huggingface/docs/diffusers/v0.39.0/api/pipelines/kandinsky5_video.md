# Kandinsky 5.0 Video

[Kandinsky 5.0](https://arxiv.org/abs/2511.14993) is a family of diffusion models for Video & Image generation.

Kandinsky 5.0 Lite line-up of lightweight video generation models (2B parameters) that ranks #1 among open-source models in its class. It outperforms larger models and offers the best understanding of Russian concepts in the open-source ecosystem.

Kandinsky 5.0 Pro line-up of large high quality video generation models (19B parameters). It offers high qualty generation in HD and more generation formats like I2V.

The model introduces several key innovations:
- **Latent diffusion pipeline** with **Flow Matching** for improved training stability
- **Diffusion Transformer (DiT)** as the main generative backbone with cross-attention to text embeddings
- Dual text encoding using **Qwen2.5-VL** and **CLIP** for comprehensive text understanding
- **HunyuanVideo 3D VAE** for efficient video encoding and decoding
- **Sparse attention mechanisms** (NABLA) for efficient long-sequence processing

The original codebase can be found at [kandinskylab/Kandinsky-5](https://github.com/kandinskylab/Kandinsky-5).

> [!TIP]
> Check out the [Kandinsky Lab](https://huggingface.co/kandinskylab) organization on the Hub for the official model checkpoints for text-to-video generation, including pretrained, SFT, no-CFG, and distilled variants.

## Available Models

Kandinsky 5.0 T2V Pro:

| model_id | Description | Use Cases |
|------------|-------------|-----------|
| **kandinskylab/Kandinsky-5.0-T2V-Pro-sft-5s-Diffusers** | 5 second Text-to-Video Pro model | High-quality text-to-video generation |
| **kandinskylab/Kandinsky-5.0-I2V-Pro-sft-5s-Diffusers** | 5 second Image-to-Video Pro model | High-quality image-to-video generation |

Kandinsky 5.0 T2V Lite:
| model_id | Description | Use Cases |
|------------|-------------|-----------|
| **kandinskylab/Kandinsky-5.0-T2V-Lite-sft-5s-Diffusers** | 5 second Supervised Fine-Tuned model | Highest generation quality |
| **kandinskylab/Kandinsky-5.0-T2V-Lite-sft-10s-Diffusers** | 10 second Supervised Fine-Tuned model | Highest generation quality |
| **kandinskylab/Kandinsky-5.0-T2V-Lite-nocfg-5s-Diffusers** | 5 second Classifier-Free Guidance distilled | 2× faster inference |
| **kandinskylab/Kandinsky-5.0-T2V-Lite-nocfg-10s-Diffusers** | 10 second Classifier-Free Guidance distilled | 2× faster inference |
| **kandinskylab/Kandinsky-5.0-T2V-Lite-distilled16steps-5s-Diffusers** | 5 second Diffusion distilled to 16 steps | 6× faster inference, minimal quality loss |
| **kandinskylab/Kandinsky-5.0-T2V-Lite-distilled16steps-10s-Diffusers** | 10 second Diffusion distilled to 16 steps | 6× faster inference, minimal quality loss |
| **kandinskylab/Kandinsky-5.0-T2V-Lite-pretrain-5s-Diffusers** | 5 second Base pretrained model | Research and fine-tuning |
| **kandinskylab/Kandinsky-5.0-T2V-Lite-pretrain-10s-Diffusers** | 10 second Base pretrained model | Research and fine-tuning |

## Usage Examples

### Basic Text-to-Video Generation

#### Pro
**⚠️ Warning!** all Pro models should be infered with pipeline.enable_model_cpu_offload()  
```python
import torch
from diffusers import Kandinsky5T2VPipeline
from diffusers.utils import export_to_video

# Load the pipeline
model_id = "kandinskylab/Kandinsky-5.0-T2V-Pro-sft-5s-Diffusers"
pipe = Kandinsky5T2VPipeline.from_pretrained(model_id, torch_dtype=torch.bfloat16)

pipe = pipe.to("cuda")
pipeline.transformer.set_attention_backend("flex")                            # <--- Set attention bakend to Flex
pipeline.enable_model_cpu_offload()                                           # <--- Enable cpu offloading for single GPU inference
pipeline.transformer.compile(mode="max-autotune-no-cudagraphs", dynamic=True) # <--- Compile with max-autotune-no-cudagraphs

# Generate video
prompt = "A cat and a dog baking a cake together in a kitchen."
negative_prompt = "Static, 2D cartoon, cartoon, 2d animation, paintings, images, worst quality, low quality, ugly, deformed, walking backwards"

output = pipe(
    prompt=prompt,
    negative_prompt=negative_prompt,
    height=768,
    width=1024,
    num_frames=121,  # ~5 seconds at 24fps
    num_inference_steps=50,
    guidance_scale=5.0,
).frames[0]

export_to_video(output, "output.mp4", fps=24, quality=9)
```

#### Lite
```python
import torch
from diffusers import Kandinsky5T2VPipeline
from diffusers.utils import export_to_video

# Load the pipeline
model_id = "kandinskylab/Kandinsky-5.0-T2V-Lite-sft-5s-Diffusers"
pipe = Kandinsky5T2VPipeline.from_pretrained(model_id, torch_dtype=torch.bfloat16)
pipe = pipe.to("cuda")

# Generate video
prompt = "A cat and a dog baking a cake together in a kitchen."
negative_prompt = "Static, 2D cartoon, cartoon, 2d animation, paintings, images, worst quality, low quality, ugly, deformed, walking backwards"

output = pipe(
    prompt=prompt,
    negative_prompt=negative_prompt,
    height=512,
    width=768,
    num_frames=121,  # ~5 seconds at 24fps
    num_inference_steps=50,
    guidance_scale=5.0,
).frames[0]

export_to_video(output, "output.mp4", fps=24, quality=9)
```

### 10 second Models
**⚠️ Warning!** all 10 second models should be used with Flex attention and max-autotune-no-cudagraphs compilation:

```python
pipe = Kandinsky5T2VPipeline.from_pretrained(
    "kandinskylab/Kandinsky-5.0-T2V-Lite-sft-10s-Diffusers", 
    torch_dtype=torch.bfloat16
)
pipe = pipe.to("cuda")

pipe.transformer.set_attention_backend(
    "flex"
)                                       # <--- Set attention bakend to Flex
pipe.transformer.compile(
    mode="max-autotune-no-cudagraphs", 
    dynamic=True
)                                       # <--- Compile with max-autotune-no-cudagraphs

prompt = "A cat and a dog baking a cake together in a kitchen."
negative_prompt = "Static, 2D cartoon, cartoon, 2d animation, paintings, images, worst quality, low quality, ugly, deformed, walking backwards"

output = pipe(
    prompt=prompt,
    negative_prompt=negative_prompt,
    height=512,
    width=768,
    num_frames=241,
    num_inference_steps=50,
    guidance_scale=5.0,
).frames[0]

export_to_video(output, "output.mp4", fps=24, quality=9)
```

### Diffusion Distilled model
**⚠️ Warning!** all nocfg and diffusion distilled models should be infered wothout CFG (```guidance_scale=1.0```):

```python
model_id = "kandinskylab/Kandinsky-5.0-T2V-Lite-distilled16steps-5s-Diffusers"
pipe = Kandinsky5T2VPipeline.from_pretrained(model_id, torch_dtype=torch.bfloat16)
pipe = pipe.to("cuda")

output = pipe(
    prompt="A beautiful sunset over mountains",
    num_inference_steps=16,  # <--- Model is distilled in 16 steps
    guidance_scale=1.0,      # <--- no CFG
).frames[0]

export_to_video(output, "output.mp4", fps=24, quality=9)
```

### Basic Image-to-Video Generation
**⚠️ Warning!** all Pro models should be infered with pipeline.enable_model_cpu_offload()  
```python
import torch
from diffusers import Kandinsky5T2VPipeline
from diffusers.utils import export_to_video

# Load the pipeline
model_id = "kandinskylab/Kandinsky-5.0-I2V-Pro-sft-5s-Diffusers"
pipe = Kandinsky5T2VPipeline.from_pretrained(model_id, torch_dtype=torch.bfloat16)

pipe = pipe.to("cuda")
pipeline.transformer.set_attention_backend("flex")                            # <--- Set attention bakend to Flex
pipeline.enable_model_cpu_offload()                                           # <--- Enable cpu offloading for single GPU inference
pipeline.transformer.compile(mode="max-autotune-no-cudagraphs", dynamic=True) # <--- Compile with max-autotune-no-cudagraphs

# Generate video
image = load_image(
    "https://huggingface.co/kandinsky-community/kandinsky-3/resolve/main/assets/title.jpg?download=true"
)
height = 896
width = 896
image = image.resize((width, height))

prompt = "An funny furry creture smiles happily and holds a sign that says 'Kandinsky'"
negative_prompt = ""

output = pipe(
    prompt=prompt,
    negative_prompt=negative_prompt,
    height=height,
    width=width,
    num_frames=121,  # ~5 seconds at 24fps
    num_inference_steps=50,
    guidance_scale=5.0,
).frames[0]

export_to_video(output, "output.mp4", fps=24, quality=9)
```

## Kandinsky 5.0 Pro Side-by-Side evaluation

  
      
          
      
      
         
      

  
      
          Comparison with Veo 3 
      
      
          Comparison with Veo 3 fast
      
  
      
          
      
      
          
      
  
      
          Comparison with Wan 2.2 A14B Text-to-Video mode
      
      
          Comparison with Wan 2.2 A14B Image-to-Video mode
      

## Kandinsky 5.0 Lite Side-by-Side evaluation

The evaluation is based on the expanded prompts from the [Movie Gen benchmark](https://github.com/facebookresearch/MovieGenBench), which are available in the expanded_prompt column of the benchmark/moviegen_bench.csv file.

  
      
          
      
      
          
      
  
      
          
      
      
          
      
  
      
          
      

## Kandinsky 5.0 Lite Distill Side-by-Side evaluation

  
      
          
      
      
          
      

## Kandinsky5T2VPipeline[[diffusers.Kandinsky5T2VPipeline]]

#### diffusers.Kandinsky5T2VPipeline[[diffusers.Kandinsky5T2VPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky.py#L131)

Pipeline for text-to-video generation using Kandinsky 5.0.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

__call__diffusers.Kandinsky5T2VPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky.py#L682[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "height", "val": ": int = 512"}, {"name": "width", "val": ": int = 768"}, {"name": "num_frames", "val": ": int = 121"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "guidance_scale", "val": ": float = 5.0"}, {"name": "num_videos_per_prompt", "val": ": int | None = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds_qwen", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds_clip", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds_qwen", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds_clip", "val": ": torch.Tensor | None = None"}, {"name": "prompt_cu_seqlens", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_cu_seqlens", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "callback_on_step_end", "val": ": typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the video generation. If not defined, pass `prompt_embeds` instead.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to avoid during video generation. If not defined, pass `negative_prompt_embeds`
  instead. Ignored when not using guidance (`guidance_scale` 0`~KandinskyPipelineOutput` or `tuple`If `return_dict` is `True`, `KandinskyPipelineOutput` is returned, otherwise a `tuple` is returned
where the first element is a list with the generated images.

The call function to the pipeline for generation.

Examples:

```python
>>> import torch
>>> from diffusers import Kandinsky5T2VPipeline
>>> from diffusers.utils import export_to_video

>>> # Available models:
>>> # kandinskylab/Kandinsky-5.0-T2V-Pro-sft-5s-Diffusers
>>> # kandinskylab/Kandinsky-5.0-T2V-Lite-sft-5s-Diffusers
>>> # kandinskylab/Kandinsky-5.0-T2V-Lite-nocfg-5s-Diffusers
>>> # kandinskylab/Kandinsky-5.0-T2V-Lite-distilled16steps-5s-Diffusers
>>> # kandinskylab/Kandinsky-5.0-T2V-Lite-pretrain-5s-Diffusers
>>> # kandinskylab/Kandinsky-5.0-T2V-Lite-sft-10s-Diffusers
>>> # kandinskylab/Kandinsky-5.0-T2V-Lite-nocfg-10s-Diffusers
>>> # kandinskylab/Kandinsky-5.0-T2V-Lite-distilled16steps-10s-Diffusers
>>> # kandinskylab/Kandinsky-5.0-T2V-Lite-pretrain-10s-Diffusers

>>> model_id = "kandinskylab/Kandinsky-5.0-T2V-Lite-sft-5s-Diffusers"
>>> pipe = Kandinsky5T2VPipeline.from_pretrained(model_id, torch_dtype=torch.bfloat16)
>>> pipe = pipe.to("cuda")

>>> prompt = "A cat and a dog baking a cake together in a kitchen."
>>> negative_prompt = "Static, 2D cartoon, cartoon, 2d animation, paintings, images, worst quality, low quality, ugly, deformed, walking backwards"

>>> output = pipe(
...     prompt=prompt,
...     negative_prompt=negative_prompt,
...     height=512,
...     width=768,
...     num_frames=121,
...     num_inference_steps=50,
...     guidance_scale=5.0,
... ).frames[0]

>>> export_to_video(output, "output.mp4", fps=24, quality=9)
```

**Parameters:**

transformer (`Kandinsky5Transformer3DModel`) : Conditional Transformer to denoise the encoded video latents.

vae ([AutoencoderKLHunyuanVideo](/docs/diffusers/v0.39.0/en/api/models/autoencoder_kl_hunyuan_video#diffusers.AutoencoderKLHunyuanVideo)) : Variational Auto-Encoder Model [hunyuanvideo-community/HunyuanVideo (vae)](https://huggingface.co/hunyuanvideo-community/HunyuanVideo) to encode and decode videos to and from latent representations.

text_encoder (`Qwen2_5_VLForConditionalGeneration`) : Frozen text-encoder [Qwen2.5-VL](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct).

tokenizer (`AutoProcessor`) : Tokenizer for Qwen2.5-VL.

text_encoder_2 (`CLIPTextModel`) : Frozen [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

tokenizer_2 (`CLIPTokenizer`) : Tokenizer for CLIP.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded video latents.

**Returns:**

``~KandinskyPipelineOutput` or `tuple``

If `return_dict` is `True`, `KandinskyPipelineOutput` is returned, otherwise a `tuple` is returned
where the first element is a list with the generated images.
#### check_inputs[[diffusers.Kandinsky5T2VPipeline.check_inputs]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky.py#L513)

Validate input parameters for the pipeline.

**Parameters:**

prompt : Input prompt

negative_prompt : Negative prompt for guidance

height : Video height

width : Video width

prompt_embeds_qwen : Pre-computed Qwen prompt embeddings

prompt_embeds_clip : Pre-computed CLIP prompt embeddings

negative_prompt_embeds_qwen : Pre-computed Qwen negative prompt embeddings

negative_prompt_embeds_clip : Pre-computed CLIP negative prompt embeddings

prompt_cu_seqlens : Pre-computed cumulative sequence lengths for Qwen positive prompt

negative_prompt_cu_seqlens : Pre-computed cumulative sequence lengths for Qwen negative prompt

callback_on_step_end_tensor_inputs : Callback tensor inputs
#### encode_prompt[[diffusers.Kandinsky5T2VPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky.py#L420)

Encodes a single prompt (positive or negative) into text encoder hidden states.

This method combines embeddings from both Qwen2.5-VL and CLIP text encoders to create comprehensive text
representations for video generation.

**Parameters:**

prompt (`str` or `list[str]`) : Prompt to be encoded.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : Number of videos to generate per prompt.

max_sequence_length (`int`, *optional*, defaults to 512) : Maximum sequence length for text encoding.

device (`torch.device`, *optional*) : Torch device.

dtype (`torch.dtype`, *optional*) : Torch dtype.

**Returns:**

`tuple[torch.Tensor, torch.Tensor, torch.Tensor]`

- Qwen text embeddings of shape (batch_size * num_videos_per_prompt, sequence_length, embedding_dim)
- CLIP pooled embeddings of shape (batch_size * num_videos_per_prompt, clip_embedding_dim)
- Cumulative sequence lengths (`cu_seqlens`) for Qwen embeddings of shape (batch_size *
  num_videos_per_prompt + 1,)
#### fast_sta_nabla[[diffusers.Kandinsky5T2VPipeline.fast_sta_nabla]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky.py#L229)

Create a sparse temporal attention (STA) mask for efficient video generation.

This method generates a mask that limits attention to nearby frames and spatial positions, reducing
computational complexity for video generation.

**Parameters:**

T (int) : Number of temporal frames

H (int) : Height in latent space

W (int) : Width in latent space

wT (int) : Temporal attention window size

wH (int) : Height attention window size

wW (int) : Width attention window size

device (str) : Device to create tensor on

**Returns:**

`torch.Tensor`

Sparse attention mask of shape (T*H*W, T*H*W)
#### get_sparse_params[[diffusers.Kandinsky5T2VPipeline.get_sparse_params]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky.py#L264)

Generate sparse attention parameters for the transformer based on sample dimensions.

This method computes the sparse attention configuration needed for efficient video processing in the
transformer model.

**Parameters:**

sample (torch.Tensor) : Input sample tensor

device (torch.device) : Device to place tensors on

**Returns:**

`Dict`

Dictionary containing sparse attention parameters
#### prepare_latents[[diffusers.Kandinsky5T2VPipeline.prepare_latents]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky.py#L599)

Prepare initial latent variables for video generation.

This method creates random noise latents or uses provided latents as starting point for the denoising process.

**Parameters:**

batch_size (int) : Number of videos to generate

num_channels_latents (int) : Number of channels in latent space

height (int) : Height of generated video

width (int) : Width of generated video

num_frames (int) : Number of frames in video

dtype (torch.dtype) : Data type for latents

device (torch.device) : Device to create latents on

generator (torch.Generator) : Random number generator

latents (torch.Tensor) : Pre-existing latents to use

**Returns:**

`torch.Tensor`

Prepared latent tensor

## Kandinsky5I2VPipeline[[diffusers.Kandinsky5I2VPipeline]]

#### diffusers.Kandinsky5I2VPipeline[[diffusers.Kandinsky5I2VPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky_i2v.py#L128)

Pipeline for image-to-video generation using Kandinsky 5.0.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

__call__diffusers.Kandinsky5I2VPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky_i2v.py#L749[{"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor]"}, {"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "height", "val": ": int = 512"}, {"name": "width", "val": ": int = 768"}, {"name": "num_frames", "val": ": int = 121"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "guidance_scale", "val": ": float = 5.0"}, {"name": "num_videos_per_prompt", "val": ": int | None = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds_qwen", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds_clip", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds_qwen", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds_clip", "val": ": torch.Tensor | None = None"}, {"name": "prompt_cu_seqlens", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_cu_seqlens", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int, NoneType], diffusers.callbacks.PipelineCallback | diffusers.callbacks.MultiPipelineCallbacks]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}]- **image** (`PipelineImageInput`) --
  The input image to condition the generation on. Must be an image, a list of images or a `torch.Tensor`.
- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the video generation. If not defined, pass `prompt_embeds` instead.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to avoid during video generation. If not defined, pass `negative_prompt_embeds`
  instead. Ignored when not using guidance (`guidance_scale` 0`~KandinskyPipelineOutput` or `tuple`If `return_dict` is `True`, `KandinskyPipelineOutput` is returned, otherwise a `tuple` is returned
where the first element is a list with the generated videos.

The call function to the pipeline for image-to-video generation.

Examples:

```python
>>> import torch
>>> from diffusers import Kandinsky5I2VPipeline
>>> from diffusers.utils import export_to_video, load_image

>>> # Available models:
>>> # kandinskylab/Kandinsky-5.0-I2V-Pro-sft-5s-Diffusers

>>> model_id = "kandinskylab/Kandinsky-5.0-I2V-Pro-sft-5s-Diffusers"
>>> pipe = Kandinsky5I2VPipeline.from_pretrained(model_id, torch_dtype=torch.bfloat16)
>>> pipe = pipe.to("cuda")

>>> image = load_image(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/astronaut.jpg"
... )
>>> prompt = "An astronaut floating in space with Earth in the background, cinematic shot"
>>> negative_prompt = "Static, 2D cartoon, cartoon, 2d animation, paintings, images, worst quality, low quality, ugly, deformed, walking backwards"

>>> output = pipe(
...     image=image,
...     prompt=prompt,
...     negative_prompt=negative_prompt,
...     height=512,
...     width=768,
...     num_frames=121,
...     num_inference_steps=50,
...     guidance_scale=5.0,
... ).frames[0]

>>> export_to_video(output, "output.mp4", fps=24, quality=9)
```

**Parameters:**

transformer (`Kandinsky5Transformer3DModel`) : Conditional Transformer to denoise the encoded video latents.

vae ([AutoencoderKLHunyuanVideo](/docs/diffusers/v0.39.0/en/api/models/autoencoder_kl_hunyuan_video#diffusers.AutoencoderKLHunyuanVideo)) : Variational Auto-Encoder Model [hunyuanvideo-community/HunyuanVideo (vae)](https://huggingface.co/hunyuanvideo-community/HunyuanVideo) to encode and decode videos to and from latent representations.

text_encoder (`Qwen2_5_VLForConditionalGeneration`) : Frozen text-encoder [Qwen2.5-VL](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct).

tokenizer (`AutoProcessor`) : Tokenizer for Qwen2.5-VL.

text_encoder_2 (`CLIPTextModel`) : Frozen [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

tokenizer_2 (`CLIPTokenizer`) : Tokenizer for CLIP.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded video latents.

**Returns:**

``~KandinskyPipelineOutput` or `tuple``

If `return_dict` is `True`, `KandinskyPipelineOutput` is returned, otherwise a `tuple` is returned
where the first element is a list with the generated videos.
#### check_inputs[[diffusers.Kandinsky5I2VPipeline.check_inputs]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky_i2v.py#L545)

Validate input parameters for the pipeline.

**Parameters:**

prompt : Input prompt

negative_prompt : Negative prompt for guidance

image : Input image for conditioning

height : Video height

width : Video width

prompt_embeds_qwen : Pre-computed Qwen prompt embeddings

prompt_embeds_clip : Pre-computed CLIP prompt embeddings

negative_prompt_embeds_qwen : Pre-computed Qwen negative prompt embeddings

negative_prompt_embeds_clip : Pre-computed CLIP negative prompt embeddings

prompt_cu_seqlens : Pre-computed cumulative sequence lengths for Qwen positive prompt

negative_prompt_cu_seqlens : Pre-computed cumulative sequence lengths for Qwen negative prompt

callback_on_step_end_tensor_inputs : Callback tensor inputs
#### encode_prompt[[diffusers.Kandinsky5I2VPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky_i2v.py#L454)

Encodes a single prompt (positive or negative) into text encoder hidden states.

This method combines embeddings from both Qwen2.5-VL and CLIP text encoders to create comprehensive text
representations for video generation.

**Parameters:**

prompt (`str` or `list[str]`) : Prompt to be encoded.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : Number of videos to generate per prompt.

max_sequence_length (`int`, *optional*, defaults to 512) : Maximum sequence length for text encoding.

device (`torch.device`, *optional*) : Torch device.

dtype (`torch.dtype`, *optional*) : Torch dtype.

**Returns:**

`tuple[torch.Tensor, torch.Tensor, torch.Tensor]`

- Qwen text embeddings of shape (batch_size * num_videos_per_prompt, sequence_length, embedding_dim)
- CLIP pooled embeddings of shape (batch_size * num_videos_per_prompt, clip_embedding_dim)
- Cumulative sequence lengths (`cu_seqlens`) for Qwen embeddings of shape (batch_size *
  num_videos_per_prompt + 1,)
#### fast_sta_nabla[[diffusers.Kandinsky5I2VPipeline.fast_sta_nabla]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky_i2v.py#L226)

Create a sparse temporal attention (STA) mask for efficient video generation.

This method generates a mask that limits attention to nearby frames and spatial positions, reducing
computational complexity for video generation.

**Parameters:**

T (int) : Number of temporal frames

H (int) : Height in latent space

W (int) : Width in latent space

wT (int) : Temporal attention window size

wH (int) : Height attention window size

wW (int) : Width attention window size

device (str) : Device to create tensor on

**Returns:**

`torch.Tensor`

Sparse attention mask of shape (T*H*W, T*H*W)
#### get_sparse_params[[diffusers.Kandinsky5I2VPipeline.get_sparse_params]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky_i2v.py#L261)

Generate sparse attention parameters for the transformer based on sample dimensions.

This method computes the sparse attention configuration needed for efficient video processing in the
transformer model.

**Parameters:**

sample (torch.Tensor) : Input sample tensor

device (torch.device) : Device to place tensors on

**Returns:**

`Dict`

Dictionary containing sparse attention parameters
#### prepare_latents[[diffusers.Kandinsky5I2VPipeline.prepare_latents]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/kandinsky5/pipeline_kandinsky_i2v.py#L636)

Prepare initial latent variables for image-to-video generation.

This method creates random noise latents for all frames except the first frame, which is replaced with the
encoded input image.

**Parameters:**

image (PipelineImageInput) : Input image to condition the generation on

batch_size (int) : Number of videos to generate

num_channels_latents (int) : Number of channels in latent space

height (int) : Height of generated video

width (int) : Width of generated video

num_frames (int) : Number of frames in video

dtype (torch.dtype) : Data type for latents

device (torch.device) : Device to create latents on

generator (torch.Generator) : Random number generator

latents (torch.Tensor) : Pre-existing latents to use

**Returns:**

`torch.Tensor`

Prepared latent tensor with first frame as encoded image

## Citation
```bibtex
@misc{kandinsky2025,
    author = {Alexander Belykh and Alexander Varlamov and Alexey Letunovskiy and Anastasia Aliaskina and Anastasia Maltseva and Anastasiia Kargapoltseva and Andrey Shutkin and Anna Averchenkova and Anna Dmitrienko and Bulat Akhmatov and Denis Dimitrov and Denis Koposov and Denis Parkhomenko and Dmitrii and Ilya Vasiliev and Ivan Kirillov and Julia Agafonova and Kirill Chernyshev and Kormilitsyn Semen and Lev Novitskiy and Maria Kovaleva and Mikhail Mamaev and Mikhailov and Nikita Kiselev and Nikita Osterov and Nikolai Gerasimenko and Nikolai Vaulin and Olga Kim and Olga Vdovchenko and Polina Gavrilova and Polina Mikhailova and Tatiana Nikulina and Viacheslav Vasilev and Vladimir Arkhipkin and Vladimir Korviakov and Vladimir Polovnikov and Yury Kolabushin},
    title = {Kandinsky 5.0: A family of diffusion models for Video & Image generation},
    howpublished = {\url{https://github.com/kandinskylab/Kandinsky-5}},
    year = 2025
}
```
