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

# Sana-Video

  
  

[SANA-Video: Efficient Video Generation with Block Linear Diffusion Transformer](https://huggingface.co/papers/2509.24695) from NVIDIA and MIT HAN Lab, by Junsong Chen, Yuyang Zhao, Jincheng Yu, Ruihang Chu, Junyu Chen, Shuai Yang, Xianbang Wang, Yicheng Pan, Daquan Zhou, Huan Ling, Haozhe Liu, Hongwei Yi, Hao Zhang, Muyang Li, Yukang Chen, Han Cai, Sanja Fidler, Ping Luo, Song Han, Enze Xie.

The abstract from the paper is:

*We introduce SANA-Video, a small diffusion model that can efficiently generate videos up to 720x1280 resolution and minute-length duration. SANA-Video synthesizes high-resolution, high-quality and long videos with strong text-video alignment at a remarkably fast speed, deployable on RTX 5090 GPU. Two core designs ensure our efficient, effective and long video generation: (1) Linear DiT: We leverage linear attention as the core operation, which is more efficient than vanilla attention given the large number of tokens processed in video generation. (2) Constant-Memory KV cache for Block Linear Attention: we design block-wise autoregressive approach for long video generation by employing a constant-memory state, derived from the cumulative properties of linear attention. This KV cache provides the Linear DiT with global context at a fixed memory cost, eliminating the need for a traditional KV cache and enabling efficient, minute-long video generation. In addition, we explore effective data filters and model training strategies, narrowing the training cost to 12 days on 64 H100 GPUs, which is only 1% of the cost of MovieGen. Given its low cost, SANA-Video achieves competitive performance compared to modern state-of-the-art small diffusion models (e.g., Wan 2.1-1.3B and SkyReel-V2-1.3B) while being 16x faster in measured latency. Moreover, SANA-Video can be deployed on RTX 5090 GPUs with NVFP4 precision, accelerating the inference speed of generating a 5-second 720p video from 71s to 29s (2.4x speedup). In summary, SANA-Video enables low-cost, high-quality video generation. [this https URL](https://github.com/NVlabs/SANA).*

This pipeline was contributed by SANA Team. The original codebase can be found [here](https://github.com/NVlabs/Sana). The original weights can be found under [hf.co/Efficient-Large-Model](https://hf.co/collections/Efficient-Large-Model/sana-video).

Available models:

| Model | Recommended dtype |
|:-----:|:-----------------:|
| [`Efficient-Large-Model/SANA-Video_2B_480p_diffusers`](https://huggingface.co/Efficient-Large-Model/ANA-Video_2B_480p_diffusers) | `torch.bfloat16` |

Refer to [this](https://huggingface.co/collections/Efficient-Large-Model/sana-video) collection for more information.

Note: The recommended dtype mentioned is for the transformer weights. The text encoder and VAE weights must stay in `torch.bfloat16` or `torch.float32` for the model to work correctly. Please refer to the inference example below to see how to load the model with the recommended dtype. 

## Generation Pipelines

`

The example below demonstrates how to use the text-to-video pipeline to generate a video using a text description.

```python
pipe = SanaVideoPipeline.from_pretrained(
    "Efficient-Large-Model/SANA-Video_2B_480p_diffusers", 
    torch_dtype=torch.bfloat16,
)
pipe.text_encoder.to(torch.bfloat16)
pipe.vae.to(torch.float32)
pipe.to("cuda")

prompt = "A cat and a dog baking a cake together in a kitchen. The cat is carefully measuring flour, while the dog is stirring the batter with a wooden spoon. The kitchen is cozy, with sunlight streaming through the window."
negative_prompt = "A chaotic sequence with misshapen, deformed limbs in heavy motion blur, sudden disappearance, jump cuts, jerky movements, rapid shot changes, frames out of sync, inconsistent character shapes, temporal artifacts, jitter, and ghosting effects, creating a disorienting visual experience."
motion_scale = 30
motion_prompt = f" motion score: {motion_scale}."
prompt = prompt + motion_prompt

video = pipe(
    prompt=prompt,
    negative_prompt=negative_prompt,
    height=480,
    width=832,
    frames=81,
    guidance_scale=6,
    num_inference_steps=50,
    generator=torch.Generator(device="cuda").manual_seed(0),
).frames[0]

export_to_video(video, "sana_video.mp4", fps=16)
```

The example below demonstrates how to use the image-to-video pipeline to generate a video using a text description and a starting frame.

```python
pipe = SanaImageToVideoPipeline.from_pretrained(
    "Efficient-Large-Model/SANA-Video_2B_480p_diffusers",
    torch_dtype=torch.bfloat16,
)
pipe.scheduler = FlowMatchEulerDiscreteScheduler.from_config(pipe.scheduler.config, flow_shift=8.0)
pipe.vae.to(torch.float32)
pipe.text_encoder.to(torch.bfloat16)
pipe.to("cuda")

image = load_image("https://raw.githubusercontent.com/NVlabs/Sana/refs/heads/main/asset/samples/i2v-1.png")
prompt = "A woman stands against a stunning sunset backdrop, her long, wavy brown hair gently blowing in the breeze. She wears a sleeveless, light-colored blouse with a deep V-neckline, which accentuates her graceful posture. The warm hues of the setting sun cast a golden glow across her face and hair, creating a serene and ethereal atmosphere. The background features a blurred landscape with soft, rolling hills and scattered clouds, adding depth to the scene. The camera remains steady, capturing the tranquil moment from a medium close-up angle."
negative_prompt = "A chaotic sequence with misshapen, deformed limbs in heavy motion blur, sudden disappearance, jump cuts, jerky movements, rapid shot changes, frames out of sync, inconsistent character shapes, temporal artifacts, jitter, and ghosting effects, creating a disorienting visual experience."
motion_scale = 30
motion_prompt = f" motion score: {motion_scale}."
prompt = prompt + motion_prompt

motion_scale = 30.0

video = pipe(
    image=image,
    prompt=prompt,
    negative_prompt=negative_prompt,
    height=480,
    width=832,
    frames=81,
    guidance_scale=6,
    num_inference_steps=50,
    generator=torch.Generator(device="cuda").manual_seed(0),
).frames[0]

export_to_video(video, "sana-i2v.mp4", fps=16)
```

## Quantization

Quantization helps reduce the memory requirements of very large models by storing model weights in a lower precision data type. However, quantization may have varying impact on video quality depending on the video model.

Refer to the [Quantization](../../quantization/overview) overview to learn more about supported quantization backends and selecting a quantization backend that supports your use case. The example below demonstrates how to load a quantized [SanaVideoPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/sana_video#diffusers.SanaVideoPipeline) for inference with bitsandbytes.

```py
import torch
from diffusers import BitsAndBytesConfig as DiffusersBitsAndBytesConfig, SanaVideoTransformer3DModel, SanaVideoPipeline
from transformers import BitsAndBytesConfig as BitsAndBytesConfig, AutoModel

quant_config = BitsAndBytesConfig(load_in_8bit=True)
text_encoder_8bit = AutoModel.from_pretrained(
    "Efficient-Large-Model/SANA-Video_2B_480p_diffusers",
    subfolder="text_encoder",
    quantization_config=quant_config,
    torch_dtype=torch.float16,
)

quant_config = DiffusersBitsAndBytesConfig(load_in_8bit=True)
transformer_8bit = SanaVideoTransformer3DModel.from_pretrained(
    "Efficient-Large-Model/SANA-Video_2B_480p_diffusers",
    subfolder="transformer",
    quantization_config=quant_config,
    torch_dtype=torch.float16,
)

pipeline = SanaVideoPipeline.from_pretrained(
    "Efficient-Large-Model/SANA-Video_2B_480p_diffusers",
    text_encoder=text_encoder_8bit,
    transformer=transformer_8bit,
    torch_dtype=torch.float16,
    device_map="balanced",
)

model_score = 30
prompt = "Evening, backlight, side lighting, soft light, high contrast, mid-shot, centered composition, clean solo shot, warm color. A young Caucasian man stands in a forest, golden light glimmers on his hair as sunlight filters through the leaves. He wears a light shirt, wind gently blowing his hair and collar, light dances across his face with his movements. The background is blurred, with dappled light and soft tree shadows in the distance. The camera focuses on his lifted gaze, clear and emotional."
negative_prompt = "A chaotic sequence with misshapen, deformed limbs in heavy motion blur, sudden disappearance, jump cuts, jerky movements, rapid shot changes, frames out of sync, inconsistent character shapes, temporal artifacts, jitter, and ghosting effects, creating a disorienting visual experience."
motion_prompt = f" motion score: {model_score}."
prompt = prompt + motion_prompt

output = pipeline(
    prompt=prompt,
    negative_prompt=negative_prompt,
    height=480,
    width=832,
    num_frames=81,
    guidance_scale=6.0,
    num_inference_steps=50
).frames[0]
export_to_video(output, "sana-video-output.mp4", fps=16)
```

## SanaVideoPipeline[[diffusers.SanaVideoPipeline]]

#### diffusers.SanaVideoPipeline[[diffusers.SanaVideoPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/sana_video/pipeline_sana_video.py#L186)

Pipeline for text-to-video generation using [Sana](https://huggingface.co/papers/2509.24695). This model inherits
from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods implemented for all
pipelines (downloading, saving, running on a particular device, etc.).

__call__diffusers.SanaVideoPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/sana_video/pipeline_sana_video.py#L712[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str = ''"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "timesteps", "val": ": list = None"}, {"name": "sigmas", "val": ": list = None"}, {"name": "guidance_scale", "val": ": float = 6.0"}, {"name": "num_videos_per_prompt", "val": ": int | None = 1"}, {"name": "height", "val": ": int = 480"}, {"name": "width", "val": ": int = 832"}, {"name": "frames", "val": ": int = 81"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "clean_caption", "val": ": bool = False"}, {"name": "use_resolution_binning", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 300"}, {"name": "complex_human_instruction", "val": ": list = [\"Given a user prompt, generate an 'Enhanced prompt' that provides detailed visual descriptions suitable for video generation. Evaluate the level of detail in the user prompt:\", '- If the prompt is simple, focus on adding specifics about colors, shapes, sizes, textures, motion, and temporal relationships to create vivid and dynamic scenes.', '- If the prompt is already detailed, refine and enhance the existing details slightly without overcomplicating.', 'Here are examples of how to transform or refine prompts:', '- User Prompt: A cat sleeping -> Enhanced: A small, fluffy white cat slowly settling into a curled position, peacefully falling asleep on a warm sunny windowsill, with gentle sunlight filtering through surrounding pots of blooming red flowers.', '- User Prompt: A busy city street -> Enhanced: A bustling city street scene at dusk, featuring glowing street lamps gradually lighting up, a diverse crowd of people in colorful clothing walking past, and a double-decker bus smoothly passing by towering glass skyscrapers.', 'Please generate only the enhanced description for the prompt below and avoid including any additional commentary or evaluations:', 'User Prompt: ']"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the video generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the video generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is
  less than `1`).
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality video at the
  expense of slower inference.
- **timesteps** (`list[int]`, *optional*) --
  Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument
  in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is
  passed will be used. Must be in descending order.
- **sigmas** (`list[float]`, *optional*) --
  Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in
  their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed
  will be used.
- **guidance_scale** (`float`, *optional*, defaults to 4.5) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2.
  of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting
  `guidance_scale > 1`. Higher guidance scale encourages to generate videos that are closely linked to
  the text `prompt`, usually at the expense of lower video quality.
- **num_videos_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of videos to generate per prompt.
- **height** (`int`, *optional*, defaults to 480) --
  The height in pixels of the generated video.
- **width** (`int`, *optional*, defaults to 832) --
  The width in pixels of the generated video.
- **frames** (`int`, *optional*, defaults to 81) --
  The number of frames in the generated video.
- **eta** (`float`, *optional*, defaults to 0.0) --
  Corresponds to parameter eta (η) in the DDIM paper: https://huggingface.co/papers/2010.02502. Only
  applies to [schedulers.DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), will be ignored for others.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for video
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **prompt_attention_mask** (`torch.Tensor`, *optional*) -- Pre-generated attention mask for text embeddings.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings. For PixArt-Sigma this negative prompt should be "". If not
  provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.
- **negative_prompt_attention_mask** (`torch.Tensor`, *optional*) --
  Pre-generated attention mask for negative text embeddings.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generated video. Choose between mp4 or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `SanaVideoPipelineOutput` instead of a plain tuple.
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
  `ASPECT_RATIO_480_BIN` or `ASPECT_RATIO_720_BIN`. After the produced latents are decoded into videos,
  they are resized back to the requested resolution. Useful for generating non-square videos.
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`List`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.
- **max_sequence_length** (`int` defaults to `300`) --
  Maximum sequence length to use with the `prompt`.
- **complex_human_instruction** (`list[str]`, *optional*) --
  Instructions for complex human attention:
  https://github.com/NVlabs/Sana/blob/main/configs/sana_app_config/Sana_1600M_app.yaml#L55.0[SanaVideoPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/sana_video#diffusers.pipelines.sana_video.pipeline_output.SanaVideoPipelineOutput) or `tuple`If `return_dict` is `True`, [SanaVideoPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/sana_video#diffusers.pipelines.sana_video.pipeline_output.SanaVideoPipelineOutput) is
returned, otherwise a `tuple` is returned where the first element is a list with the generated videos

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import SanaVideoPipeline
>>> from diffusers.utils import export_to_video

>>> pipe = SanaVideoPipeline.from_pretrained("Efficient-Large-Model/SANA-Video_2B_480p_diffusers")
>>> pipe.transformer.to(torch.bfloat16)
>>> pipe.text_encoder.to(torch.bfloat16)
>>> pipe.vae.to(torch.float32)
>>> pipe.to("cuda")
>>> motion_score = 30

>>> prompt = "Evening, backlight, side lighting, soft light, high contrast, mid-shot, centered composition, clean solo shot, warm color. A young Caucasian man stands in a forest, golden light glimmers on his hair as sunlight filters through the leaves. He wears a light shirt, wind gently blowing his hair and collar, light dances across his face with his movements. The background is blurred, with dappled light and soft tree shadows in the distance. The camera focuses on his lifted gaze, clear and emotional."
>>> negative_prompt = "A chaotic sequence with misshapen, deformed limbs in heavy motion blur, sudden disappearance, jump cuts, jerky movements, rapid shot changes, frames out of sync, inconsistent character shapes, temporal artifacts, jitter, and ghosting effects, creating a disorienting visual experience."
>>> motion_prompt = f" motion score: {motion_score}."
>>> prompt = prompt + motion_prompt

>>> output = pipe(
...     prompt=prompt,
...     negative_prompt=negative_prompt,
...     height=480,
...     width=832,
...     frames=81,
...     guidance_scale=6,
...     num_inference_steps=50,
...     generator=torch.Generator(device="cuda").manual_seed(42),
... ).frames[0]

>>> export_to_video(output, "sana-video-output.mp4", fps=16)
```

**Parameters:**

tokenizer (`GemmaTokenizer` or `GemmaTokenizerFast`) : The tokenizer used to tokenize the prompt.

text_encoder (`Gemma2PreTrainedModel`) : Text encoder model to encode the input prompts.

vae ([`AutoencoderKLWan`, `AutoencoderDC`, or `AutoencoderKLLTX2Video`]) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

transformer ([SanaVideoTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/sana_video_transformer3d#diffusers.SanaVideoTransformer3DModel)) : Conditional Transformer to denoise the input latents.

scheduler ([DPMSolverMultistepScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/multistep_dpm_solver#diffusers.DPMSolverMultistepScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded video latents.

**Returns:**

`[SanaVideoPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/sana_video#diffusers.pipelines.sana_video.pipeline_output.SanaVideoPipelineOutput) or `tuple``

If `return_dict` is `True`, [SanaVideoPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/sana_video#diffusers.pipelines.sana_video.pipeline_output.SanaVideoPipelineOutput) is
returned, otherwise a `tuple` is returned where the first element is a list with the generated videos
#### encode_prompt[[diffusers.SanaVideoPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/sana_video/pipeline_sana_video.py#L303)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt not to guide the video generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`). For PixArt-Alpha, this should be "".

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : whether to use classifier free guidance or not

num_videos_per_prompt (`int`, *optional*, defaults to 1) : number of videos that should be generated per prompt

device : (`torch.device`, *optional*): torch device to place the resulting embeddings on

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. For Sana, it's should be the embeddings of the "" string.

clean_caption (`bool`, defaults to `False`) : If `True`, the function will preprocess and clean the provided caption before encoding.

max_sequence_length (`int`, defaults to 300) : Maximum sequence length to use for the prompt.

complex_human_instruction (`list[str]`, defaults to `complex_human_instruction`) : If `complex_human_instruction` is not empty, the function will use the complex Human instruction for the prompt.

## SanaImageToVideoPipeline[[diffusers.SanaImageToVideoPipeline]]

#### diffusers.SanaImageToVideoPipeline[[diffusers.SanaImageToVideoPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/sana_video/pipeline_sana_video_i2v.py#L176)

Pipeline for image/text-to-video generation using [Sana](https://huggingface.co/papers/2509.24695). This model
inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods implemented for all
pipelines (downloading, saving, running on a particular device, etc.).

__call__diffusers.SanaImageToVideoPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/sana_video/pipeline_sana_video_i2v.py#L739[{"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor]"}, {"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str = ''"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "timesteps", "val": ": list = None"}, {"name": "sigmas", "val": ": list = None"}, {"name": "guidance_scale", "val": ": float = 6.0"}, {"name": "num_videos_per_prompt", "val": ": int | None = 1"}, {"name": "height", "val": ": int = 480"}, {"name": "width", "val": ": int = 832"}, {"name": "frames", "val": ": int = 81"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "clean_caption", "val": ": bool = False"}, {"name": "use_resolution_binning", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 300"}, {"name": "complex_human_instruction", "val": ": list = [\"Given a user prompt, generate an 'Enhanced prompt' that provides detailed visual descriptions suitable for video generation. Evaluate the level of detail in the user prompt:\", '- If the prompt is simple, focus on adding specifics about colors, shapes, sizes, textures, motion, and temporal relationships to create vivid and dynamic scenes.', '- If the prompt is already detailed, refine and enhance the existing details slightly without overcomplicating.', 'Here are examples of how to transform or refine prompts:', '- User Prompt: A cat sleeping -> Enhanced: A small, fluffy white cat slowly settling into a curled position, peacefully falling asleep on a warm sunny windowsill, with gentle sunlight filtering through surrounding pots of blooming red flowers.', '- User Prompt: A busy city street -> Enhanced: A bustling city street scene at dusk, featuring glowing street lamps gradually lighting up, a diverse crowd of people in colorful clothing walking past, and a double-decker bus smoothly passing by towering glass skyscrapers.', 'Please generate only the enhanced description for the prompt below and avoid including any additional commentary or evaluations:', 'User Prompt: ']"}]- **image** (`PipelineImageInput`) --
  The input image to condition the video generation on. The first frame of the generated video will be
  conditioned on this image.
- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the video generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the video generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is
  less than `1`).
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality video at the
  expense of slower inference.
- **timesteps** (`list[int]`, *optional*) --
  Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument
  in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is
  passed will be used. Must be in descending order.
- **sigmas** (`list[float]`, *optional*) --
  Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in
  their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed
  will be used.
- **guidance_scale** (`float`, *optional*, defaults to 4.5) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2.
  of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting
  `guidance_scale > 1`. Higher guidance scale encourages to generate videos that are closely linked to
  the text `prompt`, usually at the expense of lower video quality.
- **num_videos_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of videos to generate per prompt.
- **height** (`int`, *optional*, defaults to 480) --
  The height in pixels of the generated video.
- **width** (`int`, *optional*, defaults to 832) --
  The width in pixels of the generated video.
- **frames** (`int`, *optional*, defaults to 81) --
  The number of frames in the generated video.
- **eta** (`float`, *optional*, defaults to 0.0) --
  Corresponds to parameter eta (η) in the DDIM paper: https://huggingface.co/papers/2010.02502. Only
  applies to [schedulers.DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), will be ignored for others.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for video
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **prompt_attention_mask** (`torch.Tensor`, *optional*) -- Pre-generated attention mask for text embeddings.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings. For PixArt-Sigma this negative prompt should be "". If not
  provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.
- **negative_prompt_attention_mask** (`torch.Tensor`, *optional*) --
  Pre-generated attention mask for negative text embeddings.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generated video. Choose between mp4 or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `SanaVideoPipelineOutput` instead of a plain tuple.
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
  `ASPECT_RATIO_480_BIN` or `ASPECT_RATIO_720_BIN`. After the produced latents are decoded into videos,
  they are resized back to the requested resolution. Useful for generating non-square videos.
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`List`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.
- **max_sequence_length** (`int` defaults to `300`) --
  Maximum sequence length to use with the `prompt`.
- **complex_human_instruction** (`list[str]`, *optional*) --
  Instructions for complex human attention:
  https://github.com/NVlabs/Sana/blob/main/configs/sana_app_config/Sana_1600M_app.yaml#L55.0[SanaVideoPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/sana_video#diffusers.pipelines.sana_video.pipeline_output.SanaVideoPipelineOutput) or `tuple`If `return_dict` is `True`, [SanaVideoPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/sana_video#diffusers.pipelines.sana_video.pipeline_output.SanaVideoPipelineOutput) is
returned, otherwise a `tuple` is returned where the first element is a list with the generated videos

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import SanaImageToVideoPipeline
>>> from diffusers.utils import export_to_video, load_image

>>> pipe = SanaImageToVideoPipeline.from_pretrained("Efficient-Large-Model/SANA-Video_2B_480p_diffusers")
>>> pipe.transformer.to(torch.bfloat16)
>>> pipe.text_encoder.to(torch.bfloat16)
>>> pipe.vae.to(torch.float32)
>>> pipe.to("cuda")
>>> motion_score = 30

>>> prompt = "A woman stands against a stunning sunset backdrop, her long, wavy brown hair gently blowing in the breeze. She wears a sleeveless, light-colored blouse with a deep V-neckline, which accentuates her graceful posture. The warm hues of the setting sun cast a golden glow across her face and hair, creating a serene and ethereal atmosphere. The background features a blurred landscape with soft, rolling hills and scattered clouds, adding depth to the scene. The camera remains steady, capturing the tranquil moment from a medium close-up angle."
>>> negative_prompt = "A chaotic sequence with misshapen, deformed limbs in heavy motion blur, sudden disappearance, jump cuts, jerky movements, rapid shot changes, frames out of sync, inconsistent character shapes, temporal artifacts, jitter, and ghosting effects, creating a disorienting visual experience."
>>> motion_prompt = f" motion score: {motion_score}."
>>> prompt = prompt + motion_prompt
>>> image = load_image("https://raw.githubusercontent.com/NVlabs/Sana/refs/heads/main/asset/samples/i2v-1.png")

>>> output = pipe(
...     image=image,
...     prompt=prompt,
...     negative_prompt=negative_prompt,
...     height=480,
...     width=832,
...     frames=81,
...     guidance_scale=6,
...     num_inference_steps=50,
...     generator=torch.Generator(device="cuda").manual_seed(42),
... ).frames[0]

>>> export_to_video(output, "sana-ti2v-output.mp4", fps=16)
```

**Parameters:**

tokenizer (`GemmaTokenizer` or `GemmaTokenizerFast`) : The tokenizer used to tokenize the prompt.

text_encoder (`Gemma2PreTrainedModel`) : Text encoder model to encode the input prompts.

vae ([`AutoencoderKLWan`, `AutoencoderDC`, or `AutoencoderKLLTX2Video`]) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

transformer ([SanaVideoTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/sana_video_transformer3d#diffusers.SanaVideoTransformer3DModel)) : Conditional Transformer to denoise the input latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded video latents.

**Returns:**

`[SanaVideoPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/sana_video#diffusers.pipelines.sana_video.pipeline_output.SanaVideoPipelineOutput) or `tuple``

If `return_dict` is `True`, [SanaVideoPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/sana_video#diffusers.pipelines.sana_video.pipeline_output.SanaVideoPipelineOutput) is
returned, otherwise a `tuple` is returned where the first element is a list with the generated videos
#### encode_prompt[[diffusers.SanaImageToVideoPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/sana_video/pipeline_sana_video_i2v.py#L301)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt not to guide the video generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`). For PixArt-Alpha, this should be "".

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : whether to use classifier free guidance or not

num_videos_per_prompt (`int`, *optional*, defaults to 1) : number of videos that should be generated per prompt

device : (`torch.device`, *optional*): torch device to place the resulting embeddings on

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. For Sana, it's should be the embeddings of the "" string.

clean_caption (`bool`, defaults to `False`) : If `True`, the function will preprocess and clean the provided caption before encoding.

max_sequence_length (`int`, defaults to 300) : Maximum sequence length to use for the prompt.

complex_human_instruction (`list[str]`, defaults to `complex_human_instruction`) : If `complex_human_instruction` is not empty, the function will use the complex Human instruction for the prompt.

## SanaVideoPipelineOutput[[diffusers.pipelines.sana_video.pipeline_output.SanaVideoPipelineOutput]]

#### diffusers.pipelines.sana_video.pipeline_output.SanaVideoPipelineOutput[[diffusers.pipelines.sana_video.pipeline_output.SanaVideoPipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/sana_video/pipeline_output.py#L9)

Output class for Sana-Video pipelines.

**Parameters:**

frames (`torch.Tensor`, `np.ndarray`, or list[list[PIL.Image.Image]]) : List of video outputs - It can be a nested list of length `batch_size,` with each sub-list containing denoised PIL image sequences of length `num_frames.` It can also be a NumPy array or Torch tensor of shape `(batch_size, num_frames, channels, height, width)`.
