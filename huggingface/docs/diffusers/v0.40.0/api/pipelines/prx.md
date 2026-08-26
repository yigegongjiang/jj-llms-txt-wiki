#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License. -->

# PRX

PRX generates high-quality images from text using a simplified MMDIT architecture where text tokens don't update through transformer blocks. It employs flow matching with discrete scheduling for efficient sampling and uses Google's T5Gemma-2B-2B-UL2 model for multi-language text encoding. The ~1.3B parameter transformer delivers fast inference without sacrificing quality. You can choose between Flux VAE (8x compression, 16 latent channels) for balanced quality and speed or DC-AE (32x compression, 32 latent channels) for latent compression and faster processing.

## Available models

PRX offers multiple variants with different VAE configurations, each optimized for specific resolutions. Base models excel with detailed prompts, capturing complex compositions and subtle details. Fine-tuned models trained on the [Alchemist dataset](https://huggingface.co/datasets/yandex/alchemist) improve aesthetic quality, especially with simpler prompts.

| Model | Resolution | Fine-tuned | Distilled | Description | Suggested prompts | Suggested parameters | Recommended dtype |
|:-----:|:-----------------:|:----------:|:----------:|:----------:|:----------:|:----------:|:----------:|
| [`Photoroom/prx-256-t2i`](https://huggingface.co/Photoroom/prx-256-t2i)| 256 | No | No | Base model pre-trained at 256 with Flux VAE|Works best with detailed prompts in natural language|28 steps, cfg=5.0| `torch.bfloat16` |
| [`Photoroom/prx-256-t2i-sft`](https://huggingface.co/Photoroom/prx-256-t2i-sft)| 512 | Yes | No | Fine-tuned on the [Alchemist dataset](https://huggingface.co/datasets/yandex/alchemist) dataset with Flux VAE | Can handle less detailed prompts|28 steps, cfg=5.0| `torch.bfloat16` |
| [`Photoroom/prx-512-t2i`](https://huggingface.co/Photoroom/prx-512-t2i)| 512 | No | No | Base model pre-trained at 512 with Flux VAE |Works best with detailed prompts in natural language|28 steps, cfg=5.0| `torch.bfloat16` |
| [`Photoroom/prx-512-t2i-sft`](https://huggingface.co/Photoroom/prx-512-t2i-sft)| 512 | Yes | No | Fine-tuned on the [Alchemist dataset](https://huggingface.co/datasets/yandex/alchemist) dataset with Flux VAE | Can handle less detailed prompts in natural language|28 steps, cfg=5.0| `torch.bfloat16` |
| [`Photoroom/prx-512-t2i-sft-distilled`](https://huggingface.co/Photoroom/prx-512-t2i-sft-distilled)| 512 | Yes | Yes | 8-step distilled model from [`Photoroom/prx-512-t2i-sft`](https://huggingface.co/Photoroom/prx-512-t2i-sft) | Can handle less detailed prompts in natural language|8 steps, cfg=1.0| `torch.bfloat16` |
| [`Photoroom/prx-512-t2i-dc-ae`](https://huggingface.co/Photoroom/prx-512-t2i-dc-ae)| 512 | No | No | Base model pre-trained at 512 with [Deep Compression Autoencoder (DC-AE)](https://hanlab.mit.edu/projects/dc-ae)|Works best with detailed prompts in natural language|28 steps, cfg=5.0| `torch.bfloat16` |
| [`Photoroom/prx-512-t2i-dc-ae-sft`](https://huggingface.co/Photoroom/prx-512-t2i-dc-ae-sft)| 512 | Yes | No | Fine-tuned on the [Alchemist dataset](https://huggingface.co/datasets/yandex/alchemist) dataset with [Deep Compression Autoencoder (DC-AE)](https://hanlab.mit.edu/projects/dc-ae) | Can handle less detailed prompts in natural language|28 steps, cfg=5.0| `torch.bfloat16` |
| [`Photoroom/prx-512-t2i-dc-ae-sft-distilled`](https://huggingface.co/Photoroom/prx-512-t2i-dc-ae-sft-distilled)| 512 | Yes | Yes | 8-step distilled model from [`Photoroom/prx-512-t2i-dc-ae-sft-distilled`](https://huggingface.co/Photoroom/prx-512-t2i-dc-ae-sft-distilled) | Can handle less detailed prompts in natural language|8 steps, cfg=1.0| `torch.bfloat16` |s

Refer to [this](https://huggingface.co/collections/Photoroom/prx-models-68e66254c202ebfab99ad38e) collection for more information.

## Loading the pipeline

Load the pipeline with [from_pretrained()](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline.from_pretrained).

```py
from diffusers.pipelines.prx import PRXPipeline

# Load pipeline - VAE and text encoder will be loaded from HuggingFace
pipe = PRXPipeline.from_pretrained("Photoroom/prx-512-t2i-sft", dtype=torch.bfloat16)
pipe.to("cuda")

prompt = "A front-facing portrait of a lion the golden savanna at sunset."
image = pipe(prompt, num_inference_steps=28, guidance_scale=5.0).images[0]
image.save("prx_output.png")
```

### Manual Component Loading

Load components individually to customize the pipeline for instance to use quantized models.

```py
import torch
from diffusers.pipelines.prx import PRXPipeline
from diffusers.models import AutoencoderKL, AutoencoderDC
from diffusers.models.transformers.transformer_prx import PRXTransformer2DModel
from diffusers.schedulers import FlowMatchEulerDiscreteScheduler
from transformers import T5GemmaModel, GemmaTokenizerFast
from diffusers import BitsAndBytesConfig as DiffusersBitsAndBytesConfig
from transformers import BitsAndBytesConfig as BitsAndBytesConfig

quant_config = DiffusersBitsAndBytesConfig(load_in_8bit=True)
# Load transformer
transformer = PRXTransformer2DModel.from_pretrained(
    "checkpoints/prx-512-t2i-sft",
    subfolder="transformer",
    quantization_config=quant_config,
    dtype=torch.bfloat16,
)

# Load scheduler
scheduler = FlowMatchEulerDiscreteScheduler.from_pretrained(
    "checkpoints/prx-512-t2i-sft", subfolder="scheduler"
)

# Load T5Gemma text encoder
t5gemma_model = T5GemmaModel.from_pretrained("google/t5gemma-2b-2b-ul2",
                                            quantization_config=quant_config,
                                            dtype=torch.bfloat16)
text_encoder = t5gemma_model.encoder.to(dtype=torch.bfloat16)
tokenizer = GemmaTokenizerFast.from_pretrained("google/t5gemma-2b-2b-ul2")
tokenizer.model_max_length = 256

# Load VAE - choose either Flux VAE or DC-AE
# Flux VAE
vae = AutoencoderKL.from_pretrained("black-forest-labs/FLUX.1-dev",
                                    subfolder="vae",
                                    quantization_config=quant_config,
                                    dtype=torch.bfloat16)

pipe = PRXPipeline(
    transformer=transformer,
    scheduler=scheduler,
    text_encoder=text_encoder,
    tokenizer=tokenizer,
    vae=vae
)
pipe.to("cuda")
```

## Memory Optimization

For memory-constrained environments:

```py
import torch
from diffusers.pipelines.prx import PRXPipeline

pipe = PRXPipeline.from_pretrained("Photoroom/prx-512-t2i-sft", dtype=torch.bfloat16)
pipe.enable_model_cpu_offload()  # Offload components to CPU when not in use

# Or use sequential CPU offload for even lower memory
pipe.enable_sequential_cpu_offload()
```

## PRXPipeline[[diffusers.PRXPipeline]]

#### diffusers.PRXPipeline[[diffusers.PRXPipeline]]

```python
diffusers.PRXPipeline(transformer: PRXTransformer2DModel, scheduler: FlowMatchEulerDiscreteScheduler, text_encoder: T5GemmaEncoder, tokenizer: transformers.models.t5.tokenization_t5.T5Tokenizer | transformers.models.gemma.tokenization_gemma.GemmaTokenizer | transformers.models.auto.tokenization_auto.AutoTokenizer, vae: diffusers.models.autoencoders.autoencoder_kl.AutoencoderKL | diffusers.models.autoencoders.autoencoder_dc.AutoencoderDC | None = None, default_sample_size: int | None = 512)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/prx/pipeline_prx.py#L257)

**Parameters:**

transformer (`PRXTransformer2DModel`) : The PRX transformer model to denoise the encoded image latents.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded image latents.

text_encoder (`T5GemmaEncoder`) : Text encoder model for encoding prompts.

tokenizer ([`T5TokenizerFast` or `GemmaTokenizerFast`]) : Tokenizer for the text encoder.

vae ([AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL) or [AutoencoderDC](/docs/diffusers/v0.40.0/en/api/models/autoencoder_dc#diffusers.AutoencoderDC)) : Variational Auto-Encoder (VAE) Model to encode and decode images to and from latent representations. Supports both AutoencoderKL (8x compression) and AutoencoderDC (32x compression).

Pipeline for text-to-image generation using PRX Transformer.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods the
library implements for all the pipelines (such as downloading or saving, running on a particular device, etc.)

#### __call__[[diffusers.PRXPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, negative_prompt: str = '', height: int | None = None, width: int | None = None, num_inference_steps: int = 28, timesteps: list = None, guidance_scale: float = 4.0, num_images_per_prompt: int | None = 1, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.FloatTensor] = None, negative_prompt_embeds: typing.Optional[torch.FloatTensor] = None, prompt_attention_mask: typing.Optional[torch.BoolTensor] = None, negative_prompt_attention_mask: typing.Optional[torch.BoolTensor] = None, output_type: str | None = 'pil', return_dict: bool = True, use_resolution_binning: bool = True, callback_on_step_end: typing.Optional[typing.Callable[[int, int], NoneType]] = None, callback_on_step_end_tensor_inputs: list = ['latents'], tokenizer_max_length: int | None = None, skip_text_cleaning: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/prx/pipeline_prx.py#L554)

**Parameters:**

prompt (*str* or *list[str]*, *optional*) : The prompt or prompts to guide the image generation. If not defined, one has to pass *prompt_embeds* instead.

negative_prompt (*str*, *optional*, defaults to *""*) : The prompt or prompts not to guide the image generation. Ignored when not using guidance (i.e., ignored if *guidance_scale* is less than *1*).

height (*int*, *optional*, defaults to self.transformer.config.sample_size * self.vae_scale_factor) : The height in pixels of the generated image.

width (*int*, *optional*, defaults to self.transformer.config.sample_size * self.vae_scale_factor) : The width in pixels of the generated image.

num_inference_steps (*int*, *optional*, defaults to 28) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

timesteps (*list[int]*, *optional*) : Custom timesteps to use for the denoising process with schedulers which support a *timesteps* argument in their *set_timesteps* method. If not defined, the default behavior when *num_inference_steps* is passed will be used. Must be in descending order.

guidance_scale (*float*, *optional*, defaults to 4.0) : Guidance scale as defined in [Classifier-Free Diffusion Guidance](https://huggingface.co/papers/2207.12598). *guidance_scale* is defined as *w* of equation 2. of [Imagen Paper](https://huggingface.co/papers/2205.11487). Guidance scale is enabled by setting *guidance_scale > 1*. Higher guidance scale encourages to generate images that are closely linked to the text *prompt*, usually at the expense of lower image quality.

num_images_per_prompt (*int*, *optional*, defaults to 1) : The number of images to generate per prompt.

generator (*torch.Generator* or *list[torch.Generator]*, *optional*) : One or a list of [torch generator(s)](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (*torch.Tensor*, *optional*) : Pre-generated noisy latents, sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor will be generated by sampling using the supplied random *generator*.

prompt_embeds (*torch.FloatTensor*, *optional*) : Pre-generated text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from *prompt* input argument.

negative_prompt_embeds (*torch.FloatTensor*, *optional*) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided and *guidance_scale > 1*, negative embeddings will be generated from an empty string.

prompt_attention_mask (*torch.BoolTensor*, *optional*) : Pre-generated attention mask for *prompt_embeds*. If not provided, attention mask will be generated from *prompt* input argument.

negative_prompt_attention_mask (*torch.BoolTensor*, *optional*) : Pre-generated attention mask for *negative_prompt_embeds*. If not provided and *guidance_scale > 1*, attention mask will be generated from an empty string.

output_type (*str*, *optional*, defaults to *"pil"*) : The output format of the generate image. Choose between [PIL](https://pillow.readthedocs.io/en/stable/): *PIL.Image.Image* or *np.array*.

tokenizer_max_length (*int*, *optional*) : Override the maximum number of tokens used when tokenizing the prompt. Defaults to the tokenizer's own `model_max_length` when not set.

skip_text_cleaning (*bool*, *optional*, defaults to *False*) : If *True*, uses only light prompt cleaning (fix encoding + unescape HTML) instead of the full DeepFloyd cleaning pipeline.

return_dict (*bool*, *optional*, defaults to *True*) : Whether or not to return a [*~pipelines.prx.PRXPipelineOutput*] instead of a plain tuple.

use_resolution_binning (*bool*, *optional*, defaults to *True*) : If set to *True*, the requested height and width are first mapped to the closest resolutions using predefined aspect ratio bins. After the produced latents are decoded into images, they are resized back to the requested resolution. Useful for generating non-square images at optimal resolutions.

callback_on_step_end (*Callable*, *optional*) : A function that calls at the end of each denoising steps during the inference. The function is called with the following arguments: *callback_on_step_end(self, step, timestep, callback_kwargs)*. *callback_kwargs* will include a list of all tensors as specified by *callback_on_step_end_tensor_inputs*.

callback_on_step_end_tensor_inputs (*list*, *optional*) : The list of tensor inputs for the *callback_on_step_end* function. The tensors specified in the list will be passed as *callback_kwargs* argument. You will only be able to include tensors that are listed in the *._callback_tensor_inputs* attribute.

**Returns:** [*~pipelines.prx.PRXPipelineOutput*] or *tuple`

[*~pipelines.prx.PRXPipelineOutput*] if *return_dict* is
True, otherwise a *tuple. When returning a tuple, the first element is a list with the generated images.

Function invoked when calling the pipeline for generation.

Examples:
```py
>>> import torch
>>> from diffusers import PRXPipeline

>>> # Load pipeline with from_pretrained
>>> pipe = PRXPipeline.from_pretrained("Photoroom/prx-512-t2i-sft")
>>> pipe.to("cuda")

>>> prompt = "A digital painting of a rusty, vintage tram on a sandy beach"
>>> image = pipe(prompt, num_inference_steps=28, guidance_scale=5.0).images[0]
>>> image.save("prx_output.png")
```

#### check_inputs[[diffusers.PRXPipeline.check_inputs]]

```python
check_inputs(prompt: str | list[str], height: int, width: int, guidance_scale: float, callback_on_step_end_tensor_inputs: list[str] | None = None, prompt_embeds: typing.Optional[torch.FloatTensor] = None, negative_prompt_embeds: typing.Optional[torch.FloatTensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/prx/pipeline_prx.py#L500)

Check that all inputs are in correct format.

#### encode_prompt[[diffusers.PRXPipeline.encode_prompt]]

```python
encode_prompt(prompt: str | list[str], device: typing.Optional[torch.device] = None, do_classifier_free_guidance: bool = True, negative_prompt: str = '', num_images_per_prompt: int = 1, prompt_embeds: typing.Optional[torch.FloatTensor] = None, negative_prompt_embeds: typing.Optional[torch.FloatTensor] = None, prompt_attention_mask: typing.Optional[torch.BoolTensor] = None, negative_prompt_attention_mask: typing.Optional[torch.BoolTensor] = None, tokenizer_max_length: int | None = None, skip_text_cleaning: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/prx/pipeline_prx.py#L376)

Encode text prompt using standard text encoder and tokenizer, or use precomputed embeddings.

#### get_default_resolution[[diffusers.PRXPipeline.get_default_resolution]]

```python
get_default_resolution()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/prx/pipeline_prx.py#L340)

**Returns:** `int`

The default sample size (height/width) to use for generation.

Determine the default resolution based on the loaded VAE and config.

#### prepare_latents[[diffusers.PRXPipeline.prepare_latents]]

```python
prepare_latents(batch_size: int, num_channels_latents: int, height: int, width: int, dtype: dtype, device: device, generator: typing.Optional[torch.Generator] = None, latents: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/prx/pipeline_prx.py#L352)

Prepare initial latents for the diffusion process.

## PRXPipelineOutput[[diffusers.pipelines.prx.PRXPipelineOutput]]

#### diffusers.pipelines.prx.PRXPipelineOutput[[diffusers.pipelines.prx.PRXPipelineOutput]]

```python
diffusers.pipelines.prx.PRXPipelineOutput(images: list[PIL.Image.Image] | numpy.ndarray)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/prx/pipeline_output.py#L24)

**Parameters:**

images (`list[PIL.Image.Image]` or `np.ndarray`) : list of denoised PIL images of length `batch_size` or numpy array of shape `(batch_size, height, width, num_channels)`. PIL images or numpy array present the denoised images of the diffusion pipeline.

Output class for PRX pipelines.
