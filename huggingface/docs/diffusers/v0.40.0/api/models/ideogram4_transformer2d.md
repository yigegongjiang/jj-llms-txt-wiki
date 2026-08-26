# Ideogram4Transformer2DModel

A transformer for image-like data from [Ideogram 4](https://github.com/ideogram-oss/ideogram-4). 

## Ideogram4Transformer2DModel[[diffusers.Ideogram4Transformer2DModel]]

#### diffusers.Ideogram4Transformer2DModel[[diffusers.Ideogram4Transformer2DModel]]

```python
diffusers.Ideogram4Transformer2DModel(in_channels: int = 128, num_layers: int = 34, attention_head_dim: int = 256, num_attention_heads: int = 18, intermediate_size: int = 12288, adaln_dim: int = 512, llm_features_dim: int = 53248, rope_theta: int = 5000000, mrope_section: tuple = (24, 20, 20), norm_eps: float = 1e-05)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_ideogram4.py#L278)

**Parameters:**

in_channels (`int`, defaults to 128) : Latent channel count after patchification (`ae_channels * patch_size ** 2`).

num_layers (`int`, defaults to 34) : Number of transformer blocks.

attention_head_dim (`int`, defaults to 256) : Dimension of each attention head; the total hidden size is `attention_head_dim * num_attention_heads`.

num_attention_heads (`int`, defaults to 18) : Number of attention heads.

intermediate_size (`int`, defaults to 12288) : Feed-forward hidden size used by the SwiGLU MLP inside each block.

adaln_dim (`int`, defaults to 512) : Dimensionality of the conditioning vector consumed by the AdaLN modulations.

llm_features_dim (`int`, defaults to 53248) : Dimensionality of the per-token text features fed into the model (typically a concatenation of hidden states from several layers of the text encoder).

rope_theta (`int`, defaults to 5_000_000) : Base used by the multi-axis rotary position embedding.

mrope_section (`tuple[int, int, int]`, defaults to `(24, 20, 20)`) : Number of frequencies allocated to each of the (t, h, w) axes of MRoPE.

norm_eps (`float`, defaults to 1e-5) : Epsilon used by the RMSNorm modules inside the transformer blocks.

The flow-matching transformer backbone used by the Ideogram 4 pipeline.

The transformer operates on a single packed sequence containing both text-conditioning tokens (produced by a
multimodal text encoder) and the patchified image latents. Per-token indicators distinguish the two roles, and a
block-diagonal attention mask derived from `segment_ids` restricts each sample to attend only to itself within a
packed batch.

#### forward[[diffusers.Ideogram4Transformer2DModel.forward]]

```python
forward(hidden_states: Tensor, timestep: Tensor, encoder_hidden_states: Tensor, position_ids: Tensor, segment_ids: Tensor, indicator: Tensor, attention_kwargs: dict | None = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_ideogram4.py#L373)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch_size, sequence_length, in_channels)`) : Packed sequence of patchified noisy image tokens. Non-image positions are masked out internally.

timestep (`torch.Tensor` of shape `(batch_size,)` or `(batch_size, sequence_length)`) : Flow-matching time in `[0, 1]` (0 is pure noise, 1 is clean data).

encoder_hidden_states (`torch.Tensor` of shape `(batch_size, sequence_length, llm_features_dim)`) : Per-token text conditioning features. Non-text positions are masked out internally.

position_ids (`torch.Tensor` of shape `(batch_size, sequence_length, 3)`) : `(t, h, w)` coordinates consumed by the multi-axis RoPE.

segment_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`) : Per-token sample id within a packed batch. Positions sharing a `segment_id` attend to each other.

indicator (`torch.Tensor` of shape `(batch_size, sequence_length)`) : Per-token role: `LLM_TOKEN_INDICATOR` (text) or `OUTPUT_IMAGE_INDICATOR` (image).

attention_kwargs (`dict`, *optional*) : A kwargs dictionary passed along to the attention processor. A `"scale"` entry scales the LoRA weights (when the PEFT backend is active).

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a [Transformer2DModelOutput](/docs/diffusers/v0.40.0/en/api/models/hunyuan_video15_transformer_3d#diffusers.models.modeling_outputs.Transformer2DModelOutput) instead of a plain tuple.

**Returns:**

[Transformer2DModelOutput](/docs/diffusers/v0.40.0/en/api/models/hunyuan_video15_transformer_3d#diffusers.models.modeling_outputs.Transformer2DModelOutput) or a `tuple` whose first element is a tensor of shape
`(batch_size, sequence_length, in_channels)` in the model's compute dtype. Only positions tagged with
`OUTPUT_IMAGE_INDICATOR` carry meaningful velocity predictions.

Predict the flow-matching velocity for the image-token positions of the packed sequence.
