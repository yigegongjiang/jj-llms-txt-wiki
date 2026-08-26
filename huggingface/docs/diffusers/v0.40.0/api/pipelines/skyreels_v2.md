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

  
    
      
    
  

# SkyReels-V2: Infinite-length Film Generative model

[SkyReels-V2](https://huggingface.co/papers/2504.13074) by the SkyReels Team from Skywork AI.

*Recent advances in video generation have been driven by diffusion models and autoregressive frameworks, yet critical challenges persist in harmonizing prompt adherence, visual quality, motion dynamics, and duration: compromises in motion dynamics to enhance temporal visual quality, constrained video duration (5-10 seconds) to prioritize resolution, and inadequate shot-aware generation stemming from general-purpose MLLMs' inability to interpret cinematic grammar, such as shot composition, actor expressions, and camera motions. These intertwined limitations hinder realistic long-form synthesis and professional film-style generation. To address these limitations, we propose SkyReels-V2, an Infinite-length Film Generative Model, that synergizes Multi-modal Large Language Model (MLLM), Multi-stage Pretraining, Reinforcement Learning, and Diffusion Forcing Framework. Firstly, we design a comprehensive structural representation of video that combines the general descriptions by the Multi-modal LLM and the detailed shot language by sub-expert models. Aided with human annotation, we then train a unified Video Captioner, named SkyCaptioner-V1, to efficiently label the video data. Secondly, we establish progressive-resolution pretraining for the fundamental video generation, followed by a four-stage post-training enhancement: Initial concept-balanced Supervised Fine-Tuning (SFT) improves baseline quality; Motion-specific Reinforcement Learning (RL) training with human-annotated and synthetic distortion data addresses dynamic artifacts; Our diffusion forcing framework with non-decreasing noise schedules enables long-video synthesis in an efficient search space; Final high-quality SFT refines visual fidelity. All the code and models are available at [this https URL](https://github.com/SkyworkAI/SkyReels-V2).*

You can find all the original SkyReels-V2 checkpoints under the [Skywork](https://huggingface.co/collections/Skywork/skyreels-v2-6801b1b93df627d441d0d0d9) organization.

The following SkyReels-V2 models are supported in Diffusers:
- [SkyReels-V2 DF 1.3B - 540P](https://huggingface.co/Skywork/SkyReels-V2-DF-1.3B-540P-Diffusers)
- [SkyReels-V2 DF 14B - 540P](https://huggingface.co/Skywork/SkyReels-V2-DF-14B-540P-Diffusers)
- [SkyReels-V2 DF 14B - 720P](https://huggingface.co/Skywork/SkyReels-V2-DF-14B-720P-Diffusers)
- [SkyReels-V2 T2V 14B - 540P](https://huggingface.co/Skywork/SkyReels-V2-T2V-14B-540P-Diffusers)
- [SkyReels-V2 T2V 14B - 720P](https://huggingface.co/Skywork/SkyReels-V2-T2V-14B-720P-Diffusers)
- [SkyReels-V2 I2V 1.3B - 540P](https://huggingface.co/Skywork/SkyReels-V2-I2V-1.3B-540P-Diffusers)
- [SkyReels-V2 I2V 14B - 540P](https://huggingface.co/Skywork/SkyReels-V2-I2V-14B-540P-Diffusers)
- [SkyReels-V2 I2V 14B - 720P](https://huggingface.co/Skywork/SkyReels-V2-I2V-14B-720P-Diffusers)

This model was contributed by [M. Tolga Cangöz](https://github.com/tolgacangoz).

> [!TIP]
> Click on the SkyReels-V2 models in the right sidebar for more examples of video generation.

### A _Visual_ Demonstration

The example below has the following parameters:

- `base_num_frames=97`
- `num_frames=97`
- `num_inference_steps=30`
- `ar_step=5`
- `causal_block_size=5`

With `vae_scale_factor_temporal=4`, expect `5` blocks of `5` frames each as calculated by:

`num_latent_frames: (97-1)//vae_scale_factor_temporal+1 = 25 frames -> 5 blocks of 5 frames each`

And the maximum context length in the latent space is calculated with `base_num_latent_frames`:

`base_num_latent_frames = (97-1)//vae_scale_factor_temporal+1 = 25 -> 25//5 = 5 blocks`

Asynchronous Processing Timeline:
```text
┌─────────────────────────────────────────────────────────────────┐
│ Steps:    1    6   11   16   21   26   31   36   41   46   50   │
│ Block 1: [■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■]                       │
│ Block 2:      [■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■]                  │
│ Block 3:           [■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■]             │
│ Block 4:                [■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■]        │
│ Block 5:                     [■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■]   │
└─────────────────────────────────────────────────────────────────┘
```

For Long Videos (`num_frames` > `base_num_frames`):
`base_num_frames` acts as the "sliding window size" for processing long videos.

Example: `257`-frame video with `base_num_frames=97`, `overlap_history=17`
```text
┌──── Iteration 1 (frames 1-97) ────┐
│ Processing window: 97 frames      │ → 5 blocks,
│ Generates: frames 1-97            │   async processing
└───────────────────────────────────┘
            ┌────── Iteration 2 (frames 81-177) ──────┐
            │ Processing window: 97 frames            │
            │ Overlap: 17 frames (81-97) from prev    │ → 5 blocks,
            │ Generates: frames 98-177                │   async processing
            └─────────────────────────────────────────┘
                        ┌────── Iteration 3 (frames 161-257) ──────┐
                        │ Processing window: 97 frames             │
                        │ Overlap: 17 frames (161-177) from prev   │ → 5 blocks,
                        │ Generates: frames 178-257                │   async processing
                        └──────────────────────────────────────────┘
```

Each iteration independently runs the asynchronous processing with its own `5` blocks.
`base_num_frames` controls:
1. Memory usage (larger window = more VRAM)
2. Model context length (must match training constraints)
3. Number of blocks per iteration (`base_num_latent_frames // causal_block_size`)

Each block takes `30` steps to complete denoising.
Block N starts at step: `1 + (N-1) x ar_step`
Total steps: `30 + (5-1) x 5 = 50` steps

Synchronous mode (`ar_step=0`) would process all blocks/frames simultaneously:
```text
┌──────────────────────────────────────────────┐
│ Steps:       1            ...            30  │
│ All blocks: [■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■] │
└──────────────────────────────────────────────┘
```
Total steps: `30` steps

An example on how the step matrix is constructed for asynchronous processing:
Given the parameters: (`num_inference_steps=30, flow_shift=8, num_frames=97, ar_step=5, causal_block_size=5`)
```
- num_latent_frames = (97 frames - 1) // (4 temporal downsampling) + 1 = 25
- step_template = [999, 995, 991, 986, 980, 975, 969, 963, 956, 948,
                   941, 932, 922, 912, 901, 888, 874, 859, 841, 822,
                   799, 773, 743, 708, 666, 615, 551, 470, 363, 216]
```

The algorithm creates a `50x25` `step_matrix` where:
```
- Row 1:  [999×5, 999×5, 999×5, 999×5, 999×5]
- Row 2:  [995×5, 999×5, 999×5, 999×5, 999×5]
- Row 3:  [991×5, 999×5, 999×5, 999×5, 999×5]
- ...
- Row 7:  [969×5, 995×5, 999×5, 999×5, 999×5]
- ...
- Row 21: [799×5, 888×5, 941×5, 975×5, 999×5]
- ...
- Row 35: [  0×5, 216×5, 666×5, 822×5, 901×5]
- ...
- Row 42: [  0×5,   0×5,   0×5, 551×5, 773×5]
- ...
- Row 50: [  0×5,   0×5,   0×5,   0×5, 216×5]
```

Detailed Row `6` Analysis:
```
- step_matrix[5]:      [ 975×5,  999×5,   999×5,   999×5,   999×5]
- step_index[5]:       [   6×5,    1×5,     0×5,     0×5,     0×5]
- step_update_mask[5]: [True×5, True×5, False×5, False×5, False×5]
- valid_interval[5]:   (0, 25)
```

Key Pattern: Block `i` lags behind Block `i-1` by exactly `ar_step=5` timesteps, creating the
staggered "diffusion forcing" effect where later blocks condition on cleaner earlier blocks.

### Text-to-Video Generation

The example below demonstrates how to generate a video from text.

Refer to the [Reduce memory usage](../../optimization/memory) guide for more details about the various memory saving techniques.

From the original repo:
>You can use --ar_step 5 to enable asynchronous inference. When asynchronous inference, --causal_block_size 5 is recommended while it is not supposed to be set for synchronous generation... Asynchronous inference will take more steps to diffuse the whole sequence which means it will be SLOWER than synchronous mode. In our experiments, asynchronous inference may improve the instruction following and visual consistent performance.

```py
import torch
from diffusers import AutoModel, SkyReelsV2DiffusionForcingPipeline, UniPCMultistepScheduler
from diffusers.utils import export_to_video

model_id = "Skywork/SkyReels-V2-DF-1.3B-540P-Diffusers"
vae = AutoModel.from_pretrained(model_id, subfolder="vae", dtype=torch.float32)

pipeline = SkyReelsV2DiffusionForcingPipeline.from_pretrained(
    model_id,
    vae=vae,
    dtype=torch.bfloat16,
)
pipeline.to("cuda")
flow_shift = 8.0  # 8.0 for T2V, 5.0 for I2V
pipeline.scheduler = UniPCMultistepScheduler.from_config(pipeline.scheduler.config, flow_shift=flow_shift)

prompt = "A cat and a dog baking a cake together in a kitchen. The cat is carefully measuring flour, while the dog is stirring the batter with a wooden spoon. The kitchen is cozy, with sunlight streaming through the window."

output = pipeline(
    prompt=prompt,
    num_inference_steps=30,
    height=544,  # 720 for 720P
    width=960,   # 1280 for 720P
    num_frames=97,
    base_num_frames=97,  # 121 for 720P
    ar_step=5,  # Controls asynchronous inference (0 for synchronous mode)
    causal_block_size=5,  # Number of frames in each block for asynchronous processing
    overlap_history=None,  # Number of frames to overlap for smooth transitions in long videos; 17 for long video generations
    addnoise_condition=20,  # Improves consistency in long video generation
).frames[0]
export_to_video(output, "video.mp4", fps=24, quality=8)
```

### First-Last-Frame-to-Video Generation

The example below demonstrates how to use the image-to-video pipeline to generate a video using a text description, a starting frame, and an ending frame.

```python
import numpy as np
import torch
import torchvision.transforms.functional as TF
from diffusers import AutoencoderKLWan, SkyReelsV2DiffusionForcingImageToVideoPipeline, UniPCMultistepScheduler
from diffusers.utils import export_to_video, load_image

model_id = "Skywork/SkyReels-V2-DF-1.3B-720P-Diffusers"
vae = AutoencoderKLWan.from_pretrained(model_id, subfolder="vae", dtype=torch.float32)
pipeline = SkyReelsV2DiffusionForcingImageToVideoPipeline.from_pretrained(
    model_id, vae=vae, dtype=torch.bfloat16
)
pipeline.to("cuda")
flow_shift = 5.0  # 8.0 for T2V, 5.0 for I2V
pipeline.scheduler = UniPCMultistepScheduler.from_config(pipeline.scheduler.config, flow_shift=flow_shift)

first_frame = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/flf2v_input_first_frame.png")
last_frame = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/flf2v_input_last_frame.png")

def aspect_ratio_resize(image, pipeline, max_area=720 * 1280):
    aspect_ratio = image.height / image.width
    mod_value = pipeline.vae_scale_factor_spatial * pipeline.transformer.config.patch_size[1]
    height = round(np.sqrt(max_area * aspect_ratio)) // mod_value * mod_value
    width = round(np.sqrt(max_area / aspect_ratio)) // mod_value * mod_value
    image = image.resize((width, height))
    return image, height, width

def center_crop_resize(image, height, width):
    # Calculate resize ratio to match first frame dimensions
    resize_ratio = max(width / image.width, height / image.height)

    # Resize the image
    width = round(image.width * resize_ratio)
    height = round(image.height * resize_ratio)
    size = [width, height]
    image = TF.center_crop(image, size)

    return image, height, width

first_frame, height, width = aspect_ratio_resize(first_frame, pipeline)
if last_frame.size != first_frame.size:
    last_frame, _, _ = center_crop_resize(last_frame, height, width)

prompt = "CG animation style, a small blue bird takes off from the ground, flapping its wings. The bird's feathers are delicate, with a unique pattern on its chest. The background shows a blue sky with white clouds under bright sunshine. The camera follows the bird upward, capturing its flight and the vastness of the sky from a close-up, low-angle perspective."

output = pipeline(
    image=first_frame, last_image=last_frame, prompt=prompt, height=height, width=width, guidance_scale=5.0
).frames[0]
export_to_video(output, "video.mp4", fps=24, quality=8)
```

### Video-to-Video Generation

`SkyReelsV2DiffusionForcingVideoToVideoPipeline` extends a given video.

```python
import numpy as np
import torch
import torchvision.transforms.functional as TF
from diffusers import AutoencoderKLWan, SkyReelsV2DiffusionForcingVideoToVideoPipeline, UniPCMultistepScheduler
from diffusers.utils import export_to_video, load_video

model_id = "Skywork/SkyReels-V2-DF-1.3B-720P-Diffusers"
vae = AutoencoderKLWan.from_pretrained(model_id, subfolder="vae", dtype=torch.float32)
pipeline = SkyReelsV2DiffusionForcingVideoToVideoPipeline.from_pretrained(
    model_id, vae=vae, dtype=torch.bfloat16
)
pipeline.to("cuda")
flow_shift = 5.0  # 8.0 for T2V, 5.0 for I2V
pipeline.scheduler = UniPCMultistepScheduler.from_config(pipeline.scheduler.config, flow_shift=flow_shift)

video = load_video("input_video.mp4")

prompt = "CG animation style, a small blue bird takes off from the ground, flapping its wings. The bird's feathers are delicate, with a unique pattern on its chest. The background shows a blue sky with white clouds under bright sunshine. The camera follows the bird upward, capturing its flight and the vastness of the sky from a close-up, low-angle perspective."

output = pipeline(
    video=video, prompt=prompt, height=720, width=1280, guidance_scale=5.0, overlap_history=17,
    num_inference_steps=30, num_frames=257, base_num_frames=121#, ar_step=5, causal_block_size=5,
).frames[0]
export_to_video(output, "video.mp4", fps=24, quality=8)
# Total frames will be the number of frames of the given video + 257
```

## Notes

- SkyReels-V2 supports LoRAs with [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.SkyReelsV2LoraLoaderMixin.load_lora_weights).

`SkyReelsV2Pipeline` and `SkyReelsV2ImageToVideoPipeline` are also available without Diffusion Forcing framework applied.

## SkyReelsV2DiffusionForcingPipeline[[diffusers.SkyReelsV2DiffusionForcingPipeline]]

#### diffusers.SkyReelsV2DiffusionForcingPipeline[[diffusers.SkyReelsV2DiffusionForcingPipeline]]

```python
diffusers.SkyReelsV2DiffusionForcingPipeline(tokenizer: AutoTokenizer, text_encoder: transformers.models.t5.modeling_t5.T5EncoderModel | transformers.models.umt5.modeling_umt5.UMT5EncoderModel, transformer: SkyReelsV2Transformer3DModel, vae: AutoencoderKLWan, scheduler: UniPCMultistepScheduler)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/skyreels_v2/pipeline_skyreels_v2_diffusion_forcing.py#L128)

**Parameters:**

tokenizer (`AutoTokenizer`) : Tokenizer from [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5Tokenizer), specifically the [google/umt5-xxl](https://huggingface.co/google/umt5-xxl) variant.

text_encoder (`UMT5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/umt5-xxl](https://huggingface.co/google/umt5-xxl) variant.

transformer ([SkyReelsV2Transformer3DModel](/docs/diffusers/v0.40.0/en/api/models/skyreels_v2_transformer_3d#diffusers.SkyReelsV2Transformer3DModel)) : Conditional Transformer to denoise the encoded image latents.

scheduler ([UniPCMultistepScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/unipc#diffusers.UniPCMultistepScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLWan](/docs/diffusers/v0.40.0/en/api/models/autoencoder_kl_wan#diffusers.AutoencoderKLWan)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

Pipeline for Text-to-Video (t2v) generation using SkyReels-V2 with diffusion forcing.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a specific device, etc.).

#### __call__[[diffusers.SkyReelsV2DiffusionForcingPipeline.__call__]]

```python
__call__(prompt: str | list[str], negative_prompt: str | list[str] = None, height: int = 544, width: int = 960, num_frames: int = 97, num_inference_steps: int = 50, guidance_scale: float = 6.0, num_videos_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, output_type: str | None = 'np', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512, overlap_history: int | None = None, addnoise_condition: float = 0, base_num_frames: int = 97, ar_step: int = 0, causal_block_size: int | None = None, fps: int = 24)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/skyreels_v2/pipeline_skyreels_v2_diffusion_forcing.py#L597)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

height (`int`, defaults to `544`) : The height of the generated video.

width (`int`, defaults to `960`) : The width of the generated video.

num_frames (`int`, defaults to `97`) : The number of frames in the generated video.

num_inference_steps (`int`, defaults to `50`) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, defaults to `6.0`) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality. (**6.0 for T2V**, **5.0 for I2V**)

num_videos_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor is generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, text embeddings are generated from the `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, text embeddings are generated from the `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"np"`) : The output format of the generated image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `SkyReelsV2PipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, `PipelineCallback`, `MultiPipelineCallbacks`, *optional*) : A function or a subclass of `PipelineCallback` or `MultiPipelineCallbacks` that is called at the end of each denoising step during the inference. with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int`, *optional*, defaults to `512`) : The maximum sequence length of the prompt.

overlap_history (`int`, *optional*, defaults to `None`) : Number of frames to overlap for smooth transitions in long videos. If `None`, the pipeline assumes short video generation mode, and no overlap is applied. 17 and 37 are recommended to set.

addnoise_condition (`float`, *optional*, defaults to `0`) : This is used to help smooth the long video generation by adding some noise to the clean condition. Too large noise can cause the inconsistency as well. 20 is a recommended value, and you may try larger ones, but it is recommended to not exceed 50.

base_num_frames (`int`, *optional*, defaults to `97`) : 97 or 121 | Base frame count (**97 for 540P**, **121 for 720P**)

ar_step (`int`, *optional*, defaults to `0`) : Controls asynchronous inference (0 for synchronous mode) You can set `ar_step=5` to enable asynchronous inference. When asynchronous inference, `causal_block_size=5` is recommended while it is not supposed to be set for synchronous generation. Asynchronous inference will take more steps to diffuse the whole sequence which means it will be SLOWER than synchronous mode. In our experiments, asynchronous inference may improve the instruction following and visual consistent performance.

causal_block_size (`int`, *optional*, defaults to `None`) : The number of frames in each block/chunk. Recommended when using asynchronous inference (when ar_step > 0)

fps (`int`, *optional*, defaults to `24`) : Frame rate of the generated video

**Returns:** `~SkyReelsV2PipelineOutput` or `tuple`

If `return_dict` is `True`, `SkyReelsV2PipelineOutput` is returned, otherwise a `tuple` is returned
where the first element is a list with the generated images and the second element is a list of `bool`s
indicating whether the corresponding generated image contains "not-safe-for-work" (nsfw) content.

The call function to the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import (
...     SkyReelsV2DiffusionForcingPipeline,
...     UniPCMultistepScheduler,
...     AutoencoderKLWan,
... )
>>> from diffusers.utils import export_to_video

>>> # Load the pipeline
>>> # Available models:
>>> # - Skywork/SkyReels-V2-DF-1.3B-540P-Diffusers
>>> # - Skywork/SkyReels-V2-DF-14B-540P-Diffusers
>>> # - Skywork/SkyReels-V2-DF-14B-720P-Diffusers
>>> vae = AutoencoderKLWan.from_pretrained(
...     "Skywork/SkyReels-V2-DF-14B-720P-Diffusers",
...     subfolder="vae",
...     torch_dtype=torch.float32,
... )
>>> pipe = SkyReelsV2DiffusionForcingPipeline.from_pretrained(
...     "Skywork/SkyReels-V2-DF-14B-720P-Diffusers",
...     vae=vae,
...     torch_dtype=torch.bfloat16,
... )
>>> flow_shift = 8.0  # 8.0 for T2V, 5.0 for I2V
>>> pipe.scheduler = UniPCMultistepScheduler.from_config(pipe.scheduler.config, flow_shift=flow_shift)
>>> pipe = pipe.to("cuda")

>>> prompt = "A cat and a dog baking a cake together in a kitchen. The cat is carefully measuring flour, while the dog is stirring the batter with a wooden spoon. The kitchen is cozy, with sunlight streaming through the window."

>>> output = pipe(
...     prompt=prompt,
...     num_inference_steps=30,
...     height=544,
...     width=960,
...     guidance_scale=6.0,  # 6.0 for T2V, 5.0 for I2V
...     num_frames=97,
...     ar_step=5,  # Controls asynchronous inference (0 for synchronous mode)
...     causal_block_size=5,  # Number of frames processed together in a causal block
...     overlap_history=None,  # Number of frames to overlap for smooth transitions in long videos
...     addnoise_condition=20,  # Improves consistency in long video generation
... ).frames[0]
>>> export_to_video(output, "video.mp4", fps=24, quality=8)
```

#### encode_prompt[[diffusers.SkyReelsV2DiffusionForcingPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] | None = None, do_classifier_free_guidance: bool = True, num_videos_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 226, device: typing.Optional[torch.device] = None, dtype: typing.Optional[torch.dtype] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/skyreels_v2/pipeline_skyreels_v2_diffusion_forcing.py#L218)

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

#### generate_timestep_matrix[[diffusers.SkyReelsV2DiffusionForcingPipeline.generate_timestep_matrix]]

```python
generate_timestep_matrix(num_latent_frames: int, step_template: Tensor, base_num_latent_frames: int, ar_step: int = 5, num_pre_ready: int = 0, causal_block_size: int = 1, shrink_interval_with_mask: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/skyreels_v2/pipeline_skyreels_v2_diffusion_forcing.py#L417)

**Parameters:**

num_latent_frames (int) : Total number of latent frames to generate

step_template (torch.Tensor) : Base timestep schedule (e.g., [1000, 800, 600, ..., 0])

base_num_latent_frames (int) : Maximum frames the model can process in one forward pass

ar_step (int, optional) : Autoregressive step size for temporal lag. 0 = synchronous, >0 = asynchronous. Defaults to 5.

num_pre_ready (int, optional) : Number of frames already denoised (e.g., from prefix in a video2video task). Defaults to 0.

causal_block_size (int, optional) : Number of frames processed as a causal block. Defaults to 1.

shrink_interval_with_mask (bool, optional) : Whether to optimize processing intervals. Defaults to False.

**Returns:** `tuple containing`

- step_matrix (torch.Tensor): Matrix of timesteps for each frame at each iteration Shape:
  [num_iterations, num_latent_frames]
- step_index (torch.Tensor): Index matrix for timestep lookup Shape: [num_iterations,
  num_latent_frames]
- step_update_mask (torch.Tensor): Boolean mask indicating which frames to update Shape:
  [num_iterations, num_latent_frames]
- valid_interval (list[tuple]): list of (start, end) intervals for each iteration

**Raises:** ``ValueError``

- ``ValueError`` -- If ar_step is too small for the given configuration

This function implements the core diffusion forcing algorithm that creates a coordinated denoising schedule
across temporal frames. It supports both synchronous and asynchronous generation modes:

**Synchronous Mode** (ar_step=0, causal_block_size=1):
- All frames are denoised simultaneously at each timestep
- Each frame follows the same denoising trajectory: [1000, 800, 600, ..., 0]
- Simpler but may have less temporal consistency for long videos

**Asynchronous Mode** (ar_step>0, causal_block_size>1):
- Frames are grouped into causal blocks and processed block/chunk-wise
- Each block is denoised in a staggered pattern creating a "denoising wave"
- Earlier blocks are more denoised, later blocks lag behind by ar_step timesteps
- Creates stronger temporal dependencies and better consistency

## SkyReelsV2DiffusionForcingImageToVideoPipeline[[diffusers.SkyReelsV2DiffusionForcingImageToVideoPipeline]]

#### diffusers.SkyReelsV2DiffusionForcingImageToVideoPipeline[[diffusers.SkyReelsV2DiffusionForcingImageToVideoPipeline]]

```python
diffusers.SkyReelsV2DiffusionForcingImageToVideoPipeline(tokenizer: AutoTokenizer, text_encoder: transformers.models.t5.modeling_t5.T5EncoderModel | transformers.models.umt5.modeling_umt5.UMT5EncoderModel, transformer: SkyReelsV2Transformer3DModel, vae: AutoencoderKLWan, scheduler: UniPCMultistepScheduler)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/skyreels_v2/pipeline_skyreels_v2_diffusion_forcing_i2v.py#L133)

**Parameters:**

tokenizer (`AutoTokenizer`) : Tokenizer from [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5Tokenizer), specifically the [google/umt5-xxl](https://huggingface.co/google/umt5-xxl) variant.

text_encoder (`UMT5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/umt5-xxl](https://huggingface.co/google/umt5-xxl) variant.

transformer ([SkyReelsV2Transformer3DModel](/docs/diffusers/v0.40.0/en/api/models/skyreels_v2_transformer_3d#diffusers.SkyReelsV2Transformer3DModel)) : Conditional Transformer to denoise the encoded image latents.

scheduler ([UniPCMultistepScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/unipc#diffusers.UniPCMultistepScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLWan](/docs/diffusers/v0.40.0/en/api/models/autoencoder_kl_wan#diffusers.AutoencoderKLWan)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

Pipeline for Image-to-Video (i2v) generation using SkyReels-V2 with diffusion forcing.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a specific device, etc.).

#### __call__[[diffusers.SkyReelsV2DiffusionForcingImageToVideoPipeline.__call__]]

```python
__call__(image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]], prompt: str | list[str] = None, negative_prompt: str | list[str] = None, height: int = 544, width: int = 960, num_frames: int = 97, num_inference_steps: int = 50, guidance_scale: float = 5.0, num_videos_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, image_embeds: typing.Optional[torch.Tensor] = None, last_image: typing.Optional[torch.Tensor] = None, output_type: str | None = 'np', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512, overlap_history: int | None = None, addnoise_condition: float = 0, base_num_frames: int = 97, ar_step: int = 0, causal_block_size: int | None = None, fps: int = 24)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/skyreels_v2/pipeline_skyreels_v2_diffusion_forcing_i2v.py#L643)

**Parameters:**

image (`PipelineImageInput`) : The input image to condition the generation on. Must be an image, a list of images or a `torch.Tensor`.

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

height (`int`, defaults to `544`) : The height of the generated video.

width (`int`, defaults to `960`) : The width of the generated video.

num_frames (`int`, defaults to `97`) : The number of frames in the generated video.

num_inference_steps (`int`, defaults to `50`) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, defaults to `5.0`) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality. (**6.0 for T2V**, **5.0 for I2V**)

num_videos_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor is generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, text embeddings are generated from the `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, text embeddings are generated from the `negative_prompt` input argument.

image_embeds (`torch.Tensor`, *optional*) : Pre-generated image embeddings. Can be used to easily tweak image inputs (weighting). If not provided, image embeddings are generated from the `image` input argument.

last_image (`torch.Tensor`, *optional*) : Pre-generated image embeddings. Can be used to easily tweak image inputs (weighting). If not provided, image embeddings are generated from the `image` input argument.

output_type (`str`, *optional*, defaults to `"np"`) : The output format of the generated image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `SkyReelsV2PipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, `PipelineCallback`, `MultiPipelineCallbacks`, *optional*) : A function or a subclass of `PipelineCallback` or `MultiPipelineCallbacks` that is called at the end of each denoising step during the inference. with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int`, *optional*, defaults to `512`) : The maximum sequence length of the prompt.

overlap_history (`int`, *optional*, defaults to `None`) : Number of frames to overlap for smooth transitions in long videos. If `None`, the pipeline assumes short video generation mode, and no overlap is applied. 17 and 37 are recommended to set.

addnoise_condition (`float`, *optional*, defaults to `0`) : This is used to help smooth the long video generation by adding some noise to the clean condition. Too large noise can cause the inconsistency as well. 20 is a recommended value, and you may try larger ones, but it is recommended to not exceed 50.

base_num_frames (`int`, *optional*, defaults to `97`) : 97 or 121 | Base frame count (**97 for 540P**, **121 for 720P**)

ar_step (`int`, *optional*, defaults to `0`) : Controls asynchronous inference (0 for synchronous mode) You can set `ar_step=5` to enable asynchronous inference. When asynchronous inference, `causal_block_size=5` is recommended while it is not supposed to be set for synchronous generation. Asynchronous inference will take more steps to diffuse the whole sequence which means it will be SLOWER than synchronous mode. In our experiments, asynchronous inference may improve the instruction following and visual consistent performance.

causal_block_size (`int`, *optional*, defaults to `None`) : The number of frames in each block/chunk. Recommended when using asynchronous inference (when ar_step > 0)

fps (`int`, *optional*, defaults to `24`) : Frame rate of the generated video

**Returns:** `~SkyReelsV2PipelineOutput` or `tuple`

If `return_dict` is `True`, `SkyReelsV2PipelineOutput` is returned, otherwise a `tuple` is returned
where the first element is a list with the generated images and the second element is a list of `bool`s
indicating whether the corresponding generated image contains "not-safe-for-work" (nsfw) content.

The call function to the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import (
...     SkyReelsV2DiffusionForcingImageToVideoPipeline,
...     UniPCMultistepScheduler,
...     AutoencoderKLWan,
... )
>>> from diffusers.utils import export_to_video
>>> from PIL import Image

>>> # Load the pipeline
>>> # Available models:
>>> # - Skywork/SkyReels-V2-DF-1.3B-540P-Diffusers
>>> # - Skywork/SkyReels-V2-DF-14B-540P-Diffusers
>>> # - Skywork/SkyReels-V2-DF-14B-720P-Diffusers
>>> vae = AutoencoderKLWan.from_pretrained(
...     "Skywork/SkyReels-V2-DF-14B-720P-Diffusers",
...     subfolder="vae",
...     torch_dtype=torch.float32,
... )
>>> pipe = SkyReelsV2DiffusionForcingImageToVideoPipeline.from_pretrained(
...     "Skywork/SkyReels-V2-DF-14B-720P-Diffusers",
...     vae=vae,
...     torch_dtype=torch.bfloat16,
... )
>>> flow_shift = 5.0  # 8.0 for T2V, 5.0 for I2V
>>> pipe.scheduler = UniPCMultistepScheduler.from_config(pipe.scheduler.config, flow_shift=flow_shift)
>>> pipe = pipe.to("cuda")

>>> prompt = "A cat and a dog baking a cake together in a kitchen. The cat is carefully measuring flour, while the dog is stirring the batter with a wooden spoon. The kitchen is cozy, with sunlight streaming through the window."
>>> image = Image.open("path/to/image.png")

>>> output = pipe(
...     image=image,
...     prompt=prompt,
...     num_inference_steps=50,
...     height=544,
...     width=960,
...     guidance_scale=5.0,  # 6.0 for T2V, 5.0 for I2V
...     num_frames=97,
...     ar_step=0,  # Controls asynchronous inference (0 for synchronous mode)
...     overlap_history=None,  # Number of frames to overlap for smooth transitions in long videos
...     addnoise_condition=20,  # Improves consistency in long video generation
... ).frames[0]
>>> export_to_video(output, "video.mp4", fps=24, quality=8)
```

#### encode_prompt[[diffusers.SkyReelsV2DiffusionForcingImageToVideoPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] | None = None, do_classifier_free_guidance: bool = True, num_videos_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 226, device: typing.Optional[torch.device] = None, dtype: typing.Optional[torch.dtype] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/skyreels_v2/pipeline_skyreels_v2_diffusion_forcing_i2v.py#L223)

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

#### generate_timestep_matrix[[diffusers.SkyReelsV2DiffusionForcingImageToVideoPipeline.generate_timestep_matrix]]

```python
generate_timestep_matrix(num_latent_frames: int, step_template: Tensor, base_num_latent_frames: int, ar_step: int = 5, num_pre_ready: int = 0, causal_block_size: int = 1, shrink_interval_with_mask: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/skyreels_v2/pipeline_skyreels_v2_diffusion_forcing_i2v.py#L463)

**Parameters:**

num_latent_frames (int) : Total number of latent frames to generate

step_template (torch.Tensor) : Base timestep schedule (e.g., [1000, 800, 600, ..., 0])

base_num_latent_frames (int) : Maximum frames the model can process in one forward pass

ar_step (int, optional) : Autoregressive step size for temporal lag. 0 = synchronous, >0 = asynchronous. Defaults to 5.

num_pre_ready (int, optional) : Number of frames already denoised (e.g., from prefix in a video2video task). Defaults to 0.

causal_block_size (int, optional) : Number of frames processed as a causal block. Defaults to 1.

shrink_interval_with_mask (bool, optional) : Whether to optimize processing intervals. Defaults to False.

**Returns:** `tuple containing`

- step_matrix (torch.Tensor): Matrix of timesteps for each frame at each iteration Shape:
  [num_iterations, num_latent_frames]
- step_index (torch.Tensor): Index matrix for timestep lookup Shape: [num_iterations,
  num_latent_frames]
- step_update_mask (torch.Tensor): Boolean mask indicating which frames to update Shape:
  [num_iterations, num_latent_frames]
- valid_interval (list[tuple]): list of (start, end) intervals for each iteration

**Raises:** ``ValueError``

- ``ValueError`` -- If ar_step is too small for the given configuration

This function implements the core diffusion forcing algorithm that creates a coordinated denoising schedule
across temporal frames. It supports both synchronous and asynchronous generation modes:

**Synchronous Mode** (ar_step=0, causal_block_size=1):
- All frames are denoised simultaneously at each timestep
- Each frame follows the same denoising trajectory: [1000, 800, 600, ..., 0]
- Simpler but may have less temporal consistency for long videos

**Asynchronous Mode** (ar_step>0, causal_block_size>1):
- Frames are grouped into causal blocks and processed block/chunk-wise
- Each block is denoised in a staggered pattern creating a "denoising wave"
- Earlier blocks are more denoised, later blocks lag behind by ar_step timesteps
- Creates stronger temporal dependencies and better consistency

## SkyReelsV2DiffusionForcingVideoToVideoPipeline[[diffusers.SkyReelsV2DiffusionForcingVideoToVideoPipeline]]

#### diffusers.SkyReelsV2DiffusionForcingVideoToVideoPipeline[[diffusers.SkyReelsV2DiffusionForcingVideoToVideoPipeline]]

```python
diffusers.SkyReelsV2DiffusionForcingVideoToVideoPipeline(tokenizer: AutoTokenizer, text_encoder: transformers.models.t5.modeling_t5.T5EncoderModel | transformers.models.umt5.modeling_umt5.UMT5EncoderModel, transformer: SkyReelsV2Transformer3DModel, vae: AutoencoderKLWan, scheduler: UniPCMultistepScheduler)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/skyreels_v2/pipeline_skyreels_v2_diffusion_forcing_v2v.py#L189)

**Parameters:**

tokenizer (`AutoTokenizer`) : Tokenizer from [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5Tokenizer), specifically the [google/umt5-xxl](https://huggingface.co/google/umt5-xxl) variant.

text_encoder (`UMT5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/umt5-xxl](https://huggingface.co/google/umt5-xxl) variant.

transformer ([SkyReelsV2Transformer3DModel](/docs/diffusers/v0.40.0/en/api/models/skyreels_v2_transformer_3d#diffusers.SkyReelsV2Transformer3DModel)) : Conditional Transformer to denoise the encoded image latents.

scheduler ([UniPCMultistepScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/unipc#diffusers.UniPCMultistepScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLWan](/docs/diffusers/v0.40.0/en/api/models/autoencoder_kl_wan#diffusers.AutoencoderKLWan)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

Pipeline for Video-to-Video (v2v) generation using SkyReels-V2 with diffusion forcing.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a specific device, etc.).

#### __call__[[diffusers.SkyReelsV2DiffusionForcingVideoToVideoPipeline.__call__]]

```python
__call__(video: list, prompt: str | list[str] = None, negative_prompt: str | list[str] = None, height: int = 544, width: int = 960, num_frames: int = 120, num_inference_steps: int = 50, guidance_scale: float = 6.0, num_videos_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, output_type: str | None = 'np', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512, overlap_history: int | None = None, addnoise_condition: float = 0, base_num_frames: int = 97, ar_step: int = 0, causal_block_size: int | None = None, fps: int = 24)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/skyreels_v2/pipeline_skyreels_v2_diffusion_forcing_v2v.py#L681)

**Parameters:**

video (`list[Image.Image]`) : The video to guide the video generation.

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the video generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the video generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

height (`int`, defaults to `544`) : The height of the generated video.

width (`int`, defaults to `960`) : The width of the generated video.

num_frames (`int`, defaults to `120`) : The number of frames in the generated video.

num_inference_steps (`int`, defaults to `50`) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, defaults to `6.0`) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://arxiv.org/abs/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://arxiv.org/pdf/2205.11487.pdf). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality. (**6.0 for T2V**, **5.0 for I2V**)

num_videos_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor is generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, text embeddings are generated from the `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, text embeddings are generated from the `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"np"`) : The output format of the generated image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `SkyReelsV2PipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, `PipelineCallback`, `MultiPipelineCallbacks`, *optional*) : A function or a subclass of `PipelineCallback` or `MultiPipelineCallbacks` that is called at the end of each denoising step during the inference. with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int`, *optional*, defaults to `512`) : The maximum sequence length of the prompt.

overlap_history (`int`, *optional*, defaults to `None`) : Number of frames to overlap for smooth transitions in long videos. If `None`, the pipeline assumes short video generation mode, and no overlap is applied. 17 and 37 are recommended to set.

addnoise_condition (`float`, *optional*, defaults to `0`) : This is used to help smooth the long video generation by adding some noise to the clean condition. Too large noise can cause the inconsistency as well. 20 is a recommended value, and you may try larger ones, but it is recommended to not exceed 50.

base_num_frames (`int`, *optional*, defaults to `97`) : 97 or 121 | Base frame count (**97 for 540P**, **121 for 720P**)

ar_step (`int`, *optional*, defaults to `0`) : Controls asynchronous inference (0 for synchronous mode) You can set `ar_step=5` to enable asynchronous inference. When asynchronous inference, `causal_block_size=5` is recommended while it is not supposed to be set for synchronous generation. Asynchronous inference will take more steps to diffuse the whole sequence which means it will be SLOWER than synchronous mode. In our experiments, asynchronous inference may improve the instruction following and visual consistent performance.

causal_block_size (`int`, *optional*, defaults to `None`) : The number of frames in each block/chunk. Recommended when using asynchronous inference (when ar_step > 0)

fps (`int`, *optional*, defaults to `24`) : Frame rate of the generated video

**Returns:** `~SkyReelsV2PipelineOutput` or `tuple`

If `return_dict` is `True`, `SkyReelsV2PipelineOutput` is returned, otherwise a `tuple` is returned
where the first element is a list with the generated images and the second element is a list of `bool`s
indicating whether the corresponding generated image contains "not-safe-for-work" (nsfw) content.

The call function to the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import (
...     SkyReelsV2DiffusionForcingVideoToVideoPipeline,
...     UniPCMultistepScheduler,
...     AutoencoderKLWan,
... )
>>> from diffusers.utils import export_to_video

>>> # Load the pipeline
>>> # Available models:
>>> # - Skywork/SkyReels-V2-DF-1.3B-540P-Diffusers
>>> # - Skywork/SkyReels-V2-DF-14B-540P-Diffusers
>>> # - Skywork/SkyReels-V2-DF-14B-720P-Diffusers
>>> vae = AutoencoderKLWan.from_pretrained(
...     "Skywork/SkyReels-V2-DF-14B-720P-Diffusers",
...     subfolder="vae",
...     torch_dtype=torch.float32,
... )
>>> pipe = SkyReelsV2DiffusionForcingVideoToVideoPipeline.from_pretrained(
...     "Skywork/SkyReels-V2-DF-14B-720P-Diffusers",
...     vae=vae,
...     torch_dtype=torch.bfloat16,
... )
>>> flow_shift = 8.0  # 8.0 for T2V, 5.0 for I2V
>>> pipe.scheduler = UniPCMultistepScheduler.from_config(pipe.scheduler.config, flow_shift=flow_shift)
>>> pipe = pipe.to("cuda")

>>> prompt = "A cat and a dog baking a cake together in a kitchen. The cat is carefully measuring flour, while the dog is stirring the batter with a wooden spoon. The kitchen is cozy, with sunlight streaming through the window."

>>> output = pipe(
...     prompt=prompt,
...     num_inference_steps=50,
...     height=544,
...     width=960,
...     guidance_scale=6.0,  # 6.0 for T2V, 5.0 for I2V
...     num_frames=97,
...     ar_step=0,  # Controls asynchronous inference (0 for synchronous mode)
...     overlap_history=None,  # Number of frames to overlap for smooth transitions in long videos
...     addnoise_condition=20,  # Improves consistency in long video generation
... ).frames[0]
>>> export_to_video(output, "video.mp4", fps=24, quality=8)
```

#### encode_prompt[[diffusers.SkyReelsV2DiffusionForcingVideoToVideoPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] | None = None, do_classifier_free_guidance: bool = True, num_videos_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 226, device: typing.Optional[torch.device] = None, dtype: typing.Optional[torch.dtype] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/skyreels_v2/pipeline_skyreels_v2_diffusion_forcing_v2v.py#L279)

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

#### generate_timestep_matrix[[diffusers.SkyReelsV2DiffusionForcingVideoToVideoPipeline.generate_timestep_matrix]]

```python
generate_timestep_matrix(num_latent_frames: int, step_template: Tensor, base_num_latent_frames: int, ar_step: int = 5, num_pre_ready: int = 0, causal_block_size: int = 1, shrink_interval_with_mask: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/skyreels_v2/pipeline_skyreels_v2_diffusion_forcing_v2v.py#L501)

**Parameters:**

num_latent_frames (int) : Total number of latent frames to generate

step_template (torch.Tensor) : Base timestep schedule (e.g., [1000, 800, 600, ..., 0])

base_num_latent_frames (int) : Maximum frames the model can process in one forward pass

ar_step (int, optional) : Autoregressive step size for temporal lag. 0 = synchronous, >0 = asynchronous. Defaults to 5.

num_pre_ready (int, optional) : Number of frames already denoised (e.g., from prefix in a video2video task). Defaults to 0.

causal_block_size (int, optional) : Number of frames processed as a causal block. Defaults to 1.

shrink_interval_with_mask (bool, optional) : Whether to optimize processing intervals. Defaults to False.

**Returns:** `tuple containing`

- step_matrix (torch.Tensor): Matrix of timesteps for each frame at each iteration Shape:
  [num_iterations, num_latent_frames]
- step_index (torch.Tensor): Index matrix for timestep lookup Shape: [num_iterations,
  num_latent_frames]
- step_update_mask (torch.Tensor): Boolean mask indicating which frames to update Shape:
  [num_iterations, num_latent_frames]
- valid_interval (list[tuple]): list of (start, end) intervals for each iteration

**Raises:** ``ValueError``

- ``ValueError`` -- If ar_step is too small for the given configuration

This function implements the core diffusion forcing algorithm that creates a coordinated denoising schedule
across temporal frames. It supports both synchronous and asynchronous generation modes:

**Synchronous Mode** (ar_step=0, causal_block_size=1):
- All frames are denoised simultaneously at each timestep
- Each frame follows the same denoising trajectory: [1000, 800, 600, ..., 0]
- Simpler but may have less temporal consistency for long videos

**Asynchronous Mode** (ar_step>0, causal_block_size>1):
- Frames are grouped into causal blocks and processed block/chunk-wise
- Each block is denoised in a staggered pattern creating a "denoising wave"
- Earlier blocks are more denoised, later blocks lag behind by ar_step timesteps
- Creates stronger temporal dependencies and better consistency

## SkyReelsV2Pipeline[[diffusers.SkyReelsV2Pipeline]]

#### diffusers.SkyReelsV2Pipeline[[diffusers.SkyReelsV2Pipeline]]

```python
diffusers.SkyReelsV2Pipeline(tokenizer: AutoTokenizer, text_encoder: transformers.models.t5.modeling_t5.T5EncoderModel | transformers.models.umt5.modeling_umt5.UMT5EncoderModel, transformer: SkyReelsV2Transformer3DModel, vae: AutoencoderKLWan, scheduler: UniPCMultistepScheduler)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/skyreels_v2/pipeline_skyreels_v2.py#L107)

**Parameters:**

tokenizer (`T5Tokenizer`) : Tokenizer from [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5Tokenizer), specifically the [google/umt5-xxl](https://huggingface.co/google/umt5-xxl) variant.

text_encoder (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/umt5-xxl](https://huggingface.co/google/umt5-xxl) variant.

transformer ([SkyReelsV2Transformer3DModel](/docs/diffusers/v0.40.0/en/api/models/skyreels_v2_transformer_3d#diffusers.SkyReelsV2Transformer3DModel)) : Conditional Transformer to denoise the input latents.

scheduler ([UniPCMultistepScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/unipc#diffusers.UniPCMultistepScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLWan](/docs/diffusers/v0.40.0/en/api/models/autoencoder_kl_wan#diffusers.AutoencoderKLWan)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

Pipeline for Text-to-Video (t2v) generation using SkyReels-V2.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

#### __call__[[diffusers.SkyReelsV2Pipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, negative_prompt: str | list[str] = None, height: int = 544, width: int = 960, num_frames: int = 97, num_inference_steps: int = 50, guidance_scale: float = 6.0, num_videos_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, output_type: str | None = 'np', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/skyreels_v2/pipeline_skyreels_v2.py#L376)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale < 1`).

height (`int`, defaults to `544`) : The height in pixels of the generated image.

width (`int`, defaults to `960`) : The width in pixels of the generated image.

num_frames (`int`, defaults to `97`) : The number of frames in the generated video.

num_inference_steps (`int`, defaults to `50`) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, defaults to `6.0`) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor is generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, text embeddings are generated from the `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, `negative_prompt_embeds` are generated from the `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"np"`) : The output format of the generated image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `SkyReelsV2PipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, `PipelineCallback`, `MultiPipelineCallbacks`, *optional*) : A function or a subclass of `PipelineCallback` or `MultiPipelineCallbacks` that is called at the end of each denoising step during the inference. with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int`, *optional*, defaults to `512`) : The maximum sequence length for the text encoder.

**Returns:** `~SkyReelsV2PipelineOutput` or `tuple`

If `return_dict` is `True`, `SkyReelsV2PipelineOutput` is returned, otherwise a `tuple` is returned
where the first element is a list with the generated images and the second element is a list of `bool`s
indicating whether the corresponding generated image contains "not-safe-for-work" (nsfw) content.

The call function to the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import (
...     SkyReelsV2Pipeline,
...     UniPCMultistepScheduler,
...     AutoencoderKLWan,
... )
>>> from diffusers.utils import export_to_video

>>> # Load the pipeline
>>> # Available models:
>>> # - Skywork/SkyReels-V2-T2V-14B-540P-Diffusers
>>> # - Skywork/SkyReels-V2-T2V-14B-720P-Diffusers
>>> vae = AutoencoderKLWan.from_pretrained(
...     "Skywork/SkyReels-V2-T2V-14B-720P-Diffusers",
...     subfolder="vae",
...     torch_dtype=torch.float32,
... )
>>> pipe = SkyReelsV2Pipeline.from_pretrained(
...     "Skywork/SkyReels-V2-T2V-14B-720P-Diffusers",
...     vae=vae,
...     torch_dtype=torch.bfloat16,
... )
>>> flow_shift = 8.0  # 8.0 for T2V, 5.0 for I2V
>>> pipe.scheduler = UniPCMultistepScheduler.from_config(pipe.scheduler.config, flow_shift=flow_shift)
>>> pipe = pipe.to("cuda")

>>> prompt = "A cat and a dog baking a cake together in a kitchen. The cat is carefully measuring flour, while the dog is stirring the batter with a wooden spoon. The kitchen is cozy, with sunlight streaming through the window."

>>> output = pipe(
...     prompt=prompt,
...     num_inference_steps=50,
...     height=544,
...     width=960,
...     guidance_scale=6.0,  # 6.0 for T2V, 5.0 for I2V
...     num_frames=97,
... ).frames[0]
>>> export_to_video(output, "video.mp4", fps=24, quality=8)
```

#### encode_prompt[[diffusers.SkyReelsV2Pipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] | None = None, do_classifier_free_guidance: bool = True, num_videos_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 226, device: typing.Optional[torch.device] = None, dtype: typing.Optional[torch.dtype] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/skyreels_v2/pipeline_skyreels_v2.py#L197)

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

## SkyReelsV2ImageToVideoPipeline[[diffusers.SkyReelsV2ImageToVideoPipeline]]

#### diffusers.SkyReelsV2ImageToVideoPipeline[[diffusers.SkyReelsV2ImageToVideoPipeline]]

```python
diffusers.SkyReelsV2ImageToVideoPipeline(tokenizer: AutoTokenizer, text_encoder: transformers.models.t5.modeling_t5.T5EncoderModel | transformers.models.umt5.modeling_umt5.UMT5EncoderModel, image_encoder: CLIPVisionModelWithProjection, image_processor: CLIPProcessor, transformer: SkyReelsV2Transformer3DModel, vae: AutoencoderKLWan, scheduler: UniPCMultistepScheduler)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/skyreels_v2/pipeline_skyreels_v2_i2v.py#L127)

**Parameters:**

tokenizer (`T5Tokenizer`) : Tokenizer from [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5Tokenizer), specifically the [google/umt5-xxl](https://huggingface.co/google/umt5-xxl) variant.

text_encoder (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/umt5-xxl](https://huggingface.co/google/umt5-xxl) variant.

image_encoder (`CLIPVisionModelWithProjection`) : [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPVisionModelWithProjection), specifically the [clip-vit-huge-patch14](https://github.com/mlfoundations/open_clip/blob/main/docs/PRETRAINED.md#vit-h14-xlm-roberta-large) variant.

transformer ([SkyReelsV2Transformer3DModel](/docs/diffusers/v0.40.0/en/api/models/skyreels_v2_transformer_3d#diffusers.SkyReelsV2Transformer3DModel)) : Conditional Transformer to denoise the input latents.

scheduler ([UniPCMultistepScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/unipc#diffusers.UniPCMultistepScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKLWan](/docs/diffusers/v0.40.0/en/api/models/autoencoder_kl_wan#diffusers.AutoencoderKLWan)) : Variational Auto-Encoder (VAE) Model to encode and decode videos to and from latent representations.

Pipeline for Image-to-Video (i2v) generation using SkyReels-V2.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

#### __call__[[diffusers.SkyReelsV2ImageToVideoPipeline.__call__]]

```python
__call__(image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]], prompt: str | list[str] = None, negative_prompt: str | list[str] = None, height: int = 544, width: int = 960, num_frames: int = 97, num_inference_steps: int = 50, guidance_scale: float = 5.0, num_videos_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, image_embeds: typing.Optional[torch.Tensor] = None, last_image: typing.Optional[torch.Tensor] = None, output_type: str | None = 'np', return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/skyreels_v2/pipeline_skyreels_v2_i2v.py#L476)

**Parameters:**

image (`PipelineImageInput`) : The input image to condition the generation on. Must be an image, a list of images or a `torch.Tensor`.

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

height (`int`, defaults to `544`) : The height of the generated video.

width (`int`, defaults to `960`) : The width of the generated video.

num_frames (`int`, defaults to `97`) : The number of frames in the generated video.

num_inference_steps (`int`, defaults to `50`) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, defaults to `5.0`) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

num_videos_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor is generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, text embeddings are generated from the `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, text embeddings are generated from the `negative_prompt` input argument.

image_embeds (`torch.Tensor`, *optional*) : Pre-generated image embeddings. Can be used to easily tweak image inputs (weighting). If not provided, image embeddings are generated from the `image` input argument.

last_image (`torch.Tensor`, *optional*) : Optional last image for image-to-video conditioning that anchors the end of the generated video.

output_type (`str`, *optional*, defaults to `"np"`) : The output format of the generated image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `WanPipelineOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, `PipelineCallback`, `MultiPipelineCallbacks`, *optional*) : A function or a subclass of `PipelineCallback` or `MultiPipelineCallbacks` that is called at the end of each denoising step during the inference. with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int`, *optional*, defaults to `512`) : The maximum sequence length of the prompt.

**Returns:** `~SkyReelsV2PipelineOutput` or `tuple`

If `return_dict` is `True`, `SkyReelsV2PipelineOutput` is returned, otherwise a `tuple` is returned
where the first element is a list with the generated images and the second element is a list of `bool`s
indicating whether the corresponding generated image contains "not-safe-for-work" (nsfw) content.

The call function to the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import (
...     SkyReelsV2ImageToVideoPipeline,
...     UniPCMultistepScheduler,
...     AutoencoderKLWan,
... )
>>> from diffusers.utils import export_to_video
>>> from PIL import Image

>>> # Load the pipeline
>>> # Available models:
>>> # - Skywork/SkyReels-V2-I2V-1.3B-540P-Diffusers
>>> # - Skywork/SkyReels-V2-I2V-14B-540P-Diffusers
>>> # - Skywork/SkyReels-V2-I2V-14B-720P-Diffusers
>>> vae = AutoencoderKLWan.from_pretrained(
...     "Skywork/SkyReels-V2-I2V-14B-720P-Diffusers",
...     subfolder="vae",
...     torch_dtype=torch.float32,
... )
>>> pipe = SkyReelsV2ImageToVideoPipeline.from_pretrained(
...     "Skywork/SkyReels-V2-I2V-14B-720P-Diffusers",
...     vae=vae,
...     torch_dtype=torch.bfloat16,
... )
>>> flow_shift = 5.0  # 8.0 for T2V, 5.0 for I2V
>>> pipe.scheduler = UniPCMultistepScheduler.from_config(pipe.scheduler.config, flow_shift=flow_shift)
>>> pipe = pipe.to("cuda")

>>> prompt = "A cat and a dog baking a cake together in a kitchen. The cat is carefully measuring flour, while the dog is stirring the batter with a wooden spoon. The kitchen is cozy, with sunlight streaming through the window."
>>> image = Image.open("path/to/image.png")

>>> output = pipe(
...     image=image,
...     prompt=prompt,
...     num_inference_steps=50,
...     height=544,
...     width=960,
...     guidance_scale=5.0,  # 6.0 for T2V, 5.0 for I2V
...     num_frames=97,
... ).frames[0]
>>> export_to_video(output, "video.mp4", fps=24, quality=8)
```

#### encode_prompt[[diffusers.SkyReelsV2ImageToVideoPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], negative_prompt: str | list[str] | None = None, do_classifier_free_guidance: bool = True, num_videos_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, max_sequence_length: int = 226, device: typing.Optional[torch.device] = None, dtype: typing.Optional[torch.dtype] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/skyreels_v2/pipeline_skyreels_v2_i2v.py#L238)

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

## SkyReelsV2PipelineOutput[[diffusers.pipelines.skyreels_v2.pipeline_output.SkyReelsV2PipelineOutput]]

#### diffusers.pipelines.skyreels_v2.pipeline_output.SkyReelsV2PipelineOutput[[diffusers.pipelines.skyreels_v2.pipeline_output.SkyReelsV2PipelineOutput]]

```python
diffusers.pipelines.skyreels_v2.pipeline_output.SkyReelsV2PipelineOutput(frames: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/skyreels_v2/pipeline_output.py#L9)

**Parameters:**

frames (`torch.Tensor`, `np.ndarray`, or list[list[PIL.Image.Image]]) : list of video outputs - It can be a nested list of length `batch_size,` with each sub-list containing denoised PIL image sequences of length `num_frames.` It can also be a NumPy array or Torch tensor of shape `(batch_size, num_frames, channels, height, width)`.

Output class for SkyReelsV2 pipelines.
