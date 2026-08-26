# AutoencoderKLKVAEVideo

The 3D variational autoencoder (VAE) model with KL loss.

The model can be loaded with the following code snippet.

```python
import torch
from diffusers import AutoencoderKLKVAEVideo

vae = AutoencoderKLKVAEVideo.from_pretrained("kandinskylab/KVAE-3D-1.0", subfolder="diffusers", dtype=torch.float16)
```

## AutoencoderKLKVAEVideo[[diffusers.AutoencoderKLKVAEVideo]]

#### diffusers.AutoencoderKLKVAEVideo[[diffusers.AutoencoderKLKVAEVideo]]

```python
diffusers.AutoencoderKLKVAEVideo(ch: int = 128, ch_mult: typing.Tuple[int, ...] = (1, 2, 4, 8), num_res_blocks: int = 2, in_channels: int = 3, out_ch: int = 3, z_channels: int = 16, temporal_compress_times: int = 4)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_kvae_video.py#L707)

**Parameters:**

ch (`int`, *optional*, defaults to 128) : Base channel count.

ch_mult (`Tuple[int]`, *optional*, defaults to `(1, 2, 4, 8)`) : Channel multipliers per level.

num_res_blocks (`int`, *optional*, defaults to 2) : Number of residual blocks per level.

in_channels (`int`, *optional*, defaults to 3) : Number of input channels.

out_ch (`int`, *optional*, defaults to 3) : Number of output channels.

z_channels (`int`, *optional*, defaults to 16) : Number of latent channels.

temporal_compress_times (`int`, *optional*, defaults to 4) : Temporal compression factor.

A VAE model with KL loss for encoding videos into latents and decoding latent representations into videos. Used in
[KVAE](https://github.com/kandinskylab/kvae-1).

This model inherits from [ModelMixin](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for its generic methods implemented for
all models (such as downloading or saving).

#### decode[[diffusers.AutoencoderKLKVAEVideo.decode]]

```python
decode(z: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_kvae_video.py#L915)

**Parameters:**

z (`torch.Tensor`) : Input batch of latent vectors with shape (B, C, T, H, W).

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a `~models.vae.DecoderOutput` instead of a plain tuple.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

Decoded video.

Decode a batch of videos.

#### disable_slicing[[diffusers.AutoencoderKLKVAEVideo.disable_slicing]]

```python
disable_slicing()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_kvae_video.py#L844)

Disable sliced VAE decoding.

#### enable_slicing[[diffusers.AutoencoderKLKVAEVideo.enable_slicing]]

```python
enable_slicing()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_kvae_video.py#L840)

Enable sliced VAE decoding.

#### encode[[diffusers.AutoencoderKLKVAEVideo.encode]]

```python
encode(x: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_kvae_video.py#L867)

**Parameters:**

x (`torch.Tensor`) : Input batch of videos with shape (B, C, T, H, W).

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a `~models.autoencoder_kl.AutoencoderKLOutput` instead of a plain tuple.

**Returns:**

The latent representations of the encoded videos.

Encode a batch of videos into latents.

#### forward[[diffusers.AutoencoderKLKVAEVideo.forward]]

```python
forward(sample: Tensor, sample_posterior: bool = False, return_dict: bool = True, generator: typing.Optional[torch.Generator] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_kvae_video.py#L938)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

sample_posterior (`bool`, *optional*, defaults to `False`) : Whether to sample from the posterior.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `DecoderOutput` instead of a plain tuple.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make sampling deterministic.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If `return_dict` is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.
