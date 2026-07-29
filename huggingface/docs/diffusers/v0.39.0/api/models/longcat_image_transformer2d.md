# LongCatImageTransformer2DModel

The model can be loaded with the following code snippet.

```python
from diffusers import LongCatImageTransformer2DModel

transformer = LongCatImageTransformer2DModel.from_pretrained("meituan-longcat/LongCat-Image ", subfolder="transformer", torch_dtype=torch.bfloat16)
```

## LongCatImageTransformer2DModel[[diffusers.LongCatImageTransformer2DModel]]

#### diffusers.LongCatImageTransformer2DModel[[diffusers.LongCatImageTransformer2DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_longcat_image.py#L395)

The Transformer model introduced in Longcat-Image.

forwarddiffusers.LongCatImageTransformer2DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_longcat_image.py#L464[{"name": "hidden_states", "val": ": Tensor"}, {"name": "encoder_hidden_states", "val": ": Tensor = None"}, {"name": "timestep", "val": ": LongTensor = None"}, {"name": "img_ids", "val": ": Tensor = None"}, {"name": "txt_ids", "val": ": Tensor = None"}, {"name": "guidance", "val": ": Tensor = None"}, {"name": "return_dict", "val": ": bool = True"}]- **hidden_states** (`torch.FloatTensor` of shape `(batch size, channel, height, width)`) --
  Input `hidden_states`.
- **encoder_hidden_states** (`torch.FloatTensor` of shape `(batch size, sequence_len, embed_dims)`) --
  Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.
- **timestep** ( `torch.LongTensor`) --
  Used to indicate denoising step.
- **img_ids** (`torch.Tensor`) --
  Image position ids used to compute the rotary positional embeddings.
- **txt_ids** (`torch.Tensor`) --
  Text position ids used to compute the rotary positional embeddings.
- **guidance** (`torch.Tensor`, *optional*) --
  Guidance scale embedding used for guidance-distilled variants of the model.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain
  tuple.0If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The forward method.

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
