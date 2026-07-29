# Import Utilities

This page goes through the transformers utilities to enable lazy and fast object import.
While we strive for minimal dependencies, some models have specific dependencies requirements that cannot be
worked around. We don't want for all users of `transformers` to have to install those dependencies to use other models,
we therefore mark those as soft dependencies rather than hard dependencies.

The transformers toolkit is not made to error-out on import of a model that has a specific dependency; instead, an
object for which you are lacking a dependency will error-out when calling any method on it. As an example, if
`torchvision` isn't installed, the fast image processors will not be available.

This object is still importable:

```python
>>> from transformers import DetrImageProcessor
>>> print(DetrImageProcessor)
<class 'DetrImageProcessor'>
```

However, no method can be called on that object:

```python
>>> DetrImageProcessor.from_pretrained()
ImportError:
DetrImageProcessor requires the Torchvision library but it was not found in your environment. Check out the instructions on the
installation page: https://pytorch.org/get-started/locally/ and follow the ones that match your environment.
Please note that you may need to restart your runtime after installation.
```

Let's see how to specify specific object dependencies.

## Specifying Object Dependencies

### Filename-based

All objects under a given filename have an automatic dependency to the tool linked to the filename

**PyTorch**: All files starting with `modeling_` have an automatic PyTorch dependency

**Tokenizers**: All files starting with `tokenization_` and ending with `_fast` have an automatic `tokenizers` dependency

**Vision**: All files starting with `image_processing_` have an automatic dependency to the `vision` dependency group;
at the time of writing, this only contains the `pillow` dependency.

**Vision + Torch + Torchvision**: All files starting with `image_processing_` and ending with `_fast` have an automatic
dependency to `vision`, `torch`, and `torchvision`.

All of these automatic dependencies are added on top of the explicit dependencies that are detailed below.

### Explicit Object Dependencies

We add a method called `requires` that is used to explicitly specify the dependencies of a given object. As an
example, the `Trainer` class has two hard dependencies: `torch` and `accelerate`. Here is how we specify these
required dependencies:

```python
from .utils.import_utils import requires

@requires(backends=("torch", "accelerate"))
class Trainer:
    ...
```

Backends that can be added here are all the backends that are available in the `import_utils.py` module.

Additionally, specific versions can be specified in each backend. For example, this is how you would specify
a requirement on torch>=2.6 on the `Trainer` class:

```python
from .utils.import_utils import requires

@requires(backends=("torch>=2.6", "accelerate"))
class Trainer:
    ...
```

You can specify the following operators: `==`, `>`, `>=`, `<`, `<=`, `!=`.

## Methods[[transformers.utils.import_utils.define_import_structure]]

This method takes a module_path as input and creates an import structure digestible by a _LazyModule.

Here's an example of an output import structure at the src.transformers.models level:

{
frozenset({'tokenizers'}): {
'albert.tokenization_albert_fast': {'AlbertTokenizer'}
},
frozenset(): {
'albert.configuration_albert': {'AlbertConfig'},
'align.processing_align': {'AlignProcessor'},
'align.configuration_align': {'AlignConfig', 'AlignTextConfig', 'AlignVisionConfig'},
'altclip.configuration_altclip': {'AltCLIPConfig', 'AltCLIPTextConfig', 'AltCLIPVisionConfig'},
'altclip.processing_altclip': {'AltCLIPProcessor'}
}
}

The import structure is a dict defined with frozensets as keys, and dicts of strings to sets of objects.

If `prefix` is not None, it will add that prefix to all keys in the returned dict.

This decorator enables two things:
- Attaching a `__backends` tuple to an object to see what are the necessary backends for it
  to execute correctly without instantiating it
- The '@requires' string is used to dynamically import objects

- **obj** -- object to be checked
- **backends** -- list or tuple of backends to check.

Method that automatically raises in case the specified backends are not available. It is often used during class

initialization to ensure the required dependencies are installed:

```py
requires_backends(self, ["torch"])
```

The backends should be defined in the `BACKEND_MAPPING` defined in `transformers.utils.import_utils`.
