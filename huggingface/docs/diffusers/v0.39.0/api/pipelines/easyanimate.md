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
# limitations under the License.
-->

# EasyAnimate
[EasyAnimate](https://github.com/aigc-apps/EasyAnimate) by Alibaba PAI.

The description from it's GitHub page:
*EasyAnimate is a pipeline based on the transformer architecture, designed for generating AI images and videos, and for training baseline models and Lora models for Diffusion Transformer. We support direct prediction from pre-trained EasyAnimate models, allowing for the generation of videos with various resolutions, approximately 6 seconds in length, at 8fps (EasyAnimateV5.1, 1 to 49 frames). Additionally, users can train their own baseline and Lora models for specific style transformations.*

This pipeline was contributed by [bubbliiiing](https://github.com/bubbliiiing). The original codebase can be found [here](https://huggingface.co/alibaba-pai). The original weights can be found under [hf.co/alibaba-pai](https://huggingface.co/alibaba-pai).

There are two official EasyAnimate checkpoints for text-to-video and video-to-video.

| checkpoints | recommended inference dtype |
|:---:|:---:|
| [`alibaba-pai/EasyAnimateV5.1-12b-zh`](https://huggingface.co/alibaba-pai/EasyAnimateV5.1-12b-zh) | torch.float16 |
| [`alibaba-pai/EasyAnimateV5.1-12b-zh-InP`](https://huggingface.co/alibaba-pai/EasyAnimateV5.1-12b-zh-InP) | torch.float16 |

There is one official EasyAnimate checkpoints available for image-to-video and video-to-video.

| checkpoints | recommended inference dtype |
|:---:|:---:|
| [`alibaba-pai/EasyAnimateV5.1-12b-zh-InP`](https://huggingface.co/alibaba-pai/EasyAnimateV5.1-12b-zh-InP) | torch.float16 |

There are two official EasyAnimate checkpoints available for control-to-video.

| checkpoints | recommended inference dtype |
|:---:|:---:|
| [`alibaba-pai/EasyAnimateV5.1-12b-zh-Control`](https://huggingface.co/alibaba-pai/EasyAnimateV5.1-12b-zh-Control) | torch.float16 |
| [`alibaba-pai/EasyAnimateV5.1-12b-zh-Control-Camera`](https://huggingface.co/alibaba-pai/EasyAnimateV5.1-12b-zh-Control-Camera) | torch.float16 |

For the EasyAnimateV5.1 series:
- Text-to-video (T2V) and Image-to-video (I2V) works for multiple resolutions. The width and height can vary from 256 to 1024.
- Both T2V and I2V models support generation with 1~49 frames and work best at this value. Exporting videos at 8 FPS is recommended.

## Quantization

Quantization helps reduce the memory requirements of very large models by storing model weights in a lower precision data type. However, quantization may have varying impact on video quality depending on the video model.

Refer to the [Quantization](../../quantization/overview) overview to learn more about supported quantization backends and selecting a quantization backend that supports your use case. The example below demonstrates how to load a quantized [EasyAnimatePipeline](/docs/diffusers/v0.39.0/en/api/pipelines/easyanimate#diffusers.EasyAnimatePipeline) for inference with bitsandbytes.

```py
import torch
from diffusers import BitsAndBytesConfig as DiffusersBitsAndBytesConfig, EasyAnimateTransformer3DModel, EasyAnimatePipeline
from diffusers.utils import export_to_video

quant_config = DiffusersBitsAndBytesConfig(load_in_8bit=True)
transformer_8bit = EasyAnimateTransformer3DModel.from_pretrained(
    "alibaba-pai/EasyAnimateV5.1-12b-zh",
    subfolder="transformer",
    quantization_config=quant_config,
    torch_dtype=torch.float16,
)

pipeline = EasyAnimatePipeline.from_pretrained(
    "alibaba-pai/EasyAnimateV5.1-12b-zh",
    transformer=transformer_8bit,
    torch_dtype=torch.float16,
    device_map="balanced",
)

prompt = "A cat walks on the grass, realistic style."
negative_prompt = "bad detailed"
video = pipeline(prompt=prompt, negative_prompt=negative_prompt, num_frames=49, num_inference_steps=30).frames[0]
export_to_video(video, "cat.mp4", fps=8)
```

## EasyAnimatePipeline[[diffusers.EasyAnimatePipeline]]

#### diffusers.EasyAnimatePipeline[[diffusers.EasyAnimatePipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/easyanimate/pipeline_easyanimate.py#L186)

Pipeline for text-to-video generation using EasyAnimate.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

EasyAnimate uses one text encoder [qwen2 vl](https://huggingface.co/Qwen/Qwen2-VL-7B-Instruct) in V5.1.

__call__diffusers.EasyAnimatePipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/easyanimate/pipeline_easyanimate.py#L524[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "num_frames", "val": ": int | None = 49"}, {"name": "height", "val": ": int | None = 512"}, {"name": "width", "val": ": int | None = 512"}, {"name": "num_inference_steps", "val": ": int | None = 50"}, {"name": "guidance_scale", "val": ": float | None = 5.0"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "eta", "val": ": float | None = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "timesteps", "val": ": list[int] | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "callback_on_step_end", "val": ": typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "guidance_rescale", "val": ": float = 0.0"}]- **prompt** (`str` or `list[str]`, *optional*) --
  Text prompts to guide the image or video generation. If not provided, use `prompt_embeds` instead.
- **num_frames** (`int`, *optional*) --
  Length of the generated video (in frames).
- **height** (`int`, *optional*) --
  Height of the generated image in pixels.
- **width** (`int`, *optional*) --
  Width of the generated image in pixels.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  Number of denoising steps during generation. More steps generally yield higher quality images but slow
  down inference.
- **guidance_scale** (`float`, *optional*, defaults to 5.0) --
  Encourages the model to align outputs with prompts. A higher value may decrease image quality.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  Prompts indicating what to exclude in generation. If not specified, use `negative_prompt_embeds`.
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  Number of images to generate for each prompt.
- **eta** (`float`, *optional*, defaults to 0.0) --
  Applies to DDIM scheduling. Controlled by the eta parameter from the related literature.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  A generator to ensure reproducibility in image generation.
- **latents** (`torch.Tensor`, *optional*) --
  Predefined latent tensors to condition generation.
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Text embeddings for the prompts. Overrides prompt string inputs for more flexibility.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Embeddings for negative prompts. Overrides string inputs if defined.
- **prompt_attention_mask** (`torch.Tensor`, *optional*) --
  Attention mask for the primary prompt embeddings.
- **negative_prompt_attention_mask** (`torch.Tensor`, *optional*) --
  Attention mask for negative prompt embeddings.
- **output_type** (`str`, *optional*, defaults to "latent") --
  Format of the generated output, either as a PIL image or as a NumPy array.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  If `True`, returns a structured output. Otherwise returns a simple tuple.
- **callback_on_step_end** (`Callable`, *optional*) --
  Functions called at the end of each denoising step.
- **callback_on_step_end_tensor_inputs** (`list[str]`, *optional*) --
  Tensor names to be included in callback function calls.
- **guidance_rescale** (`float`, *optional*, defaults to 0.0) --
  Adjusts noise levels based on guidance scale.
- **timesteps** (`list[int]`, *optional*) --
  Custom timesteps to use for the denoising process. If not defined, the scheduler's default schedule for
  `num_inference_steps` is used.0[StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) or `tuple`If `return_dict` is `True`, [StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated images and the
second element is a list of `bool`s indicating whether the corresponding generated image contains
"not-safe-for-work" (nsfw) content.

Generates images or video using the EasyAnimate pipeline based on the provided prompts.

Examples:
```python
>>> import torch
>>> from diffusers import EasyAnimatePipeline
>>> from diffusers.utils import export_to_video

>>> # Models: "alibaba-pai/EasyAnimateV5.1-12b-zh"
>>> pipe = EasyAnimatePipeline.from_pretrained(
...     "alibaba-pai/EasyAnimateV5.1-7b-zh-diffusers", torch_dtype=torch.float16
... ).to("cuda")
>>> prompt = (
...     "A panda, dressed in a small, red jacket and a tiny hat, sits on a wooden stool in a serene bamboo forest. "
...     "The panda's fluffy paws strum a miniature acoustic guitar, producing soft, melodic tunes. Nearby, a few other "
...     "pandas gather, watching curiously and some clapping in rhythm. Sunlight filters through the tall bamboo, "
...     "casting a gentle glow on the scene. The panda's face is expressive, showing concentration and joy as it plays. "
...     "The background includes a small, flowing stream and vibrant green foliage, enhancing the peaceful and magical "
...     "atmosphere of this unique musical performance."
... )
>>> sample_size = (512, 512)
>>> video = pipe(
...     prompt=prompt,
...     guidance_scale=6,
...     negative_prompt="bad detailed",
...     height=sample_size[0],
...     width=sample_size[1],
...     num_inference_steps=50,
... ).frames[0]
>>> export_to_video(video, "output.mp4", fps=8)
```

**Parameters:**

vae ([AutoencoderKLMagvit](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl_magvit#diffusers.AutoencoderKLMagvit)) : Variational Auto-Encoder (VAE) Model to encode and decode video to and from latent representations.

text_encoder (`~transformers.Qwen2VLForConditionalGeneration`, `~transformers.BertModel` | None) : EasyAnimate uses [qwen2 vl](https://huggingface.co/Qwen/Qwen2-VL-7B-Instruct) in V5.1.

tokenizer (`~transformers.Qwen2Tokenizer`, `~transformers.BertTokenizer` | None) : A `Qwen2Tokenizer` or `BertTokenizer` to tokenize text.

transformer ([EasyAnimateTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/easyanimate_transformer3d#diffusers.EasyAnimateTransformer3DModel)) : The EasyAnimate model designed by EasyAnimate Team.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with EasyAnimate to denoise the encoded image latents.

**Returns:**

`[StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) or `tuple``

If `return_dict` is `True`, [StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated images and the
second element is a list of `bool`s indicating whether the corresponding generated image contains
"not-safe-for-work" (nsfw) content.
#### encode_prompt[[diffusers.EasyAnimatePipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/easyanimate/pipeline_easyanimate.py#L241)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

device : (`torch.device`): torch device

dtype (`torch.dtype`) : torch dtype

num_images_per_prompt (`int`) : number of images that should be generated per prompt

do_classifier_free_guidance (`bool`) : whether to use classifier free guidance or not

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

prompt_attention_mask (`torch.Tensor`, *optional*) : Attention mask for the prompt. Required when `prompt_embeds` is passed directly.

negative_prompt_attention_mask (`torch.Tensor`, *optional*) : Attention mask for the negative prompt. Required when `negative_prompt_embeds` is passed directly.

max_sequence_length (`int`, *optional*) : maximum sequence length to use for the prompt.

## EasyAnimatePipelineOutput[[diffusers.pipelines.easyanimate.pipeline_output.EasyAnimatePipelineOutput]]

#### diffusers.pipelines.easyanimate.pipeline_output.EasyAnimatePipelineOutput[[diffusers.pipelines.easyanimate.pipeline_output.EasyAnimatePipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/easyanimate/pipeline_output.py#L9)

Output class for EasyAnimate pipelines.

**Parameters:**

frames (`torch.Tensor`, `np.ndarray`, or list[list[PIL.Image.Image]]) : list of video outputs - It can be a nested list of length `batch_size,` with each sub-list containing denoised PIL image sequences of length `num_frames.` It can also be a NumPy array or Torch tensor of shape `(batch_size, num_frames, channels, height, width)`.
