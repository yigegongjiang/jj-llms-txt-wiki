## LatteTransformer3DModel

A Diffusion Transformer model for 3D data from [Latte](https://github.com/Vchitect/Latte).

## LatteTransformer3DModel[[diffusers.LatteTransformer3DModel]]

#### diffusers.LatteTransformer3DModel[[diffusers.LatteTransformer3DModel]]

```python
diffusers.LatteTransformer3DModel(num_attention_heads: int = 16, attention_head_dim: int = 88, in_channels: int | None = None, out_channels: int | None = None, num_layers: int = 1, dropout: float = 0.0, cross_attention_dim: int | None = None, attention_bias: bool = False, sample_size: int = 64, patch_size: int | None = None, activation_fn: str = 'geglu', num_embeds_ada_norm: int | None = None, norm_type: str = 'layer_norm', norm_elementwise_affine: bool = True, norm_eps: float = 1e-05, caption_channels: int = None, video_length: int = 16)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/latte_transformer_3d.py#L27)

#### forward[[diffusers.LatteTransformer3DModel.forward]]

```python
forward(hidden_states: Tensor, timestep: typing.Optional[torch.LongTensor] = None, encoder_hidden_states: typing.Optional[torch.Tensor] = None, encoder_attention_mask: typing.Optional[torch.Tensor] = None, enable_temporal_attentions: bool = True, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/latte_transformer_3d.py#L166)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch size, channel, num_frame, height, width)`) : Input `hidden_states`.

timestep ( `torch.LongTensor`, *optional*) : Used to indicate denoising step. Optional timestep to be applied as an embedding in `AdaLayerNorm`.

encoder_hidden_states ( `torch.FloatTensor` of shape `(batch size, sequence len, embed dims)`, *optional*) : Conditional embeddings for cross attention layer. If not given, cross-attention defaults to self-attention.

encoder_attention_mask ( `torch.Tensor`, *optional*) : Cross-attention mask applied to `encoder_hidden_states`. Two formats supported:  * Mask `(batcheight, sequence_length)` True = keep, False = discard. * Bias `(batcheight, 1, sequence_length)` 0 = keep, -10000 = discard.  If `ndim == 2`: will be interpreted as a mask, then converted into a bias consistent with the format above. This bias will be added to the cross-attention scores.

enable_temporal_attentions : (`bool`, *optional*, defaults to `True`): Whether to enable temporal attentions.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.unet_2d_condition.UNet2DConditionOutput` instead of a plain tuple.

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [LatteTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/latte_transformer3d#diffusers.LatteTransformer3DModel) forward method.
