# ControlNet with Hunyuan-DiT

HunyuanDiTControlNetPipeline is an implementation of ControlNet for [Hunyuan-DiT](https://huggingface.co/papers/2405.08748).

ControlNet was introduced in [Adding Conditional Control to Text-to-Image Diffusion Models](https://huggingface.co/papers/2302.05543) by Lvmin Zhang, Anyi Rao, and Maneesh Agrawala.

With a ControlNet model, you can provide an additional control image to condition and control Hunyuan-DiT generation. For example, if you provide a depth map, the ControlNet model generates an image that'll preserve the spatial information from the depth map. It is a more flexible and accurate way to control the image generation process.

The abstract from the paper is:

*We present ControlNet, a neural network architecture to add spatial conditioning controls to large, pretrained text-to-image diffusion models. ControlNet locks the production-ready large diffusion models, and reuses their deep and robust encoding layers pretrained with billions of images as a strong backbone to learn a diverse set of conditional controls. The neural architecture is connected with "zero convolutions" (zero-initialized convolution layers) that progressively grow the parameters from zero and ensure that no harmful noise could affect the finetuning. We test various conditioning controls, eg, edges, depth, segmentation, human pose, etc, with Stable Diffusion, using single or multiple conditions, with or without prompts. We show that the training of ControlNets is robust with small (<50k) and large (>1m) datasets. Extensive results show that ControlNet may facilitate wider applications to control image diffusion models.*

This code is implemented by Tencent Hunyuan Team. You can find pre-trained checkpoints for Hunyuan-DiT ControlNets on [Tencent Hunyuan](https://huggingface.co/Tencent-Hunyuan).

> [!TIP]
> Make sure to check out the Schedulers [guide](../../using-diffusers/schedulers) to learn how to explore the tradeoff between scheduler speed and quality, and see the [reuse components across pipelines](../../using-diffusers/loading#reuse-a-pipeline) section to learn how to efficiently load the same components into multiple pipelines.

## HunyuanDiTControlNetPipeline[[diffusers.HunyuanDiTControlNetPipeline]]

#### diffusers.HunyuanDiTControlNetPipeline[[diffusers.HunyuanDiTControlNetPipeline]]

```python
diffusers.HunyuanDiTControlNetPipeline(vae: AutoencoderKL, text_encoder: BertModel, tokenizer: BertTokenizer, transformer: HunyuanDiT2DModel, scheduler: DDPMScheduler, safety_checker: StableDiffusionSafetyChecker, feature_extractor: CLIPImageProcessorPil, controlnet: diffusers.models.controlnets.controlnet_hunyuan.HunyuanDiT2DControlNetModel | list[diffusers.models.controlnets.controlnet_hunyuan.HunyuanDiT2DControlNetModel] | tuple[diffusers.models.controlnets.controlnet_hunyuan.HunyuanDiT2DControlNetModel] | diffusers.models.controlnets.controlnet_hunyuan.HunyuanDiT2DMultiControlNetModel, text_encoder_2: transformers.models.t5.modeling_t5.T5EncoderModel | None = None, tokenizer_2: transformers.models.t5.tokenization_t5.T5Tokenizer | None = None, requires_safety_checker: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/controlnet_hunyuandit/pipeline_hunyuandit_controlnet.py#L165)

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations. We use `sdxl-vae-fp16-fix`.

text_encoder (`~transformers.BertModel`, `~transformers.CLIPTextModel` | None) : Frozen text-encoder ([clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14)). HunyuanDiT uses a fine-tuned [bilingual CLIP].

tokenizer (`~transformers.BertTokenizer`, `~transformers.CLIPTokenizer` | None) : A `BertTokenizer` or `CLIPTokenizer` to tokenize text.

transformer ([HunyuanDiT2DModel](/docs/diffusers/v0.40.0/en/api/models/hunyuan_transformer2d#diffusers.HunyuanDiT2DModel)) : The HunyuanDiT model designed by Tencent Hunyuan.

text_encoder_2 (`T5EncoderModel`) : The mT5 embedder. Specifically, it is 't5-v1_1-xxl'.

tokenizer_2 (`T5Tokenizer`) : The tokenizer for the mT5 embedder.

scheduler ([DDPMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddpm#diffusers.DDPMScheduler)) : A scheduler to be used in combination with HunyuanDiT to denoise the encoded image latents.

controlnet ([HunyuanDiT2DControlNetModel](/docs/diffusers/v0.40.0/en/api/models/controlnet_hunyuandit#diffusers.HunyuanDiT2DControlNetModel) or `list[HunyuanDiT2DControlNetModel]` or [HunyuanDiT2DControlNetModel](/docs/diffusers/v0.40.0/en/api/models/controlnet_hunyuandit#diffusers.HunyuanDiT2DControlNetModel)) : Provides additional conditioning to the `unet` during the denoising process. If you set multiple ControlNets as a list, the outputs from each ControlNet are added together to create one combined additional conditioning.

Pipeline for English/Chinese-to-image generation using HunyuanDiT.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

HunyuanDiT uses two text encoders: [mT5](https://huggingface.co/google/mt5-base) and [bilingual CLIP](fine-tuned by
ourselves)

#### __call__[[diffusers.HunyuanDiTControlNetPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, height: int | None = None, width: int | None = None, num_inference_steps: int | None = 50, guidance_scale: float | None = 5.0, control_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, controlnet_conditioning_scale: float | list[float] = 1.0, negative_prompt: str | list[str] | None = None, num_images_per_prompt: int | None = 1, eta: float | None = 0.0, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_embeds_2: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds_2: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, prompt_attention_mask_2: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask_2: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, callback_on_step_end: typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None, callback_on_step_end_tensor_inputs: list = ['latents'], guidance_rescale: float = 0.0, original_size: tuple[int, int] | None = (1024, 1024), target_size: tuple[int, int] | None = None, crops_coords_top_left: tuple = (0, 0), use_resolution_binning: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/controlnet_hunyuandit/pipeline_hunyuandit_controlnet.py#L632)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide image generation. If not defined, you need to pass `prompt_embeds`.

height (`int`) : The height in pixels of the generated image.

width (`int`) : The width in pixels of the generated image.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference. This parameter is modulated by `strength`.

guidance_scale (`float`, *optional*, defaults to 7.5) : A higher guidance scale value encourages the model to generate images closely linked to the text `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.

control_image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, `list[np.ndarray]`, : `list[list[torch.Tensor]]`, `list[list[np.ndarray]]` or `list[list[PIL.Image.Image]]`): The ControlNet input condition to provide guidance to the `unet` for generation. If the type is specified as `torch.Tensor`, it is passed to ControlNet as is. `PIL.Image.Image` can also be accepted as an image. The dimensions of the output image defaults to `image`'s dimensions. If height and/or width are passed, `image` is resized accordingly. If multiple ControlNets are specified in `init`, images must be passed as a list such that each element of the list can be correctly batched for input to a single ControlNet.

controlnet_conditioning_scale (`float` or `list[float]`, *optional*, defaults to 1.0) : The outputs of the ControlNet are multiplied by `controlnet_conditioning_scale` before they are added to the residual in the original `unet`. If multiple ControlNets are specified in `init`, you can set the corresponding scale as a list.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide what to not include in image generation. If not defined, you need to pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale < 1`).

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

eta (`float`, *optional*, defaults to 0.0) : Corresponds to parameter eta (η) from the [DDIM](https://huggingface.co/papers/2010.02502) paper. Only applies to the [DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), and is ignored in other schedulers.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor is generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, text embeddings are generated from the `prompt` input argument.

prompt_embeds_2 (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, text embeddings are generated from the `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, `negative_prompt_embeds` are generated from the `negative_prompt` input argument.

negative_prompt_embeds_2 (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, `negative_prompt_embeds` are generated from the `negative_prompt` input argument.

prompt_attention_mask (`torch.Tensor`, *optional*) : Attention mask for the prompt. Required when `prompt_embeds` is passed directly.

prompt_attention_mask_2 (`torch.Tensor`, *optional*) : Attention mask for the prompt. Required when `prompt_embeds_2` is passed directly.

negative_prompt_attention_mask (`torch.Tensor`, *optional*) : Attention mask for the negative prompt. Required when `negative_prompt_embeds` is passed directly.

negative_prompt_attention_mask_2 (`torch.Tensor`, *optional*) : Attention mask for the negative prompt. Required when `negative_prompt_embeds_2` is passed directly.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generated image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [StableDiffusionPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/stable_diffusion/image_variation#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) instead of a plain tuple.

callback_on_step_end (`Callable[[int, int], None]`, `PipelineCallback`, `MultiPipelineCallbacks`, *optional*) : A callback function or a list of callback functions to be called at the end of each denoising step.

callback_on_step_end_tensor_inputs (`list[str]`, *optional*) : A list of tensor inputs that should be passed to the callback function. If not defined, all tensor inputs will be passed.

guidance_rescale (`float`, *optional*, defaults to 0.0) : Rescale the noise_cfg according to `guidance_rescale`. Based on findings of [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891). See Section 3.4

original_size (`tuple[int, int]`, *optional*, defaults to `(1024, 1024)`) : The original size of the image. Used to calculate the time ids.

target_size (`tuple[int, int]`, *optional*) : The target size of the image. Used to calculate the time ids.

crops_coords_top_left (`tuple[int, int]`, *optional*, defaults to `(0, 0)`) : The top left coordinates of the crop. Used to calculate the time ids.

use_resolution_binning (`bool`, *optional*, defaults to `True`) : Whether to use resolution binning or not. If `True`, the input resolution will be mapped to the closest standard resolution. Supported resolutions are 1024x1024, 1280x1280, 1024x768, 1152x864, 1280x960, 768x1024, 864x1152, 960x1280, 1280x768, and 768x1280. It is recommended to set this to `True`.

**Returns:** [StableDiffusionPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/stable_diffusion/image_variation#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) or `tuple`

If `return_dict` is `True`, [StableDiffusionPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/stable_diffusion/image_variation#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated images and the
second element is a list of `bool`s indicating whether the corresponding generated image contains
"not-safe-for-work" (nsfw) content.

The call function to the pipeline for generation with HunyuanDiT.

Examples:
```py
from diffusers import HunyuanDiT2DControlNetModel, HunyuanDiTControlNetPipeline
import torch

controlnet = HunyuanDiT2DControlNetModel.from_pretrained(
    "Tencent-Hunyuan/HunyuanDiT-v1.1-ControlNet-Diffusers-Canny", torch_dtype=torch.float16
)

pipe = HunyuanDiTControlNetPipeline.from_pretrained(
    "Tencent-Hunyuan/HunyuanDiT-v1.1-Diffusers", controlnet=controlnet, torch_dtype=torch.float16
)
pipe.to("cuda")

from diffusers.utils import load_image

cond_image = load_image(
    "https://huggingface.co/Tencent-Hunyuan/HunyuanDiT-v1.1-ControlNet-Diffusers-Canny/resolve/main/canny.jpg?download=true"
)

## You may also use English prompt as HunyuanDiT supports both English and Chinese
prompt = "在夜晚的酒店门前，一座古老的中国风格的狮子雕像矗立着，它的眼睛闪烁着光芒，仿佛在守护着这座建筑。背景是夜晚的酒店前，构图方式是特写，平视，居中构图。这张照片呈现了真实摄影风格，蕴含了中国雕塑文化，同时展现了神秘氛围"
# prompt="At night, an ancient Chinese-style lion statue stands in front of the hotel, its eyes gleaming as if guarding the building. The background is the hotel entrance at night, with a close-up, eye-level, and centered composition. This photo presents a realistic photographic style, embodies Chinese sculpture culture, and reveals a mysterious atmosphere."
image = pipe(
    prompt,
    height=1024,
    width=1024,
    control_image=cond_image,
    num_inference_steps=50,
).images[0]
```

#### encode_prompt[[diffusers.HunyuanDiTControlNetPipeline.encode_prompt]]

```python
encode_prompt(prompt: str, device: device = None, dtype: dtype = None, num_images_per_prompt: int = 1, do_classifier_free_guidance: bool = True, negative_prompt: str | None = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, prompt_attention_mask: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, max_sequence_length: int | None = None, text_encoder_index: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/controlnet_hunyuandit/pipeline_hunyuandit_controlnet.py#L276)

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

text_encoder_index (`int`, *optional*) : Index of the text encoder to use. `0` for clip and `1` for T5.

Encodes the prompt into text encoder hidden states.
