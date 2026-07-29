# DreamLite

DreamLite is a text-to-image and image-editing model from ByteDance. It pairs a custom 2D U-Net
(`DreamLiteUNetModel`) with the `Qwen3-VL` multimodal encoder as its prompt / image-instruction encoder,
and uses an `AutoencoderTiny` (TAESD-style) VAE for fast latent encode/decode.

Two pipelines are exposed:

| Pipeline | Modes | CFG | Use case |
|---|---|---|---|
| [DreamLitePipeline](/docs/diffusers/v0.39.0/en/api/pipelines/dreamlite#diffusers.DreamLitePipeline) | text-to-image **and** image-editing (auto-selected by whether `image` is `None`) | 3-branch dual CFG (`guidance_scale` on text branch, `image_guidance_scale` on image branch, à la InstructPix2Pix) | Highest quality |
| [DreamLiteMobilePipeline](/docs/diffusers/v0.39.0/en/api/pipelines/dreamlite#diffusers.DreamLiteMobilePipeline) | text-to-image **and** image-editing (auto-selected by whether `image` is `None`) | None — distilled, single UNet forward per step | On-device / low-latency |

Official checkpoints:

* Base model: [carlofkl/DreamLite-base](https://huggingface.co/carlofkl/DreamLite-base)
* Distilled mobile model: [carlofkl/DreamLite-mobile](https://huggingface.co/carlofkl/DreamLite-mobile)

> [!TIP]
> Both pipelines auto-detect text-to-image vs. image-editing mode from whether the `image` argument is
> provided. There is no separate `Img2Img` class.

> [!TIP]
> When loading an input image for editing, prefer `diffusers.utils.load_image(...)` over raw `PIL.Image.open(...)`.
> `load_image` enforces an RGB conversion and applies EXIF orientation, both of which the pipeline assumes.
> A plain `Image.open` of an RGBA / palette / EXIF-rotated source will silently produce a different latent
> conditioning and degrade output quality.

## Text-to-image (Base)

```python
import torch
from diffusers import DreamLitePipeline

pipe = DreamLitePipeline.from_pretrained("carlofkl/DreamLite-base", revision="diffusers", torch_dtype=torch.bfloat16)
pipe = pipe.to("cuda")

image = pipe(
    prompt="a dog running on the grass",
    negative_prompt="",
    height=1024,
    width=1024,
    num_inference_steps=28,
    generator=torch.Generator("cpu").manual_seed(42),
).images[0]
image.save("dreamlite_t2i.png")
```

## Image editing (Base)

Pass an `image` to enter edit mode. Both `guidance_scale` (text branch) and `image_guidance_scale`
(image branch) are active here.

```python
import torch
from diffusers import DreamLitePipeline
from diffusers.utils import load_image

pipe = DreamLitePipeline.from_pretrained("carlofkl/DreamLite-base", revision="diffusers", torch_dtype=torch.bfloat16)
pipe = pipe.to("cuda")

source = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/cat.png")

image = pipe(
    prompt="turn the cat into a corgi",
    image=source,
    height=1024,
    width=1024,
    num_inference_steps=28,
    generator=torch.Generator("cpu").manual_seed(42),
).images[0]
image.save("dreamlite_edit.png")
```

## Text-to-image (Mobile)

The mobile pipeline is distilled and skips CFG entirely — a single UNet forward per step. It accepts the
same `prompt` / `height` / `width` / `num_inference_steps` arguments, but **ignores** `guidance_scale` and
`image_guidance_scale` if passed (a warning is logged).

```python
import torch
from diffusers import DreamLiteMobilePipeline

pipe = DreamLiteMobilePipeline.from_pretrained("carlofkl/DreamLite-mobile", revision="diffusers", torch_dtype=torch.bfloat16)
pipe = pipe.to("cuda")

image = pipe(
    prompt="a dog running on the grass",
    height=1024,
    width=1024,
    num_inference_steps=4,
    generator=torch.Generator("cpu").manual_seed(42),
).images[0]
image.save("dreamlite_mobile_t2i.png")
```

## Image editing (Mobile)

```python
import torch
from diffusers import DreamLiteMobilePipeline
from diffusers.utils import load_image

pipe = DreamLiteMobilePipeline.from_pretrained("carlofkl/DreamLite-mobile", revision="diffusers", torch_dtype=torch.bfloat16)
pipe = pipe.to("cuda")

source = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/cat.png")

image = pipe(
    prompt="turn the cat into a corgi",
    image=source,
    height=1024,
    width=1024,
    num_inference_steps=4,
    generator=torch.Generator("cpu").manual_seed(42),
).images[0]
image.save("dreamlite_mobile_edit.png")
```

## Notes and limitations

* Both pipelines force `batch_size = 1` internally; `num_images_per_prompt` controls how many samples
  are drawn from the same prompt rather than parallel batching.
* The prompt encoder is `Qwen3-VL`, which is a multimodal model. Loading the full pipeline therefore
  requires sufficient GPU memory for both the U-Net and the Qwen3-VL text encoder (~4 GB + ~0.7 GB
  in bf16 for the base release).
* The VAE is `AutoencoderTiny` and exposes `encoder_block_out_channels`; `vae_scale_factor` is derived
  from it at pipeline init time.

## DreamLitePipeline[[diffusers.DreamLitePipeline]]

#### diffusers.DreamLitePipeline[[diffusers.DreamLitePipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/dreamlite/pipeline_dreamlite.py#L155)

DreamLite pipeline for text-to-image and instruction-based image editing.

The same pipeline supports both modes; the operating mode is auto-detected from the inputs:

- `image is None` -> text-to-image (single CFG on text).
- `image is not None` -> image-to-image / instruction edit (dual CFG: text + image).

Components:
text_encoder ([*~transformers.Qwen3VLForConditionalGeneration*]):
Multimodal text/vision encoder used to produce conditioning embeddings.
tokenizer ([*~transformers.AutoTokenizer*]):
Tokenizer for text-only (generate) mode.
processor ([*~transformers.Qwen3VLProcessor*]):
Multimodal processor for edit mode (text + image template).
vae ([*~diffusers.AutoencoderTiny*]):
Mobile-friendly tiny VAE for latent encode/decode.
unet ([*~diffusers.DreamLiteUNetModel*]):
DreamLite UNet (GQA + qk_norm + depthwise-separable convs).
scheduler ([*~diffusers.FlowMatchEulerDiscreteScheduler*]):
Flow-matching Euler scheduler with dynamic shift.

Note:
`batch_size` is currently forced to `1`; `num_images_per_prompt` is supported.

__call__diffusers.DreamLitePipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/dreamlite/pipeline_dreamlite.py#L388[{"name": "prompt", "val": ": typing.Optional[str] = None"}, {"name": "negative_prompt", "val": ": typing.Optional[str] = None"}, {"name": "image", "val": ": typing.Optional[PIL.Image.Image] = None"}, {"name": "height", "val": ": typing.Optional[int] = None"}, {"name": "width", "val": ": typing.Optional[int] = None"}, {"name": "guidance_scale", "val": ": float = 3.5"}, {"name": "image_guidance_scale", "val": ": float = 1.5"}, {"name": "num_inference_steps", "val": ": int = 30"}, {"name": "sigmas", "val": ": typing.Optional[typing.List[float]] = None"}, {"name": "num_images_per_prompt", "val": ": typing.Optional[int] = 1"}, {"name": "generator", "val": ": typing.Union[torch._C.Generator, typing.List[torch._C.Generator], NoneType] = None"}, {"name": "output_type", "val": ": typing.Optional[str] = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "max_sequence_length", "val": ": int = 200"}, {"name": "text_pad_embedding", "val": ": typing.Optional[torch.Tensor] = None"}]- **prompt** -- Text prompt.
- **negative_prompt** -- Negative text prompt (defaults to empty string).
- **image** -- Optional input image. If provided, the pipeline runs in **edit / image-to-image** mode
  with dual classifier-free guidance; otherwise it runs in **text-to-image** mode.
- **height** -- Output resolution (height). Defaults to `default_sample_size * vae_scale_factor` (1024).
  The same default applies in both T2I and I2I; pass an explicit value to override.
- **width** -- Output resolution (width). Defaults to `default_sample_size * vae_scale_factor` (1024).
  The same default applies in both T2I and I2I; pass an explicit value to override.
- **guidance_scale** -- CFG scale on the text branch (both modes).
- **image_guidance_scale** -- Additional CFG scale on the image branch (edit mode only).
- **num_inference_steps** -- Number of denoising steps.
- **sigmas** -- Optional explicit FlowMatch sigmas; defaults to a uniform linspace.
- **num_images_per_prompt** -- Output images per prompt (note: `batch_size` is forced to 1).
- **generator** -- Random generator(s).
- **output_type** -- `"pil"`, `"np"`, `"pt"` or `"latent"`.
- **return_dict** -- If True, returns a [DreamLitePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/dreamlite#diffusers.DreamLitePipelineOutput); else a tuple `(images,)`.
- **max_sequence_length** -- Maximum number of user-prompt tokens kept after dropping the chat-template
  prefix. Only applies to `generate` mode (the `edit` mode uses the multimodal processor's native
  padding).
- **text_pad_embedding** -- Optional learned pad embedding for masked positions.0[DreamLitePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/dreamlite#diffusers.DreamLitePipelineOutput) or `tuple`.
Run the DreamLite pipeline.

**Parameters:**

prompt : Text prompt.

negative_prompt : Negative text prompt (defaults to empty string).

image : Optional input image. If provided, the pipeline runs in **edit / image-to-image** mode with dual classifier-free guidance; otherwise it runs in **text-to-image** mode.

height : Output resolution (height). Defaults to `default_sample_size * vae_scale_factor` (1024). The same default applies in both T2I and I2I; pass an explicit value to override.

width : Output resolution (width). Defaults to `default_sample_size * vae_scale_factor` (1024). The same default applies in both T2I and I2I; pass an explicit value to override.

guidance_scale : CFG scale on the text branch (both modes).

image_guidance_scale : Additional CFG scale on the image branch (edit mode only).

num_inference_steps : Number of denoising steps.

sigmas : Optional explicit FlowMatch sigmas; defaults to a uniform linspace.

num_images_per_prompt : Output images per prompt (note: `batch_size` is forced to 1).

generator : Random generator(s).

output_type : `"pil"`, `"np"`, `"pt"` or `"latent"`.

return_dict : If True, returns a [DreamLitePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/dreamlite#diffusers.DreamLitePipelineOutput); else a tuple `(images,)`.

max_sequence_length : Maximum number of user-prompt tokens kept after dropping the chat-template prefix. Only applies to `generate` mode (the `edit` mode uses the multimodal processor's native padding).

text_pad_embedding : Optional learned pad embedding for masked positions.

**Returns:**

[DreamLitePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/dreamlite#diffusers.DreamLitePipelineOutput) or `tuple`.

## DreamLiteMobilePipeline[[diffusers.DreamLiteMobilePipeline]]

#### diffusers.DreamLiteMobilePipeline[[diffusers.DreamLiteMobilePipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/dreamlite/pipeline_dreamlite_mobile.py#L156)

DreamLite **Mobile** pipeline: a distilled, classifier-free-guidance-free variant of
[DreamLitePipeline](/docs/diffusers/v0.39.0/en/api/pipelines/dreamlite#diffusers.DreamLitePipeline) for fast few-step inference (default 4 steps).

The operating mode is auto-detected from inputs (same as the base pipeline):

- `image is None` -> text-to-image.
- `image is not None` -> image-to-image / instruction edit.

Because classifier-free guidance is **distilled away**, `guidance_scale` and `image_guidance_scale` are
accepted for API parity with [DreamLitePipeline](/docs/diffusers/v0.39.0/en/api/pipelines/dreamlite#diffusers.DreamLitePipeline) but are ignored in the denoising loop. `negative_prompt`
is intentionally absent.

Components (identical to the base pipeline):
text_encoder ([*~transformers.Qwen3VLForConditionalGeneration*]):
Multimodal text/vision encoder.
tokenizer ([*~transformers.AutoTokenizer*]):
Tokenizer for text-only (generate) mode.
processor ([*~transformers.Qwen3VLProcessor*]):
Multimodal processor for edit mode.
vae ([*~diffusers.AutoencoderTiny*]):
Mobile-friendly tiny VAE.
unet ([*~diffusers.DreamLiteUNetModel*]):
DreamLite UNet.
scheduler ([*~diffusers.FlowMatchEulerDiscreteScheduler*]):
Flow-matching Euler scheduler with dynamic shift.

Note:
`batch_size` is currently forced to `1`; `num_images_per_prompt` is supported.

__call__diffusers.DreamLiteMobilePipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/dreamlite/pipeline_dreamlite_mobile.py#L384[{"name": "prompt", "val": ": typing.Union[str, typing.List[str]] = None"}, {"name": "image", "val": ": typing.Optional[PIL.Image.Image] = None"}, {"name": "height", "val": ": typing.Optional[int] = None"}, {"name": "width", "val": ": typing.Optional[int] = None"}, {"name": "num_inference_steps", "val": ": int = 4"}, {"name": "guidance_scale", "val": ": typing.Optional[float] = None"}, {"name": "image_guidance_scale", "val": ": typing.Optional[float] = None"}, {"name": "sigmas", "val": ": typing.Optional[typing.List[float]] = None"}, {"name": "num_images_per_prompt", "val": ": typing.Optional[int] = 1"}, {"name": "generator", "val": ": typing.Union[torch._C.Generator, typing.List[torch._C.Generator], NoneType] = None"}, {"name": "output_type", "val": ": typing.Optional[str] = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "max_sequence_length", "val": ": int = 200"}, {"name": "text_pad_embedding", "val": ": typing.Optional[torch.Tensor] = None"}]- **prompt** -- Text prompt.
- **image** -- Optional input image. If provided, runs in **edit / image-to-image** mode;
  otherwise runs in **text-to-image** mode.
- **height** -- Output resolution (height). Defaults to `default_sample_size * vae_scale_factor` (1024).
- **width** -- Output resolution (width). Defaults to `default_sample_size * vae_scale_factor` (1024).
- **num_inference_steps** -- Number of denoising steps. Defaults to **4** (distilled).
- **guidance_scale** -- Accepted for API parity with [DreamLitePipeline](/docs/diffusers/v0.39.0/en/api/pipelines/dreamlite#diffusers.DreamLitePipeline); **ignored**
  because CFG was distilled away.
- **image_guidance_scale** -- Accepted for API parity with [DreamLitePipeline](/docs/diffusers/v0.39.0/en/api/pipelines/dreamlite#diffusers.DreamLitePipeline); **ignored**
  because CFG was distilled away.
- **sigmas** -- Optional explicit FlowMatch sigmas; defaults to a uniform linspace.
- **num_images_per_prompt** -- Output images per prompt (note: `batch_size` is forced to 1).
- **generator** -- Random generator(s).
- **output_type** -- `"pil"`, `"np"`, `"pt"` or `"latent"`.
- **return_dict** -- If True, returns a [DreamLitePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/dreamlite#diffusers.DreamLitePipelineOutput); else `(images,)`.
- **max_sequence_length** -- Maximum number of user-prompt tokens kept after dropping the chat-template
  prefix. Only applies to `generate` mode (the `edit` mode uses the multimodal processor's native
  padding).
- **text_pad_embedding** -- Optional learned pad embedding for masked positions.0[DreamLitePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/dreamlite#diffusers.DreamLitePipelineOutput) or `tuple`.
Run the distilled DreamLite Mobile pipeline.

**Parameters:**

prompt : Text prompt.

image : Optional input image. If provided, runs in **edit / image-to-image** mode; otherwise runs in **text-to-image** mode.

height : Output resolution (height). Defaults to `default_sample_size * vae_scale_factor` (1024).

width : Output resolution (width). Defaults to `default_sample_size * vae_scale_factor` (1024).

num_inference_steps : Number of denoising steps. Defaults to **4** (distilled).

guidance_scale : Accepted for API parity with [DreamLitePipeline](/docs/diffusers/v0.39.0/en/api/pipelines/dreamlite#diffusers.DreamLitePipeline); **ignored** because CFG was distilled away.

image_guidance_scale : Accepted for API parity with [DreamLitePipeline](/docs/diffusers/v0.39.0/en/api/pipelines/dreamlite#diffusers.DreamLitePipeline); **ignored** because CFG was distilled away.

sigmas : Optional explicit FlowMatch sigmas; defaults to a uniform linspace.

num_images_per_prompt : Output images per prompt (note: `batch_size` is forced to 1).

generator : Random generator(s).

output_type : `"pil"`, `"np"`, `"pt"` or `"latent"`.

return_dict : If True, returns a [DreamLitePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/dreamlite#diffusers.DreamLitePipelineOutput); else `(images,)`.

max_sequence_length : Maximum number of user-prompt tokens kept after dropping the chat-template prefix. Only applies to `generate` mode (the `edit` mode uses the multimodal processor's native padding).

text_pad_embedding : Optional learned pad embedding for masked positions.

**Returns:**

[DreamLitePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/dreamlite#diffusers.DreamLitePipelineOutput) or `tuple`.

## DreamLitePipelineOutput[[diffusers.DreamLitePipelineOutput]]

#### diffusers.DreamLitePipelineOutput[[diffusers.DreamLitePipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/dreamlite/pipeline_output.py#L25)

Output class for DreamLite pipelines.

**Parameters:**

images (`List[PIL.Image.Image]` or `np.ndarray`) : List of denoised PIL images of length `batch_size` or NumPy array of shape `(batch_size, height, width, num_channels)`. PIL images or NumPy array present the denoised images of the diffusion pipeline.
