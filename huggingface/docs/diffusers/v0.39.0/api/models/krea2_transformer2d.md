# Krea2Transformer2DModel

The single-stream MMDiT flow-matching transformer used by [Krea 2](https://github.com/krea-ai/krea-2).

## Krea2Transformer2DModel[[diffusers.Krea2Transformer2DModel]]

#### diffusers.Krea2Transformer2DModel[[diffusers.Krea2Transformer2DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_krea2.py#L330)

The single-stream MMDiT flow-matching backbone used by the Krea 2 pipeline.

Text conditioning enters as a stack of hidden states tapped from several layers of a multimodal text encoder. A
small text-fusion transformer collapses the layer axis and refines the token sequence; the result is concatenated
with the patchified image latents into a single `[text, image]` sequence processed by the transformer blocks. The
timestep conditions every block through one shared modulation vector plus per-block learned tables.

forwarddiffusers.Krea2Transformer2DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_krea2.py#L447[{"name": "hidden_states", "val": ": Tensor"}, {"name": "encoder_hidden_states", "val": ": Tensor"}, {"name": "timestep", "val": ": Tensor"}, {"name": "position_ids", "val": ": Tensor"}, {"name": "encoder_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "return_dict", "val": ": bool = True"}]- **hidden_states** (`torch.Tensor` of shape `(batch_size, image_seq_len, in_channels)`) --
  Packed (patchified) noisy image latents.
- **encoder_hidden_states** (`torch.Tensor` of shape `(batch_size, text_seq_len, num_text_layers, text_hidden_dim)`) --
  Stack of tapped text-encoder hidden states per token.
- **timestep** (`torch.Tensor` of shape `(batch_size,)`) --
  Flow-matching time in `[0, 1]` (1 is pure noise, 0 is clean data).
- **position_ids** (`torch.Tensor` of shape `(text_seq_len + image_seq_len, 3)`) --
  `(t, h, w)` rotary coordinates for the combined sequence. Text rows are all-zero; image rows hold the
  latent-grid coordinates.
- **encoder_attention_mask** (`torch.Tensor` of shape `(batch_size, text_seq_len)`, *optional*) --
  Boolean mask marking valid text tokens. Pass `None` when every text token is valid.
- **attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that, when it contains a `scale` entry, sets the LoRA scale applied to this
  transformer's adapters for the duration of the forward pass.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether to return a [Transformer2DModelOutput](/docs/diffusers/v0.39.0/en/api/models/sana_video_transformer3d#diffusers.models.modeling_outputs.Transformer2DModelOutput) instead of a plain tuple.0[Transformer2DModelOutput](/docs/diffusers/v0.39.0/en/api/models/sana_video_transformer3d#diffusers.models.modeling_outputs.Transformer2DModelOutput) or a `tuple` whose first element is the velocity
tensor of shape `(batch_size, image_seq_len, in_channels)`.

Predict the flow-matching velocity for the image tokens.

**Parameters:**

in_channels (`int`, defaults to 64) : Latent channel count after patchification (`vae_channels * patch_size ** 2`).

num_layers (`int`, defaults to 28) : Number of transformer blocks.

attention_head_dim (`int`, defaults to 128) : Dimension of each attention head; the total hidden size is `attention_head_dim * num_attention_heads`.

num_attention_heads (`int`, defaults to 48) : Number of query heads.

num_key_value_heads (`int`, defaults to 12) : Number of key/value heads for grouped-query attention.

intermediate_size (`int`, defaults to 16384) : Feed-forward hidden size of the SwiGLU MLP inside each block.

timestep_embed_dim (`int`, defaults to 256) : Width of the sinusoidal timestep embedding before its MLP.

text_hidden_dim (`int`, defaults to 2560) : Hidden size of the text encoder whose hidden states are consumed.

num_text_layers (`int`, defaults to 12) : Number of tapped text-encoder hidden states stacked per token.

text_num_attention_heads (`int`, defaults to 20) : Number of query heads in the text fusion blocks.

text_num_key_value_heads (`int`, defaults to 20) : Number of key/value heads in the text fusion blocks.

text_intermediate_size (`int`, defaults to 6912) : Feed-forward hidden size of the SwiGLU MLP inside the text fusion blocks.

num_layerwise_text_blocks (`int`, defaults to 2) : Number of text fusion blocks applied across the tapped-layer axis (per token).

num_refiner_text_blocks (`int`, defaults to 2) : Number of text fusion blocks applied across the token sequence.

axes_dims_rope (`tuple[int, int, int]`, defaults to `(32, 48, 48)`) : Head-dim split across the (t, h, w) rotary position axes.

rope_theta (`float`, defaults to 1000.0) : Base used by the rotary position embedding.

norm_eps (`float`, defaults to 1e-5) : Epsilon used by all RMSNorm modules.

**Returns:**

[Transformer2DModelOutput](/docs/diffusers/v0.39.0/en/api/models/sana_video_transformer3d#diffusers.models.modeling_outputs.Transformer2DModelOutput) or a `tuple` whose first element is the velocity
tensor of shape `(batch_size, image_seq_len, in_channels)`.
