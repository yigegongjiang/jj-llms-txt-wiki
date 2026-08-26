# AuraFlowTransformer2DModel

A Transformer model for image-like data from [AuraFlow](https://blog.fal.ai/auraflow/).

## AuraFlowTransformer2DModel[[diffusers.AuraFlowTransformer2DModel]]

#### diffusers.AuraFlowTransformer2DModel[[diffusers.AuraFlowTransformer2DModel]]

```python
diffusers.AuraFlowTransformer2DModel(sample_size: int = 64, patch_size: int = 2, in_channels: int = 4, num_mmdit_layers: int = 4, num_single_dit_layers: int = 32, attention_head_dim: int = 256, num_attention_heads: int = 12, joint_attention_dim: int = 2048, caption_projection_dim: int = 3072, out_channels: int = 4, pos_embed_max_size: int = 1024)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/auraflow_transformer_2d.py#L278)

**Parameters:**

sample_size (`int`) : The width of the latent images. This is fixed during training since it is used to learn a number of position embeddings.

patch_size (`int`) : Patch size to turn the input data into small patches.

in_channels (`int`, *optional*, defaults to 4) : The number of channels in the input.

num_mmdit_layers (`int`, *optional*, defaults to 4) : The number of layers of MMDiT Transformer blocks to use.

num_single_dit_layers (`int`, *optional*, defaults to 32) : The number of layers of Transformer blocks to use. These blocks use concatenated image and text representations.

attention_head_dim (`int`, *optional*, defaults to 256) : The number of channels in each head.

num_attention_heads (`int`, *optional*, defaults to 12) : The number of heads to use for multi-head attention.

joint_attention_dim (`int`, *optional*) : The number of `encoder_hidden_states` dimensions to use.

caption_projection_dim (`int`) : Number of dimensions to use when projecting the `encoder_hidden_states`.

out_channels (`int`, defaults to 4) : Number of output channels.

pos_embed_max_size (`int`, defaults to 1024) : Maximum positions to embed from the image latents.

A 2D Transformer model as introduced in AuraFlow (https://blog.fal.ai/auraflow/).

#### forward[[diffusers.AuraFlowTransformer2DModel.forward]]

```python
forward(hidden_states: FloatTensor, encoder_hidden_states: FloatTensor = None, timestep: LongTensor = None, attention_kwargs: dict[str, typing.Any] | None = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/auraflow_transformer_2d.py#L400)

**Parameters:**

hidden_states (`torch.FloatTensor` of shape `(batch size, channel, height, width)`) : Input `hidden_states`.

encoder_hidden_states (`torch.FloatTensor` of shape `(batch size, sequence_len, embed_dims)`) : Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.

timestep (`torch.LongTensor`) : Used to indicate denoising step.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [AuraFlowTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/aura_flow_transformer2d#diffusers.AuraFlowTransformer2DModel) forward method.

#### fuse_qkv_projections[[diffusers.AuraFlowTransformer2DModel.fuse_qkv_projections]]

```python
fuse_qkv_projections()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/auraflow_transformer_2d.py#L369)

Enables fused QKV projections. For self-attention modules, all projection matrices (i.e., query, key, value)
are fused. For cross-attention modules, key and value projection matrices are fused.

> [!WARNING] > This API is 🧪 experimental.

#### unfuse_qkv_projections[[diffusers.AuraFlowTransformer2DModel.unfuse_qkv_projections]]

```python
unfuse_qkv_projections()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/auraflow_transformer_2d.py#L391)

Disables the fused QKV projection if enabled.

> [!WARNING] > This API is 🧪 experimental.
