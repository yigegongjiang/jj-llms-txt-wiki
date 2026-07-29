# Model Weight Transformation Specs

The transformation specs API defines how model weights are transformed between the original Transformers implementation and the custom implementation optimized for Neuron devices. This enables automatic weight conversion during model loading and checkpoint consolidation.

## Base Classes

### ModelWeightTransformationSpec[[optimum.neuron.models.training.ModelWeightTransformationSpec]]

#### optimum.neuron.models.training.ModelWeightTransformationSpec[[optimum.neuron.models.training.ModelWeightTransformationSpec]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L91)

This class defines the interface for transforming model weights between the original Transformers implementation
and the custom implementation for Neuron.

adapt_peft_configoptimum.neuron.models.training.ModelWeightTransformationSpec.adapt_peft_confighttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L121[{"name": "peft_config", "val": ": PeftConfig"}, {"name": "inplace", "val": ": bool = False"}]

Adapts the PEFT config to match the custom modeling implementation.
#### adapt_state_dict[[optimum.neuron.models.training.ModelWeightTransformationSpec.adapt_state_dict]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L157)

Transforms the state dict from the original Transformers model to match the custom modeling implementation.
#### get_relevant_parameter_names[[optimum.neuron.models.training.ModelWeightTransformationSpec.get_relevant_parameter_names]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L107)

Returns the set of parameter names that this spec would affect.
#### guess_peft_type[[optimum.neuron.models.training.ModelWeightTransformationSpec.guess_peft_type]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L114)

Guesses the PEFT type of the module associated to the spec.
#### to_original_peft_config[[optimum.neuron.models.training.ModelWeightTransformationSpec.to_original_peft_config]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L128)

Restores the PEFT config to the original one that matches the original Transformers implementation.
#### to_original_weights[[optimum.neuron.models.training.ModelWeightTransformationSpec.to_original_weights]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L207)

Produces the weights associated to this transformation spec from the custom model to match the original
Transformers weights.

**Parameters:**

sharded_state_dicts (dict[str, list[torch.Tensor]]) : The sharded state dicts from the custom modeling implementation.

parameters_metadata (dict[str, dict[str, Any]]) : Metadata about the parameters in the original model.

**Returns:**

`tuple[dict[str, torch.Tensor], list[str]]`

A tuple containing the transformed weights and a list of the
names of the parameters to remove from the final state dict.

### ModelWeightTransformationSpecs[[optimum.neuron.models.training.ModelWeightTransformationSpecs]]

#### optimum.neuron.models.training.ModelWeightTransformationSpecs[[optimum.neuron.models.training.ModelWeightTransformationSpecs]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L239)

Defines a list of transformation specs for a given module of the model.

### CustomModule[[optimum.neuron.models.training.CustomModule]]

#### optimum.neuron.models.training.CustomModule[[optimum.neuron.models.training.CustomModule]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L342)

This class is used to mark a module as a custom module. It is used to identify the modules that contain weights
that need to transformed when loading and saving the model.

## Transformation Specifications

### FusedLinearsSpec[[optimum.neuron.models.training.FusedLinearsSpec]]

#### optimum.neuron.models.training.FusedLinearsSpec[[optimum.neuron.models.training.FusedLinearsSpec]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L365)

Represents a transformation where multiple linear layers are fused into a single linear layer.
It can handle the case where the fused linear layer is sharded across multiple tensor parallel ranks.

### GQAQKVColumnParallelLinearSpec[[optimum.neuron.models.training.GQAQKVColumnParallelLinearSpec]]

#### optimum.neuron.models.training.GQAQKVColumnParallelLinearSpec[[optimum.neuron.models.training.GQAQKVColumnParallelLinearSpec]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L736)

Represents the transformation of separate query, key, and value projections into a single GQAQKVColumnParalleLinear
projection.

compute_query_indices_for_rankoptimum.neuron.models.training.GQAQKVColumnParallelLinearSpec.compute_query_indices_for_rankhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L808[{"name": "tp_size", "val": ": int"}, {"name": "tp_rank", "val": ": int"}, {"name": "num_attention_heads", "val": ": int"}, {"name": "num_key_value_heads", "val": ": int"}, {"name": "kv_size_multiplier", "val": ": int"}]

Computes the permutation for the query weight for a given TP rank.
#### create_kv_proj_local_weight_from_regular_weight[[optimum.neuron.models.training.GQAQKVColumnParallelLinearSpec.create_kv_proj_local_weight_from_regular_weight]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L851)

Creates the local version of the key or value projections weight for the given TP rank.
#### create_query_or_output_projection_local_weight_from_regular_weight[[optimum.neuron.models.training.GQAQKVColumnParallelLinearSpec.create_query_or_output_projection_local_weight_from_regular_weight]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L866)

Creates the local version of the query or output projections weight for the given TP rank.

## Utility Functions

### Weight Creation Functions[[optimum.neuron.models.training.transformations_utils.create_local_weight_with_padding]]

#### optimum.neuron.models.training.transformations_utils.create_local_weight_with_padding[[optimum.neuron.models.training.transformations_utils.create_local_weight_with_padding]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L54)

Shards a tensor along a given axis and return a slice corresponding to the rank.
This will round up the layer to the next multiple if there is need to pad the tensor.

#### optimum.neuron.models.training.transformations_utils.create_local_fused_weight[[optimum.neuron.models.training.transformations_utils.create_local_fused_weight]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L73)

Shards individual weights across the tensor parallel ranks and fuses them into a single weight.

### Model-level Functions[[optimum.neuron.models.training.specialize_transformation_specs_for_model]]

#### optimum.neuron.models.training.specialize_transformation_specs_for_model[[optimum.neuron.models.training.specialize_transformation_specs_for_model]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L1458)

#### optimum.neuron.models.training.adapt_peft_config_for_model[[optimum.neuron.models.training.adapt_peft_config_for_model]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L1467)

#### optimum.neuron.models.training.to_original_peft_config_for_model[[optimum.neuron.models.training.to_original_peft_config_for_model]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L1484)

### State Dict Functions[[optimum.neuron.models.training.adapt_state_dict]]

#### optimum.neuron.models.training.adapt_state_dict[[optimum.neuron.models.training.adapt_state_dict]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L1516)

Transforms the state dict from the original Transformers model to match the custom modeling implementation.

#### optimum.neuron.models.training.to_original_weights[[optimum.neuron.models.training.to_original_weights]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L1590)

Consolidates the sharded state dicts produced by saving the custom model into a single state dict that matches the
original Transformers model weights.

### Metadata Functions[[optimum.neuron.models.training.create_parameter_metadata]]

#### optimum.neuron.models.training.create_parameter_metadata[[optimum.neuron.models.training.create_parameter_metadata]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L1659)

Creates the metadata to be saved with the model weights to be able to reconstruct the original weights when
consolidating the sharded state dicts.

#### optimum.neuron.models.training.transformations_utils.get_tensor_model_parallel_attributes[[optimum.neuron.models.training.transformations_utils.get_tensor_model_parallel_attributes]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L1645)

Returns the tensor model parallel attributes of a tensor.

### Helper Functions[[optimum.neuron.models.training.transformations_utils.remove_adapter_name]]

#### optimum.neuron.models.training.transformations_utils.remove_adapter_name[[optimum.neuron.models.training.transformations_utils.remove_adapter_name]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L1501)

#### optimum.neuron.models.training.transformations_utils.is_base_layer[[optimum.neuron.models.training.transformations_utils.is_base_layer]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L1505)

#### optimum.neuron.models.training.transformations_utils.get_adapter_name[[optimum.neuron.models.training.transformations_utils.get_adapter_name]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/training/transformations_utils.py#L1509)
