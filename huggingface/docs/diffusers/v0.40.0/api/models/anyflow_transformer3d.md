# AnyFlowTransformer3DModel

The bidirectional 3D Transformer used by [`AnyFlowPipeline`](../pipelines/anyflow#anyflowpipeline). It is the
v0.35.1 Wan2.1 backbone with one structural change: the timestep embedder is replaced by
``AnyFlowDualTimestepTextImageEmbedding``, so every forward call conditions on both the source timestep
``t`` and the target timestep ``r``. This is the embedding required to learn the flow map
$\Phi_{r\leftarrow t}$ introduced in
[AnyFlow](https://huggingface.co/papers/2605.13724). See the [`AnyFlowPipeline`](../pipelines/anyflow) page
for paper, authors, and released checkpoints.

For chunk-wise autoregressive (FAR causal) generation, use
[`AnyFlowFARTransformer3DModel`](anyflow_far_transformer3d) instead.

```python
from diffusers import AnyFlowTransformer3DModel

# Bidirectional AnyFlow checkpoint (T2V):
transformer = AnyFlowTransformer3DModel.from_pretrained(
    "nvidia/AnyFlow-Wan2.1-T2V-1.3B-Diffusers", subfolder="transformer"
)
```

## AnyFlowTransformer3DModel[[diffusers.AnyFlowTransformer3DModel]]

#### diffusers.AnyFlowTransformer3DModel[[diffusers.AnyFlowTransformer3DModel]]

```python
diffusers.AnyFlowTransformer3DModel(patch_size: typing.Tuple[int] = (1, 2, 2), num_attention_heads: int = 40, attention_head_dim: int = 128, in_channels: int = 16, out_channels: int = 16, text_dim: int = 4096, freq_dim: int = 256, ffn_dim: int = 13824, num_layers: int = 40, cross_attn_norm: bool = True, eps: float = 1e-06, image_dim: typing.Optional[int] = None, rope_max_seq_len: int = 1024, gate_value: float = 0.25, deltatime_type: str = 'r')
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_anyflow.py#L507)

**Parameters:**

patch_size (*Tuple[int]*, defaults to *(1, 2, 2)*) : 3D patch dimensions for video embedding (t_patch, h_patch, w_patch).

num_attention_heads (*int*, defaults to *40*) : Number of attention heads.

attention_head_dim (*int*, defaults to *128*) : The number of channels in each head.

in_channels (*int*, defaults to *16*) : The number of channels in the input latent.

out_channels (*int*, defaults to *16*) : The number of channels in the output latent.

text_dim (*int*, defaults to *4096*) : Input dimension for text embeddings (UMT5).

freq_dim (*int*, defaults to *256*) : Dimension for sinusoidal time embeddings.

ffn_dim (*int*, defaults to *13824*) : Intermediate dimension in feed-forward network.

num_layers (*int*, defaults to *40*) : Number of transformer blocks.

cross_attn_norm (*bool*, defaults to *True*) : Enable cross-attention normalization.

eps (*float*, defaults to *1e-6*) : Epsilon for normalization layers.

image_dim (*Optional[int]*, *optional*, defaults to *None*) : Image embedding dimension for I2V conditioning (*1280* for the original Wan2.1-I2V model).

rope_max_seq_len (*int*, defaults to *1024*) : Maximum sequence length used to precompute rotary position frequencies.

gate_value (*float*, defaults to *0.25*) : Mixing gate between source-timestep and delta-timestep embeddings (the AnyFlow paper's \\(g\\) parameter, fixed at 0.25 in stage-1 distillation).

deltatime_type (*str*, defaults to *'r'*) : Either `"r"` (delta is the target timestep) or `"t-r"` (delta is the absolute interval).

Bidirectional 3D Transformer for AnyFlow flow-map sampling.

The architecture is the v0.35.1 Wan2.1 3D DiT backbone with one structural change: the timestep embedder is
replaced by `AnyFlowDualTimestepTextImageEmbedding` so that every forward call conditions on both the source
timestep `t` and the target timestep `r`. This is the embedding required to learn the flow map
\\(\Phi_&amp;lcub;r\leftarrow t}\\) introduced in [AnyFlow](https://huggingface.co/papers/2605.13724).

For chunk-wise autoregressive (FAR causal) generation, use `AnyFlowFARTransformer3DModel` instead; that variant
adds the FAR causal block-mask and a compressed-frame patch embedding on top of the same backbone.

#### forward[[diffusers.AnyFlowTransformer3DModel.forward]]

```python
forward(hidden_states: Tensor, timestep: Tensor, r_timestep: Tensor, encoder_hidden_states: Tensor, encoder_hidden_states_image: typing.Optional[torch.Tensor] = None, attention_kwargs: typing.Optional[typing.Dict[str, typing.Any]] = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_anyflow.py#L626)

**Parameters:**

hidden_states (*torch.Tensor* of shape *(batch_size, num_frames, num_channels, height, width)*) : Input video latents.

timestep (*torch.Tensor*) : Source (noisier) flow-map timestep *t*.

r_timestep (*torch.Tensor*) : Target (cleaner) flow-map timestep *r*; defines the destination of the flow-map step.

encoder_hidden_states (*torch.Tensor* of shape *(batch_size, sequence_len, embed_dims)*) : Text-conditioning embeddings.

encoder_hidden_states_image (*torch.Tensor*, *optional*) : Image-conditioning embeddings; concatenated before the text tokens when provided.

attention_kwargs (*dict*, *optional*) : Kwargs forwarded to the *AttentionProcessor* as defined under *self.processor* in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

return_dict (*bool*, *optional*, defaults to *True*) : Whether to return a [*~models.transformer_2d.Transformer2DModelOutput*] instead of a plain tuple.

**Returns:**

[*~models.transformer_2d.Transformer2DModelOutput*] if *return_dict* is True, otherwise a *tuple* whose
first element is the predicted velocity tensor.

Bidirectional flow-map forward pass. `hidden_states` is laid out as `(B, F, C, H, W)` (per-frame latents).
The input is patchified with the standard `patch_embedding` (kernel = stride = `patch_size`) and denoised
with global bidirectional self-attention over the resulting flat token sequence.
