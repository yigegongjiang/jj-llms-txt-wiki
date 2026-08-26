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

# HunyuanImage2.1

HunyuanImage-2.1 is a 17B text-to-image model that is capable of generating 2K (2048 x 2048) resolution images

HunyuanImage-2.1 comes in the following variants:

| model type | model id |
|:----------:|:--------:|
| HunyuanImage-2.1 | [hunyuanvideo-community/HunyuanImage-2.1-Diffusers](https://huggingface.co/hunyuanvideo-community/HunyuanImage-2.1-Diffusers) |
| HunyuanImage-2.1-Distilled | [hunyuanvideo-community/HunyuanImage-2.1-Distilled-Diffusers](https://huggingface.co/hunyuanvideo-community/HunyuanImage-2.1-Distilled-Diffusers) |
| HunyuanImage-2.1-Refiner | [hunyuanvideo-community/HunyuanImage-2.1-Refiner-Diffusers](https://huggingface.co/hunyuanvideo-community/HunyuanImage-2.1-Refiner-Diffusers) |

> [!TIP]
> [Caching](../../optimization/cache) may also speed up inference by storing and reusing intermediate outputs.

## HunyuanImage-2.1

HunyuanImage-2.1 applies [Adaptive Projected Guidance (APG)](https://huggingface.co/papers/2410.02416) combined with Classifier-Free Guidance (CFG) in the denoising loop. `HunyuanImagePipeline` has a `guider` component (read more about [Guider](../../using-diffusers/guiders)) and does not take a `guidance_scale` parameter at runtime. To change guider-related parameters, e.g., `guidance_scale`, you can update the `guider` configuration instead.

```python
import torch
from diffusers import HunyuanImagePipeline

pipe = HunyuanImagePipeline.from_pretrained(
    "hunyuanvideo-community/HunyuanImage-2.1-Diffusers", 
    dtype=torch.bfloat16
)
pipe = pipe.to("cuda")
``` 

You can inspect the `guider` object:

```py
>>> pipe.guider
AdaptiveProjectedMixGuidance {
  "_class_name": "AdaptiveProjectedMixGuidance",
  "_diffusers_version": "0.36.0.dev0",
  "adaptive_projected_guidance_momentum": -0.5,
  "adaptive_projected_guidance_rescale": 10.0,
  "adaptive_projected_guidance_scale": 10.0,
  "adaptive_projected_guidance_start_step": 5,
  "enabled": true,
  "eta": 0.0,
  "guidance_rescale": 0.0,
  "guidance_scale": 3.5,
  "start": 0.0,
  "stop": 1.0,
  "use_original_formulation": false
}

State:
  step: None
  num_inference_steps: None
  timestep: None
  count_prepared: 0
  enabled: True
  num_conditions: 2
  momentum_buffer: None
  is_apg_enabled: False
  is_cfg_enabled: True
```

To update the guider with a different configuration, use the `new()` method. For example, to generate an image with `guidance_scale=5.0` while keeping all other default guidance parameters:

```py
import torch
from diffusers import HunyuanImagePipeline

pipe = HunyuanImagePipeline.from_pretrained(
    "hunyuanvideo-community/HunyuanImage-2.1-Diffusers", 
    dtype=torch.bfloat16
)
pipe = pipe.to("cuda")

# Update the guider configuration
pipe.guider = pipe.guider.new(guidance_scale=5.0)

prompt = (
    "A cute, cartoon-style anthropomorphic penguin plush toy with fluffy fur, standing in a painting studio, "
    "wearing a red knitted scarf and a red beret with the word 'Tencent' on it, holding a paintbrush with a "
    "focused expression as it paints an oil painting of the Mona Lisa, rendered in a photorealistic photographic style."
)

image = pipe(
    prompt=prompt, 
    num_inference_steps=50, 
    height=2048, 
    width=2048,
).images[0]
image.save("image.png")
```

## HunyuanImage-2.1-Distilled

use `distilled_guidance_scale` with the guidance-distilled checkpoint, 

```py
import torch
from diffusers import HunyuanImagePipeline
pipe = HunyuanImagePipeline.from_pretrained("hunyuanvideo-community/HunyuanImage-2.1-Distilled-Diffusers", dtype=torch.bfloat16)
pipe = pipe.to("cuda")

prompt = (
    "A cute, cartoon-style anthropomorphic penguin plush toy with fluffy fur, standing in a painting studio, "
    "wearing a red knitted scarf and a red beret with the word 'Tencent' on it, holding a paintbrush with a "
    "focused expression as it paints an oil painting of the Mona Lisa, rendered in a photorealistic photographic style."
)

out = pipe(
    prompt,
    num_inference_steps=8,
    distilled_guidance_scale=3.25,
    height=2048,
    width=2048,
    generator=generator,
).images[0]

```

## HunyuanImagePipeline[[diffusers.HunyuanImagePipeline]]

#### diffusers.HunyuanImagePipeline[[diffusers.HunyuanImagePipeline]]

```python
diffusers.HunyuanImagePipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKLHunyuanImage, text_encoder: Qwen2_5_VLForConditionalGeneration, tokenizer: Qwen2Tokenizer, text_encoder_2: T5EncoderModel, tokenizer_2: ByT5Tokenizer, transformer: HunyuanImageTransformer2DModel, guider: AdaptiveProjectedMixGuidance | None = None, ocr_guider: AdaptiveProjectedMixGuidance | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/hunyuan_image/pipeline_hunyuanimage.py#L160)

**Parameters:**

transformer ([HunyuanImageTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/hunyuanimage_transformer_2d#diffusers.HunyuanImageTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLHunyuanImage](/docs/diffusers/v0.40.0/en/api/models/autoencoder_kl_hunyuanimage#diffusers.AutoencoderKLHunyuanImage)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`Qwen2.5-VL-7B-Instruct`) : [Qwen2.5-VL-7B-Instruct](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct), specifically the [Qwen2.5-VL-7B-Instruct](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct) variant.

tokenizer (`Qwen2Tokenizer`) : Tokenizer of class [Qwen2Tokenizer].

text_encoder_2 (`T5EncoderModel`) : [T5EncoderModel](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel) variant.

tokenizer_2 (`ByT5Tokenizer`) : Tokenizer of class [ByT5Tokenizer]

guider (`AdaptiveProjectedMixGuidance`) : [AdaptiveProjectedMixGuidance]to be used to guide the image generation.

ocr_guider (`AdaptiveProjectedMixGuidance`, *optional*) : [AdaptiveProjectedMixGuidance] to be used to guide the image generation when text rendering is needed.

The HunyuanImage pipeline for text-to-image generation.

#### __call__[[diffusers.HunyuanImagePipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, negative_prompt: str | list[str] = None, height: int | None = None, width: int | None = None, num_inference_steps: int = 50, distilled_guidance_scale: float | None = 3.25, sigmas: list[float] | None = None, num_images_per_prompt: int = 1, generator: torch.Generator | list[torch.Generator] | None = None, latents: torch.Tensor | None = None, prompt_embeds: torch.Tensor | None = None, prompt_embeds_mask: torch.Tensor | None = None, negative_prompt_embeds: torch.Tensor | None = None, negative_prompt_embeds_mask: torch.Tensor | None = None, prompt_embeds_2: torch.Tensor | None = None, prompt_embeds_mask_2: torch.Tensor | None = None, negative_prompt_embeds_2: torch.Tensor | None = None, negative_prompt_embeds_mask_2: torch.Tensor | None = None, output_type: str | None = 'pil', return_dict: bool = True, attention_kwargs: dict[str, Any] | None = None, callback_on_step_end: Callable[[int, int], None] | None = None, callback_on_step_end_tensor_inputs: list[str] = ['latents'])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/hunyuan_image/pipeline_hunyuanimage.py#L504)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined and negative_prompt_embeds is not provided, will use an empty negative prompt. Ignored when not using guidance. ).

height (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The height in pixels of the generated image. This is set to 1024 by default for the best results.

width (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The width in pixels of the generated image. This is set to 1024 by default for the best results.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`list[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

distilled_guidance_scale (`float`, *optional*, defaults to None) : A guidance scale value for guidance distilled models. Unlike the traditional classifier-free guidance where the guidance scale is applied during inference through noise prediction rescaling, guidance distilled models take the guidance scale directly as an input parameter during forward pass. Guidance is enabled by setting `distilled_guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality. For guidance distilled models, this parameter is required. For non-distilled models, this parameter will be ignored.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

prompt_embeds_mask (`torch.Tensor`, *optional*) : Pre-generated text embeddings mask. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings mask will be generated from `prompt` input argument.

prompt_embeds_2 (`torch.Tensor`, *optional*) : Pre-generated text embeddings for ocr. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings for ocr will be generated from `prompt` input argument.

prompt_embeds_mask_2 (`torch.Tensor`, *optional*) : Pre-generated text embeddings mask for ocr. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings mask for ocr will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

negative_prompt_embeds_mask (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings mask. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative text embeddings mask will be generated from `negative_prompt` input argument.

negative_prompt_embeds_2 (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings for ocr. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative text embeddings for ocr will be generated from `negative_prompt` input argument.

negative_prompt_embeds_mask_2 (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings mask for ocr. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative text embeddings mask for ocr will be generated from `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.qwenimage.QwenImagePipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`List`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

**Returns:** `~pipelines.hunyuan_image.HunyuanImagePipelineOutput` or `tuple`

`~pipelines.hunyuan_image.HunyuanImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When
returning a tuple, the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import HunyuanImagePipeline

>>> pipe = HunyuanImagePipeline.from_pretrained(
...     "hunyuanvideo-community/HunyuanImage-2.1-Diffusers", torch_dtype=torch.bfloat16
... )
>>> pipe.to("cuda")
>>> prompt = "A cat holding a sign that says hello world"
>>> # Depending on the variant being used, the pipeline call will slightly vary.
>>> # Refer to the pipeline documentation for more details.
>>> image = pipe(prompt, negative_prompt="", num_inference_steps=50).images[0]
>>> image.save("hunyuanimage.png")
```

#### encode_prompt[[diffusers.HunyuanImagePipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], device: torch.device | None = None, batch_size: int = 1, num_images_per_prompt: int = 1, prompt_embeds: torch.Tensor | None = None, prompt_embeds_mask: torch.Tensor | None = None, prompt_embeds_2: torch.Tensor | None = None, prompt_embeds_mask_2: torch.Tensor | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/hunyuan_image/pipeline_hunyuanimage.py#L296)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

device : (`torch.device`): torch device

batch_size (`int`) : batch size of prompts, defaults to 1

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. If not provided, text embeddings will be generated from `prompt` input argument.

prompt_embeds_mask (`torch.Tensor`, *optional*) : Pre-generated text mask. If not provided, text mask will be generated from `prompt` input argument.

prompt_embeds_2 (`torch.Tensor`, *optional*) : Pre-generated glyph text embeddings from ByT5. If not provided, will be generated from `prompt` input argument using self.tokenizer_2 and self.text_encoder_2.

prompt_embeds_mask_2 (`torch.Tensor`, *optional*) : Pre-generated glyph text mask from ByT5. If not provided, will be generated from `prompt` input argument using self.tokenizer_2 and self.text_encoder_2.

## HunyuanImageRefinerPipeline[[diffusers.HunyuanImageRefinerPipeline]]

#### diffusers.HunyuanImageRefinerPipeline[[diffusers.HunyuanImageRefinerPipeline]]

```python
diffusers.HunyuanImageRefinerPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKLHunyuanImageRefiner, text_encoder: Qwen2_5_VLForConditionalGeneration, tokenizer: Qwen2Tokenizer, transformer: HunyuanImageTransformer2DModel, guider: AdaptiveProjectedMixGuidance | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/hunyuan_image/pipeline_hunyuanimage_refiner.py#L138)

**Parameters:**

transformer ([HunyuanImageTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/hunyuanimage_transformer_2d#diffusers.HunyuanImageTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLHunyuanImageRefiner](/docs/diffusers/v0.40.0/en/api/models/autoencoder_kl_hunyuanimage_refiner#diffusers.AutoencoderKLHunyuanImageRefiner)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`Qwen2.5-VL-7B-Instruct`) : [Qwen2.5-VL-7B-Instruct](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct), specifically the [Qwen2.5-VL-7B-Instruct](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct) variant.

tokenizer (`Qwen2Tokenizer`) : Tokenizer of class [Qwen2Tokenizer].

The HunyuanImage pipeline for text-to-image generation.

#### __call__[[diffusers.HunyuanImageRefinerPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, negative_prompt: str | list[str] = None, distilled_guidance_scale: float | None = 3.25, image: PipelineImageInput | None = None, height: int | None = None, width: int | None = None, num_inference_steps: int = 4, sigmas: list[float] | None = None, num_images_per_prompt: int = 1, generator: torch.Generator | list[torch.Generator] | None = None, latents: torch.Tensor | None = None, prompt_embeds: torch.Tensor | None = None, prompt_embeds_mask: torch.Tensor | None = None, negative_prompt_embeds: torch.Tensor | None = None, negative_prompt_embeds_mask: torch.Tensor | None = None, output_type: str | None = 'pil', return_dict: bool = True, attention_kwargs: dict[str, Any] | None = None, callback_on_step_end: Callable[[int, int], None] | None = None, callback_on_step_end_tensor_inputs: list[str] = ['latents'])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/hunyuan_image/pipeline_hunyuanimage_refiner.py#L436)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, will use an empty negative prompt. Ignored when not using guidance.

distilled_guidance_scale (`float`, *optional*, defaults to None) : A guidance scale value for guidance distilled models. Unlike the traditional classifier-free guidance where the guidance scale is applied during inference through noise prediction rescaling, guidance distilled models take the guidance scale directly as an input parameter during forward pass. Guidance is enabled by setting `distilled_guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality. For guidance distilled models, this parameter is required. For non-distilled models, this parameter will be ignored.

image (`PipelineImageInput`, *optional*) : The input image to be refined.

num_images_per_prompt (`int`, *optional*, defaults to 1) --

height (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The height in pixels of the generated image. This is set to 1024 by default for the best results.

width (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The width in pixels of the generated image. This is set to 1024 by default for the best results.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`list[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

prompt_embeds_mask (`torch.Tensor`, *optional*) : Attention mask for `prompt_embeds`.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

negative_prompt_embeds_mask (`torch.Tensor`, *optional*) : Attention mask for `negative_prompt_embeds`.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.qwenimage.QwenImagePipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`List`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

**Returns:** `~pipelines.hunyuan_image.HunyuanImagePipelineOutput` or `tuple`

`~pipelines.hunyuan_image.HunyuanImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When
returning a tuple, the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import HunyuanImageRefinerPipeline

>>> pipe = HunyuanImageRefinerPipeline.from_pretrained(
...     "hunyuanvideo-community/HunyuanImage-2.1-Refiner-Diffusers", torch_dtype=torch.bfloat16
... )
>>> pipe.to("cuda")
>>> prompt = "A cat holding a sign that says hello world"
>>> image = load_image("path/to/image.png")
>>> # Depending on the variant being used, the pipeline call will slightly vary.
>>> # Refer to the pipeline documentation for more details.
>>> image = pipe(prompt, image=image, num_inference_steps=4).images[0]
>>> image.save("hunyuanimage.png")
```

#### encode_prompt[[diffusers.HunyuanImageRefinerPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str] | None = None, device: torch.device | None = None, batch_size: int = 1, num_images_per_prompt: int = 1, prompt_embeds: torch.Tensor | None = None, prompt_embeds_mask: torch.Tensor | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/hunyuan_image/pipeline_hunyuanimage_refiner.py#L225)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

device : (`torch.device`): torch device

batch_size (`int`) : batch size of prompts, defaults to 1

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. If not provided, text embeddings will be generated from `prompt` input argument.

prompt_embeds_mask (`torch.Tensor`, *optional*) : Pre-generated text mask. If not provided, text mask will be generated from `prompt` input argument.

prompt_embeds_2 (`torch.Tensor`, *optional*) : Pre-generated glyph text embeddings from ByT5. If not provided, will be generated from `prompt` input argument using self.tokenizer_2 and self.text_encoder_2.

prompt_embeds_mask_2 (`torch.Tensor`, *optional*) : Pre-generated glyph text mask from ByT5. If not provided, will be generated from `prompt` input argument using self.tokenizer_2 and self.text_encoder_2.

## HunyuanImagePipelineOutput[[diffusers.pipelines.hunyuan_image.pipeline_output.HunyuanImagePipelineOutput]]

#### diffusers.pipelines.hunyuan_image.pipeline_output.HunyuanImagePipelineOutput[[diffusers.pipelines.hunyuan_image.pipeline_output.HunyuanImagePipelineOutput]]

```python
diffusers.pipelines.hunyuan_image.pipeline_output.HunyuanImagePipelineOutput(images: list)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/hunyuan_image/pipeline_output.py#L10)

**Parameters:**

images (`list[PIL.Image.Image]` or `np.ndarray`) : List of denoised PIL images of length `batch_size` or numpy array of shape `(batch_size, height, width, num_channels)`. PIL images or numpy array present the denoised images of the diffusion pipeline.

Output class for HunyuanImage pipelines.
