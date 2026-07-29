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

# SANA-Sprint

  

[SANA-Sprint: One-Step Diffusion with Continuous-Time Consistency Distillation](https://huggingface.co/papers/2503.09641) from NVIDIA, MIT HAN Lab, and Hugging Face by Junsong Chen, Shuchen Xue, Yuyang Zhao, Jincheng Yu, Sayak Paul, Junyu Chen, Han Cai, Enze Xie, Song Han

The abstract from the paper is:

*This paper presents SANA-Sprint, an efficient diffusion model for ultra-fast text-to-image (T2I) generation. SANA-Sprint is built on a pre-trained foundation model and augmented with hybrid distillation, dramatically reducing inference steps from 20 to 1-4. We introduce three key innovations: (1) We propose a training-free approach that transforms a pre-trained flow-matching model for continuous-time consistency distillation (sCM), eliminating costly training from scratch and achieving high training efficiency. Our hybrid distillation strategy combines sCM with latent adversarial distillation (LADD): sCM ensures alignment with the teacher model, while LADD enhances single-step generation fidelity. (2) SANA-Sprint is a unified step-adaptive model that achieves high-quality generation in 1-4 steps, eliminating step-specific training and improving efficiency. (3) We integrate ControlNet with SANA-Sprint for real-time interactive image generation, enabling instant visual feedback for user interaction. SANA-Sprint establishes a new Pareto frontier in speed-quality tradeoffs, achieving state-of-the-art performance with 7.59 FID and 0.74 GenEval in only 1 step — outperforming FLUX-schnell (7.94 FID / 0.71 GenEval) while being 10× faster (0.1s vs 1.1s on H100). It also achieves 0.1s (T2I) and 0.25s (ControlNet) latency for 1024×1024 images on H100, and 0.31s (T2I) on an RTX 4090, showcasing its exceptional efficiency and potential for AI-powered consumer applications (AIPC). Code and pre-trained models will be open-sourced.*

This pipeline was contributed by [lawrence-cj](https://github.com/lawrence-cj), [shuchen Xue](https://github.com/scxue) and [Enze Xie](https://github.com/xieenze). The original codebase can be found [here](https://github.com/NVlabs/Sana). The original weights can be found under [hf.co/Efficient-Large-Model](https://huggingface.co/Efficient-Large-Model/).

Available models:

|                                                                    Model                                                                    | Recommended dtype |
|:-------------------------------------------------------------------------------------------------------------------------------------------:|:-----------------:|
| [`Efficient-Large-Model/Sana_Sprint_1.6B_1024px_diffusers`](https://huggingface.co/Efficient-Large-Model/Sana_Sprint_1.6B_1024px_diffusers) | `torch.bfloat16`  |
| [`Efficient-Large-Model/Sana_Sprint_0.6B_1024px_diffusers`](https://huggingface.co/Efficient-Large-Model/Sana_Sprint_0.6B_1024px_diffusers) | `torch.bfloat16`  |

Refer to [this](https://huggingface.co/collections/Efficient-Large-Model/sana-sprint-67d6810d65235085b3b17c76) collection for more information.

Note: The recommended dtype mentioned is for the transformer weights. The text encoder must stay in `torch.bfloat16` and VAE weights must stay in `torch.bfloat16` or `torch.float32` for the model to work correctly. Please refer to the inference example below to see how to load the model with the recommended dtype. 

## Quantization

Quantization helps reduce the memory requirements of very large models by storing model weights in a lower precision data type. However, quantization may have varying impact on video quality depending on the video model.

Refer to the [Quantization](../../quantization/overview) overview to learn more about supported quantization backends and selecting a quantization backend that supports your use case. The example below demonstrates how to load a quantized [SanaSprintPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/sana_sprint#diffusers.SanaSprintPipeline) for inference with bitsandbytes.

```py
import torch
from diffusers import BitsAndBytesConfig as DiffusersBitsAndBytesConfig, SanaTransformer2DModel, SanaSprintPipeline
from transformers import BitsAndBytesConfig as BitsAndBytesConfig, AutoModel

quant_config = BitsAndBytesConfig(load_in_8bit=True)
text_encoder_8bit = AutoModel.from_pretrained(
    "Efficient-Large-Model/Sana_Sprint_1.6B_1024px_diffusers",
    subfolder="text_encoder",
    quantization_config=quant_config,
    torch_dtype=torch.bfloat16,
)

quant_config = DiffusersBitsAndBytesConfig(load_in_8bit=True)
transformer_8bit = SanaTransformer2DModel.from_pretrained(
    "Efficient-Large-Model/Sana_Sprint_1.6B_1024px_diffusers",
    subfolder="transformer",
    quantization_config=quant_config,
    torch_dtype=torch.bfloat16,
)

pipeline = SanaSprintPipeline.from_pretrained(
    "Efficient-Large-Model/Sana_Sprint_1.6B_1024px_diffusers",
    text_encoder=text_encoder_8bit,
    transformer=transformer_8bit,
    torch_dtype=torch.bfloat16,
    device_map="balanced",
)

prompt = "a tiny astronaut hatching from an egg on the moon"
image = pipeline(prompt).images[0]
image.save("sana.png")
```

## Setting `max_timesteps`

Users can tweak the `max_timesteps` value for experimenting with the visual quality of the generated outputs. The default `max_timesteps` value was obtained with an inference-time search process. For more details about it, check out the paper.

## Image to Image 

The [SanaSprintImg2ImgPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/sana_sprint#diffusers.SanaSprintImg2ImgPipeline) is a pipeline for image-to-image generation. It takes an input image and a prompt, and generates a new image based on the input image and the prompt.

```py
import torch
from diffusers import SanaSprintImg2ImgPipeline
from diffusers.utils.loading_utils import load_image

image = load_image(
    "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/penguin.png"
)

pipe = SanaSprintImg2ImgPipeline.from_pretrained(
    "Efficient-Large-Model/Sana_Sprint_1.6B_1024px_diffusers", 
    torch_dtype=torch.bfloat16)
pipe.to("cuda")

image = pipe(
    prompt="a cute pink bear", 
    image=image, 
    strength=0.5, 
    height=832, 
    width=480
).images[0]
image.save("output.png")
```

## SanaSprintPipeline[[diffusers.SanaSprintPipeline]]

#### diffusers.SanaSprintPipeline[[diffusers.SanaSprintPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/sana/pipeline_sana_sprint.py#L141)

Pipeline for text-to-image generation using [SANA-Sprint](https://huggingface.co/papers/2503.09641).

__call__diffusers.SanaSprintPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/sana/pipeline_sana_sprint.py#L615[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "num_inference_steps", "val": ": int = 2"}, {"name": "timesteps", "val": ": list = None"}, {"name": "max_timesteps", "val": ": float = 1.5708"}, {"name": "intermediate_timesteps", "val": ": float = 1.3"}, {"name": "guidance_scale", "val": ": float = 4.5"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "height", "val": ": int = 1024"}, {"name": "width", "val": ": int = 1024"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "clean_caption", "val": ": bool = False"}, {"name": "use_resolution_binning", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 300"}, {"name": "complex_human_instruction", "val": ": list = [\"Given a user prompt, generate an 'Enhanced prompt' that provides detailed visual descriptions suitable for image generation. Evaluate the level of detail in the user prompt:\", '- If the prompt is simple, focus on adding specifics about colors, shapes, sizes, textures, and spatial relationships to create vivid and concrete scenes.', '- If the prompt is already detailed, refine and enhance the existing details slightly without overcomplicating.', 'Here are examples of how to transform or refine prompts:', '- User Prompt: A cat sleeping -> Enhanced: A small, fluffy white cat curled up in a round shape, sleeping peacefully on a warm sunny windowsill, surrounded by pots of blooming red flowers.', '- User Prompt: A busy city street -> Enhanced: A bustling city street scene at dusk, featuring glowing street lamps, a diverse crowd of people in colorful clothing, and a double-decker bus passing by towering glass skyscrapers.', 'Please generate only the enhanced description for the prompt below and avoid including any additional commentary or evaluations:', 'User Prompt: ']"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **num_inference_steps** (`int`, *optional*, defaults to 20) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **max_timesteps** (`float`, *optional*, defaults to 1.57080) --
  The maximum timestep value used in the SCM scheduler.
- **intermediate_timesteps** (`float`, *optional*, defaults to 1.3) --
  The intermediate timestep value used in SCM scheduler (only used when num_inference_steps=2).
- **timesteps** (`list[int]`, *optional*) --
  Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument
  in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is
  passed will be used. Must be in descending order.
- **guidance_scale** (`float`, *optional*, defaults to 4.5) --
  Embedded guiddance scale is enabled by setting `guidance_scale` > 1. Higher `guidance_scale` encourages
  a model to generate images more aligned with `prompt` at the expense of lower image quality.

  Guidance-distilled models approximates true classifer-free guidance for `guidance_scale` > 1. Refer to
  the [paper](https://huggingface.co/papers/2210.03142) to learn more.
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **height** (`int`, *optional*, defaults to self.unet.config.sample_size) --
  The height in pixels of the generated image.
- **width** (`int`, *optional*, defaults to self.unet.config.sample_size) --
  The width in pixels of the generated image.
- **eta** (`float`, *optional*, defaults to 0.0) --
  Corresponds to parameter eta (η) in the DDIM paper: https://huggingface.co/papers/2010.02502. Only
  applies to [schedulers.DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), will be ignored for others.
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
- **prompt_attention_mask** (`torch.Tensor`, *optional*) -- Pre-generated attention mask for text embeddings.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.stable_diffusion.IFPipelineOutput` instead of a plain tuple.
- **attention_kwargs** --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **clean_caption** (`bool`, *optional*, defaults to `True`) --
  Whether or not to clean the caption before creating embeddings. Requires `beautifulsoup4` and `ftfy` to
  be installed. If the dependencies are not installed, the embeddings will be created from the raw
  prompt.
- **use_resolution_binning** (`bool` defaults to `True`) --
  If set to `True`, the requested height and width are first mapped to the closest resolutions using
  `ASPECT_RATIO_1024_BIN`. After the produced latents are decoded into images, they are resized back to
  the requested resolution. Useful for generating non-square images.
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`list`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.
- **max_sequence_length** (`int` defaults to `300`) --
  Maximum sequence length to use with the `prompt`.
- **complex_human_instruction** (`list[str]`, *optional*) --
  Instructions for complex human attention:
  https://github.com/NVlabs/Sana/blob/main/configs/sana_app_config/Sana_1600M_app.yaml#L55.0[SanaPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/sana_sprint#diffusers.pipelines.sana.pipeline_output.SanaPipelineOutput) or `tuple`If `return_dict` is `True`, [SanaPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/sana_sprint#diffusers.pipelines.sana.pipeline_output.SanaPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated images

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import SanaSprintPipeline

>>> pipe = SanaSprintPipeline.from_pretrained(
...     "Efficient-Large-Model/Sana_Sprint_1.6B_1024px_diffusers", torch_dtype=torch.bfloat16
... )
>>> pipe.to("cuda")

>>> image = pipe(prompt="a tiny astronaut hatching from an egg on the moon")[0]
>>> image[0].save("output.png")
```

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

num_inference_steps (`int`, *optional*, defaults to 20) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

max_timesteps (`float`, *optional*, defaults to 1.57080) : The maximum timestep value used in the SCM scheduler.

intermediate_timesteps (`float`, *optional*, defaults to 1.3) : The intermediate timestep value used in SCM scheduler (only used when num_inference_steps=2).

timesteps (`list[int]`, *optional*) : Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used. Must be in descending order.

guidance_scale (`float`, *optional*, defaults to 4.5) : Embedded guiddance scale is enabled by setting `guidance_scale` > 1. Higher `guidance_scale` encourages a model to generate images more aligned with `prompt` at the expense of lower image quality.  Guidance-distilled models approximates true classifer-free guidance for `guidance_scale` > 1. Refer to the [paper](https://huggingface.co/papers/2210.03142) to learn more.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

height (`int`, *optional*, defaults to self.unet.config.sample_size) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to self.unet.config.sample_size) : The width in pixels of the generated image.

eta (`float`, *optional*, defaults to 0.0) : Corresponds to parameter eta (η) in the DDIM paper: https://huggingface.co/papers/2010.02502. Only applies to [schedulers.DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), will be ignored for others.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

prompt_attention_mask (`torch.Tensor`, *optional*) : Pre-generated attention mask for text embeddings.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.stable_diffusion.IFPipelineOutput` instead of a plain tuple.

attention_kwargs : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

clean_caption (`bool`, *optional*, defaults to `True`) : Whether or not to clean the caption before creating embeddings. Requires `beautifulsoup4` and `ftfy` to be installed. If the dependencies are not installed, the embeddings will be created from the raw prompt.

use_resolution_binning (`bool` defaults to `True`) : If set to `True`, the requested height and width are first mapped to the closest resolutions using `ASPECT_RATIO_1024_BIN`. After the produced latents are decoded into images, they are resized back to the requested resolution. Useful for generating non-square images.

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int` defaults to `300`) : Maximum sequence length to use with the `prompt`.

complex_human_instruction (`list[str]`, *optional*) : Instructions for complex human attention: https://github.com/NVlabs/Sana/blob/main/configs/sana_app_config/Sana_1600M_app.yaml#L55.

**Returns:**

`[SanaPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/sana_sprint#diffusers.pipelines.sana.pipeline_output.SanaPipelineOutput) or `tuple``

If `return_dict` is `True`, [SanaPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/sana_sprint#diffusers.pipelines.sana.pipeline_output.SanaPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated images
#### disable_vae_slicing[[diffusers.SanaSprintPipeline.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/sana/pipeline_sana_sprint.py#L187)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### disable_vae_tiling[[diffusers.SanaSprintPipeline.disable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/sana/pipeline_sana_sprint.py#L214)

Disable tiled VAE decoding. If `enable_vae_tiling` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_vae_slicing[[diffusers.SanaSprintPipeline.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/sana/pipeline_sana_sprint.py#L174)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### enable_vae_tiling[[diffusers.SanaSprintPipeline.enable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/sana/pipeline_sana_sprint.py#L200)

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.
#### encode_prompt[[diffusers.SanaSprintPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/sana/pipeline_sana_sprint.py#L286)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded 

num_images_per_prompt (`int`, *optional*, defaults to 1) : number of images that should be generated per prompt

device : (`torch.device`, *optional*): torch device to place the resulting embeddings on

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

clean_caption (`bool`, defaults to `False`) : If `True`, the function will preprocess and clean the provided caption before encoding.

max_sequence_length (`int`, defaults to 300) : Maximum sequence length to use for the prompt.

complex_human_instruction (`list[str]`, defaults to `complex_human_instruction`) : If `complex_human_instruction` is not empty, the function will use the complex Human instruction for the prompt.

## SanaSprintImg2ImgPipeline[[diffusers.SanaSprintImg2ImgPipeline]]

#### diffusers.SanaSprintImg2ImgPipeline[[diffusers.SanaSprintImg2ImgPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/sana/pipeline_sana_sprint_img2img.py#L147)

Pipeline for text-to-image generation using [SANA-Sprint](https://huggingface.co/papers/2503.09641).

__call__diffusers.SanaSprintImg2ImgPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/sana/pipeline_sana_sprint_img2img.py#L686[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "num_inference_steps", "val": ": int = 2"}, {"name": "timesteps", "val": ": list = None"}, {"name": "max_timesteps", "val": ": float = 1.5708"}, {"name": "intermediate_timesteps", "val": ": float = 1.3"}, {"name": "guidance_scale", "val": ": float = 4.5"}, {"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] = None"}, {"name": "strength", "val": ": float = 0.6"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "height", "val": ": int = 1024"}, {"name": "width", "val": ": int = 1024"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "clean_caption", "val": ": bool = False"}, {"name": "use_resolution_binning", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 300"}, {"name": "complex_human_instruction", "val": ": list = [\"Given a user prompt, generate an 'Enhanced prompt' that provides detailed visual descriptions suitable for image generation. Evaluate the level of detail in the user prompt:\", '- If the prompt is simple, focus on adding specifics about colors, shapes, sizes, textures, and spatial relationships to create vivid and concrete scenes.', '- If the prompt is already detailed, refine and enhance the existing details slightly without overcomplicating.', 'Here are examples of how to transform or refine prompts:', '- User Prompt: A cat sleeping -> Enhanced: A small, fluffy white cat curled up in a round shape, sleeping peacefully on a warm sunny windowsill, surrounded by pots of blooming red flowers.', '- User Prompt: A busy city street -> Enhanced: A bustling city street scene at dusk, featuring glowing street lamps, a diverse crowd of people in colorful clothing, and a double-decker bus passing by towering glass skyscrapers.', 'Please generate only the enhanced description for the prompt below and avoid including any additional commentary or evaluations:', 'User Prompt: ']"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **num_inference_steps** (`int`, *optional*, defaults to 20) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **max_timesteps** (`float`, *optional*, defaults to 1.57080) --
  The maximum timestep value used in the SCM scheduler.
- **intermediate_timesteps** (`float`, *optional*, defaults to 1.3) --
  The intermediate timestep value used in SCM scheduler (only used when num_inference_steps=2).
- **timesteps** (`list[int]`, *optional*) --
  Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument
  in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is
  passed will be used. Must be in descending order.
- **guidance_scale** (`float`, *optional*, defaults to 4.5) --
  Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://arxiv.org/abs/2207.12598).
  `guidance_scale` is defined as `w` of equation 2. of [Imagen
  Paper](https://arxiv.org/pdf/2205.11487.pdf). Guidance scale is enabled by setting `guidance_scale >
  1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`,
  usually at the expense of lower image quality.
- **image** (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) --
  `Image`, numpy array or tensor representing an image batch to be used as the starting point. Can also
  accept image latents as `image`, but if passing latents directly it is not encoded again.
- **strength** (`float`, *optional*, defaults to 0.6) --
  Indicates extent to transform the reference `image`. Must be between 0 and 1. `image` is used as a
  starting point and more noise is added the higher the `strength`. The number of denoising steps depends
  on the amount of noise initially added. When `strength` is 1, added noise is maximum and the denoising
  process runs for the full number of iterations specified in `num_inference_steps`. A value of 1
  essentially ignores `image`.
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **height** (`int`, *optional*, defaults to self.unet.config.sample_size) --
  The height in pixels of the generated image.
- **width** (`int`, *optional*, defaults to self.unet.config.sample_size) --
  The width in pixels of the generated image.
- **eta** (`float`, *optional*, defaults to 0.0) --
  Corresponds to parameter eta (η) in the DDIM paper: https://arxiv.org/abs/2010.02502. Only applies to
  [schedulers.DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), will be ignored for others.
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
- **prompt_attention_mask** (`torch.Tensor`, *optional*) -- Pre-generated attention mask for text embeddings.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.stable_diffusion.IFPipelineOutput` instead of a plain tuple.
- **attention_kwargs** --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **clean_caption** (`bool`, *optional*, defaults to `True`) --
  Whether or not to clean the caption before creating embeddings. Requires `beautifulsoup4` and `ftfy` to
  be installed. If the dependencies are not installed, the embeddings will be created from the raw
  prompt.
- **use_resolution_binning** (`bool` defaults to `True`) --
  If set to `True`, the requested height and width are first mapped to the closest resolutions using
  `ASPECT_RATIO_1024_BIN`. After the produced latents are decoded into images, they are resized back to
  the requested resolution. Useful for generating non-square images.
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`list`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.
- **max_sequence_length** (`int` defaults to `300`) --
  Maximum sequence length to use with the `prompt`.
- **complex_human_instruction** (`list[str]`, *optional*) --
  Instructions for complex human attention:
  https://github.com/NVlabs/Sana/blob/main/configs/sana_app_config/Sana_1600M_app.yaml#L55.0[SanaPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/sana_sprint#diffusers.pipelines.sana.pipeline_output.SanaPipelineOutput) or `tuple`If `return_dict` is `True`, [SanaPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/sana_sprint#diffusers.pipelines.sana.pipeline_output.SanaPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated images

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import SanaSprintImg2ImgPipeline
>>> from diffusers.utils.loading_utils import load_image

>>> pipe = SanaSprintImg2ImgPipeline.from_pretrained(
...     "Efficient-Large-Model/Sana_Sprint_1.6B_1024px_diffusers", torch_dtype=torch.bfloat16
... )
>>> pipe.to("cuda")

>>> image = load_image(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/penguin.png"
... )

>>> image = pipe(prompt="a cute pink bear", image=image, strength=0.5, height=832, width=480).images[0]
>>> image[0].save("output.png")
```

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

num_inference_steps (`int`, *optional*, defaults to 20) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

max_timesteps (`float`, *optional*, defaults to 1.57080) : The maximum timestep value used in the SCM scheduler.

intermediate_timesteps (`float`, *optional*, defaults to 1.3) : The intermediate timestep value used in SCM scheduler (only used when num_inference_steps=2).

timesteps (`list[int]`, *optional*) : Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used. Must be in descending order.

guidance_scale (`float`, *optional*, defaults to 4.5) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://arxiv.org/abs/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://arxiv.org/pdf/2205.11487.pdf). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) : `Image`, numpy array or tensor representing an image batch to be used as the starting point. Can also accept image latents as `image`, but if passing latents directly it is not encoded again.

strength (`float`, *optional*, defaults to 0.6) : Indicates extent to transform the reference `image`. Must be between 0 and 1. `image` is used as a starting point and more noise is added the higher the `strength`. The number of denoising steps depends on the amount of noise initially added. When `strength` is 1, added noise is maximum and the denoising process runs for the full number of iterations specified in `num_inference_steps`. A value of 1 essentially ignores `image`.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

height (`int`, *optional*, defaults to self.unet.config.sample_size) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to self.unet.config.sample_size) : The width in pixels of the generated image.

eta (`float`, *optional*, defaults to 0.0) : Corresponds to parameter eta (η) in the DDIM paper: https://arxiv.org/abs/2010.02502. Only applies to [schedulers.DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), will be ignored for others.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

prompt_attention_mask (`torch.Tensor`, *optional*) : Pre-generated attention mask for text embeddings.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.stable_diffusion.IFPipelineOutput` instead of a plain tuple.

attention_kwargs : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

clean_caption (`bool`, *optional*, defaults to `True`) : Whether or not to clean the caption before creating embeddings. Requires `beautifulsoup4` and `ftfy` to be installed. If the dependencies are not installed, the embeddings will be created from the raw prompt.

use_resolution_binning (`bool` defaults to `True`) : If set to `True`, the requested height and width are first mapped to the closest resolutions using `ASPECT_RATIO_1024_BIN`. After the produced latents are decoded into images, they are resized back to the requested resolution. Useful for generating non-square images.

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int` defaults to `300`) : Maximum sequence length to use with the `prompt`.

complex_human_instruction (`list[str]`, *optional*) : Instructions for complex human attention: https://github.com/NVlabs/Sana/blob/main/configs/sana_app_config/Sana_1600M_app.yaml#L55.

**Returns:**

`[SanaPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/sana_sprint#diffusers.pipelines.sana.pipeline_output.SanaPipelineOutput) or `tuple``

If `return_dict` is `True`, [SanaPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/sana_sprint#diffusers.pipelines.sana.pipeline_output.SanaPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated images
#### disable_vae_slicing[[diffusers.SanaSprintImg2ImgPipeline.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/sana/pipeline_sana_sprint_img2img.py#L196)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### disable_vae_tiling[[diffusers.SanaSprintImg2ImgPipeline.disable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/sana/pipeline_sana_sprint_img2img.py#L224)

Disable tiled VAE decoding. If `enable_vae_tiling` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_vae_slicing[[diffusers.SanaSprintImg2ImgPipeline.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/sana/pipeline_sana_sprint_img2img.py#L182)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### enable_vae_tiling[[diffusers.SanaSprintImg2ImgPipeline.enable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/sana/pipeline_sana_sprint_img2img.py#L210)

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.
#### encode_prompt[[diffusers.SanaSprintImg2ImgPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/sana/pipeline_sana_sprint_img2img.py#L297)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded 

num_images_per_prompt (`int`, *optional*, defaults to 1) : number of images that should be generated per prompt

device : (`torch.device`, *optional*): torch device to place the resulting embeddings on

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

clean_caption (`bool`, defaults to `False`) : If `True`, the function will preprocess and clean the provided caption before encoding.

max_sequence_length (`int`, defaults to 300) : Maximum sequence length to use for the prompt.

complex_human_instruction (`list[str]`, defaults to `complex_human_instruction`) : If `complex_human_instruction` is not empty, the function will use the complex Human instruction for the prompt.

## SanaPipelineOutput[[diffusers.pipelines.sana.pipeline_output.SanaPipelineOutput]]

#### diffusers.pipelines.sana.pipeline_output.SanaPipelineOutput[[diffusers.pipelines.sana.pipeline_output.SanaPipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/sana/pipeline_output.py#L10)

Output class for Sana pipelines.

**Parameters:**

images (`list[PIL.Image.Image]` or `np.ndarray`) : list of denoised PIL images of length `batch_size` or numpy array of shape `(batch_size, height, width, num_channels)`. PIL images or numpy array present the denoised images of the diffusion pipeline.
