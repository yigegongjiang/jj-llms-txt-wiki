# FP8

Below are functions and classes relative to the underlying FP8 implementation

## FP8RecipeKwargs[[accelerate.utils.FP8RecipeKwargs]]

#### accelerate.utils.FP8RecipeKwargs[[accelerate.utils.FP8RecipeKwargs]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L457)

Deprecated. Please use one of the proper FP8 recipe kwargs classes such as `TERecipeKwargs` or `MSAMPRecipeKwargs`
instead.

## convert_model[[accelerate.utils.convert_model]]

#### accelerate.utils.convert_model[[accelerate.utils.convert_model]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/transformer_engine.py#L26)

Recursively converts the linear and layernorm layers of a model to their `transformers_engine` counterpart.

## has_transformer_engine_layers[[accelerate.utils.has_transformer_engine_layers]]

#### accelerate.utils.has_transformer_engine_layers[[accelerate.utils.has_transformer_engine_layers]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/transformer_engine.py#L95)

Returns whether a given model has some `transformer_engine` layer or not.

## contextual_fp8_autocast[[accelerate.utils.contextual_fp8_autocast]]

#### accelerate.utils.contextual_fp8_autocast[[accelerate.utils.contextual_fp8_autocast]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/transformer_engine.py#L118)

Wrapper for a model's forward method to apply FP8 autocast. Is context aware, meaning that by default it will
disable FP8 autocast during eval mode, which is generally better for more accurate metrics.

## apply_fp8_autowrap[[accelerate.utils.apply_fp8_autowrap]]

#### accelerate.utils.apply_fp8_autowrap[[accelerate.utils.apply_fp8_autowrap]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/transformer_engine.py#L142)

Applies FP8 context manager to the model's forward method
