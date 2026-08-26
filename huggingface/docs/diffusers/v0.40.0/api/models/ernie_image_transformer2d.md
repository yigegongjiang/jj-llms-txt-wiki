# ErnieImageTransformer2DModel

A Transformer model for image-like data from [ERNIE-Image](https://huggingface.co/baidu/ERNIE-Image).

A Transformer model for image-like data from [ERNIE-Image-Turbo](https://huggingface.co/baidu/ERNIE-Image-Turbo).

## ErnieImageTransformer2DModel[[diffusers.ErnieImageTransformer2DModel]]

#### diffusers.ErnieImageTransformer2DModel[[diffusers.ErnieImageTransformer2DModel]]

```python
diffusers.ErnieImageTransformer2DModel(hidden_size: int = 3072, num_attention_heads: int = 24, num_layers: int = 24, ffn_hidden_size: int = 8192, in_channels: int = 128, out_channels: int = 128, patch_size: int = 1, text_in_dim: int = 2560, rope_theta: int = 256, rope_axes_dim: typing.Tuple[int, int, int] = (32, 48, 48), eps: float = 1e-06, qk_layernorm: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_ernie_image.py#L296)

#### forward[[diffusers.ErnieImageTransformer2DModel.forward]]

```python
forward(hidden_states: Tensor, timestep: Tensor, text_bth: Tensor, text_lens: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_ernie_image.py#L348)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch_size, in_channels, height, width)`) : Input `hidden_states`.

timestep (`torch.LongTensor`) : Used to indicate denoising step.

text_bth (`torch.Tensor`) : Conditional text embeddings (embeddings computed from the input conditions such as prompts) to use, shaped `(batch_size, text_length, embed_dims)`.

text_lens (`torch.Tensor`) : Per-sample text sequence lengths used to build the attention mask.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

The [ErnieImageTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/ernie_image_transformer2d#diffusers.ErnieImageTransformer2DModel) forward method.
