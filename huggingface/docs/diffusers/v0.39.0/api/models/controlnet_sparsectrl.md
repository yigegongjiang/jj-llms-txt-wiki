# SparseControlNetModel

SparseControlNetModel is an implementation of ControlNet for [AnimateDiff](https://huggingface.co/papers/2307.04725).

ControlNet was introduced in [Adding Conditional Control to Text-to-Image Diffusion Models](https://huggingface.co/papers/2302.05543) by Lvmin Zhang, Anyi Rao, and Maneesh Agrawala.

The SparseCtrl version of ControlNet was introduced in [SparseCtrl: Adding Sparse Controls to Text-to-Video Diffusion Models](https://huggingface.co/papers/2311.16933) for achieving controlled generation in text-to-video diffusion models by Yuwei Guo, Ceyuan Yang, Anyi Rao, Maneesh Agrawala, Dahua Lin, and Bo Dai.

The abstract from the paper is:

*The development of text-to-video (T2V), i.e., generating videos with a given text prompt, has been significantly advanced in recent years. However, relying solely on text prompts often results in ambiguous frame composition due to spatial uncertainty. The research community thus leverages the dense structure signals, e.g., per-frame depth/edge sequences, to enhance controllability, whose collection accordingly increases the burden of inference. In this work, we present SparseCtrl to enable flexible structure control with temporally sparse signals, requiring only one or a few inputs, as shown in Figure 1. It incorporates an additional condition encoder to process these sparse signals while leaving the pre-trained T2V model untouched. The proposed approach is compatible with various modalities, including sketches, depth maps, and RGB images, providing more practical control for video generation and promoting applications such as storyboarding, depth rendering, keyframe animation, and interpolation. Extensive experiments demonstrate the generalization of SparseCtrl on both original and personalized T2V generators. Codes and models will be publicly available at [this https URL](https://guoyww.github.io/projects/SparseCtrl).*

## Example for loading SparseControlNetModel

```python
import torch
from diffusers import SparseControlNetModel

# fp32 variant in float16
# 1. Scribble checkpoint
controlnet = SparseControlNetModel.from_pretrained("guoyww/animatediff-sparsectrl-scribble", torch_dtype=torch.float16)

# 2. RGB checkpoint
controlnet = SparseControlNetModel.from_pretrained("guoyww/animatediff-sparsectrl-rgb", torch_dtype=torch.float16)

# For loading fp16 variant, pass `variant="fp16"` as an additional parameter
```

## SparseControlNetModel[[diffusers.SparseControlNetModel]]

#### diffusers.SparseControlNetModel[[diffusers.SparseControlNetModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet_sparsectrl.py#L97)

A SparseControlNet model as described in [SparseCtrl: Adding Sparse Controls to Text-to-Video Diffusion
Models](https://huggingface.co/papers/2311.16933).

forwarddiffusers.SparseControlNetModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet_sparsectrl.py#L534[{"name": "sample", "val": ": Tensor"}, {"name": "timestep", "val": ": torch.Tensor | float | int"}, {"name": "encoder_hidden_states", "val": ": Tensor"}, {"name": "controlnet_cond", "val": ": Tensor"}, {"name": "conditioning_scale", "val": ": float = 1.0"}, {"name": "timestep_cond", "val": ": torch.Tensor | None = None"}, {"name": "attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "conditioning_mask", "val": ": torch.Tensor | None = None"}, {"name": "guess_mode", "val": ": bool = False"}, {"name": "return_dict", "val": ": bool = True"}]- **sample** (`torch.Tensor`) --
  The noisy input tensor.
- **timestep** (`torch.Tensor | float | int`) --
  The number of timesteps to denoise an input.
- **encoder_hidden_states** (`torch.Tensor`) --
  The encoder hidden states.
- **controlnet_cond** (`torch.Tensor`) --
  The conditional input tensor of shape `(batch_size, sequence_length, hidden_size)`.
- **conditioning_scale** (`float`, defaults to `1.0`) --
  The scale factor for ControlNet outputs.
- **timestep_cond** (`torch.Tensor`, *optional*, defaults to `None`) --
  Additional conditional embeddings for timestep. If provided, the embeddings will be summed with the
  timestep_embedding passed through the `self.time_embedding` layer to obtain the final timestep
  embeddings.
- **attention_mask** (`torch.Tensor`, *optional*, defaults to `None`) --
  An attention mask of shape `(batch, key_tokens)` is applied to `encoder_hidden_states`. If `1` the mask
  is kept, otherwise if `0` it is discarded. Mask will be converted into a bias, which adds large
  negative values to the attention scores corresponding to "discard" tokens.
- **conditioning_mask** (`torch.Tensor`, *optional*, defaults to `None`) --
  Optional mask indicating which frames in `controlnet_cond` are valid conditioning frames.
- **cross_attention_kwargs** (`dict[str]`, *optional*, defaults to `None`) --
  A kwargs dictionary that if specified is passed along to the `AttnProcessor`.
- **guess_mode** (`bool`, defaults to `False`) --
  In this mode, the ControlNet encoder tries its best to recognize the input content of the input even if
  you remove all prompts. A `guidance_scale` between 3.0 and 5.0 is recommended.
- **return_dict** (`bool`, defaults to `True`) --
  Whether or not to return a `~models.controlnet.ControlNetOutput` instead of a plain tuple.0`~models.controlnet.ControlNetOutput` **or** `tuple`If `return_dict` is `True`, a `~models.controlnet.ControlNetOutput` is returned, otherwise a tuple is
returned where the first element is the sample tensor.

The [SparseControlNetModel](/docs/diffusers/v0.39.0/en/api/models/controlnet_sparsectrl#diffusers.SparseControlNetModel) forward method.

**Parameters:**

in_channels (`int`, defaults to 4) : The number of channels in the input sample.

conditioning_channels (`int`, defaults to 4) : The number of input channels in the controlnet conditional embedding module. If `concat_condition_embedding` is True, the value provided here is incremented by 1.

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

transformer_layers_per_mid_block (`int` or `tuple[int]`, *optional*, defaults to 1) : The number of transformer layers to use in each layer in the middle block.

attention_head_dim (`int` or `tuple[int]`, defaults to 8) : The dimension of the attention heads.

num_attention_heads (`int` or `tuple[int]`, *optional*) : The number of heads to use for multi-head attention.

use_linear_projection (`bool`, defaults to `False`) --

upcast_attention (`bool`, defaults to `False`) --

resnet_time_scale_shift (`str`, defaults to `"default"`) : Time scale shift config for ResNet blocks (see `ResnetBlock2D`). Choose from `default` or `scale_shift`.

conditioning_embedding_out_channels (`tuple[int]`, defaults to `(16, 32, 96, 256)`) : The tuple of output channel for each block in the `conditioning_embedding` layer.

global_pool_conditions (`bool`, defaults to `False`) : TODO(Patrick) - unused parameter

controlnet_conditioning_channel_order (`str`, defaults to `rgb`) --

motion_max_seq_length (`int`, defaults to `32`) : The maximum sequence length to use in the motion module.

motion_num_attention_heads (`int` or `tuple[int]`, defaults to `8`) : The number of heads to use in each attention layer of the motion module.

concat_conditioning_mask (`bool`, defaults to `True`) --

use_simplified_condition_embedding (`bool`, defaults to `True`) --

**Returns:**

``~models.controlnet.ControlNetOutput` **or** `tuple``

If `return_dict` is `True`, a `~models.controlnet.ControlNetOutput` is returned, otherwise a tuple is
returned where the first element is the sample tensor.
#### from_unet[[diffusers.SparseControlNetModel.from_unet]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet_sparsectrl.py#L388)

Instantiate a [SparseControlNetModel](/docs/diffusers/v0.39.0/en/api/models/controlnet_sparsectrl#diffusers.SparseControlNetModel) from [UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel).

**Parameters:**

unet (`UNet2DConditionModel`) : The UNet model weights to copy to the [SparseControlNetModel](/docs/diffusers/v0.39.0/en/api/models/controlnet_sparsectrl#diffusers.SparseControlNetModel). All configuration options are also copied where applicable.
#### set_attention_slice[[diffusers.SparseControlNetModel.set_attention_slice]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet_sparsectrl.py#L469)

Enable sliced attention computation.

When this option is enabled, the attention module splits the input tensor in slices to compute attention in
several steps. This is useful for saving some memory in exchange for a small decrease in speed.

**Parameters:**

slice_size (`str` or `int` or `list(int)`, *optional*, defaults to `"auto"`) : When `"auto"`, input to the attention heads is halved, so attention is computed in two steps. If `"max"`, maximum amount of memory is saved by running only one slice at a time. If a number is provided, uses as many slices as `attention_head_dim // slice_size`. In this case, `attention_head_dim` must be a multiple of `slice_size`.
#### set_default_attn_processor[[diffusers.SparseControlNetModel.set_default_attn_processor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet_sparsectrl.py#L453)

Disables custom attention processors and sets the default attention implementation.

## SparseControlNetOutput[[diffusers.models.controlnets.SparseControlNetOutput]]

#### diffusers.models.controlnets.SparseControlNetOutput[[diffusers.models.controlnets.SparseControlNetOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet_sparsectrl.py#L44)

The output of [SparseControlNetModel](/docs/diffusers/v0.39.0/en/api/models/controlnet_sparsectrl#diffusers.SparseControlNetModel).

**Parameters:**

down_block_res_samples (`tuple[torch.Tensor]`) : A tuple of downsample activations at different resolutions for each downsampling block. Each tensor should be of shape `(batch_size, channel * resolution, height //resolution, width // resolution)`. Output can be used to condition the original UNet's downsampling activations.

mid_down_block_re_sample (`torch.Tensor`) : The activation of the middle block (the lowest sample resolution). Each tensor should be of shape `(batch_size, channel * lowest_resolution, height // lowest_resolution, width // lowest_resolution)`. Output can be used to condition the original UNet's middle block activation.
