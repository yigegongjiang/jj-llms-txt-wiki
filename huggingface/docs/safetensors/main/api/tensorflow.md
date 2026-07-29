# Tensorflow API[[safetensors.tensorflow.load_file]]

#### safetensors.tensorflow.load_file[[safetensors.tensorflow.load_file]]

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/tensorflow.py#L103)

Loads a safetensors file into tensorflow format.

Example:

```python
from safetensors.tensorflow import load_file

file_path = "./my_folder/bert.safetensors"
loaded = load_file(file_path)
```

**Parameters:**

filename (`str`, or `os.PathLike`)) : The name of the file which contains the tensors

backend (`str`, *optional*, defaults to `"mmap"`) : Storage backend used to serve tensor bytes. `"mmap"` (default) and `"pread"` uses `pread(2)` to read tensor bytes.

**Returns:**

``Dict[str, tf.Tensor]``

dictionary that contains name as key, value as `tf.Tensor`

#### safetensors.tensorflow.load[[safetensors.tensorflow.load]]

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/tensorflow.py#L76)

Loads a safetensors file into tensorflow format from pure bytes.

Example:

```python
from safetensors.tensorflow import load

file_path = "./my_folder/bert.safetensors"
with open(file_path, "rb") as f:
    data = f.read()

loaded = load(data)
```

**Parameters:**

data (`bytes`) : The content of a safetensors file

**Returns:**

``Dict[str, tf.Tensor]``

dictionary that contains name as key, value as `tf.Tensor` on cpu

#### safetensors.tensorflow.save_file[[safetensors.tensorflow.save_file]]

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/tensorflow.py#L41)

Saves a dictionary of tensors into raw bytes in safetensors format.

Example:

```python
from safetensors.tensorflow import save_file
import tensorflow as tf

tensors = {"embedding": tf.zeros((512, 1024)), "attention": tf.zeros((256, 256))}
save_file(tensors, "model.safetensors")
```

**Parameters:**

tensors (`Dict[str, tf.Tensor]`) : The incoming tensors. Tensors need to be contiguous and dense.

filename (`str`, or `os.PathLike`)) : The filename we're saving into.

metadata (`Dict[str, str]`, *optional*, defaults to `None`) : Optional text only metadata you might want to save in your header. For instance it can be useful to specify more about the underlying tensors. This is purely informative and does not affect tensor loading.

**Returns:**

``None``

#### safetensors.tensorflow.save[[safetensors.tensorflow.save]]

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/tensorflow.py#L10)

Saves a dictionary of tensors into raw bytes in safetensors format.

Example:

```python
from safetensors.tensorflow import save
import tensorflow as tf

tensors = {"embedding": tf.zeros((512, 1024)), "attention": tf.zeros((256, 256))}
byte_data = save(tensors)
```

**Parameters:**

tensors (`Dict[str, tf.Tensor]`) : The incoming tensors. Tensors need to be contiguous and dense.

metadata (`Dict[str, str]`, *optional*, defaults to `None`) : Optional text only metadata you might want to save in your header. For instance it can be useful to specify more about the underlying tensors. This is purely informative and does not affect tensor loading.

**Returns:**

``bytes``

The raw bytes representing the format
