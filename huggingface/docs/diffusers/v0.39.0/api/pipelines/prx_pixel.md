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

# PRX Pixel

PRXPixel is a pixel-space text-to-image generation model by Photoroom. A ~7B `PRXTransformer2DModel`
denoises raw RGB images directly — no VAE is needed. The model is conditioned on a Qwen3-VL text encoder
and uses flow matching where the transformer predicts the clean image at each step (x-prediction). The
generation resolution is fed into the timestep modulation so the model is aware of the target size.

## Available models

| Model | Resolution | Description | Suggested parameters | Recommended dtype |
|:-----:|:---------:|:----------:|:----------:|:----------:|
| [`Photoroom/prxpixel-t2i`](https://huggingface.co/Photoroom/prxpixel-t2i) | 1024 | Pixel-space ~7B model with Qwen3-VL text encoder | 28 steps, cfg=5.0 | `torch.bfloat16` |

## Loading the pipeline

[PRXPixelPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/prx_pixel#diffusers.PRXPixelPipeline) requires `transformers >= 4.57` (the version that introduced `Qwen3VLTextModel`). Load it with [from_pretrained()](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline.from_pretrained):

```py
import torch
from diffusers import PRXPixelPipeline

pipe = PRXPixelPipeline.from_pretrained("Photoroom/prxpixel-t2i", torch_dtype=torch.bfloat16)
pipe.to("cuda")

prompt = "A front-facing portrait of a lion in the golden savanna at sunset."
image = pipe(prompt, num_inference_steps=28, guidance_scale=5.0).images[0]
image.save("prxpixel_output.png")
```

## Memory Optimization

For memory-constrained environments:

```py
import torch
from diffusers import PRXPixelPipeline

pipe = PRXPixelPipeline.from_pretrained("Photoroom/prxpixel-t2i", torch_dtype=torch.bfloat16)
pipe.enable_model_cpu_offload()

# Or use sequential CPU offload for even lower memory
pipe.enable_sequential_cpu_offload()
```

## PRXPixelPipeline[[diffusers.PRXPixelPipeline]]

#### diffusers.PRXPixelPipeline[[diffusers.PRXPixelPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/prx/pipeline_prx_pixel.py#L98)

Pipeline for text-to-image generation with the PRXPixel model.

PRXPixel is a standalone, pixel-space text-to-image pipeline. It denoises raw RGB directly with a ~7B-parameter
`PRXTransformer2DModel` and has no VAE (generation happens entirely in pixel space, so the denoised output *is*
the image). Prompts are encoded with a Qwen3-VL text encoder (the vision tower is discarded). Unlike
[PRXPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/prx#diffusers.PRXPipeline) the transformer is trained with x-prediction: at every step it predicts the clean image `x0`, which
is converted to a flow-matching velocity before the scheduler step. Sampling starts from `randn * noise_scale`
(`noise_scale=2.0` by default) and the default resolution is 1024px.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

Examples:
```py
>>> import torch
>>> from diffusers import PRXPixelPipeline

>>> pipe = PRXPixelPipeline.from_pretrained("Photoroom/prxpixel-t2i", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> prompt = "A front-facing portrait of a lion in the golden savanna at sunset."
>>> image = pipe(prompt, num_inference_steps=28, guidance_scale=5.0).images[0]
>>> image.save("prxpixel_output.png")
```

__call__diffusers.PRXPixelPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/prx/pipeline_prx_pixel.py#L406[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str = ''"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 28"}, {"name": "timesteps", "val": ": list = None"}, {"name": "guidance_scale", "val": ": float = 4.0"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.FloatTensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.FloatTensor | None = None"}, {"name": "prompt_attention_mask", "val": ": torch.BoolTensor | None = None"}, {"name": "negative_prompt_attention_mask", "val": ": torch.BoolTensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "use_resolution_binning", "val": ": bool = True"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`
  instead.
- **negative_prompt** (`str`, *optional*, defaults to `""`) --
  The prompt or prompts not to guide the image generation. Ignored when not using guidance (i.e., ignored
  if `guidance_scale` is less than `1`).
- **height** (`int`, *optional*, defaults to `default_sample_size`) --
  The height in pixels of the generated image.
- **width** (`int`, *optional*, defaults to `default_sample_size`) --
  The width in pixels of the generated image.
- **num_inference_steps** (`int`, *optional*, defaults to 28) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **timesteps** (`list[int]`, *optional*) --
  Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument
  in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is
  passed will be used. Must be in descending order.
- **guidance_scale** (`float`, *optional*, defaults to 4.0) --
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
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **negative_prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided and `guidance_scale > 1`, negative embeddings will be generated from an
  empty string.
- **prompt_attention_mask** (`torch.BoolTensor`, *optional*) --
  Pre-generated attention mask for `prompt_embeds`. If not provided, attention mask will be generated
  from `prompt` input argument.
- **negative_prompt_attention_mask** (`torch.BoolTensor`, *optional*) --
  Pre-generated attention mask for `negative_prompt_embeds`. If not provided and `guidance_scale > 1`,
  attention mask will be generated from an empty string.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a [PRXPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/prx#diffusers.pipelines.prx.PRXPipelineOutput) instead of a plain tuple.
- **use_resolution_binning** (`bool`, *optional*, defaults to `True`) --
  If set to `True`, the requested height and width are first mapped to the closest resolutions using
  predefined aspect ratio bins. After the produced latents are decoded into images, they are resized back
  to the requested resolution. Useful for generating non-square images at optimal resolutions.
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `callback_on_step_end(self, step, timestep, callback_kwargs)`.
  `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`list`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include tensors that are listed
  in the `._callback_tensor_inputs` attribute.0[PRXPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/prx#diffusers.pipelines.prx.PRXPipelineOutput) or `tuple`[PRXPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/prx#diffusers.pipelines.prx.PRXPipelineOutput) if `return_dict` is
True, otherwise a `tuple. When returning a tuple, the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import PRXPixelPipeline

>>> pipe = PRXPixelPipeline.from_pretrained("Photoroom/prxpixel-t2i", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> prompt = "A front-facing portrait of a lion in the golden savanna at sunset."
>>> image = pipe(prompt, num_inference_steps=28, guidance_scale=5.0).images[0]
>>> image.save("prxpixel_output.png")
```

**Parameters:**

transformer (`PRXTransformer2DModel`) : The ~7B-parameter PRX denoiser. For PRXPixel this is built with `in_channels=3`, a bottleneck `img_in`, and `resolution_embeds=True`, and it is trained to predict the clean image `x0`.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : Flow-matching scheduler used to denoise the (pixel-space) latents.

text_encoder (`PreTrainedModel`) : The Qwen3-VL text backbone used to encode prompts (the vision tower is discarded). Must return a `last_hidden_state`.

tokenizer (`PreTrainedTokenizerBase`) : Tokenizer for `text_encoder` (typically loaded via `AutoTokenizer`).

default_sample_size (`int`, *optional*, defaults to 1024) : Default height/width used when none is provided to `__call__`.

prompt_max_tokens (`int`, *optional*, defaults to 256) : Number of text tokens the prompt is padded/truncated to before encoding.

noise_scale (`float`, *optional*, defaults to 2.0) : Scale applied to the initial Gaussian noise. PRXPixel trains with a non-unit initial-noise scale, so sampling must start from `randn * noise_scale`.

**Returns:**

`[PRXPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/prx#diffusers.pipelines.prx.PRXPipelineOutput) or `tuple``

[PRXPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/prx#diffusers.pipelines.prx.PRXPipelineOutput) if `return_dict` is
True, otherwise a `tuple. When returning a tuple, the first element is a list with the generated images.
#### check_inputs[[diffusers.PRXPixelPipeline.check_inputs]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/prx/pipeline_prx_pixel.py#L324)

Check that all inputs are in correct format.
#### encode_prompt[[diffusers.PRXPixelPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/prx/pipeline_prx_pixel.py#L263)

Encode text prompt using standard text encoder and tokenizer, or use precomputed embeddings.
#### prepare_latents[[diffusers.PRXPixelPipeline.prepare_latents]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/prx/pipeline_prx_pixel.py#L378)

Prepare initial latents for the diffusion process.

PRXPixel trains with a non-unit initial-noise scale, so the sampled noise is multiplied by
`self.config.noise_scale`.

## PRXPipelineOutput[[diffusers.pipelines.prx.PRXPipelineOutput]]

#### diffusers.pipelines.prx.PRXPipelineOutput[[diffusers.pipelines.prx.PRXPipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/prx/pipeline_output.py#L24)

Output class for PRX pipelines.

**Parameters:**

images (`list[PIL.Image.Image]` or `np.ndarray`) : list of denoised PIL images of length `batch_size` or numpy array of shape `(batch_size, height, width, num_channels)`. PIL images or numpy array present the denoised images of the diffusion pipeline.
