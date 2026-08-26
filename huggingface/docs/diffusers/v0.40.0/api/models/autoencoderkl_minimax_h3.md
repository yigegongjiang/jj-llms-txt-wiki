# AutoencoderKLMiniMaxH3

The video variational autoencoder (VAE) model with KL loss used in [MiniMax-H3](https://huggingface.co/MiniMaxAI/MiniMax-H3) by MiniMax. It pairs a causal 3D CNN encoder with a non-causal ViT decoder and compresses 16x spatially and 4x temporally.

Three things set it apart from most autoencoders in the library:

- **Latents are normalized per channel.** There is no `scaling_factor`: a pipeline encodes with `(latent - latents_mean) / latents_std` and decodes with `latent * latents_std + latents_mean`.
- **The pixel convention is ImageNet-normalized RGB over a `[0, 1]` base range**, not the usual `[-1, 1]`. `encode` expects `(pixel - imagenet_mean) / imagenet_std` and `decode` returns values in that same space, so a pipeline applies `sample * imagenet_std + imagenet_mean` and clamps to `[0, 1]` before postprocessing.
- **Spatial tiling is on by default.** MiniMax-H3 was released with tiling enabled for both encoding and decoding and the released frames are the blended-tile ones, so turning it off changes the output. Use `enable_tiling` to change the tile geometry and `disable_tiling` to switch it off.

The temporal geometry is fixed by `clip_length` (17 pixel frames per encoder chunk) and `token_drop` (3 trailing latent frames dropped per encode), so `17 * n + 5` pixel frames map to `5 * n + 2` latent frames.

```python
import torch
from diffusers import AutoencoderKLMiniMaxH3

vae = AutoencoderKLMiniMaxH3.from_pretrained(
    "MiniMaxAI/MiniMax-H3", subfolder="vae", dtype=torch.float32
).to("cuda")
```

## AutoencoderKLMiniMaxH3[[diffusers.AutoencoderKLMiniMaxH3]]

#### diffusers.AutoencoderKLMiniMaxH3[[diffusers.AutoencoderKLMiniMaxH3]]

```python
diffusers.AutoencoderKLMiniMaxH3(in_channels: int = 3, out_channels: int = 3, latent_channels: int = 24, block_out_channels: tuple = (128, 256, 256, 512, 512, 1024), layers_per_block: int = 2, spatial_downsample_factors: tuple = (2, 2, 2, 2, 1, 1), temporal_downsample_factors: tuple = (1, 2, 2, 1, 1, 1), norm_num_groups: int = 32, norm_eps: float = 1e-06, spatial_padding_mode: str = 'reflect', decoder_num_layers: int = 36, decoder_num_attention_heads: int = 32, decoder_attention_head_dim: int = 64, decoder_num_register_tokens: int = 4, decoder_ffn_mult: int = 4, decoder_rope_theta: float = 100.0, decoder_rope_dim_ratio: float = 0.75, decoder_norm_eps: float = 1e-05, clip_length: int = 17, token_drop: int = 3, latents_mean: tuple = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), latents_std: tuple = (1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0))
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_minimax_h3.py#L501)

A VAE model with a causal 3D CNN encoder and a non-causal ViT decoder, used in
[MiniMax-H3](https://huggingface.co/MiniMaxAI).

This model inherits from [ModelMixin](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for it's generic methods implemented
for all models (such as downloading or saving).

Latents are normalized with per-channel `latents_mean` / `latents_std` rather than a `scaling_factor`; a pipeline
encodes with `(latent - latents_mean) / latents_std` and decodes with `latent * latents_std + latents_mean`.

The pixel convention is ImageNet-normalized RGB over a `[0, 1]` base range, not the usual `[-1, 1]`: `encode`
expects `(pixel - imagenet_mean) / imagenet_std` and `decode` returns values in that same space, so a pipeline has
to apply `sample * imagenet_std + imagenet_mean` (mean `(0.485, 0.456, 0.406)`, std `(0.229, 0.224, 0.225)`) and
clamp to `[0, 1]` before postprocessing.

The temporal geometry is fixed by `clip_length` (17 pixel frames per encoder chunk) and `token_drop` (3 trailing
latent frames dropped per encode): `17 * n + 5` pixel frames map to `5 * n + 2` latent frames.

Unlike most autoencoders in the library, spatial tiling is **on by default**: MiniMax-H3 was released with tiling
enabled for both encoding and decoding, and the released frames are the blended-tile ones, so disabling tiling
changes the output. Use `enable_tiling` to change the tile geometry, `disable_tiling` to turn it off.

#### encode[[diffusers.AutoencoderKLMiniMaxH3.encode]]

```python
encode(x: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_minimax_h3.py#L844)

**Parameters:**

x (`torch.Tensor`) : Input batch of videos, shape `(batch_size, in_channels, num_frames, height, width)`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a [AutoencoderKLOutput](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.models.modeling_outputs.AutoencoderKLOutput) instead of a plain tuple.

**Returns:**

The latent distribution of the encoded videos. Note that MiniMax-H3 normalizes the sampled latents with
`latents_mean` / `latents_std` afterwards.

Encode a batch of videos into latents.

#### decode[[diffusers.AutoencoderKLMiniMaxH3.decode]]

```python
decode(z: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_minimax_h3.py#L872)

**Parameters:**

z (`torch.Tensor`) : Input batch of latent videos, shape `(batch_size, latent_channels, num_latent_frames, height, width)`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a [DecoderOutput](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.models.autoencoders.vae.DecoderOutput) instead of a plain tuple.

**Returns:** [DecoderOutput](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.models.autoencoders.vae.DecoderOutput) or `tuple`

The decoded videos, shape `(batch_size, out_channels, num_frames, height, width)`.

Decode a batch of latent videos.

#### enable_tiling[[diffusers.AutoencoderKLMiniMaxH3.enable_tiling]]

```python
enable_tiling(tile_sample_min_height: int | None = None, tile_sample_min_width: int | None = None, tile_sample_min_overlap_height: int | None = None, tile_sample_min_overlap_width: int | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_minimax_h3.py#L617)

**Parameters:**

tile_sample_min_height (`int`, *optional*) : The tile height in pixel space. Frames taller than this are split along the height dimension.

tile_sample_min_width (`int`, *optional*) : The tile width in pixel space. Frames wider than this are split along the width dimension.

tile_sample_min_overlap_height (`int`, *optional*) : The minimum overlap, in pixels, between two consecutive vertical tiles.

tile_sample_min_overlap_width (`int`, *optional*) : The minimum overlap, in pixels, between two consecutive horizontal tiles.

Enable tiled VAE encoding/decoding. When this option is enabled, the VAE splits the frames into tiles, encodes
or decodes each tile separately and linearly blends the overlaps back together. This lowers the memory
requirement and allows processing larger frames.

#### forward[[diffusers.AutoencoderKLMiniMaxH3.forward]]

```python
forward(sample: Tensor, sample_posterior: bool = False, generator: typing.Optional[torch.Generator] = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_minimax_h3.py#L896)

**Parameters:**

sample (`torch.Tensor`) : Input batch of videos, shape `(batch_size, in_channels, num_frames, height, width)`.

sample_posterior (`bool`, *optional*, defaults to `False`) : Whether to sample the posterior instead of taking its mode.

generator (`torch.Generator`, *optional*) : Generator used when `sample_posterior=True`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a [DecoderOutput](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.models.autoencoders.vae.DecoderOutput) instead of a plain tuple.

**Returns:** [DecoderOutput](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.models.autoencoders.vae.DecoderOutput) or `tuple`

The round-tripped videos, shape `(batch_size, out_channels, num_frames, height, width)`.

Encode then decode a batch of videos.
