# Motif-Video

[Technical Report](https://arxiv.org/abs/2604.16503)

Motif-Video is a 2B parameter diffusion transformer designed for text-to-video and image-to-video generation. It features a three-stage architecture with 12 dual-stream + 16 single-stream + 8 DDT decoder layers, Shared Cross-Attention for stable text-video alignment under long video sequences, T5Gemma2 text encoder, and rectified flow matching for velocity prediction.

  

## Text-to-Video Generation

Use `MotifVideoPipeline` for text-to-video generation:

```python
import torch
from diffusers import MotifVideoPipeline
from diffusers.utils import export_to_video

pipe = MotifVideoPipeline.from_pretrained(
    "Motif-Technologies/Motif-Video-2B",
    torch_dtype=torch.bfloat16,
)
pipe.to("cuda")

prompt = "A woman with long brown hair and light skin smiles at another woman with long blonde hair."
negative_prompt = "worst quality, inconsistent motion, blurry, jittery, distorted"

video = pipe(
    prompt=prompt,
    negative_prompt=negative_prompt,
    width=1280,
    height=736,
    num_frames=121,
    num_inference_steps=50,
).frames[0]
export_to_video(video, "output.mp4", fps=24)
```

## Image-to-Video Generation

Use `MotifVideoImage2VideoPipeline` for image-to-video generation:

```python
import torch
from diffusers import MotifVideoImage2VideoPipeline
from diffusers.utils import export_to_video, load_image

pipe = MotifVideoImage2VideoPipeline.from_pretrained(
    "Motif-Technologies/Motif-Video-2B",
    torch_dtype=torch.bfloat16,
)
pipe.to("cuda")

image = load_image("input_image.png")
prompt = "A cinematic scene with vivid colors."
negative_prompt = "worst quality, blurry, jittery, distorted"

video = pipe(
    image=image,
    prompt=prompt,
    negative_prompt=negative_prompt,
    width=1280,
    height=736,
    num_frames=121,
    num_inference_steps=50,
).frames[0]
export_to_video(video, "i2v_output.mp4", fps=24)
```

### Memory-efficient Inference

For GPUs with less than 30GB VRAM (e.g., RTX 4090), use model CPU offloading:

```bash
export PYTORCH_CUDA_ALLOC_CONF=expandable_segments:True
```

```python
import torch
from diffusers import MotifVideoPipeline
from diffusers.utils import export_to_video

pipe = MotifVideoPipeline.from_pretrained(
    "Motif-Technologies/Motif-Video-2B",
    torch_dtype=torch.bfloat16,
)
pipe.enable_model_cpu_offload()

prompt = "A woman with long brown hair and light skin smiles at another woman with long blonde hair."
negative_prompt = "worst quality, inconsistent motion, blurry, jittery, distorted"

video = pipe(
    prompt=prompt,
    negative_prompt=negative_prompt,
    width=1280,
    height=736,
    num_frames=121,
    num_inference_steps=50,
).frames[0]
export_to_video(video, "output.mp4", fps=24)
```

## MotifVideoPipeline[[diffusers.MotifVideoPipeline]]

#### diffusers.MotifVideoPipeline[[diffusers.MotifVideoPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/motif_video/pipeline_motif_video.py#L148)

Pipeline for text-to-video generation using Motif-Video.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

__call__diffusers.MotifVideoPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/motif_video/pipeline_motif_video.py#L492[{"name": "prompt", "val": ": typing.Union[str, typing.List[str], NoneType] = None"}, {"name": "negative_prompt", "val": ": typing.Union[str, typing.List[str], NoneType] = None"}, {"name": "height", "val": ": int = 736"}, {"name": "width", "val": ": int = 1280"}, {"name": "num_frames", "val": ": int = 121"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "timesteps", "val": ": typing.Optional[typing.List[int]] = None"}, {"name": "num_videos_per_prompt", "val": ": typing.Optional[int] = 1"}, {"name": "generator", "val": ": typing.Union[torch._C.Generator, typing.List[torch._C.Generator], NoneType] = None"}, {"name": "latents", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "prompt_embeds", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "prompt_attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "negative_prompt_embeds", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "negative_prompt_attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "output_type", "val": ": typing.Optional[str] = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": typing.Optional[typing.Dict[str, typing.Any]] = None"}, {"name": "callback_on_step_end", "val": ": typing.Union[typing.Callable[[int, int, typing.Dict], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": typing.List[str] = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}, {"name": "vae_batch_size", "val": ": int | None = None"}]- **prompt** (`str` or `List[str]`, *optional*) --
  The prompt or prompts to guide the video generation. If not defined, one has to pass `prompt_embeds`.
- **negative_prompt** (`str` or `List[str]`, *optional*) --
  The prompt or prompts not to guide the video generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance.
- **height** (`int`, defaults to `736`) --
  The height in pixels of the generated video.
- **width** (`int`, defaults to `1280`) --
  The width in pixels of the generated video.
- **num_frames** (`int`, defaults to `121`) --
  The number of video frames to generate.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality video at the
  expense of slower inference.
- **timesteps** (`List[int]`, *optional*) --
  Custom timesteps to use for the denoising process.
- **num_videos_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of videos to generate per prompt.
- **generator** (`torch.Generator` or `List[torch.Generator]`, *optional*) --
  PyTorch Generator object(s) for deterministic generation.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents.
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings.
- **prompt_attention_mask** (`torch.Tensor`, *optional*) --
  Pre-generated attention mask for text embeddings.
- **negative_prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated negative text embeddings.
- **negative_prompt_attention_mask** (`torch.FloatTensor`, *optional*) --
  Pre-generated attention mask for negative text embeddings.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generated video. Choose between `"pil"`, `"np"`, or `"latent"`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a [~MotifVideoPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/motif_video#diffusers.MotifVideoPipelineOutput) instead of a plain tuple.
- **attention_kwargs** (`dict`, *optional*) --
  Arguments passed to the attention processor.
- **callback_on_step_end** (`Callable`, *optional*) --
  A function or subclass of `PipelineCallback` or `MultiPipelineCallbacks` called at the end of each
  denoising step.
- **callback_on_step_end_tensor_inputs** (`List`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function.
- **max_sequence_length** (`int`, defaults to `512`) --
  Maximum sequence length for the tokenizer.
- **vae_batch_size** (`int`, *optional*) --
  Batch size for VAE decoding. If provided and latents batch size is larger, VAE decoding will be done in
  chunks.0[~MotifVideoPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/motif_video#diffusers.MotifVideoPipelineOutput) or `tuple`If `return_dict` is `True`, [~MotifVideoPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/motif_video#diffusers.MotifVideoPipelineOutput) is returned, otherwise a `tuple` is returned
where the first element is a list of generated video frames.

The call function to the pipeline for text-to-video generation.

Examples:
```python
>>> import torch
>>> from diffusers import MotifVideoPipeline
>>> from diffusers.utils import export_to_video

>>> # Load the Motif-Video pipeline
>>> motif_video_model_id = "Motif-Technologies/Motif-Video-2B"
>>> pipe = MotifVideoPipeline.from_pretrained(motif_video_model_id, torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> prompt = "A woman with long brown hair and light skin smiles at another woman with long blonde hair. The woman with brown hair wears a black jacket and has a small, barely noticeable mole on her right cheek. The camera angle is a close-up, focused on the woman with brown hair's face. The lighting is warm and natural, likely from the setting sun, casting a soft glow on the scene. The scene appears to be real-life footage"
>>> negative_prompt = "worst quality, inconsistent motion, blurry, jittery, distorted"

>>> video = pipe(
...     prompt=prompt,
...     negative_prompt=negative_prompt,
...     width=1280,
...     height=736,
...     num_frames=121,
...     num_inference_steps=50,
... ).frames[0]
>>> export_to_video(video, "output.mp4", fps=24)
```

**Parameters:**

transformer ([MotifVideoTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/motif_video_transformer_3d#diffusers.MotifVideoTransformer3DModel)) : Conditional Transformer architecture to denoise the encoded video latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `transformer` to denoise the encoded video latents. Should be an instance of a class inheriting from `SchedulerMixin`, such as [DPMSolverMultistepScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/multistep_dpm_solver#diffusers.DPMSolverMultistepScheduler). If not provided, uses the scheduler attached to the pretrained model.

vae ([AutoencoderKLWan](/docs/diffusers/v0.39.0/en/api/models/autoencoder_kl_wan#diffusers.AutoencoderKLWan)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

text_encoder (`T5Gemma2Encoder`) : Primary text encoder for encoding text prompts into embeddings.

tokenizer (`PreTrainedTokenizerBase`) : Tokenizer corresponding to the primary text encoder.

guider ([BaseGuidance](/docs/diffusers/v0.39.0/en/api/modular_diffusers/guiders#diffusers.BaseGuidance)) : The guidance method to use. Should be an instance of a class inheriting from `BaseGuidance`, such as [ClassifierFreeGuidance](/docs/diffusers/v0.39.0/en/api/modular_diffusers/guiders#diffusers.ClassifierFreeGuidance), [AdaptiveProjectedGuidance](/docs/diffusers/v0.39.0/en/api/modular_diffusers/guiders#diffusers.AdaptiveProjectedGuidance), or [SkipLayerGuidance](/docs/diffusers/v0.39.0/en/api/modular_diffusers/guiders#diffusers.SkipLayerGuidance). If not provided, defaults to `ClassifierFreeGuidance`.

**Returns:**

`[~MotifVideoPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/motif_video#diffusers.MotifVideoPipelineOutput) or `tuple``

If `return_dict` is `True`, [~MotifVideoPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/motif_video#diffusers.MotifVideoPipelineOutput) is returned, otherwise a `tuple` is returned
where the first element is a list of generated video frames.
#### encode_prompt[[diffusers.MotifVideoPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/motif_video/pipeline_motif_video.py#L247)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `List[str]`, *optional*) : The prompt or prompts to be encoded.

negative_prompt (`str` or `List[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

num_videos_per_prompt (`int`, *optional*, defaults to 1) : Number of videos to generate per prompt.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

prompt_attention_mask (`torch.Tensor`, *optional*) : Pre-generated attention mask for text embeddings.

negative_prompt_attention_mask (`torch.Tensor`, *optional*) : Pre-generated attention mask for negative text embeddings.

max_sequence_length (`int`, defaults to 512) : Maximum sequence length for the tokenizer.

device (`torch.device`, *optional*) : Device to place tensors on.

dtype (`torch.dtype`, *optional*) : Data type for tensors.

**Returns:**

``tuple[torch.Tensor, torch.Tensor, torch.Tensor, torch.Tensor]``

A tuple containing:
- `prompt_embeds`: The text embeddings for the positive prompt
- `negative_prompt_embeds`: The text embeddings for the negative prompt (None if not using guidance)
- `prompt_attention_mask`: The attention mask for the positive prompt
- `negative_prompt_attention_mask`: The attention mask for the negative prompt (None if not using
  guidance)

## MotifVideoImage2VideoPipeline[[diffusers.MotifVideoImage2VideoPipeline]]

#### diffusers.MotifVideoImage2VideoPipeline[[diffusers.MotifVideoImage2VideoPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/motif_video/pipeline_motif_video_image2video.py#L157)

Pipeline for image-to-video generation using Motif-Video with first frame conditioning.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

__call__diffusers.MotifVideoImage2VideoPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/motif_video/pipeline_motif_video_image2video.py#L620[{"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor]"}, {"name": "prompt", "val": ": typing.Union[str, typing.List[str]]"}, {"name": "negative_prompt", "val": ": typing.Union[str, typing.List[str], NoneType] = None"}, {"name": "height", "val": ": int = 736"}, {"name": "width", "val": ": int = 1280"}, {"name": "num_frames", "val": ": int = 121"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "timesteps", "val": ": typing.Optional[typing.List[int]] = None"}, {"name": "num_videos_per_prompt", "val": ": typing.Optional[int] = 1"}, {"name": "generator", "val": ": typing.Union[torch._C.Generator, typing.List[torch._C.Generator], NoneType] = None"}, {"name": "latents", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "prompt_embeds", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "prompt_attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "negative_prompt_embeds", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "negative_prompt_attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "output_type", "val": ": typing.Optional[str] = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": typing.Optional[typing.Dict[str, typing.Any]] = None"}, {"name": "callback_on_step_end", "val": ": typing.Union[typing.Callable[[int, int, typing.Dict], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": typing.List[str] = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}]- **image** (`PipelineImageInput`) --
  The input image to use as the first frame for video generation.
- **prompt** (`str` or `List[str]`) --
  The prompt or prompts to guide the video generation.
- **negative_prompt** (`str` or `List[str]`, *optional*) --
  The prompt or prompts not to guide the video generation.
- **height** (`int`, defaults to `736`) --
  The height in pixels of the generated video.
- **width** (`int`, defaults to `1280`) --
  The width in pixels of the generated video.
- **num_frames** (`int`, defaults to `121`) --
  The number of video frames to generate.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps.
- **timesteps** (`List[int]`, *optional*) --
  Custom timesteps to use for the denoising process.
- **num_videos_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of videos to generate per prompt.
- **generator** (`torch.Generator` or `List[torch.Generator]`, *optional*) --
  PyTorch Generator object(s) for deterministic generation.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents.
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings.
- **prompt_attention_mask** (`torch.Tensor`, *optional*) --
  Pre-generated attention mask for text embeddings.
- **negative_prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated negative text embeddings.
- **negative_prompt_attention_mask** (`torch.FloatTensor`, *optional*) --
  Pre-generated attention mask for negative text embeddings.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generated video.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a [~MotifVideoPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/motif_video#diffusers.MotifVideoPipelineOutput) instead of a plain tuple.
- **attention_kwargs** (`dict`, *optional*) --
  Arguments passed to the attention processor.
- **callback_on_step_end** (`Callable`, *optional*) --
  A function or subclass of `PipelineCallback` called at the end of each denoising step.
- **callback_on_step_end_tensor_inputs** (`List`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function.
- **max_sequence_length** (`int`, defaults to `512`) --
  Maximum sequence length for the tokenizer.0[~MotifVideoPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/motif_video#diffusers.MotifVideoPipelineOutput) or `tuple`If `return_dict` is `True`, [~MotifVideoPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/motif_video#diffusers.MotifVideoPipelineOutput) is returned, otherwise a `tuple` is returned
where the first element is a list of generated video frames.

The call function to the pipeline for image-to-video generation.

Examples:
```python
>>> import torch
>>> from PIL import Image
>>> from diffusers import MotifVideoImage2VideoPipeline
>>> from diffusers.utils import export_to_video, load_image

>>> # Load the Motif-Video image-to-video pipeline
>>> motif_video_model_id = "Motif-Technologies/Motif-Video-2B"
>>> pipe = MotifVideoImage2VideoPipeline.from_pretrained(motif_video_model_id, torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> # Load an image
>>> image = load_image(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/astronaut.png"
... )

>>> prompt = "An astronaut is walking on the moon surface, kicking up dust with each step"
>>> negative_prompt = "worst quality, inconsistent motion, blurry, jittery, distorted"

>>> video = pipe(
...     image=image,
...     prompt=prompt,
...     negative_prompt=negative_prompt,
...     width=1280,
...     height=736,
...     num_frames=121,
...     num_inference_steps=50,
... ).frames[0]
>>> export_to_video(video, "output.mp4", fps=24)
```

**Parameters:**

transformer ([MotifVideoTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/motif_video_transformer_3d#diffusers.MotifVideoTransformer3DModel)) : Conditional Transformer architecture to denoise the encoded video latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `transformer` to denoise the encoded video latents. Should be an instance of a class inheriting from `SchedulerMixin`, such as [DPMSolverMultistepScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/multistep_dpm_solver#diffusers.DPMSolverMultistepScheduler). If not provided, uses the scheduler attached to the pretrained model.

vae ([AutoencoderKLWan](/docs/diffusers/v0.39.0/en/api/models/autoencoder_kl_wan#diffusers.AutoencoderKLWan)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

text_encoder (`T5Gemma2Encoder`) : Primary text encoder for encoding text prompts into embeddings.

tokenizer (`PreTrainedTokenizerBase`) : Tokenizer corresponding to the primary text encoder.

feature_extractor (`SiglipImageProcessor`) : Image processor for the SigLIP vision encoder.

guider ([BaseGuidance](/docs/diffusers/v0.39.0/en/api/modular_diffusers/guiders#diffusers.BaseGuidance)) : The guidance method to use. Should be an instance of a class inheriting from `BaseGuidance`, such as [ClassifierFreeGuidance](/docs/diffusers/v0.39.0/en/api/modular_diffusers/guiders#diffusers.ClassifierFreeGuidance), [AdaptiveProjectedGuidance](/docs/diffusers/v0.39.0/en/api/modular_diffusers/guiders#diffusers.AdaptiveProjectedGuidance), or [SkipLayerGuidance](/docs/diffusers/v0.39.0/en/api/modular_diffusers/guiders#diffusers.SkipLayerGuidance). If not provided, defaults to `ClassifierFreeGuidance`.

**Returns:**

`[~MotifVideoPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/motif_video#diffusers.MotifVideoPipelineOutput) or `tuple``

If `return_dict` is `True`, [~MotifVideoPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/motif_video#diffusers.MotifVideoPipelineOutput) is returned, otherwise a `tuple` is returned
where the first element is a list of generated video frames.
#### encode_prompt[[diffusers.MotifVideoImage2VideoPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/motif_video/pipeline_motif_video_image2video.py#L259)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `List[str]`, *optional*) : The prompt or prompts to be encoded.

negative_prompt (`str` or `List[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

num_videos_per_prompt (`int`, *optional*, defaults to 1) : Number of videos to generate per prompt.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

prompt_attention_mask (`torch.Tensor`, *optional*) : Pre-generated attention mask for text embeddings.

negative_prompt_attention_mask (`torch.Tensor`, *optional*) : Pre-generated attention mask for negative text embeddings.

max_sequence_length (`int`, defaults to 512) : Maximum sequence length for the tokenizer.

device (`torch.device`, *optional*) : Device to place tensors on.

dtype (`torch.dtype`, *optional*) : Data type for tensors.

**Returns:**

``tuple[torch.Tensor, torch.Tensor, torch.Tensor, torch.Tensor]``

A tuple containing:
- `prompt_embeds`: The text embeddings for the positive prompt
- `negative_prompt_embeds`: The text embeddings for the negative prompt (None if not using guidance)
- `prompt_attention_mask`: The attention mask for the positive prompt
- `negative_prompt_attention_mask`: The attention mask for the negative prompt (None if not using
  guidance)

## MotifVideoPipelineOutput[[diffusers.MotifVideoPipelineOutput]]

#### diffusers.MotifVideoPipelineOutput[[diffusers.MotifVideoPipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/motif_video/pipeline_output.py#L9)

Output class for Motif-Video pipelines.

**Parameters:**

frames (`torch.Tensor`, `np.ndarray`, or List[List[PIL.Image.Image]]) : List of video outputs - It can be a nested list of length `batch_size,` with each sub-list containing denoised PIL image sequences of length `num_frames.` It can also be a NumPy array or Torch tensor of shape `(batch_size, num_frames, channels, height, width)`.
