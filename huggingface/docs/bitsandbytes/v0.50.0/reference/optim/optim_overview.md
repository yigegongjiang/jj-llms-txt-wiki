# Overview

[8-bit optimizers](https://hf.co/papers/2110.02861) reduce the memory footprint of 32-bit optimizers without any performance degradation which means you can train large models with many parameters faster. At the core of 8-bit optimizers is block-wise quantization which enables quantization accuracy, computational efficiency, and stability.

bitsandbytes provides 8-bit optimizers through the base `Optimizer8bit` class, and additionally provides `Optimizer2State` and `Optimizer1State` for 2-state (for example, `Adam`) and 1-state (for example, `Adagrad`) optimizers respectively. To provide custom optimizer hyperparameters, use the `GlobalOptimManager` class to configure the optimizer.

## Optimizer8bit[[bitsandbytes.optim.optimizer.Optimizer8bit]]

- **params** (`torch.Tensor`) --
  The input parameters to optimize.
- **optim_bits** (`int`, defaults to 32) --
  The number of bits of the optimizer state.
- **is_paged** (`bool`, defaults to `False`) --
  Whether the optimizer is a paged optimizer or not.

Base 8-bit optimizer class.

## Optimizer2State[[bitsandbytes.optim.optimizer.Optimizer2State]]

- **optimizer_name** (`str`) --
  The name of the optimizer.
- **params** (`torch.Tensor`) --
  The input parameters to optimize.
- **lr** (`float`, defaults to 1e-3) --
  The learning rate.
- **betas** (`tuple`, defaults to (0.9, 0.999)) --
  The beta values for the optimizer.
- **eps** (`float`, defaults to 1e-8) --
  The epsilon value for the optimizer.
- **weight_decay** (`float`, defaults to 0.0) --
  The weight decay value for the optimizer.
- **optim_bits** (`int`, defaults to 32) --
  The number of bits of the optimizer state.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.
- **max_unorm** (`float`, defaults to 0.0) --
  The maximum value to normalize each block with.
- **skip_zeros** (`bool`, defaults to `False`) --
  Whether to skip zero values for sparse gradients and models to ensure correct updates.
- **is_paged** (`bool`, defaults to `False`) --
  Whether the optimizer is a paged optimizer or not.
- **alpha** (`float`, defaults to 0.0) --
  The alpha value for the AdEMAMix optimizer.
- **t_alpha** (`Optional[int]`, defaults to `None`) --
  Number of iterations for alpha scheduling with AdEMAMix.
- **t_beta3** (`Optional[int]`, defaults to `None`) --
  Number of iterations for beta scheduling with AdEMAMix.

Base 2-state update optimizer class.

## Optimizer1State[[bitsandbytes.optim.optimizer.Optimizer1State]]

- **optimizer_name** (`str`) --
  The name of the optimizer.
- **params** (`torch.Tensor`) --
  The input parameters to optimize.
- **lr** (`float`, defaults to 1e-3) --
  The learning rate.
- **betas** (`tuple`, defaults to (0.9, 0.0)) --
  The beta values for the optimizer.
- **eps** (`float`, defaults to 1e-8) --
  The epsilon value for the optimizer.
- **weight_decay** (`float`, defaults to 0.0) --
  The weight decay value for the optimizer.
- **optim_bits** (`int`, defaults to 32) --
  The number of bits of the optimizer state.
- **args** (`object`, defaults to `None`) --
  An object with additional arguments.
- **min_8bit_size** (`int`, defaults to 4096) --
  The minimum number of elements of the parameter tensors for 8-bit optimization.
- **max_unorm** (`float`, defaults to 0.0) --
  The maximum value to normalize each block with.
- **skip_zeros** (`bool`, defaults to `False`) --
  Whether to skip zero values for sparse gradients and models to ensure correct updates.
- **is_paged** (`bool`, defaults to `False`) --
  Whether the optimizer is a paged optimizer or not.

Base 1-state update optimizer class.

## Utilities[[bitsandbytes.optim.GlobalOptimManager]]

A global optimizer manager for enabling custom optimizer configs.

- **parameters** (`torch.Tensor` or `list(torch.Tensors)`) --
  The input parameters.
- **key** (`str`) --
  The hyperparameter to override.
- **value** --
  The hyperparameter value.
- **key_value_dict** (`dict`) --
  A dictionary with multiple key-values to override.

Override initial optimizer config with specific hyperparameters.

The key-values of the optimizer config for the input parameters are overridden
This can be both, optimizer parameters like `betas` or `lr`, or it can be
8-bit specific parameters like `optim_bits`.

Example:

```py
import torch
import bitsandbytes as bnb

mng = bnb.optim.GlobalOptimManager.get_instance()

model = MyModel()
mng.register_parameters(model.parameters()) # 1. register parameters while still on CPU

model = model.cuda()
# use 8-bit optimizer states for all parameters
adam = bnb.optim.Adam(model.parameters(), lr=0.001, optim_bits=8)

# 2. override: the parameter model.fc1.weight now uses 32-bit Adam
mng.override_config(model.fc1.weight, 'optim_bits', 32)
```
