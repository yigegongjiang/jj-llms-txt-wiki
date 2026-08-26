# StableAudio3DiTModel

A rectified-flow velocity-prediction Diffusion Transformer (DiT) for audio generation, used in
[Stable Audio 3](https://stability.ai/news/stable-audio-3).

Each `StableAudio3DiTBlock` performs:

1. **Self-attention** — differential multi-head attention with rotary position embeddings (RoPE).
2. **Cross-attention** — attends to the token sequence from the T5Gemma text encoder.
3. **Feed-forward** — SwiGLU projection.

The model is conditioned on a **timestep** (exponential Fourier features → linear projection) and a **global
conditioning vector** (duration embedding from [StableAudio3DurationEmbedder](/docs/diffusers/v0.40.0/en/api/pipelines/stable_audio_3#diffusers.StableAudio3DurationEmbedder)).

## StableAudio3DiTModel[[diffusers.StableAudio3DiTModel]]

#### diffusers.StableAudio3DiTModel[[diffusers.StableAudio3DiTModel]]

```python
diffusers.StableAudio3DiTModel(io_channels: int = 256, patch_size: int = 1, embed_dim: int = 1536, depth: int = 24, num_heads: int = 24, cond_token_dim: int = 768, global_cond_dim: int = 768, local_add_cond_dim: int = 257, timestep_features_dim: int = 256, ff_mult: float = 4.0, num_memory_tokens: int = 64, use_differential_attention: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_stable_audio3.py#L403)

**Parameters:**

io_channels (`int`, defaults to 256) : Number of latent channels.

patch_size (`int`, defaults to 1) : Temporal patch size applied before the transformer.

embed_dim (`int`, defaults to 1536) : Transformer hidden dimension.

depth (`int`, defaults to 24) : Number of `StableAudio3DiTBlock` layers.

num_heads (`int`, defaults to 24) : Number of attention heads.

cond_token_dim (`int`, defaults to 768) : Dimension of the cross-attention context tokens.

global_cond_dim (`int`, defaults to 768) : Dimension of the global duration embedding.

local_add_cond_dim (`int`, defaults to 257) : Channels of the local-additive (inpaint) tensor.

timestep_features_dim (`int`, defaults to 256) : Output dimension of the Fourier timestep features.

ff_mult (`float`, defaults to 4.0) : SwiGLU feed-forward expansion factor.

num_memory_tokens (`int`, defaults to 64) : Number of learnable memory tokens.

use_differential_attention (`bool`, defaults to `True`) : Enable differential self/cross attention.

The Diffusion Transformer (DiT) backbone of [Stable Audio 3](https://stability.ai/news/stable-audio-3).

The model takes a batch of noisy audio latents, a scalar timestep, a cross-attention context (projected text and
duration tokens), and a global duration embedding, and predicts the velocity field (rectified-flow objective).

Conditioning:
- Cross-attention context (`encoder_hidden_states`) is projected by `to_cond_embed`.
- The global duration embedding (`global_hidden_states`) is projected by `to_global_embed`, summed with the
  timestep embedding, then expanded by `global_cond_embedder` into the per-block AdaLN modulation.
- `local_add_cond` (inpainting) is projected per-block by `to_local_embed`.

`num_memory_tokens` learnable tokens are prepended to the audio sequence inside the transformer and removed before
the output projection.

#### forward[[diffusers.StableAudio3DiTModel.forward]]

```python
forward(hidden_states: Tensor, timestep: Tensor, encoder_hidden_states: Tensor, global_hidden_states: Tensor, encoder_attention_mask: typing.Optional[torch.Tensor] = None, local_add_cond: typing.Optional[torch.Tensor] = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_stable_audio3.py#L521)

**Parameters:**

hidden_states (`torch.Tensor`) : Noisy latent audio `(batch, io_channels, T)`.

timestep (`torch.Tensor`) : Diffusion timestep `(batch,)` in `[0, 1]`.

encoder_hidden_states (`torch.Tensor`) : Cross-attention context `(batch, T_ctx, cond_token_dim)`.

global_hidden_states (`torch.Tensor`) : Global duration embedding `(batch, global_cond_dim)`.

encoder_attention_mask (`torch.Tensor`, *optional*) : Bool mask `(batch, T_ctx)`, `True` = valid.

local_add_cond (`torch.Tensor`, *optional*) : Local-additive (inpaint) conditioning `(batch, local_add_cond_dim, T)`.

return_dict (`bool`, defaults to `True`) : Whether to return a `StableAudio3DiTModelOutput`.

**Returns:** `StableAudio3DiTModelOutput` or `tuple`

the predicted velocity field, same shape as `hidden_states`.

## StableAudio3DiTBlock[[diffusers.models.transformers.transformer_stable_audio3.StableAudio3DiTBlock]]

#### diffusers.models.transformers.transformer_stable_audio3.StableAudio3DiTBlock[[diffusers.models.transformers.transformer_stable_audio3.StableAudio3DiTBlock]]

```python
diffusers.models.transformers.transformer_stable_audio3.StableAudio3DiTBlock(dim: int, context_dim: int, dim_heads: int = 64, use_differential: bool = True, ff_mult: float = 4.0, local_add_cond_dim: int = 257)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_stable_audio3.py#L316)

Single SA3 DiT transformer block.

Order of operations:
1. AdaLN-modulated self-attention (partial RoPE, RMS QK-norm)
2. Cross-attention to the text/duration context (plain RMS pre-norm)
3. AdaLN-modulated SwiGLU feed-forward

The AdaLN modulation is `to_scale_shift_gate + global_modulation`, split into six chunks `(scale_attn, shift_attn,
gate_attn, scale_ff, shift_ff, gate_ff)`. Each gated branch is scaled by `sigmoid(1 - gate)`. Cross-attention is
*not* AdaLN-modulated, matching the reference (`cross_attend_norm` is a plain RMS norm).

When `local_seq` is provided (inpainting), it is projected per-block by `to_local_embed` and added to the audio
positions of the residual stream after cross-attention (and before the feed-forward), matching the reference.

## StableAudio3DiTModelOutput[[diffusers.models.transformers.transformer_stable_audio3.StableAudio3DiTModelOutput]]

#### diffusers.models.transformers.transformer_stable_audio3.StableAudio3DiTModelOutput[[diffusers.models.transformers.transformer_stable_audio3.StableAudio3DiTModelOutput]]

```python
diffusers.models.transformers.transformer_stable_audio3.StableAudio3DiTModelOutput(sample: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_stable_audio3.py#L34)

**Parameters:**

sample (`torch.Tensor`) : The predicted velocity field, of the same shape as the input `hidden_states`.

The output of [StableAudio3DiTModel](/docs/diffusers/v0.40.0/en/api/models/stable_audio_3_transformer#diffusers.StableAudio3DiTModel).
