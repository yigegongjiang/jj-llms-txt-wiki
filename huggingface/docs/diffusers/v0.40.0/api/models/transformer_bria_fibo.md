# BriaFiboTransformer2DModel

A modified flux Transformer model from [Bria](https://huggingface.co/briaai/FIBO)

## BriaFiboTransformer2DModel[[diffusers.BriaFiboTransformer2DModel]]

#### diffusers.BriaFiboTransformer2DModel[[diffusers.BriaFiboTransformer2DModel]]

```python
diffusers.BriaFiboTransformer2DModel(patch_size: int = 1, in_channels: int = 64, num_layers: int = 19, num_single_layers: int = 38, attention_head_dim: int = 128, num_attention_heads: int = 24, joint_attention_dim: int = 4096, pooled_projection_dim: int = None, guidance_embeds: bool = False, axes_dims_rope: list = [16, 56, 56], rope_theta = 10000, time_theta = 10000, text_encoder_dim: int = 2048)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_bria_fibo.py#L428)

**Parameters:**

patch_size (`int`) : Patch size to turn the input data into small patches.

in_channels (`int`, *optional*, defaults to 16) : The number of channels in the input.

num_layers (`int`, *optional*, defaults to 18) : The number of layers of MMDiT blocks to use.

num_single_layers (`int`, *optional*, defaults to 18) : The number of layers of single DiT blocks to use.

attention_head_dim (`int`, *optional*, defaults to 64) : The number of channels in each head.

num_attention_heads (`int`, *optional*, defaults to 18) : The number of heads to use for multi-head attention.

joint_attention_dim (`int`, *optional*) : The number of `encoder_hidden_states` dimensions to use.

pooled_projection_dim (`int`) : Number of dimensions to use when projecting the `pooled_projections`.

guidance_embeds (`bool`, defaults to False) : Whether to use guidance embeddings.

... --

#### forward[[diffusers.BriaFiboTransformer2DModel.forward]]

```python
forward(hidden_states: Tensor, encoder_hidden_states: Tensor = None, text_encoder_layers: list = None, pooled_projections: Tensor = None, timestep: LongTensor = None, img_ids: Tensor = None, txt_ids: Tensor = None, guidance: Tensor = None, joint_attention_kwargs: dict[str, typing.Any] | None = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_bria_fibo.py#L509)

**Parameters:**

hidden_states (`torch.FloatTensor` of shape `(batch size, channel, height, width)`) : Input `hidden_states`.

encoder_hidden_states (`torch.FloatTensor` of shape `(batch size, sequence_len, embed_dims)`) : Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.

text_encoder_layers (`list` of `torch.Tensor`) : Per-block text encoder hidden states, one tensor per transformer block.

pooled_projections (`torch.FloatTensor` of shape `(batch_size, projection_dim)`) : Embeddings projected from the embeddings of input conditions.

timestep ( `torch.LongTensor`) : Used to indicate denoising step.

img_ids (`torch.Tensor`) : Image position ids used to compute the rotary positional embeddings.

txt_ids (`torch.Tensor`) : Text position ids used to compute the rotary positional embeddings.

guidance (`torch.Tensor`, *optional*) : Guidance scale embedding used for guidance-distilled variants of the model.

joint_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.
