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

# Anima

Anima is a text-to-image model that reuses the [CosmosTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/cosmos_transformer3d#diffusers.CosmosTransformer3DModel) with a Qwen3 text encoder, a T5-token text conditioner, and the [AutoencoderKLQwenImage](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl_qwenimage#diffusers.AutoencoderKLQwenImage) VAE.

```python
import torch
from diffusers import ModularPipeline

pipe = ModularPipeline.from_pretrained("circlestone-labs/Anima-Base-v1.0-Diffusers")
pipe.load_components(dtype=torch.bfloat16)
pipe.to("cuda")

image = pipe(prompt="masterpiece, best quality, 1girl, solo, city lights").images[0]
```

## AnimaModularPipeline[[diffusers.AnimaModularPipeline]]

#### diffusers.AnimaModularPipeline[[diffusers.AnimaModularPipeline]]

```python
diffusers.AnimaModularPipeline(blocks: diffusers.modular_pipelines.modular_pipeline.ModularPipelineBlocks | None = None, pretrained_model_name_or_path: str | os.PathLike | None = None, components_manager: diffusers.modular_pipelines.components_manager.ComponentsManager | None = None, collection: str | None = None, workflow: str | None = None, modular_config_dict: dict[str, typing.Any] | None = None, config_dict: dict[str, typing.Any] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/anima/modular_pipeline.py#L19)

A ModularPipeline for Anima.

## AnimaAutoBlocks[[diffusers.AnimaAutoBlocks]]

#### diffusers.AnimaAutoBlocks[[diffusers.AnimaAutoBlocks]]

```python
diffusers.AnimaAutoBlocks()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/anima/modular_blocks_anima.py#L312)

Auto Modular pipeline for text-to-image and image-to-image generation using Anima.

Supported workflows:
- `text2image`: requires `prompt`
- `img2img`: requires `image`, `prompt`

Components:
text_encoder (`Qwen3Model`) tokenizer (`Qwen2Tokenizer`) t5_tokenizer (`T5Tokenizer`) guider
(`ClassifierFreeGuidance`) vae (`AutoencoderKLQwenImage`) image_processor (`VaeImageProcessor`)
text_conditioner (`AnimaTextConditioner`) transformer (`CosmosTransformer3DModel`) scheduler
(`FlowMatchEulerDiscreteScheduler`)

Inputs:
prompt (`str`):
The prompt or prompts to guide image generation.
negative_prompt (`str`, *optional*):
The prompt or prompts not to guide the image generation.
max_sequence_length (`int`, *optional*, defaults to 512):
Maximum sequence length for prompt encoding.
image (`Image | list`, *optional*):
Reference image(s) for denoising. Can be a single image or list of images.
height (`int`, *optional*):
The height in pixels of the generated image.
width (`int`, *optional*):
The width in pixels of the generated image.
generator (`Generator`, *optional*):
Torch generator for deterministic generation.
num_images_per_prompt (`int`, *optional*, defaults to 1):
The number of images to generate per prompt.
image_latents (`Tensor`, *optional*):
image latents used to guide the image generation. Can be generated from vae_encoder step.
num_inference_steps (`int`):
The number of denoising steps.
sigmas (`list`, *optional*):
Custom sigmas for the denoising process.
strength (`float`, *optional*, defaults to 0.9):
Strength for img2img/inpainting.
latents (`Tensor`):
Pre-generated noisy latents for image generation.
**denoiser_input_fields (`None`, *optional*):
The conditional model inputs for the Anima denoiser.
output_type (`str`, *optional*, defaults to pil):
Output format: 'pil', 'np', 'pt'.

Outputs:
images (`list`):
Generated images.

## AnimaTextConditioner[[diffusers.AnimaTextConditioner]]

#### diffusers.AnimaTextConditioner[[diffusers.AnimaTextConditioner]]

```python
diffusers.AnimaTextConditioner(source_dim: int = 1024, target_dim: int = 1024, model_dim: int = 1024, num_layers: int = 6, num_attention_heads: int = 16, mlp_ratio: float = 4.0, target_vocab_size: int = 32128, use_self_attention: bool = True, use_layer_norm: bool = False, min_sequence_length: int = 512)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/condition_embedders/condition_embedder_anima.py#L229)

Text conditioner used by Anima to map Qwen3 hidden states and T5 token ids to Cosmos text embeddings.

Anima reuses the Cosmos Predict2 DiT. The only model-specific conditioning module is this LLM adapter, which
cross-attends from learned T5 token embeddings to Qwen3 text encoder hidden states before the diffusion loop.
`target_dim` is the conditioner output dimension and must match the transformer's `text_embed_dim`.

#### forward[[diffusers.AnimaTextConditioner.forward]]

```python
forward(source_hidden_states: Tensor, target_input_ids: Tensor, target_attention_mask: typing.Optional[torch.Tensor] = None, source_attention_mask: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/condition_embedders/condition_embedder_anima.py#L285)

**Parameters:**

source_hidden_states (`torch.Tensor` of shape `(batch_size, source_sequence_length, source_dim)`) : Qwen3 text encoder hidden states to condition on.

target_input_ids (`torch.Tensor` of shape `(batch_size, target_sequence_length)`) : T5 token ids used as learned query tokens.

target_attention_mask (`torch.Tensor`, *optional*) : Attention mask for the target T5 token ids.

source_attention_mask (`torch.Tensor`, *optional*) : Attention mask for the source Qwen3 hidden states.

**Returns:** `torch.Tensor`

Text conditioning embeddings for the Cosmos transformer.
