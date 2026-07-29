# MotifVideoTransformer3DModel

A Diffusion Transformer model for 3D video-like data was introduced in Motif-Video by the Motif Technologies Team.

The model uses a three-stage architecture with 12 dual-stream + 16 single-stream + 8 DDT decoder layers and rotary positional embeddings (RoPE) for video generation.

The model can be loaded with the following code snippet.

```python
from diffusers import MotifVideoTransformer3DModel

transformer = MotifVideoTransformer3DModel.from_pretrained("Motif-Technologies/Motif-Video-2B", subfolder="transformer", torch_dtype=torch.bfloat16)
```

## MotifVideoTransformer3DModel[[diffusers.MotifVideoTransformer3DModel]]

#### diffusers.MotifVideoTransformer3DModel[[diffusers.MotifVideoTransformer3DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_motif_video.py#L730)

A Transformer model for video-like data used in the Motif-Video model.

forwarddiffusers.MotifVideoTransformer3DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_motif_video.py#L888[{"name": "hidden_states", "val": ": Tensor"}, {"name": "timestep", "val": ": LongTensor"}, {"name": "encoder_hidden_states", "val": ": Tensor"}, {"name": "encoder_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "image_embeds", "val": ": torch.Tensor | None = None"}, {"name": "attention_kwargs", "val": ": typing.Optional[typing.Dict[str, typing.Any]] = None"}, {"name": "return_dict", "val": ": bool = True"}]- **hidden_states** (`torch.Tensor`) --
  Input latent tensor of shape `(batch_size, channels, num_frames, height, width)`.
- **timestep** (`torch.LongTensor`) --
  Diffusion timesteps of shape `(batch_size,)`.
- **encoder_hidden_states** (`torch.Tensor`) --
  Text conditioning of shape `(batch_size, sequence_length, embed_dim)`.
- **encoder_attention_mask** (`torch.Tensor`) --
  Mask for text conditioning of shape `(batch_size, sequence_length)`.
- **image_embeds** (`torch.Tensor`, *optional*) --
  Image embeddings from vision encoder of shape `(batch_size, num_tokens, embed_dim)`.
- **attention_kwargs** (`dict`, *optional*) --
  Additional arguments for attention processors.
- **return_dict** (`bool`, defaults to `True`) --
  Whether to return a [Transformer2DModelOutput](/docs/diffusers/v0.39.0/en/api/models/sana_video_transformer3d#diffusers.models.modeling_outputs.Transformer2DModelOutput).0[Transformer2DModelOutput](/docs/diffusers/v0.39.0/en/api/models/sana_video_transformer3d#diffusers.models.modeling_outputs.Transformer2DModelOutput) or `tuple`The predicted samples.

Forward pass of the MotifVideoTransformer3DModel.

**Parameters:**

in_channels (`int`, defaults to `33`) : The number of channels in the input.

out_channels (`int`, defaults to `16`) : The number of channels in the output.

num_attention_heads (`int`, defaults to `24`) : The number of heads to use for multi-head attention.

attention_head_dim (`int`, defaults to `128`) : The number of channels in each head.

num_layers (`int`, defaults to `20`) : The number of layers of dual-stream blocks to use.

num_single_layers (`int`, defaults to `40`) : The number of layers of single-stream blocks to use.

num_decoder_layers (`int`, defaults to `0`) : The number of decoder layers in single-stream blocks.

mlp_ratio (`float`, defaults to `4.0`) : The ratio of the hidden layer size to the input size in the feedforward network.

patch_size (`int`, defaults to `2`) : The size of the spatial patches to use in the patch embedding layer.

patch_size_t (`int`, defaults to `1`) : The size of the temporal patches to use in the patch embedding layer.

qk_norm (`str`, defaults to `rms_norm`) : The normalization to use for the query and key projections in the attention layers.

text_embed_dim (`int`, defaults to `4096`) : Input dimension of text embeddings from the text encoder.

image_embed_dim (`int`, *optional*) : Input dimension of image embeddings from a vision encoder. If provided, enables image conditioning.

rope_theta (`float`, defaults to `256.0`) : The value of theta to use in the RoPE layer.

rope_axes_dim (`Tuple[int]`, defaults to `(16, 56, 56)`) : The dimensions of the axes to use in the RoPE layer.

**Returns:**

`[Transformer2DModelOutput](/docs/diffusers/v0.39.0/en/api/models/sana_video_transformer3d#diffusers.models.modeling_outputs.Transformer2DModelOutput) or `tuple``

The predicted samples.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_outputs.py#L21)

The output of [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel).

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.
