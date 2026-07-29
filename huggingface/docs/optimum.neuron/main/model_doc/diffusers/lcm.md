# Latent Consistency Models

## Overview

Latent Consistency Models (LCMs) were proposed in [Latent Consistency Models: Synthesizing High-Resolution Images with Few-Step Inference by Simian Luo, Yiqin Tan, Longbo Huang, Jian Li, and Hang Zhao](https://huggingface.co/papers/2310.04378). LCMs enable inference with fewer steps on any pre-trained LDMs, including Stable Diffusion and SDXL.

In `optimum-neuron`, you can:
  - Use the class `NeuronLatentConsistencyModelPipeline` to compile and run inference of LCMs distilled from Stable Diffusion (SD) models.
  - And continue to use the class `NeuronStableDiffusionXLPipeline` for LCMs distilled from SDXL models.

Here are examples to compile the LCMs of Stable Diffusion ( [SimianLuo/LCM_Dreamshaper_v7](https://huggingface.co/SimianLuo/LCM_Dreamshaper_v7) ) and Stable Diffusion XL( [latent-consistency/lcm-sdxl](https://huggingface.co/latent-consistency/lcm-sdxl) ), and then run inference on AWS Inferentia 2 :

## Export to Neuron

### LCM of Stable Diffusion

```python
from optimum.neuron import NeuronLatentConsistencyModelPipeline

model_id = "SimianLuo/LCM_Dreamshaper_v7"
num_images_per_prompt = 1
input_shapes = {"batch_size": 1, "height": 768, "width": 768, "num_images_per_prompt": num_images_per_prompt}
compiler_args = {"auto_cast": "matmul", "auto_cast_type": "bf16"}

stable_diffusion = NeuronLatentConsistencyModelPipeline.from_pretrained(
    model_id, export=True, **compiler_args, **input_shapes
)
save_directory = "lcm_sd_neuron/"
stable_diffusion.save_pretrained(save_directory)

# Push to hub
stable_diffusion.push_to_hub(save_directory, repository_id="my-neuron-repo")  # Replace with your repo id, eg. "Jingya/LCM_Dreamshaper_v7_neuronx"
```

### LCM of Stable Diffusion XL

```python
from optimum.neuron import NeuronStableDiffusionXLPipeline

model_id = "stabilityai/stable-diffusion-xl-base-1.0"
unet_id = "latent-consistency/lcm-sdxl"
num_images_per_prompt = 1
input_shapes = {"batch_size": 1, "height": 1024, "width": 1024, "num_images_per_prompt": num_images_per_prompt}
compiler_args = {"auto_cast": "matmul", "auto_cast_type": "bf16"}

stable_diffusion = NeuronStableDiffusionXLPipeline.from_pretrained(
    model_id, unet_id=unet_id, export=True, **compiler_args, **input_shapes
)
save_directory = "lcm_sdxl_neuron/"
stable_diffusion.save_pretrained(save_directory)

# Push to hub
stable_diffusion.push_to_hub(save_directory, repository_id="my-neuron-repo")   # Replace with your repo id, eg. "Jingya/lcm-sdxl-neuronx"
```

## Text-to-Image

Now we can generate images from text prompts on Inf2 using the pre-compiled model:

* LCM of Stable Diffusion

```python
from optimum.neuron import NeuronLatentConsistencyModelPipeline

pipe = NeuronLatentConsistencyModelPipeline.from_pretrained("Jingya/LCM_Dreamshaper_v7_neuronx")
prompts = ["Self-portrait oil painting, a beautiful cyborg with golden hair, 8k"] * 2

images = pipe(prompt=prompts, num_inference_steps=4, guidance_scale=8.0).images
```

* LCM of Stable Diffusion XL

```python
from optimum.neuron import NeuronStableDiffusionXLPipeline

pipe = NeuronStableDiffusionXLPipeline.from_pretrained("Jingya/lcm-sdxl-neuronx")
prompts = ["a close-up picture of an old man standing in the rain"] * 2

images = pipe(prompt=prompts, num_inference_steps=4, guidance_scale=8.0).images
```

## NeuronLatentConsistencyModelPipeline[[optimum.neuron.NeuronLatentConsistencyModelPipeline]]

#### optimum.neuron.NeuronLatentConsistencyModelPipeline[[optimum.neuron.NeuronLatentConsistencyModelPipeline]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_diffusion.py#L1556)

__call__optimum.neuron.NeuronLatentConsistencyModelPipeline.__call__https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_diffusion.py#L1094[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]

Are there any other diffusion features that you want us to support in 🤗`Optimum-neuron`? Please file an issue to [`Optimum-neuron` Github repo](https://github.com/huggingface/optimum-neuron) or discuss with us on [HuggingFace’s community forum](https://discuss.huggingface.co/c/optimum/), cheers 🤗 !
