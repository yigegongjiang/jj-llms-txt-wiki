# Cosmos3OmniTransformer

A Mixture-of-Transformer (MoT) joint vision-language transformer introduced as part of NVIDIA's Cosmos3 world foundation model family. The model runs two parallel computation pathways over a packed joint sequence:

- a **causal "understanding" pathway** that self-attends over text tokens with causal masking, and
- a **bi-directional "generation" pathway** that cross-attends from generation tokens (vision + optional sound latents) over the full understanding-plus-generation key/value set.

The two pathways share the same hidden size and number of layers but maintain **separate Q/K/V/O projections, MLPs, and RMSNorm parameters**, which is what makes the architecture a Mixture-of-Transformer rather than a standard Mixture-of-Experts. Position information is supplied through a 3D multimodal RoPE (mRoPE) that interleaves temporal / height / width frequencies for video latents and reuses the temporal axis for text and audio.

The model can be loaded as follows.

```python
import torch
from diffusers import Cosmos3OmniTransformer

transformer = Cosmos3OmniTransformer.from_pretrained(
    "nvidia/Cosmos3-Nano", subfolder="transformer", torch_dtype=torch.bfloat16
)
```

## Cosmos3OmniTransformer[[diffusers.Cosmos3OmniTransformer]]

#### diffusers.Cosmos3OmniTransformer[[diffusers.Cosmos3OmniTransformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_cosmos3.py#L297)

forwarddiffusers.Cosmos3OmniTransformer.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_cosmos3.py#L554[{"name": "input_ids", "val": ": Tensor"}, {"name": "text_indexes", "val": ": Tensor"}, {"name": "position_ids", "val": ": Tensor"}, {"name": "und_len", "val": ": int"}, {"name": "sequence_length", "val": ": int"}, {"name": "vision_tokens", "val": ": list"}, {"name": "vision_token_shapes", "val": ": list"}, {"name": "vision_sequence_indexes", "val": ": Tensor"}, {"name": "vision_mse_loss_indexes", "val": ": Tensor"}, {"name": "vision_timesteps", "val": ": Tensor"}, {"name": "vision_noisy_frame_indexes", "val": ": list"}, {"name": "sound_tokens", "val": ": list[torch.Tensor] | None = None"}, {"name": "sound_token_shapes", "val": ": list[tuple[int, int, int]] | None = None"}, {"name": "sound_sequence_indexes", "val": ": torch.Tensor | None = None"}, {"name": "sound_mse_loss_indexes", "val": ": torch.Tensor | None = None"}, {"name": "sound_timesteps", "val": ": torch.Tensor | None = None"}, {"name": "sound_noisy_frame_indexes", "val": ": list[torch.Tensor] | None = None"}, {"name": "action_tokens", "val": ": list[torch.Tensor] | None = None"}, {"name": "action_token_shapes", "val": ": list[tuple[int, int, int]] | None = None"}, {"name": "action_sequence_indexes", "val": ": torch.Tensor | None = None"}, {"name": "action_mse_loss_indexes", "val": ": torch.Tensor | None = None"}, {"name": "action_timesteps", "val": ": torch.Tensor | None = None"}, {"name": "action_noisy_frame_indexes", "val": ": list[torch.Tensor] | None = None"}, {"name": "action_domain_ids", "val": ": list[torch.Tensor] | None = None"}]- **input_ids** -- Text token IDs placed at `text_indexes` in the joint sequence.
- **text_indexes** -- Indices of text tokens in the joint sequence.
- **position_ids** -- `[3, sequence_length]` mRoPE position IDs for the full joint sequence.
- **und_len** -- Length of the causal text (understanding) prefix; generation tokens follow.
- **sequence_length** -- Total length of the joint packed sequence.
- **vision_tokens** -- Per-item vision latent tensors before patchify.
- **vision_token_shapes** -- Patch grid shapes `(T, H, W)` per vision item.
- **vision_sequence_indexes** -- Indices of vision tokens in the joint sequence.
- **vision_mse_loss_indexes** -- Indices used to read vision predictions after the backbone.
- **vision_timesteps** -- Per-patch diffusion timesteps for vision tokens.
- **vision_noisy_frame_indexes** -- Noisy frame indices per vision item.
- **sound_tokens** -- Optional sound latent tensors before packing.
- **sound_token_shapes** -- Optional patch grid shapes for sound items.
- **sound_sequence_indexes** -- Optional indices of sound tokens in the joint sequence.
- **sound_mse_loss_indexes** -- Optional indices used to read sound predictions.
- **sound_timesteps** -- Optional per-token diffusion timesteps for sound.
- **sound_noisy_frame_indexes** -- Optional noisy frame indices per sound item.
- **action_tokens** -- Optional action latent tensors before packing.
- **action_token_shapes** -- Optional patch grid shapes `(T, H, W)` per action item.
- **action_sequence_indexes** -- Optional indices of action tokens in the joint sequence.
- **action_mse_loss_indexes** -- Optional indices used to read action predictions after the backbone.
- **action_timesteps** -- Optional per-token diffusion timesteps for action tokens.
- **action_noisy_frame_indexes** -- Optional noisy frame indices per action item.
- **action_domain_ids** -- Optional per-item domain IDs selecting the action head weights.0`(preds_vision, preds_sound, preds_action)` — lists of per-modality predictions. Optional modalities
return `None` when their inputs are omitted.
Run a full denoising-step forward pass.

**Parameters:**

input_ids : Text token IDs placed at `text_indexes` in the joint sequence.

text_indexes : Indices of text tokens in the joint sequence.

position_ids : `[3, sequence_length]` mRoPE position IDs for the full joint sequence.

und_len : Length of the causal text (understanding) prefix; generation tokens follow.

sequence_length : Total length of the joint packed sequence.

vision_tokens : Per-item vision latent tensors before patchify.

vision_token_shapes : Patch grid shapes `(T, H, W)` per vision item.

vision_sequence_indexes : Indices of vision tokens in the joint sequence.

vision_mse_loss_indexes : Indices used to read vision predictions after the backbone.

vision_timesteps : Per-patch diffusion timesteps for vision tokens.

vision_noisy_frame_indexes : Noisy frame indices per vision item.

sound_tokens : Optional sound latent tensors before packing.

sound_token_shapes : Optional patch grid shapes for sound items.

sound_sequence_indexes : Optional indices of sound tokens in the joint sequence.

sound_mse_loss_indexes : Optional indices used to read sound predictions.

sound_timesteps : Optional per-token diffusion timesteps for sound.

sound_noisy_frame_indexes : Optional noisy frame indices per sound item.

action_tokens : Optional action latent tensors before packing.

action_token_shapes : Optional patch grid shapes `(T, H, W)` per action item.

action_sequence_indexes : Optional indices of action tokens in the joint sequence.

action_mse_loss_indexes : Optional indices used to read action predictions after the backbone.

action_timesteps : Optional per-token diffusion timesteps for action tokens.

action_noisy_frame_indexes : Optional noisy frame indices per action item.

action_domain_ids : Optional per-item domain IDs selecting the action head weights.

**Returns:**

`(preds_vision, preds_sound, preds_action)` — lists of per-modality predictions. Optional modalities
return `None` when their inputs are omitted.
