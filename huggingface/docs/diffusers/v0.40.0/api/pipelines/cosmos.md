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

# Cosmos

[Cosmos World Foundation Model Platform for Physical AI](https://huggingface.co/papers/2501.03575) by NVIDIA.

*Physical AI needs to be trained digitally first. It needs a digital twin of itself, the policy model, and a digital twin of the world, the world model. In this paper, we present the Cosmos World Foundation Model Platform to help developers build customized world models for their Physical AI setups. We position a world foundation model as a general-purpose world model that can be fine-tuned into customized world models for downstream applications. Our platform covers a video curation pipeline, pre-trained world foundation models, examples of post-training of pre-trained world foundation models, and video tokenizers. To help Physical AI builders solve the most critical problems of our society, we make our platform open-source and our models open-weight with permissive licenses available via https://github.com/NVIDIA/Cosmos.*

> [!TIP]
> Make sure to check out the Schedulers [guide](../../using-diffusers/schedulers) to learn how to explore the tradeoff between scheduler speed and quality, and see the [reuse components across pipelines](../../using-diffusers/loading#reuse-a-pipeline) section to learn how to efficiently load the same components into multiple pipelines.

## Basic usage

```python
import torch
from diffusers import Cosmos2_5_PredictBasePipeline
from diffusers.utils import export_to_video

model_id = "nvidia/Cosmos-Predict2.5-2B"
pipe = Cosmos2_5_PredictBasePipeline.from_pretrained(
    model_id, revision="diffusers/base/post-trained", dtype=torch.bfloat16
)
pipe.to("cuda")

prompt = "As the red light shifts to green, the red bus at the intersection begins to move forward, its headlights cutting through the falling snow. The snowy tire tracks deepen as the vehicle inches ahead, casting fresh lines onto the slushy road. Around it, streetlights glow warmer, illuminating the drifting flakes and wet reflections on the asphalt. Other cars behind start to edge forward, their beams joining the scene. The stillness of the urban street transitions into motion as the quiet snowfall is punctuated by the slow advance of traffic through the frosty city corridor."
negative_prompt = "The video captures a series of frames showing ugly scenes, static with no motion, motion blur, over-saturation, shaky footage, low resolution, grainy texture, pixelated images, poorly lit areas, underexposed and overexposed scenes, poor color balance, washed out colors, choppy sequences, jerky movements, low frame rate, artifacting, color banding, unnatural transitions, outdated special effects, fake elements, unconvincing visuals, poorly edited content, jump cuts, visual noise, and flickering. Overall, the video is of poor quality."

output = pipe(
    image=None,
    video=None,
    prompt=prompt,
    negative_prompt=negative_prompt,
    num_frames=93,
    generator=torch.Generator().manual_seed(1),
).frames[0]
export_to_video(output, "text2world.mp4", fps=16)
```

## Cosmos2_5_TransferPipeline[[diffusers.Cosmos2_5_TransferPipeline]]

#### diffusers.Cosmos2_5_TransferPipeline[[diffusers.Cosmos2_5_TransferPipeline]]

```python
diffusers.Cosmos2_5_TransferPipeline(text_encoder: Qwen2_5_VLForConditionalGeneration, tokenizer: AutoTokenizer, transformer: CosmosTransformer3DModel, vae: AutoencoderKLWan, scheduler: UniPCMultistepScheduler, controlnet: CosmosControlNetModel, safety_checker: typing.Optional[diffusers.pipelines.cosmos.pipeline_cosmos2_5_transfer.CosmosSafetyChecker] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cosmos/pipeline_cosmos2_5_transfer.py#L152)

**Parameters:**

text_encoder (`Qwen2_5_VLForConditionalGeneration`) : Frozen text-encoder. Cosmos Transfer2.5 uses the [Qwen2.5 VL](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct) encoder.

tokenizer (`AutoTokenizer`) : Tokenizer associated with the Qwen2.5 VL encoder.

transformer ([CosmosTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/cosmos_transformer3d#diffusers.CosmosTransformer3DModel)) : Conditional Transformer to denoise the encoded image latents.

scheduler ([UniPCMultistepScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/unipc#diffusers.UniPCMultistepScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLWan](/docs/diffusers/v0.40.0/en/api/models/autoencoder_kl_wan#diffusers.AutoencoderKLWan)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

controlnet (`CosmosControlNetModel`) : ControlNet used to condition generation on control inputs.

Pipeline for Cosmos Transfer2.5, supporting auto-regressive inference.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

#### __call__[[diffusers.Cosmos2_5_TransferPipeline.__call__]]

```python
__call__(controls: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], typing.List[typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]]], controls_conditioning_scale: typing.Union[float, typing.List[float]] = 1.0, prompt: typing.Union[str, typing.List[str], NoneType] = None, negative_prompt: typing.Union[str, typing.List[str]] = 'The video captures a series of frames showing ugly scenes, static with no motion, motion blur, over-saturation, shaky footage, low resolution, grainy texture, pixelated images, poorly lit areas, underexposed and overexposed scenes, poor color balance, washed out colors, choppy sequences, jerky movements, low frame rate, artifacting, color banding, unnatural transitions, outdated special effects, fake elements, unconvincing visuals, poorly edited content, jump cuts, visual noise, and flickering. Overall, the video is of poor quality.', height: int = 704, width: typing.Optional[int] = None, num_frames: typing.Optional[int] = None, num_frames_per_chunk: int = 93, num_inference_steps: int = 36, guidance_scale: float = 3.0, num_videos_per_prompt: int = 1, generator: typing.Union[torch.Generator, typing.List[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, output_type: typing.Optional[str] = 'pil', return_dict: bool = True, callback_on_step_end: typing.Union[typing.Callable[[int, int, typing.Dict], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None, callback_on_step_end_tensor_inputs: typing.List[str] = ['latents'], max_sequence_length: int = 512, conditional_frame_timestep: float = 0.1, num_ar_conditional_frames: typing.Optional[int] = 1, num_ar_latent_conditional_frames: typing.Optional[int] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cosmos/pipeline_cosmos2_5_transfer.py#L569)

**Parameters:**

controls (`PipelineImageInput`, `List[PipelineImageInput]`) : Control image or video input used by the ControlNet.

controls_conditioning_scale (`float` or `List[float]`, *optional*, defaults to `1.0`) : The scale factor(s) for the ControlNet outputs. A single float is broadcast to all control blocks.

prompt (`str` or `List[str]`, *optional*) : The prompt or prompts to guide generation. Required unless `prompt_embeds` is supplied.

negative_prompt (`str` or `List[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is not greater than `1`).

height (`int`, defaults to `704`) : The height in pixels of the generated image.

width (`int`, *optional*) : The width in pixels of the generated image. If not provided, this will be determined based on the aspect ratio of the input and the provided height.

num_frames (`int`, *optional*) : Number of output frames. Defaults to `None` to output the same number of frames as the input `controls`.

num_frames_per_chunk (`int`, *optional*, defaults to `93`) : Number of frames generated per auto-regressive chunk. When the total number of frames exceeds this value, generation is split into multiple chunks using a sliding-window approach.

num_inference_steps (`int`, defaults to `36`) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, defaults to `3.0`) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `List[torch.Generator]`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor is generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. For PixArt-Sigma this negative prompt should be "". If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generated image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `CosmosPipelineOutput` instead of a plain tuple.

callback_on_step_end (`Callable`, `PipelineCallback`, `MultiPipelineCallbacks`, *optional*) : A function or a subclass of `PipelineCallback` or `MultiPipelineCallbacks` that is called at the end of each denoising step during the inference. with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`List`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int`, defaults to `512`) : The maximum number of tokens in the prompt. If the prompt exceeds this length, it will be truncated. If the prompt is shorter than this length, it will be padded.

conditional_frame_timestep (`float`, *optional*, defaults to 0.1) : Timestep value used for the conditional frames during denoising. Must be in the `[0, 1]` interval.

num_ar_conditional_frames (`int`, *optional*, defaults to `1`) : Number of frames to condition on subsequent inference loops in auto-regressive inference, i.e. for the second chunk and onwards. Only used if `num_ar_latent_conditional_frames` is `None`.  This is only used when auto-regressive inference is performed, i.e. when the number of frames in controls is > num_frames_per_chunk

num_ar_latent_conditional_frames (`int`, *optional*) : Number of latent frames to condition on subsequent inference loops in auto-regressive inference, i.e. for the second chunk and onwards. Only used if `num_ar_conditional_frames` is `None`.  This is only used when auto-regressive inference is performed, i.e. when the number of frames in controls is > num_frames_per_chunk

**Returns:** `~CosmosPipelineOutput` or `tuple`

If `return_dict` is `True`, `CosmosPipelineOutput` is returned, otherwise a `tuple` is returned where
the first element is a list with the generated images and the second element is a list of `bool`s
indicating whether the corresponding generated image contains "not-safe-for-work" (nsfw) content.

`controls` drive the conditioning through ControlNet. Controls are assumed to be pre-processed, e.g. edge maps
are pre-computed.

Setting `num_frames` will restrict the total number of frames output, if not provided or assigned to None
(default) then the number of output frames will match the input `controls`.

Auto-regressive inference is supported and thus a sliding window of `num_frames_per_chunk` frames are used per
denoising loop. In addition, when auto-regressive inference is performed, the previous
`num_ar_latent_conditional_frames` or `num_ar_conditional_frames` are used to condition the following denoising
inference loops.

Examples:
```python
>>> import cv2
>>> import numpy as np
>>> from PIL import Image
>>> import torch
>>> from diffusers import Cosmos2_5_TransferPipeline, AutoModel
>>> from diffusers.utils import export_to_video, load_video

>>> model_id = "nvidia/Cosmos-Transfer2.5-2B"
>>> # Load a Transfer2.5 controlnet variant (edge, depth, seg, or blur)
>>> controlnet = AutoModel.from_pretrained(
...     model_id, revision="diffusers/controlnet/general/edge", torch_dtype=torch.bfloat16
... )
>>> pipe = Cosmos2_5_TransferPipeline.from_pretrained(
...     model_id, controlnet=controlnet, revision="diffusers/general", torch_dtype=torch.bfloat16
... )
>>> pipe = pipe.to("cuda")

>>> # Video2World with edge control: Generate video guided by edge maps extracted from input video.
>>> prompt = (
...     "The video is a demonstration of robotic manipulation, likely in a laboratory or testing environment. It"
...     "features two robotic arms interacting with a piece of blue fabric. The setting is a room with a beige"
...     "couch in the background, providing a neutral backdrop for the robotic activity. The robotic arms are"
...     "positioned on either side of the fabric, which is placed on a yellow cushion. The left robotic arm is"
...     "white with a black gripper, while the right arm is black with a more complex, articulated gripper. At the"
...     "beginning, the fabric is laid out on the cushion. The left robotic arm approaches the fabric, its gripper"
...     "opening and closing as it positions itself. The right arm remains stationary initially, poised to assist."
...     "As the video progresses, the left arm grips the fabric, lifting it slightly off the cushion. The right arm"
...     "then moves in, its gripper adjusting to grasp the opposite side of the fabric. Both arms work in"
...     "coordination, lifting and holding the fabric between them. The fabric is manipulated with precision,"
...     "showcasing the dexterity and control of the robotic arms. The camera remains static throughout, focusing"
...     "on the interaction between the robotic arms and the fabric, allowing viewers to observe the detailed"
...     "movements and coordination involved in the task."
... )
>>> negative_prompt = (
...     "The video captures a series of frames showing ugly scenes, static with no motion, motion blur, "
...     "over-saturation, shaky footage, low resolution, grainy texture, pixelated images, poorly lit areas, "
...     "underexposed and overexposed scenes, poor color balance, washed out colors, choppy sequences, jerky "
...     "movements, low frame rate, artifacting, color banding, unnatural transitions, outdated special effects, "
...     "fake elements, unconvincing visuals, poorly edited content, jump cuts, visual noise, and flickering. "
...     "Overall, the video is of poor quality."
... )
>>> input_video = load_video(
...     "https://github.com/nvidia-cosmos/cosmos-transfer2.5/raw/refs/heads/main/assets/robot_example/robot_input.mp4"
... )
>>> num_frames = 93

>>> # Extract edge maps from the input video using Canny edge detection
>>> edge_maps = [
...     cv2.Canny(cv2.cvtColor(np.array(frame.convert("RGB")), cv2.COLOR_RGB2BGR), 100, 200)
...     for frame in input_video[:num_frames]
... ]
>>> edge_maps = np.stack(edge_maps)[None]  # (T, H, W) -> (1, T, H, W)
>>> controls = torch.from_numpy(edge_maps).expand(3, -1, -1, -1)  # (1, T, H, W) -> (3, T, H, W)
>>> controls = [Image.fromarray(x.numpy()) for x in controls.permute(1, 2, 3, 0)]
>>> export_to_video(controls, "edge_controlled_video_edge.mp4", fps=30)

>>> # Transfer inference with controls.
>>> video = pipe(
...     controls=controls,
...     controls_conditioning_scale=1.0,
...     prompt=prompt,
...     negative_prompt=negative_prompt,
...     num_frames=num_frames,
... ).frames[0]
>>> export_to_video(video, "edge_controlled_video.mp4", fps=30)
```

#### encode_prompt[[diffusers.Cosmos2_5_TransferPipeline.encode_prompt]]

```python
encode_prompt(prompt: typing.Union[str, typing.List[str]], negative_prompt: typing.Union[str, typing.List[str], NoneType] = None, do_classifier_free_guidance: bool = True, num_videos_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 512, device: typing.Optional[torch.device] = None, dtype: typing.Optional[torch.dtype] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cosmos/pipeline_cosmos2_5_transfer.py#L296)

**Parameters:**

prompt (`str` or `List[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `List[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : Whether to use classifier free guidance or not.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : Number of videos that should be generated per prompt. torch device to place the resulting embeddings on

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

device : (`torch.device`, *optional*): torch device

dtype : (`torch.dtype`, *optional*): torch dtype

Encodes the prompt into text encoder hidden states.

## Cosmos2_5_PredictBasePipeline[[diffusers.Cosmos2_5_PredictBasePipeline]]

#### diffusers.Cosmos2_5_PredictBasePipeline[[diffusers.Cosmos2_5_PredictBasePipeline]]

```python
diffusers.Cosmos2_5_PredictBasePipeline(text_encoder: Qwen2_5_VLForConditionalGeneration, tokenizer: AutoTokenizer, transformer: CosmosTransformer3DModel, vae: AutoencoderKLWan, scheduler: UniPCMultistepScheduler, safety_checker: CosmosSafetyChecker = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cosmos/pipeline_cosmos2_5_predict.py#L185)

**Parameters:**

text_encoder (`Qwen2_5_VLForConditionalGeneration`) : Frozen text-encoder. Cosmos Predict2.5 uses the [Qwen2.5 VL](https://huggingface.co/Qwen/Qwen2.5-VL-7B-Instruct) encoder.

tokenizer (`AutoTokenizer`) : Tokenizer associated with the Qwen2.5 VL encoder.

transformer ([CosmosTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/cosmos_transformer3d#diffusers.CosmosTransformer3DModel)) : Conditional Transformer to denoise the encoded image latents.

scheduler ([UniPCMultistepScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/unipc#diffusers.UniPCMultistepScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLWan](/docs/diffusers/v0.40.0/en/api/models/autoencoder_kl_wan#diffusers.AutoencoderKLWan)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

Pipeline for [Cosmos Predict2.5](https://github.com/nvidia-cosmos/cosmos-predict2.5) base model.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

#### __call__[[diffusers.Cosmos2_5_PredictBasePipeline.__call__]]

```python
__call__(image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, video: list[typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]] | None = None, prompt: str | list[str] | None = None, negative_prompt: str | list[str] | None = None, height: int = 704, width: int = 1280, num_frames: int = 93, num_inference_steps: int = 36, guidance_scale: float = 7.0, num_videos_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, callback_on_step_end: typing.Optional[typing.Callable[[int, int, NoneType], diffusers.callbacks.PipelineCallback | diffusers.callbacks.MultiPipelineCallbacks]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512, conditional_frame_timestep: float = 0.0001, num_latent_conditional_frames: int = 2)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cosmos/pipeline_cosmos2_5_predict.py#L544)

**Parameters:**

image (`PIL.Image.Image`, `np.ndarray`, `torch.Tensor`, *optional*) : Optional single image for Image2World conditioning. Must be `None` when `video` is provided.

video (`list[PIL.Image.Image]`, `np.ndarray`, `torch.Tensor`, *optional*) : Optional input video for Video2World conditioning. Must be `None` when `image` is provided.

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide generation. Required unless `prompt_embeds` is supplied.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is not greater than `1`).

height (`int`, defaults to `704`) : The height in pixels of the generated image.

width (`int`, defaults to `1280`) : The width in pixels of the generated image.

num_frames (`int`, defaults to `93`) : Number of output frames. Use `93` for world (video) generation; set to `1` to return a single frame.

num_inference_steps (`int`, defaults to `35`) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, defaults to `7.0`) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor is generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. For PixArt-Sigma this negative prompt should be "". If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generated image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `CosmosPipelineOutput` instead of a plain tuple.

callback_on_step_end (`Callable`, `PipelineCallback`, `MultiPipelineCallbacks`, *optional*) : A function or a subclass of `PipelineCallback` or `MultiPipelineCallbacks` that is called at the end of each denoising step during the inference. with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`List`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int`, defaults to `512`) : The maximum number of tokens in the prompt. If the prompt exceeds this length, it will be truncated. If the prompt is shorter than this length, it will be padded.

num_latent_conditional_frames (`int`, defaults to `2`) : Number of latent conditional frames to use for Video2World conditioning. The number of pixel frames extracted from the input video is calculated as `4 * (num_latent_conditional_frames - 1) + 1`. Set to 1 for Image2World-like behavior (single frame conditioning).

conditional_frame_timestep (`float`, *optional*, defaults to 0.0001) : Timestep value used for the conditional frames during denoising.

**Returns:** `~CosmosPipelineOutput` or `tuple`

If `return_dict` is `True`, `CosmosPipelineOutput` is returned, otherwise a `tuple` is returned where
the first element is a list with the generated images and the second element is a list of `bool`s
indicating whether the corresponding generated image contains "not-safe-for-work" (nsfw) content.

The call function to the pipeline for generation. Supports three modes:

- **Text2World**: `image=None`, `video=None`, `prompt` provided. Generates a world clip.
- **Image2World**: `image` provided, `video=None`, `prompt` provided. Conditions on a single frame.
- **Video2World**: `video` provided, `image=None`, `prompt` provided. Conditions on an input clip.

Set `num_frames=93` (default) to produce a world video, or `num_frames=1` to produce a single image frame (the
above in "*2Image mode").

Outputs follow `output_type` (e.g., `"pil"` returns a list of `num_frames` PIL images per prompt).

Examples:
```python
>>> import torch
>>> from diffusers import Cosmos2_5_PredictBasePipeline
>>> from diffusers.utils import export_to_video, load_image, load_video

>>> model_id = "nvidia/Cosmos-Predict2.5-2B"
>>> pipe = Cosmos2_5_PredictBasePipeline.from_pretrained(
...     model_id, revision="diffusers/base/post-trained", torch_dtype=torch.bfloat16
... )
>>> pipe = pipe.to("cuda")

>>> # Common negative prompt reused across modes.
>>> negative_prompt = (
...     "The video captures a series of frames showing ugly scenes, static with no motion, motion blur, "
...     "over-saturation, shaky footage, low resolution, grainy texture, pixelated images, poorly lit areas, "
...     "underexposed and overexposed scenes, poor color balance, washed out colors, choppy sequences, jerky "
...     "movements, low frame rate, artifacting, color banding, unnatural transitions, outdated special effects, "
...     "fake elements, unconvincing visuals, poorly edited content, jump cuts, visual noise, and flickering. "
...     "Overall, the video is of poor quality."
... )

>>> # Text2World: generate a 93-frame world video from text only.
>>> prompt = (
...     "As the red light shifts to green, the red bus at the intersection begins to move forward, its headlights "
...     "cutting through the falling snow. The snowy tire tracks deepen as the vehicle inches ahead, casting fresh "
...     "lines onto the slushy road. Around it, streetlights glow warmer, illuminating the drifting flakes and wet "
...     "reflections on the asphalt. Other cars behind start to edge forward, their beams joining the scene. "
...     "The stillness of the urban street transitions into motion as the quiet snowfall is punctuated by the slow "
...     "advance of traffic through the frosty city corridor."
... )
>>> video = pipe(
...     image=None,
...     video=None,
...     prompt=prompt,
...     negative_prompt=negative_prompt,
...     num_frames=93,
...     generator=torch.Generator().manual_seed(1),
... ).frames[0]
>>> export_to_video(video, "text2world.mp4", fps=16)

>>> # Image2World: condition on a single image and generate a 93-frame world video.
>>> prompt = (
...     "A high-definition video captures the precision of robotic welding in an industrial setting. "
...     "The first frame showcases a robotic arm, equipped with a welding torch, positioned over a large metal structure. "
...     "The welding process is in full swing, with bright sparks and intense light illuminating the scene, creating a vivid "
...     "display of blue and white hues. A significant amount of smoke billows around the welding area, partially obscuring "
...     "the view but emphasizing the heat and activity. The background reveals parts of the workshop environment, including a "
...     "ventilation system and various pieces of machinery, indicating a busy and functional industrial workspace. As the video "
...     "progresses, the robotic arm maintains its steady position, continuing the welding process and moving to its left. "
...     "The welding torch consistently emits sparks and light, and the smoke continues to rise, diffusing slightly as it moves upward. "
...     "The metal surface beneath the torch shows ongoing signs of heating and melting. The scene retains its industrial ambiance, with "
...     "the welding sparks and smoke dominating the visual field, underscoring the ongoing nature of the welding operation."
... )
>>> image = load_image(
...     "https://media.githubusercontent.com/media/nvidia-cosmos/cosmos-predict2.5/refs/heads/main/assets/base/robot_welding.jpg"
... )
>>> video = pipe(
...     image=image,
...     video=None,
...     prompt=prompt,
...     negative_prompt=negative_prompt,
...     num_frames=93,
...     generator=torch.Generator().manual_seed(1),
... ).frames[0]
>>> export_to_video(video, "image2world.mp4", fps=16)

>>> # Video2World: condition on an input clip and predict a 93-frame world video.
>>> prompt = (
...     "The video opens with an aerial view of a large-scale sand mining construction operation, showcasing extensive piles "
...     "of brown sand meticulously arranged in parallel rows. A central water channel, fed by a water pipe, flows through the "
...     "middle of these sand heaps, creating ripples and movement as it cascades down. The surrounding area features dense green "
...     "vegetation on the left, contrasting with the sandy terrain, while a body of water is visible in the background on the right. "
...     "As the video progresses, a piece of heavy machinery, likely a bulldozer, enters the frame from the right, moving slowly along "
...     "the edge of the sand piles. This machinery's presence indicates ongoing construction work in the operation. The final frame "
...     "captures the same scene, with the water continuing its flow and the bulldozer still in motion, maintaining the dynamic yet "
...     "steady pace of the construction activity."
... )
>>> input_video = load_video(
...     "https://github.com/nvidia-cosmos/cosmos-predict2.5/raw/refs/heads/main/assets/base/sand_mining.mp4"
... )
>>> video = pipe(
...     image=None,
...     video=input_video,
...     prompt=prompt,
...     negative_prompt=negative_prompt,
...     num_frames=93,
...     generator=torch.Generator().manual_seed(1),
... ).frames[0]
>>> export_to_video(video, "video2world.mp4", fps=16)

>>> # To produce an image instead of a world (video) clip, set num_frames=1 and
>>> # save the first frame: pipe(..., num_frames=1).frames[0][0].
```

#### encode_prompt[[diffusers.Cosmos2_5_PredictBasePipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] | None = None, do_classifier_free_guidance: bool = True, num_videos_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 512, device: typing.Optional[torch.device] = None, dtype: typing.Optional[torch.dtype] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cosmos/pipeline_cosmos2_5_predict.py#L324)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : Whether to use classifier free guidance or not.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : Number of videos that should be generated per prompt. torch device to place the resulting embeddings on

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

device : (`torch.device`, *optional*): torch device

dtype : (`torch.dtype`, *optional*): torch dtype

Encodes the prompt into text encoder hidden states.

## CosmosTextToWorldPipeline[[diffusers.CosmosTextToWorldPipeline]]

#### diffusers.CosmosTextToWorldPipeline[[diffusers.CosmosTextToWorldPipeline]]

```python
diffusers.CosmosTextToWorldPipeline(text_encoder: T5EncoderModel, tokenizer: T5Tokenizer, transformer: CosmosTransformer3DModel, vae: AutoencoderKLCosmos, scheduler: EDMEulerScheduler, safety_checker: CosmosSafetyChecker = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cosmos/pipeline_cosmos_text2world.py#L140)

**Parameters:**

text_encoder (`T5EncoderModel`) : Frozen text-encoder. Cosmos uses [T5](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5EncoderModel); specifically the [t5-11b](https://huggingface.co/google-t5/t5-11b) variant.

tokenizer (`T5TokenizerFast`) : Tokenizer of class [T5Tokenizer](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5Tokenizer).

transformer ([CosmosTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/cosmos_transformer3d#diffusers.CosmosTransformer3DModel)) : Conditional Transformer to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLCosmos](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl_cosmos#diffusers.AutoencoderKLCosmos)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

Pipeline for text-to-world generation using [Cosmos Predict1](https://github.com/nvidia-cosmos/cosmos-predict1).

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

#### __call__[[diffusers.CosmosTextToWorldPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, negative_prompt: str | list[str] | None = None, height: int = 704, width: int = 1280, num_frames: int = 121, num_inference_steps: int = 36, guidance_scale: float = 7.0, fps: int = 30, num_videos_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, callback_on_step_end: typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cosmos/pipeline_cosmos_text2world.py#L401)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is not greater than `1`).

height (`int`, defaults to `720`) : The height in pixels of the generated image.

width (`int`, defaults to `1280`) : The width in pixels of the generated image.

num_frames (`int`, defaults to `121`) : The number of frames in the generated video.

num_inference_steps (`int`, defaults to `36`) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, defaults to `7.0`) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`.

fps (`int`, defaults to `30`) : The frames per second of the generated video.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor is generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. For PixArt-Sigma this negative prompt should be "". If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generated image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `CosmosPipelineOutput` instead of a plain tuple.

callback_on_step_end (`Callable`, `PipelineCallback`, `MultiPipelineCallbacks`, *optional*) : A function or a subclass of `PipelineCallback` or `MultiPipelineCallbacks` that is called at the end of each denoising step during the inference. with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int`, defaults to `512`) : The maximum number of tokens in the prompt. If the prompt exceeds this length, it will be truncated. If the prompt is shorter than this length, it will be padded.

**Returns:** `~CosmosPipelineOutput` or `tuple`

If `return_dict` is `True`, `CosmosPipelineOutput` is returned, otherwise a `tuple` is returned where
the first element is a list with the generated images and the second element is a list of `bool`s
indicating whether the corresponding generated image contains "not-safe-for-work" (nsfw) content.

The call function to the pipeline for generation.

Examples:
```python
>>> import torch
>>> from diffusers import CosmosTextToWorldPipeline
>>> from diffusers.utils import export_to_video

>>> model_id = "nvidia/Cosmos-1.0-Diffusion-7B-Text2World"
>>> pipe = CosmosTextToWorldPipeline.from_pretrained(model_id, torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> prompt = "A sleek, humanoid robot stands in a vast warehouse filled with neatly stacked cardboard boxes on industrial shelves. The robot's metallic body gleams under the bright, even lighting, highlighting its futuristic design and intricate joints. A glowing blue light emanates from its chest, adding a touch of advanced technology. The background is dominated by rows of boxes, suggesting a highly organized storage system. The floor is lined with wooden pallets, enhancing the industrial setting. The camera remains static, capturing the robot's poised stance amidst the orderly environment, with a shallow depth of field that keeps the focus on the robot while subtly blurring the background for a cinematic effect."

>>> output = pipe(prompt=prompt).frames[0]
>>> export_to_video(output, "output.mp4", fps=30)
```

#### encode_prompt[[diffusers.CosmosTextToWorldPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] | None = None, do_classifier_free_guidance: bool = True, num_videos_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 512, device: typing.Optional[torch.device] = None, dtype: typing.Optional[torch.dtype] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cosmos/pipeline_cosmos_text2world.py#L239)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : Whether to use classifier free guidance or not.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : Number of videos that should be generated per prompt. torch device to place the resulting embeddings on

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

device : (`torch.device`, *optional*): torch device

dtype : (`torch.dtype`, *optional*): torch dtype

Encodes the prompt into text encoder hidden states.

## CosmosVideoToWorldPipeline[[diffusers.CosmosVideoToWorldPipeline]]

#### diffusers.CosmosVideoToWorldPipeline[[diffusers.CosmosVideoToWorldPipeline]]

```python
diffusers.CosmosVideoToWorldPipeline(text_encoder: T5EncoderModel, tokenizer: T5Tokenizer, transformer: CosmosTransformer3DModel, vae: AutoencoderKLCosmos, scheduler: EDMEulerScheduler, safety_checker: CosmosSafetyChecker = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cosmos/pipeline_cosmos_video2world.py#L183)

**Parameters:**

text_encoder (`T5EncoderModel`) : Frozen text-encoder. Cosmos uses [T5](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5EncoderModel); specifically the [t5-11b](https://huggingface.co/google-t5/t5-11b) variant.

tokenizer (`T5TokenizerFast`) : Tokenizer of class [T5Tokenizer](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5Tokenizer).

transformer ([CosmosTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/cosmos_transformer3d#diffusers.CosmosTransformer3DModel)) : Conditional Transformer to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLCosmos](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl_cosmos#diffusers.AutoencoderKLCosmos)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

Pipeline for image-to-world and video-to-world generation using [Cosmos
Predict-1](https://github.com/nvidia-cosmos/cosmos-predict1).

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

#### __call__[[diffusers.CosmosVideoToWorldPipeline.__call__]]

```python
__call__(image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, video: list = None, prompt: str | list[str] = None, negative_prompt: str | list[str] | None = None, height: int = 704, width: int = 1280, num_frames: int = 121, num_inference_steps: int = 36, guidance_scale: float = 7.0, input_frames_guidance: bool = False, augment_sigma: float = 0.001, fps: int = 30, num_videos_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, callback_on_step_end: typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cosmos/pipeline_cosmos_video2world.py#L513)

**Parameters:**

image (`PIL.Image.Image`, `np.ndarray`, `torch.Tensor`, *optional*) : The image to be used as a conditioning input for the video generation.

video (`list[PIL.Image.Image]`, `np.ndarray`, `torch.Tensor`, *optional*) : The video to be used as a conditioning input for the video generation.

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is not greater than `1`).

height (`int`, defaults to `720`) : The height in pixels of the generated image.

width (`int`, defaults to `1280`) : The width in pixels of the generated image.

num_frames (`int`, defaults to `121`) : The number of frames in the generated video.

num_inference_steps (`int`, defaults to `36`) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, defaults to `7.0`) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`.

input_frames_guidance (`bool`, *optional*, defaults to `False`) : Whether to apply guidance on the conditional input frames.

augment_sigma (`float`, *optional*, defaults to 0.001) : Sigma value used to augment the conditional latents during denoising.

fps (`int`, defaults to `30`) : The frames per second of the generated video.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor is generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. For PixArt-Sigma this negative prompt should be "". If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generated image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `CosmosPipelineOutput` instead of a plain tuple.

callback_on_step_end (`Callable`, `PipelineCallback`, `MultiPipelineCallbacks`, *optional*) : A function or a subclass of `PipelineCallback` or `MultiPipelineCallbacks` that is called at the end of each denoising step during the inference. with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int`, defaults to `512`) : The maximum number of tokens in the prompt. If the prompt exceeds this length, it will be truncated. If the prompt is shorter than this length, it will be padded.

**Returns:** `~CosmosPipelineOutput` or `tuple`

If `return_dict` is `True`, `CosmosPipelineOutput` is returned, otherwise a `tuple` is returned where
the first element is a list with the generated images and the second element is a list of `bool`s
indicating whether the corresponding generated image contains "not-safe-for-work" (nsfw) content.

The call function to the pipeline for generation.

Examples:

Image conditioning:

```python
>>> import torch
>>> from diffusers import CosmosVideoToWorldPipeline
>>> from diffusers.utils import export_to_video, load_image

>>> model_id = "nvidia/Cosmos-1.0-Diffusion-7B-Video2World"
>>> pipe = CosmosVideoToWorldPipeline.from_pretrained(model_id, torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> prompt = "The video depicts a long, straight highway stretching into the distance, flanked by metal guardrails. The road is divided into multiple lanes, with a few vehicles visible in the far distance. The surrounding landscape features dry, grassy fields on one side and rolling hills on the other. The sky is mostly clear with a few scattered clouds, suggesting a bright, sunny day."
>>> image = load_image(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/cosmos/cosmos-video2world-input.jpg"
... )

>>> video = pipe(image=image, prompt=prompt).frames[0]
>>> export_to_video(video, "output.mp4", fps=30)
```

Video conditioning:

```python
>>> import torch
>>> from diffusers import CosmosVideoToWorldPipeline
>>> from diffusers.utils import export_to_video, load_video

>>> model_id = "nvidia/Cosmos-1.0-Diffusion-7B-Video2World"
>>> pipe = CosmosVideoToWorldPipeline.from_pretrained(model_id, torch_dtype=torch.bfloat16)
>>> pipe.transformer = torch.compile(pipe.transformer)
>>> pipe.to("cuda")

>>> prompt = "The video depicts a winding mountain road covered in snow, with a single vehicle traveling along it. The road is flanked by steep, rocky cliffs and sparse vegetation. The landscape is characterized by rugged terrain and a river visible in the distance. The scene captures the solitude and beauty of a winter drive through a mountainous region."
>>> video = load_video(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/cosmos/cosmos-video2world-input-vid.mp4"
... )[
...     :21
... ]  # This example uses only the first 21 frames

>>> video = pipe(video=video, prompt=prompt).frames[0]
>>> export_to_video(video, "output.mp4", fps=30)
```

#### encode_prompt[[diffusers.CosmosVideoToWorldPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] | None = None, do_classifier_free_guidance: bool = True, num_videos_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 512, device: typing.Optional[torch.device] = None, dtype: typing.Optional[torch.dtype] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cosmos/pipeline_cosmos_video2world.py#L285)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : Whether to use classifier free guidance or not.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : Number of videos that should be generated per prompt. torch device to place the resulting embeddings on

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

device : (`torch.device`, *optional*): torch device

dtype : (`torch.dtype`, *optional*): torch dtype

Encodes the prompt into text encoder hidden states.

## Cosmos2TextToImagePipeline[[diffusers.Cosmos2TextToImagePipeline]]

#### diffusers.Cosmos2TextToImagePipeline[[diffusers.Cosmos2TextToImagePipeline]]

```python
diffusers.Cosmos2TextToImagePipeline(text_encoder: T5EncoderModel, tokenizer: T5Tokenizer, transformer: CosmosTransformer3DModel, vae: AutoencoderKLWan, scheduler: FlowMatchEulerDiscreteScheduler, safety_checker: CosmosSafetyChecker = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cosmos/pipeline_cosmos2_text2image.py#L143)

**Parameters:**

text_encoder (`T5EncoderModel`) : Frozen text-encoder. Cosmos uses [T5](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5EncoderModel); specifically the [t5-11b](https://huggingface.co/google-t5/t5-11b) variant.

tokenizer (`T5TokenizerFast`) : Tokenizer of class [T5Tokenizer](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5Tokenizer).

transformer ([CosmosTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/cosmos_transformer3d#diffusers.CosmosTransformer3DModel)) : Conditional Transformer to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLWan](/docs/diffusers/v0.40.0/en/api/models/autoencoder_kl_wan#diffusers.AutoencoderKLWan)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

Pipeline for text-to-image generation using [Cosmos Predict2](https://github.com/nvidia-cosmos/cosmos-predict2).

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

#### __call__[[diffusers.Cosmos2TextToImagePipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, negative_prompt: str | list[str] | None = None, height: int = 768, width: int = 1360, num_inference_steps: int = 35, guidance_scale: float = 7.0, num_images_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, callback_on_step_end: typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cosmos/pipeline_cosmos2_text2image.py#L417)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is not greater than `1`).

height (`int`, defaults to `768`) : The height in pixels of the generated image.

width (`int`, defaults to `1360`) : The width in pixels of the generated image.

num_inference_steps (`int`, defaults to `35`) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, defaults to `7.0`) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor is generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. For PixArt-Sigma this negative prompt should be "". If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generated image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `CosmosImagePipelineOutput` instead of a plain tuple.

callback_on_step_end (`Callable`, `PipelineCallback`, `MultiPipelineCallbacks`, *optional*) : A function or a subclass of `PipelineCallback` or `MultiPipelineCallbacks` that is called at the end of each denoising step during the inference. with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int`, defaults to `512`) : The maximum number of tokens in the prompt. If the prompt exceeds this length, it will be truncated. If the prompt is shorter than this length, it will be padded.

**Returns:** `~CosmosImagePipelineOutput` or `tuple`

If `return_dict` is `True`, `CosmosImagePipelineOutput` is returned, otherwise a `tuple` is returned
where the first element is a list with the generated images and the second element is a list of `bool`s
indicating whether the corresponding generated image contains "not-safe-for-work" (nsfw) content.

The call function to the pipeline for generation.

Examples:
```python
>>> import torch
>>> from diffusers import Cosmos2TextToImagePipeline

>>> # Available checkpoints: nvidia/Cosmos-Predict2-2B-Text2Image, nvidia/Cosmos-Predict2-14B-Text2Image
>>> model_id = "nvidia/Cosmos-Predict2-2B-Text2Image"
>>> pipe = Cosmos2TextToImagePipeline.from_pretrained(model_id, torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> prompt = "A close-up shot captures a vibrant yellow scrubber vigorously working on a grimy plate, its bristles moving in circular motions to lift stubborn grease and food residue. The dish, once covered in remnants of a hearty meal, gradually reveals its original glossy surface. Suds form and bubble around the scrubber, creating a satisfying visual of cleanliness in progress. The sound of scrubbing fills the air, accompanied by the gentle clinking of the dish against the sink. As the scrubber continues its task, the dish transforms, gleaming under the bright kitchen lights, symbolizing the triumph of cleanliness over mess."
>>> negative_prompt = "The video captures a series of frames showing ugly scenes, static with no motion, motion blur, over-saturation, shaky footage, low resolution, grainy texture, pixelated images, poorly lit areas, underexposed and overexposed scenes, poor color balance, washed out colors, choppy sequences, jerky movements, low frame rate, artifacting, color banding, unnatural transitions, outdated special effects, fake elements, unconvincing visuals, poorly edited content, jump cuts, visual noise, and flickering. Overall, the video is of poor quality."

>>> output = pipe(
...     prompt=prompt, negative_prompt=negative_prompt, generator=torch.Generator().manual_seed(1)
... ).images[0]
>>> output.save("output.png")
```

#### encode_prompt[[diffusers.Cosmos2TextToImagePipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] | None = None, do_classifier_free_guidance: bool = True, num_images_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 512, device: typing.Optional[torch.device] = None, dtype: typing.Optional[torch.dtype] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cosmos/pipeline_cosmos2_text2image.py#L254)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : Whether to use classifier free guidance or not.

num_images_per_prompt (`int`, *optional*, defaults to 1) : Number of videos that should be generated per prompt. torch device to place the resulting embeddings on

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

device : (`torch.device`, *optional*): torch device

dtype : (`torch.dtype`, *optional*): torch dtype

Encodes the prompt into text encoder hidden states.

## Cosmos2VideoToWorldPipeline[[diffusers.Cosmos2VideoToWorldPipeline]]

#### diffusers.Cosmos2VideoToWorldPipeline[[diffusers.Cosmos2VideoToWorldPipeline]]

```python
diffusers.Cosmos2VideoToWorldPipeline(text_encoder: T5EncoderModel, tokenizer: T5Tokenizer, transformer: CosmosTransformer3DModel, vae: AutoencoderKLWan, scheduler: FlowMatchEulerDiscreteScheduler, safety_checker: CosmosSafetyChecker = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cosmos/pipeline_cosmos2_video2world.py#L162)

**Parameters:**

text_encoder (`T5EncoderModel`) : Frozen text-encoder. Cosmos uses [T5](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5EncoderModel); specifically the [t5-11b](https://huggingface.co/google-t5/t5-11b) variant.

tokenizer (`T5TokenizerFast`) : Tokenizer of class [T5Tokenizer](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5Tokenizer).

transformer ([CosmosTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/cosmos_transformer3d#diffusers.CosmosTransformer3DModel)) : Conditional Transformer to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLWan](/docs/diffusers/v0.40.0/en/api/models/autoencoder_kl_wan#diffusers.AutoencoderKLWan)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

Pipeline for video-to-world generation using [Cosmos Predict2](https://github.com/nvidia-cosmos/cosmos-predict2).

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

#### __call__[[diffusers.Cosmos2VideoToWorldPipeline.__call__]]

```python
__call__(image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, video: list = None, prompt: str | list[str] = None, negative_prompt: str | list[str] | None = None, height: int = 704, width: int = 1280, num_frames: int = 93, num_inference_steps: int = 35, guidance_scale: float = 7.0, fps: int = 16, num_videos_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, callback_on_step_end: typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512, sigma_conditioning: float = 0.0001)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cosmos/pipeline_cosmos2_video2world.py#L485)

**Parameters:**

image (`PIL.Image.Image`, `np.ndarray`, `torch.Tensor`, *optional*) : The image to be used as a conditioning input for the video generation.

video (`list[PIL.Image.Image]`, `np.ndarray`, `torch.Tensor`, *optional*) : The video to be used as a conditioning input for the video generation.

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is not greater than `1`).

height (`int`, defaults to `704`) : The height in pixels of the generated image.

width (`int`, defaults to `1280`) : The width in pixels of the generated image.

num_frames (`int`, defaults to `93`) : The number of frames in the generated video.

num_inference_steps (`int`, defaults to `35`) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, defaults to `7.0`) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`.

fps (`int`, defaults to `16`) : The frames per second of the generated video.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor is generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. For PixArt-Sigma this negative prompt should be "". If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generated image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `CosmosPipelineOutput` instead of a plain tuple.

callback_on_step_end (`Callable`, `PipelineCallback`, `MultiPipelineCallbacks`, *optional*) : A function or a subclass of `PipelineCallback` or `MultiPipelineCallbacks` that is called at the end of each denoising step during the inference. with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int`, defaults to `512`) : The maximum number of tokens in the prompt. If the prompt exceeds this length, it will be truncated. If the prompt is shorter than this length, it will be padded.

sigma_conditioning (`float`, defaults to `0.0001`) : The sigma value used for scaling conditioning latents. Ideally, it should not be changed or should be set to a small value close to zero.

**Returns:** `~CosmosPipelineOutput` or `tuple`

If `return_dict` is `True`, `CosmosPipelineOutput` is returned, otherwise a `tuple` is returned where
the first element is a list with the generated images and the second element is a list of `bool`s
indicating whether the corresponding generated image contains "not-safe-for-work" (nsfw) content.

The call function to the pipeline for generation.

Examples:
```python
>>> import torch
>>> from diffusers import Cosmos2VideoToWorldPipeline
>>> from diffusers.utils import export_to_video, load_image

>>> # Available checkpoints: nvidia/Cosmos-Predict2-2B-Video2World, nvidia/Cosmos-Predict2-14B-Video2World
>>> model_id = "nvidia/Cosmos-Predict2-2B-Video2World"
>>> pipe = Cosmos2VideoToWorldPipeline.from_pretrained(model_id, torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> prompt = "A close-up shot captures a vibrant yellow scrubber vigorously working on a grimy plate, its bristles moving in circular motions to lift stubborn grease and food residue. The dish, once covered in remnants of a hearty meal, gradually reveals its original glossy surface. Suds form and bubble around the scrubber, creating a satisfying visual of cleanliness in progress. The sound of scrubbing fills the air, accompanied by the gentle clinking of the dish against the sink. As the scrubber continues its task, the dish transforms, gleaming under the bright kitchen lights, symbolizing the triumph of cleanliness over mess."
>>> negative_prompt = "The video captures a series of frames showing ugly scenes, static with no motion, motion blur, over-saturation, shaky footage, low resolution, grainy texture, pixelated images, poorly lit areas, underexposed and overexposed scenes, poor color balance, washed out colors, choppy sequences, jerky movements, low frame rate, artifacting, color banding, unnatural transitions, outdated special effects, fake elements, unconvincing visuals, poorly edited content, jump cuts, visual noise, and flickering. Overall, the video is of poor quality."
>>> image = load_image(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/yellow-scrubber.png"
... )

>>> video = pipe(
...     image=image, prompt=prompt, negative_prompt=negative_prompt, generator=torch.Generator().manual_seed(1)
... ).frames[0]
>>> export_to_video(video, "output.mp4", fps=16)
```

#### encode_prompt[[diffusers.Cosmos2VideoToWorldPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] | None = None, do_classifier_free_guidance: bool = True, num_videos_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 512, device: typing.Optional[torch.device] = None, dtype: typing.Optional[torch.dtype] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cosmos/pipeline_cosmos2_video2world.py#L273)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : Whether to use classifier free guidance or not.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : Number of videos that should be generated per prompt. torch device to place the resulting embeddings on

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

device : (`torch.device`, *optional*): torch device

dtype : (`torch.dtype`, *optional*): torch dtype

Encodes the prompt into text encoder hidden states.

## CosmosPipelineOutput[[diffusers.pipelines.cosmos.pipeline_output.CosmosPipelineOutput]]

#### diffusers.pipelines.cosmos.pipeline_output.CosmosPipelineOutput[[diffusers.pipelines.cosmos.pipeline_output.CosmosPipelineOutput]]

```python
diffusers.pipelines.cosmos.pipeline_output.CosmosPipelineOutput(frames: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cosmos/pipeline_output.py#L14)

**Parameters:**

frames (`torch.Tensor`, `np.ndarray`, or list[list[PIL.Image.Image]]) : list of video outputs - It can be a nested list of length `batch_size,` with each sub-list containing denoised PIL image sequences of length `num_frames.` It can also be a NumPy array or Torch tensor of shape `(batch_size, num_frames, channels, height, width)`.

Output class for Cosmos any-to-world/video pipelines.

## CosmosImagePipelineOutput[[diffusers.pipelines.cosmos.pipeline_output.CosmosImagePipelineOutput]]

#### diffusers.pipelines.cosmos.pipeline_output.CosmosImagePipelineOutput[[diffusers.pipelines.cosmos.pipeline_output.CosmosImagePipelineOutput]]

```python
diffusers.pipelines.cosmos.pipeline_output.CosmosImagePipelineOutput(images: list[PIL.Image.Image] | numpy.ndarray)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/cosmos/pipeline_output.py#L29)

**Parameters:**

images (`list[PIL.Image.Image]` or `np.ndarray`) : list of denoised PIL images of length `batch_size` or numpy array of shape `(batch_size, height, width, num_channels)`. PIL images or numpy array present the denoised images of the diffusion pipeline.

Output class for Cosmos any-to-image pipelines.
