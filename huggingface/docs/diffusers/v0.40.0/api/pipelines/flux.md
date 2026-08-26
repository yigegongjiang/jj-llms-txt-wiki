# Flux

  
  

Flux is a series of text-to-image generation models based on diffusion transformers. To know more about Flux, check out the original [blog post](https://blackforestlabs.ai/announcing-black-forest-labs/) by the creators of Flux, Black Forest Labs.

Original model checkpoints for Flux can be found [here](https://huggingface.co/black-forest-labs). Original inference code can be found [here](https://github.com/black-forest-labs/flux).

> [!TIP]
> Flux can be quite expensive to run on consumer hardware devices. However, you can perform a suite of optimizations to run it faster and in a more memory-friendly manner. Check out [this section](https://huggingface.co/blog/sd3#memory-optimizations-for-sd3) for more details. Additionally, Flux can benefit from quantization for memory efficiency with a trade-off in inference latency. Refer to [this blog post](https://huggingface.co/blog/quanto-diffusers) to learn more.  For an exhaustive list of resources, check out [this gist](https://gist.github.com/sayakpaul/b664605caf0aa3bf8585ab109dd5ac9c).
>
> [Caching](../../optimization/cache) may also speed up inference by storing and reusing intermediate outputs.

Flux comes in the following variants:

| model type | model id |
|:----------:|:--------:|
| Timestep-distilled | [`black-forest-labs/FLUX.1-schnell`](https://huggingface.co/black-forest-labs/FLUX.1-schnell) |
| Guidance-distilled | [`black-forest-labs/FLUX.1-dev`](https://huggingface.co/black-forest-labs/FLUX.1-dev) |
| Fill Inpainting/Outpainting (Guidance-distilled) | [`black-forest-labs/FLUX.1-Fill-dev`](https://huggingface.co/black-forest-labs/FLUX.1-Fill-dev) |
| Canny Control (Guidance-distilled) | [`black-forest-labs/FLUX.1-Canny-dev`](https://huggingface.co/black-forest-labs/FLUX.1-Canny-dev) |
| Depth Control (Guidance-distilled) | [`black-forest-labs/FLUX.1-Depth-dev`](https://huggingface.co/black-forest-labs/FLUX.1-Depth-dev) |
| Canny Control (LoRA) | [`black-forest-labs/FLUX.1-Canny-dev-lora`](https://huggingface.co/black-forest-labs/FLUX.1-Canny-dev-lora) |
| Depth Control (LoRA) | [`black-forest-labs/FLUX.1-Depth-dev-lora`](https://huggingface.co/black-forest-labs/FLUX.1-Depth-dev-lora) |
| Redux (Adapter) | [`black-forest-labs/FLUX.1-Redux-dev`](https://huggingface.co/black-forest-labs/FLUX.1-Redux-dev) |
| Kontext | [`black-forest-labs/FLUX.1-kontext`](https://huggingface.co/black-forest-labs/FLUX.1-Kontext-dev) |

All checkpoints have different usage which we detail below.

### Timestep-distilled

* `max_sequence_length` cannot be more than 256.
* `guidance_scale` needs to be 0.
* As this is a timestep-distilled model, it benefits from fewer sampling steps.

```python
import torch
from diffusers import FluxPipeline

pipe = FluxPipeline.from_pretrained("black-forest-labs/FLUX.1-schnell", dtype=torch.bfloat16)
pipe.enable_model_cpu_offload()

prompt = "A cat holding a sign that says hello world"
out = pipe(
    prompt=prompt,
    guidance_scale=0.,
    height=768,
    width=1360,
    num_inference_steps=4,
    max_sequence_length=256,
).images[0]
out.save("image.png")
```

### Guidance-distilled

* The guidance-distilled variant takes about 50 sampling steps for good-quality generation.
* It doesn't have any limitations around the `max_sequence_length`.

```python
import torch
from diffusers import FluxPipeline

pipe = FluxPipeline.from_pretrained("black-forest-labs/FLUX.1-dev", dtype=torch.bfloat16)
pipe.enable_model_cpu_offload()

prompt = "a tiny astronaut hatching from an egg on the moon"
out = pipe(
    prompt=prompt,
    guidance_scale=3.5,
    height=768,
    width=1360,
    num_inference_steps=50,
).images[0]
out.save("image.png")
```

### Fill Inpainting/Outpainting

* Flux Fill pipeline does not require `strength` as an input like regular inpainting pipelines.
* It supports both inpainting and outpainting.

```python
import torch
from diffusers import FluxFillPipeline
from diffusers.utils import load_image

image = load_image("https://huggingface.co/datasets/YiYiXu/testing-images/resolve/main/cup.png")
mask = load_image("https://huggingface.co/datasets/YiYiXu/testing-images/resolve/main/cup_mask.png")

repo_id = "black-forest-labs/FLUX.1-Fill-dev"
pipe = FluxFillPipeline.from_pretrained(repo_id, dtype=torch.bfloat16).to("cuda")

image = pipe(
    prompt="a white paper cup",
    image=image,
    mask_image=mask,
    height=1632,
    width=1232,
    max_sequence_length=512,
    generator=torch.Generator("cpu").manual_seed(0)
).images[0]
image.save(f"output.png")
```

### Canny Control

**Note:** `black-forest-labs/Flux.1-Canny-dev` is _not_ a [ControlNetModel](/docs/diffusers/v0.40.0/en/api/models/controlnet#diffusers.ControlNetModel) model. ControlNet models are a separate component from the UNet/Transformer whose residuals are added to the actual underlying model. Canny Control is an alternate architecture that achieves effectively the same results as a ControlNet model would, by using channel-wise concatenation with input control condition and ensuring the transformer learns structure control by following the condition as closely as possible. 

```python
# !pip install -U controlnet-aux
import torch
from controlnet_aux import CannyDetector
from diffusers import FluxControlPipeline
from diffusers.utils import load_image

pipe = FluxControlPipeline.from_pretrained("black-forest-labs/FLUX.1-Canny-dev", dtype=torch.bfloat16).to("cuda")

prompt = "A robot made of exotic candies and chocolates of different kinds. The background is filled with confetti and celebratory gifts."
control_image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/robot.png")

processor = CannyDetector()
control_image = processor(control_image, low_threshold=50, high_threshold=200, detect_resolution=1024, image_resolution=1024)

image = pipe(
    prompt=prompt,
    control_image=control_image,
    height=1024,
    width=1024,
    num_inference_steps=50,
    guidance_scale=30.0,
).images[0]
image.save("output.png")
```

Canny Control is also possible with a LoRA variant of this condition. The usage is as follows:

```python
# !pip install -U controlnet-aux
import torch
from controlnet_aux import CannyDetector
from diffusers import FluxControlPipeline
from diffusers.utils import load_image

pipe = FluxControlPipeline.from_pretrained("black-forest-labs/FLUX.1-dev", dtype=torch.bfloat16).to("cuda")
pipe.load_lora_weights("black-forest-labs/FLUX.1-Canny-dev-lora")

prompt = "A robot made of exotic candies and chocolates of different kinds. The background is filled with confetti and celebratory gifts."
control_image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/robot.png")

processor = CannyDetector()
control_image = processor(control_image, low_threshold=50, high_threshold=200, detect_resolution=1024, image_resolution=1024)

image = pipe(
    prompt=prompt,
    control_image=control_image,
    height=1024,
    width=1024,
    num_inference_steps=50,
    guidance_scale=30.0,
).images[0]
image.save("output.png")
```

### Depth Control

**Note:** `black-forest-labs/Flux.1-Depth-dev` is _not_ a ControlNet model. [ControlNetModel](/docs/diffusers/v0.40.0/en/api/models/controlnet#diffusers.ControlNetModel) models are a separate component from the UNet/Transformer whose residuals are added to the actual underlying model. Depth Control is an alternate architecture that achieves effectively the same results as a ControlNet model would, by using channel-wise concatenation with input control condition and ensuring the transformer learns structure control by following the condition as closely as possible.

```python
# !pip install git+https://github.com/huggingface/image_gen_aux
import torch
from diffusers import FluxControlPipeline, FluxTransformer2DModel
from diffusers.utils import load_image
from image_gen_aux import DepthPreprocessor

pipe = FluxControlPipeline.from_pretrained("black-forest-labs/FLUX.1-Depth-dev", dtype=torch.bfloat16).to("cuda")

prompt = "A robot made of exotic candies and chocolates of different kinds. The background is filled with confetti and celebratory gifts."
control_image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/robot.png")

processor = DepthPreprocessor.from_pretrained("LiheYoung/depth-anything-large-hf")
control_image = processor(control_image)[0].convert("RGB")

image = pipe(
    prompt=prompt,
    control_image=control_image,
    height=1024,
    width=1024,
    num_inference_steps=30,
    guidance_scale=10.0,
    generator=torch.Generator().manual_seed(42),
).images[0]
image.save("output.png")
```

Depth Control is also possible with a LoRA variant of this condition. The usage is as follows:

```python
# !pip install git+https://github.com/huggingface/image_gen_aux
import torch
from diffusers import FluxControlPipeline, FluxTransformer2DModel
from diffusers.utils import load_image
from image_gen_aux import DepthPreprocessor

pipe = FluxControlPipeline.from_pretrained("black-forest-labs/FLUX.1-dev", dtype=torch.bfloat16).to("cuda")
pipe.load_lora_weights("black-forest-labs/FLUX.1-Depth-dev-lora")

prompt = "A robot made of exotic candies and chocolates of different kinds. The background is filled with confetti and celebratory gifts."
control_image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/robot.png")

processor = DepthPreprocessor.from_pretrained("LiheYoung/depth-anything-large-hf")
control_image = processor(control_image)[0].convert("RGB")

image = pipe(
    prompt=prompt,
    control_image=control_image,
    height=1024,
    width=1024,
    num_inference_steps=30,
    guidance_scale=10.0,
    generator=torch.Generator().manual_seed(42),
).images[0]
image.save("output.png")
```

### Redux

* Flux Redux pipeline is an adapter for FLUX.1 base models. It can be used with both flux-dev and flux-schnell, for image-to-image generation.
* You can first use the `FluxPriorReduxPipeline` to get the `prompt_embeds` and `pooled_prompt_embeds`, and then feed them into the `FluxPipeline` for image-to-image generation.
* When use `FluxPriorReduxPipeline` with a base pipeline, you can set `text_encoder=None` and `text_encoder_2=None` in the base pipeline, in order to save VRAM.

```python
import torch
from diffusers import FluxPriorReduxPipeline, FluxPipeline
from diffusers.utils import load_image
device = "cuda"
dtype = torch.bfloat16

repo_redux = "black-forest-labs/FLUX.1-Redux-dev"
repo_base = "black-forest-labs/FLUX.1-dev" 
pipe_prior_redux = FluxPriorReduxPipeline.from_pretrained(repo_redux, dtype=dtype).to(device)
pipe = FluxPipeline.from_pretrained(
    repo_base, 
    text_encoder=None,
    text_encoder_2=None,
    dtype=torch.bfloat16
).to(device)

image = load_image("https://huggingface.co/datasets/YiYiXu/testing-images/resolve/main/style_ziggy/img5.png")
pipe_prior_output = pipe_prior_redux(image)
images = pipe(
    guidance_scale=2.5,
    num_inference_steps=50,
    generator=torch.Generator("cpu").manual_seed(0),
    **pipe_prior_output,
).images
images[0].save("flux-redux.png")
```

### Kontext

Flux Kontext is a model that allows in-context control of the image generation process, allowing for editing, refinement, relighting, style transfer, character customization, and more.

```python
import torch
from diffusers import FluxKontextPipeline
from diffusers.utils import load_image

pipe = FluxKontextPipeline.from_pretrained(
    "black-forest-labs/FLUX.1-Kontext-dev", dtype=torch.bfloat16
)
pipe.to("cuda")

image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/yarn-art-pikachu.png").convert("RGB")
prompt = "Make Pikachu hold a sign that says 'Black Forest Labs is awesome', yarn art style, detailed, vibrant colors"
image = pipe(
    image=image,
    prompt=prompt,
    guidance_scale=2.5,
    generator=torch.Generator().manual_seed(42),
).images[0]
image.save("flux-kontext.png")
```

Flux Kontext comes with an integrity safety checker, which should be run after the image generation step. To run the safety checker, install the official repository from [black-forest-labs/flux](https://github.com/black-forest-labs/flux) and add the following code:

```python
from flux.content_filters import PixtralContentFilter

# ... pipeline invocation to generate images

integrity_checker = PixtralContentFilter(torch.device("cuda"))
image_ = np.array(image) / 255.0
image_ = 2 * image_ - 1
image_ = torch.from_numpy(image_).to("cuda", dtype=torch.float32).unsqueeze(0).permute(0, 3, 1, 2)
if integrity_checker.test_image(image_):
    raise ValueError("Your image has been flagged. Choose another prompt/image or try again.")
```

### Kontext Inpainting
`FluxKontextInpaintPipeline` enables image modification within a fixed mask region. It currently supports both text-based conditioning and image-reference conditioning.

```python
import torch
from diffusers import FluxKontextInpaintPipeline
from diffusers.utils import load_image

prompt = "Change the yellow dinosaur to green one"
img_url = (
    "https://github.com/ZenAI-Vietnam/Flux-Kontext-pipelines/blob/main/assets/dinosaur_input.jpeg?raw=true"
)
mask_url = (
    "https://github.com/ZenAI-Vietnam/Flux-Kontext-pipelines/blob/main/assets/dinosaur_mask.png?raw=true"
)

source = load_image(img_url)
mask = load_image(mask_url)

pipe = FluxKontextInpaintPipeline.from_pretrained(
    "black-forest-labs/FLUX.1-Kontext-dev", dtype=torch.bfloat16
)
pipe.to("cuda")

image = pipe(prompt=prompt, image=source, mask_image=mask, strength=1.0).images[0]
image.save("kontext_inpainting_normal.png")
```

```python
import torch
from diffusers import FluxKontextInpaintPipeline
from diffusers.utils import load_image

pipe = FluxKontextInpaintPipeline.from_pretrained(
    "black-forest-labs/FLUX.1-Kontext-dev", dtype=torch.bfloat16
)
pipe.to("cuda")

prompt = "Replace this ball"
img_url = "https://images.pexels.com/photos/39362/the-ball-stadion-football-the-pitch-39362.jpeg?auto=compress&cs=tinysrgb&dpr=1&w=500"
mask_url = "https://github.com/ZenAI-Vietnam/Flux-Kontext-pipelines/blob/main/assets/ball_mask.png?raw=true"
image_reference_url = "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTah3x6OL_ECMBaZ5ZlJJhNsyC-OSMLWAI-xw&s"

source = load_image(img_url)
mask = load_image(mask_url)
image_reference = load_image(image_reference_url)

mask = pipe.mask_processor.blur(mask, blur_factor=12)
image = pipe(
    prompt=prompt, image=source, mask_image=mask, image_reference=image_reference, strength=1.0
).images[0]
image.save("kontext_inpainting_ref.png")
```

## Combining Flux Turbo LoRAs with Flux Control, Fill, and Redux

We can combine Flux Turbo LoRAs with Flux Control and other pipelines like Fill and Redux to enable few-steps' inference. The example below shows how to do that for Flux Control LoRA for depth and turbo LoRA from [`ByteDance/Hyper-SD`](https://hf.co/ByteDance/Hyper-SD).

```py
from diffusers import FluxControlPipeline
from image_gen_aux import DepthPreprocessor
from diffusers.utils import load_image
from huggingface_hub import hf_hub_download
import torch

control_pipe = FluxControlPipeline.from_pretrained("black-forest-labs/FLUX.1-dev", dtype=torch.bfloat16)
control_pipe.load_lora_weights("black-forest-labs/FLUX.1-Depth-dev-lora", adapter_name="depth")
control_pipe.load_lora_weights(
    hf_hub_download("ByteDance/Hyper-SD", "Hyper-FLUX.1-dev-8steps-lora.safetensors"), adapter_name="hyper-sd"
)
control_pipe.set_adapters(["depth", "hyper-sd"], adapter_weights=[0.85, 0.125])
control_pipe.enable_model_cpu_offload()

prompt = "A robot made of exotic candies and chocolates of different kinds. The background is filled with confetti and celebratory gifts."
control_image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/robot.png")

processor = DepthPreprocessor.from_pretrained("LiheYoung/depth-anything-large-hf")
control_image = processor(control_image)[0].convert("RGB")

image = control_pipe(
    prompt=prompt,
    control_image=control_image,
    height=1024,
    width=1024,
    num_inference_steps=8,
    guidance_scale=10.0,
    generator=torch.Generator().manual_seed(42),
).images[0]
image.save("output.png")
```

## Note about `unload_lora_weights()` when using Flux LoRAs

When unloading the Control LoRA weights, call `pipe.unload_lora_weights(reset_to_overwritten_params=True)` to reset the `pipe.transformer` completely back to its original form. The resultant pipeline can then be used with methods like [DiffusionPipeline.from_pipe()](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline.from_pipe). More details about this argument are available in [this PR](https://github.com/huggingface/diffusers/pull/10397).

## IP-Adapter

> [!TIP]
> Check out [IP-Adapter](../../using-diffusers/ip_adapter) to learn more about how IP-Adapters work.

An IP-Adapter lets you prompt Flux with images, in addition to the text prompt. This is especially useful when describing complex concepts that are difficult to articulate through text alone and you have reference images.

```python
import torch
from diffusers import FluxPipeline
from diffusers.utils import load_image

pipe = FluxPipeline.from_pretrained(
    "black-forest-labs/FLUX.1-dev", dtype=torch.bfloat16
).to("cuda")

image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/flux_ip_adapter_input.jpg").resize((1024, 1024))

pipe.load_ip_adapter(
    "XLabs-AI/flux-ip-adapter",
    weight_name="ip_adapter.safetensors",
    image_encoder_pretrained_model_name_or_path="openai/clip-vit-large-patch14"
)
pipe.set_ip_adapter_scale(1.0)

image = pipe(
    width=1024,
    height=1024,
    prompt="wearing sunglasses",
    negative_prompt="",
    true_cfg_scale=4.0,
    generator=torch.Generator().manual_seed(4444),
    ip_adapter_image=image,
).images[0]

image.save('flux_ip_adapter_output.jpg')
```

    
    IP-Adapter examples with prompt "wearing sunglasses"

## Optimize

Flux is a very large model and requires ~50GB of RAM/VRAM to load all the modeling components. Enable some of the optimizations below to lower the memory requirements.

### Group offloading

[Group offloading](../../optimization/memory#group-offloading) lowers VRAM usage by offloading groups of internal layers rather than the whole model or weights. You need to use [apply_group_offloading()](/docs/diffusers/v0.40.0/en/api/utilities#diffusers.hooks.apply_group_offloading) on all the model components of a pipeline. The `offload_type` parameter allows you to toggle between block and leaf-level offloading. Setting it to `leaf_level` offloads the lowest leaf-level parameters to the CPU instead of offloading at the module-level.

On CUDA devices that support asynchronous data streaming, set `use_stream=True` to overlap data transfer and computation to accelerate inference.

> [!TIP]
> It is possible to mix block and leaf-level offloading for different components in a pipeline.

```py
import torch
from diffusers import FluxPipeline
from diffusers.hooks import apply_group_offloading

model_id = "black-forest-labs/FLUX.1-dev"
dtype = torch.bfloat16
pipe = FluxPipeline.from_pretrained(
	model_id,
	dtype=dtype,
)

apply_group_offloading(
    pipe.transformer,
    offload_type="leaf_level",
    offload_device=torch.device("cpu"),
    onload_device=torch.device("cuda"),
    use_stream=True,
)
apply_group_offloading(
    pipe.text_encoder, 
    offload_device=torch.device("cpu"),
    onload_device=torch.device("cuda"),
    offload_type="leaf_level",
    use_stream=True,
)
apply_group_offloading(
    pipe.text_encoder_2, 
    offload_device=torch.device("cpu"),
    onload_device=torch.device("cuda"),
    offload_type="leaf_level",
    use_stream=True,
)
apply_group_offloading(
    pipe.vae, 
    offload_device=torch.device("cpu"),
    onload_device=torch.device("cuda"),
    offload_type="leaf_level",
    use_stream=True,
)

prompt="A cat wearing sunglasses and working as a lifeguard at pool."

generator = torch.Generator().manual_seed(181201)
image = pipe(
    prompt,
    width=576,
    height=1024,
    num_inference_steps=30,
    generator=generator
).images[0]
image
```

### Running FP16 inference

Flux can generate high-quality images with FP16 (i.e. to accelerate inference on Turing/Volta GPUs) but produces different outputs compared to FP32/BF16. The issue is that some activations in the text encoders have to be clipped when running in FP16, which affects the overall image. Forcing text encoders to run with FP32 inference thus removes this output difference. See [here](https://github.com/huggingface/diffusers/pull/9097#issuecomment-2272292516) for details.

FP16 inference code:
```python
import torch
from diffusers import FluxPipeline

pipe = FluxPipeline.from_pretrained("black-forest-labs/FLUX.1-schnell", dtype=torch.bfloat16) # can replace schnell with dev
# to run on low vram GPUs (i.e. between 4 and 32 GB VRAM)
pipe.enable_sequential_cpu_offload()
pipe.vae.enable_slicing()
pipe.vae.enable_tiling()

pipe.to(torch.float16) # casting here instead of in the pipeline constructor because doing so in the constructor loads all models into CPU memory at once

prompt = "A cat holding a sign that says hello world"
out = pipe(
    prompt=prompt,
    guidance_scale=0.,
    height=768,
    width=1360,
    num_inference_steps=4,
    max_sequence_length=256,
).images[0]
out.save("image.png")
```

### Quantization

Quantization helps reduce the memory requirements of very large models by storing model weights in a lower precision data type. However, quantization may have varying impact on video quality depending on the video model.

Refer to the [Quantization](../../quantization/overview) overview to learn more about supported quantization backends and selecting a quantization backend that supports your use case. The example below demonstrates how to load a quantized [FluxPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/flux#diffusers.FluxPipeline) for inference with bitsandbytes.

```py
import torch
from diffusers import BitsAndBytesConfig as DiffusersBitsAndBytesConfig, FluxTransformer2DModel, FluxPipeline
from transformers import BitsAndBytesConfig as BitsAndBytesConfig, T5EncoderModel

quant_config = BitsAndBytesConfig(load_in_8bit=True)
text_encoder_8bit = T5EncoderModel.from_pretrained(
    "black-forest-labs/FLUX.1-dev",
    subfolder="text_encoder_2",
    quantization_config=quant_config,
    dtype=torch.float16,
)

quant_config = DiffusersBitsAndBytesConfig(load_in_8bit=True)
transformer_8bit = FluxTransformer2DModel.from_pretrained(
    "black-forest-labs/FLUX.1-dev",
    subfolder="transformer",
    quantization_config=quant_config,
    dtype=torch.float16,
)

pipeline = FluxPipeline.from_pretrained(
    "black-forest-labs/FLUX.1-dev",
    text_encoder_2=text_encoder_8bit,
    transformer=transformer_8bit,
    dtype=torch.float16,
    device_map="balanced",
)

prompt = "a tiny astronaut hatching from an egg on the moon"
image = pipeline(prompt, guidance_scale=3.5, height=768, width=1360, num_inference_steps=50).images[0]
image.save("flux.png")
```

## Single File Loading for the `FluxTransformer2DModel`

The `FluxTransformer2DModel` supports loading checkpoints in the original format shipped by Black Forest Labs. This is also useful when trying to load finetunes or quantized versions of the models that have been published by the community.

> [!TIP]
> `FP8` inference can be brittle depending on the GPU type, CUDA version, and `torch` version that you are using. It is recommended that you use the `optimum-quanto` library in order to run FP8 inference on your machine.

The following example demonstrates how to run Flux with less than 16GB of VRAM.

First install `optimum-quanto`

```shell
pip install optimum-quanto
```

Then run the following example

```python
import torch
from diffusers import FluxTransformer2DModel, FluxPipeline
from transformers import T5EncoderModel, CLIPTextModel
from optimum.quanto import freeze, qfloat8, quantize

bfl_repo = "black-forest-labs/FLUX.1-dev"
dtype = torch.bfloat16

transformer = FluxTransformer2DModel.from_single_file("https://huggingface.co/Kijai/flux-fp8/blob/main/flux1-dev-fp8.safetensors", dtype=dtype)
quantize(transformer, weights=qfloat8)
freeze(transformer)

text_encoder_2 = T5EncoderModel.from_pretrained(bfl_repo, subfolder="text_encoder_2", dtype=dtype)
quantize(text_encoder_2, weights=qfloat8)
freeze(text_encoder_2)

pipe = FluxPipeline.from_pretrained(bfl_repo, transformer=None, text_encoder_2=None, dtype=dtype)
pipe.transformer = transformer
pipe.text_encoder_2 = text_encoder_2

pipe.enable_model_cpu_offload()

prompt = "A cat holding a sign that says hello world"
image = pipe(
    prompt,
    guidance_scale=3.5,
    output_type="pil",
    num_inference_steps=20,
    generator=torch.Generator("cpu").manual_seed(0)
).images[0]

image.save("flux-fp8-dev.png")
```

## FluxPipeline[[diffusers.FluxPipeline]]

#### diffusers.FluxPipeline[[diffusers.FluxPipeline]]

```python
diffusers.FluxPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKL, text_encoder: CLIPTextModel, tokenizer: CLIPTokenizer, text_encoder_2: T5EncoderModel, tokenizer_2: T5Tokenizer, transformer: FluxTransformer2DModel, image_encoder: CLIPVisionModelWithProjection = None, feature_extractor: CLIPImageProcessorPil = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux.py#L146)

**Parameters:**

transformer ([FluxTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/flux_transformer#diffusers.FluxTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

text_encoder_2 (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer_2 (`T5TokenizerFast`) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

The Flux pipeline for text-to-image generation.

Reference: https://blackforestlabs.ai/announcing-black-forest-labs/

#### __call__[[diffusers.FluxPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, prompt_2: str | list[str] | None = None, negative_prompt: str | list[str] = None, negative_prompt_2: str | list[str] | None = None, true_cfg_scale: float = 1.0, height: int | None = None, width: int | None = None, num_inference_steps: int = 28, sigmas: list[float] | None = None, guidance_scale: float = 3.5, num_images_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.FloatTensor] = None, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, ip_adapter_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, ip_adapter_image_embeds: list[torch.Tensor] | None = None, negative_ip_adapter_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, negative_ip_adapter_image_embeds: list[torch.Tensor] | None = None, negative_prompt_embeds: typing.Optional[torch.FloatTensor] = None, negative_pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, output_type: str | None = 'pil', return_dict: bool = True, joint_attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux.py#L598)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is will be used instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `true_cfg_scale` is not greater than `1`).

negative_prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `negative_prompt` is used in all the text-encoders.

true_cfg_scale (`float`, *optional*, defaults to 1.0) : True classifier-free guidance (guidance scale) is enabled when `true_cfg_scale` > 1 and `negative_prompt` is provided.

height (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The height in pixels of the generated image. This is set to 1024 by default for the best results.

width (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The width in pixels of the generated image. This is set to 1024 by default for the best results.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`list[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

guidance_scale (`float`, *optional*, defaults to 3.5) : Embedded guiddance scale is enabled by setting `guidance_scale` > 1. Higher `guidance_scale` encourages a model to generate images more aligned with `prompt` at the expense of lower image quality.  Guidance-distilled models approximates true classifer-free guidance for `guidance_scale` > 1. Refer to the [paper](https://huggingface.co/papers/2210.03142) to learn more.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.FloatTensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

ip_adapter_image : (`PipelineImageInput`, *optional*): Optional image input to work with IP Adapters.

ip_adapter_image_embeds (`list[torch.Tensor]`, *optional*) : Pre-generated image embeddings for IP-Adapter. It should be a list of length same as number of IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. If not provided, embeddings are computed from the `ip_adapter_image` input argument.

negative_ip_adapter_image : (`PipelineImageInput`, *optional*): Optional image input to work with IP Adapters.

negative_ip_adapter_image_embeds (`list[torch.Tensor]`, *optional*) : Pre-generated image embeddings for IP-Adapter. It should be a list of length same as number of IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. If not provided, embeddings are computed from the `ip_adapter_image` input argument.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

negative_pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled negative_prompt_embeds will be generated from `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.flux.FluxPipelineOutput` instead of a plain tuple.

joint_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int` defaults to 512) : Maximum sequence length to use with the `prompt`.

**Returns:** `~pipelines.flux.FluxPipelineOutput` or `tuple`

`~pipelines.flux.FluxPipelineOutput` if `return_dict`
is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the generated
images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import FluxPipeline

>>> pipe = FluxPipeline.from_pretrained("black-forest-labs/FLUX.1-schnell", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")
>>> prompt = "A cat holding a sign that says hello world"
>>> # Depending on the variant being used, the pipeline call will slightly vary.
>>> # Refer to the pipeline documentation for more details.
>>> image = pipe(prompt, num_inference_steps=4, guidance_scale=0.0).images[0]
>>> image.save("flux.png")
```

#### encode_prompt[[diffusers.FluxPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], prompt_2: str | list[str] | None = None, device: typing.Optional[torch.device] = None, num_images_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, max_sequence_length: int = 512, lora_scale: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux.py#L310)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is used in all text-encoders

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

## FluxImg2ImgPipeline[[diffusers.FluxImg2ImgPipeline]]

#### diffusers.FluxImg2ImgPipeline[[diffusers.FluxImg2ImgPipeline]]

```python
diffusers.FluxImg2ImgPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKL, text_encoder: CLIPTextModel, tokenizer: CLIPTokenizer, text_encoder_2: T5EncoderModel, tokenizer_2: T5Tokenizer, transformer: FluxTransformer2DModel, image_encoder: CLIPVisionModelWithProjection = None, feature_extractor: CLIPImageProcessorPil = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_img2img.py#L169)

**Parameters:**

transformer ([FluxTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/flux_transformer#diffusers.FluxTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

text_encoder_2 (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer_2 (`T5TokenizerFast`) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

The Flux pipeline for image inpainting.

Reference: https://blackforestlabs.ai/announcing-black-forest-labs/

#### __call__[[diffusers.FluxImg2ImgPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, prompt_2: str | list[str] | None = None, negative_prompt: str | list[str] = None, negative_prompt_2: str | list[str] | None = None, true_cfg_scale: float = 1.0, image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, height: int | None = None, width: int | None = None, strength: float = 0.6, num_inference_steps: int = 28, sigmas: list[float] | None = None, guidance_scale: float = 7.0, num_images_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.FloatTensor] = None, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, ip_adapter_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, ip_adapter_image_embeds: list[torch.Tensor] | None = None, negative_ip_adapter_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, negative_ip_adapter_image_embeds: list[torch.Tensor] | None = None, negative_prompt_embeds: typing.Optional[torch.FloatTensor] = None, negative_pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, output_type: str | None = 'pil', return_dict: bool = True, joint_attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_img2img.py#L676)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is will be used instead

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `true_cfg_scale` is not greater than `1`).

negative_prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `negative_prompt` is used in all the text-encoders.

true_cfg_scale (`float`, *optional*, defaults to 1.0) : True classifier-free guidance (guidance scale) is enabled when `true_cfg_scale` > 1 and `negative_prompt` is provided.

image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) : `Image`, numpy array or tensor representing an image batch to be used as the starting point. For both numpy array and pytorch tensor, the expected value range is between `[0, 1]` If it's a tensor or a list or tensors, the expected shape should be `(B, C, H, W)` or `(C, H, W)`. If it is a numpy array or a list of arrays, the expected shape should be `(B, H, W, C)` or `(H, W, C)` It can also accept image latents as `image`, but if passing latents directly it is not encoded again.

height (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The height in pixels of the generated image. This is set to 1024 by default for the best results.

width (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The width in pixels of the generated image. This is set to 1024 by default for the best results.

strength (`float`, *optional*, defaults to 1.0) : Indicates extent to transform the reference `image`. Must be between 0 and 1. `image` is used as a starting point and more noise is added the higher the `strength`. The number of denoising steps depends on the amount of noise initially added. When `strength` is 1, added noise is maximum and the denoising process runs for the full number of iterations specified in `num_inference_steps`. A value of 1 essentially ignores `image`.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`list[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

guidance_scale (`float`, *optional*, defaults to 7.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.FloatTensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

negative_pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled negative_prompt_embeds will be generated from `negative_prompt` input argument.

ip_adapter_image : (`PipelineImageInput`, *optional*): Optional image input to work with IP Adapters.

ip_adapter_image_embeds (`list[torch.Tensor]`, *optional*) : Pre-generated image embeddings for IP-Adapter. It should be a list of length same as number of IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. If not provided, embeddings are computed from the `ip_adapter_image` input argument.

negative_ip_adapter_image : (`PipelineImageInput`, *optional*): Optional image input to work with IP Adapters.

negative_ip_adapter_image_embeds (`list[torch.Tensor]`, *optional*) : Pre-generated image embeddings for IP-Adapter. It should be a list of length same as number of IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. If not provided, embeddings are computed from the `ip_adapter_image` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.flux.FluxPipelineOutput` instead of a plain tuple.

joint_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int` defaults to 512) : Maximum sequence length to use with the `prompt`.

**Returns:** `~pipelines.flux.FluxPipelineOutput` or `tuple`

`~pipelines.flux.FluxPipelineOutput` if `return_dict`
is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the generated
images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch

>>> from diffusers import FluxImg2ImgPipeline
>>> from diffusers.utils import load_image

>>> device = "cuda"
>>> pipe = FluxImg2ImgPipeline.from_pretrained("black-forest-labs/FLUX.1-schnell", torch_dtype=torch.bfloat16)
>>> pipe = pipe.to(device)

>>> url = "https://raw.githubusercontent.com/CompVis/stable-diffusion/main/assets/stable-samples/img2img/sketch-mountains-input.jpg"
>>> init_image = load_image(url).resize((1024, 1024))

>>> prompt = "cat wizard, gandalf, lord of the rings, detailed, fantasy, cute, adorable, Pixar, Disney, 8k"

>>> images = pipe(
...     prompt=prompt, image=init_image, num_inference_steps=4, strength=0.95, guidance_scale=0.0
... ).images[0]
```

#### encode_prompt[[diffusers.FluxImg2ImgPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], prompt_2: str | list[str] | None = None, device: typing.Optional[torch.device] = None, num_images_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, max_sequence_length: int = 512, lora_scale: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_img2img.py#L333)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is used in all text-encoders

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

## FluxInpaintPipeline[[diffusers.FluxInpaintPipeline]]

#### diffusers.FluxInpaintPipeline[[diffusers.FluxInpaintPipeline]]

```python
diffusers.FluxInpaintPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKL, text_encoder: CLIPTextModel, tokenizer: CLIPTokenizer, text_encoder_2: T5EncoderModel, tokenizer_2: T5Tokenizer, transformer: FluxTransformer2DModel, image_encoder: CLIPVisionModelWithProjection = None, feature_extractor: CLIPImageProcessorPil = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_inpaint.py#L166)

**Parameters:**

transformer ([FluxTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/flux_transformer#diffusers.FluxTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

text_encoder_2 (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer_2 (`T5TokenizerFast`) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

The Flux pipeline for image inpainting.

Reference: https://blackforestlabs.ai/announcing-black-forest-labs/

#### __call__[[diffusers.FluxInpaintPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, prompt_2: str | list[str] | None = None, negative_prompt: str | list[str] = None, negative_prompt_2: str | list[str] | None = None, true_cfg_scale: float = 1.0, image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, mask_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, masked_image_latents: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, height: int | None = None, width: int | None = None, padding_mask_crop: int | None = None, strength: float = 0.6, num_inference_steps: int = 28, sigmas: list[float] | None = None, guidance_scale: float = 7.0, num_images_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.FloatTensor] = None, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, ip_adapter_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, ip_adapter_image_embeds: list[torch.Tensor] | None = None, negative_ip_adapter_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, negative_ip_adapter_image_embeds: list[torch.Tensor] | None = None, negative_prompt_embeds: typing.Optional[torch.FloatTensor] = None, negative_pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, output_type: str | None = 'pil', return_dict: bool = True, joint_attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_inpaint.py#L775)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is will be used instead

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `true_cfg_scale` is not greater than `1`).

negative_prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `negative_prompt` is used in all the text-encoders.

true_cfg_scale (`float`, *optional*, defaults to 1.0) : True classifier-free guidance (guidance scale) is enabled when `true_cfg_scale` > 1 and `negative_prompt` is provided.

image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) : `Image`, numpy array or tensor representing an image batch to be used as the starting point. For both numpy array and pytorch tensor, the expected value range is between `[0, 1]` If it's a tensor or a list or tensors, the expected shape should be `(B, C, H, W)` or `(C, H, W)`. If it is a numpy array or a list of arrays, the expected shape should be `(B, H, W, C)` or `(H, W, C)` It can also accept image latents as `image`, but if passing latents directly it is not encoded again.

mask_image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) : `Image`, numpy array or tensor representing an image batch to mask `image`. White pixels in the mask are repainted while black pixels are preserved. If `mask_image` is a PIL image, it is converted to a single channel (luminance) before use. If it's a numpy array or pytorch tensor, it should contain one color channel (L) instead of 3, so the expected shape for pytorch tensor would be `(B, 1, H, W)`, `(B, H, W)`, `(1, H, W)`, `(H, W)`. And for numpy array would be for `(B, H, W, 1)`, `(B, H, W)`, `(H, W, 1)`, or `(H, W)`.

masked_image_latents (`torch.Tensor`, `list[torch.Tensor]`) : `Tensor` representing an image batch to mask `image` generated by VAE. If not provided, the mask latents tensor will be generated by `mask_image`.

height (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The height in pixels of the generated image. This is set to 1024 by default for the best results.

width (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The width in pixels of the generated image. This is set to 1024 by default for the best results.

padding_mask_crop (`int`, *optional*, defaults to `None`) : The size of margin in the crop to be applied to the image and masking. If `None`, no crop is applied to image and mask_image. If `padding_mask_crop` is not `None`, it will first find a rectangular region with the same aspect ration of the image and contains all masked area, and then expand that area based on `padding_mask_crop`. The image and mask_image will then be cropped based on the expanded area before resizing to the original image size for inpainting. This is useful when the masked area is small while the image is large and contain information irrelevant for inpainting, such as background.

strength (`float`, *optional*, defaults to 1.0) : Indicates extent to transform the reference `image`. Must be between 0 and 1. `image` is used as a starting point and more noise is added the higher the `strength`. The number of denoising steps depends on the amount of noise initially added. When `strength` is 1, added noise is maximum and the denoising process runs for the full number of iterations specified in `num_inference_steps`. A value of 1 essentially ignores `image`.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`list[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

guidance_scale (`float`, *optional*, defaults to 7.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.FloatTensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

negative_pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled negative_prompt_embeds will be generated from `negative_prompt` input argument.

ip_adapter_image : (`PipelineImageInput`, *optional*): Optional image input to work with IP Adapters.

ip_adapter_image_embeds (`list[torch.Tensor]`, *optional*) : Pre-generated image embeddings for IP-Adapter. It should be a list of length same as number of IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. If not provided, embeddings are computed from the `ip_adapter_image` input argument.

negative_ip_adapter_image : (`PipelineImageInput`, *optional*): Optional image input to work with IP Adapters.

negative_ip_adapter_image_embeds (`list[torch.Tensor]`, *optional*) : Pre-generated image embeddings for IP-Adapter. It should be a list of length same as number of IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. If not provided, embeddings are computed from the `ip_adapter_image` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.flux.FluxPipelineOutput` instead of a plain tuple.

joint_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int` defaults to 512) : Maximum sequence length to use with the `prompt`.

**Returns:** `~pipelines.flux.FluxPipelineOutput` or `tuple`

`~pipelines.flux.FluxPipelineOutput` if `return_dict`
is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the generated
images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import FluxInpaintPipeline
>>> from diffusers.utils import load_image

>>> pipe = FluxInpaintPipeline.from_pretrained("black-forest-labs/FLUX.1-schnell", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")
>>> prompt = "Face of a yellow cat, high resolution, sitting on a park bench"
>>> img_url = "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
>>> mask_url = "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"
>>> source = load_image(img_url)
>>> mask = load_image(mask_url)
>>> image = pipe(prompt=prompt, image=source, mask_image=mask).images[0]
>>> image.save("flux_inpainting.png")
```

#### encode_prompt[[diffusers.FluxInpaintPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], prompt_2: str | list[str] | None = None, device: typing.Optional[torch.device] = None, num_images_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, max_sequence_length: int = 512, lora_scale: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_inpaint.py#L337)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is used in all text-encoders

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

## FluxControlNetInpaintPipeline[[diffusers.FluxControlNetInpaintPipeline]]

#### diffusers.FluxControlNetInpaintPipeline[[diffusers.FluxControlNetInpaintPipeline]]

```python
diffusers.FluxControlNetInpaintPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKL, text_encoder: CLIPTextModel, tokenizer: CLIPTokenizer, text_encoder_2: T5EncoderModel, tokenizer_2: T5Tokenizer, transformer: FluxTransformer2DModel, controlnet: diffusers.models.controlnets.controlnet_flux.FluxControlNetModel | list[diffusers.models.controlnets.controlnet_flux.FluxControlNetModel] | tuple[diffusers.models.controlnets.controlnet_flux.FluxControlNetModel] | diffusers.models.controlnets.controlnet_flux.FluxMultiControlNetModel)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_controlnet_inpainting.py#L174)

**Parameters:**

transformer ([FluxTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/flux_transformer#diffusers.FluxTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

text_encoder_2 (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer_2 (`T5TokenizerFast`) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

The Flux controlnet pipeline for inpainting.

Reference: https://blackforestlabs.ai/announcing-black-forest-labs/

#### __call__[[diffusers.FluxControlNetInpaintPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, prompt_2: str | list[str] | None = None, image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, mask_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, masked_image_latents: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, control_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, height: int | None = None, width: int | None = None, strength: float = 0.6, padding_mask_crop: int | None = None, sigmas: list[float] | None = None, num_inference_steps: int = 28, guidance_scale: float = 7.0, control_guidance_start: float | list[float] = 0.0, control_guidance_end: float | list[float] = 1.0, control_mode: int | list[int] | None = None, controlnet_conditioning_scale: float | list[float] = 1.0, num_images_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.FloatTensor] = None, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, output_type: str | None = 'pil', return_dict: bool = True, joint_attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_controlnet_inpainting.py#L739)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation.

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`.

image (`PIL.Image.Image` or `list[PIL.Image.Image]` or `torch.FloatTensor`) : The image(s) to inpaint.

mask_image (`PIL.Image.Image` or `list[PIL.Image.Image]` or `torch.FloatTensor`) : The mask image(s) to use for inpainting. White pixels in the mask will be repainted, while black pixels will be preserved.

masked_image_latents (`torch.FloatTensor`, *optional*) : Pre-generated masked image latents.

control_image (`PIL.Image.Image` or `list[PIL.Image.Image]` or `torch.FloatTensor`) : The ControlNet input condition. Image to control the generation.

height (`int`, *optional*, defaults to self.default_sample_size * self.vae_scale_factor) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to self.default_sample_size * self.vae_scale_factor) : The width in pixels of the generated image.

strength (`float`, *optional*, defaults to 0.6) : Conceptually, indicates how much to inpaint the masked area. Must be between 0 and 1.

padding_mask_crop (`int`, *optional*) : The size of the padding to use when cropping the mask.

num_inference_steps (`int`, *optional*, defaults to 28) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`list[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

guidance_scale (`float`, *optional*, defaults to 7.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598).

control_guidance_start (`float` or `list[float]`, *optional*, defaults to 0.0) : The percentage of total steps at which the ControlNet starts applying.

control_guidance_end (`float` or `list[float]`, *optional*, defaults to 1.0) : The percentage of total steps at which the ControlNet stops applying.

control_mode (`int` or `list[int]`, *optional*) : The mode for the ControlNet. If multiple ControlNets are used, this should be a list.

controlnet_conditioning_scale (`float` or `list[float]`, *optional*, defaults to 1.0) : The outputs of the ControlNet are multiplied by `controlnet_conditioning_scale` before they are added to the residual in the original transformer.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or more [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.FloatTensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts.

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.flux.FluxPipelineOutput` instead of a plain tuple.

joint_attention_kwargs (`dict`, *optional*) : Additional keyword arguments to be passed to the joint attention mechanism.

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising step during the inference.

callback_on_step_end_tensor_inputs (`list[str]`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function.

max_sequence_length (`int`, *optional*, defaults to 512) : The maximum length of the sequence to be generated.

**Returns:** `~pipelines.flux.FluxPipelineOutput` or `tuple`

`~pipelines.flux.FluxPipelineOutput` if `return_dict`
is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the generated
images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import FluxControlNetInpaintPipeline
>>> from diffusers.models import FluxControlNetModel
>>> from diffusers.utils import load_image

>>> controlnet = FluxControlNetModel.from_pretrained(
...     "InstantX/FLUX.1-dev-controlnet-canny", torch_dtype=torch.float16
... )
>>> pipe = FluxControlNetInpaintPipeline.from_pretrained(
...     "black-forest-labs/FLUX.1-schnell", controlnet=controlnet, torch_dtype=torch.float16
... )
>>> pipe.to("cuda")

>>> control_image = load_image(
...     "https://huggingface.co/InstantX/FLUX.1-dev-Controlnet-Canny-alpha/resolve/main/canny.jpg"
... )
>>> init_image = load_image(
...     "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
... )
>>> mask_image = load_image(
...     "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"
... )

>>> prompt = "A girl holding a sign that says InstantX"
>>> image = pipe(
...     prompt,
...     image=init_image,
...     mask_image=mask_image,
...     control_image=control_image,
...     control_guidance_start=0.2,
...     control_guidance_end=0.8,
...     controlnet_conditioning_scale=0.7,
...     strength=0.7,
...     num_inference_steps=28,
...     guidance_scale=3.5,
... ).images[0]
>>> image.save("flux_controlnet_inpaint.png")
```

#### encode_prompt[[diffusers.FluxControlNetInpaintPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], prompt_2: str | list[str] | None = None, device: typing.Optional[torch.device] = None, num_images_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, max_sequence_length: int = 512, lora_scale: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_controlnet_inpainting.py#L347)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is used in all text-encoders

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

## FluxControlNetImg2ImgPipeline[[diffusers.FluxControlNetImg2ImgPipeline]]

#### diffusers.FluxControlNetImg2ImgPipeline[[diffusers.FluxControlNetImg2ImgPipeline]]

```python
diffusers.FluxControlNetImg2ImgPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKL, text_encoder: CLIPTextModel, tokenizer: CLIPTokenizer, text_encoder_2: T5EncoderModel, tokenizer_2: T5Tokenizer, transformer: FluxTransformer2DModel, controlnet: diffusers.models.controlnets.controlnet_flux.FluxControlNetModel | list[diffusers.models.controlnets.controlnet_flux.FluxControlNetModel] | tuple[diffusers.models.controlnets.controlnet_flux.FluxControlNetModel] | diffusers.models.controlnets.controlnet_flux.FluxMultiControlNetModel)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_controlnet_image_to_image.py#L172)

**Parameters:**

transformer ([FluxTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/flux_transformer#diffusers.FluxTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

text_encoder_2 (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer_2 (`T5TokenizerFast`) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

The Flux controlnet pipeline for image-to-image generation.

Reference: https://blackforestlabs.ai/announcing-black-forest-labs/

#### __call__[[diffusers.FluxControlNetImg2ImgPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, prompt_2: str | list[str] | None = None, image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, control_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, height: int | None = None, width: int | None = None, strength: float = 0.6, num_inference_steps: int = 28, sigmas: list[float] | None = None, guidance_scale: float = 7.0, control_guidance_start: float | list[float] = 0.0, control_guidance_end: float | list[float] = 1.0, control_mode: int | list[int] | None = None, controlnet_conditioning_scale: float | list[float] = 1.0, num_images_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.FloatTensor] = None, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, output_type: str | None = 'pil', return_dict: bool = True, joint_attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_controlnet_image_to_image.py#L635)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation.

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`.

image (`PIL.Image.Image` or `list[PIL.Image.Image]` or `torch.FloatTensor`) : The image(s) to modify with the pipeline.

control_image (`PIL.Image.Image` or `list[PIL.Image.Image]` or `torch.FloatTensor`) : The ControlNet input condition. Image to control the generation.

height (`int`, *optional*, defaults to self.default_sample_size * self.vae_scale_factor) : The height in pixels of the generated image.

width (`int`, *optional*, defaults to self.default_sample_size * self.vae_scale_factor) : The width in pixels of the generated image.

strength (`float`, *optional*, defaults to 0.6) : Conceptually, indicates how much to transform the reference `image`. Must be between 0 and 1.

num_inference_steps (`int`, *optional*, defaults to 28) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`list[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

guidance_scale (`float`, *optional*, defaults to 7.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598).

control_mode (`int` or `list[int]`, *optional*) : The mode for the ControlNet. If multiple ControlNets are used, this should be a list.

controlnet_conditioning_scale (`float` or `list[float]`, *optional*, defaults to 1.0) : The outputs of the ControlNet are multiplied by `controlnet_conditioning_scale` before they are added to the residual in the original transformer.

control_guidance_start (`float` or `list[float]`, *optional*, defaults to 0.0) : The percentage of total steps at which the ControlNet starts applying.

control_guidance_end (`float` or `list[float]`, *optional*, defaults to 1.0) : The percentage of total steps at which the ControlNet stops applying.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or more [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.FloatTensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts.

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.flux.FluxPipelineOutput` instead of a plain tuple.

joint_attention_kwargs (`dict`, *optional*) : Additional keyword arguments to be passed to the joint attention mechanism.

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising step during the inference.

callback_on_step_end_tensor_inputs (`list[str]`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function.

max_sequence_length (`int`, *optional*, defaults to 512) : The maximum length of the sequence to be generated.

**Returns:** `~pipelines.flux.FluxPipelineOutput` or `tuple`

`~pipelines.flux.FluxPipelineOutput` if `return_dict`
is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the generated
images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import FluxControlNetImg2ImgPipeline, FluxControlNetModel
>>> from diffusers.utils import load_image

>>> device = "cuda" if torch.cuda.is_available() else "cpu"

>>> controlnet = FluxControlNetModel.from_pretrained(
...     "InstantX/FLUX.1-dev-Controlnet-Canny-alpha", torch_dtype=torch.bfloat16
... )

>>> pipe = FluxControlNetImg2ImgPipeline.from_pretrained(
...     "black-forest-labs/FLUX.1-schnell", controlnet=controlnet, torch_dtype=torch.float16
... )

>>> pipe.text_encoder.to(torch.float16)
>>> pipe.controlnet.to(torch.float16)
>>> pipe.to("cuda")

>>> control_image = load_image("https://huggingface.co/InstantX/SD3-Controlnet-Canny/resolve/main/canny.jpg")
>>> init_image = load_image(
...     "https://raw.githubusercontent.com/CompVis/stable-diffusion/main/assets/stable-samples/img2img/sketch-mountains-input.jpg"
... )

>>> prompt = "A girl in city, 25 years old, cool, futuristic"
>>> image = pipe(
...     prompt,
...     image=init_image,
...     control_image=control_image,
...     control_guidance_start=0.2,
...     control_guidance_end=0.8,
...     controlnet_conditioning_scale=1.0,
...     strength=0.7,
...     num_inference_steps=2,
...     guidance_scale=3.5,
... ).images[0]
>>> image.save("flux_controlnet_img2img.png")
```

#### encode_prompt[[diffusers.FluxControlNetImg2ImgPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], prompt_2: str | list[str] | None = None, device: typing.Optional[torch.device] = None, num_images_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, max_sequence_length: int = 512, lora_scale: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_controlnet_image_to_image.py#L336)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is used in all text-encoders

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

## FluxControlPipeline[[diffusers.FluxControlPipeline]]

#### diffusers.FluxControlPipeline[[diffusers.FluxControlPipeline]]

```python
diffusers.FluxControlPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKL, text_encoder: CLIPTextModel, tokenizer: CLIPTokenizer, text_encoder_2: T5EncoderModel, tokenizer_2: T5Tokenizer, transformer: FluxTransformer2DModel)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_control.py#L159)

**Parameters:**

transformer ([FluxTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/flux_transformer#diffusers.FluxTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

text_encoder_2 (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer_2 (`T5TokenizerFast`) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

The Flux pipeline for controllable text-to-image generation with image conditions.

Reference: https://bfl.ai/flux-1-tools

#### __call__[[diffusers.FluxControlPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, prompt_2: str | list[str] | None = None, control_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, height: int | None = None, width: int | None = None, num_inference_steps: int = 28, sigmas: list[float] | None = None, guidance_scale: float = 3.5, num_images_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.FloatTensor] = None, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, output_type: str | None = 'pil', return_dict: bool = True, joint_attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_control.py#L581)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is will be used instead

control_image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, `list[np.ndarray]`, : `list[list[torch.Tensor]]`, `list[list[np.ndarray]]` or `list[list[PIL.Image.Image]]`): The ControlNet input condition to provide guidance to the `unet` for generation. If the type is specified as `torch.Tensor`, it is passed to ControlNet as is. `PIL.Image.Image` can also be accepted as an image. The dimensions of the output image defaults to `image`'s dimensions. If height and/or width are passed, `image` is resized accordingly. If multiple ControlNets are specified in `init`, images must be passed as a list such that each element of the list can be correctly batched for input to a single ControlNet.

height (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The height in pixels of the generated image. This is set to 1024 by default for the best results.

width (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The width in pixels of the generated image. This is set to 1024 by default for the best results.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`list[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

guidance_scale (`float`, *optional*, defaults to 3.5) : Embedded guidance scale is enabled by setting `guidance_scale` > 1. Higher `guidance_scale` encourages a model to generate images more aligned with prompt at the expense of lower image quality.  Guidance-distilled models approximates true classifier-free guidance for `guidance_scale` > 1. Refer to the [paper](https://huggingface.co/papers/2210.03142) to learn more.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.FloatTensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.flux.FluxPipelineOutput` instead of a plain tuple.

joint_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int` defaults to 512) : Maximum sequence length to use with the `prompt`.

**Returns:** `~pipelines.flux.FluxPipelineOutput` or `tuple`

`~pipelines.flux.FluxPipelineOutput` if `return_dict`
is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the generated
images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from controlnet_aux import CannyDetector
>>> from diffusers import FluxControlPipeline
>>> from diffusers.utils import load_image

>>> pipe = FluxControlPipeline.from_pretrained(
...     "black-forest-labs/FLUX.1-Canny-dev", torch_dtype=torch.bfloat16
... ).to("cuda")

>>> prompt = "A robot made of exotic candies and chocolates of different kinds. The background is filled with confetti and celebratory gifts."
>>> control_image = load_image(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/robot.png"
... )

>>> processor = CannyDetector()
>>> control_image = processor(
...     control_image, low_threshold=50, high_threshold=200, detect_resolution=1024, image_resolution=1024
... )

>>> image = pipe(
...     prompt=prompt,
...     control_image=control_image,
...     height=1024,
...     width=1024,
...     num_inference_steps=50,
...     guidance_scale=30.0,
... ).images[0]
>>> image.save("output.png")
```

#### encode_prompt[[diffusers.FluxControlPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], prompt_2: str | list[str] | None = None, device: typing.Optional[torch.device] = None, num_images_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, max_sequence_length: int = 512, lora_scale: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_control.py#L324)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is used in all text-encoders

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

## FluxControlImg2ImgPipeline[[diffusers.FluxControlImg2ImgPipeline]]

#### diffusers.FluxControlImg2ImgPipeline[[diffusers.FluxControlImg2ImgPipeline]]

```python
diffusers.FluxControlImg2ImgPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKL, text_encoder: CLIPTextModel, tokenizer: CLIPTokenizer, text_encoder_2: T5EncoderModel, tokenizer_2: T5Tokenizer, transformer: FluxTransformer2DModel)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_control_img2img.py#L178)

**Parameters:**

transformer ([FluxTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/flux_transformer#diffusers.FluxTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

text_encoder_2 (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer_2 (`T5TokenizerFast`) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

The Flux pipeline for image inpainting.

Reference: https://blackforestlabs.ai/announcing-black-forest-labs/

#### __call__[[diffusers.FluxControlImg2ImgPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, prompt_2: str | list[str] | None = None, image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, control_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, height: int | None = None, width: int | None = None, strength: float = 0.6, num_inference_steps: int = 28, sigmas: list[float] | None = None, guidance_scale: float = 7.0, num_images_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.FloatTensor] = None, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, output_type: str | None = 'pil', return_dict: bool = True, joint_attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_control_img2img.py#L634)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is will be used instead

image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) : `Image`, numpy array or tensor representing an image batch to be used as the starting point. For both numpy array and pytorch tensor, the expected value range is between `[0, 1]` If it's a tensor or a list or tensors, the expected shape should be `(B, C, H, W)` or `(C, H, W)`. If it is a numpy array or a list of arrays, the expected shape should be `(B, H, W, C)` or `(H, W, C)` It can also accept image latents as `image`, but if passing latents directly it is not encoded again.

control_image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, `list[np.ndarray]`, : `list[list[torch.Tensor]]`, `list[list[np.ndarray]]` or `list[list[PIL.Image.Image]]`): The ControlNet input condition to provide guidance to the `unet` for generation. If the type is specified as `torch.Tensor`, it is passed to ControlNet as is. `PIL.Image.Image` can also be accepted as an image. The dimensions of the output image defaults to `image`'s dimensions. If height and/or width are passed, `image` is resized accordingly. If multiple ControlNets are specified in `init`, images must be passed as a list such that each element of the list can be correctly batched for input to a single ControlNet.

height (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The height in pixels of the generated image. This is set to 1024 by default for the best results.

width (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The width in pixels of the generated image. This is set to 1024 by default for the best results.

strength (`float`, *optional*, defaults to 1.0) : Indicates extent to transform the reference `image`. Must be between 0 and 1. `image` is used as a starting point and more noise is added the higher the `strength`. The number of denoising steps depends on the amount of noise initially added. When `strength` is 1, added noise is maximum and the denoising process runs for the full number of iterations specified in `num_inference_steps`. A value of 1 essentially ignores `image`.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`list[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

guidance_scale (`float`, *optional*, defaults to 7.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.FloatTensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.flux.FluxPipelineOutput` instead of a plain tuple.

joint_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int` defaults to 512) : Maximum sequence length to use with the `prompt`.

**Returns:** `~pipelines.flux.FluxPipelineOutput` or `tuple`

`~pipelines.flux.FluxPipelineOutput` if `return_dict`
is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the generated
images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from controlnet_aux import CannyDetector
>>> from diffusers import FluxControlImg2ImgPipeline
>>> from diffusers.utils import load_image

>>> pipe = FluxControlImg2ImgPipeline.from_pretrained(
...     "black-forest-labs/FLUX.1-Canny-dev", torch_dtype=torch.bfloat16
... ).to("cuda")

>>> prompt = "A robot made of exotic candies and chocolates of different kinds. Abstract background"
>>> image = load_image(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/watercolor-painting.jpg"
... )
>>> control_image = load_image(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/robot.png"
... )

>>> processor = CannyDetector()
>>> control_image = processor(
...     control_image, low_threshold=50, high_threshold=200, detect_resolution=1024, image_resolution=1024
... )

>>> image = pipe(
...     prompt=prompt,
...     image=image,
...     control_image=control_image,
...     strength=0.8,
...     height=1024,
...     width=1024,
...     num_inference_steps=50,
...     guidance_scale=30.0,
... ).images[0]
>>> image.save("output.png")
```

#### encode_prompt[[diffusers.FluxControlImg2ImgPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], prompt_2: str | list[str] | None = None, device: typing.Optional[torch.device] = None, num_images_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, max_sequence_length: int = 512, lora_scale: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_control_img2img.py#L335)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is used in all text-encoders

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

## FluxPriorReduxPipeline[[diffusers.FluxPriorReduxPipeline]]

#### diffusers.FluxPriorReduxPipeline[[diffusers.FluxPriorReduxPipeline]]

```python
diffusers.FluxPriorReduxPipeline(image_encoder: SiglipVisionModel, feature_extractor: SiglipImageProcessorPil, image_embedder: ReduxImageEncoder, text_encoder: CLIPTextModel = None, tokenizer: CLIPTokenizer = None, text_encoder_2: T5EncoderModel = None, tokenizer_2: T5Tokenizer = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_prior_redux.py#L82)

**Parameters:**

image_encoder (`SiglipVisionModel`) : SIGLIP vision model to encode the input image.

feature_extractor (`SiglipImageProcessor`) : Image processor for preprocessing images for the SIGLIP model.

image_embedder (`ReduxImageEncoder`) : Redux image encoder to process the SIGLIP embeddings.

text_encoder (`CLIPTextModel`, *optional*) : [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

text_encoder_2 (`T5EncoderModel`, *optional*) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`CLIPTokenizer`, *optional*) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer_2 (`T5TokenizerFast`, *optional*) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

The Flux Redux pipeline for image-to-image generation.

Reference: https://blackforestlabs.ai/flux-1-tools/

#### __call__[[diffusers.FluxPriorReduxPipeline.__call__]]

```python
__call__(image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]], prompt: str | list[str] = None, prompt_2: str | list[str] | None = None, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, prompt_embeds_scale: float | list[float] | None = 1.0, pooled_prompt_embeds_scale: float | list[float] | None = 1.0, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_prior_redux.py#L375)

**Parameters:**

image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) : `Image`, numpy array or tensor representing an image batch to be used as the starting point. For both numpy array and pytorch tensor, the expected value range is between `[0, 1]` If it's a tensor or a list or tensors, the expected shape should be `(B, C, H, W)` or `(C, H, W)`. If it is a numpy array or a list of arrays, the expected shape should be `(B, H, W, C)` or `(H, W, C)`

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. **experimental feature**: to use this feature, make sure to explicitly load text encoders to the pipeline. Prompts will be ignored if text encoders are not loaded.

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`.

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings.

prompt_embeds_scale (`float` or `list[float]`, *optional*, defaults to 1.0) : Scale factor (or per-image list of scale factors) applied to the redux prompt embeddings before they are returned.

pooled_prompt_embeds_scale (`float` or `list[float]`, *optional*, defaults to 1.0) : Scale factor (or per-image list of scale factors) applied to the redux pooled prompt embeddings before they are returned.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.flux.FluxPriorReduxPipelineOutput` instead of a plain tuple.

**Returns:** `~pipelines.flux.FluxPriorReduxPipelineOutput` or `tuple`

`~pipelines.flux.FluxPriorReduxPipelineOutput` if `return_dict` is True, otherwise a `tuple`. When
returning a tuple, the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import FluxPriorReduxPipeline, FluxPipeline
>>> from diffusers.utils import load_image

>>> device = "cuda"
>>> dtype = torch.bfloat16

>>> repo_redux = "black-forest-labs/FLUX.1-Redux-dev"
>>> repo_base = "black-forest-labs/FLUX.1-dev"
>>> pipe_prior_redux = FluxPriorReduxPipeline.from_pretrained(repo_redux, torch_dtype=dtype).to(device)
>>> pipe = FluxPipeline.from_pretrained(
...     repo_base, text_encoder=None, text_encoder_2=None, torch_dtype=torch.bfloat16
... ).to(device)

>>> image = load_image(
...     "https://huggingface.co/datasets/YiYiXu/testing-images/resolve/main/style_ziggy/img5.png"
... )
>>> pipe_prior_output = pipe_prior_redux(image)
>>> images = pipe(
...     guidance_scale=2.5,
...     num_inference_steps=50,
...     generator=torch.Generator("cpu").manual_seed(0),
...     **pipe_prior_output,
... ).images
>>> images[0].save("flux-redux.png")
```

#### encode_prompt[[diffusers.FluxPriorReduxPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], prompt_2: str | list[str] | None = None, device: typing.Optional[torch.device] = None, num_images_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, max_sequence_length: int = 512, lora_scale: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_prior_redux.py#L296)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is used in all text-encoders

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

## FluxFillPipeline[[diffusers.FluxFillPipeline]]

#### diffusers.FluxFillPipeline[[diffusers.FluxFillPipeline]]

```python
diffusers.FluxFillPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKL, text_encoder: CLIPTextModel, tokenizer: CLIPTokenizer, text_encoder_2: T5EncoderModel, tokenizer_2: T5Tokenizer, transformer: FluxTransformer2DModel)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_fill.py#L167)

**Parameters:**

transformer ([FluxTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/flux_transformer#diffusers.FluxTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

text_encoder_2 (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer_2 (`T5TokenizerFast`) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

The Flux Fill pipeline for image inpainting/outpainting.

Reference: https://blackforestlabs.ai/flux-1-tools/

#### __call__[[diffusers.FluxFillPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, prompt_2: str | list[str] | None = None, image: typing.Optional[torch.FloatTensor] = None, mask_image: typing.Optional[torch.FloatTensor] = None, masked_image_latents: typing.Optional[torch.FloatTensor] = None, height: int | None = None, width: int | None = None, strength: float = 1.0, num_inference_steps: int = 50, sigmas: list[float] | None = None, guidance_scale: float = 30.0, num_images_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.FloatTensor] = None, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, output_type: str | None = 'pil', return_dict: bool = True, joint_attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_fill.py#L698)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is will be used instead

image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) : `Image`, numpy array or tensor representing an image batch to be used as the starting point. For both numpy array and pytorch tensor, the expected value range is between `[0, 1]` If it's a tensor or a list or tensors, the expected shape should be `(B, C, H, W)` or `(C, H, W)`. If it is a numpy array or a list of arrays, the expected shape should be `(B, H, W, C)` or `(H, W, C)`.

mask_image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) : `Image`, numpy array or tensor representing an image batch to mask `image`. White pixels in the mask are repainted while black pixels are preserved. If `mask_image` is a PIL image, it is converted to a single channel (luminance) before use. If it's a numpy array or pytorch tensor, it should contain one color channel (L) instead of 3, so the expected shape for pytorch tensor would be `(B, 1, H, W)`, `(B, H, W)`, `(1, H, W)`, `(H, W)`. And for numpy array would be for `(B, H, W, 1)`, `(B, H, W)`, `(H, W, 1)`, or `(H, W)`.

masked_image_latents (`torch.Tensor`, `list[torch.Tensor]`) : `Tensor` representing an image batch to mask `image` generated by VAE. If not provided, the mask latents tensor will be generated by `mask_image`.

height (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The height in pixels of the generated image. This is set to 1024 by default for the best results.

width (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The width in pixels of the generated image. This is set to 1024 by default for the best results.

strength (`float`, *optional*, defaults to 1.0) : Indicates extent to transform the reference `image`. Must be between 0 and 1. `image` is used as a starting point and more noise is added the higher the `strength`. The number of denoising steps depends on the amount of noise initially added. When `strength` is 1, added noise is maximum and the denoising process runs for the full number of iterations specified in `num_inference_steps`. A value of 1 essentially ignores `image`.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`list[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

guidance_scale (`float`, *optional*, defaults to 30.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). `guidance_scale` is defined as `w` of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.FloatTensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.flux.FluxPipelineOutput` instead of a plain tuple.

joint_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int` defaults to 512) : Maximum sequence length to use with the `prompt`.

**Returns:** `~pipelines.flux.FluxPipelineOutput` or `tuple`

`~pipelines.flux.FluxPipelineOutput` if `return_dict`
is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the generated
images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import FluxFillPipeline
>>> from diffusers.utils import load_image

>>> image = load_image("https://huggingface.co/datasets/YiYiXu/testing-images/resolve/main/cup.png")
>>> mask = load_image("https://huggingface.co/datasets/YiYiXu/testing-images/resolve/main/cup_mask.png")

>>> pipe = FluxFillPipeline.from_pretrained("black-forest-labs/FLUX.1-Fill-dev", torch_dtype=torch.bfloat16)
>>> pipe.enable_model_cpu_offload()  # save some VRAM by offloading the model to CPU

>>> image = pipe(
...     prompt="a white paper cup",
...     image=image,
...     mask_image=mask,
...     height=1632,
...     width=1232,
...     guidance_scale=30,
...     num_inference_steps=50,
...     max_sequence_length=512,
...     generator=torch.Generator("cpu").manual_seed(0),
... ).images[0]
>>> image.save("flux_fill.png")
```

#### encode_prompt[[diffusers.FluxFillPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], prompt_2: str | list[str] | None = None, device: typing.Optional[torch.device] = None, num_images_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, max_sequence_length: int = 512, lora_scale: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_fill.py#L419)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is used in all text-encoders

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

## FluxKontextPipeline[[diffusers.FluxKontextPipeline]]

#### diffusers.FluxKontextPipeline[[diffusers.FluxKontextPipeline]]

```python
diffusers.FluxKontextPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKL, text_encoder: CLIPTextModel, tokenizer: CLIPTokenizer, text_encoder_2: T5EncoderModel, tokenizer_2: T5Tokenizer, transformer: FluxTransformer2DModel, image_encoder: CLIPVisionModelWithProjection = None, feature_extractor: CLIPImageProcessorPil = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_kontext.py#L190)

**Parameters:**

transformer ([FluxTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/flux_transformer#diffusers.FluxTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

text_encoder_2 (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer_2 (`T5TokenizerFast`) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

The Flux Kontext pipeline for image-to-image and text-to-image generation.

Reference: https://bfl.ai/announcements/flux-1-kontext-dev

#### __call__[[diffusers.FluxKontextPipeline.__call__]]

```python
__call__(image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, prompt: str | list[str] = None, prompt_2: str | list[str] | None = None, negative_prompt: str | list[str] = None, negative_prompt_2: str | list[str] | None = None, true_cfg_scale: float = 1.0, height: int | None = None, width: int | None = None, num_inference_steps: int = 28, sigmas: list[float] | None = None, guidance_scale: float = 3.5, num_images_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.FloatTensor] = None, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, ip_adapter_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, ip_adapter_image_embeds: list[torch.Tensor] | None = None, negative_ip_adapter_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, negative_ip_adapter_image_embeds: list[torch.Tensor] | None = None, negative_prompt_embeds: typing.Optional[torch.FloatTensor] = None, negative_pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, output_type: str | None = 'pil', return_dict: bool = True, joint_attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512, max_area: int = 1048576, _auto_resize: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_kontext.py#L693)

**Parameters:**

image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) : `Image`, numpy array or tensor representing an image batch to be used as the starting point. For both numpy array and pytorch tensor, the expected value range is between `[0, 1]` If it's a tensor or a list or tensors, the expected shape should be `(B, C, H, W)` or `(C, H, W)`. If it is a numpy array or a list of arrays, the expected shape should be `(B, H, W, C)` or `(H, W, C)` It can also accept image latents as `image`, but if passing latents directly it is not encoded again.

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is will be used instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `true_cfg_scale` is not greater than `1`).

negative_prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `negative_prompt` is used in all the text-encoders.

true_cfg_scale (`float`, *optional*, defaults to 1.0) : When > 1.0 and a provided `negative_prompt`, enables true classifier-free guidance.

height (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The height in pixels of the generated image. This is set to 1024 by default for the best results.

width (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The width in pixels of the generated image. This is set to 1024 by default for the best results.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`list[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

guidance_scale (`float`, *optional*, defaults to 3.5) : Embedded guidance scale is enabled by setting `guidance_scale` > 1. Higher `guidance_scale` encourages a model to generate images more aligned with prompt at the expense of lower image quality.  Guidance-distilled models approximates true classifier-free guidance for `guidance_scale` > 1. Refer to the [paper](https://huggingface.co/papers/2210.03142) to learn more.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.FloatTensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

ip_adapter_image : (`PipelineImageInput`, *optional*): Optional image input to work with IP Adapters.

ip_adapter_image_embeds (`list[torch.Tensor]`, *optional*) : Pre-generated image embeddings for IP-Adapter. It should be a list of length same as number of IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. If not provided, embeddings are computed from the `ip_adapter_image` input argument.

negative_ip_adapter_image : (`PipelineImageInput`, *optional*): Optional image input to work with IP Adapters.

negative_ip_adapter_image_embeds (`list[torch.Tensor]`, *optional*) : Pre-generated image embeddings for IP-Adapter. It should be a list of length same as number of IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. If not provided, embeddings are computed from the `ip_adapter_image` input argument.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

negative_pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled negative_prompt_embeds will be generated from `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.flux.FluxPipelineOutput` instead of a plain tuple.

joint_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int` defaults to 512) : Maximum sequence length to use with the `prompt`.

max_area (`int`, defaults to `1024 ** 2`) : The maximum area of the generated image in pixels. The height and width will be adjusted to fit this area while maintaining the aspect ratio.

_auto_resize (`bool`, *optional*, defaults to `True`) : Whether to automatically resize the input image to the preferred resolutions.

**Returns:** `~pipelines.flux.FluxPipelineOutput` or `tuple`

`~pipelines.flux.FluxPipelineOutput` if `return_dict`
is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the generated
images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import FluxKontextPipeline
>>> from diffusers.utils import load_image

>>> pipe = FluxKontextPipeline.from_pretrained(
...     "black-forest-labs/FLUX.1-Kontext-dev", torch_dtype=torch.bfloat16
... )
>>> pipe.to("cuda")

>>> image = load_image(
...     "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/yarn-art-pikachu.png"
... ).convert("RGB")
>>> prompt = "Make Pikachu hold a sign that says 'Black Forest Labs is awesome', yarn art style, detailed, vibrant colors"
>>> image = pipe(
...     image=image,
...     prompt=prompt,
...     guidance_scale=2.5,
...     generator=torch.Generator().manual_seed(42),
... ).images[0]
>>> image.save("output.png")
```

#### encode_prompt[[diffusers.FluxKontextPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], prompt_2: str | list[str] | None = None, device: typing.Optional[torch.device] = None, num_images_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, max_sequence_length: int = 512, lora_scale: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_kontext.py#L358)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is used in all text-encoders

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.

## FluxKontextInpaintPipeline[[diffusers.FluxKontextInpaintPipeline]]

#### diffusers.FluxKontextInpaintPipeline[[diffusers.FluxKontextInpaintPipeline]]

```python
diffusers.FluxKontextInpaintPipeline(scheduler: FlowMatchEulerDiscreteScheduler, vae: AutoencoderKL, text_encoder: CLIPTextModel, tokenizer: CLIPTokenizer, text_encoder_2: T5EncoderModel, tokenizer_2: T5Tokenizer, transformer: FluxTransformer2DModel, image_encoder: CLIPVisionModelWithProjection = None, feature_extractor: CLIPImageProcessorPil = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_kontext_inpaint.py#L214)

**Parameters:**

transformer ([FluxTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/flux_transformer#diffusers.FluxTransformer2DModel)) : Conditional Transformer (MMDiT) architecture to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations.

text_encoder (`CLIPTextModel`) : [CLIP](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPTextModel), specifically the [clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14) variant.

text_encoder_2 (`T5EncoderModel`) : [T5](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5EncoderModel), specifically the [google/t5-v1_1-xxl](https://huggingface.co/google/t5-v1_1-xxl) variant.

tokenizer (`CLIPTokenizer`) : Tokenizer of class [CLIPTokenizer](https://huggingface.co/docs/transformers/en/model_doc/clip#transformers.CLIPTokenizer).

tokenizer_2 (`T5TokenizerFast`) : Second Tokenizer of class [T5TokenizerFast](https://huggingface.co/docs/transformers/en/model_doc/t5#transformers.T5TokenizerFast).

The Flux Kontext pipeline for text-to-image generation.

Reference: https://blackforestlabs.ai/announcing-black-forest-labs/

#### __call__[[diffusers.FluxKontextInpaintPipeline.__call__]]

```python
__call__(image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, image_reference: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, mask_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]] = None, prompt: str | list[str] = None, prompt_2: str | list[str] | None = None, negative_prompt: str | list[str] = None, negative_prompt_2: str | list[str] | None = None, true_cfg_scale: float = 1.0, height: int | None = None, width: int | None = None, strength: float = 1.0, padding_mask_crop: int | None = None, num_inference_steps: int = 28, sigmas: list[float] | None = None, guidance_scale: float = 3.5, num_images_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.FloatTensor] = None, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, ip_adapter_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, ip_adapter_image_embeds: list[torch.Tensor] | None = None, negative_ip_adapter_image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], NoneType] = None, negative_ip_adapter_image_embeds: list[torch.Tensor] | None = None, negative_prompt_embeds: typing.Optional[torch.FloatTensor] = None, negative_pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, output_type: str | None = 'pil', return_dict: bool = True, joint_attention_kwargs: dict[str, typing.Any] | None = None, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], max_sequence_length: int = 512, max_area: int = 1048576, _auto_resize: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_kontext_inpaint.py#L882)

**Parameters:**

image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) : `Image`, numpy array or tensor representing an image batch to be be inpainted (which parts of the image to be masked out with `mask_image` and repainted according to `prompt` and `image_reference`). For both numpy array and pytorch tensor, the expected value range is between `[0, 1]` If it's a tensor or a list or tensors, the expected shape should be `(B, C, H, W)` or `(C, H, W)`. If it is a numpy array or a list of arrays, the expected shape should be `(B, H, W, C)` or `(H, W, C)` It can also accept image latents as `image`, but if passing latents directly it is not encoded again.

image_reference (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) : `Image`, numpy array or tensor representing an image batch to be used as the starting point for the masked area. For both numpy array and pytorch tensor, the expected value range is between `[0, 1]` If it's a tensor or a list or tensors, the expected shape should be `(B, C, H, W)` or `(C, H, W)` If it is a numpy array or a list of arrays, the expected shape should be `(B, H, W, C)` or `(H, W, C)` It can also accept image latents as `image`, but if passing latents directly it is not encoded again.

mask_image (`torch.Tensor`, `PIL.Image.Image`, `np.ndarray`, `list[torch.Tensor]`, `list[PIL.Image.Image]`, or `list[np.ndarray]`) : `Image`, numpy array or tensor representing an image batch to mask `image`. White pixels in the mask are repainted while black pixels are preserved. If `mask_image` is a PIL image, it is converted to a single channel (luminance) before use. If it's a numpy array or pytorch tensor, it should contain one color channel (L) instead of 3, so the expected shape for pytorch tensor would be `(B, 1, H, W)`, `(B, H, W)`, `(1, H, W)`, `(H, W)`. And for numpy array would be for `(B, H, W, 1)`, `(B, H, W)`, `(H, W, 1)`, or `(H, W)`.

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass `prompt_embeds`. instead.

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is will be used instead.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `true_cfg_scale` is not greater than `1`).

negative_prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the image generation to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `negative_prompt` is used in all the text-encoders.

true_cfg_scale (`float`, *optional*, defaults to 1.0) : True classifier-free guidance (guidance scale) is enabled when `true_cfg_scale` > 1 and `negative_prompt` is provided.

height (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The height in pixels of the generated image. This is set to 1024 by default for the best results.

width (`int`, *optional*, defaults to self.unet.config.sample_size * self.vae_scale_factor) : The width in pixels of the generated image. This is set to 1024 by default for the best results.

strength (`float`, *optional*, defaults to 1.0) : Indicates extent to transform the reference `image`. Must be between 0 and 1. `image` is used as a starting point and more noise is added the higher the `strength`. The number of denoising steps depends on the amount of noise initially added. When `strength` is 1, added noise is maximum and the denoising process runs for the full number of iterations specified in `num_inference_steps`. A value of 1 essentially ignores `image`.

padding_mask_crop (`int`, *optional*, defaults to `None`) : The size of margin in the crop to be applied to the image and masking. If `None`, no crop is applied to image and mask_image. If `padding_mask_crop` is not `None`, it will first find a rectangular region with the same aspect ration of the image and contains all masked area, and then expand that area based on `padding_mask_crop`. The image and mask_image will then be cropped based on the expanded area before resizing to the original image size for inpainting. This is useful when the masked area is small while the image is large and contain information irrelevant for inpainting, such as background.

num_inference_steps (`int`, *optional*, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

sigmas (`list[float]`, *optional*) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

guidance_scale (`float`, *optional*, defaults to 3.5) : Embedded guidance scale is enabled by setting `guidance_scale` > 1. Higher `guidance_scale` encourages a model to generate images more aligned with `prompt` at the expense of lower image quality.  Guidance-distilled models approximates true classifier-free guidance for `guidance_scale` > 1. Refer to the [paper](https://huggingface.co/papers/2210.03142) to learn more.

num_images_per_prompt (`int`, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.FloatTensor`, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

ip_adapter_image : (`PipelineImageInput`, *optional*): Optional image input to work with IP Adapters.

ip_adapter_image_embeds (`list[torch.Tensor]`, *optional*) : Pre-generated image embeddings for IP-Adapter. It should be a list of length same as number of IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. If not provided, embeddings are computed from the `ip_adapter_image` input argument.

negative_ip_adapter_image : (`PipelineImageInput`, *optional*): Optional image input to work with IP Adapters.

negative_ip_adapter_image_embeds (`list[torch.Tensor]`, *optional*) : Pre-generated image embeddings for IP-Adapter. It should be a list of length same as number of IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. If not provided, embeddings are computed from the `ip_adapter_image` input argument.

negative_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be generated from `negative_prompt` input argument.

negative_pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled negative_prompt_embeds will be generated from `negative_prompt` input argument.

output_type (`str`, *optional*, defaults to `"pil"`) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): `PIL.Image.Image` or `np.array`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~pipelines.flux.FluxPipelineOutput` instead of a plain tuple.

joint_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

callback_on_step_end (`Callable`, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list`, *optional*) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

max_sequence_length (`int` defaults to 512) : Maximum sequence length to use with the `prompt`.

max_area (`int`, defaults to `1024 ** 2`) : The maximum area of the generated image in pixels. The height and width will be adjusted to fit this area while maintaining the aspect ratio.

_auto_resize (`bool`, *optional*, defaults to `True`) : Whether to automatically resize the input image to the preferred resolutions.

**Returns:** `~pipelines.flux.FluxPipelineOutput` or `tuple`

`~pipelines.flux.FluxPipelineOutput` if `return_dict`
is True, otherwise a `tuple`. When returning a tuple, the first element is a list with the generated
images.

Function invoked when calling the pipeline for generation.

Examples:
# Inpainting with text only

```py
>>> import torch
>>> from diffusers import FluxKontextInpaintPipeline
>>> from diffusers.utils import load_image

>>> prompt = "Change the yellow dinosaur to green one"
>>> img_url = (
...     "https://github.com/ZenAI-Vietnam/Flux-Kontext-pipelines/blob/main/assets/dinosaur_input.jpeg?raw=true"
... )
>>> mask_url = (
...     "https://github.com/ZenAI-Vietnam/Flux-Kontext-pipelines/blob/main/assets/dinosaur_mask.png?raw=true"
... )

>>> source = load_image(img_url)
>>> mask = load_image(mask_url)

>>> pipe = FluxKontextInpaintPipeline.from_pretrained(
...     "black-forest-labs/FLUX.1-Kontext-dev", torch_dtype=torch.bfloat16
... )
>>> pipe.to("cuda")

>>> image = pipe(prompt=prompt, image=source, mask_image=mask, strength=1.0).images[0]
>>> image.save("kontext_inpainting_normal.png")
```

# Inpainting with image conditioning

```py
>>> import torch
>>> from diffusers import FluxKontextInpaintPipeline
>>> from diffusers.utils import load_image

>>> pipe = FluxKontextInpaintPipeline.from_pretrained(
...     "black-forest-labs/FLUX.1-Kontext-dev", torch_dtype=torch.bfloat16
... )
>>> pipe.to("cuda")

>>> prompt = "Replace this ball"
>>> img_url = "https://images.pexels.com/photos/39362/the-ball-stadion-football-the-pitch-39362.jpeg?auto=compress&cs=tinysrgb&dpr=1&w=500"
>>> mask_url = (
...     "https://github.com/ZenAI-Vietnam/Flux-Kontext-pipelines/blob/main/assets/ball_mask.png?raw=true"
... )
>>> image_reference_url = (
...     "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTah3x6OL_ECMBaZ5ZlJJhNsyC-OSMLWAI-xw&s"
... )

>>> source = load_image(img_url)
>>> mask = load_image(mask_url)
>>> image_reference = load_image(image_reference_url)

>>> mask = pipe.mask_processor.blur(mask, blur_factor=12)
>>> image = pipe(
...     prompt=prompt, image=source, mask_image=mask, image_reference=image_reference, strength=1.0
... ).images[0]
>>> image.save("kontext_inpainting_ref.png")
```

#### encode_prompt[[diffusers.FluxKontextInpaintPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], prompt_2: str | list[str] | None = None, device: typing.Optional[torch.device] = None, num_images_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.FloatTensor] = None, pooled_prompt_embeds: typing.Optional[torch.FloatTensor] = None, max_sequence_length: int = 512, lora_scale: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/flux/pipeline_flux_kontext_inpaint.py#L391)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

prompt_2 (`str` or `list[str]`, *optional*) : The prompt or prompts to be sent to the `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is used in all text-encoders

device : (`torch.device`): torch device

num_images_per_prompt (`int`) : number of images that should be generated per prompt

prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

pooled_prompt_embeds (`torch.FloatTensor`, *optional*) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, pooled text embeddings will be generated from `prompt` input argument.

lora_scale (`float`, *optional*) : A lora scale that will be applied to all LoRA layers of the text encoder if LoRA layers are loaded.
