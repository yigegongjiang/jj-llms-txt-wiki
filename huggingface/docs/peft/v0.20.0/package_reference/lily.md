# Lily: Low-Rank Interconnected Adaptation across Layers

[Lily](https://huggingface.co/papers/2407.09946) is a parameter-efficient fine-tuning technique that introduces cross-layer weight sharing for adapter matrices. Instead of learning an independent AB pair per layer as in LoRA, Lily uses **locally shared A adapters** (each A is shared across a block of `stride_A` consecutive layers) and **globally shared B experts** (a small pool of `num_B` B adapters is shared across all layers). At each forward pass, a lightweight data-dependent router computes a softmax-weighted combination of the B experts to produce the effective B for that layer and input.

This sharing can reduce the total number of adapter matrices from `2N` (standard LoRA) to `N / stride_A + num_B`, freeing up the parameter budget to use a **much larger rank `r`** — typically `2×`–`4×` what you would use in LoRA. Higher rank and better interconnectivity increase the effective rank of the weight update `ΔW = A × combined_B`, leading to better adaptation performance.

Because the B combination is **data-dependent** (the router weights depend on the input activations at runtime), `merge` and `unmerge` are **not supported**. If weight merging is required for your deployment, consider other methods such as LoRA instead.

Lily currently has the following additional constraints:
- Only `nn.Linear` layers are supported.
- Quantized layers are not supported.

If these constraints don't work for your use case, consider other methods instead.

The abstract from the paper is:

> Low-rank adaptation (LoRA) is a widely used parameter-efficient fine-tuning (PEFT) method that learns weight updates ΔW = AB for pretrained weights W through low-rank adapters A and B. While LoRA ensures hardware efficiency, its low-rank weight updates limit adaptation performance. In this paper, we propose low-rank interconnected adaptation across layers (Lily), a novel PEFT method that introduces an interconnected framework with locally shared A and globally shared B experts. This structure eliminates redundant per-layer AB pairs, enabling higher-rank ΔW with equal or fewer parameters. To enhance expressiveness, we use data-dependent routers to determine A-B interconnections, preventing B experts from converging to the same behavior and improving representational power across domains. Experiments across modalities, architectures, and model sizes demonstrate Lily's superior performance and efficiency.

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=LILY"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## LilyConfig[[peft.LilyConfig]]

#### peft.LilyConfig[[peft.LilyConfig]]

```python
peft.LilyConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, r: int = 32, stride_A: int = 1, num_B: int = 2, scaling: float = 1.0, target_modules: Optional[Union[list[str], str]] = None, exclude_modules: Optional[Union[list[str], str]] = None, modules_to_save: Optional[list[str]] = None, layers_to_transform: Optional[Union[list[int], int]] = None, layers_pattern: Optional[Union[list[str], str]] = None, init_weights: bool = True)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/lily/config.py#L24)

**Parameters:**

r (`int`) : Lily's rank. Determines the inner hidden dimension of each adapter and the rank of the weight update `A @ B`. In Lily, since the number of adapters is typically smaller than in LoRA, each adapter needs to carry more capacity, so it is recommended to use a larger `r` than in LoRA — typically `2x`, `3x`, or `4x` the LoRA rank you would normally use. The total number of trainable parameters scales with `r * (total_layers / stride_A + num_B)`, so increasing `r` while keeping `stride_A` large and `num_B` small is the recommended trade-off.

stride_A (`int`) : The number of consecutive layers that share one A adapter. For example, if `stride_A=4`, every 4 adjacent layers share the same A adapter, resulting in `total_layers / stride_A` distinct A adapters in total. The A adapter compresses the input into a low-rank representation of size `r`. `stride_A` should be no less than 1. Suggested values: `2`, `3`, or `4` (i.e. sharing every 2, 3, or 4 layers). Keeping `stride_A` large (fewer distinct A adapters) and increasing `r` instead leads to better performance than the opposite trade-off (small `stride_A`, small `r`). Setting `stride_A=1` means every layer has its own A adapter. NOTE: the A sharing happens within each target (layers with the same target suffix). For example, if your target_modules are `['q_proj', 'v_proj']` and you set `stride_A=2`, then every 2 adjacent q_proj layers will share an A adapter, and every 2 adjacent v_proj layers will share another A adapter, but the q_proj and v_proj layers will not share A adapters with each other since they have different suffixes.

num_B (`int`) : The number of shared B adapters. Unlike A adapters (which are grouped by layer), all B adapters are shared globally across every layer. For each forward pass, a router computes a weighted combination of all `num_B` B adapters (using softmax-normalized weights) to produce a single combined B adapter, which then projects the low-rank representation back to the original dimension. It is recommended to set `num_B` to a similar order as `total_layers / stride_A`. Suggested values: `total_layers / 2`, `total_layers / 3`, or `total_layers / 4`. Similar to `stride_A`, prefer smaller `num_B` with larger `r` over larger `num_B` with smaller `r`. NOTE: to train the router, you need at least 2 B adapters (i.e. `num_B >= 2`), since the router learns to compute a weighted combination of the B adapters. NOTE: the B sharing happens within each target (layers with the same target suffix). For example, if your target_modules are `['q_proj', 'v_proj']` and you set `num_B=4`, then there will be 4 B adapters shared across all q_proj layers, and another 4 B adapters shared across all v_proj layers, but the q_proj and v_proj layers will not share B adapters with each other since they have different suffixes.

target_modules (`Union[List[str], str]`, *optional*) : The names of the modules to apply Lily to. Can be a list of module name strings (e.g. `['q_proj', 'v_proj']`) or a regex pattern (e.g. `'.*decoder.*(SelfAttention|EncDecAttention).*(q|v)$'`). If not specified, Lily will be applied to all supported linear layers.

scaling (`float`) : A scalar multiplier applied to the combined adapter output (`scaling * A @ combined_B`) before adding it to the frozen weight's forward pass. Unlike LoRA, Lily does not use an `alpha / r` formulation; instead, `scaling` is a direct multiplier. This design makes it straightforward to sweep over values on a log scale (e.g. `0.01`, `0.1`, `1.0`, `10.0`). The optimal value is task-dependent and should be treated as a hyperparameter. We recommend starting with `1.0`.

modules_to_save (`List[str]`, *optional*) : List of modules apart from Lily layers to be set as trainable and saved in the final checkpoint. For example, in Sequence Classification or Token Classification tasks, the final layer `classifier/score` are randomly initialized and as such need to be trainable and saved.

exclude_modules (`Union[List[str], str]`, *optional*) : The names of the modules to not apply the adapter. When passing a string, a regex match will be performed. When passing a list of strings, either an exact match will be performed or it is checked if the name of the module ends with any of the passed strings.

layers_to_transform (`Union[list[int], int]`, *optional*) : The layer indexes to transform, if this argument is specified, PEFT will transform only the layers indexes that are specified inside this list. If a single integer is passed, PEFT will transform only the layer at this index.

layers_pattern (`Optional[Union[List[str], str]]`, *optional*) : The layer pattern name, used only if `layers_to_transform` is different to None and if the layer pattern is not in the common layers pattern. This should target the `nn.ModuleList` of the model, which is often called `'layers'` or `'h'`.

init_weights (`bool`) : Whether to initialize Lily adapter weights using the default initialization scheme: A matrices are initialized with Kaiming uniform, and B matrices are initialized to zero, ensuring that the adapter output is zero at the start of training and does not disturb the pretrained model. It is strongly recommended to keep this as `True` unless you have a specific reason to change it.

This is the configuration class to store the configuration of a [LilyModel](/docs/peft/v0.20.0/en/package_reference/lily#peft.LilyModel).

## LilyModel[[peft.LilyModel]]

#### peft.LilyModel[[peft.LilyModel]]

```python
peft.LilyModel(model, peft_config: Union[PeftConfig, dict[str, PeftConfig]], adapter_name: str, low_cpu_mem_usage: bool = False, state_dict: Optional[dict[str, torch.Tensor]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/lily/model.py#L30)

**Parameters:**

model (`torch.nn.Module`) : The model to be adapted.

config ([LilyConfig](/docs/peft/v0.20.0/en/package_reference/lily#peft.LilyConfig)) : The configuration of the Lily model.

adapter_name (`str`) : The name of the adapter, defaults to `"default"`.

**Returns:** `torch.nn.Module`

The Lily PEFT model.

Creates a Low-Rank Interconnected Adaptation Across Layers (Lily) model from a pretrained transformers model.

The method is described in detail in https://arxiv.org/abs/2407.09946.

**Attributes**:
- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The model to be adapted.
- **peft_config** ([LilyConfig](/docs/peft/v0.20.0/en/package_reference/lily#peft.LilyConfig)): The configuration of the Lily model.
