# UNet2DConditionModel

The [UNet](https://huggingface.co/papers/1505.04597) model was originally introduced by Ronneberger et al. for biomedical image segmentation, but it is also commonly used in 🤗 Diffusers because it outputs images that are the same size as the input. It is one of the most important components of a diffusion system because it facilitates the actual diffusion process. There are several variants of the UNet model in 🤗 Diffusers, depending on it's number of dimensions and whether it is a conditional model or not. This is a 2D UNet conditional model.

The abstract from the paper is:

*There is large consent that successful training of deep networks requires many thousand annotated training samples. In this paper, we present a network and training strategy that relies on the strong use of data augmentation to use the available annotated samples more efficiently. The architecture consists of a contracting path to capture context and a symmetric expanding path that enables precise localization. We show that such a network can be trained end-to-end from very few images and outperforms the prior best method (a sliding-window convolutional network) on the ISBI challenge for segmentation of neuronal structures in electron microscopic stacks. Using the same network trained on transmitted light microscopy images (phase contrast and DIC) we won the ISBI cell tracking challenge 2015 in these categories by a large margin. Moreover, the network is fast. Segmentation of a 512x512 image takes less than a second on a recent GPU. The full implementation (based on Caffe) and the trained networks are available at http://lmb.informatik.uni-freiburg.de/people/ronneber/u-net.*

## UNet2DConditionModel[[diffusers.UNet2DConditionModel]]
#### diffusers.UNet2DConditionModel[[diffusers.UNet2DConditionModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/unets/unet_2d_condition.py#L76)

A conditional 2D UNet model that takes a noisy sample, conditional state, and a timestep and returns a sample
shaped output.

This model inherits from [ModelMixin](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for it's generic methods implemented
for all models (such as downloading or saving).

disable_freeudiffusers.UNet2DConditionModel.disable_freeuhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/unets/unet_2d_condition.py#L814[]
Disables the FreeU mechanism.

**Parameters:**

sample_size (`int` or `tuple[int, int]`, *optional*, defaults to `None`) : Height and width of input/output sample.

in_channels (`int`, *optional*, defaults to 4) : Number of channels in the input sample.

out_channels (`int`, *optional*, defaults to 4) : Number of channels in the output.

center_input_sample (`bool`, *optional*, defaults to `False`) : Whether to center the input sample.

flip_sin_to_cos (`bool`, *optional*, defaults to `True`) : Whether to flip the sin to cos in the time embedding.

freq_shift (`int`, *optional*, defaults to 0) : The frequency shift to apply to the time embedding.

down_block_types (`tuple[str]`, *optional*, defaults to `("CrossAttnDownBlock2D", "CrossAttnDownBlock2D", "CrossAttnDownBlock2D", "DownBlock2D")`) : The tuple of downsample blocks to use.

mid_block_type (`str`, *optional*, defaults to `"UNetMidBlock2DCrossAttn"`) : Block type for middle of UNet, it can be one of `UNetMidBlock2DCrossAttn`, `UNetMidBlock2D`, or `UNetMidBlock2DSimpleCrossAttn`. If `None`, the mid block layer is skipped.

up_block_types (`tuple[str]`, *optional*, defaults to `("UpBlock2D", "CrossAttnUpBlock2D", "CrossAttnUpBlock2D", "CrossAttnUpBlock2D")`) : The tuple of upsample blocks to use.

only_cross_attention(`bool` or `tuple[bool]`, *optional*, default to `False`) : Whether to include self-attention in the basic transformer blocks, see `BasicTransformerBlock`.

block_out_channels (`tuple[int]`, *optional*, defaults to `(320, 640, 1280, 1280)`) : The tuple of output channels for each block.

layers_per_block (`int`, *optional*, defaults to 2) : The number of layers per block.

downsample_padding (`int`, *optional*, defaults to 1) : The padding to use for the downsampling convolution.

mid_block_scale_factor (`float`, *optional*, defaults to 1.0) : The scale factor to use for the mid block.

dropout (`float`, *optional*, defaults to 0.0) : The dropout probability to use.

act_fn (`str`, *optional*, defaults to `"silu"`) : The activation function to use.

norm_num_groups (`int`, *optional*, defaults to 32) : The number of groups to use for the normalization. If `None`, normalization and activation layers is skipped in post-processing.

norm_eps (`float`, *optional*, defaults to 1e-5) : The epsilon to use for the normalization.

cross_attention_dim (`int` or `tuple[int]`, *optional*, defaults to 1280) : The dimension of the cross attention features.

transformer_layers_per_block (`int`, `tuple[int]`, or `tuple[tuple]` , *optional*, defaults to 1) : The number of transformer blocks of type `BasicTransformerBlock`. Only relevant for `CrossAttnDownBlock2D`, `CrossAttnUpBlock2D`, `UNetMidBlock2DCrossAttn`.

reverse_transformer_layers_per_block : (`tuple[tuple]`, *optional*, defaults to None): The number of transformer blocks of type `BasicTransformerBlock`, in the upsampling blocks of the U-Net. Only relevant if `transformer_layers_per_block` is of type `tuple[tuple]` and for `CrossAttnDownBlock2D`, `CrossAttnUpBlock2D`, `UNetMidBlock2DCrossAttn`.

encoder_hid_dim (`int`, *optional*, defaults to None) : If `encoder_hid_dim_type` is defined, `encoder_hidden_states` will be projected from `encoder_hid_dim` dimension to `cross_attention_dim`.

encoder_hid_dim_type (`str`, *optional*, defaults to `None`) : If given, the `encoder_hidden_states` and potentially other embeddings are down-projected to text embeddings of dimension `cross_attention` according to `encoder_hid_dim_type`.

attention_head_dim (`int`, *optional*, defaults to 8) : The dimension of the attention heads.

num_attention_heads (`int`, *optional*) : The number of attention heads. If not defined, defaults to `attention_head_dim`

resnet_time_scale_shift (`str`, *optional*, defaults to `"default"`) : Time scale shift config for ResNet blocks (see `ResnetBlock2D`). Choose from `default` or `scale_shift`.

class_embed_type (`str`, *optional*, defaults to `None`) : The type of class embedding to use which is ultimately summed with the time embeddings. Choose from `None`, `"timestep"`, `"identity"`, `"projection"`, or `"simple_projection"`.

addition_embed_type (`str`, *optional*, defaults to `None`) : Configures an optional embedding which will be summed with the time embeddings. Choose from `None` or "text". "text" will use the `TextTimeEmbedding` layer.

addition_time_embed_dim : (`int`, *optional*, defaults to `None`): Dimension for the timestep embeddings.

num_class_embeds (`int`, *optional*, defaults to `None`) : Input dimension of the learnable embedding matrix to be projected to `time_embed_dim`, when performing class conditioning with `class_embed_type` equal to `None`.

time_embedding_type (`str`, *optional*, defaults to `positional`) : The type of position embedding to use for timesteps. Choose from `positional` or `fourier`.

time_embedding_dim (`int`, *optional*, defaults to `None`) : An optional override for the dimension of the projected time embedding.

time_embedding_act_fn (`str`, *optional*, defaults to `None`) : Optional activation function to use only once on the time embeddings before they are passed to the rest of the UNet. Choose from `silu`, `mish`, `gelu`, and `swish`.

timestep_post_act (`str`, *optional*, defaults to `None`) : The second activation function to use in timestep embedding. Choose from `silu`, `mish` and `gelu`.

time_cond_proj_dim (`int`, *optional*, defaults to `None`) : The dimension of `cond_proj` layer in the timestep embedding.

conv_in_kernel (`int`, *optional*, default to `3`) : The kernel size of `conv_in` layer.

conv_out_kernel (`int`, *optional*, default to `3`) : The kernel size of `conv_out` layer.

projection_class_embeddings_input_dim (`int`, *optional*) : The dimension of the `class_labels` input when `class_embed_type="projection"`. Required when `class_embed_type="projection"`.

class_embeddings_concat (`bool`, *optional*, defaults to `False`) : Whether to concatenate the time embeddings with the class embeddings.

mid_block_only_cross_attention (`bool`, *optional*, defaults to `None`) : Whether to use cross attention with the mid block when using the `UNetMidBlock2DSimpleCrossAttn`. If `only_cross_attention` is given as a single boolean and `mid_block_only_cross_attention` is `None`, the `only_cross_attention` value is used as the value for `mid_block_only_cross_attention`. Default to `False` otherwise.
#### enable_freeu[[diffusers.UNet2DConditionModel.enable_freeu]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/unets/unet_2d_condition.py#L790)

Enables the FreeU mechanism from https://huggingface.co/papers/2309.11497.

The suffixes after the scaling factors represent the stage blocks where they are being applied.

Please refer to the [official repository](https://github.com/ChenyangSi/FreeU) for combinations of values that
are known to work well for different pipelines such as Stable Diffusion v1, v2, and Stable Diffusion XL.

**Parameters:**

s1 (`float`) : Scaling factor for stage 1 to attenuate the contributions of the skip features. This is done to mitigate the "oversmoothing effect" in the enhanced denoising process.

s2 (`float`) : Scaling factor for stage 2 to attenuate the contributions of the skip features. This is done to mitigate the "oversmoothing effect" in the enhanced denoising process.

b1 (`float`) : Scaling factor for stage 1 to amplify the contributions of backbone features.

b2 (`float`) : Scaling factor for stage 2 to amplify the contributions of backbone features.
#### forward[[diffusers.UNet2DConditionModel.forward]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/unets/unet_2d_condition.py#L978)

The [UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel) forward method.

**Parameters:**

sample (`torch.Tensor`) : The noisy input tensor with the following shape `(batch, channel, height, width)`.

timestep (`torch.Tensor` or `float` or `int`) : The number of timesteps to denoise an input.

encoder_hidden_states (`torch.Tensor`) : The encoder hidden states with shape `(batch, sequence_length, feature_dim)`.

class_labels (`torch.Tensor`, *optional*, defaults to `None`) : Optional class labels for conditioning. Their embeddings will be summed with the timestep embeddings.

timestep_cond : (`torch.Tensor`, *optional*, defaults to `None`): Conditional embeddings for timestep. If provided, the embeddings will be summed with the samples passed through the `self.time_embedding` layer to obtain the timestep embeddings.

attention_mask (`torch.Tensor`, *optional*, defaults to `None`) : An attention mask of shape `(batch, key_tokens)` is applied to `encoder_hidden_states`. If `1` the mask is kept, otherwise if `0` it is discarded. Mask will be converted into a bias, which adds large negative values to the attention scores corresponding to "discard" tokens.

cross_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

added_cond_kwargs : (`dict`, *optional*): A kwargs dictionary containing additional embeddings that if specified are added to the embeddings that are passed along to the UNet blocks.

down_block_additional_residuals : (`tuple` of `torch.Tensor`, *optional*): A tuple of tensors that if specified are added to the residuals of down unet blocks.

mid_block_additional_residual : (`torch.Tensor`, *optional*): A tensor that if specified is added to the residual of the middle unet block.

down_intrablock_additional_residuals (`tuple` of `torch.Tensor`, *optional*) : additional residuals to be added within UNet down blocks, for example from T2I-Adapter side model(s)

encoder_attention_mask (`torch.Tensor`) : A cross-attention mask of shape `(batch, sequence_length)` is applied to `encoder_hidden_states`. If `True` the mask is kept, otherwise if `False` it is discarded. Mask will be converted into a bias, which adds large negative values to the attention scores corresponding to "discard" tokens.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [UNet2DConditionOutput](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.models.unets.unet_2d_condition.UNet2DConditionOutput) instead of a plain tuple.

**Returns:**

`[UNet2DConditionOutput](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.models.unets.unet_2d_condition.UNet2DConditionOutput) or `tuple``

If `return_dict` is True, an [UNet2DConditionOutput](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.models.unets.unet_2d_condition.UNet2DConditionOutput) is returned,
otherwise a `tuple` is returned where the first element is the sample tensor.
#### fuse_qkv_projections[[diffusers.UNet2DConditionModel.fuse_qkv_projections]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/unets/unet_2d_condition.py#L822)

Enables fused QKV projections. For self-attention modules, all projection matrices (i.e., query, key, value)
are fused. For cross-attention modules, key and value projection matrices are fused.

> [!WARNING] > This API is 🧪 experimental.
#### set_attention_slice[[diffusers.UNet2DConditionModel.set_attention_slice]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/unets/unet_2d_condition.py#L725)

Enable sliced attention computation.

When this option is enabled, the attention module splits the input tensor in slices to compute attention in
several steps. This is useful for saving some memory in exchange for a small decrease in speed.

**Parameters:**

slice_size (`str` or `int` or `list(int)`, *optional*, defaults to `"auto"`) : When `"auto"`, input to the attention heads is halved, so attention is computed in two steps. If `"max"`, maximum amount of memory is saved by running only one slice at a time. If a number is provided, uses as many slices as `attention_head_dim // slice_size`. In this case, `attention_head_dim` must be a multiple of `slice_size`.
#### set_default_attn_processor[[diffusers.UNet2DConditionModel.set_default_attn_processor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/unets/unet_2d_condition.py#L710)

Disables custom attention processors and sets the default attention implementation.
#### unfuse_qkv_projections[[diffusers.UNet2DConditionModel.unfuse_qkv_projections]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/unets/unet_2d_condition.py#L843)

Disables the fused QKV projection if enabled.

> [!WARNING] > This API is 🧪 experimental.

## UNet2DConditionOutput[[diffusers.models.unets.unet_2d_condition.UNet2DConditionOutput]]
#### diffusers.models.unets.unet_2d_condition.UNet2DConditionOutput[[diffusers.models.unets.unet_2d_condition.UNet2DConditionOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/unets/unet_2d_condition.py#L64)

The output of [UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel).

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)`) : The hidden states output conditioned on `encoder_hidden_states` input. Output of last layer of model.
