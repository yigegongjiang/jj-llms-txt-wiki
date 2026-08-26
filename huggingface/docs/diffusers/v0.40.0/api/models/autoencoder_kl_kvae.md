# AutoencoderKLKVAE

The 2D variational autoencoder (VAE) model with KL loss.

The model can be loaded with the following code snippet.

```python
import torch
from diffusers import AutoencoderKLKVAE

vae = AutoencoderKLKVAE.from_pretrained("kandinskylab/KVAE-2D-1.0", subfolder="diffusers", dtype=torch.bfloat16)
```

## AutoencoderKLKVAE[[diffusers.AutoencoderKLKVAE]]

#### diffusers.AutoencoderKLKVAE[[diffusers.AutoencoderKLKVAE]]

```python
diffusers.AutoencoderKLKVAE(in_channels: int = 3, channels: int = 128, num_enc_blocks: int = 2, num_dec_blocks: int = 2, z_channels: int = 16, double_z: bool = True, ch_mult: typing.Tuple[int, ...] = (1, 2, 4, 8), sample_size: int = 1024)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_kvae.py#L521)

**Parameters:**

in_channels (int, *optional*, defaults to 3) : Number of channels in the input image.

channels (int,  *optional*, defaults to 128) : The base number of channels in multiresolution blocks.

num_enc_blocks (int, *optional*, defaults to 2) : The number of Resnet blocks in encoder multiresolution layers.

num_dec_blocks (int, *optional*, defaults to 2) : The number of Resnet blocks in decoder multiresolution layers.

z_channels (int, *optional*, defaults to 16) : Number of channels in the latent space.

double_z (`bool`, *optional*, defaults to `True`) : Whether to double the number of output channels of encoder.

ch_mult (`Tuple[int, ...]`, *optional*, default to `(1, 2, 4, 8)`) : The channel multipliers in multiresolution blocks.

sample_size (`int`, *optional*, defaults to `1024`) : Sample input size.

A VAE model with KL loss for encoding images into latents and decoding latent representations into images.

This model inherits from [ModelMixin](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for its generic methods implemented for
all models (such as downloading or saving).

#### decode[[diffusers.AutoencoderKLKVAE.decode]]

```python
decode(z: FloatTensor, return_dict: bool = True, generator = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_kvae.py#L642)

**Parameters:**

z (`torch.Tensor`) : Input batch of latent vectors.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a `~models.vae.DecoderOutput` instead of a plain tuple.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If return_dict is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

Decode a batch of images.

#### encode[[diffusers.AutoencoderKLKVAE.encode]]

```python
encode(x: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_kvae.py#L602)

**Parameters:**

x (`torch.Tensor`) : Input batch of images.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a `~models.autoencoder_kl.AutoencoderKLOutput` instead of a plain tuple.

**Returns:**

The latent representations of the encoded images. If `return_dict` is True, a
`~models.autoencoder_kl.AutoencoderKLOutput` is returned, otherwise a plain `tuple` is returned.

Encode a batch of images into latents.

#### forward[[diffusers.AutoencoderKLKVAE.forward]]

```python
forward(sample: Tensor, sample_posterior: bool = False, return_dict: bool = True, generator: typing.Optional[torch.Generator] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_kvae.py#L776)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

sample_posterior (`bool`, *optional*, defaults to `False`) : Whether to sample from the posterior.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `DecoderOutput` instead of a plain tuple.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make sampling deterministic.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If `return_dict` is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

#### tiled_decode[[diffusers.AutoencoderKLKVAE.tiled_decode]]

```python
tiled_decode(z: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_kvae.py#L729)

**Parameters:**

z (`torch.Tensor`) : Input batch of latent vectors.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.vae.DecoderOutput` instead of a plain tuple.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If return_dict is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

Decode a batch of images using a tiled decoder.
