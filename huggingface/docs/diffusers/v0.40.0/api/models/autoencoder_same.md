# AutoencoderSAME

The **SAME** (Semantically-Aligned Music Encoder) autoencoder is used by [Stable Audio 3](https://stability.ai/news/stable-audio-3)
to compress stereo audio waveforms into a compact latent sequence and reconstruct them.

The encoder stacks `SAMETransformerResamplingBlock` modules, each of which groups a fixed number of audio
patch frames and produces one learnable output token via a differential transformer. The decoder inverts this
process, expanding each latent token back to a patch of audio frames.

A soft-norm bottleneck (`SoftNormBottleneck`) normalises latents before and after the diffusion model,
providing stable training dynamics.

## AutoencoderSAME[[diffusers.AutoencoderSAME]]

#### diffusers.AutoencoderSAME[[diffusers.AutoencoderSAME]]

```python
diffusers.AutoencoderSAME(audio_channels: int = 2, patch_size: int = 256, encoder_channels: int = 128, encoder_c_mults: typing.List[int] = (6,), encoder_strides: typing.List[int] = (16,), encoder_transformer_depths: typing.List[int] = (6,), latent_dim: int = 256, use_differential_attention: bool = True, dim_heads: int = 64, ff_mult: int = 3, sliding_window: int = 1, encoder_sinusoidal_blocks: typing.List[int] = (0,), decoder_sinusoidal_blocks: typing.List[int] = (0,), sampling_rate: int = 44100)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_same.py#L608)

**Parameters:**

audio_channels : Number of audio channels (2 for stereo).

patch_size : Non-overlapping patch size applied before the TRB encoder (and reversed after the TRB decoder). Contributes `patch_size ×` to the total downsampling ratio. Production value: 256.

encoder_channels : Base channel count for the TRB. 128 for SAME-S, 256 for SAME-L.

encoder_c_mults : Channel multiplier for each TRB level (one entry per TRB). Both SAME-S and SAME-L use `(6,)` — a single TRB whose hidden dimension is `encoder_channels × 6`.

encoder_strides : Down-/up-sampling stride for each TRB level. Both SAME-S and SAME-L use `(16,)` — one TRB with stride 16.

encoder_transformer_depths : Transformer layers per TRB level. 6 for SAME-S, 12 for SAME-L.

latent_dim : Dimensionality of the latent space. 256 for both variants.

use_differential_attention : If `True`, use differential attention inside each TRB transformer block (default on for SAME-S/L).

dim_heads : Attention head dimension. 64 for production SAME-S/L.

ff_mult : SwiGLU feed-forward expansion factor.

sliding_window : Sliding-window half-width (in latents) for the band-mask attention. Production SAME-S/L use 1.

encoder_sinusoidal_blocks : Per-TRB count of trailing transformer layers that use `sin` FFN gating in the encoder (SAME-L: `(0,)`).

decoder_sinusoidal_blocks : Per-TRB count of trailing transformer layers that use `sin` FFN gating in the decoder (SAME-L: `(8,)`).

sampling_rate : Audio sample rate in Hz (e.g. 44100).

Semantically-Aligned Music Encoder (SAME) autoencoder from *Stable Audio 3* ([arXiv 2605.17991](https://arxiv.org/abs/2605.17991)).

The model consists of:

- **Patch embedding** — reshapes stereo audio into non-overlapping patches, trading time for channels (`patch_size ×` downsample, no learned params).
- **Encoder TRB stack** — `SAMETransformerResamplingBlock` blocks that further downsample by a factor of
  `∏(encoder_strides)`.
- **Soft-norm bottleneck** — learnable affine normalisation with running std.
- **Decoder TRB stack** — mirrors the encoder in reverse.
- **Unpatch** — reshapes channels back into the time dimension.

Total downsampling ratio: `patch_size × ∏(encoder_strides)`.

The default hyperparameters match the **SAME-S** checkpoint (`stabilityai/SAME-S`). To load **SAME-L**
(`stabilityai/SAME-L`, used by SA3 Medium) pass `encoder_channels=256, encoder_transformer_depths=(12,)`.

```python
# SAME-S (108 M params, used by SA3 small models) model = AutoencoderSAME() # default values

# SAME-L (852 M params, used by SA3 Medium) model = AutoencoderSAME(encoder_channels=256,
encoder_transformer_depths=(12,))
```

#### encode[[diffusers.AutoencoderSAME.encode]]

```python
encode(sample: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_same.py#L733)

**Parameters:**

sample : `(B, audio_channels, T)` waveform tensor.

return_dict : If `True` return an `AutoencoderSAMEOutput`.

**Returns:**

Latent tensor of shape `(B, latent_dim, T // downsampling_ratio)` (wrapped in
`AutoencoderSAMEOutput` when *return_dict* is `True`).

Encode stereo audio to latents.

#### decode[[diffusers.AutoencoderSAME.decode]]

```python
decode(latents: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_same.py#L758)

**Parameters:**

latents : `(B, latent_dim, T_latent)` latent tensor.

return_dict : If `True` return an `AutoencoderSAMEDecoderOutput`.

**Returns:**

Waveform tensor of shape `(B, audio_channels, T_latent × downsampling_ratio)` (wrapped in
`AutoencoderSAMEDecoderOutput` when *return_dict* is `True`).

Decode latents back to stereo audio.

#### forward[[diffusers.AutoencoderSAME.forward]]

```python
forward(sample: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_same.py#L783)

**Parameters:**

sample : `(B, audio_channels, T)` waveform tensor.

return_dict : If `True` return an `AutoencoderSAMEDecoderOutput`.

**Returns:**

Reconstructed waveform (same shape as *sample*, possibly longer due to padding), wrapped in
`AutoencoderSAMEDecoderOutput` when *return_dict* is `True`.

Encode and immediately decode *sample* (reconstruction).

## SAMETransformerResamplingBlock[[diffusers.models.autoencoders.autoencoder_same.SAMETransformerResamplingBlock]]

#### diffusers.models.autoencoders.autoencoder_same.SAMETransformerResamplingBlock[[diffusers.models.autoencoders.autoencoder_same.SAMETransformerResamplingBlock]]

```python
diffusers.models.autoencoders.autoencoder_same.SAMETransformerResamplingBlock(in_channels: int, out_channels: int, stride: int, mode: str = 'encoder', transformer_depth: int = 3, dim_heads: int = 128, use_differential: bool = True, ff_mult: int = 3, sliding_window: int = 1, sinusoidal_blocks: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_same.py#L294)

**Parameters:**

in_channels : Number of input channels.

out_channels : Number of output channels.

stride : Down-/up-sampling factor.

mode : `"encoder"` or `"decoder"`.

transformer_depth : Number of `TransformerBlock` layers.

dim_heads : Attention head dimension.

use_differential : Whether to use differential attention.

ff_mult : Feed-forward expansion factor.

sliding_window : Sliding-window half-width in latents (band half-width is `sliding_window * (stride + 1)`).

sinusoidal_blocks : Number of trailing transformer layers that use `sin` FFN gating instead of SiLU.

Core building block of SAME.

**Encoder mode** (stride S):
Groups S consecutive input frames into one segment, appends a single learnable output embedding, then runs D
transformer layers over the full flattened segment sequence and keeps only the output embedding → downsample by
S.

**Decoder mode** (stride S):
Groups 1 input frame with S learnable output embeddings, runs D transformer layers over the full flattened
sequence, then keeps the S output embeddings → upsample by S.

Attention uses an overlapping *sliding-window* band mask over the flattened segment sequence: each token attends to
`sliding_window * (stride + 1)` neighbours on each side. RoPE is computed over the full sequence length. This
matches the reference implementation exactly (a single non-overlapping chunk would only match for one segment).

## AutoencoderSAMEOutput[[diffusers.models.autoencoders.autoencoder_same.AutoencoderSAMEOutput]]

#### diffusers.models.autoencoders.autoencoder_same.AutoencoderSAMEOutput[[diffusers.models.autoencoders.autoencoder_same.AutoencoderSAMEOutput]]

```python
diffusers.models.autoencoders.autoencoder_same.AutoencoderSAMEOutput(latents: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_same.py#L52)

Output of [AutoencoderSAME.encode()](/docs/diffusers/v0.40.0/en/api/models/autoencoder_same#diffusers.AutoencoderSAME.encode).

## AutoencoderSAMEDecoderOutput[[diffusers.models.autoencoders.autoencoder_same.AutoencoderSAMEDecoderOutput]]

#### diffusers.models.autoencoders.autoencoder_same.AutoencoderSAMEDecoderOutput[[diffusers.models.autoencoders.autoencoder_same.AutoencoderSAMEDecoderOutput]]

```python
diffusers.models.autoencoders.autoencoder_same.AutoencoderSAMEDecoderOutput(sample: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_same.py#L59)

Output of [AutoencoderSAME.decode()](/docs/diffusers/v0.40.0/en/api/models/autoencoder_same#diffusers.AutoencoderSAME.decode).
