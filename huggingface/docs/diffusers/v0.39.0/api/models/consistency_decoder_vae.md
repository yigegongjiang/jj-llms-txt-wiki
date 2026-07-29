# Consistency Decoder

Consistency decoder can be used to decode the latents from the denoising UNet in the [StableDiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/text2img#diffusers.StableDiffusionPipeline). This decoder was introduced in the [DALL-E 3 technical report](https://openai.com/dall-e-3).

The original codebase can be found at [openai/consistencydecoder](https://github.com/openai/consistencydecoder).

> [!WARNING]
> Inference is only supported for 2 iterations as of now.

The pipeline could not have been contributed without the help of [madebyollin](https://github.com/madebyollin) and [mrsteyk](https://github.com/mrsteyk) from [this issue](https://github.com/openai/consistencydecoder/issues/1).

## ConsistencyDecoderVAE[[diffusers.ConsistencyDecoderVAE]]
#### diffusers.ConsistencyDecoderVAE[[diffusers.ConsistencyDecoderVAE]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/consistency_decoder_vae.py#L51)

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

wrapperdiffusers.ConsistencyDecoderVAE.decodehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/accelerate_utils.py#L43[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]
#### forward[[diffusers.ConsistencyDecoderVAE.forward]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/consistency_decoder_vae.py#L336)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

sample_posterior (`bool`, *optional*, defaults to `False`) : Whether to sample from the posterior.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `DecoderOutput` instead of a plain tuple.

generator (`torch.Generator`, *optional*, defaults to `None`) : Generator to use for sampling.

**Returns:**

``DecoderOutput` or `tuple``

If return_dict is True, a `DecoderOutput` is returned, otherwise a plain `tuple` is returned.
#### set_default_attn_processor[[diffusers.ConsistencyDecoderVAE.set_default_attn_processor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/consistency_decoder_vae.py#L170)

Disables custom attention processors and sets the default attention implementation.
#### tiled_encode[[diffusers.ConsistencyDecoderVAE.tiled_encode]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/consistency_decoder_vae.py#L281)

Encode a batch of images using a tiled encoder.

When this option is enabled, the VAE will split the input tensor into tiles to compute encoding in several
steps. This is useful to keep memory use constant regardless of image size. The end result of tiled encoding is
different from non-tiled encoding because each tile uses a different encoder. To avoid tiling artifacts, the
tiles overlap and are blended together to form a smooth output. You may still see tile-sized changes in the
output, but they should be much less noticeable.

**Parameters:**

x (`torch.Tensor`) : Input batch of images.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `ConsistencyDecoderVAEOutput` instead of a plain tuple.

**Returns:**

``ConsistencyDecoderVAEOutput` or `tuple``

If return_dict is True, a `ConsistencyDecoderVAEOutput`
is returned, otherwise a plain `tuple` is returned.
