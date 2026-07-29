# DPMSolverMultistepInverse

`DPMSolverMultistepInverse` is the inverted scheduler from [DPM-Solver: A Fast ODE Solver for Diffusion Probabilistic Model Sampling in Around 10 Steps](https://huggingface.co/papers/2206.00927) and [DPM-Solver++: Fast Solver for Guided Sampling of Diffusion Probabilistic Models](https://huggingface.co/papers/2211.01095) by Cheng Lu, Yuhao Zhou, Fan Bao, Jianfei Chen, Chongxuan Li, and Jun Zhu.

The implementation is mostly based on the DDIM inversion definition of [Null-text Inversion for Editing Real Images using Guided Diffusion Models](https://huggingface.co/papers/2211.09794) and notebook implementation of the `DiffEdit` latent inversion from [Xiang-cd/DiffEdit-stable-diffusion](https://github.com/Xiang-cd/DiffEdit-stable-diffusion/blob/main/diffedit.ipynb).

## Tips

Dynamic thresholding from [Imagen](https://huggingface.co/papers/2205.11487) is supported, and for pixel-space
diffusion models, you can set both `algorithm_type="dpmsolver++"` and `thresholding=True` to use the dynamic
thresholding. This thresholding method is unsuitable for latent-space diffusion models such as
Stable Diffusion.

## DPMSolverMultistepInverseScheduler[[diffusers.DPMSolverMultistepInverseScheduler]]
#### diffusers.DPMSolverMultistepInverseScheduler[[diffusers.DPMSolverMultistepInverseScheduler]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_dpmsolver_multistep_inverse.py#L86)

`DPMSolverMultistepInverseScheduler` is the reverse scheduler of [DPMSolverMultistepScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/multistep_dpm_solver#diffusers.DPMSolverMultistepScheduler).

This model inherits from [SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

add_noisediffusers.DPMSolverMultistepInverseScheduler.add_noisehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_dpmsolver_multistep_inverse.py#L1115[{"name": "original_samples", "val": ": Tensor"}, {"name": "noise", "val": ": Tensor"}, {"name": "timesteps", "val": ": IntTensor"}]- **original_samples** (`torch.Tensor`) --
  The original samples to add noise to.
- **noise** (`torch.Tensor`) --
  The noise tensor.
- **timesteps** (`torch.IntTensor`) --
  The timesteps at which to add noise.0`torch.Tensor`The noisy samples.

Add noise to the clean `original_samples` using the scheduler's equivalent function.

**Parameters:**

num_train_timesteps (`int`, defaults to 1000) : The number of diffusion steps to train the model.

beta_start (`float`, defaults to 0.0001) : The starting `beta` value of inference.

beta_end (`float`, defaults to 0.02) : The final `beta` value.

beta_schedule (`str`, defaults to `"linear"`) : The beta schedule, a mapping from a beta range to a sequence of betas for stepping the model. Choose from `linear`, `scaled_linear`, or `squaredcos_cap_v2`.

trained_betas (`np.ndarray`, *optional*) : Pass an array of betas directly to the constructor to bypass `beta_start` and `beta_end`.

solver_order (`int`, defaults to 2) : The DPMSolver order which can be `1` or `2` or `3`. It is recommended to use `solver_order=2` for guided sampling, and `solver_order=3` for unconditional sampling.

prediction_type (`str`, defaults to `epsilon`, *optional*) : Prediction type of the scheduler function; can be `epsilon` (predicts the noise of the diffusion process), `sample` (directly predicts the noisy sample`) or `v_prediction` (see section 2.4 of [Imagen Video](https://huggingface.co/papers/2210.02303) paper).

thresholding (`bool`, defaults to `False`) : Whether to use the "dynamic thresholding" method. This is unsuitable for latent-space diffusion models such as Stable Diffusion.

dynamic_thresholding_ratio (`float`, defaults to 0.995) : The ratio for the dynamic thresholding method. Valid only when `thresholding=True`.

sample_max_value (`float`, defaults to 1.0) : The threshold value for dynamic thresholding. Valid only when `thresholding=True` and `algorithm_type="dpmsolver++"`.

algorithm_type (`str`, defaults to `dpmsolver++`) : Algorithm type for the solver; can be `dpmsolver`, `dpmsolver++`, `sde-dpmsolver` or `sde-dpmsolver++`. The `dpmsolver` type implements the algorithms in the [DPMSolver](https://huggingface.co/papers/2206.00927) paper, and the `dpmsolver++` type implements the algorithms in the [DPMSolver++](https://huggingface.co/papers/2211.01095) paper. It is recommended to use `dpmsolver++` or `sde-dpmsolver++` with `solver_order=2` for guided sampling like in Stable Diffusion.

solver_type (`str`, defaults to `midpoint`) : Solver type for the second-order solver; can be `midpoint` or `heun`. The solver type slightly affects the sample quality, especially for a small number of steps. It is recommended to use `midpoint` solvers.

lower_order_final (`bool`, defaults to `True`) : Whether to use lower-order solvers in the final steps. Only valid for < 15 inference steps. This can stabilize the sampling of DPMSolver for steps < 15, especially for steps <= 10.

euler_at_final (`bool`, defaults to `False`) : Whether to use Euler's method in the final step. It is a trade-off between numerical stability and detail richness. This can stabilize the sampling of the SDE variant of DPMSolver for small number of inference steps, but sometimes may result in blurring.

use_karras_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use Karras sigmas for step sizes in the noise schedule during the sampling process. If `True`, the sigmas are determined according to a sequence of noise levels {σi}.

use_exponential_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use exponential sigmas for step sizes in the noise schedule during the sampling process.

use_beta_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use beta sigmas for step sizes in the noise schedule during the sampling process. Refer to [Beta Sampling is All You Need](https://huggingface.co/papers/2407.12173) for more information.

use_flow_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use flow sigmas for step sizes in the noise schedule during the sampling process.

flow_shift (`float`, *optional*, defaults to 1.0) : The flow shift factor. Valid only when `use_flow_sigmas=True`.

lambda_min_clipped (`float`, defaults to `-inf`) : Clipping threshold for the minimum value of `lambda(t)` for numerical stability. This is critical for the cosine (`squaredcos_cap_v2`) noise schedule.

variance_type (`str`, *optional*) : Set to "learned" or "learned_range" for diffusion models that predict variance. If set, the model's output contains the predicted Gaussian variance.

timestep_spacing (`str`, defaults to `"linspace"`) : The way the timesteps should be scaled. Refer to Table 2 of the [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891) for more information.

steps_offset (`int`, defaults to 0) : An offset added to the inference steps, as required by some model families.

**Returns:**

``torch.Tensor``

The noisy samples.
#### convert_model_output[[diffusers.DPMSolverMultistepInverseScheduler.convert_model_output]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_dpmsolver_multistep_inverse.py#L600)

Convert the model output to the corresponding type the DPMSolver/DPMSolver++ algorithm needs. DPM-Solver is
designed to discretize an integral of the noise prediction model, and DPM-Solver++ is designed to discretize an
integral of the data prediction model.

> [!TIP] > The algorithm and model type are decoupled. You can use either DPMSolver or DPMSolver++ for both
noise > prediction and data prediction models.

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

sample (`torch.Tensor`, *optional*) : A current instance of a sample created by the diffusion process.

**Returns:**

``torch.Tensor``

The converted model output.
#### dpm_solver_first_order_update[[diffusers.DPMSolverMultistepInverseScheduler.dpm_solver_first_order_update]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_dpmsolver_multistep_inverse.py#L699)

One step for the first-order DPMSolver (equivalent to DDIM).

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

sample (`torch.Tensor`, *optional*) : A current instance of a sample created by the diffusion process.

noise (`torch.Tensor`, *optional*) : The noise tensor.

**Returns:**

``torch.Tensor``

The sample tensor at the previous timestep.
#### multistep_dpm_solver_second_order_update[[diffusers.DPMSolverMultistepInverseScheduler.multistep_dpm_solver_second_order_update]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_dpmsolver_multistep_inverse.py#L774)

One step for the second-order multistep DPMSolver.

**Parameters:**

model_output_list (`list[torch.Tensor]`) : The direct outputs from learned diffusion model at current and latter timesteps.

sample (`torch.Tensor`, *optional*) : A current instance of a sample created by the diffusion process.

**Returns:**

``torch.Tensor``

The sample tensor at the previous timestep.
#### multistep_dpm_solver_third_order_update[[diffusers.DPMSolverMultistepInverseScheduler.multistep_dpm_solver_third_order_update]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_dpmsolver_multistep_inverse.py#L898)

One step for the third-order multistep DPMSolver.

**Parameters:**

model_output_list (`list[torch.Tensor]`) : The direct outputs from learned diffusion model at current and latter timesteps.

sample (`torch.Tensor`, *optional*) : A current instance of a sample created by diffusion process.

noise (`torch.Tensor`, *optional*) : The noise tensor.

**Returns:**

``torch.Tensor``

The sample tensor at the previous timestep.
#### scale_model_input[[diffusers.DPMSolverMultistepInverseScheduler.scale_model_input]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_dpmsolver_multistep_inverse.py#L1100)

Ensures interchangeability with schedulers that need to scale the denoising model input depending on the
current timestep.

**Parameters:**

sample (`torch.Tensor`) : The input sample.

**Returns:**

``torch.Tensor``

A scaled input sample.
#### set_timesteps[[diffusers.DPMSolverMultistepInverseScheduler.set_timesteps]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_dpmsolver_multistep_inverse.py#L283)

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

**Parameters:**

num_inference_steps (`int`) : The number of diffusion steps used when generating samples with a pre-trained model.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.
#### step[[diffusers.DPMSolverMultistepInverseScheduler.step]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_dpmsolver_multistep_inverse.py#L1014)

Predict the sample from the previous timestep by reversing the SDE. This function propagates the sample with
the multistep DPMSolver.

**Parameters:**

model_output (`torch.Tensor`) : The direct output from learned diffusion model.

timestep (`int`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

generator (`torch.Generator`, *optional*) : A random number generator.

variance_noise (`torch.Tensor`) : Alternative to generating noise with `generator` by directly providing the noise for the variance itself. Useful for methods such as `CycleDiffusion`.

return_dict (`bool`) : Whether or not to return a [SchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/dpm_discrete#diffusers.schedulers.scheduling_utils.SchedulerOutput) or `tuple`.

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
