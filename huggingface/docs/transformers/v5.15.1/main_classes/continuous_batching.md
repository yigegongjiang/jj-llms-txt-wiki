# Continuous batching

This page documents the classes behind continuous batching inference: submitting prompts, configuring scheduling and memory limits, and retrieving results.

For usage examples, see the [Continuous batching](../continuous_batching) guide and for how scheduling and memory interact, see the [Continuous batching architecture](../continuous_batching_architecture) doc.

## ContinuousMixin.generate_batch[[transformers.ContinuousMixin.generate_batch]]

#### transformers.ContinuousMixin.generate_batch[[transformers.ContinuousMixin.generate_batch]]

```python
transformers.ContinuousMixin.generate_batch(inputs: list, generation_config: transformers.generation.configuration_utils.GenerationConfig | None = None, continuous_batching_config: transformers.generation.configuration_utils.ContinuousBatchingConfig | None = None, record_timestamps: bool = False, progress_bar: bool = True, persistent_manager: bool = False, warmup: bool = True, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L1201)

**Parameters:**

inputs : List of input token sequences (prompts)

generation_config : Optional generation configuration

continuous_batching_config : Optional continuous batching configuration

record_timestamps : If set to true, the requests will have a timestamp for each token generated

progress_bar : If set to true, a progress bar will be displayed

persistent_manager : whether to persist the manager after the generation is finished. Default is False.

warmup : whether to pre-capture CUDA graphs before processing requests. Default is True.

**Returns:** `dict[str, GenerationOutput]`

a dictionary of request ids to GenerationOutput objects

Generate sequences for a batch of prompts using continuous batching.

## ContinuousBatchingManager[[transformers.ContinuousBatchingManager]]

#### transformers.ContinuousBatchingManager[[transformers.ContinuousBatchingManager]]

```python
transformers.ContinuousBatchingManager(model: ProtoPretrainedModel, generation_config: GenerationConfig, continuous_batching_config: ContinuousBatchingConfig, workload_hints: transformers.generation.continuous_batching.utils.WorkloadHints | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L553)

Manager for handling continuous batching of generation requests. It provides a user interface for submitting
generation requests, retrieving results, and managing the background generation thread. This class should not be
created directly, but through one of the following entry points (all methods of the `ContinuousMixin` mixin):
- `init_continuous_batching`
- `continuous_batching_context_manager`
- `generate_batch`

#### add_request[[transformers.ContinuousBatchingManager.add_request]]

```python
add_request(input_ids: list, request_id: str | None = None, max_new_tokens: int | None = None, streaming: bool = False, record_timestamps: bool = False, eos_token_id: int | list[int] | None = None, **logit_processor_kwargs: typing.Any)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L763)

**Parameters:**

input_ids : Input token IDs to use as prompt

request_id : Optional custom request ID (auto-generated if None)

max_new_tokens : Maximum number of new tokens to generate

streaming : Whether to stream tokens as they're generated

record_timestamps : Whether to record timestamps for each generated token

eos_token_id : End-of-sequence token ID(s)

logit_processor_kwargs : Keyword arguments for the logits processor.

**Returns:** `str | None`

The request ID if the process is a TP driver, None otherwise.

Add a new generation request to the queue. If the process is not a TP driver, this is a no-op.

#### add_requests[[transformers.ContinuousBatchingManager.add_requests]]

```python
add_requests(inputs: list, max_new_tokens: int | None = None, streaming: bool = False, record_timestamps: bool = False, **logit_processor_kwargs: typing.Any)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L820)

Utility function to batch `add_request` and return their IDs. Check its documentation for more details.

#### cancel_request[[transformers.ContinuousBatchingManager.cancel_request]]

```python
cancel_request(request_id: str)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L855)

Cancel a request by its ID. If this called from a process that is not a TP driver, it's a no-op: only TP
driver processes interact with the manager.

#### destroy[[transformers.ContinuousBatchingManager.destroy]]

```python
destroy()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L754)

Terminate the manager and release distributed resources. Safe to call multiple times. After calling this,
the manager cannot be restarted.

#### get_result[[transformers.ContinuousBatchingManager.get_result]]

```python
get_result(request_id: str | None = None, timeout: float | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L864)

Retrieve one result from the output queue. If an ID is provided, returns the first matching request. If a
timeout is provided, returns None after the timeout (in seconds).

#### is_running[[transformers.ContinuousBatchingManager.is_running]]

```python
is_running()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L675)

Returns True if the background generation thread has been started and is still alive.

#### join[[transformers.ContinuousBatchingManager.join]]

```python
join(stop_trigger_time: float, timeout: float | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L739)

Wait for the background thread to finish. Wait can be capped using the timeout argument (in seconds).

#### register_result_handler[[transformers.ContinuousBatchingManager.register_result_handler]]

```python
register_result_handler(request_id: str, callback: Callable)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L898)

**Parameters:**

request_id (*str*) : The request ID to receive outputs for.

callback (*callable*) : Called with a `GenerationOutput` for each result.

Register a callback for result delivery (streaming or non-streaming).

The callback is invoked on the event loop via `call_soon_threadsafe` each time a result is produced for this
request. For streaming requests, this happens on every token; for non-streaming, only on completion. The handler
is automatically cleaned up when the request finishes.

#### request_id_iter[[transformers.ContinuousBatchingManager.request_id_iter]]

```python
request_id_iter(request_id: str)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L885)

Iterate over results matching a specific request id (blocking).

Uses the shared output queue with requeue. For high-concurrency serving,
use `register_result_handler` instead.

#### start[[transformers.ContinuousBatchingManager.start]]

```python
start()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L679)

Start the background generation thread.

#### stop[[transformers.ContinuousBatchingManager.stop]]

```python
stop(block: bool = True, timeout: float | None = None, keep_for_next_session: bool = False, hard_stop: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L689)

Stop the background generation thread. If the `block` flag is set to True, then this method waits for the
thread to stop for a maximum time of `timeout` seconds (None means no timeout). If the `keep_for_next_session`
flag is set to True, then the manager is cached on the model for future use. If the `hard_stop` flag is set,
the background generation thread will be stopped immediately and pending requests will be failed.

#### switch_to_cb_friendly_attn[[transformers.ContinuousBatchingManager.switch_to_cb_friendly_attn]]

```python
switch_to_cb_friendly_attn(model: ProtoPretrainedModel)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L631)

Switch the attn implementation to one that is CB friendly: try to find a flash implementation if flash is
requested and, in any cases, switch to a paged implementation.

#### warmup[[transformers.ContinuousBatchingManager.warmup]]

```python
warmup()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L665)

Pre-capture CUDA graphs for varlen and decode paths by running dummy batches. Initializes the batch
processor if not already done.

## Continuous batching config[[transformers.ContinuousBatchingConfig]]

#### transformers.ContinuousBatchingConfig[[transformers.ContinuousBatchingConfig]]

```python
transformers.ContinuousBatchingConfig(block_size: int = 256, num_blocks: int | None = None, max_batch_tokens: int | None = None, max_memory_percent: float | None = None, max_requests_per_batch: int | None = None, max_blocks_per_request: int | None = None, allow_block_sharing: bool = True, use_async_batching: bool | None = None, use_cuda_graph: bool | tuple[bool, bool] | None = None, q_padding_interval_size: int = 0, kv_padding_interval_size: int = 0, varlen_compile_config: transformers.generation.configuration_utils.CompileConfig | None = None, decode_compile_config: transformers.generation.configuration_utils.CompileConfig | None = None, default_compile_level: int = 0, scheduler_type: str = 'fifo', safety_margin: float | None = None, return_logprobs: bool = False, seed: int | None = None, cpu_offload_space: float | None = 0.0, cpu_offload_space_safety_threshold: float = 0.8, max_queue_size: int = 0, per_request_processors: bool = False, drop_unsupported_processors: bool = True, disable_nccl_graph_mixing: bool = True, cpu_group_timeout: float | None = 300.0, use_default_compile_configs: bool | None = None, max_cached_graphs: int | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/configuration_utils.py#L1654)

**Parameters:**

block_size (`int`, *optional*, defaults to 256) : Size of each KV cache block in tokens.

num_blocks (`int`, *optional*) : Number of blocks in the KV cache. Auto-inferred from GPU memory when `None`.

max_batch_tokens (`int`, *optional*) : Maximum number of tokens in a batch. Auto-inferred from GPU memory when `None`.

max_memory_percent (`float`, *optional*) : Maximum percentage of free GPU memory (after the model is loaded) to use for the KV cache. When `None`, resolved at runtime to 0.9 if there is no logit processing and 0.8 if there is, to leave headroom for vocabulary-sized temporary tensors.

max_requests_per_batch (`int`, *optional*) : Maximum number of requests per batch. Auto-inferred from workload hints when `None`, with fallback of 1024.

max_blocks_per_request (`int`, *optional*) : Maximum blocks per request, used in the `flash_attn_with_kvcache` fast decode path to dimension the block table. Setting this to 0 disables the fast decode path. Default is None (auto-inferred).

allow_block_sharing (`bool`, *optional*, defaults to `True`) : Whether to allow block sharing for prefix caching. Block sharing can only be allowed, never forced, as some models do not support it. Disable if you have few short prompts but long generation lengths.

use_async_batching (`bool`, *optional*) : Whether to enable async double-buffering, which removes CPU overhead from the continuous batching loop at the cost of doubled VRAM usage. Auto-detected when `None`.

use_cuda_graph (`bool` or `tuple[bool, bool]`, *optional*) : Whether to enable CUDA graphs. This can be a tuple of booleans (one for the varlen path and one for the decode fast path), a boolean which will apply to both paths, or None (automatically inferred). After calling `decide_use_cuda_graphs`, the attribute will be a tuple of booleans. Default is None (automatically inferred).

q_padding_interval_size (`int`, *optional*, defaults to 0) : Query padding granularity in tokens for CUDA graphs. Uses a preset from `continuous_api.py` when set to 0.

kv_padding_interval_size (`int`, *optional*, defaults to 0) : KV padding granularity in tokens for CUDA graphs. Uses a preset from `continuous_api.py` when set to 0.

varlen_compile_config (`CompileConfig`, *optional*) : CompileConfig for varlen (prefill) path. Default is None (uses generation_config fallback) The varlen path handles batches with varying query and KV lengths, often benefiting from dynamic=True.

decode_compile_config (`CompileConfig`, *optional*) : CompileConfig for decode (fast) path. Default is None (uses generation_config fallback) The decode path handles batches has no dynamic KV length, so static shapes are a better fit.

default_compile_level (`int`, *optional*, defaults to 0) : If this is >0 and no compile config is provided for varlen or decode path, a default compile config will be provided. The level can go up to 3, and a higher level means more performance but longer warmup time.

scheduler_type (`str`, *optional*, defaults to `"fifo"`) : Scheduler type to use.

safety_margin (`float`, *optional*) : Safety margin used to limit the amount of offloading. Defaults to None (use class default).

return_logprobs (`bool`, *optional*, defaults to `False`) : Whether to return log probabilities along with the generated tokens.

seed (`int | None`, *optional*) : An optional seed for generation. If not specified, the internal seed will be set to a random value.

cpu_offload_space (`float`, *optional*, defaults to 0.0) : CPU swap space in GiB for KV cache offloading. A pre-allocated pinned CPU buffer of this size is created at initialization. When the GPU cache is full, evicted requests' KV caches are copied here instead of being discarded. 0 disables offloading (default).

cpu_offload_space_safety_threshold (`float`, *optional*, defaults to 0.8) : If `cpu_offload_space` exceeds this fraction of total system RAM, it is clamped to avoid host OOM. Set to 1.0 to disable the safety cap. Ignored when psutil is not available.

max_queue_size (`int`, *optional*, defaults to 0) : Maximum request queue size for serving. 0 means unlimited.

per_request_processors (`bool`, *optional*, defaults to `False`) : Enable per-request logits processor parameters. Default is False.

drop_unsupported_processors (`bool`, *optional*, defaults to `True`) : Remove unsupported logits processors instead of erroring. Default is True.

disable_nccl_graph_mixing (`bool`, *optional*, defaults to `True`) : Disable NCCL's safety net for parallel graph-captured comms. Never happens in CB and gives TP a perf boost.

cpu_group_timeout (`float`, *optional*, defaults to 300.0) : The time (in seconds) after which a CPU communication will timeout and the process will crash. Leave to None for no timeout. Default is 300 seconds.

use_default_compile_configs (`bool | None`, *optional*) : Deprecated in 5.11: please use default_compile_level instead.

max_cached_graphs (`int`, *optional*) : Deprecated in 5.13: maximum number of graph is no longer an issue.

Class that holds arguments relative to continuous batching, when using continuous batching through the
`generate_batch` method or the `continuous_batching_context_manager` context manager.
