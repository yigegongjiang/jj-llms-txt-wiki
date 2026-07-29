# AutoencoderKLHunyuanImageRefiner

The 3D variational autoencoder (VAE) model with KL loss used in [HunyuanImage2.1](https://github.com/Tencent-Hunyuan/HunyuanImage-2.1) for its refiner pipeline.

The model can be loaded with the following code snippet.

```python
from diffusers import AutoencoderKLHunyuanImageRefiner

vae = AutoencoderKLHunyuanImageRefiner.from_pretrained("hunyuanvideo-community/HunyuanImage-2.1-Refiner-Diffusers", subfolder="vae", torch_dtype=torch.bfloat16)
```

## AutoencoderKLHunyuanImageRefiner[[diffusers.AutoencoderKLHunyuanImageRefiner]]

#### diffusers.AutoencoderKLHunyuanImageRefiner[[diffusers.AutoencoderKLHunyuanImageRefiner]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuanimage_refiner.py#L593)

A VAE model with KL loss for encoding videos into latents and decoding latent representations into videos. Used for
HunyuanImage-2.1 Refiner.

This model inherits from [ModelMixin](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for it's generic methods implemented
for all models (such as downloading or saving).

wrapperdiffusers.AutoencoderKLHunyuanImageRefiner.decodehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/accelerate_utils.py#L43[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]
#### enable_tiling[[diffusers.AutoencoderKLHunyuanImageRefiner.enable_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuanimage_refiner.py#L662)

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.

**Parameters:**

tile_sample_min_height (`int`, *optional*) : The minimum height required for a sample to be separated into tiles across the height dimension.

tile_sample_min_width (`int`, *optional*) : The minimum width required for a sample to be separated into tiles across the width dimension.

tile_sample_stride_height (`int`, *optional*) : The minimum amount of overlap between two consecutive vertical tiles. This is to ensure that there are no tiling artifacts produced across the height dimension.

tile_sample_stride_width (`int`, *optional*) : The stride between two consecutive horizontal tiles. This is to ensure that there are no tiling artifacts produced across the width dimension.
#### forward[[diffusers.AutoencoderKLHunyuanImageRefiner.forward]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuanimage_refiner.py#L897)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

sample_posterior (`bool`, *optional*, defaults to `False`) : Whether to sample from the posterior.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `DecoderOutput` instead of a plain tuple.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make sampling deterministic.

**Returns:**

``~models.vae.DecoderOutput` or `tuple``

If `return_dict` is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.
#### tiled_decode[[diffusers.AutoencoderKLHunyuanImageRefiner.tiled_decode]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuanimage_refiner.py#L843)

Decode a batch of images using a tiled decoder.

**Parameters:**

z (`torch.Tensor`) : Input batch of latent vectors.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.vae.DecoderOutput` instead of a plain tuple.

**Returns:**

``~models.vae.DecoderOutput` or `tuple``

If return_dict is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.
#### tiled_encode[[diffusers.AutoencoderKLHunyuanImageRefiner.tiled_encode]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_hunyuanimage_refiner.py#L793)

Encode a batch of images using a tiled encoder.

**Parameters:**

x (`torch.Tensor`) : Input batch of videos.

**Returns:**

``torch.Tensor``

The latent representation of the encoded videos.

## DecoderOutput[[diffusers.models.autoencoders.vae.DecoderOutput]]

#### diffusers.models.autoencoders.vae.DecoderOutput[[diffusers.models.autoencoders.vae.DecoderOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/vae.py#L46)

Output of decoding method.

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)`) : The decoded output sample from the last layer of the model.
