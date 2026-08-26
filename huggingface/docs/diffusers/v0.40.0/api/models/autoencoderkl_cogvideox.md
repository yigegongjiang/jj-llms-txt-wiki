# AutoencoderKLCogVideoX

The 3D variational autoencoder (VAE) model with KL loss used in [CogVideoX](https://github.com/THUDM/CogVideo) was introduced in [CogVideoX: Text-to-Video Diffusion Models with An Expert Transformer](https://github.com/THUDM/CogVideo/blob/main/resources/CogVideoX.pdf) by Tsinghua University & ZhipuAI.

The model can be loaded with the following code snippet.

```python
from diffusers import AutoencoderKLCogVideoX

vae = AutoencoderKLCogVideoX.from_pretrained("THUDM/CogVideoX-2b", subfolder="vae", dtype=torch.float16).to("cuda")
```

## AutoencoderKLCogVideoX[[diffusers.AutoencoderKLCogVideoX]]

#### diffusers.AutoencoderKLCogVideoX[[diffusers.AutoencoderKLCogVideoX]]

```python
diffusers.AutoencoderKLCogVideoX(in_channels: int = 3, out_channels: int = 3, down_block_types: tuple = ('CogVideoXDownBlock3D', 'CogVideoXDownBlock3D', 'CogVideoXDownBlock3D', 'CogVideoXDownBlock3D'), up_block_types: tuple = ('CogVideoXUpBlock3D', 'CogVideoXUpBlock3D', 'CogVideoXUpBlock3D', 'CogVideoXUpBlock3D'), block_out_channels: tuple = (128, 256, 256, 512), latent_channels: int = 16, layers_per_block: int = 3, act_fn: str = 'silu', norm_eps: float = 1e-06, norm_num_groups: int = 32, temporal_compression_ratio: float = 4, sample_height: int = 480, sample_width: int = 720, scaling_factor: float = 1.15258426, shift_factor: float | None = None, latents_mean: tuple[float] | None = None, latents_std: tuple[float] | None = None, force_upcast: float = True, use_quant_conv: bool = False, use_post_quant_conv: bool = False, invert_scale_latents: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_cogvideox.py#L956)

**Parameters:**

in_channels (int, *optional*, defaults to 3) : Number of channels in the input image.

out_channels (int,  *optional*, defaults to 3) : Number of channels in the output.

down_block_types (`tuple[str]`, *optional*, defaults to `("DownEncoderBlock2D",)`) : tuple of downsample block types.

up_block_types (`tuple[str]`, *optional*, defaults to `("UpDecoderBlock2D",)`) : tuple of upsample block types.

block_out_channels (`tuple[int]`, *optional*, defaults to `(64,)`) : tuple of block output channels.

act_fn (`str`, *optional*, defaults to `"silu"`) : The activation function to use.

sample_size (`int`, *optional*, defaults to `32`) : Sample input size.

scaling_factor (`float`, *optional*, defaults to `1.15258426`) : The component-wise standard deviation of the trained latent space computed using the first batch of the training set. This is used to scale the latent space to have unit variance when training the diffusion model. The latents are scaled with the formula `z = z * scaling_factor` before being passed to the diffusion model. When decoding, the latents are scaled back to the original scale with the formula: `z = 1 / scaling_factor * z`. For more details, refer to sections 4.3.2 and D.1 of the [High-Resolution Image Synthesis with Latent Diffusion Models](https://huggingface.co/papers/2112.10752) paper.

force_upcast (`bool`, *optional*, default to `True`) : If enabled it will force the VAE to run in float32 for high image resolution pipelines, such as SD-XL. VAE can be fine-tuned / trained to a lower range without losing too much precision in which case `force_upcast` can be set to `False` - see: https://huggingface.co/madebyollin/sdxl-vae-fp16-fix

A VAE model with KL loss for encoding images into latents and decoding latent representations into images. Used in
[CogVideoX](https://github.com/THUDM/CogVideo).

This model inherits from [ModelMixin](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for it's generic methods implemented
for all models (such as downloading or saving).

#### decode[[diffusers.AutoencoderKLCogVideoX.decode]]

```python
decode(z: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_cogvideox.py#L1207)

**Parameters:**

z (`torch.Tensor`) : Input batch of latent vectors.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a `~models.vae.DecoderOutput` instead of a plain tuple.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If return_dict is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

Decode a batch of images.

#### encode[[diffusers.AutoencoderKLCogVideoX.encode]]

```python
encode(x: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_cogvideox.py#L1151)

**Parameters:**

x (`torch.Tensor`) : Input batch of images.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a `~models.autoencoder_kl.AutoencoderKLOutput` instead of a plain tuple.

**Returns:**

The latent representations of the encoded videos. If `return_dict` is True, a
`~models.autoencoder_kl.AutoencoderKLOutput` is returned, otherwise a plain `tuple` is returned.

Encode a batch of images into latents.

#### enable_tiling[[diffusers.AutoencoderKLCogVideoX.enable_tiling]]

```python
enable_tiling(tile_sample_min_height: int | None = None, tile_sample_min_width: int | None = None, tile_overlap_factor_height: float | None = None, tile_overlap_factor_width: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_cogvideox.py#L1089)

**Parameters:**

tile_sample_min_height (`int`, *optional*) : The minimum height required for a sample to be separated into tiles across the height dimension.

tile_sample_min_width (`int`, *optional*) : The minimum width required for a sample to be separated into tiles across the width dimension.

tile_overlap_factor_height (`int`, *optional*) : The minimum amount of overlap between two consecutive vertical tiles. This is to ensure that there are no tiling artifacts produced across the height dimension. Must be between 0 and 1. Setting a higher value might cause more tiles to be processed leading to slow down of the decoding process.

tile_overlap_factor_width (`int`, *optional*) : The minimum amount of overlap between two consecutive horizontal tiles. This is to ensure that there are no tiling artifacts produced across the width dimension. Must be between 0 and 1. Setting a higher value might cause more tiles to be processed leading to slow down of the decoding process.

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.

#### forward[[diffusers.AutoencoderKLCogVideoX.forward]]

```python
forward(sample: Tensor, sample_posterior: bool = False, return_dict: bool = True, generator: typing.Optional[torch.Generator] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_cogvideox.py#L1405)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

sample_posterior (`bool`, *optional*, defaults to `False`) : Whether to sample from the posterior.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `DecoderOutput` instead of a plain tuple.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make sampling deterministic.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If `return_dict` is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

#### tiled_decode[[diffusers.AutoencoderKLCogVideoX.tiled_decode]]

```python
tiled_decode(z: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_cogvideox.py#L1322)

**Parameters:**

z (`torch.Tensor`) : Input batch of latent vectors.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.vae.DecoderOutput` instead of a plain tuple.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If return_dict is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

Decode a batch of images using a tiled decoder.

#### tiled_encode[[diffusers.AutoencoderKLCogVideoX.tiled_encode]]

```python
tiled_encode(x: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_cogvideox.py#L1248)

**Parameters:**

x (`torch.Tensor`) : Input batch of videos.

**Returns:** `torch.Tensor`

The latent representation of the encoded videos.

Encode a batch of images using a tiled encoder.

When this option is enabled, the VAE will split the input tensor into tiles to compute encoding in several
steps. This is useful to keep memory use constant regardless of image size. The end result of tiled encoding is
different from non-tiled encoding because each tile uses a different encoder. To avoid tiling artifacts, the
tiles overlap and are blended together to form a smooth output. You may still see tile-sized changes in the
output, but they should be much less noticeable.

## AutoencoderKLOutput[[diffusers.models.modeling_outputs.AutoencoderKLOutput]]

#### diffusers.models.modeling_outputs.AutoencoderKLOutput[[diffusers.models.modeling_outputs.AutoencoderKLOutput]]

```python
diffusers.models.modeling_outputs.AutoencoderKLOutput(latent_dist: DiagonalGaussianDistribution)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/modeling_outputs.py#L7)

**Parameters:**

latent_dist (`DiagonalGaussianDistribution`) : Encoded outputs of `Encoder` represented as the mean and logvar of `DiagonalGaussianDistribution`. `DiagonalGaussianDistribution` allows for sampling latents from the distribution.

Output of AutoencoderKL encoding method.

## DecoderOutput[[diffusers.models.autoencoders.vae.DecoderOutput]]

#### diffusers.models.autoencoders.vae.DecoderOutput[[diffusers.models.autoencoders.vae.DecoderOutput]]

```python
diffusers.models.autoencoders.vae.DecoderOutput(sample: Tensor, commit_loss: typing.Optional[torch.FloatTensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/vae.py#L46)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)`) : The decoded output sample from the last layer of the model.

Output of decoding method.
