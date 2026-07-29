# Intel Gaudi

The Intel Gaudi AI accelerator family includes [Intel Gaudi 1](https://habana.ai/products/gaudi/), [Intel Gaudi 2](https://habana.ai/products/gaudi2/), and [Intel Gaudi 3](https://habana.ai/products/gaudi3/). Each server is equipped with 8 devices, known as Habana Processing Units (HPUs), providing 128GB of memory on Gaudi 3, 96GB on Gaudi 2, and 32GB on the first-gen Gaudi. For more details on the underlying hardware architecture, check out the [Gaudi Architecture](https://docs.habana.ai/en/latest/Gaudi_Overview/Gaudi_Architecture.html) overview.

Diffusers pipelines can take advantage of HPU acceleration, even if a pipeline hasn't been added to [Optimum for Intel Gaudi](https://huggingface.co/docs/optimum/main/en/habana/index) yet, with the [GPU Migration Toolkit](https://docs.habana.ai/en/latest/PyTorch/PyTorch_Model_Porting/GPU_Migration_Toolkit/GPU_Migration_Toolkit.html).

Call `.to("hpu")` on your pipeline to move it to a HPU device as shown below for Flux:
```py
import torch
from diffusers import DiffusionPipeline

pipeline = DiffusionPipeline.from_pretrained("black-forest-labs/FLUX.1-schnell", torch_dtype=torch.bfloat16)
pipeline.to("hpu")

image = pipeline("An image of a squirrel in Picasso style").images[0]
```

> [!TIP]
> For Gaudi-optimized diffusion pipeline implementations, we recommend using [Optimum for Intel Gaudi](https://huggingface.co/docs/optimum/main/en/habana/index).
