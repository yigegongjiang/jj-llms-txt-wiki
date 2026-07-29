# AutoencoderKLAllegro

The 3D variational autoencoder (VAE) model with KL loss used in [Allegro](https://github.com/rhymes-ai/Allegro) was introduced in [Allegro: Open the Black Box of Commercial-Level Video Generation Model](https://huggingface.co/papers/2410.15458) by RhymesAI.

The model can be loaded with the following code snippet.

```python
from diffusers import AutoencoderKLAllegro

vae = AutoencoderKLAllegro.from_pretrained("rhymes-ai/Allegro", subfolder="vae", torch_dtype=torch.float32).to("cuda")
```

## AutoencoderKLAllegro[[diffusers.AutoencoderKLAllegro]]

#### diffusers.AutoencoderKLAllegro[[diffusers.AutoencoderKLAllegro]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_allegro.py#L676)

A VAE model with KL loss for encoding videos into latents and decoding latent representations into videos. Used in
[Allegro](https://github.com/rhymes-ai/Allegro).

This model inherits from [ModelMixin](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for it's generic methods implemented
for all models (such as downloading or saving).

wrapperdiffusers.AutoencoderKLAllegro.decodehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/accelerate_utils.py#L43[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]

**Parameters:**

in_channels (int, defaults to `3`) : Number of channels in the input image.

out_channels (int, defaults to `3`) : Number of channels in the output.

down_block_types (`tuple[str, ...]`, defaults to `("AllegroDownBlock3D", "AllegroDownBlock3D", "AllegroDownBlock3D", "AllegroDownBlock3D")`) : tuple of strings denoting which types of down blocks to use.

up_block_types (`tuple[str, ...]`, defaults to `("AllegroUpBlock3D", "AllegroUpBlock3D", "AllegroUpBlock3D", "AllegroUpBlock3D")`) : tuple of strings denoting which types of up blocks to use.

block_out_channels (`tuple[int, ...]`, defaults to `(128, 256, 512, 512)`) : tuple of integers denoting number of output channels in each block.

temporal_downsample_blocks (`tuple[bool, ...]`, defaults to `(True, True, False, False)`) : tuple of booleans denoting which blocks to enable temporal downsampling in.

latent_channels (`int`, defaults to `4`) : Number of channels in latents.

layers_per_block (`int`, defaults to `2`) : Number of resnet or attention or temporal convolution layers per down/up block.

act_fn (`str`, defaults to `"silu"`) : The activation function to use.

norm_num_groups (`int`, defaults to `32`) : Number of groups to use in normalization layers.

temporal_compression_ratio (`int`, defaults to `4`) : Ratio by which temporal dimension of samples are compressed.

sample_size (`int`, defaults to `320`) : Default latent size.

scaling_factor (`float`, defaults to `0.13235`) : The component-wise standard deviation of the trained latent space computed using the first batch of the training set. This is used to scale the latent space to have unit variance when training the diffusion model. The latents are scaled with the formula `z = z * scaling_factor` before being passed to the diffusion model. When decoding, the latents are scaled back to the original scale with the formula: `z = 1 / scaling_factor * z`. For more details, refer to sections 4.3.2 and D.1 of the [High-Resolution Image Synthesis with Latent Diffusion Models](https://huggingface.co/papers/2112.10752) paper.

force_upcast (`bool`, default to `True`) : If enabled it will force the VAE to run in float32 for high image resolution pipelines, such as SD-XL. VAE can be fine-tuned / trained to a lower range without losing too much precision in which case `force_upcast` can be set to `False` - see: https://huggingface.co/madebyollin/sdxl-vae-fp16-fix
#### wrapper[[diffusers.AutoencoderKLAllegro.encode]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/accelerate_utils.py#L43)
#### forward[[diffusers.AutoencoderKLAllegro.forward]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_allegro.py#L1041)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

sample_posterior (`bool`, *optional*, defaults to `False`) : Whether to sample from the posterior.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `DecoderOutput` instead of a plain tuple.

generator (`torch.Generator`, *optional*) : PyTorch random number generator.

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
