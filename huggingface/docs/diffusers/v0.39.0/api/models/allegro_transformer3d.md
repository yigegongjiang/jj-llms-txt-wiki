# AllegroTransformer3DModel

A Diffusion Transformer model for 3D data from [Allegro](https://github.com/rhymes-ai/Allegro) was introduced in [Allegro: Open the Black Box of Commercial-Level Video Generation Model](https://huggingface.co/papers/2410.15458) by RhymesAI.

The model can be loaded with the following code snippet.

```python
from diffusers import AllegroTransformer3DModel

transformer = AllegroTransformer3DModel.from_pretrained("rhymes-ai/Allegro", subfolder="transformer", torch_dtype=torch.bfloat16).to("cuda")
```

## AllegroTransformer3DModel[[diffusers.AllegroTransformer3DModel]]

#### diffusers.AllegroTransformer3DModel[[diffusers.AllegroTransformer3DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_allegro.py#L174)

forwarddiffusers.AllegroTransformer3DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_allegro.py#L305[{"name": "hidden_states", "val": ": Tensor"}, {"name": "encoder_hidden_states", "val": ": Tensor"}, {"name": "timestep", "val": ": LongTensor"}, {"name": "attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "encoder_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "image_rotary_emb", "val": ": tuple[torch.Tensor, torch.Tensor] | None = None"}, {"name": "return_dict", "val": ": bool = True"}]- **hidden_states** (`torch.Tensor` of shape `(batch_size, num_channels, num_frames, height, width)`) --
  Input `hidden_states`.
- **encoder_hidden_states** (`torch.Tensor` of shape `(batch_size, sequence_len, embed_dims)`) --
  Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.
- **timestep** (`torch.LongTensor`) --
  Used to indicate denoising step.
- **attention_mask** (`torch.Tensor`, *optional*) --
  Self-attention mask applied to `hidden_states`.
- **encoder_attention_mask** (`torch.Tensor`, *optional*) --
  Cross-attention mask applied to `encoder_hidden_states`.
- **image_rotary_emb** (`tuple` of `torch.Tensor`, *optional*) --
  Pre-computed rotary positional embeddings.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain
  tuple.0If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [AllegroTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/allegro_transformer3d#diffusers.AllegroTransformer3DModel) forward method.

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch_size, num_channels, num_frames, height, width)`) : Input `hidden_states`.

encoder_hidden_states (`torch.Tensor` of shape `(batch_size, sequence_len, embed_dims)`) : Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.

timestep (`torch.LongTensor`) : Used to indicate denoising step.

attention_mask (`torch.Tensor`, *optional*) : Self-attention mask applied to `hidden_states`.

encoder_attention_mask (`torch.Tensor`, *optional*) : Cross-attention mask applied to `encoder_hidden_states`.

image_rotary_emb (`tuple` of `torch.Tensor`, *optional*) : Pre-computed rotary positional embeddings.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_outputs.py#L21)

The output of [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel).

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.
