# AutoencoderKLCosmos

[Cosmos Tokenizers](https://github.com/NVIDIA/Cosmos-Tokenizer).

Supported models:
- [nvidia/Cosmos-1.0-Tokenizer-CV8x8x8](https://huggingface.co/nvidia/Cosmos-1.0-Tokenizer-CV8x8x8)

The model can be loaded with the following code snippet.

```python
from diffusers import AutoencoderKLCosmos

vae = AutoencoderKLCosmos.from_pretrained("nvidia/Cosmos-1.0-Tokenizer-CV8x8x8", subfolder="vae")
```

## AutoencoderKLCosmos[[diffusers.AutoencoderKLCosmos]]

#### diffusers.AutoencoderKLCosmos[[diffusers.AutoencoderKLCosmos]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_cosmos.py#L879)

Autoencoder used in [Cosmos](https://huggingface.co/papers/2501.03575).

wrapperdiffusers.AutoencoderKLCosmos.decodehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/accelerate_utils.py#L43[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]

**Parameters:**

in_channels (`int`, defaults to `3`) : Number of input channels.

out_channels (`int`, defaults to `3`) : Number of output channels.

latent_channels (`int`, defaults to `16`) : Number of latent channels.

encoder_block_out_channels (`tuple[int, ...]`, defaults to `(128, 256, 512, 512)`) : Number of output channels for each encoder down block.

decode_block_out_channels (`tuple[int, ...]`, defaults to `(256, 512, 512, 512)`) : Number of output channels for each decoder up block.

attention_resolutions (`tuple[int, ...]`, defaults to `(32,)`) : list of image/video resolutions at which to apply attention.

resolution (`int`, defaults to `1024`) : Base image/video resolution used for computing whether a block should have attention layers.

num_layers (`int`, defaults to `2`) : Number of resnet blocks in each encoder/decoder block.

patch_size (`int`, defaults to `4`) : Patch size used for patching the input image/video.

patch_type (`str`, defaults to `haar`) : Patch type used for patching the input image/video. Can be either `haar` or `rearrange`.

scaling_factor (`float`, defaults to `1.0`) : The component-wise standard deviation of the trained latent space computed using the first batch of the training set. This is used to scale the latent space to have unit variance when training the diffusion model. The latents are scaled with the formula `z = z * scaling_factor` before being passed to the diffusion model. When decoding, the latents are scaled back to the original scale with the formula: `z = 1 / scaling_factor * z`. For more details, refer to sections 4.3.2 and D.1 of the [High-Resolution Image Synthesis with Latent Diffusion Models](https://huggingface.co/papers/2112.10752) paper. Not applicable in Cosmos, but we default to 1.0 for consistency.

spatial_compression_ratio (`int`, defaults to `8`) : The spatial compression ratio to apply in the VAE. The number of downsample blocks is determined using this.

temporal_compression_ratio (`int`, defaults to `8`) : The temporal compression ratio to apply in the VAE. The number of downsample blocks is determined using this.
#### wrapper[[diffusers.AutoencoderKLCosmos.encode]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/accelerate_utils.py#L43)
#### enable_tiling[[diffusers.AutoencoderKLCosmos.enable_tiling]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_cosmos.py#L1001)

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.

**Parameters:**

tile_sample_min_height (`int`, *optional*) : The minimum height required for a sample to be separated into tiles across the height dimension.

tile_sample_min_width (`int`, *optional*) : The minimum width required for a sample to be separated into tiles across the width dimension.

tile_sample_stride_height (`int`, *optional*) : The minimum amount of overlap between two consecutive vertical tiles. This is to ensure that there are no tiling artifacts produced across the height dimension.

tile_sample_stride_width (`int`, *optional*) : The stride between two consecutive horizontal tiles. This is to ensure that there are no tiling artifacts produced across the width dimension.
#### forward[[diffusers.AutoencoderKLCosmos.forward]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_cosmos.py#L1074)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

sample_posterior (`bool`, *optional*, defaults to `False`) : Whether to sample from the posterior.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `DecoderOutput` instead of a plain tuple.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make sampling deterministic.

**Returns:**

``~models.vae.DecoderOutput` or `tuple``

If `return_dict` is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

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
