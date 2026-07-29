# Stable Diffusion XL Turbo

## Overview

SDXL Turbo is an adversarial time-distilled Stable Diffusion XL (SDXL) model capable of running inference in as little as 1 step ([check `🤗diffusers` for more details](https://huggingface.co/docs/diffusers/using-diffusers/sdxl_turbo)).

In `optimum-neuron`, you can:
  - Use the class `NeuronStableDiffusionXLPipeline` to compile and run inference.

Here we will compile the [`stabilityai/sdxl-turbo`](https://huggingface.co/stabilityai/sdxl-turbo) model with Optimum CLI.

## Export to Neuron

```bash
optimum-cli export neuron --model stabilityai/sdxl-turbo --batch_size 1 --height 512 --width 512 --auto_cast matmul --auto_cast_type bf16 sdxl_turbo_neuron/
```

## Text-to-Image

Now we can generate images from text prompts on Inf2 using the pre-compiled model:

```python
from optimum.neuron import NeuronStableDiffusionXLPipeline

pipe = NeuronStableDiffusionXLPipeline.from_pretrained("sdxl_turbo_neuron/", data_parallel_mode="all")
prompt = ["Self-portrait oil painting, a beautiful cyborg with golden hair, 8k"] * 2

images = pipe(prompt=prompt, guidance_scale=0.0, num_inference_steps=1).images
```

Inf2 instances contain one or more Neuron devices, and each Neuron device includes 2 NeuronCore-v2. By default, we load the whole pipeline of LCM to both Neuron cores. It means that when the batch size is divisible by 2, you can fully leverage the compute power of both cores.

Are there any other diffusion features that you want us to support in 🤗`Optimum-neuron`? Please file an issue to [`Optimum-neuron` Github repo](https://github.com/huggingface/optimum-neuron) or discuss with us on [HuggingFace’s community forum](https://discuss.huggingface.co/c/optimum/), cheers 🤗 !
