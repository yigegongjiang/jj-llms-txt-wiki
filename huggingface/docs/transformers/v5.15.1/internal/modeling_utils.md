# Custom Layers and Utilities

This page lists all the custom layers used by the library, as well as the utility functions and classes it provides for modeling.

Most of those are only useful if you are studying the code of the models in the library.

## WeightRenaming[[transformers.GroupWeightRename]]

#### transformers.GroupWeightRename[[transformers.GroupWeightRename]]

```python
transformers.GroupWeightRename(source_patterns: list[str], target_patterns: list[str])
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/core_model_loading.py#L1018)

Applies a list of paired WeightRenaming transforms, activated lazily by the first ("guard")
source pattern.  Use this when two renames share an intermediate name (e.g. `norm0→norm1`
and `norm1→norm2`) so that loading an already-converted checkpoint (which has `norm1`
and `norm2` but no `norm0`) does not incorrectly re-apply the renames.

NOTE: order `source_patterns` so that the one that is absent in an already-converted checkpoint
comes first.  The state dict is iterated in sorted key order, so the guard pattern must be
lexicographically smaller than the dependent patterns. Otherwise the dependents will be
skipped on the first pass and never retried.

## WeightConverter[[transformers.WeightConverter]]

#### transformers.WeightConverter[[transformers.WeightConverter]]

```python
transformers.WeightConverter(source_patterns: str | list[str], target_patterns: str | list[str], operations: list[ConversionOps])
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/core_model_loading.py#L1138)

### Conversion operations[[transformers.ConversionOps]]

#### transformers.ConversionOps[[transformers.ConversionOps]]

```python
transformers.ConversionOps()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/core_model_loading.py#L83)

Base class for weight conversion operations.

#### transformers.Chunk[[transformers.Chunk]]

```python
transformers.Chunk(dim: int = 0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/core_model_loading.py#L114)

Split a tensor along `dim` into equally sized chunks.

#### transformers.Concatenate[[transformers.Concatenate]]

```python
transformers.Concatenate(dim: int = 0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/core_model_loading.py#L138)

Concatenate tensors along `dim`.

#### transformers.MergeModulelist[[transformers.MergeModulelist]]

```python
transformers.MergeModulelist(dim: int = 0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/core_model_loading.py#L208)

Merge a list of tensors into a single tensor along the first dimension.
We explicitly define this because for EP or TP you want to make sure you know what you are doing!

#### transformers.SplitModulelist[[transformers.SplitModulelist]]

```python
transformers.SplitModulelist(dim: int = 0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/core_model_loading.py#L259)

Inverse of `MergeModulelist` using explicit split sizes per group.

#### transformers.PermuteForRope[[transformers.PermuteForRope]]

```python
transformers.PermuteForRope(subconfig_key: str | None = None, permute_layer_names: list[str] | None = None, inverse: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/core_model_loading.py#L414)

Applies the permutation required to convert complex RoPE weights to the split sin/cos format.

#### transformers.VisionFuseAndPermuteForRope[[transformers.VisionFuseAndPermuteForRope]]

```python
transformers.VisionFuseAndPermuteForRope(dim: int = 0, permute_layer_names: list[str] | None = None, inverse: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/core_model_loading.py#L474)

Applies the permutation required to convert complex RoPE weights to the split sin/cos format on fused QKV.
Same as calling `PermuteForRope() + Concatenate()` but lets us call `Permute` only on a subset of chunked tensors.

NOTE: this conversion applies only to a vision backbone in multimodal models, because it checks `config.vision_config`

#### transformers.VisionUnfuseAndPermuteForRope[[transformers.VisionUnfuseAndPermuteForRope]]

```python
transformers.VisionUnfuseAndPermuteForRope(dim: int = 0, permute_layer_names: list[str] | None = None, inverse: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/core_model_loading.py#L528)

Applies the permutation required to convert complex RoPE weights to the split sin/cos format on fused QKV.
Same as calling `Chunk() + PermuteForRope()` but lets us call `Permute` only on a subset of chunked tensors.

NOTE: this conversion applies only to a vision backbone in multimodal models, because it checks `config.vision_config`

## Layers[[transformers.GradientCheckpointingLayer]]

#### transformers.GradientCheckpointingLayer[[transformers.GradientCheckpointingLayer]]

```python
transformers.GradientCheckpointingLayer(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_layers.py#L51)

Base class for layers with gradient checkpointing.

This class enables gradient checkpointing functionality for a layer. By default, gradient checkpointing is disabled
(`gradient_checkpointing = False`). When `model.set_gradient_checkpointing()` is called, gradient checkpointing is
enabled by setting `gradient_checkpointing = True` and assigning a checkpointing function to `_gradient_checkpointing_func`.

Important:

When using gradient checkpointing with `use_reentrant=True`, inputs that require gradients (e.g. hidden states)
must be passed as positional arguments (`*args`) rather than keyword arguments to properly propagate gradients.

Example:

```python
>>> # Correct - hidden_states passed as positional arg
>>> out = self.layer(hidden_states, attention_mask=attention_mask)

>>> # Incorrect - hidden_states passed as keyword arg
>>> out = self.layer(hidden_states=hidden_states, attention_mask=attention_mask)
```

## Attention Functions[[transformers.AttentionInterface]]

#### transformers.AttentionInterface[[transformers.AttentionInterface]]

```python
transformers.AttentionInterface()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_utils.py#L5093)

Dict-like object keeping track of allowed attention functions. You can easily add a new attention function
with a call to `register()`. If a model needs to locally overwrite an existing attention function, say `sdpa`,
it needs to declare a new instance of this class inside the `modeling_<model>.py`, and declare it on that instance.

#### register[[transformers.AttentionInterface.register]]

```python
register(key: str, value: Callable)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/utils/generic.py#L1130)

## Attention Mask Functions[[transformers.AttentionMaskInterface]]

#### transformers.AttentionMaskInterface[[transformers.AttentionMaskInterface]]

```python
transformers.AttentionMaskInterface()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/masking_utils.py#L711)

#### register[[transformers.AttentionMaskInterface.register]]

```python
register(key: str, value: Callable)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/utils/generic.py#L1130)

## Rotary Position Embedding Functions[[transformers.dynamic_rope_update]]

#### transformers.dynamic_rope_update[[transformers.dynamic_rope_update]]

```python
transformers.dynamic_rope_update(rope_forward)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_rope_utils.py#L34)

**Parameters:**

rope_forward (Callable) : The forward pass of the RoPE implementation.

**Returns:**

The decorated forward pass.

Decorator function to update the RoPE parameters in the forward pass, if the model is using a dynamic RoPE
(i.e. a RoPE implementation that may recompute its frequencies in the forward pass).

## Pytorch custom modules[[transformers.Conv1D]]

#### transformers.Conv1D[[transformers.Conv1D]]

```python
transformers.Conv1D(nf, nx)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/pytorch_utils.py#L95)

**Parameters:**

nf (`int`) : The number of output features.

nx (`int`) : The number of input features.

1D-convolutional layer as defined by Radford et al. for OpenAI GPT (and also used in GPT-2).

Basically works like a linear layer but the weights are transposed.

## PyTorch Helper Functions[[transformers.apply_chunking_to_forward]]

#### transformers.apply_chunking_to_forward[[transformers.apply_chunking_to_forward]]

```python
transformers.apply_chunking_to_forward(forward_fn: Callable[..., torch.Tensor], chunk_size: int, chunk_dim: int, *input_tensors)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/pytorch_utils.py#L124)

**Parameters:**

forward_fn (`Callable[..., torch.Tensor]`) : The forward function of the model.

chunk_size (`int`) : The chunk size of a chunked tensor: `num_chunks = len(input_tensors[0]) / chunk_size`.

chunk_dim (`int`) : The dimension over which the `input_tensors` should be chunked.

input_tensors (`tuple[torch.Tensor]`) : The input tensors of `forward_fn` which will be chunked

**Returns:** `torch.Tensor`

A tensor with the same shape as the `forward_fn` would have given if applied`.

This function chunks the `input_tensors` into smaller input tensor parts of size `chunk_size` over the dimension
`chunk_dim`. It then applies a layer `forward_fn` to each chunk independently to save memory.

If the `forward_fn` is independent across the `chunk_dim` this function will yield the same result as directly
applying `forward_fn` to `input_tensors`.

Examples:

```python
# rename the usual forward() fn to forward_chunk()
def forward_chunk(self, hidden_states):
    hidden_states = self.decoder(hidden_states)
    return hidden_states

# implement a chunked forward function
def forward(self, hidden_states):
    return apply_chunking_to_forward(self.forward_chunk, self.chunk_size_lm_head, self.seq_len_dim, hidden_states)
```

#### transformers.pytorch_utils.prune_linear_layer[[transformers.pytorch_utils.prune_linear_layer]]

```python
transformers.pytorch_utils.prune_linear_layer(layer: nn.Linear, index: torch.LongTensor, dim: int = 0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/pytorch_utils.py#L61)

**Parameters:**

layer (`torch.nn.Linear`) : The layer to prune.

index (`torch.LongTensor`) : The indices to keep in the layer.

dim (`int`, *optional*, defaults to 0) : The dimension on which to keep the indices.

**Returns:** `torch.nn.Linear`

The pruned layer as a new layer with `requires_grad=True`.

Prune a linear layer to keep only entries in index.

Used to remove heads.
