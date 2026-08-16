# Strict Dataclasses

The `huggingface_hub` package provides a utility to create **strict dataclasses**. These are enhanced versions of Python's standard `dataclass` with additional validation features. Strict dataclasses ensure that fields are validated both during initialization and assignment, making them ideal for scenarios where data integrity is critical.

## Overview

Strict dataclasses are created using the `@strict` decorator. They extend the functionality of regular dataclasses by:

- Validating field types based on type hints
- Supporting custom validators for additional checks
- Optionally allowing arbitrary keyword arguments in the constructor
- Validating fields both at initialization and during assignment

## Benefits

- **Data Integrity**: Ensures fields always contain valid data
- **Ease of Use**: Integrates seamlessly with Python's `dataclass` module
- **Flexibility**: Supports custom validators for complex validation logic
- **Lightweight**: Requires no additional dependencies such as Pydantic, attrs, or similar libraries

## Usage

### Basic Example

```python
from dataclasses import dataclass
from huggingface_hub.dataclasses import strict, as_validated_field

# Custom validator to ensure a value is positive
@as_validated_field
def positive_int(value: int):
    if not value > 0:
        raise ValueError(f"Value must be positive, got {value}")

@strict
@dataclass
class Config:
    model_type: str
    hidden_size: int = positive_int(default=16)
    vocab_size: int = 32  # Default value

    # Methods named `validate_xxx` are treated as class-wise validators
    def validate_big_enough_vocab(self):
        if self.vocab_size < self.hidden_size:
            raise ValueError(f"vocab_size ({self.vocab_size}) must be greater than hidden_size ({self.hidden_size})")
```

Fields are validated during initialization:

```python
config = Config(model_type="bert", hidden_size=24)   # Valid
config = Config(model_type="bert", hidden_size=-1)   # Raises StrictDataclassFieldValidationError
```

Consistency between fields is also validated during initialization (class-wise validation):

```python
# `vocab_size` too small compared to `hidden_size`
config = Config(model_type="bert", hidden_size=32, vocab_size=16)   # Raises StrictDataclassClassValidationError
```

Fields are also validated during assignment:

```python
config.hidden_size = 512  # Valid
config.hidden_size = -1   # Raises StrictDataclassFieldValidationError
```

To re-run class-wide validation after assignment, you must call `.validate` explicitly:

```python
config.validate()  # Runs all class validators
```

### Custom Validators

You can attach multiple custom validators to fields using `validated_field`. A validator is a callable that takes a single argument and raises an exception if the value is invalid.

```python
from dataclasses import dataclass
from huggingface_hub.dataclasses import strict, validated_field

def multiple_of_64(value: int):
    if value % 64 != 0:
        raise ValueError(f"Value must be a multiple of 64, got {value}")

@strict
@dataclass
class Config:
    hidden_size: int = validated_field(validator=[positive_int, multiple_of_64])
```

In this example, both validators are applied to the `hidden_size` field.

### Additional Keyword Arguments

By default, strict dataclasses only accept fields defined in the class. You can allow additional keyword arguments by setting `accept_kwargs=True` in the `@strict` decorator.

```python
from dataclasses import dataclass
from huggingface_hub.dataclasses import strict

@strict(accept_kwargs=True)
@dataclass
class ConfigWithKwargs:
    model_type: str
    vocab_size: int = 16

config = ConfigWithKwargs(model_type="bert", vocab_size=30000, extra_field="extra_value")
print(config)  # ConfigWithKwargs(model_type='bert', vocab_size=30000, *extra_field='extra_value')
```

Additional keyword arguments appear in the string representation of the dataclass but are prefixed with `*` to highlight that they are not validated.

### Integration with Type Hints

Strict dataclasses respect type hints and validate them automatically. For example:

```python
from typing import List
from dataclasses import dataclass
from huggingface_hub.dataclasses import strict

@strict
@dataclass
class Config:
    layers: List[int]

config = Config(layers=[64, 128])  # Valid
config = Config(layers="not_a_list")  # Raises StrictDataclassFieldValidationError
```

Supported types include:
- Any
- Union
- Optional
- Literal
- List
- Dict
- Tuple
- Set

And any combination of these types. If your need more complex type validation, you can do it through a custom validator.

### Class validators

Methods named `validate_xxx` are treated as class validators. These methods must only take `self` as an argument. Class validators are run once during initialization, right after `__post_init__`. You can define as many of them as needed—they'll be executed sequentially in the order they appear.

Note that class validators are not automatically re-run when a field is updated after initialization. To manually re-validate the object, you need to call `obj.validate()`.

```py
from dataclasses import dataclass
from huggingface_hub.dataclasses import strict

@strict
@dataclass
class Config:
    foo: str
    foo_length: int
    upper_case: bool = False

    def validate_foo_length(self):
        if len(self.foo) != self.foo_length:
            raise ValueError(f"foo must be {self.foo_length} characters long, got {len(self.foo)}")

    def validate_foo_casing(self):
        if self.upper_case and self.foo.upper() != self.foo:
            raise ValueError(f"foo must be uppercase, got {self.foo}")

config = Config(foo="bar", foo_length=3) # ok

config.upper_case = True
config.validate() # Raises StrictDataclassClassValidationError

Config(foo="abcd", foo_length=3) # Raises StrictDataclassFieldValidationError
Config(foo="Bar", foo_length=3, upper_case=True) # Raises StrictDataclassFieldValidationError
```

> [!WARNING]
> Method `.validate()` is a reserved name on strict dataclasses.
> To prevent unexpected behaviors, a `StrictDataclassDefinitionError` error will be raised if your class already defines one.

## API Reference

### `@strict`[[huggingface_hub.dataclasses.strict]]

The `@strict` decorator enhances a dataclass with strict validation.

#### huggingface_hub.dataclasses.strict[[huggingface_hub.dataclasses.strict]]

```python
huggingface_hub.dataclasses.strict(accept_kwargs: bool = False)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/dataclasses.py#L56)

**Parameters:**

cls : The class to convert to a strict dataclass.

accept_kwargs (`bool`, *optional*) : If True, allows arbitrary keyword arguments in `__init__`. Defaults to False.

**Returns:**

The enhanced dataclass with strict validation on field assignment.

Decorator to add strict validation to a dataclass.

This decorator must be used on top of `@dataclass` to ensure IDEs and static typing tools
recognize the class as a dataclass.

Can be used with or without arguments:
- `@strict`
- `@strict(accept_kwargs=True)`

Example:
```py
>>> from dataclasses import dataclass
>>> from huggingface_hub.dataclasses import as_validated_field, strict, validated_field

>>> @as_validated_field
>>> def positive_int(value: int):
...     if not value >= 0:
...         raise ValueError(f"Value must be positive, got {value}")

>>> @strict(accept_kwargs=True)
... @dataclass
... class User:
...     name: str
...     age: int = positive_int(default=10)

# Initialize
>>> User(name="John")
User(name='John', age=10)

# Extra kwargs are accepted
>>> User(name="John", age=30, lastname="Doe")
User(name='John', age=30, *lastname='Doe')

# Invalid type => raises
>>> User(name="John", age="30")
huggingface_hub.errors.StrictDataclassFieldValidationError: Validation error for field 'age':
    TypeError: Field 'age' expected int, got str (value: '30')

# Invalid value => raises
>>> User(name="John", age=-1)
huggingface_hub.errors.StrictDataclassFieldValidationError: Validation error for field 'age':
    ValueError: Value must be positive, got -1
```

### `validate_typed_dict`[[huggingface_hub.dataclasses.validate_typed_dict]]

Method to validate that a dictionary conforms to the types defined in a `TypedDict` class.

This is the equivalent to dataclass validation but for `TypedDict`s. Since typed dicts are never instantiated (only used by static type checkers), validation step must be manually called.

#### huggingface_hub.dataclasses.validate_typed_dict[[huggingface_hub.dataclasses.validate_typed_dict]]

```python
huggingface_hub.dataclasses.validate_typed_dict(schema: type, data: dict)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/dataclasses.py#L286)

**Parameters:**

schema (`type[TypedDictType]`) : The TypedDict class defining the expected structure and types.

data (`dict`) : The dictionary to validate.

**Raises:** ``StrictDataclassFieldValidationError``

- ``StrictDataclassFieldValidationError`` -- 
  If any field in the dictionary does not conform to the expected type.

Validate that a dictionary conforms to the types defined in a TypedDict class.

Under the hood, the typed dict is converted to a strict dataclass and validated using the `@strict` decorator.

Example:
```py
>>> from typing import Annotated, TypedDict
>>> from huggingface_hub.dataclasses import validate_typed_dict

>>> def positive_int(value: int):
...     if not value >= 0:
...         raise ValueError(f"Value must be positive, got {value}")

>>> class User(TypedDict):
...     name: str
...     age: Annotated[int, positive_int]

>>> # Valid data
>>> validate_typed_dict(User, {"name": "John", "age": 30})

>>> # Invalid type for age
>>> validate_typed_dict(User, {"name": "John", "age": "30"})
huggingface_hub.errors.StrictDataclassFieldValidationError: Validation error for field 'age':
    TypeError: Field 'age' expected int, got str (value: '30')

>>> # Invalid value for age
>>> validate_typed_dict(User, {"name": "John", "age": -1})
huggingface_hub.errors.StrictDataclassFieldValidationError: Validation error for field 'age':
    ValueError: Value must be positive, got -1
```

### `as_validated_field`[[huggingface_hub.dataclasses.as_validated_field]]

Decorator to create a `validated_field`. Recommended for fields with a single validator to avoid boilerplate code.

#### huggingface_hub.dataclasses.as_validated_field[[huggingface_hub.dataclasses.as_validated_field]]

```python
huggingface_hub.dataclasses.as_validated_field(validator: Callable)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/dataclasses.py#L426)

**Parameters:**

validator (`Callable`) : A method that takes a value as input and raises ValueError/TypeError if the value is invalid.

Decorates a validator function as a `validated_field` (i.e. a dataclass field with a custom validator).

### `validated_field`[[huggingface_hub.dataclasses.validated_field]]

Creates a dataclass field with custom validation.

#### huggingface_hub.dataclasses.validated_field[[huggingface_hub.dataclasses.validated_field]]

```python
huggingface_hub.dataclasses.validated_field(validator: list[collections.abc.Callable[[typing.Any], None]] | collections.abc.Callable[[typing.Any], None], default: typing.Any = <dataclasses._MISSING_TYPE object at 0x7f8e7d51d090>, default_factory: typing.Any = <dataclasses._MISSING_TYPE object at 0x7f8e7d51d090>, init: bool = True, repr: bool = True, hash: bool | None = None, compare: bool = True, metadata: dict | None = None, **kwargs: typing.Any)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/dataclasses.py#L383)

**Parameters:**

validator (`Callable` or `list[Callable]`) : A method that takes a value as input and raises ValueError/TypeError if the value is invalid. Can be a list of validators to apply multiple checks.

- ****kwargs** : Additional arguments to pass to `dataclasses.field()`.

**Returns:**

A field with the validator attached in metadata

Create a dataclass field with a custom validator.

Useful to apply several checks to a field. If only applying one rule, check out the `as_validated_field` decorator.

### Errors[[huggingface_hub.errors.StrictDataclassError]]

#### huggingface_hub.errors.StrictDataclassError[[huggingface_hub.errors.StrictDataclassError]]

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/errors.py#L528)

Base exception for strict dataclasses.

#### huggingface_hub.errors.StrictDataclassDefinitionError[[huggingface_hub.errors.StrictDataclassDefinitionError]]

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/errors.py#L532)

Exception thrown when a strict dataclass is defined incorrectly.

#### huggingface_hub.errors.StrictDataclassFieldValidationError[[huggingface_hub.errors.StrictDataclassFieldValidationError]]

```python
huggingface_hub.errors.StrictDataclassFieldValidationError(field: str, cause: Exception)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/errors.py#L536)

Exception thrown when a strict dataclass fails validation for a given field.

## Why Not Use `pydantic`? (or `attrs`? or `marshmallow_dataclass`?)

- See discussion in https://github.com/huggingface/transformers/issues/36329 regarding adding Pydantic as a dependency. It would be a heavy addition and require careful logic to support both v1 and v2.
- We don't need most of Pydantic's features, especially those related to automatic casting, jsonschema, serialization, aliases, etc.
- We don't need the ability to instantiate a class from a dictionary.
- We don't want to mutate data. In `@strict`, "validation" means "checking if a value is valid." In Pydantic, "validation" means "casting a value, possibly mutating it, and then checking if it's valid."
- We don't need blazing-fast validation. `@strict` isn't designed for heavy loads where performance is critical. Common use cases involve validating a model configuration (performed once and negligible compared to running a model). This allows us to keep the code minimal.
