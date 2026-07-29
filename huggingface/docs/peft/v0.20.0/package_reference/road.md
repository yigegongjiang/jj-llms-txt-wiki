# RoAd

[RoAd](https://huggingface.co/papers/2409.00119) is a parameter‑efficient fine‑tuning technique that adapts large language models by learning a small set of 2×2 rotation matrices (and optional scaling factors) applied to pairs of hidden dimensions. RoAd achieves competitive or superior performance compared to other PEFT methods with under 0.1% trainable parameters. Unlike LoRA’s batched low‑rank updates, RoAd’s sparse rotations reformulate to simple element‑wise operations, yielding significantly higher serving throughput when handling heterogeneous requests in the same batch, i.e. serving multiple adapters simultaneously. Moreover, RoAd integrates seamlessly into a distributed interchange intervention framework, interpreting its sparse 2D rotations as task-specific interventions within learned subspaces of hidden representations. These orthogonal subspaces can be composed to merge multiple task-specific behaviors—like multilingual capabilities or instruction following—without additional fine-tuning, enabling modular, interpretable adaptations in LLMs.

Finetuning with RoAd typically requires higher learning rate compared to LoRA or similar methods, around 1e-3. Currently RoAd only supports linear layers and it can be used on models quantized with bitsandbytes (4-bit or 8-bit).

For running inference with different RoAd adapters in the same batch see [Inference with different LoRA adapters in the same batch](lora#inference-with-different-lora-adapters-in-the-same-batch).

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=ROAD"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## RoadConfig[[peft.RoadConfig]]

- **variant** (Union[`RoadVariant`, `str`]) --
  The variant of the Road model to use. It can be one of road_1, road_2, or road_4. Refer to the paper for
  more details.
  - road_1: Uses the same scale and angle for all pairs of elements.
  This variant has lowest number of parameters, it stores a number equal to the output hidden size of
  parameters for each layer that RoAd is applied to.
  - road_2: Uses the same scale and angle for each element.
  This variant has 2x the number of parameters compared to road_1.
  - road_4: Uses two different scales and angles for each element.
  This variant has 4x the number of parameters compared to road_1.
- **group_size** (`int`) --
  Group size defines how elements are grouped together into 2D vectors for rotation. Within each group
  element 0 is paired with element group_size/2, then element 1 is paired with element group_size/2+1 and so
  on. This has no effect on the model performance, since elements are unordered, however it has some effect
  on inference speed when used in e.g. VLLM. For best speed group size of at least 32 or 64 (the default) is
  recommended. Note that model hidden size (or hidden size per partition when used with tensor parallelism)
  must be divisible by group_size, so for very small models you might need to reduce this parameter.
- **init_weights** (`bool`) --
  Whether to perform initialization of RoAd weights.
- **target_modules** (`Optional[Union[List[str], str]]`) --
  The names of the modules to apply the adapter to. If this is specified, only the modules with the specified
  names will be replaced. When passing a string, a regex match will be performed. When passing a list of
  strings, either an exact match will be performed or it is checked if the name of the module ends with any
  of the passed strings. If this is specified as 'all-linear', then all linear/Conv1D modules are chosen (if
  the model is a PreTrainedModel, the output layer excluded). If this is not specified, modules will be
  chosen according to the model architecture. If the architecture is not known, an error will be raised -- in
  this case, you should specify the target modules manually.
- **modules_to_save** (`List[str]`) --
  List of modules apart from Road layers to be set as trainable and saved in the final checkpoint.

This is the configuration class to store the configuration of a [RoadModel](/docs/peft/v0.20.0/en/package_reference/road#peft.RoadModel). RoAd adapter is proposed in
https://huggingface.co/papers/2409.00119.

## RoadModel[[peft.RoadModel]]
