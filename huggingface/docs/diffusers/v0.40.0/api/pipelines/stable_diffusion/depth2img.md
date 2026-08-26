# Depth-to-image

  

The Stable Diffusion model can also infer depth based on an image using [MiDaS](https://github.com/isl-org/MiDaS). This allows you to pass a text prompt and an initial image to condition the generation of new images as well as a `depth_map` to preserve the image structure.

> [!TIP]
> Make sure to check out the Stable Diffusion [Tips](overview#tips) section to learn how to explore the tradeoff between scheduler speed and quality, and how to reuse pipeline components efficiently!
>
> If you're interested in using one of the official checkpoints for a task, explore the [CompVis](https://huggingface.co/CompVis) and [Stability AI](https://huggingface.co/stabilityai) Hub organizations!

## StableDiffusionDepth2ImgPipeline[[diffusers.StableDiffusionDepth2ImgPipeline]]

#### diffusers.StableDiffusionDepth2ImgPipeline[[diffusers.StableDiffusionDepth2ImgPipeline]]

```python
diffusers.StableDiffusionDepth2ImgPipeline(vae: AutoencoderKL, text_encoder: CLIPTextModel, tokenizer: CLIPTokenizer, unet: UNet2DConditionModel, scheduler: KarrasDiffusionSchedulers, depth_estimator: DPTForDepthEstimation, feature_extractor: DPTImageProcessorPil)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_diffusion/pipeline_stable_diffusion_depth2img.py#L92)

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) model to encode and decode images to and from latent representations.

text_encoder ([CLIPTextModel](https://huggingface.co/docs/transformers/v5.15.1/en/model_doc/clip#transformers.CLIPTextModel)) : Frozen text-encoder ([clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14)).

tokenizer ([CLIPTokenizer](https://huggingface.co/docs/transformers/v5.15.1/en/model_doc/clip#transformers.CLIPTokenizer)) : A `CLIPTokenizer` to tokenize text.

unet ([UNet2DConditionModel](/docs/diffusers/v0.40.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : A `UNet2DConditionModel` to denoise the encoded image latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `unet` to denoise the encoded image latents. Can be one of [DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), [LMSDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/lms_discrete#diffusers.LMSDiscreteScheduler), or [PNDMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/pndm#diffusers.PNDMScheduler).

Pipeline for text-guided depth-based image-to-image generation using Stable Diffusion.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

The pipeline also inherits the following loading methods:
- [load_textual_inversion()](/docs/diffusers/v0.40.0/en/api/loaders/textual_inversion#diffusers.loaders.TextualInversionLoaderMixin.load_textual_inversion) for loading textual inversion embeddings
- [load_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_weights) for loading LoRA weights
- [save_lora_weights()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.save_lora_weights) for saving LoRA weights

#### __call__[[diffusers.StableDiffusionDepth2ImgPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, depth_map: typing.Optional[torch.Tensor] = None, strength: float = 0.8, num_inference_steps: int | None = 50, guidance_scale: float | None = 7.5, negative_prompt: str | list[str] | None = None, num_images_per_prompt: int | None = 1, eta: float | None = 0.0, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, output_type: str | None = 'pil', return_dict: bool = True, cross_attention_kwargs: dict[str, typing.Any] | None = None, clip_skip: int | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_diffusion/pipeline_stable_diffusion_depth2img.py#L634)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide image generation. If not defined, you need to pass `prompt_embeds`.

image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) : `Image` or tensor representing an image batch to be used as the starting point. Can accept image latents as `image` only if `depth_map` is not `None`.

depth_map (`torch.Tensor`, *optional*) : Depth prediction to be used as additional conditioning for the image generation process. If not defined, it automatically predicts the depth with `self.depth_estimator`.

strength (`float`, *optional*, defaults to 0.8) : Indicates extent to transform the reference `image`. Must be between 0 and 1. `image` is used as a starting point and more noise is added the higher the `strength`. The number of denoising steps depends on the amount of noise initially added. When `strength` is 1, added noise is maximum and the denoising process runs for the full number of iterations specified in `num_inference_steps`. A value of 1 essentially ignores `image`.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference. This parameter is modulated by `strength`.

guidance_scale (`float`, *optional*, defaults to 7.5) : A higher guidance scale value encourages the model to generate images closely linked to the text `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide what to not include in image generation. If not defined, you need to pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale < 1`).

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

eta (`float`, *optional*, defaults to 0.0) : Corresponds to parameter eta (η) from the [DDIM](https://huggingface.co/papers/2010.02502) paper. Only applies to the [DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), and is ignored in other schedulers.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, text embeddings are generated from the `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, `negative_prompt_embeds` are generated from the `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generated image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [StableDiffusionPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/stable_diffusion/image_variation#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) instead of a plain tuple.

cross_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined in [`self.processor`](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

clip_skip (`int`, *optional*) : Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that the output of the pre-final layer will be used for computing the prompt embeddings.

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

**Returns:** [StableDiffusionPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/stable_diffusion/image_variation#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) or `tuple`

If `return_dict` is `True`, [StableDiffusionPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/stable_diffusion/image_variation#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated images.

The call function to the pipeline for generation.

Examples:

```py
>>> import torch
>>> import requests
>>> from PIL import Image

>>> from diffusers import StableDiffusionDepth2ImgPipeline

>>> pipe = StableDiffusionDepth2ImgPipeline.from_pretrained(
...     "stabilityai/stable-diffusion-2-depth",
...     torch_dtype=torch.float16,
... )
>>> pipe.to("cuda")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> init_image = Image.open(requests.get(url, stream=True).raw)
>>> prompt = "two tigers"
>>> n_prompt = "bad, deformed, ugly, bad anotomy"
>>> image = pipe(prompt=prompt, image=init_image, negative_prompt=n_prompt, strength=0.7).images[0]
```

#### enable_attention_slicing[[diffusers.StableDiffusionDepth2ImgPipeline.enable_attention_slicing]]

```python
enable_attention_slicing(slice_size: str | int = 'auto')
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/pipeline_utils.py#L2076)

**Parameters:**

slice_size (`str` or `int`, *optional*, defaults to `"auto"`) : When `"auto"`, halves the input to the attention heads, so attention will be computed in two steps. If `"max"`, maximum amount of memory will be saved by running only one slice at a time. If a number is provided, uses as many slices as `attention_head_dim // slice_size`. In this case, `attention_head_dim` must be a multiple of `slice_size`.

Enable sliced attention computation. When this option is enabled, the attention module splits the input tensor
in slices to compute attention in several steps. For more than one attention head, the computation is performed
sequentially over each head. This is useful to save some memory in exchange for a small speed decrease.

> [!WARNING] > ⚠️ Don't enable attention slicing if you're already using `scaled_dot_product_attention` (SDPA)
from PyTorch > 2.0 or xFormers. These attention computations are already very memory efficient so you won't
need to enable > this function. If you enable attention slicing with SDPA or xFormers, it can lead to serious
slow downs!

Examples:

```py
>>> import torch
>>> from diffusers import StableDiffusionPipeline

>>> pipe = StableDiffusionPipeline.from_pretrained(
...     "stable-diffusion-v1-5/stable-diffusion-v1-5",
...     dtype=torch.float16,
...     use_safetensors=True,
... )

>>> prompt = "a photo of an astronaut riding a horse on mars"
>>> pipe.enable_attention_slicing()
>>> image = pipe(prompt).images[0]
```

#### disable_attention_slicing[[diffusers.StableDiffusionDepth2ImgPipeline.disable_attention_slicing]]

```python
disable_attention_slicing()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/pipeline_utils.py#L2113)

Disable sliced attention computation. If `enable_attention_slicing` was previously called, attention is
computed in one step.

#### enable_xformers_memory_efficient_attention[[diffusers.StableDiffusionDepth2ImgPipeline.enable_xformers_memory_efficient_attention]]

```python
enable_xformers_memory_efficient_attention(attention_op: typing.Optional[typing.Callable] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/pipeline_utils.py#L2021)

**Parameters:**

attention_op (`Callable`, *optional*) : Override the default `None` operator for use as `op` argument to the [`memory_efficient_attention()`](https://facebookresearch.github.io/xformers/components/ops.html#xformers.ops.memory_efficient_attention) function of xFormers.

Enable memory efficient attention from [xFormers](https://facebookresearch.github.io/xformers/). When this
option is enabled, you should observe lower GPU memory usage and a potential speed up during inference. Speed
up during training is not guaranteed.

> [!WARNING] > ⚠️ When memory efficient attention and sliced attention are both enabled, memory efficient
attention takes > precedent.

Examples:

```py
>>> import torch
>>> from diffusers import DiffusionPipeline
>>> from xformers.ops import MemoryEfficientAttentionFlashAttentionOp

>>> pipe = DiffusionPipeline.from_pretrained("stabilityai/stable-diffusion-2-1", dtype=torch.float16)
>>> pipe = pipe.to("cuda")
>>> pipe.enable_xformers_memory_efficient_attention(attention_op=MemoryEfficientAttentionFlashAttentionOp)
>>> # Workaround for not accepting attention shape using VAE for Flash Attention
>>> pipe.vae.enable_xformers_memory_efficient_attention(attention_op=None)
```

#### disable_xformers_memory_efficient_attention[[diffusers.StableDiffusionDepth2ImgPipeline.disable_xformers_memory_efficient_attention]]

```python
disable_xformers_memory_efficient_attention()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/pipeline_utils.py#L2052)

Disable memory efficient attention from [xFormers](https://facebookresearch.github.io/xformers/).

#### load_textual_inversion[[diffusers.StableDiffusionDepth2ImgPipeline.load_textual_inversion]]

```python
load_textual_inversion(pretrained_model_name_or_path: str | list[str] | dict[str, torch.Tensor] | list[dict[str, torch.Tensor]], token: str | list[str] | None = None, tokenizer: 'PreTrainedTokenizer' | None = None, text_encoder: 'PreTrainedModel' | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/textual_inversion.py#L271)

**Parameters:**

pretrained_model_name_or_path (`str` or `os.PathLike` or `list[str or os.PathLike]` or `Dict` or `list[Dict]`) : Can be either one of the following or a list of them:  - A string, the *model id* (for example `sd-concepts-library/low-poly-hd-logos-icons`) of a pretrained model hosted on the Hub. - A path to a *directory* (for example `./my_text_inversion_directory/`) containing the textual inversion weights. - A path to a *file* (for example `./my_text_inversions.pt`) containing textual inversion weights. - A [torch state dict](https://pytorch.org/tutorials/beginner/saving_loading_models.html#what-is-a-state-dict). 

token (`str` or `list[str]`, *optional*) : Override the token to use for the textual inversion weights. If `pretrained_model_name_or_path` is a list, then `token` must also be a list of equal length.

text_encoder ([CLIPTextModel](https://huggingface.co/docs/transformers/v5.15.1/en/model_doc/clip#transformers.CLIPTextModel), *optional*) : Frozen text-encoder ([clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14)). If not specified, function will take self.tokenizer.

tokenizer ([CLIPTokenizer](https://huggingface.co/docs/transformers/v5.15.1/en/model_doc/clip#transformers.CLIPTokenizer), *optional*) : A `CLIPTokenizer` to tokenize text. If not specified, function will take self.tokenizer.

weight_name (`str`, *optional*) : Name of a custom weight file. This should be used when:  - The saved textual inversion file is in 🤗 Diffusers format, but was saved under a specific weight name such as `text_inv.bin`. - The saved textual inversion file is in the Automatic1111 format.

cache_dir (`str | os.PathLike`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist. 

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

local_files_only (`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to `True`, the model won't be downloaded from the Hub.

hf_token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

subfolder (`str`, *optional*, defaults to `""`) : The subfolder location of a model file within a larger model repository on the Hub or locally.

mirror (`str`, *optional*) : Mirror source to resolve accessibility issues if you're downloading a model in China. We do not guarantee the timeliness or safety of the source, and you should refer to the mirror site for more information.

Load Textual Inversion embeddings into the text encoder of [StableDiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/stable_diffusion/text2img#diffusers.StableDiffusionPipeline) (both 🤗 Diffusers and
Automatic1111 formats are supported).

Example:

To load a Textual Inversion embedding vector in 🤗 Diffusers format:

```py
from diffusers import StableDiffusionPipeline
import torch

model_id = "stable-diffusion-v1-5/stable-diffusion-v1-5"
pipe = StableDiffusionPipeline.from_pretrained(model_id, torch_dtype=torch.float16).to("cuda")

pipe.load_textual_inversion("sd-concepts-library/cat-toy")

prompt = "A <cat-toy> backpack"

image = pipe(prompt, num_inference_steps=50).images[0]
image.save("cat-backpack.png")
```

To load a Textual Inversion embedding vector in Automatic1111 format, make sure to download the vector first
(for example from [civitAI](https://civitai.com/models/3036?modelVersionId=9857)) and then load the vector

locally:

```py
from diffusers import StableDiffusionPipeline
import torch

model_id = "stable-diffusion-v1-5/stable-diffusion-v1-5"
pipe = StableDiffusionPipeline.from_pretrained(model_id, torch_dtype=torch.float16).to("cuda")

pipe.load_textual_inversion("./charturnerv2.pt", token="charturnerv2")

prompt = "charturnerv2, multiple views of the same character in the same outfit, a character turnaround of a woman wearing a black jacket and red shirt, best quality, intricate details."

image = pipe(prompt, num_inference_steps=50).images[0]
image.save("character.png")
```

#### load_lora_weights[[diffusers.StableDiffusionDepth2ImgPipeline.load_lora_weights]]

```python
load_lora_weights(pretrained_model_name_or_path_or_dict: str | dict[str, torch.Tensor], adapter_name: str | None = None, hotswap: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L143)

**Parameters:**

pretrained_model_name_or_path_or_dict (`str` or `os.PathLike` or `dict`) : See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict).

adapter_name (`str`, *optional*) : Adapter name to be used for referencing the loaded adapter model. If not specified, it will use `default_{i}` where i is the total number of adapters being loaded.

low_cpu_mem_usage (`bool`, *optional*) : Speed up model loading by only loading the pretrained LoRA weights and not initializing the random weights.

hotswap (`bool`, *optional*) : Defaults to `False`. Whether to substitute an existing (LoRA) adapter with the newly loaded adapter in-place. This means that, instead of loading an additional adapter, this will take the existing adapter weights and replace them with the weights of the new adapter. This can be faster and more memory efficient. However, the main advantage of hotswapping is that when the model is compiled with torch.compile, loading the new adapter does not require recompilation of the model. When using hotswapping, the passed `adapter_name` should be the name of an already loaded adapter.  If the new adapter and the old adapter have different ranks and/or LoRA alphas (i.e. scaling), you need to call an additional method before loading the adapter:  ```py pipeline = ...  # load diffusers pipeline max_rank = ...  # the highest rank among all LoRAs that you want to load # call *before* compiling and loading the LoRA adapter pipeline.enable_lora_hotswap(target_rank=max_rank) pipeline.load_lora_weights(file_name) # optionally compile the model now ```  Note that hotswapping adapters of the text encoder is not yet supported. There are some further limitations to this technique, which are documented here: https://huggingface.co/docs/peft/main/en/package_reference/hotswap

kwargs (`dict`, *optional*) : See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict).

Load LoRA weights specified in `pretrained_model_name_or_path_or_dict` into `self.unet` and
`self.text_encoder`.

All kwargs are forwarded to `self.lora_state_dict`.

See [lora_state_dict()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.lora_state_dict) for more details on how the state dict is
loaded.

See [load_lora_into_unet()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_unet) for more details on how the state dict is
loaded into `self.unet`.

See [load_lora_into_text_encoder()](/docs/diffusers/v0.40.0/en/api/loaders/lora#diffusers.loaders.StableDiffusionLoraLoaderMixin.load_lora_into_text_encoder) for more details on how the state
dict is loaded into `self.text_encoder`.

#### save_lora_weights[[diffusers.StableDiffusionDepth2ImgPipeline.save_lora_weights]]

```python
save_lora_weights(save_directory: str | os.PathLike, unet_lora_layers: dict = None, text_encoder_lora_layers: dict = None, is_main_process: bool = True, weight_name: str = None, save_function: typing.Callable = None, safe_serialization: bool = True, unet_lora_adapter_metadata = None, text_encoder_lora_adapter_metadata = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/lora_pipeline.py#L461)

**Parameters:**

save_directory (`str` or `os.PathLike`) : Directory to save LoRA parameters to. Will be created if it doesn't exist.

unet_lora_layers (`dict[str, torch.nn.Module]` or `dict[str, torch.Tensor]`) : State dict of the LoRA layers corresponding to the `unet`.

text_encoder_lora_layers (`dict[str, torch.nn.Module]` or `dict[str, torch.Tensor]`) : State dict of the LoRA layers corresponding to the `text_encoder`. Must explicitly pass the text encoder LoRA state dict because it comes from 🤗 Transformers.

is_main_process (`bool`, *optional*, defaults to `True`) : Whether the process calling this is the main process or not. Useful during distributed training and you need to call this function on all processes. In this case, set `is_main_process=True` only on the main process to avoid race conditions.

save_function (`Callable`) : The function to use to save the state dictionary. Useful during distributed training when you need to replace `torch.save` with another method. Can be configured with the environment variable `DIFFUSERS_SAVE_MODE`.

safe_serialization (`bool`, *optional*, defaults to `True`) : Whether to save the model using `safetensors` or the traditional PyTorch way with `pickle`.

unet_lora_adapter_metadata : LoRA adapter metadata associated with the unet to be serialized with the state dict.

text_encoder_lora_adapter_metadata : LoRA adapter metadata associated with the text encoder to be serialized with the state dict.

Save the LoRA parameters corresponding to the UNet and text encoder.

#### encode_prompt[[diffusers.StableDiffusionDepth2ImgPipeline.encode_prompt]]

```python
encode_prompt(prompt, device, num_images_per_prompt, do_classifier_free_guidance, negative_prompt = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, lora_scale: float | None = None, clip_skip: int | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_diffusion/pipeline_stable_diffusion_depth2img.py#L204)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

do_classifier_free_guidance (`bool`) : whether to use classifier free guidance or not

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

lora_scale (`float`, *optional*) : A LoRA scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

clip_skip (`int`, *optional*) : Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that the output of the pre-final layer will be used for computing the prompt embeddings.

Encodes the prompt into text encoder hidden states.

## StableDiffusionPipelineOutput[[diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput]]

#### diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput[[diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput]]

```python
diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput(images: list[PIL.Image.Image] | numpy.ndarray, nsfw_content_detected: list[bool] | None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_diffusion/pipeline_output.py#L10)

**Parameters:**

images (`list[PIL.Image.Image]` or `np.ndarray`) : list of denoised PIL images of length `batch_size` or NumPy array of shape `(batch_size, height, width, num_channels)`.

nsfw_content_detected (`list[bool]`) : list indicating whether the corresponding generated image contains "not-safe-for-work" (nsfw) content or `None` if safety checking could not be performed.

Output class for Stable Diffusion pipelines.
