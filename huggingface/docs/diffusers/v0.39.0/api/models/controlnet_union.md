# ControlNetUnionModel

ControlNetUnionModel is an implementation of ControlNet for Stable Diffusion XL.

The ControlNet model was introduced in [ControlNetPlus](https://github.com/xinsir6/ControlNetPlus) by xinsir6. It supports multiple conditioning inputs without increasing computation.

*We design a new architecture that can support 10+ control types in condition text-to-image generation and can generate high resolution images visually comparable with midjourney. The network is based on the original ControlNet architecture, we propose two new modules to: 1 Extend the original ControlNet to support different image conditions using the same network parameter. 2 Support multiple conditions input without increasing computation offload, which is especially important for designers who want to edit image in detail, different conditions use the same condition encoder, without adding extra computations or parameters.*

## Loading

By default the [ControlNetUnionModel](/docs/diffusers/v0.39.0/en/api/models/controlnet_union#diffusers.ControlNetUnionModel) should be loaded with [from_pretrained()](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin.from_pretrained).

```py
from diffusers import StableDiffusionXLControlNetUnionPipeline, ControlNetUnionModel

controlnet = ControlNetUnionModel.from_pretrained("xinsir/controlnet-union-sdxl-1.0")
pipe = StableDiffusionXLControlNetUnionPipeline.from_pretrained("stabilityai/stable-diffusion-xl-base-1.0", controlnet=controlnet)
```

## ControlNetUnionModel[[diffusers.ControlNetUnionModel]]

#### diffusers.ControlNetUnionModel[[diffusers.ControlNetUnionModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet_union.py#L85)

A ControlNetUnion model.

forwarddiffusers.ControlNetUnionModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet_union.py#L541[{"name": "sample", "val": ": Tensor"}, {"name": "timestep", "val": ": torch.Tensor | float | int"}, {"name": "encoder_hidden_states", "val": ": Tensor"}, {"name": "controlnet_cond", "val": ": list"}, {"name": "control_type", "val": ": Tensor"}, {"name": "control_type_idx", "val": ": list"}, {"name": "conditioning_scale", "val": ": float | list[float] = 1.0"}, {"name": "class_labels", "val": ": torch.Tensor | None = None"}, {"name": "timestep_cond", "val": ": torch.Tensor | None = None"}, {"name": "attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "added_cond_kwargs", "val": ": dict[str, torch.Tensor] | None = None"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "from_multi", "val": ": bool = False"}, {"name": "guess_mode", "val": ": bool = False"}, {"name": "return_dict", "val": ": bool = True"}]- **sample** (`torch.Tensor`) --
  The noisy input tensor.
- **timestep** (`torch.Tensor | float | int`) --
  The number of timesteps to denoise an input.
- **encoder_hidden_states** (`torch.Tensor`) --
  The encoder hidden states.
- **controlnet_cond** (`list[torch.Tensor]`) --
  The conditional input tensors.
- **control_type** (`torch.Tensor`) --
  A tensor of shape `(batch, num_control_type)` with values `0` or `1` depending on whether the control
  type is used.
- **control_type_idx** (`list[int]`) --
  The indices of `control_type`.
- **conditioning_scale** (`float`, defaults to `1.0`) --
  The scale factor for ControlNet outputs.
- **class_labels** (`torch.Tensor`, *optional*, defaults to `None`) --
  Optional class labels for conditioning. Their embeddings will be summed with the timestep embeddings.
- **timestep_cond** (`torch.Tensor`, *optional*, defaults to `None`) --
  Additional conditional embeddings for timestep. If provided, the embeddings will be summed with the
  timestep_embedding passed through the `self.time_embedding` layer to obtain the final timestep
  embeddings.
- **attention_mask** (`torch.Tensor`, *optional*, defaults to `None`) --
  An attention mask of shape `(batch, key_tokens)` is applied to `encoder_hidden_states`. If `1` the mask
  is kept, otherwise if `0` it is discarded. Mask will be converted into a bias, which adds large
  negative values to the attention scores corresponding to "discard" tokens.
- **added_cond_kwargs** (`dict`) --
  Additional conditions for the Stable Diffusion XL UNet.
- **cross_attention_kwargs** (`dict[str]`, *optional*, defaults to `None`) --
  A kwargs dictionary that if specified is passed along to the `AttnProcessor`.
- **from_multi** (`bool`, defaults to `False`) --
  Use standard scaling when called from `MultiControlNetUnionModel`.
- **guess_mode** (`bool`, defaults to `False`) --
  In this mode, the ControlNet encoder tries its best to recognize the input content of the input even if
  you remove all prompts. A `guidance_scale` between 3.0 and 5.0 is recommended.
- **return_dict** (`bool`, defaults to `True`) --
  Whether or not to return a `~models.controlnet.ControlNetOutput` instead of a plain tuple.0`~models.controlnet.ControlNetOutput` **or** `tuple`If `return_dict` is `True`, a `~models.controlnet.ControlNetOutput` is returned, otherwise a tuple is
returned where the first element is the sample tensor.

The [ControlNetUnionModel](/docs/diffusers/v0.39.0/en/api/models/controlnet_union#diffusers.ControlNetUnionModel) forward method.

**Parameters:**

in_channels (`int`, defaults to 4) : The number of channels in the input sample.

flip_sin_to_cos (`bool`, defaults to `True`) : Whether to flip the sin to cos in the time embedding.

freq_shift (`int`, defaults to 0) : The frequency shift to apply to the time embedding.

down_block_types (`tuple[str]`, defaults to `("CrossAttnDownBlock2D", "CrossAttnDownBlock2D", "CrossAttnDownBlock2D", "DownBlock2D")`) : The tuple of downsample blocks to use.

only_cross_attention (`bool | tuple[bool]`, defaults to `False`) --

block_out_channels (`tuple[int]`, defaults to `(320, 640, 1280, 1280)`) : The tuple of output channels for each block.

layers_per_block (`int`, defaults to 2) : The number of layers per block.

downsample_padding (`int`, defaults to 1) : The padding to use for the downsampling convolution.

mid_block_scale_factor (`float`, defaults to 1) : The scale factor to use for the mid block.

act_fn (`str`, defaults to "silu") : The activation function to use.

norm_num_groups (`int`, *optional*, defaults to 32) : The number of groups to use for the normalization. If None, normalization and activation layers is skipped in post-processing.

norm_eps (`float`, defaults to 1e-5) : The epsilon to use for the normalization.

cross_attention_dim (`int`, defaults to 1280) : The dimension of the cross attention features.

transformer_layers_per_block (`int` or `tuple[int]`, *optional*, defaults to 1) : The number of transformer blocks of type `BasicTransformerBlock`. Only relevant for `~models.unet_2d_blocks.CrossAttnDownBlock2D`, `~models.unet_2d_blocks.CrossAttnUpBlock2D`, `~models.unet_2d_blocks.UNetMidBlock2DCrossAttn`.

encoder_hid_dim (`int`, *optional*, defaults to None) : If `encoder_hid_dim_type` is defined, `encoder_hidden_states` will be projected from `encoder_hid_dim` dimension to `cross_attention_dim`.

encoder_hid_dim_type (`str`, *optional*, defaults to `None`) : If given, the `encoder_hidden_states` and potentially other embeddings are down-projected to text embeddings of dimension `cross_attention` according to `encoder_hid_dim_type`.

attention_head_dim (`int | tuple[int]`, defaults to 8) : The dimension of the attention heads.

use_linear_projection (`bool`, defaults to `False`) --

class_embed_type (`str`, *optional*, defaults to `None`) : The type of class embedding to use which is ultimately summed with the time embeddings. Choose from None, `"timestep"`, `"identity"`, `"projection"`, or `"simple_projection"`.

addition_embed_type (`str`, *optional*, defaults to `None`) : Configures an optional embedding which will be summed with the time embeddings. Choose from `None` or "text". "text" will use the `TextTimeEmbedding` layer.

num_class_embeds (`int`, *optional*, defaults to 0) : Input dimension of the learnable embedding matrix to be projected to `time_embed_dim`, when performing class conditioning with `class_embed_type` equal to `None`.

upcast_attention (`bool`, defaults to `False`) --

resnet_time_scale_shift (`str`, defaults to `"default"`) : Time scale shift config for ResNet blocks (see `ResnetBlock2D`). Choose from `default` or `scale_shift`.

projection_class_embeddings_input_dim (`int`, *optional*, defaults to `None`) : The dimension of the `class_labels` input when `class_embed_type="projection"`. Required when `class_embed_type="projection"`.

controlnet_conditioning_channel_order (`str`, defaults to `"rgb"`) : The channel order of conditional image. Will convert to `rgb` if it's `bgr`.

conditioning_embedding_out_channels (`tuple[int]`, *optional*, defaults to `(48, 96, 192, 384)`) : The tuple of output channel for each block in the `conditioning_embedding` layer.

global_pool_conditions (`bool`, defaults to `False`) --

**Returns:**

``~models.controlnet.ControlNetOutput` **or** `tuple``

If `return_dict` is `True`, a `~models.controlnet.ControlNetOutput` is returned, otherwise a tuple is
returned where the first element is the sample tensor.
#### from_unet[[diffusers.ControlNetUnionModel.from_unet]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet_union.py#L389)

Instantiate a [ControlNetUnionModel](/docs/diffusers/v0.39.0/en/api/models/controlnet_union#diffusers.ControlNetUnionModel) from [UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel).

**Parameters:**

unet (`UNet2DConditionModel`) : The UNet model weights to copy to the [ControlNetUnionModel](/docs/diffusers/v0.39.0/en/api/models/controlnet_union#diffusers.ControlNetUnionModel). All configuration options are also copied where applicable.
#### set_attention_slice[[diffusers.ControlNetUnionModel.set_attention_slice]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet_union.py#L476)

Enable sliced attention computation.

When this option is enabled, the attention module splits the input tensor in slices to compute attention in
several steps. This is useful for saving some memory in exchange for a small decrease in speed.

**Parameters:**

slice_size (`str` or `int` or `list(int)`, *optional*, defaults to `"auto"`) : When `"auto"`, input to the attention heads is halved, so attention is computed in two steps. If `"max"`, maximum amount of memory is saved by running only one slice at a time. If a number is provided, uses as many slices as `attention_head_dim // slice_size`. In this case, `attention_head_dim` must be a multiple of `slice_size`.
#### set_default_attn_processor[[diffusers.ControlNetUnionModel.set_default_attn_processor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet_union.py#L460)

Disables custom attention processors and sets the default attention implementation.
