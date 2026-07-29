# AdaMSS

[AdaMSS](https://openreview.net/forum?id=8ZdWmpYxT0) (AdaMSS: Adaptive Multi-Subspace Approach for Parameter-Efficient Fine-Tuning) is a parameter-efficient fine-tuning method that decomposes weight matrices using SVD and clusters the decomposed space into multiple trainable subspaces. Each subspace learns independent low-rank updates while the original weights remain frozen. AdaMSS also supports Adaptive Subspace Allocation (ASA), which dynamically prunes less important subspaces during training based on gradient information.

The abstract from the paper is:

> We propose AdaMSS, an adaptive multi-subspace approach for parameter-efficient fine-tuning of large models. Unlike traditional parameterefficient fine-tuning methods that operate within a large single subspace of the network weights, AdaMSS leverages subspace segmentation to obtain multiple smaller subspaces and adaptively reduces the number of trainable parameters during training, ultimately updating only those associated with a small subset of subspaces most relevant to the target downstream task. By using the lowest-rank representation, AdaMSS achieves more compact expressiveness and finer tuning of the model parameters. Theoretical analyses demonstrate that AdaMSS has better generalization guarantee than LoRA, PiSSA, and other single-subspace low-rankbased methods. Extensive experiments across image classification, natural language understanding, and natural language generation tasks show that AdaMSS achieves comparable performance to full fine-tuning and outperforms other parameterefficient fine-tuning methods in most cases, all while requiring fewer trainable parameters. Notably, on the ViT-Large model, AdaMSS achieves 4.7% higher average accuracy than LoRA across seven tasks, using just 15.4% of the trainable parameters. On RoBERTa-Large, AdaMSS outperforms PiSSA by 7% in average accuracy across six tasks while reducing the number of trainable parameters by approximately 94.4%. These results demonstrate the effectiveness of AdaMSS in parameter-efficient fine-tuning. The code for AdaMSS is available at https: //github.com/jzheng20/AdaMSS.

AdaMSS currently has the following constraints:
- Only `nn.Linear` layers are supported.
- Requires scikit-learn for the KMeans clustering step.

If these constraints don't work for your use case, consider other methods instead.

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=ADAMSS"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## AdamssConfig[[peft.AdamssConfig]]

- **r** (`int`) --
  Total rank for SVD decomposition (denoted as R in the paper). This determines how many singular vectors are
  used to represent the weight matrix before clustering. Higher values capture more information from the
  original weights but require more computation and memory. Lower values provide stronger regularization.
  Typical values range from 50 to 500. Default is 100.

- **num_subspaces** (`int`) --
  Number of subspaces (K) to cluster the SVD-decomposed space into. Each subspace learns independent low-rank
  updates. Increasing this value allows finer-grained adaptation but increases the number of trainable
  parameters proportionally. When using ASA (Adaptive Subspace Allocation), this determines the initial
  number of subspaces before pruning. Typical values range from 3 to 10. Default is 5.

- **subspace_rank** (`int`) --
  The rank (r_i) for each trainable subspace. This controls the capacity of each subspace to learn
  adaptations. Higher values increase expressiveness but also increase trainable parameters. Total trainable
  parameters scale as O(num_subspaces * subspace_rank * (in_dim + out_dim) / num_subspaces). For most tasks,
  values of 1-4 work well. Default is 1.

- **target_modules** (`Optional[Union[list[str], str]]`) --
  The names of the modules to apply AdaMSS to. If specified, only these modules will be adapted. Can be a
  list of exact module names or a regex expression. For example, `['q_proj', 'v_proj']` for attention layers,
  or `'.*decoder.*(SelfAttention|EncDecAttention).*(q|v)$'` for regex matching.

- **modules_to_save** (`Optional[list[str]]`) --
  List of modules apart from AdaMSS layers to be set as trainable and saved in the final checkpoint. These
  modules will be fully fine-tuned (not just low-rank). Required for randomly initialized heads like
  `classifier` or `score` in classification tasks.

- **init_weights** (`Literal["orthogonal"]`) --
  Initialization method for AdaMSS trainable weights. Currently only "orthogonal" is supported, which uses
  orthogonal initialization for the B matrices (output projection). The A matrices are initialized to zero to
  ensure the model starts from the pretrained weights. Set to None to skip initialization when loading from a
  checkpoint. Default is "orthogonal".

- **layers_to_transform** (`Optional[Union[list[int], int]]`) --
  Specific layer indices to apply AdaMSS to. If specified, only these layers will be adapted, useful for
  experimenting with which layers benefit most from adaptation. Can be a single integer or a list of
  integers.

- **layers_pattern** (`Optional[Union[list[str], str]]`) --
  Pattern to match layer names when `layers_to_transform` is specified. Used to extract layer indices from
  module names that don't follow the common pattern.

- **use_asa** (`bool`) --
  Whether to enable Adaptive Subspace Allocation (ASA). When enabled, ASA dynamically prunes less important
  subspaces during training based on gradient information, reducing the effective number of parameters while
  maintaining performance. Requires integration with a training callback. Default is False.

- **asa_target_subspaces** (`int`) --
  Target total number of active subspaces across all layers when ASA is enabled. ASA will progressively prune
  subspaces until this target is reached. Lower values result in more aggressive pruning and fewer trainable
  parameters. Should be less than `num_subspaces * num_target_modules`. Typical values range from 20 to 100
  depending on model size. Default is 50. Must be a positive integer when `use_asa=True`.

- **init_warmup** (`int`) --
  Number of training steps to wait before starting ASA pruning. During warmup, all subspaces remain active to
  allow importance scores to stabilize. Higher values give more time for accurate importance estimation but
  delay pruning. Typical values range from 50 to 200. Default is 50. Must be smaller than `final_warmup` when
  `use_asa=True`.

- **final_warmup** (`int`) --
  Training step at which ASA completes pruning and reaches `asa_target_subspaces` active subspaces. The
  pruning is distributed between `init_warmup` and `final_warmup`. Should be set based on total training
  steps; typically 1/3 to 1/2 of total training steps. Default is 1000. Must be larger than `init_warmup`
  when `use_asa=True`.

- **mask_interval** (`int`) --
  Number of training steps between ASA mask updates. Lower values allow more frequent adaptation but increase
  overhead. Higher values provide more stable importance estimates between updates. Typical values range from
  50 to 200. Default is 100. Must be a positive integer when `use_asa=True`.

- **asa_importance_beta** (`float`) --
  Exponential moving average (EMA) coefficient for smoothing subspace importance scores. Higher values
  (closer to 1.0) give more weight to historical importance, providing stability. Lower values make
  importance more responsive to recent gradients. Typical values range from 0.8 to 0.95. Default is 0.85.

- **asa_uncertainty_beta** (`float`) --
  EMA coefficient for smoothing importance uncertainty estimates. Controls how quickly uncertainty adapts to
  gradient variance. Similar to asa_importance_beta, higher values provide more stable estimates. Typical
  values range from 0.8 to 0.95. Default is 0.85.

- **asa_schedule_exponent** (`float`) --
  Schedule exponent controlling the decay rate from total subspaces to `asa_target_subspaces` during ASA
  warmup. Higher values result in faster initial pruning (more aggressive early reduction), while lower
  values provide a more gradual, linear-like decay. The formula is: current_active_subspaces =
  asa_target_subspaces + (asa_total_subspaces - asa_target_subspaces) * (progress ** exponent). Typical
  values range from 1.0 (linear) to 5.0 (aggressive). Default is 3.0. Must be a positive number when
  `use_asa=True` (a zero or negative exponent either degenerates the schedule to a permanent no-op or, once
  `progress` reaches exactly 0.0, raises a `ZeroDivisionError`).

- **use_dynamic_rank** (`bool`) --
  Whether to dynamically determine subspace ranks based on singular value magnitudes. When True, each
  subspace's rank is determined by counting singular values above a threshold, allowing different subspaces
  to have different effective ranks. When False, all subspaces use the fixed `subspace_rank`. Default is
  False.

- **svd_threshold** (`float`) --
  Threshold ratio for dynamic rank selection, only used when `use_dynamic_rank=True`. A singular value is
  considered significant if it exceeds `threshold * max_singular_value`. Higher values result in lower
  effective ranks (more aggressive truncation). Typical values range from 0.05 to 0.2. Default is 0.1 (10% of
  max).

Configuration class for Adamss (Adaptive Multi-Subspaces) method.

AdaMSS is a parameter-efficient fine-tuning method that decomposes weight matrices using SVD and clusters the
decomposed space into multiple trainable subspaces. It learns low-rank updates within these subspaces while keeping
the original weights frozen.

## AdamssModel[[peft.AdamssModel]]

- **model** (`torch.nn.Module`) -- The model to be adapted.
- **config** (`AdamssConfig`) -- The configuration of the Adamss model.
- **adapter_name** (`str`) -- The name of the adapter, defaults to `"default"`.`torch.nn.Module`The Adamss model.

Creates Adamss (Adaptive Multi-Subspaces) model from a pretrained model.

The method decomposes weight matrices using SVD and clusters the decomposed space into multiple trainable subspaces
for parameter-efficient fine-tuning.

Example:
```python
>>> from transformers import AutoModelForImageClassification
>>> from peft import AdamssConfig, get_peft_model

>>> config = AdamssConfig(
...     r=500,
...     num_subspaces=5,
...     target_modules=["query", "value"],
... )

>>> model = AutoModelForImageClassification.from_pretrained("google/vit-base-patch16-224")
>>> adamss_model = get_peft_model(model, config)
```

**Attributes**:
- **model** (`~torch.nn.Module`) -- The model to be adapted.
- **peft_config** ([AdamssConfig](/docs/peft/v0.20.0/en/package_reference/adamss#peft.AdamssConfig)): The configuration of the Adamss model.

- **global_step** (*int*) -- The current training step.

Update importance scores and apply ASA masking (if enabled).

This method should be called in **every** training step after `loss.backward()` and before
`optimizer.zero_grad()` when ASA is enabled. Internally it:

1. Accumulates importance scores via EMA every step during the warmup period.
2. At mask intervals, applies global top-K masking and resets importance.

This is the single entry point for ASA – using the `AdamssAsaCallback` with HuggingFace `Trainer`
simply delegates to this method. For custom training loops, call this directly instead of the callback.

Example:

```python
for step, batch in enumerate(dataloader):
loss = model(**batch).loss loss.backward() optimizer.step() model.base_model.update_and_allocate(step)
optimizer.zero_grad()
```
