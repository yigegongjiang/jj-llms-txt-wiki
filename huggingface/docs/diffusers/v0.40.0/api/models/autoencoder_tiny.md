# Tiny AutoEncoder

Tiny AutoEncoder for Stable Diffusion (TAESD) was introduced in [madebyollin/taesd](https://github.com/madebyollin/taesd) by Ollin Boer Bohan. It is a tiny distilled version of Stable Diffusion's VAE that can quickly decode the latents in a [StableDiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/stable_diffusion/text2img#diffusers.StableDiffusionPipeline) or [StableDiffusionXLPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/stable_diffusion/stable_diffusion_xl#diffusers.StableDiffusionXLPipeline) almost instantly.

To use with Stable Diffusion v-2.1:

```python
import torch
from diffusers import DiffusionPipeline, AutoencoderTiny

pipe = DiffusionPipeline.from_pretrained(
    "stabilityai/stable-diffusion-2-1-base", dtype=torch.float16
)
pipe.vae = AutoencoderTiny.from_pretrained("madebyollin/taesd", dtype=torch.float16)
pipe = pipe.to("cuda")

prompt = "slice of delicious New York-style berry cheesecake"
image = pipe(prompt, num_inference_steps=25).images[0]
image
```

To use with Stable Diffusion XL 1.0

```python
import torch
from diffusers import DiffusionPipeline, AutoencoderTiny

pipe = DiffusionPipeline.from_pretrained(
    "stabilityai/stable-diffusion-xl-base-1.0", dtype=torch.float16
)
pipe.vae = AutoencoderTiny.from_pretrained("madebyollin/taesdxl", dtype=torch.float16)
pipe = pipe.to("cuda")

prompt = "slice of delicious New York-style berry cheesecake"
image = pipe(prompt, num_inference_steps=25).images[0]
image
```

## AutoencoderTiny[[diffusers.AutoencoderTiny]]

#### diffusers.AutoencoderTiny[[diffusers.AutoencoderTiny]]

```python
diffusers.AutoencoderTiny(in_channels: int = 3, out_channels: int = 3, encoder_block_out_channels: tuple = (64, 64, 64, 64), decoder_block_out_channels: tuple = (64, 64, 64, 64), act_fn: str = 'relu', upsample_fn: str = 'nearest', latent_channels: int = 4, upsampling_scaling_factor: int = 2, num_encoder_blocks: tuple = (1, 3, 3, 3), num_decoder_blocks: tuple = (3, 3, 3, 1), latent_magnitude: int = 3, latent_shift: float = 0.5, force_upcast: bool = False, scaling_factor: float = 1.0, shift_factor: float = 0.0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_tiny.py#L40)

**Parameters:**

in_channels (`int`, *optional*, defaults to 3) : Number of channels in the input image.

out_channels (`int`,  *optional*, defaults to 3) : Number of channels in the output.

encoder_block_out_channels (`tuple[int]`, *optional*, defaults to `(64, 64, 64, 64)`) : tuple of integers representing the number of output channels for each encoder block. The length of the tuple should be equal to the number of encoder blocks.

decoder_block_out_channels (`tuple[int]`, *optional*, defaults to `(64, 64, 64, 64)`) : tuple of integers representing the number of output channels for each decoder block. The length of the tuple should be equal to the number of decoder blocks.

act_fn (`str`, *optional*, defaults to `"relu"`) : Activation function to be used throughout the model.

latent_channels (`int`, *optional*, defaults to 4) : Number of channels in the latent representation. The latent space acts as a compressed representation of the input image.

upsampling_scaling_factor (`int`, *optional*, defaults to 2) : Scaling factor for upsampling in the decoder. It determines the size of the output image during the upsampling process.

num_encoder_blocks (`tuple[int]`, *optional*, defaults to `(1, 3, 3, 3)`) : tuple of integers representing the number of encoder blocks at each stage of the encoding process. The length of the tuple should be equal to the number of stages in the encoder. Each stage has a different number of encoder blocks.

num_decoder_blocks (`tuple[int]`, *optional*, defaults to `(3, 3, 3, 1)`) : tuple of integers representing the number of decoder blocks at each stage of the decoding process. The length of the tuple should be equal to the number of stages in the decoder. Each stage has a different number of decoder blocks.

latent_magnitude (`float`, *optional*, defaults to 3.0) : Magnitude of the latent representation. This parameter scales the latent representation values to control the extent of information preservation.

latent_shift (float, *optional*, defaults to 0.5) : Shift applied to the latent representation. This parameter controls the center of the latent space.

scaling_factor (`float`, *optional*, defaults to 1.0) : The component-wise standard deviation of the trained latent space computed using the first batch of the training set. This is used to scale the latent space to have unit variance when training the diffusion model. The latents are scaled with the formula `z = z * scaling_factor` before being passed to the diffusion model. When decoding, the latents are scaled back to the original scale with the formula: `z = 1 / scaling_factor * z`. For more details, refer to sections 4.3.2 and D.1 of the [High-Resolution Image Synthesis with Latent Diffusion Models](https://huggingface.co/papers/2112.10752) paper. For this Autoencoder, however, no such scaling factor was used, hence the value of 1.0 as the default.

force_upcast (`bool`, *optional*, default to `False`) : If enabled it will force the VAE to run in float32 for high image resolution pipelines, such as SD-XL. VAE can be fine-tuned / trained to a lower range without losing too much precision, in which case `force_upcast` can be set to `False` (see this fp16-friendly [AutoEncoder](https://huggingface.co/madebyollin/sdxl-vae-fp16-fix)).

A tiny distilled VAE model for encoding images into latents and decoding latent representations into images.

[AutoencoderTiny](/docs/diffusers/v0.40.0/en/api/models/autoencoder_tiny#diffusers.AutoencoderTiny) is a wrapper around the original implementation of `TAESD`.

This model inherits from [ModelMixin](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for its generic methods implemented for
all models (such as downloading or saving).

#### forward[[diffusers.AutoencoderTiny.forward]]

```python
forward(sample: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_tiny.py#L291)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `DecoderOutput` instead of a plain tuple.

**Returns:** `DecoderOutput` or `tuple`

If `return_dict` is True, a `DecoderOutput` is returned, otherwise a plain `tuple` is returned.

#### scale_latents[[diffusers.AutoencoderTiny.scale_latents]]

```python
scale_latents(x: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_tiny.py#L156)

raw latents -> [0, 1]

#### unscale_latents[[diffusers.AutoencoderTiny.unscale_latents]]

```python
unscale_latents(x: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_tiny.py#L160)

[0, 1] -> raw latents

## AutoencoderTinyOutput[[diffusers.models.autoencoders.autoencoder_tiny.AutoencoderTinyOutput]]

#### diffusers.models.autoencoders.autoencoder_tiny.AutoencoderTinyOutput[[diffusers.models.autoencoders.autoencoder_tiny.AutoencoderTinyOutput]]

```python
diffusers.models.autoencoders.autoencoder_tiny.AutoencoderTinyOutput(latents: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_tiny.py#L28)

**Parameters:**

latents (`torch.Tensor`) : Encoded outputs of the `Encoder`.

Output of AutoencoderTiny encoding method.
