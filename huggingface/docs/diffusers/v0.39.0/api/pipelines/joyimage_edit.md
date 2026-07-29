# JoyAI-Image-Edit

[JoyAI-Image](https://github.com/jd-opensource/JoyAI-Image) is a unified multimodal foundation model for image understanding, text-to-image generation, and instruction-guided image editing. It combines an 8B Multimodal Large Language Model (MLLM) with a 16B Multimodal Diffusion Transformer (MMDiT). A central principle of JoyAI-Image is the closed-loop collaboration between understanding, generation, and editing.

JoyAI-Image-Edit supports general image editing as well as spatial editing capabilities including object move, object rotation, and camera control.

| Model | Description | Download |
|:-----:|:-----------:|:--------:|
| JoyAI-Image-Edit | Instruction-guided image editing with precise and controllable spatial manipulation | [Hugging Face](https://huggingface.co/jdopensource/JoyAI-Image-Edit-Diffusers) |

```python
import torch
from diffusers import JoyImageEditPipeline
from diffusers.utils import load_image

pipeline = JoyImageEditPipeline.from_pretrained(
    "jdopensource/JoyAI-Image-Edit-Diffusers", torch_dtype=torch.bfloat16
)
pipeline.to("cuda")

image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/astronaut.jpg")
prompt = "Add wings to the astronaut."

output = pipeline(
    image=image,
    prompt=prompt,
    num_inference_steps=40,
    guidance_scale=4.0,
    generator=torch.Generator("cuda").manual_seed(0),
).images[0]
output.save("joyimage_edit_output.png")
```

## Spatial editing

JoyAI-Image supports three spatial editing prompt patterns: **Object Move**, **Object Rotation**, and **Camera Control**. For best results, follow the prompt templates below as closely as possible. For more information, refer to [SpatialEdit](https://github.com/EasonXiao-888/SpatialEdit).

### Object Move

Move a target object into a specified region marked by a red box in the input image.

```text
Move the <object> into the red box and finally remove the red box.
```

### Object Rotation

Rotate an object to a specific canonical view. Supported `<view>` values: `front`, `right`, `left`, `rear`, `front right`, `front left`, `rear right`, `rear left`.

```text
Rotate the <object> to show the <view> side view.
```

### Camera Control

Change the camera viewpoint while keeping the 3D scene unchanged.

```text
Move the camera.
- Camera rotation: Yaw {y_rotation}°, Pitch {p_rotation}°.
- Camera zoom: in/out/unchanged.
- Keep the 3D scene static; only change the viewpoint.
```

## JoyImageEditPipeline[[diffusers.JoyImageEditPipeline]]

#### diffusers.JoyImageEditPipeline[[diffusers.JoyImageEditPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/joyimage/pipeline_joyimage_edit.py#L100)

Diffusion pipeline for image editing using the JoyImage architecture.

The pipeline encodes text and image conditioning via a Qwen3-VL text encoder, denoises latents with a 3-D
transformer, and decodes the result with a WAN VAE.

Model offloading order: text_encoder -> transformer -> vae.

__call__diffusers.JoyImageEditPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/joyimage/pipeline_joyimage_edit.py#L600[{"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "prompt", "val": ": str | list[str] = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 40"}, {"name": "timesteps", "val": ": typing.List[int] = None"}, {"name": "sigmas", "val": ": typing.List[float] = None"}, {"name": "guidance_scale", "val": ": float = 4.0"}, {"name": "negative_prompt", "val": ": typing.Union[str, typing.List[str], NoneType] = None"}, {"name": "num_images_per_prompt", "val": ": typing.Optional[int] = 1"}, {"name": "generator", "val": ": typing.Union[torch._C.Generator, typing.List[torch._C.Generator], NoneType] = None"}, {"name": "latents", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "prompt_embeds", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "prompt_embeds_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "negative_prompt_embeds", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "negative_prompt_embeds_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "output_type", "val": ": typing.Optional[str] = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "callback_on_step_end", "val": ": typing.Union[typing.Callable[[int, int, typing.Dict], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": typing.List[str] = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 4096"}, {"name": "enable_denormalization", "val": ": bool = True"}]- **prompt** (*str* or *List[str]*) --
  The prompt or prompts to guide generation.
- **height** (*int*) --
  Height of the generated output in pixels.
- **width** (*int*) --
  Width of the generated output in pixels.
- **image** (*PipelineImageInput*, *optional*) --
  Reference image used for conditioning. When provided the pipeline operates in image-editing mode with
  `num_items=2`.
- **num_inference_steps** (*int*, *optional*, defaults to 40) --
  Number of denoising steps. More steps generally improve quality at the cost of slower inference.
- **timesteps** (*List[int]*, *optional*) --
  Custom timesteps for the denoising process. When provided, `num_inference_steps` is inferred from the
  list length.
- **sigmas** (*List[float]*, *optional*) --
  Custom sigmas for the denoising process. Mutually exclusive with `timesteps`.
- **guidance_scale** (*float*, *optional*, defaults to 4.0) --
  Classifier-free guidance scale.
- **negative_prompt** (*str* or *List[str]*, *optional*) --
  Negative prompt(s) used to suppress undesired content.
- **num_images_per_prompt** (*int*, *optional*, defaults to 1) --
  Number of generated samples per prompt.
- **generator** (*torch.Generator* or *List[torch.Generator]*, *optional*) --
  RNG generator(s) for deterministic sampling.
- **latents** (*torch.Tensor*, *optional*) --
  Pre-generated noisy latents for the target slot. Sampled from a Gaussian distribution when not
  provided. Can be used to seed generation from a specific starting noise tensor.
- **prompt_embeds** (*torch.Tensor*, *optional*) --
  Pre-computed prompt embeddings. When provided `prompt` can be omitted.
- **prompt_embeds_mask** (*torch.Tensor*, *optional*) --
  Attention mask for `prompt_embeds`.
- **negative_prompt_embeds** (*torch.Tensor*, *optional*) --
  Pre-computed negative prompt embeddings.
- **negative_prompt_embeds_mask** (*torch.Tensor*, *optional*) --
  Attention mask for `negative_prompt_embeds`.
- **output_type** (*str*, *optional*, defaults to `"pil"`) --
  Output format. Pass `"latent"` to return raw latents.
- **return_dict** (*bool*, *optional*, defaults to *True*) --
  Whether to return a [JoyImageEditPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/joyimage_edit#diffusers.JoyImageEditPipelineOutput) or a plain tensor.
- **callback_on_step_end** (*Callable*, *PipelineCallback*, *MultiPipelineCallbacks*, *optional*) --
  Callback invoked at the end of each denoising step with signature `(self, step: int, timestep: int, callback_kwargs: Dict)`.
- **callback_on_step_end_tensor_inputs** (*List[str]*, *optional*, defaults to `["latents"]`) --
  Tensor keys included in `callback_kwargs` for `callback_on_step_end`.
- **max_sequence_length** (*int*, *optional*, defaults to 4096) --
  Maximum sequence length for prompt encoding.
- **enable_denormalization** (*bool*, *optional*, defaults to *True*) --
  Denormalise latents before VAE decoding.0[*~pipelines.joyimage.JoyImageEditPipelineOutput*] or *torch.Tensor*If `return_dict` is `True`, returns a pipeline output object containing the generated image(s).
Otherwise returns the image tensor directly.

Generate an edited image conditioned on a reference image and a text prompt.

Examples:
```python
>>> import torch
>>> from diffusers import JoyImageEditPipeline
>>> from diffusers.utils import load_image

>>> model_id = "jdopensource/JoyAI-Image-Edit-Diffusers"
>>> pipe = JoyImageEditPipeline.from_pretrained(model_id, torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> image = load_image("https://huggingface.co/datasets/diffusers/docs-images/resolve/main/astronaut.jpg")
>>> output = pipe(
...     image=image,  # pass an image for editing; omit for text-to-image generation
...     prompt="Add wings to the astronaut.",
...     num_inference_steps=40,
...     guidance_scale=4.0,
...     generator=torch.manual_seed(0),
... )
>>> output.images[0].save("joyimage_edit.png")
```

**Parameters:**

prompt (*str* or *List[str]*) : The prompt or prompts to guide generation.

height (*int*) : Height of the generated output in pixels.

width (*int*) : Width of the generated output in pixels.

image (*PipelineImageInput*, *optional*) : Reference image used for conditioning. When provided the pipeline operates in image-editing mode with `num_items=2`.

num_inference_steps (*int*, *optional*, defaults to 40) : Number of denoising steps. More steps generally improve quality at the cost of slower inference.

timesteps (*List[int]*, *optional*) : Custom timesteps for the denoising process. When provided, `num_inference_steps` is inferred from the list length.

sigmas (*List[float]*, *optional*) : Custom sigmas for the denoising process. Mutually exclusive with `timesteps`.

guidance_scale (*float*, *optional*, defaults to 4.0) : Classifier-free guidance scale.

negative_prompt (*str* or *List[str]*, *optional*) : Negative prompt(s) used to suppress undesired content.

num_images_per_prompt (*int*, *optional*, defaults to 1) : Number of generated samples per prompt.

generator (*torch.Generator* or *List[torch.Generator]*, *optional*) : RNG generator(s) for deterministic sampling.

latents (*torch.Tensor*, *optional*) : Pre-generated noisy latents for the target slot. Sampled from a Gaussian distribution when not provided. Can be used to seed generation from a specific starting noise tensor.

prompt_embeds (*torch.Tensor*, *optional*) : Pre-computed prompt embeddings. When provided `prompt` can be omitted.

prompt_embeds_mask (*torch.Tensor*, *optional*) : Attention mask for `prompt_embeds`.

negative_prompt_embeds (*torch.Tensor*, *optional*) : Pre-computed negative prompt embeddings.

negative_prompt_embeds_mask (*torch.Tensor*, *optional*) : Attention mask for `negative_prompt_embeds`.

output_type (*str*, *optional*, defaults to `"pil"`) : Output format. Pass `"latent"` to return raw latents.

return_dict (*bool*, *optional*, defaults to *True*) : Whether to return a [JoyImageEditPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/joyimage_edit#diffusers.JoyImageEditPipelineOutput) or a plain tensor.

callback_on_step_end (*Callable*, *PipelineCallback*, *MultiPipelineCallbacks*, *optional*) : Callback invoked at the end of each denoising step with signature `(self, step: int, timestep: int, callback_kwargs: Dict)`.

callback_on_step_end_tensor_inputs (*List[str]*, *optional*, defaults to `["latents"]`) : Tensor keys included in `callback_kwargs` for `callback_on_step_end`.

max_sequence_length (*int*, *optional*, defaults to 4096) : Maximum sequence length for prompt encoding.

enable_denormalization (*bool*, *optional*, defaults to *True*) : Denormalise latents before VAE decoding.

**Returns:**

`[*~pipelines.joyimage.JoyImageEditPipelineOutput*] or *torch.Tensor*`

If `return_dict` is `True`, returns a pipeline output object containing the generated image(s).
Otherwise returns the image tensor directly.
#### check_inputs[[diffusers.JoyImageEditPipeline.check_inputs]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/joyimage/pipeline_joyimage_edit.py#L409)

Validate pipeline inputs before the forward pass.
#### denormalize_latents[[diffusers.JoyImageEditPipeline.denormalize_latents]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/joyimage/pipeline_joyimage_edit.py#L476)

Invert `normalize_latents` to recover the original latent scale.

**Parameters:**

latent : Normalised latent tensor.

**Returns:**

Latent tensor in the scale expected by `vae.decode`.
#### encode_prompt[[diffusers.JoyImageEditPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/joyimage/pipeline_joyimage_edit.py#L364)

Encode a text prompt into embeddings (text-only path).

Pre-computed `prompt_embeds` bypass encoding entirely.

**Parameters:**

prompt : Prompt string or list of prompt strings.

device : Target device.

num_images_per_prompt : Number of outputs to generate per prompt.

prompt_embeds : Pre-computed prompt embeddings.

prompt_embeds_mask : Attention mask for pre-computed embeddings.

max_sequence_length : Maximum output sequence length.

template_type : Prompt template key (`"image"` or `"multiple_images"`).

**Returns:**

Tuple of (prompt_embeds, prompt_embeds_mask).
#### encode_prompt_multiple_images[[diffusers.JoyImageEditPipeline.encode_prompt_multiple_images]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/joyimage/pipeline_joyimage_edit.py#L286)

Encode prompts that contain inline image tokens via the Qwen processor.

`&amp;lt;image>\n` placeholders in each prompt string are replaced by the Qwen vision special tokens before being
fed to the multimodal encoder.

**Parameters:**

prompt : Prompt string(s), optionally containing `&amp;lt;image>\n` tokens.

device : Target device.

num_images_per_prompt : Number of outputs to generate per prompt.

images : Pixel tensors corresponding to the inline image tokens.

prompt_embeds : Pre-computed prompt embeddings.

prompt_embeds_mask : Attention mask for pre-computed embeddings.

template_type : Must be `"multiple_images"`.

max_sequence_length : If set, truncate the output to this length (keeping the last `max_sequence_length` tokens).

**Returns:**

Tuple of (prompt_embeds, prompt_embeds_mask).
#### normalize_latents[[diffusers.JoyImageEditPipeline.normalize_latents]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/joyimage/pipeline_joyimage_edit.py#L447)

Normalise latents using per-channel statistics from the VAE config.

Uses (latent - mean) / std when the VAE exposes `latents_mean` and `latents_std`; otherwise falls back to
scaling by `scaling_factor`.

**Parameters:**

latent : Raw latent tensor from `vae.encode`.

**Returns:**

Normalised latent tensor.
#### prepare_latents[[diffusers.JoyImageEditPipeline.prepare_latents]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/joyimage/pipeline_joyimage_edit.py#L502)

Prepare the initial noisy latent tensor for the denoising loop.

**Parameters:**

batch_size : Number of samples in the batch.

num_channels_latents : Latent channel dimension from the transformer config.

height : Spatial height in pixels.

width : Spatial width in pixels.

video_length : Number of frames (1 for image inference).

dtype : Floating-point dtype for the latent tensor.

device : Target device.

generator : RNG generator(s) for reproducible sampling.

latents : Optional user-provided initial noise for the target slot. When `None` random noise is sampled.

image : Optional list of PIL reference images to VAE-encode as conditioning slots.

enable_denormalization : Whether to normalise encoded reference latents.

**Returns:**

Tuple of `(latents, image_latents)` where `latents` has shape `(B, 1, C, T, H', W')` and
`image_latents` has shape `(B, N_ref, C, T, H', W')` or `None` when no reference images are given.

## JoyImageEditPipelineOutput[[diffusers.JoyImageEditPipelineOutput]]

#### diffusers.JoyImageEditPipelineOutput[[diffusers.JoyImageEditPipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/joyimage/pipeline_output.py#L11)

Output class for JoyImageEdit generation pipelines.
