# Logging

🤗 Diffusers has a centralized logging system to easily manage the verbosity of the library. The default verbosity is set to `WARNING`.

To change the verbosity level, use one of the direct setters. For instance, to change the verbosity to the `INFO` level.

```python
import diffusers

diffusers.logging.set_verbosity_info()
```

You can also use the environment variable `DIFFUSERS_VERBOSITY` to override the default verbosity. You can set it
to one of the following: `debug`, `info`, `warning`, `error`, `critical`. For example:

```bash
DIFFUSERS_VERBOSITY=error ./myprogram.py
```

Additionally, some `warnings` can be disabled by setting the environment variable
`DIFFUSERS_NO_ADVISORY_WARNINGS` to a true value, like `1`. This disables any warning logged by
`logger.warning_advice`. For example:

```bash
DIFFUSERS_NO_ADVISORY_WARNINGS=1 ./myprogram.py
```

Here is an example of how to use the same logger as the library in your own module or script:

```python
from diffusers.utils import logging

logging.set_verbosity_info()
logger = logging.get_logger("diffusers")
logger.info("INFO")
logger.warning("WARN")
```

All methods of the logging module are documented below. The main methods are
`logging.get_verbosity` to get the current level of verbosity in the logger and
`logging.set_verbosity` to set the verbosity to the level of your choice.

In order from the least verbose to the most verbose:

|                                                    Method | Integer value |                                         Description |
|----------------------------------------------------------:|--------------:|----------------------------------------------------:|
| `diffusers.logging.CRITICAL` or `diffusers.logging.FATAL` |            50 |                only report the most critical errors |
|                                 `diffusers.logging.ERROR` |            40 |                                  only report errors |
|   `diffusers.logging.WARNING` or `diffusers.logging.WARN` |            30 |           only report errors and warnings (default) |
|                                  `diffusers.logging.INFO` |            20 | only report errors, warnings, and basic information |
|                                 `diffusers.logging.DEBUG` |            10 |                              report all information |

By default, `tqdm` progress bars are displayed during model download. `logging.disable_progress_bar` and `logging.enable_progress_bar` are used to enable or disable this behavior.

## Base setters[[diffusers.utils.logging.set_verbosity_error]]

#### diffusers.utils.logging.set_verbosity_error[[diffusers.utils.logging.set_verbosity_error]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/logging.py#L201)

Set the verbosity to the `ERROR` level.

#### diffusers.utils.logging.set_verbosity_warning[[diffusers.utils.logging.set_verbosity_warning]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/logging.py#L191)

Set the verbosity to the `WARNING` level.

#### diffusers.utils.logging.set_verbosity_info[[diffusers.utils.logging.set_verbosity_info]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/logging.py#L186)

Set the verbosity to the `INFO` level.

#### diffusers.utils.logging.set_verbosity_debug[[diffusers.utils.logging.set_verbosity_debug]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/logging.py#L196)

Set the verbosity to the `DEBUG` level.

## Other functions[[diffusers.utils.logging.get_verbosity]]

#### diffusers.utils.logging.get_verbosity[[diffusers.utils.logging.get_verbosity]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/logging.py#L147)

Return the current level for the 🤗 Diffusers' root logger as an `int`.

**Returns:**

``int``

Logging level integers which can be one of:

- `50`: `diffusers.logging.CRITICAL` or `diffusers.logging.FATAL`
- `40`: `diffusers.logging.ERROR`
- `30`: `diffusers.logging.WARNING` or `diffusers.logging.WARN`
- `20`: `diffusers.logging.INFO`
- `10`: `diffusers.logging.DEBUG`

#### diffusers.utils.logging.set_verbosity[[diffusers.utils.logging.set_verbosity]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/logging.py#L167)

Set the verbosity level for the 🤗 Diffusers' root logger.

**Parameters:**

verbosity (`int`) : Logging level which can be one of:  - `diffusers.logging.CRITICAL` or `diffusers.logging.FATAL` - `diffusers.logging.ERROR` - `diffusers.logging.WARNING` or `diffusers.logging.WARN` - `diffusers.logging.INFO` - `diffusers.logging.DEBUG`

#### diffusers.utils.get_logger[[diffusers.utils.get_logger]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/logging.py#L131)

Return a logger with the specified name.

This function is not supposed to be directly accessed unless you are writing a custom diffusers module.

#### diffusers.utils.logging.enable_default_handler[[diffusers.utils.logging.enable_default_handler]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/logging.py#L215)

Enable the default handler of the 🤗 Diffusers' root logger.

#### diffusers.utils.logging.disable_default_handler[[diffusers.utils.logging.disable_default_handler]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/logging.py#L206)

Disable the default handler of the 🤗 Diffusers' root logger.

#### diffusers.utils.logging.enable_explicit_format[[diffusers.utils.logging.enable_explicit_format]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/logging.py#L261)

Enable explicit formatting for every 🤗 Diffusers' logger. The explicit formatter is as follows:
```
[LEVELNAME|FILENAME|LINE NUMBER] TIME >> MESSAGE
```

All handlers currently bound to the root logger are affected by this method.

#### diffusers.utils.logging.reset_format[[diffusers.utils.logging.reset_format]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/logging.py#L276)

Resets the formatting for 🤗 Diffusers' loggers.

All handlers currently bound to the root logger are affected by this method.

#### diffusers.utils.logging.enable_progress_bar[[diffusers.utils.logging.enable_progress_bar]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/logging.py#L352)

Enable tqdm progress bar.

#### diffusers.utils.logging.disable_progress_bar[[diffusers.utils.logging.disable_progress_bar]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/utils/logging.py#L358)

Disable tqdm progress bar.
