# Krea 2

Krea 2 (K2) is a flow-matching text-to-image model built around a single-stream MMDiT with grouped-query attention. A
Qwen3-VL text encoder provides the conditioning: instead of the last hidden state, hidden states from twelve decoder
layers are tapped per token and fused inside the transformer by a small text-fusion stage. Images are decoded with the
Qwen-Image VAE.

Two checkpoints are released, sharing the same architecture but with different recommended sampler settings:

- **Base (midtrain)** — use the full sampler with classifier-free guidance: `num_inference_steps=28`,
  `guidance_scale=4.5`.
- **TDM (distilled)** — distilled for few-step sampling, run with `num_inference_steps=8` and guidance disabled
  (`guidance_scale=0.0`).

`guidance_scale` follows the Krea 2 convention: the velocity is computed as `cond + guidance_scale * (cond - uncond)`
and guidance is enabled whenever `guidance_scale > 0` (this equals the usual CFG formulation with scale
`1 + guidance_scale`).

## Text-to-image

```python
import torch
from diffusers import Krea2Pipeline

# Load from a local directory produced by the Krea 2 conversion (no hub repo yet).
pipe = Krea2Pipeline.from_pretrained("path/to/krea2-diffusers", torch_dtype=torch.bfloat16)
pipe.to("cuda")

prompt = "a fox in the snow"
image = pipe(
    prompt,
    height=1024,
    width=1024,
    num_inference_steps=28,
    guidance_scale=4.5,
    generator=torch.Generator("cuda").manual_seed(0),
).images[0]
image.save("krea2.png")
```

## Krea2Pipeline[[diffusers.Krea2Pipeline]]

#### diffusers.Krea2Pipeline[[diffusers.Krea2Pipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/krea2/pipeline_krea2.py#L134)

The Krea 2 pipeline for text-to-image generation.

__call__diffusers.Krea2Pipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/krea2/pipeline_krea2.py#L445[{"name": "prompt", "val": ": str | list[str] | None = None"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "height", "val": ": int = 1024"}, {"name": "width", "val": ": int = 1024"}, {"name": "num_inference_steps", "val": ": int = 28"}, {"name": "sigmas", "val": ": list[float] | None = None"}, {"name": "guidance_scale", "val": ": float = 4.5"}, {"name": "num_images_per_prompt", "val": ": int = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds_mask", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds_mask", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int, dict], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "max_sequence_length", "val": ": int = 512"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. Ignored when `guidance_scale  0` (this equals
  the usual CFG formulation with scale `1 + guidance_scale`). Set to `0.0` to disable (e.g. for the TDM
  checkpoint).
- **num_images_per_prompt** (`int`, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or more [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to
  make generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents in packed form `(batch_size, image_seq_len, in_channels)`, sampled from a
  Gaussian distribution, to be used as inputs for image generation.
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings of shape `(batch_size, text_seq_len, num_text_layers, text_hidden_dim)`.
  If not provided, embeddings are generated from `prompt`.
- **prompt_embeds_mask** (`torch.Tensor`, *optional*) --
  Boolean mask for `prompt_embeds`; required when `prompt_embeds` is passed.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings; same layout as `prompt_embeds`.
- **negative_prompt_embeds_mask** (`torch.Tensor`, *optional*) --
  Boolean mask for `negative_prompt_embeds`; required when `negative_prompt_embeds` is passed.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generated image. Choose between `"pil"`, `"np"`, `"pt"` or `"latent"`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a [Krea2PipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/krea2#diffusers.pipelines.krea2.Krea2PipelineOutput) instead of a plain tuple.
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that is called at the end of each denoising step with `callback_on_step_end(self, step,
  timestep, callback_kwargs)`.
- **callback_on_step_end_tensor_inputs** (`list[str]`, *optional*, defaults to `["latents"]`) --
  The list of tensor inputs for the `callback_on_step_end` function. Must be a subset of
  `._callback_tensor_inputs`.
- **attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **max_sequence_length** (`int`, defaults to 512) --
  Fixed text sequence length consumed by the transformer; prompts are padded or truncated to it.0[Krea2PipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/krea2#diffusers.pipelines.krea2.Krea2PipelineOutput) or `tuple`[Krea2PipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/krea2#diffusers.pipelines.krea2.Krea2PipelineOutput) if
`return_dict` is True, otherwise a `tuple`, whose first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import Krea2Pipeline

>>> # Load from a local directory produced by the Krea 2 conversion (no hub repo yet).
>>> pipe = Krea2Pipeline.from_pretrained("path/to/krea2-diffusers", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")
>>> prompt = "a fox in the snow"
>>> # Base (midtrain) checkpoint defaults. For the few-step distilled (TDM) checkpoint use
>>> # `num_inference_steps=8, guidance_scale=0.0` instead.
>>> image = pipe(prompt, num_inference_steps=28, guidance_scale=4.5).images[0]
>>> image.save("krea2.png")
```

**Parameters:**

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : Euler flow-matching scheduler. The Krea 2 sigma schedule is the resolution-aware exponential time shift, so the scheduler config is expected to set `use_dynamic_shifting=True` together with the Krea 2 shift parameters (`base_shift=0.5`, `max_shift=1.15`, `base_image_seq_len=256`, `max_image_seq_len=6400`).

vae ([AutoencoderKLQwenImage](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl_qwenimage#diffusers.AutoencoderKLQwenImage)) : The Qwen-Image variational auto-encoder (f8, 16 latent channels) used to decode latents to images.

text_encoder ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.12.1/en/main_classes/model#transformers.PreTrainedModel)) : A Qwen3-VL model (e.g. `Qwen3VLModel` of `Qwen/Qwen3-VL-4B-Instruct`). The pipeline consumes a stack of hidden states tapped from several decoder layers rather than the last hidden state.

tokenizer ([AutoTokenizer](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/auto#transformers.AutoTokenizer)) : The tokenizer paired with the text encoder.

transformer ([Krea2Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/krea2_transformer2d#diffusers.Krea2Transformer2DModel)) : The Krea 2 single-stream MMDiT that predicts the flow-matching velocity.

text_encoder_select_layers (`tuple[int, ...]`, *optional*) : Indices into the text encoder's `hidden_states` tuple (0 is the embedding output) whose states are stacked per token as the transformer's text conditioning. Must have `transformer.config.num_text_layers` entries.

is_distilled (`bool`, *optional*, defaults to `False`) : Whether the transformer is the few-step distilled (TDM/turbo) checkpoint. When `True` a fixed timestep shift `mu=1.15` is used; otherwise `mu` is computed from the image resolution.

patch_size (`int`, *optional*, defaults to 2) : Side length of the square patches the latents are packed into before entering the transformer. The effective pixel-to-token downsampling factor is `vae_scale_factor * patch_size`.

**Returns:**

`[Krea2PipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/krea2#diffusers.pipelines.krea2.Krea2PipelineOutput) or `tuple``

[Krea2PipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/krea2#diffusers.pipelines.krea2.Krea2PipelineOutput) if
`return_dict` is True, otherwise a `tuple`, whose first element is a list with the generated images.
#### encode_prompt[[diffusers.Krea2Pipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/krea2/pipeline_krea2.py#L263)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings of shape `(batch_size, text_seq_len, num_text_layers, text_hidden_dim)`. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

prompt_embeds_mask (`torch.Tensor`, *optional*) : Pre-generated boolean mask marking valid text tokens, of shape `(batch_size, text_seq_len)`. Required when `prompt_embeds` is passed.

max_sequence_length (`int`, defaults to 512) : Fixed text sequence length consumed by the transformer; prompts are padded or truncated to it.
#### get_text_hidden_states[[diffusers.Krea2Pipeline.get_text_hidden_states]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/krea2/pipeline_krea2.py#L214)

Tokenize `prompt` into the fixed-length Krea 2 layout and tap the selected encoder hidden states.

Returns a `(hidden_states, attention_mask)` tuple of shapes `(batch_size, text_seq_len, num_text_layers,
text_hidden_dim)` and `(batch_size, text_seq_len)` (bool).
#### prepare_position_ids[[diffusers.Krea2Pipeline.prepare_position_ids]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/krea2/pipeline_krea2.py#L381)

Build the `(text_seq_len + grid_height * grid_width, 3)` rotary coordinates for the combined sequence:
text tokens sit at the origin, image tokens carry their `(0, h, w)` latent-grid coordinates.

## Krea2PipelineOutput[[diffusers.pipelines.krea2.Krea2PipelineOutput]]

#### diffusers.pipelines.krea2.Krea2PipelineOutput[[diffusers.pipelines.krea2.Krea2PipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/krea2/pipeline_output.py#L24)

Output class for the Krea 2 pipeline.

**Parameters:**

images (`list[PIL.Image.Image]` or `np.ndarray`) : List of denoised PIL images of length `batch_size` or numpy array of shape `(batch_size, height, width, num_channels)`.
