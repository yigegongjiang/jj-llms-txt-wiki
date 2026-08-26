# AutoencoderKLLTXVideo

The 3D variational autoencoder (VAE) model with KL loss used in [LTX](https://huggingface.co/Lightricks/LTX-Video) was introduced by Lightricks.

The model can be loaded with the following code snippet.

```python
from diffusers import AutoencoderKLLTXVideo

vae = AutoencoderKLLTXVideo.from_pretrained("Lightricks/LTX-Video", subfolder="vae", dtype=torch.float32).to("cuda")
```

## AutoencoderKLLTXVideo[[diffusers.AutoencoderKLLTXVideo]]

#### diffusers.AutoencoderKLLTXVideo[[diffusers.AutoencoderKLLTXVideo]]

```python
diffusers.AutoencoderKLLTXVideo(in_channels: int = 3, out_channels: int = 3, latent_channels: int = 128, block_out_channels: tuple = (128, 256, 512, 512), down_block_types: tuple = ('LTXVideoDownBlock3D', 'LTXVideoDownBlock3D', 'LTXVideoDownBlock3D', 'LTXVideoDownBlock3D'), decoder_block_out_channels: tuple = (128, 256, 512, 512), layers_per_block: tuple = (4, 3, 3, 3, 4), decoder_layers_per_block: tuple = (4, 3, 3, 3, 4), spatio_temporal_scaling: tuple = (True, True, True, False), decoder_spatio_temporal_scaling: tuple = (True, True, True, False), decoder_inject_noise: tuple = (False, False, False, False, False), downsample_type: tuple = ('conv', 'conv', 'conv', 'conv'), upsample_residual: tuple = (False, False, False, False), upsample_factor: tuple = (1, 1, 1, 1), timestep_conditioning: bool = False, patch_size: int = 4, patch_size_t: int = 1, resnet_norm_eps: float = 1e-06, scaling_factor: float = 1.0, encoder_causal: bool = True, decoder_causal: bool = False, spatial_compression_ratio: int = None, temporal_compression_ratio: int = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_ltx.py#L1035)

**Parameters:**

in_channels (`int`, defaults to `3`) : Number of input channels.

out_channels (`int`, defaults to `3`) : Number of output channels.

latent_channels (`int`, defaults to `128`) : Number of latent channels.

block_out_channels (`tuple[int, ...]`, defaults to `(128, 256, 512, 512)`) : The number of output channels for each block.

spatio_temporal_scaling (`tuple[bool, ...], defaults to `(True, True, True, False)` : Whether a block should contain spatio-temporal downscaling or not.

layers_per_block (`tuple[int, ...]`, defaults to `(4, 3, 3, 3, 4)`) : The number of layers per block.

patch_size (`int`, defaults to `4`) : The size of spatial patches.

patch_size_t (`int`, defaults to `1`) : The size of temporal patches.

resnet_norm_eps (`float`, defaults to `1e-6`) : Epsilon value for ResNet normalization layers.

scaling_factor (`float`, *optional*, defaults to `1.0`) : The component-wise standard deviation of the trained latent space computed using the first batch of the training set. This is used to scale the latent space to have unit variance when training the diffusion model. The latents are scaled with the formula `z = z * scaling_factor` before being passed to the diffusion model. When decoding, the latents are scaled back to the original scale with the formula: `z = 1 / scaling_factor * z`. For more details, refer to sections 4.3.2 and D.1 of the [High-Resolution Image Synthesis with Latent Diffusion Models](https://huggingface.co/papers/2112.10752) paper.

encoder_causal (`bool`, defaults to `True`) : Whether the encoder should behave causally (future frames depend only on past frames) or not.

decoder_causal (`bool`, defaults to `False`) : Whether the decoder should behave causally (future frames depend only on past frames) or not.

A VAE model with KL loss for encoding images into latents and decoding latent representations into images. Used in
[LTX](https://huggingface.co/Lightricks/LTX-Video).

This model inherits from [ModelMixin](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for it's generic methods implemented
for all models (such as downloading or saving).

#### decode[[diffusers.AutoencoderKLLTXVideo.decode]]

```python
decode(z: Tensor, temb: typing.Optional[torch.Tensor] = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_ltx.py#L1281)

**Parameters:**

z (`torch.Tensor`) : Input batch of latent vectors.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a `~models.vae.DecoderOutput` instead of a plain tuple.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If return_dict is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

Decode a batch of images.

#### encode[[diffusers.AutoencoderKLLTXVideo.encode]]

```python
encode(x: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_ltx.py#L1233)

**Parameters:**

x (`torch.Tensor`) : Input batch of images.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a `~models.autoencoder_kl.AutoencoderKLOutput` instead of a plain tuple.

**Returns:**

The latent representations of the encoded videos. If `return_dict` is True, a
`~models.autoencoder_kl.AutoencoderKLOutput` is returned, otherwise a plain `tuple` is returned.

Encode a batch of images into latents.

#### enable_tiling[[diffusers.AutoencoderKLLTXVideo.enable_tiling]]

```python
enable_tiling(tile_sample_min_height: int | None = None, tile_sample_min_width: int | None = None, tile_sample_min_num_frames: int | None = None, tile_sample_stride_height: float | None = None, tile_sample_stride_width: float | None = None, tile_sample_stride_num_frames: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_ltx.py#L1186)

**Parameters:**

tile_sample_min_height (`int`, *optional*) : The minimum height required for a sample to be separated into tiles across the height dimension.

tile_sample_min_width (`int`, *optional*) : The minimum width required for a sample to be separated into tiles across the width dimension.

tile_sample_stride_height (`int`, *optional*) : The minimum amount of overlap between two consecutive vertical tiles. This is to ensure that there are no tiling artifacts produced across the height dimension.

tile_sample_stride_width (`int`, *optional*) : The stride between two consecutive horizontal tiles. This is to ensure that there are no tiling artifacts produced across the width dimension.

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.

#### forward[[diffusers.AutoencoderKLLTXVideo.forward]]

```python
forward(sample: Tensor, temb: typing.Optional[torch.Tensor] = None, sample_posterior: bool = False, return_dict: bool = True, generator: typing.Optional[torch.Generator] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_ltx.py#L1517)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

temb (`torch.Tensor`, *optional*) : Optional timestep embedding tensor used to condition the decoder.

sample_posterior (`bool`, *optional*, defaults to `False`) : Whether to sample from the posterior.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `DecoderOutput` instead of a plain tuple.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make sampling deterministic.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If `return_dict` is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

#### tiled_decode[[diffusers.AutoencoderKLLTXVideo.tiled_decode]]

```python
tiled_decode(z: Tensor, temb: typing.Optional[torch.Tensor], return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_ltx.py#L1389)

**Parameters:**

z (`torch.Tensor`) : Input batch of latent vectors.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.vae.DecoderOutput` instead of a plain tuple.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If return_dict is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

Decode a batch of images using a tiled decoder.

#### tiled_encode[[diffusers.AutoencoderKLLTXVideo.tiled_encode]]

```python
tiled_encode(x: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_ltx.py#L1338)

**Parameters:**

x (`torch.Tensor`) : Input batch of videos.

**Returns:** `torch.Tensor`

The latent representation of the encoded videos.

Encode a batch of images using a tiled encoder.

## AutoencoderKLOutput[[diffusers.models.modeling_outputs.AutoencoderKLOutput]]

#### diffusers.models.modeling_outputs.AutoencoderKLOutput[[diffusers.models.modeling_outputs.AutoencoderKLOutput]]

```python
diffusers.models.modeling_outputs.AutoencoderKLOutput(latent_dist: DiagonalGaussianDistribution)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/modeling_outputs.py#L7)

**Parameters:**

latent_dist (`DiagonalGaussianDistribution`) : Encoded outputs of `Encoder` represented as the mean and logvar of `DiagonalGaussianDistribution`. `DiagonalGaussianDistribution` allows for sampling latents from the distribution.

Output of AutoencoderKL encoding method.

## DecoderOutput[[diffusers.models.autoencoders.vae.DecoderOutput]]

#### diffusers.models.autoencoders.vae.DecoderOutput[[diffusers.models.autoencoders.vae.DecoderOutput]]

```python
diffusers.models.autoencoders.vae.DecoderOutput(sample: Tensor, commit_loss: typing.Optional[torch.FloatTensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/vae.py#L46)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)`) : The decoded output sample from the last layer of the model.

Output of decoding method.
