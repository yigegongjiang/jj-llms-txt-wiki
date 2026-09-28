# Errors[[peft.PeftError]]

#### peft.PeftError[[peft.PeftError]]

[Source](https://github.com/huggingface/peft/blob/v0.21.0/src/peft/utils/error.py#L16)

Base PEFT error

#### peft.NoMatchingPeftModuleError[[peft.NoMatchingPeftModuleError]]

[Source](https://github.com/huggingface/peft/blob/v0.21.0/src/peft/utils/error.py#L20)

The adapter being injected matched no module or parameter of the base model.

Raised e.g. when the `target_modules` of the PEFT config matched nothing, which most often points at a
misconfiguration. It subclasses `ValueError` for backwards compatibility with code that intercepted the generic
error raised previously. Code that adds such an adapter intentionally can catch this error (or `PeftError`) and
proceed.
