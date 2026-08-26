# WanAnimateTransformer3DModel

A Diffusion Transformer model for 3D video-like data was introduced in [Wan Animate](https://github.com/Wan-Video/Wan2.2) by the Alibaba Wan Team.

The model can be loaded with the following code snippet.

```python
from diffusers import WanAnimateTransformer3DModel

transformer = WanAnimateTransformer3DModel.from_pretrained("Wan-AI/Wan2.2-Animate-14B-Diffusers", subfolder="transformer", dtype=torch.bfloat16)
```

## WanAnimateTransformer3DModel[[diffusers.WanAnimateTransformer3DModel]]

#### diffusers.WanAnimateTransformer3DModel[[diffusers.WanAnimateTransformer3DModel]]

```python
diffusers.WanAnimateTransformer3DModel(patch_size: tuple = (1, 2, 2), num_attention_heads: int = 40, attention_head_dim: int = 128, in_channels: int | None = 36, latent_channels: int | None = 16, out_channels: int | None = 16, text_dim: int = 4096, freq_dim: int = 256, ffn_dim: int = 13824, num_layers: int = 40, cross_attn_norm: bool = True, qk_norm: str | None = 'rms_norm_across_heads', eps: float = 1e-06, image_dim: int | None = 1280, added_kv_proj_dim: int | None = None, rope_max_seq_len: int = 1024, pos_embed_seq_len: int | None = None, motion_encoder_channel_sizes: dict[str, int] | None = None, motion_encoder_size: int = 512, motion_style_dim: int = 512, motion_dim: int = 20, motion_encoder_dim: int = 512, face_encoder_hidden_dim: int = 1024, face_encoder_num_heads: int = 4, inject_face_latents_blocks: int = 5, motion_encoder_batch_size: int = 8)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_wan_animate.py#L986)

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

image_dim (`int`, *optional*, defaults to `1280`) : The number of channels to use for the image embedding. If `None`, no projection is used.

added_kv_proj_dim (`int`, *optional*, defaults to `5120`) : The number of channels to use for the added key and value projections. If `None`, no projection is used.

A Transformer model for video-like data used in the WanAnimate model.

#### forward[[diffusers.WanAnimateTransformer3DModel.forward]]

```python
forward(hidden_states: Tensor, timestep: LongTensor, encoder_hidden_states: Tensor, encoder_hidden_states_image: typing.Optional[torch.Tensor] = None, pose_hidden_states: typing.Optional[torch.Tensor] = None, face_pixel_values: typing.Optional[torch.Tensor] = None, motion_encode_batch_size: int | None = None, return_dict: bool = True, attention_kwargs: dict[str, typing.Any] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_wan_animate.py#L1154)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(B, 2C + 4, T + 1, H, W)`) : Input noisy video latents of shape `(B, 2C + 4, T + 1, H, W)`, where B is the batch size, C is the number of latent channels (16 for Wan VAE), T is the number of latent frames in an inference segment, H is the latent height, and W is the latent width.

timestep : (`torch.LongTensor`): The current timestep in the denoising loop.

encoder_hidden_states (`torch.Tensor`) : Text embeddings from the text encoder (umT5 for Wan Animate).

encoder_hidden_states_image (`torch.Tensor`) : CLIP visual features of the reference (character) image.

pose_hidden_states (`torch.Tensor` of shape `(B, C, T, H, W)`) : Pose video latents. TODO: description

face_pixel_values (`torch.Tensor` of shape `(B, C', S, H', W')`) : Face video in pixel space (not latent space). Typically C' = 3 and H' and W' are the height/width of the face video in pixels. Here S is the inference segment length, usually set to 77.

motion_encode_batch_size (`int`, *optional*) : The batch size for batched encoding of the face video via the motion encoder. Will default to `self.config.motion_encoder_batch_size` if not set.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return the output as a dict or tuple.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

**Returns:** `~models.transformer_2d.Transformer2DModelOutput` or `tuple`

If `return_dict` is True, a `~models.transformer_2d.Transformer2DModelOutput` whose `sample` is the
denoised video latent is returned, otherwise a plain `tuple` whose first element is that tensor is
returned.

Forward pass of Wan2.2-Animate transformer model.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

```python
diffusers.models.modeling_outputs.Transformer2DModelOutput(sample: torch.Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/modeling_outputs.py#L21)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.

The output of [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel).
