# LuminaNextDiT2DModel

A Next Version of Diffusion Transformer model for 2D data from [Lumina-T2X](https://github.com/Alpha-VLLM/Lumina-T2X).

## LuminaNextDiT2DModel[[diffusers.LuminaNextDiT2DModel]]

#### diffusers.LuminaNextDiT2DModel[[diffusers.LuminaNextDiT2DModel]]

```python
diffusers.LuminaNextDiT2DModel(sample_size: int = 128, patch_size: int | None = 2, in_channels: int | None = 4, hidden_size: int | None = 2304, num_layers: int | None = 32, num_attention_heads: int | None = 32, num_kv_heads: int | None = None, multiple_of: int | None = 256, ffn_dim_multiplier: float | None = None, norm_eps: float | None = 1e-05, learn_sigma: bool | None = True, qk_norm: bool | None = True, cross_attention_dim: int | None = 2048, scaling_factor: float | None = 1.0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/lumina_nextdit2d.py#L178)

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

learn_sigma (`bool`, *optional*, defaults to True) : Whether the model should learn the sigma parameter, which might be related to uncertainty or variance in predictions.

qk_norm (`bool`, *optional*, defaults to True) : Indicates if the queries and keys in the attention mechanism should be normalized.

cross_attention_dim (`int`, *optional*, defaults to 2048) : The dimensionality of the text embeddings. This parameter defines the size of the text representations used in the model.

scaling_factor (`float`, *optional*, defaults to 1.0) : A scaling factor applied to certain parameters or layers in the model. This can be used for adjusting the overall scale of the model's operations.

LuminaNextDiT: Diffusion model with a Transformer backbone.

Inherit ModelMixin and ConfigMixin to be compatible with the sampler StableDiffusionPipeline of diffusers.

#### forward[[diffusers.LuminaNextDiT2DModel.forward]]

```python
forward(hidden_states: Tensor, timestep: Tensor, encoder_hidden_states: Tensor, encoder_mask: Tensor, image_rotary_emb: Tensor, cross_attention_kwargs: dict = None, return_dict = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/lumina_nextdit2d.py#L291)

**Parameters:**

hidden_states (torch.Tensor) : Input tensor of shape (N, C, H, W).

timestep (torch.Tensor) : Tensor of diffusion timesteps of shape (N,).

encoder_hidden_states (torch.Tensor) : Tensor of caption features of shape (N, D).

encoder_mask (torch.Tensor) : Tensor of caption masks of shape (N, L).

image_rotary_emb (`torch.Tensor`) : Pre-computed rotary positional embeddings.

cross_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

**Returns:** `~models.transformer_2d.Transformer2DModelOutput` or `tuple`

If `return_dict` is True, a `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise
a plain `tuple` is returned.

Forward pass of LuminaNextDiT.
