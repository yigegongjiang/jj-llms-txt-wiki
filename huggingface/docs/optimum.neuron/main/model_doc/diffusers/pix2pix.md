# InstructPix2Pix

## Overview

[InstructPix2Pix: Learning to Follow Image Editing Instructions](https://huggingface.co/papers/2211.09800) is by Tim Brooks, Aleksander Holynski and Alexei A. Efros.

🤗 `Optimum` extends `Diffusers` to support inference on the second generation of Neuron devices(powering Trainium and Inferentia 2). It aims at inheriting the ease of Diffusers on Neuron.

## Export to Neuron

To deploy models, you will need to compile them to TorchScript optimized for AWS Neuron. In the case of Stable Diffusion, there are four components which need to be exported to the `.neuron` format to boost the performance:

* Text encoder
* U-Net
* VAE encoder
* VAE decoder

You can either compile and export a Stable Diffusion Checkpoint via CLI or `NeuronStableDiffusionInstructPix2PixPipeline` class.

## Usage Example

With the `NeuronStableDiffusionInstructPix2PixPipeline` class, you can apply instruction-based image editing using both text guidance and image guidance.

```python
import requests
import PIL
from io import BytesIO
from optimum.neuron import NeuronStableDiffusionInstructPix2PixPipeline

def download_image(url):
    response = requests.get(url)
    return PIL.Image.open(BytesIO(response.content)).convert("RGB")

model_id = "timbrooks/instruct-pix2pix"
input_shapes = {"batch_size": 1, "height": 512, "width": 512}

pipe = NeuronStableDiffusionInstructPix2PixPipeline.from_pretrained(
  model_id, export=True, dynamic_batch_size=True, **input_shapes,
)
pipe.save_pretrained("sd_ip2p/")

img_url = "https://huggingface.co/datasets/diffusers/diffusers-images-docs/resolve/main/mountain.png"
init_image = download_image(img_url).resize((512, 512))

prompt = "Add a beautiful sunset"
image = pipe(prompt=prompt, image=init_image).images[0]
image.save("sunset_mountain.png")
```

`image`          | `prompt` | output |
:-------------------------:|:-------------------------:|-------------------------:|
 | ***Add a beautiful sunset*** |  |

#### NeuronStableDiffusionInstructPix2PixPipeline[[optimum.neuron.NeuronStableDiffusionInstructPix2PixPipeline]]

#### optimum.neuron.NeuronStableDiffusionInstructPix2PixPipeline[[optimum.neuron.NeuronStableDiffusionInstructPix2PixPipeline]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_diffusion.py#L1548)

__call__optimum.neuron.NeuronStableDiffusionInstructPix2PixPipeline.__call__https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_diffusion.py#L1094[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]

Are there any other diffusion features that you want us to support in 🤗`Optimum-neuron`? Please file an issue to [`Optimum-neuron` Github repo](https://github.com/huggingface/optimum-neuron) or discuss with us on [HuggingFace’s community forum](https://discuss.huggingface.co/c/optimum/), cheers 🤗 !
