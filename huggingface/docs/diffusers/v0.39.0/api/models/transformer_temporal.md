# TransformerTemporalModel

A Transformer model for video-like data.

## TransformerTemporalModel[[diffusers.TransformerTemporalModel]]

#### diffusers.TransformerTemporalModel[[diffusers.TransformerTemporalModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_temporal.py#L41)

A Transformer model for video-like data.

forwarddiffusers.TransformerTemporalModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_temporal.py#L123[{"name": "hidden_states", "val": ": Tensor"}, {"name": "encoder_hidden_states", "val": ": torch.LongTensor | None = None"}, {"name": "timestep", "val": ": torch.LongTensor | None = None"}, {"name": "class_labels", "val": ": LongTensor = None"}, {"name": "num_frames", "val": ": int = 1"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "return_dict", "val": ": bool = True"}]- **hidden_states** (`torch.LongTensor` of shape `(batch size, num latent pixels)` if discrete, `torch.Tensor` of shape `(batch size, channel, height, width)` if continuous) --
  Input hidden_states.
- **encoder_hidden_states** ( `torch.LongTensor` of shape `(batch size, encoder_hidden_states dim)`, *optional*) --
  Conditional embeddings for cross attention layer. If not given, cross-attention defaults to
  self-attention.
- **timestep** ( `torch.LongTensor`, *optional*) --
  Used to indicate denoising step. Optional timestep to be applied as an embedding in `AdaLayerNorm`.
- **class_labels** ( `torch.LongTensor` of shape `(batch size, num classes)`, *optional*) --
  Used to indicate class labels conditioning. Optional class labels to be applied as an embedding in
  `AdaLayerZeroNorm`.
- **num_frames** (`int`, *optional*, defaults to 1) --
  The number of frames to be processed per batch. This is used to reshape the hidden states.
- **cross_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a [TransformerTemporalModelOutput](/docs/diffusers/v0.39.0/en/api/models/transformer_temporal#diffusers.models.transformers.transformer_temporal.TransformerTemporalModelOutput)
  instead of a plain tuple.0[TransformerTemporalModelOutput](/docs/diffusers/v0.39.0/en/api/models/transformer_temporal#diffusers.models.transformers.transformer_temporal.TransformerTemporalModelOutput) or `tuple`If `return_dict` is True, an
[TransformerTemporalModelOutput](/docs/diffusers/v0.39.0/en/api/models/transformer_temporal#diffusers.models.transformers.transformer_temporal.TransformerTemporalModelOutput) is returned, otherwise a
`tuple` where the first element is the sample tensor.

The `TransformerTemporal` forward method.

**Parameters:**

num_attention_heads (`int`, *optional*, defaults to 16) : The number of heads to use for multi-head attention.

attention_head_dim (`int`, *optional*, defaults to 88) : The number of channels in each head.

in_channels (`int`, *optional*) : The number of channels in the input and output (specify if the input is **continuous**).

num_layers (`int`, *optional*, defaults to 1) : The number of layers of Transformer blocks to use.

dropout (`float`, *optional*, defaults to 0.0) : The dropout probability to use.

cross_attention_dim (`int`, *optional*) : The number of `encoder_hidden_states` dimensions to use.

attention_bias (`bool`, *optional*) : Configure if the `TransformerBlock` attention should contain a bias parameter.

sample_size (`int`, *optional*) : The width of the latent images (specify if the input is **discrete**). This is fixed during training since it is used to learn a number of position embeddings.

activation_fn (`str`, *optional*, defaults to `"geglu"`) : Activation function to use in feed-forward. See `diffusers.models.activations.get_activation` for supported activation functions.

norm_elementwise_affine (`bool`, *optional*) : Configure if the `TransformerBlock` should use learnable elementwise affine parameters for normalization.

double_self_attention (`bool`, *optional*) : Configure if each `TransformerBlock` should contain two self-attention layers.

positional_embeddings : (`str`, *optional*): The type of positional embeddings to apply to the sequence input before passing use.

num_positional_embeddings : (`int`, *optional*): The maximum length of the sequence over which to apply positional embeddings.

**Returns:**

`[TransformerTemporalModelOutput](/docs/diffusers/v0.39.0/en/api/models/transformer_temporal#diffusers.models.transformers.transformer_temporal.TransformerTemporalModelOutput) or `tuple``

If `return_dict` is True, an
[TransformerTemporalModelOutput](/docs/diffusers/v0.39.0/en/api/models/transformer_temporal#diffusers.models.transformers.transformer_temporal.TransformerTemporalModelOutput) is returned, otherwise a
`tuple` where the first element is the sample tensor.

## TransformerTemporalModelOutput[[diffusers.models.transformers.transformer_temporal.TransformerTemporalModelOutput]]

#### diffusers.models.transformers.transformer_temporal.TransformerTemporalModelOutput[[diffusers.models.transformers.transformer_temporal.TransformerTemporalModelOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_temporal.py#L29)

The output of [TransformerTemporalModel](/docs/diffusers/v0.39.0/en/api/models/transformer_temporal#diffusers.TransformerTemporalModel).

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size x num_frames, num_channels, height, width)`) : The hidden states output conditioned on `encoder_hidden_states` input.
