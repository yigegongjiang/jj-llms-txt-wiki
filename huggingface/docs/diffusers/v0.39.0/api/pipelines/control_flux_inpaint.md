# FluxControlInpaint

  

FluxControlInpaintPipeline is an implementation of Inpainting for Flux.1 Depth/Canny models. It is a pipeline that allows you to inpaint images using the Flux.1 Depth/Canny models. The pipeline takes an image and a mask as input and returns the inpainted image.

FLUX.1 Depth and Canny [dev] is a 12 billion parameter rectified flow transformer capable of generating an image based on a text description while following the structure of a given input image. **This is not a ControlNet model**.

| Control type | Developer | Link |
| -------- | ---------- | ---- |
| Depth | [Black Forest Labs](https://huggingface.co/black-forest-labs) | [Link](https://huggingface.co/black-forest-labs/FLUX.1-Depth-dev) |
| Canny | [Black Forest Labs](https://huggingface.co/black-forest-labs) | [Link](https://huggingface.co/black-forest-labs/FLUX.1-Canny-dev) |

> [!TIP]
> Flux can be quite expensive to run on consumer hardware devices. However, you can perform a suite of optimizations to run it faster and in a more memory-friendly manner. Check out [this section](https://huggingface.co/blog/sd3#memory-optimizations-for-sd3) for more details. Additionally, Flux can benefit from quantization for memory efficiency with a trade-off in inference latency. Refer to [this blog post](https://huggingface.co/blog/quanto-diffusers) to learn more. For an exhaustive list of resources, check out [this gist](https://gist.github.com/sayakpaul/b664605caf0aa3bf8585ab109dd5ac9c).

```python
import torch
from diffusers import FluxControlInpaintPipeline
from diffusers.models.transformers import FluxTransformer2DModel
from transformers import T5EncoderModel
from diffusers.utils import load_image, make_image_grid
from image_gen_aux import DepthPreprocessor # https://github.com/huggingface/image_gen_aux
from PIL import Image
import numpy as np

pipe = FluxControlInpaintPipeline.from_pretrained(
    "black-forest-labs/FLUX.1-Depth-dev",
    torch_dtype=torch.bfloat16,
)
# use following lines if you have GPU constraints
# ---------------------------------------------------------------
transformer = FluxTransformer2DModel.from_pretrained(
    "sayakpaul/FLUX.1-Depth-dev-nf4", subfolder="transformer", torch_dtype=torch.bfloat16
)
text_encoder_2 = T5EncoderModel.from_pretrained(
    "sayakpaul/FLUX.1-Depth-dev-nf4", subfolder="text_encoder_2", torch_dtype=torch.bfloat16
)
pipe.transformer = transformer
pipe.text_encoder_2 = text_encoder_2
pipe.enable_model_cpu_offload()
# ---------------------------------------------------------------
pipe.to("cuda")

prompt = "a blue robot singing opera with human-like expressions"
image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/robot.png")

head_mask = np.zeros_like(image)
head_mask[65:580,300:642] = 255
mask_image = Image.fromarray(head_mask)

processor = DepthPreprocessor.from_pretrained("LiheYoung/depth-anything-large-hf")
control_image = processor(image)[0].convert("RGB")

output = pipe(
    prompt=prompt,
    image=image,
    control_image=control_image,
    mask_image=mask_image,
    num_inference_steps=30,
    strength=0.9,
    guidance_scale=10.0,
    generator=torch.Generator().manual_seed(42),
).images[0]
make_image_grid([image, control_image, mask_image, output.resize(image.size)], rows=1, cols=4).save("output.png")
```

## FluxControlInpaintPipeline[[diffusers.FluxControlInpaintPipeline]]
#### diffusers.FluxControlInpaintPipeline[[diffusers.FluxControlInpaintPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/flux/pipeline_flux_control_inpaint.py#L205)

The Flux pipeline for image inpainting using Flux-dev-Depth/Canny.

Reference: https://blackforestlabs.ai/announcing-black-forest-labs/

__call__diffusers.FluxControlInpaintPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/flux/pipeline_flux_control_inpaint.py#L805[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "prompt_2", "val": ": str | list[str] | None = None"}, {"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] = None"}, {"name": "control_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] = None"}, {"name": "mask_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] = None"}, {"name": "masked_image_latents", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "strength", "val": ": float = 0.6"}, {"name": "num_inference_steps", "val": ": int = 28"}, {"name": "sigmas", "val": ": list[float] | None = None"}, {"name": "guidance_scale", "val": ": float = 7.0"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.FloatTensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.FloatTensor | None = None"}, {"name": "pooled_prompt_embeds", "val": ": torch.FloatTensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "joint_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **prompt_2** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is
  will be used instead
- **image** (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) --
  `Image`, numpy array or tensor representing an image batch to be used as the starting point. For both
  numpy array and pytorch tensor, the expected value range is between `[0, 1]` If it's a tensor or a list
  or tensors, the expected shape should be `(B, C, H, W)` or `(C, H, W)`. If it is a numpy array or a
  list of arrays, the expected shape should be `(B, H, W, C)` or `(H, W, C)` It can also accept image
  latents as `image`, but if passing latents directly it is not encoded again.
- **control_image** (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, `list[np.ndarray]`, --
  `list[list[torch.Tensor]]`, `list[list[np.ndarray]]` or `list[list[PIL.Image.Image]]`):
  The ControlNet input condition to provide guidance to the `unet` for generation. If the type is
  specified as `torch.Tensor`, it is passed to ControlNet as is. `PIL.Image.Image` can also be accepted
  as an image. The dimensions of the output image defaults to `image`'s dimensions. If height and/or
  width are passed, `image` is resized accordingly. If multiple ControlNets are specified in `init`,
  images must be passed as a list such that each element of the list can be correctly batched for input
  to a single ControlNet.
- **mask_image** (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) --
  `Image`, numpy array or tensor representing an image batch to mask `image`. White pixels in the mask
  are repainted while black pixels are preserved. If `mask_image` is a PIL image, it is converted to a
  single channel (luminance) before use. If it's a numpy array or pytorch tensor, it should contain one
  color channel (L) instead of 3, so the expected shape for pytorch tensor would be `(B, 1, H, W)`, `(B,
  H, W)`, `(1, H, W)`, `(H, W)`. And for numpy array would be for `(B, H, W, 1)`, `(B, H, W)`, `(H, W,
  1)`, or `(H, W)`.
- **masked_image_latents** (`torch.Tensor`, `list[torch.Tensor]`) --
  `Tensor` representing an image batch to mask `image` generated by VAE. If not provided, the mask
  latents tensor will be generated by `mask_image`.
- **height** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The height in pixels of the generated image. This is set to 1024 by default for the best results.
- **width** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The width in pixels of the generated image. This is set to 1024 by default for the best results.
- **strength** (`float`, *optional*, defaults to 1.0) --
  Indicates extent to transform the reference `image`. Must be between 0 and 1. `image` is used as a
  starting point and more noise is added the higher the `strength`. The number of denoising steps depends
  on the amount of noise initially added. When `strength` is 1, added noise is maximum and the denoising
  process runs for the full number of iterations specified in `num_inference_steps`. A value of 1
  essentially ignores `image`.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **sigmas** (`list[float]`, *optional*) --
  Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in
  their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed
  will be used.
- **guidance_scale** (`float`, *optional*, defaults to 7.0) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2.
  of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting
  `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to
  the text `prompt`, usually at the expense of lower image quality.
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.FloatTensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **pooled_prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting.
  If not provided, pooled text embeddings will be generated from `prompt` input argument.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.flux.FluxPipelineOutput` instead of a plain tuple.
- **joint_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`list`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.
- **max_sequence_length** (`int` defaults to 512) -- Maximum sequence length to use with the `prompt`.0`~pipelines.flux.FluxPipelineOutput` or `tuple``~pipelines.flux.FluxPipelineOutput` if `return_dict`
is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the generated
images.

Function invoked when calling the pipeline for generation.

Examples:
```py
import torch
from diffusers import FluxControlInpaintPipeline
from diffusers.models.transformers import FluxTransformer2DModel
from transformers import T5EncoderModel
from diffusers.utils import load_image, make_image_grid
from image_gen_aux import DepthPreprocessor  # https://github.com/huggingface/image_gen_aux
from PIL import Image
import numpy as np

pipe = FluxControlInpaintPipeline.from_pretrained(
    "black-forest-labs/FLUX.1-Depth-dev",
    torch_dtype=torch.bfloat16,
)
# use following lines if you have GPU constraints
# ---------------------------------------------------------------
transformer = FluxTransformer2DModel.from_pretrained(
    "sayakpaul/FLUX.1-Depth-dev-nf4", subfolder="transformer", torch_dtype=torch.bfloat16
)
text_encoder_2 = T5EncoderModel.from_pretrained(
    "sayakpaul/FLUX.1-Depth-dev-nf4", subfolder="text_encoder_2", torch_dtype=torch.bfloat16
)
pipe.transformer = transformer
pipe.text_encoder_2 = text_encoder_2
pipe.enable_model_cpu_offload()
# ---------------------------------------------------------------
pipe.to("cuda")

prompt = "a blue robot singing opera with human-like expressions"
image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/robot.png")

head_mask = np.zeros_like(image)
head_mask[65:580, 300:642] = 255
mask_image = Image.fromarray(head_mask)

processor = DepthPreprocessor.from_pretrained("LiheYoung/depth-anything-large-hf")
control_image = processor(image)[0].convert("RGB")

output = pipe(
    prompt=prompt,
    image=image,
    control_image=control_image,
    mask_image=mask_image,
    num_inference_steps=30,
    strength=0.9,
    guidance_scale=10.0,
    generator=torch.Generator().manual_seed(42),
).images[0]
make_image_grid([image, control_image, mask_image, output.resize(image.size)], rows=1, cols=4).save(
    "output.png"
)
```

**Parameters:**

transformer ([FluxTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/flux_transformer#diffusers.FluxTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

text_encoder_2 (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer_2 (`T5TokenizerFast`) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

**Returns:**

``~pipelines.flux.FluxPipelineOutput` or `tuple``

`~pipelines.flux.FluxPipelineOutput` if `return_dict`
is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the generated
images.
#### disable_vae_slicing[[diffusers.FluxControlInpaintPipeline.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/flux/pipeline_flux_control_inpaint.py#L589)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### disable_vae_tiling[[diffusers.FluxControlInpaintPipeline.disable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/flux/pipeline_flux_control_inpaint.py#L616)

Disable tiled VAE decoding. If `enable_vae_tiling` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_vae_slicing[[diffusers.FluxControlInpaintPipeline.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/flux/pipeline_flux_control_inpaint.py#L576)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### enable_vae_tiling[[diffusers.FluxControlInpaintPipeline.enable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/flux/pipeline_flux_control_inpaint.py#L602)

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.
#### encode_prompt[[diffusers.FluxControlInpaintPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/flux/pipeline_flux_control_inpaint.py#L375)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is used in all text-encoders

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

## FluxPipelineOutput[[diffusers.pipelines.flux.pipeline_output.FluxPipelineOutput]]
#### diffusers.pipelines.flux.pipeline_output.FluxPipelineOutput[[diffusers.pipelines.flux.pipeline_output.FluxPipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/flux/pipeline_output.py#L11)

Output class for Flux image generation pipelines.

**Parameters:**

images (`list[PIL.Image.Image]` or `torch.Tensor` or `np.ndarray`) : list of denoised PIL images of length `batch_size` or numpy array or torch tensor of shape `(batch_size, height, width, num_channels)`. PIL images or numpy array present the denoised images of the diffusion pipeline. Torch tensors can represent either the denoised images or the intermediate latents ready to be passed to the decoder.
