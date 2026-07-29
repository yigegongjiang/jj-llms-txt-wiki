# Ideogram 4

Ideogram 4 is a flow-matching text-to-image model that uses a multimodal text encoder and an asymmetric
classifier-free guidance scheme: a dedicated `unconditional_transformer` produces the negative branch with zeroed text
features, while the main `transformer` consumes the full packed text + image sequence.

The pipeline defaults are the recommended settings for best quality, so a plain `pipe(prompt)` call produces
best-quality results out of the box: 48 flow-matching steps on a logit-normal schedule (`mu=0.0`, `std=1.5`) with
classifier-free guidance held at 7.0 for the main steps and dropped to 3.0 for the final 3 "polish" steps.

Key inference-time knobs are exposed via the pipeline call:

- `num_inference_steps`, `mu`, and `std` control the resolution-aware logit-normal flow-matching schedule.
- `guidance_scale` (or a full per-step `guidance_schedule`) blends the conditional and unconditional velocities.

## Text-to-image

```python
import torch
from diffusers import Ideogram4Pipeline

pipe = Ideogram4Pipeline.from_pretrained("ideogram-ai/ideogram-v4", torch_dtype=torch.bfloat16)
pipe.to("cuda")

prompt = "A photo of a cat holding a sign that says hello world"
# The defaults are the recommended settings for best quality.
image = pipe(prompt, height=1024, width=1024, generator=torch.Generator("cuda").manual_seed(0)).images[0]
image.save("ideogram4.png")
```

## Prompt upsampling

Ideogram 4 is trained on a structured JSON caption rather than a free-form prompt, so a short prompt is best
expanded into that native schema before generation. There are two ways to produce the caption.

### Remote (Ideogram API)

For the best results, expand the prompt with Ideogram's hosted magic-prompt API and pass the returned caption
straight to the pipeline (get a key at [developer.ideogram.ai](https://developer.ideogram.ai/)):

```python
import json
import requests
import torch
from diffusers import Ideogram4Pipeline

pipe = Ideogram4Pipeline.from_pretrained("ideogram-ai/ideogram-4-nf4", torch_dtype=torch.bfloat16)
pipe.to("cuda")

# Expand the prompt into a structured JSON caption with Ideogram's hosted magic-prompt API.
response = requests.post(
    "https://api.ideogram.ai/v1/ideogram-v4/magic-prompt",
    headers={"Api-Key": "your_ideogram_api_key"},
    json={"text_prompt": "A photo of a cat holding a sign that says hello world", "aspect_ratio": "1x1"},
).json()
caption = json.dumps(response["json_prompt"])

# The caption is already upsampled, so pass it directly (no prompt_upsampling).
image = pipe(caption, height=1024, width=1024, generator=torch.Generator("cuda").manual_seed(0)).images[0]
image.save("ideogram4_upsampled.png")
```

### Local (on-device)

For a fully local pipeline, load a small [Ideogram4PromptEnhancerHead](/docs/diffusers/v0.39.0/en/api/pipelines/ideogram4#diffusers.Ideogram4PromptEnhancerHead) (the Qwen3-VL LM head) as the optional
`prompt_enhancer_head` component and pass `prompt_upsampling=True`. The head is grafted onto the shared
`text_encoder`, so no second text encoder is loaded. Install `outlines` for schema-constrained captions (the nf4
checkpoint also needs `bitsandbytes`):

```python
import torch
from diffusers import Ideogram4Pipeline, Ideogram4PromptEnhancerHead

prompt_enhancer_head = Ideogram4PromptEnhancerHead.from_pretrained(
    "diffusers/qwen3-vl-8b-instruct-lm-head", torch_dtype=torch.bfloat16
)
pipe = Ideogram4Pipeline.from_pretrained(
    "ideogram-ai/ideogram-4-nf4", prompt_enhancer_head=prompt_enhancer_head, torch_dtype=torch.bfloat16
)
pipe.to("cuda")

prompt = "A photo of a cat holding a sign that says hello world"
image = pipe(
    prompt,
    height=1024,
    width=1024,
    prompt_upsampling=True,
    generator=torch.Generator("cuda").manual_seed(0),
).images[0]
image.save("ideogram4_upsampled.png")
```

## Ideogram4Pipeline[[diffusers.Ideogram4Pipeline]]

#### diffusers.Ideogram4Pipeline[[diffusers.Ideogram4Pipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ideogram4/pipeline_ideogram4.py#L141)

Text-to-image pipeline for Ideogram4.

Ideogram4 is a flow-matching model trained with asymmetric classifier-free guidance: a `transformer` consumes
text-conditioned features alongside the image latents, while a separate `unconditional_transformer` denoises with
zeroed text features. The two velocity predictions are linearly blended each step.

__call__diffusers.Ideogram4Pipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ideogram4/pipeline_ideogram4.py#L481[{"name": "prompt", "val": ": str | list[str] | None = None"}, {"name": "height", "val": ": int = 2048"}, {"name": "width", "val": ": int = 2048"}, {"name": "num_inference_steps", "val": ": int = 48"}, {"name": "guidance_scale", "val": ": float | None = None"}, {"name": "guidance_schedule", "val": ": list[float] | torch.Tensor | None = (7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 3.0, 3.0, 3.0)"}, {"name": "mu", "val": ": float = 0.0"}, {"name": "std", "val": ": float = 1.5"}, {"name": "prompt_upsampling", "val": ": bool = False"}, {"name": "prompt_upsampling_temperature", "val": ": float = 1.0"}, {"name": "max_sequence_length", "val": ": int = 2048"}, {"name": "num_images_per_prompt", "val": ": int = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[ForwardRef('Ideogram4Pipeline'), int, int, dict[str, typing.Any]], dict[str, typing.Any]]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}]- **prompt** (`str` or `list[str]`) --
  Prompt(s) to guide image generation.
- **height** (`int`, *optional*, defaults to 2048) --
  Output image height in pixels; must be a multiple of `vae_scale_factor * patch_size`.
- **width** (`int`, *optional*, defaults to 2048) --
  Output image width in pixels; must be a multiple of `vae_scale_factor * patch_size`.
- **num_inference_steps** (`int`, *optional*, defaults to 48) --
  Number of flow-matching steps. The default is the recommended setting for best quality.
- **guidance_scale** (`float`, *optional*) --
  Constant classifier-free guidance scale applied at every step. The conditional and unconditional
  velocity predictions are blended as `v = guidance_scale * v_pos + (1 - guidance_scale) * v_neg`.
  Mutually exclusive with `guidance_schedule` (setting both raises). Defaults to `None`.
- **guidance_schedule** (`list[float]` or `torch.Tensor`, *optional*) --
  Per-step guidance scale schedule; must have length `num_inference_steps`. The first entry corresponds
  to the first step (largest noise level). Mutually exclusive with `guidance_scale`; exactly one must be
  set. Defaults to the recommended schedule (7.0 for the main steps, dropping to 3.0 for the final 3
  "polish" steps). To use a constant scale instead, pass `guidance_scale` and `guidance_schedule=None`.
- **mu** (`float`, *optional*, defaults to 0.0) --
  Base mean of the logit-normal flow-matching schedule. The schedule mean is shifted by half the log of
  the resolution ratio relative to 512x512.
- **std** (`float`, *optional*, defaults to 1.5) --
  Standard deviation of the logit-normal flow-matching schedule.
- **prompt_upsampling** (`bool`, *optional*, defaults to `False`) --
  If `True`, rewrite `prompt` into Ideogram4's native structured JSON caption via
  [upsample_prompt()](/docs/diffusers/v0.39.0/en/api/pipelines/ideogram4#diffusers.Ideogram4Pipeline.upsample_prompt) before encoding. Requires the optional `prompt_enhancer_head`
  component; install `outlines` for schema-constrained captions. `generator` is reused to make the
  upsampling reproducible.
- **prompt_upsampling_temperature** (`float`, *optional*, defaults to 1.0) --
  Sampling temperature for prompt upsampling when `prompt_upsampling=True`.
- **max_sequence_length** (`int`, *optional*, defaults to 2048) --
  Maximum number of text tokens per prompt.
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  Number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  Generator(s) used to make sampling deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noise of shape `(batch_size, num_image_tokens, latent_dim)`.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  One of `"pil"`, `"np"`, `"pt"`, or `"latent"`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether to return an [Ideogram4PipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ideogram4#diffusers.pipelines.ideogram4.Ideogram4PipelineOutput).
- **attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary passed along to the attention processor of each transformer. A `"scale"` entry
  scales the loaded LoRA weights (e.g. `{"scale": 0.7}`) when the PEFT backend is active.
- **callback_on_step_end** (`Callable`, *optional*) --
  Callback invoked at the end of every denoising step.
- **callback_on_step_end_tensor_inputs** (`list[str]`, *optional*) --
  Names of tensors to expose to the callback via `callback_kwargs`.0[Ideogram4PipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ideogram4#diffusers.pipelines.ideogram4.Ideogram4PipelineOutput) or `tuple`.

Run text-to-image generation.

Examples:
```py
>>> import torch
>>> from diffusers import Ideogram4Pipeline

>>> pipe = Ideogram4Pipeline.from_pretrained("ideogram-ai/ideogram-v4", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> prompt = "A photo of a cat holding a sign that says hello world"
>>> # The defaults are the recommended settings for best quality.
>>> image = pipe(prompt, height=2048, width=2048, generator=torch.Generator("cuda").manual_seed(0)).images[0]
>>> image.save("ideogram4.png")
```

**Parameters:**

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : Flow-matching scheduler. The pipeline overrides the default sigma schedule with a resolution-aware logit-normal schedule.

vae (`AutoencoderKLFlux2`) : Variational auto-encoder used to decode latents back into images.

text_encoder (`PreTrainedModel`) : Multimodal text encoder. The pipeline consumes hidden states from a fixed set of intermediate decoder layers (see `QWEN3_VL_ACTIVATION_LAYERS`).

tokenizer (`AutoTokenizer`) : Tokenizer paired with `text_encoder`.

transformer ([Ideogram4Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/ideogram4_transformer2d#diffusers.Ideogram4Transformer2DModel)) : Conditional flow-matching transformer.

unconditional_transformer ([Ideogram4Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/ideogram4_transformer2d#diffusers.Ideogram4Transformer2DModel)) : Unconditional (asymmetric-CFG) flow-matching transformer.

**Returns:**

[Ideogram4PipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ideogram4#diffusers.pipelines.ideogram4.Ideogram4PipelineOutput) or `tuple`.
#### encode_prompt[[diffusers.Ideogram4Pipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ideogram4/pipeline_ideogram4.py#L335)

Prepare the conditioning for the packed text+image sequence (one entry per prompt).

Returns a flat tuple `(prompt_embeds, position_ids, segment_ids, indicator)`. The unconditional branch carries
no text, so the pipeline builds its (zeroed) inputs directly rather than encoding a negative prompt.
#### upsample_prompt[[diffusers.Ideogram4Pipeline.upsample_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ideogram4/pipeline_ideogram4.py#L204)

Rewrite each prompt into Ideogram4's native structured JSON caption.

Requires the optional `prompt_enhancer_head` component, which is grafted onto the shared `text_encoder` body to
make it generative. Generation is schema-constrained when `outlines` is installed, otherwise it runs
unconstrained. Pass `generator` (the same one accepted by `__call__`) to make sampling reproducible.

## Ideogram4PromptEnhancerHead[[diffusers.Ideogram4PromptEnhancerHead]]

#### diffusers.Ideogram4PromptEnhancerHead[[diffusers.Ideogram4PromptEnhancerHead]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ideogram4/prompt_enhancer.py#L42)

LM head that makes the head-less Qwen3-VL `text_encoder` generative for prompt upsampling.

An optional pipeline component (`prompt_enhancer_head`): its weights load via a normal `from_pretrained` (its own
small repo, or bundled in the model repo) rather than an in-pipeline download. At upsample time the pipeline
combines it with the shared `text_encoder` body to form the generative model.

## Ideogram4PipelineOutput[[diffusers.pipelines.ideogram4.Ideogram4PipelineOutput]]

#### diffusers.pipelines.ideogram4.Ideogram4PipelineOutput[[diffusers.pipelines.ideogram4.Ideogram4PipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ideogram4/pipeline_output.py#L24)

Output class for the Ideogram 4 pipeline.

**Parameters:**

images (`list[PIL.Image.Image]` or `np.ndarray`) : List of denoised PIL images of length `batch_size`, or numpy array of shape `(batch_size, height, width, num_channels)`.
