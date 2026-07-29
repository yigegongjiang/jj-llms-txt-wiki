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

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/controlnet_hunyuandit/pipeline_hunyuandit_controlnet.py#L165)

Pipeline for English/Chinese-to-image generation using HunyuanDiT.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

HunyuanDiT uses two text encoders: [mT5](https://huggingface.co/google/mt5-base) and [bilingual CLIP](fine-tuned by
ourselves)

__call__diffusers.HunyuanDiTControlNetPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/controlnet_hunyuandit/pipeline_hunyuandit_controlnet.py#L632[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int | None = 50"}, {"name": "guidance_scale", "val": ": float | None = 5.0"}, {"name": "control_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] = None"}, {"name": "controlnet_conditioning_scale", "val": ": float | list[float] = 1.0"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "eta", "val": ": float | None = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds_2", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds_2", "val": ": torch.Tensor | None = None"}, {"name": "prompt_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "prompt_attention_mask_2", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_attention_mask_2", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "callback_on_step_end", "val": ": typing.Union[typing.Callable[[int, int], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "guidance_rescale", "val": ": float = 0.0"}, {"name": "original_size", "val": ": tuple[int, int] | None = (1024, 1024)"}, {"name": "target_size", "val": ": tuple[int, int] | None = None"}, {"name": "crops_coords_top_left", "val": ": tuple = (0, 0)"}, {"name": "use_resolution_binning", "val": ": bool = True"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide image generation. If not defined, you need to pass `prompt_embeds`.
- **height** (`int`) --
  The height in pixels of the generated image.
- **width** (`int`) --
  The width in pixels of the generated image.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference. This parameter is modulated by `strength`.
- **guidance_scale** (`float`, *optional*, defaults to 7.5) --
  A higher guidance scale value encourages the model to generate images closely linked to the text
  `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.
- **control_image** (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, `list[np.ndarray]`, --
  `list[list[torch.Tensor]]`, `list[list[np.ndarray]]` or `list[list[PIL.Image.Image]]`):
  The ControlNet input condition to provide guidance to the `unet` for generation. If the type is
  specified as `torch.Tensor`, it is passed to ControlNet as is. `PIL.Image.Image` can also be accepted
  as an image. The dimensions of the output image defaults to `image`'s dimensions. If height and/or
  width are passed, `image` is resized accordingly. If multiple ControlNets are specified in `init`,
  images must be passed as a list such that each element of the list can be correctly batched for input
  to a single ControlNet.
- **controlnet_conditioning_scale** (`float` or `list[float]`, *optional*, defaults to 1.0) --
  The outputs of the ControlNet are multiplied by `controlnet_conditioning_scale` before they are added
  to the residual in the original `unet`. If multiple ControlNets are specified in `init`, you can set
  the corresponding scale as a list.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide what to not include in image generation. If not defined, you need to
  pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale 0[StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) or `tuple`If `return_dict` is `True`, [StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) is returned,
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

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations. We use `sdxl-vae-fp16-fix`.

text_encoder (`~transformers.BertModel`, `~transformers.CLIPTextModel` | None) : Frozen text-encoder ([clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14)). HunyuanDiT uses a fine-tuned [bilingual CLIP].

tokenizer (`~transformers.BertTokenizer`, `~transformers.CLIPTokenizer` | None) : A `BertTokenizer` or `CLIPTokenizer` to tokenize text.

transformer ([HunyuanDiT2DModel](/docs/diffusers/v0.39.0/en/api/models/hunyuan_transformer2d#diffusers.HunyuanDiT2DModel)) : The HunyuanDiT model designed by Tencent Hunyuan.

text_encoder_2 (`T5EncoderModel`) : The mT5 embedder. Specifically, it is 't5-v1_1-xxl'.

tokenizer_2 (`T5Tokenizer`) : The tokenizer for the mT5 embedder.

scheduler ([DDPMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddpm#diffusers.DDPMScheduler)) : A scheduler to be used in combination with HunyuanDiT to denoise the encoded image latents.

controlnet ([HunyuanDiT2DControlNetModel](/docs/diffusers/v0.39.0/en/api/models/controlnet_hunyuandit#diffusers.HunyuanDiT2DControlNetModel) or `list[HunyuanDiT2DControlNetModel]` or [HunyuanDiT2DControlNetModel](/docs/diffusers/v0.39.0/en/api/models/controlnet_hunyuandit#diffusers.HunyuanDiT2DControlNetModel)) : Provides additional conditioning to the `unet` during the denoising process. If you set multiple ControlNets as a list, the outputs from each ControlNet are added together to create one combined additional conditioning.

**Returns:**

`[StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) or `tuple``

If `return_dict` is `True`, [StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated images and the
second element is a list of `bool`s indicating whether the corresponding generated image contains
"not-safe-for-work" (nsfw) content.
#### encode_prompt[[diffusers.HunyuanDiTControlNetPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/controlnet_hunyuandit/pipeline_hunyuandit_controlnet.py#L276)

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

text_encoder_index (`int`, *optional*) : Index of the text encoder to use. `0` for clip and `1` for T5.
