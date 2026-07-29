# LoRA for Neuron

LoRA (Low-Rank Adaptation) implementation optimized for distributed training on AWS Trainium devices. This module provides efficient parameter-efficient fine-tuning with tensor parallelism and sequence parallelism support.

## PEFT Model Classes

### NeuronPeftModel[[optimum.neuron.peft.NeuronPeftModel]]

#### optimum.neuron.peft.NeuronPeftModel[[optimum.neuron.peft.NeuronPeftModel]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/peft/peft_model.py#L82)

### NeuronPeftModelForCausalLM[[optimum.neuron.peft.NeuronPeftModelForCausalLM]]

#### optimum.neuron.peft.NeuronPeftModelForCausalLM[[optimum.neuron.peft.NeuronPeftModelForCausalLM]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/peft/peft_model.py#L463)

## LoRA Layer Implementations

### Base LoRA Layer[[optimum.neuron.peft.tuners.lora.layer.NeuronLoraLayer]]

#### optimum.neuron.peft.tuners.lora.layer.NeuronLoraLayer[[optimum.neuron.peft.tuners.lora.layer.NeuronLoraLayer]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/peft/tuners/lora/layer.py#L81)

### Parallel Linear LoRA[[optimum.neuron.peft.tuners.lora.layer.ParallelLinear]]

#### optimum.neuron.peft.tuners.lora.layer.ParallelLinear[[optimum.neuron.peft.tuners.lora.layer.ParallelLinear]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/peft/tuners/lora/layer.py#L232)

mergeoptimum.neuron.peft.tuners.lora.layer.ParallelLinear.mergehttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/peft/tuners/lora/layer.py#L307[{"name": "safe_merge", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}]- **safe_merge** -- If True, perform merge in a copy and check for NaNs before merging.
- **adapter_names** -- List of adapter names to merge. If None, all active adapters will be merged.0

Merge the active adapter weights into the base weights.

This works with distributed parallel linear layers (RowParallelLinear, ColumnParallelLinear).
The merge happens on the sharded weights - each rank merges its own shard.

**Parameters:**

safe_merge : If True, perform merge in a copy and check for NaNs before merging.

adapter_names : List of adapter names to merge. If None, all active adapters will be merged.
#### unmerge[[optimum.neuron.peft.tuners.lora.layer.ParallelLinear.unmerge]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/peft/tuners/lora/layer.py#L361)

Unmerge all merged adapter layers from the base weights.

This works with distributed parallel linear layers (RowParallelLinear, ColumnParallelLinear).
The unmerge happens on the sharded weights - each rank unmerges its own shard.

### GQA QKV Column Parallel LoRA[[optimum.neuron.peft.tuners.lora.layer.GQAQKVColumnParallelLinear]]

#### optimum.neuron.peft.tuners.lora.layer.GQAQKVColumnParallelLinear[[optimum.neuron.peft.tuners.lora.layer.GQAQKVColumnParallelLinear]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/peft/tuners/lora/layer.py#L433)

get_delta_weightoptimum.neuron.peft.tuners.lora.layer.GQAQKVColumnParallelLinear.get_delta_weighthttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/peft/tuners/lora/layer.py#L578[{"name": "adapter", "val": ": str"}]- **adapter** -- The name of the adapter for which the delta weight should be computed.0Dict mapping "q"/"k"/"v" (or "qkv") to their delta weight tensors (sharded).

Compute the delta weights for Q, K, V for the given adapter.

Returns a dict with keys "q", "k", "v" (or "qkv" if fused) containing the delta tensors.

**Parameters:**

adapter : The name of the adapter for which the delta weight should be computed.

**Returns:**

Dict mapping "q"/"k"/"v" (or "qkv") to their delta weight tensors (sharded).
#### merge[[optimum.neuron.peft.tuners.lora.layer.GQAQKVColumnParallelLinear.merge]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/peft/tuners/lora/layer.py#L625)

Merge the active adapter weights into the base Q, K, V weights.

This works with GQAQKVColumnParallelLinear layers.
The merge happens on the sharded weights - each rank merges its own shard.

**Parameters:**

safe_merge : If True, perform merge in a copy and check for NaNs before merging.

adapter_names : List of adapter names to merge. If None, all active adapters will be merged.
#### unmerge[[optimum.neuron.peft.tuners.lora.layer.GQAQKVColumnParallelLinear.unmerge]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/peft/tuners/lora/layer.py#L688)

Unmerge all merged adapter layers from the base Q, K, V weights.

This works with GQAQKVColumnParallelLinear layers.
The unmerge happens on the sharded weights - each rank unmerges its own shard.

### Parallel Embedding LoRA[[optimum.neuron.peft.tuners.lora.layer.ParallelEmbedding]]

#### optimum.neuron.peft.tuners.lora.layer.ParallelEmbedding[[optimum.neuron.peft.tuners.lora.layer.ParallelEmbedding]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/peft/tuners/lora/layer.py#L758)

mergeoptimum.neuron.peft.tuners.lora.layer.ParallelEmbedding.mergehttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/peft/tuners/lora/layer.py#L847[{"name": "safe_merge", "val": ": bool = False"}, {"name": "adapter_names", "val": ": list[str] | None = None"}]- **safe_merge** -- If True, perform merge in a copy and check for NaNs before merging.
- **adapter_names** -- List of adapter names to merge. If None, all active adapters will be merged.0

Merge the active adapter weights into the base embedding weights.

This works with ParallelEmbedding layers.
The merge happens on the sharded weights - each rank merges its own shard.

**Parameters:**

safe_merge : If True, perform merge in a copy and check for NaNs before merging.

adapter_names : List of adapter names to merge. If None, all active adapters will be merged.
#### unmerge[[optimum.neuron.peft.tuners.lora.layer.ParallelEmbedding.unmerge]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/peft/tuners/lora/layer.py#L885)

Unmerge all merged adapter layers from the base embedding weights.

This works with ParallelEmbedding layers.
The unmerge happens on the sharded weights - each rank unmerges its own shard.

## LoRA Model

### NeuronLoraModel[[optimum.neuron.peft.tuners.NeuronLoraModel]]

#### optimum.neuron.peft.tuners.NeuronLoraModel[[optimum.neuron.peft.tuners.NeuronLoraModel]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/peft/tuners/lora/model.py#L29)

## Utility Functions

### get_peft_model[[optimum.neuron.peft.get_peft_model]]

#### optimum.neuron.peft.get_peft_model[[optimum.neuron.peft.get_peft_model]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/peft/mapping_func.py#L45)

## Architecture Support

The Neuron LoRA implementation supports the following parallel layer types:

- **ColumnParallelLinear**: For layers that split weights along the output dimension
- **RowParallelLinear**: For layers that split weights along the input dimension  
- **ParallelEmbedding**: For embedding layers distributed across ranks
- **GQAQKVColumnParallelLinear**: For grouped query attention projections with challenging tensor parallel configurations

Each layer type has a corresponding LoRA implementation that maintains the parallelization strategy while adding low-rank adaptation capabilities.

## Key Features

- **Distributed Training**: Full support for tensor parallelism and sequence parallelism
- **Checkpoint Consolidation**: Automatic conversion between sharded and consolidated checkpoints
- **Weight Transformation**: Seamless integration with model weight transformation specs
- **Compatibility**: Works with all supported custom modeling architectures in Optimum Neuron
