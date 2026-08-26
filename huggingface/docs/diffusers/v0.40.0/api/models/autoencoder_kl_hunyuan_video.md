# AutoencoderKLHunyuanVideo

The 3D variational autoencoder (VAE) model with KL loss used in [HunyuanVideo](https://github.com/Tencent/HunyuanVideo/), which was introduced in [HunyuanVideo: A Systematic Framework For Large Video Generative Models](https://huggingface.co/papers/2412.03603) by Tencent.

The model can be loaded with the following code snippet.

```python
from diffusers import AutoencoderKLHunyuanVideo

vae = AutoencoderKLHunyuanVideo.from_pretrained("hunyuanvideo-community/HunyuanVideo", subfolder="vae", dtype=torch.float16)
```

## AutoencoderKLHunyuanVideo[[diffusers.AutoencoderKLHunyuanVideo]]

#### diffusers.AutoencoderKLHunyuanVideo[[diffusers.AutoencoderKLHunyuanVideo]]

```python
diffusers.AutoencoderKLHunyuanVideo(in_channels: int = 3, out_channels: int = 3, latent_channels: int = 16, down_block_types: tuple = ('HunyuanVideoDownBlock3D', 'HunyuanVideoDownBlock3D', 'HunyuanVideoDownBlock3D', 'HunyuanVideoDownBlock3D'), up_block_types: tuple = ('HunyuanVideoUpBlock3D', 'HunyuanVideoUpBlock3D', 'HunyuanVideoUpBlock3D', 'HunyuanVideoUpBlock3D'), block_out_channels: tuple = (128, 256, 512, 512), layers_per_block: int = 2, act_fn: str = 'silu', norm_num_groups: int = 32, scaling_factor: float = 0.476986, spatial_compression_ratio: int = 8, temporal_compression_ratio: int = 4, mid_block_add_attention: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuan_video.py#L625)

A VAE model with KL loss for encoding videos into latents and decoding latent representations into videos.
Introduced in [HunyuanVideo](https://huggingface.co/papers/2412.03603).

This model inherits from [ModelMixin](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for it's generic methods implemented
for all models (such as downloading or saving).

#### decode[[diffusers.AutoencoderKLHunyuanVideo.decode]]

```python
decode(z: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuan_video.py#L825)

**Parameters:**

z (`torch.Tensor`) : Input batch of latent vectors.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a `~models.vae.DecoderOutput` instead of a plain tuple.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If return_dict is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

Decode a batch of images.

#### enable_tiling[[diffusers.AutoencoderKLHunyuanVideo.enable_tiling]]

```python
enable_tiling(tile_sample_min_height: int | None = None, tile_sample_min_width: int | None = None, tile_sample_min_num_frames: int | None = None, tile_sample_stride_height: float | None = None, tile_sample_stride_width: float | None = None, tile_sample_stride_num_frames: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuan_video.py#L724)

**Parameters:**

tile_sample_min_height (`int`, *optional*) : The minimum height required for a sample to be separated into tiles across the height dimension.

tile_sample_min_width (`int`, *optional*) : The minimum width required for a sample to be separated into tiles across the width dimension.

tile_sample_min_num_frames (`int`, *optional*) : The minimum number of frames required for a sample to be separated into tiles across the frame dimension.

tile_sample_stride_height (`int`, *optional*) : The minimum amount of overlap between two consecutive vertical tiles. This is to ensure that there are no tiling artifacts produced across the height dimension.

tile_sample_stride_width (`int`, *optional*) : The stride between two consecutive horizontal tiles. This is to ensure that there are no tiling artifacts produced across the width dimension.

tile_sample_stride_num_frames (`int`, *optional*) : The stride between two consecutive frame tiles. This is to ensure that there are no tiling artifacts produced across the frame dimension.

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.

#### encode[[diffusers.AutoencoderKLHunyuanVideo.encode]]

```python
encode(x: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuan_video.py#L777)

**Parameters:**

x (`torch.Tensor`) : Input batch of images.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a `~models.autoencoder_kl.AutoencoderKLOutput` instead of a plain tuple.

**Returns:**

The latent representations of the encoded videos. If `return_dict` is True, a
`~models.autoencoder_kl.AutoencoderKLOutput` is returned, otherwise a plain `tuple` is returned.

Encode a batch of images into latents.

#### forward[[diffusers.AutoencoderKLHunyuanVideo.forward]]

```python
forward(sample: Tensor, sample_posterior: bool = False, return_dict: bool = True, generator: typing.Optional[torch.Generator] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuan_video.py#L1050)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

sample_posterior (`bool`, *optional*, defaults to `False`) : Whether to sample from the posterior.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `DecoderOutput` instead of a plain tuple.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make sampling deterministic.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If `return_dict` is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

#### tiled_decode[[diffusers.AutoencoderKLHunyuanVideo.tiled_decode]]

```python
tiled_decode(z: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuan_video.py#L925)

**Parameters:**

z (`torch.Tensor`) : Input batch of latent vectors.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.vae.DecoderOutput` instead of a plain tuple.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If return_dict is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

Decode a batch of images using a tiled decoder.

#### tiled_encode[[diffusers.AutoencoderKLHunyuanVideo.tiled_encode]]

```python
tiled_encode(x: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuan_video.py#L875)

**Parameters:**

x (`torch.Tensor`) : Input batch of videos.

**Returns:** `torch.Tensor`

The latent representation of the encoded videos.

Encode a batch of images using a tiled encoder.

## DecoderOutput[[diffusers.models.autoencoders.vae.DecoderOutput]]

#### diffusers.models.autoencoders.vae.DecoderOutput[[diffusers.models.autoencoders.vae.DecoderOutput]]

```python
diffusers.models.autoencoders.vae.DecoderOutput(sample: Tensor, commit_loss: typing.Optional[torch.FloatTensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/vae.py#L46)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)`) : The decoded output sample from the last layer of the model.

Output of decoding method.
