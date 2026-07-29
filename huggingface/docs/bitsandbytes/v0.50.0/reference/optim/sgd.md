# SGD

Stochastic gradient descent (SGD) is a basic gradient descent optimizer to minimize loss given a set of model parameters and updates the parameters in the opposite direction of the gradient. The update is performed on a randomly sampled mini-batch of data from the dataset.

bitsandbytes also supports momentum and Nesterov momentum to accelerate SGD by adding a weighted average of past gradients to the current gradient.

## SGD[[api-class]][[bitsandbytes.optim.SGD]]

- **params** (`torch.tensor`) --
  The input parameters to optimize.
- **lr** (`float`) --
  The learning rate.
- **momentum** (`float`, defaults to 0) --
  The momentum value speeds up the optimizer by taking bigger steps.
- **dampening** (`float`, defaults to 0) --
  The dampening value reduces the momentum of the optimizer.
- **weight_decay** (`float`, defaults to 0.0) --
  The weight decay value for the optimizer.
- **nesterov** (`bool`, defaults to `False`) --
  Whether to use Nesterov momentum.
- **optim_bits** (`int`, defaults to 32) --
  The number of bits of the optimizer state.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.

Base SGD optimizer.

## SGD8bit[[bitsandbytes.optim.SGD8bit]]

- **params** (`torch.tensor`) --
  The input parameters to optimize.
- **lr** (`float`) --
  The learning rate.
- **momentum** (`float`, defaults to 0) --
  The momentum value speeds up the optimizer by taking bigger steps.
- **dampening** (`float`, defaults to 0) --
  The dampening value reduces the momentum of the optimizer.
- **weight_decay** (`float`, defaults to 0.0) --
  The weight decay value for the optimizer.
- **nesterov** (`bool`, defaults to `False`) --
  Whether to use Nesterov momentum.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.

8-bit SGD optimizer.

## SGD32bit[[bitsandbytes.optim.SGD32bit]]

- **params** (`torch.tensor`) --
  The input parameters to optimize.
- **lr** (`float`) --
  The learning rate.
- **momentum** (`float`, defaults to 0) --
  The momentum value speeds up the optimizer by taking bigger steps.
- **dampening** (`float`, defaults to 0) --
  The dampening value reduces the momentum of the optimizer.
- **weight_decay** (`float`, defaults to 0.0) --
  The weight decay value for the optimizer.
- **nesterov** (`bool`, defaults to `False`) --
  Whether to use Nesterov momentum.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.

32-bit SGD optimizer.
