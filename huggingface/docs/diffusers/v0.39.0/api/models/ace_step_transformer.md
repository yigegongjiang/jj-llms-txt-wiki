# AceStepTransformer1DModel

A 1D Diffusion Transformer for music generation from [ACE-Step 1.5](https://github.com/ace-step/ACE-Step-1.5). The model operates on the 25 Hz stereo latents produced by [AutoencoderOobleck](/docs/diffusers/v0.39.0/en/api/models/autoencoder_oobleck#diffusers.AutoencoderOobleck) using flow matching, and is trained with a Qwen3-derived backbone (grouped-query attention, rotary position embedding, RMSNorm, AdaLN-Zero timestep conditioning) plus cross-attention to the text / lyric / timbre conditions built by `AceStepConditionEncoder`.

## AceStepTransformer1DModel[[diffusers.AceStepTransformer1DModel]]

#### diffusers.AceStepTransformer1DModel[[diffusers.AceStepTransformer1DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/ace_step_transformer.py#L431)

Diffusion Transformer for ACE-Step 1.5 music generation.

Generates audio latents conditioned on text, lyrics, and timbre. Uses 1D patch embedding (`Conv1d` with stride
`patch_size`) followed by a stack of `AceStepTransformerBlock`s with alternating sliding-window / full attention on
the self-attention branch. Cross-attention consumes the packed `encoder_hidden_states` produced by
`AceStepConditionEncoder`.

forwarddiffusers.AceStepTransformer1DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/ace_step_transformer.py#L531[{"name": "hidden_states", "val": ": Tensor"}, {"name": "timestep", "val": ": Tensor"}, {"name": "timestep_r", "val": ": Tensor"}, {"name": "encoder_hidden_states", "val": ": Tensor"}, {"name": "context_latents", "val": ": Tensor"}, {"name": "return_dict", "val": ": bool = True"}]- **hidden_states** (`torch.Tensor` of shape `(batch_size, seq_len, channels)`) --
  Noisy latent input for the diffusion process.
- **timestep** (`torch.Tensor` of shape `(batch_size,)`) --
  Current diffusion timestep `t`.
- **timestep_r** (`torch.Tensor` of shape `(batch_size,)`) --
  Reference timestep `r` (set equal to `t` for standard inference).
- **encoder_hidden_states** (`torch.Tensor` of shape `(batch_size, encoder_seq_len, hidden_size)`) --
  Conditioning embeddings from the condition encoder (text + lyrics + timbre).
- **context_latents** (`torch.Tensor` of shape `(batch_size, seq_len, context_dim)`) --
  Context latents (source latents concatenated with chunk masks) — fed to the patchify conv alongside
  `hidden_states`.
- **return_dict** (`bool`, defaults to `True`) --
  Whether to return a `Transformer2DModelOutput` or a plain tuple.0`Transformer2DModelOutput` or `tuple`The predicted velocity field.
The [AceStepTransformer1DModel](/docs/diffusers/v0.39.0/en/api/models/ace_step_transformer#diffusers.AceStepTransformer1DModel) forward method.

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch_size, seq_len, channels)`) : Noisy latent input for the diffusion process.

timestep (`torch.Tensor` of shape `(batch_size,)`) : Current diffusion timestep `t`.

timestep_r (`torch.Tensor` of shape `(batch_size,)`) : Reference timestep `r` (set equal to `t` for standard inference).

encoder_hidden_states (`torch.Tensor` of shape `(batch_size, encoder_seq_len, hidden_size)`) : Conditioning embeddings from the condition encoder (text + lyrics + timbre).

context_latents (`torch.Tensor` of shape `(batch_size, seq_len, context_dim)`) : Context latents (source latents concatenated with chunk masks) — fed to the patchify conv alongside `hidden_states`.

return_dict (`bool`, defaults to `True`) : Whether to return a `Transformer2DModelOutput` or a plain tuple.

**Returns:**

``Transformer2DModelOutput` or `tuple``

The predicted velocity field.
