# Lumina2Transformer2DModel

A Diffusion Transformer model for 3D video-like data was introduced in [Lumina Image 2.0](https://huggingface.co/Alpha-VLLM/Lumina-Image-2.0) by Alpha-VLLM.

The model can be loaded with the following code snippet.

```python
from diffusers import Lumina2Transformer2DModel

transformer = Lumina2Transformer2DModel.from_pretrained("Alpha-VLLM/Lumina-Image-2.0", subfolder="transformer", dtype=torch.bfloat16)
```

## Lumina2Transformer2DModel[[diffusers.Lumina2Transformer2DModel]]

#### diffusers.Lumina2Transformer2DModel[[diffusers.Lumina2Transformer2DModel]]

```python
diffusers.Lumina2Transformer2DModel(sample_size: int = 128, patch_size: int = 2, in_channels: int = 16, out_channels: int | None = None, hidden_size: int = 2304, num_layers: int = 26, num_refiner_layers: int = 2, num_attention_heads: int = 24, num_kv_heads: int = 8, multiple_of: int = 256, ffn_dim_multiplier: float | None = None, norm_eps: float = 1e-05, scaling_factor: float = 1.0, axes_dim_rope: tuple = (32, 32, 32), axes_lens: tuple = (300, 512, 512), cap_feat_dim: int = 1024)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_lumina2.py#L325)

**Parameters:**

sample_size (`int`) : The width of the latent images. This is fixed during training since it is used to learn a number of position embeddings.

patch_size (`int`, *optional*, (`int`, *optional*, defaults to 2) : The size of each patch in the image. This parameter defines the resolution of patches fed into the model.

in_channels (`int`, *optional*, defaults to 4) : The number of input channels for the model. Typically, this matches the number of channels in the input images.

hidden_size (`int`, *optional*, defaults to 4096) : The dimensionality of the hidden layers in the model. This parameter determines the width of the model's hidden representations.

num_layers (`int`, *optional*, default to 32) : The number of layers in the model. This defines the depth of the neural network.

num_attention_heads (`int`, *optional*, defaults to 32) : The number of attention heads in each attention layer. This parameter specifies how many separate attention mechanisms are used.

num_kv_heads (`int`, *optional*, defaults to 8) : The number of key-value heads in the attention mechanism, if different from the number of attention heads. If None, it defaults to num_attention_heads.

multiple_of (`int`, *optional*, defaults to 256) : A factor that the hidden size should be a multiple of. This can help optimize certain hardware configurations.

ffn_dim_multiplier (`float`, *optional*) : A multiplier for the dimensionality of the feed-forward network. If None, it uses a default value based on the model configuration.

norm_eps (`float`, *optional*, defaults to 1e-5) : A small value added to the denominator for numerical stability in normalization layers.

scaling_factor (`float`, *optional*, defaults to 1.0) : A scaling factor applied to certain parameters or layers in the model. This can be used for adjusting the overall scale of the model's operations.

Lumina2NextDiT: Diffusion model with a Transformer backbone.

#### forward[[diffusers.Lumina2Transformer2DModel.forward]]

```python
forward(hidden_states: Tensor, timestep: Tensor, encoder_hidden_states: Tensor, encoder_attention_mask: Tensor, attention_kwargs: dict[str, typing.Any] | None = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_lumina2.py#L458)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch_size, in_channels, height, width)`) : Input `hidden_states`.

timestep (`torch.LongTensor`) : Used to indicate denoising step.

encoder_hidden_states (`torch.Tensor` of shape `(batch_size, sequence_len, embed_dims)`) : Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.

encoder_attention_mask (`torch.Tensor`) : Mask applied to `encoder_hidden_states` during attention.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [Lumina2Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/lumina2_transformer2d#diffusers.Lumina2Transformer2DModel) forward method.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

```python
diffusers.models.modeling_outputs.Transformer2DModelOutput(sample: torch.Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/modeling_outputs.py#L21)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.

The output of [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel).
