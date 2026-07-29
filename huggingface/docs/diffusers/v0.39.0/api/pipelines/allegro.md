# Allegro

[Allegro: Open the Black Box of Commercial-Level Video Generation Model](https://huggingface.co/papers/2410.15458) from RhymesAI, by Yuan Zhou, Qiuyue Wang, Yuxuan Cai, Huan Yang.

The abstract from the paper is:

*Significant advancements have been made in the field of video generation, with the open-source community contributing a wealth of research papers and tools for training high-quality models. However, despite these efforts, the available information and resources remain insufficient for achieving commercial-level performance. In this report, we open the black box and introduce Allegro, an advanced video generation model that excels in both quality and temporal consistency. We also highlight the current limitations in the field and present a comprehensive methodology for training high-performance, commercial-level video generation models, addressing key aspects such as data, model architecture, training pipeline, and evaluation. Our user study shows that Allegro surpasses existing open-source models and most commercial models, ranking just behind Hailuo and Kling. Code: https://github.com/rhymes-ai/Allegro , Model: https://huggingface.co/rhymes-ai/Allegro , Gallery: https://rhymes.ai/allegro_gallery .*

> [!TIP]
> Make sure to check out the Schedulers [guide](../../using-diffusers/schedulers) to learn how to explore the tradeoff between scheduler speed and quality, and see the [reuse components across pipelines](../../using-diffusers/loading#reuse-a-pipeline) section to learn how to efficiently load the same components into multiple pipelines.

## Quantization

Quantization helps reduce the memory requirements of very large models by storing model weights in a lower precision data type. However, quantization may have varying impact on video quality depending on the video model.

Refer to the [Quantization](../../quantization/overview) overview to learn more about supported quantization backends and selecting a quantization backend that supports your use case. The example below demonstrates how to load a quantized [AllegroPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/allegro#diffusers.AllegroPipeline) for inference with bitsandbytes.

```py
import torch
from diffusers import BitsAndBytesConfig as DiffusersBitsAndBytesConfig, AllegroTransformer3DModel, AllegroPipeline
from diffusers.utils import export_to_video
from transformers import BitsAndBytesConfig as BitsAndBytesConfig, T5EncoderModel

quant_config = BitsAndBytesConfig(load_in_8bit=True)
text_encoder_8bit = T5EncoderModel.from_pretrained(
    "rhymes-ai/Allegro",
    subfolder="text_encoder",
    quantization_config=quant_config,
    torch_dtype=torch.float16,
)

quant_config = DiffusersBitsAndBytesConfig(load_in_8bit=True)
transformer_8bit = AllegroTransformer3DModel.from_pretrained(
    "rhymes-ai/Allegro",
    subfolder="transformer",
    quantization_config=quant_config,
    torch_dtype=torch.float16,
)

pipeline = AllegroPipeline.from_pretrained(
    "rhymes-ai/Allegro",
    text_encoder=text_encoder_8bit,
    transformer=transformer_8bit,
    torch_dtype=torch.float16,
    device_map="balanced",
)

prompt = (
    "A seaside harbor with bright sunlight and sparkling seawater, with many boats in the water. From an aerial view, "
    "the boats vary in size and color, some moving and some stationary. Fishing boats in the water suggest that this "
    "location might be a popular spot for docking fishing boats."
)
video = pipeline(prompt, guidance_scale=7.5, max_sequence_length=512).frames[0]
export_to_video(video, "harbor.mp4", fps=15)
```

## AllegroPipeline[[diffusers.AllegroPipeline]]

#### diffusers.AllegroPipeline[[diffusers.AllegroPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/allegro/pipeline_allegro.py#L144)

Pipeline for text-to-video generation using Allegro.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

__call__diffusers.AllegroPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/allegro/pipeline_allegro.py#L718[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str = ''"}, {"name": "num_inference_steps", "val": ": int = 100"}, {"name": "timesteps", "val": ": list = None"}, {"name": "guidance_scale", "val": ": float = 7.5"}, {"name": "num_frames", "val": ": int | None = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_videos_per_prompt", "val": ": int = 1"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "callback_on_step_end", "val": ": typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "clean_caption", "val": ": bool = True"}, {"name": "max_sequence_length", "val": ": int = 512"}]- **prompt** (`str` or `list[str]`, *optional*) --
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
- **guidance_scale** (`float`, *optional*, defaults to 7.5) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2.
  of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting
  `guidance_scale > 1`. Higher guidance scale encourages to generate videos that are closely linked to
  the text `prompt`, usually at the expense of lower video quality.
- **num_videos_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of videos to generate per prompt.
- **num_frames** -- (`int`, *optional*, defaults to 88):
  The number controls the generated video frames.
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
- **latents** (`torch.Tensor`, *optional*) --
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for video
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
  The output format of the generate video. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.stable_diffusion.IFPipelineOutput` instead of a plain tuple.
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that is called at the end of each denoising step during inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`List`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.
- **clean_caption** (`bool`, *optional*, defaults to `True`) --
  Whether or not to clean the caption before creating embeddings. Requires `beautifulsoup4` and `ftfy` to
  be installed. If the dependencies are not installed, the embeddings will be created from the raw
  prompt.
- **max_sequence_length** (`int` defaults to `512`) --
  Maximum sequence length to use with the `prompt`.0[AllegroPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/allegro#diffusers.pipelines.allegro.pipeline_output.AllegroPipelineOutput) or `tuple`If `return_dict` is `True`, [AllegroPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/allegro#diffusers.pipelines.allegro.pipeline_output.AllegroPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated videos.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import AutoencoderKLAllegro, AllegroPipeline
>>> from diffusers.utils import export_to_video

>>> vae = AutoencoderKLAllegro.from_pretrained("rhymes-ai/Allegro", subfolder="vae", torch_dtype=torch.float32)
>>> pipe = AllegroPipeline.from_pretrained("rhymes-ai/Allegro", vae=vae, torch_dtype=torch.bfloat16).to("cuda")
>>> pipe.enable_vae_tiling()

>>> prompt = (
...     "A seaside harbor with bright sunlight and sparkling seawater, with many boats in the water. From an aerial view, "
...     "the boats vary in size and color, some moving and some stationary. Fishing boats in the water suggest that this "
...     "location might be a popular spot for docking fishing boats."
... )
>>> video = pipe(prompt, guidance_scale=7.5, max_sequence_length=512).frames[0]
>>> export_to_video(video, "output.mp4", fps=15)
```

**Parameters:**

vae (`AllegroAutoEncoderKL3D`) : Variational Auto-Encoder (VAE) Model to encode and decode video to and from latent representations.

text_encoder (`T5EncoderModel`) : Frozen text-encoder. PixArt-Alpha uses [T5](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5EncoderModel), specifically the [t5-v1_1-xxl](https://huggingface.co/PixArt-alpha/PixArt-alpha/tree/main/t5-v1_1-xxl) variant.

tokenizer (`T5Tokenizer`) : Tokenizer of class [T5Tokenizer](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5Tokenizer).

transformer ([AllegroTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/allegro_transformer3d#diffusers.AllegroTransformer3DModel)) : A text conditioned `AllegroTransformer3DModel` to denoise the encoded video latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `transformer` to denoise the encoded video latents.

**Returns:**

`[AllegroPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/allegro#diffusers.pipelines.allegro.pipeline_output.AllegroPipelineOutput) or `tuple``

If `return_dict` is `True`, [AllegroPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/allegro#diffusers.pipelines.allegro.pipeline_output.AllegroPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated videos.
#### disable_vae_slicing[[diffusers.AllegroPipeline.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/allegro/pipeline_allegro.py#L662)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### disable_vae_tiling[[diffusers.AllegroPipeline.disable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/allegro/pipeline_allegro.py#L689)

Disable tiled VAE decoding. If `enable_vae_tiling` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_vae_slicing[[diffusers.AllegroPipeline.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/allegro/pipeline_allegro.py#L649)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### enable_vae_tiling[[diffusers.AllegroPipeline.enable_vae_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/allegro/pipeline_allegro.py#L675)

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.
#### encode_prompt[[diffusers.AllegroPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/allegro/pipeline_allegro.py#L215)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`). For PixArt-Alpha, this should be "".

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : whether to use classifier free guidance or not

num_videos_per_prompt (`int`, *optional*, defaults to 1) : number of images that should be generated per prompt

device : (`torch.device`, *optional*): torch device to place the resulting embeddings on

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. For PixArt-Alpha, it's should be the embeddings of the "" string.

clean_caption (`bool`, defaults to `False`) : If `True`, the function will preprocess and clean the provided caption before encoding.

max_sequence_length (`int`, defaults to 512) : Maximum sequence length to use for the prompt.

## AllegroPipelineOutput[[diffusers.pipelines.allegro.pipeline_output.AllegroPipelineOutput]]

#### diffusers.pipelines.allegro.pipeline_output.AllegroPipelineOutput[[diffusers.pipelines.allegro.pipeline_output.AllegroPipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/allegro/pipeline_output.py#L11)

Output class for Allegro pipelines.

**Parameters:**

frames (`torch.Tensor`, `np.ndarray`, or list[list[PIL.Image.Image]]) : list of video outputs - It can be a nested list of length `batch_size,` with each sub-list containing denoised PIL image sequences of length `num_frames.` It can also be a NumPy array or Torch tensor of shape `(batch_size, num_frames, channels, height, width)`.
