# CogVideoXTransformer3DModel

A Diffusion Transformer model for 3D data from [CogVideoX](https://github.com/THUDM/CogVideo) was introduced in [CogVideoX: Text-to-Video Diffusion Models with An Expert Transformer](https://github.com/THUDM/CogVideo/blob/main/resources/CogVideoX.pdf) by Tsinghua University & ZhipuAI.

The model can be loaded with the following code snippet.

```python
from diffusers import CogVideoXTransformer3DModel

transformer = CogVideoXTransformer3DModel.from_pretrained("THUDM/CogVideoX-2b", subfolder="transformer", torch_dtype=torch.float16).to("cuda")
```

## CogVideoXTransformer3DModel[[diffusers.CogVideoXTransformer3DModel]]

#### diffusers.CogVideoXTransformer3DModel[[diffusers.CogVideoXTransformer3DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/cogvideox_transformer_3d.py#L160)

A Transformer model for video-like data in [CogVideoX](https://github.com/THUDM/CogVideo).

forwarddiffusers.CogVideoXTransformer3DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/cogvideox_transformer_3d.py#L366[{"name": "hidden_states", "val": ": Tensor"}, {"name": "encoder_hidden_states", "val": ": Tensor"}, {"name": "timestep", "val": ": int | float | torch.LongTensor"}, {"name": "timestep_cond", "val": ": torch.Tensor | None = None"}, {"name": "ofs", "val": ": int | float | torch.LongTensor | None = None"}, {"name": "image_rotary_emb", "val": ": tuple[torch.Tensor, torch.Tensor] | None = None"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "return_dict", "val": ": bool = True"}]- **hidden_states** (`torch.Tensor` of shape `(batch_size, num_frames, channels, height, width)`) --
  Input `hidden_states`.
- **encoder_hidden_states** (`torch.Tensor` of shape `(batch_size, sequence_len, embed_dims)`) --
  Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.
- **timestep** (`torch.LongTensor`) --
  Used to indicate denoising step.
- **timestep_cond** (`torch.Tensor`, *optional*) --
  Conditional embeddings for timestep. If provided, the embeddings will be summed with the samples passed
  through the `self.time_embedding` layer to obtain the final timestep embeddings.
- **ofs** (`torch.Tensor`, *optional*) --
  Offset embeddings used in CogVideoX-5b-I2V.
- **image_rotary_emb** (`tuple` of `torch.Tensor`, *optional*) --
  Pre-computed rotary positional embeddings.
- **attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain
  tuple.0If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [CogVideoXTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/cogvideox_transformer3d#diffusers.CogVideoXTransformer3DModel) forward method.

**Parameters:**

num_attention_heads (`int`, defaults to `30`) : The number of heads to use for multi-head attention.

attention_head_dim (`int`, defaults to `64`) : The number of channels in each head.

in_channels (`int`, defaults to `16`) : The number of channels in the input.

out_channels (`int`, *optional*, defaults to `16`) : The number of channels in the output.

flip_sin_to_cos (`bool`, defaults to `True`) : Whether to flip the sin to cos in the time embedding.

time_embed_dim (`int`, defaults to `512`) : Output dimension of timestep embeddings.

ofs_embed_dim (`int`, defaults to `512`) : Output dimension of "ofs" embeddings used in CogVideoX-5b-I2B in version 1.5

text_embed_dim (`int`, defaults to `4096`) : Input dimension of text embeddings from the text encoder.

num_layers (`int`, defaults to `30`) : The number of layers of Transformer blocks to use.

dropout (`float`, defaults to `0.0`) : The dropout probability to use.

attention_bias (`bool`, defaults to `True`) : Whether to use bias in the attention projection layers.

sample_width (`int`, defaults to `90`) : The width of the input latents.

sample_height (`int`, defaults to `60`) : The height of the input latents.

sample_frames (`int`, defaults to `49`) : The number of frames in the input latents. Note that this parameter was incorrectly initialized to 49 instead of 13 because CogVideoX processed 13 latent frames at once in its default and recommended settings, but cannot be changed to the correct value to ensure backwards compatibility. To create a transformer with K latent frames, the correct value to pass here would be: ((K - 1) * temporal_compression_ratio + 1).

patch_size (`int`, defaults to `2`) : The size of the patches to use in the patch embedding layer.

temporal_compression_ratio (`int`, defaults to `4`) : The compression ratio across the temporal dimension. See documentation for `sample_frames`.

max_text_seq_length (`int`, defaults to `226`) : The maximum sequence length of the input text embeddings.

activation_fn (`str`, defaults to `"gelu-approximate"`) : Activation function to use in feed-forward.

timestep_activation_fn (`str`, defaults to `"silu"`) : Activation function to use when generating the timestep embeddings.

norm_elementwise_affine (`bool`, defaults to `True`) : Whether to use elementwise affine in normalization layers.

norm_eps (`float`, defaults to `1e-5`) : The epsilon value to use in normalization layers.

spatial_interpolation_scale (`float`, defaults to `1.875`) : Scaling factor to apply in 3D positional embeddings across spatial dimensions.

temporal_interpolation_scale (`float`, defaults to `1.0`) : Scaling factor to apply in 3D positional embeddings across temporal dimensions.

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.
#### fuse_qkv_projections[[diffusers.CogVideoXTransformer3DModel.fuse_qkv_projections]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/cogvideox_transformer_3d.py#L335)

Enables fused QKV projections. For self-attention modules, all projection matrices (i.e., query, key, value)
are fused. For cross-attention modules, key and value projection matrices are fused.

> [!WARNING] > This API is 🧪 experimental.
#### unfuse_qkv_projections[[diffusers.CogVideoXTransformer3DModel.unfuse_qkv_projections]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/cogvideox_transformer_3d.py#L357)

Disables the fused QKV projection if enabled.

> [!WARNING] > This API is 🧪 experimental.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_outputs.py#L21)

The output of [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel).

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.
