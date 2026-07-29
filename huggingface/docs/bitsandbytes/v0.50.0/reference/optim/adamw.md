# AdamW

[AdamW](https://hf.co/papers/1711.05101) is a variant of the `Adam` optimizer that separates weight decay from the gradient update based on the observation that the weight decay formulation is different when applied to `SGD` and `Adam`.

bitsandbytes also supports paged optimizers which take advantage of CUDAs unified memory to transfer memory from the GPU to the CPU when GPU memory is exhausted.

## AdamW[[api-class]][[bitsandbytes.optim.AdamW]]

- **params** (`torch.Tensor`) --
  The input parameters to optimize.
- **lr** (`float`, defaults to 1e-3) --
  The learning rate.
- **betas** (`tuple(float, float)`, defaults to (0.9, 0.999)) --
  The beta values are the decay rates of the first and second-order moment of the optimizer.
- **eps** (`float`, defaults to 1e-8) --
  The epsilon value prevents division by zero in the optimizer.
- **weight_decay** (`float`, defaults to 1e-2) --
  The weight decay value for the optimizer.
- **amsgrad** (`bool`, defaults to `False`) --
  Whether to use the [AMSGrad](https://hf.co/papers/1904.09237) variant of Adam that uses the maximum of past squared gradients instead.
- **optim_bits** (`int`, defaults to 32) --
  The number of bits of the optimizer state.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.
- **is_paged** (`bool`, defaults to `False`) --
  Whether the optimizer is a paged optimizer or not.

Base AdamW optimizer.

## AdamW8bit[[bitsandbytes.optim.AdamW8bit]]

- **params** (`torch.Tensor`) --
  The input parameters to optimize.
- **lr** (`float`, defaults to 1e-3) --
  The learning rate.
- **betas** (`tuple(float, float)`, defaults to (0.9, 0.999)) --
  The beta values are the decay rates of the first and second-order moment of the optimizer.
- **eps** (`float`, defaults to 1e-8) --
  The epsilon value prevents division by zero in the optimizer.
- **weight_decay** (`float`, defaults to 1e-2) --
  The weight decay value for the optimizer.
- **amsgrad** (`bool`, defaults to `False`) --
  Whether to use the [AMSGrad](https://hf.co/papers/1904.09237) variant of Adam that uses the maximum of past squared gradients instead.
  Note: This parameter is not supported in AdamW8bit and must be False.
- **optim_bits** (`int`, defaults to 32) --
  The number of bits of the optimizer state.
  Note: This parameter is not used in AdamW8bit as it always uses 8-bit optimization.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.
- **is_paged** (`bool`, defaults to `False`) --
  Whether the optimizer is a paged optimizer or not.

8-bit AdamW optimizer.

## AdamW32bit[[bitsandbytes.optim.AdamW32bit]]

- **params** (`torch.Tensor`) --
  The input parameters to optimize.
- **lr** (`float`, defaults to 1e-3) --
  The learning rate.
- **betas** (`tuple(float, float)`, defaults to (0.9, 0.999)) --
  The beta values are the decay rates of the first and second-order moment of the optimizer.
- **eps** (`float`, defaults to 1e-8) --
  The epsilon value prevents division by zero in the optimizer.
- **weight_decay** (`float`, defaults to 1e-2) --
  The weight decay value for the optimizer.
- **amsgrad** (`bool`, defaults to `False`) --
  Whether to use the [AMSGrad](https://hf.co/papers/1904.09237) variant of Adam that uses the maximum of past squared gradients instead.
- **optim_bits** (`int`, defaults to 32) --
  The number of bits of the optimizer state.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.
- **is_paged** (`bool`, defaults to `False`) --
  Whether the optimizer is a paged optimizer or not.

32-bit AdamW optimizer.

## PagedAdamW[[bitsandbytes.optim.PagedAdamW]]

- **params** (`torch.Tensor`) --
  The input parameters to optimize.
- **lr** (`float`, defaults to 1e-3) --
  The learning rate.
- **betas** (`tuple(float, float)`, defaults to (0.9, 0.999)) --
  The beta values are the decay rates of the first and second-order moment of the optimizer.
- **eps** (`float`, defaults to 1e-8) --
  The epsilon value prevents division by zero in the optimizer.
- **weight_decay** (`float`, defaults to 1e-2) --
  The weight decay value for the optimizer.
- **amsgrad** (`bool`, defaults to `False`) --
  Whether to use the [AMSGrad](https://hf.co/papers/1904.09237) variant of Adam that uses the maximum of past squared gradients instead.
- **optim_bits** (`int`, defaults to 32) --
  The number of bits of the optimizer state.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.

Paged AdamW optimizer.

## PagedAdamW8bit[[bitsandbytes.optim.PagedAdamW8bit]]

- **params** (`torch.Tensor`) --
  The input parameters to optimize.
- **lr** (`float`, defaults to 1e-3) --
  The learning rate.
- **betas** (`tuple(float, float)`, defaults to (0.9, 0.999)) --
  The beta values are the decay rates of the first and second-order moment of the optimizer.
- **eps** (`float`, defaults to 1e-8) --
  The epsilon value prevents division by zero in the optimizer.
- **weight_decay** (`float`, defaults to 1e-2) --
  The weight decay value for the optimizer.
- **amsgrad** (`bool`, defaults to `False`) --
  Whether to use the [AMSGrad](https://hf.co/papers/1904.09237) variant of Adam that uses the maximum of past squared gradients instead.
  Note: This parameter is not supported in PagedAdamW8bit and must be False.
- **optim_bits** (`int`, defaults to 32) --
  The number of bits of the optimizer state.
  Note: This parameter is not used in PagedAdamW8bit as it always uses 8-bit optimization.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.

Paged 8-bit AdamW optimizer.

## PagedAdamW32bit[[bitsandbytes.optim.PagedAdamW32bit]]

- **params** (`torch.Tensor`) --
  The input parameters to optimize.
- **lr** (`float`, defaults to 1e-3) --
  The learning rate.
- **betas** (`tuple(float, float)`, defaults to (0.9, 0.999)) --
  The beta values are the decay rates of the first and second-order moment of the optimizer.
- **eps** (`float`, defaults to 1e-8) --
  The epsilon value prevents division by zero in the optimizer.
- **weight_decay** (`float`, defaults to 1e-2) --
  The weight decay value for the optimizer.
- **amsgrad** (`bool`, defaults to `False`) --
  Whether to use the [AMSGrad](https://hf.co/papers/1904.09237) variant of Adam that uses the maximum of past squared gradients instead.
- **optim_bits** (`int`, defaults to 32) --
  The number of bits of the optimizer state.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.

Paged 32-bit AdamW optimizer.
