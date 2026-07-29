# ControlNet

ControlNet conditions the stable diffusion model with an additional input image. In Optimum Neuron, we support the compilation of one or multiple ControlNet(s) along with the stable diffusion checkpoint. Then you can use the compiled artifacts to generate styled images.

## Export to Neuron

We can either compile one or multiple ControlNet via the Optimum CLI or programmatically via the `NeuronStableDiffusionControlNetPipeline` class by passing the `controlnet_ids`.

### Option 1: CLI

```bash
optimum-cli export neuron -m stable-diffusion-v1-5/stable-diffusion-v1-5 --batch_size 1 --height 512 --width 512 --controlnet_ids lllyasviel/sd-controlnet-canny --num_images_per_prompt 1 sd_neuron_controlnet/
```

### Option 2: Python API

```python
from optimum.neuron import NeuronStableDiffusionControlNetPipeline

model_id = "stable-diffusion-v1-5/stable-diffusion-v1-5"
controlnet_id = "lllyasviel/sd-controlnet-canny"

# [Neuron] pipeline
input_shapes = {"batch_size": 1, "height": 512, "width": 512, "num_images_per_prompt": 1}
compiler_args = {"auto_cast": "matmul", "auto_cast_type": "bf16"}
pipe = NeuronStableDiffusionControlNetPipeline.from_pretrained(
    model_id,
    controlnet_ids=controlnet_id,
    export=True,
    **input_shapes,
    **compiler_args,
)
pipe.save_pretrained("sd_neuron_controlnet")
```

## Text-to-Image

For text-to-image, we can specify an additional conditioning input.

Here is an example with a canny image, a white outline of an image on a black background. The ControlNet will use the canny image as a control to guide the model to generate an image with the same outline.

```python
import cv2
import numpy as np
from diffusers import UniPCMultistepScheduler
from diffusers.utils import load_image, make_image_grid
from PIL import Image

from optimum.neuron import NeuronStableDiffusionControlNetPipeline

# prepare canny image
original_image = load_image(
    "https://hf.co/datasets/huggingface/documentation-images/resolve/main/diffusers/input_image_vermeer.png"
)

image = np.array(original_image)

low_threshold = 100
high_threshold = 200

image = cv2.Canny(image, low_threshold, high_threshold)
image = image[:, :, None]
image = np.concatenate([image, image, image], axis=2)
canny_image = Image.fromarray(image)

# load pre-compiled neuron model
pipe = NeuronStableDiffusionControlNetPipeline.from_pretrained("sd_neuron_controlnet")
pipe.scheduler = UniPCMultistepScheduler.from_config(pipe.scheduler.config)

# inference
output = pipe("the mona lisa", image=canny_image).images[0]
compare = make_image_grid([original_image, canny_image, output], rows=1, cols=3)
compare.save("compare.png")
```

## MultiControlNet

With Optimum Neuron, you can also compose multiple ControlNet conditionings from different image inputs:

* Compile multiple ControlNet for SD1.5

```bash
optimum-cli export neuron --inline-weights-neff --model jyoung105/stable-diffusion-v1-5 --task stable-diffusion --auto_cast matmul --auto_cast_type bf16 --batch_size 1 --num_images_per_prompt 1 --controlnet_ids lllyasviel/control_v11p_sd15_openpose lllyasviel/control_v11f1p_sd15_depth --height 512 --width 512 sd15-512x512-bf16-openpose-depth
```

* Run SD1.5 with OpenPose and Depth conditionings:

```python
import numpy as np
import torch
from PIL import Image

from controlnet_aux import OpenposeDetector
from transformers import pipeline
from diffusers import UniPCMultistepScheduler
from diffusers.utils import load_image
from optimum.neuron import NeuronStableDiffusionControlNetPipeline

# OpenPose+Depth ControlNet
model_id = "sd15-512x512-bf16-openpose-depth"

# Load ControlNet images

# 1. openpose
image = load_image("https://huggingface.co/lllyasviel/control_v11p_sd15_openpose/resolve/main/images/input.png")
processor = OpenposeDetector.from_pretrained('lllyasviel/ControlNet')
openpose_image = processor(image)

# 2. depth
image = load_image("https://huggingface.co/lllyasviel/control_v11p_sd15_depth/resolve/main/images/input.png")
depth_estimator = pipeline('depth-estimation')
image = depth_estimator(image)['depth']
image = np.array(image)
image = image[:, :, None]
image = np.concatenate([image, image, image], axis=2)
depth_image = Image.fromarray(image)

images = [openpose_image.resize((512, 512)), depth_image.resize((512, 512))]

# 3. inference
pipe = NeuronStableDiffusionControlNetPipeline.from_pretrained(model_id)
pipe.scheduler = UniPCMultistepScheduler.from_config(pipe.scheduler.config)
prompt = "a giant in a fantasy landscape, best quality"
negative_prompt = "monochrome, lowres, bad anatomy, worst quality, low quality"

image = pipe(prompt=prompt, image=images).images[0]
image.save('out.png')
```

## ControlNet with Stable Diffusion XL

### Export to Neuron

```bash
optimum-cli export neuron -m stabilityai/stable-diffusion-xl-base-1.0 --task stable-diffusion-xl --batch_size 1 --height 1024 --width 1024 --controlnet_ids diffusers/controlnet-canny-sdxl-1.0-small --num_images_per_prompt 1 sdxl_neuron_controlnet/
```

### Text-to-Image

```python
import cv2
import numpy as np
from diffusers.utils import load_image
from PIL import Image
from optimum.neuron import NeuronStableDiffusionXLControlNetPipeline

# Inputs
prompt = "aerial view, a futuristic research complex in a bright foggy jungle, hard lighting"
negative_prompt = "low quality, bad quality, sketches"

image = load_image(
    "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main/sd_controlnet/hf-logo.png"
)
image = np.array(image)
image = cv2.Canny(image, 100, 200)
image = image[:, :, None]
image = np.concatenate([image, image, image], axis=2)
image = Image.fromarray(image)

controlnet_conditioning_scale = 0.5  # recommended for good generalization

pipe = NeuronStableDiffusionXLControlNetPipeline.from_pretrained("sdxl_neuron_controlnet")

images = pipe(
    prompt,
    negative_prompt=negative_prompt,
    image=image,
    controlnet_conditioning_scale=controlnet_conditioning_scale,
).images
images[0].save("hug_lab.png")
```

## NeuronStableDiffusionControlNetPipeline[[optimum.neuron.NeuronStableDiffusionControlNetPipeline]]

#### optimum.neuron.NeuronStableDiffusionControlNetPipeline[[optimum.neuron.NeuronStableDiffusionControlNetPipeline]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_diffusion.py#L1561)

__call__optimum.neuron.NeuronStableDiffusionControlNetPipeline.__call__https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/pipelines/diffusers/pipeline_controlnet.py#L33[{"name": "prompt", "val": ": str | list[str] | None = None"}, {"name": "image", "val": ": typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, typing.List[PIL.Image.Image], typing.List[numpy.ndarray], typing.List[torch.Tensor]] = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "timesteps", "val": ": list[int] | None = None"}, {"name": "sigmas", "val": ": list[float] | None = None"}, {"name": "guidance_scale", "val": ": float = 7.5"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "ip_adapter_image", "val": ": typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, typing.List[PIL.Image.Image], typing.List[numpy.ndarray], typing.List[torch.Tensor], NoneType] = None"}, {"name": "ip_adapter_image_embeds", "val": ": list[torch.Tensor] | None = None"}, {"name": "output_type", "val": ": str = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "controlnet_conditioning_scale", "val": ": float | list[float] = 1.0"}, {"name": "guess_mode", "val": ": bool = False"}, {"name": "control_guidance_start", "val": ": float | list[float] = 0.0"}, {"name": "control_guidance_end", "val": ": float | list[float] = 1.0"}, {"name": "clip_skip", "val": ": int | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Union[typing.Callable[[int, int, dict], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list[str] = ['latents']"}, {"name": "**kwargs", "val": ""}]- **prompt** (`str | list[str] | None`, defaults to `None`) --
  The prompt or prompts to guide image generation. If not defined, you need to pass `prompt_embeds`.
- **image** (`"PipelineImageInput" | None`, defaults to `None`) --
  The ControlNet input condition to provide guidance to the `unet` for generation. If the type is
  specified as `torch.Tensor`, it is passed to ControlNet as is. `PIL.Image.Image` can also be accepted
  as an image. The dimensions of the output image defaults to `image`'s dimensions. If height and/or
  width are passed, `image` is resized accordingly. If multiple ControlNets are specified in `init`,
  images must be passed as a list such that each element of the list can be correctly batched for input
  to a single ControlNet. When `prompt` is a list, and if a list of images is passed for a single
  ControlNet, each will be paired with each prompt in the `prompt` list. This also applies to multiple
  ControlNets, where a list of image lists can be passed to batch for each prompt and each ControlNet.
- **num_inference_steps** (`int`, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **timesteps** (`list[int] | None`, defaults to `None`) --
  Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument
  in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is
  passed will be used. Must be in descending order.
- **sigmas** (`list[int] | None`, defaults to `None`) --
  Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in
  their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed
  will be used.
- **guidance_scale** (`float`, defaults to 7.5) --
  A higher guidance scale value encourages the model to generate images closely linked to the text
  `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.
- **negative_prompt** (`str | list[str] | None`, defaults to `None`) --
  The prompt or prompts to guide what to not include in image generation. If not defined, you need to
  pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale 0`diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput` or `tuple`If `return_dict` is `True`, `diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput` is returned,
otherwise a `tuple` is returned where the first element is a list with the generated images and the
second element is a list of `bool`s indicating whether the corresponding generated image contains
"not-safe-for-work" (nsfw) content.

The call function to the pipeline for generation.

**Parameters:**

prompt (`str | list[str] | None`, defaults to `None`) : The prompt or prompts to guide image generation. If not defined, you need to pass `prompt_embeds`.

image (`"PipelineImageInput" | None`, defaults to `None`) : The ControlNet input condition to provide guidance to the `unet` for generation. If the type is specified as `torch.Tensor`, it is passed to ControlNet as is. `PIL.Image.Image` can also be accepted as an image. The dimensions of the output image defaults to `image`'s dimensions. If height and/or width are passed, `image` is resized accordingly. If multiple ControlNets are specified in `init`, images must be passed as a list such that each element of the list can be correctly batched for input to a single ControlNet. When `prompt` is a list, and if a list of images is passed for a single ControlNet, each will be paired with each prompt in the `prompt` list. This also applies to multiple ControlNets, where a list of image lists can be passed to batch for each prompt and each ControlNet.

num_inference_steps (`int`, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

timesteps (`list[int] | None`, defaults to `None`) : Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used. Must be in descending order.

sigmas (`list[int] | None`, defaults to `None`) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

guidance_scale (`float`, defaults to 7.5) : A higher guidance scale value encourages the model to generate images closely linked to the text `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.

negative_prompt (`str | list[str] | None`, defaults to `None`) : The prompt or prompts to guide what to not include in image generation. If not defined, you need to pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale  1`.
- **negative_prompt** (`str | list[str] | None`, defaults to `None`) --
  The prompt or prompts to guide what to not include in image generation. If not defined, you need to
  pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale 0`diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput` or `tuple`If `return_dict` is `True`, `diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput` is returned,
otherwise a `tuple` is returned containing the output images.

The call function to the pipeline for generation.

Examples:

**Parameters:**

prompt (`str | list[str]`, defaults to `None`) : The prompt or prompts to guide image generation. If not defined, you need to pass `prompt_embeds`.

prompt_2 (`str | list[str]`, defaults to `None`) : The prompt or prompts to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is used in both text-encoders.

image (`PipelineImageInput | None`, defaults to `None`) : The ControlNet input condition to provide guidance to the `unet` for generation. If the type is specified as `torch.Tensor`, it is passed to ControlNet as is. `PIL.Image.Image` can also be accepted as an image. The dimensions of the output image default to `image`'s dimensions. If height and/or width are passed, `image` is resized accordingly. If multiple ControlNets are specified in `init`, images must be passed as a list such that each element of the list can be correctly batched for input to a single ControlNet.

num_inference_steps (`int`, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

timesteps (`list[int] | None`, defaults to `None`) : Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used. Must be in descending order.

sigmas (`list[float] | None`, defaults to `None`) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

denoising_end (`float | None`, defaults to `None`) : When specified, determines the fraction (between 0.0 and 1.0) of the total denoising process to be completed before it is intentionally prematurely terminated. As a result, the returned sample will still retain a substantial amount of noise as determined by the discrete timesteps selected by the scheduler. The denoising_end parameter should ideally be utilized when this pipeline forms a part of a "Mixture of Denoisers" multi-pipeline setup, as elaborated in [**Refining the Image Output**](https://huggingface.co/docs/diffusers/api/pipelines/stable_diffusion/stable_diffusion_xl#refining-the-image-output)

guidance_scale (`float`, defaults to 5.0) : A higher guidance scale value encourages the model to generate images closely linked to the text `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.

negative_prompt (`str | list[str] | None`, defaults to `None`) : The prompt or prompts to guide what to not include in image generation. If not defined, you need to pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale < 1`).

negative_prompt_2 (`str | list[str] | None`, defaults to `None`) : The prompt or prompts to guide what to not include in image generation. This is sent to `tokenizer_2` and `text_encoder_2`. If not defined, `negative_prompt` is used in both text-encoders.

num_images_per_prompt (`int`, defaults to 1) : The number of images to generate per prompt.

eta (`float`, defaults to 0.0) : Corresponds to parameter eta (η) from the [DDIM](https://arxiv.org/abs/2010.02502) paper. Only applies to the `diffusers.schedulers.DDIMScheduler`, and is ignored in other schedulers.

generator (`torch.Generator | list[torch.Generator] | None`, defaults to `None`) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor | None`, defaults to `None`) : Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor is generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor | None`, defaults to `None`) : Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, text embeddings are generated from the `prompt` input argument.

negative_prompt_embeds (`torch.Tensor | None`, defaults to `None`) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, `negative_prompt_embeds` are generated from the `negative_prompt` input argument.

pooled_prompt_embeds (`torch.Tensor | None`, defaults to `None`) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, pooled text embeddings are generated from `prompt` input argument.

negative_pooled_prompt_embeds (`torch.Tensor | None`, defaults to `None`) : Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, pooled `negative_prompt_embeds` are generated from `negative_prompt` input argument.

ip_adapter_image (`PipelineImageInput | None`, defaults to `None`) : Optional image input to work with IP Adapters.

ip_adapter_image_embeds (`list[torch.Tensor] | None`, defaults to `None`) : Pre-generated image embeddings for IP-Adapter. It should be a list of length the same as the number of IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. It should contain the negative image embedding if `do_classifier_free_guidance` is set to `True`. If not provided, embeddings are computed from the `ip_adapter_image` input argument.

output_type (`str | None`, defaults to `"pil"`) : The output format of the generated image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, defaults to `True`) : Whether or not to return a `~pipelines.stable_diffusion.StableDiffusionPipelineOutput` instead of a plain tuple.

cross_attention_kwargs (`dict[str, Any] | None`, defaults to `None`) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined in [`self.processor`](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

controlnet_conditioning_scale (`float | list[float]`, defaults to 1.0) : The outputs of the ControlNet are multiplied by `controlnet_conditioning_scale` before they are added to the residual in the original `unet`. If multiple ControlNets are specified in `init`, you can set the corresponding scale as a list.

guess_mode (`bool`, defaults to `False`) : The ControlNet encoder tries to recognize the content of the input image even if you remove all prompts. A `guidance_scale` value between 3.0 and 5.0 is recommended.

control_guidance_start (`float | list[float]`, defaults to 0.0) : The percentage of total steps at which the ControlNet starts applying.

control_guidance_end (`float | list[float]`, defaults to 1.0) : The percentage of total steps at which the ControlNet stops applying.

original_size (`tuple[int, int] | None`, defaults to (1024, 1024)) : If `original_size` is not the same as `target_size`, the image will appear to be down- or upsampled. `original_size` defaults to `(height, width)` if not specified. Part of SDXL's micro-conditioning as explained in section 2.2 of [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).

crops_coords_top_left (`tuple[int, int]`, defaults to (0, 0)) : `crops_coords_top_left` can be used to generate an image that appears to be "cropped" from the position `crops_coords_top_left` downwards. Favorable, well-centered images are usually achieved by setting `crops_coords_top_left` to (0, 0). Part of SDXL's micro-conditioning as explained in section 2.2 of [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).

target_size (`tuple[int, int] | None`, defaults to `None`) : For most cases, `target_size` should be set to the desired height and width of the generated image. If not specified, it will default to `(height, width)`. Part of SDXL's micro-conditioning as explained in section 2.2 of [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).

negative_original_size (`tuple[int, int] | None`, defaults to `None`) : To negatively condition the generation process based on a specific image resolution. Part of SDXL's micro-conditioning as explained in section 2.2 of [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952). For more information, refer to this issue thread: https://github.com/huggingface/diffusers/issues/4208.

negative_crops_coords_top_left (`tuple[int, int]`, defaults to (0, 0)) : To negatively condition the generation process based on a specific crop coordinates. Part of SDXL's micro-conditioning as explained in section 2.2 of [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952). For more information, refer to this issue thread: https://github.com/huggingface/diffusers/issues/4208.

negative_target_size (`tuple[int, int] | None`, defaults to `None`) : To negatively condition the generation process based on a target image resolution. It should be the same as the `target_size` for most cases. Part of SDXL's micro-conditioning as explained in section 2.2 of [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952). For more information, refer to this issue thread: https://github.com/huggingface/diffusers/issues/4208.

clip_skip (`int | None`, defaults to `None`) : Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that the output of the pre-final layer will be used for computing the prompt embeddings.

callback_on_step_end (`Callable[[int, int, dict], None] | PipelineCallback | MultiPipelineCallbacks | None`, defaults to `None`) : A function or a subclass of `PipelineCallback` or `MultiPipelineCallbacks` that is called at the end of each denoising step during the inference with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list[str]`, defaults to `["latents"]`) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

**Returns:**

``diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput` or `tuple``

If `return_dict` is `True`, `diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput` is returned,
otherwise a `tuple` is returned containing the output images.

Are there any other diffusion features that you want us to support in 🤗`Optimum-neuron`? Please file an issue to [`Optimum-neuron` Github repo](https://github.com/huggingface/optimum-neuron) or discuss with us on [HuggingFace’s community forum](https://discuss.huggingface.co/c/optimum/), cheers 🤗 !
