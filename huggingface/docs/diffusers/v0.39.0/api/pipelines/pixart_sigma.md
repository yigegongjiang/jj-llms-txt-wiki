# PixArt-Σ

![](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/pixart/header_collage_sigma.jpg)

[PixArt-Σ: Weak-to-Strong Training of Diffusion Transformer for 4K Text-to-Image Generation](https://huggingface.co/papers/2403.04692) is Junsong Chen, Jincheng Yu, Chongjian Ge, Lewei Yao, Enze Xie, Yue Wu, Zhongdao Wang, James Kwok, Ping Luo, Huchuan Lu, and Zhenguo Li.

The abstract from the paper is:

*In this paper, we introduce PixArt-Σ, a Diffusion Transformer model (DiT) capable of directly generating images at 4K resolution. PixArt-Σ represents a significant advancement over its predecessor, PixArt-α, offering images of markedly higher fidelity and improved alignment with text prompts. A key feature of PixArt-Σ is its training efficiency. Leveraging the foundational pre-training of PixArt-α, it evolves from the ‘weaker’ baseline to a ‘stronger’ model via incorporating higher quality data, a process we term “weak-to-strong training”. The advancements in PixArt-Σ are twofold: (1) High-Quality Training Data: PixArt-Σ incorporates superior-quality image data, paired with more precise and detailed image captions. (2) Efficient Token Compression: we propose a novel attention module within the DiT framework that compresses both keys and values, significantly improving efficiency and facilitating ultra-high-resolution image generation. Thanks to these improvements, PixArt-Σ achieves superior image quality and user prompt adherence capabilities with significantly smaller model size (0.6B parameters) than existing text-to-image diffusion models, such as SDXL (2.6B parameters) and SD Cascade (5.1B parameters). Moreover, PixArt-Σ’s capability to generate 4K images supports the creation of high-resolution posters and wallpapers, efficiently bolstering the production of highquality visual content in industries such as film and gaming.*

You can find the original codebase at [PixArt-alpha/PixArt-sigma](https://github.com/PixArt-alpha/PixArt-sigma) and all the available checkpoints at [PixArt-alpha](https://huggingface.co/PixArt-alpha).

Some notes about this pipeline:

* It uses a Transformer backbone (instead of a UNet) for denoising. As such it has a similar architecture as [DiT](https://hf.co/docs/transformers/model_doc/dit).
* It was trained using text conditions computed from T5. This aspect makes the pipeline better at following complex text prompts with intricate details.
* It is good at producing high-resolution images at different aspect ratios. To get the best results, the authors recommend some size brackets which can be found [here](https://github.com/PixArt-alpha/PixArt-sigma/blob/master/diffusion/data/datasets/utils.py).
* It rivals the quality of state-of-the-art text-to-image generation systems (as of this writing) such as PixArt-α, Stable Diffusion XL, Playground V2.0 and DALL-E 3, while being more efficient than them.
* It shows the ability of generating super high resolution images, such as 2048px or even 4K.
* It shows that text-to-image models can grow from a weak model to a stronger one through several improvements (VAEs, datasets, and so on.)

> [!TIP]
> Make sure to check out the Schedulers [guide](../../using-diffusers/schedulers) to learn how to explore the tradeoff between scheduler speed and quality, and see the [reuse components across pipelines](../../using-diffusers/loading#reuse-a-pipeline) section to learn how to efficiently load the same components into multiple pipelines.

> [!TIP]
> You can further improve generation quality by passing the generated image from [PixArtSigmaPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/pixart_sigma#diffusers.PixArtSigmaPipeline) to the [SDXL refiner](./stable_diffusion/stable_diffusion_xl#base-to-refiner-model) model.

## Inference with under 8GB GPU VRAM

Run the [PixArtSigmaPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/pixart_sigma#diffusers.PixArtSigmaPipeline) with under 8GB GPU VRAM by loading the text encoder in 8-bit precision. Let's walk through a full-fledged example.

First, install the [bitsandbytes](https://github.com/TimDettmers/bitsandbytes) library:

```bash
pip install -U bitsandbytes
```

Then load the text encoder in 8-bit:

```python
from transformers import T5EncoderModel
from diffusers import PixArtSigmaPipeline
import torch

text_encoder = T5EncoderModel.from_pretrained(
    "PixArt-alpha/PixArt-Sigma-XL-2-1024-MS",
    subfolder="text_encoder",
    load_in_8bit=True,
    device_map="auto",
)
pipe = PixArtSigmaPipeline.from_pretrained(
    "PixArt-alpha/PixArt-Sigma-XL-2-1024-MS",
    text_encoder=text_encoder,
    transformer=None,
    device_map="balanced"
)
```

Now, use the `pipe` to encode a prompt:

```python
with torch.no_grad():
    prompt = "cute cat"
    prompt_embeds, prompt_attention_mask, negative_embeds, negative_prompt_attention_mask = pipe.encode_prompt(prompt)
```

Since text embeddings have been computed, remove the `text_encoder` and `pipe` from the memory, and free up some GPU VRAM:

```python
import gc

def flush():
    gc.collect()
    torch.cuda.empty_cache()

del text_encoder
del pipe
flush()
```

Then compute the latents with the prompt embeddings as inputs:

```python
pipe = PixArtSigmaPipeline.from_pretrained(
    "PixArt-alpha/PixArt-Sigma-XL-2-1024-MS",
    text_encoder=None,
    torch_dtype=torch.float16,
).to("cuda")

latents = pipe(
    negative_prompt=None,
    prompt_embeds=prompt_embeds,
    negative_prompt_embeds=negative_embeds,
    prompt_attention_mask=prompt_attention_mask,
    negative_prompt_attention_mask=negative_prompt_attention_mask,
    num_images_per_prompt=1,
    output_type="latent",
).images

del pipe.transformer
flush()
```

> [!TIP]
> Notice that while initializing `pipe`, you're setting `text_encoder` to `None` so that it's not loaded.

Once the latents are computed, pass it off to the VAE to decode into a real image:

```python
with torch.no_grad():
    image = pipe.vae.decode(latents / pipe.vae.config.scaling_factor, return_dict=False)[0]
image = pipe.image_processor.postprocess(image, output_type="pil")[0]
image.save("cat.png")
```

By deleting components you aren't using and flushing the GPU VRAM, you should be able to run [PixArtSigmaPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/pixart_sigma#diffusers.PixArtSigmaPipeline) with under 8GB GPU VRAM.

![](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/pixart/8bits_cat.png)

If you want a report of your memory-usage, run this [script](https://gist.github.com/sayakpaul/3ae0f847001d342af27018a96f467e4e).

> [!WARNING]
> Text embeddings computed in 8-bit can impact the quality of the generated images because of the information loss in the representation space caused by the reduced precision. It's recommended to compare the outputs with and without 8-bit.

While loading the `text_encoder`, you set `load_in_8bit` to `True`. You could also specify `load_in_4bit` to bring your memory requirements down even further to under 7GB.

## PixArtSigmaPipeline[[diffusers.PixArtSigmaPipeline]]

#### diffusers.PixArtSigmaPipeline[[diffusers.PixArtSigmaPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pixart_alpha/pipeline_pixart_sigma.py#L185)

Pipeline for text-to-image generation using PixArt-Sigma.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

__call__diffusers.PixArtSigmaPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pixart_alpha/pipeline_pixart_sigma.py#L631[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "negative_prompt", "val": ": str = ''"}, {"name": "num_inference_steps", "val": ": int = 20"}, {"name": "timesteps", "val": ": list = None"}, {"name": "sigmas", "val": ": list = None"}, {"name": "guidance_scale", "val": ": float = 4.5"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "prompt_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "callback", "val": ": typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None"}, {"name": "callback_steps", "val": ": int = 1"}, {"name": "clean_caption", "val": ": bool = True"}, {"name": "use_resolution_binning", "val": ": bool = True"}, {"name": "max_sequence_length", "val": ": int = 300"}, {"name": "**kwargs", "val": ""}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is
  less than `1`).
- **num_inference_steps** (`int`, *optional*, defaults to 100) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
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
  `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to
  the text `prompt`, usually at the expense of lower image quality.
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **height** (`int`, *optional*, defaults to self.unet.config.sample_size) --
  The height in pixels of the generated image.
- **width** (`int`, *optional*, defaults to self.unet.config.sample_size) --
  The width in pixels of the generated image.
- **eta** (`float`, *optional*, defaults to 0.0) --
  Corresponds to parameter eta (η) in the DDIM paper: https://huggingface.co/papers/2010.02502. Only
  applies to [schedulers.DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), will be ignored for others.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
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
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.stable_diffusion.IFPipelineOutput` instead of a plain tuple.
- **callback** (`Callable`, *optional*) --
  A function that will be called every `callback_steps` steps during inference. The function will be
  called with the following arguments: `callback(step: int, timestep: int, latents: torch.Tensor)`.
- **callback_steps** (`int`, *optional*, defaults to 1) --
  The frequency at which the `callback` function will be called. If not specified, the callback will be
  called at every step.
- **clean_caption** (`bool`, *optional*, defaults to `True`) --
  Whether or not to clean the caption before creating embeddings. Requires `beautifulsoup4` and `ftfy` to
  be installed. If the dependencies are not installed, the embeddings will be created from the raw
  prompt.
- **use_resolution_binning** (`bool` defaults to `True`) --
  If set to `True`, the requested height and width are first mapped to the closest resolutions using
  `ASPECT_RATIO_1024_BIN`. After the produced latents are decoded into images, they are resized back to
  the requested resolution. Useful for generating non-square images.
- **max_sequence_length** (`int` defaults to 300) -- Maximum sequence length to use with the `prompt`.0[ImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple`If `return_dict` is `True`, [ImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) is returned, otherwise a `tuple` is
returned where the first element is a list with the generated images

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import PixArtSigmaPipeline

>>> # You can replace the checkpoint id with "PixArt-alpha/PixArt-Sigma-XL-2-512-MS" too.
>>> pipe = PixArtSigmaPipeline.from_pretrained(
...     "PixArt-alpha/PixArt-Sigma-XL-2-1024-MS", torch_dtype=torch.float16
... )
>>> # Enable memory optimizations.
>>> # pipe.enable_model_cpu_offload()

>>> prompt = "A small cactus with a happy face in the Sahara desert."
>>> image = pipe(prompt).images[0]
```

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`T5EncoderModel`) : Frozen text-encoder. PixArt-Alpha uses [T5](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5EncoderModel), specifically the [t5-v1_1-xxl](https://huggingface.co/PixArt-alpha/PixArt-alpha/tree/main/t5-v1_1-xxl) variant.

tokenizer (`T5Tokenizer`) : Tokenizer of class [T5Tokenizer](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5Tokenizer).

transformer ([PixArtTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/pixart_transformer2d#diffusers.PixArtTransformer2DModel)) : A text conditioned `PixArtTransformer2DModel` to denoise the encoded image latents. Initially published as [`Transformer2DModel`](https://huggingface.co/PixArt-alpha/PixArt-Sigma-XL-2-1024-MS/blob/main/transformer/config.json#L2) in the config, but the mismatch can be ignored.

scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

**Returns:**

`[ImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) or `tuple``

If `return_dict` is `True`, [ImagePipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) is returned, otherwise a `tuple` is
returned where the first element is a list with the generated images
#### encode_prompt[[diffusers.PixArtSigmaPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pixart_alpha/pipeline_pixart_sigma.py#L247)

Encodes the prompt into text encoder hidden states.

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

negative_prompt (`str` or `list[str]`, *optional*) : The prompt not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`). For PixArt-Alpha, this should be "".

do_classifier_free_guidance (`bool`, *optional*, defaults to `True`) : whether to use classifier free guidance or not

num_images_per_prompt (`int`, *optional*, defaults to 1) : number of images that should be generated per prompt

device : (`torch.device`, *optional*): torch device to place the resulting embeddings on

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. For PixArt-Alpha, it's should be the embeddings of the "" string.

clean_caption (`bool`, defaults to `False`) : If `True`, the function will preprocess and clean the provided caption before encoding.

max_sequence_length (`int`, defaults to 300) : Maximum sequence length to use for the prompt.
