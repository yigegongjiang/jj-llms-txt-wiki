# LARS

[LARS (Layer-wise Adaptive Rate Scaling)](https:/hf.co/papers/1708.03888) is an optimizer designed for training with large batch sizes to accelerate training. LARS uses a separate learning rate for each *layer* instead of each parameter. The learning rate is calculated from a *trust ratio* between the weight and gradient norm in a layer. This helps calibrate a stable update size.

## LARS[[api-class]][[bitsandbytes.optim.LARS]]

- **params** (`torch.tensor`) --
  The input parameters to optimize.
- **lr** (`float`) --
  The learning rate.
- **momentum** (`float`, defaults to 0) --
  The momentum value speeds up the optimizer by taking bigger steps.
- **dampening** (`float`, defaults to 0) --
  The dampening value reduces the momentum of the optimizer.
- **weight_decay** (`float`, defaults to 1e-2) --
  The weight decay value for the optimizer.
- **nesterov** (`bool`, defaults to `False`) --
  Whether to use Nesterov momentum.
- **optim_bits** (`int`, defaults to 32) --
  The number of bits of the optimizer state.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.
- **max_unorm** (`float`, defaults to 0.02) --
  The maximum gradient norm.

Base LARS optimizer.

## LARS8bit[[bitsandbytes.optim.LARS8bit]]

- **params** (`torch.tensor`) --
  The input parameters to optimize.
- **lr** (`float`) --
  The learning rate.
- **momentum** (`float`, defaults to 0) --
  The momentum value speeds up the optimizer by taking bigger steps.
- **dampening** (`float`, defaults to 0) --
  The dampening value reduces the momentum of the optimizer.
- **weight_decay** (`float`, defaults to 1e-2) --
  The weight decay value for the optimizer.
- **nesterov** (`bool`, defaults to `False`) --
  Whether to use Nesterov momentum.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.
- **max_unorm** (`float`, defaults to 0.02) --
  The maximum gradient norm.

8-bit LARS optimizer.

## LARS32bit[[bitsandbytes.optim.LARS32bit]]

- **params** (`torch.tensor`) --
  The input parameters to optimize.
- **lr** (`float`) --
  The learning rate.
- **momentum** (`float`, defaults to 0) --
  The momentum value speeds up the optimizer by taking bigger steps.
- **dampening** (`float`, defaults to 0) --
  The dampening value reduces the momentum of the optimizer.
- **weight_decay** (`float`, defaults to 1e-2) --
  The weight decay value for the optimizer.
- **nesterov** (`bool`, defaults to `False`) --
  Whether to use Nesterov momentum.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.
- **max_unorm** (`float`, defaults to 0.02) --
  The maximum gradient norm.

32-bit LARS optimizer.
