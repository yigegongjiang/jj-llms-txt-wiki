# JoyAI-Image-Edit-Plus

[JoyAI-Image](https://github.com/jd-opensource/JoyAI-Image) is a unified multimodal foundation model for image understanding, text-to-image generation, and instruction-guided image editing. It combines an 8B Multimodal Large Language Model (MLLM) with a 16B Multimodal Diffusion Transformer (MMDiT).

JoyAI-Image-Edit-Plus is a multi-image instruction-guided editing model that accepts **multiple reference images** and a text instruction to generate a new image that combines elements from the references according to the instruction. It supports 1–5 reference images per sample.

| Model | Description | Download |
|:-----:|:-----------:|:--------:|
| JoyAI-Image-Edit-Plus | Multi-image instruction-guided editing with element composition from multiple references | [Hugging Face](https://huggingface.co/jdopensource/JoyAI-Image-Edit-Plus-Diffusers) |

```python
import torch
from PIL import Image
from diffusers import JoyImageEditPlusPipeline

pipeline = JoyImageEditPlusPipeline.from_pretrained(
    "jdopensource/JoyAI-Image-Edit-Plus-Diffusers", dtype=torch.bfloat16
)
pipeline.to("cuda")

images = [
    Image.open("reference_0.png").convert("RGB"),
    Image.open("reference_1.png").convert("RGB"),
]

target_h, target_w = pipeline.image_processor.get_default_height_width(images[-1])

output = pipeline(
    images=images,
    prompt="Combine the person from the second image with the scene from the first image.",
    negative_prompt="low quality, blurry, deformed",
    height=target_h,
    width=target_w,
    num_inference_steps=30,
    guidance_scale=4.0,
    generator=torch.Generator("cuda").manual_seed(42),
).images[0]
output.save("joyimage_edit_plus_output.png")
```

## JoyImageEditPlusPipeline[[diffusers.JoyImageEditPlusPipeline]]

#### diffusers.JoyImageEditPlusPipeline[[diffusers.JoyImageEditPlusPipeline]]

```python
diffusers.JoyImageEditPlusPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKLWan, text_encoder: Qwen3VLForConditionalGeneration, tokenizer: Qwen2Tokenizer, transformer: JoyImageEditPlusTransformer3DModel, processor: Qwen3VLProcessor, text_token_max_length: int = 2048)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/joyimage/pipeline_joyimage_edit_plus.py#L129)

**Parameters:**

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLWan](/docs/diffusers/v0.40.0/en/api/models/autoencoder_kl_wan#diffusers.AutoencoderKLWan)) : Variational Auto-Encoder (VAE) model to encode and decode images to and from latent representations.

text_encoder (`Qwen3VLForConditionalGeneration`) : Multimodal text encoder for prompt encoding with inline image understanding.

tokenizer (`Qwen2Tokenizer`) : Tokenizer for text processing.

transformer ([JoyImageEditPlusTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/transformer_joyimage_edit_plus#diffusers.JoyImageEditPlusTransformer3DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

processor (`Qwen3VLProcessor`) : Processor for multimodal inputs (text + images).

text_token_max_length (`int`, defaults to `2048`) : Maximum token length for text encoding.

Diffusion pipeline for multi-image instruction-guided editing using JoyImage Edit Plus.

Supports multiple reference images with different resolutions. Each reference image is independently VAE-encoded
and patchified, then concatenated with the target noise patches for joint denoising.

#### __call__[[diffusers.JoyImageEditPlusPipeline.__call__]]

```python
__call__(images: list[PIL.Image.Image] | list[list[PIL.Image.Image]] | None = None, prompt: str | list[str] = None, height: int | None = None, width: int | None = None, num_inference_steps: int = 30, timesteps: list = None, sigmas: list = None, guidance_scale: float = 4.0, negative_prompt: str | list[str] | None = None, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_embeds_mask: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds_mask: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, callback_on_step_end: typing.Union[typing.Callable[[int, int, dict], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 4096)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/joyimage/pipeline_joyimage_edit_plus.py#L441)

**Parameters:**

images (`list[Image.Image]` or `list[list[Image.Image]]`, *optional*) : Reference images for editing. Each image can have a different resolution. If a flat list is provided, it is treated as one sample with multiple references.

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds` instead.

height (`int`, *optional*) : The height in pixels of the generated image. If `None`, determined from the last reference image.

width (`int`, *optional*) : The width in pixels of the generated image. If `None`, determined from the last reference image.

num_inference_steps (`int`, *optional*, defaults to `30`) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

timesteps (`list[int]`, *optional*) : Custom timesteps to use for the denoising process. If not defined, equal spacing is used.

sigmas (`list[float]`, *optional*) : Custom sigmas to use for the denoising process.

guidance_scale (`float`, *optional*, defaults to `4.0`) : Classifier-free guidance scale. Higher values encourage the model to generate images more aligned with the `prompt` at the expense of lower image quality.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, a blank prompt is used for classifier-free guidance.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents to be used as inputs for image generation.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs.

prompt_embeds_mask (`torch.Tensor`, *optional*) : Attention mask for pre-generated text embeddings.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings.

negative_prompt_embeds_mask (`torch.Tensor`, *optional*) : Attention mask for pre-generated negative text embeddings.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generated image. Choose between `"pil"` (`PIL.Image.Image`), `"np"` (`np.ndarray`), `"pt"` (`torch.Tensor`), or `"latent"` for raw latent output.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [JoyImageEditPlusPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/joyimage_edit_plus#diffusers.JoyImageEditPlusPipelineOutput) instead of a plain tuple.

callback_on_step_end (`Callable`, *optional*) : A function called at the end of each denoising step with arguments: the pipeline, step index, timestep, and a dict of callback tensor inputs.

callback_on_step_end_tensor_inputs (`list[str]`, *optional*, defaults to `["latents"]`) : The list of tensor inputs for the `callback_on_step_end` function.

max_sequence_length (`int`, *optional*, defaults to `4096`) : Maximum sequence length for the text encoder.

**Returns:** [JoyImageEditPlusPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/joyimage_edit_plus#diffusers.JoyImageEditPlusPipelineOutput) or `tuple`

If `return_dict` is `True`, [JoyImageEditPlusPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/joyimage_edit_plus#diffusers.JoyImageEditPlusPipelineOutput) is returned, otherwise a `tuple` is
returned where the first element is a list of generated images.

Function invoked when calling the pipeline for generation.

Examples:
```python
>>> import torch
>>> from diffusers import JoyImageEditPlusPipeline
>>> from diffusers.utils import load_image

>>> model_id = "jdopensource/JoyAI-Image-Edit-Plus-Diffusers"
>>> pipe = JoyImageEditPlusPipeline.from_pretrained(model_id, torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> images = [
...     load_image("dog.png"),
...     load_image("person.png"),
... ]
>>> output = pipe(
...     images=images,
...     prompt="Let the person lovingly play with the dog.",
...     height=1024,
...     width=1024,
...     num_inference_steps=30,
...     guidance_scale=4.0,
...     generator=torch.manual_seed(42),
... )
>>> output.images[0].save("output.png")
```

#### encode_prompt_multiple_images[[diffusers.JoyImageEditPlusPipeline.encode_prompt_multiple_images]]

```python
encode_prompt_multiple_images(prompt: str | list[str], device: typing.Optional[torch.device] = None, images: list[PIL.Image.Image] | None = None, max_sequence_length: int | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/joyimage/pipeline_joyimage_edit_plus.py#L229)

Encode prompts with inline  tokens via the Qwen3-VL processor.

#### prepare_latents[[diffusers.JoyImageEditPlusPipeline.prepare_latents]]

```python
prepare_latents(batch_size: int, num_channels_latents: int, height: int, width: int, dtype: dtype, device: device, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType], reference_images: list[list[PIL.Image.Image]] | None = None, latents: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/joyimage/pipeline_joyimage_edit_plus.py#L274)

**Parameters:**

latents : Optional pre-computed noise for the target slot. Shape `(B, C, 1, H', W')` where `H'` and `W'` are the latent-space dimensions. When `None`, random noise is sampled.

**Returns:** `padded_latents`

[B, max_patches, C, pt, ph, pw] target_mask: [B, max_patches] (True for target patches)
shape_list: per-sample list of (t, h, w) tuples for each component

Prepare 6D padded latent tensor with target noise + reference image latents.

## JoyImageEditPlusPipelineOutput[[diffusers.JoyImageEditPlusPipelineOutput]]

#### diffusers.JoyImageEditPlusPipelineOutput[[diffusers.JoyImageEditPlusPipelineOutput]]

```python
diffusers.JoyImageEditPlusPipelineOutput(images: typing.Union[typing.List[PIL.Image.Image], numpy.ndarray])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/joyimage/pipeline_output.py#L20)

Output class for JoyImage Edit Plus multi-image editing pipelines.
