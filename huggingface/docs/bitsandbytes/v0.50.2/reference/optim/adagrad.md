# AdaGrad

[AdaGrad (Adaptive Gradient)](https://jmlr.org/papers/v12/duchi11a.html) is an adaptive learning rate optimizer. AdaGrad stores a sum of the squared past gradients for each parameter and uses it to scale their learning rate. This allows the learning rate to be automatically lower or higher depending on the magnitude of the gradient, eliminating the need to manually tune the learning rate.

## Adagrad[[api-class]][[bitsandbytes.optim.Adagrad]]

#### bitsandbytes.optim.Adagrad[[bitsandbytes.optim.Adagrad]]

```python
bitsandbytes.optim.Adagrad(params, lr = 0.01, lr_decay = 0, weight_decay = 0, initial_accumulator_value = 0, eps = 1e-10, optim_bits = 32, args = None, min_8bit_size = 4096)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/adagrad.py#L8)

#### __init__[[bitsandbytes.optim.Adagrad.__init__]]

```python
__init__(params, lr = 0.01, lr_decay = 0, weight_decay = 0, initial_accumulator_value = 0, eps = 1e-10, optim_bits = 32, args = None, min_8bit_size = 4096)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/adagrad.py#L9)

**Parameters:**

params (`torch.tensor`) : The input parameters to optimize.

lr (`float`, defaults to 1e-2) : The learning rate.

lr_decay (`int`, defaults to 0) : The learning rate decay.

weight_decay (`float`, defaults to 0.0) : The weight decay value for the optimizer.

initial_accumulator_value (`int`, defaults to 0) : The initial momemtum values.

eps (`float`, defaults to 1e-10) : The epsilon value prevents division by zero in the optimizer.

optim_bits (`int`, defaults to 32) : The number of bits of the optimizer state.

args (`object`, defaults to `None`) : An object with additional arguments.

min_8bit_size (`int`, defaults to 4096) : The minimum number of elements of the parameter tensors for 8-bit optimization.

Base Adagrad optimizer.

## Adagrad8bit[[bitsandbytes.optim.Adagrad8bit]]

#### bitsandbytes.optim.Adagrad8bit[[bitsandbytes.optim.Adagrad8bit]]

```python
bitsandbytes.optim.Adagrad8bit(params, lr = 0.01, lr_decay = 0, weight_decay = 0, initial_accumulator_value = 0, eps = 1e-10, optim_bits = 8, args = None, min_8bit_size = 4096)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/adagrad.py#L67)

#### __init__[[bitsandbytes.optim.Adagrad8bit.__init__]]

```python
__init__(params, lr = 0.01, lr_decay = 0, weight_decay = 0, initial_accumulator_value = 0, eps = 1e-10, optim_bits = 8, args = None, min_8bit_size = 4096)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/adagrad.py#L68)

**Parameters:**

params (`torch.tensor`) : The input parameters to optimize.

lr (`float`, defaults to 1e-2) : The learning rate.

lr_decay (`int`, defaults to 0) : The learning rate decay.

weight_decay (`float`, defaults to 0.0) : The weight decay value for the optimizer.

initial_accumulator_value (`int`, defaults to 0) : The initial momemtum values.

eps (`float`, defaults to 1e-10) : The epsilon value prevents division by zero in the optimizer.

optim_bits (`int`, defaults to 8) : The number of bits of the optimizer state. Note: This parameter is not used in Adagrad8bit as it always uses 8-bit optimization.

args (`object`, defaults to `None`) : An object with additional arguments.

min_8bit_size (`int`, defaults to 4096) : The minimum number of elements of the parameter tensors for 8-bit optimization.

8-bit Adagrad optimizer.

## Adagrad32bit[[bitsandbytes.optim.Adagrad32bit]]

#### bitsandbytes.optim.Adagrad32bit[[bitsandbytes.optim.Adagrad32bit]]

```python
bitsandbytes.optim.Adagrad32bit(params, lr = 0.01, lr_decay = 0, weight_decay = 0, initial_accumulator_value = 0, eps = 1e-10, optim_bits = 32, args = None, min_8bit_size = 4096)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/adagrad.py#L131)

#### __init__[[bitsandbytes.optim.Adagrad32bit.__init__]]

```python
__init__(params, lr = 0.01, lr_decay = 0, weight_decay = 0, initial_accumulator_value = 0, eps = 1e-10, optim_bits = 32, args = None, min_8bit_size = 4096)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/adagrad.py#L132)

**Parameters:**

params (`torch.tensor`) : The input parameters to optimize.

lr (`float`, defaults to 1e-2) : The learning rate.

lr_decay (`int`, defaults to 0) : The learning rate decay.

weight_decay (`float`, defaults to 0.0) : The weight decay value for the optimizer.

initial_accumulator_value (`int`, defaults to 0) : The initial momemtum values.

eps (`float`, defaults to 1e-10) : The epsilon value prevents division by zero in the optimizer.

optim_bits (`int`, defaults to 32) : The number of bits of the optimizer state.

args (`object`, defaults to `None`) : An object with additional arguments.

min_8bit_size (`int`, defaults to 4096) : The minimum number of elements of the parameter tensors for 8-bit optimization.

32-bit Adagrad optimizer.
