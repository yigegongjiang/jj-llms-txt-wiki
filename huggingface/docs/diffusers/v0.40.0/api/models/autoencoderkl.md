# AutoencoderKL

The variational autoencoder (VAE) model with KL loss was introduced in [Auto-Encoding Variational Bayes](https://huggingface.co/papers/1312.6114v11) by Diederik P. Kingma and Max Welling. The model is used in 🤗 Diffusers to encode images into latents and to decode latent representations into images.

The abstract from the paper is:

*How can we perform efficient inference and learning in directed probabilistic models, in the presence of continuous latent variables with intractable posterior distributions, and large datasets? We introduce a stochastic variational inference and learning algorithm that scales to large datasets and, under some mild differentiability conditions, even works in the intractable case. Our contributions are two-fold. First, we show that a reparameterization of the variational lower bound yields a lower bound estimator that can be straightforwardly optimized using standard stochastic gradient methods. Second, we show that for i.i.d. datasets with continuous latent variables per datapoint, posterior inference can be made especially efficient by fitting an approximate inference model (also called a recognition model) to the intractable posterior using the proposed lower bound estimator. Theoretical advantages are reflected in experimental results.*

## Loading from the original format

By default the [AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL) should be loaded with [from_pretrained()](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin.from_pretrained), but it can also be loaded
from the original format using `FromOriginalModelMixin.from_single_file` as follows:

```py
from diffusers import AutoencoderKL

url = "https://huggingface.co/stabilityai/sd-vae-ft-mse-original/blob/main/vae-ft-mse-840000-ema-pruned.safetensors"  # can also be a local file
model = AutoencoderKL.from_single_file(url)
```

## AutoencoderKL[[diffusers.AutoencoderKL]]

#### diffusers.AutoencoderKL[[diffusers.AutoencoderKL]]

```python
diffusers.AutoencoderKL(in_channels: int = 3, out_channels: int = 3, down_block_types: tuple = ('DownEncoderBlock2D',), up_block_types: tuple = ('UpDecoderBlock2D',), block_out_channels: tuple = (64,), layers_per_block: int = 1, act_fn: str = 'silu', latent_channels: int = 4, norm_num_groups: int = 32, sample_size: int = 32, scaling_factor: float = 0.18215, shift_factor: float | None = None, latents_mean: tuple[float] | None = None, latents_std: tuple[float] | None = None, force_upcast: bool = True, use_quant_conv: bool = True, use_post_quant_conv: bool = True, mid_block_add_attention: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl.py#L36)

**Parameters:**

in_channels (int, *optional*, defaults to 3) : Number of channels in the input image.

out_channels (int,  *optional*, defaults to 3) : Number of channels in the output.

down_block_types (`tuple[str]`, *optional*, defaults to `("DownEncoderBlock2D",)`) : tuple of downsample block types.

up_block_types (`tuple[str]`, *optional*, defaults to `("UpDecoderBlock2D",)`) : tuple of upsample block types.

block_out_channels (`tuple[int]`, *optional*, defaults to `(64,)`) : tuple of block output channels.

act_fn (`str`, *optional*, defaults to `"silu"`) : The activation function to use.

latent_channels (`int`, *optional*, defaults to 4) : Number of channels in the latent space.

sample_size (`int`, *optional*, defaults to `32`) : Sample input size.

scaling_factor (`float`, *optional*, defaults to 0.18215) : The component-wise standard deviation of the trained latent space computed using the first batch of the training set. This is used to scale the latent space to have unit variance when training the diffusion model. The latents are scaled with the formula `z = z * scaling_factor` before being passed to the diffusion model. When decoding, the latents are scaled back to the original scale with the formula: `z = 1 / scaling_factor * z`. For more details, refer to sections 4.3.2 and D.1 of the [High-Resolution Image Synthesis with Latent Diffusion Models](https://huggingface.co/papers/2112.10752) paper.

force_upcast (`bool`, *optional*, default to `True`) : If enabled it will force the VAE to run in float32 for high image resolution pipelines, such as SD-XL. VAE can be fine-tuned / trained to a lower range without losing too much precision in which case `force_upcast` can be set to `False` - see: https://huggingface.co/madebyollin/sdxl-vae-fp16-fix

mid_block_add_attention (`bool`, *optional*, default to `True`) : If enabled, the mid_block of the Encoder and Decoder will have attention blocks. If set to false, the mid_block will only have resnet blocks

A VAE model with KL loss for encoding images into latents and decoding latent representations into images.

This model inherits from [ModelMixin](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for it's generic methods implemented
for all models (such as downloading or saving).

#### decode[[diffusers.AutoencoderKL.decode]]

```python
decode(z: FloatTensor, return_dict: bool = True, generator = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl.py#L213)

**Parameters:**

z (`torch.Tensor`) : Input batch of latent vectors.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a `~models.vae.DecoderOutput` instead of a plain tuple.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If return_dict is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

Decode a batch of images.

#### encode[[diffusers.AutoencoderKL.encode]]

```python
encode(x: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl.py#L170)

**Parameters:**

x (`torch.Tensor`) : Input batch of images.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a `~models.autoencoder_kl.AutoencoderKLOutput` instead of a plain tuple.

**Returns:**

The latent representations of the encoded images. If `return_dict` is True, a
`~models.autoencoder_kl.AutoencoderKLOutput` is returned, otherwise a plain `tuple` is returned.

Encode a batch of images into latents.

#### forward[[diffusers.AutoencoderKL.forward]]

```python
forward(sample: Tensor, sample_posterior: bool = False, return_dict: bool = True, generator: typing.Optional[torch.Generator] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl.py#L413)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

sample_posterior (`bool`, *optional*, defaults to `False`) : Whether to sample from the posterior.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `DecoderOutput` instead of a plain tuple.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make sampling deterministic.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If `return_dict` is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

#### fuse_qkv_projections[[diffusers.AutoencoderKL.fuse_qkv_projections]]

```python
fuse_qkv_projections()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl.py#L450)

Enables fused QKV projections. For self-attention modules, all projection matrices (i.e., query, key, value)
are fused. For cross-attention modules, key and value projection matrices are fused.

> [!WARNING] > This API is 🧪 experimental.

#### set_default_attn_processor[[diffusers.AutoencoderKL.set_default_attn_processor]]

```python
set_default_attn_processor()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl.py#L143)

Disables custom attention processors and sets the default attention implementation.

#### tiled_decode[[diffusers.AutoencoderKL.tiled_decode]]

```python
tiled_decode(z: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl.py#L364)

**Parameters:**

z (`torch.Tensor`) : Input batch of latent vectors.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.vae.DecoderOutput` instead of a plain tuple.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If return_dict is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

Decode a batch of images using a tiled decoder.

#### tiled_encode[[diffusers.AutoencoderKL.tiled_encode]]

```python
tiled_encode(x: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl.py#L302)

**Parameters:**

x (`torch.Tensor`) : Input batch of images.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.autoencoder_kl.AutoencoderKLOutput` instead of a plain tuple.

**Returns:** `~models.autoencoder_kl.AutoencoderKLOutput` or `tuple`

If return_dict is True, a `~models.autoencoder_kl.AutoencoderKLOutput` is returned, otherwise a plain
`tuple` is returned.

Encode a batch of images using a tiled encoder.

When this option is enabled, the VAE will split the input tensor into tiles to compute encoding in several
steps. This is useful to keep memory use constant regardless of image size. The end result of tiled encoding is
different from non-tiled encoding because each tile uses a different encoder. To avoid tiling artifacts, the
tiles overlap and are blended together to form a smooth output. You may still see tile-sized changes in the
output, but they should be much less noticeable.

#### unfuse_qkv_projections[[diffusers.AutoencoderKL.unfuse_qkv_projections]]

```python
unfuse_qkv_projections()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl.py#L472)

Disables the fused QKV projection if enabled.

> [!WARNING] > This API is 🧪 experimental.

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
