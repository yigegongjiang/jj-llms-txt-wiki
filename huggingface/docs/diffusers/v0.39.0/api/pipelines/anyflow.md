# AnyFlow

[AnyFlow: Any-Step Video Diffusion Model with On-Policy Flow Map Distillation](https://huggingface.co/papers/2605.13724) from NVIDIA, National University of Singapore, and Massachusetts Institute of Technology, by Yuchao Gu, Guian Fang, Yuxin Jiang, Weijia Mao, Song Han, Han Cai, Mike Zheng Shou.

> **TL;DR:** AnyFlow is the first any-step video diffusion framework built on flow maps, which enables a single model (bidirectional or causal) to adapt to arbitrary inference budgets.

*Few-step video generation has been significantly advanced by consistency models. However, their performance often degrades in any-step video diffusion models due to the fixed-point formulation. To address this limitation, we present AnyFlow, the first any-step video diffusion distillation framework built on flow maps. Instead of learning only the mapping z_t → z_0, AnyFlow learns transitions z_t → z_r over arbitrary time intervals, enabling a single model to adapt to different inference budgets. We design an improved forward flow map training recipe that fine-tunes pretrained video diffusion models into flow map models, and introduce Flow Map Backward Simulation to enable on-policy distillation for flow map models. Extensive experiments across both bidirectional and causal architectures, at scales ranging from 1.3B to 14B, on text-to-video and image-to-video tasks demonstrate that AnyFlow outperforms consistency-based baselines while preserving high fidelity and flexible sampling under varying step budgets.*

The AnyFlow pipelines were contributed by the AnyFlow Team. The original code is available on [GitHub](https://github.com/NVlabs/AnyFlow), the project page is at [nvlabs.github.io/AnyFlow](https://nvlabs.github.io/AnyFlow), and pretrained models can be found in the [nvidia/anyflow](https://huggingface.co/collections/nvidia/anyflow) collection on Hugging Face.

Available Models:

| Checkpoint | Backbone | Description |
|---|---|---|
| [`nvidia/AnyFlow-Wan2.1-T2V-1.3B-Diffusers`](https://huggingface.co/nvidia/AnyFlow-Wan2.1-T2V-1.3B-Diffusers) | Wan2.1 1.3B | Bidirectional T2V |
| [`nvidia/AnyFlow-Wan2.1-T2V-14B-Diffusers`](https://huggingface.co/nvidia/AnyFlow-Wan2.1-T2V-14B-Diffusers) | Wan2.1 14B | Bidirectional T2V |
| [`nvidia/AnyFlow-FAR-Wan2.1-1.3B-Diffusers`](https://huggingface.co/nvidia/AnyFlow-FAR-Wan2.1-1.3B-Diffusers) | FAR + Wan2.1 1.3B | Causal T2V / I2V / V2V |
| [`nvidia/AnyFlow-FAR-Wan2.1-14B-Diffusers`](https://huggingface.co/nvidia/AnyFlow-FAR-Wan2.1-14B-Diffusers) | FAR + Wan2.1 14B | Causal T2V / I2V / V2V |

> [!TIP]
> `AnyFlowPipeline` is designed for bidirectional diffusion models in text-to-video (T2V) generation. `AnyFlowFARPipeline` is a chunk-wise causal diffusion model that supports text-to-video (T2V) generation, image-to-video (I2V) generation, and video continuation (V2V).

### Generation with AnyFlow (Bidirectional T2V)

```py
import torch
from diffusers import AnyFlowPipeline
from diffusers.utils import export_to_video

pipe = AnyFlowPipeline.from_pretrained(
    "nvidia/AnyFlow-Wan2.1-T2V-1.3B-Diffusers", torch_dtype=torch.bfloat16
).to("cuda")

prompt = (
    "An astronaut runs smoothly and appears almost weightless on the lunar surface, "
    "as seen from a low-angle shot that highlights the vast, desolate background of the moon. "
    "The moon's craters and rocky terrain are clearly visible, creating a stark contrast against "
    "the running astronaut who moves with graceful, fluid motions."
)
video = pipe(prompt, num_inference_steps=4, num_frames=81).frames[0]
export_to_video(video, "anyflow_t2v.mp4", fps=16)
```

### Generation with AnyFlow (FAR Causal)

The causal pipeline selects between T2V / I2V / V2V via the ``video`` (or ``video_latents``) argument:
omit both for plain text-to-video, or pass ``video=<tensor>`` of shape ``(B, T, C, H, W)`` in ``[0, 1]``
with ``T = 4n + 1`` to condition on existing frames. Use a single conditioning frame for I2V and a longer
clip for V2V continuation. If you already have pre-encoded latents in the model layout, pass them via
``video_latents=<tensor>`` to skip VAE encoding. ``video`` and ``video_latents`` are mutually exclusive.

> [!IMPORTANT]
> The released checkpoints bake `chunk_partition=[1, 3, 3, 3, 3, 3, 3, 2]` (sum 21) into the transformer
> config, matched to the canonical 81 raw frames (21 latent frames at the VAE temporal stride of 4). When
> you change `num_frames`, pass a matching `chunk_partition` summing to `(num_frames - 1) // 4 + 1`,
> otherwise the pipeline raises a `ValueError`.

```py
import torch
from diffusers import AnyFlowFARPipeline
from diffusers.utils import export_to_video

pipe = AnyFlowFARPipeline.from_pretrained(
    "nvidia/AnyFlow-FAR-Wan2.1-1.3B-Diffusers", torch_dtype=torch.bfloat16
).to("cuda")

prompt = (
    "An astronaut runs smoothly and appears almost weightless on the lunar surface, "
    "as seen from a low-angle shot that highlights the vast, desolate background of the moon."
)
video = pipe(prompt, num_inference_steps=4, num_frames=81).frames[0]
export_to_video(video, "anyflow_far_t2v.mp4", fps=16)
```

```py
import numpy as np
import torch
from diffusers import AnyFlowFARPipeline
from diffusers.utils import export_to_video, load_image

pipe = AnyFlowFARPipeline.from_pretrained(
    "nvidia/AnyFlow-FAR-Wan2.1-1.3B-Diffusers", torch_dtype=torch.bfloat16
).to("cuda")

# Example conditioning image from the AnyFlow repo.
first_frame = load_image(
    "https://raw.githubusercontent.com/NVlabs/AnyFlow/main/assets/evaluation/example/images/1.jpg"
).resize((832, 480))
arr = np.asarray(first_frame).astype("float32") / 255.0  # (480, 832, 3)
context_tensor = torch.from_numpy(arr).permute(2, 0, 1).unsqueeze(0).unsqueeze(1).to("cuda")  # (1, 1, 3, 480, 832)

prompt = (
    "A towering, battle-scarred humanoid robot, reminiscent of a Transformer with powerful, segmented armor "
    "and glowing red optics, walking through the skeletal remains of a city ruin. Twisted metal and shattered "
    "concrete crunch under its heavy steps, as the robot scans the desolate, dust-choked skyline under an dark sky."
)
video = pipe(
    prompt=prompt,
    video=context_tensor,
    num_inference_steps=4,
    num_frames=81,
).frames[0]
export_to_video(video, "anyflow_far_i2v.mp4", fps=16)
```

```py
import numpy as np
import torch
from diffusers import AnyFlowFARPipeline
from diffusers.utils import export_to_video, load_video

pipe = AnyFlowFARPipeline.from_pretrained(
    "nvidia/AnyFlow-FAR-Wan2.1-1.3B-Diffusers", torch_dtype=torch.bfloat16
).to("cuda")

# Example conditioning clip from the AnyFlow repo — take the first 9 frames (3 latent frames at VAE temporal stride 4).
context_frames = load_video(
    "https://raw.githubusercontent.com/NVlabs/AnyFlow/main/assets/evaluation/example/videos/2.mp4"
)[:9]
arr = np.stack([np.asarray(f.resize((832, 480))) for f in context_frames]).astype("float32") / 255.0
context_tensor = torch.from_numpy(arr).permute(0, 3, 1, 2).unsqueeze(0).to("cuda")  # (1, 9, 3, 480, 832)

prompt = (
    "A focused trail runner's powerful strides through a dense, sun-dappled forest. "
    "The camera tracks alongside, highlighting muscular exertion, sweat, and determined facial expression."
)
video = pipe(
    prompt=prompt,
    video=context_tensor,
    num_inference_steps=4,
    num_frames=81,
    # Override chunk_partition so the first chunk covers exactly the 3 latent context frames.
    chunk_partition=[3, 3, 3, 3, 3, 3, 3],
).frames[0]
export_to_video(video, "anyflow_far_v2v.mp4", fps=16)
```

## Notes

- Classifier-free guidance is fused into the released checkpoints, so inference does not run a second guided forward pass. Keep the default `guidance_scale=1.0` unless your own checkpoint requires otherwise.
- `FlowMapEulerDiscreteScheduler` is general-purpose. You can attach it to any flow-map-distilled checkpoint via `from_pretrained(..., scheduler=FlowMapEulerDiscreteScheduler.from_config(...))`.
- `AnyFlowPipeline` uses [`AnyFlowTransformer3DModel`](../models/anyflow_transformer3d) (bidirectional). `AnyFlowFARPipeline` uses [`AnyFlowFARTransformer3DModel`](../models/anyflow_far_transformer3d), which adds a compressed-frame patch embedding and the FAR causal block-mask.
- LoRA loading is supported via `WanLoraLoaderMixin`, the same mixin used by the upstream Wan pipelines.
- For training recipes (forward flow-map training and on-policy distillation), refer to the original AnyFlow training framework at [`NVlabs/AnyFlow`](https://github.com/NVlabs/AnyFlow); training is out of scope for diffusers.

## AnyFlowPipeline[[diffusers.AnyFlowPipeline]]

#### diffusers.AnyFlowPipeline[[diffusers.AnyFlowPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/anyflow/pipeline_anyflow.py#L80)

Bidirectional text-to-video generation pipeline for AnyFlow flow-map-distilled checkpoints, introduced in
[AnyFlow](https://huggingface.co/papers/2605.13724).

AnyFlow learns arbitrary-interval transitions \\(z_t \to z_r\\) rather than the fixed \\(z_t \to z_0\\) mapping
of consistency models, so a single distilled checkpoint can be evaluated at 1, 2, 4, 8, 16... NFE without
retraining. This pipeline operates over the full video tensor in one bidirectional pass; for chunk-wise
autoregressive (causal) generation use `AnyFlowFARPipeline`.

Sampling is plain Euler in mean-velocity form (`z_r = z_t - (t - r) * u`) with no re-noising. The released NVIDIA
checkpoints fold classifier-free guidance into the model weights, so the default `guidance_scale=1.0` is the
recommended setting.

This model inherits from [*DiffusionPipeline*]. Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

__call__diffusers.AnyFlowPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/anyflow/pipeline_anyflow.py#L379[{"name": "prompt", "val": ": typing.Union[str, typing.List[str]] = None"}, {"name": "video", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "video_latents", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "negative_prompt", "val": ": typing.Union[str, typing.List[str]] = None"}, {"name": "height", "val": ": int = 480"}, {"name": "width", "val": ": int = 832"}, {"name": "num_frames", "val": ": int = 81"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "sigmas", "val": ": typing.Optional[typing.List[float]] = None"}, {"name": "timesteps", "val": ": typing.Optional[typing.List[float]] = None"}, {"name": "guidance_scale", "val": ": float = 1.0"}, {"name": "num_videos_per_prompt", "val": ": typing.Optional[int] = 1"}, {"name": "generator", "val": ": typing.Union[torch._C.Generator, typing.List[torch._C.Generator], NoneType] = None"}, {"name": "latents", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "prompt_embeds", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "negative_prompt_embeds", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "output_type", "val": ": typing.Optional[str] = 'np'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": typing.Optional[typing.Dict[str, typing.Any]] = None"}, {"name": "callback_on_step_end", "val": ": typing.Union[typing.Callable[[int, int, typing.Dict], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": typing.List[str] = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}, {"name": "use_mean_velocity", "val": ": bool = True"}]- **prompt** (`str` or `List[str]`, *optional*) --
  The prompt or prompts to guide the video generation. If not defined, pass `prompt_embeds` instead.
- **video** (`torch.Tensor`, *optional*) --
  Pre-VAE conditioning frames of shape `(B, T, C, H, W)` in `[0, 1]`. When provided, the pipeline
  VAE-encodes them and keeps the corresponding latent prefix fixed during sampling. Mutually exclusive
  with `video_latents`.
- **video_latents** (`torch.Tensor`, *optional*) --
  Pre-encoded VAE latents in the AnyFlow layout `(B, T_latent, C, H_latent, W_latent)`. Skips VAE
  encoding on the pipeline side. Mutually exclusive with `video`.
- **negative_prompt** (`str` or `List[str]`, *optional*) --
  The prompt or prompts to avoid during video generation. Ignored when not using guidance
  (`guidance_scale 0`~AnyFlowPipelineOutput` or `tuple`If `return_dict` is `True`, `AnyFlowPipelineOutput` is returned, otherwise a `tuple` whose first
element is the generated video.

The call function to the pipeline for generation.

Examples:
```python
>>> import torch
>>> from diffusers import AnyFlowPipeline
>>> from diffusers.utils import export_to_video

>>> pipe = AnyFlowPipeline.from_pretrained(
...     "nvidia/AnyFlow-Wan2.1-T2V-14B-Diffusers", torch_dtype=torch.bfloat16
... ).to("cuda")

>>> prompt = "A red panda eating bamboo in a forest, cinematic lighting"
>>> video = pipe(prompt, num_inference_steps=4, num_frames=33).frames[0]
>>> export_to_video(video, "anyflow_t2v.mp4", fps=16)
```

**Parameters:**

tokenizer ([*AutoTokenizer*]) : Tokenizer from [google/umt5-xxl](https://huggingface.co/google/umt5-xxl).

text_encoder ([*UMT5EncoderModel*]) : [google/umt5-xxl](https://huggingface.co/google/umt5-xxl) text encoder.

transformer ([*AnyFlowTransformer3DModel*]) : Bidirectional flow-map 3D Transformer.

vae ([*AutoencoderKLWan*]) : VAE that encodes/decodes videos to and from latent representations.

scheduler ([*FlowMapEulerDiscreteScheduler*]) : Flow-map sampler. The pipeline drives `scheduler.step(..., timestep, sample, r_timestep)` per inference step.

**Returns:**

``~AnyFlowPipelineOutput` or `tuple``

If `return_dict` is `True`, `AnyFlowPipelineOutput` is returned, otherwise a `tuple` whose first
element is the generated video.
#### encode_prompt[[diffusers.AnyFlowPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/anyflow/pipeline_anyflow.py#L179)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : Whether to use classifier free guidance or not.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : Number of videos that should be generated per prompt. torch device to place the resulting embeddings on

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

device : (`torch.device`, *optional*): torch device

dtype : (`torch.dtype`, *optional*): torch dtype
#### encode_video[[diffusers.AnyFlowPipeline.encode_video]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/anyflow/pipeline_anyflow.py#L359)

Encode a pixel-space video into AnyFlow's latent layout.

Mirrors the single-helper convention of other diffusers pipelines (cf.
`WanImageToVideoPipeline.encode_image`): wraps preprocessing, VAE encoding, and latent normalization into one
call. Output layout is `(B, T_latent, C, H, W)`, which is what the AnyFlow transformer expects for
conditioning frames.

## AnyFlowFARPipeline[[diffusers.AnyFlowFARPipeline]]

#### diffusers.AnyFlowFARPipeline[[diffusers.AnyFlowFARPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/anyflow/pipeline_anyflow_far.py#L92)

Causal (FAR-based) text-to-video / image-to-video / video-to-video pipeline for AnyFlow checkpoints, introduced in
[AnyFlow](https://huggingface.co/papers/2605.13724).

The pipeline drives a chunk-wise autoregressive sampling loop: each chunk is denoised with flow-map steps while
attending only to past chunks via block-sparse causal attention, and intermediate KV cache is reused across chunks.

The task mode (T2V / I2V / V2V) is selected by which conditioning argument is passed to `__call__`:

- both `video=None` and `video_latents=None` — pure text-to-video.
- `video=&amp;lt;tensor of shape (B, T, C, H, W) in [0, 1] with T = 4n + 1>` — pre-VAE conditioning frames; the pipeline
  VAE-encodes them. Pass a single-frame video for I2V or a multi-frame clip for V2V.
- `video_latents=&amp;lt;latent tensor of shape (B, T_latent, C, H_latent, W_latent)>` — already-encoded latents in the
  FAR layout (skips the VAE encode step).

The FAR backbone is the causal Wan2.1 variant introduced by [FAR](https://huggingface.co/papers/2503.19325).
Inference is plain Euler in mean-velocity form per chunk with no re-noising. Joint T2V / I2V / V2V is supported by
a single distilled model.

This model inherits from [*DiffusionPipeline*]. Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

__call__diffusers.AnyFlowFARPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/anyflow/pipeline_anyflow_far.py#L447[{"name": "prompt", "val": ": typing.Union[str, typing.List[str]] = None"}, {"name": "video", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "video_latents", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "negative_prompt", "val": ": typing.Union[str, typing.List[str]] = None"}, {"name": "height", "val": ": int = 480"}, {"name": "width", "val": ": int = 832"}, {"name": "num_frames", "val": ": int = 81"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "sigmas", "val": ": typing.Optional[typing.List[float]] = None"}, {"name": "timesteps", "val": ": typing.Optional[typing.List[float]] = None"}, {"name": "guidance_scale", "val": ": float = 1.0"}, {"name": "num_videos_per_prompt", "val": ": typing.Optional[int] = 1"}, {"name": "generator", "val": ": typing.Union[torch._C.Generator, typing.List[torch._C.Generator], NoneType] = None"}, {"name": "latents", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "prompt_embeds", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "negative_prompt_embeds", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "output_type", "val": ": typing.Optional[str] = 'np'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": typing.Optional[typing.Dict[str, typing.Any]] = None"}, {"name": "callback_on_step_end", "val": ": typing.Union[typing.Callable[[int, int, typing.Dict], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": typing.List[str] = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}, {"name": "use_mean_velocity", "val": ": bool = True"}, {"name": "use_kv_cache", "val": ": bool = True"}, {"name": "chunk_partition", "val": ": typing.Optional[typing.List[int]] = None"}]- **prompt** (`str` or `List[str]`, *optional*) --
  The prompt or prompts to guide the video generation. If not defined, pass `prompt_embeds` instead.
- **video** (`torch.Tensor`, *optional*) --
  Pre-VAE conditioning frames of shape `(B, T, C, H, W)` in `[0, 1]` (`T = 4n + 1`). When provided, the
  pipeline VAE-encodes them and keeps the corresponding latent prefix fixed during sampling. Mutually
  exclusive with `video_latents`.
- **video_latents** (`torch.Tensor`, *optional*) --
  Pre-encoded VAE latents in the FAR layout `(B, T_latent, C, H_latent, W_latent)`. Skips VAE encoding on
  the pipeline side. Mutually exclusive with `video`.
- **negative_prompt** (`str` or `List[str]`, *optional*) --
  The prompt or prompts to avoid during video generation. Ignored when not using guidance
  (`guidance_scale 0`~AnyFlowPipelineOutput` or `tuple`If `return_dict` is `True`, an `AnyFlowPipelineOutput` is returned, otherwise a `tuple` whose first
element is the generated video.

The call function to the pipeline for generation.

Examples:
```python
>>> import numpy as np
>>> import torch
>>> from diffusers import AnyFlowFARPipeline
>>> from diffusers.utils import export_to_video, load_image

>>> pipe = AnyFlowFARPipeline.from_pretrained(
...     "nvidia/AnyFlow-FAR-Wan2.1-1.3B-Diffusers", torch_dtype=torch.bfloat16
... ).to("cuda")

>>> # Single-frame I2V: wrap the conditioning image as a (1, 1, 3, H, W) tensor in [0, 1].
>>> first_frame = load_image("path/to/first_frame.png").resize((832, 480))
>>> arr = np.asarray(first_frame).astype("float32") / 255.0
>>> context = torch.from_numpy(arr).permute(2, 0, 1).unsqueeze(0).unsqueeze(1).to("cuda")

>>> video = pipe(
...     prompt="a cat walks across a sunlit lawn",
...     video=context,
...     num_inference_steps=4,
...     num_frames=81,
... ).frames[0]
>>> export_to_video(video, "anyflow_far.mp4", fps=16)
```

**Parameters:**

tokenizer ([*AutoTokenizer*]) : Tokenizer from [google/umt5-xxl](https://huggingface.co/google/umt5-xxl).

text_encoder ([*UMT5EncoderModel*]) : [google/umt5-xxl](https://huggingface.co/google/umt5-xxl) text encoder.

transformer ([*AnyFlowFARTransformer3DModel*]) : FAR causal flow-map 3D Transformer.

vae ([*AutoencoderKLWan*]) : VAE that encodes/decodes videos to and from latent representations.

scheduler ([*FlowMapEulerDiscreteScheduler*]) : Flow-map sampler.

**Returns:**

``~AnyFlowPipelineOutput` or `tuple``

If `return_dict` is `True`, an `AnyFlowPipelineOutput` is returned, otherwise a `tuple` whose first
element is the generated video.
#### encode_prompt[[diffusers.AnyFlowFARPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/anyflow/pipeline_anyflow_far.py#L196)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : Whether to use classifier free guidance or not.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : Number of videos that should be generated per prompt. torch device to place the resulting embeddings on

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

device : (`torch.device`, *optional*): torch device

dtype : (`torch.dtype`, *optional*): torch dtype
#### encode_video[[diffusers.AnyFlowFARPipeline.encode_video]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/anyflow/pipeline_anyflow_far.py#L379)

Encode a pixel-space video into AnyFlow's latent layout.

Mirrors the single-helper convention of other diffusers pipelines (cf.
`WanImageToVideoPipeline.encode_image`): wraps preprocessing, VAE encoding, and latent normalization into one
call. Output layout is `(B, T_latent, C, H, W)`, which is what the AnyFlow transformer expects for
conditioning frames.

## AnyFlowPipelineOutput[[diffusers.pipelines.anyflow.pipeline_output.AnyFlowPipelineOutput]]

#### diffusers.pipelines.anyflow.pipeline_output.AnyFlowPipelineOutput[[diffusers.pipelines.anyflow.pipeline_output.AnyFlowPipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/anyflow/pipeline_output.py#L23)

Output class for AnyFlow pipelines.

**Parameters:**

frames (`torch.Tensor`, `np.ndarray`, or list[list[PIL.Image.Image]]) : list of video outputs - It can be a nested list of length `batch_size,` with each sub-list containing denoised PIL image sequences of length `num_frames.` It can also be a NumPy array or Torch tensor of shape `(batch_size, num_frames, channels, height, width)`.
