# LAMB

[LAMB (Layerwise adaptive large batch optimization)](https://hf.co/papers/1904.00962) is an adaptive optimizer designed for training with large batch sizes to accelerate training, combining ideas from `LARS` and `Adam` to automatically scale the learning rate for each layer:

- calculates a *trust ratio* between the weight and gradient norm in a layer and clips the ratio to prevent overly large or small updates
- updates weights with the first and second-moments

## LAMB[[api-class]][[bitsandbytes.optim.LAMB]]

#### bitsandbytes.optim.LAMB[[bitsandbytes.optim.LAMB]]

```python
bitsandbytes.optim.LAMB(params, lr = 0.001, bias_correction = True, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0, amsgrad = False, adam_w_mode = True, optim_bits = 32, args = None, min_8bit_size = 4096, max_unorm = 1.0)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/lamb.py#L8)

#### __init__[[bitsandbytes.optim.LAMB.__init__]]

```python
__init__(params, lr = 0.001, bias_correction = True, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0, amsgrad = False, adam_w_mode = True, optim_bits = 32, args = None, min_8bit_size = 4096, max_unorm = 1.0)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/lamb.py#L9)

**Parameters:**

params (`torch.tensor`) : The input parameters to optimize.

lr (`float`, defaults to 1e-3) : The learning rate.

bias_correction (`bool`, defaults to `True`) : Whether to apply bias correction to the first and second-order moments.

betas (`tuple(float, float)`, defaults to (0.9, 0.999)) : The beta values are the decay rates of the first and second-order moment of the optimizer.

eps (`float`, defaults to 1e-8) : The epsilon value prevents division by zero in the optimizer.

weight_decay (`float`, defaults to 1e-2) : The weight decay value for the optimizer.

amsgrad (`bool`, defaults to `False`) : Whether to use the [AMSGrad](https://hf.co/papers/1904.09237) variant of Adam that uses the maximum of past squared gradients instead.

adam_w_mode (`bool`, defaults to `True`) : Whether to use the AdamW variant.

optim_bits (`int`, defaults to 32) : The number of bits of the optimizer state.

args (`object`, defaults to `None`) : An object with additional arguments.

min_8bit_size (`int`, defaults to 4096) : The minimum number of elements of the parameter tensors for 8-bit optimization.

max_unorm (`float`, defaults to 1.0) : The maximum gradient norm.

Base LAMB optimizer.

## LAMB8bit[[bitsandbytes.optim.LAMB8bit]]

#### bitsandbytes.optim.LAMB8bit[[bitsandbytes.optim.LAMB8bit]]

```python
bitsandbytes.optim.LAMB8bit(params, lr = 0.001, bias_correction = True, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0, amsgrad = False, adam_w_mode = True, args = None, min_8bit_size = 4096, max_unorm = 1.0)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/lamb.py#L67)

#### __init__[[bitsandbytes.optim.LAMB8bit.__init__]]

```python
__init__(params, lr = 0.001, bias_correction = True, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0, amsgrad = False, adam_w_mode = True, args = None, min_8bit_size = 4096, max_unorm = 1.0)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/lamb.py#L68)

**Parameters:**

params (`torch.tensor`) : The input parameters to optimize.

lr (`float`, defaults to 1e-3) : The learning rate.

bias_correction (`bool`, defaults to `True`) : Whether to apply bias correction to the first and second-order moments.

betas (`tuple(float, float)`, defaults to (0.9, 0.999)) : The beta values are the decay rates of the first and second-order moment of the optimizer.

eps (`float`, defaults to 1e-8) : The epsilon value prevents division by zero in the optimizer.

weight_decay (`float`, defaults to 1e-2) : The weight decay value for the optimizer.

amsgrad (`bool`, defaults to `False`) : Whether to use the [AMSGrad](https://hf.co/papers/1904.09237) variant of Adam that uses the maximum of past squared gradients instead. Note: This parameter is not supported in LAMB8bit and must be False.

adam_w_mode (`bool`, defaults to `True`) : Whether to use the AdamW variant.

args (`object`, defaults to `None`) : An object with additional arguments.

min_8bit_size (`int`, defaults to 4096) : The minimum number of elements of the parameter tensors for 8-bit optimization.

max_unorm (`float`, defaults to 1.0) : The maximum update norm for trust-ratio clipping. Note: This parameter is not supported in LAMB8bit and must be left at the default 1.0. The 8-bit blockwise update does not implement update-norm clipping; it is honored by the 32-bit LAMB / LAMB32bit optimizers.

8-bit LAMB optimizer.

## LAMB32bit[[bitsandbytes.optim.LAMB32bit]]

#### bitsandbytes.optim.LAMB32bit[[bitsandbytes.optim.LAMB32bit]]

```python
bitsandbytes.optim.LAMB32bit(params, lr = 0.001, bias_correction = True, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0, amsgrad = False, adam_w_mode = True, args = None, min_8bit_size = 4096, max_unorm = 1.0)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/lamb.py#L137)

#### __init__[[bitsandbytes.optim.LAMB32bit.__init__]]

```python
__init__(params, lr = 0.001, bias_correction = True, betas = (0.9, 0.999), eps = 1e-08, weight_decay = 0, amsgrad = False, adam_w_mode = True, args = None, min_8bit_size = 4096, max_unorm = 1.0)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/lamb.py#L138)

**Parameters:**

params (`torch.tensor`) : The input parameters to optimize.

lr (`float`, defaults to 1e-3) : The learning rate.

bias_correction (`bool`, defaults to `True`) : Whether to apply bias correction to the first and second-order moments.

betas (`tuple(float, float)`, defaults to (0.9, 0.999)) : The beta values are the decay rates of the first and second-order moment of the optimizer.

eps (`float`, defaults to 1e-8) : The epsilon value prevents division by zero in the optimizer.

weight_decay (`float`, defaults to 1e-2) : The weight decay value for the optimizer.

amsgrad (`bool`, defaults to `False`) : Whether to use the [AMSGrad](https://hf.co/papers/1904.09237) variant of Adam that uses the maximum of past squared gradients instead.

adam_w_mode (`bool`, defaults to `True`) : Whether to use the AdamW variant.

args (`object`, defaults to `None`) : An object with additional arguments.

min_8bit_size (`int`, defaults to 4096) : The minimum number of elements of the parameter tensors for 8-bit optimization.

max_unorm (`float`, defaults to 1.0) : The maximum gradient norm.

32-bit LAMB optimizer.
