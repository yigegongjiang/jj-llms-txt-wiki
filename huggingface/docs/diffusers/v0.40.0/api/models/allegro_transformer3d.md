# AllegroTransformer3DModel

A Diffusion Transformer model for 3D data from [Allegro](https://github.com/rhymes-ai/Allegro) was introduced in [Allegro: Open the Black Box of Commercial-Level Video Generation Model](https://huggingface.co/papers/2410.15458) by RhymesAI.

The model can be loaded with the following code snippet.

```python
from diffusers import AllegroTransformer3DModel

transformer = AllegroTransformer3DModel.from_pretrained("rhymes-ai/Allegro", subfolder="transformer", dtype=torch.bfloat16).to("cuda")
```

## AllegroTransformer3DModel[[diffusers.AllegroTransformer3DModel]]

#### diffusers.AllegroTransformer3DModel[[diffusers.AllegroTransformer3DModel]]

```python
diffusers.AllegroTransformer3DModel(patch_size: int = 2, patch_size_t: int = 1, num_attention_heads: int = 24, attention_head_dim: int = 96, in_channels: int = 4, out_channels: int = 4, num_layers: int = 32, dropout: float = 0.0, cross_attention_dim: int = 2304, attention_bias: bool = True, sample_height: int = 90, sample_width: int = 160, sample_frames: int = 22, activation_fn: str = 'gelu-approximate', norm_elementwise_affine: bool = False, norm_eps: float = 1e-06, caption_channels: int = 4096, interpolation_scale_h: float = 2.0, interpolation_scale_w: float = 2.0, interpolation_scale_t: float = 2.2)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_allegro.py#L174)

#### forward[[diffusers.AllegroTransformer3DModel.forward]]

```python
forward(hidden_states: Tensor, encoder_hidden_states: Tensor, timestep: LongTensor, attention_mask: typing.Optional[torch.Tensor] = None, encoder_attention_mask: typing.Optional[torch.Tensor] = None, image_rotary_emb: tuple[torch.Tensor, torch.Tensor] | None = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_allegro.py#L305)

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

The [AllegroTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/allegro_transformer3d#diffusers.AllegroTransformer3DModel) forward method.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

```python
diffusers.models.modeling_outputs.Transformer2DModelOutput(sample: torch.Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/modeling_outputs.py#L21)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.

The output of [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel).
