# Load adapters

There are several training techniques for personalizing diffusion models to generate images of a specific subject or images in certain styles. Each of these training methods produces a different type of adapter. Some of the adapters generate an entirely new model, while other adapters only modify a smaller set of embeddings or weights. This means the loading process for each adapter is also different.

This guide will show you how to load LoRA weights.

## LoRA

Low-Rank Adaptation is the fastest way for Stable Diffusion to adapt the styles of the generated images. In Optimum Neuron, we support using one or multiple LoRA adapters by fusing their parameters into the original parameters of the text encoder(s) and the unet during the compilation. Here below is an example of compiling stable diffusion models with LoRA adapters of your choice and using the compiled artifacts to generate styled images:

```python

from diffusers import LCMScheduler
from optimum.neuron import NeuronStableDiffusionPipeline

model_id = "Lykon/dreamshaper-7"
adapter_id = "latent-consistency/lcm-lora-sdv1-5"
input_shapes = {"batch_size": 1, "height": 512, "width": 512, "num_images_per_prompt": 1}
compiler_args = {"auto_cast": "matmul", "auto_cast_type": "bf16"}

# Compile
pipe = NeuronStableDiffusionPipeline.from_pretrained(
    model_id,
    export=True,
    inline_weights_to_neff=True,  # caveat: performance drop if neff/weights separated, will be improved by a future Neuron sdk release.
    lora_model_ids=adapter_id,
    lora_weight_names="pytorch_lora_weights.safetensors",
    lora_adapter_names="lcm",
    **input_shapes,
    **compiler_args,
)
pipe.scheduler = LCMScheduler.from_config(pipe.scheduler.config)

# Save locally or upload to the HuggingFace Hub
pipe.save_pretrained("dreamshaper_7_lcm_lora_neuron/")

# Inference
prompt = "Self-portrait oil painting, a beautiful cyborg with golden hair, 8k"
image = pipe(prompt, num_inference_steps=4, guidance_scale=0).images[0]
```

Are there any other diffusion features that you want us to support in 🤗`Optimum-neuron`? Please file an issue to [`Optimum-neuron` Github repo](https://github.com/huggingface/optimum-neuron) or discuss with us on [HuggingFace’s community forum](https://discuss.huggingface.co/c/optimum/), cheers 🤗 !
