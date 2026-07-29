# Numpy API[[safetensors.numpy.load_file]]

#### safetensors.numpy.load_file[[safetensors.numpy.load_file]]

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/numpy.py#L125)

Loads a safetensors file into numpy format.

Example:

```python
from safetensors.numpy import load_file

file_path = "./my_folder/bert.safetensors"
loaded = load_file(file_path)
```

**Parameters:**

filename (`str`, or `os.PathLike`)) : The name of the file which contains the tensors

backend (`str`, *optional*, defaults to `"mmap"`) : Storage backend used to serve tensor bytes. `"mmap"` (default) and `"pread"` uses `pread(2)` to read tensor bytes.

**Returns:**

``Dict[str, np.ndarray]``

dictionary that contains name as key, value as `np.ndarray`

#### safetensors.numpy.load[[safetensors.numpy.load]]

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/numpy.py#L98)

Loads a safetensors file into numpy format from pure bytes.

Example:

```python
from safetensors.numpy import load

file_path = "./my_folder/bert.safetensors"
with open(file_path, "rb") as f:
    data = f.read()

loaded = load(data)
```

**Parameters:**

data (`bytes`) : The content of a safetensors file

**Returns:**

``Dict[str, np.ndarray]``

dictionary that contains name as key, value as `np.ndarray` on cpu

#### safetensors.numpy.save_file[[safetensors.numpy.save_file]]

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/numpy.py#L61)

Saves a dictionary of tensors into raw bytes in safetensors format.

Example:

```python
from safetensors.numpy import save_file
import numpy as np

tensors = {"embedding": np.zeros((512, 1024)), "attention": np.zeros((256, 256))}
save_file(tensors, "model.safetensors")
```

**Parameters:**

tensor_dict (`Dict[str, np.ndarray]`) : The incoming tensors. Tensors need to be contiguous and dense.

filename (`str`, or `os.PathLike`)) : The filename we're saving into.

metadata (`Dict[str, str]`, *optional*, defaults to `None`) : Optional text only metadata you might want to save in your header. For instance it can be useful to specify more about the underlying tensors. This is purely informative and does not affect tensor loading.

**Returns:**

``None``

#### safetensors.numpy.save[[safetensors.numpy.save]]

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/numpy.py#L28)

Saves a dictionary of tensors into raw bytes in safetensors format.

Example:

```python
from safetensors.numpy import save
import numpy as np

tensors = {"embedding": np.zeros((512, 1024)), "attention": np.zeros((256, 256))}
byte_data = save(tensors)
```

**Parameters:**

tensor_dict (`Dict[str, np.ndarray]`) : The incoming tensors. Tensors need to be contiguous and dense.

metadata (`Dict[str, str]`, *optional*, defaults to `None`) : Optional text only metadata you might want to save in your header. For instance it can be useful to specify more about the underlying tensors. This is purely informative and does not affect tensor loading.

**Returns:**

``bytes``

The raw bytes representing the format
