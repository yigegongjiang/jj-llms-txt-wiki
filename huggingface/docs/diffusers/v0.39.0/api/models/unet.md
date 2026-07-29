# UNet1DModel

The [UNet](https://huggingface.co/papers/1505.04597) model was originally introduced by Ronneberger et al. for biomedical image segmentation, but it is also commonly used in 🤗 Diffusers because it outputs images that are the same size as the input. It is one of the most important components of a diffusion system because it facilitates the actual diffusion process. There are several variants of the UNet model in 🤗 Diffusers, depending on it's number of dimensions and whether it is a conditional model or not. This is a 1D UNet model.

The abstract from the paper is:

*There is large consent that successful training of deep networks requires many thousand annotated training samples. In this paper, we present a network and training strategy that relies on the strong use of data augmentation to use the available annotated samples more efficiently. The architecture consists of a contracting path to capture context and a symmetric expanding path that enables precise localization. We show that such a network can be trained end-to-end from very few images and outperforms the prior best method (a sliding-window convolutional network) on the ISBI challenge for segmentation of neuronal structures in electron microscopic stacks. Using the same network trained on transmitted light microscopy images (phase contrast and DIC) we won the ISBI cell tracking challenge 2015 in these categories by a large margin. Moreover, the network is fast. Segmentation of a 512x512 image takes less than a second on a recent GPU. The full implementation (based on Caffe) and the trained networks are available at http://lmb.informatik.uni-freiburg.de/people/ronneber/u-net.*

## UNet1DModel[[diffusers.UNet1DModel]]
#### diffusers.UNet1DModel[[diffusers.UNet1DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/unets/unet_1d.py#L40)

A 1D UNet model that takes a noisy sample and a timestep and returns a sample shaped output.

This model inherits from [ModelMixin](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for it's generic methods implemented
for all models (such as downloading or saving).

forwarddiffusers.UNet1DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/unets/unet_1d.py#L205[{"name": "sample", "val": ": Tensor"}, {"name": "timestep", "val": ": torch.Tensor | float | int"}, {"name": "return_dict", "val": ": bool = True"}]- **sample** (`torch.Tensor`) --
  The noisy input tensor with the following shape `(batch_size, num_channels, sample_size)`.
- **timestep** (`torch.Tensor` or `float` or `int`) -- The number of timesteps to denoise an input.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a [UNet1DOutput](/docs/diffusers/v0.39.0/en/api/models/unet#diffusers.models.unets.unet_1d.UNet1DOutput) instead of a plain tuple.0[UNet1DOutput](/docs/diffusers/v0.39.0/en/api/models/unet#diffusers.models.unets.unet_1d.UNet1DOutput) or `tuple`If `return_dict` is True, an [UNet1DOutput](/docs/diffusers/v0.39.0/en/api/models/unet#diffusers.models.unets.unet_1d.UNet1DOutput) is returned, otherwise a `tuple` is
returned where the first element is the sample tensor.

The [UNet1DModel](/docs/diffusers/v0.39.0/en/api/models/unet#diffusers.UNet1DModel) forward method.

**Parameters:**

sample_size (`int`, *optional*) : Default length of sample. Should be adaptable at runtime.

in_channels (`int`, *optional*, defaults to 2) : Number of channels in the input sample.

out_channels (`int`, *optional*, defaults to 2) : Number of channels in the output.

extra_in_channels (`int`, *optional*, defaults to 0) : Number of additional channels to be added to the input of the first down block. Useful for cases where the input data has more channels than what the model was initially designed for.

time_embedding_type (`str`, *optional*, defaults to `"fourier"`) : Type of time embedding to use.

freq_shift (`float`, *optional*, defaults to 0.0) : Frequency shift for Fourier time embedding.

flip_sin_to_cos (`bool`, *optional*, defaults to `False`) : Whether to flip sin to cos for Fourier time embedding.

down_block_types (`tuple[str]`, *optional*, defaults to `("DownBlock1DNoSkip", "DownBlock1D", "AttnDownBlock1D")`) : tuple of downsample block types.

up_block_types (`tuple[str]`, *optional*, defaults to `("AttnUpBlock1D", "UpBlock1D", "UpBlock1DNoSkip")`) : tuple of upsample block types.

block_out_channels (`tuple[int]`, *optional*, defaults to `(32, 32, 64)`) : tuple of block output channels.

mid_block_type (`str`, *optional*, defaults to `"UNetMidBlock1D"`) : Block type for middle of UNet.

out_block_type (`str`, *optional*, defaults to `None`) : Optional output processing block of UNet.

act_fn (`str`, *optional*, defaults to `None`) : Optional activation function in UNet blocks.

norm_num_groups (`int`, *optional*, defaults to 8) : The number of groups for normalization.

layers_per_block (`int`, *optional*, defaults to 1) : The number of layers per block.

downsample_each_block (`int`, *optional*, defaults to `False`) : Experimental feature for using a UNet without upsampling.

**Returns:**

`[UNet1DOutput](/docs/diffusers/v0.39.0/en/api/models/unet#diffusers.models.unets.unet_1d.UNet1DOutput) or `tuple``

If `return_dict` is True, an [UNet1DOutput](/docs/diffusers/v0.39.0/en/api/models/unet#diffusers.models.unets.unet_1d.UNet1DOutput) is returned, otherwise a `tuple` is
returned where the first element is the sample tensor.

## UNet1DOutput[[diffusers.models.unets.unet_1d.UNet1DOutput]]
#### diffusers.models.unets.unet_1d.UNet1DOutput[[diffusers.models.unets.unet_1d.UNet1DOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/unets/unet_1d.py#L28)

The output of [UNet1DModel](/docs/diffusers/v0.39.0/en/api/models/unet#diffusers.UNet1DModel).

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, sample_size)`) : The hidden states output from the last layer of the model.
