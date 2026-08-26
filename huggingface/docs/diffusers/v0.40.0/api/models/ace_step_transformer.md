# AceStepTransformer1DModel

A 1D Diffusion Transformer for music generation from [ACE-Step 1.5](https://github.com/ace-step/ACE-Step-1.5). The model operates on the 25 Hz stereo latents produced by [AutoencoderOobleck](/docs/diffusers/v0.40.0/en/api/models/autoencoder_oobleck#diffusers.AutoencoderOobleck) using flow matching, and is trained with a Qwen3-derived backbone (grouped-query attention, rotary position embedding, RMSNorm, AdaLN-Zero timestep conditioning) plus cross-attention to the text / lyric / timbre conditions built by `AceStepConditionEncoder`.

## AceStepTransformer1DModel[[diffusers.AceStepTransformer1DModel]]

#### diffusers.AceStepTransformer1DModel[[diffusers.AceStepTransformer1DModel]]

```python
diffusers.AceStepTransformer1DModel(hidden_size: int = 2048, intermediate_size: int = 6144, num_hidden_layers: int = 24, num_attention_heads: int = 16, num_key_value_heads: int = 8, head_dim: int = 128, in_channels: int = 192, audio_acoustic_hidden_dim: int = 64, patch_size: int = 2, rope_theta: float = 1000000.0, attention_bias: bool = False, attention_dropout: float = 0.0, rms_norm_eps: float = 1e-06, sliding_window: int = 128, layer_types: typing.Optional[typing.List[str]] = None, encoder_hidden_size: typing.Optional[int] = None, is_turbo: bool = False, model_version: typing.Optional[str] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/ace_step_transformer.py#L432)

Diffusion Transformer for ACE-Step 1.5 music generation.

Generates audio latents conditioned on text, lyrics, and timbre. Uses 1D patch embedding (`Conv1d` with stride
`patch_size`) followed by a stack of `AceStepTransformerBlock`s with alternating sliding-window / full attention on
the self-attention branch. Cross-attention consumes the packed `encoder_hidden_states` produced by
`AceStepConditionEncoder`.

#### forward[[diffusers.AceStepTransformer1DModel.forward]]

```python
forward(hidden_states: Tensor, timestep: Tensor, timestep_r: Tensor, encoder_hidden_states: Tensor, context_latents: Tensor, attention_kwargs: typing.Optional[dict] = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/ace_step_transformer.py#L532)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch_size, seq_len, channels)`) : Noisy latent input for the diffusion process.

timestep (`torch.Tensor` of shape `(batch_size,)`) : Current diffusion timestep `t`.

timestep_r (`torch.Tensor` of shape `(batch_size,)`) : Reference timestep `r` (set equal to `t` for standard inference).

encoder_hidden_states (`torch.Tensor` of shape `(batch_size, encoder_seq_len, hidden_size)`) : Conditioning embeddings from the condition encoder (text + lyrics + timbre).

context_latents (`torch.Tensor` of shape `(batch_size, seq_len, context_dim)`) : Context latents (source latents concatenated with chunk masks) — fed to the patchify conv alongside `hidden_states`.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary passed along to the `AttentionProcessor`. Used to pass the LoRA scale via `{"scale": float}`.

return_dict (`bool`, defaults to `True`) : Whether to return a `Transformer2DModelOutput` or a plain tuple.

**Returns:** `Transformer2DModelOutput` or `tuple`

The predicted velocity field.

The [AceStepTransformer1DModel](/docs/diffusers/v0.40.0/en/api/models/ace_step_transformer#diffusers.AceStepTransformer1DModel) forward method.
