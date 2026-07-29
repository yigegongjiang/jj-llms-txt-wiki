# AutoencoderKLLTX2Audio

The 3D variational autoencoder (VAE) model with KL loss used in [LTX-2](https://huggingface.co/Lightricks/LTX-2) was introduced by Lightricks. This is for encoding and decoding audio latent representations.

The model can be loaded with the following code snippet.

```python
from diffusers import AutoencoderKLLTX2Audio

vae = AutoencoderKLLTX2Audio.from_pretrained("Lightricks/LTX-2", subfolder="vae", torch_dtype=torch.float32).to("cuda")
```

## AutoencoderKLLTX2Audio[[diffusers.AutoencoderKLLTX2Audio]]

#### diffusers.AutoencoderKLLTX2Audio[[diffusers.AutoencoderKLLTX2Audio]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_ltx2_audio.py#L668)

LTX2 audio VAE for encoding and decoding audio latent representations.

wrapperdiffusers.AutoencoderKLLTX2Audio.encodehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/accelerate_utils.py#L43[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]
#### wrapper[[diffusers.AutoencoderKLLTX2Audio.decode]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/accelerate_utils.py#L43)
#### forward[[diffusers.AutoencoderKLLTX2Audio.forward]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_kl_ltx2_audio.py#L788)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

sample_posterior (`bool`, *optional*, defaults to `False`) : Whether to sample from the posterior.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `DecoderOutput` instead of a plain tuple.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make sampling deterministic.

**Returns:**

``DecoderOutput` or `tuple``

If `return_dict` is True, a `DecoderOutput` is returned, otherwise a plain `tuple` is returned.
