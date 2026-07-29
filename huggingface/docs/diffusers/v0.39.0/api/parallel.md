# Parallelism

Parallelism strategies help speed up diffusion transformers by distributing computations across multiple devices, allowing for faster inference/training times. Refer to the [Distributed inferece](../training/distributed_inference) guide to learn more.

## ParallelConfig[[diffusers.ParallelConfig]]

#### diffusers.ParallelConfig[[diffusers.ParallelConfig]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/_modeling_parallel.py#L158)

Configuration for applying different parallelisms.

**Parameters:**

context_parallel_config (`ContextParallelConfig`, *optional*) : Configuration for context parallelism.

## ContextParallelConfig[[diffusers.ContextParallelConfig]]

#### diffusers.ContextParallelConfig[[diffusers.ContextParallelConfig]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/_modeling_parallel.py#L42)

Configuration for context parallelism.

**Parameters:**

ring_degree (`int`, *optional*, defaults to `1`) : Number of devices to use for Ring Attention. Sequence is split across devices. Each device computes attention between its local Q and KV chunks passed sequentially around ring. Lower memory (only holds 1/N of KV at a time), overlaps compute with communication, but requires N iterations to see all tokens. Best for long sequences with limited memory/bandwidth. Number of devices to use for ring attention within a context parallel region. Must be a divisor of the total number of devices in the context parallel mesh.

ulysses_degree (`int`, *optional*, defaults to `1`) : Number of devices to use for Ulysses Attention. Sequence split is across devices. Each device computes local QKV, then all-gathers all KV chunks to compute full attention in one pass. Higher memory (stores all KV), requires high-bandwidth all-to-all communication, but lower latency. Best for moderate sequences with good interconnect bandwidth.

convert_to_fp32 (`bool`, *optional*, defaults to `True`) : Whether to convert output and LSE to float32 for ring attention numerical stability.

rotate_method (`str`, *optional*, defaults to `"allgather"`) : Method to use for rotating key/value states across devices in ring attention. Currently, only `"allgather"` is supported.

ulysses_anything (`bool`, *optional*, defaults to `False`) : Whether to enable "Ulysses Anything" mode, which supports arbitrary sequence lengths and head counts that are not evenly divisible by `ulysses_degree`. When enabled, `ulysses_degree` must be greater than 1 and `ring_degree` must be 1.

ring_anything (`bool`, *optional*, defaults to `False`) : Whether to enable "Ring Anything" mode, which supports arbitrary sequence lengths. When enabled, `ring_degree` must be greater than 1 and `ulysses_degree` must be 1.

mesh (`torch.distributed.device_mesh.DeviceMesh`, *optional*) : A custom device mesh to use for context parallelism. If provided, this mesh will be used instead of creating a new one. This is useful when combining context parallelism with other parallelism strategies (e.g., FSDP, tensor parallelism) that share the same device mesh. The mesh must have both "ring" and "ulysses" dimensions. Use size 1 for dimensions not being used (e.g., `mesh_shape=(2, 1, 4)` with `mesh_dim_names=("ring", "ulysses", "fsdp")` for ring attention only with FSDP).

#### diffusers.hooks.apply_context_parallel[[diffusers.hooks.apply_context_parallel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/hooks/context_parallel.py#L80)

Apply context parallel on a model.
