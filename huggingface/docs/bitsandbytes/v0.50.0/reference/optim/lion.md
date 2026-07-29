# Lion

[Lion (Evolved Sign Momentum)](https://hf.co/papers/2302.06675) is a unique optimizer that uses the sign of the gradient to determine the update direction of the momentum. This makes Lion more memory-efficient and faster than `AdamW` which tracks and store the first and second-order moments.

## Lion[[api-class]][[bitsandbytes.optim.Lion]]

- **params** (`torch.tensor`) --
  The input parameters to optimize.
- **lr** (`float`, defaults to 1e-4) --
  The learning rate.
- **betas** (`tuple(float, float)`, defaults to (0.9, 0.999)) --
  The beta values are the decay rates of the first and second-order moment of the optimizer.
- **weight_decay** (`float`, defaults to 0) --
  The weight decay value for the optimizer.
- **optim_bits** (`int`, defaults to 32) --
  The number of bits of the optimizer state.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.
- **is_paged** (`bool`, defaults to `False`) --
  Whether the optimizer is a paged optimizer or not.

Base Lion optimizer.

## Lion8bit[[bitsandbytes.optim.Lion8bit]]

- **params** (`torch.tensor`) --
  The input parameters to optimize.
- **lr** (`float`, defaults to 1e-4) --
  The learning rate.
- **betas** (`tuple(float, float)`, defaults to (0.9, 0.999)) --
  The beta values are the decay rates of the first and second-order moment of the optimizer.
- **weight_decay** (`float`, defaults to 0) --
  The weight decay value for the optimizer.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.
- **is_paged** (`bool`, defaults to `False`) --
  Whether the optimizer is a paged optimizer or not.

8-bit Lion optimizer.

## Lion32bit[[bitsandbytes.optim.Lion32bit]]

- **params** (`torch.tensor`) --
  The input parameters to optimize.
- **lr** (`float`, defaults to 1e-4) --
  The learning rate.
- **betas** (`tuple(float, float)`, defaults to (0.9, 0.999)) --
  The beta values are the decay rates of the first and second-order moment of the optimizer.
- **weight_decay** (`float`, defaults to 0) --
  The weight decay value for the optimizer.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.
- **is_paged** (`bool`, defaults to `False`) --
  Whether the optimizer is a paged optimizer or not.

32-bit Lion optimizer.

## PagedLion[[bitsandbytes.optim.PagedLion]]

- **params** (`torch.tensor`) --
  The input parameters to optimize.
- **lr** (`float`, defaults to 1e-4) --
  The learning rate.
- **betas** (`tuple(float, float)`, defaults to (0.9, 0.999)) --
  The beta values are the decay rates of the first and second-order moment of the optimizer.
- **weight_decay** (`float`, defaults to 0) --
  The weight decay value for the optimizer.
- **optim_bits** (`int`, defaults to 32) --
  The number of bits of the optimizer state.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.

Paged Lion optimizer.

## PagedLion8bit[[bitsandbytes.optim.PagedLion8bit]]

- **params** (`torch.tensor`) --
  The input parameters to optimize.
- **lr** (`float`, defaults to 1e-4) --
  The learning rate.
- **betas** (`tuple(float, float)`, defaults to (0.9, 0.999)) --
  The beta values are the decay rates of the first and second-order moment of the optimizer.
- **weight_decay** (`float`, defaults to 0) --
  The weight decay value for the optimizer.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.

Paged 8-bit Lion optimizer.

## PagedLion32bit[[bitsandbytes.optim.PagedLion32bit]]

- **params** (`torch.tensor`) --
  The input parameters to optimize.
- **lr** (`float`, defaults to 1e-4) --
  The learning rate.
- **betas** (`tuple(float, float)`, defaults to (0.9, 0.999)) --
  The beta values are the decay rates of the first and second-order moment of the optimizer.
- **weight_decay** (`float`, defaults to 0) --
  The weight decay value for the optimizer.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.

Paged 32-bit Lion optimizer.
