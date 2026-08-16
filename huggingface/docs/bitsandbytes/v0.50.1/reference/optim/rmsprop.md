# RMSprop

RMSprop is an adaptive learning rate optimizer that is very similar to `Adagrad`. RMSprop stores a *weighted average* of the squared past gradients for each parameter and uses it to scale their learning rate. This allows the learning rate to be automatically lower or higher depending on the magnitude of the gradient, and it prevents the learning rate from diminishing.

## RMSprop[[api-class]][[bitsandbytes.optim.RMSprop]]

#### bitsandbytes.optim.RMSprop[[bitsandbytes.optim.RMSprop]]

```python
bitsandbytes.optim.RMSprop(params, lr = 0.01, alpha = 0.99, eps = 1e-08, weight_decay = 0, momentum = 0, centered = False, optim_bits = 32, args = None, min_8bit_size = 4096)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.1/bitsandbytes/optim/rmsprop.py#L8)

## RMSprop8bit[[bitsandbytes.optim.RMSprop8bit]]

#### bitsandbytes.optim.RMSprop8bit[[bitsandbytes.optim.RMSprop8bit]]

```python
bitsandbytes.optim.RMSprop8bit(params, lr = 0.01, alpha = 0.99, eps = 1e-08, weight_decay = 0, momentum = 0, centered = False, args = None, min_8bit_size = 4096)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.1/bitsandbytes/optim/rmsprop.py#L64)

## RMSprop32bit[[bitsandbytes.optim.RMSprop32bit]]

#### bitsandbytes.optim.RMSprop32bit[[bitsandbytes.optim.RMSprop32bit]]

```python
bitsandbytes.optim.RMSprop32bit(params, lr = 0.01, alpha = 0.99, eps = 1e-08, weight_decay = 0, momentum = 0, centered = False, args = None, min_8bit_size = 4096)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.1/bitsandbytes/optim/rmsprop.py#L117)
