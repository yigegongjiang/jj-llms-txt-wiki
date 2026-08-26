# GlmImageTransformer2DModel

A Diffusion Transformer model for 2D data from [GlmImageTransformer2DModel] (TODO).

## GlmImageTransformer2DModel[[diffusers.GlmImageTransformer2DModel]]

#### diffusers.GlmImageTransformer2DModel[[diffusers.GlmImageTransformer2DModel]]

```python
diffusers.GlmImageTransformer2DModel(patch_size: int = 2, in_channels: int = 16, out_channels: int = 16, num_layers: int = 30, attention_head_dim: int = 40, num_attention_heads: int = 64, text_embed_dim: int = 1472, time_embed_dim: int = 512, condition_dim: int = 256, prior_vq_quantizer_codebook_size: int = 16384)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_glm_image.py#L503)

**Parameters:**

patch_size (`int`, defaults to `2`) : The size of the patches to use in the patch embedding layer.

in_channels (`int`, defaults to `16`) : The number of channels in the input.

num_layers (`int`, defaults to `30`) : The number of layers of Transformer blocks to use.

attention_head_dim (`int`, defaults to `40`) : The number of channels in each head.

num_attention_heads (`int`, defaults to `64`) : The number of heads to use for multi-head attention.

out_channels (`int`, defaults to `16`) : The number of channels in the output.

text_embed_dim (`int`, defaults to `1472`) : Input dimension of text embeddings from the text encoder.

time_embed_dim (`int`, defaults to `512`) : Output dimension of timestep embeddings.

condition_dim (`int`, defaults to `256`) : The embedding dimension of the input SDXL-style resolution conditions (original_size, target_size, crop_coords).

pos_embed_max_size (`int`, defaults to `128`) : The maximum resolution of the positional embeddings, from which slices of shape `H x W` are taken and added to input patched latents, where `H` and `W` are the latent height and width respectively. A value of 128 means that the maximum supported height and width for image generation is `128 * vae_scale_factor * patch_size => 128 * 8 * 2 => 2048`.

sample_size (`int`, defaults to `128`) : The base resolution of input latents. If height/width is not provided during generation, this value is used to determine the resolution as `sample_size * vae_scale_factor => 128 * 8 => 1024`

#### forward[[diffusers.GlmImageTransformer2DModel.forward]]

```python
forward(hidden_states: Tensor, encoder_hidden_states: Tensor, prior_token_id: Tensor, prior_token_drop: Tensor, timestep: LongTensor, target_size: Tensor, crop_coords: Tensor, attention_kwargs: dict[str, typing.Any] | None = None, return_dict: bool = True, attention_mask: typing.Optional[torch.Tensor] = None, kv_caches: diffusers.models.transformers.transformer_glm_image.GlmImageKVCache | None = None, image_rotary_emb: tuple[torch.Tensor, torch.Tensor] | list[tuple[torch.Tensor, torch.Tensor]] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_glm_image.py#L597)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch_size, in_channels, height, width)`) : Input `hidden_states`.

encoder_hidden_states (`torch.Tensor` of shape `(batch_size, sequence_len, embed_dims)`) : Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.

prior_token_id (`torch.Tensor`) : Token ids for the prior embedding lookup.

prior_token_drop (`torch.Tensor`) : Boolean mask indicating which prior embeddings should be dropped (zeroed out).

timestep (`torch.LongTensor`) : Used to indicate denoising step.

target_size (`torch.Tensor`) : Target image size conditioning.

crop_coords (`torch.Tensor`) : Crop coordinates conditioning.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

attention_mask (`torch.Tensor`, *optional*) : Mask applied to attention scores.

kv_caches (`GlmImageKVCache`, *optional*) : Pre-computed key/value caches used to speed up inference.

image_rotary_emb (`tuple` of `torch.Tensor`, *optional*) : Pre-computed rotary positional embeddings.

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [GlmImageTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/glm_image_transformer2d#diffusers.GlmImageTransformer2DModel) forward method.
