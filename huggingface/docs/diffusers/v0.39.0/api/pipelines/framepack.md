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

# Framepack

  

[Packing Input Frame Context in Next-Frame Prediction Models for Video Generation](https://huggingface.co/papers/2504.12626) by Lvmin Zhang and Maneesh Agrawala.

*We present a neural network structure, FramePack, to train next-frame (or next-frame-section) prediction models for video generation. The FramePack compresses input frames to make the transformer context length a fixed number regardless of the video length. As a result, we are able to process a large number of frames using video diffusion with computation bottleneck similar to image diffusion. This also makes the training video batch sizes significantly higher (batch sizes become comparable to image diffusion training). We also propose an anti-drifting sampling method that generates frames in inverted temporal order with early-established endpoints to avoid exposure bias (error accumulation over iterations). Finally, we show that existing video diffusion models can be finetuned with FramePack, and their visual quality may be improved because the next-frame prediction supports more balanced diffusion schedulers with less extreme flow shift timesteps.*

> [!TIP]
> Make sure to check out the Schedulers [guide](../../using-diffusers/schedulers) to learn how to explore the tradeoff between scheduler speed and quality, and see the [reuse components across pipelines](../../using-diffusers/loading#reuse-a-pipeline) section to learn how to efficiently load the same components into multiple pipelines.

## Available models

| Model name | Description |
|:---|:---|
- [`lllyasviel/FramePackI2V_HY`](https://huggingface.co/lllyasviel/FramePackI2V_HY) | Trained with the "inverted anti-drifting" strategy as described in the paper. Inference requires setting `sampling_type="inverted_anti_drifting"` when running the pipeline. |
- [`lllyasviel/FramePack_F1_I2V_HY_20250503`](https://huggingface.co/lllyasviel/FramePack_F1_I2V_HY_20250503) | Trained with a novel anti-drifting strategy but inference is performed in "vanilla" strategy as described in the paper. Inference requires setting `sampling_type="vanilla"` when running the pipeline. |

## Usage

Refer to the pipeline documentation for basic usage examples. The following section contains examples of offloading, different sampling methods, quantization, and more.

### First and last frame to video

The following example shows how to use Framepack with start and end image controls, using the inverted anti-drifiting sampling model.

```python
import torch
from diffusers import HunyuanVideoFramepackPipeline, HunyuanVideoFramepackTransformer3DModel
from diffusers.utils import export_to_video, load_image
from transformers import SiglipImageProcessor, SiglipVisionModel

transformer = HunyuanVideoFramepackTransformer3DModel.from_pretrained(
    "lllyasviel/FramePackI2V_HY", torch_dtype=torch.bfloat16
)
feature_extractor = SiglipImageProcessor.from_pretrained(
    "lllyasviel/flux_redux_bfl", subfolder="feature_extractor"
)
image_encoder = SiglipVisionModel.from_pretrained(
    "lllyasviel/flux_redux_bfl", subfolder="image_encoder", torch_dtype=torch.float16
)
pipe = HunyuanVideoFramepackPipeline.from_pretrained(
    "hunyuanvideo-community/HunyuanVideo",
    transformer=transformer,
    feature_extractor=feature_extractor,
    image_encoder=image_encoder,
    torch_dtype=torch.float16,
)

# Enable memory optimizations
pipe.enable_model_cpu_offload()
pipe.vae.enable_tiling()

prompt = "CG animation style, a small blue bird takes off from the ground, flapping its wings. The bird's feathers are delicate, with a unique pattern on its chest. The background shows a blue sky with white clouds under bright sunshine. The camera follows the bird upward, capturing its flight and the vastness of the sky from a close-up, low-angle perspective."
first_image = load_image(
    "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/flf2v_input_first_frame.png"
)
last_image = load_image(
    "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/flf2v_input_last_frame.png"
)
output = pipe(
    image=first_image,
    last_image=last_image,
    prompt=prompt,
    height=512,
    width=512,
    num_frames=91,
    num_inference_steps=30,
    guidance_scale=9.0,
    generator=torch.Generator().manual_seed(0),
    sampling_type="inverted_anti_drifting",
).frames[0]
export_to_video(output, "output.mp4", fps=30)
```

### Vanilla sampling

The following example shows how to use Framepack with the F1 model trained with vanilla sampling but new regulation approach for anti-drifting.

```python
import torch
from diffusers import HunyuanVideoFramepackPipeline, HunyuanVideoFramepackTransformer3DModel
from diffusers.utils import export_to_video, load_image
from transformers import SiglipImageProcessor, SiglipVisionModel

transformer = HunyuanVideoFramepackTransformer3DModel.from_pretrained(
    "lllyasviel/FramePack_F1_I2V_HY_20250503", torch_dtype=torch.bfloat16
)
feature_extractor = SiglipImageProcessor.from_pretrained(
    "lllyasviel/flux_redux_bfl", subfolder="feature_extractor"
)
image_encoder = SiglipVisionModel.from_pretrained(
    "lllyasviel/flux_redux_bfl", subfolder="image_encoder", torch_dtype=torch.float16
)
pipe = HunyuanVideoFramepackPipeline.from_pretrained(
    "hunyuanvideo-community/HunyuanVideo",
    transformer=transformer,
    feature_extractor=feature_extractor,
    image_encoder=image_encoder,
    torch_dtype=torch.float16,
)

# Enable memory optimizations
pipe.enable_model_cpu_offload()
pipe.vae.enable_tiling()

image = load_image(
    "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/penguin.png"
)
output = pipe(
    image=image,
    prompt="A penguin dancing in the snow",
    height=832,
    width=480,
    num_frames=91,
    num_inference_steps=30,
    guidance_scale=9.0,
    generator=torch.Generator().manual_seed(0),
    sampling_type="vanilla",
).frames[0]
export_to_video(output, "output.mp4", fps=30)
```

### Group offloading

Group offloading ([apply_group_offloading()](/docs/diffusers/v0.39.0/en/api/utilities#diffusers.hooks.apply_group_offloading)) provides aggressive memory optimizations for offloading internal parts of any model to the CPU, with possibly no additional overhead to generation time. If you have very low VRAM available, this approach may be suitable for you depending on the amount of CPU RAM available.

```python
import torch
from diffusers import HunyuanVideoFramepackPipeline, HunyuanVideoFramepackTransformer3DModel
from diffusers.hooks import apply_group_offloading
from diffusers.utils import export_to_video, load_image
from transformers import SiglipImageProcessor, SiglipVisionModel

transformer = HunyuanVideoFramepackTransformer3DModel.from_pretrained(
    "lllyasviel/FramePack_F1_I2V_HY_20250503", torch_dtype=torch.bfloat16
)
feature_extractor = SiglipImageProcessor.from_pretrained(
    "lllyasviel/flux_redux_bfl", subfolder="feature_extractor"
)
image_encoder = SiglipVisionModel.from_pretrained(
    "lllyasviel/flux_redux_bfl", subfolder="image_encoder", torch_dtype=torch.float16
)
pipe = HunyuanVideoFramepackPipeline.from_pretrained(
    "hunyuanvideo-community/HunyuanVideo",
    transformer=transformer,
    feature_extractor=feature_extractor,
    image_encoder=image_encoder,
    torch_dtype=torch.float16,
)

# Enable group offloading
onload_device = torch.device("cuda")
offload_device = torch.device("cpu")
list(map(
    lambda x: apply_group_offloading(x, onload_device, offload_device, offload_type="leaf_level", use_stream=True, low_cpu_mem_usage=True),
    [pipe.text_encoder, pipe.text_encoder_2, pipe.transformer]
))
pipe.image_encoder.to(onload_device)
pipe.vae.to(onload_device)
pipe.vae.enable_tiling()

image = load_image(
    "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/penguin.png"
)
output = pipe(
    image=image,
    prompt="A penguin dancing in the snow",
    height=832,
    width=480,
    num_frames=91,
    num_inference_steps=30,
    guidance_scale=9.0,
    generator=torch.Generator().manual_seed(0),
    sampling_type="vanilla",
).frames[0]
print(f"Max memory: {torch.cuda.max_memory_allocated() / 1024**3:.3f} GB")
export_to_video(output, "output.mp4", fps=30)
```

## HunyuanVideoFramepackPipeline[[diffusers.HunyuanVideoFramepackPipeline]]

#### diffusers.HunyuanVideoFramepackPipeline[[diffusers.HunyuanVideoFramepackPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/hunyuan_video/pipeline_hunyuan_video_framepack.py#L243)

Pipeline for text-to-video generation using HunyuanVideo.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

__call__diffusers.HunyuanVideoFramepackPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/hunyuan_video/pipeline_hunyuan_video_framepack.py#L639[{"name": "image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor]"}, {"name": "last_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "prompt", "val": ": str | list[str] = None"}, {"name": "prompt_2", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt_2", "val": ": str | list[str] = None"}, {"name": "height", "val": ": int = 720"}, {"name": "width", "val": ": int = 1280"}, {"name": "num_frames", "val": ": int = 129"}, {"name": "latent_window_size", "val": ": int = 9"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "sigmas", "val": ": list = None"}, {"name": "true_cfg_scale", "val": ": float = 1.0"}, {"name": "guidance_scale", "val": ": float = 6.0"}, {"name": "num_videos_per_prompt", "val": ": int | None = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "image_latents", "val": ": torch.Tensor | None = None"}, {"name": "last_image_latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "pooled_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_pooled_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "prompt_template", "val": ": dict = {'template': 'system\\n\\nDescribe the video by detailing the following aspects: 1. The main content and theme of the video.2. The color, shape, size, texture, quantity, text, and spatial relationships of the objects.3. Actions, events, behaviors temporal relationships, physical movement changes of the objects.4. background environment, light, style and atmosphere.5. camera angles, movements, and transitions used in the video:user\\n\\n{}', 'crop_start': 95}"}, {"name": "max_sequence_length", "val": ": int = 256"}, {"name": "sampling_type", "val": ": FramepackSamplingType = "}]- **image** (`PIL.Image.Image` or `np.ndarray` or `torch.Tensor`) --
  The image to be used as the starting point for the video generation.
- **last_image** (`PIL.Image.Image` or `np.ndarray` or `torch.Tensor`, *optional*) --
  The optional last image to be used as the ending point for the video generation. This is useful for
  generating transitions between two images.
- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **prompt_2** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is
  will be used instead.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `true_cfg_scale` is
  not greater than `1`).
- **negative_prompt_2** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation to be sent to `tokenizer_2` and
  `text_encoder_2`. If not defined, `negative_prompt` is used in all the text-encoders.
- **height** (`int`, defaults to `720`) --
  The height in pixels of the generated image.
- **width** (`int`, defaults to `1280`) --
  The width in pixels of the generated image.
- **num_frames** (`int`, defaults to `129`) --
  The number of frames in the generated video.
- **latent_window_size** (`int`, defaults to `9`) --
  Number of latent frames produced per Framepack sampling window.
- **num_inference_steps** (`int`, defaults to `50`) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **sigmas** (`list[float]`, *optional*) --
  Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in
  their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed
  will be used.
- **true_cfg_scale** (`float`, *optional*, defaults to 1.0) --
  When > 1.0 and a provided `negative_prompt`, enables true classifier-free guidance.
- **guidance_scale** (`float`, defaults to `6.0`) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2.
  of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting
  `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to
  the text `prompt`, usually at the expense of lower image quality. Note that the only available
  HunyuanVideo model is CFG-distilled, which means that traditional guidance between unconditional and
  conditional latent is not applied.
- **num_videos_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make
  generation deterministic.
- **image_latents** (`torch.Tensor`, *optional*) --
  Pre-encoded image latents. If not provided, the image will be encoded using the VAE.
- **last_image_latents** (`torch.Tensor`, *optional*) --
  Pre-encoded last image latents. If not provided, the last image will be encoded using the VAE.
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not
  provided, text embeddings are generated from the `prompt` input argument.
- **pooled_prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting.
  If not provided, pooled text embeddings will be generated from `prompt` input argument.
- **negative_prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
  argument.
- **negative_pooled_prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, pooled negative_prompt_embeds will be generated from `negative_prompt`
  input argument.
- **prompt_attention_mask** (`torch.Tensor`, *optional*) --
  Attention mask for `prompt_embeds`. Required when `prompt_embeds` is passed directly.
- **negative_prompt_attention_mask** (`torch.Tensor`, *optional*) --
  Attention mask for `negative_prompt_embeds`. Required when `negative_prompt_embeds` is passed directly.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generated image. Choose between `PIL.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `HunyuanVideoFramepackPipelineOutput` instead of a plain tuple.
- **attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **prompt_template** (`dict`, *optional*) --
  Template used to format the prompt before encoding. Defaults to the model's default template.
- **max_sequence_length** (`int`, *optional*, defaults to 256) --
  Maximum sequence length to use for the prompt encoder.
- **sampling_type** (`FramepackSamplingType`, *optional*) --
  The Framepack sampling strategy to use when iterating over latent windows.
- **callback_on_step_end** (`Callable`, `PipelineCallback`, `MultiPipelineCallbacks`, *optional*) --
  A function or a subclass of `PipelineCallback` or `MultiPipelineCallbacks` that is called at the end of
  each denoising step during the inference. with the following arguments: `callback_on_step_end(self:
  DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a
  list of all tensors as specified by `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`list`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.0`~HunyuanVideoFramepackPipelineOutput` or `tuple`If `return_dict` is `True`, `HunyuanVideoFramepackPipelineOutput` is returned, otherwise a `tuple` is
returned where the first element is a list with the generated images and the second element is a list
of `bool`s indicating whether the corresponding generated image contains "not-safe-for-work" (nsfw)
content.

The call function to the pipeline for generation.

Examples:
##### Image-to-Video

```python
>>> import torch
>>> from diffusers import HunyuanVideoFramepackPipeline, HunyuanVideoFramepackTransformer3DModel
>>> from diffusers.utils import export_to_video, load_image
>>> from transformers import SiglipImageProcessor, SiglipVisionModel

>>> transformer = HunyuanVideoFramepackTransformer3DModel.from_pretrained(
...     "lllyasviel/FramePackI2V_HY", torch_dtype=torch.bfloat16
... )
>>> feature_extractor = SiglipImageProcessor.from_pretrained(
...     "lllyasviel/flux_redux_bfl", subfolder="feature_extractor"
... )
>>> image_encoder = SiglipVisionModel.from_pretrained(
...     "lllyasviel/flux_redux_bfl", subfolder="image_encoder", torch_dtype=torch.float16
... )
>>> pipe = HunyuanVideoFramepackPipeline.from_pretrained(
...     "hunyuanvideo-community/HunyuanVideo",
...     transformer=transformer,
...     feature_extractor=feature_extractor,
...     image_encoder=image_encoder,
...     torch_dtype=torch.float16,
... )
>>> pipe.vae.enable_tiling()
>>> pipe.to("cuda")

>>> image = load_image(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/penguin.png"
... )
>>> output = pipe(
...     image=image,
...     prompt="A penguin dancing in the snow",
...     height=832,
...     width=480,
...     num_frames=91,
...     num_inference_steps=30,
...     guidance_scale=9.0,
...     generator=torch.Generator().manual_seed(0),
...     sampling_type="inverted_anti_drifting",
... ).frames[0]
>>> export_to_video(output, "output.mp4", fps=30)
```

##### First and Last Image-to-Video

```python
>>> import torch
>>> from diffusers import HunyuanVideoFramepackPipeline, HunyuanVideoFramepackTransformer3DModel
>>> from diffusers.utils import export_to_video, load_image
>>> from transformers import SiglipImageProcessor, SiglipVisionModel

>>> transformer = HunyuanVideoFramepackTransformer3DModel.from_pretrained(
...     "lllyasviel/FramePackI2V_HY", torch_dtype=torch.bfloat16
... )
>>> feature_extractor = SiglipImageProcessor.from_pretrained(
...     "lllyasviel/flux_redux_bfl", subfolder="feature_extractor"
... )
>>> image_encoder = SiglipVisionModel.from_pretrained(
...     "lllyasviel/flux_redux_bfl", subfolder="image_encoder", torch_dtype=torch.float16
... )
>>> pipe = HunyuanVideoFramepackPipeline.from_pretrained(
...     "hunyuanvideo-community/HunyuanVideo",
...     transformer=transformer,
...     feature_extractor=feature_extractor,
...     image_encoder=image_encoder,
...     torch_dtype=torch.float16,
... )
>>> pipe.to("cuda")

>>> prompt = "CG animation style, a small blue bird takes off from the ground, flapping its wings. The bird's feathers are delicate, with a unique pattern on its chest. The background shows a blue sky with white clouds under bright sunshine. The camera follows the bird upward, capturing its flight and the vastness of the sky from a close-up, low-angle perspective."
>>> first_image = load_image(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/flf2v_input_first_frame.png"
... )
>>> last_image = load_image(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/flf2v_input_last_frame.png"
... )
>>> output = pipe(
...     image=first_image,
...     last_image=last_image,
...     prompt=prompt,
...     height=512,
...     width=512,
...     num_frames=91,
...     num_inference_steps=30,
...     guidance_scale=9.0,
...     generator=torch.Generator().manual_seed(0),
...     sampling_type="inverted_anti_drifting",
... ).frames[0]
>>> export_to_video(output, "output.mp4", fps=30)
```

**Parameters:**

text_encoder (`LlamaModel`) : [Llava Llama3-8B](https://huggingface.co/xtuner/llava-llama-3-8b-v1_1-transformers).

tokenizer (`LlamaTokenizer`) : Tokenizer from [Llava Llama3-8B](https://huggingface.co/xtuner/llava-llama-3-8b-v1_1-transformers).

transformer ([HunyuanVideoTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/hunyuan_video_transformer_3d#diffusers.HunyuanVideoTransformer3DModel)) : Conditional Transformer to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLHunyuanVideo](/docs/diffusers/v0.39.0/en/api/models/autoencoder_kl_hunyuan_video#diffusers.AutoencoderKLHunyuanVideo)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

text_encoder_2 (`CLIPTextModel`) : [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

tokenizer_2 (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

**Returns:**

``~HunyuanVideoFramepackPipelineOutput` or `tuple``

If `return_dict` is `True`, `HunyuanVideoFramepackPipelineOutput` is returned, otherwise a `tuple` is
returned where the first element is a list with the generated images and the second element is a list
of `bool`s indicating whether the corresponding generated image contains "not-safe-for-work" (nsfw)
content.
#### disable_vae_slicing[[diffusers.HunyuanVideoFramepackPipeline.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/hunyuan_video/pipeline_hunyuan_video_framepack.py#L579)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### disable_vae_tiling[[diffusers.HunyuanVideoFramepackPipeline.disable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/hunyuan_video/pipeline_hunyuan_video_framepack.py#L606)

Disable tiled VAE decoding. If `enable_vae_tiling` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_vae_slicing[[diffusers.HunyuanVideoFramepackPipeline.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/hunyuan_video/pipeline_hunyuan_video_framepack.py#L566)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### enable_vae_tiling[[diffusers.HunyuanVideoFramepackPipeline.enable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/hunyuan_video/pipeline_hunyuan_video_framepack.py#L592)

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.

## HunyuanVideoPipelineOutput[[diffusers.pipelines.hunyuan_video.pipeline_output.HunyuanVideoPipelineOutput]]

#### diffusers.pipelines.hunyuan_video.pipeline_output.HunyuanVideoPipelineOutput[[diffusers.pipelines.hunyuan_video.pipeline_output.HunyuanVideoPipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/hunyuan_video/pipeline_output.py#L11)

Output class for HunyuanVideo pipelines.

**Parameters:**

frames (`torch.Tensor`, `np.ndarray`, or list[list[PIL.Image.Image]]) : list of video outputs - It can be a nested list of length `batch_size,` with each sub-list containing denoised PIL image sequences of length `num_frames.` It can also be a NumPy array or Torch tensor of shape `(batch_size, num_frames, channels, height, width)`.
