# Flax API[[safetensors.flax.load_file]]

#### safetensors.flax.load_file[[safetensors.flax.load_file]]

```python
safetensors.flax.load_file(filename: typing.Union[str, os.PathLike], backend: str = 'mmap')
```

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/flax.py#L102)

**Parameters:**

filename (`str`, or `os.PathLike`)) : The name of the file which contains the tensors

backend (`str`, *optional*, defaults to `"mmap"`) : Storage backend used to serve tensor bytes. `"mmap"` (default) and `"pread"` uses `pread(2)` to read tensor bytes.

**Returns:** `Dict[str, Array]`

dictionary that contains name as key, value as `Array`

Loads a safetensors file into flax format.

Example:

```python
from safetensors.flax import load_file

file_path = "./my_folder/bert.safetensors"
loaded = load_file(file_path)
```

#### safetensors.flax.load[[safetensors.flax.load]]

```python
safetensors.flax.load(data: bytes)
```

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/flax.py#L75)

**Parameters:**

data (`bytes`) : The content of a safetensors file

**Returns:** `Dict[str, Array]`

dictionary that contains name as key, value as `Array` on cpu

Loads a safetensors file into flax format from pure bytes.

Example:

```python
from safetensors.flax import load

file_path = "./my_folder/bert.safetensors"
with open(file_path, "rb") as f:
    data = f.read()

loaded = load(data)
```

#### safetensors.flax.save_file[[safetensors.flax.save_file]]

```python
safetensors.flax.save_file(tensors: typing.Dict[str, jax.Array], filename: typing.Union[str, os.PathLike], metadata: typing.Optional[typing.Dict[str, str]] = None)
```

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/flax.py#L40)

**Parameters:**

tensors (`Dict[str, Array]`) : The incoming tensors. Tensors need to be contiguous and dense.

filename (`str`, or `os.PathLike`)) : The filename we're saving into.

metadata (`Dict[str, str]`, *optional*, defaults to `None`) : Optional text only metadata you might want to save in your header. For instance it can be useful to specify more about the underlying tensors. This is purely informative and does not affect tensor loading.

**Returns:** `None`

Saves a dictionary of tensors into raw bytes in safetensors format.

Example:

```python
from safetensors.flax import save_file
from jax import numpy as jnp

tensors = {"embedding": jnp.zeros((512, 1024)), "attention": jnp.zeros((256, 256))}
save_file(tensors, "model.safetensors")
```

#### safetensors.flax.save[[safetensors.flax.save]]

```python
safetensors.flax.save(tensors: typing.Dict[str, jax.Array], metadata: typing.Optional[typing.Dict[str, str]] = None)
```

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/flax.py#L11)

**Parameters:**

tensors (`Dict[str, Array]`) : The incoming tensors. Tensors need to be contiguous and dense.

metadata (`Dict[str, str]`, *optional*, defaults to `None`) : Optional text only metadata you might want to save in your header. For instance it can be useful to specify more about the underlying tensors. This is purely informative and does not affect tensor loading.

**Returns:** `bytes`

The raw bytes representing the format

Saves a dictionary of tensors into raw bytes in safetensors format.

Example:

```python
from safetensors.flax import save
from jax import numpy as jnp

tensors = {"embedding": jnp.zeros((512, 1024)), "attention": jnp.zeros((256, 256))}
byte_data = save(tensors)
```
