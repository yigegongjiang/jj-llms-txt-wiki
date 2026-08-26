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

  
    
      
    
  

# HunyuanVideo

[HunyuanVideo](https://huggingface.co/papers/2412.03603) is a 13B parameter diffusion transformer model designed to be competitive with closed-source video foundation models and enable wider community access. This model uses a "dual-stream to single-stream" architecture to separately process the video and text tokens first, before concatenating and feeding them to the transformer to fuse the multimodal information. A pretrained multimodal large language model (MLLM) is used as the encoder because it has better image-text alignment, better image detail description and reasoning, and it can be used as a zero-shot learner if system instructions are added to user prompts. Finally, HunyuanVideo uses a 3D causal variational autoencoder to more efficiently process video data at the original resolution and frame rate.

You can find all the original HunyuanVideo checkpoints under the [Tencent](https://huggingface.co/tencent) organization.

> [!TIP]
> Click on the HunyuanVideo models in the right sidebar for more examples of video generation tasks.
>
> The examples below use a checkpoint from [hunyuanvideo-community](https://huggingface.co/hunyuanvideo-community) because the weights are stored in a layout compatible with Diffusers.

The example below demonstrates how to generate a video optimized for memory or inference speed.

Refer to the [Reduce memory usage](../../optimization/memory) guide for more details about the various memory saving techniques.

The quantized HunyuanVideo model below requires ~14GB of VRAM.

```py
import torch
from diffusers import AutoModel, HunyuanVideoPipeline
from diffusers.quantizers import PipelineQuantizationConfig
from diffusers.utils import export_to_video

# quantize weights to int4 with bitsandbytes
pipeline_quant_config = PipelineQuantizationConfig(
    quant_backend="bitsandbytes_4bit",
    quant_kwargs={
      "load_in_4bit": True,
      "bnb_4bit_quant_type": "nf4",
      "bnb_4bit_compute_dtype": torch.bfloat16
      },
    components_to_quantize="transformer"
)

pipeline = HunyuanVideoPipeline.from_pretrained(
    "hunyuanvideo-community/HunyuanVideo",
    quantization_config=pipeline_quant_config,
    dtype=torch.bfloat16,
)

# model-offloading and tiling
pipeline.enable_model_cpu_offload()
pipeline.vae.enable_tiling()

prompt = "A fluffy teddy bear sits on a bed of soft pillows surrounded by children's toys."
video = pipeline(prompt=prompt, num_frames=61, num_inference_steps=30).frames[0]
export_to_video(video, "output.mp4", fps=15)
```

[Compilation](../../optimization/fp16#torchcompile) is slow the first time but subsequent calls to the pipeline are faster.

```py
import torch
from diffusers import AutoModel, HunyuanVideoPipeline
from diffusers.quantizers import PipelineQuantizationConfig
from diffusers.utils import export_to_video

# quantize weights to int4 with bitsandbytes
pipeline_quant_config = PipelineQuantizationConfig(
    quant_backend="bitsandbytes_4bit",
    quant_kwargs={
      "load_in_4bit": True,
      "bnb_4bit_quant_type": "nf4",
      "bnb_4bit_compute_dtype": torch.bfloat16
      },
    components_to_quantize="transformer"
)

pipeline = HunyuanVideoPipeline.from_pretrained(
    "hunyuanvideo-community/HunyuanVideo",
    quantization_config=pipeline_quant_config,
    dtype=torch.bfloat16,
)

# model-offloading and tiling
pipeline.enable_model_cpu_offload()
pipeline.vae.enable_tiling()

# torch.compile
pipeline.transformer.to(memory_format=torch.channels_last)
pipeline.transformer = torch.compile(
    pipeline.transformer, mode="max-autotune", fullgraph=True
)

prompt = "A fluffy teddy bear sits on a bed of soft pillows surrounded by children's toys."
video = pipeline(prompt=prompt, num_frames=61, num_inference_steps=30).frames[0]
export_to_video(video, "output.mp4", fps=15)
```

## Notes

- HunyuanVideo supports LoRAs with [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.HunyuanVideoLoraLoaderMixin.load_lora_weights).

  
  Show example code

  ```py
  import torch
  from diffusers import AutoModel, HunyuanVideoPipeline
  from diffusers.quantizers import PipelineQuantizationConfig
  from diffusers.utils import export_to_video

  # quantize weights to int4 with bitsandbytes
  pipeline_quant_config = PipelineQuantizationConfig(
      quant_backend="bitsandbytes_4bit",
      quant_kwargs={
        "load_in_4bit": True,
        "bnb_4bit_quant_type": "nf4",
        "bnb_4bit_compute_dtype": torch.bfloat16
        },
      components_to_quantize="transformer"
  )

  pipeline = HunyuanVideoPipeline.from_pretrained(
      "hunyuanvideo-community/HunyuanVideo",
      quantization_config=pipeline_quant_config,
      dtype=torch.bfloat16,
  )

  # load LoRA weights
  pipeline.load_lora_weights("https://huggingface.co/lucataco/hunyuan-steamboat-willie-10", adapter_name="steamboat-willie")
  pipeline.set_adapters("steamboat-willie", 0.9)

  # model-offloading and tiling
  pipeline.enable_model_cpu_offload()
  pipeline.vae.enable_tiling()

  # use "In the style of SWR" to trigger the LoRA
  prompt = """
  In the style of SWR. A black and white animated scene featuring a fluffy teddy bear sits on a bed of soft pillows surrounded by children's toys.
  """
  video = pipeline(prompt=prompt, num_frames=61, num_inference_steps=30).frames[0]
  export_to_video(video, "output.mp4", fps=15)
  ```

  

- Refer to the table below for recommended inference values.

  | parameter | recommended value |
  |---|---|
  | text encoder dtype | `torch.float16` |
  | transformer dtype | `torch.bfloat16` |
  | vae dtype | `torch.float16` |
  | `num_frames (k)` | 4 * `k` + 1 |

- Try lower `shift` values (`2.0` to `5.0`) for lower resolution videos and higher `shift` values (`7.0` to `12.0`) for higher resolution images.

## HunyuanVideoPipeline[[diffusers.HunyuanVideoPipeline]]

#### diffusers.HunyuanVideoPipeline[[diffusers.HunyuanVideoPipeline]]

```python
diffusers.HunyuanVideoPipeline(text_encoder: LlamaModel, tokenizer: LlamaTokenizer, transformer: HunyuanVideoTransformer3DModel, vae: AutoencoderKLHunyuanVideo, scheduler: FlowMatchEulerDiscreteScheduler, text_encoder_2: CLIPTextModel, tokenizer_2: CLIPTokenizer)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/hunyuan_video/pipeline_hunyuan_video.py#L144)

**Parameters:**

text_encoder (`LlamaModel`) : [Llava Llama3-8B](https://huggingface.co/xtuner/llava-llama-3-8b-v1_1-transformers).

tokenizer (`LlamaTokenizer`) : Tokenizer from [Llava Llama3-8B](https://huggingface.co/xtuner/llava-llama-3-8b-v1_1-transformers).

transformer ([HunyuanVideoTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/hunyuan_video_transformer_3d#diffusers.HunyuanVideoTransformer3DModel)) : Conditional Transformer to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLHunyuanVideo](/docs/diffusers/v0.40.0/en/api/models/autoencoder_kl_hunyuan_video#diffusers.AutoencoderKLHunyuanVideo)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

text_encoder_2 (`CLIPTextModel`) : [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

tokenizer_2 (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

Pipeline for text-to-video generation using HunyuanVideo.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

#### __call__[[diffusers.HunyuanVideoPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, prompt_2: str | list[str] = None, negative_prompt: str | list[str] = None, negative_prompt_2: str | list[str] = None, height: int = 720, width: int = 1280, num_frames: int = 129, num_inference_steps: int = 50, sigmas: list = None, true_cfg_scale: float = 1.0, guidance_scale: float = 6.0, num_videos_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, pooled_prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, negative_pooled_prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None, callback_on_step_end_tensor_inputs: list = ['latents'], prompt_template: dict = {'template': '<|start_header_id|>system<|end_header_id|>\n\nDescribe the video by detailing the following aspects: 1. The main content and theme of the video.2. The color, shape, size, texture, quantity, text, and spatial relationships of the objects.3. Actions, events, behaviors temporal relationships, physical movement changes of the objects.4. background environment, light, style and atmosphere.5. camera angles, movements, and transitions used in the video:<|eot_id|><|start_header_id|>user<|end_header_id|>\n\n{}<|eot_id|>', 'crop_start': 95}, max_sequence_length: int = 256)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/hunyuan_video/pipeline_hunyuan_video.py#L438)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is will be used instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `true_cfg_scale` is not greater than `1`).

negative_prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `negative_prompt` is used in all the text-encoders.

height (`int`, defaults to `720`) : The height in pixels of the generated image.

width (`int`, defaults to `1280`) : The width in pixels of the generated image.

num_frames (`int`, defaults to `129`) : The number of frames in the generated video.

num_inference_steps (`int`, defaults to `50`) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`list[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

true_cfg_scale (`float`, *optional*, defaults to 1.0) : True classifier-free guidance (guidance scale) is enabled when `true_cfg_scale` > 1 and `negative_prompt` is provided.

guidance_scale (`float`, defaults to `6.0`) : Embedded guiddance scale is enabled by setting `guidance_scale` > 1. Higher `guidance_scale` encourages a model to generate images more aligned with `prompt` at the expense of lower image quality.  Guidance-distilled models approximates true classifer-free guidance for `guidance_scale` > 1. Refer to the [paper](https://huggingface.co/papers/2210.03142) to learn more.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor is generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, text embeddings are generated from the `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

negative_pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled negative_prompt_embeds will be generated from `negative_prompt` input argument.

prompt_attention_mask (`torch.Tensor`, *optional*) : Attention mask for `prompt_embeds`. Required when `prompt_embeds` is passed directly.

negative_prompt_attention_mask (`torch.Tensor`, *optional*) : Attention mask for `negative_prompt_embeds`. Required when `negative_prompt_embeds` is passed directly.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generated image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `HunyuanVideoPipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

prompt_template (`dict`, *optional*) : Template used to format the prompt before encoding. Defaults to the model's default template.

max_sequence_length (`int`, *optional*, defaults to 256) : Maximum sequence length to use for the prompt encoder.

callback_on_step_end (`Callable`, `PipelineCallback`, `MultiPipelineCallbacks`, *optional*) : A function or a subclass of `PipelineCallback` or `MultiPipelineCallbacks` that is called at the end of each denoising step during the inference. with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

**Returns:** `~HunyuanVideoPipelineOutput` or `tuple`

If `return_dict` is `True`, `HunyuanVideoPipelineOutput` is returned, otherwise a `tuple` is returned
where the first element is a list with the generated images and the second element is a list of `bool`s
indicating whether the corresponding generated image contains "not-safe-for-work" (nsfw) content.

The call function to the pipeline for generation.

Examples:
```python
>>> import torch
>>> from diffusers import HunyuanVideoPipeline, HunyuanVideoTransformer3DModel
>>> from diffusers.utils import export_to_video

>>> model_id = "hunyuanvideo-community/HunyuanVideo"
>>> transformer = HunyuanVideoTransformer3DModel.from_pretrained(
...     model_id, subfolder="transformer", torch_dtype=torch.bfloat16
... )
>>> pipe = HunyuanVideoPipeline.from_pretrained(model_id, transformer=transformer, torch_dtype=torch.float16)
>>> pipe.vae.enable_tiling()
>>> pipe.to("cuda")

>>> output = pipe(
...     prompt="A cat walks on the grass, realistic",
...     height=320,
...     width=512,
...     num_frames=61,
...     num_inference_steps=30,
... ).frames[0]
>>> export_to_video(output, "output.mp4", fps=15)
```

## HunyuanVideoPipelineOutput[[diffusers.pipelines.hunyuan_video.pipeline_output.HunyuanVideoPipelineOutput]]

#### diffusers.pipelines.hunyuan_video.pipeline_output.HunyuanVideoPipelineOutput[[diffusers.pipelines.hunyuan_video.pipeline_output.HunyuanVideoPipelineOutput]]

```python
diffusers.pipelines.hunyuan_video.pipeline_output.HunyuanVideoPipelineOutput(frames: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/hunyuan_video/pipeline_output.py#L11)

**Parameters:**

frames (`torch.Tensor`, `np.ndarray`, or list[list[PIL.Image.Image]]) : list of video outputs - It can be a nested list of length `batch_size,` with each sub-list containing denoised PIL image sequences of length `num_frames.` It can also be a NumPy array or Torch tensor of shape `(batch_size, num_frames, channels, height, width)`.

Output class for HunyuanVideo pipelines.
