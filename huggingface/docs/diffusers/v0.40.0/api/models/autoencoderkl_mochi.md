# AutoencoderKLMochi

The 3D variational autoencoder (VAE) model with KL loss used in [Mochi](https://github.com/genmoai/models) was introduced in [Mochi 1 Preview](https://huggingface.co/genmo/mochi-1-preview) by Tsinghua University & ZhipuAI.

The model can be loaded with the following code snippet.

```python
from diffusers import AutoencoderKLMochi

vae = AutoencoderKLMochi.from_pretrained("genmo/mochi-1-preview", subfolder="vae", dtype=torch.float32).to("cuda")
```

## AutoencoderKLMochi[[diffusers.AutoencoderKLMochi]]

#### diffusers.AutoencoderKLMochi[[diffusers.AutoencoderKLMochi]]

```python
diffusers.AutoencoderKLMochi(in_channels: int = 15, out_channels: int = 3, encoder_block_out_channels: tuple = (64, 128, 256, 384), decoder_block_out_channels: tuple = (128, 256, 512, 768), latent_channels: int = 12, layers_per_block: tuple = (3, 3, 4, 6, 3), act_fn: str = 'silu', temporal_expansions: tuple = (1, 2, 3), spatial_expansions: tuple = (2, 2, 2), add_attention_block: tuple = (False, True, True, True, True), latents_mean: tuple = (-0.06730895953510081, -0.038011381506090416, -0.07477820912866141, -0.05565264470995561, 0.012767231469026969, -0.04703542746246419, 0.043896967884726704, -0.09346305707025976, -0.09918314763016893, -0.008729793427399178, -0.011931556316503654, -0.0321993391887285), latents_std: tuple = (0.9263795028493863, 0.9248894543193766, 0.9393059390890617, 0.959253732819592, 0.8244560132752793, 0.917259975397747, 0.9294154431013696, 1.3720942357788521, 0.881393668867029, 0.9168315692124348, 0.9185249279345552, 0.9274757570805041), scaling_factor: float = 1.0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_mochi.py#L655)

**Parameters:**

in_channels (int, *optional*, defaults to 3) : Number of channels in the input image.

out_channels (int,  *optional*, defaults to 3) : Number of channels in the output.

block_out_channels (`tuple[int]`, *optional*, defaults to `(64,)`) : tuple of block output channels.

act_fn (`str`, *optional*, defaults to `"silu"`) : The activation function to use.

scaling_factor (`float`, *optional*, defaults to `1.15258426`) : The component-wise standard deviation of the trained latent space computed using the first batch of the training set. This is used to scale the latent space to have unit variance when training the diffusion model. The latents are scaled with the formula `z = z * scaling_factor` before being passed to the diffusion model. When decoding, the latents are scaled back to the original scale with the formula: `z = 1 / scaling_factor * z`. For more details, refer to sections 4.3.2 and D.1 of the [High-Resolution Image Synthesis with Latent Diffusion Models](https://huggingface.co/papers/2112.10752) paper.

A VAE model with KL loss for encoding images into latents and decoding latent representations into images. Used in
[Mochi 1 preview](https://github.com/genmoai/models).

This model inherits from [ModelMixin](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for it's generic methods implemented
for all models (such as downloading or saving).

#### decode[[diffusers.AutoencoderKLMochi.decode]]

```python
decode(z: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_mochi.py#L912)

**Parameters:**

z (`torch.Tensor`) : Input batch of latent vectors.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a `~models.vae.DecoderOutput` instead of a plain tuple.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If return_dict is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

Decode a batch of images.

#### enable_tiling[[diffusers.AutoencoderKLMochi.enable_tiling]]

```python
enable_tiling(tile_sample_min_height: int | None = None, tile_sample_min_width: int | None = None, tile_sample_stride_height: float | None = None, tile_sample_stride_width: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_mochi.py#L786)

**Parameters:**

tile_sample_min_height (`int`, *optional*) : The minimum height required for a sample to be separated into tiles across the height dimension.

tile_sample_min_width (`int`, *optional*) : The minimum width required for a sample to be separated into tiles across the width dimension.

tile_sample_stride_height (`int`, *optional*) : The minimum amount of overlap between two consecutive vertical tiles. This is to ensure that there are no tiling artifacts produced across the height dimension.

tile_sample_stride_width (`int`, *optional*) : The stride between two consecutive horizontal tiles. This is to ensure that there are no tiling artifacts produced across the width dimension.

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.

#### encode[[diffusers.AutoencoderKLMochi.encode]]

```python
encode(x: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_mochi.py#L855)

**Parameters:**

x (`torch.Tensor`) : Input batch of images.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a `~models.autoencoder_kl.AutoencoderKLOutput` instead of a plain tuple.

**Returns:**

The latent representations of the encoded videos. If `return_dict` is True, a
`~models.autoencoder_kl.AutoencoderKLOutput` is returned, otherwise a plain `tuple` is returned.

Encode a batch of images into latents.

#### forward[[diffusers.AutoencoderKLMochi.forward]]

```python
forward(sample: Tensor, sample_posterior: bool = False, return_dict: bool = True, generator: typing.Optional[torch.Generator] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_mochi.py#L1089)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

sample_posterior (`bool`, *optional*, defaults to `False`) : Whether to sample from the posterior.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `DecoderOutput` instead of a plain tuple.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make sampling deterministic.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If `return_dict` is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

#### tiled_decode[[diffusers.AutoencoderKLMochi.tiled_decode]]

```python
tiled_decode(z: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_mochi.py#L1011)

**Parameters:**

z (`torch.Tensor`) : Input batch of latent vectors.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.vae.DecoderOutput` instead of a plain tuple.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If return_dict is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

Decode a batch of images using a tiled decoder.

#### tiled_encode[[diffusers.AutoencoderKLMochi.tiled_encode]]

```python
tiled_encode(x: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_mochi.py#L954)

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
