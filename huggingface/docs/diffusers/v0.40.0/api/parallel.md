# Parallelism

Parallelism strategies help speed up diffusion transformers by distributing computations across multiple devices, allowing for faster inference/training times. Refer to the [Distributed inferece](../training/distributed_inference) guide to learn more.

## ParallelConfig[[diffusers.ParallelConfig]]

#### diffusers.ParallelConfig[[diffusers.ParallelConfig]]

```python
diffusers.ParallelConfig(context_parallel_config: diffusers.models._modeling_parallel.ContextParallelConfig | None = None, tensor_parallel_config: diffusers.models._modeling_parallel.TensorParallelConfig | None = None, _rank: int = None, _world_size: int = None, _device: device = None, _mesh: DeviceMesh = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/_modeling_parallel.py#L197)

**Parameters:**

context_parallel_config (`ContextParallelConfig`, *optional*) : Configuration for context parallelism.

tensor_parallel_config (`TensorParallelConfig`, *optional*) : Configuration for tensor parallelism.

Configuration for applying different parallelisms.

## ContextParallelConfig[[diffusers.ContextParallelConfig]]

#### diffusers.ContextParallelConfig[[diffusers.ContextParallelConfig]]

```python
diffusers.ContextParallelConfig(ring_degree: int | None = None, ulysses_degree: int | None = None, convert_to_fp32: bool = True, rotate_method: typing.Literal['allgather', 'alltoall'] = 'allgather', mesh: typing.Optional[torch.distributed.device_mesh.DeviceMesh] = None, ulysses_anything: bool = False, ring_anything: bool = False, _rank: int = None, _world_size: int = None, _device: device = None, _mesh: DeviceMesh = None, _flattened_mesh: DeviceMesh = None, _ring_mesh: DeviceMesh = None, _ulysses_mesh: DeviceMesh = None, _ring_local_rank: int = None, _ulysses_local_rank: int = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/_modeling_parallel.py#L41)

**Parameters:**

ring_degree (`int`, *optional*, defaults to `1`) : Number of devices to use for Ring Attention. Sequence is split across devices. Each device computes attention between its local Q and KV chunks passed sequentially around ring. Lower memory (only holds 1/N of KV at a time), overlaps compute with communication, but requires N iterations to see all tokens. Best for long sequences with limited memory/bandwidth. Number of devices to use for ring attention within a context parallel region. Must be a divisor of the total number of devices in the context parallel mesh.

ulysses_degree (`int`, *optional*, defaults to `1`) : Number of devices to use for Ulysses Attention. Sequence split is across devices. Each device computes local QKV, then all-gathers all KV chunks to compute full attention in one pass. Higher memory (stores all KV), requires high-bandwidth all-to-all communication, but lower latency. Best for moderate sequences with good interconnect bandwidth.

convert_to_fp32 (`bool`, *optional*, defaults to `True`) : Whether to convert output and LSE to float32 for ring attention numerical stability.

rotate_method (`str`, *optional*, defaults to `"allgather"`) : Method to use for rotating key/value states across devices in ring attention. Currently, only `"allgather"` is supported.

ulysses_anything (`bool`, *optional*, defaults to `False`) : Whether to enable "Ulysses Anything" mode, which supports arbitrary sequence lengths and head counts that are not evenly divisible by `ulysses_degree`. When enabled, `ulysses_degree` must be greater than 1 and `ring_degree` must be 1.

ring_anything (`bool`, *optional*, defaults to `False`) : Whether to enable "Ring Anything" mode, which supports arbitrary sequence lengths. When enabled, `ring_degree` must be greater than 1 and `ulysses_degree` must be 1.

mesh (`torch.distributed.device_mesh.DeviceMesh`, *optional*) : A custom device mesh to use for context parallelism. If provided, this mesh will be used instead of creating a new one. This is useful when combining context parallelism with other parallelism strategies (e.g., FSDP, tensor parallelism) that share the same device mesh. The mesh must have both "ring" and "ulysses" dimensions. Use size 1 for dimensions not being used (e.g., `mesh_shape=(2, 1, 4)` with `mesh_dim_names=("ring", "ulysses", "fsdp")` for ring attention only with FSDP).

Configuration for context parallelism.

#### diffusers.hooks.apply_context_parallel[[diffusers.hooks.apply_context_parallel]]

```python
diffusers.hooks.apply_context_parallel(module: Module, parallel_config: ContextParallelConfig, plan: dict)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/hooks/context_parallel.py#L80)

Apply context parallel on a model.

## TensorParallelConfig[[diffusers.TensorParallelConfig]]

#### diffusers.TensorParallelConfig[[diffusers.TensorParallelConfig]]

```python
diffusers.TensorParallelConfig(tp_degree: int = 1, mesh: typing.Optional[torch.distributed.device_mesh.DeviceMesh] = None, _rank: int = None, _world_size: int = None, _device: device = None, _mesh: DeviceMesh = None, _tp_degree: int = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/_modeling_parallel.py#L157)

**Parameters:**

tp_degree (`int`, defaults to `1`) : Number of devices to shard across. Must be a divisor of the number of attention heads (and FFN hidden dimensions) of the model being parallelised.

mesh (`torch.distributed.device_mesh.DeviceMesh`, *optional*) : A custom device mesh to use. If provided, `tp_degree` is inferred from `mesh.size()` and the argument is ignored. Useful when combining TP with other parallelism strategies (e.g. CP) that share the same mesh.

Configuration for tensor parallelism.

Tensor parallelism shards weight matrices (column-wise and row-wise) across devices. Each device computes a partial
result; an AllReduce/AllGather at layer boundaries reconstructs the full output. Uses
`torch.distributed.tensor.parallelize_module` with `ColwiseParallel` / `RowwiseParallel` sharding styles. Supported
device types are `"cuda"` and `"neuron"`.

#### diffusers.hooks.apply_tensor_parallel[[diffusers.hooks.apply_tensor_parallel]]

```python
diffusers.hooks.apply_tensor_parallel(model: Module, config: TensorParallelConfig, tp_plan: dict)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/hooks/tensor_parallel.py#L243)

Apply tensor parallel on a model from its flat `_tp_plan`.
