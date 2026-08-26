# MiniMaxMusic3Transformer1DModel

The 2.4B flow-matching Diffusion Transformer of [MiniMax Music 3](https://huggingface.co/MiniMaxAI/MiniMax-Music3). It
denoises 128-channel Flow-VAE audio latents conditioned on the per-frame hidden states of the model's autoregressive
language-model stage, prepending the flow-matching timestep as an extra sequence token (a Stable-Audio-lineage
continuous transformer with partial rotary attention and GLU feedforwards).

## MiniMaxMusic3Transformer1DModel[[diffusers.MiniMaxMusic3Transformer1DModel]]

#### diffusers.MiniMaxMusic3Transformer1DModel[[diffusers.MiniMaxMusic3Transformer1DModel]]

```python
diffusers.MiniMaxMusic3Transformer1DModel(in_channels: int = 128, condition_dim: int = 2048, num_layers: int = 36, num_attention_heads: int = 32, attention_head_dim: int = 64, ff_inner_dim: int = 8192, rotary_dim: int = 32, fourier_embedding_dim: int = 256)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_minimax_music3.py#L147)

The flow-matching diffusion transformer of MiniMax Music 3. It denoises Flow-VAE audio latents conditioned on
per-frame hidden states produced by the autoregressive language-model stage.

Inputs are 1D latent sequences of shape `(batch, in_channels, length)`. The conditioning signal
(`encoder_hidden_states`, shape `(batch, length, condition_dim)`) must already be aligned to the latent timeline —
see `MiniMaxMusic3ConditionEncoder`. The flow-matching `timestep` runs from 0 (noise) to 1 (data).

#### forward[[diffusers.MiniMaxMusic3Transformer1DModel.forward]]

```python
forward(hidden_states: Tensor, timestep: Tensor, encoder_hidden_states: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_minimax_music3.py#L196)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch, in_channels, length)`) : Noisy Flow-VAE latents.

timestep (`torch.Tensor` of shape `(batch,)`) : Flow-matching time in `[0, 1]`, where 0 is pure noise and 1 is data.

encoder_hidden_states (`torch.Tensor` of shape `(batch, length, condition_dim)`) : Frame-aligned conditioning from `MiniMaxMusic3ConditionEncoder`. Pass zeros for the unconditional branch of classifier-free guidance.

return_dict (`bool`, defaults to `True`) : Whether to return a [Transformer2DModelOutput](/docs/diffusers/v0.40.0/en/api/models/hunyuan_video15_transformer_3d#diffusers.models.modeling_outputs.Transformer2DModelOutput) instead of a plain tuple.

**Returns:**

The predicted flow-matching velocity with the same shape as `hidden_states`.
