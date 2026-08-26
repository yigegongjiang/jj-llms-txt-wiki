# General Utilities

This page lists all of Transformers general utility functions that are found in the file `utils.py`.

Most of those are only useful if you are studying the general code in the library.

## Enums and namedtuples[[transformers.utils.ExplicitEnum]]

#### transformers.utils.ExplicitEnum[[transformers.utils.ExplicitEnum]]

```python
transformers.utils.ExplicitEnum(value, names = None, module = None, qualname = None, type = None, start = 1)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/utils/generic.py#L567)

Enum with more explicit error message for missing values.

#### transformers.utils.PaddingStrategy[[transformers.utils.PaddingStrategy]]

```python
transformers.utils.PaddingStrategy(value, names = None, module = None, qualname = None, type = None, start = 1)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/utils/generic.py#L579)

Possible values for the `padding` argument in [PreTrainedTokenizerBase.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__). Useful for tab-completion in an
IDE.

#### transformers.TensorType[[transformers.TensorType]]

```python
transformers.TensorType(value, names = None, module = None, qualname = None, type = None, start = 1)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/utils/generic.py#L590)

Possible values for the `return_tensors` argument in [PreTrainedTokenizerBase.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__). Useful for
tab-completion in an IDE.

## Special Decorators[[transformers.add_start_docstrings]]

#### transformers.add_start_docstrings[[transformers.add_start_docstrings]]

```python
transformers.add_start_docstrings(*docstr)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/utils/doc.py#L38)

#### transformers.utils.add_start_docstrings_to_model_forward[[transformers.utils.add_start_docstrings_to_model_forward]]

```python
transformers.utils.add_start_docstrings_to_model_forward(*docstr)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/utils/doc.py#L46)

#### transformers.add_end_docstrings[[transformers.add_end_docstrings]]

```python
transformers.add_end_docstrings(*docstr)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/utils/doc.py#L82)

#### transformers.utils.add_code_sample_docstrings[[transformers.utils.add_code_sample_docstrings]]

```python
transformers.utils.add_code_sample_docstrings(*docstr, processor_class = None, checkpoint = None, output_type = None, config_class = None, mask = '[MASK]', qa_target_start_index = 14, qa_target_end_index = 15, model_cls = None, modality = None, expected_output = None, expected_loss = None, real_checkpoint = None, revision = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/utils/doc.py#L969)

#### transformers.utils.replace_return_docstrings[[transformers.utils.replace_return_docstrings]]

```python
transformers.utils.replace_return_docstrings(output_type = None, config_class = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/utils/doc.py#L1063)

## Other Utilities[[transformers.utils._LazyModule]]

#### transformers.utils._LazyModule[[transformers.utils._LazyModule]]

```python
transformers.utils._LazyModule(name: str, module_file: str, import_structure: dict, module_spec: _frozen_importlib.ModuleSpec | None = None, extra_objects: dict[str, object] | None = None, explicit_import_shortcut: dict[str, list[str]] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/utils/import_utils.py#L2370)

Module class that surfaces all objects but only performs associated imports when the objects are requested.
