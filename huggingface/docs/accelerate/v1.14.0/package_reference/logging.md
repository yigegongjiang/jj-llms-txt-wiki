# Logging[[accelerate.logging.get_logger]]

Refer to the [Troubleshooting guide](../usage_guides/troubleshooting#logging) or to the example below to learn 
how to use Accelerate's logger. 

#### accelerate.logging.get_logger[[accelerate.logging.get_logger]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/logging.py#L93)

Returns a `logging.Logger` for `name` that can handle multiprocessing.

If a log should be called on all processes, pass `main_process_only=False` If a log should be called on all
processes and in order, also pass `in_order=True`

Example:

```python
>>> from accelerate.logging import get_logger
>>> from accelerate import Accelerator

>>> logger = get_logger(__name__)

>>> accelerator = Accelerator()
>>> logger.info("My log", main_process_only=False)
>>> logger.debug("My log", main_process_only=True)

>>> logger = get_logger(__name__, log_level="DEBUG")
>>> logger.info("My log")
>>> logger.debug("My second log")

>>> array = ["a", "b", "c", "d"]
>>> letter_at_rank = array[accelerator.process_index]
>>> logger.info(letter_at_rank, in_order=True)
```

**Parameters:**

name (`str`) : The name for the logger, such as `__file__`

log_level (`str`, *optional*) : The log level to use. If not passed, will default to the `LOG_LEVEL` environment variable, or `INFO` if not
