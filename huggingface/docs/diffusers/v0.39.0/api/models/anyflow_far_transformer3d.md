# AnyFlowFARTransformer3DModel

The causal (FAR) 3D Transformer used by [`AnyFlowFARPipeline`](../pipelines/anyflow#anyflowfarpipeline) —
the FAR variant of [AnyFlow](https://huggingface.co/papers/2605.13724). See the
[`AnyFlowFARPipeline`](../pipelines/anyflow) page for paper, authors, and released checkpoints. It extends
the v0.35.1 Wan2.1 backbone with three additions:

1. **FAR causal block-mask** via `torch.nn.attention.flex_attention`, supporting chunk-wise autoregressive
   generation as introduced in [FAR](https://huggingface.co/papers/2503.19325).
2. **Compressed-frame patch embedding** (`far_patch_embedding`) for context (already-generated) frames,
   warm-started from the full-resolution `patch_embedding` at construction time via trilinear interpolation.
3. **Dual-timestep flow-map embedding** (same as
   [`AnyFlowTransformer3DModel`](anyflow_transformer3d)) — every forward call conditions on both the source
   timestep ``t`` and the target timestep ``r``.

The default chunk schedule (`chunk_partition`) is stored in the model config; the released NVIDIA AnyFlow-FAR
checkpoints use `[1, 3, 3, 3, 3, 3, 3, 2]` for the canonical 81-frame setting. `forward` accepts a per-call
`chunk_partition` override, so the same checkpoint also handles other `num_frames` configurations without
retraining.

```python
from diffusers import AnyFlowFARTransformer3DModel

# Causal AnyFlow checkpoint (FAR):
transformer = AnyFlowFARTransformer3DModel.from_pretrained(
    "nvidia/AnyFlow-FAR-Wan2.1-1.3B-Diffusers", subfolder="transformer"
)
```

## AnyFlowFARTransformer3DModel[[diffusers.AnyFlowFARTransformer3DModel]]

#### diffusers.AnyFlowFARTransformer3DModel[[diffusers.AnyFlowFARTransformer3DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_anyflow_far.py#L961)

Causal (FAR) 3D Transformer for AnyFlow flow-map sampling with chunk-wise autoregressive generation.

Extends the v0.35.1 Wan2.1 backbone with:

- **FAR causal block-mask** via `torch.nn.attention.flex_attention`, supporting chunk-wise autoregressive
  generation ([FAR](https://huggingface.co/papers/2503.19325)).
- **Compressed-frame patch embedding** `far_patch_embedding` for context (already-generated) frames, initialized
  from `patch_embedding` via trilinear interpolation so a freshly constructed model is already at a reasonable
  starting point even before LoRA fine-tuning.
- **Dual-timestep flow-map embedding** for any-step sampling (same as `AnyFlowTransformer3DModel`).

Use `AnyFlowTransformer3DModel` instead for plain bidirectional T2V — that variant skips the FAR causal masking
and `far_patch_embedding` and is ~5–10% smaller.

build_attention_maskdiffusers.AnyFlowFARTransformer3DModel.build_attention_maskhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_anyflow_far.py#L1232[{"name": "chunk_partition", "val": ": typing.List[int]"}, {"name": "height", "val": ": int"}, {"name": "width", "val": ": int"}, {"name": "has_clean_context", "val": ": bool = False"}, {"name": "device", "val": ": typing.Optional[torch.device] = None"}, {"name": "mode", "val": ": str = 'train'"}]- **chunk_partition** -- per-chunk frame counts (must sum to the number of latent frames).
- **height,** width -- latent spatial dimensions.
- **has_clean_context** -- `True` when `clean_hidden_states` will be threaded through `forward`
  (training V2V/I2V); only this presence flag affects the mask layout.
- **device** -- device for the resulting `BlockMask`. The mask is not auto-moved by
  `device_map="auto"`; build it on the same device the transformer's inputs will live on.
- **mode** -- `"train"` (matches `_forward_train`) or `"cache"` (matches `_forward_cache`).
  The autoregressive `_forward_inference` path attends through the KV cache and has no mode here.0`~torch.nn.attention.flex_attention.BlockMask`causal mask spanning the FAR layout, padded to a
multiple of 128 along the sequence dimension (the BlockMask block-size requirement).- `ValueError` -- if `mode` is neither `"train"` nor `"cache"`.`ValueError`
Pre-build the causal `~torch.nn.attention.flex_attention.BlockMask` outside `forward`.

Pass the result via `forward`'s `attention_mask` kwarg to make the whole transformer compatible with
`torch.compile(fullgraph=True)`. Without a pre-built mask, `forward` falls back to constructing it
internally — that path uses `flex_attention.create_block_mask(_compile=False)` and breaks the compile graph.

**Parameters:**

patch_size (*Tuple[int]*, defaults to *(1, 2, 2)*) : 3D patch dimensions for full-resolution chunks.

compressed_patch_size (*Tuple[int]*, defaults to *(1, 4, 4)*) : Larger patch dimensions for the FAR-compressed (context) chunks.

full_chunk_limit (*int*, defaults to *3*) : Maximum number of full-resolution chunks before earlier chunks are demoted to compressed FAR context. The released checkpoints use `3`.

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

image_dim (*Optional[int]*, *optional*, defaults to *None*) : Image embedding dimension for I2V conditioning.

rope_max_seq_len (*int*, defaults to *1024*) : Maximum sequence length used to precompute rotary position frequencies.

gate_value (*float*, defaults to *0.25*) : Mixing gate between source-timestep and delta-timestep embeddings.

deltatime_type (*str*, defaults to *'r'*) : Either `"r"` (delta is the target timestep) or `"t-r"` (delta is the absolute interval).

chunk_partition (*Tuple[int, ...]*, defaults to *(1, 3, 3, 3, 3, 3, 3, 2)*) : Default per-chunk frame counts used by the pipeline. The released NVIDIA AnyFlow-FAR checkpoints target `num_frames=81` (21 latent frames at VAE temporal stride 4) split as `1 + 3*6 + 2`. A different `num_frames` requires a matching `chunk_partition` override passed to [AnyFlowFARPipeline.__call__()](/docs/diffusers/v0.39.0/en/api/pipelines/anyflow#diffusers.AnyFlowFARPipeline.__call__) (and likewise to `forward`).

**Returns:**

``~torch.nn.attention.flex_attention.BlockMask``

causal mask spanning the FAR layout, padded to a
multiple of 128 along the sequence dimension (the BlockMask block-size requirement).
#### forward[[diffusers.AnyFlowFARTransformer3DModel.forward]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_anyflow_far.py#L1098)

FAR causal forward pass. Dispatches to one of three internal paths:

- `kv_cache is None` → causal training rollout (returns `Transformer2DModelOutput`).
- `kv_cache is not None` and `kv_cache_flag["is_cache_step"]` → cache-prefill (returns
  `AnyFlowFARTransformerOutput` with `sample=None`).
- Otherwise → autoregressive inference step (returns `AnyFlowFARTransformerOutput`).

**Parameters:**

hidden_states (*torch.Tensor*) : Latent input of shape `(B, F, C, H, W)`.

timestep (*torch.Tensor*) : Source (noisier) flow-map timestep *t*.

r_timestep (*torch.Tensor*) : Target (cleaner) flow-map timestep *r*.

encoder_hidden_states (*torch.Tensor*) : UMT5 text embeddings.

chunk_partition (*List[int]*) : Per-chunk frame counts; total must match the number of latent frames in `hidden_states`.

encoder_hidden_states_image (*torch.Tensor*, *optional*) : I2V image embedding; concatenated before text tokens when provided.

clean_hidden_states (*torch.Tensor*, *optional*) : Clean (noise-free) conditioning frames used by the training rollout.

clean_timestep (*torch.Tensor*, *optional*) : Timesteps for the clean conditioning frames in the training rollout.

kv_cache (*List[Dict[str, torch.Tensor]]*, *optional*) : Per-block KV cache for autoregressive inference. *None* selects the training path.

kv_cache_flag (*Dict[str, Any]*, *optional*) : KV-cache metadata (e.g. `is_cache_step` flag and token counts).

attention_mask (*BlockMask*, *optional*) : Pre-built causal mask, typically constructed via `build_attention_mask`. Consumed by the train and KV-cache prefill paths; the autoregressive inference path attends through the KV cache and does not use a full mask. When `None`, the train / cache paths build the mask internally; that fallback is not compile-safe (the underlying `flex_attention.create_block_mask` breaks the graph under `fullgraph=True`), so pass a pre-built mask whenever wrapping `forward` in `torch.compile`.

attention_kwargs (*dict*, *optional*) : Forwarded to the attention processors.

return_dict (*bool*, *optional*, defaults to *True*) : If *False*, returns positional tuples instead of an output dataclass.

**Returns:**

`[*~models.transformer_2d.Transformer2DModelOutput*], [*AnyFlowFARTransformerOutput*] or *tuple*`

When *return_dict* is *False*, a plain *tuple* is returned. Otherwise, the causal training rollout
(*kv_cache is None*) returns a [*~models.transformer_2d.Transformer2DModelOutput*], while the
cache-prefill and autoregressive inference paths return an [*AnyFlowFARTransformerOutput*].

## AnyFlowFARTransformerOutput[[diffusers.models.transformers.transformer_anyflow_far.AnyFlowFARTransformerOutput]]

#### diffusers.models.transformers.transformer_anyflow_far.AnyFlowFARTransformerOutput[[diffusers.models.transformers.transformer_anyflow_far.AnyFlowFARTransformerOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_anyflow_far.py#L55)

Output dataclass for `AnyFlowFARTransformer3DModel`'s causal forward paths.

**Parameters:**

sample (*torch.Tensor* or *None*) : Predicted denoising target for the autoregressive chunk. `None` for the cache-prefill path, which only writes the KV cache and produces no usable sample.

kv_cache (*list[dict[str, torch.Tensor]]*, *optional*) : Per-block KV cache state used by subsequent autoregressive steps.
