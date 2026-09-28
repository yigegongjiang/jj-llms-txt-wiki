# Block-Diagonal LoRA for Eliminating Communication Overhead in Tensor Parallel LoRA Serving

Block-Diagonal LoRA (BD-LoRA) is a LoRA variant in which some LoRA factors are constrained to be block-diagonal. This allows faster serving by eliminating communication overheads
when running inference on multiple GPUs. Despite the block-diagonal constraint, BD-LoRA is similarly performant to vanilla LoRA at similar parameter counts.

BD-LoRA is designed to be used with tensor parallelism, which means sharding the weights of a model among multiple GPUs. A popular sharding strategy is the [Megatron Sharding Strategy](https://arxiv.org/abs/1909.08053). For two linear layers $W_1$, $W_2$ that follow each other (for example the up and down projections in a transformer MLP module), we will shard the first layer in a column-parallel way (which requires LoRA B to be block-diagonal) and the second layer in a row-parallel way (which requires LoRA A to be block-diagonal). For the attention module, this can be similarly achieved by taking the Q, K and V projections together as $W_1$ and the out projection as $W_2$, sharding accordingly. This sharding allows a compatible inference engine to distribute each block-diagonal shard over a different GPU, cutting the need to communicate partial results among GPUs. In the image below, you can see the exact sharding strategy and how this saves computational efforts.

Paper: https://hf.co/papers/2510.23346

### Performance, rank and parameter count
BD-LoRA achieves similar performance to LoRA (see image below, or the `method_comparison` folder in the peft repository root) at the same parameter count. However, as every other factor in BD-LoRA is block-diagonal, a BD-LoRA adapter will have less parameters than a LoRA adapter at the same rank. The performance of BD-LoRA is only competitive when the rank is then increased accordingly. We provide example code for rank-matching at the end of this example notebook.

# API

## BdLoraConfig[[peft.BdLoraConfig]]

#### peft.BdLoraConfig[[peft.BdLoraConfig]]

```python
peft.BdLoraConfig(target_modules_bd_a: Optional[list[str]] = None, target_modules_bd_b: Optional[list[str]] = None, nblocks: int = 1, match_strict: bool = True)
```

[Source](https://github.com/huggingface/peft/blob/v0.21.0/src/peft/tuners/lora/config.py#L189)

**Parameters:**

target_modules_bd_a : Modules where the LoRA-A is block-diagonal. Matches each pattern in the list against the module name via `pattern is in target_name`. Example: ['up_proj', 'q_proj', 'v_proj', 'k_proj']

target_modules_bd_b : Modules where the LoRA-B is block-diagonal. Matches each pattern in the list against the module name via `pattern is in target_name`. Example: ['out_proj', 'down_proj']

nblocks : Number of blocks in block-diagonal matrices

match_strict : If set to true, requires each target_module to have either a block-diagonal LoRA-A or LoRA-B, and raises an error otherwise. You can set this to False to mix LoRA and BD-LoRA training, e.g. if some layers in your module do not benefit from BD-LoRA.

Configuration for BD-LoRA (Block-Diagonal LoRA). BD-LoRA is a LoRA variant that can be used for efficient
multi-LoRA serving in inference engines. The speedup results from reduced inter-GPU communication by setting
certain LoRA modules to be block-diagonal.

To determine which LoRA factors should be set as block-diagonal, follow these guidelines:
- For attention, set
  - Q,K,V projections to be LoRA-B block-diagonal
  - Out projection to be LoRA-A block-diagonal
- For MLPs, set
  - Up, Gate projection to be LoRA-B block-diagonal
  - Down projection to be LoRA-A block-diagonal

For other modules and/or architectures, look into the code of your target inference engine. Modules that are
row-sharded should have LoRA-A block-diagonal, modules that are column-sharded should have LoRA-B block-diagonal.
