# AdaGrad

[AdaGrad (Adaptive Gradient)](https://jmlr.org/papers/v12/duchi11a.html) is an adaptive learning rate optimizer. AdaGrad stores a sum of the squared past gradients for each parameter and uses it to scale their learning rate. This allows the learning rate to be automatically lower or higher depending on the magnitude of the gradient, eliminating the need to manually tune the learning rate.

## Adagrad[[api-class]][[bitsandbytes.optim.Adagrad]]

- **params** (`torch.tensor`) --
  The input parameters to optimize.
- **lr** (`float`, defaults to 1e-2) --
  The learning rate.
- **lr_decay** (`int`, defaults to 0) --
  The learning rate decay.
- **weight_decay** (`float`, defaults to 0.0) --
  The weight decay value for the optimizer.
- **initial_accumulator_value** (`int`, defaults to 0) --
  The initial momemtum values.
- **eps** (`float`, defaults to 1e-10) --
  The epsilon value prevents division by zero in the optimizer.
- **optim_bits** (`int`, defaults to 32) --
  The number of bits of the optimizer state.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.

Base Adagrad optimizer.

## Adagrad8bit[[bitsandbytes.optim.Adagrad8bit]]

- **params** (`torch.tensor`) --
  The input parameters to optimize.
- **lr** (`float`, defaults to 1e-2) --
  The learning rate.
- **lr_decay** (`int`, defaults to 0) --
  The learning rate decay.
- **weight_decay** (`float`, defaults to 0.0) --
  The weight decay value for the optimizer.
- **initial_accumulator_value** (`int`, defaults to 0) --
  The initial momemtum values.
- **eps** (`float`, defaults to 1e-10) --
  The epsilon value prevents division by zero in the optimizer.
- **optim_bits** (`int`, defaults to 8) --
  The number of bits of the optimizer state.
  Note: This parameter is not used in Adagrad8bit as it always uses 8-bit optimization.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.

8-bit Adagrad optimizer.

## Adagrad32bit[[bitsandbytes.optim.Adagrad32bit]]

- **params** (`torch.tensor`) --
  The input parameters to optimize.
- **lr** (`float`, defaults to 1e-2) --
  The learning rate.
- **lr_decay** (`int`, defaults to 0) --
  The learning rate decay.
- **weight_decay** (`float`, defaults to 0.0) --
  The weight decay value for the optimizer.
- **initial_accumulator_value** (`int`, defaults to 0) --
  The initial momemtum values.
- **eps** (`float`, defaults to 1e-10) --
  The epsilon value prevents division by zero in the optimizer.
- **optim_bits** (`int`, defaults to 32) --
  The number of bits of the optimizer state.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.

32-bit Adagrad optimizer.
