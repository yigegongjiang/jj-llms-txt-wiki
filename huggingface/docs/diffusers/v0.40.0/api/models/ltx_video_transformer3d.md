# LTXVideoTransformer3DModel

A Diffusion Transformer model for 3D data from [LTX](https://huggingface.co/Lightricks/LTX-Video) was introduced by Lightricks.

The model can be loaded with the following code snippet.

```python
from diffusers import LTXVideoTransformer3DModel

transformer = LTXVideoTransformer3DModel.from_pretrained("Lightricks/LTX-Video", subfolder="transformer", dtype=torch.bfloat16).to("cuda")
```

## LTXVideoTransformer3DModel[[diffusers.LTXVideoTransformer3DModel]]

#### diffusers.LTXVideoTransformer3DModel[[diffusers.LTXVideoTransformer3DModel]]

```python
diffusers.LTXVideoTransformer3DModel(in_channels: int = 128, out_channels: int = 128, patch_size: int = 1, patch_size_t: int = 1, num_attention_heads: int = 32, attention_head_dim: int = 64, cross_attention_dim: int = 2048, num_layers: int = 28, activation_fn: str = 'gelu-approximate', qk_norm: str = 'rms_norm_across_heads', norm_elementwise_affine: bool = False, norm_eps: float = 1e-06, caption_channels: int = 4096, attention_bias: bool = True, attention_out_bias: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_ltx.py#L385)

**Parameters:**

in_channels (`int`, defaults to `128`) : The number of channels in the input.

out_channels (`int`, defaults to `128`) : The number of channels in the output.

patch_size (`int`, defaults to `1`) : The size of the spatial patches to use in the patch embedding layer.

patch_size_t (`int`, defaults to `1`) : The size of the tmeporal patches to use in the patch embedding layer.

num_attention_heads (`int`, defaults to `32`) : The number of heads to use for multi-head attention.

attention_head_dim (`int`, defaults to `64`) : The number of channels in each head.

cross_attention_dim (`int`, defaults to `2048 `) : The number of channels for cross attention heads.

num_layers (`int`, defaults to `28`) : The number of layers of Transformer blocks to use.

activation_fn (`str`, defaults to `"gelu-approximate"`) : Activation function to use in feed-forward.

qk_norm (`str`, defaults to `"rms_norm_across_heads"`) : The normalization layer to use.

A Transformer model for video-like data used in [LTX](https://huggingface.co/Lightricks/LTX-Video).

#### forward[[diffusers.LTXVideoTransformer3DModel.forward]]

```python
forward(hidden_states: Tensor, encoder_hidden_states: Tensor, timestep: LongTensor, encoder_attention_mask: Tensor, num_frames: int | None = None, height: int | None = None, width: int | None = None, rope_interpolation_scale: typing.Union[tuple[float, float, float], torch.Tensor, NoneType] = None, video_coords: typing.Optional[torch.Tensor] = None, attention_kwargs: dict[str, typing.Any] | None = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_ltx.py#L494)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch_size, sequence_length, in_channels)`) : Input `hidden_states`.

encoder_hidden_states (`torch.Tensor` of shape `(batch_size, sequence_len, embed_dims)`) : Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.

timestep (`torch.LongTensor`) : Used to indicate denoising step.

encoder_attention_mask (`torch.Tensor`) : Mask applied to `encoder_hidden_states` during attention.

num_frames (`int`, *optional*) : Number of frames in the video used to compute the rotary positional embeddings.

height (`int`, *optional*) : Height of the latent used to compute the rotary positional embeddings.

width (`int`, *optional*) : Width of the latent used to compute the rotary positional embeddings.

rope_interpolation_scale (`tuple` of `float` or `torch.Tensor`, *optional*) : Interpolation scale used by the rotary positional embeddings.

video_coords (`torch.Tensor`, *optional*) : Pre-computed video coordinates used by the rotary positional embeddings.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

**Returns:** `torch.Tensor`

The denoised output tensor of shape `(batch_size, sequence_length, out_channels)`.

The [LTXVideoTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/ltx_video_transformer3d#diffusers.LTXVideoTransformer3DModel) forward method.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

```python
diffusers.models.modeling_outputs.Transformer2DModelOutput(sample: torch.Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/modeling_outputs.py#L21)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.

The output of [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel).
