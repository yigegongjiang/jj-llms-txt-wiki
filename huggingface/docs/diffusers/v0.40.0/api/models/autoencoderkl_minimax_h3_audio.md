# AutoencoderKLMiniMaxH3Audio

The audio autoencoder used in [MiniMax-H3](https://huggingface.co/MiniMaxAI) by MiniMax. It is waveform in and waveform out, with no mel front-end and no separate vocoder: a DAC-lineage strided convolutional encoder, a causal-attention projection onto the diffusion latent width, and a BigVGAN decoder.

The encoder hops 800 samples at 32 kHz, i.e. 40 latents per second, so a waveform of `800 * n` samples encodes to `n` latents. Waveforms that are not a whole number of hops are right-padded.

The causal-attention projection goes through the attention dispatcher, so `set_attention_backend` applies to it; its mask is `is_causal=True`, which every backend honours except `_native_npu`, whose kernel takes no causal flag.

The autoencoder is **mono**, and it normalizes latents per channel with `latents_mean` / `latents_std` rather than a scalar `scaling_factor`. MiniMax-H3 carries stereo as two *batch* items, and it always consumes the posterior mean (`latent_dist.mode()`), never a sample.

```python
import torch
from diffusers import AutoencoderKLMiniMaxH3Audio

audio_vae = AutoencoderKLMiniMaxH3Audio.from_pretrained(
    "MiniMaxAI/MiniMax-H3", subfolder="audio_vae", dtype=torch.float32
).to("cuda")
```

## AutoencoderKLMiniMaxH3Audio[[diffusers.AutoencoderKLMiniMaxH3Audio]]

#### diffusers.AutoencoderKLMiniMaxH3Audio[[diffusers.AutoencoderKLMiniMaxH3Audio]]

```python
diffusers.AutoencoderKLMiniMaxH3Audio(encoder_dim: int = 64, encoder_rates: tuple = (2, 4, 4, 5, 5), latent_dim: int = 2048, latent_channels: int = 32, num_attention_heads: int = 8, decoder_dim: int = 1024, decoder_rates: tuple = (5, 5, 2, 2, 2, 2, 2), decoder_kernel_sizes: tuple = (9, 9, 4, 4, 4, 4, 4), resblock_kernel_sizes: tuple = (3, 7, 11), resblock_dilation_sizes: tuple = ((1, 3, 5), (1, 3, 5), (1, 3, 5)), sampling_rate: int = 32000, latents_mean: list[float] | None = None, latents_std: list[float] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_minimax_h3_audio.py#L489)

**Parameters:**

encoder_dim (`int`, defaults to `64`) : Channel width of the encoder's first convolution; doubles at every downsampling stage.

encoder_rates (`tuple[int]`, defaults to `(2, 4, 4, 5, 5)`) : Encoder strides. Their product (`800`) is the hop length, i.e. 40 latents/s at 32 kHz.

latent_dim (`int`, defaults to `2048`) : Width of the encoder trunk and of the decoder input, before/after the latent projections.

latent_channels (`int`, defaults to `32`) : Width of the diffusion latent, i.e. the `mean_proj` / `logs_proj` output channels.

num_attention_heads (`int`, defaults to `8`) : Number of heads in the causal-attention projection `pre_block`.

decoder_dim (`int`, defaults to `1024`) : BigVGAN initial channel count; halved at every upsampling stage.

decoder_rates (`tuple[int]`, defaults to `(5, 5, 2, 2, 2, 2, 2)`) : BigVGAN upsampling rates. Their product must equal `prod(encoder_rates)`.

decoder_kernel_sizes (`tuple[int]`, defaults to `(9, 9, 4, 4, 4, 4, 4)`) : Transposed-convolution kernel size per upsampling stage.

resblock_kernel_sizes (`tuple[int]`, defaults to `(3, 7, 11)`) : Kernel sizes of the parallel AMP residual blocks at each upsampling stage.

resblock_dilation_sizes (`tuple[tuple[int]]`, defaults to `((1, 3, 5), (1, 3, 5), (1, 3, 5))`) : Per-AMP-block dilations.

sampling_rate (`int`, defaults to `32000`) : Waveform sampling rate.

latents_mean (`list[float]`, *optional*) : Per-channel latent mean the pipeline uses to normalize / denormalize latents.

latents_std (`list[float]`, *optional*) : Per-channel latent standard deviation the pipeline uses to normalize / denormalize latents.

The audio autoencoder used by [MiniMax-H3](https://huggingface.co/MiniMaxAI): a DAC-lineage convolutional encoder
and a BigVGAN decoder, operating directly on mono 32 kHz waveforms.

This model inherits from [ModelMixin](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for the generic methods the library
implements for all models (such as downloading or saving).

#### encode[[diffusers.AutoencoderKLMiniMaxH3Audio.encode]]

```python
encode(sample: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_minimax_h3_audio.py#L583)

**Parameters:**

sample (`torch.Tensor`) : Mono waveform of shape `[batch_size, 1, samples]`. MiniMax-H3 passes the two stereo channels of a reference clip as `batch_size = 2`.

return_dict (`bool`, defaults to `True`) : Whether to return a `MiniMaxH3AudioEncoderOutput` instead of a plain tuple.

**Returns:** `MiniMaxH3AudioEncoderOutput` or `tuple`

The latent posterior over `[batch_size, latent_channels, samples / 800]`.

Encode a waveform into the audio latent posterior.

The waveform is right-padded to a multiple of `hop_length` (800 samples) first. MiniMax-H3 always consumes the
posterior **mean** (`latent_dist.mode()`) — the `logs_proj` head is never evaluated by the reference pipeline.

#### decode[[diffusers.AutoencoderKLMiniMaxH3Audio.decode]]

```python
decode(latents: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_minimax_h3_audio.py#L623)

**Parameters:**

latents (`torch.Tensor`) : Denormalized latents of shape `[batch_size, latent_channels, num_frames]`. MiniMax-H3 passes the two stereo channels as `batch_size = 2`.

return_dict (`bool`, defaults to `True`) : Whether to return a [DecoderOutput](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.models.autoencoders.vae.DecoderOutput) instead of a plain tuple.

**Returns:** [DecoderOutput](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.models.autoencoders.vae.DecoderOutput) or `tuple`

Waveform of shape `[batch_size, 1, num_frames * 800]`, clamped to `[-1, 1]`.

Decode audio latents into a waveform.

#### forward[[diffusers.AutoencoderKLMiniMaxH3Audio.forward]]

```python
forward(sample: Tensor, sample_posterior: bool = False, return_dict: bool = True, generator: typing.Optional[torch.Generator] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_minimax_h3_audio.py#L653)

**Parameters:**

sample (`torch.Tensor`) : Mono waveform of shape `[batch_size, 1, samples]`.

sample_posterior (`bool`, defaults to `False`) : Whether to sample the posterior instead of taking its mode. MiniMax-H3 uses the mode.

return_dict (`bool`, defaults to `True`) : Whether to return a [DecoderOutput](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.models.autoencoders.vae.DecoderOutput) instead of a plain tuple.

generator (`torch.Generator`, *optional*) : Generator used when `sample_posterior=True`.

**Returns:** [DecoderOutput](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.models.autoencoders.vae.DecoderOutput) or `tuple`

The round-tripped waveform of shape `[batch_size, 1, num_frames * 800]`, clamped to `[-1, 1]`.

Encode then decode a waveform.
