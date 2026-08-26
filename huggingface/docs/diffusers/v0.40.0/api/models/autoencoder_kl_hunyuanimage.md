# AutoencoderKLHunyuanImage

The 2D variational autoencoder (VAE) model with KL loss used in [HunyuanImage2.1].

The model can be loaded with the following code snippet.

```python
from diffusers import AutoencoderKLHunyuanImage

vae = AutoencoderKLHunyuanImage.from_pretrained("hunyuanvideo-community/HunyuanImage-2.1-Diffusers", subfolder="vae", dtype=torch.bfloat16)
```

## AutoencoderKLHunyuanImage[[diffusers.AutoencoderKLHunyuanImage]]

#### diffusers.AutoencoderKLHunyuanImage[[diffusers.AutoencoderKLHunyuanImage]]

```python
diffusers.AutoencoderKLHunyuanImage(in_channels: int, out_channels: int, latent_channels: int, block_out_channels: tuple, layers_per_block: int, spatial_compression_ratio: int, sample_size: int, scaling_factor: float = None, downsample_match_channel: bool = True, upsample_match_channel: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuanimage.py#L412)

A VAE model for 2D images with spatial tiling support.

This model inherits from [ModelMixin](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for it's generic methods implemented
for all models (such as downloading or saving).

#### decode[[diffusers.AutoencoderKLHunyuanImage.decode]]

```python
decode(z: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuanimage.py#L540)

**Parameters:**

z (`torch.Tensor`) : Input batch of latent vectors.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a `~models.vae.DecoderOutput` instead of a plain tuple.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If return_dict is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

Decode a batch of images.

#### enable_tiling[[diffusers.AutoencoderKLHunyuanImage.enable_tiling]]

```python
enable_tiling(tile_sample_min_size: int | None = None, tile_overlap_factor: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuanimage.py#L467)

**Parameters:**

tile_sample_min_size (`int`, *optional*) : The minimum size required for a sample to be separated into tiles across the spatial dimension.

tile_overlap_factor (`float`, *optional*) : The overlap factor required for a latent to be separated into tiles across the spatial dimension.

Enable spatial tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles
to compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to
allow processing larger images.

#### encode[[diffusers.AutoencoderKLHunyuanImage.encode]]

```python
encode(x: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuanimage.py#L499)

**Parameters:**

x (`torch.Tensor`) : Input batch of images.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a `~models.autoencoder_kl.AutoencoderKLOutput` instead of a plain tuple.

**Returns:**

The latent representations of the encoded videos. If `return_dict` is True, a
`~models.autoencoder_kl.AutoencoderKLOutput` is returned, otherwise a plain `tuple` is returned.

Encode a batch of images into latents.

#### forward[[diffusers.AutoencoderKLHunyuanImage.forward]]

```python
forward(sample: Tensor, sample_posterior: bool = False, return_dict: bool = True, generator: typing.Optional[torch.Generator] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuanimage.py#L667)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

sample_posterior (`bool`, *optional*, defaults to `False`) : Whether to sample from the posterior.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `DecoderOutput` instead of a plain tuple.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make sampling deterministic.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If `return_dict` is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

#### tiled_decode[[diffusers.AutoencoderKLHunyuanImage.tiled_decode]]

```python
tiled_decode(z: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuanimage.py#L622)

**Parameters:**

z (`torch.Tensor`) : Latent tensor of shape (B, C, H, W).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.vae.DecoderOutput` instead of a plain tuple.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If return_dict is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

Decode latent using spatial tiling strategy.

#### tiled_encode[[diffusers.AutoencoderKLHunyuanImage.tiled_encode]]

```python
tiled_encode(x: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuanimage.py#L582)

**Parameters:**

x (`torch.Tensor`) : Input tensor of shape (B, C, T, H, W).

**Returns:** `torch.Tensor`

The latent representation of the encoded images.

Encode input using spatial tiling strategy.

## DecoderOutput[[diffusers.models.autoencoders.vae.DecoderOutput]]

#### diffusers.models.autoencoders.vae.DecoderOutput[[diffusers.models.autoencoders.vae.DecoderOutput]]

```python
diffusers.models.autoencoders.vae.DecoderOutput(sample: Tensor, commit_loss: typing.Optional[torch.FloatTensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/vae.py#L46)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)`) : The decoded output sample from the last layer of the model.

Output of decoding method.
