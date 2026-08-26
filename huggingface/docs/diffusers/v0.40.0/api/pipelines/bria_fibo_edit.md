# Bria Fibo Edit

Fibo Edit is an 8B parameter image-to-image model that introduces a new paradigm of structured control, operating on JSON inputs paired with source images to enable deterministic and repeatable editing workflows.
Featuring native masking for granular precision, it moves beyond simple prompt-based diffusion to offer explicit, interpretable control optimized for production environments.
Its lightweight architecture is designed for deep customization, empowering researchers to build specialized "Edit" models for domain-specific tasks while delivering top-tier aesthetic quality

## Usage
_As the model is gated, before using it with diffusers you first need to go to the [Bria Fibo Hugging Face page](https://huggingface.co/briaai/Fibo-Edit), fill in the form and accept the gate. Once you are in, you need to login so that your system knows you’ve accepted the gate._

Use the command below to log in:

```bash
hf auth login
```

## BriaFiboEditPipeline[[diffusers.BriaFiboEditPipeline]]

#### diffusers.BriaFiboEditPipeline[[diffusers.BriaFiboEditPipeline]]

```python
diffusers.BriaFiboEditPipeline(transformer: BriaFiboTransformer2DModel, scheduler: typing.Union[diffusers.schedulers.scheduling_flow_match_euler_discrete.FlowMatchEulerDiscreteScheduler, diffusers.schedulers.scheduling_utils.KarrasDiffusionSchedulers], vae: AutoencoderKLWan, text_encoder: SmolLM3ForCausalLM, tokenizer: AutoTokenizer)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/bria_fibo/pipeline_bria_fibo_edit.py#L234)

**Parameters:**

transformer (`BriaFiboTransformer2DModel`) : The transformer model for 2D diffusion modeling.

scheduler (`FlowMatchEulerDiscreteScheduler` or `KarrasDiffusionSchedulers`) : Scheduler to be used with `transformer` to denoise the encoded latents.

vae (`AutoencoderKLWan`) : Variational Auto-Encoder for encoding and decoding images to and from latent representations.

text_encoder (`SmolLM3ForCausalLM`) : Text encoder for processing input prompts.

tokenizer (`AutoTokenizer`) : Tokenizer used for processing the input text prompts for the text_encoder.

#### __call__[[diffusers.BriaFiboEditPipeline.__call__]]

```python
__call__(prompt: typing.Union[str, typing.List[str]] = None, image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, mask: typing.Union[torch.FloatTensor, PIL.Image.Image, typing.List[PIL.Image.Image], typing.List[torch.FloatTensor], numpy.ndarray, typing.List[numpy.ndarray], NoneType] = None, height: int | None = None, width: int | None = None, num_inference_steps: int = 30, timesteps: typing.List[int] = None, seed: int | None = None, guidance_scale: float = 5, negative_prompt: typing.Union[str, typing.List[str], NoneType] = None, num_images_per_prompt: typing.Optional[int] = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.FloatTensor] = None, output_type: str = 'pil', return_dict: bool = True, joint_attention_kwargs: typing.Optional[typing.Dict[str, typing.Any]] = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int, typing.Dict], NoneType]] = None, callback_on_step_end_tensor_inputs: typing.List[str] = ['latents'], max_sequence_length: int = 3000, do_patching = False, _auto_resize: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/bria_fibo/pipeline_bria_fibo_edit.py#L603)

**Parameters:**

prompt (`str` or `List[str]`) : The prompt or prompts to guide the image generation.

image (`PIL.Image.Image` or `torch.FloatTensor`, *optional*) : The image to guide the image generation. If not defined, the pipeline will generate an image from scratch.

mask (`PipelineMaskInput`, *optional*) : Optional mask defining the region of `image` to be edited. Pixels covered by the mask are regenerated while the rest of the image is preserved.

height (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The height in pixels of the generated image. This is set to 1024 by default for the best results.

width (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The width in pixels of the generated image. This is set to 1024 by default for the best results.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

seed (`int`, *optional*) : A seed used to make generation deterministic.

timesteps (`List[int]`, *optional*) : Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used. Must be in descending order.

guidance_scale (`float`, *optional*, defaults to 5.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

negative_prompt (`str` or `List[str]`, *optional*) : The prompt or prompts not to guide the image generation. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `List[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.FloatTensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will ge generated by sampling using the supplied random `generator`.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.stable_diffusion_xl.StableDiffusionXLPipelineOutput` instead of a plain tuple.

joint_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`List`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int` defaults to 3000) : Maximum sequence length to use with the `prompt`.

do_patching (`bool`, *optional*, defaults to `False`) : Whether to use patching.

_auto_resize (`bool`, *optional*, defaults to `True`) : Whether to automatically resize the input image to the preferred resolutions.

**Returns:** `~pipelines.flux.BriaFiboPipelineOutput` or `tuple`

`~pipelines.flux.BriaFiboPipelineOutput` if
`return_dict` is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the
generated images.

Function invoked when calling the pipeline for generation.

Example:
```python
import torch
from diffusers import BriaFiboEditPipeline
from diffusers.modular_pipelines import ModularPipeline

torch.set_grad_enabled(False)
vlm_pipe = ModularPipelineBlocks.from_pretrained("briaai/FIBO-VLM-prompt-to-JSON", trust_remote_code=True)
vlm_pipe = vlm_pipe.init_pipeline()

pipe = BriaFiboEditPipeline.from_pretrained(
    "briaai/fibo-edit",
    torch_dtype=torch.bfloat16,
)
pipe.to("cuda")

output = vlm_pipe(
    prompt="A hyper-detailed, ultra-fluffy owl sitting in the trees at night, looking directly at the camera with wide, adorable, expressive eyes. Its feathers are soft and voluminous, catching the cool moonlight with subtle silver highlights. The owl's gaze is curious and full of charm, giving it a whimsical, storybook-like personality."
)
json_prompt_generate = json.loads(output.values["json_prompt"])

image = Image.open("image_generate.png")

edit_prompt = "Make the owl to be a cat"

json_prompt_generate["edit_instruction"] = edit_prompt

results_generate = pipe(
    prompt=json_prompt_generate, num_inference_steps=50, guidance_scale=3.5, image=image, output_type="np"
)
```

#### encode_prompt[[diffusers.BriaFiboEditPipeline.encode_prompt]]

```python
encode_prompt(prompt: typing.Union[str, typing.List[str]], device: typing.Optional[torch.device] = None, num_images_per_prompt: int = 1, guidance_scale: float = 5, negative_prompt: typing.Union[str, typing.List[str], NoneType] = None, max_sequence_length: int = 3000, lora_scale: bool | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/bria_fibo/pipeline_bria_fibo_edit.py#L359)

**Parameters:**

prompt (`str` or `List[str]`, *optional*) : prompt to be encoded

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

guidance_scale (`float`) : Guidance scale for classifier free guidance.

negative_prompt (`str` or `List[str]`, *optional*) : The prompt or prompts not to guide the image generation. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).
