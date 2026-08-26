# LTX2VideoDiffusionDecoderModel

The diffusion video decoder introduced in LTX-2.5 by Lightricks. Neighborhood-attention stages
upsample the latent into a context volume, and a final stage denoises pixels conditioned on that context.

It is a decoder, not an autoencoder: encoding stays with [AutoencoderKLLTX2Video](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl_ltx_2#diffusers.AutoencoderKLLTX2Video), whose latent space this
consumes unchanged, so latents are interchangeable between the convolutional decoder and this one. Because it is
itself a diffusion model it is driven by [LTX2VideoDiffusionDecodePipeline](/docs/diffusers/v0.40.0/en/api/pipelines/ltx2#diffusers.LTX2VideoDiffusionDecodePipeline) rather than being passed as a
pipeline's `vae`: run any LTX-2 pipeline with `output_type="latent"`, then decode.

```python
import torch
from diffusers import LTX2Pipeline, LTX2VideoDiffusionDecodePipeline, LTX2VideoDiffusionDecoderModel

pipe = LTX2Pipeline.from_pretrained("Lightricks/LTX-2.5-Diffusers", dtype=torch.bfloat16).to("cuda")
latents = pipe(prompt="a potter shaping a clay vase", output_type="latent").frames

decoder = LTX2VideoDiffusionDecoderModel.from_pretrained(
    "Lightricks/LTX-2.5-Diffusers", subfolder="diffusion_decoder", dtype=torch.bfloat16
).to("cuda")
decode_pipe = LTX2VideoDiffusionDecodePipeline(diffusion_decoder=decoder, scheduler=pipe.scheduler)

# `denormalize=False`: `output_type="latent"` already applied the latent statistics, so applying them
# again here would scale every channel by its std a second time.
# The decoder also draws the noise it denoises, so decoding is only reproducible with a generator.
video = decode_pipe(
    latents, generator=torch.Generator("cuda").manual_seed(0), denormalize=False
).frames[0]
```

`vae` is an optional component on the decode pipeline: it is only consulted for the latent statistics when
`denormalize=True`, and the decoder carries its own, so a decode-only workflow does not have to load a second
autoencoder.

## Attention backends

The neighborhood-attention window is expressed as a `BlockMask`, so the decoder runs on the `flex` attention
backend by default and needs no extra dependency. PyTorch does not compile `flex_attention` unless you ask it to,
and uncompiled it materializes the full score matrix — which is impractical at full-resolution sequence lengths.
For those, either compile the decoder or switch to [NATTEN](https://github.com/SHI-Labs/NATTEN)'s kernels, which
are also what the original implementation uses. The processor fetches NATTEN from the Hub
([`shi-labs/natten`](https://huggingface.co/shi-labs/natten)) through the
[`kernels`](https://github.com/huggingface/kernels) package, so it needs `pip install kernels` rather than a local
NATTEN build:

```python
from diffusers.models.autoencoders.ltx2_diffusion_decoder import LTX2VideoVaeNeighborhoodNattenProcessor

decoder.set_attn_processor(LTX2VideoVaeNeighborhoodNattenProcessor())
```

Fetching the kernel downloads code from the Hub, so the processor raises when remote code is disabled globally with
`DIFFUSERS_DISABLE_REMOTE_CODE=true`.

Every attention module in the decoder is the same neighborhood attention (per-stage differences like the kernel
size live on the module, not the processor), so `set_attn_processor` swaps them all with one shared instance.

Switching the *backend* (`decoder.set_attention_backend(...)`) to anything but `flex` raises: no other backend
accepts the `BlockMask`. Use the NATTEN processor above instead.

## Tiling

`decoder.enable_tiling()` decodes in overlapping tiles that are blended back together, bounding peak memory by the
tile size instead of the video size. The cheap early upsampling stages still see the full latent — only the last
upsampling stage and the diffusion stage, which dominate decode memory, run per tile — so tiling changes the output
only near tile borders. Because the diffusion stage denoises each tile separately, a tiled decode does not
reproduce the untiled result exactly; the default tile and overlap sizes match the reference implementation's.
Neighborhood attention rejects any grid smaller than its kernel, so a trailing remnant tile is merged into its
neighbor rather than decoded on its own.

## LTX2VideoDiffusionDecoderModel[[diffusers.LTX2VideoDiffusionDecoderModel]]

#### diffusers.LTX2VideoDiffusionDecoderModel[[diffusers.LTX2VideoDiffusionDecoderModel]]

```python
diffusers.LTX2VideoDiffusionDecoderModel(out_channels: int = 3, latent_channels: int = 128, patch_size: int = 4, scaling_factor: float = 1.0, decoder_head_dim: int = 64, decoder_stage_channels: tuple = (2048, 1024, 512, 512, 256), decoder_stage_depths: tuple = (4, 6, 4, 2, 8), decoder_stage_kernels: tuple = ((3, 7, 7), (3, 7, 7), (3, 5, 5), (3, 5, 5)), decoder_upsample_strides: tuple = ((1, 2, 2), (2, 1, 1), (2, 2, 2), (2, 2, 2)), decoder_upsample_channel_reductions: tuple = (2, 2, 1, 2), decoder_stage5_kernel: tuple = (11, 11, 11), decoder_t_emb_dim: int = 384, decoder_timestep_scale_multiplier: float = 1000.0, decoder_model_output_type: str = 'x0', decoder_num_inference_steps: int = 1, spatial_compression_ratio: int = 32, temporal_compression_ratio: int = 8)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/ltx2_diffusion_decoder.py#L700)

The LTX-2 diffusion video decoder, introduced in LTX-2.5.

This is a decoder, not an autoencoder: it has no encoder and cannot produce latents. Encoding stays with
[AutoencoderKLLTX2Video](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl_ltx_2#diffusers.AutoencoderKLLTX2Video), whose latent space this consumes unchanged, so latents are interchangeable between the
convolutional decoder and this one.

It is also a diffusion model rather than a deterministic decoder — it denoises pixels conditioned on a context
volume built from the latents — which is why it is driven by [LTX2VideoDiffusionDecodePipeline](/docs/diffusers/v0.40.0/en/api/pipelines/ltx2#diffusers.LTX2VideoDiffusionDecodePipeline) rather than being
passed as a pipeline's `vae`.

The latent statistics are carried here as buffers so the decode pipeline can denormalize without loading a second
autoencoder just for two vectors.

This model inherits from [ModelMixin](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for it's generic methods implemented
for all models (such as downloading or saving).

#### decode[[diffusers.LTX2VideoDiffusionDecoderModel.decode]]

```python
decode(z: Tensor, generator: typing.Optional[torch.Generator] = None, num_inference_steps: int | None = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/ltx2_diffusion_decoder.py#L988)

Decode a batch of latents.

`z` is expected to be denormalized already (the pipeline applies `latents_mean` / `latents_std`), matching
[AutoencoderKLLTX2Video](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl_ltx_2#diffusers.AutoencoderKLLTX2Video). This decoder denoises, so pass `generator` for reproducibility.

#### enable_tiling[[diffusers.LTX2VideoDiffusionDecoderModel.enable_tiling]]

```python
enable_tiling(tile_sample_min_height: int | None = None, tile_sample_min_width: int | None = None, tile_sample_min_num_frames: int | None = None, tile_sample_stride_height: int | None = None, tile_sample_stride_width: int | None = None, tile_sample_stride_num_frames: int | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/ltx2_diffusion_decoder.py#L787)

**Parameters:**

tile_sample_min_height (`int`, *optional*) : The height of one decoded tile, in pixels.

tile_sample_min_width (`int`, *optional*) : The width of one decoded tile, in pixels.

tile_sample_min_num_frames (`int`, *optional*) : The number of frames of one decoded tile.

tile_sample_stride_height (`int`, *optional*) : The distance in pixels between the tops of two consecutive vertical tiles; the difference to `tile_sample_min_height` is the blended overlap.

tile_sample_stride_width (`int`, *optional*) : The distance in pixels between the left edges of two consecutive horizontal tiles.

tile_sample_stride_num_frames (`int`, *optional*) : The distance in frames between the starts of two consecutive temporal tiles.

Enable tiled decoding. The deterministic upsampling stages before the last one always process the full latent
(they run at low resolution and are cheap); the last stage and the stage-5 diffusion blocks — which dominate
decode memory — run on overlapping tiles whose seams are blended linearly.

#### disable_tiling[[diffusers.LTX2VideoDiffusionDecoderModel.disable_tiling]]

```python
disable_tiling()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/ltx2_diffusion_decoder.py#L824)

Disable tiled decoding, returning to decoding the whole video in one pass.

#### forward[[diffusers.LTX2VideoDiffusionDecoderModel.forward]]

```python
forward(z: Tensor, generator: typing.Optional[torch.Generator] = None, num_inference_steps: int | None = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/ltx2_diffusion_decoder.py#L1017)

**Parameters:**

z (`torch.Tensor`) : Latents of shape `(B, C, F, H, W)`, expected to be denormalized already (the pipeline applies `latents_mean` / `latents_std`), matching [AutoencoderKLLTX2Video](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl_ltx_2#diffusers.AutoencoderKLLTX2Video).

generator (`torch.Generator`, *optional*) : This decoder denoises, so pass a generator to make decoding reproducible.

num_inference_steps (`int`, *optional*) : Number of denoising steps. Defaults to the decoder's `decoder_num_inference_steps` config value.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a [DecoderOutput](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.models.autoencoders.vae.DecoderOutput) instead of a plain tuple.

**Returns:** [DecoderOutput](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.models.autoencoders.vae.DecoderOutput) or `tuple`

#### tiled_decode[[diffusers.LTX2VideoDiffusionDecoderModel.tiled_decode]]

```python
tiled_decode(z: Tensor, generator: typing.Optional[torch.Generator] = None, num_inference_steps: int | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/ltx2_diffusion_decoder.py#L855)

Decode a batch of latents with the last deterministic stage and the diffusion stage running per tile.

Tiles live on the grid entering the last deterministic stage, where one cell maps to a fixed block of output
pixels; the `tile_sample_*` sizes are converted to that grid, so they should be multiples of the cell size (8
px spatially and 2 frames temporally for the production config). Temporal tiles follow the causal frame
mapping: the tile containing t=0 drops the temporal upsample's duplicate leading frame and only the tile
containing the video end carries the NATTEN border padding.
