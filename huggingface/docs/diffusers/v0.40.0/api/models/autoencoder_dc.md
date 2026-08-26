# AutoencoderDC

The 2D Autoencoder model used in [SANA](https://huggingface.co/papers/2410.10629) and introduced in [DCAE](https://huggingface.co/papers/2410.10733) by authors Junyu Chen\*, Han Cai\*, Junsong Chen, Enze Xie, Shang Yang, Haotian Tang, Muyang Li, Yao Lu, Song Han from MIT HAN Lab.

The abstract from the paper is:

*We present Deep Compression Autoencoder (DC-AE), a new family of autoencoder models for accelerating high-resolution diffusion models. Existing autoencoder models have demonstrated impressive results at a moderate spatial compression ratio (e.g., 8x), but fail to maintain satisfactory reconstruction accuracy for high spatial compression ratios (e.g., 64x). We address this challenge by introducing two key techniques: (1) Residual Autoencoding, where we design our models to learn residuals based on the space-to-channel transformed features to alleviate the optimization difficulty of high spatial-compression autoencoders; (2) Decoupled High-Resolution Adaptation, an efficient decoupled three-phases training strategy for mitigating the generalization penalty of high spatial-compression autoencoders. With these designs, we improve the autoencoder's spatial compression ratio up to 128 while maintaining the reconstruction quality. Applying our DC-AE to latent diffusion models, we achieve significant speedup without accuracy drop. For example, on ImageNet 512x512, our DC-AE provides 19.1x inference speedup and 17.9x training speedup on H100 GPU for UViT-H while achieving a better FID, compared with the widely used SD-VAE-f8 autoencoder. Our code is available at [this https URL](https://github.com/mit-han-lab/efficientvit).*

The following DCAE models are released and supported in Diffusers.

| Diffusers format | Original format |
|:----------------:|:---------------:|
| [`mit-han-lab/dc-ae-f32c32-sana-1.0-diffusers`](https://huggingface.co/mit-han-lab/dc-ae-f32c32-sana-1.0-diffusers) | [`mit-han-lab/dc-ae-f32c32-sana-1.0`](https://huggingface.co/mit-han-lab/dc-ae-f32c32-sana-1.0)
| [`mit-han-lab/dc-ae-f32c32-in-1.0-diffusers`](https://huggingface.co/mit-han-lab/dc-ae-f32c32-in-1.0-diffusers) | [`mit-han-lab/dc-ae-f32c32-in-1.0`](https://huggingface.co/mit-han-lab/dc-ae-f32c32-in-1.0)
| [`mit-han-lab/dc-ae-f32c32-mix-1.0-diffusers`](https://huggingface.co/mit-han-lab/dc-ae-f32c32-mix-1.0-diffusers) | [`mit-han-lab/dc-ae-f32c32-mix-1.0`](https://huggingface.co/mit-han-lab/dc-ae-f32c32-mix-1.0)
| [`mit-han-lab/dc-ae-f64c128-in-1.0-diffusers`](https://huggingface.co/mit-han-lab/dc-ae-f64c128-in-1.0-diffusers) | [`mit-han-lab/dc-ae-f64c128-in-1.0`](https://huggingface.co/mit-han-lab/dc-ae-f64c128-in-1.0)
| [`mit-han-lab/dc-ae-f64c128-mix-1.0-diffusers`](https://huggingface.co/mit-han-lab/dc-ae-f64c128-mix-1.0-diffusers) | [`mit-han-lab/dc-ae-f64c128-mix-1.0`](https://huggingface.co/mit-han-lab/dc-ae-f64c128-mix-1.0)
| [`mit-han-lab/dc-ae-f128c512-in-1.0-diffusers`](https://huggingface.co/mit-han-lab/dc-ae-f128c512-in-1.0-diffusers) | [`mit-han-lab/dc-ae-f128c512-in-1.0`](https://huggingface.co/mit-han-lab/dc-ae-f128c512-in-1.0)
| [`mit-han-lab/dc-ae-f128c512-mix-1.0-diffusers`](https://huggingface.co/mit-han-lab/dc-ae-f128c512-mix-1.0-diffusers) | [`mit-han-lab/dc-ae-f128c512-mix-1.0`](https://huggingface.co/mit-han-lab/dc-ae-f128c512-mix-1.0)

This model was contributed by [lawrence-cj](https://github.com/lawrence-cj).

Load a model in Diffusers format with [from_pretrained()](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin.from_pretrained).

```python
from diffusers import AutoencoderDC

ae = AutoencoderDC.from_pretrained("mit-han-lab/dc-ae-f32c32-sana-1.0-diffusers", dtype=torch.float32).to("cuda")
```

## Load a model in Diffusers via `from_single_file`

```python
from difusers import AutoencoderDC

ckpt_path = "https://huggingface.co/mit-han-lab/dc-ae-f32c32-sana-1.0/blob/main/model.safetensors"
model = AutoencoderDC.from_single_file(ckpt_path) 

```

The `AutoencoderDC` model has `in` and `mix` single file checkpoint variants that have matching checkpoint keys, but use different scaling factors. It is not possible for Diffusers to automatically infer the correct config file to use with the model based on just the checkpoint and will default to configuring the model using the `mix` variant config file. To override the automatically determined config, please use the `config` argument when using single file loading with `in` variant checkpoints. 

```python
from diffusers import AutoencoderDC

ckpt_path = "https://huggingface.co/mit-han-lab/dc-ae-f128c512-in-1.0/blob/main/model.safetensors"
model = AutoencoderDC.from_single_file(ckpt_path, config="mit-han-lab/dc-ae-f128c512-in-1.0-diffusers")
```

## AutoencoderDC[[diffusers.AutoencoderDC]]

#### diffusers.AutoencoderDC[[diffusers.AutoencoderDC]]

```python
diffusers.AutoencoderDC(in_channels: int = 3, latent_channels: int = 32, attention_head_dim: int = 32, encoder_block_types: str | tuple[str] = 'ResBlock', decoder_block_types: str | tuple[str] = 'ResBlock', encoder_block_out_channels: tuple = (128, 256, 512, 512, 1024, 1024), decoder_block_out_channels: tuple = (128, 256, 512, 512, 1024, 1024), encoder_layers_per_block: tuple = (2, 2, 2, 3, 3, 3), decoder_layers_per_block: tuple = (3, 3, 3, 3, 3, 3), encoder_qkv_multiscales: tuple = ((), (), (), (5,), (5,), (5,)), decoder_qkv_multiscales: tuple = ((), (), (), (5,), (5,), (5,)), upsample_block_type: str = 'pixel_shuffle', downsample_block_type: str = 'pixel_unshuffle', decoder_norm_types: str | tuple[str] = 'rms_norm', decoder_act_fns: str | tuple[str] = 'silu', encoder_out_shortcut: bool = True, decoder_in_shortcut: bool = True, decoder_conv_act_fn: str = 'relu', scaling_factor: float = 1.0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_dc.py#L380)

**Parameters:**

in_channels (`int`, defaults to `3`) : The number of input channels in samples.

latent_channels (`int`, defaults to `32`) : The number of channels in the latent space representation.

encoder_block_types (`str | tuple[str]`, defaults to `"ResBlock"`) : The type(s) of block to use in the encoder.

decoder_block_types (`str | tuple[str]`, defaults to `"ResBlock"`) : The type(s) of block to use in the decoder.

encoder_block_out_channels (`tuple[int, ...]`, defaults to `(128, 256, 512, 512, 1024, 1024)`) : The number of output channels for each block in the encoder.

decoder_block_out_channels (`tuple[int, ...]`, defaults to `(128, 256, 512, 512, 1024, 1024)`) : The number of output channels for each block in the decoder.

encoder_layers_per_block (`tuple[int]`, defaults to `(2, 2, 2, 3, 3, 3)`) : The number of layers per block in the encoder.

decoder_layers_per_block (`tuple[int]`, defaults to `(3, 3, 3, 3, 3, 3)`) : The number of layers per block in the decoder.

encoder_qkv_multiscales (`tuple[tuple[int, ...], ...]`, defaults to `((), (), (), (5,), (5,), (5,))`) : Multi-scale configurations for the encoder's QKV (query-key-value) transformations.

decoder_qkv_multiscales (`tuple[tuple[int, ...], ...]`, defaults to `((), (), (), (5,), (5,), (5,))`) : Multi-scale configurations for the decoder's QKV (query-key-value) transformations.

upsample_block_type (`str`, defaults to `"pixel_shuffle"`) : The type of block to use for upsampling in the decoder.

downsample_block_type (`str`, defaults to `"pixel_unshuffle"`) : The type of block to use for downsampling in the encoder.

decoder_norm_types (`str | tuple[str]`, defaults to `"rms_norm"`) : The normalization type(s) to use in the decoder.

decoder_act_fns (`str | tuple[str]`, defaults to `"silu"`) : The activation function(s) to use in the decoder.

encoder_out_shortcut  (`bool`, defaults to `True`) : Whether to use shortcut at the end of the encoder.

decoder_in_shortcut (`bool`, defaults to `True`) : Whether to use shortcut at the beginning of the decoder.

decoder_conv_act_fn (`str`, defaults to `"relu"`) : The activation function to use at the end of the decoder.

scaling_factor (`float`, defaults to `1.0`) : The multiplicative inverse of the root mean square of the latent features. This is used to scale the latent space to have unit variance when training the diffusion model. The latents are scaled with the formula `z = z * scaling_factor` before being passed to the diffusion model. When decoding, the latents are scaled back to the original scale with the formula: `z = 1 / scaling_factor * z`.

An Autoencoder model introduced in [DCAE](https://huggingface.co/papers/2410.10733) and used in
[SANA](https://huggingface.co/papers/2410.10629).

This model inherits from [ModelMixin](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for it's generic methods implemented
for all models (such as downloading or saving).

#### encode[[diffusers.AutoencoderDC.encode]]

```python
encode(x: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_dc.py#L548)

**Parameters:**

x (`torch.Tensor`) : Input batch of images.

return_dict (`bool`, defaults to `True`) : Whether to return a `~models.vae.EncoderOutput` instead of a plain tuple.

**Returns:**

The latent representations of the encoded videos. If `return_dict` is True, a
`~models.vae.EncoderOutput` is returned, otherwise a plain `tuple` is returned.

Encode a batch of images into latents.

#### decode[[diffusers.AutoencoderDC.decode]]

```python
decode(z: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_dc.py#L582)

**Parameters:**

z (`torch.Tensor`) : Input batch of latent vectors.

return_dict (`bool`, defaults to `True`) : Whether to return a `~models.vae.DecoderOutput` instead of a plain tuple.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If return_dict is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

Decode a batch of images.

#### enable_tiling[[diffusers.AutoencoderDC.enable_tiling]]

```python
enable_tiling(tile_sample_min_height: int | None = None, tile_sample_min_width: int | None = None, tile_sample_stride_height: float | None = None, tile_sample_stride_width: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_dc.py#L506)

**Parameters:**

tile_sample_min_height (`int`, *optional*) : The minimum height required for a sample to be separated into tiles across the height dimension.

tile_sample_min_width (`int`, *optional*) : The minimum width required for a sample to be separated into tiles across the width dimension.

tile_sample_stride_height (`int`, *optional*) : The minimum amount of overlap between two consecutive vertical tiles. This is to ensure that there are no tiling artifacts produced across the height dimension.

tile_sample_stride_width (`int`, *optional*) : The stride between two consecutive horizontal tiles. This is to ensure that there are no tiling artifacts produced across the width dimension.

Enable tiled AE decoding. When this option is enabled, the AE will split the input tensor into tiles to compute
decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.

#### forward[[diffusers.AutoencoderDC.forward]]

```python
forward(sample: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_dc.py#L708)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `DecoderOutput` instead of a plain tuple.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If `return_dict` is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

## DecoderOutput[[diffusers.models.autoencoders.vae.DecoderOutput]]

#### diffusers.models.autoencoders.vae.DecoderOutput[[diffusers.models.autoencoders.vae.DecoderOutput]]

```python
diffusers.models.autoencoders.vae.DecoderOutput(sample: Tensor, commit_loss: typing.Optional[torch.FloatTensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/vae.py#L46)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)`) : The decoded output sample from the last layer of the model.

Output of decoding method.
