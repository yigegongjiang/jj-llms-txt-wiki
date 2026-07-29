# ControlNetModel

The ControlNet model was introduced in [Adding Conditional Control to Text-to-Image Diffusion Models](https://huggingface.co/papers/2302.05543) by Lvmin Zhang, Anyi Rao, Maneesh Agrawala. It provides a greater degree of control over text-to-image generation by conditioning the model on additional inputs such as edge maps, depth maps, segmentation maps, and keypoints for pose detection.

The abstract from the paper is:

*We present ControlNet, a neural network architecture to add spatial conditioning controls to large, pretrained text-to-image diffusion models. ControlNet locks the production-ready large diffusion models, and reuses their deep and robust encoding layers pretrained with billions of images as a strong backbone to learn a diverse set of conditional controls. The neural architecture is connected with "zero convolutions" (zero-initialized convolution layers) that progressively grow the parameters from zero and ensure that no harmful noise could affect the finetuning. We test various conditioning controls, eg, edges, depth, segmentation, human pose, etc, with Stable Diffusion, using single or multiple conditions, with or without prompts. We show that the training of ControlNets is robust with small (<50k) and large (>1m) datasets. Extensive results show that ControlNet may facilitate wider applications to control image diffusion models.*

## Loading from the original format

By default the [ControlNetModel](/docs/diffusers/v0.39.0/en/api/models/controlnet#diffusers.ControlNetModel) should be loaded with [from_pretrained()](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin.from_pretrained), but it can also be loaded
from the original format using `FromOriginalModelMixin.from_single_file` as follows:

```py
from diffusers import StableDiffusionControlNetPipeline, ControlNetModel

url = "https://huggingface.co/lllyasviel/ControlNet-v1-1/blob/main/control_v11p_sd15_canny.pth"  # can also be a local path
controlnet = ControlNetModel.from_single_file(url)

url = "https://huggingface.co/stable-diffusion-v1-5/stable-diffusion-v1-5/blob/main/v1-5-pruned.safetensors"  # can also be a local path
pipe = StableDiffusionControlNetPipeline.from_single_file(url, controlnet=controlnet)
```

## Loading from Control LoRA

Control-LoRA is introduced by Stability AI in [stabilityai/control-lora](https://huggingface.co/stabilityai/control-lora) by adding low-rank parameter efficient fine tuning to ControlNet. This approach offers a more efficient and compact method to bring model control to a wider variety of consumer GPUs.

```py
from diffusers import ControlNetModel, UNet2DConditionModel

lora_id = "stabilityai/control-lora"
lora_filename = "control-LoRAs-rank128/control-lora-canny-rank128.safetensors"

unet = UNet2DConditionModel.from_pretrained("stabilityai/stable-diffusion-xl-base-1.0", subfolder="unet", torch_dtype=torch.bfloat16).to("cuda")
controlnet = ControlNetModel.from_unet(unet).to(device="cuda", dtype=torch.bfloat16)
controlnet.load_lora_adapter(lora_id, weight_name=lora_filename, prefix=None, controlnet_config=controlnet.config)
```

## ControlNetModel[[diffusers.ControlNetModel]]

#### diffusers.ControlNetModel[[diffusers.ControlNetModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet.py#L111)

A ControlNet model.

forwarddiffusers.ControlNetModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet.py#L602[{"name": "sample", "val": ": Tensor"}, {"name": "timestep", "val": ": torch.Tensor | float | int"}, {"name": "encoder_hidden_states", "val": ": Tensor"}, {"name": "controlnet_cond", "val": ": Tensor"}, {"name": "conditioning_scale", "val": ": float = 1.0"}, {"name": "class_labels", "val": ": torch.Tensor | None = None"}, {"name": "timestep_cond", "val": ": torch.Tensor | None = None"}, {"name": "attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "added_cond_kwargs", "val": ": dict[str, torch.Tensor] | None = None"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "guess_mode", "val": ": bool = False"}, {"name": "return_dict", "val": ": bool = True"}]- **sample** (`torch.Tensor`) --
  The noisy input tensor.
- **timestep** (`torch.Tensor | float | int`) --
  The number of timesteps to denoise an input.
- **encoder_hidden_states** (`torch.Tensor`) --
  The encoder hidden states.
- **controlnet_cond** (`torch.Tensor`) --
  The conditional input tensor of shape `(batch_size, sequence_length, hidden_size)`.
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
- **guess_mode** (`bool`, defaults to `False`) --
  In this mode, the ControlNet encoder tries its best to recognize the input content of the input even if
  you remove all prompts. A `guidance_scale` between 3.0 and 5.0 is recommended.
- **return_dict** (`bool`, defaults to `True`) --
  Whether or not to return a [ControlNetOutput](/docs/diffusers/v0.39.0/en/api/models/controlnet#diffusers.models.controlnets.ControlNetOutput) instead of a plain
  tuple.0[ControlNetOutput](/docs/diffusers/v0.39.0/en/api/models/controlnet#diffusers.models.controlnets.ControlNetOutput) **or** `tuple`If `return_dict` is `True`, a [ControlNetOutput](/docs/diffusers/v0.39.0/en/api/models/controlnet#diffusers.models.controlnets.ControlNetOutput) is returned,
otherwise a tuple is returned where the first element is the sample tensor.

The [ControlNetModel](/docs/diffusers/v0.39.0/en/api/models/controlnet#diffusers.ControlNetModel) forward method.

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

conditioning_embedding_out_channels (`tuple[int]`, *optional*, defaults to `(16, 32, 96, 256)`) : The tuple of output channel for each block in the `conditioning_embedding` layer.

global_pool_conditions (`bool`, defaults to `False`) : TODO(Patrick) - unused parameter.

addition_embed_type_num_heads (`int`, defaults to 64) : The number of heads to use for the `TextTimeEmbedding` layer.

**Returns:**

`[ControlNetOutput](/docs/diffusers/v0.39.0/en/api/models/controlnet#diffusers.models.controlnets.ControlNetOutput) **or** `tuple``

If `return_dict` is `True`, a [ControlNetOutput](/docs/diffusers/v0.39.0/en/api/models/controlnet#diffusers.models.controlnets.ControlNetOutput) is returned,
otherwise a tuple is returned where the first element is the sample tensor.
#### from_unet[[diffusers.ControlNetModel.from_unet]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet.py#L444)

Instantiate a [ControlNetModel](/docs/diffusers/v0.39.0/en/api/models/controlnet#diffusers.ControlNetModel) from [UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel).

**Parameters:**

unet (`UNet2DConditionModel`) : The UNet model weights to copy to the [ControlNetModel](/docs/diffusers/v0.39.0/en/api/models/controlnet#diffusers.ControlNetModel). All configuration options are also copied where applicable.
#### set_attention_slice[[diffusers.ControlNetModel.set_attention_slice]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet.py#L537)

Enable sliced attention computation.

When this option is enabled, the attention module splits the input tensor in slices to compute attention in
several steps. This is useful for saving some memory in exchange for a small decrease in speed.

**Parameters:**

slice_size (`str` or `int` or `list(int)`, *optional*, defaults to `"auto"`) : When `"auto"`, input to the attention heads is halved, so attention is computed in two steps. If `"max"`, maximum amount of memory is saved by running only one slice at a time. If a number is provided, uses as many slices as `attention_head_dim // slice_size`. In this case, `attention_head_dim` must be a multiple of `slice_size`.
#### set_default_attn_processor[[diffusers.ControlNetModel.set_default_attn_processor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet.py#L521)

Disables custom attention processors and sets the default attention implementation.

## ControlNetOutput[[diffusers.models.controlnets.ControlNetOutput]]

#### diffusers.models.controlnets.ControlNetOutput[[diffusers.models.controlnets.ControlNetOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet.py#L47)

The output of [ControlNetModel](/docs/diffusers/v0.39.0/en/api/models/controlnet#diffusers.ControlNetModel).

**Parameters:**

down_block_res_samples (`tuple[torch.Tensor]`) : A tuple of downsample activations at different resolutions for each downsampling block. Each tensor should be of shape `(batch_size, channel * resolution, height //resolution, width // resolution)`. Output can be used to condition the original UNet's downsampling activations.

mid_down_block_re_sample (`torch.Tensor`) : The activation of the middle block (the lowest sample resolution). Each tensor should be of shape `(batch_size, channel * lowest_resolution, height // lowest_resolution, width // lowest_resolution)`. Output can be used to condition the original UNet's middle block activation.
