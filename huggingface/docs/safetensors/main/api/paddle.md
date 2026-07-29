# PaddlePaddle API[[safetensors.paddle.load_file]]

#### safetensors.paddle.load_file[[safetensors.paddle.load_file]]

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/paddle.py#L119)

Loads a safetensors file into paddle format.

Example:

```python
from safetensors.paddle import load_file

file_path = "./my_folder/bert.safetensors"
loaded = load_file(file_path)
```

**Parameters:**

filename (`str`, or `os.PathLike`)) : The name of the file which contains the tensors

device (`Union[Dict[str, any], str]`, *optional*, defaults to `cpu`) : The device where the tensors need to be located after load. available options are all regular paddle device locations

backend (`str`, *optional*, defaults to `"mmap"`) : Storage backend used to serve tensor bytes. `"mmap"` (default) and `"pread"` uses `pread(2)` to read tensor bytes.

**Returns:**

``Dict[str, paddle.Tensor]``

dictionary that contains name as key, value as `paddle.Tensor`

#### safetensors.paddle.load[[safetensors.paddle.load]]

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/paddle.py#L88)

Loads a safetensors file into paddle format from pure bytes.

Example:

```python
from safetensors.paddle import load

file_path = "./my_folder/bert.safetensors"
with open(file_path, "rb") as f:
    data = f.read()

loaded = load(data)
```

**Parameters:**

data (`bytes`) : The content of a safetensors file

**Returns:**

``Dict[str, paddle.Tensor]``

dictionary that contains name as key, value as `paddle.Tensor` on cpu

#### safetensors.paddle.save_file[[safetensors.paddle.save_file]]

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/paddle.py#L51)

Saves a dictionary of tensors into raw bytes in safetensors format.

Example:

```python
from safetensors.paddle import save_file
import paddle

tensors = {"embedding": paddle.zeros((512, 1024)), "attention": paddle.zeros((256, 256))}
save_file(tensors, "model.safetensors")
```

**Parameters:**

tensors (`Dict[str, paddle.Tensor]`) : The incoming tensors. Tensors need to be contiguous and dense.

filename (`str`, or `os.PathLike`)) : The filename we're saving into.

metadata (`Dict[str, str]`, *optional*, defaults to `None`) : Optional text only metadata you might want to save in your header. For instance it can be useful to specify more about the underlying tensors. This is purely informative and does not affect tensor loading.

**Returns:**

``None``

#### safetensors.paddle.save[[safetensors.paddle.save]]

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/paddle.py#L18)

Saves a dictionary of tensors into raw bytes in safetensors format.

Example:

```python
from safetensors.paddle import save
import paddle

tensors = {"embedding": paddle.zeros((512, 1024)), "attention": paddle.zeros((256, 256))}
byte_data = save(tensors)
```

**Parameters:**

tensors (`Dict[str, paddle.Tensor]`) : The incoming tensors. Tensors need to be contiguous and dense.

metadata (`Dict[str, str]`, *optional*, defaults to `None`) : Optional text only metadata you might want to save in your header. For instance it can be useful to specify more about the underlying tensors. This is purely informative and does not affect tensor loading.

**Returns:**

``bytes``

The raw bytes representing the format
