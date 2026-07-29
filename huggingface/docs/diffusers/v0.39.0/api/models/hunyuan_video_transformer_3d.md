# HunyuanVideoTransformer3DModel

A Diffusion Transformer model for 3D video-like data was introduced in [HunyuanVideo: A Systematic Framework For Large Video Generative Models](https://huggingface.co/papers/2412.03603) by Tencent.

The model can be loaded with the following code snippet.

```python
from diffusers import HunyuanVideoTransformer3DModel

transformer = HunyuanVideoTransformer3DModel.from_pretrained("hunyuanvideo-community/HunyuanVideo", subfolder="transformer", torch_dtype=torch.bfloat16)
```

## HunyuanVideoTransformer3DModel[[diffusers.HunyuanVideoTransformer3DModel]]

#### diffusers.HunyuanVideoTransformer3DModel[[diffusers.HunyuanVideoTransformer3DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_hunyuan_video.py#L841)

A Transformer model for video-like data used in [HunyuanVideo](https://huggingface.co/tencent/HunyuanVideo).

forwarddiffusers.HunyuanVideoTransformer3DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_hunyuan_video.py#L994[{"name": "hidden_states", "val": ": Tensor"}, {"name": "timestep", "val": ": LongTensor"}, {"name": "encoder_hidden_states", "val": ": Tensor"}, {"name": "encoder_attention_mask", "val": ": Tensor"}, {"name": "pooled_projections", "val": ": Tensor"}, {"name": "guidance", "val": ": Tensor = None"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "return_dict", "val": ": bool = True"}]- **hidden_states** (`torch.Tensor` of shape `(batch_size, num_channels, num_frames, height, width)`) --
  Input `hidden_states`.
- **timestep** (`torch.LongTensor`) --
  Used to indicate denoising step.
- **encoder_hidden_states** (`torch.Tensor` of shape `(batch_size, sequence_len, embed_dims)`) --
  Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.
- **encoder_attention_mask** (`torch.Tensor`) --
  Mask applied to `encoder_hidden_states` during attention.
- **pooled_projections** (`torch.Tensor` of shape `(batch_size, projection_dim)`) --
  Embeddings projected from the embeddings of input conditions.
- **guidance** (`torch.Tensor`, *optional*) --
  Guidance scale embedding used for guidance-distilled variants of the model.
- **attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain
  tuple.0If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [HunyuanVideoTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/hunyuan_video_transformer_3d#diffusers.HunyuanVideoTransformer3DModel) forward method.

**Parameters:**

in_channels (`int`, defaults to `16`) : The number of channels in the input.

out_channels (`int`, defaults to `16`) : The number of channels in the output.

num_attention_heads (`int`, defaults to `24`) : The number of heads to use for multi-head attention.

attention_head_dim (`int`, defaults to `128`) : The number of channels in each head.

num_layers (`int`, defaults to `20`) : The number of layers of dual-stream blocks to use.

num_single_layers (`int`, defaults to `40`) : The number of layers of single-stream blocks to use.

num_refiner_layers (`int`, defaults to `2`) : The number of layers of refiner blocks to use.

mlp_ratio (`float`, defaults to `4.0`) : The ratio of the hidden layer size to the input size in the feedforward network.

patch_size (`int`, defaults to `2`) : The size of the spatial patches to use in the patch embedding layer.

patch_size_t (`int`, defaults to `1`) : The size of the tmeporal patches to use in the patch embedding layer.

qk_norm (`str`, defaults to `rms_norm`) : The normalization to use for the query and key projections in the attention layers.

guidance_embeds (`bool`, defaults to `True`) : Whether to use guidance embeddings in the model.

text_embed_dim (`int`, defaults to `4096`) : Input dimension of text embeddings from the text encoder.

pooled_projection_dim (`int`, defaults to `768`) : The dimension of the pooled projection of the text embeddings.

rope_theta (`float`, defaults to `256.0`) : The value of theta to use in the RoPE layer.

rope_axes_dim (`tuple[int]`, defaults to `(16, 56, 56)`) : The dimensions of the axes to use in the RoPE layer.

image_condition_type (`str`, *optional*, defaults to `None`) : The type of image conditioning to use. If `None`, no image conditioning is used. If `latent_concat`, the image is concatenated to the latent stream. If `token_replace`, the image is used to replace first-frame tokens in the latent stream and apply conditioning.

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_outputs.py#L21)

The output of [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel).

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.
