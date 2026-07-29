# Torch API[[safetensors.torch.load_file]]

#### safetensors.torch.load_file[[safetensors.torch.load_file]]

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/torch.py#L328)

Loads a safetensors file into torch format.

Example:

```python
from safetensors.torch import load_file

file_path = "./my_folder/bert.safetensors"
loaded = load_file(file_path)
```

**Parameters:**

filename (`str`, or `os.PathLike`) : The name of the file which contains the tensors

device (`Union[str, int]`, *optional*, defaults to `cpu`) : The device where the tensors need to be located after load. available options are all regular torch device locations.

backend (`str`, *optional*, defaults to `"mmap"`) : Storage backend used to serve tensor bytes. `"mmap"` (default) and `"pread"` uses `pread(2)` to read tensor bytes.

**Returns:**

``Dict[str, torch.Tensor]``

dictionary that contains name as key, value as `torch.Tensor`

#### safetensors.torch.load[[safetensors.torch.load]]

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/torch.py#L363)

Loads a safetensors file into torch format from pure bytes.

Example:

```python
from safetensors.torch import load

file_path = "./my_folder/bert.safetensors"
with open(file_path, "rb") as f:
    data = f.read()

loaded = load(data)
```

**Parameters:**

data (`bytes`) : The content of a safetensors file

**Returns:**

``Dict[str, torch.Tensor]``

dictionary that contains name as key, value as `torch.Tensor` on cpu

#### safetensors.torch.save_file[[safetensors.torch.save_file]]

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/torch.py#L288)

Saves a dictionary of tensors into `filename` in safetensors format.
There is no mechanism in place to prevent the caller from modifying the data while a file save occurs,
please be wary when calling `save_file` and modifying tensors referenced in the `tensors` dict concurrently;
it may lead to corrupted files.

Example:

```python
from safetensors.torch import save_file
import torch

tensors = {"embedding": torch.zeros((512, 1024)), "attention": torch.zeros((256, 256))}
save_file(tensors, "model.safetensors")
```

**Parameters:**

tensors (`Dict[str, torch.Tensor]`) : The incoming tensors. Tensors need to be contiguous and dense.

filename (`str`, or `os.PathLike`)) : The filename we're saving into.

metadata (`Dict[str, str]`, *optional*, defaults to `None`) : Optional text only metadata you might want to save in your header. For instance it can be useful to specify more about the underlying tensors. This is purely informative and does not affect tensor loading.

**Returns:**

``None``

#### safetensors.torch.save[[safetensors.torch.save]]

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/torch.py#L253)

Saves a dictionary of tensors into raw bytes in safetensors format.

Example:

```python
from safetensors.torch import save
import torch

tensors = {"embedding": torch.zeros((512, 1024)), "attention": torch.zeros((256, 256))}
byte_data = save(tensors)
```

**Parameters:**

tensors (`Dict[str, torch.Tensor]`) : The incoming tensors. Tensors need to be contiguous and dense.

metadata (`Dict[str, str]`, *optional*, defaults to `None`) : Optional text only metadata you might want to save in your header. For instance it can be useful to specify more about the underlying tensors. This is purely informative and does not affect tensor loading.

**Returns:**

``bytes``

The raw bytes representing the format

#### safetensors.torch.load_model[[safetensors.torch.load_model]]

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/torch.py#L194)

Loads a given filename onto a torch model.
This method exists specifically to avoid tensor sharing issues which are
not allowed in `safetensors`. [More information on tensor sharing](../torch_shared_tensors)

**Parameters:**

model (`torch.nn.Module`) : The model to load onto.

filename (`str`, or `os.PathLike`) : The filename location to load the file from.

strict (`bool`, *optional*, defaults to True) : Whether to fail if you're missing keys or having unexpected ones. When false, the function simply returns missing and unexpected names.

device (`Union[str, int]`, *optional*, defaults to `cpu`) : The device where the tensors need to be located after load. available options are all regular torch device locations.

backend (`str`, *optional*, defaults to `"mmap"`) : Storage backend used to serve tensor bytes. `"mmap"` (default) and `"pread"` uses `pread(2)` to read tensor bytes.

**Returns:**

``(missing, unexpected)`

(List[str], List[str])`
`missing` are names in the model which were not modified during loading
`unexpected` are names that are on the file, but weren't used during
the load.

#### safetensors.torch.save_model[[safetensors.torch.save_model]]

[Source](https://github.com/huggingface/safetensors/blob/main/bindings/python/py_src/safetensors/torch.py#L145)

Saves a given torch model to specified filename.
This method exists specifically to avoid tensor sharing issues which are
not allowed in `safetensors`. [More information on tensor sharing](../torch_shared_tensors)

**Parameters:**

model (`torch.nn.Module`) : The model to save on disk.

filename (`str`) : The filename location to save the file

metadata (`Dict[str, str]`, *optional*) : Extra information to save along with the file. Some metadata will be added for each dropped tensors. This information will not be enough to recover the entire shared structure but might help understanding things

force_contiguous (`boolean`, *optional*, defaults to True) : Forcing the state_dict to be saved as contiguous tensors. This has no effect on the correctness of the model, but it could potentially change performance if the layout of the tensor was chosen specifically for that reason.
