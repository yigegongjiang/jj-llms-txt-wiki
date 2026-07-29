# Flax API[[safetensors.flax.load_file]]

#### safetensors.flax.load_file[[safetensors.flax.load_file]]

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/flax.py#L102)

Loads a safetensors file into flax format.

Example:

```python
from safetensors.flax import load_file

file_path = "./my_folder/bert.safetensors"
loaded = load_file(file_path)
```

**Parameters:**

filename (`str`, or `os.PathLike`)) : The name of the file which contains the tensors

backend (`str`, *optional*, defaults to `"mmap"`) : Storage backend used to serve tensor bytes. `"mmap"` (default) and `"pread"` uses `pread(2)` to read tensor bytes.

**Returns:**

``Dict[str, Array]``

dictionary that contains name as key, value as `Array`

#### safetensors.flax.load[[safetensors.flax.load]]

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/flax.py#L75)

Loads a safetensors file into flax format from pure bytes.

Example:

```python
from safetensors.flax import load

file_path = "./my_folder/bert.safetensors"
with open(file_path, "rb") as f:
    data = f.read()

loaded = load(data)
```

**Parameters:**

data (`bytes`) : The content of a safetensors file

**Returns:**

``Dict[str, Array]``

dictionary that contains name as key, value as `Array` on cpu

#### safetensors.flax.save_file[[safetensors.flax.save_file]]

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/flax.py#L40)

Saves a dictionary of tensors into raw bytes in safetensors format.

Example:

```python
from safetensors.flax import save_file
from jax import numpy as jnp

tensors = {"embedding": jnp.zeros((512, 1024)), "attention": jnp.zeros((256, 256))}
save_file(tensors, "model.safetensors")
```

**Parameters:**

tensors (`Dict[str, Array]`) : The incoming tensors. Tensors need to be contiguous and dense.

filename (`str`, or `os.PathLike`)) : The filename we're saving into.

metadata (`Dict[str, str]`, *optional*, defaults to `None`) : Optional text only metadata you might want to save in your header. For instance it can be useful to specify more about the underlying tensors. This is purely informative and does not affect tensor loading.

**Returns:**

``None``

#### safetensors.flax.save[[safetensors.flax.save]]

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/flax.py#L11)

Saves a dictionary of tensors into raw bytes in safetensors format.

Example:

```python
from safetensors.flax import save
from jax import numpy as jnp

tensors = {"embedding": jnp.zeros((512, 1024)), "attention": jnp.zeros((256, 256))}
byte_data = save(tensors)
```

**Parameters:**

tensors (`Dict[str, Array]`) : The incoming tensors. Tensors need to be contiguous and dense.

metadata (`Dict[str, str]`, *optional*, defaults to `None`) : Optional text only metadata you might want to save in your header. For instance it can be useful to specify more about the underlying tensors. This is purely informative and does not affect tensor loading.

**Returns:**

``bytes``

The raw bytes representing the format
