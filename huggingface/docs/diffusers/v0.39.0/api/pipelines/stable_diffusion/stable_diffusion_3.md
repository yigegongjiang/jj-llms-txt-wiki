# Stable Diffusion 3

  
  

Stable Diffusion 3 (SD3) was proposed in [Scaling Rectified Flow Transformers for High-Resolution Image Synthesis](https://huggingface.co/papers/2403.03206) by Patrick Esser, Sumith Kulal, Andreas Blattmann, Rahim Entezari, Jonas Muller, Harry Saini, Yam Levi, Dominik Lorenz, Axel Sauer, Frederic Boesel, Dustin Podell, Tim Dockhorn, Zion English, Kyle Lacey, Alex Goodwin, Yannik Marek, and Robin Rombach.

The abstract from the paper is:

*Diffusion models create data from noise by inverting the forward paths of data towards noise and have emerged as a powerful generative modeling technique for high-dimensional, perceptual data such as images and videos. Rectified flow is a recent generative model formulation that connects data and noise in a straight line. Despite its better theoretical properties and conceptual simplicity, it is not yet decisively established as standard practice. In this work, we improve existing noise sampling techniques for training rectified flow models by biasing them towards perceptually relevant scales. Through a large-scale study, we demonstrate the superior performance of this approach compared to established diffusion formulations for high-resolution text-to-image synthesis. Additionally, we present a novel transformer-based architecture for text-to-image generation that uses separate weights for the two modalities and enables a bidirectional flow of information between image and text tokens, improving text comprehension typography, and human preference ratings. We demonstrate that this architecture follows predictable scaling trends and correlates lower validation loss to improved text-to-image synthesis as measured by various metrics and human evaluations.*

## Usage Example

_As the model is gated, before using it with diffusers you first need to go to the [Stable Diffusion 3 Medium Hugging Face page](https://huggingface.co/stabilityai/stable-diffusion-3-medium-diffusers), fill in the form and accept the gate. Once you are in, you need to login so that your system knows you’ve accepted the gate._

Use the command below to log in:

```bash
hf auth login
```

> [!TIP]
> The SD3 pipeline uses three text encoders to generate an image. Model offloading is necessary in order for it to run on most commodity hardware. Please use the `torch.float16` data type for additional memory savings.

```python
import torch
from diffusers import StableDiffusion3Pipeline

pipe = StableDiffusion3Pipeline.from_pretrained("stabilityai/stable-diffusion-3-medium-diffusers", torch_dtype=torch.float16)
pipe.to("cuda")

image = pipe(
    prompt="a photo of a cat holding a sign that says hello world",
    negative_prompt="",
    num_inference_steps=28,
    height=1024,
    width=1024,
    guidance_scale=7.0,
).images[0]

image.save("sd3_hello_world.png")
```

**Note:** Stable Diffusion 3.5 can also be run using the SD3 pipeline, and all mentioned optimizations and techniques apply to it as well. In total there are three official models in the SD3 family:
- [`stabilityai/stable-diffusion-3-medium-diffusers`](https://huggingface.co/stabilityai/stable-diffusion-3-medium-diffusers)
- [`stabilityai/stable-diffusion-3.5-large`](https://huggingface.co/stabilityai/stable-diffusion-3-5-large)
- [`stabilityai/stable-diffusion-3.5-large-turbo`](https://huggingface.co/stabilityai/stable-diffusion-3-5-large-turbo)

## Image Prompting with IP-Adapters

An IP-Adapter lets you prompt SD3 with images, in addition to the text prompt. This is especially useful when describing complex concepts that are difficult to articulate through text alone and you have reference images. To load and use an IP-Adapter, you need:

- `image_encoder`: Pre-trained vision model used to obtain image features, usually a CLIP image encoder.
- `feature_extractor`: Image processor that prepares the input image for the chosen `image_encoder`.
- `ip_adapter_id`: Checkpoint containing parameters of image cross attention layers and image projection. 

IP-Adapters are trained for a specific model architecture, so they also work in finetuned variations of the base model. You can use the `~SD3IPAdapterMixin.set_ip_adapter_scale` function to adjust how strongly the output aligns with the image prompt. The higher the value, the more closely the model follows the image prompt. A default value of 0.5 is typically a good balance, ensuring the model considers both the text and image prompts equally.

```python
import torch
from PIL import Image

from diffusers import StableDiffusion3Pipeline
from transformers import SiglipVisionModel, SiglipImageProcessor

image_encoder_id = "google/siglip-so400m-patch14-384"
ip_adapter_id = "InstantX/SD3.5-Large-IP-Adapter"

feature_extractor = SiglipImageProcessor.from_pretrained(
    image_encoder_id,
    torch_dtype=torch.float16
)
image_encoder = SiglipVisionModel.from_pretrained(
    image_encoder_id,
    torch_dtype=torch.float16
).to( "cuda")

pipe = StableDiffusion3Pipeline.from_pretrained(
    "stabilityai/stable-diffusion-3.5-large",
    torch_dtype=torch.float16,
    feature_extractor=feature_extractor,
    image_encoder=image_encoder,
).to("cuda")

pipe.load_ip_adapter(ip_adapter_id)
pipe.set_ip_adapter_scale(0.6)

ref_img = Image.open("image.jpg").convert('RGB')

image = pipe(
    width=1024,
    height=1024,
    prompt="a cat",
    negative_prompt="lowres, low quality, worst quality",
    num_inference_steps=24,
    guidance_scale=5.0,
    ip_adapter_image=ref_img
).images[0]

image.save("result.jpg")
```

    
    IP-Adapter examples with prompt "a cat"

> [!TIP]
> Check out [IP-Adapter](../../../using-diffusers/ip_adapter) to learn more about how IP-Adapters work.

## Memory Optimisations for SD3

SD3 uses three text encoders, one of which is the very large T5-XXL model. This makes it challenging to run the model on GPUs with less than 24GB of VRAM, even when using `fp16` precision. The following section outlines a few memory optimizations in Diffusers that make it easier to run SD3 on low resource hardware.

### Running Inference with Model Offloading

The most basic memory optimization available in Diffusers allows you to offload the components of the model to CPU during inference in order to save memory, while seeing a slight increase in inference latency. Model offloading will only move a model component onto the GPU when it needs to be executed, while keeping the remaining components on the CPU.

```python
import torch
from diffusers import StableDiffusion3Pipeline

pipe = StableDiffusion3Pipeline.from_pretrained("stabilityai/stable-diffusion-3-medium-diffusers", torch_dtype=torch.float16)
pipe.enable_model_cpu_offload()

image = pipe(
    prompt="a photo of a cat holding a sign that says hello world",
    negative_prompt="",
    num_inference_steps=28,
    height=1024,
    width=1024,
    guidance_scale=7.0,
).images[0]

image.save("sd3_hello_world.png")
```

### Dropping the T5 Text Encoder during Inference

Removing the memory-intensive 4.7B parameter T5-XXL text encoder during inference can significantly decrease the memory requirements for SD3 with only a slight loss in performance.

```python
import torch
from diffusers import StableDiffusion3Pipeline

pipe = StableDiffusion3Pipeline.from_pretrained(
    "stabilityai/stable-diffusion-3-medium-diffusers",
    text_encoder_3=None,
    tokenizer_3=None,
    torch_dtype=torch.float16
)
pipe.to("cuda")

image = pipe(
    prompt="a photo of a cat holding a sign that says hello world",
    negative_prompt="",
    num_inference_steps=28,
    height=1024,
    width=1024,
    guidance_scale=7.0,
).images[0]

image.save("sd3_hello_world-no-T5.png")
```

### Using a Quantized Version of the T5 Text Encoder

We can leverage the `bitsandbytes` library to load and quantize the T5-XXL text encoder to 8-bit precision. This allows you to keep using all three text encoders while only slightly impacting performance.

First install the `bitsandbytes` library.

```shell
pip install bitsandbytes
```

Then load the T5-XXL model using the `BitsAndBytesConfig`.

```python
import torch
from diffusers import StableDiffusion3Pipeline
from transformers import T5EncoderModel, BitsAndBytesConfig

quantization_config = BitsAndBytesConfig(load_in_8bit=True)

model_id = "stabilityai/stable-diffusion-3-medium-diffusers"
text_encoder = T5EncoderModel.from_pretrained(
    model_id,
    subfolder="text_encoder_3",
    quantization_config=quantization_config,
)
pipe = StableDiffusion3Pipeline.from_pretrained(
    model_id,
    text_encoder_3=text_encoder,
    device_map="balanced",
    torch_dtype=torch.float16
)

image = pipe(
    prompt="a photo of a cat holding a sign that says hello world",
    negative_prompt="",
    num_inference_steps=28,
    height=1024,
    width=1024,
    guidance_scale=7.0,
).images[0]

image.save("sd3_hello_world-8bit-T5.png")
```

You can find the end-to-end script [here](https://gist.github.com/sayakpaul/82acb5976509851f2db1a83456e504f1).

## Performance Optimizations for SD3

### Using Torch Compile to Speed Up Inference

Using compiled components in the SD3 pipeline can speed up inference by as much as 4X. The following code snippet demonstrates how to compile the Transformer and VAE components of the SD3 pipeline.

```python
import torch
from diffusers import StableDiffusion3Pipeline

torch.set_float32_matmul_precision("high")

torch._inductor.config.conv_1x1_as_mm = True
torch._inductor.config.coordinate_descent_tuning = True
torch._inductor.config.epilogue_fusion = False
torch._inductor.config.coordinate_descent_check_all_directions = True

pipe = StableDiffusion3Pipeline.from_pretrained(
    "stabilityai/stable-diffusion-3-medium-diffusers",
    torch_dtype=torch.float16
).to("cuda")
pipe.set_progress_bar_config(disable=True)

pipe.transformer.to(memory_format=torch.channels_last)
pipe.vae.to(memory_format=torch.channels_last)

pipe.transformer = torch.compile(pipe.transformer, mode="max-autotune", fullgraph=True)
pipe.vae.decode = torch.compile(pipe.vae.decode, mode="max-autotune", fullgraph=True)

# Warm Up
prompt = "a photo of a cat holding a sign that says hello world"
for _ in range(3):
    _ = pipe(prompt=prompt, generator=torch.manual_seed(1))

# Run Inference
image = pipe(prompt=prompt, generator=torch.manual_seed(1)).images[0]
image.save("sd3_hello_world.png")
```

Check out the full script [here](https://gist.github.com/sayakpaul/508d89d7aad4f454900813da5d42ca97).

## Quantization

Quantization helps reduce the memory requirements of very large models by storing model weights in a lower precision data type. However, quantization may have varying impact on video quality depending on the video model.

Refer to the [Quantization](../../../quantization/overview) overview to learn more about supported quantization backends and selecting a quantization backend that supports your use case. The example below demonstrates how to load a quantized [StableDiffusion3Pipeline](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/stable_diffusion_3#diffusers.StableDiffusion3Pipeline) for inference with bitsandbytes.

```py
import torch
from diffusers import BitsAndBytesConfig as DiffusersBitsAndBytesConfig, SD3Transformer2DModel, StableDiffusion3Pipeline
from transformers import BitsAndBytesConfig as BitsAndBytesConfig, T5EncoderModel

quant_config = BitsAndBytesConfig(load_in_8bit=True)
text_encoder_8bit = T5EncoderModel.from_pretrained(
    "stabilityai/stable-diffusion-3.5-large",
    subfolder="text_encoder_3",
    quantization_config=quant_config,
    torch_dtype=torch.float16,
)

quant_config = DiffusersBitsAndBytesConfig(load_in_8bit=True)
transformer_8bit = SD3Transformer2DModel.from_pretrained(
    "stabilityai/stable-diffusion-3.5-large",
    subfolder="transformer",
    quantization_config=quant_config,
    torch_dtype=torch.float16,
)

pipeline = StableDiffusion3Pipeline.from_pretrained(
    "stabilityai/stable-diffusion-3.5-large",
    text_encoder=text_encoder_8bit,
    transformer=transformer_8bit,
    torch_dtype=torch.float16,
    device_map="balanced",
)

prompt = "a tiny astronaut hatching from an egg on the moon"
image = pipeline(prompt, num_inference_steps=28, guidance_scale=7.0).images[0]
image.save("sd3.png")
```

## Using Long Prompts with the T5 Text Encoder

By default, the T5 Text Encoder prompt uses a maximum sequence length of `256`. This can be adjusted by setting the `max_sequence_length` to accept fewer or more tokens. Keep in mind that longer sequences require additional resources and result in longer generation times, such as during batch inference.

```python
prompt = "A whimsical and creative image depicting a hybrid creature that is a mix of a waffle and a hippopotamus, basking in a river of melted butter amidst a breakfast-themed landscape. It features the distinctive, bulky body shape of a hippo. However, instead of the usual grey skin, the creature’s body resembles a golden-brown, crispy waffle fresh off the griddle. The skin is textured with the familiar grid pattern of a waffle, each square filled with a glistening sheen of syrup. The environment combines the natural habitat of a hippo with elements of a breakfast table setting, a river of warm, melted butter, with oversized utensils or plates peeking out from the lush, pancake-like foliage in the background, a towering pepper mill standing in for a tree.  As the sun rises in this fantastical world, it casts a warm, buttery glow over the scene. The creature, content in its butter river, lets out a yawn. Nearby, a flock of birds take flight"

image = pipe(
    prompt=prompt,
    negative_prompt="",
    num_inference_steps=28,
    guidance_scale=4.5,
    max_sequence_length=512,
).images[0]
```

### Sending a different prompt to the T5 Text Encoder

You can send a different prompt to the CLIP Text Encoders and the T5 Text Encoder to prevent the prompt from being truncated by the CLIP Text Encoders and to improve generation.

> [!TIP]
> The prompt with the CLIP Text Encoders is still truncated to the 77 token limit.

```python
prompt = "A whimsical and creative image depicting a hybrid creature that is a mix of a waffle and a hippopotamus, basking in a river of melted butter amidst a breakfast-themed landscape. A river of warm, melted butter, pancake-like foliage in the background, a towering pepper mill standing in for a tree."

prompt_3 = "A whimsical and creative image depicting a hybrid creature that is a mix of a waffle and a hippopotamus, basking in a river of melted butter amidst a breakfast-themed landscape. It features the distinctive, bulky body shape of a hippo. However, instead of the usual grey skin, the creature’s body resembles a golden-brown, crispy waffle fresh off the griddle. The skin is textured with the familiar grid pattern of a waffle, each square filled with a glistening sheen of syrup. The environment combines the natural habitat of a hippo with elements of a breakfast table setting, a river of warm, melted butter, with oversized utensils or plates peeking out from the lush, pancake-like foliage in the background, a towering pepper mill standing in for a tree.  As the sun rises in this fantastical world, it casts a warm, buttery glow over the scene. The creature, content in its butter river, lets out a yawn. Nearby, a flock of birds take flight"

image = pipe(
    prompt=prompt,
    prompt_3=prompt_3,
    negative_prompt="",
    num_inference_steps=28,
    guidance_scale=4.5,
    max_sequence_length=512,
).images[0]
```

## Tiny AutoEncoder for Stable Diffusion 3

Tiny AutoEncoder for Stable Diffusion (TAESD3) is a tiny distilled version of Stable Diffusion 3's VAE by [Ollin Boer Bohan](https://github.com/madebyollin/taesd) that can decode [StableDiffusion3Pipeline](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/stable_diffusion_3#diffusers.StableDiffusion3Pipeline) latents almost instantly.

To use with Stable Diffusion 3:

```python
import torch
from diffusers import StableDiffusion3Pipeline, AutoencoderTiny

pipe = StableDiffusion3Pipeline.from_pretrained(
    "stabilityai/stable-diffusion-3-medium-diffusers", torch_dtype=torch.float16
)
pipe.vae = AutoencoderTiny.from_pretrained("madebyollin/taesd3", torch_dtype=torch.float16)
pipe = pipe.to("cuda")

prompt = "slice of delicious New York-style berry cheesecake"
image = pipe(prompt, num_inference_steps=25).images[0]
image.save("cheesecake.png")
```

## Loading the original checkpoints via `from_single_file`

The `SD3Transformer2DModel` and `StableDiffusion3Pipeline` classes support loading the original checkpoints via the `from_single_file` method. This method allows you to load the original checkpoint files that were used to train the models.

## Loading the original checkpoints for the `SD3Transformer2DModel`

```python
from diffusers import SD3Transformer2DModel

model = SD3Transformer2DModel.from_single_file("https://huggingface.co/stabilityai/stable-diffusion-3-medium/blob/main/sd3_medium.safetensors")
```

## Loading the single checkpoint for the `StableDiffusion3Pipeline`

### Loading the single file checkpoint without T5

```python
import torch
from diffusers import StableDiffusion3Pipeline

pipe = StableDiffusion3Pipeline.from_single_file(
    "https://huggingface.co/stabilityai/stable-diffusion-3-medium/blob/main/sd3_medium_incl_clips.safetensors",
    torch_dtype=torch.float16,
    text_encoder_3=None
)
pipe.enable_model_cpu_offload()

image = pipe("a picture of a cat holding a sign that says hello world").images[0]
image.save('sd3-single-file.png')
```

### Loading the single file checkpoint with T5

> [!TIP]
> The following example loads a checkpoint stored in a 8-bit floating point format which requires PyTorch 2.3 or later.

```python
import torch
from diffusers import StableDiffusion3Pipeline

pipe = StableDiffusion3Pipeline.from_single_file(
    "https://huggingface.co/stabilityai/stable-diffusion-3-medium/blob/main/sd3_medium_incl_clips_t5xxlfp8.safetensors",
    torch_dtype=torch.float16,
)
pipe.enable_model_cpu_offload()

image = pipe("a picture of a cat holding a sign that says hello world").images[0]
image.save('sd3-single-file-t5-fp8.png')
```

### Loading the single file checkpoint for the Stable Diffusion 3.5 Transformer Model

```python
import torch
from diffusers import SD3Transformer2DModel, StableDiffusion3Pipeline

transformer = SD3Transformer2DModel.from_single_file(
    "https://huggingface.co/stabilityai/stable-diffusion-3.5-large-turbo/blob/main/sd3.5_large.safetensors",
    torch_dtype=torch.bfloat16,
)
pipe = StableDiffusion3Pipeline.from_pretrained(
    "stabilityai/stable-diffusion-3.5-large",
    transformer=transformer,
    torch_dtype=torch.bfloat16,
)
pipe.enable_model_cpu_offload()
image = pipe("a cat holding a sign that says hello world").images[0]
image.save("sd35.png")
```

## StableDiffusion3Pipeline[[diffusers.StableDiffusion3Pipeline]]

#### diffusers.StableDiffusion3Pipeline[[diffusers.StableDiffusion3Pipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_diffusion_3/pipeline_stable_diffusion_3.py#L148)

__call__diffusers.StableDiffusion3Pipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_diffusion_3/pipeline_stable_diffusion_3.py#L772[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "prompt_2", "val": ": str | list[str] | None = None"}, {"name": "prompt_3", "val": ": str | list[str] | None = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "num_inference_steps", "val": ": int = 28"}, {"name": "sigmas", "val": ": list[float] | None = None"}, {"name": "guidance_scale", "val": ": float = 7.0"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "negative_prompt_2", "val": ": str | list[str] | None = None"}, {"name": "negative_prompt_3", "val": ": str | list[str] | None = None"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.FloatTensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.FloatTensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.FloatTensor | None = None"}, {"name": "pooled_prompt_embeds", "val": ": torch.FloatTensor | None = None"}, {"name": "negative_pooled_prompt_embeds", "val": ": torch.FloatTensor | None = None"}, {"name": "ip_adapter_image", "val": ": PIL.Image.Image | numpy.ndarray | torch.Tensor | list[PIL.Image.Image] | list[numpy.ndarray] | list[torch.Tensor] | None = None"}, {"name": "ip_adapter_image_embeds", "val": ": torch.Tensor | None = None"}, {"name": "output_type", "val": ": str | None = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "joint_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "clip_skip", "val": ": int | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[[int, int], NoneType]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list = ['latents']"}, {"name": "max_sequence_length", "val": ": int = 256"}, {"name": "skip_guidance_layers", "val": ": list = None"}, {"name": "skip_layer_guidance_scale", "val": ": float = 2.8"}, {"name": "skip_layer_guidance_stop", "val": ": float = 0.2"}, {"name": "skip_layer_guidance_start", "val": ": float = 0.01"}, {"name": "mu", "val": ": float | None = None"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`.
  instead.
- **prompt_2** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is
  will be used instead
- **prompt_3** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to be sent to `tokenizer_3` and `text_encoder_3`. If not defined, `prompt` is
  will be used instead
- **height** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The height in pixels of the generated image. This is set to 1024 by default for the best results.
- **width** (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) --
  The width in pixels of the generated image. This is set to 1024 by default for the best results.
- **num_inference_steps** (`int`, *optional*, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **sigmas** (`list[float]`, *optional*) --
  Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in
  their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed
  will be used.
- **guidance_scale** (`float`, *optional*, defaults to 7.0) --
  Guidance scale as defined in [Classifier-Free Diffusion
  Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2.
  of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting
  `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to
  the text `prompt`, usually at the expense of lower image quality.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation. If not defined, one has to pass
  `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is
  less than `1`).
- **negative_prompt_2** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation to be sent to `tokenizer_2` and
  `text_encoder_2`. If not defined, `negative_prompt` is used instead
- **negative_prompt_3** (`str` or `list[str]`, *optional*) --
  The prompt or prompts not to guide the image generation to be sent to `tokenizer_3` and
  `text_encoder_3`. If not defined, `negative_prompt` is used instead
- **num_images_per_prompt** (`int`, *optional*, defaults to 1) --
  The number of images to generate per prompt.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html)
  to make generation deterministic.
- **latents** (`torch.FloatTensor`, *optional*) --
  Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor will be generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not
  provided, text embeddings will be generated from `prompt` input argument.
- **negative_prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input
  argument.
- **pooled_prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting.
  If not provided, pooled text embeddings will be generated from `prompt` input argument.
- **negative_pooled_prompt_embeds** (`torch.FloatTensor`, *optional*) --
  Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt
  weighting. If not provided, pooled negative_prompt_embeds will be generated from `negative_prompt`
  input argument.
- **ip_adapter_image** (`PipelineImageInput`, *optional*) --
  Optional image input to work with IP Adapters.
- **ip_adapter_image_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated image embeddings for IP-Adapter. Should be a tensor of shape `(batch_size, num_images,
  emb_dim)`. It should contain the negative image embedding if `do_classifier_free_guidance` is set to
  `True`. If not provided, embeddings are computed from the `ip_adapter_image` input argument.
- **output_type** (`str`, *optional*, defaults to `"pil"`) --
  The output format of the generate image. Choose between
  [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~pipelines.stable_diffusion_3.StableDiffusion3PipelineOutput` instead of
  a plain tuple.
- **joint_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **clip_skip** (`int`, *optional*) --
  Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that
  the output of the pre-final layer will be used for computing the prompt embeddings.
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that calls at the end of each denoising steps during the inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`list`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.
- **max_sequence_length** (`int` defaults to 256) -- Maximum sequence length to use with the `prompt`.
- **skip_guidance_layers** (`list[int]`, *optional*) --
  A list of integers that specify layers to skip during guidance. If not provided, all layers will be
  used for guidance. If provided, the guidance will only be applied to the layers specified in the list.
  Recommended value by StabiltyAI for Stable Diffusion 3.5 Medium is [7, 8, 9].
- **skip_layer_guidance_scale** (`int`, *optional*) -- The scale of the guidance for the layers specified in
  `skip_guidance_layers`. The guidance will be applied to the layers specified in `skip_guidance_layers`
  with a scale of `skip_layer_guidance_scale`. The guidance will be applied to the rest of the layers
  with a scale of `1`.
- **skip_layer_guidance_stop** (`int`, *optional*) -- The step at which the guidance for the layers specified in
  `skip_guidance_layers` will stop. The guidance will be applied to the layers specified in
  `skip_guidance_layers` until the fraction specified in `skip_layer_guidance_stop`. Recommended value by
  StabiltyAI for Stable Diffusion 3.5 Medium is 0.2.
- **skip_layer_guidance_start** (`int`, *optional*) -- The step at which the guidance for the layers specified in
  `skip_guidance_layers` will start. The guidance will be applied to the layers specified in
  `skip_guidance_layers` from the fraction specified in `skip_layer_guidance_start`. Recommended value by
  StabiltyAI for Stable Diffusion 3.5 Medium is 0.01.
- **mu** (`float`, *optional*) -- `mu` value used for `dynamic_shifting`.0`~pipelines.stable_diffusion_3.StableDiffusion3PipelineOutput` or `tuple``~pipelines.stable_diffusion_3.StableDiffusion3PipelineOutput` if `return_dict` is True, otherwise a
`tuple`. When returning a tuple, the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import StableDiffusion3Pipeline

>>> pipe = StableDiffusion3Pipeline.from_pretrained(
...     "stabilityai/stable-diffusion-3-medium-diffusers", torch_dtype=torch.float16
... )
>>> pipe.to("cuda")
>>> prompt = "A cat holding a sign that says hello world"
>>> image = pipe(prompt).images[0]
>>> image.save("sd3.png")
```

**Parameters:**

transformer ([SD3Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/sd3_transformer2d#diffusers.SD3Transformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModelWithProjection`) : [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModelWithProjection), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant, with an additional added projection layer that is initialized with a diagonal matrix with the `hidden_size` as its dimension.

text_encoder_2 (`CLIPTextModelWithProjection`) : [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModelWithProjection), specifically the [laion/CLIP-ViT-bigG-14-laion2B-39B-b160k](https://huggingface.co/laion/CLIP-ViT-bigG-14-laion2B-39B-b160k) variant.

text_encoder_3 (`T5EncoderModel`) : Frozen text-encoder. Stable Diffusion 3 uses [T5](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5EncoderModel), specifically the [t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer_2 (`CLIPTokenizer`) : Second Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/v4.21.0/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer_3 (`T5TokenizerFast`) : Tokenizer of class [T5Tokenizer](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5Tokenizer).

image_encoder (`SiglipVisionModel`, *optional*) : Pre-trained Vision Model for IP Adapter.

feature_extractor (`SiglipImageProcessor`, *optional*) : Image processor for IP Adapter.

**Returns:**

``~pipelines.stable_diffusion_3.StableDiffusion3PipelineOutput` or `tuple``

`~pipelines.stable_diffusion_3.StableDiffusion3PipelineOutput` if `return_dict` is True, otherwise a
`tuple`. When returning a tuple, the first element is a list with the generated images.
#### encode_image[[diffusers.StableDiffusion3Pipeline.encode_image]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_diffusion_3/pipeline_stable_diffusion_3.py#L696)

Encodes the given image into a feature representation using a pre-trained image encoder.

**Parameters:**

image (`PipelineImageInput`) : Input image to be encoded.

device : (`torch.device`): Torch device.

**Returns:**

``torch.Tensor``

The encoded image feature representation.
#### encode_prompt[[diffusers.StableDiffusion3Pipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_diffusion_3/pipeline_stable_diffusion_3.py#L344)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is used in all text-encoders

prompt_3 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to the `tokenizer_3` and `text_encoder_3`. If not defined, `prompt` is used in all text-encoders

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

do_classifier_free_guidance (`bool`) : whether to use classifier free guidance or not

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

negative_prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `negative_prompt` is used in all the text-encoders.

negative_prompt_3 (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation to be sent to `tokenizer_3` and `text_encoder_3`. If not defined, `negative_prompt` is used in all the text-encoders.

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

negative_pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled negative_prompt_embeds will be generated from `negative_prompt` input argument.

clip_skip (`int`, *optional*) : Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that the output of the pre-final layer will be used for computing the prompt embeddings.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.
#### prepare_ip_adapter_image_embeds[[diffusers.StableDiffusion3Pipeline.prepare_ip_adapter_image_embeds]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/stable_diffusion_3/pipeline_stable_diffusion_3.py#L716)

Prepares image embeddings for use in the IP-Adapter.

Either `ip_adapter_image` or `ip_adapter_image_embeds` must be passed.

**Parameters:**

ip_adapter_image (`PipelineImageInput`, *optional*) : The input image to extract features from for IP-Adapter.

ip_adapter_image_embeds (`torch.Tensor`, *optional*) : Precomputed image embeddings.

device : (`torch.device`, *optional*): Torch device.

num_images_per_prompt (`int`, defaults to 1) : Number of images that should be generated per prompt.

do_classifier_free_guidance (`bool`, defaults to True) : Whether to use classifier free guidance or not.
