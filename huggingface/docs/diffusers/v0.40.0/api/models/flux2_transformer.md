# Flux2Transformer2DModel

A Transformer model for image-like data from [Flux2](https://hf.co/black-forest-labs/FLUX.2-dev).

## Flux2Transformer2DModel[[diffusers.Flux2Transformer2DModel]]

#### diffusers.Flux2Transformer2DModel[[diffusers.Flux2Transformer2DModel]]

```python
diffusers.Flux2Transformer2DModel(patch_size: int = 1, in_channels: int = 128, out_channels: int | None = None, num_layers: int = 8, num_single_layers: int = 48, attention_head_dim: int = 128, num_attention_heads: int = 48, joint_attention_dim: int = 15360, timestep_guidance_channels: int = 256, mlp_ratio: float = 3.0, axes_dims_rope: tuple = (32, 32, 32, 32), rope_theta: int = 2000, eps: float = 1e-06, guidance_embeds: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_flux2.py#L1059)

**Parameters:**

patch_size (`int`, defaults to `1`) : Patch size to turn the input data into small patches.

in_channels (`int`, defaults to `128`) : The number of channels in the input.

out_channels (`int`, *optional*, defaults to `None`) : The number of channels in the output. If not specified, it defaults to `in_channels`.

num_layers (`int`, defaults to `8`) : The number of layers of dual stream DiT blocks to use.

num_single_layers (`int`, defaults to `48`) : The number of layers of single stream DiT blocks to use.

attention_head_dim (`int`, defaults to `128`) : The number of dimensions to use for each attention head.

num_attention_heads (`int`, defaults to `48`) : The number of attention heads to use.

joint_attention_dim (`int`, defaults to `15360`) : The number of dimensions to use for the joint attention (embedding/channel dimension of `encoder_hidden_states`).

pooled_projection_dim (`int`, defaults to `768`) : The number of dimensions to use for the pooled projection.

guidance_embeds (`bool`, defaults to `True`) : Whether to use guidance embeddings for guidance-distilled variant of the model.

axes_dims_rope (`tuple[int]`, defaults to `(32, 32, 32, 32)`) : The dimensions to use for the rotary positional embeddings.

The Transformer model introduced in Flux 2.

Reference: https://blackforestlabs.ai/announcing-black-forest-labs/

#### forward[[diffusers.Flux2Transformer2DModel.forward]]

```python
forward(hidden_states: Tensor, encoder_hidden_states: Tensor = None, timestep: LongTensor = None, img_ids: Tensor = None, txt_ids: Tensor = None, guidance: Tensor = None, joint_attention_kwargs: dict[str, typing.Any] | None = None, return_dict: bool = True, kv_cache: Flux2KVCache | None = None, kv_cache_mode: str | None = None, num_ref_tokens: int = 0, ref_fixed_timestep: float = 0.0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_flux2.py#L1225)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch_size, image_sequence_length, in_channels)`) : Input `hidden_states`.

encoder_hidden_states (`torch.Tensor` of shape `(batch_size, text_sequence_length, joint_attention_dim)`) : Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.

timestep (`torch.LongTensor`) : Used to indicate denoising step.

img_ids (`torch.Tensor`) : Image position ids used to compute the rotary positional embeddings.

txt_ids (`torch.Tensor`) : Text position ids used to compute the rotary positional embeddings.

guidance (`torch.Tensor`, *optional*) : Guidance scale embedding used for guidance-distilled variants of the model.

joint_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

kv_cache (`Flux2KVCache`, *optional*) : KV cache for reference image tokens. When `kv_cache_mode` is "extract", a new cache is created and returned. When "cached", the provided cache is used to inject ref K/V during attention.

kv_cache_mode (`str`, *optional*) : One of "extract" (first step with ref tokens) or "cached" (subsequent steps using cached ref K/V). When `None`, standard forward pass without KV caching.

num_ref_tokens (`int`, defaults to `0`) : Number of reference image tokens prepended to `hidden_states` (only used when `kv_cache_mode="extract"`).

ref_fixed_timestep (`float`, defaults to `0.0`) : Fixed timestep for reference token modulation (only used when `kv_cache_mode="extract"`).

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor. When `kv_cache_mode="extract"`, also returns the
populated `Flux2KVCache`.

The [Flux2Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/flux2_transformer#diffusers.Flux2Transformer2DModel) forward method.

## Flux2Transformer2DModelOutput[[diffusers.models.transformers.transformer_flux2.Flux2Transformer2DModelOutput]]

#### diffusers.models.transformers.transformer_flux2.Flux2Transformer2DModelOutput[[diffusers.models.transformers.transformer_flux2.Flux2Transformer2DModelOutput]]

```python
diffusers.models.transformers.transformer_flux2.Flux2Transformer2DModelOutput(sample: torch.Tensor, kv_cache: Flux2KVCache | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_flux2.py#L46)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)`) : The hidden states output conditioned on the `encoder_hidden_states` input.

kv_cache (`Flux2KVCache`, *optional*) : The populated KV cache for reference image tokens. Only returned when `kv_cache_mode="extract"`.

The output of [Flux2Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/flux2_transformer#diffusers.Flux2Transformer2DModel).
