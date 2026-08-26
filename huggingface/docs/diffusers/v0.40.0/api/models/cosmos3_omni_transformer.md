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
    "nvidia/Cosmos3-Nano", subfolder="transformer", dtype=torch.bfloat16
)
```

## Cosmos3OmniTransformer[[diffusers.Cosmos3OmniTransformer]]

#### diffusers.Cosmos3OmniTransformer[[diffusers.Cosmos3OmniTransformer]]

```python
diffusers.Cosmos3OmniTransformer(attention_bias: bool = False, attention_dropout: float = 0.0, dtype: str = 'bfloat16', head_dim: int = 128, hidden_size: int = 4096, intermediate_size: int = 12288, base_fps: int = 24, enable_fps_modulation: bool = True, latent_channel: int = 48, unified_3d_mrope_reset_spatial_ids: bool = True, unified_3d_mrope_temporal_modality_margin: int = 15000, latent_patch_size: int = 2, num_attention_heads: int = 32, num_hidden_layers: int = 36, num_key_value_heads: int = 8, patch_latent_dim: int = 192, rms_norm_eps: float = 1e-06, rope_scaling: dict | None = None, rope_theta: float = 5000000.0, action_dim: int | None = None, action_gen: bool = False, num_embodiment_domains: int = 32, sound_dim: int | None = None, sound_gen: bool = False, sound_latent_fps: float = 25.0, timestep_scale: float = 0.001, vocab_size: int = 151936, hidden_act: str = 'silu', qk_norm_for_text: bool = True, use_und_k_norm_for_gen: bool = False, rope_axes_dim: tuple[int, int, int] | list[int] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_cosmos3.py#L373)

#### forward[[diffusers.Cosmos3OmniTransformer.forward]]

```python
forward(input_ids: Tensor, text_indexes: Tensor, position_ids: Tensor, und_len: int, sequence_length: int, vision_tokens: list, vision_token_shapes: list, vision_sequence_indexes: Tensor, vision_mse_loss_indexes: Tensor, vision_timesteps: Tensor, vision_noisy_frame_indexes: list, sound_tokens: list[torch.Tensor] | None = None, sound_token_shapes: list[tuple[int, int, int]] | None = None, sound_sequence_indexes: typing.Optional[torch.Tensor] = None, sound_mse_loss_indexes: typing.Optional[torch.Tensor] = None, sound_timesteps: typing.Optional[torch.Tensor] = None, sound_noisy_frame_indexes: list[torch.Tensor] | None = None, action_tokens: list[torch.Tensor] | None = None, action_token_shapes: list[tuple[int, int, int]] | None = None, action_sequence_indexes: typing.Optional[torch.Tensor] = None, action_mse_loss_indexes: typing.Optional[torch.Tensor] = None, action_timesteps: typing.Optional[torch.Tensor] = None, action_noisy_frame_indexes: list[torch.Tensor] | None = None, action_domain_ids: list[torch.Tensor] | None = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_cosmos3.py#L654)

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

return_dict : Whether to return a [*Cosmos3OmniTransformerOutput*] instead of a tuple.

**Returns:**

A [*Cosmos3OmniTransformerOutput*] or a tuple of per-modality prediction lists. Optional modalities return
`None` when their inputs are omitted.

Run a full denoising-step forward pass.
