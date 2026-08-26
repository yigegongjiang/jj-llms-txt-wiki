# DiTTransformer2DModel

A Transformer model for image-like data from [DiT](https://huggingface.co/papers/2212.09748).

## DiTTransformer2DModel[[diffusers.DiTTransformer2DModel]]

#### diffusers.DiTTransformer2DModel[[diffusers.DiTTransformer2DModel]]

```python
diffusers.DiTTransformer2DModel(num_attention_heads: int = 16, attention_head_dim: int = 72, in_channels: int = 4, out_channels: int | None = None, num_layers: int = 28, dropout: float = 0.0, norm_num_groups: int = 32, attention_bias: bool = True, sample_size: int = 32, patch_size: int = 2, activation_fn: str = 'gelu-approximate', num_embeds_ada_norm: int | None = 1000, upcast_attention: bool = False, norm_type: str = 'ada_norm_zero', norm_elementwise_affine: bool = False, norm_eps: float = 1e-05)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/dit_transformer_2d.py#L31)

**Parameters:**

num_attention_heads (int, optional, defaults to 16) : The number of heads to use for multi-head attention.

attention_head_dim (int, optional, defaults to 72) : The number of channels in each head.

in_channels (int, defaults to 4) : The number of channels in the input.

out_channels (int, optional) : The number of channels in the output. Specify this parameter if the output channel number differs from the input.

num_layers (int, optional, defaults to 28) : The number of layers of Transformer blocks to use.

dropout (float, optional, defaults to 0.0) : The dropout probability to use within the Transformer blocks.

norm_num_groups (int, optional, defaults to 32) : Number of groups for group normalization within Transformer blocks.

attention_bias (bool, optional, defaults to True) : Configure if the Transformer blocks' attention should contain a bias parameter.

sample_size (int, defaults to 32) : The width of the latent images. This parameter is fixed during training.

patch_size (int, defaults to 2) : Size of the patches the model processes, relevant for architectures working on non-sequential data.

activation_fn (str, optional, defaults to "gelu-approximate") : Activation function to use in feed-forward networks within Transformer blocks.

num_embeds_ada_norm (int, optional, defaults to 1000) : Number of embeddings for AdaLayerNorm, fixed during training and affects the maximum denoising steps during inference.

upcast_attention (bool, optional, defaults to False) : If true, upcasts the attention mechanism dimensions for potentially improved performance.

norm_type (str, optional, defaults to "ada_norm_zero") : Specifies the type of normalization used, can be 'ada_norm_zero'.

norm_elementwise_affine (bool, optional, defaults to False) : If true, enables element-wise affine parameters in the normalization layers.

norm_eps (float, optional, defaults to 1e-5) : A small constant added to the denominator in normalization layers to prevent division by zero.

A 2D Transformer model as introduced in DiT (https://huggingface.co/papers/2212.09748).

#### forward[[diffusers.DiTTransformer2DModel.forward]]

```python
forward(hidden_states: Tensor, timestep: typing.Optional[torch.LongTensor] = None, class_labels: typing.Optional[torch.LongTensor] = None, cross_attention_kwargs: dict = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/dit_transformer_2d.py#L148)

**Parameters:**

hidden_states (`torch.LongTensor` of shape `(batch size, num latent pixels)` if discrete, `torch.FloatTensor` of shape `(batch size, channel, height, width)` if continuous) : Input `hidden_states`.

timestep ( `torch.LongTensor`, *optional*) : Used to indicate denoising step. Optional timestep to be applied as an embedding in `AdaLayerNorm`.

class_labels ( `torch.LongTensor` of shape `(batch size, num classes)`, *optional*) : Used to indicate class labels conditioning. Optional class labels to be applied as an embedding in `AdaLayerZeroNorm`.

cross_attention_kwargs ( `dict[str, Any]`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [UNet2DConditionOutput](/docs/diffusers/v0.40.0/en/api/models/unet2d-cond#diffusers.models.unets.unet_2d_condition.UNet2DConditionOutput) instead of a plain tuple.

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [DiTTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/dit_transformer2d#diffusers.DiTTransformer2DModel) forward method.
