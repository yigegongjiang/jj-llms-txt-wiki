# UNet3DConditionModel

The [UNet](https://huggingface.co/papers/1505.04597) model was originally introduced by Ronneberger et al. for biomedical image segmentation, but it is also commonly used in 🤗 Diffusers because it outputs images that are the same size as the input. It is one of the most important components of a diffusion system because it facilitates the actual diffusion process. There are several variants of the UNet model in 🤗 Diffusers, depending on it's number of dimensions and whether it is a conditional model or not. This is a 3D UNet conditional model.

The abstract from the paper is:

*There is large consent that successful training of deep networks requires many thousand annotated training samples. In this paper, we present a network and training strategy that relies on the strong use of data augmentation to use the available annotated samples more efficiently. The architecture consists of a contracting path to capture context and a symmetric expanding path that enables precise localization. We show that such a network can be trained end-to-end from very few images and outperforms the prior best method (a sliding-window convolutional network) on the ISBI challenge for segmentation of neuronal structures in electron microscopic stacks. Using the same network trained on transmitted light microscopy images (phase contrast and DIC) we won the ISBI cell tracking challenge 2015 in these categories by a large margin. Moreover, the network is fast. Segmentation of a 512x512 image takes less than a second on a recent GPU. The full implementation (based on Caffe) and the trained networks are available at http://lmb.informatik.uni-freiburg.de/people/ronneber/u-net.*

## UNet3DConditionModel[[diffusers.UNet3DConditionModel]]

#### diffusers.UNet3DConditionModel[[diffusers.UNet3DConditionModel]]

```python
diffusers.UNet3DConditionModel(sample_size: int | None = None, in_channels: int = 4, out_channels: int = 4, down_block_types: tuple = ('CrossAttnDownBlock3D', 'CrossAttnDownBlock3D', 'CrossAttnDownBlock3D', 'DownBlock3D'), up_block_types: tuple = ('UpBlock3D', 'CrossAttnUpBlock3D', 'CrossAttnUpBlock3D', 'CrossAttnUpBlock3D'), block_out_channels: tuple = (320, 640, 1280, 1280), layers_per_block: int = 2, downsample_padding: int = 1, mid_block_scale_factor: float = 1, act_fn: str = 'silu', norm_num_groups: int | None = 32, norm_eps: float = 1e-05, cross_attention_dim: int = 1024, attention_head_dim: int | tuple[int] = 64, num_attention_heads: int | tuple[int] | None = None, time_cond_proj_dim: int | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_3d_condition.py#L62)

**Parameters:**

sample_size (`int` or `tuple[int, int]`, *optional*, defaults to `None`) : Height and width of input/output sample.

in_channels (`int`, *optional*, defaults to 4) : The number of channels in the input sample.

out_channels (`int`, *optional*, defaults to 4) : The number of channels in the output.

down_block_types (`tuple[str]`, *optional*, defaults to `("CrossAttnDownBlock3D", "CrossAttnDownBlock3D", "CrossAttnDownBlock3D", "DownBlock3D")`) : The tuple of downsample blocks to use.

up_block_types (`tuple[str]`, *optional*, defaults to `("UpBlock3D", "CrossAttnUpBlock3D", "CrossAttnUpBlock3D", "CrossAttnUpBlock3D")`) : The tuple of upsample blocks to use.

block_out_channels (`tuple[int]`, *optional*, defaults to `(320, 640, 1280, 1280)`) : The tuple of output channels for each block.

layers_per_block (`int`, *optional*, defaults to 2) : The number of layers per block.

downsample_padding (`int`, *optional*, defaults to 1) : The padding to use for the downsampling convolution.

mid_block_scale_factor (`float`, *optional*, defaults to 1.0) : The scale factor to use for the mid block.

act_fn (`str`, *optional*, defaults to `"silu"`) : The activation function to use.

norm_num_groups (`int`, *optional*, defaults to 32) : The number of groups to use for the normalization. If `None`, normalization and activation layers is skipped in post-processing.

norm_eps (`float`, *optional*, defaults to 1e-5) : The epsilon to use for the normalization.

cross_attention_dim (`int`, *optional*, defaults to 1024) : The dimension of the cross attention features.

attention_head_dim (`int`, *optional*, defaults to 64) : The dimension of the attention heads.

num_attention_heads (`int`, *optional*) : The number of attention heads.

time_cond_proj_dim (`int`, *optional*, defaults to `None`) : The dimension of `cond_proj` layer in the timestep embedding.

A conditional 3D UNet model that takes a noisy sample, conditional state, and a timestep and returns a sample
shaped output.

This model inherits from [ModelMixin](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for it's generic methods implemented
for all models (such as downloading or saving).

#### disable_freeu[[diffusers.UNet3DConditionModel.disable_freeu]]

```python
disable_freeu()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_3d_condition.py#L437)

Disables the FreeU mechanism.

#### enable_forward_chunking[[diffusers.UNet3DConditionModel.enable_forward_chunking]]

```python
enable_forward_chunking(chunk_size: int | None = None, dim: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_3d_condition.py#L355)

**Parameters:**

chunk_size (`int`, *optional*) : The chunk size of the feed-forward layers. If not specified, will run feed-forward layer individually over each tensor of dim=`dim`.

dim (`int`, *optional*, defaults to `0`) : The dimension over which the feed-forward computation should be chunked. Choose between dim=0 (batch) or dim=1 (sequence length).

Sets the attention processor to use [feed forward
chunking](https://huggingface.co/blog/reformer#2-chunked-feed-forward-layers).

#### enable_freeu[[diffusers.UNet3DConditionModel.enable_freeu]]

```python
enable_freeu(s1, s2, b1, b2)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_3d_condition.py#L412)

**Parameters:**

s1 (`float`) : Scaling factor for stage 1 to attenuate the contributions of the skip features. This is done to mitigate the "oversmoothing effect" in the enhanced denoising process.

s2 (`float`) : Scaling factor for stage 2 to attenuate the contributions of the skip features. This is done to mitigate the "oversmoothing effect" in the enhanced denoising process.

b1 (`float`) : Scaling factor for stage 1 to amplify the contributions of backbone features.

b2 (`float`) : Scaling factor for stage 2 to amplify the contributions of backbone features.

Enables the FreeU mechanism from https://huggingface.co/papers/2309.11497.

The suffixes after the scaling factors represent the stage blocks where they are being applied.

Please refer to the [official repository](https://github.com/ChenyangSi/FreeU) for combinations of values that
are known to work well for different pipelines such as Stable Diffusion v1, v2, and Stable Diffusion XL.

#### forward[[diffusers.UNet3DConditionModel.forward]]

```python
forward(sample: Tensor, timestep: typing.Union[torch.Tensor, float, int], encoder_hidden_states: Tensor, class_labels: typing.Optional[torch.Tensor] = None, timestep_cond: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, cross_attention_kwargs: dict[str, typing.Any] | None = None, down_block_additional_residuals: tuple[torch.Tensor] | None = None, mid_block_additional_residual: typing.Optional[torch.Tensor] = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_3d_condition.py#L477)

**Parameters:**

sample (`torch.Tensor`) : The noisy input tensor with the following shape `(batch, num_channels, num_frames, height, width`.

timestep (`torch.Tensor` or `float` or `int`) : The number of timesteps to denoise an input.

encoder_hidden_states (`torch.Tensor`) : The encoder hidden states with shape `(batch, sequence_length, feature_dim)`.

class_labels (`torch.Tensor`, *optional*, defaults to `None`) : Optional class labels for conditioning. Their embeddings will be summed with the timestep embeddings.

timestep_cond : (`torch.Tensor`, *optional*, defaults to `None`): Conditional embeddings for timestep. If provided, the embeddings will be summed with the samples passed through the `self.time_embedding` layer to obtain the timestep embeddings.

attention_mask (`torch.Tensor`, *optional*, defaults to `None`) : An attention mask of shape `(batch, key_tokens)` is applied to `encoder_hidden_states`. If `1` the mask is kept, otherwise if `0` it is discarded. Mask will be converted into a bias, which adds large negative values to the attention scores corresponding to "discard" tokens.

cross_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

down_block_additional_residuals : (`tuple` of `torch.Tensor`, *optional*): A tuple of tensors that if specified are added to the residuals of down unet blocks.

mid_block_additional_residual : (`torch.Tensor`, *optional*): A tensor that if specified is added to the residual of the middle unet block.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [UNet3DConditionOutput](/docs/diffusers/v0.40.0/en/api/models/unet3d-cond#diffusers.models.unets.unet_3d_condition.UNet3DConditionOutput) instead of a plain tuple.

cross_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttnProcessor`.

**Returns:** [UNet3DConditionOutput](/docs/diffusers/v0.40.0/en/api/models/unet3d-cond#diffusers.models.unets.unet_3d_condition.UNet3DConditionOutput) or `tuple`

If `return_dict` is True, an [UNet3DConditionOutput](/docs/diffusers/v0.40.0/en/api/models/unet3d-cond#diffusers.models.unets.unet_3d_condition.UNet3DConditionOutput) is returned,
otherwise a `tuple` is returned where the first element is the sample tensor.

The [UNet3DConditionModel](/docs/diffusers/v0.40.0/en/api/models/unet3d-cond#diffusers.UNet3DConditionModel) forward method.

#### fuse_qkv_projections[[diffusers.UNet3DConditionModel.fuse_qkv_projections]]

```python
fuse_qkv_projections()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_3d_condition.py#L446)

Enables fused QKV projections. For self-attention modules, all projection matrices (i.e., query, key, value)
are fused. For cross-attention modules, key and value projection matrices are fused.

> [!WARNING] > This API is 🧪 experimental.

#### set_attention_slice[[diffusers.UNet3DConditionModel.set_attention_slice]]

```python
set_attention_slice(slice_size: str | int | list[int])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_3d_condition.py#L290)

**Parameters:**

slice_size (`str` or `int` or `list(int)`, *optional*, defaults to `"auto"`) : When `"auto"`, input to the attention heads is halved, so attention is computed in two steps. If `"max"`, maximum amount of memory is saved by running only one slice at a time. If a number is provided, uses as many slices as `attention_head_dim // slice_size`. In this case, `attention_head_dim` must be a multiple of `slice_size`.

Enable sliced attention computation.

When this option is enabled, the attention module splits the input tensor in slices to compute attention in
several steps. This is useful for saving some memory in exchange for a small decrease in speed.

#### set_default_attn_processor[[diffusers.UNet3DConditionModel.set_default_attn_processor]]

```python
set_default_attn_processor()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_3d_condition.py#L396)

Disables custom attention processors and sets the default attention implementation.

#### unfuse_qkv_projections[[diffusers.UNet3DConditionModel.unfuse_qkv_projections]]

```python
unfuse_qkv_projections()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_3d_condition.py#L468)

Disables the fused QKV projection if enabled.

> [!WARNING] > This API is 🧪 experimental.

## UNet3DConditionOutput[[diffusers.models.unets.unet_3d_condition.UNet3DConditionOutput]]

#### diffusers.models.unets.unet_3d_condition.UNet3DConditionOutput[[diffusers.models.unets.unet_3d_condition.UNet3DConditionOutput]]

```python
diffusers.models.unets.unet_3d_condition.UNet3DConditionOutput(sample: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_3d_condition.py#L50)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, num_frames, height, width)`) : The hidden states output conditioned on `encoder_hidden_states` input. Output of last layer of model.

The output of [UNet3DConditionModel](/docs/diffusers/v0.40.0/en/api/models/unet3d-cond#diffusers.UNet3DConditionModel).
