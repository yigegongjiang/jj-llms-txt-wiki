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

Anima is a text-to-image model that reuses the [CosmosTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/cosmos_transformer3d#diffusers.CosmosTransformer3DModel) with a Qwen3 text encoder, a T5-token text conditioner, and the [AutoencoderKLQwenImage](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl_qwenimage#diffusers.AutoencoderKLQwenImage) VAE.

```python
import torch
from diffusers import ModularPipeline

pipe = ModularPipeline.from_pretrained("circlestone-labs/Anima-Base-v1.0-Diffusers")
pipe.load_components(torch_dtype=torch.bfloat16)
pipe.to("cuda")

image = pipe(prompt="masterpiece, best quality, 1girl, solo, city lights").images[0]
```

## AnimaModularPipeline[[diffusers.AnimaModularPipeline]]

#### diffusers.AnimaModularPipeline[[diffusers.AnimaModularPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/anima/modular_pipeline.py#L19)

A ModularPipeline for Anima.

> [!WARNING] > This is an experimental feature and is likely to change in the future.

## AnimaAutoBlocks[[diffusers.AnimaAutoBlocks]]

#### diffusers.AnimaAutoBlocks[[diffusers.AnimaAutoBlocks]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/anima/modular_blocks_anima.py#L126)

Auto Modular pipeline for text-to-image generation using Anima.

Supported workflows:
- `text2image`: requires `prompt`

Components:
text_encoder (`Qwen3Model`) tokenizer (`Qwen2Tokenizer`) t5_tokenizer (`T5TokenizerFast`) text_conditioner
(`AnimaTextConditioner`) guider (`ClassifierFreeGuidance`) transformer (`CosmosTransformer3DModel`) scheduler
(`FlowMatchEulerDiscreteScheduler`) vae (`AutoencoderKLQwenImage`) image_processor (`VaeImageProcessor`)

Inputs:
prompt (`str`):
The prompt or prompts to guide image generation.
negative_prompt (`str`, *optional*):
The prompt or prompts not to guide the image generation.
max_sequence_length (`int`, *optional*, defaults to 512):
Maximum sequence length for prompt encoding.
num_images_per_prompt (`int`, *optional*, defaults to 1):
The number of images to generate per prompt.
height (`int`, *optional*):
The height in pixels of the generated image.
width (`int`, *optional*):
The width in pixels of the generated image.
latents (`Tensor`, *optional*):
Pre-generated noisy latents for image generation.
generator (`Generator`, *optional*):
Torch generator for deterministic generation.
num_inference_steps (`int`, *optional*, defaults to 50):
The number of denoising steps.
sigmas (`list`, *optional*):
Custom sigmas for the denoising process.
**denoiser_input_fields (`None`, *optional*):
The conditional model inputs for the Anima denoiser.
output_type (`str`, *optional*, defaults to pil):
Output format: 'pil', 'np', 'pt'.

Outputs:
images (`list`):
Generated images.

## AnimaTextConditioner[[diffusers.AnimaTextConditioner]]

#### diffusers.AnimaTextConditioner[[diffusers.AnimaTextConditioner]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/condition_embedders/condition_embedder_anima.py#L229)

Text conditioner used by Anima to map Qwen3 hidden states and T5 token ids to Cosmos text embeddings.

Anima reuses the Cosmos Predict2 DiT. The only model-specific conditioning module is this LLM adapter, which
cross-attends from learned T5 token embeddings to Qwen3 text encoder hidden states before the diffusion loop.
`target_dim` is the conditioner output dimension and must match the transformer's `text_embed_dim`.

forwarddiffusers.AnimaTextConditioner.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/condition_embedders/condition_embedder_anima.py#L285[{"name": "source_hidden_states", "val": ": Tensor"}, {"name": "target_input_ids", "val": ": Tensor"}, {"name": "target_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "source_attention_mask", "val": ": torch.Tensor | None = None"}]- **source_hidden_states** (`torch.Tensor` of shape `(batch_size, source_sequence_length, source_dim)`) --
  Qwen3 text encoder hidden states to condition on.
- **target_input_ids** (`torch.Tensor` of shape `(batch_size, target_sequence_length)`) --
  T5 token ids used as learned query tokens.
- **target_attention_mask** (`torch.Tensor`, *optional*) --
  Attention mask for the target T5 token ids.
- **source_attention_mask** (`torch.Tensor`, *optional*) --
  Attention mask for the source Qwen3 hidden states.0`torch.Tensor`Text conditioning embeddings for the Cosmos transformer.

**Parameters:**

source_hidden_states (`torch.Tensor` of shape `(batch_size, source_sequence_length, source_dim)`) : Qwen3 text encoder hidden states to condition on.

target_input_ids (`torch.Tensor` of shape `(batch_size, target_sequence_length)`) : T5 token ids used as learned query tokens.

target_attention_mask (`torch.Tensor`, *optional*) : Attention mask for the target T5 token ids.

source_attention_mask (`torch.Tensor`, *optional*) : Attention mask for the source Qwen3 hidden states.

**Returns:**

``torch.Tensor``

Text conditioning embeddings for the Cosmos transformer.
