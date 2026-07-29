# SD3 Transformer Model

The Transformer model introduced in [Stable Diffusion 3](https://hf.co/papers/2403.03206). Its novelty lies in the MMDiT transformer block.

## SD3Transformer2DModel[[diffusers.SD3Transformer2DModel]]

#### diffusers.SD3Transformer2DModel[[diffusers.SD3Transformer2DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_sd3.py#L79)

The Transformer model introduced in [Stable Diffusion 3](https://huggingface.co/papers/2403.03206).

enable_forward_chunkingdiffusers.SD3Transformer2DModel.enable_forward_chunkinghttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_sd3.py#L175[{"name": "chunk_size", "val": ": int | None = None"}, {"name": "dim", "val": ": int = 0"}]- **chunk_size** (`int`, *optional*) --
  The chunk size of the feed-forward layers. If not specified, will run feed-forward layer individually
  over each tensor of dim=`dim`.
- **dim** (`int`, *optional*, defaults to `0`) --
  The dimension over which the feed-forward computation should be chunked. Choose between dim=0 (batch)
  or dim=1 (sequence length).0

Sets the attention processor to use [feed forward
chunking](https://huggingface.co/blog/reformer#2-chunked-feed-forward-layers).

**Parameters:**

sample_size (`int`, defaults to `128`) : The width/height of the latents. This is fixed during training since it is used to learn a number of position embeddings.

patch_size (`int`, defaults to `2`) : Patch size to turn the input data into small patches.

in_channels (`int`, defaults to `16`) : The number of latent channels in the input.

num_layers (`int`, defaults to `18`) : The number of layers of transformer blocks to use.

attention_head_dim (`int`, defaults to `64`) : The number of channels in each head.

num_attention_heads (`int`, defaults to `18`) : The number of heads to use for multi-head attention.

joint_attention_dim (`int`, defaults to `4096`) : The embedding dimension to use for joint text-image attention.

caption_projection_dim (`int`, defaults to `1152`) : The embedding dimension of caption embeddings.

pooled_projection_dim (`int`, defaults to `2048`) : The embedding dimension of pooled text projections.

out_channels (`int`, defaults to `16`) : The number of latent channels in the output.

pos_embed_max_size (`int`, defaults to `96`) : The maximum latent height/width of positional embeddings.

dual_attention_layers (`tuple[int, ...]`, defaults to `()`) : The number of dual-stream transformer blocks to use.

qk_norm (`str`, *optional*, defaults to `None`) : The normalization to use for query and key in the attention layer. If `None`, no normalization is used.
#### forward[[diffusers.SD3Transformer2DModel.forward]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_sd3.py#L248)

The [SD3Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/sd3_transformer2d#diffusers.SD3Transformer2DModel) forward method.

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch size, channel, height, width)`) : Input `hidden_states`.

encoder_hidden_states (`torch.Tensor` of shape `(batch size, sequence_len, embed_dims)`) : Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.

pooled_projections (`torch.Tensor` of shape `(batch_size, projection_dim)`) : Embeddings projected from the embeddings of input conditions.

timestep (`torch.LongTensor`) : Used to indicate denoising step.

block_controlnet_hidden_states (`list` of `torch.Tensor`) : A list of tensors that if specified are added to the residuals of transformer blocks.

joint_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

skip_layers (`list` of `int`, *optional*) : A list of layer indices to skip during the forward pass.

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.
#### fuse_qkv_projections[[diffusers.SD3Transformer2DModel.fuse_qkv_projections]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_sd3.py#L217)

Enables fused QKV projections. For self-attention modules, all projection matrices (i.e., query, key, value)
are fused. For cross-attention modules, key and value projection matrices are fused.

> [!WARNING] > This API is 🧪 experimental.
#### unfuse_qkv_projections[[diffusers.SD3Transformer2DModel.unfuse_qkv_projections]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_sd3.py#L239)

Disables the fused QKV projection if enabled.

> [!WARNING] > This API is 🧪 experimental.
