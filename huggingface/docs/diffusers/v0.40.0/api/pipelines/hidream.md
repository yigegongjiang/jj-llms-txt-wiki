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

# HiDreamImage

[HiDream-I1](https://huggingface.co/HiDream-ai) by HiDream.ai

> [!TIP]
> [Caching](../../optimization/cache) may also speed up inference by storing and reusing intermediate outputs.

## Available models

The following models are available for the [HiDreamImagePipeline](/docs/diffusers/v0.40.0/en/api/pipelines/hidream#diffusers.HiDreamImagePipeline) pipeline:

| Model name | Description |
|:---|:---|
| [`HiDream-ai/HiDream-I1-Full`](https://huggingface.co/HiDream-ai/HiDream-I1-Full) | - |
| [`HiDream-ai/HiDream-I1-Dev`](https://huggingface.co/HiDream-ai/HiDream-I1-Dev) | - |
| [`HiDream-ai/HiDream-I1-Fast`](https://huggingface.co/HiDream-ai/HiDream-I1-Fast) | - |

## HiDreamImagePipeline[[diffusers.HiDreamImagePipeline]]

#### diffusers.HiDreamImagePipeline[[diffusers.HiDreamImagePipeline]]

```python
diffusers.HiDreamImagePipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKL, text_encoder: CLIPTextModelWithProjection, tokenizer: CLIPTokenizer, text_encoder_2: CLIPTextModelWithProjection, tokenizer_2: CLIPTokenizer, text_encoder_3: T5EncoderModel, tokenizer_3: T5Tokenizer, text_encoder_4: LlamaForCausalLM, tokenizer_4: TokenizersBackend, transformer: HiDreamImageTransformer2DModel)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/hidream_image/pipeline_hidream_image.py#L159)

#### __call__[[diffusers.HiDreamImagePipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, prompt_2: str | list[str] | None = None, prompt_3: str | list[str] | None = None, prompt_4: str | list[str] | None = None, height: int | None = None, width: int | None = None, num_inference_steps: int = 50, sigmas: list[float] | None = None, guidance_scale: float = 5.0, negative_prompt: str | list[str] | None = None, negative_prompt_2: str | list[str] | None = None, negative_prompt_3: str | list[str] | None = None, negative_prompt_4: str | list[str] | None = None, num_images_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.FloatTensor] = None, prompt_embeds_t5: typing.Optional[torch.FloatTensor] = None, prompt_embeds_llama3: typing.Optional[torch.FloatTensor] = None, negative_prompt_embeds_t5: typing.Optional[torch.FloatTensor] = None, negative_prompt_embeds_llama3: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, negative_pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, output_type: str | None = 'pil', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 128, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/hidream_image/pipeline_hidream_image.py#L674)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is will be used instead.

prompt_3 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to `tokenizer_3` and `text_encoder_3`. If not defined, `prompt` is will be used instead.

prompt_4 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to `tokenizer_4` and `text_encoder_4`. If not defined, `prompt` is will be used instead.

height (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The height in pixels of the generated image. This is set to 1024 by default for the best results.

width (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The width in pixels of the generated image. This is set to 1024 by default for the best results.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`list[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

guidance_scale (`float`, *optional*, defaults to 3.5) : Embedded guiddance scale is enabled by setting `guidance_scale` > 1. Higher `guidance_scale` encourages a model to generate images more aligned with `prompt` at the expense of lower image quality.  Guidance-distilled models approximates true classifer-free guidance for `guidance_scale` > 1. Refer to the [paper](https://huggingface.co/papers/2210.03142) to learn more.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `true_cfg_scale` is not greater than `1`).

negative_prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `negative_prompt` is used in all the text-encoders.

negative_prompt_3 (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation to be sent to `tokenizer_3` and `text_encoder_3`. If not defined, `negative_prompt` is used in all the text-encoders.

negative_prompt_4 (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation to be sent to `tokenizer_4` and `text_encoder_4`. If not defined, `negative_prompt` is used in all the text-encoders.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.FloatTensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds_t5 (`torch.FloatTensor`, *optional*) : Pre-generated T5 text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

prompt_embeds_llama3 (`torch.FloatTensor`, *optional*) : Pre-generated LLaMA3 text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds_t5 (`torch.FloatTensor`, *optional*) : Pre-generated negative T5 text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, embeddings will be generated from `negative_prompt` input argument.

negative_prompt_embeds_llama3 (`torch.FloatTensor`, *optional*) : Pre-generated negative LLaMA3 text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, embeddings will be generated from `negative_prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

negative_pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled negative_prompt_embeds will be generated from `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.flux.FluxPipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int` defaults to 128) : Maximum sequence length to use with the `prompt`.

**Returns:** `~pipelines.hidream_image.HiDreamImagePipelineOutput` or `tuple`

`~pipelines.hidream_image.HiDreamImagePipelineOutput` if `return_dict` is True, otherwise a `tuple`. When
returning a tuple, the first element is a list with the generated. images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from transformers import AutoTokenizer, LlamaForCausalLM
>>> from diffusers import HiDreamImagePipeline

>>> tokenizer_4 = AutoTokenizer.from_pretrained("meta-llama/Meta-Llama-3.1-8B-Instruct")
>>> text_encoder_4 = LlamaForCausalLM.from_pretrained(
...     "meta-llama/Meta-Llama-3.1-8B-Instruct",
...     output_hidden_states=True,
...     output_attentions=True,
...     torch_dtype=torch.bfloat16,
... )

>>> pipe = HiDreamImagePipeline.from_pretrained(
...     "HiDream-ai/HiDream-I1-Full",
...     tokenizer_4=tokenizer_4,
...     text_encoder_4=text_encoder_4,
...     torch_dtype=torch.bfloat16,
... )
>>> pipe.enable_model_cpu_offload()

>>> image = pipe(
...     'A cat holding a sign that says "Hi-Dreams.ai".',
...     height=1024,
...     width=1024,
...     guidance_scale=5.0,
...     num_inference_steps=50,
...     generator=torch.Generator("cuda").manual_seed(0),
... ).images[0]
>>> image.save("output.png")
```

## HiDreamImagePipelineOutput[[diffusers.pipelines.hidream_image.pipeline_output.HiDreamImagePipelineOutput]]

#### diffusers.pipelines.hidream_image.pipeline_output.HiDreamImagePipelineOutput[[diffusers.pipelines.hidream_image.pipeline_output.HiDreamImagePipelineOutput]]

```python
diffusers.pipelines.hidream_image.pipeline_output.HiDreamImagePipelineOutput(images: list[PIL.Image.Image] | numpy.ndarray)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/hidream_image/pipeline_output.py#L24)

**Parameters:**

images (`list[PIL.Image.Image]` or `np.ndarray`) : list of denoised PIL images of length `batch_size` or numpy array of shape `(batch_size, height, width, num_channels)`. PIL images or numpy array present the denoised images of the diffusion pipeline.

Output class for HiDreamImage pipelines.
