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

# Latte

![latte text-to-video](https://github.com/Vchitect/Latte/blob/52bc0029899babbd6e9250384c83d8ed2670ff7a/visuals/latte.gif?raw=true)

[Latte: Latent Diffusion Transformer for Video Generation](https://huggingface.co/papers/2401.03048) from Monash University, Shanghai AI Lab, Nanjing University, and Nanyang Technological University.

The abstract from the paper is:

*We propose a novel Latent Diffusion Transformer, namely Latte, for video generation. Latte first extracts spatio-temporal tokens from input videos and then adopts a series of Transformer blocks to model video distribution in the latent space. In order to model a substantial number of tokens extracted from videos, four efficient variants are introduced from the perspective of decomposing the spatial and temporal dimensions of input videos. To improve the quality of generated videos, we determine the best practices of Latte through rigorous experimental analysis, including video clip patch embedding, model variants, timestep-class information injection, temporal positional embedding, and learning strategies. Our comprehensive evaluation demonstrates that Latte achieves state-of-the-art performance across four standard video generation datasets, i.e., FaceForensics, SkyTimelapse, UCF101, and Taichi-HD. In addition, we extend Latte to text-to-video generation (T2V) task, where Latte achieves comparable results compared to recent T2V models. We strongly believe that Latte provides valuable insights for future research on incorporating Transformers into diffusion models for video generation.*

**Highlights**: Latte is a latent diffusion transformer proposed as a backbone for modeling different modalities (trained for text-to-video generation here). It achieves state-of-the-art performance across four standard video benchmarks - [FaceForensics](https://huggingface.co/papers/1803.09179), [SkyTimelapse](https://huggingface.co/papers/1709.07592), [UCF101](https://huggingface.co/papers/1212.0402) and [Taichi-HD](https://huggingface.co/papers/2003.00196). To prepare and download the datasets for evaluation, please refer to [this https URL](https://github.com/Vchitect/Latte/blob/main/docs/datasets_evaluation.md).

This pipeline was contributed by [maxin-cn](https://github.com/maxin-cn). The original codebase can be found [here](https://github.com/Vchitect/Latte). The original weights can be found under [hf.co/maxin-cn](https://huggingface.co/maxin-cn).

> [!TIP]
> Make sure to check out the Schedulers [guide](../../using-diffusers/schedulers) to learn how to explore the tradeoff between scheduler speed and quality, and see the [reuse components across pipelines](../../using-diffusers/loading#reuse-a-pipeline) section to learn how to efficiently load the same components into multiple pipelines.

### Inference

Use [`torch.compile`](https://huggingface.co/docs/diffusers/main/en/tutorials/fast_diffusion#torchcompile) to reduce the inference latency.

First, load the pipeline:

```python
import torch
from diffusers import LattePipeline

pipeline = LattePipeline.from_pretrained(
	"maxin-cn/Latte-1", torch_dtype=torch.float16
).to("cuda")
```

Then change the memory layout of the pipelines `transformer` and `vae` components to `torch.channels-last`:

```python
pipeline.transformer.to(memory_format=torch.channels_last)
pipeline.vae.to(memory_format=torch.channels_last)
```

Finally, compile the components and run inference:

```python
pipeline.transformer = torch.compile(pipeline.transformer)
pipeline.vae.decode = torch.compile(pipeline.vae.decode)

video = pipeline(prompt="A dog wearing sunglasses floating in space, surreal, nebulae in background").frames[0]
```

The [benchmark](https://gist.github.com/a-r-r-o-w/4e1694ca46374793c0361d740a99ff19) results on an 80GB A100 machine are:

```
Without torch.compile(): Average inference time: 16.246 seconds.
With torch.compile(): Average inference time: 14.573 seconds.
```

## Quantization

Quantization helps reduce the memory requirements of very large models by storing model weights in a lower precision data type. However, quantization may have varying impact on video quality depending on the video model.

Refer to the [Quantization](../../quantization/overview) overview to learn more about supported quantization backends and selecting a quantization backend that supports your use case. The example below demonstrates how to load a quantized [LattePipeline](/docs/diffusers/v0.39.0/en/api/pipelines/latte#diffusers.LattePipeline) for inference with bitsandbytes.

```py
import torch
from diffusers import BitsAndBytesConfig as DiffusersBitsAndBytesConfig, LatteTransformer3DModel, LattePipeline
from diffusers.utils import export_to_gif
from transformers import BitsAndBytesConfig as BitsAndBytesConfig, T5EncoderModel

quant_config = BitsAndBytesConfig(load_in_8bit=True)
text_encoder_8bit = T5EncoderModel.from_pretrained(
    "maxin-cn/Latte-1",
    subfolder="text_encoder",
    quantization_config=quant_config,
    torch_dtype=torch.float16,
)

quant_config = DiffusersBitsAndBytesConfig(load_in_8bit=True)
transformer_8bit = LatteTransformer3DModel.from_pretrained(
    "maxin-cn/Latte-1",
    subfolder="transformer",
    quantization_config=quant_config,
    torch_dtype=torch.float16,
)

pipeline = LattePipeline.from_pretrained(
    "maxin-cn/Latte-1",
    text_encoder=text_encoder_8bit,
    transformer=transformer_8bit,
    torch_dtype=torch.float16,
    device_map="balanced",
)

prompt = "A small cactus with a happy face in the Sahara desert."
video = pipeline(prompt).frames[0]
export_to_gif(video, "latte.gif")
```

## LattePipeline[[diffusers.LattePipeline]]

#### diffusers.LattePipeline[[diffusers.LattePipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/latte/pipeline_latte.py#L145)

Pipeline for text-to-video generation using Latte.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

__call__diffusers.LattePipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/latte/pipeline_latte.py#L613[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str = ''"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "timesteps", "val": ": list[int] | None = None"}, {"name": "guidance_scale", "val": ": float = 7.5"}, {"name": "num_images_per_prompt", "val": ": int = 1"}, {"name": "video_length", "val": ": int = 16"}, {"name": "height", "val": ": int = 512"}, {"name": "width", "val": ": int = 512"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.FloatTensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.FloatTensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.FloatTensor | None = None"}, {"name": "output_type", "val": ": str = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "callback_on_step_end", "val": ": typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "clean_caption", "val": ": bool = True"}, {"name": "mask_feature", "val": ": bool = True"}, {"name": "enable_temporal_attentions", "val": ": bool = True"}, {"name": "decode_chunk_size", "val": ": int = 14"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the video generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the video generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is
  less than `1`).
- **num_inference_steps** (`int`, *optional*, defaults to 100) --
  The number of denoising steps. More denoising steps usually lead to a higher quality video at the
  expense of slower inference.
- **timesteps** (`list[int]`, *optional*) --
  Custom timesteps to use for the denoising process. If not defined, equal spaced `num_inference_steps`
  timesteps are used. Must be in descending order.
- **guidance_scale** (`float`, *optional*, defaults to 7.0) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2.
  of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting
  `guidance_scale > 1`. Higher guidance scale encourages to generate videos that are closely linked to
  the text `prompt`, usually at the expense of lower video quality.
- **video_length** (`int`, *optional*, defaults to 16) --
  The number of video frames that are generated. Defaults to 16 frames which at 8 frames per seconds
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of videos to generate per prompt.
- **height** (`int`, *optional*, defaults to self.unet.config.sample_size) --
  The height in pixels of the generated video.
- **width** (`int`, *optional*, defaults to self.unet.config.sample_size) --
  The width in pixels of the generated video.
- **eta** (`float`, *optional*, defaults to 0.0) --
  Corresponds to parameter eta (η) in the DDIM paper: https://huggingface.co/papers/2010.02502. Only
  applies to [schedulers.DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), will be ignored for others.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.FloatTensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for video
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **negative_prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated negative text embeddings. For Latte this negative prompt should be "". If not provided,
  negative_prompt_embeds will be generated from `negative_prompt` input argument.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate video. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.stable_diffusion.IFPipelineOutput` instead of a plain tuple.
- **callback_on_step_end** (`Callable[[int, int], None]`, `PipelineCallback`, `MultiPipelineCallbacks`, *optional*) --
  A callback function or a list of callback functions to be called at the end of each denoising step.
- **callback_on_step_end_tensor_inputs** (`list[str]`, *optional*) --
  A list of tensor inputs that should be passed to the callback function. If not defined, all tensor
  inputs will be passed.
- **clean_caption** (`bool`, *optional*, defaults to `True`) --
  Whether or not to clean the caption before creating embeddings. Requires `beautifulsoup4` and `ftfy` to
  be installed. If the dependencies are not installed, the embeddings will be created from the raw
  prompt.
- **mask_feature** (`bool` defaults to `True`) -- If set to `True`, the text embeddings will be masked.
- **enable_temporal_attentions** (`bool`, *optional*, defaults to `True`) -- Whether to enable temporal attentions
- **decode_chunk_size** (`int`, *optional*) --
  The number of frames to decode at a time. Higher chunk size leads to better temporal consistency at the
  expense of more memory usage. By default, the decoder decodes all frames at once for maximal quality.
  For lower memory usage, reduce `decode_chunk_size`.0`LattePipelineOutput` or `tuple`If `return_dict` is `True`, `LattePipelineOutput` is returned,
otherwise a `tuple` is returned where the first element is a list with the generated images

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import LattePipeline
>>> from diffusers.utils import export_to_gif

>>> # You can replace the checkpoint id with "maxin-cn/Latte-1" too.
>>> pipe = LattePipeline.from_pretrained("maxin-cn/Latte-1", torch_dtype=torch.float16)
>>> # Enable memory optimizations.
>>> pipe.enable_model_cpu_offload()

>>> prompt = "A small cactus with a happy face in the Sahara desert."
>>> videos = pipe(prompt).frames[0]
>>> export_to_gif(videos, "latte.gif")
```

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

text_encoder (`T5EncoderModel`) : Frozen text-encoder. Latte uses [T5](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5EncoderModel), specifically the [t5-v1_1-xxl](https://huggingface.co/PixArt-alpha/PixArt-alpha/tree/main/t5-v1_1-xxl) variant.

tokenizer (`T5Tokenizer`) : Tokenizer of class [T5Tokenizer](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5Tokenizer).

transformer ([LatteTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/latte_transformer3d#diffusers.LatteTransformer3DModel)) : A text conditioned `LatteTransformer3DModel` to denoise the encoded video latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `transformer` to denoise the encoded video latents.

**Returns:**

``LattePipelineOutput` or `tuple``

If `return_dict` is `True`, `LattePipelineOutput` is returned,
otherwise a `tuple` is returned where the first element is a list with the generated images
#### encode_prompt[[diffusers.LattePipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/latte/pipeline_latte.py#L206)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt not to guide the video generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`). For Latte, this should be "".

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : whether to use classifier free guidance or not

num_images_per_prompt (`int`, *optional*, defaults to 1) : number of video that should be generated per prompt

device : (`torch.device`, *optional*): torch device to place the resulting embeddings on

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. For Latte, it's should be the embeddings of the "" string.

clean_caption (bool, defaults to `False`) : If `True`, the function will preprocess and clean the provided caption before encoding.

mask_feature : (bool, defaults to `True`): If `True`, the function will mask the text embeddings.
