# Adam

[Adam (Adaptive moment estimation)](https://hf.co/papers/1412.6980) is an adaptive learning rate optimizer, combining ideas from `SGD` with momentum and `RMSprop` to automatically scale the learning rate:

- a weighted average of the past gradients to provide direction (first-moment)
- a weighted average of the *squared* past gradients to adapt the learning rate to each parameter (second-moment)

bitsandbytes also supports paged optimizers which take advantage of CUDAs unified memory to transfer memory from the GPU to the CPU when GPU memory is exhausted.

## Adam[[api-class]][[bitsandbytes.optim.Adam]]

#### bitsandbytes.optim.Adam[[bitsandbytes.optim.Adam]]

```python
bitsandbytes.optim.Adam(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096, is_paged = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/adam.py#L9)

#### __init__[[bitsandbytes.optim.Adam.__init__]]

```python
__init__(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096, is_paged = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/adam.py#L10)

**Parameters:**

params (`torch.tensor`) : The input parameters to optimize.

lr (`float`, defaults to 1e-3) : The learning rate.

betas (`tuple(float, float)`, defaults to (0.9, 0.999)) : The beta values are the decay rates of the first and second-order moment of the optimizer.

eps (`float`, defaults to 1e-8) : The epsilon value prevents division by zero in the optimizer.

weight_decay (`float`, defaults to 0.0) : The weight decay value for the optimizer.

amsgrad (`bool`, defaults to `False`) : Whether to use the [AMSGrad](https://hf.co/papers/1904.09237) variant of Adam that uses the maximum of past squared gradients instead.

optim_bits (`int`, defaults to 32) : The number of bits of the optimizer state.

args (`object`, defaults to `None`) : An object with additional arguments.

min_8bit_size (`int`, defaults to 4096) : The minimum number of elements of the parameter tensors for 8-bit optimization.

is_paged (`bool`, defaults to `False`) : Whether the optimizer is a paged optimizer or not.

Base Adam optimizer.

## Adam8bit[[bitsandbytes.optim.Adam8bit]]

#### bitsandbytes.optim.Adam8bit[[bitsandbytes.optim.Adam8bit]]

```python
bitsandbytes.optim.Adam8bit(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096, is_paged = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/adam.py#L62)

#### __init__[[bitsandbytes.optim.Adam8bit.__init__]]

```python
__init__(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096, is_paged = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/adam.py#L63)

**Parameters:**

params (`torch.tensor`) : The input parameters to optimize.

lr (`float`, defaults to 1e-3) : The learning rate.

betas (`tuple(float, float)`, defaults to (0.9, 0.999)) : The beta values are the decay rates of the first and second-order moment of the optimizer.

eps (`float`, defaults to 1e-8) : The epsilon value prevents division by zero in the optimizer.

weight_decay (`float`, defaults to 0.0) : The weight decay value for the optimizer.

amsgrad (`bool`, defaults to `False`) : Whether to use the [AMSGrad](https://hf.co/papers/1904.09237) variant of Adam that uses the maximum of past squared gradients instead. Note: This parameter is not supported in Adam8bit and must be False.

optim_bits (`int`, defaults to 32) : The number of bits of the optimizer state. Note: This parameter is not used in Adam8bit as it always uses 8-bit optimization.

args (`object`, defaults to `None`) : An object with additional arguments.

min_8bit_size (`int`, defaults to 4096) : The minimum number of elements of the parameter tensors for 8-bit optimization.

is_paged (`bool`, defaults to `False`) : Whether the optimizer is a paged optimizer or not.

8-bit Adam optimizer.

## Adam32bit[[bitsandbytes.optim.Adam32bit]]

#### bitsandbytes.optim.Adam32bit[[bitsandbytes.optim.Adam32bit]]

```python
bitsandbytes.optim.Adam32bit(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096, is_paged = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/adam.py#L126)

#### __init__[[bitsandbytes.optim.Adam32bit.__init__]]

```python
__init__(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096, is_paged = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/adam.py#L127)

**Parameters:**

params (`torch.tensor`) : The input parameters to optimize.

lr (`float`, defaults to 1e-3) : The learning rate.

betas (`tuple(float, float)`, defaults to (0.9, 0.999)) : The beta values are the decay rates of the first and second-order moment of the optimizer.

eps (`float`, defaults to 1e-8) : The epsilon value prevents division by zero in the optimizer.

weight_decay (`float`, defaults to 0.0) : The weight decay value for the optimizer.

amsgrad (`bool`, defaults to `False`) : Whether to use the [AMSGrad](https://hf.co/papers/1904.09237) variant of Adam that uses the maximum of past squared gradients instead.

optim_bits (`int`, defaults to 32) : The number of bits of the optimizer state.

args (`object`, defaults to `None`) : An object with additional arguments.

min_8bit_size (`int`, defaults to 4096) : The minimum number of elements of the parameter tensors for 8-bit optimization.

is_paged (`bool`, defaults to `False`) : Whether the optimizer is a paged optimizer or not.

32-bit Adam optimizer.

## PagedAdam[[bitsandbytes.optim.PagedAdam]]

#### bitsandbytes.optim.PagedAdam[[bitsandbytes.optim.PagedAdam]]

```python
bitsandbytes.optim.PagedAdam(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096, is_paged = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/adam.py#L179)

#### __init__[[bitsandbytes.optim.PagedAdam.__init__]]

```python
__init__(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096, is_paged = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/adam.py#L180)

**Parameters:**

params (`torch.tensor`) : The input parameters to optimize.

lr (`float`, defaults to 1e-3) : The learning rate.

betas (`tuple(float, float)`, defaults to (0.9, 0.999)) : The beta values are the decay rates of the first and second-order moment of the optimizer.

eps (`float`, defaults to 1e-8) : The epsilon value prevents division by zero in the optimizer.

weight_decay (`float`, defaults to 0.0) : The weight decay value for the optimizer.

amsgrad (`bool`, defaults to `False`) : Whether to use the [AMSGrad](https://hf.co/papers/1904.09237) variant of Adam that uses the maximum of past squared gradients instead.

optim_bits (`int`, defaults to 32) : The number of bits of the optimizer state.

args (`object`, defaults to `None`) : An object with additional arguments.

min_8bit_size (`int`, defaults to 4096) : The minimum number of elements of the parameter tensors for 8-bit optimization.

is_paged (`bool`, defaults to `False`) : Whether the optimizer is a paged optimizer or not.

Paged Adam optimizer.

## PagedAdam8bit[[bitsandbytes.optim.PagedAdam8bit]]

#### bitsandbytes.optim.PagedAdam8bit[[bitsandbytes.optim.PagedAdam8bit]]

```python
bitsandbytes.optim.PagedAdam8bit(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096, is_paged = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/adam.py#L232)

#### __init__[[bitsandbytes.optim.PagedAdam8bit.__init__]]

```python
__init__(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096, is_paged = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/adam.py#L233)

**Parameters:**

params (`torch.tensor`) : The input parameters to optimize.

lr (`float`, defaults to 1e-3) : The learning rate.

betas (`tuple(float, float)`, defaults to (0.9, 0.999)) : The beta values are the decay rates of the first and second-order moment of the optimizer.

eps (`float`, defaults to 1e-8) : The epsilon value prevents division by zero in the optimizer.

weight_decay (`float`, defaults to 0.0) : The weight decay value for the optimizer.

amsgrad (`bool`, defaults to `False`) : Whether to use the [AMSGrad](https://hf.co/papers/1904.09237) variant of Adam that uses the maximum of past squared gradients instead. Note: This parameter is not supported in PagedAdam8bit and must be False.

optim_bits (`int`, defaults to 32) : The number of bits of the optimizer state. Note: This parameter is not used in PagedAdam8bit as it always uses 8-bit optimization.

args (`object`, defaults to `None`) : An object with additional arguments.

min_8bit_size (`int`, defaults to 4096) : The minimum number of elements of the parameter tensors for 8-bit optimization.

is_paged (`bool`, defaults to `False`) : Whether the optimizer is a paged optimizer or not.

8-bit paged Adam optimizer.

## PagedAdam32bit[[bitsandbytes.optim.PagedAdam32bit]]

#### bitsandbytes.optim.PagedAdam32bit[[bitsandbytes.optim.PagedAdam32bit]]

```python
bitsandbytes.optim.PagedAdam32bit(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096, is_paged = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/adam.py#L296)

#### __init__[[bitsandbytes.optim.PagedAdam32bit.__init__]]

```python
__init__(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096, is_paged = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/adam.py#L297)

**Parameters:**

params (`torch.tensor`) : The input parameters to optimize.

lr (`float`, defaults to 1e-3) : The learning rate.

betas (`tuple(float, float)`, defaults to (0.9, 0.999)) : The beta values are the decay rates of the first and second-order moment of the optimizer.

eps (`float`, defaults to 1e-8) : The epsilon value prevents division by zero in the optimizer.

weight_decay (`float`, defaults to 0.0) : The weight decay value for the optimizer.

amsgrad (`bool`, defaults to `False`) : Whether to use the [AMSGrad](https://hf.co/papers/1904.09237) variant of Adam that uses the maximum of past squared gradients instead.

optim_bits (`int`, defaults to 32) : The number of bits of the optimizer state.

args (`object`, defaults to `None`) : An object with additional arguments.

min_8bit_size (`int`, defaults to 4096) : The minimum number of elements of the parameter tensors for 8-bit optimization.

is_paged (`bool`, defaults to `False`) : Whether the optimizer is a paged optimizer or not.

Paged 32-bit Adam optimizer.
