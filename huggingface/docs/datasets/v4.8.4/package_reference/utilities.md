# Utilities

## Configure logging[[datasets.utils.logging.get_verbosity]]

🤗 Datasets strives to be transparent and explicit about how it works, but this can be quite verbose at times. We have included a series of logging methods which allow you to easily adjust the level of verbosity of the entire library. Currently the default verbosity of the library is set to `WARNING`.

To change the level of verbosity, use one of the direct setters. For instance, here is how to change the verbosity to the `INFO` level:

```py
import datasets
datasets.logging.set_verbosity_info()
```

You can also use the environment variable `DATASETS_VERBOSITY` to override the default verbosity, and set it to one of the following: `debug`, `info`, `warning`, `error`, `critical`:

```bash
DATASETS_VERBOSITY=error ./myprogram.py
```

All the methods of this logging module are documented below. The main ones are:

- [logging.get_verbosity()](/docs/datasets/v4.8.4/en/package_reference/utilities#datasets.utils.logging.get_verbosity) to get the current level of verbosity in the logger
- [logging.set_verbosity()](/docs/datasets/v4.8.4/en/package_reference/utilities#datasets.utils.logging.set_verbosity) to set the verbosity to the level of your choice

In order from the least to the most verbose (with their corresponding `int` values):

1. `logging.CRITICAL` or `logging.FATAL` (int value, 50): only report the most critical errors.
2. `logging.ERROR` (int value, 40): only report errors.
3. `logging.WARNING` or `logging.WARN` (int value, 30): only reports error and warnings. This the default level used by the library.
4. `logging.INFO` (int value, 20): reports error, warnings and basic information.
5. `logging.DEBUG` (int value, 10): report all information.

#### datasets.utils.logging.get_verbosity[[datasets.utils.logging.get_verbosity]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/utils/logging.py#L94)

Return the current level for the HuggingFace datasets library's root logger.

> [!TIP]
> HuggingFace datasets library has following logging levels:
>     - `datasets.logging.CRITICAL`, `datasets.logging.FATAL`
>     - `datasets.logging.ERROR`
>     - `datasets.logging.WARNING`, `datasets.logging.WARN`
>     - `datasets.logging.INFO`
>     - `datasets.logging.DEBUG`

**Returns:**

Logging level, e.g., `datasets.logging.DEBUG` and `datasets.logging.INFO`.

#### datasets.utils.logging.set_verbosity[[datasets.utils.logging.set_verbosity]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/utils/logging.py#L110)

Set the level for the Hugging Face Datasets library's root logger.

**Parameters:**

verbosity : Logging level, e.g., `datasets.logging.DEBUG` and `datasets.logging.INFO`.

#### datasets.utils.logging.set_verbosity_info[[datasets.utils.logging.set_verbosity_info]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/utils/logging.py#L119)

Set the level for the Hugging Face datasets library's root logger to `INFO`.

This will display most of the logging information and tqdm bars.

Shortcut to `datasets.logging.set_verbosity(datasets.logging.INFO)`.

#### datasets.utils.logging.set_verbosity_warning[[datasets.utils.logging.set_verbosity_warning]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/utils/logging.py#L129)

Set the level for the Hugging Face datasets library's root logger to `WARNING`.

This will display only the warning and errors logging information and tqdm bars.

Shortcut to `datasets.logging.set_verbosity(datasets.logging.WARNING)`.

#### datasets.utils.logging.set_verbosity_debug[[datasets.utils.logging.set_verbosity_debug]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/utils/logging.py#L139)

Set the level for the Hugging Face datasets library's root logger to `DEBUG`.

This will display all the logging information and tqdm bars.

Shortcut to `datasets.logging.set_verbosity(datasets.logging.DEBUG)`.

#### datasets.utils.logging.set_verbosity_error[[datasets.utils.logging.set_verbosity_error]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/utils/logging.py#L149)

Set the level for the Hugging Face datasets library's root logger to `ERROR`.

This will display only the errors logging information and tqdm bars.

Shortcut to `datasets.logging.set_verbosity(datasets.logging.ERROR)`.

#### datasets.utils.logging.disable_propagation[[datasets.utils.logging.disable_propagation]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/utils/logging.py#L159)

Disable propagation of the library log outputs.
Note that log propagation is disabled by default.

#### datasets.utils.logging.enable_propagation[[datasets.utils.logging.enable_propagation]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/utils/logging.py#L166)

Enable propagation of the library log outputs.
Please disable the Hugging Face datasets library's default handler to prevent double logging if the root logger has
been configured.

## Configure progress bars[[datasets.enable_progress_bars]]

By default, `tqdm` progress bars will be displayed during dataset download and preprocessing. You can disable them globally by setting `HF_DATASETS_DISABLE_PROGRESS_BARS`
environment variable. You can also enable/disable them using [enable_progress_bars()](/docs/datasets/v4.8.4/en/package_reference/utilities#datasets.enable_progress_bars) and [disable_progress_bars()](/docs/datasets/v4.8.4/en/package_reference/utilities#datasets.disable_progress_bars). If set, the environment variable has priority on the helpers.

#### datasets.enable_progress_bars[[datasets.enable_progress_bars]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/utils/tqdm.py#L78)

Enable globally progress bars used in `datasets` except if `HF_DATASETS_DISABLE_PROGRESS_BARS` environment
variable has been set.

Use [disable_progress_bars()](/docs/datasets/v4.8.4/en/package_reference/utilities#datasets.disable_progress_bars) to disable them.

#### datasets.disable_progress_bars[[datasets.disable_progress_bars]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/utils/tqdm.py#L61)

Disable globally progress bars used in `datasets` except if `HF_DATASETS_DISABLE_PROGRESS_BARS` environment
variable has been set.

Use [enable_progress_bars()](/docs/datasets/v4.8.4/en/package_reference/utilities#datasets.enable_progress_bars) to re-enable them.

#### datasets.are_progress_bars_disabled[[datasets.are_progress_bars_disabled]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/utils/tqdm.py#L95)

Return whether progress bars are globally disabled or not.

Progress bars used in `datasets` can be enable or disabled globally using [enable_progress_bars()](/docs/datasets/v4.8.4/en/package_reference/utilities#datasets.enable_progress_bars)
and [disable_progress_bars()](/docs/datasets/v4.8.4/en/package_reference/utilities#datasets.disable_progress_bars) or by setting `HF_DATASETS_DISABLE_PROGRESS_BARS` as environment variable.
