# ChromaTransformer2DModel

A modified flux Transformer model from [Chroma](https://huggingface.co/lodestones/Chroma1-HD)

## ChromaTransformer2DModel[[diffusers.ChromaTransformer2DModel]]

#### diffusers.ChromaTransformer2DModel[[diffusers.ChromaTransformer2DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_chroma.py#L370)

The Transformer model introduced in Flux, modified for Chroma.

Reference: https://huggingface.co/lodestones/Chroma1-HD

forwarddiffusers.ChromaTransformer2DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_chroma.py#L476[{"name": "hidden_states", "val": ": Tensor"}, {"name": "encoder_hidden_states", "val": ": Tensor = None"}, {"name": "timestep", "val": ": LongTensor = None"}, {"name": "img_ids", "val": ": Tensor = None"}, {"name": "txt_ids", "val": ": Tensor = None"}, {"name": "attention_mask", "val": ": Tensor = None"}, {"name": "joint_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "controlnet_block_samples", "val": " = None"}, {"name": "controlnet_single_block_samples", "val": " = None"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "controlnet_blocks_repeat", "val": ": bool = False"}]- **hidden_states** (`torch.Tensor` of shape `(batch_size, image_sequence_length, in_channels)`) --
  Input `hidden_states`.
- **encoder_hidden_states** (`torch.Tensor` of shape `(batch_size, text_sequence_length, joint_attention_dim)`) --
  Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.
- **timestep** ( `torch.LongTensor`) --
  Used to indicate denoising step.
- **img_ids** (`torch.Tensor`) --
  Image position ids used to compute the rotary positional embeddings.
- **txt_ids** (`torch.Tensor`) --
  Text position ids used to compute the rotary positional embeddings.
- **attention_mask** (`torch.Tensor`, *optional*) --
  Mask applied to `encoder_hidden_states` during attention.
- **controlnet_block_samples** (`list` of `torch.Tensor`, *optional*) --
  A list of tensors that if specified are added to the residuals of transformer blocks.
- **controlnet_single_block_samples** (`list` of `torch.Tensor`, *optional*) --
  A list of tensors that if specified are added to the residuals of single transformer blocks.
- **controlnet_blocks_repeat** (`bool`, *optional*, defaults to `False`) --
  Whether to repeat the controlnet block samples across all transformer blocks.
- **joint_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain
  tuple.0If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [FluxTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/flux_transformer#diffusers.FluxTransformer2DModel) forward method.

**Parameters:**

patch_size (`int`, defaults to `1`) : Patch size to turn the input data into small patches.

in_channels (`int`, defaults to `64`) : The number of channels in the input.

out_channels (`int`, *optional*, defaults to `None`) : The number of channels in the output. If not specified, it defaults to `in_channels`.

num_layers (`int`, defaults to `19`) : The number of layers of dual stream DiT blocks to use.

num_single_layers (`int`, defaults to `38`) : The number of layers of single stream DiT blocks to use.

attention_head_dim (`int`, defaults to `128`) : The number of dimensions to use for each attention head.

num_attention_heads (`int`, defaults to `24`) : The number of attention heads to use.

joint_attention_dim (`int`, defaults to `4096`) : The number of dimensions to use for the joint attention (embedding/channel dimension of `encoder_hidden_states`).

axes_dims_rope (`tuple[int]`, defaults to `(16, 56, 56)`) : The dimensions to use for the rotary positional embeddings.

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.
