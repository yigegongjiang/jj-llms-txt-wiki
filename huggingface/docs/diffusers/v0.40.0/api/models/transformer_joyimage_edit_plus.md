# JoyImageEditPlusTransformer3DModel

The model can be loaded with the following code snippet.

```python
from diffusers import JoyImageEditPlusTransformer3DModel

transformer = JoyImageEditPlusTransformer3DModel.from_pretrained("jdopensource/JoyAI-Image-Edit-Plus-Diffusers", subfolder="transformer", dtype=torch.bfloat16)
```

## JoyImageEditPlusTransformer3DModel[[diffusers.JoyImageEditPlusTransformer3DModel]]

#### diffusers.JoyImageEditPlusTransformer3DModel[[diffusers.JoyImageEditPlusTransformer3DModel]]

```python
diffusers.JoyImageEditPlusTransformer3DModel(patch_size: list = [1, 2, 2], in_channels: int = 16, out_channels: int | None = None, hidden_size: int = 3072, num_attention_heads: int = 24, text_dim: int = 4096, mlp_width_ratio: float = 4.0, num_layers: int = 20, rope_dim_list: list = [16, 56, 56], rope_type: str = 'rope', theta: int = 256)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_joyimage_edit_plus.py#L317)

**Parameters:**

patch_size (`list`, defaults to `[1, 2, 2]`) : Patch size for patchifying the latent input along `(t, h, w)` dimensions.

in_channels (`int`, defaults to `16`) : The number of channels in the input latent.

out_channels (`int`, *optional*, defaults to `None`) : The number of channels in the output. If not specified, it defaults to `in_channels`.

hidden_size (`int`, defaults to `3072`) : The dimensionality of the hidden representations.

num_attention_heads (`int`, defaults to `24`) : The number of attention heads.

text_dim (`int`, defaults to `4096`) : The dimensionality of the text encoder output.

mlp_width_ratio (`float`, defaults to `4.0`) : The ratio of MLP hidden dimension to `hidden_size`.

num_layers (`int`, defaults to `20`) : The number of double-stream transformer blocks.

rope_dim_list (`list[int]`, defaults to `[16, 56, 56]`) : The dimensions for 3D rotary positional embeddings along `(t, h, w)`.

rope_type (`str`, defaults to `"rope"`) : The type of rotary positional embedding.

theta (`int`, defaults to `256`) : The base frequency for rotary embeddings.

JoyImage Edit Plus Transformer for multi-image editing.

Uses a patchify+padding approach where each reference image and the target noise are independently patchified and
concatenated into a flat patch sequence. Supports variable-resolution reference images.

Input format: `[B, max_patches, C, pt, ph, pw]` (6D padded patches).

#### forward[[diffusers.JoyImageEditPlusTransformer3DModel.forward]]

```python
forward(hidden_states: Tensor, timestep: Tensor, encoder_hidden_states: Tensor, encoder_hidden_states_mask: typing.Optional[torch.Tensor] = None, shape_list: list = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_joyimage_edit_plus.py#L444)

**Parameters:**

hidden_states : [B, max_patches, C, pt, ph, pw] - patchified latent input.

timestep : [B] - diffusion timestep.

encoder_hidden_states : [B, L, D] - text encoder outputs.

encoder_hidden_states_mask : [B, L] - attention mask for text tokens.

shape_list : Per-sample list of (t, h, w) tuples for each component (target + references).

return_dict : Whether to return a dict or tuple.

**Returns:**

If `return_dict` is True, an [Transformer2DModelOutput](/docs/diffusers/v0.40.0/en/api/models/hunyuan_video15_transformer_3d#diffusers.models.modeling_outputs.Transformer2DModelOutput) is returned, otherwise a
`tuple` where the first element is the sample tensor.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

```python
diffusers.models.modeling_outputs.Transformer2DModelOutput(sample: torch.Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/modeling_outputs.py#L21)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.

The output of [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel).
