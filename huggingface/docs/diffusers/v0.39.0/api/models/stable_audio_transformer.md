# StableAudioDiTModel

A Transformer model for audio waveforms from [Stable Audio Open](https://huggingface.co/papers/2407.14358).

## StableAudioDiTModel[[diffusers.StableAudioDiTModel]]

#### diffusers.StableAudioDiTModel[[diffusers.StableAudioDiTModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/stable_audio_transformer.py#L183)

The Diffusion Transformer model introduced in Stable Audio.

Reference: https://github.com/Stability-AI/stable-audio-tools

forwarddiffusers.StableAudioDiTModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/stable_audio_transformer.py#L282[{"name": "hidden_states", "val": ": FloatTensor"}, {"name": "timestep", "val": ": LongTensor = None"}, {"name": "encoder_hidden_states", "val": ": FloatTensor = None"}, {"name": "global_hidden_states", "val": ": FloatTensor = None"}, {"name": "rotary_embedding", "val": ": FloatTensor = None"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "attention_mask", "val": ": torch.LongTensor | None = None"}, {"name": "encoder_attention_mask", "val": ": torch.LongTensor | None = None"}]- **hidden_states** (`torch.FloatTensor` of shape `(batch size, in_channels, sequence_len)`) --
  Input `hidden_states`.
- **timestep** ( `torch.LongTensor`) --
  Used to indicate denoising step.
- **encoder_hidden_states** (`torch.FloatTensor` of shape `(batch size, encoder_sequence_len, cross_attention_input_dim)`) --
  Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.
- **global_hidden_states** (`torch.FloatTensor` of shape `(batch size, global_sequence_len, global_states_input_dim)`) --
  Global embeddings that will be prepended to the hidden states.
- **rotary_embedding** (`torch.Tensor`) --
  The rotary embeddings to apply on query and key tensors during attention calculation.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain
  tuple.
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_len)`, *optional*) --
  Mask to avoid performing attention on padding token indices, formed by concatenating the attention
  masks
  for the two text encoders together. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
- **encoder_attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_len)`, *optional*) --
  Mask to avoid performing attention on padding token cross-attention indices, formed by concatenating
  the attention masks
  for the two text encoders together. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.0If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [StableAudioDiTModel](/docs/diffusers/v0.39.0/en/api/models/stable_audio_transformer#diffusers.StableAudioDiTModel) forward method.

**Parameters:**

sample_size ( `int`, *optional*, defaults to 1024) : The size of the input sample.

in_channels (`int`, *optional*, defaults to 64) : The number of channels in the input.

num_layers (`int`, *optional*, defaults to 24) : The number of layers of Transformer blocks to use.

attention_head_dim (`int`, *optional*, defaults to 64) : The number of channels in each head.

num_attention_heads (`int`, *optional*, defaults to 24) : The number of heads to use for the query states.

num_key_value_attention_heads (`int`, *optional*, defaults to 12) : The number of heads to use for the key and value states.

out_channels (`int`, defaults to 64) : Number of output channels.

cross_attention_dim ( `int`, *optional*, defaults to 768) : Dimension of the cross-attention projection.

time_proj_dim ( `int`, *optional*, defaults to 256) : Dimension of the timestep inner projection.

global_states_input_dim ( `int`, *optional*, defaults to 1536) : Input dimension of the global hidden states projection.

cross_attention_input_dim ( `int`, *optional*, defaults to 768) : Input dimension of the cross-attention projection

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.
#### set_default_attn_processor[[diffusers.StableAudioDiTModel.set_default_attn_processor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/stable_audio_transformer.py#L276)

Disables custom attention processors and sets the default attention implementation.
