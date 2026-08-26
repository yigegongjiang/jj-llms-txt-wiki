# Time Series Utilities

This page lists all the utility functions and classes that can be used for Time Series based models.

Most of those are only useful if you are studying the code of the time series models or you wish to add to the collection of distributional output classes.

## Distributional Output[[transformers.time_series_utils.NormalOutput]]

#### transformers.time_series_utils.NormalOutput[[transformers.time_series_utils.NormalOutput]]

```python
transformers.time_series_utils.NormalOutput(dim: int = 1)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/time_series_utils.py#L179)

Normal distribution output class.

#### Normal[[transformers.time_series_utils.NormalOutput.distribution_class]]

```python
Normal(*args, **kwargs)
```

A mock value for a dotted path (e.g. `torch.float32`): attribute access chains,
calls behave as pass-through decorators, `repr` is the dotted path, and using it
as a base class substitutes a plain-`type` base (PEP 560 `__mro_entries__`), so
real subclasses keep a normal metaclass and `inspect.signature` reads their real
`__init__` instead of a mock's.

#### transformers.time_series_utils.StudentTOutput[[transformers.time_series_utils.StudentTOutput]]

```python
transformers.time_series_utils.StudentTOutput(dim: int = 1)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/time_series_utils.py#L164)

Student-T distribution output class.

#### StudentT[[transformers.time_series_utils.StudentTOutput.distribution_class]]

```python
StudentT(*args, **kwargs)
```

A mock value for a dotted path (e.g. `torch.float32`): attribute access chains,
calls behave as pass-through decorators, `repr` is the dotted path, and using it
as a base class substitutes a plain-`type` base (PEP 560 `__mro_entries__`), so
real subclasses keep a normal metaclass and `inspect.signature` reads their real
`__init__` instead of a mock's.

#### transformers.time_series_utils.NegativeBinomialOutput[[transformers.time_series_utils.NegativeBinomialOutput]]

```python
transformers.time_series_utils.NegativeBinomialOutput(dim: int = 1)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/time_series_utils.py#L193)

Negative Binomial distribution output class.

#### NegativeBinomial[[transformers.time_series_utils.NegativeBinomialOutput.distribution_class]]

```python
NegativeBinomial(*args, **kwargs)
```

A mock value for a dotted path (e.g. `torch.float32`): attribute access chains,
calls behave as pass-through decorators, `repr` is the dotted path, and using it
as a base class substitutes a plain-`type` base (PEP 560 `__mro_entries__`), so
real subclasses keep a normal metaclass and `inspect.signature` reads their real
`__init__` instead of a mock's.
