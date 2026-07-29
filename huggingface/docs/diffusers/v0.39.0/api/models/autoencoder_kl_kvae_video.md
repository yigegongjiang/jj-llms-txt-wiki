# AutoencoderKLKVAEVideo

The 3D variational autoencoder (VAE) model with KL loss.

The model can be loaded with the following code snippet.

```python
import torch
from diffusers import AutoencoderKLKVAEVideo

vae = AutoencoderKLKVAEVideo.from_pretrained("kandinskylab/KVAE-3D-1.0", subfolder="diffusers", torch_dtype=torch.float16)
```

## AutoencoderKLKVAEVideo[[diffusers.AutoencoderKLKVAEVideo]]

#### diffusers.AutoencoderKLKVAEVideo[[diffusers.AutoencoderKLKVAEVideo]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_kvae_video.py#L707)

A VAE model with KL loss for encoding videos into latents and decoding latent representations into videos. Used in
[KVAE](https://github.com/kandinskylab/kvae-1).

This model inherits from [ModelMixin](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for its generic methods implemented for
all models (such as downloading or saving).

wrapperdiffusers.AutoencoderKLKVAEVideo.decodehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/accelerate_utils.py#L43[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]

**Parameters:**

ch (`int`, *optional*, defaults to 128) : Base channel count.

ch_mult (`Tuple[int]`, *optional*, defaults to `(1, 2, 4, 8)`) : Channel multipliers per level.

num_res_blocks (`int`, *optional*, defaults to 2) : Number of residual blocks per level.

in_channels (`int`, *optional*, defaults to 3) : Number of input channels.

out_ch (`int`, *optional*, defaults to 3) : Number of output channels.

z_channels (`int`, *optional*, defaults to 16) : Number of latent channels.

temporal_compress_times (`int`, *optional*, defaults to 4) : Temporal compression factor.
#### disable_slicing[[diffusers.AutoencoderKLKVAEVideo.disable_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_kvae_video.py#L844)

Disable sliced VAE decoding.
#### enable_slicing[[diffusers.AutoencoderKLKVAEVideo.enable_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_kvae_video.py#L840)

Enable sliced VAE decoding.
#### forward[[diffusers.AutoencoderKLKVAEVideo.forward]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_kvae_video.py#L938)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

sample_posterior (`bool`, *optional*, defaults to `False`) : Whether to sample from the posterior.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `DecoderOutput` instead of a plain tuple.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make sampling deterministic.

**Returns:**

``~models.vae.DecoderOutput` or `tuple``

If `return_dict` is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.
