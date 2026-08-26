# QwenImageTransformer2DModel

The model can be loaded with the following code snippet.

```python
from diffusers import QwenImageTransformer2DModel

transformer = QwenImageTransformer2DModel.from_pretrained("Qwen/QwenImage-20B", subfolder="transformer", dtype=torch.bfloat16)
```

## QwenImageTransformer2DModel[[diffusers.QwenImageTransformer2DModel]]

#### diffusers.QwenImageTransformer2DModel[[diffusers.QwenImageTransformer2DModel]]

```python
diffusers.QwenImageTransformer2DModel(patch_size: int = 2, in_channels: int = 64, out_channels: int | None = 16, num_layers: int = 60, attention_head_dim: int = 128, num_attention_heads: int = 24, joint_attention_dim: int = 3584, guidance_embeds: bool = False, axes_dims_rope: tuple = (16, 56, 56), zero_cond_t: bool = False, use_additional_t_cond: bool = False, use_layer3d_rope: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_qwenimage.py#L791)

**Parameters:**

patch_size (`int`, defaults to `2`) : Patch size to turn the input data into small patches.

in_channels (`int`, defaults to `64`) : The number of channels in the input.

out_channels (`int`, *optional*, defaults to `None`) : The number of channels in the output. If not specified, it defaults to `in_channels`.

num_layers (`int`, defaults to `60`) : The number of layers of dual stream DiT blocks to use.

attention_head_dim (`int`, defaults to `128`) : The number of dimensions to use for each attention head.

num_attention_heads (`int`, defaults to `24`) : The number of attention heads to use.

joint_attention_dim (`int`, defaults to `3584`) : The number of dimensions to use for the joint attention (embedding/channel dimension of `encoder_hidden_states`).

guidance_embeds (`bool`, defaults to `False`) : Whether to use guidance embeddings for guidance-distilled variant of the model.

axes_dims_rope (`tuple[int]`, defaults to `(16, 56, 56)`) : The dimensions to use for the rotary positional embeddings.

The Transformer model introduced in Qwen.

#### forward[[diffusers.QwenImageTransformer2DModel.forward]]

```python
forward(hidden_states: Tensor, encoder_hidden_states: Tensor = None, encoder_hidden_states_mask: Tensor = None, timestep: LongTensor = None, img_shapes: list[tuple[int, int, int]] | None = None, guidance: Tensor = None, attention_kwargs: dict[str, typing.Any] | None = None, controlnet_block_samples = None, additional_t_cond = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_qwenimage.py#L911)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch_size, image_sequence_length, in_channels)`) : Input `hidden_states`.

encoder_hidden_states (`torch.Tensor` of shape `(batch_size, text_sequence_length, joint_attention_dim)`) : Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.

encoder_hidden_states_mask (`torch.Tensor` of shape `(batch_size, text_sequence_length)`, *optional*) : Mask for the encoder hidden states. Expected to have 1.0 for valid tokens and 0.0 for padding tokens. Used in the attention processor to prevent attending to padding tokens. The mask can have any pattern (not just contiguous valid tokens followed by padding) since it's applied element-wise in attention.

timestep ( `torch.LongTensor`) : Used to indicate denoising step.

img_shapes (`list[tuple[int, int, int]]`, *optional*) : Image shapes for RoPE computation.

guidance (`torch.Tensor`, *optional*) : Guidance tensor for conditional generation.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

controlnet_block_samples (*optional*) : ControlNet block samples to add to the transformer blocks.

additional_t_cond (`torch.Tensor`, *optional*) : Additional timestep conditioning added to the timestep embedding.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The `QwenTransformer2DModel` forward method.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

```python
diffusers.models.modeling_outputs.Transformer2DModelOutput(sample: torch.Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/modeling_outputs.py#L21)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.

The output of [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel).
