# ErnieImageTransformer2DModel

A Transformer model for image-like data from [ERNIE-Image](https://huggingface.co/baidu/ERNIE-Image).

A Transformer model for image-like data from [ERNIE-Image-Turbo](https://huggingface.co/baidu/ERNIE-Image-Turbo).

## ErnieImageTransformer2DModel[[diffusers.ErnieImageTransformer2DModel]]

#### diffusers.ErnieImageTransformer2DModel[[diffusers.ErnieImageTransformer2DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_ernie_image.py#L296)

forwarddiffusers.ErnieImageTransformer2DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_ernie_image.py#L348[{"name": "hidden_states", "val": ": Tensor"}, {"name": "timestep", "val": ": Tensor"}, {"name": "text_bth", "val": ": Tensor"}, {"name": "text_lens", "val": ": Tensor"}, {"name": "return_dict", "val": ": bool = True"}]- **hidden_states** (`torch.Tensor` of shape `(batch_size, in_channels, height, width)`) --
  Input `hidden_states`.
- **timestep** (`torch.LongTensor`) --
  Used to indicate denoising step.
- **text_bth** (`torch.Tensor`) --
  Conditional text embeddings (embeddings computed from the input conditions such as prompts) to use,
  shaped `(batch_size, text_length, embed_dims)`.
- **text_lens** (`torch.Tensor`) --
  Per-sample text sequence lengths used to build the attention mask.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain
  tuple.0

The [ErnieImageTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/ernie_image_transformer2d#diffusers.ErnieImageTransformer2DModel) forward method.

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch_size, in_channels, height, width)`) : Input `hidden_states`.

timestep (`torch.LongTensor`) : Used to indicate denoising step.

text_bth (`torch.Tensor`) : Conditional text embeddings (embeddings computed from the input conditions such as prompts) to use, shaped `(batch_size, text_length, embed_dims)`.

text_lens (`torch.Tensor`) : Per-sample text sequence lengths used to build the attention mask.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.
