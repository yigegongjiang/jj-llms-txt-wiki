# Flux

Flux is a series of text-to-image generation models based on diffusion transformers.

> [!TIP]
> We recommend using a `inf2.24xlarge` instance with tensor parallel size 8 for the model compilation and inference.

### Export to Neuron

* Option 1: CLI

```bash
optimum-cli export neuron --model black-forest-labs/FLUX.1-dev --tensor_parallel_size 8 --batch_size 1 --height 1024 --width 1024 --num_images_per_prompt 1 --torch_dtype bfloat16 flux_dev_neuron/
```

* Option 2: Python API

```python
from optimum.neuron import NeuronFluxPipeline

if __name__ == "__main__":
    compiler_args = {"auto_cast": "none"}
    input_shapes = {"batch_size": 1, "height": 1024, "width": 1024}

    pipe = NeuronFluxPipeline.from_pretrained(
        "black-forest-labs/FLUX.1-dev",
        torch_dtype=torch.bfloat16,
        export=True,
        tensor_parallel_size=8,
        **compiler_args,
        **input_shapes
    )

    # Save locally
    pipe.save_pretrained("flux_dev_neuron_1024_tp8/")

    # Upload to the HuggingFace Hub
    pipe.push_to_hub(
        "flux_dev_neuron_1024_tp8/", repository_id="Jingya/FLUX.1-dev-neuronx-1024x1024-tp8"  # Replace with your HF Hub repo id
    )
```

## Guidance-distilled

* The guidance-distilled variant takes about 50 sampling steps for good-quality generation.

```python
from optimum.neuron import NeuronFluxPipeline

pipe = NeuronFluxPipeline.from_pretrained("flux_dev_neuron_1024_tp8/")
prompt = "A cat holding a sign that says hello world"
out = pipe(
    prompt,
    guidance_scale=3.5,
    num_inference_steps=50,
    generator=torch.Generator("cpu").manual_seed(0)
).images[0]
out.save("flux_optimum.png")
```

## Timestep-distilled

* max_sequence_length cannot be more than 256.
* guidance_scale needs to be 0.
* As this is a timestep-distilled model, it benefits from fewer sampling steps.

```bash
optimum-cli export neuron --model black-forest-labs/FLUX.1-schnell --tensor_parallel_size 8 --batch_size 1 --height 1024 --width 1024 --num_images_per_prompt 1 --sequence_length 256 --torch_dtype bfloat16 flux_schnell_neuron_1024_tp8/
```

```python
from optimum.neuron import NeuronFluxPipeline

pipe = NeuronFluxPipeline.from_pretrained("flux_schnell_neuron_1024_tp8")
prompt = "A cat holding a sign that says hello world"
out = pipe(prompt, max_sequence_length=256, num_inference_steps=4).images[0]
```

## NeuronFluxPipeline[[optimum.neuron.NeuronFluxPipeline]]

The Flux pipeline for text-to-image generation.

#### optimum.neuron.NeuronFluxPipeline[[optimum.neuron.NeuronFluxPipeline]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_diffusion.py#L1620)

__call__optimum.neuron.NeuronFluxPipeline.__call__https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_diffusion.py#L1094[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]

## NeuronFluxInpaintPipeline[[optimum.neuron.NeuronFluxInpaintPipeline]]

The Flux pipeline for image inpainting.

#### optimum.neuron.NeuronFluxInpaintPipeline[[optimum.neuron.NeuronFluxInpaintPipeline]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_diffusion.py#L1630)

__call__optimum.neuron.NeuronFluxInpaintPipeline.__call__https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_diffusion.py#L1094[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]

With `NeuronFluxInpaintPipeline`, pass the original image and a mask of what you want to replace in the original image. Then replace the masked area with content described in a prompt.

```python
from diffusers.utils import load_image
from optimum.neuron import NeuronFluxInpaintPipeline

pipe = NeuronFluxInpaintPipeline.from_pretrained("Jingya/Flux.1-Schnell-1024x1024-neuronx-tp8")
prompt = "Face of a yellow cat, high resolution, sitting on a park bench"
img_url = "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
mask_url = "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"
source = load_image(img_url)
mask = load_image(mask_url)
images = pipe(prompt=prompt, image=source, mask_image=mask, max_sequence_length=256)
```

## NeuronFluxKontextPipeline[[optimum.neuron.NeuronFluxKontextPipeline]]

The Flux pipeline for image editing.

#### optimum.neuron.NeuronFluxKontextPipeline[[optimum.neuron.NeuronFluxKontextPipeline]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_diffusion.py#L1625)

__call__optimum.neuron.NeuronFluxKontextPipeline.__call__https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_diffusion.py#L1094[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]

With `NeuronFluxKontextPipeline`, pass the original image and a prompt describing what you want to change about the original image.

```python
from diffusers.utils import load_image
from optimum.neuron import NeuronFluxKontextPipeline

pipe = NeuronFluxKontextPipeline.from_pretrained("Jlonge4/FLUX.1-kontext-neuronx-1024x1024-tp8")
prompt = "Change the cushions in the chair from red to green"
img_url = "https://huggingface.co/datasets/Jlonge4/document_images/resolve/main/flux_optimum.png"
source = load_image(img_url)
images = pipe(prompt=prompt, image=source, guidance_scale=2.5)
```

| Image | Prompt | Output |
|:-----:|:------:|:------:|
|  | ***Change the cushions in the chair from red to green*** |  |

Are there any other diffusion features that you want us to support in 🤗`Optimum-neuron`? Please file an issue to [`Optimum-neuron` Github repo](https://github.com/huggingface/optimum-neuron) or discuss with us on [HuggingFace’s community forum](https://discuss.huggingface.co/c/optimum/), cheers 🤗 !
