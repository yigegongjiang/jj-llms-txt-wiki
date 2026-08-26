# UNet2DModel

The [UNet](https://huggingface.co/papers/1505.04597) model was originally introduced by Ronneberger et al. for biomedical image segmentation, but it is also commonly used in 🤗 Diffusers because it outputs images that are the same size as the input. It is one of the most important components of a diffusion system because it facilitates the actual diffusion process. There are several variants of the UNet model in 🤗 Diffusers, depending on it's number of dimensions and whether it is a conditional model or not. This is a 2D UNet model.

The abstract from the paper is:

*There is large consent that successful training of deep networks requires many thousand annotated training samples. In this paper, we present a network and training strategy that relies on the strong use of data augmentation to use the available annotated samples more efficiently. The architecture consists of a contracting path to capture context and a symmetric expanding path that enables precise localization. We show that such a network can be trained end-to-end from very few images and outperforms the prior best method (a sliding-window convolutional network) on the ISBI challenge for segmentation of neuronal structures in electron microscopic stacks. Using the same network trained on transmitted light microscopy images (phase contrast and DIC) we won the ISBI cell tracking challenge 2015 in these categories by a large margin. Moreover, the network is fast. Segmentation of a 512x512 image takes less than a second on a recent GPU. The full implementation (based on Caffe) and the trained networks are available at http://lmb.informatik.uni-freiburg.de/people/ronneber/u-net.*

## UNet2DModel[[diffusers.UNet2DModel]]

#### diffusers.UNet2DModel[[diffusers.UNet2DModel]]

```python
diffusers.UNet2DModel(sample_size: int | tuple[int, int] | None = None, in_channels: int = 3, out_channels: int = 3, center_input_sample: bool = False, time_embedding_type: str = 'positional', time_embedding_dim: int | None = None, freq_shift: int = 0, flip_sin_to_cos: bool = True, down_block_types: tuple = ('DownBlock2D', 'AttnDownBlock2D', 'AttnDownBlock2D', 'AttnDownBlock2D'), mid_block_type: str | None = 'UNetMidBlock2D', up_block_types: tuple = ('AttnUpBlock2D', 'AttnUpBlock2D', 'AttnUpBlock2D', 'UpBlock2D'), block_out_channels: tuple = (224, 448, 672, 896), layers_per_block: int = 2, mid_block_scale_factor: float = 1, downsample_padding: int = 1, downsample_type: str = 'conv', upsample_type: str = 'conv', dropout: float = 0.0, act_fn: str = 'silu', attention_head_dim: int | None = 8, norm_num_groups: int = 32, attn_norm_num_groups: int | None = None, norm_eps: float = 1e-05, resnet_time_scale_shift: str = 'default', add_attention: bool = True, class_embed_type: str | None = None, num_class_embeds: int | None = None, num_train_timesteps: int | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_2d.py#L39)

**Parameters:**

sample_size (`int` or `tuple[int, int]`, *optional*, defaults to `None`) : Height and width of input/output sample. Dimensions must be a multiple of `2 ** (len(block_out_channels) - 1)`.

in_channels (`int`, *optional*, defaults to 3) : Number of channels in the input sample.

out_channels (`int`, *optional*, defaults to 3) : Number of channels in the output.

center_input_sample (`bool`, *optional*, defaults to `False`) : Whether to center the input sample.

time_embedding_type (`str`, *optional*, defaults to `"positional"`) : Type of time embedding to use.

freq_shift (`int`, *optional*, defaults to 0) : Frequency shift for Fourier time embedding.

flip_sin_to_cos (`bool`, *optional*, defaults to `True`) : Whether to flip sin to cos for Fourier time embedding.

down_block_types (`tuple[str]`, *optional*, defaults to `("DownBlock2D", "AttnDownBlock2D", "AttnDownBlock2D", "AttnDownBlock2D")`) : tuple of downsample block types.

mid_block_type (`str`, *optional*, defaults to `"UNetMidBlock2D"`) : Block type for middle of UNet, it can be either `UNetMidBlock2D` or `None`.

up_block_types (`tuple[str]`, *optional*, defaults to `("AttnUpBlock2D", "AttnUpBlock2D", "AttnUpBlock2D", "UpBlock2D")`) : tuple of upsample block types.

block_out_channels (`tuple[int]`, *optional*, defaults to `(224, 448, 672, 896)`) : tuple of block output channels.

layers_per_block (`int`, *optional*, defaults to `2`) : The number of layers per block.

mid_block_scale_factor (`float`, *optional*, defaults to `1`) : The scale factor for the mid block.

downsample_padding (`int`, *optional*, defaults to `1`) : The padding for the downsample convolution.

downsample_type (`str`, *optional*, defaults to `conv`) : The downsample type for downsampling layers. Choose between "conv" and "resnet"

upsample_type (`str`, *optional*, defaults to `conv`) : The upsample type for upsampling layers. Choose between "conv" and "resnet"

dropout (`float`, *optional*, defaults to 0.0) : The dropout probability to use.

act_fn (`str`, *optional*, defaults to `"silu"`) : The activation function to use.

attention_head_dim (`int`, *optional*, defaults to `8`) : The attention head dimension.

norm_num_groups (`int`, *optional*, defaults to `32`) : The number of groups for normalization.

attn_norm_num_groups (`int`, *optional*, defaults to `None`) : If set to an integer, a group norm layer will be created in the mid block's `Attention` layer with the given number of groups. If left as `None`, the group norm layer will only be created if `resnet_time_scale_shift` is set to `default`, and if created will have `norm_num_groups` groups.

norm_eps (`float`, *optional*, defaults to `1e-5`) : The epsilon for normalization.

resnet_time_scale_shift (`str`, *optional*, defaults to `"default"`) : Time scale shift config for ResNet blocks (see `ResnetBlock2D`). Choose from `default` or `scale_shift`.

class_embed_type (`str`, *optional*, defaults to `None`) : The type of class embedding to use which is ultimately summed with the time embeddings. Choose from `None`, `"timestep"`, or `"identity"`.

num_class_embeds (`int`, *optional*, defaults to `None`) : Input dimension of the learnable embedding matrix to be projected to `time_embed_dim` when performing class conditioning with `class_embed_type` equal to `None`.

A 2D UNet model that takes a noisy sample and a timestep and returns a sample shaped output.

This model inherits from [ModelMixin](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for it's generic methods implemented
for all models (such as downloading or saving).

#### forward[[diffusers.UNet2DModel.forward]]

```python
forward(sample: Tensor, timestep: typing.Union[torch.Tensor, float, int], class_labels: typing.Optional[torch.Tensor] = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_2d.py#L249)

**Parameters:**

sample (`torch.Tensor`) : The noisy input tensor with the following shape `(batch, channel, height, width)`.

timestep (`torch.Tensor` or `float` or `int`) : The number of timesteps to denoise an input.

class_labels (`torch.Tensor`, *optional*, defaults to `None`) : Optional class labels for conditioning. Their embeddings will be summed with the timestep embeddings.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [UNet2DOutput](/docs/diffusers/v0.40.0/en/api/models/unet2d#diffusers.models.unets.unet_2d.UNet2DOutput) instead of a plain tuple.

**Returns:** [UNet2DOutput](/docs/diffusers/v0.40.0/en/api/models/unet2d#diffusers.models.unets.unet_2d.UNet2DOutput) or `tuple`

If `return_dict` is True, an [UNet2DOutput](/docs/diffusers/v0.40.0/en/api/models/unet2d#diffusers.models.unets.unet_2d.UNet2DOutput) is returned, otherwise a `tuple` is
returned where the first element is the sample tensor.

The [UNet2DModel](/docs/diffusers/v0.40.0/en/api/models/unet2d#diffusers.UNet2DModel) forward method.

## UNet2DOutput[[diffusers.models.unets.unet_2d.UNet2DOutput]]

#### diffusers.models.unets.unet_2d.UNet2DOutput[[diffusers.models.unets.unet_2d.UNet2DOutput]]

```python
diffusers.models.unets.unet_2d.UNet2DOutput(sample: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_2d.py#L27)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)`) : The hidden states output from the last layer of the model.

The output of [UNet2DModel](/docs/diffusers/v0.40.0/en/api/models/unet2d#diffusers.UNet2DModel).
