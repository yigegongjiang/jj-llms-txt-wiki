# VQModel

The VQ-VAE model was introduced in [Neural Discrete Representation Learning](https://huggingface.co/papers/1711.00937) by Aaron van den Oord, Oriol Vinyals and Koray Kavukcuoglu. The model is used in 🤗 Diffusers to decode latent representations into images. Unlike [AutoencoderKL](/docs/diffusers/v0.40.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL), the [VQModel](/docs/diffusers/v0.40.0/en/api/models/vq#diffusers.VQModel) works in a quantized latent space.

The abstract from the paper is:

*Learning useful representations without supervision remains a key challenge in machine learning. In this paper, we propose a simple yet powerful generative model that learns such discrete representations. Our model, the Vector Quantised-Variational AutoEncoder (VQ-VAE), differs from VAEs in two key ways: the encoder network outputs discrete, rather than continuous, codes; and the prior is learnt rather than static. In order to learn a discrete latent representation, we incorporate ideas from vector quantisation (VQ). Using the VQ method allows the model to circumvent issues of "posterior collapse" -- where the latents are ignored when they are paired with a powerful autoregressive decoder -- typically observed in the VAE framework. Pairing these representations with an autoregressive prior, the model can generate high quality images, videos, and speech as well as doing high quality speaker conversion and unsupervised learning of phonemes, providing further evidence of the utility of the learnt representations.*

## VQModel[[diffusers.VQModel]]

#### diffusers.VQModel[[diffusers.VQModel]]

```python
diffusers.VQModel(in_channels: int = 3, out_channels: int = 3, down_block_types: tuple = ('DownEncoderBlock2D',), up_block_types: tuple = ('UpDecoderBlock2D',), block_out_channels: tuple = (64,), layers_per_block: int = 1, act_fn: str = 'silu', latent_channels: int = 3, sample_size: int = 32, num_vq_embeddings: int = 256, norm_num_groups: int = 32, vq_embed_dim: int | None = None, scaling_factor: float = 0.18215, norm_type: str = 'group', mid_block_add_attention = True, lookup_from_codebook = False, force_upcast = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/vq_model.py#L40)

**Parameters:**

in_channels (int, *optional*, defaults to 3) : Number of channels in the input image.

out_channels (int,  *optional*, defaults to 3) : Number of channels in the output.

down_block_types (`tuple[str]`, *optional*, defaults to `("DownEncoderBlock2D",)`) : tuple of downsample block types.

up_block_types (`tuple[str]`, *optional*, defaults to `("UpDecoderBlock2D",)`) : tuple of upsample block types.

block_out_channels (`tuple[int]`, *optional*, defaults to `(64,)`) : tuple of block output channels.

layers_per_block (`int`, *optional*, defaults to `1`) : Number of layers per block.

act_fn (`str`, *optional*, defaults to `"silu"`) : The activation function to use.

latent_channels (`int`, *optional*, defaults to `3`) : Number of channels in the latent space.

sample_size (`int`, *optional*, defaults to `32`) : Sample input size.

num_vq_embeddings (`int`, *optional*, defaults to `256`) : Number of codebook vectors in the VQ-VAE.

norm_num_groups (`int`, *optional*, defaults to `32`) : Number of groups for normalization layers.

vq_embed_dim (`int`, *optional*) : Hidden dim of codebook vectors in the VQ-VAE.

scaling_factor (`float`, *optional*, defaults to `0.18215`) : The component-wise standard deviation of the trained latent space computed using the first batch of the training set. This is used to scale the latent space to have unit variance when training the diffusion model. The latents are scaled with the formula `z = z * scaling_factor` before being passed to the diffusion model. When decoding, the latents are scaled back to the original scale with the formula: `z = 1 / scaling_factor * z`. For more details, refer to sections 4.3.2 and D.1 of the [High-Resolution Image Synthesis with Latent Diffusion Models](https://huggingface.co/papers/2112.10752) paper.

norm_type (`str`, *optional*, defaults to `"group"`) : Type of normalization layer to use. Can be one of `"group"` or `"spatial"`.

A VQ-VAE model for decoding latent representations.

This model inherits from [ModelMixin](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for it's generic methods implemented
for all models (such as downloading or saving).

#### forward[[diffusers.VQModel.forward]]

```python
forward(sample: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/vq_model.py#L163)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [models.autoencoders.vq_model.VQEncoderOutput](/docs/diffusers/v0.40.0/en/api/models/vq#diffusers.models.autoencoders.vq_model.VQEncoderOutput) instead of a plain tuple.

**Returns:** [VQEncoderOutput](/docs/diffusers/v0.40.0/en/api/models/vq#diffusers.models.autoencoders.vq_model.VQEncoderOutput) or `tuple`

If return_dict is True, a [VQEncoderOutput](/docs/diffusers/v0.40.0/en/api/models/vq#diffusers.models.autoencoders.vq_model.VQEncoderOutput) is returned, otherwise a
plain `tuple` is returned.

The [VQModel](/docs/diffusers/v0.40.0/en/api/models/vq#diffusers.VQModel) forward method.

## VQEncoderOutput[[diffusers.models.autoencoders.vq_model.VQEncoderOutput]]

#### diffusers.models.autoencoders.vq_model.VQEncoderOutput[[diffusers.models.autoencoders.vq_model.VQEncoderOutput]]

```python
diffusers.models.autoencoders.vq_model.VQEncoderOutput(latents: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/vq_model.py#L28)

**Parameters:**

latents (`torch.Tensor` of shape `(batch_size, num_channels, height, width)`) : The encoded output sample from the last layer of the model.

Output of VQModel encoding method.
