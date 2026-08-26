# AutoencoderKLLTX2Audio

The 3D variational autoencoder (VAE) model with KL loss used in [LTX-2](https://huggingface.co/Lightricks/LTX-2) was introduced by Lightricks. This is for encoding and decoding audio latent representations.

The model can be loaded with the following code snippet.

```python
from diffusers import AutoencoderKLLTX2Audio

vae = AutoencoderKLLTX2Audio.from_pretrained("Lightricks/LTX-2", subfolder="vae", dtype=torch.float32).to("cuda")
```

## AutoencoderKLLTX2Audio[[diffusers.AutoencoderKLLTX2Audio]]

#### diffusers.AutoencoderKLLTX2Audio[[diffusers.AutoencoderKLLTX2Audio]]

```python
diffusers.AutoencoderKLLTX2Audio(base_channels: int = 128, output_channels: int = 2, ch_mult: tuple = (1, 2, 4), num_res_blocks: int = 2, attn_resolutions: tuple[int, ...] | None = None, in_channels: int = 2, resolution: int = 256, latent_channels: int = 8, norm_type: str = 'pixel', causality_axis: str | None = 'height', dropout: float = 0.0, mid_block_add_attention: bool = False, sample_rate: int = 16000, mel_hop_length: int = 160, is_causal: bool = True, mel_bins: int | None = 64, double_z: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_ltx2_audio.py#L668)

LTX2 audio VAE for encoding and decoding audio latent representations.

#### encode[[diffusers.AutoencoderKLLTX2Audio.encode]]

```python
encode(x: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_ltx2_audio.py#L759)

#### decode[[diffusers.AutoencoderKLLTX2Audio.decode]]

```python
decode(z: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_ltx2_audio.py#L775)

#### forward[[diffusers.AutoencoderKLLTX2Audio.forward]]

```python
forward(sample: Tensor, sample_posterior: bool = False, return_dict: bool = True, generator: typing.Optional[torch.Generator] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_ltx2_audio.py#L788)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

sample_posterior (`bool`, *optional*, defaults to `False`) : Whether to sample from the posterior.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `DecoderOutput` instead of a plain tuple.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make sampling deterministic.

**Returns:** `DecoderOutput` or `tuple`

If `return_dict` is True, a `DecoderOutput` is returned, otherwise a plain `tuple` is returned.
