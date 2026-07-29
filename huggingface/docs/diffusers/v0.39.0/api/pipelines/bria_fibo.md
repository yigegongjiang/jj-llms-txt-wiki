# Bria Fibo

Text-to-image models have mastered imagination - but not control. FIBO changes that.

FIBO is trained on structured JSON captions up to 1,000+ words and designed to understand and control different visual parameters such as lighting, composition, color, and camera settings, enabling precise and reproducible outputs.

With only 8 billion parameters, FIBO provides a new level of image quality, prompt adherence and proffesional control.

FIBO is trained exclusively on a structured prompt and will not work with freeform text prompts.
you can use the [FIBO-VLM-prompt-to-JSON](https://huggingface.co/briaai/FIBO-VLM-prompt-to-JSON) model or the [FIBO-gemini-prompt-to-JSON](https://huggingface.co/briaai/FIBO-gemini-prompt-to-JSON)  to convert your freeform text prompt to a structured JSON prompt.

> [!NOTE]
> Avoid using freeform text prompts directly with FIBO because it does not produce the best results.

Refer to the Bria Fibo Hugging Face [page](https://huggingface.co/briaai/FIBO) to learn more.

## Usage

_As the model is gated, before using it with diffusers you first need to go to the [Bria Fibo Hugging Face page](https://huggingface.co/briaai/FIBO), fill in the form and accept the gate. Once you are in, you need to login so that your system knows you’ve accepted the gate._

Use the command below to log in:

```bash
hf auth login
```

## BriaFiboPipeline[[diffusers.BriaFiboPipeline]]

#### diffusers.BriaFiboPipeline[[diffusers.BriaFiboPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/bria_fibo/pipeline_bria_fibo.py#L76)

__call__diffusers.BriaFiboPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/bria_fibo/pipeline_bria_fibo.py#L445[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 30"}, {"name": "timesteps", "val": ": list = None"}, {"name": "guidance_scale", "val": ": float = 5"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.FloatTensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "joint_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 3000"}, {"name": "do_patching", "val": " = False"}]- **prompt** (`str` or `list[str]`) --
  The prompt or prompts to guide the image generation.
- **height** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The height in pixels of the generated image. This is set to 1024 by default for the best results.
- **width** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The width in pixels of the generated image. This is set to 1024 by default for the best results.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **timesteps** (`list[int]`, *optional*) --
  Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument
  in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is
  passed will be used. Must be in descending order.
- **guidance_scale** (`float`, *optional*, defaults to 5.0) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2.
  of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting
  `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to
  the text `prompt`, usually at the expense of lower image quality.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. Ignored when not using guidance (i.e., ignored
  if `guidance_scale` is less than `1`).
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.FloatTensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will ge generated by sampling using the supplied random `generator`.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.stable_diffusion_xl.StableDiffusionXLPipelineOutput` instead
  of a plain tuple.
- **joint_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`List`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.
- **max_sequence_length** (`int` defaults to 3000) -- Maximum sequence length to use with the `prompt`.
- **do_patching** (`bool`, *optional*, defaults to `False`) -- Whether to use patching.0`~pipelines.flux.BriaFiboPipelineOutput` or `tuple``~pipelines.flux.BriaFiboPipelineOutput` if
`return_dict` is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the
generated images.

Function invoked when calling the pipeline for generation.

Example:
```python
import torch
from diffusers import BriaFiboPipeline
from diffusers.modular_pipelines import ModularPipeline

torch.set_grad_enabled(False)
vlm_pipe = ModularPipeline.from_pretrained("briaai/FIBO-VLM-prompt-to-JSON", trust_remote_code=True)

pipe = BriaFiboPipeline.from_pretrained(
    "briaai/FIBO",
    trust_remote_code=True,
    torch_dtype=torch.bfloat16,
)
pipe.enable_model_cpu_offload()

with torch.inference_mode():
    # 1. Create a prompt to generate an initial image
    output = vlm_pipe(prompt="a beautiful dog")
    json_prompt_generate = output.values["json_prompt"]

    # Generate the image from the structured json prompt
    results_generate = pipe(prompt=json_prompt_generate, num_inference_steps=50, guidance_scale=5)
    results_generate.images[0].save("image_generate.png")
```

**Parameters:**

transformer (`BriaFiboTransformer2DModel`) : The transformer model for 2D diffusion modeling.

scheduler (`FlowMatchEulerDiscreteScheduler` or `KarrasDiffusionSchedulers`) : Scheduler to be used with `transformer` to denoise the encoded latents.

vae (`AutoencoderKLWan`) : Variational Auto-Encoder for encoding and decoding images to and from latent representations.

text_encoder (`SmolLM3ForCausalLM`) : Text encoder for processing input prompts.

tokenizer (`AutoTokenizer`) : Tokenizer used for processing the input text prompts for the text_encoder.

**Returns:**

``~pipelines.flux.BriaFiboPipelineOutput` or `tuple``

`~pipelines.flux.BriaFiboPipelineOutput` if
`return_dict` is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the
generated images.
#### encode_prompt[[diffusers.BriaFiboPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/bria_fibo/pipeline_bria_fibo.py#L201)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

guidance_scale (`float`) : Guidance scale for classifier free guidance.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).
