## LatteTransformer3DModel

A Diffusion Transformer model for 3D data from [Latte](https://github.com/Vchitect/Latte).

## LatteTransformer3DModel[[diffusers.LatteTransformer3DModel]]

#### diffusers.LatteTransformer3DModel[[diffusers.LatteTransformer3DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/latte_transformer_3d.py#L27)

forwarddiffusers.LatteTransformer3DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/latte_transformer_3d.py#L166[{"name": "hidden_states", "val": ": Tensor"}, {"name": "timestep", "val": ": torch.LongTensor | None = None"}, {"name": "encoder_hidden_states", "val": ": torch.Tensor | None = None"}, {"name": "encoder_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "enable_temporal_attentions", "val": ": bool = True"}, {"name": "return_dict", "val": ": bool = True"}]- **hidden_states** (`torch.Tensor` of shape `(batch size, channel, num_frame, height, width)`) --
  Input `hidden_states`.
- **timestep** ( `torch.LongTensor`, *optional*) --
  Used to indicate denoising step. Optional timestep to be applied as an embedding in `AdaLayerNorm`.
- **encoder_hidden_states** ( `torch.FloatTensor` of shape `(batch size, sequence len, embed dims)`, *optional*) --
  Conditional embeddings for cross attention layer. If not given, cross-attention defaults to
  self-attention.
- **encoder_attention_mask** ( `torch.Tensor`, *optional*) --
  Cross-attention mask applied to `encoder_hidden_states`. Two formats supported:

  * Mask `(batcheight, sequence_length)` True = keep, False = discard.
  * Bias `(batcheight, 1, sequence_length)` 0 = keep, -10000 = discard.

  If `ndim == 2`: will be interpreted as a mask, then converted into a bias consistent with the format
  above. This bias will be added to the cross-attention scores.
- **enable_temporal_attentions** --
  (`bool`, *optional*, defaults to `True`): Whether to enable temporal attentions.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~models.unet_2d_condition.UNet2DConditionOutput` instead of a plain
  tuple.0If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [LatteTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/latte_transformer3d#diffusers.LatteTransformer3DModel) forward method.

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
