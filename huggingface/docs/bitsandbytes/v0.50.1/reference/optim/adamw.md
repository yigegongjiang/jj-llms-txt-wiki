# AdamW

[AdamW](https://hf.co/papers/1711.05101) is a variant of the `Adam` optimizer that separates weight decay from the gradient update based on the observation that the weight decay formulation is different when applied to `SGD` and `Adam`.

bitsandbytes also supports paged optimizers which take advantage of CUDAs unified memory to transfer memory from the GPU to the CPU when GPU memory is exhausted.

## AdamW[[api-class]][[bitsandbytes.optim.AdamW]]

#### bitsandbytes.optim.AdamW[[bitsandbytes.optim.AdamW]]

```python
bitsandbytes.optim.AdamW(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0.01, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096, is_paged = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.1/bitsandbytes/optim/adamw.py#L9)

#### __init__[[bitsandbytes.optim.AdamW.__init__]]

```python
__init__(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0.01, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096, is_paged = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.1/bitsandbytes/optim/adamw.py#L10)

**Parameters:**

params (`torch.Tensor`) : The input parameters to optimize.

lr (`float`, defaults to 1e-3) : The learning rate.

betas (`tuple(float, float)`, defaults to (0.9, 0.999)) : The beta values are the decay rates of the first and second-order moment of the optimizer.

eps (`float`, defaults to 1e-8) : The epsilon value prevents division by zero in the optimizer.

weight_decay (`float`, defaults to 1e-2) : The weight decay value for the optimizer.

amsgrad (`bool`, defaults to `False`) : Whether to use the [AMSGrad](https://hf.co/papers/1904.09237) variant of Adam that uses the maximum of past squared gradients instead.

optim_bits (`int`, defaults to 32) : The number of bits of the optimizer state.

args (`object`, defaults to `None`) : An object with additional arguments.

min_8bit_size (`int`, defaults to 4096) : The minimum number of elements of the parameter tensors for 8-bit optimization.

is_paged (`bool`, defaults to `False`) : Whether the optimizer is a paged optimizer or not.

Base AdamW optimizer.

## AdamW8bit[[bitsandbytes.optim.AdamW8bit]]

#### bitsandbytes.optim.AdamW8bit[[bitsandbytes.optim.AdamW8bit]]

```python
bitsandbytes.optim.AdamW8bit(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0.01, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096, is_paged = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.1/bitsandbytes/optim/adamw.py#L62)

#### __init__[[bitsandbytes.optim.AdamW8bit.__init__]]

```python
__init__(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0.01, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096, is_paged = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.1/bitsandbytes/optim/adamw.py#L63)

**Parameters:**

params (`torch.Tensor`) : The input parameters to optimize.

lr (`float`, defaults to 1e-3) : The learning rate.

betas (`tuple(float, float)`, defaults to (0.9, 0.999)) : The beta values are the decay rates of the first and second-order moment of the optimizer.

eps (`float`, defaults to 1e-8) : The epsilon value prevents division by zero in the optimizer.

weight_decay (`float`, defaults to 1e-2) : The weight decay value for the optimizer.

amsgrad (`bool`, defaults to `False`) : Whether to use the [AMSGrad](https://hf.co/papers/1904.09237) variant of Adam that uses the maximum of past squared gradients instead. Note: This parameter is not supported in AdamW8bit and must be False.

optim_bits (`int`, defaults to 32) : The number of bits of the optimizer state. Note: This parameter is not used in AdamW8bit as it always uses 8-bit optimization.

args (`object`, defaults to `None`) : An object with additional arguments.

min_8bit_size (`int`, defaults to 4096) : The minimum number of elements of the parameter tensors for 8-bit optimization.

is_paged (`bool`, defaults to `False`) : Whether the optimizer is a paged optimizer or not.

8-bit AdamW optimizer.

## AdamW32bit[[bitsandbytes.optim.AdamW32bit]]

#### bitsandbytes.optim.AdamW32bit[[bitsandbytes.optim.AdamW32bit]]

```python
bitsandbytes.optim.AdamW32bit(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0.01, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096, is_paged = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.1/bitsandbytes/optim/adamw.py#L126)

#### __init__[[bitsandbytes.optim.AdamW32bit.__init__]]

```python
__init__(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0.01, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096, is_paged = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.1/bitsandbytes/optim/adamw.py#L127)

**Parameters:**

params (`torch.Tensor`) : The input parameters to optimize.

lr (`float`, defaults to 1e-3) : The learning rate.

betas (`tuple(float, float)`, defaults to (0.9, 0.999)) : The beta values are the decay rates of the first and second-order moment of the optimizer.

eps (`float`, defaults to 1e-8) : The epsilon value prevents division by zero in the optimizer.

weight_decay (`float`, defaults to 1e-2) : The weight decay value for the optimizer.

amsgrad (`bool`, defaults to `False`) : Whether to use the [AMSGrad](https://hf.co/papers/1904.09237) variant of Adam that uses the maximum of past squared gradients instead.

optim_bits (`int`, defaults to 32) : The number of bits of the optimizer state.

args (`object`, defaults to `None`) : An object with additional arguments.

min_8bit_size (`int`, defaults to 4096) : The minimum number of elements of the parameter tensors for 8-bit optimization.

is_paged (`bool`, defaults to `False`) : Whether the optimizer is a paged optimizer or not.

32-bit AdamW optimizer.

## PagedAdamW[[bitsandbytes.optim.PagedAdamW]]

#### bitsandbytes.optim.PagedAdamW[[bitsandbytes.optim.PagedAdamW]]

```python
bitsandbytes.optim.PagedAdamW(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0.01, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.1/bitsandbytes/optim/adamw.py#L179)

#### __init__[[bitsandbytes.optim.PagedAdamW.__init__]]

```python
__init__(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0.01, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.1/bitsandbytes/optim/adamw.py#L180)

**Parameters:**

params (`torch.Tensor`) : The input parameters to optimize.

lr (`float`, defaults to 1e-3) : The learning rate.

betas (`tuple(float, float)`, defaults to (0.9, 0.999)) : The beta values are the decay rates of the first and second-order moment of the optimizer.

eps (`float`, defaults to 1e-8) : The epsilon value prevents division by zero in the optimizer.

weight_decay (`float`, defaults to 1e-2) : The weight decay value for the optimizer.

amsgrad (`bool`, defaults to `False`) : Whether to use the [AMSGrad](https://hf.co/papers/1904.09237) variant of Adam that uses the maximum of past squared gradients instead.

optim_bits (`int`, defaults to 32) : The number of bits of the optimizer state.

args (`object`, defaults to `None`) : An object with additional arguments.

min_8bit_size (`int`, defaults to 4096) : The minimum number of elements of the parameter tensors for 8-bit optimization.

Paged AdamW optimizer.

## PagedAdamW8bit[[bitsandbytes.optim.PagedAdamW8bit]]

#### bitsandbytes.optim.PagedAdamW8bit[[bitsandbytes.optim.PagedAdamW8bit]]

```python
bitsandbytes.optim.PagedAdamW8bit(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0.01, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.1/bitsandbytes/optim/adamw.py#L229)

#### __init__[[bitsandbytes.optim.PagedAdamW8bit.__init__]]

```python
__init__(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0.01, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.1/bitsandbytes/optim/adamw.py#L230)

**Parameters:**

params (`torch.Tensor`) : The input parameters to optimize.

lr (`float`, defaults to 1e-3) : The learning rate.

betas (`tuple(float, float)`, defaults to (0.9, 0.999)) : The beta values are the decay rates of the first and second-order moment of the optimizer.

eps (`float`, defaults to 1e-8) : The epsilon value prevents division by zero in the optimizer.

weight_decay (`float`, defaults to 1e-2) : The weight decay value for the optimizer.

amsgrad (`bool`, defaults to `False`) : Whether to use the [AMSGrad](https://hf.co/papers/1904.09237) variant of Adam that uses the maximum of past squared gradients instead. Note: This parameter is not supported in PagedAdamW8bit and must be False.

optim_bits (`int`, defaults to 32) : The number of bits of the optimizer state. Note: This parameter is not used in PagedAdamW8bit as it always uses 8-bit optimization.

args (`object`, defaults to `None`) : An object with additional arguments.

min_8bit_size (`int`, defaults to 4096) : The minimum number of elements of the parameter tensors for 8-bit optimization.

Paged 8-bit AdamW optimizer.

## PagedAdamW32bit[[bitsandbytes.optim.PagedAdamW32bit]]

#### bitsandbytes.optim.PagedAdamW32bit[[bitsandbytes.optim.PagedAdamW32bit]]

```python
bitsandbytes.optim.PagedAdamW32bit(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0.01, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.1/bitsandbytes/optim/adamw.py#L290)

#### __init__[[bitsandbytes.optim.PagedAdamW32bit.__init__]]

```python
__init__(params, lr = 0.001, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0.01, amsgrad = False, optim_bits = 32, args = None, min_8bit_size = 4096)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.1/bitsandbytes/optim/adamw.py#L291)

**Parameters:**

params (`torch.Tensor`) : The input parameters to optimize.

lr (`float`, defaults to 1e-3) : The learning rate.

betas (`tuple(float, float)`, defaults to (0.9, 0.999)) : The beta values are the decay rates of the first and second-order moment of the optimizer.

eps (`float`, defaults to 1e-8) : The epsilon value prevents division by zero in the optimizer.

weight_decay (`float`, defaults to 1e-2) : The weight decay value for the optimizer.

amsgrad (`bool`, defaults to `False`) : Whether to use the [AMSGrad](https://hf.co/papers/1904.09237) variant of Adam that uses the maximum of past squared gradients instead.

optim_bits (`int`, defaults to 32) : The number of bits of the optimizer state.

args (`object`, defaults to `None`) : An object with additional arguments.

min_8bit_size (`int`, defaults to 4096) : The minimum number of elements of the parameter tensors for 8-bit optimization.

Paged 32-bit AdamW optimizer.
