# HeliosTransformer3DModel

A 14B Real-Time Autogressive Diffusion Transformer model (support T2V, I2V and V2V) for 3D video-like data from [Helios](https://github.com/PKU-YuanGroup/Helios) was introduced in [Helios: Real Real-Time Long Video Generation Model](https://huggingface.co/papers/2603.04379) by Peking University & ByteDance & etc.

The model can be loaded with the following code snippet.

```python
from diffusers import HeliosTransformer3DModel

# Best Quality
transformer = HeliosTransformer3DModel.from_pretrained("BestWishYsh/Helios-Base", subfolder="transformer", dtype=torch.bfloat16)
# Intermediate Weight
transformer = HeliosTransformer3DModel.from_pretrained("BestWishYsh/Helios-Mid", subfolder="transformer", dtype=torch.bfloat16)
# Best Efficiency
transformer = HeliosTransformer3DModel.from_pretrained("BestWishYsh/Helios-Distilled", subfolder="transformer", dtype=torch.bfloat16)
```

## HeliosTransformer3DModel[[diffusers.HeliosTransformer3DModel]]

#### diffusers.HeliosTransformer3DModel[[diffusers.HeliosTransformer3DModel]]

```python
diffusers.HeliosTransformer3DModel(patch_size: tuple = (1, 2, 2), num_attention_heads: int = 40, attention_head_dim: int = 128, in_channels: int = 16, out_channels: int = 16, text_dim: int = 4096, freq_dim: int = 256, ffn_dim: int = 13824, num_layers: int = 40, cross_attn_norm: bool = True, qk_norm: str | None = 'rms_norm_across_heads', eps: float = 1e-06, added_kv_proj_dim: int | None = None, rope_dim: tuple = (44, 42, 42), rope_theta: float = 10000.0, guidance_cross_attn: bool = True, zero_history_timestep: bool = True, has_multi_term_memory_patch: bool = True, is_amplify_history: bool = False, history_scale_mode: str = 'per_head')
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_helios.py#L501)

**Parameters:**

patch_size (`tuple[int]`, defaults to `(1, 2, 2)`) : 3D patch dimensions for video embedding (t_patch, h_patch, w_patch).

num_attention_heads (`int`, defaults to `40`) : Fixed length for text embeddings.

attention_head_dim (`int`, defaults to `128`) : The number of channels in each head.

in_channels (`int`, defaults to `16`) : The number of channels in the input.

out_channels (`int`, defaults to `16`) : The number of channels in the output.

text_dim (`int`, defaults to `512`) : Input dimension for text embeddings.

freq_dim (`int`, defaults to `256`) : Dimension for sinusoidal time embeddings.

ffn_dim (`int`, defaults to `13824`) : Intermediate dimension in feed-forward network.

num_layers (`int`, defaults to `40`) : The number of layers of transformer blocks to use.

window_size (`tuple[int]`, defaults to `(-1, -1)`) : Window size for local attention (-1 indicates global attention).

cross_attn_norm (`bool`, defaults to `True`) : Enable cross-attention normalization.

qk_norm (`bool`, defaults to `True`) : Enable query/key normalization.

eps (`float`, defaults to `1e-6`) : Epsilon value for normalization layers.

add_img_emb (`bool`, defaults to `False`) : Whether to use img_emb.

added_kv_proj_dim (`int`, *optional*, defaults to `None`) : The number of channels to use for the added key and value projections. If `None`, no projection is used.

A Transformer model for video-like data used in the Helios model.

#### forward[[diffusers.HeliosTransformer3DModel.forward]]

```python
forward(hidden_states: Tensor, timestep: LongTensor, encoder_hidden_states: Tensor, indices_hidden_states = None, indices_latents_history_short = None, indices_latents_history_mid = None, indices_latents_history_long = None, latents_history_short = None, latents_history_mid = None, latents_history_long = None, return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_helios.py#L661)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch_size, num_channels, num_frames, height, width)`) : Input `hidden_states`.

timestep (`torch.LongTensor`) : Used to indicate denoising step.

encoder_hidden_states (`torch.Tensor` of shape `(batch_size, sequence_len, embed_dims)`) : Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.

indices_hidden_states (`torch.Tensor`, *optional*) : Frame indices for `hidden_states` used to compute the rotary positional embeddings.

indices_latents_history_short (`torch.Tensor`, *optional*) : Frame indices for the short history latents.

indices_latents_history_mid (`torch.Tensor`, *optional*) : Frame indices for the mid history latents.

indices_latents_history_long (`torch.Tensor`, *optional*) : Frame indices for the long history latents.

latents_history_short (`torch.Tensor`, *optional*) : Short history latents conditioning.

latents_history_mid (`torch.Tensor`, *optional*) : Mid history latents conditioning.

latents_history_long (`torch.Tensor`, *optional*) : Long history latents conditioning.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [HeliosTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/helios_transformer3d#diffusers.HeliosTransformer3DModel) forward method.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

```python
diffusers.models.modeling_outputs.Transformer2DModelOutput(sample: torch.Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/modeling_outputs.py#L21)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.

The output of [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel).
