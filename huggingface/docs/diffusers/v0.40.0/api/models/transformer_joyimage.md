# JoyImageEditTransformer3DModel

The model can be loaded with the following code snippet.

```python
from diffusers import JoyImageEditTransformer3DModel

transformer = JoyImageEditTransformer3DModel.from_pretrained("jdopensource/JoyAI-Image-Edit-Diffusers", subfolder="transformer", dtype=torch.bfloat16)
```

## JoyImageEditTransformer3DModel[[diffusers.JoyImageEditTransformer3DModel]]

#### diffusers.JoyImageEditTransformer3DModel[[diffusers.JoyImageEditTransformer3DModel]]

```python
diffusers.JoyImageEditTransformer3DModel(patch_size: list = [1, 2, 2], in_channels: int = 16, out_channels: int | None = None, hidden_size: int = 3072, num_attention_heads: int = 24, text_dim: int = 4096, mlp_width_ratio: float = 4.0, num_layers: int = 20, rope_dim_list: list = [16, 56, 56], rope_type: str = 'rope', theta: int = 256)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_joyimage.py#L372)

JoyImage Transformer model for image generation / editing.

Dual-stream DiT architecture with WAN-style conditioning embeddings and custom rotary position embeddings.

#### forward[[diffusers.JoyImageEditTransformer3DModel.forward]]

```python
forward(hidden_states: Tensor, timestep: Tensor, encoder_hidden_states: Tensor = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_joyimage.py#L522)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch_size, num_channels, num_frames, height, width)` or `(batch_size, num_items, num_channels, num_frames, height, width)`) : Input `hidden_states`.

timestep (`torch.LongTensor`) : Used to indicate denoising step.

encoder_hidden_states (`torch.Tensor`, *optional*) : Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

The [JoyImageEditTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/transformer_joyimage#diffusers.JoyImageEditTransformer3DModel) forward method.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

```python
diffusers.models.modeling_outputs.Transformer2DModelOutput(sample: torch.Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/modeling_outputs.py#L21)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.

The output of [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel).
