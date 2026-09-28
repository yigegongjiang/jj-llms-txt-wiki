# FP8

Below are functions and classes relative to the underlying FP8 implementation

## FP8RecipeKwargs[[accelerate.utils.FP8RecipeKwargs]]

#### accelerate.utils.FP8RecipeKwargs[[accelerate.utils.FP8RecipeKwargs]]

```python
accelerate.utils.FP8RecipeKwargs(opt_level: typing.Literal['O1', 'O2'] = None, use_autocast_during_eval: typing.Optional[bool] = None, margin: typing.Optional[int] = None, interval: typing.Optional[int] = None, fp8_format: typing.Literal['HYBRID', 'E4M3', 'E5M2'] = None, amax_history_len: typing.Optional[int] = None, amax_compute_algo: typing.Literal['max', 'most_recent'] = None, override_linear_precision: tuple = None, use_mxfp8_block_scaling: typing.Optional[bool] = None, backend: typing.Literal['MSAMP', 'TE'] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L457)

Deprecated. Please use one of the proper FP8 recipe kwargs classes such as `TERecipeKwargs` or `MSAMPRecipeKwargs`
instead.

## convert_model[[accelerate.utils.convert_model]]

#### accelerate.utils.convert_model[[accelerate.utils.convert_model]]

```python
accelerate.utils.convert_model(model, to_transformer_engine = True, _convert_linear = True, _convert_ln = True)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/transformer_engine.py#L26)

Recursively converts the linear and layernorm layers of a model to their `transformers_engine` counterpart.

## has_transformer_engine_layers[[accelerate.utils.has_transformer_engine_layers]]

#### accelerate.utils.has_transformer_engine_layers[[accelerate.utils.has_transformer_engine_layers]]

```python
accelerate.utils.has_transformer_engine_layers(model)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/transformer_engine.py#L95)

Returns whether a given model has some `transformer_engine` layer or not.

## contextual_fp8_autocast[[accelerate.utils.contextual_fp8_autocast]]

#### accelerate.utils.contextual_fp8_autocast[[accelerate.utils.contextual_fp8_autocast]]

```python
accelerate.utils.contextual_fp8_autocast(model_forward, fp8_recipe, use_during_eval = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/transformer_engine.py#L118)

Wrapper for a model's forward method to apply FP8 autocast. Is context aware, meaning that by default it will
disable FP8 autocast during eval mode, which is generally better for more accurate metrics.

## apply_fp8_autowrap[[accelerate.utils.apply_fp8_autowrap]]

#### accelerate.utils.apply_fp8_autowrap[[accelerate.utils.apply_fp8_autowrap]]

```python
accelerate.utils.apply_fp8_autowrap(model, fp8_recipe_handler)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/transformer_engine.py#L142)

Applies FP8 context manager to the model's forward method
