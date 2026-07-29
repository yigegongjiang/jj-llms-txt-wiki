# AutoencoderKLLTXVideo

The 3D variational autoencoder (VAE) model with KL loss used in [LTX](https://huggingface.co/Lightricks/LTX-Video) was introduced by Lightricks.

The model can be loaded with the following code snippet.

```python
from diffusers import AutoencoderKLLTXVideo

vae = AutoencoderKLLTXVideo.from_pretrained("Lightricks/LTX-Video", subfolder="vae", torch_dtype=torch.float32).to("cuda")
```

## AutoencoderKLLTXVideo[[diffusers.AutoencoderKLLTXVideo]]

#### diffusers.AutoencoderKLLTXVideo[[diffusers.AutoencoderKLLTXVideo]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_ltx.py#L1035)

A VAE model with KL loss for encoding images into latents and decoding latent representations into images. Used in
[LTX](https://huggingface.co/Lightricks/LTX-Video).

This model inherits from [ModelMixin](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for it's generic methods implemented
for all models (such as downloading or saving).

wrapperdiffusers.AutoencoderKLLTXVideo.decodehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/accelerate_utils.py#L43[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]

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
#### wrapper[[diffusers.AutoencoderKLLTXVideo.encode]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/accelerate_utils.py#L43)
#### enable_tiling[[diffusers.AutoencoderKLLTXVideo.enable_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_ltx.py#L1186)

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.

**Parameters:**

tile_sample_min_height (`int`, *optional*) : The minimum height required for a sample to be separated into tiles across the height dimension.

tile_sample_min_width (`int`, *optional*) : The minimum width required for a sample to be separated into tiles across the width dimension.

tile_sample_stride_height (`int`, *optional*) : The minimum amount of overlap between two consecutive vertical tiles. This is to ensure that there are no tiling artifacts produced across the height dimension.

tile_sample_stride_width (`int`, *optional*) : The stride between two consecutive horizontal tiles. This is to ensure that there are no tiling artifacts produced across the width dimension.
#### forward[[diffusers.AutoencoderKLLTXVideo.forward]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_ltx.py#L1517)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

temb (`torch.Tensor`, *optional*) : Optional timestep embedding tensor used to condition the decoder.

sample_posterior (`bool`, *optional*, defaults to `False`) : Whether to sample from the posterior.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `DecoderOutput` instead of a plain tuple.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make sampling deterministic.

**Returns:**

``~models.vae.DecoderOutput` or `tuple``

If `return_dict` is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.
#### tiled_decode[[diffusers.AutoencoderKLLTXVideo.tiled_decode]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_ltx.py#L1389)

Decode a batch of images using a tiled decoder.

**Parameters:**

z (`torch.Tensor`) : Input batch of latent vectors.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.vae.DecoderOutput` instead of a plain tuple.

**Returns:**

``~models.vae.DecoderOutput` or `tuple``

If return_dict is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.
#### tiled_encode[[diffusers.AutoencoderKLLTXVideo.tiled_encode]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_ltx.py#L1338)

Encode a batch of images using a tiled encoder.

**Parameters:**

x (`torch.Tensor`) : Input batch of videos.

**Returns:**

``torch.Tensor``

The latent representation of the encoded videos.

## AutoencoderKLOutput[[diffusers.models.modeling_outputs.AutoencoderKLOutput]]

#### diffusers.models.modeling_outputs.AutoencoderKLOutput[[diffusers.models.modeling_outputs.AutoencoderKLOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_outputs.py#L7)

Output of AutoencoderKL encoding method.

**Parameters:**

latent_dist (`DiagonalGaussianDistribution`) : Encoded outputs of `Encoder` represented as the mean and logvar of `DiagonalGaussianDistribution`. `DiagonalGaussianDistribution` allows for sampling latents from the distribution.

## DecoderOutput[[diffusers.models.autoencoders.vae.DecoderOutput]]

#### diffusers.models.autoencoders.vae.DecoderOutput[[diffusers.models.autoencoders.vae.DecoderOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/vae.py#L46)

Output of decoding method.

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)`) : The decoded output sample from the last layer of the model.
