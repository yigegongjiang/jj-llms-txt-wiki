# SD3ControlNetModel

SD3ControlNetModel is an implementation of ControlNet for Stable Diffusion 3.

The ControlNet model was introduced in [Adding Conditional Control to Text-to-Image Diffusion Models](https://huggingface.co/papers/2302.05543) by Lvmin Zhang, Anyi Rao, Maneesh Agrawala. It provides a greater degree of control over text-to-image generation by conditioning the model on additional inputs such as edge maps, depth maps, segmentation maps, and keypoints for pose detection.

The abstract from the paper is:

*We present ControlNet, a neural network architecture to add spatial conditioning controls to large, pretrained text-to-image diffusion models. ControlNet locks the production-ready large diffusion models, and reuses their deep and robust encoding layers pretrained with billions of images as a strong backbone to learn a diverse set of conditional controls. The neural architecture is connected with "zero convolutions" (zero-initialized convolution layers) that progressively grow the parameters from zero and ensure that no harmful noise could affect the finetuning. We test various conditioning controls, eg, edges, depth, segmentation, human pose, etc, with Stable Diffusion, using single or multiple conditions, with or without prompts. We show that the training of ControlNets is robust with small (<50k) and large (>1m) datasets. Extensive results show that ControlNet may facilitate wider applications to control image diffusion models.*

## Loading from the original format

By default the [SD3ControlNetModel](/docs/diffusers/v0.40.0/en/api/models/controlnet_sd3#diffusers.SD3ControlNetModel) should be loaded with [from_pretrained()](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin.from_pretrained).

```py
from diffusers import StableDiffusion3ControlNetPipeline
from diffusers.models import SD3ControlNetModel, SD3MultiControlNetModel

controlnet = SD3ControlNetModel.from_pretrained("InstantX/SD3-Controlnet-Canny")
pipe = StableDiffusion3ControlNetPipeline.from_pretrained("stabilityai/stable-diffusion-3-medium-diffusers", controlnet=controlnet)
```

## SD3ControlNetModel[[diffusers.SD3ControlNetModel]]

#### diffusers.SD3ControlNetModel[[diffusers.SD3ControlNetModel]]

```python
diffusers.SD3ControlNetModel(sample_size: int = 128, patch_size: int = 2, in_channels: int = 16, num_layers: int = 18, attention_head_dim: int = 64, num_attention_heads: int = 18, joint_attention_dim: int = 4096, caption_projection_dim: int = 1152, pooled_projection_dim: int = 2048, out_channels: int = 16, pos_embed_max_size: int = 96, extra_conditioning_channels: int = 0, dual_attention_layers: tuple = (), qk_norm: str | None = None, pos_embed_type: str | None = 'sincos', use_pos_embed: bool = True, force_zeros_for_pooled_projection: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/controlnets/controlnet_sd3.py#L42)

**Parameters:**

sample_size (`int`, defaults to `128`) : The width/height of the latents. This is fixed during training since it is used to learn a number of position embeddings.

patch_size (`int`, defaults to `2`) : Patch size to turn the input data into small patches.

in_channels (`int`, defaults to `16`) : The number of latent channels in the input.

num_layers (`int`, defaults to `18`) : The number of layers of transformer blocks to use.

attention_head_dim (`int`, defaults to `64`) : The number of channels in each head.

num_attention_heads (`int`, defaults to `18`) : The number of heads to use for multi-head attention.

joint_attention_dim (`int`, defaults to `4096`) : The embedding dimension to use for joint text-image attention.

caption_projection_dim (`int`, defaults to `1152`) : The embedding dimension of caption embeddings.

pooled_projection_dim (`int`, defaults to `2048`) : The embedding dimension of pooled text projections.

out_channels (`int`, defaults to `16`) : The number of latent channels in the output.

pos_embed_max_size (`int`, defaults to `96`) : The maximum latent height/width of positional embeddings.

extra_conditioning_channels (`int`, defaults to `0`) : The number of extra channels to use for conditioning for patch embedding.

dual_attention_layers (`tuple[int, ...]`, defaults to `()`) : The number of dual-stream transformer blocks to use.

qk_norm (`str`, *optional*, defaults to `None`) : The normalization to use for query and key in the attention layer. If `None`, no normalization is used.

pos_embed_type (`str`, defaults to `"sincos"`) : The type of positional embedding to use. Choose between `"sincos"` and `None`.

use_pos_embed (`bool`, defaults to `True`) : Whether to use positional embeddings.

force_zeros_for_pooled_projection (`bool`, defaults to `True`) : Whether to force zeros for pooled projection embeddings. This is handled in the pipelines by reading the config value of the ControlNet model.

ControlNet model for [Stable Diffusion 3](https://huggingface.co/papers/2403.03206).

#### enable_forward_chunking[[diffusers.SD3ControlNetModel.enable_forward_chunking]]

```python
enable_forward_chunking(chunk_size: int | None = None, dim: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/controlnets/controlnet_sd3.py#L178)

**Parameters:**

chunk_size (`int`, *optional*) : The chunk size of the feed-forward layers. If not specified, will run feed-forward layer individually over each tensor of dim=`dim`.

dim (`int`, *optional*, defaults to `0`) : The dimension over which the feed-forward computation should be chunked. Choose between dim=0 (batch) or dim=1 (sequence length).

Sets the attention processor to use [feed forward
chunking](https://huggingface.co/blog/reformer#2-chunked-feed-forward-layers).

#### forward[[diffusers.SD3ControlNetModel.forward]]

```python
forward(hidden_states: Tensor, controlnet_cond: Tensor, conditioning_scale: float = 1.0, encoder_hidden_states: Tensor = None, pooled_projections: Tensor = None, timestep: LongTensor = None, joint_attention_kwargs: dict[str, typing.Any] | None = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/controlnets/controlnet_sd3.py#L272)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch size, channel, height, width)`) : Input `hidden_states`.

controlnet_cond (`torch.Tensor`) : The conditional input tensor of shape `(batch_size, sequence_length, hidden_size)`.

conditioning_scale (`float`, defaults to `1.0`) : The scale factor for ControlNet outputs.

encoder_hidden_states (`torch.Tensor` of shape `(batch size, sequence_len, embed_dims)`) : Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.

pooled_projections (`torch.Tensor` of shape `(batch_size, projection_dim)`) : Embeddings projected from the embeddings of input conditions.

timestep ( `torch.LongTensor`) : Used to indicate denoising step.

joint_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [SD3Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/sd3_transformer2d#diffusers.SD3Transformer2DModel) forward method.

#### fuse_qkv_projections[[diffusers.SD3ControlNetModel.fuse_qkv_projections]]

```python
fuse_qkv_projections()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/controlnets/controlnet_sd3.py#L208)

Enables fused QKV projections. For self-attention modules, all projection matrices (i.e., query, key, value)
are fused. For cross-attention modules, key and value projection matrices are fused.

> [!WARNING] > This API is 🧪 experimental.

#### unfuse_qkv_projections[[diffusers.SD3ControlNetModel.unfuse_qkv_projections]]

```python
unfuse_qkv_projections()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/controlnets/controlnet_sd3.py#L230)

Disables the fused QKV projection if enabled.

> [!WARNING] > This API is 🧪 experimental.

## SD3ControlNetOutput[[diffusers.models.controlnets.SD3ControlNetOutput]]

#### diffusers.models.controlnets.SD3ControlNetOutput[[diffusers.models.controlnets.SD3ControlNetOutput]]

```python
diffusers.models.controlnets.SD3ControlNetOutput(controlnet_block_samples: tuple)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/controlnets/controlnet_sd3.py#L38)
