# LongCatImageTransformer2DModel

The model can be loaded with the following code snippet.

```python
from diffusers import LongCatImageTransformer2DModel

transformer = LongCatImageTransformer2DModel.from_pretrained("meituan-longcat/LongCat-Image ", subfolder="transformer", dtype=torch.bfloat16)
```

## LongCatImageTransformer2DModel[[diffusers.LongCatImageTransformer2DModel]]

#### diffusers.LongCatImageTransformer2DModel[[diffusers.LongCatImageTransformer2DModel]]

```python
diffusers.LongCatImageTransformer2DModel(patch_size: int = 1, in_channels: int = 64, num_layers: int = 19, num_single_layers: int = 38, attention_head_dim: int = 128, num_attention_heads: int = 24, joint_attention_dim: int = 3584, pooled_projection_dim: int = 3584, axes_dims_rope: list = [16, 56, 56])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_longcat_image.py#L395)

The Transformer model introduced in Longcat-Image.

#### forward[[diffusers.LongCatImageTransformer2DModel.forward]]

```python
forward(hidden_states: Tensor, encoder_hidden_states: Tensor = None, timestep: LongTensor = None, img_ids: Tensor = None, txt_ids: Tensor = None, guidance: Tensor = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_longcat_image.py#L464)

**Parameters:**

hidden_states (`torch.FloatTensor` of shape `(batch size, channel, height, width)`) : Input `hidden_states`.

encoder_hidden_states (`torch.FloatTensor` of shape `(batch size, sequence_len, embed_dims)`) : Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.

timestep ( `torch.LongTensor`) : Used to indicate denoising step.

img_ids (`torch.Tensor`) : Image position ids used to compute the rotary positional embeddings.

txt_ids (`torch.Tensor`) : Text position ids used to compute the rotary positional embeddings.

guidance (`torch.Tensor`, *optional*) : Guidance scale embedding used for guidance-distilled variants of the model.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The forward method.
