# WanTransformer3DModel

A Diffusion Transformer model for 3D video-like data was introduced in [Wan 2.1](https://github.com/Wan-Video/Wan2.1) by the Alibaba Wan Team.

The model can be loaded with the following code snippet.

```python
from diffusers import WanTransformer3DModel

transformer = WanTransformer3DModel.from_pretrained("Wan-AI/Wan2.1-T2V-1.3B-Diffusers", subfolder="transformer", dtype=torch.bfloat16)
```

## WanTransformer3DModel[[diffusers.WanTransformer3DModel]]

#### diffusers.WanTransformer3DModel[[diffusers.WanTransformer3DModel]]

```python
diffusers.WanTransformer3DModel(patch_size: tuple = (1, 2, 2), num_attention_heads: int = 40, attention_head_dim: int = 128, in_channels: int = 16, out_channels: int = 16, text_dim: int = 4096, freq_dim: int = 256, ffn_dim: int = 13824, num_layers: int = 40, cross_attn_norm: bool = True, qk_norm: str | None = 'rms_norm_across_heads', eps: float = 1e-06, image_dim: int | None = None, added_kv_proj_dim: int | None = None, rope_max_seq_len: int = 1024, pos_embed_seq_len: int | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_wan.py#L507)

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

A Transformer model for video-like data used in the Wan model.

#### forward[[diffusers.WanTransformer3DModel.forward]]

```python
forward(hidden_states: Tensor, timestep: LongTensor, encoder_hidden_states: Tensor, encoder_hidden_states_image: typing.Optional[torch.Tensor] = None, return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_wan.py#L628)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch_size, num_channels, num_frames, height, width)`) : Input `hidden_states`.

timestep (`torch.LongTensor`) : Used to indicate denoising step.

encoder_hidden_states (`torch.Tensor` of shape `(batch_size, sequence_len, embed_dims)`) : Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.

encoder_hidden_states_image (`torch.Tensor`, *optional*) : Conditional image embeddings for image-conditioned generation.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [WanTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/wan_transformer_3d#diffusers.WanTransformer3DModel) forward method.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

```python
diffusers.models.modeling_outputs.Transformer2DModelOutput(sample: torch.Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/modeling_outputs.py#L21)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.

The output of [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel).
