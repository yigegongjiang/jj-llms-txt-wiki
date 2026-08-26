# UNetMotionModel

The [UNet](https://huggingface.co/papers/1505.04597) model was originally introduced by Ronneberger et al for biomedical image segmentation, but it is also commonly used in 🤗 Diffusers because it outputs images that are the same size as the input. It is one of the most important components of a diffusion system because it facilitates the actual diffusion process. There are several variants of the UNet model in 🤗 Diffusers, depending on it's number of dimensions and whether it is a conditional model or not. This is a 2D UNet model.

The abstract from the paper is:

*There is large consent that successful training of deep networks requires many thousand annotated training samples. In this paper, we present a network and training strategy that relies on the strong use of data augmentation to use the available annotated samples more efficiently. The architecture consists of a contracting path to capture context and a symmetric expanding path that enables precise localization. We show that such a network can be trained end-to-end from very few images and outperforms the prior best method (a sliding-window convolutional network) on the ISBI challenge for segmentation of neuronal structures in electron microscopic stacks. Using the same network trained on transmitted light microscopy images (phase contrast and DIC) we won the ISBI cell tracking challenge 2015 in these categories by a large margin. Moreover, the network is fast. Segmentation of a 512x512 image takes less than a second on a recent GPU. The full implementation (based on Caffe) and the trained networks are available at http://lmb.informatik.uni-freiburg.de/people/ronneber/u-net.*

## UNetMotionModel[[diffusers.UNetMotionModel]]

#### diffusers.UNetMotionModel[[diffusers.UNetMotionModel]]

```python
diffusers.UNetMotionModel(sample_size: int | None = None, in_channels: int = 4, out_channels: int = 4, down_block_types: tuple = ('CrossAttnDownBlockMotion', 'CrossAttnDownBlockMotion', 'CrossAttnDownBlockMotion', 'DownBlockMotion'), up_block_types: tuple = ('UpBlockMotion', 'CrossAttnUpBlockMotion', 'CrossAttnUpBlockMotion', 'CrossAttnUpBlockMotion'), block_out_channels: tuple = (320, 640, 1280, 1280), layers_per_block: int | tuple[int] = 2, downsample_padding: int = 1, mid_block_scale_factor: float = 1, act_fn: str = 'silu', norm_num_groups: int = 32, norm_eps: float = 1e-05, cross_attention_dim: int = 1280, transformer_layers_per_block: int | tuple[int] | tuple[tuple] = 1, reverse_transformer_layers_per_block: int | tuple[int] | tuple[tuple] | None = None, temporal_transformer_layers_per_block: int | tuple[int] | tuple[tuple] = 1, reverse_temporal_transformer_layers_per_block: int | tuple[int] | tuple[tuple] | None = None, transformer_layers_per_mid_block: int | tuple[int] | None = None, temporal_transformer_layers_per_mid_block: int | tuple[int] | None = 1, use_linear_projection: bool = False, num_attention_heads: int | tuple[int, ...] = 8, motion_max_seq_length: int = 32, motion_num_attention_heads: int | tuple[int, ...] = 8, reverse_motion_num_attention_heads: int | tuple[int, ...] | tuple[tuple[int, ...], ...] | None = None, use_motion_mid_block: bool = True, mid_block_layers: int = 1, encoder_hid_dim: int | None = None, encoder_hid_dim_type: str | None = None, addition_embed_type: str | None = None, addition_time_embed_dim: int | None = None, projection_class_embeddings_input_dim: int | None = None, time_cond_proj_dim: int | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_motion_model.py#L1201)

A modified conditional 2D UNet model that takes a noisy sample, conditional state, and a timestep and returns a
sample shaped output.

This model inherits from [ModelMixin](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for it's generic methods implemented
for all models (such as downloading or saving).

#### disable_freeu[[diffusers.UNetMotionModel.disable_freeu]]

```python
disable_freeu()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_motion_model.py#L1842)

Disables the FreeU mechanism.

#### enable_forward_chunking[[diffusers.UNetMotionModel.enable_forward_chunking]]

```python
enable_forward_chunking(chunk_size: int | None = None, dim: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_motion_model.py#L1760)

**Parameters:**

chunk_size (`int`, *optional*) : The chunk size of the feed-forward layers. If not specified, will run feed-forward layer individually over each tensor of dim=`dim`.

dim (`int`, *optional*, defaults to `0`) : The dimension over which the feed-forward computation should be chunked. Choose between dim=0 (batch) or dim=1 (sequence length).

Sets the attention processor to use [feed forward
chunking](https://huggingface.co/blog/reformer#2-chunked-feed-forward-layers).

#### enable_freeu[[diffusers.UNetMotionModel.enable_freeu]]

```python
enable_freeu(s1: float, s2: float, b1: float, b2: float)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_motion_model.py#L1817)

**Parameters:**

s1 (`float`) : Scaling factor for stage 1 to attenuate the contributions of the skip features. This is done to mitigate the "oversmoothing effect" in the enhanced denoising process.

s2 (`float`) : Scaling factor for stage 2 to attenuate the contributions of the skip features. This is done to mitigate the "oversmoothing effect" in the enhanced denoising process.

b1 (`float`) : Scaling factor for stage 1 to amplify the contributions of backbone features.

b2 (`float`) : Scaling factor for stage 2 to amplify the contributions of backbone features.

Enables the FreeU mechanism from https://huggingface.co/papers/2309.11497.

The suffixes after the scaling factors represent the stage blocks where they are being applied.

Please refer to the [official repository](https://github.com/ChenyangSi/FreeU) for combinations of values that
are known to work well for different pipelines such as Stable Diffusion v1, v2, and Stable Diffusion XL.

#### forward[[diffusers.UNetMotionModel.forward]]

```python
forward(sample: Tensor, timestep: typing.Union[torch.Tensor, float, int], encoder_hidden_states: Tensor, timestep_cond: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, cross_attention_kwargs: dict[str, typing.Any] | None = None, added_cond_kwargs: dict[str, torch.Tensor] | None = None, down_block_additional_residuals: tuple[torch.Tensor] | None = None, mid_block_additional_residual: typing.Optional[torch.Tensor] = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_motion_model.py#L1882)

**Parameters:**

sample (`torch.Tensor`) : The noisy input tensor with the following shape `(batch, num_frames, channel, height, width`.

timestep (`torch.Tensor` or `float` or `int`) : The number of timesteps to denoise an input.

encoder_hidden_states (`torch.Tensor`) : The encoder hidden states with shape `(batch, sequence_length, feature_dim)`.

timestep_cond : (`torch.Tensor`, *optional*, defaults to `None`): Conditional embeddings for timestep. If provided, the embeddings will be summed with the samples passed through the `self.time_embedding` layer to obtain the timestep embeddings.

attention_mask (`torch.Tensor`, *optional*, defaults to `None`) : An attention mask of shape `(batch, key_tokens)` is applied to `encoder_hidden_states`. If `1` the mask is kept, otherwise if `0` it is discarded. Mask will be converted into a bias, which adds large negative values to the attention scores corresponding to "discard" tokens.

cross_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

added_cond_kwargs (`dict`, *optional*) : A dictionary of additional embeddings (e.g. text and time embeddings) used to condition the model.

down_block_additional_residuals : (`tuple` of `torch.Tensor`, *optional*): A tuple of tensors that if specified are added to the residuals of down unet blocks.

mid_block_additional_residual : (`torch.Tensor`, *optional*): A tensor that if specified is added to the residual of the middle unet block.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `UNetMotionOutput` instead of a plain tuple.

**Returns:** `UNetMotionOutput` or `tuple`

If `return_dict` is True, an `UNetMotionOutput` is returned,
otherwise a `tuple` is returned where the first element is the sample tensor.

The [UNetMotionModel](/docs/diffusers/v0.40.0/en/api/models/unet-motion#diffusers.UNetMotionModel) forward method.

#### freeze_unet2d_params[[diffusers.UNetMotionModel.freeze_unet2d_params]]

```python
freeze_unet2d_params()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_motion_model.py#L1691)

Freeze the weights of just the UNet2DConditionModel, and leave the motion modules
unfrozen for fine tuning.

#### fuse_qkv_projections[[diffusers.UNetMotionModel.fuse_qkv_projections]]

```python
fuse_qkv_projections()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_motion_model.py#L1851)

Enables fused QKV projections. For self-attention modules, all projection matrices (i.e., query, key, value)
are fused. For cross-attention modules, key and value projection matrices are fused.

> [!WARNING] > This API is 🧪 experimental.

#### set_default_attn_processor[[diffusers.UNetMotionModel.set_default_attn_processor]]

```python
set_default_attn_processor()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_motion_model.py#L1801)

Disables custom attention processors and sets the default attention implementation.

#### unfuse_qkv_projections[[diffusers.UNetMotionModel.unfuse_qkv_projections]]

```python
unfuse_qkv_projections()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_motion_model.py#L1873)

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
