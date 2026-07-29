# AutoencoderKLHunyuanImage

The 2D variational autoencoder (VAE) model with KL loss used in [HunyuanImage2.1].

The model can be loaded with the following code snippet.

```python
from diffusers import AutoencoderKLHunyuanImage

vae = AutoencoderKLHunyuanImage.from_pretrained("hunyuanvideo-community/HunyuanImage-2.1-Diffusers", subfolder="vae", torch_dtype=torch.bfloat16)
```

## AutoencoderKLHunyuanImage[[diffusers.AutoencoderKLHunyuanImage]]

#### diffusers.AutoencoderKLHunyuanImage[[diffusers.AutoencoderKLHunyuanImage]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuanimage.py#L412)

A VAE model for 2D images with spatial tiling support.

This model inherits from [ModelMixin](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for it's generic methods implemented
for all models (such as downloading or saving).

wrapperdiffusers.AutoencoderKLHunyuanImage.decodehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/accelerate_utils.py#L43[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]
#### enable_tiling[[diffusers.AutoencoderKLHunyuanImage.enable_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuanimage.py#L467)

Enable spatial tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles
to compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to
allow processing larger images.

**Parameters:**

tile_sample_min_size (`int`, *optional*) : The minimum size required for a sample to be separated into tiles across the spatial dimension.

tile_overlap_factor (`float`, *optional*) : The overlap factor required for a latent to be separated into tiles across the spatial dimension.
#### forward[[diffusers.AutoencoderKLHunyuanImage.forward]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuanimage.py#L667)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

sample_posterior (`bool`, *optional*, defaults to `False`) : Whether to sample from the posterior.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `DecoderOutput` instead of a plain tuple.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make sampling deterministic.

**Returns:**

``~models.vae.DecoderOutput` or `tuple``

If `return_dict` is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.
#### tiled_decode[[diffusers.AutoencoderKLHunyuanImage.tiled_decode]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuanimage.py#L622)

Decode latent using spatial tiling strategy.

**Parameters:**

z (`torch.Tensor`) : Latent tensor of shape (B, C, H, W).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.vae.DecoderOutput` instead of a plain tuple.

**Returns:**

``~models.vae.DecoderOutput` or `tuple``

If return_dict is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.
#### tiled_encode[[diffusers.AutoencoderKLHunyuanImage.tiled_encode]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuanimage.py#L582)

Encode input using spatial tiling strategy.

**Parameters:**

x (`torch.Tensor`) : Input tensor of shape (B, C, T, H, W).

**Returns:**

``torch.Tensor``

The latent representation of the encoded images.

## DecoderOutput[[diffusers.models.autoencoders.vae.DecoderOutput]]

#### diffusers.models.autoencoders.vae.DecoderOutput[[diffusers.models.autoencoders.vae.DecoderOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/vae.py#L46)

Output of decoding method.

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)`) : The decoded output sample from the last layer of the model.
