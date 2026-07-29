# PixArtTransformer2DModel

A Transformer model for image-like data from [PixArt-Alpha](https://huggingface.co/papers/2310.00426) and [PixArt-Sigma](https://huggingface.co/papers/2403.04692).

## PixArtTransformer2DModel[[diffusers.PixArtTransformer2DModel]]

#### diffusers.PixArtTransformer2DModel[[diffusers.PixArtTransformer2DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/pixart_transformer_2d.py#L32)

A 2D Transformer model as introduced in PixArt family of models (https://huggingface.co/papers/2310.00426,
https://huggingface.co/papers/2403.04692).

forwarddiffusers.PixArtTransformer2DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/pixart_transformer_2d.py#L227[{"name": "hidden_states", "val": ": Tensor"}, {"name": "encoder_hidden_states", "val": ": torch.Tensor | None = None"}, {"name": "timestep", "val": ": torch.LongTensor | None = None"}, {"name": "added_cond_kwargs", "val": ": dict = None"}, {"name": "cross_attention_kwargs", "val": ": dict = None"}, {"name": "attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "encoder_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "return_dict", "val": ": bool = True"}]- **hidden_states** (`torch.FloatTensor` of shape `(batch size, channel, height, width)`) --
  Input `hidden_states`.
- **encoder_hidden_states** (`torch.FloatTensor` of shape `(batch size, sequence len, embed dims)`, *optional*) --
  Conditional embeddings for cross attention layer. If not given, cross-attention defaults to
  self-attention.
- **timestep** (`torch.LongTensor`, *optional*) --
  Used to indicate denoising step. Optional timestep to be applied as an embedding in `AdaLayerNorm`.
- **added_cond_kwargs** -- (`dict[str, Any]`, *optional*): Additional conditions to be used as inputs.
- **cross_attention_kwargs** ( `dict[str, Any]`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **attention_mask** ( `torch.Tensor`, *optional*) --
  An attention mask of shape `(batch, key_tokens)` is applied to `encoder_hidden_states`. If `1` the mask
  is kept, otherwise if `0` it is discarded. Mask will be converted into a bias, which adds large
  negative values to the attention scores corresponding to "discard" tokens.
- **encoder_attention_mask** ( `torch.Tensor`, *optional*) --
  Cross-attention mask applied to `encoder_hidden_states`. Two formats supported:

  * Mask `(batch, sequence_length)` True = keep, False = discard.
  * Bias `(batch, 1, sequence_length)` 0 = keep, -10000 = discard.

  If `ndim == 2`: will be interpreted as a mask, then converted into a bias consistent with the format
  above. This bias will be added to the cross-attention scores.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a [UNet2DConditionOutput](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.models.unets.unet_2d_condition.UNet2DConditionOutput) instead of a plain
  tuple.0If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [PixArtTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/pixart_transformer2d#diffusers.PixArtTransformer2DModel) forward method.

**Parameters:**

num_attention_heads (int, optional, defaults to 16) : The number of heads to use for multi-head attention.

attention_head_dim (int, optional, defaults to 72) : The number of channels in each head.

in_channels (int, defaults to 4) : The number of channels in the input.

out_channels (int, optional) : The number of channels in the output. Specify this parameter if the output channel number differs from the input.

num_layers (int, optional, defaults to 28) : The number of layers of Transformer blocks to use.

dropout (float, optional, defaults to 0.0) : The dropout probability to use within the Transformer blocks.

norm_num_groups (int, optional, defaults to 32) : Number of groups for group normalization within Transformer blocks.

cross_attention_dim (int, optional) : The dimensionality for cross-attention layers, typically matching the encoder's hidden dimension.

attention_bias (bool, optional, defaults to True) : Configure if the Transformer blocks' attention should contain a bias parameter.

sample_size (int, defaults to 128) : The width of the latent images. This parameter is fixed during training.

patch_size (int, defaults to 2) : Size of the patches the model processes, relevant for architectures working on non-sequential data.

activation_fn (str, optional, defaults to "gelu-approximate") : Activation function to use in feed-forward networks within Transformer blocks.

num_embeds_ada_norm (int, optional, defaults to 1000) : Number of embeddings for AdaLayerNorm, fixed during training and affects the maximum denoising steps during inference.

upcast_attention (bool, optional, defaults to False) : If true, upcasts the attention mechanism dimensions for potentially improved performance.

norm_type (str, optional, defaults to "ada_norm_zero") : Specifies the type of normalization used, can be 'ada_norm_zero'.

norm_elementwise_affine (bool, optional, defaults to False) : If true, enables element-wise affine parameters in the normalization layers.

norm_eps (float, optional, defaults to 1e-6) : A small constant added to the denominator in normalization layers to prevent division by zero.

interpolation_scale (int, optional) : Scale factor to use during interpolating the position embeddings.

use_additional_conditions (bool, optional) : If we're using additional conditions as inputs.

attention_type (str, optional, defaults to "default") : Kind of attention mechanism to be used.

caption_channels (int, optional, defaults to None) : Number of channels to use for projecting the caption embeddings.

use_linear_projection (bool, optional, defaults to False) : Deprecated argument. Will be removed in a future version.

num_vector_embeds (bool, optional, defaults to False) : Deprecated argument. Will be removed in a future version.

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.
#### fuse_qkv_projections[[diffusers.PixArtTransformer2DModel.fuse_qkv_projections]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/pixart_transformer_2d.py#L196)

Enables fused QKV projections. For self-attention modules, all projection matrices (i.e., query, key, value)
are fused. For cross-attention modules, key and value projection matrices are fused.

> [!WARNING] > This API is 🧪 experimental.
#### set_default_attn_processor[[diffusers.PixArtTransformer2DModel.set_default_attn_processor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/pixart_transformer_2d.py#L187)

Disables custom attention processors and sets the default attention implementation.

Safe to just use `AttnProcessor()` as PixArt doesn't have any exotic attention processors in default model.
#### unfuse_qkv_projections[[diffusers.PixArtTransformer2DModel.unfuse_qkv_projections]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/pixart_transformer_2d.py#L218)

Disables the fused QKV projection if enabled.

> [!WARNING] > This API is 🧪 experimental.
