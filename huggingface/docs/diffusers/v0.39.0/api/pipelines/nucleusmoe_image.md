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

# NucleusMoE-Image

[NucleusMoE-Image](https://huggingface.co/NucleusAI/NucleusMoE-Image) is a text-to-image model that pairs a single-stream DiT with Mixture-of-Experts feed-forward layers, cross-attention to a Qwen3-VL text encoder, and a flow-matching Euler discrete scheduler.

> [!TIP]
> Make sure to check out the Schedulers [guide](../../using-diffusers/schedulers) to learn how to explore the tradeoff between scheduler speed and quality, and see the [reuse components across pipelines](../../using-diffusers/loading#reuse-a-pipeline) section to learn how to efficiently load the same components into multiple pipelines.

## NucleusMoEImagePipeline[[diffusers.NucleusMoEImagePipeline]]

#### diffusers.NucleusMoEImagePipeline[[diffusers.NucleusMoEImagePipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/nucleusmoe_image/pipeline_nucleusmoe_image.py#L132)

Pipeline for text-to-image generation using NucleusMoE.

This pipeline uses a single-stream DiT with Mixture-of-Experts feed-forward layers, cross-attention to a Qwen3-VL
text encoder, and a flow-matching Euler discrete scheduler.

__call__diffusers.NucleusMoEImagePipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/nucleusmoe_image/pipeline_nucleusmoe_image.py#L379[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str | list[str] = None"}, {"name": "guidance_scale", "val": ": float = 4.0"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "sigmas", "val": ": list[float] | None = None"}, {"name": "num_images_per_prompt", "val": ": int = 1"}, {"name": "max_sequence_length", "val": ": int | None = None"}, {"name": "return_index", "val": ": int | None = None"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds_mask", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds_mask", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int, dict], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, an empty string is used when
  `true_cfg_scale > 1`.
- **guidance_scale** (`float`, *optional*, defaults to 4.0) --
  Classifier-free guidance scale. Values greater than 1 enable CFG.
- **return_index** (`int`, *optional*) --
  Layer index of the text encoder output to use for the prompt embeddings.
- **height** (`int`, *optional*, defaults to `self.default_sample_size * self.vae_scale_factor`) --
  The height in pixels of the generated image.
- **width** (`int`, *optional*, defaults to `self.default_sample_size * self.vae_scale_factor`) --
  The width in pixels of the generated image.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps.
- **sigmas** (`list[float]`, *optional*) --
  Custom sigmas for the denoising schedule. If not defined, a linear schedule is used.
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of torch generators to make generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents to be used as inputs for image generation.
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings.
- **prompt_embeds_mask** (`torch.Tensor`, *optional*) --
  Attention mask for pre-generated text embeddings.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings.
- **negative_prompt_embeds_mask** (`torch.Tensor`, *optional*) --
  Attention mask for pre-generated negative text embeddings.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generated image. Choose between `"pil"`, `"np"`, or `"latent"`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `NucleusMoEImagePipelineOutput` instead of a plain tuple.
- **attention_kwargs** (`dict`, *optional*) --
  Kwargs passed to the attention processor.
- **callback_on_step_end** (`Callable`, *optional*) --
  A function called at the end of each denoising step.
- **callback_on_step_end_tensor_inputs** (`list`, *optional*) --
  Tensor inputs for the `callback_on_step_end` function.
- **max_sequence_length** (`int`, defaults to 512) --
  Maximum sequence length for the text prompt.0`NucleusMoEImagePipelineOutput` or `tuple``NucleusMoEImagePipelineOutput` if `return_dict` is True, otherwise a `tuple` where the first element
is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import NucleusMoEImagePipeline

>>> pipe = NucleusMoEImagePipeline.from_pretrained("NucleusAI/NucleusMoE-Image", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")
>>> prompt = "A cat holding a sign that says hello world"
>>> image = pipe(prompt, num_inference_steps=50).images[0]
>>> image.save("nucleus_moe.png")
```

**Parameters:**

transformer (`NucleusMoEImageTransformer2DModel`) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLQwenImage](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl_qwenimage#diffusers.AutoencoderKLQwenImage)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`Qwen3VLForConditionalGeneration`) : Text encoder for computing prompt embeddings.

processor (`Qwen3VLProcessor`) : Processor for tokenizing text inputs.

**Returns:**

``NucleusMoEImagePipelineOutput` or `tuple``

`NucleusMoEImagePipelineOutput` if `return_dict` is True, otherwise a `tuple` where the first element
is a list with the generated images.
#### encode_prompt[[diffusers.NucleusMoEImagePipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/nucleusmoe_image/pipeline_nucleusmoe_image.py#L187)

Encode text prompt(s) into embeddings using the Qwen3-VL text encoder.

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to encode.

device (`torch.device`, *optional*) : Torch device for the resulting tensors.

num_images_per_prompt (`int`, defaults to 1) : Number of images to generate per prompt.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Skips encoding when provided.

prompt_embeds_mask (`torch.Tensor`, *optional*) : Attention mask for pre-generated embeddings.

max_sequence_length (`int`, defaults to 1024) : Maximum token length for the encoded prompt.

## NucleusMoEImagePipelineOutput[[diffusers.pipelines.nucleusmoe_image.pipeline_output.NucleusMoEImagePipelineOutput]]

#### diffusers.pipelines.nucleusmoe_image.pipeline_output.NucleusMoEImagePipelineOutput[[diffusers.pipelines.nucleusmoe_image.pipeline_output.NucleusMoEImagePipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/nucleusmoe_image/pipeline_output.py#L10)

Output class for NucleusMoE Image pipelines.

**Parameters:**

images (`list[PIL.Image.Image]` or `np.ndarray`) : List of denoised PIL images of length `batch_size` or numpy array of shape `(batch_size, height, width, num_channels)`. PIL images or numpy array present the denoised images of the diffusion pipeline.
