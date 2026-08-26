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
pipe = Krea2Pipeline.from_pretrained("krea/Krea-2-Raw", dtype=torch.bfloat16)
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

We additionally provide an example for using Krea2 Turbo :

```python
import torch
from diffusers import Krea2Pipeline

pipe = Krea2Pipeline.from_pretrained("krea/Krea-2-Turbo", dtype=torch.bfloat16)
pipe.to("cuda")

image = pipe(
    "a fox in the snow",
    height=1024,
    width=1024,
    num_inference_steps=8,
    guidance_scale=0.0,
    generator=torch.Generator("cuda").manual_seed(0),
).images[0]
image.save("krea2_turbo.png")
```

## Krea2Pipeline[[diffusers.Krea2Pipeline]]

#### diffusers.Krea2Pipeline[[diffusers.Krea2Pipeline]]

```python
diffusers.Krea2Pipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKLQwenImage, text_encoder: Qwen3VLModel, tokenizer: AutoTokenizer, transformer: Krea2Transformer2DModel, text_encoder_select_layers: tuple[int, ...] | list[int] | None = None, is_distilled: bool = False, patch_size: int = 2)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/krea2/pipeline_krea2.py#L134)

**Parameters:**

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : Euler flow-matching scheduler. The Krea 2 sigma schedule is the resolution-aware exponential time shift, so the scheduler config is expected to set `use_dynamic_shifting=True` together with the Krea 2 shift parameters (`base_shift=0.5`, `max_shift=1.15`, `base_image_seq_len=256`, `max_image_seq_len=6400`).

vae ([AutoencoderKLQwenImage](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl_qwenimage#diffusers.AutoencoderKLQwenImage)) : The Qwen-Image variational auto-encoder (f8, 16 latent channels) used to decode latents to images.

text_encoder ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel)) : A Qwen3-VL model (e.g. `Qwen3VLModel` of `Qwen/Qwen3-VL-4B-Instruct`). The pipeline consumes a stack of hidden states tapped from several decoder layers rather than the last hidden state.

tokenizer ([AutoTokenizer](https://huggingface.co/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer)) : The tokenizer paired with the text encoder.

transformer ([Krea2Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/krea2_transformer2d#diffusers.Krea2Transformer2DModel)) : The Krea 2 single-stream MMDiT that predicts the flow-matching velocity.

text_encoder_select_layers (`tuple[int, ...]`, *optional*) : Indices into the text encoder's `hidden_states` tuple (0 is the embedding output) whose states are stacked per token as the transformer's text conditioning. Must have `transformer.config.num_text_layers` entries.

is_distilled (`bool`, *optional*, defaults to `False`) : Whether the transformer is the few-step distilled (TDM/turbo) checkpoint. When `True` a fixed timestep shift `mu=1.15` is used; otherwise `mu` is computed from the image resolution.

patch_size (`int`, *optional*, defaults to 2) : Side length of the square patches the latents are packed into before entering the transformer. The effective pixel-to-token downsampling factor is `vae_scale_factor * patch_size`.

The Krea 2 pipeline for text-to-image generation.

#### __call__[[diffusers.Krea2Pipeline.__call__]]

```python
__call__(prompt: str | list[str] | None = None, negative_prompt: str | list[str] | None = None, height: int = 1024, width: int = 1024, num_inference_steps: int = 28, sigmas: list[float] | None = None, guidance_scale: float = 4.5, num_images_per_prompt: int = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_embeds_mask: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds_mask: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, callback_on_step_end: typing.Optional[typing.Callable[[int, int, dict], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], attention_kwargs: dict[str, typing.Any] | None = None, max_sequence_length: int = 512)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/krea2/pipeline_krea2.py#L445)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. Ignored when `guidance_scale <= 0`; defaults to an empty prompt when guidance is enabled.

height (`int`, defaults to 1024) : The height in pixels of the generated image. Rounded up to a multiple of 16 if needed.

width (`int`, defaults to 1024) : The width in pixels of the generated image. Rounded up to a multiple of 16 if needed.

num_inference_steps (`int`, defaults to 28) : The number of denoising steps. Use 28 for the base (midtrain) checkpoint and 8 for the few-step distilled (TDM) checkpoint.

sigmas (`list[float]`, *optional*) : Custom sigmas for the scheduler. If not defined, the default `linspace(1.0, 1/num_inference_steps, num_inference_steps)` grid is used (the resolution-aware shift is applied inside the scheduler).

guidance_scale (`float`, defaults to 4.5) : Classifier-free guidance scale, following the Krea 2 convention: the velocity is computed as `cond + guidance_scale * (cond - uncond)` and guidance is enabled whenever `guidance_scale > 0` (this equals the usual CFG formulation with scale `1 + guidance_scale`). Set to `0.0` to disable (e.g. for the TDM checkpoint).

num_images_per_prompt (`int`, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or more [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents in packed form `(batch_size, image_seq_len, in_channels)`, sampled from a Gaussian distribution, to be used as inputs for image generation.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings of shape `(batch_size, text_seq_len, num_text_layers, text_hidden_dim)`. If not provided, embeddings are generated from `prompt`.

prompt_embeds_mask (`torch.Tensor`, *optional*) : Boolean mask for `prompt_embeds`; required when `prompt_embeds` is passed.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings; same layout as `prompt_embeds`.

negative_prompt_embeds_mask (`torch.Tensor`, *optional*) : Boolean mask for `negative_prompt_embeds`; required when `negative_prompt_embeds` is passed.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generated image. Choose between `"pil"`, `"np"`, `"pt"` or `"latent"`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [Krea2PipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/krea2#diffusers.pipelines.krea2.Krea2PipelineOutput) instead of a plain tuple.

callback_on_step_end (`Callable`, *optional*) : A function that is called at the end of each denoising step with `callback_on_step_end(self, step, timestep, callback_kwargs)`.

callback_on_step_end_tensor_inputs (`list[str]`, *optional*, defaults to `["latents"]`) : The list of tensor inputs for the `callback_on_step_end` function. Must be a subset of `._callback_tensor_inputs`.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

max_sequence_length (`int`, defaults to 512) : Fixed text sequence length consumed by the transformer; prompts are padded or truncated to it.

**Returns:** [Krea2PipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/krea2#diffusers.pipelines.krea2.Krea2PipelineOutput) or `tuple`

[Krea2PipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/krea2#diffusers.pipelines.krea2.Krea2PipelineOutput) if
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

#### encode_prompt[[diffusers.Krea2Pipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], device: typing.Optional[torch.device] = None, num_images_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_embeds_mask: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 512)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/krea2/pipeline_krea2.py#L263)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings of shape `(batch_size, text_seq_len, num_text_layers, text_hidden_dim)`. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

prompt_embeds_mask (`torch.Tensor`, *optional*) : Pre-generated boolean mask marking valid text tokens, of shape `(batch_size, text_seq_len)`. Required when `prompt_embeds` is passed.

max_sequence_length (`int`, defaults to 512) : Fixed text sequence length consumed by the transformer; prompts are padded or truncated to it.

#### get_text_hidden_states[[diffusers.Krea2Pipeline.get_text_hidden_states]]

```python
get_text_hidden_states(prompt: str | list[str], max_sequence_length: int = 512, device: typing.Optional[torch.device] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/krea2/pipeline_krea2.py#L214)

Tokenize `prompt` into the fixed-length Krea 2 layout and tap the selected encoder hidden states.

Returns a `(hidden_states, attention_mask)` tuple of shapes `(batch_size, text_seq_len, num_text_layers,
text_hidden_dim)` and `(batch_size, text_seq_len)` (bool).

#### prepare_position_ids[[diffusers.Krea2Pipeline.prepare_position_ids]]

```python
prepare_position_ids(text_seq_len: int, grid_height: int, grid_width: int, device: device)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/krea2/pipeline_krea2.py#L381)

Build the `(text_seq_len + grid_height * grid_width, 3)` rotary coordinates for the combined sequence:
text tokens sit at the origin, image tokens carry their `(0, h, w)` latent-grid coordinates.

## Krea2PipelineOutput[[diffusers.pipelines.krea2.Krea2PipelineOutput]]

#### diffusers.pipelines.krea2.Krea2PipelineOutput[[diffusers.pipelines.krea2.Krea2PipelineOutput]]

```python
diffusers.pipelines.krea2.Krea2PipelineOutput(images: list[PIL.Image.Image] | numpy.ndarray)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/krea2/pipeline_output.py#L24)

**Parameters:**

images (`list[PIL.Image.Image]` or `np.ndarray`) : List of denoised PIL images of length `batch_size` or numpy array of shape `(batch_size, height, width, num_channels)`.

Output class for the Krea 2 pipeline.

## Modular

Krea 2 is also available as a [modular pipeline](../../modular_diffusers/overview). Classifier-free guidance is
configured through the `guider` component rather than a `guidance_scale` call argument. Krea 2 uses cond-anchored CFG,
which is [ClassifierFreeGuidance](/docs/diffusers/v0.40.0/en/api/modular_diffusers/guiders#diffusers.ClassifierFreeGuidance) with `use_original_formulation=True`.

```python
import torch
from diffusers import ClassifierFreeGuidance, ModularPipeline

pipe = ModularPipeline.from_pretrained("krea/Krea-2-Raw")
pipe.load_components(dtype=torch.bfloat16)
pipe.to("cuda")

image = pipe(
    prompt="a fox in the snow",
    height=1024,
    width=1024,
    num_inference_steps=28,
    generator=torch.Generator("cuda").manual_seed(0),
).images[0]
image.save("krea2.png")
```

We additionally provide an example for using Krea2 Turbo. The distilled checkpoint maps to its own set of blocks
([Krea2TurboAutoBlocks](/docs/diffusers/v0.40.0/en/api/pipelines/krea2#diffusers.Krea2TurboAutoBlocks)): it runs guidance-free (no `guider`), takes no negative prompt, and samples in a few steps.
`ModularPipeline.from_pretrained` picks the turbo blocks automatically from the checkpoint's `is_distilled` config, so
no guidance configuration is needed:

```python
import torch
from diffusers import ModularPipeline

pipe = ModularPipeline.from_pretrained("krea/Krea-2-Turbo")
pipe.load_components(dtype=torch.bfloat16)
pipe.to("cuda")

image = pipe(
    prompt="a fox in the snow",
    height=1024,
    width=1024,
    num_inference_steps=8,
    generator=torch.Generator("cuda").manual_seed(0),
).images[0]
image.save("krea2_turbo.png")
```

## Krea2ModularPipeline[[diffusers.Krea2ModularPipeline]]

#### diffusers.Krea2ModularPipeline[[diffusers.Krea2ModularPipeline]]

```python
diffusers.Krea2ModularPipeline(blocks: diffusers.modular_pipelines.modular_pipeline.ModularPipelineBlocks | None = None, pretrained_model_name_or_path: str | os.PathLike | None = None, components_manager: diffusers.modular_pipelines.components_manager.ComponentsManager | None = None, collection: str | None = None, workflow: str | None = None, modular_config_dict: dict[str, typing.Any] | None = None, config_dict: dict[str, typing.Any] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/krea2/modular_pipeline.py#L19)

A ModularPipeline for Krea 2.

## Krea2AutoBlocks[[diffusers.Krea2AutoBlocks]]

#### diffusers.Krea2AutoBlocks[[diffusers.Krea2AutoBlocks]]

```python
diffusers.Krea2AutoBlocks()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/krea2/modular_blocks_krea2.py#L105)

Auto Modular pipeline for text-to-image generation using Krea 2: encode text -> core denoise (symmetric CFG) ->
decode.

Supported workflows:
- `text2image`: requires `prompt`

Components:
text_encoder (`Qwen3VLModel`): The Qwen3-VL text encoder. tokenizer (`AutoTokenizer`): The tokenizer paired
with the text encoder. guider (`ClassifierFreeGuidance`) transformer (`Krea2Transformer2DModel`) scheduler
(`FlowMatchEulerDiscreteScheduler`) vae (`AutoencoderKLQwenImage`) image_processor (`VaeImageProcessor`)

Inputs:
prompt (`str`):
The prompt or prompts to guide image generation.
negative_prompt (`str`, *optional*):
The negative prompt(s) for CFG.
max_sequence_length (`int`, *optional*, defaults to 512):
Maximum sequence length for prompt encoding.
num_images_per_prompt (`int`, *optional*, defaults to 1):
The number of images to generate per prompt.
latents (`Tensor`, *optional*):
Pre-generated noisy latents for image generation.
height (`int`, *optional*, defaults to 1024):
The height in pixels of the generated image.
width (`int`, *optional*, defaults to 1024):
The width in pixels of the generated image.
generator (`Generator`, *optional*):
Torch generator for deterministic generation.
num_inference_steps (`int`, *optional*, defaults to 28):
The number of denoising steps.
sigmas (`list`, *optional*):
Custom sigma schedule (defaults to a linear ramp).
attention_kwargs (`dict`, *optional*):
Additional kwargs for attention processors.
output_type (`str`, *optional*, defaults to pil):
Output format: 'pil', 'np', 'pt'.

Outputs:
images (`list`):
Generated images.

## Krea2TurboModularPipeline[[diffusers.Krea2TurboModularPipeline]]

#### diffusers.Krea2TurboModularPipeline[[diffusers.Krea2TurboModularPipeline]]

```python
diffusers.Krea2TurboModularPipeline(blocks: diffusers.modular_pipelines.modular_pipeline.ModularPipelineBlocks | None = None, pretrained_model_name_or_path: str | os.PathLike | None = None, components_manager: diffusers.modular_pipelines.components_manager.ComponentsManager | None = None, collection: str | None = None, workflow: str | None = None, modular_config_dict: dict[str, typing.Any] | None = None, config_dict: dict[str, typing.Any] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/krea2/modular_pipeline.py#L54)

A ModularPipeline for the distilled Krea 2 turbo (TDM) checkpoint. It runs without classifier-free guidance, so it
takes no negative prompt and has no guider.

## Krea2TurboAutoBlocks[[diffusers.Krea2TurboAutoBlocks]]

#### diffusers.Krea2TurboAutoBlocks[[diffusers.Krea2TurboAutoBlocks]]

```python
diffusers.Krea2TurboAutoBlocks()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/krea2/modular_blocks_krea2_turbo.py#L101)

Auto Modular pipeline for text-to-image generation using the distilled Krea 2 turbo checkpoint: encode text -> core
denoise (guidance-free) -> decode.

Supported workflows:
- `text2image`: requires `prompt`

Components:
text_encoder (`Qwen3VLModel`): The Qwen3-VL text encoder. tokenizer (`AutoTokenizer`): The tokenizer paired
with the text encoder. transformer (`Krea2Transformer2DModel`) scheduler (`FlowMatchEulerDiscreteScheduler`)
vae (`AutoencoderKLQwenImage`) image_processor (`VaeImageProcessor`)

Inputs:
prompt (`str`):
The prompt or prompts to guide image generation.
max_sequence_length (`int`, *optional*, defaults to 512):
Maximum sequence length for prompt encoding.
num_images_per_prompt (`int`, *optional*, defaults to 1):
The number of images to generate per prompt.
latents (`Tensor`, *optional*):
Pre-generated noisy latents for image generation.
height (`int`, *optional*, defaults to 1024):
The height in pixels of the generated image.
width (`int`, *optional*, defaults to 1024):
The width in pixels of the generated image.
generator (`Generator`, *optional*):
Torch generator for deterministic generation.
num_inference_steps (`int`, *optional*, defaults to 8):
The number of denoising steps.
sigmas (`list`, *optional*):
Custom sigma schedule (defaults to a linear ramp).
attention_kwargs (`dict`, *optional*):
Additional kwargs for attention processors.
output_type (`str`, *optional*, defaults to pil):
Output format: 'pil', 'np', 'pt'.

Outputs:
images (`list`):
Generated images.
