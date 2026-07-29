# AutoencoderRAE

The Representation Autoencoder (RAE) model introduced in [Diffusion Transformers with Representation Autoencoders](https://huggingface.co/papers/2510.11690) by Boyang Zheng, Nanye Ma, Shengbang Tong, Saining Xie from NYU VISIONx.

RAE combines a frozen pretrained vision encoder (DINOv2, SigLIP2, or MAE) with a trainable ViT-MAE-style decoder. In the two-stage RAE training recipe, the autoencoder is trained in stage 1 (reconstruction), and then a diffusion model is trained on the resulting latent space in stage 2 (generation).

The following RAE models are released and supported in Diffusers:

| Model | Encoder | Latent shape (224px input) |
|:------|:--------|:---------------------------|
| [`nyu-visionx/RAE-dinov2-wReg-base-ViTXL-n08`](https://huggingface.co/nyu-visionx/RAE-dinov2-wReg-base-ViTXL-n08) | DINOv2-base | 768 x 16 x 16 |
| [`nyu-visionx/RAE-dinov2-wReg-base-ViTXL-n08-i512`](https://huggingface.co/nyu-visionx/RAE-dinov2-wReg-base-ViTXL-n08-i512) | DINOv2-base (512px) | 768 x 32 x 32 |
| [`nyu-visionx/RAE-dinov2-wReg-small-ViTXL-n08`](https://huggingface.co/nyu-visionx/RAE-dinov2-wReg-small-ViTXL-n08) | DINOv2-small | 384 x 16 x 16 |
| [`nyu-visionx/RAE-dinov2-wReg-large-ViTXL-n08`](https://huggingface.co/nyu-visionx/RAE-dinov2-wReg-large-ViTXL-n08) | DINOv2-large | 1024 x 16 x 16 |
| [`nyu-visionx/RAE-siglip2-base-p16-i256-ViTXL-n08`](https://huggingface.co/nyu-visionx/RAE-siglip2-base-p16-i256-ViTXL-n08) | SigLIP2-base | 768 x 16 x 16 |
| [`nyu-visionx/RAE-mae-base-p16-ViTXL-n08`](https://huggingface.co/nyu-visionx/RAE-mae-base-p16-ViTXL-n08) | MAE-base | 768 x 16 x 16 |

## Loading a pretrained model

```python
from diffusers import AutoencoderRAE

model = AutoencoderRAE.from_pretrained(
    "nyu-visionx/RAE-dinov2-wReg-base-ViTXL-n08"
).to("cuda").eval()
```

## Encoding and decoding a real image

```python
import torch
from diffusers import AutoencoderRAE
from diffusers.utils import load_image
from torchvision.transforms.functional import to_tensor, to_pil_image

model = AutoencoderRAE.from_pretrained(
    "nyu-visionx/RAE-dinov2-wReg-base-ViTXL-n08"
).to("cuda").eval()

image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/cat.png")
image = image.convert("RGB").resize((224, 224))
x = to_tensor(image).unsqueeze(0).to("cuda")  # (1, 3, 224, 224), values in [0, 1]

with torch.no_grad():
    latents = model.encode(x).latent        # (1, 768, 16, 16)
    recon = model.decode(latents).sample     # (1, 3, 256, 256)

recon_image = to_pil_image(recon[0].clamp(0, 1).cpu())
recon_image.save("recon.png")
```

## Latent normalization

Some pretrained checkpoints include per-channel `latents_mean` and `latents_std` statistics for normalizing the latent space. When present, `encode` and `decode` automatically apply the normalization and denormalization, respectively.

```python
model = AutoencoderRAE.from_pretrained(
    "nyu-visionx/RAE-dinov2-wReg-base-ViTXL-n08"
).to("cuda").eval()

# Latent normalization is handled automatically inside encode/decode
# when the checkpoint config includes latents_mean/latents_std.
with torch.no_grad():
    latents = model.encode(x).latent   # normalized latents
    recon = model.decode(latents).sample
```

## AutoencoderRAE[[diffusers.AutoencoderRAE]]

#### diffusers.AutoencoderRAE[[diffusers.AutoencoderRAE]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_rae.py#L393)

Representation Autoencoder (RAE) model for encoding images to latents and decoding latents to images.

This model uses a frozen pretrained encoder (DINOv2, SigLIP2, or MAE) with a trainable ViT decoder to reconstruct
images from learned representations.

This model inherits from [ModelMixin](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for its generic methods implemented for
all models (such as downloading or saving).

wrapperdiffusers.AutoencoderRAE.encodehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/accelerate_utils.py#L43[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]

**Parameters:**

encoder_type (`str`, *optional*, defaults to `"dinov2"`) : Type of frozen encoder to use. One of `"dinov2"`, `"siglip2"`, or `"mae"`.

encoder_hidden_size (`int`, *optional*, defaults to `768`) : Hidden size of the encoder model.

encoder_patch_size (`int`, *optional*, defaults to `14`) : Patch size of the encoder model.

encoder_num_hidden_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the encoder model.

patch_size (`int`, *optional*, defaults to `16`) : Decoder patch size (used for unpatchify and decoder head).

encoder_input_size (`int`, *optional*, defaults to `224`) : Input size expected by the encoder.

image_size (`int`, *optional*) : Decoder output image size. If `None`, it is derived from encoder token count and `patch_size` like RAE-main: `image_size = patch_size * sqrt(num_patches)`, where `num_patches = (encoder_input_size // encoder_patch_size) ** 2`.

num_channels (`int`, *optional*, defaults to `3`) : Number of input/output channels.

encoder_norm_mean (`list`, *optional*, defaults to `[0.485, 0.456, 0.406]`) : Channel-wise mean for encoder input normalization (ImageNet defaults).

encoder_norm_std (`list`, *optional*, defaults to `[0.229, 0.224, 0.225]`) : Channel-wise std for encoder input normalization (ImageNet defaults).

latents_mean (`list` or `tuple`, *optional*) : Optional mean for latent normalization. Tensor inputs are accepted and converted to config-serializable lists.

latents_std (`list` or `tuple`, *optional*) : Optional standard deviation for latent normalization. Tensor inputs are accepted and converted to config-serializable lists.

noise_tau (`float`, *optional*, defaults to `0.0`) : Noise level for training (adds noise to latents during training).

reshape_to_2d (`bool`, *optional*, defaults to `True`) : Whether to reshape latents to 2D (B, C, H, W) format.

use_encoder_loss (`bool`, *optional*, defaults to `False`) : Whether to use encoder hidden states in the loss (for advanced training).
#### wrapper[[diffusers.AutoencoderRAE.decode]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/accelerate_utils.py#L43)
#### forward[[diffusers.AutoencoderRAE.forward]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_rae.py#L682)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `DecoderOutput` instead of a plain tuple.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make sampling deterministic.

**Returns:**

``DecoderOutput` or `tuple``

If `return_dict` is True, a `DecoderOutput` is returned, otherwise a plain `tuple` is returned.

## DecoderOutput[[diffusers.models.autoencoders.vae.DecoderOutput]]

#### diffusers.models.autoencoders.vae.DecoderOutput[[diffusers.models.autoencoders.vae.DecoderOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/vae.py#L46)

Output of decoding method.

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)`) : The decoded output sample from the last layer of the model.
