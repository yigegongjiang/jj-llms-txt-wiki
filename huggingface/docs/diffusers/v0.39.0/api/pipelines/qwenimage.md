#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License. -->

# QwenImage

  

Qwen-Image from the Qwen team is an image generation foundation model in the Qwen series that achieves significant advances in complex text rendering and precise image editing. Experiments show strong general capabilities in both image generation and editing, with exceptional performance in text rendering, especially for Chinese.

Qwen-Image comes in the following variants:

| model type | model id |
|:----------:|:--------:|
| Qwen-Image | [`Qwen/Qwen-Image`](https://huggingface.co/Qwen/Qwen-Image) |
| Qwen-Image-Edit | [`Qwen/Qwen-Image-Edit`](https://huggingface.co/Qwen/Qwen-Image-Edit) |
| Qwen-Image-Edit Plus | [Qwen/Qwen-Image-Edit-2509](https://huggingface.co/Qwen/Qwen-Image-Edit-2509) |

> [!TIP]
> See the [Caching](../../optimization/cache) guide to speed up inference by storing and reusing intermediate outputs.

## LoRA for faster inference

Use a LoRA from `lightx2v/Qwen-Image-Lightning` to speed up inference by reducing the
number of steps. Refer to the code snippet below:

Code

```py
from diffusers import DiffusionPipeline, FlowMatchEulerDiscreteScheduler
import torch 
import math

ckpt_id = "Qwen/Qwen-Image"

# From
# https://github.com/ModelTC/Qwen-Image-Lightning/blob/342260e8f5468d2f24d084ce04f55e101007118b/generate_with_diffusers.py#L82C9-L97C10
scheduler_config = {
    "base_image_seq_len": 256,
    "base_shift": math.log(3),  # We use shift=3 in distillation
    "invert_sigmas": False,
    "max_image_seq_len": 8192,
    "max_shift": math.log(3),  # We use shift=3 in distillation
    "num_train_timesteps": 1000,
    "shift": 1.0,
    "shift_terminal": None,  # set shift_terminal to None
    "stochastic_sampling": False,
    "time_shift_type": "exponential",
    "use_beta_sigmas": False,
    "use_dynamic_shifting": True,
    "use_exponential_sigmas": False,
    "use_karras_sigmas": False,
}
scheduler = FlowMatchEulerDiscreteScheduler.from_config(scheduler_config)
pipe = DiffusionPipeline.from_pretrained(
    ckpt_id, scheduler=scheduler, torch_dtype=torch.bfloat16
).to("cuda")
pipe.load_lora_weights(
    "lightx2v/Qwen-Image-Lightning", weight_name="Qwen-Image-Lightning-8steps-V1.0.safetensors"
)

prompt = "a tiny astronaut hatching from an egg on the moon, Ultra HD, 4K, cinematic composition."
negative_prompt = " "
image = pipe(
    prompt=prompt,
    negative_prompt=negative_prompt,
    width=1024,
    height=1024,
    num_inference_steps=8,
    true_cfg_scale=1.0,
    generator=torch.manual_seed(0),
).images[0]
image.save("qwen_fewsteps.png")
```

> [!TIP]
> The `guidance_scale` parameter in the pipeline is there to support future guidance-distilled models when they come up. Note that passing `guidance_scale` to the pipeline is ineffective. To enable classifier-free guidance, please pass `true_cfg_scale` and `negative_prompt` (even an empty negative prompt like " ") should enable classifier-free guidance computations.

## Multi-image reference with QwenImageEditPlusPipeline

With [QwenImageEditPlusPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/qwenimage#diffusers.QwenImageEditPlusPipeline), one can provide multiple images as input reference.

```py
import torch
from PIL import Image
from diffusers import QwenImageEditPlusPipeline
from diffusers.utils import load_image

pipe = QwenImageEditPlusPipeline.from_pretrained(
    "Qwen/Qwen-Image-Edit-2509", torch_dtype=torch.bfloat16
).to("cuda")

image_1 = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/grumpy.jpg")
image_2 = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/peng.png")
image = pipe(
    image=[image_1, image_2],
    prompt='''put the penguin and the cat at a game show called "Qwen Edit Plus Games"''',
    num_inference_steps=50
).images[0]
```

## Performance

### torch.compile

Using `torch.compile` on the transformer provides ~2.4x speedup (A100 80GB: 4.70s → 1.93s):

```python
import torch
from diffusers import QwenImagePipeline

pipe = QwenImagePipeline.from_pretrained("Qwen/Qwen-Image", torch_dtype=torch.bfloat16).to("cuda")
pipe.transformer = torch.compile(pipe.transformer)

# First call triggers compilation (~7s overhead)
# Subsequent calls run at ~2.4x faster
image = pipe("a cat", num_inference_steps=50).images[0]
```

### Batched Inference with Variable-Length Prompts

When using classifier-free guidance (CFG) with prompts of different lengths, the pipeline properly handles padding through attention masking. This ensures padding tokens do not influence the generated output.

```python
# CFG with different prompt lengths works correctly
image = pipe(
    prompt="A cat",
    negative_prompt="blurry, low quality, distorted",
    true_cfg_scale=3.5,
    num_inference_steps=50,
).images[0]
```

For detailed benchmark scripts and results, see [this gist](https://gist.github.com/cdutr/bea337e4680268168550292d7819dc2f).

## QwenImagePipeline[[diffusers.QwenImagePipeline]]

#### diffusers.QwenImagePipeline[[diffusers.QwenImagePipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage.py#L132)

The QwenImage pipeline for text-to-image generation.

__call__diffusers.QwenImagePipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage.py#L461[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str | list[str] = None"}, {"name": "true_cfg_scale", "val": ": float = 4.0"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "sigmas", "val": ": list[float] | None = None"}, {"name": "guidance_scale", "val": ": float | None = None"}, {"name": "num_images_per_prompt", "val": ": int = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds_mask", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds_mask", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `true_cfg_scale` is
  not greater than `1`).
- **true_cfg_scale** (`float`, *optional*, defaults to 1.0) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `true_cfg_scale` is defined as `w` of equation 2.
  of [Imagen Paper](https://huggingface.co/papers/2205.11487). Classifier-free guidance is enabled by
  setting `true_cfg_scale > 1` and a provided `negative_prompt`. Higher guidance scale encourages to
  generate images that are closely linked to the text `prompt`, usually at the expense of lower image
  quality.
- **height** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The height in pixels of the generated image. This is set to 1024 by default for the best results.
- **width** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The width in pixels of the generated image. This is set to 1024 by default for the best results.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **sigmas** (`list[float]`, *optional*) --
  Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in
  their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed
  will be used.
- **guidance_scale** (`float`, *optional*, defaults to None) --
  A guidance scale value for guidance distilled models. Unlike the traditional classifier-free guidance
  where the guidance scale is applied during inference through noise prediction rescaling, guidance
  distilled models take the guidance scale directly as an input parameter during forward pass. Guidance
  scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images
  that are closely linked to the text `prompt`, usually at the expense of lower image quality. This
  parameter in the pipeline is there to support future guidance-distilled models when they come up. It is
  ignored when not using guidance distilled models. To enable traditional classifier-free guidance,
  please pass `true_cfg_scale > 1.0` and `negative_prompt` (even an empty negative prompt like " " should
  enable classifier-free guidance computations).
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **prompt_embeds_mask** (`torch.Tensor`, *optional*) --
  Attention mask for `prompt_embeds`.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
  argument.
- **negative_prompt_embeds_mask** (`torch.Tensor`, *optional*) --
  Attention mask for `negative_prompt_embeds`.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.qwenimage.QwenImagePipelineOutput` instead of a plain tuple.
- **attention_kwargs** (`dict`, *optional*) --
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
- **max_sequence_length** (`int` defaults to 512) -- Maximum sequence length to use with the `prompt`.0`~pipelines.qwenimage.QwenImagePipelineOutput` or `tuple``~pipelines.qwenimage.QwenImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When
returning a tuple, the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import QwenImagePipeline

>>> pipe = QwenImagePipeline.from_pretrained("Qwen/Qwen-Image", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")
>>> prompt = "A cat holding a sign that says hello world"
>>> # Depending on the variant being used, the pipeline call will slightly vary.
>>> # Refer to the pipeline documentation for more details.
>>> image = pipe(prompt, num_inference_steps=50).images[0]
>>> image.save("qwenimage.png")
```

**Parameters:**

transformer ([QwenImageTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/qwenimage_transformer2d#diffusers.QwenImageTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`Qwen2.5-VL-7B-Instruct`) : [Qwen2.5-VL-7B-Instruct](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct), specifically the [Qwen2.5-VL-7B-Instruct](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct) variant.

tokenizer (`Qwen2Tokenizer`) : Tokenizer of class https://huggingface.co/docs/transformers/en/model_doc/qwen2#transformers.Qwen2Tokenizer

**Returns:**

``~pipelines.qwenimage.QwenImagePipelineOutput` or `tuple``

`~pipelines.qwenimage.QwenImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When
returning a tuple, the first element is a list with the generated images.
#### disable_vae_slicing[[diffusers.QwenImagePipeline.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage.py#L369)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### disable_vae_tiling[[diffusers.QwenImagePipeline.disable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage.py#L396)

Disable tiled VAE decoding. If `enable_vae_tiling` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_vae_slicing[[diffusers.QwenImagePipeline.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage.py#L356)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### enable_vae_tiling[[diffusers.QwenImagePipeline.enable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage.py#L382)

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.
#### encode_prompt[[diffusers.QwenImagePipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage.py#L225)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

## QwenImageImg2ImgPipeline[[diffusers.QwenImageImg2ImgPipeline]]

#### diffusers.QwenImageImg2ImgPipeline[[diffusers.QwenImageImg2ImgPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_img2img.py#L134)

The QwenImage pipeline for text-to-image generation.

__call__diffusers.QwenImageImg2ImgPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_img2img.py#L535[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str | list[str] = None"}, {"name": "true_cfg_scale", "val": ": float = 4.0"}, {"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "strength", "val": ": float = 0.6"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "sigmas", "val": ": list[float] | None = None"}, {"name": "guidance_scale", "val": ": float | None = None"}, {"name": "num_images_per_prompt", "val": ": int = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds_mask", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds_mask", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `true_cfg_scale` is
  not greater than `1`).
- **image** (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) --
  `Image`, numpy array or tensor representing an image batch to be used as the starting point. For both
  numpy array and pytorch tensor, the expected value range is between `[0, 1]` If it's a tensor or a list
  or tensors, the expected shape should be `(B, C, H, W)` or `(C, H, W)`. If it is a numpy array or a
  list of arrays, the expected shape should be `(B, H, W, C)` or `(H, W, C)` It can also accept image
  latents as `image`, but if passing latents directly it is not encoded again.
- **true_cfg_scale** (`float`, *optional*, defaults to 1.0) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `true_cfg_scale` is defined as `w` of equation 2.
  of [Imagen Paper](https://huggingface.co/papers/2205.11487). Classifier-free guidance is enabled by
  setting `true_cfg_scale > 1` and a provided `negative_prompt`. Higher guidance scale encourages to
  generate images that are closely linked to the text `prompt`, usually at the expense of lower image
  quality.
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
- **guidance_scale** (`float`, *optional*, defaults to None) --
  A guidance scale value for guidance distilled models. Unlike the traditional classifier-free guidance
  where the guidance scale is applied during inference through noise prediction rescaling, guidance
  distilled models take the guidance scale directly as an input parameter during forward pass. Guidance
  scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images
  that are closely linked to the text `prompt`, usually at the expense of lower image quality. This
  parameter in the pipeline is there to support future guidance-distilled models when they come up. It is
  ignored when not using guidance distilled models. To enable traditional classifier-free guidance,
  please pass `true_cfg_scale > 1.0` and `negative_prompt` (even an empty negative prompt like " " should
  enable classifier-free guidance computations).
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **prompt_embeds_mask** (`torch.Tensor`, *optional*) --
  Attention mask for `prompt_embeds`.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
  argument.
- **negative_prompt_embeds_mask** (`torch.Tensor`, *optional*) --
  Attention mask for `negative_prompt_embeds`.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.qwenimage.QwenImagePipelineOutput` instead of a plain tuple.
- **attention_kwargs** (`dict`, *optional*) --
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
- **max_sequence_length** (`int` defaults to 512) -- Maximum sequence length to use with the `prompt`.0`~pipelines.qwenimage.QwenImagePipelineOutput` or `tuple``~pipelines.qwenimage.QwenImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When
returning a tuple, the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import QwenImageImg2ImgPipeline
>>> from diffusers.utils import load_image

>>> pipe = QwenImageImg2ImgPipeline.from_pretrained("Qwen/Qwen-Image", torch_dtype=torch.bfloat16)
>>> pipe = pipe.to("cuda")
>>> url = "https://raw.githubusercontent.com/CompVis/stable-diffusion/main/assets/stable-samples/img2img/sketch-mountains-input.jpg"
>>> init_image = load_image(url).resize((1024, 1024))
>>> prompt = "cat wizard, gandalf, lord of the rings, detailed, fantasy, cute, adorable, Pixar, Disney"
>>> images = pipe(prompt=prompt, negative_prompt=" ", image=init_image, strength=0.95).images[0]
>>> images.save("qwenimage_img2img.png")
```

**Parameters:**

transformer ([QwenImageTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/qwenimage_transformer2d#diffusers.QwenImageTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`Qwen2.5-VL-7B-Instruct`) : [Qwen2.5-VL-7B-Instruct](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct), specifically the [Qwen2.5-VL-7B-Instruct](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct) variant.

tokenizer (`Qwen2Tokenizer`) : Tokenizer of class https://huggingface.co/docs/transformers/en/model_doc/qwen2#transformers.Qwen2Tokenizer

**Returns:**

``~pipelines.qwenimage.QwenImagePipelineOutput` or `tuple``

`~pipelines.qwenimage.QwenImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When
returning a tuple, the first element is a list with the generated images.
#### disable_vae_slicing[[diffusers.QwenImageImg2ImgPipeline.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_img2img.py#L418)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### disable_vae_tiling[[diffusers.QwenImageImg2ImgPipeline.disable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_img2img.py#L445)

Disable tiled VAE decoding. If `enable_vae_tiling` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_vae_slicing[[diffusers.QwenImageImg2ImgPipeline.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_img2img.py#L405)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### enable_vae_tiling[[diffusers.QwenImageImg2ImgPipeline.enable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_img2img.py#L431)

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.
#### encode_prompt[[diffusers.QwenImageImg2ImgPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_img2img.py#L268)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

## QwenImageInpaintPipeline[[diffusers.QwenImageInpaintPipeline]]

#### diffusers.QwenImageInpaintPipeline[[diffusers.QwenImageInpaintPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_inpaint.py#L137)

The QwenImage pipeline for text-to-image generation.

__call__diffusers.QwenImageInpaintPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_inpaint.py#L645[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str | list[str] = None"}, {"name": "true_cfg_scale", "val": ": float = 4.0"}, {"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] = None"}, {"name": "mask_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] = None"}, {"name": "masked_image_latents", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "padding_mask_crop", "val": ": int | None = None"}, {"name": "strength", "val": ": float = 0.6"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "sigmas", "val": ": list[float] | None = None"}, {"name": "guidance_scale", "val": ": float | None = None"}, {"name": "num_images_per_prompt", "val": ": int = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds_mask", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds_mask", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `true_cfg_scale` is
  not greater than `1`).
- **image** (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) --
  `Image`, numpy array or tensor representing an image batch to be used as the starting point. For both
  numpy array and pytorch tensor, the expected value range is between `[0, 1]` If it's a tensor or a list
  or tensors, the expected shape should be `(B, C, H, W)` or `(C, H, W)`. If it is a numpy array or a
  list of arrays, the expected shape should be `(B, H, W, C)` or `(H, W, C)` It can also accept image
  latents as `image`, but if passing latents directly it is not encoded again.
- **true_cfg_scale** (`float`, *optional*, defaults to 1.0) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `true_cfg_scale` is defined as `w` of equation 2.
  of [Imagen Paper](https://huggingface.co/papers/2205.11487). Classifier-free guidance is enabled by
  setting `true_cfg_scale > 1` and a provided `negative_prompt`. Higher guidance scale encourages to
  generate images that are closely linked to the text `prompt`, usually at the expense of lower image
  quality.
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
- **padding_mask_crop** (`int`, *optional*, defaults to `None`) --
  The size of margin in the crop to be applied to the image and masking. If `None`, no crop is applied to
  image and mask_image. If `padding_mask_crop` is not `None`, it will first find a rectangular region
  with the same aspect ration of the image and contains all masked area, and then expand that area based
  on `padding_mask_crop`. The image and mask_image will then be cropped based on the expanded area before
  resizing to the original image size for inpainting. This is useful when the masked area is small while
  the image is large and contain information irrelevant for inpainting, such as background.
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
- **guidance_scale** (`float`, *optional*, defaults to None) --
  A guidance scale value for guidance distilled models. Unlike the traditional classifier-free guidance
  where the guidance scale is applied during inference through noise prediction rescaling, guidance
  distilled models take the guidance scale directly as an input parameter during forward pass. Guidance
  scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images
  that are closely linked to the text `prompt`, usually at the expense of lower image quality. This
  parameter in the pipeline is there to support future guidance-distilled models when they come up. It is
  ignored when not using guidance distilled models. To enable traditional classifier-free guidance,
  please pass `true_cfg_scale > 1.0` and `negative_prompt` (even an empty negative prompt like " " should
  enable classifier-free guidance computations).
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **prompt_embeds_mask** (`torch.Tensor`, *optional*) --
  Attention mask for `prompt_embeds`.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
  argument.
- **negative_prompt_embeds_mask** (`torch.Tensor`, *optional*) --
  Attention mask for `negative_prompt_embeds`.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.qwenimage.QwenImagePipelineOutput` instead of a plain tuple.
- **attention_kwargs** (`dict`, *optional*) --
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
- **max_sequence_length** (`int` defaults to 512) -- Maximum sequence length to use with the `prompt`.0`~pipelines.qwenimage.QwenImagePipelineOutput` or `tuple``~pipelines.qwenimage.QwenImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When
returning a tuple, the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import QwenImageInpaintPipeline
>>> from diffusers.utils import load_image

>>> pipe = QwenImageInpaintPipeline.from_pretrained("Qwen/Qwen-Image", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")
>>> prompt = "Face of a yellow cat, high resolution, sitting on a park bench"
>>> img_url = "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
>>> mask_url = "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"
>>> source = load_image(img_url)
>>> mask = load_image(mask_url)
>>> image = pipe(prompt=prompt, negative_prompt=" ", image=source, mask_image=mask, strength=0.85).images[0]
>>> image.save("qwenimage_inpainting.png")
```

**Parameters:**

transformer ([QwenImageTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/qwenimage_transformer2d#diffusers.QwenImageTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`Qwen2.5-VL-7B-Instruct`) : [Qwen2.5-VL-7B-Instruct](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct), specifically the [Qwen2.5-VL-7B-Instruct](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct) variant.

tokenizer (`Qwen2Tokenizer`) : Tokenizer of class https://huggingface.co/docs/transformers/en/model_doc/qwen2#transformers.Qwen2Tokenizer

**Returns:**

``~pipelines.qwenimage.QwenImagePipelineOutput` or `tuple``

`~pipelines.qwenimage.QwenImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When
returning a tuple, the first element is a list with the generated images.
#### disable_vae_slicing[[diffusers.QwenImageInpaintPipeline.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_inpaint.py#L445)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### disable_vae_tiling[[diffusers.QwenImageInpaintPipeline.disable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_inpaint.py#L472)

Disable tiled VAE decoding. If `enable_vae_tiling` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_vae_slicing[[diffusers.QwenImageInpaintPipeline.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_inpaint.py#L432)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### enable_vae_tiling[[diffusers.QwenImageInpaintPipeline.enable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_inpaint.py#L458)

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.
#### encode_prompt[[diffusers.QwenImageInpaintPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_inpaint.py#L279)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

## QwenImageEditPipeline[[diffusers.QwenImageEditPipeline]]

#### diffusers.QwenImageEditPipeline[[diffusers.QwenImageEditPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_edit.py#L165)

The Qwen-Image-Edit pipeline for image editing.

__call__diffusers.QwenImageEditPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_edit.py#L557[{"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str | list[str] = None"}, {"name": "true_cfg_scale", "val": ": float = 4.0"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "sigmas", "val": ": list[float] | None = None"}, {"name": "guidance_scale", "val": ": float | None = None"}, {"name": "num_images_per_prompt", "val": ": int = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds_mask", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds_mask", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}]- **image** (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) --
  `Image`, numpy array or tensor representing an image batch to be used as the starting point. For both
  numpy array and pytorch tensor, the expected value range is between `[0, 1]` If it's a tensor or a list
  or tensors, the expected shape should be `(B, C, H, W)` or `(C, H, W)`. If it is a numpy array or a
  list of arrays, the expected shape should be `(B, H, W, C)` or `(H, W, C)` It can also accept image
  latents as `image`, but if passing latents directly it is not encoded again.
- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `true_cfg_scale` is
  not greater than `1`).
- **true_cfg_scale** (`float`, *optional*, defaults to 1.0) --
  true_cfg_scale (`float`, *optional*, defaults to 1.0): Guidance scale as defined in [Classifier-Free
  Diffusion Guidance](https://huggingface.co/papers/2207.12598). `true_cfg_scale` is defined as `w` of
  equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Classifier-free guidance is
  enabled by setting `true_cfg_scale > 1` and a provided `negative_prompt`. Higher guidance scale
  encourages to generate images that are closely linked to the text `prompt`, usually at the expense of
  lower image quality.
- **height** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The height in pixels of the generated image. This is set to 1024 by default for the best results.
- **width** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The width in pixels of the generated image. This is set to 1024 by default for the best results.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **sigmas** (`list[float]`, *optional*) --
  Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in
  their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed
  will be used.
- **guidance_scale** (`float`, *optional*, defaults to None) --
  A guidance scale value for guidance distilled models. Unlike the traditional classifier-free guidance
  where the guidance scale is applied during inference through noise prediction rescaling, guidance
  distilled models take the guidance scale directly as an input parameter during forward pass. Guidance
  scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images
  that are closely linked to the text `prompt`, usually at the expense of lower image quality. This
  parameter in the pipeline is there to support future guidance-distilled models when they come up. It is
  ignored when not using guidance distilled models. To enable traditional classifier-free guidance,
  please pass `true_cfg_scale > 1.0` and `negative_prompt` (even an empty negative prompt like " " should
  enable classifier-free guidance computations).
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **prompt_embeds_mask** (`torch.Tensor`, *optional*) --
  Attention mask for `prompt_embeds`.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
  argument.
- **negative_prompt_embeds_mask** (`torch.Tensor`, *optional*) --
  Attention mask for `negative_prompt_embeds`.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.qwenimage.QwenImagePipelineOutput` instead of a plain tuple.
- **attention_kwargs** (`dict`, *optional*) --
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
- **max_sequence_length** (`int` defaults to 512) -- Maximum sequence length to use with the `prompt`.0`~pipelines.qwenimage.QwenImagePipelineOutput` or `tuple``~pipelines.qwenimage.QwenImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When
returning a tuple, the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from PIL import Image
>>> from diffusers import QwenImageEditPipeline
>>> from diffusers.utils import load_image

>>> pipe = QwenImageEditPipeline.from_pretrained("Qwen/Qwen-Image-Edit", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")
>>> image = load_image(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/yarn-art-pikachu.png"
... ).convert("RGB")
>>> prompt = (
...     "Make Pikachu hold a sign that says 'Qwen Edit is awesome', yarn art style, detailed, vibrant colors"
... )
>>> # Depending on the variant being used, the pipeline call will slightly vary.
>>> # Refer to the pipeline documentation for more details.
>>> image = pipe(image, prompt, num_inference_steps=50).images[0]
>>> image.save("qwenimage_edit.png")
```

**Parameters:**

transformer ([QwenImageTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/qwenimage_transformer2d#diffusers.QwenImageTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`Qwen2.5-VL-7B-Instruct`) : [Qwen2.5-VL-7B-Instruct](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct), specifically the [Qwen2.5-VL-7B-Instruct](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct) variant.

tokenizer (`Qwen2Tokenizer`) : Tokenizer of class https://huggingface.co/docs/transformers/en/model_doc/qwen2#transformers.Qwen2Tokenizer

**Returns:**

``~pipelines.qwenimage.QwenImagePipelineOutput` or `tuple``

`~pipelines.qwenimage.QwenImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When
returning a tuple, the first element is a list with the generated images.
#### disable_vae_slicing[[diffusers.QwenImageEditPipeline.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_edit.py#L442)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### disable_vae_tiling[[diffusers.QwenImageEditPipeline.disable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_edit.py#L469)

Disable tiled VAE decoding. If `enable_vae_tiling` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_vae_slicing[[diffusers.QwenImageEditPipeline.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_edit.py#L429)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### enable_vae_tiling[[diffusers.QwenImageEditPipeline.enable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_edit.py#L455)

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.
#### encode_prompt[[diffusers.QwenImageEditPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_edit.py#L272)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

image (`torch.Tensor`, *optional*) : image to be encoded

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

## QwenImageEditInpaintPipeline[[diffusers.QwenImageEditInpaintPipeline]]

#### diffusers.QwenImageEditInpaintPipeline[[diffusers.QwenImageEditInpaintPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_edit_inpaint.py#L167)

The Qwen-Image-Edit pipeline for image editing.

__call__diffusers.QwenImageEditInpaintPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_edit_inpaint.py#L690[{"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str | list[str] = None"}, {"name": "mask_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] = None"}, {"name": "masked_image_latents", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] = None"}, {"name": "true_cfg_scale", "val": ": float = 4.0"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "padding_mask_crop", "val": ": int | None = None"}, {"name": "strength", "val": ": float = 0.6"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "sigmas", "val": ": list[float] | None = None"}, {"name": "guidance_scale", "val": ": float | None = None"}, {"name": "num_images_per_prompt", "val": ": int = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds_mask", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds_mask", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}]- **image** (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) --
  `Image`, numpy array or tensor representing an image batch to be used as the starting point. For both
  numpy array and pytorch tensor, the expected value range is between `[0, 1]` If it's a tensor or a list
  or tensors, the expected shape should be `(B, C, H, W)` or `(C, H, W)`. If it is a numpy array or a
  list of arrays, the expected shape should be `(B, H, W, C)` or `(H, W, C)` It can also accept image
  latents as `image`, but if passing latents directly it is not encoded again.
- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `true_cfg_scale` is
  not greater than `1`).
- **true_cfg_scale** (`float`, *optional*, defaults to 1.0) --
  true_cfg_scale (`float`, *optional*, defaults to 1.0): Guidance scale as defined in [Classifier-Free
  Diffusion Guidance](https://huggingface.co/papers/2207.12598). `true_cfg_scale` is defined as `w` of
  equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Classifier-free guidance is
  enabled by setting `true_cfg_scale > 1` and a provided `negative_prompt`. Higher guidance scale
  encourages to generate images that are closely linked to the text `prompt`, usually at the expense of
  lower image quality.
- **mask_image** (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) --
  `Image`, numpy array or tensor representing an image batch to mask `image`. White pixels in the mask
  are repainted while black pixels are preserved. If `mask_image` is a PIL image, it is converted to a
  single channel (luminance) before use. If it's a numpy array or pytorch tensor, it should contain one
  color channel (L) instead of 3, so the expected shape for pytorch tensor would be `(B, 1, H, W)`, `(B,
  H, W)`, `(1, H, W)`, `(H, W)`. And for numpy array would be for `(B, H, W, 1)`, `(B, H, W)`, `(H, W,
  1)`, or `(H, W)`.
- **masked_image_latents** (`torch.Tensor`, `list[torch.Tensor]`) --
  `Tensor` representing an image batch to mask `image` generated by VAE. If not provided, the mask
  latents tensor will ge generated by `mask_image`.
- **height** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The height in pixels of the generated image. This is set to 1024 by default for the best results.
- **width** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The width in pixels of the generated image. This is set to 1024 by default for the best results.
- **padding_mask_crop** (`int`, *optional*, defaults to `None`) --
  The size of margin in the crop to be applied to the image and masking. If `None`, no crop is applied to
  image and mask_image. If `padding_mask_crop` is not `None`, it will first find a rectangular region
  with the same aspect ration of the image and contains all masked area, and then expand that area based
  on `padding_mask_crop`. The image and mask_image will then be cropped based on the expanded area before
  resizing to the original image size for inpainting. This is useful when the masked area is small while
  the image is large and contain information irrelevant for inpainting, such as background.
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
- **guidance_scale** (`float`, *optional*, defaults to None) --
  A guidance scale value for guidance distilled models. Unlike the traditional classifier-free guidance
  where the guidance scale is applied during inference through noise prediction rescaling, guidance
  distilled models take the guidance scale directly as an input parameter during forward pass. Guidance
  scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images
  that are closely linked to the text `prompt`, usually at the expense of lower image quality. This
  parameter in the pipeline is there to support future guidance-distilled models when they come up. It is
  ignored when not using guidance distilled models. To enable traditional classifier-free guidance,
  please pass `true_cfg_scale > 1.0` and `negative_prompt` (even an empty negative prompt like " " should
  enable classifier-free guidance computations).
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **prompt_embeds_mask** (`torch.Tensor`, *optional*) --
  Attention mask for `prompt_embeds`.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
  argument.
- **negative_prompt_embeds_mask** (`torch.Tensor`, *optional*) --
  Attention mask for `negative_prompt_embeds`.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.qwenimage.QwenImagePipelineOutput` instead of a plain tuple.
- **attention_kwargs** (`dict`, *optional*) --
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
- **max_sequence_length** (`int` defaults to 512) -- Maximum sequence length to use with the `prompt`.0`~pipelines.qwenimage.QwenImagePipelineOutput` or `tuple``~pipelines.qwenimage.QwenImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When
returning a tuple, the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from PIL import Image
>>> from diffusers import QwenImageEditInpaintPipeline
>>> from diffusers.utils import load_image

>>> pipe = QwenImageEditInpaintPipeline.from_pretrained("Qwen/Qwen-Image-Edit", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")
>>> prompt = "Face of a yellow cat, high resolution, sitting on a park bench"

>>> img_url = "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
>>> mask_url = "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"
>>> source = load_image(img_url)
>>> mask = load_image(mask_url)
>>> image = pipe(
...     prompt=prompt, negative_prompt=" ", image=source, mask_image=mask, strength=1.0, num_inference_steps=50
... ).images[0]
>>> image.save("qwenimage_inpainting.png")
```

**Parameters:**

transformer ([QwenImageTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/qwenimage_transformer2d#diffusers.QwenImageTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`Qwen2.5-VL-7B-Instruct`) : [Qwen2.5-VL-7B-Instruct](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct), specifically the [Qwen2.5-VL-7B-Instruct](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct) variant.

tokenizer (`Qwen2Tokenizer`) : Tokenizer of class https://huggingface.co/docs/transformers/en/model_doc/qwen2#transformers.Qwen2Tokenizer

**Returns:**

``~pipelines.qwenimage.QwenImagePipelineOutput` or `tuple``

`~pipelines.qwenimage.QwenImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When
returning a tuple, the first element is a list with the generated images.
#### disable_vae_slicing[[diffusers.QwenImageEditInpaintPipeline.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_edit_inpaint.py#L488)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### disable_vae_tiling[[diffusers.QwenImageEditInpaintPipeline.disable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_edit_inpaint.py#L515)

Disable tiled VAE decoding. If `enable_vae_tiling` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_vae_slicing[[diffusers.QwenImageEditInpaintPipeline.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_edit_inpaint.py#L475)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### enable_vae_tiling[[diffusers.QwenImageEditInpaintPipeline.enable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_edit_inpaint.py#L501)

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.
#### encode_prompt[[diffusers.QwenImageEditInpaintPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_edit_inpaint.py#L284)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

image (`torch.Tensor`, *optional*) : image to be encoded

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

## QwenImageControlNetPipeline[[diffusers.QwenImageControlNetPipeline]]

#### diffusers.QwenImageControlNetPipeline[[diffusers.QwenImageControlNetPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_controlnet.py#L192)

The QwenImage pipeline for text-to-image generation.

__call__diffusers.QwenImageControlNetPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_controlnet.py#L564[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str | list[str] = None"}, {"name": "true_cfg_scale", "val": ": float = 4.0"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "sigmas", "val": ": list[float] | None = None"}, {"name": "guidance_scale", "val": ": float | None = None"}, {"name": "control_guidance_start", "val": ": float | list[float] = 0.0"}, {"name": "control_guidance_end", "val": ": float | list[float] = 1.0"}, {"name": "control_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] = None"}, {"name": "controlnet_conditioning_scale", "val": ": float | list[float] = 1.0"}, {"name": "num_images_per_prompt", "val": ": int = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds_mask", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds_mask", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `true_cfg_scale` is
  not greater than `1`).
- **true_cfg_scale** (`float`, *optional*, defaults to 1.0) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `true_cfg_scale` is defined as `w` of equation 2.
  of [Imagen Paper](https://huggingface.co/papers/2205.11487). Classifier-free guidance is enabled by
  setting `true_cfg_scale > 1` and a provided `negative_prompt`. Higher guidance scale encourages to
  generate images that are closely linked to the text `prompt`, usually at the expense of lower image
  quality.
- **height** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The height in pixels of the generated image. This is set to 1024 by default for the best results.
- **width** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The width in pixels of the generated image. This is set to 1024 by default for the best results.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **sigmas** (`list[float]`, *optional*) --
  Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in
  their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed
  will be used.
- **guidance_scale** (`float`, *optional*, defaults to None) --
  A guidance scale value for guidance distilled models. Unlike the traditional classifier-free guidance
  where the guidance scale is applied during inference through noise prediction rescaling, guidance
  distilled models take the guidance scale directly as an input parameter during forward pass. Guidance
  scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images
  that are closely linked to the text `prompt`, usually at the expense of lower image quality. This
  parameter in the pipeline is there to support future guidance-distilled models when they come up. It is
  ignored when not using guidance distilled models. To enable traditional classifier-free guidance,
  please pass `true_cfg_scale > 1.0` and `negative_prompt` (even an empty negative prompt like " " should
  enable classifier-free guidance computations).
- **control_guidance_start** (`float` or `list[float]`, *optional*, defaults to 0.0) --
  The percentage of total steps at which the ControlNet starts applying.
- **control_guidance_end** (`float` or `list[float]`, *optional*, defaults to 1.0) --
  The percentage of total steps at which the ControlNet stops applying.
- **control_image** (`PipelineImageInput`, *optional*) --
  The ControlNet input condition to provide guidance for the generation.
- **controlnet_conditioning_scale** (`float` or `list[float]`, *optional*, defaults to 1.0) --
  The outputs of the ControlNet are multiplied by `controlnet_conditioning_scale` before they are added
  to the residual in the original `transformer`.
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **prompt_embeds_mask** (`torch.Tensor`, *optional*) --
  Attention mask for `prompt_embeds`.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
  argument.
- **negative_prompt_embeds_mask** (`torch.Tensor`, *optional*) --
  Attention mask for `negative_prompt_embeds`.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.qwenimage.QwenImagePipelineOutput` instead of a plain tuple.
- **attention_kwargs** (`dict`, *optional*) --
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
- **max_sequence_length** (`int` defaults to 512) -- Maximum sequence length to use with the `prompt`.0`~pipelines.qwenimage.QwenImagePipelineOutput` or `tuple``~pipelines.qwenimage.QwenImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When
returning a tuple, the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers.utils import load_image
>>> from diffusers import QwenImageControlNetModel, QwenImageMultiControlNetModel, QwenImageControlNetPipeline

>>> # QwenImageControlNetModel
>>> controlnet = QwenImageControlNetModel.from_pretrained(
...     "InstantX/Qwen-Image-ControlNet-Union", torch_dtype=torch.bfloat16
... )
>>> pipe = QwenImageControlNetPipeline.from_pretrained(
...     "Qwen/Qwen-Image", controlnet=controlnet, torch_dtype=torch.bfloat16
... )
>>> pipe.to("cuda")
>>> prompt = "Aesthetics art, traditional asian pagoda, elaborate golden accents, sky blue and white color palette, swirling cloud pattern, digital illustration, east asian architecture, ornamental rooftop, intricate detailing on building, cultural representation."
>>> negative_prompt = " "
>>> control_image = load_image(
...     "https://huggingface.co/InstantX/Qwen-Image-ControlNet-Union/resolve/main/conds/canny.png"
... )
>>> # Depending on the variant being used, the pipeline call will slightly vary.
>>> # Refer to the pipeline documentation for more details.
>>> image = pipe(
...     prompt,
...     negative_prompt=negative_prompt,
...     control_image=control_image,
...     controlnet_conditioning_scale=1.0,
...     num_inference_steps=30,
...     true_cfg_scale=4.0,
... ).images[0]
>>> image.save("qwenimage_cn_union.png")

>>> # QwenImageMultiControlNetModel
>>> controlnet = QwenImageControlNetModel.from_pretrained(
...     "InstantX/Qwen-Image-ControlNet-Union", torch_dtype=torch.bfloat16
... )
>>> controlnet = QwenImageMultiControlNetModel([controlnet])
>>> pipe = QwenImageControlNetPipeline.from_pretrained(
...     "Qwen/Qwen-Image", controlnet=controlnet, torch_dtype=torch.bfloat16
... )
>>> pipe.to("cuda")
>>> prompt = "Aesthetics art, traditional asian pagoda, elaborate golden accents, sky blue and white color palette, swirling cloud pattern, digital illustration, east asian architecture, ornamental rooftop, intricate detailing on building, cultural representation."
>>> negative_prompt = " "
>>> control_image = load_image(
...     "https://huggingface.co/InstantX/Qwen-Image-ControlNet-Union/resolve/main/conds/canny.png"
... )
>>> # Depending on the variant being used, the pipeline call will slightly vary.
>>> # Refer to the pipeline documentation for more details.
>>> image = pipe(
...     prompt,
...     negative_prompt=negative_prompt,
...     control_image=[control_image, control_image],
...     controlnet_conditioning_scale=[0.5, 0.5],
...     num_inference_steps=30,
...     true_cfg_scale=4.0,
... ).images[0]
>>> image.save("qwenimage_cn_union_multi.png")
```

**Parameters:**

transformer ([QwenImageTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/qwenimage_transformer2d#diffusers.QwenImageTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`Qwen2.5-VL-7B-Instruct`) : [Qwen2.5-VL-7B-Instruct](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct), specifically the [Qwen2.5-VL-7B-Instruct](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct) variant.

tokenizer (`Qwen2Tokenizer`) : Tokenizer of class https://huggingface.co/docs/transformers/en/model_doc/qwen2#transformers.Qwen2Tokenizer

**Returns:**

``~pipelines.qwenimage.QwenImagePipelineOutput` or `tuple``

`~pipelines.qwenimage.QwenImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When
returning a tuple, the first element is a list with the generated images.
#### disable_vae_slicing[[diffusers.QwenImageControlNetPipeline.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_controlnet.py#L436)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### disable_vae_tiling[[diffusers.QwenImageControlNetPipeline.disable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_controlnet.py#L463)

Disable tiled VAE decoding. If `enable_vae_tiling` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_vae_slicing[[diffusers.QwenImageControlNetPipeline.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_controlnet.py#L423)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### enable_vae_tiling[[diffusers.QwenImageControlNetPipeline.enable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_controlnet.py#L449)

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.
#### encode_prompt[[diffusers.QwenImageControlNetPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_controlnet.py#L290)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

## QwenImageEditPlusPipeline[[diffusers.QwenImageEditPlusPipeline]]

#### diffusers.QwenImageEditPlusPipeline[[diffusers.QwenImageEditPlusPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_edit_plus.py#L168)

The Qwen-Image-Edit pipeline for image editing.

__call__diffusers.QwenImageEditPlusPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_edit_plus.py#L526[{"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str | list[str] = None"}, {"name": "true_cfg_scale", "val": ": float = 4.0"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "sigmas", "val": ": list[float] | None = None"}, {"name": "guidance_scale", "val": ": float | None = None"}, {"name": "num_images_per_prompt", "val": ": int = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds_mask", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds_mask", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}]- **image** (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) --
  `Image`, numpy array or tensor representing an image batch to be used as the starting point. For both
  numpy array and pytorch tensor, the expected value range is between `[0, 1]` If it's a tensor or a list
  or tensors, the expected shape should be `(B, C, H, W)` or `(C, H, W)`. If it is a numpy array or a
  list of arrays, the expected shape should be `(B, H, W, C)` or `(H, W, C)` It can also accept image
  latents as `image`, but if passing latents directly it is not encoded again.
- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `true_cfg_scale` is
  not greater than `1`).
- **true_cfg_scale** (`float`, *optional*, defaults to 1.0) --
  true_cfg_scale (`float`, *optional*, defaults to 1.0): Guidance scale as defined in [Classifier-Free
  Diffusion Guidance](https://huggingface.co/papers/2207.12598). `true_cfg_scale` is defined as `w` of
  equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Classifier-free guidance is
  enabled by setting `true_cfg_scale > 1` and a provided `negative_prompt`. Higher guidance scale
  encourages to generate images that are closely linked to the text `prompt`, usually at the expense of
  lower image quality.
- **height** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The height in pixels of the generated image. This is set to 1024 by default for the best results.
- **width** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The width in pixels of the generated image. This is set to 1024 by default for the best results.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **sigmas** (`list[float]`, *optional*) --
  Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in
  their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed
  will be used.
- **guidance_scale** (`float`, *optional*, defaults to None) --
  A guidance scale value for guidance distilled models. Unlike the traditional classifier-free guidance
  where the guidance scale is applied during inference through noise prediction rescaling, guidance
  distilled models take the guidance scale directly as an input parameter during forward pass. Guidance
  scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images
  that are closely linked to the text `prompt`, usually at the expense of lower image quality. This
  parameter in the pipeline is there to support future guidance-distilled models when they come up. It is
  ignored when not using guidance distilled models. To enable traditional classifier-free guidance,
  please pass `true_cfg_scale > 1.0` and `negative_prompt` (even an empty negative prompt like " " should
  enable classifier-free guidance computations).
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **prompt_embeds_mask** (`torch.Tensor`, *optional*) --
  Attention mask for `prompt_embeds`.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
  argument.
- **negative_prompt_embeds_mask** (`torch.Tensor`, *optional*) --
  Attention mask for `negative_prompt_embeds`.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.qwenimage.QwenImagePipelineOutput` instead of a plain tuple.
- **attention_kwargs** (`dict`, *optional*) --
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
- **max_sequence_length** (`int` defaults to 512) -- Maximum sequence length to use with the `prompt`.0`~pipelines.qwenimage.QwenImagePipelineOutput` or `tuple``~pipelines.qwenimage.QwenImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When
returning a tuple, the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from PIL import Image
>>> from diffusers import QwenImageEditPlusPipeline
>>> from diffusers.utils import load_image

>>> pipe = QwenImageEditPlusPipeline.from_pretrained("Qwen/Qwen-Image-Edit-2509", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")
>>> image = load_image(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/yarn-art-pikachu.png"
... ).convert("RGB")
>>> prompt = (
...     "Make Pikachu hold a sign that says 'Qwen Edit is awesome', yarn art style, detailed, vibrant colors"
... )
>>> # Depending on the variant being used, the pipeline call will slightly vary.
>>> # Refer to the pipeline documentation for more details.
>>> image = pipe(image, prompt, num_inference_steps=50).images[0]
>>> image.save("qwenimage_edit_plus.png")
```

**Parameters:**

transformer ([QwenImageTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/qwenimage_transformer2d#diffusers.QwenImageTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`Qwen2.5-VL-7B-Instruct`) : [Qwen2.5-VL-7B-Instruct](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct), specifically the [Qwen2.5-VL-7B-Instruct](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct) variant.

tokenizer (`Qwen2Tokenizer`) : Tokenizer of class https://huggingface.co/docs/transformers/en/model_doc/qwen2#transformers.Qwen2Tokenizer

**Returns:**

``~pipelines.qwenimage.QwenImagePipelineOutput` or `tuple``

`~pipelines.qwenimage.QwenImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When
returning a tuple, the first element is a list with the generated images.
#### encode_prompt[[diffusers.QwenImageEditPlusPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_edit_plus.py#L286)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

image (`torch.Tensor`, *optional*) : image to be encoded

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

## QwenImageLayeredPipeline[[diffusers.QwenImageLayeredPipeline]]

#### diffusers.QwenImageLayeredPipeline[[diffusers.QwenImageLayeredPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_layered.py#L174)

The Qwen-Image-Layered pipeline for image decomposing.

__call__diffusers.QwenImageLayeredPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_layered.py#L537[{"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str | list[str] = None"}, {"name": "true_cfg_scale", "val": ": float = 4.0"}, {"name": "layers", "val": ": int | None = 4"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "sigmas", "val": ": list[float] | None = None"}, {"name": "guidance_scale", "val": ": float | None = None"}, {"name": "num_images_per_prompt", "val": ": int = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds_mask", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds_mask", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 512"}, {"name": "resolution", "val": ": int = 640"}, {"name": "cfg_normalize", "val": ": bool = False"}, {"name": "use_en_prompt", "val": ": bool = False"}]- **image** (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) --
  `Image`, numpy array or tensor representing an image batch to be used as the starting point. For both
  numpy array and pytorch tensor, the expected value range is between `[0, 1]` If it's a tensor or a list
  or tensors, the expected shape should be `(B, C, H, W)` or `(C, H, W)`. If it is a numpy array or a
  list of arrays, the expected shape should be `(B, H, W, C)` or `(H, W, C)` It can also accept image
  latents as `image`, but if passing latents directly it is not encoded again.
- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `true_cfg_scale` is
  not greater than `1`).
- **true_cfg_scale** (`float`, *optional*, defaults to 1.0) --
  true_cfg_scale (`float`, *optional*, defaults to 1.0): Guidance scale as defined in [Classifier-Free
  Diffusion Guidance](https://huggingface.co/papers/2207.12598). `true_cfg_scale` is defined as `w` of
  equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Classifier-free guidance is
  enabled by setting `true_cfg_scale > 1` and a provided `negative_prompt`. Higher guidance scale
  encourages to generate images that are closely linked to the text `prompt`, usually at the expense of
  lower image quality.
- **layers** (`int`, *optional*, defaults to 4) --
  Number of latent layers to generate for the layered output.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **sigmas** (`list[float]`, *optional*) --
  Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in
  their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed
  will be used.
- **guidance_scale** (`float`, *optional*, defaults to None) --
  A guidance scale value for guidance distilled models. Unlike the traditional classifier-free guidance
  where the guidance scale is applied during inference through noise prediction rescaling, guidance
  distilled models take the guidance scale directly as an input parameter during forward pass. Guidance
  scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images
  that are closely linked to the text `prompt`, usually at the expense of lower image quality. This
  parameter in the pipeline is there to support future guidance-distilled models when they come up. It is
  ignored when not using guidance distilled models. To enable traditional classifier-free guidance,
  please pass `true_cfg_scale > 1.0` and `negative_prompt` (even an empty negative prompt like " " should
  enable classifier-free guidance computations).
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **prompt_embeds_mask** (`torch.Tensor`, *optional*) --
  Attention mask for `prompt_embeds`.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
  argument.
- **negative_prompt_embeds_mask** (`torch.Tensor`, *optional*) --
  Attention mask for `negative_prompt_embeds`.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.qwenimage.QwenImagePipelineOutput` instead of a plain tuple.
- **attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`List`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.
- **max_sequence_length** (`int` defaults to 512) -- Maximum sequence length to use with the `prompt`.
- **resolution** (`int`, *optional*, defaults to 640) --
  using different bucket in (640, 1024) to determin the condition and output resolution
- **cfg_normalize** (`bool`, *optional*, defaults to `False`) --
  whether enable cfg normalization.
- **use_en_prompt** (`bool`, *optional*, defaults to `False`) --
  automatic caption language if user does not provide caption0`~pipelines.qwenimage.QwenImagePipelineOutput` or `tuple``~pipelines.qwenimage.QwenImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When
returning a tuple, the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from PIL import Image
>>> from diffusers import QwenImageLayeredPipeline
>>> from diffusers.utils import load_image

>>> pipe = QwenImageLayeredPipeline.from_pretrained("Qwen/Qwen-Image-Layered", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")
>>> image = load_image(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/yarn-art-pikachu.png"
... ).convert("RGBA")
>>> prompt = ""
>>> # Depending on the variant being used, the pipeline call will slightly vary.
>>> # Refer to the pipeline documentation for more details.
>>> images = pipe(
...     image,
...     prompt,
...     num_inference_steps=50,
...     true_cfg_scale=4.0,
...     layers=4,
...     resolution=640,
...     cfg_normalize=False,
...     use_en_prompt=True,
... ).images[0]
>>> for i, image in enumerate(images):
...     image.save(f"{i}.out.png")
```

**Parameters:**

transformer ([QwenImageTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/qwenimage_transformer2d#diffusers.QwenImageTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`Qwen2.5-VL-7B-Instruct`) : [Qwen2.5-VL-7B-Instruct](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct), specifically the [Qwen2.5-VL-7B-Instruct](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct) variant.

tokenizer (`Qwen2Tokenizer`) : Tokenizer of class https://huggingface.co/docs/transformers/en/model_doc/qwen2#transformers.Qwen2Tokenizer

**Returns:**

``~pipelines.qwenimage.QwenImagePipelineOutput` or `tuple``

`~pipelines.qwenimage.QwenImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When
returning a tuple, the first element is a list with the generated images.
#### encode_prompt[[diffusers.QwenImageLayeredPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_qwenimage_layered.py#L291)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

## QwenImagePipelineOutput[[diffusers.pipelines.qwenimage.pipeline_output.QwenImagePipelineOutput]]

#### diffusers.pipelines.qwenimage.pipeline_output.QwenImagePipelineOutput[[diffusers.pipelines.qwenimage.pipeline_output.QwenImagePipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/qwenimage/pipeline_output.py#L10)

Output class for Stable Diffusion pipelines.

**Parameters:**

images (`list[PIL.Image.Image]` or `np.ndarray`) : list of denoised PIL images of length `batch_size` or numpy array of shape `(batch_size, height, width, num_channels)`. PIL images or numpy array present the denoised images of the diffusion pipeline.
