# EasyAnimateTransformer3DModel

A Diffusion Transformer model for 3D data from [EasyAnimate](https://github.com/aigc-apps/EasyAnimate) was introduced by Alibaba PAI.

The model can be loaded with the following code snippet.

```python
from diffusers import EasyAnimateTransformer3DModel

transformer = EasyAnimateTransformer3DModel.from_pretrained("alibaba-pai/EasyAnimateV5.1-12b-zh", subfolder="transformer", dtype=torch.float16).to("cuda")
```

## EasyAnimateTransformer3DModel[[diffusers.EasyAnimateTransformer3DModel]]

#### diffusers.EasyAnimateTransformer3DModel[[diffusers.EasyAnimateTransformer3DModel]]

```python
diffusers.EasyAnimateTransformer3DModel(num_attention_heads: int = 48, attention_head_dim: int = 64, in_channels: int | None = None, out_channels: int | None = None, patch_size: int | None = None, sample_width: int = 90, sample_height: int = 60, activation_fn: str = 'gelu-approximate', timestep_activation_fn: str = 'silu', freq_shift: int = 0, num_layers: int = 48, mmdit_layers: int = 48, dropout: float = 0.0, time_embed_dim: int = 512, add_norm_text_encoder: bool = False, text_embed_dim: int = 3584, text_embed_dim_t5: int = None, norm_eps: float = 1e-05, norm_elementwise_affine: bool = True, flip_sin_to_cos: bool = True, time_position_encoding_type: str = '3d_rope', after_norm = False, resize_inpaint_mask_directly: bool = True, enable_text_attention_mask: bool = True, add_noise_in_inpaint_model: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_easyanimate.py#L316)

**Parameters:**

num_attention_heads (`int`, defaults to `48`) : The number of heads to use for multi-head attention.

attention_head_dim (`int`, defaults to `64`) : The number of channels in each head.

in_channels (`int`, defaults to `16`) : The number of channels in the input.

out_channels (`int`, *optional*, defaults to `16`) : The number of channels in the output.

patch_size (`int`, defaults to `2`) : The size of the patches to use in the patch embedding layer.

sample_width (`int`, defaults to `90`) : The width of the input latents.

sample_height (`int`, defaults to `60`) : The height of the input latents.

activation_fn (`str`, defaults to `"gelu-approximate"`) : Activation function to use in feed-forward.

timestep_activation_fn (`str`, defaults to `"silu"`) : Activation function to use when generating the timestep embeddings.

num_layers (`int`, defaults to `30`) : The number of layers of Transformer blocks to use.

mmdit_layers (`int`, defaults to `1000`) : The number of layers of Multi Modal Transformer blocks to use.

dropout (`float`, defaults to `0.0`) : The dropout probability to use.

time_embed_dim (`int`, defaults to `512`) : Output dimension of timestep embeddings.

text_embed_dim (`int`, defaults to `4096`) : Input dimension of text embeddings from the text encoder.

norm_eps (`float`, defaults to `1e-5`) : The epsilon value to use in normalization layers.

norm_elementwise_affine (`bool`, defaults to `True`) : Whether to use elementwise affine in normalization layers.

flip_sin_to_cos (`bool`, defaults to `True`) : Whether to flip the sin to cos in the time embedding.

time_position_encoding_type (`str`, defaults to `3d_rope`) : Type of time position encoding.

after_norm (`bool`, defaults to `False`) : Flag to apply normalization after.

resize_inpaint_mask_directly (`bool`, defaults to `True`) : Flag to resize inpaint mask directly.

enable_text_attention_mask (`bool`, defaults to `True`) : Flag to enable text attention mask.

add_noise_in_inpaint_model (`bool`, defaults to `False`) : Flag to add noise in inpaint model.

A Transformer model for video-like data in [EasyAnimate](https://github.com/aigc-apps/EasyAnimate).

#### forward[[diffusers.EasyAnimateTransformer3DModel.forward]]

```python
forward(hidden_states: Tensor, timestep: Tensor, timestep_cond: typing.Optional[torch.Tensor] = None, encoder_hidden_states: typing.Optional[torch.Tensor] = None, encoder_hidden_states_t5: typing.Optional[torch.Tensor] = None, inpaint_latents: typing.Optional[torch.Tensor] = None, control_latents: typing.Optional[torch.Tensor] = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_easyanimate.py#L461)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch_size, channels, num_frames, height, width)`) : Input `hidden_states`.

timestep (`torch.LongTensor`) : Used to indicate denoising step.

timestep_cond (`torch.Tensor`, *optional*) : Conditional embeddings for timestep. If provided, the embeddings will be summed with the samples passed through the `self.time_embedding` layer to obtain the final timestep embeddings.

encoder_hidden_states (`torch.Tensor`, *optional*) : Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.

encoder_hidden_states_t5 (`torch.Tensor`, *optional*) : Additional conditional embeddings computed from a T5 text encoder.

inpaint_latents (`torch.Tensor`, *optional*) : Latents concatenated to `hidden_states` for inpainting variants of the model.

control_latents (`torch.Tensor`, *optional*) : Latents concatenated to `hidden_states` for control variants of the model.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [EasyAnimateTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/easyanimate_transformer3d#diffusers.EasyAnimateTransformer3DModel) forward method.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

```python
diffusers.models.modeling_outputs.Transformer2DModelOutput(sample: torch.Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/modeling_outputs.py#L21)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.

The output of [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel).
