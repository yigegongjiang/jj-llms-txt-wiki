# Consistency Decoder

Consistency decoder can be used to decode the latents from the denoising UNet in the [StableDiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/stable_diffusion/text2img#diffusers.StableDiffusionPipeline). This decoder was introduced in the [DALL-E 3 technical report](https://openai.com/dall-e-3).

The original codebase can be found at [openai/consistencydecoder](https://github.com/openai/consistencydecoder).

> [!WARNING]
> Inference is only supported for 2 iterations as of now.

The pipeline could not have been contributed without the help of [madebyollin](https://github.com/madebyollin) and [mrsteyk](https://github.com/mrsteyk) from [this issue](https://github.com/openai/consistencydecoder/issues/1).

## ConsistencyDecoderVAE[[diffusers.ConsistencyDecoderVAE]]

#### diffusers.ConsistencyDecoderVAE[[diffusers.ConsistencyDecoderVAE]]

```python
diffusers.ConsistencyDecoderVAE(scaling_factor: float = 0.18215, latent_channels: int = 4, sample_size: int = 32, encoder_act_fn: str = 'silu', encoder_block_out_channels: tuple = (128, 256, 512, 512), encoder_double_z: bool = True, encoder_down_block_types: tuple = ('DownEncoderBlock2D', 'DownEncoderBlock2D', 'DownEncoderBlock2D', 'DownEncoderBlock2D'), encoder_in_channels: int = 3, encoder_layers_per_block: int = 2, encoder_norm_num_groups: int = 32, encoder_out_channels: int = 4, decoder_add_attention: bool = False, decoder_block_out_channels: tuple = (320, 640, 1024, 1024), decoder_down_block_types: tuple = ('ResnetDownsampleBlock2D', 'ResnetDownsampleBlock2D', 'ResnetDownsampleBlock2D', 'ResnetDownsampleBlock2D'), decoder_downsample_padding: int = 1, decoder_in_channels: int = 7, decoder_layers_per_block: int = 3, decoder_norm_eps: float = 1e-05, decoder_norm_num_groups: int = 32, decoder_num_train_timesteps: int = 1024, decoder_out_channels: int = 6, decoder_resnet_time_scale_shift: str = 'scale_shift', decoder_time_embedding_type: str = 'learned', decoder_up_block_types: tuple = ('ResnetUpsampleBlock2D', 'ResnetUpsampleBlock2D', 'ResnetUpsampleBlock2D', 'ResnetUpsampleBlock2D'))
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/consistency_decoder_vae.py#L51)

The consistency decoder used with DALL-E 3.

Examples:
```py
>>> import torch
>>> from diffusers import StableDiffusionPipeline, ConsistencyDecoderVAE

>>> vae = ConsistencyDecoderVAE.from_pretrained("openai/consistency-decoder", torch_dtype=torch.float16)
>>> pipe = StableDiffusionPipeline.from_pretrained(
...     "stable-diffusion-v1-5/stable-diffusion-v1-5", vae=vae, torch_dtype=torch.float16
... ).to("cuda")

>>> image = pipe("horse", generator=torch.manual_seed(0)).images[0]
>>> image
```

#### decode[[diffusers.ConsistencyDecoderVAE.decode]]

```python
decode(z: Tensor, generator: typing.Optional[torch.Generator] = None, return_dict: bool = True, num_inference_steps: int = 2)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/consistency_decoder_vae.py#L220)

**Parameters:**

z (torch.Tensor) : The input latent vector.

generator (torch.Generator | None) : The random number generator. Default is None.

return_dict (bool) : Whether to return the output as a dictionary. Default is True.

num_inference_steps (int) : The number of inference steps. Default is 2.

**Returns:** DecoderOutput | tuple[torch.Tensor]

The decoded output.

Decodes the input latent vector `z` using the consistency decoder VAE model.

#### encode[[diffusers.ConsistencyDecoderVAE.encode]]

```python
encode(x: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/consistency_decoder_vae.py#L185)

**Parameters:**

x (`torch.Tensor`) : Input batch of images.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a `ConsistencyDecoderVAEOutput` instead of a plain tuple.

**Returns:**

The latent representations of the encoded images. If `return_dict` is True, a
`ConsistencyDecoderVAEOutput` is returned, otherwise a
plain `tuple` is returned.

Encode a batch of images into latents.

#### forward[[diffusers.ConsistencyDecoderVAE.forward]]

```python
forward(sample: Tensor, sample_posterior: bool = False, return_dict: bool = True, generator: typing.Optional[torch.Generator] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/consistency_decoder_vae.py#L336)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

sample_posterior (`bool`, *optional*, defaults to `False`) : Whether to sample from the posterior.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `DecoderOutput` instead of a plain tuple.

generator (`torch.Generator`, *optional*, defaults to `None`) : Generator to use for sampling.

**Returns:** `DecoderOutput` or `tuple`

If return_dict is True, a `DecoderOutput` is returned, otherwise a plain `tuple` is returned.

#### set_default_attn_processor[[diffusers.ConsistencyDecoderVAE.set_default_attn_processor]]

```python
set_default_attn_processor()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/consistency_decoder_vae.py#L170)

Disables custom attention processors and sets the default attention implementation.

#### tiled_encode[[diffusers.ConsistencyDecoderVAE.tiled_encode]]

```python
tiled_encode(x: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/consistency_decoder_vae.py#L281)

**Parameters:**

x (`torch.Tensor`) : Input batch of images.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `ConsistencyDecoderVAEOutput` instead of a plain tuple.

**Returns:** `ConsistencyDecoderVAEOutput` or `tuple`

If return_dict is True, a `ConsistencyDecoderVAEOutput`
is returned, otherwise a plain `tuple` is returned.

Encode a batch of images using a tiled encoder.

When this option is enabled, the VAE will split the input tensor into tiles to compute encoding in several
steps. This is useful to keep memory use constant regardless of image size. The end result of tiled encoding is
different from non-tiled encoding because each tile uses a different encoder. To avoid tiling artifacts, the
tiles overlap and are blended together to form a smooth output. You may still see tile-sized changes in the
output, but they should be much less noticeable.
