# DPMSolverSinglestepScheduler

`DPMSolverSinglestepScheduler` is a single step scheduler from [DPM-Solver: A Fast ODE Solver for Diffusion Probabilistic Model Sampling in Around 10 Steps](https://huggingface.co/papers/2206.00927) and [DPM-Solver++: Fast Solver for Guided Sampling of Diffusion Probabilistic Models](https://huggingface.co/papers/2211.01095) by Cheng Lu, Yuhao Zhou, Fan Bao, Jianfei Chen, Chongxuan Li, and Jun Zhu.

DPMSolver (and the improved version DPMSolver++) is a fast dedicated high-order solver for diffusion ODEs with convergence order guarantee. Empirically, DPMSolver sampling with only 20 steps can generate high-quality
samples, and it can generate quite good samples even in 10 steps.

The original implementation can be found at [LuChengTHU/dpm-solver](https://github.com/LuChengTHU/dpm-solver).

## Tips

It is recommended to set `solver_order` to 2 for guide sampling, and `solver_order=3` for unconditional sampling.

Dynamic thresholding from [Imagen](https://huggingface.co/papers/2205.11487) is supported, and for pixel-space
diffusion models, you can set both `algorithm_type="dpmsolver++"` and `thresholding=True` to use dynamic
thresholding. This thresholding method is unsuitable for latent-space diffusion models such as
Stable Diffusion.

## DPMSolverSinglestepScheduler[[diffusers.DPMSolverSinglestepScheduler]]
#### diffusers.DPMSolverSinglestepScheduler[[diffusers.DPMSolverSinglestepScheduler]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_dpmsolver_singlestep.py#L88)

`DPMSolverSinglestepScheduler` is a fast dedicated high-order solver for diffusion ODEs.

This model inherits from [SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

add_noisediffusers.DPMSolverSinglestepScheduler.add_noisehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_dpmsolver_singlestep.py#L1262[{"name": "original_samples", "val": ": Tensor"}, {"name": "noise", "val": ": Tensor"}, {"name": "timesteps", "val": ": IntTensor"}]- **original_samples** (`torch.Tensor`) --
  The original samples without noise.
- **noise** (`torch.Tensor`) --
  The noise to add to the samples.
- **timesteps** (`torch.IntTensor`) --
  The timesteps at which to add noise to the samples.0`torch.Tensor`The noisy samples.

Add noise to the original samples according to the noise schedule at the specified timesteps.

**Parameters:**

num_train_timesteps (`int`, defaults to `1000`) : The number of diffusion steps to train the model.

beta_start (`float`, defaults to `0.0001`) : The starting `beta` value of inference.

beta_end (`float`, defaults to `0.02`) : The final `beta` value.

beta_schedule (`"linear"`, `"scaled_linear"`, or `"squaredcos_cap_v2"`, defaults to `"linear"`) : The beta schedule, a mapping from a beta range to a sequence of betas for stepping the model. Choose from `linear`, `scaled_linear`, or `squaredcos_cap_v2`.

trained_betas (`np.ndarray` or `list[float]`, *optional*) : Pass an array of betas directly to the constructor to bypass `beta_start` and `beta_end`.

solver_order (`int`, defaults to `2`) : The DPMSolver order which can be `1` or `2` or `3`. It is recommended to use `solver_order=2` for guided sampling, and `solver_order=3` for unconditional sampling.

prediction_type (`"epsilon"`, `"sample"`, `"v_prediction"`, or `"flow_prediction"`, defaults to `"epsilon"`) : Prediction type of the scheduler function; can be `epsilon` (predicts the noise of the diffusion process), `sample` (directly predicts the noisy sample`), `v_prediction` (see section 2.4 of [Imagen Video](https://huggingface.co/papers/2210.02303) paper), or `flow_prediction`.

thresholding (`bool`, defaults to `False`) : Whether to use the "dynamic thresholding" method. This is unsuitable for latent-space diffusion models such as Stable Diffusion.

dynamic_thresholding_ratio (`float`, defaults to `0.995`) : The ratio for the dynamic thresholding method. Valid only when `thresholding=True`.

sample_max_value (`float`, defaults to `1.0`) : The threshold value for dynamic thresholding. Valid only when `thresholding=True` and `algorithm_type="dpmsolver++"`.

algorithm_type (`"dpmsolver"`, `"dpmsolver++"`, or `"sde-dpmsolver++"`, defaults to `"dpmsolver++"`) : Algorithm type for the solver; can be `dpmsolver`, `dpmsolver++`, or `sde-dpmsolver++`. The `dpmsolver` type implements the algorithms in the [DPMSolver](https://huggingface.co/papers/2206.00927) paper, and the `dpmsolver++` type implements the algorithms in the [DPMSolver++](https://huggingface.co/papers/2211.01095) paper. It is recommended to use `dpmsolver++` or `sde-dpmsolver++` with `solver_order=2` for guided sampling like in Stable Diffusion.

solver_type (`"midpoint"` or `"heun"`, defaults to `"midpoint"`) : Solver type for the second-order solver; can be `midpoint` or `heun`. The solver type slightly affects the sample quality, especially for a small number of steps. It is recommended to use `midpoint` solvers.

lower_order_final (`bool`, defaults to `False`) : Whether to use lower-order solvers in the final steps. Only valid for < 15 inference steps. This can stabilize the sampling of DPMSolver for steps < 15, especially for steps <= 10.

use_karras_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use Karras sigmas for step sizes in the noise schedule during the sampling process. If `True`, the sigmas are determined according to a sequence of noise levels {σi}.

use_exponential_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use exponential sigmas for step sizes in the noise schedule during the sampling process.

use_beta_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use beta sigmas for step sizes in the noise schedule during the sampling process. Refer to [Beta Sampling is All You Need](https://huggingface.co/papers/2407.12173) for more information.

use_flow_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use flow sigmas for step sizes in the noise schedule during the sampling process.

flow_shift (`float`, *optional*, defaults to `1.0`) : The flow shift parameter for flow-based models.

final_sigmas_type (`"zero"` or `"sigma_min"`, *optional*, defaults to `"zero"`) : The final `sigma` value for the noise schedule during the sampling process. If `"sigma_min"`, the final sigma is the same as the last sigma in the training schedule. If `"zero"`, the final sigma is set to 0.

lambda_min_clipped (`float`, defaults to `-inf`) : Clipping threshold for the minimum value of `lambda(t)` for numerical stability. This is critical for the cosine (`squaredcos_cap_v2`) noise schedule.

variance_type (`"learned"` or `"learned_range"`, *optional*) : Set to `"learned"` or `"learned_range"` for diffusion models that predict variance. If set, the model's output contains the predicted Gaussian variance.

use_dynamic_shifting (`bool`, defaults to `False`) : Whether to use dynamic shifting for the noise schedule.

time_shift_type (`"exponential"`, defaults to `"exponential"`) : The type of time shifting to apply.

**Returns:**

``torch.Tensor``

The noisy samples.
#### convert_model_output[[diffusers.DPMSolverSinglestepScheduler.convert_model_output]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_dpmsolver_singlestep.py#L663)

Convert the model output to the corresponding type the DPMSolver/DPMSolver++ algorithm needs. DPM-Solver is
designed to discretize an integral of the noise prediction model, and DPM-Solver++ is designed to discretize an
integral of the data prediction model.

> [!TIP] > The algorithm and model type are decoupled. You can use either DPMSolver or DPMSolver++ for both
noise > prediction and data prediction models.

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

**Returns:**

``torch.Tensor``

The converted model output.
#### dpm_solver_first_order_update[[diffusers.DPMSolverSinglestepScheduler.dpm_solver_first_order_update]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_dpmsolver_singlestep.py#L759)

One step for the first-order DPMSolver (equivalent to DDIM).

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

timestep (`int`) : The current discrete timestep in the diffusion chain.

prev_timestep (`int`) : The previous discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

**Returns:**

``torch.Tensor``

The sample tensor at the previous timestep.
#### get_order_list[[diffusers.DPMSolverSinglestepScheduler.get_order_list]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_dpmsolver_singlestep.py#L253)

Computes the solver order at each time step.

**Parameters:**

num_inference_steps (`int`) : The number of diffusion steps used when generating samples with a pre-trained model.

**Returns:**

``list[int]``

The list of solver orders for each timestep.
#### index_for_timestep[[diffusers.DPMSolverSinglestepScheduler.index_for_timestep]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_dpmsolver_singlestep.py#L1120)

Find the index for a given timestep in the schedule.

**Parameters:**

timestep (`int` or `torch.Tensor`) : The timestep for which to find the index.

schedule_timesteps (`torch.Tensor`, *optional*) : The timestep schedule to search in. If `None`, uses `self.timesteps`.

**Returns:**

``int``

The index of the timestep in the schedule.
#### scale_model_input[[diffusers.DPMSolverSinglestepScheduler.scale_model_input]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_dpmsolver_singlestep.py#L1246)

Ensures interchangeability with schedulers that need to scale the denoising model input depending on the
current timestep.

**Parameters:**

sample (`torch.Tensor`) : The input sample.

**Returns:**

``torch.Tensor``

A scaled input sample.
#### set_begin_index[[diffusers.DPMSolverSinglestepScheduler.set_begin_index]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_dpmsolver_singlestep.py#L320)

Sets the begin index for the scheduler. This function should be run from pipeline before the inference.

**Parameters:**

begin_index (`int`, defaults to `0`) : The begin index for the scheduler.
#### set_timesteps[[diffusers.DPMSolverSinglestepScheduler.set_timesteps]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_dpmsolver_singlestep.py#L330)

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

**Parameters:**

num_inference_steps (`int`, *optional*) : The number of diffusion steps used when generating samples with a pre-trained model.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.

timesteps (`list[int]`, *optional*) : Custom timesteps used to support arbitrary spacing between timesteps. If `None`, then the default timestep spacing strategy of equal spacing between timesteps schedule is used. If `timesteps` is passed, `num_inference_steps` must be `None`.
#### singlestep_dpm_solver_second_order_update[[diffusers.DPMSolverSinglestepScheduler.singlestep_dpm_solver_second_order_update]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_dpmsolver_singlestep.py#L823)

One step for the second-order singlestep DPMSolver that computes the solution at time `prev_timestep` from the
time `timestep_list[-2]`.

**Parameters:**

model_output_list (`list[torch.Tensor]`) : The direct outputs from learned diffusion model at current and latter timesteps.

timestep (`int`) : The current and latter discrete timestep in the diffusion chain.

prev_timestep (`int`) : The previous discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

**Returns:**

``torch.Tensor``

The sample tensor at the previous timestep.
#### singlestep_dpm_solver_third_order_update[[diffusers.DPMSolverSinglestepScheduler.singlestep_dpm_solver_third_order_update]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_dpmsolver_singlestep.py#L934)

One step for the third-order singlestep DPMSolver that computes the solution at time `prev_timestep` from the
time `timestep_list[-3]`.

**Parameters:**

model_output_list (`list[torch.Tensor]`) : The direct outputs from learned diffusion model at current and latter timesteps.

timestep (`int`) : The current and latter discrete timestep in the diffusion chain.

prev_timestep (`int`) : The previous discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by diffusion process.

**Returns:**

``torch.Tensor``

The sample tensor at the previous timestep.
#### singlestep_dpm_solver_update[[diffusers.DPMSolverSinglestepScheduler.singlestep_dpm_solver_update]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_dpmsolver_singlestep.py#L1056)

One step for the singlestep DPMSolver.

**Parameters:**

model_output_list (`list[torch.Tensor]`) : The direct outputs from learned diffusion model at current and latter timesteps.

timestep (`int`) : The current and latter discrete timestep in the diffusion chain.

prev_timestep (`int`) : The previous discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by diffusion process.

order (`int`) : The solver order at this step.

**Returns:**

``torch.Tensor``

The sample tensor at the previous timestep.
#### step[[diffusers.DPMSolverSinglestepScheduler.step]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_dpmsolver_singlestep.py#L1173)

Predict the sample from the previous timestep by reversing the SDE. This function propagates the sample with
the singlestep DPMSolver.

**Parameters:**

model_output (`torch.Tensor`) : The direct output from learned diffusion model.

timestep (`int` or `torch.Tensor`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

generator (`torch.Generator`, *optional*) : A random number generator for stochastic sampling.

return_dict (`bool`, defaults to `True`) : Whether or not to return a [SchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/dpm_discrete#diffusers.schedulers.scheduling_utils.SchedulerOutput) or `tuple`.

**Returns:**

`[SchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/dpm_discrete#diffusers.schedulers.scheduling_utils.SchedulerOutput) or `tuple``

If return_dict is `True`, [SchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/dpm_discrete#diffusers.schedulers.scheduling_utils.SchedulerOutput) is returned, otherwise a
tuple is returned where the first element is the sample tensor.

## SchedulerOutput[[diffusers.schedulers.scheduling_utils.SchedulerOutput]]
#### diffusers.schedulers.scheduling_utils.SchedulerOutput[[diffusers.schedulers.scheduling_utils.SchedulerOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_utils.py#L61)

Base class for the output of a scheduler's `step` function.

**Parameters:**

prev_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : Computed sample `(x_{t-1})` of previous timestep. `prev_sample` should be used as next model input in the denoising loop.
