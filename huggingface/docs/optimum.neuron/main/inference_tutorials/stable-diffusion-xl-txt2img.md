# Stable Diffusion XL with Neuronx: Text to image

`🤗 Optimum` extends `🤗 Diffusers` to support inference on the second generation of Neuron devices(powering Trainium and Inferentia 2). It aims at inheriting the ease of Diffusers on Neuron.

To get started, make sure you have [configured your inf2 / trn1 instance](https://huggingface.co/docs/optimum-neuron/installation), and installed optimum:

```python
!pip install "optimum-neuron[neuronx]" diffusers matplotlib
```

## Compilation

To deploy SDXL models, we will also start by compiling the models. We support the export of following components in the pipeline to boost the speed:

* Text encoder
* Second text encoder
* U-Net (a three times larger UNet than the one in Stable Diffusion pipeline)
* VAE encoder
* VAE decoder

You can either compile and export a Stable Diffusion Checkpoint via CLI or `NeuronStableDiffusionXLPipeline` class. 
In this tutorial, we will export [`stabilityai/stable-diffusion-xl-base-1.0`](https://huggingface.co/stabilityai/stable-diffusion-xl-base-1.0) with the API.

```python
from optimum.neuron import NeuronStableDiffusionXLPipeline

model_id = "stabilityai/stable-diffusion-xl-base-1.0"
num_image_per_prompt = 1
input_shapes = {"batch_size": 1, "height": 1024, "width": 1024, "num_image_per_prompt": num_image_per_prompt}
compiler_args = {"auto_cast": "matmul", "auto_cast_type": "bf16"}
```

```python
# Compile and save
stable_diffusion_xl = NeuronStableDiffusionXLPipeline.from_pretrained(
    model_id, export=True, device_ids=[0, 1], **compiler_args, **input_shapes
)

save_directory = "sd_neuron_xl/"
stable_diffusion_xl.save_pretrained(save_directory)
```

```python
# Push and share your model to the HuggingFace hub
repository_id = "your-username/your-awesome-model"  # Replace with your repo id, eg. "Jingya/stable-diffusion-xl-base-1.0-neuronx".
stable_diffusion_xl.push_to_hub(save_directory, repository_id=repository_id, use_auth_token=True)
```

Feel free to use the following command as well for compiling and exporting the model:

```bash
optimum-cli export neuron --model stabilityai/stable-diffusion-xl-base-1.0 \
  --task stable-diffusion-xl \
  --batch_size 1 \
  --height 1024 `# height in pixels of generated image, eg. 768, 1024` \
  --width 1024 `# width in pixels of generated image, eg. 768, 1024` \
  --num_images_per_prompt 1 `# number of images to generate per prompt, defaults to 1` \
  --auto_cast matmul `# cast only matrix multiplication operations` \
  --auto_cast_type bf16 `# cast operations from FP32 to BF16` \
  sd_neuron_xl/
```

We Recommend `inf2.8xlarge` or larger for compilation. You will also be able to compile the models with a CPU-only instance *(needs ~92GB memory)* using the CLI with `--disable-validation`, which disables the validation of inference on neuron devices.

In the following section, we will run the pre-compiled model on Neuron devices, to reduce expenses, you can run inference with `inf2.xlarge` instance.

## Text-to-image Inference

If you have pre-compiled Stable Diffusion XL models, you can load them directly to skip the compilation: 

```python
# stable_diffusion_xl = NeuronStableDiffusionXLPipeline.from_pretrained("your-username/your-awesome-model")  # Pass a local path or your repo id on the HuggingFace hub.
```

```python
from diffusers import DPMSolverMultistepScheduler

stable_diffusion_xl.scheduler = DPMSolverMultistepScheduler.from_config(stable_diffusion_xl.scheduler.config)
```

```python
import time

import numpy as np
from matplotlib import image as mpimg
from matplotlib import pyplot as plt
```

```python
>>> # Run pipeline
>>> prompt = [
...     "a photo of an astronaut riding a horse on mars",
...     "sonic on the moon",
...     "elvis playing guitar while eating a hotdog",
...     "saved by the bell",
...     "engineers eating lunch at the opera",
...     "panda eating bamboo on a plane",
...     "A digital illustration of a steampunk flying machine in the sky with cogs and mechanisms, 4k, detailed, trending in artstation, fantasy vivid colors",
...     "kids playing soccer at the FIFA World Cup"
... ]

>>> plt.title("Image")
>>> plt.xlabel("X pixel scaling")
>>> plt.ylabel("Y pixels scaling")

>>> total_time = 0
>>> for x in prompt:
...     start_time = time.time()
...     image = stable_diffusion_xl(x).images[0]
...     inf_time = time.time() - start_time
...     print(f"[Inference Time] {np.round(inf_time, 2)} seconds.")
...     image.save("image.png")
...     image = mpimg.imread("image.png")
...     #clear_output(wait=True)
...     plt.imshow(image)
...     plt.show()
```

[Inference Time] 13.74 seconds.

```python

```
