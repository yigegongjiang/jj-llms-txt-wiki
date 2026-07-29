# General Utilities

This page lists all of Transformers general utility functions that are found in the file `utils.py`.

Most of those are only useful if you are studying the general code in the library.

## Enums and namedtuples[[transformers.utils.ExplicitEnum]]

Enum with more explicit error message for missing values.

Possible values for the `padding` argument in [PreTrainedTokenizerBase.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__). Useful for tab-completion in an
IDE.

Possible values for the `return_tensors` argument in [PreTrainedTokenizerBase.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__). Useful for
tab-completion in an IDE.

## Special Decorators[[transformers.add_start_docstrings]]

## Other Utilities[[transformers.utils._LazyModule]]

Module class that surfaces all objects but only performs associated imports when the objects are requested.
