# AsymmetricAutoencoderKL

Improved larger variational autoencoder (VAE) model with KL loss for inpainting task: [Designing a Better Asymmetric VQGAN for StableDiffusion](https://huggingface.co/papers/2306.04632) by Zixin Zhu, Xuelu Feng, Dongdong Chen, Jianmin Bao, Le Wang, Yinpeng Chen, Lu Yuan, Gang Hua.

The abstract from the paper is:

*StableDiffusion is a revolutionary text-to-image generator that is causing a stir in the world of image generation and editing. Unlike traditional methods that learn a diffusion model in pixel space, StableDiffusion learns a diffusion model in the latent space via a VQGAN, ensuring both efficiency and quality. It not only supports image generation tasks, but also enables image editing for real images, such as image inpainting and local editing. However, we have observed that the vanilla VQGAN used in StableDiffusion leads to significant information loss, causing distortion artifacts even in non-edited image regions. To this end, we propose a new asymmetric VQGAN with two simple designs. Firstly, in addition to the input from the encoder, the decoder contains a conditional branch that incorporates information from task-specific priors, such as the unmasked image region in inpainting. Secondly, the decoder is much heavier than the encoder, allowing for more detailed recovery while only slightly increasing the total inference cost. The training cost of our asymmetric VQGAN is cheap, and we only need to retrain a new asymmetric decoder while keeping the vanilla VQGAN encoder and StableDiffusion unchanged. Our asymmetric VQGAN can be widely used in StableDiffusion-based inpainting and local editing methods. Extensive experiments demonstrate that it can significantly improve the inpainting and editing performance, while maintaining the original text-to-image capability. The code is available at https://github.com/buxiangzhiren/Asymmetric_VQGAN*

Evaluation results can be found in section 4.1 of the original paper.

## Available checkpoints

* [https://huggingface.co/cross-attention/asymmetric-autoencoder-kl-x-1-5](https://huggingface.co/cross-attention/asymmetric-autoencoder-kl-x-1-5)
* [https://huggingface.co/cross-attention/asymmetric-autoencoder-kl-x-2](https://huggingface.co/cross-attention/asymmetric-autoencoder-kl-x-2)

## Example Usage

```python
from diffusers import AsymmetricAutoencoderKL, StableDiffusionInpaintPipeline
from diffusers.utils import load_image, make_image_grid

prompt = "a photo of a person with beard"
img_url = "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main/repaint/celeba_hq_256.png"
mask_url = "https://huggingface.co/datasets/hf-internal-testing/diffusers-images/resolve/main/repaint/mask_256.png"

original_image = load_image(img_url).resize((512, 512))
mask_image = load_image(mask_url).resize((512, 512))

pipe = StableDiffusionInpaintPipeline.from_pretrained("stable-diffusion-v1-5/stable-diffusion-inpainting")
pipe.vae = AsymmetricAutoencoderKL.from_pretrained("cross-attention/asymmetric-autoencoder-kl-x-1-5")
pipe.to("cuda")

image = pipe(prompt=prompt, image=original_image, mask_image=mask_image).images[0]
make_image_grid([original_image, mask_image, image], rows=1, cols=3)
```

## AsymmetricAutoencoderKL[[diffusers.AsymmetricAutoencoderKL]]

#### diffusers.AsymmetricAutoencoderKL[[diffusers.AsymmetricAutoencoderKL]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_asym_kl.py#L24)

Designing a Better Asymmetric VQGAN for StableDiffusion https://huggingface.co/papers/2306.04632 . A VAE model with
KL loss for encoding images into latents and decoding latent representations into images.

This model inherits from [ModelMixin](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for it's generic methods implemented
for all models (such as downloading or saving).

forwarddiffusers.AsymmetricAutoencoderKL.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/autoencoder_asym_kl.py#L153[{"name": "sample", "val": ": Tensor"}, {"name": "mask", "val": ": torch.Tensor | None = None"}, {"name": "sample_posterior", "val": ": bool = False"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "generator", "val": ": torch._C.Generator | None = None"}]- **sample** (`torch.Tensor`) -- Input sample.
- **mask** (`torch.Tensor`, *optional*, defaults to `None`) -- Optional inpainting mask.
- **sample_posterior** (`bool`, *optional*, defaults to `False`) --
  Whether to sample from the posterior.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `DecoderOutput` instead of a plain tuple.
- **generator** (`torch.Generator`, *optional*) --
  A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make sampling
  deterministic.0`DecoderOutput` or `tuple`If `return_dict` is True, a `DecoderOutput` is returned, otherwise a plain `tuple` is returned.

**Parameters:**

in_channels (int, *optional*, defaults to 3) : Number of channels in the input image.

out_channels (int,  *optional*, defaults to 3) : Number of channels in the output.

down_block_types (`tuple[str]`, *optional*, defaults to `("DownEncoderBlock2D",)`) : tuple of downsample block types.

down_block_out_channels (`tuple[int]`, *optional*, defaults to `(64,)`) : tuple of down block output channels.

layers_per_down_block (`int`, *optional*, defaults to `1`) : Number layers for down block.

up_block_types (`tuple[str]`, *optional*, defaults to `("UpDecoderBlock2D",)`) : tuple of upsample block types.

up_block_out_channels (`tuple[int]`, *optional*, defaults to `(64,)`) : tuple of up block output channels.

layers_per_up_block (`int`, *optional*, defaults to `1`) : Number layers for up block.

act_fn (`str`, *optional*, defaults to `"silu"`) : The activation function to use.

latent_channels (`int`, *optional*, defaults to 4) : Number of channels in the latent space.

sample_size (`int`, *optional*, defaults to `32`) : Sample input size.

norm_num_groups (`int`, *optional*, defaults to `32`) : Number of groups to use for the first normalization layer in ResNet blocks.

scaling_factor (`float`, *optional*, defaults to 0.18215) : The component-wise standard deviation of the trained latent space computed using the first batch of the training set. This is used to scale the latent space to have unit variance when training the diffusion model. The latents are scaled with the formula `z = z * scaling_factor` before being passed to the diffusion model. When decoding, the latents are scaled back to the original scale with the formula: `z = 1 / scaling_factor * z`. For more details, refer to sections 4.3.2 and D.1 of the [High-Resolution Image Synthesis with Latent Diffusion Models](https://huggingface.co/papers/2112.10752) paper.

**Returns:**

``DecoderOutput` or `tuple``

If `return_dict` is True, a `DecoderOutput` is returned, otherwise a plain `tuple` is returned.

## AutoencoderKLOutput[[diffusers.models.modeling_outputs.AutoencoderKLOutput]]

#### diffusers.models.modeling_outputs.AutoencoderKLOutput[[diffusers.models.modeling_outputs.AutoencoderKLOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_outputs.py#L7)

Output of AutoencoderKL encoding method.

**Parameters:**

latent_dist (`DiagonalGaussianDistribution`) : Encoded outputs of `Encoder` represented as the mean and logvar of `DiagonalGaussianDistribution`. `DiagonalGaussianDistribution` allows for sampling latents from the distribution.

## DecoderOutput[[diffusers.models.autoencoders.vae.DecoderOutput]]

#### diffusers.models.autoencoders.vae.DecoderOutput[[diffusers.models.autoencoders.vae.DecoderOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/autoencoders/vae.py#L46)

Output of decoding method.

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)`) : The decoded output sample from the last layer of the model.
