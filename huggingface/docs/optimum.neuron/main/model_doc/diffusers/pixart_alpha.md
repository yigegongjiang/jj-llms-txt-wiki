# PixArt-α

## Overview

[PixArt-α: Fast Training of Diffusion Transformer for Photorealistic Text-to-Image Synthesis](https://huggingface.co/papers/2310.00426) is Junsong Chen, Jincheng Yu, Chongjian Ge, Lewei Yao, Enze Xie, Yue Wu, Zhongdao Wang, James Kwok, Ping Luo, Huchuan Lu, and Zhenguo Li.

Some notes about this pipeline:

* It uses a Transformer backbone (instead of a UNet) for denoising. As such it has a similar architecture as [DiT](./dit).
* It was trained using text conditions computed from T5. This aspect makes the pipeline better at following complex text prompts with intricate details.
* It is good at producing high-resolution images at different aspect ratios. To get the best results, the authors recommend some size brackets which can be found [here](https://github.com/PixArt-alpha/PixArt-alpha/blob/08fbbd281ec96866109bdd2cdb75f2f58fb17610/diffusion/data/datasets/utils.py).
* It rivals the quality of state-of-the-art text-to-image generation systems (as of this writing) such as Stable Diffusion XL, Imagen, and DALL-E 2, while being more efficient than them.

You can find the original codebase at [PixArt-alpha/PixArt-alpha](https://github.com/PixArt-alpha/PixArt-alpha) and all the available checkpoints at [PixArt-alpha](https://huggingface.co/PixArt-alpha).

🤗 `Optimum` extends `Diffusers` to support inference on the second generation of Neuron devices(powering Trainium and Inferentia 2). It aims at inheriting the ease of Diffusers on Neuron.

## Export to Neuron

To deploy models in the PixArt-α pipeline, you will need to compile them to TorchScript optimized for AWS Neuron. There are four components which need to be exported to the `.neuron` format to boost the performance:

* Text encoder
* Transformer
* VAE encoder
* VAE decoder

You can either compile and export a PixArt-α Checkpoint via CLI or `NeuronPixArtAlphaPipeline` class.

### Option 1: CLI

```bash
optimum-cli export neuron --model PixArt-alpha/PixArt-XL-2-512x512 --batch_size 1 --height 512 --width 512 --num_images_per_prompt 1 --torch_dtype bfloat16 --sequence_length 120 pixart_alpha_neuron_512/
```

> [!TIP]
> We recommend using a `inf2.8xlarge` or a larger instance for the model compilation. You will also be able to compile the model with the Optimum CLI on a CPU-only instance (needs ~35 GB memory), and then run the pre-compiled model on `inf2.xlarge` to reduce the expenses. In this case, don't forget to disable validation of inference by adding the `--disable-validation` argument.

### Option 2: Python API

```python
import torch
from optimum.neuron import NeuronPixArtAlphaPipeline

# Compile
compiler_args = {"auto_cast": "none"}
input_shapes = {"batch_size": 1, "height": 512, "width": 512, "sequence_length": 120}

neuron_model = NeuronPixArtAlphaPipeline.from_pretrained("PixArt-alpha/PixArt-XL-2-512x512", torch_dtype=torch.bfloat16, export=True, disable_neuron_cache=True, **compiler_args, **input_shapes)

# Save locally
neuron_model.save_pretrained("pixart_alpha_neuron_512/")

# Upload to the HuggingFace Hub
neuron_model.push_to_hub(
    "pixart_alpha_neuron_512/", repository_id="Jingya/PixArt-XL-2-512x512-neuronx"  # Replace with your HF Hub repo id
)
```

## Text-to-Image

`NeuronPixArtAlphaPipeline` class allows you to generate images from a text prompt on neuron devices similar to the experience with `Diffusers`.

With pre-compiled PixArt-α models, now generate an image with a prompt on Neuron:

```python
from optimum.neuron import NeuronPixArtAlphaPipeline

neuron_model = NeuronPixArtAlphaPipeline.from_pretrained("pixart_alpha_neuron_512/")
prompt = "Oppenheimer sits on the beach on a chair, watching a nuclear exposition with a huge mushroom cloud, 120mm."
image = neuron_model(prompt=prompt).images[0]
```

## NeuronPixArtAlphaPipeline[[optimum.neuron.NeuronPixArtAlphaPipeline]]

Pipeline for text-to-image generation using PixArt-α.

#### optimum.neuron.NeuronPixArtAlphaPipeline[[optimum.neuron.NeuronPixArtAlphaPipeline]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_diffusion.py#L1568)

__call__optimum.neuron.NeuronPixArtAlphaPipeline.__call__https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_diffusion.py#L1094[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]

Are there any other diffusion features that you want us to support in 🤗`Optimum-neuron`? Please file an issue to [`Optimum-neuron` Github repo](https://github.com/huggingface/optimum-neuron) or discuss with us on [HuggingFace’s community forum](https://discuss.huggingface.co/c/optimum/), cheers 🤗 !
