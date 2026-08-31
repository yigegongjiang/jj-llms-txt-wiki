# AdEMAMix

[AdEMAMix](https://hf.co/papers/2409.03137) is a variant of the `Adam` optimizer.

bitsandbytes also supports paged optimizers which take advantage of CUDAs unified memory to transfer memory from the GPU to the CPU when GPU memory is exhausted.

## AdEMAMix[[api-class]][[bitsandbytes.optim.AdEMAMix]]

#### bitsandbytes.optim.AdEMAMix[[bitsandbytes.optim.AdEMAMix]]

```python
bitsandbytes.optim.AdEMAMix(params: Iterable, lr: float = 0.001, betas: tuple = (0.9, 0.999, 0.9999), alpha: float = 5.0, t_alpha: typing.Optional[int] = None, t_beta3: typing.Optional[int] = None, eps: float = 1e-08, weight_decay: float = 0.01, optim_bits: typing.Literal[8, 32] = 32, min_8bit_size: int = 4096, is_paged: bool = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/ademamix.py#L107)

#### __init__[[bitsandbytes.optim.AdEMAMix.__init__]]

```python
__init__(params: Iterable, lr: float = 0.001, betas: tuple = (0.9, 0.999, 0.9999), alpha: float = 5.0, t_alpha: typing.Optional[int] = None, t_beta3: typing.Optional[int] = None, eps: float = 1e-08, weight_decay: float = 0.01, optim_bits: typing.Literal[8, 32] = 32, min_8bit_size: int = 4096, is_paged: bool = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/ademamix.py#L108)

## AdEMAMix8bit[[bitsandbytes.optim.AdEMAMix8bit]]

#### bitsandbytes.optim.AdEMAMix8bit[[bitsandbytes.optim.AdEMAMix8bit]]

```python
bitsandbytes.optim.AdEMAMix8bit(params: Iterable, lr: float = 0.001, betas: tuple = (0.9, 0.999, 0.9999), alpha: float = 5.0, t_alpha: typing.Optional[int] = None, t_beta3: typing.Optional[int] = None, eps: float = 1e-08, weight_decay: float = 0.01, min_8bit_size: int = 4096, is_paged: bool = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/ademamix.py#L270)

#### __init__[[bitsandbytes.optim.AdEMAMix8bit.__init__]]

```python
__init__(params: Iterable, lr: float = 0.001, betas: tuple = (0.9, 0.999, 0.9999), alpha: float = 5.0, t_alpha: typing.Optional[int] = None, t_beta3: typing.Optional[int] = None, eps: float = 1e-08, weight_decay: float = 0.01, min_8bit_size: int = 4096, is_paged: bool = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/ademamix.py#L271)

## AdEMAMix32bit[[bitsandbytes.optim.AdEMAMix32bit]]

#### bitsandbytes.optim.AdEMAMix32bit[[bitsandbytes.optim.AdEMAMix32bit]]

```python
bitsandbytes.optim.AdEMAMix32bit(params: Iterable, lr: float = 0.001, betas: tuple = (0.9, 0.999, 0.9999), alpha: float = 5.0, t_alpha: typing.Optional[int] = None, t_beta3: typing.Optional[int] = None, eps: float = 1e-08, weight_decay: float = 0.01, min_8bit_size: int = 4096, is_paged: bool = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/ademamix.py#L355)

#### __init__[[bitsandbytes.optim.AdEMAMix32bit.__init__]]

```python
__init__(params: Iterable, lr: float = 0.001, betas: tuple = (0.9, 0.999, 0.9999), alpha: float = 5.0, t_alpha: typing.Optional[int] = None, t_beta3: typing.Optional[int] = None, eps: float = 1e-08, weight_decay: float = 0.01, min_8bit_size: int = 4096, is_paged: bool = False)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/ademamix.py#L356)

## PagedAdEMAMix[[bitsandbytes.optim.PagedAdEMAMix]]

#### bitsandbytes.optim.PagedAdEMAMix[[bitsandbytes.optim.PagedAdEMAMix]]

```python
bitsandbytes.optim.PagedAdEMAMix(params: Iterable, lr: float = 0.001, betas: tuple = (0.9, 0.999, 0.9999), alpha: float = 5.0, t_alpha: typing.Optional[int] = None, t_beta3: typing.Optional[int] = None, eps: float = 1e-08, weight_decay: float = 0.01, optim_bits: typing.Literal[8, 32] = 32, min_8bit_size: int = 4096)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/ademamix.py#L326)

#### __init__[[bitsandbytes.optim.PagedAdEMAMix.__init__]]

```python
__init__(params: Iterable, lr: float = 0.001, betas: tuple = (0.9, 0.999, 0.9999), alpha: float = 5.0, t_alpha: typing.Optional[int] = None, t_beta3: typing.Optional[int] = None, eps: float = 1e-08, weight_decay: float = 0.01, optim_bits: typing.Literal[8, 32] = 32, min_8bit_size: int = 4096)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/ademamix.py#L327)

## PagedAdEMAMix8bit[[bitsandbytes.optim.PagedAdEMAMix8bit]]

#### bitsandbytes.optim.PagedAdEMAMix8bit[[bitsandbytes.optim.PagedAdEMAMix8bit]]

```python
bitsandbytes.optim.PagedAdEMAMix8bit(params: Iterable, lr: float = 0.001, betas: tuple = (0.9, 0.999, 0.9999), alpha: float = 5.0, t_alpha: typing.Optional[int] = None, t_beta3: typing.Optional[int] = None, eps: float = 1e-08, weight_decay: float = 0.01, min_8bit_size: int = 4096)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/ademamix.py#L299)

#### __init__[[bitsandbytes.optim.PagedAdEMAMix8bit.__init__]]

```python
__init__(params: Iterable, lr: float = 0.001, betas: tuple = (0.9, 0.999, 0.9999), alpha: float = 5.0, t_alpha: typing.Optional[int] = None, t_beta3: typing.Optional[int] = None, eps: float = 1e-08, weight_decay: float = 0.01, min_8bit_size: int = 4096)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/ademamix.py#L300)

## PagedAdEMAMix32bit[[bitsandbytes.optim.PagedAdEMAMix32bit]]

#### bitsandbytes.optim.PagedAdEMAMix32bit[[bitsandbytes.optim.PagedAdEMAMix32bit]]

```python
bitsandbytes.optim.PagedAdEMAMix32bit(params: Iterable, lr: float = 0.001, betas: tuple = (0.9, 0.999, 0.9999), alpha: float = 5.0, t_alpha: typing.Optional[int] = None, t_beta3: typing.Optional[int] = None, eps: float = 1e-08, weight_decay: float = 0.01, min_8bit_size: int = 4096)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/ademamix.py#L386)

#### __init__[[bitsandbytes.optim.PagedAdEMAMix32bit.__init__]]

```python
__init__(params: Iterable, lr: float = 0.001, betas: tuple = (0.9, 0.999, 0.9999), alpha: float = 5.0, t_alpha: typing.Optional[int] = None, t_beta3: typing.Optional[int] = None, eps: float = 1e-08, weight_decay: float = 0.01, min_8bit_size: int = 4096)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/optim/ademamix.py#L387)
